
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
    pub type _xmlXPathCompExpr;
    pub type _xmlRelaxNG;
    pub type _xmlRelaxNGValidCtxt;
    pub type _xmlRelaxNGParserCtxt;
    #[no_mangle]
    static mut stdout: *mut FILE;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> std::os::raw::c_int;
    #[no_mangle]
    fn fopen(__filename: *const std::os::raw::c_char, __modes: *const std::os::raw::c_char)
     -> *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    fn snprintf(_: *mut std::os::raw::c_char, _: std::os::raw::c_ulong,
                _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    /* *
 * BAD_CAST:
 *
 * Macro to cast a string to an xmlChar * when one know its safe.
 */
    /*
 * xmlChar handling
 */
    #[no_mangle]
    fn xmlStrEqual(str1: *const xmlChar, str2: *const xmlChar) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlStrstr(str: *const xmlChar, val: *const xmlChar) -> *const xmlChar;
    #[no_mangle]
    fn xmlStrchr(str: *const xmlChar, val: xmlChar) -> *const xmlChar;
    #[no_mangle]
    fn xmlStrlen(str: *const xmlChar) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlStrdup(cur: *const xmlChar) -> *mut xmlChar;
    #[no_mangle]
    fn xmlCheckUTF8(utf: *const std::os::raw::c_uchar) -> std::os::raw::c_int;
    #[no_mangle]
    fn sscanf(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char, _: ...)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn fputc(__c: std::os::raw::c_int, __stream: *mut FILE) -> std::os::raw::c_int;
    #[no_mangle]
    fn strcmp(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char) -> std::os::raw::c_int;
    #[no_mangle]
    fn strlen(_: *const std::os::raw::c_char) -> std::os::raw::c_ulong;
    #[no_mangle]
    fn free(__ptr: *mut std::os::raw::c_void);
    /*
 * Lookup of entry in the dictionary.
 */
    #[no_mangle]
    fn xmlDictLookup(dict: xmlDictPtr, name: *const xmlChar, len: std::os::raw::c_int)
     -> *const xmlChar;
    #[no_mangle]
    fn xmlDictOwns(dict: xmlDictPtr, str: *const xmlChar) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlValidateName(value: *const xmlChar, space: std::os::raw::c_int)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlFreeDtd(cur: xmlDtdPtr);
    #[no_mangle]
    fn xmlFreeDoc(cur: xmlDocPtr);
    #[no_mangle]
    fn xmlGetNodePath(node: *const xmlNode) -> *mut xmlChar;
    /* defined(LIBXML_TREE_ENABLED) || defined(LIBXML_DEBUG_ENABLED) */
    #[no_mangle]
    fn xmlDocGetRootElement(doc: *const xmlDoc) -> xmlNodePtr;
    #[no_mangle]
    fn xmlAddChildList(parent: xmlNodePtr, cur: xmlNodePtr) -> xmlNodePtr;
    #[no_mangle]
    fn xmlFreeNodeList(cur: xmlNodePtr);
    /* LIBXML_TREE_ENABLED */
    #[no_mangle]
    fn xmlNodeGetBase(doc: *const xmlDoc, cur: *const xmlNode)
     -> *mut xmlChar;
    #[no_mangle]
    fn xmlNodeSetBase(cur: xmlNodePtr, uri: *const xmlChar);
    #[no_mangle]
    fn xmlDocDump(f: *mut FILE, cur: xmlDocPtr) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlElemDump(f: *mut FILE, doc: xmlDocPtr, cur: xmlNodePtr);
    #[no_mangle]
    fn xmlSaveFile(filename: *const std::os::raw::c_char, cur: xmlDocPtr)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlHashScan(table: xmlHashTablePtr, f: xmlHashScanner,
                   data: *mut std::os::raw::c_void);
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
    fn xmlMemShow(fp: *mut FILE, nr: std::os::raw::c_int);
    #[no_mangle]
    fn xmlSnprintfElementContent(buf: *mut std::os::raw::c_char, size: std::os::raw::c_int,
                                 content: xmlElementContentPtr,
                                 englob: std::os::raw::c_int);
    #[no_mangle]
    fn xmlValidateDtd(ctxt: xmlValidCtxtPtr, doc: xmlDocPtr, dtd: xmlDtdPtr)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlValidateDocument(ctxt: xmlValidCtxtPtr, doc: xmlDocPtr)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlGetDocEntity(doc: *const xmlDoc, name: *const xmlChar)
     -> xmlEntityPtr;
    #[no_mangle]
    fn xmlParseDTD(ExternalID: *const xmlChar, SystemID: *const xmlChar)
     -> xmlDtdPtr;
    #[no_mangle]
    fn xmlParseInNodeContext(node: xmlNodePtr, data: *const std::os::raw::c_char,
                             datalen: std::os::raw::c_int, options: std::os::raw::c_int,
                             lst: *mut xmlNodePtr) -> xmlParserErrors;
    #[no_mangle]
    static mut xmlMalloc: xmlMallocFunc;
    #[no_mangle]
    static mut xmlFree: xmlFreeFunc;
    #[no_mangle]
    fn __xmlGenericError() -> *mut xmlGenericErrorFunc;
    #[no_mangle]
    fn __xmlGenericErrorContext() -> *mut *mut std::os::raw::c_void;
    #[no_mangle]
    fn xmlReadFile(URL: *const std::os::raw::c_char, encoding: *const std::os::raw::c_char,
                   options: std::os::raw::c_int) -> xmlDocPtr;
    /* *
 * IS_BASECHAR:
 * @c:  an UNICODE value (int)
 *
 * Macro to check the following production in the XML spec:
 *
 * [85] BaseChar ::= ... long list see REC ...
 */
    /* *
 * IS_DIGIT:
 * @c:  an UNICODE value (int)
 *
 * Macro to check the following production in the XML spec:
 *
 * [88] Digit ::= ... long list see REC ...
 */
    /* *
 * IS_DIGIT_CH:
 * @c:  an xmlChar value (usually an unsigned char)
 *
 * Behaves like IS_DIGIT but with a single byte argument
 */
    /* *
 * IS_COMBINING:
 * @c:  an UNICODE value (int)
 *
 * Macro to check the following production in the XML spec:
 *
 * [87] CombiningChar ::= ... long list see REC ...
 */
    /* *
 * IS_COMBINING_CH:
 * @c:  an xmlChar (usually an unsigned char)
 *
 * Always false (all combining chars > 0xff)
 */
    /* *
 * IS_EXTENDER:
 * @c:  an UNICODE value (int)
 *
 * Macro to check the following production in the XML spec:
 *
 *
 * [89] Extender ::= #x00B7 | #x02D0 | #x02D1 | #x0387 | #x0640 |
 *                   #x0E46 | #x0EC6 | #x3005 | [#x3031-#x3035] |
 *                   [#x309D-#x309E] | [#x30FC-#x30FE]
 */
    /* *
 * IS_EXTENDER_CH:
 * @c:  an xmlChar value (usually an unsigned char)
 *
 * Behaves like IS_EXTENDER but with a single-byte argument
 */
    /* *
 * IS_IDEOGRAPHIC:
 * @c:  an UNICODE value (int)
 *
 * Macro to check the following production in the XML spec:
 *
 *
 * [86] Ideographic ::= [#x4E00-#x9FA5] | #x3007 | [#x3021-#x3029]
 */
    /* *
 * IS_LETTER:
 * @c:  an UNICODE value (int)
 *
 * Macro to check the following production in the XML spec:
 *
 *
 * [84] Letter ::= BaseChar | Ideographic
 */
    /* *
 * IS_LETTER_CH:
 * @c:  an xmlChar value (normally unsigned char)
 *
 * Macro behaves like IS_LETTER, but only check base chars
 *
 */
    /* *
 * IS_ASCII_LETTER:
 * @c: an xmlChar value
 *
 * Macro to check [a-zA-Z]
 *
 */
    /* *
 * IS_ASCII_DIGIT:
 * @c: an xmlChar value
 *
 * Macro to check [0-9]
 *
 */
    /* *
 * IS_PUBIDCHAR:
 * @c:  an UNICODE value (int)
 *
 * Macro to check the following production in the XML spec:
 *
 *
 * [13] PubidChar ::= #x20 | #xD | #xA | [a-zA-Z0-9] | [-'()+,./:=?;!*#@$_%]
 */
    /* *
 * IS_PUBIDCHAR_CH:
 * @c:  an xmlChar value (normally unsigned char)
 *
 * Same as IS_PUBIDCHAR but for single-byte value
 */
    /* *
 * SKIP_EOL:
 * @p:  and UTF8 string pointer
 *
 * Skips the end of line chars.
 */
    /* *
 * MOVETO_ENDTAG:
 * @p:  and UTF8 string pointer
 *
 * Skips to the next '>' char.
 */
    /* *
 * MOVETO_STARTTAG:
 * @p:  and UTF8 string pointer
 *
 * Skips to the next '<' char.
 */
    /* *
 * Global variables used for predefined strings.
 */
    #[no_mangle]
    static xmlStringText: [xmlChar; 0];
    #[no_mangle]
    static xmlStringTextNoenc: [xmlChar; 0];
    #[no_mangle]
    static xmlStringComment: [xmlChar; 0];
    #[no_mangle]
    fn htmlParseFile(filename: *const std::os::raw::c_char,
                     encoding: *const std::os::raw::c_char) -> htmlDocPtr;
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
    fn xmlXPathFreeObject(obj: xmlXPathObjectPtr);
    #[no_mangle]
    fn htmlDocDump(f: *mut FILE, cur: xmlDocPtr) -> std::os::raw::c_int;
    #[no_mangle]
    fn htmlSaveFile(filename: *const std::os::raw::c_char, cur: xmlDocPtr)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn htmlNodeDumpFile(out: *mut FILE, doc: xmlDocPtr, cur: xmlNodePtr);
    #[no_mangle]
    fn xmlXPathDebugDumpObject(output: *mut FILE, cur: xmlXPathObjectPtr,
                               depth: std::os::raw::c_int);
    /* *
 * Extending a context.
 */
    #[no_mangle]
    fn xmlXPathRegisterNs(ctxt: xmlXPathContextPtr, prefix: *const xmlChar,
                          ns_uri: *const xmlChar) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlCanonicPath(path: *const xmlChar) -> *mut xmlChar;
    /*
 * Interfaces for parsing.
 */
    #[no_mangle]
    fn xmlRelaxNGNewParserCtxt(URL: *const std::os::raw::c_char)
     -> xmlRelaxNGParserCtxtPtr;
    #[no_mangle]
    fn xmlRelaxNGFreeParserCtxt(ctxt: xmlRelaxNGParserCtxtPtr);
    #[no_mangle]
    fn xmlRelaxNGSetParserErrors(ctxt: xmlRelaxNGParserCtxtPtr,
                                 err: xmlRelaxNGValidityErrorFunc,
                                 warn: xmlRelaxNGValidityWarningFunc,
                                 ctx: *mut std::os::raw::c_void);
    #[no_mangle]
    fn xmlRelaxNGParse(ctxt: xmlRelaxNGParserCtxtPtr) -> xmlRelaxNGPtr;
    #[no_mangle]
    fn xmlRelaxNGFree(schema: xmlRelaxNGPtr);
    /* LIBXML_OUTPUT_ENABLED */
    /*
 * Interfaces for validating
 */
    #[no_mangle]
    fn xmlRelaxNGSetValidErrors(ctxt: xmlRelaxNGValidCtxtPtr,
                                err: xmlRelaxNGValidityErrorFunc,
                                warn: xmlRelaxNGValidityWarningFunc,
                                ctx: *mut std::os::raw::c_void);
    #[no_mangle]
    fn xmlRelaxNGNewValidCtxt(schema: xmlRelaxNGPtr)
     -> xmlRelaxNGValidCtxtPtr;
    #[no_mangle]
    fn xmlRelaxNGFreeValidCtxt(ctxt: xmlRelaxNGValidCtxtPtr);
    #[no_mangle]
    fn xmlRelaxNGValidateDoc(ctxt: xmlRelaxNGValidCtxtPtr, doc: xmlDocPtr)
     -> std::os::raw::c_int;
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
pub type xmlDictPtr = *mut xmlDict;
pub type xmlDict = _xmlDict;
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
pub type xmlEntityPtr = *mut xmlEntity;
pub type xmlEntity = _xmlEntity;
/*
 * Summary: interface for the XML entities handling
 * Description: this module provides some of the entity API needed
 *              for the parser and applications.
 *
 * Copy: See Copyright for the status of this software.
 *
 * Author: Daniel Veillard
 */
/*
 * The different valid entity types.
 */
/*
 * An unit of storage for an entity, contains the string, the value
 * and the linkind data needed for the linking in the hash table.
 */
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
pub type xmlElementContentPtr = *mut xmlElementContent;
pub type xmlElementContent = _xmlElementContent;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlElementContent {
    pub type_0: xmlElementContentType,
    pub ocur: xmlElementContentOccur,
    pub name: *const xmlChar,
    pub c1: *mut _xmlElementContent,
    pub c2: *mut _xmlElementContent,
    pub parent: *mut _xmlElementContent,
    pub prefix: *const xmlChar,
}
pub type xmlElementContentOccur = std::os::raw::c_uint;
pub const XML_ELEMENT_CONTENT_PLUS: xmlElementContentOccur = 4;
pub const XML_ELEMENT_CONTENT_MULT: xmlElementContentOccur = 3;
pub const XML_ELEMENT_CONTENT_OPT: xmlElementContentOccur = 2;
pub const XML_ELEMENT_CONTENT_ONCE: xmlElementContentOccur = 1;
pub type xmlElementContentType = std::os::raw::c_uint;
pub const XML_ELEMENT_CONTENT_OR: xmlElementContentType = 4;
pub const XML_ELEMENT_CONTENT_SEQ: xmlElementContentType = 3;
pub const XML_ELEMENT_CONTENT_ELEMENT: xmlElementContentType = 2;
pub const XML_ELEMENT_CONTENT_PCDATA: xmlElementContentType = 1;
pub type xmlEnumerationPtr = *mut xmlEnumeration;
pub type xmlEnumeration = _xmlEnumeration;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlEnumeration {
    pub next: *mut _xmlEnumeration,
    pub name: *const xmlChar,
}
pub type xmlAttributeDefault = std::os::raw::c_uint;
pub const XML_ATTRIBUTE_FIXED: xmlAttributeDefault = 4;
pub const XML_ATTRIBUTE_IMPLIED: xmlAttributeDefault = 3;
pub const XML_ATTRIBUTE_REQUIRED: xmlAttributeDefault = 2;
pub const XML_ATTRIBUTE_NONE: xmlAttributeDefault = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlAttribute {
    pub _private: *mut std::os::raw::c_void,
    pub type_0: xmlElementType,
    pub name: *const xmlChar,
    pub children: *mut _xmlNode,
    pub last: *mut _xmlNode,
    pub parent: *mut _xmlDtd,
    pub next: *mut _xmlNode,
    pub prev: *mut _xmlNode,
    pub doc: *mut _xmlDoc,
    pub nexth: *mut _xmlAttribute,
    pub atype: xmlAttributeType,
    pub def: xmlAttributeDefault,
    pub defaultValue: *const xmlChar,
    pub tree: xmlEnumerationPtr,
    pub prefix: *const xmlChar,
    pub elem: *const xmlChar,
}
pub type xmlAttribute = _xmlAttribute;
pub type xmlAttributePtr = *mut xmlAttribute;
pub type xmlElementTypeVal = std::os::raw::c_uint;
pub const XML_ELEMENT_TYPE_ELEMENT: xmlElementTypeVal = 4;
pub const XML_ELEMENT_TYPE_MIXED: xmlElementTypeVal = 3;
pub const XML_ELEMENT_TYPE_ANY: xmlElementTypeVal = 2;
pub const XML_ELEMENT_TYPE_EMPTY: xmlElementTypeVal = 1;
pub const XML_ELEMENT_TYPE_UNDEFINED: xmlElementTypeVal = 0;
pub type xmlRegexp = _xmlRegexp;
pub type xmlRegexpPtr = *mut xmlRegexp;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlElement {
    pub _private: *mut std::os::raw::c_void,
    pub type_0: xmlElementType,
    pub name: *const xmlChar,
    pub children: *mut _xmlNode,
    pub last: *mut _xmlNode,
    pub parent: *mut _xmlDtd,
    pub next: *mut _xmlNode,
    pub prev: *mut _xmlNode,
    pub doc: *mut _xmlDoc,
    pub etype: xmlElementTypeVal,
    pub content: xmlElementContentPtr,
    pub attributes: xmlAttributePtr,
    pub prefix: *const xmlChar,
    pub contModel: xmlRegexpPtr,
}
pub type xmlElement = _xmlElement;
pub type xmlElementPtr = *mut xmlElement;
pub type xmlNsPtr = *mut xmlNs;
pub type xmlDtd = _xmlDtd;
pub type xmlDtdPtr = *mut xmlDtd;
/* *
 * xmlHashScanner:
 * @payload:  the data in the hash
 * @data:  extra scannner data
 * @name:  the name associated
 *
 * Callback when scanning data in a hash with the simple scanner.
 */
pub type xmlHashScanner
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *mut std::os::raw::c_void,
                                _: *const xmlChar) -> ()>;
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
pub type xmlParserErrors = std::os::raw::c_uint;
pub const XML_BUF_OVERFLOW: xmlParserErrors = 7000;
pub const XML_I18N_NO_OUTPUT: xmlParserErrors = 6004;
pub const XML_I18N_CONV_FAILED: xmlParserErrors = 6003;
pub const XML_I18N_EXCESS_HANDLER: xmlParserErrors = 6002;
pub const XML_I18N_NO_HANDLER: xmlParserErrors = 6001;
pub const XML_I18N_NO_NAME: xmlParserErrors = 6000;
pub const XML_CHECK_NAME_NOT_NULL: xmlParserErrors = 5037;
pub const XML_CHECK_WRONG_NAME: xmlParserErrors = 5036;
pub const XML_CHECK_OUTSIDE_DICT: xmlParserErrors = 5035;
pub const XML_CHECK_NOT_NCNAME: xmlParserErrors = 5034;
pub const XML_CHECK_NO_DICT: xmlParserErrors = 5033;
pub const XML_CHECK_NOT_UTF8: xmlParserErrors = 5032;
pub const XML_CHECK_NS_ANCESTOR: xmlParserErrors = 5031;
pub const XML_CHECK_NS_SCOPE: xmlParserErrors = 5030;
pub const XML_CHECK_WRONG_PARENT: xmlParserErrors = 5029;
pub const XML_CHECK_NO_HREF: xmlParserErrors = 5028;
pub const XML_CHECK_NOT_NS_DECL: xmlParserErrors = 5027;
pub const XML_CHECK_NOT_ENTITY_DECL: xmlParserErrors = 5026;
pub const XML_CHECK_NOT_ELEM_DECL: xmlParserErrors = 5025;
pub const XML_CHECK_NOT_ATTR_DECL: xmlParserErrors = 5024;
pub const XML_CHECK_NOT_ATTR: xmlParserErrors = 5023;
pub const XML_CHECK_NOT_DTD: xmlParserErrors = 5022;
pub const XML_CHECK_WRONG_NEXT: xmlParserErrors = 5021;
pub const XML_CHECK_NO_NEXT: xmlParserErrors = 5020;
pub const XML_CHECK_WRONG_PREV: xmlParserErrors = 5019;
pub const XML_CHECK_NO_PREV: xmlParserErrors = 5018;
pub const XML_CHECK_WRONG_DOC: xmlParserErrors = 5017;
pub const XML_CHECK_NO_ELEM: xmlParserErrors = 5016;
pub const XML_CHECK_NO_NAME: xmlParserErrors = 5015;
pub const XML_CHECK_NO_DOC: xmlParserErrors = 5014;
pub const XML_CHECK_NO_PARENT: xmlParserErrors = 5013;
pub const XML_CHECK_ENTITY_TYPE: xmlParserErrors = 5012;
pub const XML_CHECK_UNKNOWN_NODE: xmlParserErrors = 5011;
pub const XML_CHECK_FOUND_NOTATION: xmlParserErrors = 5010;
pub const XML_CHECK_FOUND_FRAGMENT: xmlParserErrors = 5009;
pub const XML_CHECK_FOUND_DOCTYPE: xmlParserErrors = 5008;
pub const XML_CHECK_FOUND_COMMENT: xmlParserErrors = 5007;
pub const XML_CHECK_FOUND_PI: xmlParserErrors = 5006;
pub const XML_CHECK_FOUND_ENTITY: xmlParserErrors = 5005;
pub const XML_CHECK_FOUND_ENTITYREF: xmlParserErrors = 5004;
pub const XML_CHECK_FOUND_CDATA: xmlParserErrors = 5003;
pub const XML_CHECK_FOUND_TEXT: xmlParserErrors = 5002;
pub const XML_CHECK_FOUND_ATTRIBUTE: xmlParserErrors = 5001;
pub const XML_CHECK_FOUND_ELEMENT: xmlParserErrors = 5000;
pub const XML_MODULE_CLOSE: xmlParserErrors = 4901;
pub const XML_MODULE_OPEN: xmlParserErrors = 4900;
pub const XML_SCHEMATRONV_REPORT: xmlParserErrors = 4001;
pub const XML_SCHEMATRONV_ASSERT: xmlParserErrors = 4000;
pub const XML_SCHEMAP_COS_ALL_LIMITED: xmlParserErrors = 3091;
pub const XML_SCHEMAP_A_PROPS_CORRECT_3: xmlParserErrors = 3090;
pub const XML_SCHEMAP_AU_PROPS_CORRECT: xmlParserErrors = 3089;
pub const XML_SCHEMAP_COS_CT_EXTENDS_1_2: xmlParserErrors = 3088;
pub const XML_SCHEMAP_AG_PROPS_CORRECT: xmlParserErrors = 3087;
pub const XML_SCHEMAP_WARN_ATTR_POINTLESS_PROH: xmlParserErrors = 3086;
pub const XML_SCHEMAP_WARN_ATTR_REDECL_PROH: xmlParserErrors = 3085;
pub const XML_SCHEMAP_WARN_UNLOCATED_SCHEMA: xmlParserErrors = 3084;
pub const XML_SCHEMAP_WARN_SKIP_SCHEMA: xmlParserErrors = 3083;
pub const XML_SCHEMAP_SRC_IMPORT: xmlParserErrors = 3082;
pub const XML_SCHEMAP_SRC_REDEFINE: xmlParserErrors = 3081;
pub const XML_SCHEMAP_C_PROPS_CORRECT: xmlParserErrors = 3080;
pub const XML_SCHEMAP_A_PROPS_CORRECT_2: xmlParserErrors = 3079;
pub const XML_SCHEMAP_AU_PROPS_CORRECT_2: xmlParserErrors = 3078;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_2_1_3: xmlParserErrors = 3077;
pub const XML_SCHEMAP_SRC_CT_1: xmlParserErrors = 3076;
pub const XML_SCHEMAP_MG_PROPS_CORRECT_2: xmlParserErrors = 3075;
pub const XML_SCHEMAP_MG_PROPS_CORRECT_1: xmlParserErrors = 3074;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_GROUP_3: xmlParserErrors = 3073;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_GROUP_2: xmlParserErrors = 3072;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_GROUP_1: xmlParserErrors = 3071;
pub const XML_SCHEMAP_NOT_DETERMINISTIC: xmlParserErrors = 3070;
pub const XML_SCHEMAP_INTERNAL: xmlParserErrors = 3069;
pub const XML_SCHEMAP_SRC_IMPORT_2_2: xmlParserErrors = 3068;
pub const XML_SCHEMAP_SRC_IMPORT_2_1: xmlParserErrors = 3067;
pub const XML_SCHEMAP_SRC_IMPORT_2: xmlParserErrors = 3066;
pub const XML_SCHEMAP_SRC_IMPORT_1_2: xmlParserErrors = 3065;
pub const XML_SCHEMAP_SRC_IMPORT_1_1: xmlParserErrors = 3064;
pub const XML_SCHEMAP_COS_CT_EXTENDS_1_1: xmlParserErrors = 3063;
pub const XML_SCHEMAP_CVC_SIMPLE_TYPE: xmlParserErrors = 3062;
pub const XML_SCHEMAP_COS_VALID_DEFAULT_2_2_2: xmlParserErrors = 3061;
pub const XML_SCHEMAP_COS_VALID_DEFAULT_2_2_1: xmlParserErrors = 3060;
pub const XML_SCHEMAP_COS_VALID_DEFAULT_2_1: xmlParserErrors = 3059;
pub const XML_SCHEMAP_COS_VALID_DEFAULT_1: xmlParserErrors = 3058;
pub const XML_SCHEMAP_NO_XSI: xmlParserErrors = 3057;
pub const XML_SCHEMAP_NO_XMLNS: xmlParserErrors = 3056;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_4: xmlParserErrors = 3055;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_3_2: xmlParserErrors = 3054;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_3_1: xmlParserErrors = 3053;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_2: xmlParserErrors = 3052;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_1: xmlParserErrors = 3051;
pub const XML_SCHEMAP_SRC_INCLUDE: xmlParserErrors = 3050;
pub const XML_SCHEMAP_E_PROPS_CORRECT_6: xmlParserErrors = 3049;
pub const XML_SCHEMAP_E_PROPS_CORRECT_5: xmlParserErrors = 3048;
pub const XML_SCHEMAP_E_PROPS_CORRECT_4: xmlParserErrors = 3047;
pub const XML_SCHEMAP_E_PROPS_CORRECT_3: xmlParserErrors = 3046;
pub const XML_SCHEMAP_E_PROPS_CORRECT_2: xmlParserErrors = 3045;
pub const XML_SCHEMAP_P_PROPS_CORRECT_2_2: xmlParserErrors = 3044;
pub const XML_SCHEMAP_P_PROPS_CORRECT_2_1: xmlParserErrors = 3043;
pub const XML_SCHEMAP_P_PROPS_CORRECT_1: xmlParserErrors = 3042;
pub const XML_SCHEMAP_SRC_ELEMENT_3: xmlParserErrors = 3041;
pub const XML_SCHEMAP_SRC_ELEMENT_2_2: xmlParserErrors = 3040;
pub const XML_SCHEMAP_SRC_ELEMENT_2_1: xmlParserErrors = 3039;
pub const XML_SCHEMAP_SRC_ELEMENT_1: xmlParserErrors = 3038;
pub const XML_SCHEMAP_S4S_ATTR_INVALID_VALUE: xmlParserErrors = 3037;
pub const XML_SCHEMAP_S4S_ATTR_MISSING: xmlParserErrors = 3036;
pub const XML_SCHEMAP_S4S_ATTR_NOT_ALLOWED: xmlParserErrors = 3035;
pub const XML_SCHEMAP_S4S_ELEM_MISSING: xmlParserErrors = 3034;
pub const XML_SCHEMAP_S4S_ELEM_NOT_ALLOWED: xmlParserErrors = 3033;
pub const XML_SCHEMAP_COS_ST_DERIVED_OK_2_2: xmlParserErrors = 3032;
pub const XML_SCHEMAP_COS_ST_DERIVED_OK_2_1: xmlParserErrors = 3031;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_5: xmlParserErrors = 3030;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_4: xmlParserErrors = 3029;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_3: xmlParserErrors = 3028;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_1: xmlParserErrors = 3027;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_2: xmlParserErrors = 3026;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_1_2: xmlParserErrors = 3025;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_1: xmlParserErrors = 3024;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_1: xmlParserErrors = 3023;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_5: xmlParserErrors = 3022;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_4: xmlParserErrors = 3021;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_3: xmlParserErrors = 3020;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_2: xmlParserErrors = 3019;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_1: xmlParserErrors = 3018;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_1_2: xmlParserErrors = 3017;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_1_1: xmlParserErrors = 3016;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_1: xmlParserErrors = 3015;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_1_3_2: xmlParserErrors = 3014;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_1_3_1: xmlParserErrors = 3013;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_1_2: xmlParserErrors = 3012;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_1_1: xmlParserErrors = 3011;
pub const XML_SCHEMAP_ST_PROPS_CORRECT_3: xmlParserErrors = 3010;
pub const XML_SCHEMAP_ST_PROPS_CORRECT_2: xmlParserErrors = 3009;
pub const XML_SCHEMAP_ST_PROPS_CORRECT_1: xmlParserErrors = 3008;
pub const XML_SCHEMAP_SRC_UNION_MEMBERTYPES_OR_SIMPLETYPES: xmlParserErrors =
    3007;
pub const XML_SCHEMAP_SRC_LIST_ITEMTYPE_OR_SIMPLETYPE: xmlParserErrors = 3006;
pub const XML_SCHEMAP_SRC_RESTRICTION_BASE_OR_SIMPLETYPE: xmlParserErrors =
    3005;
