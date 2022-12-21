#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, label_break_value, main,
           ptr_offset_from, register_tool)]
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
 * Summary: implementation of the Relax-NG validation
 * Description: implementation of the Relax-NG validation
 *
 * Copy: See Copyright for the status of this software.
 *
 * Author: Daniel Veillard
 */
    pub type _xmlRelaxNG;
    /* *
 * A schemas validation context
 */
    pub type _xmlRelaxNGParserCtxt;
    pub type _xmlRelaxNGValidCtxt;
    /*
 * Summary: incomplete XML Schemas structure implementation
 * Description: interface to the XML Schemas handling and schema validity
 *              checking, it is incomplete right now.
 *
 * Copy: See Copyright for the status of this software.
 *
 * Author: Daniel Veillard
 */
    /* *
 * This error codes are obsolete; not used any more.
 */
    /*
* ATTENTION: Change xmlSchemaSetValidOptions's check
* for invalid values, if adding to the validation
* options below.
*/
/* *
 * xmlSchemaValidOption:
 *
 * This is the set of XML Schema validation options.
 */
    /* Default/fixed: create an attribute node
	* or an element's text node on the instance.
	*/
    /*
    XML_SCHEMA_VAL_XSI_ASSEMBLE			= 1<<1,
	* assemble schemata using
	* xsi:schemaLocation and
	* xsi:noNamespaceSchemaLocation
*/
    /* *
 * The schemas related types are kept internal
 */
    pub type _xmlSchema;
    /* *
 * A schemas validation context
 */
    pub type _xmlSchemaParserCtxt;
    pub type _xmlSchemaValidCtxt;
    pub type _xmlPattern;
    pub type _xmlXIncludeCtxt;
    /* streaming interfaces */
    pub type _xmlStreamCtxt;
    pub type _xmlXPathCompExpr;
    pub type _xmlSchematron;
    pub type _xmlSchematronParserCtxt;
    pub type _xmlSchematronValidCtxt;
    pub type _xmlSaveCtxt;
    #[no_mangle]
    static mut stdin: *mut FILE;
    #[no_mangle]
    static mut stdout: *mut FILE;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> std::os::raw::c_int;
    #[no_mangle]
    fn fflush(__stream: *mut FILE) -> std::os::raw::c_int;
    #[no_mangle]
    fn fopen(__filename: *const std::os::raw::c_char, __modes: *const std::os::raw::c_char)
     -> *mut FILE;
    #[no_mangle]
    fn xmlStrndup(cur: *const xmlChar, len: std::os::raw::c_int) -> *mut xmlChar;
    #[no_mangle]
    fn xmlStrdup(cur: *const xmlChar) -> *mut xmlChar;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    fn sscanf(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char, _: ...)
     -> std::os::raw::c_int;
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
    fn fgets(__s: *mut std::os::raw::c_char, __n: std::os::raw::c_int, __stream: *mut FILE)
     -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn fread(__ptr: *mut std::os::raw::c_void, __size: size_t, __n: size_t,
             __stream: *mut FILE) -> size_t;
    #[no_mangle]
    fn xmlStrcat(cur: *mut xmlChar, add: *const xmlChar) -> *mut xmlChar;
    #[no_mangle]
    fn xmlGetIntSubset(doc: *const xmlDoc) -> xmlDtdPtr;
    #[no_mangle]
    fn xmlFreeDtd(cur: xmlDtdPtr);
    #[no_mangle]
    fn xmlNewDoc(version: *const xmlChar) -> xmlDocPtr;
    #[no_mangle]
    fn xmlFreeDoc(cur: xmlDocPtr);
    /* LIBXML_TREE_ENABLED */
    #[no_mangle]
    fn xmlCopyDoc(doc: xmlDocPtr, recursive: std::os::raw::c_int) -> xmlDocPtr;
    /* defined(LIBXML_TREE_ENABLED) || defined(LIBXML_SCHEMAS_ENABLED) */
    /*
 * Creating new nodes.
 */
    #[no_mangle]
    fn xmlNewDocNode(doc: xmlDocPtr, ns: xmlNsPtr, name: *const xmlChar,
                     content: *const xmlChar) -> xmlNodePtr;
    #[no_mangle]
    fn xmlGetNodePath(node: *const xmlNode) -> *mut xmlChar;
    /* defined(LIBXML_TREE_ENABLED) || defined(LIBXML_DEBUG_ENABLED) */
    #[no_mangle]
    fn xmlDocGetRootElement(doc: *const xmlDoc) -> xmlNodePtr;
    /*
 * Changing the structure.
 */
    #[no_mangle]
    fn xmlDocSetRootElement(doc: xmlDocPtr, root: xmlNodePtr) -> xmlNodePtr;
    #[no_mangle]
    fn xmlUnlinkNode(cur: xmlNodePtr);
    /* LIBXML_TREE_ENABLED */
    #[no_mangle]
    fn xmlNodeSetContent(cur: xmlNodePtr, content: *const xmlChar);
    /*
 * Saving.
 */
    #[no_mangle]
    fn xmlDocDumpFormatMemory(cur: xmlDocPtr, mem: *mut *mut xmlChar,
                              size: *mut std::os::raw::c_int, format_0: std::os::raw::c_int);
    #[no_mangle]
    fn xmlDocDumpMemory(cur: xmlDocPtr, mem: *mut *mut xmlChar,
                        size: *mut std::os::raw::c_int);
    #[no_mangle]
    fn xmlDocDumpMemoryEnc(out_doc: xmlDocPtr, doc_txt_ptr: *mut *mut xmlChar,
                           doc_txt_len: *mut std::os::raw::c_int,
                           txt_encoding: *const std::os::raw::c_char);
    #[no_mangle]
    fn xmlDocDumpFormatMemoryEnc(out_doc: xmlDocPtr,
                                 doc_txt_ptr: *mut *mut xmlChar,
                                 doc_txt_len: *mut std::os::raw::c_int,
                                 txt_encoding: *const std::os::raw::c_char,
                                 format_0: std::os::raw::c_int);
    #[no_mangle]
    fn xmlDocDump(f: *mut FILE, cur: xmlDocPtr) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlSaveFile(filename: *const std::os::raw::c_char, cur: xmlDocPtr)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlSaveFormatFile(filename: *const std::os::raw::c_char, cur: xmlDocPtr,
                         format_0: std::os::raw::c_int) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlSaveFormatFileEnc(filename: *const std::os::raw::c_char, cur: xmlDocPtr,
                            encoding_0: *const std::os::raw::c_char,
                            format_0: std::os::raw::c_int) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlSaveFileEnc(filename: *const std::os::raw::c_char, cur: xmlDocPtr,
                      encoding_0: *const std::os::raw::c_char) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlSetCompressMode(mode: std::os::raw::c_int);
    /*
 * The 4 interfaces used for all memory handling within libxml.
LIBXML_DLL_IMPORT xmlFreeFunc xmlFree;
LIBXML_DLL_IMPORT xmlMallocFunc xmlMalloc;
LIBXML_DLL_IMPORT xmlMallocFunc xmlMallocAtomic;
LIBXML_DLL_IMPORT xmlReallocFunc xmlRealloc;
LIBXML_DLL_IMPORT xmlStrdupFunc xmlMemStrdup;
 */
    /*
 * The way to overload the existing functions.
 * The xmlGc function have an extra entry for atomic block
 * allocations useful for garbage collected memory allocators
 */
    #[no_mangle]
    fn xmlMemSetup(freeFunc: xmlFreeFunc, mallocFunc: xmlMallocFunc,
                   reallocFunc: xmlReallocFunc, strdupFunc: xmlStrdupFunc)
     -> std::os::raw::c_int;
    /*
 * These are specific to the XML debug memory wrapper.
 */
    #[no_mangle]
    fn xmlMemUsed() -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlMemoryDump();
    #[no_mangle]
    fn xmlMemMalloc(size: size_t) -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn xmlMemRealloc(ptr: *mut std::os::raw::c_void, size: size_t)
     -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn xmlMemFree(ptr: *mut std::os::raw::c_void);
    #[no_mangle]
    fn xmlMemoryStrdup(str: *const std::os::raw::c_char) -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn xmlFreeEnumeration(cur: xmlEnumerationPtr);
    /* *
 * The public function calls related to validity checking.
 */
    /* Allocate/Release Validation Contexts */
    #[no_mangle]
    fn xmlNewValidCtxt() -> xmlValidCtxtPtr;
    #[no_mangle]
    fn xmlFreeValidCtxt(_: xmlValidCtxtPtr);
    #[no_mangle]
    fn xmlValidateDtd(ctxt: xmlValidCtxtPtr, doc: xmlDocPtr, dtd: xmlDtdPtr)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlValidateDocument(ctxt: xmlValidCtxtPtr, doc: xmlDocPtr)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlValidGetValidElements(prev: *mut xmlNode, next: *mut xmlNode,
                                names: *mut *const xmlChar, max: std::os::raw::c_int)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlAddEncodingAlias(name: *const std::os::raw::c_char,
                           alias: *const std::os::raw::c_char) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlParserInputBufferCreateFilename(URI: *const std::os::raw::c_char,
                                          enc: xmlCharEncoding)
     -> xmlParserInputBufferPtr;
    #[no_mangle]
    fn xmlFreeParserInputBuffer(in_0: xmlParserInputBufferPtr);
    /*
 * A predefined entity loader disabling network accesses
 */
    #[no_mangle]
    fn xmlNoNetExternalEntityLoader(URL: *const std::os::raw::c_char,
                                    ID: *const std::os::raw::c_char,
                                    ctxt: xmlParserCtxtPtr)
     -> xmlParserInputPtr;
    #[no_mangle]
    fn xmlCleanupParser();
    #[no_mangle]
    fn xmlParseFile(filename: *const std::os::raw::c_char) -> xmlDocPtr;
    /* LIBXML_SAX1_ENABLED */
    #[no_mangle]
    fn xmlSubstituteEntitiesDefault(val: std::os::raw::c_int) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlKeepBlanksDefault(val: std::os::raw::c_int) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlPedanticParserDefault(val: std::os::raw::c_int) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlLineNumbersDefault(val: std::os::raw::c_int) -> std::os::raw::c_int;
    /* LIBXML_SAX1_ENABLED */
    /*
 * Less common routines and SAX interfaces
 */
    #[no_mangle]
    fn xmlParseDocument(ctxt: xmlParserCtxtPtr) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlParseDTD(ExternalID: *const xmlChar, SystemID: *const xmlChar)
     -> xmlDtdPtr;
    /*
 * Parser contexts handling.
 */
    #[no_mangle]
    fn xmlNewParserCtxt() -> xmlParserCtxtPtr;
    #[no_mangle]
    fn xmlFreeParserCtxt(ctxt: xmlParserCtxtPtr);
    /* LIBXML_LEGACY_ENABLED */
    /*
 * Interfaces for the Push mode.
 */
    #[no_mangle]
    fn xmlCreatePushParserCtxt(sax_0: xmlSAXHandlerPtr,
                               user_data: *mut std::os::raw::c_void,
                               chunk: *const std::os::raw::c_char, size: std::os::raw::c_int,
                               filename: *const std::os::raw::c_char)
     -> xmlParserCtxtPtr;
    #[no_mangle]
    fn xmlParseChunk(ctxt: xmlParserCtxtPtr, chunk: *const std::os::raw::c_char,
                     size: std::os::raw::c_int, terminate: std::os::raw::c_int)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlNewIOInputStream(ctxt: xmlParserCtxtPtr,
                           input: xmlParserInputBufferPtr,
                           enc: xmlCharEncoding) -> xmlParserInputPtr;
    /*
 * External entities handling actually implemented in xmlIO.
 */
    #[no_mangle]
    fn xmlSetExternalEntityLoader(f: xmlExternalEntityLoader);
    #[no_mangle]
    fn xmlGetExternalEntityLoader() -> xmlExternalEntityLoader;
    #[no_mangle]
    fn xmlCtxtUseOptions(ctxt: xmlParserCtxtPtr, options_0: std::os::raw::c_int)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlReadFile(URL: *const std::os::raw::c_char, encoding_0: *const std::os::raw::c_char,
                   options_0: std::os::raw::c_int) -> xmlDocPtr;
    #[no_mangle]
    fn xmlReadMemory(buffer: *const std::os::raw::c_char, size: std::os::raw::c_int,
                     URL: *const std::os::raw::c_char,
                     encoding_0: *const std::os::raw::c_char, options_0: std::os::raw::c_int)
     -> xmlDocPtr;
    #[no_mangle]
    fn xmlReadFd(fd: std::os::raw::c_int, URL: *const std::os::raw::c_char,
                 encoding_0: *const std::os::raw::c_char, options_0: std::os::raw::c_int)
     -> xmlDocPtr;
    #[no_mangle]
    fn xmlReadIO(ioread: xmlInputReadCallback, ioclose: xmlInputCloseCallback,
                 ioctx: *mut std::os::raw::c_void, URL: *const std::os::raw::c_char,
                 encoding_0: *const std::os::raw::c_char, options_0: std::os::raw::c_int)
     -> xmlDocPtr;
    #[no_mangle]
    fn xmlCtxtReadFile(ctxt: xmlParserCtxtPtr, filename: *const std::os::raw::c_char,
                       encoding_0: *const std::os::raw::c_char,
                       options_0: std::os::raw::c_int) -> xmlDocPtr;
    #[no_mangle]
    fn xmlCtxtReadMemory(ctxt: xmlParserCtxtPtr, buffer: *const std::os::raw::c_char,
                         size: std::os::raw::c_int, URL: *const std::os::raw::c_char,
                         encoding_0: *const std::os::raw::c_char,
                         options_0: std::os::raw::c_int) -> xmlDocPtr;
    #[no_mangle]
    fn xmlCtxtReadIO(ctxt: xmlParserCtxtPtr, ioread: xmlInputReadCallback,
                     ioclose: xmlInputCloseCallback, ioctx: *mut std::os::raw::c_void,
                     URL: *const std::os::raw::c_char,
                     encoding_0: *const std::os::raw::c_char, options_0: std::os::raw::c_int)
     -> xmlDocPtr;
    #[no_mangle]
    fn xmlHasFeature(feature: xmlFeature) -> std::os::raw::c_int;
    #[no_mangle]
    fn strtol(__nptr: *const std::os::raw::c_char, __endptr: *mut *mut std::os::raw::c_char,
              __base: std::os::raw::c_int) -> std::os::raw::c_long;
    #[no_mangle]
    fn malloc(_: std::os::raw::c_ulong) -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn free(__ptr: *mut std::os::raw::c_void);
    #[no_mangle]
    fn exit(_: std::os::raw::c_int) -> !;
    #[no_mangle]
    fn getenv(__name: *const std::os::raw::c_char) -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn xmlSAXDefaultVersion(version: std::os::raw::c_int) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlRegisterNodeDefault(func: xmlRegisterNodeFunc)
     -> xmlRegisterNodeFunc;
    #[no_mangle]
    fn xmlDeregisterNodeDefault(func: xmlDeregisterNodeFunc)
     -> xmlDeregisterNodeFunc;
    #[no_mangle]
    static mut xmlFree: xmlFreeFunc;
    #[no_mangle]
    fn __xmlDoValidityCheckingDefaultValue() -> *mut std::os::raw::c_int;
    #[no_mangle]
    fn __xmlGenericError() -> *mut xmlGenericErrorFunc;
    #[no_mangle]
    fn __xmlGenericErrorContext() -> *mut *mut std::os::raw::c_void;
    #[no_mangle]
    fn __xmlGetWarningsDefaultValue() -> *mut std::os::raw::c_int;
    #[no_mangle]
    fn __xmlTreeIndentString() -> *mut *const std::os::raw::c_char;
    #[no_mangle]
    fn __xmlLoadExtDtdDefaultValue() -> *mut std::os::raw::c_int;
    #[no_mangle]
    fn __xmlParserDebugEntities() -> *mut std::os::raw::c_int;
    #[no_mangle]
    fn __xmlParserVersion() -> *mut *const std::os::raw::c_char;
    #[no_mangle]
    fn xmlRelaxNGCleanupTypes();
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
    fn xmlRelaxNGFree(schema_0: xmlRelaxNGPtr);
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
    fn xmlRelaxNGNewValidCtxt(schema_0: xmlRelaxNGPtr)
     -> xmlRelaxNGValidCtxtPtr;
    #[no_mangle]
    fn xmlRelaxNGFreeValidCtxt(ctxt: xmlRelaxNGValidCtxtPtr);
    #[no_mangle]
    fn xmlRelaxNGValidateDoc(ctxt: xmlRelaxNGValidCtxtPtr, doc: xmlDocPtr)
     -> std::os::raw::c_int;
    /*
 * Interfaces for parsing.
 */
    #[no_mangle]
    fn xmlSchemaNewParserCtxt(URL: *const std::os::raw::c_char)
     -> xmlSchemaParserCtxtPtr;
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
    fn xmlSchemaFree(schema_0: xmlSchemaPtr);
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
    fn xmlSchemaValidateSetFilename(vctxt: xmlSchemaValidCtxtPtr,
                                    filename: *const std::os::raw::c_char);
    #[no_mangle]
    fn xmlSchemaNewValidCtxt(schema_0: xmlSchemaPtr) -> xmlSchemaValidCtxtPtr;
    #[no_mangle]
    fn xmlSchemaFreeValidCtxt(ctxt: xmlSchemaValidCtxtPtr);
    #[no_mangle]
    fn xmlSchemaValidateDoc(ctxt: xmlSchemaValidCtxtPtr, instance: xmlDocPtr)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlSchemaValidateStream(ctxt: xmlSchemaValidCtxtPtr,
                               input: xmlParserInputBufferPtr,
                               enc: xmlCharEncoding, sax_0: xmlSAXHandlerPtr,
                               user_data: *mut std::os::raw::c_void) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlFreeTextReader(reader: xmlTextReaderPtr);
    #[no_mangle]
    fn xmlTextReaderRead(reader: xmlTextReaderPtr) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlTextReaderDepth(reader: xmlTextReaderPtr) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlTextReaderHasValue(reader: xmlTextReaderPtr) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlTextReaderIsEmptyElement(reader: xmlTextReaderPtr) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlTextReaderNodeType(reader: xmlTextReaderPtr) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlTextReaderConstLocalName(reader: xmlTextReaderPtr)
     -> *const xmlChar;
    #[no_mangle]
    fn xmlTextReaderConstName(reader: xmlTextReaderPtr) -> *const xmlChar;
    #[no_mangle]
    fn xmlTextReaderConstNamespaceUri(reader: xmlTextReaderPtr)
     -> *const xmlChar;
    #[no_mangle]
    fn xmlTextReaderConstValue(reader: xmlTextReaderPtr) -> *const xmlChar;
    #[no_mangle]
    fn xmlTextReaderSetParserProp(reader: xmlTextReaderPtr, prop: std::os::raw::c_int,
                                  value: std::os::raw::c_int) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlTextReaderCurrentNode(reader: xmlTextReaderPtr) -> xmlNodePtr;
    #[no_mangle]
    fn xmlTextReaderIsValid(reader: xmlTextReaderPtr) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlTextReaderRelaxNGValidate(reader: xmlTextReaderPtr,
                                    rng: *const std::os::raw::c_char) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlTextReaderSchemaValidate(reader: xmlTextReaderPtr,
                                   xsd: *const std::os::raw::c_char) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlReaderWalker(doc: xmlDocPtr) -> xmlTextReaderPtr;
    #[no_mangle]
    fn xmlReaderForFile(filename: *const std::os::raw::c_char,
                        encoding_0: *const std::os::raw::c_char,
                        options_0: std::os::raw::c_int) -> xmlTextReaderPtr;
    #[no_mangle]
    fn xmlReaderForMemory(buffer: *const std::os::raw::c_char, size: std::os::raw::c_int,
                          URL: *const std::os::raw::c_char,
                          encoding_0: *const std::os::raw::c_char,
                          options_0: std::os::raw::c_int) -> xmlTextReaderPtr;
    #[no_mangle]
    fn xmlXIncludeProcessFlags(doc: xmlDocPtr, flags: std::os::raw::c_int)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlFreePattern(comp: xmlPatternPtr);
    #[no_mangle]
    fn xmlPatterncompile(pattern_0: *const xmlChar, dict: *mut xmlDict,
                         flags: std::os::raw::c_int, namespaces: *mut *const xmlChar)
     -> xmlPatternPtr;
    #[no_mangle]
    fn xmlPatternMatch(comp: xmlPatternPtr, node: xmlNodePtr) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlPatternGetStreamCtxt(comp: xmlPatternPtr) -> xmlStreamCtxtPtr;
    #[no_mangle]
    fn xmlFreeStreamCtxt(stream_0: xmlStreamCtxtPtr);
    #[no_mangle]
    fn xmlStreamPush(stream_0: xmlStreamCtxtPtr, name: *const xmlChar,
                     ns: *const xmlChar) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlStreamPop(stream_0: xmlStreamCtxtPtr) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmllint_endTimer(fmt: *const std::os::raw::c_char, _: ...);
    #[no_mangle]
    fn xmlHTMLError(ctx: *mut std::os::raw::c_void, msg: *const std::os::raw::c_char, _: ...);
    #[no_mangle]
    fn xmlHTMLWarning(ctx: *mut std::os::raw::c_void, msg: *const std::os::raw::c_char,
                      _: ...);
    #[no_mangle]
    fn xmlHTMLValidityError(ctx: *mut std::os::raw::c_void, msg: *const std::os::raw::c_char,
                            _: ...);
    #[no_mangle]
    fn xmlHTMLValidityWarning(ctx: *mut std::os::raw::c_void,
                              msg: *const std::os::raw::c_char, _: ...);
    #[no_mangle]
    fn xmllint_warningDebug(ctx: *mut std::os::raw::c_void, msg: *const std::os::raw::c_char,
                            _: ...);
    #[no_mangle]
    fn xmllint_errorDebug(ctx: *mut std::os::raw::c_void, msg: *const std::os::raw::c_char,
                          _: ...);
    #[no_mangle]
    fn xmllint_fatalErrorDebug(ctx: *mut std::os::raw::c_void,
                               msg: *const std::os::raw::c_char, _: ...);
    #[no_mangle]
    fn strcmp(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char) -> std::os::raw::c_int;
    #[no_mangle]
    fn memcpy(_: *mut std::os::raw::c_void, _: *const std::os::raw::c_void, _: std::os::raw::c_ulong)
     -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn strlen(_: *const std::os::raw::c_char) -> std::os::raw::c_ulong;
    #[no_mangle]
    fn __assert_fail(__assertion: *const std::os::raw::c_char,
                     __file: *const std::os::raw::c_char, __line: std::os::raw::c_uint,
                     __function: *const std::os::raw::c_char) -> !;
    #[no_mangle]
    fn gettimeofday(__tv: *mut timeval, __tz: *mut std::os::raw::c_void)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn __xstat(__ver: std::os::raw::c_int, __filename: *const std::os::raw::c_char,
               __stat_buf: *mut stat) -> std::os::raw::c_int;
    #[no_mangle]
    fn open(__file: *const std::os::raw::c_char, __oflag: std::os::raw::c_int, _: ...)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn close(__fd: std::os::raw::c_int) -> std::os::raw::c_int;
    #[no_mangle]
    fn write(__fd: std::os::raw::c_int, __buf: *const std::os::raw::c_void, __n: size_t)
     -> ssize_t;
    #[no_mangle]
    fn mmap(__addr: *mut std::os::raw::c_void, __len: size_t, __prot: std::os::raw::c_int,
            __flags: std::os::raw::c_int, __fd: std::os::raw::c_int, __offset: __off64_t)
     -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn munmap(__addr: *mut std::os::raw::c_void, __len: size_t) -> std::os::raw::c_int;
    /* *
 * Interfaces for the Push mode.
 */
    #[no_mangle]
    fn htmlParseChunk(ctxt: htmlParserCtxtPtr, chunk: *const std::os::raw::c_char,
                      size: std::os::raw::c_int, terminate: std::os::raw::c_int)
     -> std::os::raw::c_int;
    /* LIBXML_PUSH_ENABLED */
    #[no_mangle]
    fn htmlFreeParserCtxt(ctxt: htmlParserCtxtPtr);
    #[no_mangle]
    fn htmlReadFile(URL: *const std::os::raw::c_char, encoding_0: *const std::os::raw::c_char,
                    options_0: std::os::raw::c_int) -> htmlDocPtr;
    #[no_mangle]
    fn htmlReadMemory(buffer: *const std::os::raw::c_char, size: std::os::raw::c_int,
                      URL: *const std::os::raw::c_char,
                      encoding_0: *const std::os::raw::c_char, options_0: std::os::raw::c_int)
     -> htmlDocPtr;
    #[no_mangle]
    fn inputPush(ctxt: xmlParserCtxtPtr, value: xmlParserInputPtr)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn htmlCreatePushParserCtxt(sax_0: htmlSAXHandlerPtr,
                                user_data: *mut std::os::raw::c_void,
                                chunk: *const std::os::raw::c_char, size: std::os::raw::c_int,
                                filename: *const std::os::raw::c_char,
                                enc: xmlCharEncoding) -> htmlParserCtxtPtr;
    #[no_mangle]
    fn htmlDocDump(f: *mut FILE, cur: xmlDocPtr) -> std::os::raw::c_int;
    #[no_mangle]
    fn htmlSaveFile(filename: *const std::os::raw::c_char, cur: xmlDocPtr)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn htmlSaveFileFormat(filename: *const std::os::raw::c_char, cur: xmlDocPtr,
                          encoding_0: *const std::os::raw::c_char,
                          format_0: std::os::raw::c_int) -> std::os::raw::c_int;
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
    /* *
 * Evaluation functions.
 */
    #[no_mangle]
    fn xmlXPathOrderDocElems(doc: xmlDocPtr) -> std::os::raw::c_long;
    #[no_mangle]
    fn xmlXPathEval(str: *const xmlChar, ctx: xmlXPathContextPtr)
     -> xmlXPathObjectPtr;
    #[no_mangle]
    fn xmlXPathIsNaN(val: std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlXPathIsInf(val: std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlDebugDumpDocument(output_0: *mut FILE, doc: xmlDocPtr);
    #[no_mangle]
    fn xmlDebugDumpEntities(output_0: *mut FILE, doc: xmlDocPtr);
    /*
 * The Shell interface.
 */
    #[no_mangle]
    fn xmlShell(doc: xmlDocPtr, filename: *mut std::os::raw::c_char,
                input: xmlShellReadlineFunc, output_0: *mut FILE);
    #[no_mangle]
    fn xmlLoadCatalogs(paths_0: *const std::os::raw::c_char);
    /*
 * Interfaces for parsing.
 */
    #[no_mangle]
    fn xmlSchematronNewParserCtxt(URL: *const std::os::raw::c_char)
     -> xmlSchematronParserCtxtPtr;
    #[no_mangle]
    fn xmlSchematronFreeParserCtxt(ctxt: xmlSchematronParserCtxtPtr);
    /* ****
XMLPUBFUN void XMLCALL
	    xmlSchematronSetParserErrors(xmlSchematronParserCtxtPtr ctxt,
					 xmlSchematronValidityErrorFunc err,
					 xmlSchematronValidityWarningFunc warn,
					 void *ctx);
XMLPUBFUN int XMLCALL
		xmlSchematronGetParserErrors(xmlSchematronParserCtxtPtr ctxt,
					xmlSchematronValidityErrorFunc * err,
					xmlSchematronValidityWarningFunc * warn,
					void **ctx);
XMLPUBFUN int XMLCALL
		xmlSchematronIsValid	(xmlSchematronValidCtxtPtr ctxt);
 *****/
    #[no_mangle]
    fn xmlSchematronParse(ctxt: xmlSchematronParserCtxtPtr)
     -> xmlSchematronPtr;
    #[no_mangle]
    fn xmlSchematronFree(schema_0: xmlSchematronPtr);
    /* *****
XMLPUBFUN void XMLCALL
	    xmlSchematronSetValidErrors	(xmlSchematronValidCtxtPtr ctxt,
					 xmlSchematronValidityErrorFunc err,
					 xmlSchematronValidityWarningFunc warn,
					 void *ctx);
XMLPUBFUN int XMLCALL
	    xmlSchematronGetValidErrors	(xmlSchematronValidCtxtPtr ctxt,
					 xmlSchematronValidityErrorFunc *err,
					 xmlSchematronValidityWarningFunc *warn,
					 void **ctx);
XMLPUBFUN int XMLCALL
	    xmlSchematronSetValidOptions(xmlSchematronValidCtxtPtr ctxt,
					 int options);
XMLPUBFUN int XMLCALL
	    xmlSchematronValidCtxtGetOptions(xmlSchematronValidCtxtPtr ctxt);
XMLPUBFUN int XMLCALL
            xmlSchematronValidateOneElement (xmlSchematronValidCtxtPtr ctxt,
			                 xmlNodePtr elem);
 *******/
    #[no_mangle]
    fn xmlSchematronNewValidCtxt(schema_0: xmlSchematronPtr,
                                 options_0: std::os::raw::c_int)
     -> xmlSchematronValidCtxtPtr;
    #[no_mangle]
    fn xmlSchematronFreeValidCtxt(ctxt: xmlSchematronValidCtxtPtr);
    #[no_mangle]
    fn xmlSchematronValidateDoc(ctxt: xmlSchematronValidCtxtPtr,
                                instance: xmlDocPtr) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlC14NDocDumpMemory(doc: xmlDocPtr, nodes: xmlNodeSetPtr,
                            mode: std::os::raw::c_int,
                            inclusive_ns_prefixes: *mut *mut xmlChar,
                            with_comments: std::os::raw::c_int,
                            doc_txt_ptr: *mut *mut xmlChar) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlSaveToFd(fd: std::os::raw::c_int, encoding_0: *const std::os::raw::c_char,
                   options_0: std::os::raw::c_int) -> xmlSaveCtxtPtr;
    #[no_mangle]
    fn xmlSaveToFilename(filename: *const std::os::raw::c_char,
                         encoding_0: *const std::os::raw::c_char,
                         options_0: std::os::raw::c_int) -> xmlSaveCtxtPtr;
    #[no_mangle]
    fn xmlSaveDoc(ctxt: xmlSaveCtxtPtr, doc: xmlDocPtr) -> std::os::raw::c_long;
    #[no_mangle]
    fn xmlSaveTree(ctxt: xmlSaveCtxtPtr, node: xmlNodePtr) -> std::os::raw::c_long;
    #[no_mangle]
    fn xmlSaveClose(ctxt: xmlSaveCtxtPtr) -> std::os::raw::c_int;
    #[no_mangle]
    static mut xmllint_noout: std::os::raw::c_int;
    #[no_mangle]
    static mut xmllint_progresult: xmllintReturnCode;
    /* ***********************************************************************
 *									*
 * Internal timing routines to remove the necessity to have		*
 * unix-specific function calls.					*
 *									*
 ************************************************************************/
    /* !HAVE_GETTIMEOFDAY */
    #[no_mangle]
    static mut xmllint_begin: timeval;
    #[no_mangle]
    static mut xmllint_callbacks: std::os::raw::c_int;
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
pub type __suseconds_t = std::os::raw::c_long;
pub type __blksize_t = std::os::raw::c_long;
pub type __blkcnt_t = std::os::raw::c_long;
pub type __ssize_t = std::os::raw::c_long;
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
pub type ssize_t = __ssize_t;
/* *
 * xmlOutputWriteCallback:
 * @context:  an Output context
 * @buffer:  the buffer of data to write
 * @len:  the length of the buffer in bytes
 *
 * Callback used in the I/O Output API to write to the resource
 *
 * Returns the number of bytes written or -1 in case of error
 */
/* *
 * xmlOutputCloseCallback:
 * @context:  an Output context
 *
 * Callback used in the I/O Output API to close the resource
 *
 * Returns 0 or -1 in case of error
 */
/* LIBXML_OUTPUT_ENABLED */
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
pub type xmlInputCloseCallback
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void) -> std::os::raw::c_int>;
pub type xmlInputReadCallback
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *mut std::os::raw::c_char,
                                _: std::os::raw::c_int) -> std::os::raw::c_int>;
