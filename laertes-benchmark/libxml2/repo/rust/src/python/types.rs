
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type PyMemberDef;
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
    pub type _xmlCatalog;
    pub type _xmlXPathCompExpr;
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
    /* *
 * xmlTextReader:
 *
 * Structure for an xmlReader context.
 */
    pub type _xmlTextReader;
    #[no_mangle]
    fn PyType_IsSubtype(_: *mut PyTypeObject, _: *mut PyTypeObject)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn PyObject_GetAttrString(_: *mut PyObject, _: *const std::os::raw::c_char)
     -> *mut PyObject;
    #[no_mangle]
    fn PyObject_HasAttrString(_: *mut PyObject, _: *const std::os::raw::c_char)
     -> std::os::raw::c_int;
    #[no_mangle]
    static mut _Py_NoneStruct: PyObject;
    #[no_mangle]
    fn PyUnicodeUCS4_AsUTF8String(unicode: *mut PyObject) -> *mut PyObject;
    #[no_mangle]
    fn PyInt_FromLong(_: std::os::raw::c_long) -> *mut PyObject;
    #[no_mangle]
    static mut PyBool_Type: PyTypeObject;
    #[no_mangle]
    static mut _Py_TrueStruct: PyIntObject;
    #[no_mangle]
    fn PyLong_FromLong(_: std::os::raw::c_long) -> *mut PyObject;
    #[no_mangle]
    static mut PyFloat_Type: PyTypeObject;
    #[no_mangle]
    fn PyFloat_FromDouble(_: std::os::raw::c_double) -> *mut PyObject;
    #[no_mangle]
    fn PyString_FromString(_: *const std::os::raw::c_char) -> *mut PyObject;
    #[no_mangle]
    fn PyTuple_New(size: Py_ssize_t) -> *mut PyObject;
    #[no_mangle]
    fn PyTuple_SetItem(_: *mut PyObject, _: Py_ssize_t, _: *mut PyObject)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn PyList_New(size: Py_ssize_t) -> *mut PyObject;
    #[no_mangle]
    fn PyList_Size(_: *mut PyObject) -> Py_ssize_t;
    #[no_mangle]
    fn PyList_GetItem(_: *mut PyObject, _: Py_ssize_t) -> *mut PyObject;
    #[no_mangle]
    fn PyList_SetItem(_: *mut PyObject, _: Py_ssize_t, _: *mut PyObject)
     -> std::os::raw::c_int;
    #[no_mangle]
    static mut PyCapsule_Type: PyTypeObject;
    #[no_mangle]
    fn PyCapsule_New(pointer: *mut std::os::raw::c_void, name: *const std::os::raw::c_char,
                     destructor: PyCapsule_Destructor) -> *mut PyObject;
    #[no_mangle]
    fn PyCapsule_GetPointer(capsule: *mut PyObject, name: *const std::os::raw::c_char)
     -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn xmlStrndup(cur: *const xmlChar, len: std::os::raw::c_int) -> *mut xmlChar;
    #[no_mangle]
    static mut xmlFree: xmlFreeFunc;
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
    #[no_mangle]
    fn xmlXPathNodeSetCreate(val: xmlNodePtr) -> xmlNodeSetPtr;
    #[no_mangle]
    fn xmlXPathWrapString(val: *mut xmlChar) -> xmlXPathObjectPtr;
    #[no_mangle]
    fn xmlXPathNewFloat(val: std::os::raw::c_double) -> xmlXPathObjectPtr;
    #[no_mangle]
    fn xmlXPathNewBoolean(val: std::os::raw::c_int) -> xmlXPathObjectPtr;
    #[no_mangle]
    fn xmlXPathNodeSetAdd(cur: xmlNodeSetPtr, val: xmlNodePtr) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlXPathWrapNodeSet(val: xmlNodeSetPtr) -> xmlXPathObjectPtr;
    /* *
 * Really internal functions
 */
    #[no_mangle]
    fn xmlXPathNodeSetFreeNs(ns: xmlNsPtr);
}
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
/* HAVE_UINTPTR_T */
/* Py_ssize_t is a signed integral type such that sizeof(Py_ssize_t) ==
 * sizeof(size_t).  C99 doesn't define such a thing directly (size_t is an
 * unsigned integral type).  See PEP 353 for details.
 */
pub type Py_ssize_t = ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _object {
    pub ob_refcnt: Py_ssize_t,
    pub ob_type: *mut _typeobject,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _typeobject {
    pub ob_refcnt: Py_ssize_t,
    pub ob_type: *mut _typeobject,
    pub ob_size: Py_ssize_t,
    pub tp_name: *const std::os::raw::c_char,
    pub tp_basicsize: Py_ssize_t,
    pub tp_itemsize: Py_ssize_t,
    pub tp_dealloc: destructor,
    pub tp_print: printfunc,
    pub tp_getattr: getattrfunc,
    pub tp_setattr: setattrfunc,
    pub tp_compare: cmpfunc,
    pub tp_repr: reprfunc,
    pub tp_as_number: *mut PyNumberMethods,
    pub tp_as_sequence: *mut PySequenceMethods,
    pub tp_as_mapping: *mut PyMappingMethods,
    pub tp_hash: hashfunc,
    pub tp_call: ternaryfunc,
    pub tp_str: reprfunc,
    pub tp_getattro: getattrofunc,
    pub tp_setattro: setattrofunc,
    pub tp_as_buffer: *mut PyBufferProcs,
    pub tp_flags: std::os::raw::c_long,
    pub tp_doc: *const std::os::raw::c_char,
    pub tp_traverse: traverseproc,
    pub tp_clear: inquiry,
    pub tp_richcompare: richcmpfunc,
    pub tp_weaklistoffset: Py_ssize_t,
    pub tp_iter: getiterfunc,
    pub tp_iternext: iternextfunc,
    pub tp_methods: *mut PyMethodDef,
    pub tp_members: *mut PyMemberDef,
    pub tp_getset: *mut PyGetSetDef,
    pub tp_base: *mut _typeobject,
    pub tp_dict: *mut PyObject,
    pub tp_descr_get: descrgetfunc,
    pub tp_descr_set: descrsetfunc,
    pub tp_dictoffset: Py_ssize_t,
    pub tp_init: initproc,
    pub tp_alloc: allocfunc,
    pub tp_new: newfunc,
    pub tp_free: freefunc,
    pub tp_is_gc: inquiry,
    pub tp_bases: *mut PyObject,
    pub tp_mro: *mut PyObject,
    pub tp_cache: *mut PyObject,
    pub tp_subclasses: *mut PyObject,
    pub tp_weaklist: *mut PyObject,
    pub tp_del: destructor,
    pub tp_version_tag: std::os::raw::c_uint,
}
pub type destructor = Option<unsafe extern "C" fn(_: *mut PyObject) -> ()>;
pub type PyObject = _object;
pub type inquiry
    =
    Option<unsafe extern "C" fn(_: *mut PyObject) -> std::os::raw::c_int>;
pub type freefunc = Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void) -> ()>;
pub type newfunc
    =
    Option<unsafe extern "C" fn(_: *mut _typeobject, _: *mut PyObject,
                                _: *mut PyObject) -> *mut PyObject>;
