
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type _xmlBuf;
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
    static mut stdout: *mut FILE;
    #[no_mangle]
    fn snprintf(_: *mut std::os::raw::c_char, _: std::os::raw::c_ulong,
                _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
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
    /* *
 * BAD_CAST:
 *
 * Macro to cast a string to an xmlChar * when one know its safe.
 */
    /*
 * xmlChar handling
 */
    #[no_mangle]
    fn xmlStrdup(cur: *const xmlChar) -> *mut xmlChar;
    #[no_mangle]
    fn fwrite(__ptr: *const std::os::raw::c_void, __size: size_t, __n: size_t,
              __s: *mut FILE) -> size_t;
    #[no_mangle]
    fn xmlStrlen(str: *const xmlChar) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlStrncmp(str1: *const xmlChar, str2: *const xmlChar,
                  len: std::os::raw::c_int) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlStrchr(str: *const xmlChar, val: xmlChar) -> *const xmlChar;
    #[no_mangle]
    fn xmlStrndup(cur: *const xmlChar, len: std::os::raw::c_int) -> *mut xmlChar;
    #[no_mangle]
    fn xmlStrcasecmp(str1: *const xmlChar, str2: *const xmlChar)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlStrEqual(str1: *const xmlChar, str2: *const xmlChar) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlStrcat(cur: *mut xmlChar, add: *const xmlChar) -> *mut xmlChar;
    #[no_mangle]
    fn xmlStrncat(cur: *mut xmlChar, add: *const xmlChar, len: std::os::raw::c_int)
     -> *mut xmlChar;
    #[no_mangle]
    fn xmlStrncatNew(str1: *const xmlChar, str2: *const xmlChar,
                     len: std::os::raw::c_int) -> *mut xmlChar;
    #[no_mangle]
    fn xmlCheckUTF8(utf: *const std::os::raw::c_uchar) -> std::os::raw::c_int;
    #[no_mangle]
    fn memset(_: *mut std::os::raw::c_void, _: std::os::raw::c_int, _: std::os::raw::c_ulong)
     -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn strlen(_: *const std::os::raw::c_char) -> std::os::raw::c_ulong;
    #[no_mangle]
    fn memmove(_: *mut std::os::raw::c_void, _: *const std::os::raw::c_void, _: std::os::raw::c_ulong)
     -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn memcpy(_: *mut std::os::raw::c_void, _: *const std::os::raw::c_void, _: std::os::raw::c_ulong)
     -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn xmlDictFree(dict: xmlDictPtr);
    /*
 * Lookup of entry in the dictionary.
 */
    #[no_mangle]
    fn xmlDictLookup(dict: xmlDictPtr, name: *const xmlChar, len: std::os::raw::c_int)
     -> *const xmlChar;
    #[no_mangle]
    fn xmlDictOwns(dict: xmlDictPtr, str: *const xmlChar) -> std::os::raw::c_int;
    /*
 * Use the following function to reset the two global variables
 * xmlGenericError and xmlGenericErrorContext.
 */
    /*
 * Default message routines used by SAX and Valid context for error
 * and warning reporting.
 */
    /*
 * Extended error information routines
 */
    /*
 * Internal callback reporting routine
 */
    #[no_mangle]
    fn __xmlSimpleError(domain: std::os::raw::c_int, code: std::os::raw::c_int,
                        node: xmlNodePtr, msg: *const std::os::raw::c_char,
                        extra: *const std::os::raw::c_char);
    #[no_mangle]
    static mut xmlMallocAtomic: xmlMallocFunc;
    #[no_mangle]
    static mut xmlFree: xmlFreeFunc;
    #[no_mangle]
    fn __xmlBufferAllocScheme() -> *mut xmlBufferAllocationScheme;
    #[no_mangle]
    fn __xmlDefaultBufferSize() -> *mut std::os::raw::c_int;
    #[no_mangle]
    static mut xmlMalloc: xmlMallocFunc;
    #[no_mangle]
    static mut xmlRealloc: xmlReallocFunc;
    #[no_mangle]
    fn __xmlRegisterNodeDefaultValue() -> *mut xmlRegisterNodeFunc;
    /*
 * External functions:
 */
    /* LIBXML_LEGACY_ENABLED */
    /* LIBXML_LEGACY_ENABLED */
    /* LIBXML_TREE_ENABLED */
    #[no_mangle]
    fn xmlFreeEntitiesTable(table: xmlEntitiesTablePtr);
    /*
 * ALL IDs attributes are stored in a table.
 * There is one table per document.
 */
    /*
 * ALL Refs attributes are stored in a table.
 * There is one table per document.
 */
    /* Notation */
    /* LIBXML_TREE_ENABLED */
    /* LIBXML_OUTPUT_ENABLED */
    /* Element Content */
/* the non Doc version are being deprecated */
    /* the new versions with doc argument */
    /* DEPRECATED */
    /* LIBXML_OUTPUT_ENABLED */
    /* DEPRECATED */
    /* Element */
    /* LIBXML_TREE_ENABLED */
    /* LIBXML_OUTPUT_ENABLED */
    /* Enumeration */
    /* LIBXML_TREE_ENABLED */
    /* Attribute */
    /* LIBXML_TREE_ENABLED */
    #[no_mangle]
    fn xmlFreeAttributeTable(table: xmlAttributeTablePtr);
    #[no_mangle]
    fn xmlFreeElementTable(table: xmlElementTablePtr);
    #[no_mangle]
    fn xmlFreeNotationTable(table: xmlNotationTablePtr);
    #[no_mangle]
    fn __xmlDeregisterNodeDefaultValue() -> *mut xmlDeregisterNodeFunc;
    /* *
 * xmlHashCopier:
 * @payload:  the data in the hash
 * @name:  the name associated
 *
 * Callback to copy data from a hash.
 *
 * Returns a copy of the data or NULL in case of error.
 */
    /* *
 * xmlHashScanner:
 * @payload:  the data in the hash
 * @data:  extra scannner data
 * @name:  the name associated
 *
 * Callback when scanning data in a hash with the simple scanner.
 */
    /* *
 * xmlHashScannerFull:
 * @payload:  the data in the hash
 * @data:  extra scannner data
 * @name:  the name associated
 * @name2:  the second name associated
 * @name3:  the third name associated
 *
 * Callback when scanning data in a hash with the full scanner.
 */
    /*
 * Constructor and destructor.
 */
    /*
 * Add a new entry to the hash table.
 */
    /*
 * Remove an entry from the hash table.
 */
    #[no_mangle]
    fn xmlHashRemoveEntry(table: xmlHashTablePtr, name: *const xmlChar,
                          f: xmlHashDeallocator) -> std::os::raw::c_int;
    /*
 * Retrieve the userdata.
 */
    #[no_mangle]
    fn xmlHashLookup(table: xmlHashTablePtr, name: *const xmlChar)
     -> *mut std::os::raw::c_void;
    /* LIBXML_OUTPUT_ENABLED */
    /* IDs */
    /* IDREFs */
    #[no_mangle]
    fn xmlFreeRefTable(table: xmlRefTablePtr);
    #[no_mangle]
    fn xmlFreeIDTable(table: xmlIDTablePtr);
    #[no_mangle]
    fn xmlRemoveID(doc: xmlDocPtr, attr: xmlAttrPtr) -> std::os::raw::c_int;
    /* *
 * The public function calls related to validity checking.
 */
    /* Allocate/Release Validation Contexts */
    /* LIBXML_VALID_ENABLED */
    /* LIBXML_VALID_ENABLED or LIBXML_SCHEMAS_ENABLED */
    #[no_mangle]
    fn xmlGetDtdQAttrDesc(dtd: xmlDtdPtr, elem: *const xmlChar,
                          name: *const xmlChar, prefix: *const xmlChar)
     -> xmlAttributePtr;
    #[no_mangle]
    fn xmlGetDocEntity(doc: *const xmlDoc, name: *const xmlChar)
     -> xmlEntityPtr;
    #[no_mangle]
    fn xmlAddID(ctxt: xmlValidCtxtPtr, doc: xmlDocPtr, value: *const xmlChar,
                attr: xmlAttrPtr) -> xmlIDPtr;
    #[no_mangle]
    fn xmlIsID(doc: xmlDocPtr, elem: xmlNodePtr, attr: xmlAttrPtr)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlEncodeEntitiesReentrant(doc: xmlDocPtr, input: *const xmlChar)
     -> *mut xmlChar;
    #[no_mangle]
    fn xmlGetDtdQElementDesc(dtd: xmlDtdPtr, name: *const xmlChar,
                             prefix: *const xmlChar) -> xmlElementPtr;
    #[no_mangle]
    fn xmlCopyEntitiesTable(table: xmlEntitiesTablePtr)
     -> xmlEntitiesTablePtr;
    #[no_mangle]
    fn xmlCopyAttributeTable(table: xmlAttributeTablePtr)
     -> xmlAttributeTablePtr;
    #[no_mangle]
    fn xmlCopyElementTable(table: xmlElementTablePtr) -> xmlElementTablePtr;
    #[no_mangle]
    fn xmlCopyNotationTable(table: xmlNotationTablePtr)
     -> xmlNotationTablePtr;
    #[no_mangle]
    static mut xmlMemStrdup: xmlStrdupFunc;
    #[no_mangle]
    fn xmlGetDtdAttrDesc(dtd: xmlDtdPtr, elem: *const xmlChar,
                         name: *const xmlChar) -> xmlAttributePtr;
    #[no_mangle]
    fn xmlEncodeSpecialChars(doc: *const xmlDoc, input: *const xmlChar)
     -> *mut xmlChar;
    #[no_mangle]
    fn xmlBuildURI(URI: *const xmlChar, base: *const xmlChar) -> *mut xmlChar;
    #[no_mangle]
    fn xmlPathToURI(path: *const xmlChar) -> *mut xmlChar;
    #[no_mangle]
    static xmlIsCombiningGroup: xmlChRangeGroup;
    #[no_mangle]
    static xmlIsBaseCharGroup: xmlChRangeGroup;
    #[no_mangle]
    fn xmlCopyCharMultiByte(out: *mut xmlChar, val: std::os::raw::c_int)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlCharInRange(val: std::os::raw::c_uint, group: *const xmlChRangeGroup)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlStringCurrentChar(ctxt: xmlParserCtxtPtr, cur: *const xmlChar,
                            len: *mut std::os::raw::c_int) -> std::os::raw::c_int;
    #[no_mangle]
    static xmlIsExtenderGroup: xmlChRangeGroup;
    #[no_mangle]
    static xmlIsDigitGroup: xmlChRangeGroup;
    /*
 * Summary: Internal Interfaces for memory buffers in libxml2
 * Description: this module describes most of the new xmlBuf buffer
 *              entry points, those are private routines, with a
 *              few exceptions exported in tree.h. This was added
 *              in 2.9.0.
 *
 * Copy: See Copyright for the status of this software.
 *
 * Author: Daniel Veillard
 */
    #[no_mangle]
    fn xmlBufCreate() -> xmlBufPtr;
    #[no_mangle]
    fn xmlBufCreateSize(size: size_t) -> xmlBufPtr;
    #[no_mangle]
    fn xmlBufSetAllocationScheme(buf: xmlBufPtr,
                                 scheme: xmlBufferAllocationScheme)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlBufFree(buf: xmlBufPtr);
    #[no_mangle]
    fn xmlBufAdd(buf: xmlBufPtr, str: *const xmlChar, len: std::os::raw::c_int)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlBufCat(buf: xmlBufPtr, str: *const xmlChar) -> std::os::raw::c_int;
    /* size_t xmlBufUse(const xmlBufPtr buf); */
    #[no_mangle]
    fn xmlBufIsEmpty(buf: xmlBufPtr) -> std::os::raw::c_int;
    /* const xmlChar * xmlBufContent(const xmlBuf *buf); */
/* const xmlChar * xmlBufEnd(xmlBufPtr buf); */
    #[no_mangle]
    fn xmlBufDetach(buf: xmlBufPtr) -> *mut xmlChar;
    #[no_mangle]
    fn xmlBufFromBuffer(buffer: xmlBufferPtr) -> xmlBufPtr;
    #[no_mangle]
    fn xmlBufBackToBuffer(buf: xmlBufPtr) -> xmlBufferPtr;
    #[no_mangle]
    fn xmlEncodeAttributeEntities(doc: xmlDocPtr, input: *const xmlChar)
     -> *mut xmlChar;
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
pub type ptrdiff_t = std::os::raw::c_long;
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
pub type xmlMallocFunc
    =
    Option<unsafe extern "C" fn(_: size_t) -> *mut std::os::raw::c_void>;
pub type xmlReallocFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: size_t)
               -> *mut std::os::raw::c_void>;
pub type xmlStrdupFunc
    =
    Option<unsafe extern "C" fn(_: *const std::os::raw::c_char) -> *mut std::os::raw::c_char>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlParserInputBuffer {
    pub context: *mut std::os::raw::c_void,
    pub readcallback: xmlInputReadCallback,
    pub closecallback: xmlInputCloseCallback,
    pub encoder: xmlCharEncodingHandlerPtr,
    pub buffer: xmlBufPtr,
    pub raw: xmlBufPtr,
    pub compressed: std::os::raw::c_int,
    pub error: std::os::raw::c_int,
    pub rawconsumed: std::os::raw::c_ulong,
}
pub type xmlBufPtr = *mut xmlBuf;
pub type xmlBuf = _xmlBuf;
/*
 * Block defining the handlers for non UTF-8 encodings.
 * If iconv is supported, there are two extra fields.
 */
pub type xmlCharEncodingHandlerPtr = *mut xmlCharEncodingHandler;
pub type xmlCharEncodingHandler = _xmlCharEncodingHandler;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlCharEncodingHandler {
    pub name: *mut std::os::raw::c_char,
    pub input: xmlCharEncodingInputFunc,
    pub output: xmlCharEncodingOutputFunc,
    pub iconv_in: iconv_t,
    pub iconv_out: iconv_t,
}
pub type iconv_t = *mut std::os::raw::c_void;
/*
 * Summary: interface for the encoding conversion functions
 * Description: interface for the encoding conversion functions needed for
 *              XML basic encoding and iconv() support.
 *
 * Related specs are
 * rfc2044        (UTF-8 and UTF-16) F. Yergeau Alis Technologies
 * [ISO-10646]    UTF-8 and UTF-16 in Annexes
 * [ISO-8859-1]   ISO Latin-1 characters codes.
 * [UNICODE]      The Unicode Consortium, "The Unicode Standard --
 *                Worldwide Character Encoding -- Version 1.0", Addison-
 *                Wesley, Volume 1, 1991, Volume 2, 1992.  UTF-8 is
 *                described in Unicode Technical Report #4.
 * [US-ASCII]     Coded Character Set--7-bit American Standard Code for
 *                Information Interchange, ANSI X3.4-1986.
 *
 * Copy: See Copyright for the status of this software.
 *
 * Author: Daniel Veillard
 */
/*
 * xmlCharEncoding:
 *
 * Predefined values for some standard encodings.
 * Libxml does not do beforehand translation on UTF8 and ISOLatinX.
 * It also supports ASCII, ISO-8859-1, and UTF16 (LE and BE) by default.
 *
 * Anything else would have to be translated to UTF8 before being
 * given to the parser itself. The BOM for UTF16 and the encoding
 * declaration are looked at and a converter is looked for at that
 * point. If not found the parser stops here as asked by the XML REC. A
 * converter can be registered by the user using xmlRegisterCharEncodingHandler
 * but the current form doesn't allow stateful transcoding (a serious
 * problem agreed !). If iconv has been found it will be used
 * automatically and allow stateful transcoding, the simplest is then
 * to be sure to enable iconv and to provide iconv libs for the encoding
 * support needed.
 *
 * Note that the generic "UTF-16" is not a predefined value.  Instead, only
 * the specific UTF-16LE and UTF-16BE are present.
 */
/* No char encoding detected */
/* No char encoding detected */
/* UTF-8 */
/* UTF-16 little endian */
/* UTF-16 big endian */
/* UCS-4 little endian */
/* UCS-4 big endian */
/* EBCDIC uh! */
/* UCS-4 unusual ordering */
/* UCS-4 unusual ordering */
/* UCS-2 */
/* ISO-8859-1 ISO Latin 1 */
/* ISO-8859-2 ISO Latin 2 */
/* ISO-8859-3 */
/* ISO-8859-4 */
/* ISO-8859-5 */
/* ISO-8859-6 */
/* ISO-8859-7 */
/* ISO-8859-8 */
/* ISO-8859-9 */
/* ISO-2022-JP */
/* Shift_JIS */
/* EUC-JP */
/* pure ASCII */
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
pub type xmlCharEncodingInputFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_uchar, _: *mut std::os::raw::c_int,
                                _: *const std::os::raw::c_uchar, _: *mut std::os::raw::c_int)
               -> std::os::raw::c_int>;
/*
 * Summary: interface for the I/O interfaces used by the parser
 * Description: interface for the I/O interfaces used by the parser
 *
 * Copy: See Copyright for the status of this software.
 *
 * Author: Daniel Veillard
 */
/*
 * Those are the functions and datatypes for the parser input
 * I/O structures.
 */
/* *
 * xmlInputMatchCallback:
 * @filename: the filename or URI
 *
 * Callback used in the I/O Input API to detect if the current handler
 * can provide input fonctionnalities for this resource.
 *
 * Returns 1 if yes and 0 if another Input module should be used
 */
/* *
 * xmlInputOpenCallback:
 * @filename: the filename or URI
 *
 * Callback used in the I/O Input API to open the resource
 *
 * Returns an Input context or NULL in case or error
 */
/* *
 * xmlInputReadCallback:
 * @context:  an Input context
 * @buffer:  the buffer to store data read
 * @len:  the length of the buffer in bytes
 *
 * Callback used in the I/O Input API to read the resource
 *
 * Returns the number of bytes read or -1 in case of error
 */
/* *
 * xmlInputCloseCallback:
 * @context:  an Input context
 *
 * Callback used in the I/O Input API to close the resource
 *
 * Returns 0 or -1 in case of error
 */
pub type xmlInputCloseCallback
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void) -> std::os::raw::c_int>;
pub type xmlInputReadCallback
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *mut std::os::raw::c_char,
                                _: std::os::raw::c_int) -> std::os::raw::c_int>;
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
pub type xmlParserInputBuffer = _xmlParserInputBuffer;
pub type xmlParserInputBufferPtr = *mut xmlParserInputBuffer;
/*
 * Summary: the core parser module
 * Description: Interfaces, constants and types related to the XML parser
 *
 * Copy: See Copyright for the status of this software.
 *
 * Author: Daniel Veillard
 */
/* *
 * XML_DEFAULT_VERSION:
 *
 * The default version of XML used: 1.0
 */
/* *
 * xmlParserInput:
 *
 * An xmlParserInput is an input flow for the XML processor.
 * Each entity parsed is associated an xmlParserInput (except the
 * few predefined ones). This is the case both for internal entities
 * - in which case the flow is already completely in memory - or
 * external entities - in which case we use the buf structure for
 * progressive reading and I18N conversions to the internal UTF-8 format.
 */
/* *
 * xmlParserInputDeallocate:
 * @str:  the string to deallocate
 *
 * Callback for freeing some parser input allocations.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlParserInput {
    pub buf: xmlParserInputBufferPtr,
    pub filename: *const std::os::raw::c_char,
    pub directory: *const std::os::raw::c_char,
    pub base: *const xmlChar,
    pub cur: *const xmlChar,
    pub end: *const xmlChar,
    pub length: std::os::raw::c_int,
    pub line: std::os::raw::c_int,
    pub col: std::os::raw::c_int,
    pub consumed: std::os::raw::c_ulong,
    pub free: xmlParserInputDeallocate,
    pub encoding: *const xmlChar,
    pub version: *const xmlChar,
    pub standalone: std::os::raw::c_int,
    pub id: std::os::raw::c_int,
}
pub type xmlParserInputDeallocate
    =
    Option<unsafe extern "C" fn(_: *mut xmlChar) -> ()>;
/* parser.h */
pub type xmlParserInput = _xmlParserInput;
pub type xmlParserInputPtr = *mut xmlParserInput;
/* an unique identifier for the entity */
/* *
 * xmlParserNodeInfo:
 *
 * The parser can be asked to collect Node informations, i.e. at what
 * place in the file they were detected.
 * NOTE: This is off by default and not very well tested.
 */
/* Position & line # that text that created the node begins & ends on */
/* *
 * xmlParserInputState:
 *
 * The parser is now working also as a state based parser.
 * The recursive one use the state info for entities processing.
 */
/* nothing is to be parsed */
/* nothing has been parsed */
/* Misc* before int subset */
/* Within a processing instruction */
/* within some DTD content */
/* Misc* after internal subset */
/* within a comment */
/* within a start tag */
/* within the content */
/* within a CDATA section */
/* within a closing tag */
/* within an entity declaration */
/* within an entity value in a decl */
/* within an attribute value */
/* within a SYSTEM value */
/* the Misc* after the last end tag */
/* within an IGNORED section */
/* within a PUBLIC value */
/* *
 * XML_DETECT_IDS:
 *
 * Bit in the loadsubset context field to tell to do ID/REFs lookups.
 * Use it to initialize xmlLoadExtDtdDefaultValue.
 */
/* *
 * XML_COMPLETE_ATTRS:
 *
 * Bit in the loadsubset context field to tell to do complete the
 * elements attributes lists with the ones defaulted from the DTDs.
 * Use it to initialize xmlLoadExtDtdDefaultValue.
 */
/* *
 * XML_SKIP_IDS:
 *
 * Bit in the loadsubset context field to tell to not do ID/REFs registration.
 * Used to initialize xmlLoadExtDtdDefaultValue in some special cases.
 */
/* *
 * xmlParserMode:
 *
 * A parser can operate in various modes
 */
/* *
 * xmlParserCtxt:
 *
 * The parser context.
 * NOTE This doesn't completely define the parser state, the (current ?)
 *      design of the parser uses recursive function calls since this allow
 *      and easy mapping from the production rules of the specification
 *      to the actual code. The drawback is that the actual function call
 *      also reflect the parser state. However most of the parsing routines
 *      takes as the only argument the parser context pointer, so migrating
 *      to a state based parser for progressive parsing shouldn't be too hard.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlParserCtxt {
    pub sax: *mut _xmlSAXHandler,
    pub userData: *mut std::os::raw::c_void,
    pub myDoc: xmlDocPtr,
    pub wellFormed: std::os::raw::c_int,
    pub replaceEntities: std::os::raw::c_int,
    pub version: *const xmlChar,
    pub encoding: *const xmlChar,
    pub standalone: std::os::raw::c_int,
    pub html: std::os::raw::c_int,
    pub input: xmlParserInputPtr,
    pub inputNr: std::os::raw::c_int,
    pub inputMax: std::os::raw::c_int,
    pub inputTab: *mut xmlParserInputPtr,
    pub node: xmlNodePtr,
    pub nodeNr: std::os::raw::c_int,
    pub nodeMax: std::os::raw::c_int,
    pub nodeTab: *mut xmlNodePtr,
    pub record_info: std::os::raw::c_int,
    pub node_seq: xmlParserNodeInfoSeq,
    pub errNo: std::os::raw::c_int,
    pub hasExternalSubset: std::os::raw::c_int,
    pub hasPErefs: std::os::raw::c_int,
    pub external: std::os::raw::c_int,
    pub valid: std::os::raw::c_int,
    pub validate: std::os::raw::c_int,
    pub vctxt: xmlValidCtxt,
    pub instate: xmlParserInputState,
    pub token: std::os::raw::c_int,
    pub directory: *mut std::os::raw::c_char,
    pub name: *const xmlChar,
    pub nameNr: std::os::raw::c_int,
    pub nameMax: std::os::raw::c_int,
    pub nameTab: *mut *const xmlChar,
    pub nbChars: std::os::raw::c_long,
    pub checkIndex: std::os::raw::c_long,
    pub keepBlanks: std::os::raw::c_int,
    pub disableSAX: std::os::raw::c_int,
    pub inSubset: std::os::raw::c_int,
    pub intSubName: *const xmlChar,
    pub extSubURI: *mut xmlChar,
    pub extSubSystem: *mut xmlChar,
    pub space: *mut std::os::raw::c_int,
    pub spaceNr: std::os::raw::c_int,
    pub spaceMax: std::os::raw::c_int,
    pub spaceTab: *mut std::os::raw::c_int,
    pub depth: std::os::raw::c_int,
    pub entity: xmlParserInputPtr,
    pub charset: std::os::raw::c_int,
    pub nodelen: std::os::raw::c_int,
    pub nodemem: std::os::raw::c_int,
    pub pedantic: std::os::raw::c_int,
    pub _private: *mut std::os::raw::c_void,
    pub loadsubset: std::os::raw::c_int,
    pub linenumbers: std::os::raw::c_int,
    pub catalogs: *mut std::os::raw::c_void,
    pub recovery: std::os::raw::c_int,
    pub progressive: std::os::raw::c_int,
    pub dict: xmlDictPtr,
    pub atts: *mut *const xmlChar,
    pub maxatts: std::os::raw::c_int,
    pub docdict: std::os::raw::c_int,
    pub str_xml: *const xmlChar,
    pub str_xmlns: *const xmlChar,
    pub str_xml_ns: *const xmlChar,
    pub sax2: std::os::raw::c_int,
    pub nsNr: std::os::raw::c_int,
    pub nsMax: std::os::raw::c_int,
    pub nsTab: *mut *const xmlChar,
    pub attallocs: *mut std::os::raw::c_int,
    pub pushTab: *mut *mut std::os::raw::c_void,
    pub attsDefault: xmlHashTablePtr,
    pub attsSpecial: xmlHashTablePtr,
    pub nsWellFormed: std::os::raw::c_int,
    pub options: std::os::raw::c_int,
    pub dictNames: std::os::raw::c_int,
    pub freeElemsNr: std::os::raw::c_int,
    pub freeElems: xmlNodePtr,
    pub freeAttrsNr: std::os::raw::c_int,
    pub freeAttrs: xmlAttrPtr,
    pub lastError: xmlError,
    pub parseMode: xmlParserMode,
    pub nbentities: std::os::raw::c_ulong,
    pub sizeentities: std::os::raw::c_ulong,
    pub nodeInfo: *mut xmlParserNodeInfo,
    pub nodeInfoNr: std::os::raw::c_int,
    pub nodeInfoMax: std::os::raw::c_int,
    pub nodeInfoTab: *mut xmlParserNodeInfo,
    pub input_id: std::os::raw::c_int,
    pub sizeentcopy: std::os::raw::c_ulong,
}
pub type xmlParserNodeInfo = _xmlParserNodeInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlParserNodeInfo {
    pub node: *const _xmlNode,
    pub begin_pos: std::os::raw::c_ulong,
    pub begin_line: std::os::raw::c_ulong,
    pub end_pos: std::os::raw::c_ulong,
    pub end_line: std::os::raw::c_ulong,
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
pub type xmlParserMode = std::os::raw::c_uint;
pub const XML_PARSE_READER: xmlParserMode = 5;
pub const XML_PARSE_PUSH_SAX: xmlParserMode = 4;
pub const XML_PARSE_PUSH_DOM: xmlParserMode = 3;
pub const XML_PARSE_SAX: xmlParserMode = 2;
pub const XML_PARSE_DOM: xmlParserMode = 1;
pub const XML_PARSE_UNKNOWN: xmlParserMode = 0;
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
pub type xmlParserInputState = std::os::raw::c_int;
pub const XML_PARSER_PUBLIC_LITERAL: xmlParserInputState = 16;
pub const XML_PARSER_IGNORE: xmlParserInputState = 15;
pub const XML_PARSER_EPILOG: xmlParserInputState = 14;
pub const XML_PARSER_SYSTEM_LITERAL: xmlParserInputState = 13;
pub const XML_PARSER_ATTRIBUTE_VALUE: xmlParserInputState = 12;
pub const XML_PARSER_ENTITY_VALUE: xmlParserInputState = 11;
pub const XML_PARSER_ENTITY_DECL: xmlParserInputState = 10;
pub const XML_PARSER_END_TAG: xmlParserInputState = 9;
pub const XML_PARSER_CDATA_SECTION: xmlParserInputState = 8;
pub const XML_PARSER_CONTENT: xmlParserInputState = 7;
pub const XML_PARSER_START_TAG: xmlParserInputState = 6;
pub const XML_PARSER_COMMENT: xmlParserInputState = 5;
pub const XML_PARSER_PROLOG: xmlParserInputState = 4;
pub const XML_PARSER_DTD: xmlParserInputState = 3;
pub const XML_PARSER_PI: xmlParserInputState = 2;
pub const XML_PARSER_MISC: xmlParserInputState = 1;
pub const XML_PARSER_START: xmlParserInputState = 0;
pub const XML_PARSER_EOF: xmlParserInputState = -1;
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
pub type xmlParserNodeInfoSeq = _xmlParserNodeInfoSeq;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlParserNodeInfoSeq {
    pub maximum: std::os::raw::c_ulong,
    pub length: std::os::raw::c_ulong,
    pub buffer: *mut xmlParserNodeInfo,
}
/* volume of entity copy */
/* *
 * xmlSAXLocator:
 *
 * A SAX Locator.
 */
/* *
 * xmlSAXHandler:
 *
 * A SAX handler is bunch of callbacks called by the parser when processing
 * of the input generate data or structure informations.
 */
/* *
 * resolveEntitySAXFunc:
 * @ctx:  the user data (XML parser context)
 * @publicId: The public ID of the entity
 * @systemId: The system ID of the entity
 *
 * Callback:
 * The entity loader, to control the loading of external entities,
 * the application can either:
 *    - override this resolveEntity() callback in the SAX block
 *    - or better use the xmlSetExternalEntityLoader() function to
 *      set up it's own entity resolution routine
 *
 * Returns the xmlParserInputPtr if inlined or NULL for DOM behaviour.
 */
/* *
 * internalSubsetSAXFunc:
 * @ctx:  the user data (XML parser context)
 * @name:  the root element name
 * @ExternalID:  the external ID
 * @SystemID:  the SYSTEM ID (e.g. filename or URL)
 *
 * Callback on internal subset declaration.
 */
/* *
 * externalSubsetSAXFunc:
 * @ctx:  the user data (XML parser context)
 * @name:  the root element name
 * @ExternalID:  the external ID
 * @SystemID:  the SYSTEM ID (e.g. filename or URL)
 *
 * Callback on external subset declaration.
 */
/* *
 * getEntitySAXFunc:
 * @ctx:  the user data (XML parser context)
 * @name: The entity name
 *
 * Get an entity by name.
 *
 * Returns the xmlEntityPtr if found.
 */
/* *
 * getParameterEntitySAXFunc:
 * @ctx:  the user data (XML parser context)
 * @name: The entity name
 *
 * Get a parameter entity by name.
 *
 * Returns the xmlEntityPtr if found.
 */
/* *
 * entityDeclSAXFunc:
 * @ctx:  the user data (XML parser context)
 * @name:  the entity name
 * @type:  the entity type
 * @publicId: The public ID of the entity
 * @systemId: The system ID of the entity
 * @content: the entity value (without processing).
 *
 * An entity definition has been parsed.
 */
/* *
 * notationDeclSAXFunc:
 * @ctx:  the user data (XML parser context)
 * @name: The name of the notation
 * @publicId: The public ID of the entity
 * @systemId: The system ID of the entity
 *
 * What to do when a notation declaration has been parsed.
 */
/* *
 * attributeDeclSAXFunc:
 * @ctx:  the user data (XML parser context)
 * @elem:  the name of the element
 * @fullname:  the attribute name
 * @type:  the attribute type
 * @def:  the type of default value
 * @defaultValue: the attribute default value
 * @tree:  the tree of enumerated value set
 *
 * An attribute definition has been parsed.
 */
/* *
 * elementDeclSAXFunc:
 * @ctx:  the user data (XML parser context)
 * @name:  the element name
 * @type:  the element type
 * @content: the element value tree
 *
 * An element definition has been parsed.
 */
/* *
 * unparsedEntityDeclSAXFunc:
 * @ctx:  the user data (XML parser context)
 * @name: The name of the entity
 * @publicId: The public ID of the entity
 * @systemId: The system ID of the entity
 * @notationName: the name of the notation
 *
 * What to do when an unparsed entity declaration is parsed.
 */
/* *
 * setDocumentLocatorSAXFunc:
 * @ctx:  the user data (XML parser context)
 * @loc: A SAX Locator
 *
 * Receive the document locator at startup, actually xmlDefaultSAXLocator.
 * Everything is available on the context, so this is useless in our case.
 */
/* *
 * startDocumentSAXFunc:
 * @ctx:  the user data (XML parser context)
 *
 * Called when the document start being processed.
 */
/* *
 * endDocumentSAXFunc:
 * @ctx:  the user data (XML parser context)
 *
 * Called when the document end has been detected.
 */
/* *
 * startElementSAXFunc:
 * @ctx:  the user data (XML parser context)
 * @name:  The element name, including namespace prefix
 * @atts:  An array of name/value attributes pairs, NULL terminated
 *
 * Called when an opening tag has been processed.
 */
/* *
 * endElementSAXFunc:
 * @ctx:  the user data (XML parser context)
 * @name:  The element name
 *
 * Called when the end of an element has been detected.
 */
/* *
 * attributeSAXFunc:
 * @ctx:  the user data (XML parser context)
 * @name:  The attribute name, including namespace prefix
 * @value:  The attribute value
 *
 * Handle an attribute that has been read by the parser.
 * The default handling is to convert the attribute into an
 * DOM subtree and past it in a new xmlAttr element added to
 * the element.
 */
/* *
 * referenceSAXFunc:
 * @ctx:  the user data (XML parser context)
 * @name:  The entity name
 *
 * Called when an entity reference is detected.
 */
/* *
 * charactersSAXFunc:
 * @ctx:  the user data (XML parser context)
 * @ch:  a xmlChar string
 * @len: the number of xmlChar
 *
 * Receiving some chars from the parser.
 */
/* *
 * ignorableWhitespaceSAXFunc:
 * @ctx:  the user data (XML parser context)
 * @ch:  a xmlChar string
 * @len: the number of xmlChar
 *
 * Receiving some ignorable whitespaces from the parser.
 * UNUSED: by default the DOM building will use characters.
 */
/* *
 * processingInstructionSAXFunc:
 * @ctx:  the user data (XML parser context)
 * @target:  the target name
 * @data: the PI data's
 *
 * A processing instruction has been parsed.
 */
/* *
 * commentSAXFunc:
 * @ctx:  the user data (XML parser context)
 * @value:  the comment content
 *
 * A comment has been parsed.
 */
/* *
 * cdataBlockSAXFunc:
 * @ctx:  the user data (XML parser context)
 * @value:  The pcdata content
 * @len:  the block length
 *
 * Called when a pcdata block has been parsed.
 */
/* *
 * warningSAXFunc:
 * @ctx:  an XML parser context
 * @msg:  the message to display/transmit
 * @...:  extra parameters for the message display
 *
 * Display and format a warning messages, callback.
 */
/* *
 * errorSAXFunc:
 * @ctx:  an XML parser context
 * @msg:  the message to display/transmit
 * @...:  extra parameters for the message display
 *
 * Display and format an error messages, callback.
 */
/* *
 * fatalErrorSAXFunc:
 * @ctx:  an XML parser context
 * @msg:  the message to display/transmit
 * @...:  extra parameters for the message display
 *
 * Display and format fatal error messages, callback.
 * Note: so far fatalError() SAX callbacks are not used, error()
 *       get all the callbacks for errors.
 */
/* *
 * isStandaloneSAXFunc:
 * @ctx:  the user data (XML parser context)
 *
 * Is this document tagged standalone?
 *
 * Returns 1 if true
 */
/* *
 * hasInternalSubsetSAXFunc:
 * @ctx:  the user data (XML parser context)
 *
 * Does this document has an internal subset.
 *
 * Returns 1 if true
 */
/* *
 * hasExternalSubsetSAXFunc:
 * @ctx:  the user data (XML parser context)
 *
 * Does this document has an external subset?
 *
 * Returns 1 if true
 */
/* ***********************************************************************
 *									*
 *			The SAX version 2 API extensions		*
 *									*
 ************************************************************************/
/* *
 * XML_SAX2_MAGIC:
 *
 * Special constant found in SAX2 blocks initialized fields
 */
/* *
 * startElementNsSAX2Func:
 * @ctx:  the user data (XML parser context)
 * @localname:  the local name of the element
 * @prefix:  the element namespace prefix if available
 * @URI:  the element namespace name if available
 * @nb_namespaces:  number of namespace definitions on that node
 * @namespaces:  pointer to the array of prefix/URI pairs namespace definitions
 * @nb_attributes:  the number of attributes on that node
 * @nb_defaulted:  the number of defaulted attributes. The defaulted
 *                  ones are at the end of the array
 * @attributes:  pointer to the array of (localname/prefix/URI/value/end)
 *               attribute values.
 *
 * SAX2 callback when an element start has been detected by the parser.
 * It provides the namespace informations for the element, as well as
 * the new namespace declarations on the element.
 */
/* *
 * endElementNsSAX2Func:
 * @ctx:  the user data (XML parser context)
 * @localname:  the local name of the element
 * @prefix:  the element namespace prefix if available
 * @URI:  the element namespace name if available
 *
 * SAX2 callback when an element end has been detected by the parser.
 * It provides the namespace informations for the element.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlSAXHandler {
    pub internalSubset: internalSubsetSAXFunc,
    pub isStandalone: isStandaloneSAXFunc,
    pub hasInternalSubset: hasInternalSubsetSAXFunc,
    pub hasExternalSubset: hasExternalSubsetSAXFunc,
    pub resolveEntity: resolveEntitySAXFunc,
    pub getEntity: getEntitySAXFunc,
    pub entityDecl: entityDeclSAXFunc,
    pub notationDecl: notationDeclSAXFunc,
    pub attributeDecl: attributeDeclSAXFunc,
    pub elementDecl: elementDeclSAXFunc,
    pub unparsedEntityDecl: unparsedEntityDeclSAXFunc,
    pub setDocumentLocator: setDocumentLocatorSAXFunc,
    pub startDocument: startDocumentSAXFunc,
    pub endDocument: endDocumentSAXFunc,
    pub startElement: startElementSAXFunc,
    pub endElement: endElementSAXFunc,
    pub reference: referenceSAXFunc,
    pub characters: charactersSAXFunc,
    pub ignorableWhitespace: ignorableWhitespaceSAXFunc,
    pub processingInstruction: processingInstructionSAXFunc,
    pub comment: commentSAXFunc,
    pub warning: warningSAXFunc,
    pub error: errorSAXFunc,
    pub fatalError: fatalErrorSAXFunc,
    pub getParameterEntity: getParameterEntitySAXFunc,
    pub cdataBlock: cdataBlockSAXFunc,
    pub externalSubset: externalSubsetSAXFunc,
    pub initialized: std::os::raw::c_uint,
    pub _private: *mut std::os::raw::c_void,
    pub startElementNs: startElementNsSAX2Func,
    pub endElementNs: endElementNsSAX2Func,
    pub serror: xmlStructuredErrorFunc,
}
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
pub type endElementNsSAX2Func
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                _: *const xmlChar, _: *const xmlChar) -> ()>;
pub type startElementNsSAX2Func
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                _: *const xmlChar, _: *const xmlChar,
                                _: std::os::raw::c_int, _: *mut *const xmlChar,
                                _: std::os::raw::c_int, _: std::os::raw::c_int,
                                _: *mut *const xmlChar) -> ()>;
pub type externalSubsetSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                _: *const xmlChar, _: *const xmlChar) -> ()>;
pub type cdataBlockSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                _: std::os::raw::c_int) -> ()>;
pub type getParameterEntitySAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar)
               -> xmlEntityPtr>;
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
pub type fatalErrorSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const std::os::raw::c_char,
                                _: ...) -> ()>;
pub type errorSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const std::os::raw::c_char,
                                _: ...) -> ()>;
pub type warningSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const std::os::raw::c_char,
                                _: ...) -> ()>;
pub type commentSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar)
               -> ()>;
pub type processingInstructionSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                _: *const xmlChar) -> ()>;
pub type ignorableWhitespaceSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                _: std::os::raw::c_int) -> ()>;
pub type charactersSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                _: std::os::raw::c_int) -> ()>;
pub type referenceSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar)
               -> ()>;
pub type endElementSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar)
               -> ()>;
pub type startElementSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                _: *mut *const xmlChar) -> ()>;
pub type endDocumentSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void) -> ()>;
pub type startDocumentSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void) -> ()>;
pub type setDocumentLocatorSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: xmlSAXLocatorPtr)
               -> ()>;
pub type xmlSAXLocatorPtr = *mut xmlSAXLocator;
pub type xmlSAXLocator = _xmlSAXLocator;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlSAXLocator {
    pub getPublicId: Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void)
                                -> *const xmlChar>,
    pub getSystemId: Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void)
                                -> *const xmlChar>,
    pub getLineNumber: Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void)
                                  -> std::os::raw::c_int>,
    pub getColumnNumber: Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void)
                                    -> std::os::raw::c_int>,
}
pub type unparsedEntityDeclSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                _: *const xmlChar, _: *const xmlChar,
                                _: *const xmlChar) -> ()>;
pub type elementDeclSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                _: std::os::raw::c_int, _: xmlElementContentPtr)
               -> ()>;
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
pub type attributeDeclSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                _: *const xmlChar, _: std::os::raw::c_int,
                                _: std::os::raw::c_int, _: *const xmlChar,
                                _: xmlEnumerationPtr) -> ()>;
pub type xmlEnumerationPtr = *mut xmlEnumeration;
pub type xmlEnumeration = _xmlEnumeration;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlEnumeration {
    pub next: *mut _xmlEnumeration,
    pub name: *const xmlChar,
}
pub type notationDeclSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                _: *const xmlChar, _: *const xmlChar) -> ()>;
pub type entityDeclSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                _: std::os::raw::c_int, _: *const xmlChar,
                                _: *const xmlChar, _: *mut xmlChar) -> ()>;
pub type getEntitySAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar)
               -> xmlEntityPtr>;
pub type resolveEntitySAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                _: *const xmlChar) -> xmlParserInputPtr>;
pub type hasExternalSubsetSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void) -> std::os::raw::c_int>;
pub type hasInternalSubsetSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void) -> std::os::raw::c_int>;
pub type isStandaloneSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void) -> std::os::raw::c_int>;
pub type internalSubsetSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                _: *const xmlChar, _: *const xmlChar) -> ()>;
pub type xmlParserCtxt = _xmlParserCtxt;
pub type xmlParserCtxtPtr = *mut xmlParserCtxt;
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
pub type C2RustUnnamed = std::os::raw::c_uint;
pub const XML_DOC_HTML: C2RustUnnamed = 128;
pub const XML_DOC_INTERNAL: C2RustUnnamed = 64;
pub const XML_DOC_USERBUILT: C2RustUnnamed = 32;
pub const XML_DOC_XINCLUDE: C2RustUnnamed = 16;
pub const XML_DOC_DTDVALID: C2RustUnnamed = 8;
pub const XML_DOC_OLD10: C2RustUnnamed = 4;
pub const XML_DOC_NSVALID: C2RustUnnamed = 2;
pub const XML_DOC_WELLFORMED: C2RustUnnamed = 1;
/* set of xmlDocProperties for this document
				   set at the end of parsing */
/* *
 * xmlDOMWrapAcquireNsFunction:
 * @ctxt:  a DOM wrapper context
 * @node:  the context node (element or attribute)
 * @nsName:  the requested namespace name
 * @nsPrefix:  the requested namespace prefix
 *
 * A function called to acquire namespaces (xmlNs) from the wrapper.
 *
 * Returns an xmlNsPtr or NULL in case of an error.
 */
/* *
 * xmlDOMWrapCtxt:
 *
 * Context for DOM wrapper-operations.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlDOMWrapCtxt {
    pub _private: *mut std::os::raw::c_void,
    pub type_0: std::os::raw::c_int,
    pub namespaceMap: *mut std::os::raw::c_void,
    pub getNsForNodeFunc: xmlDOMWrapAcquireNsFunction,
}
pub type xmlDOMWrapAcquireNsFunction
    =
    Option<unsafe extern "C" fn(_: xmlDOMWrapCtxtPtr, _: xmlNodePtr,
                                _: *const xmlChar, _: *const xmlChar)
               -> xmlNsPtr>;
pub type xmlDOMWrapCtxtPtr = *mut xmlDOMWrapCtxt;
pub type xmlDOMWrapCtxt = _xmlDOMWrapCtxt;
pub type xmlChRangeGroup = _xmlChRangeGroup;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlChRangeGroup {
    pub nbShortRange: std::os::raw::c_int,
    pub nbLongRange: std::os::raw::c_int,
    pub shortRange: *const xmlChSRange,
    pub longRange: *const xmlChLRange,
}
pub type xmlChLRange = _xmlChLRange;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlChLRange {
    pub low: std::os::raw::c_uint,
    pub high: std::os::raw::c_uint,
}
pub type xmlChSRange = _xmlChSRange;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlChSRange {
    pub low: std::os::raw::c_ushort,
    pub high: std::os::raw::c_ushort,
}
pub const XML_ERR_NO_MEMORY: C2RustUnnamed_1 = 2;
pub const XML_FROM_TREE: C2RustUnnamed_0 = 2;
pub type xmlRegisterNodeFunc
    =
    Option<unsafe extern "C" fn(_: xmlNodePtr) -> ()>;
/* was the entity content checked */
/* this is also used to count entities
					 * references done from that entity
					 * and if it contains '<' */
/*
 * All entities are stored in an hash table.
 * There is 2 separate hash tables for global and parameter entities.
 */
pub type xmlEntitiesTablePtr = *mut xmlEntitiesTable;
pub type xmlEntitiesTable = _xmlHashTable;
/* used to build the automata */
/*
 * ALL notation declarations are stored in a table.
 * There is one table per DTD.
 */
/*
 * ALL element declarations are stored in a table.
 * There is one table per DTD.
 */
/*
 * ALL attribute declarations are stored in a table.
 * There is one table per DTD.
 */
pub type xmlAttributeTablePtr = *mut xmlAttributeTable;
pub type xmlAttributeTable = _xmlHashTable;
pub type xmlElementTablePtr = *mut xmlElementTable;
pub type xmlElementTable = _xmlHashTable;
pub type xmlNotationTablePtr = *mut xmlNotationTable;
pub type xmlNotationTable = _xmlHashTable;
pub type xmlDeregisterNodeFunc
    =
    Option<unsafe extern "C" fn(_: xmlNodePtr) -> ()>;
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
pub type xmlRefTablePtr = *mut xmlRefTable;
pub type xmlRefTable = _xmlHashTable;
pub type xmlIDTablePtr = *mut xmlIDTable;
pub type xmlIDTable = _xmlHashTable;
pub const XML_CHAR_ENCODING_UTF8: C2RustUnnamed_2 = 1;
pub const XML_TREE_UNTERMINATED_ENTITY: C2RustUnnamed_1 = 1302;
pub const XML_TREE_NOT_UTF8: C2RustUnnamed_1 = 1303;
pub const XML_TREE_INVALID_DEC: C2RustUnnamed_1 = 1301;
pub const XML_TREE_INVALID_HEX: C2RustUnnamed_1 = 1300;
pub type xmlValidCtxtPtr = *mut xmlValidCtxt;
pub type xmlNsMapPtr = *mut xmlNsMap;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xmlNsMap {
    pub first: xmlNsMapItemPtr,
    pub last: xmlNsMapItemPtr,
    pub pool: xmlNsMapItemPtr,
}
pub type xmlNsMapItemPtr = *mut xmlNsMapItem;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xmlNsMapItem {
    pub next: xmlNsMapItemPtr,
    pub prev: xmlNsMapItemPtr,
    pub oldNs: xmlNsPtr,
    pub newNs: xmlNsPtr,
    pub shadowDepth: std::os::raw::c_int,
    pub depth: std::os::raw::c_int,
}
pub const XML_DOM_RECONNS_REMOVEREDUND: xmlDOMReconcileNSOptions = 1;
pub type xmlDOMReconcileNSOptions = std::os::raw::c_uint;
pub type C2RustUnnamed_0 = std::os::raw::c_uint;
pub const XML_FROM_URI: C2RustUnnamed_0 = 30;
pub const XML_FROM_BUFFER: C2RustUnnamed_0 = 29;
pub const XML_FROM_SCHEMATRONV: C2RustUnnamed_0 = 28;
pub const XML_FROM_I18N: C2RustUnnamed_0 = 27;
pub const XML_FROM_MODULE: C2RustUnnamed_0 = 26;
pub const XML_FROM_WRITER: C2RustUnnamed_0 = 25;
pub const XML_FROM_CHECK: C2RustUnnamed_0 = 24;
pub const XML_FROM_VALID: C2RustUnnamed_0 = 23;
pub const XML_FROM_XSLT: C2RustUnnamed_0 = 22;
pub const XML_FROM_C14N: C2RustUnnamed_0 = 21;
pub const XML_FROM_CATALOG: C2RustUnnamed_0 = 20;
pub const XML_FROM_RELAXNGV: C2RustUnnamed_0 = 19;
pub const XML_FROM_RELAXNGP: C2RustUnnamed_0 = 18;
pub const XML_FROM_SCHEMASV: C2RustUnnamed_0 = 17;
pub const XML_FROM_SCHEMASP: C2RustUnnamed_0 = 16;
pub const XML_FROM_DATATYPE: C2RustUnnamed_0 = 15;
pub const XML_FROM_REGEXP: C2RustUnnamed_0 = 14;
pub const XML_FROM_XPOINTER: C2RustUnnamed_0 = 13;
pub const XML_FROM_XPATH: C2RustUnnamed_0 = 12;
pub const XML_FROM_XINCLUDE: C2RustUnnamed_0 = 11;
pub const XML_FROM_HTTP: C2RustUnnamed_0 = 10;
pub const XML_FROM_FTP: C2RustUnnamed_0 = 9;
pub const XML_FROM_IO: C2RustUnnamed_0 = 8;
pub const XML_FROM_OUTPUT: C2RustUnnamed_0 = 7;
pub const XML_FROM_MEMORY: C2RustUnnamed_0 = 6;
pub const XML_FROM_HTML: C2RustUnnamed_0 = 5;
pub const XML_FROM_DTD: C2RustUnnamed_0 = 4;
pub const XML_FROM_NAMESPACE: C2RustUnnamed_0 = 3;
pub const XML_FROM_PARSER: C2RustUnnamed_0 = 1;
pub const XML_FROM_NONE: C2RustUnnamed_0 = 0;
pub type C2RustUnnamed_1 = std::os::raw::c_uint;
pub const XML_BUF_OVERFLOW: C2RustUnnamed_1 = 7000;
pub const XML_I18N_NO_OUTPUT: C2RustUnnamed_1 = 6004;
pub const XML_I18N_CONV_FAILED: C2RustUnnamed_1 = 6003;
pub const XML_I18N_EXCESS_HANDLER: C2RustUnnamed_1 = 6002;
pub const XML_I18N_NO_HANDLER: C2RustUnnamed_1 = 6001;
pub const XML_I18N_NO_NAME: C2RustUnnamed_1 = 6000;
pub const XML_CHECK_NAME_NOT_NULL: C2RustUnnamed_1 = 5037;
pub const XML_CHECK_WRONG_NAME: C2RustUnnamed_1 = 5036;
pub const XML_CHECK_OUTSIDE_DICT: C2RustUnnamed_1 = 5035;
pub const XML_CHECK_NOT_NCNAME: C2RustUnnamed_1 = 5034;
pub const XML_CHECK_NO_DICT: C2RustUnnamed_1 = 5033;
pub const XML_CHECK_NOT_UTF8: C2RustUnnamed_1 = 5032;
pub const XML_CHECK_NS_ANCESTOR: C2RustUnnamed_1 = 5031;
pub const XML_CHECK_NS_SCOPE: C2RustUnnamed_1 = 5030;
pub const XML_CHECK_WRONG_PARENT: C2RustUnnamed_1 = 5029;
pub const XML_CHECK_NO_HREF: C2RustUnnamed_1 = 5028;
pub const XML_CHECK_NOT_NS_DECL: C2RustUnnamed_1 = 5027;
pub const XML_CHECK_NOT_ENTITY_DECL: C2RustUnnamed_1 = 5026;
pub const XML_CHECK_NOT_ELEM_DECL: C2RustUnnamed_1 = 5025;
pub const XML_CHECK_NOT_ATTR_DECL: C2RustUnnamed_1 = 5024;
pub const XML_CHECK_NOT_ATTR: C2RustUnnamed_1 = 5023;
pub const XML_CHECK_NOT_DTD: C2RustUnnamed_1 = 5022;
pub const XML_CHECK_WRONG_NEXT: C2RustUnnamed_1 = 5021;
pub const XML_CHECK_NO_NEXT: C2RustUnnamed_1 = 5020;
pub const XML_CHECK_WRONG_PREV: C2RustUnnamed_1 = 5019;
pub const XML_CHECK_NO_PREV: C2RustUnnamed_1 = 5018;
pub const XML_CHECK_WRONG_DOC: C2RustUnnamed_1 = 5017;
pub const XML_CHECK_NO_ELEM: C2RustUnnamed_1 = 5016;
pub const XML_CHECK_NO_NAME: C2RustUnnamed_1 = 5015;
pub const XML_CHECK_NO_DOC: C2RustUnnamed_1 = 5014;
pub const XML_CHECK_NO_PARENT: C2RustUnnamed_1 = 5013;
pub const XML_CHECK_ENTITY_TYPE: C2RustUnnamed_1 = 5012;
pub const XML_CHECK_UNKNOWN_NODE: C2RustUnnamed_1 = 5011;
pub const XML_CHECK_FOUND_NOTATION: C2RustUnnamed_1 = 5010;
pub const XML_CHECK_FOUND_FRAGMENT: C2RustUnnamed_1 = 5009;
pub const XML_CHECK_FOUND_DOCTYPE: C2RustUnnamed_1 = 5008;
pub const XML_CHECK_FOUND_COMMENT: C2RustUnnamed_1 = 5007;
pub const XML_CHECK_FOUND_PI: C2RustUnnamed_1 = 5006;
pub const XML_CHECK_FOUND_ENTITY: C2RustUnnamed_1 = 5005;
pub const XML_CHECK_FOUND_ENTITYREF: C2RustUnnamed_1 = 5004;
pub const XML_CHECK_FOUND_CDATA: C2RustUnnamed_1 = 5003;
pub const XML_CHECK_FOUND_TEXT: C2RustUnnamed_1 = 5002;
pub const XML_CHECK_FOUND_ATTRIBUTE: C2RustUnnamed_1 = 5001;
pub const XML_CHECK_FOUND_ELEMENT: C2RustUnnamed_1 = 5000;
pub const XML_MODULE_CLOSE: C2RustUnnamed_1 = 4901;
pub const XML_MODULE_OPEN: C2RustUnnamed_1 = 4900;
pub const XML_SCHEMATRONV_REPORT: C2RustUnnamed_1 = 4001;
pub const XML_SCHEMATRONV_ASSERT: C2RustUnnamed_1 = 4000;
pub const XML_SCHEMAP_COS_ALL_LIMITED: C2RustUnnamed_1 = 3091;
pub const XML_SCHEMAP_A_PROPS_CORRECT_3: C2RustUnnamed_1 = 3090;
pub const XML_SCHEMAP_AU_PROPS_CORRECT: C2RustUnnamed_1 = 3089;
pub const XML_SCHEMAP_COS_CT_EXTENDS_1_2: C2RustUnnamed_1 = 3088;
pub const XML_SCHEMAP_AG_PROPS_CORRECT: C2RustUnnamed_1 = 3087;
pub const XML_SCHEMAP_WARN_ATTR_POINTLESS_PROH: C2RustUnnamed_1 = 3086;
pub const XML_SCHEMAP_WARN_ATTR_REDECL_PROH: C2RustUnnamed_1 = 3085;
pub const XML_SCHEMAP_WARN_UNLOCATED_SCHEMA: C2RustUnnamed_1 = 3084;
pub const XML_SCHEMAP_WARN_SKIP_SCHEMA: C2RustUnnamed_1 = 3083;
pub const XML_SCHEMAP_SRC_IMPORT: C2RustUnnamed_1 = 3082;
pub const XML_SCHEMAP_SRC_REDEFINE: C2RustUnnamed_1 = 3081;
pub const XML_SCHEMAP_C_PROPS_CORRECT: C2RustUnnamed_1 = 3080;
pub const XML_SCHEMAP_A_PROPS_CORRECT_2: C2RustUnnamed_1 = 3079;
pub const XML_SCHEMAP_AU_PROPS_CORRECT_2: C2RustUnnamed_1 = 3078;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_2_1_3: C2RustUnnamed_1 = 3077;
pub const XML_SCHEMAP_SRC_CT_1: C2RustUnnamed_1 = 3076;
pub const XML_SCHEMAP_MG_PROPS_CORRECT_2: C2RustUnnamed_1 = 3075;
pub const XML_SCHEMAP_MG_PROPS_CORRECT_1: C2RustUnnamed_1 = 3074;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_GROUP_3: C2RustUnnamed_1 = 3073;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_GROUP_2: C2RustUnnamed_1 = 3072;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_GROUP_1: C2RustUnnamed_1 = 3071;
pub const XML_SCHEMAP_NOT_DETERMINISTIC: C2RustUnnamed_1 = 3070;
pub const XML_SCHEMAP_INTERNAL: C2RustUnnamed_1 = 3069;
pub const XML_SCHEMAP_SRC_IMPORT_2_2: C2RustUnnamed_1 = 3068;
pub const XML_SCHEMAP_SRC_IMPORT_2_1: C2RustUnnamed_1 = 3067;
pub const XML_SCHEMAP_SRC_IMPORT_2: C2RustUnnamed_1 = 3066;
pub const XML_SCHEMAP_SRC_IMPORT_1_2: C2RustUnnamed_1 = 3065;
pub const XML_SCHEMAP_SRC_IMPORT_1_1: C2RustUnnamed_1 = 3064;
pub const XML_SCHEMAP_COS_CT_EXTENDS_1_1: C2RustUnnamed_1 = 3063;
pub const XML_SCHEMAP_CVC_SIMPLE_TYPE: C2RustUnnamed_1 = 3062;
pub const XML_SCHEMAP_COS_VALID_DEFAULT_2_2_2: C2RustUnnamed_1 = 3061;
pub const XML_SCHEMAP_COS_VALID_DEFAULT_2_2_1: C2RustUnnamed_1 = 3060;
pub const XML_SCHEMAP_COS_VALID_DEFAULT_2_1: C2RustUnnamed_1 = 3059;
pub const XML_SCHEMAP_COS_VALID_DEFAULT_1: C2RustUnnamed_1 = 3058;
pub const XML_SCHEMAP_NO_XSI: C2RustUnnamed_1 = 3057;
pub const XML_SCHEMAP_NO_XMLNS: C2RustUnnamed_1 = 3056;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_4: C2RustUnnamed_1 = 3055;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_3_2: C2RustUnnamed_1 = 3054;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_3_1: C2RustUnnamed_1 = 3053;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_2: C2RustUnnamed_1 = 3052;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_1: C2RustUnnamed_1 = 3051;
pub const XML_SCHEMAP_SRC_INCLUDE: C2RustUnnamed_1 = 3050;
pub const XML_SCHEMAP_E_PROPS_CORRECT_6: C2RustUnnamed_1 = 3049;
pub const XML_SCHEMAP_E_PROPS_CORRECT_5: C2RustUnnamed_1 = 3048;
pub const XML_SCHEMAP_E_PROPS_CORRECT_4: C2RustUnnamed_1 = 3047;
pub const XML_SCHEMAP_E_PROPS_CORRECT_3: C2RustUnnamed_1 = 3046;
pub const XML_SCHEMAP_E_PROPS_CORRECT_2: C2RustUnnamed_1 = 3045;
pub const XML_SCHEMAP_P_PROPS_CORRECT_2_2: C2RustUnnamed_1 = 3044;
pub const XML_SCHEMAP_P_PROPS_CORRECT_2_1: C2RustUnnamed_1 = 3043;
pub const XML_SCHEMAP_P_PROPS_CORRECT_1: C2RustUnnamed_1 = 3042;
pub const XML_SCHEMAP_SRC_ELEMENT_3: C2RustUnnamed_1 = 3041;
pub const XML_SCHEMAP_SRC_ELEMENT_2_2: C2RustUnnamed_1 = 3040;
pub const XML_SCHEMAP_SRC_ELEMENT_2_1: C2RustUnnamed_1 = 3039;
pub const XML_SCHEMAP_SRC_ELEMENT_1: C2RustUnnamed_1 = 3038;
pub const XML_SCHEMAP_S4S_ATTR_INVALID_VALUE: C2RustUnnamed_1 = 3037;
pub const XML_SCHEMAP_S4S_ATTR_MISSING: C2RustUnnamed_1 = 3036;
pub const XML_SCHEMAP_S4S_ATTR_NOT_ALLOWED: C2RustUnnamed_1 = 3035;
pub const XML_SCHEMAP_S4S_ELEM_MISSING: C2RustUnnamed_1 = 3034;
pub const XML_SCHEMAP_S4S_ELEM_NOT_ALLOWED: C2RustUnnamed_1 = 3033;
pub const XML_SCHEMAP_COS_ST_DERIVED_OK_2_2: C2RustUnnamed_1 = 3032;
pub const XML_SCHEMAP_COS_ST_DERIVED_OK_2_1: C2RustUnnamed_1 = 3031;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_5: C2RustUnnamed_1 = 3030;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_4: C2RustUnnamed_1 = 3029;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_3: C2RustUnnamed_1 = 3028;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_1: C2RustUnnamed_1 = 3027;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_2: C2RustUnnamed_1 = 3026;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_1_2: C2RustUnnamed_1 = 3025;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_1: C2RustUnnamed_1 = 3024;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_1: C2RustUnnamed_1 = 3023;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_5: C2RustUnnamed_1 = 3022;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_4: C2RustUnnamed_1 = 3021;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_3: C2RustUnnamed_1 = 3020;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_2: C2RustUnnamed_1 = 3019;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_1: C2RustUnnamed_1 = 3018;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_1_2: C2RustUnnamed_1 = 3017;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_1_1: C2RustUnnamed_1 = 3016;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_1: C2RustUnnamed_1 = 3015;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_1_3_2: C2RustUnnamed_1 = 3014;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_1_3_1: C2RustUnnamed_1 = 3013;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_1_2: C2RustUnnamed_1 = 3012;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_1_1: C2RustUnnamed_1 = 3011;
pub const XML_SCHEMAP_ST_PROPS_CORRECT_3: C2RustUnnamed_1 = 3010;
pub const XML_SCHEMAP_ST_PROPS_CORRECT_2: C2RustUnnamed_1 = 3009;
pub const XML_SCHEMAP_ST_PROPS_CORRECT_1: C2RustUnnamed_1 = 3008;
pub const XML_SCHEMAP_SRC_UNION_MEMBERTYPES_OR_SIMPLETYPES: C2RustUnnamed_1 =
    3007;
pub const XML_SCHEMAP_SRC_LIST_ITEMTYPE_OR_SIMPLETYPE: C2RustUnnamed_1 = 3006;
pub const XML_SCHEMAP_SRC_RESTRICTION_BASE_OR_SIMPLETYPE: C2RustUnnamed_1 =
    3005;
pub const XML_SCHEMAP_SRC_RESOLVE: C2RustUnnamed_1 = 3004;
pub const XML_SCHEMAP_SRC_SIMPLE_TYPE_4: C2RustUnnamed_1 = 3003;
pub const XML_SCHEMAP_SRC_SIMPLE_TYPE_3: C2RustUnnamed_1 = 3002;
pub const XML_SCHEMAP_SRC_SIMPLE_TYPE_2: C2RustUnnamed_1 = 3001;
pub const XML_SCHEMAP_SRC_SIMPLE_TYPE_1: C2RustUnnamed_1 = 3000;
pub const XML_HTTP_UNKNOWN_HOST: C2RustUnnamed_1 = 2022;
pub const XML_HTTP_USE_IP: C2RustUnnamed_1 = 2021;
pub const XML_HTTP_URL_SYNTAX: C2RustUnnamed_1 = 2020;
pub const XML_FTP_URL_SYNTAX: C2RustUnnamed_1 = 2003;
pub const XML_FTP_ACCNT: C2RustUnnamed_1 = 2002;
pub const XML_FTP_EPSV_ANSWER: C2RustUnnamed_1 = 2001;
pub const XML_FTP_PASV_ANSWER: C2RustUnnamed_1 = 2000;
pub const XML_C14N_RELATIVE_NAMESPACE: C2RustUnnamed_1 = 1955;
pub const XML_C14N_UNKNOW_NODE: C2RustUnnamed_1 = 1954;
pub const XML_C14N_INVALID_NODE: C2RustUnnamed_1 = 1953;
pub const XML_C14N_CREATE_STACK: C2RustUnnamed_1 = 1952;
pub const XML_C14N_REQUIRES_UTF8: C2RustUnnamed_1 = 1951;
pub const XML_C14N_CREATE_CTXT: C2RustUnnamed_1 = 1950;
pub const XML_XPTR_EXTRA_OBJECTS: C2RustUnnamed_1 = 1903;
pub const XML_XPTR_EVAL_FAILED: C2RustUnnamed_1 = 1902;
pub const XML_XPTR_CHILDSEQ_START: C2RustUnnamed_1 = 1901;
pub const XML_XPTR_UNKNOWN_SCHEME: C2RustUnnamed_1 = 1900;
pub const XML_SCHEMAV_MISC: C2RustUnnamed_1 = 1879;
pub const XML_SCHEMAV_CVC_WILDCARD: C2RustUnnamed_1 = 1878;
pub const XML_SCHEMAV_CVC_IDC: C2RustUnnamed_1 = 1877;
pub const XML_SCHEMAV_CVC_TYPE_2: C2RustUnnamed_1 = 1876;
pub const XML_SCHEMAV_CVC_TYPE_1: C2RustUnnamed_1 = 1875;
pub const XML_SCHEMAV_CVC_AU: C2RustUnnamed_1 = 1874;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_1: C2RustUnnamed_1 = 1873;
pub const XML_SCHEMAV_DOCUMENT_ELEMENT_MISSING: C2RustUnnamed_1 = 1872;
pub const XML_SCHEMAV_ELEMENT_CONTENT: C2RustUnnamed_1 = 1871;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_5_2: C2RustUnnamed_1 = 1870;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_5_1: C2RustUnnamed_1 = 1869;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_4: C2RustUnnamed_1 = 1868;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_3_2_2: C2RustUnnamed_1 = 1867;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_3_2_1: C2RustUnnamed_1 = 1866;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_3_1: C2RustUnnamed_1 = 1865;
pub const XML_SCHEMAV_CVC_ATTRIBUTE_4: C2RustUnnamed_1 = 1864;
pub const XML_SCHEMAV_CVC_ATTRIBUTE_3: C2RustUnnamed_1 = 1863;
pub const XML_SCHEMAV_CVC_ATTRIBUTE_2: C2RustUnnamed_1 = 1862;
pub const XML_SCHEMAV_CVC_ATTRIBUTE_1: C2RustUnnamed_1 = 1861;
pub const XML_SCHEMAV_CVC_ELT_7: C2RustUnnamed_1 = 1860;
pub const XML_SCHEMAV_CVC_ELT_6: C2RustUnnamed_1 = 1859;
pub const XML_SCHEMAV_CVC_ELT_5_2_2_2_2: C2RustUnnamed_1 = 1858;
pub const XML_SCHEMAV_CVC_ELT_5_2_2_2_1: C2RustUnnamed_1 = 1857;
pub const XML_SCHEMAV_CVC_ELT_5_2_2_1: C2RustUnnamed_1 = 1856;
pub const XML_SCHEMAV_CVC_ELT_5_2_1: C2RustUnnamed_1 = 1855;
pub const XML_SCHEMAV_CVC_ELT_5_1_2: C2RustUnnamed_1 = 1854;
pub const XML_SCHEMAV_CVC_ELT_5_1_1: C2RustUnnamed_1 = 1853;
pub const XML_SCHEMAV_CVC_ELT_4_3: C2RustUnnamed_1 = 1852;
pub const XML_SCHEMAV_CVC_ELT_4_2: C2RustUnnamed_1 = 1851;
pub const XML_SCHEMAV_CVC_ELT_4_1: C2RustUnnamed_1 = 1850;
pub const XML_SCHEMAV_CVC_ELT_3_2_2: C2RustUnnamed_1 = 1849;
pub const XML_SCHEMAV_CVC_ELT_3_2_1: C2RustUnnamed_1 = 1848;
pub const XML_SCHEMAV_CVC_ELT_3_1: C2RustUnnamed_1 = 1847;
pub const XML_SCHEMAV_CVC_ELT_2: C2RustUnnamed_1 = 1846;
pub const XML_SCHEMAV_CVC_ELT_1: C2RustUnnamed_1 = 1845;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_2_4: C2RustUnnamed_1 = 1844;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_2_3: C2RustUnnamed_1 = 1843;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_2_2: C2RustUnnamed_1 = 1842;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_2_1: C2RustUnnamed_1 = 1841;
pub const XML_SCHEMAV_CVC_ENUMERATION_VALID: C2RustUnnamed_1 = 1840;
pub const XML_SCHEMAV_CVC_PATTERN_VALID: C2RustUnnamed_1 = 1839;
pub const XML_SCHEMAV_CVC_FRACTIONDIGITS_VALID: C2RustUnnamed_1 = 1838;
pub const XML_SCHEMAV_CVC_TOTALDIGITS_VALID: C2RustUnnamed_1 = 1837;
pub const XML_SCHEMAV_CVC_MAXEXCLUSIVE_VALID: C2RustUnnamed_1 = 1836;
pub const XML_SCHEMAV_CVC_MINEXCLUSIVE_VALID: C2RustUnnamed_1 = 1835;
pub const XML_SCHEMAV_CVC_MAXINCLUSIVE_VALID: C2RustUnnamed_1 = 1834;
pub const XML_SCHEMAV_CVC_MININCLUSIVE_VALID: C2RustUnnamed_1 = 1833;
pub const XML_SCHEMAV_CVC_MAXLENGTH_VALID: C2RustUnnamed_1 = 1832;
pub const XML_SCHEMAV_CVC_MINLENGTH_VALID: C2RustUnnamed_1 = 1831;
pub const XML_SCHEMAV_CVC_LENGTH_VALID: C2RustUnnamed_1 = 1830;
pub const XML_SCHEMAV_CVC_FACET_VALID: C2RustUnnamed_1 = 1829;
pub const XML_SCHEMAV_CVC_TYPE_3_1_2: C2RustUnnamed_1 = 1828;
pub const XML_SCHEMAV_CVC_TYPE_3_1_1: C2RustUnnamed_1 = 1827;
pub const XML_SCHEMAV_CVC_DATATYPE_VALID_1_2_3: C2RustUnnamed_1 = 1826;
pub const XML_SCHEMAV_CVC_DATATYPE_VALID_1_2_2: C2RustUnnamed_1 = 1825;
pub const XML_SCHEMAV_CVC_DATATYPE_VALID_1_2_1: C2RustUnnamed_1 = 1824;
pub const XML_SCHEMAV_FACET: C2RustUnnamed_1 = 1823;
pub const XML_SCHEMAV_VALUE: C2RustUnnamed_1 = 1822;
pub const XML_SCHEMAV_ATTRINVALID: C2RustUnnamed_1 = 1821;
pub const XML_SCHEMAV_ATTRUNKNOWN: C2RustUnnamed_1 = 1820;
pub const XML_SCHEMAV_NOTSIMPLE: C2RustUnnamed_1 = 1819;
pub const XML_SCHEMAV_INTERNAL: C2RustUnnamed_1 = 1818;
pub const XML_SCHEMAV_CONSTRUCT: C2RustUnnamed_1 = 1817;
pub const XML_SCHEMAV_NOTDETERMINIST: C2RustUnnamed_1 = 1816;
pub const XML_SCHEMAV_INVALIDELEM: C2RustUnnamed_1 = 1815;
pub const XML_SCHEMAV_INVALIDATTR: C2RustUnnamed_1 = 1814;
pub const XML_SCHEMAV_EXTRACONTENT: C2RustUnnamed_1 = 1813;
pub const XML_SCHEMAV_NOTNILLABLE: C2RustUnnamed_1 = 1812;
pub const XML_SCHEMAV_HAVEDEFAULT: C2RustUnnamed_1 = 1811;
pub const XML_SCHEMAV_ELEMCONT: C2RustUnnamed_1 = 1810;
pub const XML_SCHEMAV_NOTEMPTY: C2RustUnnamed_1 = 1809;
pub const XML_SCHEMAV_ISABSTRACT: C2RustUnnamed_1 = 1808;
pub const XML_SCHEMAV_NOROLLBACK: C2RustUnnamed_1 = 1807;
pub const XML_SCHEMAV_NOTYPE: C2RustUnnamed_1 = 1806;
pub const XML_SCHEMAV_WRONGELEM: C2RustUnnamed_1 = 1805;
pub const XML_SCHEMAV_MISSING: C2RustUnnamed_1 = 1804;
pub const XML_SCHEMAV_NOTTOPLEVEL: C2RustUnnamed_1 = 1803;
pub const XML_SCHEMAV_UNDECLAREDELEM: C2RustUnnamed_1 = 1802;
pub const XML_SCHEMAV_NOROOT: C2RustUnnamed_1 = 1801;
pub const XML_SCHEMAP_COS_CT_EXTENDS_1_3: C2RustUnnamed_1 = 1800;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_4_3: C2RustUnnamed_1 = 1799;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_4_2: C2RustUnnamed_1 = 1798;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_4_1: C2RustUnnamed_1 = 1797;
pub const XML_SCHEMAP_SRC_IMPORT_3_2: C2RustUnnamed_1 = 1796;
pub const XML_SCHEMAP_SRC_IMPORT_3_1: C2RustUnnamed_1 = 1795;
pub const XML_SCHEMAP_UNION_NOT_EXPRESSIBLE: C2RustUnnamed_1 = 1794;
pub const XML_SCHEMAP_INTERSECTION_NOT_EXPRESSIBLE: C2RustUnnamed_1 = 1793;
pub const XML_SCHEMAP_WILDCARD_INVALID_NS_MEMBER: C2RustUnnamed_1 = 1792;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_3: C2RustUnnamed_1 = 1791;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_2_2: C2RustUnnamed_1 = 1790;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_2_1_2: C2RustUnnamed_1 = 1789;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_2_1_1: C2RustUnnamed_1 = 1788;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_1: C2RustUnnamed_1 = 1787;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_5: C2RustUnnamed_1 = 1786;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_4: C2RustUnnamed_1 = 1785;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_3: C2RustUnnamed_1 = 1784;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_2: C2RustUnnamed_1 = 1783;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_1: C2RustUnnamed_1 = 1782;
pub const XML_SCHEMAP_REF_AND_CONTENT: C2RustUnnamed_1 = 1781;
pub const XML_SCHEMAP_INVALID_ATTR_NAME: C2RustUnnamed_1 = 1780;
pub const XML_SCHEMAP_MISSING_SIMPLETYPE_CHILD: C2RustUnnamed_1 = 1779;
pub const XML_SCHEMAP_INVALID_ATTR_INLINE_COMBINATION: C2RustUnnamed_1 = 1778;
pub const XML_SCHEMAP_INVALID_ATTR_COMBINATION: C2RustUnnamed_1 = 1777;
pub const XML_SCHEMAP_SUPERNUMEROUS_LIST_ITEM_TYPE: C2RustUnnamed_1 = 1776;
pub const XML_SCHEMAP_RECURSIVE: C2RustUnnamed_1 = 1775;
pub const XML_SCHEMAP_INVALID_ATTR_USE: C2RustUnnamed_1 = 1774;
pub const XML_SCHEMAP_UNKNOWN_MEMBER_TYPE: C2RustUnnamed_1 = 1773;
pub const XML_SCHEMAP_NOT_SCHEMA: C2RustUnnamed_1 = 1772;
pub const XML_SCHEMAP_INCLUDE_SCHEMA_NO_URI: C2RustUnnamed_1 = 1771;
pub const XML_SCHEMAP_INCLUDE_SCHEMA_NOT_URI: C2RustUnnamed_1 = 1770;
pub const XML_SCHEMAP_UNKNOWN_INCLUDE_CHILD: C2RustUnnamed_1 = 1769;
pub const XML_SCHEMAP_DEF_AND_PREFIX: C2RustUnnamed_1 = 1768;
pub const XML_SCHEMAP_UNKNOWN_PREFIX: C2RustUnnamed_1 = 1767;
pub const XML_SCHEMAP_FAILED_PARSE: C2RustUnnamed_1 = 1766;
pub const XML_SCHEMAP_REDEFINED_NOTATION: C2RustUnnamed_1 = 1765;
pub const XML_SCHEMAP_REDEFINED_ATTR: C2RustUnnamed_1 = 1764;
pub const XML_SCHEMAP_REDEFINED_ATTRGROUP: C2RustUnnamed_1 = 1763;
pub const XML_SCHEMAP_REDEFINED_ELEMENT: C2RustUnnamed_1 = 1762;
pub const XML_SCHEMAP_REDEFINED_TYPE: C2RustUnnamed_1 = 1761;
pub const XML_SCHEMAP_REDEFINED_GROUP: C2RustUnnamed_1 = 1760;
pub const XML_SCHEMAP_NOROOT: C2RustUnnamed_1 = 1759;
pub const XML_SCHEMAP_NOTHING_TO_PARSE: C2RustUnnamed_1 = 1758;
pub const XML_SCHEMAP_FAILED_LOAD: C2RustUnnamed_1 = 1757;
pub const XML_SCHEMAP_REGEXP_INVALID: C2RustUnnamed_1 = 1756;
pub const XML_SCHEMAP_ELEM_DEFAULT_FIXED: C2RustUnnamed_1 = 1755;
pub const XML_SCHEMAP_UNKNOWN_UNION_CHILD: C2RustUnnamed_1 = 1754;
pub const XML_SCHEMAP_UNKNOWN_TYPE: C2RustUnnamed_1 = 1753;
pub const XML_SCHEMAP_UNKNOWN_SIMPLETYPE_CHILD: C2RustUnnamed_1 = 1752;
pub const XML_SCHEMAP_UNKNOWN_SIMPLECONTENT_CHILD: C2RustUnnamed_1 = 1751;
pub const XML_SCHEMAP_UNKNOWN_SEQUENCE_CHILD: C2RustUnnamed_1 = 1750;
pub const XML_SCHEMAP_UNKNOWN_SCHEMAS_CHILD: C2RustUnnamed_1 = 1749;
pub const XML_SCHEMAP_UNKNOWN_RESTRICTION_CHILD: C2RustUnnamed_1 = 1748;
pub const XML_SCHEMAP_UNKNOWN_REF: C2RustUnnamed_1 = 1747;
pub const XML_SCHEMAP_UNKNOWN_PROCESSCONTENT_CHILD: C2RustUnnamed_1 = 1746;
pub const XML_SCHEMAP_UNKNOWN_NOTATION_CHILD: C2RustUnnamed_1 = 1745;
pub const XML_SCHEMAP_UNKNOWN_LIST_CHILD: C2RustUnnamed_1 = 1744;
pub const XML_SCHEMAP_UNKNOWN_IMPORT_CHILD: C2RustUnnamed_1 = 1743;
pub const XML_SCHEMAP_UNKNOWN_GROUP_CHILD: C2RustUnnamed_1 = 1742;
pub const XML_SCHEMAP_UNKNOWN_FACET_TYPE: C2RustUnnamed_1 = 1741;
pub const XML_SCHEMAP_UNKNOWN_FACET_CHILD: C2RustUnnamed_1 = 1740;
pub const XML_SCHEMAP_UNKNOWN_EXTENSION_CHILD: C2RustUnnamed_1 = 1739;
pub const XML_SCHEMAP_UNKNOWN_ELEM_CHILD: C2RustUnnamed_1 = 1738;
pub const XML_SCHEMAP_UNKNOWN_COMPLEXTYPE_CHILD: C2RustUnnamed_1 = 1737;
pub const XML_SCHEMAP_UNKNOWN_COMPLEXCONTENT_CHILD: C2RustUnnamed_1 = 1736;
pub const XML_SCHEMAP_UNKNOWN_CHOICE_CHILD: C2RustUnnamed_1 = 1735;
pub const XML_SCHEMAP_UNKNOWN_BASE_TYPE: C2RustUnnamed_1 = 1734;
pub const XML_SCHEMAP_UNKNOWN_ATTRIBUTE_GROUP: C2RustUnnamed_1 = 1733;
pub const XML_SCHEMAP_UNKNOWN_ATTRGRP_CHILD: C2RustUnnamed_1 = 1732;
pub const XML_SCHEMAP_UNKNOWN_ATTR_CHILD: C2RustUnnamed_1 = 1731;
pub const XML_SCHEMAP_UNKNOWN_ANYATTRIBUTE_CHILD: C2RustUnnamed_1 = 1730;
pub const XML_SCHEMAP_UNKNOWN_ALL_CHILD: C2RustUnnamed_1 = 1729;
pub const XML_SCHEMAP_TYPE_AND_SUBTYPE: C2RustUnnamed_1 = 1728;
pub const XML_SCHEMAP_SIMPLETYPE_NONAME: C2RustUnnamed_1 = 1727;
pub const XML_SCHEMAP_RESTRICTION_NONAME_NOREF: C2RustUnnamed_1 = 1726;
pub const XML_SCHEMAP_REF_AND_SUBTYPE: C2RustUnnamed_1 = 1725;
pub const XML_SCHEMAP_NOTYPE_NOREF: C2RustUnnamed_1 = 1724;
pub const XML_SCHEMAP_NOTATION_NO_NAME: C2RustUnnamed_1 = 1723;
pub const XML_SCHEMAP_NOATTR_NOREF: C2RustUnnamed_1 = 1722;
pub const XML_SCHEMAP_INVALID_WHITE_SPACE: C2RustUnnamed_1 = 1721;
pub const XML_SCHEMAP_INVALID_REF_AND_SUBTYPE: C2RustUnnamed_1 = 1720;
pub const XML_SCHEMAP_INVALID_MINOCCURS: C2RustUnnamed_1 = 1719;
pub const XML_SCHEMAP_INVALID_MAXOCCURS: C2RustUnnamed_1 = 1718;
pub const XML_SCHEMAP_INVALID_FACET_VALUE: C2RustUnnamed_1 = 1717;
pub const XML_SCHEMAP_INVALID_FACET: C2RustUnnamed_1 = 1716;
pub const XML_SCHEMAP_INVALID_ENUM: C2RustUnnamed_1 = 1715;
pub const XML_SCHEMAP_INVALID_BOOLEAN: C2RustUnnamed_1 = 1714;
pub const XML_SCHEMAP_IMPORT_SCHEMA_NOT_URI: C2RustUnnamed_1 = 1713;
pub const XML_SCHEMAP_IMPORT_REDEFINE_NSNAME: C2RustUnnamed_1 = 1712;
pub const XML_SCHEMAP_IMPORT_NAMESPACE_NOT_URI: C2RustUnnamed_1 = 1711;
pub const XML_SCHEMAP_GROUP_NONAME_NOREF: C2RustUnnamed_1 = 1710;
pub const XML_SCHEMAP_FAILED_BUILD_IMPORT: C2RustUnnamed_1 = 1709;
pub const XML_SCHEMAP_FACET_NO_VALUE: C2RustUnnamed_1 = 1708;
pub const XML_SCHEMAP_EXTENSION_NO_BASE: C2RustUnnamed_1 = 1707;
pub const XML_SCHEMAP_ELEM_NONAME_NOREF: C2RustUnnamed_1 = 1706;
pub const XML_SCHEMAP_ELEMFORMDEFAULT_VALUE: C2RustUnnamed_1 = 1705;
pub const XML_SCHEMAP_COMPLEXTYPE_NONAME_NOREF: C2RustUnnamed_1 = 1704;
pub const XML_SCHEMAP_ATTR_NONAME_NOREF: C2RustUnnamed_1 = 1703;
pub const XML_SCHEMAP_ATTRGRP_NONAME_NOREF: C2RustUnnamed_1 = 1702;
pub const XML_SCHEMAP_ATTRFORMDEFAULT_VALUE: C2RustUnnamed_1 = 1701;
pub const XML_SCHEMAP_PREFIX_UNDEFINED: C2RustUnnamed_1 = 1700;
pub const XML_CATALOG_RECURSION: C2RustUnnamed_1 = 1654;
pub const XML_CATALOG_NOT_CATALOG: C2RustUnnamed_1 = 1653;
pub const XML_CATALOG_PREFER_VALUE: C2RustUnnamed_1 = 1652;
pub const XML_CATALOG_ENTRY_BROKEN: C2RustUnnamed_1 = 1651;
pub const XML_CATALOG_MISSING_ATTR: C2RustUnnamed_1 = 1650;
pub const XML_XINCLUDE_FRAGMENT_ID: C2RustUnnamed_1 = 1618;
pub const XML_XINCLUDE_DEPRECATED_NS: C2RustUnnamed_1 = 1617;
pub const XML_XINCLUDE_FALLBACK_NOT_IN_INCLUDE: C2RustUnnamed_1 = 1616;
pub const XML_XINCLUDE_FALLBACKS_IN_INCLUDE: C2RustUnnamed_1 = 1615;
pub const XML_XINCLUDE_INCLUDE_IN_INCLUDE: C2RustUnnamed_1 = 1614;
pub const XML_XINCLUDE_XPTR_RESULT: C2RustUnnamed_1 = 1613;
pub const XML_XINCLUDE_XPTR_FAILED: C2RustUnnamed_1 = 1612;
pub const XML_XINCLUDE_MULTIPLE_ROOT: C2RustUnnamed_1 = 1611;
pub const XML_XINCLUDE_UNKNOWN_ENCODING: C2RustUnnamed_1 = 1610;
pub const XML_XINCLUDE_BUILD_FAILED: C2RustUnnamed_1 = 1609;
pub const XML_XINCLUDE_INVALID_CHAR: C2RustUnnamed_1 = 1608;
pub const XML_XINCLUDE_TEXT_DOCUMENT: C2RustUnnamed_1 = 1607;
pub const XML_XINCLUDE_TEXT_FRAGMENT: C2RustUnnamed_1 = 1606;
pub const XML_XINCLUDE_HREF_URI: C2RustUnnamed_1 = 1605;
pub const XML_XINCLUDE_NO_FALLBACK: C2RustUnnamed_1 = 1604;
pub const XML_XINCLUDE_NO_HREF: C2RustUnnamed_1 = 1603;
pub const XML_XINCLUDE_ENTITY_DEF_MISMATCH: C2RustUnnamed_1 = 1602;
pub const XML_XINCLUDE_PARSE_VALUE: C2RustUnnamed_1 = 1601;
pub const XML_XINCLUDE_RECURSION: C2RustUnnamed_1 = 1600;
pub const XML_IO_EAFNOSUPPORT: C2RustUnnamed_1 = 1556;
pub const XML_IO_EALREADY: C2RustUnnamed_1 = 1555;
pub const XML_IO_EADDRINUSE: C2RustUnnamed_1 = 1554;
pub const XML_IO_ENETUNREACH: C2RustUnnamed_1 = 1553;
pub const XML_IO_ECONNREFUSED: C2RustUnnamed_1 = 1552;
pub const XML_IO_EISCONN: C2RustUnnamed_1 = 1551;
pub const XML_IO_ENOTSOCK: C2RustUnnamed_1 = 1550;
pub const XML_IO_LOAD_ERROR: C2RustUnnamed_1 = 1549;
pub const XML_IO_BUFFER_FULL: C2RustUnnamed_1 = 1548;
pub const XML_IO_NO_INPUT: C2RustUnnamed_1 = 1547;
pub const XML_IO_WRITE: C2RustUnnamed_1 = 1546;
pub const XML_IO_FLUSH: C2RustUnnamed_1 = 1545;
pub const XML_IO_ENCODER: C2RustUnnamed_1 = 1544;
pub const XML_IO_NETWORK_ATTEMPT: C2RustUnnamed_1 = 1543;
pub const XML_IO_EXDEV: C2RustUnnamed_1 = 1542;
pub const XML_IO_ETIMEDOUT: C2RustUnnamed_1 = 1541;
pub const XML_IO_ESRCH: C2RustUnnamed_1 = 1540;
pub const XML_IO_ESPIPE: C2RustUnnamed_1 = 1539;
pub const XML_IO_EROFS: C2RustUnnamed_1 = 1538;
pub const XML_IO_ERANGE: C2RustUnnamed_1 = 1537;
pub const XML_IO_EPIPE: C2RustUnnamed_1 = 1536;
pub const XML_IO_EPERM: C2RustUnnamed_1 = 1535;
pub const XML_IO_ENXIO: C2RustUnnamed_1 = 1534;
pub const XML_IO_ENOTTY: C2RustUnnamed_1 = 1533;
pub const XML_IO_ENOTSUP: C2RustUnnamed_1 = 1532;
pub const XML_IO_ENOTEMPTY: C2RustUnnamed_1 = 1531;
pub const XML_IO_ENOTDIR: C2RustUnnamed_1 = 1530;
pub const XML_IO_ENOSYS: C2RustUnnamed_1 = 1529;
pub const XML_IO_ENOSPC: C2RustUnnamed_1 = 1528;
pub const XML_IO_ENOMEM: C2RustUnnamed_1 = 1527;
pub const XML_IO_ENOLCK: C2RustUnnamed_1 = 1526;
pub const XML_IO_ENOEXEC: C2RustUnnamed_1 = 1525;
pub const XML_IO_ENOENT: C2RustUnnamed_1 = 1524;
pub const XML_IO_ENODEV: C2RustUnnamed_1 = 1523;
pub const XML_IO_ENFILE: C2RustUnnamed_1 = 1522;
pub const XML_IO_ENAMETOOLONG: C2RustUnnamed_1 = 1521;
pub const XML_IO_EMSGSIZE: C2RustUnnamed_1 = 1520;
pub const XML_IO_EMLINK: C2RustUnnamed_1 = 1519;
pub const XML_IO_EMFILE: C2RustUnnamed_1 = 1518;
pub const XML_IO_EISDIR: C2RustUnnamed_1 = 1517;
pub const XML_IO_EIO: C2RustUnnamed_1 = 1516;
pub const XML_IO_EINVAL: C2RustUnnamed_1 = 1515;
pub const XML_IO_EINTR: C2RustUnnamed_1 = 1514;
pub const XML_IO_EINPROGRESS: C2RustUnnamed_1 = 1513;
pub const XML_IO_EFBIG: C2RustUnnamed_1 = 1512;
pub const XML_IO_EFAULT: C2RustUnnamed_1 = 1511;
pub const XML_IO_EEXIST: C2RustUnnamed_1 = 1510;
pub const XML_IO_EDOM: C2RustUnnamed_1 = 1509;
pub const XML_IO_EDEADLK: C2RustUnnamed_1 = 1508;
pub const XML_IO_ECHILD: C2RustUnnamed_1 = 1507;
pub const XML_IO_ECANCELED: C2RustUnnamed_1 = 1506;
pub const XML_IO_EBUSY: C2RustUnnamed_1 = 1505;
pub const XML_IO_EBADMSG: C2RustUnnamed_1 = 1504;
pub const XML_IO_EBADF: C2RustUnnamed_1 = 1503;
pub const XML_IO_EAGAIN: C2RustUnnamed_1 = 1502;
pub const XML_IO_EACCES: C2RustUnnamed_1 = 1501;
pub const XML_IO_UNKNOWN: C2RustUnnamed_1 = 1500;
pub const XML_REGEXP_COMPILE_ERROR: C2RustUnnamed_1 = 1450;
pub const XML_SAVE_UNKNOWN_ENCODING: C2RustUnnamed_1 = 1403;
pub const XML_SAVE_NO_DOCTYPE: C2RustUnnamed_1 = 1402;
pub const XML_SAVE_CHAR_INVALID: C2RustUnnamed_1 = 1401;
pub const XML_SAVE_NOT_UTF8: C2RustUnnamed_1 = 1400;
pub const XML_XPATH_INVALID_CHAR_ERROR: C2RustUnnamed_1 = 1221;
pub const XML_XPATH_ENCODING_ERROR: C2RustUnnamed_1 = 1220;
pub const XML_XPATH_UNDEF_PREFIX_ERROR: C2RustUnnamed_1 = 1219;
pub const XML_XPTR_SUB_RESOURCE_ERROR: C2RustUnnamed_1 = 1218;
pub const XML_XPTR_RESOURCE_ERROR: C2RustUnnamed_1 = 1217;
pub const XML_XPTR_SYNTAX_ERROR: C2RustUnnamed_1 = 1216;
pub const XML_XPATH_MEMORY_ERROR: C2RustUnnamed_1 = 1215;
pub const XML_XPATH_INVALID_CTXT_POSITION: C2RustUnnamed_1 = 1214;
pub const XML_XPATH_INVALID_CTXT_SIZE: C2RustUnnamed_1 = 1213;
pub const XML_XPATH_INVALID_ARITY: C2RustUnnamed_1 = 1212;
pub const XML_XPATH_INVALID_TYPE: C2RustUnnamed_1 = 1211;
pub const XML_XPATH_INVALID_OPERAND: C2RustUnnamed_1 = 1210;
pub const XML_XPATH_UNKNOWN_FUNC_ERROR: C2RustUnnamed_1 = 1209;
pub const XML_XPATH_UNCLOSED_ERROR: C2RustUnnamed_1 = 1208;
pub const XML_XPATH_EXPR_ERROR: C2RustUnnamed_1 = 1207;
pub const XML_XPATH_INVALID_PREDICATE_ERROR: C2RustUnnamed_1 = 1206;
pub const XML_XPATH_UNDEF_VARIABLE_ERROR: C2RustUnnamed_1 = 1205;
pub const XML_XPATH_VARIABLE_REF_ERROR: C2RustUnnamed_1 = 1204;
pub const XML_XPATH_START_LITERAL_ERROR: C2RustUnnamed_1 = 1203;
pub const XML_XPATH_UNFINISHED_LITERAL_ERROR: C2RustUnnamed_1 = 1202;
pub const XML_XPATH_NUMBER_ERROR: C2RustUnnamed_1 = 1201;
pub const XML_XPATH_EXPRESSION_OK: C2RustUnnamed_1 = 1200;
pub const XML_RNGP_XML_NS: C2RustUnnamed_1 = 1122;
pub const XML_RNGP_XMLNS_NAME: C2RustUnnamed_1 = 1121;
pub const XML_RNGP_VALUE_NO_CONTENT: C2RustUnnamed_1 = 1120;
pub const XML_RNGP_VALUE_EMPTY: C2RustUnnamed_1 = 1119;
pub const XML_RNGP_URI_NOT_ABSOLUTE: C2RustUnnamed_1 = 1118;
pub const XML_RNGP_URI_FRAGMENT: C2RustUnnamed_1 = 1117;
pub const XML_RNGP_UNKNOWN_TYPE_LIB: C2RustUnnamed_1 = 1116;
pub const XML_RNGP_UNKNOWN_CONSTRUCT: C2RustUnnamed_1 = 1115;
pub const XML_RNGP_UNKNOWN_COMBINE: C2RustUnnamed_1 = 1114;
pub const XML_RNGP_UNKNOWN_ATTRIBUTE: C2RustUnnamed_1 = 1113;
pub const XML_RNGP_TYPE_VALUE: C2RustUnnamed_1 = 1112;
pub const XML_RNGP_TYPE_NOT_FOUND: C2RustUnnamed_1 = 1111;
pub const XML_RNGP_TYPE_MISSING: C2RustUnnamed_1 = 1110;
pub const XML_RNGP_TEXT_HAS_CHILD: C2RustUnnamed_1 = 1109;
pub const XML_RNGP_TEXT_EXPECTED: C2RustUnnamed_1 = 1108;
pub const XML_RNGP_START_MISSING: C2RustUnnamed_1 = 1107;
pub const XML_RNGP_START_EMPTY: C2RustUnnamed_1 = 1106;
pub const XML_RNGP_START_CONTENT: C2RustUnnamed_1 = 1105;
pub const XML_RNGP_START_CHOICE_AND_INTERLEAVE: C2RustUnnamed_1 = 1104;
pub const XML_RNGP_REF_NOT_EMPTY: C2RustUnnamed_1 = 1103;
pub const XML_RNGP_REF_NO_NAME: C2RustUnnamed_1 = 1102;
pub const XML_RNGP_REF_NO_DEF: C2RustUnnamed_1 = 1101;
pub const XML_RNGP_REF_NAME_INVALID: C2RustUnnamed_1 = 1100;
pub const XML_RNGP_REF_CYCLE: C2RustUnnamed_1 = 1099;
pub const XML_RNGP_REF_CREATE_FAILED: C2RustUnnamed_1 = 1098;
pub const XML_RNGP_PREFIX_UNDEFINED: C2RustUnnamed_1 = 1097;
pub const XML_RNGP_PAT_START_VALUE: C2RustUnnamed_1 = 1096;
pub const XML_RNGP_PAT_START_TEXT: C2RustUnnamed_1 = 1095;
pub const XML_RNGP_PAT_START_ONEMORE: C2RustUnnamed_1 = 1094;
pub const XML_RNGP_PAT_START_LIST: C2RustUnnamed_1 = 1093;
pub const XML_RNGP_PAT_START_INTERLEAVE: C2RustUnnamed_1 = 1092;
pub const XML_RNGP_PAT_START_GROUP: C2RustUnnamed_1 = 1091;
pub const XML_RNGP_PAT_START_EMPTY: C2RustUnnamed_1 = 1090;
pub const XML_RNGP_PAT_START_DATA: C2RustUnnamed_1 = 1089;
pub const XML_RNGP_PAT_START_ATTR: C2RustUnnamed_1 = 1088;
pub const XML_RNGP_PAT_ONEMORE_INTERLEAVE_ATTR: C2RustUnnamed_1 = 1087;
pub const XML_RNGP_PAT_ONEMORE_GROUP_ATTR: C2RustUnnamed_1 = 1086;
pub const XML_RNGP_PAT_NSNAME_EXCEPT_NSNAME: C2RustUnnamed_1 = 1085;
pub const XML_RNGP_PAT_NSNAME_EXCEPT_ANYNAME: C2RustUnnamed_1 = 1084;
pub const XML_RNGP_PAT_LIST_TEXT: C2RustUnnamed_1 = 1083;
pub const XML_RNGP_PAT_LIST_REF: C2RustUnnamed_1 = 1082;
pub const XML_RNGP_PAT_LIST_LIST: C2RustUnnamed_1 = 1081;
pub const XML_RNGP_PAT_LIST_INTERLEAVE: C2RustUnnamed_1 = 1080;
pub const XML_RNGP_PAT_LIST_ELEM: C2RustUnnamed_1 = 1079;
pub const XML_RNGP_PAT_LIST_ATTR: C2RustUnnamed_1 = 1078;
pub const XML_RNGP_PAT_DATA_EXCEPT_TEXT: C2RustUnnamed_1 = 1077;
pub const XML_RNGP_PAT_DATA_EXCEPT_REF: C2RustUnnamed_1 = 1076;
pub const XML_RNGP_PAT_DATA_EXCEPT_ONEMORE: C2RustUnnamed_1 = 1075;
pub const XML_RNGP_PAT_DATA_EXCEPT_LIST: C2RustUnnamed_1 = 1074;
pub const XML_RNGP_PAT_DATA_EXCEPT_INTERLEAVE: C2RustUnnamed_1 = 1073;
pub const XML_RNGP_PAT_DATA_EXCEPT_GROUP: C2RustUnnamed_1 = 1072;
pub const XML_RNGP_PAT_DATA_EXCEPT_EMPTY: C2RustUnnamed_1 = 1071;
pub const XML_RNGP_PAT_DATA_EXCEPT_ELEM: C2RustUnnamed_1 = 1070;
pub const XML_RNGP_PAT_DATA_EXCEPT_ATTR: C2RustUnnamed_1 = 1069;
pub const XML_RNGP_PAT_ATTR_ELEM: C2RustUnnamed_1 = 1068;
pub const XML_RNGP_PAT_ATTR_ATTR: C2RustUnnamed_1 = 1067;
pub const XML_RNGP_PAT_ANYNAME_EXCEPT_ANYNAME: C2RustUnnamed_1 = 1066;
pub const XML_RNGP_PARSE_ERROR: C2RustUnnamed_1 = 1065;
pub const XML_RNGP_PARENTREF_NOT_EMPTY: C2RustUnnamed_1 = 1064;
pub const XML_RNGP_PARENTREF_NO_PARENT: C2RustUnnamed_1 = 1063;
pub const XML_RNGP_PARENTREF_NO_NAME: C2RustUnnamed_1 = 1062;
pub const XML_RNGP_PARENTREF_NAME_INVALID: C2RustUnnamed_1 = 1061;
pub const XML_RNGP_PARENTREF_CREATE_FAILED: C2RustUnnamed_1 = 1060;
pub const XML_RNGP_PARAM_NAME_MISSING: C2RustUnnamed_1 = 1059;
pub const XML_RNGP_PARAM_FORBIDDEN: C2RustUnnamed_1 = 1058;
pub const XML_RNGP_NSNAME_NO_NS: C2RustUnnamed_1 = 1057;
pub const XML_RNGP_NSNAME_ATTR_ANCESTOR: C2RustUnnamed_1 = 1056;
pub const XML_RNGP_NOTALLOWED_NOT_EMPTY: C2RustUnnamed_1 = 1055;
pub const XML_RNGP_NEED_COMBINE: C2RustUnnamed_1 = 1054;
pub const XML_RNGP_NAME_MISSING: C2RustUnnamed_1 = 1053;
pub const XML_RNGP_MISSING_HREF: C2RustUnnamed_1 = 1052;
pub const XML_RNGP_INVALID_VALUE: C2RustUnnamed_1 = 1051;
pub const XML_RNGP_INVALID_URI: C2RustUnnamed_1 = 1050;
pub const XML_RNGP_INVALID_DEFINE_NAME: C2RustUnnamed_1 = 1049;
pub const XML_RNGP_INTERLEAVE_NO_CONTENT: C2RustUnnamed_1 = 1048;
pub const XML_RNGP_INTERLEAVE_EMPTY: C2RustUnnamed_1 = 1047;
pub const XML_RNGP_INTERLEAVE_CREATE_FAILED: C2RustUnnamed_1 = 1046;
pub const XML_RNGP_INTERLEAVE_ADD: C2RustUnnamed_1 = 1045;
pub const XML_RNGP_INCLUDE_RECURSE: C2RustUnnamed_1 = 1044;
pub const XML_RNGP_INCLUDE_FAILURE: C2RustUnnamed_1 = 1043;
pub const XML_RNGP_INCLUDE_EMPTY: C2RustUnnamed_1 = 1042;
pub const XML_RNGP_HREF_ERROR: C2RustUnnamed_1 = 1041;
pub const XML_RNGP_GROUP_ATTR_CONFLICT: C2RustUnnamed_1 = 1040;
pub const XML_RNGP_GRAMMAR_NO_START: C2RustUnnamed_1 = 1039;
pub const XML_RNGP_GRAMMAR_MISSING: C2RustUnnamed_1 = 1038;
pub const XML_RNGP_GRAMMAR_EMPTY: C2RustUnnamed_1 = 1037;
pub const XML_RNGP_GRAMMAR_CONTENT: C2RustUnnamed_1 = 1036;
pub const XML_RNGP_FOREIGN_ELEMENT: C2RustUnnamed_1 = 1035;
pub const XML_RNGP_FORBIDDEN_ATTRIBUTE: C2RustUnnamed_1 = 1034;
pub const XML_RNGP_EXTERNALREF_RECURSE: C2RustUnnamed_1 = 1033;
pub const XML_RNGP_EXTERNAL_REF_FAILURE: C2RustUnnamed_1 = 1032;
pub const XML_RNGP_EXTERNALREF_EMTPY: C2RustUnnamed_1 = 1031;
pub const XML_RNGP_EXCEPT_NO_CONTENT: C2RustUnnamed_1 = 1030;
pub const XML_RNGP_EXCEPT_MULTIPLE: C2RustUnnamed_1 = 1029;
pub const XML_RNGP_EXCEPT_MISSING: C2RustUnnamed_1 = 1028;
pub const XML_RNGP_EXCEPT_EMPTY: C2RustUnnamed_1 = 1027;
pub const XML_RNGP_ERROR_TYPE_LIB: C2RustUnnamed_1 = 1026;
pub const XML_RNGP_EMPTY_NOT_EMPTY: C2RustUnnamed_1 = 1025;
pub const XML_RNGP_EMPTY_CONTENT: C2RustUnnamed_1 = 1024;
pub const XML_RNGP_EMPTY_CONSTRUCT: C2RustUnnamed_1 = 1023;
pub const XML_RNGP_EMPTY: C2RustUnnamed_1 = 1022;
pub const XML_RNGP_ELEM_TEXT_CONFLICT: C2RustUnnamed_1 = 1021;
pub const XML_RNGP_ELEMENT_NO_CONTENT: C2RustUnnamed_1 = 1020;
pub const XML_RNGP_ELEMENT_NAME: C2RustUnnamed_1 = 1019;
pub const XML_RNGP_ELEMENT_CONTENT: C2RustUnnamed_1 = 1018;
pub const XML_RNGP_ELEMENT_EMPTY: C2RustUnnamed_1 = 1017;
pub const XML_RNGP_ELEM_CONTENT_ERROR: C2RustUnnamed_1 = 1016;
pub const XML_RNGP_ELEM_CONTENT_EMPTY: C2RustUnnamed_1 = 1015;
pub const XML_RNGP_DEFINE_NAME_MISSING: C2RustUnnamed_1 = 1014;
pub const XML_RNGP_DEFINE_MISSING: C2RustUnnamed_1 = 1013;
pub const XML_RNGP_DEFINE_EMPTY: C2RustUnnamed_1 = 1012;
pub const XML_RNGP_DEFINE_CREATE_FAILED: C2RustUnnamed_1 = 1011;
pub const XML_RNGP_DEF_CHOICE_AND_INTERLEAVE: C2RustUnnamed_1 = 1010;
pub const XML_RNGP_DATA_CONTENT: C2RustUnnamed_1 = 1009;
pub const XML_RNGP_CREATE_FAILURE: C2RustUnnamed_1 = 1008;
pub const XML_RNGP_CHOICE_EMPTY: C2RustUnnamed_1 = 1007;
pub const XML_RNGP_CHOICE_CONTENT: C2RustUnnamed_1 = 1006;
pub const XML_RNGP_ATTRIBUTE_NOOP: C2RustUnnamed_1 = 1005;
pub const XML_RNGP_ATTRIBUTE_EMPTY: C2RustUnnamed_1 = 1004;
pub const XML_RNGP_ATTRIBUTE_CONTENT: C2RustUnnamed_1 = 1003;
pub const XML_RNGP_ATTRIBUTE_CHILDREN: C2RustUnnamed_1 = 1002;
pub const XML_RNGP_ATTR_CONFLICT: C2RustUnnamed_1 = 1001;
pub const XML_RNGP_ANYNAME_ATTR_ANCESTOR: C2RustUnnamed_1 = 1000;
pub const XML_HTML_UNKNOWN_TAG: C2RustUnnamed_1 = 801;
pub const XML_HTML_STRUCURE_ERROR: C2RustUnnamed_1 = 800;
pub const XML_DTD_DUP_TOKEN: C2RustUnnamed_1 = 541;
pub const XML_DTD_XMLID_TYPE: C2RustUnnamed_1 = 540;
pub const XML_DTD_XMLID_VALUE: C2RustUnnamed_1 = 539;
pub const XML_DTD_STANDALONE_DEFAULTED: C2RustUnnamed_1 = 538;
pub const XML_DTD_UNKNOWN_NOTATION: C2RustUnnamed_1 = 537;
pub const XML_DTD_UNKNOWN_ID: C2RustUnnamed_1 = 536;
pub const XML_DTD_UNKNOWN_ENTITY: C2RustUnnamed_1 = 535;
pub const XML_DTD_UNKNOWN_ELEM: C2RustUnnamed_1 = 534;
pub const XML_DTD_UNKNOWN_ATTRIBUTE: C2RustUnnamed_1 = 533;
pub const XML_DTD_STANDALONE_WHITE_SPACE: C2RustUnnamed_1 = 532;
pub const XML_DTD_ROOT_NAME: C2RustUnnamed_1 = 531;
pub const XML_DTD_NOT_STANDALONE: C2RustUnnamed_1 = 530;
pub const XML_DTD_NOT_PCDATA: C2RustUnnamed_1 = 529;
pub const XML_DTD_NOT_EMPTY: C2RustUnnamed_1 = 528;
pub const XML_DTD_NOTATION_VALUE: C2RustUnnamed_1 = 527;
pub const XML_DTD_NOTATION_REDEFINED: C2RustUnnamed_1 = 526;
pub const XML_DTD_NO_ROOT: C2RustUnnamed_1 = 525;
pub const XML_DTD_NO_PREFIX: C2RustUnnamed_1 = 524;
pub const XML_DTD_NO_ELEM_NAME: C2RustUnnamed_1 = 523;
pub const XML_DTD_NO_DTD: C2RustUnnamed_1 = 522;
pub const XML_DTD_NO_DOC: C2RustUnnamed_1 = 521;
pub const XML_DTD_MULTIPLE_ID: C2RustUnnamed_1 = 520;
pub const XML_DTD_MIXED_CORRUPT: C2RustUnnamed_1 = 519;
pub const XML_DTD_MISSING_ATTRIBUTE: C2RustUnnamed_1 = 518;
pub const XML_DTD_LOAD_ERROR: C2RustUnnamed_1 = 517;
pub const XML_DTD_INVALID_DEFAULT: C2RustUnnamed_1 = 516;
pub const XML_DTD_INVALID_CHILD: C2RustUnnamed_1 = 515;
pub const XML_DTD_ID_SUBSET: C2RustUnnamed_1 = 514;
pub const XML_DTD_ID_REDEFINED: C2RustUnnamed_1 = 513;
pub const XML_DTD_ID_FIXED: C2RustUnnamed_1 = 512;
pub const XML_DTD_ENTITY_TYPE: C2RustUnnamed_1 = 511;
pub const XML_DTD_EMPTY_NOTATION: C2RustUnnamed_1 = 510;
pub const XML_DTD_ELEM_REDEFINED: C2RustUnnamed_1 = 509;
pub const XML_DTD_ELEM_NAMESPACE: C2RustUnnamed_1 = 508;
pub const XML_DTD_ELEM_DEFAULT_NAMESPACE: C2RustUnnamed_1 = 507;
pub const XML_DTD_DIFFERENT_PREFIX: C2RustUnnamed_1 = 506;
pub const XML_DTD_CONTENT_NOT_DETERMINIST: C2RustUnnamed_1 = 505;
pub const XML_DTD_CONTENT_MODEL: C2RustUnnamed_1 = 504;
pub const XML_DTD_CONTENT_ERROR: C2RustUnnamed_1 = 503;
pub const XML_DTD_ATTRIBUTE_VALUE: C2RustUnnamed_1 = 502;
pub const XML_DTD_ATTRIBUTE_REDEFINED: C2RustUnnamed_1 = 501;
pub const XML_DTD_ATTRIBUTE_DEFAULT: C2RustUnnamed_1 = 500;
pub const XML_NS_ERR_COLON: C2RustUnnamed_1 = 205;
pub const XML_NS_ERR_EMPTY: C2RustUnnamed_1 = 204;
pub const XML_NS_ERR_ATTRIBUTE_REDEFINED: C2RustUnnamed_1 = 203;
pub const XML_NS_ERR_QNAME: C2RustUnnamed_1 = 202;
pub const XML_NS_ERR_UNDEFINED_NAMESPACE: C2RustUnnamed_1 = 201;
pub const XML_NS_ERR_XML_NAMESPACE: C2RustUnnamed_1 = 200;
pub const XML_ERR_USER_STOP: C2RustUnnamed_1 = 111;
pub const XML_ERR_NAME_TOO_LONG: C2RustUnnamed_1 = 110;
pub const XML_ERR_VERSION_MISMATCH: C2RustUnnamed_1 = 109;
pub const XML_ERR_UNKNOWN_VERSION: C2RustUnnamed_1 = 108;
pub const XML_WAR_ENTITY_REDEFINED: C2RustUnnamed_1 = 107;
pub const XML_WAR_NS_COLUMN: C2RustUnnamed_1 = 106;
pub const XML_ERR_NOTATION_PROCESSING: C2RustUnnamed_1 = 105;
pub const XML_ERR_ENTITY_PROCESSING: C2RustUnnamed_1 = 104;
pub const XML_ERR_NOT_STANDALONE: C2RustUnnamed_1 = 103;
pub const XML_WAR_SPACE_VALUE: C2RustUnnamed_1 = 102;
pub const XML_ERR_MISSING_ENCODING: C2RustUnnamed_1 = 101;
pub const XML_WAR_NS_URI_RELATIVE: C2RustUnnamed_1 = 100;
pub const XML_WAR_NS_URI: C2RustUnnamed_1 = 99;
pub const XML_WAR_LANG_VALUE: C2RustUnnamed_1 = 98;
pub const XML_WAR_UNKNOWN_VERSION: C2RustUnnamed_1 = 97;
pub const XML_ERR_VERSION_MISSING: C2RustUnnamed_1 = 96;
pub const XML_ERR_CONDSEC_INVALID_KEYWORD: C2RustUnnamed_1 = 95;
pub const XML_ERR_NO_DTD: C2RustUnnamed_1 = 94;
pub const XML_WAR_CATALOG_PI: C2RustUnnamed_1 = 93;
pub const XML_ERR_URI_FRAGMENT: C2RustUnnamed_1 = 92;
pub const XML_ERR_INVALID_URI: C2RustUnnamed_1 = 91;
pub const XML_ERR_ENTITY_BOUNDARY: C2RustUnnamed_1 = 90;
pub const XML_ERR_ENTITY_LOOP: C2RustUnnamed_1 = 89;
pub const XML_ERR_ENTITY_PE_INTERNAL: C2RustUnnamed_1 = 88;
pub const XML_ERR_ENTITY_CHAR_ERROR: C2RustUnnamed_1 = 87;
pub const XML_ERR_EXTRA_CONTENT: C2RustUnnamed_1 = 86;
pub const XML_ERR_NOT_WELL_BALANCED: C2RustUnnamed_1 = 85;
pub const XML_ERR_VALUE_REQUIRED: C2RustUnnamed_1 = 84;
pub const XML_ERR_CONDSEC_INVALID: C2RustUnnamed_1 = 83;
pub const XML_ERR_EXT_ENTITY_STANDALONE: C2RustUnnamed_1 = 82;
pub const XML_ERR_INVALID_ENCODING: C2RustUnnamed_1 = 81;
pub const XML_ERR_HYPHEN_IN_COMMENT: C2RustUnnamed_1 = 80;
pub const XML_ERR_ENCODING_NAME: C2RustUnnamed_1 = 79;
pub const XML_ERR_STANDALONE_VALUE: C2RustUnnamed_1 = 78;
pub const XML_ERR_TAG_NOT_FINISHED: C2RustUnnamed_1 = 77;
pub const XML_ERR_TAG_NAME_MISMATCH: C2RustUnnamed_1 = 76;
pub const XML_ERR_EQUAL_REQUIRED: C2RustUnnamed_1 = 75;
pub const XML_ERR_LTSLASH_REQUIRED: C2RustUnnamed_1 = 74;
pub const XML_ERR_GT_REQUIRED: C2RustUnnamed_1 = 73;
pub const XML_ERR_LT_REQUIRED: C2RustUnnamed_1 = 72;
pub const XML_ERR_PUBID_REQUIRED: C2RustUnnamed_1 = 71;
pub const XML_ERR_URI_REQUIRED: C2RustUnnamed_1 = 70;
pub const XML_ERR_PCDATA_REQUIRED: C2RustUnnamed_1 = 69;
pub const XML_ERR_NAME_REQUIRED: C2RustUnnamed_1 = 68;
pub const XML_ERR_NMTOKEN_REQUIRED: C2RustUnnamed_1 = 67;
pub const XML_ERR_SEPARATOR_REQUIRED: C2RustUnnamed_1 = 66;
pub const XML_ERR_SPACE_REQUIRED: C2RustUnnamed_1 = 65;
pub const XML_ERR_RESERVED_XML_NAME: C2RustUnnamed_1 = 64;
pub const XML_ERR_CDATA_NOT_FINISHED: C2RustUnnamed_1 = 63;
pub const XML_ERR_MISPLACED_CDATA_END: C2RustUnnamed_1 = 62;
pub const XML_ERR_DOCTYPE_NOT_FINISHED: C2RustUnnamed_1 = 61;
pub const XML_ERR_EXT_SUBSET_NOT_FINISHED: C2RustUnnamed_1 = 60;
pub const XML_ERR_CONDSEC_NOT_FINISHED: C2RustUnnamed_1 = 59;
pub const XML_ERR_CONDSEC_NOT_STARTED: C2RustUnnamed_1 = 58;
pub const XML_ERR_XMLDECL_NOT_FINISHED: C2RustUnnamed_1 = 57;
pub const XML_ERR_XMLDECL_NOT_STARTED: C2RustUnnamed_1 = 56;
pub const XML_ERR_ELEMCONTENT_NOT_FINISHED: C2RustUnnamed_1 = 55;
pub const XML_ERR_ELEMCONTENT_NOT_STARTED: C2RustUnnamed_1 = 54;
pub const XML_ERR_MIXED_NOT_FINISHED: C2RustUnnamed_1 = 53;
pub const XML_ERR_MIXED_NOT_STARTED: C2RustUnnamed_1 = 52;
pub const XML_ERR_ATTLIST_NOT_FINISHED: C2RustUnnamed_1 = 51;
pub const XML_ERR_ATTLIST_NOT_STARTED: C2RustUnnamed_1 = 50;
pub const XML_ERR_NOTATION_NOT_FINISHED: C2RustUnnamed_1 = 49;
pub const XML_ERR_NOTATION_NOT_STARTED: C2RustUnnamed_1 = 48;
pub const XML_ERR_PI_NOT_FINISHED: C2RustUnnamed_1 = 47;
pub const XML_ERR_PI_NOT_STARTED: C2RustUnnamed_1 = 46;
pub const XML_ERR_COMMENT_NOT_FINISHED: C2RustUnnamed_1 = 45;
pub const XML_ERR_LITERAL_NOT_FINISHED: C2RustUnnamed_1 = 44;
pub const XML_ERR_LITERAL_NOT_STARTED: C2RustUnnamed_1 = 43;
pub const XML_ERR_ATTRIBUTE_REDEFINED: C2RustUnnamed_1 = 42;
pub const XML_ERR_ATTRIBUTE_WITHOUT_VALUE: C2RustUnnamed_1 = 41;
pub const XML_ERR_ATTRIBUTE_NOT_FINISHED: C2RustUnnamed_1 = 40;
pub const XML_ERR_ATTRIBUTE_NOT_STARTED: C2RustUnnamed_1 = 39;
pub const XML_ERR_LT_IN_ATTRIBUTE: C2RustUnnamed_1 = 38;
pub const XML_ERR_ENTITY_NOT_FINISHED: C2RustUnnamed_1 = 37;
pub const XML_ERR_ENTITY_NOT_STARTED: C2RustUnnamed_1 = 36;
pub const XML_ERR_NS_DECL_ERROR: C2RustUnnamed_1 = 35;
pub const XML_ERR_STRING_NOT_CLOSED: C2RustUnnamed_1 = 34;
pub const XML_ERR_STRING_NOT_STARTED: C2RustUnnamed_1 = 33;
pub const XML_ERR_UNSUPPORTED_ENCODING: C2RustUnnamed_1 = 32;
pub const XML_ERR_UNKNOWN_ENCODING: C2RustUnnamed_1 = 31;
pub const XML_ERR_ENTITY_IS_PARAMETER: C2RustUnnamed_1 = 30;
pub const XML_ERR_ENTITY_IS_EXTERNAL: C2RustUnnamed_1 = 29;
pub const XML_ERR_UNPARSED_ENTITY: C2RustUnnamed_1 = 28;
pub const XML_WAR_UNDECLARED_ENTITY: C2RustUnnamed_1 = 27;
pub const XML_ERR_UNDECLARED_ENTITY: C2RustUnnamed_1 = 26;
pub const XML_ERR_PEREF_SEMICOL_MISSING: C2RustUnnamed_1 = 25;
pub const XML_ERR_PEREF_NO_NAME: C2RustUnnamed_1 = 24;
pub const XML_ERR_ENTITYREF_SEMICOL_MISSING: C2RustUnnamed_1 = 23;
pub const XML_ERR_ENTITYREF_NO_NAME: C2RustUnnamed_1 = 22;
pub const XML_ERR_PEREF_IN_INT_SUBSET: C2RustUnnamed_1 = 21;
pub const XML_ERR_PEREF_IN_EPILOG: C2RustUnnamed_1 = 20;
pub const XML_ERR_PEREF_IN_PROLOG: C2RustUnnamed_1 = 19;
pub const XML_ERR_PEREF_AT_EOF: C2RustUnnamed_1 = 18;
pub const XML_ERR_ENTITYREF_IN_DTD: C2RustUnnamed_1 = 17;
pub const XML_ERR_ENTITYREF_IN_EPILOG: C2RustUnnamed_1 = 16;
pub const XML_ERR_ENTITYREF_IN_PROLOG: C2RustUnnamed_1 = 15;
pub const XML_ERR_ENTITYREF_AT_EOF: C2RustUnnamed_1 = 14;
pub const XML_ERR_CHARREF_IN_DTD: C2RustUnnamed_1 = 13;
pub const XML_ERR_CHARREF_IN_EPILOG: C2RustUnnamed_1 = 12;
pub const XML_ERR_CHARREF_IN_PROLOG: C2RustUnnamed_1 = 11;
pub const XML_ERR_CHARREF_AT_EOF: C2RustUnnamed_1 = 10;
pub const XML_ERR_INVALID_CHAR: C2RustUnnamed_1 = 9;
pub const XML_ERR_INVALID_CHARREF: C2RustUnnamed_1 = 8;
pub const XML_ERR_INVALID_DEC_CHARREF: C2RustUnnamed_1 = 7;
pub const XML_ERR_INVALID_HEX_CHARREF: C2RustUnnamed_1 = 6;
pub const XML_ERR_DOCUMENT_END: C2RustUnnamed_1 = 5;
pub const XML_ERR_DOCUMENT_EMPTY: C2RustUnnamed_1 = 4;
pub const XML_ERR_DOCUMENT_START: C2RustUnnamed_1 = 3;
pub const XML_ERR_INTERNAL_ERROR: C2RustUnnamed_1 = 1;
pub const XML_ERR_OK: C2RustUnnamed_1 = 0;
pub type C2RustUnnamed_2 = std::os::raw::c_int;
pub const XML_CHAR_ENCODING_ASCII: C2RustUnnamed_2 = 22;
pub const XML_CHAR_ENCODING_EUC_JP: C2RustUnnamed_2 = 21;
pub const XML_CHAR_ENCODING_SHIFT_JIS: C2RustUnnamed_2 = 20;
pub const XML_CHAR_ENCODING_2022_JP: C2RustUnnamed_2 = 19;
pub const XML_CHAR_ENCODING_8859_9: C2RustUnnamed_2 = 18;
pub const XML_CHAR_ENCODING_8859_8: C2RustUnnamed_2 = 17;
pub const XML_CHAR_ENCODING_8859_7: C2RustUnnamed_2 = 16;
pub const XML_CHAR_ENCODING_8859_6: C2RustUnnamed_2 = 15;
pub const XML_CHAR_ENCODING_8859_5: C2RustUnnamed_2 = 14;
pub const XML_CHAR_ENCODING_8859_4: C2RustUnnamed_2 = 13;
pub const XML_CHAR_ENCODING_8859_3: C2RustUnnamed_2 = 12;
pub const XML_CHAR_ENCODING_8859_2: C2RustUnnamed_2 = 11;
pub const XML_CHAR_ENCODING_8859_1: C2RustUnnamed_2 = 10;
pub const XML_CHAR_ENCODING_UCS2: C2RustUnnamed_2 = 9;
pub const XML_CHAR_ENCODING_UCS4_3412: C2RustUnnamed_2 = 8;
pub const XML_CHAR_ENCODING_UCS4_2143: C2RustUnnamed_2 = 7;
pub const XML_CHAR_ENCODING_EBCDIC: C2RustUnnamed_2 = 6;
pub const XML_CHAR_ENCODING_UCS4BE: C2RustUnnamed_2 = 5;
pub const XML_CHAR_ENCODING_UCS4LE: C2RustUnnamed_2 = 4;
pub const XML_CHAR_ENCODING_UTF16BE: C2RustUnnamed_2 = 3;
pub const XML_CHAR_ENCODING_UTF16LE: C2RustUnnamed_2 = 2;
pub const XML_CHAR_ENCODING_NONE: C2RustUnnamed_2 = 0;
pub const XML_CHAR_ENCODING_ERROR: C2RustUnnamed_2 = -1;
/*
 * tree.c : implementation of access function for an XML tree.
 *
 * References:
 *   XHTML 1.0 W3C REC: http://www.w3.org/TR/2002/REC-xhtml1-20020801/
 *
 * See Copyright for the status of this software.
 *
 * daniel@veillard.com
 *
 */
/* To avoid EBCDIC trouble when parsing on zOS */
/* for memset() only ! */
#[no_mangle]
pub static mut __xmlRegisterCallbacks: std::os::raw::c_int = 0 as std::os::raw::c_int;
/* ***********************************************************************
 *									*
 *		Tree memory error handler				*
 *									*
 ************************************************************************/
/* *
 * xmlTreeErrMemory:
 * @extra:  extra informations
 *
 * Handle an out of memory condition
 */
unsafe extern "C" fn xmlTreeErrMemory(mut extra: *const std::os::raw::c_char) {
    __xmlSimpleError(XML_FROM_TREE as std::os::raw::c_int,
                     XML_ERR_NO_MEMORY as std::os::raw::c_int, 0 as xmlNodePtr,
                     0 as *const std::os::raw::c_char, extra);
}
/* *
 * xmlTreeErr:
 * @code:  the error number
 * @extra:  extra informations
 *
 * Handle an out of memory condition
 */
unsafe extern "C" fn xmlTreeErr(mut code: std::os::raw::c_int, mut node: xmlNodePtr,
                                mut extra: *const std::os::raw::c_char) {
    let mut msg: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    match code {
        1300 => {
            msg =
                b"invalid hexadecimal character value\n\x00" as *const u8 as
                    *const std::os::raw::c_char
        }
        1301 => {
            msg =
                b"invalid decimal character value\n\x00" as *const u8 as
                    *const std::os::raw::c_char
        }
        1302 => {
            msg =
                b"unterminated entity reference %15s\n\x00" as *const u8 as
                    *const std::os::raw::c_char
        }
        1303 => {
            msg =
                b"string is not in UTF-8\n\x00" as *const u8 as
                    *const std::os::raw::c_char
        }
        _ => {
            msg =
                b"unexpected error number\n\x00" as *const u8 as
                    *const std::os::raw::c_char
        }
    }
    __xmlSimpleError(XML_FROM_TREE as std::os::raw::c_int, code, node, msg, extra);
}
/* ***********************************************************************
 *									*
 *		A few static variables and macros			*
 *									*
 ************************************************************************/
/* #undef xmlStringText */
#[no_mangle]
pub static mut xmlStringText: [xmlChar; 5] =
    ['t' as i32 as xmlChar, 'e' as i32 as xmlChar, 'x' as i32 as xmlChar,
     't' as i32 as xmlChar, 0 as std::os::raw::c_int as xmlChar];
/* #undef xmlStringTextNoenc */
#[no_mangle]
pub static mut xmlStringTextNoenc: [xmlChar; 10] =
    ['t' as i32 as xmlChar, 'e' as i32 as xmlChar, 'x' as i32 as xmlChar,
     't' as i32 as xmlChar, 'n' as i32 as xmlChar, 'o' as i32 as xmlChar,
     'e' as i32 as xmlChar, 'n' as i32 as xmlChar, 'c' as i32 as xmlChar,
     0 as std::os::raw::c_int as xmlChar];
/* #undef xmlStringComment */
#[no_mangle]
pub static mut xmlStringComment: [xmlChar; 8] =
    ['c' as i32 as xmlChar, 'o' as i32 as xmlChar, 'm' as i32 as xmlChar,
     'm' as i32 as xmlChar, 'e' as i32 as xmlChar, 'n' as i32 as xmlChar,
     't' as i32 as xmlChar, 0 as std::os::raw::c_int as xmlChar];
static mut xmlCompressMode: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut xmlCheckDTD: std::os::raw::c_int = 1 as std::os::raw::c_int;
/* #define DEBUG_BUFFER */
/* #define DEBUG_TREE */
/* ***********************************************************************
 *									*
 *		Functions to move to entities.c once the		*
 *		API freeze is smoothen and they can be made public.	*
 *									*
 ************************************************************************/
/* *
 * xmlGetEntityFromDtd:
 * @dtd:  A pointer to the DTD to search
 * @name:  The entity name
 *
 * Do an entity lookup in the DTD entity hash table and
 * return the corresponding entity, if found.
 *
 * Returns A pointer to the entity structure or NULL if not found.
 */
unsafe extern "C" fn xmlGetEntityFromDtd(mut dtd: *const xmlDtd,
                                         mut name: *const xmlChar)
 -> xmlEntityPtr {
    let mut table: xmlEntitiesTablePtr = 0 as *mut xmlEntitiesTable;
    if !dtd.is_null() && !(*dtd).entities.is_null() {
        table = (*dtd).entities as xmlEntitiesTablePtr;
        return xmlHashLookup(table, name) as xmlEntityPtr
        /* return(xmlGetEntityFromTable(table, name)); */
    }
    return 0 as xmlEntityPtr;
}
/* *
 * xmlGetParameterEntityFromDtd:
 * @dtd:  A pointer to the DTD to search
 * @name:  The entity name
 *
 * Do an entity lookup in the DTD pararmeter entity hash table and
 * return the corresponding entity, if found.
 *
 * Returns A pointer to the entity structure or NULL if not found.
 */
unsafe extern "C" fn xmlGetParameterEntityFromDtd(mut dtd: *const xmlDtd,
                                                  mut name: *const xmlChar)
 -> xmlEntityPtr {
    let mut table: xmlEntitiesTablePtr = 0 as *mut xmlEntitiesTable;
    if !dtd.is_null() && !(*dtd).pentities.is_null() {
        table = (*dtd).pentities as xmlEntitiesTablePtr;
        return xmlHashLookup(table, name) as xmlEntityPtr
        /* return(xmlGetEntityFromTable(table, name)); */
    }
    return 0 as xmlEntityPtr;
}
/* LIBXML_TREE_ENABLED */
/* ***********************************************************************
 *									*
 *			QName handling helper				*
 *									*
 ************************************************************************/
/* *
 * xmlBuildQName:
 * @ncname:  the Name
 * @prefix:  the prefix
 * @memory:  preallocated memory
 * @len:  preallocated memory length
 *
 * Builds the QName @prefix:@ncname in @memory if there is enough space
 * and prefix is not NULL nor empty, otherwise allocate a new string.
 * If prefix is NULL or empty it returns ncname.
 *
 * Returns the new string which must be freed by the caller if different from
 *         @memory and @ncname or NULL in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlBuildQName(mut ncname: *const xmlChar,
                                       mut prefix: *const xmlChar,
                                       mut memory: *mut xmlChar,
                                       mut len: std::os::raw::c_int) -> *mut xmlChar {
    let mut lenn: std::os::raw::c_int = 0;
    let mut lenp: std::os::raw::c_int = 0;
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    if ncname.is_null() { return 0 as *mut xmlChar }
    if prefix.is_null() { return ncname as *mut xmlChar }
    lenn = strlen(ncname as *mut std::os::raw::c_char) as std::os::raw::c_int;
    lenp = strlen(prefix as *mut std::os::raw::c_char) as std::os::raw::c_int;
    if memory.is_null() || len < lenn + lenp + 2 as std::os::raw::c_int {
        ret =
            xmlMallocAtomic.expect("non-null function pointer")((lenn + lenp +
                                                                     2 as
                                                                         std::os::raw::c_int)
                                                                    as size_t)
                as *mut xmlChar;
        if ret.is_null() {
            xmlTreeErrMemory(b"building QName\x00" as *const u8 as
                                 *const std::os::raw::c_char);
            return 0 as *mut xmlChar
        }
    } else { ret = memory }
    memcpy(&mut *ret.offset(0 as std::os::raw::c_int as isize) as *mut xmlChar as
               *mut std::os::raw::c_void, prefix as *const std::os::raw::c_void,
           lenp as std::os::raw::c_ulong);
    *ret.offset(lenp as isize) = ':' as i32 as xmlChar;
    memcpy(&mut *ret.offset((lenp + 1 as std::os::raw::c_int) as isize) as
               *mut xmlChar as *mut std::os::raw::c_void,
           ncname as *const std::os::raw::c_void, lenn as std::os::raw::c_ulong);
    *ret.offset((lenn + lenp + 1 as std::os::raw::c_int) as isize) =
        0 as std::os::raw::c_int as xmlChar;
    return ret;
}
/* *
 * xmlSplitQName2:
 * @name:  the full QName
 * @prefix:  a xmlChar **
 *
 * parse an XML qualified name string
 *
 * [NS 5] QName ::= (Prefix ':')? LocalPart
 *
 * [NS 6] Prefix ::= NCName
 *
 * [NS 7] LocalPart ::= NCName
 *
 * Returns NULL if the name doesn't have a prefix. Otherwise, returns the
 * local part, and prefix is updated to get the Prefix. Both the return value
 * and the prefix must be freed by the caller.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSplitQName2(mut name: *const xmlChar,
                                        mut prefix: *mut *mut xmlChar)
 -> *mut xmlChar {
    let mut len: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    if prefix.is_null() { return 0 as *mut xmlChar }
    *prefix = 0 as *mut xmlChar;
    if name.is_null() { return 0 as *mut xmlChar }
    /* nasty but valid */
    if *name.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int == ':' as i32 {
        return 0 as *mut xmlChar
    }
    /*
     * we are not trying to validate but just to cut, and yes it will
     * work even if this is as set of UTF-8 encoded chars
     */
    while *name.offset(len as isize) as std::os::raw::c_int != 0 as std::os::raw::c_int &&
              *name.offset(len as isize) as std::os::raw::c_int != ':' as i32 {
        len += 1
    }
    if *name.offset(len as isize) as std::os::raw::c_int == 0 as std::os::raw::c_int {
        return 0 as *mut xmlChar
    }
    *prefix = xmlStrndup(name, len);
    if (*prefix).is_null() {
        xmlTreeErrMemory(b"QName split\x00" as *const u8 as
                             *const std::os::raw::c_char);
        return 0 as *mut xmlChar
    }
    ret = xmlStrdup(&*name.offset((len + 1 as std::os::raw::c_int) as isize));
    if ret.is_null() {
        xmlTreeErrMemory(b"QName split\x00" as *const u8 as
                             *const std::os::raw::c_char);
        if !(*prefix).is_null() {
            xmlFree.expect("non-null function pointer")(*prefix as
                                                            *mut std::os::raw::c_void);
            *prefix = 0 as *mut xmlChar
        }
        return 0 as *mut xmlChar
    }
    return ret;
}
/* *
 * xmlSplitQName3:
 * @name:  the full QName
 * @len: an int *
 *
 * parse an XML qualified name string,i
 *
 * returns NULL if it is not a Qualified Name, otherwise, update len
 *         with the length in byte of the prefix and return a pointer
 *         to the start of the name without the prefix
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSplitQName3(mut name: *const xmlChar,
                                        mut len: *mut std::os::raw::c_int)
 -> *const xmlChar {
    let mut l: std::os::raw::c_int = 0 as std::os::raw::c_int;
    if name.is_null() { return 0 as *const xmlChar }
    if len.is_null() { return 0 as *const xmlChar }
    /* nasty but valid */
    if *name.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int == ':' as i32 {
        return 0 as *const xmlChar
    }
    /*
     * we are not trying to validate but just to cut, and yes it will
     * work even if this is as set of UTF-8 encoded chars
     */
    while *name.offset(l as isize) as std::os::raw::c_int != 0 as std::os::raw::c_int &&
              *name.offset(l as isize) as std::os::raw::c_int != ':' as i32 {
        l += 1
    }
    if *name.offset(l as isize) as std::os::raw::c_int == 0 as std::os::raw::c_int {
        return 0 as *const xmlChar
    }
    *len = l;
    return &*name.offset((l + 1 as std::os::raw::c_int) as isize) as *const xmlChar;
}
/* *
 * xmlValidateNCName:
 * @value: the value to check
 * @space: allow spaces in front and end of the string
 *
 * Check that a value conforms to the lexical space of NCName
 *
 * Returns 0 if this validates, a positive error code number otherwise
 *         and -1 in case of internal or API error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlValidateNCName(mut value: *const xmlChar,
                                           mut space: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut cur: *const xmlChar = value;
    let mut c: std::os::raw::c_int = 0;
    let mut l: std::os::raw::c_int = 0;
    if value.is_null() { return -(1 as std::os::raw::c_int) }
    /*
     * First quick algorithm for ASCII range
     */
    if space != 0 {
        while *cur as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
                  0x9 as std::os::raw::c_int <= *cur as std::os::raw::c_int &&
                      *cur as std::os::raw::c_int <= 0xa as std::os::raw::c_int ||
                  *cur as std::os::raw::c_int == 0xd as std::os::raw::c_int {
            cur = cur.offset(1)
        }
    }
    if *cur as std::os::raw::c_int >= 'a' as i32 && *cur as std::os::raw::c_int <= 'z' as i32
           ||
           *cur as std::os::raw::c_int >= 'A' as i32 &&
               *cur as std::os::raw::c_int <= 'Z' as i32 ||
           *cur as std::os::raw::c_int == '_' as i32 {
        cur = cur.offset(1);
        while *cur as std::os::raw::c_int >= 'a' as i32 &&
                  *cur as std::os::raw::c_int <= 'z' as i32 ||
                  *cur as std::os::raw::c_int >= 'A' as i32 &&
                      *cur as std::os::raw::c_int <= 'Z' as i32 ||
                  *cur as std::os::raw::c_int >= '0' as i32 &&
                      *cur as std::os::raw::c_int <= '9' as i32 ||
                  *cur as std::os::raw::c_int == '_' as i32 ||
                  *cur as std::os::raw::c_int == '-' as i32 ||
                  *cur as std::os::raw::c_int == '.' as i32 {
            cur = cur.offset(1)
        }
        if space != 0 {
            while *cur as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
                      0x9 as std::os::raw::c_int <= *cur as std::os::raw::c_int &&
                          *cur as std::os::raw::c_int <= 0xa as std::os::raw::c_int ||
                      *cur as std::os::raw::c_int == 0xd as std::os::raw::c_int {
                cur = cur.offset(1)
            }
        }
        if *cur as std::os::raw::c_int == 0 as std::os::raw::c_int { return 0 as std::os::raw::c_int }
    }
    /*
     * Second check for chars outside the ASCII range
     */
    cur = value;
    c = xmlStringCurrentChar(0 as xmlParserCtxtPtr, cur, &mut l);
    if space != 0 {
        while if c < 0x100 as std::os::raw::c_int {
                  (c == 0x20 as std::os::raw::c_int ||
                       0x9 as std::os::raw::c_int <= c && c <= 0xa as std::os::raw::c_int ||
                       c == 0xd as std::os::raw::c_int) as std::os::raw::c_int
              } else { 0 as std::os::raw::c_int } != 0 {
            cur = cur.offset(l as isize);
            c = xmlStringCurrentChar(0 as xmlParserCtxtPtr, cur, &mut l)
        }
    }
    if !((if c < 0x100 as std::os::raw::c_int {
              (0x41 as std::os::raw::c_int <= c && c <= 0x5a as std::os::raw::c_int ||
                   0x61 as std::os::raw::c_int <= c && c <= 0x7a as std::os::raw::c_int ||
                   0xc0 as std::os::raw::c_int <= c && c <= 0xd6 as std::os::raw::c_int ||
                   0xd8 as std::os::raw::c_int <= c && c <= 0xf6 as std::os::raw::c_int ||
                   0xf8 as std::os::raw::c_int <= c) as std::os::raw::c_int
          } else { xmlCharInRange(c as std::os::raw::c_uint, &xmlIsBaseCharGroup) })
             != 0 ||
             (if c < 0x100 as std::os::raw::c_int {
                  0 as std::os::raw::c_int
              } else {
                  (0x4e00 as std::os::raw::c_int <= c && c <= 0x9fa5 as std::os::raw::c_int ||
                       c == 0x3007 as std::os::raw::c_int ||
                       0x3021 as std::os::raw::c_int <= c &&
                           c <= 0x3029 as std::os::raw::c_int) as std::os::raw::c_int
              }) != 0) && c != '_' as i32 {
        return 1 as std::os::raw::c_int
    }
    cur = cur.offset(l as isize);
    c = xmlStringCurrentChar(0 as xmlParserCtxtPtr, cur, &mut l);
    while (if c < 0x100 as std::os::raw::c_int {
               (0x41 as std::os::raw::c_int <= c && c <= 0x5a as std::os::raw::c_int ||
                    0x61 as std::os::raw::c_int <= c && c <= 0x7a as std::os::raw::c_int ||
                    0xc0 as std::os::raw::c_int <= c && c <= 0xd6 as std::os::raw::c_int ||
                    0xd8 as std::os::raw::c_int <= c && c <= 0xf6 as std::os::raw::c_int ||
                    0xf8 as std::os::raw::c_int <= c) as std::os::raw::c_int
           } else { xmlCharInRange(c as std::os::raw::c_uint, &xmlIsBaseCharGroup) })
              != 0 ||
              (if c < 0x100 as std::os::raw::c_int {
                   0 as std::os::raw::c_int
               } else {
                   (0x4e00 as std::os::raw::c_int <= c && c <= 0x9fa5 as std::os::raw::c_int
                        || c == 0x3007 as std::os::raw::c_int ||
                        0x3021 as std::os::raw::c_int <= c &&
                            c <= 0x3029 as std::os::raw::c_int) as std::os::raw::c_int
               }) != 0 ||
              (if c < 0x100 as std::os::raw::c_int {
                   (0x30 as std::os::raw::c_int <= c && c <= 0x39 as std::os::raw::c_int) as
                       std::os::raw::c_int
               } else { xmlCharInRange(c as std::os::raw::c_uint, &xmlIsDigitGroup) })
                  != 0 || c == '.' as i32 || c == '-' as i32 ||
              c == '_' as i32 ||
              (if c < 0x100 as std::os::raw::c_int {
                   0 as std::os::raw::c_int
               } else {
                   xmlCharInRange(c as std::os::raw::c_uint, &xmlIsCombiningGroup)
               }) != 0 ||
              (if c < 0x100 as std::os::raw::c_int {
                   (c == 0xb7 as std::os::raw::c_int) as std::os::raw::c_int
               } else {
                   xmlCharInRange(c as std::os::raw::c_uint, &xmlIsExtenderGroup)
               }) != 0 {
        cur = cur.offset(l as isize);
        c = xmlStringCurrentChar(0 as xmlParserCtxtPtr, cur, &mut l)
    }
    if space != 0 {
        while if c < 0x100 as std::os::raw::c_int {
                  (c == 0x20 as std::os::raw::c_int ||
                       0x9 as std::os::raw::c_int <= c && c <= 0xa as std::os::raw::c_int ||
                       c == 0xd as std::os::raw::c_int) as std::os::raw::c_int
              } else { 0 as std::os::raw::c_int } != 0 {
            cur = cur.offset(l as isize);
            c = xmlStringCurrentChar(0 as xmlParserCtxtPtr, cur, &mut l)
        }
    }
    if c != 0 as std::os::raw::c_int { return 1 as std::os::raw::c_int }
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlValidateQName:
 * @value: the value to check
 * @space: allow spaces in front and end of the string
 *
 * Check that a value conforms to the lexical space of QName
 *
 * Returns 0 if this validates, a positive error code number otherwise
 *         and -1 in case of internal or API error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlValidateQName(mut value: *const xmlChar,
                                          mut space: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut current_block: u64;
    let mut cur: *const xmlChar = value;
    let mut c: std::os::raw::c_int = 0;
    let mut l: std::os::raw::c_int = 0;
    if value.is_null() { return -(1 as std::os::raw::c_int) }
    /*
     * First quick algorithm for ASCII range
     */
    if space != 0 {
        while *cur as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
                  0x9 as std::os::raw::c_int <= *cur as std::os::raw::c_int &&
                      *cur as std::os::raw::c_int <= 0xa as std::os::raw::c_int ||
                  *cur as std::os::raw::c_int == 0xd as std::os::raw::c_int {
            cur = cur.offset(1)
        }
    }
    if *cur as std::os::raw::c_int >= 'a' as i32 && *cur as std::os::raw::c_int <= 'z' as i32
           ||
           *cur as std::os::raw::c_int >= 'A' as i32 &&
               *cur as std::os::raw::c_int <= 'Z' as i32 ||
           *cur as std::os::raw::c_int == '_' as i32 {
        cur = cur.offset(1);
        while *cur as std::os::raw::c_int >= 'a' as i32 &&
                  *cur as std::os::raw::c_int <= 'z' as i32 ||
                  *cur as std::os::raw::c_int >= 'A' as i32 &&
                      *cur as std::os::raw::c_int <= 'Z' as i32 ||
                  *cur as std::os::raw::c_int >= '0' as i32 &&
                      *cur as std::os::raw::c_int <= '9' as i32 ||
                  *cur as std::os::raw::c_int == '_' as i32 ||
                  *cur as std::os::raw::c_int == '-' as i32 ||
                  *cur as std::os::raw::c_int == '.' as i32 {
            cur = cur.offset(1)
        }
        if *cur as std::os::raw::c_int == ':' as i32 {
            cur = cur.offset(1);
            if *cur as std::os::raw::c_int >= 'a' as i32 &&
                   *cur as std::os::raw::c_int <= 'z' as i32 ||
                   *cur as std::os::raw::c_int >= 'A' as i32 &&
                       *cur as std::os::raw::c_int <= 'Z' as i32 ||
                   *cur as std::os::raw::c_int == '_' as i32 {
                cur = cur.offset(1);
                while *cur as std::os::raw::c_int >= 'a' as i32 &&
                          *cur as std::os::raw::c_int <= 'z' as i32 ||
                          *cur as std::os::raw::c_int >= 'A' as i32 &&
                              *cur as std::os::raw::c_int <= 'Z' as i32 ||
                          *cur as std::os::raw::c_int >= '0' as i32 &&
                              *cur as std::os::raw::c_int <= '9' as i32 ||
                          *cur as std::os::raw::c_int == '_' as i32 ||
                          *cur as std::os::raw::c_int == '-' as i32 ||
                          *cur as std::os::raw::c_int == '.' as i32 {
                    cur = cur.offset(1)
                }
                current_block = 1054647088692577877;
            } else { current_block = 10551850636568454277; }
        } else { current_block = 1054647088692577877; }
        match current_block {
            10551850636568454277 => { }
            _ => {
                if space != 0 {
                    while *cur as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
                              0x9 as std::os::raw::c_int <= *cur as std::os::raw::c_int &&
                                  *cur as std::os::raw::c_int <= 0xa as std::os::raw::c_int ||
                              *cur as std::os::raw::c_int == 0xd as std::os::raw::c_int {
                        cur = cur.offset(1)
                    }
                }
                if *cur as std::os::raw::c_int == 0 as std::os::raw::c_int {
                    return 0 as std::os::raw::c_int
                }
            }
        }
    }
    /*
     * Second check for chars outside the ASCII range
     */
    cur = value;
    c = xmlStringCurrentChar(0 as xmlParserCtxtPtr, cur, &mut l);
    if space != 0 {
        while if c < 0x100 as std::os::raw::c_int {
                  (c == 0x20 as std::os::raw::c_int ||
                       0x9 as std::os::raw::c_int <= c && c <= 0xa as std::os::raw::c_int ||
                       c == 0xd as std::os::raw::c_int) as std::os::raw::c_int
              } else { 0 as std::os::raw::c_int } != 0 {
            cur = cur.offset(l as isize);
            c = xmlStringCurrentChar(0 as xmlParserCtxtPtr, cur, &mut l)
        }
    }
    if !((if c < 0x100 as std::os::raw::c_int {
              (0x41 as std::os::raw::c_int <= c && c <= 0x5a as std::os::raw::c_int ||
                   0x61 as std::os::raw::c_int <= c && c <= 0x7a as std::os::raw::c_int ||
                   0xc0 as std::os::raw::c_int <= c && c <= 0xd6 as std::os::raw::c_int ||
                   0xd8 as std::os::raw::c_int <= c && c <= 0xf6 as std::os::raw::c_int ||
                   0xf8 as std::os::raw::c_int <= c) as std::os::raw::c_int
          } else { xmlCharInRange(c as std::os::raw::c_uint, &xmlIsBaseCharGroup) })
             != 0 ||
             (if c < 0x100 as std::os::raw::c_int {
                  0 as std::os::raw::c_int
              } else {
                  (0x4e00 as std::os::raw::c_int <= c && c <= 0x9fa5 as std::os::raw::c_int ||
                       c == 0x3007 as std::os::raw::c_int ||
                       0x3021 as std::os::raw::c_int <= c &&
                           c <= 0x3029 as std::os::raw::c_int) as std::os::raw::c_int
              }) != 0) && c != '_' as i32 {
        return 1 as std::os::raw::c_int
    }
    cur = cur.offset(l as isize);
    c = xmlStringCurrentChar(0 as xmlParserCtxtPtr, cur, &mut l);
    while (if c < 0x100 as std::os::raw::c_int {
               (0x41 as std::os::raw::c_int <= c && c <= 0x5a as std::os::raw::c_int ||
                    0x61 as std::os::raw::c_int <= c && c <= 0x7a as std::os::raw::c_int ||
                    0xc0 as std::os::raw::c_int <= c && c <= 0xd6 as std::os::raw::c_int ||
                    0xd8 as std::os::raw::c_int <= c && c <= 0xf6 as std::os::raw::c_int ||
                    0xf8 as std::os::raw::c_int <= c) as std::os::raw::c_int
           } else { xmlCharInRange(c as std::os::raw::c_uint, &xmlIsBaseCharGroup) })
              != 0 ||
              (if c < 0x100 as std::os::raw::c_int {
                   0 as std::os::raw::c_int
               } else {
                   (0x4e00 as std::os::raw::c_int <= c && c <= 0x9fa5 as std::os::raw::c_int
                        || c == 0x3007 as std::os::raw::c_int ||
                        0x3021 as std::os::raw::c_int <= c &&
                            c <= 0x3029 as std::os::raw::c_int) as std::os::raw::c_int
               }) != 0 ||
              (if c < 0x100 as std::os::raw::c_int {
                   (0x30 as std::os::raw::c_int <= c && c <= 0x39 as std::os::raw::c_int) as
                       std::os::raw::c_int
               } else { xmlCharInRange(c as std::os::raw::c_uint, &xmlIsDigitGroup) })
                  != 0 || c == '.' as i32 || c == '-' as i32 ||
              c == '_' as i32 ||
              (if c < 0x100 as std::os::raw::c_int {
                   0 as std::os::raw::c_int
               } else {
                   xmlCharInRange(c as std::os::raw::c_uint, &xmlIsCombiningGroup)
               }) != 0 ||
              (if c < 0x100 as std::os::raw::c_int {
                   (c == 0xb7 as std::os::raw::c_int) as std::os::raw::c_int
               } else {
                   xmlCharInRange(c as std::os::raw::c_uint, &xmlIsExtenderGroup)
               }) != 0 {
        cur = cur.offset(l as isize);
        c = xmlStringCurrentChar(0 as xmlParserCtxtPtr, cur, &mut l)
    }
    if c == ':' as i32 {
        cur = cur.offset(l as isize);
        c = xmlStringCurrentChar(0 as xmlParserCtxtPtr, cur, &mut l);
        if !((if c < 0x100 as std::os::raw::c_int {
                  (0x41 as std::os::raw::c_int <= c && c <= 0x5a as std::os::raw::c_int ||
                       0x61 as std::os::raw::c_int <= c && c <= 0x7a as std::os::raw::c_int ||
                       0xc0 as std::os::raw::c_int <= c && c <= 0xd6 as std::os::raw::c_int ||
                       0xd8 as std::os::raw::c_int <= c && c <= 0xf6 as std::os::raw::c_int ||
                       0xf8 as std::os::raw::c_int <= c) as std::os::raw::c_int
              } else {
                  xmlCharInRange(c as std::os::raw::c_uint, &xmlIsBaseCharGroup)
              }) != 0 ||
                 (if c < 0x100 as std::os::raw::c_int {
                      0 as std::os::raw::c_int
                  } else {
                      (0x4e00 as std::os::raw::c_int <= c &&
                           c <= 0x9fa5 as std::os::raw::c_int ||
                           c == 0x3007 as std::os::raw::c_int ||
                           0x3021 as std::os::raw::c_int <= c &&
                               c <= 0x3029 as std::os::raw::c_int) as std::os::raw::c_int
                  }) != 0) && c != '_' as i32 {
            return 1 as std::os::raw::c_int
        }
        cur = cur.offset(l as isize);
        c = xmlStringCurrentChar(0 as xmlParserCtxtPtr, cur, &mut l);
        while (if c < 0x100 as std::os::raw::c_int {
                   (0x41 as std::os::raw::c_int <= c && c <= 0x5a as std::os::raw::c_int ||
                        0x61 as std::os::raw::c_int <= c && c <= 0x7a as std::os::raw::c_int
                        ||
                        0xc0 as std::os::raw::c_int <= c && c <= 0xd6 as std::os::raw::c_int
                        ||
                        0xd8 as std::os::raw::c_int <= c && c <= 0xf6 as std::os::raw::c_int
                        || 0xf8 as std::os::raw::c_int <= c) as std::os::raw::c_int
               } else {
                   xmlCharInRange(c as std::os::raw::c_uint, &xmlIsBaseCharGroup)
               }) != 0 ||
                  (if c < 0x100 as std::os::raw::c_int {
                       0 as std::os::raw::c_int
                   } else {
                       (0x4e00 as std::os::raw::c_int <= c &&
                            c <= 0x9fa5 as std::os::raw::c_int ||
                            c == 0x3007 as std::os::raw::c_int ||
                            0x3021 as std::os::raw::c_int <= c &&
                                c <= 0x3029 as std::os::raw::c_int) as std::os::raw::c_int
                   }) != 0 ||
                  (if c < 0x100 as std::os::raw::c_int {
                       (0x30 as std::os::raw::c_int <= c && c <= 0x39 as std::os::raw::c_int)
                           as std::os::raw::c_int
                   } else {
                       xmlCharInRange(c as std::os::raw::c_uint, &xmlIsDigitGroup)
                   }) != 0 || c == '.' as i32 || c == '-' as i32 ||
                  c == '_' as i32 ||
                  (if c < 0x100 as std::os::raw::c_int {
                       0 as std::os::raw::c_int
                   } else {
                       xmlCharInRange(c as std::os::raw::c_uint, &xmlIsCombiningGroup)
                   }) != 0 ||
                  (if c < 0x100 as std::os::raw::c_int {
                       (c == 0xb7 as std::os::raw::c_int) as std::os::raw::c_int
                   } else {
                       xmlCharInRange(c as std::os::raw::c_uint, &xmlIsExtenderGroup)
                   }) != 0 {
            cur = cur.offset(l as isize);
            c = xmlStringCurrentChar(0 as xmlParserCtxtPtr, cur, &mut l)
        }
    }
    if space != 0 {
        while if c < 0x100 as std::os::raw::c_int {
                  (c == 0x20 as std::os::raw::c_int ||
                       0x9 as std::os::raw::c_int <= c && c <= 0xa as std::os::raw::c_int ||
                       c == 0xd as std::os::raw::c_int) as std::os::raw::c_int
              } else { 0 as std::os::raw::c_int } != 0 {
            cur = cur.offset(l as isize);
            c = xmlStringCurrentChar(0 as xmlParserCtxtPtr, cur, &mut l)
        }
    }
    if c != 0 as std::os::raw::c_int { return 1 as std::os::raw::c_int }
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlValidateName:
 * @value: the value to check
 * @space: allow spaces in front and end of the string
 *
 * Check that a value conforms to the lexical space of Name
 *
 * Returns 0 if this validates, a positive error code number otherwise
 *         and -1 in case of internal or API error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlValidateName(mut value: *const xmlChar,
                                         mut space: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut cur: *const xmlChar = value;
    let mut c: std::os::raw::c_int = 0;
    let mut l: std::os::raw::c_int = 0;
    if value.is_null() { return -(1 as std::os::raw::c_int) }
    /*
     * First quick algorithm for ASCII range
     */
    if space != 0 {
        while *cur as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
                  0x9 as std::os::raw::c_int <= *cur as std::os::raw::c_int &&
                      *cur as std::os::raw::c_int <= 0xa as std::os::raw::c_int ||
                  *cur as std::os::raw::c_int == 0xd as std::os::raw::c_int {
            cur = cur.offset(1)
        }
    }
    if *cur as std::os::raw::c_int >= 'a' as i32 && *cur as std::os::raw::c_int <= 'z' as i32
           ||
           *cur as std::os::raw::c_int >= 'A' as i32 &&
               *cur as std::os::raw::c_int <= 'Z' as i32 ||
           *cur as std::os::raw::c_int == '_' as i32 ||
           *cur as std::os::raw::c_int == ':' as i32 {
        cur = cur.offset(1);
        while *cur as std::os::raw::c_int >= 'a' as i32 &&
                  *cur as std::os::raw::c_int <= 'z' as i32 ||
                  *cur as std::os::raw::c_int >= 'A' as i32 &&
                      *cur as std::os::raw::c_int <= 'Z' as i32 ||
                  *cur as std::os::raw::c_int >= '0' as i32 &&
                      *cur as std::os::raw::c_int <= '9' as i32 ||
                  *cur as std::os::raw::c_int == '_' as i32 ||
                  *cur as std::os::raw::c_int == '-' as i32 ||
                  *cur as std::os::raw::c_int == '.' as i32 ||
                  *cur as std::os::raw::c_int == ':' as i32 {
            cur = cur.offset(1)
        }
        if space != 0 {
            while *cur as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
                      0x9 as std::os::raw::c_int <= *cur as std::os::raw::c_int &&
                          *cur as std::os::raw::c_int <= 0xa as std::os::raw::c_int ||
                      *cur as std::os::raw::c_int == 0xd as std::os::raw::c_int {
                cur = cur.offset(1)
            }
        }
        if *cur as std::os::raw::c_int == 0 as std::os::raw::c_int { return 0 as std::os::raw::c_int }
    }
    /*
     * Second check for chars outside the ASCII range
     */
    cur = value;
    c = xmlStringCurrentChar(0 as xmlParserCtxtPtr, cur, &mut l);
    if space != 0 {
        while if c < 0x100 as std::os::raw::c_int {
                  (c == 0x20 as std::os::raw::c_int ||
                       0x9 as std::os::raw::c_int <= c && c <= 0xa as std::os::raw::c_int ||
                       c == 0xd as std::os::raw::c_int) as std::os::raw::c_int
              } else { 0 as std::os::raw::c_int } != 0 {
            cur = cur.offset(l as isize);
            c = xmlStringCurrentChar(0 as xmlParserCtxtPtr, cur, &mut l)
        }
    }
    if !((if c < 0x100 as std::os::raw::c_int {
              (0x41 as std::os::raw::c_int <= c && c <= 0x5a as std::os::raw::c_int ||
                   0x61 as std::os::raw::c_int <= c && c <= 0x7a as std::os::raw::c_int ||
                   0xc0 as std::os::raw::c_int <= c && c <= 0xd6 as std::os::raw::c_int ||
                   0xd8 as std::os::raw::c_int <= c && c <= 0xf6 as std::os::raw::c_int ||
                   0xf8 as std::os::raw::c_int <= c) as std::os::raw::c_int
          } else { xmlCharInRange(c as std::os::raw::c_uint, &xmlIsBaseCharGroup) })
             != 0 ||
             (if c < 0x100 as std::os::raw::c_int {
                  0 as std::os::raw::c_int
              } else {
                  (0x4e00 as std::os::raw::c_int <= c && c <= 0x9fa5 as std::os::raw::c_int ||
                       c == 0x3007 as std::os::raw::c_int ||
                       0x3021 as std::os::raw::c_int <= c &&
                           c <= 0x3029 as std::os::raw::c_int) as std::os::raw::c_int
              }) != 0) && c != '_' as i32 && c != ':' as i32 {
        return 1 as std::os::raw::c_int
    }
    cur = cur.offset(l as isize);
    c = xmlStringCurrentChar(0 as xmlParserCtxtPtr, cur, &mut l);
    while (if c < 0x100 as std::os::raw::c_int {
               (0x41 as std::os::raw::c_int <= c && c <= 0x5a as std::os::raw::c_int ||
                    0x61 as std::os::raw::c_int <= c && c <= 0x7a as std::os::raw::c_int ||
                    0xc0 as std::os::raw::c_int <= c && c <= 0xd6 as std::os::raw::c_int ||
                    0xd8 as std::os::raw::c_int <= c && c <= 0xf6 as std::os::raw::c_int ||
                    0xf8 as std::os::raw::c_int <= c) as std::os::raw::c_int
           } else { xmlCharInRange(c as std::os::raw::c_uint, &xmlIsBaseCharGroup) })
              != 0 ||
              (if c < 0x100 as std::os::raw::c_int {
                   0 as std::os::raw::c_int
               } else {
                   (0x4e00 as std::os::raw::c_int <= c && c <= 0x9fa5 as std::os::raw::c_int
                        || c == 0x3007 as std::os::raw::c_int ||
                        0x3021 as std::os::raw::c_int <= c &&
                            c <= 0x3029 as std::os::raw::c_int) as std::os::raw::c_int
               }) != 0 ||
              (if c < 0x100 as std::os::raw::c_int {
                   (0x30 as std::os::raw::c_int <= c && c <= 0x39 as std::os::raw::c_int) as
                       std::os::raw::c_int
               } else { xmlCharInRange(c as std::os::raw::c_uint, &xmlIsDigitGroup) })
                  != 0 || c == '.' as i32 || c == ':' as i32 ||
              c == '-' as i32 || c == '_' as i32 ||
              (if c < 0x100 as std::os::raw::c_int {
                   0 as std::os::raw::c_int
               } else {
                   xmlCharInRange(c as std::os::raw::c_uint, &xmlIsCombiningGroup)
               }) != 0 ||
              (if c < 0x100 as std::os::raw::c_int {
                   (c == 0xb7 as std::os::raw::c_int) as std::os::raw::c_int
               } else {
                   xmlCharInRange(c as std::os::raw::c_uint, &xmlIsExtenderGroup)
               }) != 0 {
        cur = cur.offset(l as isize);
        c = xmlStringCurrentChar(0 as xmlParserCtxtPtr, cur, &mut l)
    }
    if space != 0 {
        while if c < 0x100 as std::os::raw::c_int {
                  (c == 0x20 as std::os::raw::c_int ||
                       0x9 as std::os::raw::c_int <= c && c <= 0xa as std::os::raw::c_int ||
                       c == 0xd as std::os::raw::c_int) as std::os::raw::c_int
              } else { 0 as std::os::raw::c_int } != 0 {
            cur = cur.offset(l as isize);
            c = xmlStringCurrentChar(0 as xmlParserCtxtPtr, cur, &mut l)
        }
    }
    if c != 0 as std::os::raw::c_int { return 1 as std::os::raw::c_int }
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlValidateNMToken:
 * @value: the value to check
 * @space: allow spaces in front and end of the string
 *
 * Check that a value conforms to the lexical space of NMToken
 *
 * Returns 0 if this validates, a positive error code number otherwise
 *         and -1 in case of internal or API error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlValidateNMToken(mut value: *const xmlChar,
                                            mut space: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut cur: *const xmlChar = value;
    let mut c: std::os::raw::c_int = 0;
    let mut l: std::os::raw::c_int = 0;
    if value.is_null() { return -(1 as std::os::raw::c_int) }
    /*
     * First quick algorithm for ASCII range
     */
    if space != 0 {
        while *cur as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
                  0x9 as std::os::raw::c_int <= *cur as std::os::raw::c_int &&
                      *cur as std::os::raw::c_int <= 0xa as std::os::raw::c_int ||
                  *cur as std::os::raw::c_int == 0xd as std::os::raw::c_int {
            cur = cur.offset(1)
        }
    }
    if *cur as std::os::raw::c_int >= 'a' as i32 && *cur as std::os::raw::c_int <= 'z' as i32
           ||
           *cur as std::os::raw::c_int >= 'A' as i32 &&
               *cur as std::os::raw::c_int <= 'Z' as i32 ||
           *cur as std::os::raw::c_int >= '0' as i32 &&
               *cur as std::os::raw::c_int <= '9' as i32 ||
           *cur as std::os::raw::c_int == '_' as i32 ||
           *cur as std::os::raw::c_int == '-' as i32 ||
           *cur as std::os::raw::c_int == '.' as i32 ||
           *cur as std::os::raw::c_int == ':' as i32 {
        cur = cur.offset(1);
        while *cur as std::os::raw::c_int >= 'a' as i32 &&
                  *cur as std::os::raw::c_int <= 'z' as i32 ||
                  *cur as std::os::raw::c_int >= 'A' as i32 &&
                      *cur as std::os::raw::c_int <= 'Z' as i32 ||
                  *cur as std::os::raw::c_int >= '0' as i32 &&
                      *cur as std::os::raw::c_int <= '9' as i32 ||
                  *cur as std::os::raw::c_int == '_' as i32 ||
                  *cur as std::os::raw::c_int == '-' as i32 ||
                  *cur as std::os::raw::c_int == '.' as i32 ||
                  *cur as std::os::raw::c_int == ':' as i32 {
            cur = cur.offset(1)
        }
        if space != 0 {
            while *cur as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
                      0x9 as std::os::raw::c_int <= *cur as std::os::raw::c_int &&
                          *cur as std::os::raw::c_int <= 0xa as std::os::raw::c_int ||
                      *cur as std::os::raw::c_int == 0xd as std::os::raw::c_int {
                cur = cur.offset(1)
            }
        }
        if *cur as std::os::raw::c_int == 0 as std::os::raw::c_int { return 0 as std::os::raw::c_int }
    }
    /*
     * Second check for chars outside the ASCII range
     */
    cur = value;
    c = xmlStringCurrentChar(0 as xmlParserCtxtPtr, cur, &mut l);
    if space != 0 {
        while if c < 0x100 as std::os::raw::c_int {
                  (c == 0x20 as std::os::raw::c_int ||
                       0x9 as std::os::raw::c_int <= c && c <= 0xa as std::os::raw::c_int ||
                       c == 0xd as std::os::raw::c_int) as std::os::raw::c_int
              } else { 0 as std::os::raw::c_int } != 0 {
            cur = cur.offset(l as isize);
            c = xmlStringCurrentChar(0 as xmlParserCtxtPtr, cur, &mut l)
        }
    }
    if !((if c < 0x100 as std::os::raw::c_int {
              (0x41 as std::os::raw::c_int <= c && c <= 0x5a as std::os::raw::c_int ||
                   0x61 as std::os::raw::c_int <= c && c <= 0x7a as std::os::raw::c_int ||
                   0xc0 as std::os::raw::c_int <= c && c <= 0xd6 as std::os::raw::c_int ||
                   0xd8 as std::os::raw::c_int <= c && c <= 0xf6 as std::os::raw::c_int ||
                   0xf8 as std::os::raw::c_int <= c) as std::os::raw::c_int
          } else { xmlCharInRange(c as std::os::raw::c_uint, &xmlIsBaseCharGroup) })
             != 0 ||
             (if c < 0x100 as std::os::raw::c_int {
                  0 as std::os::raw::c_int
              } else {
                  (0x4e00 as std::os::raw::c_int <= c && c <= 0x9fa5 as std::os::raw::c_int ||
                       c == 0x3007 as std::os::raw::c_int ||
                       0x3021 as std::os::raw::c_int <= c &&
                           c <= 0x3029 as std::os::raw::c_int) as std::os::raw::c_int
              }) != 0 ||
             (if c < 0x100 as std::os::raw::c_int {
                  (0x30 as std::os::raw::c_int <= c && c <= 0x39 as std::os::raw::c_int) as
                      std::os::raw::c_int
              } else { xmlCharInRange(c as std::os::raw::c_uint, &xmlIsDigitGroup) })
                 != 0 || c == '.' as i32 || c == ':' as i32 || c == '-' as i32
             || c == '_' as i32 ||
             (if c < 0x100 as std::os::raw::c_int {
                  0 as std::os::raw::c_int
              } else {
                  xmlCharInRange(c as std::os::raw::c_uint, &xmlIsCombiningGroup)
              }) != 0 ||
             (if c < 0x100 as std::os::raw::c_int {
                  (c == 0xb7 as std::os::raw::c_int) as std::os::raw::c_int
              } else {
                  xmlCharInRange(c as std::os::raw::c_uint, &xmlIsExtenderGroup)
              }) != 0) {
        return 1 as std::os::raw::c_int
    }
    cur = cur.offset(l as isize);
    c = xmlStringCurrentChar(0 as xmlParserCtxtPtr, cur, &mut l);
    while (if c < 0x100 as std::os::raw::c_int {
               (0x41 as std::os::raw::c_int <= c && c <= 0x5a as std::os::raw::c_int ||
                    0x61 as std::os::raw::c_int <= c && c <= 0x7a as std::os::raw::c_int ||
                    0xc0 as std::os::raw::c_int <= c && c <= 0xd6 as std::os::raw::c_int ||
                    0xd8 as std::os::raw::c_int <= c && c <= 0xf6 as std::os::raw::c_int ||
                    0xf8 as std::os::raw::c_int <= c) as std::os::raw::c_int
           } else { xmlCharInRange(c as std::os::raw::c_uint, &xmlIsBaseCharGroup) })
              != 0 ||
              (if c < 0x100 as std::os::raw::c_int {
                   0 as std::os::raw::c_int
               } else {
                   (0x4e00 as std::os::raw::c_int <= c && c <= 0x9fa5 as std::os::raw::c_int
                        || c == 0x3007 as std::os::raw::c_int ||
                        0x3021 as std::os::raw::c_int <= c &&
                            c <= 0x3029 as std::os::raw::c_int) as std::os::raw::c_int
               }) != 0 ||
              (if c < 0x100 as std::os::raw::c_int {
                   (0x30 as std::os::raw::c_int <= c && c <= 0x39 as std::os::raw::c_int) as
                       std::os::raw::c_int
               } else { xmlCharInRange(c as std::os::raw::c_uint, &xmlIsDigitGroup) })
                  != 0 || c == '.' as i32 || c == ':' as i32 ||
              c == '-' as i32 || c == '_' as i32 ||
              (if c < 0x100 as std::os::raw::c_int {
                   0 as std::os::raw::c_int
               } else {
                   xmlCharInRange(c as std::os::raw::c_uint, &xmlIsCombiningGroup)
               }) != 0 ||
              (if c < 0x100 as std::os::raw::c_int {
                   (c == 0xb7 as std::os::raw::c_int) as std::os::raw::c_int
               } else {
                   xmlCharInRange(c as std::os::raw::c_uint, &xmlIsExtenderGroup)
               }) != 0 {
        cur = cur.offset(l as isize);
        c = xmlStringCurrentChar(0 as xmlParserCtxtPtr, cur, &mut l)
    }
    if space != 0 {
        while if c < 0x100 as std::os::raw::c_int {
                  (c == 0x20 as std::os::raw::c_int ||
                       0x9 as std::os::raw::c_int <= c && c <= 0xa as std::os::raw::c_int ||
                       c == 0xd as std::os::raw::c_int) as std::os::raw::c_int
              } else { 0 as std::os::raw::c_int } != 0 {
            cur = cur.offset(l as isize);
            c = xmlStringCurrentChar(0 as xmlParserCtxtPtr, cur, &mut l)
        }
    }
    if c != 0 as std::os::raw::c_int { return 1 as std::os::raw::c_int }
    return 0 as std::os::raw::c_int;
}
/* LIBXML_TREE_ENABLED */
/* ***********************************************************************
 *									*
 *		Allocation and deallocation of basic structures		*
 *									*
 ************************************************************************/
/* *
 * xmlSetBufferAllocationScheme:
 * @scheme:  allocation method to use
 *
 * Set the buffer allocation method.  Types are
 * XML_BUFFER_ALLOC_EXACT - use exact sizes, keeps memory usage down
 * XML_BUFFER_ALLOC_DOUBLEIT - double buffer when extra needed,
 *                             improves performance
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSetBufferAllocationScheme(mut scheme:
                                                          xmlBufferAllocationScheme) {
    if scheme as std::os::raw::c_uint ==
           XML_BUFFER_ALLOC_EXACT as std::os::raw::c_int as std::os::raw::c_uint ||
           scheme as std::os::raw::c_uint ==
               XML_BUFFER_ALLOC_DOUBLEIT as std::os::raw::c_int as std::os::raw::c_uint ||
           scheme as std::os::raw::c_uint ==
               XML_BUFFER_ALLOC_HYBRID as std::os::raw::c_int as std::os::raw::c_uint {
        *__xmlBufferAllocScheme() = scheme
    };
}
/* *
 * xmlGetBufferAllocationScheme:
 *
 * Types are
 * XML_BUFFER_ALLOC_EXACT - use exact sizes, keeps memory usage down
 * XML_BUFFER_ALLOC_DOUBLEIT - double buffer when extra needed,
 *                             improves performance
 * XML_BUFFER_ALLOC_HYBRID - use exact sizes on small strings to keep memory usage tight
 *                            in normal usage, and doubleit on large strings to avoid
 *                            pathological performance.
 *
 * Returns the current allocation scheme
 */
#[no_mangle]
pub unsafe extern "C" fn xmlGetBufferAllocationScheme()
 -> xmlBufferAllocationScheme {
    return *__xmlBufferAllocScheme();
}
/* *
 * xmlNewNs:
 * @node:  the element carrying the namespace
 * @href:  the URI associated
 * @prefix:  the prefix for the namespace
 *
 * Creation of a new Namespace. This function will refuse to create
 * a namespace with a similar prefix than an existing one present on this
 * node.
 * Note that for a default namespace, @prefix should be NULL.
 *
 * We use href==NULL in the case of an element creation where the namespace
 * was not defined.
 *
 * Returns a new namespace pointer or NULL
 */
#[no_mangle]
pub unsafe extern "C" fn xmlNewNs(mut node: xmlNodePtr,
                                  mut href: *const xmlChar,
                                  mut prefix: *const xmlChar) -> xmlNsPtr {
    let mut cur: xmlNsPtr = 0 as *mut xmlNs;
    if !node.is_null() &&
           (*node).type_0 as std::os::raw::c_uint !=
               XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        return 0 as xmlNsPtr
    }
    if !prefix.is_null() &&
           xmlStrEqual(prefix,
                       b"xml\x00" as *const u8 as *const std::os::raw::c_char as
                           *mut xmlChar) != 0 {
        /* xml namespace is predefined, no need to add it */
        if xmlStrEqual(href,
                       b"http://www.w3.org/XML/1998/namespace\x00" as
                           *const u8 as *const std::os::raw::c_char as *const xmlChar)
               != 0 {
            return 0 as xmlNsPtr
        }
        /*
         * Problem, this is an attempt to bind xml prefix to a wrong
         * namespace, which breaks
         * Namespace constraint: Reserved Prefixes and Namespace Names
         * from XML namespace. But documents authors may not care in
         * their context so let's proceed.
         */
    }
    /*
     * Allocate a new Namespace and fill the fields.
     */
    cur =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlNs>()
                                                          as std::os::raw::c_ulong) as
            xmlNsPtr;
    if cur.is_null() {
        xmlTreeErrMemory(b"building namespace\x00" as *const u8 as
                             *const std::os::raw::c_char);
        return 0 as xmlNsPtr
    }
    memset(cur as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<xmlNs>() as std::os::raw::c_ulong);
    (*cur).type_0 = XML_NAMESPACE_DECL;
    if !href.is_null() { (*cur).href = xmlStrdup(href) }
    if !prefix.is_null() { (*cur).prefix = xmlStrdup(prefix) }
    /*
     * Add it at the end to preserve parsing order ...
     * and checks for existing use of the prefix
     */
    if !node.is_null() {
        if (*node).nsDef.is_null() {
            (*node).nsDef = cur
        } else {
            let mut prev: xmlNsPtr = (*node).nsDef;
            if (*prev).prefix.is_null() && (*cur).prefix.is_null() ||
                   xmlStrEqual((*prev).prefix, (*cur).prefix) != 0 {
                xmlFreeNs(cur);
                return 0 as xmlNsPtr
            }
            while !(*prev).next.is_null() {
                prev = (*prev).next;
                if (*prev).prefix.is_null() && (*cur).prefix.is_null() ||
                       xmlStrEqual((*prev).prefix, (*cur).prefix) != 0 {
                    xmlFreeNs(cur);
                    return 0 as xmlNsPtr
                }
            }
            (*prev).next = cur
        }
    }
    return cur;
}
/* *
 * xmlSetNs:
 * @node:  a node in the document
 * @ns:  a namespace pointer
 *
 * Associate a namespace to a node, a posteriori.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSetNs(mut node: xmlNodePtr, mut ns: xmlNsPtr) {
    if node.is_null() { return }
    if (*node).type_0 as std::os::raw::c_uint ==
           XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint ||
           (*node).type_0 as std::os::raw::c_uint ==
               XML_ATTRIBUTE_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        (*node).ns = ns
    };
}
/* *
 * xmlFreeNs:
 * @cur:  the namespace pointer
 *
 * Free up the structures associated to a namespace
 */
#[no_mangle]
pub unsafe extern "C" fn xmlFreeNs(mut cur: xmlNsPtr) {
    if cur.is_null() { return }
    if !(*cur).href.is_null() {
        xmlFree.expect("non-null function pointer")((*cur).href as
                                                        *mut std::os::raw::c_char as
                                                        *mut std::os::raw::c_void);
    }
    if !(*cur).prefix.is_null() {
        xmlFree.expect("non-null function pointer")((*cur).prefix as
                                                        *mut std::os::raw::c_char as
                                                        *mut std::os::raw::c_void);
    }
    xmlFree.expect("non-null function pointer")(cur as *mut std::os::raw::c_void);
}
/* *
 * xmlFreeNsList:
 * @cur:  the first namespace pointer
 *
 * Free up all the structures associated to the chained namespaces.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlFreeNsList(mut cur: xmlNsPtr) {
    let mut next: xmlNsPtr = 0 as *mut xmlNs;
    if cur.is_null() { return }
    while !cur.is_null() { next = (*cur).next; xmlFreeNs(cur); cur = next };
}
/* *
 * xmlNewDtd:
 * @doc:  the document pointer
 * @name:  the DTD name
 * @ExternalID:  the external ID
 * @SystemID:  the system ID
 *
 * Creation of a new DTD for the external subset. To create an
 * internal subset, use xmlCreateIntSubset().
 *
 * Returns a pointer to the new DTD structure
 */
#[no_mangle]
pub unsafe extern "C" fn xmlNewDtd(mut doc: xmlDocPtr,
                                   mut name: *const xmlChar,
                                   mut ExternalID: *const xmlChar,
                                   mut SystemID: *const xmlChar)
 -> xmlDtdPtr {
    let mut cur: xmlDtdPtr = 0 as *mut xmlDtd;
    if !doc.is_null() && !(*doc).extSubset.is_null() { return 0 as xmlDtdPtr }
    /*
     * Allocate a new DTD and fill the fields.
     */
    cur =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlDtd>()
                                                          as std::os::raw::c_ulong) as
            xmlDtdPtr;
    if cur.is_null() {
        xmlTreeErrMemory(b"building DTD\x00" as *const u8 as
                             *const std::os::raw::c_char);
        return 0 as xmlDtdPtr
    }
    memset(cur as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<xmlDtd>() as std::os::raw::c_ulong);
    (*cur).type_0 = XML_DTD_NODE;
    if !name.is_null() { (*cur).name = xmlStrdup(name) }
    if !ExternalID.is_null() { (*cur).ExternalID = xmlStrdup(ExternalID) }
    if !SystemID.is_null() { (*cur).SystemID = xmlStrdup(SystemID) }
    if !doc.is_null() { (*doc).extSubset = cur }
    (*cur).doc = doc;
    if __xmlRegisterCallbacks != 0 &&
           (*__xmlRegisterNodeDefaultValue()).is_some() {
        (*__xmlRegisterNodeDefaultValue()).expect("non-null function pointer")(cur
                                                                                   as
                                                                                   xmlNodePtr);
    }
    return cur;
}
/* *
 * xmlGetIntSubset:
 * @doc:  the document pointer
 *
 * Get the internal subset of a document
 * Returns a pointer to the DTD structure or NULL if not found
 */
#[no_mangle]
pub unsafe extern "C" fn xmlGetIntSubset(mut doc: *const xmlDoc)
 -> xmlDtdPtr {
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    if doc.is_null() { return 0 as xmlDtdPtr }
    cur = (*doc).children;
    while !cur.is_null() {
        if (*cur).type_0 as std::os::raw::c_uint ==
               XML_DTD_NODE as std::os::raw::c_int as std::os::raw::c_uint {
            return cur as xmlDtdPtr
        }
        cur = (*cur).next
    }
    return (*doc).intSubset as xmlDtdPtr;
}
/* *
 * xmlCreateIntSubset:
 * @doc:  the document pointer
 * @name:  the DTD name
 * @ExternalID:  the external (PUBLIC) ID
 * @SystemID:  the system ID
 *
 * Create the internal subset of a document
 * Returns a pointer to the new DTD structure
 */
#[no_mangle]
pub unsafe extern "C" fn xmlCreateIntSubset(mut doc: xmlDocPtr,
                                            mut name: *const xmlChar,
                                            mut ExternalID: *const xmlChar,
                                            mut SystemID: *const xmlChar)
 -> xmlDtdPtr {
    let mut cur: xmlDtdPtr = 0 as *mut xmlDtd;
    if !doc.is_null() && !xmlGetIntSubset(doc as *const xmlDoc).is_null() {
        return 0 as xmlDtdPtr
    }
    /*
     * Allocate a new DTD and fill the fields.
     */
    cur =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlDtd>()
                                                          as std::os::raw::c_ulong) as
            xmlDtdPtr;
    if cur.is_null() {
        xmlTreeErrMemory(b"building internal subset\x00" as *const u8 as
                             *const std::os::raw::c_char);
        return 0 as xmlDtdPtr
    }
    memset(cur as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<xmlDtd>() as std::os::raw::c_ulong);
    (*cur).type_0 = XML_DTD_NODE;
    if !name.is_null() {
        (*cur).name = xmlStrdup(name);
        if (*cur).name.is_null() {
            xmlTreeErrMemory(b"building internal subset\x00" as *const u8 as
                                 *const std::os::raw::c_char);
            xmlFree.expect("non-null function pointer")(cur as
                                                            *mut std::os::raw::c_void);
            return 0 as xmlDtdPtr
        }
    }
    if !ExternalID.is_null() {
        (*cur).ExternalID = xmlStrdup(ExternalID);
        if (*cur).ExternalID.is_null() {
            xmlTreeErrMemory(b"building internal subset\x00" as *const u8 as
                                 *const std::os::raw::c_char);
            if !(*cur).name.is_null() {
                xmlFree.expect("non-null function pointer")((*cur).name as
                                                                *mut std::os::raw::c_char
                                                                as
                                                                *mut std::os::raw::c_void);
            }
            xmlFree.expect("non-null function pointer")(cur as
                                                            *mut std::os::raw::c_void);
            return 0 as xmlDtdPtr
        }
    }
    if !SystemID.is_null() {
        (*cur).SystemID = xmlStrdup(SystemID);
        if (*cur).SystemID.is_null() {
            xmlTreeErrMemory(b"building internal subset\x00" as *const u8 as
                                 *const std::os::raw::c_char);
            if !(*cur).name.is_null() {
                xmlFree.expect("non-null function pointer")((*cur).name as
                                                                *mut std::os::raw::c_char
                                                                as
                                                                *mut std::os::raw::c_void);
            }
            if !(*cur).ExternalID.is_null() {
                xmlFree.expect("non-null function pointer")((*cur).ExternalID
                                                                as
                                                                *mut std::os::raw::c_char
                                                                as
                                                                *mut std::os::raw::c_void);
            }
            xmlFree.expect("non-null function pointer")(cur as
                                                            *mut std::os::raw::c_void);
            return 0 as xmlDtdPtr
        }
    }
    if !doc.is_null() {
        (*doc).intSubset = cur;
        (*cur).parent = doc;
        (*cur).doc = doc;
        if (*doc).children.is_null() {
            (*doc).children = cur as xmlNodePtr;
            (*doc).last = cur as xmlNodePtr
        } else if (*doc).type_0 as std::os::raw::c_uint ==
                      XML_HTML_DOCUMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
            let mut prev: xmlNodePtr = 0 as *mut xmlNode;
            prev = (*doc).children;
            (*prev).prev = cur as xmlNodePtr;
            (*cur).next = prev;
            (*doc).children = cur as xmlNodePtr
        } else {
            let mut next: xmlNodePtr = 0 as *mut xmlNode;
            next = (*doc).children;
            while !next.is_null() &&
                      (*next).type_0 as std::os::raw::c_uint !=
                          XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
                next = (*next).next
            }
            if next.is_null() {
                (*cur).prev = (*doc).last;
                (*(*cur).prev).next = cur as xmlNodePtr;
                (*cur).next = 0 as *mut _xmlNode;
                (*doc).last = cur as xmlNodePtr
            } else {
                (*cur).next = next;
                (*cur).prev = (*next).prev;
                if (*cur).prev.is_null() {
                    (*doc).children = cur as xmlNodePtr
                } else { (*(*cur).prev).next = cur as xmlNodePtr }
                (*next).prev = cur as xmlNodePtr
            }
        }
    }
    if __xmlRegisterCallbacks != 0 &&
           (*__xmlRegisterNodeDefaultValue()).is_some() {
        (*__xmlRegisterNodeDefaultValue()).expect("non-null function pointer")(cur
                                                                                   as
                                                                                   xmlNodePtr);
    }
    return cur;
}
/* *
 * DICT_FREE:
 * @str:  a string
 *
 * Free a string if it is not owned by the "dict" dictionary in the
 * current scope
 */
/* *
 * DICT_COPY:
 * @str:  a string
 *
 * Copy a string using a "dict" dictionary in the current scope,
 * if availabe.
 */
/* *
 * DICT_CONST_COPY:
 * @str:  a string
 *
 * Copy a string using a "dict" dictionary in the current scope,
 * if availabe.
 */
/* *
 * xmlFreeDtd:
 * @cur:  the DTD structure to free up
 *
 * Free a DTD structure.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlFreeDtd(mut cur: xmlDtdPtr) {
    let mut dict: xmlDictPtr = 0 as xmlDictPtr;
    if cur.is_null() { return }
    if !(*cur).doc.is_null() { dict = (*(*cur).doc).dict }
    if __xmlRegisterCallbacks != 0 &&
           (*__xmlDeregisterNodeDefaultValue()).is_some() {
        (*__xmlDeregisterNodeDefaultValue()).expect("non-null function pointer")(cur
                                                                                     as
                                                                                     xmlNodePtr);
    }
    if !(*cur).children.is_null() {
        let mut next: xmlNodePtr = 0 as *mut xmlNode;
        let mut c: xmlNodePtr = (*cur).children;
        /*
	 * Cleanup all nodes which are not part of the specific lists
	 * of notations, elements, attributes and entities.
	 */
        while !c.is_null() {
            next = (*c).next;
            if (*c).type_0 as std::os::raw::c_uint !=
                   XML_NOTATION_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
                   (*c).type_0 as std::os::raw::c_uint !=
                       XML_ELEMENT_DECL as std::os::raw::c_int as std::os::raw::c_uint &&
                   (*c).type_0 as std::os::raw::c_uint !=
                       XML_ATTRIBUTE_DECL as std::os::raw::c_int as std::os::raw::c_uint &&
                   (*c).type_0 as std::os::raw::c_uint !=
                       XML_ENTITY_DECL as std::os::raw::c_int as std::os::raw::c_uint {
                xmlUnlinkNode(c);
                xmlFreeNode(c);
            }
            c = next
        }
    }
    if !(*cur).name.is_null() &&
           (dict.is_null() ||
                xmlDictOwns(dict, (*cur).name) == 0 as std::os::raw::c_int) {
        xmlFree.expect("non-null function pointer")((*cur).name as
                                                        *mut std::os::raw::c_char as
                                                        *mut std::os::raw::c_void);
    }
    if !(*cur).SystemID.is_null() &&
           (dict.is_null() ||
                xmlDictOwns(dict, (*cur).SystemID) == 0 as std::os::raw::c_int) {
        xmlFree.expect("non-null function pointer")((*cur).SystemID as
                                                        *mut std::os::raw::c_char as
                                                        *mut std::os::raw::c_void);
    }
    if !(*cur).ExternalID.is_null() &&
           (dict.is_null() ||
                xmlDictOwns(dict, (*cur).ExternalID) == 0 as std::os::raw::c_int) {
        xmlFree.expect("non-null function pointer")((*cur).ExternalID as
                                                        *mut std::os::raw::c_char as
                                                        *mut std::os::raw::c_void);
    }
    /* TODO !!! */
    if !(*cur).notations.is_null() {
        xmlFreeNotationTable((*cur).notations as xmlNotationTablePtr);
    }
    if !(*cur).elements.is_null() {
        xmlFreeElementTable((*cur).elements as xmlElementTablePtr);
    }
    if !(*cur).attributes.is_null() {
        xmlFreeAttributeTable((*cur).attributes as xmlAttributeTablePtr);
    }
    if !(*cur).entities.is_null() {
        xmlFreeEntitiesTable((*cur).entities as xmlEntitiesTablePtr);
    }
    if !(*cur).pentities.is_null() {
        xmlFreeEntitiesTable((*cur).pentities as xmlEntitiesTablePtr);
    }
    xmlFree.expect("non-null function pointer")(cur as *mut std::os::raw::c_void);
}
/* *
 * xmlNewDoc:
 * @version:  xmlChar string giving the version of XML "1.0"
 *
 * Creates a new XML document
 *
 * Returns a new document
 */
#[no_mangle]
pub unsafe extern "C" fn xmlNewDoc(mut version: *const xmlChar) -> xmlDocPtr {
    let mut cur: xmlDocPtr = 0 as *mut xmlDoc;
    if version.is_null() {
        version =
            b"1.0\x00" as *const u8 as *const std::os::raw::c_char as *const xmlChar
    }
    /*
     * Allocate a new document and fill the fields.
     */
    cur =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlDoc>()
                                                          as std::os::raw::c_ulong) as
            xmlDocPtr; /* not initialized */
    if cur.is_null() {
        xmlTreeErrMemory(b"building doc\x00" as *const u8 as
                             *const std::os::raw::c_char);
        return 0 as xmlDocPtr
    }
    memset(cur as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<xmlDoc>() as std::os::raw::c_ulong);
    (*cur).type_0 = XML_DOCUMENT_NODE;
    (*cur).version = xmlStrdup(version);
    if (*cur).version.is_null() {
        xmlTreeErrMemory(b"building doc\x00" as *const u8 as
                             *const std::os::raw::c_char);
        xmlFree.expect("non-null function pointer")(cur as *mut std::os::raw::c_void);
        return 0 as xmlDocPtr
    }
    (*cur).standalone = -(1 as std::os::raw::c_int);
    (*cur).compression = -(1 as std::os::raw::c_int);
    (*cur).doc = cur;
    (*cur).parseFlags = 0 as std::os::raw::c_int;
    (*cur).properties = XML_DOC_USERBUILT as std::os::raw::c_int;
    /*
     * The in memory encoding is always UTF8
     * This field will never change and would
     * be obsolete if not for binary compatibility.
     */
    (*cur).charset = XML_CHAR_ENCODING_UTF8 as std::os::raw::c_int;
    if __xmlRegisterCallbacks != 0 &&
           (*__xmlRegisterNodeDefaultValue()).is_some() {
        (*__xmlRegisterNodeDefaultValue()).expect("non-null function pointer")(cur
                                                                                   as
                                                                                   xmlNodePtr);
    }
    return cur;
}
/* *
 * xmlFreeDoc:
 * @cur:  pointer to the document
 *
 * Free up all the structures used by a document, tree included.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlFreeDoc(mut cur: xmlDocPtr) {
    let mut extSubset: xmlDtdPtr = 0 as *mut xmlDtd;
    let mut intSubset: xmlDtdPtr = 0 as *mut xmlDtd;
    let mut dict: xmlDictPtr = 0 as xmlDictPtr;
    if cur.is_null() { return }
    if !cur.is_null() { dict = (*cur).dict }
    if __xmlRegisterCallbacks != 0 &&
           (*__xmlDeregisterNodeDefaultValue()).is_some() {
        (*__xmlDeregisterNodeDefaultValue()).expect("non-null function pointer")(cur
                                                                                     as
                                                                                     xmlNodePtr);
    }
    /*
     * Do this before freeing the children list to avoid ID lookups
     */
    if !(*cur).ids.is_null() { xmlFreeIDTable((*cur).ids as xmlIDTablePtr); }
    (*cur).ids = 0 as *mut std::os::raw::c_void;
    if !(*cur).refs.is_null() {
        xmlFreeRefTable((*cur).refs as xmlRefTablePtr);
    }
    (*cur).refs = 0 as *mut std::os::raw::c_void;
    extSubset = (*cur).extSubset;
    intSubset = (*cur).intSubset;
    if intSubset == extSubset { extSubset = 0 as xmlDtdPtr }
    if !extSubset.is_null() {
        xmlUnlinkNode((*cur).extSubset as xmlNodePtr);
        (*cur).extSubset = 0 as *mut _xmlDtd;
        xmlFreeDtd(extSubset);
    }
    if !intSubset.is_null() {
        xmlUnlinkNode((*cur).intSubset as xmlNodePtr);
        (*cur).intSubset = 0 as *mut _xmlDtd;
        xmlFreeDtd(intSubset);
    }
    if !(*cur).children.is_null() { xmlFreeNodeList((*cur).children); }
    if !(*cur).oldNs.is_null() { xmlFreeNsList((*cur).oldNs); }
    if !(*cur).version.is_null() &&
           (dict.is_null() ||
                xmlDictOwns(dict, (*cur).version) == 0 as std::os::raw::c_int) {
        xmlFree.expect("non-null function pointer")((*cur).version as
                                                        *mut std::os::raw::c_char as
                                                        *mut std::os::raw::c_void);
    }
    if !(*cur).name.is_null() &&
           (dict.is_null() ||
                xmlDictOwns(dict, (*cur).name as *const xmlChar) ==
                    0 as std::os::raw::c_int) {
        xmlFree.expect("non-null function pointer")((*cur).name as
                                                        *mut std::os::raw::c_void);
    }
    if !(*cur).encoding.is_null() &&
           (dict.is_null() ||
                xmlDictOwns(dict, (*cur).encoding) == 0 as std::os::raw::c_int) {
        xmlFree.expect("non-null function pointer")((*cur).encoding as
                                                        *mut std::os::raw::c_char as
                                                        *mut std::os::raw::c_void);
    }
    if !(*cur).URL.is_null() &&
           (dict.is_null() ||
                xmlDictOwns(dict, (*cur).URL) == 0 as std::os::raw::c_int) {
        xmlFree.expect("non-null function pointer")((*cur).URL as
                                                        *mut std::os::raw::c_char as
                                                        *mut std::os::raw::c_void);
    }
    xmlFree.expect("non-null function pointer")(cur as *mut std::os::raw::c_void);
    if !dict.is_null() { xmlDictFree(dict); };
}
/* *
 * xmlStringLenGetNodeList:
 * @doc:  the document
 * @value:  the value of the text
 * @len:  the length of the string value
 *
 * Parse the value string and build the node list associated. Should
 * produce a flat tree with only TEXTs and ENTITY_REFs.
 * Returns a pointer to the first child
 */
#[no_mangle]
pub unsafe extern "C" fn xmlStringLenGetNodeList(mut doc: *const xmlDoc,
                                                 mut value: *const xmlChar,
                                                 mut len: std::os::raw::c_int)
 -> xmlNodePtr {
    let mut current_block: u64;
    let mut ret: xmlNodePtr = 0 as xmlNodePtr;
    let mut last: xmlNodePtr = 0 as xmlNodePtr;
    let mut node: xmlNodePtr = 0 as *mut xmlNode;
    let mut val: *mut xmlChar = 0 as *mut xmlChar;
    let mut cur: *const xmlChar = value;
    let mut end: *const xmlChar = cur.offset(len as isize);
    let mut q: *const xmlChar = 0 as *const xmlChar;
    let mut ent: xmlEntityPtr = 0 as *mut xmlEntity;
    let mut buf: xmlBufPtr = 0 as *mut xmlBuf;
    if value.is_null() { return 0 as xmlNodePtr }
    buf = xmlBufCreateSize(0 as std::os::raw::c_int as size_t);
    if buf.is_null() { return 0 as xmlNodePtr }
    xmlBufSetAllocationScheme(buf, XML_BUFFER_ALLOC_HYBRID);
    q = cur;
    loop  {
        if !(cur < end && *cur as std::os::raw::c_int != 0 as std::os::raw::c_int) {
            current_block = 16590946904645350046;
            break ;
        }
        if *cur.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int == '&' as i32
           {
            let mut charval: std::os::raw::c_int = 0 as std::os::raw::c_int;
            let mut tmp: xmlChar = 0;
            /*
	     * Save the current text.
	     */
            if cur != q {
                if xmlBufAdd(buf, q,
                             cur.offset_from(q) as std::os::raw::c_long as
                                 std::os::raw::c_int) != 0 {
                    current_block = 9537528482611773413;
                    break ;
                }
            }
            q = cur;
            if cur.offset(2 as std::os::raw::c_int as isize) < end &&
                   *cur.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                       '#' as i32 &&
                   *cur.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                       'x' as i32 {
                cur = cur.offset(3 as std::os::raw::c_int as isize);
                if cur < end {
                    tmp = *cur
                } else { tmp = 0 as std::os::raw::c_int as xmlChar }
                while tmp as std::os::raw::c_int != ';' as i32 {
                    /* Non input consuming loop */
                    if tmp as std::os::raw::c_int >= '0' as i32 &&
                           tmp as std::os::raw::c_int <= '9' as i32 {
                        charval =
                            charval * 16 as std::os::raw::c_int +
                                (tmp as std::os::raw::c_int - '0' as i32)
                    } else if tmp as std::os::raw::c_int >= 'a' as i32 &&
                                  tmp as std::os::raw::c_int <= 'f' as i32 {
                        charval =
                            charval * 16 as std::os::raw::c_int +
                                (tmp as std::os::raw::c_int - 'a' as i32) +
                                10 as std::os::raw::c_int
                    } else if tmp as std::os::raw::c_int >= 'A' as i32 &&
                                  tmp as std::os::raw::c_int <= 'F' as i32 {
                        charval =
                            charval * 16 as std::os::raw::c_int +
                                (tmp as std::os::raw::c_int - 'A' as i32) +
                                10 as std::os::raw::c_int
                    } else {
                        xmlTreeErr(XML_TREE_INVALID_HEX as std::os::raw::c_int,
                                   doc as xmlNodePtr,
                                   0 as *const std::os::raw::c_char);
                        charval = 0 as std::os::raw::c_int;
                        break ;
                    }
                    cur = cur.offset(1);
                    if cur < end {
                        tmp = *cur
                    } else { tmp = 0 as std::os::raw::c_int as xmlChar }
                }
                if tmp as std::os::raw::c_int == ';' as i32 { cur = cur.offset(1) }
                q = cur
            } else if cur.offset(1 as std::os::raw::c_int as isize) < end &&
                          *cur.offset(1 as std::os::raw::c_int as isize) as
                              std::os::raw::c_int == '#' as i32 {
                cur = cur.offset(2 as std::os::raw::c_int as isize);
                if cur < end {
                    tmp = *cur
                } else { tmp = 0 as std::os::raw::c_int as xmlChar }
                while tmp as std::os::raw::c_int != ';' as i32 {
                    /* Non input consuming loops */
                    if tmp as std::os::raw::c_int >= '0' as i32 &&
                           tmp as std::os::raw::c_int <= '9' as i32 {
                        charval =
                            charval * 10 as std::os::raw::c_int +
                                (tmp as std::os::raw::c_int - '0' as i32);
                        cur = cur.offset(1);
                        if cur < end {
                            tmp = *cur
                        } else { tmp = 0 as std::os::raw::c_int as xmlChar }
                    } else {
                        xmlTreeErr(XML_TREE_INVALID_DEC as std::os::raw::c_int,
                                   doc as xmlNodePtr,
                                   0 as *const std::os::raw::c_char);
                        charval = 0 as std::os::raw::c_int;
                        break ;
                    }
                }
                if tmp as std::os::raw::c_int == ';' as i32 { cur = cur.offset(1) }
                q = cur
            } else {
                /*
		 * Read the entity string
		 */
                cur = cur.offset(1);
                q = cur;
                while cur < end && *cur as std::os::raw::c_int != 0 as std::os::raw::c_int &&
                          *cur as std::os::raw::c_int != ';' as i32 {
                    cur = cur.offset(1)
                }
                if cur >= end || *cur as std::os::raw::c_int == 0 as std::os::raw::c_int {
                    xmlTreeErr(XML_TREE_UNTERMINATED_ENTITY as std::os::raw::c_int,
                               doc as xmlNodePtr, q as *const std::os::raw::c_char);
                    current_block = 9537528482611773413;
                    break ;
                } else {
                    if cur != q {
                        /*
		     * Predefined entities don't generate nodes
		     */
                        val =
                            xmlStrndup(q,
                                       cur.offset_from(q) as
                                           std::os::raw::c_long as std::os::raw::c_int);
                        ent = xmlGetDocEntity(doc, val);
                        if !ent.is_null() &&
                               (*ent).etype as std::os::raw::c_uint ==
                                   XML_INTERNAL_PREDEFINED_ENTITY as
                                       std::os::raw::c_int as std::os::raw::c_uint {
                            if xmlBufCat(buf, (*ent).content) != 0 {
                                current_block = 9537528482611773413;
                                break ;
                            }
                        } else {
                            /*
			 * Flush buffer so far
			 */
                            if xmlBufIsEmpty(buf) == 0 {
                                node =
                                    xmlNewDocText(doc, 0 as *const xmlChar);
                                if node.is_null() {
                                    if !val.is_null() {
                                        xmlFree.expect("non-null function pointer")(val
                                                                                        as
                                                                                        *mut std::os::raw::c_void);
                                    }
                                    current_block = 9537528482611773413;
                                    break ;
                                } else {
                                    (*node).content = xmlBufDetach(buf);
                                    if last.is_null() {
                                        ret = node;
                                        last = ret
                                    } else {
                                        last = xmlAddNextSibling(last, node)
                                    }
                                }
                            }
                            /*
			 * Create a new REFERENCE_REF node
			 */
                            node = xmlNewReference(doc, val);
                            if node.is_null() {
                                if !val.is_null() {
                                    xmlFree.expect("non-null function pointer")(val
                                                                                    as
                                                                                    *mut std::os::raw::c_void);
                                }
                                current_block = 9537528482611773413;
                                break ;
                            } else {
                                if !ent.is_null() && (*ent).children.is_null()
                                   {
                                    let mut temp: xmlNodePtr =
                                        0 as *mut xmlNode;
                                    /* Set to non-NULL value to avoid recursion. */
                                    (*ent).children =
                                        -(1 as std::os::raw::c_int) as xmlNodePtr;
                                    (*ent).children =
                                        xmlStringGetNodeList(doc,
                                                             (*node).content
                                                                 as
                                                                 *const xmlChar);
                                    (*ent).owner = 1 as std::os::raw::c_int;
                                    temp = (*ent).children;
                                    while !temp.is_null() {
                                        (*temp).parent = ent as xmlNodePtr;
                                        (*ent).last = temp;
                                        temp = (*temp).next
                                    }
                                }
                                if last.is_null() {
                                    ret = node;
                                    last = ret
                                } else {
                                    last = xmlAddNextSibling(last, node)
                                }
                            }
                        }
                        xmlFree.expect("non-null function pointer")(val as
                                                                        *mut std::os::raw::c_void);
                    }
                    cur = cur.offset(1);
                    q = cur
                }
            }
            if !(charval != 0 as std::os::raw::c_int) { continue ; }
            let mut buffer: [xmlChar; 10] = [0; 10];
            let mut l: std::os::raw::c_int = 0;
            l = xmlCopyCharMultiByte(buffer.as_mut_ptr(), charval);
            buffer[l as usize] = 0 as std::os::raw::c_int as xmlChar;
            if xmlBufCat(buf, buffer.as_mut_ptr()) != 0 {
                current_block = 9537528482611773413;
                break ;
            }
            charval = 0 as std::os::raw::c_int
        } else { cur = cur.offset(1) }
    }
    match current_block {
        16590946904645350046 => {
            if cur != q {
                /*
	 * Handle the last piece of text.
	 */
                if xmlBufAdd(buf, q,
                             cur.offset_from(q) as std::os::raw::c_long as
                                 std::os::raw::c_int) != 0 {
                    current_block = 9537528482611773413;
                } else { current_block = 1677945370889843322; }
            } else { current_block = 1677945370889843322; }
            match current_block {
                9537528482611773413 => { }
                _ => {
                    if xmlBufIsEmpty(buf) == 0 {
                        node = xmlNewDocText(doc, 0 as *const xmlChar);
                        if !node.is_null() {
                            (*node).content = xmlBufDetach(buf);
                            if last.is_null() {
                                ret = node
                            } else { xmlAddNextSibling(last, node); }
                        }
                    } else if ret.is_null() {
                        ret =
                            xmlNewDocText(doc,
                                          b"\x00" as *const u8 as
                                              *const std::os::raw::c_char as
                                              *mut xmlChar)
                    }
                }
            }
        }
        _ => { }
    }
    xmlBufFree(buf);
    return ret;
}
/* *
 * xmlStringGetNodeList:
 * @doc:  the document
 * @value:  the value of the attribute
 *
 * Parse the value string and build the node list associated. Should
 * produce a flat tree with only TEXTs and ENTITY_REFs.
 * Returns a pointer to the first child
 */
#[no_mangle]
pub unsafe extern "C" fn xmlStringGetNodeList(mut doc: *const xmlDoc,
                                              mut value: *const xmlChar)
 -> xmlNodePtr {
    let mut current_block: u64;
    let mut ret: xmlNodePtr = 0 as xmlNodePtr;
    let mut last: xmlNodePtr = 0 as xmlNodePtr;
    let mut node: xmlNodePtr = 0 as *mut xmlNode;
    let mut val: *mut xmlChar = 0 as *mut xmlChar;
    let mut cur: *const xmlChar = value;
    let mut q: *const xmlChar = 0 as *const xmlChar;
    let mut ent: xmlEntityPtr = 0 as *mut xmlEntity;
    let mut buf: xmlBufPtr = 0 as *mut xmlBuf;
    if value.is_null() { return 0 as xmlNodePtr }
    buf = xmlBufCreateSize(0 as std::os::raw::c_int as size_t);
    if buf.is_null() { return 0 as xmlNodePtr }
    xmlBufSetAllocationScheme(buf, XML_BUFFER_ALLOC_HYBRID);
    q = cur;
    loop  {
        if !(*cur as std::os::raw::c_int != 0 as std::os::raw::c_int) {
            current_block = 2945622622075328793;
            break ;
        }
        if *cur.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int == '&' as i32
           {
            let mut charval: std::os::raw::c_int = 0 as std::os::raw::c_int;
            let mut tmp: xmlChar = 0;
            /*
	     * Save the current text.
	     */
            if cur != q {
                if xmlBufAdd(buf, q,
                             cur.offset_from(q) as std::os::raw::c_long as
                                 std::os::raw::c_int) != 0 {
                    current_block = 15788669585006550891;
                    break ;
                }
            }
            q = cur;
            if *cur.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                   '#' as i32 &&
                   *cur.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                       'x' as i32 {
                cur = cur.offset(3 as std::os::raw::c_int as isize);
                tmp = *cur;
                while tmp as std::os::raw::c_int != ';' as i32 {
                    /* Non input consuming loop */
                    if tmp as std::os::raw::c_int >= '0' as i32 &&
                           tmp as std::os::raw::c_int <= '9' as i32 {
                        charval =
                            charval * 16 as std::os::raw::c_int +
                                (tmp as std::os::raw::c_int - '0' as i32)
                    } else if tmp as std::os::raw::c_int >= 'a' as i32 &&
                                  tmp as std::os::raw::c_int <= 'f' as i32 {
                        charval =
                            charval * 16 as std::os::raw::c_int +
                                (tmp as std::os::raw::c_int - 'a' as i32) +
                                10 as std::os::raw::c_int
                    } else if tmp as std::os::raw::c_int >= 'A' as i32 &&
                                  tmp as std::os::raw::c_int <= 'F' as i32 {
                        charval =
                            charval * 16 as std::os::raw::c_int +
                                (tmp as std::os::raw::c_int - 'A' as i32) +
                                10 as std::os::raw::c_int
                    } else {
                        xmlTreeErr(XML_TREE_INVALID_HEX as std::os::raw::c_int,
                                   doc as xmlNodePtr,
                                   0 as *const std::os::raw::c_char);
                        charval = 0 as std::os::raw::c_int;
                        break ;
                    }
                    cur = cur.offset(1);
                    tmp = *cur
                }
                if tmp as std::os::raw::c_int == ';' as i32 { cur = cur.offset(1) }
                q = cur
            } else if *cur.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                          '#' as i32 {
                cur = cur.offset(2 as std::os::raw::c_int as isize);
                tmp = *cur;
                while tmp as std::os::raw::c_int != ';' as i32 {
                    /* Non input consuming loops */
                    if tmp as std::os::raw::c_int >= '0' as i32 &&
                           tmp as std::os::raw::c_int <= '9' as i32 {
                        charval =
                            charval * 10 as std::os::raw::c_int +
                                (tmp as std::os::raw::c_int - '0' as i32);
                        cur = cur.offset(1);
                        tmp = *cur
                    } else {
                        xmlTreeErr(XML_TREE_INVALID_DEC as std::os::raw::c_int,
                                   doc as xmlNodePtr,
                                   0 as *const std::os::raw::c_char);
                        charval = 0 as std::os::raw::c_int;
                        break ;
                    }
                }
                if tmp as std::os::raw::c_int == ';' as i32 { cur = cur.offset(1) }
                q = cur
            } else {
                /*
		 * Read the entity string
		 */
                cur = cur.offset(1);
                q = cur;
                while *cur as std::os::raw::c_int != 0 as std::os::raw::c_int &&
                          *cur as std::os::raw::c_int != ';' as i32 {
                    cur = cur.offset(1)
                }
                if *cur as std::os::raw::c_int == 0 as std::os::raw::c_int {
                    xmlTreeErr(XML_TREE_UNTERMINATED_ENTITY as std::os::raw::c_int,
                               doc as xmlNodePtr, q as *const std::os::raw::c_char);
                    current_block = 15788669585006550891;
                    break ;
                } else {
                    if cur != q {
                        /*
		     * Predefined entities don't generate nodes
		     */
                        val =
                            xmlStrndup(q,
                                       cur.offset_from(q) as
                                           std::os::raw::c_long as std::os::raw::c_int);
                        ent = xmlGetDocEntity(doc, val);
                        if !ent.is_null() &&
                               (*ent).etype as std::os::raw::c_uint ==
                                   XML_INTERNAL_PREDEFINED_ENTITY as
                                       std::os::raw::c_int as std::os::raw::c_uint {
                            if xmlBufCat(buf, (*ent).content) != 0 {
                                current_block = 15788669585006550891;
                                break ;
                            }
                        } else {
                            /*
			 * Flush buffer so far
			 */
                            if xmlBufIsEmpty(buf) == 0 {
                                node =
                                    xmlNewDocText(doc, 0 as *const xmlChar);
                                (*node).content = xmlBufDetach(buf);
                                if last.is_null() {
                                    ret = node;
                                    last = ret
                                } else {
                                    last = xmlAddNextSibling(last, node)
                                }
                            }
                            /*
			 * Create a new REFERENCE_REF node
			 */
                            node = xmlNewReference(doc, val);
                            if node.is_null() {
                                if !val.is_null() {
                                    xmlFree.expect("non-null function pointer")(val
                                                                                    as
                                                                                    *mut std::os::raw::c_void);
                                }
                                current_block = 15788669585006550891;
                                break ;
                            } else {
                                if !ent.is_null() && (*ent).children.is_null()
                                   {
                                    let mut temp: xmlNodePtr =
                                        0 as *mut xmlNode;
                                    /* Set to non-NULL value to avoid recursion. */
                                    (*ent).children =
                                        -(1 as std::os::raw::c_int) as xmlNodePtr;
                                    (*ent).children =
                                        xmlStringGetNodeList(doc,
                                                             (*node).content
                                                                 as
                                                                 *const xmlChar);
                                    (*ent).owner = 1 as std::os::raw::c_int;
                                    temp = (*ent).children;
                                    while !temp.is_null() {
                                        (*temp).parent = ent as xmlNodePtr;
                                        (*ent).last = temp;
                                        temp = (*temp).next
                                    }
                                }
                                if last.is_null() {
                                    ret = node;
                                    last = ret
                                } else {
                                    last = xmlAddNextSibling(last, node)
                                }
                            }
                        }
                        xmlFree.expect("non-null function pointer")(val as
                                                                        *mut std::os::raw::c_void);
                    }
                    cur = cur.offset(1);
                    q = cur
                }
            }
            if !(charval != 0 as std::os::raw::c_int) { continue ; }
            let mut buffer: [xmlChar; 10] = [0; 10];
            let mut len: std::os::raw::c_int = 0;
            len = xmlCopyCharMultiByte(buffer.as_mut_ptr(), charval);
            buffer[len as usize] = 0 as std::os::raw::c_int as xmlChar;
            if xmlBufCat(buf, buffer.as_mut_ptr()) != 0 {
                current_block = 15788669585006550891;
                break ;
            }
            charval = 0 as std::os::raw::c_int
        } else { cur = cur.offset(1) }
    }
    match current_block {
        2945622622075328793 => {
            if cur != q || ret.is_null() {
                /*
	 * Handle the last piece of text.
	 */
                xmlBufAdd(buf, q,
                          cur.offset_from(q) as std::os::raw::c_long as
                              std::os::raw::c_int);
            }
            if xmlBufIsEmpty(buf) == 0 {
                node = xmlNewDocText(doc, 0 as *const xmlChar);
                (*node).content = xmlBufDetach(buf);
                if last.is_null() {
                    ret = node
                } else { xmlAddNextSibling(last, node); }
            }
        }
        _ => { }
    }
    xmlBufFree(buf);
    return ret;
}
/* *
 * xmlNodeListGetString:
 * @doc:  the document
 * @list:  a Node list
 * @inLine:  should we replace entity contents or show their external form
 *
 * Build the string equivalent to the text contained in the Node list
 * made of TEXTs and ENTITY_REFs
 *
 * Returns a pointer to the string copy, the caller must free it with xmlFree().
 */
#[no_mangle]
pub unsafe extern "C" fn xmlNodeListGetString(mut doc: xmlDocPtr,
                                              mut list: *const xmlNode,
                                              mut inLine: std::os::raw::c_int)
 -> *mut xmlChar {
    let mut node: *const xmlNode = list;
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    let mut ent: xmlEntityPtr = 0 as *mut xmlEntity;
    let mut attr: std::os::raw::c_int = 0;
    if list.is_null() { return 0 as *mut xmlChar }
    if !(*list).parent.is_null() &&
           (*(*list).parent).type_0 as std::os::raw::c_uint ==
               XML_ATTRIBUTE_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        attr = 1 as std::os::raw::c_int
    } else { attr = 0 as std::os::raw::c_int }
    while !node.is_null() {
        if (*node).type_0 as std::os::raw::c_uint ==
               XML_TEXT_NODE as std::os::raw::c_int as std::os::raw::c_uint ||
               (*node).type_0 as std::os::raw::c_uint ==
                   XML_CDATA_SECTION_NODE as std::os::raw::c_int as std::os::raw::c_uint {
            if inLine != 0 {
                ret = xmlStrcat(ret, (*node).content)
            } else {
                let mut buffer: *mut xmlChar = 0 as *mut xmlChar;
                if attr != 0 {
                    buffer = xmlEncodeAttributeEntities(doc, (*node).content)
                } else {
                    buffer = xmlEncodeEntitiesReentrant(doc, (*node).content)
                }
                if !buffer.is_null() {
                    ret = xmlStrcat(ret, buffer);
                    xmlFree.expect("non-null function pointer")(buffer as
                                                                    *mut std::os::raw::c_void);
                }
            }
        } else if (*node).type_0 as std::os::raw::c_uint ==
                      XML_ENTITY_REF_NODE as std::os::raw::c_int as std::os::raw::c_uint {
            if inLine != 0 {
                ent = xmlGetDocEntity(doc as *const xmlDoc, (*node).name);
                if !ent.is_null() {
                    let mut buffer_0: *mut xmlChar = 0 as *mut xmlChar;
                    /* an entity content can be any "well balanced chunk",
                     * i.e. the result of the content [43] production:
                     * http://www.w3.org/TR/REC-xml#NT-content.
                     * So it can contain text, CDATA section or nested
                     * entity reference nodes (among others).
                     * -> we recursive  call xmlNodeListGetString()
                     * which handles these types */
                    buffer_0 =
                        xmlNodeListGetString(doc, (*ent).children,
                                             1 as std::os::raw::c_int);
                    if !buffer_0.is_null() {
                        ret = xmlStrcat(ret, buffer_0);
                        xmlFree.expect("non-null function pointer")(buffer_0
                                                                        as
                                                                        *mut std::os::raw::c_void);
                    }
                } else { ret = xmlStrcat(ret, (*node).content) }
            } else {
                let mut buf: [xmlChar; 2] = [0; 2];
                buf[0 as std::os::raw::c_int as usize] = '&' as i32 as xmlChar;
                buf[1 as std::os::raw::c_int as usize] = 0 as std::os::raw::c_int as xmlChar;
                ret = xmlStrncat(ret, buf.as_mut_ptr(), 1 as std::os::raw::c_int);
                ret = xmlStrcat(ret, (*node).name);
                buf[0 as std::os::raw::c_int as usize] = ';' as i32 as xmlChar;
                buf[1 as std::os::raw::c_int as usize] = 0 as std::os::raw::c_int as xmlChar;
                ret = xmlStrncat(ret, buf.as_mut_ptr(), 1 as std::os::raw::c_int)
            }
        }
        node = (*node).next
    }
    return ret;
}
/* *
 * xmlNodeListGetRawString:
 * @doc:  the document
 * @list:  a Node list
 * @inLine:  should we replace entity contents or show their external form
 *
 * Builds the string equivalent to the text contained in the Node list
 * made of TEXTs and ENTITY_REFs, contrary to xmlNodeListGetString()
 * this function doesn't do any character encoding handling.
 *
 * Returns a pointer to the string copy, the caller must free it with xmlFree().
 */
#[no_mangle]
pub unsafe extern "C" fn xmlNodeListGetRawString(mut doc: *const xmlDoc,
                                                 mut list: *const xmlNode,
                                                 mut inLine: std::os::raw::c_int)
 -> *mut xmlChar {
    let mut node: *const xmlNode = list;
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    let mut ent: xmlEntityPtr = 0 as *mut xmlEntity;
    if list.is_null() { return 0 as *mut xmlChar }
    while !node.is_null() {
        if (*node).type_0 as std::os::raw::c_uint ==
               XML_TEXT_NODE as std::os::raw::c_int as std::os::raw::c_uint ||
               (*node).type_0 as std::os::raw::c_uint ==
                   XML_CDATA_SECTION_NODE as std::os::raw::c_int as std::os::raw::c_uint {
            if inLine != 0 {
                ret = xmlStrcat(ret, (*node).content)
            } else {
                let mut buffer: *mut xmlChar = 0 as *mut xmlChar;
                buffer = xmlEncodeSpecialChars(doc, (*node).content);
                if !buffer.is_null() {
                    ret = xmlStrcat(ret, buffer);
                    xmlFree.expect("non-null function pointer")(buffer as
                                                                    *mut std::os::raw::c_void);
                }
            }
        } else if (*node).type_0 as std::os::raw::c_uint ==
                      XML_ENTITY_REF_NODE as std::os::raw::c_int as std::os::raw::c_uint {
            if inLine != 0 {
                ent = xmlGetDocEntity(doc, (*node).name);
                if !ent.is_null() {
                    let mut buffer_0: *mut xmlChar = 0 as *mut xmlChar;
                    /* an entity content can be any "well balanced chunk",
                     * i.e. the result of the content [43] production:
                     * http://www.w3.org/TR/REC-xml#NT-content.
                     * So it can contain text, CDATA section or nested
                     * entity reference nodes (among others).
                     * -> we recursive  call xmlNodeListGetRawString()
                     * which handles these types */
                    buffer_0 =
                        xmlNodeListGetRawString(doc, (*ent).children,
                                                1 as std::os::raw::c_int);
                    if !buffer_0.is_null() {
                        ret = xmlStrcat(ret, buffer_0);
                        xmlFree.expect("non-null function pointer")(buffer_0
                                                                        as
                                                                        *mut std::os::raw::c_void);
                    }
                } else { ret = xmlStrcat(ret, (*node).content) }
            } else {
                let mut buf: [xmlChar; 2] = [0; 2];
                buf[0 as std::os::raw::c_int as usize] = '&' as i32 as xmlChar;
                buf[1 as std::os::raw::c_int as usize] = 0 as std::os::raw::c_int as xmlChar;
                ret = xmlStrncat(ret, buf.as_mut_ptr(), 1 as std::os::raw::c_int);
                ret = xmlStrcat(ret, (*node).name);
                buf[0 as std::os::raw::c_int as usize] = ';' as i32 as xmlChar;
                buf[1 as std::os::raw::c_int as usize] = 0 as std::os::raw::c_int as xmlChar;
                ret = xmlStrncat(ret, buf.as_mut_ptr(), 1 as std::os::raw::c_int)
            }
        }
        node = (*node).next
    }
    return ret;
}
/* LIBXML_TREE_ENABLED */
unsafe extern "C" fn xmlNewPropInternal(mut node: xmlNodePtr,
                                        mut ns: xmlNsPtr,
                                        mut name: *const xmlChar,
                                        mut value: *const xmlChar,
                                        mut eatname: std::os::raw::c_int)
 -> xmlAttrPtr {
    let mut cur: xmlAttrPtr = 0 as *mut xmlAttr;
    let mut doc: xmlDocPtr = 0 as xmlDocPtr;
    if !node.is_null() &&
           (*node).type_0 as std::os::raw::c_uint !=
               XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        if eatname == 1 as std::os::raw::c_int &&
               ((*node).doc.is_null() ||
                    xmlDictOwns((*(*node).doc).dict, name) == 0) {
            xmlFree.expect("non-null function pointer")(name as *mut xmlChar
                                                            as
                                                            *mut std::os::raw::c_void);
        }
        return 0 as xmlAttrPtr
    }
    /*
     * Allocate a new property and fill the fields.
     */
    cur =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlAttr>()
                                                          as std::os::raw::c_ulong) as
            xmlAttrPtr;
    if cur.is_null() {
        if eatname == 1 as std::os::raw::c_int &&
               (node.is_null() || (*node).doc.is_null() ||
                    xmlDictOwns((*(*node).doc).dict, name) == 0) {
            xmlFree.expect("non-null function pointer")(name as *mut xmlChar
                                                            as
                                                            *mut std::os::raw::c_void);
        }
        xmlTreeErrMemory(b"building attribute\x00" as *const u8 as
                             *const std::os::raw::c_char);
        return 0 as xmlAttrPtr
    }
    memset(cur as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<xmlAttr>() as std::os::raw::c_ulong);
    (*cur).type_0 = XML_ATTRIBUTE_NODE;
    (*cur).parent = node;
    if !node.is_null() { doc = (*node).doc; (*cur).doc = doc }
    (*cur).ns = ns;
    if eatname == 0 as std::os::raw::c_int {
        if !doc.is_null() && !(*doc).dict.is_null() {
            (*cur).name =
                xmlDictLookup((*doc).dict, name, -(1 as std::os::raw::c_int)) as
                    *mut xmlChar
        } else { (*cur).name = xmlStrdup(name) }
    } else { (*cur).name = name }
    if !value.is_null() {
        let mut tmp: xmlNodePtr = 0 as *mut xmlNode;
        if xmlCheckUTF8(value) == 0 {
            xmlTreeErr(XML_TREE_NOT_UTF8 as std::os::raw::c_int, doc as xmlNodePtr,
                       0 as *const std::os::raw::c_char);
            if !doc.is_null() {
                (*doc).encoding =
                    xmlStrdup(b"ISO-8859-1\x00" as *const u8 as
                                  *const std::os::raw::c_char as *mut xmlChar)
            }
        }
        (*cur).children = xmlNewDocText(doc as *const xmlDoc, value);
        (*cur).last = 0 as *mut _xmlNode;
        tmp = (*cur).children;
        while !tmp.is_null() {
            (*tmp).parent = cur as xmlNodePtr;
            if (*tmp).next.is_null() { (*cur).last = tmp }
            tmp = (*tmp).next
        }
    }
    /*
     * Add it at the end to preserve parsing order ...
     */
    if !node.is_null() {
        if (*node).properties.is_null() {
            (*node).properties = cur
        } else {
            let mut prev: xmlAttrPtr = (*node).properties;
            while !(*prev).next.is_null() { prev = (*prev).next }
            (*prev).next = cur;
            (*cur).prev = prev
        }
    }
    if !value.is_null() && !node.is_null() &&
           xmlIsID((*node).doc, node, cur) == 1 as std::os::raw::c_int {
        xmlAddID(0 as xmlValidCtxtPtr, (*node).doc, value, cur);
    }
    if __xmlRegisterCallbacks != 0 &&
           (*__xmlRegisterNodeDefaultValue()).is_some() {
        (*__xmlRegisterNodeDefaultValue()).expect("non-null function pointer")(cur
                                                                                   as
                                                                                   xmlNodePtr);
    }
    return cur;
}
/* *
 * xmlNewProp:
 * @node:  the holding node
 * @name:  the name of the attribute
 * @value:  the value of the attribute
 *
 * Create a new property carried by a node.
 * Returns a pointer to the attribute
 */
#[no_mangle]
pub unsafe extern "C" fn xmlNewProp(mut node: xmlNodePtr,
                                    mut name: *const xmlChar,
                                    mut value: *const xmlChar) -> xmlAttrPtr {
    if name.is_null() { return 0 as xmlAttrPtr }
    return xmlNewPropInternal(node, 0 as xmlNsPtr, name, value,
                              0 as std::os::raw::c_int);
}
/* LIBXML_TREE_ENABLED */
/* *
 * xmlNewNsProp:
 * @node:  the holding node
 * @ns:  the namespace
 * @name:  the name of the attribute
 * @value:  the value of the attribute
 *
 * Create a new property tagged with a namespace and carried by a node.
 * Returns a pointer to the attribute
 */
#[no_mangle]
pub unsafe extern "C" fn xmlNewNsProp(mut node: xmlNodePtr, mut ns: xmlNsPtr,
                                      mut name: *const xmlChar,
                                      mut value: *const xmlChar)
 -> xmlAttrPtr {
    if name.is_null() { return 0 as xmlAttrPtr }
    return xmlNewPropInternal(node, ns, name, value, 0 as std::os::raw::c_int);
}
/* *
 * xmlNewNsPropEatName:
 * @node:  the holding node
 * @ns:  the namespace
 * @name:  the name of the attribute
 * @value:  the value of the attribute
 *
 * Create a new property tagged with a namespace and carried by a node.
 * Returns a pointer to the attribute
 */
#[no_mangle]
pub unsafe extern "C" fn xmlNewNsPropEatName(mut node: xmlNodePtr,
                                             mut ns: xmlNsPtr,
                                             mut name: *mut xmlChar,
                                             mut value: *const xmlChar)
 -> xmlAttrPtr {
    if name.is_null() { return 0 as xmlAttrPtr }
    return xmlNewPropInternal(node, ns, name, value, 1 as std::os::raw::c_int);
}
/* *
 * xmlNewDocProp:
 * @doc:  the document
 * @name:  the name of the attribute
 * @value:  the value of the attribute
 *
 * Create a new property carried by a document.
 * Returns a pointer to the attribute
 */
#[no_mangle]
pub unsafe extern "C" fn xmlNewDocProp(mut doc: xmlDocPtr,
                                       mut name: *const xmlChar,
                                       mut value: *const xmlChar)
 -> xmlAttrPtr {
    let mut cur: xmlAttrPtr = 0 as *mut xmlAttr;
    if name.is_null() { return 0 as xmlAttrPtr }
    /*
     * Allocate a new property and fill the fields.
     */
    cur =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlAttr>()
                                                          as std::os::raw::c_ulong) as
            xmlAttrPtr;
    if cur.is_null() {
        xmlTreeErrMemory(b"building attribute\x00" as *const u8 as
                             *const std::os::raw::c_char);
        return 0 as xmlAttrPtr
    }
    memset(cur as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<xmlAttr>() as std::os::raw::c_ulong);
    (*cur).type_0 = XML_ATTRIBUTE_NODE;
    if !doc.is_null() && !(*doc).dict.is_null() {
        (*cur).name = xmlDictLookup((*doc).dict, name, -(1 as std::os::raw::c_int))
    } else { (*cur).name = xmlStrdup(name) }
    (*cur).doc = doc;
    if !value.is_null() {
        let mut tmp: xmlNodePtr = 0 as *mut xmlNode;
        (*cur).children = xmlStringGetNodeList(doc as *const xmlDoc, value);
        (*cur).last = 0 as *mut _xmlNode;
        tmp = (*cur).children;
        while !tmp.is_null() {
            (*tmp).parent = cur as xmlNodePtr;
            if (*tmp).next.is_null() { (*cur).last = tmp }
            tmp = (*tmp).next
        }
    }
    if __xmlRegisterCallbacks != 0 &&
           (*__xmlRegisterNodeDefaultValue()).is_some() {
        (*__xmlRegisterNodeDefaultValue()).expect("non-null function pointer")(cur
                                                                                   as
                                                                                   xmlNodePtr);
    }
    return cur;
}
/* *
 * xmlFreePropList:
 * @cur:  the first property in the list
 *
 * Free a property and all its siblings, all the children are freed too.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlFreePropList(mut cur: xmlAttrPtr) {
    let mut next: xmlAttrPtr = 0 as *mut xmlAttr;
    if cur.is_null() { return }
    while !cur.is_null() { next = (*cur).next; xmlFreeProp(cur); cur = next };
}
/* *
 * xmlFreeProp:
 * @cur:  an attribute
 *
 * Free one attribute, all the content is freed too
 */
#[no_mangle]
pub unsafe extern "C" fn xmlFreeProp(mut cur: xmlAttrPtr) {
    let mut dict: xmlDictPtr = 0 as xmlDictPtr;
    if cur.is_null() { return }
    if !(*cur).doc.is_null() { dict = (*(*cur).doc).dict }
    if __xmlRegisterCallbacks != 0 &&
           (*__xmlDeregisterNodeDefaultValue()).is_some() {
        (*__xmlDeregisterNodeDefaultValue()).expect("non-null function pointer")(cur
                                                                                     as
                                                                                     xmlNodePtr);
    }
    /* Check for ID removal -> leading to invalid references ! */
    if !(*cur).doc.is_null() &&
           (*cur).atype as std::os::raw::c_uint ==
               XML_ATTRIBUTE_ID as std::os::raw::c_int as std::os::raw::c_uint {
        xmlRemoveID((*cur).doc, cur);
    }
    if !(*cur).children.is_null() { xmlFreeNodeList((*cur).children); }
    if !(*cur).name.is_null() &&
           (dict.is_null() ||
                xmlDictOwns(dict, (*cur).name) == 0 as std::os::raw::c_int) {
        xmlFree.expect("non-null function pointer")((*cur).name as
                                                        *mut std::os::raw::c_char as
                                                        *mut std::os::raw::c_void);
    }
    xmlFree.expect("non-null function pointer")(cur as *mut std::os::raw::c_void);
}
/* *
 * xmlRemoveProp:
 * @cur:  an attribute
 *
 * Unlink and free one attribute, all the content is freed too
 * Note this doesn't work for namespace definition attributes
 *
 * Returns 0 if success and -1 in case of error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlRemoveProp(mut cur: xmlAttrPtr) -> std::os::raw::c_int {
    let mut tmp: xmlAttrPtr = 0 as *mut xmlAttr;
    if cur.is_null() { return -(1 as std::os::raw::c_int) }
    if (*cur).parent.is_null() { return -(1 as std::os::raw::c_int) }
    tmp = (*(*cur).parent).properties;
    if tmp == cur {
        (*(*cur).parent).properties = (*cur).next;
        if !(*cur).next.is_null() { (*(*cur).next).prev = 0 as *mut _xmlAttr }
        xmlFreeProp(cur);
        return 0 as std::os::raw::c_int
    }
    while !tmp.is_null() {
        if (*tmp).next == cur {
            (*tmp).next = (*cur).next;
            if !(*tmp).next.is_null() { (*(*tmp).next).prev = tmp }
            xmlFreeProp(cur);
            return 0 as std::os::raw::c_int
        }
        tmp = (*tmp).next
    }
    return -(1 as std::os::raw::c_int);
}
/* *
 * xmlNewDocPI:
 * @doc:  the target document
 * @name:  the processing instruction name
 * @content:  the PI content
 *
 * Creation of a processing instruction element.
 * Returns a pointer to the new node object.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlNewDocPI(mut doc: xmlDocPtr,
                                     mut name: *const xmlChar,
                                     mut content: *const xmlChar)
 -> xmlNodePtr {
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    if name.is_null() { return 0 as xmlNodePtr }
    /*
     * Allocate a new node and fill the fields.
     */
    cur =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlNode>()
                                                          as std::os::raw::c_ulong) as
            xmlNodePtr;
    if cur.is_null() {
        xmlTreeErrMemory(b"building PI\x00" as *const u8 as
                             *const std::os::raw::c_char);
        return 0 as xmlNodePtr
    }
    memset(cur as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<xmlNode>() as std::os::raw::c_ulong);
    (*cur).type_0 = XML_PI_NODE;
    if !doc.is_null() && !(*doc).dict.is_null() {
        (*cur).name = xmlDictLookup((*doc).dict, name, -(1 as std::os::raw::c_int))
    } else { (*cur).name = xmlStrdup(name) }
    if !content.is_null() { (*cur).content = xmlStrdup(content) }
    (*cur).doc = doc;
    if __xmlRegisterCallbacks != 0 &&
           (*__xmlRegisterNodeDefaultValue()).is_some() {
        (*__xmlRegisterNodeDefaultValue()).expect("non-null function pointer")(cur);
    }
    return cur;
}
/* *
 * xmlNewPI:
 * @name:  the processing instruction name
 * @content:  the PI content
 *
 * Creation of a processing instruction element.
 * Use xmlDocNewPI preferably to get string interning
 *
 * Returns a pointer to the new node object.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlNewPI(mut name: *const xmlChar,
                                  mut content: *const xmlChar) -> xmlNodePtr {
    return xmlNewDocPI(0 as xmlDocPtr, name, content);
}
/* *
 * xmlNewNode:
 * @ns:  namespace if any
 * @name:  the node name
 *
 * Creation of a new node element. @ns is optional (NULL).
 *
 * Returns a pointer to the new node object. Uses xmlStrdup() to make
 * copy of @name.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlNewNode(mut ns: xmlNsPtr,
                                    mut name: *const xmlChar) -> xmlNodePtr {
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    if name.is_null() { return 0 as xmlNodePtr }
    /*
     * Allocate a new node and fill the fields.
     */
    cur =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlNode>()
                                                          as std::os::raw::c_ulong) as
            xmlNodePtr;
    if cur.is_null() {
        xmlTreeErrMemory(b"building node\x00" as *const u8 as
                             *const std::os::raw::c_char);
        return 0 as xmlNodePtr
    }
    memset(cur as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<xmlNode>() as std::os::raw::c_ulong);
    (*cur).type_0 = XML_ELEMENT_NODE;
    (*cur).name = xmlStrdup(name);
    (*cur).ns = ns;
    if __xmlRegisterCallbacks != 0 &&
           (*__xmlRegisterNodeDefaultValue()).is_some() {
        (*__xmlRegisterNodeDefaultValue()).expect("non-null function pointer")(cur);
    }
    return cur;
}
/* *
 * xmlNewNodeEatName:
 * @ns:  namespace if any
 * @name:  the node name
 *
 * Creation of a new node element. @ns is optional (NULL).
 *
 * Returns a pointer to the new node object, with pointer @name as
 * new node's name. Use xmlNewNode() if a copy of @name string is
 * is needed as new node's name.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlNewNodeEatName(mut ns: xmlNsPtr,
                                           mut name: *mut xmlChar)
 -> xmlNodePtr {
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    if name.is_null() { return 0 as xmlNodePtr }
    /*
     * Allocate a new node and fill the fields.
     */
    cur =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlNode>()
                                                          as std::os::raw::c_ulong) as
            xmlNodePtr;
    if cur.is_null() {
        xmlTreeErrMemory(b"building node\x00" as *const u8 as
                             *const std::os::raw::c_char);
        /* we can't check here that name comes from the doc dictionary */
        return 0 as xmlNodePtr
    }
    memset(cur as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<xmlNode>() as std::os::raw::c_ulong);
    (*cur).type_0 = XML_ELEMENT_NODE;
    (*cur).name = name;
    (*cur).ns = ns;
    if __xmlRegisterCallbacks != 0 &&
           (*__xmlRegisterNodeDefaultValue()).is_some() {
        (*__xmlRegisterNodeDefaultValue()).expect("non-null function pointer")(cur);
    }
    return cur;
}
/* *
 * xmlNewDocNode:
 * @doc:  the document
 * @ns:  namespace if any
 * @name:  the node name
 * @content:  the XML text content if any
 *
 * Creation of a new node element within a document. @ns and @content
 * are optional (NULL).
 * NOTE: @content is supposed to be a piece of XML CDATA, so it allow entities
 *       references, but XML special chars need to be escaped first by using
 *       xmlEncodeEntitiesReentrant(). Use xmlNewDocRawNode() if you don't
 *       need entities support.
 *
 * Returns a pointer to the new node object.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlNewDocNode(mut doc: xmlDocPtr, mut ns: xmlNsPtr,
                                       mut name: *const xmlChar,
                                       mut content: *const xmlChar)
 -> xmlNodePtr {
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    if !doc.is_null() && !(*doc).dict.is_null() {
        cur =
            xmlNewNodeEatName(ns,
                              xmlDictLookup((*doc).dict, name,
                                            -(1 as std::os::raw::c_int)) as
                                  *mut xmlChar)
    } else { cur = xmlNewNode(ns, name) }
    if !cur.is_null() {
        (*cur).doc = doc;
        if !content.is_null() {
            (*cur).children =
                xmlStringGetNodeList(doc as *const xmlDoc, content);
            if !cur.is_null() {
                let mut ulccur: xmlNodePtr = (*cur).children;
                if ulccur.is_null() {
                    (*cur).last = 0 as *mut _xmlNode
                } else {
                    while !(*ulccur).next.is_null() {
                        (*ulccur).parent = cur;
                        ulccur = (*ulccur).next
                    }
                    (*ulccur).parent = cur;
                    (*cur).last = ulccur
                }
            }
        }
    }
    return cur;
}
/* *
 * xmlNewDocNodeEatName:
 * @doc:  the document
 * @ns:  namespace if any
 * @name:  the node name
 * @content:  the XML text content if any
 *
 * Creation of a new node element within a document. @ns and @content
 * are optional (NULL).
 * NOTE: @content is supposed to be a piece of XML CDATA, so it allow entities
 *       references, but XML special chars need to be escaped first by using
 *       xmlEncodeEntitiesReentrant(). Use xmlNewDocRawNode() if you don't
 *       need entities support.
 *
 * Returns a pointer to the new node object.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlNewDocNodeEatName(mut doc: xmlDocPtr,
                                              mut ns: xmlNsPtr,
                                              mut name: *mut xmlChar,
                                              mut content: *const xmlChar)
 -> xmlNodePtr {
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    cur = xmlNewNodeEatName(ns, name);
    if !cur.is_null() {
        (*cur).doc = doc;
        if !content.is_null() {
            (*cur).children =
                xmlStringGetNodeList(doc as *const xmlDoc, content);
            if !cur.is_null() {
                let mut ulccur: xmlNodePtr = (*cur).children;
                if ulccur.is_null() {
                    (*cur).last = 0 as *mut _xmlNode
                } else {
                    while !(*ulccur).next.is_null() {
                        (*ulccur).parent = cur;
                        ulccur = (*ulccur).next
                    }
                    (*ulccur).parent = cur;
                    (*cur).last = ulccur
                }
            }
        }
    } else if !name.is_null() && !doc.is_null() &&
                  xmlDictOwns((*doc).dict, name) == 0 {
        xmlFree.expect("non-null function pointer")(name as
                                                        *mut std::os::raw::c_void);
    }
    return cur;
}
/* if name don't come from the doc dictionary free it here */
/* *
 * xmlNewDocRawNode:
 * @doc:  the document
 * @ns:  namespace if any
 * @name:  the node name
 * @content:  the text content if any
 *
 * Creation of a new node element within a document. @ns and @content
 * are optional (NULL).
 *
 * Returns a pointer to the new node object.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlNewDocRawNode(mut doc: xmlDocPtr,
                                          mut ns: xmlNsPtr,
                                          mut name: *const xmlChar,
                                          mut content: *const xmlChar)
 -> xmlNodePtr {
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    cur = xmlNewDocNode(doc, ns, name, 0 as *const xmlChar);
    if !cur.is_null() {
        (*cur).doc = doc;
        if !content.is_null() {
            (*cur).children = xmlNewDocText(doc as *const xmlDoc, content);
            if !cur.is_null() {
                let mut ulccur: xmlNodePtr = (*cur).children;
                if ulccur.is_null() {
                    (*cur).last = 0 as *mut _xmlNode
                } else {
                    while !(*ulccur).next.is_null() {
                        (*ulccur).parent = cur;
                        ulccur = (*ulccur).next
                    }
                    (*ulccur).parent = cur;
                    (*cur).last = ulccur
                }
            }
        }
    }
    return cur;
}
/* *
 * xmlNewDocFragment:
 * @doc:  the document owning the fragment
 *
 * Creation of a new Fragment node.
 * Returns a pointer to the new node object.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlNewDocFragment(mut doc: xmlDocPtr) -> xmlNodePtr {
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    /*
     * Allocate a new DocumentFragment node and fill the fields.
     */
    cur =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlNode>()
                                                          as std::os::raw::c_ulong) as
            xmlNodePtr;
    if cur.is_null() {
        xmlTreeErrMemory(b"building fragment\x00" as *const u8 as
                             *const std::os::raw::c_char);
        return 0 as xmlNodePtr
    }
    memset(cur as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<xmlNode>() as std::os::raw::c_ulong);
    (*cur).type_0 = XML_DOCUMENT_FRAG_NODE;
    (*cur).doc = doc;
    if __xmlRegisterCallbacks != 0 &&
           (*__xmlRegisterNodeDefaultValue()).is_some() {
        (*__xmlRegisterNodeDefaultValue()).expect("non-null function pointer")(cur);
    }
    return cur;
}
/* LIBXML_TREE_ENABLED */
/* *
 * xmlNewText:
 * @content:  the text content
 *
 * Creation of a new text node.
 * Returns a pointer to the new node object.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlNewText(mut content: *const xmlChar)
 -> xmlNodePtr {
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    /*
     * Allocate a new node and fill the fields.
     */
    cur =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlNode>()
                                                          as std::os::raw::c_ulong) as
            xmlNodePtr;
    if cur.is_null() {
        xmlTreeErrMemory(b"building text\x00" as *const u8 as
                             *const std::os::raw::c_char);
        return 0 as xmlNodePtr
    }
    memset(cur as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<xmlNode>() as std::os::raw::c_ulong);
    (*cur).type_0 = XML_TEXT_NODE;
    (*cur).name = xmlStringText.as_ptr();
    if !content.is_null() { (*cur).content = xmlStrdup(content) }
    if __xmlRegisterCallbacks != 0 &&
           (*__xmlRegisterNodeDefaultValue()).is_some() {
        (*__xmlRegisterNodeDefaultValue()).expect("non-null function pointer")(cur);
    }
    return cur;
}
/* *
 * xmlNewTextChild:
 * @parent:  the parent node
 * @ns:  a namespace if any
 * @name:  the name of the child
 * @content:  the text content of the child if any.
 *
 * Creation of a new child element, added at the end of @parent children list.
 * @ns and @content parameters are optional (NULL). If @ns is NULL, the newly
 * created element inherits the namespace of @parent. If @content is non NULL,
 * a child TEXT node will be created containing the string @content.
 * NOTE: Use xmlNewChild() if @content will contain entities that need to be
 * preserved. Use this function, xmlNewTextChild(), if you need to ensure that
 * reserved XML chars that might appear in @content, such as the ampersand,
 * greater-than or less-than signs, are automatically replaced by their XML
 * escaped entity representations.
 *
 * Returns a pointer to the new node object.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlNewTextChild(mut parent: xmlNodePtr,
                                         mut ns: xmlNsPtr,
                                         mut name: *const xmlChar,
                                         mut content: *const xmlChar)
 -> xmlNodePtr {
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    let mut prev: xmlNodePtr = 0 as *mut xmlNode;
    if parent.is_null() { return 0 as xmlNodePtr }
    if name.is_null() { return 0 as xmlNodePtr }
    /*
     * Allocate a new node
     */
    if (*parent).type_0 as std::os::raw::c_uint ==
           XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        if ns.is_null() {
            cur = xmlNewDocRawNode((*parent).doc, (*parent).ns, name, content)
        } else { cur = xmlNewDocRawNode((*parent).doc, ns, name, content) }
    } else if (*parent).type_0 as std::os::raw::c_uint ==
                  XML_DOCUMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint ||
                  (*parent).type_0 as std::os::raw::c_uint ==
                      XML_HTML_DOCUMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        if ns.is_null() {
            cur =
                xmlNewDocRawNode(parent as xmlDocPtr, 0 as xmlNsPtr, name,
                                 content)
        } else {
            cur = xmlNewDocRawNode(parent as xmlDocPtr, ns, name, content)
        }
    } else if (*parent).type_0 as std::os::raw::c_uint ==
                  XML_DOCUMENT_FRAG_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        cur = xmlNewDocRawNode((*parent).doc, ns, name, content)
    } else { return 0 as xmlNodePtr }
    if cur.is_null() { return 0 as xmlNodePtr }
    /*
     * add the new element at the end of the children list.
     */
    (*cur).type_0 = XML_ELEMENT_NODE;
    (*cur).parent = parent;
    (*cur).doc = (*parent).doc;
    if (*parent).children.is_null() {
        (*parent).children = cur;
        (*parent).last = cur
    } else {
        prev = (*parent).last;
        (*prev).next = cur;
        (*cur).prev = prev;
        (*parent).last = cur
    }
    return cur;
}
/* LIBXML_TREE_ENABLED */
/* *
 * xmlNewCharRef:
 * @doc: the document
 * @name:  the char ref string, starting with # or "&# ... ;"
 *
 * Creation of a new character reference node.
 * Returns a pointer to the new node object.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlNewCharRef(mut doc: xmlDocPtr,
                                       mut name: *const xmlChar)
 -> xmlNodePtr {
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    if name.is_null() { return 0 as xmlNodePtr }
    /*
     * Allocate a new node and fill the fields.
     */
    cur =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlNode>()
                                                          as std::os::raw::c_ulong) as
            xmlNodePtr;
    if cur.is_null() {
        xmlTreeErrMemory(b"building character reference\x00" as *const u8 as
                             *const std::os::raw::c_char);
        return 0 as xmlNodePtr
    }
    memset(cur as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<xmlNode>() as std::os::raw::c_ulong);
    (*cur).type_0 = XML_ENTITY_REF_NODE;
    (*cur).doc = doc;
    if *name.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int == '&' as i32 {
        let mut len: std::os::raw::c_int = 0;
        name = name.offset(1);
        len = xmlStrlen(name);
        if *name.offset((len - 1 as std::os::raw::c_int) as isize) as std::os::raw::c_int ==
               ';' as i32 {
            (*cur).name = xmlStrndup(name, len - 1 as std::os::raw::c_int)
        } else { (*cur).name = xmlStrndup(name, len) }
    } else { (*cur).name = xmlStrdup(name) }
    if __xmlRegisterCallbacks != 0 &&
           (*__xmlRegisterNodeDefaultValue()).is_some() {
        (*__xmlRegisterNodeDefaultValue()).expect("non-null function pointer")(cur);
    }
    return cur;
}
/* *
 * xmlNewReference:
 * @doc: the document
 * @name:  the reference name, or the reference string with & and ;
 *
 * Creation of a new reference node.
 * Returns a pointer to the new node object.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlNewReference(mut doc: *const xmlDoc,
                                         mut name: *const xmlChar)
 -> xmlNodePtr {
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    let mut ent: xmlEntityPtr = 0 as *mut xmlEntity;
    if name.is_null() { return 0 as xmlNodePtr }
    /*
     * Allocate a new node and fill the fields.
     */
    cur =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlNode>()
                                                          as std::os::raw::c_ulong) as
            xmlNodePtr;
    if cur.is_null() {
        xmlTreeErrMemory(b"building reference\x00" as *const u8 as
                             *const std::os::raw::c_char);
        return 0 as xmlNodePtr
    }
    memset(cur as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<xmlNode>() as std::os::raw::c_ulong);
    (*cur).type_0 = XML_ENTITY_REF_NODE;
    (*cur).doc = doc as *mut xmlDoc;
    if *name.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int == '&' as i32 {
        let mut len: std::os::raw::c_int = 0;
        name = name.offset(1);
        len = xmlStrlen(name);
        if *name.offset((len - 1 as std::os::raw::c_int) as isize) as std::os::raw::c_int ==
               ';' as i32 {
            (*cur).name = xmlStrndup(name, len - 1 as std::os::raw::c_int)
        } else { (*cur).name = xmlStrndup(name, len) }
    } else { (*cur).name = xmlStrdup(name) }
    ent = xmlGetDocEntity(doc, (*cur).name);
    if !ent.is_null() {
        (*cur).content = (*ent).content;
        /*
	 * The parent pointer in entity is a DTD pointer and thus is NOT
	 * updated.  Not sure if this is 100% correct.
	 *  -George
	 */
        (*cur).children = ent as xmlNodePtr;
        (*cur).last = ent as xmlNodePtr
    }
    if __xmlRegisterCallbacks != 0 &&
           (*__xmlRegisterNodeDefaultValue()).is_some() {
        (*__xmlRegisterNodeDefaultValue()).expect("non-null function pointer")(cur);
    }
    return cur;
}
/* *
 * xmlNewDocText:
 * @doc: the document
 * @content:  the text content
 *
 * Creation of a new text node within a document.
 * Returns a pointer to the new node object.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlNewDocText(mut doc: *const xmlDoc,
                                       mut content: *const xmlChar)
 -> xmlNodePtr {
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    cur = xmlNewText(content);
    if !cur.is_null() { (*cur).doc = doc as *mut xmlDoc }
    return cur;
}
/* *
 * xmlNewTextLen:
 * @content:  the text content
 * @len:  the text len.
 *
 * Creation of a new text node with an extra parameter for the content's length
 * Returns a pointer to the new node object.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlNewTextLen(mut content: *const xmlChar,
                                       mut len: std::os::raw::c_int) -> xmlNodePtr {
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    /*
     * Allocate a new node and fill the fields.
     */
    cur =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlNode>()
                                                          as std::os::raw::c_ulong) as
            xmlNodePtr;
    if cur.is_null() {
        xmlTreeErrMemory(b"building text\x00" as *const u8 as
                             *const std::os::raw::c_char);
        return 0 as xmlNodePtr
    }
    memset(cur as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<xmlNode>() as std::os::raw::c_ulong);
    (*cur).type_0 = XML_TEXT_NODE;
    (*cur).name = xmlStringText.as_ptr();
    if !content.is_null() { (*cur).content = xmlStrndup(content, len) }
    if __xmlRegisterCallbacks != 0 &&
           (*__xmlRegisterNodeDefaultValue()).is_some() {
        (*__xmlRegisterNodeDefaultValue()).expect("non-null function pointer")(cur);
    }
    return cur;
}
/* *
 * xmlNewDocTextLen:
 * @doc: the document
 * @content:  the text content
 * @len:  the text len.
 *
 * Creation of a new text node with an extra content length parameter. The
 * text node pertain to a given document.
 * Returns a pointer to the new node object.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlNewDocTextLen(mut doc: xmlDocPtr,
                                          mut content: *const xmlChar,
                                          mut len: std::os::raw::c_int)
 -> xmlNodePtr {
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    cur = xmlNewTextLen(content, len);
    if !cur.is_null() { (*cur).doc = doc }
    return cur;
}
/* *
 * xmlNewComment:
 * @content:  the comment content
 *
 * Creation of a new node containing a comment.
 * Returns a pointer to the new node object.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlNewComment(mut content: *const xmlChar)
 -> xmlNodePtr {
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    /*
     * Allocate a new node and fill the fields.
     */
    cur =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlNode>()
                                                          as std::os::raw::c_ulong) as
            xmlNodePtr;
    if cur.is_null() {
        xmlTreeErrMemory(b"building comment\x00" as *const u8 as
                             *const std::os::raw::c_char);
        return 0 as xmlNodePtr
    }
    memset(cur as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<xmlNode>() as std::os::raw::c_ulong);
    (*cur).type_0 = XML_COMMENT_NODE;
    (*cur).name = xmlStringComment.as_ptr();
    if !content.is_null() { (*cur).content = xmlStrdup(content) }
    if __xmlRegisterCallbacks != 0 &&
           (*__xmlRegisterNodeDefaultValue()).is_some() {
        (*__xmlRegisterNodeDefaultValue()).expect("non-null function pointer")(cur);
    }
    return cur;
}
/* *
 * xmlNewCDataBlock:
 * @doc:  the document
 * @content:  the CDATA block content content
 * @len:  the length of the block
 *
 * Creation of a new node containing a CDATA block.
 * Returns a pointer to the new node object.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlNewCDataBlock(mut doc: xmlDocPtr,
                                          mut content: *const xmlChar,
                                          mut len: std::os::raw::c_int)
 -> xmlNodePtr {
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    /*
     * Allocate a new node and fill the fields.
     */
    cur =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlNode>()
                                                          as std::os::raw::c_ulong) as
            xmlNodePtr;
    if cur.is_null() {
        xmlTreeErrMemory(b"building CDATA\x00" as *const u8 as
                             *const std::os::raw::c_char);
        return 0 as xmlNodePtr
    }
    memset(cur as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<xmlNode>() as std::os::raw::c_ulong);
    (*cur).type_0 = XML_CDATA_SECTION_NODE;
    (*cur).doc = doc;
    if !content.is_null() { (*cur).content = xmlStrndup(content, len) }
    if __xmlRegisterCallbacks != 0 &&
           (*__xmlRegisterNodeDefaultValue()).is_some() {
        (*__xmlRegisterNodeDefaultValue()).expect("non-null function pointer")(cur);
    }
    return cur;
}
/* *
 * xmlNewDocComment:
 * @doc:  the document
 * @content:  the comment content
 *
 * Creation of a new node containing a comment within a document.
 * Returns a pointer to the new node object.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlNewDocComment(mut doc: xmlDocPtr,
                                          mut content: *const xmlChar)
 -> xmlNodePtr {
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    cur = xmlNewComment(content);
    if !cur.is_null() { (*cur).doc = doc }
    return cur;
}
/* *
 * xmlSetTreeDoc:
 * @tree:  the top element
 * @doc:  the document
 *
 * update all nodes under the tree to point to the right document
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSetTreeDoc(mut tree: xmlNodePtr,
                                       mut doc: xmlDocPtr) {
    let mut prop: xmlAttrPtr = 0 as *mut xmlAttr;
    if tree.is_null() ||
           (*tree).type_0 as std::os::raw::c_uint ==
               XML_NAMESPACE_DECL as std::os::raw::c_int as std::os::raw::c_uint {
        return
    }
    if (*tree).doc != doc {
        if (*tree).type_0 as std::os::raw::c_uint ==
               XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
            prop = (*tree).properties;
            while !prop.is_null() {
                if (*prop).atype as std::os::raw::c_uint ==
                       XML_ATTRIBUTE_ID as std::os::raw::c_int as std::os::raw::c_uint {
                    xmlRemoveID((*tree).doc, prop);
                }
                (*prop).doc = doc;
                xmlSetListDoc((*prop).children, doc);
                /*
                 * TODO: ID attributes should be also added to the new
                 * document, but this breaks things like xmlReplaceNode.
                 * The underlying problem is that xmlRemoveID is only called
                 * if a node is destroyed, not if it's unlinked.
                 */
                prop = (*prop).next
            }
        }
        if !(*tree).children.is_null() {
            xmlSetListDoc((*tree).children, doc);
        }
        (*tree).doc = doc
    };
}
/* *
 * xmlSetListDoc:
 * @list:  the first element
 * @doc:  the document
 *
 * update all nodes in the list to point to the right document
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSetListDoc(mut list: xmlNodePtr,
                                       mut doc: xmlDocPtr) {
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    if list.is_null() ||
           (*list).type_0 as std::os::raw::c_uint ==
               XML_NAMESPACE_DECL as std::os::raw::c_int as std::os::raw::c_uint {
        return
    }
    cur = list;
    while !cur.is_null() {
        if (*cur).doc != doc { xmlSetTreeDoc(cur, doc); }
        cur = (*cur).next
    };
}
/* *
 * xmlNewChild:
 * @parent:  the parent node
 * @ns:  a namespace if any
 * @name:  the name of the child
 * @content:  the XML content of the child if any.
 *
 * Creation of a new child element, added at the end of @parent children list.
 * @ns and @content parameters are optional (NULL). If @ns is NULL, the newly
 * created element inherits the namespace of @parent. If @content is non NULL,
 * a child list containing the TEXTs and ENTITY_REFs node will be created.
 * NOTE: @content is supposed to be a piece of XML CDATA, so it allows entity
 *       references. XML special chars must be escaped first by using
 *       xmlEncodeEntitiesReentrant(), or xmlNewTextChild() should be used.
 *
 * Returns a pointer to the new node object.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlNewChild(mut parent: xmlNodePtr, mut ns: xmlNsPtr,
                                     mut name: *const xmlChar,
                                     mut content: *const xmlChar)
 -> xmlNodePtr {
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    let mut prev: xmlNodePtr = 0 as *mut xmlNode;
    if parent.is_null() { return 0 as xmlNodePtr }
    if name.is_null() { return 0 as xmlNodePtr }
    /*
     * Allocate a new node
     */
    if (*parent).type_0 as std::os::raw::c_uint ==
           XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        if ns.is_null() {
            cur = xmlNewDocNode((*parent).doc, (*parent).ns, name, content)
        } else { cur = xmlNewDocNode((*parent).doc, ns, name, content) }
    } else if (*parent).type_0 as std::os::raw::c_uint ==
                  XML_DOCUMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint ||
                  (*parent).type_0 as std::os::raw::c_uint ==
                      XML_HTML_DOCUMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        if ns.is_null() {
            cur =
                xmlNewDocNode(parent as xmlDocPtr, 0 as xmlNsPtr, name,
                              content)
        } else { cur = xmlNewDocNode(parent as xmlDocPtr, ns, name, content) }
    } else if (*parent).type_0 as std::os::raw::c_uint ==
                  XML_DOCUMENT_FRAG_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        cur = xmlNewDocNode((*parent).doc, ns, name, content)
    } else { return 0 as xmlNodePtr }
    if cur.is_null() { return 0 as xmlNodePtr }
    /*
     * add the new element at the end of the children list.
     */
    (*cur).type_0 = XML_ELEMENT_NODE;
    (*cur).parent = parent;
    (*cur).doc = (*parent).doc;
    if (*parent).children.is_null() {
        (*parent).children = cur;
        (*parent).last = cur
    } else {
        prev = (*parent).last;
        (*prev).next = cur;
        (*cur).prev = prev;
        (*parent).last = cur
    }
    return cur;
}
/* LIBXML_TREE_ENABLED */
/* *
 * xmlAddPropSibling:
 * @prev:  the attribute to which @prop is added after
 * @cur:   the base attribute passed to calling function
 * @prop:  the new attribute
 *
 * Add a new attribute after @prev using @cur as base attribute.
 * When inserting before @cur, @prev is passed as @cur->prev.
 * When inserting after @cur, @prev is passed as @cur.
 * If an existing attribute is found it is detroyed prior to adding @prop.
 *
 * Returns the attribute being inserted or NULL in case of error.
 */
unsafe extern "C" fn xmlAddPropSibling(mut prev: xmlNodePtr,
                                       mut cur: xmlNodePtr,
                                       mut prop: xmlNodePtr) -> xmlNodePtr {
    let mut attr: xmlAttrPtr = 0 as *mut xmlAttr;
    if cur.is_null() ||
           (*cur).type_0 as std::os::raw::c_uint !=
               XML_ATTRIBUTE_NODE as std::os::raw::c_int as std::os::raw::c_uint ||
           prop.is_null() ||
           (*prop).type_0 as std::os::raw::c_uint !=
               XML_ATTRIBUTE_NODE as std::os::raw::c_int as std::os::raw::c_uint ||
           !prev.is_null() &&
               (*prev).type_0 as std::os::raw::c_uint !=
                   XML_ATTRIBUTE_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        return 0 as xmlNodePtr
    }
    /* check if an attribute with the same name exists */
    if (*prop).ns.is_null() {
        attr = xmlHasNsProp((*cur).parent, (*prop).name, 0 as *const xmlChar)
    } else {
        attr = xmlHasNsProp((*cur).parent, (*prop).name, (*(*prop).ns).href)
    }
    if (*prop).doc != (*cur).doc { xmlSetTreeDoc(prop, (*cur).doc); }
    (*prop).parent = (*cur).parent;
    (*prop).prev = prev;
    if !prev.is_null() {
        (*prop).next = (*prev).next;
        (*prev).next = prop;
        if !(*prop).next.is_null() { (*(*prop).next).prev = prop }
    } else { (*prop).next = cur; (*cur).prev = prop }
    if (*prop).prev.is_null() && !(*prop).parent.is_null() {
        (*(*prop).parent).properties = prop as xmlAttrPtr
    }
    if !attr.is_null() &&
           (*attr).type_0 as std::os::raw::c_uint !=
               XML_ATTRIBUTE_DECL as std::os::raw::c_int as std::os::raw::c_uint {
        /* different instance, destroy it (attributes must be unique) */
        xmlRemoveProp(attr);
    }
    return prop;
}
/* *
 * xmlAddNextSibling:
 * @cur:  the child node
 * @elem:  the new node
 *
 * Add a new node @elem as the next sibling of @cur
 * If the new node was already inserted in a document it is
 * first unlinked from its existing context.
 * As a result of text merging @elem may be freed.
 * If the new node is ATTRIBUTE, it is added into properties instead of children.
 * If there is an attribute with equal name, it is first destroyed.
 *
 * Returns the new node or NULL in case of error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlAddNextSibling(mut cur: xmlNodePtr,
                                           mut elem: xmlNodePtr)
 -> xmlNodePtr {
    if cur.is_null() ||
           (*cur).type_0 as std::os::raw::c_uint ==
               XML_NAMESPACE_DECL as std::os::raw::c_int as std::os::raw::c_uint {
        return 0 as xmlNodePtr
    }
    if elem.is_null() ||
           (*elem).type_0 as std::os::raw::c_uint ==
               XML_NAMESPACE_DECL as std::os::raw::c_int as std::os::raw::c_uint {
        return 0 as xmlNodePtr
    }
    if cur == elem { return 0 as xmlNodePtr }
    xmlUnlinkNode(elem);
    if (*elem).type_0 as std::os::raw::c_uint ==
           XML_TEXT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        if (*cur).type_0 as std::os::raw::c_uint ==
               XML_TEXT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
            xmlNodeAddContent(cur, (*elem).content);
            xmlFreeNode(elem);
            return cur
        }
        if !(*cur).next.is_null() &&
               (*(*cur).next).type_0 as std::os::raw::c_uint ==
                   XML_TEXT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
               (*cur).name == (*(*cur).next).name {
            let mut tmp: *mut xmlChar = 0 as *mut xmlChar;
            tmp = xmlStrdup((*elem).content);
            tmp = xmlStrcat(tmp, (*(*cur).next).content);
            xmlNodeSetContent((*cur).next, tmp);
            xmlFree.expect("non-null function pointer")(tmp as
                                                            *mut std::os::raw::c_void);
            xmlFreeNode(elem);
            return (*cur).next
        }
    } else if (*elem).type_0 as std::os::raw::c_uint ==
                  XML_ATTRIBUTE_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        return xmlAddPropSibling(cur, cur, elem)
    }
    if (*elem).doc != (*cur).doc { xmlSetTreeDoc(elem, (*cur).doc); }
    (*elem).parent = (*cur).parent;
    (*elem).prev = cur;
    (*elem).next = (*cur).next;
    (*cur).next = elem;
    if !(*elem).next.is_null() { (*(*elem).next).prev = elem }
    if !(*elem).parent.is_null() && (*(*elem).parent).last == cur {
        (*(*elem).parent).last = elem
    }
    return elem;
}
/* *
 * xmlAddPrevSibling:
 * @cur:  the child node
 * @elem:  the new node
 *
 * Add a new node @elem as the previous sibling of @cur
 * merging adjacent TEXT nodes (@elem may be freed)
 * If the new node was already inserted in a document it is
 * first unlinked from its existing context.
 * If the new node is ATTRIBUTE, it is added into properties instead of children.
 * If there is an attribute with equal name, it is first destroyed.
 *
 * Returns the new node or NULL in case of error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlAddPrevSibling(mut cur: xmlNodePtr,
                                           mut elem: xmlNodePtr)
 -> xmlNodePtr {
    if cur.is_null() ||
           (*cur).type_0 as std::os::raw::c_uint ==
               XML_NAMESPACE_DECL as std::os::raw::c_int as std::os::raw::c_uint {
        return 0 as xmlNodePtr
    }
    if elem.is_null() ||
           (*elem).type_0 as std::os::raw::c_uint ==
               XML_NAMESPACE_DECL as std::os::raw::c_int as std::os::raw::c_uint {
        return 0 as xmlNodePtr
    }
    if cur == elem { return 0 as xmlNodePtr }
    xmlUnlinkNode(elem);
    if (*elem).type_0 as std::os::raw::c_uint ==
           XML_TEXT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        if (*cur).type_0 as std::os::raw::c_uint ==
               XML_TEXT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
            let mut tmp: *mut xmlChar = 0 as *mut xmlChar;
            tmp = xmlStrdup((*elem).content);
            tmp = xmlStrcat(tmp, (*cur).content);
            xmlNodeSetContent(cur, tmp);
            xmlFree.expect("non-null function pointer")(tmp as
                                                            *mut std::os::raw::c_void);
            xmlFreeNode(elem);
            return cur
        }
        if !(*cur).prev.is_null() &&
               (*(*cur).prev).type_0 as std::os::raw::c_uint ==
                   XML_TEXT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
               (*cur).name == (*(*cur).prev).name {
            xmlNodeAddContent((*cur).prev, (*elem).content);
            xmlFreeNode(elem);
            return (*cur).prev
        }
    } else if (*elem).type_0 as std::os::raw::c_uint ==
                  XML_ATTRIBUTE_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        return xmlAddPropSibling((*cur).prev, cur, elem)
    }
    if (*elem).doc != (*cur).doc { xmlSetTreeDoc(elem, (*cur).doc); }
    (*elem).parent = (*cur).parent;
    (*elem).next = cur;
    (*elem).prev = (*cur).prev;
    (*cur).prev = elem;
    if !(*elem).prev.is_null() { (*(*elem).prev).next = elem }
    if !(*elem).parent.is_null() && (*(*elem).parent).children == cur {
        (*(*elem).parent).children = elem
    }
    return elem;
}
/* LIBXML_TREE_ENABLED */
/* *
 * xmlAddSibling:
 * @cur:  the child node
 * @elem:  the new node
 *
 * Add a new element @elem to the list of siblings of @cur
 * merging adjacent TEXT nodes (@elem may be freed)
 * If the new element was already inserted in a document it is
 * first unlinked from its existing context.
 *
 * Returns the new element or NULL in case of error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlAddSibling(mut cur: xmlNodePtr,
                                       mut elem: xmlNodePtr) -> xmlNodePtr {
    let mut parent: xmlNodePtr = 0 as *mut xmlNode;
    if cur.is_null() ||
           (*cur).type_0 as std::os::raw::c_uint ==
               XML_NAMESPACE_DECL as std::os::raw::c_int as std::os::raw::c_uint {
        return 0 as xmlNodePtr
    }
    if elem.is_null() ||
           (*elem).type_0 as std::os::raw::c_uint ==
               XML_NAMESPACE_DECL as std::os::raw::c_int as std::os::raw::c_uint {
        return 0 as xmlNodePtr
    }
    if cur == elem { return 0 as xmlNodePtr }
    /*
     * Constant time is we can rely on the ->parent->last to find
     * the last sibling.
     */
    if (*cur).type_0 as std::os::raw::c_uint !=
           XML_ATTRIBUTE_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
           !(*cur).parent.is_null() && !(*(*cur).parent).children.is_null() &&
           !(*(*cur).parent).last.is_null() &&
           (*(*(*cur).parent).last).next.is_null() {
        cur = (*(*cur).parent).last
    } else { while !(*cur).next.is_null() { cur = (*cur).next } }
    xmlUnlinkNode(elem);
    if (*cur).type_0 as std::os::raw::c_uint ==
           XML_TEXT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
           (*elem).type_0 as std::os::raw::c_uint ==
               XML_TEXT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
           (*cur).name == (*elem).name {
        xmlNodeAddContent(cur, (*elem).content);
        xmlFreeNode(elem);
        return cur
    } else {
        if (*elem).type_0 as std::os::raw::c_uint ==
               XML_ATTRIBUTE_NODE as std::os::raw::c_int as std::os::raw::c_uint {
            return xmlAddPropSibling(cur, cur, elem)
        }
    }
    if (*elem).doc != (*cur).doc { xmlSetTreeDoc(elem, (*cur).doc); }
    parent = (*cur).parent;
    (*elem).prev = cur;
    (*elem).next = 0 as *mut _xmlNode;
    (*elem).parent = parent;
    (*cur).next = elem;
    if !parent.is_null() { (*parent).last = elem }
    return elem;
}
/* *
 * xmlAddChildList:
 * @parent:  the parent node
 * @cur:  the first node in the list
 *
 * Add a list of node at the end of the child list of the parent
 * merging adjacent TEXT nodes (@cur may be freed)
 *
 * Returns the last child or NULL in case of error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlAddChildList(mut parent: xmlNodePtr,
                                         mut cur: xmlNodePtr) -> xmlNodePtr {
    let mut prev: xmlNodePtr = 0 as *mut xmlNode;
    if parent.is_null() ||
           (*parent).type_0 as std::os::raw::c_uint ==
               XML_NAMESPACE_DECL as std::os::raw::c_int as std::os::raw::c_uint {
        return 0 as xmlNodePtr
    }
    if cur.is_null() ||
           (*cur).type_0 as std::os::raw::c_uint ==
               XML_NAMESPACE_DECL as std::os::raw::c_int as std::os::raw::c_uint {
        return 0 as xmlNodePtr
    }
    (!(*cur).doc.is_null() && !(*parent).doc.is_null()) &&
        (*cur).doc != (*parent).doc;
    /*
     * add the first element at the end of the children list.
     */
    if (*parent).children.is_null() {
        (*parent).children = cur
    } else {
        /*
	 * If cur and parent->last both are TEXT nodes, then merge them.
	 */
        if (*cur).type_0 as std::os::raw::c_uint ==
               XML_TEXT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
               (*(*parent).last).type_0 as std::os::raw::c_uint ==
                   XML_TEXT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
               (*cur).name == (*(*parent).last).name {
            xmlNodeAddContent((*parent).last, (*cur).content);
            /*
	     * if it's the only child, nothing more to be done.
	     */
            if (*cur).next.is_null() {
                xmlFreeNode(cur);
                return (*parent).last
            }
            prev = cur;
            cur = (*cur).next;
            xmlFreeNode(prev);
        }
        prev = (*parent).last;
        (*prev).next = cur;
        (*cur).prev = prev
    }
    while !(*cur).next.is_null() {
        (*cur).parent = parent;
        if (*cur).doc != (*parent).doc { xmlSetTreeDoc(cur, (*parent).doc); }
        cur = (*cur).next
    }
    (*cur).parent = parent;
    /* the parent may not be linked to a doc ! */
    if (*cur).doc != (*parent).doc { xmlSetTreeDoc(cur, (*parent).doc); }
    (*parent).last = cur;
    return cur;
}
/* *
 * xmlAddChild:
 * @parent:  the parent node
 * @cur:  the child node
 *
 * Add a new node to @parent, at the end of the child (or property) list
 * merging adjacent TEXT nodes (in which case @cur is freed)
 * If the new node is ATTRIBUTE, it is added into properties instead of children.
 * If there is an attribute with equal name, it is first destroyed.
 *
 * Returns the child or NULL in case of error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlAddChild(mut parent: xmlNodePtr,
                                     mut cur: xmlNodePtr) -> xmlNodePtr {
    let mut prev: xmlNodePtr = 0 as *mut xmlNode;
    if parent.is_null() ||
           (*parent).type_0 as std::os::raw::c_uint ==
               XML_NAMESPACE_DECL as std::os::raw::c_int as std::os::raw::c_uint {
        return 0 as xmlNodePtr
    }
    if cur.is_null() ||
           (*cur).type_0 as std::os::raw::c_uint ==
               XML_NAMESPACE_DECL as std::os::raw::c_int as std::os::raw::c_uint {
        return 0 as xmlNodePtr
    }
    if parent == cur { return 0 as xmlNodePtr }
    /*
     * If cur is a TEXT node, merge its content with adjacent TEXT nodes
     * cur is then freed.
     */
    if (*cur).type_0 as std::os::raw::c_uint ==
           XML_TEXT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        if (*parent).type_0 as std::os::raw::c_uint ==
               XML_TEXT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
               !(*parent).content.is_null() && (*parent).name == (*cur).name {
            xmlNodeAddContent(parent, (*cur).content);
            xmlFreeNode(cur);
            return parent
        }
        if !(*parent).last.is_null() &&
               (*(*parent).last).type_0 as std::os::raw::c_uint ==
                   XML_TEXT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
               (*(*parent).last).name == (*cur).name && (*parent).last != cur
           {
            xmlNodeAddContent((*parent).last, (*cur).content);
            xmlFreeNode(cur);
            return (*parent).last
        }
    }
    /*
     * add the new element at the end of the children list.
     */
    prev = (*cur).parent;
    (*cur).parent = parent;
    if (*cur).doc != (*parent).doc { xmlSetTreeDoc(cur, (*parent).doc); }
    /* this check prevents a loop on tree-traversions if a developer
     * tries to add a node to its parent multiple times
     */
    if prev == parent { return cur }
    /*
     * Coalescing
     */
    if (*parent).type_0 as std::os::raw::c_uint ==
           XML_TEXT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
           !(*parent).content.is_null() && parent != cur {
        xmlNodeAddContent(parent, (*cur).content);
        xmlFreeNode(cur);
        return parent
    }
    if (*cur).type_0 as std::os::raw::c_uint ==
           XML_ATTRIBUTE_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        if (*parent).type_0 as std::os::raw::c_uint !=
               XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
            return 0 as xmlNodePtr
        }
        if !(*parent).properties.is_null() {
            /* check if an attribute with the same name exists */
            let mut lastattr: xmlAttrPtr = 0 as *mut xmlAttr;
            if (*cur).ns.is_null() {
                lastattr =
                    xmlHasNsProp(parent as *const xmlNode, (*cur).name,
                                 0 as *const xmlChar)
            } else {
                lastattr =
                    xmlHasNsProp(parent as *const xmlNode, (*cur).name,
                                 (*(*cur).ns).href)
            }
            if !lastattr.is_null() && lastattr != cur as xmlAttrPtr &&
                   (*lastattr).type_0 as std::os::raw::c_uint !=
                       XML_ATTRIBUTE_DECL as std::os::raw::c_int as std::os::raw::c_uint {
                /* different instance, destroy it (attributes must be unique) */
                xmlUnlinkNode(lastattr as xmlNodePtr);
                xmlFreeProp(lastattr);
            }
            if lastattr == cur as xmlAttrPtr { return cur }
        }
        if (*parent).properties.is_null() {
            (*parent).properties = cur as xmlAttrPtr
        } else {
            /* find the end */
            let mut lastattr_0: xmlAttrPtr = (*parent).properties;
            while !(*lastattr_0).next.is_null() {
                lastattr_0 = (*lastattr_0).next
            }
            (*lastattr_0).next = cur as xmlAttrPtr;
            let ref mut fresh0 = (*(cur as xmlAttrPtr)).prev;
            *fresh0 = lastattr_0
        }
    } else if (*parent).children.is_null() {
        (*parent).children = cur;
        (*parent).last = cur
    } else {
        prev = (*parent).last;
        (*prev).next = cur;
        (*cur).prev = prev;
        (*parent).last = cur
    }
    return cur;
}
/* *
 * xmlGetLastChild:
 * @parent:  the parent node
 *
 * Search the last child of a node.
 * Returns the last child or NULL if none.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlGetLastChild(mut parent: *const xmlNode)
 -> xmlNodePtr {
    if parent.is_null() ||
           (*parent).type_0 as std::os::raw::c_uint ==
               XML_NAMESPACE_DECL as std::os::raw::c_int as std::os::raw::c_uint {
        return 0 as xmlNodePtr
    }
    return (*parent).last;
}
/*
 * 5 interfaces from DOM ElementTraversal
 */
/* *
 * xmlChildElementCount:
 * @parent: the parent node
 *
 * Finds the current number of child nodes of that element which are
 * element nodes.
 * Note the handling of entities references is different than in
 * the W3C DOM element traversal spec since we don't have back reference
 * from entities content to entities references.
 *
 * Returns the count of element child or 0 if not available
 */
#[no_mangle]
pub unsafe extern "C" fn xmlChildElementCount(mut parent: xmlNodePtr)
 -> std::os::raw::c_ulong {
    let mut ret: std::os::raw::c_ulong = 0 as std::os::raw::c_int as std::os::raw::c_ulong;
    let mut cur: xmlNodePtr = 0 as xmlNodePtr;
    if parent.is_null() { return 0 as std::os::raw::c_int as std::os::raw::c_ulong }
    match (*parent).type_0 as std::os::raw::c_uint {
        1 | 6 | 9 | 11 | 13 => { cur = (*parent).children }
        _ => { return 0 as std::os::raw::c_int as std::os::raw::c_ulong }
    }
    while !cur.is_null() {
        if (*cur).type_0 as std::os::raw::c_uint ==
               XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
            ret = ret.wrapping_add(1)
        }
        cur = (*cur).next
    }
    return ret;
}
/* *
 * xmlFirstElementChild:
 * @parent: the parent node
 *
 * Finds the first child node of that element which is a Element node
 * Note the handling of entities references is different than in
 * the W3C DOM element traversal spec since we don't have back reference
 * from entities content to entities references.
 *
 * Returns the first element child or NULL if not available
 */
#[no_mangle]
pub unsafe extern "C" fn xmlFirstElementChild(mut parent: xmlNodePtr)
 -> xmlNodePtr {
    let mut cur: xmlNodePtr = 0 as xmlNodePtr;
    if parent.is_null() { return 0 as xmlNodePtr }
    match (*parent).type_0 as std::os::raw::c_uint {
        1 | 6 | 9 | 11 | 13 => { cur = (*parent).children }
        _ => { return 0 as xmlNodePtr }
    }
    while !cur.is_null() {
        if (*cur).type_0 as std::os::raw::c_uint ==
               XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
            return cur
        }
        cur = (*cur).next
    }
    return 0 as xmlNodePtr;
}
/* *
 * xmlLastElementChild:
 * @parent: the parent node
 *
 * Finds the last child node of that element which is a Element node
 * Note the handling of entities references is different than in
 * the W3C DOM element traversal spec since we don't have back reference
 * from entities content to entities references.
 *
 * Returns the last element child or NULL if not available
 */
#[no_mangle]
pub unsafe extern "C" fn xmlLastElementChild(mut parent: xmlNodePtr)
 -> xmlNodePtr {
    let mut cur: xmlNodePtr = 0 as xmlNodePtr;
    if parent.is_null() { return 0 as xmlNodePtr }
    match (*parent).type_0 as std::os::raw::c_uint {
        1 | 6 | 9 | 11 | 13 => { cur = (*parent).last }
        _ => { return 0 as xmlNodePtr }
    }
    while !cur.is_null() {
        if (*cur).type_0 as std::os::raw::c_uint ==
               XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
            return cur
        }
        cur = (*cur).prev
    }
    return 0 as xmlNodePtr;
}
/* *
 * xmlPreviousElementSibling:
 * @node: the current node
 *
 * Finds the first closest previous sibling of the node which is an
 * element node.
 * Note the handling of entities references is different than in
 * the W3C DOM element traversal spec since we don't have back reference
 * from entities content to entities references.
 *
 * Returns the previous element sibling or NULL if not available
 */
#[no_mangle]
pub unsafe extern "C" fn xmlPreviousElementSibling(mut node: xmlNodePtr)
 -> xmlNodePtr {
    if node.is_null() { return 0 as xmlNodePtr }
    match (*node).type_0 as std::os::raw::c_uint {
        1 | 3 | 4 | 5 | 6 | 7 | 8 | 19 | 20 => { node = (*node).prev }
        _ => { return 0 as xmlNodePtr }
    }
    while !node.is_null() {
        if (*node).type_0 as std::os::raw::c_uint ==
               XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
            return node
        }
        node = (*node).prev
    }
    return 0 as xmlNodePtr;
}
/*
 * 5 interfaces from DOM ElementTraversal, but different in entities
 * traversal.
 */
/* *
 * xmlNextElementSibling:
 * @node: the current node
 *
 * Finds the first closest next sibling of the node which is an
 * element node.
 * Note the handling of entities references is different than in
 * the W3C DOM element traversal spec since we don't have back reference
 * from entities content to entities references.
 *
 * Returns the next element sibling or NULL if not available
 */
#[no_mangle]
pub unsafe extern "C" fn xmlNextElementSibling(mut node: xmlNodePtr)
 -> xmlNodePtr {
    if node.is_null() { return 0 as xmlNodePtr }
    match (*node).type_0 as std::os::raw::c_uint {
        1 | 3 | 4 | 5 | 6 | 7 | 8 | 14 | 19 | 20 => { node = (*node).next }
        _ => { return 0 as xmlNodePtr }
    }
    while !node.is_null() {
        if (*node).type_0 as std::os::raw::c_uint ==
               XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
            return node
        }
        node = (*node).next
    }
    return 0 as xmlNodePtr;
}
/* LIBXML_TREE_ENABLED */
/* *
 * xmlFreeNodeList:
 * @cur:  the first node in the list
 *
 * Free a node and all its siblings, this is a recursive behaviour, all
 * the children are freed too.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlFreeNodeList(mut cur: xmlNodePtr) {
    let mut next: xmlNodePtr = 0 as *mut xmlNode;
    let mut dict: xmlDictPtr = 0 as xmlDictPtr;
    if cur.is_null() { return }
    if (*cur).type_0 as std::os::raw::c_uint ==
           XML_NAMESPACE_DECL as std::os::raw::c_int as std::os::raw::c_uint {
        xmlFreeNsList(cur as xmlNsPtr);
        return
    }
    if (*cur).type_0 as std::os::raw::c_uint ==
           XML_DOCUMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint ||
           (*cur).type_0 as std::os::raw::c_uint ==
               XML_DOCB_DOCUMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint ||
           (*cur).type_0 as std::os::raw::c_uint ==
               XML_HTML_DOCUMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        xmlFreeDoc(cur as xmlDocPtr);
        return
    }
    if !(*cur).doc.is_null() { dict = (*(*cur).doc).dict }
    while !cur.is_null() {
        next = (*cur).next;
        if (*cur).type_0 as std::os::raw::c_uint !=
               XML_DTD_NODE as std::os::raw::c_int as std::os::raw::c_uint {
            if __xmlRegisterCallbacks != 0 &&
                   (*__xmlDeregisterNodeDefaultValue()).is_some() {
                (*__xmlDeregisterNodeDefaultValue()).expect("non-null function pointer")(cur);
            }
            if !(*cur).children.is_null() &&
                   (*cur).type_0 as std::os::raw::c_uint !=
                       XML_ENTITY_REF_NODE as std::os::raw::c_int as std::os::raw::c_uint {
                xmlFreeNodeList((*cur).children);
            }
            if ((*cur).type_0 as std::os::raw::c_uint ==
                    XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint ||
                    (*cur).type_0 as std::os::raw::c_uint ==
                        XML_XINCLUDE_START as std::os::raw::c_int as std::os::raw::c_uint ||
                    (*cur).type_0 as std::os::raw::c_uint ==
                        XML_XINCLUDE_END as std::os::raw::c_int as std::os::raw::c_uint) &&
                   !(*cur).properties.is_null() {
                xmlFreePropList((*cur).properties);
            }
            if (*cur).type_0 as std::os::raw::c_uint !=
                   XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
                   (*cur).type_0 as std::os::raw::c_uint !=
                       XML_XINCLUDE_START as std::os::raw::c_int as std::os::raw::c_uint &&
                   (*cur).type_0 as std::os::raw::c_uint !=
                       XML_XINCLUDE_END as std::os::raw::c_int as std::os::raw::c_uint &&
                   (*cur).type_0 as std::os::raw::c_uint !=
                       XML_ENTITY_REF_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
                   (*cur).content !=
                       &mut (*cur).properties as *mut *mut _xmlAttr as
                           *mut xmlChar {
                if !(*cur).content.is_null() &&
                       (dict.is_null() ||
                            xmlDictOwns(dict,
                                        (*cur).content as *const xmlChar) ==
                                0 as std::os::raw::c_int) {
                    xmlFree.expect("non-null function pointer")((*cur).content
                                                                    as
                                                                    *mut std::os::raw::c_char
                                                                    as
                                                                    *mut std::os::raw::c_void);
                }
            }
            if ((*cur).type_0 as std::os::raw::c_uint ==
                    XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint ||
                    (*cur).type_0 as std::os::raw::c_uint ==
                        XML_XINCLUDE_START as std::os::raw::c_int as std::os::raw::c_uint ||
                    (*cur).type_0 as std::os::raw::c_uint ==
                        XML_XINCLUDE_END as std::os::raw::c_int as std::os::raw::c_uint) &&
                   !(*cur).nsDef.is_null() {
                xmlFreeNsList((*cur).nsDef);
            }
            /*
	     * When a node is a text node or a comment, it uses a global static
	     * variable for the name of the node.
	     * Otherwise the node name might come from the document's
	     * dictionary
	     */
            if !(*cur).name.is_null() &&
                   (*cur).type_0 as std::os::raw::c_uint !=
                       XML_TEXT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
                   (*cur).type_0 as std::os::raw::c_uint !=
                       XML_COMMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
                if !(*cur).name.is_null() &&
                       (dict.is_null() ||
                            xmlDictOwns(dict, (*cur).name) ==
                                0 as std::os::raw::c_int) {
                    xmlFree.expect("non-null function pointer")((*cur).name as
                                                                    *mut std::os::raw::c_char
                                                                    as
                                                                    *mut std::os::raw::c_void);
                }
            }
            xmlFree.expect("non-null function pointer")(cur as
                                                            *mut std::os::raw::c_void);
        }
        cur = next
    };
}
/* *
 * xmlFreeNode:
 * @cur:  the node
 *
 * Free a node, this is a recursive behaviour, all the children are freed too.
 * This doesn't unlink the child from the list, use xmlUnlinkNode() first.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlFreeNode(mut cur: xmlNodePtr) {
    let mut dict: xmlDictPtr = 0 as xmlDictPtr;
    if cur.is_null() { return }
    /* use xmlFreeDtd for DTD nodes */
    if (*cur).type_0 as std::os::raw::c_uint ==
           XML_DTD_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        xmlFreeDtd(cur as xmlDtdPtr);
        return
    }
    if (*cur).type_0 as std::os::raw::c_uint ==
           XML_NAMESPACE_DECL as std::os::raw::c_int as std::os::raw::c_uint {
        xmlFreeNs(cur as xmlNsPtr);
        return
    }
    if (*cur).type_0 as std::os::raw::c_uint ==
           XML_ATTRIBUTE_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        xmlFreeProp(cur as xmlAttrPtr);
        return
    }
    if __xmlRegisterCallbacks != 0 &&
           (*__xmlDeregisterNodeDefaultValue()).is_some() {
        (*__xmlDeregisterNodeDefaultValue()).expect("non-null function pointer")(cur);
    }
    if !(*cur).doc.is_null() { dict = (*(*cur).doc).dict }
    if (*cur).type_0 as std::os::raw::c_uint ==
           XML_ENTITY_DECL as std::os::raw::c_int as std::os::raw::c_uint {
        let mut ent: xmlEntityPtr = cur as xmlEntityPtr;
        if !(*ent).SystemID.is_null() &&
               (dict.is_null() ||
                    xmlDictOwns(dict, (*ent).SystemID) == 0 as std::os::raw::c_int) {
            xmlFree.expect("non-null function pointer")((*ent).SystemID as
                                                            *mut std::os::raw::c_char
                                                            as
                                                            *mut std::os::raw::c_void);
        }
        if !(*ent).ExternalID.is_null() &&
               (dict.is_null() ||
                    xmlDictOwns(dict, (*ent).ExternalID) == 0 as std::os::raw::c_int)
           {
            xmlFree.expect("non-null function pointer")((*ent).ExternalID as
                                                            *mut std::os::raw::c_char
                                                            as
                                                            *mut std::os::raw::c_void);
        }
    }
    if !(*cur).children.is_null() &&
           (*cur).type_0 as std::os::raw::c_uint !=
               XML_ENTITY_REF_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        xmlFreeNodeList((*cur).children);
    }
    if ((*cur).type_0 as std::os::raw::c_uint ==
            XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint ||
            (*cur).type_0 as std::os::raw::c_uint ==
                XML_XINCLUDE_START as std::os::raw::c_int as std::os::raw::c_uint ||
            (*cur).type_0 as std::os::raw::c_uint ==
                XML_XINCLUDE_END as std::os::raw::c_int as std::os::raw::c_uint) &&
           !(*cur).properties.is_null() {
        xmlFreePropList((*cur).properties);
    }
    if (*cur).type_0 as std::os::raw::c_uint !=
           XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
           !(*cur).content.is_null() &&
           (*cur).type_0 as std::os::raw::c_uint !=
               XML_ENTITY_REF_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
           (*cur).type_0 as std::os::raw::c_uint !=
               XML_XINCLUDE_END as std::os::raw::c_int as std::os::raw::c_uint &&
           (*cur).type_0 as std::os::raw::c_uint !=
               XML_XINCLUDE_START as std::os::raw::c_int as std::os::raw::c_uint &&
           (*cur).content !=
               &mut (*cur).properties as *mut *mut _xmlAttr as *mut xmlChar {
        if !(*cur).content.is_null() &&
               (dict.is_null() ||
                    xmlDictOwns(dict, (*cur).content as *const xmlChar) ==
                        0 as std::os::raw::c_int) {
            xmlFree.expect("non-null function pointer")((*cur).content as
                                                            *mut std::os::raw::c_char
                                                            as
                                                            *mut std::os::raw::c_void);
        }
    }
    /*
     * When a node is a text node or a comment, it uses a global static
     * variable for the name of the node.
     * Otherwise the node name might come from the document's dictionary
     */
    if !(*cur).name.is_null() &&
           (*cur).type_0 as std::os::raw::c_uint !=
               XML_TEXT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
           (*cur).type_0 as std::os::raw::c_uint !=
               XML_COMMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        if !(*cur).name.is_null() &&
               (dict.is_null() ||
                    xmlDictOwns(dict, (*cur).name) == 0 as std::os::raw::c_int) {
            xmlFree.expect("non-null function pointer")((*cur).name as
                                                            *mut std::os::raw::c_char
                                                            as
                                                            *mut std::os::raw::c_void);
        }
    }
    if ((*cur).type_0 as std::os::raw::c_uint ==
            XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint ||
            (*cur).type_0 as std::os::raw::c_uint ==
                XML_XINCLUDE_START as std::os::raw::c_int as std::os::raw::c_uint ||
            (*cur).type_0 as std::os::raw::c_uint ==
                XML_XINCLUDE_END as std::os::raw::c_int as std::os::raw::c_uint) &&
           !(*cur).nsDef.is_null() {
        xmlFreeNsList((*cur).nsDef);
    }
    xmlFree.expect("non-null function pointer")(cur as *mut std::os::raw::c_void);
}
/* *
 * xmlUnlinkNode:
 * @cur:  the node
 *
 * Unlink a node from it's current context, the node is not freed
 * If one need to free the node, use xmlFreeNode() routine after the
 * unlink to discard it.
 * Note that namespace nodes can't be unlinked as they do not have
 * pointer to their parent.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlUnlinkNode(mut cur: xmlNodePtr) {
    if cur.is_null() { return }
    if (*cur).type_0 as std::os::raw::c_uint ==
           XML_NAMESPACE_DECL as std::os::raw::c_int as std::os::raw::c_uint {
        return
    }
    if (*cur).type_0 as std::os::raw::c_uint ==
           XML_DTD_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        let mut doc: xmlDocPtr = 0 as *mut xmlDoc;
        doc = (*cur).doc;
        if !doc.is_null() {
            if (*doc).intSubset == cur as xmlDtdPtr {
                (*doc).intSubset = 0 as *mut _xmlDtd
            }
            if (*doc).extSubset == cur as xmlDtdPtr {
                (*doc).extSubset = 0 as *mut _xmlDtd
            }
        }
    }
    if (*cur).type_0 as std::os::raw::c_uint ==
           XML_ENTITY_DECL as std::os::raw::c_int as std::os::raw::c_uint {
        let mut doc_0: xmlDocPtr = 0 as *mut xmlDoc;
        doc_0 = (*cur).doc;
        if !doc_0.is_null() {
            if !(*doc_0).intSubset.is_null() {
                if xmlHashLookup((*(*doc_0).intSubset).entities as
                                     xmlHashTablePtr, (*cur).name) ==
                       cur as *mut std::os::raw::c_void {
                    xmlHashRemoveEntry((*(*doc_0).intSubset).entities as
                                           xmlHashTablePtr, (*cur).name,
                                       None);
                }
                if xmlHashLookup((*(*doc_0).intSubset).pentities as
                                     xmlHashTablePtr, (*cur).name) ==
                       cur as *mut std::os::raw::c_void {
                    xmlHashRemoveEntry((*(*doc_0).intSubset).pentities as
                                           xmlHashTablePtr, (*cur).name,
                                       None);
                }
            }
            if !(*doc_0).extSubset.is_null() {
                if xmlHashLookup((*(*doc_0).extSubset).entities as
                                     xmlHashTablePtr, (*cur).name) ==
                       cur as *mut std::os::raw::c_void {
                    xmlHashRemoveEntry((*(*doc_0).extSubset).entities as
                                           xmlHashTablePtr, (*cur).name,
                                       None);
                }
                if xmlHashLookup((*(*doc_0).extSubset).pentities as
                                     xmlHashTablePtr, (*cur).name) ==
                       cur as *mut std::os::raw::c_void {
                    xmlHashRemoveEntry((*(*doc_0).extSubset).pentities as
                                           xmlHashTablePtr, (*cur).name,
                                       None);
                }
            }
        }
    }
    if !(*cur).parent.is_null() {
        let mut parent: xmlNodePtr = 0 as *mut xmlNode;
        parent = (*cur).parent;
        if (*cur).type_0 as std::os::raw::c_uint ==
               XML_ATTRIBUTE_NODE as std::os::raw::c_int as std::os::raw::c_uint {
            if (*parent).properties == cur as xmlAttrPtr {
                (*parent).properties = (*(cur as xmlAttrPtr)).next
            }
        } else {
            if (*parent).children == cur { (*parent).children = (*cur).next }
            if (*parent).last == cur { (*parent).last = (*cur).prev }
        }
        (*cur).parent = 0 as *mut _xmlNode
    }
    if !(*cur).next.is_null() { (*(*cur).next).prev = (*cur).prev }
    if !(*cur).prev.is_null() { (*(*cur).prev).next = (*cur).next }
    (*cur).prev = 0 as *mut _xmlNode;
    (*cur).next = (*cur).prev;
}
/* *
 * xmlReplaceNode:
 * @old:  the old node
 * @cur:  the node
 *
 * Unlink the old node from its current context, prune the new one
 * at the same place. If @cur was already inserted in a document it is
 * first unlinked from its existing context.
 *
 * Returns the @old node
 */
#[no_mangle]
pub unsafe extern "C" fn xmlReplaceNode(mut old: xmlNodePtr,
                                        mut cur: xmlNodePtr) -> xmlNodePtr {
    if old == cur { return 0 as xmlNodePtr }
    if old.is_null() ||
           (*old).type_0 as std::os::raw::c_uint ==
               XML_NAMESPACE_DECL as std::os::raw::c_int as std::os::raw::c_uint ||
           (*old).parent.is_null() {
        return 0 as xmlNodePtr
    }
    if cur.is_null() ||
           (*cur).type_0 as std::os::raw::c_uint ==
               XML_NAMESPACE_DECL as std::os::raw::c_int as std::os::raw::c_uint {
        xmlUnlinkNode(old);
        return old
    }
    if cur == old { return old }
    if (*old).type_0 as std::os::raw::c_uint ==
           XML_ATTRIBUTE_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
           (*cur).type_0 as std::os::raw::c_uint !=
               XML_ATTRIBUTE_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        return old
    }
    if (*cur).type_0 as std::os::raw::c_uint ==
           XML_ATTRIBUTE_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
           (*old).type_0 as std::os::raw::c_uint !=
               XML_ATTRIBUTE_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        return old
    }
    xmlUnlinkNode(cur);
    xmlSetTreeDoc(cur, (*old).doc);
    (*cur).parent = (*old).parent;
    (*cur).next = (*old).next;
    if !(*cur).next.is_null() { (*(*cur).next).prev = cur }
    (*cur).prev = (*old).prev;
    if !(*cur).prev.is_null() { (*(*cur).prev).next = cur }
    if !(*cur).parent.is_null() {
        if (*cur).type_0 as std::os::raw::c_uint ==
               XML_ATTRIBUTE_NODE as std::os::raw::c_int as std::os::raw::c_uint {
            if (*(*cur).parent).properties == old as xmlAttrPtr {
                (*(*cur).parent).properties = cur as xmlAttrPtr
            }
        } else {
            if (*(*cur).parent).children == old {
                (*(*cur).parent).children = cur
            }
            if (*(*cur).parent).last == old { (*(*cur).parent).last = cur }
        }
    }
    (*old).prev = 0 as *mut _xmlNode;
    (*old).next = (*old).prev;
    (*old).parent = 0 as *mut _xmlNode;
    return old;
}
/* LIBXML_TREE_ENABLED */
/* ***********************************************************************
 *									*
 *		Copy operations						*
 *									*
 ************************************************************************/
/* *
 * xmlCopyNamespace:
 * @cur:  the namespace
 *
 * Do a copy of the namespace.
 *
 * Returns: a new #xmlNsPtr, or NULL in case of error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlCopyNamespace(mut cur: xmlNsPtr) -> xmlNsPtr {
    let mut ret: xmlNsPtr = 0 as *mut xmlNs;
    if cur.is_null() { return 0 as xmlNsPtr }
    match (*cur).type_0 as std::os::raw::c_uint {
        18 => { ret = xmlNewNs(0 as xmlNodePtr, (*cur).href, (*cur).prefix) }
        _ => { return 0 as xmlNsPtr }
    }
    return ret;
}
/* *
 * xmlCopyNamespaceList:
 * @cur:  the first namespace
 *
 * Do a copy of an namespace list.
 *
 * Returns: a new #xmlNsPtr, or NULL in case of error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlCopyNamespaceList(mut cur: xmlNsPtr) -> xmlNsPtr {
    let mut ret: xmlNsPtr = 0 as xmlNsPtr;
    let mut p: xmlNsPtr = 0 as xmlNsPtr;
    let mut q: xmlNsPtr = 0 as *mut xmlNs;
    while !cur.is_null() {
        q = xmlCopyNamespace(cur);
        if p.is_null() { p = q; ret = p } else { (*p).next = q; p = q }
        cur = (*cur).next
    }
    return ret;
}
unsafe extern "C" fn xmlCopyPropInternal(mut doc: xmlDocPtr,
                                         mut target: xmlNodePtr,
                                         mut cur: xmlAttrPtr) -> xmlAttrPtr {
    let mut ret: xmlAttrPtr = 0 as *mut xmlAttr;
    if cur.is_null() { return 0 as xmlAttrPtr }
    if !target.is_null() &&
           (*target).type_0 as std::os::raw::c_uint !=
               XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        return 0 as xmlAttrPtr
    }
    if !target.is_null() {
        ret = xmlNewDocProp((*target).doc, (*cur).name, 0 as *const xmlChar)
    } else if !doc.is_null() {
        ret = xmlNewDocProp(doc, (*cur).name, 0 as *const xmlChar)
    } else if !(*cur).parent.is_null() {
        ret =
            xmlNewDocProp((*(*cur).parent).doc, (*cur).name,
                          0 as *const xmlChar)
    } else if !(*cur).children.is_null() {
        ret =
            xmlNewDocProp((*(*cur).children).doc, (*cur).name,
                          0 as *const xmlChar)
    } else {
        ret = xmlNewDocProp(0 as xmlDocPtr, (*cur).name, 0 as *const xmlChar)
    }
    if ret.is_null() { return 0 as xmlAttrPtr }
    (*ret).parent = target;
    if !(*cur).ns.is_null() && !target.is_null() {
        let mut ns: xmlNsPtr = 0 as *mut xmlNs;
        ns = xmlSearchNs((*target).doc, target, (*(*cur).ns).prefix);
        if ns.is_null() {
            /*
         * Humm, we are copying an element whose namespace is defined
         * out of the new tree scope. Search it in the original tree
         * and add it at the top of the new tree
         */
            ns = xmlSearchNs((*cur).doc, (*cur).parent, (*(*cur).ns).prefix);
            if !ns.is_null() {
                let mut root: xmlNodePtr = target;
                let mut pred: xmlNodePtr = 0 as xmlNodePtr;
                while !(*root).parent.is_null() {
                    pred = root;
                    root = (*root).parent
                }
                if root == (*target).doc as xmlNodePtr {
                    /* correct possibly cycling above the document elt */
                    root = pred
                }
                (*ret).ns = xmlNewNs(root, (*ns).href, (*ns).prefix)
            }
        } else if xmlStrEqual((*ns).href, (*(*cur).ns).href) != 0 {
            /*
         * we have to find something appropriate here since
         * we cant be sure, that the namespce we found is identified
         * by the prefix
         */
            /* this is the nice case */
            (*ret).ns = ns
        } else {
            /*
           * we are in trouble: we need a new reconcilied namespace.
           * This is expensive
           */
            (*ret).ns = xmlNewReconciliedNs((*target).doc, target, (*cur).ns)
        }
    } else { (*ret).ns = 0 as *mut xmlNs }
    if !(*cur).children.is_null() {
        let mut tmp: xmlNodePtr = 0 as *mut xmlNode;
        (*ret).children =
            xmlStaticCopyNodeList((*cur).children, (*ret).doc,
                                  ret as xmlNodePtr);
        (*ret).last = 0 as *mut _xmlNode;
        tmp = (*ret).children;
        while !tmp.is_null() {
            /* tmp->parent = (xmlNodePtr)ret; */
            if (*tmp).next.is_null() { (*ret).last = tmp }
            tmp = (*tmp).next
        }
    }
    /*
     * Try to handle IDs
     */
    if !target.is_null() && !cur.is_null() && !(*target).doc.is_null() &&
           !(*cur).doc.is_null() && !(*(*cur).doc).ids.is_null() &&
           !(*cur).parent.is_null() {
        if xmlIsID((*cur).doc, (*cur).parent, cur) != 0 {
            let mut id: *mut xmlChar = 0 as *mut xmlChar;
            id =
                xmlNodeListGetString((*cur).doc, (*cur).children,
                                     1 as std::os::raw::c_int);
            if !id.is_null() {
                xmlAddID(0 as xmlValidCtxtPtr, (*target).doc, id, ret);
                xmlFree.expect("non-null function pointer")(id as
                                                                *mut std::os::raw::c_void);
            }
        }
    }
    return ret;
}
/* *
 * xmlCopyProp:
 * @target:  the element where the attribute will be grafted
 * @cur:  the attribute
 *
 * Do a copy of the attribute.
 *
 * Returns: a new #xmlAttrPtr, or NULL in case of error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlCopyProp(mut target: xmlNodePtr,
                                     mut cur: xmlAttrPtr) -> xmlAttrPtr {
    return xmlCopyPropInternal(0 as xmlDocPtr, target, cur);
}
/* *
 * xmlCopyPropList:
 * @target:  the element where the attributes will be grafted
 * @cur:  the first attribute
 *
 * Do a copy of an attribute list.
 *
 * Returns: a new #xmlAttrPtr, or NULL in case of error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlCopyPropList(mut target: xmlNodePtr,
                                         mut cur: xmlAttrPtr) -> xmlAttrPtr {
    let mut ret: xmlAttrPtr = 0 as xmlAttrPtr;
    let mut p: xmlAttrPtr = 0 as xmlAttrPtr;
    let mut q: xmlAttrPtr = 0 as *mut xmlAttr;
    if !target.is_null() &&
           (*target).type_0 as std::os::raw::c_uint !=
               XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        return 0 as xmlAttrPtr
    }
    while !cur.is_null() {
        q = xmlCopyProp(target, cur);
        if q.is_null() { return 0 as xmlAttrPtr }
        if p.is_null() {
            p = q;
            ret = p
        } else { (*p).next = q; (*q).prev = p; p = q }
        cur = (*cur).next
    }
    return ret;
}
/*
 * NOTE about the CopyNode operations !
 *
 * They are split into external and internal parts for one
 * tricky reason: namespaces. Doing a direct copy of a node
 * say RPM:Copyright without changing the namespace pointer to
 * something else can produce stale links. One way to do it is
 * to keep a reference counter but this doesn't work as soon
 * as one move the element or the subtree out of the scope of
 * the existing namespace. The actual solution seems to add
 * a copy of the namespace at the top of the copied tree if
 * not available in the subtree.
 * Hence two functions, the public front-end call the inner ones
 * The argument "recursive" normally indicates a recursive copy
 * of the node with values 0 (no) and 1 (yes).  For XInclude,
 * however, we allow a value of 2 to indicate copy properties and
 * namespace info, but don't recurse on children.
 */
unsafe extern "C" fn xmlStaticCopyNode(mut node: xmlNodePtr,
                                       mut doc: xmlDocPtr,
                                       mut parent: xmlNodePtr,
                                       mut extended: std::os::raw::c_int)
 -> xmlNodePtr {
    let mut ret: xmlNodePtr = 0 as *mut xmlNode;
    if node.is_null() { return 0 as xmlNodePtr }
    match (*node).type_0 as std::os::raw::c_uint {
        2 => {
            return xmlCopyPropInternal(doc, parent, node as xmlAttrPtr) as
                       xmlNodePtr
        }
        18 => { return xmlCopyNamespaceList(node as xmlNsPtr) as xmlNodePtr }
        9 | 13 | 21 => {
            return xmlCopyDoc(node as xmlDocPtr, extended) as xmlNodePtr
        }
        10 | 12 | 14 | 15 | 16 | 17 => {
            /* LIBXML_TREE_ENABLED */
            return 0 as xmlNodePtr
        }
        3 | 4 | 1 | 11 | 5 | 6 | 7 | 8 | 19 | 20 | _ => { }
    }
    /*
     * Allocate a new node and fill the fields.
     */
    ret =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlNode>()
                                                          as std::os::raw::c_ulong) as
            xmlNodePtr;
    if ret.is_null() {
        xmlTreeErrMemory(b"copying node\x00" as *const u8 as
                             *const std::os::raw::c_char);
        return 0 as xmlNodePtr
    }
    memset(ret as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<xmlNode>() as std::os::raw::c_ulong);
    (*ret).type_0 = (*node).type_0;
    (*ret).doc = doc;
    (*ret).parent = parent;
    if (*node).name == xmlStringText.as_ptr() {
        (*ret).name = xmlStringText.as_ptr()
    } else if (*node).name == xmlStringTextNoenc.as_ptr() {
        (*ret).name = xmlStringTextNoenc.as_ptr()
    } else if (*node).name == xmlStringComment.as_ptr() {
        (*ret).name = xmlStringComment.as_ptr()
    } else if !(*node).name.is_null() {
        if !doc.is_null() && !(*doc).dict.is_null() {
            (*ret).name =
                xmlDictLookup((*doc).dict, (*node).name, -(1 as std::os::raw::c_int))
        } else { (*ret).name = xmlStrdup((*node).name) }
    }
    if (*node).type_0 as std::os::raw::c_uint !=
           XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
           !(*node).content.is_null() &&
           (*node).type_0 as std::os::raw::c_uint !=
               XML_ENTITY_REF_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
           (*node).type_0 as std::os::raw::c_uint !=
               XML_XINCLUDE_END as std::os::raw::c_int as std::os::raw::c_uint &&
           (*node).type_0 as std::os::raw::c_uint !=
               XML_XINCLUDE_START as std::os::raw::c_int as std::os::raw::c_uint {
        (*ret).content = xmlStrdup((*node).content)
    } else if (*node).type_0 as std::os::raw::c_uint ==
                  XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        (*ret).line = (*node).line
    }
    if !parent.is_null() {
        let mut tmp: xmlNodePtr = 0 as *mut xmlNode;
        /*
	 * this is a tricky part for the node register thing:
	 * in case ret does get coalesced in xmlAddChild
	 * the deregister-node callback is called; so we register ret now already
	 */
        if __xmlRegisterCallbacks != 0 &&
               (*__xmlRegisterNodeDefaultValue()).is_some() {
            (*__xmlRegisterNodeDefaultValue()).expect("non-null function pointer")(ret);
        }
        tmp = xmlAddChild(parent, ret);
        /* node could have coalesced */
        if tmp != ret { return tmp }
    }
    if !(extended == 0) {
        if ((*node).type_0 as std::os::raw::c_uint ==
                XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint ||
                (*node).type_0 as std::os::raw::c_uint ==
                    XML_XINCLUDE_START as std::os::raw::c_int as std::os::raw::c_uint) &&
               !(*node).nsDef.is_null() {
            (*ret).nsDef = xmlCopyNamespaceList((*node).nsDef)
        }
        if !(*node).ns.is_null() {
            let mut ns: xmlNsPtr = 0 as *mut xmlNs;
            ns = xmlSearchNs(doc, ret, (*(*node).ns).prefix);
            if ns.is_null() {
                /*
	     * Humm, we are copying an element whose namespace is defined
	     * out of the new tree scope. Search it in the original tree
	     * and add it at the top of the new tree
	     */
                ns = xmlSearchNs((*node).doc, node, (*(*node).ns).prefix);
                if !ns.is_null() {
                    let mut root: xmlNodePtr = ret;
                    while !(*root).parent.is_null() { root = (*root).parent }
                    (*ret).ns = xmlNewNs(root, (*ns).href, (*ns).prefix)
                } else {
                    (*ret).ns = xmlNewReconciliedNs(doc, ret, (*node).ns)
                }
            } else {
                /*
	     * reference the existing namespace definition in our own tree.
	     */
                (*ret).ns = ns
            }
        }
        if ((*node).type_0 as std::os::raw::c_uint ==
                XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint ||
                (*node).type_0 as std::os::raw::c_uint ==
                    XML_XINCLUDE_START as std::os::raw::c_int as std::os::raw::c_uint) &&
               !(*node).properties.is_null() {
            (*ret).properties = xmlCopyPropList(ret, (*node).properties)
        }
        if (*node).type_0 as std::os::raw::c_uint ==
               XML_ENTITY_REF_NODE as std::os::raw::c_int as std::os::raw::c_uint {
            if doc.is_null() || (*node).doc != doc {
                /*
	     * The copied node will go into a separate document, so
	     * to avoid dangling references to the ENTITY_DECL node
	     * we cannot keep the reference. Try to find it in the
	     * target document.
	     */
                (*ret).children =
                    xmlGetDocEntity(doc as *const xmlDoc, (*ret).name) as
                        xmlNodePtr
            } else { (*ret).children = (*node).children }
            (*ret).last = (*ret).children
        } else if !(*node).children.is_null() && extended != 2 as std::os::raw::c_int
         {
            (*ret).children =
                xmlStaticCopyNodeList((*node).children, doc, ret);
            if !ret.is_null() {
                let mut ulccur: xmlNodePtr = (*ret).children;
                if ulccur.is_null() {
                    (*ret).last = 0 as *mut _xmlNode
                } else {
                    while !(*ulccur).next.is_null() {
                        (*ulccur).parent = ret;
                        ulccur = (*ulccur).next
                    }
                    (*ulccur).parent = ret;
                    (*ret).last = ulccur
                }
            }
        }
    }
    /* if parent != NULL we already registered the node above */
    if parent.is_null() &&
           (__xmlRegisterCallbacks != 0 &&
                (*__xmlRegisterNodeDefaultValue()).is_some()) {
        (*__xmlRegisterNodeDefaultValue()).expect("non-null function pointer")(ret);
    }
    return ret;
}
unsafe extern "C" fn xmlStaticCopyNodeList(mut node: xmlNodePtr,
                                           mut doc: xmlDocPtr,
                                           mut parent: xmlNodePtr)
 -> xmlNodePtr {
    let mut ret: xmlNodePtr = 0 as xmlNodePtr;
    let mut p: xmlNodePtr = 0 as xmlNodePtr;
    let mut q: xmlNodePtr = 0 as *mut xmlNode;
    while !node.is_null() {
        if (*node).type_0 as std::os::raw::c_uint ==
               XML_DTD_NODE as std::os::raw::c_int as std::os::raw::c_uint {
            if doc.is_null() {
                node = (*node).next;
                continue ;
            } else if (*doc).intSubset.is_null() {
                q = xmlCopyDtd(node as xmlDtdPtr) as xmlNodePtr;
                if q.is_null() { return 0 as xmlNodePtr }
                (*q).doc = doc;
                (*q).parent = parent;
                (*doc).intSubset = q as xmlDtdPtr;
                xmlAddChild(parent, q);
            } else {
                q = (*doc).intSubset as xmlNodePtr;
                xmlAddChild(parent, q);
            }
        } else {
            /* LIBXML_TREE_ENABLED */
            q = xmlStaticCopyNode(node, doc, parent, 1 as std::os::raw::c_int)
        }
        if q.is_null() { return 0 as xmlNodePtr }
        if ret.is_null() {
            (*q).prev = 0 as *mut _xmlNode;
            p = q;
            ret = p
        } else if p != q {
            /* the test is required if xmlStaticCopyNode coalesced 2 text nodes */
            (*p).next = q;
            (*q).prev = p;
            p = q
        }
        node = (*node).next
    }
    return ret;
}
/* *
 * xmlCopyNode:
 * @node:  the node
 * @extended:   if 1 do a recursive copy (properties, namespaces and children
 *			when applicable)
 *		if 2 copy properties and namespaces (when applicable)
 *
 * Do a copy of the node.
 *
 * Returns: a new #xmlNodePtr, or NULL in case of error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlCopyNode(mut node: xmlNodePtr,
                                     mut extended: std::os::raw::c_int)
 -> xmlNodePtr {
    let mut ret: xmlNodePtr = 0 as *mut xmlNode;
    ret = xmlStaticCopyNode(node, 0 as xmlDocPtr, 0 as xmlNodePtr, extended);
    return ret;
}
/* *
 * xmlDocCopyNode:
 * @node:  the node
 * @doc:  the document
 * @extended:   if 1 do a recursive copy (properties, namespaces and children
 *			when applicable)
 *		if 2 copy properties and namespaces (when applicable)
 *
 * Do a copy of the node to a given document.
 *
 * Returns: a new #xmlNodePtr, or NULL in case of error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlDocCopyNode(mut node: xmlNodePtr,
                                        mut doc: xmlDocPtr,
                                        mut extended: std::os::raw::c_int)
 -> xmlNodePtr {
    let mut ret: xmlNodePtr = 0 as *mut xmlNode;
    ret = xmlStaticCopyNode(node, doc, 0 as xmlNodePtr, extended);
    return ret;
}
/* *
 * xmlDocCopyNodeList:
 * @doc: the target document
 * @node:  the first node in the list.
 *
 * Do a recursive copy of the node list.
 *
 * Returns: a new #xmlNodePtr, or NULL in case of error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlDocCopyNodeList(mut doc: xmlDocPtr,
                                            mut node: xmlNodePtr)
 -> xmlNodePtr {
    let mut ret: xmlNodePtr =
        xmlStaticCopyNodeList(node, doc, 0 as xmlNodePtr);
    return ret;
}
/* *
 * xmlCopyNodeList:
 * @node:  the first node in the list.
 *
 * Do a recursive copy of the node list.
 * Use xmlDocCopyNodeList() if possible to ensure string interning.
 *
 * Returns: a new #xmlNodePtr, or NULL in case of error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlCopyNodeList(mut node: xmlNodePtr) -> xmlNodePtr {
    let mut ret: xmlNodePtr =
        xmlStaticCopyNodeList(node, 0 as xmlDocPtr, 0 as xmlNodePtr);
    return ret;
}
/* *
 * xmlCopyDtd:
 * @dtd:  the dtd
 *
 * Do a copy of the dtd.
 *
 * Returns: a new #xmlDtdPtr, or NULL in case of error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlCopyDtd(mut dtd: xmlDtdPtr) -> xmlDtdPtr {
    let mut ret: xmlDtdPtr = 0 as *mut xmlDtd;
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    let mut p: xmlNodePtr = 0 as xmlNodePtr;
    let mut q: xmlNodePtr = 0 as *mut xmlNode;
    if dtd.is_null() { return 0 as xmlDtdPtr }
    ret =
        xmlNewDtd(0 as xmlDocPtr, (*dtd).name, (*dtd).ExternalID,
                  (*dtd).SystemID);
    if ret.is_null() { return 0 as xmlDtdPtr }
    if !(*dtd).entities.is_null() {
        (*ret).entities =
            xmlCopyEntitiesTable((*dtd).entities as xmlEntitiesTablePtr) as
                *mut std::os::raw::c_void
    }
    if !(*dtd).notations.is_null() {
        (*ret).notations =
            xmlCopyNotationTable((*dtd).notations as xmlNotationTablePtr) as
                *mut std::os::raw::c_void
    }
    if !(*dtd).elements.is_null() {
        (*ret).elements =
            xmlCopyElementTable((*dtd).elements as xmlElementTablePtr) as
                *mut std::os::raw::c_void
    }
    if !(*dtd).attributes.is_null() {
        (*ret).attributes =
            xmlCopyAttributeTable((*dtd).attributes as xmlAttributeTablePtr)
                as *mut std::os::raw::c_void
    }
    if !(*dtd).pentities.is_null() {
        (*ret).pentities =
            xmlCopyEntitiesTable((*dtd).pentities as xmlEntitiesTablePtr) as
                *mut std::os::raw::c_void
    }
    cur = (*dtd).children;
    while !cur.is_null() {
        q = 0 as xmlNodePtr;
        if (*cur).type_0 as std::os::raw::c_uint ==
               XML_ENTITY_DECL as std::os::raw::c_int as std::os::raw::c_uint {
            let mut tmp: xmlEntityPtr = cur as xmlEntityPtr;
            match (*tmp).etype as std::os::raw::c_uint {
                1 | 2 | 3 => {
                    q =
                        xmlGetEntityFromDtd(ret as *const xmlDtd, (*tmp).name)
                            as xmlNodePtr
                }
                4 | 5 => {
                    q =
                        xmlGetParameterEntityFromDtd(ret as *const xmlDtd,
                                                     (*tmp).name) as
                            xmlNodePtr
                }
                6 | _ => { }
            }
        } else if (*cur).type_0 as std::os::raw::c_uint ==
                      XML_ELEMENT_DECL as std::os::raw::c_int as std::os::raw::c_uint {
            let mut tmp_0: xmlElementPtr = cur as xmlElementPtr;
            q =
                xmlGetDtdQElementDesc(ret, (*tmp_0).name, (*tmp_0).prefix) as
                    xmlNodePtr
        } else if (*cur).type_0 as std::os::raw::c_uint ==
                      XML_ATTRIBUTE_DECL as std::os::raw::c_int as std::os::raw::c_uint {
            let mut tmp_1: xmlAttributePtr = cur as xmlAttributePtr;
            q =
                xmlGetDtdQAttrDesc(ret, (*tmp_1).elem, (*tmp_1).name,
                                   (*tmp_1).prefix) as xmlNodePtr
        } else if (*cur).type_0 as std::os::raw::c_uint ==
                      XML_COMMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
            q = xmlCopyNode(cur, 0 as std::os::raw::c_int)
        }
        if q.is_null() {
            cur = (*cur).next
        } else {
            if p.is_null() { (*ret).children = q } else { (*p).next = q }
            (*q).prev = p;
            (*q).parent = ret as xmlNodePtr;
            (*q).next = 0 as *mut _xmlNode;
            (*ret).last = q;
            p = q;
            cur = (*cur).next
        }
    }
    return ret;
}
/* *
 * xmlCopyDoc:
 * @doc:  the document
 * @recursive:  if not zero do a recursive copy.
 *
 * Do a copy of the document info. If recursive, the content tree will
 * be copied too as well as DTD, namespaces and entities.
 *
 * Returns: a new #xmlDocPtr, or NULL in case of error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlCopyDoc(mut doc: xmlDocPtr,
                                    mut recursive: std::os::raw::c_int) -> xmlDocPtr {
    let mut ret: xmlDocPtr = 0 as *mut xmlDoc;
    if doc.is_null() { return 0 as xmlDocPtr }
    ret = xmlNewDoc((*doc).version);
    if ret.is_null() { return 0 as xmlDocPtr }
    if !(*doc).name.is_null() {
        (*ret).name =
            xmlMemStrdup.expect("non-null function pointer")((*doc).name)
    }
    if !(*doc).encoding.is_null() {
        (*ret).encoding = xmlStrdup((*doc).encoding)
    }
    if !(*doc).URL.is_null() { (*ret).URL = xmlStrdup((*doc).URL) }
    (*ret).charset = (*doc).charset;
    (*ret).compression = (*doc).compression;
    (*ret).standalone = (*doc).standalone;
    if recursive == 0 { return ret }
    (*ret).last = 0 as *mut _xmlNode;
    (*ret).children = 0 as *mut _xmlNode;
    if !(*doc).intSubset.is_null() {
        (*ret).intSubset = xmlCopyDtd((*doc).intSubset);
        if (*ret).intSubset.is_null() {
            xmlFreeDoc(ret);
            return 0 as xmlDocPtr
        }
        xmlSetTreeDoc((*ret).intSubset as xmlNodePtr, ret);
        (*(*ret).intSubset).parent = ret
    }
    if !(*doc).oldNs.is_null() {
        (*ret).oldNs = xmlCopyNamespaceList((*doc).oldNs)
    }
    if !(*doc).children.is_null() {
        let mut tmp: xmlNodePtr = 0 as *mut xmlNode;
        (*ret).children =
            xmlStaticCopyNodeList((*doc).children, ret, ret as xmlNodePtr);
        (*ret).last = 0 as *mut _xmlNode;
        tmp = (*ret).children;
        while !tmp.is_null() {
            if (*tmp).next.is_null() { (*ret).last = tmp }
            tmp = (*tmp).next
        }
    }
    return ret;
}
/* LIBXML_TREE_ENABLED */
/* ***********************************************************************
 *									*
 *		Content access functions				*
 *									*
 ************************************************************************/
/* *
 * xmlGetLineNoInternal:
 * @node: valid node
 * @depth: used to limit any risk of recursion
 *
 * Get line number of @node.
 * Try to override the limitation of lines being store in 16 bits ints
 *
 * Returns the line number if successful, -1 otherwise
 */
unsafe extern "C" fn xmlGetLineNoInternal(mut node: *const xmlNode,
                                          mut depth: std::os::raw::c_int)
 -> std::os::raw::c_long {
    let mut result: std::os::raw::c_long = -(1 as std::os::raw::c_int) as std::os::raw::c_long;
    if depth >= 5 as std::os::raw::c_int {
        return -(1 as std::os::raw::c_int) as std::os::raw::c_long
    }
    if node.is_null() { return result }
    if (*node).type_0 as std::os::raw::c_uint ==
           XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint ||
           (*node).type_0 as std::os::raw::c_uint ==
               XML_TEXT_NODE as std::os::raw::c_int as std::os::raw::c_uint ||
           (*node).type_0 as std::os::raw::c_uint ==
               XML_COMMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint ||
           (*node).type_0 as std::os::raw::c_uint ==
               XML_PI_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        if (*node).line as std::os::raw::c_int == 65535 as std::os::raw::c_int {
            if (*node).type_0 as std::os::raw::c_uint ==
                   XML_TEXT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
                   !(*node).psvi.is_null() {
                result = (*node).psvi as ptrdiff_t
            } else if (*node).type_0 as std::os::raw::c_uint ==
                          XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
                          !(*node).children.is_null() {
                result =
                    xmlGetLineNoInternal((*node).children,
                                         depth + 1 as std::os::raw::c_int)
            } else if !(*node).next.is_null() {
                result =
                    xmlGetLineNoInternal((*node).next,
                                         depth + 1 as std::os::raw::c_int)
            } else if !(*node).prev.is_null() {
                result =
                    xmlGetLineNoInternal((*node).prev,
                                         depth + 1 as std::os::raw::c_int)
            }
        }
        if result == -(1 as std::os::raw::c_int) as std::os::raw::c_long ||
               result == 65535 as std::os::raw::c_int as std::os::raw::c_long {
            result = (*node).line as std::os::raw::c_long
        }
    } else if !(*node).prev.is_null() &&
                  ((*(*node).prev).type_0 as std::os::raw::c_uint ==
                       XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint ||
                       (*(*node).prev).type_0 as std::os::raw::c_uint ==
                           XML_TEXT_NODE as std::os::raw::c_int as std::os::raw::c_uint ||
                       (*(*node).prev).type_0 as std::os::raw::c_uint ==
                           XML_COMMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint ||
                       (*(*node).prev).type_0 as std::os::raw::c_uint ==
                           XML_PI_NODE as std::os::raw::c_int as std::os::raw::c_uint) {
        result = xmlGetLineNoInternal((*node).prev, depth + 1 as std::os::raw::c_int)
    } else if !(*node).parent.is_null() &&
                  (*(*node).parent).type_0 as std::os::raw::c_uint ==
                      XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        result =
            xmlGetLineNoInternal((*node).parent, depth + 1 as std::os::raw::c_int)
    }
    return result;
}
/* *
 * xmlGetLineNo:
 * @node: valid node
 *
 * Get line number of @node.
 * Try to override the limitation of lines being store in 16 bits ints
 * if XML_PARSE_BIG_LINES parser option was used
 *
 * Returns the line number if successful, -1 otherwise
 */
#[no_mangle]
pub unsafe extern "C" fn xmlGetLineNo(mut node: *const xmlNode)
 -> std::os::raw::c_long {
    return xmlGetLineNoInternal(node, 0 as std::os::raw::c_int);
}
/* *
 * xmlGetNodePath:
 * @node: a node
 *
 * Build a structure based Path for the given node
 *
 * Returns the new path or NULL in case of error. The caller must free
 *     the returned string
 */
#[no_mangle]
pub unsafe extern "C" fn xmlGetNodePath(mut node: *const xmlNode)
 -> *mut xmlChar {
    let mut cur: *const xmlNode = 0 as *const xmlNode;
    let mut tmp: *const xmlNode = 0 as *const xmlNode;
    let mut next: *const xmlNode = 0 as *const xmlNode;
    let mut buffer: *mut xmlChar = 0 as *mut xmlChar;
    let mut temp: *mut xmlChar = 0 as *mut xmlChar;
    let mut buf_len: size_t = 0;
    let mut buf: *mut xmlChar = 0 as *mut xmlChar;
    let mut sep: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut name: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut nametemp: [std::os::raw::c_char; 100] = [0; 100];
    let mut occur: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut generic: std::os::raw::c_int = 0;
    if node.is_null() ||
           (*node).type_0 as std::os::raw::c_uint ==
               XML_NAMESPACE_DECL as std::os::raw::c_int as std::os::raw::c_uint {
        return 0 as *mut xmlChar
    }
    buf_len = 500 as std::os::raw::c_int as size_t;
    buffer =
        xmlMallocAtomic.expect("non-null function pointer")(buf_len.wrapping_mul(::std::mem::size_of::<xmlChar>()
                                                                                     as
                                                                                     std::os::raw::c_ulong))
            as *mut xmlChar;
    if buffer.is_null() {
        xmlTreeErrMemory(b"getting node path\x00" as *const u8 as
                             *const std::os::raw::c_char);
        return 0 as *mut xmlChar
    }
    buf =
        xmlMallocAtomic.expect("non-null function pointer")(buf_len.wrapping_mul(::std::mem::size_of::<xmlChar>()
                                                                                     as
                                                                                     std::os::raw::c_ulong))
            as *mut xmlChar;
    if buf.is_null() {
        xmlTreeErrMemory(b"getting node path\x00" as *const u8 as
                             *const std::os::raw::c_char);
        xmlFree.expect("non-null function pointer")(buffer as
                                                        *mut std::os::raw::c_void);
        return 0 as *mut xmlChar
    }
    *buffer.offset(0 as std::os::raw::c_int as isize) = 0 as std::os::raw::c_int as xmlChar;
    cur = node;
    loop  {
        name = b"\x00" as *const u8 as *const std::os::raw::c_char;
        sep = b"?\x00" as *const u8 as *const std::os::raw::c_char;
        occur = 0 as std::os::raw::c_int;
        if (*cur).type_0 as std::os::raw::c_uint ==
               XML_DOCUMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint ||
               (*cur).type_0 as std::os::raw::c_uint ==
                   XML_HTML_DOCUMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
            if *buffer.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                   '/' as i32 {
                break ;
            }
            sep = b"/\x00" as *const u8 as *const std::os::raw::c_char;
            next = 0 as *const xmlNode
        } else if (*cur).type_0 as std::os::raw::c_uint ==
                      XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
            generic = 0 as std::os::raw::c_int;
            sep = b"/\x00" as *const u8 as *const std::os::raw::c_char;
            name = (*cur).name as *const std::os::raw::c_char;
            if !(*cur).ns.is_null() {
                if !(*(*cur).ns).prefix.is_null() {
                    snprintf(nametemp.as_mut_ptr(),
                             (::std::mem::size_of::<[std::os::raw::c_char; 100]>() as
                                  std::os::raw::c_ulong).wrapping_sub(1 as std::os::raw::c_int
                                                                  as
                                                                  std::os::raw::c_ulong),
                             b"%s:%s\x00" as *const u8 as *const std::os::raw::c_char,
                             (*(*cur).ns).prefix as *mut std::os::raw::c_char,
                             (*cur).name as *mut std::os::raw::c_char);
                    nametemp[(::std::mem::size_of::<[std::os::raw::c_char; 100]>() as
                                  std::os::raw::c_ulong).wrapping_sub(1 as std::os::raw::c_int
                                                                  as
                                                                  std::os::raw::c_ulong)
                                 as usize] = 0 as std::os::raw::c_int as std::os::raw::c_char;
                    name = nametemp.as_mut_ptr()
                } else {
                    /*
		    * We cannot express named elements in the default
		    * namespace, so use "*".
		    */
                    generic = 1 as std::os::raw::c_int;
                    name = b"*\x00" as *const u8 as *const std::os::raw::c_char
                }
            }
            next = (*cur).parent;
            /*
             * Thumbler index computation
	     * TODO: the ocurence test seems bogus for namespaced names
             */
            tmp = (*cur).prev;
            while !tmp.is_null() {
                if (*tmp).type_0 as std::os::raw::c_uint ==
                       XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
                       (generic != 0 ||
                            xmlStrEqual((*cur).name, (*tmp).name) != 0 &&
                                ((*tmp).ns == (*cur).ns ||
                                     !(*tmp).ns.is_null() &&
                                         !(*cur).ns.is_null() &&
                                         xmlStrEqual((*(*cur).ns).prefix,
                                                     (*(*tmp).ns).prefix) !=
                                             0)) {
                    occur += 1
                }
                tmp = (*tmp).prev
            }
            if occur == 0 as std::os::raw::c_int {
                tmp = (*cur).next;
                while !tmp.is_null() && occur == 0 as std::os::raw::c_int {
                    if (*tmp).type_0 as std::os::raw::c_uint ==
                           XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
                           (generic != 0 ||
                                xmlStrEqual((*cur).name, (*tmp).name) != 0 &&
                                    ((*tmp).ns == (*cur).ns ||
                                         !(*tmp).ns.is_null() &&
                                             !(*cur).ns.is_null() &&
                                             xmlStrEqual((*(*cur).ns).prefix,
                                                         (*(*tmp).ns).prefix)
                                                 != 0)) {
                        occur += 1
                    }
                    tmp = (*tmp).next
                }
                if occur != 0 as std::os::raw::c_int { occur = 1 as std::os::raw::c_int }
            } else { occur += 1 }
        } else if (*cur).type_0 as std::os::raw::c_uint ==
                      XML_COMMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
            sep = b"/\x00" as *const u8 as *const std::os::raw::c_char;
            name = b"comment()\x00" as *const u8 as *const std::os::raw::c_char;
            next = (*cur).parent;
            /*
             * Thumbler index computation
             */
            tmp = (*cur).prev;
            while !tmp.is_null() {
                if (*tmp).type_0 as std::os::raw::c_uint ==
                       XML_COMMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
                    occur += 1
                }
                tmp = (*tmp).prev
            }
            if occur == 0 as std::os::raw::c_int {
                tmp = (*cur).next;
                while !tmp.is_null() && occur == 0 as std::os::raw::c_int {
                    if (*tmp).type_0 as std::os::raw::c_uint ==
                           XML_COMMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
                        occur += 1
                    }
                    tmp = (*tmp).next
                }
                if occur != 0 as std::os::raw::c_int { occur = 1 as std::os::raw::c_int }
            } else { occur += 1 }
        } else if (*cur).type_0 as std::os::raw::c_uint ==
                      XML_TEXT_NODE as std::os::raw::c_int as std::os::raw::c_uint ||
                      (*cur).type_0 as std::os::raw::c_uint ==
                          XML_CDATA_SECTION_NODE as std::os::raw::c_int as
                              std::os::raw::c_uint {
            sep = b"/\x00" as *const u8 as *const std::os::raw::c_char;
            name = b"text()\x00" as *const u8 as *const std::os::raw::c_char;
            next = (*cur).parent;
            /*
             * Thumbler index computation
             */
            tmp = (*cur).prev;
            while !tmp.is_null() {
                if (*tmp).type_0 as std::os::raw::c_uint ==
                       XML_TEXT_NODE as std::os::raw::c_int as std::os::raw::c_uint ||
                       (*tmp).type_0 as std::os::raw::c_uint ==
                           XML_CDATA_SECTION_NODE as std::os::raw::c_int as
                               std::os::raw::c_uint {
                    occur += 1
                }
                tmp = (*tmp).prev
            }
            /*
	    * Evaluate if this is the only text- or CDATA-section-node;
	    * if yes, then we'll get "text()", otherwise "text()[1]".
	    */
            if occur == 0 as std::os::raw::c_int {
                tmp = (*cur).next;
                while !tmp.is_null() {
                    if (*tmp).type_0 as std::os::raw::c_uint ==
                           XML_TEXT_NODE as std::os::raw::c_int as std::os::raw::c_uint ||
                           (*tmp).type_0 as std::os::raw::c_uint ==
                               XML_CDATA_SECTION_NODE as std::os::raw::c_int as
                                   std::os::raw::c_uint {
                        occur = 1 as std::os::raw::c_int;
                        break ;
                    } else { tmp = (*tmp).next }
                }
            } else { occur += 1 }
        } else if (*cur).type_0 as std::os::raw::c_uint ==
                      XML_PI_NODE as std::os::raw::c_int as std::os::raw::c_uint {
            sep = b"/\x00" as *const u8 as *const std::os::raw::c_char;
            snprintf(nametemp.as_mut_ptr(),
                     (::std::mem::size_of::<[std::os::raw::c_char; 100]>() as
                          std::os::raw::c_ulong).wrapping_sub(1 as std::os::raw::c_int as
                                                          std::os::raw::c_ulong),
                     b"processing-instruction(\'%s\')\x00" as *const u8 as
                         *const std::os::raw::c_char,
                     (*cur).name as *mut std::os::raw::c_char);
            nametemp[(::std::mem::size_of::<[std::os::raw::c_char; 100]>() as
                          std::os::raw::c_ulong).wrapping_sub(1 as std::os::raw::c_int as
                                                          std::os::raw::c_ulong) as
                         usize] = 0 as std::os::raw::c_int as std::os::raw::c_char;
            name = nametemp.as_mut_ptr();
            next = (*cur).parent;
            /*
             * Thumbler index computation
             */
            tmp = (*cur).prev;
            while !tmp.is_null() {
                if (*tmp).type_0 as std::os::raw::c_uint ==
                       XML_PI_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
                       xmlStrEqual((*cur).name, (*tmp).name) != 0 {
                    occur += 1
                }
                tmp = (*tmp).prev
            }
            if occur == 0 as std::os::raw::c_int {
                tmp = (*cur).next;
                while !tmp.is_null() && occur == 0 as std::os::raw::c_int {
                    if (*tmp).type_0 as std::os::raw::c_uint ==
                           XML_PI_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
                           xmlStrEqual((*cur).name, (*tmp).name) != 0 {
                        occur += 1
                    }
                    tmp = (*tmp).next
                }
                if occur != 0 as std::os::raw::c_int { occur = 1 as std::os::raw::c_int }
            } else { occur += 1 }
        } else if (*cur).type_0 as std::os::raw::c_uint ==
                      XML_ATTRIBUTE_NODE as std::os::raw::c_int as std::os::raw::c_uint {
            sep = b"/@\x00" as *const u8 as *const std::os::raw::c_char;
            name = (*(cur as xmlAttrPtr)).name as *const std::os::raw::c_char;
            if !(*cur).ns.is_null() {
                if !(*(*cur).ns).prefix.is_null() {
                    snprintf(nametemp.as_mut_ptr(),
                             (::std::mem::size_of::<[std::os::raw::c_char; 100]>() as
                                  std::os::raw::c_ulong).wrapping_sub(1 as std::os::raw::c_int
                                                                  as
                                                                  std::os::raw::c_ulong),
                             b"%s:%s\x00" as *const u8 as *const std::os::raw::c_char,
                             (*(*cur).ns).prefix as *mut std::os::raw::c_char,
                             (*cur).name as *mut std::os::raw::c_char);
                } else {
                    snprintf(nametemp.as_mut_ptr(),
                             (::std::mem::size_of::<[std::os::raw::c_char; 100]>() as
                                  std::os::raw::c_ulong).wrapping_sub(1 as std::os::raw::c_int
                                                                  as
                                                                  std::os::raw::c_ulong),
                             b"%s\x00" as *const u8 as *const std::os::raw::c_char,
                             (*cur).name as *mut std::os::raw::c_char);
                }
                nametemp[(::std::mem::size_of::<[std::os::raw::c_char; 100]>() as
                              std::os::raw::c_ulong).wrapping_sub(1 as std::os::raw::c_int as
                                                              std::os::raw::c_ulong)
                             as usize] = 0 as std::os::raw::c_int as std::os::raw::c_char;
                name = nametemp.as_mut_ptr()
            }
            next = (*(cur as xmlAttrPtr)).parent
        } else { next = (*cur).parent }
        /*
         * Make sure there is enough room
         */
        if (xmlStrlen(buffer) as
                std::os::raw::c_ulong).wrapping_add(::std::mem::size_of::<[std::os::raw::c_char; 100]>()
                                                as
                                                std::os::raw::c_ulong).wrapping_add(20
                                                                                as
                                                                                std::os::raw::c_int
                                                                                as
                                                                                std::os::raw::c_ulong)
               > buf_len {
            buf_len =
                (2 as std::os::raw::c_int as
                     std::os::raw::c_ulong).wrapping_mul(buf_len).wrapping_add(xmlStrlen(buffer)
                                                                           as
                                                                           std::os::raw::c_ulong).wrapping_add(::std::mem::size_of::<[std::os::raw::c_char; 100]>()
                                                                                                           as
                                                                                                           std::os::raw::c_ulong).wrapping_add(20
                                                                                                                                           as
                                                                                                                                           std::os::raw::c_int
                                                                                                                                           as
                                                                                                                                           std::os::raw::c_ulong);
            temp =
                xmlRealloc.expect("non-null function pointer")(buffer as
                                                                   *mut std::os::raw::c_void,
                                                               buf_len) as
                    *mut xmlChar;
            if temp.is_null() {
                xmlTreeErrMemory(b"getting node path\x00" as *const u8 as
                                     *const std::os::raw::c_char);
                xmlFree.expect("non-null function pointer")(buf as
                                                                *mut std::os::raw::c_void);
                xmlFree.expect("non-null function pointer")(buffer as
                                                                *mut std::os::raw::c_void);
                return 0 as *mut xmlChar
            }
            buffer = temp;
            temp =
                xmlRealloc.expect("non-null function pointer")(buf as
                                                                   *mut std::os::raw::c_void,
                                                               buf_len) as
                    *mut xmlChar;
            if temp.is_null() {
                xmlTreeErrMemory(b"getting node path\x00" as *const u8 as
                                     *const std::os::raw::c_char);
                xmlFree.expect("non-null function pointer")(buf as
                                                                *mut std::os::raw::c_void);
                xmlFree.expect("non-null function pointer")(buffer as
                                                                *mut std::os::raw::c_void);
                return 0 as *mut xmlChar
            }
            buf = temp
        }
        if occur == 0 as std::os::raw::c_int {
            snprintf(buf as *mut std::os::raw::c_char, buf_len,
                     b"%s%s%s\x00" as *const u8 as *const std::os::raw::c_char, sep,
                     name, buffer as *mut std::os::raw::c_char);
        } else {
            snprintf(buf as *mut std::os::raw::c_char, buf_len,
                     b"%s%s[%d]%s\x00" as *const u8 as *const std::os::raw::c_char,
                     sep, name, occur, buffer as *mut std::os::raw::c_char);
        }
        snprintf(buffer as *mut std::os::raw::c_char, buf_len,
                 b"%s\x00" as *const u8 as *const std::os::raw::c_char,
                 buf as *mut std::os::raw::c_char);
        cur = next;
        if cur.is_null() { break ; }
    }
    xmlFree.expect("non-null function pointer")(buf as *mut std::os::raw::c_void);
    return buffer;
}
/* LIBXML_TREE_ENABLED */
/* *
 * xmlDocGetRootElement:
 * @doc:  the document
 *
 * Get the root element of the document (doc->children is a list
 * containing possibly comments, PIs, etc ...).
 *
 * Returns the #xmlNodePtr for the root or NULL
 */
#[no_mangle]
pub unsafe extern "C" fn xmlDocGetRootElement(mut doc: *const xmlDoc)
 -> xmlNodePtr {
    let mut ret: xmlNodePtr = 0 as *mut xmlNode;
    if doc.is_null() { return 0 as xmlNodePtr }
    ret = (*doc).children;
    while !ret.is_null() {
        if (*ret).type_0 as std::os::raw::c_uint ==
               XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
            return ret
        }
        ret = (*ret).next
    }
    return ret;
}
/* *
 * xmlDocSetRootElement:
 * @doc:  the document
 * @root:  the new document root element, if root is NULL no action is taken,
 *         to remove a node from a document use xmlUnlinkNode(root) instead.
 *
 * Set the root element of the document (doc->children is a list
 * containing possibly comments, PIs, etc ...).
 *
 * Returns the old root element if any was found, NULL if root was NULL
 */
#[no_mangle]
pub unsafe extern "C" fn xmlDocSetRootElement(mut doc: xmlDocPtr,
                                              mut root: xmlNodePtr)
 -> xmlNodePtr {
    let mut old: xmlNodePtr = 0 as xmlNodePtr;
    if doc.is_null() { return 0 as xmlNodePtr }
    if root.is_null() ||
           (*root).type_0 as std::os::raw::c_uint ==
               XML_NAMESPACE_DECL as std::os::raw::c_int as std::os::raw::c_uint {
        return 0 as xmlNodePtr
    }
    xmlUnlinkNode(root);
    xmlSetTreeDoc(root, doc);
    (*root).parent = doc as xmlNodePtr;
    old = (*doc).children;
    while !old.is_null() {
        if (*old).type_0 as std::os::raw::c_uint ==
               XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
            break ;
        }
        old = (*old).next
    }
    if old.is_null() {
        if (*doc).children.is_null() {
            (*doc).children = root;
            (*doc).last = root
        } else { xmlAddSibling((*doc).children, root); }
    } else { xmlReplaceNode(old, root); }
    return old;
}
/* *
 * xmlNodeSetLang:
 * @cur:  the node being changed
 * @lang:  the language description
 *
 * Set the language of a node, i.e. the values of the xml:lang
 * attribute.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlNodeSetLang(mut cur: xmlNodePtr,
                                        mut lang: *const xmlChar) {
    let mut ns: xmlNsPtr = 0 as *mut xmlNs;
    if cur.is_null() { return }
    match (*cur).type_0 as std::os::raw::c_uint {
        3 | 4 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 7 | 5 | 6 | 18
        | 21 | 19 | 20 => {
            return
        }
        1 | 2 | _ => { }
    }
    ns =
        xmlSearchNsByHref((*cur).doc, cur,
                          b"http://www.w3.org/XML/1998/namespace\x00" as
                              *const u8 as *const std::os::raw::c_char as
                              *const xmlChar);
    if ns.is_null() { return }
    xmlSetNsProp(cur, ns,
                 b"lang\x00" as *const u8 as *const std::os::raw::c_char as
                     *mut xmlChar, lang);
}
/* LIBXML_TREE_ENABLED */
/* *
 * xmlNodeGetLang:
 * @cur:  the node being checked
 *
 * Searches the language of a node, i.e. the values of the xml:lang
 * attribute or the one carried by the nearest ancestor.
 *
 * Returns a pointer to the lang value, or NULL if not found
 *     It's up to the caller to free the memory with xmlFree().
 */
#[no_mangle]
pub unsafe extern "C" fn xmlNodeGetLang(mut cur: *const xmlNode)
 -> *mut xmlChar {
    let mut lang: *mut xmlChar = 0 as *mut xmlChar;
    if cur.is_null() ||
           (*cur).type_0 as std::os::raw::c_uint ==
               XML_NAMESPACE_DECL as std::os::raw::c_int as std::os::raw::c_uint {
        return 0 as *mut xmlChar
    }
    while !cur.is_null() {
        lang =
            xmlGetNsProp(cur,
                         b"lang\x00" as *const u8 as *const std::os::raw::c_char as
                             *mut xmlChar,
                         b"http://www.w3.org/XML/1998/namespace\x00" as
                             *const u8 as *const std::os::raw::c_char as
                             *const xmlChar);
        if !lang.is_null() { return lang }
        cur = (*cur).parent
    }
    return 0 as *mut xmlChar;
}
/* *
 * xmlNodeSetSpacePreserve:
 * @cur:  the node being changed
 * @val:  the xml:space value ("0": default, 1: "preserve")
 *
 * Set (or reset) the space preserving behaviour of a node, i.e. the
 * value of the xml:space attribute.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlNodeSetSpacePreserve(mut cur: xmlNodePtr,
                                                 mut val: std::os::raw::c_int) {
    let mut ns: xmlNsPtr = 0 as *mut xmlNs;
    if cur.is_null() { return }
    match (*cur).type_0 as std::os::raw::c_uint {
        3 | 4 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 7 | 5 | 6 | 18
        | 19 | 20 | 21 => {
            return
        }
        1 | 2 | _ => { }
    }
    ns =
        xmlSearchNsByHref((*cur).doc, cur,
                          b"http://www.w3.org/XML/1998/namespace\x00" as
                              *const u8 as *const std::os::raw::c_char as
                              *const xmlChar);
    if ns.is_null() { return }
    match val {
        0 => {
            xmlSetNsProp(cur, ns,
                         b"space\x00" as *const u8 as *const std::os::raw::c_char as
                             *mut xmlChar,
                         b"default\x00" as *const u8 as *const std::os::raw::c_char as
                             *mut xmlChar);
        }
        1 => {
            xmlSetNsProp(cur, ns,
                         b"space\x00" as *const u8 as *const std::os::raw::c_char as
                             *mut xmlChar,
                         b"preserve\x00" as *const u8 as *const std::os::raw::c_char
                             as *mut xmlChar);
        }
        _ => { }
    };
}
/* LIBXML_TREE_ENABLED */
/* *
 * xmlNodeGetSpacePreserve:
 * @cur:  the node being checked
 *
 * Searches the space preserving behaviour of a node, i.e. the values
 * of the xml:space attribute or the one carried by the nearest
 * ancestor.
 *
 * Returns -1 if xml:space is not inherited, 0 if "default", 1 if "preserve"
 */
#[no_mangle]
pub unsafe extern "C" fn xmlNodeGetSpacePreserve(mut cur: *const xmlNode)
 -> std::os::raw::c_int {
    let mut space: *mut xmlChar = 0 as *mut xmlChar;
    if cur.is_null() ||
           (*cur).type_0 as std::os::raw::c_uint !=
               XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        return -(1 as std::os::raw::c_int)
    }
    while !cur.is_null() {
        space =
            xmlGetNsProp(cur,
                         b"space\x00" as *const u8 as *const std::os::raw::c_char as
                             *mut xmlChar,
                         b"http://www.w3.org/XML/1998/namespace\x00" as
                             *const u8 as *const std::os::raw::c_char as
                             *const xmlChar);
        if !space.is_null() {
            if xmlStrEqual(space,
                           b"preserve\x00" as *const u8 as *const std::os::raw::c_char
                               as *mut xmlChar) != 0 {
                xmlFree.expect("non-null function pointer")(space as
                                                                *mut std::os::raw::c_void);
                return 1 as std::os::raw::c_int
            }
            if xmlStrEqual(space,
                           b"default\x00" as *const u8 as *const std::os::raw::c_char
                               as *mut xmlChar) != 0 {
                xmlFree.expect("non-null function pointer")(space as
                                                                *mut std::os::raw::c_void);
                return 0 as std::os::raw::c_int
            }
            xmlFree.expect("non-null function pointer")(space as
                                                            *mut std::os::raw::c_void);
        }
        cur = (*cur).parent
    }
    return -(1 as std::os::raw::c_int);
}
/* *
 * xmlNodeSetName:
 * @cur:  the node being changed
 * @name:  the new tag name
 *
 * Set (or reset) the name of a node.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlNodeSetName(mut cur: xmlNodePtr,
                                        mut name: *const xmlChar) {
    let mut doc: xmlDocPtr = 0 as *mut xmlDoc;
    let mut dict: xmlDictPtr = 0 as *mut xmlDict;
    let mut freeme: *const xmlChar = 0 as *const xmlChar;
    if cur.is_null() { return }
    if name.is_null() { return }
    match (*cur).type_0 as std::os::raw::c_uint {
        3 | 4 | 8 | 10 | 11 | 12 | 13 | 18 | 19 | 20 | 21 => { return }
        1 | 2 | 7 | 5 | 6 | 14 | 9 | 15 | 16 | 17 | _ => { }
    }
    doc = (*cur).doc;
    if !doc.is_null() { dict = (*doc).dict } else { dict = 0 as xmlDictPtr }
    if !dict.is_null() {
        if !(*cur).name.is_null() && xmlDictOwns(dict, (*cur).name) == 0 {
            freeme = (*cur).name
        }
        (*cur).name = xmlDictLookup(dict, name, -(1 as std::os::raw::c_int))
    } else {
        if !(*cur).name.is_null() { freeme = (*cur).name }
        (*cur).name = xmlStrdup(name)
    }
    if !freeme.is_null() {
        xmlFree.expect("non-null function pointer")(freeme as *mut xmlChar as
                                                        *mut std::os::raw::c_void);
    };
}
/* *
 * xmlNodeSetBase:
 * @cur:  the node being changed
 * @uri:  the new base URI
 *
 * Set (or reset) the base URI of a node, i.e. the value of the
 * xml:base attribute.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlNodeSetBase(mut cur: xmlNodePtr,
                                        mut uri: *const xmlChar) {
    let mut ns: xmlNsPtr = 0 as *mut xmlNs;
    let mut fixed: *mut xmlChar = 0 as *mut xmlChar;
    if cur.is_null() { return }
    match (*cur).type_0 as std::os::raw::c_uint {
        3 | 4 | 8 | 10 | 11 | 12 | 14 | 15 | 16 | 17 | 7 | 5 | 6 | 18 | 19 |
        20 => {
            return
        }
        9 | 21 | 13 => {
            let mut doc: xmlDocPtr = cur as xmlDocPtr;
            if !(*doc).URL.is_null() {
                xmlFree.expect("non-null function pointer")((*doc).URL as
                                                                *mut xmlChar
                                                                as
                                                                *mut std::os::raw::c_void);
            }
            if uri.is_null() {
                (*doc).URL = 0 as *const xmlChar
            } else { (*doc).URL = xmlPathToURI(uri) }
            return
        }
        1 | 2 | _ => { }
    }
    ns =
        xmlSearchNsByHref((*cur).doc, cur,
                          b"http://www.w3.org/XML/1998/namespace\x00" as
                              *const u8 as *const std::os::raw::c_char as
                              *const xmlChar);
    if ns.is_null() { return }
    fixed = xmlPathToURI(uri);
    if !fixed.is_null() {
        xmlSetNsProp(cur, ns,
                     b"base\x00" as *const u8 as *const std::os::raw::c_char as
                         *mut xmlChar, fixed);
        xmlFree.expect("non-null function pointer")(fixed as
                                                        *mut std::os::raw::c_void);
    } else {
        xmlSetNsProp(cur, ns,
                     b"base\x00" as *const u8 as *const std::os::raw::c_char as
                         *mut xmlChar, uri);
    };
}
/* LIBXML_TREE_ENABLED */
/* *
 * xmlNodeGetBase:
 * @doc:  the document the node pertains to
 * @cur:  the node being checked
 *
 * Searches for the BASE URL. The code should work on both XML
 * and HTML document even if base mechanisms are completely different.
 * It returns the base as defined in RFC 2396 sections
 * 5.1.1. Base URI within Document Content
 * and
 * 5.1.2. Base URI from the Encapsulating Entity
 * However it does not return the document base (5.1.3), use
 * doc->URL in this case
 *
 * Returns a pointer to the base URL, or NULL if not found
 *     It's up to the caller to free the memory with xmlFree().
 */
#[no_mangle]
pub unsafe extern "C" fn xmlNodeGetBase(mut doc: *const xmlDoc,
                                        mut cur: *const xmlNode)
 -> *mut xmlChar {
    let mut oldbase: *mut xmlChar = 0 as *mut xmlChar;
    let mut base: *mut xmlChar = 0 as *mut xmlChar;
    let mut newbase: *mut xmlChar = 0 as *mut xmlChar;
    if cur.is_null() && doc.is_null() { return 0 as *mut xmlChar }
    if !cur.is_null() &&
           (*cur).type_0 as std::os::raw::c_uint ==
               XML_NAMESPACE_DECL as std::os::raw::c_int as std::os::raw::c_uint {
        return 0 as *mut xmlChar
    }
    if doc.is_null() { doc = (*cur).doc }
    if !doc.is_null() &&
           (*doc).type_0 as std::os::raw::c_uint ==
               XML_HTML_DOCUMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        cur = (*doc).children;
        while !cur.is_null() && !(*cur).name.is_null() {
            if (*cur).type_0 as std::os::raw::c_uint !=
                   XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
                cur = (*cur).next
            } else if xmlStrcasecmp((*cur).name,
                                    b"html\x00" as *const u8 as
                                        *const std::os::raw::c_char as *mut xmlChar)
                          == 0 {
                cur = (*cur).children
            } else if xmlStrcasecmp((*cur).name,
                                    b"head\x00" as *const u8 as
                                        *const std::os::raw::c_char as *mut xmlChar)
                          == 0 {
                cur = (*cur).children
            } else {
                if xmlStrcasecmp((*cur).name,
                                 b"base\x00" as *const u8 as
                                     *const std::os::raw::c_char as *mut xmlChar) == 0
                   {
                    return xmlGetProp(cur,
                                      b"href\x00" as *const u8 as
                                          *const std::os::raw::c_char as *mut xmlChar)
                }
                cur = (*cur).next
            }
        }
        return 0 as *mut xmlChar
    }
    while !cur.is_null() {
        if (*cur).type_0 as std::os::raw::c_uint ==
               XML_ENTITY_DECL as std::os::raw::c_int as std::os::raw::c_uint {
            let mut ent: xmlEntityPtr = cur as xmlEntityPtr;
            return xmlStrdup((*ent).URI)
        }
        if (*cur).type_0 as std::os::raw::c_uint ==
               XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
            base =
                xmlGetNsProp(cur,
                             b"base\x00" as *const u8 as *const std::os::raw::c_char
                                 as *mut xmlChar,
                             b"http://www.w3.org/XML/1998/namespace\x00" as
                                 *const u8 as *const std::os::raw::c_char as
                                 *const xmlChar);
            if !base.is_null() {
                if !oldbase.is_null() {
                    newbase = xmlBuildURI(oldbase, base);
                    if !newbase.is_null() {
                        xmlFree.expect("non-null function pointer")(oldbase as
                                                                        *mut std::os::raw::c_void);
                        xmlFree.expect("non-null function pointer")(base as
                                                                        *mut std::os::raw::c_void);
                        oldbase = newbase
                    } else {
                        xmlFree.expect("non-null function pointer")(oldbase as
                                                                        *mut std::os::raw::c_void);
                        xmlFree.expect("non-null function pointer")(base as
                                                                        *mut std::os::raw::c_void);
                        return 0 as *mut xmlChar
                    }
                } else { oldbase = base }
                if xmlStrncmp(oldbase,
                              b"http://\x00" as *const u8 as
                                  *const std::os::raw::c_char as *mut xmlChar,
                              7 as std::os::raw::c_int) == 0 ||
                       xmlStrncmp(oldbase,
                                  b"ftp://\x00" as *const u8 as
                                      *const std::os::raw::c_char as *mut xmlChar,
                                  6 as std::os::raw::c_int) == 0 ||
                       xmlStrncmp(oldbase,
                                  b"urn:\x00" as *const u8 as
                                      *const std::os::raw::c_char as *mut xmlChar,
                                  4 as std::os::raw::c_int) == 0 {
                    return oldbase
                }
            }
        }
        cur = (*cur).parent
    }
    if !doc.is_null() && !(*doc).URL.is_null() {
        if oldbase.is_null() { return xmlStrdup((*doc).URL) }
        newbase = xmlBuildURI(oldbase, (*doc).URL);
        xmlFree.expect("non-null function pointer")(oldbase as
                                                        *mut std::os::raw::c_void);
        return newbase
    }
    return oldbase;
}
/* *
 * xmlNodeBufGetContent:
 * @buffer:  a buffer
 * @cur:  the node being read
 *
 * Read the value of a node @cur, this can be either the text carried
 * directly by this node if it's a TEXT node or the aggregate string
 * of the values carried by this node child's (TEXT and ENTITY_REF).
 * Entity references are substituted.
 * Fills up the buffer @buffer with this value
 *
 * Returns 0 in case of success and -1 in case of error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlNodeBufGetContent(mut buffer: xmlBufferPtr,
                                              mut cur: *const xmlNode)
 -> std::os::raw::c_int {
    let mut buf: xmlBufPtr = 0 as *mut xmlBuf;
    let mut ret: std::os::raw::c_int = 0;
    if cur.is_null() || buffer.is_null() { return -(1 as std::os::raw::c_int) }
    buf = xmlBufFromBuffer(buffer);
    ret = xmlBufGetNodeContent(buf, cur);
    buffer = xmlBufBackToBuffer(buf);
    if ret < 0 as std::os::raw::c_int || buffer.is_null() {
        return -(1 as std::os::raw::c_int)
    }
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlBufGetNodeContent:
 * @buf:  a buffer xmlBufPtr
 * @cur:  the node being read
 *
 * Read the value of a node @cur, this can be either the text carried
 * directly by this node if it's a TEXT node or the aggregate string
 * of the values carried by this node child's (TEXT and ENTITY_REF).
 * Entity references are substituted.
 * Fills up the buffer @buf with this value
 *
 * Returns 0 in case of success and -1 in case of error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlBufGetNodeContent(mut buf: xmlBufPtr,
                                              mut cur: *const xmlNode)
 -> std::os::raw::c_int {
    if cur.is_null() || buf.is_null() { return -(1 as std::os::raw::c_int) }
    match (*cur).type_0 as std::os::raw::c_uint {
        4 | 3 => { xmlBufCat(buf, (*cur).content); }
        11 | 1 => {
            let mut tmp: *const xmlNode = cur;
            while !tmp.is_null() {
                match (*tmp).type_0 as std::os::raw::c_uint {
                    4 | 3 => {
                        if !(*tmp).content.is_null() {
                            xmlBufCat(buf, (*tmp).content);
                        }
                    }
                    5 => { xmlBufGetNodeContent(buf, tmp); }
                    _ => { }
                }
                /*
                     * Skip to next node
                     */
                if !(*tmp).children.is_null() {
                    if (*(*tmp).children).type_0 as std::os::raw::c_uint !=
                           XML_ENTITY_DECL as std::os::raw::c_int as std::os::raw::c_uint {
                        tmp = (*tmp).children;
                        continue ;
                    }
                }
                if tmp == cur { break ; }
                if !(*tmp).next.is_null() {
                    tmp = (*tmp).next
                } else {
                    loop  {
                        tmp = (*tmp).parent;
                        if tmp.is_null() { break ; }
                        if tmp == cur {
                            tmp = 0 as *const xmlNode;
                            break ;
                        } else if !(*tmp).next.is_null() {
                            tmp = (*tmp).next;
                            break ;
                        } else if tmp.is_null() { break ; }
                    }
                }
            }
        }
        2 => {
            let mut attr: xmlAttrPtr = cur as xmlAttrPtr;
            let mut tmp_0: xmlNodePtr = (*attr).children;
            while !tmp_0.is_null() {
                if (*tmp_0).type_0 as std::os::raw::c_uint ==
                       XML_TEXT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
                    xmlBufCat(buf, (*tmp_0).content);
                } else { xmlBufGetNodeContent(buf, tmp_0 as *const xmlNode); }
                tmp_0 = (*tmp_0).next
            }
        }
        8 | 7 => { xmlBufCat(buf, (*cur).content); }
        5 => {
            let mut ent: xmlEntityPtr = 0 as *mut xmlEntity;
            let mut tmp_1: xmlNodePtr = 0 as *mut xmlNode;
            /* lookup entity declaration */
            ent = xmlGetDocEntity((*cur).doc, (*cur).name);
            if ent.is_null() { return -(1 as std::os::raw::c_int) }
            /* an entity content can be any "well balanced chunk",
                 * i.e. the result of the content [43] production:
                 * http://www.w3.org/TR/REC-xml#NT-content
                 * -> we iterate through child nodes and recursive call
                 * xmlNodeGetContent() which handles all possible node types */
            tmp_1 = (*ent).children;
            while !tmp_1.is_null() {
                xmlBufGetNodeContent(buf, tmp_1 as *const xmlNode);
                tmp_1 = (*tmp_1).next
            }
        }
        9 | 21 | 13 => {
            cur = (*cur).children;
            while !cur.is_null() {
                if (*cur).type_0 as std::os::raw::c_uint ==
                       XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint ||
                       (*cur).type_0 as std::os::raw::c_uint ==
                           XML_TEXT_NODE as std::os::raw::c_int as std::os::raw::c_uint ||
                       (*cur).type_0 as std::os::raw::c_uint ==
                           XML_CDATA_SECTION_NODE as std::os::raw::c_int as
                               std::os::raw::c_uint {
                    xmlBufGetNodeContent(buf, cur);
                }
                cur = (*cur).next
            }
        }
        18 => { xmlBufCat(buf, (*(cur as xmlNsPtr)).href); }
        6 | 10 | 12 | 14 | 19 | 20 | 15 | 16 | 17 | _ => { }
    }
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlNodeGetContent:
 * @cur:  the node being read
 *
 * Read the value of a node, this can be either the text carried
 * directly by this node if it's a TEXT node or the aggregate string
 * of the values carried by this node child's (TEXT and ENTITY_REF).
 * Entity references are substituted.
 * Returns a new #xmlChar * or NULL if no content is available.
 *     It's up to the caller to free the memory with xmlFree().
 */
#[no_mangle]
pub unsafe extern "C" fn xmlNodeGetContent(mut cur: *const xmlNode)
 -> *mut xmlChar {
    if cur.is_null() { return 0 as *mut xmlChar }
    match (*cur).type_0 as std::os::raw::c_uint {
        11 | 1 => {
            let mut buf: xmlBufPtr = 0 as *mut xmlBuf;
            let mut ret: *mut xmlChar = 0 as *mut xmlChar;
            buf = xmlBufCreateSize(64 as std::os::raw::c_int as size_t);
            if buf.is_null() { return 0 as *mut xmlChar }
            xmlBufGetNodeContent(buf, cur);
            ret = xmlBufDetach(buf);
            xmlBufFree(buf);
            return ret
        }
        2 => {
            return xmlGetPropNodeValueInternal(cur as xmlAttrPtr as
                                                   *const xmlAttr)
        }
        8 | 7 => {
            if !(*cur).content.is_null() { return xmlStrdup((*cur).content) }
            return 0 as *mut xmlChar
        }
        5 => {
            let mut ent: xmlEntityPtr = 0 as *mut xmlEntity;
            let mut buf_0: xmlBufPtr = 0 as *mut xmlBuf;
            let mut ret_0: *mut xmlChar = 0 as *mut xmlChar;
            /* lookup entity declaration */
            ent = xmlGetDocEntity((*cur).doc, (*cur).name);
            if ent.is_null() { return 0 as *mut xmlChar }
            buf_0 = xmlBufCreate();
            if buf_0.is_null() { return 0 as *mut xmlChar }
            xmlBufGetNodeContent(buf_0, cur);
            ret_0 = xmlBufDetach(buf_0);
            xmlBufFree(buf_0);
            return ret_0
        }
        6 | 10 | 12 | 14 | 19 | 20 => { return 0 as *mut xmlChar }
        9 | 21 | 13 => {
            let mut buf_1: xmlBufPtr = 0 as *mut xmlBuf;
            let mut ret_1: *mut xmlChar = 0 as *mut xmlChar;
            buf_1 = xmlBufCreate();
            if buf_1.is_null() { return 0 as *mut xmlChar }
            xmlBufGetNodeContent(buf_1, cur as xmlNodePtr as *const xmlNode);
            ret_1 = xmlBufDetach(buf_1);
            xmlBufFree(buf_1);
            return ret_1
        }
        18 => {
            let mut tmp: *mut xmlChar = 0 as *mut xmlChar;
            tmp = xmlStrdup((*(cur as xmlNsPtr)).href);
            return tmp
        }
        15 => {
            /* TODO !!! */
            return 0 as *mut xmlChar
        }
        16 => {
            /* TODO !!! */
            return 0 as *mut xmlChar
        }
        17 => {
            /* TODO !!! */
            return 0 as *mut xmlChar
        }
        4 | 3 => {
            if !(*cur).content.is_null() { return xmlStrdup((*cur).content) }
            return 0 as *mut xmlChar
        }
        _ => { }
    }
    return 0 as *mut xmlChar;
}
/* *
 * xmlNodeSetContent:
 * @cur:  the node being modified
 * @content:  the new value of the content
 *
 * Replace the content of a node.
 * NOTE: @content is supposed to be a piece of XML CDATA, so it allows entity
 *       references, but XML special chars need to be escaped first by using
 *       xmlEncodeEntitiesReentrant() resp. xmlEncodeSpecialChars().
 */
#[no_mangle]
pub unsafe extern "C" fn xmlNodeSetContent(mut cur: xmlNodePtr,
                                           mut content: *const xmlChar) {
    if cur.is_null() { return }
    match (*cur).type_0 as std::os::raw::c_uint {
        11 | 1 | 2 => {
            if !(*cur).children.is_null() {
                xmlFreeNodeList((*cur).children);
            }
            (*cur).children = xmlStringGetNodeList((*cur).doc, content);
            if !cur.is_null() {
                let mut ulccur: xmlNodePtr = (*cur).children;
                if ulccur.is_null() {
                    (*cur).last = 0 as *mut _xmlNode
                } else {
                    while !(*ulccur).next.is_null() {
                        (*ulccur).parent = cur;
                        ulccur = (*ulccur).next
                    }
                    (*ulccur).parent = cur;
                    (*cur).last = ulccur
                }
            }
        }
        3 | 4 | 5 | 6 | 7 | 8 => {
            if !(*cur).content.is_null() &&
                   (*cur).content !=
                       &mut (*cur).properties as *mut *mut _xmlAttr as
                           *mut xmlChar {
                if !(!(*cur).doc.is_null() && !(*(*cur).doc).dict.is_null() &&
                         xmlDictOwns((*(*cur).doc).dict, (*cur).content) != 0)
                   {
                    xmlFree.expect("non-null function pointer")((*cur).content
                                                                    as
                                                                    *mut std::os::raw::c_void);
                }
            }
            if !(*cur).children.is_null() {
                xmlFreeNodeList((*cur).children);
            }
            (*cur).children = 0 as *mut _xmlNode;
            (*cur).last = (*cur).children;
            if !content.is_null() {
                (*cur).content = xmlStrdup(content)
            } else { (*cur).content = 0 as *mut xmlChar }
            (*cur).properties = 0 as *mut _xmlAttr;
            (*cur).nsDef = 0 as *mut xmlNs
        }
        15 => { }
        17 => { }
        9 | 13 | 10 | 19 | 20 | 21 | 12 | 14 | 18 | 16 | _ => { }
    };
}
/* *
 * xmlNodeSetContentLen:
 * @cur:  the node being modified
 * @content:  the new value of the content
 * @len:  the size of @content
 *
 * Replace the content of a node.
 * NOTE: @content is supposed to be a piece of XML CDATA, so it allows entity
 *       references, but XML special chars need to be escaped first by using
 *       xmlEncodeEntitiesReentrant() resp. xmlEncodeSpecialChars().
 */
#[no_mangle]
pub unsafe extern "C" fn xmlNodeSetContentLen(mut cur: xmlNodePtr,
                                              mut content: *const xmlChar,
                                              mut len: std::os::raw::c_int) {
    if cur.is_null() { return }
    match (*cur).type_0 as std::os::raw::c_uint {
        11 | 1 | 2 => {
            if !(*cur).children.is_null() {
                xmlFreeNodeList((*cur).children);
            }
            (*cur).children =
                xmlStringLenGetNodeList((*cur).doc, content, len);
            if !cur.is_null() {
                let mut ulccur: xmlNodePtr = (*cur).children;
                if ulccur.is_null() {
                    (*cur).last = 0 as *mut _xmlNode
                } else {
                    while !(*ulccur).next.is_null() {
                        (*ulccur).parent = cur;
                        ulccur = (*ulccur).next
                    }
                    (*ulccur).parent = cur;
                    (*cur).last = ulccur
                }
            }
        }
        3 | 4 | 5 | 6 | 7 | 8 | 12 => {
            if !(*cur).content.is_null() &&
                   (*cur).content !=
                       &mut (*cur).properties as *mut *mut _xmlAttr as
                           *mut xmlChar {
                if !(!(*cur).doc.is_null() && !(*(*cur).doc).dict.is_null() &&
                         xmlDictOwns((*(*cur).doc).dict, (*cur).content) != 0)
                   {
                    xmlFree.expect("non-null function pointer")((*cur).content
                                                                    as
                                                                    *mut std::os::raw::c_void);
                }
            }
            if !(*cur).children.is_null() {
                xmlFreeNodeList((*cur).children);
            }
            (*cur).last = 0 as *mut _xmlNode;
            (*cur).children = (*cur).last;
            if !content.is_null() {
                (*cur).content = xmlStrndup(content, len)
            } else { (*cur).content = 0 as *mut xmlChar }
            (*cur).properties = 0 as *mut _xmlAttr;
            (*cur).nsDef = 0 as *mut xmlNs
        }
        15 => { }
        17 => { }
        9 | 14 | 13 | 10 | 18 | 19 | 20 | 21 | 16 | _ => { }
    };
}
/* LIBXML_TREE_ENABLED */
/* *
 * xmlNodeAddContentLen:
 * @cur:  the node being modified
 * @content:  extra content
 * @len:  the size of @content
 *
 * Append the extra substring to the node content.
 * NOTE: In contrast to xmlNodeSetContentLen(), @content is supposed to be
 *       raw text, so unescaped XML special chars are allowed, entity
 *       references are not supported.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlNodeAddContentLen(mut cur: xmlNodePtr,
                                              mut content: *const xmlChar,
                                              mut len: std::os::raw::c_int) {
    if cur.is_null() { return }
    if len <= 0 as std::os::raw::c_int { return }
    match (*cur).type_0 as std::os::raw::c_uint {
        11 | 1 => {
            let mut last: xmlNodePtr = 0 as *mut xmlNode;
            let mut newNode: xmlNodePtr = 0 as *mut xmlNode;
            let mut tmp: xmlNodePtr = 0 as *mut xmlNode;
            last = (*cur).last;
            newNode = xmlNewTextLen(content, len);
            if !newNode.is_null() {
                tmp = xmlAddChild(cur, newNode);
                if tmp != newNode { return }
                if !last.is_null() && (*last).next == newNode {
                    xmlTextMerge(last, newNode);
                }
            }
        }
        3 | 4 | 5 | 6 | 7 | 8 | 12 => {
            if !content.is_null() {
                if (*cur).content ==
                       &mut (*cur).properties as *mut *mut _xmlAttr as
                           *mut xmlChar ||
                       !(*cur).doc.is_null() && !(*(*cur).doc).dict.is_null()
                           &&
                           xmlDictOwns((*(*cur).doc).dict, (*cur).content) !=
                               0 {
                    (*cur).content =
                        xmlStrncatNew((*cur).content, content, len);
                    (*cur).properties = 0 as *mut _xmlAttr;
                    (*cur).nsDef = 0 as *mut xmlNs
                } else {
                    (*cur).content = xmlStrncat((*cur).content, content, len)
                }
            }
        }
        2 | 9 | 14 | 13 | 10 | 18 | 19 | 20 | 21 | 15 | 16 | 17 | _ => { }
    };
}
/* *
 * xmlNodeAddContent:
 * @cur:  the node being modified
 * @content:  extra content
 *
 * Append the extra substring to the node content.
 * NOTE: In contrast to xmlNodeSetContent(), @content is supposed to be
 *       raw text, so unescaped XML special chars are allowed, entity
 *       references are not supported.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlNodeAddContent(mut cur: xmlNodePtr,
                                           mut content: *const xmlChar) {
    let mut len: std::os::raw::c_int = 0;
    if cur.is_null() { return }
    if content.is_null() { return }
    len = xmlStrlen(content);
    xmlNodeAddContentLen(cur, content, len);
}
/* *
 * xmlTextMerge:
 * @first:  the first text node
 * @second:  the second text node being merged
 *
 * Merge two text nodes into one
 * Returns the first text node augmented
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextMerge(mut first: xmlNodePtr,
                                      mut second: xmlNodePtr) -> xmlNodePtr {
    if first.is_null() { return second }
    if second.is_null() { return first }
    if (*first).type_0 as std::os::raw::c_uint !=
           XML_TEXT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        return first
    }
    if (*second).type_0 as std::os::raw::c_uint !=
           XML_TEXT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        return first
    }
    if (*second).name != (*first).name { return first }
    xmlNodeAddContent(first, (*second).content);
    xmlUnlinkNode(second);
    xmlFreeNode(second);
    return first;
}
/* *
 * xmlGetNsList:
 * @doc:  the document
 * @node:  the current node
 *
 * Search all the namespace applying to a given element.
 * Returns an NULL terminated array of all the #xmlNsPtr found
 *         that need to be freed by the caller or NULL if no
 *         namespace if defined
 */
#[no_mangle]
pub unsafe extern "C" fn xmlGetNsList(mut doc: *const xmlDoc,
                                      mut node: *const xmlNode)
 -> *mut xmlNsPtr {
    let mut cur: xmlNsPtr = 0 as *mut xmlNs;
    let mut ret: *mut xmlNsPtr = 0 as *mut xmlNsPtr;
    let mut nbns: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut maxns: std::os::raw::c_int = 10 as std::os::raw::c_int;
    let mut i: std::os::raw::c_int = 0;
    if node.is_null() ||
           (*node).type_0 as std::os::raw::c_uint ==
               XML_NAMESPACE_DECL as std::os::raw::c_int as std::os::raw::c_uint {
        return 0 as *mut xmlNsPtr
    }
    while !node.is_null() {
        if (*node).type_0 as std::os::raw::c_uint ==
               XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
            cur = (*node).nsDef;
            while !cur.is_null() {
                if ret.is_null() {
                    ret =
                        xmlMalloc.expect("non-null function pointer")(((maxns
                                                                            +
                                                                            1
                                                                                as
                                                                                std::os::raw::c_int)
                                                                           as
                                                                           std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<xmlNsPtr>()
                                                                                                           as
                                                                                                           std::os::raw::c_ulong))
                            as *mut xmlNsPtr;
                    if ret.is_null() {
                        xmlTreeErrMemory(b"getting namespace list\x00" as
                                             *const u8 as
                                             *const std::os::raw::c_char);
                        return 0 as *mut xmlNsPtr
                    }
                    let ref mut fresh1 = *ret.offset(nbns as isize);
                    *fresh1 = 0 as xmlNsPtr
                }
                i = 0 as std::os::raw::c_int;
                while i < nbns {
                    if (*cur).prefix == (**ret.offset(i as isize)).prefix ||
                           xmlStrEqual((*cur).prefix,
                                       (**ret.offset(i as isize)).prefix) != 0
                       {
                        break ;
                    }
                    i += 1
                }
                if i >= nbns {
                    if nbns >= maxns {
                        maxns *= 2 as std::os::raw::c_int;
                        ret =
                            xmlRealloc.expect("non-null function pointer")(ret
                                                                               as
                                                                               *mut std::os::raw::c_void,
                                                                           ((maxns
                                                                                 +
                                                                                 1
                                                                                     as
                                                                                     std::os::raw::c_int)
                                                                                as
                                                                                std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<xmlNsPtr>()
                                                                                                                as
                                                                                                                std::os::raw::c_ulong))
                                as *mut xmlNsPtr;
                        if ret.is_null() {
                            xmlTreeErrMemory(b"getting namespace list\x00" as
                                                 *const u8 as
                                                 *const std::os::raw::c_char);
                            return 0 as *mut xmlNsPtr
                        }
                    }
                    let fresh2 = nbns;
                    nbns = nbns + 1;
                    let ref mut fresh3 = *ret.offset(fresh2 as isize);
                    *fresh3 = cur;
                    let ref mut fresh4 = *ret.offset(nbns as isize);
                    *fresh4 = 0 as xmlNsPtr
                }
                cur = (*cur).next
            }
        }
        node = (*node).parent
    }
    return ret;
}
/* LIBXML_TREE_ENABLED */
/*
* xmlTreeEnsureXMLDecl:
* @doc: the doc
*
* Ensures that there is an XML namespace declaration on the doc.
*
* Returns the XML ns-struct or NULL on API and internal errors.
*/
unsafe extern "C" fn xmlTreeEnsureXMLDecl(mut doc: xmlDocPtr) -> xmlNsPtr {
    if doc.is_null() { return 0 as xmlNsPtr }
    if !(*doc).oldNs.is_null() { return (*doc).oldNs }
    let mut ns: xmlNsPtr = 0 as *mut xmlNs;
    ns =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlNs>()
                                                          as std::os::raw::c_ulong) as
            xmlNsPtr;
    if ns.is_null() {
        xmlTreeErrMemory(b"allocating the XML namespace\x00" as *const u8 as
                             *const std::os::raw::c_char);
        return 0 as xmlNsPtr
    }
    memset(ns as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<xmlNs>() as std::os::raw::c_ulong);
    (*ns).type_0 = XML_NAMESPACE_DECL;
    (*ns).href =
        xmlStrdup(b"http://www.w3.org/XML/1998/namespace\x00" as *const u8 as
                      *const std::os::raw::c_char as *const xmlChar);
    (*ns).prefix =
        xmlStrdup(b"xml\x00" as *const u8 as *const std::os::raw::c_char as
                      *const xmlChar);
    (*doc).oldNs = ns;
    return ns;
}
/* *
 * xmlSearchNs:
 * @doc:  the document
 * @node:  the current node
 * @nameSpace:  the namespace prefix
 *
 * Search a Ns registered under a given name space for a document.
 * recurse on the parents until it finds the defined namespace
 * or return NULL otherwise.
 * @nameSpace can be NULL, this is a search for the default namespace.
 * We don't allow to cross entities boundaries. If you don't declare
 * the namespace within those you will be in troubles !!! A warning
 * is generated to cover this case.
 *
 * Returns the namespace pointer or NULL.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSearchNs(mut doc: xmlDocPtr, mut node: xmlNodePtr,
                                     mut nameSpace: *const xmlChar)
 -> xmlNsPtr {
    let mut cur: xmlNsPtr = 0 as *mut xmlNs;
    let mut orig: *const xmlNode = node as *const xmlNode;
    if node.is_null() ||
           (*node).type_0 as std::os::raw::c_uint ==
               XML_NAMESPACE_DECL as std::os::raw::c_int as std::os::raw::c_uint {
        return 0 as xmlNsPtr
    }
    if !nameSpace.is_null() &&
           xmlStrEqual(nameSpace,
                       b"xml\x00" as *const u8 as *const std::os::raw::c_char as
                           *const xmlChar) != 0 {
        if doc.is_null() &&
               (*node).type_0 as std::os::raw::c_uint ==
                   XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
            /*
	     * The XML-1.0 namespace is normally held on the root
	     * element. In this case exceptionally create it on the
	     * node element.
	     */
            cur =
                xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlNs>()
                                                                  as
                                                                  std::os::raw::c_ulong)
                    as xmlNsPtr;
            if cur.is_null() {
                xmlTreeErrMemory(b"searching namespace\x00" as *const u8 as
                                     *const std::os::raw::c_char);
                return 0 as xmlNsPtr
            }
            memset(cur as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
                   ::std::mem::size_of::<xmlNs>() as std::os::raw::c_ulong);
            (*cur).type_0 = XML_NAMESPACE_DECL;
            (*cur).href =
                xmlStrdup(b"http://www.w3.org/XML/1998/namespace\x00" as
                              *const u8 as *const std::os::raw::c_char as
                              *const xmlChar);
            (*cur).prefix =
                xmlStrdup(b"xml\x00" as *const u8 as *const std::os::raw::c_char as
                              *const xmlChar);
            (*cur).next = (*node).nsDef;
            (*node).nsDef = cur;
            return cur
        }
        if doc.is_null() {
            doc = (*node).doc;
            if doc.is_null() { return 0 as xmlNsPtr }
        }
        /*
	* Return the XML namespace declaration held by the doc.
	*/
        if (*doc).oldNs.is_null() {
            return xmlTreeEnsureXMLDecl(doc)
        } else { return (*doc).oldNs }
    }
    while !node.is_null() {
        if (*node).type_0 as std::os::raw::c_uint ==
               XML_ENTITY_REF_NODE as std::os::raw::c_int as std::os::raw::c_uint ||
               (*node).type_0 as std::os::raw::c_uint ==
                   XML_ENTITY_NODE as std::os::raw::c_int as std::os::raw::c_uint ||
               (*node).type_0 as std::os::raw::c_uint ==
                   XML_ENTITY_DECL as std::os::raw::c_int as std::os::raw::c_uint {
            return 0 as xmlNsPtr
        }
        if (*node).type_0 as std::os::raw::c_uint ==
               XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
            cur = (*node).nsDef;
            while !cur.is_null() {
                if (*cur).prefix.is_null() && nameSpace.is_null() &&
                       !(*cur).href.is_null() {
                    return cur
                }
                if !(*cur).prefix.is_null() && !nameSpace.is_null() &&
                       !(*cur).href.is_null() &&
                       xmlStrEqual((*cur).prefix, nameSpace) != 0 {
                    return cur
                }
                cur = (*cur).next
            }
            if orig != node as *const xmlNode {
                cur = (*node).ns;
                if !cur.is_null() {
                    if (*cur).prefix.is_null() && nameSpace.is_null() &&
                           !(*cur).href.is_null() {
                        return cur
                    }
                    if !(*cur).prefix.is_null() && !nameSpace.is_null() &&
                           !(*cur).href.is_null() &&
                           xmlStrEqual((*cur).prefix, nameSpace) != 0 {
                        return cur
                    }
                }
            }
        }
        node = (*node).parent
    }
    return 0 as xmlNsPtr;
}
/* *
 * xmlNsInScope:
 * @doc:  the document
 * @node:  the current node
 * @ancestor:  the ancestor carrying the namespace
 * @prefix:  the namespace prefix
 *
 * Verify that the given namespace held on @ancestor is still in scope
 * on node.
 *
 * Returns 1 if true, 0 if false and -1 in case of error.
 */
unsafe extern "C" fn xmlNsInScope(mut doc: xmlDocPtr, mut node: xmlNodePtr,
                                  mut ancestor: xmlNodePtr,
                                  mut prefix: *const xmlChar) -> std::os::raw::c_int {
    let mut tst: xmlNsPtr = 0 as *mut xmlNs;
    while !node.is_null() && node != ancestor {
        if (*node).type_0 as std::os::raw::c_uint ==
               XML_ENTITY_REF_NODE as std::os::raw::c_int as std::os::raw::c_uint ||
               (*node).type_0 as std::os::raw::c_uint ==
                   XML_ENTITY_NODE as std::os::raw::c_int as std::os::raw::c_uint ||
               (*node).type_0 as std::os::raw::c_uint ==
                   XML_ENTITY_DECL as std::os::raw::c_int as std::os::raw::c_uint {
            return -(1 as std::os::raw::c_int)
        }
        if (*node).type_0 as std::os::raw::c_uint ==
               XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
            tst = (*node).nsDef;
            while !tst.is_null() {
                if (*tst).prefix.is_null() && prefix.is_null() {
                    return 0 as std::os::raw::c_int
                }
                if !(*tst).prefix.is_null() && !prefix.is_null() &&
                       xmlStrEqual((*tst).prefix, prefix) != 0 {
                    return 0 as std::os::raw::c_int
                }
                tst = (*tst).next
            }
        }
        node = (*node).parent
    }
    if node != ancestor { return -(1 as std::os::raw::c_int) }
    return 1 as std::os::raw::c_int;
}
/* *
 * xmlSearchNsByHref:
 * @doc:  the document
 * @node:  the current node
 * @href:  the namespace value
 *
 * Search a Ns aliasing a given URI. Recurse on the parents until it finds
 * the defined namespace or return NULL otherwise.
 * Returns the namespace pointer or NULL.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSearchNsByHref(mut doc: xmlDocPtr,
                                           mut node: xmlNodePtr,
                                           mut href: *const xmlChar)
 -> xmlNsPtr {
    let mut cur: xmlNsPtr = 0 as *mut xmlNs;
    let mut orig: xmlNodePtr = node;
    let mut is_attr: std::os::raw::c_int = 0;
    if node.is_null() ||
           (*node).type_0 as std::os::raw::c_uint ==
               XML_NAMESPACE_DECL as std::os::raw::c_int as std::os::raw::c_uint ||
           href.is_null() {
        return 0 as xmlNsPtr
    }
    if xmlStrEqual(href,
                   b"http://www.w3.org/XML/1998/namespace\x00" as *const u8 as
                       *const std::os::raw::c_char as *const xmlChar) != 0 {
        /*
         * Only the document can hold the XML spec namespace.
         */
        if doc.is_null() &&
               (*node).type_0 as std::os::raw::c_uint ==
                   XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
            /*
             * The XML-1.0 namespace is normally held on the root
             * element. In this case exceptionally create it on the
             * node element.
             */
            cur =
                xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlNs>()
                                                                  as
                                                                  std::os::raw::c_ulong)
                    as xmlNsPtr;
            if cur.is_null() {
                xmlTreeErrMemory(b"searching namespace\x00" as *const u8 as
                                     *const std::os::raw::c_char);
                return 0 as xmlNsPtr
            }
            memset(cur as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
                   ::std::mem::size_of::<xmlNs>() as std::os::raw::c_ulong);
            (*cur).type_0 = XML_NAMESPACE_DECL;
            (*cur).href =
                xmlStrdup(b"http://www.w3.org/XML/1998/namespace\x00" as
                              *const u8 as *const std::os::raw::c_char as
                              *const xmlChar);
            (*cur).prefix =
                xmlStrdup(b"xml\x00" as *const u8 as *const std::os::raw::c_char as
                              *const xmlChar);
            (*cur).next = (*node).nsDef;
            (*node).nsDef = cur;
            return cur
        }
        if doc.is_null() {
            doc = (*node).doc;
            if doc.is_null() { return 0 as xmlNsPtr }
        }
        /*
	* Return the XML namespace declaration held by the doc.
	*/
        if (*doc).oldNs.is_null() {
            return xmlTreeEnsureXMLDecl(doc)
        } else { return (*doc).oldNs }
    }
    is_attr =
        ((*node).type_0 as std::os::raw::c_uint ==
             XML_ATTRIBUTE_NODE as std::os::raw::c_int as std::os::raw::c_uint) as
            std::os::raw::c_int;
    while !node.is_null() {
        if (*node).type_0 as std::os::raw::c_uint ==
               XML_ENTITY_REF_NODE as std::os::raw::c_int as std::os::raw::c_uint ||
               (*node).type_0 as std::os::raw::c_uint ==
                   XML_ENTITY_NODE as std::os::raw::c_int as std::os::raw::c_uint ||
               (*node).type_0 as std::os::raw::c_uint ==
                   XML_ENTITY_DECL as std::os::raw::c_int as std::os::raw::c_uint {
            return 0 as xmlNsPtr
        }
        if (*node).type_0 as std::os::raw::c_uint ==
               XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
            cur = (*node).nsDef;
            while !cur.is_null() {
                if !(*cur).href.is_null() && !href.is_null() &&
                       xmlStrEqual((*cur).href, href) != 0 {
                    if (is_attr == 0 || !(*cur).prefix.is_null()) &&
                           xmlNsInScope(doc, orig, node, (*cur).prefix) ==
                               1 as std::os::raw::c_int {
                        return cur
                    }
                }
                cur = (*cur).next
            }
            if orig != node {
                cur = (*node).ns;
                if !cur.is_null() {
                    if !(*cur).href.is_null() && !href.is_null() &&
                           xmlStrEqual((*cur).href, href) != 0 {
                        if (is_attr == 0 || !(*cur).prefix.is_null()) &&
                               xmlNsInScope(doc, orig, node, (*cur).prefix) ==
                                   1 as std::os::raw::c_int {
                            return cur
                        }
                    }
                }
            }
        }
        node = (*node).parent
    }
    return 0 as xmlNsPtr;
}
/* ***********************************************************************
 *									*
 *		Forward declarations					*
 *									*
 ************************************************************************/
/* *
 * xmlNewReconciliedNs:
 * @doc:  the document
 * @tree:  a node expected to hold the new namespace
 * @ns:  the original namespace
 *
 * This function tries to locate a namespace definition in a tree
 * ancestors, or create a new namespace definition node similar to
 * @ns trying to reuse the same prefix. However if the given prefix is
 * null (default namespace) or reused within the subtree defined by
 * @tree or on one of its ancestors then a new prefix is generated.
 * Returns the (new) namespace definition or NULL in case of error
 */
unsafe extern "C" fn xmlNewReconciliedNs(mut doc: xmlDocPtr,
                                         mut tree: xmlNodePtr,
                                         mut ns: xmlNsPtr) -> xmlNsPtr {
    let mut def: xmlNsPtr = 0 as *mut xmlNs;
    let mut prefix: [xmlChar; 50] = [0; 50];
    let mut counter: std::os::raw::c_int = 1 as std::os::raw::c_int;
    if tree.is_null() ||
           (*tree).type_0 as std::os::raw::c_uint !=
               XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        return 0 as xmlNsPtr
    }
    if ns.is_null() ||
           (*ns).type_0 as std::os::raw::c_uint !=
               XML_NAMESPACE_DECL as std::os::raw::c_int as std::os::raw::c_uint {
        return 0 as xmlNsPtr
    }
    /*
     * Search an existing namespace definition inherited.
     */
    def = xmlSearchNsByHref(doc, tree, (*ns).href);
    if !def.is_null() { return def }
    /*
     * Find a close prefix which is not already in use.
     * Let's strip namespace prefixes longer than 20 chars !
     */
    if (*ns).prefix.is_null() {
        snprintf(prefix.as_mut_ptr() as *mut std::os::raw::c_char,
                 ::std::mem::size_of::<[xmlChar; 50]>() as std::os::raw::c_ulong,
                 b"default\x00" as *const u8 as *const std::os::raw::c_char);
    } else {
        snprintf(prefix.as_mut_ptr() as *mut std::os::raw::c_char,
                 ::std::mem::size_of::<[xmlChar; 50]>() as std::os::raw::c_ulong,
                 b"%.20s\x00" as *const u8 as *const std::os::raw::c_char,
                 (*ns).prefix as *mut std::os::raw::c_char);
    }
    def = xmlSearchNs(doc, tree, prefix.as_mut_ptr());
    while !def.is_null() {
        if counter > 1000 as std::os::raw::c_int { return 0 as xmlNsPtr }
        if (*ns).prefix.is_null() {
            let fresh5 = counter;
            counter = counter + 1;
            snprintf(prefix.as_mut_ptr() as *mut std::os::raw::c_char,
                     ::std::mem::size_of::<[xmlChar; 50]>() as std::os::raw::c_ulong,
                     b"default%d\x00" as *const u8 as *const std::os::raw::c_char,
                     fresh5);
        } else {
            let fresh6 = counter;
            counter = counter + 1;
            snprintf(prefix.as_mut_ptr() as *mut std::os::raw::c_char,
                     ::std::mem::size_of::<[xmlChar; 50]>() as std::os::raw::c_ulong,
                     b"%.20s%d\x00" as *const u8 as *const std::os::raw::c_char,
                     (*ns).prefix as *mut std::os::raw::c_char, fresh6);
        }
        def = xmlSearchNs(doc, tree, prefix.as_mut_ptr())
    }
    /*
     * OK, now we are ready to create a new one.
     */
    def = xmlNewNs(tree, (*ns).href, prefix.as_mut_ptr());
    return def;
}
/* *
 * xmlReconciliateNs:
 * @doc:  the document
 * @tree:  a node defining the subtree to reconciliate
 *
 * This function checks that all the namespaces declared within the given
 * tree are properly declared. This is needed for example after Copy or Cut
 * and then paste operations. The subtree may still hold pointers to
 * namespace declarations outside the subtree or invalid/masked. As much
 * as possible the function try to reuse the existing namespaces found in
 * the new environment. If not possible the new namespaces are redeclared
 * on @tree at the top of the given subtree.
 * Returns the number of namespace declarations created or -1 in case of error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlReconciliateNs(mut doc: xmlDocPtr,
                                           mut tree: xmlNodePtr)
 -> std::os::raw::c_int {
    let mut oldNs: *mut xmlNsPtr = 0 as *mut xmlNsPtr;
    let mut newNs: *mut xmlNsPtr = 0 as *mut xmlNsPtr;
    let mut sizeCache: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut nbCache: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut n: xmlNsPtr = 0 as *mut xmlNs;
    let mut node: xmlNodePtr = tree;
    let mut attr: xmlAttrPtr = 0 as *mut xmlAttr;
    let mut ret: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut i: std::os::raw::c_int = 0;
    if node.is_null() ||
           (*node).type_0 as std::os::raw::c_uint !=
               XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        return -(1 as std::os::raw::c_int)
    }
    if doc.is_null() ||
           (*doc).type_0 as std::os::raw::c_uint !=
               XML_DOCUMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        return -(1 as std::os::raw::c_int)
    }
    if (*node).doc != doc { return -(1 as std::os::raw::c_int) }
    while !node.is_null() {
        /*
	 * Reconciliate the node namespace
	 */
        if !(*node).ns.is_null() {
            /*
	     * initialize the cache if needed
	     */
            if sizeCache == 0 as std::os::raw::c_int {
                sizeCache = 10 as std::os::raw::c_int;
                oldNs =
                    xmlMalloc.expect("non-null function pointer")((sizeCache
                                                                       as
                                                                       std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<xmlNsPtr>()
                                                                                                       as
                                                                                                       std::os::raw::c_ulong))
                        as *mut xmlNsPtr;
                if oldNs.is_null() {
                    xmlTreeErrMemory(b"fixing namespaces\x00" as *const u8 as
                                         *const std::os::raw::c_char);
                    return -(1 as std::os::raw::c_int)
                }
                newNs =
                    xmlMalloc.expect("non-null function pointer")((sizeCache
                                                                       as
                                                                       std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<xmlNsPtr>()
                                                                                                       as
                                                                                                       std::os::raw::c_ulong))
                        as *mut xmlNsPtr;
                if newNs.is_null() {
                    xmlTreeErrMemory(b"fixing namespaces\x00" as *const u8 as
                                         *const std::os::raw::c_char);
                    xmlFree.expect("non-null function pointer")(oldNs as
                                                                    *mut std::os::raw::c_void);
                    return -(1 as std::os::raw::c_int)
                }
            }
            i = 0 as std::os::raw::c_int;
            while i < nbCache {
                if *oldNs.offset(i as isize) == (*node).ns {
                    (*node).ns = *newNs.offset(i as isize);
                    break ;
                } else { i += 1 }
            }
            if i == nbCache {
                /*
		 * OK we need to recreate a new namespace definition
		 */
                n = xmlNewReconciliedNs(doc, tree, (*node).ns);
                if !n.is_null() {
                    /* :-( what if else ??? */
                    /*
		     * check if we need to grow the cache buffers.
		     */
                    if sizeCache <= nbCache {
                        sizeCache *= 2 as std::os::raw::c_int;
                        oldNs =
                            xmlRealloc.expect("non-null function pointer")(oldNs
                                                                               as
                                                                               *mut std::os::raw::c_void,
                                                                           (sizeCache
                                                                                as
                                                                                std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<xmlNsPtr>()
                                                                                                                as
                                                                                                                std::os::raw::c_ulong))
                                as *mut xmlNsPtr;
                        if oldNs.is_null() {
                            xmlTreeErrMemory(b"fixing namespaces\x00" as
                                                 *const u8 as
                                                 *const std::os::raw::c_char);
                            xmlFree.expect("non-null function pointer")(newNs
                                                                            as
                                                                            *mut std::os::raw::c_void);
                            return -(1 as std::os::raw::c_int)
                        }
                        newNs =
                            xmlRealloc.expect("non-null function pointer")(newNs
                                                                               as
                                                                               *mut std::os::raw::c_void,
                                                                           (sizeCache
                                                                                as
                                                                                std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<xmlNsPtr>()
                                                                                                                as
                                                                                                                std::os::raw::c_ulong))
                                as *mut xmlNsPtr;
                        if newNs.is_null() {
                            xmlTreeErrMemory(b"fixing namespaces\x00" as
                                                 *const u8 as
                                                 *const std::os::raw::c_char);
                            xmlFree.expect("non-null function pointer")(oldNs
                                                                            as
                                                                            *mut std::os::raw::c_void);
                            return -(1 as std::os::raw::c_int)
                        }
                    }
                    let ref mut fresh7 = *newNs.offset(nbCache as isize);
                    *fresh7 = n;
                    let fresh8 = nbCache;
                    nbCache = nbCache + 1;
                    let ref mut fresh9 = *oldNs.offset(fresh8 as isize);
                    *fresh9 = (*node).ns;
                    (*node).ns = n
                }
            }
        }
        /*
	 * now check for namespace hold by attributes on the node.
	 */
        if (*node).type_0 as std::os::raw::c_uint ==
               XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
            attr = (*node).properties;
            while !attr.is_null() {
                if !(*attr).ns.is_null() {
                    /*
		     * initialize the cache if needed
		     */
                    if sizeCache == 0 as std::os::raw::c_int {
                        sizeCache = 10 as std::os::raw::c_int;
                        oldNs =
                            xmlMalloc.expect("non-null function pointer")((sizeCache
                                                                               as
                                                                               std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<xmlNsPtr>()
                                                                                                               as
                                                                                                               std::os::raw::c_ulong))
                                as *mut xmlNsPtr;
                        if oldNs.is_null() {
                            xmlTreeErrMemory(b"fixing namespaces\x00" as
                                                 *const u8 as
                                                 *const std::os::raw::c_char);
                            return -(1 as std::os::raw::c_int)
                        }
                        newNs =
                            xmlMalloc.expect("non-null function pointer")((sizeCache
                                                                               as
                                                                               std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<xmlNsPtr>()
                                                                                                               as
                                                                                                               std::os::raw::c_ulong))
                                as *mut xmlNsPtr;
                        if newNs.is_null() {
                            xmlTreeErrMemory(b"fixing namespaces\x00" as
                                                 *const u8 as
                                                 *const std::os::raw::c_char);
                            xmlFree.expect("non-null function pointer")(oldNs
                                                                            as
                                                                            *mut std::os::raw::c_void);
                            return -(1 as std::os::raw::c_int)
                        }
                    }
                    i = 0 as std::os::raw::c_int;
                    while i < nbCache {
                        if *oldNs.offset(i as isize) == (*attr).ns {
                            (*attr).ns = *newNs.offset(i as isize);
                            break ;
                        } else { i += 1 }
                    }
                    if i == nbCache {
                        /*
			 * OK we need to recreate a new namespace definition
			 */
                        n = xmlNewReconciliedNs(doc, tree, (*attr).ns);
                        if !n.is_null() {
                            /* :-( what if else ??? */
                            /*
			     * check if we need to grow the cache buffers.
			     */
                            if sizeCache <= nbCache {
                                sizeCache *= 2 as std::os::raw::c_int;
                                oldNs =
                                    xmlRealloc.expect("non-null function pointer")(oldNs
                                                                                       as
                                                                                       *mut std::os::raw::c_void,
                                                                                   (sizeCache
                                                                                        as
                                                                                        std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<xmlNsPtr>()
                                                                                                                        as
                                                                                                                        std::os::raw::c_ulong))
                                        as *mut xmlNsPtr;
                                if oldNs.is_null() {
                                    xmlTreeErrMemory(b"fixing namespaces\x00"
                                                         as *const u8 as
                                                         *const std::os::raw::c_char);
                                    xmlFree.expect("non-null function pointer")(newNs
                                                                                    as
                                                                                    *mut std::os::raw::c_void);
                                    return -(1 as std::os::raw::c_int)
                                }
                                newNs =
                                    xmlRealloc.expect("non-null function pointer")(newNs
                                                                                       as
                                                                                       *mut std::os::raw::c_void,
                                                                                   (sizeCache
                                                                                        as
                                                                                        std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<xmlNsPtr>()
                                                                                                                        as
                                                                                                                        std::os::raw::c_ulong))
                                        as *mut xmlNsPtr;
                                if newNs.is_null() {
                                    xmlTreeErrMemory(b"fixing namespaces\x00"
                                                         as *const u8 as
                                                         *const std::os::raw::c_char);
                                    xmlFree.expect("non-null function pointer")(oldNs
                                                                                    as
                                                                                    *mut std::os::raw::c_void);
                                    return -(1 as std::os::raw::c_int)
                                }
                            }
                            let ref mut fresh10 =
                                *newNs.offset(nbCache as isize);
                            *fresh10 = n;
                            let fresh11 = nbCache;
                            nbCache = nbCache + 1;
                            let ref mut fresh12 =
                                *oldNs.offset(fresh11 as isize);
                            *fresh12 = (*attr).ns;
                            (*attr).ns = n
                        }
                    }
                }
                attr = (*attr).next
            }
        }
        /*
	 * Browse the full subtree, deep first
	 */
        if !(*node).children.is_null() &&
               (*node).type_0 as std::os::raw::c_uint !=
                   XML_ENTITY_REF_NODE as std::os::raw::c_int as std::os::raw::c_uint {
            /* deep first */
            node = (*node).children
        } else if node != tree && !(*node).next.is_null() {
            /* then siblings */
            node = (*node).next
        } else {
            if !(node != tree) { break ; }
            /* go up to parents->next if needed */
            while node != tree {
                if !(*node).parent.is_null() { node = (*node).parent }
                if node != tree && !(*node).next.is_null() {
                    node = (*node).next;
                    break ;
                } else {
                    if !(*node).parent.is_null() { continue ; }
                    node = 0 as xmlNodePtr;
                    break ;
                }
            }
            /* exit condition */
            if node == tree { node = 0 as xmlNodePtr }
        }
    }
    if !oldNs.is_null() {
        xmlFree.expect("non-null function pointer")(oldNs as
                                                        *mut std::os::raw::c_void);
    }
    if !newNs.is_null() {
        xmlFree.expect("non-null function pointer")(newNs as
                                                        *mut std::os::raw::c_void);
    }
    return ret;
}
/* LIBXML_TREE_ENABLED */
unsafe extern "C" fn xmlGetPropNodeInternal(mut node: *const xmlNode,
                                            mut name: *const xmlChar,
                                            mut nsName: *const xmlChar,
                                            mut useDTD: std::os::raw::c_int)
 -> xmlAttrPtr {
    let mut prop: xmlAttrPtr = 0 as *mut xmlAttr;
    if node.is_null() ||
           (*node).type_0 as std::os::raw::c_uint !=
               XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint ||
           name.is_null() {
        return 0 as xmlAttrPtr
    }
    if !(*node).properties.is_null() {
        prop = (*node).properties;
        if nsName.is_null() {
            loop 
                 /*
	    * We want the attr to be in no namespace.
	    */
                 {
                if (*prop).ns.is_null() &&
                       xmlStrEqual((*prop).name, name) != 0 {
                    return prop
                }
                prop = (*prop).next;
                if prop.is_null() { break ; }
            }
        } else {
            loop 
                 /*
	    * We want the attr to be in the specified namespace.
	    */
                 {
                if !(*prop).ns.is_null() &&
                       xmlStrEqual((*prop).name, name) != 0 &&
                       ((*(*prop).ns).href == nsName ||
                            xmlStrEqual((*(*prop).ns).href, nsName) != 0) {
                    return prop
                }
                prop = (*prop).next;
                if prop.is_null() { break ; }
            }
        }
    }
    if useDTD == 0 { return 0 as xmlAttrPtr }
    /*
     * Check if there is a default/fixed attribute declaration in
     * the internal or external subset.
     */
    if !(*node).doc.is_null() && !(*(*node).doc).intSubset.is_null() {
        let mut doc: xmlDocPtr = (*node).doc;
        let mut attrDecl: xmlAttributePtr = 0 as xmlAttributePtr;
        let mut elemQName: *mut xmlChar = 0 as *mut xmlChar;
        let mut tmpstr: *mut xmlChar = 0 as *mut xmlChar;
        /*
	* We need the QName of the element for the DTD-lookup.
	*/
        if !(*node).ns.is_null() && !(*(*node).ns).prefix.is_null() {
            tmpstr = xmlStrdup((*(*node).ns).prefix);
            tmpstr =
                xmlStrcat(tmpstr,
                          b":\x00" as *const u8 as *const std::os::raw::c_char as
                              *mut xmlChar);
            tmpstr = xmlStrcat(tmpstr, (*node).name);
            if tmpstr.is_null() { return 0 as xmlAttrPtr }
            elemQName = tmpstr
        } else { elemQName = (*node).name as *mut xmlChar }
        if nsName.is_null() {
            /*
	    * The common and nice case: Attr in no namespace.
	    */
            attrDecl =
                xmlGetDtdQAttrDesc((*doc).intSubset, elemQName, name,
                                   0 as *const xmlChar);
            if attrDecl.is_null() && !(*doc).extSubset.is_null() {
                attrDecl =
                    xmlGetDtdQAttrDesc((*doc).extSubset, elemQName, name,
                                       0 as *const xmlChar)
            }
        } else {
            let mut nsList: *mut xmlNsPtr = 0 as *mut xmlNsPtr;
            let mut cur: *mut xmlNsPtr = 0 as *mut xmlNsPtr;
            /*
	    * The ugly case: Search using the prefixes of in-scope
	    * ns-decls corresponding to @nsName.
	    */
            nsList = xmlGetNsList((*node).doc, node);
            if nsList.is_null() {
                if !tmpstr.is_null() {
                    xmlFree.expect("non-null function pointer")(tmpstr as
                                                                    *mut std::os::raw::c_void);
                }
                return 0 as xmlAttrPtr
            }
            cur = nsList;
            while !(*cur).is_null() {
                if xmlStrEqual((**cur).href, nsName) != 0 {
                    attrDecl =
                        xmlGetDtdQAttrDesc((*doc).intSubset, elemQName, name,
                                           (**cur).prefix);
                    if !attrDecl.is_null() { break ; }
                    if !(*doc).extSubset.is_null() {
                        attrDecl =
                            xmlGetDtdQAttrDesc((*doc).extSubset, elemQName,
                                               name, (**cur).prefix);
                        if !attrDecl.is_null() { break ; }
                    }
                }
                cur = cur.offset(1)
            }
            xmlFree.expect("non-null function pointer")(nsList as
                                                            *mut std::os::raw::c_void);
        }
        if !tmpstr.is_null() {
            xmlFree.expect("non-null function pointer")(tmpstr as
                                                            *mut std::os::raw::c_void);
        }
        /*
	* Only default/fixed attrs are relevant.
	*/
        if !attrDecl.is_null() && !(*attrDecl).defaultValue.is_null() {
            return attrDecl as xmlAttrPtr
        }
    }
    /* LIBXML_TREE_ENABLED */
    return 0 as xmlAttrPtr;
}
unsafe extern "C" fn xmlGetPropNodeValueInternal(mut prop: *const xmlAttr)
 -> *mut xmlChar {
    if prop.is_null() { return 0 as *mut xmlChar }
    if (*prop).type_0 as std::os::raw::c_uint ==
           XML_ATTRIBUTE_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        /*
	* Note that we return at least the empty string.
	*   TODO: Do we really always want that?
	*/
        if !(*prop).children.is_null() {
            if (*(*prop).children).next.is_null() &&
                   ((*(*prop).children).type_0 as std::os::raw::c_uint ==
                        XML_TEXT_NODE as std::os::raw::c_int as std::os::raw::c_uint ||
                        (*(*prop).children).type_0 as std::os::raw::c_uint ==
                            XML_CDATA_SECTION_NODE as std::os::raw::c_int as
                                std::os::raw::c_uint) {
                /*
		* Optimization for the common case: only 1 text node.
		*/
                return xmlStrdup((*(*prop).children).content)
            } else {
                let mut ret: *mut xmlChar = 0 as *mut xmlChar;
                ret =
                    xmlNodeListGetString((*prop).doc, (*prop).children,
                                         1 as std::os::raw::c_int);
                if !ret.is_null() { return ret }
            }
        }
        return xmlStrdup(b"\x00" as *const u8 as *const std::os::raw::c_char as
                             *mut xmlChar)
    } else {
        if (*prop).type_0 as std::os::raw::c_uint ==
               XML_ATTRIBUTE_DECL as std::os::raw::c_int as std::os::raw::c_uint {
            return xmlStrdup((*(prop as xmlAttributePtr)).defaultValue)
        }
    }
    return 0 as *mut xmlChar;
}
/* *
 * xmlHasProp:
 * @node:  the node
 * @name:  the attribute name
 *
 * Search an attribute associated to a node
 * This function also looks in DTD attribute declaration for #FIXED or
 * default declaration values unless DTD use has been turned off.
 *
 * Returns the attribute or the attribute declaration or NULL if
 *         neither was found.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlHasProp(mut node: *const xmlNode,
                                    mut name: *const xmlChar) -> xmlAttrPtr {
    let mut prop: xmlAttrPtr = 0 as *mut xmlAttr;
    let mut doc: xmlDocPtr = 0 as *mut xmlDoc;
    if node.is_null() ||
           (*node).type_0 as std::os::raw::c_uint !=
               XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint ||
           name.is_null() {
        return 0 as xmlAttrPtr
    }
    /*
     * Check on the properties attached to the node
     */
    prop = (*node).properties;
    while !prop.is_null() {
        if xmlStrEqual((*prop).name, name) != 0 { return prop }
        prop = (*prop).next
    }
    if xmlCheckDTD == 0 { return 0 as xmlAttrPtr }
    /*
     * Check if there is a default declaration in the internal
     * or external subsets
     */
    doc = (*node).doc;
    if !doc.is_null() {
        let mut attrDecl: xmlAttributePtr = 0 as *mut xmlAttribute;
        if !(*doc).intSubset.is_null() {
            attrDecl =
                xmlGetDtdAttrDesc((*doc).intSubset, (*node).name, name);
            if attrDecl.is_null() && !(*doc).extSubset.is_null() {
                attrDecl =
                    xmlGetDtdAttrDesc((*doc).extSubset, (*node).name, name)
            }
            if !attrDecl.is_null() && !(*attrDecl).defaultValue.is_null() {
                /* return attribute declaration only if a default value is given
                 (that includes #FIXED declarations) */
                return attrDecl as xmlAttrPtr
            }
        }
    }
    return 0 as xmlAttrPtr;
}
/* *
 * xmlHasNsProp:
 * @node:  the node
 * @name:  the attribute name
 * @nameSpace:  the URI of the namespace
 *
 * Search for an attribute associated to a node
 * This attribute has to be anchored in the namespace specified.
 * This does the entity substitution.
 * This function looks in DTD attribute declaration for #FIXED or
 * default declaration values unless DTD use has been turned off.
 * Note that a namespace of NULL indicates to use the default namespace.
 *
 * Returns the attribute or the attribute declaration or NULL
 *     if neither was found.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlHasNsProp(mut node: *const xmlNode,
                                      mut name: *const xmlChar,
                                      mut nameSpace: *const xmlChar)
 -> xmlAttrPtr {
    return xmlGetPropNodeInternal(node, name, nameSpace, xmlCheckDTD);
}
/* *
 * xmlGetProp:
 * @node:  the node
 * @name:  the attribute name
 *
 * Search and get the value of an attribute associated to a node
 * This does the entity substitution.
 * This function looks in DTD attribute declaration for #FIXED or
 * default declaration values unless DTD use has been turned off.
 * NOTE: this function acts independently of namespaces associated
 *       to the attribute. Use xmlGetNsProp() or xmlGetNoNsProp()
 *       for namespace aware processing.
 *
 * Returns the attribute value or NULL if not found.
 *     It's up to the caller to free the memory with xmlFree().
 */
#[no_mangle]
pub unsafe extern "C" fn xmlGetProp(mut node: *const xmlNode,
                                    mut name: *const xmlChar)
 -> *mut xmlChar {
    let mut prop: xmlAttrPtr = 0 as *mut xmlAttr;
    prop = xmlHasProp(node, name);
    if prop.is_null() { return 0 as *mut xmlChar }
    return xmlGetPropNodeValueInternal(prop as *const xmlAttr);
}
/* *
 * xmlGetNoNsProp:
 * @node:  the node
 * @name:  the attribute name
 *
 * Search and get the value of an attribute associated to a node
 * This does the entity substitution.
 * This function looks in DTD attribute declaration for #FIXED or
 * default declaration values unless DTD use has been turned off.
 * This function is similar to xmlGetProp except it will accept only
 * an attribute in no namespace.
 *
 * Returns the attribute value or NULL if not found.
 *     It's up to the caller to free the memory with xmlFree().
 */
#[no_mangle]
pub unsafe extern "C" fn xmlGetNoNsProp(mut node: *const xmlNode,
                                        mut name: *const xmlChar)
 -> *mut xmlChar {
    let mut prop: xmlAttrPtr = 0 as *mut xmlAttr;
    prop =
        xmlGetPropNodeInternal(node, name, 0 as *const xmlChar, xmlCheckDTD);
    if prop.is_null() { return 0 as *mut xmlChar }
    return xmlGetPropNodeValueInternal(prop as *const xmlAttr);
}
/* *
 * xmlGetNsProp:
 * @node:  the node
 * @name:  the attribute name
 * @nameSpace:  the URI of the namespace
 *
 * Search and get the value of an attribute associated to a node
 * This attribute has to be anchored in the namespace specified.
 * This does the entity substitution.
 * This function looks in DTD attribute declaration for #FIXED or
 * default declaration values unless DTD use has been turned off.
 *
 * Returns the attribute value or NULL if not found.
 *     It's up to the caller to free the memory with xmlFree().
 */
#[no_mangle]
pub unsafe extern "C" fn xmlGetNsProp(mut node: *const xmlNode,
                                      mut name: *const xmlChar,
                                      mut nameSpace: *const xmlChar)
 -> *mut xmlChar {
    let mut prop: xmlAttrPtr = 0 as *mut xmlAttr;
    prop = xmlGetPropNodeInternal(node, name, nameSpace, xmlCheckDTD);
    if prop.is_null() { return 0 as *mut xmlChar }
    return xmlGetPropNodeValueInternal(prop as *const xmlAttr);
}
/* *
 * xmlUnsetProp:
 * @node:  the node
 * @name:  the attribute name
 *
 * Remove an attribute carried by a node.
 * This handles only attributes in no namespace.
 * Returns 0 if successful, -1 if not found
 */
#[no_mangle]
pub unsafe extern "C" fn xmlUnsetProp(mut node: xmlNodePtr,
                                      mut name: *const xmlChar)
 -> std::os::raw::c_int {
    let mut prop: xmlAttrPtr = 0 as *mut xmlAttr;
    prop =
        xmlGetPropNodeInternal(node as *const xmlNode, name,
                               0 as *const xmlChar, 0 as std::os::raw::c_int);
    if prop.is_null() { return -(1 as std::os::raw::c_int) }
    xmlUnlinkNode(prop as xmlNodePtr);
    xmlFreeProp(prop);
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlUnsetNsProp:
 * @node:  the node
 * @ns:  the namespace definition
 * @name:  the attribute name
 *
 * Remove an attribute carried by a node.
 * Returns 0 if successful, -1 if not found
 */
#[no_mangle]
pub unsafe extern "C" fn xmlUnsetNsProp(mut node: xmlNodePtr,
                                        mut ns: xmlNsPtr,
                                        mut name: *const xmlChar)
 -> std::os::raw::c_int {
    let mut prop: xmlAttrPtr = 0 as *mut xmlAttr;
    prop =
        xmlGetPropNodeInternal(node as *const xmlNode, name,
                               if !ns.is_null() {
                                   (*ns).href
                               } else { 0 as *const xmlChar },
                               0 as std::os::raw::c_int);
    if prop.is_null() { return -(1 as std::os::raw::c_int) }
    xmlUnlinkNode(prop as xmlNodePtr);
    xmlFreeProp(prop);
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlSetProp:
 * @node:  the node
 * @name:  the attribute name (a QName)
 * @value:  the attribute value
 *
 * Set (or reset) an attribute carried by a node.
 * If @name has a prefix, then the corresponding
 * namespace-binding will be used, if in scope; it is an
 * error it there's no such ns-binding for the prefix in
 * scope.
 * Returns the attribute pointer.
 *
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSetProp(mut node: xmlNodePtr,
                                    mut name: *const xmlChar,
                                    mut value: *const xmlChar) -> xmlAttrPtr {
    let mut len: std::os::raw::c_int = 0;
    let mut nqname: *const xmlChar = 0 as *const xmlChar;
    if node.is_null() || name.is_null() ||
           (*node).type_0 as std::os::raw::c_uint !=
               XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        return 0 as xmlAttrPtr
    }
    /*
     * handle QNames
     */
    nqname = xmlSplitQName3(name, &mut len);
    if !nqname.is_null() {
        let mut ns: xmlNsPtr = 0 as *mut xmlNs;
        let mut prefix: *mut xmlChar = xmlStrndup(name, len);
        ns = xmlSearchNs((*node).doc, node, prefix);
        if !prefix.is_null() {
            xmlFree.expect("non-null function pointer")(prefix as
                                                            *mut std::os::raw::c_void);
        }
        if !ns.is_null() { return xmlSetNsProp(node, ns, nqname, value) }
    }
    return xmlSetNsProp(node, 0 as xmlNsPtr, name, value);
}
/* *
 * xmlSetNsProp:
 * @node:  the node
 * @ns:  the namespace definition
 * @name:  the attribute name
 * @value:  the attribute value
 *
 * Set (or reset) an attribute carried by a node.
 * The ns structure must be in scope, this is not checked
 *
 * Returns the attribute pointer.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSetNsProp(mut node: xmlNodePtr, mut ns: xmlNsPtr,
                                      mut name: *const xmlChar,
                                      mut value: *const xmlChar)
 -> xmlAttrPtr {
    let mut prop: xmlAttrPtr = 0 as *mut xmlAttr;
    if !ns.is_null() && (*ns).href.is_null() { return 0 as xmlAttrPtr }
    prop =
        xmlGetPropNodeInternal(node as *const xmlNode, name,
                               if !ns.is_null() {
                                   (*ns).href
                               } else { 0 as *const xmlChar },
                               0 as std::os::raw::c_int);
    if !prop.is_null() {
        /*
	* Modify the attribute's value.
	*/
        if (*prop).atype as std::os::raw::c_uint ==
               XML_ATTRIBUTE_ID as std::os::raw::c_int as std::os::raw::c_uint {
            xmlRemoveID((*node).doc, prop);
            (*prop).atype = XML_ATTRIBUTE_ID
        }
        if !(*prop).children.is_null() { xmlFreeNodeList((*prop).children); }
        (*prop).children = 0 as *mut _xmlNode;
        (*prop).last = 0 as *mut _xmlNode;
        (*prop).ns = ns;
        if !value.is_null() {
            let mut tmp: xmlNodePtr = 0 as *mut xmlNode;
            if xmlCheckUTF8(value) == 0 {
                xmlTreeErr(XML_TREE_NOT_UTF8 as std::os::raw::c_int,
                           (*node).doc as xmlNodePtr,
                           0 as *const std::os::raw::c_char);
                if !(*node).doc.is_null() {
                    (*(*node).doc).encoding =
                        xmlStrdup(b"ISO-8859-1\x00" as *const u8 as
                                      *const std::os::raw::c_char as *mut xmlChar)
                }
            }
            (*prop).children = xmlNewDocText((*node).doc, value);
            (*prop).last = 0 as *mut _xmlNode;
            tmp = (*prop).children;
            while !tmp.is_null() {
                (*tmp).parent = prop as xmlNodePtr;
                if (*tmp).next.is_null() { (*prop).last = tmp }
                tmp = (*tmp).next
            }
        }
        if (*prop).atype as std::os::raw::c_uint ==
               XML_ATTRIBUTE_ID as std::os::raw::c_int as std::os::raw::c_uint {
            xmlAddID(0 as xmlValidCtxtPtr, (*node).doc, value, prop);
        }
        return prop
    }
    /*
    * No equal attr found; create a new one.
    */
    return xmlNewPropInternal(node, ns, name, value, 0 as std::os::raw::c_int);
}
/* LIBXML_TREE_ENABLED */
/* *
 * xmlNodeIsText:
 * @node:  the node
 *
 * Is this node a Text node ?
 * Returns 1 yes, 0 no
 */
#[no_mangle]
pub unsafe extern "C" fn xmlNodeIsText(mut node: *const xmlNode)
 -> std::os::raw::c_int {
    if node.is_null() { return 0 as std::os::raw::c_int }
    if (*node).type_0 as std::os::raw::c_uint ==
           XML_TEXT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        return 1 as std::os::raw::c_int
    }
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlIsBlankNode:
 * @node:  the node
 *
 * Checks whether this node is an empty or whitespace only
 * (and possibly ignorable) text-node.
 *
 * Returns 1 yes, 0 no
 */
#[no_mangle]
pub unsafe extern "C" fn xmlIsBlankNode(mut node: *const xmlNode)
 -> std::os::raw::c_int {
    let mut cur: *const xmlChar = 0 as *const xmlChar;
    if node.is_null() { return 0 as std::os::raw::c_int }
    if (*node).type_0 as std::os::raw::c_uint !=
           XML_TEXT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
           (*node).type_0 as std::os::raw::c_uint !=
               XML_CDATA_SECTION_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        return 0 as std::os::raw::c_int
    }
    if (*node).content.is_null() { return 1 as std::os::raw::c_int }
    cur = (*node).content;
    while *cur as std::os::raw::c_int != 0 as std::os::raw::c_int {
        if !(*cur as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
                 0x9 as std::os::raw::c_int <= *cur as std::os::raw::c_int &&
                     *cur as std::os::raw::c_int <= 0xa as std::os::raw::c_int ||
                 *cur as std::os::raw::c_int == 0xd as std::os::raw::c_int) {
            return 0 as std::os::raw::c_int
        }
        cur = cur.offset(1)
    }
    return 1 as std::os::raw::c_int;
}
/* *
 * xmlTextConcat:
 * @node:  the node
 * @content:  the content
 * @len:  @content length
 *
 * Concat the given string at the end of the existing node content
 *
 * Returns -1 in case of error, 0 otherwise
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextConcat(mut node: xmlNodePtr,
                                       mut content: *const xmlChar,
                                       mut len: std::os::raw::c_int) -> std::os::raw::c_int {
    if node.is_null() { return -(1 as std::os::raw::c_int) }
    if (*node).type_0 as std::os::raw::c_uint !=
           XML_TEXT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
           (*node).type_0 as std::os::raw::c_uint !=
               XML_CDATA_SECTION_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
           (*node).type_0 as std::os::raw::c_uint !=
               XML_COMMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
           (*node).type_0 as std::os::raw::c_uint !=
               XML_PI_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        return -(1 as std::os::raw::c_int)
    }
    /* need to check if content is currently in the dictionary */
    if (*node).content ==
           &mut (*node).properties as *mut *mut _xmlAttr as *mut xmlChar ||
           !(*node).doc.is_null() && !(*(*node).doc).dict.is_null() &&
               xmlDictOwns((*(*node).doc).dict, (*node).content) != 0 {
        (*node).content = xmlStrncatNew((*node).content, content, len)
    } else { (*node).content = xmlStrncat((*node).content, content, len) }
    (*node).properties = 0 as *mut _xmlAttr;
    if (*node).content.is_null() { return -(1 as std::os::raw::c_int) }
    return 0 as std::os::raw::c_int;
}
/* ***********************************************************************
 *									*
 *			Output : to a FILE or in memory			*
 *									*
 ************************************************************************/
/* *
 * xmlBufferCreate:
 *
 * routine to create an XML buffer.
 * returns the new structure.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlBufferCreate() -> xmlBufferPtr {
    let mut ret: xmlBufferPtr = 0 as *mut xmlBuffer;
    ret =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlBuffer>()
                                                          as std::os::raw::c_ulong) as
            xmlBufferPtr;
    if ret.is_null() {
        xmlTreeErrMemory(b"creating buffer\x00" as *const u8 as
                             *const std::os::raw::c_char);
        return 0 as xmlBufferPtr
    }
    (*ret).use_0 = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    (*ret).size = *__xmlDefaultBufferSize() as std::os::raw::c_uint;
    (*ret).alloc = *__xmlBufferAllocScheme();
    (*ret).content =
        xmlMallocAtomic.expect("non-null function pointer")(((*ret).size as
                                                                 std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<xmlChar>()
                                                                                                 as
                                                                                                 std::os::raw::c_ulong))
            as *mut xmlChar;
    if (*ret).content.is_null() {
        xmlTreeErrMemory(b"creating buffer\x00" as *const u8 as
                             *const std::os::raw::c_char);
        xmlFree.expect("non-null function pointer")(ret as *mut std::os::raw::c_void);
        return 0 as xmlBufferPtr
    }
    *(*ret).content.offset(0 as std::os::raw::c_int as isize) =
        0 as std::os::raw::c_int as xmlChar;
    (*ret).contentIO = 0 as *mut xmlChar;
    return ret;
}
/* *
 * xmlBufferCreateSize:
 * @size: initial size of buffer
 *
 * routine to create an XML buffer.
 * returns the new structure.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlBufferCreateSize(mut size: size_t)
 -> xmlBufferPtr {
    let mut ret: xmlBufferPtr = 0 as *mut xmlBuffer; /* +1 for ending null */
    ret =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlBuffer>()
                                                          as std::os::raw::c_ulong) as
            xmlBufferPtr;
    if ret.is_null() {
        xmlTreeErrMemory(b"creating buffer\x00" as *const u8 as
                             *const std::os::raw::c_char);
        return 0 as xmlBufferPtr
    }
    (*ret).use_0 = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    (*ret).alloc = *__xmlBufferAllocScheme();
    (*ret).size =
        if size != 0 {
            size.wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong)
        } else { 0 as std::os::raw::c_int as std::os::raw::c_ulong } as std::os::raw::c_uint;
    if (*ret).size != 0 {
        (*ret).content =
            xmlMallocAtomic.expect("non-null function pointer")(((*ret).size
                                                                     as
                                                                     std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<xmlChar>()
                                                                                                     as
                                                                                                     std::os::raw::c_ulong))
                as *mut xmlChar;
        if (*ret).content.is_null() {
            xmlTreeErrMemory(b"creating buffer\x00" as *const u8 as
                                 *const std::os::raw::c_char);
            xmlFree.expect("non-null function pointer")(ret as
                                                            *mut std::os::raw::c_void);
            return 0 as xmlBufferPtr
        }
        *(*ret).content.offset(0 as std::os::raw::c_int as isize) =
            0 as std::os::raw::c_int as xmlChar
    } else { (*ret).content = 0 as *mut xmlChar }
    (*ret).contentIO = 0 as *mut xmlChar;
    return ret;
}
/* *
 * xmlBufferDetach:
 * @buf:  the buffer
 *
 * Remove the string contained in a buffer and gie it back to the
 * caller. The buffer is reset to an empty content.
 * This doesn't work with immutable buffers as they can't be reset.
 *
 * Returns the previous string contained by the buffer.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlBufferDetach(mut buf: xmlBufferPtr)
 -> *mut xmlChar {
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    if buf.is_null() { return 0 as *mut xmlChar }
    if (*buf).alloc as std::os::raw::c_uint ==
           XML_BUFFER_ALLOC_IMMUTABLE as std::os::raw::c_int as std::os::raw::c_uint {
        return 0 as *mut xmlChar
    }
    ret = (*buf).content;
    (*buf).content = 0 as *mut xmlChar;
    (*buf).size = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    (*buf).use_0 = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    return ret;
}
/* *
 * xmlBufferCreateStatic:
 * @mem: the memory area
 * @size:  the size in byte
 *
 * routine to create an XML buffer from an immutable memory area.
 * The area won't be modified nor copied, and is expected to be
 * present until the end of the buffer lifetime.
 *
 * returns the new structure.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlBufferCreateStatic(mut mem: *mut std::os::raw::c_void,
                                               mut size: size_t)
 -> xmlBufferPtr {
    let mut ret: xmlBufferPtr = 0 as *mut xmlBuffer;
    if mem.is_null() || size == 0 as std::os::raw::c_int as std::os::raw::c_ulong {
        return 0 as xmlBufferPtr
    }
    ret =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlBuffer>()
                                                          as std::os::raw::c_ulong) as
            xmlBufferPtr;
    if ret.is_null() {
        xmlTreeErrMemory(b"creating buffer\x00" as *const u8 as
                             *const std::os::raw::c_char);
        return 0 as xmlBufferPtr
    }
    (*ret).use_0 = size as std::os::raw::c_uint;
    (*ret).size = size as std::os::raw::c_uint;
    (*ret).alloc = XML_BUFFER_ALLOC_IMMUTABLE;
    (*ret).content = mem as *mut xmlChar;
    return ret;
}
/* *
 * xmlBufferSetAllocationScheme:
 * @buf:  the buffer to tune
 * @scheme:  allocation scheme to use
 *
 * Sets the allocation scheme for this buffer
 */
#[no_mangle]
pub unsafe extern "C" fn xmlBufferSetAllocationScheme(mut buf: xmlBufferPtr,
                                                      mut scheme:
                                                          xmlBufferAllocationScheme) {
    if buf.is_null() { return }
    if (*buf).alloc as std::os::raw::c_uint ==
           XML_BUFFER_ALLOC_IMMUTABLE as std::os::raw::c_int as std::os::raw::c_uint ||
           (*buf).alloc as std::os::raw::c_uint ==
               XML_BUFFER_ALLOC_IO as std::os::raw::c_int as std::os::raw::c_uint {
        return
    }
    if scheme as std::os::raw::c_uint ==
           XML_BUFFER_ALLOC_DOUBLEIT as std::os::raw::c_int as std::os::raw::c_uint ||
           scheme as std::os::raw::c_uint ==
               XML_BUFFER_ALLOC_EXACT as std::os::raw::c_int as std::os::raw::c_uint ||
           scheme as std::os::raw::c_uint ==
               XML_BUFFER_ALLOC_HYBRID as std::os::raw::c_int as std::os::raw::c_uint ||
           scheme as std::os::raw::c_uint ==
               XML_BUFFER_ALLOC_IMMUTABLE as std::os::raw::c_int as std::os::raw::c_uint {
        (*buf).alloc = scheme
    };
}
/* *
 * xmlBufferFree:
 * @buf:  the buffer to free
 *
 * Frees an XML buffer. It frees both the content and the structure which
 * encapsulate it.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlBufferFree(mut buf: xmlBufferPtr) {
    if buf.is_null() { return }
    if (*buf).alloc as std::os::raw::c_uint ==
           XML_BUFFER_ALLOC_IO as std::os::raw::c_int as std::os::raw::c_uint &&
           !(*buf).contentIO.is_null() {
        xmlFree.expect("non-null function pointer")((*buf).contentIO as
                                                        *mut std::os::raw::c_void);
    } else if !(*buf).content.is_null() &&
                  (*buf).alloc as std::os::raw::c_uint !=
                      XML_BUFFER_ALLOC_IMMUTABLE as std::os::raw::c_int as
                          std::os::raw::c_uint {
        xmlFree.expect("non-null function pointer")((*buf).content as
                                                        *mut std::os::raw::c_void);
    }
    xmlFree.expect("non-null function pointer")(buf as *mut std::os::raw::c_void);
}
/* *
 * xmlBufferEmpty:
 * @buf:  the buffer
 *
 * empty a buffer.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlBufferEmpty(mut buf: xmlBufferPtr) {
    if buf.is_null() { return }
    if (*buf).content.is_null() { return }
    (*buf).use_0 = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    if (*buf).alloc as std::os::raw::c_uint ==
           XML_BUFFER_ALLOC_IMMUTABLE as std::os::raw::c_int as std::os::raw::c_uint {
        (*buf).content =
            b"\x00" as *const u8 as *const std::os::raw::c_char as *mut xmlChar
    } else if (*buf).alloc as std::os::raw::c_uint ==
                  XML_BUFFER_ALLOC_IO as std::os::raw::c_int as std::os::raw::c_uint &&
                  !(*buf).contentIO.is_null() {
        let mut start_buf: size_t =
            (*buf).content.offset_from((*buf).contentIO) as
                std::os::raw::c_long as size_t;
        (*buf).size =
            ((*buf).size as std::os::raw::c_ulong).wrapping_add(start_buf) as
                std::os::raw::c_uint as std::os::raw::c_uint;
        (*buf).content = (*buf).contentIO;
        *(*buf).content.offset(0 as std::os::raw::c_int as isize) =
            0 as std::os::raw::c_int as xmlChar
    } else {
        *(*buf).content.offset(0 as std::os::raw::c_int as isize) =
            0 as std::os::raw::c_int as xmlChar
    };
}
/* *
 * xmlBufferShrink:
 * @buf:  the buffer to dump
 * @len:  the number of xmlChar to remove
 *
 * Remove the beginning of an XML buffer.
 *
 * Returns the number of #xmlChar removed, or -1 in case of failure.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlBufferShrink(mut buf: xmlBufferPtr,
                                         mut len: std::os::raw::c_uint)
 -> std::os::raw::c_int {
    if buf.is_null() { return -(1 as std::os::raw::c_int) }
    if len == 0 as std::os::raw::c_int as std::os::raw::c_uint { return 0 as std::os::raw::c_int }
    if len > (*buf).use_0 { return -(1 as std::os::raw::c_int) }
    (*buf).use_0 = (*buf).use_0.wrapping_sub(len);
    if (*buf).alloc as std::os::raw::c_uint ==
           XML_BUFFER_ALLOC_IMMUTABLE as std::os::raw::c_int as std::os::raw::c_uint ||
           (*buf).alloc as std::os::raw::c_uint ==
               XML_BUFFER_ALLOC_IO as std::os::raw::c_int as std::os::raw::c_uint &&
               !(*buf).contentIO.is_null() {
        /*
	 * we just move the content pointer, but also make sure
	 * the perceived buffer size has shrinked accordingly
	 */
        (*buf).content = (*buf).content.offset(len as isize);
        (*buf).size = (*buf).size.wrapping_sub(len);
        /*
	 * sometimes though it maybe be better to really shrink
	 * on IO buffers
	 */
        if (*buf).alloc as std::os::raw::c_uint ==
               XML_BUFFER_ALLOC_IO as std::os::raw::c_int as std::os::raw::c_uint &&
               !(*buf).contentIO.is_null() {
            let mut start_buf: size_t =
                (*buf).content.offset_from((*buf).contentIO) as
                    std::os::raw::c_long as size_t;
            if start_buf >= (*buf).size as std::os::raw::c_ulong {
                memmove((*buf).contentIO as *mut std::os::raw::c_void,
                        &mut *(*buf).content.offset(0 as std::os::raw::c_int as isize)
                            as *mut xmlChar as *const std::os::raw::c_void,
                        (*buf).use_0 as std::os::raw::c_ulong);
                (*buf).content = (*buf).contentIO;
                *(*buf).content.offset((*buf).use_0 as isize) =
                    0 as std::os::raw::c_int as xmlChar;
                (*buf).size =
                    ((*buf).size as std::os::raw::c_ulong).wrapping_add(start_buf) as
                        std::os::raw::c_uint as std::os::raw::c_uint
            }
        }
    } else {
        memmove((*buf).content as *mut std::os::raw::c_void,
                &mut *(*buf).content.offset(len as isize) as *mut xmlChar as
                    *const std::os::raw::c_void, (*buf).use_0 as std::os::raw::c_ulong);
        *(*buf).content.offset((*buf).use_0 as isize) =
            0 as std::os::raw::c_int as xmlChar
    }
    return len as std::os::raw::c_int;
}
/* *
 * xmlBufferGrow:
 * @buf:  the buffer
 * @len:  the minimum free size to allocate
 *
 * Grow the available space of an XML buffer.
 *
 * Returns the new available space or -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlBufferGrow(mut buf: xmlBufferPtr,
                                       mut len: std::os::raw::c_uint) -> std::os::raw::c_int {
    let mut size: std::os::raw::c_int = 0;
    let mut newbuf: *mut xmlChar = 0 as *mut xmlChar;
    if buf.is_null() { return -(1 as std::os::raw::c_int) }
    if (*buf).alloc as std::os::raw::c_uint ==
           XML_BUFFER_ALLOC_IMMUTABLE as std::os::raw::c_int as std::os::raw::c_uint {
        return 0 as std::os::raw::c_int
    }
    if len.wrapping_add((*buf).use_0) < (*buf).size {
        return 0 as std::os::raw::c_int
    }
    /*
     * Windows has a BIG problem on realloc timing, so we try to double
     * the buffer size (if that's enough) (bug 146697)
     * Apparently BSD too, and it's probably best for linux too
     * On an embedded system this may be something to change
     */
    if (*buf).size > len {
        size =
            (*buf).size.wrapping_mul(2 as std::os::raw::c_int as std::os::raw::c_uint) as
                std::os::raw::c_int
    } else {
        size =
            (*buf).use_0.wrapping_add(len).wrapping_add(100 as std::os::raw::c_int as
                                                            std::os::raw::c_uint) as
                std::os::raw::c_int
    }
    if (*buf).alloc as std::os::raw::c_uint ==
           XML_BUFFER_ALLOC_IO as std::os::raw::c_int as std::os::raw::c_uint &&
           !(*buf).contentIO.is_null() {
        let mut start_buf: size_t =
            (*buf).content.offset_from((*buf).contentIO) as
                std::os::raw::c_long as size_t;
        newbuf =
            xmlRealloc.expect("non-null function pointer")((*buf).contentIO as
                                                               *mut std::os::raw::c_void,
                                                           start_buf.wrapping_add(size
                                                                                      as
                                                                                      std::os::raw::c_ulong))
                as *mut xmlChar;
        if newbuf.is_null() {
            xmlTreeErrMemory(b"growing buffer\x00" as *const u8 as
                                 *const std::os::raw::c_char);
            return -(1 as std::os::raw::c_int)
        }
        (*buf).contentIO = newbuf;
        (*buf).content = newbuf.offset(start_buf as isize)
    } else {
        newbuf =
            xmlRealloc.expect("non-null function pointer")((*buf).content as
                                                               *mut std::os::raw::c_void,
                                                           size as size_t) as
                *mut xmlChar;
        if newbuf.is_null() {
            xmlTreeErrMemory(b"growing buffer\x00" as *const u8 as
                                 *const std::os::raw::c_char);
            return -(1 as std::os::raw::c_int)
        }
        (*buf).content = newbuf
    }
    (*buf).size = size as std::os::raw::c_uint;
    return (*buf).size.wrapping_sub((*buf).use_0) as std::os::raw::c_int;
}
/* *
 * xmlBufferDump:
 * @file:  the file output
 * @buf:  the buffer to dump
 *
 * Dumps an XML buffer to  a FILE *.
 * Returns the number of #xmlChar written
 */
#[no_mangle]
pub unsafe extern "C" fn xmlBufferDump(mut file: *mut FILE,
                                       mut buf: xmlBufferPtr) -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0;
    if buf.is_null() { return 0 as std::os::raw::c_int }
    if (*buf).content.is_null() { return 0 as std::os::raw::c_int }
    if file.is_null() { file = stdout }
    ret =
        fwrite((*buf).content as *const std::os::raw::c_void,
               ::std::mem::size_of::<xmlChar>() as std::os::raw::c_ulong,
               (*buf).use_0 as size_t, file) as std::os::raw::c_int;
    return ret;
}
/* *
 * xmlBufferContent:
 * @buf:  the buffer
 *
 * Function to extract the content of a buffer
 *
 * Returns the internal content
 */
#[no_mangle]
pub unsafe extern "C" fn xmlBufferContent(mut buf: *const xmlBuffer)
 -> *const xmlChar {
    if buf.is_null() { return 0 as *const xmlChar }
    return (*buf).content;
}
/* *
 * xmlBufferLength:
 * @buf:  the buffer
 *
 * Function to get the length of a buffer
 *
 * Returns the length of data in the internal content
 */
#[no_mangle]
pub unsafe extern "C" fn xmlBufferLength(mut buf: *const xmlBuffer)
 -> std::os::raw::c_int {
    if buf.is_null() { return 0 as std::os::raw::c_int }
    return (*buf).use_0 as std::os::raw::c_int;
}
/* *
 * xmlBufferResize:
 * @buf:  the buffer to resize
 * @size:  the desired size
 *
 * Resize a buffer to accommodate minimum size of @size.
 *
 * Returns  0 in case of problems, 1 otherwise
 */
#[no_mangle]
pub unsafe extern "C" fn xmlBufferResize(mut buf: xmlBufferPtr,
                                         mut size: std::os::raw::c_uint)
 -> std::os::raw::c_int {
    let mut newSize: std::os::raw::c_uint = 0;
    let mut rebuf: *mut xmlChar = 0 as *mut xmlChar;
    let mut start_buf: size_t = 0;
    if buf.is_null() { return 0 as std::os::raw::c_int }
    if (*buf).alloc as std::os::raw::c_uint ==
           XML_BUFFER_ALLOC_IMMUTABLE as std::os::raw::c_int as std::os::raw::c_uint {
        return 0 as std::os::raw::c_int
    }
    /* Don't resize if we don't have to */
    if size < (*buf).size { return 1 as std::os::raw::c_int }
    /* figure out new size */
    match (*buf).alloc as std::os::raw::c_uint {
        3 | 0 => {
            /*take care of empty case*/
            newSize =
                if (*buf).size != 0 {
                    (*buf).size.wrapping_mul(2 as std::os::raw::c_int as std::os::raw::c_uint)
                } else {
                    size.wrapping_add(10 as std::os::raw::c_int as std::os::raw::c_uint)
                };
            while size > newSize {
                if newSize >
                       (2147483647 as std::os::raw::c_int as
                            std::os::raw::c_uint).wrapping_mul(2 as
                                                           std::os::raw::c_uint).wrapping_add(1
                                                                                          as
                                                                                          std::os::raw::c_uint).wrapping_div(2
                                                                                                                         as
                                                                                                                         std::os::raw::c_int
                                                                                                                         as
                                                                                                                         std::os::raw::c_uint)
                   {
                    xmlTreeErrMemory(b"growing buffer\x00" as *const u8 as
                                         *const std::os::raw::c_char);
                    return 0 as std::os::raw::c_int
                }
                newSize =
                    newSize.wrapping_mul(2 as std::os::raw::c_int as std::os::raw::c_uint)
            }
        }
        1 => {
            newSize = size.wrapping_add(10 as std::os::raw::c_int as std::os::raw::c_uint)
        }
        4 => {
            if (*buf).use_0 < 4096 as std::os::raw::c_int as std::os::raw::c_uint {
                newSize = size
            } else {
                newSize =
                    (*buf).size.wrapping_mul(2 as std::os::raw::c_int as
                                                 std::os::raw::c_uint);
                while size > newSize {
                    if newSize >
                           (2147483647 as std::os::raw::c_int as
                                std::os::raw::c_uint).wrapping_mul(2 as
                                                               std::os::raw::c_uint).wrapping_add(1
                                                                                              as
                                                                                              std::os::raw::c_uint).wrapping_div(2
                                                                                                                             as
                                                                                                                             std::os::raw::c_int
                                                                                                                             as
                                                                                                                             std::os::raw::c_uint)
                       {
                        xmlTreeErrMemory(b"growing buffer\x00" as *const u8 as
                                             *const std::os::raw::c_char);
                        return 0 as std::os::raw::c_int
                    }
                    newSize =
                        newSize.wrapping_mul(2 as std::os::raw::c_int as std::os::raw::c_uint)
                }
            }
        }
        _ => {
            newSize = size.wrapping_add(10 as std::os::raw::c_int as std::os::raw::c_uint)
        }
    }
    if (*buf).alloc as std::os::raw::c_uint ==
           XML_BUFFER_ALLOC_IO as std::os::raw::c_int as std::os::raw::c_uint &&
           !(*buf).contentIO.is_null() {
        start_buf =
            (*buf).content.offset_from((*buf).contentIO) as
                std::os::raw::c_long as size_t;
        if start_buf > newSize as std::os::raw::c_ulong {
            /* move data back to start */
            memmove((*buf).contentIO as *mut std::os::raw::c_void,
                    (*buf).content as *const std::os::raw::c_void,
                    (*buf).use_0 as std::os::raw::c_ulong);
            (*buf).content = (*buf).contentIO;
            *(*buf).content.offset((*buf).use_0 as isize) =
                0 as std::os::raw::c_int as xmlChar;
            (*buf).size =
                ((*buf).size as std::os::raw::c_ulong).wrapping_add(start_buf) as
                    std::os::raw::c_uint as std::os::raw::c_uint
        } else {
            rebuf =
                xmlRealloc.expect("non-null function pointer")((*buf).contentIO
                                                                   as
                                                                   *mut std::os::raw::c_void,
                                                               start_buf.wrapping_add(newSize
                                                                                          as
                                                                                          std::os::raw::c_ulong))
                    as *mut xmlChar;
            if rebuf.is_null() {
                xmlTreeErrMemory(b"growing buffer\x00" as *const u8 as
                                     *const std::os::raw::c_char);
                return 0 as std::os::raw::c_int
            }
            (*buf).contentIO = rebuf;
            (*buf).content = rebuf.offset(start_buf as isize)
        }
    } else {
        if (*buf).content.is_null() {
            rebuf =
                xmlMallocAtomic.expect("non-null function pointer")(newSize as
                                                                        size_t)
                    as *mut xmlChar
        } else if (*buf).size.wrapping_sub((*buf).use_0) <
                      100 as std::os::raw::c_int as std::os::raw::c_uint {
            rebuf =
                xmlRealloc.expect("non-null function pointer")((*buf).content
                                                                   as
                                                                   *mut std::os::raw::c_void,
                                                               newSize as
                                                                   size_t) as
                    *mut xmlChar
        } else {
            /*
	     * if we are reallocating a buffer far from being full, it's
	     * better to make a new allocation and copy only the used range
	     * and free the old one.
	     */
            rebuf =
                xmlMallocAtomic.expect("non-null function pointer")(newSize as
                                                                        size_t)
                    as *mut xmlChar;
            if !rebuf.is_null() {
                memcpy(rebuf as *mut std::os::raw::c_void,
                       (*buf).content as *const std::os::raw::c_void,
                       (*buf).use_0 as std::os::raw::c_ulong);
                xmlFree.expect("non-null function pointer")((*buf).content as
                                                                *mut std::os::raw::c_void);
                *rebuf.offset((*buf).use_0 as isize) =
                    0 as std::os::raw::c_int as xmlChar
            }
        }
        if rebuf.is_null() {
            xmlTreeErrMemory(b"growing buffer\x00" as *const u8 as
                                 *const std::os::raw::c_char);
            return 0 as std::os::raw::c_int
        }
        (*buf).content = rebuf
    }
    (*buf).size = newSize;
    return 1 as std::os::raw::c_int;
}
/* *
 * xmlBufferAdd:
 * @buf:  the buffer to dump
 * @str:  the #xmlChar string
 * @len:  the number of #xmlChar to add
 *
 * Add a string range to an XML buffer. if len == -1, the length of
 * str is recomputed.
 *
 * Returns 0 successful, a positive error code number otherwise
 *         and -1 in case of internal or API error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlBufferAdd(mut buf: xmlBufferPtr,
                                      mut str: *const xmlChar,
                                      mut len: std::os::raw::c_int) -> std::os::raw::c_int {
    let mut needSize: std::os::raw::c_uint = 0;
    if str.is_null() || buf.is_null() { return -(1 as std::os::raw::c_int) }
    if (*buf).alloc as std::os::raw::c_uint ==
           XML_BUFFER_ALLOC_IMMUTABLE as std::os::raw::c_int as std::os::raw::c_uint {
        return -(1 as std::os::raw::c_int)
    }
    if len < -(1 as std::os::raw::c_int) { return -(1 as std::os::raw::c_int) }
    if len == 0 as std::os::raw::c_int { return 0 as std::os::raw::c_int }
    if len < 0 as std::os::raw::c_int { len = xmlStrlen(str) }
    if len < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
    if len == 0 as std::os::raw::c_int { return 0 as std::os::raw::c_int }
    needSize =
        (*buf).use_0.wrapping_add(len as
                                      std::os::raw::c_uint).wrapping_add(2 as
                                                                     std::os::raw::c_int
                                                                     as
                                                                     std::os::raw::c_uint);
    if needSize > (*buf).size {
        if xmlBufferResize(buf, needSize) == 0 {
            xmlTreeErrMemory(b"growing buffer\x00" as *const u8 as
                                 *const std::os::raw::c_char);
            return XML_ERR_NO_MEMORY as std::os::raw::c_int
        }
    }
    memmove(&mut *(*buf).content.offset((*buf).use_0 as isize) as *mut xmlChar
                as *mut std::os::raw::c_void, str as *const std::os::raw::c_void,
            (len as
                 std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<xmlChar>()
                                                 as std::os::raw::c_ulong));
    (*buf).use_0 = (*buf).use_0.wrapping_add(len as std::os::raw::c_uint);
    *(*buf).content.offset((*buf).use_0 as isize) =
        0 as std::os::raw::c_int as xmlChar;
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlBufferAddHead:
 * @buf:  the buffer
 * @str:  the #xmlChar string
 * @len:  the number of #xmlChar to add
 *
 * Add a string range to the beginning of an XML buffer.
 * if len == -1, the length of @str is recomputed.
 *
 * Returns 0 successful, a positive error code number otherwise
 *         and -1 in case of internal or API error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlBufferAddHead(mut buf: xmlBufferPtr,
                                          mut str: *const xmlChar,
                                          mut len: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut needSize: std::os::raw::c_uint = 0;
    if buf.is_null() { return -(1 as std::os::raw::c_int) }
    if (*buf).alloc as std::os::raw::c_uint ==
           XML_BUFFER_ALLOC_IMMUTABLE as std::os::raw::c_int as std::os::raw::c_uint {
        return -(1 as std::os::raw::c_int)
    }
    if str.is_null() { return -(1 as std::os::raw::c_int) }
    if len < -(1 as std::os::raw::c_int) { return -(1 as std::os::raw::c_int) }
    if len == 0 as std::os::raw::c_int { return 0 as std::os::raw::c_int }
    if len < 0 as std::os::raw::c_int { len = xmlStrlen(str) }
    if len <= 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
    if (*buf).alloc as std::os::raw::c_uint ==
           XML_BUFFER_ALLOC_IO as std::os::raw::c_int as std::os::raw::c_uint &&
           !(*buf).contentIO.is_null() {
        let mut start_buf: size_t =
            (*buf).content.offset_from((*buf).contentIO) as
                std::os::raw::c_long as size_t;
        if start_buf > len as std::os::raw::c_uint as std::os::raw::c_ulong {
            /*
	     * We can add it in the space previously shrinked
	     */
            (*buf).content = (*buf).content.offset(-(len as isize));
            memmove(&mut *(*buf).content.offset(0 as std::os::raw::c_int as isize) as
                        *mut xmlChar as *mut std::os::raw::c_void,
                    str as *const std::os::raw::c_void, len as std::os::raw::c_ulong);
            (*buf).use_0 = (*buf).use_0.wrapping_add(len as std::os::raw::c_uint);
            (*buf).size = (*buf).size.wrapping_add(len as std::os::raw::c_uint);
            return 0 as std::os::raw::c_int
        }
    }
    needSize =
        (*buf).use_0.wrapping_add(len as
                                      std::os::raw::c_uint).wrapping_add(2 as
                                                                     std::os::raw::c_int
                                                                     as
                                                                     std::os::raw::c_uint);
    if needSize > (*buf).size {
        if xmlBufferResize(buf, needSize) == 0 {
            xmlTreeErrMemory(b"growing buffer\x00" as *const u8 as
                                 *const std::os::raw::c_char);
            return XML_ERR_NO_MEMORY as std::os::raw::c_int
        }
    }
    memmove(&mut *(*buf).content.offset(len as isize) as *mut xmlChar as
                *mut std::os::raw::c_void,
            &mut *(*buf).content.offset(0 as std::os::raw::c_int as isize) as
                *mut xmlChar as *const std::os::raw::c_void,
            (*buf).use_0 as std::os::raw::c_ulong);
    memmove(&mut *(*buf).content.offset(0 as std::os::raw::c_int as isize) as
                *mut xmlChar as *mut std::os::raw::c_void, str as *const std::os::raw::c_void,
            len as std::os::raw::c_ulong);
    (*buf).use_0 = (*buf).use_0.wrapping_add(len as std::os::raw::c_uint);
    *(*buf).content.offset((*buf).use_0 as isize) =
        0 as std::os::raw::c_int as xmlChar;
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlBufferCat:
 * @buf:  the buffer to add to
 * @str:  the #xmlChar string
 *
 * Append a zero terminated string to an XML buffer.
 *
 * Returns 0 successful, a positive error code number otherwise
 *         and -1 in case of internal or API error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlBufferCat(mut buf: xmlBufferPtr,
                                      mut str: *const xmlChar)
 -> std::os::raw::c_int {
    if buf.is_null() { return -(1 as std::os::raw::c_int) }
    if (*buf).alloc as std::os::raw::c_uint ==
           XML_BUFFER_ALLOC_IMMUTABLE as std::os::raw::c_int as std::os::raw::c_uint {
        return -(1 as std::os::raw::c_int)
    }
    if str.is_null() { return -(1 as std::os::raw::c_int) }
    return xmlBufferAdd(buf, str, -(1 as std::os::raw::c_int));
}
/* *
 * xmlBufferCCat:
 * @buf:  the buffer to dump
 * @str:  the C char string
 *
 * Append a zero terminated C string to an XML buffer.
 *
 * Returns 0 successful, a positive error code number otherwise
 *         and -1 in case of internal or API error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlBufferCCat(mut buf: xmlBufferPtr,
                                       mut str: *const std::os::raw::c_char)
 -> std::os::raw::c_int {
    let mut cur: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    if buf.is_null() { return -(1 as std::os::raw::c_int) }
    if (*buf).alloc as std::os::raw::c_uint ==
           XML_BUFFER_ALLOC_IMMUTABLE as std::os::raw::c_int as std::os::raw::c_uint {
        return -(1 as std::os::raw::c_int)
    }
    if str.is_null() { return -(1 as std::os::raw::c_int) }
    cur = str;
    while *cur as std::os::raw::c_int != 0 as std::os::raw::c_int {
        if (*buf).use_0.wrapping_add(10 as std::os::raw::c_int as std::os::raw::c_uint) >=
               (*buf).size {
            if xmlBufferResize(buf,
                               (*buf).use_0.wrapping_add(10 as std::os::raw::c_int as
                                                             std::os::raw::c_uint)) ==
                   0 {
                xmlTreeErrMemory(b"growing buffer\x00" as *const u8 as
                                     *const std::os::raw::c_char);
                return XML_ERR_NO_MEMORY as std::os::raw::c_int
            }
        }
        let fresh13 = (*buf).use_0;
        (*buf).use_0 = (*buf).use_0.wrapping_add(1);
        *(*buf).content.offset(fresh13 as isize) = *cur as xmlChar;
        cur = cur.offset(1)
    }
    *(*buf).content.offset((*buf).use_0 as isize) =
        0 as std::os::raw::c_int as xmlChar;
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlBufferWriteCHAR:
 * @buf:  the XML buffer
 * @string:  the string to add
 *
 * routine which manages and grows an output buffer. This one adds
 * xmlChars at the end of the buffer.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlBufferWriteCHAR(mut buf: xmlBufferPtr,
                                            mut string: *const xmlChar) {
    if buf.is_null() { return }
    if (*buf).alloc as std::os::raw::c_uint ==
           XML_BUFFER_ALLOC_IMMUTABLE as std::os::raw::c_int as std::os::raw::c_uint {
        return
    }
    xmlBufferCat(buf, string);
}
/* *
 * xmlBufferWriteChar:
 * @buf:  the XML buffer output
 * @string:  the string to add
 *
 * routine which manage and grows an output buffer. This one add
 * C chars at the end of the array.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlBufferWriteChar(mut buf: xmlBufferPtr,
                                            mut string: *const std::os::raw::c_char) {
    if buf.is_null() { return }
    if (*buf).alloc as std::os::raw::c_uint ==
           XML_BUFFER_ALLOC_IMMUTABLE as std::os::raw::c_int as std::os::raw::c_uint {
        return
    }
    xmlBufferCCat(buf, string);
}
/* *
 * xmlBufferWriteQuotedString:
 * @buf:  the XML buffer output
 * @string:  the string to add
 *
 * routine which manage and grows an output buffer. This one writes
 * a quoted or double quoted #xmlChar string, checking first if it holds
 * quote or double-quotes internally
 */
#[no_mangle]
pub unsafe extern "C" fn xmlBufferWriteQuotedString(mut buf: xmlBufferPtr,
                                                    mut string:
                                                        *const xmlChar) {
    let mut cur: *const xmlChar = 0 as *const xmlChar;
    let mut base: *const xmlChar = 0 as *const xmlChar;
    if buf.is_null() { return }
    if (*buf).alloc as std::os::raw::c_uint ==
           XML_BUFFER_ALLOC_IMMUTABLE as std::os::raw::c_int as std::os::raw::c_uint {
        return
    }
    if !xmlStrchr(string, '\"' as i32 as xmlChar).is_null() {
        if !xmlStrchr(string, '\'' as i32 as xmlChar).is_null() {
            xmlBufferCCat(buf, b"\"\x00" as *const u8 as *const std::os::raw::c_char);
            cur = string;
            base = cur;
            while *cur as std::os::raw::c_int != 0 as std::os::raw::c_int {
                if *cur as std::os::raw::c_int == '\"' as i32 {
                    if base != cur {
                        xmlBufferAdd(buf, base,
                                     cur.offset_from(base) as
                                         std::os::raw::c_long as std::os::raw::c_int);
                    }
                    xmlBufferAdd(buf,
                                 b"&quot;\x00" as *const u8 as
                                     *const std::os::raw::c_char as *mut xmlChar,
                                 6 as std::os::raw::c_int);
                    cur = cur.offset(1);
                    base = cur
                } else { cur = cur.offset(1) }
            }
            if base != cur {
                xmlBufferAdd(buf, base,
                             cur.offset_from(base) as std::os::raw::c_long as
                                 std::os::raw::c_int);
            }
            xmlBufferCCat(buf, b"\"\x00" as *const u8 as *const std::os::raw::c_char);
        } else {
            xmlBufferCCat(buf, b"\'\x00" as *const u8 as *const std::os::raw::c_char);
            xmlBufferCat(buf, string);
            xmlBufferCCat(buf, b"\'\x00" as *const u8 as *const std::os::raw::c_char);
        }
    } else {
        xmlBufferCCat(buf, b"\"\x00" as *const u8 as *const std::os::raw::c_char);
        xmlBufferCat(buf, string);
        xmlBufferCCat(buf, b"\"\x00" as *const u8 as *const std::os::raw::c_char);
    };
}
/* *
 * xmlGetDocCompressMode:
 * @doc:  the document
 *
 * get the compression ratio for a document, ZLIB based
 * Returns 0 (uncompressed) to 9 (max compression)
 */
#[no_mangle]
pub unsafe extern "C" fn xmlGetDocCompressMode(mut doc: *const xmlDoc)
 -> std::os::raw::c_int {
    if doc.is_null() { return -(1 as std::os::raw::c_int) }
    return (*doc).compression;
}
/* *
 * xmlSetDocCompressMode:
 * @doc:  the document
 * @mode:  the compression ratio
 *
 * set the compression ratio for a document, ZLIB based
 * Correct values: 0 (uncompressed) to 9 (max compression)
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSetDocCompressMode(mut doc: xmlDocPtr,
                                               mut mode: std::os::raw::c_int) {
    if doc.is_null() { return }
    if mode < 0 as std::os::raw::c_int {
        (*doc).compression = 0 as std::os::raw::c_int
    } else if mode > 9 as std::os::raw::c_int {
        (*doc).compression = 9 as std::os::raw::c_int
    } else { (*doc).compression = mode };
}
/* *
 * xmlGetCompressMode:
 *
 * get the default compression mode used, ZLIB based.
 * Returns 0 (uncompressed) to 9 (max compression)
 */
#[no_mangle]
pub unsafe extern "C" fn xmlGetCompressMode() -> std::os::raw::c_int {
    return xmlCompressMode;
}
/* *
 * xmlSetCompressMode:
 * @mode:  the compression ratio
 *
 * set the default compression mode used, ZLIB based
 * Correct values: 0 (uncompressed) to 9 (max compression)
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSetCompressMode(mut mode: std::os::raw::c_int) {
    if mode < 0 as std::os::raw::c_int {
        xmlCompressMode = 0 as std::os::raw::c_int
    } else if mode > 9 as std::os::raw::c_int {
        xmlCompressMode = 9 as std::os::raw::c_int
    } else { xmlCompressMode = mode };
}
/*
* xmlDOMWrapNsMapFree:
* @map: the ns-map
*
* Frees the ns-map
*/
unsafe extern "C" fn xmlDOMWrapNsMapFree(mut nsmap: xmlNsMapPtr) {
    let mut cur: xmlNsMapItemPtr = 0 as *mut xmlNsMapItem;
    let mut tmp: xmlNsMapItemPtr = 0 as *mut xmlNsMapItem;
    if nsmap.is_null() { return }
    cur = (*nsmap).pool;
    while !cur.is_null() {
        tmp = cur;
        cur = (*cur).next;
        xmlFree.expect("non-null function pointer")(tmp as *mut std::os::raw::c_void);
    }
    cur = (*nsmap).first;
    while !cur.is_null() {
        tmp = cur;
        cur = (*cur).next;
        xmlFree.expect("non-null function pointer")(tmp as *mut std::os::raw::c_void);
    }
    xmlFree.expect("non-null function pointer")(nsmap as *mut std::os::raw::c_void);
}
/*
* xmlDOMWrapNsMapAddItem:
* @map: the ns-map
* @oldNs: the old ns-struct
* @newNs: the new ns-struct
* @depth: depth and ns-kind information
*
* Adds an ns-mapping item.
*/
unsafe extern "C" fn xmlDOMWrapNsMapAddItem(mut nsmap: *mut xmlNsMapPtr,
                                            mut position: std::os::raw::c_int,
                                            mut oldNs: xmlNsPtr,
                                            mut newNs: xmlNsPtr,
                                            mut depth: std::os::raw::c_int)
 -> xmlNsMapItemPtr {
    let mut ret: xmlNsMapItemPtr = 0 as *mut xmlNsMapItem;
    let mut map: xmlNsMapPtr = 0 as *mut xmlNsMap;
    if nsmap.is_null() { return 0 as xmlNsMapItemPtr }
    if position != -(1 as std::os::raw::c_int) && position != 0 as std::os::raw::c_int {
        return 0 as xmlNsMapItemPtr
    }
    map = *nsmap;
    if map.is_null() {
        /*
	* Create the ns-map.
	*/
        map =
            xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlNsMap>()
                                                              as
                                                              std::os::raw::c_ulong)
                as xmlNsMapPtr;
        if map.is_null() {
            xmlTreeErrMemory(b"allocating namespace map\x00" as *const u8 as
                                 *const std::os::raw::c_char);
            return 0 as xmlNsMapItemPtr
        }
        memset(map as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
               ::std::mem::size_of::<xmlNsMap>() as std::os::raw::c_ulong);
        *nsmap = map
    }
    if !(*map).pool.is_null() {
        /*
	* Reuse an item from the pool.
	*/
        ret = (*map).pool;
        (*map).pool = (*ret).next;
        memset(ret as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
               ::std::mem::size_of::<xmlNsMapItem>() as std::os::raw::c_ulong);
    } else {
        /*
	* Create a new item.
	*/
        ret =
            xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlNsMapItem>()
                                                              as
                                                              std::os::raw::c_ulong)
                as xmlNsMapItemPtr;
        if ret.is_null() {
            xmlTreeErrMemory(b"allocating namespace map item\x00" as *const u8
                                 as *const std::os::raw::c_char);
            return 0 as xmlNsMapItemPtr
        }
        memset(ret as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
               ::std::mem::size_of::<xmlNsMapItem>() as std::os::raw::c_ulong);
    }
    if (*map).first.is_null() {
        /*
	* First ever.
	*/
        (*map).first = ret;
        (*map).last = ret
    } else if position == -(1 as std::os::raw::c_int) {
        /*
	* Append.
	*/
        (*ret).prev = (*map).last;
        (*(*map).last).next = ret;
        (*map).last = ret
    } else if position == 0 as std::os::raw::c_int {
        /*
	* Set on first position.
	*/
        (*(*map).first).prev = ret;
        (*ret).next = (*map).first;
        (*map).first = ret
    }
    (*ret).oldNs = oldNs;
    (*ret).newNs = newNs;
    (*ret).shadowDepth = -(1 as std::os::raw::c_int);
    (*ret).depth = depth;
    return ret;
}
/*
* xmlDOMWrapStoreNs:
* @doc: the doc
* @nsName: the namespace name
* @prefix: the prefix
*
* Creates or reuses an xmlNs struct on doc->oldNs with
* the given prefix and namespace name.
*
* Returns the aquired ns struct or NULL in case of an API
*         or internal error.
*/
unsafe extern "C" fn xmlDOMWrapStoreNs(mut doc: xmlDocPtr,
                                       mut nsName: *const xmlChar,
                                       mut prefix: *const xmlChar)
 -> xmlNsPtr {
    let mut ns: xmlNsPtr = 0 as *mut xmlNs;
    if doc.is_null() { return 0 as xmlNsPtr }
    ns = xmlTreeEnsureXMLDecl(doc);
    if ns.is_null() { return 0 as xmlNsPtr }
    if !(*ns).next.is_null() {
        /* Reuse. */
        ns = (*ns).next;
        while !ns.is_null() {
            if ((*ns).prefix == prefix ||
                    xmlStrEqual((*ns).prefix, prefix) != 0) &&
                   xmlStrEqual((*ns).href, nsName) != 0 {
                return ns
            }
            if (*ns).next.is_null() { break ; }
            ns = (*ns).next
        }
    }
    /* Create. */
    if !ns.is_null() {
        (*ns).next = xmlNewNs(0 as xmlNodePtr, nsName, prefix);
        return (*ns).next
    }
    return 0 as xmlNsPtr;
}
/*
* xmlDOMWrapNewCtxt:
*
* Allocates and initializes a new DOM-wrapper context.
*
* Returns the xmlDOMWrapCtxtPtr or NULL in case of an internal error.
*/
#[no_mangle]
pub unsafe extern "C" fn xmlDOMWrapNewCtxt() -> xmlDOMWrapCtxtPtr {
    let mut ret: xmlDOMWrapCtxtPtr = 0 as *mut xmlDOMWrapCtxt;
    ret =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlDOMWrapCtxt>()
                                                          as std::os::raw::c_ulong) as
            xmlDOMWrapCtxtPtr;
    if ret.is_null() {
        xmlTreeErrMemory(b"allocating DOM-wrapper context\x00" as *const u8 as
                             *const std::os::raw::c_char);
        return 0 as xmlDOMWrapCtxtPtr
    }
    memset(ret as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<xmlDOMWrapCtxt>() as std::os::raw::c_ulong);
    return ret;
}
/*
* xmlDOMWrapFreeCtxt:
* @ctxt: the DOM-wrapper context
*
* Frees the DOM-wrapper context.
*/
#[no_mangle]
pub unsafe extern "C" fn xmlDOMWrapFreeCtxt(mut ctxt: xmlDOMWrapCtxtPtr) {
    if ctxt.is_null() { return }
    if !(*ctxt).namespaceMap.is_null() {
        xmlDOMWrapNsMapFree((*ctxt).namespaceMap as xmlNsMapPtr);
    }
    /*
    * TODO: Store the namespace map in the context.
    */
    xmlFree.expect("non-null function pointer")(ctxt as *mut std::os::raw::c_void);
}
/*
* xmlTreeLookupNsListByPrefix:
* @nsList: a list of ns-structs
* @prefix: the searched prefix
*
* Searches for a ns-decl with the given prefix in @nsList.
*
* Returns the ns-decl if found, NULL if not found and on
*         API errors.
*/
unsafe extern "C" fn xmlTreeNSListLookupByPrefix(mut nsList: xmlNsPtr,
                                                 mut prefix: *const xmlChar)
 -> xmlNsPtr {
    if nsList.is_null() { return 0 as xmlNsPtr }
    let mut ns: xmlNsPtr = 0 as *mut xmlNs;
    ns = nsList;
    loop  {
        if prefix == (*ns).prefix || xmlStrEqual(prefix, (*ns).prefix) != 0 {
            return ns
        }
        ns = (*ns).next;
        if ns.is_null() { break ; }
    }
    return 0 as xmlNsPtr;
}
/*
*
* xmlDOMWrapNSNormGatherInScopeNs:
* @map: the namespace map
* @node: the node to start with
*
* Puts in-scope namespaces into the ns-map.
*
* Returns 0 on success, -1 on API or internal errors.
*/
unsafe extern "C" fn xmlDOMWrapNSNormGatherInScopeNs(mut map:
                                                         *mut xmlNsMapPtr,
                                                     mut node: xmlNodePtr)
 -> std::os::raw::c_int {
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    let mut ns: xmlNsPtr = 0 as *mut xmlNs;
    let mut mi: xmlNsMapItemPtr = 0 as *mut xmlNsMapItem;
    let mut shadowed: std::os::raw::c_int = 0;
    if map.is_null() || !(*map).is_null() { return -(1 as std::os::raw::c_int) }
    if node.is_null() ||
           (*node).type_0 as std::os::raw::c_uint ==
               XML_NAMESPACE_DECL as std::os::raw::c_int as std::os::raw::c_uint {
        return -(1 as std::os::raw::c_int)
    }
    /*
    * Get in-scope ns-decls of @parent.
    */
    cur = node;
    while !cur.is_null() && cur != (*cur).doc as xmlNodePtr {
        if (*cur).type_0 as std::os::raw::c_uint ==
               XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
            if !(*cur).nsDef.is_null() {
                ns = (*cur).nsDef;
                loop  {
                    shadowed = 0 as std::os::raw::c_int;
                    if !(*map).is_null() && !(**map).first.is_null() {
                        /*
			* Skip shadowed prefixes.
			*/
                        mi = (**map).first;
                        while !mi.is_null() {
                            if (*ns).prefix == (*(*mi).newNs).prefix ||
                                   xmlStrEqual((*ns).prefix,
                                               (*(*mi).newNs).prefix) != 0 {
                                shadowed = 1 as std::os::raw::c_int;
                                break ;
                            } else { mi = (*mi).next }
                        }
                    }
                    /*
		    * Insert mapping.
		    */
                    mi =
                        xmlDOMWrapNsMapAddItem(map, 0 as std::os::raw::c_int,
                                               0 as xmlNsPtr, ns,
                                               -(1 as std::os::raw::c_int));
                    if mi.is_null() { return -(1 as std::os::raw::c_int) }
                    if shadowed != 0 { (*mi).shadowDepth = 0 as std::os::raw::c_int }
                    ns = (*ns).next;
                    if ns.is_null() { break ; }
                }
            }
        }
        cur = (*cur).parent
    }
    return 0 as std::os::raw::c_int;
}
/*
* XML_TREE_ADOPT_STR: If we have a dest-dict, put @str in the dict;
* otherwise copy it, when it was in the source-dict.
*/
/*
* XML_TREE_ADOPT_STR_2: If @str was in the source-dict, then
* put it in dest-dict or copy it.
*/
/*
* xmlDOMWrapNSNormAddNsMapItem2:
*
* For internal use. Adds a ns-decl mapping.
*
* Returns 0 on success, -1 on internal errors.
*/
unsafe extern "C" fn xmlDOMWrapNSNormAddNsMapItem2(mut list:
                                                       *mut *mut xmlNsPtr,
                                                   mut size: *mut std::os::raw::c_int,
                                                   mut number:
                                                       *mut std::os::raw::c_int,
                                                   mut oldNs: xmlNsPtr,
                                                   mut newNs: xmlNsPtr)
 -> std::os::raw::c_int {
    if (*list).is_null() {
        *list =
            xmlMalloc.expect("non-null function pointer")((6 as std::os::raw::c_int as
                                                               std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<xmlNsPtr>()
                                                                                               as
                                                                                               std::os::raw::c_ulong))
                as *mut xmlNsPtr;
        if (*list).is_null() {
            xmlTreeErrMemory(b"alloc ns map item\x00" as *const u8 as
                                 *const std::os::raw::c_char);
            return -(1 as std::os::raw::c_int)
        }
        *size = 3 as std::os::raw::c_int;
        *number = 0 as std::os::raw::c_int
    } else if *number >= *size {
        *size *= 2 as std::os::raw::c_int;
        *list =
            xmlRealloc.expect("non-null function pointer")(*list as
                                                               *mut std::os::raw::c_void,
                                                           ((*size *
                                                                 2 as
                                                                     std::os::raw::c_int)
                                                                as
                                                                std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<xmlNsPtr>()
                                                                                                as
                                                                                                std::os::raw::c_ulong))
                as *mut xmlNsPtr;
        if (*list).is_null() {
            xmlTreeErrMemory(b"realloc ns map item\x00" as *const u8 as
                                 *const std::os::raw::c_char);
            return -(1 as std::os::raw::c_int)
        }
    }
    let ref mut fresh14 =
        *(*list).offset((2 as std::os::raw::c_int * *number) as isize);
    *fresh14 = oldNs;
    let ref mut fresh15 =
        *(*list).offset((2 as std::os::raw::c_int * *number + 1 as std::os::raw::c_int) as
                            isize);
    *fresh15 = newNs;
    *number += 1;
    return 0 as std::os::raw::c_int;
}
/*
* xmlDOMWrapRemoveNode:
* @ctxt: a DOM wrapper context
* @doc: the doc
* @node: the node to be removed.
* @options: set of options, unused at the moment
*
* Unlinks the given node from its owner.
* This will substitute ns-references to node->nsDef for
* ns-references to doc->oldNs, thus ensuring the removed
* branch to be autark wrt ns-references.
*
* NOTE: This function was not intensively tested.
*
* Returns 0 on success, 1 if the node is not supported,
*         -1 on API and internal errors.
*/
#[no_mangle]
pub unsafe extern "C" fn xmlDOMWrapRemoveNode(mut ctxt: xmlDOMWrapCtxtPtr,
                                              mut doc: xmlDocPtr,
                                              mut node: xmlNodePtr,
                                              mut options: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut current_block: u64;
    let mut list: *mut xmlNsPtr = 0 as *mut xmlNsPtr;
    let mut sizeList: std::os::raw::c_int = 0;
    let mut nbList: std::os::raw::c_int = 0;
    let mut i: std::os::raw::c_int = 0;
    let mut j: std::os::raw::c_int = 0;
    let mut ns: xmlNsPtr = 0 as *mut xmlNs;
    if node.is_null() || doc.is_null() || (*node).doc != doc {
        return -(1 as std::os::raw::c_int)
    }
    /* TODO: 0 or -1 ? */
    if (*node).parent.is_null() { return 0 as std::os::raw::c_int }
    match (*node).type_0 as std::os::raw::c_uint {
        3 | 4 | 5 | 7 | 8 => { xmlUnlinkNode(node); return 0 as std::os::raw::c_int }
        1 | 2 => { }
        _ => { return 1 as std::os::raw::c_int }
    }
    xmlUnlinkNode(node);
    's_67:
        loop 
             /*
    * Save out-of-scope ns-references in doc->oldNs.
    */
             {
            match (*node).type_0 as std::os::raw::c_uint {
                1 => {
                    if ctxt.is_null() && !(*node).nsDef.is_null() {
                        ns = (*node).nsDef;
                        loop  {
                            if xmlDOMWrapNSNormAddNsMapItem2(&mut list,
                                                             &mut sizeList,
                                                             &mut nbList, ns,
                                                             ns) ==
                                   -(1 as std::os::raw::c_int) {
                                current_block = 4242573898291644546;
                                break 's_67 ;
                            }
                            ns = (*ns).next;
                            if ns.is_null() { break ; }
                        }
                        current_block = 9505007389286077660;
                    } else { current_block = 9505007389286077660; }
                }
                2 => { current_block = 9505007389286077660; }
                _ => { current_block = 13628895987633767338; }
            }
            match current_block {
                9505007389286077660 =>
                /* Falls through. */
                {
                    if !(*node).ns.is_null() {
                        /*
		    * Find a mapping.
		    */
                        if !list.is_null() {
                            i = 0 as std::os::raw::c_int;
                            j = 0 as std::os::raw::c_int;
                            loop  {
                                if !(i < nbList) {
                                    current_block = 3437258052017859086;
                                    break ;
                                }
                                if (*node).ns == *list.offset(j as isize) {
                                    j += 1;
                                    (*node).ns = *list.offset(j as isize);
                                    current_block = 4086940171903379882;
                                    break ;
                                } else { i += 1; j += 2 as std::os::raw::c_int }
                            }
                        } else { current_block = 3437258052017859086; }
                        match current_block {
                            4086940171903379882 => { }
                            _ => {
                                ns = 0 as xmlNsPtr;
                                if ctxt.is_null() {
                                    /*
			* Add to doc's oldNs.
			*/
                                    ns =
                                        xmlDOMWrapStoreNs(doc,
                                                          (*(*node).ns).href,
                                                          (*(*node).ns).prefix);
                                    if ns.is_null() {
                                        current_block = 4242573898291644546;
                                        break ;
                                    }
                                }
                                /*
			* User defined.
			*/
                                if !ns.is_null() {
                                    /*
			* Add mapping.
			*/
                                    if xmlDOMWrapNSNormAddNsMapItem2(&mut list,
                                                                     &mut sizeList,
                                                                     &mut nbList,
                                                                     (*node).ns,
                                                                     ns) ==
                                           -(1 as std::os::raw::c_int) {
                                        current_block = 4242573898291644546;
                                        break ;
                                    }
                                }
                                (*node).ns = ns;
                                current_block = 3123434771885419771;
                            }
                        }
                    } else { current_block = 3123434771885419771; }
                    match current_block {
                        3123434771885419771 => {
                            if (*node).type_0 as std::os::raw::c_uint ==
                                   XML_ELEMENT_NODE as std::os::raw::c_int as
                                       std::os::raw::c_uint &&
                                   !(*node).properties.is_null() {
                                node = (*node).properties as xmlNodePtr;
                                current_block = 2979737022853876585;
                            } else { current_block = 4086940171903379882; }
                        }
                        _ => { }
                    }
                    match current_block {
                        2979737022853876585 => { }
                        _ => {
                            if (*node).type_0 as std::os::raw::c_uint ==
                                   XML_ELEMENT_NODE as std::os::raw::c_int as
                                       std::os::raw::c_uint &&
                                   !(*node).children.is_null() {
                                node = (*node).children;
                                current_block = 2979737022853876585;
                            } else { current_block = 13628895987633767338; }
                        }
                    }
                }
                _ => { }
            }
            loop  {
                match current_block {
                    2979737022853876585 => {
                        if !node.is_null() {
                            break ;
                        } else {
                            current_block = 10095721787123848864;
                            break 's_67 ;
                        }
                    }
                    _ => {
                        if node.is_null() {
                            current_block = 10095721787123848864;
                            break 's_67 ;
                        }
                        if !(*node).next.is_null() {
                            node = (*node).next;
                            current_block = 2979737022853876585;
                        } else {
                            node = (*node).parent;
                            current_block = 13628895987633767338;
                        }
                    }
                }
            }
        }
    match current_block {
        4242573898291644546 => {
            if !list.is_null() {
                xmlFree.expect("non-null function pointer")(list as
                                                                *mut std::os::raw::c_void);
            }
            return -(1 as std::os::raw::c_int)
        }
        _ => {
            if !list.is_null() {
                xmlFree.expect("non-null function pointer")(list as
                                                                *mut std::os::raw::c_void);
            }
            return 0 as std::os::raw::c_int
        }
    };
}
/*
* xmlSearchNsByNamespaceStrict:
* @doc: the document
* @node: the start node
* @nsName: the searched namespace name
* @retNs: the resulting ns-decl
* @prefixed: if the found ns-decl must have a prefix (for attributes)
*
* Dynamically searches for a ns-declaration which matches
* the given @nsName in the ancestor-or-self axis of @node.
*
* Returns 1 if a ns-decl was found, 0 if not and -1 on API
*         and internal errors.
*/
unsafe extern "C" fn xmlSearchNsByNamespaceStrict(mut doc: xmlDocPtr,
                                                  mut node: xmlNodePtr,
                                                  mut nsName: *const xmlChar,
                                                  mut retNs: *mut xmlNsPtr,
                                                  mut prefixed: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    let mut prev: xmlNodePtr = 0 as xmlNodePtr;
    let mut out: xmlNodePtr = 0 as xmlNodePtr;
    let mut ns: xmlNsPtr = 0 as *mut xmlNs;
    let mut prevns: xmlNsPtr = 0 as *mut xmlNs;
    if doc.is_null() || nsName.is_null() || retNs.is_null() {
        return -(1 as std::os::raw::c_int)
    }
    if node.is_null() ||
           (*node).type_0 as std::os::raw::c_uint ==
               XML_NAMESPACE_DECL as std::os::raw::c_int as std::os::raw::c_uint {
        return -(1 as std::os::raw::c_int)
    }
    *retNs = 0 as xmlNsPtr;
    if xmlStrEqual(nsName,
                   b"http://www.w3.org/XML/1998/namespace\x00" as *const u8 as
                       *const std::os::raw::c_char as *const xmlChar) != 0 {
        *retNs = xmlTreeEnsureXMLDecl(doc);
        if (*retNs).is_null() { return -(1 as std::os::raw::c_int) }
        return 1 as std::os::raw::c_int
    }
    cur = node;
    loop  {
        if (*cur).type_0 as std::os::raw::c_uint ==
               XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
            if !(*cur).nsDef.is_null() {
                let mut current_block_20: u64;
                ns = (*cur).nsDef;
                while !ns.is_null() {
                    if !(prefixed != 0 && (*ns).prefix.is_null()) {
                        if !prev.is_null() {
                            /*
			* Check the last level of ns-decls for a
			* shadowing prefix.
			*/
                            prevns = (*prev).nsDef;
                            while !((*prevns).prefix == (*ns).prefix ||
                                        !(*prevns).prefix.is_null() &&
                                            !(*ns).prefix.is_null() &&
                                            xmlStrEqual((*prevns).prefix,
                                                        (*ns).prefix) != 0) {
                                prevns = (*prevns).next;
                                if prevns.is_null() { break ; }
                            }
                            if !prevns.is_null() {
                                current_block_20 = 12349973810996921269;
                            } else { current_block_20 = 2719512138335094285; }
                        } else { current_block_20 = 2719512138335094285; }
                        match current_block_20 {
                            12349973810996921269 => { }
                            _ =>
                            /*
		    * Ns-name comparison.
		    */
                            {
                                if nsName == (*ns).href ||
                                       xmlStrEqual(nsName, (*ns).href) != 0 {
                                    /*
			* At this point the prefix can only be shadowed,
			* if we are the the (at least) 3rd level of
			* ns-decls.
			*/
                                    if !out.is_null() {
                                        let mut ret: std::os::raw::c_int = 0;
                                        ret =
                                            xmlNsInScope(doc, node, prev,
                                                         (*ns).prefix);
                                        if ret < 0 as std::os::raw::c_int {
                                            return -(1 as std::os::raw::c_int)
                                        }
                                        /*
			    * TODO: Should we try to find a matching ns-name
			    * only once? This here keeps on searching.
			    * I think we should try further since, there might
			    * be an other matching ns-decl with an unshadowed
			    * prefix.
			    */
                                        if ret == 0 {
                                            current_block_20 =
                                                12349973810996921269;
                                        } else {
                                            current_block_20 =
                                                17281240262373992796;
                                        }
                                    } else {
                                        current_block_20 =
                                            17281240262373992796;
                                    }
                                    match current_block_20 {
                                        12349973810996921269 => { }
                                        _ => {
                                            *retNs = ns;
                                            return 1 as std::os::raw::c_int
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ns = (*ns).next
                }
                out = prev;
                prev = cur
            }
        } else if (*cur).type_0 as std::os::raw::c_uint ==
                      XML_ENTITY_NODE as std::os::raw::c_int as std::os::raw::c_uint ||
                      (*cur).type_0 as std::os::raw::c_uint ==
                          XML_ENTITY_DECL as std::os::raw::c_int as std::os::raw::c_uint {
            return 0 as std::os::raw::c_int
        }
        cur = (*cur).parent;
        if !(!cur.is_null() && (*cur).doc != cur as xmlDocPtr) { break ; }
    }
    return 0 as std::os::raw::c_int;
}
/*
* xmlSearchNsByPrefixStrict:
* @doc: the document
* @node: the start node
* @prefix: the searched namespace prefix
* @retNs: the resulting ns-decl
*
* Dynamically searches for a ns-declaration which matches
* the given @nsName in the ancestor-or-self axis of @node.
*
* Returns 1 if a ns-decl was found, 0 if not and -1 on API
*         and internal errors.
*/
unsafe extern "C" fn xmlSearchNsByPrefixStrict(mut doc: xmlDocPtr,
                                               mut node: xmlNodePtr,
                                               mut prefix: *const xmlChar,
                                               mut retNs: *mut xmlNsPtr)
 -> std::os::raw::c_int {
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    let mut ns: xmlNsPtr = 0 as *mut xmlNs;
    if doc.is_null() || node.is_null() ||
           (*node).type_0 as std::os::raw::c_uint ==
               XML_NAMESPACE_DECL as std::os::raw::c_int as std::os::raw::c_uint {
        return -(1 as std::os::raw::c_int)
    }
    if !retNs.is_null() { *retNs = 0 as xmlNsPtr }
    if !prefix.is_null() &&
           *prefix.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
               'x' as i32 &&
           *prefix.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
               'm' as i32 &&
           *prefix.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
               'l' as i32 &&
           *prefix.offset(3 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
               0 as std::os::raw::c_int {
        if !retNs.is_null() {
            *retNs = xmlTreeEnsureXMLDecl(doc);
            if (*retNs).is_null() { return -(1 as std::os::raw::c_int) }
        }
        return 1 as std::os::raw::c_int
    }
    cur = node;
    loop  {
        if (*cur).type_0 as std::os::raw::c_uint ==
               XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
            if !(*cur).nsDef.is_null() {
                ns = (*cur).nsDef;
                loop  {
                    if prefix == (*ns).prefix ||
                           xmlStrEqual(prefix, (*ns).prefix) != 0 {
                        /*
			* Disabled namespaces, e.g. xmlns:abc="".
			*/
                        if (*ns).href.is_null() { return 0 as std::os::raw::c_int }
                        if !retNs.is_null() { *retNs = ns }
                        return 1 as std::os::raw::c_int
                    }
                    ns = (*ns).next;
                    if ns.is_null() { break ; }
                }
            }
        } else if (*cur).type_0 as std::os::raw::c_uint ==
                      XML_ENTITY_NODE as std::os::raw::c_int as std::os::raw::c_uint ||
                      (*cur).type_0 as std::os::raw::c_uint ==
                          XML_ENTITY_DECL as std::os::raw::c_int as std::os::raw::c_uint {
            return 0 as std::os::raw::c_int
        }
        cur = (*cur).parent;
        if !(!cur.is_null() && (*cur).doc != cur as xmlDocPtr) { break ; }
    }
    return 0 as std::os::raw::c_int;
}
/*
* xmlDOMWrapNSNormDeclareNsForced:
* @doc: the doc
* @elem: the element-node to declare on
* @nsName: the namespace-name of the ns-decl
* @prefix: the preferred prefix of the ns-decl
* @checkShadow: ensure that the new ns-decl doesn't shadow ancestor ns-decls
*
* Declares a new namespace on @elem. It tries to use the
* given @prefix; if a ns-decl with the given prefix is already existent
* on @elem, it will generate an other prefix.
*
* Returns 1 if a ns-decl was found, 0 if not and -1 on API
*         and internal errors.
*/
unsafe extern "C" fn xmlDOMWrapNSNormDeclareNsForced(mut doc: xmlDocPtr,
                                                     mut elem: xmlNodePtr,
                                                     mut nsName:
                                                         *const xmlChar,
                                                     mut prefix:
                                                         *const xmlChar,
                                                     mut checkShadow:
                                                         std::os::raw::c_int)
 -> xmlNsPtr {
    let mut current_block: u64;
    let mut ret: xmlNsPtr = 0 as *mut xmlNs;
    let mut buf: [std::os::raw::c_char; 50] = [0; 50];
    let mut pref: *const xmlChar = 0 as *const xmlChar;
    let mut counter: std::os::raw::c_int = 0 as std::os::raw::c_int;
    if doc.is_null() || elem.is_null() ||
           (*elem).type_0 as std::os::raw::c_uint !=
               XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        return 0 as xmlNsPtr
    }
    /*
    * Create a ns-decl on @anchor.
    */
    pref = prefix;
    loop 
         /*
	* Lookup whether the prefix is unused in elem's ns-decls.
	*/
         {
        if !(!(*elem).nsDef.is_null() &&
                 !xmlTreeNSListLookupByPrefix((*elem).nsDef, pref).is_null())
           {
            if checkShadow != 0 && !(*elem).parent.is_null() &&
                   (*(*elem).parent).doc as xmlNodePtr != (*elem).parent {
                /*
	    * Does it shadow ancestor ns-decls?
	    */
                if xmlSearchNsByPrefixStrict(doc, (*elem).parent, pref,
                                             0 as *mut xmlNsPtr) ==
                       1 as std::os::raw::c_int {
                    current_block = 13374222940099593895;
                } else { current_block = 1394248824506584008; }
            } else { current_block = 1394248824506584008; }
            match current_block {
                13374222940099593895 => { }
                _ => {
                    ret = xmlNewNs(0 as xmlNodePtr, nsName, pref);
                    if ret.is_null() { return 0 as xmlNsPtr }
                    if (*elem).nsDef.is_null() {
                        (*elem).nsDef = ret
                    } else {
                        let mut ns2: xmlNsPtr = (*elem).nsDef;
                        while !(*ns2).next.is_null() { ns2 = (*ns2).next }
                        (*ns2).next = ret
                    }
                    return ret
                }
            }
        }
        counter += 1;
        if counter > 1000 as std::os::raw::c_int { return 0 as xmlNsPtr }
        if prefix.is_null() {
            snprintf(buf.as_mut_ptr(),
                     ::std::mem::size_of::<[std::os::raw::c_char; 50]>() as
                         std::os::raw::c_ulong,
                     b"ns_%d\x00" as *const u8 as *const std::os::raw::c_char,
                     counter);
        } else {
            snprintf(buf.as_mut_ptr(),
                     ::std::mem::size_of::<[std::os::raw::c_char; 50]>() as
                         std::os::raw::c_ulong,
                     b"%.30s_%d\x00" as *const u8 as *const std::os::raw::c_char,
                     prefix as *mut std::os::raw::c_char, counter);
        }
        pref = buf.as_mut_ptr() as *mut xmlChar
    };
}
/*
* xmlDOMWrapNSNormAquireNormalizedNs:
* @doc: the doc
* @elem: the element-node to declare namespaces on
* @ns: the ns-struct to use for the search
* @retNs: the found/created ns-struct
* @nsMap: the ns-map
* @depth: the current tree depth
* @ancestorsOnly: search in ancestor ns-decls only
* @prefixed: if the searched ns-decl must have a prefix (for attributes)
*
* Searches for a matching ns-name in the ns-decls of @nsMap, if not
* found it will either declare it on @elem, or store it in doc->oldNs.
* If a new ns-decl needs to be declared on @elem, it tries to use the
* @ns->prefix for it, if this prefix is already in use on @elem, it will
* change the prefix or the new ns-decl.
*
* Returns 0 if succeeded, -1 otherwise and on API/internal errors.
*/
unsafe extern "C" fn xmlDOMWrapNSNormAquireNormalizedNs(mut doc: xmlDocPtr,
                                                        mut elem: xmlNodePtr,
                                                        mut ns: xmlNsPtr,
                                                        mut retNs:
                                                            *mut xmlNsPtr,
                                                        mut nsMap:
                                                            *mut xmlNsMapPtr,
                                                        mut depth:
                                                            std::os::raw::c_int,
                                                        mut ancestorsOnly:
                                                            std::os::raw::c_int,
                                                        mut prefixed:
                                                            std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut mi: xmlNsMapItemPtr = 0 as *mut xmlNsMapItem;
    if doc.is_null() || ns.is_null() || retNs.is_null() || nsMap.is_null() {
        return -(1 as std::os::raw::c_int)
    }
    *retNs = 0 as xmlNsPtr;
    /*
    * Handle XML namespace.
    */
    if !(*ns).prefix.is_null() &&
           *(*ns).prefix.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
               'x' as i32 &&
           *(*ns).prefix.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
               'm' as i32 &&
           *(*ns).prefix.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
               'l' as i32 &&
           *(*ns).prefix.offset(3 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
               0 as std::os::raw::c_int {
        /*
	* Insert XML namespace mapping.
	*/
        *retNs = xmlTreeEnsureXMLDecl(doc);
        if (*retNs).is_null() { return -(1 as std::os::raw::c_int) }
        return 0 as std::os::raw::c_int
    }
    /*
    * If the search should be done in ancestors only and no
    * @elem (the first ancestor) was specified, then skip the search.
    */
    if !(*nsMap).is_null() && !(**nsMap).first.is_null() &&
           !(ancestorsOnly != 0 && elem.is_null()) {
        /*
	* Try to find an equal ns-name in in-scope ns-decls.
	*/
        mi = (**nsMap).first;
        while !mi.is_null() {
            if (*mi).depth >= -(1 as std::os::raw::c_int) &&
                   (ancestorsOnly == 0 || (*mi).depth == -(1 as std::os::raw::c_int))
                   && (*mi).shadowDepth == -(1 as std::os::raw::c_int) &&
                   (!(*(*mi).newNs).href.is_null() &&
                        *(*(*mi).newNs).href.offset(0 as std::os::raw::c_int as isize)
                            as std::os::raw::c_int != 0 as std::os::raw::c_int) &&
                   (prefixed == 0 || !(*(*mi).newNs).prefix.is_null()) &&
                   ((*(*mi).newNs).href == (*ns).href ||
                        xmlStrEqual((*(*mi).newNs).href, (*ns).href) != 0) {
                /* Set the mapping. */
                (*mi).oldNs = ns;
                *retNs = (*mi).newNs;
                return 0 as std::os::raw::c_int
            }
            mi = (*mi).next
        }
    }
    /*
    * No luck, the namespace is out of scope or shadowed.
    */
    if elem.is_null() {
        let mut tmpns: xmlNsPtr = 0 as *mut xmlNs;
        /*
	* Store ns-decls in "oldNs" of the document-node.
	*/
        tmpns = xmlDOMWrapStoreNs(doc, (*ns).href, (*ns).prefix);
        if tmpns.is_null() { return -(1 as std::os::raw::c_int) }
        /*
	* Insert mapping.
	*/
        if xmlDOMWrapNsMapAddItem(nsMap, -(1 as std::os::raw::c_int), ns, tmpns,
                                  -(3 as std::os::raw::c_int)).is_null() {
            xmlFreeNs(tmpns);
            return -(1 as std::os::raw::c_int)
        }
        *retNs = tmpns
    } else {
        let mut tmpns_0: xmlNsPtr = 0 as *mut xmlNs;
        tmpns_0 =
            xmlDOMWrapNSNormDeclareNsForced(doc, elem, (*ns).href,
                                            (*ns).prefix, 0 as std::os::raw::c_int);
        if tmpns_0.is_null() { return -(1 as std::os::raw::c_int) }
        if !(*nsMap).is_null() {
            /*
	    * Does it shadow ancestor ns-decls?
	    */
            mi = (**nsMap).first;
            while !mi.is_null() {
                if (*mi).depth < depth &&
                       (*mi).shadowDepth == -(1 as std::os::raw::c_int) &&
                       ((*ns).prefix == (*(*mi).newNs).prefix ||
                            xmlStrEqual((*ns).prefix, (*(*mi).newNs).prefix)
                                != 0) {
                    /*
		    * Shadows.
		    */
                    (*mi).shadowDepth = depth;
                    break ;
                } else { mi = (*mi).next }
            }
        }
        if xmlDOMWrapNsMapAddItem(nsMap, -(1 as std::os::raw::c_int), ns, tmpns_0,
                                  depth).is_null() {
            xmlFreeNs(tmpns_0);
            return -(1 as std::os::raw::c_int)
        }
        *retNs = tmpns_0
    }
    return 0 as std::os::raw::c_int;
}
/*
* xmlDOMWrapReconcileNamespaces:
* @ctxt: DOM wrapper context, unused at the moment
* @elem: the element-node
* @options: option flags
*
* Ensures that ns-references point to ns-decls hold on element-nodes.
* Ensures that the tree is namespace wellformed by creating additional
* ns-decls where needed. Note that, since prefixes of already existent
* ns-decls can be shadowed by this process, it could break QNames in
* attribute values or element content.
*
* NOTE: This function was not intensively tested.
*
* Returns 0 if succeeded, -1 otherwise and on API/internal errors.
*/
#[no_mangle]
pub unsafe extern "C" fn xmlDOMWrapReconcileNamespaces(mut ctxt:
                                                           xmlDOMWrapCtxtPtr,
                                                       mut elem: xmlNodePtr,
                                                       mut options:
                                                           std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut current_block: u64;
    let mut depth: std::os::raw::c_int = -(1 as std::os::raw::c_int);
    let mut adoptns: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut parnsdone: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut ns: xmlNsPtr = 0 as *mut xmlNs;
    let mut prevns: xmlNsPtr = 0 as *mut xmlNs;
    let mut doc: xmlDocPtr = 0 as *mut xmlDoc;
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    let mut curElem: xmlNodePtr = 0 as xmlNodePtr;
    let mut nsMap: xmlNsMapPtr = 0 as xmlNsMapPtr;
    let mut mi: xmlNsMapItemPtr = 0 as *mut xmlNsMapItem;
    /* @ancestorsOnly should be set by an option flag. */
    let mut ancestorsOnly: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut optRemoveRedundantNS: std::os::raw::c_int =
        if options as xmlDOMReconcileNSOptions as std::os::raw::c_uint &
               XML_DOM_RECONNS_REMOVEREDUND as std::os::raw::c_int as std::os::raw::c_uint !=
               0 {
            1 as std::os::raw::c_int
        } else { 0 as std::os::raw::c_int };
    let mut listRedund: *mut xmlNsPtr = 0 as *mut xmlNsPtr;
    let mut sizeRedund: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut nbRedund: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut ret: std::os::raw::c_int = 0;
    let mut i: std::os::raw::c_int = 0;
    let mut j: std::os::raw::c_int = 0;
    if elem.is_null() || (*elem).doc.is_null() ||
           (*elem).type_0 as std::os::raw::c_uint !=
               XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        return -(1 as std::os::raw::c_int)
    }
    doc = (*elem).doc;
    cur = elem;
    's_51:
        loop  {
            match (*cur).type_0 as std::os::raw::c_uint {
                1 => {
                    adoptns = 1 as std::os::raw::c_int;
                    curElem = cur;
                    depth += 1;
                    /*
		* Namespace declarations.
		*/
                    if !(*cur).nsDef.is_null() {
                        prevns = 0 as xmlNsPtr;
                        ns = (*cur).nsDef;
                        while !ns.is_null() {
                            if parnsdone == 0 {
                                if !(*elem).parent.is_null() &&
                                       (*(*elem).parent).doc as xmlNodePtr !=
                                           (*elem).parent {
                                    /*
				* Gather ancestor in-scope ns-decls.
				*/
                                    if xmlDOMWrapNSNormGatherInScopeNs(&mut nsMap,
                                                                       (*elem).parent)
                                           == -(1 as std::os::raw::c_int) {
                                        current_block = 10580685211620783579;
                                        break 's_51 ;
                                    }
                                }
                                parnsdone = 1 as std::os::raw::c_int
                            }
                            /*
			* Lookup the ns ancestor-axis for equal ns-decls in scope.
			*/
                            if optRemoveRedundantNS != 0 &&
                                   (!nsMap.is_null() &&
                                        !(*nsMap).first.is_null()) {
                                mi = (*nsMap).first;
                                loop  {
                                    if mi.is_null() {
                                        current_block = 652864300344834934;
                                        break ;
                                    }
                                    if (*mi).depth >= -(1 as std::os::raw::c_int) &&
                                           (*mi).shadowDepth ==
                                               -(1 as std::os::raw::c_int) &&
                                           ((*ns).prefix ==
                                                (*(*mi).newNs).prefix ||
                                                xmlStrEqual((*ns).prefix,
                                                            (*(*mi).newNs).prefix)
                                                    != 0) &&
                                           ((*ns).href == (*(*mi).newNs).href
                                                ||
                                                xmlStrEqual((*ns).href,
                                                            (*(*mi).newNs).href)
                                                    != 0) {
                                        /*
				    * A redundant ns-decl was found.
				    * Add it to the list of redundant ns-decls.
				    */
                                        if xmlDOMWrapNSNormAddNsMapItem2(&mut listRedund,
                                                                         &mut sizeRedund,
                                                                         &mut nbRedund,
                                                                         ns,
                                                                         (*mi).newNs)
                                               == -(1 as std::os::raw::c_int) {
                                            current_block =
                                                10580685211620783579;
                                            break 's_51 ;
                                        }
                                        /*
				    * Remove the ns-decl from the element-node.
				    */
                                        if !prevns.is_null() {
                                            (*prevns).next = (*ns).next
                                        } else { (*cur).nsDef = (*ns).next }
                                        current_block = 2285365832428419935;
                                        break ;
                                    } else { mi = (*mi).next }
                                }
                            } else { current_block = 652864300344834934; }
                            match current_block {
                                652864300344834934 => {
                                    /*
			* Skip ns-references handling if the referenced
			* ns-decl is declared on the same element.
			*/
                                    if !(*cur).ns.is_null() && adoptns != 0 &&
                                           (*cur).ns == ns {
                                        adoptns = 0 as std::os::raw::c_int
                                    }
                                    /*
			* Does it shadow any ns-decl?
			*/
                                    if !nsMap.is_null() &&
                                           !(*nsMap).first.is_null() {
                                        mi = (*nsMap).first;
                                        while !mi.is_null() {
                                            if (*mi).depth >=
                                                   -(1 as std::os::raw::c_int) &&
                                                   (*mi).shadowDepth ==
                                                       -(1 as std::os::raw::c_int) &&
                                                   ((*ns).prefix ==
                                                        (*(*mi).newNs).prefix
                                                        ||
                                                        xmlStrEqual((*ns).prefix,
                                                                    (*(*mi).newNs).prefix)
                                                            != 0) {
                                                (*mi).shadowDepth = depth
                                            }
                                            mi = (*mi).next
                                        }
                                    }
                                    /*
			* Push mapping.
			*/
                                    if xmlDOMWrapNsMapAddItem(&mut nsMap,
                                                              -(1 as
                                                                    std::os::raw::c_int),
                                                              ns, ns,
                                                              depth).is_null()
                                       {
                                        current_block = 10580685211620783579;
                                        break 's_51 ;
                                    }
                                    prevns = ns
                                }
                                _ => { }
                            }
                            ns = (*ns).next
                        }
                    }
                    if adoptns == 0 {
                        current_block = 10508608658004042745;
                    } else {
                        /* Falls through. */
                        current_block = 14849880465176821447;
                    }
                }
                2 => { current_block = 14849880465176821447; }
                _ => { current_block = 11327682861772062331; }
            }
            match current_block {
                14849880465176821447 =>
                /* No ns, no fun. */
                {
                    if (*cur).ns.is_null() {
                        current_block = 10508608658004042745;
                    } else {
                        if parnsdone == 0 {
                            if !(*elem).parent.is_null() &&
                                   (*(*elem).parent).doc as xmlNodePtr !=
                                       (*elem).parent {
                                if xmlDOMWrapNSNormGatherInScopeNs(&mut nsMap,
                                                                   (*elem).parent)
                                       == -(1 as std::os::raw::c_int) {
                                    current_block = 10580685211620783579;
                                    break ;
                                }
                            }
                            parnsdone = 1 as std::os::raw::c_int
                        }
                        /*
		* Adjust the reference if this was a redundant ns-decl.
		*/
                        if !listRedund.is_null() {
                            i = 0 as std::os::raw::c_int;
                            j = 0 as std::os::raw::c_int;
                            while i < nbRedund {
                                if (*cur).ns == *listRedund.offset(j as isize)
                                   {
                                    j += 1;
                                    (*cur).ns =
                                        *listRedund.offset(j as isize);
                                    break ;
                                } else { i += 1; j += 2 as std::os::raw::c_int }
                            }
                        }
                        /*
		* Adopt ns-references.
		*/
                        if !nsMap.is_null() && !(*nsMap).first.is_null() {
                            /*
		    * Search for a mapping.
		    */
                            mi = (*nsMap).first;
                            loop  {
                                if mi.is_null() {
                                    current_block = 7189308829251266000;
                                    break ;
                                }
                                if (*mi).shadowDepth == -(1 as std::os::raw::c_int) &&
                                       (*cur).ns == (*mi).oldNs {
                                    (*cur).ns = (*mi).newNs;
                                    current_block = 10508608658004042745;
                                    break ;
                                } else { mi = (*mi).next }
                            }
                        } else { current_block = 7189308829251266000; }
                        match current_block {
                            10508608658004042745 => { }
                            _ =>
                            /*
		* Aquire a normalized ns-decl and add it to the map.
		*/
                            {
                                if xmlDOMWrapNSNormAquireNormalizedNs(doc,
                                                                      curElem,
                                                                      (*cur).ns,
                                                                      &mut ns,
                                                                      &mut nsMap,
                                                                      depth,
                                                                      ancestorsOnly,
                                                                      (if (*cur).type_0
                                                                              as
                                                                              std::os::raw::c_uint
                                                                              ==
                                                                              XML_ATTRIBUTE_NODE
                                                                                  as
                                                                                  std::os::raw::c_int
                                                                                  as
                                                                                  std::os::raw::c_uint
                                                                          {
                                                                           1
                                                                               as
                                                                               std::os::raw::c_int
                                                                       } else {
                                                                           0
                                                                               as
                                                                               std::os::raw::c_int
                                                                       })) ==
                                       -(1 as std::os::raw::c_int) {
                                    current_block = 10580685211620783579;
                                    break ;
                                }
                                (*cur).ns = ns;
                                current_block = 10508608658004042745;
                            }
                        }
                    }
                }
                _ => { }
            }
            match current_block {
                10508608658004042745 => {
                    if (*cur).type_0 as std::os::raw::c_uint ==
                           XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
                           !(*cur).properties.is_null() {
                        /*
		    * Process attributes.
		    */
                        cur = (*cur).properties as xmlNodePtr;
                        current_block = 1394248824506584008;
                    } else { current_block = 5404344754664734192; }
                }
                _ => { }
            }
            loop  {
                match current_block {
                    1394248824506584008 => {
                        if !cur.is_null() {
                            break ;
                        } else {
                            current_block = 7739940392431776979;
                            break 's_51 ;
                        }
                    }
                    11327682861772062331 => {
                        if cur == elem {
                            current_block = 7739940392431776979;
                            break 's_51 ;
                        }
                        if (*cur).type_0 as std::os::raw::c_uint ==
                               XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint
                           {
                            if !nsMap.is_null() && !(*nsMap).first.is_null() {
                                /*
		* Pop mappings.
		*/
                                while !(*nsMap).last.is_null() &&
                                          (*(*nsMap).last).depth >= depth {
                                    mi = (*nsMap).last;
                                    (*nsMap).last = (*mi).prev;
                                    if (*nsMap).last.is_null() {
                                        (*nsMap).first = 0 as xmlNsMapItemPtr
                                    } else {
                                        (*(*nsMap).last).next =
                                            0 as xmlNsMapItemPtr
                                    }
                                    (*mi).next = (*nsMap).pool;
                                    (*nsMap).pool = mi
                                }
                                /*
		* Unshadow.
		*/
                                mi = (*nsMap).first;
                                while !mi.is_null() {
                                    if (*mi).shadowDepth >= depth {
                                        (*mi).shadowDepth =
                                            -(1 as std::os::raw::c_int)
                                    }
                                    mi = (*mi).next
                                }
                            }
                            depth -= 1
                        }
                        if !(*cur).next.is_null() {
                            cur = (*cur).next;
                            current_block = 1394248824506584008;
                        } else if (*cur).type_0 as std::os::raw::c_uint ==
                                      XML_ATTRIBUTE_NODE as std::os::raw::c_int as
                                          std::os::raw::c_uint {
                            cur = (*cur).parent;
                            current_block = 5404344754664734192;
                        } else {
                            cur = (*cur).parent;
                            current_block = 11327682861772062331;
                        }
                    }
                    _ => {
                        if !((*cur).type_0 as std::os::raw::c_uint ==
                                 XML_ELEMENT_NODE as std::os::raw::c_int as
                                     std::os::raw::c_uint &&
                                 !(*cur).children.is_null()) {
                            current_block = 11327682861772062331;
                            continue ;
                        }
                        /*
	    * Process content of element-nodes only.
	    */
                        cur = (*cur).children;
                        current_block = 1394248824506584008;
                    }
                }
            }
        }
    match current_block {
        10580685211620783579 => { ret = -(1 as std::os::raw::c_int) }
        _ => { ret = 0 as std::os::raw::c_int }
    }
    if !listRedund.is_null() {
        i = 0 as std::os::raw::c_int;
        j = 0 as std::os::raw::c_int;
        while i < nbRedund {
            xmlFreeNs(*listRedund.offset(j as isize));
            i += 1;
            j += 2 as std::os::raw::c_int
        }
        xmlFree.expect("non-null function pointer")(listRedund as
                                                        *mut std::os::raw::c_void);
    }
    if !nsMap.is_null() { xmlDOMWrapNsMapFree(nsMap); }
    return ret;
}
/*
* xmlDOMWrapAdoptBranch:
* @ctxt: the optional context for custom processing
* @sourceDoc: the optional sourceDoc
* @node: the element-node to start with
* @destDoc: the destination doc for adoption
* @destParent: the optional new parent of @node in @destDoc
* @options: option flags
*
* Ensures that ns-references point to @destDoc: either to
* elements->nsDef entries if @destParent is given, or to
* @destDoc->oldNs otherwise.
* If @destParent is given, it ensures that the tree is namespace
* wellformed by creating additional ns-decls where needed.
* Note that, since prefixes of already existent ns-decls can be
* shadowed by this process, it could break QNames in attribute
* values or element content.
*
* NOTE: This function was not intensively tested.
*
* Returns 0 if succeeded, -1 otherwise and on API/internal errors.
*/
unsafe extern "C" fn xmlDOMWrapAdoptBranch(mut ctxt: xmlDOMWrapCtxtPtr,
                                           mut sourceDoc: xmlDocPtr,
                                           mut node: xmlNodePtr,
                                           mut destDoc: xmlDocPtr,
                                           mut destParent: xmlNodePtr,
                                           mut options: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut current_block: u64;
    let mut ret: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    let mut curElem: xmlNodePtr = 0 as xmlNodePtr;
    let mut nsMap: xmlNsMapPtr = 0 as xmlNsMapPtr;
    let mut mi: xmlNsMapItemPtr = 0 as *mut xmlNsMapItem;
    let mut ns: xmlNsPtr = 0 as xmlNsPtr;
    let mut depth: std::os::raw::c_int = -(1 as std::os::raw::c_int);
    let mut adoptStr: std::os::raw::c_int = 1 as std::os::raw::c_int;
    /* gather @parent's ns-decls. */
    let mut parnsdone: std::os::raw::c_int = 0;
    /* @ancestorsOnly should be set per option. */
    let mut ancestorsOnly: std::os::raw::c_int = 0 as std::os::raw::c_int;
    /*
    * Optimize string adoption for equal or none dicts.
    */
    if !sourceDoc.is_null() && (*sourceDoc).dict == (*destDoc).dict {
        adoptStr = 0 as std::os::raw::c_int
    } else { adoptStr = 1 as std::os::raw::c_int }
    /*
    * Get the ns-map from the context if available.
    */
    if !ctxt.is_null() { nsMap = (*ctxt).namespaceMap as xmlNsMapPtr }
    /*
    * Disable search for ns-decls in the parent-axis of the
    * desination element, if:
    * 1) there's no destination parent
    * 2) custom ns-reference handling is used
    */
    if destParent.is_null() ||
           !ctxt.is_null() && (*ctxt).getNsForNodeFunc.is_some() {
        parnsdone = 1 as std::os::raw::c_int
    } else { parnsdone = 0 as std::os::raw::c_int }
    cur = node;
    if !cur.is_null() &&
           (*cur).type_0 as std::os::raw::c_uint ==
               XML_NAMESPACE_DECL as std::os::raw::c_int as std::os::raw::c_uint {
        current_block = 4927006332000510053;
    } else { current_block = 17860125682698302841; }
    'c_43988:
        loop  {
            match current_block {
                4927006332000510053 => { ret = -(1 as std::os::raw::c_int); break ; }
                _ => {
                    if cur.is_null() { break ; }
                    /*
	* Paranoid source-doc sanity check.
	*/
                    if (*cur).doc != sourceDoc {
                        /*
	    * We'll assume XIncluded nodes if the doc differs.
	    * TODO: Do we need to reconciliate XIncluded nodes?
	    * This here skips XIncluded nodes and tries to handle
	    * broken sequences.
	    */
                        if (*cur).next.is_null() {
                            current_block = 2540843420819540057;
                        } else {
                            loop  {
                                cur = (*cur).next;
                                if (*cur).type_0 as std::os::raw::c_uint ==
                                       XML_XINCLUDE_END as std::os::raw::c_int as
                                           std::os::raw::c_uint ||
                                       (*cur).doc == (*node).doc {
                                    break ;
                                }
                                if (*cur).next.is_null() { break ; }
                            }
                            if (*cur).doc != (*node).doc {
                                current_block = 2540843420819540057;
                            } else { current_block = 11298138898191919651; }
                        }
                    } else { current_block = 11298138898191919651; }
                    match current_block {
                        11298138898191919651 => {
                            (*cur).doc = destDoc;
                            match (*cur).type_0 as std::os::raw::c_uint {
                                19 | 20 => {
                                    /*
		* TODO
		*/
                                    return -(1 as std::os::raw::c_int)
                                }
                                1 => {
                                    curElem = cur;
                                    depth += 1;
                                    /*
		* Namespace declarations.
		* - ns->href and ns->prefix are never in the dict, so
		*   we need not move the values over to the destination dict.
		* - Note that for custom handling of ns-references,
		*   the ns-decls need not be stored in the ns-map,
		*   since they won't be referenced by node->ns.
		*/
                                    if !(*cur).nsDef.is_null() &&
                                           (ctxt.is_null() ||
                                                (*ctxt).getNsForNodeFunc.is_none())
                                       {
                                        if parnsdone == 0 {
                                            /*
			* Gather @parent's in-scope ns-decls.
			*/
                                            if xmlDOMWrapNSNormGatherInScopeNs(&mut nsMap,
                                                                               destParent)
                                                   == -(1 as std::os::raw::c_int) {
                                                current_block =
                                                    4927006332000510053;
                                                continue ;
                                            }
                                            parnsdone = 1 as std::os::raw::c_int
                                        }
                                        ns = (*cur).nsDef;
                                        while !ns.is_null() {
                                            /*
			* NOTE: ns->prefix and ns->href are never in the dict.
			* XML_TREE_ADOPT_STR(ns->prefix)
			* XML_TREE_ADOPT_STR(ns->href)
			*/
			/*
			* Does it shadow any ns-decl?
			*/
                                            if !nsMap.is_null() &&
                                                   !(*nsMap).first.is_null() {
                                                mi = (*nsMap).first;
                                                while !mi.is_null() {
                                                    if (*mi).depth >=
                                                           -(1 as std::os::raw::c_int)
                                                           &&
                                                           (*mi).shadowDepth
                                                               ==
                                                               -(1 as
                                                                     std::os::raw::c_int)
                                                           &&
                                                           ((*ns).prefix ==
                                                                (*(*mi).newNs).prefix
                                                                ||
                                                                xmlStrEqual((*ns).prefix,
                                                                            (*(*mi).newNs).prefix)
                                                                    != 0) {
                                                        (*mi).shadowDepth =
                                                            depth
                                                    }
                                                    mi = (*mi).next
                                                }
                                            }
                                            /*
			* Push mapping.
			*/
                                            if xmlDOMWrapNsMapAddItem(&mut nsMap,
                                                                      -(1 as
                                                                            std::os::raw::c_int),
                                                                      ns, ns,
                                                                      depth).is_null()
                                               {
                                                current_block =
                                                    4927006332000510053;
                                                continue 'c_43988 ;
                                            }
                                            ns = (*ns).next
                                        }
                                    }
                                    /* Falls through. */
                                    current_block = 8802929608902314781;
                                }
                                2 => { current_block = 8802929608902314781; }
                                3 | 4 => {
                                    /*
		* This puts the content in the dest dict, only if
		* it was previously in the source dict.
		*/
                                    if adoptStr != 0 &&
                                           !(*cur).content.is_null() &&
                                           !sourceDoc.is_null() &&
                                           !(*sourceDoc).dict.is_null() &&
                                           xmlDictOwns((*sourceDoc).dict,
                                                       (*cur).content) != 0 {
                                        if !(*destDoc).dict.is_null() {
                                            (*cur).content =
                                                xmlDictLookup((*destDoc).dict,
                                                              (*cur).content,
                                                              -(1 as
                                                                    std::os::raw::c_int))
                                                    as *mut xmlChar
                                        } else {
                                            (*cur).content =
                                                xmlStrdup((*cur).content)
                                        }
                                    }
                                    current_block = 2540843420819540057;
                                }
                                5 => {
                                    /*
		* Remove reference to the entitity-node.
		*/
                                    (*cur).content = 0 as *mut xmlChar;
                                    (*cur).children = 0 as *mut _xmlNode;
                                    (*cur).last = 0 as *mut _xmlNode;
                                    if !(*destDoc).intSubset.is_null() ||
                                           !(*destDoc).extSubset.is_null() {
                                        let mut ent: xmlEntityPtr =
                                            0 as *mut xmlEntity;
                                        /*
		    * Assign new entity-node if available.
		    */
                                        ent =
                                            xmlGetDocEntity(destDoc as
                                                                *const xmlDoc,
                                                            (*cur).name);
                                        if !ent.is_null() {
                                            (*cur).content = (*ent).content;
                                            (*cur).children =
                                                ent as xmlNodePtr;
                                            (*cur).last = ent as xmlNodePtr
                                        }
                                    }
                                    current_block = 2540843420819540057;
                                }
                                7 => {
                                    if adoptStr != 0 && !(*cur).name.is_null()
                                       {
                                        if !(*destDoc).dict.is_null() {
                                            let mut old_0: *const xmlChar =
                                                (*cur).name;
                                            (*cur).name =
                                                xmlDictLookup((*destDoc).dict,
                                                              (*cur).name,
                                                              -(1 as
                                                                    std::os::raw::c_int));
                                            if sourceDoc.is_null() ||
                                                   (*sourceDoc).dict.is_null()
                                                   ||
                                                   xmlDictOwns((*sourceDoc).dict,
                                                               old_0) == 0 {
                                                xmlFree.expect("non-null function pointer")(old_0
                                                                                                as
                                                                                                *mut std::os::raw::c_char
                                                                                                as
                                                                                                *mut std::os::raw::c_void);
                                            }
                                        } else if !sourceDoc.is_null() &&
                                                      !(*sourceDoc).dict.is_null()
                                                      &&
                                                      xmlDictOwns((*sourceDoc).dict,
                                                                  (*cur).name)
                                                          != 0 {
                                            (*cur).name =
                                                xmlStrdup((*cur).name)
                                        }
                                    }
                                    if adoptStr != 0 &&
                                           !(*cur).content.is_null() &&
                                           !sourceDoc.is_null() &&
                                           !(*sourceDoc).dict.is_null() &&
                                           xmlDictOwns((*sourceDoc).dict,
                                                       (*cur).content) != 0 {
                                        if !(*destDoc).dict.is_null() {
                                            (*cur).content =
                                                xmlDictLookup((*destDoc).dict,
                                                              (*cur).content,
                                                              -(1 as
                                                                    std::os::raw::c_int))
                                                    as *mut xmlChar
                                        } else {
                                            (*cur).content =
                                                xmlStrdup((*cur).content)
                                        }
                                    }
                                    current_block = 11162283542402356847;
                                }
                                8 => { current_block = 11162283542402356847; }
                                _ => {
                                    current_block = 4927006332000510053;
                                    continue ;
                                }
                            }
                            match current_block {
                                2540843420819540057 => { }
                                _ => {
                                    match current_block {
                                        8802929608902314781 =>
                                        /* No namespace, no fun. */
                                        {
                                            if !(*cur).ns.is_null() {
                                                if parnsdone == 0 {
                                                    if xmlDOMWrapNSNormGatherInScopeNs(&mut nsMap,
                                                                                       destParent)
                                                           ==
                                                           -(1 as std::os::raw::c_int)
                                                       {
                                                        current_block =
                                                            4927006332000510053;
                                                        continue ;
                                                    }
                                                    parnsdone =
                                                        1 as std::os::raw::c_int
                                                }
                                                /*
		* Adopt ns-references.
		*/
                                                if !nsMap.is_null() &&
                                                       !(*nsMap).first.is_null()
                                                   {
                                                    /*
		    * Search for a mapping.
		    */
                                                    mi = (*nsMap).first;
                                                    loop  {
                                                        if mi.is_null() {
                                                            current_block =
                                                                2520131295878969859;
                                                            break ;
                                                        }
                                                        if (*mi).shadowDepth
                                                               ==
                                                               -(1 as
                                                                     std::os::raw::c_int)
                                                               &&
                                                               (*cur).ns ==
                                                                   (*mi).oldNs
                                                           {
                                                            (*cur).ns =
                                                                (*mi).newNs;
                                                            current_block =
                                                                7091955107207075669;
                                                            break ;
                                                        } else {
                                                            mi = (*mi).next
                                                        }
                                                    }
                                                } else {
                                                    current_block =
                                                        2520131295878969859;
                                                }
                                                match current_block {
                                                    7091955107207075669 => { }
                                                    _ =>
                                                    /*
		* No matching namespace in scope. We need a new one.
		*/
                                                    {
                                                        if !ctxt.is_null() &&
                                                               (*ctxt).getNsForNodeFunc.is_some()
                                                           {
                                                            /*
		    * User-defined behaviour.
		    */
                                                            ns =
                                                                (*ctxt).getNsForNodeFunc.expect("non-null function pointer")(ctxt,
                                                                                                                             cur,
                                                                                                                             (*(*cur).ns).href,
                                                                                                                             (*(*cur).ns).prefix);
                                                            /*
		    * Insert mapping if ns is available; it's the users fault
		    * if not.
		    */
                                                            if xmlDOMWrapNsMapAddItem(&mut nsMap,
                                                                                      -(1
                                                                                            as
                                                                                            std::os::raw::c_int),
                                                                                      (*cur).ns,
                                                                                      ns,
                                                                                      -(4
                                                                                            as
                                                                                            std::os::raw::c_int)).is_null()
                                                               {
                                                                current_block
                                                                    =
                                                                    4927006332000510053;
                                                                continue ;
                                                            }
                                                            (*cur).ns = ns
                                                        } else {
                                                            /*
		    * Aquire a normalized ns-decl and add it to the map.
		    */
                                                            if xmlDOMWrapNSNormAquireNormalizedNs(destDoc,
                                                                                                  (if !destParent.is_null()
                                                                                                      {
                                                                                                       curElem
                                                                                                   } else {
                                                                                                       0
                                                                                                           as
                                                                                                           xmlNodePtr
                                                                                                   }),
                                                                                                  (*cur).ns,
                                                                                                  &mut ns,
                                                                                                  &mut nsMap,
                                                                                                  depth,
                                                                                                  ancestorsOnly,
                                                                                                  (if (*cur).type_0
                                                                                                          as
                                                                                                          std::os::raw::c_uint
                                                                                                          ==
                                                                                                          XML_ATTRIBUTE_NODE
                                                                                                              as
                                                                                                              std::os::raw::c_int
                                                                                                              as
                                                                                                              std::os::raw::c_uint
                                                                                                      {
                                                                                                       1
                                                                                                           as
                                                                                                           std::os::raw::c_int
                                                                                                   } else {
                                                                                                       0
                                                                                                           as
                                                                                                           std::os::raw::c_int
                                                                                                   }))
                                                                   ==
                                                                   -(1 as
                                                                         std::os::raw::c_int)
                                                               {
                                                                current_block
                                                                    =
                                                                    4927006332000510053;
                                                                continue ;
                                                            }
                                                            (*cur).ns = ns
                                                        }
                                                    }
                                                }
                                            }
                                            /*
		* Further node properties.
		* TODO: Is this all?
		*/
                                            if adoptStr != 0 &&
                                                   !(*cur).name.is_null() {
                                                if !(*destDoc).dict.is_null()
                                                   {
                                                    let mut old:
                                                            *const xmlChar =
                                                        (*cur).name;
                                                    (*cur).name =
                                                        xmlDictLookup((*destDoc).dict,
                                                                      (*cur).name,
                                                                      -(1 as
                                                                            std::os::raw::c_int));
                                                    if sourceDoc.is_null() ||
                                                           (*sourceDoc).dict.is_null()
                                                           ||
                                                           xmlDictOwns((*sourceDoc).dict,
                                                                       old) ==
                                                               0 {
                                                        xmlFree.expect("non-null function pointer")(old
                                                                                                        as
                                                                                                        *mut std::os::raw::c_char
                                                                                                        as
                                                                                                        *mut std::os::raw::c_void);
                                                    }
                                                } else if !sourceDoc.is_null()
                                                              &&
                                                              !(*sourceDoc).dict.is_null()
                                                              &&
                                                              xmlDictOwns((*sourceDoc).dict,
                                                                          (*cur).name)
                                                                  != 0 {
                                                    (*cur).name =
                                                        xmlStrdup((*cur).name)
                                                }
                                            }
                                            if (*cur).type_0 as std::os::raw::c_uint
                                                   ==
                                                   XML_ELEMENT_NODE as
                                                       std::os::raw::c_int as
                                                       std::os::raw::c_uint {
                                                (*cur).psvi =
                                                    0 as *mut std::os::raw::c_void;
                                                (*cur).line =
                                                    0 as std::os::raw::c_int as
                                                        std::os::raw::c_ushort;
                                                (*cur).extra =
                                                    0 as std::os::raw::c_int as
                                                        std::os::raw::c_ushort;
                                                /*
		    * Walk attributes.
		    */
                                                if !(*cur).properties.is_null()
                                                   {
                                                    /*
			* Process first attribute node.
			*/
                                                    cur =
                                                        (*cur).properties as
                                                            xmlNodePtr;
                                                    current_block =
                                                        17860125682698302841;
                                                    continue ;
                                                }
                                            } else {
                                                /*
		    * Attributes.
		    */
                                                if !sourceDoc.is_null() &&
                                                       (*(cur as
                                                              xmlAttrPtr)).atype
                                                           as std::os::raw::c_uint ==
                                                           XML_ATTRIBUTE_ID as
                                                               std::os::raw::c_int as
                                                               std::os::raw::c_uint {
                                                    xmlRemoveID(sourceDoc,
                                                                cur as
                                                                    xmlAttrPtr);
                                                }
                                                (*(cur as xmlAttrPtr)).atype =
                                                    0 as xmlAttributeType;
                                                let ref mut fresh16 =
                                                    (*(cur as
                                                           xmlAttrPtr)).psvi;
                                                *fresh16 =
                                                    0 as *mut std::os::raw::c_void
                                            }
                                        }
                                        _ => { }
                                    }
                                    /*
	* Walk the tree.
	*/
                                    if !(*cur).children.is_null() {
                                        cur = (*cur).children;
                                        current_block = 17860125682698302841;
                                        continue ;
                                    }
                                }
                            }
                        }
                        _ => { }
                    }
                    loop  {
                        if cur == node { break 'c_43988 ; }
                        if (*cur).type_0 as std::os::raw::c_uint ==
                               XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint
                               ||
                               (*cur).type_0 as std::os::raw::c_uint ==
                                   XML_XINCLUDE_START as std::os::raw::c_int as
                                       std::os::raw::c_uint ||
                               (*cur).type_0 as std::os::raw::c_uint ==
                                   XML_XINCLUDE_END as std::os::raw::c_int as
                                       std::os::raw::c_uint {
                            /*
	    * TODO: Do we expect nsDefs on XML_XINCLUDE_START?
	    */
                            if !nsMap.is_null() && !(*nsMap).first.is_null() {
                                /*
		* Pop mappings.
		*/
                                while !(*nsMap).last.is_null() &&
                                          (*(*nsMap).last).depth >= depth {
                                    mi = (*nsMap).last;
                                    (*nsMap).last = (*mi).prev;
                                    if (*nsMap).last.is_null() {
                                        (*nsMap).first = 0 as xmlNsMapItemPtr
                                    } else {
                                        (*(*nsMap).last).next =
                                            0 as xmlNsMapItemPtr
                                    }
                                    (*mi).next = (*nsMap).pool;
                                    (*nsMap).pool = mi
                                }
                                /*
		* Unshadow.
		*/
                                mi = (*nsMap).first;
                                while !mi.is_null() {
                                    if (*mi).shadowDepth >= depth {
                                        (*mi).shadowDepth =
                                            -(1 as std::os::raw::c_int)
                                    }
                                    mi = (*mi).next
                                }
                            }
                            depth -= 1
                        }
                        if !(*cur).next.is_null() {
                            cur = (*cur).next;
                            current_block = 17860125682698302841;
                            break ;
                        } else if (*cur).type_0 as std::os::raw::c_uint ==
                                      XML_ATTRIBUTE_NODE as std::os::raw::c_int as
                                          std::os::raw::c_uint &&
                                      !(*(*cur).parent).children.is_null() {
                            cur = (*(*cur).parent).children;
                            current_block = 17860125682698302841;
                            break ;
                        } else { cur = (*cur).parent }
                    }
                }
            }
        }
    /*
    * Cleanup.
    */
    if !nsMap.is_null() {
        if !ctxt.is_null() &&
               (*ctxt).namespaceMap == nsMap as *mut std::os::raw::c_void {
            /*
	    * Just cleanup the map but don't free.
	    */
            if !(*nsMap).first.is_null() {
                if !(*nsMap).pool.is_null() {
                    (*(*nsMap).last).next = (*nsMap).pool
                }
                (*nsMap).pool = (*nsMap).first;
                (*nsMap).first = 0 as xmlNsMapItemPtr
            }
        } else { xmlDOMWrapNsMapFree(nsMap); }
    }
    return ret;
}
/*
* xmlDOMWrapCloneNode:
* @ctxt: the optional context for custom processing
* @sourceDoc: the optional sourceDoc
* @node: the node to start with
* @resNode: the clone of the given @node
* @destDoc: the destination doc
* @destParent: the optional new parent of @node in @destDoc
* @deep: descend into child if set
* @options: option flags
*
* References of out-of scope ns-decls are remapped to point to @destDoc:
* 1) If @destParent is given, then nsDef entries on element-nodes are used
* 2) If *no* @destParent is given, then @destDoc->oldNs entries are used.
*    This is the case when you don't know already where the cloned branch
*    will be added to.
*
* If @destParent is given, it ensures that the tree is namespace
* wellformed by creating additional ns-decls where needed.
* Note that, since prefixes of already existent ns-decls can be
* shadowed by this process, it could break QNames in attribute
* values or element content.
* TODO:
*   1) What to do with XInclude? Currently this returns an error for XInclude.
*
* Returns 0 if the operation succeeded,
*         1 if a node of unsupported (or not yet supported) type was given,
*         -1 on API/internal errors.
*/
#[no_mangle]
pub unsafe extern "C" fn xmlDOMWrapCloneNode(mut ctxt: xmlDOMWrapCtxtPtr,
                                             mut sourceDoc: xmlDocPtr,
                                             mut node: xmlNodePtr,
                                             mut resNode: *mut xmlNodePtr,
                                             mut destDoc: xmlDocPtr,
                                             mut destParent: xmlNodePtr,
                                             mut deep: std::os::raw::c_int,
                                             mut options: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut current_block: u64;
    let mut ret: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    let mut curElem: xmlNodePtr = 0 as xmlNodePtr;
    let mut nsMap: xmlNsMapPtr = 0 as xmlNsMapPtr;
    let mut mi: xmlNsMapItemPtr = 0 as *mut xmlNsMapItem;
    let mut ns: xmlNsPtr = 0 as *mut xmlNs;
    let mut depth: std::os::raw::c_int = -(1 as std::os::raw::c_int);
    /* int adoptStr = 1; */
    /* gather @parent's ns-decls. */
    let mut parnsdone: std::os::raw::c_int = 0 as std::os::raw::c_int;
    /*
    * @ancestorsOnly:
    * TODO: @ancestorsOnly should be set per option.
    *
    */
    let mut ancestorsOnly: std::os::raw::c_int =
        0 as std::os::raw::c_int; /* The destination dict */
    let mut resultClone: xmlNodePtr = 0 as xmlNodePtr;
    let mut clone: xmlNodePtr = 0 as xmlNodePtr;
    let mut parentClone: xmlNodePtr = 0 as xmlNodePtr;
    let mut prevClone: xmlNodePtr = 0 as xmlNodePtr;
    let mut cloneNs: xmlNsPtr = 0 as xmlNsPtr;
    let mut cloneNsDefSlot: *mut xmlNsPtr = 0 as *mut xmlNsPtr;
    let mut dict: xmlDictPtr = 0 as *mut xmlDict;
    if node.is_null() || resNode.is_null() || destDoc.is_null() {
        return -(1 as std::os::raw::c_int)
    }
    /*
    * TODO: Initially we support only element-nodes.
    */
    if (*node).type_0 as std::os::raw::c_uint !=
           XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        return 1 as std::os::raw::c_int
    }
    /*
    * Check node->doc sanity.
    */
    if !(*node).doc.is_null() && !sourceDoc.is_null() &&
           (*node).doc != sourceDoc {
        /*
	* Might be an XIncluded node.
	*/
        return -(1 as std::os::raw::c_int)
    }
    if sourceDoc.is_null() { sourceDoc = (*node).doc }
    if sourceDoc.is_null() { return -(1 as std::os::raw::c_int) }
    dict = (*destDoc).dict;
    /*
    * Reuse the namespace map of the context.
    */
    if !ctxt.is_null() { nsMap = (*ctxt).namespaceMap as xmlNsMapPtr }
    *resNode = 0 as xmlNodePtr;
    cur = node;
    if !cur.is_null() &&
           (*cur).type_0 as std::os::raw::c_uint ==
               XML_NAMESPACE_DECL as std::os::raw::c_int as std::os::raw::c_uint {
        return -(1 as std::os::raw::c_int)
    }
    's_109:
        loop  {
            if cur.is_null() { current_block = 7432106818040070753; break ; }
            if (*cur).doc != sourceDoc {
                /*
	    * We'll assume XIncluded nodes if the doc differs.
	    * TODO: Do we need to reconciliate XIncluded nodes?
	    * TODO: This here returns -1 in this case.
	    */
                current_block = 14407885047339588652;
                break ;
            } else {
                /*
	* Create a new node.
	*/
                match (*cur).type_0 as std::os::raw::c_uint {
                    19 | 20 => {
                        /*
		* TODO: What to do with XInclude?
		*/
                        current_block = 14407885047339588652;
                        break ;
                    }
                    1 | 3 | 4 | 8 | 7 | 11 | 5 | 6 => {
                        /*
		* Nodes of xmlNode structure.
		*/
                        clone =
                            xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlNode>()
                                                                              as
                                                                              std::os::raw::c_ulong)
                                as xmlNodePtr;
                        if clone.is_null() {
                            xmlTreeErrMemory(b"xmlDOMWrapCloneNode(): allocating a node\x00"
                                                 as *const u8 as
                                                 *const std::os::raw::c_char);
                            current_block = 14407885047339588652;
                            break ;
                        } else {
                            memset(clone as *mut std::os::raw::c_void,
                                   0 as std::os::raw::c_int,
                                   ::std::mem::size_of::<xmlNode>() as
                                       std::os::raw::c_ulong);
                            /*
		* Set hierachical links.
		*/
                            if !resultClone.is_null() {
                                (*clone).parent = parentClone;
                                if !prevClone.is_null() {
                                    (*prevClone).next = clone;
                                    (*clone).prev = prevClone
                                } else { (*parentClone).children = clone }
                            } else { resultClone = clone }
                        }
                    }
                    2 => {
                        /*
		* Attributes (xmlAttr).
		*/
                        clone =
                            xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlAttr>()
                                                                              as
                                                                              std::os::raw::c_ulong)
                                as xmlNodePtr;
                        if clone.is_null() {
                            xmlTreeErrMemory(b"xmlDOMWrapCloneNode(): allocating an attr-node\x00"
                                                 as *const u8 as
                                                 *const std::os::raw::c_char);
                            current_block = 14407885047339588652;
                            break ;
                        } else {
                            memset(clone as *mut std::os::raw::c_void,
                                   0 as std::os::raw::c_int,
                                   ::std::mem::size_of::<xmlAttr>() as
                                       std::os::raw::c_ulong);
                            /*
		* Set hierachical links.
		* TODO: Change this to add to the end of attributes.
		*/
                            if !resultClone.is_null() {
                                (*clone).parent = parentClone;
                                if !prevClone.is_null() {
                                    (*prevClone).next = clone;
                                    (*clone).prev = prevClone
                                } else {
                                    (*parentClone).properties =
                                        clone as xmlAttrPtr
                                }
                            } else { resultClone = clone }
                        }
                    }
                    _ => { current_block = 14407885047339588652; break ; }
                }
                (*clone).type_0 = (*cur).type_0;
                (*clone).doc = destDoc;
                /*
	* Clone the name of the node if any.
	*/
                if (*cur).name == xmlStringText.as_ptr() {
                    (*clone).name = xmlStringText.as_ptr()
                } else if (*cur).name == xmlStringTextNoenc.as_ptr() {
                    /*
	    * NOTE: Although xmlStringTextNoenc is never assigned to a node
	    *   in tree.c, it might be set in Libxslt via
	    *   "xsl:disable-output-escaping".
	    */
                    (*clone).name = xmlStringTextNoenc.as_ptr()
                } else if (*cur).name == xmlStringComment.as_ptr() {
                    (*clone).name = xmlStringComment.as_ptr()
                } else if !(*cur).name.is_null() {
                    if !(*cur).name.is_null() {
                        if !dict.is_null() {
                            if xmlDictOwns(dict, (*cur).name) != 0 {
                                (*clone).name = (*cur).name
                            } else {
                                (*clone).name =
                                    xmlDictLookup(dict, (*cur).name,
                                                  -(1 as std::os::raw::c_int))
                            }
                        } else {
                            (*clone).name =
                                xmlStrdup((*cur).name) as *const xmlChar
                        }
                    }
                }
                match (*cur).type_0 as std::os::raw::c_uint {
                    19 | 20 => {
                        /*
		* TODO
		*/
                        return -(1 as std::os::raw::c_int)
                    }
                    1 => {
                        curElem = cur;
                        depth += 1;
                        /*
		* Namespace declarations.
		*/
                        if !(*cur).nsDef.is_null() {
                            if parnsdone == 0 {
                                if !destParent.is_null() && ctxt.is_null() {
                                    /*
			    * Gather @parent's in-scope ns-decls.
			    */
                                    if xmlDOMWrapNSNormGatherInScopeNs(&mut nsMap,
                                                                       destParent)
                                           == -(1 as std::os::raw::c_int) {
                                        current_block = 14407885047339588652;
                                        break ;
                                    }
                                }
                                parnsdone = 1 as std::os::raw::c_int
                            }
                            /*
		    * Clone namespace declarations.
		    */
                            cloneNsDefSlot = &mut (*clone).nsDef;
                            ns = (*cur).nsDef;
                            while !ns.is_null() {
                                /*
			* Create a new xmlNs.
			*/
                                cloneNs =
                                    xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlNs>()
                                                                                      as
                                                                                      std::os::raw::c_ulong)
                                        as xmlNsPtr;
                                if cloneNs.is_null() {
                                    xmlTreeErrMemory(b"xmlDOMWrapCloneNode(): allocating namespace\x00"
                                                         as *const u8 as
                                                         *const std::os::raw::c_char);
                                    return -(1 as std::os::raw::c_int)
                                }
                                memset(cloneNs as *mut std::os::raw::c_void,
                                       0 as std::os::raw::c_int,
                                       ::std::mem::size_of::<xmlNs>() as
                                           std::os::raw::c_ulong);
                                (*cloneNs).type_0 = XML_NAMESPACE_DECL;
                                if !(*ns).href.is_null() {
                                    (*cloneNs).href = xmlStrdup((*ns).href)
                                }
                                if !(*ns).prefix.is_null() {
                                    (*cloneNs).prefix =
                                        xmlStrdup((*ns).prefix)
                                }
                                *cloneNsDefSlot = cloneNs;
                                cloneNsDefSlot = &mut (*cloneNs).next;
                                /*
			* Note that for custom handling of ns-references,
			* the ns-decls need not be stored in the ns-map,
			* since they won't be referenced by node->ns.
			*/
                                if ctxt.is_null() ||
                                       (*ctxt).getNsForNodeFunc.is_none() {
                                    /*
			    * Does it shadow any ns-decl?
			    */
                                    if !nsMap.is_null() &&
                                           !(*nsMap).first.is_null() {
                                        mi = (*nsMap).first;
                                        while !mi.is_null() {
                                            if (*mi).depth >=
                                                   -(1 as std::os::raw::c_int) &&
                                                   (*mi).shadowDepth ==
                                                       -(1 as std::os::raw::c_int) &&
                                                   ((*ns).prefix ==
                                                        (*(*mi).newNs).prefix
                                                        ||
                                                        xmlStrEqual((*ns).prefix,
                                                                    (*(*mi).newNs).prefix)
                                                            != 0) {
                                                /*
					* Mark as shadowed at the current
					* depth.
					*/
                                                (*mi).shadowDepth = depth
                                            }
                                            mi = (*mi).next
                                        }
                                    }
                                    /*
			    * Push mapping.
			    */
                                    if xmlDOMWrapNsMapAddItem(&mut nsMap,
                                                              -(1 as
                                                                    std::os::raw::c_int),
                                                              ns, cloneNs,
                                                              depth).is_null()
                                       {
                                        current_block = 14407885047339588652;
                                        break 's_109 ;
                                    }
                                }
                                ns = (*ns).next
                            }
                        }
                        /* cur->ns will be processed further down. */
                        current_block = 2544535129495155983;
                    }
                    2 => { current_block = 2544535129495155983; }
                    3 | 4 => {
                        /*
		* Note that this will also cover the values of attributes.
		*/
                        if !(*cur).content.is_null() {
                            if !dict.is_null() {
                                if xmlDictOwns(dict,
                                               (*cur).content as
                                                   *const xmlChar) != 0 {
                                    (*clone).content = (*cur).content
                                } else {
                                    (*clone).content =
                                        xmlDictLookup(dict,
                                                      (*cur).content as
                                                          *const xmlChar,
                                                      -(1 as std::os::raw::c_int)) as
                                            *mut xmlChar
                                }
                            } else {
                                (*clone).content =
                                    xmlStrdup((*cur).content as
                                                  *const xmlChar)
                            }
                        }
                        current_block = 12364176829744102857;
                    }
                    6 => {
                        /* TODO: What to do here? */
                        current_block = 12364176829744102857;
                    }
                    5 => {
                        if sourceDoc != destDoc {
                            if !(*destDoc).intSubset.is_null() ||
                                   !(*destDoc).extSubset.is_null() {
                                let mut ent: xmlEntityPtr =
                                    0 as *mut xmlEntity;
                                /*
			* Different doc: Assign new entity-node if available.
			*/
                                ent =
                                    xmlGetDocEntity(destDoc as *const xmlDoc,
                                                    (*cur).name);
                                if !ent.is_null() {
                                    (*clone).content = (*ent).content;
                                    (*clone).children = ent as xmlNodePtr;
                                    (*clone).last = ent as xmlNodePtr
                                }
                            }
                        } else {
                            /*
		    * Same doc: Use the current node's entity declaration
		    * and value.
		    */
                            (*clone).content = (*cur).content;
                            (*clone).children = (*cur).children;
                            (*clone).last = (*cur).last
                        }
                        current_block = 12364176829744102857;
                    }
                    7 => {
                        if !(*cur).content.is_null() {
                            if !dict.is_null() {
                                if xmlDictOwns(dict,
                                               (*cur).content as
                                                   *const xmlChar) != 0 {
                                    (*clone).content = (*cur).content
                                } else {
                                    (*clone).content =
                                        xmlDictLookup(dict,
                                                      (*cur).content as
                                                          *const xmlChar,
                                                      -(1 as std::os::raw::c_int)) as
                                            *mut xmlChar
                                }
                            } else {
                                (*clone).content =
                                    xmlStrdup((*cur).content as
                                                  *const xmlChar)
                            }
                        }
                        current_block = 12364176829744102857;
                    }
                    8 => {
                        if !(*cur).content.is_null() {
                            if !dict.is_null() {
                                if xmlDictOwns(dict,
                                               (*cur).content as
                                                   *const xmlChar) != 0 {
                                    (*clone).content = (*cur).content
                                } else {
                                    (*clone).content =
                                        xmlDictLookup(dict,
                                                      (*cur).content as
                                                          *const xmlChar,
                                                      -(1 as std::os::raw::c_int)) as
                                            *mut xmlChar
                                }
                            } else {
                                (*clone).content =
                                    xmlStrdup((*cur).content as
                                                  *const xmlChar)
                            }
                        }
                        current_block = 12364176829744102857;
                    }
                    _ => { current_block = 14407885047339588652; break ; }
                }
                match current_block {
                    2544535129495155983 =>
                    /* IDs will be processed further down. */
		/* cur->ns will be processed further down. */
                    {
                        if !(*cur).ns.is_null() {
                            /* handle_ns_reference: */
	/*
	** The following will take care of references to ns-decls ********
	** and is intended only for element- and attribute-nodes.
	**
	*/
                            if parnsdone == 0 {
                                if !destParent.is_null() && ctxt.is_null() {
                                    if xmlDOMWrapNSNormGatherInScopeNs(&mut nsMap,
                                                                       destParent)
                                           == -(1 as std::os::raw::c_int) {
                                        current_block = 14407885047339588652;
                                        break ;
                                    }
                                }
                                parnsdone = 1 as std::os::raw::c_int
                            }
                            /*
	* Adopt ns-references.
	*/
                            if !nsMap.is_null() && !(*nsMap).first.is_null() {
                                /*
	    * Search for a mapping.
	    */
                                mi = (*nsMap).first;
                                loop  {
                                    if mi.is_null() {
                                        current_block = 9657096306311191688;
                                        break ;
                                    }
                                    if (*mi).shadowDepth ==
                                           -(1 as std::os::raw::c_int) &&
                                           (*cur).ns == (*mi).oldNs {
                                        /*
		    * This is the nice case: a mapping was found.
		    */
                                        (*clone).ns = (*mi).newNs;
                                        current_block = 6235162343018973695;
                                        break ;
                                    } else { mi = (*mi).next }
                                }
                            } else { current_block = 9657096306311191688; }
                            match current_block {
                                6235162343018973695 => { }
                                _ =>
                                /*
	* No matching namespace in scope. We need a new one.
	*/
                                {
                                    if !ctxt.is_null() &&
                                           (*ctxt).getNsForNodeFunc.is_some()
                                       {
                                        /*
	    * User-defined behaviour.
	    */
                                        ns =
                                            (*ctxt).getNsForNodeFunc.expect("non-null function pointer")(ctxt,
                                                                                                         cur,
                                                                                                         (*(*cur).ns).href,
                                                                                                         (*(*cur).ns).prefix);
                                        /*
	    * Add user's mapping.
	    */
                                        if xmlDOMWrapNsMapAddItem(&mut nsMap,
                                                                  -(1 as
                                                                        std::os::raw::c_int),
                                                                  (*cur).ns,
                                                                  ns,
                                                                  -(4 as
                                                                        std::os::raw::c_int)).is_null()
                                           {
                                            current_block =
                                                14407885047339588652;
                                            break ;
                                        }
                                        (*clone).ns = ns
                                    } else {
                                        /*
	    * Aquire a normalized ns-decl and add it to the map.
	    */
                                        if xmlDOMWrapNSNormAquireNormalizedNs(destDoc,
                                                                              (if !destParent.is_null()
                                                                                  {
                                                                                   curElem
                                                                               } else {
                                                                                   0
                                                                                       as
                                                                                       xmlNodePtr
                                                                               }),
                                                                              (*cur).ns,
                                                                              &mut ns,
                                                                              &mut nsMap,
                                                                              depth,
                                                                              ancestorsOnly,
                                                                              (if (*cur).type_0
                                                                                      as
                                                                                      std::os::raw::c_uint
                                                                                      ==
                                                                                      XML_ATTRIBUTE_NODE
                                                                                          as
                                                                                          std::os::raw::c_int
                                                                                          as
                                                                                          std::os::raw::c_uint
                                                                                  {
                                                                                   1
                                                                                       as
                                                                                       std::os::raw::c_int
                                                                               } else {
                                                                                   0
                                                                                       as
                                                                                       std::os::raw::c_int
                                                                               }))
                                               == -(1 as std::os::raw::c_int) {
                                            current_block =
                                                14407885047339588652;
                                            break ;
                                        }
                                        (*clone).ns = ns
                                    }
                                }
                            }
                        }
                        /*
	* Some post-processing.
	*
	* Handle ID attributes.
	*/
                        if (*clone).type_0 as std::os::raw::c_uint ==
                               XML_ATTRIBUTE_NODE as std::os::raw::c_int as
                                   std::os::raw::c_uint && !(*clone).parent.is_null()
                           {
                            if xmlIsID(destDoc, (*clone).parent,
                                       clone as xmlAttrPtr) != 0 {
                                let mut idVal: *mut xmlChar =
                                    0 as *mut xmlChar;
                                idVal =
                                    xmlNodeListGetString((*cur).doc,
                                                         (*cur).children,
                                                         1 as std::os::raw::c_int);
                                if !idVal.is_null() {
                                    if xmlAddID(0 as xmlValidCtxtPtr, destDoc,
                                                idVal,
                                                cur as xmlAttrPtr).is_null() {
                                        /* TODO: error message. */
                                        xmlFree.expect("non-null function pointer")(idVal
                                                                                        as
                                                                                        *mut std::os::raw::c_void);
                                        current_block = 14407885047339588652;
                                        break ;
                                    } else {
                                        xmlFree.expect("non-null function pointer")(idVal
                                                                                        as
                                                                                        *mut std::os::raw::c_void);
                                    }
                                }
                            }
                        }
                        /*
	**
	** The following will traverse the tree **************************
	**
	*
	* Walk the element's attributes before descending into child-nodes.
	*/
                        if (*cur).type_0 as std::os::raw::c_uint ==
                               XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint
                               && !(*cur).properties.is_null() {
                            prevClone = 0 as xmlNodePtr;
                            parentClone = clone;
                            cur = (*cur).properties as xmlNodePtr;
                            continue ;
                        } else { current_block = 6637781077342651821; }
                    }
                    _ => { }
                }
                loop  {
                    match current_block {
                        12364176829744102857 =>
                        /*
	* At this point we are done with the node, its content
	* and an element-nodes's attribute-nodes.
	*/
                        {
                            if cur == node {
                                current_block = 7432106818040070753;
                                break 's_109 ;
                            }
                            if (*cur).type_0 as std::os::raw::c_uint ==
                                   XML_ELEMENT_NODE as std::os::raw::c_int as
                                       std::os::raw::c_uint ||
                                   (*cur).type_0 as std::os::raw::c_uint ==
                                       XML_XINCLUDE_START as std::os::raw::c_int as
                                           std::os::raw::c_uint ||
                                   (*cur).type_0 as std::os::raw::c_uint ==
                                       XML_XINCLUDE_END as std::os::raw::c_int as
                                           std::os::raw::c_uint {
                                /*
	    * TODO: Do we expect nsDefs on XML_XINCLUDE_START?
	    */
                                if !nsMap.is_null() &&
                                       !(*nsMap).first.is_null() {
                                    /*
		* Pop mappings.
		*/
                                    while !(*nsMap).last.is_null() &&
                                              (*(*nsMap).last).depth >= depth
                                          {
                                        mi = (*nsMap).last;
                                        (*nsMap).last = (*mi).prev;
                                        if (*nsMap).last.is_null() {
                                            (*nsMap).first =
                                                0 as xmlNsMapItemPtr
                                        } else {
                                            (*(*nsMap).last).next =
                                                0 as xmlNsMapItemPtr
                                        }
                                        (*mi).next = (*nsMap).pool;
                                        (*nsMap).pool = mi
                                    }
                                    /*
		* Unshadow.
		*/
                                    mi = (*nsMap).first;
                                    while !mi.is_null() {
                                        if (*mi).shadowDepth >= depth {
                                            (*mi).shadowDepth =
                                                -(1 as std::os::raw::c_int)
                                        }
                                        mi = (*mi).next
                                    }
                                }
                                depth -= 1
                            }
                            if !(*cur).next.is_null() {
                                prevClone = clone;
                                cur = (*cur).next;
                                break ;
                            } else if (*cur).type_0 as std::os::raw::c_uint !=
                                          XML_ATTRIBUTE_NODE as std::os::raw::c_int as
                                              std::os::raw::c_uint {
                                /*
	    * Set clone->last.
	    */
                                if !(*clone).parent.is_null() {
                                    (*(*clone).parent).last = clone
                                }
                                clone = (*clone).parent;
                                if !clone.is_null() {
                                    parentClone = (*clone).parent
                                }
                                /*
	    * Process parent --> next;
	    */
                                cur = (*cur).parent;
                                current_block = 12364176829744102857;
                            } else {
                                /* This is for attributes only. */
                                clone = (*clone).parent;
                                parentClone = (*clone).parent;
                                /*
	    * Process parent-element --> children.
	    */
                                cur = (*cur).parent;
                                current_block = 6637781077342651821;
                            }
                        }
                        _ =>
                        /*
	* Descend into child-nodes.
	*/
                        {
                            if (*cur).children.is_null() {
                                current_block = 12364176829744102857;
                                continue ;
                            }
                            if !(deep != 0 ||
                                     (*cur).type_0 as std::os::raw::c_uint ==
                                         XML_ATTRIBUTE_NODE as std::os::raw::c_int as
                                             std::os::raw::c_uint) {
                                current_block = 12364176829744102857;
                                continue ;
                            }
                            prevClone = 0 as xmlNodePtr;
                            parentClone = clone;
                            cur = (*cur).children;
                            break ;
                        }
                    }
                }
            }
        }
    match current_block {
        14407885047339588652 =>
        /*
		* TODO QUESTION: Any other nodes expected?
		*/
        {
            ret = -(1 as std::os::raw::c_int)
        }
        _ => { }
    }
    /*
    * Cleanup.
    */
    if !nsMap.is_null() {
        if !ctxt.is_null() &&
               (*ctxt).namespaceMap == nsMap as *mut std::os::raw::c_void {
            /*
	    * Just cleanup the map but don't free.
	    */
            if !(*nsMap).first.is_null() {
                if !(*nsMap).pool.is_null() {
                    (*(*nsMap).last).next = (*nsMap).pool
                }
                (*nsMap).pool = (*nsMap).first;
                (*nsMap).first = 0 as xmlNsMapItemPtr
            }
        } else { xmlDOMWrapNsMapFree(nsMap); }
    }
    /*
    * TODO: Should we try a cleanup of the cloned node in case of a
    * fatal error?
    */
    *resNode = resultClone;
    return ret;
}
/*
* xmlDOMWrapAdoptAttr:
* @ctxt: the optional context for custom processing
* @sourceDoc: the optional source document of attr
* @attr: the attribute-node to be adopted
* @destDoc: the destination doc for adoption
* @destParent: the optional new parent of @attr in @destDoc
* @options: option flags
*
* @attr is adopted by @destDoc.
* Ensures that ns-references point to @destDoc: either to
* elements->nsDef entries if @destParent is given, or to
* @destDoc->oldNs otherwise.
*
* Returns 0 if succeeded, -1 otherwise and on API/internal errors.
*/
unsafe extern "C" fn xmlDOMWrapAdoptAttr(mut ctxt: xmlDOMWrapCtxtPtr,
                                         mut sourceDoc: xmlDocPtr,
                                         mut attr: xmlAttrPtr,
                                         mut destDoc: xmlDocPtr,
                                         mut destParent: xmlNodePtr,
                                         mut options: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut current_block: u64;
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    let mut adoptStr: std::os::raw::c_int = 1 as std::os::raw::c_int;
    if attr.is_null() || destDoc.is_null() { return -(1 as std::os::raw::c_int) }
    (*attr).doc = destDoc;
    if !(*attr).ns.is_null() {
        let mut ns: xmlNsPtr = 0 as xmlNsPtr;
        !ctxt.is_null();
        /* XML Namespace. */
        if !(*(*attr).ns).prefix.is_null() &&
               *(*(*attr).ns).prefix.offset(0 as std::os::raw::c_int as isize) as
                   std::os::raw::c_int == 'x' as i32 &&
               *(*(*attr).ns).prefix.offset(1 as std::os::raw::c_int as isize) as
                   std::os::raw::c_int == 'm' as i32 &&
               *(*(*attr).ns).prefix.offset(2 as std::os::raw::c_int as isize) as
                   std::os::raw::c_int == 'l' as i32 &&
               *(*(*attr).ns).prefix.offset(3 as std::os::raw::c_int as isize) as
                   std::os::raw::c_int == 0 as std::os::raw::c_int {
            ns = xmlTreeEnsureXMLDecl(destDoc);
            current_block = 5143058163439228106;
        } else if destParent.is_null() {
            /*
	    * Store in @destDoc->oldNs.
	    */
            ns =
                xmlDOMWrapStoreNs(destDoc, (*(*attr).ns).href,
                                  (*(*attr).ns).prefix);
            current_block = 5143058163439228106;
        } else if xmlSearchNsByNamespaceStrict(destDoc, destParent,
                                               (*(*attr).ns).href, &mut ns,
                                               1 as std::os::raw::c_int) ==
                      -(1 as std::os::raw::c_int) {
            current_block = 17594073304962145041;
        } else {
            if ns.is_null() {
                ns =
                    xmlDOMWrapNSNormDeclareNsForced(destDoc, destParent,
                                                    (*(*attr).ns).href,
                                                    (*(*attr).ns).prefix,
                                                    1 as std::os::raw::c_int)
            }
            current_block = 5143058163439228106;
        }
        match current_block {
            17594073304962145041 => { }
            _ => {
                if ns.is_null() {
                    current_block = 17594073304962145041;
                } else {
                    (*attr).ns = ns;
                    current_block = 12124785117276362961;
                }
            }
        }
    } else { current_block = 12124785117276362961; }
    match current_block {
        12124785117276362961 => {
            if adoptStr != 0 && !(*attr).name.is_null() {
                if !(*destDoc).dict.is_null() {
                    let mut old: *const xmlChar = (*attr).name;
                    (*attr).name =
                        xmlDictLookup((*destDoc).dict, (*attr).name,
                                      -(1 as std::os::raw::c_int));
                    if sourceDoc.is_null() || (*sourceDoc).dict.is_null() ||
                           xmlDictOwns((*sourceDoc).dict, old) == 0 {
                        xmlFree.expect("non-null function pointer")(old as
                                                                        *mut std::os::raw::c_char
                                                                        as
                                                                        *mut std::os::raw::c_void);
                    }
                } else if !sourceDoc.is_null() && !(*sourceDoc).dict.is_null()
                              &&
                              xmlDictOwns((*sourceDoc).dict, (*attr).name) !=
                                  0 {
                    (*attr).name = xmlStrdup((*attr).name)
                }
            }
            (*attr).atype = 0 as xmlAttributeType;
            (*attr).psvi = 0 as *mut std::os::raw::c_void;
            /*
	    * Declare on @destParent.
	    */
            /*
    * Walk content.
    */
            if (*attr).children.is_null() { return 0 as std::os::raw::c_int }
            cur = (*attr).children;
            if !(!cur.is_null() &&
                     (*cur).type_0 as std::os::raw::c_uint ==
                         XML_NAMESPACE_DECL as std::os::raw::c_int as std::os::raw::c_uint) {
                's_181:
                    while !cur.is_null() {
                        (*cur).doc = destDoc;
                        match (*cur).type_0 as std::os::raw::c_uint {
                            3 | 4 => {
                                if adoptStr != 0 && !(*cur).content.is_null()
                                       && !sourceDoc.is_null() &&
                                       !(*sourceDoc).dict.is_null() &&
                                       xmlDictOwns((*sourceDoc).dict,
                                                   (*cur).content) != 0 {
                                    if !(*destDoc).dict.is_null() {
                                        (*cur).content =
                                            xmlDictLookup((*destDoc).dict,
                                                          (*cur).content,
                                                          -(1 as std::os::raw::c_int))
                                                as *mut xmlChar
                                    } else {
                                        (*cur).content =
                                            xmlStrdup((*cur).content)
                                    }
                                }
                            }
                            5 => {
                                /*
		* Remove reference to the entitity-node.
		*/
                                (*cur).content = 0 as *mut xmlChar;
                                (*cur).children = 0 as *mut _xmlNode;
                                (*cur).last = 0 as *mut _xmlNode;
                                if !(*destDoc).intSubset.is_null() ||
                                       !(*destDoc).extSubset.is_null() {
                                    let mut ent: xmlEntityPtr =
                                        0 as *mut xmlEntity;
                                    /*
		    * Assign new entity-node if available.
		    */
                                    ent =
                                        xmlGetDocEntity(destDoc as
                                                            *const xmlDoc,
                                                        (*cur).name);
                                    if !ent.is_null() {
                                        (*cur).content = (*ent).content;
                                        (*cur).children = ent as xmlNodePtr;
                                        (*cur).last = ent as xmlNodePtr
                                    }
                                }
                            }
                            _ => { }
                        }
                        if !(*cur).children.is_null() {
                            cur = (*cur).children
                        } else {
                            loop  {
                                if cur == attr as xmlNodePtr {
                                    break 's_181 ;
                                }
                                if !(*cur).next.is_null() { break ; }
                                cur = (*cur).parent
                            }
                            cur = (*cur).next
                        }
                    }
                return 0 as std::os::raw::c_int
            }
        }
        _ => { }
    }
    return -(1 as std::os::raw::c_int);
}
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
/*
 * Handling Buffers, the old ones see @xmlBuf for the new ones.
 */
/*
 * Creating/freeing new structures.
 */
/* LIBXML_LEGACY_ENABLED */
/* LIBXML_TREE_ENABLED */
/* defined(LIBXML_TREE_ENABLED) || defined(LIBXML_SCHEMAS_ENABLED) */
/*
 * Creating new nodes.
 */
/* LIBXML_TREE_ENABLED */
/*
 * Navigating.
 */
/* defined(LIBXML_TREE_ENABLED) || defined(LIBXML_DEBUG_ENABLED) */
/*
 * Changing the structure.
 */
/* defined(LIBXML_TREE_ENABLED) || defined(LIBXML_WRITER_ENABLED) */
/* LIBXML_TREE_ENABLED */
/* defined(LIBXML_TREE_ENABLED) || defined(LIBXML_WRITER_ENABLED) */
/* LIBXML_TREE_ENABLED || LIBXML_HTML_ENABLED || LIBXML_SCHEMAS_ENABLED */
/*
 * Namespaces.
 */
/* defined(LIBXML_TREE_ENABLED) || defined(LIBXML_XPATH_ENABLED) */
/*
 * Changing the content.
 */
/* defined(LIBXML_TREE_ENABLED) || defined(LIBXML_XINCLUDE_ENABLED) || \
	  defined(LIBXML_SCHEMAS_ENABLED) || defined(LIBXML_HTML_ENABLED) */
/* LIBXML_TREE_ENABLED */
/* LIBXML_TREE_ENABLED */
/* LIBXML_TREE_ENABLED */
/*
 * Removing content.
 */
/* defined(LIBXML_TREE_ENABLED) || defined(LIBXML_SCHEMAS_ENABLED) */
/*
 * Internal, don't use.
 */
/* LIBXML_OUTPUT_ENABLED */
/*
 * Namespace handling.
 */
/*
 * Saving.
 */
/* LIBXML_OUTPUT_ENABLED */
/*
 * XHTML
 */
/*
 * Compression.
 */
/*
* DOM-wrapper helper functions.
*/
/*
* xmlDOMWrapAdoptNode:
* @ctxt: the optional context for custom processing
* @sourceDoc: the optional sourceDoc
* @node: the node to start with
* @destDoc: the destination doc
* @destParent: the optional new parent of @node in @destDoc
* @options: option flags
*
* References of out-of scope ns-decls are remapped to point to @destDoc:
* 1) If @destParent is given, then nsDef entries on element-nodes are used
* 2) If *no* @destParent is given, then @destDoc->oldNs entries are used
*    This is the case when you have an unlinked node and just want to move it
*    to the context of
*
* If @destParent is given, it ensures that the tree is namespace
* wellformed by creating additional ns-decls where needed.
* Note that, since prefixes of already existent ns-decls can be
* shadowed by this process, it could break QNames in attribute
* values or element content.
* NOTE: This function was not intensively tested.
*
* Returns 0 if the operation succeeded,
*         1 if a node of unsupported type was given,
*         2 if a node of not yet supported type was given and
*         -1 on API/internal errors.
*/
#[no_mangle]
pub unsafe extern "C" fn xmlDOMWrapAdoptNode(mut ctxt: xmlDOMWrapCtxtPtr,
                                             mut sourceDoc: xmlDocPtr,
                                             mut node: xmlNodePtr,
                                             mut destDoc: xmlDocPtr,
                                             mut destParent: xmlNodePtr,
                                             mut options: std::os::raw::c_int)
 -> std::os::raw::c_int {
    if node.is_null() ||
           (*node).type_0 as std::os::raw::c_uint ==
               XML_NAMESPACE_DECL as std::os::raw::c_int as std::os::raw::c_uint ||
           destDoc.is_null() ||
           !destParent.is_null() && (*destParent).doc != destDoc {
        return -(1 as std::os::raw::c_int)
    }
    /*
    * Check node->doc sanity.
    */
    if !(*node).doc.is_null() && !sourceDoc.is_null() &&
           (*node).doc != sourceDoc {
        /*
	* Might be an XIncluded node.
	*/
        return -(1 as std::os::raw::c_int)
    }
    if sourceDoc.is_null() { sourceDoc = (*node).doc }
    if sourceDoc == destDoc { return -(1 as std::os::raw::c_int) }
    match (*node).type_0 as std::os::raw::c_uint {
        1 | 2 | 3 | 4 | 5 | 7 | 8 => { }
        11 => {
            /* TODO: Support document-fragment-nodes. */
            return 2 as std::os::raw::c_int
        }
        _ => { return 1 as std::os::raw::c_int }
    }
    /*
    * Unlink only if @node was not already added to @destParent.
    */
    if !(*node).parent.is_null() && destParent != (*node).parent {
        xmlUnlinkNode(node);
    }
    if (*node).type_0 as std::os::raw::c_uint ==
           XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        return xmlDOMWrapAdoptBranch(ctxt, sourceDoc, node, destDoc,
                                     destParent, options)
    } else {
        if (*node).type_0 as std::os::raw::c_uint ==
               XML_ATTRIBUTE_NODE as std::os::raw::c_int as std::os::raw::c_uint {
            return xmlDOMWrapAdoptAttr(ctxt, sourceDoc, node as xmlAttrPtr,
                                       destDoc, destParent, options)
        } else {
            let mut cur: xmlNodePtr = node;
            let mut adoptStr: std::os::raw::c_int = 1 as std::os::raw::c_int;
            (*cur).doc = destDoc;
            /*
	* Optimize string adoption.
	*/
            if !sourceDoc.is_null() && (*sourceDoc).dict == (*destDoc).dict {
                adoptStr = 0 as std::os::raw::c_int
            }
            match (*node).type_0 as std::os::raw::c_uint {
                3 | 4 => {
                    if adoptStr != 0 && !(*node).content.is_null() &&
                           !sourceDoc.is_null() &&
                           !(*sourceDoc).dict.is_null() &&
                           xmlDictOwns((*sourceDoc).dict, (*cur).content) != 0
                       {
                        if !(*destDoc).dict.is_null() {
                            (*cur).content =
                                xmlDictLookup((*destDoc).dict, (*cur).content,
                                              -(1 as std::os::raw::c_int)) as
                                    *mut xmlChar
                        } else { (*cur).content = xmlStrdup((*cur).content) }
                    }
                }
                5 => {
                    /*
		* Remove reference to the entitity-node.
		*/
                    (*node).content = 0 as *mut xmlChar;
                    (*node).children = 0 as *mut _xmlNode;
                    (*node).last = 0 as *mut _xmlNode;
                    if !(*destDoc).intSubset.is_null() ||
                           !(*destDoc).extSubset.is_null() {
                        let mut ent: xmlEntityPtr = 0 as *mut xmlEntity;
                        /*
		    * Assign new entity-node if available.
		    */
                        ent =
                            xmlGetDocEntity(destDoc as *const xmlDoc,
                                            (*node).name);
                        if !ent.is_null() {
                            (*node).content = (*ent).content;
                            (*node).children = ent as xmlNodePtr;
                            (*node).last = ent as xmlNodePtr
                        }
                    }
                    if adoptStr != 0 && !(*node).name.is_null() {
                        if !(*destDoc).dict.is_null() {
                            let mut old: *const xmlChar = (*node).name;
                            (*node).name =
                                xmlDictLookup((*destDoc).dict, (*node).name,
                                              -(1 as std::os::raw::c_int));
                            if sourceDoc.is_null() ||
                                   (*sourceDoc).dict.is_null() ||
                                   xmlDictOwns((*sourceDoc).dict, old) == 0 {
                                xmlFree.expect("non-null function pointer")(old
                                                                                as
                                                                                *mut std::os::raw::c_char
                                                                                as
                                                                                *mut std::os::raw::c_void);
                            }
                        } else if !sourceDoc.is_null() &&
                                      !(*sourceDoc).dict.is_null() &&
                                      xmlDictOwns((*sourceDoc).dict,
                                                  (*node).name) != 0 {
                            (*node).name = xmlStrdup((*node).name)
                        }
                    }
                }
                7 => {
                    if adoptStr != 0 && !(*node).name.is_null() {
                        if !(*destDoc).dict.is_null() {
                            let mut old_0: *const xmlChar = (*node).name;
                            (*node).name =
                                xmlDictLookup((*destDoc).dict, (*node).name,
                                              -(1 as std::os::raw::c_int));
                            if sourceDoc.is_null() ||
                                   (*sourceDoc).dict.is_null() ||
                                   xmlDictOwns((*sourceDoc).dict, old_0) == 0
                               {
                                xmlFree.expect("non-null function pointer")(old_0
                                                                                as
                                                                                *mut std::os::raw::c_char
                                                                                as
                                                                                *mut std::os::raw::c_void);
                            }
                        } else if !sourceDoc.is_null() &&
                                      !(*sourceDoc).dict.is_null() &&
                                      xmlDictOwns((*sourceDoc).dict,
                                                  (*node).name) != 0 {
                            (*node).name = xmlStrdup((*node).name)
                        }
                    }
                    if adoptStr != 0 && !(*node).content.is_null() &&
                           !sourceDoc.is_null() &&
                           !(*sourceDoc).dict.is_null() &&
                           xmlDictOwns((*sourceDoc).dict, (*cur).content) != 0
                       {
                        if !(*destDoc).dict.is_null() {
                            (*cur).content =
                                xmlDictLookup((*destDoc).dict, (*cur).content,
                                              -(1 as std::os::raw::c_int)) as
                                    *mut xmlChar
                        } else { (*cur).content = xmlStrdup((*cur).content) }
                    }
                }
                _ => { }
            }
        }
    }
    return 0 as std::os::raw::c_int;
}
/* __INCLUDE_ELFGCCHACK */