pub type xmlParserInputBuffer = _xmlParserInputBuffer;
pub type xmlParserInputBufferPtr = *mut xmlParserInputBuffer;
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
pub type xmlParserInput = _xmlParserInput;
pub type xmlParserInputPtr = *mut xmlParserInput;
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
/* Input buffer */
/* UTF-8 encoded buffer */
/* The file analyzed, if any */
/* the directory/base of the file */
/* Base of the array to parse */
/* Current char being parsed */
/* end of the array to parse */
/* length if known */
/* Current line */
/* Current column */
/*
     * NOTE: consumed is only tested for equality in the parser code,
     *       so even if there is an overflow this should not give troubles
     *       for parsing very large instances.
     */
/* How many xmlChars already consumed */
/* function to deallocate the base */
/* the encoding string for entity */
/* the version string for entity */
/* Was that entity marked standalone */
/* an unique identifier for the entity */
/* *
 * xmlParserNodeInfo:
 *
 * The parser can be asked to collect Node informations, i.e. at what
 * place in the file they were detected.
 * NOTE: This is off by default and not very well tested.
 */
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
/* The SAX handler */
/* For SAX interface only, used by DOM build */
/* the document being built */
/* is the document well formed */
/* shall we replace entities ? */
/* the XML version string */
/* the declared encoding, if any */
/* standalone document */
/* an HTML(1)/Docbook(2) document
                                       * 3 is HTML after <head>
                                       * 10 is HTML after <body>
                                       */
/* Input stream stack */
/* Current input stream */
/* Number of current input streams */
/* Max number of input streams */
/* stack of inputs */
/* Node analysis stack only used for DOM building */
/* Current parsed Node */
/* Depth of the parsing stack */
/* Max depth of the parsing stack */
/* array of nodes */
/* Whether node info should be kept */
/* info about each node parsed */
/* error code */
/* reference and external subset */
/* the internal subset has PE refs */
/* are we parsing an external entity */
/* is the document valid */
/* shall we try to validate ? */
/* The validity context */
/* current type of input */
/* next char look-ahead */
/* the data directory */
/* Node name stack */
/* Current parsed Node */
/* Depth of the parsing stack */
/* Max depth of the parsing stack */
/* array of nodes */
/* number of xmlChar processed */
/* used by progressive parsing lookup */
/* ugly but ... */
/* SAX callbacks are disabled */
/* Parsing is in int 1/ext 2 subset */
/* name of subset */
/* URI of external subset */
/* SYSTEM ID of external subset */
/* xml:space values */
/* Should the parser preserve spaces */
/* Depth of the parsing stack */
/* Max depth of the parsing stack */
/* array of space infos */
/* to prevent entity substitution loops */
/* used to check entities boundaries */
/* encoding of the in-memory content
				         actually an xmlCharEncoding */
/* Those two fields are there to */
/* Speed up large node parsing */
/* signal pedantic warnings */
/* For user data, libxml won't touch it */
/* should the external subset be loaded */
/* set line number in element content */
/* document's own catalog */
/* run in recovery mode */
/* is this a progressive parsing */
/* dictionary for the parser */
/* array for the attributes callbacks */
/* the size of the array */
/* use strings from dict to build tree */
/*
     * pre-interned strings
     */
/*
     * Everything below is used only by the new SAX mode
     */
/* operating in the new SAX mode */
/* the number of inherited namespaces */
/* the size of the arrays */
/* the array of prefix/namespace name */
/* which attribute were allocated */
/* array of data for push */
/* defaulted attributes if any */
/* non-CDATA attributes if any */
/* is the document XML Nanespace okay */
/* Extra options */
/*
     * Those fields are needed only for treaming parsing so far
     */
/* Use dictionary names for the tree */
/* number of freed element nodes */
/* List of freed element nodes */
/* number of freed attributes nodes */
/* List of freed attributes nodes */
/*
     * the complete error informations for the last error.
     */
/* the parser mode */
/* number of entities references */
/* size of parsed entities */
/* for use by HTML non-recursive parser */
/* Current NodeInfo */
/* Depth of the parsing stack */
/* Max depth of the parsing stack */
/* array of nodeInfos */
/* we need to label inputs */
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
pub type xmlSAXHandler = _xmlSAXHandler;
pub type xmlSAXHandlerPtr = *mut xmlSAXHandler;
pub type xmlNsPtr = *mut xmlNs;
pub type xmlDtd = _xmlDtd;
pub type xmlDtdPtr = *mut xmlDtd;
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
/* *
 * xmlStrdupFunc:
 * @str: a zero terminated string
 *
 * Signature for an strdup() implementation.
 *
 * Returns the copy of the string or NULL in case of error.
 */
pub type xmlStrdupFunc
    =
    Option<unsafe extern "C" fn(_: *const std::os::raw::c_char) -> *mut std::os::raw::c_char>;
pub type xmlGenericErrorFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const std::os::raw::c_char,
                                _: ...) -> ()>;
pub type xmlValidCtxtPtr = *mut xmlValidCtxt;
/* *
 * xmlExternalEntityLoader:
 * @URL: The System ID of the resource requested
 * @ID: The Public ID of the resource requested
 * @context: the XML parser context
 *
 * External entity loaders types.
 *
 * Returns the entity input parser.
 */
pub type xmlExternalEntityLoader
    =
    Option<unsafe extern "C" fn(_: *const std::os::raw::c_char,
                                _: *const std::os::raw::c_char, _: xmlParserCtxtPtr)
               -> xmlParserInputPtr>;
pub type xmlCharEncoding = std::os::raw::c_int;
pub const XML_CHAR_ENCODING_ASCII: xmlCharEncoding = 22;
pub const XML_CHAR_ENCODING_EUC_JP: xmlCharEncoding = 21;
pub const XML_CHAR_ENCODING_SHIFT_JIS: xmlCharEncoding = 20;
pub const XML_CHAR_ENCODING_2022_JP: xmlCharEncoding = 19;
pub const XML_CHAR_ENCODING_8859_9: xmlCharEncoding = 18;
pub const XML_CHAR_ENCODING_8859_8: xmlCharEncoding = 17;
pub const XML_CHAR_ENCODING_8859_7: xmlCharEncoding = 16;
pub const XML_CHAR_ENCODING_8859_6: xmlCharEncoding = 15;
pub const XML_CHAR_ENCODING_8859_5: xmlCharEncoding = 14;
pub const XML_CHAR_ENCODING_8859_4: xmlCharEncoding = 13;
pub const XML_CHAR_ENCODING_8859_3: xmlCharEncoding = 12;
pub const XML_CHAR_ENCODING_8859_2: xmlCharEncoding = 11;
pub const XML_CHAR_ENCODING_8859_1: xmlCharEncoding = 10;
pub const XML_CHAR_ENCODING_UCS2: xmlCharEncoding = 9;
pub const XML_CHAR_ENCODING_UCS4_3412: xmlCharEncoding = 8;
pub const XML_CHAR_ENCODING_UCS4_2143: xmlCharEncoding = 7;
pub const XML_CHAR_ENCODING_EBCDIC: xmlCharEncoding = 6;
pub const XML_CHAR_ENCODING_UCS4BE: xmlCharEncoding = 5;
pub const XML_CHAR_ENCODING_UCS4LE: xmlCharEncoding = 4;
pub const XML_CHAR_ENCODING_UTF16BE: xmlCharEncoding = 3;
pub const XML_CHAR_ENCODING_UTF16LE: xmlCharEncoding = 2;
pub const XML_CHAR_ENCODING_UTF8: xmlCharEncoding = 1;
pub const XML_CHAR_ENCODING_NONE: xmlCharEncoding = 0;
pub const XML_CHAR_ENCODING_ERROR: xmlCharEncoding = -1;
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
/*
 * Library wide options
 */
/* *
 * xmlFeature:
 *
 * Used to examine the existance of features that can be enabled
 * or disabled at compile-time.
 * They used to be called XML_FEATURE_xxx but this clashed with Expat
 */
pub type xmlFeature = std::os::raw::c_uint;
pub const XML_WITH_NONE: xmlFeature = 99999;
pub const XML_WITH_LZMA: xmlFeature = 33;
pub const XML_WITH_ICU: xmlFeature = 32;
pub const XML_WITH_ZLIB: xmlFeature = 31;
pub const XML_WITH_DEBUG_RUN: xmlFeature = 30;
pub const XML_WITH_DEBUG_MEM: xmlFeature = 29;
pub const XML_WITH_DEBUG: xmlFeature = 28;
pub const XML_WITH_MODULES: xmlFeature = 27;
pub const XML_WITH_SCHEMATRON: xmlFeature = 26;
pub const XML_WITH_SCHEMAS: xmlFeature = 25;
pub const XML_WITH_EXPR: xmlFeature = 24;
pub const XML_WITH_AUTOMATA: xmlFeature = 23;
pub const XML_WITH_REGEXP: xmlFeature = 22;
pub const XML_WITH_UNICODE: xmlFeature = 21;
pub const XML_WITH_ISO8859X: xmlFeature = 20;
pub const XML_WITH_ICONV: xmlFeature = 19;
pub const XML_WITH_XINCLUDE: xmlFeature = 18;
pub const XML_WITH_XPTR: xmlFeature = 17;
pub const XML_WITH_XPATH: xmlFeature = 16;
pub const XML_WITH_CATALOG: xmlFeature = 15;
pub const XML_WITH_C14N: xmlFeature = 14;
pub const XML_WITH_LEGACY: xmlFeature = 13;
pub const XML_WITH_HTML: xmlFeature = 12;
pub const XML_WITH_VALID: xmlFeature = 11;
pub const XML_WITH_HTTP: xmlFeature = 10;
pub const XML_WITH_FTP: xmlFeature = 9;
pub const XML_WITH_SAX1: xmlFeature = 8;
pub const XML_WITH_WRITER: xmlFeature = 7;
pub const XML_WITH_PATTERN: xmlFeature = 6;
pub const XML_WITH_READER: xmlFeature = 5;
pub const XML_WITH_PUSH: xmlFeature = 4;
pub const XML_WITH_OUTPUT: xmlFeature = 3;
pub const XML_WITH_TREE: xmlFeature = 2;
pub const XML_WITH_THREAD: xmlFeature = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type xmlRegisterNodeFunc
    =
    Option<unsafe extern "C" fn(_: xmlNodePtr) -> ()>;
pub type xmlDeregisterNodeFunc
    =
    Option<unsafe extern "C" fn(_: xmlNodePtr) -> ()>;
pub type xmlRelaxNG = _xmlRelaxNG;
pub type xmlRelaxNGPtr = *mut xmlRelaxNG;
/* just to be sure of allocation size */
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
pub type xmlRelaxNGParserCtxt = _xmlRelaxNGParserCtxt;
pub type xmlRelaxNGParserCtxtPtr = *mut xmlRelaxNGParserCtxt;
pub type xmlRelaxNGValidCtxt = _xmlRelaxNGValidCtxt;
pub type xmlRelaxNGValidCtxtPtr = *mut xmlRelaxNGValidCtxt;
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
pub type xmlSchemaParserCtxt = _xmlSchemaParserCtxt;
pub type xmlSchemaParserCtxtPtr = *mut xmlSchemaParserCtxt;
pub type xmlSchemaValidCtxt = _xmlSchemaValidCtxt;
pub type xmlSchemaValidCtxtPtr = *mut xmlSchemaValidCtxt;
// #include <libxml/xmlwriter.h>
// from xmllint.c:
// from xmlreader.c:
/* the parsing mode */
/* when walking an existing doc */
/* is there any validation */
/* what structure were deallocated */
/* the parser context */
/* the parser SAX callbacks */
/* the input */
/* initial SAX callbacks */
/* idem */
/* idem */
/* idem */
/* base of the segment in the input */
/* current position in the input */
/* current node */
/* current attribute node */
/* depth of the current node */
/* fake xmlNs chld */
/* preserve the resulting document */
/* used to return const xmlChar * */
/* the context dictionary */
/* entity stack when traversing entities content */
/* Current Entity Ref Node */
/* Depth of the entities stack */
/* Max depth of the entities stack */
/* array of entities */
/* error handling */
/* callback function */
/* callback function user argument */
/* Handling of RelaxNG validation */
/* The Relax NG schemas */
/* The Relax NG validation context */
/* 1 if the context was provided by the user */
/* The number of errors detected */
/* the node if RNG not progressive */
/* Handling of Schemas validation */
/* The Schemas schemas */
/* The Schemas validation context */
/* 1 if the context was provided by the user */
/* The number of errors detected */
/* the schemas plug in SAX pipeline */
/* Handling of XInclude processing */
/* is xinclude asked for */
/* the xinclude name from dict */
/* the xinclude context */
/* counts for xinclude */
/* number of preserve patterns */
/* max preserve patterns */
/* array of preserve patterns */
/* level of preserves */
/* the set of options set */
/* Structured error handling */
/* callback function */
// from xmlschemas.c:
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlSchemaSAXPlug {
    pub magic: std::os::raw::c_uint,
    pub user_sax_ptr: *mut xmlSAXHandlerPtr,
    pub user_sax: xmlSAXHandlerPtr,
    pub user_data_ptr: *mut *mut std::os::raw::c_void,
    pub user_data: *mut std::os::raw::c_void,
    pub schemas_sax: xmlSAXHandler,
    pub ctxt: xmlSchemaValidCtxtPtr,
}
/*
 * Interface to insert Schemas SAX validation in a SAX stream
 */
pub type xmlSchemaSAXPlugStruct = _xmlSchemaSAXPlug;
pub type xmlSchemaSAXPlugPtr = *mut xmlSchemaSAXPlugStruct;
/*
 * Summary: the XMLReader implementation
 * Description: API of the XML streaming API based on C# interfaces.
 *
 * Copy: See Copyright for the status of this software.
 *
 * Author: Daniel Veillard
 */
/* *
 * xmlParserSeverities:
 *
 * How severe an error callback is when the per-reader error callback API
 * is used.
 */
pub type xmlParserSeverities = std::os::raw::c_uint;
pub const XML_PARSER_SEVERITY_ERROR: xmlParserSeverities = 4;
pub const XML_PARSER_SEVERITY_WARNING: xmlParserSeverities = 3;
pub const XML_PARSER_SEVERITY_VALIDITY_ERROR: xmlParserSeverities = 2;
pub const XML_PARSER_SEVERITY_VALIDITY_WARNING: xmlParserSeverities = 1;
/* *
 * xmlParserProperties:
 *
 * Some common options to use with xmlTextReaderSetParserProp, but it
 * is better to use xmlParserOption and the xmlReaderNewxxx and
 * xmlReaderForxxx APIs now.
 */
pub type C2RustUnnamed_0 = std::os::raw::c_uint;
pub const XML_PARSER_SUBST_ENTITIES: C2RustUnnamed_0 = 4;
pub const XML_PARSER_VALIDATE: C2RustUnnamed_0 = 3;
pub const XML_PARSER_DEFAULTATTRS: C2RustUnnamed_0 = 2;
pub const XML_PARSER_LOADDTD: C2RustUnnamed_0 = 1;
/* *
 * xmlReaderTypes:
 *
 * Predefined constants for the different types of nodes.
 */
pub type C2RustUnnamed_1 = std::os::raw::c_uint;
pub const XML_READER_TYPE_XML_DECLARATION: C2RustUnnamed_1 = 17;
pub const XML_READER_TYPE_END_ENTITY: C2RustUnnamed_1 = 16;
pub const XML_READER_TYPE_END_ELEMENT: C2RustUnnamed_1 = 15;
pub const XML_READER_TYPE_SIGNIFICANT_WHITESPACE: C2RustUnnamed_1 = 14;
pub const XML_READER_TYPE_WHITESPACE: C2RustUnnamed_1 = 13;
pub const XML_READER_TYPE_NOTATION: C2RustUnnamed_1 = 12;
pub const XML_READER_TYPE_DOCUMENT_FRAGMENT: C2RustUnnamed_1 = 11;
pub const XML_READER_TYPE_DOCUMENT_TYPE: C2RustUnnamed_1 = 10;
pub const XML_READER_TYPE_DOCUMENT: C2RustUnnamed_1 = 9;
pub const XML_READER_TYPE_COMMENT: C2RustUnnamed_1 = 8;
pub const XML_READER_TYPE_PROCESSING_INSTRUCTION: C2RustUnnamed_1 = 7;
pub const XML_READER_TYPE_ENTITY: C2RustUnnamed_1 = 6;
pub const XML_READER_TYPE_ENTITY_REFERENCE: C2RustUnnamed_1 = 5;
pub const XML_READER_TYPE_CDATA: C2RustUnnamed_1 = 4;
pub const XML_READER_TYPE_TEXT: C2RustUnnamed_1 = 3;
pub const XML_READER_TYPE_ATTRIBUTE: C2RustUnnamed_1 = 2;
pub const XML_READER_TYPE_ELEMENT: C2RustUnnamed_1 = 1;
pub const XML_READER_TYPE_NONE: C2RustUnnamed_1 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlTextReader {
    pub mode: std::os::raw::c_int,
    pub doc: xmlDocPtr,
    pub validate: xmlTextReaderValidate,
    pub allocs: std::os::raw::c_int,
    pub state: xmlTextReaderState,
    pub ctxt: xmlParserCtxtPtr,
    pub sax: xmlSAXHandlerPtr,
    pub input: xmlParserInputBufferPtr,
    pub startElement: startElementSAXFunc,
    pub endElement: endElementSAXFunc,
    pub startElementNs: startElementNsSAX2Func,
    pub endElementNs: endElementNsSAX2Func,
    pub characters: charactersSAXFunc,
    pub cdataBlock: cdataBlockSAXFunc,
    pub base: std::os::raw::c_uint,
    pub cur: std::os::raw::c_uint,
    pub node: xmlNodePtr,
    pub curnode: xmlNodePtr,
    pub depth: std::os::raw::c_int,
    pub faketext: xmlNodePtr,
    pub preserve: std::os::raw::c_int,
    pub buffer: xmlBufPtr,
    pub dict: xmlDictPtr,
    pub ent: xmlNodePtr,
    pub entNr: std::os::raw::c_int,
    pub entMax: std::os::raw::c_int,
    pub entTab: *mut xmlNodePtr,
    pub errorFunc: xmlTextReaderErrorFunc,
    pub errorFuncArg: *mut std::os::raw::c_void,
    pub rngSchemas: xmlRelaxNGPtr,
    pub rngValidCtxt: xmlRelaxNGValidCtxtPtr,
    pub rngPreserveCtxt: std::os::raw::c_int,
    pub rngValidErrors: std::os::raw::c_int,
    pub rngFullNode: xmlNodePtr,
    pub xsdSchemas: xmlSchemaPtr,
    pub xsdValidCtxt: xmlSchemaValidCtxtPtr,
    pub xsdPreserveCtxt: std::os::raw::c_int,
    pub xsdValidErrors: std::os::raw::c_int,
    pub xsdPlug: xmlSchemaSAXPlugPtr,
    pub xinclude: std::os::raw::c_int,
    pub xinclude_name: *const xmlChar,
    pub xincctxt: xmlXIncludeCtxtPtr,
    pub in_xinclude: std::os::raw::c_int,
    pub patternNr: std::os::raw::c_int,
    pub patternMax: std::os::raw::c_int,
    pub patternTab: *mut xmlPatternPtr,
    pub preserves: std::os::raw::c_int,
    pub parserFlags: std::os::raw::c_int,
    pub sErrorFunc: xmlStructuredErrorFunc,
}
/*
 * Summary: pattern expression handling
 * Description: allows to compile and test pattern expressions for nodes
 *              either in a tree or based on a parser state.
 *
 * Copy: See Copyright for the status of this software.
 *
 * Author: Daniel Veillard
 */
/* *
 * xmlPattern:
 *
 * A compiled (XPath based) pattern to select nodes
 */
pub type xmlPatternPtr = *mut xmlPattern;
pub type xmlPattern = _xmlPattern;
/*
 * Summary: implementation of XInclude
 * Description: API to handle XInclude processing,
 * implements the
 * World Wide Web Consortium Last Call Working Draft 10 November 2003
 * http://www.w3.org/TR/2003/WD-xinclude-20031110
 *
 * Copy: See Copyright for the status of this software.
 *
 * Author: Daniel Veillard
 */
/* *
 * XINCLUDE_NS:
 *
 * Macro defining the Xinclude namespace: http://www.w3.org/2003/XInclude
 */
/* *
 * XINCLUDE_OLD_NS:
 *
 * Macro defining the draft Xinclude namespace: http://www.w3.org/2001/XInclude
 */
/* *
 * XINCLUDE_NODE:
 *
 * Macro defining "include"
 */
/* *
 * XINCLUDE_FALLBACK:
 *
 * Macro defining "fallback"
 */
/* *
 * XINCLUDE_HREF:
 *
 * Macro defining "href"
 */
/* *
 * XINCLUDE_PARSE:
 *
 * Macro defining "parse"
 */
/* *
 * XINCLUDE_PARSE_XML:
 *
 * Macro defining "xml"
 */
/* *
 * XINCLUDE_PARSE_TEXT:
 *
 * Macro defining "text"
 */
/* *
 * XINCLUDE_PARSE_ENCODING:
 *
 * Macro defining "encoding"
 */
/* *
 * XINCLUDE_PARSE_XPOINTER:
 *
 * Macro defining "xpointer"
 */
pub type xmlXIncludeCtxtPtr = *mut xmlXIncludeCtxt;
pub type xmlXIncludeCtxt = _xmlXIncludeCtxt;
/* *
 * xmlTextReader:
 *
 * Structure for an xmlReader context.
 */
/* *
 * xmlTextReaderPtr:
 *
 * Pointer to an xmlReader context.
 */
/*
 * Constructors & Destructor
 */
/*
 * Iterators
 */
/*
 * Attributes of the node
 */
/*
 * use the Const version of the routine for
 * better performance and simpler code
 */
/*
 * Methods of the XmlTextReader
 */
/*
 * Extensions
 */
/* LIBXML_PATTERN_ENABLED */
/*
 * Index lookup
 */
/*
 * New more complete APIs for simpler creation and reuse of readers
 */
/*
 * Error handling extensions
 */
/* *
 * xmlTextReaderErrorFunc:
 * @arg: the user argument
 * @msg: the message
 * @severity: the severity of the error
 * @locator: a locator indicating where the error occurred
 *
 * Signature of an error callback from a reader parser
 */
pub type xmlTextReaderErrorFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const std::os::raw::c_char,
                                _: xmlParserSeverities,
                                _: xmlTextReaderLocatorPtr) -> ()>;
pub type xmlTextReaderLocatorPtr = *mut std::os::raw::c_void;
pub type xmlTextReaderState = std::os::raw::c_int;
pub const XML_TEXTREADER_ERROR: xmlTextReaderState = 6;
pub const XML_TEXTREADER_DONE: xmlTextReaderState = 5;
pub const XML_TEXTREADER_BACKTRACK: xmlTextReaderState = 4;
pub const XML_TEXTREADER_EMPTY: xmlTextReaderState = 3;
pub const XML_TEXTREADER_END: xmlTextReaderState = 2;
pub const XML_TEXTREADER_ELEMENT: xmlTextReaderState = 1;
pub const XML_TEXTREADER_START: xmlTextReaderState = 0;
pub const XML_TEXTREADER_NONE: xmlTextReaderState = -1;
pub type xmlTextReaderValidate = std::os::raw::c_uint;
pub const XML_TEXTREADER_VALIDATE_XSD: xmlTextReaderValidate = 4;
pub const XML_TEXTREADER_VALIDATE_RNG: xmlTextReaderValidate = 2;
pub const XML_TEXTREADER_VALIDATE_DTD: xmlTextReaderValidate = 1;
pub const XML_TEXTREADER_NOT_VALIDATE: xmlTextReaderValidate = 0;
pub type xmlTextReader = _xmlTextReader;
pub type xmlTextReaderPtr = *mut xmlTextReader;
pub type xmlStreamCtxt = _xmlStreamCtxt;
pub type xmlStreamCtxtPtr = *mut xmlStreamCtxt;
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
pub type htmlParserCtxtPtr = xmlParserCtxtPtr;
pub type htmlSAXHandlerPtr = xmlSAXHandlerPtr;
pub type htmlDocPtr = xmlDocPtr;
/*
 * New set of simpler/more flexible APIs
 */
/* *
 * xmlParserOption:
 *
 * This is the set of XML parser options that can be passed down
 * to the xmlReadDoc() and similar calls.
 */
pub type C2RustUnnamed_2 = std::os::raw::c_uint;
/* ignore internal document encoding hint */
/* compact small text nodes */
pub const HTML_PARSE_IGNORE_ENC: C2RustUnnamed_2 = 2097152;
/* Do not add implied html/body... elements */
pub const HTML_PARSE_COMPACT: C2RustUnnamed_2 = 65536;
/* Forbid network access */
pub const HTML_PARSE_NOIMPLIED: C2RustUnnamed_2 = 8192;
/* remove blank nodes */
pub const HTML_PARSE_NONET: C2RustUnnamed_2 = 2048;
/* pedantic error reporting */
pub const HTML_PARSE_NOBLANKS: C2RustUnnamed_2 = 256;
/* suppress warning reports */
pub const HTML_PARSE_PEDANTIC: C2RustUnnamed_2 = 128;
/* suppress error reports */
pub const HTML_PARSE_NOWARNING: C2RustUnnamed_2 = 64;
/* do not default a doctype if not found */
pub const HTML_PARSE_NOERROR: C2RustUnnamed_2 = 32;
/* Relaxed parsing */
pub const HTML_PARSE_NODEFDTD: C2RustUnnamed_2 = 4;
pub const HTML_PARSE_RECOVER: C2RustUnnamed_2 = 1;
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
pub type xmlShellReadlineFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_char) -> *mut std::os::raw::c_char>;
pub type C2RustUnnamed_3 = std::os::raw::c_uint;
pub const XML_SCHEMATRON_OUT_IO: C2RustUnnamed_3 = 1024;
pub const XML_SCHEMATRON_OUT_BUFFER: C2RustUnnamed_3 = 512;
pub const XML_SCHEMATRON_OUT_FILE: C2RustUnnamed_3 = 256;
pub const XML_SCHEMATRON_OUT_ERROR: C2RustUnnamed_3 = 8;
pub const XML_SCHEMATRON_OUT_XML: C2RustUnnamed_3 = 4;
pub const XML_SCHEMATRON_OUT_TEXT: C2RustUnnamed_3 = 2;
pub const XML_SCHEMATRON_OUT_QUIET: C2RustUnnamed_3 = 1;
/* *
 * The schemas related types are kept internal
 */
pub type xmlSchematron = _xmlSchematron;
pub type xmlSchematronPtr = *mut xmlSchematron;
/* *
 * A schemas validation context
 */
