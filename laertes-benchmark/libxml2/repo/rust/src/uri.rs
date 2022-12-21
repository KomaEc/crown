
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
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
    fn xmlStrndup(cur: *const xmlChar, len: std::os::raw::c_int) -> *mut xmlChar;
    #[no_mangle]
    fn xmlStrchr(str: *const xmlChar, val: xmlChar) -> *const xmlChar;
    #[no_mangle]
    fn xmlStrstr(str: *const xmlChar, val: *const xmlChar) -> *const xmlChar;
    #[no_mangle]
    fn xmlStrcmp(str1: *const xmlChar, str2: *const xmlChar) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlStrEqual(str1: *const xmlChar, str2: *const xmlChar) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlStrlen(str: *const xmlChar) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlStrcat(cur: *mut xmlChar, add: *const xmlChar) -> *mut xmlChar;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    fn snprintf(_: *mut std::os::raw::c_char, _: std::os::raw::c_ulong,
                _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    fn memcpy(_: *mut std::os::raw::c_void, _: *const std::os::raw::c_void, _: std::os::raw::c_ulong)
     -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn memset(_: *mut std::os::raw::c_void, _: std::os::raw::c_int, _: std::os::raw::c_ulong)
     -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn strlen(_: *const std::os::raw::c_char) -> std::os::raw::c_ulong;
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
    static mut xmlMalloc: xmlMallocFunc;
    #[no_mangle]
    static mut xmlMallocAtomic: xmlMallocFunc;
    #[no_mangle]
    static mut xmlRealloc: xmlReallocFunc;
    #[no_mangle]
    static mut xmlFree: xmlFreeFunc;
    #[no_mangle]
    static mut xmlMemStrdup: xmlStrdupFunc;
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
pub type xmlReallocFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: size_t)
               -> *mut std::os::raw::c_void>;
pub type xmlStrdupFunc
    =
    Option<unsafe extern "C" fn(_: *const std::os::raw::c_char) -> *mut std::os::raw::c_char>;
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
pub type C2RustUnnamed_0 = std::os::raw::c_uint;
pub const XML_BUF_OVERFLOW: C2RustUnnamed_0 = 7000;
pub const XML_I18N_NO_OUTPUT: C2RustUnnamed_0 = 6004;
pub const XML_I18N_CONV_FAILED: C2RustUnnamed_0 = 6003;
pub const XML_I18N_EXCESS_HANDLER: C2RustUnnamed_0 = 6002;
pub const XML_I18N_NO_HANDLER: C2RustUnnamed_0 = 6001;
pub const XML_I18N_NO_NAME: C2RustUnnamed_0 = 6000;
pub const XML_CHECK_NAME_NOT_NULL: C2RustUnnamed_0 = 5037;
pub const XML_CHECK_WRONG_NAME: C2RustUnnamed_0 = 5036;
pub const XML_CHECK_OUTSIDE_DICT: C2RustUnnamed_0 = 5035;
pub const XML_CHECK_NOT_NCNAME: C2RustUnnamed_0 = 5034;
pub const XML_CHECK_NO_DICT: C2RustUnnamed_0 = 5033;
pub const XML_CHECK_NOT_UTF8: C2RustUnnamed_0 = 5032;
pub const XML_CHECK_NS_ANCESTOR: C2RustUnnamed_0 = 5031;
pub const XML_CHECK_NS_SCOPE: C2RustUnnamed_0 = 5030;
pub const XML_CHECK_WRONG_PARENT: C2RustUnnamed_0 = 5029;
pub const XML_CHECK_NO_HREF: C2RustUnnamed_0 = 5028;
pub const XML_CHECK_NOT_NS_DECL: C2RustUnnamed_0 = 5027;
pub const XML_CHECK_NOT_ENTITY_DECL: C2RustUnnamed_0 = 5026;
pub const XML_CHECK_NOT_ELEM_DECL: C2RustUnnamed_0 = 5025;
pub const XML_CHECK_NOT_ATTR_DECL: C2RustUnnamed_0 = 5024;
pub const XML_CHECK_NOT_ATTR: C2RustUnnamed_0 = 5023;
pub const XML_CHECK_NOT_DTD: C2RustUnnamed_0 = 5022;
pub const XML_CHECK_WRONG_NEXT: C2RustUnnamed_0 = 5021;
pub const XML_CHECK_NO_NEXT: C2RustUnnamed_0 = 5020;
pub const XML_CHECK_WRONG_PREV: C2RustUnnamed_0 = 5019;
pub const XML_CHECK_NO_PREV: C2RustUnnamed_0 = 5018;
pub const XML_CHECK_WRONG_DOC: C2RustUnnamed_0 = 5017;
pub const XML_CHECK_NO_ELEM: C2RustUnnamed_0 = 5016;
pub const XML_CHECK_NO_NAME: C2RustUnnamed_0 = 5015;
pub const XML_CHECK_NO_DOC: C2RustUnnamed_0 = 5014;
pub const XML_CHECK_NO_PARENT: C2RustUnnamed_0 = 5013;
pub const XML_CHECK_ENTITY_TYPE: C2RustUnnamed_0 = 5012;
pub const XML_CHECK_UNKNOWN_NODE: C2RustUnnamed_0 = 5011;
pub const XML_CHECK_FOUND_NOTATION: C2RustUnnamed_0 = 5010;
pub const XML_CHECK_FOUND_FRAGMENT: C2RustUnnamed_0 = 5009;
pub const XML_CHECK_FOUND_DOCTYPE: C2RustUnnamed_0 = 5008;
pub const XML_CHECK_FOUND_COMMENT: C2RustUnnamed_0 = 5007;
pub const XML_CHECK_FOUND_PI: C2RustUnnamed_0 = 5006;
pub const XML_CHECK_FOUND_ENTITY: C2RustUnnamed_0 = 5005;
pub const XML_CHECK_FOUND_ENTITYREF: C2RustUnnamed_0 = 5004;
pub const XML_CHECK_FOUND_CDATA: C2RustUnnamed_0 = 5003;
pub const XML_CHECK_FOUND_TEXT: C2RustUnnamed_0 = 5002;
pub const XML_CHECK_FOUND_ATTRIBUTE: C2RustUnnamed_0 = 5001;
pub const XML_CHECK_FOUND_ELEMENT: C2RustUnnamed_0 = 5000;
pub const XML_MODULE_CLOSE: C2RustUnnamed_0 = 4901;
pub const XML_MODULE_OPEN: C2RustUnnamed_0 = 4900;
pub const XML_SCHEMATRONV_REPORT: C2RustUnnamed_0 = 4001;
pub const XML_SCHEMATRONV_ASSERT: C2RustUnnamed_0 = 4000;
pub const XML_SCHEMAP_COS_ALL_LIMITED: C2RustUnnamed_0 = 3091;
pub const XML_SCHEMAP_A_PROPS_CORRECT_3: C2RustUnnamed_0 = 3090;
pub const XML_SCHEMAP_AU_PROPS_CORRECT: C2RustUnnamed_0 = 3089;
pub const XML_SCHEMAP_COS_CT_EXTENDS_1_2: C2RustUnnamed_0 = 3088;
pub const XML_SCHEMAP_AG_PROPS_CORRECT: C2RustUnnamed_0 = 3087;
pub const XML_SCHEMAP_WARN_ATTR_POINTLESS_PROH: C2RustUnnamed_0 = 3086;
pub const XML_SCHEMAP_WARN_ATTR_REDECL_PROH: C2RustUnnamed_0 = 3085;
pub const XML_SCHEMAP_WARN_UNLOCATED_SCHEMA: C2RustUnnamed_0 = 3084;
pub const XML_SCHEMAP_WARN_SKIP_SCHEMA: C2RustUnnamed_0 = 3083;
pub const XML_SCHEMAP_SRC_IMPORT: C2RustUnnamed_0 = 3082;
pub const XML_SCHEMAP_SRC_REDEFINE: C2RustUnnamed_0 = 3081;
pub const XML_SCHEMAP_C_PROPS_CORRECT: C2RustUnnamed_0 = 3080;
pub const XML_SCHEMAP_A_PROPS_CORRECT_2: C2RustUnnamed_0 = 3079;
pub const XML_SCHEMAP_AU_PROPS_CORRECT_2: C2RustUnnamed_0 = 3078;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_2_1_3: C2RustUnnamed_0 = 3077;
pub const XML_SCHEMAP_SRC_CT_1: C2RustUnnamed_0 = 3076;
pub const XML_SCHEMAP_MG_PROPS_CORRECT_2: C2RustUnnamed_0 = 3075;
pub const XML_SCHEMAP_MG_PROPS_CORRECT_1: C2RustUnnamed_0 = 3074;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_GROUP_3: C2RustUnnamed_0 = 3073;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_GROUP_2: C2RustUnnamed_0 = 3072;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_GROUP_1: C2RustUnnamed_0 = 3071;
pub const XML_SCHEMAP_NOT_DETERMINISTIC: C2RustUnnamed_0 = 3070;
pub const XML_SCHEMAP_INTERNAL: C2RustUnnamed_0 = 3069;
pub const XML_SCHEMAP_SRC_IMPORT_2_2: C2RustUnnamed_0 = 3068;
pub const XML_SCHEMAP_SRC_IMPORT_2_1: C2RustUnnamed_0 = 3067;
pub const XML_SCHEMAP_SRC_IMPORT_2: C2RustUnnamed_0 = 3066;
pub const XML_SCHEMAP_SRC_IMPORT_1_2: C2RustUnnamed_0 = 3065;
pub const XML_SCHEMAP_SRC_IMPORT_1_1: C2RustUnnamed_0 = 3064;
pub const XML_SCHEMAP_COS_CT_EXTENDS_1_1: C2RustUnnamed_0 = 3063;
pub const XML_SCHEMAP_CVC_SIMPLE_TYPE: C2RustUnnamed_0 = 3062;
pub const XML_SCHEMAP_COS_VALID_DEFAULT_2_2_2: C2RustUnnamed_0 = 3061;
pub const XML_SCHEMAP_COS_VALID_DEFAULT_2_2_1: C2RustUnnamed_0 = 3060;
pub const XML_SCHEMAP_COS_VALID_DEFAULT_2_1: C2RustUnnamed_0 = 3059;
pub const XML_SCHEMAP_COS_VALID_DEFAULT_1: C2RustUnnamed_0 = 3058;
pub const XML_SCHEMAP_NO_XSI: C2RustUnnamed_0 = 3057;
pub const XML_SCHEMAP_NO_XMLNS: C2RustUnnamed_0 = 3056;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_4: C2RustUnnamed_0 = 3055;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_3_2: C2RustUnnamed_0 = 3054;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_3_1: C2RustUnnamed_0 = 3053;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_2: C2RustUnnamed_0 = 3052;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_1: C2RustUnnamed_0 = 3051;
pub const XML_SCHEMAP_SRC_INCLUDE: C2RustUnnamed_0 = 3050;
pub const XML_SCHEMAP_E_PROPS_CORRECT_6: C2RustUnnamed_0 = 3049;
pub const XML_SCHEMAP_E_PROPS_CORRECT_5: C2RustUnnamed_0 = 3048;
pub const XML_SCHEMAP_E_PROPS_CORRECT_4: C2RustUnnamed_0 = 3047;
pub const XML_SCHEMAP_E_PROPS_CORRECT_3: C2RustUnnamed_0 = 3046;
pub const XML_SCHEMAP_E_PROPS_CORRECT_2: C2RustUnnamed_0 = 3045;
pub const XML_SCHEMAP_P_PROPS_CORRECT_2_2: C2RustUnnamed_0 = 3044;
pub const XML_SCHEMAP_P_PROPS_CORRECT_2_1: C2RustUnnamed_0 = 3043;
pub const XML_SCHEMAP_P_PROPS_CORRECT_1: C2RustUnnamed_0 = 3042;
pub const XML_SCHEMAP_SRC_ELEMENT_3: C2RustUnnamed_0 = 3041;
pub const XML_SCHEMAP_SRC_ELEMENT_2_2: C2RustUnnamed_0 = 3040;
pub const XML_SCHEMAP_SRC_ELEMENT_2_1: C2RustUnnamed_0 = 3039;
pub const XML_SCHEMAP_SRC_ELEMENT_1: C2RustUnnamed_0 = 3038;
pub const XML_SCHEMAP_S4S_ATTR_INVALID_VALUE: C2RustUnnamed_0 = 3037;
pub const XML_SCHEMAP_S4S_ATTR_MISSING: C2RustUnnamed_0 = 3036;
pub const XML_SCHEMAP_S4S_ATTR_NOT_ALLOWED: C2RustUnnamed_0 = 3035;
pub const XML_SCHEMAP_S4S_ELEM_MISSING: C2RustUnnamed_0 = 3034;
pub const XML_SCHEMAP_S4S_ELEM_NOT_ALLOWED: C2RustUnnamed_0 = 3033;
pub const XML_SCHEMAP_COS_ST_DERIVED_OK_2_2: C2RustUnnamed_0 = 3032;
pub const XML_SCHEMAP_COS_ST_DERIVED_OK_2_1: C2RustUnnamed_0 = 3031;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_5: C2RustUnnamed_0 = 3030;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_4: C2RustUnnamed_0 = 3029;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_3: C2RustUnnamed_0 = 3028;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_1: C2RustUnnamed_0 = 3027;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_2: C2RustUnnamed_0 = 3026;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_1_2: C2RustUnnamed_0 = 3025;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_1: C2RustUnnamed_0 = 3024;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_1: C2RustUnnamed_0 = 3023;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_5: C2RustUnnamed_0 = 3022;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_4: C2RustUnnamed_0 = 3021;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_3: C2RustUnnamed_0 = 3020;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_2: C2RustUnnamed_0 = 3019;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_1: C2RustUnnamed_0 = 3018;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_1_2: C2RustUnnamed_0 = 3017;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_1_1: C2RustUnnamed_0 = 3016;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_1: C2RustUnnamed_0 = 3015;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_1_3_2: C2RustUnnamed_0 = 3014;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_1_3_1: C2RustUnnamed_0 = 3013;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_1_2: C2RustUnnamed_0 = 3012;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_1_1: C2RustUnnamed_0 = 3011;
pub const XML_SCHEMAP_ST_PROPS_CORRECT_3: C2RustUnnamed_0 = 3010;
pub const XML_SCHEMAP_ST_PROPS_CORRECT_2: C2RustUnnamed_0 = 3009;
pub const XML_SCHEMAP_ST_PROPS_CORRECT_1: C2RustUnnamed_0 = 3008;
pub const XML_SCHEMAP_SRC_UNION_MEMBERTYPES_OR_SIMPLETYPES: C2RustUnnamed_0 =
    3007;
pub const XML_SCHEMAP_SRC_LIST_ITEMTYPE_OR_SIMPLETYPE: C2RustUnnamed_0 = 3006;
pub const XML_SCHEMAP_SRC_RESTRICTION_BASE_OR_SIMPLETYPE: C2RustUnnamed_0 =
    3005;
