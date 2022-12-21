
pub type __uint32_t = std::os::raw::c_uint;
pub type __uint64_t = std::os::raw::c_ulong;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type size_t = std::os::raw::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlNodeCommon {
    pub ptr1: *mut std::os::raw::c_void,
    pub type_0: std::os::raw::c_uint,
}
// Cross-checks are only supported on Linux,
// and this doesn't compile on Mac OS anyway
#[no_mangle]
pub unsafe extern "C" fn __c2rust_hash_internal_state_struct(mut x:
                                                                 *mut std::os::raw::c_void,
                                                             mut depth:
                                                                 size_t)
 -> uint64_t {
    return 0xabcd0001 as std::os::raw::c_uint as uint64_t;
}
#[no_mangle]
pub unsafe extern "C" fn __c2rust_hash_lzma_internal_s_struct(mut x:
                                                                  *mut std::os::raw::c_void,
                                                              mut depth:
                                                                  size_t)
 -> uint64_t {
    return 0xabcd0002 as std::os::raw::c_uint as uint64_t;
}
#[no_mangle]
pub unsafe extern "C" fn xcheck_hash_xmlSchemaValDate(mut x:
                                                          *mut std::os::raw::c_void,
                                                      mut depth: size_t)
 -> uint64_t {
    return 0xabcd0003 as std::os::raw::c_uint as uint64_t;
}
#[no_mangle]
pub unsafe extern "C" fn xcheck_hash_xmlSchemaValDecimal(mut x:
                                                             *mut std::os::raw::c_void,
                                                         mut depth: size_t)
 -> uint64_t {
    return 0xabcd0004 as std::os::raw::c_uint as uint64_t;
}
#[no_mangle]
pub unsafe extern "C" fn xcheck_hash_xmlSchemaType(mut x: *mut std::os::raw::c_void,
                                                   mut depth: size_t)
 -> uint64_t {
    return ((0xabcd0005 as std::os::raw::c_ulonglong) << 32 as std::os::raw::c_int |
                *(x as *mut uint32_t) as std::os::raw::c_ulonglong) as uint64_t;
}
#[no_mangle]
pub unsafe extern "C" fn xcheck_hash_xmlSchemaAttribute(mut x:
                                                            *mut std::os::raw::c_void,
                                                        mut depth: size_t)
 -> uint64_t {
    return ((0xabcd0005 as std::os::raw::c_ulonglong) << 32 as std::os::raw::c_int |
                *(x as *mut uint32_t) as std::os::raw::c_ulonglong) as uint64_t;
}
#[no_mangle]
pub unsafe extern "C" fn xcheck_hash_xmlSchemaWildcard(mut x:
                                                           *mut std::os::raw::c_void,
                                                       mut depth: size_t)
 -> uint64_t {
    return ((0xabcd0005 as std::os::raw::c_ulonglong) << 32 as std::os::raw::c_int |
                *(x as *mut uint32_t) as std::os::raw::c_ulonglong) as uint64_t;
}
#[no_mangle]
pub unsafe extern "C" fn xcheck_hash_xmlSchemaAttributeGroup(mut x:
                                                                 *mut std::os::raw::c_void,
                                                             mut depth:
                                                                 size_t)
 -> uint64_t {
    return ((0xabcd0005 as std::os::raw::c_ulonglong) << 32 as std::os::raw::c_int |
                *(x as *mut uint32_t) as std::os::raw::c_ulonglong) as uint64_t;
}
#[no_mangle]
pub unsafe extern "C" fn xcheck_hash_xmlSchemaElement(mut x:
                                                          *mut std::os::raw::c_void,
                                                      mut depth: size_t)
 -> uint64_t {
    return ((0xabcd0005 as std::os::raw::c_ulonglong) << 32 as std::os::raw::c_int |
                *(x as *mut uint32_t) as std::os::raw::c_ulonglong) as uint64_t;
}
#[no_mangle]
pub unsafe extern "C" fn xcheck_hash_xmlSchemaFacet(mut x: *mut std::os::raw::c_void,
                                                    mut depth: size_t)
 -> uint64_t {
    return ((0xabcd0005 as std::os::raw::c_ulonglong) << 32 as std::os::raw::c_int |
                *(x as *mut uint32_t) as std::os::raw::c_ulonglong) as uint64_t;
}
#[no_mangle]
pub unsafe extern "C" fn xcheck_hash_xmlSchemaNotation(mut x:
                                                           *mut std::os::raw::c_void,
                                                       mut depth: size_t)
 -> uint64_t {
    return ((0xabcd0005 as std::os::raw::c_ulonglong) << 32 as std::os::raw::c_int |
                *(x as *mut uint32_t) as std::os::raw::c_ulonglong) as uint64_t;
}
#[no_mangle]
pub unsafe extern "C" fn xcheck_hash_xmlNode(mut x: *mut std::os::raw::c_void,
                                             mut depth: size_t) -> uint64_t {
    let mut nc: *mut _xmlNodeCommon = x as *mut _xmlNodeCommon;
    return ((0xabcd0006 as std::os::raw::c_ulonglong) << 32 as std::os::raw::c_int |
                (*nc).type_0 as std::os::raw::c_ulonglong) as uint64_t;
}
// __linux__
