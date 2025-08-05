use ::libc;
extern "C" {
    fn BrotliDefaultAllocFunc(
        opaque: *mut libc::c_void,
        size: size_t
    ) -> *mut libc::c_void;
    fn BrotliDefaultFreeFunc(
        opaque: *mut libc::c_void,
        address: *mut libc::c_void
    );
    fn exit(_: libc::c_int) -> !;
}
pub type size_t = libc::c_ulong;
pub type brotli_alloc_func = Option<
    unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void
>;
pub type brotli_free_func =
    Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MemoryManager {
    pub alloc_func: brotli_alloc_func,
    pub free_func: brotli_free_func,
    pub opaque: *mut libc::c_void
}
#[no_mangle]
pub unsafe extern "C" fn BrotliInitMemoryManager(
    mut m: *mut MemoryManager,
    mut alloc_func: brotli_alloc_func,
    mut free_func: brotli_free_func,
    mut opaque: *mut libc::c_void
) {
    if alloc_func.is_none() {
        (*m).alloc_func = Some(
            BrotliDefaultAllocFunc
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    size_t
                ) -> *mut libc::c_void
        );
        (*m).free_func = Some(
            BrotliDefaultFreeFunc
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void
                ) -> ()
        );
        (*m).opaque = 0 as *mut libc::c_void;
    } else {
        (*m).alloc_func = alloc_func;
        (*m).free_func = free_func;
        (*m).opaque = opaque;
    };
}
#[no_mangle]
pub unsafe extern "C" fn BrotliAllocate(
    mut m: *mut MemoryManager,
    mut n: size_t
) -> *mut libc::c_void {
    let mut result: *mut libc::c_void =
        ((*m).alloc_func).expect("non-null function pointer")((*m).opaque, n);
    if result.is_null() {
        exit(1 as libc::c_int);
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn BrotliFree(
    mut m: *mut MemoryManager,
    mut p: *mut libc::c_void
) {
    ((*m).free_func).expect("non-null function pointer")((*m).opaque, p);
}
#[no_mangle]
pub unsafe extern "C" fn BrotliWipeOutMemoryManager(mut m: *mut MemoryManager) {
}