pub const XML_SCHEMAP_SRC_RESOLVE: C2RustUnnamed_0 = 3004;
pub const XML_SCHEMAP_SRC_SIMPLE_TYPE_4: C2RustUnnamed_0 = 3003;
pub const XML_SCHEMAP_SRC_SIMPLE_TYPE_3: C2RustUnnamed_0 = 3002;
pub const XML_SCHEMAP_SRC_SIMPLE_TYPE_2: C2RustUnnamed_0 = 3001;
pub const XML_SCHEMAP_SRC_SIMPLE_TYPE_1: C2RustUnnamed_0 = 3000;
pub const XML_HTTP_UNKNOWN_HOST: C2RustUnnamed_0 = 2022;
pub const XML_HTTP_USE_IP: C2RustUnnamed_0 = 2021;
pub const XML_HTTP_URL_SYNTAX: C2RustUnnamed_0 = 2020;
pub const XML_FTP_URL_SYNTAX: C2RustUnnamed_0 = 2003;
pub const XML_FTP_ACCNT: C2RustUnnamed_0 = 2002;
pub const XML_FTP_EPSV_ANSWER: C2RustUnnamed_0 = 2001;
pub const XML_FTP_PASV_ANSWER: C2RustUnnamed_0 = 2000;
pub const XML_C14N_RELATIVE_NAMESPACE: C2RustUnnamed_0 = 1955;
pub const XML_C14N_UNKNOW_NODE: C2RustUnnamed_0 = 1954;
pub const XML_C14N_INVALID_NODE: C2RustUnnamed_0 = 1953;
pub const XML_C14N_CREATE_STACK: C2RustUnnamed_0 = 1952;
pub const XML_C14N_REQUIRES_UTF8: C2RustUnnamed_0 = 1951;
pub const XML_C14N_CREATE_CTXT: C2RustUnnamed_0 = 1950;
pub const XML_XPTR_EXTRA_OBJECTS: C2RustUnnamed_0 = 1903;
pub const XML_XPTR_EVAL_FAILED: C2RustUnnamed_0 = 1902;
pub const XML_XPTR_CHILDSEQ_START: C2RustUnnamed_0 = 1901;
pub const XML_XPTR_UNKNOWN_SCHEME: C2RustUnnamed_0 = 1900;
pub const XML_SCHEMAV_MISC: C2RustUnnamed_0 = 1879;
pub const XML_SCHEMAV_CVC_WILDCARD: C2RustUnnamed_0 = 1878;
pub const XML_SCHEMAV_CVC_IDC: C2RustUnnamed_0 = 1877;
pub const XML_SCHEMAV_CVC_TYPE_2: C2RustUnnamed_0 = 1876;
pub const XML_SCHEMAV_CVC_TYPE_1: C2RustUnnamed_0 = 1875;
pub const XML_SCHEMAV_CVC_AU: C2RustUnnamed_0 = 1874;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_1: C2RustUnnamed_0 = 1873;
pub const XML_SCHEMAV_DOCUMENT_ELEMENT_MISSING: C2RustUnnamed_0 = 1872;
pub const XML_SCHEMAV_ELEMENT_CONTENT: C2RustUnnamed_0 = 1871;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_5_2: C2RustUnnamed_0 = 1870;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_5_1: C2RustUnnamed_0 = 1869;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_4: C2RustUnnamed_0 = 1868;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_3_2_2: C2RustUnnamed_0 = 1867;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_3_2_1: C2RustUnnamed_0 = 1866;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_3_1: C2RustUnnamed_0 = 1865;
pub const XML_SCHEMAV_CVC_ATTRIBUTE_4: C2RustUnnamed_0 = 1864;
pub const XML_SCHEMAV_CVC_ATTRIBUTE_3: C2RustUnnamed_0 = 1863;
pub const XML_SCHEMAV_CVC_ATTRIBUTE_2: C2RustUnnamed_0 = 1862;
pub const XML_SCHEMAV_CVC_ATTRIBUTE_1: C2RustUnnamed_0 = 1861;
pub const XML_SCHEMAV_CVC_ELT_7: C2RustUnnamed_0 = 1860;
pub const XML_SCHEMAV_CVC_ELT_6: C2RustUnnamed_0 = 1859;
pub const XML_SCHEMAV_CVC_ELT_5_2_2_2_2: C2RustUnnamed_0 = 1858;
pub const XML_SCHEMAV_CVC_ELT_5_2_2_2_1: C2RustUnnamed_0 = 1857;
pub const XML_SCHEMAV_CVC_ELT_5_2_2_1: C2RustUnnamed_0 = 1856;
pub const XML_SCHEMAV_CVC_ELT_5_2_1: C2RustUnnamed_0 = 1855;
pub const XML_SCHEMAV_CVC_ELT_5_1_2: C2RustUnnamed_0 = 1854;
pub const XML_SCHEMAV_CVC_ELT_5_1_1: C2RustUnnamed_0 = 1853;
pub const XML_SCHEMAV_CVC_ELT_4_3: C2RustUnnamed_0 = 1852;
pub const XML_SCHEMAV_CVC_ELT_4_2: C2RustUnnamed_0 = 1851;
pub const XML_SCHEMAV_CVC_ELT_4_1: C2RustUnnamed_0 = 1850;
pub const XML_SCHEMAV_CVC_ELT_3_2_2: C2RustUnnamed_0 = 1849;
pub const XML_SCHEMAV_CVC_ELT_3_2_1: C2RustUnnamed_0 = 1848;
pub const XML_SCHEMAV_CVC_ELT_3_1: C2RustUnnamed_0 = 1847;
pub const XML_SCHEMAV_CVC_ELT_2: C2RustUnnamed_0 = 1846;
pub const XML_SCHEMAV_CVC_ELT_1: C2RustUnnamed_0 = 1845;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_2_4: C2RustUnnamed_0 = 1844;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_2_3: C2RustUnnamed_0 = 1843;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_2_2: C2RustUnnamed_0 = 1842;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_2_1: C2RustUnnamed_0 = 1841;
pub const XML_SCHEMAV_CVC_ENUMERATION_VALID: C2RustUnnamed_0 = 1840;
pub const XML_SCHEMAV_CVC_PATTERN_VALID: C2RustUnnamed_0 = 1839;
pub const XML_SCHEMAV_CVC_FRACTIONDIGITS_VALID: C2RustUnnamed_0 = 1838;
pub const XML_SCHEMAV_CVC_TOTALDIGITS_VALID: C2RustUnnamed_0 = 1837;
pub const XML_SCHEMAV_CVC_MAXEXCLUSIVE_VALID: C2RustUnnamed_0 = 1836;
pub const XML_SCHEMAV_CVC_MINEXCLUSIVE_VALID: C2RustUnnamed_0 = 1835;
pub const XML_SCHEMAV_CVC_MAXINCLUSIVE_VALID: C2RustUnnamed_0 = 1834;
pub const XML_SCHEMAV_CVC_MININCLUSIVE_VALID: C2RustUnnamed_0 = 1833;
pub const XML_SCHEMAV_CVC_MAXLENGTH_VALID: C2RustUnnamed_0 = 1832;
pub const XML_SCHEMAV_CVC_MINLENGTH_VALID: C2RustUnnamed_0 = 1831;
pub const XML_SCHEMAV_CVC_LENGTH_VALID: C2RustUnnamed_0 = 1830;
pub const XML_SCHEMAV_CVC_FACET_VALID: C2RustUnnamed_0 = 1829;
pub const XML_SCHEMAV_CVC_TYPE_3_1_2: C2RustUnnamed_0 = 1828;
pub const XML_SCHEMAV_CVC_TYPE_3_1_1: C2RustUnnamed_0 = 1827;
pub const XML_SCHEMAV_CVC_DATATYPE_VALID_1_2_3: C2RustUnnamed_0 = 1826;
pub const XML_SCHEMAV_CVC_DATATYPE_VALID_1_2_2: C2RustUnnamed_0 = 1825;
pub const XML_SCHEMAV_CVC_DATATYPE_VALID_1_2_1: C2RustUnnamed_0 = 1824;
pub const XML_SCHEMAV_FACET: C2RustUnnamed_0 = 1823;
pub const XML_SCHEMAV_VALUE: C2RustUnnamed_0 = 1822;
pub const XML_SCHEMAV_ATTRINVALID: C2RustUnnamed_0 = 1821;
pub const XML_SCHEMAV_ATTRUNKNOWN: C2RustUnnamed_0 = 1820;
pub const XML_SCHEMAV_NOTSIMPLE: C2RustUnnamed_0 = 1819;
pub const XML_SCHEMAV_INTERNAL: C2RustUnnamed_0 = 1818;
pub const XML_SCHEMAV_CONSTRUCT: C2RustUnnamed_0 = 1817;
pub const XML_SCHEMAV_NOTDETERMINIST: C2RustUnnamed_0 = 1816;
pub const XML_SCHEMAV_INVALIDELEM: C2RustUnnamed_0 = 1815;
pub const XML_SCHEMAV_INVALIDATTR: C2RustUnnamed_0 = 1814;
pub const XML_SCHEMAV_EXTRACONTENT: C2RustUnnamed_0 = 1813;
pub const XML_SCHEMAV_NOTNILLABLE: C2RustUnnamed_0 = 1812;
pub const XML_SCHEMAV_HAVEDEFAULT: C2RustUnnamed_0 = 1811;
pub const XML_SCHEMAV_ELEMCONT: C2RustUnnamed_0 = 1810;
pub const XML_SCHEMAV_NOTEMPTY: C2RustUnnamed_0 = 1809;
pub const XML_SCHEMAV_ISABSTRACT: C2RustUnnamed_0 = 1808;
pub const XML_SCHEMAV_NOROLLBACK: C2RustUnnamed_0 = 1807;
pub const XML_SCHEMAV_NOTYPE: C2RustUnnamed_0 = 1806;
pub const XML_SCHEMAV_WRONGELEM: C2RustUnnamed_0 = 1805;
pub const XML_SCHEMAV_MISSING: C2RustUnnamed_0 = 1804;
pub const XML_SCHEMAV_NOTTOPLEVEL: C2RustUnnamed_0 = 1803;
pub const XML_SCHEMAV_UNDECLAREDELEM: C2RustUnnamed_0 = 1802;
pub const XML_SCHEMAV_NOROOT: C2RustUnnamed_0 = 1801;
pub const XML_SCHEMAP_COS_CT_EXTENDS_1_3: C2RustUnnamed_0 = 1800;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_4_3: C2RustUnnamed_0 = 1799;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_4_2: C2RustUnnamed_0 = 1798;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_4_1: C2RustUnnamed_0 = 1797;
pub const XML_SCHEMAP_SRC_IMPORT_3_2: C2RustUnnamed_0 = 1796;
pub const XML_SCHEMAP_SRC_IMPORT_3_1: C2RustUnnamed_0 = 1795;
pub const XML_SCHEMAP_UNION_NOT_EXPRESSIBLE: C2RustUnnamed_0 = 1794;
pub const XML_SCHEMAP_INTERSECTION_NOT_EXPRESSIBLE: C2RustUnnamed_0 = 1793;
pub const XML_SCHEMAP_WILDCARD_INVALID_NS_MEMBER: C2RustUnnamed_0 = 1792;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_3: C2RustUnnamed_0 = 1791;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_2_2: C2RustUnnamed_0 = 1790;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_2_1_2: C2RustUnnamed_0 = 1789;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_2_1_1: C2RustUnnamed_0 = 1788;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_1: C2RustUnnamed_0 = 1787;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_5: C2RustUnnamed_0 = 1786;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_4: C2RustUnnamed_0 = 1785;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_3: C2RustUnnamed_0 = 1784;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_2: C2RustUnnamed_0 = 1783;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_1: C2RustUnnamed_0 = 1782;
pub const XML_SCHEMAP_REF_AND_CONTENT: C2RustUnnamed_0 = 1781;
pub const XML_SCHEMAP_INVALID_ATTR_NAME: C2RustUnnamed_0 = 1780;
pub const XML_SCHEMAP_MISSING_SIMPLETYPE_CHILD: C2RustUnnamed_0 = 1779;
pub const XML_SCHEMAP_INVALID_ATTR_INLINE_COMBINATION: C2RustUnnamed_0 = 1778;
pub const XML_SCHEMAP_INVALID_ATTR_COMBINATION: C2RustUnnamed_0 = 1777;
pub const XML_SCHEMAP_SUPERNUMEROUS_LIST_ITEM_TYPE: C2RustUnnamed_0 = 1776;
pub const XML_SCHEMAP_RECURSIVE: C2RustUnnamed_0 = 1775;
pub const XML_SCHEMAP_INVALID_ATTR_USE: C2RustUnnamed_0 = 1774;
pub const XML_SCHEMAP_UNKNOWN_MEMBER_TYPE: C2RustUnnamed_0 = 1773;
pub const XML_SCHEMAP_NOT_SCHEMA: C2RustUnnamed_0 = 1772;
pub const XML_SCHEMAP_INCLUDE_SCHEMA_NO_URI: C2RustUnnamed_0 = 1771;
pub const XML_SCHEMAP_INCLUDE_SCHEMA_NOT_URI: C2RustUnnamed_0 = 1770;
pub const XML_SCHEMAP_UNKNOWN_INCLUDE_CHILD: C2RustUnnamed_0 = 1769;
pub const XML_SCHEMAP_DEF_AND_PREFIX: C2RustUnnamed_0 = 1768;
pub const XML_SCHEMAP_UNKNOWN_PREFIX: C2RustUnnamed_0 = 1767;
pub const XML_SCHEMAP_FAILED_PARSE: C2RustUnnamed_0 = 1766;
pub const XML_SCHEMAP_REDEFINED_NOTATION: C2RustUnnamed_0 = 1765;
pub const XML_SCHEMAP_REDEFINED_ATTR: C2RustUnnamed_0 = 1764;
pub const XML_SCHEMAP_REDEFINED_ATTRGROUP: C2RustUnnamed_0 = 1763;
pub const XML_SCHEMAP_REDEFINED_ELEMENT: C2RustUnnamed_0 = 1762;
pub const XML_SCHEMAP_REDEFINED_TYPE: C2RustUnnamed_0 = 1761;
pub const XML_SCHEMAP_REDEFINED_GROUP: C2RustUnnamed_0 = 1760;
pub const XML_SCHEMAP_NOROOT: C2RustUnnamed_0 = 1759;
pub const XML_SCHEMAP_NOTHING_TO_PARSE: C2RustUnnamed_0 = 1758;
pub const XML_SCHEMAP_FAILED_LOAD: C2RustUnnamed_0 = 1757;
pub const XML_SCHEMAP_REGEXP_INVALID: C2RustUnnamed_0 = 1756;
pub const XML_SCHEMAP_ELEM_DEFAULT_FIXED: C2RustUnnamed_0 = 1755;
pub const XML_SCHEMAP_UNKNOWN_UNION_CHILD: C2RustUnnamed_0 = 1754;
pub const XML_SCHEMAP_UNKNOWN_TYPE: C2RustUnnamed_0 = 1753;
pub const XML_SCHEMAP_UNKNOWN_SIMPLETYPE_CHILD: C2RustUnnamed_0 = 1752;
pub const XML_SCHEMAP_UNKNOWN_SIMPLECONTENT_CHILD: C2RustUnnamed_0 = 1751;
pub const XML_SCHEMAP_UNKNOWN_SEQUENCE_CHILD: C2RustUnnamed_0 = 1750;
pub const XML_SCHEMAP_UNKNOWN_SCHEMAS_CHILD: C2RustUnnamed_0 = 1749;
pub const XML_SCHEMAP_UNKNOWN_RESTRICTION_CHILD: C2RustUnnamed_0 = 1748;
pub const XML_SCHEMAP_UNKNOWN_REF: C2RustUnnamed_0 = 1747;
pub const XML_SCHEMAP_UNKNOWN_PROCESSCONTENT_CHILD: C2RustUnnamed_0 = 1746;
pub const XML_SCHEMAP_UNKNOWN_NOTATION_CHILD: C2RustUnnamed_0 = 1745;
pub const XML_SCHEMAP_UNKNOWN_LIST_CHILD: C2RustUnnamed_0 = 1744;
pub const XML_SCHEMAP_UNKNOWN_IMPORT_CHILD: C2RustUnnamed_0 = 1743;
pub const XML_SCHEMAP_UNKNOWN_GROUP_CHILD: C2RustUnnamed_0 = 1742;
pub const XML_SCHEMAP_UNKNOWN_FACET_TYPE: C2RustUnnamed_0 = 1741;
pub const XML_SCHEMAP_UNKNOWN_FACET_CHILD: C2RustUnnamed_0 = 1740;
pub const XML_SCHEMAP_UNKNOWN_EXTENSION_CHILD: C2RustUnnamed_0 = 1739;
pub const XML_SCHEMAP_UNKNOWN_ELEM_CHILD: C2RustUnnamed_0 = 1738;
pub const XML_SCHEMAP_UNKNOWN_COMPLEXTYPE_CHILD: C2RustUnnamed_0 = 1737;
pub const XML_SCHEMAP_UNKNOWN_COMPLEXCONTENT_CHILD: C2RustUnnamed_0 = 1736;
pub const XML_SCHEMAP_UNKNOWN_CHOICE_CHILD: C2RustUnnamed_0 = 1735;
pub const XML_SCHEMAP_UNKNOWN_BASE_TYPE: C2RustUnnamed_0 = 1734;
pub const XML_SCHEMAP_UNKNOWN_ATTRIBUTE_GROUP: C2RustUnnamed_0 = 1733;
pub const XML_SCHEMAP_UNKNOWN_ATTRGRP_CHILD: C2RustUnnamed_0 = 1732;
pub const XML_SCHEMAP_UNKNOWN_ATTR_CHILD: C2RustUnnamed_0 = 1731;
pub const XML_SCHEMAP_UNKNOWN_ANYATTRIBUTE_CHILD: C2RustUnnamed_0 = 1730;
pub const XML_SCHEMAP_UNKNOWN_ALL_CHILD: C2RustUnnamed_0 = 1729;
pub const XML_SCHEMAP_TYPE_AND_SUBTYPE: C2RustUnnamed_0 = 1728;
pub const XML_SCHEMAP_SIMPLETYPE_NONAME: C2RustUnnamed_0 = 1727;
pub const XML_SCHEMAP_RESTRICTION_NONAME_NOREF: C2RustUnnamed_0 = 1726;
pub const XML_SCHEMAP_REF_AND_SUBTYPE: C2RustUnnamed_0 = 1725;
pub const XML_SCHEMAP_NOTYPE_NOREF: C2RustUnnamed_0 = 1724;
pub const XML_SCHEMAP_NOTATION_NO_NAME: C2RustUnnamed_0 = 1723;
pub const XML_SCHEMAP_NOATTR_NOREF: C2RustUnnamed_0 = 1722;
pub const XML_SCHEMAP_INVALID_WHITE_SPACE: C2RustUnnamed_0 = 1721;
pub const XML_SCHEMAP_INVALID_REF_AND_SUBTYPE: C2RustUnnamed_0 = 1720;
pub const XML_SCHEMAP_INVALID_MINOCCURS: C2RustUnnamed_0 = 1719;
pub const XML_SCHEMAP_INVALID_MAXOCCURS: C2RustUnnamed_0 = 1718;
pub const XML_SCHEMAP_INVALID_FACET_VALUE: C2RustUnnamed_0 = 1717;
pub const XML_SCHEMAP_INVALID_FACET: C2RustUnnamed_0 = 1716;
pub const XML_SCHEMAP_INVALID_ENUM: C2RustUnnamed_0 = 1715;
pub const XML_SCHEMAP_INVALID_BOOLEAN: C2RustUnnamed_0 = 1714;
pub const XML_SCHEMAP_IMPORT_SCHEMA_NOT_URI: C2RustUnnamed_0 = 1713;
pub const XML_SCHEMAP_IMPORT_REDEFINE_NSNAME: C2RustUnnamed_0 = 1712;
pub const XML_SCHEMAP_IMPORT_NAMESPACE_NOT_URI: C2RustUnnamed_0 = 1711;
pub const XML_SCHEMAP_GROUP_NONAME_NOREF: C2RustUnnamed_0 = 1710;
pub const XML_SCHEMAP_FAILED_BUILD_IMPORT: C2RustUnnamed_0 = 1709;
pub const XML_SCHEMAP_FACET_NO_VALUE: C2RustUnnamed_0 = 1708;
pub const XML_SCHEMAP_EXTENSION_NO_BASE: C2RustUnnamed_0 = 1707;
pub const XML_SCHEMAP_ELEM_NONAME_NOREF: C2RustUnnamed_0 = 1706;
pub const XML_SCHEMAP_ELEMFORMDEFAULT_VALUE: C2RustUnnamed_0 = 1705;
pub const XML_SCHEMAP_COMPLEXTYPE_NONAME_NOREF: C2RustUnnamed_0 = 1704;
pub const XML_SCHEMAP_ATTR_NONAME_NOREF: C2RustUnnamed_0 = 1703;
pub const XML_SCHEMAP_ATTRGRP_NONAME_NOREF: C2RustUnnamed_0 = 1702;
pub const XML_SCHEMAP_ATTRFORMDEFAULT_VALUE: C2RustUnnamed_0 = 1701;
pub const XML_SCHEMAP_PREFIX_UNDEFINED: C2RustUnnamed_0 = 1700;
pub const XML_CATALOG_RECURSION: C2RustUnnamed_0 = 1654;
pub const XML_CATALOG_NOT_CATALOG: C2RustUnnamed_0 = 1653;
pub const XML_CATALOG_PREFER_VALUE: C2RustUnnamed_0 = 1652;
pub const XML_CATALOG_ENTRY_BROKEN: C2RustUnnamed_0 = 1651;
pub const XML_CATALOG_MISSING_ATTR: C2RustUnnamed_0 = 1650;
pub const XML_XINCLUDE_FRAGMENT_ID: C2RustUnnamed_0 = 1618;
pub const XML_XINCLUDE_DEPRECATED_NS: C2RustUnnamed_0 = 1617;
pub const XML_XINCLUDE_FALLBACK_NOT_IN_INCLUDE: C2RustUnnamed_0 = 1616;
pub const XML_XINCLUDE_FALLBACKS_IN_INCLUDE: C2RustUnnamed_0 = 1615;
pub const XML_XINCLUDE_INCLUDE_IN_INCLUDE: C2RustUnnamed_0 = 1614;
pub const XML_XINCLUDE_XPTR_RESULT: C2RustUnnamed_0 = 1613;
pub const XML_XINCLUDE_XPTR_FAILED: C2RustUnnamed_0 = 1612;
pub const XML_XINCLUDE_MULTIPLE_ROOT: C2RustUnnamed_0 = 1611;
pub const XML_XINCLUDE_UNKNOWN_ENCODING: C2RustUnnamed_0 = 1610;
pub const XML_XINCLUDE_BUILD_FAILED: C2RustUnnamed_0 = 1609;
pub const XML_XINCLUDE_INVALID_CHAR: C2RustUnnamed_0 = 1608;
pub const XML_XINCLUDE_TEXT_DOCUMENT: C2RustUnnamed_0 = 1607;
pub const XML_XINCLUDE_TEXT_FRAGMENT: C2RustUnnamed_0 = 1606;
pub const XML_XINCLUDE_HREF_URI: C2RustUnnamed_0 = 1605;
pub const XML_XINCLUDE_NO_FALLBACK: C2RustUnnamed_0 = 1604;
pub const XML_XINCLUDE_NO_HREF: C2RustUnnamed_0 = 1603;
pub const XML_XINCLUDE_ENTITY_DEF_MISMATCH: C2RustUnnamed_0 = 1602;
pub const XML_XINCLUDE_PARSE_VALUE: C2RustUnnamed_0 = 1601;
pub const XML_XINCLUDE_RECURSION: C2RustUnnamed_0 = 1600;
pub const XML_IO_EAFNOSUPPORT: C2RustUnnamed_0 = 1556;
pub const XML_IO_EALREADY: C2RustUnnamed_0 = 1555;
pub const XML_IO_EADDRINUSE: C2RustUnnamed_0 = 1554;
pub const XML_IO_ENETUNREACH: C2RustUnnamed_0 = 1553;
pub const XML_IO_ECONNREFUSED: C2RustUnnamed_0 = 1552;
pub const XML_IO_EISCONN: C2RustUnnamed_0 = 1551;
pub const XML_IO_ENOTSOCK: C2RustUnnamed_0 = 1550;
pub const XML_IO_LOAD_ERROR: C2RustUnnamed_0 = 1549;
pub const XML_IO_BUFFER_FULL: C2RustUnnamed_0 = 1548;
pub const XML_IO_NO_INPUT: C2RustUnnamed_0 = 1547;
pub const XML_IO_WRITE: C2RustUnnamed_0 = 1546;
pub const XML_IO_FLUSH: C2RustUnnamed_0 = 1545;
pub const XML_IO_ENCODER: C2RustUnnamed_0 = 1544;
pub const XML_IO_NETWORK_ATTEMPT: C2RustUnnamed_0 = 1543;
pub const XML_IO_EXDEV: C2RustUnnamed_0 = 1542;
pub const XML_IO_ETIMEDOUT: C2RustUnnamed_0 = 1541;
pub const XML_IO_ESRCH: C2RustUnnamed_0 = 1540;
pub const XML_IO_ESPIPE: C2RustUnnamed_0 = 1539;
pub const XML_IO_EROFS: C2RustUnnamed_0 = 1538;
pub const XML_IO_ERANGE: C2RustUnnamed_0 = 1537;
pub const XML_IO_EPIPE: C2RustUnnamed_0 = 1536;
pub const XML_IO_EPERM: C2RustUnnamed_0 = 1535;
pub const XML_IO_ENXIO: C2RustUnnamed_0 = 1534;
pub const XML_IO_ENOTTY: C2RustUnnamed_0 = 1533;
pub const XML_IO_ENOTSUP: C2RustUnnamed_0 = 1532;
pub const XML_IO_ENOTEMPTY: C2RustUnnamed_0 = 1531;
pub const XML_IO_ENOTDIR: C2RustUnnamed_0 = 1530;
pub const XML_IO_ENOSYS: C2RustUnnamed_0 = 1529;
pub const XML_IO_ENOSPC: C2RustUnnamed_0 = 1528;
pub const XML_IO_ENOMEM: C2RustUnnamed_0 = 1527;
pub const XML_IO_ENOLCK: C2RustUnnamed_0 = 1526;
pub const XML_IO_ENOEXEC: C2RustUnnamed_0 = 1525;
pub const XML_IO_ENOENT: C2RustUnnamed_0 = 1524;
pub const XML_IO_ENODEV: C2RustUnnamed_0 = 1523;
pub const XML_IO_ENFILE: C2RustUnnamed_0 = 1522;
pub const XML_IO_ENAMETOOLONG: C2RustUnnamed_0 = 1521;
pub const XML_IO_EMSGSIZE: C2RustUnnamed_0 = 1520;
pub const XML_IO_EMLINK: C2RustUnnamed_0 = 1519;
pub const XML_IO_EMFILE: C2RustUnnamed_0 = 1518;
pub const XML_IO_EISDIR: C2RustUnnamed_0 = 1517;
pub const XML_IO_EIO: C2RustUnnamed_0 = 1516;
pub const XML_IO_EINVAL: C2RustUnnamed_0 = 1515;
pub const XML_IO_EINTR: C2RustUnnamed_0 = 1514;
pub const XML_IO_EINPROGRESS: C2RustUnnamed_0 = 1513;
pub const XML_IO_EFBIG: C2RustUnnamed_0 = 1512;
pub const XML_IO_EFAULT: C2RustUnnamed_0 = 1511;
pub const XML_IO_EEXIST: C2RustUnnamed_0 = 1510;
pub const XML_IO_EDOM: C2RustUnnamed_0 = 1509;
pub const XML_IO_EDEADLK: C2RustUnnamed_0 = 1508;
pub const XML_IO_ECHILD: C2RustUnnamed_0 = 1507;
pub const XML_IO_ECANCELED: C2RustUnnamed_0 = 1506;
pub const XML_IO_EBUSY: C2RustUnnamed_0 = 1505;
pub const XML_IO_EBADMSG: C2RustUnnamed_0 = 1504;
pub const XML_IO_EBADF: C2RustUnnamed_0 = 1503;
pub const XML_IO_EAGAIN: C2RustUnnamed_0 = 1502;
pub const XML_IO_EACCES: C2RustUnnamed_0 = 1501;
pub const XML_IO_UNKNOWN: C2RustUnnamed_0 = 1500;
pub const XML_REGEXP_COMPILE_ERROR: C2RustUnnamed_0 = 1450;
pub const XML_SAVE_UNKNOWN_ENCODING: C2RustUnnamed_0 = 1403;
pub const XML_SAVE_NO_DOCTYPE: C2RustUnnamed_0 = 1402;
pub const XML_SAVE_CHAR_INVALID: C2RustUnnamed_0 = 1401;
pub const XML_SAVE_NOT_UTF8: C2RustUnnamed_0 = 1400;
pub const XML_TREE_NOT_UTF8: C2RustUnnamed_0 = 1303;
pub const XML_TREE_UNTERMINATED_ENTITY: C2RustUnnamed_0 = 1302;
pub const XML_TREE_INVALID_DEC: C2RustUnnamed_0 = 1301;
pub const XML_TREE_INVALID_HEX: C2RustUnnamed_0 = 1300;
pub const XML_XPATH_INVALID_CHAR_ERROR: C2RustUnnamed_0 = 1221;
pub const XML_XPATH_ENCODING_ERROR: C2RustUnnamed_0 = 1220;
pub const XML_XPATH_UNDEF_PREFIX_ERROR: C2RustUnnamed_0 = 1219;
pub const XML_XPTR_SUB_RESOURCE_ERROR: C2RustUnnamed_0 = 1218;
pub const XML_XPTR_RESOURCE_ERROR: C2RustUnnamed_0 = 1217;
pub const XML_XPTR_SYNTAX_ERROR: C2RustUnnamed_0 = 1216;
pub const XML_XPATH_MEMORY_ERROR: C2RustUnnamed_0 = 1215;
pub const XML_XPATH_INVALID_CTXT_POSITION: C2RustUnnamed_0 = 1214;
pub const XML_XPATH_INVALID_CTXT_SIZE: C2RustUnnamed_0 = 1213;
pub const XML_XPATH_INVALID_ARITY: C2RustUnnamed_0 = 1212;
pub const XML_XPATH_INVALID_TYPE: C2RustUnnamed_0 = 1211;
pub const XML_XPATH_INVALID_OPERAND: C2RustUnnamed_0 = 1210;
pub const XML_XPATH_UNKNOWN_FUNC_ERROR: C2RustUnnamed_0 = 1209;
pub const XML_XPATH_UNCLOSED_ERROR: C2RustUnnamed_0 = 1208;
pub const XML_XPATH_EXPR_ERROR: C2RustUnnamed_0 = 1207;
pub const XML_XPATH_INVALID_PREDICATE_ERROR: C2RustUnnamed_0 = 1206;
pub const XML_XPATH_UNDEF_VARIABLE_ERROR: C2RustUnnamed_0 = 1205;
pub const XML_XPATH_VARIABLE_REF_ERROR: C2RustUnnamed_0 = 1204;
pub const XML_XPATH_START_LITERAL_ERROR: C2RustUnnamed_0 = 1203;
pub const XML_XPATH_UNFINISHED_LITERAL_ERROR: C2RustUnnamed_0 = 1202;
pub const XML_XPATH_NUMBER_ERROR: C2RustUnnamed_0 = 1201;
pub const XML_XPATH_EXPRESSION_OK: C2RustUnnamed_0 = 1200;
pub const XML_RNGP_XML_NS: C2RustUnnamed_0 = 1122;
pub const XML_RNGP_XMLNS_NAME: C2RustUnnamed_0 = 1121;
pub const XML_RNGP_VALUE_NO_CONTENT: C2RustUnnamed_0 = 1120;
pub const XML_RNGP_VALUE_EMPTY: C2RustUnnamed_0 = 1119;
pub const XML_RNGP_URI_NOT_ABSOLUTE: C2RustUnnamed_0 = 1118;
pub const XML_RNGP_URI_FRAGMENT: C2RustUnnamed_0 = 1117;
pub const XML_RNGP_UNKNOWN_TYPE_LIB: C2RustUnnamed_0 = 1116;
pub const XML_RNGP_UNKNOWN_CONSTRUCT: C2RustUnnamed_0 = 1115;
pub const XML_RNGP_UNKNOWN_COMBINE: C2RustUnnamed_0 = 1114;
pub const XML_RNGP_UNKNOWN_ATTRIBUTE: C2RustUnnamed_0 = 1113;
pub const XML_RNGP_TYPE_VALUE: C2RustUnnamed_0 = 1112;
pub const XML_RNGP_TYPE_NOT_FOUND: C2RustUnnamed_0 = 1111;
pub const XML_RNGP_TYPE_MISSING: C2RustUnnamed_0 = 1110;
pub const XML_RNGP_TEXT_HAS_CHILD: C2RustUnnamed_0 = 1109;
pub const XML_RNGP_TEXT_EXPECTED: C2RustUnnamed_0 = 1108;
pub const XML_RNGP_START_MISSING: C2RustUnnamed_0 = 1107;
pub const XML_RNGP_START_EMPTY: C2RustUnnamed_0 = 1106;
pub const XML_RNGP_START_CONTENT: C2RustUnnamed_0 = 1105;
pub const XML_RNGP_START_CHOICE_AND_INTERLEAVE: C2RustUnnamed_0 = 1104;
pub const XML_RNGP_REF_NOT_EMPTY: C2RustUnnamed_0 = 1103;
pub const XML_RNGP_REF_NO_NAME: C2RustUnnamed_0 = 1102;
pub const XML_RNGP_REF_NO_DEF: C2RustUnnamed_0 = 1101;
pub const XML_RNGP_REF_NAME_INVALID: C2RustUnnamed_0 = 1100;
pub const XML_RNGP_REF_CYCLE: C2RustUnnamed_0 = 1099;
pub const XML_RNGP_REF_CREATE_FAILED: C2RustUnnamed_0 = 1098;
pub const XML_RNGP_PREFIX_UNDEFINED: C2RustUnnamed_0 = 1097;
pub const XML_RNGP_PAT_START_VALUE: C2RustUnnamed_0 = 1096;
pub const XML_RNGP_PAT_START_TEXT: C2RustUnnamed_0 = 1095;
pub const XML_RNGP_PAT_START_ONEMORE: C2RustUnnamed_0 = 1094;
pub const XML_RNGP_PAT_START_LIST: C2RustUnnamed_0 = 1093;
pub const XML_RNGP_PAT_START_INTERLEAVE: C2RustUnnamed_0 = 1092;
pub const XML_RNGP_PAT_START_GROUP: C2RustUnnamed_0 = 1091;
pub const XML_RNGP_PAT_START_EMPTY: C2RustUnnamed_0 = 1090;
pub const XML_RNGP_PAT_START_DATA: C2RustUnnamed_0 = 1089;
pub const XML_RNGP_PAT_START_ATTR: C2RustUnnamed_0 = 1088;
pub const XML_RNGP_PAT_ONEMORE_INTERLEAVE_ATTR: C2RustUnnamed_0 = 1087;
pub const XML_RNGP_PAT_ONEMORE_GROUP_ATTR: C2RustUnnamed_0 = 1086;
pub const XML_RNGP_PAT_NSNAME_EXCEPT_NSNAME: C2RustUnnamed_0 = 1085;
pub const XML_RNGP_PAT_NSNAME_EXCEPT_ANYNAME: C2RustUnnamed_0 = 1084;
pub const XML_RNGP_PAT_LIST_TEXT: C2RustUnnamed_0 = 1083;
pub const XML_RNGP_PAT_LIST_REF: C2RustUnnamed_0 = 1082;
pub const XML_RNGP_PAT_LIST_LIST: C2RustUnnamed_0 = 1081;
pub const XML_RNGP_PAT_LIST_INTERLEAVE: C2RustUnnamed_0 = 1080;
pub const XML_RNGP_PAT_LIST_ELEM: C2RustUnnamed_0 = 1079;
pub const XML_RNGP_PAT_LIST_ATTR: C2RustUnnamed_0 = 1078;
pub const XML_RNGP_PAT_DATA_EXCEPT_TEXT: C2RustUnnamed_0 = 1077;
pub const XML_RNGP_PAT_DATA_EXCEPT_REF: C2RustUnnamed_0 = 1076;
pub const XML_RNGP_PAT_DATA_EXCEPT_ONEMORE: C2RustUnnamed_0 = 1075;
pub const XML_RNGP_PAT_DATA_EXCEPT_LIST: C2RustUnnamed_0 = 1074;
pub const XML_RNGP_PAT_DATA_EXCEPT_INTERLEAVE: C2RustUnnamed_0 = 1073;
pub const XML_RNGP_PAT_DATA_EXCEPT_GROUP: C2RustUnnamed_0 = 1072;
pub const XML_RNGP_PAT_DATA_EXCEPT_EMPTY: C2RustUnnamed_0 = 1071;
pub const XML_RNGP_PAT_DATA_EXCEPT_ELEM: C2RustUnnamed_0 = 1070;
pub const XML_RNGP_PAT_DATA_EXCEPT_ATTR: C2RustUnnamed_0 = 1069;
pub const XML_RNGP_PAT_ATTR_ELEM: C2RustUnnamed_0 = 1068;
pub const XML_RNGP_PAT_ATTR_ATTR: C2RustUnnamed_0 = 1067;
pub const XML_RNGP_PAT_ANYNAME_EXCEPT_ANYNAME: C2RustUnnamed_0 = 1066;
pub const XML_RNGP_PARSE_ERROR: C2RustUnnamed_0 = 1065;
pub const XML_RNGP_PARENTREF_NOT_EMPTY: C2RustUnnamed_0 = 1064;
pub const XML_RNGP_PARENTREF_NO_PARENT: C2RustUnnamed_0 = 1063;
pub const XML_RNGP_PARENTREF_NO_NAME: C2RustUnnamed_0 = 1062;
pub const XML_RNGP_PARENTREF_NAME_INVALID: C2RustUnnamed_0 = 1061;
pub const XML_RNGP_PARENTREF_CREATE_FAILED: C2RustUnnamed_0 = 1060;
pub const XML_RNGP_PARAM_NAME_MISSING: C2RustUnnamed_0 = 1059;
pub const XML_RNGP_PARAM_FORBIDDEN: C2RustUnnamed_0 = 1058;
pub const XML_RNGP_NSNAME_NO_NS: C2RustUnnamed_0 = 1057;
pub const XML_RNGP_NSNAME_ATTR_ANCESTOR: C2RustUnnamed_0 = 1056;
pub const XML_RNGP_NOTALLOWED_NOT_EMPTY: C2RustUnnamed_0 = 1055;
pub const XML_RNGP_NEED_COMBINE: C2RustUnnamed_0 = 1054;
pub const XML_RNGP_NAME_MISSING: C2RustUnnamed_0 = 1053;
pub const XML_RNGP_MISSING_HREF: C2RustUnnamed_0 = 1052;
pub const XML_RNGP_INVALID_VALUE: C2RustUnnamed_0 = 1051;
pub const XML_RNGP_INVALID_URI: C2RustUnnamed_0 = 1050;
pub const XML_RNGP_INVALID_DEFINE_NAME: C2RustUnnamed_0 = 1049;
pub const XML_RNGP_INTERLEAVE_NO_CONTENT: C2RustUnnamed_0 = 1048;
pub const XML_RNGP_INTERLEAVE_EMPTY: C2RustUnnamed_0 = 1047;
pub const XML_RNGP_INTERLEAVE_CREATE_FAILED: C2RustUnnamed_0 = 1046;
pub const XML_RNGP_INTERLEAVE_ADD: C2RustUnnamed_0 = 1045;
pub const XML_RNGP_INCLUDE_RECURSE: C2RustUnnamed_0 = 1044;
pub const XML_RNGP_INCLUDE_FAILURE: C2RustUnnamed_0 = 1043;
pub const XML_RNGP_INCLUDE_EMPTY: C2RustUnnamed_0 = 1042;
pub const XML_RNGP_HREF_ERROR: C2RustUnnamed_0 = 1041;
pub const XML_RNGP_GROUP_ATTR_CONFLICT: C2RustUnnamed_0 = 1040;
pub const XML_RNGP_GRAMMAR_NO_START: C2RustUnnamed_0 = 1039;
pub const XML_RNGP_GRAMMAR_MISSING: C2RustUnnamed_0 = 1038;
pub const XML_RNGP_GRAMMAR_EMPTY: C2RustUnnamed_0 = 1037;
pub const XML_RNGP_GRAMMAR_CONTENT: C2RustUnnamed_0 = 1036;
pub const XML_RNGP_FOREIGN_ELEMENT: C2RustUnnamed_0 = 1035;
pub const XML_RNGP_FORBIDDEN_ATTRIBUTE: C2RustUnnamed_0 = 1034;
pub const XML_RNGP_EXTERNALREF_RECURSE: C2RustUnnamed_0 = 1033;
pub const XML_RNGP_EXTERNAL_REF_FAILURE: C2RustUnnamed_0 = 1032;
pub const XML_RNGP_EXTERNALREF_EMTPY: C2RustUnnamed_0 = 1031;
pub const XML_RNGP_EXCEPT_NO_CONTENT: C2RustUnnamed_0 = 1030;
pub const XML_RNGP_EXCEPT_MULTIPLE: C2RustUnnamed_0 = 1029;
pub const XML_RNGP_EXCEPT_MISSING: C2RustUnnamed_0 = 1028;
pub const XML_RNGP_EXCEPT_EMPTY: C2RustUnnamed_0 = 1027;
pub const XML_RNGP_ERROR_TYPE_LIB: C2RustUnnamed_0 = 1026;
pub const XML_RNGP_EMPTY_NOT_EMPTY: C2RustUnnamed_0 = 1025;
pub const XML_RNGP_EMPTY_CONTENT: C2RustUnnamed_0 = 1024;
pub const XML_RNGP_EMPTY_CONSTRUCT: C2RustUnnamed_0 = 1023;
pub const XML_RNGP_EMPTY: C2RustUnnamed_0 = 1022;
pub const XML_RNGP_ELEM_TEXT_CONFLICT: C2RustUnnamed_0 = 1021;
pub const XML_RNGP_ELEMENT_NO_CONTENT: C2RustUnnamed_0 = 1020;
pub const XML_RNGP_ELEMENT_NAME: C2RustUnnamed_0 = 1019;
pub const XML_RNGP_ELEMENT_CONTENT: C2RustUnnamed_0 = 1018;
pub const XML_RNGP_ELEMENT_EMPTY: C2RustUnnamed_0 = 1017;
pub const XML_RNGP_ELEM_CONTENT_ERROR: C2RustUnnamed_0 = 1016;
pub const XML_RNGP_ELEM_CONTENT_EMPTY: C2RustUnnamed_0 = 1015;
pub const XML_RNGP_DEFINE_NAME_MISSING: C2RustUnnamed_0 = 1014;
pub const XML_RNGP_DEFINE_MISSING: C2RustUnnamed_0 = 1013;
pub const XML_RNGP_DEFINE_EMPTY: C2RustUnnamed_0 = 1012;
pub const XML_RNGP_DEFINE_CREATE_FAILED: C2RustUnnamed_0 = 1011;
pub const XML_RNGP_DEF_CHOICE_AND_INTERLEAVE: C2RustUnnamed_0 = 1010;
pub const XML_RNGP_DATA_CONTENT: C2RustUnnamed_0 = 1009;
pub const XML_RNGP_CREATE_FAILURE: C2RustUnnamed_0 = 1008;
pub const XML_RNGP_CHOICE_EMPTY: C2RustUnnamed_0 = 1007;
pub const XML_RNGP_CHOICE_CONTENT: C2RustUnnamed_0 = 1006;
pub const XML_RNGP_ATTRIBUTE_NOOP: C2RustUnnamed_0 = 1005;
pub const XML_RNGP_ATTRIBUTE_EMPTY: C2RustUnnamed_0 = 1004;
pub const XML_RNGP_ATTRIBUTE_CONTENT: C2RustUnnamed_0 = 1003;
pub const XML_RNGP_ATTRIBUTE_CHILDREN: C2RustUnnamed_0 = 1002;
pub const XML_RNGP_ATTR_CONFLICT: C2RustUnnamed_0 = 1001;
pub const XML_RNGP_ANYNAME_ATTR_ANCESTOR: C2RustUnnamed_0 = 1000;
pub const XML_HTML_UNKNOWN_TAG: C2RustUnnamed_0 = 801;
pub const XML_HTML_STRUCURE_ERROR: C2RustUnnamed_0 = 800;
pub const XML_DTD_DUP_TOKEN: C2RustUnnamed_0 = 541;
pub const XML_DTD_XMLID_TYPE: C2RustUnnamed_0 = 540;
pub const XML_DTD_XMLID_VALUE: C2RustUnnamed_0 = 539;
pub const XML_DTD_STANDALONE_DEFAULTED: C2RustUnnamed_0 = 538;
pub const XML_DTD_UNKNOWN_NOTATION: C2RustUnnamed_0 = 537;
pub const XML_DTD_UNKNOWN_ID: C2RustUnnamed_0 = 536;
pub const XML_DTD_UNKNOWN_ENTITY: C2RustUnnamed_0 = 535;
pub const XML_DTD_UNKNOWN_ELEM: C2RustUnnamed_0 = 534;
pub const XML_DTD_UNKNOWN_ATTRIBUTE: C2RustUnnamed_0 = 533;
pub const XML_DTD_STANDALONE_WHITE_SPACE: C2RustUnnamed_0 = 532;
pub const XML_DTD_ROOT_NAME: C2RustUnnamed_0 = 531;
pub const XML_DTD_NOT_STANDALONE: C2RustUnnamed_0 = 530;
pub const XML_DTD_NOT_PCDATA: C2RustUnnamed_0 = 529;
pub const XML_DTD_NOT_EMPTY: C2RustUnnamed_0 = 528;
pub const XML_DTD_NOTATION_VALUE: C2RustUnnamed_0 = 527;
pub const XML_DTD_NOTATION_REDEFINED: C2RustUnnamed_0 = 526;
pub const XML_DTD_NO_ROOT: C2RustUnnamed_0 = 525;
pub const XML_DTD_NO_PREFIX: C2RustUnnamed_0 = 524;
pub const XML_DTD_NO_ELEM_NAME: C2RustUnnamed_0 = 523;
pub const XML_DTD_NO_DTD: C2RustUnnamed_0 = 522;
pub const XML_DTD_NO_DOC: C2RustUnnamed_0 = 521;
pub const XML_DTD_MULTIPLE_ID: C2RustUnnamed_0 = 520;
pub const XML_DTD_MIXED_CORRUPT: C2RustUnnamed_0 = 519;
pub const XML_DTD_MISSING_ATTRIBUTE: C2RustUnnamed_0 = 518;
pub const XML_DTD_LOAD_ERROR: C2RustUnnamed_0 = 517;
pub const XML_DTD_INVALID_DEFAULT: C2RustUnnamed_0 = 516;
pub const XML_DTD_INVALID_CHILD: C2RustUnnamed_0 = 515;
pub const XML_DTD_ID_SUBSET: C2RustUnnamed_0 = 514;
pub const XML_DTD_ID_REDEFINED: C2RustUnnamed_0 = 513;
pub const XML_DTD_ID_FIXED: C2RustUnnamed_0 = 512;
pub const XML_DTD_ENTITY_TYPE: C2RustUnnamed_0 = 511;
pub const XML_DTD_EMPTY_NOTATION: C2RustUnnamed_0 = 510;
pub const XML_DTD_ELEM_REDEFINED: C2RustUnnamed_0 = 509;
pub const XML_DTD_ELEM_NAMESPACE: C2RustUnnamed_0 = 508;
pub const XML_DTD_ELEM_DEFAULT_NAMESPACE: C2RustUnnamed_0 = 507;
pub const XML_DTD_DIFFERENT_PREFIX: C2RustUnnamed_0 = 506;
pub const XML_DTD_CONTENT_NOT_DETERMINIST: C2RustUnnamed_0 = 505;
pub const XML_DTD_CONTENT_MODEL: C2RustUnnamed_0 = 504;
pub const XML_DTD_CONTENT_ERROR: C2RustUnnamed_0 = 503;
pub const XML_DTD_ATTRIBUTE_VALUE: C2RustUnnamed_0 = 502;
pub const XML_DTD_ATTRIBUTE_REDEFINED: C2RustUnnamed_0 = 501;
pub const XML_DTD_ATTRIBUTE_DEFAULT: C2RustUnnamed_0 = 500;
pub const XML_NS_ERR_COLON: C2RustUnnamed_0 = 205;
pub const XML_NS_ERR_EMPTY: C2RustUnnamed_0 = 204;
pub const XML_NS_ERR_ATTRIBUTE_REDEFINED: C2RustUnnamed_0 = 203;
pub const XML_NS_ERR_QNAME: C2RustUnnamed_0 = 202;
pub const XML_NS_ERR_UNDEFINED_NAMESPACE: C2RustUnnamed_0 = 201;
pub const XML_NS_ERR_XML_NAMESPACE: C2RustUnnamed_0 = 200;
pub const XML_ERR_USER_STOP: C2RustUnnamed_0 = 111;
pub const XML_ERR_NAME_TOO_LONG: C2RustUnnamed_0 = 110;
pub const XML_ERR_VERSION_MISMATCH: C2RustUnnamed_0 = 109;
pub const XML_ERR_UNKNOWN_VERSION: C2RustUnnamed_0 = 108;
pub const XML_WAR_ENTITY_REDEFINED: C2RustUnnamed_0 = 107;
pub const XML_WAR_NS_COLUMN: C2RustUnnamed_0 = 106;
pub const XML_ERR_NOTATION_PROCESSING: C2RustUnnamed_0 = 105;
pub const XML_ERR_ENTITY_PROCESSING: C2RustUnnamed_0 = 104;
pub const XML_ERR_NOT_STANDALONE: C2RustUnnamed_0 = 103;
pub const XML_WAR_SPACE_VALUE: C2RustUnnamed_0 = 102;
pub const XML_ERR_MISSING_ENCODING: C2RustUnnamed_0 = 101;
pub const XML_WAR_NS_URI_RELATIVE: C2RustUnnamed_0 = 100;
pub const XML_WAR_NS_URI: C2RustUnnamed_0 = 99;
pub const XML_WAR_LANG_VALUE: C2RustUnnamed_0 = 98;
pub const XML_WAR_UNKNOWN_VERSION: C2RustUnnamed_0 = 97;
pub const XML_ERR_VERSION_MISSING: C2RustUnnamed_0 = 96;
pub const XML_ERR_CONDSEC_INVALID_KEYWORD: C2RustUnnamed_0 = 95;
pub const XML_ERR_NO_DTD: C2RustUnnamed_0 = 94;
pub const XML_WAR_CATALOG_PI: C2RustUnnamed_0 = 93;
pub const XML_ERR_URI_FRAGMENT: C2RustUnnamed_0 = 92;
pub const XML_ERR_INVALID_URI: C2RustUnnamed_0 = 91;
pub const XML_ERR_ENTITY_BOUNDARY: C2RustUnnamed_0 = 90;
pub const XML_ERR_ENTITY_LOOP: C2RustUnnamed_0 = 89;
pub const XML_ERR_ENTITY_PE_INTERNAL: C2RustUnnamed_0 = 88;
pub const XML_ERR_ENTITY_CHAR_ERROR: C2RustUnnamed_0 = 87;
pub const XML_ERR_EXTRA_CONTENT: C2RustUnnamed_0 = 86;
pub const XML_ERR_NOT_WELL_BALANCED: C2RustUnnamed_0 = 85;
pub const XML_ERR_VALUE_REQUIRED: C2RustUnnamed_0 = 84;
pub const XML_ERR_CONDSEC_INVALID: C2RustUnnamed_0 = 83;
pub const XML_ERR_EXT_ENTITY_STANDALONE: C2RustUnnamed_0 = 82;
pub const XML_ERR_INVALID_ENCODING: C2RustUnnamed_0 = 81;
pub const XML_ERR_HYPHEN_IN_COMMENT: C2RustUnnamed_0 = 80;
pub const XML_ERR_ENCODING_NAME: C2RustUnnamed_0 = 79;
pub const XML_ERR_STANDALONE_VALUE: C2RustUnnamed_0 = 78;
pub const XML_ERR_TAG_NOT_FINISHED: C2RustUnnamed_0 = 77;
pub const XML_ERR_TAG_NAME_MISMATCH: C2RustUnnamed_0 = 76;
pub const XML_ERR_EQUAL_REQUIRED: C2RustUnnamed_0 = 75;
pub const XML_ERR_LTSLASH_REQUIRED: C2RustUnnamed_0 = 74;
pub const XML_ERR_GT_REQUIRED: C2RustUnnamed_0 = 73;
pub const XML_ERR_LT_REQUIRED: C2RustUnnamed_0 = 72;
pub const XML_ERR_PUBID_REQUIRED: C2RustUnnamed_0 = 71;
pub const XML_ERR_URI_REQUIRED: C2RustUnnamed_0 = 70;
pub const XML_ERR_PCDATA_REQUIRED: C2RustUnnamed_0 = 69;
pub const XML_ERR_NAME_REQUIRED: C2RustUnnamed_0 = 68;
pub const XML_ERR_NMTOKEN_REQUIRED: C2RustUnnamed_0 = 67;
pub const XML_ERR_SEPARATOR_REQUIRED: C2RustUnnamed_0 = 66;
pub const XML_ERR_SPACE_REQUIRED: C2RustUnnamed_0 = 65;
pub const XML_ERR_RESERVED_XML_NAME: C2RustUnnamed_0 = 64;
pub const XML_ERR_CDATA_NOT_FINISHED: C2RustUnnamed_0 = 63;
pub const XML_ERR_MISPLACED_CDATA_END: C2RustUnnamed_0 = 62;
pub const XML_ERR_DOCTYPE_NOT_FINISHED: C2RustUnnamed_0 = 61;
pub const XML_ERR_EXT_SUBSET_NOT_FINISHED: C2RustUnnamed_0 = 60;
pub const XML_ERR_CONDSEC_NOT_FINISHED: C2RustUnnamed_0 = 59;
pub const XML_ERR_CONDSEC_NOT_STARTED: C2RustUnnamed_0 = 58;
pub const XML_ERR_XMLDECL_NOT_FINISHED: C2RustUnnamed_0 = 57;
pub const XML_ERR_XMLDECL_NOT_STARTED: C2RustUnnamed_0 = 56;
pub const XML_ERR_ELEMCONTENT_NOT_FINISHED: C2RustUnnamed_0 = 55;
pub const XML_ERR_ELEMCONTENT_NOT_STARTED: C2RustUnnamed_0 = 54;
pub const XML_ERR_MIXED_NOT_FINISHED: C2RustUnnamed_0 = 53;
pub const XML_ERR_MIXED_NOT_STARTED: C2RustUnnamed_0 = 52;
pub const XML_ERR_ATTLIST_NOT_FINISHED: C2RustUnnamed_0 = 51;
pub const XML_ERR_ATTLIST_NOT_STARTED: C2RustUnnamed_0 = 50;
pub const XML_ERR_NOTATION_NOT_FINISHED: C2RustUnnamed_0 = 49;
pub const XML_ERR_NOTATION_NOT_STARTED: C2RustUnnamed_0 = 48;
pub const XML_ERR_PI_NOT_FINISHED: C2RustUnnamed_0 = 47;
pub const XML_ERR_PI_NOT_STARTED: C2RustUnnamed_0 = 46;
pub const XML_ERR_COMMENT_NOT_FINISHED: C2RustUnnamed_0 = 45;
pub const XML_ERR_LITERAL_NOT_FINISHED: C2RustUnnamed_0 = 44;
pub const XML_ERR_LITERAL_NOT_STARTED: C2RustUnnamed_0 = 43;
pub const XML_ERR_ATTRIBUTE_REDEFINED: C2RustUnnamed_0 = 42;
pub const XML_ERR_ATTRIBUTE_WITHOUT_VALUE: C2RustUnnamed_0 = 41;
pub const XML_ERR_ATTRIBUTE_NOT_FINISHED: C2RustUnnamed_0 = 40;
pub const XML_ERR_ATTRIBUTE_NOT_STARTED: C2RustUnnamed_0 = 39;
pub const XML_ERR_LT_IN_ATTRIBUTE: C2RustUnnamed_0 = 38;
pub const XML_ERR_ENTITY_NOT_FINISHED: C2RustUnnamed_0 = 37;
pub const XML_ERR_ENTITY_NOT_STARTED: C2RustUnnamed_0 = 36;
pub const XML_ERR_NS_DECL_ERROR: C2RustUnnamed_0 = 35;
pub const XML_ERR_STRING_NOT_CLOSED: C2RustUnnamed_0 = 34;
pub const XML_ERR_STRING_NOT_STARTED: C2RustUnnamed_0 = 33;
pub const XML_ERR_UNSUPPORTED_ENCODING: C2RustUnnamed_0 = 32;
pub const XML_ERR_UNKNOWN_ENCODING: C2RustUnnamed_0 = 31;
pub const XML_ERR_ENTITY_IS_PARAMETER: C2RustUnnamed_0 = 30;
pub const XML_ERR_ENTITY_IS_EXTERNAL: C2RustUnnamed_0 = 29;
pub const XML_ERR_UNPARSED_ENTITY: C2RustUnnamed_0 = 28;
pub const XML_WAR_UNDECLARED_ENTITY: C2RustUnnamed_0 = 27;
pub const XML_ERR_UNDECLARED_ENTITY: C2RustUnnamed_0 = 26;
pub const XML_ERR_PEREF_SEMICOL_MISSING: C2RustUnnamed_0 = 25;
pub const XML_ERR_PEREF_NO_NAME: C2RustUnnamed_0 = 24;
pub const XML_ERR_ENTITYREF_SEMICOL_MISSING: C2RustUnnamed_0 = 23;
pub const XML_ERR_ENTITYREF_NO_NAME: C2RustUnnamed_0 = 22;
pub const XML_ERR_PEREF_IN_INT_SUBSET: C2RustUnnamed_0 = 21;
pub const XML_ERR_PEREF_IN_EPILOG: C2RustUnnamed_0 = 20;
pub const XML_ERR_PEREF_IN_PROLOG: C2RustUnnamed_0 = 19;
pub const XML_ERR_PEREF_AT_EOF: C2RustUnnamed_0 = 18;
pub const XML_ERR_ENTITYREF_IN_DTD: C2RustUnnamed_0 = 17;
pub const XML_ERR_ENTITYREF_IN_EPILOG: C2RustUnnamed_0 = 16;
pub const XML_ERR_ENTITYREF_IN_PROLOG: C2RustUnnamed_0 = 15;
pub const XML_ERR_ENTITYREF_AT_EOF: C2RustUnnamed_0 = 14;
pub const XML_ERR_CHARREF_IN_DTD: C2RustUnnamed_0 = 13;
pub const XML_ERR_CHARREF_IN_EPILOG: C2RustUnnamed_0 = 12;
pub const XML_ERR_CHARREF_IN_PROLOG: C2RustUnnamed_0 = 11;
pub const XML_ERR_CHARREF_AT_EOF: C2RustUnnamed_0 = 10;
pub const XML_ERR_INVALID_CHAR: C2RustUnnamed_0 = 9;
pub const XML_ERR_INVALID_CHARREF: C2RustUnnamed_0 = 8;
pub const XML_ERR_INVALID_DEC_CHARREF: C2RustUnnamed_0 = 7;
pub const XML_ERR_INVALID_HEX_CHARREF: C2RustUnnamed_0 = 6;
pub const XML_ERR_DOCUMENT_END: C2RustUnnamed_0 = 5;
pub const XML_ERR_DOCUMENT_EMPTY: C2RustUnnamed_0 = 4;
pub const XML_ERR_DOCUMENT_START: C2RustUnnamed_0 = 3;
pub const XML_ERR_NO_MEMORY: C2RustUnnamed_0 = 2;
pub const XML_ERR_INTERNAL_ERROR: C2RustUnnamed_0 = 1;
pub const XML_ERR_OK: C2RustUnnamed_0 = 0;
/* *
 * xmlGenericErrorFunc:
 * @ctx:  a parsing context
 * @msg:  the message
 * @...:  the extra arguments of the varags to format the message
 *
 * Signature of the function to use when there is an error and
 * no parsing or validity context available .
 */