pub const XML_SCHEMAP_SRC_RESOLVE: xmlParserErrors = 3004;
pub const XML_SCHEMAP_SRC_SIMPLE_TYPE_4: xmlParserErrors = 3003;
pub const XML_SCHEMAP_SRC_SIMPLE_TYPE_3: xmlParserErrors = 3002;
pub const XML_SCHEMAP_SRC_SIMPLE_TYPE_2: xmlParserErrors = 3001;
pub const XML_SCHEMAP_SRC_SIMPLE_TYPE_1: xmlParserErrors = 3000;
pub const XML_HTTP_UNKNOWN_HOST: xmlParserErrors = 2022;
pub const XML_HTTP_USE_IP: xmlParserErrors = 2021;
pub const XML_HTTP_URL_SYNTAX: xmlParserErrors = 2020;
pub const XML_FTP_URL_SYNTAX: xmlParserErrors = 2003;
pub const XML_FTP_ACCNT: xmlParserErrors = 2002;
pub const XML_FTP_EPSV_ANSWER: xmlParserErrors = 2001;
pub const XML_FTP_PASV_ANSWER: xmlParserErrors = 2000;
pub const XML_C14N_RELATIVE_NAMESPACE: xmlParserErrors = 1955;
pub const XML_C14N_UNKNOW_NODE: xmlParserErrors = 1954;
pub const XML_C14N_INVALID_NODE: xmlParserErrors = 1953;
pub const XML_C14N_CREATE_STACK: xmlParserErrors = 1952;
pub const XML_C14N_REQUIRES_UTF8: xmlParserErrors = 1951;
pub const XML_C14N_CREATE_CTXT: xmlParserErrors = 1950;
pub const XML_XPTR_EXTRA_OBJECTS: xmlParserErrors = 1903;
pub const XML_XPTR_EVAL_FAILED: xmlParserErrors = 1902;
pub const XML_XPTR_CHILDSEQ_START: xmlParserErrors = 1901;
pub const XML_XPTR_UNKNOWN_SCHEME: xmlParserErrors = 1900;
pub const XML_SCHEMAV_MISC: xmlParserErrors = 1879;
pub const XML_SCHEMAV_CVC_WILDCARD: xmlParserErrors = 1878;
pub const XML_SCHEMAV_CVC_IDC: xmlParserErrors = 1877;
pub const XML_SCHEMAV_CVC_TYPE_2: xmlParserErrors = 1876;
pub const XML_SCHEMAV_CVC_TYPE_1: xmlParserErrors = 1875;
pub const XML_SCHEMAV_CVC_AU: xmlParserErrors = 1874;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_1: xmlParserErrors = 1873;
pub const XML_SCHEMAV_DOCUMENT_ELEMENT_MISSING: xmlParserErrors = 1872;
pub const XML_SCHEMAV_ELEMENT_CONTENT: xmlParserErrors = 1871;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_5_2: xmlParserErrors = 1870;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_5_1: xmlParserErrors = 1869;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_4: xmlParserErrors = 1868;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_3_2_2: xmlParserErrors = 1867;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_3_2_1: xmlParserErrors = 1866;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_3_1: xmlParserErrors = 1865;
pub const XML_SCHEMAV_CVC_ATTRIBUTE_4: xmlParserErrors = 1864;
pub const XML_SCHEMAV_CVC_ATTRIBUTE_3: xmlParserErrors = 1863;
pub const XML_SCHEMAV_CVC_ATTRIBUTE_2: xmlParserErrors = 1862;
pub const XML_SCHEMAV_CVC_ATTRIBUTE_1: xmlParserErrors = 1861;
pub const XML_SCHEMAV_CVC_ELT_7: xmlParserErrors = 1860;
pub const XML_SCHEMAV_CVC_ELT_6: xmlParserErrors = 1859;
pub const XML_SCHEMAV_CVC_ELT_5_2_2_2_2: xmlParserErrors = 1858;
pub const XML_SCHEMAV_CVC_ELT_5_2_2_2_1: xmlParserErrors = 1857;
pub const XML_SCHEMAV_CVC_ELT_5_2_2_1: xmlParserErrors = 1856;
pub const XML_SCHEMAV_CVC_ELT_5_2_1: xmlParserErrors = 1855;
pub const XML_SCHEMAV_CVC_ELT_5_1_2: xmlParserErrors = 1854;
pub const XML_SCHEMAV_CVC_ELT_5_1_1: xmlParserErrors = 1853;
pub const XML_SCHEMAV_CVC_ELT_4_3: xmlParserErrors = 1852;
pub const XML_SCHEMAV_CVC_ELT_4_2: xmlParserErrors = 1851;
pub const XML_SCHEMAV_CVC_ELT_4_1: xmlParserErrors = 1850;
pub const XML_SCHEMAV_CVC_ELT_3_2_2: xmlParserErrors = 1849;
pub const XML_SCHEMAV_CVC_ELT_3_2_1: xmlParserErrors = 1848;
pub const XML_SCHEMAV_CVC_ELT_3_1: xmlParserErrors = 1847;
pub const XML_SCHEMAV_CVC_ELT_2: xmlParserErrors = 1846;
pub const XML_SCHEMAV_CVC_ELT_1: xmlParserErrors = 1845;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_2_4: xmlParserErrors = 1844;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_2_3: xmlParserErrors = 1843;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_2_2: xmlParserErrors = 1842;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_2_1: xmlParserErrors = 1841;
pub const XML_SCHEMAV_CVC_ENUMERATION_VALID: xmlParserErrors = 1840;
pub const XML_SCHEMAV_CVC_PATTERN_VALID: xmlParserErrors = 1839;
pub const XML_SCHEMAV_CVC_FRACTIONDIGITS_VALID: xmlParserErrors = 1838;
pub const XML_SCHEMAV_CVC_TOTALDIGITS_VALID: xmlParserErrors = 1837;
pub const XML_SCHEMAV_CVC_MAXEXCLUSIVE_VALID: xmlParserErrors = 1836;
pub const XML_SCHEMAV_CVC_MINEXCLUSIVE_VALID: xmlParserErrors = 1835;
pub const XML_SCHEMAV_CVC_MAXINCLUSIVE_VALID: xmlParserErrors = 1834;
pub const XML_SCHEMAV_CVC_MININCLUSIVE_VALID: xmlParserErrors = 1833;
pub const XML_SCHEMAV_CVC_MAXLENGTH_VALID: xmlParserErrors = 1832;
pub const XML_SCHEMAV_CVC_MINLENGTH_VALID: xmlParserErrors = 1831;
pub const XML_SCHEMAV_CVC_LENGTH_VALID: xmlParserErrors = 1830;
pub const XML_SCHEMAV_CVC_FACET_VALID: xmlParserErrors = 1829;
pub const XML_SCHEMAV_CVC_TYPE_3_1_2: xmlParserErrors = 1828;
pub const XML_SCHEMAV_CVC_TYPE_3_1_1: xmlParserErrors = 1827;
pub const XML_SCHEMAV_CVC_DATATYPE_VALID_1_2_3: xmlParserErrors = 1826;
pub const XML_SCHEMAV_CVC_DATATYPE_VALID_1_2_2: xmlParserErrors = 1825;
pub const XML_SCHEMAV_CVC_DATATYPE_VALID_1_2_1: xmlParserErrors = 1824;
pub const XML_SCHEMAV_FACET: xmlParserErrors = 1823;
pub const XML_SCHEMAV_VALUE: xmlParserErrors = 1822;
pub const XML_SCHEMAV_ATTRINVALID: xmlParserErrors = 1821;
pub const XML_SCHEMAV_ATTRUNKNOWN: xmlParserErrors = 1820;
pub const XML_SCHEMAV_NOTSIMPLE: xmlParserErrors = 1819;
pub const XML_SCHEMAV_INTERNAL: xmlParserErrors = 1818;
pub const XML_SCHEMAV_CONSTRUCT: xmlParserErrors = 1817;
pub const XML_SCHEMAV_NOTDETERMINIST: xmlParserErrors = 1816;
pub const XML_SCHEMAV_INVALIDELEM: xmlParserErrors = 1815;
pub const XML_SCHEMAV_INVALIDATTR: xmlParserErrors = 1814;
pub const XML_SCHEMAV_EXTRACONTENT: xmlParserErrors = 1813;
pub const XML_SCHEMAV_NOTNILLABLE: xmlParserErrors = 1812;
pub const XML_SCHEMAV_HAVEDEFAULT: xmlParserErrors = 1811;
pub const XML_SCHEMAV_ELEMCONT: xmlParserErrors = 1810;
pub const XML_SCHEMAV_NOTEMPTY: xmlParserErrors = 1809;
pub const XML_SCHEMAV_ISABSTRACT: xmlParserErrors = 1808;
pub const XML_SCHEMAV_NOROLLBACK: xmlParserErrors = 1807;
pub const XML_SCHEMAV_NOTYPE: xmlParserErrors = 1806;
pub const XML_SCHEMAV_WRONGELEM: xmlParserErrors = 1805;
pub const XML_SCHEMAV_MISSING: xmlParserErrors = 1804;
pub const XML_SCHEMAV_NOTTOPLEVEL: xmlParserErrors = 1803;
pub const XML_SCHEMAV_UNDECLAREDELEM: xmlParserErrors = 1802;
pub const XML_SCHEMAV_NOROOT: xmlParserErrors = 1801;
pub const XML_SCHEMAP_COS_CT_EXTENDS_1_3: xmlParserErrors = 1800;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_4_3: xmlParserErrors = 1799;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_4_2: xmlParserErrors = 1798;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_4_1: xmlParserErrors = 1797;
pub const XML_SCHEMAP_SRC_IMPORT_3_2: xmlParserErrors = 1796;
pub const XML_SCHEMAP_SRC_IMPORT_3_1: xmlParserErrors = 1795;
pub const XML_SCHEMAP_UNION_NOT_EXPRESSIBLE: xmlParserErrors = 1794;
pub const XML_SCHEMAP_INTERSECTION_NOT_EXPRESSIBLE: xmlParserErrors = 1793;
pub const XML_SCHEMAP_WILDCARD_INVALID_NS_MEMBER: xmlParserErrors = 1792;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_3: xmlParserErrors = 1791;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_2_2: xmlParserErrors = 1790;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_2_1_2: xmlParserErrors = 1789;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_2_1_1: xmlParserErrors = 1788;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_1: xmlParserErrors = 1787;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_5: xmlParserErrors = 1786;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_4: xmlParserErrors = 1785;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_3: xmlParserErrors = 1784;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_2: xmlParserErrors = 1783;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_1: xmlParserErrors = 1782;
pub const XML_SCHEMAP_REF_AND_CONTENT: xmlParserErrors = 1781;
pub const XML_SCHEMAP_INVALID_ATTR_NAME: xmlParserErrors = 1780;
pub const XML_SCHEMAP_MISSING_SIMPLETYPE_CHILD: xmlParserErrors = 1779;
pub const XML_SCHEMAP_INVALID_ATTR_INLINE_COMBINATION: xmlParserErrors = 1778;
pub const XML_SCHEMAP_INVALID_ATTR_COMBINATION: xmlParserErrors = 1777;
pub const XML_SCHEMAP_SUPERNUMEROUS_LIST_ITEM_TYPE: xmlParserErrors = 1776;
pub const XML_SCHEMAP_RECURSIVE: xmlParserErrors = 1775;
pub const XML_SCHEMAP_INVALID_ATTR_USE: xmlParserErrors = 1774;
pub const XML_SCHEMAP_UNKNOWN_MEMBER_TYPE: xmlParserErrors = 1773;
pub const XML_SCHEMAP_NOT_SCHEMA: xmlParserErrors = 1772;
pub const XML_SCHEMAP_INCLUDE_SCHEMA_NO_URI: xmlParserErrors = 1771;
pub const XML_SCHEMAP_INCLUDE_SCHEMA_NOT_URI: xmlParserErrors = 1770;
pub const XML_SCHEMAP_UNKNOWN_INCLUDE_CHILD: xmlParserErrors = 1769;
pub const XML_SCHEMAP_DEF_AND_PREFIX: xmlParserErrors = 1768;
pub const XML_SCHEMAP_UNKNOWN_PREFIX: xmlParserErrors = 1767;
pub const XML_SCHEMAP_FAILED_PARSE: xmlParserErrors = 1766;
pub const XML_SCHEMAP_REDEFINED_NOTATION: xmlParserErrors = 1765;
pub const XML_SCHEMAP_REDEFINED_ATTR: xmlParserErrors = 1764;
pub const XML_SCHEMAP_REDEFINED_ATTRGROUP: xmlParserErrors = 1763;
pub const XML_SCHEMAP_REDEFINED_ELEMENT: xmlParserErrors = 1762;
pub const XML_SCHEMAP_REDEFINED_TYPE: xmlParserErrors = 1761;
pub const XML_SCHEMAP_REDEFINED_GROUP: xmlParserErrors = 1760;
pub const XML_SCHEMAP_NOROOT: xmlParserErrors = 1759;
pub const XML_SCHEMAP_NOTHING_TO_PARSE: xmlParserErrors = 1758;
pub const XML_SCHEMAP_FAILED_LOAD: xmlParserErrors = 1757;
pub const XML_SCHEMAP_REGEXP_INVALID: xmlParserErrors = 1756;
pub const XML_SCHEMAP_ELEM_DEFAULT_FIXED: xmlParserErrors = 1755;
pub const XML_SCHEMAP_UNKNOWN_UNION_CHILD: xmlParserErrors = 1754;
pub const XML_SCHEMAP_UNKNOWN_TYPE: xmlParserErrors = 1753;
pub const XML_SCHEMAP_UNKNOWN_SIMPLETYPE_CHILD: xmlParserErrors = 1752;
pub const XML_SCHEMAP_UNKNOWN_SIMPLECONTENT_CHILD: xmlParserErrors = 1751;
pub const XML_SCHEMAP_UNKNOWN_SEQUENCE_CHILD: xmlParserErrors = 1750;
pub const XML_SCHEMAP_UNKNOWN_SCHEMAS_CHILD: xmlParserErrors = 1749;
pub const XML_SCHEMAP_UNKNOWN_RESTRICTION_CHILD: xmlParserErrors = 1748;
pub const XML_SCHEMAP_UNKNOWN_REF: xmlParserErrors = 1747;
pub const XML_SCHEMAP_UNKNOWN_PROCESSCONTENT_CHILD: xmlParserErrors = 1746;
pub const XML_SCHEMAP_UNKNOWN_NOTATION_CHILD: xmlParserErrors = 1745;
pub const XML_SCHEMAP_UNKNOWN_LIST_CHILD: xmlParserErrors = 1744;
pub const XML_SCHEMAP_UNKNOWN_IMPORT_CHILD: xmlParserErrors = 1743;
pub const XML_SCHEMAP_UNKNOWN_GROUP_CHILD: xmlParserErrors = 1742;
pub const XML_SCHEMAP_UNKNOWN_FACET_TYPE: xmlParserErrors = 1741;
pub const XML_SCHEMAP_UNKNOWN_FACET_CHILD: xmlParserErrors = 1740;
pub const XML_SCHEMAP_UNKNOWN_EXTENSION_CHILD: xmlParserErrors = 1739;
pub const XML_SCHEMAP_UNKNOWN_ELEM_CHILD: xmlParserErrors = 1738;
pub const XML_SCHEMAP_UNKNOWN_COMPLEXTYPE_CHILD: xmlParserErrors = 1737;
pub const XML_SCHEMAP_UNKNOWN_COMPLEXCONTENT_CHILD: xmlParserErrors = 1736;
pub const XML_SCHEMAP_UNKNOWN_CHOICE_CHILD: xmlParserErrors = 1735;
pub const XML_SCHEMAP_UNKNOWN_BASE_TYPE: xmlParserErrors = 1734;
pub const XML_SCHEMAP_UNKNOWN_ATTRIBUTE_GROUP: xmlParserErrors = 1733;
pub const XML_SCHEMAP_UNKNOWN_ATTRGRP_CHILD: xmlParserErrors = 1732;
pub const XML_SCHEMAP_UNKNOWN_ATTR_CHILD: xmlParserErrors = 1731;
pub const XML_SCHEMAP_UNKNOWN_ANYATTRIBUTE_CHILD: xmlParserErrors = 1730;
pub const XML_SCHEMAP_UNKNOWN_ALL_CHILD: xmlParserErrors = 1729;
pub const XML_SCHEMAP_TYPE_AND_SUBTYPE: xmlParserErrors = 1728;
pub const XML_SCHEMAP_SIMPLETYPE_NONAME: xmlParserErrors = 1727;
pub const XML_SCHEMAP_RESTRICTION_NONAME_NOREF: xmlParserErrors = 1726;
pub const XML_SCHEMAP_REF_AND_SUBTYPE: xmlParserErrors = 1725;
pub const XML_SCHEMAP_NOTYPE_NOREF: xmlParserErrors = 1724;
pub const XML_SCHEMAP_NOTATION_NO_NAME: xmlParserErrors = 1723;
pub const XML_SCHEMAP_NOATTR_NOREF: xmlParserErrors = 1722;
pub const XML_SCHEMAP_INVALID_WHITE_SPACE: xmlParserErrors = 1721;
pub const XML_SCHEMAP_INVALID_REF_AND_SUBTYPE: xmlParserErrors = 1720;
pub const XML_SCHEMAP_INVALID_MINOCCURS: xmlParserErrors = 1719;
pub const XML_SCHEMAP_INVALID_MAXOCCURS: xmlParserErrors = 1718;
pub const XML_SCHEMAP_INVALID_FACET_VALUE: xmlParserErrors = 1717;
pub const XML_SCHEMAP_INVALID_FACET: xmlParserErrors = 1716;
pub const XML_SCHEMAP_INVALID_ENUM: xmlParserErrors = 1715;
pub const XML_SCHEMAP_INVALID_BOOLEAN: xmlParserErrors = 1714;
pub const XML_SCHEMAP_IMPORT_SCHEMA_NOT_URI: xmlParserErrors = 1713;
pub const XML_SCHEMAP_IMPORT_REDEFINE_NSNAME: xmlParserErrors = 1712;
pub const XML_SCHEMAP_IMPORT_NAMESPACE_NOT_URI: xmlParserErrors = 1711;
pub const XML_SCHEMAP_GROUP_NONAME_NOREF: xmlParserErrors = 1710;
pub const XML_SCHEMAP_FAILED_BUILD_IMPORT: xmlParserErrors = 1709;
pub const XML_SCHEMAP_FACET_NO_VALUE: xmlParserErrors = 1708;
pub const XML_SCHEMAP_EXTENSION_NO_BASE: xmlParserErrors = 1707;
pub const XML_SCHEMAP_ELEM_NONAME_NOREF: xmlParserErrors = 1706;
pub const XML_SCHEMAP_ELEMFORMDEFAULT_VALUE: xmlParserErrors = 1705;
pub const XML_SCHEMAP_COMPLEXTYPE_NONAME_NOREF: xmlParserErrors = 1704;
pub const XML_SCHEMAP_ATTR_NONAME_NOREF: xmlParserErrors = 1703;
pub const XML_SCHEMAP_ATTRGRP_NONAME_NOREF: xmlParserErrors = 1702;
pub const XML_SCHEMAP_ATTRFORMDEFAULT_VALUE: xmlParserErrors = 1701;
pub const XML_SCHEMAP_PREFIX_UNDEFINED: xmlParserErrors = 1700;
pub const XML_CATALOG_RECURSION: xmlParserErrors = 1654;
pub const XML_CATALOG_NOT_CATALOG: xmlParserErrors = 1653;
pub const XML_CATALOG_PREFER_VALUE: xmlParserErrors = 1652;
pub const XML_CATALOG_ENTRY_BROKEN: xmlParserErrors = 1651;
pub const XML_CATALOG_MISSING_ATTR: xmlParserErrors = 1650;
pub const XML_XINCLUDE_FRAGMENT_ID: xmlParserErrors = 1618;
pub const XML_XINCLUDE_DEPRECATED_NS: xmlParserErrors = 1617;
pub const XML_XINCLUDE_FALLBACK_NOT_IN_INCLUDE: xmlParserErrors = 1616;
pub const XML_XINCLUDE_FALLBACKS_IN_INCLUDE: xmlParserErrors = 1615;
pub const XML_XINCLUDE_INCLUDE_IN_INCLUDE: xmlParserErrors = 1614;
pub const XML_XINCLUDE_XPTR_RESULT: xmlParserErrors = 1613;
pub const XML_XINCLUDE_XPTR_FAILED: xmlParserErrors = 1612;
pub const XML_XINCLUDE_MULTIPLE_ROOT: xmlParserErrors = 1611;
pub const XML_XINCLUDE_UNKNOWN_ENCODING: xmlParserErrors = 1610;
pub const XML_XINCLUDE_BUILD_FAILED: xmlParserErrors = 1609;
pub const XML_XINCLUDE_INVALID_CHAR: xmlParserErrors = 1608;
pub const XML_XINCLUDE_TEXT_DOCUMENT: xmlParserErrors = 1607;
pub const XML_XINCLUDE_TEXT_FRAGMENT: xmlParserErrors = 1606;
pub const XML_XINCLUDE_HREF_URI: xmlParserErrors = 1605;
pub const XML_XINCLUDE_NO_FALLBACK: xmlParserErrors = 1604;
pub const XML_XINCLUDE_NO_HREF: xmlParserErrors = 1603;
pub const XML_XINCLUDE_ENTITY_DEF_MISMATCH: xmlParserErrors = 1602;
pub const XML_XINCLUDE_PARSE_VALUE: xmlParserErrors = 1601;
pub const XML_XINCLUDE_RECURSION: xmlParserErrors = 1600;
pub const XML_IO_EAFNOSUPPORT: xmlParserErrors = 1556;
pub const XML_IO_EALREADY: xmlParserErrors = 1555;
pub const XML_IO_EADDRINUSE: xmlParserErrors = 1554;
pub const XML_IO_ENETUNREACH: xmlParserErrors = 1553;
pub const XML_IO_ECONNREFUSED: xmlParserErrors = 1552;
pub const XML_IO_EISCONN: xmlParserErrors = 1551;
pub const XML_IO_ENOTSOCK: xmlParserErrors = 1550;
pub const XML_IO_LOAD_ERROR: xmlParserErrors = 1549;
pub const XML_IO_BUFFER_FULL: xmlParserErrors = 1548;
pub const XML_IO_NO_INPUT: xmlParserErrors = 1547;
pub const XML_IO_WRITE: xmlParserErrors = 1546;
pub const XML_IO_FLUSH: xmlParserErrors = 1545;
pub const XML_IO_ENCODER: xmlParserErrors = 1544;
pub const XML_IO_NETWORK_ATTEMPT: xmlParserErrors = 1543;
pub const XML_IO_EXDEV: xmlParserErrors = 1542;
pub const XML_IO_ETIMEDOUT: xmlParserErrors = 1541;
pub const XML_IO_ESRCH: xmlParserErrors = 1540;
pub const XML_IO_ESPIPE: xmlParserErrors = 1539;
pub const XML_IO_EROFS: xmlParserErrors = 1538;
pub const XML_IO_ERANGE: xmlParserErrors = 1537;
pub const XML_IO_EPIPE: xmlParserErrors = 1536;
pub const XML_IO_EPERM: xmlParserErrors = 1535;
pub const XML_IO_ENXIO: xmlParserErrors = 1534;
pub const XML_IO_ENOTTY: xmlParserErrors = 1533;
pub const XML_IO_ENOTSUP: xmlParserErrors = 1532;
pub const XML_IO_ENOTEMPTY: xmlParserErrors = 1531;
pub const XML_IO_ENOTDIR: xmlParserErrors = 1530;
pub const XML_IO_ENOSYS: xmlParserErrors = 1529;
pub const XML_IO_ENOSPC: xmlParserErrors = 1528;
pub const XML_IO_ENOMEM: xmlParserErrors = 1527;
pub const XML_IO_ENOLCK: xmlParserErrors = 1526;
pub const XML_IO_ENOEXEC: xmlParserErrors = 1525;
pub const XML_IO_ENOENT: xmlParserErrors = 1524;
pub const XML_IO_ENODEV: xmlParserErrors = 1523;
pub const XML_IO_ENFILE: xmlParserErrors = 1522;
pub const XML_IO_ENAMETOOLONG: xmlParserErrors = 1521;
pub const XML_IO_EMSGSIZE: xmlParserErrors = 1520;
pub const XML_IO_EMLINK: xmlParserErrors = 1519;
pub const XML_IO_EMFILE: xmlParserErrors = 1518;
pub const XML_IO_EISDIR: xmlParserErrors = 1517;
pub const XML_IO_EIO: xmlParserErrors = 1516;
pub const XML_IO_EINVAL: xmlParserErrors = 1515;
pub const XML_IO_EINTR: xmlParserErrors = 1514;
pub const XML_IO_EINPROGRESS: xmlParserErrors = 1513;
pub const XML_IO_EFBIG: xmlParserErrors = 1512;
pub const XML_IO_EFAULT: xmlParserErrors = 1511;
pub const XML_IO_EEXIST: xmlParserErrors = 1510;
pub const XML_IO_EDOM: xmlParserErrors = 1509;
pub const XML_IO_EDEADLK: xmlParserErrors = 1508;
pub const XML_IO_ECHILD: xmlParserErrors = 1507;
pub const XML_IO_ECANCELED: xmlParserErrors = 1506;
pub const XML_IO_EBUSY: xmlParserErrors = 1505;
pub const XML_IO_EBADMSG: xmlParserErrors = 1504;
pub const XML_IO_EBADF: xmlParserErrors = 1503;
pub const XML_IO_EAGAIN: xmlParserErrors = 1502;
pub const XML_IO_EACCES: xmlParserErrors = 1501;
pub const XML_IO_UNKNOWN: xmlParserErrors = 1500;
pub const XML_REGEXP_COMPILE_ERROR: xmlParserErrors = 1450;
pub const XML_SAVE_UNKNOWN_ENCODING: xmlParserErrors = 1403;
pub const XML_SAVE_NO_DOCTYPE: xmlParserErrors = 1402;
pub const XML_SAVE_CHAR_INVALID: xmlParserErrors = 1401;
pub const XML_SAVE_NOT_UTF8: xmlParserErrors = 1400;
pub const XML_TREE_NOT_UTF8: xmlParserErrors = 1303;
pub const XML_TREE_UNTERMINATED_ENTITY: xmlParserErrors = 1302;
pub const XML_TREE_INVALID_DEC: xmlParserErrors = 1301;
pub const XML_TREE_INVALID_HEX: xmlParserErrors = 1300;
pub const XML_XPATH_INVALID_CHAR_ERROR: xmlParserErrors = 1221;
pub const XML_XPATH_ENCODING_ERROR: xmlParserErrors = 1220;
pub const XML_XPATH_UNDEF_PREFIX_ERROR: xmlParserErrors = 1219;
pub const XML_XPTR_SUB_RESOURCE_ERROR: xmlParserErrors = 1218;
pub const XML_XPTR_RESOURCE_ERROR: xmlParserErrors = 1217;
pub const XML_XPTR_SYNTAX_ERROR: xmlParserErrors = 1216;
pub const XML_XPATH_MEMORY_ERROR: xmlParserErrors = 1215;
pub const XML_XPATH_INVALID_CTXT_POSITION: xmlParserErrors = 1214;
pub const XML_XPATH_INVALID_CTXT_SIZE: xmlParserErrors = 1213;
pub const XML_XPATH_INVALID_ARITY: xmlParserErrors = 1212;
pub const XML_XPATH_INVALID_TYPE: xmlParserErrors = 1211;
pub const XML_XPATH_INVALID_OPERAND: xmlParserErrors = 1210;
pub const XML_XPATH_UNKNOWN_FUNC_ERROR: xmlParserErrors = 1209;
pub const XML_XPATH_UNCLOSED_ERROR: xmlParserErrors = 1208;
pub const XML_XPATH_EXPR_ERROR: xmlParserErrors = 1207;
pub const XML_XPATH_INVALID_PREDICATE_ERROR: xmlParserErrors = 1206;
pub const XML_XPATH_UNDEF_VARIABLE_ERROR: xmlParserErrors = 1205;
pub const XML_XPATH_VARIABLE_REF_ERROR: xmlParserErrors = 1204;
pub const XML_XPATH_START_LITERAL_ERROR: xmlParserErrors = 1203;
pub const XML_XPATH_UNFINISHED_LITERAL_ERROR: xmlParserErrors = 1202;
pub const XML_XPATH_NUMBER_ERROR: xmlParserErrors = 1201;
pub const XML_XPATH_EXPRESSION_OK: xmlParserErrors = 1200;
pub const XML_RNGP_XML_NS: xmlParserErrors = 1122;
pub const XML_RNGP_XMLNS_NAME: xmlParserErrors = 1121;
pub const XML_RNGP_VALUE_NO_CONTENT: xmlParserErrors = 1120;
pub const XML_RNGP_VALUE_EMPTY: xmlParserErrors = 1119;
pub const XML_RNGP_URI_NOT_ABSOLUTE: xmlParserErrors = 1118;
pub const XML_RNGP_URI_FRAGMENT: xmlParserErrors = 1117;
pub const XML_RNGP_UNKNOWN_TYPE_LIB: xmlParserErrors = 1116;
pub const XML_RNGP_UNKNOWN_CONSTRUCT: xmlParserErrors = 1115;
pub const XML_RNGP_UNKNOWN_COMBINE: xmlParserErrors = 1114;
pub const XML_RNGP_UNKNOWN_ATTRIBUTE: xmlParserErrors = 1113;
pub const XML_RNGP_TYPE_VALUE: xmlParserErrors = 1112;
pub const XML_RNGP_TYPE_NOT_FOUND: xmlParserErrors = 1111;
pub const XML_RNGP_TYPE_MISSING: xmlParserErrors = 1110;
pub const XML_RNGP_TEXT_HAS_CHILD: xmlParserErrors = 1109;
pub const XML_RNGP_TEXT_EXPECTED: xmlParserErrors = 1108;
pub const XML_RNGP_START_MISSING: xmlParserErrors = 1107;
pub const XML_RNGP_START_EMPTY: xmlParserErrors = 1106;
pub const XML_RNGP_START_CONTENT: xmlParserErrors = 1105;
pub const XML_RNGP_START_CHOICE_AND_INTERLEAVE: xmlParserErrors = 1104;
pub const XML_RNGP_REF_NOT_EMPTY: xmlParserErrors = 1103;
pub const XML_RNGP_REF_NO_NAME: xmlParserErrors = 1102;
pub const XML_RNGP_REF_NO_DEF: xmlParserErrors = 1101;
pub const XML_RNGP_REF_NAME_INVALID: xmlParserErrors = 1100;
pub const XML_RNGP_REF_CYCLE: xmlParserErrors = 1099;
pub const XML_RNGP_REF_CREATE_FAILED: xmlParserErrors = 1098;
pub const XML_RNGP_PREFIX_UNDEFINED: xmlParserErrors = 1097;
pub const XML_RNGP_PAT_START_VALUE: xmlParserErrors = 1096;
pub const XML_RNGP_PAT_START_TEXT: xmlParserErrors = 1095;
pub const XML_RNGP_PAT_START_ONEMORE: xmlParserErrors = 1094;
pub const XML_RNGP_PAT_START_LIST: xmlParserErrors = 1093;
pub const XML_RNGP_PAT_START_INTERLEAVE: xmlParserErrors = 1092;
pub const XML_RNGP_PAT_START_GROUP: xmlParserErrors = 1091;
pub const XML_RNGP_PAT_START_EMPTY: xmlParserErrors = 1090;
pub const XML_RNGP_PAT_START_DATA: xmlParserErrors = 1089;
pub const XML_RNGP_PAT_START_ATTR: xmlParserErrors = 1088;
pub const XML_RNGP_PAT_ONEMORE_INTERLEAVE_ATTR: xmlParserErrors = 1087;
pub const XML_RNGP_PAT_ONEMORE_GROUP_ATTR: xmlParserErrors = 1086;
pub const XML_RNGP_PAT_NSNAME_EXCEPT_NSNAME: xmlParserErrors = 1085;
pub const XML_RNGP_PAT_NSNAME_EXCEPT_ANYNAME: xmlParserErrors = 1084;
pub const XML_RNGP_PAT_LIST_TEXT: xmlParserErrors = 1083;
pub const XML_RNGP_PAT_LIST_REF: xmlParserErrors = 1082;
pub const XML_RNGP_PAT_LIST_LIST: xmlParserErrors = 1081;
pub const XML_RNGP_PAT_LIST_INTERLEAVE: xmlParserErrors = 1080;
pub const XML_RNGP_PAT_LIST_ELEM: xmlParserErrors = 1079;
pub const XML_RNGP_PAT_LIST_ATTR: xmlParserErrors = 1078;
pub const XML_RNGP_PAT_DATA_EXCEPT_TEXT: xmlParserErrors = 1077;
pub const XML_RNGP_PAT_DATA_EXCEPT_REF: xmlParserErrors = 1076;
pub const XML_RNGP_PAT_DATA_EXCEPT_ONEMORE: xmlParserErrors = 1075;
pub const XML_RNGP_PAT_DATA_EXCEPT_LIST: xmlParserErrors = 1074;
pub const XML_RNGP_PAT_DATA_EXCEPT_INTERLEAVE: xmlParserErrors = 1073;
pub const XML_RNGP_PAT_DATA_EXCEPT_GROUP: xmlParserErrors = 1072;
pub const XML_RNGP_PAT_DATA_EXCEPT_EMPTY: xmlParserErrors = 1071;
pub const XML_RNGP_PAT_DATA_EXCEPT_ELEM: xmlParserErrors = 1070;
pub const XML_RNGP_PAT_DATA_EXCEPT_ATTR: xmlParserErrors = 1069;
pub const XML_RNGP_PAT_ATTR_ELEM: xmlParserErrors = 1068;
pub const XML_RNGP_PAT_ATTR_ATTR: xmlParserErrors = 1067;
pub const XML_RNGP_PAT_ANYNAME_EXCEPT_ANYNAME: xmlParserErrors = 1066;
pub const XML_RNGP_PARSE_ERROR: xmlParserErrors = 1065;
pub const XML_RNGP_PARENTREF_NOT_EMPTY: xmlParserErrors = 1064;
pub const XML_RNGP_PARENTREF_NO_PARENT: xmlParserErrors = 1063;
pub const XML_RNGP_PARENTREF_NO_NAME: xmlParserErrors = 1062;
pub const XML_RNGP_PARENTREF_NAME_INVALID: xmlParserErrors = 1061;
pub const XML_RNGP_PARENTREF_CREATE_FAILED: xmlParserErrors = 1060;
pub const XML_RNGP_PARAM_NAME_MISSING: xmlParserErrors = 1059;
pub const XML_RNGP_PARAM_FORBIDDEN: xmlParserErrors = 1058;
pub const XML_RNGP_NSNAME_NO_NS: xmlParserErrors = 1057;
pub const XML_RNGP_NSNAME_ATTR_ANCESTOR: xmlParserErrors = 1056;
pub const XML_RNGP_NOTALLOWED_NOT_EMPTY: xmlParserErrors = 1055;
pub const XML_RNGP_NEED_COMBINE: xmlParserErrors = 1054;
pub const XML_RNGP_NAME_MISSING: xmlParserErrors = 1053;
pub const XML_RNGP_MISSING_HREF: xmlParserErrors = 1052;
pub const XML_RNGP_INVALID_VALUE: xmlParserErrors = 1051;
pub const XML_RNGP_INVALID_URI: xmlParserErrors = 1050;
pub const XML_RNGP_INVALID_DEFINE_NAME: xmlParserErrors = 1049;
pub const XML_RNGP_INTERLEAVE_NO_CONTENT: xmlParserErrors = 1048;
pub const XML_RNGP_INTERLEAVE_EMPTY: xmlParserErrors = 1047;
pub const XML_RNGP_INTERLEAVE_CREATE_FAILED: xmlParserErrors = 1046;
pub const XML_RNGP_INTERLEAVE_ADD: xmlParserErrors = 1045;
pub const XML_RNGP_INCLUDE_RECURSE: xmlParserErrors = 1044;
pub const XML_RNGP_INCLUDE_FAILURE: xmlParserErrors = 1043;
pub const XML_RNGP_INCLUDE_EMPTY: xmlParserErrors = 1042;
pub const XML_RNGP_HREF_ERROR: xmlParserErrors = 1041;
pub const XML_RNGP_GROUP_ATTR_CONFLICT: xmlParserErrors = 1040;
pub const XML_RNGP_GRAMMAR_NO_START: xmlParserErrors = 1039;
pub const XML_RNGP_GRAMMAR_MISSING: xmlParserErrors = 1038;
pub const XML_RNGP_GRAMMAR_EMPTY: xmlParserErrors = 1037;
pub const XML_RNGP_GRAMMAR_CONTENT: xmlParserErrors = 1036;
pub const XML_RNGP_FOREIGN_ELEMENT: xmlParserErrors = 1035;
pub const XML_RNGP_FORBIDDEN_ATTRIBUTE: xmlParserErrors = 1034;
pub const XML_RNGP_EXTERNALREF_RECURSE: xmlParserErrors = 1033;
pub const XML_RNGP_EXTERNAL_REF_FAILURE: xmlParserErrors = 1032;
pub const XML_RNGP_EXTERNALREF_EMTPY: xmlParserErrors = 1031;
pub const XML_RNGP_EXCEPT_NO_CONTENT: xmlParserErrors = 1030;
pub const XML_RNGP_EXCEPT_MULTIPLE: xmlParserErrors = 1029;
pub const XML_RNGP_EXCEPT_MISSING: xmlParserErrors = 1028;
pub const XML_RNGP_EXCEPT_EMPTY: xmlParserErrors = 1027;
pub const XML_RNGP_ERROR_TYPE_LIB: xmlParserErrors = 1026;
pub const XML_RNGP_EMPTY_NOT_EMPTY: xmlParserErrors = 1025;
pub const XML_RNGP_EMPTY_CONTENT: xmlParserErrors = 1024;
pub const XML_RNGP_EMPTY_CONSTRUCT: xmlParserErrors = 1023;
pub const XML_RNGP_EMPTY: xmlParserErrors = 1022;
pub const XML_RNGP_ELEM_TEXT_CONFLICT: xmlParserErrors = 1021;
pub const XML_RNGP_ELEMENT_NO_CONTENT: xmlParserErrors = 1020;
pub const XML_RNGP_ELEMENT_NAME: xmlParserErrors = 1019;
pub const XML_RNGP_ELEMENT_CONTENT: xmlParserErrors = 1018;
pub const XML_RNGP_ELEMENT_EMPTY: xmlParserErrors = 1017;
pub const XML_RNGP_ELEM_CONTENT_ERROR: xmlParserErrors = 1016;
pub const XML_RNGP_ELEM_CONTENT_EMPTY: xmlParserErrors = 1015;
pub const XML_RNGP_DEFINE_NAME_MISSING: xmlParserErrors = 1014;
pub const XML_RNGP_DEFINE_MISSING: xmlParserErrors = 1013;
pub const XML_RNGP_DEFINE_EMPTY: xmlParserErrors = 1012;
pub const XML_RNGP_DEFINE_CREATE_FAILED: xmlParserErrors = 1011;
pub const XML_RNGP_DEF_CHOICE_AND_INTERLEAVE: xmlParserErrors = 1010;
pub const XML_RNGP_DATA_CONTENT: xmlParserErrors = 1009;
pub const XML_RNGP_CREATE_FAILURE: xmlParserErrors = 1008;
pub const XML_RNGP_CHOICE_EMPTY: xmlParserErrors = 1007;
pub const XML_RNGP_CHOICE_CONTENT: xmlParserErrors = 1006;
pub const XML_RNGP_ATTRIBUTE_NOOP: xmlParserErrors = 1005;
pub const XML_RNGP_ATTRIBUTE_EMPTY: xmlParserErrors = 1004;
pub const XML_RNGP_ATTRIBUTE_CONTENT: xmlParserErrors = 1003;
pub const XML_RNGP_ATTRIBUTE_CHILDREN: xmlParserErrors = 1002;
pub const XML_RNGP_ATTR_CONFLICT: xmlParserErrors = 1001;
pub const XML_RNGP_ANYNAME_ATTR_ANCESTOR: xmlParserErrors = 1000;
pub const XML_HTML_UNKNOWN_TAG: xmlParserErrors = 801;
pub const XML_HTML_STRUCURE_ERROR: xmlParserErrors = 800;
pub const XML_DTD_DUP_TOKEN: xmlParserErrors = 541;
pub const XML_DTD_XMLID_TYPE: xmlParserErrors = 540;
pub const XML_DTD_XMLID_VALUE: xmlParserErrors = 539;
pub const XML_DTD_STANDALONE_DEFAULTED: xmlParserErrors = 538;
pub const XML_DTD_UNKNOWN_NOTATION: xmlParserErrors = 537;
pub const XML_DTD_UNKNOWN_ID: xmlParserErrors = 536;
pub const XML_DTD_UNKNOWN_ENTITY: xmlParserErrors = 535;
pub const XML_DTD_UNKNOWN_ELEM: xmlParserErrors = 534;
pub const XML_DTD_UNKNOWN_ATTRIBUTE: xmlParserErrors = 533;
pub const XML_DTD_STANDALONE_WHITE_SPACE: xmlParserErrors = 532;
pub const XML_DTD_ROOT_NAME: xmlParserErrors = 531;
pub const XML_DTD_NOT_STANDALONE: xmlParserErrors = 530;
pub const XML_DTD_NOT_PCDATA: xmlParserErrors = 529;
pub const XML_DTD_NOT_EMPTY: xmlParserErrors = 528;
pub const XML_DTD_NOTATION_VALUE: xmlParserErrors = 527;
pub const XML_DTD_NOTATION_REDEFINED: xmlParserErrors = 526;
pub const XML_DTD_NO_ROOT: xmlParserErrors = 525;
pub const XML_DTD_NO_PREFIX: xmlParserErrors = 524;
pub const XML_DTD_NO_ELEM_NAME: xmlParserErrors = 523;
pub const XML_DTD_NO_DTD: xmlParserErrors = 522;
pub const XML_DTD_NO_DOC: xmlParserErrors = 521;
pub const XML_DTD_MULTIPLE_ID: xmlParserErrors = 520;
pub const XML_DTD_MIXED_CORRUPT: xmlParserErrors = 519;
pub const XML_DTD_MISSING_ATTRIBUTE: xmlParserErrors = 518;
pub const XML_DTD_LOAD_ERROR: xmlParserErrors = 517;
pub const XML_DTD_INVALID_DEFAULT: xmlParserErrors = 516;
pub const XML_DTD_INVALID_CHILD: xmlParserErrors = 515;
pub const XML_DTD_ID_SUBSET: xmlParserErrors = 514;
pub const XML_DTD_ID_REDEFINED: xmlParserErrors = 513;
pub const XML_DTD_ID_FIXED: xmlParserErrors = 512;
pub const XML_DTD_ENTITY_TYPE: xmlParserErrors = 511;
pub const XML_DTD_EMPTY_NOTATION: xmlParserErrors = 510;
pub const XML_DTD_ELEM_REDEFINED: xmlParserErrors = 509;
pub const XML_DTD_ELEM_NAMESPACE: xmlParserErrors = 508;
pub const XML_DTD_ELEM_DEFAULT_NAMESPACE: xmlParserErrors = 507;
pub const XML_DTD_DIFFERENT_PREFIX: xmlParserErrors = 506;
pub const XML_DTD_CONTENT_NOT_DETERMINIST: xmlParserErrors = 505;
pub const XML_DTD_CONTENT_MODEL: xmlParserErrors = 504;
pub const XML_DTD_CONTENT_ERROR: xmlParserErrors = 503;
pub const XML_DTD_ATTRIBUTE_VALUE: xmlParserErrors = 502;
pub const XML_DTD_ATTRIBUTE_REDEFINED: xmlParserErrors = 501;
pub const XML_DTD_ATTRIBUTE_DEFAULT: xmlParserErrors = 500;
pub const XML_NS_ERR_COLON: xmlParserErrors = 205;
pub const XML_NS_ERR_EMPTY: xmlParserErrors = 204;
pub const XML_NS_ERR_ATTRIBUTE_REDEFINED: xmlParserErrors = 203;
pub const XML_NS_ERR_QNAME: xmlParserErrors = 202;
pub const XML_NS_ERR_UNDEFINED_NAMESPACE: xmlParserErrors = 201;
pub const XML_NS_ERR_XML_NAMESPACE: xmlParserErrors = 200;
pub const XML_ERR_USER_STOP: xmlParserErrors = 111;
pub const XML_ERR_NAME_TOO_LONG: xmlParserErrors = 110;
pub const XML_ERR_VERSION_MISMATCH: xmlParserErrors = 109;
pub const XML_ERR_UNKNOWN_VERSION: xmlParserErrors = 108;
pub const XML_WAR_ENTITY_REDEFINED: xmlParserErrors = 107;
pub const XML_WAR_NS_COLUMN: xmlParserErrors = 106;
pub const XML_ERR_NOTATION_PROCESSING: xmlParserErrors = 105;
pub const XML_ERR_ENTITY_PROCESSING: xmlParserErrors = 104;
pub const XML_ERR_NOT_STANDALONE: xmlParserErrors = 103;
pub const XML_WAR_SPACE_VALUE: xmlParserErrors = 102;
pub const XML_ERR_MISSING_ENCODING: xmlParserErrors = 101;
pub const XML_WAR_NS_URI_RELATIVE: xmlParserErrors = 100;
pub const XML_WAR_NS_URI: xmlParserErrors = 99;
pub const XML_WAR_LANG_VALUE: xmlParserErrors = 98;
pub const XML_WAR_UNKNOWN_VERSION: xmlParserErrors = 97;
pub const XML_ERR_VERSION_MISSING: xmlParserErrors = 96;
pub const XML_ERR_CONDSEC_INVALID_KEYWORD: xmlParserErrors = 95;
pub const XML_ERR_NO_DTD: xmlParserErrors = 94;
pub const XML_WAR_CATALOG_PI: xmlParserErrors = 93;
pub const XML_ERR_URI_FRAGMENT: xmlParserErrors = 92;
pub const XML_ERR_INVALID_URI: xmlParserErrors = 91;
pub const XML_ERR_ENTITY_BOUNDARY: xmlParserErrors = 90;
pub const XML_ERR_ENTITY_LOOP: xmlParserErrors = 89;
pub const XML_ERR_ENTITY_PE_INTERNAL: xmlParserErrors = 88;
pub const XML_ERR_ENTITY_CHAR_ERROR: xmlParserErrors = 87;
pub const XML_ERR_EXTRA_CONTENT: xmlParserErrors = 86;
pub const XML_ERR_NOT_WELL_BALANCED: xmlParserErrors = 85;
pub const XML_ERR_VALUE_REQUIRED: xmlParserErrors = 84;
pub const XML_ERR_CONDSEC_INVALID: xmlParserErrors = 83;
pub const XML_ERR_EXT_ENTITY_STANDALONE: xmlParserErrors = 82;
pub const XML_ERR_INVALID_ENCODING: xmlParserErrors = 81;
pub const XML_ERR_HYPHEN_IN_COMMENT: xmlParserErrors = 80;
pub const XML_ERR_ENCODING_NAME: xmlParserErrors = 79;
pub const XML_ERR_STANDALONE_VALUE: xmlParserErrors = 78;
pub const XML_ERR_TAG_NOT_FINISHED: xmlParserErrors = 77;
pub const XML_ERR_TAG_NAME_MISMATCH: xmlParserErrors = 76;
pub const XML_ERR_EQUAL_REQUIRED: xmlParserErrors = 75;
pub const XML_ERR_LTSLASH_REQUIRED: xmlParserErrors = 74;
pub const XML_ERR_GT_REQUIRED: xmlParserErrors = 73;
pub const XML_ERR_LT_REQUIRED: xmlParserErrors = 72;
pub const XML_ERR_PUBID_REQUIRED: xmlParserErrors = 71;
pub const XML_ERR_URI_REQUIRED: xmlParserErrors = 70;
pub const XML_ERR_PCDATA_REQUIRED: xmlParserErrors = 69;
pub const XML_ERR_NAME_REQUIRED: xmlParserErrors = 68;
pub const XML_ERR_NMTOKEN_REQUIRED: xmlParserErrors = 67;
pub const XML_ERR_SEPARATOR_REQUIRED: xmlParserErrors = 66;
pub const XML_ERR_SPACE_REQUIRED: xmlParserErrors = 65;
pub const XML_ERR_RESERVED_XML_NAME: xmlParserErrors = 64;
pub const XML_ERR_CDATA_NOT_FINISHED: xmlParserErrors = 63;
pub const XML_ERR_MISPLACED_CDATA_END: xmlParserErrors = 62;
pub const XML_ERR_DOCTYPE_NOT_FINISHED: xmlParserErrors = 61;
pub const XML_ERR_EXT_SUBSET_NOT_FINISHED: xmlParserErrors = 60;
pub const XML_ERR_CONDSEC_NOT_FINISHED: xmlParserErrors = 59;
pub const XML_ERR_CONDSEC_NOT_STARTED: xmlParserErrors = 58;
pub const XML_ERR_XMLDECL_NOT_FINISHED: xmlParserErrors = 57;
pub const XML_ERR_XMLDECL_NOT_STARTED: xmlParserErrors = 56;
pub const XML_ERR_ELEMCONTENT_NOT_FINISHED: xmlParserErrors = 55;
pub const XML_ERR_ELEMCONTENT_NOT_STARTED: xmlParserErrors = 54;
pub const XML_ERR_MIXED_NOT_FINISHED: xmlParserErrors = 53;
pub const XML_ERR_MIXED_NOT_STARTED: xmlParserErrors = 52;
pub const XML_ERR_ATTLIST_NOT_FINISHED: xmlParserErrors = 51;
pub const XML_ERR_ATTLIST_NOT_STARTED: xmlParserErrors = 50;
pub const XML_ERR_NOTATION_NOT_FINISHED: xmlParserErrors = 49;
pub const XML_ERR_NOTATION_NOT_STARTED: xmlParserErrors = 48;
pub const XML_ERR_PI_NOT_FINISHED: xmlParserErrors = 47;
pub const XML_ERR_PI_NOT_STARTED: xmlParserErrors = 46;
pub const XML_ERR_COMMENT_NOT_FINISHED: xmlParserErrors = 45;
pub const XML_ERR_LITERAL_NOT_FINISHED: xmlParserErrors = 44;
pub const XML_ERR_LITERAL_NOT_STARTED: xmlParserErrors = 43;
pub const XML_ERR_ATTRIBUTE_REDEFINED: xmlParserErrors = 42;
pub const XML_ERR_ATTRIBUTE_WITHOUT_VALUE: xmlParserErrors = 41;
pub const XML_ERR_ATTRIBUTE_NOT_FINISHED: xmlParserErrors = 40;
pub const XML_ERR_ATTRIBUTE_NOT_STARTED: xmlParserErrors = 39;
pub const XML_ERR_LT_IN_ATTRIBUTE: xmlParserErrors = 38;
pub const XML_ERR_ENTITY_NOT_FINISHED: xmlParserErrors = 37;
pub const XML_ERR_ENTITY_NOT_STARTED: xmlParserErrors = 36;
pub const XML_ERR_NS_DECL_ERROR: xmlParserErrors = 35;
pub const XML_ERR_STRING_NOT_CLOSED: xmlParserErrors = 34;
pub const XML_ERR_STRING_NOT_STARTED: xmlParserErrors = 33;
pub const XML_ERR_UNSUPPORTED_ENCODING: xmlParserErrors = 32;
pub const XML_ERR_UNKNOWN_ENCODING: xmlParserErrors = 31;
pub const XML_ERR_ENTITY_IS_PARAMETER: xmlParserErrors = 30;
pub const XML_ERR_ENTITY_IS_EXTERNAL: xmlParserErrors = 29;
pub const XML_ERR_UNPARSED_ENTITY: xmlParserErrors = 28;
pub const XML_WAR_UNDECLARED_ENTITY: xmlParserErrors = 27;
pub const XML_ERR_UNDECLARED_ENTITY: xmlParserErrors = 26;
pub const XML_ERR_PEREF_SEMICOL_MISSING: xmlParserErrors = 25;
pub const XML_ERR_PEREF_NO_NAME: xmlParserErrors = 24;
pub const XML_ERR_ENTITYREF_SEMICOL_MISSING: xmlParserErrors = 23;
pub const XML_ERR_ENTITYREF_NO_NAME: xmlParserErrors = 22;
pub const XML_ERR_PEREF_IN_INT_SUBSET: xmlParserErrors = 21;
pub const XML_ERR_PEREF_IN_EPILOG: xmlParserErrors = 20;
pub const XML_ERR_PEREF_IN_PROLOG: xmlParserErrors = 19;
pub const XML_ERR_PEREF_AT_EOF: xmlParserErrors = 18;
pub const XML_ERR_ENTITYREF_IN_DTD: xmlParserErrors = 17;
pub const XML_ERR_ENTITYREF_IN_EPILOG: xmlParserErrors = 16;
pub const XML_ERR_ENTITYREF_IN_PROLOG: xmlParserErrors = 15;
pub const XML_ERR_ENTITYREF_AT_EOF: xmlParserErrors = 14;
pub const XML_ERR_CHARREF_IN_DTD: xmlParserErrors = 13;
pub const XML_ERR_CHARREF_IN_EPILOG: xmlParserErrors = 12;
pub const XML_ERR_CHARREF_IN_PROLOG: xmlParserErrors = 11;
pub const XML_ERR_CHARREF_AT_EOF: xmlParserErrors = 10;
pub const XML_ERR_INVALID_CHAR: xmlParserErrors = 9;
pub const XML_ERR_INVALID_CHARREF: xmlParserErrors = 8;
pub const XML_ERR_INVALID_DEC_CHARREF: xmlParserErrors = 7;
pub const XML_ERR_INVALID_HEX_CHARREF: xmlParserErrors = 6;
pub const XML_ERR_DOCUMENT_END: xmlParserErrors = 5;
pub const XML_ERR_DOCUMENT_EMPTY: xmlParserErrors = 4;
pub const XML_ERR_DOCUMENT_START: xmlParserErrors = 3;
pub const XML_ERR_NO_MEMORY: xmlParserErrors = 2;
pub const XML_ERR_INTERNAL_ERROR: xmlParserErrors = 1;
pub const XML_ERR_OK: xmlParserErrors = 0;
pub type xmlGenericErrorFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const std::os::raw::c_char,
                                _: ...) -> ()>;
