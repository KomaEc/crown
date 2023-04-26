fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        quadtree_rs::src::test::main_0(
            (args.len() - 1) as libc::c_int,
            args.as_mut_ptr() as *mut *const libc::c_char,
        );
    }
}