pub type allocfunc
    =
    Option<unsafe extern "C" fn(_: *mut _typeobject, _: Py_ssize_t)
               -> *mut PyObject>;
pub type initproc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: *mut PyObject,
                                _: *mut PyObject) -> std::os::raw::c_int>;
pub type descrsetfunc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: *mut PyObject,
                                _: *mut PyObject) -> std::os::raw::c_int>;
pub type descrgetfunc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: *mut PyObject,
                                _: *mut PyObject) -> *mut PyObject>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PyGetSetDef {
    pub name: *mut std::os::raw::c_char,
    pub get: getter,
    pub set: setter,
    pub doc: *mut std::os::raw::c_char,
    pub closure: *mut std::os::raw::c_void,
}
/* Descriptors */
pub type setter
    =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: *mut PyObject,
                                _: *mut std::os::raw::c_void) -> std::os::raw::c_int>;
pub type getter
    =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: *mut std::os::raw::c_void)
               -> *mut PyObject>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PyMethodDef {
    pub ml_name: *const std::os::raw::c_char,
    pub ml_meth: PyCFunction,
    pub ml_flags: std::os::raw::c_int,
    pub ml_doc: *const std::os::raw::c_char,
}
/* Method object interface */
/* This is about the type 'builtin_function_or_method',
   not Python methods in user-defined classes.  See classobject.h
   for the latter. */
pub type PyCFunction
    =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: *mut PyObject)
               -> *mut PyObject>;
pub type iternextfunc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject) -> *mut PyObject>;
pub type getiterfunc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject) -> *mut PyObject>;
pub type richcmpfunc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: *mut PyObject,
                                _: std::os::raw::c_int) -> *mut PyObject>;
pub type traverseproc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: visitproc,
                                _: *mut std::os::raw::c_void) -> std::os::raw::c_int>;
pub type visitproc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: *mut std::os::raw::c_void)
               -> std::os::raw::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PyBufferProcs {
    pub bf_getreadbuffer: readbufferproc,
    pub bf_getwritebuffer: writebufferproc,
    pub bf_getsegcount: segcountproc,
    pub bf_getcharbuffer: charbufferproc,
    pub bf_getbuffer: getbufferproc,
    pub bf_releasebuffer: releasebufferproc,
}
pub type releasebufferproc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: *mut Py_buffer) -> ()>;
pub type Py_buffer = bufferinfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bufferinfo {
    pub buf: *mut std::os::raw::c_void,
    pub obj: *mut PyObject,
    pub len: Py_ssize_t,
    pub itemsize: Py_ssize_t,
    pub readonly: std::os::raw::c_int,
    pub ndim: std::os::raw::c_int,
    pub format: *mut std::os::raw::c_char,
    pub shape: *mut Py_ssize_t,
    pub strides: *mut Py_ssize_t,
    pub suboffsets: *mut Py_ssize_t,
    pub smalltable: [Py_ssize_t; 2],
    pub internal: *mut std::os::raw::c_void,
}
pub type getbufferproc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: *mut Py_buffer,
                                _: std::os::raw::c_int) -> std::os::raw::c_int>;
pub type charbufferproc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: Py_ssize_t,
                                _: *mut *mut std::os::raw::c_char) -> Py_ssize_t>;
pub type segcountproc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: *mut Py_ssize_t)
               -> Py_ssize_t>;
pub type writebufferproc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: Py_ssize_t,
                                _: *mut *mut std::os::raw::c_void) -> Py_ssize_t>;
pub type readbufferproc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: Py_ssize_t,
                                _: *mut *mut std::os::raw::c_void) -> Py_ssize_t>;
pub type setattrofunc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: *mut PyObject,
                                _: *mut PyObject) -> std::os::raw::c_int>;
pub type getattrofunc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: *mut PyObject)
               -> *mut PyObject>;
pub type reprfunc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject) -> *mut PyObject>;
pub type ternaryfunc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: *mut PyObject,
                                _: *mut PyObject) -> *mut PyObject>;
pub type hashfunc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject) -> std::os::raw::c_long>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PyMappingMethods {
    pub mp_length: lenfunc,
    pub mp_subscript: binaryfunc,
    pub mp_ass_subscript: objobjargproc,
}
pub type objobjargproc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: *mut PyObject,
                                _: *mut PyObject) -> std::os::raw::c_int>;
pub type binaryfunc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: *mut PyObject)
               -> *mut PyObject>;
pub type lenfunc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject) -> Py_ssize_t>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PySequenceMethods {
    pub sq_length: lenfunc,
    pub sq_concat: binaryfunc,
    pub sq_repeat: ssizeargfunc,
    pub sq_item: ssizeargfunc,
    pub sq_slice: ssizessizeargfunc,
    pub sq_ass_item: ssizeobjargproc,
    pub sq_ass_slice: ssizessizeobjargproc,
    pub sq_contains: objobjproc,
    pub sq_inplace_concat: binaryfunc,
    pub sq_inplace_repeat: ssizeargfunc,
}
pub type ssizeargfunc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: Py_ssize_t)
               -> *mut PyObject>;
pub type objobjproc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: *mut PyObject)
               -> std::os::raw::c_int>;
pub type ssizessizeobjargproc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: Py_ssize_t,
                                _: Py_ssize_t, _: *mut PyObject)
               -> std::os::raw::c_int>;
pub type ssizeobjargproc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: Py_ssize_t,
                                _: *mut PyObject) -> std::os::raw::c_int>;
pub type ssizessizeargfunc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: Py_ssize_t,
                                _: Py_ssize_t) -> *mut PyObject>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PyNumberMethods {
    pub nb_add: binaryfunc,
    pub nb_subtract: binaryfunc,
    pub nb_multiply: binaryfunc,
    pub nb_divide: binaryfunc,
    pub nb_remainder: binaryfunc,
    pub nb_divmod: binaryfunc,
    pub nb_power: ternaryfunc,
    pub nb_negative: unaryfunc,
    pub nb_positive: unaryfunc,
    pub nb_absolute: unaryfunc,
    pub nb_nonzero: inquiry,
    pub nb_invert: unaryfunc,
    pub nb_lshift: binaryfunc,
    pub nb_rshift: binaryfunc,
    pub nb_and: binaryfunc,
    pub nb_xor: binaryfunc,
    pub nb_or: binaryfunc,
    pub nb_coerce: coercion,
    pub nb_int: unaryfunc,
    pub nb_long: unaryfunc,
    pub nb_float: unaryfunc,
    pub nb_oct: unaryfunc,
    pub nb_hex: unaryfunc,
    pub nb_inplace_add: binaryfunc,
    pub nb_inplace_subtract: binaryfunc,
    pub nb_inplace_multiply: binaryfunc,
    pub nb_inplace_divide: binaryfunc,
    pub nb_inplace_remainder: binaryfunc,
    pub nb_inplace_power: ternaryfunc,
    pub nb_inplace_lshift: binaryfunc,
    pub nb_inplace_rshift: binaryfunc,
    pub nb_inplace_and: binaryfunc,
    pub nb_inplace_xor: binaryfunc,
    pub nb_inplace_or: binaryfunc,
    pub nb_floor_divide: binaryfunc,
    pub nb_true_divide: binaryfunc,
    pub nb_inplace_floor_divide: binaryfunc,
    pub nb_inplace_true_divide: binaryfunc,
    pub nb_index: unaryfunc,
}
pub type unaryfunc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject) -> *mut PyObject>;
pub type coercion
    =
    Option<unsafe extern "C" fn(_: *mut *mut PyObject, _: *mut *mut PyObject)
               -> std::os::raw::c_int>;