pub type xmlValidCtxtPtr = *mut xmlValidCtxt;
/* was the entity content checked */
/* this is also used to count entities
					 * references done from that entity
					 * and if it contains '<' */
/*
 * All entities are stored in an hash table.
 * There is 2 separate hash tables for global and parameter entities.
 */
pub type xmlEntitiesTable = _xmlHashTable;
pub type xmlEntitiesTablePtr = *mut xmlEntitiesTable;
pub type C2RustUnnamed_0 = std::os::raw::c_uint;
pub const XML_PARSE_BIG_LINES: C2RustUnnamed_0 = 4194304;
pub const XML_PARSE_IGNORE_ENC: C2RustUnnamed_0 = 2097152;
pub const XML_PARSE_OLDSAX: C2RustUnnamed_0 = 1048576;
pub const XML_PARSE_HUGE: C2RustUnnamed_0 = 524288;
pub const XML_PARSE_NOBASEFIX: C2RustUnnamed_0 = 262144;
pub const XML_PARSE_OLD10: C2RustUnnamed_0 = 131072;
pub const XML_PARSE_COMPACT: C2RustUnnamed_0 = 65536;
pub const XML_PARSE_NOXINCNODE: C2RustUnnamed_0 = 32768;
pub const XML_PARSE_NOCDATA: C2RustUnnamed_0 = 16384;
pub const XML_PARSE_NSCLEAN: C2RustUnnamed_0 = 8192;
pub const XML_PARSE_NODICT: C2RustUnnamed_0 = 4096;
pub const XML_PARSE_NONET: C2RustUnnamed_0 = 2048;
pub const XML_PARSE_XINCLUDE: C2RustUnnamed_0 = 1024;
pub const XML_PARSE_SAX1: C2RustUnnamed_0 = 512;
pub const XML_PARSE_NOBLANKS: C2RustUnnamed_0 = 256;
pub const XML_PARSE_PEDANTIC: C2RustUnnamed_0 = 128;
pub const XML_PARSE_NOWARNING: C2RustUnnamed_0 = 64;
pub const XML_PARSE_NOERROR: C2RustUnnamed_0 = 32;
pub const XML_PARSE_DTDVALID: C2RustUnnamed_0 = 16;
pub const XML_PARSE_DTDATTR: C2RustUnnamed_0 = 8;
pub const XML_PARSE_DTDLOAD: C2RustUnnamed_0 = 4;
pub const XML_PARSE_NOENT: C2RustUnnamed_0 = 2;
pub const XML_PARSE_RECOVER: C2RustUnnamed_0 = 1;
pub type htmlDocPtr = xmlDocPtr;
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
pub type xmlDebugCtxt = _xmlDebugCtxt;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlDebugCtxt {
    pub output: *mut FILE,
    pub shift: [std::os::raw::c_char; 101],
    pub depth: std::os::raw::c_int,
    pub doc: xmlDocPtr,
    pub node: xmlNodePtr,
    pub dict: xmlDictPtr,
    pub check: std::os::raw::c_int,
    pub errors: std::os::raw::c_int,
    pub nodict: std::os::raw::c_int,
    pub options: std::os::raw::c_int,
}
pub type xmlDebugCtxtPtr = *mut xmlDebugCtxt;
pub type xmlShellReadlineFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_char) -> *mut std::os::raw::c_char>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlShellCtxt {
    pub filename: *mut std::os::raw::c_char,
    pub doc: xmlDocPtr,
    pub node: xmlNodePtr,
    pub pctxt: xmlXPathContextPtr,
    pub loaded: std::os::raw::c_int,
    pub output: *mut FILE,
    pub input: xmlShellReadlineFunc,
}
pub type xmlShellCtxt = _xmlShellCtxt;
pub type xmlShellCtxtPtr = *mut xmlShellCtxt;
pub type xmlRelaxNGPtr = *mut xmlRelaxNG;
/*
 * Summary: implementation of the Relax-NG validation
 * Description: implementation of the Relax-NG validation
 *
 * Copy: See Copyright for the status of this software.
 *
 * Author: Daniel Veillard
 */
pub type xmlRelaxNG = _xmlRelaxNG;
pub type xmlRelaxNGValidCtxtPtr = *mut xmlRelaxNGValidCtxt;
pub type xmlRelaxNGValidCtxt = _xmlRelaxNGValidCtxt;
/* *
 * xmlRelaxNGValidityWarningFunc:
 * @ctx: the validation context
 * @msg: the message
 * @...: extra arguments
 *
 * Signature of a warning callback from a Relax-NG validation
 */
pub type xmlRelaxNGValidityWarningFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const std::os::raw::c_char,
                                _: ...) -> ()>;
/* *
 * xmlRelaxNGValidityErrorFunc:
 * @ctx: the validation context
 * @msg: the message
 * @...: extra arguments
 *
 * Signature of an error callback from a Relax-NG validation
 */
pub type xmlRelaxNGValidityErrorFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const std::os::raw::c_char,
                                _: ...) -> ()>;
pub type xmlRelaxNGParserCtxtPtr = *mut xmlRelaxNGParserCtxt;
/* *
 * A schemas validation context
 */
pub type xmlRelaxNGParserCtxt = _xmlRelaxNGParserCtxt;
unsafe extern "C" fn xmlCtxtDumpInitCtxt(mut ctxt: xmlDebugCtxtPtr) {
    let mut i: std::os::raw::c_int = 0;
    (*ctxt).depth = 0 as std::os::raw::c_int;
    (*ctxt).check = 0 as std::os::raw::c_int;
    (*ctxt).errors = 0 as std::os::raw::c_int;
    (*ctxt).output = stdout;
    (*ctxt).doc = 0 as xmlDocPtr;
    (*ctxt).node = 0 as xmlNodePtr;
    (*ctxt).dict = 0 as xmlDictPtr;
    (*ctxt).nodict = 0 as std::os::raw::c_int;
    (*ctxt).options = 0 as std::os::raw::c_int;
    i = 0 as std::os::raw::c_int;
    while i < 100 as std::os::raw::c_int {
        (*ctxt).shift[i as usize] = ' ' as i32 as std::os::raw::c_char;
        i += 1
    }
    (*ctxt).shift[100 as std::os::raw::c_int as usize] =
        0 as std::os::raw::c_int as std::os::raw::c_char;
}
unsafe extern "C" fn xmlCtxtDumpCleanCtxt(mut ctxt: xmlDebugCtxtPtr) {
    /* remove the ATTRIBUTE_UNUSED when this is added */
}
/* *
 * xmlNsCheckScope:
 * @node: the node
 * @ns: the namespace node
 *
 * Check that a given namespace is in scope on a node.
 *
 * Returns 1 if in scope, -1 in case of argument error,
 *         -2 if the namespace is not in scope, and -3 if not on
 *         an ancestor node.
 */
unsafe extern "C" fn xmlNsCheckScope(mut node: xmlNodePtr, mut ns: xmlNsPtr)
 -> std::os::raw::c_int {
    let mut cur: xmlNsPtr = 0 as *mut xmlNs;
    if node.is_null() || ns.is_null() { return -(1 as std::os::raw::c_int) }
    if (*node).type_0 as std::os::raw::c_uint !=
           XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
           (*node).type_0 as std::os::raw::c_uint !=
               XML_ATTRIBUTE_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
           (*node).type_0 as std::os::raw::c_uint !=
               XML_DOCUMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
           (*node).type_0 as std::os::raw::c_uint !=
               XML_TEXT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
           (*node).type_0 as std::os::raw::c_uint !=
               XML_HTML_DOCUMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
           (*node).type_0 as std::os::raw::c_uint !=
               XML_XINCLUDE_START as std::os::raw::c_int as std::os::raw::c_uint {
        return -(2 as std::os::raw::c_int)
    }
    while !node.is_null() &&
              ((*node).type_0 as std::os::raw::c_uint ==
                   XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint ||
                   (*node).type_0 as std::os::raw::c_uint ==
                       XML_ATTRIBUTE_NODE as std::os::raw::c_int as std::os::raw::c_uint ||
                   (*node).type_0 as std::os::raw::c_uint ==
                       XML_TEXT_NODE as std::os::raw::c_int as std::os::raw::c_uint ||
                   (*node).type_0 as std::os::raw::c_uint ==
                       XML_XINCLUDE_START as std::os::raw::c_int as std::os::raw::c_uint) {
        if (*node).type_0 as std::os::raw::c_uint ==
               XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint ||
               (*node).type_0 as std::os::raw::c_uint ==
                   XML_XINCLUDE_START as std::os::raw::c_int as std::os::raw::c_uint {
            cur = (*node).nsDef;
            while !cur.is_null() {
                if cur == ns { return 1 as std::os::raw::c_int }
                if xmlStrEqual((*cur).prefix, (*ns).prefix) != 0 {
                    return -(2 as std::os::raw::c_int)
                }
                cur = (*cur).next
            }
        }
        node = (*node).parent
    }
    /* the xml namespace may be declared on the document node */
    if !node.is_null() &&
           ((*node).type_0 as std::os::raw::c_uint ==
                XML_DOCUMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint ||
                (*node).type_0 as std::os::raw::c_uint ==
                    XML_HTML_DOCUMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint) {
        let mut oldNs: xmlNsPtr = (*(node as xmlDocPtr)).oldNs;
        if oldNs == ns { return 1 as std::os::raw::c_int }
    }
    return -(3 as std::os::raw::c_int);
}
unsafe extern "C" fn xmlCtxtDumpSpaces(mut ctxt: xmlDebugCtxtPtr) {
    if (*ctxt).check != 0 { return }
    if !(*ctxt).output.is_null() && (*ctxt).depth > 0 as std::os::raw::c_int {
        if (*ctxt).depth < 50 as std::os::raw::c_int {
            fprintf((*ctxt).output,
                    b"%s\x00" as *const u8 as *const std::os::raw::c_char,
                    &mut *(*ctxt).shift.as_mut_ptr().offset((100 as
                                                                 std::os::raw::c_int -
                                                                 2 as
                                                                     std::os::raw::c_int
                                                                     *
                                                                     (*ctxt).depth)
                                                                as isize) as
                        *mut std::os::raw::c_char);
        } else {
            fprintf((*ctxt).output,
                    b"%s\x00" as *const u8 as *const std::os::raw::c_char,
                    (*ctxt).shift.as_mut_ptr());
        }
    };
}
/* *
 * xmlDebugErr:
 * @ctxt:  a debug context
 * @error:  the error code
 *
 * Handle a debug error.
 */
unsafe extern "C" fn xmlDebugErr(mut ctxt: xmlDebugCtxtPtr,
                                 mut error: std::os::raw::c_int,
                                 mut msg: *const std::os::raw::c_char) {
    (*ctxt).errors += 1;
    __xmlRaiseError(None, None, 0 as *mut std::os::raw::c_void,
                    0 as *mut std::os::raw::c_void, (*ctxt).node as *mut std::os::raw::c_void,
                    XML_FROM_CHECK as std::os::raw::c_int, error, XML_ERR_ERROR,
                    0 as *const std::os::raw::c_char, 0 as std::os::raw::c_int,
                    0 as *const std::os::raw::c_char, 0 as *const std::os::raw::c_char,
                    0 as *const std::os::raw::c_char, 0 as std::os::raw::c_int,
                    0 as std::os::raw::c_int,
                    b"%s\x00" as *const u8 as *const std::os::raw::c_char, msg);
}
unsafe extern "C" fn xmlDebugErr2(mut ctxt: xmlDebugCtxtPtr,
                                  mut error: std::os::raw::c_int,
                                  mut msg: *const std::os::raw::c_char,
                                  mut extra: std::os::raw::c_int) {
    (*ctxt).errors += 1;
    __xmlRaiseError(None, None, 0 as *mut std::os::raw::c_void,
                    0 as *mut std::os::raw::c_void, (*ctxt).node as *mut std::os::raw::c_void,
                    XML_FROM_CHECK as std::os::raw::c_int, error, XML_ERR_ERROR,
                    0 as *const std::os::raw::c_char, 0 as std::os::raw::c_int,
                    0 as *const std::os::raw::c_char, 0 as *const std::os::raw::c_char,
                    0 as *const std::os::raw::c_char, 0 as std::os::raw::c_int,
                    0 as std::os::raw::c_int, msg, extra);
}
unsafe extern "C" fn xmlDebugErr3(mut ctxt: xmlDebugCtxtPtr,
                                  mut error: std::os::raw::c_int,
                                  mut msg: *const std::os::raw::c_char,
                                  mut extra: *const std::os::raw::c_char) {
    (*ctxt).errors += 1;
    __xmlRaiseError(None, None, 0 as *mut std::os::raw::c_void,
                    0 as *mut std::os::raw::c_void, (*ctxt).node as *mut std::os::raw::c_void,
                    XML_FROM_CHECK as std::os::raw::c_int, error, XML_ERR_ERROR,
                    0 as *const std::os::raw::c_char, 0 as std::os::raw::c_int,
                    0 as *const std::os::raw::c_char, 0 as *const std::os::raw::c_char,
                    0 as *const std::os::raw::c_char, 0 as std::os::raw::c_int,
                    0 as std::os::raw::c_int, msg, extra);
}
/* *
 * xmlCtxtNsCheckScope:
 * @ctxt: the debugging context
 * @node: the node
 * @ns: the namespace node
 *
 * Report if a given namespace is is not in scope.
 */
