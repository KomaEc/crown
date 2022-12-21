
extern "C" {
    #[no_mangle]
    fn gzclose_r(file: gzFile) -> std::os::raw::c_int;
}
pub type __off64_t = std::os::raw::c_long;
pub type off64_t = __off64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gzFile_s {
    pub have: std::os::raw::c_uint,
    pub next: *mut std::os::raw::c_uchar,
    pub pos: off64_t,
}
pub type gzFile = *mut gzFile_s;
/* gzclose.c -- zlib gzclose() function
 * Copyright (C) 2004, 2010 Mark Adler
 * For conditions of distribution and use, see copyright notice in zlib.h
 */
/* gzclose() is in a separate file so that it is linked in only if it is used.
   That way the other gzclose functions can be used instead to avoid linking in
   unneeded compression or decompression routines. */
#[no_mangle]
pub unsafe extern "C" fn gzclose(mut file: gzFile) -> std::os::raw::c_int {
    return gzclose_r(file);
}