pub type cmpfunc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: *mut PyObject)
               -> std::os::raw::c_int>;
pub type setattrfunc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: *mut std::os::raw::c_char,
                                _: *mut PyObject) -> std::os::raw::c_int>;
pub type getattrfunc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: *mut std::os::raw::c_char)
               -> *mut PyObject>;
pub type printfunc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: *mut FILE,
                                _: std::os::raw::c_int) -> std::os::raw::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PyVarObject {
    pub ob_refcnt: Py_ssize_t,
    pub ob_type: *mut _typeobject,
    pub ob_size: Py_ssize_t,
}
pub type PyTypeObject = _typeobject;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PyIntObject {
    pub ob_refcnt: Py_ssize_t,
    pub ob_type: *mut _typeobject,
    pub ob_ival: std::os::raw::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PyFloatObject {
    pub ob_refcnt: Py_ssize_t,
    pub ob_type: *mut _typeobject,
    pub ob_fval: std::os::raw::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PyStringObject {
    pub ob_refcnt: Py_ssize_t,
    pub ob_type: *mut _typeobject,
    pub ob_size: Py_ssize_t,
    pub ob_shash: std::os::raw::c_long,
    pub ob_sstate: std::os::raw::c_int,
    pub ob_sval: [std::os::raw::c_char; 1],
}
pub type PyCapsule_Destructor
    =
    Option<unsafe extern "C" fn(_: *mut PyObject) -> ()>;
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
/*
 * Those are the functions and datatypes for the library output
 * I/O structures.
 */
/* *
 * xmlOutputMatchCallback:
 * @filename: the filename or URI
 *
 * Callback used in the I/O Output API to detect if the current handler
 * can provide output fonctionnalities for this resource.
 *
 * Returns 1 if yes and 0 if another Output module should be used
 */
/* *
 * xmlOutputOpenCallback:
 * @filename: the filename or URI
 *
 * Callback used in the I/O Output API to open the resource
 *
 * Returns an Output context or NULL in case or error
 */
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
pub type xmlBufPtr = *mut xmlBuf;
pub type xmlBuf = _xmlBuf;
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
/* amount consumed from raw */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlOutputBuffer {
    pub context: *mut std::os::raw::c_void,
    pub writecallback: xmlOutputWriteCallback,
    pub closecallback: xmlOutputCloseCallback,
    pub encoder: xmlCharEncodingHandlerPtr,
    pub buffer: xmlBufPtr,
    pub conv: xmlBufPtr,
    pub written: std::os::raw::c_int,
    pub error: std::os::raw::c_int,
}
pub type xmlOutputCloseCallback
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void) -> std::os::raw::c_int>;
pub type xmlOutputWriteCallback
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const std::os::raw::c_char,
                                _: std::os::raw::c_int) -> std::os::raw::c_int>;
pub type xmlOutputBuffer = _xmlOutputBuffer;
pub type xmlOutputBufferPtr = *mut xmlOutputBuffer;
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
pub type xmlFreeFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void) -> ()>;
pub type xmlValidCtxtPtr = *mut xmlValidCtxt;
pub type xmlCatalog = _xmlCatalog;
pub type xmlCatalogPtr = *mut xmlCatalog;
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
pub type xmlURI = _xmlURI;
pub type xmlURIPtr = *mut xmlURI;
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
/* *
 * The set of XPath error codes.
 */
/*
 * A node-set (an unordered collection of nodes without duplicates).
 */
/* number of nodes in the set */
/* size of the array as allocated */
/* array of nodes in no particular order */
/* @@ with_ns to check wether namespace nodes should be looked at @@ */
/*
 * An expression is evaluated to yield an object, which
 * has one of the following four basic types:
 *   - node-set
 *   - boolean
 *   - number
 *   - string
 *
 * @@ XPointer will add more types !
 */
/* An XSLT value tree, non modifiable */
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
/*
 * Extra type: a name and a conversion function.
 */
/* the type name */
/* the conversion function */
/*
 * Extra variable: a name and a value.
 */
/* the variable name */
/* the value */
/* *
 * xmlXPathEvalFunc:
 * @ctxt: an XPath parser context
 * @nargs: the number of arguments passed to the function
 *
 * An XPath evaluation function, the parameters are on the XPath context stack.
 */
/*
 * Extra function: a name and a evaluation function.
 */
/* the function name */
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
/*
 * Extra axis: a name and an axis function.
 */
/* the axis name */
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
pub type xmlXPathFuncLookupFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                _: *const xmlChar) -> xmlXPathFunction>;
pub type xmlXPathFunction
    =
    Option<unsafe extern "C" fn(_: xmlXPathParserContextPtr, _: std::os::raw::c_int)
               -> ()>;
pub type xmlXPathParserContextPtr = *mut xmlXPathParserContext;
pub type xmlXPathParserContext = _xmlXPathParserContext;
/*
 * The structure of a compiled expression form is not public.
 */
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
pub type xmlXPathContext = _xmlXPathContext;
pub type xmlXPathVariableLookupFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                _: *const xmlChar) -> xmlXPathObjectPtr>;
pub type xmlXPathAxisPtr = *mut xmlXPathAxis;
pub type xmlXPathAxis = _xmlXPathAxis;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlXPathAxis {
    pub name: *const xmlChar,
    pub func: xmlXPathAxisFunc,
}
pub type xmlXPathAxisFunc
    =
    Option<unsafe extern "C" fn(_: xmlXPathParserContextPtr,
                                _: xmlXPathObjectPtr) -> xmlXPathObjectPtr>;
pub type xmlXPathTypePtr = *mut xmlXPathType;
pub type xmlXPathType = _xmlXPathType;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlXPathType {
    pub name: *const xmlChar,
    pub func: xmlXPathConvertFunc,
}
pub type xmlXPathConvertFunc
    =
    Option<unsafe extern "C" fn(_: xmlXPathObjectPtr, _: std::os::raw::c_int)
               -> std::os::raw::c_int>;
/*
 * Summary: API to handle XML Pointers
 * Description: API to handle XML Pointers
 * Base implementation was made accordingly to
 * W3C Candidate Recommendation 7 June 2000
 * http://www.w3.org/TR/2000/CR-xptr-20000607
 *
 * Added support for the element() scheme described in:
 * W3C Proposed Recommendation 13 November 2002
 * http://www.w3.org/TR/2002/PR-xptr-element-20021113/
 *
 * Copy: See Copyright for the status of this software.
 *
 * Author: Daniel Veillard
 */