unsafe extern "C" fn xmlCtxtNsCheckScope(mut ctxt: xmlDebugCtxtPtr,
                                         mut node: xmlNodePtr,
                                         mut ns: xmlNsPtr) {
    let mut ret: std::os::raw::c_int = 0;
    ret = xmlNsCheckScope(node, ns);
    if ret == -(2 as std::os::raw::c_int) {
        if (*ns).prefix.is_null() {
            xmlDebugErr(ctxt, XML_CHECK_NS_SCOPE as std::os::raw::c_int,
                        b"Reference to default namespace not in scope\n\x00"
                            as *const u8 as *const std::os::raw::c_char);
        } else {
            xmlDebugErr3(ctxt, XML_CHECK_NS_SCOPE as std::os::raw::c_int,
                         b"Reference to namespace \'%s\' not in scope\n\x00"
                             as *const u8 as *const std::os::raw::c_char,
                         (*ns).prefix as *mut std::os::raw::c_char);
        }
    }
    if ret == -(3 as std::os::raw::c_int) {
        if (*ns).prefix.is_null() {
            xmlDebugErr(ctxt, XML_CHECK_NS_ANCESTOR as std::os::raw::c_int,
                        b"Reference to default namespace not on ancestor\n\x00"
                            as *const u8 as *const std::os::raw::c_char);
        } else {
            xmlDebugErr3(ctxt, XML_CHECK_NS_ANCESTOR as std::os::raw::c_int,
                         b"Reference to namespace \'%s\' not on ancestor\n\x00"
                             as *const u8 as *const std::os::raw::c_char,
                         (*ns).prefix as *mut std::os::raw::c_char);
        }
    };
}
/* *
 * xmlCtxtCheckString:
 * @ctxt: the debug context
 * @str: the string
 *
 * Do debugging on the string, currently it just checks the UTF-8 content
 */
unsafe extern "C" fn xmlCtxtCheckString(mut ctxt: xmlDebugCtxtPtr,
                                        mut str: *const xmlChar) {
    if str.is_null() { return }
    if (*ctxt).check != 0 {
        if xmlCheckUTF8(str) == 0 {
            xmlDebugErr3(ctxt, XML_CHECK_NOT_UTF8 as std::os::raw::c_int,
                         b"String is not UTF-8 %s\x00" as *const u8 as
                             *const std::os::raw::c_char, str as *const std::os::raw::c_char);
        }
    };
}
/* *
 * xmlCtxtCheckName:
 * @ctxt: the debug context
 * @name: the name
 *
 * Do debugging on the name, for example the dictionary status and
 * conformance to the Name production.
 */
unsafe extern "C" fn xmlCtxtCheckName(mut ctxt: xmlDebugCtxtPtr,
                                      mut name: *const xmlChar) {
    if (*ctxt).check != 0 {
        if name.is_null() {
            xmlDebugErr(ctxt, XML_CHECK_NO_NAME as std::os::raw::c_int,
                        b"Name is NULL\x00" as *const u8 as
                            *const std::os::raw::c_char);
            return
        }
        if xmlValidateName(name, 0 as std::os::raw::c_int) != 0 {
            xmlDebugErr3(ctxt, XML_CHECK_NOT_NCNAME as std::os::raw::c_int,
                         b"Name is not an NCName \'%s\'\x00" as *const u8 as
                             *const std::os::raw::c_char,
                         name as *const std::os::raw::c_char);
        }
        if !(*ctxt).dict.is_null() && xmlDictOwns((*ctxt).dict, name) == 0 &&
               ((*ctxt).doc.is_null() ||
                    (*(*ctxt).doc).parseFlags &
                        (XML_PARSE_SAX1 as std::os::raw::c_int |
                             XML_PARSE_NODICT as std::os::raw::c_int) ==
                        0 as std::os::raw::c_int) {
            xmlDebugErr3(ctxt, XML_CHECK_OUTSIDE_DICT as std::os::raw::c_int,
                         b"Name is not from the document dictionary \'%s\'\x00"
                             as *const u8 as *const std::os::raw::c_char,
                         name as *const std::os::raw::c_char);
        }
    };
}
unsafe extern "C" fn xmlCtxtGenericNodeCheck(mut ctxt: xmlDebugCtxtPtr,
                                             mut node: xmlNodePtr) {
    let mut doc: xmlDocPtr = 0 as *mut xmlDoc;
    let mut dict: xmlDictPtr = 0 as *mut xmlDict;
    doc = (*node).doc;
    if (*node).parent.is_null() {
        xmlDebugErr(ctxt, XML_CHECK_NO_PARENT as std::os::raw::c_int,
                    b"Node has no parent\n\x00" as *const u8 as
                        *const std::os::raw::c_char);
    }
    if (*node).doc.is_null() {
        xmlDebugErr(ctxt, XML_CHECK_NO_DOC as std::os::raw::c_int,
                    b"Node has no doc\n\x00" as *const u8 as
                        *const std::os::raw::c_char);
        dict = 0 as xmlDictPtr
    } else {
        dict = (*doc).dict;
        if dict.is_null() && (*ctxt).nodict == 0 as std::os::raw::c_int {
            (*ctxt).nodict = 1 as std::os::raw::c_int
        }
        if (*ctxt).doc.is_null() { (*ctxt).doc = doc }
        if (*ctxt).dict.is_null() { (*ctxt).dict = dict }
    }
    if !(*node).parent.is_null() && (*node).doc != (*(*node).parent).doc &&
           xmlStrEqual((*node).name,
                       b"pseudoroot\x00" as *const u8 as *const std::os::raw::c_char
                           as *mut xmlChar) == 0 {
        xmlDebugErr(ctxt, XML_CHECK_WRONG_DOC as std::os::raw::c_int,
                    b"Node doc differs from parent\'s one\n\x00" as *const u8
                        as *const std::os::raw::c_char);
    }
    if (*node).prev.is_null() {
        if (*node).type_0 as std::os::raw::c_uint ==
               XML_ATTRIBUTE_NODE as std::os::raw::c_int as std::os::raw::c_uint {
            if !(*node).parent.is_null() &&
                   node != (*(*node).parent).properties as xmlNodePtr {
                xmlDebugErr(ctxt, XML_CHECK_NO_PREV as std::os::raw::c_int,
                            b"Attr has no prev and not first of attr list\n\x00"
                                as *const u8 as *const std::os::raw::c_char);
            }
        } else if !(*node).parent.is_null() &&
                      (*(*node).parent).children != node {
            xmlDebugErr(ctxt, XML_CHECK_NO_PREV as std::os::raw::c_int,
                        b"Node has no prev and not first of parent list\n\x00"
                            as *const u8 as *const std::os::raw::c_char);
        }
    } else if (*(*node).prev).next != node {
        xmlDebugErr(ctxt, XML_CHECK_WRONG_PREV as std::os::raw::c_int,
                    b"Node prev->next : back link wrong\n\x00" as *const u8 as
                        *const std::os::raw::c_char);
    }
    if (*node).next.is_null() {
        if !(*node).parent.is_null() &&
               (*node).type_0 as std::os::raw::c_uint !=
                   XML_ATTRIBUTE_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
               (*(*node).parent).last != node &&
               (*(*node).parent).type_0 as std::os::raw::c_uint ==
                   XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
            xmlDebugErr(ctxt, XML_CHECK_NO_NEXT as std::os::raw::c_int,
                        b"Node has no next and not last of parent list\n\x00"
                            as *const u8 as *const std::os::raw::c_char);
        }
    } else {
        if (*(*node).next).prev != node {
            xmlDebugErr(ctxt, XML_CHECK_WRONG_NEXT as std::os::raw::c_int,
                        b"Node next->prev : forward link wrong\n\x00" as
                            *const u8 as *const std::os::raw::c_char);
        }
        if (*(*node).next).parent != (*node).parent {
            xmlDebugErr(ctxt, XML_CHECK_WRONG_PARENT as std::os::raw::c_int,
                        b"Node next->prev : forward link wrong\n\x00" as
                            *const u8 as *const std::os::raw::c_char);
        }
    }
    if (*node).type_0 as std::os::raw::c_uint ==
           XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        let mut ns: xmlNsPtr = 0 as *mut xmlNs;
        ns = (*node).nsDef;
        while !ns.is_null() {
            xmlCtxtNsCheckScope(ctxt, node, ns);
            ns = (*ns).next
        }
        if !(*node).ns.is_null() {
            xmlCtxtNsCheckScope(ctxt, node, (*node).ns);
        }
    } else if (*node).type_0 as std::os::raw::c_uint ==
                  XML_ATTRIBUTE_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        if !(*node).ns.is_null() {
            xmlCtxtNsCheckScope(ctxt, node, (*node).ns);
        }
    }
    if (*node).type_0 as std::os::raw::c_uint !=
           XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
           (*node).type_0 as std::os::raw::c_uint !=
               XML_ATTRIBUTE_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
           (*node).type_0 as std::os::raw::c_uint !=
               XML_ELEMENT_DECL as std::os::raw::c_int as std::os::raw::c_uint &&
           (*node).type_0 as std::os::raw::c_uint !=
               XML_ATTRIBUTE_DECL as std::os::raw::c_int as std::os::raw::c_uint &&
           (*node).type_0 as std::os::raw::c_uint !=
               XML_DTD_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
           (*node).type_0 as std::os::raw::c_uint !=
               XML_HTML_DOCUMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
           (*node).type_0 as std::os::raw::c_uint !=
               XML_DOCUMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        if !(*node).content.is_null() {
            xmlCtxtCheckString(ctxt, (*node).content as *const xmlChar);
        }
    }
    match (*node).type_0 as std::os::raw::c_uint {
        1 | 2 => { xmlCtxtCheckName(ctxt, (*node).name); }
        3 => {
            if !((*node).name == xmlStringText.as_ptr() ||
                     (*node).name == xmlStringTextNoenc.as_ptr()) {
                /* some case of entity substitution can lead to this */
                if !(!(*ctxt).dict.is_null() &&
                         (*node).name ==
                             xmlDictLookup((*ctxt).dict,
                                           b"nbktext\x00" as *const u8 as
                                               *const std::os::raw::c_char as
                                               *mut xmlChar,
                                           7 as std::os::raw::c_int)) {
                    xmlDebugErr3(ctxt, XML_CHECK_WRONG_NAME as std::os::raw::c_int,
                                 b"Text node has wrong name \'%s\'\x00" as
                                     *const u8 as *const std::os::raw::c_char,
                                 (*node).name as *const std::os::raw::c_char);
                }
            }
        }
        8 => {
            if !((*node).name == xmlStringComment.as_ptr()) {
                xmlDebugErr3(ctxt, XML_CHECK_WRONG_NAME as std::os::raw::c_int,
                             b"Comment node has wrong name \'%s\'\x00" as
                                 *const u8 as *const std::os::raw::c_char,
                             (*node).name as *const std::os::raw::c_char);
            }
        }
        7 => { xmlCtxtCheckName(ctxt, (*node).name); }
        4 => {
            if !(*node).name.is_null() {
                xmlDebugErr3(ctxt, XML_CHECK_NAME_NOT_NULL as std::os::raw::c_int,
                             b"CData section has non NULL name \'%s\'\x00" as
                                 *const u8 as *const std::os::raw::c_char,
                             (*node).name as *const std::os::raw::c_char);
            }
        }
        5 | 6 | 10 | 11 | 12 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 9 | 13
        | _ => {
        }
    };
}
unsafe extern "C" fn xmlCtxtDumpString(mut ctxt: xmlDebugCtxtPtr,
                                       mut str: *const xmlChar) {
    let mut i: std::os::raw::c_int = 0;
    if (*ctxt).check != 0 { return }
    /* TODO: check UTF8 content of the string */
    if str.is_null() {
        fprintf((*ctxt).output,
                b"(NULL)\x00" as *const u8 as *const std::os::raw::c_char);
        return
    }
    i = 0 as std::os::raw::c_int;
    while i < 40 as std::os::raw::c_int {
        if *str.offset(i as isize) as std::os::raw::c_int == 0 as std::os::raw::c_int {
            return
        } else {
            if *str.offset(i as isize) as std::os::raw::c_int == 0x20 as std::os::raw::c_int
                   ||
                   0x9 as std::os::raw::c_int <=
                       *str.offset(i as isize) as std::os::raw::c_int &&
                       *str.offset(i as isize) as std::os::raw::c_int <=
                           0xa as std::os::raw::c_int ||
                   *str.offset(i as isize) as std::os::raw::c_int ==
                       0xd as std::os::raw::c_int {
                fputc(' ' as i32, (*ctxt).output);
            } else if *str.offset(i as isize) as std::os::raw::c_int >=
                          0x80 as std::os::raw::c_int {
                fprintf((*ctxt).output,
                        b"#%X\x00" as *const u8 as *const std::os::raw::c_char,
                        *str.offset(i as isize) as std::os::raw::c_int);
            } else {
                fputc(*str.offset(i as isize) as std::os::raw::c_int, (*ctxt).output);
            }
        }
        i += 1
    }
    fprintf((*ctxt).output, b"...\x00" as *const u8 as *const std::os::raw::c_char);
}
unsafe extern "C" fn xmlCtxtDumpDtdNode(mut ctxt: xmlDebugCtxtPtr,
                                        mut dtd: xmlDtdPtr) {
    xmlCtxtDumpSpaces(ctxt);
    if dtd.is_null() {
        if (*ctxt).check == 0 {
            fprintf((*ctxt).output,
                    b"DTD node is NULL\n\x00" as *const u8 as
                        *const std::os::raw::c_char);
        }
        return
    }
    if (*dtd).type_0 as std::os::raw::c_uint !=
           XML_DTD_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        xmlDebugErr(ctxt, XML_CHECK_NOT_DTD as std::os::raw::c_int,
                    b"Node is not a DTD\x00" as *const u8 as
                        *const std::os::raw::c_char);
        return
    }
    if (*ctxt).check == 0 {
        if !(*dtd).name.is_null() {
            fprintf((*ctxt).output,
                    b"DTD(%s)\x00" as *const u8 as *const std::os::raw::c_char,
                    (*dtd).name as *mut std::os::raw::c_char);
        } else {
            fprintf((*ctxt).output,
                    b"DTD\x00" as *const u8 as *const std::os::raw::c_char);
        }
        if !(*dtd).ExternalID.is_null() {
            fprintf((*ctxt).output,
                    b", PUBLIC %s\x00" as *const u8 as *const std::os::raw::c_char,
                    (*dtd).ExternalID as *mut std::os::raw::c_char);
        }
        if !(*dtd).SystemID.is_null() {
            fprintf((*ctxt).output,
                    b", SYSTEM %s\x00" as *const u8 as *const std::os::raw::c_char,
                    (*dtd).SystemID as *mut std::os::raw::c_char);
        }
        fprintf((*ctxt).output,
                b"\n\x00" as *const u8 as *const std::os::raw::c_char);
    }
    /*
     * Do a bit of checking
     */
    xmlCtxtGenericNodeCheck(ctxt, dtd as xmlNodePtr);
}
unsafe extern "C" fn xmlCtxtDumpAttrDecl(mut ctxt: xmlDebugCtxtPtr,
                                         mut attr: xmlAttributePtr) {
    xmlCtxtDumpSpaces(ctxt);
    if attr.is_null() {
        if (*ctxt).check == 0 {
            fprintf((*ctxt).output,
                    b"Attribute declaration is NULL\n\x00" as *const u8 as
                        *const std::os::raw::c_char);
        }
        return
    }
    if (*attr).type_0 as std::os::raw::c_uint !=
           XML_ATTRIBUTE_DECL as std::os::raw::c_int as std::os::raw::c_uint {
        xmlDebugErr(ctxt, XML_CHECK_NOT_ATTR_DECL as std::os::raw::c_int,
                    b"Node is not an attribute declaration\x00" as *const u8
                        as *const std::os::raw::c_char);
        return
    }
    if !(*attr).name.is_null() {
        if (*ctxt).check == 0 {
            fprintf((*ctxt).output,
                    b"ATTRDECL(%s)\x00" as *const u8 as *const std::os::raw::c_char,
                    (*attr).name as *mut std::os::raw::c_char);
        }
    } else {
        xmlDebugErr(ctxt, XML_CHECK_NO_NAME as std::os::raw::c_int,
                    b"Node attribute declaration has no name\x00" as *const u8
                        as *const std::os::raw::c_char);
    }
    if !(*attr).elem.is_null() {
        if (*ctxt).check == 0 {
            fprintf((*ctxt).output,
                    b" for %s\x00" as *const u8 as *const std::os::raw::c_char,
                    (*attr).elem as *mut std::os::raw::c_char);
        }
    } else {
        xmlDebugErr(ctxt, XML_CHECK_NO_ELEM as std::os::raw::c_int,
                    b"Node attribute declaration has no element name\x00" as
                        *const u8 as *const std::os::raw::c_char);
    }
    if (*ctxt).check == 0 {
        match (*attr).atype as std::os::raw::c_uint {
            1 => {
                fprintf((*ctxt).output,
                        b" CDATA\x00" as *const u8 as *const std::os::raw::c_char);
            }
            2 => {
                fprintf((*ctxt).output,
                        b" ID\x00" as *const u8 as *const std::os::raw::c_char);
            }
            3 => {
                fprintf((*ctxt).output,
                        b" IDREF\x00" as *const u8 as *const std::os::raw::c_char);
            }
            4 => {
                fprintf((*ctxt).output,
                        b" IDREFS\x00" as *const u8 as *const std::os::raw::c_char);
            }
            5 => {
                fprintf((*ctxt).output,
                        b" ENTITY\x00" as *const u8 as *const std::os::raw::c_char);
            }
            6 => {
                fprintf((*ctxt).output,
                        b" ENTITIES\x00" as *const u8 as *const std::os::raw::c_char);
            }
            7 => {
                fprintf((*ctxt).output,
                        b" NMTOKEN\x00" as *const u8 as *const std::os::raw::c_char);
            }
            8 => {
                fprintf((*ctxt).output,
                        b" NMTOKENS\x00" as *const u8 as *const std::os::raw::c_char);
            }
            9 => {
                fprintf((*ctxt).output,
                        b" ENUMERATION\x00" as *const u8 as
                            *const std::os::raw::c_char);
            }
            10 => {
                fprintf((*ctxt).output,
                        b" NOTATION \x00" as *const u8 as
                            *const std::os::raw::c_char);
            }
            _ => { }
        }
        if !(*attr).tree.is_null() {
            let mut indx: std::os::raw::c_int = 0;
            let mut cur: xmlEnumerationPtr = (*attr).tree;
            indx = 0 as std::os::raw::c_int;
            while indx < 5 as std::os::raw::c_int {
                if indx != 0 as std::os::raw::c_int {
                    fprintf((*ctxt).output,
                            b"|%s\x00" as *const u8 as *const std::os::raw::c_char,
                            (*cur).name as *mut std::os::raw::c_char);
                } else {
                    fprintf((*ctxt).output,
                            b" (%s\x00" as *const u8 as *const std::os::raw::c_char,
                            (*cur).name as *mut std::os::raw::c_char);
                }
                cur = (*cur).next;
                if cur.is_null() { break ; }
                indx += 1
            }
            if cur.is_null() {
                fprintf((*ctxt).output,
                        b")\x00" as *const u8 as *const std::os::raw::c_char);
            } else {
                fprintf((*ctxt).output,
                        b"...)\x00" as *const u8 as *const std::os::raw::c_char);
            }
        }
        match (*attr).def as std::os::raw::c_uint {
            2 => {
                fprintf((*ctxt).output,
                        b" REQUIRED\x00" as *const u8 as *const std::os::raw::c_char);
            }
            3 => {
                fprintf((*ctxt).output,
                        b" IMPLIED\x00" as *const u8 as *const std::os::raw::c_char);
            }
            4 => {
                fprintf((*ctxt).output,
                        b" FIXED\x00" as *const u8 as *const std::os::raw::c_char);
            }
            1 | _ => { }
        }
        if !(*attr).defaultValue.is_null() {
            fprintf((*ctxt).output,
                    b"\"\x00" as *const u8 as *const std::os::raw::c_char);
            xmlCtxtDumpString(ctxt, (*attr).defaultValue);
            fprintf((*ctxt).output,
                    b"\"\x00" as *const u8 as *const std::os::raw::c_char);
        }
        fprintf((*ctxt).output,
                b"\n\x00" as *const u8 as *const std::os::raw::c_char);
    }
    /*
     * Do a bit of checking
     */
    xmlCtxtGenericNodeCheck(ctxt, attr as xmlNodePtr);
}
unsafe extern "C" fn xmlCtxtDumpElemDecl(mut ctxt: xmlDebugCtxtPtr,
                                         mut elem: xmlElementPtr) {
    xmlCtxtDumpSpaces(ctxt);
    if elem.is_null() {
        if (*ctxt).check == 0 {
            fprintf((*ctxt).output,
                    b"Element declaration is NULL\n\x00" as *const u8 as
                        *const std::os::raw::c_char);
        }
        return
    }
    if (*elem).type_0 as std::os::raw::c_uint !=
           XML_ELEMENT_DECL as std::os::raw::c_int as std::os::raw::c_uint {
        xmlDebugErr(ctxt, XML_CHECK_NOT_ELEM_DECL as std::os::raw::c_int,
                    b"Node is not an element declaration\x00" as *const u8 as
                        *const std::os::raw::c_char);
        return
    }
    if !(*elem).name.is_null() {
        if (*ctxt).check == 0 {
            fprintf((*ctxt).output,
                    b"ELEMDECL(\x00" as *const u8 as *const std::os::raw::c_char);
            xmlCtxtDumpString(ctxt, (*elem).name);
            fprintf((*ctxt).output,
                    b")\x00" as *const u8 as *const std::os::raw::c_char);
        }
    } else {
        xmlDebugErr(ctxt, XML_CHECK_NO_NAME as std::os::raw::c_int,
                    b"Element declaration has no name\x00" as *const u8 as
                        *const std::os::raw::c_char);
    }
    if (*ctxt).check == 0 {
        match (*elem).etype as std::os::raw::c_uint {
            0 => {
                fprintf((*ctxt).output,
                        b", UNDEFINED\x00" as *const u8 as
                            *const std::os::raw::c_char);
            }
            1 => {
                fprintf((*ctxt).output,
                        b", EMPTY\x00" as *const u8 as *const std::os::raw::c_char);
            }
            2 => {
                fprintf((*ctxt).output,
                        b", ANY\x00" as *const u8 as *const std::os::raw::c_char);
            }
            3 => {
                fprintf((*ctxt).output,
                        b", MIXED \x00" as *const u8 as *const std::os::raw::c_char);
            }
            4 => {
                fprintf((*ctxt).output,
                        b", MIXED \x00" as *const u8 as *const std::os::raw::c_char);
            }
            _ => { }
        }
        if (*elem).type_0 as std::os::raw::c_uint !=
               XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
               !(*elem).content.is_null() {
            let mut buf: [std::os::raw::c_char; 5001] = [0; 5001];
            buf[0 as std::os::raw::c_int as usize] = 0 as std::os::raw::c_int as std::os::raw::c_char;
            xmlSnprintfElementContent(buf.as_mut_ptr(), 5000 as std::os::raw::c_int,
                                      (*elem).content, 1 as std::os::raw::c_int);
            buf[5000 as std::os::raw::c_int as usize] =
                0 as std::os::raw::c_int as std::os::raw::c_char;
            fprintf((*ctxt).output,
                    b"%s\x00" as *const u8 as *const std::os::raw::c_char,
                    buf.as_mut_ptr());
        }
        fprintf((*ctxt).output,
                b"\n\x00" as *const u8 as *const std::os::raw::c_char);
    }
    /*
     * Do a bit of checking
     */
    xmlCtxtGenericNodeCheck(ctxt, elem as xmlNodePtr);
}
unsafe extern "C" fn xmlCtxtDumpEntityDecl(mut ctxt: xmlDebugCtxtPtr,
                                           mut ent: xmlEntityPtr) {
    xmlCtxtDumpSpaces(ctxt);
    if ent.is_null() {
        if (*ctxt).check == 0 {
            fprintf((*ctxt).output,
                    b"Entity declaration is NULL\n\x00" as *const u8 as
                        *const std::os::raw::c_char);
        }
        return
    }
    if (*ent).type_0 as std::os::raw::c_uint !=
           XML_ENTITY_DECL as std::os::raw::c_int as std::os::raw::c_uint {
        xmlDebugErr(ctxt, XML_CHECK_NOT_ENTITY_DECL as std::os::raw::c_int,
                    b"Node is not an entity declaration\x00" as *const u8 as
                        *const std::os::raw::c_char);
        return
    }
    if !(*ent).name.is_null() {
        if (*ctxt).check == 0 {
            fprintf((*ctxt).output,
                    b"ENTITYDECL(\x00" as *const u8 as *const std::os::raw::c_char);
            xmlCtxtDumpString(ctxt, (*ent).name);
            fprintf((*ctxt).output,
                    b")\x00" as *const u8 as *const std::os::raw::c_char);
        }
    } else {
        xmlDebugErr(ctxt, XML_CHECK_NO_NAME as std::os::raw::c_int,
                    b"Entity declaration has no name\x00" as *const u8 as
                        *const std::os::raw::c_char);
    }
    if (*ctxt).check == 0 {
        match (*ent).etype as std::os::raw::c_uint {
            1 => {
                fprintf((*ctxt).output,
                        b", internal\n\x00" as *const u8 as
                            *const std::os::raw::c_char);
            }
            2 => {
                fprintf((*ctxt).output,
                        b", external parsed\n\x00" as *const u8 as
                            *const std::os::raw::c_char);
            }
            3 => {
                fprintf((*ctxt).output,
                        b", unparsed\n\x00" as *const u8 as
                            *const std::os::raw::c_char);
            }
            4 => {
                fprintf((*ctxt).output,
                        b", parameter\n\x00" as *const u8 as
                            *const std::os::raw::c_char);
            }
            5 => {
                fprintf((*ctxt).output,
                        b", external parameter\n\x00" as *const u8 as
                            *const std::os::raw::c_char);
            }
            6 => {
                fprintf((*ctxt).output,
                        b", predefined\n\x00" as *const u8 as
                            *const std::os::raw::c_char);
            }
            _ => { }
        }
        if !(*ent).ExternalID.is_null() {
            xmlCtxtDumpSpaces(ctxt);
            fprintf((*ctxt).output,
                    b" ExternalID=%s\n\x00" as *const u8 as
                        *const std::os::raw::c_char,
                    (*ent).ExternalID as *mut std::os::raw::c_char);
        }
        if !(*ent).SystemID.is_null() {
            xmlCtxtDumpSpaces(ctxt);
            fprintf((*ctxt).output,
                    b" SystemID=%s\n\x00" as *const u8 as *const std::os::raw::c_char,
                    (*ent).SystemID as *mut std::os::raw::c_char);
        }
        if !(*ent).URI.is_null() {
            xmlCtxtDumpSpaces(ctxt);
            fprintf((*ctxt).output,
                    b" URI=%s\n\x00" as *const u8 as *const std::os::raw::c_char,
                    (*ent).URI as *mut std::os::raw::c_char);
        }
        if !(*ent).content.is_null() {
            xmlCtxtDumpSpaces(ctxt);
            fprintf((*ctxt).output,
                    b" content=\x00" as *const u8 as *const std::os::raw::c_char);
            xmlCtxtDumpString(ctxt, (*ent).content);
            fprintf((*ctxt).output,
                    b"\n\x00" as *const u8 as *const std::os::raw::c_char);
        }
    }
    /*
     * Do a bit of checking
     */
    xmlCtxtGenericNodeCheck(ctxt, ent as xmlNodePtr);
}
unsafe extern "C" fn xmlCtxtDumpNamespace(mut ctxt: xmlDebugCtxtPtr,
                                          mut ns: xmlNsPtr) {
    xmlCtxtDumpSpaces(ctxt);
    if ns.is_null() {
        if (*ctxt).check == 0 {
            fprintf((*ctxt).output,
                    b"namespace node is NULL\n\x00" as *const u8 as
                        *const std::os::raw::c_char);
        }
        return
    }
    if (*ns).type_0 as std::os::raw::c_uint !=
           XML_NAMESPACE_DECL as std::os::raw::c_int as std::os::raw::c_uint {
        xmlDebugErr(ctxt, XML_CHECK_NOT_NS_DECL as std::os::raw::c_int,
                    b"Node is not a namespace declaration\x00" as *const u8 as
                        *const std::os::raw::c_char);
        return
    }
    if (*ns).href.is_null() {
        if !(*ns).prefix.is_null() {
            xmlDebugErr3(ctxt, XML_CHECK_NO_HREF as std::os::raw::c_int,
                         b"Incomplete namespace %s href=NULL\n\x00" as
                             *const u8 as *const std::os::raw::c_char,
                         (*ns).prefix as *mut std::os::raw::c_char);
        } else {
            xmlDebugErr(ctxt, XML_CHECK_NO_HREF as std::os::raw::c_int,
                        b"Incomplete default namespace href=NULL\n\x00" as
                            *const u8 as *const std::os::raw::c_char);
        }
    } else if (*ctxt).check == 0 {
        if !(*ns).prefix.is_null() {
            fprintf((*ctxt).output,
                    b"namespace %s href=\x00" as *const u8 as
                        *const std::os::raw::c_char,
                    (*ns).prefix as *mut std::os::raw::c_char);
        } else {
            fprintf((*ctxt).output,
                    b"default namespace href=\x00" as *const u8 as
                        *const std::os::raw::c_char);
        }
        xmlCtxtDumpString(ctxt, (*ns).href);
        fprintf((*ctxt).output,
                b"\n\x00" as *const u8 as *const std::os::raw::c_char);
    };
}
unsafe extern "C" fn xmlCtxtDumpNamespaceList(mut ctxt: xmlDebugCtxtPtr,
                                              mut ns: xmlNsPtr) {
    while !ns.is_null() { xmlCtxtDumpNamespace(ctxt, ns); ns = (*ns).next };
}
unsafe extern "C" fn xmlCtxtDumpEntity(mut ctxt: xmlDebugCtxtPtr,
                                       mut ent: xmlEntityPtr) {
    xmlCtxtDumpSpaces(ctxt);
    if ent.is_null() {
        if (*ctxt).check == 0 {
            fprintf((*ctxt).output,
                    b"Entity is NULL\n\x00" as *const u8 as
                        *const std::os::raw::c_char);
        }
        return
    }
    if (*ctxt).check == 0 {
        match (*ent).etype as std::os::raw::c_uint {
            1 => {
                fprintf((*ctxt).output,
                        b"INTERNAL_GENERAL_ENTITY \x00" as *const u8 as
                            *const std::os::raw::c_char);
            }
            2 => {
                fprintf((*ctxt).output,
                        b"EXTERNAL_GENERAL_PARSED_ENTITY \x00" as *const u8 as
                            *const std::os::raw::c_char);
            }
            3 => {
                fprintf((*ctxt).output,
                        b"EXTERNAL_GENERAL_UNPARSED_ENTITY \x00" as *const u8
                            as *const std::os::raw::c_char);
            }
            4 => {
                fprintf((*ctxt).output,
                        b"INTERNAL_PARAMETER_ENTITY \x00" as *const u8 as
                            *const std::os::raw::c_char);
            }
            5 => {
                fprintf((*ctxt).output,
                        b"EXTERNAL_PARAMETER_ENTITY \x00" as *const u8 as
                            *const std::os::raw::c_char);
            }
            _ => {
                fprintf((*ctxt).output,
                        b"ENTITY_%d ! \x00" as *const u8 as
                            *const std::os::raw::c_char, (*ent).etype as std::os::raw::c_int);
            }
        }
        fprintf((*ctxt).output,
                b"%s\n\x00" as *const u8 as *const std::os::raw::c_char, (*ent).name);
        if !(*ent).ExternalID.is_null() {
            xmlCtxtDumpSpaces(ctxt);
            fprintf((*ctxt).output,
                    b"ExternalID=%s\n\x00" as *const u8 as
                        *const std::os::raw::c_char,
                    (*ent).ExternalID as *mut std::os::raw::c_char);
        }
        if !(*ent).SystemID.is_null() {
            xmlCtxtDumpSpaces(ctxt);
            fprintf((*ctxt).output,
                    b"SystemID=%s\n\x00" as *const u8 as *const std::os::raw::c_char,
                    (*ent).SystemID as *mut std::os::raw::c_char);
        }
        if !(*ent).URI.is_null() {
            xmlCtxtDumpSpaces(ctxt);
            fprintf((*ctxt).output,
                    b"URI=%s\n\x00" as *const u8 as *const std::os::raw::c_char,
                    (*ent).URI as *mut std::os::raw::c_char);
        }
        if !(*ent).content.is_null() {
            xmlCtxtDumpSpaces(ctxt);
            fprintf((*ctxt).output,
                    b"content=\x00" as *const u8 as *const std::os::raw::c_char);
            xmlCtxtDumpString(ctxt, (*ent).content);
            fprintf((*ctxt).output,
                    b"\n\x00" as *const u8 as *const std::os::raw::c_char);
        }
    };
}
/* *
 * xmlCtxtDumpAttr:
 * @output:  the FILE * for the output
 * @attr:  the attribute
 * @depth:  the indentation level.
 *
 * Dumps debug information for the attribute
 */
