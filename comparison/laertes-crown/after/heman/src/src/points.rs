
extern "C" {
    
    
    fn rand() -> std::os::raw::c_int;
    fn malloc(_: std::os::raw::c_ulong) -> * mut core::ffi::c_void;
    fn free(_: * mut core::ffi::c_void);
    fn memcpy(
        _: * mut core::ffi::c_void,
        _: * const core::ffi::c_void,
        _: std::os::raw::c_ulong,
    ) -> * mut core::ffi::c_void;
    fn __assert_fail(
        __assertion: * const std::os::raw::c_char,
        __file: * const std::os::raw::c_char,
        __line: std::os::raw::c_uint,
        __function: * const std::os::raw::c_char,
    ) -> !;
    fn sqrtf(_: std::os::raw::c_float) -> std::os::raw::c_float;
    fn sqrt(_: std::os::raw::c_double) -> std::os::raw::c_double;
    fn ceil(_: std::os::raw::c_double) -> std::os::raw::c_double;
    
    
    
    
}
pub use crate::src::kazmath::vec2::kmVec2Add;
pub use crate::src::kazmath::vec2::kmVec2LengthSq;
pub use crate::src::kazmath::vec2::kmVec2Scale;
pub use crate::src::kazmath::vec2::kmVec2Subtract;
pub use crate::src::src::image::heman_image_create;
pub use crate::src::src::image::heman_image_sample;
// #[derive(Copy, Clone)]

pub type heman_image_s = crate::src::src::color::heman_image_s;
pub type heman_image = crate::src::src::color::heman_image_s;
pub type heman_points = crate::src::src::color::heman_image_s;
// #[derive(Copy, Clone)]