/*
 * A Location Set
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlLocationSet {
    pub locNr: std::os::raw::c_int,
    pub locMax: std::os::raw::c_int,
    pub locTab: *mut xmlXPathObjectPtr,
}
pub type xmlLocationSet = _xmlLocationSet;
pub type xmlLocationSetPtr = *mut xmlLocationSet;
pub type xmlRelaxNG = _xmlRelaxNG;
pub type xmlRelaxNGPtr = *mut xmlRelaxNG;
pub type xmlRelaxNGParserCtxt = _xmlRelaxNGParserCtxt;
pub type xmlRelaxNGParserCtxtPtr = *mut xmlRelaxNGParserCtxt;
pub type xmlRelaxNGValidCtxt = _xmlRelaxNGValidCtxt;
pub type xmlRelaxNGValidCtxtPtr = *mut xmlRelaxNGValidCtxt;
pub type xmlSchema = _xmlSchema;
pub type xmlSchemaPtr = *mut xmlSchema;
pub type xmlSchemaParserCtxt = _xmlSchemaParserCtxt;
pub type xmlSchemaParserCtxtPtr = *mut xmlSchemaParserCtxt;
pub type xmlSchemaValidCtxt = _xmlSchemaValidCtxt;
pub type xmlSchemaValidCtxtPtr = *mut xmlSchemaValidCtxt;
pub type xmlTextReader = _xmlTextReader;
/* *
 * xmlTextReaderPtr:
 *
 * Pointer to an xmlReader context.
 */
pub type xmlTextReaderPtr = *mut xmlTextReader;
/*
 * Error handling extensions
 */