unsafe extern "C" fn xmlCtxtDumpAttr(mut ctxt: xmlDebugCtxtPtr,
                                     mut attr: xmlAttrPtr) {
    xmlCtxtDumpSpaces(ctxt);
    if attr.is_null() {
        if (*ctxt).check == 0 {
            fprintf((*ctxt).output,
                    b"Attr is NULL\x00" as *const u8 as *const std::os::raw::c_char);
        }
        return
    }
    if (*ctxt).check == 0 {
        fprintf((*ctxt).output,
                b"ATTRIBUTE \x00" as *const u8 as *const std::os::raw::c_char);
        xmlCtxtDumpString(ctxt, (*attr).name);
        fprintf((*ctxt).output,
                b"\n\x00" as *const u8 as *const std::os::raw::c_char);
        if !(*attr).children.is_null() {
            (*ctxt).depth += 1;
            xmlCtxtDumpNodeList(ctxt, (*attr).children);
            (*ctxt).depth -= 1
        }
    }
    if (*attr).name.is_null() {
        xmlDebugErr(ctxt, XML_CHECK_NO_NAME as std::os::raw::c_int,
                    b"Attribute has no name\x00" as *const u8 as
                        *const std::os::raw::c_char);
    }
    /*
     * Do a bit of checking
     */
    xmlCtxtGenericNodeCheck(ctxt, attr as xmlNodePtr);
}
/* *
 * xmlCtxtDumpAttrList:
 * @output:  the FILE * for the output
 * @attr:  the attribute list
 * @depth:  the indentation level.
 *
 * Dumps debug information for the attribute list
 */
unsafe extern "C" fn xmlCtxtDumpAttrList(mut ctxt: xmlDebugCtxtPtr,
                                         mut attr: xmlAttrPtr) {
    while !attr.is_null() {
        xmlCtxtDumpAttr(ctxt, attr);
        attr = (*attr).next
    };
}
/* *
 * xmlCtxtDumpOneNode:
 * @output:  the FILE * for the output
 * @node:  the node
 * @depth:  the indentation level.
 *
 * Dumps debug information for the element node, it is not recursive
 */
unsafe extern "C" fn xmlCtxtDumpOneNode(mut ctxt: xmlDebugCtxtPtr,
                                        mut node: xmlNodePtr) {
    if node.is_null() {
        if (*ctxt).check == 0 {
            xmlCtxtDumpSpaces(ctxt);
            fprintf((*ctxt).output,
                    b"node is NULL\n\x00" as *const u8 as
                        *const std::os::raw::c_char);
        }
        return
    }
    (*ctxt).node = node;
    match (*node).type_0 as std::os::raw::c_uint {
        1 => {
            if (*ctxt).check == 0 {
                xmlCtxtDumpSpaces(ctxt);
                fprintf((*ctxt).output,
                        b"ELEMENT \x00" as *const u8 as *const std::os::raw::c_char);
                if !(*node).ns.is_null() && !(*(*node).ns).prefix.is_null() {
                    xmlCtxtDumpString(ctxt, (*(*node).ns).prefix);
                    fprintf((*ctxt).output,
                            b":\x00" as *const u8 as *const std::os::raw::c_char);
                }
                xmlCtxtDumpString(ctxt, (*node).name);
                fprintf((*ctxt).output,
                        b"\n\x00" as *const u8 as *const std::os::raw::c_char);
            }
        }
        2 => {
            if (*ctxt).check == 0 { xmlCtxtDumpSpaces(ctxt); }
            fprintf((*ctxt).output,
                    b"Error, ATTRIBUTE found here\n\x00" as *const u8 as
                        *const std::os::raw::c_char);
            xmlCtxtGenericNodeCheck(ctxt, node);
            return
        }
        3 => {
            if (*ctxt).check == 0 {
                xmlCtxtDumpSpaces(ctxt);
                if (*node).name == xmlStringTextNoenc.as_ptr() {
                    fprintf((*ctxt).output,
                            b"TEXT no enc\x00" as *const u8 as
                                *const std::os::raw::c_char);
                } else {
                    fprintf((*ctxt).output,
                            b"TEXT\x00" as *const u8 as *const std::os::raw::c_char);
                }
                if (*ctxt).options & 1 as std::os::raw::c_int != 0 {
                    if (*node).content ==
                           &mut (*node).properties as *mut *mut _xmlAttr as
                               *mut xmlChar {
                        fprintf((*ctxt).output,
                                b" compact\n\x00" as *const u8 as
                                    *const std::os::raw::c_char);
                    } else if xmlDictOwns((*ctxt).dict, (*node).content) ==
                                  1 as std::os::raw::c_int {
                        fprintf((*ctxt).output,
                                b" interned\n\x00" as *const u8 as
                                    *const std::os::raw::c_char);
                    } else {
                        fprintf((*ctxt).output,
                                b"\n\x00" as *const u8 as
                                    *const std::os::raw::c_char);
                    }
                } else {
                    fprintf((*ctxt).output,
                            b"\n\x00" as *const u8 as *const std::os::raw::c_char);
                }
            }
        }
        4 => {
            if (*ctxt).check == 0 {
                xmlCtxtDumpSpaces(ctxt);
                fprintf((*ctxt).output,
                        b"CDATA_SECTION\n\x00" as *const u8 as
                            *const std::os::raw::c_char);
            }
        }
        5 => {
            if (*ctxt).check == 0 {
                xmlCtxtDumpSpaces(ctxt);
                fprintf((*ctxt).output,
                        b"ENTITY_REF(%s)\n\x00" as *const u8 as
                            *const std::os::raw::c_char,
                        (*node).name as *mut std::os::raw::c_char);
            }
        }
        6 => {
            if (*ctxt).check == 0 {
                xmlCtxtDumpSpaces(ctxt);
                fprintf((*ctxt).output,
                        b"ENTITY\n\x00" as *const u8 as *const std::os::raw::c_char);
            }
        }
        7 => {
            if (*ctxt).check == 0 {
                xmlCtxtDumpSpaces(ctxt);
                fprintf((*ctxt).output,
                        b"PI %s\n\x00" as *const u8 as *const std::os::raw::c_char,
                        (*node).name as *mut std::os::raw::c_char);
            }
        }
        8 => {
            if (*ctxt).check == 0 {
                xmlCtxtDumpSpaces(ctxt);
                fprintf((*ctxt).output,
                        b"COMMENT\n\x00" as *const u8 as *const std::os::raw::c_char);
            }
        }
        9 | 13 => {
            if (*ctxt).check == 0 { xmlCtxtDumpSpaces(ctxt); }
            fprintf((*ctxt).output,
                    b"Error, DOCUMENT found here\n\x00" as *const u8 as
                        *const std::os::raw::c_char);
            xmlCtxtGenericNodeCheck(ctxt, node);
            return
        }
        10 => {
            if (*ctxt).check == 0 {
                xmlCtxtDumpSpaces(ctxt);
                fprintf((*ctxt).output,
                        b"DOCUMENT_TYPE\n\x00" as *const u8 as
                            *const std::os::raw::c_char);
            }
        }
        11 => {
            if (*ctxt).check == 0 {
                xmlCtxtDumpSpaces(ctxt);
                fprintf((*ctxt).output,
                        b"DOCUMENT_FRAG\n\x00" as *const u8 as
                            *const std::os::raw::c_char);
            }
        }
        12 => {
            if (*ctxt).check == 0 {
                xmlCtxtDumpSpaces(ctxt);
                fprintf((*ctxt).output,
                        b"NOTATION\n\x00" as *const u8 as
                            *const std::os::raw::c_char);
            }
        }
        14 => { xmlCtxtDumpDtdNode(ctxt, node as xmlDtdPtr); return }
        15 => { xmlCtxtDumpElemDecl(ctxt, node as xmlElementPtr); return }
        16 => { xmlCtxtDumpAttrDecl(ctxt, node as xmlAttributePtr); return }
        17 => { xmlCtxtDumpEntityDecl(ctxt, node as xmlEntityPtr); return }
        18 => { xmlCtxtDumpNamespace(ctxt, node as xmlNsPtr); return }
        19 => {
            if (*ctxt).check == 0 {
                xmlCtxtDumpSpaces(ctxt);
                fprintf((*ctxt).output,
                        b"INCLUDE START\n\x00" as *const u8 as
                            *const std::os::raw::c_char);
            }
            return
        }
        20 => {
            if (*ctxt).check == 0 {
                xmlCtxtDumpSpaces(ctxt);
                fprintf((*ctxt).output,
                        b"INCLUDE END\n\x00" as *const u8 as
                            *const std::os::raw::c_char);
            }
            return
        }
        _ => {
            if (*ctxt).check == 0 { xmlCtxtDumpSpaces(ctxt); }
            xmlDebugErr2(ctxt, XML_CHECK_UNKNOWN_NODE as std::os::raw::c_int,
                         b"Unknown node type %d\n\x00" as *const u8 as
                             *const std::os::raw::c_char,
                         (*node).type_0 as std::os::raw::c_int);
            return
        }
    }
    if (*node).doc.is_null() {
        if (*ctxt).check == 0 { xmlCtxtDumpSpaces(ctxt); }
        fprintf((*ctxt).output,
                b"PBM: doc == NULL !!!\n\x00" as *const u8 as
                    *const std::os::raw::c_char);
    }
    (*ctxt).depth += 1;
    if (*node).type_0 as std::os::raw::c_uint ==
           XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
           !(*node).nsDef.is_null() {
        xmlCtxtDumpNamespaceList(ctxt, (*node).nsDef);
    }
    if (*node).type_0 as std::os::raw::c_uint ==
           XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
           !(*node).properties.is_null() {
        xmlCtxtDumpAttrList(ctxt, (*node).properties);
    }
    if (*node).type_0 as std::os::raw::c_uint !=
           XML_ENTITY_REF_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        if (*node).type_0 as std::os::raw::c_uint !=
               XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
               !(*node).content.is_null() {
            if (*ctxt).check == 0 {
                xmlCtxtDumpSpaces(ctxt);
                fprintf((*ctxt).output,
                        b"content=\x00" as *const u8 as *const std::os::raw::c_char);
                xmlCtxtDumpString(ctxt, (*node).content);
                fprintf((*ctxt).output,
                        b"\n\x00" as *const u8 as *const std::os::raw::c_char);
            }
        }
    } else {
        let mut ent: xmlEntityPtr = 0 as *mut xmlEntity;
        ent = xmlGetDocEntity((*node).doc, (*node).name);
        if !ent.is_null() { xmlCtxtDumpEntity(ctxt, ent); }
    }
    (*ctxt).depth -= 1;
    /*
     * Do a bit of checking
     */
    xmlCtxtGenericNodeCheck(ctxt, node);
}
/* *
 * xmlCtxtDumpNode:
 * @output:  the FILE * for the output
 * @node:  the node
 * @depth:  the indentation level.
 *
 * Dumps debug information for the element node, it is recursive
 */
unsafe extern "C" fn xmlCtxtDumpNode(mut ctxt: xmlDebugCtxtPtr,
                                     mut node: xmlNodePtr) {
    if node.is_null() {
        if (*ctxt).check == 0 {
            xmlCtxtDumpSpaces(ctxt);
            fprintf((*ctxt).output,
                    b"node is NULL\n\x00" as *const u8 as
                        *const std::os::raw::c_char);
        }
        return
    }
    xmlCtxtDumpOneNode(ctxt, node);
    if (*node).type_0 as std::os::raw::c_uint !=
           XML_NAMESPACE_DECL as std::os::raw::c_int as std::os::raw::c_uint &&
           !(*node).children.is_null() &&
           (*node).type_0 as std::os::raw::c_uint !=
               XML_ENTITY_REF_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        (*ctxt).depth += 1;
        xmlCtxtDumpNodeList(ctxt, (*node).children);
        (*ctxt).depth -= 1
    };
}
/* options */
/* *
 * xmlCtxtDumpNodeList:
 * @output:  the FILE * for the output
 * @node:  the node list
 * @depth:  the indentation level.
 *
 * Dumps debug information for the list of element node, it is recursive
 */
unsafe extern "C" fn xmlCtxtDumpNodeList(mut ctxt: xmlDebugCtxtPtr,
                                         mut node: xmlNodePtr) {
    while !node.is_null() {
        xmlCtxtDumpNode(ctxt, node);
        node = (*node).next
    };
}
unsafe extern "C" fn xmlCtxtDumpDocHead(mut ctxt: xmlDebugCtxtPtr,
                                        mut doc: xmlDocPtr) {
    if doc.is_null() {
        if (*ctxt).check == 0 {
            fprintf((*ctxt).output,
                    b"DOCUMENT == NULL !\n\x00" as *const u8 as
                        *const std::os::raw::c_char);
        }
        return
    }
    (*ctxt).node = doc as xmlNodePtr;
    match (*doc).type_0 as std::os::raw::c_uint {
        1 => {
            xmlDebugErr(ctxt, XML_CHECK_FOUND_ELEMENT as std::os::raw::c_int,
                        b"Misplaced ELEMENT node\n\x00" as *const u8 as
                            *const std::os::raw::c_char);
        }
        2 => {
            xmlDebugErr(ctxt, XML_CHECK_FOUND_ATTRIBUTE as std::os::raw::c_int,
                        b"Misplaced ATTRIBUTE node\n\x00" as *const u8 as
                            *const std::os::raw::c_char);
        }
        3 => {
            xmlDebugErr(ctxt, XML_CHECK_FOUND_TEXT as std::os::raw::c_int,
                        b"Misplaced TEXT node\n\x00" as *const u8 as
                            *const std::os::raw::c_char);
        }
        4 => {
            xmlDebugErr(ctxt, XML_CHECK_FOUND_CDATA as std::os::raw::c_int,
                        b"Misplaced CDATA node\n\x00" as *const u8 as
                            *const std::os::raw::c_char);
        }
        5 => {
            xmlDebugErr(ctxt, XML_CHECK_FOUND_ENTITYREF as std::os::raw::c_int,
                        b"Misplaced ENTITYREF node\n\x00" as *const u8 as
                            *const std::os::raw::c_char);
        }
        6 => {
            xmlDebugErr(ctxt, XML_CHECK_FOUND_ENTITY as std::os::raw::c_int,
                        b"Misplaced ENTITY node\n\x00" as *const u8 as
                            *const std::os::raw::c_char);
        }
        7 => {
            xmlDebugErr(ctxt, XML_CHECK_FOUND_PI as std::os::raw::c_int,
                        b"Misplaced PI node\n\x00" as *const u8 as
                            *const std::os::raw::c_char);
        }
        8 => {
            xmlDebugErr(ctxt, XML_CHECK_FOUND_COMMENT as std::os::raw::c_int,
                        b"Misplaced COMMENT node\n\x00" as *const u8 as
                            *const std::os::raw::c_char);
        }
        9 => {
            if (*ctxt).check == 0 {
                fprintf((*ctxt).output,
                        b"DOCUMENT\n\x00" as *const u8 as
                            *const std::os::raw::c_char);
            }
        }
        13 => {
            if (*ctxt).check == 0 {
                fprintf((*ctxt).output,
                        b"HTML DOCUMENT\n\x00" as *const u8 as
                            *const std::os::raw::c_char);
            }
        }
        10 => {
            xmlDebugErr(ctxt, XML_CHECK_FOUND_DOCTYPE as std::os::raw::c_int,
                        b"Misplaced DOCTYPE node\n\x00" as *const u8 as
                            *const std::os::raw::c_char);
        }
        11 => {
            xmlDebugErr(ctxt, XML_CHECK_FOUND_FRAGMENT as std::os::raw::c_int,
                        b"Misplaced FRAGMENT node\n\x00" as *const u8 as
                            *const std::os::raw::c_char);
        }
        12 => {
            xmlDebugErr(ctxt, XML_CHECK_FOUND_NOTATION as std::os::raw::c_int,
                        b"Misplaced NOTATION node\n\x00" as *const u8 as
                            *const std::os::raw::c_char);
        }
        _ => {
            xmlDebugErr2(ctxt, XML_CHECK_UNKNOWN_NODE as std::os::raw::c_int,
                         b"Unknown node type %d\n\x00" as *const u8 as
                             *const std::os::raw::c_char,
                         (*doc).type_0 as std::os::raw::c_int);
        }
    };
}
/* *
 * xmlCtxtDumpDocumentHead:
 * @output:  the FILE * for the output
 * @doc:  the document
 *
 * Dumps debug information cncerning the document, not recursive
 */
unsafe extern "C" fn xmlCtxtDumpDocumentHead(mut ctxt: xmlDebugCtxtPtr,
                                             mut doc: xmlDocPtr) {
    if doc.is_null() { return }
    xmlCtxtDumpDocHead(ctxt, doc);
    if (*ctxt).check == 0 {
        if !(*doc).name.is_null() {
            fprintf((*ctxt).output,
                    b"name=\x00" as *const u8 as *const std::os::raw::c_char);
            xmlCtxtDumpString(ctxt, (*doc).name as *mut xmlChar);
            fprintf((*ctxt).output,
                    b"\n\x00" as *const u8 as *const std::os::raw::c_char);
        }
        if !(*doc).version.is_null() {
            fprintf((*ctxt).output,
                    b"version=\x00" as *const u8 as *const std::os::raw::c_char);
            xmlCtxtDumpString(ctxt, (*doc).version);
            fprintf((*ctxt).output,
                    b"\n\x00" as *const u8 as *const std::os::raw::c_char);
        }
        if !(*doc).encoding.is_null() {
            fprintf((*ctxt).output,
                    b"encoding=\x00" as *const u8 as *const std::os::raw::c_char);
            xmlCtxtDumpString(ctxt, (*doc).encoding);
            fprintf((*ctxt).output,
                    b"\n\x00" as *const u8 as *const std::os::raw::c_char);
        }
        if !(*doc).URL.is_null() {
            fprintf((*ctxt).output,
                    b"URL=\x00" as *const u8 as *const std::os::raw::c_char);
            xmlCtxtDumpString(ctxt, (*doc).URL);
            fprintf((*ctxt).output,
                    b"\n\x00" as *const u8 as *const std::os::raw::c_char);
        }
        if (*doc).standalone != 0 {
            fprintf((*ctxt).output,
                    b"standalone=true\n\x00" as *const u8 as
                        *const std::os::raw::c_char);
        }
    }
    if !(*doc).oldNs.is_null() {
        xmlCtxtDumpNamespaceList(ctxt, (*doc).oldNs);
    };
}
/* *
 * xmlCtxtDumpDocument:
 * @output:  the FILE * for the output
 * @doc:  the document
 *
 * Dumps debug information for the document, it's recursive
 */
unsafe extern "C" fn xmlCtxtDumpDocument(mut ctxt: xmlDebugCtxtPtr,
                                         mut doc: xmlDocPtr) {
    if doc.is_null() {
        if (*ctxt).check == 0 {
            fprintf((*ctxt).output,
                    b"DOCUMENT == NULL !\n\x00" as *const u8 as
                        *const std::os::raw::c_char);
        }
        return
    }
    xmlCtxtDumpDocumentHead(ctxt, doc);
    if ((*doc).type_0 as std::os::raw::c_uint ==
            XML_DOCUMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint ||
            (*doc).type_0 as std::os::raw::c_uint ==
                XML_HTML_DOCUMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint) &&
           !(*doc).children.is_null() {
        (*ctxt).depth += 1;
        xmlCtxtDumpNodeList(ctxt, (*doc).children);
        (*ctxt).depth -= 1
    };
}
unsafe extern "C" fn xmlCtxtDumpEntityCallback(mut payload: *mut std::os::raw::c_void,
                                               mut data: *mut std::os::raw::c_void,
                                               mut name: *const xmlChar) {
    let mut cur: xmlEntityPtr = payload as xmlEntityPtr;
    let mut ctxt: xmlDebugCtxtPtr = data as xmlDebugCtxtPtr;
    if cur.is_null() {
        if (*ctxt).check == 0 {
            fprintf((*ctxt).output,
                    b"Entity is NULL\x00" as *const u8 as
                        *const std::os::raw::c_char);
        }
        return
    }
    if (*ctxt).check == 0 {
        fprintf((*ctxt).output,
                b"%s : \x00" as *const u8 as *const std::os::raw::c_char,
                (*cur).name as *mut std::os::raw::c_char);
        match (*cur).etype as std::os::raw::c_uint {
            1 => {
                fprintf((*ctxt).output,
                        b"INTERNAL GENERAL, \x00" as *const u8 as
                            *const std::os::raw::c_char);
            }
            2 => {
                fprintf((*ctxt).output,
                        b"EXTERNAL PARSED, \x00" as *const u8 as
                            *const std::os::raw::c_char);
            }
            3 => {
                fprintf((*ctxt).output,
                        b"EXTERNAL UNPARSED, \x00" as *const u8 as
                            *const std::os::raw::c_char);
            }
            4 => {
                fprintf((*ctxt).output,
                        b"INTERNAL PARAMETER, \x00" as *const u8 as
                            *const std::os::raw::c_char);
            }
            5 => {
                fprintf((*ctxt).output,
                        b"EXTERNAL PARAMETER, \x00" as *const u8 as
                            *const std::os::raw::c_char);
            }
            _ => {
                xmlDebugErr2(ctxt, XML_CHECK_ENTITY_TYPE as std::os::raw::c_int,
                             b"Unknown entity type %d\n\x00" as *const u8 as
                                 *const std::os::raw::c_char,
                             (*cur).etype as std::os::raw::c_int);
            }
        }
        if !(*cur).ExternalID.is_null() {
            fprintf((*ctxt).output,
                    b"ID \"%s\"\x00" as *const u8 as *const std::os::raw::c_char,
                    (*cur).ExternalID as *mut std::os::raw::c_char);
        }
        if !(*cur).SystemID.is_null() {
            fprintf((*ctxt).output,
                    b"SYSTEM \"%s\"\x00" as *const u8 as *const std::os::raw::c_char,
                    (*cur).SystemID as *mut std::os::raw::c_char);
        }
        if !(*cur).orig.is_null() {
            fprintf((*ctxt).output,
                    b"\n orig \"%s\"\x00" as *const u8 as *const std::os::raw::c_char,
                    (*cur).orig as *mut std::os::raw::c_char);
        }
        if (*cur).type_0 as std::os::raw::c_uint !=
               XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
               !(*cur).content.is_null() {
            fprintf((*ctxt).output,
                    b"\n content \"%s\"\x00" as *const u8 as
                        *const std::os::raw::c_char,
                    (*cur).content as *mut std::os::raw::c_char);
        }
        fprintf((*ctxt).output,
                b"\n\x00" as *const u8 as *const std::os::raw::c_char);
    };
}
/* *
 * xmlCtxtDumpEntities:
 * @output:  the FILE * for the output
 * @doc:  the document
 *
 * Dumps debug information for all the entities in use by the document
 */
unsafe extern "C" fn xmlCtxtDumpEntities(mut ctxt: xmlDebugCtxtPtr,
                                         mut doc: xmlDocPtr) {
    if doc.is_null() { return }
    xmlCtxtDumpDocHead(ctxt, doc);
    if !(*doc).intSubset.is_null() && !(*(*doc).intSubset).entities.is_null()
       {
        let mut table: xmlEntitiesTablePtr =
            (*(*doc).intSubset).entities as xmlEntitiesTablePtr;
        if (*ctxt).check == 0 {
            fprintf((*ctxt).output,
                    b"Entities in internal subset\n\x00" as *const u8 as
                        *const std::os::raw::c_char);
        }
        xmlHashScan(table,
                    Some(xmlCtxtDumpEntityCallback as
                             unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                                  _: *mut std::os::raw::c_void,
                                                  _: *const xmlChar) -> ()),
                    ctxt as *mut std::os::raw::c_void);
    } else {
        fprintf((*ctxt).output,
                b"No entities in internal subset\n\x00" as *const u8 as
                    *const std::os::raw::c_char);
    }
    if !(*doc).extSubset.is_null() && !(*(*doc).extSubset).entities.is_null()
       {
        let mut table_0: xmlEntitiesTablePtr =
            (*(*doc).extSubset).entities as xmlEntitiesTablePtr;
        if (*ctxt).check == 0 {
            fprintf((*ctxt).output,
                    b"Entities in external subset\n\x00" as *const u8 as
                        *const std::os::raw::c_char);
        }
        xmlHashScan(table_0,
                    Some(xmlCtxtDumpEntityCallback as
                             unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                                  _: *mut std::os::raw::c_void,
                                                  _: *const xmlChar) -> ()),
                    ctxt as *mut std::os::raw::c_void);
    } else if (*ctxt).check == 0 {
        fprintf((*ctxt).output,
                b"No entities in external subset\n\x00" as *const u8 as
                    *const std::os::raw::c_char);
    };
}
/* *
 * xmlCtxtDumpDTD:
 * @output:  the FILE * for the output
 * @dtd:  the DTD
 *
 * Dumps debug information for the DTD
 */
unsafe extern "C" fn xmlCtxtDumpDTD(mut ctxt: xmlDebugCtxtPtr,
                                    mut dtd: xmlDtdPtr) {
    if dtd.is_null() {
        if (*ctxt).check == 0 {
            fprintf((*ctxt).output,
                    b"DTD is NULL\n\x00" as *const u8 as *const std::os::raw::c_char);
        }
        return
    }
    xmlCtxtDumpDtdNode(ctxt, dtd);
    if (*dtd).children.is_null() {
        fprintf((*ctxt).output,
                b"    DTD is empty\n\x00" as *const u8 as
                    *const std::os::raw::c_char);
    } else {
        (*ctxt).depth += 1;
        xmlCtxtDumpNodeList(ctxt, (*dtd).children);
        (*ctxt).depth -= 1
    };
}
/* ***********************************************************************
 *									*
 *			Public entry points for dump			*
 *									*
 ************************************************************************/
/* *
 * xmlDebugDumpString:
 * @output:  the FILE * for the output
 * @str:  the string
 *
 * Dumps informations about the string, shorten it if necessary
 */
#[no_mangle]
pub unsafe extern "C" fn xmlDebugDumpString(mut output: *mut FILE,
                                            mut str: *const xmlChar) {
    let mut i: std::os::raw::c_int = 0;
    if output.is_null() { output = stdout }
    if str.is_null() {
        fprintf(output, b"(NULL)\x00" as *const u8 as *const std::os::raw::c_char);
        return
    }
    i = 0 as std::os::raw::c_int;
    while i < 40 as std::os::raw::c_int {
        if *str.offset(i as isize) as std::os::raw::c_int == 0 as std::os::raw::c_int {
            return
        } else {
            if *str.offset(i as isize) as std::os::raw::c_int == 0x20 as std::os::raw::c_int
                   ||
                   0x9 as std::os::raw::c_int <=
                       *str.offset(i as isize) as std::os::raw::c_int &&
                       *str.offset(i as isize) as std::os::raw::c_int <=
                           0xa as std::os::raw::c_int ||
                   *str.offset(i as isize) as std::os::raw::c_int ==
                       0xd as std::os::raw::c_int {
                fputc(' ' as i32, output);
            } else if *str.offset(i as isize) as std::os::raw::c_int >=
                          0x80 as std::os::raw::c_int {
                fprintf(output,
                        b"#%X\x00" as *const u8 as *const std::os::raw::c_char,
                        *str.offset(i as isize) as std::os::raw::c_int);
            } else { fputc(*str.offset(i as isize) as std::os::raw::c_int, output); }
        }
        i += 1
    }
    fprintf(output, b"...\x00" as *const u8 as *const std::os::raw::c_char);
}
/* *
 * xmlDebugDumpAttr:
 * @output:  the FILE * for the output
 * @attr:  the attribute
 * @depth:  the indentation level.
 *
 * Dumps debug information for the attribute
 */
#[no_mangle]
pub unsafe extern "C" fn xmlDebugDumpAttr(mut output: *mut FILE,
                                          mut attr: xmlAttrPtr,
                                          mut depth: std::os::raw::c_int) {
    let mut ctxt: xmlDebugCtxt =
        xmlDebugCtxt{output: 0 as *mut FILE,
                     shift: [0; 101],
                     depth: 0,
                     doc: 0 as *mut xmlDoc,
                     node: 0 as *mut xmlNode,
                     dict: 0 as *mut xmlDict,
                     check: 0,
                     errors: 0,
                     nodict: 0,
                     options: 0,};
    if output.is_null() { return }
    xmlCtxtDumpInitCtxt(&mut ctxt);
    ctxt.output = output;
    ctxt.depth = depth;
    xmlCtxtDumpAttr(&mut ctxt, attr);
    xmlCtxtDumpCleanCtxt(&mut ctxt);
}
/* *
 * xmlDebugDumpEntities:
 * @output:  the FILE * for the output
 * @doc:  the document
 *
 * Dumps debug information for all the entities in use by the document
 */
#[no_mangle]
pub unsafe extern "C" fn xmlDebugDumpEntities(mut output: *mut FILE,
                                              mut doc: xmlDocPtr) {
    let mut ctxt: xmlDebugCtxt =
        xmlDebugCtxt{output: 0 as *mut FILE,
                     shift: [0; 101],
                     depth: 0,
                     doc: 0 as *mut xmlDoc,
                     node: 0 as *mut xmlNode,
                     dict: 0 as *mut xmlDict,
                     check: 0,
                     errors: 0,
                     nodict: 0,
                     options: 0,};
    if output.is_null() { return }
    xmlCtxtDumpInitCtxt(&mut ctxt);
    ctxt.output = output;
    xmlCtxtDumpEntities(&mut ctxt, doc);
    xmlCtxtDumpCleanCtxt(&mut ctxt);
}
/*
 * Summary: Tree debugging APIs
 * Description: Interfaces to a set of routines used for debugging the tree
 *              produced by the XML parser.
 *
 * Copy: See Copyright for the status of this software.
 *
 * Author: Daniel Veillard
 */