pub type xmlGenericErrorFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const std::os::raw::c_char,
                                _: ...) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlURI {
    pub scheme: *mut std::os::raw::c_char,
    pub opaque: *mut std::os::raw::c_char,
    pub authority: *mut std::os::raw::c_char,
    pub server: *mut std::os::raw::c_char,
    pub user: *mut std::os::raw::c_char,
    pub port: std::os::raw::c_int,
    pub path: *mut std::os::raw::c_char,
    pub query: *mut std::os::raw::c_char,
    pub fragment: *mut std::os::raw::c_char,
    pub cleanup: std::os::raw::c_int,
    pub query_raw: *mut std::os::raw::c_char,
}
/* *
 * Summary: library of generic URI related routines
 * Description: library of generic URI related routines
 *              Implements RFC 2396
 *
 * Copy: See Copyright for the status of this software.
 *
 * Author: Daniel Veillard
 */
/* *
 * xmlURI:
 *
 * A parsed URI reference. This is a struct containing the various fields
 * as described in RFC 2396 but separated for further processing.
 *
 * Note: query is a deprecated field which is incorrectly unescaped.
 * query_raw takes precedence over query if the former is set.
 * See: http://mail.gnome.org/archives/xml/2007-April/thread.html#00127
 */
pub type xmlURI = _xmlURI;
pub type xmlURIPtr = *mut xmlURI;
unsafe extern "C" fn xmlURIErrMemory(mut extra: *const std::os::raw::c_char) {
    if !extra.is_null() {
        __xmlRaiseError(None, None, 0 as *mut std::os::raw::c_void,
                        0 as *mut std::os::raw::c_void, 0 as *mut std::os::raw::c_void,
                        XML_FROM_URI as std::os::raw::c_int,
                        XML_ERR_NO_MEMORY as std::os::raw::c_int, XML_ERR_FATAL,
                        0 as *const std::os::raw::c_char, 0 as std::os::raw::c_int, extra,
                        0 as *const std::os::raw::c_char, 0 as *const std::os::raw::c_char,
                        0 as std::os::raw::c_int, 0 as std::os::raw::c_int,
                        b"Memory allocation failed : %s\n\x00" as *const u8 as
                            *const std::os::raw::c_char, extra);
    } else {
        __xmlRaiseError(None, None, 0 as *mut std::os::raw::c_void,
                        0 as *mut std::os::raw::c_void, 0 as *mut std::os::raw::c_void,
                        XML_FROM_URI as std::os::raw::c_int,
                        XML_ERR_NO_MEMORY as std::os::raw::c_int, XML_ERR_FATAL,
                        0 as *const std::os::raw::c_char, 0 as std::os::raw::c_int,
                        0 as *const std::os::raw::c_char, 0 as *const std::os::raw::c_char,
                        0 as *const std::os::raw::c_char, 0 as std::os::raw::c_int,
                        0 as std::os::raw::c_int,
                        b"Memory allocation failed\n\x00" as *const u8 as
                            *const std::os::raw::c_char);
    };
}
/* *
 * xmlParse3986Scheme:
 * @uri:  pointer to an URI structure
 * @str:  pointer to the string to analyze
 *
 * Parse an URI scheme
 *
 * ALPHA *( ALPHA / DIGIT / "+" / "-" / "." )
 *
 * Returns 0 or the error code
 */
