
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn heman_image_texel(
        _: *mut heman_image,
        x: std::os::raw::c_int,
        y: std::os::raw::c_int,
    ) -> *mut std::os::raw::c_float;
    fn __assert_fail(
        __assertion: *const std::os::raw::c_char,
        __file: *const std::os::raw::c_char,
        __line: std::os::raw::c_uint,
        __function: *const std::os::raw::c_char,
    ) -> !;
    fn fclose(__stream: *mut FILE) -> std::os::raw::c_int;
    fn fopen(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn fputc(__c: std::os::raw::c_int, __stream: *mut FILE) -> std::os::raw::c_int;
    fn fwrite(
        _: *const std::os::raw::c_void,
        _: std::os::raw::c_ulong,
        _: std::os::raw::c_ulong,
        _: *mut FILE,
    ) -> std::os::raw::c_ulong;
    fn free(_: *mut std::os::raw::c_void);
    fn malloc(_: std::os::raw::c_ulong) -> *mut std::os::raw::c_void;
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
pub type FILE = _IO_FILE;
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
pub type size_t = std::os::raw::c_ulong;
pub type __off64_t = std::os::raw::c_long;
pub type _IO_lock_t = ();
pub type __off_t = std::os::raw::c_long;
#[no_mangle]
pub unsafe extern "C" fn heman_export_ply(
    mut img: *mut heman_image,
    mut filename: *const std::os::raw::c_char,
) {
    if (*img).nbands == 1 as std::os::raw::c_int {} else {
        __assert_fail(
            b"img->nbands == 1\0" as *const u8 as *const std::os::raw::c_char,
            b"../src/export.c\0" as *const u8 as *const std::os::raw::c_char,
            8 as std::os::raw::c_int as std::os::raw::c_uint,
            (*::std::mem::transmute::<
                &[u8; 51],
                &[std::os::raw::c_char; 51],
            >(b"void heman_export_ply(heman_image *, const char *)\0"))
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
    let mut vert: [std::os::raw::c_float; 3] = [0.; 3];
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
    let mut face: [std::os::raw::c_int; 5] = [0; 5];
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
pub unsafe extern "C" fn heman_export_with_colors_ply(
    mut hmap: *mut heman_image,
    mut colors: *mut heman_image,
    mut filename: *const std::os::raw::c_char,
) {
    let mut width = (*hmap).width;
    let mut height = (*hmap).height;
    if (*hmap).nbands == 1 as std::os::raw::c_int {} else {
        __assert_fail(
            b"hmap->nbands == 1\0" as *const u8 as *const std::os::raw::c_char,
            b"../src/export.c\0" as *const u8 as *const std::os::raw::c_char,
            57 as std::os::raw::c_int as std::os::raw::c_uint,
            (*::std::mem::transmute::<
                &[u8; 78],
                &[std::os::raw::c_char; 78],
            >(
                b"void heman_export_with_colors_ply(heman_image *, heman_image *, const char *)\0",
            ))
                .as_ptr(),
        );
    }
    if (*colors).nbands == 3 as std::os::raw::c_int {} else {
        __assert_fail(
            b"colors->nbands == 3\0" as *const u8 as *const std::os::raw::c_char,
            b"../src/export.c\0" as *const u8 as *const std::os::raw::c_char,
            58 as std::os::raw::c_int as std::os::raw::c_uint,
            (*::std::mem::transmute::<
                &[u8; 78],
                &[std::os::raw::c_char; 78],
            >(
                b"void heman_export_with_colors_ply(heman_image *, heman_image *, const char *)\0",
            ))
                .as_ptr(),
        );
    }
    if (*colors).width == width {} else {
        __assert_fail(
            b"colors->width == width\0" as *const u8 as *const std::os::raw::c_char,
            b"../src/export.c\0" as *const u8 as *const std::os::raw::c_char,
            59 as std::os::raw::c_int as std::os::raw::c_uint,
            (*::std::mem::transmute::<
                &[u8; 78],
                &[std::os::raw::c_char; 78],
            >(
                b"void heman_export_with_colors_ply(heman_image *, heman_image *, const char *)\0",
            ))
                .as_ptr(),
        );
    }
    if (*colors).height == height {} else {
        __assert_fail(
            b"colors->height == height\0" as *const u8 as *const std::os::raw::c_char,
            b"../src/export.c\0" as *const u8 as *const std::os::raw::c_char,
            60 as std::os::raw::c_int as std::os::raw::c_uint,
            (*::std::mem::transmute::<
                &[u8; 78],
                &[std::os::raw::c_char; 78],
            >(
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
    heman_export_u8(colors, 0.0f64 as std::os::raw::c_float, 1.0f64 as std::os::raw::c_float, colordata);
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
    let mut vert: [std::os::raw::c_float; 3] = [0.; 3];
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
    let mut face: [std::os::raw::c_int; 5] = [0; 5];
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
pub unsafe extern "C" fn heman_export_u8(
    mut source: *mut heman_image,
    mut minv: std::os::raw::c_float,
    mut maxv: std::os::raw::c_float,
    mut outp: *mut heman_byte,
) {
    let mut inp: *const std::os::raw::c_float = (*source).data;
    let mut scale = 1.0f32 / (maxv - minv);
    let mut size = (*source).height * (*source).width * (*source).nbands;
    let mut i = 0 as std::os::raw::c_int;
    while i < size {
        let fresh0 = inp;
        inp = inp.offset(1);
        let mut v = 255 as std::os::raw::c_int as std::os::raw::c_float * (*fresh0 - minv) * scale;
        let fresh1 = outp;
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
