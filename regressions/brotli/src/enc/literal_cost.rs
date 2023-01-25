use ::libc;
extern "C" {
    fn log2(_: libc::c_double) -> libc::c_double;
    static kBrotliLog2Table: [libc::c_double; 256];
    
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type uint8_t = __uint8_t;
#[inline(always)]
unsafe extern "C" fn brotli_min_size_t(mut a: size_t, mut b: size_t) -> size_t {
    return if a < b { a } else { b };
}
#[inline(always)]
unsafe extern "C" fn FastLog2(mut v: size_t) -> libc::c_double {
    if v < 256 as libc::c_int as libc::c_ulong {
        return crate::src::enc::literal_cost::kBrotliLog2Table[v as usize];
    }
    return log2(v as libc::c_double);
}
static mut kMinUTF8Ratio: libc::c_double = 0.75f64;
unsafe extern "C" fn UTF8Position(
    mut last: size_t,
    mut c: size_t,
    mut clamp: size_t,
) -> size_t {
    if c < 128 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int as size_t
    } else if c >= 192 as libc::c_int as libc::c_ulong {
        return brotli_min_size_t(1 as libc::c_int as size_t, clamp)
    } else if last < 0xe0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int as size_t
    } else {
        return brotli_min_size_t(2 as libc::c_int as size_t, clamp)
    };
}
unsafe extern "C" fn DecideMultiByteStatsLevel(
    mut pos: size_t,
    mut len: size_t,
    mut mask: size_t,
    mut data: *const uint8_t,
) -> size_t {
    let mut counts: [size_t; 3] = [0 as libc::c_int as size_t, 0, 0];
    let mut max_utf8 = 1 as libc::c_int as size_t;
    let mut last_c = 0 as libc::c_int as size_t;
    let mut i: size_t = 0;
    i= 0 as libc::c_int as size_t;
    while i < len {
        let mut c = *data.offset((pos.wrapping_add(i) & mask) as isize) as size_t;
        counts[UTF8Position(last_c, c, 2 as libc::c_int as size_t)
            as usize]= counts[UTF8Position(last_c, c, 2 as libc::c_int as size_t)
            as usize]
            .wrapping_add(1);
        last_c= c;
        i= i.wrapping_add(1);
    }
    if counts[2 as libc::c_int as usize] < 500 as libc::c_int as libc::c_ulong {
        max_utf8= 1 as libc::c_int as size_t;
    }
    if counts[1 as libc::c_int as usize]
        .wrapping_add(counts[2 as libc::c_int as usize])
        < 25 as libc::c_int as libc::c_ulong
    {
        max_utf8= 0 as libc::c_int as size_t;
    }
    return max_utf8;
}
unsafe extern "C" fn EstimateBitCostsForLiteralsUTF8(
    mut pos: size_t,
    mut len: size_t,
    mut mask: size_t,
    mut data: *const uint8_t,
    mut cost: *mut libc::c_float,
) {
    let max_utf8 = DecideMultiByteStatsLevel(pos, len, mask, data);
    let mut histogram: [[size_t; 256]; 3] = [
        [
            0 as libc::c_int as size_t,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        [0; 256],
        [0; 256],
    ];
    let mut window_half = 495 as libc::c_int as size_t;
    let mut in_window = brotli_min_size_t(window_half, len);
    let mut in_window_utf8: [size_t; 3] = [0 as libc::c_int as size_t, 0, 0];
    let mut i: size_t = 0;
    let mut last_c = 0 as libc::c_int as size_t;
    let mut utf8_pos = 0 as libc::c_int as size_t;
    i= 0 as libc::c_int as size_t;
    while i < in_window {
        let mut c = *data.offset((pos.wrapping_add(i) & mask) as isize) as size_t;
        histogram[utf8_pos
            as usize][c
            as usize]= histogram[utf8_pos as usize][c as usize].wrapping_add(1);
        in_window_utf8[utf8_pos
            as usize]= in_window_utf8[utf8_pos as usize].wrapping_add(1);
        utf8_pos= UTF8Position(last_c, c, max_utf8);
        last_c= c;
        i= i.wrapping_add(1);
    }
    i= 0 as libc::c_int as size_t;
    while i < len {
        if i >= window_half {
            let mut c_0 = (if i
                < window_half.wrapping_add(1 as libc::c_int as libc::c_ulong)
            {
                0 as libc::c_int
            } else {
                *data
                    .offset(
                        (pos
                            .wrapping_add(i)
                            .wrapping_sub(window_half)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong) & mask)
                            as isize,
                    ) as libc::c_int
            }) as size_t;
            let mut last_c_0 = (if i
                < window_half.wrapping_add(2 as libc::c_int as libc::c_ulong)
            {
                0 as libc::c_int
            } else {
                *data
                    .offset(
                        (pos
                            .wrapping_add(i)
                            .wrapping_sub(window_half)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong) & mask)
                            as isize,
                    ) as libc::c_int
            }) as size_t;
            let mut utf8_pos2 = UTF8Position(last_c_0, c_0, max_utf8);
            histogram[utf8_pos2
                as usize][*data
                .offset((pos.wrapping_add(i).wrapping_sub(window_half) & mask) as isize)
                as usize]= histogram[utf8_pos2
                as usize][*data
                .offset((pos.wrapping_add(i).wrapping_sub(window_half) & mask) as isize)
                as usize]
                .wrapping_sub(1);
            in_window_utf8[utf8_pos2
                as usize]= in_window_utf8[utf8_pos2 as usize].wrapping_sub(1);
        }
        if i.wrapping_add(window_half) < len {
            let mut c_1 = *data
                .offset(
                    (pos
                        .wrapping_add(i)
                        .wrapping_add(window_half)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong) & mask) as isize,
                ) as size_t;
            let mut last_c_1 = *data
                .offset(
                    (pos
                        .wrapping_add(i)
                        .wrapping_add(window_half)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong) & mask) as isize,
                ) as size_t;
            let mut utf8_pos2_0 = UTF8Position(last_c_1, c_1, max_utf8);
            histogram[utf8_pos2_0
                as usize][*data
                .offset((pos.wrapping_add(i).wrapping_add(window_half) & mask) as isize)
                as usize]= histogram[utf8_pos2_0
                as usize][*data
                .offset((pos.wrapping_add(i).wrapping_add(window_half) & mask) as isize)
                as usize]
                .wrapping_add(1);
            in_window_utf8[utf8_pos2_0
                as usize]= in_window_utf8[utf8_pos2_0 as usize].wrapping_add(1);
        }
        let mut c_2 = (if i < 1 as libc::c_int as libc::c_ulong {
            0 as libc::c_int
        } else {
            *data
                .offset(
                    (pos.wrapping_add(i).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        & mask) as isize,
                ) as libc::c_int
        }) as size_t;
        let mut last_c_2 = (if i < 2 as libc::c_int as libc::c_ulong {
            0 as libc::c_int
        } else {
            *data
                .offset(
                    (pos.wrapping_add(i).wrapping_sub(2 as libc::c_int as libc::c_ulong)
                        & mask) as isize,
                ) as libc::c_int
        }) as size_t;
        let mut utf8_pos_0 = UTF8Position(last_c_2, c_2, max_utf8);
        let mut masked_pos = pos.wrapping_add(i) & mask;
        let mut histo = histogram[utf8_pos_0
            as usize][*data.offset(masked_pos as isize) as usize];
        let mut lit_cost: libc::c_double = 0.;
        if histo == 0 as libc::c_int as libc::c_ulong {
            histo= 1 as libc::c_int as size_t;
        }
        lit_cost= FastLog2(in_window_utf8[utf8_pos_0 as usize]) - FastLog2(histo);
        lit_cost+= 0.02905f64;
        if lit_cost < 1.0f64 {
            lit_cost*= 0.5f64;
            lit_cost+= 0.5f64;
        }
        if i < 2000 as libc::c_int as libc::c_ulong {
            lit_cost+= 0.7f64
                    - (2000 as libc::c_int as libc::c_ulong).wrapping_sub(i)
                        as libc::c_double / 2000.0f64 * 0.35f64;
        }
        *cost.offset(i as isize) = lit_cost as libc::c_float;
        i= i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn BrotliEstimateBitCostsForLiterals(
    mut pos: size_t,
    mut len: size_t,
    mut mask: size_t,
    mut data: *const uint8_t,
    mut cost: *mut libc::c_float,
) {
    if crate::src::enc::utf8_util::BrotliIsMostlyUTF8(data, pos, mask, len, crate::src::enc::literal_cost::kMinUTF8Ratio) != 0 {
        EstimateBitCostsForLiteralsUTF8(pos, len, mask, data, cost);
        return;
    } else {
        let mut histogram: [size_t; 256] = [
            0 as libc::c_int as size_t,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ];
        let mut window_half = 2000 as libc::c_int as size_t;
        let mut in_window = brotli_min_size_t(window_half, len);
        let mut i: size_t = 0;
        i= 0 as libc::c_int as size_t;
        while i < in_window {
            histogram[*data.offset((pos.wrapping_add(i) & mask) as isize)
                as usize]= histogram[*data
                .offset((pos.wrapping_add(i) & mask) as isize) as usize]
                .wrapping_add(1);
            i= i.wrapping_add(1);
        }
        i= 0 as libc::c_int as size_t;
        while i < len {
            let mut histo: size_t = 0;
            if i >= window_half {
                histogram[*data
                    .offset(
                        (pos.wrapping_add(i).wrapping_sub(window_half) & mask) as isize,
                    )
                    as usize]= histogram[*data
                    .offset(
                        (pos.wrapping_add(i).wrapping_sub(window_half) & mask) as isize,
                    ) as usize]
                    .wrapping_sub(1);
                in_window= in_window.wrapping_sub(1);
            }
            if i.wrapping_add(window_half) < len {
                histogram[*data
                    .offset(
                        (pos.wrapping_add(i).wrapping_add(window_half) & mask) as isize,
                    )
                    as usize]= histogram[*data
                    .offset(
                        (pos.wrapping_add(i).wrapping_add(window_half) & mask) as isize,
                    ) as usize]
                    .wrapping_add(1);
                in_window= in_window.wrapping_add(1);
            }
            histo= histogram[*data.offset((pos.wrapping_add(i) & mask) as isize)
                as usize];
            if histo == 0 as libc::c_int as libc::c_ulong {
                histo= 1 as libc::c_int as size_t;
            }
            let mut lit_cost = FastLog2(in_window) - FastLog2(histo);
            lit_cost+= 0.029f64;
            if lit_cost < 1.0f64 {
                lit_cost*= 0.5f64;
                lit_cost+= 0.5f64;
            }
            *cost.offset(i as isize) = lit_cost as libc::c_float;
            i= i.wrapping_add(1);
        }
    };
}