/*
 * The standard Dump routines.
 */
/* *
 * xmlDebugDumpAttrList:
 * @output:  the FILE * for the output
 * @attr:  the attribute list
 * @depth:  the indentation level.
 *
 * Dumps debug information for the attribute list
 */
#[no_mangle]
pub unsafe extern "C" fn xmlDebugDumpAttrList(mut output: *mut FILE,
                                              mut attr: xmlAttrPtr,
                                              mut depth: std::os::raw::c_int) {
    let mut ctxt: xmlDebugCtxt =
        xmlDebugCtxt{output: 0 as *mut FILE,
                     shift: [0; 101],
                     depth: 0,
                     doc: 0 as *mut xmlDoc,
                     node: 0 as *mut xmlNode,
                     dict: 0 as *mut xmlDict,
                     check: 0,
                     errors: 0,
                     nodict: 0,
                     options: 0,};
    if output.is_null() { return }
    xmlCtxtDumpInitCtxt(&mut ctxt);
    ctxt.output = output;
    ctxt.depth = depth;
    xmlCtxtDumpAttrList(&mut ctxt, attr);
    xmlCtxtDumpCleanCtxt(&mut ctxt);
}
/* *
 * xmlDebugDumpOneNode:
 * @output:  the FILE * for the output
 * @node:  the node
 * @depth:  the indentation level.
 *
 * Dumps debug information for the element node, it is not recursive
 */
#[no_mangle]
pub unsafe extern "C" fn xmlDebugDumpOneNode(mut output: *mut FILE,
                                             mut node: xmlNodePtr,
                                             mut depth: std::os::raw::c_int) {
    let mut ctxt: xmlDebugCtxt =
        xmlDebugCtxt{output: 0 as *mut FILE,
                     shift: [0; 101],
                     depth: 0,
                     doc: 0 as *mut xmlDoc,
                     node: 0 as *mut xmlNode,
                     dict: 0 as *mut xmlDict,
                     check: 0,
                     errors: 0,
                     nodict: 0,
                     options: 0,};
    if output.is_null() { return }
    xmlCtxtDumpInitCtxt(&mut ctxt);
    ctxt.output = output;
    ctxt.depth = depth;
    xmlCtxtDumpOneNode(&mut ctxt, node);
    xmlCtxtDumpCleanCtxt(&mut ctxt);
}
/* *
 * xmlDebugDumpNode:
 * @output:  the FILE * for the output
 * @node:  the node
 * @depth:  the indentation level.
 *
 * Dumps debug information for the element node, it is recursive
 */
#[no_mangle]
pub unsafe extern "C" fn xmlDebugDumpNode(mut output: *mut FILE,
                                          mut node: xmlNodePtr,
                                          mut depth: std::os::raw::c_int) {
    let mut ctxt: xmlDebugCtxt =
        xmlDebugCtxt{output: 0 as *mut FILE,
                     shift: [0; 101],
                     depth: 0,
                     doc: 0 as *mut xmlDoc,
                     node: 0 as *mut xmlNode,
                     dict: 0 as *mut xmlDict,
                     check: 0,
                     errors: 0,
                     nodict: 0,
                     options: 0,};
    if output.is_null() { output = stdout }
    xmlCtxtDumpInitCtxt(&mut ctxt);
    ctxt.output = output;
    ctxt.depth = depth;
    xmlCtxtDumpNode(&mut ctxt, node);
    xmlCtxtDumpCleanCtxt(&mut ctxt);
}
/* *
 * xmlDebugDumpNodeList:
 * @output:  the FILE * for the output
 * @node:  the node list
 * @depth:  the indentation level.
 *
 * Dumps debug information for the list of element node, it is recursive
 */
#[no_mangle]
pub unsafe extern "C" fn xmlDebugDumpNodeList(mut output: *mut FILE,
                                              mut node: xmlNodePtr,
                                              mut depth: std::os::raw::c_int) {
    let mut ctxt: xmlDebugCtxt =
        xmlDebugCtxt{output: 0 as *mut FILE,
                     shift: [0; 101],
                     depth: 0,
                     doc: 0 as *mut xmlDoc,
                     node: 0 as *mut xmlNode,
                     dict: 0 as *mut xmlDict,
                     check: 0,
                     errors: 0,
                     nodict: 0,
                     options: 0,};
    if output.is_null() { output = stdout }
    xmlCtxtDumpInitCtxt(&mut ctxt);
    ctxt.output = output;
    ctxt.depth = depth;
    xmlCtxtDumpNodeList(&mut ctxt, node);
    xmlCtxtDumpCleanCtxt(&mut ctxt);
}
/* *
 * xmlDebugDumpDocumentHead:
 * @output:  the FILE * for the output
 * @doc:  the document
 *
 * Dumps debug information cncerning the document, not recursive
 */
#[no_mangle]
pub unsafe extern "C" fn xmlDebugDumpDocumentHead(mut output: *mut FILE,
                                                  mut doc: xmlDocPtr) {
    let mut ctxt: xmlDebugCtxt =
        xmlDebugCtxt{output: 0 as *mut FILE,
                     shift: [0; 101],
                     depth: 0,
                     doc: 0 as *mut xmlDoc,
                     node: 0 as *mut xmlNode,
                     dict: 0 as *mut xmlDict,
                     check: 0,
                     errors: 0,
                     nodict: 0,
                     options: 0,};
    if output.is_null() { output = stdout }
    xmlCtxtDumpInitCtxt(&mut ctxt);
    ctxt.options |= 1 as std::os::raw::c_int;
    ctxt.output = output;
    xmlCtxtDumpDocumentHead(&mut ctxt, doc);
    xmlCtxtDumpCleanCtxt(&mut ctxt);
}
/* *
 * xmlDebugDumpDocument:
 * @output:  the FILE * for the output
 * @doc:  the document
 *
 * Dumps debug information for the document, it's recursive
 */
#[no_mangle]
pub unsafe extern "C" fn xmlDebugDumpDocument(mut output: *mut FILE,
                                              mut doc: xmlDocPtr) {
    let mut ctxt: xmlDebugCtxt =
        xmlDebugCtxt{output: 0 as *mut FILE,
                     shift: [0; 101],
                     depth: 0,
                     doc: 0 as *mut xmlDoc,
                     node: 0 as *mut xmlNode,
                     dict: 0 as *mut xmlDict,
                     check: 0,
                     errors: 0,
                     nodict: 0,
                     options: 0,};
    if output.is_null() { output = stdout }
    xmlCtxtDumpInitCtxt(&mut ctxt);
    ctxt.options |= 1 as std::os::raw::c_int;
    ctxt.output = output;
    xmlCtxtDumpDocument(&mut ctxt, doc);
    xmlCtxtDumpCleanCtxt(&mut ctxt);
}
/* *
 * xmlDebugDumpDTD:
 * @output:  the FILE * for the output
 * @dtd:  the DTD
 *
 * Dumps debug information for the DTD
 */
#[no_mangle]
pub unsafe extern "C" fn xmlDebugDumpDTD(mut output: *mut FILE,
                                         mut dtd: xmlDtdPtr) {
    let mut ctxt: xmlDebugCtxt =
        xmlDebugCtxt{output: 0 as *mut FILE,
                     shift: [0; 101],
                     depth: 0,
                     doc: 0 as *mut xmlDoc,
                     node: 0 as *mut xmlNode,
                     dict: 0 as *mut xmlDict,
                     check: 0,
                     errors: 0,
                     nodict: 0,
                     options: 0,};
    if output.is_null() { output = stdout }
    xmlCtxtDumpInitCtxt(&mut ctxt);
    ctxt.options |= 1 as std::os::raw::c_int;
    ctxt.output = output;
    xmlCtxtDumpDTD(&mut ctxt, dtd);
    xmlCtxtDumpCleanCtxt(&mut ctxt);
}
/* ***********************************************************************
 *									*
 *			Public entry points for checkings		*
 *									*
 ************************************************************************/
/* *
 * xmlDebugCheckDocument:
 * @output:  the FILE * for the output
 * @doc:  the document
 *
 * Check the document for potential content problems, and output
 * the errors to @output
 *
 * Returns the number of errors found
 */
#[no_mangle]
pub unsafe extern "C" fn xmlDebugCheckDocument(mut output: *mut FILE,
                                               mut doc: xmlDocPtr)
 -> std::os::raw::c_int {
    let mut ctxt: xmlDebugCtxt =
        xmlDebugCtxt{output: 0 as *mut FILE,
                     shift: [0; 101],
                     depth: 0,
                     doc: 0 as *mut xmlDoc,
                     node: 0 as *mut xmlNode,
                     dict: 0 as *mut xmlDict,
                     check: 0,
                     errors: 0,
                     nodict: 0,
                     options: 0,};
    if output.is_null() { output = stdout }
    xmlCtxtDumpInitCtxt(&mut ctxt);
    ctxt.output = output;
    ctxt.check = 1 as std::os::raw::c_int;
    xmlCtxtDumpDocument(&mut ctxt, doc);
    xmlCtxtDumpCleanCtxt(&mut ctxt);
    return ctxt.errors;
}
/* ***********************************************************************
 *									*
 *			Helpers for Shell				*
 *									*
 ************************************************************************/
/* *
 * xmlLsCountNode:
 * @node:  the node to count
 *
 * Count the children of @node.
 *
 * Returns the number of children of @node.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlLsCountNode(mut node: xmlNodePtr) -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut list: xmlNodePtr = 0 as xmlNodePtr;
    if node.is_null() { return 0 as std::os::raw::c_int }
    match (*node).type_0 as std::os::raw::c_uint {
        1 => { list = (*node).children }
        9 | 13 | 21 => { list = (*(node as xmlDocPtr)).children }
        2 => { list = (*(node as xmlAttrPtr)).children }
        3 | 4 | 7 | 8 => {
            if !(*node).content.is_null() { ret = xmlStrlen((*node).content) }
        }
        5 | 10 | 6 | 11 | 12 | 14 | 15 | 16 | 17 | 18 | 19 | 20 => {
            ret = 1 as std::os::raw::c_int
        }
        _ => { }
    }
    while !list.is_null() { list = (*list).next; ret += 1 }
    return ret;
}
/* *
 * xmlLsOneNode:
 * @output:  the FILE * for the output
 * @node:  the node to dump
 *
 * Dump to @output the type and name of @node.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlLsOneNode(mut output: *mut FILE,
                                      mut node: xmlNodePtr) {
    if output.is_null() { return }
    if node.is_null() {
        fprintf(output, b"NULL\n\x00" as *const u8 as *const std::os::raw::c_char);
        return
    }
    match (*node).type_0 as std::os::raw::c_uint {
        1 => {
            fprintf(output, b"-\x00" as *const u8 as *const std::os::raw::c_char);
        }
        2 => {
            fprintf(output, b"a\x00" as *const u8 as *const std::os::raw::c_char);
        }
        3 => {
            fprintf(output, b"t\x00" as *const u8 as *const std::os::raw::c_char);
        }
        4 => {
            fprintf(output, b"C\x00" as *const u8 as *const std::os::raw::c_char);
        }
        5 => {
            fprintf(output, b"e\x00" as *const u8 as *const std::os::raw::c_char);
        }
        6 => {
            fprintf(output, b"E\x00" as *const u8 as *const std::os::raw::c_char);
        }
        7 => {
            fprintf(output, b"p\x00" as *const u8 as *const std::os::raw::c_char);
        }
        8 => {
            fprintf(output, b"c\x00" as *const u8 as *const std::os::raw::c_char);
        }
        9 => {
            fprintf(output, b"d\x00" as *const u8 as *const std::os::raw::c_char);
        }
        13 => {
            fprintf(output, b"h\x00" as *const u8 as *const std::os::raw::c_char);
        }
        10 => {
            fprintf(output, b"T\x00" as *const u8 as *const std::os::raw::c_char);
        }
        11 => {
            fprintf(output, b"F\x00" as *const u8 as *const std::os::raw::c_char);
        }
        12 => {
            fprintf(output, b"N\x00" as *const u8 as *const std::os::raw::c_char);
        }
        18 => {
            fprintf(output, b"n\x00" as *const u8 as *const std::os::raw::c_char);
        }
        _ => {
            fprintf(output, b"?\x00" as *const u8 as *const std::os::raw::c_char);
        }
    }
    if (*node).type_0 as std::os::raw::c_uint !=
           XML_NAMESPACE_DECL as std::os::raw::c_int as std::os::raw::c_uint {
        if !(*node).properties.is_null() {
            fprintf(output, b"a\x00" as *const u8 as *const std::os::raw::c_char);
        } else {
            fprintf(output, b"-\x00" as *const u8 as *const std::os::raw::c_char);
        }
        if !(*node).nsDef.is_null() {
            fprintf(output, b"n\x00" as *const u8 as *const std::os::raw::c_char);
        } else {
            fprintf(output, b"-\x00" as *const u8 as *const std::os::raw::c_char);
        }
    }
    fprintf(output, b" %8d \x00" as *const u8 as *const std::os::raw::c_char,
            xmlLsCountNode(node));
    match (*node).type_0 as std::os::raw::c_uint {
        1 => {
            if !(*node).name.is_null() {
                if !(*node).ns.is_null() && !(*(*node).ns).prefix.is_null() {
                    fprintf(output,
                            b"%s:\x00" as *const u8 as *const std::os::raw::c_char,
                            (*(*node).ns).prefix);
                }
                fprintf(output, b"%s\x00" as *const u8 as *const std::os::raw::c_char,
                        (*node).name as *const std::os::raw::c_char);
            }
        }
        2 => {
            if !(*node).name.is_null() {
                fprintf(output, b"%s\x00" as *const u8 as *const std::os::raw::c_char,
                        (*node).name as *const std::os::raw::c_char);
            }
        }
        3 => {
            if !(*node).content.is_null() {
                xmlDebugDumpString(output, (*node).content);
            }
        }
        5 => {
            if !(*node).name.is_null() {
                fprintf(output, b"%s\x00" as *const u8 as *const std::os::raw::c_char,
                        (*node).name as *const std::os::raw::c_char);
            }
        }
        6 => {
            if !(*node).name.is_null() {
                fprintf(output, b"%s\x00" as *const u8 as *const std::os::raw::c_char,
                        (*node).name as *const std::os::raw::c_char);
            }
        }
        7 => {
            if !(*node).name.is_null() {
                fprintf(output, b"%s\x00" as *const u8 as *const std::os::raw::c_char,
                        (*node).name as *const std::os::raw::c_char);
            }
        }
        4 | 8 | 9 | 13 | 10 | 11 | 12 => { }
        18 => {
            let mut ns: xmlNsPtr = node as xmlNsPtr;
            if (*ns).prefix.is_null() {
                fprintf(output,
                        b"default -> %s\x00" as *const u8 as
                            *const std::os::raw::c_char,
                        (*ns).href as *mut std::os::raw::c_char);
            } else {
                fprintf(output,
                        b"%s -> %s\x00" as *const u8 as *const std::os::raw::c_char,
                        (*ns).prefix as *mut std::os::raw::c_char,
                        (*ns).href as *mut std::os::raw::c_char);
            }
        }
        _ => {
            if !(*node).name.is_null() {
                fprintf(output, b"%s\x00" as *const u8 as *const std::os::raw::c_char,
                        (*node).name as *const std::os::raw::c_char);
            }
        }
    }
    fprintf(output, b"\n\x00" as *const u8 as *const std::os::raw::c_char);
}
/* *
 * xmlBoolToText:
 * @boolval: a bool to turn into text
 *
 * Convenient way to turn bool into text
 *
 * Returns a pointer to either "True" or "False"
 */
#[no_mangle]
pub unsafe extern "C" fn xmlBoolToText(mut boolval: std::os::raw::c_int)
 -> *const std::os::raw::c_char {
    if boolval != 0 {
        return b"True\x00" as *const u8 as *const std::os::raw::c_char
    } else { return b"False\x00" as *const u8 as *const std::os::raw::c_char };
}
/* ***************************************************************
 *								*
 *		The XML shell related functions			*
 *								*
 ****************************************************************/
/*
 * TODO: Improvement/cleanups for the XML shell
 *     - allow to shell out an editor on a subpart
 *     - cleanup function registrations (with help) and calling
 *     - provide registration routines
 */
/* *
 * xmlShellPrintXPathError:
 * @errorType: valid xpath error id
 * @arg: the argument that cause xpath to fail
 *
 * Print the xpath error to libxml default error channel
 */
#[no_mangle]
pub unsafe extern "C" fn xmlShellPrintXPathError(mut errorType: std::os::raw::c_int,
                                                 mut arg:
                                                     *const std::os::raw::c_char) {
    let mut default_arg: *const std::os::raw::c_char =
        b"Result\x00" as *const u8 as *const std::os::raw::c_char;
    if arg.is_null() { arg = default_arg }
    match errorType {
        0 => {
            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                       b"%s: no such node\n\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const std::os::raw::c_char,
                                                                       arg);
        }
        2 => {
            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                       b"%s is a Boolean\n\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const std::os::raw::c_char,
                                                                       arg);
        }
        3 => {
            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                       b"%s is a number\n\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const std::os::raw::c_char,
                                                                       arg);
        }
        4 => {
            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                       b"%s is a string\n\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const std::os::raw::c_char,
                                                                       arg);
        }
        5 => {
            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                       b"%s is a point\n\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const std::os::raw::c_char,
                                                                       arg);
        }
        6 => {
            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                       b"%s is a range\n\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const std::os::raw::c_char,
                                                                       arg);
        }
        7 => {
            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                       b"%s is a range\n\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const std::os::raw::c_char,
                                                                       arg);
        }
        8 => {
            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                       b"%s is user-defined\n\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const std::os::raw::c_char,
                                                                       arg);
        }
        9 => {
            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                       b"%s is an XSLT value tree\n\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const std::os::raw::c_char,
                                                                       arg);
        }
        _ => { }
    };
}
/* *
 * xmlShellPrintNodeCtxt:
 * @ctxt : a non-null shell context
 * @node : a non-null node to print to the output FILE
 *
 * Print node to the output FILE
 */
unsafe extern "C" fn xmlShellPrintNodeCtxt(mut ctxt: xmlShellCtxtPtr,
                                           mut node: xmlNodePtr) {
    let mut fp: *mut FILE = 0 as *mut FILE;
    if node.is_null() { return }
    if ctxt.is_null() { fp = stdout } else { fp = (*ctxt).output }
    if (*node).type_0 as std::os::raw::c_uint ==
           XML_DOCUMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        xmlDocDump(fp, node as xmlDocPtr);
    } else if (*node).type_0 as std::os::raw::c_uint ==
                  XML_ATTRIBUTE_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        xmlDebugDumpAttrList(fp, node as xmlAttrPtr, 0 as std::os::raw::c_int);
    } else { xmlElemDump(fp, (*node).doc, node); }
    fprintf(fp, b"\n\x00" as *const u8 as *const std::os::raw::c_char);
}
/* *
 * xmlShellPrintNode:
 * @node : a non-null node to print to the output FILE
 *
 * Print node to the output FILE
 */
#[no_mangle]
pub unsafe extern "C" fn xmlShellPrintNode(mut node: xmlNodePtr) {
    xmlShellPrintNodeCtxt(0 as xmlShellCtxtPtr, node);
}
/* LIBXML_OUTPUT_ENABLED */
/* *
 * xmlShellPrintXPathResultCtxt:
 * @ctxt: a valid shell context
 * @list: a valid result generated by an xpath evaluation
 *
 * Prints result to the output FILE
 */
unsafe extern "C" fn xmlShellPrintXPathResultCtxt(mut ctxt: xmlShellCtxtPtr,
                                                  mut list:
                                                      xmlXPathObjectPtr) {
    if ctxt.is_null() { return }
    if !list.is_null() {
        match (*list).type_0 as std::os::raw::c_uint {
            1 => {
                let mut indx: std::os::raw::c_int = 0;
                if !(*list).nodesetval.is_null() {
                    indx = 0 as std::os::raw::c_int;
                    while indx < (*(*list).nodesetval).nodeNr {
                        xmlShellPrintNodeCtxt(ctxt,
                                              *(*(*list).nodesetval).nodeTab.offset(indx
                                                                                        as
                                                                                        isize));
                        indx += 1
                    }
                } else {
                    (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                               b"Empty node set\n\x00"
                                                                                   as
                                                                                   *const u8
                                                                                   as
                                                                                   *const std::os::raw::c_char);
                }
                /* LIBXML_OUTPUT_ENABLED */
            }
            2 => {
                (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                           b"Is a Boolean:%s\n\x00"
                                                                               as
                                                                               *const u8
                                                                               as
                                                                               *const std::os::raw::c_char,
                                                                           xmlBoolToText((*list).boolval));
            }
            3 => {
                (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                           b"Is a number:%0g\n\x00"
                                                                               as
                                                                               *const u8
                                                                               as
                                                                               *const std::os::raw::c_char,
                                                                           (*list).floatval);
            }
            4 => {
                (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                           b"Is a string:%s\n\x00"
                                                                               as
                                                                               *const u8
                                                                               as
                                                                               *const std::os::raw::c_char,
                                                                           (*list).stringval);
            }
            _ => {
                xmlShellPrintXPathError((*list).type_0 as std::os::raw::c_int,
                                        0 as *const std::os::raw::c_char);
            }
        }
    };
}
/* *
 * xmlShellPrintXPathResult:
 * @list: a valid result generated by an xpath evaluation
 *
 * Prints result to the output FILE
 */
#[no_mangle]
pub unsafe extern "C" fn xmlShellPrintXPathResult(mut list:
                                                      xmlXPathObjectPtr) {
    xmlShellPrintXPathResultCtxt(0 as xmlShellCtxtPtr, list);
}
/* *
 * xmlShellList:
 * @ctxt:  the shell context
 * @arg:  unused
 * @node:  a node
 * @node2:  unused
 *
 * Implements the XML shell function "ls"
 * Does an Unix like listing of the given node (like a directory)
 *
 * Returns 0
 */
#[no_mangle]
pub unsafe extern "C" fn xmlShellList(mut ctxt: xmlShellCtxtPtr,
                                      mut arg: *mut std::os::raw::c_char,
                                      mut node: xmlNodePtr,
                                      mut node2: xmlNodePtr) -> std::os::raw::c_int {
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    if ctxt.is_null() { return 0 as std::os::raw::c_int }
    if node.is_null() {
        fprintf((*ctxt).output,
                b"NULL\n\x00" as *const u8 as *const std::os::raw::c_char);
        return 0 as std::os::raw::c_int
    }
    if (*node).type_0 as std::os::raw::c_uint ==
           XML_DOCUMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint ||
           (*node).type_0 as std::os::raw::c_uint ==
               XML_HTML_DOCUMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        cur = (*(node as xmlDocPtr)).children
    } else if (*node).type_0 as std::os::raw::c_uint ==
                  XML_NAMESPACE_DECL as std::os::raw::c_int as std::os::raw::c_uint {
        xmlLsOneNode((*ctxt).output, node);
        return 0 as std::os::raw::c_int
    } else {
        if !(*node).children.is_null() {
            cur = (*node).children
        } else { xmlLsOneNode((*ctxt).output, node); return 0 as std::os::raw::c_int }
    }
    while !cur.is_null() {
        xmlLsOneNode((*ctxt).output, cur);
        cur = (*cur).next
    }
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlShellBase:
 * @ctxt:  the shell context
 * @arg:  unused
 * @node:  a node
 * @node2:  unused
 *
 * Implements the XML shell function "base"
 * dumps the current XML base of the node
 *
 * Returns 0
 */