pub type xmlSchematronParserCtxt = _xmlSchematronParserCtxt;
pub type xmlSchematronParserCtxtPtr = *mut xmlSchematronParserCtxt;
pub type xmlSchematronValidCtxt = _xmlSchematronValidCtxt;
pub type xmlSchematronValidCtxtPtr = *mut xmlSchematronValidCtxt;
pub type C2RustUnnamed_4 = std::os::raw::c_uint;
pub const XML_C14N_1_1: C2RustUnnamed_4 = 2;
pub const XML_C14N_EXCLUSIVE_1_0: C2RustUnnamed_4 = 1;
pub const XML_C14N_1_0: C2RustUnnamed_4 = 0;
pub type C2RustUnnamed_5 = std::os::raw::c_uint;
pub const XML_SAVE_WSNONSIG: C2RustUnnamed_5 = 128;
pub const XML_SAVE_AS_HTML: C2RustUnnamed_5 = 64;
pub const XML_SAVE_AS_XML: C2RustUnnamed_5 = 32;
pub const XML_SAVE_XHTML: C2RustUnnamed_5 = 16;
pub const XML_SAVE_NO_XHTML: C2RustUnnamed_5 = 8;
pub const XML_SAVE_NO_EMPTY: C2RustUnnamed_5 = 4;
pub const XML_SAVE_NO_DECL: C2RustUnnamed_5 = 2;
pub const XML_SAVE_FORMAT: C2RustUnnamed_5 = 1;
pub type xmlSaveCtxt = _xmlSaveCtxt;
pub type xmlSaveCtxtPtr = *mut xmlSaveCtxt;
pub type xmllintReturnCode = std::os::raw::c_uint;
pub const XMLLINT_ERR_XPATH: xmllintReturnCode = 10;
pub const XMLLINT_ERR_MEM: xmllintReturnCode = 9;
pub const XMLLINT_ERR_RDREGIS: xmllintReturnCode = 8;
pub const XMLLINT_ERR_SCHEMAPAT: xmllintReturnCode = 7;
pub const XMLLINT_ERR_OUT: xmllintReturnCode = 6;
pub const XMLLINT_ERR_SCHEMACOMP: xmllintReturnCode = 5;
pub const XMLLINT_ERR_RDFILE: xmllintReturnCode = 4;
pub const XMLLINT_ERR_VALID: xmllintReturnCode = 3;
pub const XMLLINT_ERR_DTD: xmllintReturnCode = 2;
pub const XMLLINT_ERR_UNCLASS: xmllintReturnCode = 1;
pub const XMLLINT_RETURN_OK: xmllintReturnCode = 0;
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const std::os::raw::c_char) -> std::os::raw::c_int {
    return strtol(__nptr, 0 as *mut std::os::raw::c_void as *mut *mut std::os::raw::c_char,
                  10 as std::os::raw::c_int) as std::os::raw::c_int;
}
#[inline]
unsafe extern "C" fn stat(mut __path: *const std::os::raw::c_char,
                          mut __statbuf: *mut stat) -> std::os::raw::c_int {
    return __xstat(1 as std::os::raw::c_int, __path, __statbuf);
}
static mut shell: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut debugent: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut debug: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut maxmem: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut copy: std::os::raw::c_int = 0 as std::os::raw::c_int;
/* LIBXML_TREE_ENABLED */
static mut recovery: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut noent: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut noenc: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut noblanks: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut nowrap: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut format: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut output: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
static mut compress: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut oldout: std::os::raw::c_int = 0 as std::os::raw::c_int;
/* LIBXML_OUTPUT_ENABLED */
static mut valid: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut postvalid: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut dtdvalid: *mut std::os::raw::c_char =
    0 as *const std::os::raw::c_char as *mut std::os::raw::c_char;
static mut dtdvalidfpi: *mut std::os::raw::c_char =
    0 as *const std::os::raw::c_char as *mut std::os::raw::c_char;
static mut relaxng: *mut std::os::raw::c_char =
    0 as *const std::os::raw::c_char as *mut std::os::raw::c_char;
static mut relaxngschemas: xmlRelaxNGPtr =
    0 as *const xmlRelaxNG as xmlRelaxNGPtr;
static mut schema: *mut std::os::raw::c_char =
    0 as *const std::os::raw::c_char as *mut std::os::raw::c_char;
static mut wxschemas: xmlSchemaPtr = 0 as *const xmlSchema as xmlSchemaPtr;
static mut schematron: *mut std::os::raw::c_char =
    0 as *const std::os::raw::c_char as *mut std::os::raw::c_char;
static mut wxschematron: xmlSchematronPtr =
    0 as *const xmlSchematron as xmlSchematronPtr;
static mut repeat: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut insert: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut html: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut xmlout: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut htmlout: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut nodefdtd: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut push: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut pushsize: std::os::raw::c_int = 4096 as std::os::raw::c_int;
/* LIBXML_PUSH_ENABLED */
static mut memory: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut testIO: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut encoding: *mut std::os::raw::c_char =
    0 as *const std::os::raw::c_char as *mut std::os::raw::c_char;
static mut xinclude: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut dtdattrs: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut loaddtd: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut timing: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut generate: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut dropdtd: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut catalogs: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut nocatalogs: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut canonical: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut canonical_11: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut exc_canonical: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut stream: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut walker: std::os::raw::c_int = 0 as std::os::raw::c_int;
/* LIBXML_READER_ENABLED */
static mut chkregister: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut nbregister: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut sax1: std::os::raw::c_int = 0 as std::os::raw::c_int;
/* LIBXML_SAX1_ENABLED */
static mut pattern: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
static mut patternc: xmlPatternPtr = 0 as *const xmlPattern as xmlPatternPtr;
static mut patstream: xmlStreamCtxtPtr =
    0 as *const xmlStreamCtxt as xmlStreamCtxtPtr;
static mut xpathquery: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
static mut options: std::os::raw::c_int =
    XML_PARSE_COMPACT as std::os::raw::c_int | XML_PARSE_BIG_LINES as std::os::raw::c_int;
static mut sax: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut oldxml10: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut paths: [*mut xmlChar; 65] =
    [0 as *const xmlChar as *mut xmlChar; 65];
static mut nbpaths: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut load_trace: std::os::raw::c_int = 0 as std::os::raw::c_int;
unsafe extern "C" fn parsePath(mut path: *const xmlChar) {
    let mut cur: *const xmlChar = 0 as *const xmlChar;
    if path.is_null() { return }
    while *path as std::os::raw::c_int != 0 as std::os::raw::c_int {
        if nbpaths >= 64 as std::os::raw::c_int {
            fprintf(stderr,
                    b"MAX_PATHS reached: too many paths\n\x00" as *const u8 as
                        *const std::os::raw::c_char);
            return
        }
        cur = path;
        while *cur as std::os::raw::c_int == ' ' as i32 ||
                  *cur as std::os::raw::c_int == ':' as i32 {
            cur = cur.offset(1)
        }
        path = cur;
        while *cur as std::os::raw::c_int != 0 as std::os::raw::c_int &&
                  *cur as std::os::raw::c_int != ' ' as i32 &&
                  *cur as std::os::raw::c_int != ':' as i32 {
            cur = cur.offset(1)
        }
        if cur != path {
            paths[nbpaths as usize] =
                xmlStrndup(path,
                           cur.offset_from(path) as std::os::raw::c_long as
                               std::os::raw::c_int);
            if !paths[nbpaths as usize].is_null() { nbpaths += 1 }
            path = cur
        }
    };
}
static mut defaultEntityLoader: xmlExternalEntityLoader = None;
unsafe extern "C" fn xmllintExternalEntityLoader(mut URL: *const std::os::raw::c_char,
                                                 mut ID: *const std::os::raw::c_char,
                                                 mut ctxt: xmlParserCtxtPtr)
 -> xmlParserInputPtr {
    let mut ret: xmlParserInputPtr = 0 as *mut xmlParserInput;
    let mut warning: warningSAXFunc = None;
    let mut err: errorSAXFunc = None;
    let mut i: std::os::raw::c_int = 0;
    let mut lastsegment: *const std::os::raw::c_char = URL;
    let mut iter: *const std::os::raw::c_char = URL;
    if nbpaths > 0 as std::os::raw::c_int && !iter.is_null() {
        while *iter as std::os::raw::c_int != 0 as std::os::raw::c_int {
            if *iter as std::os::raw::c_int == '/' as i32 {
                lastsegment = iter.offset(1 as std::os::raw::c_int as isize)
            }
            iter = iter.offset(1)
        }
    }
    if !ctxt.is_null() && !(*ctxt).sax.is_null() {
        warning = (*(*ctxt).sax).warning;
        err = (*(*ctxt).sax).error;
        (*(*ctxt).sax).warning = None;
        (*(*ctxt).sax).error = None
    }
    if defaultEntityLoader.is_some() {
        ret =
            defaultEntityLoader.expect("non-null function pointer")(URL, ID,
                                                                    ctxt);
        if !ret.is_null() {
            if warning.is_some() { (*(*ctxt).sax).warning = warning }
            if err.is_some() { (*(*ctxt).sax).error = err }
            if load_trace != 0 {
                fprintf(stderr,
                        b"Loaded URL=\"%s\" ID=\"%s\"\n\x00" as *const u8 as
                            *const std::os::raw::c_char,
                        if !URL.is_null() {
                            URL
                        } else {
                            b"(null)\x00" as *const u8 as *const std::os::raw::c_char
                        },
                        if !ID.is_null() {
                            ID
                        } else {
                            b"(null)\x00" as *const u8 as *const std::os::raw::c_char
                        });
            }
            return ret
        }
    }
    i = 0 as std::os::raw::c_int;
    while i < nbpaths {
        let mut newURL: *mut xmlChar = 0 as *mut xmlChar;
        newURL = xmlStrdup(paths[i as usize] as *const xmlChar);
        newURL =
            xmlStrcat(newURL,
                      b"/\x00" as *const u8 as *const std::os::raw::c_char as
                          *const xmlChar);
        newURL = xmlStrcat(newURL, lastsegment as *const xmlChar);
        if !newURL.is_null() {
            ret =
                defaultEntityLoader.expect("non-null function pointer")(newURL
                                                                            as
                                                                            *const std::os::raw::c_char,
                                                                        ID,
                                                                        ctxt);
            if !ret.is_null() {
                if warning.is_some() { (*(*ctxt).sax).warning = warning }
                if err.is_some() { (*(*ctxt).sax).error = err }
                if load_trace != 0 {
                    fprintf(stderr,
                            b"Loaded URL=\"%s\" ID=\"%s\"\n\x00" as *const u8
                                as *const std::os::raw::c_char, newURL,
                            if !ID.is_null() {
                                ID
                            } else {
                                b"(null)\x00" as *const u8 as
                                    *const std::os::raw::c_char
                            });
                }
                xmlFree.expect("non-null function pointer")(newURL as
                                                                *mut std::os::raw::c_void);
                return ret
            }
            xmlFree.expect("non-null function pointer")(newURL as
                                                            *mut std::os::raw::c_void);
        }
        i += 1
    }
    if err.is_some() { (*(*ctxt).sax).error = err }
    if warning.is_some() {
        (*(*ctxt).sax).warning = warning;
        if !URL.is_null() {
            warning.expect("non-null function pointer")(ctxt as
                                                            *mut std::os::raw::c_void,
                                                        b"failed to load external entity \"%s\"\n\x00"
                                                            as *const u8 as
                                                            *const std::os::raw::c_char,
                                                        URL);
        } else if !ID.is_null() {
            warning.expect("non-null function pointer")(ctxt as
                                                            *mut std::os::raw::c_void,
                                                        b"failed to load external entity \"%s\"\n\x00"
                                                            as *const u8 as
                                                            *const std::os::raw::c_char,
                                                        ID);
        }
    }
    return 0 as xmlParserInputPtr;
}
/* ***********************************************************************
 *									*
 * Memory allocation consumption debugging				*
 *									*
 ************************************************************************/
unsafe extern "C" fn OOM() {
    fprintf(stderr,
            b"Ran out of memory needs > %d bytes\n\x00" as *const u8 as
                *const std::os::raw::c_char, maxmem);
    xmllint_progresult = XMLLINT_ERR_MEM;
}
unsafe extern "C" fn myFreeFunc(mut mem: *mut std::os::raw::c_void) {
    xmlMemFree(mem);
}
unsafe extern "C" fn myMallocFunc(mut size: size_t) -> *mut std::os::raw::c_void {
    let mut ret: *mut std::os::raw::c_void = 0 as *mut std::os::raw::c_void;
    ret = xmlMemMalloc(size);
    if !ret.is_null() {
        if xmlMemUsed() > maxmem {
            OOM();
            xmlMemFree(ret);
            return 0 as *mut std::os::raw::c_void
        }
    }
    return ret;
}
unsafe extern "C" fn myReallocFunc(mut mem: *mut std::os::raw::c_void,
                                   mut size: size_t) -> *mut std::os::raw::c_void {
    let mut ret: *mut std::os::raw::c_void = 0 as *mut std::os::raw::c_void;
    ret = xmlMemRealloc(mem, size);
    if !ret.is_null() {
        if xmlMemUsed() > maxmem {
            OOM();
            xmlMemFree(ret);
            return 0 as *mut std::os::raw::c_void
        }
    }
    return ret;
}
unsafe extern "C" fn myStrdupFunc(mut str: *const std::os::raw::c_char)
 -> *mut std::os::raw::c_char {
    let mut ret: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    ret = xmlMemoryStrdup(str);
    if !ret.is_null() {
        if xmlMemUsed() > maxmem {
            OOM();
            xmlFree.expect("non-null function pointer")(ret as
                                                            *mut std::os::raw::c_void);
            return 0 as *mut std::os::raw::c_char
        }
    }
    return ret;
}
/*
 * xmllint_startTimer: call where you want to start timing
 */
unsafe extern "C" fn xmllint_startTimer() {
    gettimeofday(&mut xmllint_begin, 0 as *mut std::os::raw::c_void);
}
// void
// xmlHTMLEncodeSend(void) {
//     char *result;
//     result = (char *) xmlEncodeEntitiesReentrant(NULL, BAD_CAST xmllint_buffer);
//     if (result) {
// 	xmlGenericError(xmlGenericErrorContext, "%s", result);
// 	xmlFree(result);
//     }
//     xmllint_buffer[0] = 0;
// }
/* *
 * xmlHTMLPrintFileInfo:
 * @input:  an xmlParserInputPtr input
 *
 * Displays the associated file and line informations for the current input
 */
// void
// xmlHTMLPrintFileInfo(xmlParserInputPtr input) {
//     int len;
//     xmlGenericError(xmlGenericErrorContext, "<p>");
//     len = strlen(xmllint_buffer);
//     if (input != NULL) {
// 	if (input->filename) {
// 	    snprintf(&xmllint_buffer[len], sizeof(xmllint_buffer) - len, "%s:%d: ", input->filename,
// 		    input->line);
// 	} else {
// 	    snprintf(&xmllint_buffer[len], sizeof(xmllint_buffer) - len, "Entity: line %d: ", input->line);
// 	}
//     }
//     xmlHTMLEncodeSend();
// }
/* *
 * xmlHTMLPrintFileContext:
 * @input:  an xmlParserInputPtr input
 *
 * Displays current context within the input content for error tracking
 */
// void
// xmlHTMLPrintFileContext(xmlParserInputPtr input) {
//     const xmlChar *cur, *base;
//     int len;
//     int n;
//     if (input == NULL) return;
//     xmlGenericError(xmlGenericErrorContext, "<pre>\n");
//     cur = input->cur;
//     base = input->base;
//     while ((cur > base) && ((*cur == '\n') || (*cur == '\r'))) {
// 	cur--;
//     }
//     n = 0;
//     while ((n++ < 80) && (cur > base) && (*cur != '\n') && (*cur != '\r'))
//         cur--;
//     if ((*cur == '\n') || (*cur == '\r')) cur++;
//     base = cur;
//     n = 0;
//     while ((*cur != 0) && (*cur != '\n') && (*cur != '\r') && (n < 79)) {
// 	len = strlen(xmllint_buffer);
//         snprintf(&xmllint_buffer[len], sizeof(xmllint_buffer) - len, "%c",
// 		    (unsigned char) *cur++);
// 	n++;
//     }
//     len = strlen(xmllint_buffer);
//     snprintf(&xmllint_buffer[len], sizeof(xmllint_buffer) - len, "\n");
//     cur = input->cur;
//     while ((*cur == '\n') || (*cur == '\r'))
// 	cur--;
//     n = 0;
//     while ((cur != base) && (n++ < 80)) {
// 	len = strlen(xmllint_buffer);
//         snprintf(&xmllint_buffer[len], sizeof(xmllint_buffer) - len, " ");
//         base++;
//     }
//     len = strlen(xmllint_buffer);
//     snprintf(&xmllint_buffer[len], sizeof(xmllint_buffer) - len, "^\n");
//     xmlHTMLEncodeSend();
//     xmlGenericError(xmlGenericErrorContext, "</pre>");
// }
/* *
 * xmlHTMLError:
 * @ctx:  an XML parser context
 * @msg:  the message to display/transmit
 * @...:  extra parameters for the message display
 *
 * Display and format an error messages, gives file, line, position and
 * extra parameters.
 */
// static void XMLCDECL LIBXML_ATTR_FORMAT(2,3)
// xmlHTMLError(void *ctx, const char *msg, ...)
// {
//     xmlParserCtxtPtr ctxt = (xmlParserCtxtPtr) ctx;
//     xmlParserInputPtr input;
//     va_list args;
//     int len;
//     xmllint_buffer[0] = 0;
//     input = ctxt->input;
//     if ((input != NULL) && (input->filename == NULL) && (ctxt->inputNr > 1)) {
//         input = ctxt->inputTab[ctxt->inputNr - 2];
//     }
//     xmlHTMLPrintFileInfo(input);
//     xmlGenericError(xmlGenericErrorContext, "<b>error</b>: ");
//     va_start(args, msg);
//     len = strlen(xmllint_buffer);
//     vsnprintf(&xmllint_buffer[len],  sizeof(xmllint_buffer) - len, msg, args);
//     va_end(args);
//     xmlHTMLEncodeSend();
//     xmlGenericError(xmlGenericErrorContext, "</p>\n");
//     xmlHTMLPrintFileContext(input);
//     xmlHTMLEncodeSend();
// }
/* *
 * xmlHTMLWarning:
 * @ctx:  an XML parser context
 * @msg:  the message to display/transmit
 * @...:  extra parameters for the message display
 *
 * Display and format a warning messages, gives file, line, position and
 * extra parameters.
 */
// static void XMLCDECL LIBXML_ATTR_FORMAT(2,3)
// xmlHTMLWarning(void *ctx, const char *msg, ...)
// {
//     xmlParserCtxtPtr ctxt = (xmlParserCtxtPtr) ctx;
//     xmlParserInputPtr input;
//     va_list args;
//     int len;
//     xmllint_buffer[0] = 0;
//     input = ctxt->input;
//     if ((input != NULL) && (input->filename == NULL) && (ctxt->inputNr > 1)) {
//         input = ctxt->inputTab[ctxt->inputNr - 2];
//     }
//     xmlHTMLPrintFileInfo(input);
//     xmlGenericError(xmlGenericErrorContext, "<b>warning</b>: ");
//     va_start(args, msg);
//     len = strlen(xmllint_buffer);
//     vsnprintf(&xmllint_buffer[len],  sizeof(xmllint_buffer) - len, msg, args);
//     va_end(args);
//     xmlHTMLEncodeSend();
//     xmlGenericError(xmlGenericErrorContext, "</p>\n");
//     xmlHTMLPrintFileContext(input);
//     xmlHTMLEncodeSend();
// }
/* *
 * xmlHTMLValidityError:
 * @ctx:  an XML parser context
 * @msg:  the message to display/transmit
 * @...:  extra parameters for the message display
 *
 * Display and format an validity error messages, gives file,
 * line, position and extra parameters.
 */
// static void XMLCDECL LIBXML_ATTR_FORMAT(2,3)
// xmlHTMLValidityError(void *ctx, const char *msg, ...)
// {
//     xmlParserCtxtPtr ctxt = (xmlParserCtxtPtr) ctx;
//     xmlParserInputPtr input;
//     va_list args;
//     int len;
//     xmllint_buffer[0] = 0;
//     input = ctxt->input;
//     if ((input->filename == NULL) && (ctxt->inputNr > 1))
//         input = ctxt->inputTab[ctxt->inputNr - 2];
//     xmlHTMLPrintFileInfo(input);
//     xmlGenericError(xmlGenericErrorContext, "<b>validity error</b>: ");
//     len = strlen(xmllint_buffer);
//     va_start(args, msg);
//     vsnprintf(&xmllint_buffer[len],  sizeof(xmllint_buffer) - len, msg, args);
//     va_end(args);
//     xmlHTMLEncodeSend();
//     xmlGenericError(xmlGenericErrorContext, "</p>\n");
//     xmlHTMLPrintFileContext(input);
//     xmlHTMLEncodeSend();
//     xmllint_progresult = XMLLINT_ERR_VALID;
// }
/* *
 * xmlHTMLValidityWarning:
 * @ctx:  an XML parser context
 * @msg:  the message to display/transmit
 * @...:  extra parameters for the message display
 *
 * Display and format a validity warning messages, gives file, line,
 * position and extra parameters.
 */
// static void XMLCDECL LIBXML_ATTR_FORMAT(2,3)
// xmlHTMLValidityWarning(void *ctx, const char *msg, ...)
// {
//     xmlParserCtxtPtr ctxt = (xmlParserCtxtPtr) ctx;
//     xmlParserInputPtr input;
//     va_list args;
//     int len;
//     xmllint_buffer[0] = 0;
//     input = ctxt->input;
//     if ((input->filename == NULL) && (ctxt->inputNr > 1))
//         input = ctxt->inputTab[ctxt->inputNr - 2];
//     xmlHTMLPrintFileInfo(input);
//     xmlGenericError(xmlGenericErrorContext, "<b>validity warning</b>: ");
//     va_start(args, msg);
//     len = strlen(xmllint_buffer);
//     vsnprintf(&xmllint_buffer[len],  sizeof(xmllint_buffer) - len, msg, args);
//     va_end(args);
//     xmlHTMLEncodeSend();
//     xmlGenericError(xmlGenericErrorContext, "</p>\n");
//     xmlHTMLPrintFileContext(input);
//     xmlHTMLEncodeSend();
// }
/* ***********************************************************************
 *									*
 *			Shell Interface					*
 *									*
 ************************************************************************/
/* *
 * xmlShellReadline:
 * @prompt:  the prompt value
 *
 * Read a string
 *
 * Returns a pointer to it or NULL on EOF the caller is expected to
 *     free the returned string.
 */
unsafe extern "C" fn xmlShellReadline(mut prompt: *mut std::os::raw::c_char)
 -> *mut std::os::raw::c_char {
    let mut line_read: [std::os::raw::c_char; 501] = [0; 501];
    let mut ret: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut len: std::os::raw::c_int = 0;
    if !prompt.is_null() {
        fprintf(stdout, b"%s\x00" as *const u8 as *const std::os::raw::c_char,
                prompt);
    }
    fflush(stdout);
    if fgets(line_read.as_mut_ptr(), 500 as std::os::raw::c_int, stdin).is_null() {
        return 0 as *mut std::os::raw::c_char
    }
    line_read[500 as std::os::raw::c_int as usize] = 0 as std::os::raw::c_int as std::os::raw::c_char;
    len = strlen(line_read.as_mut_ptr()) as std::os::raw::c_int;
    ret =
        malloc((len + 1 as std::os::raw::c_int) as std::os::raw::c_ulong) as
            *mut std::os::raw::c_char;
    if !ret.is_null() {
        memcpy(ret as *mut std::os::raw::c_void,
               line_read.as_mut_ptr() as *const std::os::raw::c_void,
               (len + 1 as std::os::raw::c_int) as std::os::raw::c_ulong);
    }
    return ret;
}
/* LIBXML_XPATH_ENABLED */
/* LIBXML_DEBUG_ENABLED */
/* ***********************************************************************
 *									*
 *			I/O Interfaces					*
 *									*
 ************************************************************************/
unsafe extern "C" fn myRead(mut f: *mut std::os::raw::c_void,
                            mut buf: *mut std::os::raw::c_char, mut len: std::os::raw::c_int)
 -> std::os::raw::c_int {
    return fread(buf as *mut std::os::raw::c_void, 1 as std::os::raw::c_int as size_t,
                 len as size_t, f as *mut FILE) as std::os::raw::c_int;
}
unsafe extern "C" fn myClose(mut context: *mut std::os::raw::c_void) -> std::os::raw::c_int {
    let mut f: *mut FILE = context as *mut FILE;
    if f == stdin { return 0 as std::os::raw::c_int }
    return fclose(f);
}
/* ***********************************************************************
 *									*
 *			SAX based tests					*
 *									*
 ************************************************************************/
/*
 * empty SAX block
 */
static mut emptySAXHandlerStruct: xmlSAXHandler =
    {
        let mut init =
            _xmlSAXHandler{internalSubset: None,
                           isStandalone: None,
                           hasInternalSubset: None,
                           hasExternalSubset: None,
                           resolveEntity: None,
                           getEntity: None,
                           entityDecl: None,
                           notationDecl: None,
                           attributeDecl: None,
                           elementDecl: None,
                           unparsedEntityDecl: None,
                           setDocumentLocator: None,
                           startDocument: None,
                           endDocument: None,
                           startElement: None,
                           endElement: None,
                           reference: None,
                           characters: None,
                           ignorableWhitespace: None,
                           processingInstruction: None,
                           comment: None,
                           warning: None,
                           error: None,
                           fatalError: None,
                           getParameterEntity: None,
                           cdataBlock: None,
                           externalSubset: None,
                           initialized: 0xdeedbeaf as std::os::raw::c_uint,
                           _private:
                               0 as *const std::os::raw::c_void as *mut std::os::raw::c_void,
                           startElementNs: None,
                           endElementNs: None,
                           serror: None,};
        init
    };
static mut emptySAXHandler: xmlSAXHandlerPtr =
    unsafe {
        &emptySAXHandlerStruct as *const xmlSAXHandler as *mut xmlSAXHandler
    };
/* *
 * isStandaloneDebug:
 * @ctxt:  An XML parser context
 *
 * Is this document tagged standalone ?
 *
 * Returns 1 if true
 */
unsafe extern "C" fn isStandaloneDebug(mut ctx: *mut std::os::raw::c_void)
 -> std::os::raw::c_int {
    xmllint_callbacks += 1;
    if xmllint_noout != 0 { return 0 as std::os::raw::c_int }
    fprintf(stdout,
            b"SAX.isStandalone()\n\x00" as *const u8 as *const std::os::raw::c_char);
    return 0 as std::os::raw::c_int;
}
/* *
 * hasInternalSubsetDebug:
 * @ctxt:  An XML parser context
 *
 * Does this document has an internal subset
 *
 * Returns 1 if true
 */
unsafe extern "C" fn hasInternalSubsetDebug(mut ctx: *mut std::os::raw::c_void)
 -> std::os::raw::c_int {
    xmllint_callbacks += 1;
    if xmllint_noout != 0 { return 0 as std::os::raw::c_int }
    fprintf(stdout,
            b"SAX.hasInternalSubset()\n\x00" as *const u8 as
                *const std::os::raw::c_char);
    return 0 as std::os::raw::c_int;
}
/* *
 * hasExternalSubsetDebug:
 * @ctxt:  An XML parser context
 *
 * Does this document has an external subset
 *
 * Returns 1 if true
 */
unsafe extern "C" fn hasExternalSubsetDebug(mut ctx: *mut std::os::raw::c_void)
 -> std::os::raw::c_int {
    xmllint_callbacks += 1;
    if xmllint_noout != 0 { return 0 as std::os::raw::c_int }
    fprintf(stdout,
            b"SAX.hasExternalSubset()\n\x00" as *const u8 as
                *const std::os::raw::c_char);
    return 0 as std::os::raw::c_int;
}
/* *
 * internalSubsetDebug:
 * @ctxt:  An XML parser context
 *
 * Does this document has an internal subset
 */
unsafe extern "C" fn internalSubsetDebug(mut ctx: *mut std::os::raw::c_void,
                                         mut name: *const xmlChar,
                                         mut ExternalID: *const xmlChar,
                                         mut SystemID: *const xmlChar) {
    xmllint_callbacks += 1;
    if xmllint_noout != 0 { return }
    fprintf(stdout,
            b"SAX.internalSubset(%s,\x00" as *const u8 as *const std::os::raw::c_char,
            name);
    if ExternalID.is_null() {
        fprintf(stdout, b" ,\x00" as *const u8 as *const std::os::raw::c_char);
    } else {
        fprintf(stdout, b" %s,\x00" as *const u8 as *const std::os::raw::c_char,
                ExternalID);
    }
    if SystemID.is_null() {
        fprintf(stdout, b" )\n\x00" as *const u8 as *const std::os::raw::c_char);
    } else {
        fprintf(stdout, b" %s)\n\x00" as *const u8 as *const std::os::raw::c_char,
                SystemID);
    };
}
/* *
 * externalSubsetDebug:
 * @ctxt:  An XML parser context
 *
 * Does this document has an external subset
 */
unsafe extern "C" fn externalSubsetDebug(mut ctx: *mut std::os::raw::c_void,
                                         mut name: *const xmlChar,
                                         mut ExternalID: *const xmlChar,
                                         mut SystemID: *const xmlChar) {
    xmllint_callbacks += 1;
    if xmllint_noout != 0 { return }
    fprintf(stdout,
            b"SAX.externalSubset(%s,\x00" as *const u8 as *const std::os::raw::c_char,
            name);
    if ExternalID.is_null() {
        fprintf(stdout, b" ,\x00" as *const u8 as *const std::os::raw::c_char);
    } else {
        fprintf(stdout, b" %s,\x00" as *const u8 as *const std::os::raw::c_char,
                ExternalID);
    }
    if SystemID.is_null() {
        fprintf(stdout, b" )\n\x00" as *const u8 as *const std::os::raw::c_char);
    } else {
        fprintf(stdout, b" %s)\n\x00" as *const u8 as *const std::os::raw::c_char,
                SystemID);
    };
}
/* *
 * resolveEntityDebug:
 * @ctxt:  An XML parser context
 * @publicId: The public ID of the entity
 * @systemId: The system ID of the entity
 *
 * Special entity resolver, better left to the parser, it has
 * more context than the application layer.
 * The default behaviour is to NOT resolve the entities, in that case
 * the ENTITY_REF nodes are built in the structure (and the parameter
 * values).
 *
 * Returns the xmlParserInputPtr if inlined or NULL for DOM behaviour.
 */
unsafe extern "C" fn resolveEntityDebug(mut ctx: *mut std::os::raw::c_void,
                                        mut publicId: *const xmlChar,
                                        mut systemId: *const xmlChar)
 -> xmlParserInputPtr {
    xmllint_callbacks += 1;
    if xmllint_noout != 0 { return 0 as xmlParserInputPtr }
    /* xmlParserCtxtPtr ctxt = (xmlParserCtxtPtr) ctx; */
    fprintf(stdout,
            b"SAX.resolveEntity(\x00" as *const u8 as *const std::os::raw::c_char);
    if !publicId.is_null() {
        fprintf(stdout, b"%s\x00" as *const u8 as *const std::os::raw::c_char,
                publicId as *mut std::os::raw::c_char);
    } else { fprintf(stdout, b" \x00" as *const u8 as *const std::os::raw::c_char); }
    if !systemId.is_null() {
        fprintf(stdout, b", %s)\n\x00" as *const u8 as *const std::os::raw::c_char,
                systemId as *mut std::os::raw::c_char);
    } else {
        fprintf(stdout, b", )\n\x00" as *const u8 as *const std::os::raw::c_char);
    }
    return 0 as xmlParserInputPtr;
}
/* *
 * getEntityDebug:
 * @ctxt:  An XML parser context
 * @name: The entity name
 *
 * Get an entity by name
 *
 * Returns the xmlParserInputPtr if inlined or NULL for DOM behaviour.
 */