pub type kmVec2 = crate::src::kazmath::aabb2::kmVec2;
#[no_mangle]
pub extern "C" fn randhash(mut seed: std::os::raw::c_uint) -> std::os::raw::c_uint {
    let mut i = (seed ^ 12345391 as std::os::raw::c_uint)
        .wrapping_mul(2654435769 as std::os::raw::c_uint);
    i ^= i << 6 as std::os::raw::c_int ^ i >> 26 as std::os::raw::c_int;
    i = i.wrapping_mul(2654435769 as std::os::raw::c_uint);
    i = i.wrapping_add(i << 5 as std::os::raw::c_int ^ i >> 12 as std::os::raw::c_int);
    return i;
}
#[no_mangle]
pub extern "C" fn randhashf(
    mut seed: std::os::raw::c_uint,
    mut a: std::os::raw::c_float,
    mut b: std::os::raw::c_float,
) -> std::os::raw::c_float {
    return (b - a) * randhash(seed) as std::os::raw::c_float
        / (2147483647 as std::os::raw::c_int as std::os::raw::c_uint)
            .wrapping_mul(2 as std::os::raw::c_uint)
            .wrapping_add(1 as std::os::raw::c_uint) as std::os::raw::c_float + a;
}
#[no_mangle]
pub unsafe extern "C" fn heman_points_create(
    mut xy: * mut std::os::raw::c_float,
    mut npoints: std::os::raw::c_int,
    mut nbands: std::os::raw::c_int,
) -> Option<Box<crate::src::src::color::heman_image_s>> {
    let mut img = Some(Box::new(crate::src::src::color::heman_image_s::new()));
    (*(owned_as_mut(&mut img)).unwrap()).width = npoints;
    (*(owned_as_mut(&mut img)).unwrap()).height = 1 as std::os::raw::c_int;
    (*(owned_as_mut(&mut img)).unwrap()).nbands = nbands;
    let mut nbytes = (::std::mem::size_of::<std::os::raw::c_float>() as std::os::raw::c_ulong)
        .wrapping_mul(npoints as std::os::raw::c_ulong)
        .wrapping_mul(nbands as std::os::raw::c_ulong) as std::os::raw::c_int;
    let ref mut fresh0 = (*(owned_as_mut(&mut img)).unwrap()).data;
    *fresh0 = malloc(nbytes as std::os::raw::c_ulong) as *mut std::os::raw::c_float;
    memcpy(
        (*(owned_as_mut(&mut img)).unwrap()).data as *mut std::os::raw::c_void,
        xy as *const std::os::raw::c_void,
        nbytes as std::os::raw::c_ulong,
    );
    return img;
}
#[no_mangle]
pub unsafe extern "C" fn heman_points_destroy(mut img: * mut crate::src::src::color::heman_image_s) {
    free((*img).data as *mut std::os::raw::c_void);
    free(img as *mut std::os::raw::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn heman_points_from_grid(
    mut width: std::os::raw::c_float,
    mut height: std::os::raw::c_float,
    mut cellsize: std::os::raw::c_float,
    mut jitter: std::os::raw::c_float,
) -> * mut crate::src::src::color::heman_image_s {
    let mut cols = (width / cellsize) as std::os::raw::c_int;
    let mut rows = (height / cellsize) as std::os::raw::c_int;
    let mut ncells = cols * rows;
    let mut result = heman_image_create(ncells, 1 as std::os::raw::c_int, 2 as std::os::raw::c_int);
    let mut rscale = (2.0f64 * jitter as std::os::raw::c_double
        / 2147483647 as std::os::raw::c_int as std::os::raw::c_float as std::os::raw::c_double) as std::os::raw::c_float;
    let mut j: i32 = 0;
    j = 0 as std::os::raw::c_int;
    while j < rows {
        let mut dst = ((*result).data).offset((j * cols * 2 as std::os::raw::c_int) as isize);
        let mut y = (cellsize as std::os::raw::c_double * 0.5f64
            + (cellsize * j as std::os::raw::c_float) as std::os::raw::c_double) as std::os::raw::c_float;
        let mut x = (cellsize as std::os::raw::c_double * 0.5f64) as std::os::raw::c_float;
        let mut i = 0 as std::os::raw::c_int;
        while i < cols {
            let mut rx = rand() as std::os::raw::c_float * rscale - jitter;
            let mut ry = rand() as std::os::raw::c_float * rscale - jitter;
            let mut fresh1 = dst;
            dst = dst.offset(1);
            *fresh1 = x + rx;
            let mut fresh2 = dst;
            dst = dst.offset(1);
            *fresh2 = y + ry;
            x += cellsize;
            i += 1;
        }
        j += 1;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn sample_annulus<'a1>(
    mut radius: std::os::raw::c_float,
    mut center: crate::src::kazmath::aabb2::kmVec2,
    mut seedptr: Option<&'a1 mut std::os::raw::c_uint>,
) -> crate::src::kazmath::aabb2::kmVec2 {
    let mut seed = *(borrow_mut(&mut seedptr)).unwrap();
    let mut r = kmVec2 { x: 0., y: 0. };
    let mut rscale = 1.0f32
        / (2147483647 as std::os::raw::c_int as std::os::raw::c_uint)
            .wrapping_mul(2 as std::os::raw::c_uint)
            .wrapping_add(1 as std::os::raw::c_uint) as std::os::raw::c_float;
    loop {
        let mut fresh3 = seed;
        seed = seed.wrapping_add(1);
        r
            .x = 4 as std::os::raw::c_int as std::os::raw::c_float * rscale
            * randhash(fresh3) as std::os::raw::c_float - 2 as std::os::raw::c_int as std::os::raw::c_float;
        let mut fresh4 = seed;
        seed = seed.wrapping_add(1);
        r
            .y = 4 as std::os::raw::c_int as std::os::raw::c_float * rscale
            * randhash(fresh4) as std::os::raw::c_float - 2 as std::os::raw::c_int as std::os::raw::c_float;
        let mut r2 = kmVec2LengthSq(Some(&mut r));
        if r2 > 1 as std::os::raw::c_int as std::os::raw::c_float
            && r2 <= 4 as std::os::raw::c_int as std::os::raw::c_float
        {
            break;
        }
    }
    *(borrow_mut(&mut seedptr)).unwrap() = seed;
    kmVec2Scale(&mut r, &mut r, radius);
    kmVec2Add(&mut r, Some(&mut r), Some(&mut center));
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn heman_points_from_poisson(
    mut width: std::os::raw::c_float,
    mut height: std::os::raw::c_float,
    mut radius: std::os::raw::c_float,
) -> * mut crate::src::src::color::heman_image_s {
    let mut maxattempts = 30 as std::os::raw::c_int;
    let mut rscale = 1.0f32
        / (2147483647 as std::os::raw::c_int as std::os::raw::c_uint)
            .wrapping_mul(2 as std::os::raw::c_uint)
            .wrapping_add(1 as std::os::raw::c_uint) as std::os::raw::c_float;
    let mut seed = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    let mut rvec = kmVec2 { x: 0., y: 0. };
    rvec.y = radius;
    rvec.x = rvec.y;
    let mut r2 = radius * radius;
    let mut cellsize = radius / sqrtf(2 as std::os::raw::c_int as std::os::raw::c_float);
    let mut invcell = 1.0f32 / cellsize;
    let mut ncols = ceil((width * invcell) as std::os::raw::c_double) as std::os::raw::c_int;
    let mut nrows = ceil((height * invcell) as std::os::raw::c_double) as std::os::raw::c_int;
    let mut maxcol = ncols - 1 as std::os::raw::c_int;
    let mut maxrow = nrows - 1 as std::os::raw::c_int;
    let mut ncells = ncols * nrows;
    let mut grid = malloc(
        (ncells as std::os::raw::c_ulong)
            .wrapping_mul(::std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong),
    ) as *mut std::os::raw::c_int;
    let mut i = 0 as std::os::raw::c_int;
    while i < ncells {
        *grid.offset(i as isize) = -(1 as std::os::raw::c_int);
        i += 1;
    }
    let mut actives = malloc(
        (ncells as std::os::raw::c_ulong)
            .wrapping_mul(::std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong),
    ) as *mut std::os::raw::c_int;
    let mut nactives = 0 as std::os::raw::c_int;
    let mut result = heman_image_create(ncells, 1 as std::os::raw::c_int, 2 as std::os::raw::c_int);
    let mut samples = (*result).data as *mut kmVec2;
    let mut nsamples = 0 as std::os::raw::c_int;
    let mut pt = kmVec2 { x: 0., y: 0. };
    let mut fresh5 = seed;
    seed = seed.wrapping_add(1);
    pt.x = width * randhash(fresh5) as std::os::raw::c_float * rscale;
    let mut fresh6 = seed;
    seed = seed.wrapping_add(1);
    pt.y = height * randhash(fresh6) as std::os::raw::c_float * rscale;
    let mut fresh7 = nactives;
    nactives = nactives + 1;
    let ref mut fresh8 = *actives.offset(fresh7 as isize);
    *fresh8 = nsamples;
    *grid
        .offset(
            ((pt.x * invcell) as std::os::raw::c_int + ncols * (pt.y * invcell) as std::os::raw::c_int)
                as isize,
        ) = *fresh8;
    let mut fresh9 = nsamples;
    nsamples = nsamples + 1;
    *samples.offset(fresh9 as isize) = pt;
    while nsamples < ncells {
        let mut fresh10 = seed;
        seed = seed.wrapping_add(1);
        let mut aindex = (if randhashf(
            fresh10,
            0 as std::os::raw::c_int as std::os::raw::c_float,
            nactives as std::os::raw::c_float,
        ) > (nactives - 1 as std::os::raw::c_int) as std::os::raw::c_float
        {
            (nactives - 1 as std::os::raw::c_int) as std::os::raw::c_float
        } else {
            let mut fresh11 = seed;
            seed = seed.wrapping_add(1);
            randhashf(
                fresh11,
                0 as std::os::raw::c_int as std::os::raw::c_float,
                nactives as std::os::raw::c_float,
            )
        }) as std::os::raw::c_int;
        let mut sindex = *actives.offset(aindex as isize);
        let mut found = 0 as std::os::raw::c_int;
        let mut j = kmVec2 { x: 0., y: 0. };
        let mut minj = kmVec2 { x: 0., y: 0. };
        let mut maxj = kmVec2 { x: 0., y: 0. };
        let mut delta = kmVec2 { x: 0., y: 0. };
        let mut attempt: i32 = 0;
        attempt = 0 as std::os::raw::c_int;
        while attempt < maxattempts && found == 0 {
            pt = sample_annulus(radius, *samples.offset(sindex as isize), Some(&mut seed));
            if !(pt.x < 0 as std::os::raw::c_int as std::os::raw::c_float || pt.x >= width
                || pt.y < 0 as std::os::raw::c_int as std::os::raw::c_float || pt.y >= height)
            {
                maxj = pt;
                minj = maxj;
                kmVec2Add(&mut maxj, Some(&mut maxj), Some(&mut rvec));
                kmVec2Subtract(&mut minj, Some(&mut minj), Some(&mut rvec));
                kmVec2Scale(&mut minj, &mut minj, invcell);
                kmVec2Scale(&mut maxj, &mut maxj, invcell);
                minj
                    .x = (if 0 as std::os::raw::c_int
                    > (if maxcol > minj.x as std::os::raw::c_int {
                        minj.x as std::os::raw::c_int
                    } else {
                        maxcol
                    })
                {
                    0 as std::os::raw::c_int
                } else if maxcol > minj.x as std::os::raw::c_int {
                    minj.x as std::os::raw::c_int
                } else {
                    maxcol
                }) as std::os::raw::c_float;
                maxj
                    .x = (if 0 as std::os::raw::c_int
                    > (if maxcol > maxj.x as std::os::raw::c_int {
                        maxj.x as std::os::raw::c_int
                    } else {
                        maxcol
                    })
                {
                    0 as std::os::raw::c_int
                } else if maxcol > maxj.x as std::os::raw::c_int {
                    maxj.x as std::os::raw::c_int
                } else {
                    maxcol
                }) as std::os::raw::c_float;
                minj
                    .y = (if 0 as std::os::raw::c_int
                    > (if maxrow > minj.y as std::os::raw::c_int {
                        minj.y as std::os::raw::c_int
                    } else {
                        maxrow
                    })
                {
                    0 as std::os::raw::c_int
                } else if maxrow > minj.y as std::os::raw::c_int {
                    minj.y as std::os::raw::c_int
                } else {
                    maxrow
                }) as std::os::raw::c_float;
                maxj
                    .y = (if 0 as std::os::raw::c_int
                    > (if maxrow > maxj.y as std::os::raw::c_int {
                        maxj.y as std::os::raw::c_int
                    } else {
                        maxrow
                    })
                {
                    0 as std::os::raw::c_int
                } else if maxrow > maxj.y as std::os::raw::c_int {
                    maxj.y as std::os::raw::c_int
                } else {
                    maxrow
                }) as std::os::raw::c_float;
                let mut reject = 0 as std::os::raw::c_int;
                j.y = minj.y;
                while j.y <= maxj.y && reject == 0 {
                    j.x = minj.x;
                    while j.x <= maxj.x && reject == 0 {
                        let mut entry = *grid
                            .offset(
                                (j.y as std::os::raw::c_int * ncols + j.x as std::os::raw::c_int) as isize,
                            );
                        if entry > -(1 as std::os::raw::c_int) && entry != sindex {
                            kmVec2Subtract(
                                &mut delta,
                                Some(&mut *samples.offset(entry as isize)),
                                Some(&mut pt),
                            );
                            if kmVec2LengthSq(Some(&mut delta)) < r2 {
                                reject = 1 as std::os::raw::c_int;
                            }
                        }
                        j.x += 1.;
                    }
                    j.y += 1.;
                }
                if !(reject != 0) {
                    found = 1 as std::os::raw::c_int;
                }
            }
            attempt += 1;
        }
        if found != 0 {
            let mut fresh12 = nactives;
            nactives = nactives + 1;
            let ref mut fresh13 = *actives.offset(fresh12 as isize);
            *fresh13 = nsamples;
            *grid
                .offset(
                    ((pt.x * invcell) as std::os::raw::c_int
                        + ncols * (pt.y * invcell) as std::os::raw::c_int) as isize,
                ) = *fresh13;
            let mut fresh14 = nsamples;
            nsamples = nsamples + 1;
            *samples.offset(fresh14 as isize) = pt;
        } else {
            nactives -= 1;
            if nactives <= 0 as std::os::raw::c_int {
                break;
            }
            *actives.offset(aindex as isize) = *actives.offset(nactives as isize);
        }
    }
    (*result).width = nsamples;
    free(grid as *mut std::os::raw::c_void);
    free(actives as *mut std::os::raw::c_void);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn heman_points_from_density(
    mut density: * mut crate::src::src::color::heman_image_s,
    mut minradius: std::os::raw::c_float,
    mut maxradius: std::os::raw::c_float,
) -> * mut crate::src::src::color::heman_image_s {
    if (*density).nbands == 1 as std::os::raw::c_int {} else {
        __assert_fail(
            b"density->nbands == 1\0" as *const u8 as *const std::os::raw::c_char,
            b"../src/points.c\0" as *const u8 as *const std::os::raw::c_char,
            215 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 69], &'_ [i8; 69]>(b"heman_points *heman_points_from_density(heman_image *, float, float)\0"))
                .as_ptr(),
        );
    }
    let mut width = 1 as std::os::raw::c_int as std::os::raw::c_float;
    let mut height = 1 as std::os::raw::c_int as std::os::raw::c_float;
    let mut maxattempts = 30 as std::os::raw::c_int;
    let mut rscale = 1.0f32
        / (2147483647 as std::os::raw::c_int as std::os::raw::c_uint)
            .wrapping_mul(2 as std::os::raw::c_uint)
            .wrapping_add(1 as std::os::raw::c_uint) as std::os::raw::c_float;
    let mut seed = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    let mut rvec = kmVec2 { x: 0., y: 0. };
    rvec.y = maxradius;
    rvec.x = rvec.y;
    let mut gindex: i32 = 0;
    let mut cellsize = maxradius / sqrtf(2 as std::os::raw::c_int as std::os::raw::c_float);
    let mut invcell = 1.0f32 / cellsize;
    let mut ncols = ceil((width * invcell) as std::os::raw::c_double) as std::os::raw::c_int;
    let mut nrows = ceil((height * invcell) as std::os::raw::c_double) as std::os::raw::c_int;
    let mut maxcol = ncols - 1 as std::os::raw::c_int;
    let mut maxrow = nrows - 1 as std::os::raw::c_int;
    let mut ncells = ncols * nrows;
    let mut ntexels = (cellsize * (*density).width as std::os::raw::c_float) as std::os::raw::c_int;
    let mut gcapacity = ntexels * ntexels;
    let mut grid = malloc(
        (ncells as std::os::raw::c_ulong)
            .wrapping_mul(::std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong)
            .wrapping_mul(gcapacity as std::os::raw::c_ulong),
    ) as *mut std::os::raw::c_int;
    let mut ngrid = malloc(
        (ncells as std::os::raw::c_ulong)
            .wrapping_mul(::std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong),
    ) as *mut std::os::raw::c_int;
    let mut i = 0 as std::os::raw::c_int;
    while i < ncells {
        *ngrid.offset(i as isize) = 0 as std::os::raw::c_int;
        i += 1;
    }
    let mut actives = malloc(
        (ncells as std::os::raw::c_ulong)
            .wrapping_mul(::std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong),
    ) as *mut std::os::raw::c_int;
    let mut nactives = 0 as std::os::raw::c_int;
    let mut maxsamples = ncells * gcapacity;
    let mut result = heman_image_create(maxsamples, 1 as std::os::raw::c_int, 2 as std::os::raw::c_int);
    let mut samples = (*result).data as *mut kmVec2;
    let mut nsamples = 0 as std::os::raw::c_int;
    let mut pt = kmVec2 { x: 0., y: 0. };
    let mut fresh15 = seed;
    seed = seed.wrapping_add(1);
    pt.x = width * randhash(fresh15) as std::os::raw::c_float * rscale;
    let mut fresh16 = seed;
    seed = seed.wrapping_add(1);
    pt.y = height * randhash(fresh16) as std::os::raw::c_float * rscale;
    let mut fresh17 = nactives;
    nactives = nactives + 1;
    *actives.offset(fresh17 as isize) = nsamples;
    gindex = (pt.x * invcell) as std::os::raw::c_int + ncols * (pt.y * invcell) as std::os::raw::c_int;
    *grid
        .offset(
            (gcapacity * gindex + *ngrid.offset(gindex as isize)) as isize,
        ) = nsamples;
    let ref mut fresh18 = *ngrid.offset(gindex as isize);
    *fresh18 += 1;
    let mut fresh19 = nsamples;
    nsamples = nsamples + 1;
    *samples.offset(fresh19 as isize) = pt;
    while nsamples < maxsamples {
        let mut fresh20 = seed;
        seed = seed.wrapping_add(1);
        let mut aindex = (if randhashf(
            fresh20,
            0 as std::os::raw::c_int as std::os::raw::c_float,
            nactives as std::os::raw::c_float,
        ) > (nactives - 1 as std::os::raw::c_int) as std::os::raw::c_float
        {
            (nactives - 1 as std::os::raw::c_int) as std::os::raw::c_float
        } else {
            let mut fresh21 = seed;
            seed = seed.wrapping_add(1);
            randhashf(
                fresh21,
                0 as std::os::raw::c_int as std::os::raw::c_float,
                nactives as std::os::raw::c_float,
            )
        }) as std::os::raw::c_int;
        let mut sindex = *actives.offset(aindex as isize);
        let mut found = 0 as std::os::raw::c_int;
        let mut j = kmVec2 { x: 0., y: 0. };
        let mut minj = kmVec2 { x: 0., y: 0. };
        let mut maxj = kmVec2 { x: 0., y: 0. };
        let mut delta = kmVec2 { x: 0., y: 0. };
        let mut attempt: i32 = 0;
        attempt = 0 as std::os::raw::c_int;
        while attempt < maxattempts && found == 0 {
            pt = sample_annulus(maxradius, *samples.offset(sindex as isize), Some(&mut seed));
            if !(pt.x < 0 as std::os::raw::c_int as std::os::raw::c_float || pt.x >= width
                || pt.y < 0 as std::os::raw::c_int as std::os::raw::c_float || pt.y >= height)
            {
                maxj = pt;
                minj = maxj;
                kmVec2Add(&mut maxj, Some(&mut maxj), Some(&mut rvec));
                kmVec2Subtract(&mut minj, Some(&mut minj), Some(&mut rvec));
                kmVec2Scale(&mut minj, &mut minj, invcell);
                kmVec2Scale(&mut maxj, &mut maxj, invcell);
                minj
                    .x = (if 0 as std::os::raw::c_int
                    > (if maxcol > minj.x as std::os::raw::c_int {
                        minj.x as std::os::raw::c_int
                    } else {
                        maxcol
                    })
                {
                    0 as std::os::raw::c_int
                } else if maxcol > minj.x as std::os::raw::c_int {
                    minj.x as std::os::raw::c_int
                } else {
                    maxcol
                }) as std::os::raw::c_float;
                maxj
                    .x = (if 0 as std::os::raw::c_int
                    > (if maxcol > maxj.x as std::os::raw::c_int {
                        maxj.x as std::os::raw::c_int
                    } else {
                        maxcol
                    })
                {
                    0 as std::os::raw::c_int
                } else if maxcol > maxj.x as std::os::raw::c_int {
                    maxj.x as std::os::raw::c_int
                } else {
                    maxcol
                }) as std::os::raw::c_float;
                minj
                    .y = (if 0 as std::os::raw::c_int
                    > (if maxrow > minj.y as std::os::raw::c_int {
                        minj.y as std::os::raw::c_int
                    } else {
                        maxrow
                    })
                {
                    0 as std::os::raw::c_int
                } else if maxrow > minj.y as std::os::raw::c_int {
                    minj.y as std::os::raw::c_int
                } else {
                    maxrow
                }) as std::os::raw::c_float;
                maxj
                    .y = (if 0 as std::os::raw::c_int
                    > (if maxrow > maxj.y as std::os::raw::c_int {
                        maxj.y as std::os::raw::c_int
                    } else {
                        maxrow
                    })
                {
                    0 as std::os::raw::c_int
                } else if maxrow > maxj.y as std::os::raw::c_int {
                    maxj.y as std::os::raw::c_int
                } else {
                    maxrow
                }) as std::os::raw::c_float;
                let mut reject = 0 as std::os::raw::c_int;
                let mut densityval: f32 = 0.;
                heman_image_sample(density, pt.x, pt.y, &mut densityval);
                densityval = sqrt(densityval as std::os::raw::c_double) as std::os::raw::c_float;
                let mut mindist = maxradius - densityval * (maxradius - minradius);
                let mut r2 = mindist * mindist;
                j.y = minj.y;
                while j.y <= maxj.y && reject == 0 {
                    j.x = minj.x;
                    while j.x <= maxj.x && reject == 0 {
                        let mut g = (j.y as std::os::raw::c_int * ncols + j.x as std::os::raw::c_int)
                            * gcapacity;
                        while g
                            < (j.y as std::os::raw::c_int * ncols + j.x as std::os::raw::c_int)
                                * gcapacity
                                + *ngrid
                                    .offset(
                                        (j.y as std::os::raw::c_int * ncols + j.x as std::os::raw::c_int) as isize,
                                    )
                        {
                            let mut entry = *grid.offset(g as isize);
                            if entry != sindex {
                                kmVec2Subtract(
                                    &mut delta,
                                    Some(&mut *samples.offset(entry as isize)),
                                    Some(&mut pt),
                                );
                                if kmVec2LengthSq(Some(&mut delta)) < r2 {
                                    reject = 1 as std::os::raw::c_int;
                                }
                            }
                            g += 1;
                        }
                        j.x += 1.;
                    }
                    j.y += 1.;
                }
                if !(reject != 0) {
                    found = 1 as std::os::raw::c_int;
                }
            }
            attempt += 1;
        }
        if found != 0
            && *ngrid
                .offset(
                    ((pt.x * invcell) as std::os::raw::c_int
                        + ncols * (pt.y * invcell) as std::os::raw::c_int) as isize,
                ) >= gcapacity
        {
            found = 0 as std::os::raw::c_int;
        }
        if found != 0 {
            let mut fresh22 = nactives;
            nactives = nactives + 1;
            *actives.offset(fresh22 as isize) = nsamples;
            gindex = (pt.x * invcell) as std::os::raw::c_int
                + ncols * (pt.y * invcell) as std::os::raw::c_int;
            *grid
                .offset(
                    (gcapacity * gindex + *ngrid.offset(gindex as isize)) as isize,
                ) = nsamples;
            let ref mut fresh23 = *ngrid.offset(gindex as isize);
            *fresh23 += 1;
            let mut fresh24 = nsamples;
            nsamples = nsamples + 1;
            *samples.offset(fresh24 as isize) = pt;
        } else {
            nactives -= 1;
            if nactives <= 0 as std::os::raw::c_int {
                break;
            }
            *actives.offset(aindex as isize) = *actives.offset(nactives as isize);
        }
    }
    (*result).width = nsamples;
    free(grid as *mut std::os::raw::c_void);
    free(ngrid as *mut std::os::raw::c_void);
    free(actives as *mut std::os::raw::c_void);
    return result;
}
use crate::laertes_rt::*;
