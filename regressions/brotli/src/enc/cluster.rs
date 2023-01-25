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

struct ErasedByPreprocessor66 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor67 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor68 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor69 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor70 { dummy: () }
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
        return crate::src::enc::cluster::kBrotliLog2Table[v as usize];
    }
    return log2(v as libc::c_double);
}
#[inline(always)]
unsafe extern "C" fn HistogramClearLiteral(mut self_0: *mut crate::src::enc::bit_cost::HistogramLiteral) {
    memset(
        (*self_0).data_.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[uint32_t; 256]>() as libc::c_ulong,
    );
    (*self_0).total_count_= 0 as libc::c_int as size_t;
    (*self_0).bit_cost_= ::std::f64::INFINITY;
}
#[inline(always)]
unsafe extern "C" fn HistogramAddHistogramLiteral(
    mut self_0: *mut crate::src::enc::bit_cost::HistogramLiteral,
    mut v: *const crate::src::enc::bit_cost::HistogramLiteral,
) {
    let mut i: size_t = 0;
    (*self_0).total_count_= ((*self_0).total_count_ as libc::c_ulong).wrapping_add((*v).total_count_) as size_t
        as size_t;
    i= 0 as libc::c_int as size_t;
    while i < 256 as libc::c_int as libc::c_ulong {
        (*self_0).data_[i as usize]= ((*self_0).data_[i as usize] as libc::c_uint).wrapping_add((*v).data_[i as usize])
            as uint32_t as uint32_t;
        i= i.wrapping_add(1);
    }
}
#[inline(always)]
unsafe extern "C" fn HistogramClearCommand(mut self_0: *mut crate::src::enc::bit_cost::HistogramCommand) {
    memset(
        (*self_0).data_.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[uint32_t; 704]>() as libc::c_ulong,
    );
    (*self_0).total_count_= 0 as libc::c_int as size_t;
    (*self_0).bit_cost_= ::std::f64::INFINITY;
}
#[inline(always)]
unsafe extern "C" fn HistogramAddHistogramCommand(
    mut self_0: *mut crate::src::enc::bit_cost::HistogramCommand,
    mut v: *const crate::src::enc::bit_cost::HistogramCommand,
) {
    let mut i: size_t = 0;
    (*self_0).total_count_= ((*self_0).total_count_ as libc::c_ulong).wrapping_add((*v).total_count_) as size_t
        as size_t;
    i= 0 as libc::c_int as size_t;
    while i < 704 as libc::c_int as libc::c_ulong {
        (*self_0).data_[i as usize]= ((*self_0).data_[i as usize] as libc::c_uint).wrapping_add((*v).data_[i as usize])
            as uint32_t as uint32_t;
        i= i.wrapping_add(1);
    }
}
#[inline(always)]
unsafe extern "C" fn HistogramClearDistance(mut self_0: *mut crate::src::enc::bit_cost::HistogramDistance) {
    memset(
        (*self_0).data_.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[uint32_t; 544]>() as libc::c_ulong,
    );
    (*self_0).total_count_= 0 as libc::c_int as size_t;
    (*self_0).bit_cost_= ::std::f64::INFINITY;
}
#[inline(always)]
unsafe extern "C" fn HistogramAddHistogramDistance(
    mut self_0: *mut crate::src::enc::bit_cost::HistogramDistance,
    mut v: *const crate::src::enc::bit_cost::HistogramDistance,
) {
    let mut i: size_t = 0;
    (*self_0).total_count_= ((*self_0).total_count_ as libc::c_ulong).wrapping_add((*v).total_count_) as size_t
        as size_t;
    i= 0 as libc::c_int as size_t;
    while i < 544 as libc::c_int as libc::c_ulong {
        (*self_0).data_[i as usize]= ((*self_0).data_[i as usize] as libc::c_uint).wrapping_add((*v).data_[i as usize])
            as uint32_t as uint32_t;
        i= i.wrapping_add(1);
    }
}
#[inline(always)]
unsafe extern "C" fn HistogramPairIsLess(
    mut p1: *const crate::src::enc::block_splitter::HistogramPair,
    mut p2: *const crate::src::enc::block_splitter::HistogramPair,
) -> libc::c_int {
    if (*p1).cost_diff != (*p2).cost_diff {
        return if (*p1).cost_diff > (*p2).cost_diff {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        };
    }
    return if (*p1).idx2.wrapping_sub((*p1).idx1)
        > (*p2).idx2.wrapping_sub((*p2).idx1)
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
    mut out: *const crate::src::enc::bit_cost::HistogramLiteral,
    mut cluster_size: *const uint32_t,
    mut idx1: uint32_t,
    mut idx2: uint32_t,
    mut max_num_pairs: size_t,
    mut pairs: *mut crate::src::enc::block_splitter::HistogramPair,
    mut num_pairs: Option<&mut size_t>,
) {
    let mut is_good_pair = 0 as libc::c_int;
    let mut p = crate::src::enc::block_splitter::HistogramPair {
        idx1: 0,
        idx2: 0,
        cost_combo: 0.,
        cost_diff: 0.,
    };
    p.idx2= 0 as libc::c_int as uint32_t;
    p.idx1= p.idx2;
    p.cost_combo= 0 as libc::c_int as libc::c_double;
    p.cost_diff= p.cost_combo;
    if idx1 == idx2 {
        return;
    }
    if idx2 < idx1 {
        let mut t = idx2;
        idx2= idx1;
        idx1= t;
    }
    p.idx1= idx1;
    p.idx2= idx2;
    p.cost_diff= 0.5f64
        * ClusterCostDiff(
            *cluster_size.offset(idx1 as isize) as size_t,
            *cluster_size.offset(idx2 as isize) as size_t,
        );
    p.cost_diff-= (*out.offset(idx1 as isize)).bit_cost_;
    p.cost_diff-= (*out.offset(idx2 as isize)).bit_cost_;
    if (*out.offset(idx1 as isize)).total_count_ == 0 as libc::c_int as libc::c_ulong {
        p.cost_combo= (*out.offset(idx2 as isize)).bit_cost_;
        is_good_pair= 1 as libc::c_int;
    } else if (*out.offset(idx2 as isize)).total_count_
        == 0 as libc::c_int as libc::c_ulong
    {
        p.cost_combo= (*out.offset(idx1 as isize)).bit_cost_;
        is_good_pair= 1 as libc::c_int;
    } else {
        let mut threshold = if (*num_pairs.as_deref().unwrap()) == 0 as libc::c_int as libc::c_ulong {
            1e99f64
        } else {
            brotli_max_double(
                0.0f64,
                (*pairs.offset(0 as libc::c_int as isize)).cost_diff,
            )
        };
        let mut combo = *out.offset(idx1 as isize);
        let mut cost_combo: libc::c_double = 0.;
        HistogramAddHistogramLiteral(core::ptr::addr_of_mut!(combo), &*out.offset(idx2 as isize));
        cost_combo= crate::src::enc::bit_cost::BrotliPopulationCostLiteral(core::ptr::addr_of!(combo));
        if cost_combo < threshold - p.cost_diff {
            p.cost_combo= cost_combo;
            is_good_pair= 1 as libc::c_int;
        }
    }
    if is_good_pair != 0 {
        p.cost_diff+= p.cost_combo;
        if (*num_pairs.as_deref().unwrap()) > 0 as libc::c_int as libc::c_ulong
            && HistogramPairIsLess(core::ptr::addr_of_mut!(*pairs.offset(0 as libc::c_int as isize)), core::ptr::addr_of!(p))
                != 0
        {
            if (*num_pairs.as_deref().unwrap()) < max_num_pairs {
                *pairs
                    .offset(
                        (*num_pairs.as_deref().unwrap()) as isize,
                    ) = *pairs.offset(0 as libc::c_int as isize);
                *num_pairs.as_deref_mut().unwrap()= (*num_pairs.as_deref().unwrap()).wrapping_add(1);
            }
            *pairs.offset(0 as libc::c_int as isize) = p;
        } else if (*num_pairs.as_deref().unwrap()) < max_num_pairs {
            *pairs.offset((*num_pairs.as_deref().unwrap()) as isize) = p;
            *num_pairs.as_deref_mut().unwrap()= (*num_pairs.as_deref().unwrap()).wrapping_add(1);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn BrotliCompareAndPushToQueueCommand(
    mut out: *const crate::src::enc::bit_cost::HistogramCommand,
    mut cluster_size: *const uint32_t,
    mut idx1: uint32_t,
    mut idx2: uint32_t,
    mut max_num_pairs: size_t,
    mut pairs: *mut crate::src::enc::block_splitter::HistogramPair,
    mut num_pairs: Option<&mut size_t>,
) {
    let mut is_good_pair = 0 as libc::c_int;
    let mut p = crate::src::enc::block_splitter::HistogramPair {
        idx1: 0,
        idx2: 0,
        cost_combo: 0.,
        cost_diff: 0.,
    };
    p.idx2= 0 as libc::c_int as uint32_t;
    p.idx1= p.idx2;
    p.cost_combo= 0 as libc::c_int as libc::c_double;
    p.cost_diff= p.cost_combo;
    if idx1 == idx2 {
        return;
    }
    if idx2 < idx1 {
        let mut t = idx2;
        idx2= idx1;
        idx1= t;
    }
    p.idx1= idx1;
    p.idx2= idx2;
    p.cost_diff= 0.5f64
        * ClusterCostDiff(
            *cluster_size.offset(idx1 as isize) as size_t,
            *cluster_size.offset(idx2 as isize) as size_t,
        );
    p.cost_diff-= (*out.offset(idx1 as isize)).bit_cost_;
    p.cost_diff-= (*out.offset(idx2 as isize)).bit_cost_;
    if (*out.offset(idx1 as isize)).total_count_ == 0 as libc::c_int as libc::c_ulong {
        p.cost_combo= (*out.offset(idx2 as isize)).bit_cost_;
        is_good_pair= 1 as libc::c_int;
    } else if (*out.offset(idx2 as isize)).total_count_
        == 0 as libc::c_int as libc::c_ulong
    {
        p.cost_combo= (*out.offset(idx1 as isize)).bit_cost_;
        is_good_pair= 1 as libc::c_int;
    } else {
        let mut threshold = if (*num_pairs.as_deref().unwrap()) == 0 as libc::c_int as libc::c_ulong {
            1e99f64
        } else {
            brotli_max_double(
                0.0f64,
                (*pairs.offset(0 as libc::c_int as isize)).cost_diff,
            )
        };
        let mut combo = *out.offset(idx1 as isize);
        let mut cost_combo: libc::c_double = 0.;
        HistogramAddHistogramCommand(core::ptr::addr_of_mut!(combo), &*out.offset(idx2 as isize));
        cost_combo= crate::src::enc::bit_cost::BrotliPopulationCostCommand(core::ptr::addr_of!(combo));
        if cost_combo < threshold - p.cost_diff {
            p.cost_combo= cost_combo;
            is_good_pair= 1 as libc::c_int;
        }
    }
    if is_good_pair != 0 {
        p.cost_diff+= p.cost_combo;
        if (*num_pairs.as_deref().unwrap()) > 0 as libc::c_int as libc::c_ulong
            && HistogramPairIsLess(core::ptr::addr_of_mut!(*pairs.offset(0 as libc::c_int as isize)), core::ptr::addr_of!(p))
                != 0
        {
            if (*num_pairs.as_deref().unwrap()) < max_num_pairs {
                *pairs
                    .offset(
                        (*num_pairs.as_deref().unwrap()) as isize,
                    ) = *pairs.offset(0 as libc::c_int as isize);
                *num_pairs.as_deref_mut().unwrap()= (*num_pairs.as_deref().unwrap()).wrapping_add(1);
            }
            *pairs.offset(0 as libc::c_int as isize) = p;
        } else if (*num_pairs.as_deref().unwrap()) < max_num_pairs {
            *pairs.offset((*num_pairs.as_deref().unwrap()) as isize) = p;
            *num_pairs.as_deref_mut().unwrap()= (*num_pairs.as_deref().unwrap()).wrapping_add(1);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn BrotliCompareAndPushToQueueDistance(
    mut out: *const crate::src::enc::bit_cost::HistogramDistance,
    mut cluster_size: *const uint32_t,
    mut idx1: uint32_t,
    mut idx2: uint32_t,
    mut max_num_pairs: size_t,
    mut pairs: *mut crate::src::enc::block_splitter::HistogramPair,
    mut num_pairs: Option<&mut size_t>,
) {
    let mut is_good_pair = 0 as libc::c_int;
    let mut p = crate::src::enc::block_splitter::HistogramPair {
        idx1: 0,
        idx2: 0,
        cost_combo: 0.,
        cost_diff: 0.,
    };
    p.idx2= 0 as libc::c_int as uint32_t;
    p.idx1= p.idx2;
    p.cost_combo= 0 as libc::c_int as libc::c_double;
    p.cost_diff= p.cost_combo;
    if idx1 == idx2 {
        return;
    }
    if idx2 < idx1 {
        let mut t = idx2;
        idx2= idx1;
        idx1= t;
    }
    p.idx1= idx1;
    p.idx2= idx2;
    p.cost_diff= 0.5f64
        * ClusterCostDiff(
            *cluster_size.offset(idx1 as isize) as size_t,
            *cluster_size.offset(idx2 as isize) as size_t,
        );
    p.cost_diff-= (*out.offset(idx1 as isize)).bit_cost_;
    p.cost_diff-= (*out.offset(idx2 as isize)).bit_cost_;
    if (*out.offset(idx1 as isize)).total_count_ == 0 as libc::c_int as libc::c_ulong {
        p.cost_combo= (*out.offset(idx2 as isize)).bit_cost_;
        is_good_pair= 1 as libc::c_int;
    } else if (*out.offset(idx2 as isize)).total_count_
        == 0 as libc::c_int as libc::c_ulong
    {
        p.cost_combo= (*out.offset(idx1 as isize)).bit_cost_;
        is_good_pair= 1 as libc::c_int;
    } else {
        let mut threshold = if (*num_pairs.as_deref().unwrap()) == 0 as libc::c_int as libc::c_ulong {
            1e99f64
        } else {
            brotli_max_double(
                0.0f64,
                (*pairs.offset(0 as libc::c_int as isize)).cost_diff,
            )
        };
        let mut combo = *out.offset(idx1 as isize);
        let mut cost_combo: libc::c_double = 0.;
        HistogramAddHistogramDistance(core::ptr::addr_of_mut!(combo), &*out.offset(idx2 as isize));
        cost_combo= crate::src::enc::bit_cost::BrotliPopulationCostDistance(core::ptr::addr_of!(combo));
        if cost_combo < threshold - p.cost_diff {
            p.cost_combo= cost_combo;
            is_good_pair= 1 as libc::c_int;
        }
    }
    if is_good_pair != 0 {
        p.cost_diff+= p.cost_combo;
        if (*num_pairs.as_deref().unwrap()) > 0 as libc::c_int as libc::c_ulong
            && HistogramPairIsLess(core::ptr::addr_of_mut!(*pairs.offset(0 as libc::c_int as isize)), core::ptr::addr_of!(p))
                != 0
        {
            if (*num_pairs.as_deref().unwrap()) < max_num_pairs {
                *pairs
                    .offset(
                        (*num_pairs.as_deref().unwrap()) as isize,
                    ) = *pairs.offset(0 as libc::c_int as isize);
                *num_pairs.as_deref_mut().unwrap()= (*num_pairs.as_deref().unwrap()).wrapping_add(1);
            }
            *pairs.offset(0 as libc::c_int as isize) = p;
        } else if (*num_pairs.as_deref().unwrap()) < max_num_pairs {
            *pairs.offset((*num_pairs.as_deref().unwrap()) as isize) = p;
            *num_pairs.as_deref_mut().unwrap()= (*num_pairs.as_deref().unwrap()).wrapping_add(1);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn BrotliHistogramCombineLiteral(
    mut out: *mut crate::src::enc::bit_cost::HistogramLiteral,
    mut cluster_size: *mut uint32_t,
    mut symbols: *mut uint32_t,
    mut clusters: *mut uint32_t,
    mut pairs: *mut crate::src::enc::block_splitter::HistogramPair,
    mut num_clusters: size_t,
    mut symbols_size: size_t,
    mut max_clusters: size_t,
    mut max_num_pairs: size_t,
) -> size_t {
    let mut cost_diff_threshold = 0.0f64;
    let mut min_cluster_size = 1 as libc::c_int as size_t;
    let mut num_pairs = 0 as libc::c_int as size_t;
    let mut idx1: size_t = 0;
    idx1= 0 as libc::c_int as size_t;
    while idx1 < num_clusters {
        let mut idx2: size_t = 0;
        idx2= idx1.wrapping_add(1 as libc::c_int as libc::c_ulong);
        while idx2 < num_clusters {
            BrotliCompareAndPushToQueueLiteral(
                out,
                cluster_size,
                *clusters.offset(idx1 as isize),
                *clusters.offset(idx2 as isize),
                max_num_pairs,
                core::ptr::addr_of_mut!(*pairs.offset(0 as libc::c_int as isize)),
                Some(&mut num_pairs),
            );
            idx2= idx2.wrapping_add(1);
        }
        idx1= idx1.wrapping_add(1);
    }
    while num_clusters > min_cluster_size {
        let mut best_idx1: uint32_t = 0;
        let mut best_idx2: uint32_t = 0;
        let mut i: size_t = 0;
        if (*pairs.offset(0 as libc::c_int as isize)).cost_diff >= cost_diff_threshold {
            cost_diff_threshold= 1e99f64;
            min_cluster_size= max_clusters;
        } else {
            best_idx1= (*pairs.offset(0 as libc::c_int as isize)).idx1;
            best_idx2= (*pairs.offset(0 as libc::c_int as isize)).idx2;
            HistogramAddHistogramLiteral(
                core::ptr::addr_of_mut!(*out.offset(best_idx1 as isize)),
                core::ptr::addr_of_mut!(*out.offset(best_idx2 as isize)),
            );
            (*out.offset(best_idx1 as isize))
                .bit_cost_ = (*pairs.offset(0 as libc::c_int as isize)).cost_combo;
            *cluster_size.offset(best_idx1 as isize) = (*cluster_size.offset(best_idx1 as isize) as libc::c_uint)
                .wrapping_add(*cluster_size.offset(best_idx2 as isize)) as uint32_t
                as uint32_t;
            i= 0 as libc::c_int as size_t;
            while i < symbols_size {
                if *symbols.offset(i as isize) == best_idx2 {
                    *symbols.offset(i as isize) = best_idx1;
                }
                i= i.wrapping_add(1);
            }
            i= 0 as libc::c_int as size_t;
            while i < num_clusters {
                if *clusters.offset(i as isize) == best_idx2 {
                    memmove(
                        core::ptr::addr_of_mut!(*clusters.offset(i as isize)) as *mut uint32_t
                            as *mut libc::c_void,
                        core::ptr::addr_of_mut!(*clusters
                            .offset(
                                i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                            )) as *mut uint32_t as *const libc::c_void,
                        num_clusters
                            .wrapping_sub(i)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<uint32_t>() as libc::c_ulong,
                            ),
                    );
                    break;
                } else {
                    i= i.wrapping_add(1);
                }
            }
            num_clusters= num_clusters.wrapping_sub(1);
            let mut copy_to_idx = 0 as libc::c_int as size_t;
            i= 0 as libc::c_int as size_t;
            while i < num_pairs {
                let mut p: *mut crate::src::enc::block_splitter::HistogramPair = core::ptr::addr_of_mut!(*pairs.offset(i as isize))
                    as *mut crate::src::enc::block_splitter::HistogramPair;
                if !((*p).idx1 == best_idx1 || (*p).idx2 == best_idx1
                    || (*p).idx1 == best_idx2 || (*p).idx2 == best_idx2)
                {
                    if HistogramPairIsLess(
                        core::ptr::addr_of_mut!(*pairs.offset(0 as libc::c_int as isize)),
                        p,
                    ) != 0
                    {
                        let mut front = *pairs.offset(0 as libc::c_int as isize);
                        *pairs.offset(0 as libc::c_int as isize) = (*p);
                        *pairs.offset(copy_to_idx as isize) = front;
                    } else {
                        *pairs.offset(copy_to_idx as isize) = (*p);
                    }
                    copy_to_idx= copy_to_idx.wrapping_add(1);
                }
                i= i.wrapping_add(1);
            }
            num_pairs= copy_to_idx;
            i= 0 as libc::c_int as size_t;
            while i < num_clusters {
                BrotliCompareAndPushToQueueLiteral(
                    out,
                    cluster_size,
                    best_idx1,
                    *clusters.offset(i as isize),
                    max_num_pairs,
                    core::ptr::addr_of_mut!(*pairs.offset(0 as libc::c_int as isize)),
                    Some(&mut num_pairs),
                );
                i= i.wrapping_add(1);
            }
        }
    }
    return num_clusters;
}
#[no_mangle]
pub unsafe extern "C" fn BrotliHistogramCombineDistance(
    mut out: *mut crate::src::enc::bit_cost::HistogramDistance,
    mut cluster_size: *mut uint32_t,
    mut symbols: *mut uint32_t,
    mut clusters: *mut uint32_t,
    mut pairs: *mut crate::src::enc::block_splitter::HistogramPair,
    mut num_clusters: size_t,
    mut symbols_size: size_t,
    mut max_clusters: size_t,
    mut max_num_pairs: size_t,
) -> size_t {
    let mut cost_diff_threshold = 0.0f64;
    let mut min_cluster_size = 1 as libc::c_int as size_t;
    let mut num_pairs = 0 as libc::c_int as size_t;
    let mut idx1: size_t = 0;
    idx1= 0 as libc::c_int as size_t;
    while idx1 < num_clusters {
        let mut idx2: size_t = 0;
        idx2= idx1.wrapping_add(1 as libc::c_int as libc::c_ulong);
        while idx2 < num_clusters {
            BrotliCompareAndPushToQueueDistance(
                out,
                cluster_size,
                *clusters.offset(idx1 as isize),
                *clusters.offset(idx2 as isize),
                max_num_pairs,
                core::ptr::addr_of_mut!(*pairs.offset(0 as libc::c_int as isize)),
                Some(&mut num_pairs),
            );
            idx2= idx2.wrapping_add(1);
        }
        idx1= idx1.wrapping_add(1);
    }
    while num_clusters > min_cluster_size {
        let mut best_idx1: uint32_t = 0;
        let mut best_idx2: uint32_t = 0;
        let mut i: size_t = 0;
        if (*pairs.offset(0 as libc::c_int as isize)).cost_diff >= cost_diff_threshold {
            cost_diff_threshold= 1e99f64;
            min_cluster_size= max_clusters;
        } else {
            best_idx1= (*pairs.offset(0 as libc::c_int as isize)).idx1;
            best_idx2= (*pairs.offset(0 as libc::c_int as isize)).idx2;
            HistogramAddHistogramDistance(
                core::ptr::addr_of_mut!(*out.offset(best_idx1 as isize)),
                core::ptr::addr_of_mut!(*out.offset(best_idx2 as isize)),
            );
            (*out.offset(best_idx1 as isize))
                .bit_cost_ = (*pairs.offset(0 as libc::c_int as isize)).cost_combo;
            *cluster_size.offset(best_idx1 as isize) = (*cluster_size.offset(best_idx1 as isize) as libc::c_uint)
                .wrapping_add(*cluster_size.offset(best_idx2 as isize)) as uint32_t
                as uint32_t;
            i= 0 as libc::c_int as size_t;
            while i < symbols_size {
                if *symbols.offset(i as isize) == best_idx2 {
                    *symbols.offset(i as isize) = best_idx1;
                }
                i= i.wrapping_add(1);
            }
            i= 0 as libc::c_int as size_t;
            while i < num_clusters {
                if *clusters.offset(i as isize) == best_idx2 {
                    memmove(
                        core::ptr::addr_of_mut!(*clusters.offset(i as isize)) as *mut uint32_t
                            as *mut libc::c_void,
                        core::ptr::addr_of_mut!(*clusters
                            .offset(
                                i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                            )) as *mut uint32_t as *const libc::c_void,
                        num_clusters
                            .wrapping_sub(i)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<uint32_t>() as libc::c_ulong,
                            ),
                    );
                    break;
                } else {
                    i= i.wrapping_add(1);
                }
            }
            num_clusters= num_clusters.wrapping_sub(1);
            let mut copy_to_idx = 0 as libc::c_int as size_t;
            i= 0 as libc::c_int as size_t;
            while i < num_pairs {
                let mut p: *mut crate::src::enc::block_splitter::HistogramPair = core::ptr::addr_of_mut!(*pairs.offset(i as isize))
                    as *mut crate::src::enc::block_splitter::HistogramPair;
                if !((*p).idx1 == best_idx1 || (*p).idx2 == best_idx1
                    || (*p).idx1 == best_idx2 || (*p).idx2 == best_idx2)
                {
                    if HistogramPairIsLess(
                        core::ptr::addr_of_mut!(*pairs.offset(0 as libc::c_int as isize)),
                        p,
                    ) != 0
                    {
                        let mut front = *pairs.offset(0 as libc::c_int as isize);
                        *pairs.offset(0 as libc::c_int as isize) = (*p);
                        *pairs.offset(copy_to_idx as isize) = front;
                    } else {
                        *pairs.offset(copy_to_idx as isize) = (*p);
                    }
                    copy_to_idx= copy_to_idx.wrapping_add(1);
                }
                i= i.wrapping_add(1);
            }
            num_pairs= copy_to_idx;
            i= 0 as libc::c_int as size_t;
            while i < num_clusters {
                BrotliCompareAndPushToQueueDistance(
                    out,
                    cluster_size,
                    best_idx1,
                    *clusters.offset(i as isize),
                    max_num_pairs,
                    core::ptr::addr_of_mut!(*pairs.offset(0 as libc::c_int as isize)),
                    Some(&mut num_pairs),
                );
                i= i.wrapping_add(1);
            }
        }
    }
    return num_clusters;
}
#[no_mangle]
pub unsafe extern "C" fn BrotliHistogramCombineCommand(
    mut out: *mut crate::src::enc::bit_cost::HistogramCommand,
    mut cluster_size: *mut uint32_t,
    mut symbols: *mut uint32_t,
    mut clusters: *mut uint32_t,
    mut pairs: *mut crate::src::enc::block_splitter::HistogramPair,
    mut num_clusters: size_t,
    mut symbols_size: size_t,
    mut max_clusters: size_t,
    mut max_num_pairs: size_t,
) -> size_t {
    let mut cost_diff_threshold = 0.0f64;
    let mut min_cluster_size = 1 as libc::c_int as size_t;
    let mut num_pairs = 0 as libc::c_int as size_t;
    let mut idx1: size_t = 0;
    idx1= 0 as libc::c_int as size_t;
    while idx1 < num_clusters {
        let mut idx2: size_t = 0;
        idx2= idx1.wrapping_add(1 as libc::c_int as libc::c_ulong);
        while idx2 < num_clusters {
            BrotliCompareAndPushToQueueCommand(
                out,
                cluster_size,
                *clusters.offset(idx1 as isize),
                *clusters.offset(idx2 as isize),
                max_num_pairs,
                core::ptr::addr_of_mut!(*pairs.offset(0 as libc::c_int as isize)),
                Some(&mut num_pairs),
            );
            idx2= idx2.wrapping_add(1);
        }
        idx1= idx1.wrapping_add(1);
    }
    while num_clusters > min_cluster_size {
        let mut best_idx1: uint32_t = 0;
        let mut best_idx2: uint32_t = 0;
        let mut i: size_t = 0;
        if (*pairs.offset(0 as libc::c_int as isize)).cost_diff >= cost_diff_threshold {
            cost_diff_threshold= 1e99f64;
            min_cluster_size= max_clusters;
        } else {
            best_idx1= (*pairs.offset(0 as libc::c_int as isize)).idx1;
            best_idx2= (*pairs.offset(0 as libc::c_int as isize)).idx2;
            HistogramAddHistogramCommand(
                core::ptr::addr_of_mut!(*out.offset(best_idx1 as isize)),
                core::ptr::addr_of_mut!(*out.offset(best_idx2 as isize)),
            );
            (*out.offset(best_idx1 as isize))
                .bit_cost_ = (*pairs.offset(0 as libc::c_int as isize)).cost_combo;
            *cluster_size.offset(best_idx1 as isize) = (*cluster_size.offset(best_idx1 as isize) as libc::c_uint)
                .wrapping_add(*cluster_size.offset(best_idx2 as isize)) as uint32_t
                as uint32_t;
            i= 0 as libc::c_int as size_t;
            while i < symbols_size {
                if *symbols.offset(i as isize) == best_idx2 {
                    *symbols.offset(i as isize) = best_idx1;
                }
                i= i.wrapping_add(1);
            }
            i= 0 as libc::c_int as size_t;
            while i < num_clusters {
                if *clusters.offset(i as isize) == best_idx2 {
                    memmove(
                        core::ptr::addr_of_mut!(*clusters.offset(i as isize)) as *mut uint32_t
                            as *mut libc::c_void,
                        core::ptr::addr_of_mut!(*clusters
                            .offset(
                                i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                            )) as *mut uint32_t as *const libc::c_void,
                        num_clusters
                            .wrapping_sub(i)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<uint32_t>() as libc::c_ulong,
                            ),
                    );
                    break;
                } else {
                    i= i.wrapping_add(1);
                }
            }
            num_clusters= num_clusters.wrapping_sub(1);
            let mut copy_to_idx = 0 as libc::c_int as size_t;
            i= 0 as libc::c_int as size_t;
            while i < num_pairs {
                let mut p: *mut crate::src::enc::block_splitter::HistogramPair = core::ptr::addr_of_mut!(*pairs.offset(i as isize))
                    as *mut crate::src::enc::block_splitter::HistogramPair;
                if !((*p).idx1 == best_idx1 || (*p).idx2 == best_idx1
                    || (*p).idx1 == best_idx2 || (*p).idx2 == best_idx2)
                {
                    if HistogramPairIsLess(
                        core::ptr::addr_of_mut!(*pairs.offset(0 as libc::c_int as isize)),
                        p,
                    ) != 0
                    {
                        let mut front = *pairs.offset(0 as libc::c_int as isize);
                        *pairs.offset(0 as libc::c_int as isize) = (*p);
                        *pairs.offset(copy_to_idx as isize) = front;
                    } else {
                        *pairs.offset(copy_to_idx as isize) = (*p);
                    }
                    copy_to_idx= copy_to_idx.wrapping_add(1);
                }
                i= i.wrapping_add(1);
            }
            num_pairs= copy_to_idx;
            i= 0 as libc::c_int as size_t;
            while i < num_clusters {
                BrotliCompareAndPushToQueueCommand(
                    out,
                    cluster_size,
                    best_idx1,
                    *clusters.offset(i as isize),
                    max_num_pairs,
                    core::ptr::addr_of_mut!(*pairs.offset(0 as libc::c_int as isize)),
                    Some(&mut num_pairs),
                );
                i= i.wrapping_add(1);
            }
        }
    }
    return num_clusters;
}
#[no_mangle]
pub unsafe extern "C" fn BrotliHistogramBitCostDistanceCommand(
    mut histogram: *const crate::src::enc::bit_cost::HistogramCommand,
    mut candidate: *const crate::src::enc::bit_cost::HistogramCommand,
) -> libc::c_double {
    if (*histogram).total_count_ == 0 as libc::c_int as libc::c_ulong {
        return 0.0f64
    } else {
        let mut tmp = (*histogram);
        HistogramAddHistogramCommand(core::ptr::addr_of_mut!(tmp), candidate);
        return crate::src::enc::bit_cost::BrotliPopulationCostCommand(core::ptr::addr_of!(tmp)) - (*candidate).bit_cost_;
    };
}
#[no_mangle]
pub unsafe extern "C" fn BrotliHistogramBitCostDistanceDistance(
    mut histogram: *const crate::src::enc::bit_cost::HistogramDistance,
    mut candidate: *const crate::src::enc::bit_cost::HistogramDistance,
) -> libc::c_double {
    if (*histogram).total_count_ == 0 as libc::c_int as libc::c_ulong {
        return 0.0f64
    } else {
        let mut tmp = (*histogram);
        HistogramAddHistogramDistance(core::ptr::addr_of_mut!(tmp), candidate);
        return crate::src::enc::bit_cost::BrotliPopulationCostDistance(core::ptr::addr_of!(tmp)) - (*candidate).bit_cost_;
    };
}
#[no_mangle]
pub unsafe extern "C" fn BrotliHistogramBitCostDistanceLiteral(
    mut histogram: *const crate::src::enc::bit_cost::HistogramLiteral,
    mut candidate: *const crate::src::enc::bit_cost::HistogramLiteral,
) -> libc::c_double {
    if (*histogram).total_count_ == 0 as libc::c_int as libc::c_ulong {
        return 0.0f64
    } else {
        let mut tmp = (*histogram);
        HistogramAddHistogramLiteral(core::ptr::addr_of_mut!(tmp), candidate);
        return crate::src::enc::bit_cost::BrotliPopulationCostLiteral(core::ptr::addr_of!(tmp)) - (*candidate).bit_cost_;
    };
}
#[no_mangle]
pub unsafe extern "C" fn BrotliHistogramRemapLiteral(
    mut in_0: *const crate::src::enc::bit_cost::HistogramLiteral,
    mut in_size: size_t,
    mut clusters: *const uint32_t,
    mut num_clusters: size_t,
    mut out: *mut crate::src::enc::bit_cost::HistogramLiteral,
    mut symbols: *mut uint32_t,
) {
    let mut i: size_t = 0;
    i= 0 as libc::c_int as size_t;
    while i < in_size {
        let mut best_out = if i == 0 as libc::c_int as libc::c_ulong {
            *symbols.offset(0 as libc::c_int as isize)
        } else {
            *symbols.offset(i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
        };
        let mut best_bits = BrotliHistogramBitCostDistanceLiteral(
            &*in_0.offset(i as isize),
            core::ptr::addr_of_mut!(*out.offset(best_out as isize)),
        );
        let mut j: size_t = 0;
        j= 0 as libc::c_int as size_t;
        while j < num_clusters {
            let cur_bits = BrotliHistogramBitCostDistanceLiteral(
                &*in_0.offset(i as isize),
                core::ptr::addr_of_mut!(*out.offset(*clusters.offset(j as isize) as isize)),
            );
            if cur_bits < best_bits {
                best_bits= cur_bits;
                best_out= *clusters.offset(j as isize);
            }
            j= j.wrapping_add(1);
        }
        *symbols.offset(i as isize) = best_out;
        i= i.wrapping_add(1);
    }
    i= 0 as libc::c_int as size_t;
    while i < num_clusters {
        HistogramClearLiteral(core::ptr::addr_of_mut!(*out.offset(*clusters.offset(i as isize) as isize)));
        i= i.wrapping_add(1);
    }
    i= 0 as libc::c_int as size_t;
    while i < in_size {
        HistogramAddHistogramLiteral(
            core::ptr::addr_of_mut!(*out.offset(*symbols.offset(i as isize) as isize)),
            &*in_0.offset(i as isize),
        );
        i= i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn BrotliHistogramRemapDistance(
    mut in_0: *const crate::src::enc::bit_cost::HistogramDistance,
    mut in_size: size_t,
    mut clusters: *const uint32_t,
    mut num_clusters: size_t,
    mut out: *mut crate::src::enc::bit_cost::HistogramDistance,
    mut symbols: *mut uint32_t,
) {
    let mut i: size_t = 0;
    i= 0 as libc::c_int as size_t;
    while i < in_size {
        let mut best_out = if i == 0 as libc::c_int as libc::c_ulong {
            *symbols.offset(0 as libc::c_int as isize)
        } else {
            *symbols.offset(i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
        };
        let mut best_bits = BrotliHistogramBitCostDistanceDistance(
            &*in_0.offset(i as isize),
            core::ptr::addr_of_mut!(*out.offset(best_out as isize)),
        );
        let mut j: size_t = 0;
        j= 0 as libc::c_int as size_t;
        while j < num_clusters {
            let cur_bits = BrotliHistogramBitCostDistanceDistance(
                &*in_0.offset(i as isize),
                core::ptr::addr_of_mut!(*out.offset(*clusters.offset(j as isize) as isize)),
            );
            if cur_bits < best_bits {
                best_bits= cur_bits;
                best_out= *clusters.offset(j as isize);
            }
            j= j.wrapping_add(1);
        }
        *symbols.offset(i as isize) = best_out;
        i= i.wrapping_add(1);
    }
    i= 0 as libc::c_int as size_t;
    while i < num_clusters {
        HistogramClearDistance(core::ptr::addr_of_mut!(*out.offset(*clusters.offset(i as isize) as isize)));
        i= i.wrapping_add(1);
    }
    i= 0 as libc::c_int as size_t;
    while i < in_size {
        HistogramAddHistogramDistance(
            core::ptr::addr_of_mut!(*out.offset(*symbols.offset(i as isize) as isize)),
            &*in_0.offset(i as isize),
        );
        i= i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn BrotliHistogramRemapCommand(
    mut in_0: *const crate::src::enc::bit_cost::HistogramCommand,
    mut in_size: size_t,
    mut clusters: *const uint32_t,
    mut num_clusters: size_t,
    mut out: *mut crate::src::enc::bit_cost::HistogramCommand,
    mut symbols: *mut uint32_t,
) {
    let mut i: size_t = 0;
    i= 0 as libc::c_int as size_t;
    while i < in_size {
        let mut best_out = if i == 0 as libc::c_int as libc::c_ulong {
            *symbols.offset(0 as libc::c_int as isize)
        } else {
            *symbols.offset(i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
        };
        let mut best_bits = BrotliHistogramBitCostDistanceCommand(
            &*in_0.offset(i as isize),
            core::ptr::addr_of_mut!(*out.offset(best_out as isize)),
        );
        let mut j: size_t = 0;
        j= 0 as libc::c_int as size_t;
        while j < num_clusters {
            let cur_bits = BrotliHistogramBitCostDistanceCommand(
                &*in_0.offset(i as isize),
                core::ptr::addr_of_mut!(*out.offset(*clusters.offset(j as isize) as isize)),
            );
            if cur_bits < best_bits {
                best_bits= cur_bits;
                best_out= *clusters.offset(j as isize);
            }
            j= j.wrapping_add(1);
        }
        *symbols.offset(i as isize) = best_out;
        i= i.wrapping_add(1);
    }
    i= 0 as libc::c_int as size_t;
    while i < num_clusters {
        HistogramClearCommand(core::ptr::addr_of_mut!(*out.offset(*clusters.offset(i as isize) as isize)));
        i= i.wrapping_add(1);
    }
    i= 0 as libc::c_int as size_t;
    while i < in_size {
        HistogramAddHistogramCommand(
            core::ptr::addr_of_mut!(*out.offset(*symbols.offset(i as isize) as isize)),
            &*in_0.offset(i as isize),
        );
        i= i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn BrotliHistogramReindexLiteral(
    mut m: *mut crate::src::enc::backward_references_hq::MemoryManager,
    mut out: *mut crate::src::enc::bit_cost::HistogramLiteral,
    mut symbols: *mut uint32_t,
    mut length: size_t,
) -> size_t {
    static mut kInvalidIndex: uint32_t = !(0 as libc::c_int as uint32_t);
    let mut new_index = if length > 0 as libc::c_int as libc::c_ulong {
        crate::src::enc::memory::BrotliAllocate(
            m,
            length.wrapping_mul(::std::mem::size_of::<uint32_t>() as libc::c_ulong),
        ) as *mut uint32_t
    } else {
        0 as *mut uint32_t
    };
    let mut next_index: uint32_t = 0;
    let mut tmp = 0 as *mut crate::src::enc::bit_cost::HistogramLiteral;
    let mut i: size_t = 0;
    if 0 as libc::c_int != 0 || 0 as libc::c_int != 0 {
        return 0 as libc::c_int as size_t;
    }
    i= 0 as libc::c_int as size_t;
    while i < length {
        *new_index.offset(i as isize) = kInvalidIndex;
        i= i.wrapping_add(1);
    }
    next_index= 0 as libc::c_int as uint32_t;
    i= 0 as libc::c_int as size_t;
    while i < length {
        if *new_index.offset(*symbols.offset(i as isize) as isize) == kInvalidIndex {
            *new_index.offset(*symbols.offset(i as isize) as isize) = next_index;
            next_index= next_index.wrapping_add(1);
        }
        i= i.wrapping_add(1);
    }
    tmp= if next_index > 0 as libc::c_int as libc::c_uint {
        crate::src::enc::memory::BrotliAllocate(
            m,
            (next_index as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<crate::src::enc::bit_cost::HistogramLiteral>() as libc::c_ulong),
        ) as *mut crate::src::enc::bit_cost::HistogramLiteral
    } else {
        0 as *mut crate::src::enc::bit_cost::HistogramLiteral
    };
    if 0 as libc::c_int != 0 || 0 as libc::c_int != 0 {
        return 0 as libc::c_int as size_t;
    }
    next_index= 0 as libc::c_int as uint32_t;
    i= 0 as libc::c_int as size_t;
    while i < length {
        if *new_index.offset(*symbols.offset(i as isize) as isize) == next_index {
            *tmp
                .offset(
                    next_index as isize,
                ) = *out.offset(*symbols.offset(i as isize) as isize);
            next_index= next_index.wrapping_add(1);
        }
        *symbols
            .offset(
                i as isize,
            ) = *new_index.offset(*symbols.offset(i as isize) as isize);
        i= i.wrapping_add(1);
    }
    crate::src::enc::memory::BrotliFree(m, new_index as *mut libc::c_void);
    new_index= 0 as *mut uint32_t;
    i= 0 as libc::c_int as size_t;
    while i < next_index as libc::c_ulong {
        *out.offset(i as isize) = *tmp.offset(i as isize);
        i= i.wrapping_add(1);
    }
    crate::src::enc::memory::BrotliFree(m, tmp as *mut libc::c_void);
    tmp= 0 as *mut crate::src::enc::bit_cost::HistogramLiteral;
    return next_index as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn BrotliHistogramReindexDistance(
    mut m: *mut crate::src::enc::backward_references_hq::MemoryManager,
    mut out: *mut crate::src::enc::bit_cost::HistogramDistance,
    mut symbols: *mut uint32_t,
    mut length: size_t,
) -> size_t {
    static mut kInvalidIndex: uint32_t = !(0 as libc::c_int as uint32_t);
    let mut new_index = if length > 0 as libc::c_int as libc::c_ulong {
        crate::src::enc::memory::BrotliAllocate(
            m,
            length.wrapping_mul(::std::mem::size_of::<uint32_t>() as libc::c_ulong),
        ) as *mut uint32_t
    } else {
        0 as *mut uint32_t
    };
    let mut next_index: uint32_t = 0;
    let mut tmp = 0 as *mut crate::src::enc::bit_cost::HistogramDistance;
    let mut i: size_t = 0;
    if 0 as libc::c_int != 0 || 0 as libc::c_int != 0 {
        return 0 as libc::c_int as size_t;
    }
    i= 0 as libc::c_int as size_t;
    while i < length {
        *new_index.offset(i as isize) = kInvalidIndex;
        i= i.wrapping_add(1);
    }
    next_index= 0 as libc::c_int as uint32_t;
    i= 0 as libc::c_int as size_t;
    while i < length {
        if *new_index.offset(*symbols.offset(i as isize) as isize) == kInvalidIndex {
            *new_index.offset(*symbols.offset(i as isize) as isize) = next_index;
            next_index= next_index.wrapping_add(1);
        }
        i= i.wrapping_add(1);
    }
    tmp= if next_index > 0 as libc::c_int as libc::c_uint {
        crate::src::enc::memory::BrotliAllocate(
            m,
            (next_index as libc::c_ulong)
                .wrapping_mul(
                    ::std::mem::size_of::<crate::src::enc::bit_cost::HistogramDistance>() as libc::c_ulong,
                ),
        ) as *mut crate::src::enc::bit_cost::HistogramDistance
    } else {
        0 as *mut crate::src::enc::bit_cost::HistogramDistance
    };
    if 0 as libc::c_int != 0 || 0 as libc::c_int != 0 {
        return 0 as libc::c_int as size_t;
    }
    next_index= 0 as libc::c_int as uint32_t;
    i= 0 as libc::c_int as size_t;
    while i < length {
        if *new_index.offset(*symbols.offset(i as isize) as isize) == next_index {
            *tmp
                .offset(
                    next_index as isize,
                ) = *out.offset(*symbols.offset(i as isize) as isize);
            next_index= next_index.wrapping_add(1);
        }
        *symbols
            .offset(
                i as isize,
            ) = *new_index.offset(*symbols.offset(i as isize) as isize);
        i= i.wrapping_add(1);
    }
    crate::src::enc::memory::BrotliFree(m, new_index as *mut libc::c_void);
    new_index= 0 as *mut uint32_t;
    i= 0 as libc::c_int as size_t;
    while i < next_index as libc::c_ulong {
        *out.offset(i as isize) = *tmp.offset(i as isize);
        i= i.wrapping_add(1);
    }
    crate::src::enc::memory::BrotliFree(m, tmp as *mut libc::c_void);
    tmp= 0 as *mut crate::src::enc::bit_cost::HistogramDistance;
    return next_index as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn BrotliHistogramReindexCommand(
    mut m: *mut crate::src::enc::backward_references_hq::MemoryManager,
    mut out: *mut crate::src::enc::bit_cost::HistogramCommand,
    mut symbols: *mut uint32_t,
    mut length: size_t,
) -> size_t {
    static mut kInvalidIndex: uint32_t = !(0 as libc::c_int as uint32_t);
    let mut new_index = if length > 0 as libc::c_int as libc::c_ulong {
        crate::src::enc::memory::BrotliAllocate(
            m,
            length.wrapping_mul(::std::mem::size_of::<uint32_t>() as libc::c_ulong),
        ) as *mut uint32_t
    } else {
        0 as *mut uint32_t
    };
    let mut next_index: uint32_t = 0;
    let mut tmp = 0 as *mut crate::src::enc::bit_cost::HistogramCommand;
    let mut i: size_t = 0;
    if 0 as libc::c_int != 0 || 0 as libc::c_int != 0 {
        return 0 as libc::c_int as size_t;
    }
    i= 0 as libc::c_int as size_t;
    while i < length {
        *new_index.offset(i as isize) = kInvalidIndex;
        i= i.wrapping_add(1);
    }
    next_index= 0 as libc::c_int as uint32_t;
    i= 0 as libc::c_int as size_t;
    while i < length {
        if *new_index.offset(*symbols.offset(i as isize) as isize) == kInvalidIndex {
            *new_index.offset(*symbols.offset(i as isize) as isize) = next_index;
            next_index= next_index.wrapping_add(1);
        }
        i= i.wrapping_add(1);
    }
    tmp= if next_index > 0 as libc::c_int as libc::c_uint {
        crate::src::enc::memory::BrotliAllocate(
            m,
            (next_index as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<crate::src::enc::bit_cost::HistogramCommand>() as libc::c_ulong),
        ) as *mut crate::src::enc::bit_cost::HistogramCommand
    } else {
        0 as *mut crate::src::enc::bit_cost::HistogramCommand
    };
    if 0 as libc::c_int != 0 || 0 as libc::c_int != 0 {
        return 0 as libc::c_int as size_t;
    }
    next_index= 0 as libc::c_int as uint32_t;
    i= 0 as libc::c_int as size_t;
    while i < length {
        if *new_index.offset(*symbols.offset(i as isize) as isize) == next_index {
            *tmp
                .offset(
                    next_index as isize,
                ) = *out.offset(*symbols.offset(i as isize) as isize);
            next_index= next_index.wrapping_add(1);
        }
        *symbols
            .offset(
                i as isize,
            ) = *new_index.offset(*symbols.offset(i as isize) as isize);
        i= i.wrapping_add(1);
    }
    crate::src::enc::memory::BrotliFree(m, new_index as *mut libc::c_void);
    new_index= 0 as *mut uint32_t;
    i= 0 as libc::c_int as size_t;
    while i < next_index as libc::c_ulong {
        *out.offset(i as isize) = *tmp.offset(i as isize);
        i= i.wrapping_add(1);
    }
    crate::src::enc::memory::BrotliFree(m, tmp as *mut libc::c_void);
    tmp= 0 as *mut crate::src::enc::bit_cost::HistogramCommand;
    return next_index as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn BrotliClusterHistogramsDistance(
    mut m: *mut crate::src::enc::backward_references_hq::MemoryManager,
    mut in_0: *const crate::src::enc::bit_cost::HistogramDistance,
    mut in_size: size_t,
    mut max_histograms: size_t,
    mut out: *mut crate::src::enc::bit_cost::HistogramDistance,
    mut out_size: *mut size_t,
    mut histogram_symbols: *mut uint32_t,
) {
    let mut cluster_size = if in_size > 0 as libc::c_int as libc::c_ulong {
        crate::src::enc::memory::BrotliAllocate(
            m,
            in_size.wrapping_mul(::std::mem::size_of::<uint32_t>() as libc::c_ulong),
        ) as *mut uint32_t
    } else {
        0 as *mut uint32_t
    };
    let mut clusters = if in_size > 0 as libc::c_int as libc::c_ulong {
        crate::src::enc::memory::BrotliAllocate(
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
        crate::src::enc::memory::BrotliAllocate(
            m,
            pairs_capacity
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<crate::src::enc::block_splitter::HistogramPair>() as libc::c_ulong),
        ) as *mut crate::src::enc::block_splitter::HistogramPair
    } else {
        0 as *mut crate::src::enc::block_splitter::HistogramPair
    };
    let mut i: size_t = 0;
    if 0 as libc::c_int != 0 || 0 as libc::c_int != 0 || 0 as libc::c_int != 0
        || 0 as libc::c_int != 0
    {
        return;
    }
    i= 0 as libc::c_int as size_t;
    while i < in_size {
        *cluster_size.offset(i as isize) = 1 as libc::c_int as uint32_t;
        i= i.wrapping_add(1);
    }
    i= 0 as libc::c_int as size_t;
    while i < in_size {
        *out.offset(i as isize) = *in_0.offset(i as isize);
        (*out.offset(i as isize))
            .bit_cost_ = crate::src::enc::bit_cost::BrotliPopulationCostDistance(&*in_0.offset(i as isize));
        *histogram_symbols.offset(i as isize) = i as uint32_t;
        i= i.wrapping_add(1);
    }
    i= 0 as libc::c_int as size_t;
    while i < in_size {
        let mut num_to_combine = brotli_min_size_t(
            in_size.wrapping_sub(i),
            max_input_histograms,
        );
        let mut num_new_clusters: size_t = 0;
        let mut j: size_t = 0;
        j= 0 as libc::c_int as size_t;
        while j < num_to_combine {
            *clusters
                .offset(
                    num_clusters.wrapping_add(j) as isize,
                ) = i.wrapping_add(j) as uint32_t;
            j= j.wrapping_add(1);
        }
        num_new_clusters= BrotliHistogramCombineDistance(
            out,
            cluster_size,
            core::ptr::addr_of_mut!(*histogram_symbols.offset(i as isize)),
            core::ptr::addr_of_mut!(*clusters.offset(num_clusters as isize)),
            pairs,
            num_to_combine,
            num_to_combine,
            max_histograms,
            pairs_capacity,
        );
        num_clusters= (num_clusters as libc::c_ulong).wrapping_add(num_new_clusters)
            as size_t as size_t;
        i= (i as libc::c_ulong).wrapping_add(max_input_histograms) as size_t as size_t;
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
        let mut new_array = 0 as *mut crate::src::enc::block_splitter::HistogramPair;
        while _new_size < max_num_pairs.wrapping_add(1 as libc::c_int as libc::c_ulong) {
            _new_size= (_new_size as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
        }
        new_array= if _new_size > 0 as libc::c_int as libc::c_ulong {
            crate::src::enc::memory::BrotliAllocate(
                m,
                _new_size
                    .wrapping_mul(
                        ::std::mem::size_of::<crate::src::enc::block_splitter::HistogramPair>() as libc::c_ulong,
                    ),
            ) as *mut crate::src::enc::block_splitter::HistogramPair
        } else {
            0 as *mut crate::src::enc::block_splitter::HistogramPair
        };
        if 0 as libc::c_int == 0 && 0 as libc::c_int == 0
            && pairs_capacity != 0 as libc::c_int as libc::c_ulong
        {
            memcpy(
                new_array as *mut libc::c_void,
                pairs as *const libc::c_void,
                pairs_capacity
                    .wrapping_mul(
                        ::std::mem::size_of::<crate::src::enc::block_splitter::HistogramPair>() as libc::c_ulong,
                    ),
            );
        }
        crate::src::enc::memory::BrotliFree(m, pairs as *mut libc::c_void);
        pairs= 0 as *mut crate::src::enc::block_splitter::HistogramPair;
        pairs= new_array;
        pairs_capacity= _new_size;
    }
    if 0 as libc::c_int != 0 {
        return;
    }
    num_clusters= BrotliHistogramCombineDistance(
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
    crate::src::enc::memory::BrotliFree(m, pairs as *mut libc::c_void);
    pairs= 0 as *mut crate::src::enc::block_splitter::HistogramPair;
    crate::src::enc::memory::BrotliFree(m, cluster_size as *mut libc::c_void);
    cluster_size= 0 as *mut uint32_t;
    BrotliHistogramRemapDistance(
        in_0,
        in_size,
        clusters,
        num_clusters,
        out,
        histogram_symbols,
    );
    crate::src::enc::memory::BrotliFree(m, clusters as *mut libc::c_void);
    clusters= 0 as *mut uint32_t;
    *out_size= BrotliHistogramReindexDistance(m, out, histogram_symbols, in_size);
    if 0 as libc::c_int != 0 {
        return;
    }
}
#[no_mangle]
pub unsafe extern "C" fn BrotliClusterHistogramsCommand(
    mut m: *mut crate::src::enc::backward_references_hq::MemoryManager,
    mut in_0: *const crate::src::enc::bit_cost::HistogramCommand,
    mut in_size: size_t,
    mut max_histograms: size_t,
    mut out: *mut crate::src::enc::bit_cost::HistogramCommand,
    mut out_size: Option<&mut size_t>,
    mut histogram_symbols: *mut uint32_t,
) {
    let mut cluster_size = if in_size > 0 as libc::c_int as libc::c_ulong {
        crate::src::enc::memory::BrotliAllocate(
            m,
            in_size.wrapping_mul(::std::mem::size_of::<uint32_t>() as libc::c_ulong),
        ) as *mut uint32_t
    } else {
        0 as *mut uint32_t
    };
    let mut clusters = if in_size > 0 as libc::c_int as libc::c_ulong {
        crate::src::enc::memory::BrotliAllocate(
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
        crate::src::enc::memory::BrotliAllocate(
            m,
            pairs_capacity
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<crate::src::enc::block_splitter::HistogramPair>() as libc::c_ulong),
        ) as *mut crate::src::enc::block_splitter::HistogramPair
    } else {
        0 as *mut crate::src::enc::block_splitter::HistogramPair
    };
    let mut i: size_t = 0;
    if 0 as libc::c_int != 0 || 0 as libc::c_int != 0 || 0 as libc::c_int != 0
        || 0 as libc::c_int != 0
    {
        return;
    }
    i= 0 as libc::c_int as size_t;
    while i < in_size {
        *cluster_size.offset(i as isize) = 1 as libc::c_int as uint32_t;
        i= i.wrapping_add(1);
    }
    i= 0 as libc::c_int as size_t;
    while i < in_size {
        *out.offset(i as isize) = *in_0.offset(i as isize);
        (*out.offset(i as isize))
            .bit_cost_ = crate::src::enc::bit_cost::BrotliPopulationCostCommand(&*in_0.offset(i as isize));
        *histogram_symbols.offset(i as isize) = i as uint32_t;
        i= i.wrapping_add(1);
    }
    i= 0 as libc::c_int as size_t;
    while i < in_size {
        let mut num_to_combine = brotli_min_size_t(
            in_size.wrapping_sub(i),
            max_input_histograms,
        );
        let mut num_new_clusters: size_t = 0;
        let mut j: size_t = 0;
        j= 0 as libc::c_int as size_t;
        while j < num_to_combine {
            *clusters
                .offset(
                    num_clusters.wrapping_add(j) as isize,
                ) = i.wrapping_add(j) as uint32_t;
            j= j.wrapping_add(1);
        }
        num_new_clusters= BrotliHistogramCombineCommand(
            out,
            cluster_size,
            core::ptr::addr_of_mut!(*histogram_symbols.offset(i as isize)),
            core::ptr::addr_of_mut!(*clusters.offset(num_clusters as isize)),
            pairs,
            num_to_combine,
            num_to_combine,
            max_histograms,
            pairs_capacity,
        );
        num_clusters= (num_clusters as libc::c_ulong).wrapping_add(num_new_clusters)
            as size_t as size_t;
        i= (i as libc::c_ulong).wrapping_add(max_input_histograms) as size_t as size_t;
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
        let mut new_array = 0 as *mut crate::src::enc::block_splitter::HistogramPair;
        while _new_size < max_num_pairs.wrapping_add(1 as libc::c_int as libc::c_ulong) {
            _new_size= (_new_size as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
        }
        new_array= if _new_size > 0 as libc::c_int as libc::c_ulong {
            crate::src::enc::memory::BrotliAllocate(
                m,
                _new_size
                    .wrapping_mul(
                        ::std::mem::size_of::<crate::src::enc::block_splitter::HistogramPair>() as libc::c_ulong,
                    ),
            ) as *mut crate::src::enc::block_splitter::HistogramPair
        } else {
            0 as *mut crate::src::enc::block_splitter::HistogramPair
        };
        if 0 as libc::c_int == 0 && 0 as libc::c_int == 0
            && pairs_capacity != 0 as libc::c_int as libc::c_ulong
        {
            memcpy(
                new_array as *mut libc::c_void,
                pairs as *const libc::c_void,
                pairs_capacity
                    .wrapping_mul(
                        ::std::mem::size_of::<crate::src::enc::block_splitter::HistogramPair>() as libc::c_ulong,
                    ),
            );
        }
        crate::src::enc::memory::BrotliFree(m, pairs as *mut libc::c_void);
        pairs= 0 as *mut crate::src::enc::block_splitter::HistogramPair;
        pairs= new_array;
        pairs_capacity= _new_size;
    }
    if 0 as libc::c_int != 0 {
        return;
    }
    num_clusters= BrotliHistogramCombineCommand(
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
    crate::src::enc::memory::BrotliFree(m, pairs as *mut libc::c_void);
    pairs= 0 as *mut crate::src::enc::block_splitter::HistogramPair;
    crate::src::enc::memory::BrotliFree(m, cluster_size as *mut libc::c_void);
    cluster_size= 0 as *mut uint32_t;
    BrotliHistogramRemapCommand(
        in_0,
        in_size,
        clusters,
        num_clusters,
        out,
        histogram_symbols,
    );
    crate::src::enc::memory::BrotliFree(m, clusters as *mut libc::c_void);
    clusters= 0 as *mut uint32_t;
    *out_size.as_deref_mut().unwrap()= BrotliHistogramReindexCommand(m, out, histogram_symbols, in_size);
    if 0 as libc::c_int != 0 {
        return;
    }
}
#[no_mangle]
pub unsafe extern "C" fn BrotliClusterHistogramsLiteral(
    mut m: *mut crate::src::enc::backward_references_hq::MemoryManager,
    mut in_0: *const crate::src::enc::bit_cost::HistogramLiteral,
    mut in_size: size_t,
    mut max_histograms: size_t,
    mut out: *mut crate::src::enc::bit_cost::HistogramLiteral,
    mut out_size: *mut size_t,
    mut histogram_symbols: *mut uint32_t,
) {
    let mut cluster_size = if in_size > 0 as libc::c_int as libc::c_ulong {
        crate::src::enc::memory::BrotliAllocate(
            m,
            in_size.wrapping_mul(::std::mem::size_of::<uint32_t>() as libc::c_ulong),
        ) as *mut uint32_t
    } else {
        0 as *mut uint32_t
    };
    let mut clusters = if in_size > 0 as libc::c_int as libc::c_ulong {
        crate::src::enc::memory::BrotliAllocate(
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
        crate::src::enc::memory::BrotliAllocate(
            m,
            pairs_capacity
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<crate::src::enc::block_splitter::HistogramPair>() as libc::c_ulong),
        ) as *mut crate::src::enc::block_splitter::HistogramPair
    } else {
        0 as *mut crate::src::enc::block_splitter::HistogramPair
    };
    let mut i: size_t = 0;
    if 0 as libc::c_int != 0 || 0 as libc::c_int != 0 || 0 as libc::c_int != 0
        || 0 as libc::c_int != 0
    {
        return;
    }
    i= 0 as libc::c_int as size_t;
    while i < in_size {
        *cluster_size.offset(i as isize) = 1 as libc::c_int as uint32_t;
        i= i.wrapping_add(1);
    }
    i= 0 as libc::c_int as size_t;
    while i < in_size {
        *out.offset(i as isize) = *in_0.offset(i as isize);
        (*out.offset(i as isize))
            .bit_cost_ = crate::src::enc::bit_cost::BrotliPopulationCostLiteral(&*in_0.offset(i as isize));
        *histogram_symbols.offset(i as isize) = i as uint32_t;
        i= i.wrapping_add(1);
    }
    i= 0 as libc::c_int as size_t;
    while i < in_size {
        let mut num_to_combine = brotli_min_size_t(
            in_size.wrapping_sub(i),
            max_input_histograms,
        );
        let mut num_new_clusters: size_t = 0;
        let mut j: size_t = 0;
        j= 0 as libc::c_int as size_t;
        while j < num_to_combine {
            *clusters
                .offset(
                    num_clusters.wrapping_add(j) as isize,
                ) = i.wrapping_add(j) as uint32_t;
            j= j.wrapping_add(1);
        }
        num_new_clusters= BrotliHistogramCombineLiteral(
            out,
            cluster_size,
            core::ptr::addr_of_mut!(*histogram_symbols.offset(i as isize)),
            core::ptr::addr_of_mut!(*clusters.offset(num_clusters as isize)),
            pairs,
            num_to_combine,
            num_to_combine,
            max_histograms,
            pairs_capacity,
        );
        num_clusters= (num_clusters as libc::c_ulong).wrapping_add(num_new_clusters)
            as size_t as size_t;
        i= (i as libc::c_ulong).wrapping_add(max_input_histograms) as size_t as size_t;
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
        let mut new_array = 0 as *mut crate::src::enc::block_splitter::HistogramPair;
        while _new_size < max_num_pairs.wrapping_add(1 as libc::c_int as libc::c_ulong) {
            _new_size= (_new_size as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
        }
        new_array= if _new_size > 0 as libc::c_int as libc::c_ulong {
            crate::src::enc::memory::BrotliAllocate(
                m,
                _new_size
                    .wrapping_mul(
                        ::std::mem::size_of::<crate::src::enc::block_splitter::HistogramPair>() as libc::c_ulong,
                    ),
            ) as *mut crate::src::enc::block_splitter::HistogramPair
        } else {
            0 as *mut crate::src::enc::block_splitter::HistogramPair
        };
        if 0 as libc::c_int == 0 && 0 as libc::c_int == 0
            && pairs_capacity != 0 as libc::c_int as libc::c_ulong
        {
            memcpy(
                new_array as *mut libc::c_void,
                pairs as *const libc::c_void,
                pairs_capacity
                    .wrapping_mul(
                        ::std::mem::size_of::<crate::src::enc::block_splitter::HistogramPair>() as libc::c_ulong,
                    ),
            );
        }
        crate::src::enc::memory::BrotliFree(m, pairs as *mut libc::c_void);
        pairs= 0 as *mut crate::src::enc::block_splitter::HistogramPair;
        pairs= new_array;
        pairs_capacity= _new_size;
    }
    if 0 as libc::c_int != 0 {
        return;
    }
    num_clusters= BrotliHistogramCombineLiteral(
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
    crate::src::enc::memory::BrotliFree(m, pairs as *mut libc::c_void);
    pairs= 0 as *mut crate::src::enc::block_splitter::HistogramPair;
    crate::src::enc::memory::BrotliFree(m, cluster_size as *mut libc::c_void);
    cluster_size= 0 as *mut uint32_t;
    BrotliHistogramRemapLiteral(
        in_0,
        in_size,
        clusters,
        num_clusters,
        out,
        histogram_symbols,
    );
    crate::src::enc::memory::BrotliFree(m, clusters as *mut libc::c_void);
    clusters= 0 as *mut uint32_t;
    *out_size= BrotliHistogramReindexLiteral(m, out, histogram_symbols, in_size);
    if 0 as libc::c_int != 0 {
        return;
    }
}
