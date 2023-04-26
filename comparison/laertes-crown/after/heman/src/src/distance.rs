
extern "C" {
    
    
    fn __assert_fail(
        __assertion: * const std::os::raw::c_char,
        __file: * const std::os::raw::c_char,
        __line: std::os::raw::c_uint,
        __function: * const std::os::raw::c_char,
    ) -> !;
    fn calloc(_: std::os::raw::c_ulong, _: std::os::raw::c_ulong) -> * mut core::ffi::c_void;
    fn free(_: * mut core::ffi::c_void);
    fn sqrt(_: std::os::raw::c_double) -> std::os::raw::c_double;
}
pub use crate::src::src::image::heman_image_create;
pub use crate::src::src::image::heman_image_destroy;
// #[derive(Copy, Clone)]

pub type heman_image_s = crate::src::src::color::heman_image_s;
pub type heman_image = crate::src::src::color::heman_image_s;
pub type uint16_t = std::os::raw::c_ushort;
pub type __uint16_t = std::os::raw::c_ushort;
#[no_mangle]
pub static mut INF: std::os::raw::c_float = 0.0; unsafe fn laertes_init_INF() {
INF = 1E20f64 as std::os::raw::c_float;}//;
unsafe extern "C" fn edt(
    mut f: * mut std::os::raw::c_float,
    mut d: * mut std::os::raw::c_float,
    mut z: * mut std::os::raw::c_float,
    mut w: * mut std::os::raw::c_ushort,
    mut n: std::os::raw::c_int,
) {
    let mut k = 0 as std::os::raw::c_int;
    let mut s: f32 = 0.;
    *w.offset(0 as std::os::raw::c_int as isize) = 0 as std::os::raw::c_int as uint16_t;
    *z.offset(0 as std::os::raw::c_int as isize) = -INF;
    *z.offset(1 as std::os::raw::c_int as isize) = INF;
    let mut q = 1 as std::os::raw::c_int;
    while q < n {
        s = (*f.offset(q as isize) + (q * q) as std::os::raw::c_float
            - (*f.offset(*w.offset(k as isize) as isize)
                + (*w.offset(k as isize) as std::os::raw::c_int
                    * *w.offset(k as isize) as std::os::raw::c_int) as std::os::raw::c_float))
            / (2 as std::os::raw::c_int * q
                - 2 as std::os::raw::c_int * *w.offset(k as isize) as std::os::raw::c_int)
                as std::os::raw::c_float;
        while s <= *z.offset(k as isize) {
            k -= 1;
            s = (*f.offset(q as isize) + (q * q) as std::os::raw::c_float
                - (*f.offset(*w.offset(k as isize) as isize)
                    + (*w.offset(k as isize) as std::os::raw::c_int
                        * *w.offset(k as isize) as std::os::raw::c_int) as std::os::raw::c_float))
                / (2 as std::os::raw::c_int * q
                    - 2 as std::os::raw::c_int * *w.offset(k as isize) as std::os::raw::c_int)
                    as std::os::raw::c_float;
        }
        k += 1;
        *w.offset(k as isize) = q as uint16_t;
        *z.offset(k as isize) = s;
        *z.offset((k + 1 as std::os::raw::c_int) as isize) = INF;
        q += 1;
    }
    k = 0 as std::os::raw::c_int;
    let mut q_0 = 0 as std::os::raw::c_int;
    while q_0 < n {
        while *z.offset((k + 1 as std::os::raw::c_int) as isize) < q_0 as std::os::raw::c_float {
            k += 1;
        }
        *d
            .offset(
                q_0 as isize,
            ) = ((q_0 - *w.offset(k as isize) as std::os::raw::c_int)
            * (q_0 - *w.offset(k as isize) as std::os::raw::c_int)) as std::os::raw::c_float
            + *f.offset(*w.offset(k as isize) as isize);
        q_0 += 1;
    }
}
unsafe extern "C" fn edt_with_payload(
    mut f: * mut std::os::raw::c_float,
    mut d: * mut std::os::raw::c_float,
    mut z: * mut std::os::raw::c_float,
    mut w: * mut std::os::raw::c_ushort,
    mut n: std::os::raw::c_int,
    mut payload_in: * mut std::os::raw::c_float,
    mut payload_out: * mut std::os::raw::c_float,
) {
    let mut k = 0 as std::os::raw::c_int;
    let mut s: f32 = 0.;
    *w.offset(0 as std::os::raw::c_int as isize) = 0 as std::os::raw::c_int as uint16_t;
    *z.offset(0 as std::os::raw::c_int as isize) = -INF;
    *z.offset(1 as std::os::raw::c_int as isize) = INF;
    let mut q = 1 as std::os::raw::c_int;
    while q < n {
        s = (*f.offset(q as isize) + (q * q) as std::os::raw::c_float
            - (*f.offset(*w.offset(k as isize) as isize)
                + (*w.offset(k as isize) as std::os::raw::c_int
                    * *w.offset(k as isize) as std::os::raw::c_int) as std::os::raw::c_float))
            / (2 as std::os::raw::c_int * q
                - 2 as std::os::raw::c_int * *w.offset(k as isize) as std::os::raw::c_int)
                as std::os::raw::c_float;
        while s <= *z.offset(k as isize) {
            k -= 1;
            s = (*f.offset(q as isize) + (q * q) as std::os::raw::c_float
                - (*f.offset(*w.offset(k as isize) as isize)
                    + (*w.offset(k as isize) as std::os::raw::c_int
                        * *w.offset(k as isize) as std::os::raw::c_int) as std::os::raw::c_float))
                / (2 as std::os::raw::c_int * q
                    - 2 as std::os::raw::c_int * *w.offset(k as isize) as std::os::raw::c_int)
                    as std::os::raw::c_float;
        }
        k += 1;
        *w.offset(k as isize) = q as uint16_t;
        *z.offset(k as isize) = s;
        *z.offset((k + 1 as std::os::raw::c_int) as isize) = INF;
        q += 1;
    }
    k = 0 as std::os::raw::c_int;
    let mut q_0 = 0 as std::os::raw::c_int;
    while q_0 < n {
        while *z.offset((k + 1 as std::os::raw::c_int) as isize) < q_0 as std::os::raw::c_float {
            k += 1;
        }
        *d
            .offset(
                q_0 as isize,
            ) = ((q_0 - *w.offset(k as isize) as std::os::raw::c_int)
            * (q_0 - *w.offset(k as isize) as std::os::raw::c_int)) as std::os::raw::c_float
            + *f.offset(*w.offset(k as isize) as isize);
        *payload_out
            .offset(
                (q_0 * 2 as std::os::raw::c_int) as isize,
            ) = *payload_in
            .offset((*w.offset(k as isize) as std::os::raw::c_int * 2 as std::os::raw::c_int) as isize);
        *payload_out
            .offset(
                (q_0 * 2 as std::os::raw::c_int + 1 as std::os::raw::c_int) as isize,
            ) = *payload_in
            .offset(
                (*w.offset(k as isize) as std::os::raw::c_int * 2 as std::os::raw::c_int
                    + 1 as std::os::raw::c_int) as isize,
            );
        q_0 += 1;
    }
}
unsafe extern "C" fn transform_to_distance(mut sdf: * mut crate::src::src::color::heman_image_s) {
    let mut width = (*sdf).width;
    let mut height = (*sdf).height;
    let mut size = width * height;
    let mut ff = calloc(
        size as std::os::raw::c_ulong,
        ::std::mem::size_of::<std::os::raw::c_float>() as std::os::raw::c_ulong,
    ) as *mut std::os::raw::c_float;
    let mut dd = calloc(
        size as std::os::raw::c_ulong,
        ::std::mem::size_of::<std::os::raw::c_float>() as std::os::raw::c_ulong,
    ) as *mut std::os::raw::c_float;
    let mut zz = calloc(
        ((height + 1 as std::os::raw::c_int) * (width + 1 as std::os::raw::c_int)) as std::os::raw::c_ulong,
        ::std::mem::size_of::<std::os::raw::c_float>() as std::os::raw::c_ulong,
    ) as *mut std::os::raw::c_float;
    let mut ww = calloc(
        size as std::os::raw::c_ulong,
        ::std::mem::size_of::<uint16_t>() as std::os::raw::c_ulong,
    ) as *mut uint16_t;
    let mut x: i32 = 0;
    x = 0 as std::os::raw::c_int;
    while x < width {
        let mut f = ff.offset((height * x) as isize);
        let mut d = dd.offset((height * x) as isize);
        let mut z = zz.offset(((height + 1 as std::os::raw::c_int) * x) as isize);
        let mut w = ww.offset((height * x) as isize);
        let mut y = 0 as std::os::raw::c_int;
        while y < height {
            *f
                .offset(
                    y as isize,
                ) = *((*sdf).data).offset((y * width) as isize).offset(x as isize);
            y += 1;
        }
        edt(f, d, z, w, height);
        let mut y_0 = 0 as std::os::raw::c_int;
        while y_0 < height {
            *((*sdf).data)
                .offset((y_0 * width) as isize)
                .offset(x as isize) = *d.offset(y_0 as isize);
            y_0 += 1;
        }
        x += 1;
    }
    let mut y_1: i32 = 0;
    y_1 = 0 as std::os::raw::c_int;
    while y_1 < height {
        let mut f_0 = ff.offset((width * y_1) as isize);
        let mut d_0 = dd.offset((width * y_1) as isize);
        let mut z_0 = zz.offset(((width + 1 as std::os::raw::c_int) * y_1) as isize);
        let mut w_0 = ww.offset((width * y_1) as isize);
        let mut x_0 = 0 as std::os::raw::c_int;
        while x_0 < width {
            *f_0
                .offset(
                    x_0 as isize,
                ) = *((*sdf).data).offset((y_1 * width) as isize).offset(x_0 as isize);
            x_0 += 1;
        }
        edt(f_0, d_0, z_0, w_0, width);
        let mut x_1 = 0 as std::os::raw::c_int;
        while x_1 < width {
            *((*sdf).data)
                .offset((y_1 * width) as isize)
                .offset(x_1 as isize) = *d_0.offset(x_1 as isize);
            x_1 += 1;
        }
        y_1 += 1;
    }
    free(ff as *mut std::os::raw::c_void);
    free(dd as *mut std::os::raw::c_void);
    free(zz as *mut std::os::raw::c_void);
    free(ww as *mut std::os::raw::c_void);
}
unsafe extern "C" fn transform_to_coordfield(
    mut sdf: * mut crate::src::src::color::heman_image_s,
    mut cf: * mut crate::src::src::color::heman_image_s,
) {
    let mut width = (*sdf).width;
    let mut height = (*sdf).height;
    let mut size = width * height;
    let mut ff = calloc(
        size as std::os::raw::c_ulong,
        ::std::mem::size_of::<std::os::raw::c_float>() as std::os::raw::c_ulong,
    ) as *mut std::os::raw::c_float;
    let mut dd = calloc(
        size as std::os::raw::c_ulong,
        ::std::mem::size_of::<std::os::raw::c_float>() as std::os::raw::c_ulong,
    ) as *mut std::os::raw::c_float;
    let mut zz = calloc(
        ((height + 1 as std::os::raw::c_int) * (width + 1 as std::os::raw::c_int)) as std::os::raw::c_ulong,
        ::std::mem::size_of::<std::os::raw::c_float>() as std::os::raw::c_ulong,
    ) as *mut std::os::raw::c_float;
    let mut ww = calloc(
        size as std::os::raw::c_ulong,
        ::std::mem::size_of::<uint16_t>() as std::os::raw::c_ulong,
    ) as *mut uint16_t;
    let mut x: i32 = 0;
    x = 0 as std::os::raw::c_int;
    while x < width {
        let mut pl1 = calloc(
            (height * 2 as std::os::raw::c_int) as std::os::raw::c_ulong,
            ::std::mem::size_of::<std::os::raw::c_float>() as std::os::raw::c_ulong,
        ) as *mut std::os::raw::c_float;
        let mut pl2 = calloc(
            (height * 2 as std::os::raw::c_int) as std::os::raw::c_ulong,
            ::std::mem::size_of::<std::os::raw::c_float>() as std::os::raw::c_ulong,
        ) as *mut std::os::raw::c_float;
        let mut f = ff.offset((height * x) as isize);
        let mut d = dd.offset((height * x) as isize);
        let mut z = zz.offset(((height + 1 as std::os::raw::c_int) * x) as isize);
        let mut w = ww.offset((height * x) as isize);
        let mut y = 0 as std::os::raw::c_int;
        while y < height {
            *f
                .offset(
                    y as isize,
                ) = *((*sdf).data).offset((y * width) as isize).offset(x as isize);
            *pl1
                .offset(
                    (y * 2 as std::os::raw::c_int) as isize,
                ) = *((*cf).data)
                .offset((2 as std::os::raw::c_int * (y * width + x)) as isize)
                .offset(0 as std::os::raw::c_int as isize);
            *pl1
                .offset(
                    (y * 2 as std::os::raw::c_int + 1 as std::os::raw::c_int) as isize,
                ) = *((*cf).data)
                .offset((2 as std::os::raw::c_int * (y * width + x)) as isize)
                .offset(1 as std::os::raw::c_int as isize);
            y += 1;
        }
        edt_with_payload(f, d, z, w, height, pl1, pl2);
        let mut y_0 = 0 as std::os::raw::c_int;
        while y_0 < height {
            *((*sdf).data)
                .offset((y_0 * width) as isize)
                .offset(x as isize) = *d.offset(y_0 as isize);
            *((*cf).data)
                .offset((2 as std::os::raw::c_int * (y_0 * width + x)) as isize)
                .offset(
                    0 as std::os::raw::c_int as isize,
                ) = *pl2.offset((2 as std::os::raw::c_int * y_0) as isize);
            *((*cf).data)
                .offset((2 as std::os::raw::c_int * (y_0 * width + x)) as isize)
                .offset(
                    1 as std::os::raw::c_int as isize,
                ) = *pl2.offset((2 as std::os::raw::c_int * y_0 + 1 as std::os::raw::c_int) as isize);
            y_0 += 1;
        }
        free(pl1 as *mut std::os::raw::c_void);
        free(pl2 as *mut std::os::raw::c_void);
        x += 1;
    }
    let mut y_1: i32 = 0;
    y_1 = 0 as std::os::raw::c_int;
    while y_1 < height {
        let mut pl1_0 = calloc(
            (width * 2 as std::os::raw::c_int) as std::os::raw::c_ulong,
            ::std::mem::size_of::<std::os::raw::c_float>() as std::os::raw::c_ulong,
        ) as *mut std::os::raw::c_float;
        let mut pl2_0 = calloc(
            (width * 2 as std::os::raw::c_int) as std::os::raw::c_ulong,
            ::std::mem::size_of::<std::os::raw::c_float>() as std::os::raw::c_ulong,
        ) as *mut std::os::raw::c_float;
        let mut f_0 = ff.offset((width * y_1) as isize);
        let mut d_0 = dd.offset((width * y_1) as isize);
        let mut z_0 = zz.offset(((width + 1 as std::os::raw::c_int) * y_1) as isize);
        let mut w_0 = ww.offset((width * y_1) as isize);
        let mut x_0 = 0 as std::os::raw::c_int;
        while x_0 < width {
            *f_0
                .offset(
                    x_0 as isize,
                ) = *((*sdf).data).offset((y_1 * width) as isize).offset(x_0 as isize);
            *pl1_0
                .offset(
                    (x_0 * 2 as std::os::raw::c_int) as isize,
                ) = *((*cf).data)
                .offset((2 as std::os::raw::c_int * (y_1 * width + x_0)) as isize)
                .offset(0 as std::os::raw::c_int as isize);
            *pl1_0
                .offset(
                    (x_0 * 2 as std::os::raw::c_int + 1 as std::os::raw::c_int) as isize,
                ) = *((*cf).data)
                .offset((2 as std::os::raw::c_int * (y_1 * width + x_0)) as isize)
                .offset(1 as std::os::raw::c_int as isize);
            x_0 += 1;
        }
        edt_with_payload(f_0, d_0, z_0, w_0, width, pl1_0, pl2_0);
        let mut x_1 = 0 as std::os::raw::c_int;
        while x_1 < width {
            *((*sdf).data)
                .offset((y_1 * width) as isize)
                .offset(x_1 as isize) = *d_0.offset(x_1 as isize);
            *((*cf).data)
                .offset((2 as std::os::raw::c_int * (y_1 * width + x_1)) as isize)
                .offset(
                    0 as std::os::raw::c_int as isize,
                ) = *pl2_0.offset((2 as std::os::raw::c_int * x_1) as isize);
            *((*cf).data)
                .offset((2 as std::os::raw::c_int * (y_1 * width + x_1)) as isize)
                .offset(
                    1 as std::os::raw::c_int as isize,
                ) = *pl2_0.offset((2 as std::os::raw::c_int * x_1 + 1 as std::os::raw::c_int) as isize);
            x_1 += 1;
        }
        free(pl1_0 as *mut std::os::raw::c_void);
        free(pl2_0 as *mut std::os::raw::c_void);
        y_1 += 1;
    }
    free(ff as *mut std::os::raw::c_void);
    free(dd as *mut std::os::raw::c_void);
    free(zz as *mut std::os::raw::c_void);
    free(ww as *mut std::os::raw::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn heman_distance_create_sdf(
    mut src: * mut crate::src::src::color::heman_image_s,
) -> * mut crate::src::src::color::heman_image_s {
    if (*src).nbands == 1 as std::os::raw::c_int
        && !(b"Distance field input must have only 1 band.\0" as *const u8
            as *const std::os::raw::c_char)
            .is_null()
    {} else {
        __assert_fail(
            b"src->nbands == 1 && \"Distance field input must have only 1 band.\"\0"
                as *const u8 as *const std::os::raw::c_char,
            b"../src/distance.c\0" as *const u8 as *const std::os::raw::c_char,
            183 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 54], &'_ [i8; 54]>(b"heman_image *heman_distance_create_sdf(heman_image *)\0"))
                .as_ptr(),
        );
    }
    let mut positive = heman_image_create((*src).width, (*src).height, 1 as std::os::raw::c_int);
    let mut negative = heman_image_create((*src).width, (*src).height, 1 as std::os::raw::c_int);
    let mut size = (*src).height * (*src).width;
    let mut pptr = (*positive).data;
    let mut nptr = (*negative).data;
    let mut sptr = (*src).data;
    let mut i = 0 as std::os::raw::c_int;
    while i < size {
        let mut fresh0 = pptr;
        pptr = pptr.offset(1);
        *fresh0 = if *sptr != 0. { INF } else { 0 as std::os::raw::c_int as std::os::raw::c_float };
        let mut fresh1 = nptr;
        nptr = nptr.offset(1);
        *fresh1 = if *sptr != 0. { 0 as std::os::raw::c_int as std::os::raw::c_float } else { INF };
        i += 1;
        sptr = sptr.offset(1);
    }
    transform_to_distance(positive);
    transform_to_distance(negative);
    let mut inv = 1.0f32 / (*src).width as std::os::raw::c_float;
    pptr = (*positive).data;
    nptr = (*negative).data;
    let mut i_0 = 0 as std::os::raw::c_int;
    while i_0 < size {
        *pptr = ((sqrt(*pptr as std::os::raw::c_double) - sqrt(*nptr as std::os::raw::c_double))
            * inv as std::os::raw::c_double) as std::os::raw::c_float;
        i_0 += 1;
        pptr = pptr.offset(1);
        nptr = nptr.offset(1);
    }
    heman_image_destroy(negative);
    return positive;
}
#[no_mangle]
pub unsafe extern "C" fn heman_distance_create_df<'a1>(
    mut src: Option<&'a1 mut crate::src::src::color::heman_image_s>,
) -> * mut crate::src::src::color::heman_image_s {
    if (*(borrow(& src)).unwrap()).nbands == 1 as std::os::raw::c_int
        && !(b"Distance field input must have only 1 band.\0" as *const u8
            as *const std::os::raw::c_char)
            .is_null()
    {} else {
        __assert_fail(
            b"src->nbands == 1 && \"Distance field input must have only 1 band.\"\0"
                as *const u8 as *const std::os::raw::c_char,
            b"../src/distance.c\0" as *const u8 as *const std::os::raw::c_char,
            208 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 53], &'_ [i8; 53]>(b"heman_image *heman_distance_create_df(heman_image *)\0"))
                .as_ptr(),
        );
    }
    let mut positive = heman_image_create((*(borrow_mut(&mut src)).unwrap()).width, (*(borrow_mut(&mut src)).unwrap()).height, 1 as std::os::raw::c_int);
    let mut size = (*(borrow(& src)).unwrap()).height * (*(borrow(& src)).unwrap()).width;
    let mut pptr = (*positive).data;
    let mut sptr = (*(borrow_mut(&mut src)).unwrap()).data;
    let mut i = 0 as std::os::raw::c_int;
    while i < size {
        let mut fresh2 = pptr;
        pptr = pptr.offset(1);
        *fresh2 = if *sptr != 0. { 0 as std::os::raw::c_int as std::os::raw::c_float } else { INF };
        i += 1;
        sptr = sptr.offset(1);
    }
    transform_to_distance(positive);
    let mut inv = 1.0f32 / (*(borrow(& src)).unwrap()).width as std::os::raw::c_float;
    pptr = (*positive).data;
    let mut i_0 = 0 as std::os::raw::c_int;
    while i_0 < size {
        *pptr = (sqrt(*pptr as std::os::raw::c_double) * inv as std::os::raw::c_double) as std::os::raw::c_float;
        i_0 += 1;
        pptr = pptr.offset(1);
    }
    return positive;
}
#[no_mangle]
pub unsafe extern "C" fn heman_distance_identity_cpcf(
    mut width: std::os::raw::c_int,
    mut height: std::os::raw::c_int,
) -> * mut crate::src::src::color::heman_image_s {
    let mut retval = heman_image_create(width, height, 2 as std::os::raw::c_int);
    let mut cdata = (*retval).data;
    let mut y = 0 as std::os::raw::c_int;
    while y < height {
        let mut x = 0 as std::os::raw::c_int;
        while x < width {
            let mut fresh3 = cdata;
            cdata = cdata.offset(1);
            *fresh3 = x as std::os::raw::c_float;
            let mut fresh4 = cdata;
            cdata = cdata.offset(1);
            *fresh4 = y as std::os::raw::c_float;
            x += 1;
        }
        y += 1;
    }
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn heman_distance_create_cpcf(
    mut src: * mut crate::src::src::color::heman_image_s,
) -> * mut crate::src::src::color::heman_image_s {
    let mut negative = heman_image_create((*src).width, (*src).height, 1 as std::os::raw::c_int);
    let mut size = (*src).height * (*src).width;
    let mut nptr = (*negative).data;
    let mut sptr = (*src).data;
    let mut i = 0 as std::os::raw::c_int;
    while i < size {
        let mut val = 0 as std::os::raw::c_int as std::os::raw::c_float;
        let mut b = 0 as std::os::raw::c_int;
        while b < (*src).nbands {
            let mut fresh5 = sptr;
            sptr = sptr.offset(1);
            val += *fresh5;
            b += 1;
        }
        let mut fresh6 = nptr;
        nptr = nptr.offset(1);
        *fresh6 = if val != 0. { 0 as std::os::raw::c_int as std::os::raw::c_float } else { INF };
        i += 1;
    }
    let mut coordfield = heman_distance_identity_cpcf((*src).width, (*src).height);
    transform_to_coordfield(negative, coordfield);
    heman_image_destroy(negative);
    return coordfield;
}
#[no_mangle]
pub unsafe extern "C" fn heman_distance_from_cpcf<'a1>(
    mut cf: Option<&'a1 mut crate::src::src::color::heman_image_s>,
) -> * mut crate::src::src::color::heman_image_s {
    if (*(borrow(& cf)).unwrap()).nbands == 2 as std::os::raw::c_int
        && !(b"Coordinate field input must have 2 bands.\0" as *const u8
            as *const std::os::raw::c_char)
            .is_null()
    {} else {
        __assert_fail(
            b"cf->nbands == 2 && \"Coordinate field input must have 2 bands.\"\0"
                as *const u8 as *const std::os::raw::c_char,
            b"../src/distance.c\0" as *const u8 as *const std::os::raw::c_char,
            259 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 53], &'_ [i8; 53]>(b"heman_image *heman_distance_from_cpcf(heman_image *)\0"))
                .as_ptr(),
        );
    }
    let mut udf = heman_image_create((*(borrow_mut(&mut cf)).unwrap()).width, (*(borrow_mut(&mut cf)).unwrap()).height, 1 as std::os::raw::c_int);
    let mut dptr = (*udf).data;
    let mut sptr = (*(borrow_mut(&mut cf)).unwrap()).data;
    let mut scale = (1.0f32 as std::os::raw::c_double
        / sqrt(
            ((*(borrow(& cf)).unwrap()).width * (*(borrow(& cf)).unwrap()).width + (*(borrow(& cf)).unwrap()).height * (*(borrow(& cf)).unwrap()).height) as std::os::raw::c_double,
        )) as std::os::raw::c_float;
    let mut y = 0 as std::os::raw::c_int;
    while y < (*(borrow(& cf)).unwrap()).height {
        let mut x = 0 as std::os::raw::c_int;
        while x < (*(borrow(& cf)).unwrap()).width {
            let mut fresh7 = sptr;
            sptr = sptr.offset(1);
            let mut u = *fresh7;
            let mut fresh8 = sptr;
            sptr = sptr.offset(1);
            let mut v = *fresh8;
            let mut dist = (sqrt(
                ((u - x as std::os::raw::c_float) * (u - x as std::os::raw::c_float)
                    + (v - y as std::os::raw::c_float) * (v - y as std::os::raw::c_float))
                    as std::os::raw::c_double,
            ) * scale as std::os::raw::c_double) as std::os::raw::c_float;
            let mut fresh9 = dptr;
            dptr = dptr.offset(1);
            *fresh9 = dist;
            x += 1;
        }
        y += 1;
    }
    return udf;
}
use crate::laertes_rt::*;
