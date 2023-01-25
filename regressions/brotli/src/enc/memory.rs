use ::libc;
extern "C" {
    
    
    fn exit(_: libc::c_int) -> !;
}
pub type size_t = libc::c_ulong;
pub type brotli_alloc_func = Option::<
    unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void,
>;
pub type brotli_free_func = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
>;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor126 { dummy: () }
#[no_mangle]
pub unsafe extern "C" fn BrotliInitMemoryManager(
    mut m: *mut crate::src::enc::backward_references_hq::MemoryManager,
    mut alloc_func: brotli_alloc_func,
    mut free_func: brotli_free_func,
    mut opaque: *mut libc::c_void,
) {
    if alloc_func.is_none() {
        (*m).alloc_func= Some(
            crate::src::common::platform::BrotliDefaultAllocFunc
                as unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void,
        );
        (*m).free_func= Some(
            crate::src::common::platform::BrotliDefaultFreeFunc
                as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
        );
        (*m).opaque= 0 as *mut libc::c_void;
    } else {
        (*m).alloc_func= alloc_func;
        (*m).free_func= free_func;
        (*m).opaque= opaque;
    };
}
#[no_mangle]
pub unsafe extern "C" fn BrotliAllocate(
    mut m: *mut crate::src::enc::backward_references_hq::MemoryManager,
    mut n: size_t,
) -> *mut libc::c_void {
    let mut result = (*m).alloc_func
        .expect("non-null function pointer")((*m).opaque, n);
    if result.is_null() {();
        exit(1 as libc::c_int);
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn BrotliFree(
    mut m: *mut crate::src::enc::backward_references_hq::MemoryManager,
    mut p: *mut libc::c_void,
) {
    (*m).free_func.expect("non-null function pointer")((*m).opaque, p);
}
#[no_mangle]
pub unsafe extern "C" fn BrotliWipeOutMemoryManager(mut m: *mut crate::src::enc::backward_references_hq::MemoryManager) {}