unsafe extern "C" fn xmlParse3986Scheme(mut uri: xmlURIPtr,
                                        mut str: *mut *const std::os::raw::c_char)
 -> std::os::raw::c_int {
    let mut cur: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    if str.is_null() { return -(1 as std::os::raw::c_int) }
    cur = *str;
    if !(*cur as std::os::raw::c_int >= 'a' as i32 &&
             *cur as std::os::raw::c_int <= 'z' as i32 ||
             *cur as std::os::raw::c_int >= 'A' as i32 &&
                 *cur as std::os::raw::c_int <= 'Z' as i32) {
        return 2 as std::os::raw::c_int
    }
    cur = cur.offset(1);
    while *cur as std::os::raw::c_int >= 'a' as i32 &&
              *cur as std::os::raw::c_int <= 'z' as i32 ||
              *cur as std::os::raw::c_int >= 'A' as i32 &&
                  *cur as std::os::raw::c_int <= 'Z' as i32 ||
              *cur as std::os::raw::c_int >= '0' as i32 &&
                  *cur as std::os::raw::c_int <= '9' as i32 ||
              *cur as std::os::raw::c_int == '+' as i32 ||
              *cur as std::os::raw::c_int == '-' as i32 ||
              *cur as std::os::raw::c_int == '.' as i32 {
        cur = cur.offset(1)
    }
    if !uri.is_null() {
        if !(*uri).scheme.is_null() {
            xmlFree.expect("non-null function pointer")((*uri).scheme as
                                                            *mut std::os::raw::c_void);
        }
        (*uri).scheme =
            xmlStrndup(*str as *const xmlChar,
                       cur.offset_from(*str) as std::os::raw::c_long as
                           std::os::raw::c_int) as *mut std::os::raw::c_char
    }
    *str = cur;
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlParse3986Fragment:
 * @uri:  pointer to an URI structure
 * @str:  pointer to the string to analyze
 *
 * Parse the query part of an URI
 *
 * fragment      = *( pchar / "/" / "?" )
 * NOTE: the strict syntax as defined by 3986 does not allow '[' and ']'
 *       in the fragment identifier but this is used very broadly for
 *       xpointer scheme selection, so we are allowing it here to not break
 *       for example all the DocBook processing chains.
 *
 * Returns 0 or the error code
 */
unsafe extern "C" fn xmlParse3986Fragment(mut uri: xmlURIPtr,
                                          mut str: *mut *const std::os::raw::c_char)
 -> std::os::raw::c_int {
    let mut cur: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    if str.is_null() { return -(1 as std::os::raw::c_int) }
    cur = *str;
    while *cur as std::os::raw::c_int >= 'a' as i32 &&
              *cur as std::os::raw::c_int <= 'z' as i32 ||
              *cur as std::os::raw::c_int >= 'A' as i32 &&
                  *cur as std::os::raw::c_int <= 'Z' as i32 ||
              *cur as std::os::raw::c_int >= '0' as i32 &&
                  *cur as std::os::raw::c_int <= '9' as i32 ||
              *cur as std::os::raw::c_int == '-' as i32 ||
              *cur as std::os::raw::c_int == '.' as i32 ||
              *cur as std::os::raw::c_int == '_' as i32 ||
              *cur as std::os::raw::c_int == '~' as i32 ||
              *cur as std::os::raw::c_int == '%' as i32 &&
                  (*cur.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int >=
                       '0' as i32 &&
                       *cur.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int
                           <= '9' as i32 ||
                       *cur.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int
                           >= 'a' as i32 &&
                           *cur.offset(1 as std::os::raw::c_int as isize) as
                               std::os::raw::c_int <= 'f' as i32 ||
                       *cur.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int
                           >= 'A' as i32 &&
                           *cur.offset(1 as std::os::raw::c_int as isize) as
                               std::os::raw::c_int <= 'F' as i32) &&
                  (*cur.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int >=
                       '0' as i32 &&
                       *cur.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int
                           <= '9' as i32 ||
                       *cur.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int
                           >= 'a' as i32 &&
                           *cur.offset(2 as std::os::raw::c_int as isize) as
                               std::os::raw::c_int <= 'f' as i32 ||
                       *cur.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int
                           >= 'A' as i32 &&
                           *cur.offset(2 as std::os::raw::c_int as isize) as
                               std::os::raw::c_int <= 'F' as i32) ||
              (*cur as std::os::raw::c_int == '!' as i32 ||
                   *cur as std::os::raw::c_int == '$' as i32 ||
                   *cur as std::os::raw::c_int == '&' as i32 ||
                   *cur as std::os::raw::c_int == '(' as i32 ||
                   *cur as std::os::raw::c_int == ')' as i32 ||
                   *cur as std::os::raw::c_int == '*' as i32 ||
                   *cur as std::os::raw::c_int == '+' as i32 ||
                   *cur as std::os::raw::c_int == ',' as i32 ||
                   *cur as std::os::raw::c_int == ';' as i32 ||
                   *cur as std::os::raw::c_int == '=' as i32 ||
                   *cur as std::os::raw::c_int == '\'' as i32) ||
              *cur as std::os::raw::c_int == ':' as i32 ||
              *cur as std::os::raw::c_int == '@' as i32 ||
              *cur as std::os::raw::c_int == '/' as i32 ||
              *cur as std::os::raw::c_int == '?' as i32 ||
              *cur as std::os::raw::c_int == '[' as i32 ||
              *cur as std::os::raw::c_int == ']' as i32 ||
              !uri.is_null() && (*uri).cleanup & 1 as std::os::raw::c_int != 0 &&
                  (*cur as std::os::raw::c_int == '{' as i32 ||
                       *cur as std::os::raw::c_int == '}' as i32 ||
                       *cur as std::os::raw::c_int == '|' as i32 ||
                       *cur as std::os::raw::c_int == '\\' as i32 ||
                       *cur as std::os::raw::c_int == '^' as i32 ||
                       *cur as std::os::raw::c_int == '[' as i32 ||
                       *cur as std::os::raw::c_int == ']' as i32 ||
                       *cur as std::os::raw::c_int == '`' as i32) {
        if *cur as std::os::raw::c_int == '%' as i32 {
            cur = cur.offset(3 as std::os::raw::c_int as isize)
        } else { cur = cur.offset(1) };
    }
    if !uri.is_null() {
        if !(*uri).fragment.is_null() {
            xmlFree.expect("non-null function pointer")((*uri).fragment as
                                                            *mut std::os::raw::c_void);
        }
        if (*uri).cleanup & 2 as std::os::raw::c_int != 0 {
            (*uri).fragment =
                xmlStrndup(*str as *const xmlChar,
                           cur.offset_from(*str) as std::os::raw::c_long as
                               std::os::raw::c_int) as *mut std::os::raw::c_char
        } else {
            (*uri).fragment =
                xmlURIUnescapeString(*str,
                                     cur.offset_from(*str) as
                                         std::os::raw::c_long as std::os::raw::c_int,
                                     0 as *mut std::os::raw::c_char)
        }
    }
    *str = cur;
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlParse3986Query:
 * @uri:  pointer to an URI structure
 * @str:  pointer to the string to analyze
 *
 * Parse the query part of an URI
 *
 * query = *uric
 *
 * Returns 0 or the error code
 */
unsafe extern "C" fn xmlParse3986Query(mut uri: xmlURIPtr,
                                       mut str: *mut *const std::os::raw::c_char)
 -> std::os::raw::c_int {
    let mut cur: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    if str.is_null() { return -(1 as std::os::raw::c_int) }
    cur = *str;
    while *cur as std::os::raw::c_int >= 'a' as i32 &&
              *cur as std::os::raw::c_int <= 'z' as i32 ||
              *cur as std::os::raw::c_int >= 'A' as i32 &&
                  *cur as std::os::raw::c_int <= 'Z' as i32 ||
              *cur as std::os::raw::c_int >= '0' as i32 &&
                  *cur as std::os::raw::c_int <= '9' as i32 ||
              *cur as std::os::raw::c_int == '-' as i32 ||
              *cur as std::os::raw::c_int == '.' as i32 ||
              *cur as std::os::raw::c_int == '_' as i32 ||
              *cur as std::os::raw::c_int == '~' as i32 ||
              *cur as std::os::raw::c_int == '%' as i32 &&
                  (*cur.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int >=
                       '0' as i32 &&
                       *cur.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int
                           <= '9' as i32 ||
                       *cur.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int
                           >= 'a' as i32 &&
                           *cur.offset(1 as std::os::raw::c_int as isize) as
                               std::os::raw::c_int <= 'f' as i32 ||
                       *cur.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int
                           >= 'A' as i32 &&
                           *cur.offset(1 as std::os::raw::c_int as isize) as
                               std::os::raw::c_int <= 'F' as i32) &&
                  (*cur.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int >=
                       '0' as i32 &&
                       *cur.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int
                           <= '9' as i32 ||
                       *cur.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int
                           >= 'a' as i32 &&
                           *cur.offset(2 as std::os::raw::c_int as isize) as
                               std::os::raw::c_int <= 'f' as i32 ||
                       *cur.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int
                           >= 'A' as i32 &&
                           *cur.offset(2 as std::os::raw::c_int as isize) as
                               std::os::raw::c_int <= 'F' as i32) ||
              (*cur as std::os::raw::c_int == '!' as i32 ||
                   *cur as std::os::raw::c_int == '$' as i32 ||
                   *cur as std::os::raw::c_int == '&' as i32 ||
                   *cur as std::os::raw::c_int == '(' as i32 ||
                   *cur as std::os::raw::c_int == ')' as i32 ||
                   *cur as std::os::raw::c_int == '*' as i32 ||
                   *cur as std::os::raw::c_int == '+' as i32 ||
                   *cur as std::os::raw::c_int == ',' as i32 ||
                   *cur as std::os::raw::c_int == ';' as i32 ||
                   *cur as std::os::raw::c_int == '=' as i32 ||
                   *cur as std::os::raw::c_int == '\'' as i32) ||
              *cur as std::os::raw::c_int == ':' as i32 ||
              *cur as std::os::raw::c_int == '@' as i32 ||
              *cur as std::os::raw::c_int == '/' as i32 ||
              *cur as std::os::raw::c_int == '?' as i32 ||
              !uri.is_null() && (*uri).cleanup & 1 as std::os::raw::c_int != 0 &&
                  (*cur as std::os::raw::c_int == '{' as i32 ||
                       *cur as std::os::raw::c_int == '}' as i32 ||
                       *cur as std::os::raw::c_int == '|' as i32 ||
                       *cur as std::os::raw::c_int == '\\' as i32 ||
                       *cur as std::os::raw::c_int == '^' as i32 ||
                       *cur as std::os::raw::c_int == '[' as i32 ||
                       *cur as std::os::raw::c_int == ']' as i32 ||
                       *cur as std::os::raw::c_int == '`' as i32) {
        if *cur as std::os::raw::c_int == '%' as i32 {
            cur = cur.offset(3 as std::os::raw::c_int as isize)
        } else { cur = cur.offset(1) };
    }
    if !uri.is_null() {
        if !(*uri).query.is_null() {
            xmlFree.expect("non-null function pointer")((*uri).query as
                                                            *mut std::os::raw::c_void);
        }
        if (*uri).cleanup & 2 as std::os::raw::c_int != 0 {
            (*uri).query =
                xmlStrndup(*str as *const xmlChar,
                           cur.offset_from(*str) as std::os::raw::c_long as
                               std::os::raw::c_int) as *mut std::os::raw::c_char
        } else {
            (*uri).query =
                xmlURIUnescapeString(*str,
                                     cur.offset_from(*str) as
                                         std::os::raw::c_long as std::os::raw::c_int,
                                     0 as *mut std::os::raw::c_char)
        }
        /* Save the raw bytes of the query as well.
	 * See: http://mail.gnome.org/archives/xml/2007-April/thread.html#00114
	 */
        if !(*uri).query_raw.is_null() {
            xmlFree.expect("non-null function pointer")((*uri).query_raw as
                                                            *mut std::os::raw::c_void);
        }
        (*uri).query_raw =
            xmlStrndup(*str as *const xmlChar,
                       cur.offset_from(*str) as std::os::raw::c_long as
                           std::os::raw::c_int) as *mut std::os::raw::c_char
    }
    *str = cur;
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlParse3986Port:
 * @uri:  pointer to an URI structure
 * @str:  the string to analyze
 *
 * Parse a port part and fills in the appropriate fields
 * of the @uri structure
 *
 * port          = *DIGIT
 *
 * Returns 0 or the error code
 */
unsafe extern "C" fn xmlParse3986Port(mut uri: xmlURIPtr,
                                      mut str: *mut *const std::os::raw::c_char)
 -> std::os::raw::c_int {
    let mut cur: *const std::os::raw::c_char =
        *str; /* unsigned for defined overflow behavior */
    let mut port: std::os::raw::c_uint =
        0 as std::os::raw::c_int as std::os::raw::c_uint; /* port value modulo INT_MAX+1 */
    if *cur as std::os::raw::c_int >= '0' as i32 && *cur as std::os::raw::c_int <= '9' as i32
       {
        while *cur as std::os::raw::c_int >= '0' as i32 &&
                  *cur as std::os::raw::c_int <= '9' as i32 {
            port =
                port.wrapping_mul(10 as std::os::raw::c_int as
                                      std::os::raw::c_uint).wrapping_add((*cur as
                                                                      std::os::raw::c_int
                                                                      -
                                                                      '0' as
                                                                          i32)
                                                                     as
                                                                     std::os::raw::c_uint);
            cur = cur.offset(1)
        }
        if !uri.is_null() {
            (*uri).port =
                (port & 2147483647 as std::os::raw::c_int as std::os::raw::c_uint) as
                    std::os::raw::c_int
        }
        *str = cur;
        return 0 as std::os::raw::c_int
    }
    return 1 as std::os::raw::c_int;
}
/* *
 * xmlParse3986Userinfo:
 * @uri:  pointer to an URI structure
 * @str:  the string to analyze
 *
 * Parse an user informations part and fills in the appropriate fields
 * of the @uri structure
 *
 * userinfo      = *( unreserved / pct-encoded / sub-delims / ":" )
 *
 * Returns 0 or the error code
 */
unsafe extern "C" fn xmlParse3986Userinfo(mut uri: xmlURIPtr,
                                          mut str: *mut *const std::os::raw::c_char)
 -> std::os::raw::c_int {
    let mut cur: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    cur = *str;
    while *cur as std::os::raw::c_int >= 'a' as i32 &&
              *cur as std::os::raw::c_int <= 'z' as i32 ||
              *cur as std::os::raw::c_int >= 'A' as i32 &&
                  *cur as std::os::raw::c_int <= 'Z' as i32 ||
              *cur as std::os::raw::c_int >= '0' as i32 &&
                  *cur as std::os::raw::c_int <= '9' as i32 ||
              *cur as std::os::raw::c_int == '-' as i32 ||
              *cur as std::os::raw::c_int == '.' as i32 ||
              *cur as std::os::raw::c_int == '_' as i32 ||
              *cur as std::os::raw::c_int == '~' as i32 ||
              *cur as std::os::raw::c_int == '%' as i32 &&
                  (*cur.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int >=
                       '0' as i32 &&
                       *cur.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int
                           <= '9' as i32 ||
                       *cur.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int
                           >= 'a' as i32 &&
                           *cur.offset(1 as std::os::raw::c_int as isize) as
                               std::os::raw::c_int <= 'f' as i32 ||
                       *cur.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int
                           >= 'A' as i32 &&
                           *cur.offset(1 as std::os::raw::c_int as isize) as
                               std::os::raw::c_int <= 'F' as i32) &&
                  (*cur.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int >=
                       '0' as i32 &&
                       *cur.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int
                           <= '9' as i32 ||
                       *cur.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int
                           >= 'a' as i32 &&
                           *cur.offset(2 as std::os::raw::c_int as isize) as
                               std::os::raw::c_int <= 'f' as i32 ||
                       *cur.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int
                           >= 'A' as i32 &&
                           *cur.offset(2 as std::os::raw::c_int as isize) as
                               std::os::raw::c_int <= 'F' as i32) ||
              (*cur as std::os::raw::c_int == '!' as i32 ||
                   *cur as std::os::raw::c_int == '$' as i32 ||
                   *cur as std::os::raw::c_int == '&' as i32 ||
                   *cur as std::os::raw::c_int == '(' as i32 ||
                   *cur as std::os::raw::c_int == ')' as i32 ||
                   *cur as std::os::raw::c_int == '*' as i32 ||
                   *cur as std::os::raw::c_int == '+' as i32 ||
                   *cur as std::os::raw::c_int == ',' as i32 ||
                   *cur as std::os::raw::c_int == ';' as i32 ||
                   *cur as std::os::raw::c_int == '=' as i32 ||
                   *cur as std::os::raw::c_int == '\'' as i32) ||
              *cur as std::os::raw::c_int == ':' as i32 {
        if *cur as std::os::raw::c_int == '%' as i32 {
            cur = cur.offset(3 as std::os::raw::c_int as isize)
        } else { cur = cur.offset(1) };
    }
    if *cur as std::os::raw::c_int == '@' as i32 {
        if !uri.is_null() {
            if !(*uri).user.is_null() {
                xmlFree.expect("non-null function pointer")((*uri).user as
                                                                *mut std::os::raw::c_void);
            }
            if (*uri).cleanup & 2 as std::os::raw::c_int != 0 {
                (*uri).user =
                    xmlStrndup(*str as *const xmlChar,
                               cur.offset_from(*str) as std::os::raw::c_long
                                   as std::os::raw::c_int) as *mut std::os::raw::c_char
            } else {
                (*uri).user =
                    xmlURIUnescapeString(*str,
                                         cur.offset_from(*str) as
                                             std::os::raw::c_long as std::os::raw::c_int,
                                         0 as *mut std::os::raw::c_char)
            }
        }
        *str = cur;
        return 0 as std::os::raw::c_int
    }
    return 1 as std::os::raw::c_int;
}
/* *
 * xmlParse3986DecOctet:
 * @str:  the string to analyze
 *
 *    dec-octet     = DIGIT                 ; 0-9
 *                  / %x31-39 DIGIT         ; 10-99
 *                  / "1" 2DIGIT            ; 100-199
 *                  / "2" %x30-34 DIGIT     ; 200-249
 *                  / "25" %x30-35          ; 250-255
 *
 * Skip a dec-octet.
 *
 * Returns 0 if found and skipped, 1 otherwise
 */
unsafe extern "C" fn xmlParse3986DecOctet(mut str: *mut *const std::os::raw::c_char)
 -> std::os::raw::c_int {
    let mut cur: *const std::os::raw::c_char = *str;
    if !(*cur as std::os::raw::c_int >= '0' as i32 &&
             *cur as std::os::raw::c_int <= '9' as i32) {
        return 1 as std::os::raw::c_int
    }
    if !(*cur.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int >= '0' as i32
             &&
             *cur.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int <=
                 '9' as i32) {
        cur = cur.offset(1)
    } else if *cur as std::os::raw::c_int != '0' as i32 &&
                  (*cur.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int >=
                       '0' as i32 &&
                       *cur.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int
                           <= '9' as i32) &&
                  !(*cur.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int >=
                        '0' as i32 &&
                        *cur.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int
                            <= '9' as i32) {
        cur = cur.offset(2 as std::os::raw::c_int as isize)
    } else if *cur as std::os::raw::c_int == '1' as i32 &&
                  (*cur.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int >=
                       '0' as i32 &&
                       *cur.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int
                           <= '9' as i32) &&
                  (*cur.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int >=
                       '0' as i32 &&
                       *cur.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int
                           <= '9' as i32) {
        cur = cur.offset(3 as std::os::raw::c_int as isize)
    } else if *cur as std::os::raw::c_int == '2' as i32 &&
                  *cur.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int >=
                      '0' as i32 &&
                  *cur.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int <=
                      '4' as i32 &&
                  (*cur.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int >=
                       '0' as i32 &&
                       *cur.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int
                           <= '9' as i32) {
        cur = cur.offset(3 as std::os::raw::c_int as isize)
    } else if *cur as std::os::raw::c_int == '2' as i32 &&
                  *cur.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                      '5' as i32 &&
                  *cur.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int >=
                      '0' as i32 &&
                  *cur.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int <=
                      '5' as i32 {
        cur = cur.offset(3 as std::os::raw::c_int as isize)
    } else { return 1 as std::os::raw::c_int }
    *str = cur;
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlParse3986Host:
 * @uri:  pointer to an URI structure
 * @str:  the string to analyze
 *
 * Parse an host part and fills in the appropriate fields
 * of the @uri structure
 *
 * host          = IP-literal / IPv4address / reg-name
 * IP-literal    = "[" ( IPv6address / IPvFuture  ) "]"
 * IPv4address   = dec-octet "." dec-octet "." dec-octet "." dec-octet
 * reg-name      = *( unreserved / pct-encoded / sub-delims )
 *
 * Returns 0 or the error code
 */
unsafe extern "C" fn xmlParse3986Host(mut uri: xmlURIPtr,
                                      mut str: *mut *const std::os::raw::c_char)
 -> std::os::raw::c_int {
    let mut current_block: u64;
    let mut cur: *const std::os::raw::c_char = *str;
    let mut host: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    host = cur;
    /*
     * IPv6 and future adressing scheme are enclosed between brackets
     */
    if *cur as std::os::raw::c_int == '[' as i32 {
        cur = cur.offset(1);
        while *cur as std::os::raw::c_int != ']' as i32 &&
                  *cur as std::os::raw::c_int != 0 as std::os::raw::c_int {
            cur = cur.offset(1)
        }
        if *cur as std::os::raw::c_int != ']' as i32 { return 1 as std::os::raw::c_int }
        cur = cur.offset(1)
    } else {
        /*
     * try to parse an IPv4
     */
        if *cur as std::os::raw::c_int >= '0' as i32 &&
               *cur as std::os::raw::c_int <= '9' as i32 {
            if xmlParse3986DecOctet(&mut cur) != 0 as std::os::raw::c_int {
                current_block = 14092497436735950494;
            } else if *cur as std::os::raw::c_int != '.' as i32 {
                current_block = 14092497436735950494;
            } else {
                cur = cur.offset(1);
                if xmlParse3986DecOctet(&mut cur) != 0 as std::os::raw::c_int {
                    current_block = 14092497436735950494;
                } else if *cur as std::os::raw::c_int != '.' as i32 {
                    current_block = 14092497436735950494;
                } else if xmlParse3986DecOctet(&mut cur) != 0 as std::os::raw::c_int {
                    current_block = 14092497436735950494;
                } else if *cur as std::os::raw::c_int != '.' as i32 {
                    current_block = 14092497436735950494;
                } else if xmlParse3986DecOctet(&mut cur) != 0 as std::os::raw::c_int {
                    current_block = 14092497436735950494;
                } else { current_block = 25117249615787885; }
            }
            match current_block {
                25117249615787885 => { }
                _ => { cur = *str; current_block = 5601891728916014340; }
            }
        } else { current_block = 5601891728916014340; }
        match current_block {
            25117249615787885 => { }
            _ => {
                /*
     * then this should be a hostname which can be empty
     */
                while *cur as std::os::raw::c_int >= 'a' as i32 &&
                          *cur as std::os::raw::c_int <= 'z' as i32 ||
                          *cur as std::os::raw::c_int >= 'A' as i32 &&
                              *cur as std::os::raw::c_int <= 'Z' as i32 ||
                          *cur as std::os::raw::c_int >= '0' as i32 &&
                              *cur as std::os::raw::c_int <= '9' as i32 ||
                          *cur as std::os::raw::c_int == '-' as i32 ||
                          *cur as std::os::raw::c_int == '.' as i32 ||
                          *cur as std::os::raw::c_int == '_' as i32 ||
                          *cur as std::os::raw::c_int == '~' as i32 ||
                          *cur as std::os::raw::c_int == '%' as i32 &&
                              (*cur.offset(1 as std::os::raw::c_int as isize) as
                                   std::os::raw::c_int >= '0' as i32 &&
                                   *cur.offset(1 as std::os::raw::c_int as isize) as
                                       std::os::raw::c_int <= '9' as i32 ||
                                   *cur.offset(1 as std::os::raw::c_int as isize) as
                                       std::os::raw::c_int >= 'a' as i32 &&
                                       *cur.offset(1 as std::os::raw::c_int as isize)
                                           as std::os::raw::c_int <= 'f' as i32 ||
                                   *cur.offset(1 as std::os::raw::c_int as isize) as
                                       std::os::raw::c_int >= 'A' as i32 &&
                                       *cur.offset(1 as std::os::raw::c_int as isize)
                                           as std::os::raw::c_int <= 'F' as i32) &&
                              (*cur.offset(2 as std::os::raw::c_int as isize) as
                                   std::os::raw::c_int >= '0' as i32 &&
                                   *cur.offset(2 as std::os::raw::c_int as isize) as
                                       std::os::raw::c_int <= '9' as i32 ||
                                   *cur.offset(2 as std::os::raw::c_int as isize) as
                                       std::os::raw::c_int >= 'a' as i32 &&
                                       *cur.offset(2 as std::os::raw::c_int as isize)
                                           as std::os::raw::c_int <= 'f' as i32 ||
                                   *cur.offset(2 as std::os::raw::c_int as isize) as
                                       std::os::raw::c_int >= 'A' as i32 &&
                                       *cur.offset(2 as std::os::raw::c_int as isize)
                                           as std::os::raw::c_int <= 'F' as i32) ||
                          (*cur as std::os::raw::c_int == '!' as i32 ||
                               *cur as std::os::raw::c_int == '$' as i32 ||
                               *cur as std::os::raw::c_int == '&' as i32 ||
                               *cur as std::os::raw::c_int == '(' as i32 ||
                               *cur as std::os::raw::c_int == ')' as i32 ||
                               *cur as std::os::raw::c_int == '*' as i32 ||
                               *cur as std::os::raw::c_int == '+' as i32 ||
                               *cur as std::os::raw::c_int == ',' as i32 ||
                               *cur as std::os::raw::c_int == ';' as i32 ||
                               *cur as std::os::raw::c_int == '=' as i32 ||
                               *cur as std::os::raw::c_int == '\'' as i32) {
                    if *cur as std::os::raw::c_int == '%' as i32 {
                        cur = cur.offset(3 as std::os::raw::c_int as isize)
                    } else { cur = cur.offset(1) };
                }
            }
        }
    }
    if !uri.is_null() {
        if !(*uri).authority.is_null() {
            xmlFree.expect("non-null function pointer")((*uri).authority as
                                                            *mut std::os::raw::c_void);
        }
        (*uri).authority = 0 as *mut std::os::raw::c_char;
        if !(*uri).server.is_null() {
            xmlFree.expect("non-null function pointer")((*uri).server as
                                                            *mut std::os::raw::c_void);
        }
        if cur != host {
            if (*uri).cleanup & 2 as std::os::raw::c_int != 0 {
                (*uri).server =
                    xmlStrndup(host as *const xmlChar,
                               cur.offset_from(host) as std::os::raw::c_long
                                   as std::os::raw::c_int) as *mut std::os::raw::c_char
            } else {
                (*uri).server =
                    xmlURIUnescapeString(host,
                                         cur.offset_from(host) as
                                             std::os::raw::c_long as std::os::raw::c_int,
                                         0 as *mut std::os::raw::c_char)
            }
        } else { (*uri).server = 0 as *mut std::os::raw::c_char }
    }
    *str = cur;
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlParse3986Authority:
 * @uri:  pointer to an URI structure
 * @str:  the string to analyze
 *
 * Parse an authority part and fills in the appropriate fields
 * of the @uri structure
 *
 * authority     = [ userinfo "@" ] host [ ":" port ]
 *
 * Returns 0 or the error code
 */
unsafe extern "C" fn xmlParse3986Authority(mut uri: xmlURIPtr,
                                           mut str: *mut *const std::os::raw::c_char)
 -> std::os::raw::c_int {
    let mut cur: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut ret: std::os::raw::c_int = 0;
    cur = *str;
    /*
     * try to parse an userinfo and check for the trailing @
     */
    ret = xmlParse3986Userinfo(uri, &mut cur);
    if ret != 0 as std::os::raw::c_int || *cur as std::os::raw::c_int != '@' as i32 {
        cur = *str
    } else { cur = cur.offset(1) }
    ret = xmlParse3986Host(uri, &mut cur);
    if ret != 0 as std::os::raw::c_int { return ret }
    if *cur as std::os::raw::c_int == ':' as i32 {
        cur = cur.offset(1);
        ret = xmlParse3986Port(uri, &mut cur);
        if ret != 0 as std::os::raw::c_int { return ret }
    }
    *str = cur;
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlParse3986Segment:
 * @str:  the string to analyze
 * @forbid: an optional forbidden character
 * @empty: allow an empty segment
 *
 * Parse a segment and fills in the appropriate fields
 * of the @uri structure
 *
 * segment       = *pchar
 * segment-nz    = 1*pchar
 * segment-nz-nc = 1*( unreserved / pct-encoded / sub-delims / "@" )
 *               ; non-zero-length segment without any colon ":"
 *
 * Returns 0 or the error code
 */
unsafe extern "C" fn xmlParse3986Segment(mut str: *mut *const std::os::raw::c_char,
                                         mut forbid: std::os::raw::c_char,
                                         mut empty: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut cur: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    cur = *str;
    if !(*cur as std::os::raw::c_int >= 'a' as i32 &&
             *cur as std::os::raw::c_int <= 'z' as i32 ||
             *cur as std::os::raw::c_int >= 'A' as i32 &&
                 *cur as std::os::raw::c_int <= 'Z' as i32 ||
             *cur as std::os::raw::c_int >= '0' as i32 &&
                 *cur as std::os::raw::c_int <= '9' as i32 ||
             *cur as std::os::raw::c_int == '-' as i32 ||
             *cur as std::os::raw::c_int == '.' as i32 ||
             *cur as std::os::raw::c_int == '_' as i32 ||
             *cur as std::os::raw::c_int == '~' as i32 ||
             *cur as std::os::raw::c_int == '%' as i32 &&
                 (*cur.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int >=
                      '0' as i32 &&
                      *cur.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int <=
                          '9' as i32 ||
                      *cur.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int >=
                          'a' as i32 &&
                          *cur.offset(1 as std::os::raw::c_int as isize) as
                              std::os::raw::c_int <= 'f' as i32 ||
                      *cur.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int >=
                          'A' as i32 &&
                          *cur.offset(1 as std::os::raw::c_int as isize) as
                              std::os::raw::c_int <= 'F' as i32) &&
                 (*cur.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int >=
                      '0' as i32 &&
                      *cur.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int <=
                          '9' as i32 ||
                      *cur.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int >=
                          'a' as i32 &&
                          *cur.offset(2 as std::os::raw::c_int as isize) as
                              std::os::raw::c_int <= 'f' as i32 ||
                      *cur.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int >=
                          'A' as i32 &&
                          *cur.offset(2 as std::os::raw::c_int as isize) as
                              std::os::raw::c_int <= 'F' as i32) ||
             (*cur as std::os::raw::c_int == '!' as i32 ||
                  *cur as std::os::raw::c_int == '$' as i32 ||
                  *cur as std::os::raw::c_int == '&' as i32 ||
                  *cur as std::os::raw::c_int == '(' as i32 ||
                  *cur as std::os::raw::c_int == ')' as i32 ||
                  *cur as std::os::raw::c_int == '*' as i32 ||
                  *cur as std::os::raw::c_int == '+' as i32 ||
                  *cur as std::os::raw::c_int == ',' as i32 ||
                  *cur as std::os::raw::c_int == ';' as i32 ||
                  *cur as std::os::raw::c_int == '=' as i32 ||
                  *cur as std::os::raw::c_int == '\'' as i32) ||
             *cur as std::os::raw::c_int == ':' as i32 ||
             *cur as std::os::raw::c_int == '@' as i32) {
        if empty != 0 { return 0 as std::os::raw::c_int }
        return 1 as std::os::raw::c_int
    }
    while (*cur as std::os::raw::c_int >= 'a' as i32 &&
               *cur as std::os::raw::c_int <= 'z' as i32 ||
               *cur as std::os::raw::c_int >= 'A' as i32 &&
                   *cur as std::os::raw::c_int <= 'Z' as i32 ||
               *cur as std::os::raw::c_int >= '0' as i32 &&
                   *cur as std::os::raw::c_int <= '9' as i32 ||
               *cur as std::os::raw::c_int == '-' as i32 ||
               *cur as std::os::raw::c_int == '.' as i32 ||
               *cur as std::os::raw::c_int == '_' as i32 ||
               *cur as std::os::raw::c_int == '~' as i32 ||
               *cur as std::os::raw::c_int == '%' as i32 &&
                   (*cur.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int >=
                        '0' as i32 &&
                        *cur.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int
                            <= '9' as i32 ||
                        *cur.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int
                            >= 'a' as i32 &&
                            *cur.offset(1 as std::os::raw::c_int as isize) as
                                std::os::raw::c_int <= 'f' as i32 ||
                        *cur.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int
                            >= 'A' as i32 &&
                            *cur.offset(1 as std::os::raw::c_int as isize) as
                                std::os::raw::c_int <= 'F' as i32) &&
                   (*cur.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int >=
                        '0' as i32 &&
                        *cur.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int
                            <= '9' as i32 ||
                        *cur.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int
                            >= 'a' as i32 &&
                            *cur.offset(2 as std::os::raw::c_int as isize) as
                                std::os::raw::c_int <= 'f' as i32 ||
                        *cur.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int
                            >= 'A' as i32 &&
                            *cur.offset(2 as std::os::raw::c_int as isize) as
                                std::os::raw::c_int <= 'F' as i32) ||
               (*cur as std::os::raw::c_int == '!' as i32 ||
                    *cur as std::os::raw::c_int == '$' as i32 ||
                    *cur as std::os::raw::c_int == '&' as i32 ||
                    *cur as std::os::raw::c_int == '(' as i32 ||
                    *cur as std::os::raw::c_int == ')' as i32 ||
                    *cur as std::os::raw::c_int == '*' as i32 ||
                    *cur as std::os::raw::c_int == '+' as i32 ||
                    *cur as std::os::raw::c_int == ',' as i32 ||
                    *cur as std::os::raw::c_int == ';' as i32 ||
                    *cur as std::os::raw::c_int == '=' as i32 ||
                    *cur as std::os::raw::c_int == '\'' as i32) ||
               *cur as std::os::raw::c_int == ':' as i32 ||
               *cur as std::os::raw::c_int == '@' as i32) &&
              *cur as std::os::raw::c_int != forbid as std::os::raw::c_int {
        if *cur as std::os::raw::c_int == '%' as i32 {
            cur = cur.offset(3 as std::os::raw::c_int as isize)
        } else { cur = cur.offset(1) };
    }
    *str = cur;
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlParse3986PathAbEmpty:
 * @uri:  pointer to an URI structure
 * @str:  the string to analyze
 *
 * Parse an path absolute or empty and fills in the appropriate fields
 * of the @uri structure
 *
 * path-abempty  = *( "/" segment )
 *
 * Returns 0 or the error code
 */
unsafe extern "C" fn xmlParse3986PathAbEmpty(mut uri: xmlURIPtr,
                                             mut str:
                                                 *mut *const std::os::raw::c_char)
 -> std::os::raw::c_int {
    let mut cur: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut ret: std::os::raw::c_int = 0;
    cur = *str;
    while *cur as std::os::raw::c_int == '/' as i32 {
        cur = cur.offset(1);
        ret =
            xmlParse3986Segment(&mut cur, 0 as std::os::raw::c_int as std::os::raw::c_char,
                                1 as std::os::raw::c_int);
        if ret != 0 as std::os::raw::c_int { return ret }
    }
    if !uri.is_null() {
        if !(*uri).path.is_null() {
            xmlFree.expect("non-null function pointer")((*uri).path as
                                                            *mut std::os::raw::c_void);
        }
        if *str != cur {
            if (*uri).cleanup & 2 as std::os::raw::c_int != 0 {
                (*uri).path =
                    xmlStrndup(*str as *const xmlChar,
                               cur.offset_from(*str) as std::os::raw::c_long
                                   as std::os::raw::c_int) as *mut std::os::raw::c_char
            } else {
                (*uri).path =
                    xmlURIUnescapeString(*str,
                                         cur.offset_from(*str) as
                                             std::os::raw::c_long as std::os::raw::c_int,
                                         0 as *mut std::os::raw::c_char)
            }
        } else { (*uri).path = 0 as *mut std::os::raw::c_char }
    }
    *str = cur;
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlParse3986PathAbsolute:
 * @uri:  pointer to an URI structure
 * @str:  the string to analyze
 *
 * Parse an path absolute and fills in the appropriate fields
 * of the @uri structure
 *
 * path-absolute = "/" [ segment-nz *( "/" segment ) ]
 *
 * Returns 0 or the error code
 */
unsafe extern "C" fn xmlParse3986PathAbsolute(mut uri: xmlURIPtr,
                                              mut str:
                                                  *mut *const std::os::raw::c_char)
 -> std::os::raw::c_int {
    let mut cur: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut ret: std::os::raw::c_int = 0;
    cur = *str;
    if *cur as std::os::raw::c_int != '/' as i32 { return 1 as std::os::raw::c_int }
    cur = cur.offset(1);
    ret =
        xmlParse3986Segment(&mut cur, 0 as std::os::raw::c_int as std::os::raw::c_char,
                            0 as std::os::raw::c_int);
    if ret == 0 as std::os::raw::c_int {
        while *cur as std::os::raw::c_int == '/' as i32 {
            cur = cur.offset(1);
            ret =
                xmlParse3986Segment(&mut cur,
                                    0 as std::os::raw::c_int as std::os::raw::c_char,
                                    1 as std::os::raw::c_int);
            if ret != 0 as std::os::raw::c_int { return ret }
        }
    }
    if !uri.is_null() {
        if !(*uri).path.is_null() {
            xmlFree.expect("non-null function pointer")((*uri).path as
                                                            *mut std::os::raw::c_void);
        }
        if cur != *str {
            if (*uri).cleanup & 2 as std::os::raw::c_int != 0 {
                (*uri).path =
                    xmlStrndup(*str as *const xmlChar,
                               cur.offset_from(*str) as std::os::raw::c_long
                                   as std::os::raw::c_int) as *mut std::os::raw::c_char
            } else {
                (*uri).path =
                    xmlURIUnescapeString(*str,
                                         cur.offset_from(*str) as
                                             std::os::raw::c_long as std::os::raw::c_int,
                                         0 as *mut std::os::raw::c_char)
            }
        } else { (*uri).path = 0 as *mut std::os::raw::c_char }
    }
    *str = cur;
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlParse3986PathRootless:
 * @uri:  pointer to an URI structure
 * @str:  the string to analyze
 *
 * Parse an path without root and fills in the appropriate fields
 * of the @uri structure
 *
 * path-rootless = segment-nz *( "/" segment )
 *
 * Returns 0 or the error code
 */
unsafe extern "C" fn xmlParse3986PathRootless(mut uri: xmlURIPtr,
                                              mut str:
                                                  *mut *const std::os::raw::c_char)
 -> std::os::raw::c_int {
    let mut cur: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut ret: std::os::raw::c_int = 0;
    cur = *str;
    ret =
        xmlParse3986Segment(&mut cur, 0 as std::os::raw::c_int as std::os::raw::c_char,
                            0 as std::os::raw::c_int);
    if ret != 0 as std::os::raw::c_int { return ret }
    while *cur as std::os::raw::c_int == '/' as i32 {
        cur = cur.offset(1);
        ret =
            xmlParse3986Segment(&mut cur, 0 as std::os::raw::c_int as std::os::raw::c_char,
                                1 as std::os::raw::c_int);
        if ret != 0 as std::os::raw::c_int { return ret }
    }
    if !uri.is_null() {
        if !(*uri).path.is_null() {
            xmlFree.expect("non-null function pointer")((*uri).path as
                                                            *mut std::os::raw::c_void);
        }
        if cur != *str {
            if (*uri).cleanup & 2 as std::os::raw::c_int != 0 {
                (*uri).path =
                    xmlStrndup(*str as *const xmlChar,
                               cur.offset_from(*str) as std::os::raw::c_long
                                   as std::os::raw::c_int) as *mut std::os::raw::c_char
            } else {
                (*uri).path =
                    xmlURIUnescapeString(*str,
                                         cur.offset_from(*str) as
                                             std::os::raw::c_long as std::os::raw::c_int,
                                         0 as *mut std::os::raw::c_char)
            }
        } else { (*uri).path = 0 as *mut std::os::raw::c_char }
    }
    *str = cur;
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlParse3986PathNoScheme:
 * @uri:  pointer to an URI structure
 * @str:  the string to analyze
 *
 * Parse an path which is not a scheme and fills in the appropriate fields
 * of the @uri structure
 *
 * path-noscheme = segment-nz-nc *( "/" segment )
 *
 * Returns 0 or the error code
 */
unsafe extern "C" fn xmlParse3986PathNoScheme(mut uri: xmlURIPtr,
                                              mut str:
                                                  *mut *const std::os::raw::c_char)
 -> std::os::raw::c_int {
    let mut cur: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut ret: std::os::raw::c_int = 0;
    cur = *str;
    ret =
        xmlParse3986Segment(&mut cur, ':' as i32 as std::os::raw::c_char,
                            0 as std::os::raw::c_int);
    if ret != 0 as std::os::raw::c_int { return ret }
    while *cur as std::os::raw::c_int == '/' as i32 {
        cur = cur.offset(1);
        ret =
            xmlParse3986Segment(&mut cur, 0 as std::os::raw::c_int as std::os::raw::c_char,
                                1 as std::os::raw::c_int);
        if ret != 0 as std::os::raw::c_int { return ret }
    }
    if !uri.is_null() {
        if !(*uri).path.is_null() {
            xmlFree.expect("non-null function pointer")((*uri).path as
                                                            *mut std::os::raw::c_void);
        }
        if cur != *str {
            if (*uri).cleanup & 2 as std::os::raw::c_int != 0 {
                (*uri).path =
                    xmlStrndup(*str as *const xmlChar,
                               cur.offset_from(*str) as std::os::raw::c_long
                                   as std::os::raw::c_int) as *mut std::os::raw::c_char
            } else {
                (*uri).path =
                    xmlURIUnescapeString(*str,
                                         cur.offset_from(*str) as
                                             std::os::raw::c_long as std::os::raw::c_int,
                                         0 as *mut std::os::raw::c_char)
            }
        } else { (*uri).path = 0 as *mut std::os::raw::c_char }
    }
    *str = cur;
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlParse3986HierPart:
 * @uri:  pointer to an URI structure
 * @str:  the string to analyze
 *
 * Parse an hierarchical part and fills in the appropriate fields
 * of the @uri structure
 *
 * hier-part     = "//" authority path-abempty
 *                / path-absolute
 *                / path-rootless
 *                / path-empty
 *
 * Returns 0 or the error code
 */
unsafe extern "C" fn xmlParse3986HierPart(mut uri: xmlURIPtr,
                                          mut str: *mut *const std::os::raw::c_char)
 -> std::os::raw::c_int {
    let mut cur: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut ret: std::os::raw::c_int = 0;
    cur = *str;
    if *cur as std::os::raw::c_int == '/' as i32 &&
           *cur.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int == '/' as i32
       {
        cur = cur.offset(2 as std::os::raw::c_int as isize);
        ret = xmlParse3986Authority(uri, &mut cur);
        if ret != 0 as std::os::raw::c_int { return ret }
        if (*uri).server.is_null() { (*uri).port = -(1 as std::os::raw::c_int) }
        ret = xmlParse3986PathAbEmpty(uri, &mut cur);
        if ret != 0 as std::os::raw::c_int { return ret }
        *str = cur;
        return 0 as std::os::raw::c_int
    } else {
        if *cur as std::os::raw::c_int == '/' as i32 {
            ret = xmlParse3986PathAbsolute(uri, &mut cur);
            if ret != 0 as std::os::raw::c_int { return ret }
        } else if *cur as std::os::raw::c_int >= 'a' as i32 &&
                      *cur as std::os::raw::c_int <= 'z' as i32 ||
                      *cur as std::os::raw::c_int >= 'A' as i32 &&
                          *cur as std::os::raw::c_int <= 'Z' as i32 ||
                      *cur as std::os::raw::c_int >= '0' as i32 &&
                          *cur as std::os::raw::c_int <= '9' as i32 ||
                      *cur as std::os::raw::c_int == '-' as i32 ||
                      *cur as std::os::raw::c_int == '.' as i32 ||
                      *cur as std::os::raw::c_int == '_' as i32 ||
                      *cur as std::os::raw::c_int == '~' as i32 ||
                      *cur as std::os::raw::c_int == '%' as i32 &&
                          (*cur.offset(1 as std::os::raw::c_int as isize) as
                               std::os::raw::c_int >= '0' as i32 &&
                               *cur.offset(1 as std::os::raw::c_int as isize) as
                                   std::os::raw::c_int <= '9' as i32 ||
                               *cur.offset(1 as std::os::raw::c_int as isize) as
                                   std::os::raw::c_int >= 'a' as i32 &&
                                   *cur.offset(1 as std::os::raw::c_int as isize) as
                                       std::os::raw::c_int <= 'f' as i32 ||
                               *cur.offset(1 as std::os::raw::c_int as isize) as
                                   std::os::raw::c_int >= 'A' as i32 &&
                                   *cur.offset(1 as std::os::raw::c_int as isize) as
                                       std::os::raw::c_int <= 'F' as i32) &&
                          (*cur.offset(2 as std::os::raw::c_int as isize) as
                               std::os::raw::c_int >= '0' as i32 &&
                               *cur.offset(2 as std::os::raw::c_int as isize) as
                                   std::os::raw::c_int <= '9' as i32 ||
                               *cur.offset(2 as std::os::raw::c_int as isize) as
                                   std::os::raw::c_int >= 'a' as i32 &&
                                   *cur.offset(2 as std::os::raw::c_int as isize) as
                                       std::os::raw::c_int <= 'f' as i32 ||
                               *cur.offset(2 as std::os::raw::c_int as isize) as
                                   std::os::raw::c_int >= 'A' as i32 &&
                                   *cur.offset(2 as std::os::raw::c_int as isize) as
                                       std::os::raw::c_int <= 'F' as i32) ||
                      (*cur as std::os::raw::c_int == '!' as i32 ||
                           *cur as std::os::raw::c_int == '$' as i32 ||
                           *cur as std::os::raw::c_int == '&' as i32 ||
                           *cur as std::os::raw::c_int == '(' as i32 ||
                           *cur as std::os::raw::c_int == ')' as i32 ||
                           *cur as std::os::raw::c_int == '*' as i32 ||
                           *cur as std::os::raw::c_int == '+' as i32 ||
                           *cur as std::os::raw::c_int == ',' as i32 ||
                           *cur as std::os::raw::c_int == ';' as i32 ||
                           *cur as std::os::raw::c_int == '=' as i32 ||
                           *cur as std::os::raw::c_int == '\'' as i32) ||
                      *cur as std::os::raw::c_int == ':' as i32 ||
                      *cur as std::os::raw::c_int == '@' as i32 {
            ret = xmlParse3986PathRootless(uri, &mut cur);
            if ret != 0 as std::os::raw::c_int { return ret }
        } else if !uri.is_null() {
            if !(*uri).path.is_null() {
                xmlFree.expect("non-null function pointer")((*uri).path as
                                                                *mut std::os::raw::c_void);
            }
            (*uri).path = 0 as *mut std::os::raw::c_char
        }
    }
    *str = cur;
    return 0 as std::os::raw::c_int;
}
/* path-empty is effectively empty */
/* *
 * xmlParse3986RelativeRef:
 * @uri:  pointer to an URI structure
 * @str:  the string to analyze
 *
 * Parse an URI string and fills in the appropriate fields
 * of the @uri structure
 *
 * relative-ref  = relative-part [ "?" query ] [ "#" fragment ]
 * relative-part = "//" authority path-abempty
 *               / path-absolute
 *               / path-noscheme
 *               / path-empty
 *
 * Returns 0 or the error code
 */
unsafe extern "C" fn xmlParse3986RelativeRef(mut uri: xmlURIPtr,
                                             mut str: *const std::os::raw::c_char)
 -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0;
    if *str as std::os::raw::c_int == '/' as i32 &&
           *str.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int == '/' as i32
       {
        str = str.offset(2 as std::os::raw::c_int as isize);
        ret = xmlParse3986Authority(uri, &mut str);
        if ret != 0 as std::os::raw::c_int { return ret }
        ret = xmlParse3986PathAbEmpty(uri, &mut str);
        if ret != 0 as std::os::raw::c_int { return ret }
    } else if *str as std::os::raw::c_int == '/' as i32 {
        ret = xmlParse3986PathAbsolute(uri, &mut str);
        if ret != 0 as std::os::raw::c_int { return ret }
    } else if *str as std::os::raw::c_int >= 'a' as i32 &&
                  *str as std::os::raw::c_int <= 'z' as i32 ||
                  *str as std::os::raw::c_int >= 'A' as i32 &&
                      *str as std::os::raw::c_int <= 'Z' as i32 ||
                  *str as std::os::raw::c_int >= '0' as i32 &&
                      *str as std::os::raw::c_int <= '9' as i32 ||
                  *str as std::os::raw::c_int == '-' as i32 ||
                  *str as std::os::raw::c_int == '.' as i32 ||
                  *str as std::os::raw::c_int == '_' as i32 ||
                  *str as std::os::raw::c_int == '~' as i32 ||
                  *str as std::os::raw::c_int == '%' as i32 &&
                      (*str.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int
                           >= '0' as i32 &&
                           *str.offset(1 as std::os::raw::c_int as isize) as
                               std::os::raw::c_int <= '9' as i32 ||
                           *str.offset(1 as std::os::raw::c_int as isize) as
                               std::os::raw::c_int >= 'a' as i32 &&
                               *str.offset(1 as std::os::raw::c_int as isize) as
                                   std::os::raw::c_int <= 'f' as i32 ||
                           *str.offset(1 as std::os::raw::c_int as isize) as
                               std::os::raw::c_int >= 'A' as i32 &&
                               *str.offset(1 as std::os::raw::c_int as isize) as
                                   std::os::raw::c_int <= 'F' as i32) &&
                      (*str.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int
                           >= '0' as i32 &&
                           *str.offset(2 as std::os::raw::c_int as isize) as
                               std::os::raw::c_int <= '9' as i32 ||
                           *str.offset(2 as std::os::raw::c_int as isize) as
                               std::os::raw::c_int >= 'a' as i32 &&
                               *str.offset(2 as std::os::raw::c_int as isize) as
                                   std::os::raw::c_int <= 'f' as i32 ||
                           *str.offset(2 as std::os::raw::c_int as isize) as
                               std::os::raw::c_int >= 'A' as i32 &&
                               *str.offset(2 as std::os::raw::c_int as isize) as
                                   std::os::raw::c_int <= 'F' as i32) ||
                  (*str as std::os::raw::c_int == '!' as i32 ||
                       *str as std::os::raw::c_int == '$' as i32 ||
                       *str as std::os::raw::c_int == '&' as i32 ||
                       *str as std::os::raw::c_int == '(' as i32 ||
                       *str as std::os::raw::c_int == ')' as i32 ||
                       *str as std::os::raw::c_int == '*' as i32 ||
                       *str as std::os::raw::c_int == '+' as i32 ||
                       *str as std::os::raw::c_int == ',' as i32 ||
                       *str as std::os::raw::c_int == ';' as i32 ||
                       *str as std::os::raw::c_int == '=' as i32 ||
                       *str as std::os::raw::c_int == '\'' as i32) ||
                  *str as std::os::raw::c_int == ':' as i32 ||
                  *str as std::os::raw::c_int == '@' as i32 {
        ret = xmlParse3986PathNoScheme(uri, &mut str);
        if ret != 0 as std::os::raw::c_int { return ret }
    } else if !uri.is_null() {
        if !(*uri).path.is_null() {
            xmlFree.expect("non-null function pointer")((*uri).path as
                                                            *mut std::os::raw::c_void);
        }
        (*uri).path = 0 as *mut std::os::raw::c_char
    }
    if *str as std::os::raw::c_int == '?' as i32 {
        str = str.offset(1);
        ret = xmlParse3986Query(uri, &mut str);
        if ret != 0 as std::os::raw::c_int { return ret }
    }
    if *str as std::os::raw::c_int == '#' as i32 {
        str = str.offset(1);
        ret = xmlParse3986Fragment(uri, &mut str);
        if ret != 0 as std::os::raw::c_int { return ret }
    }
    if *str as std::os::raw::c_int != 0 as std::os::raw::c_int {
        xmlCleanURI(uri);
        return 1 as std::os::raw::c_int
    }
    return 0 as std::os::raw::c_int;
}
/* path-empty is effectively empty */
/* *
 * xmlParse3986URI:
 * @uri:  pointer to an URI structure
 * @str:  the string to analyze
 *
 * Parse an URI string and fills in the appropriate fields
 * of the @uri structure
 *
 * scheme ":" hier-part [ "?" query ] [ "#" fragment ]
 *
 * Returns 0 or the error code
 */
unsafe extern "C" fn xmlParse3986URI(mut uri: xmlURIPtr,
                                     mut str: *const std::os::raw::c_char)
 -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0;
    ret = xmlParse3986Scheme(uri, &mut str);
    if ret != 0 as std::os::raw::c_int { return ret }
    if *str as std::os::raw::c_int != ':' as i32 { return 1 as std::os::raw::c_int }
    str = str.offset(1);
    ret = xmlParse3986HierPart(uri, &mut str);
    if ret != 0 as std::os::raw::c_int { return ret }
    if *str as std::os::raw::c_int == '?' as i32 {
        str = str.offset(1);
        ret = xmlParse3986Query(uri, &mut str);
        if ret != 0 as std::os::raw::c_int { return ret }
    }
    if *str as std::os::raw::c_int == '#' as i32 {
        str = str.offset(1);
        ret = xmlParse3986Fragment(uri, &mut str);
        if ret != 0 as std::os::raw::c_int { return ret }
    }
    if *str as std::os::raw::c_int != 0 as std::os::raw::c_int {
        xmlCleanURI(uri);
        return 1 as std::os::raw::c_int
    }
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlParse3986URIReference:
 * @uri:  pointer to an URI structure
 * @str:  the string to analyze
 *
 * Parse an URI reference string and fills in the appropriate fields
 * of the @uri structure
 *
 * URI-reference = URI / relative-ref
 *
 * Returns 0 or the error code
 */
unsafe extern "C" fn xmlParse3986URIReference(mut uri: xmlURIPtr,
                                              mut str: *const std::os::raw::c_char)
 -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0;
    if str.is_null() { return -(1 as std::os::raw::c_int) }
    xmlCleanURI(uri);
    /*
     * Try first to parse absolute refs, then fallback to relative if
     * it fails.
     */
    ret = xmlParse3986URI(uri, str);
    if ret != 0 as std::os::raw::c_int {
        xmlCleanURI(uri);
        ret = xmlParse3986RelativeRef(uri, str);
        if ret != 0 as std::os::raw::c_int { xmlCleanURI(uri); return ret }
    }
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlParseURI:
 * @str:  the URI string to analyze
 *
 * Parse an URI based on RFC 3986
 *
 * URI-reference = [ absoluteURI | relativeURI ] [ "#" fragment ]
 *
 * Returns a newly built xmlURIPtr or NULL in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlParseURI(mut str: *const std::os::raw::c_char)
 -> xmlURIPtr {
    let mut uri: xmlURIPtr = 0 as *mut xmlURI;
    let mut ret: std::os::raw::c_int = 0;
    if str.is_null() { return 0 as xmlURIPtr }
    uri = xmlCreateURI();
    if !uri.is_null() {
        ret = xmlParse3986URIReference(uri, str);
        if ret != 0 { xmlFreeURI(uri); return 0 as xmlURIPtr }
    }
    return uri;
}
/* *
 * xmlParseURIReference:
 * @uri:  pointer to an URI structure
 * @str:  the string to analyze
 *
 * Parse an URI reference string based on RFC 3986 and fills in the
 * appropriate fields of the @uri structure
 *
 * URI-reference = URI / relative-ref
 *
 * Returns 0 or the error code
 */
#[no_mangle]
pub unsafe extern "C" fn xmlParseURIReference(mut uri: xmlURIPtr,
                                              mut str: *const std::os::raw::c_char)
 -> std::os::raw::c_int {
    return xmlParse3986URIReference(uri, str);
}
/* *
 * xmlParseURIRaw:
 * @str:  the URI string to analyze
 * @raw:  if 1 unescaping of URI pieces are disabled
 *
 * Parse an URI but allows to keep intact the original fragments.
 *
 * URI-reference = URI / relative-ref
 *
 * Returns a newly built xmlURIPtr or NULL in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlParseURIRaw(mut str: *const std::os::raw::c_char,
                                        mut raw: std::os::raw::c_int) -> xmlURIPtr {
    let mut uri: xmlURIPtr = 0 as *mut xmlURI;
    let mut ret: std::os::raw::c_int = 0;
    if str.is_null() { return 0 as xmlURIPtr }
    uri = xmlCreateURI();
    if !uri.is_null() {
        if raw != 0 { (*uri).cleanup |= 2 as std::os::raw::c_int }
        ret = xmlParseURIReference(uri, str);
        if ret != 0 { xmlFreeURI(uri); return 0 as xmlURIPtr }
    }
    return uri;
}
/* the query string (as it appears in the URI) */
/*
 * This function is in tree.h:
 * xmlChar *	xmlNodeGetBase	(xmlDocPtr doc,
 *                               xmlNodePtr cur);
 */
/* ***********************************************************************
 *									*
 *			Generic URI structure functions			*
 *									*
 ************************************************************************/
/* *
 * xmlCreateURI:
 *
 * Simply creates an empty xmlURI
 *
 * Returns the new structure or NULL in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlCreateURI() -> xmlURIPtr {
    let mut ret: xmlURIPtr = 0 as *mut xmlURI;
    ret =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlURI>()
                                                          as std::os::raw::c_ulong) as
            xmlURIPtr;
    if ret.is_null() {
        xmlURIErrMemory(b"creating URI structure\n\x00" as *const u8 as
                            *const std::os::raw::c_char);
        return 0 as xmlURIPtr
    }
    memset(ret as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<xmlURI>() as std::os::raw::c_ulong);
    return ret;
}
/* *
 * xmlSaveUriRealloc:
 *
 * Function to handle properly a reallocation when saving an URI
 * Also imposes some limit on the length of an URI string output
 */
unsafe extern "C" fn xmlSaveUriRealloc(mut ret: *mut xmlChar,
                                       mut max: *mut std::os::raw::c_int)
 -> *mut xmlChar {
    let mut temp: *mut xmlChar = 0 as *mut xmlChar;
    let mut tmp: std::os::raw::c_int = 0;
    if *max > 1024 as std::os::raw::c_int * 1024 as std::os::raw::c_int {
        xmlURIErrMemory(b"reaching arbitrary MAX_URI_LENGTH limit\n\x00" as
                            *const u8 as *const std::os::raw::c_char);
        return 0 as *mut xmlChar
    }
    tmp = *max * 2 as std::os::raw::c_int;
    temp =
        xmlRealloc.expect("non-null function pointer")(ret as
                                                           *mut std::os::raw::c_void,
                                                       (tmp +
                                                            1 as std::os::raw::c_int)
                                                           as size_t) as
            *mut xmlChar;
    if temp.is_null() {
        xmlURIErrMemory(b"saving URI\n\x00" as *const u8 as
                            *const std::os::raw::c_char);
        return 0 as *mut xmlChar
    }
    *max = tmp;
    return temp;
}
/* *
 * xmlSaveUri:
 * @uri:  pointer to an xmlURI
 *
 * Save the URI as an escaped string
 *
 * Returns a new string (to be deallocated by caller)
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSaveUri(mut uri: xmlURIPtr) -> *mut xmlChar {
    let mut current_block: u64;
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    let mut temp: *mut xmlChar = 0 as *mut xmlChar;
    let mut p: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut len: std::os::raw::c_int = 0;
    let mut max: std::os::raw::c_int = 0;
    if uri.is_null() { return 0 as *mut xmlChar }
    max = 80 as std::os::raw::c_int;
    ret =
        xmlMallocAtomic.expect("non-null function pointer")(((max +
                                                                  1 as
                                                                      std::os::raw::c_int)
                                                                 as
                                                                 std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<xmlChar>()
                                                                                                 as
                                                                                                 std::os::raw::c_ulong))
            as *mut xmlChar;
    if ret.is_null() {
        xmlURIErrMemory(b"saving URI\n\x00" as *const u8 as
                            *const std::os::raw::c_char);
        return 0 as *mut xmlChar
    }
    len = 0 as std::os::raw::c_int;
    if !(*uri).scheme.is_null() {
        p = (*uri).scheme;
        loop  {
            if !(*p as std::os::raw::c_int != 0 as std::os::raw::c_int) {
                current_block = 15904375183555213903;
                break ;
            }
            if len >= max {
                temp = xmlSaveUriRealloc(ret, &mut max);
                if temp.is_null() {
                    current_block = 13636304053406529094;
                    break ;
                }
                ret = temp
            }
            let fresh0 = p;
            p = p.offset(1);
            let fresh1 = len;
            len = len + 1;
            *ret.offset(fresh1 as isize) = *fresh0 as xmlChar
        }
        match current_block {
            13636304053406529094 => { }
            _ => {
                if len >= max {
                    temp = xmlSaveUriRealloc(ret, &mut max);
                    if temp.is_null() {
                        current_block = 13636304053406529094;
                    } else {
                        ret = temp;
                        current_block = 14401909646449704462;
                    }
                } else { current_block = 14401909646449704462; }
                match current_block {
                    13636304053406529094 => { }
                    _ => {
                        let fresh2 = len;
                        len = len + 1;
                        *ret.offset(fresh2 as isize) = ':' as i32 as xmlChar;
                        current_block = 17478428563724192186;
                    }
                }
            }
        }
    } else { current_block = 17478428563724192186; }
    match current_block {
        17478428563724192186 => {
            if !(*uri).opaque.is_null() {
                p = (*uri).opaque;
                loop  {
                    if !(*p as std::os::raw::c_int != 0 as std::os::raw::c_int) {
                        current_block = 13161952823003036500;
                        break ;
                    }
                    if len + 3 as std::os::raw::c_int >= max {
                        temp = xmlSaveUriRealloc(ret, &mut max);
                        if temp.is_null() {
                            current_block = 13636304053406529094;
                            break ;
                        }
                        ret = temp
                    }
                    if *p as std::os::raw::c_int == ';' as i32 ||
                           *p as std::os::raw::c_int == '/' as i32 ||
                           *p as std::os::raw::c_int == '?' as i32 ||
                           *p as std::os::raw::c_int == ':' as i32 ||
                           *p as std::os::raw::c_int == '@' as i32 ||
                           *p as std::os::raw::c_int == '&' as i32 ||
                           *p as std::os::raw::c_int == '=' as i32 ||
                           *p as std::os::raw::c_int == '+' as i32 ||
                           *p as std::os::raw::c_int == '$' as i32 ||
                           *p as std::os::raw::c_int == ',' as i32 ||
                           *p as std::os::raw::c_int == '[' as i32 ||
                           *p as std::os::raw::c_int == ']' as i32 ||
                           (*p as std::os::raw::c_int >= 'a' as i32 &&
                                *p as std::os::raw::c_int <= 'z' as i32 ||
                                *p as std::os::raw::c_int >= 'A' as i32 &&
                                    *p as std::os::raw::c_int <= 'Z' as i32 ||
                                *p as std::os::raw::c_int >= '0' as i32 &&
                                    *p as std::os::raw::c_int <= '9' as i32 ||
                                (*p as std::os::raw::c_int == '-' as i32 ||
                                     *p as std::os::raw::c_int == '_' as i32 ||
                                     *p as std::os::raw::c_int == '.' as i32 ||
                                     *p as std::os::raw::c_int == '!' as i32 ||
                                     *p as std::os::raw::c_int == '~' as i32 ||
                                     *p as std::os::raw::c_int == '*' as i32 ||
                                     *p as std::os::raw::c_int == '\'' as i32 ||
                                     *p as std::os::raw::c_int == '(' as i32 ||
                                     *p as std::os::raw::c_int == ')' as i32)) {
                        let fresh3 = p;
                        p = p.offset(1);
                        let fresh4 = len;
                        len = len + 1;
                        *ret.offset(fresh4 as isize) = *fresh3 as xmlChar
                    } else {
                        let fresh5 = p;
                        p = p.offset(1);
                        let mut val: std::os::raw::c_int =
                            *(fresh5 as *mut std::os::raw::c_uchar) as std::os::raw::c_int;
                        let mut hi: std::os::raw::c_int = val / 0x10 as std::os::raw::c_int;
                        let mut lo: std::os::raw::c_int = val % 0x10 as std::os::raw::c_int;
                        let fresh6 = len;
                        len = len + 1;
                        *ret.offset(fresh6 as isize) = '%' as i32 as xmlChar;
                        let fresh7 = len;
                        len = len + 1;
                        *ret.offset(fresh7 as isize) =
                            (hi +
                                 (if hi > 9 as std::os::raw::c_int {
                                      ('A' as i32) - 10 as std::os::raw::c_int
                                  } else { '0' as i32 })) as xmlChar;
                        let fresh8 = len;
                        len = len + 1;
                        *ret.offset(fresh8 as isize) =
                            (lo +
                                 (if lo > 9 as std::os::raw::c_int {
                                      ('A' as i32) - 10 as std::os::raw::c_int
                                  } else { '0' as i32 })) as xmlChar
                    }
                }
            } else {
                if !(*uri).server.is_null() ||
                       (*uri).port == -(1 as std::os::raw::c_int) {
                    if len + 3 as std::os::raw::c_int >= max {
                        temp = xmlSaveUriRealloc(ret, &mut max);
                        if temp.is_null() {
                            current_block = 13636304053406529094;
                        } else {
                            ret = temp;
                            current_block = 9441801433784995173;
                        }
                    } else { current_block = 9441801433784995173; }
                    match current_block {
                        13636304053406529094 => { }
                        _ => {
                            let fresh9 = len;
                            len = len + 1;
                            *ret.offset(fresh9 as isize) =
                                '/' as i32 as xmlChar;
                            let fresh10 = len;
                            len = len + 1;
                            *ret.offset(fresh10 as isize) =
                                '/' as i32 as xmlChar;
                            if !(*uri).user.is_null() {
                                p = (*uri).user;
                                loop  {
                                    if !(*p as std::os::raw::c_int !=
                                             0 as std::os::raw::c_int) {
                                        current_block = 8835654301469918283;
                                        break ;
                                    }
                                    if len + 3 as std::os::raw::c_int >= max {
                                        temp =
                                            xmlSaveUriRealloc(ret, &mut max);
                                        if temp.is_null() {
                                            current_block =
                                                13636304053406529094;
                                            break ;
                                        }
                                        ret = temp
                                    }
                                    if *p as std::os::raw::c_int >= 'a' as i32 &&
                                           *p as std::os::raw::c_int <= 'z' as i32 ||
                                           *p as std::os::raw::c_int >= 'A' as i32 &&
                                               *p as std::os::raw::c_int <= 'Z' as i32
                                           ||
                                           *p as std::os::raw::c_int >= '0' as i32 &&
                                               *p as std::os::raw::c_int <= '9' as i32
                                           ||
                                           (*p as std::os::raw::c_int == '-' as i32 ||
                                                *p as std::os::raw::c_int ==
                                                    '_' as i32 ||
                                                *p as std::os::raw::c_int ==
                                                    '.' as i32 ||
                                                *p as std::os::raw::c_int ==
                                                    '!' as i32 ||
                                                *p as std::os::raw::c_int ==
                                                    '~' as i32 ||
                                                *p as std::os::raw::c_int ==
                                                    '*' as i32 ||
                                                *p as std::os::raw::c_int ==
                                                    '\'' as i32 ||
                                                *p as std::os::raw::c_int ==
                                                    '(' as i32 ||
                                                *p as std::os::raw::c_int ==
                                                    ')' as i32) ||
                                           *p as std::os::raw::c_int == ';' as i32 ||
                                           *p as std::os::raw::c_int == ':' as i32 ||
                                           *p as std::os::raw::c_int == '&' as i32 ||
                                           *p as std::os::raw::c_int == '=' as i32 ||
                                           *p as std::os::raw::c_int == '+' as i32 ||
                                           *p as std::os::raw::c_int == '$' as i32 ||
                                           *p as std::os::raw::c_int == ',' as i32 {
                                        let fresh11 = p;
                                        p = p.offset(1);
                                        let fresh12 = len;
                                        len = len + 1;
                                        *ret.offset(fresh12 as isize) =
                                            *fresh11 as xmlChar
                                    } else {
                                        let fresh13 = p;
                                        p = p.offset(1);
                                        let mut val_0: std::os::raw::c_int =
                                            *(fresh13 as *mut std::os::raw::c_uchar)
                                                as std::os::raw::c_int;
                                        let mut hi_0: std::os::raw::c_int =
                                            val_0 / 0x10 as std::os::raw::c_int;
                                        let mut lo_0: std::os::raw::c_int =
                                            val_0 % 0x10 as std::os::raw::c_int;
                                        let fresh14 = len;
                                        len = len + 1;
                                        *ret.offset(fresh14 as isize) =
                                            '%' as i32 as xmlChar;
                                        let fresh15 = len;
                                        len = len + 1;
                                        *ret.offset(fresh15 as isize) =
                                            (hi_0 +
                                                 (if hi_0 > 9 as std::os::raw::c_int {
                                                      ('A' as i32) -
                                                          10 as std::os::raw::c_int
                                                  } else { '0' as i32 })) as
                                                xmlChar;
                                        let fresh16 = len;
                                        len = len + 1;
                                        *ret.offset(fresh16 as isize) =
                                            (lo_0 +
                                                 (if lo_0 > 9 as std::os::raw::c_int {
                                                      ('A' as i32) -
                                                          10 as std::os::raw::c_int
                                                  } else { '0' as i32 })) as
                                                xmlChar
                                    }
                                }
                                match current_block {
                                    13636304053406529094 => { }
                                    _ => {
                                        if len + 3 as std::os::raw::c_int >= max {
                                            temp =
                                                xmlSaveUriRealloc(ret,
                                                                  &mut max);
                                            if temp.is_null() {
                                                current_block =
                                                    13636304053406529094;
                                            } else {
                                                ret = temp;
                                                current_block =
                                                    307447392441238883;
                                            }
                                        } else {
                                            current_block =
                                                307447392441238883;
                                        }
                                        match current_block {
                                            13636304053406529094 => { }
                                            _ => {
                                                let fresh17 = len;
                                                len = len + 1;
                                                *ret.offset(fresh17 as isize)
                                                    = '@' as i32 as xmlChar;
                                                current_block =
                                                    15970011996474399071;
                                            }
                                        }
                                    }
                                }
                            } else { current_block = 15970011996474399071; }
                            match current_block {
                                13636304053406529094 => { }
                                _ => {
                                    if !(*uri).server.is_null() {
                                        p = (*uri).server;
                                        loop  {
                                            if !(*p as std::os::raw::c_int !=
                                                     0 as std::os::raw::c_int) {
                                                current_block =
                                                    3736434875406665187;
                                                break ;
                                            }
                                            if len >= max {
                                                temp =
                                                    xmlSaveUriRealloc(ret,
                                                                      &mut max);
                                                if temp.is_null() {
                                                    current_block =
                                                        13636304053406529094;
                                                    break ;
                                                }
                                                ret = temp
                                            }
                                            let fresh18 = p;
                                            p = p.offset(1);
                                            let fresh19 = len;
                                            len = len + 1;
                                            *ret.offset(fresh19 as isize) =
                                                *fresh18 as xmlChar
                                        }
                                        match current_block {
                                            13636304053406529094 => { }
                                            _ => {
                                                if (*uri).port >
                                                       0 as std::os::raw::c_int {
                                                    if len + 10 as std::os::raw::c_int
                                                           >= max {
                                                        temp =
                                                            xmlSaveUriRealloc(ret,
                                                                              &mut max);
                                                        if temp.is_null() {
                                                            current_block =
                                                                13636304053406529094;
                                                        } else {
                                                            ret = temp;
                                                            current_block =
                                                                993425571616822999;
                                                        }
                                                    } else {
                                                        current_block =
                                                            993425571616822999;
                                                    }
                                                    match current_block {
                                                        13636304053406529094
                                                        => {
                                                        }
                                                        _ => {
                                                            len +=
                                                                snprintf(&mut *ret.offset(len
                                                                                              as
                                                                                              isize)
                                                                             as
                                                                             *mut xmlChar
                                                                             as
                                                                             *mut std::os::raw::c_char,
                                                                         (max
                                                                              -
                                                                              len)
                                                                             as
                                                                             std::os::raw::c_ulong,
                                                                         b":%d\x00"
                                                                             as
                                                                             *const u8
                                                                             as
                                                                             *const std::os::raw::c_char,
                                                                         (*uri).port);
                                                            current_block =
                                                                9775647934248138666;
                                                        }
                                                    }
                                                } else {
                                                    current_block =
                                                        9775647934248138666;
                                                }
                                            }
                                        }
                                    } else {
                                        current_block = 9775647934248138666;
                                    }
                                }
                            }
                        }
                    }
                } else if !(*uri).authority.is_null() {
                    if len + 3 as std::os::raw::c_int >= max {
                        temp = xmlSaveUriRealloc(ret, &mut max);
                        if temp.is_null() {
                            current_block = 13636304053406529094;
                        } else {
                            ret = temp;
                            current_block = 12705158477165241210;
                        }
                    } else { current_block = 12705158477165241210; }
                    match current_block {
                        13636304053406529094 => { }
                        _ => {
                            let fresh20 = len;
                            len = len + 1;
                            *ret.offset(fresh20 as isize) =
                                '/' as i32 as xmlChar;
                            let fresh21 = len;
                            len = len + 1;
                            *ret.offset(fresh21 as isize) =
                                '/' as i32 as xmlChar;
                            p = (*uri).authority;
                            loop  {
                                if !(*p as std::os::raw::c_int != 0 as std::os::raw::c_int) {
                                    current_block = 9775647934248138666;
                                    break ;
                                }
                                if len + 3 as std::os::raw::c_int >= max {
                                    temp = xmlSaveUriRealloc(ret, &mut max);
                                    if temp.is_null() {
                                        current_block = 13636304053406529094;
                                        break ;
                                    }
                                    ret = temp
                                }
                                if *p as std::os::raw::c_int >= 'a' as i32 &&
                                       *p as std::os::raw::c_int <= 'z' as i32 ||
                                       *p as std::os::raw::c_int >= 'A' as i32 &&
                                           *p as std::os::raw::c_int <= 'Z' as i32 ||
                                       *p as std::os::raw::c_int >= '0' as i32 &&
                                           *p as std::os::raw::c_int <= '9' as i32 ||
                                       (*p as std::os::raw::c_int == '-' as i32 ||
                                            *p as std::os::raw::c_int == '_' as i32 ||
                                            *p as std::os::raw::c_int == '.' as i32 ||
                                            *p as std::os::raw::c_int == '!' as i32 ||
                                            *p as std::os::raw::c_int == '~' as i32 ||
                                            *p as std::os::raw::c_int == '*' as i32 ||
                                            *p as std::os::raw::c_int == '\'' as i32
                                            || *p as std::os::raw::c_int == '(' as i32
                                            ||
                                            *p as std::os::raw::c_int == ')' as i32)
                                       || *p as std::os::raw::c_int == '$' as i32 ||
                                       *p as std::os::raw::c_int == ',' as i32 ||
                                       *p as std::os::raw::c_int == ';' as i32 ||
                                       *p as std::os::raw::c_int == ':' as i32 ||
                                       *p as std::os::raw::c_int == '@' as i32 ||
                                       *p as std::os::raw::c_int == '&' as i32 ||
                                       *p as std::os::raw::c_int == '=' as i32 ||
                                       *p as std::os::raw::c_int == '+' as i32 {
                                    let fresh22 = p;
                                    p = p.offset(1);
                                    let fresh23 = len;
                                    len = len + 1;
                                    *ret.offset(fresh23 as isize) =
                                        *fresh22 as xmlChar
                                } else {
                                    let fresh24 = p;
                                    p = p.offset(1);
                                    let mut val_1: std::os::raw::c_int =
                                        *(fresh24 as *mut std::os::raw::c_uchar) as
                                            std::os::raw::c_int;
                                    let mut hi_1: std::os::raw::c_int =
                                        val_1 / 0x10 as std::os::raw::c_int;
                                    let mut lo_1: std::os::raw::c_int =
                                        val_1 % 0x10 as std::os::raw::c_int;
                                    let fresh25 = len;
                                    len = len + 1;
                                    *ret.offset(fresh25 as isize) =
                                        '%' as i32 as xmlChar;
                                    let fresh26 = len;
                                    len = len + 1;
                                    *ret.offset(fresh26 as isize) =
                                        (hi_1 +
                                             (if hi_1 > 9 as std::os::raw::c_int {
                                                  ('A' as i32) -
                                                      10 as std::os::raw::c_int
                                              } else { '0' as i32 })) as
                                            xmlChar;
                                    let fresh27 = len;
                                    len = len + 1;
                                    *ret.offset(fresh27 as isize) =
                                        (lo_1 +
                                             (if lo_1 > 9 as std::os::raw::c_int {
                                                  ('A' as i32) -
                                                      10 as std::os::raw::c_int
                                              } else { '0' as i32 })) as
                                            xmlChar
                                }
                            }
                        }
                    }
                } else if !(*uri).scheme.is_null() {
                    if len + 3 as std::os::raw::c_int >= max {
                        temp = xmlSaveUriRealloc(ret, &mut max);
                        if temp.is_null() {
                            current_block = 13636304053406529094;
                        } else {
                            ret = temp;
                            current_block = 9775647934248138666;
                        }
                    } else { current_block = 9775647934248138666; }
                } else { current_block = 9775647934248138666; }
                match current_block {
                    13636304053406529094 => { }
                    _ => {
                        if !(*uri).path.is_null() {
                            p = (*uri).path;
                            /*
	     * the colon in file:///d: should not be escaped or
	     * Windows accesses fail later.
	     */
                            if !(*uri).scheme.is_null() &&
                                   *p.offset(0 as std::os::raw::c_int as isize) as
                                       std::os::raw::c_int == '/' as i32 &&
                                   (*p.offset(1 as std::os::raw::c_int as isize) as
                                        std::os::raw::c_int >= 'a' as i32 &&
                                        *p.offset(1 as std::os::raw::c_int as isize)
                                            as std::os::raw::c_int <= 'z' as i32 ||
                                        *p.offset(1 as std::os::raw::c_int as isize)
                                            as std::os::raw::c_int >= 'A' as i32 &&
                                            *p.offset(1 as std::os::raw::c_int as
                                                          isize) as
                                                std::os::raw::c_int <= 'Z' as i32) &&
                                   *p.offset(2 as std::os::raw::c_int as isize) as
                                       std::os::raw::c_int == ':' as i32 &&
                                   xmlStrEqual((*uri).scheme as *mut xmlChar,
                                               b"file\x00" as *const u8 as
                                                   *const std::os::raw::c_char as
                                                   *mut xmlChar) != 0 {
                                if len + 3 as std::os::raw::c_int >= max {
                                    temp = xmlSaveUriRealloc(ret, &mut max);
                                    if temp.is_null() {
                                        current_block = 13636304053406529094;
                                    } else {
                                        ret = temp;
                                        current_block = 5248622017361056354;
                                    }
                                } else {
                                    current_block = 5248622017361056354;
                                }
                                match current_block {
                                    13636304053406529094 => { }
                                    _ => {
                                        let fresh28 = p;
                                        p = p.offset(1);
                                        let fresh29 = len;
                                        len = len + 1;
                                        *ret.offset(fresh29 as isize) =
                                            *fresh28 as xmlChar;
                                        let fresh30 = p;
                                        p = p.offset(1);
                                        let fresh31 = len;
                                        len = len + 1;
                                        *ret.offset(fresh31 as isize) =
                                            *fresh30 as xmlChar;
                                        let fresh32 = p;
                                        p = p.offset(1);
                                        let fresh33 = len;
                                        len = len + 1;
                                        *ret.offset(fresh33 as isize) =
                                            *fresh32 as xmlChar;
                                        current_block = 13014351284863956202;
                                    }
                                }
                            } else { current_block = 13014351284863956202; }
                            match current_block {
                                13636304053406529094 => { }
                                _ => {
                                    loop  {
                                        if !(*p as std::os::raw::c_int !=
                                                 0 as std::os::raw::c_int) {
                                            current_block =
                                                7923086311623215889;
                                            break ;
                                        }
                                        if len + 3 as std::os::raw::c_int >= max {
                                            temp =
                                                xmlSaveUriRealloc(ret,
                                                                  &mut max);
                                            if temp.is_null() {
                                                current_block =
                                                    13636304053406529094;
                                                break ;
                                            }
                                            ret = temp
                                        }
                                        if *p as std::os::raw::c_int >= 'a' as i32 &&
                                               *p as std::os::raw::c_int <= 'z' as i32
                                               ||
                                               *p as std::os::raw::c_int >= 'A' as i32
                                                   &&
                                                   *p as std::os::raw::c_int <=
                                                       'Z' as i32 ||
                                               *p as std::os::raw::c_int >= '0' as i32
                                                   &&
                                                   *p as std::os::raw::c_int <=
                                                       '9' as i32 ||
                                               (*p as std::os::raw::c_int ==
                                                    '-' as i32 ||
                                                    *p as std::os::raw::c_int ==
                                                        '_' as i32 ||
                                                    *p as std::os::raw::c_int ==
                                                        '.' as i32 ||
                                                    *p as std::os::raw::c_int ==
                                                        '!' as i32 ||
                                                    *p as std::os::raw::c_int ==
                                                        '~' as i32 ||
                                                    *p as std::os::raw::c_int ==
                                                        '*' as i32 ||
                                                    *p as std::os::raw::c_int ==
                                                        '\'' as i32 ||
                                                    *p as std::os::raw::c_int ==
                                                        '(' as i32 ||
                                                    *p as std::os::raw::c_int ==
                                                        ')' as i32) ||
                                               *p as std::os::raw::c_int == '/' as i32
                                               ||
                                               *p as std::os::raw::c_int == ';' as i32
                                               ||
                                               *p as std::os::raw::c_int == '@' as i32
                                               ||
                                               *p as std::os::raw::c_int == '&' as i32
                                               ||
                                               *p as std::os::raw::c_int == '=' as i32
                                               ||
                                               *p as std::os::raw::c_int == '+' as i32
                                               ||
                                               *p as std::os::raw::c_int == '$' as i32
                                               ||
                                               *p as std::os::raw::c_int == ',' as i32
                                           {
                                            let fresh34 = p;
                                            p = p.offset(1);
                                            let fresh35 = len;
                                            len = len + 1;
                                            *ret.offset(fresh35 as isize) =
                                                *fresh34 as xmlChar
                                        } else {
                                            let fresh36 = p;
                                            p = p.offset(1);
                                            let mut val_2: std::os::raw::c_int =
                                                *(fresh36 as
                                                      *mut std::os::raw::c_uchar) as
                                                    std::os::raw::c_int;
                                            let mut hi_2: std::os::raw::c_int =
                                                val_2 / 0x10 as std::os::raw::c_int;
                                            let mut lo_2: std::os::raw::c_int =
                                                val_2 % 0x10 as std::os::raw::c_int;
                                            let fresh37 = len;
                                            len = len + 1;
                                            *ret.offset(fresh37 as isize) =
                                                '%' as i32 as xmlChar;
                                            let fresh38 = len;
                                            len = len + 1;
                                            *ret.offset(fresh38 as isize) =
                                                (hi_2 +
                                                     (if hi_2 >
                                                             9 as std::os::raw::c_int
                                                         {
                                                          ('A' as i32) -
                                                              10 as
                                                                  std::os::raw::c_int
                                                      } else { '0' as i32 }))
                                                    as xmlChar;
                                            let fresh39 = len;
                                            len = len + 1;
                                            *ret.offset(fresh39 as isize) =
                                                (lo_2 +
                                                     (if lo_2 >
                                                             9 as std::os::raw::c_int
                                                         {
                                                          ('A' as i32) -
                                                              10 as
                                                                  std::os::raw::c_int
                                                      } else { '0' as i32 }))
                                                    as xmlChar
                                        }
                                    }
                                }
                            }
                        } else { current_block = 7923086311623215889; }
                        match current_block {
                            13636304053406529094 => { }
                            _ => {
                                if !(*uri).query_raw.is_null() {
                                    if len + 1 as std::os::raw::c_int >= max {
                                        temp =
                                            xmlSaveUriRealloc(ret, &mut max);
                                        if temp.is_null() {
                                            current_block =
                                                13636304053406529094;
                                        } else {
                                            ret = temp;
                                            current_block = 92352228884877657;
                                        }
                                    } else {
                                        current_block = 92352228884877657;
                                    }
                                    match current_block {
                                        13636304053406529094 => { }
                                        _ => {
                                            let fresh40 = len;
                                            len = len + 1;
                                            *ret.offset(fresh40 as isize) =
                                                '?' as i32 as xmlChar;
                                            p = (*uri).query_raw;
                                            loop  {
                                                if !(*p as std::os::raw::c_int !=
                                                         0 as std::os::raw::c_int) {
                                                    current_block =
                                                        13161952823003036500;
                                                    break ;
                                                }
                                                if len + 1 as std::os::raw::c_int >=
                                                       max {
                                                    temp =
                                                        xmlSaveUriRealloc(ret,
                                                                          &mut max);
                                                    if temp.is_null() {
                                                        current_block =
                                                            13636304053406529094;
                                                        break ;
                                                    }
                                                    ret = temp
                                                }
                                                let fresh41 = p;
                                                p = p.offset(1);
                                                let fresh42 = len;
                                                len = len + 1;
                                                *ret.offset(fresh42 as isize)
                                                    = *fresh41 as xmlChar
                                            }
                                        }
                                    }
                                } else if !(*uri).query.is_null() {
                                    if len + 3 as std::os::raw::c_int >= max {
                                        temp =
                                            xmlSaveUriRealloc(ret, &mut max);
                                        if temp.is_null() {
                                            current_block =
                                                13636304053406529094;
                                        } else {
                                            ret = temp;
                                            current_block =
                                                2838755337219234678;
                                        }
                                    } else {
                                        current_block = 2838755337219234678;
                                    }
                                    match current_block {
                                        13636304053406529094 => { }
                                        _ => {
                                            let fresh43 = len;
                                            len = len + 1;
                                            *ret.offset(fresh43 as isize) =
                                                '?' as i32 as xmlChar;
                                            p = (*uri).query;
                                            loop  {
                                                if !(*p as std::os::raw::c_int !=
                                                         0 as std::os::raw::c_int) {
                                                    current_block =
                                                        13161952823003036500;
                                                    break ;
                                                }
                                                if len + 3 as std::os::raw::c_int >=
                                                       max {
                                                    temp =
                                                        xmlSaveUriRealloc(ret,
                                                                          &mut max);
                                                    if temp.is_null() {
                                                        current_block =
                                                            13636304053406529094;
                                                        break ;
                                                    }
                                                    ret = temp
                                                }
                                                if *p as std::os::raw::c_int >=
                                                       'a' as i32 &&
                                                       *p as std::os::raw::c_int <=
                                                           'z' as i32 ||
                                                       *p as std::os::raw::c_int >=
                                                           'A' as i32 &&
                                                           *p as std::os::raw::c_int
                                                               <= 'Z' as i32
                                                       ||
                                                       *p as std::os::raw::c_int >=
                                                           '0' as i32 &&
                                                           *p as std::os::raw::c_int
                                                               <= '9' as i32
                                                       ||
                                                       (*p as std::os::raw::c_int ==
                                                            '-' as i32 ||
                                                            *p as std::os::raw::c_int
                                                                == '_' as i32
                                                            ||
                                                            *p as std::os::raw::c_int
                                                                == '.' as i32
                                                            ||
                                                            *p as std::os::raw::c_int
                                                                == '!' as i32
                                                            ||
                                                            *p as std::os::raw::c_int
                                                                == '~' as i32
                                                            ||
                                                            *p as std::os::raw::c_int
                                                                == '*' as i32
                                                            ||
                                                            *p as std::os::raw::c_int
                                                                == '\'' as i32
                                                            ||
                                                            *p as std::os::raw::c_int
                                                                == '(' as i32
                                                            ||
                                                            *p as std::os::raw::c_int
                                                                == ')' as i32)
                                                       ||
                                                       (*p as std::os::raw::c_int ==
                                                            ';' as i32 ||
                                                            *p as std::os::raw::c_int
                                                                == '/' as i32
                                                            ||
                                                            *p as std::os::raw::c_int
                                                                == '?' as i32
                                                            ||
                                                            *p as std::os::raw::c_int
                                                                == ':' as i32
                                                            ||
                                                            *p as std::os::raw::c_int
                                                                == '@' as i32
                                                            ||
                                                            *p as std::os::raw::c_int
                                                                == '&' as i32
                                                            ||
                                                            *p as std::os::raw::c_int
                                                                == '=' as i32
                                                            ||
                                                            *p as std::os::raw::c_int
                                                                == '+' as i32
                                                            ||
                                                            *p as std::os::raw::c_int
                                                                == '$' as i32
                                                            ||
                                                            *p as std::os::raw::c_int
                                                                == ',' as i32
                                                            ||
                                                            *p as std::os::raw::c_int
                                                                == '[' as i32
                                                            ||
                                                            *p as std::os::raw::c_int
                                                                == ']' as i32)
                                                   {
                                                    let fresh44 = p;
                                                    p = p.offset(1);
                                                    let fresh45 = len;
                                                    len = len + 1;
                                                    *ret.offset(fresh45 as
                                                                    isize) =
                                                        *fresh44 as xmlChar
                                                } else {
                                                    let fresh46 = p;
                                                    p = p.offset(1);
                                                    let mut val_3:
                                                            std::os::raw::c_int =
                                                        *(fresh46 as
                                                              *mut std::os::raw::c_uchar)
                                                            as std::os::raw::c_int;
                                                    let mut hi_3:
                                                            std::os::raw::c_int =
                                                        val_3 /
                                                            0x10 as
                                                                std::os::raw::c_int;
                                                    let mut lo_3:
                                                            std::os::raw::c_int =
                                                        val_3 %
                                                            0x10 as
                                                                std::os::raw::c_int;
                                                    let fresh47 = len;
                                                    len = len + 1;
                                                    *ret.offset(fresh47 as
                                                                    isize) =
                                                        '%' as i32 as xmlChar;
                                                    let fresh48 = len;
                                                    len = len + 1;
                                                    *ret.offset(fresh48 as
                                                                    isize) =
                                                        (hi_3 +
                                                             (if hi_3 >
                                                                     9 as
                                                                         std::os::raw::c_int
                                                                 {
                                                                  ('A' as i32)
                                                                      -
                                                                      10 as
                                                                          std::os::raw::c_int
                                                              } else {
                                                                  '0' as i32
                                                              })) as xmlChar;
                                                    let fresh49 = len;
                                                    len = len + 1;
                                                    *ret.offset(fresh49 as
                                                                    isize) =
                                                        (lo_3 +
                                                             (if lo_3 >
                                                                     9 as
                                                                         std::os::raw::c_int
                                                                 {
                                                                  ('A' as i32)
                                                                      -
                                                                      10 as
                                                                          std::os::raw::c_int
                                                              } else {
                                                                  '0' as i32
                                                              })) as xmlChar
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    current_block = 13161952823003036500;
                                }
                            }
                        }
                    }
                }
            }
            match current_block {
                13636304053406529094 => { }
                _ => {
                    if !(*uri).fragment.is_null() {
                        if len + 3 as std::os::raw::c_int >= max {
                            temp = xmlSaveUriRealloc(ret, &mut max);
                            if temp.is_null() {
                                current_block = 13636304053406529094;
                            } else {
                                ret = temp;
                                current_block = 654039154479240366;
                            }
                        } else { current_block = 654039154479240366; }
                        match current_block {
                            13636304053406529094 => { }
                            _ => {
                                let fresh50 = len;
                                len = len + 1;
                                *ret.offset(fresh50 as isize) =
                                    '#' as i32 as xmlChar;
                                p = (*uri).fragment;
                                loop  {
                                    if !(*p as std::os::raw::c_int !=
                                             0 as std::os::raw::c_int) {
                                        current_block = 7256998052328658819;
                                        break ;
                                    }
                                    if len + 3 as std::os::raw::c_int >= max {
                                        temp =
                                            xmlSaveUriRealloc(ret, &mut max);
                                        if temp.is_null() {
                                            current_block =
                                                13636304053406529094;
                                            break ;
                                        }
                                        ret = temp
                                    }
                                    if *p as std::os::raw::c_int >= 'a' as i32 &&
                                           *p as std::os::raw::c_int <= 'z' as i32 ||
                                           *p as std::os::raw::c_int >= 'A' as i32 &&
                                               *p as std::os::raw::c_int <= 'Z' as i32
                                           ||
                                           *p as std::os::raw::c_int >= '0' as i32 &&
                                               *p as std::os::raw::c_int <= '9' as i32
                                           ||
                                           (*p as std::os::raw::c_int == '-' as i32 ||
                                                *p as std::os::raw::c_int ==
                                                    '_' as i32 ||
                                                *p as std::os::raw::c_int ==
                                                    '.' as i32 ||
                                                *p as std::os::raw::c_int ==
                                                    '!' as i32 ||
                                                *p as std::os::raw::c_int ==
                                                    '~' as i32 ||
                                                *p as std::os::raw::c_int ==
                                                    '*' as i32 ||
                                                *p as std::os::raw::c_int ==
                                                    '\'' as i32 ||
                                                *p as std::os::raw::c_int ==
                                                    '(' as i32 ||
                                                *p as std::os::raw::c_int ==
                                                    ')' as i32) ||
                                           (*p as std::os::raw::c_int == ';' as i32 ||
                                                *p as std::os::raw::c_int ==
                                                    '/' as i32 ||
                                                *p as std::os::raw::c_int ==
                                                    '?' as i32 ||
                                                *p as std::os::raw::c_int ==
                                                    ':' as i32 ||
                                                *p as std::os::raw::c_int ==
                                                    '@' as i32 ||
                                                *p as std::os::raw::c_int ==
                                                    '&' as i32 ||
                                                *p as std::os::raw::c_int ==
                                                    '=' as i32 ||
                                                *p as std::os::raw::c_int ==
                                                    '+' as i32 ||
                                                *p as std::os::raw::c_int ==
                                                    '$' as i32 ||
                                                *p as std::os::raw::c_int ==
                                                    ',' as i32 ||
                                                *p as std::os::raw::c_int ==
                                                    '[' as i32 ||
                                                *p as std::os::raw::c_int ==
                                                    ']' as i32) {
                                        let fresh51 = p;
                                        p = p.offset(1);
                                        let fresh52 = len;
                                        len = len + 1;
                                        *ret.offset(fresh52 as isize) =
                                            *fresh51 as xmlChar
                                    } else {
                                        let fresh53 = p;
                                        p = p.offset(1);
                                        let mut val_4: std::os::raw::c_int =
                                            *(fresh53 as *mut std::os::raw::c_uchar)
                                                as std::os::raw::c_int;
                                        let mut hi_4: std::os::raw::c_int =
                                            val_4 / 0x10 as std::os::raw::c_int;
                                        let mut lo_4: std::os::raw::c_int =
                                            val_4 % 0x10 as std::os::raw::c_int;
                                        let fresh54 = len;
                                        len = len + 1;
                                        *ret.offset(fresh54 as isize) =
                                            '%' as i32 as xmlChar;
                                        let fresh55 = len;
                                        len = len + 1;
                                        *ret.offset(fresh55 as isize) =
                                            (hi_4 +
                                                 (if hi_4 > 9 as std::os::raw::c_int {
                                                      ('A' as i32) -
                                                          10 as std::os::raw::c_int
                                                  } else { '0' as i32 })) as
                                                xmlChar;
                                        let fresh56 = len;
                                        len = len + 1;
                                        *ret.offset(fresh56 as isize) =
                                            (lo_4 +
                                                 (if lo_4 > 9 as std::os::raw::c_int {
                                                      ('A' as i32) -
                                                          10 as std::os::raw::c_int
                                                  } else { '0' as i32 })) as
                                                xmlChar
                                    }
                                }
                            }
                        }
                    } else { current_block = 7256998052328658819; }
                    match current_block {
                        13636304053406529094 => { }
                        _ => {
                            if len >= max {
                                temp = xmlSaveUriRealloc(ret, &mut max);
                                if temp.is_null() {
                                    current_block = 13636304053406529094;
                                } else {
                                    ret = temp;
                                    current_block = 16813369756331276724;
                                }
                            } else { current_block = 16813369756331276724; }
                            match current_block {
                                13636304053406529094 => { }
                                _ => {
                                    *ret.offset(len as isize) =
                                        0 as std::os::raw::c_int as xmlChar;
                                    return ret
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => { }
    }
    xmlFree.expect("non-null function pointer")(ret as *mut std::os::raw::c_void);
    return 0 as *mut xmlChar;
}
/* *
 * xmlPrintURI:
 * @stream:  a FILE* for the output
 * @uri:  pointer to an xmlURI
 *
 * Prints the URI in the stream @stream.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlPrintURI(mut stream: *mut FILE,
                                     mut uri: xmlURIPtr) {
    let mut out: *mut xmlChar = 0 as *mut xmlChar;
    out = xmlSaveUri(uri);
    if !out.is_null() {
        fprintf(stream, b"%s\x00" as *const u8 as *const std::os::raw::c_char,
                out as *mut std::os::raw::c_char);
        xmlFree.expect("non-null function pointer")(out as *mut std::os::raw::c_void);
    };
}
/* *
 * xmlCleanURI:
 * @uri:  pointer to an xmlURI
 *
 * Make sure the xmlURI struct is free of content
 */
unsafe extern "C" fn xmlCleanURI(mut uri: xmlURIPtr) {
    if uri.is_null() { return }
    if !(*uri).scheme.is_null() {
        xmlFree.expect("non-null function pointer")((*uri).scheme as
                                                        *mut std::os::raw::c_void);
    }
    (*uri).scheme = 0 as *mut std::os::raw::c_char;
    if !(*uri).server.is_null() {
        xmlFree.expect("non-null function pointer")((*uri).server as
                                                        *mut std::os::raw::c_void);
    }
    (*uri).server = 0 as *mut std::os::raw::c_char;
    if !(*uri).user.is_null() {
        xmlFree.expect("non-null function pointer")((*uri).user as
                                                        *mut std::os::raw::c_void);
    }
    (*uri).user = 0 as *mut std::os::raw::c_char;
    if !(*uri).path.is_null() {
        xmlFree.expect("non-null function pointer")((*uri).path as
                                                        *mut std::os::raw::c_void);
    }
    (*uri).path = 0 as *mut std::os::raw::c_char;
    if !(*uri).fragment.is_null() {
        xmlFree.expect("non-null function pointer")((*uri).fragment as
                                                        *mut std::os::raw::c_void);
    }
    (*uri).fragment = 0 as *mut std::os::raw::c_char;
    if !(*uri).opaque.is_null() {
        xmlFree.expect("non-null function pointer")((*uri).opaque as
                                                        *mut std::os::raw::c_void);
    }
    (*uri).opaque = 0 as *mut std::os::raw::c_char;
    if !(*uri).authority.is_null() {
        xmlFree.expect("non-null function pointer")((*uri).authority as
                                                        *mut std::os::raw::c_void);
    }
    (*uri).authority = 0 as *mut std::os::raw::c_char;
    if !(*uri).query.is_null() {
        xmlFree.expect("non-null function pointer")((*uri).query as
                                                        *mut std::os::raw::c_void);
    }
    (*uri).query = 0 as *mut std::os::raw::c_char;
    if !(*uri).query_raw.is_null() {
        xmlFree.expect("non-null function pointer")((*uri).query_raw as
                                                        *mut std::os::raw::c_void);
    }
    (*uri).query_raw = 0 as *mut std::os::raw::c_char;
}
/* *
 * xmlFreeURI:
 * @uri:  pointer to an xmlURI
 *
 * Free up the xmlURI struct
 */
#[no_mangle]
pub unsafe extern "C" fn xmlFreeURI(mut uri: xmlURIPtr) {
    if uri.is_null() { return }
    if !(*uri).scheme.is_null() {
        xmlFree.expect("non-null function pointer")((*uri).scheme as
                                                        *mut std::os::raw::c_void);
    }
    if !(*uri).server.is_null() {
        xmlFree.expect("non-null function pointer")((*uri).server as
                                                        *mut std::os::raw::c_void);
    }
    if !(*uri).user.is_null() {
        xmlFree.expect("non-null function pointer")((*uri).user as
                                                        *mut std::os::raw::c_void);
    }
    if !(*uri).path.is_null() {
        xmlFree.expect("non-null function pointer")((*uri).path as
                                                        *mut std::os::raw::c_void);
    }
    if !(*uri).fragment.is_null() {
        xmlFree.expect("non-null function pointer")((*uri).fragment as
                                                        *mut std::os::raw::c_void);
    }
    if !(*uri).opaque.is_null() {
        xmlFree.expect("non-null function pointer")((*uri).opaque as
                                                        *mut std::os::raw::c_void);
    }
    if !(*uri).authority.is_null() {
        xmlFree.expect("non-null function pointer")((*uri).authority as
                                                        *mut std::os::raw::c_void);
    }
    if !(*uri).query.is_null() {
        xmlFree.expect("non-null function pointer")((*uri).query as
                                                        *mut std::os::raw::c_void);
    }
    if !(*uri).query_raw.is_null() {
        xmlFree.expect("non-null function pointer")((*uri).query_raw as
                                                        *mut std::os::raw::c_void);
    }
    xmlFree.expect("non-null function pointer")(uri as *mut std::os::raw::c_void);
}
/* ***********************************************************************
 *									*
 *			Helper functions				*
 *									*
 ************************************************************************/
/* *
 * xmlNormalizeURIPath:
 * @path:  pointer to the path string
 *
 * Applies the 5 normalization steps to a path string--that is, RFC 2396
 * Section 5.2, steps 6.c through 6.g.
 *
 * Normalization occurs directly on the string, no new allocation is done
 *
 * Returns 0 or an error code
 */
#[no_mangle]
pub unsafe extern "C" fn xmlNormalizeURIPath(mut path: *mut std::os::raw::c_char)
 -> std::os::raw::c_int {
    let mut cur: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut out: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    if path.is_null() { return -(1 as std::os::raw::c_int) }
    /* Skip all initial "/" chars.  We want to get to the beginning of the
     * first non-empty segment.
     */
    cur = path;
    while *cur.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int == '/' as i32
          {
        cur = cur.offset(1)
    }
    if *cur.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int == '\u{0}' as i32
       {
        return 0 as std::os::raw::c_int
    }
    /* Keep everything we've seen so far.  */
    out = cur;
    /*
     * Analyze each segment in sequence for cases (c) and (d).
     */
    's_39:
        while *cur.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int !=
                  '\u{0}' as i32 {
            /*
	 * c) All occurrences of "./", where "." is a complete path segment,
	 *    are removed from the buffer string.
	 */
            if *cur.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                   '.' as i32 &&
                   *cur.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                       '/' as i32 {
                cur = cur.offset(2 as std::os::raw::c_int as isize);
                /* '//' normalization should be done at this point too */
                while *cur.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                          '/' as i32 {
                    cur = cur.offset(1)
                }
            } else {
                /*
	 * d) If the buffer string ends with "." as a complete path segment,
	 *    that "." is removed.
	 */
                if *cur.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                       '.' as i32 &&
                       *cur.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int
                           == '\u{0}' as i32 {
                    break ;
                }
                /* Otherwise keep the segment.  */
                while *cur.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int !=
                          '/' as i32 {
                    if *cur.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int
                           == '\u{0}' as i32 {
                        break 's_39 ;
                    }
                    let fresh57 = cur;
                    cur = cur.offset(1);
                    let fresh58 = out;
                    out = out.offset(1);
                    *fresh58.offset(0 as std::os::raw::c_int as isize) =
                        *fresh57.offset(0 as std::os::raw::c_int as isize)
                }
                /* nomalize // */
                while *cur.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                          '/' as i32 &&
                          *cur.offset(1 as std::os::raw::c_int as isize) as
                              std::os::raw::c_int == '/' as i32 {
                    cur = cur.offset(1)
                }
                let fresh59 = cur;
                cur = cur.offset(1);
                let fresh60 = out;
                out = out.offset(1);
                *fresh60.offset(0 as std::os::raw::c_int as isize) =
                    *fresh59.offset(0 as std::os::raw::c_int as isize)
            }
        }
    *out.offset(0 as std::os::raw::c_int as isize) = '\u{0}' as i32 as std::os::raw::c_char;
    /* Reset to the beginning of the first segment for the next sequence.  */
    cur = path;
    while *cur.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int == '/' as i32
          {
        cur = cur.offset(1)
    }
    if *cur.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int == '\u{0}' as i32
       {
        return 0 as std::os::raw::c_int
    }
    loop 
         /*
     * Analyze each segment in sequence for cases (e) and (f).
     *
     * e) All occurrences of "<segment>/../", where <segment> is a
     *    complete path segment not equal to "..", are removed from the
     *    buffer string.  Removal of these path segments is performed
     *    iteratively, removing the leftmost matching pattern on each
     *    iteration, until no matching pattern remains.
     *
     * f) If the buffer string ends with "<segment>/..", where <segment>
     *    is a complete path segment not equal to "..", that
     *    "<segment>/.." is removed.
     *
     * To satisfy the "iterative" clause in (e), we need to collapse the
     * string every time we find something that needs to be removed.  Thus,
     * we don't need to keep two pointers into the string: we only need a
     * "current position" pointer.
     */
         {
        let mut segp: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
        let mut tmp: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
        /* At the beginning of each iteration of this loop, "cur" points to
         * the first character of the segment we want to examine.
         */
        /* Find the end of the current segment.  */
        segp = cur;
        while *segp.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int !=
                  '/' as i32 &&
                  *segp.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int !=
                      '\u{0}' as i32 {
            segp = segp.offset(1)
        }
        /* If this is the last segment, we're done (we need at least two
         * segments to meet the criteria for the (e) and (f) cases).
         */
        if *segp.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
               '\u{0}' as i32 {
            break ;
        }
        /* If the first segment is "..", or if the next segment _isn't_ "..",
         * keep this segment and try the next one.
         */
        segp = segp.offset(1);
        if *cur.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int == '.' as i32
               &&
               *cur.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                   '.' as i32 && segp == cur.offset(3 as std::os::raw::c_int as isize)
               ||
               (*segp.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int !=
                    '.' as i32 ||
                    *segp.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int !=
                        '.' as i32 ||
                    *segp.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int !=
                        '/' as i32 &&
                        *segp.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int
                            != '\u{0}' as i32) {
            cur = segp
        } else if *segp.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                      '\u{0}' as i32 {
            *cur.offset(0 as std::os::raw::c_int as isize) =
                '\u{0}' as i32 as std::os::raw::c_char;
            break ;
        } else {
            /* If we get here, remove this segment and the next one and back up
         * to the previous segment (if there is one), to implement the
         * "iteratively" clause.  It's pretty much impossible to back up
         * while maintaining two pointers into the buffer, so just compact
         * the whole buffer now.
         */
            /* If this is the end of the buffer, we're done.  */
            /* Valgrind complained, strcpy(cur, segp + 3); */
        /* string will overlap, do not use strcpy */
            tmp = cur;
            segp = segp.offset(3 as std::os::raw::c_int as isize);
            loop  {
                let fresh61 = segp;
                segp = segp.offset(1);
                let fresh62 = tmp;
                tmp = tmp.offset(1);
                *fresh62 = *fresh61;
                if !(*fresh62 as std::os::raw::c_int != 0 as std::os::raw::c_int) { break ; }
            }
            /* If there are no previous segments, then keep going from here.  */
            segp = cur;
            while segp > path &&
                      {
                          segp = segp.offset(-1);
                          (*segp.offset(0 as std::os::raw::c_int as isize) as
                               std::os::raw::c_int) == '/' as i32
                      } {
            }
            if segp == path { continue ; }
            /* "segp" is pointing to the end of a previous segment; find it's
         * start.  We need to back up to the previous segment and start
         * over with that to handle things like "foo/bar/../..".  If we
         * don't do this, then on the first pass we'll remove the "bar/..",
         * but be pointing at the second ".." so we won't realize we can also
         * remove the "foo/..".
         */
            cur = segp;
            while cur > path &&
                      *cur.offset(-(1 as std::os::raw::c_int) as isize) as std::os::raw::c_int
                          != '/' as i32 {
                cur = cur.offset(-1)
            }
        }
    }
    *out.offset(0 as std::os::raw::c_int as isize) = '\u{0}' as i32 as std::os::raw::c_char;
    /*
     * g) If the resulting buffer string still begins with one or more
     *    complete path segments of "..", then the reference is
     *    considered to be in error. Implementations may handle this
     *    error by retaining these components in the resolved path (i.e.,
     *    treating them as part of the final URI), by removing them from
     *    the resolved path (i.e., discarding relative levels above the
     *    root), or by avoiding traversal of the reference.
     *
     * We discard them from the final path.
     */
    if *path.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int == '/' as i32 {
        cur = path;
        while *cur.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                  '/' as i32 &&
                  *cur.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                      '.' as i32 &&
                  *cur.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                      '.' as i32 &&
                  (*cur.offset(3 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                       '/' as i32 ||
                       *cur.offset(3 as std::os::raw::c_int as isize) as std::os::raw::c_int
                           == '\u{0}' as i32) {
            cur = cur.offset(3 as std::os::raw::c_int as isize)
        }
        if cur != path {
            out = path;
            while *cur.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int !=
                      '\u{0}' as i32 {
                let fresh63 = cur;
                cur = cur.offset(1);
                let fresh64 = out;
                out = out.offset(1);
                *fresh64.offset(0 as std::os::raw::c_int as isize) =
                    *fresh63.offset(0 as std::os::raw::c_int as isize)
            }
            *out.offset(0 as std::os::raw::c_int as isize) =
                0 as std::os::raw::c_int as std::os::raw::c_char
        }
    }
    return 0 as std::os::raw::c_int;
}
unsafe extern "C" fn is_hex(mut c: std::os::raw::c_char) -> std::os::raw::c_int {
    if c as std::os::raw::c_int >= '0' as i32 && c as std::os::raw::c_int <= '9' as i32 ||
           c as std::os::raw::c_int >= 'a' as i32 && c as std::os::raw::c_int <= 'f' as i32 ||
           c as std::os::raw::c_int >= 'A' as i32 && c as std::os::raw::c_int <= 'F' as i32 {
        return 1 as std::os::raw::c_int
    }
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlURIUnescapeString:
 * @str:  the string to unescape
 * @len:   the length in bytes to unescape (or <= 0 to indicate full string)
 * @target:  optional destination buffer
 *
 * Unescaping routine, but does not check that the string is an URI. The
 * output is a direct unsigned char translation of %XX values (no encoding)
 * Note that the length of the result can only be smaller or same size as
 * the input string.
 *
 * Returns a copy of the string, but unescaped, will return NULL only in case
 * of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlURIUnescapeString(mut str: *const std::os::raw::c_char,
                                              mut len: std::os::raw::c_int,
                                              mut target: *mut std::os::raw::c_char)
 -> *mut std::os::raw::c_char {
    let mut ret: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut out: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut in_0: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    if str.is_null() { return 0 as *mut std::os::raw::c_char }
    if len <= 0 as std::os::raw::c_int { len = strlen(str) as std::os::raw::c_int }
    if len < 0 as std::os::raw::c_int { return 0 as *mut std::os::raw::c_char }
    if target.is_null() {
        ret =
            xmlMallocAtomic.expect("non-null function pointer")((len +
                                                                     1 as
                                                                         std::os::raw::c_int)
                                                                    as size_t)
                as *mut std::os::raw::c_char;
        if ret.is_null() {
            xmlURIErrMemory(b"unescaping URI value\n\x00" as *const u8 as
                                *const std::os::raw::c_char);
            return 0 as *mut std::os::raw::c_char
        }
    } else { ret = target }
    in_0 = str;
    out = ret;
    while len > 0 as std::os::raw::c_int {
        if len > 2 as std::os::raw::c_int && *in_0 as std::os::raw::c_int == '%' as i32 &&
               is_hex(*in_0.offset(1 as std::os::raw::c_int as isize)) != 0 &&
               is_hex(*in_0.offset(2 as std::os::raw::c_int as isize)) != 0 {
            in_0 = in_0.offset(1);
            if *in_0 as std::os::raw::c_int >= '0' as i32 &&
                   *in_0 as std::os::raw::c_int <= '9' as i32 {
                *out = (*in_0 as std::os::raw::c_int - '0' as i32) as std::os::raw::c_char
            } else if *in_0 as std::os::raw::c_int >= 'a' as i32 &&
                          *in_0 as std::os::raw::c_int <= 'f' as i32 {
                *out =
                    (*in_0 as std::os::raw::c_int - 'a' as i32 + 10 as std::os::raw::c_int) as
                        std::os::raw::c_char
            } else if *in_0 as std::os::raw::c_int >= 'A' as i32 &&
                          *in_0 as std::os::raw::c_int <= 'F' as i32 {
                *out =
                    (*in_0 as std::os::raw::c_int - 'A' as i32 + 10 as std::os::raw::c_int) as
                        std::os::raw::c_char
            }
            in_0 = in_0.offset(1);
            if *in_0 as std::os::raw::c_int >= '0' as i32 &&
                   *in_0 as std::os::raw::c_int <= '9' as i32 {
                *out =
                    (*out as std::os::raw::c_int * 16 as std::os::raw::c_int +
                         (*in_0 as std::os::raw::c_int - '0' as i32)) as std::os::raw::c_char
            } else if *in_0 as std::os::raw::c_int >= 'a' as i32 &&
                          *in_0 as std::os::raw::c_int <= 'f' as i32 {
                *out =
                    (*out as std::os::raw::c_int * 16 as std::os::raw::c_int +
                         (*in_0 as std::os::raw::c_int - 'a' as i32) +
                         10 as std::os::raw::c_int) as std::os::raw::c_char
            } else if *in_0 as std::os::raw::c_int >= 'A' as i32 &&
                          *in_0 as std::os::raw::c_int <= 'F' as i32 {
                *out =
                    (*out as std::os::raw::c_int * 16 as std::os::raw::c_int +
                         (*in_0 as std::os::raw::c_int - 'A' as i32) +
                         10 as std::os::raw::c_int) as std::os::raw::c_char
            }
            in_0 = in_0.offset(1);
            len -= 3 as std::os::raw::c_int;
            out = out.offset(1)
        } else {
            let fresh65 = in_0;
            in_0 = in_0.offset(1);
            let fresh66 = out;
            out = out.offset(1);
            *fresh66 = *fresh65;
            len -= 1
        }
    }
    *out = 0 as std::os::raw::c_int as std::os::raw::c_char;
    return ret;
}
/* *
 * xmlURIEscapeStr:
 * @str:  string to escape
 * @list: exception list string of chars not to escape
 *
 * This routine escapes a string to hex, ignoring reserved characters (a-z)
 * and the characters in the exception list.
 *
 * Returns a new escaped string or NULL in case of error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlURIEscapeStr(mut str: *const xmlChar,
                                         mut list: *const xmlChar)
 -> *mut xmlChar {
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    let mut ch: xmlChar = 0;
    let mut temp: *mut xmlChar = 0 as *mut xmlChar;
    let mut in_0: *const xmlChar = 0 as *const xmlChar;
    let mut len: std::os::raw::c_int = 0;
    let mut out: std::os::raw::c_int = 0;
    if str.is_null() { return 0 as *mut xmlChar }
    if *str.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
           0 as std::os::raw::c_int {
        return xmlStrdup(str)
    }
    len = xmlStrlen(str);
    if !(len > 0 as std::os::raw::c_int) { return 0 as *mut xmlChar }
    len += 20 as std::os::raw::c_int;
    ret =
        xmlMallocAtomic.expect("non-null function pointer")(len as size_t) as
            *mut xmlChar;
    if ret.is_null() {
        xmlURIErrMemory(b"escaping URI value\n\x00" as *const u8 as
                            *const std::os::raw::c_char);
        return 0 as *mut xmlChar
    }
    in_0 = str;
    out = 0 as std::os::raw::c_int;
    while *in_0 as std::os::raw::c_int != 0 as std::os::raw::c_int {
        if len - out <= 3 as std::os::raw::c_int {
            temp = xmlSaveUriRealloc(ret, &mut len);
            if temp.is_null() {
                xmlURIErrMemory(b"escaping URI value\n\x00" as *const u8 as
                                    *const std::os::raw::c_char);
                xmlFree.expect("non-null function pointer")(ret as
                                                                *mut std::os::raw::c_void);
                return 0 as *mut xmlChar
            }
            ret = temp
        }
        ch = *in_0;
        if ch as std::os::raw::c_int != '@' as i32 &&
               !(ch as std::os::raw::c_int >= 'a' as i32 &&
                     ch as std::os::raw::c_int <= 'z' as i32 ||
                     ch as std::os::raw::c_int >= 'A' as i32 &&
                         ch as std::os::raw::c_int <= 'Z' as i32 ||
                     ch as std::os::raw::c_int >= '0' as i32 &&
                         ch as std::os::raw::c_int <= '9' as i32 ||
                     (ch as std::os::raw::c_int == '-' as i32 ||
                          ch as std::os::raw::c_int == '_' as i32 ||
                          ch as std::os::raw::c_int == '.' as i32 ||
                          ch as std::os::raw::c_int == '!' as i32 ||
                          ch as std::os::raw::c_int == '~' as i32 ||
                          ch as std::os::raw::c_int == '*' as i32 ||
                          ch as std::os::raw::c_int == '\'' as i32 ||
                          ch as std::os::raw::c_int == '(' as i32 ||
                          ch as std::os::raw::c_int == ')' as i32)) &&
               xmlStrchr(list, ch).is_null() {
            let mut val: std::os::raw::c_uchar = 0;
            let fresh67 = out;
            out = out + 1;
            *ret.offset(fresh67 as isize) = '%' as i32 as xmlChar;
            val = (ch as std::os::raw::c_int >> 4 as std::os::raw::c_int) as std::os::raw::c_uchar;
            if val as std::os::raw::c_int <= 9 as std::os::raw::c_int {
                let fresh68 = out;
                out = out + 1;
                *ret.offset(fresh68 as isize) =
                    ('0' as i32 + val as std::os::raw::c_int) as xmlChar
            } else {
                let fresh69 = out;
                out = out + 1;
                *ret.offset(fresh69 as isize) =
                    ('A' as i32 + val as std::os::raw::c_int - 0xa as std::os::raw::c_int) as
                        xmlChar
            }
            val = (ch as std::os::raw::c_int & 0xf as std::os::raw::c_int) as std::os::raw::c_uchar;
            if val as std::os::raw::c_int <= 9 as std::os::raw::c_int {
                let fresh70 = out;
                out = out + 1;
                *ret.offset(fresh70 as isize) =
                    ('0' as i32 + val as std::os::raw::c_int) as xmlChar
            } else {
                let fresh71 = out;
                out = out + 1;
                *ret.offset(fresh71 as isize) =
                    ('A' as i32 + val as std::os::raw::c_int - 0xa as std::os::raw::c_int) as
                        xmlChar
            }
            in_0 = in_0.offset(1)
        } else {
            let fresh72 = in_0;
            in_0 = in_0.offset(1);
            let fresh73 = out;
            out = out + 1;
            *ret.offset(fresh73 as isize) = *fresh72
        }
    }
    *ret.offset(out as isize) = 0 as std::os::raw::c_int as xmlChar;
    return ret;
}
/* *
 * xmlURIEscape:
 * @str:  the string of the URI to escape
 *
 * Escaping routine, does not do validity checks !
 * It will try to escape the chars needing this, but this is heuristic
 * based it's impossible to be sure.
 *
 * Returns an copy of the string, but escaped
 *
 * 25 May 2001
 * Uses xmlParseURI and xmlURIEscapeStr to try to escape correctly
 * according to RFC2396.
 *   - Carl Douglas
 */
#[no_mangle]
pub unsafe extern "C" fn xmlURIEscape(mut str: *const xmlChar)
 -> *mut xmlChar {
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    let mut segment: *mut xmlChar = 0 as *mut xmlChar;
    let mut uri: xmlURIPtr = 0 as *mut xmlURI;
    let mut ret2: std::os::raw::c_int = 0;
    if str.is_null() { return 0 as *mut xmlChar }
    uri = xmlCreateURI();
    if !uri.is_null() {
        /*
	 * Allow escaping errors in the unescaped form
	 */
        (*uri).cleanup = 1 as std::os::raw::c_int;
        ret2 = xmlParseURIReference(uri, str as *const std::os::raw::c_char);
        if ret2 != 0 { xmlFreeURI(uri); return 0 as *mut xmlChar }
    }
    if uri.is_null() { return 0 as *mut xmlChar }
    ret = 0 as *mut xmlChar;
    if !(*uri).scheme.is_null() {
        segment =
            xmlURIEscapeStr((*uri).scheme as *mut xmlChar,
                            b"+-.\x00" as *const u8 as *const std::os::raw::c_char as
                                *mut xmlChar);
        if segment.is_null() {
            xmlURIErrMemory(b"escaping URI value\n\x00" as *const u8 as
                                *const std::os::raw::c_char);
            xmlFreeURI(uri);
            return 0 as *mut xmlChar
        }
        ret = xmlStrcat(ret, segment);
        ret =
            xmlStrcat(ret,
                      b":\x00" as *const u8 as *const std::os::raw::c_char as
                          *mut xmlChar);
        xmlFree.expect("non-null function pointer")(segment as
                                                        *mut std::os::raw::c_void);
    }
    if !(*uri).authority.is_null() {
        segment =
            xmlURIEscapeStr((*uri).authority as *mut xmlChar,
                            b"/?;:@\x00" as *const u8 as *const std::os::raw::c_char
                                as *mut xmlChar);
        if segment.is_null() {
            xmlURIErrMemory(b"escaping URI value\n\x00" as *const u8 as
                                *const std::os::raw::c_char);
            xmlFreeURI(uri);
            return 0 as *mut xmlChar
        }
        ret =
            xmlStrcat(ret,
                      b"//\x00" as *const u8 as *const std::os::raw::c_char as
                          *mut xmlChar);
        ret = xmlStrcat(ret, segment);
        xmlFree.expect("non-null function pointer")(segment as
                                                        *mut std::os::raw::c_void);
    }
    if !(*uri).user.is_null() {
        segment =
            xmlURIEscapeStr((*uri).user as *mut xmlChar,
                            b";:&=+$,\x00" as *const u8 as *const std::os::raw::c_char
                                as *mut xmlChar);
        if segment.is_null() {
            xmlURIErrMemory(b"escaping URI value\n\x00" as *const u8 as
                                *const std::os::raw::c_char);
            xmlFreeURI(uri);
            return 0 as *mut xmlChar
        }
        ret =
            xmlStrcat(ret,
                      b"//\x00" as *const u8 as *const std::os::raw::c_char as
                          *mut xmlChar);
        ret = xmlStrcat(ret, segment);
        ret =
            xmlStrcat(ret,
                      b"@\x00" as *const u8 as *const std::os::raw::c_char as
                          *mut xmlChar);
        xmlFree.expect("non-null function pointer")(segment as
                                                        *mut std::os::raw::c_void);
    }
    if !(*uri).server.is_null() {
        segment =
            xmlURIEscapeStr((*uri).server as *mut xmlChar,
                            b"/?;:@\x00" as *const u8 as *const std::os::raw::c_char
                                as *mut xmlChar);
        if segment.is_null() {
            xmlURIErrMemory(b"escaping URI value\n\x00" as *const u8 as
                                *const std::os::raw::c_char);
            xmlFreeURI(uri);
            return 0 as *mut xmlChar
        }
        if (*uri).user.is_null() {
            ret =
                xmlStrcat(ret,
                          b"//\x00" as *const u8 as *const std::os::raw::c_char as
                              *mut xmlChar)
        }
        ret = xmlStrcat(ret, segment);
        xmlFree.expect("non-null function pointer")(segment as
                                                        *mut std::os::raw::c_void);
    }
    if (*uri).port != 0 {
        let mut port: [xmlChar; 10] = [0; 10];
        snprintf(port.as_mut_ptr() as *mut std::os::raw::c_char,
                 10 as std::os::raw::c_int as std::os::raw::c_ulong,
                 b"%d\x00" as *const u8 as *const std::os::raw::c_char, (*uri).port);
        ret =
            xmlStrcat(ret,
                      b":\x00" as *const u8 as *const std::os::raw::c_char as
                          *mut xmlChar);
        ret = xmlStrcat(ret, port.as_mut_ptr())
    }
    if !(*uri).path.is_null() {
        segment =
            xmlURIEscapeStr((*uri).path as *mut xmlChar,
                            b":@&=+$,/?;\x00" as *const u8 as
                                *const std::os::raw::c_char as *mut xmlChar);
        if segment.is_null() {
            xmlURIErrMemory(b"escaping URI value\n\x00" as *const u8 as
                                *const std::os::raw::c_char);
            xmlFreeURI(uri);
            return 0 as *mut xmlChar
        }
        ret = xmlStrcat(ret, segment);
        xmlFree.expect("non-null function pointer")(segment as
                                                        *mut std::os::raw::c_void);
    }
    if !(*uri).query_raw.is_null() {
        ret =
            xmlStrcat(ret,
                      b"?\x00" as *const u8 as *const std::os::raw::c_char as
                          *mut xmlChar);
        ret = xmlStrcat(ret, (*uri).query_raw as *mut xmlChar)
    } else if !(*uri).query.is_null() {
        segment =
            xmlURIEscapeStr((*uri).query as *mut xmlChar,
                            b";/?:@&=+,$\x00" as *const u8 as
                                *const std::os::raw::c_char as *mut xmlChar);
        if segment.is_null() {
            xmlURIErrMemory(b"escaping URI value\n\x00" as *const u8 as
                                *const std::os::raw::c_char);
            xmlFreeURI(uri);
            return 0 as *mut xmlChar
        }
        ret =
            xmlStrcat(ret,
                      b"?\x00" as *const u8 as *const std::os::raw::c_char as
                          *mut xmlChar);
        ret = xmlStrcat(ret, segment);
        xmlFree.expect("non-null function pointer")(segment as
                                                        *mut std::os::raw::c_void);
    }
    if !(*uri).opaque.is_null() {
        segment =
            xmlURIEscapeStr((*uri).opaque as *mut xmlChar,
                            b"\x00" as *const u8 as *const std::os::raw::c_char as
                                *mut xmlChar);
        if segment.is_null() {
            xmlURIErrMemory(b"escaping URI value\n\x00" as *const u8 as
                                *const std::os::raw::c_char);
            xmlFreeURI(uri);
            return 0 as *mut xmlChar
        }
        ret = xmlStrcat(ret, segment);
        xmlFree.expect("non-null function pointer")(segment as
                                                        *mut std::os::raw::c_void);
    }
    if !(*uri).fragment.is_null() {
        segment =
            xmlURIEscapeStr((*uri).fragment as *mut xmlChar,
                            b"#\x00" as *const u8 as *const std::os::raw::c_char as
                                *mut xmlChar);
        if segment.is_null() {
            xmlURIErrMemory(b"escaping URI value\n\x00" as *const u8 as
                                *const std::os::raw::c_char);
            xmlFreeURI(uri);
            return 0 as *mut xmlChar
        }
        ret =
            xmlStrcat(ret,
                      b"#\x00" as *const u8 as *const std::os::raw::c_char as
                          *mut xmlChar);
        ret = xmlStrcat(ret, segment);
        xmlFree.expect("non-null function pointer")(segment as
                                                        *mut std::os::raw::c_void);
    }
    xmlFreeURI(uri);
    return ret;
}
/* ***********************************************************************
 *									*
 *			Public functions				*
 *									*
 ************************************************************************/
/* *
 * xmlBuildURI:
 * @URI:  the URI instance found in the document
 * @base:  the base value
 *
 * Computes he final URI of the reference done by checking that
 * the given URI is valid, and building the final URI using the
 * base URI. This is processed according to section 5.2 of the
 * RFC 2396
 *
 * 5.2. Resolving Relative References to Absolute Form
 *
 * Returns a new URI string (to be freed by the caller) or NULL in case
 *         of error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlBuildURI(mut URI: *const xmlChar,
                                     mut base: *const xmlChar)
 -> *mut xmlChar {
    let mut current_block: u64;
    let mut val: *mut xmlChar = 0 as *mut xmlChar;
    let mut ret: std::os::raw::c_int = 0;
    let mut len: std::os::raw::c_int = 0;
    let mut indx: std::os::raw::c_int = 0;
    let mut cur: std::os::raw::c_int = 0;
    let mut out: std::os::raw::c_int = 0;
    let mut ref_0: xmlURIPtr = 0 as xmlURIPtr;
    let mut bas: xmlURIPtr = 0 as xmlURIPtr;
    let mut res: xmlURIPtr = 0 as xmlURIPtr;
    /*
     * 1) The URI reference is parsed into the potential four components and
     *    fragment identifier, as described in Section 4.3.
     *
     *    NOTE that a completely empty URI is treated by modern browsers
     *    as a reference to "." rather than as a synonym for the current
     *    URI.  Should we do that here?
     */
    if URI.is_null() {
        ret = -(1 as std::os::raw::c_int);
        current_block = 13109137661213826276;
    } else if *URI != 0 {
        ref_0 = xmlCreateURI();
        if ref_0.is_null() {
            current_block = 15027794710874555033;
        } else {
            ret = xmlParseURIReference(ref_0, URI as *const std::os::raw::c_char);
            current_block = 13109137661213826276;
        }
    } else { ret = 0 as std::os::raw::c_int; current_block = 13109137661213826276; }
    match current_block {
        13109137661213826276 => {
            if !(ret != 0 as std::os::raw::c_int) {
                if !ref_0.is_null() && !(*ref_0).scheme.is_null() {
                    /*
	 * The URI is absolute don't modify.
	 */
                    val = xmlStrdup(URI)
                } else {
                    if base.is_null() {
                        ret = -(1 as std::os::raw::c_int);
                        current_block = 7175849428784450219;
                    } else {
                        bas = xmlCreateURI();
                        if bas.is_null() {
                            current_block = 15027794710874555033;
                        } else {
                            ret =
                                xmlParseURIReference(bas,
                                                     base as
                                                         *const std::os::raw::c_char);
                            current_block = 7175849428784450219;
                        }
                    }
                    match current_block {
                        15027794710874555033 => { }
                        _ => {
                            if ret != 0 as std::os::raw::c_int {
                                if !ref_0.is_null() {
                                    val = xmlSaveUri(ref_0)
                                }
                            } else if ref_0.is_null() {
                                /*
	 * the base fragment must be ignored
	 */
                                if !(*bas).fragment.is_null() {
                                    xmlFree.expect("non-null function pointer")((*bas).fragment
                                                                                    as
                                                                                    *mut std::os::raw::c_void);
                                    (*bas).fragment = 0 as *mut std::os::raw::c_char
                                }
                                val = xmlSaveUri(bas)
                            } else {
                                /*
     * 2) If the path component is empty and the scheme, authority, and
     *    query components are undefined, then it is a reference to the
     *    current document and we are done.  Otherwise, the reference URI's
     *    query and fragment components are defined as found (or not found)
     *    within the URI reference and not inherited from the base URI.
     *
     *    NOTE that in modern browsers, the parsing differs from the above
     *    in the following aspect:  the query component is allowed to be
     *    defined while still treating this as a reference to the current
     *    document.
     */
                                res = xmlCreateURI();
                                if !res.is_null() {
                                    if (*ref_0).scheme.is_null() &&
                                           (*ref_0).path.is_null() &&
                                           ((*ref_0).authority.is_null() &&
                                                (*ref_0).server.is_null()) {
                                        if !(*bas).scheme.is_null() {
                                            (*res).scheme =
                                                xmlMemStrdup.expect("non-null function pointer")((*bas).scheme)
                                        }
                                        if !(*bas).authority.is_null() {
                                            (*res).authority =
                                                xmlMemStrdup.expect("non-null function pointer")((*bas).authority)
                                        } else if !(*bas).server.is_null() ||
                                                      (*bas).port ==
                                                          -(1 as std::os::raw::c_int)
                                         {
                                            if !(*bas).server.is_null() {
                                                (*res).server =
                                                    xmlMemStrdup.expect("non-null function pointer")((*bas).server)
                                            }
                                            if !(*bas).user.is_null() {
                                                (*res).user =
                                                    xmlMemStrdup.expect("non-null function pointer")((*bas).user)
                                            }
                                            (*res).port = (*bas).port
                                        }
                                        if !(*bas).path.is_null() {
                                            (*res).path =
                                                xmlMemStrdup.expect("non-null function pointer")((*bas).path)
                                        }
                                        if !(*ref_0).query_raw.is_null() {
                                            (*res).query_raw =
                                                xmlMemStrdup.expect("non-null function pointer")((*ref_0).query_raw)
                                        } else if !(*ref_0).query.is_null() {
                                            (*res).query =
                                                xmlMemStrdup.expect("non-null function pointer")((*ref_0).query)
                                        } else if !(*bas).query_raw.is_null()
                                         {
                                            (*res).query_raw =
                                                xmlMemStrdup.expect("non-null function pointer")((*bas).query_raw)
                                        } else if !(*bas).query.is_null() {
                                            (*res).query =
                                                xmlMemStrdup.expect("non-null function pointer")((*bas).query)
                                        }
                                        if !(*ref_0).fragment.is_null() {
                                            (*res).fragment =
                                                xmlMemStrdup.expect("non-null function pointer")((*ref_0).fragment)
                                        }
                                        current_block = 11150479873881321503;
                                    } else if !(*ref_0).scheme.is_null() {
                                        val = xmlSaveUri(ref_0);
                                        current_block = 15027794710874555033;
                                    } else {
                                        if !(*bas).scheme.is_null() {
                                            (*res).scheme =
                                                xmlMemStrdup.expect("non-null function pointer")((*bas).scheme)
                                        }
                                        if !(*ref_0).query_raw.is_null() {
                                            (*res).query_raw =
                                                xmlMemStrdup.expect("non-null function pointer")((*ref_0).query_raw)
                                        } else if !(*ref_0).query.is_null() {
                                            (*res).query =
                                                xmlMemStrdup.expect("non-null function pointer")((*ref_0).query)
                                        }
                                        if !(*ref_0).fragment.is_null() {
                                            (*res).fragment =
                                                xmlMemStrdup.expect("non-null function pointer")((*ref_0).fragment)
                                        }
                                        /*
     * 3) If the scheme component is defined, indicating that the reference
     *    starts with a scheme name, then the reference is interpreted as an
     *    absolute URI and we are done.  Otherwise, the reference URI's
     *    scheme is inherited from the base URI's scheme component.
     */
                                        /*
     * 4) If the authority component is defined, then the reference is a
     *    network-path and we skip to step 7.  Otherwise, the reference
     *    URI's authority is inherited from the base URI's authority
     *    component, which will also be undefined if the URI scheme does not
     *    use an authority component.
     */
                                        if !(*ref_0).authority.is_null() ||
                                               !(*ref_0).server.is_null() {
                                            if !(*ref_0).authority.is_null() {
                                                (*res).authority =
                                                    xmlMemStrdup.expect("non-null function pointer")((*ref_0).authority)
                                            } else {
                                                (*res).server =
                                                    xmlMemStrdup.expect("non-null function pointer")((*ref_0).server);
                                                if !(*ref_0).user.is_null() {
                                                    (*res).user =
                                                        xmlMemStrdup.expect("non-null function pointer")((*ref_0).user)
                                                }
                                                (*res).port = (*ref_0).port
                                            }
                                            if !(*ref_0).path.is_null() {
                                                (*res).path =
                                                    xmlMemStrdup.expect("non-null function pointer")((*ref_0).path)
                                            }
                                            current_block =
                                                11150479873881321503;
                                        } else {
                                            if !(*bas).authority.is_null() {
                                                (*res).authority =
                                                    xmlMemStrdup.expect("non-null function pointer")((*bas).authority)
                                            } else if !(*bas).server.is_null()
                                                          ||
                                                          (*bas).port ==
                                                              -(1 as
                                                                    std::os::raw::c_int)
                                             {
                                                if !(*bas).server.is_null() {
                                                    (*res).server =
                                                        xmlMemStrdup.expect("non-null function pointer")((*bas).server)
                                                }
                                                if !(*bas).user.is_null() {
                                                    (*res).user =
                                                        xmlMemStrdup.expect("non-null function pointer")((*bas).user)
                                                }
                                                (*res).port = (*bas).port
                                            }
                                            /*
     * 5) If the path component begins with a slash character ("/"), then
     *    the reference is an absolute-path and we skip to step 7.
     */
                                            if !(*ref_0).path.is_null() &&
                                                   *(*ref_0).path.offset(0 as
                                                                             std::os::raw::c_int
                                                                             as
                                                                             isize)
                                                       as std::os::raw::c_int ==
                                                       '/' as i32 {
                                                (*res).path =
                                                    xmlMemStrdup.expect("non-null function pointer")((*ref_0).path);
                                                current_block =
                                                    11150479873881321503;
                                            } else {
                                                /*
     * 6) If this step is reached, then we are resolving a relative-path
     *    reference.  The relative path needs to be merged with the base
     *    URI's path.  Although there are many ways to do this, we will
     *    describe a simple method using a separate string buffer.
     *
     * Allocate a buffer large enough for the result string.
     */
                                                len =
                                                    2 as
                                                        std::os::raw::c_int; /* extra / and 0 */
                                                if !(*ref_0).path.is_null() {
                                                    len =
                                                        (len as
                                                             std::os::raw::c_ulong).wrapping_add(strlen((*ref_0).path))
                                                            as std::os::raw::c_int as
                                                            std::os::raw::c_int
                                                }
                                                if !(*bas).path.is_null() {
                                                    len =
                                                        (len as
                                                             std::os::raw::c_ulong).wrapping_add(strlen((*bas).path))
                                                            as std::os::raw::c_int as
                                                            std::os::raw::c_int
                                                }
                                                (*res).path =
                                                    xmlMallocAtomic.expect("non-null function pointer")(len
                                                                                                            as
                                                                                                            size_t)
                                                        as *mut std::os::raw::c_char;
                                                if (*res).path.is_null() {
                                                    xmlURIErrMemory(b"resolving URI against base\n\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const std::os::raw::c_char);
                                                    current_block =
                                                        15027794710874555033;
                                                } else {
                                                    *(*res).path.offset(0 as
                                                                            std::os::raw::c_int
                                                                            as
                                                                            isize)
                                                        =
                                                        0 as std::os::raw::c_int as
                                                            std::os::raw::c_char;
                                                    /*
     * a) All but the last segment of the base URI's path component is
     *    copied to the buffer.  In other words, any characters after the
     *    last (right-most) slash character, if any, are excluded.
     */
                                                    cur = 0 as std::os::raw::c_int;
                                                    out = 0 as std::os::raw::c_int;
                                                    if !(*bas).path.is_null()
                                                       {
                                                        while *(*bas).path.offset(cur
                                                                                      as
                                                                                      isize)
                                                                  as
                                                                  std::os::raw::c_int
                                                                  !=
                                                                  0 as
                                                                      std::os::raw::c_int
                                                              {
                                                            while *(*bas).path.offset(cur
                                                                                          as
                                                                                          isize)
                                                                      as
                                                                      std::os::raw::c_int
                                                                      !=
                                                                      0 as
                                                                          std::os::raw::c_int
                                                                      &&
                                                                      *(*bas).path.offset(cur
                                                                                              as
                                                                                              isize)
                                                                          as
                                                                          std::os::raw::c_int
                                                                          !=
                                                                          '/'
                                                                              as
                                                                              i32
                                                                  {
                                                                cur += 1
                                                            }
                                                            if *(*bas).path.offset(cur
                                                                                       as
                                                                                       isize)
                                                                   as
                                                                   std::os::raw::c_int
                                                                   ==
                                                                   0 as
                                                                       std::os::raw::c_int
                                                               {
                                                                break ;
                                                            }
                                                            cur += 1;
                                                            while out < cur {
                                                                *(*res).path.offset(out
                                                                                        as
                                                                                        isize)
                                                                    =
                                                                    *(*bas).path.offset(out
                                                                                            as
                                                                                            isize);
                                                                out += 1
                                                            }
                                                        }
                                                    }
                                                    *(*res).path.offset(out as
                                                                            isize)
                                                        =
                                                        0 as std::os::raw::c_int as
                                                            std::os::raw::c_char;
                                                    /*
     * b) The reference's path component is appended to the buffer
     *    string.
     */
                                                    if !(*ref_0).path.is_null()
                                                           &&
                                                           *(*ref_0).path.offset(0
                                                                                     as
                                                                                     std::os::raw::c_int
                                                                                     as
                                                                                     isize)
                                                               as std::os::raw::c_int
                                                               !=
                                                               0 as
                                                                   std::os::raw::c_int
                                                       {
                                                        indx =
                                                            0 as std::os::raw::c_int;
                                                        /*
	 * Ensure the path includes a '/'
	 */
                                                        if out ==
                                                               0 as
                                                                   std::os::raw::c_int
                                                               &&
                                                               !(*bas).server.is_null()
                                                           {
                                                            let fresh74 = out;
                                                            out = out + 1;
                                                            *(*res).path.offset(fresh74
                                                                                    as
                                                                                    isize)
                                                                =
                                                                '/' as i32 as
                                                                    std::os::raw::c_char
                                                        }
                                                        while *(*ref_0).path.offset(indx
                                                                                        as
                                                                                        isize)
                                                                  as
                                                                  std::os::raw::c_int
                                                                  !=
                                                                  0 as
                                                                      std::os::raw::c_int
                                                              {
                                                            let fresh75 =
                                                                indx;
                                                            indx = indx + 1;
                                                            let fresh76 = out;
                                                            out = out + 1;
                                                            *(*res).path.offset(fresh76
                                                                                    as
                                                                                    isize)
                                                                =
                                                                *(*ref_0).path.offset(fresh75
                                                                                          as
                                                                                          isize)
                                                        }
                                                    }
                                                    *(*res).path.offset(out as
                                                                            isize)
                                                        =
                                                        0 as std::os::raw::c_int as
                                                            std::os::raw::c_char;
                                                    /*
     * Steps c) to h) are really path normalization steps
     */
                                                    xmlNormalizeURIPath((*res).path);
                                                    current_block =
                                                        11150479873881321503;
                                                }
                                            }
                                        }
                                    }
                                    match current_block {
                                        15027794710874555033 => { }
                                        _ => {
                                            /*
     * 7) The resulting URI components, including any inherited from the
     *    base URI, are recombined to give the absolute form of the URI
     *    reference.
     */
                                            val = xmlSaveUri(res)
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => { }
    }
    if !ref_0.is_null() { xmlFreeURI(ref_0); }
    if !bas.is_null() { xmlFreeURI(bas); }
    if !res.is_null() { xmlFreeURI(res); }
    return val;
}
/* *
 * xmlBuildRelativeURI:
 * @URI:  the URI reference under consideration
 * @base:  the base value
 *
 * Expresses the URI of the reference in terms relative to the
 * base.  Some examples of this operation include:
 *     base = "http://site1.com/docs/book1.html"
 *        URI input                        URI returned
 *     docs/pic1.gif                    pic1.gif
 *     docs/img/pic1.gif                img/pic1.gif
 *     img/pic1.gif                     ../img/pic1.gif
 *     http://site1.com/docs/pic1.gif   pic1.gif
 *     http://site2.com/docs/pic1.gif   http://site2.com/docs/pic1.gif
 *
 *     base = "docs/book1.html"
 *        URI input                        URI returned
 *     docs/pic1.gif                    pic1.gif
 *     docs/img/pic1.gif                img/pic1.gif
 *     img/pic1.gif                     ../img/pic1.gif
 *     http://site1.com/docs/pic1.gif   http://site1.com/docs/pic1.gif
 *
 *
 * Note: if the URI reference is really wierd or complicated, it may be
 *       worthwhile to first convert it into a "nice" one by calling
 *       xmlBuildURI (using 'base') before calling this routine,
 *       since this routine (for reasonable efficiency) assumes URI has
 *       already been through some validation.
 *
 * Returns a new URI string (to be freed by the caller) or NULL in case
 * error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlBuildRelativeURI(mut URI: *const xmlChar,
                                             mut base: *const xmlChar)
 -> *mut xmlChar {
    let mut current_block: u64;
    let mut val: *mut xmlChar = 0 as *mut xmlChar;
    let mut ret: std::os::raw::c_int = 0;
    let mut ix: std::os::raw::c_int = 0;
    let mut nbslash: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut len: std::os::raw::c_int = 0;
    let mut ref_0: xmlURIPtr = 0 as xmlURIPtr;
    let mut bas: xmlURIPtr = 0 as xmlURIPtr;
    let mut bptr: *mut xmlChar = 0 as *mut xmlChar;
    let mut uptr: *mut xmlChar = 0 as *mut xmlChar;
    let mut vptr: *mut xmlChar = 0 as *mut xmlChar;
    let mut remove_path: std::os::raw::c_int = 0 as std::os::raw::c_int;
    if URI.is_null() || *URI as std::os::raw::c_int == 0 as std::os::raw::c_int {
        return 0 as *mut xmlChar
    }
    /*
     * First parse URI into a standard form
     */
    ref_0 = xmlCreateURI();
    if ref_0.is_null() { return 0 as *mut xmlChar }
    /* If URI not already in "relative" form */
    if *URI.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int != '.' as i32 {
        ret = xmlParseURIReference(ref_0, URI as *const std::os::raw::c_char);
        if ret != 0 as std::os::raw::c_int {
            current_block = 10661585731996755370;
        } else { current_block = 10599921512955367680; }
        /* Error in URI, return NULL */
    } else {
        (*ref_0).path = xmlStrdup(URI) as *mut std::os::raw::c_char;
        current_block = 10599921512955367680;
    }
    match current_block {
        10599921512955367680 =>
        /*
     * Next parse base into the same standard form
     */
        {
            if base.is_null() || *base as std::os::raw::c_int == 0 as std::os::raw::c_int {
                val = xmlStrdup(URI)
            } else {
                bas = xmlCreateURI();
                if !bas.is_null() {
                    if *base.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int
                           != '.' as i32 {
                        ret =
                            xmlParseURIReference(bas,
                                                 base as *const std::os::raw::c_char);
                        if ret != 0 as std::os::raw::c_int {
                            current_block = 10661585731996755370;
                        } else { current_block = 14576567515993809846; }
                        /* Error in base, return NULL */
                    } else {
                        (*bas).path = xmlStrdup(base) as *mut std::os::raw::c_char;
                        current_block = 14576567515993809846;
                    }
                    match current_block {
                        10661585731996755370 => { }
                        _ =>
                        /*
     * If the scheme / server on the URI differs from the base,
     * just return the URI
     */
                        {
                            if !(*ref_0).scheme.is_null() &&
                                   ((*bas).scheme.is_null() ||
                                        xmlStrcmp((*bas).scheme as
                                                      *mut xmlChar,
                                                  (*ref_0).scheme as
                                                      *mut xmlChar) != 0 ||
                                        xmlStrcmp((*bas).server as
                                                      *mut xmlChar,
                                                  (*ref_0).server as
                                                      *mut xmlChar) != 0) {
                                val = xmlStrdup(URI)
                            } else if xmlStrEqual((*bas).path as *mut xmlChar,
                                                  (*ref_0).path as
                                                      *mut xmlChar) != 0 {
                                val =
                                    xmlStrdup(b"\x00" as *const u8 as
                                                  *const std::os::raw::c_char as
                                                  *mut xmlChar)
                            } else if (*bas).path.is_null() {
                                val = xmlStrdup((*ref_0).path as *mut xmlChar)
                            } else {
                                if (*ref_0).path.is_null() {
                                    (*ref_0).path =
                                        b"/\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut std::os::raw::c_char;
                                    remove_path = 1 as std::os::raw::c_int
                                }
                                /*
     * At this point (at last!) we can compare the two paths
     *
     * First we take care of the special case where either of the
     * two path components may be missing (bug 316224)
     */
                                if (*bas).path.is_null() {
                                    if !(*ref_0).path.is_null() {
                                        uptr = (*ref_0).path as *mut xmlChar;
                                        if *uptr as std::os::raw::c_int == '/' as i32
                                           {
                                            uptr = uptr.offset(1)
                                        }
                                        /* exception characters from xmlSaveUri */
                                        val =
                                            xmlURIEscapeStr(uptr,
                                                            b"/;&=+$,\x00" as
                                                                *const u8 as
                                                                *const std::os::raw::c_char
                                                                as
                                                                *mut xmlChar)
                                    }
                                } else {
                                    bptr = (*bas).path as *mut xmlChar;
                                    if (*ref_0).path.is_null() {
                                        ix = 0 as std::os::raw::c_int;
                                        while *bptr.offset(ix as isize) as
                                                  std::os::raw::c_int !=
                                                  0 as std::os::raw::c_int {
                                            if *bptr.offset(ix as isize) as
                                                   std::os::raw::c_int == '/' as i32 {
                                                nbslash += 1
                                            }
                                            ix += 1
                                        }
                                        uptr = 0 as *mut xmlChar;
                                        len = 1 as std::os::raw::c_int;
                                        current_block = 3879520548144599102;
                                        /* this is for a string terminator only */
                                    } else {
                                        let mut rptr: *mut xmlChar =
                                            (*ref_0).path as *mut xmlChar;
                                        let mut pos: std::os::raw::c_int =
                                            0 as std::os::raw::c_int;
                                        /*
         * Next we compare the two strings and find where they first differ
         */
                                        if *rptr as std::os::raw::c_int == '.' as i32
                                               &&
                                               *rptr.offset(1 as std::os::raw::c_int
                                                                as isize) as
                                                   std::os::raw::c_int == '/' as i32 {
                                            rptr =
                                                rptr.offset(2 as std::os::raw::c_int
                                                                as isize)
                                        }
                                        if *bptr as std::os::raw::c_int == '.' as i32
                                               &&
                                               *bptr.offset(1 as std::os::raw::c_int
                                                                as isize) as
                                                   std::os::raw::c_int == '/' as i32 {
                                            bptr =
                                                bptr.offset(2 as std::os::raw::c_int
                                                                as isize)
                                        } else if *bptr as std::os::raw::c_int ==
                                                      '/' as i32 &&
                                                      *rptr as std::os::raw::c_int !=
                                                          '/' as i32 {
                                            bptr = bptr.offset(1)
                                        }
                                        while *bptr.offset(pos as isize) as
                                                  std::os::raw::c_int ==
                                                  *rptr.offset(pos as isize)
                                                      as std::os::raw::c_int &&
                                                  *bptr.offset(pos as isize)
                                                      as std::os::raw::c_int !=
                                                      0 as std::os::raw::c_int {
                                            pos += 1
                                        }
                                        if *bptr.offset(pos as isize) as
                                               std::os::raw::c_int ==
                                               *rptr.offset(pos as isize) as
                                                   std::os::raw::c_int {
                                            val =
                                                xmlStrdup(b"\x00" as *const u8
                                                              as
                                                              *const std::os::raw::c_char
                                                              as
                                                              *mut xmlChar);
                                            current_block =
                                                10661585731996755370;
                                            /* (I can't imagine why anyone would do this) */
                                        } else {
                                            /*
	 * In URI, "back up" to the last '/' encountered.  This will be the
	 * beginning of the "unique" suffix of URI
	 */
                                            ix = pos;
                                            if *rptr.offset(ix as isize) as
                                                   std::os::raw::c_int == '/' as i32
                                                   && ix > 0 as std::os::raw::c_int {
                                                ix -= 1
                                            } else if *rptr.offset(ix as
                                                                       isize)
                                                          as std::os::raw::c_int ==
                                                          0 as std::os::raw::c_int &&
                                                          ix >
                                                              1 as std::os::raw::c_int
                                                          &&
                                                          *rptr.offset((ix -
                                                                            1
                                                                                as
                                                                                std::os::raw::c_int)
                                                                           as
                                                                           isize)
                                                              as std::os::raw::c_int
                                                              == '/' as i32 {
                                                ix -= 2 as std::os::raw::c_int
                                            }
                                            while ix > 0 as std::os::raw::c_int {
                                                if *rptr.offset(ix as isize)
                                                       as std::os::raw::c_int ==
                                                       '/' as i32 {
                                                    break ;
                                                }
                                                ix -= 1
                                            }
                                            if ix == 0 as std::os::raw::c_int {
                                                uptr = rptr
                                            } else {
                                                ix += 1;
                                                uptr =
                                                    &mut *rptr.offset(ix as
                                                                          isize)
                                                        as *mut xmlChar
                                            }
                                            /*
	 * In base, count the number of '/' from the differing point
	 */
                                            if *bptr.offset(pos as isize) as
                                                   std::os::raw::c_int !=
                                                   *rptr.offset(pos as isize)
                                                       as std::os::raw::c_int {
                                                /* check for trivial URI == base */
                                                while *bptr.offset(ix as
                                                                       isize)
                                                          as std::os::raw::c_int !=
                                                          0 as std::os::raw::c_int {
                                                    if *bptr.offset(ix as
                                                                        isize)
                                                           as std::os::raw::c_int ==
                                                           '/' as i32 {
                                                        nbslash += 1
                                                    }
                                                    ix += 1
                                                }
                                            }
                                            len =
                                                xmlStrlen(uptr) +
                                                    1 as std::os::raw::c_int;
                                            current_block =
                                                3879520548144599102;
                                        }
                                    }
                                    match current_block {
                                        10661585731996755370 => { }
                                        _ => {
                                            if nbslash == 0 as std::os::raw::c_int {
                                                if !uptr.is_null() {
                                                    /* exception characters from xmlSaveUri */
                                                    val =
                                                        xmlURIEscapeStr(uptr,
                                                                        b"/;&=+$,\x00"
                                                                            as
                                                                            *const u8
                                                                            as
                                                                            *const std::os::raw::c_char
                                                                            as
                                                                            *mut xmlChar)
                                                }
                                            } else {
                                                /*
     * Allocate just enough space for the returned string -
     * length of the remainder of the URI, plus enough space
     * for the "../" groups, plus one for the terminator
     */
                                                val =
                                                    xmlMalloc.expect("non-null function pointer")((len
                                                                                                       +
                                                                                                       3
                                                                                                           as
                                                                                                           std::os::raw::c_int
                                                                                                           *
                                                                                                           nbslash)
                                                                                                      as
                                                                                                      size_t)
                                                        as *mut xmlChar;
                                                if val.is_null() {
                                                    xmlURIErrMemory(b"building relative URI\n\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const std::os::raw::c_char);
                                                } else {
                                                    vptr = val;
                                                    /*
     * Put in as many "../" as needed
     */
                                                    while nbslash >
                                                              0 as std::os::raw::c_int
                                                          {
                                                        let fresh77 = vptr;
                                                        vptr = vptr.offset(1);
                                                        *fresh77 =
                                                            '.' as i32 as
                                                                xmlChar;
                                                        let fresh78 = vptr;
                                                        vptr = vptr.offset(1);
                                                        *fresh78 =
                                                            '.' as i32 as
                                                                xmlChar;
                                                        let fresh79 = vptr;
                                                        vptr = vptr.offset(1);
                                                        *fresh79 =
                                                            '/' as i32 as
                                                                xmlChar;
                                                        nbslash -= 1
                                                    }
                                                    /*
     * Finish up with the end of the URI
     */
                                                    if !uptr.is_null() {
                                                        if vptr > val &&
                                                               len >
                                                                   0 as
                                                                       std::os::raw::c_int
                                                               &&
                                                               *uptr.offset(0
                                                                                as
                                                                                std::os::raw::c_int
                                                                                as
                                                                                isize)
                                                                   as
                                                                   std::os::raw::c_int
                                                                   ==
                                                                   '/' as i32
                                                               &&
                                                               *vptr.offset(-(1
                                                                                  as
                                                                                  std::os::raw::c_int)
                                                                                as
                                                                                isize)
                                                                   as
                                                                   std::os::raw::c_int
                                                                   ==
                                                                   '/' as i32
                                                           {
                                                            memcpy(vptr as
                                                                       *mut std::os::raw::c_void,
                                                                   uptr.offset(1
                                                                                   as
                                                                                   std::os::raw::c_int
                                                                                   as
                                                                                   isize)
                                                                       as
                                                                       *const std::os::raw::c_void,
                                                                   (len -
                                                                        1 as
                                                                            std::os::raw::c_int)
                                                                       as
                                                                       std::os::raw::c_ulong);
                                                            *vptr.offset((len
                                                                              -
                                                                              2
                                                                                  as
                                                                                  std::os::raw::c_int)
                                                                             as
                                                                             isize)
                                                                =
                                                                0 as
                                                                    std::os::raw::c_int
                                                                    as xmlChar
                                                        } else {
                                                            memcpy(vptr as
                                                                       *mut std::os::raw::c_void,
                                                                   uptr as
                                                                       *const std::os::raw::c_void,
                                                                   len as
                                                                       std::os::raw::c_ulong);
                                                            *vptr.offset((len
                                                                              -
                                                                              1
                                                                                  as
                                                                                  std::os::raw::c_int)
                                                                             as
                                                                             isize)
                                                                =
                                                                0 as
                                                                    std::os::raw::c_int
                                                                    as xmlChar
                                                        }
                                                    } else {
                                                        *vptr.offset((len -
                                                                          1 as
                                                                              std::os::raw::c_int)
                                                                         as
                                                                         isize)
                                                            =
                                                            0 as std::os::raw::c_int
                                                                as xmlChar
                                                    }
                                                    /* escape the freshly-built path */
                                                    vptr = val;
                                                    /* exception characters from xmlSaveUri */
                                                    val =
                                                        xmlURIEscapeStr(vptr,
                                                                        b"/;&=+$,\x00"
                                                                            as
                                                                            *const u8
                                                                            as
                                                                            *const std::os::raw::c_char
                                                                            as
                                                                            *mut xmlChar);
                                                    xmlFree.expect("non-null function pointer")(vptr
                                                                                                    as
                                                                                                    *mut std::os::raw::c_void);
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => { }
    }
    /*
     * Free the working variables
     */
    if remove_path != 0 as std::os::raw::c_int {
        (*ref_0).path = 0 as *mut std::os::raw::c_char
    }
    if !ref_0.is_null() { xmlFreeURI(ref_0); }
    if !bas.is_null() { xmlFreeURI(bas); }
    return val;
}
/* *
 * xmlCanonicPath:
 * @path:  the resource locator in a filesystem notation
 *
 * Constructs a canonic path from the specified path.
 *
 * Returns a new canonic path, or a duplicate of the path parameter if the
 * construction fails. The caller is responsible for freeing the memory occupied
 * by the returned string. If there is insufficient memory available, or the
 * argument is NULL, the function returns NULL.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlCanonicPath(mut path: *const xmlChar)
 -> *mut xmlChar {
    let mut current_block: u64;
    /*
 * For Windows implementations, additional work needs to be done to
 * replace backslashes in pathnames with "forward slashes"
 */
    let mut uri: xmlURIPtr = 0 as *mut xmlURI;
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    let mut absuri: *const xmlChar = 0 as *const xmlChar;
    if path.is_null() { return 0 as *mut xmlChar }
    /* sanitize filename starting with // so it can be used as URI */
    if *path.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int == '/' as i32 &&
           *path.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
               '/' as i32 &&
           *path.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int !=
               '/' as i32 {
        path = path.offset(1)
    }
    uri = xmlParseURI(path as *const std::os::raw::c_char);
    if !uri.is_null() { xmlFreeURI(uri); return xmlStrdup(path) }
    /* Check if this is an "absolute uri" */
    absuri =
        xmlStrstr(path,
                  b"://\x00" as *const u8 as *const std::os::raw::c_char as
                      *mut xmlChar);
    if !absuri.is_null() {
        let mut l: std::os::raw::c_int = 0;
        let mut j: std::os::raw::c_int = 0;
        let mut c: std::os::raw::c_uchar = 0;
        let mut escURI: *mut xmlChar = 0 as *mut xmlChar;
        /*
	 * this looks like an URI where some parts have not been
	 * escaped leading to a parsing problem.  Check that the first
	 * part matches a protocol.
	 */
        l = absuri.offset_from(path) as std::os::raw::c_long as std::os::raw::c_int;
        /* Bypass if first part (part before the '://') is > 20 chars */
        if !(l <= 0 as std::os::raw::c_int || l > 20 as std::os::raw::c_int) {
            /* Bypass if any non-alpha characters are present in first part */
            j = 0 as std::os::raw::c_int;
            loop  {
                if !(j < l) { current_block = 5948590327928692120; break ; }
                c = *path.offset(j as isize);
                if !(c as std::os::raw::c_int >= 'a' as i32 &&
                         c as std::os::raw::c_int <= 'z' as i32 ||
                         c as std::os::raw::c_int >= 'A' as i32 &&
                             c as std::os::raw::c_int <= 'Z' as i32) {
                    current_block = 8448203511946012530;
                    break ;
                }
                j += 1
            }
            match current_block {
                8448203511946012530 => { }
                _ => {
                    /* Escape all except the characters specified in the supplied path */
                    escURI =
                        xmlURIEscapeStr(path,
                                        b":/?_.#&;=\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut xmlChar);
                    if !escURI.is_null() {
                        /* Try parsing the escaped path */
                        uri = xmlParseURI(escURI as *const std::os::raw::c_char);
                        /* If successful, return the escaped string */
                        if !uri.is_null() { xmlFreeURI(uri); return escURI }
                        xmlFree.expect("non-null function pointer")(escURI as
                                                                        *mut std::os::raw::c_void);
                    }
                }
            }
        }
    }
    /* For Windows implementations, replace backslashes with 'forward slashes' */
    ret = xmlStrdup(path);
    return ret;
}
/* *
 * xmlPathToURI:
 * @path:  the resource locator in a filesystem notation
 *
 * Constructs an URI expressing the existing path
 *
 * Returns a new URI, or a duplicate of the path parameter if the
 * construction fails. The caller is responsible for freeing the memory
 * occupied by the returned string. If there is insufficient memory available,
 * or the argument is NULL, the function returns NULL.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlPathToURI(mut path: *const xmlChar)
 -> *mut xmlChar {
    let mut uri: xmlURIPtr = 0 as *mut xmlURI;
    let mut temp: xmlURI =
        xmlURI{scheme: 0 as *mut std::os::raw::c_char,
               opaque: 0 as *mut std::os::raw::c_char,
               authority: 0 as *mut std::os::raw::c_char,
               server: 0 as *mut std::os::raw::c_char,
               user: 0 as *mut std::os::raw::c_char,
               port: 0,
               path: 0 as *mut std::os::raw::c_char,
               query: 0 as *mut std::os::raw::c_char,
               fragment: 0 as *mut std::os::raw::c_char,
               cleanup: 0,
               query_raw: 0 as *mut std::os::raw::c_char,};
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    let mut cal: *mut xmlChar = 0 as *mut xmlChar;
    if path.is_null() { return 0 as *mut xmlChar }
    uri = xmlParseURI(path as *const std::os::raw::c_char);
    if !uri.is_null() { xmlFreeURI(uri); return xmlStrdup(path) }
    cal = xmlCanonicPath(path);
    if cal.is_null() { return 0 as *mut xmlChar }
    memset(&mut temp as *mut xmlURI as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<xmlURI>() as std::os::raw::c_ulong);
    temp.path = cal as *mut std::os::raw::c_char;
    ret = xmlSaveUri(&mut temp);
    xmlFree.expect("non-null function pointer")(cal as *mut std::os::raw::c_void);
    return ret;
}
/* __INCLUDE_ELFGCCHACK */
