use ::libc;
extern "C" {
    fn log2(_: libc::c_double) -> libc::c_double;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    static kBrotliLog2Table: [libc::c_double; 256];
    fn BrotliAllocate(m: *mut MemoryManager, n: size_t) -> *mut libc::c_void;
    fn BrotliFree(m: *mut MemoryManager, p: *mut libc::c_void);
    fn BrotliPopulationCostLiteral(_: *const HistogramLiteral) -> libc::c_double;
    fn BrotliPopulationCostCommand(_: *const HistogramCommand) -> libc::c_double;
    fn BrotliPopulationCostDistance(_: *const HistogramDistance) -> libc::c_double;
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
pub type brotli_alloc_func = Option::<
    unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void,
>;
pub type brotli_free_func = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MemoryManager {
    pub alloc_func: brotli_alloc_func,
    pub free_func: brotli_free_func,
    pub opaque: *mut libc::c_void,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HistogramPair {
    pub idx1: uint32_t,
    pub idx2: uint32_t,
    pub cost_combo: libc::c_double,
    pub cost_diff: libc::c_double,
}
#[inline(always)]
unsafe extern "C" fn brotli_min_size_t(mut a: size_t, mut b: size_t) -> size_t {
    return if a < b { a } else { b };
}
#[inline(always)]
unsafe extern "C" fn brotli_max_double(
    mut a: libc::c_double,
    mut b: libc::c_double,
) -> libc::c_double {
    return if a > b { a } else { b };
}
#[inline(always)]
unsafe extern "C" fn FastLog2(mut v: size_t) -> libc::c_double {
    if v < 256 as libc::c_int as libc::c_ulong {
        return kBrotliLog2Table[v as usize];
    }
    return log2(v as libc::c_double);
}
#[inline(always)]
unsafe extern "C" fn HistogramClearLiteral(mut self_0: *mut HistogramLiteral) {
    memset(
        ((*self_0).data_).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[uint32_t; 256]>() as libc::c_ulong,
    );
    (*self_0).total_count_ = 0 as libc::c_int as size_t;
    (*self_0).bit_cost_ = ::std::f64::INFINITY;
}
#[inline(always)]
unsafe extern "C" fn HistogramAddHistogramLiteral(
    mut self_0: *mut HistogramLiteral,
    mut v: *const HistogramLiteral,
) {
    let mut i: size_t = 0;
    let ref mut fresh0 = (*self_0).total_count_;
    *fresh0 = (*fresh0 as libc::c_ulong).wrapping_add((*v).total_count_) as size_t
        as size_t;
    i = 0 as libc::c_int as size_t;
    while i < 256 as libc::c_int as libc::c_ulong {
        let ref mut fresh1 = (*self_0).data_[i as usize];
        *fresh1 = (*fresh1 as libc::c_uint).wrapping_add((*v).data_[i as usize])
            as uint32_t as uint32_t;
        i = i.wrapping_add(1);
    }
}
#[inline(always)]
unsafe extern "C" fn HistogramClearCommand(mut self_0: *mut HistogramCommand) {
    memset(
        ((*self_0).data_).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[uint32_t; 704]>() as libc::c_ulong,
    );
    (*self_0).total_count_ = 0 as libc::c_int as size_t;
    (*self_0).bit_cost_ = ::std::f64::INFINITY;
}
#[inline(always)]
unsafe extern "C" fn HistogramAddHistogramCommand(
    mut self_0: *mut HistogramCommand,
    mut v: *const HistogramCommand,
) {
    let mut i: size_t = 0;
    let ref mut fresh2 = (*self_0).total_count_;
    *fresh2 = (*fresh2 as libc::c_ulong).wrapping_add((*v).total_count_) as size_t
        as size_t;
    i = 0 as libc::c_int as size_t;
    while i < 704 as libc::c_int as libc::c_ulong {
        let ref mut fresh3 = (*self_0).data_[i as usize];
        *fresh3 = (*fresh3 as libc::c_uint).wrapping_add((*v).data_[i as usize])
            as uint32_t as uint32_t;
        i = i.wrapping_add(1);
    }
}
#[inline(always)]
unsafe extern "C" fn HistogramClearDistance(mut self_0: *mut HistogramDistance) {
    memset(
        ((*self_0).data_).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[uint32_t; 544]>() as libc::c_ulong,
    );
    (*self_0).total_count_ = 0 as libc::c_int as size_t;
    (*self_0).bit_cost_ = ::std::f64::INFINITY;
}
#[inline(always)]
unsafe extern "C" fn HistogramAddHistogramDistance(
    mut self_0: *mut HistogramDistance,
    mut v: *const HistogramDistance,
) {
    let mut i: size_t = 0;
    let ref mut fresh4 = (*self_0).total_count_;
    *fresh4 = (*fresh4 as libc::c_ulong).wrapping_add((*v).total_count_) as size_t
        as size_t;
    i = 0 as libc::c_int as size_t;
    while i < 544 as libc::c_int as libc::c_ulong {
        let ref mut fresh5 = (*self_0).data_[i as usize];
        *fresh5 = (*fresh5 as libc::c_uint).wrapping_add((*v).data_[i as usize])
            as uint32_t as uint32_t;
        i = i.wrapping_add(1);
    }
}
#[inline(always)]
unsafe extern "C" fn HistogramPairIsLess(
    mut p1: *const HistogramPair,
    mut p2: *const HistogramPair,
) -> libc::c_int {
    if (*p1).cost_diff != (*p2).cost_diff {
        return if (*p1).cost_diff > (*p2).cost_diff {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        };
    }
    return if ((*p1).idx2).wrapping_sub((*p1).idx1)
        > ((*p2).idx2).wrapping_sub((*p2).idx1)
    {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
}
#[inline(always)]
unsafe extern "C" fn ClusterCostDiff(
    mut size_a: size_t,
    mut size_b: size_t,
) -> libc::c_double {
    let mut size_c = size_a.wrapping_add(size_b);
    return size_a as libc::c_double * FastLog2(size_a)
        + size_b as libc::c_double * FastLog2(size_b)
        - size_c as libc::c_double * FastLog2(size_c);
}
#[no_mangle]
pub unsafe extern "C" fn BrotliCompareAndPushToQueueLiteral(
    mut out: *const HistogramLiteral,
    mut cluster_size: *const uint32_t,
    mut idx1: uint32_t,
    mut idx2: uint32_t,
    mut max_num_pairs: size_t,
    mut pairs: *mut HistogramPair,
    mut num_pairs: *mut size_t,
) {
    let mut is_good_pair = 0 as libc::c_int;
    let mut p = HistogramPair {
        idx1: 0,
        idx2: 0,
        cost_combo: 0.,
        cost_diff: 0.,
    };
    p.idx2 = 0 as libc::c_int as uint32_t;
    p.idx1 = p.idx2;
    p.cost_combo = 0 as libc::c_int as libc::c_double;
    p.cost_diff = p.cost_combo;
    if idx1 == idx2 {
        return;
    }
    if idx2 < idx1 {
        let mut t = idx2;
        idx2 = idx1;
        idx1 = t;
    }
    p.idx1 = idx1;
    p.idx2 = idx2;
    p
        .cost_diff = 0.5f64
        * ClusterCostDiff(
            *cluster_size.offset(idx1 as isize) as size_t,
            *cluster_size.offset(idx2 as isize) as size_t,
        );
    p.cost_diff -= (*out.offset(idx1 as isize)).bit_cost_;
    p.cost_diff -= (*out.offset(idx2 as isize)).bit_cost_;
    if (*out.offset(idx1 as isize)).total_count_ == 0 as libc::c_int as libc::c_ulong {
        p.cost_combo = (*out.offset(idx2 as isize)).bit_cost_;
        is_good_pair = 1 as libc::c_int;
    } else if (*out.offset(idx2 as isize)).total_count_
        == 0 as libc::c_int as libc::c_ulong
    {
        p.cost_combo = (*out.offset(idx1 as isize)).bit_cost_;
        is_good_pair = 1 as libc::c_int;
    } else {
        let mut threshold = if *num_pairs == 0 as libc::c_int as libc::c_ulong {
            1e99f64
        } else {
            brotli_max_double(
                0.0f64,
                (*pairs.offset(0 as libc::c_int as isize)).cost_diff,
            )
        };
        let mut combo = *out.offset(idx1 as isize);
        let mut cost_combo: libc::c_double = 0.;
        HistogramAddHistogramLiteral(&mut combo, &*out.offset(idx2 as isize));
        cost_combo = BrotliPopulationCostLiteral(&mut combo);
        if cost_combo < threshold - p.cost_diff {
            p.cost_combo = cost_combo;
            is_good_pair = 1 as libc::c_int;
        }
    }
    if is_good_pair != 0 {
        p.cost_diff += p.cost_combo;
        if *num_pairs > 0 as libc::c_int as libc::c_ulong
            && HistogramPairIsLess(&mut *pairs.offset(0 as libc::c_int as isize), &mut p)
                != 0
        {
            if *num_pairs < max_num_pairs {
                *pairs
                    .offset(
                        *num_pairs as isize,
                    ) = *pairs.offset(0 as libc::c_int as isize);
                *num_pairs = (*num_pairs).wrapping_add(1);
            }
            *pairs.offset(0 as libc::c_int as isize) = p;
        } else if *num_pairs < max_num_pairs {
            *pairs.offset(*num_pairs as isize) = p;
            *num_pairs = (*num_pairs).wrapping_add(1);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn BrotliCompareAndPushToQueueCommand(
    mut out: *const HistogramCommand,
    mut cluster_size: *const uint32_t,
    mut idx1: uint32_t,
    mut idx2: uint32_t,
    mut max_num_pairs: size_t,
    mut pairs: *mut HistogramPair,
    mut num_pairs: *mut size_t,
) {
    let mut is_good_pair = 0 as libc::c_int;
    let mut p = HistogramPair {
        idx1: 0,
        idx2: 0,
        cost_combo: 0.,
        cost_diff: 0.,
    };
    p.idx2 = 0 as libc::c_int as uint32_t;
    p.idx1 = p.idx2;
    p.cost_combo = 0 as libc::c_int as libc::c_double;
    p.cost_diff = p.cost_combo;
    if idx1 == idx2 {
        return;
    }
    if idx2 < idx1 {
        let mut t = idx2;
        idx2 = idx1;
        idx1 = t;
    }
    p.idx1 = idx1;
    p.idx2 = idx2;
    p
        .cost_diff = 0.5f64
        * ClusterCostDiff(
            *cluster_size.offset(idx1 as isize) as size_t,
            *cluster_size.offset(idx2 as isize) as size_t,
        );
    p.cost_diff -= (*out.offset(idx1 as isize)).bit_cost_;
    p.cost_diff -= (*out.offset(idx2 as isize)).bit_cost_;
    if (*out.offset(idx1 as isize)).total_count_ == 0 as libc::c_int as libc::c_ulong {
        p.cost_combo = (*out.offset(idx2 as isize)).bit_cost_;
        is_good_pair = 1 as libc::c_int;
    } else if (*out.offset(idx2 as isize)).total_count_
        == 0 as libc::c_int as libc::c_ulong
    {
        p.cost_combo = (*out.offset(idx1 as isize)).bit_cost_;
        is_good_pair = 1 as libc::c_int;
    } else {
        let mut threshold = if *num_pairs == 0 as libc::c_int as libc::c_ulong {
            1e99f64
        } else {
            brotli_max_double(
                0.0f64,
                (*pairs.offset(0 as libc::c_int as isize)).cost_diff,
            )
        };
        let mut combo = *out.offset(idx1 as isize);
        let mut cost_combo: libc::c_double = 0.;
        HistogramAddHistogramCommand(&mut combo, &*out.offset(idx2 as isize));
        cost_combo = BrotliPopulationCostCommand(&mut combo);
        if cost_combo < threshold - p.cost_diff {
            p.cost_combo = cost_combo;
            is_good_pair = 1 as libc::c_int;
        }
    }
    if is_good_pair != 0 {
        p.cost_diff += p.cost_combo;
        if *num_pairs > 0 as libc::c_int as libc::c_ulong
            && HistogramPairIsLess(&mut *pairs.offset(0 as libc::c_int as isize), &mut p)
                != 0
        {
            if *num_pairs < max_num_pairs {
                *pairs
                    .offset(
                        *num_pairs as isize,
                    ) = *pairs.offset(0 as libc::c_int as isize);
                *num_pairs = (*num_pairs).wrapping_add(1);
            }
            *pairs.offset(0 as libc::c_int as isize) = p;
        } else if *num_pairs < max_num_pairs {
            *pairs.offset(*num_pairs as isize) = p;
            *num_pairs = (*num_pairs).wrapping_add(1);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn BrotliCompareAndPushToQueueDistance(
    mut out: *const HistogramDistance,
    mut cluster_size: *const uint32_t,
    mut idx1: uint32_t,
    mut idx2: uint32_t,
    mut max_num_pairs: size_t,
    mut pairs: *mut HistogramPair,
    mut num_pairs: *mut size_t,
) {
    let mut is_good_pair = 0 as libc::c_int;
    let mut p = HistogramPair {
        idx1: 0,
        idx2: 0,
        cost_combo: 0.,
        cost_diff: 0.,
    };
    p.idx2 = 0 as libc::c_int as uint32_t;
    p.idx1 = p.idx2;
    p.cost_combo = 0 as libc::c_int as libc::c_double;
    p.cost_diff = p.cost_combo;
    if idx1 == idx2 {
        return;
    }
    if idx2 < idx1 {
        let mut t = idx2;
        idx2 = idx1;
        idx1 = t;
    }
    p.idx1 = idx1;
    p.idx2 = idx2;
    p
        .cost_diff = 0.5f64
        * ClusterCostDiff(
            *cluster_size.offset(idx1 as isize) as size_t,
            *cluster_size.offset(idx2 as isize) as size_t,
        );
    p.cost_diff -= (*out.offset(idx1 as isize)).bit_cost_;
    p.cost_diff -= (*out.offset(idx2 as isize)).bit_cost_;
    if (*out.offset(idx1 as isize)).total_count_ == 0 as libc::c_int as libc::c_ulong {
        p.cost_combo = (*out.offset(idx2 as isize)).bit_cost_;
        is_good_pair = 1 as libc::c_int;
    } else if (*out.offset(idx2 as isize)).total_count_
        == 0 as libc::c_int as libc::c_ulong
    {
        p.cost_combo = (*out.offset(idx1 as isize)).bit_cost_;
        is_good_pair = 1 as libc::c_int;
    } else {
        let mut threshold = if *num_pairs == 0 as libc::c_int as libc::c_ulong {
            1e99f64
        } else {
            brotli_max_double(
                0.0f64,
                (*pairs.offset(0 as libc::c_int as isize)).cost_diff,
            )
        };
        let mut combo = *out.offset(idx1 as isize);
        let mut cost_combo: libc::c_double = 0.;
        HistogramAddHistogramDistance(&mut combo, &*out.offset(idx2 as isize));
        cost_combo = BrotliPopulationCostDistance(&mut combo);
        if cost_combo < threshold - p.cost_diff {
            p.cost_combo = cost_combo;
            is_good_pair = 1 as libc::c_int;
        }
    }
    if is_good_pair != 0 {
        p.cost_diff += p.cost_combo;
        if *num_pairs > 0 as libc::c_int as libc::c_ulong
            && HistogramPairIsLess(&mut *pairs.offset(0 as libc::c_int as isize), &mut p)
                != 0
        {
            if *num_pairs < max_num_pairs {
                *pairs
                    .offset(
                        *num_pairs as isize,
                    ) = *pairs.offset(0 as libc::c_int as isize);
                *num_pairs = (*num_pairs).wrapping_add(1);
            }
            *pairs.offset(0 as libc::c_int as isize) = p;
        } else if *num_pairs < max_num_pairs {
            *pairs.offset(*num_pairs as isize) = p;
            *num_pairs = (*num_pairs).wrapping_add(1);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn BrotliHistogramCombineLiteral(
    mut out: *mut HistogramLiteral,
    mut cluster_size: *mut uint32_t,
    mut symbols: *mut uint32_t,
    mut clusters: *mut uint32_t,
    mut pairs: *mut HistogramPair,
    mut num_clusters: size_t,
    mut symbols_size: size_t,
    mut max_clusters: size_t,
    mut max_num_pairs: size_t,
) -> size_t {
    let mut cost_diff_threshold = 0.0f64;
    let mut min_cluster_size = 1 as libc::c_int as size_t;
    let mut num_pairs = 0 as libc::c_int as size_t;
    let mut idx1: size_t = 0;
    idx1 = 0 as libc::c_int as size_t;
    while idx1 < num_clusters {
        let mut idx2: size_t = 0;
        idx2 = idx1.wrapping_add(1 as libc::c_int as libc::c_ulong);
        while idx2 < num_clusters {
            BrotliCompareAndPushToQueueLiteral(
                out,
                cluster_size,
                *clusters.offset(idx1 as isize),
                *clusters.offset(idx2 as isize),
                max_num_pairs,
                &mut *pairs.offset(0 as libc::c_int as isize),
                &mut num_pairs,
            );
            idx2 = idx2.wrapping_add(1);
        }
        idx1 = idx1.wrapping_add(1);
    }
    while num_clusters > min_cluster_size {
        let mut best_idx1: uint32_t = 0;
        let mut best_idx2: uint32_t = 0;
        let mut i: size_t = 0;
        if (*pairs.offset(0 as libc::c_int as isize)).cost_diff >= cost_diff_threshold {
            cost_diff_threshold = 1e99f64;
            min_cluster_size = max_clusters;
        } else {
            best_idx1 = (*pairs.offset(0 as libc::c_int as isize)).idx1;
            best_idx2 = (*pairs.offset(0 as libc::c_int as isize)).idx2;
            HistogramAddHistogramLiteral(
                &mut *out.offset(best_idx1 as isize),
                &mut *out.offset(best_idx2 as isize),
            );
            (*out.offset(best_idx1 as isize))
                .bit_cost_ = (*pairs.offset(0 as libc::c_int as isize)).cost_combo;
            let ref mut fresh6 = *cluster_size.offset(best_idx1 as isize);
            *fresh6 = (*fresh6 as libc::c_uint)
                .wrapping_add(*cluster_size.offset(best_idx2 as isize)) as uint32_t
                as uint32_t;
            i = 0 as libc::c_int as size_t;
            while i < symbols_size {
                if *symbols.offset(i as isize) == best_idx2 {
                    *symbols.offset(i as isize) = best_idx1;
                }
                i = i.wrapping_add(1);
            }
            i = 0 as libc::c_int as size_t;
            while i < num_clusters {
                if *clusters.offset(i as isize) == best_idx2 {
                    memmove(
                        &mut *clusters.offset(i as isize) as *mut uint32_t
                            as *mut libc::c_void,
                        &mut *clusters
                            .offset(
                                i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                            ) as *mut uint32_t as *const libc::c_void,
                        num_clusters
                            .wrapping_sub(i)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<uint32_t>() as libc::c_ulong,
                            ),
                    );
                    break;
                } else {
                    i = i.wrapping_add(1);
                }
            }
            num_clusters = num_clusters.wrapping_sub(1);
            let mut copy_to_idx = 0 as libc::c_int as size_t;
            i = 0 as libc::c_int as size_t;
            while i < num_pairs {
                let mut p: *mut HistogramPair = &mut *pairs.offset(i as isize)
                    as *mut HistogramPair;
                if !((*p).idx1 == best_idx1 || (*p).idx2 == best_idx1
                    || (*p).idx1 == best_idx2 || (*p).idx2 == best_idx2)
                {
                    if HistogramPairIsLess(
                        &mut *pairs.offset(0 as libc::c_int as isize),
                        p,
                    ) != 0
                    {
                        let mut front = *pairs.offset(0 as libc::c_int as isize);
                        *pairs.offset(0 as libc::c_int as isize) = *p;
                        *pairs.offset(copy_to_idx as isize) = front;
                    } else {
                        *pairs.offset(copy_to_idx as isize) = *p;
                    }
                    copy_to_idx = copy_to_idx.wrapping_add(1);
                }
                i = i.wrapping_add(1);
            }
            num_pairs = copy_to_idx;
            i = 0 as libc::c_int as size_t;
            while i < num_clusters {
                BrotliCompareAndPushToQueueLiteral(
                    out,
                    cluster_size,
                    best_idx1,
                    *clusters.offset(i as isize),
                    max_num_pairs,
                    &mut *pairs.offset(0 as libc::c_int as isize),
                    &mut num_pairs,
                );
                i = i.wrapping_add(1);
            }
        }
    }
    return num_clusters;
}
#[no_mangle]
pub unsafe extern "C" fn BrotliHistogramCombineDistance(
    mut out: *mut HistogramDistance,
    mut cluster_size: *mut uint32_t,
    mut symbols: *mut uint32_t,
    mut clusters: *mut uint32_t,
    mut pairs: *mut HistogramPair,
    mut num_clusters: size_t,
    mut symbols_size: size_t,
    mut max_clusters: size_t,
    mut max_num_pairs: size_t,
) -> size_t {
    let mut cost_diff_threshold = 0.0f64;
    let mut min_cluster_size = 1 as libc::c_int as size_t;
    let mut num_pairs = 0 as libc::c_int as size_t;
    let mut idx1: size_t = 0;
    idx1 = 0 as libc::c_int as size_t;
    while idx1 < num_clusters {
        let mut idx2: size_t = 0;
        idx2 = idx1.wrapping_add(1 as libc::c_int as libc::c_ulong);
        while idx2 < num_clusters {
            BrotliCompareAndPushToQueueDistance(
                out,
                cluster_size,
                *clusters.offset(idx1 as isize),
                *clusters.offset(idx2 as isize),
                max_num_pairs,
                &mut *pairs.offset(0 as libc::c_int as isize),
                &mut num_pairs,
            );
            idx2 = idx2.wrapping_add(1);
        }
        idx1 = idx1.wrapping_add(1);
    }
    while num_clusters > min_cluster_size {
        let mut best_idx1: uint32_t = 0;
        let mut best_idx2: uint32_t = 0;
        let mut i: size_t = 0;
        if (*pairs.offset(0 as libc::c_int as isize)).cost_diff >= cost_diff_threshold {
            cost_diff_threshold = 1e99f64;
            min_cluster_size = max_clusters;
        } else {
            best_idx1 = (*pairs.offset(0 as libc::c_int as isize)).idx1;
            best_idx2 = (*pairs.offset(0 as libc::c_int as isize)).idx2;
            HistogramAddHistogramDistance(
                &mut *out.offset(best_idx1 as isize),
                &mut *out.offset(best_idx2 as isize),
            );
            (*out.offset(best_idx1 as isize))
                .bit_cost_ = (*pairs.offset(0 as libc::c_int as isize)).cost_combo;
            let ref mut fresh7 = *cluster_size.offset(best_idx1 as isize);
            *fresh7 = (*fresh7 as libc::c_uint)
                .wrapping_add(*cluster_size.offset(best_idx2 as isize)) as uint32_t
                as uint32_t;
            i = 0 as libc::c_int as size_t;
            while i < symbols_size {
                if *symbols.offset(i as isize) == best_idx2 {
                    *symbols.offset(i as isize) = best_idx1;
                }
                i = i.wrapping_add(1);
            }
            i = 0 as libc::c_int as size_t;
            while i < num_clusters {
                if *clusters.offset(i as isize) == best_idx2 {
                    memmove(
                        &mut *clusters.offset(i as isize) as *mut uint32_t
                            as *mut libc::c_void,
                        &mut *clusters
                            .offset(
                                i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                            ) as *mut uint32_t as *const libc::c_void,
                        num_clusters
                            .wrapping_sub(i)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<uint32_t>() as libc::c_ulong,
                            ),
                    );
                    break;
                } else {
                    i = i.wrapping_add(1);
                }
            }
            num_clusters = num_clusters.wrapping_sub(1);
            let mut copy_to_idx = 0 as libc::c_int as size_t;
            i = 0 as libc::c_int as size_t;
            while i < num_pairs {
                let mut p: *mut HistogramPair = &mut *pairs.offset(i as isize)
                    as *mut HistogramPair;
                if !((*p).idx1 == best_idx1 || (*p).idx2 == best_idx1
                    || (*p).idx1 == best_idx2 || (*p).idx2 == best_idx2)
                {
                    if HistogramPairIsLess(
                        &mut *pairs.offset(0 as libc::c_int as isize),
                        p,
                    ) != 0
                    {
                        let mut front = *pairs.offset(0 as libc::c_int as isize);
                        *pairs.offset(0 as libc::c_int as isize) = *p;
                        *pairs.offset(copy_to_idx as isize) = front;
                    } else {
                        *pairs.offset(copy_to_idx as isize) = *p;
                    }
                    copy_to_idx = copy_to_idx.wrapping_add(1);
                }
                i = i.wrapping_add(1);
            }
            num_pairs = copy_to_idx;
            i = 0 as libc::c_int as size_t;
            while i < num_clusters {
                BrotliCompareAndPushToQueueDistance(
                    out,
                    cluster_size,
                    best_idx1,
                    *clusters.offset(i as isize),
                    max_num_pairs,
                    &mut *pairs.offset(0 as libc::c_int as isize),
                    &mut num_pairs,
                );
                i = i.wrapping_add(1);
            }
        }
    }
    return num_clusters;
}
#[no_mangle]
pub unsafe extern "C" fn BrotliHistogramCombineCommand(
    mut out: *mut HistogramCommand,
    mut cluster_size: *mut uint32_t,
    mut symbols: *mut uint32_t,
    mut clusters: *mut uint32_t,
    mut pairs: *mut HistogramPair,
    mut num_clusters: size_t,
    mut symbols_size: size_t,
    mut max_clusters: size_t,
    mut max_num_pairs: size_t,
) -> size_t {
    let mut cost_diff_threshold = 0.0f64;
    let mut min_cluster_size = 1 as libc::c_int as size_t;
    let mut num_pairs = 0 as libc::c_int as size_t;
    let mut idx1: size_t = 0;
    idx1 = 0 as libc::c_int as size_t;
    while idx1 < num_clusters {
        let mut idx2: size_t = 0;
        idx2 = idx1.wrapping_add(1 as libc::c_int as libc::c_ulong);
        while idx2 < num_clusters {
            BrotliCompareAndPushToQueueCommand(
                out,
                cluster_size,
                *clusters.offset(idx1 as isize),
                *clusters.offset(idx2 as isize),
                max_num_pairs,
                &mut *pairs.offset(0 as libc::c_int as isize),
                &mut num_pairs,
            );
            idx2 = idx2.wrapping_add(1);
        }
        idx1 = idx1.wrapping_add(1);
    }
    while num_clusters > min_cluster_size {
        let mut best_idx1: uint32_t = 0;
        let mut best_idx2: uint32_t = 0;
        let mut i: size_t = 0;
        if (*pairs.offset(0 as libc::c_int as isize)).cost_diff >= cost_diff_threshold {
            cost_diff_threshold = 1e99f64;
            min_cluster_size = max_clusters;
        } else {
            best_idx1 = (*pairs.offset(0 as libc::c_int as isize)).idx1;
            best_idx2 = (*pairs.offset(0 as libc::c_int as isize)).idx2;
            HistogramAddHistogramCommand(
                &mut *out.offset(best_idx1 as isize),
                &mut *out.offset(best_idx2 as isize),
            );
            (*out.offset(best_idx1 as isize))
                .bit_cost_ = (*pairs.offset(0 as libc::c_int as isize)).cost_combo;
            let ref mut fresh8 = *cluster_size.offset(best_idx1 as isize);
            *fresh8 = (*fresh8 as libc::c_uint)
                .wrapping_add(*cluster_size.offset(best_idx2 as isize)) as uint32_t
                as uint32_t;
            i = 0 as libc::c_int as size_t;
            while i < symbols_size {
                if *symbols.offset(i as isize) == best_idx2 {
                    *symbols.offset(i as isize) = best_idx1;
                }
                i = i.wrapping_add(1);
            }
            i = 0 as libc::c_int as size_t;
            while i < num_clusters {
                if *clusters.offset(i as isize) == best_idx2 {
                    memmove(
                        &mut *clusters.offset(i as isize) as *mut uint32_t
                            as *mut libc::c_void,
                        &mut *clusters
                            .offset(
                                i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                            ) as *mut uint32_t as *const libc::c_void,
                        num_clusters
                            .wrapping_sub(i)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<uint32_t>() as libc::c_ulong,
                            ),
                    );
                    break;
                } else {
                    i = i.wrapping_add(1);
                }
            }
            num_clusters = num_clusters.wrapping_sub(1);
            let mut copy_to_idx = 0 as libc::c_int as size_t;
            i = 0 as libc::c_int as size_t;
            while i < num_pairs {
                let mut p: *mut HistogramPair = &mut *pairs.offset(i as isize)
                    as *mut HistogramPair;
                if !((*p).idx1 == best_idx1 || (*p).idx2 == best_idx1
                    || (*p).idx1 == best_idx2 || (*p).idx2 == best_idx2)
                {
                    if HistogramPairIsLess(
                        &mut *pairs.offset(0 as libc::c_int as isize),
                        p,
                    ) != 0
                    {
                        let mut front = *pairs.offset(0 as libc::c_int as isize);
                        *pairs.offset(0 as libc::c_int as isize) = *p;
                        *pairs.offset(copy_to_idx as isize) = front;
                    } else {
                        *pairs.offset(copy_to_idx as isize) = *p;
                    }
                    copy_to_idx = copy_to_idx.wrapping_add(1);
                }
                i = i.wrapping_add(1);
            }
            num_pairs = copy_to_idx;
            i = 0 as libc::c_int as size_t;
            while i < num_clusters {
                BrotliCompareAndPushToQueueCommand(
                    out,
                    cluster_size,
                    best_idx1,
                    *clusters.offset(i as isize),
                    max_num_pairs,
                    &mut *pairs.offset(0 as libc::c_int as isize),
                    &mut num_pairs,
                );
                i = i.wrapping_add(1);
            }
        }
    }
    return num_clusters;
}
#[no_mangle]
pub unsafe extern "C" fn BrotliHistogramBitCostDistanceCommand(
    mut histogram: *const HistogramCommand,
    mut candidate: *const HistogramCommand,
) -> libc::c_double {
    if (*histogram).total_count_ == 0 as libc::c_int as libc::c_ulong {
        return 0.0f64
    } else {
        let mut tmp = *histogram;
        HistogramAddHistogramCommand(&mut tmp, candidate);
        return BrotliPopulationCostCommand(&mut tmp) - (*candidate).bit_cost_;
    };
}
#[no_mangle]
pub unsafe extern "C" fn BrotliHistogramBitCostDistanceDistance(
    mut histogram: *const HistogramDistance,
    mut candidate: *const HistogramDistance,
) -> libc::c_double {
    if (*histogram).total_count_ == 0 as libc::c_int as libc::c_ulong {
        return 0.0f64
    } else {
        let mut tmp = *histogram;
        HistogramAddHistogramDistance(&mut tmp, candidate);
        return BrotliPopulationCostDistance(&mut tmp) - (*candidate).bit_cost_;
    };
}
#[no_mangle]
pub unsafe extern "C" fn BrotliHistogramBitCostDistanceLiteral(
    mut histogram: *const HistogramLiteral,
    mut candidate: *const HistogramLiteral,
) -> libc::c_double {
    if (*histogram).total_count_ == 0 as libc::c_int as libc::c_ulong {
        return 0.0f64
    } else {
        let mut tmp = *histogram;
        HistogramAddHistogramLiteral(&mut tmp, candidate);
        return BrotliPopulationCostLiteral(&mut tmp) - (*candidate).bit_cost_;
    };
}
#[no_mangle]
pub unsafe extern "C" fn BrotliHistogramRemapLiteral(
    mut in_0: *const HistogramLiteral,
    mut in_size: size_t,
    mut clusters: *const uint32_t,
    mut num_clusters: size_t,
    mut out: *mut HistogramLiteral,
    mut symbols: *mut uint32_t,
) {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < in_size {
        let mut best_out = if i == 0 as libc::c_int as libc::c_ulong {
            *symbols.offset(0 as libc::c_int as isize)
        } else {
            *symbols.offset(i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
        };
        let mut best_bits = BrotliHistogramBitCostDistanceLiteral(
            &*in_0.offset(i as isize),
            &mut *out.offset(best_out as isize),
        );
        let mut j: size_t = 0;
        j = 0 as libc::c_int as size_t;
        while j < num_clusters {
            let cur_bits = BrotliHistogramBitCostDistanceLiteral(
                &*in_0.offset(i as isize),
                &mut *out.offset(*clusters.offset(j as isize) as isize),
            );
            if cur_bits < best_bits {
                best_bits = cur_bits;
                best_out = *clusters.offset(j as isize);
            }
            j = j.wrapping_add(1);
        }
        *symbols.offset(i as isize) = best_out;
        i = i.wrapping_add(1);
    }
    i = 0 as libc::c_int as size_t;
    while i < num_clusters {
        HistogramClearLiteral(&mut *out.offset(*clusters.offset(i as isize) as isize));
        i = i.wrapping_add(1);
    }
    i = 0 as libc::c_int as size_t;
    while i < in_size {
        HistogramAddHistogramLiteral(
            &mut *out.offset(*symbols.offset(i as isize) as isize),
            &*in_0.offset(i as isize),
        );
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn BrotliHistogramRemapDistance(
    mut in_0: *const HistogramDistance,
    mut in_size: size_t,
    mut clusters: *const uint32_t,
    mut num_clusters: size_t,
    mut out: *mut HistogramDistance,
    mut symbols: *mut uint32_t,
) {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < in_size {
        let mut best_out = if i == 0 as libc::c_int as libc::c_ulong {
            *symbols.offset(0 as libc::c_int as isize)
        } else {
            *symbols.offset(i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
        };
        let mut best_bits = BrotliHistogramBitCostDistanceDistance(
            &*in_0.offset(i as isize),
            &mut *out.offset(best_out as isize),
        );
        let mut j: size_t = 0;
        j = 0 as libc::c_int as size_t;
        while j < num_clusters {
            let cur_bits = BrotliHistogramBitCostDistanceDistance(
                &*in_0.offset(i as isize),
                &mut *out.offset(*clusters.offset(j as isize) as isize),
            );
            if cur_bits < best_bits {
                best_bits = cur_bits;
                best_out = *clusters.offset(j as isize);
            }
            j = j.wrapping_add(1);
        }
        *symbols.offset(i as isize) = best_out;
        i = i.wrapping_add(1);
    }
    i = 0 as libc::c_int as size_t;
    while i < num_clusters {
        HistogramClearDistance(&mut *out.offset(*clusters.offset(i as isize) as isize));
        i = i.wrapping_add(1);
    }
    i = 0 as libc::c_int as size_t;
    while i < in_size {
        HistogramAddHistogramDistance(
            &mut *out.offset(*symbols.offset(i as isize) as isize),
            &*in_0.offset(i as isize),
        );
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn BrotliHistogramRemapCommand(
    mut in_0: *const HistogramCommand,
    mut in_size: size_t,
    mut clusters: *const uint32_t,
    mut num_clusters: size_t,
    mut out: *mut HistogramCommand,
    mut symbols: *mut uint32_t,
) {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < in_size {
        let mut best_out = if i == 0 as libc::c_int as libc::c_ulong {
            *symbols.offset(0 as libc::c_int as isize)
        } else {
            *symbols.offset(i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
        };
        let mut best_bits = BrotliHistogramBitCostDistanceCommand(
            &*in_0.offset(i as isize),
            &mut *out.offset(best_out as isize),
        );
        let mut j: size_t = 0;
        j = 0 as libc::c_int as size_t;
        while j < num_clusters {
            let cur_bits = BrotliHistogramBitCostDistanceCommand(
                &*in_0.offset(i as isize),
                &mut *out.offset(*clusters.offset(j as isize) as isize),
            );
            if cur_bits < best_bits {
                best_bits = cur_bits;
                best_out = *clusters.offset(j as isize);
            }
            j = j.wrapping_add(1);
        }
        *symbols.offset(i as isize) = best_out;
        i = i.wrapping_add(1);
    }
    i = 0 as libc::c_int as size_t;
    while i < num_clusters {
        HistogramClearCommand(&mut *out.offset(*clusters.offset(i as isize) as isize));
        i = i.wrapping_add(1);
    }
    i = 0 as libc::c_int as size_t;
    while i < in_size {
        HistogramAddHistogramCommand(
            &mut *out.offset(*symbols.offset(i as isize) as isize),
            &*in_0.offset(i as isize),
        );
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn BrotliHistogramReindexLiteral(
    mut m: *mut MemoryManager,
    mut out: *mut HistogramLiteral,
    mut symbols: *mut uint32_t,
    mut length: size_t,
) -> size_t {
    static mut kInvalidIndex: uint32_t = !(0 as libc::c_int as uint32_t);
    let mut new_index = if length > 0 as libc::c_int as libc::c_ulong {
        BrotliAllocate(
            m,
            length.wrapping_mul(::std::mem::size_of::<uint32_t>() as libc::c_ulong),
        ) as *mut uint32_t
    } else {
        0 as *mut uint32_t
    };
    let mut next_index: uint32_t = 0;
    let mut tmp = 0 as *mut HistogramLiteral;
    let mut i: size_t = 0;
    if 0 as libc::c_int != 0 || 0 as libc::c_int != 0 {
        return 0 as libc::c_int as size_t;
    }
    i = 0 as libc::c_int as size_t;
    while i < length {
        *new_index.offset(i as isize) = kInvalidIndex;
        i = i.wrapping_add(1);
    }
    next_index = 0 as libc::c_int as uint32_t;
    i = 0 as libc::c_int as size_t;
    while i < length {
        if *new_index.offset(*symbols.offset(i as isize) as isize) == kInvalidIndex {
            *new_index.offset(*symbols.offset(i as isize) as isize) = next_index;
            next_index = next_index.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    tmp = if next_index > 0 as libc::c_int as libc::c_uint {
        BrotliAllocate(
            m,
            (next_index as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<HistogramLiteral>() as libc::c_ulong),
        ) as *mut HistogramLiteral
    } else {
        0 as *mut HistogramLiteral
    };
    if 0 as libc::c_int != 0 || 0 as libc::c_int != 0 {
        return 0 as libc::c_int as size_t;
    }
    next_index = 0 as libc::c_int as uint32_t;
    i = 0 as libc::c_int as size_t;
    while i < length {
        if *new_index.offset(*symbols.offset(i as isize) as isize) == next_index {
            *tmp
                .offset(
                    next_index as isize,
                ) = *out.offset(*symbols.offset(i as isize) as isize);
            next_index = next_index.wrapping_add(1);
        }
        *symbols
            .offset(
                i as isize,
            ) = *new_index.offset(*symbols.offset(i as isize) as isize);
        i = i.wrapping_add(1);
    }
    BrotliFree(m, new_index as *mut libc::c_void);
    new_index = 0 as *mut uint32_t;
    i = 0 as libc::c_int as size_t;
    while i < next_index as libc::c_ulong {
        *out.offset(i as isize) = *tmp.offset(i as isize);
        i = i.wrapping_add(1);
    }
    BrotliFree(m, tmp as *mut libc::c_void);
    tmp = 0 as *mut HistogramLiteral;
    return next_index as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn BrotliHistogramReindexDistance(
    mut m: *mut MemoryManager,
    mut out: *mut HistogramDistance,
    mut symbols: *mut uint32_t,
    mut length: size_t,
) -> size_t {
    static mut kInvalidIndex: uint32_t = !(0 as libc::c_int as uint32_t);
    let mut new_index = if length > 0 as libc::c_int as libc::c_ulong {
        BrotliAllocate(
            m,
            length.wrapping_mul(::std::mem::size_of::<uint32_t>() as libc::c_ulong),
        ) as *mut uint32_t
    } else {
        0 as *mut uint32_t
    };
    let mut next_index: uint32_t = 0;
    let mut tmp = 0 as *mut HistogramDistance;
    let mut i: size_t = 0;
    if 0 as libc::c_int != 0 || 0 as libc::c_int != 0 {
        return 0 as libc::c_int as size_t;
    }
    i = 0 as libc::c_int as size_t;
    while i < length {
        *new_index.offset(i as isize) = kInvalidIndex;
        i = i.wrapping_add(1);
    }
    next_index = 0 as libc::c_int as uint32_t;
    i = 0 as libc::c_int as size_t;
    while i < length {
        if *new_index.offset(*symbols.offset(i as isize) as isize) == kInvalidIndex {
            *new_index.offset(*symbols.offset(i as isize) as isize) = next_index;
            next_index = next_index.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    tmp = if next_index > 0 as libc::c_int as libc::c_uint {
        BrotliAllocate(
            m,
            (next_index as libc::c_ulong)
                .wrapping_mul(
                    ::std::mem::size_of::<HistogramDistance>() as libc::c_ulong,
                ),
        ) as *mut HistogramDistance
    } else {
        0 as *mut HistogramDistance
    };
    if 0 as libc::c_int != 0 || 0 as libc::c_int != 0 {
        return 0 as libc::c_int as size_t;
    }
    next_index = 0 as libc::c_int as uint32_t;
    i = 0 as libc::c_int as size_t;
    while i < length {
        if *new_index.offset(*symbols.offset(i as isize) as isize) == next_index {
            *tmp
                .offset(
                    next_index as isize,
                ) = *out.offset(*symbols.offset(i as isize) as isize);
            next_index = next_index.wrapping_add(1);
        }
        *symbols
            .offset(
                i as isize,
            ) = *new_index.offset(*symbols.offset(i as isize) as isize);
        i = i.wrapping_add(1);
    }
    BrotliFree(m, new_index as *mut libc::c_void);
    new_index = 0 as *mut uint32_t;
    i = 0 as libc::c_int as size_t;
    while i < next_index as libc::c_ulong {
        *out.offset(i as isize) = *tmp.offset(i as isize);
        i = i.wrapping_add(1);
    }
    BrotliFree(m, tmp as *mut libc::c_void);
    tmp = 0 as *mut HistogramDistance;
    return next_index as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn BrotliHistogramReindexCommand(
    mut m: *mut MemoryManager,
    mut out: *mut HistogramCommand,
    mut symbols: *mut uint32_t,
    mut length: size_t,
) -> size_t {
    static mut kInvalidIndex: uint32_t = !(0 as libc::c_int as uint32_t);
    let mut new_index = if length > 0 as libc::c_int as libc::c_ulong {
        BrotliAllocate(
            m,
            length.wrapping_mul(::std::mem::size_of::<uint32_t>() as libc::c_ulong),
        ) as *mut uint32_t
    } else {
        0 as *mut uint32_t
    };
    let mut next_index: uint32_t = 0;
    let mut tmp = 0 as *mut HistogramCommand;
    let mut i: size_t = 0;
    if 0 as libc::c_int != 0 || 0 as libc::c_int != 0 {
        return 0 as libc::c_int as size_t;
    }
    i = 0 as libc::c_int as size_t;
    while i < length {
        *new_index.offset(i as isize) = kInvalidIndex;
        i = i.wrapping_add(1);
    }
    next_index = 0 as libc::c_int as uint32_t;
    i = 0 as libc::c_int as size_t;
    while i < length {
        if *new_index.offset(*symbols.offset(i as isize) as isize) == kInvalidIndex {
            *new_index.offset(*symbols.offset(i as isize) as isize) = next_index;
            next_index = next_index.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    tmp = if next_index > 0 as libc::c_int as libc::c_uint {
        BrotliAllocate(
            m,
            (next_index as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<HistogramCommand>() as libc::c_ulong),
        ) as *mut HistogramCommand
    } else {
        0 as *mut HistogramCommand
    };
    if 0 as libc::c_int != 0 || 0 as libc::c_int != 0 {
        return 0 as libc::c_int as size_t;
    }
    next_index = 0 as libc::c_int as uint32_t;
    i = 0 as libc::c_int as size_t;
    while i < length {
        if *new_index.offset(*symbols.offset(i as isize) as isize) == next_index {
            *tmp
                .offset(
                    next_index as isize,
                ) = *out.offset(*symbols.offset(i as isize) as isize);
            next_index = next_index.wrapping_add(1);
        }
        *symbols
            .offset(
                i as isize,
            ) = *new_index.offset(*symbols.offset(i as isize) as isize);
        i = i.wrapping_add(1);
    }
    BrotliFree(m, new_index as *mut libc::c_void);
    new_index = 0 as *mut uint32_t;
    i = 0 as libc::c_int as size_t;
    while i < next_index as libc::c_ulong {
        *out.offset(i as isize) = *tmp.offset(i as isize);
        i = i.wrapping_add(1);
    }
    BrotliFree(m, tmp as *mut libc::c_void);
    tmp = 0 as *mut HistogramCommand;
    return next_index as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn BrotliClusterHistogramsDistance(
    mut m: *mut MemoryManager,
    mut in_0: *const HistogramDistance,
    in_size: size_t,
    mut max_histograms: size_t,
    mut out: *mut HistogramDistance,
    mut out_size: *mut size_t,
    mut histogram_symbols: *mut uint32_t,
) {
    let mut cluster_size = if in_size > 0 as libc::c_int as libc::c_ulong {
        BrotliAllocate(
            m,
            in_size.wrapping_mul(::std::mem::size_of::<uint32_t>() as libc::c_ulong),
        ) as *mut uint32_t
    } else {
        0 as *mut uint32_t
    };
    let mut clusters = if in_size > 0 as libc::c_int as libc::c_ulong {
        BrotliAllocate(
            m,
            in_size.wrapping_mul(::std::mem::size_of::<uint32_t>() as libc::c_ulong),
        ) as *mut uint32_t
    } else {
        0 as *mut uint32_t
    };
    let mut num_clusters = 0 as libc::c_int as size_t;
    let max_input_histograms = 64 as libc::c_int as size_t;
    let mut pairs_capacity = max_input_histograms
        .wrapping_mul(max_input_histograms)
        .wrapping_div(2 as libc::c_int as libc::c_ulong);
    let mut pairs = if pairs_capacity.wrapping_add(1 as libc::c_int as libc::c_ulong)
        > 0 as libc::c_int as libc::c_ulong
    {
        BrotliAllocate(
            m,
            pairs_capacity
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<HistogramPair>() as libc::c_ulong),
        ) as *mut HistogramPair
    } else {
        0 as *mut HistogramPair
    };
    let mut i: size_t = 0;
    if 0 as libc::c_int != 0 || 0 as libc::c_int != 0 || 0 as libc::c_int != 0
        || 0 as libc::c_int != 0
    {
        return;
    }
    i = 0 as libc::c_int as size_t;
    while i < in_size {
        *cluster_size.offset(i as isize) = 1 as libc::c_int as uint32_t;
        i = i.wrapping_add(1);
    }
    i = 0 as libc::c_int as size_t;
    while i < in_size {
        *out.offset(i as isize) = *in_0.offset(i as isize);
        (*out.offset(i as isize))
            .bit_cost_ = BrotliPopulationCostDistance(&*in_0.offset(i as isize));
        *histogram_symbols.offset(i as isize) = i as uint32_t;
        i = i.wrapping_add(1);
    }
    i = 0 as libc::c_int as size_t;
    while i < in_size {
        let mut num_to_combine = brotli_min_size_t(
            in_size.wrapping_sub(i),
            max_input_histograms,
        );
        let mut num_new_clusters: size_t = 0;
        let mut j: size_t = 0;
        j = 0 as libc::c_int as size_t;
        while j < num_to_combine {
            *clusters
                .offset(
                    num_clusters.wrapping_add(j) as isize,
                ) = i.wrapping_add(j) as uint32_t;
            j = j.wrapping_add(1);
        }
        num_new_clusters = BrotliHistogramCombineDistance(
            out,
            cluster_size,
            &mut *histogram_symbols.offset(i as isize),
            &mut *clusters.offset(num_clusters as isize),
            pairs,
            num_to_combine,
            num_to_combine,
            max_histograms,
            pairs_capacity,
        );
        num_clusters = (num_clusters as libc::c_ulong).wrapping_add(num_new_clusters)
            as size_t as size_t;
        i = (i as libc::c_ulong).wrapping_add(max_input_histograms) as size_t as size_t;
    }
    let mut max_num_pairs = brotli_min_size_t(
        (64 as libc::c_int as libc::c_ulong).wrapping_mul(num_clusters),
        num_clusters
            .wrapping_div(2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(num_clusters),
    );
    if pairs_capacity < max_num_pairs.wrapping_add(1 as libc::c_int as libc::c_ulong) {
        let mut _new_size = if pairs_capacity == 0 as libc::c_int as libc::c_ulong {
            max_num_pairs.wrapping_add(1 as libc::c_int as libc::c_ulong)
        } else {
            pairs_capacity
        };
        let mut new_array = 0 as *mut HistogramPair;
        while _new_size < max_num_pairs.wrapping_add(1 as libc::c_int as libc::c_ulong) {
            _new_size = (_new_size as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
        }
        new_array = if _new_size > 0 as libc::c_int as libc::c_ulong {
            BrotliAllocate(
                m,
                _new_size
                    .wrapping_mul(
                        ::std::mem::size_of::<HistogramPair>() as libc::c_ulong,
                    ),
            ) as *mut HistogramPair
        } else {
            0 as *mut HistogramPair
        };
        if 0 as libc::c_int == 0 && 0 as libc::c_int == 0
            && pairs_capacity != 0 as libc::c_int as libc::c_ulong
        {
            memcpy(
                new_array as *mut libc::c_void,
                pairs as *const libc::c_void,
                pairs_capacity
                    .wrapping_mul(
                        ::std::mem::size_of::<HistogramPair>() as libc::c_ulong,
                    ),
            );
        }
        BrotliFree(m, pairs as *mut libc::c_void);
        pairs = 0 as *mut HistogramPair;
        pairs = new_array;
        pairs_capacity = _new_size;
    }
    if 0 as libc::c_int != 0 {
        return;
    }
    num_clusters = BrotliHistogramCombineDistance(
        out,
        cluster_size,
        histogram_symbols,
        clusters,
        pairs,
        num_clusters,
        in_size,
        max_histograms,
        max_num_pairs,
    );
    BrotliFree(m, pairs as *mut libc::c_void);
    pairs = 0 as *mut HistogramPair;
    BrotliFree(m, cluster_size as *mut libc::c_void);
    cluster_size = 0 as *mut uint32_t;
    BrotliHistogramRemapDistance(
        in_0,
        in_size,
        clusters,
        num_clusters,
        out,
        histogram_symbols,
    );
    BrotliFree(m, clusters as *mut libc::c_void);
    clusters = 0 as *mut uint32_t;
    *out_size = BrotliHistogramReindexDistance(m, out, histogram_symbols, in_size);
    if 0 as libc::c_int != 0 {
        return;
    }
}
#[no_mangle]
pub unsafe extern "C" fn BrotliClusterHistogramsCommand(
    mut m: *mut MemoryManager,
    mut in_0: *const HistogramCommand,
    in_size: size_t,
    mut max_histograms: size_t,
    mut out: *mut HistogramCommand,
    mut out_size: *mut size_t,
    mut histogram_symbols: *mut uint32_t,
) {
    let mut cluster_size = if in_size > 0 as libc::c_int as libc::c_ulong {
        BrotliAllocate(
            m,
            in_size.wrapping_mul(::std::mem::size_of::<uint32_t>() as libc::c_ulong),
        ) as *mut uint32_t
    } else {
        0 as *mut uint32_t
    };
    let mut clusters = if in_size > 0 as libc::c_int as libc::c_ulong {
        BrotliAllocate(
            m,
            in_size.wrapping_mul(::std::mem::size_of::<uint32_t>() as libc::c_ulong),
        ) as *mut uint32_t
    } else {
        0 as *mut uint32_t
    };
    let mut num_clusters = 0 as libc::c_int as size_t;
    let max_input_histograms = 64 as libc::c_int as size_t;
    let mut pairs_capacity = max_input_histograms
        .wrapping_mul(max_input_histograms)
        .wrapping_div(2 as libc::c_int as libc::c_ulong);
    let mut pairs = if pairs_capacity.wrapping_add(1 as libc::c_int as libc::c_ulong)
        > 0 as libc::c_int as libc::c_ulong
    {
        BrotliAllocate(
            m,
            pairs_capacity
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<HistogramPair>() as libc::c_ulong),
        ) as *mut HistogramPair
    } else {
        0 as *mut HistogramPair
    };
    let mut i: size_t = 0;
    if 0 as libc::c_int != 0 || 0 as libc::c_int != 0 || 0 as libc::c_int != 0
        || 0 as libc::c_int != 0
    {
        return;
    }
    i = 0 as libc::c_int as size_t;
    while i < in_size {
        *cluster_size.offset(i as isize) = 1 as libc::c_int as uint32_t;
        i = i.wrapping_add(1);
    }
    i = 0 as libc::c_int as size_t;
    while i < in_size {
        *out.offset(i as isize) = *in_0.offset(i as isize);
        (*out.offset(i as isize))
            .bit_cost_ = BrotliPopulationCostCommand(&*in_0.offset(i as isize));
        *histogram_symbols.offset(i as isize) = i as uint32_t;
        i = i.wrapping_add(1);
    }
    i = 0 as libc::c_int as size_t;
    while i < in_size {
        let mut num_to_combine = brotli_min_size_t(
            in_size.wrapping_sub(i),
            max_input_histograms,
        );
        let mut num_new_clusters: size_t = 0;
        let mut j: size_t = 0;
        j = 0 as libc::c_int as size_t;
        while j < num_to_combine {
            *clusters
                .offset(
                    num_clusters.wrapping_add(j) as isize,
                ) = i.wrapping_add(j) as uint32_t;
            j = j.wrapping_add(1);
        }
        num_new_clusters = BrotliHistogramCombineCommand(
            out,
            cluster_size,
            &mut *histogram_symbols.offset(i as isize),
            &mut *clusters.offset(num_clusters as isize),
            pairs,
            num_to_combine,
            num_to_combine,
            max_histograms,
            pairs_capacity,
        );
        num_clusters = (num_clusters as libc::c_ulong).wrapping_add(num_new_clusters)
            as size_t as size_t;
        i = (i as libc::c_ulong).wrapping_add(max_input_histograms) as size_t as size_t;
    }
    let mut max_num_pairs = brotli_min_size_t(
        (64 as libc::c_int as libc::c_ulong).wrapping_mul(num_clusters),
        num_clusters
            .wrapping_div(2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(num_clusters),
    );
    if pairs_capacity < max_num_pairs.wrapping_add(1 as libc::c_int as libc::c_ulong) {
        let mut _new_size = if pairs_capacity == 0 as libc::c_int as libc::c_ulong {
            max_num_pairs.wrapping_add(1 as libc::c_int as libc::c_ulong)
        } else {
            pairs_capacity
        };
        let mut new_array = 0 as *mut HistogramPair;
        while _new_size < max_num_pairs.wrapping_add(1 as libc::c_int as libc::c_ulong) {
            _new_size = (_new_size as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
        }
        new_array = if _new_size > 0 as libc::c_int as libc::c_ulong {
            BrotliAllocate(
                m,
                _new_size
                    .wrapping_mul(
                        ::std::mem::size_of::<HistogramPair>() as libc::c_ulong,
                    ),
            ) as *mut HistogramPair
        } else {
            0 as *mut HistogramPair
        };
        if 0 as libc::c_int == 0 && 0 as libc::c_int == 0
            && pairs_capacity != 0 as libc::c_int as libc::c_ulong
        {
            memcpy(
                new_array as *mut libc::c_void,
                pairs as *const libc::c_void,
                pairs_capacity
                    .wrapping_mul(
                        ::std::mem::size_of::<HistogramPair>() as libc::c_ulong,
                    ),
            );
        }
        BrotliFree(m, pairs as *mut libc::c_void);
        pairs = 0 as *mut HistogramPair;
        pairs = new_array;
        pairs_capacity = _new_size;
    }
    if 0 as libc::c_int != 0 {
        return;
    }
    num_clusters = BrotliHistogramCombineCommand(
        out,
        cluster_size,
        histogram_symbols,
        clusters,
        pairs,
        num_clusters,
        in_size,
        max_histograms,
        max_num_pairs,
    );
    BrotliFree(m, pairs as *mut libc::c_void);
    pairs = 0 as *mut HistogramPair;
    BrotliFree(m, cluster_size as *mut libc::c_void);
    cluster_size = 0 as *mut uint32_t;
    BrotliHistogramRemapCommand(
        in_0,
        in_size,
        clusters,
        num_clusters,
        out,
        histogram_symbols,
    );
    BrotliFree(m, clusters as *mut libc::c_void);
    clusters = 0 as *mut uint32_t;
    *out_size = BrotliHistogramReindexCommand(m, out, histogram_symbols, in_size);
    if 0 as libc::c_int != 0 {
        return;
    }
}
#[no_mangle]
pub unsafe extern "C" fn BrotliClusterHistogramsLiteral(
    mut m: *mut MemoryManager,
    mut in_0: *const HistogramLiteral,
    in_size: size_t,
    mut max_histograms: size_t,
    mut out: *mut HistogramLiteral,
    mut out_size: *mut size_t,
    mut histogram_symbols: *mut uint32_t,
) {
    let mut cluster_size = if in_size > 0 as libc::c_int as libc::c_ulong {
        BrotliAllocate(
            m,
            in_size.wrapping_mul(::std::mem::size_of::<uint32_t>() as libc::c_ulong),
        ) as *mut uint32_t
    } else {
        0 as *mut uint32_t
    };
    let mut clusters = if in_size > 0 as libc::c_int as libc::c_ulong {
        BrotliAllocate(
            m,
            in_size.wrapping_mul(::std::mem::size_of::<uint32_t>() as libc::c_ulong),
        ) as *mut uint32_t
    } else {
        0 as *mut uint32_t
    };
    let mut num_clusters = 0 as libc::c_int as size_t;
    let max_input_histograms = 64 as libc::c_int as size_t;
    let mut pairs_capacity = max_input_histograms
        .wrapping_mul(max_input_histograms)
        .wrapping_div(2 as libc::c_int as libc::c_ulong);
    let mut pairs = if pairs_capacity.wrapping_add(1 as libc::c_int as libc::c_ulong)
        > 0 as libc::c_int as libc::c_ulong
    {
        BrotliAllocate(
            m,
            pairs_capacity
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<HistogramPair>() as libc::c_ulong),
        ) as *mut HistogramPair
    } else {
        0 as *mut HistogramPair
    };
    let mut i: size_t = 0;
    if 0 as libc::c_int != 0 || 0 as libc::c_int != 0 || 0 as libc::c_int != 0
        || 0 as libc::c_int != 0
    {
        return;
    }
    i = 0 as libc::c_int as size_t;
    while i < in_size {
        *cluster_size.offset(i as isize) = 1 as libc::c_int as uint32_t;
        i = i.wrapping_add(1);
    }
    i = 0 as libc::c_int as size_t;
    while i < in_size {
        *out.offset(i as isize) = *in_0.offset(i as isize);
        (*out.offset(i as isize))
            .bit_cost_ = BrotliPopulationCostLiteral(&*in_0.offset(i as isize));
        *histogram_symbols.offset(i as isize) = i as uint32_t;
        i = i.wrapping_add(1);
    }
    i = 0 as libc::c_int as size_t;
    while i < in_size {
        let mut num_to_combine = brotli_min_size_t(
            in_size.wrapping_sub(i),
            max_input_histograms,
        );
        let mut num_new_clusters: size_t = 0;
        let mut j: size_t = 0;
        j = 0 as libc::c_int as size_t;
        while j < num_to_combine {
            *clusters
                .offset(
                    num_clusters.wrapping_add(j) as isize,
                ) = i.wrapping_add(j) as uint32_t;
            j = j.wrapping_add(1);
        }
        num_new_clusters = BrotliHistogramCombineLiteral(
            out,
            cluster_size,
            &mut *histogram_symbols.offset(i as isize),
            &mut *clusters.offset(num_clusters as isize),
            pairs,
            num_to_combine,
            num_to_combine,
            max_histograms,
            pairs_capacity,
        );
        num_clusters = (num_clusters as libc::c_ulong).wrapping_add(num_new_clusters)
            as size_t as size_t;
        i = (i as libc::c_ulong).wrapping_add(max_input_histograms) as size_t as size_t;
    }
    let mut max_num_pairs = brotli_min_size_t(
        (64 as libc::c_int as libc::c_ulong).wrapping_mul(num_clusters),
        num_clusters
            .wrapping_div(2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(num_clusters),
    );
    if pairs_capacity < max_num_pairs.wrapping_add(1 as libc::c_int as libc::c_ulong) {
        let mut _new_size = if pairs_capacity == 0 as libc::c_int as libc::c_ulong {
            max_num_pairs.wrapping_add(1 as libc::c_int as libc::c_ulong)
        } else {
            pairs_capacity
        };
        let mut new_array = 0 as *mut HistogramPair;
        while _new_size < max_num_pairs.wrapping_add(1 as libc::c_int as libc::c_ulong) {
            _new_size = (_new_size as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
        }
        new_array = if _new_size > 0 as libc::c_int as libc::c_ulong {
            BrotliAllocate(
                m,
                _new_size
                    .wrapping_mul(
                        ::std::mem::size_of::<HistogramPair>() as libc::c_ulong,
                    ),
            ) as *mut HistogramPair
        } else {
            0 as *mut HistogramPair
        };
        if 0 as libc::c_int == 0 && 0 as libc::c_int == 0
            && pairs_capacity != 0 as libc::c_int as libc::c_ulong
        {
            memcpy(
                new_array as *mut libc::c_void,
                pairs as *const libc::c_void,
                pairs_capacity
                    .wrapping_mul(
                        ::std::mem::size_of::<HistogramPair>() as libc::c_ulong,
                    ),
            );
        }
        BrotliFree(m, pairs as *mut libc::c_void);
        pairs = 0 as *mut HistogramPair;
        pairs = new_array;
        pairs_capacity = _new_size;
    }
    if 0 as libc::c_int != 0 {
        return;
    }
    num_clusters = BrotliHistogramCombineLiteral(
        out,
        cluster_size,
        histogram_symbols,
        clusters,
        pairs,
        num_clusters,
        in_size,
        max_histograms,
        max_num_pairs,
    );
    BrotliFree(m, pairs as *mut libc::c_void);
    pairs = 0 as *mut HistogramPair;
    BrotliFree(m, cluster_size as *mut libc::c_void);
    cluster_size = 0 as *mut uint32_t;
    BrotliHistogramRemapLiteral(
        in_0,
        in_size,
        clusters,
        num_clusters,
        out,
        histogram_symbols,
    );
    BrotliFree(m, clusters as *mut libc::c_void);
    clusters = 0 as *mut uint32_t;
    *out_size = BrotliHistogramReindexLiteral(m, out, histogram_symbols, in_size);
    if 0 as libc::c_int != 0 {
        return;
    }
}
