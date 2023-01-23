use ::libc;
extern "C" {
    fn heman_image_create(
        width: libc::c_int,
        height: libc::c_int,
        nbands: libc::c_int,
    ) -> *mut heman_image;
    fn heman_image_sample(
        _: *mut heman_image,
        u: libc::c_float,
        v: libc::c_float,
        result: *mut libc::c_float,
    );
    fn rand() -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn sqrtf(_: libc::c_float) -> libc::c_float;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn ceil(_: libc::c_double) -> libc::c_double;
    fn kmVec2Scale(
        pOut: *mut kmVec2,
        pIn: *const kmVec2,
        s: libc::c_float,
    ) -> *mut kmVec2;
    fn kmVec2Add(
        pOut: *mut kmVec2,
        pV1: *const kmVec2,
        pV2: *const kmVec2,
    ) -> *mut kmVec2;
    fn kmVec2Subtract(
        pOut: *mut kmVec2,
        pV1: *const kmVec2,
        pV2: *const kmVec2,
    ) -> *mut kmVec2;
    fn kmVec2LengthSq(pIn: *const kmVec2) -> libc::c_float;
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
pub type heman_points = heman_image_s;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct kmVec2 {
    pub x: libc::c_float,
    pub y: libc::c_float,
}
#[no_mangle]
pub unsafe extern "C" fn randhash(mut seed: libc::c_uint) -> libc::c_uint {
    let mut i = (seed ^ 12345391 as libc::c_uint)
        .wrapping_mul(2654435769 as libc::c_uint);
    i ^= i << 6 as libc::c_int ^ i >> 26 as libc::c_int;
    i = i.wrapping_mul(2654435769 as libc::c_uint);
    i = i.wrapping_add(i << 5 as libc::c_int ^ i >> 12 as libc::c_int);
    return i;
}
#[no_mangle]
pub unsafe extern "C" fn randhashf(
    mut seed: libc::c_uint,
    mut a: libc::c_float,
    mut b: libc::c_float,
) -> libc::c_float {
    return (b - a) * randhash(seed) as libc::c_float
        / (2147483647 as libc::c_int as libc::c_uint)
            .wrapping_mul(2 as libc::c_uint)
            .wrapping_add(1 as libc::c_uint) as libc::c_float + a;
}
#[no_mangle]
pub unsafe extern "C" fn heman_points_create(
    mut xy: *mut libc::c_float,
    mut npoints: libc::c_int,
    mut nbands: libc::c_int,
) -> *mut heman_image {
    let mut img = malloc(::std::mem::size_of::<heman_image>() as libc::c_ulong)
        as *mut heman_points;
    (*img).width = npoints;
    (*img).height = 1 as libc::c_int;
    (*img).nbands = nbands;
    let mut nbytes = (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
        .wrapping_mul(npoints as libc::c_ulong)
        .wrapping_mul(nbands as libc::c_ulong) as libc::c_int;
    let ref mut fresh0 = (*img).data;
    *fresh0 = malloc(nbytes as libc::c_ulong) as *mut libc::c_float;
    memcpy(
        (*img).data as *mut libc::c_void,
        xy as *const libc::c_void,
        nbytes as libc::c_ulong,
    );
    return img;
}
#[no_mangle]
pub unsafe extern "C" fn heman_points_destroy(mut img: *mut heman_points) {
    free((*img).data as *mut libc::c_void);
    free(img as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn heman_points_from_grid(
    mut width: libc::c_float,
    mut height: libc::c_float,
    mut cellsize: libc::c_float,
    mut jitter: libc::c_float,
) -> *mut heman_points {
    let mut cols = (width / cellsize) as libc::c_int;
    let mut rows = (height / cellsize) as libc::c_int;
    let mut ncells = cols * rows;
    let mut result = heman_image_create(ncells, 1 as libc::c_int, 2 as libc::c_int);
    let mut rscale = (2.0f64 * jitter as libc::c_double
        / 2147483647 as libc::c_int as libc::c_float as libc::c_double) as libc::c_float;
    let mut j: libc::c_int = 0;
    j = 0 as libc::c_int;
    while j < rows {
        let mut dst = ((*result).data).offset((j * cols * 2 as libc::c_int) as isize);
        let mut y = (cellsize as libc::c_double * 0.5f64
            + (cellsize * j as libc::c_float) as libc::c_double) as libc::c_float;
        let mut x = (cellsize as libc::c_double * 0.5f64) as libc::c_float;
        let mut i = 0 as libc::c_int;
        while i < cols {
            let mut rx = rand() as libc::c_float * rscale - jitter;
            let mut ry = rand() as libc::c_float * rscale - jitter;
            let fresh1 = dst;
            dst = dst.offset(1);
            *fresh1 = x + rx;
            let fresh2 = dst;
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
pub unsafe extern "C" fn sample_annulus(
    mut radius: libc::c_float,
    mut center: kmVec2,
    mut seedptr: *mut libc::c_uint,
) -> kmVec2 {
    let mut seed = *seedptr;
    let mut r = kmVec2 { x: 0., y: 0. };
    let mut rscale = 1.0f32
        / (2147483647 as libc::c_int as libc::c_uint)
            .wrapping_mul(2 as libc::c_uint)
            .wrapping_add(1 as libc::c_uint) as libc::c_float;
    loop {
        let fresh3 = seed;
        seed = seed.wrapping_add(1);
        r
            .x = 4 as libc::c_int as libc::c_float * rscale
            * randhash(fresh3) as libc::c_float - 2 as libc::c_int as libc::c_float;
        let fresh4 = seed;
        seed = seed.wrapping_add(1);
        r
            .y = 4 as libc::c_int as libc::c_float * rscale
            * randhash(fresh4) as libc::c_float - 2 as libc::c_int as libc::c_float;
        let mut r2 = kmVec2LengthSq(&mut r);
        if r2 > 1 as libc::c_int as libc::c_float
            && r2 <= 4 as libc::c_int as libc::c_float
        {
            break;
        }
    }
    *seedptr = seed;
    kmVec2Scale(&mut r, &mut r, radius);
    kmVec2Add(&mut r, &mut r, &mut center);
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn heman_points_from_poisson(
    mut width: libc::c_float,
    mut height: libc::c_float,
    mut radius: libc::c_float,
) -> *mut heman_points {
    let mut maxattempts = 30 as libc::c_int;
    let mut rscale = 1.0f32
        / (2147483647 as libc::c_int as libc::c_uint)
            .wrapping_mul(2 as libc::c_uint)
            .wrapping_add(1 as libc::c_uint) as libc::c_float;
    let mut seed = 0 as libc::c_int as libc::c_uint;
    let mut rvec = kmVec2 { x: 0., y: 0. };
    rvec.y = radius;
    rvec.x = rvec.y;
    let mut r2 = radius * radius;
    let mut cellsize = radius / sqrtf(2 as libc::c_int as libc::c_float);
    let mut invcell = 1.0f32 / cellsize;
    let mut ncols = ceil((width * invcell) as libc::c_double) as libc::c_int;
    let mut nrows = ceil((height * invcell) as libc::c_double) as libc::c_int;
    let mut maxcol = ncols - 1 as libc::c_int;
    let mut maxrow = nrows - 1 as libc::c_int;
    let mut ncells = ncols * nrows;
    let mut grid = malloc(
        (ncells as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    let mut i = 0 as libc::c_int;
    while i < ncells {
        *grid.offset(i as isize) = -(1 as libc::c_int);
        i += 1;
    }
    let mut actives = malloc(
        (ncells as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    let mut nactives = 0 as libc::c_int;
    let mut result = heman_image_create(ncells, 1 as libc::c_int, 2 as libc::c_int);
    let mut samples = (*result).data as *mut kmVec2;
    let mut nsamples = 0 as libc::c_int;
    let mut pt = kmVec2 { x: 0., y: 0. };
    let fresh5 = seed;
    seed = seed.wrapping_add(1);
    pt.x = width * randhash(fresh5) as libc::c_float * rscale;
    let fresh6 = seed;
    seed = seed.wrapping_add(1);
    pt.y = height * randhash(fresh6) as libc::c_float * rscale;
    let fresh7 = nactives;
    nactives = nactives + 1;
    let ref mut fresh8 = *actives.offset(fresh7 as isize);
    *fresh8 = nsamples;
    *grid
        .offset(
            ((pt.x * invcell) as libc::c_int + ncols * (pt.y * invcell) as libc::c_int)
                as isize,
        ) = *fresh8;
    let fresh9 = nsamples;
    nsamples = nsamples + 1;
    *samples.offset(fresh9 as isize) = pt;
    while nsamples < ncells {
        let fresh10 = seed;
        seed = seed.wrapping_add(1);
        let mut aindex = (if randhashf(
            fresh10,
            0 as libc::c_int as libc::c_float,
            nactives as libc::c_float,
        ) > (nactives - 1 as libc::c_int) as libc::c_float
        {
            (nactives - 1 as libc::c_int) as libc::c_float
        } else {
            let fresh11 = seed;
            seed = seed.wrapping_add(1);
            randhashf(
                fresh11,
                0 as libc::c_int as libc::c_float,
                nactives as libc::c_float,
            )
        }) as libc::c_int;
        let mut sindex = *actives.offset(aindex as isize);
        let mut found = 0 as libc::c_int;
        let mut j = kmVec2 { x: 0., y: 0. };
        let mut minj = kmVec2 { x: 0., y: 0. };
        let mut maxj = kmVec2 { x: 0., y: 0. };
        let mut delta = kmVec2 { x: 0., y: 0. };
        let mut attempt: libc::c_int = 0;
        attempt = 0 as libc::c_int;
        while attempt < maxattempts && found == 0 {
            pt = sample_annulus(radius, *samples.offset(sindex as isize), &mut seed);
            if !(pt.x < 0 as libc::c_int as libc::c_float || pt.x >= width
                || pt.y < 0 as libc::c_int as libc::c_float || pt.y >= height)
            {
                maxj = pt;
                minj = maxj;
                kmVec2Add(&mut maxj, &mut maxj, &mut rvec);
                kmVec2Subtract(&mut minj, &mut minj, &mut rvec);
                kmVec2Scale(&mut minj, &mut minj, invcell);
                kmVec2Scale(&mut maxj, &mut maxj, invcell);
                minj
                    .x = (if 0 as libc::c_int
                    > (if maxcol > minj.x as libc::c_int {
                        minj.x as libc::c_int
                    } else {
                        maxcol
                    })
                {
                    0 as libc::c_int
                } else if maxcol > minj.x as libc::c_int {
                    minj.x as libc::c_int
                } else {
                    maxcol
                }) as libc::c_float;
                maxj
                    .x = (if 0 as libc::c_int
                    > (if maxcol > maxj.x as libc::c_int {
                        maxj.x as libc::c_int
                    } else {
                        maxcol
                    })
                {
                    0 as libc::c_int
                } else if maxcol > maxj.x as libc::c_int {
                    maxj.x as libc::c_int
                } else {
                    maxcol
                }) as libc::c_float;
                minj
                    .y = (if 0 as libc::c_int
                    > (if maxrow > minj.y as libc::c_int {
                        minj.y as libc::c_int
                    } else {
                        maxrow
                    })
                {
                    0 as libc::c_int
                } else if maxrow > minj.y as libc::c_int {
                    minj.y as libc::c_int
                } else {
                    maxrow
                }) as libc::c_float;
                maxj
                    .y = (if 0 as libc::c_int
                    > (if maxrow > maxj.y as libc::c_int {
                        maxj.y as libc::c_int
                    } else {
                        maxrow
                    })
                {
                    0 as libc::c_int
                } else if maxrow > maxj.y as libc::c_int {
                    maxj.y as libc::c_int
                } else {
                    maxrow
                }) as libc::c_float;
                let mut reject = 0 as libc::c_int;
                j.y = minj.y;
                while j.y <= maxj.y && reject == 0 {
                    j.x = minj.x;
                    while j.x <= maxj.x && reject == 0 {
                        let mut entry = *grid
                            .offset(
                                (j.y as libc::c_int * ncols + j.x as libc::c_int) as isize,
                            );
                        if entry > -(1 as libc::c_int) && entry != sindex {
                            kmVec2Subtract(
                                &mut delta,
                                &mut *samples.offset(entry as isize),
                                &mut pt,
                            );
                            if kmVec2LengthSq(&mut delta) < r2 {
                                reject = 1 as libc::c_int;
                            }
                        }
                        j.x += 1.;
                    }
                    j.y += 1.;
                }
                if !(reject != 0) {
                    found = 1 as libc::c_int;
                }
            }
            attempt += 1;
        }
        if found != 0 {
            let fresh12 = nactives;
            nactives = nactives + 1;
            let ref mut fresh13 = *actives.offset(fresh12 as isize);
            *fresh13 = nsamples;
            *grid
                .offset(
                    ((pt.x * invcell) as libc::c_int
                        + ncols * (pt.y * invcell) as libc::c_int) as isize,
                ) = *fresh13;
            let fresh14 = nsamples;
            nsamples = nsamples + 1;
            *samples.offset(fresh14 as isize) = pt;
        } else {
            nactives -= 1;
            if nactives <= 0 as libc::c_int {
                break;
            }
            *actives.offset(aindex as isize) = *actives.offset(nactives as isize);
        }
    }
    (*result).width = nsamples;
    free(grid as *mut libc::c_void);
    free(actives as *mut libc::c_void);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn heman_points_from_density(
    mut density: *mut heman_image,
    mut minradius: libc::c_float,
    mut maxradius: libc::c_float,
) -> *mut heman_points {
    if (*density).nbands == 1 as libc::c_int {} else {
        __assert_fail(
            b"density->nbands == 1\0" as *const u8 as *const libc::c_char,
            b"../src/points.c\0" as *const u8 as *const libc::c_char,
            215 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 69],
                &[libc::c_char; 69],
            >(b"heman_points *heman_points_from_density(heman_image *, float, float)\0"))
                .as_ptr(),
        );
    }
    let mut width = 1 as libc::c_int as libc::c_float;
    let mut height = 1 as libc::c_int as libc::c_float;
    let mut maxattempts = 30 as libc::c_int;
    let mut rscale = 1.0f32
        / (2147483647 as libc::c_int as libc::c_uint)
            .wrapping_mul(2 as libc::c_uint)
            .wrapping_add(1 as libc::c_uint) as libc::c_float;
    let mut seed = 0 as libc::c_int as libc::c_uint;
    let mut rvec = kmVec2 { x: 0., y: 0. };
    rvec.y = maxradius;
    rvec.x = rvec.y;
    let mut gindex: libc::c_int = 0;
    let mut cellsize = maxradius / sqrtf(2 as libc::c_int as libc::c_float);
    let mut invcell = 1.0f32 / cellsize;
    let mut ncols = ceil((width * invcell) as libc::c_double) as libc::c_int;
    let mut nrows = ceil((height * invcell) as libc::c_double) as libc::c_int;
    let mut maxcol = ncols - 1 as libc::c_int;
    let mut maxrow = nrows - 1 as libc::c_int;
    let mut ncells = ncols * nrows;
    let mut ntexels = (cellsize * (*density).width as libc::c_float) as libc::c_int;
    let mut gcapacity = ntexels * ntexels;
    let mut grid = malloc(
        (ncells as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(gcapacity as libc::c_ulong),
    ) as *mut libc::c_int;
    let mut ngrid = malloc(
        (ncells as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    let mut i = 0 as libc::c_int;
    while i < ncells {
        *ngrid.offset(i as isize) = 0 as libc::c_int;
        i += 1;
    }
    let mut actives = malloc(
        (ncells as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    let mut nactives = 0 as libc::c_int;
    let mut maxsamples = ncells * gcapacity;
    let mut result = heman_image_create(maxsamples, 1 as libc::c_int, 2 as libc::c_int);
    let mut samples = (*result).data as *mut kmVec2;
    let mut nsamples = 0 as libc::c_int;
    let mut pt = kmVec2 { x: 0., y: 0. };
    let fresh15 = seed;
    seed = seed.wrapping_add(1);
    pt.x = width * randhash(fresh15) as libc::c_float * rscale;
    let fresh16 = seed;
    seed = seed.wrapping_add(1);
    pt.y = height * randhash(fresh16) as libc::c_float * rscale;
    let fresh17 = nactives;
    nactives = nactives + 1;
    *actives.offset(fresh17 as isize) = nsamples;
    gindex = (pt.x * invcell) as libc::c_int + ncols * (pt.y * invcell) as libc::c_int;
    *grid
        .offset(
            (gcapacity * gindex + *ngrid.offset(gindex as isize)) as isize,
        ) = nsamples;
    let ref mut fresh18 = *ngrid.offset(gindex as isize);
    *fresh18 += 1;
    let fresh19 = nsamples;
    nsamples = nsamples + 1;
    *samples.offset(fresh19 as isize) = pt;
    while nsamples < maxsamples {
        let fresh20 = seed;
        seed = seed.wrapping_add(1);
        let mut aindex = (if randhashf(
            fresh20,
            0 as libc::c_int as libc::c_float,
            nactives as libc::c_float,
        ) > (nactives - 1 as libc::c_int) as libc::c_float
        {
            (nactives - 1 as libc::c_int) as libc::c_float
        } else {
            let fresh21 = seed;
            seed = seed.wrapping_add(1);
            randhashf(
                fresh21,
                0 as libc::c_int as libc::c_float,
                nactives as libc::c_float,
            )
        }) as libc::c_int;
        let mut sindex = *actives.offset(aindex as isize);
        let mut found = 0 as libc::c_int;
        let mut j = kmVec2 { x: 0., y: 0. };
        let mut minj = kmVec2 { x: 0., y: 0. };
        let mut maxj = kmVec2 { x: 0., y: 0. };
        let mut delta = kmVec2 { x: 0., y: 0. };
        let mut attempt: libc::c_int = 0;
        attempt = 0 as libc::c_int;
        while attempt < maxattempts && found == 0 {
            pt = sample_annulus(maxradius, *samples.offset(sindex as isize), &mut seed);
            if !(pt.x < 0 as libc::c_int as libc::c_float || pt.x >= width
                || pt.y < 0 as libc::c_int as libc::c_float || pt.y >= height)
            {
                maxj = pt;
                minj = maxj;
                kmVec2Add(&mut maxj, &mut maxj, &mut rvec);
                kmVec2Subtract(&mut minj, &mut minj, &mut rvec);
                kmVec2Scale(&mut minj, &mut minj, invcell);
                kmVec2Scale(&mut maxj, &mut maxj, invcell);
                minj
                    .x = (if 0 as libc::c_int
                    > (if maxcol > minj.x as libc::c_int {
                        minj.x as libc::c_int
                    } else {
                        maxcol
                    })
                {
                    0 as libc::c_int
                } else if maxcol > minj.x as libc::c_int {
                    minj.x as libc::c_int
                } else {
                    maxcol
                }) as libc::c_float;
                maxj
                    .x = (if 0 as libc::c_int
                    > (if maxcol > maxj.x as libc::c_int {
                        maxj.x as libc::c_int
                    } else {
                        maxcol
                    })
                {
                    0 as libc::c_int
                } else if maxcol > maxj.x as libc::c_int {
                    maxj.x as libc::c_int
                } else {
                    maxcol
                }) as libc::c_float;
                minj
                    .y = (if 0 as libc::c_int
                    > (if maxrow > minj.y as libc::c_int {
                        minj.y as libc::c_int
                    } else {
                        maxrow
                    })
                {
                    0 as libc::c_int
                } else if maxrow > minj.y as libc::c_int {
                    minj.y as libc::c_int
                } else {
                    maxrow
                }) as libc::c_float;
                maxj
                    .y = (if 0 as libc::c_int
                    > (if maxrow > maxj.y as libc::c_int {
                        maxj.y as libc::c_int
                    } else {
                        maxrow
                    })
                {
                    0 as libc::c_int
                } else if maxrow > maxj.y as libc::c_int {
                    maxj.y as libc::c_int
                } else {
                    maxrow
                }) as libc::c_float;
                let mut reject = 0 as libc::c_int;
                let mut densityval: libc::c_float = 0.;
                heman_image_sample(density, pt.x, pt.y, &mut densityval);
                densityval = sqrt(densityval as libc::c_double) as libc::c_float;
                let mut mindist = maxradius - densityval * (maxradius - minradius);
                let mut r2 = mindist * mindist;
                j.y = minj.y;
                while j.y <= maxj.y && reject == 0 {
                    j.x = minj.x;
                    while j.x <= maxj.x && reject == 0 {
                        let mut g = (j.y as libc::c_int * ncols + j.x as libc::c_int)
                            * gcapacity;
                        while g
                            < (j.y as libc::c_int * ncols + j.x as libc::c_int)
                                * gcapacity
                                + *ngrid
                                    .offset(
                                        (j.y as libc::c_int * ncols + j.x as libc::c_int) as isize,
                                    )
                        {
                            let mut entry = *grid.offset(g as isize);
                            if entry != sindex {
                                kmVec2Subtract(
                                    &mut delta,
                                    &mut *samples.offset(entry as isize),
                                    &mut pt,
                                );
                                if kmVec2LengthSq(&mut delta) < r2 {
                                    reject = 1 as libc::c_int;
                                }
                            }
                            g += 1;
                        }
                        j.x += 1.;
                    }
                    j.y += 1.;
                }
                if !(reject != 0) {
                    found = 1 as libc::c_int;
                }
            }
            attempt += 1;
        }
        if found != 0
            && *ngrid
                .offset(
                    ((pt.x * invcell) as libc::c_int
                        + ncols * (pt.y * invcell) as libc::c_int) as isize,
                ) >= gcapacity
        {
            found = 0 as libc::c_int;
        }
        if found != 0 {
            let fresh22 = nactives;
            nactives = nactives + 1;
            *actives.offset(fresh22 as isize) = nsamples;
            gindex = (pt.x * invcell) as libc::c_int
                + ncols * (pt.y * invcell) as libc::c_int;
            *grid
                .offset(
                    (gcapacity * gindex + *ngrid.offset(gindex as isize)) as isize,
                ) = nsamples;
            let ref mut fresh23 = *ngrid.offset(gindex as isize);
            *fresh23 += 1;
            let fresh24 = nsamples;
            nsamples = nsamples + 1;
            *samples.offset(fresh24 as isize) = pt;
        } else {
            nactives -= 1;
            if nactives <= 0 as libc::c_int {
                break;
            }
            *actives.offset(aindex as isize) = *actives.offset(nactives as isize);
        }
    }
    (*result).width = nsamples;
    free(grid as *mut libc::c_void);
    free(ngrid as *mut libc::c_void);
    free(actives as *mut libc::c_void);
    return result;
}
