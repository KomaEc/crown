use ::libc;
extern "C" {
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type int16_t = __int16_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor120 { dummy: () }
pub type HuffmanTreeComparator = Option::<
    unsafe extern "C" fn(*const crate::src::enc::brotli_bit_stream::HuffmanTree, *const crate::src::enc::brotli_bit_stream::HuffmanTree) -> libc::c_int,
>;
#[inline(always)]
unsafe extern "C" fn brotli_max_uint32_t(mut a: uint32_t, mut b: uint32_t) -> uint32_t {
    return if a > b { a } else { b };
}
#[inline(always)]
unsafe extern "C" fn InitHuffmanTree(
    mut self_0: Option<&mut crate::src::enc::brotli_bit_stream::HuffmanTree>,
    mut count: uint32_t,
    mut left: int16_t,
    mut right: int16_t,
) {
    (*self_0.as_deref_mut().unwrap()).total_count_= count;
    (*self_0.as_deref_mut().unwrap()).index_left_= left;
    (*self_0.as_deref_mut().unwrap()).index_right_or_value_= right;
}
#[inline(always)]
unsafe extern "C" fn SortHuffmanTreeItems(
    mut items: *mut crate::src::enc::brotli_bit_stream::HuffmanTree,
    mut n: size_t,
    mut comparator: HuffmanTreeComparator,
) {
    if n < 13 as libc::c_int as libc::c_ulong {
        let mut i: size_t = 0;
        i= 1 as libc::c_int as size_t;
        while i < n {
            let mut tmp = *items.offset(i as isize);
            let mut k = i;
            let mut j = i.wrapping_sub(1 as libc::c_int as libc::c_ulong);
            while comparator
                .expect(
                    "non-null function pointer",
                )(core::ptr::addr_of!(tmp), core::ptr::addr_of_mut!(*items.offset(j as isize))) != 0
            {
                *items.offset(k as isize) = *items.offset(j as isize);
                k= j;
                let fresh0 = j;
                j= j.wrapping_sub(1);
                if fresh0 == 0 {
                    break;
                }
            }
            *items.offset(k as isize) = tmp;
            i= i.wrapping_add(1);
        }
        return;
    } else {
        let mut g = if n < 57 as libc::c_int as libc::c_ulong {
            2 as libc::c_int
        } else {
            0 as libc::c_int
        };
        while g < 6 as libc::c_int {
            let mut gap = crate::src::enc::entropy_encode::kBrotliShellGaps[g as usize];
            let mut i_0: size_t = 0;
            i_0= gap;
            while i_0 < n {
                let mut j_0 = i_0;
                let mut tmp_0 = *items.offset(i_0 as isize);
                while j_0 >= gap
                    && comparator
                        .expect(
                            "non-null function pointer",
                        )(core::ptr::addr_of!(tmp_0), core::ptr::addr_of_mut!(*items.offset(j_0.wrapping_sub(gap) as isize)))
                        != 0
                {
                    *items
                        .offset(
                            j_0 as isize,
                        ) = *items.offset(j_0.wrapping_sub(gap) as isize);
                    j_0= (j_0 as libc::c_ulong).wrapping_sub(gap) as size_t as size_t;
                }
                *items.offset(j_0 as isize) = tmp_0;
                i_0= i_0.wrapping_add(1);
            }
            g+= 1;
        }
    };
}
#[no_mangle]
pub static mut kBrotliShellGaps: [size_t; 6] = [
    132 as libc::c_int as size_t,
    57 as libc::c_int as size_t,
    23 as libc::c_int as size_t,
    10 as libc::c_int as size_t,
    4 as libc::c_int as size_t,
    1 as libc::c_int as size_t,
];
#[no_mangle]
pub unsafe extern "C" fn BrotliSetDepth(
    mut p0: libc::c_int,
    mut pool: *mut crate::src::enc::brotli_bit_stream::HuffmanTree,
    mut depth: *mut uint8_t,
    mut max_depth: libc::c_int,
) -> libc::c_int {
    let mut stack: [libc::c_int; 16] = [0; 16];
    let mut level = 0 as libc::c_int;
    let mut p = p0;
    stack[0 as libc::c_int as usize]= -(1 as libc::c_int);
    loop {
        if (*pool.offset(p as isize)).index_left_ as libc::c_int >= 0 as libc::c_int {
            level+= 1;
            if level > max_depth {
                return 0 as libc::c_int;
            }
            stack[level
                as usize]= (*pool.offset(p as isize)).index_right_or_value_
                as libc::c_int;
            p= (*pool.offset(p as isize)).index_left_ as libc::c_int;
        } else {
            *depth
                .offset(
                    (*pool.offset(p as isize)).index_right_or_value_ as isize,
                ) = level as uint8_t;
            while level >= 0 as libc::c_int
                && stack[level as usize] == -(1 as libc::c_int)
            {
                level-= 1;
            }
            if level < 0 as libc::c_int {
                return 1 as libc::c_int;
            }
            p= stack[level as usize];
            stack[level as usize]= -(1 as libc::c_int);
        }
    };
}
#[inline(always)]
unsafe extern "C" fn SortHuffmanTree(
    mut v0: *const crate::src::enc::brotli_bit_stream::HuffmanTree,
    mut v1: *const crate::src::enc::brotli_bit_stream::HuffmanTree,
) -> libc::c_int {
    if (*v0).total_count_ != (*v1).total_count_ {
        return if (*v0).total_count_ < (*v1).total_count_ {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        };
    }
    return if (*v0).index_right_or_value_ as libc::c_int
        > (*v1).index_right_or_value_ as libc::c_int
    {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn BrotliCreateHuffmanTree(
    mut data: *const uint32_t,
    mut length: size_t,
    mut tree_limit: libc::c_int,
    mut tree: *mut crate::src::enc::brotli_bit_stream::HuffmanTree,
    mut depth: *mut uint8_t,
) {
    let mut count_limit: uint32_t = 0;
    let mut sentinel = crate::src::enc::brotli_bit_stream::HuffmanTree {
        total_count_: 0,
        index_left_: 0,
        index_right_or_value_: 0,
    };
    InitHuffmanTree(
        Some(&mut sentinel),
        !(0 as libc::c_int as uint32_t),
        -(1 as libc::c_int) as int16_t,
        -(1 as libc::c_int) as int16_t,
    );
    count_limit= 1 as libc::c_int as uint32_t;
    loop {
        let mut n = 0 as libc::c_int as size_t;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        let mut k: size_t = 0;
        i= length;
        while i != 0 as libc::c_int as libc::c_ulong {
            i= i.wrapping_sub(1);
            if *data.offset(i as isize) != 0 {
                let count = brotli_max_uint32_t(*data.offset(i as isize), count_limit);
                let fresh1 = n;
                n= n.wrapping_add(1);
                InitHuffmanTree(
                    Some(&mut *tree.offset(fresh1 as isize)),
                    count,
                    -(1 as libc::c_int) as int16_t,
                    i as int16_t,
                );
            }
        }
        if n == 1 as libc::c_int as libc::c_ulong {
            *depth
                .offset(
                    (*tree.offset(0 as libc::c_int as isize)).index_right_or_value_
                        as isize,
                ) = 1 as libc::c_int as uint8_t;
            break;
        } else {
            SortHuffmanTreeItems(
                tree,
                n,
                Some(
                    SortHuffmanTree
                        as unsafe extern "C" fn(
                            *const crate::src::enc::brotli_bit_stream::HuffmanTree,
                            *const crate::src::enc::brotli_bit_stream::HuffmanTree,
                        ) -> libc::c_int,
                ),
            );
            *tree.offset(n as isize) = sentinel;
            *tree
                .offset(
                    n.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = sentinel;
            i= 0 as libc::c_int as size_t;
            j= n.wrapping_add(1 as libc::c_int as libc::c_ulong);
            k= n.wrapping_sub(1 as libc::c_int as libc::c_ulong);
            while k != 0 as libc::c_int as libc::c_ulong {
                let mut left: size_t = 0;
                let mut right: size_t = 0;
                if (*tree.offset(i as isize)).total_count_
                    <= (*tree.offset(j as isize)).total_count_
                {
                    left= i;
                    i= i.wrapping_add(1);
                } else {
                    left= j;
                    j= j.wrapping_add(1);
                }
                if (*tree.offset(i as isize)).total_count_
                    <= (*tree.offset(j as isize)).total_count_
                {
                    right= i;
                    i= i.wrapping_add(1);
                } else {
                    right= j;
                    j= j.wrapping_add(1);
                }
                let mut j_end = (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(n)
                    .wrapping_sub(k);
                (*tree.offset(j_end as isize))
                    .total_count_ = ((*tree.offset(left as isize)).total_count_)
                    .wrapping_add((*tree.offset(right as isize)).total_count_);
                (*tree.offset(j_end as isize)).index_left_ = left as int16_t;
                (*tree.offset(j_end as isize)).index_right_or_value_ = right as int16_t;
                *tree
                    .offset(
                        j_end.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    ) = sentinel;
                k= k.wrapping_sub(1);
            }
            if BrotliSetDepth(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(n)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                core::ptr::addr_of_mut!(*tree.offset(0 as libc::c_int as isize)),
                depth,
                tree_limit,
            ) != 0
            {
                break;
            }
            count_limit= (count_limit as libc::c_uint)
                .wrapping_mul(2 as libc::c_int as libc::c_uint) as uint32_t as uint32_t;
        }
    };
}
unsafe extern "C" fn Reverse(mut v: *mut uint8_t, mut start: size_t, mut end: size_t) {
    end= end.wrapping_sub(1);
    while start < end {
        let mut tmp = *v.offset(start as isize);
        *v.offset(start as isize) = *v.offset(end as isize);
        *v.offset(end as isize) = tmp;
        start= start.wrapping_add(1);
        end= end.wrapping_sub(1);
    }
}
unsafe extern "C" fn BrotliWriteHuffmanTreeRepetitions(
    mut previous_value: uint8_t,
    mut value: uint8_t,
    mut repetitions: size_t,
    mut tree_size: Option<&mut size_t>,
    mut tree: *mut uint8_t,
    mut extra_bits_data: *mut uint8_t,
) {
    if previous_value as libc::c_int != value as libc::c_int {
        *tree.offset((*tree_size.as_deref().unwrap()) as isize) = value;
        *extra_bits_data.offset((*tree_size.as_deref().unwrap()) as isize) = 0 as libc::c_int as uint8_t;
        *tree_size.as_deref_mut().unwrap()= (*tree_size.as_deref().unwrap()).wrapping_add(1);
        repetitions= repetitions.wrapping_sub(1);
    }
    if repetitions == 7 as libc::c_int as libc::c_ulong {
        *tree.offset((*tree_size.as_deref().unwrap()) as isize) = value;
        *extra_bits_data.offset((*tree_size.as_deref().unwrap()) as isize) = 0 as libc::c_int as uint8_t;
        *tree_size.as_deref_mut().unwrap()= (*tree_size.as_deref().unwrap()).wrapping_add(1);
        repetitions= repetitions.wrapping_sub(1);
    }
    if repetitions < 3 as libc::c_int as libc::c_ulong {
        let mut i: size_t = 0;
        i= 0 as libc::c_int as size_t;
        while i < repetitions {
            *tree.offset((*tree_size.as_deref().unwrap()) as isize) = value;
            *extra_bits_data.offset((*tree_size.as_deref().unwrap()) as isize) = 0 as libc::c_int as uint8_t;
            *tree_size.as_deref_mut().unwrap()= (*tree_size.as_deref().unwrap()).wrapping_add(1);
            i= i.wrapping_add(1);
        }
    } else {
        let mut start = (*tree_size.as_deref().unwrap());
        repetitions= (repetitions as libc::c_ulong)
            .wrapping_sub(3 as libc::c_int as libc::c_ulong) as size_t as size_t;
        loop {
            *tree.offset((*tree_size.as_deref().unwrap()) as isize) = 16 as libc::c_int as uint8_t;
            *extra_bits_data
                .offset(
                    (*tree_size.as_deref().unwrap()) as isize,
                ) = (repetitions & 0x3 as libc::c_int as libc::c_ulong) as uint8_t;
            *tree_size.as_deref_mut().unwrap()= (*tree_size.as_deref().unwrap()).wrapping_add(1);
            repetitions>>= 2 as libc::c_int;
            if repetitions == 0 as libc::c_int as libc::c_ulong {
                break;
            }
            repetitions= repetitions.wrapping_sub(1);
        }
        Reverse(tree, start, (*tree_size.as_deref().unwrap()));
        Reverse(extra_bits_data, start, (*tree_size.as_deref().unwrap()));
    };
}
unsafe extern "C" fn BrotliWriteHuffmanTreeRepetitionsZeros(
    mut repetitions: size_t,
    mut tree_size: Option<&mut size_t>,
    mut tree: *mut uint8_t,
    mut extra_bits_data: *mut uint8_t,
) {
    if repetitions == 11 as libc::c_int as libc::c_ulong {
        *tree.offset((*tree_size.as_deref().unwrap()) as isize) = 0 as libc::c_int as uint8_t;
        *extra_bits_data.offset((*tree_size.as_deref().unwrap()) as isize) = 0 as libc::c_int as uint8_t;
        *tree_size.as_deref_mut().unwrap()= (*tree_size.as_deref().unwrap()).wrapping_add(1);
        repetitions= repetitions.wrapping_sub(1);
    }
    if repetitions < 3 as libc::c_int as libc::c_ulong {
        let mut i: size_t = 0;
        i= 0 as libc::c_int as size_t;
        while i < repetitions {
            *tree.offset((*tree_size.as_deref().unwrap()) as isize) = 0 as libc::c_int as uint8_t;
            *extra_bits_data.offset((*tree_size.as_deref().unwrap()) as isize) = 0 as libc::c_int as uint8_t;
            *tree_size.as_deref_mut().unwrap()= (*tree_size.as_deref().unwrap()).wrapping_add(1);
            i= i.wrapping_add(1);
        }
    } else {
        let mut start = (*tree_size.as_deref().unwrap());
        repetitions= (repetitions as libc::c_ulong)
            .wrapping_sub(3 as libc::c_int as libc::c_ulong) as size_t as size_t;
        loop {
            *tree.offset((*tree_size.as_deref().unwrap()) as isize) = 17 as libc::c_int as uint8_t;
            *extra_bits_data
                .offset(
                    (*tree_size.as_deref().unwrap()) as isize,
                ) = (repetitions & 0x7 as libc::c_int as libc::c_ulong) as uint8_t;
            *tree_size.as_deref_mut().unwrap()= (*tree_size.as_deref().unwrap()).wrapping_add(1);
            repetitions>>= 3 as libc::c_int;
            if repetitions == 0 as libc::c_int as libc::c_ulong {
                break;
            }
            repetitions= repetitions.wrapping_sub(1);
        }
        Reverse(tree, start, (*tree_size.as_deref().unwrap()));
        Reverse(extra_bits_data, start, (*tree_size.as_deref().unwrap()));
    };
}
#[no_mangle]
pub unsafe extern "C" fn BrotliOptimizeHuffmanCountsForRle(
    mut length: size_t,
    mut counts: *mut uint32_t,
    mut good_for_rle: *mut uint8_t,
) {
    let mut nonzero_count = 0 as libc::c_int as size_t;
    let mut stride: size_t = 0;
    let mut limit: size_t = 0;
    let mut sum: size_t = 0;
    let streak_limit = 1240 as libc::c_int as size_t;
    let mut i: size_t = 0;
    i= 0 as libc::c_int as size_t;
    while i < length {
        if *counts.offset(i as isize) != 0 {
            nonzero_count= nonzero_count.wrapping_add(1);
        }
        i= i.wrapping_add(1);
    }
    if nonzero_count < 16 as libc::c_int as libc::c_ulong {
        return;
    }
    while length != 0 as libc::c_int as libc::c_ulong
        && *counts
            .offset(length.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            == 0 as libc::c_int as libc::c_uint
    {
        length= length.wrapping_sub(1);
    }
    if length == 0 as libc::c_int as libc::c_ulong {
        return;
    }
    let mut nonzeros = 0 as libc::c_int as size_t;
    let mut smallest_nonzero = ((1 as libc::c_int) << 30 as libc::c_int) as uint32_t;
    i= 0 as libc::c_int as size_t;
    while i < length {
        if *counts.offset(i as isize) != 0 as libc::c_int as libc::c_uint {
            nonzeros= nonzeros.wrapping_add(1);
            if smallest_nonzero > *counts.offset(i as isize) {
                smallest_nonzero= *counts.offset(i as isize);
            }
        }
        i= i.wrapping_add(1);
    }
    if nonzeros < 5 as libc::c_int as libc::c_ulong {
        return;
    }
    if smallest_nonzero < 4 as libc::c_int as libc::c_uint {
        let mut zeros = length.wrapping_sub(nonzeros);
        if zeros < 6 as libc::c_int as libc::c_ulong {
            i= 1 as libc::c_int as size_t;
            while i < length.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
                if *counts
                    .offset(i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                    != 0 as libc::c_int as libc::c_uint
                    && *counts.offset(i as isize) == 0 as libc::c_int as libc::c_uint
                    && *counts
                        .offset(
                            i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        ) != 0 as libc::c_int as libc::c_uint
                {
                    *counts.offset(i as isize) = 1 as libc::c_int as uint32_t;
                }
                i= i.wrapping_add(1);
            }
        }
    }
    if nonzeros < 28 as libc::c_int as libc::c_ulong {
        return;
    }
    memset(good_for_rle as *mut libc::c_void, 0 as libc::c_int, length);
    let mut symbol = *counts.offset(0 as libc::c_int as isize);
    let mut step = 0 as libc::c_int as size_t;
    i= 0 as libc::c_int as size_t;
    while i <= length {
        if i == length || *counts.offset(i as isize) != symbol {
            if symbol == 0 as libc::c_int as libc::c_uint
                && step >= 5 as libc::c_int as libc::c_ulong
                || symbol != 0 as libc::c_int as libc::c_uint
                    && step >= 7 as libc::c_int as libc::c_ulong
            {
                let mut k: size_t = 0;
                k= 0 as libc::c_int as size_t;
                while k < step {
                    *good_for_rle
                        .offset(
                            i
                                .wrapping_sub(k)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        ) = 1 as libc::c_int as uint8_t;
                    k= k.wrapping_add(1);
                }
            }
            step= 1 as libc::c_int as size_t;
            if i != length {
                symbol= *counts.offset(i as isize);
            }
        } else {
            step= step.wrapping_add(1);
        }
        i= i.wrapping_add(1);
    }
    stride= 0 as libc::c_int as size_t;
    limit= (256 as libc::c_int as libc::c_uint)
        .wrapping_mul(
            (*counts.offset(0 as libc::c_int as isize))
                .wrapping_add(*counts.offset(1 as libc::c_int as isize))
                .wrapping_add(*counts.offset(2 as libc::c_int as isize)),
        )
        .wrapping_div(3 as libc::c_int as libc::c_uint)
        .wrapping_add(420 as libc::c_int as libc::c_uint) as size_t;
    sum= 0 as libc::c_int as size_t;
    i= 0 as libc::c_int as size_t;
    while i <= length {
        if i == length || *good_for_rle.offset(i as isize) as libc::c_int != 0
            || i != 0 as libc::c_int as libc::c_ulong
                && *good_for_rle
                    .offset(i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                    as libc::c_int != 0
            || ((256 as libc::c_int as libc::c_uint)
                .wrapping_mul(*counts.offset(i as isize)) as libc::c_ulong)
                .wrapping_sub(limit)
                .wrapping_add(streak_limit)
                >= (2 as libc::c_int as libc::c_ulong).wrapping_mul(streak_limit)
        {
            if stride >= 4 as libc::c_int as libc::c_ulong
                || stride >= 3 as libc::c_int as libc::c_ulong
                    && sum == 0 as libc::c_int as libc::c_ulong
            {
                let mut k_0: size_t = 0;
                let mut count = sum
                    .wrapping_add(stride.wrapping_div(2 as libc::c_int as libc::c_ulong))
                    .wrapping_div(stride);
                if count == 0 as libc::c_int as libc::c_ulong {
                    count= 1 as libc::c_int as size_t;
                }
                if sum == 0 as libc::c_int as libc::c_ulong {
                    count= 0 as libc::c_int as size_t;
                }
                k_0= 0 as libc::c_int as size_t;
                while k_0 < stride {
                    *counts
                        .offset(
                            i
                                .wrapping_sub(k_0)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        ) = count as uint32_t;
                    k_0= k_0.wrapping_add(1);
                }
            }
            stride= 0 as libc::c_int as size_t;
            sum= 0 as libc::c_int as size_t;
            if i < length.wrapping_sub(2 as libc::c_int as libc::c_ulong) {
                limit= (256 as libc::c_int as libc::c_uint)
                    .wrapping_mul(
                        (*counts.offset(i as isize))
                            .wrapping_add(
                                *counts
                                    .offset(
                                        i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                                    ),
                            )
                            .wrapping_add(
                                *counts
                                    .offset(
                                        i.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize,
                                    ),
                            ),
                    )
                    .wrapping_div(3 as libc::c_int as libc::c_uint)
                    .wrapping_add(420 as libc::c_int as libc::c_uint) as size_t;
            } else if i < length {
                limit= (256 as libc::c_int as libc::c_uint)
                    .wrapping_mul(*counts.offset(i as isize)) as size_t;
            } else {
                limit= 0 as libc::c_int as size_t;
            }
        }
        stride= stride.wrapping_add(1);
        if i != length {
            sum= (sum as libc::c_ulong)
                .wrapping_add(*counts.offset(i as isize) as libc::c_ulong) as size_t
                as size_t;
            if stride >= 4 as libc::c_int as libc::c_ulong {
                limit= (256 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(sum)
                    .wrapping_add(stride.wrapping_div(2 as libc::c_int as libc::c_ulong))
                    .wrapping_div(stride);
            }
            if stride == 4 as libc::c_int as libc::c_ulong {
                limit= (limit as libc::c_ulong)
                    .wrapping_add(120 as libc::c_int as libc::c_ulong) as size_t
                    as size_t;
            }
        }
        i= i.wrapping_add(1);
    }
}
unsafe extern "C" fn DecideOverRleUse(
    mut depth: *const uint8_t,
    mut length: size_t,
    mut use_rle_for_non_zero: Option<&mut libc::c_int>,
    mut use_rle_for_zero: Option<&mut libc::c_int>,
) {
    let mut total_reps_zero = 0 as libc::c_int as size_t;
    let mut total_reps_non_zero = 0 as libc::c_int as size_t;
    let mut count_reps_zero = 1 as libc::c_int as size_t;
    let mut count_reps_non_zero = 1 as libc::c_int as size_t;
    let mut i: size_t = 0;
    i= 0 as libc::c_int as size_t;
    while i < length {
        let value = *depth.offset(i as isize);
        let mut reps = 1 as libc::c_int as size_t;
        let mut k: size_t = 0;
        k= i.wrapping_add(1 as libc::c_int as libc::c_ulong);
        while k < length
            && *depth.offset(k as isize) as libc::c_int == value as libc::c_int
        {
            reps= reps.wrapping_add(1);
            k= k.wrapping_add(1);
        }
        if reps >= 3 as libc::c_int as libc::c_ulong
            && value as libc::c_int == 0 as libc::c_int
        {
            total_reps_zero= (total_reps_zero as libc::c_ulong).wrapping_add(reps)
                as size_t as size_t;
            count_reps_zero= count_reps_zero.wrapping_add(1);
        }
        if reps >= 4 as libc::c_int as libc::c_ulong
            && value as libc::c_int != 0 as libc::c_int
        {
            total_reps_non_zero= (total_reps_non_zero as libc::c_ulong)
                .wrapping_add(reps) as size_t as size_t;
            count_reps_non_zero= count_reps_non_zero.wrapping_add(1);
        }
        i= (i as libc::c_ulong).wrapping_add(reps) as size_t as size_t;
    }
    *use_rle_for_non_zero.as_deref_mut().unwrap()= if total_reps_non_zero
        > count_reps_non_zero.wrapping_mul(2 as libc::c_int as libc::c_ulong)
    {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
    *use_rle_for_zero.as_deref_mut().unwrap()= if total_reps_zero
        > count_reps_zero.wrapping_mul(2 as libc::c_int as libc::c_ulong)
    {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn BrotliWriteHuffmanTree(
    mut depth: *const uint8_t,
    mut length: size_t,
    mut tree_size: Option<&mut size_t>,
    mut tree: *mut uint8_t,
    mut extra_bits_data: *mut uint8_t,
) {
    let mut previous_value = 8 as libc::c_int as uint8_t;
    let mut i: size_t = 0;
    let mut use_rle_for_non_zero = 0 as libc::c_int;
    let mut use_rle_for_zero = 0 as libc::c_int;
    let mut new_length = length;
    i= 0 as libc::c_int as size_t;
    while i < length {
        if !(*depth
            .offset(
                length.wrapping_sub(i).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    as isize,
            ) as libc::c_int == 0 as libc::c_int)
        {
            break;
        }
        new_length= new_length.wrapping_sub(1);
        i= i.wrapping_add(1);
    }
    if length > 50 as libc::c_int as libc::c_ulong {
        DecideOverRleUse(
            depth,
            new_length,
            Some(&mut use_rle_for_non_zero),
            Some(&mut use_rle_for_zero),
        );
    }
    i= 0 as libc::c_int as size_t;
    while i < new_length {
        let value = *depth.offset(i as isize);
        let mut reps = 1 as libc::c_int as size_t;
        if value as libc::c_int != 0 as libc::c_int && use_rle_for_non_zero != 0
            || value as libc::c_int == 0 as libc::c_int && use_rle_for_zero != 0
        {
            let mut k: size_t = 0;
            k= i.wrapping_add(1 as libc::c_int as libc::c_ulong);
            while k < new_length
                && *depth.offset(k as isize) as libc::c_int == value as libc::c_int
            {
                reps= reps.wrapping_add(1);
                k= k.wrapping_add(1);
            }
        }
        if value as libc::c_int == 0 as libc::c_int {
            BrotliWriteHuffmanTreeRepetitionsZeros(
                reps,
                tree_size.as_deref_mut(),
                tree,
                extra_bits_data,
            );
        } else {
            BrotliWriteHuffmanTreeRepetitions(
                previous_value,
                value,
                reps,
                tree_size.as_deref_mut(),
                tree,
                extra_bits_data,
            );
            previous_value= value;
        }
        i= (i as libc::c_ulong).wrapping_add(reps) as size_t as size_t;
    }
}
unsafe extern "C" fn BrotliReverseBits(
    mut num_bits: size_t,
    mut bits: uint16_t,
) -> uint16_t {
    static mut kLut: [size_t; 16] = [
        0 as libc::c_int as size_t,
        0x8 as libc::c_int as size_t,
        0x4 as libc::c_int as size_t,
        0xc as libc::c_int as size_t,
        0x2 as libc::c_int as size_t,
        0xa as libc::c_int as size_t,
        0x6 as libc::c_int as size_t,
        0xe as libc::c_int as size_t,
        0x1 as libc::c_int as size_t,
        0x9 as libc::c_int as size_t,
        0x5 as libc::c_int as size_t,
        0xd as libc::c_int as size_t,
        0x3 as libc::c_int as size_t,
        0xb as libc::c_int as size_t,
        0x7 as libc::c_int as size_t,
        0xf as libc::c_int as size_t,
    ];
    let mut retval = kLut[(bits as libc::c_int & 0xf as libc::c_int) as usize];
    let mut i: size_t = 0;
    i= 4 as libc::c_int as size_t;
    while i < num_bits {
        retval<<= 4 as libc::c_int;
        bits= (bits as libc::c_int >> 4 as libc::c_int) as uint16_t;
        retval|= kLut[(bits as libc::c_int & 0xf as libc::c_int) as usize];
        i= (i as libc::c_ulong).wrapping_add(4 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
    }
    retval>>= (0 as libc::c_int as libc::c_ulong).wrapping_sub(num_bits)
            & 0x3 as libc::c_int as libc::c_ulong;
    return retval as uint16_t;
}
#[no_mangle]
pub unsafe extern "C" fn BrotliConvertBitDepthsToSymbols(
    mut depth: *const uint8_t,
    mut len: size_t,
    mut bits: *mut uint16_t,
) {
    let mut bl_count: [uint16_t; 16] = [
        0 as libc::c_int as uint16_t,
        0,
        0,
        0,
        0,
        0,
        0,
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
    let mut next_code: [uint16_t; 16] = [0; 16];
    let mut i: size_t = 0;
    let mut code = 0 as libc::c_int;
    i= 0 as libc::c_int as size_t;
    while i < len {
        bl_count[*depth.offset(i as isize)
            as usize]= bl_count[*depth.offset(i as isize) as usize].wrapping_add(1);
        i= i.wrapping_add(1);
    }
    bl_count[0 as libc::c_int as usize]= 0 as libc::c_int as uint16_t;
    next_code[0 as libc::c_int as usize]= 0 as libc::c_int as uint16_t;
    i= 1 as libc::c_int as size_t;
    while i < 16 as libc::c_int as libc::c_ulong {
        code= (code
            + bl_count[i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as usize]
                as libc::c_int) << 1 as libc::c_int;
        next_code[i as usize]= code as uint16_t;
        i= i.wrapping_add(1);
    }
    i= 0 as libc::c_int as size_t;
    while i < len {
        if *depth.offset(i as isize) != 0 {
            let fresh2 = next_code[*depth.offset(i as isize) as usize];
            next_code[*depth.offset(i as isize)
                as usize]= next_code[*depth.offset(i as isize) as usize]
                .wrapping_add(1);
            *bits
                .offset(
                    i as isize,
                ) = BrotliReverseBits(*depth.offset(i as isize) as size_t, fresh2);
        }
        i= i.wrapping_add(1);
    }
}
