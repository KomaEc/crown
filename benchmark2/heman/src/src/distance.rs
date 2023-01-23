use ::libc;
extern "C" {
    fn heman_image_create(
        width: libc::c_int,
        height: libc::c_int,
        nbands: libc::c_int,
    ) -> *mut heman_image;
    fn heman_image_destroy(_: *mut heman_image);
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn sqrt(_: libc::c_double) -> libc::c_double;
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
pub type uint16_t = __uint16_t;
pub type __uint16_t = libc::c_ushort;
#[no_mangle]
pub static mut INF: libc::c_float = 1E20f64 as libc::c_float;
unsafe extern "C" fn edt(
    mut f: *mut libc::c_float,
    mut d: *mut libc::c_float,
    mut z: *mut libc::c_float,
    mut w: *mut uint16_t,
    mut n: libc::c_int,
) {
    let mut k = 0 as libc::c_int;
    let mut s: libc::c_float = 0.;
    *w.offset(0 as libc::c_int as isize) = 0 as libc::c_int as uint16_t;
    *z.offset(0 as libc::c_int as isize) = -INF;
    *z.offset(1 as libc::c_int as isize) = INF;
    let mut q = 1 as libc::c_int;
    while q < n {
        s = (*f.offset(q as isize) + (q * q) as libc::c_float
            - (*f.offset(*w.offset(k as isize) as isize)
                + (*w.offset(k as isize) as libc::c_int
                    * *w.offset(k as isize) as libc::c_int) as libc::c_float))
            / (2 as libc::c_int * q
                - 2 as libc::c_int * *w.offset(k as isize) as libc::c_int)
                as libc::c_float;
        while s <= *z.offset(k as isize) {
            k -= 1;
            s = (*f.offset(q as isize) + (q * q) as libc::c_float
                - (*f.offset(*w.offset(k as isize) as isize)
                    + (*w.offset(k as isize) as libc::c_int
                        * *w.offset(k as isize) as libc::c_int) as libc::c_float))
                / (2 as libc::c_int * q
                    - 2 as libc::c_int * *w.offset(k as isize) as libc::c_int)
                    as libc::c_float;
        }
        k += 1;
        *w.offset(k as isize) = q as uint16_t;
        *z.offset(k as isize) = s;
        *z.offset((k + 1 as libc::c_int) as isize) = INF;
        q += 1;
    }
    k = 0 as libc::c_int;
    let mut q_0 = 0 as libc::c_int;
    while q_0 < n {
        while *z.offset((k + 1 as libc::c_int) as isize) < q_0 as libc::c_float {
            k += 1;
        }
        *d
            .offset(
                q_0 as isize,
            ) = ((q_0 - *w.offset(k as isize) as libc::c_int)
            * (q_0 - *w.offset(k as isize) as libc::c_int)) as libc::c_float
            + *f.offset(*w.offset(k as isize) as isize);
        q_0 += 1;
    }
}
unsafe extern "C" fn edt_with_payload(
    mut f: *mut libc::c_float,
    mut d: *mut libc::c_float,
    mut z: *mut libc::c_float,
    mut w: *mut uint16_t,
    mut n: libc::c_int,
    mut payload_in: *mut libc::c_float,
    mut payload_out: *mut libc::c_float,
) {
    let mut k = 0 as libc::c_int;
    let mut s: libc::c_float = 0.;
    *w.offset(0 as libc::c_int as isize) = 0 as libc::c_int as uint16_t;
    *z.offset(0 as libc::c_int as isize) = -INF;
    *z.offset(1 as libc::c_int as isize) = INF;
    let mut q = 1 as libc::c_int;
    while q < n {
        s = (*f.offset(q as isize) + (q * q) as libc::c_float
            - (*f.offset(*w.offset(k as isize) as isize)
                + (*w.offset(k as isize) as libc::c_int
                    * *w.offset(k as isize) as libc::c_int) as libc::c_float))
            / (2 as libc::c_int * q
                - 2 as libc::c_int * *w.offset(k as isize) as libc::c_int)
                as libc::c_float;
        while s <= *z.offset(k as isize) {
            k -= 1;
            s = (*f.offset(q as isize) + (q * q) as libc::c_float
                - (*f.offset(*w.offset(k as isize) as isize)
                    + (*w.offset(k as isize) as libc::c_int
                        * *w.offset(k as isize) as libc::c_int) as libc::c_float))
                / (2 as libc::c_int * q
                    - 2 as libc::c_int * *w.offset(k as isize) as libc::c_int)
                    as libc::c_float;
        }
        k += 1;
        *w.offset(k as isize) = q as uint16_t;
        *z.offset(k as isize) = s;
        *z.offset((k + 1 as libc::c_int) as isize) = INF;
        q += 1;
    }
    k = 0 as libc::c_int;
    let mut q_0 = 0 as libc::c_int;
    while q_0 < n {
        while *z.offset((k + 1 as libc::c_int) as isize) < q_0 as libc::c_float {
            k += 1;
        }
        *d
            .offset(
                q_0 as isize,
            ) = ((q_0 - *w.offset(k as isize) as libc::c_int)
            * (q_0 - *w.offset(k as isize) as libc::c_int)) as libc::c_float
            + *f.offset(*w.offset(k as isize) as isize);
        *payload_out
            .offset(
                (q_0 * 2 as libc::c_int) as isize,
            ) = *payload_in
            .offset((*w.offset(k as isize) as libc::c_int * 2 as libc::c_int) as isize);
        *payload_out
            .offset(
                (q_0 * 2 as libc::c_int + 1 as libc::c_int) as isize,
            ) = *payload_in
            .offset(
                (*w.offset(k as isize) as libc::c_int * 2 as libc::c_int
                    + 1 as libc::c_int) as isize,
            );
        q_0 += 1;
    }
}
unsafe extern "C" fn transform_to_distance(mut sdf: *mut heman_image) {
    let mut width = (*sdf).width;
    let mut height = (*sdf).height;
    let mut size = width * height;
    let mut ff = calloc(
        size as libc::c_ulong,
        ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
    ) as *mut libc::c_float;
    let mut dd = calloc(
        size as libc::c_ulong,
        ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
    ) as *mut libc::c_float;
    let mut zz = calloc(
        ((height + 1 as libc::c_int) * (width + 1 as libc::c_int)) as libc::c_ulong,
        ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
    ) as *mut libc::c_float;
    let mut ww = calloc(
        size as libc::c_ulong,
        ::std::mem::size_of::<uint16_t>() as libc::c_ulong,
    ) as *mut uint16_t;
    let mut x: libc::c_int = 0;
    x = 0 as libc::c_int;
    while x < width {
        let mut f = ff.offset((height * x) as isize);
        let mut d = dd.offset((height * x) as isize);
        let mut z = zz.offset(((height + 1 as libc::c_int) * x) as isize);
        let mut w = ww.offset((height * x) as isize);
        let mut y = 0 as libc::c_int;
        while y < height {
            *f
                .offset(
                    y as isize,
                ) = *((*sdf).data).offset((y * width) as isize).offset(x as isize);
            y += 1;
        }
        edt(f, d, z, w, height);
        let mut y_0 = 0 as libc::c_int;
        while y_0 < height {
            *((*sdf).data)
                .offset((y_0 * width) as isize)
                .offset(x as isize) = *d.offset(y_0 as isize);
            y_0 += 1;
        }
        x += 1;
    }
    let mut y_1: libc::c_int = 0;
    y_1 = 0 as libc::c_int;
    while y_1 < height {
        let mut f_0 = ff.offset((width * y_1) as isize);
        let mut d_0 = dd.offset((width * y_1) as isize);
        let mut z_0 = zz.offset(((width + 1 as libc::c_int) * y_1) as isize);
        let mut w_0 = ww.offset((width * y_1) as isize);
        let mut x_0 = 0 as libc::c_int;
        while x_0 < width {
            *f_0
                .offset(
                    x_0 as isize,
                ) = *((*sdf).data).offset((y_1 * width) as isize).offset(x_0 as isize);
            x_0 += 1;
        }
        edt(f_0, d_0, z_0, w_0, width);
        let mut x_1 = 0 as libc::c_int;
        while x_1 < width {
            *((*sdf).data)
                .offset((y_1 * width) as isize)
                .offset(x_1 as isize) = *d_0.offset(x_1 as isize);
            x_1 += 1;
        }
        y_1 += 1;
    }
    free(ff as *mut libc::c_void);
    free(dd as *mut libc::c_void);
    free(zz as *mut libc::c_void);
    free(ww as *mut libc::c_void);
}
unsafe extern "C" fn transform_to_coordfield(
    mut sdf: *mut heman_image,
    mut cf: *mut heman_image,
) {
    let mut width = (*sdf).width;
    let mut height = (*sdf).height;
    let mut size = width * height;
    let mut ff = calloc(
        size as libc::c_ulong,
        ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
    ) as *mut libc::c_float;
    let mut dd = calloc(
        size as libc::c_ulong,
        ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
    ) as *mut libc::c_float;
    let mut zz = calloc(
        ((height + 1 as libc::c_int) * (width + 1 as libc::c_int)) as libc::c_ulong,
        ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
    ) as *mut libc::c_float;
    let mut ww = calloc(
        size as libc::c_ulong,
        ::std::mem::size_of::<uint16_t>() as libc::c_ulong,
    ) as *mut uint16_t;
    let mut x: libc::c_int = 0;
    x = 0 as libc::c_int;
    while x < width {
        let mut pl1 = calloc(
            (height * 2 as libc::c_int) as libc::c_ulong,
            ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
        ) as *mut libc::c_float;
        let mut pl2 = calloc(
            (height * 2 as libc::c_int) as libc::c_ulong,
            ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
        ) as *mut libc::c_float;
        let mut f = ff.offset((height * x) as isize);
        let mut d = dd.offset((height * x) as isize);
        let mut z = zz.offset(((height + 1 as libc::c_int) * x) as isize);
        let mut w = ww.offset((height * x) as isize);
        let mut y = 0 as libc::c_int;
        while y < height {
            *f
                .offset(
                    y as isize,
                ) = *((*sdf).data).offset((y * width) as isize).offset(x as isize);
            *pl1
                .offset(
                    (y * 2 as libc::c_int) as isize,
                ) = *((*cf).data)
                .offset((2 as libc::c_int * (y * width + x)) as isize)
                .offset(0 as libc::c_int as isize);
            *pl1
                .offset(
                    (y * 2 as libc::c_int + 1 as libc::c_int) as isize,
                ) = *((*cf).data)
                .offset((2 as libc::c_int * (y * width + x)) as isize)
                .offset(1 as libc::c_int as isize);
            y += 1;
        }
        edt_with_payload(f, d, z, w, height, pl1, pl2);
        let mut y_0 = 0 as libc::c_int;
        while y_0 < height {
            *((*sdf).data)
                .offset((y_0 * width) as isize)
                .offset(x as isize) = *d.offset(y_0 as isize);
            *((*cf).data)
                .offset((2 as libc::c_int * (y_0 * width + x)) as isize)
                .offset(
                    0 as libc::c_int as isize,
                ) = *pl2.offset((2 as libc::c_int * y_0) as isize);
            *((*cf).data)
                .offset((2 as libc::c_int * (y_0 * width + x)) as isize)
                .offset(
                    1 as libc::c_int as isize,
                ) = *pl2.offset((2 as libc::c_int * y_0 + 1 as libc::c_int) as isize);
            y_0 += 1;
        }
        free(pl1 as *mut libc::c_void);
        free(pl2 as *mut libc::c_void);
        x += 1;
    }
    let mut y_1: libc::c_int = 0;
    y_1 = 0 as libc::c_int;
    while y_1 < height {
        let mut pl1_0 = calloc(
            (width * 2 as libc::c_int) as libc::c_ulong,
            ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
        ) as *mut libc::c_float;
        let mut pl2_0 = calloc(
            (width * 2 as libc::c_int) as libc::c_ulong,
            ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
        ) as *mut libc::c_float;
        let mut f_0 = ff.offset((width * y_1) as isize);
        let mut d_0 = dd.offset((width * y_1) as isize);
        let mut z_0 = zz.offset(((width + 1 as libc::c_int) * y_1) as isize);
        let mut w_0 = ww.offset((width * y_1) as isize);
        let mut x_0 = 0 as libc::c_int;
        while x_0 < width {
            *f_0
                .offset(
                    x_0 as isize,
                ) = *((*sdf).data).offset((y_1 * width) as isize).offset(x_0 as isize);
            *pl1_0
                .offset(
                    (x_0 * 2 as libc::c_int) as isize,
                ) = *((*cf).data)
                .offset((2 as libc::c_int * (y_1 * width + x_0)) as isize)
                .offset(0 as libc::c_int as isize);
            *pl1_0
                .offset(
                    (x_0 * 2 as libc::c_int + 1 as libc::c_int) as isize,
                ) = *((*cf).data)
                .offset((2 as libc::c_int * (y_1 * width + x_0)) as isize)
                .offset(1 as libc::c_int as isize);
            x_0 += 1;
        }
        edt_with_payload(f_0, d_0, z_0, w_0, width, pl1_0, pl2_0);
        let mut x_1 = 0 as libc::c_int;
        while x_1 < width {
            *((*sdf).data)
                .offset((y_1 * width) as isize)
                .offset(x_1 as isize) = *d_0.offset(x_1 as isize);
            *((*cf).data)
                .offset((2 as libc::c_int * (y_1 * width + x_1)) as isize)
                .offset(
                    0 as libc::c_int as isize,
                ) = *pl2_0.offset((2 as libc::c_int * x_1) as isize);
            *((*cf).data)
                .offset((2 as libc::c_int * (y_1 * width + x_1)) as isize)
                .offset(
                    1 as libc::c_int as isize,
                ) = *pl2_0.offset((2 as libc::c_int * x_1 + 1 as libc::c_int) as isize);
            x_1 += 1;
        }
        free(pl1_0 as *mut libc::c_void);
        free(pl2_0 as *mut libc::c_void);
        y_1 += 1;
    }
    free(ff as *mut libc::c_void);
    free(dd as *mut libc::c_void);
    free(zz as *mut libc::c_void);
    free(ww as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn heman_distance_create_sdf(
    mut src: *mut heman_image,
) -> *mut heman_image {
    if (*src).nbands == 1 as libc::c_int
        && !(b"Distance field input must have only 1 band.\0" as *const u8
            as *const libc::c_char)
            .is_null()
    {} else {
        __assert_fail(
            b"src->nbands == 1 && \"Distance field input must have only 1 band.\"\0"
                as *const u8 as *const libc::c_char,
            b"../src/distance.c\0" as *const u8 as *const libc::c_char,
            183 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 54],
                &[libc::c_char; 54],
            >(b"heman_image *heman_distance_create_sdf(heman_image *)\0"))
                .as_ptr(),
        );
    }
    let mut positive = heman_image_create((*src).width, (*src).height, 1 as libc::c_int);
    let mut negative = heman_image_create((*src).width, (*src).height, 1 as libc::c_int);
    let mut size = (*src).height * (*src).width;
    let mut pptr = (*positive).data;
    let mut nptr = (*negative).data;
    let mut sptr = (*src).data;
    let mut i = 0 as libc::c_int;
    while i < size {
        let fresh0 = pptr;
        pptr = pptr.offset(1);
        *fresh0 = if *sptr != 0. { INF } else { 0 as libc::c_int as libc::c_float };
        let fresh1 = nptr;
        nptr = nptr.offset(1);
        *fresh1 = if *sptr != 0. { 0 as libc::c_int as libc::c_float } else { INF };
        i += 1;
        sptr = sptr.offset(1);
    }
    transform_to_distance(positive);
    transform_to_distance(negative);
    let mut inv = 1.0f32 / (*src).width as libc::c_float;
    pptr = (*positive).data;
    nptr = (*negative).data;
    let mut i_0 = 0 as libc::c_int;
    while i_0 < size {
        *pptr = ((sqrt(*pptr as libc::c_double) - sqrt(*nptr as libc::c_double))
            * inv as libc::c_double) as libc::c_float;
        i_0 += 1;
        pptr = pptr.offset(1);
        nptr = nptr.offset(1);
    }
    heman_image_destroy(negative);
    return positive;
}
#[no_mangle]
pub unsafe extern "C" fn heman_distance_create_df(
    mut src: *mut heman_image,
) -> *mut heman_image {
    if (*src).nbands == 1 as libc::c_int
        && !(b"Distance field input must have only 1 band.\0" as *const u8
            as *const libc::c_char)
            .is_null()
    {} else {
        __assert_fail(
            b"src->nbands == 1 && \"Distance field input must have only 1 band.\"\0"
                as *const u8 as *const libc::c_char,
            b"../src/distance.c\0" as *const u8 as *const libc::c_char,
            208 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 53],
                &[libc::c_char; 53],
            >(b"heman_image *heman_distance_create_df(heman_image *)\0"))
                .as_ptr(),
        );
    }
    let mut positive = heman_image_create((*src).width, (*src).height, 1 as libc::c_int);
    let mut size = (*src).height * (*src).width;
    let mut pptr = (*positive).data;
    let mut sptr = (*src).data;
    let mut i = 0 as libc::c_int;
    while i < size {
        let fresh2 = pptr;
        pptr = pptr.offset(1);
        *fresh2 = if *sptr != 0. { 0 as libc::c_int as libc::c_float } else { INF };
        i += 1;
        sptr = sptr.offset(1);
    }
    transform_to_distance(positive);
    let mut inv = 1.0f32 / (*src).width as libc::c_float;
    pptr = (*positive).data;
    let mut i_0 = 0 as libc::c_int;
    while i_0 < size {
        *pptr = (sqrt(*pptr as libc::c_double) * inv as libc::c_double) as libc::c_float;
        i_0 += 1;
        pptr = pptr.offset(1);
    }
    return positive;
}
#[no_mangle]
pub unsafe extern "C" fn heman_distance_identity_cpcf(
    mut width: libc::c_int,
    mut height: libc::c_int,
) -> *mut heman_image {
    let mut retval = heman_image_create(width, height, 2 as libc::c_int);
    let mut cdata = (*retval).data;
    let mut y = 0 as libc::c_int;
    while y < height {
        let mut x = 0 as libc::c_int;
        while x < width {
            let fresh3 = cdata;
            cdata = cdata.offset(1);
            *fresh3 = x as libc::c_float;
            let fresh4 = cdata;
            cdata = cdata.offset(1);
            *fresh4 = y as libc::c_float;
            x += 1;
        }
        y += 1;
    }
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn heman_distance_create_cpcf(
    mut src: *mut heman_image,
) -> *mut heman_image {
    let mut negative = heman_image_create((*src).width, (*src).height, 1 as libc::c_int);
    let mut size = (*src).height * (*src).width;
    let mut nptr = (*negative).data;
    let mut sptr = (*src).data;
    let mut i = 0 as libc::c_int;
    while i < size {
        let mut val = 0 as libc::c_int as libc::c_float;
        let mut b = 0 as libc::c_int;
        while b < (*src).nbands {
            let fresh5 = sptr;
            sptr = sptr.offset(1);
            val += *fresh5;
            b += 1;
        }
        let fresh6 = nptr;
        nptr = nptr.offset(1);
        *fresh6 = if val != 0. { 0 as libc::c_int as libc::c_float } else { INF };
        i += 1;
    }
    let mut coordfield = heman_distance_identity_cpcf((*src).width, (*src).height);
    transform_to_coordfield(negative, coordfield);
    heman_image_destroy(negative);
    return coordfield;
}
#[no_mangle]
pub unsafe extern "C" fn heman_distance_from_cpcf(
    mut cf: *mut heman_image,
) -> *mut heman_image {
    if (*cf).nbands == 2 as libc::c_int
        && !(b"Coordinate field input must have 2 bands.\0" as *const u8
            as *const libc::c_char)
            .is_null()
    {} else {
        __assert_fail(
            b"cf->nbands == 2 && \"Coordinate field input must have 2 bands.\"\0"
                as *const u8 as *const libc::c_char,
            b"../src/distance.c\0" as *const u8 as *const libc::c_char,
            259 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 53],
                &[libc::c_char; 53],
            >(b"heman_image *heman_distance_from_cpcf(heman_image *)\0"))
                .as_ptr(),
        );
    }
    let mut udf = heman_image_create((*cf).width, (*cf).height, 1 as libc::c_int);
    let mut dptr = (*udf).data;
    let mut sptr = (*cf).data;
    let mut scale = (1.0f32 as libc::c_double
        / sqrt(
            ((*cf).width * (*cf).width + (*cf).height * (*cf).height) as libc::c_double,
        )) as libc::c_float;
    let mut y = 0 as libc::c_int;
    while y < (*cf).height {
        let mut x = 0 as libc::c_int;
        while x < (*cf).width {
            let fresh7 = sptr;
            sptr = sptr.offset(1);
            let mut u = *fresh7;
            let fresh8 = sptr;
            sptr = sptr.offset(1);
            let mut v = *fresh8;
            let mut dist = (sqrt(
                ((u - x as libc::c_float) * (u - x as libc::c_float)
                    + (v - y as libc::c_float) * (v - y as libc::c_float))
                    as libc::c_double,
            ) * scale as libc::c_double) as libc::c_float;
            let fresh9 = dptr;
            dptr = dptr.offset(1);
            *fresh9 = dist;
            x += 1;
        }
        y += 1;
    }
    return udf;
}