unsafe extern "C" fn getEntityDebug(mut ctx: *mut std::os::raw::c_void,
                                    mut name: *const xmlChar)
 -> xmlEntityPtr {
    xmllint_callbacks += 1;
    if xmllint_noout != 0 { return 0 as xmlEntityPtr }
    fprintf(stdout,
            b"SAX.getEntity(%s)\n\x00" as *const u8 as *const std::os::raw::c_char,
            name);
    return 0 as xmlEntityPtr;
}
/* *
 * getParameterEntityDebug:
 * @ctxt:  An XML parser context
 * @name: The entity name
 *
 * Get a parameter entity by name
 *
 * Returns the xmlParserInputPtr
 */
unsafe extern "C" fn getParameterEntityDebug(mut ctx: *mut std::os::raw::c_void,
                                             mut name: *const xmlChar)
 -> xmlEntityPtr {
    xmllint_callbacks += 1;
    if xmllint_noout != 0 { return 0 as xmlEntityPtr }
    fprintf(stdout,
            b"SAX.getParameterEntity(%s)\n\x00" as *const u8 as
                *const std::os::raw::c_char, name);
    return 0 as xmlEntityPtr;
}
/* *
 * entityDeclDebug:
 * @ctxt:  An XML parser context
 * @name:  the entity name
 * @type:  the entity type
 * @publicId: The public ID of the entity
 * @systemId: The system ID of the entity
 * @content: the entity value (without processing).
 *
 * An entity definition has been parsed
 */
unsafe extern "C" fn entityDeclDebug(mut ctx: *mut std::os::raw::c_void,
                                     mut name: *const xmlChar,
                                     mut type_0: std::os::raw::c_int,
                                     mut publicId: *const xmlChar,
                                     mut systemId: *const xmlChar,
                                     mut content: *mut xmlChar) {
    let mut nullstr: *const xmlChar =
        b"(null)\x00" as *const u8 as *const std::os::raw::c_char as *mut xmlChar;
    /* not all libraries handle printing null pointers nicely */
    if publicId.is_null() { publicId = nullstr }
    if systemId.is_null() { systemId = nullstr }
    if content.is_null() { content = nullstr as *mut xmlChar }
    xmllint_callbacks += 1;
    if xmllint_noout != 0 { return }
    fprintf(stdout,
            b"SAX.entityDecl(%s, %d, %s, %s, %s)\n\x00" as *const u8 as
                *const std::os::raw::c_char, name, type_0, publicId, systemId,
            content);
}
/* *
 * attributeDeclDebug:
 * @ctxt:  An XML parser context
 * @name:  the attribute name
 * @type:  the attribute type
 *
 * An attribute definition has been parsed
 */
unsafe extern "C" fn attributeDeclDebug(mut ctx: *mut std::os::raw::c_void,
                                        mut elem: *const xmlChar,
                                        mut name: *const xmlChar,
                                        mut type_0: std::os::raw::c_int,
                                        mut def: std::os::raw::c_int,
                                        mut defaultValue: *const xmlChar,
                                        mut tree: xmlEnumerationPtr) {
    xmllint_callbacks += 1;
    if xmllint_noout != 0 { return }
    if defaultValue.is_null() {
        fprintf(stdout,
                b"SAX.attributeDecl(%s, %s, %d, %d, NULL, ...)\n\x00" as
                    *const u8 as *const std::os::raw::c_char, elem, name, type_0,
                def);
    } else {
        fprintf(stdout,
                b"SAX.attributeDecl(%s, %s, %d, %d, %s, ...)\n\x00" as
                    *const u8 as *const std::os::raw::c_char, elem, name, type_0, def,
                defaultValue);
    }
    xmlFreeEnumeration(tree);
}
/* *
 * elementDeclDebug:
 * @ctxt:  An XML parser context
 * @name:  the element name
 * @type:  the element type
 * @content: the element value (without processing).
 *
 * An element definition has been parsed
 */
unsafe extern "C" fn elementDeclDebug(mut ctx: *mut std::os::raw::c_void,
                                      mut name: *const xmlChar,
                                      mut type_0: std::os::raw::c_int,
                                      mut content: xmlElementContentPtr) {
    xmllint_callbacks += 1;
    if xmllint_noout != 0 { return }
    fprintf(stdout,
            b"SAX.elementDecl(%s, %d, ...)\n\x00" as *const u8 as
                *const std::os::raw::c_char, name, type_0);
}
/* *
 * notationDeclDebug:
 * @ctxt:  An XML parser context
 * @name: The name of the notation
 * @publicId: The public ID of the entity
 * @systemId: The system ID of the entity
 *
 * What to do when a notation declaration has been parsed.
 */
unsafe extern "C" fn notationDeclDebug(mut ctx: *mut std::os::raw::c_void,
                                       mut name: *const xmlChar,
                                       mut publicId: *const xmlChar,
                                       mut systemId: *const xmlChar) {
    xmllint_callbacks += 1;
    if xmllint_noout != 0 { return }
    fprintf(stdout,
            b"SAX.notationDecl(%s, %s, %s)\n\x00" as *const u8 as
                *const std::os::raw::c_char, name as *mut std::os::raw::c_char,
            publicId as *mut std::os::raw::c_char, systemId as *mut std::os::raw::c_char);
}
/* *
 * unparsedEntityDeclDebug:
 * @ctxt:  An XML parser context
 * @name: The name of the entity
 * @publicId: The public ID of the entity
 * @systemId: The system ID of the entity
 * @notationName: the name of the notation
 *
 * What to do when an unparsed entity declaration is parsed
 */
unsafe extern "C" fn unparsedEntityDeclDebug(mut ctx: *mut std::os::raw::c_void,
                                             mut name: *const xmlChar,
                                             mut publicId: *const xmlChar,
                                             mut systemId: *const xmlChar,
                                             mut notationName:
                                                 *const xmlChar) {
    let mut nullstr: *const xmlChar =
        b"(null)\x00" as *const u8 as *const std::os::raw::c_char as *mut xmlChar;
    if publicId.is_null() { publicId = nullstr }
    if systemId.is_null() { systemId = nullstr }
    if notationName.is_null() { notationName = nullstr }
    xmllint_callbacks += 1;
    if xmllint_noout != 0 { return }
    fprintf(stdout,
            b"SAX.unparsedEntityDecl(%s, %s, %s, %s)\n\x00" as *const u8 as
                *const std::os::raw::c_char, name as *mut std::os::raw::c_char,
            publicId as *mut std::os::raw::c_char, systemId as *mut std::os::raw::c_char,
            notationName as *mut std::os::raw::c_char);
}
/* *
 * setDocumentLocatorDebug:
 * @ctxt:  An XML parser context
 * @loc: A SAX Locator
 *
 * Receive the document locator at startup, actually xmlDefaultSAXLocator
 * Everything is available on the context, so this is useless in our case.
 */
unsafe extern "C" fn setDocumentLocatorDebug(mut ctx: *mut std::os::raw::c_void,
                                             mut loc: xmlSAXLocatorPtr) {
    xmllint_callbacks += 1;
    if xmllint_noout != 0 { return }
    fprintf(stdout,
            b"SAX.setDocumentLocator()\n\x00" as *const u8 as
                *const std::os::raw::c_char);
}
/* *
 * startDocumentDebug:
 * @ctxt:  An XML parser context
 *
 * called when the document start being processed.
 */
unsafe extern "C" fn startDocumentDebug(mut ctx: *mut std::os::raw::c_void) {
    xmllint_callbacks += 1;
    if xmllint_noout != 0 { return }
    fprintf(stdout,
            b"SAX.startDocument()\n\x00" as *const u8 as *const std::os::raw::c_char);
}
/* *
 * endDocumentDebug:
 * @ctxt:  An XML parser context
 *
 * called when the document end has been detected.
 */
unsafe extern "C" fn endDocumentDebug(mut ctx: *mut std::os::raw::c_void) {
    xmllint_callbacks += 1;
    if xmllint_noout != 0 { return }
    fprintf(stdout,
            b"SAX.endDocument()\n\x00" as *const u8 as *const std::os::raw::c_char);
}
/* *
 * startElementDebug:
 * @ctxt:  An XML parser context
 * @name:  The element name
 *
 * called when an opening tag has been processed.
 */
unsafe extern "C" fn startElementDebug(mut ctx: *mut std::os::raw::c_void,
                                       mut name: *const xmlChar,
                                       mut atts: *mut *const xmlChar) {
    let mut i: std::os::raw::c_int = 0;
    xmllint_callbacks += 1;
    if xmllint_noout != 0 { return }
    fprintf(stdout,
            b"SAX.startElement(%s\x00" as *const u8 as *const std::os::raw::c_char,
            name as *mut std::os::raw::c_char);
    if !atts.is_null() {
        i = 0 as std::os::raw::c_int;
        while !(*atts.offset(i as isize)).is_null() {
            let fresh0 = i;
            i = i + 1;
            fprintf(stdout,
                    b", %s=\'\x00" as *const u8 as *const std::os::raw::c_char,
                    *atts.offset(fresh0 as isize));
            if !(*atts.offset(i as isize)).is_null() {
                fprintf(stdout,
                        b"%s\'\x00" as *const u8 as *const std::os::raw::c_char,
                        *atts.offset(i as isize));
            }
            i += 1
        }
    }
    fprintf(stdout, b")\n\x00" as *const u8 as *const std::os::raw::c_char);
}
/* *
 * endElementDebug:
 * @ctxt:  An XML parser context
 * @name:  The element name
 *
 * called when the end of an element has been detected.
 */
unsafe extern "C" fn endElementDebug(mut ctx: *mut std::os::raw::c_void,
                                     mut name: *const xmlChar) {
    xmllint_callbacks += 1;
    if xmllint_noout != 0 { return }
    fprintf(stdout,
            b"SAX.endElement(%s)\n\x00" as *const u8 as *const std::os::raw::c_char,
            name as *mut std::os::raw::c_char);
}
/* *
 * charactersDebug:
 * @ctxt:  An XML parser context
 * @ch:  a xmlChar string
 * @len: the number of xmlChar
 *
 * receiving some chars from the parser.
 * Question: how much at a time ???
 */
unsafe extern "C" fn charactersDebug(mut ctx: *mut std::os::raw::c_void,
                                     mut ch: *const xmlChar,
                                     mut len: std::os::raw::c_int) {
    let mut out: [std::os::raw::c_char; 40] = [0; 40];
    let mut i: std::os::raw::c_int = 0;
    xmllint_callbacks += 1;
    if xmllint_noout != 0 { return }
    i = 0 as std::os::raw::c_int;
    while i < len && i < 30 as std::os::raw::c_int {
        out[i as usize] = *ch.offset(i as isize) as std::os::raw::c_char;
        i += 1
    }
    out[i as usize] = 0 as std::os::raw::c_int as std::os::raw::c_char;
    fprintf(stdout,
            b"SAX.characters(%s, %d)\n\x00" as *const u8 as
                *const std::os::raw::c_char, out.as_mut_ptr(), len);
}
/* *
 * referenceDebug:
 * @ctxt:  An XML parser context
 * @name:  The entity name
 *
 * called when an entity reference is detected.
 */
unsafe extern "C" fn referenceDebug(mut ctx: *mut std::os::raw::c_void,
                                    mut name: *const xmlChar) {
    xmllint_callbacks += 1;
    if xmllint_noout != 0 { return }
    fprintf(stdout,
            b"SAX.reference(%s)\n\x00" as *const u8 as *const std::os::raw::c_char,
            name);
}
/* *
 * ignorableWhitespaceDebug:
 * @ctxt:  An XML parser context
 * @ch:  a xmlChar string
 * @start: the first char in the string
 * @len: the number of xmlChar
 *
 * receiving some ignorable whitespaces from the parser.
 * Question: how much at a time ???
 */
unsafe extern "C" fn ignorableWhitespaceDebug(mut ctx: *mut std::os::raw::c_void,
                                              mut ch: *const xmlChar,
                                              mut len: std::os::raw::c_int) {
    let mut out: [std::os::raw::c_char; 40] = [0; 40];
    let mut i: std::os::raw::c_int = 0;
    xmllint_callbacks += 1;
    if xmllint_noout != 0 { return }
    i = 0 as std::os::raw::c_int;
    while i < len && i < 30 as std::os::raw::c_int {
        out[i as usize] = *ch.offset(i as isize) as std::os::raw::c_char;
        i += 1
    }
    out[i as usize] = 0 as std::os::raw::c_int as std::os::raw::c_char;
    fprintf(stdout,
            b"SAX.ignorableWhitespace(%s, %d)\n\x00" as *const u8 as
                *const std::os::raw::c_char, out.as_mut_ptr(), len);
}
/* *
 * processingInstructionDebug:
 * @ctxt:  An XML parser context
 * @target:  the target name
 * @data: the PI data's
 * @len: the number of xmlChar
 *
 * A processing instruction has been parsed.
 */
unsafe extern "C" fn processingInstructionDebug(mut ctx: *mut std::os::raw::c_void,
                                                mut target: *const xmlChar,
                                                mut data: *const xmlChar) {
    xmllint_callbacks += 1;
    if xmllint_noout != 0 { return }
    if !data.is_null() {
        fprintf(stdout,
                b"SAX.processingInstruction(%s, %s)\n\x00" as *const u8 as
                    *const std::os::raw::c_char, target as *mut std::os::raw::c_char,
                data as *mut std::os::raw::c_char);
    } else {
        fprintf(stdout,
                b"SAX.processingInstruction(%s, NULL)\n\x00" as *const u8 as
                    *const std::os::raw::c_char, target as *mut std::os::raw::c_char);
    };
}
/* *
 * cdataBlockDebug:
 * @ctx: the user data (XML parser context)
 * @value:  The pcdata content
 * @len:  the block length
 *
 * called when a pcdata block has been parsed
 */
unsafe extern "C" fn cdataBlockDebug(mut ctx: *mut std::os::raw::c_void,
                                     mut value: *const xmlChar,
                                     mut len: std::os::raw::c_int) {
    xmllint_callbacks += 1;
    if xmllint_noout != 0 { return }
    fprintf(stdout,
            b"SAX.pcdata(%.20s, %d)\n\x00" as *const u8 as
                *const std::os::raw::c_char, value as *mut std::os::raw::c_char, len);
}
/* *
 * commentDebug:
 * @ctxt:  An XML parser context
 * @value:  the comment content
 *
 * A comment has been parsed.
 */
unsafe extern "C" fn commentDebug(mut ctx: *mut std::os::raw::c_void,
                                  mut value: *const xmlChar) {
    xmllint_callbacks += 1;
    if xmllint_noout != 0 { return }
    fprintf(stdout,
            b"SAX.comment(%s)\n\x00" as *const u8 as *const std::os::raw::c_char,
            value);
}
/* *
 * warningDebug:
 * @ctxt:  An XML parser context
 * @msg:  the message to display/transmit
 * @...:  extra parameters for the message display
 *
 * Display and format a warning messages, gives file, line, position and
 * extra parameters.
 */
// static void XMLCDECL LIBXML_ATTR_FORMAT(2,3)
// warningDebug(void *ctx ATTRIBUTE_UNUSED, const char *msg, ...)
// {
//     va_list args;
//     xmllint_callbacks++;
//     if (xmllint_noout)
// 	return;
//     va_start(args, msg);
//     fprintf(stdout, "SAX.warning: ");
//     vfprintf(stdout, msg, args);
//     va_end(args);
// }
/* *
 * errorDebug:
 * @ctxt:  An XML parser context
 * @msg:  the message to display/transmit
 * @...:  extra parameters for the message display
 *
 * Display and format a error messages, gives file, line, position and
 * extra parameters.
 */
// static void XMLCDECL LIBXML_ATTR_FORMAT(2,3)
// errorDebug(void *ctx ATTRIBUTE_UNUSED, const char *msg, ...)
// {
//     va_list args;
//     xmllint_callbacks++;
//     if (xmllint_noout)
// 	return;
//     va_start(args, msg);
//     fprintf(stdout, "SAX.error: ");
//     vfprintf(stdout, msg, args);
//     va_end(args);
// }
/* *
 * fatalErrorDebug:
 * @ctxt:  An XML parser context
 * @msg:  the message to display/transmit
 * @...:  extra parameters for the message display
 *
 * Display and format a fatalError messages, gives file, line, position and
 * extra parameters.
 */
// static void XMLCDECL LIBXML_ATTR_FORMAT(2,3)
// fatalErrorDebug(void *ctx ATTRIBUTE_UNUSED, const char *msg, ...)
// {
//     va_list args;
//     xmllint_callbacks++;
//     if (xmllint_noout)
// 	return;
//     va_start(args, msg);
//     fprintf(stdout, "SAX.fatalError: ");
//     vfprintf(stdout, msg, args);
//     va_end(args);
// }
static mut debugSAXHandlerStruct: xmlSAXHandler =
    unsafe {
        {
            let mut init =
                _xmlSAXHandler{internalSubset:
                                   Some(internalSubsetDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     *const xmlChar)
                                                -> ()),
                               isStandalone:
                                   Some(isStandaloneDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void)
                                                -> std::os::raw::c_int),
                               hasInternalSubset:
                                   Some(hasInternalSubsetDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void)
                                                -> std::os::raw::c_int),
                               hasExternalSubset:
                                   Some(hasExternalSubsetDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void)
                                                -> std::os::raw::c_int),
                               resolveEntity:
                                   Some(resolveEntityDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     *const xmlChar)
                                                -> xmlParserInputPtr),
                               getEntity:
                                   Some(getEntityDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     *const xmlChar)
                                                -> xmlEntityPtr),
                               entityDecl:
                                   Some(entityDeclDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     std::os::raw::c_int,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     *mut xmlChar)
                                                -> ()),
                               notationDecl:
                                   Some(notationDeclDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     *const xmlChar)
                                                -> ()),
                               attributeDecl:
                                   Some(attributeDeclDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     std::os::raw::c_int,
                                                                 _:
                                                                     std::os::raw::c_int,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     xmlEnumerationPtr)
                                                -> ()),
                               elementDecl:
                                   Some(elementDeclDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     std::os::raw::c_int,
                                                                 _:
                                                                     xmlElementContentPtr)
                                                -> ()),
                               unparsedEntityDecl:
                                   Some(unparsedEntityDeclDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     *const xmlChar)
                                                -> ()),
                               setDocumentLocator:
                                   Some(setDocumentLocatorDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     xmlSAXLocatorPtr)
                                                -> ()),
                               startDocument:
                                   Some(startDocumentDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void)
                                                -> ()),
                               endDocument:
                                   Some(endDocumentDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void)
                                                -> ()),
                               startElement:
                                   Some(startElementDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     *mut *const xmlChar)
                                                -> ()),
                               endElement:
                                   Some(endElementDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     *const xmlChar)
                                                -> ()),
                               reference:
                                   Some(referenceDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     *const xmlChar)
                                                -> ()),
                               characters:
                                   Some(charactersDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     std::os::raw::c_int)
                                                -> ()),
                               ignorableWhitespace:
                                   Some(ignorableWhitespaceDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     std::os::raw::c_int)
                                                -> ()),
                               processingInstruction:
                                   Some(processingInstructionDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     *const xmlChar)
                                                -> ()),
                               comment:
                                   Some(commentDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     *const xmlChar)
                                                -> ()),
                               warning:
                                   Some(xmllint_warningDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     *const std::os::raw::c_char,
                                                                 _: ...)
                                                -> ()),
                               error:
                                   Some(xmllint_errorDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     *const std::os::raw::c_char,
                                                                 _: ...)
                                                -> ()),
                               fatalError:
                                   Some(xmllint_fatalErrorDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     *const std::os::raw::c_char,
                                                                 _: ...)
                                                -> ()),
                               getParameterEntity:
                                   Some(getParameterEntityDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     *const xmlChar)
                                                -> xmlEntityPtr),
                               cdataBlock:
                                   Some(cdataBlockDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     std::os::raw::c_int)
                                                -> ()),
                               externalSubset:
                                   Some(externalSubsetDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     *const xmlChar)
                                                -> ()),
                               initialized: 1 as std::os::raw::c_int as std::os::raw::c_uint,
                               _private:
                                   0 as *const std::os::raw::c_void as
                                       *mut std::os::raw::c_void,
                               startElementNs: None,
                               endElementNs: None,
                               serror: None,};
            init
        }
    };
#[no_mangle]
pub static mut debugSAXHandler: xmlSAXHandlerPtr =
    unsafe {
        &debugSAXHandlerStruct as *const xmlSAXHandler as *mut xmlSAXHandler
    };
/*
 * SAX2 specific callbacks
 */
/* *
 * startElementNsDebug:
 * @ctxt:  An XML parser context
 * @name:  The element name
 *
 * called when an opening tag has been processed.
 */
unsafe extern "C" fn startElementNsDebug(mut ctx: *mut std::os::raw::c_void,
                                         mut localname: *const xmlChar,
                                         mut prefix: *const xmlChar,
                                         mut URI: *const xmlChar,
                                         mut nb_namespaces: std::os::raw::c_int,
                                         mut namespaces: *mut *const xmlChar,
                                         mut nb_attributes: std::os::raw::c_int,
                                         mut nb_defaulted: std::os::raw::c_int,
                                         mut attributes:
                                             *mut *const xmlChar) {
    let mut i: std::os::raw::c_int = 0;
    xmllint_callbacks += 1;
    if xmllint_noout != 0 { return }
    fprintf(stdout,
            b"SAX.startElementNs(%s\x00" as *const u8 as *const std::os::raw::c_char,
            localname as *mut std::os::raw::c_char);
    if prefix.is_null() {
        fprintf(stdout, b", NULL\x00" as *const u8 as *const std::os::raw::c_char);
    } else {
        fprintf(stdout, b", %s\x00" as *const u8 as *const std::os::raw::c_char,
                prefix as *mut std::os::raw::c_char);
    }
    if URI.is_null() {
        fprintf(stdout, b", NULL\x00" as *const u8 as *const std::os::raw::c_char);
    } else {
        fprintf(stdout, b", \'%s\'\x00" as *const u8 as *const std::os::raw::c_char,
                URI as *mut std::os::raw::c_char);
    }
    fprintf(stdout, b", %d\x00" as *const u8 as *const std::os::raw::c_char,
            nb_namespaces);
    if !namespaces.is_null() {
        i = 0 as std::os::raw::c_int;
        while i < nb_namespaces * 2 as std::os::raw::c_int {
            fprintf(stdout,
                    b", xmlns\x00" as *const u8 as *const std::os::raw::c_char);
            if !(*namespaces.offset(i as isize)).is_null() {
                fprintf(stdout,
                        b":%s\x00" as *const u8 as *const std::os::raw::c_char,
                        *namespaces.offset(i as isize));
            }
            i += 1;
            fprintf(stdout,
                    b"=\'%s\'\x00" as *const u8 as *const std::os::raw::c_char,
                    *namespaces.offset(i as isize));
            i += 1
        }
    }
    fprintf(stdout, b", %d, %d\x00" as *const u8 as *const std::os::raw::c_char,
            nb_attributes, nb_defaulted);
    if !attributes.is_null() {
        i = 0 as std::os::raw::c_int;
        while i < nb_attributes * 5 as std::os::raw::c_int {
            if !(*attributes.offset((i + 1 as std::os::raw::c_int) as
                                        isize)).is_null() {
                fprintf(stdout,
                        b", %s:%s=\'\x00" as *const u8 as *const std::os::raw::c_char,
                        *attributes.offset((i + 1 as std::os::raw::c_int) as isize),
                        *attributes.offset(i as isize));
            } else {
                fprintf(stdout,
                        b", %s=\'\x00" as *const u8 as *const std::os::raw::c_char,
                        *attributes.offset(i as isize));
            }
            fprintf(stdout,
                    b"%.4s...\', %d\x00" as *const u8 as *const std::os::raw::c_char,
                    *attributes.offset((i + 3 as std::os::raw::c_int) as isize),
                    (*attributes.offset((i + 4 as std::os::raw::c_int) as
                                            isize)).offset_from(*attributes.offset((i
                                                                                                 +
                                                                                                 3
                                                                                                     as
                                                                                                     std::os::raw::c_int)
                                                                                                as
                                                                                                isize))
                        as std::os::raw::c_long as std::os::raw::c_int);
            i += 5 as std::os::raw::c_int
        }
    }
    fprintf(stdout, b")\n\x00" as *const u8 as *const std::os::raw::c_char);
}
/* *
 * endElementDebug:
 * @ctxt:  An XML parser context
 * @name:  The element name
 *
 * called when the end of an element has been detected.
 */
unsafe extern "C" fn endElementNsDebug(mut ctx: *mut std::os::raw::c_void,
                                       mut localname: *const xmlChar,
                                       mut prefix: *const xmlChar,
                                       mut URI: *const xmlChar) {
    xmllint_callbacks += 1; /* mostly for debugging */
    if xmllint_noout != 0 { return }
    fprintf(stdout,
            b"SAX.endElementNs(%s\x00" as *const u8 as *const std::os::raw::c_char,
            localname as *mut std::os::raw::c_char);
    if prefix.is_null() {
        fprintf(stdout, b", NULL\x00" as *const u8 as *const std::os::raw::c_char);
    } else {
        fprintf(stdout, b", %s\x00" as *const u8 as *const std::os::raw::c_char,
                prefix as *mut std::os::raw::c_char);
    }
    if URI.is_null() {
        fprintf(stdout, b", NULL)\n\x00" as *const u8 as *const std::os::raw::c_char);
    } else {
        fprintf(stdout,
                b", \'%s\')\n\x00" as *const u8 as *const std::os::raw::c_char,
                URI as *mut std::os::raw::c_char);
    };
}
static mut debugSAX2HandlerStruct: xmlSAXHandler =
    unsafe {
        {
            let mut init =
                _xmlSAXHandler{internalSubset:
                                   Some(internalSubsetDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     *const xmlChar)
                                                -> ()),
                               isStandalone:
                                   Some(isStandaloneDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void)
                                                -> std::os::raw::c_int),
                               hasInternalSubset:
                                   Some(hasInternalSubsetDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void)
                                                -> std::os::raw::c_int),
                               hasExternalSubset:
                                   Some(hasExternalSubsetDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void)
                                                -> std::os::raw::c_int),
                               resolveEntity:
                                   Some(resolveEntityDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     *const xmlChar)
                                                -> xmlParserInputPtr),
                               getEntity:
                                   Some(getEntityDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     *const xmlChar)
                                                -> xmlEntityPtr),
                               entityDecl:
                                   Some(entityDeclDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     std::os::raw::c_int,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     *mut xmlChar)
                                                -> ()),
                               notationDecl:
                                   Some(notationDeclDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     *const xmlChar)
                                                -> ()),
                               attributeDecl:
                                   Some(attributeDeclDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     std::os::raw::c_int,
                                                                 _:
                                                                     std::os::raw::c_int,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     xmlEnumerationPtr)
                                                -> ()),
                               elementDecl:
                                   Some(elementDeclDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     std::os::raw::c_int,
                                                                 _:
                                                                     xmlElementContentPtr)
                                                -> ()),
                               unparsedEntityDecl:
                                   Some(unparsedEntityDeclDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     *const xmlChar)
                                                -> ()),
                               setDocumentLocator:
                                   Some(setDocumentLocatorDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     xmlSAXLocatorPtr)
                                                -> ()),
                               startDocument:
                                   Some(startDocumentDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void)
                                                -> ()),
                               endDocument:
                                   Some(endDocumentDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void)
                                                -> ()),
                               startElement: None,
                               endElement: None,
                               reference:
                                   Some(referenceDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     *const xmlChar)
                                                -> ()),
                               characters:
                                   Some(charactersDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     std::os::raw::c_int)
                                                -> ()),
                               ignorableWhitespace:
                                   Some(ignorableWhitespaceDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     std::os::raw::c_int)
                                                -> ()),
                               processingInstruction:
                                   Some(processingInstructionDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     *const xmlChar)
                                                -> ()),
                               comment:
                                   Some(commentDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     *const xmlChar)
                                                -> ()),
                               warning:
                                   Some(xmllint_warningDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     *const std::os::raw::c_char,
                                                                 _: ...)
                                                -> ()),
                               error:
                                   Some(xmllint_errorDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     *const std::os::raw::c_char,
                                                                 _: ...)
                                                -> ()),
                               fatalError:
                                   Some(xmllint_fatalErrorDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     *const std::os::raw::c_char,
                                                                 _: ...)
                                                -> ()),
                               getParameterEntity:
                                   Some(getParameterEntityDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     *const xmlChar)
                                                -> xmlEntityPtr),
                               cdataBlock:
                                   Some(cdataBlockDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     std::os::raw::c_int)
                                                -> ()),
                               externalSubset:
                                   Some(externalSubsetDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     *const xmlChar)
                                                -> ()),
                               initialized: 0xdeedbeaf as std::os::raw::c_uint,
                               _private:
                                   0 as *const std::os::raw::c_void as
                                       *mut std::os::raw::c_void,
                               startElementNs:
                                   Some(startElementNsDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     std::os::raw::c_int,
                                                                 _:
                                                                     *mut *const xmlChar,
                                                                 _:
                                                                     std::os::raw::c_int,
                                                                 _:
                                                                     std::os::raw::c_int,
                                                                 _:
                                                                     *mut *const xmlChar)
                                                -> ()),
                               endElementNs:
                                   Some(endElementNsDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     *const xmlChar)
                                                -> ()),
                               serror: None,};
            init
        }
    };
static mut debugSAX2Handler: xmlSAXHandlerPtr =
    unsafe {
        &debugSAX2HandlerStruct as *const xmlSAXHandler as *mut xmlSAXHandler
    };
