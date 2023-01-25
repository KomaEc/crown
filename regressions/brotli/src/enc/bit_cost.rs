use ::libc;
extern "C" {
    fn log2(_: libc::c_double) -> libc::c_double;
    static kBrotliLog2Table: [libc::c_double; 256];
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HistogramLiteral {
    pub data_: [uint32_t; 256],
    pub total_count_: size_t,
    pub bit_cost_: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HistogramCommand {
    pub data_: [uint32_t; 704],
    pub total_count_: size_t,
    pub bit_cost_: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HistogramDistance {
    pub data_: [uint32_t; 544],
    pub total_count_: size_t,
    pub bit_cost_: libc::c_double,
}
#[inline(always)]
unsafe extern "C" fn brotli_max_uint32_t(mut a: uint32_t, mut b: uint32_t) -> uint32_t {
    return if a > b { a } else { b };
}
#[inline(always)]
unsafe extern "C" fn FastLog2(mut v: size_t) -> libc::c_double {
    if v < 256 as libc::c_int as libc::c_ulong {
        return crate::src::enc::bit_cost::kBrotliLog2Table[v as usize];
    }
    return log2(v as libc::c_double);
}
#[inline(always)]
unsafe extern "C" fn HistogramDataSizeLiteral() -> size_t {
    return 256 as libc::c_int as size_t;
}
#[inline(always)]
unsafe extern "C" fn HistogramDataSizeCommand() -> size_t {
    return 704 as libc::c_int as size_t;
}
#[inline(always)]
unsafe extern "C" fn HistogramDataSizeDistance() -> size_t {
    return 544 as libc::c_int as size_t;
}
#[inline(always)]
unsafe extern "C" fn ShannonEntropy(
    mut population: *const uint32_t,
    mut size: size_t,
    mut total: Option<&mut size_t>,
) -> libc::c_double {
    let mut current_block: u64;
    let mut sum = 0 as libc::c_int as size_t;
    let mut retval = 0 as libc::c_int as libc::c_double;
    let mut population_end = population.offset(size as isize);
    let mut p: size_t = 0;
    if size & 1 as libc::c_int as libc::c_ulong != 0 {
        current_block= 15800646830660818732;
    } else {
        current_block= 715039052867723359;
    }
    loop {
        match current_block {
            715039052867723359 => {
                if !(population < population_end) {
                    break;
                }
                let fresh0 = population;
                population= population.offset(1);
                p= (*fresh0) as size_t;
                sum= (sum as libc::c_ulong).wrapping_add(p) as size_t as size_t;
                retval-= p as libc::c_double * FastLog2(p);
                current_block= 15800646830660818732;
            }
            _ => {
                let fresh1 = population;
                population= population.offset(1);
                p= (*fresh1) as size_t;
                sum= (sum as libc::c_ulong).wrapping_add(p) as size_t as size_t;
                retval-= p as libc::c_double * FastLog2(p);
                current_block= 715039052867723359;
            }
        }
    }
    if sum != 0 {
        retval+= sum as libc::c_double * FastLog2(sum);
    }
    *total.as_deref_mut().unwrap()= sum;
    return retval;
}
#[inline(always)]
unsafe extern "C" fn BitsEntropy(
    mut population: *const uint32_t,
    mut size: size_t,
) -> libc::c_double {
    let mut sum: size_t = 0;
    let mut retval = ShannonEntropy(population, size, Some(&mut sum));
    if retval < sum as libc::c_double {
        retval= sum as libc::c_double;
    }
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn BrotliPopulationCostDistance(
    mut histogram: *const HistogramDistance,
) -> libc::c_double {
    static mut kOneSymbolHistogramCost: libc::c_double = 12 as libc::c_int
        as libc::c_double;
    static mut kTwoSymbolHistogramCost: libc::c_double = 20 as libc::c_int
        as libc::c_double;
    static mut kThreeSymbolHistogramCost: libc::c_double = 28 as libc::c_int
        as libc::c_double;
    static mut kFourSymbolHistogramCost: libc::c_double = 37 as libc::c_int
        as libc::c_double;
    let data_size = HistogramDataSizeDistance();
    let mut count = 0 as libc::c_int;
    let mut s: [size_t; 5] = [0; 5];
    let mut bits = 0.0f64;
    let mut i: size_t = 0;
    if (*histogram).total_count_ == 0 as libc::c_int as libc::c_ulong {
        return kOneSymbolHistogramCost;
    }
    i= 0 as libc::c_int as size_t;
    while i < data_size {
        if (*histogram).data_[i as usize] > 0 as libc::c_int as libc::c_uint {
            s[count as usize]= i;
            count+= 1;
            if count > 4 as libc::c_int {
                break;
            }
        }
        i= i.wrapping_add(1);
    }
    if count == 1 as libc::c_int {
        return kOneSymbolHistogramCost;
    }
    if count == 2 as libc::c_int {
        return kTwoSymbolHistogramCost + (*histogram).total_count_ as libc::c_double;
    }
    if count == 3 as libc::c_int {
        let histo0 = (*histogram).data_[s[0 as libc::c_int as usize] as usize];
        let histo1 = (*histogram).data_[s[1 as libc::c_int as usize] as usize];
        let histo2 = (*histogram).data_[s[2 as libc::c_int as usize] as usize];
        let histomax = brotli_max_uint32_t(histo0, brotli_max_uint32_t(histo1, histo2));
        return kThreeSymbolHistogramCost
            + (2 as libc::c_int as libc::c_uint)
                .wrapping_mul(histo0.wrapping_add(histo1).wrapping_add(histo2))
                as libc::c_double - histomax as libc::c_double;
    }
    if count == 4 as libc::c_int {
        let mut histo: [uint32_t; 4] = [0; 4];
        let mut h23: uint32_t = 0;
        let mut histomax_0: uint32_t = 0;
        i= 0 as libc::c_int as size_t;
        while i < 4 as libc::c_int as libc::c_ulong {
            histo[i as usize]= (*histogram).data_[s[i as usize] as usize];
            i= i.wrapping_add(1);
        }
        i= 0 as libc::c_int as size_t;
        while i < 4 as libc::c_int as libc::c_ulong {
            let mut j: size_t = 0;
            j= i.wrapping_add(1 as libc::c_int as libc::c_ulong);
            while j < 4 as libc::c_int as libc::c_ulong {
                if histo[j as usize] > histo[i as usize] {
                    let mut __brotli_swap_tmp = histo[j as usize];
                    histo[j as usize]= histo[i as usize];
                    histo[i as usize]= __brotli_swap_tmp;
                }
                j= j.wrapping_add(1);
            }
            i= i.wrapping_add(1);
        }
        h23= histo[2 as libc::c_int as usize]
            .wrapping_add(histo[3 as libc::c_int as usize]);
        histomax_0= brotli_max_uint32_t(h23, histo[0 as libc::c_int as usize]);
        return kFourSymbolHistogramCost
            + (3 as libc::c_int as libc::c_uint).wrapping_mul(h23) as libc::c_double
            + (2 as libc::c_int as libc::c_uint)
                .wrapping_mul(
                    histo[0 as libc::c_int as usize]
                        .wrapping_add(histo[1 as libc::c_int as usize]),
                ) as libc::c_double - histomax_0 as libc::c_double;
    }
    let mut max_depth = 1 as libc::c_int as size_t;
    let mut depth_histo: [uint32_t; 18] = [
        0 as libc::c_int as uint32_t,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
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
    let log2total = FastLog2((*histogram).total_count_);
    i= 0 as libc::c_int as size_t;
    while i < data_size {
        if (*histogram).data_[i as usize] > 0 as libc::c_int as libc::c_uint {
            let mut log2p = log2total
                - FastLog2((*histogram).data_[i as usize] as size_t);
            let mut depth = (log2p + 0.5f64) as size_t;
            bits+= (*histogram).data_[i as usize] as libc::c_double * log2p;
            if depth > 15 as libc::c_int as libc::c_ulong {
                depth= 15 as libc::c_int as size_t;
            }
            if depth > max_depth {
                max_depth= depth;
            }
            depth_histo[depth as usize]= depth_histo[depth as usize].wrapping_add(1);
            i= i.wrapping_add(1);
        } else {
            let mut reps = 1 as libc::c_int as uint32_t;
            let mut k: size_t = 0;
            k= i.wrapping_add(1 as libc::c_int as libc::c_ulong);
            while k < data_size
                && (*histogram).data_[k as usize] == 0 as libc::c_int as libc::c_uint
            {
                reps= reps.wrapping_add(1);
                k= k.wrapping_add(1);
            }
            i= (i as libc::c_ulong).wrapping_add(reps as libc::c_ulong) as size_t
                as size_t;
            if i == data_size {
                break;
            }
            if reps < 3 as libc::c_int as libc::c_uint {
                depth_histo[0 as libc::c_int
                    as usize]= (depth_histo[0 as libc::c_int as usize] as libc::c_uint)
                    .wrapping_add(reps) as uint32_t as uint32_t;
            } else {
                reps= (reps as libc::c_uint)
                    .wrapping_sub(2 as libc::c_int as libc::c_uint) as uint32_t
                    as uint32_t;
                while reps > 0 as libc::c_int as libc::c_uint {
                    depth_histo[17 as libc::c_int
                        as usize]= depth_histo[17 as libc::c_int as usize]
                        .wrapping_add(1);
                    bits+= 3 as libc::c_int as libc::c_double;
                    reps>>= 3 as libc::c_int;
                }
            }
        }
    }
    bits+= (18 as libc::c_int as libc::c_ulong)
            .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(max_depth))
            as libc::c_double;
    bits+= BitsEntropy(
            depth_histo.as_mut_ptr(),
            (17 as libc::c_int + 1 as libc::c_int) as size_t,
        );
    return bits;
}
#[no_mangle]
pub unsafe extern "C" fn BrotliPopulationCostCommand(
    mut histogram: *const HistogramCommand,
) -> libc::c_double {
    static mut kOneSymbolHistogramCost: libc::c_double = 12 as libc::c_int
        as libc::c_double;
    static mut kTwoSymbolHistogramCost: libc::c_double = 20 as libc::c_int
        as libc::c_double;
    static mut kThreeSymbolHistogramCost: libc::c_double = 28 as libc::c_int
        as libc::c_double;
    static mut kFourSymbolHistogramCost: libc::c_double = 37 as libc::c_int
        as libc::c_double;
    let data_size = HistogramDataSizeCommand();
    let mut count = 0 as libc::c_int;
    let mut s: [size_t; 5] = [0; 5];
    let mut bits = 0.0f64;
    let mut i: size_t = 0;
    if (*histogram).total_count_ == 0 as libc::c_int as libc::c_ulong {
        return kOneSymbolHistogramCost;
    }
    i= 0 as libc::c_int as size_t;
    while i < data_size {
        if (*histogram).data_[i as usize] > 0 as libc::c_int as libc::c_uint {
            s[count as usize]= i;
            count+= 1;
            if count > 4 as libc::c_int {
                break;
            }
        }
        i= i.wrapping_add(1);
    }
    if count == 1 as libc::c_int {
        return kOneSymbolHistogramCost;
    }
    if count == 2 as libc::c_int {
        return kTwoSymbolHistogramCost + (*histogram).total_count_ as libc::c_double;
    }
    if count == 3 as libc::c_int {
        let histo0 = (*histogram).data_[s[0 as libc::c_int as usize] as usize];
        let histo1 = (*histogram).data_[s[1 as libc::c_int as usize] as usize];
        let histo2 = (*histogram).data_[s[2 as libc::c_int as usize] as usize];
        let histomax = brotli_max_uint32_t(histo0, brotli_max_uint32_t(histo1, histo2));
        return kThreeSymbolHistogramCost
            + (2 as libc::c_int as libc::c_uint)
                .wrapping_mul(histo0.wrapping_add(histo1).wrapping_add(histo2))
                as libc::c_double - histomax as libc::c_double;
    }
    if count == 4 as libc::c_int {
        let mut histo: [uint32_t; 4] = [0; 4];
        let mut h23: uint32_t = 0;
        let mut histomax_0: uint32_t = 0;
        i= 0 as libc::c_int as size_t;
        while i < 4 as libc::c_int as libc::c_ulong {
            histo[i as usize]= (*histogram).data_[s[i as usize] as usize];
            i= i.wrapping_add(1);
        }
        i= 0 as libc::c_int as size_t;
        while i < 4 as libc::c_int as libc::c_ulong {
            let mut j: size_t = 0;
            j= i.wrapping_add(1 as libc::c_int as libc::c_ulong);
            while j < 4 as libc::c_int as libc::c_ulong {
                if histo[j as usize] > histo[i as usize] {
                    let mut __brotli_swap_tmp = histo[j as usize];
                    histo[j as usize]= histo[i as usize];
                    histo[i as usize]= __brotli_swap_tmp;
                }
                j= j.wrapping_add(1);
            }
            i= i.wrapping_add(1);
        }
        h23= histo[2 as libc::c_int as usize]
            .wrapping_add(histo[3 as libc::c_int as usize]);
        histomax_0= brotli_max_uint32_t(h23, histo[0 as libc::c_int as usize]);
        return kFourSymbolHistogramCost
            + (3 as libc::c_int as libc::c_uint).wrapping_mul(h23) as libc::c_double
            + (2 as libc::c_int as libc::c_uint)
                .wrapping_mul(
                    histo[0 as libc::c_int as usize]
                        .wrapping_add(histo[1 as libc::c_int as usize]),
                ) as libc::c_double - histomax_0 as libc::c_double;
    }
    let mut max_depth = 1 as libc::c_int as size_t;
    let mut depth_histo: [uint32_t; 18] = [
        0 as libc::c_int as uint32_t,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
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
    let log2total = FastLog2((*histogram).total_count_);
    i= 0 as libc::c_int as size_t;
    while i < data_size {
        if (*histogram).data_[i as usize] > 0 as libc::c_int as libc::c_uint {
            let mut log2p = log2total
                - FastLog2((*histogram).data_[i as usize] as size_t);
            let mut depth = (log2p + 0.5f64) as size_t;
            bits+= (*histogram).data_[i as usize] as libc::c_double * log2p;
            if depth > 15 as libc::c_int as libc::c_ulong {
                depth= 15 as libc::c_int as size_t;
            }
            if depth > max_depth {
                max_depth= depth;
            }
            depth_histo[depth as usize]= depth_histo[depth as usize].wrapping_add(1);
            i= i.wrapping_add(1);
        } else {
            let mut reps = 1 as libc::c_int as uint32_t;
            let mut k: size_t = 0;
            k= i.wrapping_add(1 as libc::c_int as libc::c_ulong);
            while k < data_size
                && (*histogram).data_[k as usize] == 0 as libc::c_int as libc::c_uint
            {
                reps= reps.wrapping_add(1);
                k= k.wrapping_add(1);
            }
            i= (i as libc::c_ulong).wrapping_add(reps as libc::c_ulong) as size_t
                as size_t;
            if i == data_size {
                break;
            }
            if reps < 3 as libc::c_int as libc::c_uint {
                depth_histo[0 as libc::c_int
                    as usize]= (depth_histo[0 as libc::c_int as usize] as libc::c_uint)
                    .wrapping_add(reps) as uint32_t as uint32_t;
            } else {
                reps= (reps as libc::c_uint)
                    .wrapping_sub(2 as libc::c_int as libc::c_uint) as uint32_t
                    as uint32_t;
                while reps > 0 as libc::c_int as libc::c_uint {
                    depth_histo[17 as libc::c_int
                        as usize]= depth_histo[17 as libc::c_int as usize]
                        .wrapping_add(1);
                    bits+= 3 as libc::c_int as libc::c_double;
                    reps>>= 3 as libc::c_int;
                }
            }
        }
    }
    bits+= (18 as libc::c_int as libc::c_ulong)
            .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(max_depth))
            as libc::c_double;
    bits+= BitsEntropy(
            depth_histo.as_mut_ptr(),
            (17 as libc::c_int + 1 as libc::c_int) as size_t,
        );
    return bits;
}
#[no_mangle]
pub unsafe extern "C" fn BrotliPopulationCostLiteral(
    mut histogram: *const HistogramLiteral,
) -> libc::c_double {
    static mut kOneSymbolHistogramCost: libc::c_double = 12 as libc::c_int
        as libc::c_double;
    static mut kTwoSymbolHistogramCost: libc::c_double = 20 as libc::c_int
        as libc::c_double;
    static mut kThreeSymbolHistogramCost: libc::c_double = 28 as libc::c_int
        as libc::c_double;
    static mut kFourSymbolHistogramCost: libc::c_double = 37 as libc::c_int
        as libc::c_double;
    let data_size = HistogramDataSizeLiteral();
    let mut count = 0 as libc::c_int;
    let mut s: [size_t; 5] = [0; 5];
    let mut bits = 0.0f64;
    let mut i: size_t = 0;
    if (*histogram).total_count_ == 0 as libc::c_int as libc::c_ulong {
        return kOneSymbolHistogramCost;
    }
    i= 0 as libc::c_int as size_t;
    while i < data_size {
        if (*histogram).data_[i as usize] > 0 as libc::c_int as libc::c_uint {
            s[count as usize]= i;
            count+= 1;
            if count > 4 as libc::c_int {
                break;
            }
        }
        i= i.wrapping_add(1);
    }
    if count == 1 as libc::c_int {
        return kOneSymbolHistogramCost;
    }
    if count == 2 as libc::c_int {
        return kTwoSymbolHistogramCost + (*histogram).total_count_ as libc::c_double;
    }
    if count == 3 as libc::c_int {
        let histo0 = (*histogram).data_[s[0 as libc::c_int as usize] as usize];
        let histo1 = (*histogram).data_[s[1 as libc::c_int as usize] as usize];
        let histo2 = (*histogram).data_[s[2 as libc::c_int as usize] as usize];
        let histomax = brotli_max_uint32_t(histo0, brotli_max_uint32_t(histo1, histo2));
        return kThreeSymbolHistogramCost
            + (2 as libc::c_int as libc::c_uint)
                .wrapping_mul(histo0.wrapping_add(histo1).wrapping_add(histo2))
                as libc::c_double - histomax as libc::c_double;
    }
    if count == 4 as libc::c_int {
        let mut histo: [uint32_t; 4] = [0; 4];
        let mut h23: uint32_t = 0;
        let mut histomax_0: uint32_t = 0;
        i= 0 as libc::c_int as size_t;
        while i < 4 as libc::c_int as libc::c_ulong {
            histo[i as usize]= (*histogram).data_[s[i as usize] as usize];
            i= i.wrapping_add(1);
        }
        i= 0 as libc::c_int as size_t;
        while i < 4 as libc::c_int as libc::c_ulong {
            let mut j: size_t = 0;
            j= i.wrapping_add(1 as libc::c_int as libc::c_ulong);
            while j < 4 as libc::c_int as libc::c_ulong {
                if histo[j as usize] > histo[i as usize] {
                    let mut __brotli_swap_tmp = histo[j as usize];
                    histo[j as usize]= histo[i as usize];
                    histo[i as usize]= __brotli_swap_tmp;
                }
                j= j.wrapping_add(1);
            }
            i= i.wrapping_add(1);
        }
        h23= histo[2 as libc::c_int as usize]
            .wrapping_add(histo[3 as libc::c_int as usize]);
        histomax_0= brotli_max_uint32_t(h23, histo[0 as libc::c_int as usize]);
        return kFourSymbolHistogramCost
            + (3 as libc::c_int as libc::c_uint).wrapping_mul(h23) as libc::c_double
            + (2 as libc::c_int as libc::c_uint)
                .wrapping_mul(
                    histo[0 as libc::c_int as usize]
                        .wrapping_add(histo[1 as libc::c_int as usize]),
                ) as libc::c_double - histomax_0 as libc::c_double;
    }
    let mut max_depth = 1 as libc::c_int as size_t;
    let mut depth_histo: [uint32_t; 18] = [
        0 as libc::c_int as uint32_t,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
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
    let log2total = FastLog2((*histogram).total_count_);
    i= 0 as libc::c_int as size_t;
    while i < data_size {
        if (*histogram).data_[i as usize] > 0 as libc::c_int as libc::c_uint {
            let mut log2p = log2total
                - FastLog2((*histogram).data_[i as usize] as size_t);
            let mut depth = (log2p + 0.5f64) as size_t;
            bits+= (*histogram).data_[i as usize] as libc::c_double * log2p;
            if depth > 15 as libc::c_int as libc::c_ulong {
                depth= 15 as libc::c_int as size_t;
            }
            if depth > max_depth {
                max_depth= depth;
            }
            depth_histo[depth as usize]= depth_histo[depth as usize].wrapping_add(1);
            i= i.wrapping_add(1);
        } else {
            let mut reps = 1 as libc::c_int as uint32_t;
            let mut k: size_t = 0;
            k= i.wrapping_add(1 as libc::c_int as libc::c_ulong);
            while k < data_size
                && (*histogram).data_[k as usize] == 0 as libc::c_int as libc::c_uint
            {
                reps= reps.wrapping_add(1);
                k= k.wrapping_add(1);
            }
            i= (i as libc::c_ulong).wrapping_add(reps as libc::c_ulong) as size_t
                as size_t;
            if i == data_size {
                break;
            }
            if reps < 3 as libc::c_int as libc::c_uint {
                depth_histo[0 as libc::c_int
                    as usize]= (depth_histo[0 as libc::c_int as usize] as libc::c_uint)
                    .wrapping_add(reps) as uint32_t as uint32_t;
            } else {
                reps= (reps as libc::c_uint)
                    .wrapping_sub(2 as libc::c_int as libc::c_uint) as uint32_t
                    as uint32_t;
                while reps > 0 as libc::c_int as libc::c_uint {
                    depth_histo[17 as libc::c_int
                        as usize]= depth_histo[17 as libc::c_int as usize]
                        .wrapping_add(1);
                    bits+= 3 as libc::c_int as libc::c_double;
                    reps>>= 3 as libc::c_int;
                }
            }
        }
    }
    bits+= (18 as libc::c_int as libc::c_ulong)
            .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(max_depth))
            as libc::c_double;
    bits+= BitsEntropy(
            depth_histo.as_mut_ptr(),
            (17 as libc::c_int + 1 as libc::c_int) as size_t,
        );
    return bits;
}
