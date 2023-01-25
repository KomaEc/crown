use ::libc;
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor141 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor142 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor143 { dummy: () }
pub const BROTLI_TRANSFORM_UPPERCASE_FIRST: BrotliWordTransformType = 10;
pub type BrotliWordTransformType = libc::c_uint;
pub const BROTLI_NUM_TRANSFORM_TYPES: BrotliWordTransformType = 23;
pub const BROTLI_TRANSFORM_SHIFT_ALL: BrotliWordTransformType = 22;
pub const BROTLI_TRANSFORM_SHIFT_FIRST: BrotliWordTransformType = 21;
pub const BROTLI_TRANSFORM_OMIT_FIRST_9: BrotliWordTransformType = 20;
pub const BROTLI_TRANSFORM_OMIT_FIRST_8: BrotliWordTransformType = 19;
pub const BROTLI_TRANSFORM_OMIT_FIRST_7: BrotliWordTransformType = 18;
pub const BROTLI_TRANSFORM_OMIT_FIRST_6: BrotliWordTransformType = 17;
pub const BROTLI_TRANSFORM_OMIT_FIRST_5: BrotliWordTransformType = 16;
pub const BROTLI_TRANSFORM_OMIT_FIRST_4: BrotliWordTransformType = 15;
pub const BROTLI_TRANSFORM_OMIT_FIRST_3: BrotliWordTransformType = 14;
pub const BROTLI_TRANSFORM_OMIT_FIRST_2: BrotliWordTransformType = 13;
pub const BROTLI_TRANSFORM_OMIT_FIRST_1: BrotliWordTransformType = 12;
pub const BROTLI_TRANSFORM_UPPERCASE_ALL: BrotliWordTransformType = 11;
pub const BROTLI_TRANSFORM_OMIT_LAST_9: BrotliWordTransformType = 9;
pub const BROTLI_TRANSFORM_OMIT_LAST_8: BrotliWordTransformType = 8;
pub const BROTLI_TRANSFORM_OMIT_LAST_7: BrotliWordTransformType = 7;
pub const BROTLI_TRANSFORM_OMIT_LAST_6: BrotliWordTransformType = 6;
pub const BROTLI_TRANSFORM_OMIT_LAST_5: BrotliWordTransformType = 5;
pub const BROTLI_TRANSFORM_OMIT_LAST_4: BrotliWordTransformType = 4;
pub const BROTLI_TRANSFORM_OMIT_LAST_3: BrotliWordTransformType = 3;
pub const BROTLI_TRANSFORM_OMIT_LAST_2: BrotliWordTransformType = 2;
pub const BROTLI_TRANSFORM_OMIT_LAST_1: BrotliWordTransformType = 1;
pub const BROTLI_TRANSFORM_IDENTITY: BrotliWordTransformType = 0;
static mut kDictHashMul32: uint32_t = 0x1e35a7bd as libc::c_int as uint32_t;
static mut kDictNumBits: libc::c_int = 15 as libc::c_int;
#[inline(always)]
unsafe extern "C" fn brotli_min_uint32_t(mut a: uint32_t, mut b: uint32_t) -> uint32_t {
    return if a < b { a } else { b };
}
#[inline(always)]
unsafe extern "C" fn brotli_max_size_t(mut a: size_t, mut b: size_t) -> size_t {
    return if a > b { a } else { b };
}
#[inline(always)]
unsafe extern "C" fn brotli_min_size_t(mut a: size_t, mut b: size_t) -> size_t {
    return if a < b { a } else { b };
}
#[inline(always)]
unsafe extern "C" fn BrotliUnalignedRead32(mut p: *const libc::c_void) -> uint32_t {
    return *(p as *const uint32_t);
}
#[inline(always)]
unsafe extern "C" fn BrotliUnalignedRead64(mut p: *const libc::c_void) -> uint64_t {
    return *(p as *const uint64_t);
}
#[inline(always)]
unsafe extern "C" fn FindMatchLengthWithLimit(
    mut s1: *const uint8_t,
    mut s2: *const uint8_t,
    mut limit: size_t,
) -> size_t {
    let mut matched = 0 as libc::c_int as size_t;
    let mut limit2 = (limit >> 3 as libc::c_int)
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    loop {
        limit2= limit2.wrapping_sub(1);
        if !((limit2 != 0) as libc::c_int as libc::c_long != 0) {
            break;
        }
        if (BrotliUnalignedRead64(s2 as *const libc::c_void)
            == BrotliUnalignedRead64(s1.offset(matched as isize) as *const libc::c_void))
            as libc::c_int as libc::c_long != 0
        {
            s2= s2.offset(8 as libc::c_int as isize);
            matched= (matched as libc::c_ulong)
                .wrapping_add(8 as libc::c_int as libc::c_ulong) as size_t as size_t;
        } else {
            let mut x = BrotliUnalignedRead64(s2 as *const libc::c_void)
                ^ BrotliUnalignedRead64(
                    s1.offset(matched as isize) as *const libc::c_void,
                );
            let mut matching_bits = (x as libc::c_ulonglong).trailing_zeros() as i32
                as size_t;
            matched= (matched as libc::c_ulong)
                .wrapping_add(matching_bits >> 3 as libc::c_int) as size_t as size_t;
            return matched;
        }
    }
    limit= (limit & 7 as libc::c_int as libc::c_ulong)
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    loop {
        limit= limit.wrapping_sub(1);
        if !(limit != 0) {
            break;
        }
        if (*s1.offset(matched as isize) as libc::c_int == (*s2) as libc::c_int)
            as libc::c_int as libc::c_long != 0
        {
            s2= s2.offset(1);
            matched= matched.wrapping_add(1);
        } else {
            return matched
        }
    }
    return matched;
}
#[inline(always)]
unsafe extern "C" fn Hash(mut data: *const uint8_t) -> uint32_t {
    let mut h = (BrotliUnalignedRead32(data as *const libc::c_void))
        .wrapping_mul(crate::src::enc::static_dict::kDictHashMul32);
    return h >> 32 as libc::c_int - crate::src::enc::static_dict::kDictNumBits;
}
#[inline(always)]
unsafe extern "C" fn AddMatch(
    mut distance: size_t,
    mut len: size_t,
    mut len_code: size_t,
    mut matches: *mut uint32_t,
) {
    let mut match_0 = (distance << 5 as libc::c_int).wrapping_add(len_code) as uint32_t;
    *matches
        .offset(
            len as isize,
        ) = brotli_min_uint32_t(*matches.offset(len as isize), match_0);
}
#[inline(always)]
unsafe extern "C" fn DictMatchLength(
    mut dictionary: *const crate::src::common::dictionary::BrotliDictionary,
    mut data: *const uint8_t,
    mut id: size_t,
    mut len: size_t,
    mut maxlen: size_t,
) -> size_t {
    let offset = ((*dictionary).offsets_by_length[len as usize] as libc::c_ulong)
        .wrapping_add(len.wrapping_mul(id));
    return FindMatchLengthWithLimit(
        &*(*dictionary).data.offset(offset as isize),
        data,
        brotli_min_size_t(len, maxlen),
    );
}
#[inline(always)]
unsafe extern "C" fn IsMatch(
    mut dictionary: *const crate::src::common::dictionary::BrotliDictionary,
    mut w: crate::src::enc::backward_references::DictWord,
    mut data: *const uint8_t,
    mut max_length: size_t,
) -> libc::c_int {
    if w.len as libc::c_ulong > max_length {
        return 0 as libc::c_int
    } else {
        let offset = ((*dictionary).offsets_by_length[w.len as usize] as libc::c_ulong)
            .wrapping_add((w.len as size_t).wrapping_mul(w.idx as size_t));
        let mut dict: *const uint8_t = &*(*dictionary).data.offset(offset as isize)
            as *const uint8_t;
        if w.transform as libc::c_int == 0 as libc::c_int {
            return if FindMatchLengthWithLimit(dict, data, w.len as size_t)
                == w.len as libc::c_ulong
            {
                1 as libc::c_int
            } else {
                0 as libc::c_int
            }
        } else if w.transform as libc::c_int == 10 as libc::c_int {
            return if *dict.offset(0 as libc::c_int as isize) as libc::c_int
                >= 'a' as i32
                && *dict.offset(0 as libc::c_int as isize) as libc::c_int <= 'z' as i32
                && *dict.offset(0 as libc::c_int as isize) as libc::c_int
                    ^ 32 as libc::c_int
                    == *data.offset(0 as libc::c_int as isize) as libc::c_int
                && FindMatchLengthWithLimit(
                    &*dict.offset(1 as libc::c_int as isize),
                    &*data.offset(1 as libc::c_int as isize),
                    (w.len as libc::c_uint).wrapping_sub(1 as libc::c_uint) as size_t,
                )
                    == (w.len as libc::c_uint).wrapping_sub(1 as libc::c_uint)
                        as libc::c_ulong
            {
                1 as libc::c_int
            } else {
                0 as libc::c_int
            }
        } else {
            let mut i: size_t = 0;
            i= 0 as libc::c_int as size_t;
            while i < w.len as libc::c_ulong {
                if *dict.offset(i as isize) as libc::c_int >= 'a' as i32
                    && *dict.offset(i as isize) as libc::c_int <= 'z' as i32
                {
                    if *dict.offset(i as isize) as libc::c_int ^ 32 as libc::c_int
                        != *data.offset(i as isize) as libc::c_int
                    {
                        return 0 as libc::c_int;
                    }
                } else if *dict.offset(i as isize) as libc::c_int
                    != *data.offset(i as isize) as libc::c_int
                {
                    return 0 as libc::c_int
                }
                i= i.wrapping_add(1);
            }
            return 1 as libc::c_int;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn BrotliFindAllStaticDictionaryMatches(
    mut dictionary: *const crate::src::enc::backward_references::BrotliEncoderDictionary,
    mut data: *const uint8_t,
    mut min_length: size_t,
    mut max_length: size_t,
    mut matches: *mut uint32_t,
) -> libc::c_int {
    let mut has_found_match = 0 as libc::c_int;
    let mut offset = *(*dictionary).buckets.offset(Hash(data) as isize) as size_t;
    let mut end = (offset == 0) as libc::c_int;
    while end == 0 {
        let fresh0 = offset;
        offset= offset.wrapping_add(1);
        let mut w = *(*dictionary).dict_words.offset(fresh0 as isize);
        let l = (w.len as libc::c_int & 0x1f as libc::c_int) as size_t;
        let n = (1 as libc::c_int as size_t)
            << (*(*dictionary).words).size_bits_by_length[l as usize] as libc::c_int;
        let id = w.idx as size_t;
        end= (w.len as libc::c_int & 0x80 as libc::c_int != 0) as libc::c_int;
        w.len= l as uint8_t;
        if w.transform as libc::c_int == 0 as libc::c_int {
            let matchlen = DictMatchLength((*dictionary).words, data, id, l, max_length);
            let mut s = 0 as *const uint8_t;
            let mut minlen: size_t = 0;
            let mut maxlen: size_t = 0;
            let mut len: size_t = 0;
            if matchlen == l {
                AddMatch(id, l, l, matches);
                has_found_match= 1 as libc::c_int;
            }
            if matchlen >= l.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
                AddMatch(
                    id
                        .wrapping_add(
                            (12 as libc::c_int as libc::c_ulong).wrapping_mul(n),
                        ),
                    l.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    l,
                    matches,
                );
                if l.wrapping_add(2 as libc::c_int as libc::c_ulong) < max_length
                    && *data
                        .offset(
                            l.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        ) as libc::c_int == 'i' as i32
                    && *data.offset(l as isize) as libc::c_int == 'n' as i32
                    && *data
                        .offset(
                            l.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        ) as libc::c_int == 'g' as i32
                    && *data
                        .offset(
                            l.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize,
                        ) as libc::c_int == ' ' as i32
                {
                    AddMatch(
                        id
                            .wrapping_add(
                                (49 as libc::c_int as libc::c_ulong).wrapping_mul(n),
                            ),
                        l.wrapping_add(3 as libc::c_int as libc::c_ulong),
                        l,
                        matches,
                    );
                }
                has_found_match= 1 as libc::c_int;
            }
            minlen= min_length;
            if l > 9 as libc::c_int as libc::c_ulong {
                minlen= brotli_max_size_t(
                    minlen,
                    l.wrapping_sub(9 as libc::c_int as libc::c_ulong),
                );
            }
            maxlen= brotli_min_size_t(
                matchlen,
                l.wrapping_sub(2 as libc::c_int as libc::c_ulong),
            );
            len= minlen;
            while len <= maxlen {
                let mut cut = l.wrapping_sub(len);
                let mut transform_id = (cut << 2 as libc::c_int)
                    .wrapping_add(
                        (*dictionary).cutoffTransforms
                            >> cut.wrapping_mul(6 as libc::c_int as libc::c_ulong)
                            & 0x3f as libc::c_int as libc::c_ulong,
                    );
                AddMatch(id.wrapping_add(transform_id.wrapping_mul(n)), len, l, matches);
                has_found_match= 1 as libc::c_int;
                len= len.wrapping_add(1);
            }
            if matchlen < l
                || l.wrapping_add(6 as libc::c_int as libc::c_ulong) >= max_length
            {
                continue;
            }
            s= &*data.offset(l as isize) as *const uint8_t;
            if *s.offset(0 as libc::c_int as isize) as libc::c_int == ' ' as i32 {
                AddMatch(
                    id.wrapping_add(n),
                    l.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    l,
                    matches,
                );
                if *s.offset(1 as libc::c_int as isize) as libc::c_int == 'a' as i32 {
                    if *s.offset(2 as libc::c_int as isize) as libc::c_int == ' ' as i32
                    {
                        AddMatch(
                            id
                                .wrapping_add(
                                    (28 as libc::c_int as libc::c_ulong).wrapping_mul(n),
                                ),
                            l.wrapping_add(3 as libc::c_int as libc::c_ulong),
                            l,
                            matches,
                        );
                    } else if *s.offset(2 as libc::c_int as isize) as libc::c_int
                        == 's' as i32
                    {
                        if *s.offset(3 as libc::c_int as isize) as libc::c_int
                            == ' ' as i32
                        {
                            AddMatch(
                                id
                                    .wrapping_add(
                                        (46 as libc::c_int as libc::c_ulong).wrapping_mul(n),
                                    ),
                                l.wrapping_add(4 as libc::c_int as libc::c_ulong),
                                l,
                                matches,
                            );
                        }
                    } else if *s.offset(2 as libc::c_int as isize) as libc::c_int
                        == 't' as i32
                    {
                        if *s.offset(3 as libc::c_int as isize) as libc::c_int
                            == ' ' as i32
                        {
                            AddMatch(
                                id
                                    .wrapping_add(
                                        (60 as libc::c_int as libc::c_ulong).wrapping_mul(n),
                                    ),
                                l.wrapping_add(4 as libc::c_int as libc::c_ulong),
                                l,
                                matches,
                            );
                        }
                    } else if *s.offset(2 as libc::c_int as isize) as libc::c_int
                        == 'n' as i32
                    {
                        if *s.offset(3 as libc::c_int as isize) as libc::c_int
                            == 'd' as i32
                            && *s.offset(4 as libc::c_int as isize) as libc::c_int
                                == ' ' as i32
                        {
                            AddMatch(
                                id
                                    .wrapping_add(
                                        (10 as libc::c_int as libc::c_ulong).wrapping_mul(n),
                                    ),
                                l.wrapping_add(5 as libc::c_int as libc::c_ulong),
                                l,
                                matches,
                            );
                        }
                    }
                } else if *s.offset(1 as libc::c_int as isize) as libc::c_int
                    == 'b' as i32
                {
                    if *s.offset(2 as libc::c_int as isize) as libc::c_int == 'y' as i32
                        && *s.offset(3 as libc::c_int as isize) as libc::c_int
                            == ' ' as i32
                    {
                        AddMatch(
                            id
                                .wrapping_add(
                                    (38 as libc::c_int as libc::c_ulong).wrapping_mul(n),
                                ),
                            l.wrapping_add(4 as libc::c_int as libc::c_ulong),
                            l,
                            matches,
                        );
                    }
                } else if *s.offset(1 as libc::c_int as isize) as libc::c_int
                    == 'i' as i32
                {
                    if *s.offset(2 as libc::c_int as isize) as libc::c_int == 'n' as i32
                    {
                        if *s.offset(3 as libc::c_int as isize) as libc::c_int
                            == ' ' as i32
                        {
                            AddMatch(
                                id
                                    .wrapping_add(
                                        (16 as libc::c_int as libc::c_ulong).wrapping_mul(n),
                                    ),
                                l.wrapping_add(4 as libc::c_int as libc::c_ulong),
                                l,
                                matches,
                            );
                        }
                    } else if *s.offset(2 as libc::c_int as isize) as libc::c_int
                        == 's' as i32
                    {
                        if *s.offset(3 as libc::c_int as isize) as libc::c_int
                            == ' ' as i32
                        {
                            AddMatch(
                                id
                                    .wrapping_add(
                                        (47 as libc::c_int as libc::c_ulong).wrapping_mul(n),
                                    ),
                                l.wrapping_add(4 as libc::c_int as libc::c_ulong),
                                l,
                                matches,
                            );
                        }
                    }
                } else if *s.offset(1 as libc::c_int as isize) as libc::c_int
                    == 'f' as i32
                {
                    if *s.offset(2 as libc::c_int as isize) as libc::c_int == 'o' as i32
                    {
                        if *s.offset(3 as libc::c_int as isize) as libc::c_int
                            == 'r' as i32
                            && *s.offset(4 as libc::c_int as isize) as libc::c_int
                                == ' ' as i32
                        {
                            AddMatch(
                                id
                                    .wrapping_add(
                                        (25 as libc::c_int as libc::c_ulong).wrapping_mul(n),
                                    ),
                                l.wrapping_add(5 as libc::c_int as libc::c_ulong),
                                l,
                                matches,
                            );
                        }
                    } else if *s.offset(2 as libc::c_int as isize) as libc::c_int
                        == 'r' as i32
                    {
                        if *s.offset(3 as libc::c_int as isize) as libc::c_int
                            == 'o' as i32
                            && *s.offset(4 as libc::c_int as isize) as libc::c_int
                                == 'm' as i32
                            && *s.offset(5 as libc::c_int as isize) as libc::c_int
                                == ' ' as i32
                        {
                            AddMatch(
                                id
                                    .wrapping_add(
                                        (37 as libc::c_int as libc::c_ulong).wrapping_mul(n),
                                    ),
                                l.wrapping_add(6 as libc::c_int as libc::c_ulong),
                                l,
                                matches,
                            );
                        }
                    }
                } else if *s.offset(1 as libc::c_int as isize) as libc::c_int
                    == 'o' as i32
                {
                    if *s.offset(2 as libc::c_int as isize) as libc::c_int == 'f' as i32
                    {
                        if *s.offset(3 as libc::c_int as isize) as libc::c_int
                            == ' ' as i32
                        {
                            AddMatch(
                                id
                                    .wrapping_add(
                                        (8 as libc::c_int as libc::c_ulong).wrapping_mul(n),
                                    ),
                                l.wrapping_add(4 as libc::c_int as libc::c_ulong),
                                l,
                                matches,
                            );
                        }
                    } else if *s.offset(2 as libc::c_int as isize) as libc::c_int
                        == 'n' as i32
                    {
                        if *s.offset(3 as libc::c_int as isize) as libc::c_int
                            == ' ' as i32
                        {
                            AddMatch(
                                id
                                    .wrapping_add(
                                        (45 as libc::c_int as libc::c_ulong).wrapping_mul(n),
                                    ),
                                l.wrapping_add(4 as libc::c_int as libc::c_ulong),
                                l,
                                matches,
                            );
                        }
                    }
                } else if *s.offset(1 as libc::c_int as isize) as libc::c_int
                    == 'n' as i32
                {
                    if *s.offset(2 as libc::c_int as isize) as libc::c_int == 'o' as i32
                        && *s.offset(3 as libc::c_int as isize) as libc::c_int
                            == 't' as i32
                        && *s.offset(4 as libc::c_int as isize) as libc::c_int
                            == ' ' as i32
                    {
                        AddMatch(
                            id
                                .wrapping_add(
                                    (80 as libc::c_int as libc::c_ulong).wrapping_mul(n),
                                ),
                            l.wrapping_add(5 as libc::c_int as libc::c_ulong),
                            l,
                            matches,
                        );
                    }
                } else if *s.offset(1 as libc::c_int as isize) as libc::c_int
                    == 't' as i32
                {
                    if *s.offset(2 as libc::c_int as isize) as libc::c_int == 'h' as i32
                    {
                        if *s.offset(3 as libc::c_int as isize) as libc::c_int
                            == 'e' as i32
                        {
                            if *s.offset(4 as libc::c_int as isize) as libc::c_int
                                == ' ' as i32
                            {
                                AddMatch(
                                    id
                                        .wrapping_add(
                                            (5 as libc::c_int as libc::c_ulong).wrapping_mul(n),
                                        ),
                                    l.wrapping_add(5 as libc::c_int as libc::c_ulong),
                                    l,
                                    matches,
                                );
                            }
                        } else if *s.offset(3 as libc::c_int as isize) as libc::c_int
                            == 'a' as i32
                        {
                            if *s.offset(4 as libc::c_int as isize) as libc::c_int
                                == 't' as i32
                                && *s.offset(5 as libc::c_int as isize) as libc::c_int
                                    == ' ' as i32
                            {
                                AddMatch(
                                    id
                                        .wrapping_add(
                                            (29 as libc::c_int as libc::c_ulong).wrapping_mul(n),
                                        ),
                                    l.wrapping_add(6 as libc::c_int as libc::c_ulong),
                                    l,
                                    matches,
                                );
                            }
                        }
                    } else if *s.offset(2 as libc::c_int as isize) as libc::c_int
                        == 'o' as i32
                    {
                        if *s.offset(3 as libc::c_int as isize) as libc::c_int
                            == ' ' as i32
                        {
                            AddMatch(
                                id
                                    .wrapping_add(
                                        (17 as libc::c_int as libc::c_ulong).wrapping_mul(n),
                                    ),
                                l.wrapping_add(4 as libc::c_int as libc::c_ulong),
                                l,
                                matches,
                            );
                        }
                    }
                } else if *s.offset(1 as libc::c_int as isize) as libc::c_int
                    == 'w' as i32
                {
                    if *s.offset(2 as libc::c_int as isize) as libc::c_int == 'i' as i32
                        && *s.offset(3 as libc::c_int as isize) as libc::c_int
                            == 't' as i32
                        && *s.offset(4 as libc::c_int as isize) as libc::c_int
                            == 'h' as i32
                        && *s.offset(5 as libc::c_int as isize) as libc::c_int
                            == ' ' as i32
                    {
                        AddMatch(
                            id
                                .wrapping_add(
                                    (35 as libc::c_int as libc::c_ulong).wrapping_mul(n),
                                ),
                            l.wrapping_add(6 as libc::c_int as libc::c_ulong),
                            l,
                            matches,
                        );
                    }
                }
            } else if *s.offset(0 as libc::c_int as isize) as libc::c_int == '"' as i32 {
                AddMatch(
                    id
                        .wrapping_add(
                            (19 as libc::c_int as libc::c_ulong).wrapping_mul(n),
                        ),
                    l.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    l,
                    matches,
                );
                if *s.offset(1 as libc::c_int as isize) as libc::c_int == '>' as i32 {
                    AddMatch(
                        id
                            .wrapping_add(
                                (21 as libc::c_int as libc::c_ulong).wrapping_mul(n),
                            ),
                        l.wrapping_add(2 as libc::c_int as libc::c_ulong),
                        l,
                        matches,
                    );
                }
            } else if *s.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32 {
                AddMatch(
                    id
                        .wrapping_add(
                            (20 as libc::c_int as libc::c_ulong).wrapping_mul(n),
                        ),
                    l.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    l,
                    matches,
                );
                if *s.offset(1 as libc::c_int as isize) as libc::c_int == ' ' as i32 {
                    AddMatch(
                        id
                            .wrapping_add(
                                (31 as libc::c_int as libc::c_ulong).wrapping_mul(n),
                            ),
                        l.wrapping_add(2 as libc::c_int as libc::c_ulong),
                        l,
                        matches,
                    );
                    if *s.offset(2 as libc::c_int as isize) as libc::c_int == 'T' as i32
                        && *s.offset(3 as libc::c_int as isize) as libc::c_int
                            == 'h' as i32
                    {
                        if *s.offset(4 as libc::c_int as isize) as libc::c_int
                            == 'e' as i32
                        {
                            if *s.offset(5 as libc::c_int as isize) as libc::c_int
                                == ' ' as i32
                            {
                                AddMatch(
                                    id
                                        .wrapping_add(
                                            (43 as libc::c_int as libc::c_ulong).wrapping_mul(n),
                                        ),
                                    l.wrapping_add(6 as libc::c_int as libc::c_ulong),
                                    l,
                                    matches,
                                );
                            }
                        } else if *s.offset(4 as libc::c_int as isize) as libc::c_int
                            == 'i' as i32
                        {
                            if *s.offset(5 as libc::c_int as isize) as libc::c_int
                                == 's' as i32
                                && *s.offset(6 as libc::c_int as isize) as libc::c_int
                                    == ' ' as i32
                            {
                                AddMatch(
                                    id
                                        .wrapping_add(
                                            (75 as libc::c_int as libc::c_ulong).wrapping_mul(n),
                                        ),
                                    l.wrapping_add(7 as libc::c_int as libc::c_ulong),
                                    l,
                                    matches,
                                );
                            }
                        }
                    }
                }
            } else if *s.offset(0 as libc::c_int as isize) as libc::c_int == ',' as i32 {
                AddMatch(
                    id
                        .wrapping_add(
                            (76 as libc::c_int as libc::c_ulong).wrapping_mul(n),
                        ),
                    l.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    l,
                    matches,
                );
                if *s.offset(1 as libc::c_int as isize) as libc::c_int == ' ' as i32 {
                    AddMatch(
                        id
                            .wrapping_add(
                                (14 as libc::c_int as libc::c_ulong).wrapping_mul(n),
                            ),
                        l.wrapping_add(2 as libc::c_int as libc::c_ulong),
                        l,
                        matches,
                    );
                }
            } else if *s.offset(0 as libc::c_int as isize) as libc::c_int == '\n' as i32
            {
                AddMatch(
                    id
                        .wrapping_add(
                            (22 as libc::c_int as libc::c_ulong).wrapping_mul(n),
                        ),
                    l.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    l,
                    matches,
                );
                if *s.offset(1 as libc::c_int as isize) as libc::c_int == '\t' as i32 {
                    AddMatch(
                        id
                            .wrapping_add(
                                (50 as libc::c_int as libc::c_ulong).wrapping_mul(n),
                            ),
                        l.wrapping_add(2 as libc::c_int as libc::c_ulong),
                        l,
                        matches,
                    );
                }
            } else if *s.offset(0 as libc::c_int as isize) as libc::c_int == ']' as i32 {
                AddMatch(
                    id
                        .wrapping_add(
                            (24 as libc::c_int as libc::c_ulong).wrapping_mul(n),
                        ),
                    l.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    l,
                    matches,
                );
            } else if *s.offset(0 as libc::c_int as isize) as libc::c_int == '\'' as i32
            {
                AddMatch(
                    id
                        .wrapping_add(
                            (36 as libc::c_int as libc::c_ulong).wrapping_mul(n),
                        ),
                    l.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    l,
                    matches,
                );
            } else if *s.offset(0 as libc::c_int as isize) as libc::c_int == ':' as i32 {
                AddMatch(
                    id
                        .wrapping_add(
                            (51 as libc::c_int as libc::c_ulong).wrapping_mul(n),
                        ),
                    l.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    l,
                    matches,
                );
            } else if *s.offset(0 as libc::c_int as isize) as libc::c_int == '(' as i32 {
                AddMatch(
                    id
                        .wrapping_add(
                            (57 as libc::c_int as libc::c_ulong).wrapping_mul(n),
                        ),
                    l.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    l,
                    matches,
                );
            } else if *s.offset(0 as libc::c_int as isize) as libc::c_int == '=' as i32 {
                if *s.offset(1 as libc::c_int as isize) as libc::c_int == '"' as i32 {
                    AddMatch(
                        id
                            .wrapping_add(
                                (70 as libc::c_int as libc::c_ulong).wrapping_mul(n),
                            ),
                        l.wrapping_add(2 as libc::c_int as libc::c_ulong),
                        l,
                        matches,
                    );
                } else if *s.offset(1 as libc::c_int as isize) as libc::c_int
                    == '\'' as i32
                {
                    AddMatch(
                        id
                            .wrapping_add(
                                (86 as libc::c_int as libc::c_ulong).wrapping_mul(n),
                            ),
                        l.wrapping_add(2 as libc::c_int as libc::c_ulong),
                        l,
                        matches,
                    );
                }
            } else if *s.offset(0 as libc::c_int as isize) as libc::c_int == 'a' as i32 {
                if *s.offset(1 as libc::c_int as isize) as libc::c_int == 'l' as i32
                    && *s.offset(2 as libc::c_int as isize) as libc::c_int == ' ' as i32
                {
                    AddMatch(
                        id
                            .wrapping_add(
                                (84 as libc::c_int as libc::c_ulong).wrapping_mul(n),
                            ),
                        l.wrapping_add(3 as libc::c_int as libc::c_ulong),
                        l,
                        matches,
                    );
                }
            } else if *s.offset(0 as libc::c_int as isize) as libc::c_int == 'e' as i32 {
                if *s.offset(1 as libc::c_int as isize) as libc::c_int == 'd' as i32 {
                    if *s.offset(2 as libc::c_int as isize) as libc::c_int == ' ' as i32
                    {
                        AddMatch(
                            id
                                .wrapping_add(
                                    (53 as libc::c_int as libc::c_ulong).wrapping_mul(n),
                                ),
                            l.wrapping_add(3 as libc::c_int as libc::c_ulong),
                            l,
                            matches,
                        );
                    }
                } else if *s.offset(1 as libc::c_int as isize) as libc::c_int
                    == 'r' as i32
                {
                    if *s.offset(2 as libc::c_int as isize) as libc::c_int == ' ' as i32
                    {
                        AddMatch(
                            id
                                .wrapping_add(
                                    (82 as libc::c_int as libc::c_ulong).wrapping_mul(n),
                                ),
                            l.wrapping_add(3 as libc::c_int as libc::c_ulong),
                            l,
                            matches,
                        );
                    }
                } else if *s.offset(1 as libc::c_int as isize) as libc::c_int
                    == 's' as i32
                {
                    if *s.offset(2 as libc::c_int as isize) as libc::c_int == 't' as i32
                        && *s.offset(3 as libc::c_int as isize) as libc::c_int
                            == ' ' as i32
                    {
                        AddMatch(
                            id
                                .wrapping_add(
                                    (95 as libc::c_int as libc::c_ulong).wrapping_mul(n),
                                ),
                            l.wrapping_add(4 as libc::c_int as libc::c_ulong),
                            l,
                            matches,
                        );
                    }
                }
            } else if *s.offset(0 as libc::c_int as isize) as libc::c_int == 'f' as i32 {
                if *s.offset(1 as libc::c_int as isize) as libc::c_int == 'u' as i32
                    && *s.offset(2 as libc::c_int as isize) as libc::c_int == 'l' as i32
                    && *s.offset(3 as libc::c_int as isize) as libc::c_int == ' ' as i32
                {
                    AddMatch(
                        id
                            .wrapping_add(
                                (90 as libc::c_int as libc::c_ulong).wrapping_mul(n),
                            ),
                        l.wrapping_add(4 as libc::c_int as libc::c_ulong),
                        l,
                        matches,
                    );
                }
            } else if *s.offset(0 as libc::c_int as isize) as libc::c_int == 'i' as i32 {
                if *s.offset(1 as libc::c_int as isize) as libc::c_int == 'v' as i32 {
                    if *s.offset(2 as libc::c_int as isize) as libc::c_int == 'e' as i32
                        && *s.offset(3 as libc::c_int as isize) as libc::c_int
                            == ' ' as i32
                    {
                        AddMatch(
                            id
                                .wrapping_add(
                                    (92 as libc::c_int as libc::c_ulong).wrapping_mul(n),
                                ),
                            l.wrapping_add(4 as libc::c_int as libc::c_ulong),
                            l,
                            matches,
                        );
                    }
                } else if *s.offset(1 as libc::c_int as isize) as libc::c_int
                    == 'z' as i32
                {
                    if *s.offset(2 as libc::c_int as isize) as libc::c_int == 'e' as i32
                        && *s.offset(3 as libc::c_int as isize) as libc::c_int
                            == ' ' as i32
                    {
                        AddMatch(
                            id
                                .wrapping_add(
                                    (100 as libc::c_int as libc::c_ulong).wrapping_mul(n),
                                ),
                            l.wrapping_add(4 as libc::c_int as libc::c_ulong),
                            l,
                            matches,
                        );
                    }
                }
            } else if *s.offset(0 as libc::c_int as isize) as libc::c_int == 'l' as i32 {
                if *s.offset(1 as libc::c_int as isize) as libc::c_int == 'e' as i32 {
                    if *s.offset(2 as libc::c_int as isize) as libc::c_int == 's' as i32
                        && *s.offset(3 as libc::c_int as isize) as libc::c_int
                            == 's' as i32
                        && *s.offset(4 as libc::c_int as isize) as libc::c_int
                            == ' ' as i32
                    {
                        AddMatch(
                            id
                                .wrapping_add(
                                    (93 as libc::c_int as libc::c_ulong).wrapping_mul(n),
                                ),
                            l.wrapping_add(5 as libc::c_int as libc::c_ulong),
                            l,
                            matches,
                        );
                    }
                } else if *s.offset(1 as libc::c_int as isize) as libc::c_int
                    == 'y' as i32
                {
                    if *s.offset(2 as libc::c_int as isize) as libc::c_int == ' ' as i32
                    {
                        AddMatch(
                            id
                                .wrapping_add(
                                    (61 as libc::c_int as libc::c_ulong).wrapping_mul(n),
                                ),
                            l.wrapping_add(3 as libc::c_int as libc::c_ulong),
                            l,
                            matches,
                        );
                    }
                }
            } else if *s.offset(0 as libc::c_int as isize) as libc::c_int == 'o' as i32 {
                if *s.offset(1 as libc::c_int as isize) as libc::c_int == 'u' as i32
                    && *s.offset(2 as libc::c_int as isize) as libc::c_int == 's' as i32
                    && *s.offset(3 as libc::c_int as isize) as libc::c_int == ' ' as i32
                {
                    AddMatch(
                        id
                            .wrapping_add(
                                (106 as libc::c_int as libc::c_ulong).wrapping_mul(n),
                            ),
                        l.wrapping_add(4 as libc::c_int as libc::c_ulong),
                        l,
                        matches,
                    );
                }
            }
        } else {
            let is_all_caps = if w.transform as libc::c_int
                != BROTLI_TRANSFORM_UPPERCASE_FIRST as libc::c_int
            {
                1 as libc::c_int
            } else {
                0 as libc::c_int
            };
            let mut s_0 = 0 as *const uint8_t;
            if IsMatch((*dictionary).words, w, data, max_length) == 0 {
                continue;
            }
            AddMatch(
                id
                    .wrapping_add(
                        ((if is_all_caps != 0 {
                            44 as libc::c_int
                        } else {
                            9 as libc::c_int
                        }) as libc::c_ulong)
                            .wrapping_mul(n),
                    ),
                l,
                l,
                matches,
            );
            has_found_match= 1 as libc::c_int;
            if l.wrapping_add(1 as libc::c_int as libc::c_ulong) >= max_length {
                continue;
            }
            s_0= &*data.offset(l as isize) as *const uint8_t;
            if *s_0.offset(0 as libc::c_int as isize) as libc::c_int == ' ' as i32 {
                AddMatch(
                    id
                        .wrapping_add(
                            ((if is_all_caps != 0 {
                                68 as libc::c_int
                            } else {
                                4 as libc::c_int
                            }) as libc::c_ulong)
                                .wrapping_mul(n),
                        ),
                    l.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    l,
                    matches,
                );
            } else if *s_0.offset(0 as libc::c_int as isize) as libc::c_int == '"' as i32
            {
                AddMatch(
                    id
                        .wrapping_add(
                            ((if is_all_caps != 0 {
                                87 as libc::c_int
                            } else {
                                66 as libc::c_int
                            }) as libc::c_ulong)
                                .wrapping_mul(n),
                        ),
                    l.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    l,
                    matches,
                );
                if *s_0.offset(1 as libc::c_int as isize) as libc::c_int == '>' as i32 {
                    AddMatch(
                        id
                            .wrapping_add(
                                ((if is_all_caps != 0 {
                                    97 as libc::c_int
                                } else {
                                    69 as libc::c_int
                                }) as libc::c_ulong)
                                    .wrapping_mul(n),
                            ),
                        l.wrapping_add(2 as libc::c_int as libc::c_ulong),
                        l,
                        matches,
                    );
                }
            } else if *s_0.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32
            {
                AddMatch(
                    id
                        .wrapping_add(
                            ((if is_all_caps != 0 {
                                101 as libc::c_int
                            } else {
                                79 as libc::c_int
                            }) as libc::c_ulong)
                                .wrapping_mul(n),
                        ),
                    l.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    l,
                    matches,
                );
                if *s_0.offset(1 as libc::c_int as isize) as libc::c_int == ' ' as i32 {
                    AddMatch(
                        id
                            .wrapping_add(
                                ((if is_all_caps != 0 {
                                    114 as libc::c_int
                                } else {
                                    88 as libc::c_int
                                }) as libc::c_ulong)
                                    .wrapping_mul(n),
                            ),
                        l.wrapping_add(2 as libc::c_int as libc::c_ulong),
                        l,
                        matches,
                    );
                }
            } else if *s_0.offset(0 as libc::c_int as isize) as libc::c_int == ',' as i32
            {
                AddMatch(
                    id
                        .wrapping_add(
                            ((if is_all_caps != 0 {
                                112 as libc::c_int
                            } else {
                                99 as libc::c_int
                            }) as libc::c_ulong)
                                .wrapping_mul(n),
                        ),
                    l.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    l,
                    matches,
                );
                if *s_0.offset(1 as libc::c_int as isize) as libc::c_int == ' ' as i32 {
                    AddMatch(
                        id
                            .wrapping_add(
                                ((if is_all_caps != 0 {
                                    107 as libc::c_int
                                } else {
                                    58 as libc::c_int
                                }) as libc::c_ulong)
                                    .wrapping_mul(n),
                            ),
                        l.wrapping_add(2 as libc::c_int as libc::c_ulong),
                        l,
                        matches,
                    );
                }
            } else if *s_0.offset(0 as libc::c_int as isize) as libc::c_int
                == '\'' as i32
            {
                AddMatch(
                    id
                        .wrapping_add(
                            ((if is_all_caps != 0 {
                                94 as libc::c_int
                            } else {
                                74 as libc::c_int
                            }) as libc::c_ulong)
                                .wrapping_mul(n),
                        ),
                    l.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    l,
                    matches,
                );
            } else if *s_0.offset(0 as libc::c_int as isize) as libc::c_int == '(' as i32
            {
                AddMatch(
                    id
                        .wrapping_add(
                            ((if is_all_caps != 0 {
                                113 as libc::c_int
                            } else {
                                78 as libc::c_int
                            }) as libc::c_ulong)
                                .wrapping_mul(n),
                        ),
                    l.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    l,
                    matches,
                );
            } else if *s_0.offset(0 as libc::c_int as isize) as libc::c_int == '=' as i32
            {
                if *s_0.offset(1 as libc::c_int as isize) as libc::c_int == '"' as i32 {
                    AddMatch(
                        id
                            .wrapping_add(
                                ((if is_all_caps != 0 {
                                    105 as libc::c_int
                                } else {
                                    104 as libc::c_int
                                }) as libc::c_ulong)
                                    .wrapping_mul(n),
                            ),
                        l.wrapping_add(2 as libc::c_int as libc::c_ulong),
                        l,
                        matches,
                    );
                } else if *s_0.offset(1 as libc::c_int as isize) as libc::c_int
                    == '\'' as i32
                {
                    AddMatch(
                        id
                            .wrapping_add(
                                ((if is_all_caps != 0 {
                                    116 as libc::c_int
                                } else {
                                    108 as libc::c_int
                                }) as libc::c_ulong)
                                    .wrapping_mul(n),
                            ),
                        l.wrapping_add(2 as libc::c_int as libc::c_ulong),
                        l,
                        matches,
                    );
                }
            }
        }
    }
    if max_length >= 5 as libc::c_int as libc::c_ulong
        && (*data.offset(0 as libc::c_int as isize) as libc::c_int == ' ' as i32
            || *data.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32)
    {
        let mut is_space = if *data.offset(0 as libc::c_int as isize) as libc::c_int
            == ' ' as i32
        {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        };
        let mut offset_0 = *(*dictionary).buckets
            .offset(Hash(&*data.offset(1 as libc::c_int as isize)) as isize) as size_t;
        let mut end_0 = (offset_0 == 0) as libc::c_int;
        while end_0 == 0 {
            let fresh1 = offset_0;
            offset_0= offset_0.wrapping_add(1);
            let mut w_0 = *(*dictionary).dict_words.offset(fresh1 as isize);
            let l_0 = (w_0.len as libc::c_int & 0x1f as libc::c_int) as size_t;
            let n_0 = (1 as libc::c_int as size_t)
                << (*(*dictionary).words).size_bits_by_length[l_0 as usize]
                    as libc::c_int;
            let id_0 = w_0.idx as size_t;
            end_0= (w_0.len as libc::c_int & 0x80 as libc::c_int != 0) as libc::c_int;
            w_0.len= l_0 as uint8_t;
            if w_0.transform as libc::c_int == 0 as libc::c_int {
                let mut s_1 = 0 as *const uint8_t;
                if IsMatch(
                    (*dictionary).words,
                    w_0,
                    &*data.offset(1 as libc::c_int as isize),
                    max_length.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                ) == 0
                {
                    continue;
                }
                AddMatch(
                    id_0
                        .wrapping_add(
                            ((if is_space != 0 {
                                6 as libc::c_int
                            } else {
                                32 as libc::c_int
                            }) as libc::c_ulong)
                                .wrapping_mul(n_0),
                        ),
                    l_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    l_0,
                    matches,
                );
                has_found_match= 1 as libc::c_int;
                if l_0.wrapping_add(2 as libc::c_int as libc::c_ulong) >= max_length {
                    continue;
                }
                s_1= &*data
                    .offset(l_0.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
                    as *const uint8_t;
                if *s_1.offset(0 as libc::c_int as isize) as libc::c_int == ' ' as i32 {
                    AddMatch(
                        id_0
                            .wrapping_add(
                                ((if is_space != 0 {
                                    2 as libc::c_int
                                } else {
                                    77 as libc::c_int
                                }) as libc::c_ulong)
                                    .wrapping_mul(n_0),
                            ),
                        l_0.wrapping_add(2 as libc::c_int as libc::c_ulong),
                        l_0,
                        matches,
                    );
                } else if *s_1.offset(0 as libc::c_int as isize) as libc::c_int
                    == '(' as i32
                {
                    AddMatch(
                        id_0
                            .wrapping_add(
                                ((if is_space != 0 {
                                    89 as libc::c_int
                                } else {
                                    67 as libc::c_int
                                }) as libc::c_ulong)
                                    .wrapping_mul(n_0),
                            ),
                        l_0.wrapping_add(2 as libc::c_int as libc::c_ulong),
                        l_0,
                        matches,
                    );
                } else if is_space != 0 {
                    if *s_1.offset(0 as libc::c_int as isize) as libc::c_int
                        == ',' as i32
                    {
                        AddMatch(
                            id_0
                                .wrapping_add(
                                    (103 as libc::c_int as libc::c_ulong).wrapping_mul(n_0),
                                ),
                            l_0.wrapping_add(2 as libc::c_int as libc::c_ulong),
                            l_0,
                            matches,
                        );
                        if *s_1.offset(1 as libc::c_int as isize) as libc::c_int
                            == ' ' as i32
                        {
                            AddMatch(
                                id_0
                                    .wrapping_add(
                                        (33 as libc::c_int as libc::c_ulong).wrapping_mul(n_0),
                                    ),
                                l_0.wrapping_add(3 as libc::c_int as libc::c_ulong),
                                l_0,
                                matches,
                            );
                        }
                    } else if *s_1.offset(0 as libc::c_int as isize) as libc::c_int
                        == '.' as i32
                    {
                        AddMatch(
                            id_0
                                .wrapping_add(
                                    (71 as libc::c_int as libc::c_ulong).wrapping_mul(n_0),
                                ),
                            l_0.wrapping_add(2 as libc::c_int as libc::c_ulong),
                            l_0,
                            matches,
                        );
                        if *s_1.offset(1 as libc::c_int as isize) as libc::c_int
                            == ' ' as i32
                        {
                            AddMatch(
                                id_0
                                    .wrapping_add(
                                        (52 as libc::c_int as libc::c_ulong).wrapping_mul(n_0),
                                    ),
                                l_0.wrapping_add(3 as libc::c_int as libc::c_ulong),
                                l_0,
                                matches,
                            );
                        }
                    } else if *s_1.offset(0 as libc::c_int as isize) as libc::c_int
                        == '=' as i32
                    {
                        if *s_1.offset(1 as libc::c_int as isize) as libc::c_int
                            == '"' as i32
                        {
                            AddMatch(
                                id_0
                                    .wrapping_add(
                                        (81 as libc::c_int as libc::c_ulong).wrapping_mul(n_0),
                                    ),
                                l_0.wrapping_add(3 as libc::c_int as libc::c_ulong),
                                l_0,
                                matches,
                            );
                        } else if *s_1.offset(1 as libc::c_int as isize) as libc::c_int
                            == '\'' as i32
                        {
                            AddMatch(
                                id_0
                                    .wrapping_add(
                                        (98 as libc::c_int as libc::c_ulong).wrapping_mul(n_0),
                                    ),
                                l_0.wrapping_add(3 as libc::c_int as libc::c_ulong),
                                l_0,
                                matches,
                            );
                        }
                    }
                }
            } else {
                if !(is_space != 0) {
                    continue;
                }
                let is_all_caps_0 = if w_0.transform as libc::c_int
                    != BROTLI_TRANSFORM_UPPERCASE_FIRST as libc::c_int
                {
                    1 as libc::c_int
                } else {
                    0 as libc::c_int
                };
                let mut s_2 = 0 as *const uint8_t;
                if IsMatch(
                    (*dictionary).words,
                    w_0,
                    &*data.offset(1 as libc::c_int as isize),
                    max_length.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                ) == 0
                {
                    continue;
                }
                AddMatch(
                    id_0
                        .wrapping_add(
                            ((if is_all_caps_0 != 0 {
                                85 as libc::c_int
                            } else {
                                30 as libc::c_int
                            }) as libc::c_ulong)
                                .wrapping_mul(n_0),
                        ),
                    l_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    l_0,
                    matches,
                );
                has_found_match= 1 as libc::c_int;
                if l_0.wrapping_add(2 as libc::c_int as libc::c_ulong) >= max_length {
                    continue;
                }
                s_2= &*data
                    .offset(l_0.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
                    as *const uint8_t;
                if *s_2.offset(0 as libc::c_int as isize) as libc::c_int == ' ' as i32 {
                    AddMatch(
                        id_0
                            .wrapping_add(
                                ((if is_all_caps_0 != 0 {
                                    83 as libc::c_int
                                } else {
                                    15 as libc::c_int
                                }) as libc::c_ulong)
                                    .wrapping_mul(n_0),
                            ),
                        l_0.wrapping_add(2 as libc::c_int as libc::c_ulong),
                        l_0,
                        matches,
                    );
                } else if *s_2.offset(0 as libc::c_int as isize) as libc::c_int
                    == ',' as i32
                {
                    if is_all_caps_0 == 0 {
                        AddMatch(
                            id_0
                                .wrapping_add(
                                    (109 as libc::c_int as libc::c_ulong).wrapping_mul(n_0),
                                ),
                            l_0.wrapping_add(2 as libc::c_int as libc::c_ulong),
                            l_0,
                            matches,
                        );
                    }
                    if *s_2.offset(1 as libc::c_int as isize) as libc::c_int
                        == ' ' as i32
                    {
                        AddMatch(
                            id_0
                                .wrapping_add(
                                    ((if is_all_caps_0 != 0 {
                                        111 as libc::c_int
                                    } else {
                                        65 as libc::c_int
                                    }) as libc::c_ulong)
                                        .wrapping_mul(n_0),
                                ),
                            l_0.wrapping_add(3 as libc::c_int as libc::c_ulong),
                            l_0,
                            matches,
                        );
                    }
                } else if *s_2.offset(0 as libc::c_int as isize) as libc::c_int
                    == '.' as i32
                {
                    AddMatch(
                        id_0
                            .wrapping_add(
                                ((if is_all_caps_0 != 0 {
                                    115 as libc::c_int
                                } else {
                                    96 as libc::c_int
                                }) as libc::c_ulong)
                                    .wrapping_mul(n_0),
                            ),
                        l_0.wrapping_add(2 as libc::c_int as libc::c_ulong),
                        l_0,
                        matches,
                    );
                    if *s_2.offset(1 as libc::c_int as isize) as libc::c_int
                        == ' ' as i32
                    {
                        AddMatch(
                            id_0
                                .wrapping_add(
                                    ((if is_all_caps_0 != 0 {
                                        117 as libc::c_int
                                    } else {
                                        91 as libc::c_int
                                    }) as libc::c_ulong)
                                        .wrapping_mul(n_0),
                                ),
                            l_0.wrapping_add(3 as libc::c_int as libc::c_ulong),
                            l_0,
                            matches,
                        );
                    }
                } else if *s_2.offset(0 as libc::c_int as isize) as libc::c_int
                    == '=' as i32
                {
                    if *s_2.offset(1 as libc::c_int as isize) as libc::c_int
                        == '"' as i32
                    {
                        AddMatch(
                            id_0
                                .wrapping_add(
                                    ((if is_all_caps_0 != 0 {
                                        110 as libc::c_int
                                    } else {
                                        118 as libc::c_int
                                    }) as libc::c_ulong)
                                        .wrapping_mul(n_0),
                                ),
                            l_0.wrapping_add(3 as libc::c_int as libc::c_ulong),
                            l_0,
                            matches,
                        );
                    } else if *s_2.offset(1 as libc::c_int as isize) as libc::c_int
                        == '\'' as i32
                    {
                        AddMatch(
                            id_0
                                .wrapping_add(
                                    ((if is_all_caps_0 != 0 {
                                        119 as libc::c_int
                                    } else {
                                        120 as libc::c_int
                                    }) as libc::c_ulong)
                                        .wrapping_mul(n_0),
                                ),
                            l_0.wrapping_add(3 as libc::c_int as libc::c_ulong),
                            l_0,
                            matches,
                        );
                    }
                }
            }
        }
    }
    if max_length >= 6 as libc::c_int as libc::c_ulong {
        if *data.offset(1 as libc::c_int as isize) as libc::c_int == ' ' as i32
            && (*data.offset(0 as libc::c_int as isize) as libc::c_int == 'e' as i32
                || *data.offset(0 as libc::c_int as isize) as libc::c_int == 's' as i32
                || *data.offset(0 as libc::c_int as isize) as libc::c_int == ',' as i32)
            || *data.offset(0 as libc::c_int as isize) as libc::c_int
                == 0xc2 as libc::c_int
                && *data.offset(1 as libc::c_int as isize) as libc::c_int
                    == 0xa0 as libc::c_int
        {
            let mut offset_1 = *(*dictionary).buckets
                .offset(Hash(&*data.offset(2 as libc::c_int as isize)) as isize)
                as size_t;
            let mut end_1 = (offset_1 == 0) as libc::c_int;
            while end_1 == 0 {
                let fresh2 = offset_1;
                offset_1= offset_1.wrapping_add(1);
                let mut w_1 = *(*dictionary).dict_words.offset(fresh2 as isize);
                let l_1 = (w_1.len as libc::c_int & 0x1f as libc::c_int) as size_t;
                let n_1 = (1 as libc::c_int as size_t)
                    << (*(*dictionary).words).size_bits_by_length[l_1 as usize]
                        as libc::c_int;
                let id_1 = w_1.idx as size_t;
                end_1= (w_1.len as libc::c_int & 0x80 as libc::c_int != 0)
                    as libc::c_int;
                w_1.len= l_1 as uint8_t;
                if w_1.transform as libc::c_int == 0 as libc::c_int
                    && IsMatch(
                        (*dictionary).words,
                        w_1,
                        &*data.offset(2 as libc::c_int as isize),
                        max_length.wrapping_sub(2 as libc::c_int as libc::c_ulong),
                    ) != 0
                {
                    if *data.offset(0 as libc::c_int as isize) as libc::c_int
                        == 0xc2 as libc::c_int
                    {
                        AddMatch(
                            id_1
                                .wrapping_add(
                                    (102 as libc::c_int as libc::c_ulong).wrapping_mul(n_1),
                                ),
                            l_1.wrapping_add(2 as libc::c_int as libc::c_ulong),
                            l_1,
                            matches,
                        );
                        has_found_match= 1 as libc::c_int;
                    } else if l_1.wrapping_add(2 as libc::c_int as libc::c_ulong)
                        < max_length
                        && *data
                            .offset(
                                l_1.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize,
                            ) as libc::c_int == ' ' as i32
                    {
                        let mut t = (if *data.offset(0 as libc::c_int as isize)
                            as libc::c_int == 'e' as i32
                        {
                            18 as libc::c_int
                        } else if *data.offset(0 as libc::c_int as isize) as libc::c_int
                            == 's' as i32
                        {
                            7 as libc::c_int
                        } else {
                            13 as libc::c_int
                        }) as size_t;
                        AddMatch(
                            id_1.wrapping_add(t.wrapping_mul(n_1)),
                            l_1.wrapping_add(3 as libc::c_int as libc::c_ulong),
                            l_1,
                            matches,
                        );
                        has_found_match= 1 as libc::c_int;
                    }
                }
            }
        }
    }
    if max_length >= 9 as libc::c_int as libc::c_ulong {
        if *data.offset(0 as libc::c_int as isize) as libc::c_int == ' ' as i32
            && *data.offset(1 as libc::c_int as isize) as libc::c_int == 't' as i32
            && *data.offset(2 as libc::c_int as isize) as libc::c_int == 'h' as i32
            && *data.offset(3 as libc::c_int as isize) as libc::c_int == 'e' as i32
            && *data.offset(4 as libc::c_int as isize) as libc::c_int == ' ' as i32
            || *data.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32
                && *data.offset(1 as libc::c_int as isize) as libc::c_int == 'c' as i32
                && *data.offset(2 as libc::c_int as isize) as libc::c_int == 'o' as i32
                && *data.offset(3 as libc::c_int as isize) as libc::c_int == 'm' as i32
                && *data.offset(4 as libc::c_int as isize) as libc::c_int == '/' as i32
        {
            let mut offset_2 = *(*dictionary).buckets
                .offset(Hash(&*data.offset(5 as libc::c_int as isize)) as isize)
                as size_t;
            let mut end_2 = (offset_2 == 0) as libc::c_int;
            while end_2 == 0 {
                let fresh3 = offset_2;
                offset_2= offset_2.wrapping_add(1);
                let mut w_2 = *(*dictionary).dict_words.offset(fresh3 as isize);
                let l_2 = (w_2.len as libc::c_int & 0x1f as libc::c_int) as size_t;
                let n_2 = (1 as libc::c_int as size_t)
                    << (*(*dictionary).words).size_bits_by_length[l_2 as usize]
                        as libc::c_int;
                let id_2 = w_2.idx as size_t;
                end_2= (w_2.len as libc::c_int & 0x80 as libc::c_int != 0)
                    as libc::c_int;
                w_2.len= l_2 as uint8_t;
                if w_2.transform as libc::c_int == 0 as libc::c_int
                    && IsMatch(
                        (*dictionary).words,
                        w_2,
                        &*data.offset(5 as libc::c_int as isize),
                        max_length.wrapping_sub(5 as libc::c_int as libc::c_ulong),
                    ) != 0
                {
                    AddMatch(
                        id_2
                            .wrapping_add(
                                ((if *data.offset(0 as libc::c_int as isize) as libc::c_int
                                    == ' ' as i32
                                {
                                    41 as libc::c_int
                                } else {
                                    72 as libc::c_int
                                }) as libc::c_ulong)
                                    .wrapping_mul(n_2),
                            ),
                        l_2.wrapping_add(5 as libc::c_int as libc::c_ulong),
                        l_2,
                        matches,
                    );
                    has_found_match= 1 as libc::c_int;
                    if l_2.wrapping_add(5 as libc::c_int as libc::c_ulong) < max_length {
                        let mut s_3: *const uint8_t = &*data
                            .offset(
                                l_2.wrapping_add(5 as libc::c_int as libc::c_ulong) as isize,
                            ) as *const uint8_t;
                        if *data.offset(0 as libc::c_int as isize) as libc::c_int
                            == ' ' as i32
                        {
                            if l_2.wrapping_add(8 as libc::c_int as libc::c_ulong)
                                < max_length
                                && *s_3.offset(0 as libc::c_int as isize) as libc::c_int
                                    == ' ' as i32
                                && *s_3.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 'o' as i32
                                && *s_3.offset(2 as libc::c_int as isize) as libc::c_int
                                    == 'f' as i32
                                && *s_3.offset(3 as libc::c_int as isize) as libc::c_int
                                    == ' ' as i32
                            {
                                AddMatch(
                                    id_2
                                        .wrapping_add(
                                            (62 as libc::c_int as libc::c_ulong).wrapping_mul(n_2),
                                        ),
                                    l_2.wrapping_add(9 as libc::c_int as libc::c_ulong),
                                    l_2,
                                    matches,
                                );
                                if l_2.wrapping_add(12 as libc::c_int as libc::c_ulong)
                                    < max_length
                                    && *s_3.offset(4 as libc::c_int as isize) as libc::c_int
                                        == 't' as i32
                                    && *s_3.offset(5 as libc::c_int as isize) as libc::c_int
                                        == 'h' as i32
                                    && *s_3.offset(6 as libc::c_int as isize) as libc::c_int
                                        == 'e' as i32
                                    && *s_3.offset(7 as libc::c_int as isize) as libc::c_int
                                        == ' ' as i32
                                {
                                    AddMatch(
                                        id_2
                                            .wrapping_add(
                                                (73 as libc::c_int as libc::c_ulong).wrapping_mul(n_2),
                                            ),
                                        l_2.wrapping_add(13 as libc::c_int as libc::c_ulong),
                                        l_2,
                                        matches,
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    return has_found_match;
}