unsafe extern "C" fn testSAX(mut filename: *const std::os::raw::c_char) {
    let mut handler: xmlSAXHandlerPtr = 0 as *mut xmlSAXHandler;
    let mut user_data: *const std::os::raw::c_char =
        b"user_data\x00" as *const u8 as *const std::os::raw::c_char;
    let mut buf: xmlParserInputBufferPtr = 0 as xmlParserInputBufferPtr;
    let mut inputStream: xmlParserInputPtr = 0 as *mut xmlParserInput;
    let mut ctxt: xmlParserCtxtPtr = 0 as xmlParserCtxtPtr;
    let mut old_sax: xmlSAXHandlerPtr = 0 as xmlSAXHandlerPtr;
    xmllint_callbacks = 0 as std::os::raw::c_int;
    if xmllint_noout != 0 {
        handler = emptySAXHandler
    } else if sax1 != 0 {
        handler = debugSAXHandler
    } else { handler = debugSAX2Handler }
    /*
     * it's not the simplest code but the most generic in term of I/O
     */
    buf =
        xmlParserInputBufferCreateFilename(filename, XML_CHAR_ENCODING_NONE);
    if !buf.is_null() {
        if !wxschemas.is_null() {
            let mut ret: std::os::raw::c_int = 0;
            let mut vctxt: xmlSchemaValidCtxtPtr =
                0 as *mut xmlSchemaValidCtxt;
            vctxt = xmlSchemaNewValidCtxt(wxschemas);
            xmlSchemaSetValidErrors(vctxt,
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
            xmlSchemaValidateSetFilename(vctxt, filename);
            ret =
                xmlSchemaValidateStream(vctxt, buf, XML_CHAR_ENCODING_NONE,
                                        handler,
                                        user_data as *mut std::os::raw::c_void);
            if repeat == 0 as std::os::raw::c_int {
                if ret == 0 as std::os::raw::c_int {
                    fprintf(stderr,
                            b"%s validates\n\x00" as *const u8 as
                                *const std::os::raw::c_char, filename);
                } else if ret > 0 as std::os::raw::c_int {
                    fprintf(stderr,
                            b"%s fails to validate\n\x00" as *const u8 as
                                *const std::os::raw::c_char, filename);
                    xmllint_progresult = XMLLINT_ERR_VALID
                } else {
                    fprintf(stderr,
                            b"%s validation generated an internal error\n\x00"
                                as *const u8 as *const std::os::raw::c_char,
                            filename);
                    xmllint_progresult = XMLLINT_ERR_VALID
                }
            }
            xmlSchemaFreeValidCtxt(vctxt);
        } else {
            /*
	 * Create the parser context amd hook the input
	 */
            ctxt = xmlNewParserCtxt();
            if ctxt.is_null() {
                xmlFreeParserInputBuffer(buf);
            } else {
                old_sax = (*ctxt).sax;
                (*ctxt).sax = handler;
                (*ctxt).userData = user_data as *mut std::os::raw::c_void;
                inputStream =
                    xmlNewIOInputStream(ctxt, buf, XML_CHAR_ENCODING_NONE);
                if inputStream.is_null() {
                    xmlFreeParserInputBuffer(buf);
                } else {
                    inputPush(ctxt, inputStream);
                    /* do the parsing */
                    xmlParseDocument(ctxt);
                    if !(*ctxt).myDoc.is_null() {
                        fprintf(stderr,
                                b"SAX generated a doc !\n\x00" as *const u8 as
                                    *const std::os::raw::c_char);
                        xmlFreeDoc((*ctxt).myDoc);
                        (*ctxt).myDoc = 0 as xmlDocPtr
                    }
                }
            }
        }
    }
    if !ctxt.is_null() { (*ctxt).sax = old_sax; xmlFreeParserCtxt(ctxt); };
}
/* ***********************************************************************
 *									*
 *			Stream Test processing				*
 *									*
 ************************************************************************/
unsafe extern "C" fn processNode(mut reader: xmlTextReaderPtr) {
    let mut name: *const xmlChar = 0 as *const xmlChar;
    let mut value: *const xmlChar = 0 as *const xmlChar;
    let mut type_0: std::os::raw::c_int = 0;
    let mut empty: std::os::raw::c_int = 0;
    type_0 = xmlTextReaderNodeType(reader);
    empty = xmlTextReaderIsEmptyElement(reader);
    if debug != 0 {
        name = xmlTextReaderConstName(reader);
        if name.is_null() {
            name =
                b"--\x00" as *const u8 as *const std::os::raw::c_char as *mut xmlChar
        }
        value = xmlTextReaderConstValue(reader);
        printf(b"%d %d %s %d %d\x00" as *const u8 as *const std::os::raw::c_char,
               xmlTextReaderDepth(reader), type_0, name, empty,
               xmlTextReaderHasValue(reader));
        if value.is_null() {
            printf(b"\n\x00" as *const u8 as *const std::os::raw::c_char);
        } else {
            printf(b" %s\n\x00" as *const u8 as *const std::os::raw::c_char, value);
        }
    }
    if !patternc.is_null() {
        let mut path: *mut xmlChar = 0 as *mut xmlChar;
        let mut match_0: std::os::raw::c_int = -(1 as std::os::raw::c_int);
        if type_0 == XML_READER_TYPE_ELEMENT as std::os::raw::c_int {
            /* do the check only on element start */
            match_0 =
                xmlPatternMatch(patternc, xmlTextReaderCurrentNode(reader));
            if match_0 != 0 {
                path =
                    xmlGetNodePath(xmlTextReaderCurrentNode(reader) as
                                       *const xmlNode);
                printf(b"Node %s matches pattern %s\n\x00" as *const u8 as
                           *const std::os::raw::c_char, path, pattern);
            }
        }
        if !patstream.is_null() {
            let mut ret: std::os::raw::c_int = 0;
            if type_0 == XML_READER_TYPE_ELEMENT as std::os::raw::c_int {
                ret =
                    xmlStreamPush(patstream,
                                  xmlTextReaderConstLocalName(reader),
                                  xmlTextReaderConstNamespaceUri(reader));
                if ret < 0 as std::os::raw::c_int {
                    fprintf(stderr,
                            b"xmlStreamPush() failure\n\x00" as *const u8 as
                                *const std::os::raw::c_char);
                    xmlFreeStreamCtxt(patstream);
                    patstream = 0 as xmlStreamCtxtPtr
                } else if ret != match_0 {
                    if path.is_null() {
                        path =
                            xmlGetNodePath(xmlTextReaderCurrentNode(reader) as
                                               *const xmlNode)
                    }
                    fprintf(stderr,
                            b"xmlPatternMatch and xmlStreamPush disagree\n\x00"
                                as *const u8 as *const std::os::raw::c_char);
                    if !path.is_null() {
                        fprintf(stderr,
                                b"  pattern %s node %s\n\x00" as *const u8 as
                                    *const std::os::raw::c_char, pattern, path);
                    } else {
                        fprintf(stderr,
                                b"  pattern %s node %s\n\x00" as *const u8 as
                                    *const std::os::raw::c_char, pattern,
                                xmlTextReaderConstName(reader));
                    }
                }
            }
            if type_0 == XML_READER_TYPE_END_ELEMENT as std::os::raw::c_int ||
                   type_0 == XML_READER_TYPE_ELEMENT as std::os::raw::c_int &&
                       empty != 0 {
                ret = xmlStreamPop(patstream);
                if ret < 0 as std::os::raw::c_int {
                    fprintf(stderr,
                            b"xmlStreamPop() failure\n\x00" as *const u8 as
                                *const std::os::raw::c_char);
                    xmlFreeStreamCtxt(patstream);
                    patstream = 0 as xmlStreamCtxtPtr
                }
            }
        }
        if !path.is_null() {
            xmlFree.expect("non-null function pointer")(path as
                                                            *mut std::os::raw::c_void);
        }
    };
}
unsafe extern "C" fn streamFile(mut filename: *mut std::os::raw::c_char) {
    let mut reader: xmlTextReaderPtr = 0 as *mut xmlTextReader;
    let mut ret: std::os::raw::c_int = 0;
    let mut fd: std::os::raw::c_int = -(1 as std::os::raw::c_int);
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
    let mut base: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut input: xmlParserInputBufferPtr = 0 as xmlParserInputBufferPtr;
    if memory != 0 {
        if stat(filename, &mut info) < 0 as std::os::raw::c_int { return }
        fd = open(filename, 0 as std::os::raw::c_int);
        if fd < 0 as std::os::raw::c_int { return }
        base =
            mmap(0 as *mut std::os::raw::c_void, info.st_size as size_t,
                 0x1 as std::os::raw::c_int, 0x1 as std::os::raw::c_int, fd,
                 0 as std::os::raw::c_int as __off64_t) as *const std::os::raw::c_char;
        if base ==
               -(1 as std::os::raw::c_int) as *mut std::os::raw::c_void as *const std::os::raw::c_char
           {
            close(fd);
            fprintf(stderr,
                    b"mmap failure for file %s\n\x00" as *const u8 as
                        *const std::os::raw::c_char, filename);
            xmllint_progresult = XMLLINT_ERR_RDFILE;
            return
        }
        reader =
            xmlReaderForMemory(base, info.st_size as std::os::raw::c_int, filename,
                               0 as *const std::os::raw::c_char, options)
    } else {
        reader = xmlReaderForFile(filename, 0 as *const std::os::raw::c_char, options)
    }
    if !pattern.is_null() {
        patternc =
            xmlPatterncompile(pattern as *const xmlChar, 0 as *mut xmlDict,
                              0 as std::os::raw::c_int, 0 as *mut *const xmlChar);
        if patternc.is_null() {
            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                       b"Pattern %s failed to compile\n\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const std::os::raw::c_char,
                                                                       pattern);
            xmllint_progresult = XMLLINT_ERR_SCHEMAPAT;
            pattern = 0 as *const std::os::raw::c_char
        }
    }
    if !patternc.is_null() {
        patstream = xmlPatternGetStreamCtxt(patternc);
        if !patstream.is_null() {
            ret =
                xmlStreamPush(patstream, 0 as *const xmlChar,
                              0 as *const xmlChar);
            if ret < 0 as std::os::raw::c_int {
                fprintf(stderr,
                        b"xmlStreamPush() failure\n\x00" as *const u8 as
                            *const std::os::raw::c_char);
                xmlFreeStreamCtxt(patstream);
                patstream = 0 as xmlStreamCtxtPtr
            }
        }
    }
    if !reader.is_null() {
        if valid != 0 {
            xmlTextReaderSetParserProp(reader,
                                       XML_PARSER_VALIDATE as std::os::raw::c_int,
                                       1 as std::os::raw::c_int);
        } else if loaddtd != 0 {
            xmlTextReaderSetParserProp(reader,
                                       XML_PARSER_LOADDTD as std::os::raw::c_int,
                                       1 as std::os::raw::c_int);
        }
        if !relaxng.is_null() {
            if timing != 0 && repeat == 0 { xmllint_startTimer(); }
            ret = xmlTextReaderRelaxNGValidate(reader, relaxng);
            if ret < 0 as std::os::raw::c_int {
                (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                           b"Relax-NG schema %s failed to compile\n\x00"
                                                                               as
                                                                               *const u8
                                                                               as
                                                                               *const std::os::raw::c_char,
                                                                           relaxng);
                xmllint_progresult = XMLLINT_ERR_SCHEMACOMP;
                relaxng = 0 as *mut std::os::raw::c_char
            }
            if timing != 0 && repeat == 0 {
                xmllint_endTimer(b"Compiling the schemas\x00" as *const u8 as
                                     *const std::os::raw::c_char);
            }
        }
        if !schema.is_null() {
            if timing != 0 && repeat == 0 { xmllint_startTimer(); }
            ret = xmlTextReaderSchemaValidate(reader, schema);
            if ret < 0 as std::os::raw::c_int {
                (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                           b"XSD schema %s failed to compile\n\x00"
                                                                               as
                                                                               *const u8
                                                                               as
                                                                               *const std::os::raw::c_char,
                                                                           schema);
                xmllint_progresult = XMLLINT_ERR_SCHEMACOMP;
                schema = 0 as *mut std::os::raw::c_char
            }
            if timing != 0 && repeat == 0 {
                xmllint_endTimer(b"Compiling the schemas\x00" as *const u8 as
                                     *const std::os::raw::c_char);
            }
        }
        /* LIBXML_VALID_ENABLED */
        /*
	 * Process all nodes in sequence
	 */
        if timing != 0 && repeat == 0 { xmllint_startTimer(); }
        ret = xmlTextReaderRead(reader);
        while ret == 1 as std::os::raw::c_int {
            if debug != 0 || !patternc.is_null() { processNode(reader); }
            ret = xmlTextReaderRead(reader)
        }
        if timing != 0 && repeat == 0 {
            if !relaxng.is_null() {
                xmllint_endTimer(b"Parsing and validating\x00" as *const u8 as
                                     *const std::os::raw::c_char);
            } else if valid != 0 {
                xmllint_endTimer(b"Parsing and validating\x00" as *const u8 as
                                     *const std::os::raw::c_char);
            } else {
                xmllint_endTimer(b"Parsing\x00" as *const u8 as
                                     *const std::os::raw::c_char);
            }
        }
        if valid != 0 {
            if xmlTextReaderIsValid(reader) != 1 as std::os::raw::c_int {
                (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                           b"Document %s does not validate\n\x00"
                                                                               as
                                                                               *const u8
                                                                               as
                                                                               *const std::os::raw::c_char,
                                                                           filename);
                xmllint_progresult = XMLLINT_ERR_VALID
            }
        }
        /* LIBXML_VALID_ENABLED */
        if !relaxng.is_null() || !schema.is_null() {
            if xmlTextReaderIsValid(reader) != 1 as std::os::raw::c_int {
                fprintf(stderr,
                        b"%s fails to validate\n\x00" as *const u8 as
                            *const std::os::raw::c_char, filename);
                xmllint_progresult = XMLLINT_ERR_VALID
            } else {
                fprintf(stderr,
                        b"%s validates\n\x00" as *const u8 as
                            *const std::os::raw::c_char, filename);
            }
        }
        /*
	 * Done, cleanup and status
	 */
        xmlFreeTextReader(reader);
        if ret != 0 as std::os::raw::c_int {
            fprintf(stderr,
                    b"%s : failed to parse\n\x00" as *const u8 as
                        *const std::os::raw::c_char, filename);
            xmllint_progresult = XMLLINT_ERR_UNCLASS
        }
    } else {
        fprintf(stderr,
                b"Unable to open %s\n\x00" as *const u8 as
                    *const std::os::raw::c_char, filename);
        xmllint_progresult = XMLLINT_ERR_UNCLASS
    }
    if !patstream.is_null() {
        xmlFreeStreamCtxt(patstream);
        patstream = 0 as xmlStreamCtxtPtr
    }
    if memory != 0 {
        xmlFreeParserInputBuffer(input);
        munmap(base as *mut std::os::raw::c_char as *mut std::os::raw::c_void,
               info.st_size as size_t);
        close(fd);
    };
}
unsafe extern "C" fn walkDoc(mut doc: xmlDocPtr) {
    let mut reader: xmlTextReaderPtr = 0 as *mut xmlTextReader;
    let mut ret: std::os::raw::c_int = 0;
    let mut root: xmlNodePtr = 0 as *mut xmlNode;
    let mut namespaces: [*const xmlChar; 22] = [0 as *const xmlChar; 22];
    let mut i: std::os::raw::c_int = 0;
    let mut ns: xmlNsPtr = 0 as *mut xmlNs;
    root = xmlDocGetRootElement(doc as *const xmlDoc);
    if root.is_null() {
        (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                   b"Document does not have a root element\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const std::os::raw::c_char);
        xmllint_progresult = XMLLINT_ERR_UNCLASS;
        return
    }
    ns = (*root).nsDef;
    i = 0 as std::os::raw::c_int;
    while !ns.is_null() && i < 20 as std::os::raw::c_int {
        let fresh1 = i;
        i = i + 1;
        namespaces[fresh1 as usize] = (*ns).href;
        let fresh2 = i;
        i = i + 1;
        namespaces[fresh2 as usize] = (*ns).prefix;
        ns = (*ns).next
    }
    let fresh3 = i;
    i = i + 1;
    namespaces[fresh3 as usize] = 0 as *const xmlChar;
    namespaces[i as usize] = 0 as *const xmlChar;
    if !pattern.is_null() {
        patternc =
            xmlPatterncompile(pattern as *const xmlChar, (*doc).dict,
                              0 as std::os::raw::c_int,
                              &mut *namespaces.as_mut_ptr().offset(0 as
                                                                       std::os::raw::c_int
                                                                       as
                                                                       isize));
        if patternc.is_null() {
            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                       b"Pattern %s failed to compile\n\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const std::os::raw::c_char,
                                                                       pattern);
            xmllint_progresult = XMLLINT_ERR_SCHEMAPAT;
            pattern = 0 as *const std::os::raw::c_char
        }
    }
    if !patternc.is_null() {
        patstream = xmlPatternGetStreamCtxt(patternc);
        if !patstream.is_null() {
            ret =
                xmlStreamPush(patstream, 0 as *const xmlChar,
                              0 as *const xmlChar);
            if ret < 0 as std::os::raw::c_int {
                fprintf(stderr,
                        b"xmlStreamPush() failure\n\x00" as *const u8 as
                            *const std::os::raw::c_char);
                xmlFreeStreamCtxt(patstream);
                patstream = 0 as xmlStreamCtxtPtr
            }
        }
    }
    /* LIBXML_PATTERN_ENABLED */
    reader = xmlReaderWalker(doc);
    if !reader.is_null() {
        if timing != 0 && repeat == 0 { xmllint_startTimer(); }
        ret = xmlTextReaderRead(reader);
        while ret == 1 as std::os::raw::c_int {
            if debug != 0 || !patternc.is_null() { processNode(reader); }
            ret = xmlTextReaderRead(reader)
        }
        if timing != 0 && repeat == 0 {
            xmllint_endTimer(b"walking through the doc\x00" as *const u8 as
                                 *const std::os::raw::c_char);
        }
        xmlFreeTextReader(reader);
        if ret != 0 as std::os::raw::c_int {
            fprintf(stderr,
                    b"failed to walk through the doc\n\x00" as *const u8 as
                        *const std::os::raw::c_char);
            xmllint_progresult = XMLLINT_ERR_UNCLASS
        }
    } else {
        fprintf(stderr,
                b"Failed to crate a reader from the document\n\x00" as
                    *const u8 as *const std::os::raw::c_char);
        xmllint_progresult = XMLLINT_ERR_UNCLASS
    }
    if !patstream.is_null() {
        xmlFreeStreamCtxt(patstream);
        patstream = 0 as xmlStreamCtxtPtr
    };
}
/* LIBXML_READER_ENABLED */
/* ***********************************************************************
 *									*
 *			XPath Query                                     *
 *									*
 ************************************************************************/
unsafe extern "C" fn doXPathDump(mut cur: xmlXPathObjectPtr) {
    match (*cur).type_0 as std::os::raw::c_uint {
        1 => {
            let mut i: std::os::raw::c_int = 0;
            let mut node: xmlNodePtr = 0 as *mut xmlNode;
            let mut ctxt: xmlSaveCtxtPtr = 0 as *mut xmlSaveCtxt;
            if (*cur).nodesetval.is_null() ||
                   (*(*cur).nodesetval).nodeNr <= 0 as std::os::raw::c_int {
                fprintf(stderr,
                        b"XPath set is empty\n\x00" as *const u8 as
                            *const std::os::raw::c_char);
                xmllint_progresult = XMLLINT_ERR_XPATH
            } else {
                ctxt =
                    xmlSaveToFd(1 as std::os::raw::c_int, 0 as *const std::os::raw::c_char,
                                0 as std::os::raw::c_int);
                if ctxt.is_null() {
                    fprintf(stderr,
                            b"Out of memory for XPath\n\x00" as *const u8 as
                                *const std::os::raw::c_char);
                    xmllint_progresult = XMLLINT_ERR_MEM;
                    return
                }
                i = 0 as std::os::raw::c_int;
                while i < (*(*cur).nodesetval).nodeNr {
                    node = *(*(*cur).nodesetval).nodeTab.offset(i as isize);
                    xmlSaveTree(ctxt, node);
                    i += 1
                }
                xmlSaveClose(ctxt);
            }
        }
        2 => {
            if (*cur).boolval != 0 {
                printf(b"true\x00" as *const u8 as *const std::os::raw::c_char);
            } else {
                printf(b"false\x00" as *const u8 as *const std::os::raw::c_char);
            }
        }
        3 => {
            match xmlXPathIsInf((*cur).floatval) {
                1 => {
                    printf(b"Infinity\x00" as *const u8 as
                               *const std::os::raw::c_char);
                }
                -1 => {
                    printf(b"-Infinity\x00" as *const u8 as
                               *const std::os::raw::c_char);
                }
                _ => {
                    if xmlXPathIsNaN((*cur).floatval) != 0 {
                        printf(b"NaN\x00" as *const u8 as
                                   *const std::os::raw::c_char);
                    } else {
                        printf(b"%0g\x00" as *const u8 as *const std::os::raw::c_char,
                               (*cur).floatval);
                    }
                }
            }
        }
        4 => {
            printf(b"%s\x00" as *const u8 as *const std::os::raw::c_char,
                   (*cur).stringval as *const std::os::raw::c_char);
        }
        0 => {
            fprintf(stderr,
                    b"XPath Object is uninitialized\n\x00" as *const u8 as
                        *const std::os::raw::c_char);
            xmllint_progresult = XMLLINT_ERR_XPATH
        }
        _ => {
            fprintf(stderr,
                    b"XPath object of unexpected type\n\x00" as *const u8 as
                        *const std::os::raw::c_char);
            xmllint_progresult = XMLLINT_ERR_XPATH
        }
    };
}
unsafe extern "C" fn doXPathQuery(mut doc: xmlDocPtr,
                                  mut query: *const std::os::raw::c_char) {
    let mut ctxt: xmlXPathContextPtr = 0 as *mut xmlXPathContext;
    let mut res: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    ctxt = xmlXPathNewContext(doc);
    if ctxt.is_null() {
        fprintf(stderr,
                b"Out of memory for XPath\n\x00" as *const u8 as
                    *const std::os::raw::c_char);
        xmllint_progresult = XMLLINT_ERR_MEM;
        return
    }
    (*ctxt).node = doc as xmlNodePtr;
    res = xmlXPathEval(query as *mut xmlChar, ctxt);
    xmlXPathFreeContext(ctxt);
    if res.is_null() {
        fprintf(stderr,
                b"XPath evaluation failure\n\x00" as *const u8 as
                    *const std::os::raw::c_char);
        xmllint_progresult = XMLLINT_ERR_XPATH;
        return
    }
    doXPathDump(res);
    xmlXPathFreeObject(res);
}
/* LIBXML_XPATH_ENABLED */
/* ***********************************************************************
 *									*
 *			Tree Test processing				*
 *									*
 ************************************************************************/
