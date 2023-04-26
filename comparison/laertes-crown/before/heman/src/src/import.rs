
extern "C" {
    fn heman_image_create(
        width: std::os::raw::c_int,
        height: std::os::raw::c_int,
        nbands: std::os::raw::c_int,
    ) -> *mut heman_image;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct heman_image_s {
    pub width: std::os::raw::c_int,
    pub height: std::os::raw::c_int,
    pub nbands: std::os::raw::c_int,
    pub data: *mut std::os::raw::c_float,
}
pub type heman_image = heman_image_s;
pub type heman_byte = std::os::raw::c_uchar;
#[no_mangle]
pub unsafe extern "C" fn heman_import_u8(
    mut width: std::os::raw::c_int,
    mut height: std::os::raw::c_int,
    mut nbands: std::os::raw::c_int,
    mut source: *const heman_byte,
    mut minval: std::os::raw::c_float,
    mut maxval: std::os::raw::c_float,
) -> *mut heman_image {
    let mut result = heman_image_create(width, height, nbands);
    let mut inp = source;
    let mut outp = (*result).data;
    let mut scale = (maxval - minval) / 255.0f32;
    let mut size = height * width * nbands;
    let mut i = 0 as std::os::raw::c_int;
    while i < size {
        let fresh0 = inp;
        inp = inp.offset(1);
        let mut v = *fresh0 as std::os::raw::c_int as std::os::raw::c_float * scale + minval;
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