pub type xmlTextReaderLocatorPtr = *mut std::os::raw::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PyxmlNode_Object {
    pub ob_refcnt: Py_ssize_t,
    pub ob_type: *mut _typeobject,
    pub obj: xmlNodePtr,
}
#[no_mangle]
pub unsafe extern "C" fn libxml_intWrap(mut val: std::os::raw::c_int)
 -> *mut PyObject {
    let mut ret: *mut PyObject = 0 as *mut PyObject;
    ret = PyInt_FromLong(val as std::os::raw::c_long);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn libxml_longWrap(mut val: std::os::raw::c_long)
 -> *mut PyObject {
    let mut ret: *mut PyObject = 0 as *mut PyObject;
    ret = PyLong_FromLong(val);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn libxml_doubleWrap(mut val: std::os::raw::c_double)
 -> *mut PyObject {
    let mut ret: *mut PyObject = 0 as *mut PyObject;
    ret = PyFloat_FromDouble(val);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn libxml_charPtrWrap(mut str: *mut std::os::raw::c_char)
 -> *mut PyObject {
    let mut ret: *mut PyObject = 0 as *mut PyObject;
    if str.is_null() {
        let ref mut fresh0 =
            (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
        *fresh0 += 1;
        return &mut _Py_NoneStruct
    }
    ret = PyString_FromString(str);
    xmlFree.expect("non-null function pointer")(str as *mut std::os::raw::c_void);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn libxml_charPtrConstWrap(mut str: *const std::os::raw::c_char)
 -> *mut PyObject {
    let mut ret: *mut PyObject = 0 as *mut PyObject;
    if str.is_null() {
        let ref mut fresh1 =
            (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
        *fresh1 += 1;
        return &mut _Py_NoneStruct
    }
    ret = PyString_FromString(str);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn libxml_xmlCharPtrWrap(mut str: *mut xmlChar)
 -> *mut PyObject {
    let mut ret: *mut PyObject = 0 as *mut PyObject;
    if str.is_null() {
        let ref mut fresh2 =
            (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
        *fresh2 += 1;
        return &mut _Py_NoneStruct
    }
    ret = PyString_FromString(str as *mut std::os::raw::c_char);
    xmlFree.expect("non-null function pointer")(str as *mut std::os::raw::c_void);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn libxml_xmlCharPtrConstWrap(mut str: *const xmlChar)
 -> *mut PyObject {
    let mut ret: *mut PyObject = 0 as *mut PyObject;
    if str.is_null() {
        let ref mut fresh3 =
            (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
        *fresh3 += 1;
        return &mut _Py_NoneStruct
    }
    ret = PyString_FromString(str as *mut std::os::raw::c_char);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn libxml_constcharPtrWrap(mut str: *const std::os::raw::c_char)
 -> *mut PyObject {
    let mut ret: *mut PyObject = 0 as *mut PyObject;
    if str.is_null() {
        let ref mut fresh4 =
            (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
        *fresh4 += 1;
        return &mut _Py_NoneStruct
    }
    ret = PyString_FromString(str);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn libxml_constxmlCharPtrWrap(mut str: *const xmlChar)
 -> *mut PyObject {
    let mut ret: *mut PyObject = 0 as *mut PyObject;
    if str.is_null() {
        let ref mut fresh5 =
            (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
        *fresh5 += 1;
        return &mut _Py_NoneStruct
    }
    ret = PyString_FromString(str as *mut std::os::raw::c_char);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn libxml_xmlDocPtrWrap(mut doc: xmlDocPtr)
 -> *mut PyObject {
    let mut ret: *mut PyObject = 0 as *mut PyObject;
    if doc.is_null() {
        let ref mut fresh6 =
            (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
        *fresh6 += 1;
        return &mut _Py_NoneStruct
    }
    /* TODO: look at deallocation */
    ret =
        PyCapsule_New(doc as *mut std::os::raw::c_void,
                      b"xmlDocPtr\x00" as *const u8 as *const std::os::raw::c_char as
                          *mut std::os::raw::c_char, None);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn libxml_xmlNodePtrWrap(mut node: xmlNodePtr)
 -> *mut PyObject {
    let mut ret: *mut PyObject = 0 as *mut PyObject;
    if node.is_null() {
        let ref mut fresh7 =
            (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
        *fresh7 += 1;
        return &mut _Py_NoneStruct
    }
    ret =
        PyCapsule_New(node as *mut std::os::raw::c_void,
                      b"xmlNodePtr\x00" as *const u8 as *const std::os::raw::c_char as
                          *mut std::os::raw::c_char, None);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn libxml_xmlURIPtrWrap(mut uri: xmlURIPtr)
 -> *mut PyObject {
    let mut ret: *mut PyObject = 0 as *mut PyObject;
    if uri.is_null() {
        let ref mut fresh8 =
            (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
        *fresh8 += 1;
        return &mut _Py_NoneStruct
    }
    ret =
        PyCapsule_New(uri as *mut std::os::raw::c_void,
                      b"xmlURIPtr\x00" as *const u8 as *const std::os::raw::c_char as
                          *mut std::os::raw::c_char, None);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn libxml_xmlNsPtrWrap(mut ns: xmlNsPtr)
 -> *mut PyObject {
    let mut ret: *mut PyObject = 0 as *mut PyObject;
    if ns.is_null() {
        let ref mut fresh9 =
            (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
        *fresh9 += 1;
        return &mut _Py_NoneStruct
    }
    ret =
        PyCapsule_New(ns as *mut std::os::raw::c_void,
                      b"xmlNsPtr\x00" as *const u8 as *const std::os::raw::c_char as
                          *mut std::os::raw::c_char, None);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn libxml_xmlAttrPtrWrap(mut attr: xmlAttrPtr)
 -> *mut PyObject {
    let mut ret: *mut PyObject = 0 as *mut PyObject;
    if attr.is_null() {
        let ref mut fresh10 =
            (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
        *fresh10 += 1;
        return &mut _Py_NoneStruct
    }
    ret =
        PyCapsule_New(attr as *mut std::os::raw::c_void,
                      b"xmlAttrPtr\x00" as *const u8 as *const std::os::raw::c_char as
                          *mut std::os::raw::c_char, None);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn libxml_xmlAttributePtrWrap(mut attr: xmlAttributePtr)
 -> *mut PyObject {
    let mut ret: *mut PyObject = 0 as *mut PyObject;
    if attr.is_null() {
        let ref mut fresh11 =
            (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
        *fresh11 += 1;
        return &mut _Py_NoneStruct
    }
    ret =
        PyCapsule_New(attr as *mut std::os::raw::c_void,
                      b"xmlAttributePtr\x00" as *const u8 as
                          *const std::os::raw::c_char as *mut std::os::raw::c_char, None);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn libxml_xmlElementPtrWrap(mut elem: xmlElementPtr)
 -> *mut PyObject {
    let mut ret: *mut PyObject = 0 as *mut PyObject;
    if elem.is_null() {
        let ref mut fresh12 =
            (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
        *fresh12 += 1;
        return &mut _Py_NoneStruct
    }
    ret =
        PyCapsule_New(elem as *mut std::os::raw::c_void,
                      b"xmlElementPtr\x00" as *const u8 as *const std::os::raw::c_char
                          as *mut std::os::raw::c_char, None);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn libxml_xmlXPathContextPtrWrap(mut ctxt:
                                                           xmlXPathContextPtr)
 -> *mut PyObject {
    let mut ret: *mut PyObject = 0 as *mut PyObject;
    if ctxt.is_null() {
        let ref mut fresh13 =
            (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
        *fresh13 += 1;
        return &mut _Py_NoneStruct
    }
    ret =
        PyCapsule_New(ctxt as *mut std::os::raw::c_void,
                      b"xmlXPathContextPtr\x00" as *const u8 as
                          *const std::os::raw::c_char as *mut std::os::raw::c_char, None);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn libxml_xmlXPathParserContextPtrWrap(mut ctxt:
                                                                 xmlXPathParserContextPtr)
 -> *mut PyObject {
    let mut ret: *mut PyObject = 0 as *mut PyObject;
    if ctxt.is_null() {
        let ref mut fresh14 =
            (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
        *fresh14 += 1;
        return &mut _Py_NoneStruct
    }
    ret =
        PyCapsule_New(ctxt as *mut std::os::raw::c_void,
                      b"xmlXPathParserContextPtr\x00" as *const u8 as
                          *const std::os::raw::c_char as *mut std::os::raw::c_char, None);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn libxml_xmlParserCtxtPtrWrap(mut ctxt:
                                                         xmlParserCtxtPtr)
 -> *mut PyObject {
    let mut ret: *mut PyObject = 0 as *mut PyObject;
    if ctxt.is_null() {
        let ref mut fresh15 =
            (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
        *fresh15 += 1;
        return &mut _Py_NoneStruct
    }
    ret =
        PyCapsule_New(ctxt as *mut std::os::raw::c_void,
                      b"xmlParserCtxtPtr\x00" as *const u8 as
                          *const std::os::raw::c_char as *mut std::os::raw::c_char, None);
    return ret;
}
/* *
 * libxml_xmlXPathDestructNsNode:
 * cap: xmlNsPtr namespace node capsule object
 *
 * This function is called if and when a namespace node returned in
 * an XPath node set is to be destroyed. That's the only kind of
 * object returned in node set not directly linked to the original
 * xmlDoc document, see xmlXPathNodeSetDupNs.
 */
unsafe extern "C" fn libxml_xmlXPathDestructNsNode(mut cap: *mut PyObject) {
    xmlXPathNodeSetFreeNs(PyCapsule_GetPointer(cap,
                                               b"xmlNsPtr\x00" as *const u8 as
                                                   *const std::os::raw::c_char) as
                              xmlNsPtr);
}
#[no_mangle]
pub unsafe extern "C" fn libxml_xmlXPathObjectPtrWrap(mut obj:
                                                          xmlXPathObjectPtr)
 -> *mut PyObject {
    let mut ret: *mut PyObject = 0 as *mut PyObject;
    if obj.is_null() {
        let ref mut fresh16 =
            (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
        *fresh16 += 1;
        return &mut _Py_NoneStruct
    }
    match (*obj).type_0 as std::os::raw::c_uint {
        9 => {
            if (*obj).nodesetval.is_null() ||
                   (*(*obj).nodesetval).nodeNr == 0 as std::os::raw::c_int ||
                   (*(*obj).nodesetval).nodeTab.is_null() {
                ret = PyList_New(0 as std::os::raw::c_int as Py_ssize_t)
            } else {
                let mut i: std::os::raw::c_int = 0;
                let mut len: std::os::raw::c_int = 0 as std::os::raw::c_int;
                let mut node: xmlNodePtr = 0 as *mut xmlNode;
                node =
                    (**(*(*obj).nodesetval).nodeTab.offset(0 as std::os::raw::c_int as
                                                               isize)).children;
                while !node.is_null() { len += 1; node = (*node).next }
                ret = PyList_New(len as Py_ssize_t);
                node =
                    (**(*(*obj).nodesetval).nodeTab.offset(0 as std::os::raw::c_int as
                                                               isize)).children;
                i = 0 as std::os::raw::c_int;
                while i < len {
                    PyList_SetItem(ret, i as Py_ssize_t,
                                   libxml_xmlNodePtrWrap(node));
                    node = (*node).next;
                    i += 1
                }
            }
            /*
	     * Return now, do not free the object passed down
	     */
            return ret
        }
        1 => {
            if (*obj).nodesetval.is_null() ||
                   (*(*obj).nodesetval).nodeNr == 0 as std::os::raw::c_int {
                ret = PyList_New(0 as std::os::raw::c_int as Py_ssize_t)
            } else {
                let mut i_0: std::os::raw::c_int = 0;
                let mut node_0: xmlNodePtr = 0 as *mut xmlNode;
                ret = PyList_New((*(*obj).nodesetval).nodeNr as Py_ssize_t);
                i_0 = 0 as std::os::raw::c_int;
                while i_0 < (*(*obj).nodesetval).nodeNr {
                    node_0 =
                        *(*(*obj).nodesetval).nodeTab.offset(i_0 as isize);
                    if (*node_0).type_0 as std::os::raw::c_uint ==
                           XML_NAMESPACE_DECL as std::os::raw::c_int as std::os::raw::c_uint {
                        let mut ns: *mut PyObject =
                            PyCapsule_New(node_0 as *mut std::os::raw::c_void,
                                          b"xmlNsPtr\x00" as *const u8 as
                                              *const std::os::raw::c_char as
                                              *mut std::os::raw::c_char,
                                          Some(libxml_xmlXPathDestructNsNode
                                                   as
                                                   unsafe extern "C" fn(_:
                                                                            *mut PyObject)
                                                       -> ()));
                        PyList_SetItem(ret, i_0 as Py_ssize_t, ns);
                        /* make sure the xmlNsPtr is not destroyed now */
                        let ref mut fresh17 =
                            *(*(*obj).nodesetval).nodeTab.offset(i_0 as
                                                                     isize); /*xmlXPathObjectPtr setobj*/
                        *fresh17 = 0 as xmlNodePtr
                    } else {
                        PyList_SetItem(ret, i_0 as Py_ssize_t,
                                       libxml_xmlNodePtrWrap(node_0));
                    }
                    i_0 += 1
                }
            }
        }
        2 => { ret = PyInt_FromLong((*obj).boolval as std::os::raw::c_long) }
        3 => { ret = PyFloat_FromDouble((*obj).floatval) }
        4 => {
            ret = PyString_FromString((*obj).stringval as *mut std::os::raw::c_char)
        }
        5 => {
            let mut node_1: *mut PyObject = 0 as *mut PyObject;
            let mut indexIntoNode: *mut PyObject = 0 as *mut PyObject;
            let mut tuple: *mut PyObject = 0 as *mut PyObject;
            node_1 = libxml_xmlNodePtrWrap((*obj).user as xmlNodePtr);
            indexIntoNode = PyInt_FromLong((*obj).index as std::os::raw::c_long);
            tuple = PyTuple_New(2 as std::os::raw::c_int as Py_ssize_t);
            PyTuple_SetItem(tuple, 0 as std::os::raw::c_int as Py_ssize_t, node_1);
            PyTuple_SetItem(tuple, 1 as std::os::raw::c_int as Py_ssize_t,
                            indexIntoNode);
            ret = tuple
        }
        6 => {
            let mut bCollapsedRange: std::os::raw::c_ushort = 0;
            bCollapsedRange =
                ((*obj).user2.is_null() ||
                     (*obj).user2 == (*obj).user &&
                         (*obj).index == (*obj).index2) as std::os::raw::c_int as
                    std::os::raw::c_ushort;
            if bCollapsedRange != 0 {
                let mut node_2: *mut PyObject = 0 as *mut PyObject;
                let mut indexIntoNode_0: *mut PyObject = 0 as *mut PyObject;
                let mut tuple_0: *mut PyObject = 0 as *mut PyObject;
                let mut list: *mut PyObject = 0 as *mut PyObject;
                list = PyList_New(1 as std::os::raw::c_int as Py_ssize_t);
                node_2 = libxml_xmlNodePtrWrap((*obj).user as xmlNodePtr);
                indexIntoNode_0 =
                    PyInt_FromLong((*obj).index as std::os::raw::c_long);
                tuple_0 = PyTuple_New(2 as std::os::raw::c_int as Py_ssize_t);
                PyTuple_SetItem(tuple_0, 0 as std::os::raw::c_int as Py_ssize_t,
                                node_2);
                PyTuple_SetItem(tuple_0, 1 as std::os::raw::c_int as Py_ssize_t,
                                indexIntoNode_0);
                PyList_SetItem(list, 0 as std::os::raw::c_int as Py_ssize_t, tuple_0);
                ret = list
            } else {
                let mut node_3: *mut PyObject = 0 as *mut PyObject;
                let mut indexIntoNode_1: *mut PyObject = 0 as *mut PyObject;
                let mut tuple_1: *mut PyObject = 0 as *mut PyObject;
                let mut list_0: *mut PyObject = 0 as *mut PyObject;
                list_0 = PyList_New(2 as std::os::raw::c_int as Py_ssize_t);
                node_3 = libxml_xmlNodePtrWrap((*obj).user as xmlNodePtr);
                indexIntoNode_1 =
                    PyInt_FromLong((*obj).index as std::os::raw::c_long);
                tuple_1 = PyTuple_New(2 as std::os::raw::c_int as Py_ssize_t);
                PyTuple_SetItem(tuple_1, 0 as std::os::raw::c_int as Py_ssize_t,
                                node_3);
                PyTuple_SetItem(tuple_1, 1 as std::os::raw::c_int as Py_ssize_t,
                                indexIntoNode_1);
                PyList_SetItem(list_0, 0 as std::os::raw::c_int as Py_ssize_t,
                               tuple_1);
                node_3 = libxml_xmlNodePtrWrap((*obj).user2 as xmlNodePtr);
                indexIntoNode_1 =
                    PyInt_FromLong((*obj).index2 as std::os::raw::c_long);
                tuple_1 = PyTuple_New(2 as std::os::raw::c_int as Py_ssize_t);
                PyTuple_SetItem(tuple_1, 0 as std::os::raw::c_int as Py_ssize_t,
                                node_3);
                PyTuple_SetItem(tuple_1, 1 as std::os::raw::c_int as Py_ssize_t,
                                indexIntoNode_1);
                PyList_SetItem(list_0, 1 as std::os::raw::c_int as Py_ssize_t,
                               tuple_1);
                ret = list_0
            }
        }
        7 => {
            let mut set: xmlLocationSetPtr = 0 as *mut xmlLocationSet;
            set = (*obj).user as xmlLocationSetPtr;
            if !set.is_null() && (*set).locNr > 0 as std::os::raw::c_int {
                let mut i_1: std::os::raw::c_int = 0;
                let mut list_1: *mut PyObject = 0 as *mut PyObject;
                list_1 = PyList_New((*set).locNr as Py_ssize_t);
                i_1 = 0 as std::os::raw::c_int;
                while i_1 < (*set).locNr {
                    let mut setobj: xmlXPathObjectPtr =
                        0 as *mut xmlXPathObject;
                    let mut pyobj: *mut PyObject = 0 as *mut PyObject;
                    setobj = *(*set).locTab.offset(i_1 as isize);
                    pyobj = libxml_xmlXPathObjectPtrWrap(setobj);
                    /* xmlXPathFreeObject(setobj) is called */
                    let ref mut fresh18 = *(*set).locTab.offset(i_1 as isize);
                    *fresh18 = 0 as xmlXPathObjectPtr;
                    PyList_SetItem(list_1, i_1 as Py_ssize_t, pyobj);
                    i_1 += 1
                }
                (*set).locNr = 0 as std::os::raw::c_int;
                ret = list_1
            } else {
                let ref mut fresh19 =
                    (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
                *fresh19 += 1;
                ret = &mut _Py_NoneStruct
            }
        }
        _ => {
            let ref mut fresh20 =
                (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
            *fresh20 += 1;
            ret = &mut _Py_NoneStruct
        }
    }
    xmlXPathFreeObject(obj);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn libxml_xmlXPathObjectPtrConvert(mut obj:
                                                             *mut PyObject)
 -> xmlXPathObjectPtr {
    let mut ret: xmlXPathObjectPtr = 0 as xmlXPathObjectPtr;
    if obj.is_null() { return 0 as xmlXPathObjectPtr }
    if (*obj).ob_type == &mut PyFloat_Type as *mut PyTypeObject ||
           PyType_IsSubtype((*obj).ob_type, &mut PyFloat_Type) != 0 {
        ret = xmlXPathNewFloat((*(obj as *mut PyFloatObject)).ob_fval)
    } else if (*(*obj).ob_type).tp_flags &
                  (1 as std::os::raw::c_long) << 24 as std::os::raw::c_int !=
                  0 as std::os::raw::c_int as std::os::raw::c_long {
        ret =
            xmlXPathNewFloat((*(obj as *mut PyIntObject)).ob_ival as
                                 std::os::raw::c_double)
    } else if (*obj).ob_type == &mut PyBool_Type as *mut PyTypeObject {
        if obj == &mut _Py_TrueStruct as *mut PyIntObject as *mut PyObject {
            ret = xmlXPathNewBoolean(1 as std::os::raw::c_int)
        } else { ret = xmlXPathNewBoolean(0 as std::os::raw::c_int) }
    } else if (*(*obj).ob_type).tp_flags &
                  (1 as std::os::raw::c_long) << 27 as std::os::raw::c_int !=
                  0 as std::os::raw::c_int as std::os::raw::c_long {
        let mut str: *mut xmlChar = 0 as *mut xmlChar;
        str =
            xmlStrndup((*(obj as *mut PyStringObject)).ob_sval.as_mut_ptr() as
                           *const xmlChar,
                       (*(obj as *mut PyVarObject)).ob_size as std::os::raw::c_int);
        ret = xmlXPathWrapString(str)
    } else if (*(*obj).ob_type).tp_flags &
                  (1 as std::os::raw::c_long) << 28 as std::os::raw::c_int !=
                  0 as std::os::raw::c_int as std::os::raw::c_long {
        let mut str_0: *mut xmlChar = 0 as *mut xmlChar;
        let mut b: *mut PyObject = 0 as *mut PyObject;
        b = PyUnicodeUCS4_AsUTF8String(obj);
        if !b.is_null() {
            str_0 =
                xmlStrndup((*(b as *mut PyStringObject)).ob_sval.as_mut_ptr()
                               as *const xmlChar,
                           (*(b as *mut PyVarObject)).ob_size as std::os::raw::c_int);
            (*b).ob_refcnt -= 1;
            if !((*b).ob_refcnt != 0 as std::os::raw::c_int as std::os::raw::c_long) {
                Some((*(*b).ob_type).tp_dealloc.expect("non-null function pointer")).expect("non-null function pointer")(b);
            }
        }
        ret = xmlXPathWrapString(str_0)
    } else if (*(*obj).ob_type).tp_flags &
                  (1 as std::os::raw::c_long) << 25 as std::os::raw::c_int !=
                  0 as std::os::raw::c_int as std::os::raw::c_long {
        let mut i: std::os::raw::c_int = 0;
        let mut node: *mut PyObject = 0 as *mut PyObject;
        let mut cur: xmlNodePtr = 0 as *mut xmlNode;
        let mut set: xmlNodeSetPtr = 0 as *mut xmlNodeSet;
        set = xmlXPathNodeSetCreate(0 as xmlNodePtr);
        i = 0 as std::os::raw::c_int;
        while (i as std::os::raw::c_long) < PyList_Size(obj) {
            node = PyList_GetItem(obj, i as Py_ssize_t);
            if !(node.is_null() || (*node).ob_type.is_null()) {
                cur = 0 as xmlNodePtr;
                if (*node).ob_type == &mut PyCapsule_Type as *mut PyTypeObject
                   {
                    cur =
                        if node == &mut _Py_NoneStruct as *mut PyObject {
                            0 as xmlNodePtr
                        } else { (*(node as *mut PyxmlNode_Object)).obj }
                } else if PyObject_HasAttrString(node,
                                                 b"_o\x00" as *const u8 as
                                                     *const std::os::raw::c_char as
                                                     *mut std::os::raw::c_char) != 0
                              &&
                              PyObject_HasAttrString(node,
                                                     b"get_doc\x00" as
                                                         *const u8 as
                                                         *const std::os::raw::c_char
                                                         as *mut std::os::raw::c_char)
                                  != 0 {
                    let mut wrapper: *mut PyObject = 0 as *mut PyObject;
                    wrapper =
                        PyObject_GetAttrString(node,
                                               b"_o\x00" as *const u8 as
                                                   *const std::os::raw::c_char as
                                                   *mut std::os::raw::c_char);
                    if !wrapper.is_null() {
                        cur =
                            if wrapper == &mut _Py_NoneStruct as *mut PyObject
                               {
                                0 as xmlNodePtr
                            } else {
                                (*(wrapper as *mut PyxmlNode_Object)).obj
                            }
                    }
                }
                if !cur.is_null() { xmlXPathNodeSetAdd(set, cur); }
            }
            i += 1
        }
        ret = xmlXPathWrapNodeSet(set)
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn libxml_xmlValidCtxtPtrWrap(mut valid:
                                                        xmlValidCtxtPtr)
 -> *mut PyObject {
    let mut ret: *mut PyObject = 0 as *mut PyObject;
    if valid.is_null() {
        let ref mut fresh21 =
            (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
        *fresh21 += 1;
        return &mut _Py_NoneStruct
    }
    ret =
        PyCapsule_New(valid as *mut std::os::raw::c_void,
                      b"xmlValidCtxtPtr\x00" as *const u8 as
                          *const std::os::raw::c_char as *mut std::os::raw::c_char, None);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn libxml_xmlCatalogPtrWrap(mut catal: xmlCatalogPtr)
 -> *mut PyObject {
    let mut ret: *mut PyObject = 0 as *mut PyObject;
    if catal.is_null() {
        let ref mut fresh22 =
            (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
        *fresh22 += 1;
        return &mut _Py_NoneStruct
    }
    ret =
        PyCapsule_New(catal as *mut std::os::raw::c_void,
                      b"xmlCatalogPtr\x00" as *const u8 as *const std::os::raw::c_char
                          as *mut std::os::raw::c_char, None);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn libxml_xmlOutputBufferPtrWrap(mut buffer:
                                                           xmlOutputBufferPtr)
 -> *mut PyObject {
    let mut ret: *mut PyObject = 0 as *mut PyObject;
    if buffer.is_null() {
        let ref mut fresh23 =
            (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
        *fresh23 += 1;
        return &mut _Py_NoneStruct
    }
    ret =
        PyCapsule_New(buffer as *mut std::os::raw::c_void,
                      b"xmlOutputBufferPtr\x00" as *const u8 as
                          *const std::os::raw::c_char as *mut std::os::raw::c_char, None);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn libxml_xmlParserInputBufferPtrWrap(mut buffer:
                                                                xmlParserInputBufferPtr)
 -> *mut PyObject {
    let mut ret: *mut PyObject = 0 as *mut PyObject;
    if buffer.is_null() {
        let ref mut fresh24 =
            (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
        *fresh24 += 1;
        return &mut _Py_NoneStruct
    }
    ret =
        PyCapsule_New(buffer as *mut std::os::raw::c_void,
                      b"xmlParserInputBufferPtr\x00" as *const u8 as
                          *const std::os::raw::c_char as *mut std::os::raw::c_char, None);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn libxml_xmlRegexpPtrWrap(mut regexp: xmlRegexpPtr)
 -> *mut PyObject {
    let mut ret: *mut PyObject = 0 as *mut PyObject;
    if regexp.is_null() {
        let ref mut fresh25 =
            (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
        *fresh25 += 1;
        return &mut _Py_NoneStruct
    }
    ret =
        PyCapsule_New(regexp as *mut std::os::raw::c_void,
                      b"xmlRegexpPtr\x00" as *const u8 as *const std::os::raw::c_char
                          as *mut std::os::raw::c_char, None);
    return ret;
}
/* LIBXML_REGEXP_ENABLED */
#[no_mangle]
pub unsafe extern "C" fn libxml_xmlTextReaderPtrWrap(mut reader:
                                                         xmlTextReaderPtr)
 -> *mut PyObject {
    let mut ret: *mut PyObject = 0 as *mut PyObject;
    if reader.is_null() {
        let ref mut fresh26 =
            (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
        *fresh26 += 1;
        return &mut _Py_NoneStruct
    }
    ret =
        PyCapsule_New(reader as *mut std::os::raw::c_void,
                      b"xmlTextReaderPtr\x00" as *const u8 as
                          *const std::os::raw::c_char as *mut std::os::raw::c_char, None);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn libxml_xmlTextReaderLocatorPtrWrap(mut locator:
                                                                xmlTextReaderLocatorPtr)
 -> *mut PyObject {
    let mut ret: *mut PyObject = 0 as *mut PyObject;
    if locator.is_null() {
        let ref mut fresh27 =
            (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
        *fresh27 += 1;
        return &mut _Py_NoneStruct
    }
    ret =
        PyCapsule_New(locator,
                      b"xmlTextReaderLocatorPtr\x00" as *const u8 as
                          *const std::os::raw::c_char as *mut std::os::raw::c_char, None);
    return ret;
}
/* LIBXML_READER_ENABLED */
#[no_mangle]
pub unsafe extern "C" fn libxml_xmlRelaxNGPtrWrap(mut ctxt: xmlRelaxNGPtr)
 -> *mut PyObject {
    let mut ret: *mut PyObject = 0 as *mut PyObject;
    if ctxt.is_null() {
        let ref mut fresh28 =
            (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
        *fresh28 += 1;
        return &mut _Py_NoneStruct
    }
    ret =
        PyCapsule_New(ctxt as *mut std::os::raw::c_void,
                      b"xmlRelaxNGPtr\x00" as *const u8 as *const std::os::raw::c_char
                          as *mut std::os::raw::c_char, None);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn libxml_xmlRelaxNGParserCtxtPtrWrap(mut ctxt:
                                                                xmlRelaxNGParserCtxtPtr)
 -> *mut PyObject {
    let mut ret: *mut PyObject = 0 as *mut PyObject;
    if ctxt.is_null() {
        let ref mut fresh29 =
            (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
        *fresh29 += 1;
        return &mut _Py_NoneStruct
    }
    ret =
        PyCapsule_New(ctxt as *mut std::os::raw::c_void,
                      b"xmlRelaxNGParserCtxtPtr\x00" as *const u8 as
                          *const std::os::raw::c_char as *mut std::os::raw::c_char, None);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn libxml_xmlRelaxNGValidCtxtPtrWrap(mut valid:
                                                               xmlRelaxNGValidCtxtPtr)
 -> *mut PyObject {
    let mut ret: *mut PyObject = 0 as *mut PyObject;
    if valid.is_null() {
        let ref mut fresh30 =
            (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
        *fresh30 += 1;
        return &mut _Py_NoneStruct
    }
    ret =
        PyCapsule_New(valid as *mut std::os::raw::c_void,
                      b"xmlRelaxNGValidCtxtPtr\x00" as *const u8 as
                          *const std::os::raw::c_char as *mut std::os::raw::c_char, None);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn libxml_xmlSchemaPtrWrap(mut ctxt: xmlSchemaPtr)
 -> *mut PyObject {
    let mut ret: *mut PyObject = 0 as *mut PyObject;
    if ctxt.is_null() {
        let ref mut fresh31 =
            (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
        *fresh31 += 1;
        return &mut _Py_NoneStruct
    }
    ret =
        PyCapsule_New(ctxt as *mut std::os::raw::c_void,
                      b"xmlSchemaPtr\x00" as *const u8 as *const std::os::raw::c_char
                          as *mut std::os::raw::c_char, None);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn libxml_xmlSchemaParserCtxtPtrWrap(mut ctxt:
                                                               xmlSchemaParserCtxtPtr)
 -> *mut PyObject {
    let mut ret: *mut PyObject = 0 as *mut PyObject;
    if ctxt.is_null() {
        let ref mut fresh32 =
            (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
        *fresh32 += 1;
        return &mut _Py_NoneStruct
    }
    ret =
        PyCapsule_New(ctxt as *mut std::os::raw::c_void,
                      b"xmlSchemaParserCtxtPtr\x00" as *const u8 as
                          *const std::os::raw::c_char as *mut std::os::raw::c_char, None);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn libxml_xmlSchemaValidCtxtPtrWrap(mut valid:
                                                              xmlSchemaValidCtxtPtr)
 -> *mut PyObject {
    let mut ret: *mut PyObject = 0 as *mut PyObject;
    if valid.is_null() {
        let ref mut fresh33 =
            (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
        *fresh33 += 1;
        return &mut _Py_NoneStruct
    }
    ret =
        PyCapsule_New(valid as *mut std::os::raw::c_void,
                      b"xmlSchemaValidCtxtPtr\x00" as *const u8 as
                          *const std::os::raw::c_char as *mut std::os::raw::c_char, None);
    return ret;
}
/*
 * for older versions of Python, we don't use PyBytes, but keep PyString
 * and don't use Capsule but CObjects
 */
/* *
 * ATTRIBUTE_UNUSED:
 *
 * Macro used to signal to GCC unused function parameters
 * Repeated here since the definition is not available when
 * compiled outside the libxml2 build tree.
 */
/* ATTRIBUTE_UNUSED */
/* LIBXML_REGEXP_ENABLED */
/* FILE * have their own internal representation */
/* LIBXML_SCHEMAS_ENABLED */
/* LIBXML_REGEXP_ENABLED */
/* LIBXML_SCHEMAS_ENABLED */
/* LIBXML_SCHEMAS_ENABLED */
#[no_mangle]
pub unsafe extern "C" fn libxml_xmlErrorPtrWrap(mut error: xmlErrorPtr)
 -> *mut PyObject {
    let mut ret: *mut PyObject = 0 as *mut PyObject;
    if error.is_null() {
        let ref mut fresh34 =
            (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
        *fresh34 += 1;
        return &mut _Py_NoneStruct
    }
    ret =
        PyCapsule_New(error as *mut std::os::raw::c_void,
                      b"xmlErrorPtr\x00" as *const u8 as *const std::os::raw::c_char
                          as *mut std::os::raw::c_char, None);
    return ret;
}