#[no_mangle]
pub unsafe extern "C" fn xmlShellBase(mut ctxt: xmlShellCtxtPtr,
                                      mut arg: *mut std::os::raw::c_char,
                                      mut node: xmlNodePtr,
                                      mut node2: xmlNodePtr) -> std::os::raw::c_int {
    let mut base: *mut xmlChar = 0 as *mut xmlChar;
    if ctxt.is_null() { return 0 as std::os::raw::c_int }
    if node.is_null() {
        fprintf((*ctxt).output,
                b"NULL\n\x00" as *const u8 as *const std::os::raw::c_char);
        return 0 as std::os::raw::c_int
    }
    base = xmlNodeGetBase((*node).doc, node as *const xmlNode);
    if base.is_null() {
        fprintf((*ctxt).output,
                b" No base found !!!\n\x00" as *const u8 as
                    *const std::os::raw::c_char);
    } else {
        fprintf((*ctxt).output,
                b"%s\n\x00" as *const u8 as *const std::os::raw::c_char, base);
        xmlFree.expect("non-null function pointer")(base as
                                                        *mut std::os::raw::c_void);
    }
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlShellSetBase:
 * @ctxt:  the shell context
 * @arg:  the new base
 * @node:  a node
 * @node2:  unused
 *
 * Implements the XML shell function "setbase"
 * change the current XML base of the node
 *
 * Returns 0
 */
unsafe extern "C" fn xmlShellSetBase(mut ctxt: xmlShellCtxtPtr,
                                     mut arg: *mut std::os::raw::c_char,
                                     mut node: xmlNodePtr,
                                     mut node2: xmlNodePtr) -> std::os::raw::c_int {
    xmlNodeSetBase(node, arg as *mut xmlChar);
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlShellRegisterNamespace:
 * @ctxt:  the shell context
 * @arg:  a string in prefix=nsuri format
 * @node:  unused
 * @node2:  unused
 *
 * Implements the XML shell function "setns"
 * register/unregister a prefix=namespace pair
 * on the XPath context
 *
 * Returns 0 on success and a negative value otherwise.
 */
unsafe extern "C" fn xmlShellRegisterNamespace(mut ctxt: xmlShellCtxtPtr,
                                               mut arg: *mut std::os::raw::c_char,
                                               mut node: xmlNodePtr,
                                               mut node2: xmlNodePtr)
 -> std::os::raw::c_int {
    let mut nsListDup: *mut xmlChar = 0 as *mut xmlChar;
    let mut prefix: *mut xmlChar = 0 as *mut xmlChar;
    let mut href: *mut xmlChar = 0 as *mut xmlChar;
    let mut next: *mut xmlChar = 0 as *mut xmlChar;
    nsListDup = xmlStrdup(arg as *mut xmlChar);
    next = nsListDup;
    while !next.is_null() {
        /* skip spaces */
	/*while((*next) == ' ') next++;*/
        if *next as std::os::raw::c_int == '\u{0}' as i32 { break ; }
        /* find prefix */
        prefix = next;
        next = xmlStrchr(next, '=' as i32 as xmlChar) as *mut xmlChar;
        if next.is_null() {
            fprintf((*ctxt).output,
                    b"setns: prefix=[nsuri] required\n\x00" as *const u8 as
                        *const std::os::raw::c_char);
            xmlFree.expect("non-null function pointer")(nsListDup as
                                                            *mut std::os::raw::c_void);
            return -(1 as std::os::raw::c_int)
        }
        let fresh0 = next;
        next = next.offset(1);
        *fresh0 = '\u{0}' as i32 as xmlChar;
        /* find href */
        href = next;
        next = xmlStrchr(next, ' ' as i32 as xmlChar) as *mut xmlChar;
        if !next.is_null() {
            let fresh1 = next;
            next = next.offset(1);
            *fresh1 = '\u{0}' as i32 as xmlChar
        }
        /* do register namespace */
        if xmlXPathRegisterNs((*ctxt).pctxt, prefix, href) != 0 as std::os::raw::c_int
           {
            fprintf((*ctxt).output,
                    b"Error: unable to register NS with prefix=\"%s\" and href=\"%s\"\n\x00"
                        as *const u8 as *const std::os::raw::c_char, prefix, href);
            xmlFree.expect("non-null function pointer")(nsListDup as
                                                            *mut std::os::raw::c_void);
            return -(1 as std::os::raw::c_int)
        }
    }
    xmlFree.expect("non-null function pointer")(nsListDup as
                                                    *mut std::os::raw::c_void);
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlShellRegisterRootNamespaces:
 * @ctxt:  the shell context
 * @arg:  unused
 * @node:  the root element
 * @node2:  unused
 *
 * Implements the XML shell function "setrootns"
 * which registers all namespaces declarations found on the root element.
 *
 * Returns 0 on success and a negative value otherwise.
 */
unsafe extern "C" fn xmlShellRegisterRootNamespaces(mut ctxt: xmlShellCtxtPtr,
                                                    mut arg:
                                                        *mut std::os::raw::c_char,
                                                    mut root: xmlNodePtr,
                                                    mut node2: xmlNodePtr)
 -> std::os::raw::c_int {
    let mut ns: xmlNsPtr = 0 as *mut xmlNs;
    if root.is_null() ||
           (*root).type_0 as std::os::raw::c_uint !=
               XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint ||
           (*root).nsDef.is_null() || ctxt.is_null() ||
           (*ctxt).pctxt.is_null() {
        return -(1 as std::os::raw::c_int)
    }
    ns = (*root).nsDef;
    while !ns.is_null() {
        if (*ns).prefix.is_null() {
            xmlXPathRegisterNs((*ctxt).pctxt,
                               b"defaultns\x00" as *const u8 as
                                   *const std::os::raw::c_char as *mut xmlChar,
                               (*ns).href);
        } else {
            xmlXPathRegisterNs((*ctxt).pctxt, (*ns).prefix, (*ns).href);
        }
        ns = (*ns).next
    }
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlShellGrep:
 * @ctxt:  the shell context
 * @arg:  the string or regular expression to find
 * @node:  a node
 * @node2:  unused
 *
 * Implements the XML shell function "grep"
 * dumps informations about the node (namespace, attributes, content).
 *
 * Returns 0
 */
unsafe extern "C" fn xmlShellGrep(mut ctxt: xmlShellCtxtPtr,
                                  mut arg: *mut std::os::raw::c_char,
                                  mut node: xmlNodePtr, mut node2: xmlNodePtr)
 -> std::os::raw::c_int {
    if ctxt.is_null() { return 0 as std::os::raw::c_int }
    if node.is_null() { return 0 as std::os::raw::c_int }
    if arg.is_null() { return 0 as std::os::raw::c_int }
    (!xmlStrchr(arg as *mut xmlChar, '?' as i32 as xmlChar).is_null() ||
         !xmlStrchr(arg as *mut xmlChar, '*' as i32 as xmlChar).is_null() ||
         !xmlStrchr(arg as *mut xmlChar, '.' as i32 as xmlChar).is_null()) ||
        !xmlStrchr(arg as *mut xmlChar, '[' as i32 as xmlChar).is_null();
    while !node.is_null() {
        if (*node).type_0 as std::os::raw::c_uint ==
               XML_COMMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
            if !xmlStrstr((*node).content, arg as *mut xmlChar).is_null() {
                fprintf((*ctxt).output,
                        b"%s : \x00" as *const u8 as *const std::os::raw::c_char,
                        xmlGetNodePath(node as *const xmlNode));
                xmlShellList(ctxt, 0 as *mut std::os::raw::c_char, node,
                             0 as xmlNodePtr);
            }
        } else if (*node).type_0 as std::os::raw::c_uint ==
                      XML_TEXT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
            if !xmlStrstr((*node).content, arg as *mut xmlChar).is_null() {
                fprintf((*ctxt).output,
                        b"%s : \x00" as *const u8 as *const std::os::raw::c_char,
                        xmlGetNodePath((*node).parent));
                xmlShellList(ctxt, 0 as *mut std::os::raw::c_char, (*node).parent,
                             0 as xmlNodePtr);
            }
        }
        /*
         * Browse the full subtree, deep first
         */
        if (*node).type_0 as std::os::raw::c_uint ==
               XML_DOCUMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint ||
               (*node).type_0 as std::os::raw::c_uint ==
                   XML_HTML_DOCUMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
            node = (*(node as xmlDocPtr)).children
        } else if !(*node).children.is_null() &&
                      (*node).type_0 as std::os::raw::c_uint !=
                          XML_ENTITY_REF_NODE as std::os::raw::c_int as std::os::raw::c_uint {
            /* deep first */
            node = (*node).children
        } else if !(*node).next.is_null() {
            /* then siblings */
            node = (*node).next
        } else {
            /* go up to parents->next if needed */
            while !node.is_null() {
                if !(*node).parent.is_null() { node = (*node).parent }
                if !(*node).next.is_null() {
                    node = (*node).next;
                    break ;
                } else {
                    if !(*node).parent.is_null() { continue ; }
                    node = 0 as xmlNodePtr;
                    break ;
                }
            }
        }
    }
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlShellDir:
 * @ctxt:  the shell context
 * @arg:  unused
 * @node:  a node
 * @node2:  unused
 *
 * Implements the XML shell function "dir"
 * dumps informations about the node (namespace, attributes, content).
 *
 * Returns 0
 */
#[no_mangle]
pub unsafe extern "C" fn xmlShellDir(mut ctxt: xmlShellCtxtPtr,
                                     mut arg: *mut std::os::raw::c_char,
                                     mut node: xmlNodePtr,
                                     mut node2: xmlNodePtr) -> std::os::raw::c_int {
    if ctxt.is_null() { return 0 as std::os::raw::c_int }
    if node.is_null() {
        fprintf((*ctxt).output,
                b"NULL\n\x00" as *const u8 as *const std::os::raw::c_char);
        return 0 as std::os::raw::c_int
    }
    if (*node).type_0 as std::os::raw::c_uint ==
           XML_DOCUMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint ||
           (*node).type_0 as std::os::raw::c_uint ==
               XML_HTML_DOCUMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        xmlDebugDumpDocumentHead((*ctxt).output, node as xmlDocPtr);
    } else if (*node).type_0 as std::os::raw::c_uint ==
                  XML_ATTRIBUTE_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        xmlDebugDumpAttr((*ctxt).output, node as xmlAttrPtr,
                         0 as std::os::raw::c_int);
    } else { xmlDebugDumpOneNode((*ctxt).output, node, 0 as std::os::raw::c_int); }
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlShellSetContent:
 * @ctxt:  the shell context
 * @value:  the content as a string
 * @node:  a node
 * @node2:  unused
 *
 * Implements the XML shell function "dir"
 * dumps informations about the node (namespace, attributes, content).
 *
 * Returns 0
 */
unsafe extern "C" fn xmlShellSetContent(mut ctxt: xmlShellCtxtPtr,
                                        mut value: *mut std::os::raw::c_char,
                                        mut node: xmlNodePtr,
                                        mut node2: xmlNodePtr)
 -> std::os::raw::c_int {
    let mut results: xmlNodePtr = 0 as *mut xmlNode;
    let mut ret: xmlParserErrors = XML_ERR_OK;
    if ctxt.is_null() { return 0 as std::os::raw::c_int }
    if node.is_null() {
        fprintf((*ctxt).output,
                b"NULL\n\x00" as *const u8 as *const std::os::raw::c_char);
        return 0 as std::os::raw::c_int
    }
    if value.is_null() {
        fprintf((*ctxt).output,
                b"NULL\n\x00" as *const u8 as *const std::os::raw::c_char);
        return 0 as std::os::raw::c_int
    }
    ret =
        xmlParseInNodeContext(node, value, strlen(value) as std::os::raw::c_int,
                              0 as std::os::raw::c_int, &mut results);
    if ret as std::os::raw::c_uint == XML_ERR_OK as std::os::raw::c_int as std::os::raw::c_uint {
        if !(*node).children.is_null() {
            xmlFreeNodeList((*node).children);
            (*node).children = 0 as *mut _xmlNode;
            (*node).last = 0 as *mut _xmlNode
        }
        xmlAddChildList(node, results);
    } else {
        fprintf((*ctxt).output,
                b"failed to parse content\n\x00" as *const u8 as
                    *const std::os::raw::c_char);
    }
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlShellRNGValidate:
 * @ctxt:  the shell context
 * @schemas:  the path to the Relax-NG schemas
 * @node:  a node
 * @node2:  unused
 *
 * Implements the XML shell function "relaxng"
 * validating the instance against a Relax-NG schemas
 *
 * Returns 0
 */
unsafe extern "C" fn xmlShellRNGValidate(mut sctxt: xmlShellCtxtPtr,
                                         mut schemas: *mut std::os::raw::c_char,
                                         mut node: xmlNodePtr,
                                         mut node2: xmlNodePtr)
 -> std::os::raw::c_int {
    let mut relaxngschemas: xmlRelaxNGPtr = 0 as *mut xmlRelaxNG;
    let mut ctxt: xmlRelaxNGParserCtxtPtr = 0 as *mut xmlRelaxNGParserCtxt;
    let mut vctxt: xmlRelaxNGValidCtxtPtr = 0 as *mut xmlRelaxNGValidCtxt;
    let mut ret: std::os::raw::c_int = 0;
    ctxt = xmlRelaxNGNewParserCtxt(schemas);
    xmlRelaxNGSetParserErrors(ctxt,
                              ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                      *mut FILE,
                                                                                  _:
                                                                                      *const std::os::raw::c_char,
                                                                                  _:
                                                                                      ...)
                                                                 ->
                                                                     std::os::raw::c_int>,
                                                      xmlRelaxNGValidityErrorFunc>(Some(fprintf
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
                                                      xmlRelaxNGValidityWarningFunc>(Some(fprintf
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
    relaxngschemas = xmlRelaxNGParse(ctxt);
    xmlRelaxNGFreeParserCtxt(ctxt);
    if relaxngschemas.is_null() {
        (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                   b"Relax-NG schema %s failed to compile\n\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const std::os::raw::c_char,
                                                                   schemas);
        return -(1 as std::os::raw::c_int)
    }
    vctxt = xmlRelaxNGNewValidCtxt(relaxngschemas);
    xmlRelaxNGSetValidErrors(vctxt,
                             ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                     *mut FILE,
                                                                                 _:
                                                                                     *const std::os::raw::c_char,
                                                                                 _:
                                                                                     ...)
                                                                ->
                                                                    std::os::raw::c_int>,
                                                     xmlRelaxNGValidityErrorFunc>(Some(fprintf
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
                                                     xmlRelaxNGValidityWarningFunc>(Some(fprintf
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
    ret = xmlRelaxNGValidateDoc(vctxt, (*sctxt).doc);
    if ret == 0 as std::os::raw::c_int {
        fprintf(stderr,
                b"%s validates\n\x00" as *const u8 as *const std::os::raw::c_char,
                (*sctxt).filename);
    } else if ret > 0 as std::os::raw::c_int {
        fprintf(stderr,
                b"%s fails to validate\n\x00" as *const u8 as
                    *const std::os::raw::c_char, (*sctxt).filename);
    } else {
        fprintf(stderr,
                b"%s validation generated an internal error\n\x00" as
                    *const u8 as *const std::os::raw::c_char, (*sctxt).filename);
    }
    xmlRelaxNGFreeValidCtxt(vctxt);
    if !relaxngschemas.is_null() { xmlRelaxNGFree(relaxngschemas); }
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlShellCat:
 * @ctxt:  the shell context
 * @arg:  unused
 * @node:  a node
 * @node2:  unused
 *
 * Implements the XML shell function "cat"
 * dumps the serialization node content (XML or HTML).
 *
 * Returns 0
 */
#[no_mangle]
pub unsafe extern "C" fn xmlShellCat(mut ctxt: xmlShellCtxtPtr,
                                     mut arg: *mut std::os::raw::c_char,
                                     mut node: xmlNodePtr,
                                     mut node2: xmlNodePtr) -> std::os::raw::c_int {
    if ctxt.is_null() { return 0 as std::os::raw::c_int }
    if node.is_null() {
        fprintf((*ctxt).output,
                b"NULL\n\x00" as *const u8 as *const std::os::raw::c_char);
        return 0 as std::os::raw::c_int
    }
    if (*(*ctxt).doc).type_0 as std::os::raw::c_uint ==
           XML_HTML_DOCUMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        if (*node).type_0 as std::os::raw::c_uint ==
               XML_HTML_DOCUMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
            htmlDocDump((*ctxt).output, node as htmlDocPtr);
        } else { htmlNodeDumpFile((*ctxt).output, (*ctxt).doc, node); }
        /* LIBXML_HTML_ENABLED */
    } else if (*node).type_0 as std::os::raw::c_uint ==
                  XML_DOCUMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        xmlDocDump((*ctxt).output, node as xmlDocPtr);
    } else { xmlElemDump((*ctxt).output, (*ctxt).doc, node); }
    fprintf((*ctxt).output, b"\n\x00" as *const u8 as *const std::os::raw::c_char);
    return 0 as std::os::raw::c_int;
}
/* LIBXML_OUTPUT_ENABLED */
/* *
 * xmlShellLoad:
 * @ctxt:  the shell context
 * @filename:  the file name
 * @node:  unused
 * @node2:  unused
 *
 * Implements the XML shell function "load"
 * loads a new document specified by the filename
 *
 * Returns 0 or -1 if loading failed
 */
#[no_mangle]
pub unsafe extern "C" fn xmlShellLoad(mut ctxt: xmlShellCtxtPtr,
                                      mut filename: *mut std::os::raw::c_char,
                                      mut node: xmlNodePtr,
                                      mut node2: xmlNodePtr) -> std::os::raw::c_int {
    let mut doc: xmlDocPtr = 0 as *mut xmlDoc;
    let mut html: std::os::raw::c_int = 0 as std::os::raw::c_int;
    if ctxt.is_null() || filename.is_null() { return -(1 as std::os::raw::c_int) }
    if !(*ctxt).doc.is_null() {
        html =
            ((*(*ctxt).doc).type_0 as std::os::raw::c_uint ==
                 XML_HTML_DOCUMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint) as
                std::os::raw::c_int
    }
    if html != 0 {
        doc = htmlParseFile(filename, 0 as *const std::os::raw::c_char)
        /* LIBXML_HTML_ENABLED */
    } else {
        doc =
            xmlReadFile(filename, 0 as *const std::os::raw::c_char, 0 as std::os::raw::c_int)
    }
    if !doc.is_null() {
        if (*ctxt).loaded == 1 as std::os::raw::c_int { xmlFreeDoc((*ctxt).doc); }
        (*ctxt).loaded = 1 as std::os::raw::c_int;
        xmlXPathFreeContext((*ctxt).pctxt);
        /* LIBXML_XPATH_ENABLED */
        xmlFree.expect("non-null function pointer")((*ctxt).filename as
                                                        *mut std::os::raw::c_void);
        (*ctxt).doc = doc;
        (*ctxt).node = doc as xmlNodePtr;
        (*ctxt).pctxt = xmlXPathNewContext(doc);
        /* LIBXML_XPATH_ENABLED */
        (*ctxt).filename =
            xmlCanonicPath(filename as *mut xmlChar) as *mut std::os::raw::c_char
    } else { return -(1 as std::os::raw::c_int) }
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlShellWrite:
 * @ctxt:  the shell context
 * @filename:  the file name
 * @node:  a node in the tree
 * @node2:  unused
 *
 * Implements the XML shell function "write"
 * Write the current node to the filename, it saves the serialization
 * of the subtree under the @node specified
 *
 * Returns 0 or -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlShellWrite(mut ctxt: xmlShellCtxtPtr,
                                       mut filename: *mut std::os::raw::c_char,
                                       mut node: xmlNodePtr,
                                       mut node2: xmlNodePtr) -> std::os::raw::c_int {
    if node.is_null() { return -(1 as std::os::raw::c_int) }
    if filename.is_null() ||
           *filename.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
               0 as std::os::raw::c_int {
        return -(1 as std::os::raw::c_int)
    }
    match (*node).type_0 as std::os::raw::c_uint {
        9 => {
            if xmlSaveFile(filename, (*ctxt).doc) < -(1 as std::os::raw::c_int) {
                (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                           b"Failed to write to %s\n\x00"
                                                                               as
                                                                               *const u8
                                                                               as
                                                                               *const std::os::raw::c_char,
                                                                           filename);
                return -(1 as std::os::raw::c_int)
            }
        }
        13 => {
            if htmlSaveFile(filename, (*ctxt).doc) < 0 as std::os::raw::c_int {
                (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                           b"Failed to write to %s\n\x00"
                                                                               as
                                                                               *const u8
                                                                               as
                                                                               *const std::os::raw::c_char,
                                                                           filename);
                return -(1 as std::os::raw::c_int)
            }
        }
        _ => {
            let mut f: *mut FILE = 0 as *mut FILE;
            f = fopen(filename, b"w\x00" as *const u8 as *const std::os::raw::c_char);
            if f.is_null() {
                (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                           b"Failed to write to %s\n\x00"
                                                                               as
                                                                               *const u8
                                                                               as
                                                                               *const std::os::raw::c_char,
                                                                           filename);
                return -(1 as std::os::raw::c_int)
            }
            xmlElemDump(f, (*ctxt).doc, node);
            fclose(f);
        }
    }
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlShellSave:
 * @ctxt:  the shell context
 * @filename:  the file name (optional)
 * @node:  unused
 * @node2:  unused
 *
 * Implements the XML shell function "save"
 * Write the current document to the filename, or it's original name
 *
 * Returns 0 or -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlShellSave(mut ctxt: xmlShellCtxtPtr,
                                      mut filename: *mut std::os::raw::c_char,
                                      mut node: xmlNodePtr,
                                      mut node2: xmlNodePtr) -> std::os::raw::c_int {
    if ctxt.is_null() || (*ctxt).doc.is_null() { return -(1 as std::os::raw::c_int) }
    if filename.is_null() ||
           *filename.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
               0 as std::os::raw::c_int {
        filename = (*ctxt).filename
    }
    if filename.is_null() { return -(1 as std::os::raw::c_int) }
    match (*(*ctxt).doc).type_0 as std::os::raw::c_uint {
        9 => {
            if xmlSaveFile(filename, (*ctxt).doc) < 0 as std::os::raw::c_int {
                (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                           b"Failed to save to %s\n\x00"
                                                                               as
                                                                               *const u8
                                                                               as
                                                                               *const std::os::raw::c_char,
                                                                           filename);
            }
        }
        13 => {
            if htmlSaveFile(filename, (*ctxt).doc) < 0 as std::os::raw::c_int {
                (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                           b"Failed to save to %s\n\x00"
                                                                               as
                                                                               *const u8
                                                                               as
                                                                               *const std::os::raw::c_char,
                                                                           filename);
            }
        }
        _ => {
            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                       b"To save to subparts of a document use the \'write\' command\n\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const std::os::raw::c_char);
            return -(1 as std::os::raw::c_int)
        }
    }
    return 0 as std::os::raw::c_int;
}
/* ***************************************************************
 *								*
 *			Checking routines			*
 *								*
 ****************************************************************/
/* ***************************************************************
 *								*
 *			XML shell helpers			*
 *								*
 ****************************************************************/
/* ***************************************************************
 *								*
 *	 The XML shell related structures and functions		*
 *								*
 ****************************************************************/
/* *
 * xmlShellReadlineFunc:
 * @prompt:  a string prompt
 *
 * This is a generic signature for the XML shell input function.
 *
 * Returns a string which will be freed by the Shell.
 */
/* *
 * xmlShellCtxt:
 *
 * A debugging shell context.
 * TODO: add the defined function tables.
 */
/* *
 * xmlShellCmd:
 * @ctxt:  a shell context
 * @arg:  a string argument
 * @node:  a first node
 * @node2:  a second node
 *
 * This is a generic signature for the XML shell functions.
 *
 * Returns an int, negative returns indicating errors.
 */
/* LIBXML_OUTPUT_ENABLED */
/* LIBXML_OUTPUT_ENABLED */
/* *
 * xmlShellValidate:
 * @ctxt:  the shell context
 * @dtd:  the DTD URI (optional)
 * @node:  unused
 * @node2:  unused
 *
 * Implements the XML shell function "validate"
 * Validate the document, if a DTD path is provided, then the validation
 * is done against the given DTD.
 *
 * Returns 0 or -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlShellValidate(mut ctxt: xmlShellCtxtPtr,
                                          mut dtd: *mut std::os::raw::c_char,
                                          mut node: xmlNodePtr,
                                          mut node2: xmlNodePtr)
 -> std::os::raw::c_int {
    let mut vctxt: xmlValidCtxt =
        xmlValidCtxt{userData: 0 as *mut std::os::raw::c_void,
                     error: None,
                     warning: None,
                     node: 0 as *mut xmlNode,
                     nodeNr: 0,
                     nodeMax: 0,
                     nodeTab: 0 as *mut xmlNodePtr,
                     finishDtd: 0,
                     doc: 0 as *mut xmlDoc,
                     valid: 0,
                     vstate: 0 as *mut xmlValidState,
                     vstateNr: 0,
                     vstateMax: 0,
                     vstateTab: 0 as *mut xmlValidState,
                     am: 0 as *mut xmlAutomata,
                     state: 0 as *mut xmlAutomataState,};
    let mut res: std::os::raw::c_int = -(1 as std::os::raw::c_int);
    if ctxt.is_null() || (*ctxt).doc.is_null() { return -(1 as std::os::raw::c_int) }
    vctxt.userData = stderr as *mut std::os::raw::c_void;
    vctxt.error =
        ::std::mem::transmute::<Option<unsafe extern "C" fn(_: *mut FILE,
                                                            _:
                                                                *const std::os::raw::c_char,
                                                            _: ...)
                                           -> std::os::raw::c_int>,
                                xmlValidityErrorFunc>(Some(fprintf as
                                                               unsafe extern "C" fn(_:
                                                                                        *mut FILE,
                                                                                    _:
                                                                                        *const std::os::raw::c_char,
                                                                                    _:
                                                                                        ...)
                                                                   ->
                                                                       std::os::raw::c_int));
    vctxt.warning =
        ::std::mem::transmute::<Option<unsafe extern "C" fn(_: *mut FILE,
                                                            _:
                                                                *const std::os::raw::c_char,
                                                            _: ...)
                                           -> std::os::raw::c_int>,
                                xmlValidityWarningFunc>(Some(fprintf as
                                                                 unsafe extern "C" fn(_:
                                                                                          *mut FILE,
                                                                                      _:
                                                                                          *const std::os::raw::c_char,
                                                                                      _:
                                                                                          ...)
                                                                     ->
                                                                         std::os::raw::c_int));
    if dtd.is_null() ||
           *dtd.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
               0 as std::os::raw::c_int {
        res = xmlValidateDocument(&mut vctxt, (*ctxt).doc)
    } else {
        let mut subset: xmlDtdPtr = 0 as *mut xmlDtd;
        subset = xmlParseDTD(0 as *const xmlChar, dtd as *mut xmlChar);
        if !subset.is_null() {
            res = xmlValidateDtd(&mut vctxt, (*ctxt).doc, subset);
            xmlFreeDtd(subset);
        }
    }
    return res;
}
/* LIBXML_VALID_ENABLED */
/* LIBXML_VALID_ENABLED */
/* *
 * xmlShellDu:
 * @ctxt:  the shell context
 * @arg:  unused
 * @tree:  a node defining a subtree
 * @node2:  unused
 *
 * Implements the XML shell function "du"
 * show the structure of the subtree under node @tree
 * If @tree is null, the command works on the current node.
 *
 * Returns 0 or -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlShellDu(mut ctxt: xmlShellCtxtPtr,
                                    mut arg: *mut std::os::raw::c_char,
                                    mut tree: xmlNodePtr,
                                    mut node2: xmlNodePtr) -> std::os::raw::c_int {
    let mut node: xmlNodePtr = 0 as *mut xmlNode;
    let mut indent: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut i: std::os::raw::c_int = 0;
    if ctxt.is_null() { return -(1 as std::os::raw::c_int) }
    if tree.is_null() { return -(1 as std::os::raw::c_int) }
    node = tree;
    while !node.is_null() {
        if (*node).type_0 as std::os::raw::c_uint ==
               XML_DOCUMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint ||
               (*node).type_0 as std::os::raw::c_uint ==
                   XML_HTML_DOCUMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
            fprintf((*ctxt).output,
                    b"/\n\x00" as *const u8 as *const std::os::raw::c_char);
        } else if (*node).type_0 as std::os::raw::c_uint ==
                      XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
            i = 0 as std::os::raw::c_int;
            while i < indent {
                fprintf((*ctxt).output,
                        b"  \x00" as *const u8 as *const std::os::raw::c_char);
                i += 1
            }
            if !(*node).ns.is_null() && !(*(*node).ns).prefix.is_null() {
                fprintf((*ctxt).output,
                        b"%s:\x00" as *const u8 as *const std::os::raw::c_char,
                        (*(*node).ns).prefix);
            }
            fprintf((*ctxt).output,
                    b"%s\n\x00" as *const u8 as *const std::os::raw::c_char,
                    (*node).name);
        }
        /*
         * Browse the full subtree, deep first
         */
        if (*node).type_0 as std::os::raw::c_uint ==
               XML_DOCUMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint ||
               (*node).type_0 as std::os::raw::c_uint ==
                   XML_HTML_DOCUMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
            node = (*(node as xmlDocPtr)).children
        } else if !(*node).children.is_null() &&
                      (*node).type_0 as std::os::raw::c_uint !=
                          XML_ENTITY_REF_NODE as std::os::raw::c_int as std::os::raw::c_uint {
            /* deep first */
            node = (*node).children;
            indent += 1
        } else if node != tree && !(*node).next.is_null() {
            /* then siblings */
            node = (*node).next
        } else if node != tree {
            /* go up to parents->next if needed */
            while node != tree {
                if !(*node).parent.is_null() {
                    node = (*node).parent;
                    indent -= 1
                }
                if node != tree && !(*node).next.is_null() {
                    node = (*node).next;
                    break ;
                } else if (*node).parent.is_null() {
                    node = 0 as xmlNodePtr;
                    break ;
                } else {
                    if !(node == tree) { continue ; }
                    node = 0 as xmlNodePtr;
                    break ;
                }
            }
            /* exit condition */
            if node == tree { node = 0 as xmlNodePtr }
        } else { node = 0 as xmlNodePtr }
    }
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlShellPwd:
 * @ctxt:  the shell context
 * @buffer:  the output buffer
 * @node:  a node
 * @node2:  unused
 *
 * Implements the XML shell function "pwd"
 * Show the full path from the root to the node, if needed building
 * thumblers when similar elements exists at a given ancestor level.
 * The output is compatible with XPath commands.
 *
 * Returns 0 or -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlShellPwd(mut ctxt: xmlShellCtxtPtr,
                                     mut buffer: *mut std::os::raw::c_char,
                                     mut node: xmlNodePtr,
                                     mut node2: xmlNodePtr) -> std::os::raw::c_int {
    let mut path: *mut xmlChar = 0 as *mut xmlChar;
    if node.is_null() || buffer.is_null() { return -(1 as std::os::raw::c_int) }
    path = xmlGetNodePath(node as *const xmlNode);
    if path.is_null() { return -(1 as std::os::raw::c_int) }
    /*
     * This test prevents buffer overflow, because this routine
     * is only called by xmlShell, in which the second argument is
     * 500 chars long.
     * It is a dirty hack before a cleaner solution is found.
     * Documentation should mention that the second argument must
     * be at least 500 chars long, and could be stripped if too long.
     */
    snprintf(buffer, 499 as std::os::raw::c_int as std::os::raw::c_ulong,
             b"%s\x00" as *const u8 as *const std::os::raw::c_char, path);
    *buffer.offset(499 as std::os::raw::c_int as isize) = '0' as i32 as std::os::raw::c_char;
    xmlFree.expect("non-null function pointer")(path as *mut std::os::raw::c_void);
    return 0 as std::os::raw::c_int;
}
/*
 * The Shell interface.
 */
/* *
 * xmlShell:
 * @doc:  the initial document
 * @filename:  the output buffer
 * @input:  the line reading function
 * @output:  the output FILE*, defaults to stdout if NULL
 *
 * Implements the XML shell
 * This allow to load, validate, view, modify and save a document
 * using a environment similar to a UNIX commandline.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlShell(mut doc: xmlDocPtr,
                                  mut filename: *mut std::os::raw::c_char,
                                  mut input: xmlShellReadlineFunc,
                                  mut output: *mut FILE) {
    let mut prompt: [std::os::raw::c_char; 500] =
        *::std::mem::transmute::<&[u8; 500],
                                 &mut [std::os::raw::c_char; 500]>(b"/ > \x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00");
    let mut cmdline: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut cur: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut command: [std::os::raw::c_char; 100] = [0; 100];
    let mut arg: [std::os::raw::c_char; 400] = [0; 400];
    let mut i: std::os::raw::c_int = 0;
    let mut ctxt: xmlShellCtxtPtr = 0 as *mut xmlShellCtxt;
    let mut list: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    if doc.is_null() { return }
    if filename.is_null() { return }
    if input.is_none() { return }
    if output.is_null() { output = stdout }
    ctxt =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlShellCtxt>()
                                                          as std::os::raw::c_ulong) as
            xmlShellCtxtPtr;
    if ctxt.is_null() { return }
    (*ctxt).loaded = 0 as std::os::raw::c_int;
    (*ctxt).doc = doc;
    (*ctxt).input = input;
    (*ctxt).output = output;
    (*ctxt).filename =
        xmlStrdup(filename as *mut xmlChar) as *mut std::os::raw::c_char;
    (*ctxt).node = (*ctxt).doc as xmlNodePtr;
    (*ctxt).pctxt = xmlXPathNewContext((*ctxt).doc);
    if (*ctxt).pctxt.is_null() {
        xmlFree.expect("non-null function pointer")(ctxt as
                                                        *mut std::os::raw::c_void);
        return
    }
    loop 
         /* LIBXML_XPATH_ENABLED */
         {
        if (*ctxt).node == (*ctxt).doc as xmlNodePtr {
            snprintf(prompt.as_mut_ptr(),
                     ::std::mem::size_of::<[std::os::raw::c_char; 500]>() as
                         std::os::raw::c_ulong,
                     b"%s > \x00" as *const u8 as *const std::os::raw::c_char,
                     b"/\x00" as *const u8 as *const std::os::raw::c_char);
        } else if !(*ctxt).node.is_null() && !(*(*ctxt).node).name.is_null()
                      && !(*(*ctxt).node).ns.is_null() &&
                      !(*(*(*ctxt).node).ns).prefix.is_null() {
            snprintf(prompt.as_mut_ptr(),
                     ::std::mem::size_of::<[std::os::raw::c_char; 500]>() as
                         std::os::raw::c_ulong,
                     b"%s:%s > \x00" as *const u8 as *const std::os::raw::c_char,
                     (*(*(*ctxt).node).ns).prefix, (*(*ctxt).node).name);
        } else if !(*ctxt).node.is_null() && !(*(*ctxt).node).name.is_null() {
            snprintf(prompt.as_mut_ptr(),
                     ::std::mem::size_of::<[std::os::raw::c_char; 500]>() as
                         std::os::raw::c_ulong,
                     b"%s > \x00" as *const u8 as *const std::os::raw::c_char,
                     (*(*ctxt).node).name);
        } else {
            snprintf(prompt.as_mut_ptr(),
                     ::std::mem::size_of::<[std::os::raw::c_char; 500]>() as
                         std::os::raw::c_ulong,
                     b"? > \x00" as *const u8 as *const std::os::raw::c_char);
        }
        prompt[(::std::mem::size_of::<[std::os::raw::c_char; 500]>() as
                    std::os::raw::c_ulong).wrapping_sub(1 as std::os::raw::c_int as
                                                    std::os::raw::c_ulong) as usize] =
            0 as std::os::raw::c_int as std::os::raw::c_char;
        /*
         * Get a new command line
         */
        cmdline =
            (*ctxt).input.expect("non-null function pointer")(prompt.as_mut_ptr());
        if cmdline.is_null() { break ; }
        /*
         * Parse the command itself
         */
        cur = cmdline;
        while *cur as std::os::raw::c_int == ' ' as i32 ||
                  *cur as std::os::raw::c_int == '\t' as i32 {
            cur = cur.offset(1)
        }
        i = 0 as std::os::raw::c_int;
        while *cur as std::os::raw::c_int != ' ' as i32 &&
                  *cur as std::os::raw::c_int != '\t' as i32 &&
                  *cur as std::os::raw::c_int != '\n' as i32 &&
                  *cur as std::os::raw::c_int != '\r' as i32 {
            if *cur as std::os::raw::c_int == 0 as std::os::raw::c_int { break ; }
            let fresh2 = cur;
            cur = cur.offset(1);
            let fresh3 = i;
            i = i + 1;
            command[fresh3 as usize] = *fresh2
        }
        command[i as usize] = 0 as std::os::raw::c_int as std::os::raw::c_char;
        if i == 0 as std::os::raw::c_int { continue ; }
        /*
         * Parse the argument
         */
        while *cur as std::os::raw::c_int == ' ' as i32 ||
                  *cur as std::os::raw::c_int == '\t' as i32 {
            cur = cur.offset(1)
        }
        i = 0 as std::os::raw::c_int;
        while *cur as std::os::raw::c_int != '\n' as i32 &&
                  *cur as std::os::raw::c_int != '\r' as i32 &&
                  *cur as std::os::raw::c_int != 0 as std::os::raw::c_int {
            if *cur as std::os::raw::c_int == 0 as std::os::raw::c_int { break ; }
            let fresh4 = cur;
            cur = cur.offset(1);
            let fresh5 = i;
            i = i + 1;
            arg[fresh5 as usize] = *fresh4
        }
        arg[i as usize] = 0 as std::os::raw::c_int as std::os::raw::c_char;
        /*
         * start interpreting the command
         */
        if strcmp(command.as_mut_ptr(),
                  b"exit\x00" as *const u8 as *const std::os::raw::c_char) == 0 {
            break ;
        }
        if strcmp(command.as_mut_ptr(),
                  b"quit\x00" as *const u8 as *const std::os::raw::c_char) == 0 {
            break ;
        }
        if strcmp(command.as_mut_ptr(),
                  b"bye\x00" as *const u8 as *const std::os::raw::c_char) == 0 {
            break ;
        }
        if strcmp(command.as_mut_ptr(),
                  b"help\x00" as *const u8 as *const std::os::raw::c_char) == 0 {
            fprintf((*ctxt).output,
                    b"\tbase         display XML base of the node\n\x00" as
                        *const u8 as *const std::os::raw::c_char);
            fprintf((*ctxt).output,
                    b"\tsetbase URI  change the XML base of the node\n\x00" as
                        *const u8 as *const std::os::raw::c_char);
            fprintf((*ctxt).output,
                    b"\tbye          leave shell\n\x00" as *const u8 as
                        *const std::os::raw::c_char);
            fprintf((*ctxt).output,
                    b"\tcat [node]   display node or current node\n\x00" as
                        *const u8 as *const std::os::raw::c_char);
            fprintf((*ctxt).output,
                    b"\tcd [path]    change directory to path or to root\n\x00"
                        as *const u8 as *const std::os::raw::c_char);
            fprintf((*ctxt).output,
                    b"\tdir [path]   dumps informations about the node (namespace, attributes, content)\n\x00"
                        as *const u8 as *const std::os::raw::c_char);
            fprintf((*ctxt).output,
                    b"\tdu [path]    show the structure of the subtree under path or the current node\n\x00"
                        as *const u8 as *const std::os::raw::c_char);
            fprintf((*ctxt).output,
                    b"\texit         leave shell\n\x00" as *const u8 as
                        *const std::os::raw::c_char);
            fprintf((*ctxt).output,
                    b"\thelp         display this help\n\x00" as *const u8 as
                        *const std::os::raw::c_char);
            fprintf((*ctxt).output,
                    b"\tfree         display memory usage\n\x00" as *const u8
                        as *const std::os::raw::c_char);
            fprintf((*ctxt).output,
                    b"\tload [name]  load a new document with name\n\x00" as
                        *const u8 as *const std::os::raw::c_char);
            fprintf((*ctxt).output,
                    b"\tls [path]    list contents of path or the current directory\n\x00"
                        as *const u8 as *const std::os::raw::c_char);
            fprintf((*ctxt).output,
                    b"\tset xml_fragment replace the current node content with the fragment parsed in context\n\x00"
                        as *const u8 as *const std::os::raw::c_char);
            fprintf((*ctxt).output,
                    b"\txpath expr   evaluate the XPath expression in that context and print the result\n\x00"
                        as *const u8 as *const std::os::raw::c_char);
            fprintf((*ctxt).output,
                    b"\tsetns nsreg  register a namespace to a prefix in the XPath evaluation context\n\x00"
                        as *const u8 as *const std::os::raw::c_char);
            fprintf((*ctxt).output,
                    b"\t             format for nsreg is: prefix=[nsuri] (i.e. prefix= unsets a prefix)\n\x00"
                        as *const u8 as *const std::os::raw::c_char);
            fprintf((*ctxt).output,
                    b"\tsetrootns    register all namespace found on the root element\n\x00"
                        as *const u8 as *const std::os::raw::c_char);
            fprintf((*ctxt).output,
                    b"\t             the default namespace if any uses \'defaultns\' prefix\n\x00"
                        as *const u8 as *const std::os::raw::c_char);
            /* LIBXML_XPATH_ENABLED */
            fprintf((*ctxt).output,
                    b"\tpwd          display current working directory\n\x00"
                        as *const u8 as *const std::os::raw::c_char);
            fprintf((*ctxt).output,
                    b"\twhereis      display absolute path of [path] or current working directory\n\x00"
                        as *const u8 as *const std::os::raw::c_char);
            fprintf((*ctxt).output,
                    b"\tquit         leave shell\n\x00" as *const u8 as
                        *const std::os::raw::c_char);
            fprintf((*ctxt).output,
                    b"\tsave [name]  save this document to name or the original name\n\x00"
                        as *const u8 as *const std::os::raw::c_char);
            fprintf((*ctxt).output,
                    b"\twrite [name] write the current node to the filename\n\x00"
                        as *const u8 as *const std::os::raw::c_char);
            /* LIBXML_OUTPUT_ENABLED */
            fprintf((*ctxt).output,
                    b"\tvalidate     check the document for errors\n\x00" as
                        *const u8 as *const std::os::raw::c_char);
            /* LIBXML_VALID_ENABLED */
            fprintf((*ctxt).output,
                    b"\trelaxng rng  validate the document against the Relax-NG schemas\n\x00"
                        as *const u8 as *const std::os::raw::c_char);
            fprintf((*ctxt).output,
                    b"\tgrep string  search for a string in the subtree\n\x00"
                        as *const u8 as *const std::os::raw::c_char);
        } else if strcmp(command.as_mut_ptr(),
                         b"validate\x00" as *const u8 as *const std::os::raw::c_char)
                      == 0 {
            xmlShellValidate(ctxt, arg.as_mut_ptr(), 0 as xmlNodePtr,
                             0 as xmlNodePtr);
            /* LIBXML_VALID_ENABLED */
        } else if strcmp(command.as_mut_ptr(),
                         b"load\x00" as *const u8 as *const std::os::raw::c_char) == 0
         {
            xmlShellLoad(ctxt, arg.as_mut_ptr(), 0 as xmlNodePtr,
                         0 as xmlNodePtr);
        } else if strcmp(command.as_mut_ptr(),
                         b"relaxng\x00" as *const u8 as *const std::os::raw::c_char)
                      == 0 {
            xmlShellRNGValidate(ctxt, arg.as_mut_ptr(), 0 as xmlNodePtr,
                                0 as xmlNodePtr);
        } else if strcmp(command.as_mut_ptr(),
                         b"save\x00" as *const u8 as *const std::os::raw::c_char) == 0
         {
            xmlShellSave(ctxt, arg.as_mut_ptr(), 0 as xmlNodePtr,
                         0 as xmlNodePtr);
        } else if strcmp(command.as_mut_ptr(),
                         b"write\x00" as *const u8 as *const std::os::raw::c_char) ==
                      0 {
            if arg[0 as std::os::raw::c_int as usize] as std::os::raw::c_int ==
                   0 as std::os::raw::c_int {
                (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                           b"Write command requires a filename argument\n\x00"
                                                                               as
                                                                               *const u8
                                                                               as
                                                                               *const std::os::raw::c_char);
            } else {
                xmlShellWrite(ctxt, arg.as_mut_ptr(), (*ctxt).node,
                              0 as xmlNodePtr);
            }
            /* LIBXML_OUTPUT_ENABLED */
        } else if strcmp(command.as_mut_ptr(),
                         b"grep\x00" as *const u8 as *const std::os::raw::c_char) == 0
         {
            xmlShellGrep(ctxt, arg.as_mut_ptr(), (*ctxt).node,
                         0 as xmlNodePtr);
        } else if strcmp(command.as_mut_ptr(),
                         b"free\x00" as *const u8 as *const std::os::raw::c_char) == 0
         {
            if arg[0 as std::os::raw::c_int as usize] as std::os::raw::c_int ==
                   0 as std::os::raw::c_int {
                xmlMemShow((*ctxt).output, 0 as std::os::raw::c_int);
            } else {
                let mut len: std::os::raw::c_int = 0 as std::os::raw::c_int;
                sscanf(arg.as_mut_ptr(),
                       b"%d\x00" as *const u8 as *const std::os::raw::c_char,
                       &mut len as *mut std::os::raw::c_int);
                xmlMemShow((*ctxt).output, len);
            }
        } else if strcmp(command.as_mut_ptr(),
                         b"pwd\x00" as *const u8 as *const std::os::raw::c_char) == 0
         {
            let mut dir: [std::os::raw::c_char; 500] = [0; 500];
            if xmlShellPwd(ctxt, dir.as_mut_ptr(), (*ctxt).node,
                           0 as xmlNodePtr) == 0 {
                fprintf((*ctxt).output,
                        b"%s\n\x00" as *const u8 as *const std::os::raw::c_char,
                        dir.as_mut_ptr());
            }
        } else if strcmp(command.as_mut_ptr(),
                         b"du\x00" as *const u8 as *const std::os::raw::c_char) == 0 {
            if arg[0 as std::os::raw::c_int as usize] as std::os::raw::c_int ==
                   0 as std::os::raw::c_int {
                xmlShellDu(ctxt, 0 as *mut std::os::raw::c_char, (*ctxt).node,
                           0 as xmlNodePtr);
            } else {
                (*(*ctxt).pctxt).node = (*ctxt).node;
                (*(*ctxt).pctxt).node = (*ctxt).node;
                list =
                    xmlXPathEval(arg.as_mut_ptr() as *mut xmlChar,
                                 (*ctxt).pctxt);
                /* LIBXML_XPATH_ENABLED */
                if !list.is_null() {
                    match (*list).type_0 as std::os::raw::c_uint {
                        0 => {
                            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                                       b"%s: no such node\n\x00"
                                                                                           as
                                                                                           *const u8
                                                                                           as
                                                                                           *const std::os::raw::c_char,
                                                                                       arg.as_mut_ptr());
                        }
                        1 => {
                            let mut indx: std::os::raw::c_int = 0;
                            if !(*list).nodesetval.is_null() {
                                indx = 0 as std::os::raw::c_int;
                                while indx < (*(*list).nodesetval).nodeNr {
                                    xmlShellDu(ctxt, 0 as *mut std::os::raw::c_char,
                                               *(*(*list).nodesetval).nodeTab.offset(indx
                                                                                         as
                                                                                         isize),
                                               0 as xmlNodePtr);
                                    indx += 1
                                }
                            }
                        }
                        2 => {
                            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                                       b"%s is a Boolean\n\x00"
                                                                                           as
                                                                                           *const u8
                                                                                           as
                                                                                           *const std::os::raw::c_char,
                                                                                       arg.as_mut_ptr());
                        }
                        3 => {
                            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                                       b"%s is a number\n\x00"
                                                                                           as
                                                                                           *const u8
                                                                                           as
                                                                                           *const std::os::raw::c_char,
                                                                                       arg.as_mut_ptr());
                        }
                        4 => {
                            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                                       b"%s is a string\n\x00"
                                                                                           as
                                                                                           *const u8
                                                                                           as
                                                                                           *const std::os::raw::c_char,
                                                                                       arg.as_mut_ptr());
                        }
                        5 => {
                            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                                       b"%s is a point\n\x00"
                                                                                           as
                                                                                           *const u8
                                                                                           as
                                                                                           *const std::os::raw::c_char,
                                                                                       arg.as_mut_ptr());
                        }
                        6 => {
                            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                                       b"%s is a range\n\x00"
                                                                                           as
                                                                                           *const u8
                                                                                           as
                                                                                           *const std::os::raw::c_char,
                                                                                       arg.as_mut_ptr());
                        }
                        7 => {
                            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                                       b"%s is a range\n\x00"
                                                                                           as
                                                                                           *const u8
                                                                                           as
                                                                                           *const std::os::raw::c_char,
                                                                                       arg.as_mut_ptr());
                        }
                        8 => {
                            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                                       b"%s is user-defined\n\x00"
                                                                                           as
                                                                                           *const u8
                                                                                           as
                                                                                           *const std::os::raw::c_char,
                                                                                       arg.as_mut_ptr());
                        }
                        9 => {
                            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                                       b"%s is an XSLT value tree\n\x00"
                                                                                           as
                                                                                           *const u8
                                                                                           as
                                                                                           *const std::os::raw::c_char,
                                                                                       arg.as_mut_ptr());
                        }
                        _ => { }
                    }
                    xmlXPathFreeObject(list);
                } else {
                    (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                               b"%s: no such node\n\x00"
                                                                                   as
                                                                                   *const u8
                                                                                   as
                                                                                   *const std::os::raw::c_char,
                                                                               arg.as_mut_ptr());
                }
                (*(*ctxt).pctxt).node = 0 as xmlNodePtr
            }
        } else if strcmp(command.as_mut_ptr(),
                         b"base\x00" as *const u8 as *const std::os::raw::c_char) == 0
         {
            xmlShellBase(ctxt, 0 as *mut std::os::raw::c_char, (*ctxt).node,
                         0 as xmlNodePtr);
        } else if strcmp(command.as_mut_ptr(),
                         b"set\x00" as *const u8 as *const std::os::raw::c_char) == 0
         {
            xmlShellSetContent(ctxt, arg.as_mut_ptr(), (*ctxt).node,
                               0 as xmlNodePtr);
        } else if strcmp(command.as_mut_ptr(),
                         b"setns\x00" as *const u8 as *const std::os::raw::c_char) ==
                      0 {
            if arg[0 as std::os::raw::c_int as usize] as std::os::raw::c_int ==
                   0 as std::os::raw::c_int {
                (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                           b"setns: prefix=[nsuri] required\n\x00"
                                                                               as
                                                                               *const u8
                                                                               as
                                                                               *const std::os::raw::c_char);
            } else {
                xmlShellRegisterNamespace(ctxt, arg.as_mut_ptr(),
                                          0 as xmlNodePtr, 0 as xmlNodePtr);
            }
        } else if strcmp(command.as_mut_ptr(),
                         b"setrootns\x00" as *const u8 as *const std::os::raw::c_char)
                      == 0 {
            let mut root: xmlNodePtr = 0 as *mut xmlNode;
            root = xmlDocGetRootElement((*ctxt).doc as *const xmlDoc);
            xmlShellRegisterRootNamespaces(ctxt, 0 as *mut std::os::raw::c_char, root,
                                           0 as xmlNodePtr);
        } else if strcmp(command.as_mut_ptr(),
                         b"xpath\x00" as *const u8 as *const std::os::raw::c_char) ==
                      0 {
            if arg[0 as std::os::raw::c_int as usize] as std::os::raw::c_int ==
                   0 as std::os::raw::c_int {
                (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                           b"xpath: expression required\n\x00"
                                                                               as
                                                                               *const u8
                                                                               as
                                                                               *const std::os::raw::c_char);
            } else {
                (*(*ctxt).pctxt).node = (*ctxt).node;
                list =
                    xmlXPathEval(arg.as_mut_ptr() as *mut xmlChar,
                                 (*ctxt).pctxt);
                xmlXPathDebugDumpObject((*ctxt).output, list,
                                        0 as std::os::raw::c_int);
                xmlXPathFreeObject(list);
            }
            /* LIBXML_XPATH_ENABLED */
        } else if strcmp(command.as_mut_ptr(),
                         b"setbase\x00" as *const u8 as *const std::os::raw::c_char)
                      == 0 {
            xmlShellSetBase(ctxt, arg.as_mut_ptr(), (*ctxt).node,
                            0 as xmlNodePtr);
        } else if strcmp(command.as_mut_ptr(),
                         b"ls\x00" as *const u8 as *const std::os::raw::c_char) == 0
                      ||
                      strcmp(command.as_mut_ptr(),
                             b"dir\x00" as *const u8 as *const std::os::raw::c_char)
                          == 0 {
            let mut dir_0: std::os::raw::c_int =
                (strcmp(command.as_mut_ptr(),
                        b"dir\x00" as *const u8 as *const std::os::raw::c_char) == 0)
                    as std::os::raw::c_int;
            if arg[0 as std::os::raw::c_int as usize] as std::os::raw::c_int ==
                   0 as std::os::raw::c_int {
                if dir_0 != 0 {
                    xmlShellDir(ctxt, 0 as *mut std::os::raw::c_char, (*ctxt).node,
                                0 as xmlNodePtr);
                } else {
                    xmlShellList(ctxt, 0 as *mut std::os::raw::c_char, (*ctxt).node,
                                 0 as xmlNodePtr);
                }
            } else {
                (*(*ctxt).pctxt).node = (*ctxt).node;
                (*(*ctxt).pctxt).node = (*ctxt).node;
                list =
                    xmlXPathEval(arg.as_mut_ptr() as *mut xmlChar,
                                 (*ctxt).pctxt);
                /* LIBXML_XPATH_ENABLED */
                if !list.is_null() {
                    match (*list).type_0 as std::os::raw::c_uint {
                        0 => {
                            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                                       b"%s: no such node\n\x00"
                                                                                           as
                                                                                           *const u8
                                                                                           as
                                                                                           *const std::os::raw::c_char,
                                                                                       arg.as_mut_ptr());
                        }
                        1 => {
                            let mut indx_0: std::os::raw::c_int = 0;
                            if !(*list).nodesetval.is_null() {
                                indx_0 = 0 as std::os::raw::c_int;
                                while indx_0 < (*(*list).nodesetval).nodeNr {
                                    if dir_0 != 0 {
                                        xmlShellDir(ctxt,
                                                    0 as *mut std::os::raw::c_char,
                                                    *(*(*list).nodesetval).nodeTab.offset(indx_0
                                                                                              as
                                                                                              isize),
                                                    0 as xmlNodePtr);
                                    } else {
                                        xmlShellList(ctxt,
                                                     0 as *mut std::os::raw::c_char,
                                                     *(*(*list).nodesetval).nodeTab.offset(indx_0
                                                                                               as
                                                                                               isize),
                                                     0 as xmlNodePtr);
                                    }
                                    indx_0 += 1
                                }
                            }
                        }
                        2 => {
                            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                                       b"%s is a Boolean\n\x00"
                                                                                           as
                                                                                           *const u8
                                                                                           as
                                                                                           *const std::os::raw::c_char,
                                                                                       arg.as_mut_ptr());
                        }
                        3 => {
                            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                                       b"%s is a number\n\x00"
                                                                                           as
                                                                                           *const u8
                                                                                           as
                                                                                           *const std::os::raw::c_char,
                                                                                       arg.as_mut_ptr());
                        }
                        4 => {
                            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                                       b"%s is a string\n\x00"
                                                                                           as
                                                                                           *const u8
                                                                                           as
                                                                                           *const std::os::raw::c_char,
                                                                                       arg.as_mut_ptr());
                        }
                        5 => {
                            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                                       b"%s is a point\n\x00"
                                                                                           as
                                                                                           *const u8
                                                                                           as
                                                                                           *const std::os::raw::c_char,
                                                                                       arg.as_mut_ptr());
                        }
                        6 => {
                            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                                       b"%s is a range\n\x00"
                                                                                           as
                                                                                           *const u8
                                                                                           as
                                                                                           *const std::os::raw::c_char,
                                                                                       arg.as_mut_ptr());
                        }
                        7 => {
                            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                                       b"%s is a range\n\x00"
                                                                                           as
                                                                                           *const u8
                                                                                           as
                                                                                           *const std::os::raw::c_char,
                                                                                       arg.as_mut_ptr());
                        }
                        8 => {
                            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                                       b"%s is user-defined\n\x00"
                                                                                           as
                                                                                           *const u8
                                                                                           as
                                                                                           *const std::os::raw::c_char,
                                                                                       arg.as_mut_ptr());
                        }
                        9 => {
                            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                                       b"%s is an XSLT value tree\n\x00"
                                                                                           as
                                                                                           *const u8
                                                                                           as
                                                                                           *const std::os::raw::c_char,
                                                                                       arg.as_mut_ptr());
                        }
                        _ => { }
                    }
                    xmlXPathFreeObject(list);
                } else {
                    (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                               b"%s: no such node\n\x00"
                                                                                   as
                                                                                   *const u8
                                                                                   as
                                                                                   *const std::os::raw::c_char,
                                                                               arg.as_mut_ptr());
                }
                (*(*ctxt).pctxt).node = 0 as xmlNodePtr
            }
        } else if strcmp(command.as_mut_ptr(),
                         b"whereis\x00" as *const u8 as *const std::os::raw::c_char)
                      == 0 {
            let mut dir_1: [std::os::raw::c_char; 500] = [0; 500];
            if arg[0 as std::os::raw::c_int as usize] as std::os::raw::c_int ==
                   0 as std::os::raw::c_int {
                if xmlShellPwd(ctxt, dir_1.as_mut_ptr(), (*ctxt).node,
                               0 as xmlNodePtr) == 0 {
                    fprintf((*ctxt).output,
                            b"%s\n\x00" as *const u8 as *const std::os::raw::c_char,
                            dir_1.as_mut_ptr());
                }
            } else {
                (*(*ctxt).pctxt).node = (*ctxt).node;
                list =
                    xmlXPathEval(arg.as_mut_ptr() as *mut xmlChar,
                                 (*ctxt).pctxt);
                /* LIBXML_XPATH_ENABLED */
                if !list.is_null() {
                    match (*list).type_0 as std::os::raw::c_uint {
                        0 => {
                            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                                       b"%s: no such node\n\x00"
                                                                                           as
                                                                                           *const u8
                                                                                           as
                                                                                           *const std::os::raw::c_char,
                                                                                       arg.as_mut_ptr());
                        }
                        1 => {
                            let mut indx_1: std::os::raw::c_int = 0;
                            if !(*list).nodesetval.is_null() {
                                indx_1 = 0 as std::os::raw::c_int;
                                while indx_1 < (*(*list).nodesetval).nodeNr {
                                    if xmlShellPwd(ctxt, dir_1.as_mut_ptr(),
                                                   *(*(*list).nodesetval).nodeTab.offset(indx_1
                                                                                             as
                                                                                             isize),
                                                   0 as xmlNodePtr) == 0 {
                                        fprintf((*ctxt).output,
                                                b"%s\n\x00" as *const u8 as
                                                    *const std::os::raw::c_char,
                                                dir_1.as_mut_ptr());
                                    }
                                    indx_1 += 1
                                }
                            }
                        }
                        2 => {
                            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                                       b"%s is a Boolean\n\x00"
                                                                                           as
                                                                                           *const u8
                                                                                           as
                                                                                           *const std::os::raw::c_char,
                                                                                       arg.as_mut_ptr());
                        }
                        3 => {
                            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                                       b"%s is a number\n\x00"
                                                                                           as
                                                                                           *const u8
                                                                                           as
                                                                                           *const std::os::raw::c_char,
                                                                                       arg.as_mut_ptr());
                        }
                        4 => {
                            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                                       b"%s is a string\n\x00"
                                                                                           as
                                                                                           *const u8
                                                                                           as
                                                                                           *const std::os::raw::c_char,
                                                                                       arg.as_mut_ptr());
                        }
                        5 => {
                            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                                       b"%s is a point\n\x00"
                                                                                           as
                                                                                           *const u8
                                                                                           as
                                                                                           *const std::os::raw::c_char,
                                                                                       arg.as_mut_ptr());
                        }
                        6 => {
                            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                                       b"%s is a range\n\x00"
                                                                                           as
                                                                                           *const u8
                                                                                           as
                                                                                           *const std::os::raw::c_char,
                                                                                       arg.as_mut_ptr());
                        }
                        7 => {
                            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                                       b"%s is a range\n\x00"
                                                                                           as
                                                                                           *const u8
                                                                                           as
                                                                                           *const std::os::raw::c_char,
                                                                                       arg.as_mut_ptr());
                        }
                        8 => {
                            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                                       b"%s is user-defined\n\x00"
                                                                                           as
                                                                                           *const u8
                                                                                           as
                                                                                           *const std::os::raw::c_char,
                                                                                       arg.as_mut_ptr());
                        }
                        9 => {
                            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                                       b"%s is an XSLT value tree\n\x00"
                                                                                           as
                                                                                           *const u8
                                                                                           as
                                                                                           *const std::os::raw::c_char,
                                                                                       arg.as_mut_ptr());
                        }
                        _ => { }
                    }
                    xmlXPathFreeObject(list);
                } else {
                    (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                               b"%s: no such node\n\x00"
                                                                                   as
                                                                                   *const u8
                                                                                   as
                                                                                   *const std::os::raw::c_char,
                                                                               arg.as_mut_ptr());
                }
                (*(*ctxt).pctxt).node = 0 as xmlNodePtr
            }
        } else if strcmp(command.as_mut_ptr(),
                         b"cd\x00" as *const u8 as *const std::os::raw::c_char) == 0 {
            if arg[0 as std::os::raw::c_int as usize] as std::os::raw::c_int ==
                   0 as std::os::raw::c_int {
                (*ctxt).node = (*ctxt).doc as xmlNodePtr
            } else {
                let mut l: std::os::raw::c_int = 0;
                (*(*ctxt).pctxt).node = (*ctxt).node;
                l = strlen(arg.as_mut_ptr()) as std::os::raw::c_int;
                if l >= 2 as std::os::raw::c_int &&
                       arg[(l - 1 as std::os::raw::c_int) as usize] as std::os::raw::c_int ==
                           '/' as i32 {
                    arg[(l - 1 as std::os::raw::c_int) as usize] =
                        0 as std::os::raw::c_int as std::os::raw::c_char
                }
                list =
                    xmlXPathEval(arg.as_mut_ptr() as *mut xmlChar,
                                 (*ctxt).pctxt);
                /* LIBXML_XPATH_ENABLED */
                if !list.is_null() {
                    match (*list).type_0 as std::os::raw::c_uint {
                        0 => {
                            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                                       b"%s: no such node\n\x00"
                                                                                           as
                                                                                           *const u8
                                                                                           as
                                                                                           *const std::os::raw::c_char,
                                                                                       arg.as_mut_ptr());
                        }
                        1 => {
                            if !(*list).nodesetval.is_null() {
                                if (*(*list).nodesetval).nodeNr ==
                                       1 as std::os::raw::c_int {
                                    (*ctxt).node =
                                        *(*(*list).nodesetval).nodeTab.offset(0
                                                                                  as
                                                                                  std::os::raw::c_int
                                                                                  as
                                                                                  isize);
                                    if !(*ctxt).node.is_null() &&
                                           (*(*ctxt).node).type_0 as
                                               std::os::raw::c_uint ==
                                               XML_NAMESPACE_DECL as
                                                   std::os::raw::c_int as std::os::raw::c_uint
                                       {
                                        (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                                                   b"cannot cd to namespace\n\x00"
                                                                                                       as
                                                                                                       *const u8
                                                                                                       as
                                                                                                       *const std::os::raw::c_char);
                                        (*ctxt).node = 0 as xmlNodePtr
                                    }
                                } else {
                                    (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                                               b"%s is a %d Node Set\n\x00"
                                                                                                   as
                                                                                                   *const u8
                                                                                                   as
                                                                                                   *const std::os::raw::c_char,
                                                                                               arg.as_mut_ptr(),
                                                                                               (*(*list).nodesetval).nodeNr);
                                }
                            } else {
                                (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                                           b"%s is an empty Node Set\n\x00"
                                                                                               as
                                                                                               *const u8
                                                                                               as
                                                                                               *const std::os::raw::c_char,
                                                                                           arg.as_mut_ptr());
                            }
                        }
                        2 => {
                            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                                       b"%s is a Boolean\n\x00"
                                                                                           as
                                                                                           *const u8
                                                                                           as
                                                                                           *const std::os::raw::c_char,
                                                                                       arg.as_mut_ptr());
                        }
                        3 => {
                            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                                       b"%s is a number\n\x00"
                                                                                           as
                                                                                           *const u8
                                                                                           as
                                                                                           *const std::os::raw::c_char,
                                                                                       arg.as_mut_ptr());
                        }
                        4 => {
                            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                                       b"%s is a string\n\x00"
                                                                                           as
                                                                                           *const u8
                                                                                           as
                                                                                           *const std::os::raw::c_char,
                                                                                       arg.as_mut_ptr());
                        }
                        5 => {
                            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                                       b"%s is a point\n\x00"
                                                                                           as
                                                                                           *const u8
                                                                                           as
                                                                                           *const std::os::raw::c_char,
                                                                                       arg.as_mut_ptr());
                        }
                        6 => {
                            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                                       b"%s is a range\n\x00"
                                                                                           as
                                                                                           *const u8
                                                                                           as
                                                                                           *const std::os::raw::c_char,
                                                                                       arg.as_mut_ptr());
                        }
                        7 => {
                            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                                       b"%s is a range\n\x00"
                                                                                           as
                                                                                           *const u8
                                                                                           as
                                                                                           *const std::os::raw::c_char,
                                                                                       arg.as_mut_ptr());
                        }
                        8 => {
                            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                                       b"%s is user-defined\n\x00"
                                                                                           as
                                                                                           *const u8
                                                                                           as
                                                                                           *const std::os::raw::c_char,
                                                                                       arg.as_mut_ptr());
                        }
                        9 => {
                            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                                       b"%s is an XSLT value tree\n\x00"
                                                                                           as
                                                                                           *const u8
                                                                                           as
                                                                                           *const std::os::raw::c_char,
                                                                                       arg.as_mut_ptr());
                        }
                        _ => { }
                    }
                    xmlXPathFreeObject(list);
                } else {
                    (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                               b"%s: no such node\n\x00"
                                                                                   as
                                                                                   *const u8
                                                                                   as
                                                                                   *const std::os::raw::c_char,
                                                                               arg.as_mut_ptr());
                }
                (*(*ctxt).pctxt).node = 0 as xmlNodePtr
            }
        } else if strcmp(command.as_mut_ptr(),
                         b"cat\x00" as *const u8 as *const std::os::raw::c_char) == 0
         {
            if arg[0 as std::os::raw::c_int as usize] as std::os::raw::c_int ==
                   0 as std::os::raw::c_int {
                xmlShellCat(ctxt, 0 as *mut std::os::raw::c_char, (*ctxt).node,
                            0 as xmlNodePtr);
            } else {
                (*(*ctxt).pctxt).node = (*ctxt).node;
                (*(*ctxt).pctxt).node = (*ctxt).node;
                list =
                    xmlXPathEval(arg.as_mut_ptr() as *mut xmlChar,
                                 (*ctxt).pctxt);
                /* LIBXML_XPATH_ENABLED */
                if !list.is_null() {
                    match (*list).type_0 as std::os::raw::c_uint {
                        0 => {
                            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                                       b"%s: no such node\n\x00"
                                                                                           as
                                                                                           *const u8
                                                                                           as
                                                                                           *const std::os::raw::c_char,
                                                                                       arg.as_mut_ptr());
                        }
                        1 => {
                            let mut indx_2: std::os::raw::c_int = 0;
                            if !(*list).nodesetval.is_null() {
                                indx_2 = 0 as std::os::raw::c_int;
                                while indx_2 < (*(*list).nodesetval).nodeNr {
                                    if i > 0 as std::os::raw::c_int {
                                        fprintf((*ctxt).output,
                                                b" -------\n\x00" as *const u8
                                                    as *const std::os::raw::c_char);
                                    }
                                    xmlShellCat(ctxt, 0 as *mut std::os::raw::c_char,
                                                *(*(*list).nodesetval).nodeTab.offset(indx_2
                                                                                          as
                                                                                          isize),
                                                0 as xmlNodePtr);
                                    indx_2 += 1
                                }
                            }
                        }
                        2 => {
                            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                                       b"%s is a Boolean\n\x00"
                                                                                           as
                                                                                           *const u8
                                                                                           as
                                                                                           *const std::os::raw::c_char,
                                                                                       arg.as_mut_ptr());
                        }
                        3 => {
                            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                                       b"%s is a number\n\x00"
                                                                                           as
                                                                                           *const u8
                                                                                           as
                                                                                           *const std::os::raw::c_char,
                                                                                       arg.as_mut_ptr());
                        }
                        4 => {
                            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                                       b"%s is a string\n\x00"
                                                                                           as
                                                                                           *const u8
                                                                                           as
                                                                                           *const std::os::raw::c_char,
                                                                                       arg.as_mut_ptr());
                        }
                        5 => {
                            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                                       b"%s is a point\n\x00"
                                                                                           as
                                                                                           *const u8
                                                                                           as
                                                                                           *const std::os::raw::c_char,
                                                                                       arg.as_mut_ptr());
                        }
                        6 => {
                            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                                       b"%s is a range\n\x00"
                                                                                           as
                                                                                           *const u8
                                                                                           as
                                                                                           *const std::os::raw::c_char,
                                                                                       arg.as_mut_ptr());
                        }
                        7 => {
                            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                                       b"%s is a range\n\x00"
                                                                                           as
                                                                                           *const u8
                                                                                           as
                                                                                           *const std::os::raw::c_char,
                                                                                       arg.as_mut_ptr());
                        }
                        8 => {
                            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                                       b"%s is user-defined\n\x00"
                                                                                           as
                                                                                           *const u8
                                                                                           as
                                                                                           *const std::os::raw::c_char,
                                                                                       arg.as_mut_ptr());
                        }
                        9 => {
                            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                                       b"%s is an XSLT value tree\n\x00"
                                                                                           as
                                                                                           *const u8
                                                                                           as
                                                                                           *const std::os::raw::c_char,
                                                                                       arg.as_mut_ptr());
                        }
                        _ => { }
                    }
                    xmlXPathFreeObject(list);
                } else {
                    (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                               b"%s: no such node\n\x00"
                                                                                   as
                                                                                   *const u8
                                                                                   as
                                                                                   *const std::os::raw::c_char,
                                                                               arg.as_mut_ptr());
                }
                (*(*ctxt).pctxt).node = 0 as xmlNodePtr
            }
            /* LIBXML_OUTPUT_ENABLED */
        } else {
            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                       b"Unknown command %s\n\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const std::os::raw::c_char,
                                                                       command.as_mut_ptr()); /* not xmlFree here ! */
        }
        free(cmdline as *mut std::os::raw::c_void);
        cmdline = 0 as *mut std::os::raw::c_char
    }
    xmlXPathFreeContext((*ctxt).pctxt);
    /* LIBXML_XPATH_ENABLED */
    if (*ctxt).loaded != 0 { xmlFreeDoc((*ctxt).doc); }
    if !(*ctxt).filename.is_null() {
        xmlFree.expect("non-null function pointer")((*ctxt).filename as
                                                        *mut std::os::raw::c_void);
    }
    xmlFree.expect("non-null function pointer")(ctxt as *mut std::os::raw::c_void);
    if !cmdline.is_null() { free(cmdline as *mut std::os::raw::c_void); };
    /* not xmlFree here ! */
}
/* LIBXML_DEBUG_ENABLED */
/* __INCLUDE_ELFGCCHACK */
/* LIBXML_XPATH_ENABLED */
