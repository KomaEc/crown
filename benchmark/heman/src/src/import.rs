use ::libc;
extern "C" {
    fn heman_image_create(
        width: libc::c_int,
        height: libc::c_int,
        nbands: libc::c_int,
    ) -> *mut heman_image;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct heman_image_s {
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub nbands: libc::c_int,
    pub data: *mut libc::c_float,
}
pub type heman_image = heman_image_s;
pub type heman_byte = libc::c_uchar;
#[no_mangle]
pub unsafe extern "C" fn heman_import_u8(
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut nbands: libc::c_int,
    mut source: *const heman_byte,
    mut minval: libc::c_float,
    mut maxval: libc::c_float,
) -> *mut heman_image {
    let mut result = heman_image_create(width, height, nbands);
    let mut inp = source;
    let mut outp = (*result).data;
    let mut scale = (maxval - minval) / 255.0f32;
    let mut size = height * width * nbands;
    let mut i = 0 as libc::c_int;
    while i < size {
        let fresh0 = inp;
        inp = inp.offset(1);
        let mut v = *fresh0 as libc::c_int as libc::c_float * scale + minval;
        let fresh1 = outp;
        outp = outp.offset(1);
        *fresh1 = if minval > (if maxval > v { v } else { maxval }) {
            minval
        } else if maxval > v {
            v
        } else {
            maxval
        };
        i += 1;
    }
    return result;
}