unsafe extern "C" fn parseAndPrintFile(mut filename: *mut std::os::raw::c_char,
                                       mut rectxt: xmlParserCtxtPtr) {
    let mut doc: xmlDocPtr = 0 as xmlDocPtr;
    let mut tmp: xmlDocPtr = 0 as *mut xmlDoc;
    /* LIBXML_TREE_ENABLED */
    if timing != 0 && repeat == 0 { xmllint_startTimer(); }
    if filename.is_null() {
        if generate != 0 {
            let mut n: xmlNodePtr = 0 as *mut xmlNode;
            doc =
                xmlNewDoc(b"1.0\x00" as *const u8 as *const std::os::raw::c_char as
                              *mut xmlChar);
            n =
                xmlNewDocNode(doc, 0 as xmlNsPtr,
                              b"info\x00" as *const u8 as *const std::os::raw::c_char
                                  as *mut xmlChar, 0 as *const xmlChar);
            xmlNodeSetContent(n,
                              b"abc\x00" as *const u8 as *const std::os::raw::c_char
                                  as *mut xmlChar);
            xmlDocSetRootElement(doc, n);
        }
    } else if html != 0 && push != 0 {
        let mut f: *mut FILE = 0 as *mut FILE;
        f = fopen(filename, b"r\x00" as *const u8 as *const std::os::raw::c_char);
        if !f.is_null() {
            let mut res: std::os::raw::c_int = 0;
            let mut chars: [std::os::raw::c_char; 4096] = [0; 4096];
            let mut ctxt: htmlParserCtxtPtr = 0 as *mut xmlParserCtxt;
            res =
                fread(chars.as_mut_ptr() as *mut std::os::raw::c_void,
                      1 as std::os::raw::c_int as size_t, 4 as std::os::raw::c_int as size_t,
                      f) as std::os::raw::c_int;
            if res > 0 as std::os::raw::c_int {
                ctxt =
                    htmlCreatePushParserCtxt(0 as htmlSAXHandlerPtr,
                                             0 as *mut std::os::raw::c_void,
                                             chars.as_mut_ptr(), res,
                                             filename,
                                             XML_CHAR_ENCODING_NONE);
                xmlCtxtUseOptions(ctxt, options);
                loop  {
                    res =
                        fread(chars.as_mut_ptr() as *mut std::os::raw::c_void,
                              1 as std::os::raw::c_int as size_t, pushsize as size_t,
                              f) as std::os::raw::c_int;
                    if !(res > 0 as std::os::raw::c_int) { break ; }
                    htmlParseChunk(ctxt, chars.as_mut_ptr(), res,
                                   0 as std::os::raw::c_int);
                }
                htmlParseChunk(ctxt, chars.as_mut_ptr(), 0 as std::os::raw::c_int,
                               1 as std::os::raw::c_int);
                doc = (*ctxt).myDoc;
                htmlFreeParserCtxt(ctxt);
            }
            fclose(f);
        }
    } else if html != 0 && memory != 0 {
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
        let mut base: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
        if stat(filename, &mut info) < 0 as std::os::raw::c_int { return }
        fd = open(filename, 0 as std::os::raw::c_int);
        if fd < 0 as std::os::raw::c_int { return }
        base =
            mmap(0 as *mut std::os::raw::c_void, info.st_size as size_t,
                 0x1 as std::os::raw::c_int, 0x1 as std::os::raw::c_int, fd,
                 0 as std::os::raw::c_int as __off64_t) as *const std::os::raw::c_char;
        if base ==
               -(1 as std::os::raw::c_int) as *mut std::os::raw::c_void as *const std::os::raw::c_char
           {
            close(fd);
            fprintf(stderr,
                    b"mmap failure for file %s\n\x00" as *const u8 as
                        *const std::os::raw::c_char, filename);
            xmllint_progresult = XMLLINT_ERR_RDFILE;
            return
        }
        doc =
            htmlReadMemory(base as *mut std::os::raw::c_char,
                           info.st_size as std::os::raw::c_int, filename,
                           0 as *const std::os::raw::c_char, options);
        munmap(base as *mut std::os::raw::c_char as *mut std::os::raw::c_void,
               info.st_size as size_t);
        close(fd);
    } else if html != 0 {
        doc = htmlReadFile(filename, 0 as *const std::os::raw::c_char, options)
    } else if push != 0 {
        let mut f_0: *mut FILE = 0 as *mut FILE;
        /* LIBXML_TREE_ENABLED */
        /* LIBXML_PUSH_ENABLED */
        /* LIBXML_HTML_ENABLED */
        /*
	 * build an XML tree from a string;
	 */
        /* '-' Usually means stdin -<sven@zen.org> */
        if *filename.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
               '-' as i32 &&
               *filename.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                   0 as std::os::raw::c_int {
            f_0 = stdin
        } else {
            f_0 =
                fopen(filename, b"r\x00" as *const u8 as *const std::os::raw::c_char)
        }
        if !f_0.is_null() {
            let mut ret: std::os::raw::c_int = 0;
            let mut res_0: std::os::raw::c_int = 0;
            let mut size: std::os::raw::c_int = 1024 as std::os::raw::c_int;
            let mut chars_0: [std::os::raw::c_char; 1024] = [0; 1024];
            let mut ctxt_0: xmlParserCtxtPtr = 0 as *mut xmlParserCtxt;
            /* if (repeat) size = 1024; */
            res_0 =
                fread(chars_0.as_mut_ptr() as *mut std::os::raw::c_void,
                      1 as std::os::raw::c_int as size_t, 4 as std::os::raw::c_int as size_t,
                      f_0) as std::os::raw::c_int;
            if res_0 > 0 as std::os::raw::c_int {
                ctxt_0 =
                    xmlCreatePushParserCtxt(0 as xmlSAXHandlerPtr,
                                            0 as *mut std::os::raw::c_void,
                                            chars_0.as_mut_ptr(), res_0,
                                            filename);
                xmlCtxtUseOptions(ctxt_0, options);
                loop  {
                    res_0 =
                        fread(chars_0.as_mut_ptr() as *mut std::os::raw::c_void,
                              1 as std::os::raw::c_int as size_t, size as size_t, f_0)
                            as std::os::raw::c_int;
                    if !(res_0 > 0 as std::os::raw::c_int) { break ; }
                    xmlParseChunk(ctxt_0, chars_0.as_mut_ptr(), res_0,
                                  0 as std::os::raw::c_int);
                }
                xmlParseChunk(ctxt_0, chars_0.as_mut_ptr(), 0 as std::os::raw::c_int,
                              1 as std::os::raw::c_int);
                doc = (*ctxt_0).myDoc;
                ret = (*ctxt_0).wellFormed;
                xmlFreeParserCtxt(ctxt_0);
                if ret == 0 { xmlFreeDoc(doc); doc = 0 as xmlDocPtr }
            }
            if f_0 != stdin { fclose(f_0); }
        }
    } else if testIO != 0 {
        if *filename.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
               '-' as i32 &&
               *filename.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                   0 as std::os::raw::c_int {
            doc =
                xmlReadFd(0 as std::os::raw::c_int, 0 as *const std::os::raw::c_char,
                          0 as *const std::os::raw::c_char, options)
        } else {
            let mut f_1: *mut FILE = 0 as *mut FILE;
            f_1 =
                fopen(filename, b"r\x00" as *const u8 as *const std::os::raw::c_char);
            if !f_1.is_null() {
                if rectxt.is_null() {
                    doc =
                        xmlReadIO(Some(myRead as
                                           unsafe extern "C" fn(_:
                                                                    *mut std::os::raw::c_void,
                                                                _:
                                                                    *mut std::os::raw::c_char,
                                                                _:
                                                                    std::os::raw::c_int)
                                               -> std::os::raw::c_int),
                                  Some(myClose as
                                           unsafe extern "C" fn(_:
                                                                    *mut std::os::raw::c_void)
                                               -> std::os::raw::c_int),
                                  f_1 as *mut std::os::raw::c_void, filename,
                                  0 as *const std::os::raw::c_char, options)
                } else {
                    doc =
                        xmlCtxtReadIO(rectxt,
                                      Some(myRead as
                                               unsafe extern "C" fn(_:
                                                                        *mut std::os::raw::c_void,
                                                                    _:
                                                                        *mut std::os::raw::c_char,
                                                                    _:
                                                                        std::os::raw::c_int)
                                                   -> std::os::raw::c_int),
                                      Some(myClose as
                                               unsafe extern "C" fn(_:
                                                                        *mut std::os::raw::c_void)
                                                   -> std::os::raw::c_int),
                                      f_1 as *mut std::os::raw::c_void, filename,
                                      0 as *const std::os::raw::c_char, options)
                }
            } else { doc = 0 as xmlDocPtr }
        }
    } else if htmlout != 0 {
        let mut ctxt_1: xmlParserCtxtPtr = 0 as *mut xmlParserCtxt;
        if rectxt.is_null() {
            ctxt_1 = xmlNewParserCtxt()
        } else { ctxt_1 = rectxt }
        if ctxt_1.is_null() {
            doc = 0 as xmlDocPtr
        } else {
            (*(*ctxt_1).sax).error =
                Some(xmlHTMLError as
                         unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                              _: *const std::os::raw::c_char, _: ...)
                             -> ());
            (*(*ctxt_1).sax).warning =
                Some(xmlHTMLWarning as
                         unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                              _: *const std::os::raw::c_char, _: ...)
                             -> ());
            (*ctxt_1).vctxt.error =
                Some(xmlHTMLValidityError as
                         unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                              _: *const std::os::raw::c_char, _: ...)
                             -> ());
            (*ctxt_1).vctxt.warning =
                Some(xmlHTMLValidityWarning as
                         unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                              _: *const std::os::raw::c_char, _: ...)
                             -> ());
            doc =
                xmlCtxtReadFile(ctxt_1, filename, 0 as *const std::os::raw::c_char,
                                options);
            if rectxt.is_null() { xmlFreeParserCtxt(ctxt_1); }
        }
    } else if memory != 0 {
        let mut fd_0: std::os::raw::c_int = 0;
        let mut info_0: stat =
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
        let mut base_0: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
        if stat(filename, &mut info_0) < 0 as std::os::raw::c_int { return }
        fd_0 = open(filename, 0 as std::os::raw::c_int);
        if fd_0 < 0 as std::os::raw::c_int { return }
        base_0 =
            mmap(0 as *mut std::os::raw::c_void, info_0.st_size as size_t,
                 0x1 as std::os::raw::c_int, 0x1 as std::os::raw::c_int, fd_0,
                 0 as std::os::raw::c_int as __off64_t) as *const std::os::raw::c_char;
        if base_0 ==
               -(1 as std::os::raw::c_int) as *mut std::os::raw::c_void as *const std::os::raw::c_char
           {
            close(fd_0);
            fprintf(stderr,
                    b"mmap failure for file %s\n\x00" as *const u8 as
                        *const std::os::raw::c_char, filename);
            xmllint_progresult = XMLLINT_ERR_RDFILE;
            return
        }
        if rectxt.is_null() {
            doc =
                xmlReadMemory(base_0 as *mut std::os::raw::c_char,
                              info_0.st_size as std::os::raw::c_int, filename,
                              0 as *const std::os::raw::c_char, options)
        } else {
            doc =
                xmlCtxtReadMemory(rectxt, base_0 as *mut std::os::raw::c_char,
                                  info_0.st_size as std::os::raw::c_int, filename,
                                  0 as *const std::os::raw::c_char, options)
        }
        munmap(base_0 as *mut std::os::raw::c_char as *mut std::os::raw::c_void,
               info_0.st_size as size_t);
        close(fd_0);
    } else if valid != 0 {
        let mut ctxt_2: xmlParserCtxtPtr = 0 as xmlParserCtxtPtr;
        if rectxt.is_null() {
            ctxt_2 = xmlNewParserCtxt()
        } else { ctxt_2 = rectxt }
        if ctxt_2.is_null() {
            doc = 0 as xmlDocPtr
        } else {
            doc =
                xmlCtxtReadFile(ctxt_2, filename, 0 as *const std::os::raw::c_char,
                                options);
            if (*ctxt_2).valid == 0 as std::os::raw::c_int {
                xmllint_progresult = XMLLINT_ERR_RDFILE
            }
            if rectxt.is_null() { xmlFreeParserCtxt(ctxt_2); }
        }
        /* LIBXML_PUSH_ENABLED */
        /* LIBXML_VALID_ENABLED */
    } else if !rectxt.is_null() {
        doc =
            xmlCtxtReadFile(rectxt, filename, 0 as *const std::os::raw::c_char,
                            options)
    } else if sax1 != 0 {
        doc = xmlParseFile(filename)
    } else {
        /* LIBXML_SAX1_ENABLED */
        doc = xmlReadFile(filename, 0 as *const std::os::raw::c_char, options)
    }
    /*
     * If we don't have a document we might as well give up.  Do we
     * want an error message here?  <sven@zen.org> */
    if doc.is_null() { xmllint_progresult = XMLLINT_ERR_UNCLASS; return }
    if timing != 0 && repeat == 0 {
        xmllint_endTimer(b"Parsing\x00" as *const u8 as *const std::os::raw::c_char);
    }
    /*
     * Remove DOCTYPE nodes
     */
    if dropdtd != 0 {
        let mut dtd: xmlDtdPtr = 0 as *mut xmlDtd;
        dtd = xmlGetIntSubset(doc as *const xmlDoc);
        if !dtd.is_null() {
            xmlUnlinkNode(dtd as xmlNodePtr);
            xmlFreeDtd(dtd);
        }
    }
    if xinclude != 0 {
        if timing != 0 && repeat == 0 { xmllint_startTimer(); }
        if xmlXIncludeProcessFlags(doc, options) < 0 as std::os::raw::c_int {
            xmllint_progresult = XMLLINT_ERR_UNCLASS
        }
        if timing != 0 && repeat == 0 {
            xmllint_endTimer(b"Xinclude processing\x00" as *const u8 as
                                 *const std::os::raw::c_char);
        }
    }
    if !xpathquery.is_null() { doXPathQuery(doc, xpathquery); }
    /*
     * shell interaction
     */
    if shell != 0 {
        xmlXPathOrderDocElems(doc);
        xmlShell(doc, filename,
                 Some(xmlShellReadline as
                          unsafe extern "C" fn(_: *mut std::os::raw::c_char)
                              -> *mut std::os::raw::c_char), stdout);
    }
    /*
     * test intermediate copy if needed.
     */
    if copy != 0 {
        tmp = doc;
        if timing != 0 { xmllint_startTimer(); }
        doc = xmlCopyDoc(doc, 1 as std::os::raw::c_int);
        if timing != 0 {
            xmllint_endTimer(b"Copying\x00" as *const u8 as
                                 *const std::os::raw::c_char);
        }
        if timing != 0 { xmllint_startTimer(); }
        xmlFreeDoc(tmp);
        if timing != 0 {
            xmllint_endTimer(b"Freeing original\x00" as *const u8 as
                                 *const std::os::raw::c_char);
        }
    }
    /* LIBXML_TREE_ENABLED */
    if insert != 0 && html == 0 {
        let mut list: [*const xmlChar; 256] = [0 as *const xmlChar; 256];
        let mut nb: std::os::raw::c_int = 0;
        let mut i: std::os::raw::c_int = 0;
        let mut node: xmlNodePtr = 0 as *mut xmlNode;
        if !(*doc).children.is_null() {
            node = (*doc).children;
            while !node.is_null() && (*node).last.is_null() {
                node = (*node).next
            }
            if !node.is_null() {
                nb =
                    xmlValidGetValidElements((*node).last, 0 as *mut xmlNode,
                                             list.as_mut_ptr(),
                                             256 as std::os::raw::c_int);
                if nb < 0 as std::os::raw::c_int {
                    fprintf(stderr,
                            b"could not get valid list of elements\n\x00" as
                                *const u8 as *const std::os::raw::c_char);
                } else if nb == 0 as std::os::raw::c_int {
                    fprintf(stderr,
                            b"No element can be inserted under root\n\x00" as
                                *const u8 as *const std::os::raw::c_char);
                } else {
                    fprintf(stderr,
                            b"%d element types can be inserted under root:\n\x00"
                                as *const u8 as *const std::os::raw::c_char, nb);
                    i = 0 as std::os::raw::c_int;
                    while i < nb {
                        fprintf(stderr,
                                b"%s\n\x00" as *const u8 as
                                    *const std::os::raw::c_char,
                                list[i as usize] as *mut std::os::raw::c_char);
                        i += 1
                    }
                }
            }
        }
    } else if walker != 0 { walkDoc(doc); }
    /* LIBXML_VALID_ENABLED */
    /* LIBXML_READER_ENABLED */
    if xmllint_noout == 0 as std::os::raw::c_int {
        let mut ret_0: std::os::raw::c_int = 0;
        /*
	 * print it.
	 */
        if debug == 0 {
            if timing != 0 && repeat == 0 { xmllint_startTimer(); }
            if html != 0 && xmlout == 0 {
                if compress != 0 {
                    htmlSaveFile(if !output.is_null() {
                                     output
                                 } else {
                                     b"-\x00" as *const u8 as
                                         *const std::os::raw::c_char
                                 }, doc);
                } else if !encoding.is_null() {
                    if format == 1 as std::os::raw::c_int {
                        htmlSaveFileFormat(if !output.is_null() {
                                               output
                                           } else {
                                               b"-\x00" as *const u8 as
                                                   *const std::os::raw::c_char
                                           }, doc, encoding,
                                           1 as std::os::raw::c_int);
                    } else {
                        htmlSaveFileFormat(if !output.is_null() {
                                               output
                                           } else {
                                               b"-\x00" as *const u8 as
                                                   *const std::os::raw::c_char
                                           }, doc, encoding,
                                           0 as std::os::raw::c_int);
                    }
                } else if format == 1 as std::os::raw::c_int {
                    htmlSaveFileFormat(if !output.is_null() {
                                           output
                                       } else {
                                           b"-\x00" as *const u8 as
                                               *const std::os::raw::c_char
                                       }, doc, 0 as *const std::os::raw::c_char,
                                       1 as std::os::raw::c_int);
                } else {
                    let mut out: *mut FILE = 0 as *mut FILE;
                    if output.is_null() {
                        out = stdout
                    } else {
                        out =
                            fopen(output,
                                  b"wb\x00" as *const u8 as
                                      *const std::os::raw::c_char)
                    }
                    if !out.is_null() {
                        if htmlDocDump(out, doc) < 0 as std::os::raw::c_int {
                            xmllint_progresult = XMLLINT_ERR_OUT
                        }
                        if !output.is_null() { fclose(out); }
                    } else {
                        fprintf(stderr,
                                b"failed to open %s\n\x00" as *const u8 as
                                    *const std::os::raw::c_char, output);
                        xmllint_progresult = XMLLINT_ERR_OUT
                    }
                }
                if timing != 0 && repeat == 0 {
                    xmllint_endTimer(b"Saving\x00" as *const u8 as
                                         *const std::os::raw::c_char);
                }
            } else if canonical != 0 {
                let mut result: *mut xmlChar = 0 as *mut xmlChar;
                let mut size_0: std::os::raw::c_int = 0;
                size_0 =
                    xmlC14NDocDumpMemory(doc, 0 as xmlNodeSetPtr,
                                         XML_C14N_1_0 as std::os::raw::c_int,
                                         0 as *mut *mut xmlChar,
                                         1 as std::os::raw::c_int, &mut result);
                if size_0 >= 0 as std::os::raw::c_int {
                    if write(1 as std::os::raw::c_int, result as *const std::os::raw::c_void,
                             size_0 as size_t) ==
                           -(1 as std::os::raw::c_int) as std::os::raw::c_long {
                        fprintf(stderr,
                                b"Can\'t write data\n\x00" as *const u8 as
                                    *const std::os::raw::c_char);
                    }
                    xmlFree.expect("non-null function pointer")(result as
                                                                    *mut std::os::raw::c_void);
                } else {
                    fprintf(stderr,
                            b"Failed to canonicalize\n\x00" as *const u8 as
                                *const std::os::raw::c_char);
                    xmllint_progresult = XMLLINT_ERR_OUT
                }
            } else if canonical_11 != 0 {
                let mut result_0: *mut xmlChar = 0 as *mut xmlChar;
                let mut size_1: std::os::raw::c_int = 0;
                size_1 =
                    xmlC14NDocDumpMemory(doc, 0 as xmlNodeSetPtr,
                                         XML_C14N_1_1 as std::os::raw::c_int,
                                         0 as *mut *mut xmlChar,
                                         1 as std::os::raw::c_int, &mut result_0);
                if size_1 >= 0 as std::os::raw::c_int {
                    if write(1 as std::os::raw::c_int,
                             result_0 as *const std::os::raw::c_void,
                             size_1 as size_t) ==
                           -(1 as std::os::raw::c_int) as std::os::raw::c_long {
                        fprintf(stderr,
                                b"Can\'t write data\n\x00" as *const u8 as
                                    *const std::os::raw::c_char);
                    }
                    xmlFree.expect("non-null function pointer")(result_0 as
                                                                    *mut std::os::raw::c_void);
                } else {
                    fprintf(stderr,
                            b"Failed to canonicalize\n\x00" as *const u8 as
                                *const std::os::raw::c_char);
                    xmllint_progresult = XMLLINT_ERR_OUT
                }
            } else if exc_canonical != 0 {
                let mut result_1: *mut xmlChar = 0 as *mut xmlChar;
                let mut size_2: std::os::raw::c_int = 0;
                size_2 =
                    xmlC14NDocDumpMemory(doc, 0 as xmlNodeSetPtr,
                                         XML_C14N_EXCLUSIVE_1_0 as
                                             std::os::raw::c_int,
                                         0 as *mut *mut xmlChar,
                                         1 as std::os::raw::c_int, &mut result_1);
                if size_2 >= 0 as std::os::raw::c_int {
                    if write(1 as std::os::raw::c_int,
                             result_1 as *const std::os::raw::c_void,
                             size_2 as size_t) ==
                           -(1 as std::os::raw::c_int) as std::os::raw::c_long {
                        fprintf(stderr,
                                b"Can\'t write data\n\x00" as *const u8 as
                                    *const std::os::raw::c_char);
                    }
                    xmlFree.expect("non-null function pointer")(result_1 as
                                                                    *mut std::os::raw::c_void);
                } else {
                    fprintf(stderr,
                            b"Failed to canonicalize\n\x00" as *const u8 as
                                *const std::os::raw::c_char);
                    xmllint_progresult = XMLLINT_ERR_OUT
                }
            } else if memory != 0 {
                let mut result_2: *mut xmlChar = 0 as *mut xmlChar;
                let mut len: std::os::raw::c_int = 0;
                if !encoding.is_null() {
                    if format == 1 as std::os::raw::c_int {
                        xmlDocDumpFormatMemoryEnc(doc, &mut result_2,
                                                  &mut len, encoding,
                                                  1 as std::os::raw::c_int);
                    } else {
                        xmlDocDumpMemoryEnc(doc, &mut result_2, &mut len,
                                            encoding);
                    }
                } else if format == 1 as std::os::raw::c_int {
                    xmlDocDumpFormatMemory(doc, &mut result_2, &mut len,
                                           1 as std::os::raw::c_int);
                } else { xmlDocDumpMemory(doc, &mut result_2, &mut len); }
                if result_2.is_null() {
                    fprintf(stderr,
                            b"Failed to save\n\x00" as *const u8 as
                                *const std::os::raw::c_char);
                    xmllint_progresult = XMLLINT_ERR_OUT
                } else {
                    if write(1 as std::os::raw::c_int,
                             result_2 as *const std::os::raw::c_void, len as size_t)
                           == -(1 as std::os::raw::c_int) as std::os::raw::c_long {
                        fprintf(stderr,
                                b"Can\'t write data\n\x00" as *const u8 as
                                    *const std::os::raw::c_char);
                    }
                    xmlFree.expect("non-null function pointer")(result_2 as
                                                                    *mut std::os::raw::c_void);
                }
            } else if compress != 0 {
                xmlSaveFile(if !output.is_null() {
                                output
                            } else {
                                b"-\x00" as *const u8 as *const std::os::raw::c_char
                            }, doc);
            } else if oldout != 0 {
                if !encoding.is_null() {
                    if format == 1 as std::os::raw::c_int {
                        ret_0 =
                            xmlSaveFormatFileEnc(if !output.is_null() {
                                                     output
                                                 } else {
                                                     b"-\x00" as *const u8 as
                                                         *const std::os::raw::c_char
                                                 }, doc, encoding,
                                                 1 as std::os::raw::c_int)
                    } else {
                        ret_0 =
                            xmlSaveFileEnc(if !output.is_null() {
                                               output
                                           } else {
                                               b"-\x00" as *const u8 as
                                                   *const std::os::raw::c_char
                                           }, doc, encoding)
                    }
                    if ret_0 < 0 as std::os::raw::c_int {
                        fprintf(stderr,
                                b"failed save to %s\n\x00" as *const u8 as
                                    *const std::os::raw::c_char,
                                if !output.is_null() {
                                    output
                                } else {
                                    b"-\x00" as *const u8 as
                                        *const std::os::raw::c_char
                                });
                        xmllint_progresult = XMLLINT_ERR_OUT
                    }
                } else if format == 1 as std::os::raw::c_int {
                    ret_0 =
                        xmlSaveFormatFile(if !output.is_null() {
                                              output
                                          } else {
                                              b"-\x00" as *const u8 as
                                                  *const std::os::raw::c_char
                                          }, doc, 1 as std::os::raw::c_int);
                    if ret_0 < 0 as std::os::raw::c_int {
                        fprintf(stderr,
                                b"failed save to %s\n\x00" as *const u8 as
                                    *const std::os::raw::c_char,
                                if !output.is_null() {
                                    output
                                } else {
                                    b"-\x00" as *const u8 as
                                        *const std::os::raw::c_char
                                });
                        xmllint_progresult = XMLLINT_ERR_OUT
                    }
                } else {
                    let mut out_0: *mut FILE = 0 as *mut FILE;
                    if output.is_null() {
                        out_0 = stdout
                    } else {
                        out_0 =
                            fopen(output,
                                  b"wb\x00" as *const u8 as
                                      *const std::os::raw::c_char)
                    }
                    if !out_0.is_null() {
                        if xmlDocDump(out_0, doc) < 0 as std::os::raw::c_int {
                            xmllint_progresult = XMLLINT_ERR_OUT
                        }
                        if !output.is_null() { fclose(out_0); }
                    } else {
                        fprintf(stderr,
                                b"failed to open %s\n\x00" as *const u8 as
                                    *const std::os::raw::c_char, output);
                        xmllint_progresult = XMLLINT_ERR_OUT
                    }
                }
            } else {
                let mut ctxt_3: xmlSaveCtxtPtr = 0 as *mut xmlSaveCtxt;
                let mut saveOpts: std::os::raw::c_int = 0 as std::os::raw::c_int;
                if format == 1 as std::os::raw::c_int {
                    saveOpts |= XML_SAVE_FORMAT as std::os::raw::c_int
                } else if format == 2 as std::os::raw::c_int {
                    saveOpts |= XML_SAVE_WSNONSIG as std::os::raw::c_int
                }
                if xmlout != 0 { saveOpts |= XML_SAVE_AS_XML as std::os::raw::c_int }
                if output.is_null() {
                    ctxt_3 = xmlSaveToFd(1 as std::os::raw::c_int, encoding, saveOpts)
                } else {
                    ctxt_3 = xmlSaveToFilename(output, encoding, saveOpts)
                }
                if !ctxt_3.is_null() {
                    if xmlSaveDoc(ctxt_3, doc) <
                           0 as std::os::raw::c_int as std::os::raw::c_long {
                        fprintf(stderr,
                                b"failed save to %s\n\x00" as *const u8 as
                                    *const std::os::raw::c_char,
                                if !output.is_null() {
                                    output
                                } else {
                                    b"-\x00" as *const u8 as
                                        *const std::os::raw::c_char
                                });
                        xmllint_progresult = XMLLINT_ERR_OUT
                    }
                    xmlSaveClose(ctxt_3);
                } else { xmllint_progresult = XMLLINT_ERR_OUT }
            }
            if timing != 0 && repeat == 0 {
                xmllint_endTimer(b"Saving\x00" as *const u8 as
                                     *const std::os::raw::c_char);
            }
        } else {
            let mut out_1: *mut FILE = 0 as *mut FILE;
            if output.is_null() {
                out_1 = stdout
            } else {
                out_1 =
                    fopen(output,
                          b"wb\x00" as *const u8 as *const std::os::raw::c_char)
            }
            if !out_1.is_null() {
                xmlDebugDumpDocument(out_1, doc);
                if !output.is_null() { fclose(out_1); }
            } else {
                fprintf(stderr,
                        b"failed to open %s\n\x00" as *const u8 as
                            *const std::os::raw::c_char, output);
                xmllint_progresult = XMLLINT_ERR_OUT
            }
        }
    }
    /* HAVE_MMAP */
    /* LIBXML_OUTPUT_ENABLED */
    /*
     * A posteriori validation test
     */
    if !dtdvalid.is_null() || !dtdvalidfpi.is_null() {
        let mut dtd_0: xmlDtdPtr = 0 as *mut xmlDtd;
        if timing != 0 && repeat == 0 { xmllint_startTimer(); }
        if !dtdvalid.is_null() {
            dtd_0 =
                xmlParseDTD(0 as *const xmlChar, dtdvalid as *const xmlChar)
        } else {
            dtd_0 =
                xmlParseDTD(dtdvalidfpi as *const xmlChar,
                            0 as *const xmlChar)
        }
        if timing != 0 && repeat == 0 {
            xmllint_endTimer(b"Parsing DTD\x00" as *const u8 as
                                 *const std::os::raw::c_char);
        }
        if dtd_0.is_null() {
            if !dtdvalid.is_null() {
                (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                           b"Could not parse DTD %s\n\x00"
                                                                               as
                                                                               *const u8
                                                                               as
                                                                               *const std::os::raw::c_char,
                                                                           dtdvalid);
            } else {
                (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                           b"Could not parse DTD %s\n\x00"
                                                                               as
                                                                               *const u8
                                                                               as
                                                                               *const std::os::raw::c_char,
                                                                           dtdvalidfpi);
            }
            xmllint_progresult = XMLLINT_ERR_DTD
        } else {
            let mut cvp: xmlValidCtxtPtr = 0 as *mut xmlValidCtxt;
            cvp = xmlNewValidCtxt();
            if cvp.is_null() {
                (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                           b"Couldn\'t allocate validation context\n\x00"
                                                                               as
                                                                               *const u8
                                                                               as
                                                                               *const std::os::raw::c_char);
                exit(-(1 as std::os::raw::c_int));
            }
            (*cvp).userData = stderr as *mut std::os::raw::c_void;
            (*cvp).error =
                ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                        *mut FILE,
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
            (*cvp).warning =
                ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                        *mut FILE,
                                                                    _:
                                                                        *const std::os::raw::c_char,
                                                                    _: ...)
                                                   -> std::os::raw::c_int>,
                                        xmlValidityWarningFunc>(Some(fprintf
                                                                         as
                                                                         unsafe extern "C" fn(_:
                                                                                                  *mut FILE,
                                                                                              _:
                                                                                                  *const std::os::raw::c_char,
                                                                                              _:
                                                                                                  ...)
                                                                             ->
                                                                                 std::os::raw::c_int));
            if timing != 0 && repeat == 0 { xmllint_startTimer(); }
            if xmlValidateDtd(cvp, doc, dtd_0) == 0 {
                if !dtdvalid.is_null() {
                    (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                               b"Document %s does not validate against %s\n\x00"
                                                                                   as
                                                                                   *const u8
                                                                                   as
                                                                                   *const std::os::raw::c_char,
                                                                               filename,
                                                                               dtdvalid);
                } else {
                    (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                               b"Document %s does not validate against %s\n\x00"
                                                                                   as
                                                                                   *const u8
                                                                                   as
                                                                                   *const std::os::raw::c_char,
                                                                               filename,
                                                                               dtdvalidfpi);
                }
                xmllint_progresult = XMLLINT_ERR_VALID
            }
            if timing != 0 && repeat == 0 {
                xmllint_endTimer(b"Validating against DTD\x00" as *const u8 as
                                     *const std::os::raw::c_char);
            }
            xmlFreeValidCtxt(cvp);
            xmlFreeDtd(dtd_0);
        }
    } else if postvalid != 0 {
        let mut cvp_0: xmlValidCtxtPtr = 0 as *mut xmlValidCtxt;
        cvp_0 = xmlNewValidCtxt();
        if cvp_0.is_null() {
            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                       b"Couldn\'t allocate validation context\n\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const std::os::raw::c_char);
            exit(-(1 as std::os::raw::c_int));
        }
        if timing != 0 && repeat == 0 { xmllint_startTimer(); }
        (*cvp_0).userData = stderr as *mut std::os::raw::c_void;
        (*cvp_0).error =
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
        (*cvp_0).warning =
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
        if xmlValidateDocument(cvp_0, doc) == 0 {
            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                       b"Document %s does not validate\n\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const std::os::raw::c_char,
                                                                       filename);
            xmllint_progresult = XMLLINT_ERR_VALID
        }
        if timing != 0 && repeat == 0 {
            xmllint_endTimer(b"Validating\x00" as *const u8 as
                                 *const std::os::raw::c_char);
        }
        xmlFreeValidCtxt(cvp_0);
    }
    /* LIBXML_VALID_ENABLED */
    if !wxschematron.is_null() {
        let mut ctxt_4: xmlSchematronValidCtxtPtr =
            0 as *mut xmlSchematronValidCtxt;
        let mut ret_1: std::os::raw::c_int = 0;
        let mut flag: std::os::raw::c_int = 0;
        if timing != 0 && repeat == 0 { xmllint_startTimer(); }
        if debug != 0 {
            flag = XML_SCHEMATRON_OUT_XML as std::os::raw::c_int
        } else { flag = XML_SCHEMATRON_OUT_TEXT as std::os::raw::c_int }
        if xmllint_noout != 0 {
            flag |= XML_SCHEMATRON_OUT_QUIET as std::os::raw::c_int
        }
        ctxt_4 = xmlSchematronNewValidCtxt(wxschematron, flag);
        ret_1 = xmlSchematronValidateDoc(ctxt_4, doc);
        if ret_1 == 0 as std::os::raw::c_int {
            fprintf(stderr,
                    b"%s validates\n\x00" as *const u8 as *const std::os::raw::c_char,
                    filename);
        } else if ret_1 > 0 as std::os::raw::c_int {
            fprintf(stderr,
                    b"%s fails to validate\n\x00" as *const u8 as
                        *const std::os::raw::c_char, filename);
            xmllint_progresult = XMLLINT_ERR_VALID
        } else {
            fprintf(stderr,
                    b"%s validation generated an internal error\n\x00" as
                        *const u8 as *const std::os::raw::c_char, filename);
            xmllint_progresult = XMLLINT_ERR_VALID
        }
        xmlSchematronFreeValidCtxt(ctxt_4);
        if timing != 0 && repeat == 0 {
            xmllint_endTimer(b"Validating\x00" as *const u8 as
                                 *const std::os::raw::c_char);
        }
    }
    if !relaxngschemas.is_null() {
        let mut ctxt_5: xmlRelaxNGValidCtxtPtr =
            0 as *mut xmlRelaxNGValidCtxt;
        let mut ret_2: std::os::raw::c_int = 0;
        if timing != 0 && repeat == 0 { xmllint_startTimer(); }
        ctxt_5 = xmlRelaxNGNewValidCtxt(relaxngschemas);
        xmlRelaxNGSetValidErrors(ctxt_5,
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
        ret_2 = xmlRelaxNGValidateDoc(ctxt_5, doc);
        if ret_2 == 0 as std::os::raw::c_int {
            fprintf(stderr,
                    b"%s validates\n\x00" as *const u8 as *const std::os::raw::c_char,
                    filename);
        } else if ret_2 > 0 as std::os::raw::c_int {
            fprintf(stderr,
                    b"%s fails to validate\n\x00" as *const u8 as
                        *const std::os::raw::c_char, filename);
            xmllint_progresult = XMLLINT_ERR_VALID
        } else {
            fprintf(stderr,
                    b"%s validation generated an internal error\n\x00" as
                        *const u8 as *const std::os::raw::c_char, filename);
            xmllint_progresult = XMLLINT_ERR_VALID
        }
        xmlRelaxNGFreeValidCtxt(ctxt_5);
        if timing != 0 && repeat == 0 {
            xmllint_endTimer(b"Validating\x00" as *const u8 as
                                 *const std::os::raw::c_char);
        }
    } else if !wxschemas.is_null() {
        let mut ctxt_6: xmlSchemaValidCtxtPtr = 0 as *mut xmlSchemaValidCtxt;
        let mut ret_3: std::os::raw::c_int = 0;
        if timing != 0 && repeat == 0 { xmllint_startTimer(); }
        ctxt_6 = xmlSchemaNewValidCtxt(wxschemas);
        xmlSchemaSetValidErrors(ctxt_6,
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
        ret_3 = xmlSchemaValidateDoc(ctxt_6, doc);
        if ret_3 == 0 as std::os::raw::c_int {
            fprintf(stderr,
                    b"%s validates\n\x00" as *const u8 as *const std::os::raw::c_char,
                    filename);
        } else if ret_3 > 0 as std::os::raw::c_int {
            fprintf(stderr,
                    b"%s fails to validate\n\x00" as *const u8 as
                        *const std::os::raw::c_char, filename);
            xmllint_progresult = XMLLINT_ERR_VALID
        } else {
            fprintf(stderr,
                    b"%s validation generated an internal error\n\x00" as
                        *const u8 as *const std::os::raw::c_char, filename);
            xmllint_progresult = XMLLINT_ERR_VALID
        }
        xmlSchemaFreeValidCtxt(ctxt_6);
        if timing != 0 && repeat == 0 {
            xmllint_endTimer(b"Validating\x00" as *const u8 as
                                 *const std::os::raw::c_char);
        }
    }
    if debugent != 0 && html == 0 { xmlDebugDumpEntities(stderr, doc); }
    /*
     * free it.
     */
    if timing != 0 && repeat == 0 { xmllint_startTimer(); }
    xmlFreeDoc(doc);
    if timing != 0 && repeat == 0 {
        xmllint_endTimer(b"Freeing\x00" as *const u8 as *const std::os::raw::c_char);
    };
}
/* ***********************************************************************
 *									*
 *			Usage and Main					*
 *									*
 ************************************************************************/
unsafe extern "C" fn showVersion(mut name: *const std::os::raw::c_char) {
    fprintf(stderr,
            b"%s: using libxml version %s\n\x00" as *const u8 as
                *const std::os::raw::c_char, name, *__xmlParserVersion());
    fprintf(stderr,
            b"   compiled with: \x00" as *const u8 as *const std::os::raw::c_char);
    if xmlHasFeature(XML_WITH_THREAD) != 0 {
        fprintf(stderr, b"Threads \x00" as *const u8 as *const std::os::raw::c_char);
    }
    if xmlHasFeature(XML_WITH_TREE) != 0 {
        fprintf(stderr, b"Tree \x00" as *const u8 as *const std::os::raw::c_char);
    }
    if xmlHasFeature(XML_WITH_OUTPUT) != 0 {
        fprintf(stderr, b"Output \x00" as *const u8 as *const std::os::raw::c_char);
    }
    if xmlHasFeature(XML_WITH_PUSH) != 0 {
        fprintf(stderr, b"Push \x00" as *const u8 as *const std::os::raw::c_char);
    }
    if xmlHasFeature(XML_WITH_READER) != 0 {
        fprintf(stderr, b"Reader \x00" as *const u8 as *const std::os::raw::c_char);
    }
    if xmlHasFeature(XML_WITH_PATTERN) != 0 {
        fprintf(stderr, b"Patterns \x00" as *const u8 as *const std::os::raw::c_char);
    }
    if xmlHasFeature(XML_WITH_WRITER) != 0 {
        fprintf(stderr, b"Writer \x00" as *const u8 as *const std::os::raw::c_char);
    }
    if xmlHasFeature(XML_WITH_SAX1) != 0 {
        fprintf(stderr, b"SAXv1 \x00" as *const u8 as *const std::os::raw::c_char);
    }
    if xmlHasFeature(XML_WITH_FTP) != 0 {
        fprintf(stderr, b"FTP \x00" as *const u8 as *const std::os::raw::c_char);
    }
    if xmlHasFeature(XML_WITH_HTTP) != 0 {
        fprintf(stderr, b"HTTP \x00" as *const u8 as *const std::os::raw::c_char);
    }
    if xmlHasFeature(XML_WITH_VALID) != 0 {
        fprintf(stderr, b"DTDValid \x00" as *const u8 as *const std::os::raw::c_char);
    }
    if xmlHasFeature(XML_WITH_HTML) != 0 {
        fprintf(stderr, b"HTML \x00" as *const u8 as *const std::os::raw::c_char);
    }
    if xmlHasFeature(XML_WITH_LEGACY) != 0 {
        fprintf(stderr, b"Legacy \x00" as *const u8 as *const std::os::raw::c_char);
    }
    if xmlHasFeature(XML_WITH_C14N) != 0 {
        fprintf(stderr, b"C14N \x00" as *const u8 as *const std::os::raw::c_char);
    }
    if xmlHasFeature(XML_WITH_CATALOG) != 0 {
        fprintf(stderr, b"Catalog \x00" as *const u8 as *const std::os::raw::c_char);
    }
    if xmlHasFeature(XML_WITH_XPATH) != 0 {
        fprintf(stderr, b"XPath \x00" as *const u8 as *const std::os::raw::c_char);
    }
    if xmlHasFeature(XML_WITH_XPTR) != 0 {
        fprintf(stderr, b"XPointer \x00" as *const u8 as *const std::os::raw::c_char);
    }
    if xmlHasFeature(XML_WITH_XINCLUDE) != 0 {
        fprintf(stderr, b"XInclude \x00" as *const u8 as *const std::os::raw::c_char);
    }
    if xmlHasFeature(XML_WITH_ICONV) != 0 {
        fprintf(stderr, b"Iconv \x00" as *const u8 as *const std::os::raw::c_char);
    }
    if xmlHasFeature(XML_WITH_ICU) != 0 {
        fprintf(stderr, b"ICU \x00" as *const u8 as *const std::os::raw::c_char);
    }
    if xmlHasFeature(XML_WITH_ISO8859X) != 0 {
        fprintf(stderr, b"ISO8859X \x00" as *const u8 as *const std::os::raw::c_char);
    }
    if xmlHasFeature(XML_WITH_UNICODE) != 0 {
        fprintf(stderr, b"Unicode \x00" as *const u8 as *const std::os::raw::c_char);
    }
    if xmlHasFeature(XML_WITH_REGEXP) != 0 {
        fprintf(stderr, b"Regexps \x00" as *const u8 as *const std::os::raw::c_char);
    }
    if xmlHasFeature(XML_WITH_AUTOMATA) != 0 {
        fprintf(stderr, b"Automata \x00" as *const u8 as *const std::os::raw::c_char);
    }
    if xmlHasFeature(XML_WITH_EXPR) != 0 {
        fprintf(stderr, b"Expr \x00" as *const u8 as *const std::os::raw::c_char);
    }
    if xmlHasFeature(XML_WITH_SCHEMAS) != 0 {
        fprintf(stderr, b"Schemas \x00" as *const u8 as *const std::os::raw::c_char);
    }
    if xmlHasFeature(XML_WITH_SCHEMATRON) != 0 {
        fprintf(stderr,
                b"Schematron \x00" as *const u8 as *const std::os::raw::c_char);
    }
    if xmlHasFeature(XML_WITH_MODULES) != 0 {
        fprintf(stderr, b"Modules \x00" as *const u8 as *const std::os::raw::c_char);
    }
    if xmlHasFeature(XML_WITH_DEBUG) != 0 {
        fprintf(stderr, b"Debug \x00" as *const u8 as *const std::os::raw::c_char);
    }
    if xmlHasFeature(XML_WITH_DEBUG_MEM) != 0 {
        fprintf(stderr, b"MemDebug \x00" as *const u8 as *const std::os::raw::c_char);
    }
    if xmlHasFeature(XML_WITH_DEBUG_RUN) != 0 {
        fprintf(stderr, b"RunDebug \x00" as *const u8 as *const std::os::raw::c_char);
    }
    if xmlHasFeature(XML_WITH_ZLIB) != 0 {
        fprintf(stderr, b"Zlib \x00" as *const u8 as *const std::os::raw::c_char);
    }
    if xmlHasFeature(XML_WITH_LZMA) != 0 {
        fprintf(stderr, b"Lzma \x00" as *const u8 as *const std::os::raw::c_char);
    }
    fprintf(stderr, b"\n\x00" as *const u8 as *const std::os::raw::c_char);
}
unsafe extern "C" fn usage(mut f: *mut FILE, mut name: *const std::os::raw::c_char) {
    fprintf(f,
            b"Usage : %s [options] XMLfiles ...\n\x00" as *const u8 as
                *const std::os::raw::c_char, name);
    fprintf(f,
            b"\tParse the XML files and output the result of the parsing\n\x00"
                as *const u8 as *const std::os::raw::c_char);
    /* LIBXML_OUTPUT_ENABLED */
    fprintf(f,
            b"\t--version : display the version of the XML library used\n\x00"
                as *const u8 as *const std::os::raw::c_char);
    fprintf(f,
            b"\t--debug : dump a debug tree of the in-memory document\n\x00"
                as *const u8 as *const std::os::raw::c_char);
    fprintf(f,
            b"\t--shell : run a navigating shell\n\x00" as *const u8 as
                *const std::os::raw::c_char);
    fprintf(f,
            b"\t--debugent : debug the entities defined in the document\n\x00"
                as *const u8 as *const std::os::raw::c_char);
    fprintf(f,
            b"\t--copy : used to test the internal copy implementation\n\x00"
                as *const u8 as *const std::os::raw::c_char);
    /* LIBXML_TREE_ENABLED */
    fprintf(f,
            b"\t--recover : output what was parsable on broken XML documents\n\x00"
                as *const u8 as *const std::os::raw::c_char);
    fprintf(f,
            b"\t--huge : remove any internal arbitrary parser limits\n\x00" as
                *const u8 as *const std::os::raw::c_char);
    fprintf(f,
            b"\t--noent : substitute entity references by their value\n\x00"
                as *const u8 as *const std::os::raw::c_char);
    fprintf(f,
            b"\t--noenc : ignore any encoding specified inside the document\n\x00"
                as *const u8 as *const std::os::raw::c_char);
    fprintf(f,
            b"\t--noout : don\'t output the result tree\n\x00" as *const u8 as
                *const std::os::raw::c_char);
    fprintf(f,
            b"\t--path \'paths\': provide a set of paths for resources\n\x00"
                as *const u8 as *const std::os::raw::c_char);
    fprintf(f,
            b"\t--load-trace : print trace of all external entities loaded\n\x00"
                as *const u8 as *const std::os::raw::c_char);
    fprintf(f,
            b"\t--nonet : refuse to fetch DTDs or entities over network\n\x00"
                as *const u8 as *const std::os::raw::c_char);
    fprintf(f,
            b"\t--nocompact : do not generate compact text nodes\n\x00" as
                *const u8 as *const std::os::raw::c_char);
    fprintf(f,
            b"\t--htmlout : output results as HTML\n\x00" as *const u8 as
                *const std::os::raw::c_char);
    fprintf(f,
            b"\t--nowrap : do not put HTML doc wrapper\n\x00" as *const u8 as
                *const std::os::raw::c_char);
    fprintf(f,
            b"\t--valid : validate the document in addition to std well-formed check\n\x00"
                as *const u8 as *const std::os::raw::c_char);
    fprintf(f,
            b"\t--postvalid : do a posteriori validation, i.e after parsing\n\x00"
                as *const u8 as *const std::os::raw::c_char);
    fprintf(f,
            b"\t--dtdvalid URL : do a posteriori validation against a given DTD\n\x00"
                as *const u8 as *const std::os::raw::c_char);
    fprintf(f,
            b"\t--dtdvalidfpi FPI : same but name the DTD with a Public Identifier\n\x00"
                as *const u8 as *const std::os::raw::c_char);
    /* LIBXML_VALID_ENABLED */
    fprintf(f,
            b"\t--timing : print some timings\n\x00" as *const u8 as
                *const std::os::raw::c_char);
    fprintf(f,
            b"\t--output file or -o file: save to a given file\n\x00" as
                *const u8 as *const std::os::raw::c_char);
    fprintf(f,
            b"\t--repeat : repeat 100 times, for timing or profiling\n\x00" as
                *const u8 as *const std::os::raw::c_char);
    fprintf(f,
            b"\t--insert : ad-hoc test for valid insertions\n\x00" as
                *const u8 as *const std::os::raw::c_char);
    fprintf(f,
            b"\t--compress : turn on gzip compression of output\n\x00" as
                *const u8 as *const std::os::raw::c_char);
    /* LIBXML_OUTPUT_ENABLED */
    fprintf(f,
            b"\t--html : use the HTML parser\n\x00" as *const u8 as
                *const std::os::raw::c_char);
    fprintf(f,
            b"\t--xmlout : force to use the XML serializer when using --html\n\x00"
                as *const u8 as *const std::os::raw::c_char);
    fprintf(f,
            b"\t--nodefdtd : do not default HTML doctype\n\x00" as *const u8
                as *const std::os::raw::c_char);
    fprintf(f,
            b"\t--push : use the push mode of the parser\n\x00" as *const u8
                as *const std::os::raw::c_char);
    fprintf(f,
            b"\t--pushsmall : use the push mode of the parser using tiny increments\n\x00"
                as *const u8 as *const std::os::raw::c_char);
    /* LIBXML_PUSH_ENABLED */
    fprintf(f,
            b"\t--memory : parse from memory\n\x00" as *const u8 as
                *const std::os::raw::c_char);
    fprintf(f,
            b"\t--maxmem nbbytes : limits memory allocation to nbbytes bytes\n\x00"
                as *const u8 as *const std::os::raw::c_char);
    fprintf(f,
            b"\t--nowarning : do not emit warnings from parser/validator\n\x00"
                as *const u8 as *const std::os::raw::c_char);
    fprintf(f,
            b"\t--noblanks : drop (ignorable?) blanks spaces\n\x00" as
                *const u8 as *const std::os::raw::c_char);
    fprintf(f,
            b"\t--nocdata : replace cdata section with text nodes\n\x00" as
                *const u8 as *const std::os::raw::c_char);
    fprintf(f,
            b"\t--format : reformat/reindent the output\n\x00" as *const u8 as
                *const std::os::raw::c_char);
    fprintf(f,
            b"\t--encode encoding : output in the given encoding\n\x00" as
                *const u8 as *const std::os::raw::c_char);
    fprintf(f,
            b"\t--dropdtd : remove the DOCTYPE of the input docs\n\x00" as
                *const u8 as *const std::os::raw::c_char);
    fprintf(f,
            b"\t--pretty STYLE : pretty-print in a particular style\n\x00" as
                *const u8 as *const std::os::raw::c_char);
    fprintf(f,
            b"\t                 0 Do not pretty print\n\x00" as *const u8 as
                *const std::os::raw::c_char);
    fprintf(f,
            b"\t                 1 Format the XML content, as --format\n\x00"
                as *const u8 as *const std::os::raw::c_char);
    fprintf(f,
            b"\t                 2 Add whitespace inside tags, preserving content\n\x00"
                as *const u8 as *const std::os::raw::c_char);
    /* LIBXML_OUTPUT_ENABLED */
    fprintf(f,
            b"\t--c14n : save in W3C canonical format v1.0 (with comments)\n\x00"
                as *const u8 as *const std::os::raw::c_char);
    fprintf(f,
            b"\t--c14n11 : save in W3C canonical format v1.1 (with comments)\n\x00"
                as *const u8 as *const std::os::raw::c_char);
    fprintf(f,
            b"\t--exc-c14n : save in W3C exclusive canonical format (with comments)\n\x00"
                as *const u8 as *const std::os::raw::c_char);
    /* LIBXML_C14N_ENABLED */
    fprintf(f,
            b"\t--nsclean : remove redundant namespace declarations\n\x00" as
                *const u8 as *const std::os::raw::c_char);
    fprintf(f,
            b"\t--testIO : test user I/O support\n\x00" as *const u8 as
                *const std::os::raw::c_char);
    fprintf(f,
            b"\t--catalogs : use SGML catalogs from $SGML_CATALOG_FILES\n\x00"
                as *const u8 as *const std::os::raw::c_char);
    fprintf(f,
            b"\t             otherwise XML Catalogs starting from \n\x00" as
                *const u8 as *const std::os::raw::c_char);
    fprintf(f,
            b"\t         %s are activated by default\n\x00" as *const u8 as
                *const std::os::raw::c_char,
            b"file:///etc/xml/catalog\x00" as *const u8 as
                *const std::os::raw::c_char);
    fprintf(f,
            b"\t--nocatalogs: deactivate all catalogs\n\x00" as *const u8 as
                *const std::os::raw::c_char);
    fprintf(f,
            b"\t--auto : generate a small doc on the fly\n\x00" as *const u8
                as *const std::os::raw::c_char);
    fprintf(f,
            b"\t--xinclude : do XInclude processing\n\x00" as *const u8 as
                *const std::os::raw::c_char);
    fprintf(f,
            b"\t--noxincludenode : same but do not generate XInclude nodes\n\x00"
                as *const u8 as *const std::os::raw::c_char);
    fprintf(f,
            b"\t--nofixup-base-uris : do not fixup xml:base uris\n\x00" as
                *const u8 as *const std::os::raw::c_char);
    fprintf(f,
            b"\t--loaddtd : fetch external DTD\n\x00" as *const u8 as
                *const std::os::raw::c_char);
    fprintf(f,
            b"\t--dtdattr : loaddtd + populate the tree with inherited attributes \n\x00"
                as *const u8 as *const std::os::raw::c_char);
    fprintf(f,
            b"\t--stream : use the streaming interface to process very large files\n\x00"
                as *const u8 as *const std::os::raw::c_char);
    fprintf(f,
            b"\t--walker : create a reader and walk though the resulting doc\n\x00"
                as *const u8 as *const std::os::raw::c_char);
    /* LIBXML_READER_ENABLED */
    fprintf(f,
            b"\t--pattern pattern_value : test the pattern support\n\x00" as
                *const u8 as *const std::os::raw::c_char);
    fprintf(f,
            b"\t--chkregister : verify the node registration code\n\x00" as
                *const u8 as *const std::os::raw::c_char);
    fprintf(f,
            b"\t--relaxng schema : do RelaxNG validation against the schema\n\x00"
                as *const u8 as *const std::os::raw::c_char);
    fprintf(f,
            b"\t--schema schema : do validation against the WXS schema\n\x00"
                as *const u8 as *const std::os::raw::c_char);
    fprintf(f,
            b"\t--schematron schema : do validation against a schematron\n\x00"
                as *const u8 as *const std::os::raw::c_char);
    fprintf(f,
            b"\t--sax1: use the old SAX1 interfaces for processing\n\x00" as
                *const u8 as *const std::os::raw::c_char);
    fprintf(f,
            b"\t--sax: do not build a tree but work just at the SAX level\n\x00"
                as *const u8 as *const std::os::raw::c_char);
    fprintf(f,
            b"\t--oldxml10: use XML-1.0 parsing rules before the 5th edition\n\x00"
                as *const u8 as *const std::os::raw::c_char);
    fprintf(f,
            b"\t--xpath expr: evaluate the XPath expression, imply --noout\n\x00"
                as *const u8 as *const std::os::raw::c_char);
    fprintf(f,
            b"\nLibxml project home page: http://xmlsoft.org/\n\x00" as
                *const u8 as *const std::os::raw::c_char);
    fprintf(f,
            b"To report bugs or get some help check: http://xmlsoft.org/bugs.html\n\x00"
                as *const u8 as *const std::os::raw::c_char);
}
unsafe extern "C" fn registerNode(mut node: xmlNodePtr) {
    (*node)._private =
        malloc(::std::mem::size_of::<std::os::raw::c_long>() as std::os::raw::c_ulong);
    if (*node)._private.is_null() {
        fprintf(stderr,
                b"Out of memory in xmllint:registerNode()\n\x00" as *const u8
                    as *const std::os::raw::c_char);
        exit(XMLLINT_ERR_MEM as std::os::raw::c_int);
    }
    *((*node)._private as *mut std::os::raw::c_long) =
        0x81726354 as std::os::raw::c_uint as std::os::raw::c_long;
    nbregister += 1;
}
unsafe extern "C" fn deregisterNode(mut node: xmlNodePtr) {
    if !(*node)._private.is_null() {
    } else {
        __assert_fail(b"node->_private != NULL\x00" as *const u8 as
                          *const std::os::raw::c_char,
                      b"xmllint.c\x00" as *const u8 as *const std::os::raw::c_char,
                      3110 as std::os::raw::c_int as std::os::raw::c_uint,
                      (*::std::mem::transmute::<&[u8; 32],
                                                &[std::os::raw::c_char; 32]>(b"void deregisterNode(xmlNodePtr)\x00")).as_ptr());
    }
    if *((*node)._private as *mut std::os::raw::c_long) ==
           0x81726354 as std::os::raw::c_uint as std::os::raw::c_long {
    } else {
        __assert_fail(b"*(long*)node->_private == (long) 0x81726354\x00" as
                          *const u8 as *const std::os::raw::c_char,
                      b"xmllint.c\x00" as *const u8 as *const std::os::raw::c_char,
                      3111 as std::os::raw::c_int as std::os::raw::c_uint,
                      (*::std::mem::transmute::<&[u8; 32],
                                                &[std::os::raw::c_char; 32]>(b"void deregisterNode(xmlNodePtr)\x00")).as_ptr());
    }
    free((*node)._private);
    nbregister -= 1;
}
unsafe fn main_0(mut argc: std::os::raw::c_int, mut argv: *mut *mut std::os::raw::c_char)
 -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    let mut acount: std::os::raw::c_int = 0;
    let mut files: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut version: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut indent: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    if argc <= 1 as std::os::raw::c_int {
        usage(stderr, *argv.offset(0 as std::os::raw::c_int as isize));
        return 1 as std::os::raw::c_int
    }
    xmlCheckVersion(20908 as std::os::raw::c_int);
    i = 1 as std::os::raw::c_int;
    while i < argc {
        if strcmp(*argv.offset(i as isize),
                  b"-\x00" as *const u8 as *const std::os::raw::c_char) == 0 {
            break ;
        }
        if !(*(*argv.offset(i as isize)).offset(0 as std::os::raw::c_int as isize) as
                 std::os::raw::c_int != '-' as i32) {
            if strcmp(*argv.offset(i as isize),
                      b"-debug\x00" as *const u8 as *const std::os::raw::c_char) == 0
                   ||
                   strcmp(*argv.offset(i as isize),
                          b"--debug\x00" as *const u8 as *const std::os::raw::c_char)
                       == 0 {
                debug += 1
            } else if strcmp(*argv.offset(i as isize),
                             b"-shell\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 ||
                          strcmp(*argv.offset(i as isize),
                                 b"--shell\x00" as *const u8 as
                                     *const std::os::raw::c_char) == 0 {
                shell += 1;
                xmllint_noout = 1 as std::os::raw::c_int
            } else if strcmp(*argv.offset(i as isize),
                             b"-copy\x00" as *const u8 as *const std::os::raw::c_char)
                          == 0 ||
                          strcmp(*argv.offset(i as isize),
                                 b"--copy\x00" as *const u8 as
                                     *const std::os::raw::c_char) == 0 {
                copy += 1
            } else if strcmp(*argv.offset(i as isize),
                             b"-recover\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 ||
                          strcmp(*argv.offset(i as isize),
                                 b"--recover\x00" as *const u8 as
                                     *const std::os::raw::c_char) == 0 {
                recovery += 1;
                options |= XML_PARSE_RECOVER as std::os::raw::c_int
            } else if strcmp(*argv.offset(i as isize),
                             b"-huge\x00" as *const u8 as *const std::os::raw::c_char)
                          == 0 ||
                          strcmp(*argv.offset(i as isize),
                                 b"--huge\x00" as *const u8 as
                                     *const std::os::raw::c_char) == 0 {
                options |= XML_PARSE_HUGE as std::os::raw::c_int
            } else if strcmp(*argv.offset(i as isize),
                             b"-noent\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 ||
                          strcmp(*argv.offset(i as isize),
                                 b"--noent\x00" as *const u8 as
                                     *const std::os::raw::c_char) == 0 {
                noent += 1;
                options |= XML_PARSE_NOENT as std::os::raw::c_int
            } else if strcmp(*argv.offset(i as isize),
                             b"-noenc\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 ||
                          strcmp(*argv.offset(i as isize),
                                 b"--noenc\x00" as *const u8 as
                                     *const std::os::raw::c_char) == 0 {
                noenc += 1;
                options |= XML_PARSE_IGNORE_ENC as std::os::raw::c_int
            } else if strcmp(*argv.offset(i as isize),
                             b"-nsclean\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 ||
                          strcmp(*argv.offset(i as isize),
                                 b"--nsclean\x00" as *const u8 as
                                     *const std::os::raw::c_char) == 0 {
                options |= XML_PARSE_NSCLEAN as std::os::raw::c_int
            } else if strcmp(*argv.offset(i as isize),
                             b"-nocdata\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 ||
                          strcmp(*argv.offset(i as isize),
                                 b"--nocdata\x00" as *const u8 as
                                     *const std::os::raw::c_char) == 0 {
                options |= XML_PARSE_NOCDATA as std::os::raw::c_int
            } else if strcmp(*argv.offset(i as isize),
                             b"-nodict\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 ||
                          strcmp(*argv.offset(i as isize),
                                 b"--nodict\x00" as *const u8 as
                                     *const std::os::raw::c_char) == 0 {
                options |= XML_PARSE_NODICT as std::os::raw::c_int
            } else if strcmp(*argv.offset(i as isize),
                             b"-version\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 ||
                          strcmp(*argv.offset(i as isize),
                                 b"--version\x00" as *const u8 as
                                     *const std::os::raw::c_char) == 0 {
                showVersion(*argv.offset(0 as std::os::raw::c_int as isize));
                version = 1 as std::os::raw::c_int
            } else if strcmp(*argv.offset(i as isize),
                             b"-noout\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 ||
                          strcmp(*argv.offset(i as isize),
                                 b"--noout\x00" as *const u8 as
                                     *const std::os::raw::c_char) == 0 {
                xmllint_noout += 1
            } else if strcmp(*argv.offset(i as isize),
                             b"-o\x00" as *const u8 as *const std::os::raw::c_char) ==
                          0 ||
                          strcmp(*argv.offset(i as isize),
                                 b"-output\x00" as *const u8 as
                                     *const std::os::raw::c_char) == 0 ||
                          strcmp(*argv.offset(i as isize),
                                 b"--output\x00" as *const u8 as
                                     *const std::os::raw::c_char) == 0 {
                i += 1;
                output = *argv.offset(i as isize)
            } else if strcmp(*argv.offset(i as isize),
                             b"-htmlout\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 ||
                          strcmp(*argv.offset(i as isize),
                                 b"--htmlout\x00" as *const u8 as
                                     *const std::os::raw::c_char) == 0 {
                htmlout += 1
            } else if strcmp(*argv.offset(i as isize),
                             b"-nowrap\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 ||
                          strcmp(*argv.offset(i as isize),
                                 b"--nowrap\x00" as *const u8 as
                                     *const std::os::raw::c_char) == 0 {
                nowrap += 1
            } else if strcmp(*argv.offset(i as isize),
                             b"-html\x00" as *const u8 as *const std::os::raw::c_char)
                          == 0 ||
                          strcmp(*argv.offset(i as isize),
                                 b"--html\x00" as *const u8 as
                                     *const std::os::raw::c_char) == 0 {
                html += 1
            } else if strcmp(*argv.offset(i as isize),
                             b"-xmlout\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 ||
                          strcmp(*argv.offset(i as isize),
                                 b"--xmlout\x00" as *const u8 as
                                     *const std::os::raw::c_char) == 0 {
                xmlout += 1
            } else if strcmp(*argv.offset(i as isize),
                             b"-nodefdtd\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 ||
                          strcmp(*argv.offset(i as isize),
                                 b"--nodefdtd\x00" as *const u8 as
                                     *const std::os::raw::c_char) == 0 {
                nodefdtd += 1;
                options |= HTML_PARSE_NODEFDTD as std::os::raw::c_int
            } else if strcmp(*argv.offset(i as isize),
                             b"-loaddtd\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 ||
                          strcmp(*argv.offset(i as isize),
                                 b"--loaddtd\x00" as *const u8 as
                                     *const std::os::raw::c_char) == 0 {
                loaddtd += 1;
                options |= XML_PARSE_DTDLOAD as std::os::raw::c_int
            } else if strcmp(*argv.offset(i as isize),
                             b"-dtdattr\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 ||
                          strcmp(*argv.offset(i as isize),
                                 b"--dtdattr\x00" as *const u8 as
                                     *const std::os::raw::c_char) == 0 {
                loaddtd += 1;
                dtdattrs += 1;
                options |= XML_PARSE_DTDATTR as std::os::raw::c_int
            } else if strcmp(*argv.offset(i as isize),
                             b"-valid\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 ||
                          strcmp(*argv.offset(i as isize),
                                 b"--valid\x00" as *const u8 as
                                     *const std::os::raw::c_char) == 0 {
                valid += 1;
                options |= XML_PARSE_DTDVALID as std::os::raw::c_int
            } else if strcmp(*argv.offset(i as isize),
                             b"-postvalid\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 ||
                          strcmp(*argv.offset(i as isize),
                                 b"--postvalid\x00" as *const u8 as
                                     *const std::os::raw::c_char) == 0 {
                postvalid += 1;
                loaddtd += 1;
                options |= XML_PARSE_DTDLOAD as std::os::raw::c_int
            } else if strcmp(*argv.offset(i as isize),
                             b"-dtdvalid\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 ||
                          strcmp(*argv.offset(i as isize),
                                 b"--dtdvalid\x00" as *const u8 as
                                     *const std::os::raw::c_char) == 0 {
                i += 1;
                dtdvalid = *argv.offset(i as isize);
                loaddtd += 1;
                options |= XML_PARSE_DTDLOAD as std::os::raw::c_int
            } else if strcmp(*argv.offset(i as isize),
                             b"-dtdvalidfpi\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 ||
                          strcmp(*argv.offset(i as isize),
                                 b"--dtdvalidfpi\x00" as *const u8 as
                                     *const std::os::raw::c_char) == 0 {
                i += 1;
                dtdvalidfpi = *argv.offset(i as isize);
                loaddtd += 1;
                options |= XML_PARSE_DTDLOAD as std::os::raw::c_int
            } else if strcmp(*argv.offset(i as isize),
                             b"-dropdtd\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 ||
                          strcmp(*argv.offset(i as isize),
                                 b"--dropdtd\x00" as *const u8 as
                                     *const std::os::raw::c_char) == 0 {
                dropdtd += 1
            } else if strcmp(*argv.offset(i as isize),
                             b"-insert\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 ||
                          strcmp(*argv.offset(i as isize),
                                 b"--insert\x00" as *const u8 as
                                     *const std::os::raw::c_char) == 0 {
                insert += 1
            } else if strcmp(*argv.offset(i as isize),
                             b"-timing\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 ||
                          strcmp(*argv.offset(i as isize),
                                 b"--timing\x00" as *const u8 as
                                     *const std::os::raw::c_char) == 0 {
                timing += 1
            } else if strcmp(*argv.offset(i as isize),
                             b"-auto\x00" as *const u8 as *const std::os::raw::c_char)
                          == 0 ||
                          strcmp(*argv.offset(i as isize),
                                 b"--auto\x00" as *const u8 as
                                     *const std::os::raw::c_char) == 0 {
                generate += 1
            } else if strcmp(*argv.offset(i as isize),
                             b"-repeat\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 ||
                          strcmp(*argv.offset(i as isize),
                                 b"--repeat\x00" as *const u8 as
                                     *const std::os::raw::c_char) == 0 {
                if repeat != 0 {
                    repeat *= 10 as std::os::raw::c_int
                } else { repeat = 100 as std::os::raw::c_int }
            } else if strcmp(*argv.offset(i as isize),
                             b"-push\x00" as *const u8 as *const std::os::raw::c_char)
                          == 0 ||
                          strcmp(*argv.offset(i as isize),
                                 b"--push\x00" as *const u8 as
                                     *const std::os::raw::c_char) == 0 {
                push += 1
            } else if strcmp(*argv.offset(i as isize),
                             b"-pushsmall\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 ||
                          strcmp(*argv.offset(i as isize),
                                 b"--pushsmall\x00" as *const u8 as
                                     *const std::os::raw::c_char) == 0 {
                push += 1;
                pushsize = 10 as std::os::raw::c_int
            } else if strcmp(*argv.offset(i as isize),
                             b"-memory\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 ||
                          strcmp(*argv.offset(i as isize),
                                 b"--memory\x00" as *const u8 as
                                     *const std::os::raw::c_char) == 0 {
                memory += 1
            } else if strcmp(*argv.offset(i as isize),
                             b"-testIO\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 ||
                          strcmp(*argv.offset(i as isize),
                                 b"--testIO\x00" as *const u8 as
                                     *const std::os::raw::c_char) == 0 {
                testIO += 1
            } else if strcmp(*argv.offset(i as isize),
                             b"-xinclude\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 ||
                          strcmp(*argv.offset(i as isize),
                                 b"--xinclude\x00" as *const u8 as
                                     *const std::os::raw::c_char) == 0 {
                xinclude += 1;
                options |= XML_PARSE_XINCLUDE as std::os::raw::c_int
            } else if strcmp(*argv.offset(i as isize),
                             b"-noxincludenode\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 ||
                          strcmp(*argv.offset(i as isize),
                                 b"--noxincludenode\x00" as *const u8 as
                                     *const std::os::raw::c_char) == 0 {
                xinclude += 1;
                options |= XML_PARSE_XINCLUDE as std::os::raw::c_int;
                options |= XML_PARSE_NOXINCNODE as std::os::raw::c_int
            } else if strcmp(*argv.offset(i as isize),
                             b"-nofixup-base-uris\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 ||
                          strcmp(*argv.offset(i as isize),
                                 b"--nofixup-base-uris\x00" as *const u8 as
                                     *const std::os::raw::c_char) == 0 {
                xinclude += 1;
                options |= XML_PARSE_XINCLUDE as std::os::raw::c_int;
                options |= XML_PARSE_NOBASEFIX as std::os::raw::c_int
            } else if strcmp(*argv.offset(i as isize),
                             b"-compress\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 ||
                          strcmp(*argv.offset(i as isize),
                                 b"--compress\x00" as *const u8 as
                                     *const std::os::raw::c_char) == 0 {
                compress += 1;
                xmlSetCompressMode(9 as std::os::raw::c_int);
            } else if strcmp(*argv.offset(i as isize),
                             b"-nowarning\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 ||
                          strcmp(*argv.offset(i as isize),
                                 b"--nowarning\x00" as *const u8 as
                                     *const std::os::raw::c_char) == 0 {
                *__xmlGetWarningsDefaultValue() = 0 as std::os::raw::c_int;
                xmlPedanticParserDefault(0 as std::os::raw::c_int);
                options |= XML_PARSE_NOWARNING as std::os::raw::c_int
            } else if strcmp(*argv.offset(i as isize),
                             b"-pedantic\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 ||
                          strcmp(*argv.offset(i as isize),
                                 b"--pedantic\x00" as *const u8 as
                                     *const std::os::raw::c_char) == 0 {
                *__xmlGetWarningsDefaultValue() = 1 as std::os::raw::c_int;
                xmlPedanticParserDefault(1 as std::os::raw::c_int);
                options |= XML_PARSE_PEDANTIC as std::os::raw::c_int
            } else if strcmp(*argv.offset(i as isize),
                             b"-debugent\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 ||
                          strcmp(*argv.offset(i as isize),
                                 b"--debugent\x00" as *const u8 as
                                     *const std::os::raw::c_char) == 0 {
                debugent += 1;
                *__xmlParserDebugEntities() = 1 as std::os::raw::c_int
            } else if strcmp(*argv.offset(i as isize),
                             b"-c14n\x00" as *const u8 as *const std::os::raw::c_char)
                          == 0 ||
                          strcmp(*argv.offset(i as isize),
                                 b"--c14n\x00" as *const u8 as
                                     *const std::os::raw::c_char) == 0 {
                canonical += 1;
                options |=
                    XML_PARSE_NOENT as std::os::raw::c_int |
                        XML_PARSE_DTDATTR as std::os::raw::c_int |
                        XML_PARSE_DTDLOAD as std::os::raw::c_int
            } else if strcmp(*argv.offset(i as isize),
                             b"-c14n11\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 ||
                          strcmp(*argv.offset(i as isize),
                                 b"--c14n11\x00" as *const u8 as
                                     *const std::os::raw::c_char) == 0 {
                canonical_11 += 1;
                options |=
                    XML_PARSE_NOENT as std::os::raw::c_int |
                        XML_PARSE_DTDATTR as std::os::raw::c_int |
                        XML_PARSE_DTDLOAD as std::os::raw::c_int
            } else if strcmp(*argv.offset(i as isize),
                             b"-exc-c14n\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 ||
                          strcmp(*argv.offset(i as isize),
                                 b"--exc-c14n\x00" as *const u8 as
                                     *const std::os::raw::c_char) == 0 {
                exc_canonical += 1;
                options |=
                    XML_PARSE_NOENT as std::os::raw::c_int |
                        XML_PARSE_DTDATTR as std::os::raw::c_int |
                        XML_PARSE_DTDLOAD as std::os::raw::c_int
            } else if strcmp(*argv.offset(i as isize),
                             b"-catalogs\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 ||
                          strcmp(*argv.offset(i as isize),
                                 b"--catalogs\x00" as *const u8 as
                                     *const std::os::raw::c_char) == 0 {
                catalogs += 1
            } else if strcmp(*argv.offset(i as isize),
                             b"-nocatalogs\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 ||
                          strcmp(*argv.offset(i as isize),
                                 b"--nocatalogs\x00" as *const u8 as
                                     *const std::os::raw::c_char) == 0 {
                nocatalogs += 1
            } else if strcmp(*argv.offset(i as isize),
                             b"-encode\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 ||
                          strcmp(*argv.offset(i as isize),
                                 b"--encode\x00" as *const u8 as
                                     *const std::os::raw::c_char) == 0 {
                i += 1;
                encoding = *argv.offset(i as isize);
                /* LIBXML_TREE_ENABLED */
                /* LIBXML_OUTPUT_ENABLED */
                /* LIBXML_HTML_ENABLED */
                /* LIBXML_VALID_ENABLED */
                /* LIBXML_PUSH_ENABLED */
                /* LIBXML_OUTPUT_ENABLED */
                /*
	     * OK it's for testing purposes
	     */
                xmlAddEncodingAlias(b"UTF-8\x00" as *const u8 as
                                        *const std::os::raw::c_char,
                                    b"DVEnc\x00" as *const u8 as
                                        *const std::os::raw::c_char);
            } else if strcmp(*argv.offset(i as isize),
                             b"-noblanks\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 ||
                          strcmp(*argv.offset(i as isize),
                                 b"--noblanks\x00" as *const u8 as
                                     *const std::os::raw::c_char) == 0 {
                noblanks += 1;
                xmlKeepBlanksDefault(0 as std::os::raw::c_int);
                options |= XML_PARSE_NOBLANKS as std::os::raw::c_int
            } else if strcmp(*argv.offset(i as isize),
                             b"-maxmem\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 ||
                          strcmp(*argv.offset(i as isize),
                                 b"--maxmem\x00" as *const u8 as
                                     *const std::os::raw::c_char) == 0 {
                i += 1;
                if sscanf(*argv.offset(i as isize),
                          b"%d\x00" as *const u8 as *const std::os::raw::c_char,
                          &mut maxmem as *mut std::os::raw::c_int) == 1 as std::os::raw::c_int
                   {
                    xmlMemSetup(Some(myFreeFunc as
                                         unsafe extern "C" fn(_:
                                                                  *mut std::os::raw::c_void)
                                             -> ()),
                                Some(myMallocFunc as
                                         unsafe extern "C" fn(_: size_t)
                                             -> *mut std::os::raw::c_void),
                                Some(myReallocFunc as
                                         unsafe extern "C" fn(_:
                                                                  *mut std::os::raw::c_void,
                                                              _: size_t)
                                             -> *mut std::os::raw::c_void),
                                Some(myStrdupFunc as
                                         unsafe extern "C" fn(_:
                                                                  *const std::os::raw::c_char)
                                             -> *mut std::os::raw::c_char));
                } else { maxmem = 0 as std::os::raw::c_int }
            } else if strcmp(*argv.offset(i as isize),
                             b"-format\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 ||
                          strcmp(*argv.offset(i as isize),
                                 b"--format\x00" as *const u8 as
                                     *const std::os::raw::c_char) == 0 {
                noblanks += 1;
                format = 1 as std::os::raw::c_int;
                /* LIBXML_OUTPUT_ENABLED */
                xmlKeepBlanksDefault(0 as std::os::raw::c_int);
            } else if strcmp(*argv.offset(i as isize),
                             b"-pretty\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 ||
                          strcmp(*argv.offset(i as isize),
                                 b"--pretty\x00" as *const u8 as
                                     *const std::os::raw::c_char) == 0 {
                i += 1;
                if !(*argv.offset(i as isize)).is_null() {
                    format = atoi(*argv.offset(i as isize));
                    if format == 1 as std::os::raw::c_int {
                        noblanks += 1;
                        xmlKeepBlanksDefault(0 as std::os::raw::c_int);
                    }
                }
                /* LIBXML_OUTPUT_ENABLED */
            } else if strcmp(*argv.offset(i as isize),
                             b"-stream\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 ||
                          strcmp(*argv.offset(i as isize),
                                 b"--stream\x00" as *const u8 as
                                     *const std::os::raw::c_char) == 0 {
                stream += 1
            } else if strcmp(*argv.offset(i as isize),
                             b"-walker\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 ||
                          strcmp(*argv.offset(i as isize),
                                 b"--walker\x00" as *const u8 as
                                     *const std::os::raw::c_char) == 0 {
                walker += 1;
                xmllint_noout += 1
            } else if strcmp(*argv.offset(i as isize),
                             b"-sax1\x00" as *const u8 as *const std::os::raw::c_char)
                          == 0 ||
                          strcmp(*argv.offset(i as isize),
                                 b"--sax1\x00" as *const u8 as
                                     *const std::os::raw::c_char) == 0 {
                sax1 += 1;
                options |= XML_PARSE_SAX1 as std::os::raw::c_int
            } else if strcmp(*argv.offset(i as isize),
                             b"-sax\x00" as *const u8 as *const std::os::raw::c_char)
                          == 0 ||
                          strcmp(*argv.offset(i as isize),
                                 b"--sax\x00" as *const u8 as
                                     *const std::os::raw::c_char) == 0 {
                sax += 1
            } else if strcmp(*argv.offset(i as isize),
                             b"-chkregister\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 ||
                          strcmp(*argv.offset(i as isize),
                                 b"--chkregister\x00" as *const u8 as
                                     *const std::os::raw::c_char) == 0 {
                chkregister += 1
            } else if strcmp(*argv.offset(i as isize),
                             b"-relaxng\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 ||
                          strcmp(*argv.offset(i as isize),
                                 b"--relaxng\x00" as *const u8 as
                                     *const std::os::raw::c_char) == 0 {
                i += 1;
                relaxng = *argv.offset(i as isize);
                noent += 1;
                options |= XML_PARSE_NOENT as std::os::raw::c_int
            } else if strcmp(*argv.offset(i as isize),
                             b"-schema\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 ||
                          strcmp(*argv.offset(i as isize),
                                 b"--schema\x00" as *const u8 as
                                     *const std::os::raw::c_char) == 0 {
                i += 1;
                schema = *argv.offset(i as isize);
                noent += 1
            } else if strcmp(*argv.offset(i as isize),
                             b"-schematron\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 ||
                          strcmp(*argv.offset(i as isize),
                                 b"--schematron\x00" as *const u8 as
                                     *const std::os::raw::c_char) == 0 {
                i += 1;
                schematron = *argv.offset(i as isize);
                noent += 1
            } else if strcmp(*argv.offset(i as isize),
                             b"-nonet\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 ||
                          strcmp(*argv.offset(i as isize),
                                 b"--nonet\x00" as *const u8 as
                                     *const std::os::raw::c_char) == 0 {
                options |= XML_PARSE_NONET as std::os::raw::c_int;
                xmlSetExternalEntityLoader(Some(xmlNoNetExternalEntityLoader
                                                    as
                                                    unsafe extern "C" fn(_:
                                                                             *const std::os::raw::c_char,
                                                                         _:
                                                                             *const std::os::raw::c_char,
                                                                         _:
                                                                             xmlParserCtxtPtr)
                                                        ->
                                                            xmlParserInputPtr));
            } else if strcmp(*argv.offset(i as isize),
                             b"-nocompact\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 ||
                          strcmp(*argv.offset(i as isize),
                                 b"--nocompact\x00" as *const u8 as
                                     *const std::os::raw::c_char) == 0 {
                options &= !(XML_PARSE_COMPACT as std::os::raw::c_int)
            } else if strcmp(*argv.offset(i as isize),
                             b"-load-trace\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 ||
                          strcmp(*argv.offset(i as isize),
                                 b"--load-trace\x00" as *const u8 as
                                     *const std::os::raw::c_char) == 0 {
                load_trace += 1
            } else if strcmp(*argv.offset(i as isize),
                             b"-path\x00" as *const u8 as *const std::os::raw::c_char)
                          == 0 ||
                          strcmp(*argv.offset(i as isize),
                                 b"--path\x00" as *const u8 as
                                     *const std::os::raw::c_char) == 0 {
                i += 1;
                parsePath(*argv.offset(i as isize) as *mut xmlChar);
            } else if strcmp(*argv.offset(i as isize),
                             b"-pattern\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 ||
                          strcmp(*argv.offset(i as isize),
                                 b"--pattern\x00" as *const u8 as
                                     *const std::os::raw::c_char) == 0 {
                i += 1;
                pattern = *argv.offset(i as isize)
            } else if strcmp(*argv.offset(i as isize),
                             b"-xpath\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 ||
                          strcmp(*argv.offset(i as isize),
                                 b"--xpath\x00" as *const u8 as
                                     *const std::os::raw::c_char) == 0 {
                i += 1;
                xmllint_noout += 1;
                xpathquery = *argv.offset(i as isize)
            } else if strcmp(*argv.offset(i as isize),
                             b"-oldxml10\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 ||
                          strcmp(*argv.offset(i as isize),
                                 b"--oldxml10\x00" as *const u8 as
                                     *const std::os::raw::c_char) == 0 {
                oldxml10 += 1;
                options |= XML_PARSE_OLD10 as std::os::raw::c_int
            } else {
                fprintf(stderr,
                        b"Unknown option %s\n\x00" as *const u8 as
                            *const std::os::raw::c_char, *argv.offset(i as isize));
                usage(stderr, *argv.offset(0 as std::os::raw::c_int as isize));
                return 1 as std::os::raw::c_int
            }
        }
        i += 1
    }
    if nocatalogs == 0 as std::os::raw::c_int {
        if catalogs != 0 {
            let mut catal: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
            catal =
                getenv(b"SGML_CATALOG_FILES\x00" as *const u8 as
                           *const std::os::raw::c_char);
            if !catal.is_null() {
                xmlLoadCatalogs(catal);
            } else {
                fprintf(stderr,
                        b"Variable $SGML_CATALOG_FILES not set\n\x00" as
                            *const u8 as *const std::os::raw::c_char);
            }
        }
    }
    if sax1 != 0 {
        xmlSAXDefaultVersion(1 as std::os::raw::c_int);
    } else { xmlSAXDefaultVersion(2 as std::os::raw::c_int); }
    /* LIBXML_READER_ENABLED */
    /* LIBXML_SAX1_ENABLED */
    /* LIBXML_SAX1_ENABLED */
    if chkregister != 0 {
        xmlRegisterNodeDefault(Some(registerNode as
                                        unsafe extern "C" fn(_: xmlNodePtr)
                                            -> ()));
        xmlDeregisterNodeDefault(Some(deregisterNode as
                                          unsafe extern "C" fn(_: xmlNodePtr)
                                              -> ()));
    }
    indent =
        getenv(b"XMLLINT_INDENT\x00" as *const u8 as *const std::os::raw::c_char);
    if !indent.is_null() {
        let ref mut fresh4 = *__xmlTreeIndentString();
        *fresh4 = indent
    }
    defaultEntityLoader = xmlGetExternalEntityLoader();
    xmlSetExternalEntityLoader(Some(xmllintExternalEntityLoader as
                                        unsafe extern "C" fn(_:
                                                                 *const std::os::raw::c_char,
                                                             _:
                                                                 *const std::os::raw::c_char,
                                                             _:
                                                                 xmlParserCtxtPtr)
                                            -> xmlParserInputPtr));
    xmlLineNumbersDefault(1 as std::os::raw::c_int);
    if loaddtd != 0 as std::os::raw::c_int {
        *__xmlLoadExtDtdDefaultValue() |= 2 as std::os::raw::c_int
    }
    if dtdattrs != 0 { *__xmlLoadExtDtdDefaultValue() |= 4 as std::os::raw::c_int }
    if noent != 0 as std::os::raw::c_int {
        xmlSubstituteEntitiesDefault(1 as std::os::raw::c_int);
    }
    if valid != 0 as std::os::raw::c_int {
        *__xmlDoValidityCheckingDefaultValue() = 1 as std::os::raw::c_int
    }
    /* LIBXML_VALID_ENABLED */
    if htmlout != 0 && nowrap == 0 {
        (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                   b"<!DOCTYPE HTML PUBLIC \"-//W3C//DTD HTML 4.0 Transitional//EN\"\n\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const std::os::raw::c_char);
        (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                   b"\t\"http://www.w3.org/TR/REC-html40/loose.dtd\">\n\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const std::os::raw::c_char);
        (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                   b"<html><head><title>%s output</title></head>\n\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const std::os::raw::c_char,
                                                                   *argv.offset(0
                                                                                    as
                                                                                    std::os::raw::c_int
                                                                                    as
                                                                                    isize));
        (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                   b"<body bgcolor=\"#ffffff\"><h1 align=\"center\">%s output</h1>\n\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const std::os::raw::c_char,
                                                                   *argv.offset(0
                                                                                    as
                                                                                    std::os::raw::c_int
                                                                                    as
                                                                                    isize));
    }
    if !schematron.is_null() && sax == 0 as std::os::raw::c_int &&
           stream == 0 as std::os::raw::c_int {
        /* LIBXML_READER_ENABLED */
        let mut ctxt: xmlSchematronParserCtxtPtr =
            0 as *mut xmlSchematronParserCtxt;
        /* forces loading the DTDs */
        *__xmlLoadExtDtdDefaultValue() |= 1 as std::os::raw::c_int;
        options |= XML_PARSE_DTDLOAD as std::os::raw::c_int;
        if timing != 0 { xmllint_startTimer(); }
        ctxt = xmlSchematronNewParserCtxt(schematron);
        wxschematron = xmlSchematronParse(ctxt);
        if wxschematron.is_null() {
            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                       b"Schematron schema %s failed to compile\n\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const std::os::raw::c_char,
                                                                       schematron);
            xmllint_progresult = XMLLINT_ERR_SCHEMACOMP;
            schematron = 0 as *mut std::os::raw::c_char
        }
        xmlSchematronFreeParserCtxt(ctxt);
        if timing != 0 {
            xmllint_endTimer(b"Compiling the schemas\x00" as *const u8 as
                                 *const std::os::raw::c_char);
        }
    }
    if !relaxng.is_null() && sax == 0 as std::os::raw::c_int &&
           stream == 0 as std::os::raw::c_int {
        /* LIBXML_READER_ENABLED */
        let mut ctxt_0: xmlRelaxNGParserCtxtPtr =
            0 as *mut xmlRelaxNGParserCtxt;
        /* forces loading the DTDs */
        *__xmlLoadExtDtdDefaultValue() |= 1 as std::os::raw::c_int;
        options |= XML_PARSE_DTDLOAD as std::os::raw::c_int;
        if timing != 0 { xmllint_startTimer(); }
        ctxt_0 = xmlRelaxNGNewParserCtxt(relaxng);
        xmlRelaxNGSetParserErrors(ctxt_0,
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
        relaxngschemas = xmlRelaxNGParse(ctxt_0);
        if relaxngschemas.is_null() {
            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                       b"Relax-NG schema %s failed to compile\n\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const std::os::raw::c_char,
                                                                       relaxng);
            xmllint_progresult = XMLLINT_ERR_SCHEMACOMP;
            relaxng = 0 as *mut std::os::raw::c_char
        }
        xmlRelaxNGFreeParserCtxt(ctxt_0);
        if timing != 0 {
            xmllint_endTimer(b"Compiling the schemas\x00" as *const u8 as
                                 *const std::os::raw::c_char);
        }
    } else if !schema.is_null() && stream == 0 as std::os::raw::c_int {
        let mut ctxt_1: xmlSchemaParserCtxtPtr =
            0 as *mut xmlSchemaParserCtxt;
        if timing != 0 { xmllint_startTimer(); }
        ctxt_1 = xmlSchemaNewParserCtxt(schema);
        xmlSchemaSetParserErrors(ctxt_1,
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
        wxschemas = xmlSchemaParse(ctxt_1);
        if wxschemas.is_null() {
            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                       b"WXS schema %s failed to compile\n\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const std::os::raw::c_char,
                                                                       schema);
            xmllint_progresult = XMLLINT_ERR_SCHEMACOMP;
            schema = 0 as *mut std::os::raw::c_char
        }
        xmlSchemaFreeParserCtxt(ctxt_1);
        if timing != 0 {
            xmllint_endTimer(b"Compiling the schemas\x00" as *const u8 as
                                 *const std::os::raw::c_char);
        }
    }
    /* LIBXML_SCHEMAS_ENABLED */
    if !pattern.is_null() && walker == 0 as std::os::raw::c_int {
        patternc =
            xmlPatterncompile(pattern as *const xmlChar, 0 as *mut xmlDict,
                              0 as std::os::raw::c_int, 0 as *mut *const xmlChar);
        if patternc.is_null() {
            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                       b"Pattern %s failed to compile\n\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const std::os::raw::c_char,
                                                                       pattern);
            xmllint_progresult = XMLLINT_ERR_SCHEMAPAT;
            pattern = 0 as *const std::os::raw::c_char
        }
    }
    /* LIBXML_PATTERN_ENABLED */
    i = 1 as std::os::raw::c_int;
    while i < argc {
        if strcmp(*argv.offset(i as isize),
                  b"-encode\x00" as *const u8 as *const std::os::raw::c_char) == 0 ||
               strcmp(*argv.offset(i as isize),
                      b"--encode\x00" as *const u8 as *const std::os::raw::c_char) ==
                   0 {
            i += 1
        } else if strcmp(*argv.offset(i as isize),
                         b"-o\x00" as *const u8 as *const std::os::raw::c_char) == 0
                      ||
                      strcmp(*argv.offset(i as isize),
                             b"-output\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 ||
                      strcmp(*argv.offset(i as isize),
                             b"--output\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 {
            i += 1
        } else if strcmp(*argv.offset(i as isize),
                         b"-dtdvalid\x00" as *const u8 as *const std::os::raw::c_char)
                      == 0 ||
                      strcmp(*argv.offset(i as isize),
                             b"--dtdvalid\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 {
            i += 1
        } else if strcmp(*argv.offset(i as isize),
                         b"-path\x00" as *const u8 as *const std::os::raw::c_char) ==
                      0 ||
                      strcmp(*argv.offset(i as isize),
                             b"--path\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 {
            i += 1
        } else if strcmp(*argv.offset(i as isize),
                         b"-dtdvalidfpi\x00" as *const u8 as
                             *const std::os::raw::c_char) == 0 ||
                      strcmp(*argv.offset(i as isize),
                             b"--dtdvalidfpi\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 {
            i += 1
        } else if strcmp(*argv.offset(i as isize),
                         b"-relaxng\x00" as *const u8 as *const std::os::raw::c_char)
                      == 0 ||
                      strcmp(*argv.offset(i as isize),
                             b"--relaxng\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 {
            i += 1
        } else if strcmp(*argv.offset(i as isize),
                         b"-maxmem\x00" as *const u8 as *const std::os::raw::c_char)
                      == 0 ||
                      strcmp(*argv.offset(i as isize),
                             b"--maxmem\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 {
            i += 1
        } else if strcmp(*argv.offset(i as isize),
                         b"-pretty\x00" as *const u8 as *const std::os::raw::c_char)
                      == 0 ||
                      strcmp(*argv.offset(i as isize),
                             b"--pretty\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 {
            i += 1
        } else if strcmp(*argv.offset(i as isize),
                         b"-schema\x00" as *const u8 as *const std::os::raw::c_char)
                      == 0 ||
                      strcmp(*argv.offset(i as isize),
                             b"--schema\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 {
            i += 1
        } else if strcmp(*argv.offset(i as isize),
                         b"-schematron\x00" as *const u8 as
                             *const std::os::raw::c_char) == 0 ||
                      strcmp(*argv.offset(i as isize),
                             b"--schematron\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 {
            i += 1
        } else if strcmp(*argv.offset(i as isize),
                         b"-pattern\x00" as *const u8 as *const std::os::raw::c_char)
                      == 0 ||
                      strcmp(*argv.offset(i as isize),
                             b"--pattern\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 {
            i += 1
        } else if strcmp(*argv.offset(i as isize),
                         b"-xpath\x00" as *const u8 as *const std::os::raw::c_char) ==
                      0 ||
                      strcmp(*argv.offset(i as isize),
                             b"--xpath\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 {
            i += 1
        } else {
            if timing != 0 && repeat != 0 { xmllint_startTimer(); }
            /* LIBXML_VALID_ENABLED */
            /* Remember file names.  "-" means stdin.  <sven@zen.org> */
            if *(*argv.offset(i as isize)).offset(0 as std::os::raw::c_int as isize)
                   as std::os::raw::c_int != '-' as i32 ||
                   strcmp(*argv.offset(i as isize),
                          b"-\x00" as *const u8 as *const std::os::raw::c_char) ==
                       0 as std::os::raw::c_int {
                if repeat != 0 {
                    let mut ctxt_2: xmlParserCtxtPtr = 0 as xmlParserCtxtPtr;
                    acount = 0 as std::os::raw::c_int;
                    while acount < repeat {
                        if stream != 0 as std::os::raw::c_int {
                            streamFile(*argv.offset(i as isize));
                        } else if sax != 0 {
                            testSAX(*argv.offset(i as isize));
                        } else {
                            if ctxt_2.is_null() {
                                ctxt_2 = xmlNewParserCtxt()
                            }
                            parseAndPrintFile(*argv.offset(i as isize),
                                              ctxt_2);
                        }
                        acount += 1
                        /* LIBXML_READER_ENABLED */
                        /* LIBXML_READER_ENABLED */
                    }
                    if !ctxt_2.is_null() { xmlFreeParserCtxt(ctxt_2); }
                } else {
                    nbregister = 0 as std::os::raw::c_int;
                    if stream != 0 as std::os::raw::c_int {
                        streamFile(*argv.offset(i as isize));
                    } else if sax != 0 {
                        testSAX(*argv.offset(i as isize));
                    } else {
                        parseAndPrintFile(*argv.offset(i as isize),
                                          0 as xmlParserCtxtPtr);
                    }
                    if chkregister != 0 && nbregister != 0 as std::os::raw::c_int {
                        fprintf(stderr,
                                b"Registration count off: %d\n\x00" as
                                    *const u8 as *const std::os::raw::c_char,
                                nbregister);
                        xmllint_progresult = XMLLINT_ERR_RDREGIS
                    }
                }
                files += 1;
                if timing != 0 && repeat != 0 {
                    xmllint_endTimer(b"%d iterations\x00" as *const u8 as
                                         *const std::os::raw::c_char, repeat);
                }
            }
        }
        i += 1
    }
    if generate != 0 {
        parseAndPrintFile(0 as *mut std::os::raw::c_char, 0 as xmlParserCtxtPtr);
    }
    if htmlout != 0 && nowrap == 0 {
        (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                   b"</body></html>\n\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const std::os::raw::c_char);
    }
    if files == 0 as std::os::raw::c_int && generate == 0 &&
           version == 0 as std::os::raw::c_int {
        usage(stderr, *argv.offset(0 as std::os::raw::c_int as isize));
    }
    if !wxschematron.is_null() { xmlSchematronFree(wxschematron); }
    if !relaxngschemas.is_null() { xmlRelaxNGFree(relaxngschemas); }
    if !wxschemas.is_null() { xmlSchemaFree(wxschemas); }
    xmlRelaxNGCleanupTypes();
    if !patternc.is_null() { xmlFreePattern(patternc); }
    xmlCleanupParser();
    xmlMemoryDump();
    return xmllint_progresult as std::os::raw::c_int;
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
/* LIBXML_READER_ENABLED */
