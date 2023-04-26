
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    
    fn __assert_fail(
        __assertion: * const std::os::raw::c_char,
        __file: * const std::os::raw::c_char,
        __line: std::os::raw::c_uint,
        __function: * const std::os::raw::c_char,
    ) -> !;
    fn fclose(__stream: * mut crate::src::src::export::_IO_FILE) -> std::os::raw::c_int;
    fn fopen(_: * const std::os::raw::c_char, _: * const std::os::raw::c_char) -> * mut crate::src::src::export::_IO_FILE;
    fn fprintf(_: * mut crate::src::src::export::_IO_FILE, _: * const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn fputc(__c: std::os::raw::c_int, __stream: * mut crate::src::src::export::_IO_FILE) -> std::os::raw::c_int;
    fn fwrite(
        _: * const core::ffi::c_void,
        _: std::os::raw::c_ulong,
        _: std::os::raw::c_ulong,
        _: * mut crate::src::src::export::_IO_FILE,
    ) -> std::os::raw::c_ulong;
    fn free(_: * mut core::ffi::c_void);
    fn malloc(_: std::os::raw::c_ulong) -> * mut core::ffi::c_void;
}
pub use crate::src::src::image::heman_image_texel;
// #[derive(Copy, Clone)]

pub type heman_image_s = crate::src::src::color::heman_image_s;
pub type heman_image = crate::src::src::color::heman_image_s;
pub type heman_byte = std::os::raw::c_uchar;
pub type FILE = crate::src::src::export::_IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: std::os::raw::c_int,
    pub _IO_read_ptr: * mut std::os::raw::c_char,
    pub _IO_read_end: * mut std::os::raw::c_char,
    pub _IO_read_base: * mut std::os::raw::c_char,
    pub _IO_write_base: * mut std::os::raw::c_char,
    pub _IO_write_ptr: * mut std::os::raw::c_char,
    pub _IO_write_end: * mut std::os::raw::c_char,
    pub _IO_buf_base: * mut std::os::raw::c_char,
    pub _IO_buf_end: * mut std::os::raw::c_char,
    pub _IO_save_base: * mut std::os::raw::c_char,
    pub _IO_backup_base: * mut std::os::raw::c_char,
    pub _IO_save_end: * mut std::os::raw::c_char,
    pub _markers: * mut crate::src::src::export::_IO_marker,
    pub _chain: * mut crate::src::src::export::_IO_FILE,
    pub _fileno: std::os::raw::c_int,
    pub _flags2: std::os::raw::c_int,
    pub _old_offset: std::os::raw::c_long,
    pub _cur_column: std::os::raw::c_ushort,
    pub _vtable_offset: std::os::raw::c_schar,
    pub _shortbuf: [std::os::raw::c_char; 1],
    pub _lock: * mut core::ffi::c_void,
    pub _offset: std::os::raw::c_long,
    pub _codecvt: * mut crate::src::src::export::_IO_codecvt,
    pub _wide_data: * mut crate::src::src::export::_IO_wide_data,
    pub _freeres_list: * mut crate::src::src::export::_IO_FILE,
    pub _freeres_buf: * mut core::ffi::c_void,
    pub __pad5: std::os::raw::c_ulong,
    pub _mode: std::os::raw::c_int,
    pub _unused2: [std::os::raw::c_char; 20],
}
impl _IO_FILE {
    pub const fn new() -> Self {
        _IO_FILE {
        _flags: 0,
        _IO_read_ptr: (0 as * mut std::os::raw::c_char),
        _IO_read_end: (0 as * mut std::os::raw::c_char),
        _IO_read_base: (0 as * mut std::os::raw::c_char),
        _IO_write_base: (0 as * mut std::os::raw::c_char),
        _IO_write_ptr: (0 as * mut std::os::raw::c_char),
        _IO_write_end: (0 as * mut std::os::raw::c_char),
        _IO_buf_base: (0 as * mut std::os::raw::c_char),
        _IO_buf_end: (0 as * mut std::os::raw::c_char),
        _IO_save_base: (0 as * mut std::os::raw::c_char),
        _IO_backup_base: (0 as * mut std::os::raw::c_char),
        _IO_save_end: (0 as * mut std::os::raw::c_char),
        _markers: (0 as * mut crate::src::src::export::_IO_marker),
        _chain: (0 as * mut crate::src::src::export::_IO_FILE),
        _fileno: 0,
        _flags2: 0,
        _old_offset: 0,
        _cur_column: 0,
        _vtable_offset: 0,
        _shortbuf: [0,],
        _lock: (0 as * mut core::ffi::c_void),
        _offset: 0,
        _codecvt: (0 as * mut crate::src::src::export::_IO_codecvt),
        _wide_data: (0 as * mut crate::src::src::export::_IO_wide_data),
        _freeres_list: (0 as * mut crate::src::src::export::_IO_FILE),
        _freeres_buf: (0 as * mut core::ffi::c_void),
        __pad5: 0,
        _mode: 0,
        _unused2: [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,]
        }
    }
}

