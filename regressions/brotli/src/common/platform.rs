use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}
pub type size_t = libc::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn BrotliDefaultAllocFunc(
    mut opaque: *mut libc::c_void,
    mut size: size_t,
) -> *mut /* owning */ libc::c_void {
    return malloc(size);
}
#[no_mangle]
pub unsafe extern "C" fn BrotliDefaultFreeFunc(
    mut opaque: *mut libc::c_void,
    mut address: *mut /* owning */ libc::c_void,
) {
    free(address);
}