impl std::default::Default for _IO_FILE {
    fn default() -> Self { _IO_FILE::new() }
}

pub type size_t = std::os::raw::c_ulong;
pub type __off64_t = std::os::raw::c_long;
pub type _IO_lock_t = ();
pub type __off_t = std::os::raw::c_long;
#[no_mangle]
pub unsafe extern "C" fn heman_export_ply(
    mut img: * mut crate::src::src::color::heman_image_s,
    mut filename: * const std::os::raw::c_char,
) {
    if (*img).nbands == 1 as std::os::raw::c_int {} else {
        __assert_fail(
            b"img->nbands == 1\0" as *const u8 as *const std::os::raw::c_char,
            b"../src/export.c\0" as *const u8 as *const std::os::raw::c_char,
            8 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 51], &'_ [i8; 51]>(b"void heman_export_ply(heman_image *, const char *)\0"))
                .as_ptr(),
        );
    }
    let mut fout = fopen(filename, b"wb\0" as *const u8 as *const std::os::raw::c_char);
    let mut ncols = (*img).width - 1 as std::os::raw::c_int;
    let mut nrows = (*img).height - 1 as std::os::raw::c_int;
    let mut ncells = ncols * nrows;
    let mut nverts = (*img).width * (*img).height;
    fprintf(
        fout,
        b"ply\nformat binary_little_endian 1.0\ncomment heman\nelement vertex %d\nproperty float32 x\nproperty float32 y\nproperty float32 z\nelement face %d\nproperty list int32 int32 vertex_indices\nend_header\n\0"
            as *const u8 as *const std::os::raw::c_char,
        nverts,
        ncells,
    );
    let mut invw = 2.0f32 / (*img).width as std::os::raw::c_float;
    let mut invh = 2.0f32 / (*img).height as std::os::raw::c_float;
    let mut vert: [f32; 3] = [0.; 3];
    let mut j = 0 as std::os::raw::c_int;
    while j < (*img).height {
        let mut i = 0 as std::os::raw::c_int;
        while i < (*img).width {
            vert[0 as std::os::raw::c_int
                as usize] = -(1 as std::os::raw::c_int) as std::os::raw::c_float
                + i as std::os::raw::c_float * invw;
            vert[1 as std::os::raw::c_int
                as usize] = -(1 as std::os::raw::c_int) as std::os::raw::c_float
                + j as std::os::raw::c_float * invh;
            vert[2 as std::os::raw::c_int as usize] = *heman_image_texel(img, i, j);
            fwrite(
                vert.as_mut_ptr() as *const std::os::raw::c_void,
                ::std::mem::size_of::<[std::os::raw::c_float; 3]>() as std::os::raw::c_ulong,
                1 as std::os::raw::c_int as std::os::raw::c_ulong,
                fout,
            );
            i += 1;
        }
        j += 1;
    }
    let mut face: [i32; 5] = [0; 5];
    face[0 as std::os::raw::c_int as usize] = 4 as std::os::raw::c_int;
    let mut j_0 = 0 as std::os::raw::c_int;
    while j_0 < nrows {
        let mut p = j_0 * (*img).width;
        let mut i_0 = 0 as std::os::raw::c_int;
        while i_0 < ncols {
            face[1 as std::os::raw::c_int as usize] = p;
            face[2 as std::os::raw::c_int as usize] = p + 1 as std::os::raw::c_int;
            face[3 as std::os::raw::c_int as usize] = p + (*img).width + 1 as std::os::raw::c_int;
            face[4 as std::os::raw::c_int as usize] = p + (*img).width;
            fwrite(
                face.as_mut_ptr() as *const std::os::raw::c_void,
                ::std::mem::size_of::<[std::os::raw::c_int; 5]>() as std::os::raw::c_ulong,
                1 as std::os::raw::c_int as std::os::raw::c_ulong,
                fout,
            );
            i_0 += 1;
            p += 1;
        }
        j_0 += 1;
    }
    fclose(fout);
}
#[no_mangle]
pub unsafe extern "C" fn heman_export_with_colors_ply<'a1>(
    mut hmap: * mut crate::src::src::color::heman_image_s,
    mut colors: Option<&'a1 mut crate::src::src::color::heman_image_s>,
    mut filename: * const std::os::raw::c_char,
) {
    let mut width = (*hmap).width;
    let mut height = (*hmap).height;
    if (*hmap).nbands == 1 as std::os::raw::c_int {} else {
        __assert_fail(
            b"hmap->nbands == 1\0" as *const u8 as *const std::os::raw::c_char,
            b"../src/export.c\0" as *const u8 as *const std::os::raw::c_char,
            57 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 78], &'_ [i8; 78]>(
                b"void heman_export_with_colors_ply(heman_image *, heman_image *, const char *)\0",
            ))
                .as_ptr(),
        );
    }
    if (*(borrow(& colors)).unwrap()).nbands == 3 as std::os::raw::c_int {} else {
        __assert_fail(
            b"colors->nbands == 3\0" as *const u8 as *const std::os::raw::c_char,
            b"../src/export.c\0" as *const u8 as *const std::os::raw::c_char,
            58 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 78], &'_ [i8; 78]>(
                b"void heman_export_with_colors_ply(heman_image *, heman_image *, const char *)\0",
            ))
                .as_ptr(),
        );
    }
    if (*(borrow(& colors)).unwrap()).width == width {} else {
        __assert_fail(
            b"colors->width == width\0" as *const u8 as *const std::os::raw::c_char,
            b"../src/export.c\0" as *const u8 as *const std::os::raw::c_char,
            59 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 78], &'_ [i8; 78]>(
                b"void heman_export_with_colors_ply(heman_image *, heman_image *, const char *)\0",
            ))
                .as_ptr(),
        );
    }
    if (*(borrow(& colors)).unwrap()).height == height {} else {
        __assert_fail(
            b"colors->height == height\0" as *const u8 as *const std::os::raw::c_char,
            b"../src/export.c\0" as *const u8 as *const std::os::raw::c_char,
            60 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 78], &'_ [i8; 78]>(
                b"void heman_export_with_colors_ply(heman_image *, heman_image *, const char *)\0",
            ))
                .as_ptr(),
        );
    }
    let mut fout = fopen(filename, b"wb\0" as *const u8 as *const std::os::raw::c_char);
    let mut ncols = (*hmap).width - 1 as std::os::raw::c_int;
    let mut nrows = (*hmap).height - 1 as std::os::raw::c_int;
    let mut ncells = ncols * nrows;
    let mut nverts = (*hmap).width * (*hmap).height;
    let mut colordata = malloc((width * height * 3 as std::os::raw::c_int) as std::os::raw::c_ulong)
        as *mut std::os::raw::c_uchar;
    heman_export_u8(borrow_mut(&mut colors), 0.0f64 as std::os::raw::c_float, 1.0f64 as std::os::raw::c_float, colordata);
    fprintf(
        fout,
        b"ply\nformat binary_little_endian 1.0\ncomment heman\nelement vertex %d\nproperty float32 x\nproperty float32 y\nproperty float32 z\nproperty uchar red\nproperty uchar green\nproperty uchar blue\nproperty uchar alpha\nelement face %d\nproperty list int32 int32 vertex_indices\nend_header\n\0"
            as *const u8 as *const std::os::raw::c_char,
        nverts,
        ncells,
    );
    let mut invw = 2.0f32 / width as std::os::raw::c_float;
    let mut invh = 2.0f32 / height as std::os::raw::c_float;
    let mut pcolor = colordata;
    let mut vert: [f32; 3] = [0.; 3];
    let mut j = 0 as std::os::raw::c_int;
    while j < height {
        let mut i = 0 as std::os::raw::c_int;
        while i < width {
            vert[0 as std::os::raw::c_int
                as usize] = -(1 as std::os::raw::c_int) as std::os::raw::c_float
                + i as std::os::raw::c_float * invw;
            vert[1 as std::os::raw::c_int
                as usize] = -(1 as std::os::raw::c_int) as std::os::raw::c_float
                + j as std::os::raw::c_float * invh;
            vert[2 as std::os::raw::c_int as usize] = *heman_image_texel(hmap, i, j);
            fwrite(
                vert.as_mut_ptr() as *const std::os::raw::c_void,
                ::std::mem::size_of::<[std::os::raw::c_float; 3]>() as std::os::raw::c_ulong,
                1 as std::os::raw::c_int as std::os::raw::c_ulong,
                fout,
            );
            fwrite(
                pcolor as *const std::os::raw::c_void,
                3 as std::os::raw::c_int as std::os::raw::c_ulong,
                1 as std::os::raw::c_int as std::os::raw::c_ulong,
                fout,
            );
            pcolor = pcolor.offset(3 as std::os::raw::c_int as isize);
            fputc(255 as std::os::raw::c_int, fout);
            i += 1;
        }
        j += 1;
    }
    let mut face: [i32; 5] = [0; 5];
    face[0 as std::os::raw::c_int as usize] = 4 as std::os::raw::c_int;
    let mut j_0 = 0 as std::os::raw::c_int;
    while j_0 < nrows {
        let mut p = j_0 * width;
        let mut i_0 = 0 as std::os::raw::c_int;
        while i_0 < ncols {
            face[1 as std::os::raw::c_int as usize] = p;
            face[2 as std::os::raw::c_int as usize] = p + 1 as std::os::raw::c_int;
            face[3 as std::os::raw::c_int as usize] = p + (*hmap).width + 1 as std::os::raw::c_int;
            face[4 as std::os::raw::c_int as usize] = p + (*hmap).width;
            fwrite(
                face.as_mut_ptr() as *const std::os::raw::c_void,
                ::std::mem::size_of::<[std::os::raw::c_int; 5]>() as std::os::raw::c_ulong,
                1 as std::os::raw::c_int as std::os::raw::c_ulong,
                fout,
            );
            i_0 += 1;
            p += 1;
        }
        j_0 += 1;
    }
    fclose(fout);
    free(colordata as *mut std::os::raw::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn heman_export_u8<'a1>(
    mut source: Option<&'a1 mut crate::src::src::color::heman_image_s>,
    mut minv: std::os::raw::c_float,
    mut maxv: std::os::raw::c_float,
    mut outp: * mut std::os::raw::c_uchar,
) {
    let mut inp: * const f32 = (*(borrow(& source)).unwrap()).data;
    let mut scale = 1.0f32 / (maxv - minv);
    let mut size = (*(borrow(& source)).unwrap()).height * (*(borrow(& source)).unwrap()).width * (*(borrow(& source)).unwrap()).nbands;
    let mut i = 0 as std::os::raw::c_int;
    while i < size {
        let mut fresh0 = inp;
        inp = inp.offset(1);
        let mut v = 255 as std::os::raw::c_int as std::os::raw::c_float * (*fresh0 - minv) * scale;
        let mut fresh1 = outp;
        outp = outp.offset(1);
        *fresh1 = (if 0 as std::os::raw::c_int as std::os::raw::c_float
            > (if 255 as std::os::raw::c_int as std::os::raw::c_float > v {
                v
            } else {
                255 as std::os::raw::c_int as std::os::raw::c_float
            })
        {
            0 as std::os::raw::c_int as std::os::raw::c_float
        } else if 255 as std::os::raw::c_int as std::os::raw::c_float > v {
            v
        } else {
            255 as std::os::raw::c_int as std::os::raw::c_float
        }) as heman_byte;
        i += 1;
    }
}
use crate::laertes_rt::*;
