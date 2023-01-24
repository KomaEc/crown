use ::libc;
pub type size_t = libc::c_ulong;
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type int8_t = __int8_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type ContextLut = *const uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BrotliDictionary {
    pub size_bits_by_length: [uint8_t; 32],
    pub offsets_by_length: [uint32_t; 32],
    pub data_size: size_t,
    pub data: *const uint8_t,
}
pub type BrotliEncoderMode = libc::c_uint;
pub const BROTLI_MODE_FONT: BrotliEncoderMode = 2;
pub const BROTLI_MODE_TEXT: BrotliEncoderMode = 1;
pub const BROTLI_MODE_GENERIC: BrotliEncoderMode = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DictWord {
    pub len: uint8_t,
    pub transform: uint8_t,
    pub idx: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BrotliEncoderDictionary {
    pub words: *const BrotliDictionary,
    pub num_transforms: uint32_t,
    pub cutoffTransformsCount: uint32_t,
    pub cutoffTransforms: uint64_t,
    pub hash_table_words: *const uint16_t,
    pub hash_table_lengths: *const uint8_t,
    pub buckets: *const uint16_t,
    pub dict_words: *const DictWord,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BrotliHasherParams {
    pub type_0: libc::c_int,
    pub bucket_bits: libc::c_int,
    pub block_bits: libc::c_int,
    pub hash_len: libc::c_int,
    pub num_last_distances_to_check: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BrotliDistanceParams {
    pub distance_postfix_bits: uint32_t,
    pub num_direct_distance_codes: uint32_t,
    pub alphabet_size_max: uint32_t,
    pub alphabet_size_limit: uint32_t,
    pub max_distance: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BrotliEncoderParams {
    pub mode: BrotliEncoderMode,
    pub quality: libc::c_int,
    pub lgwin: libc::c_int,
    pub lgblock: libc::c_int,
    pub stream_offset: size_t,
    pub size_hint: size_t,
    pub disable_literal_context_modeling: libc::c_int,
    pub large_window: libc::c_int,
    pub hasher: BrotliHasherParams,
    pub dist: BrotliDistanceParams,
    pub dictionary: BrotliEncoderDictionary,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Command {
    pub insert_len_: uint32_t,
    pub copy_len_: uint32_t,
    pub dist_extra_: uint32_t,
    pub cmd_prefix_: uint16_t,
    pub dist_prefix_: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HasherCommon {
    pub extra: *mut libc::c_void,
    pub dict_num_lookups: size_t,
    pub dict_num_matches: size_t,
    pub params: BrotliHasherParams,
    pub is_prepared_: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HasherSearchResult {
    pub len: size_t,
    pub distance: size_t,
    pub score: size_t,
    pub len_code_delta: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H10 {
    pub window_mask_: size_t,
    pub buckets_: *mut uint32_t,
    pub invalid_pos_: uint32_t,
    pub forest_: *mut uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H2 {
    pub common: *mut HasherCommon,
    pub buckets_: *mut uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H3 {
    pub common: *mut HasherCommon,
    pub buckets_: *mut uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H4 {
    pub common: *mut HasherCommon,
    pub buckets_: *mut uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H5 {
    pub bucket_size_: size_t,
    pub block_size_: size_t,
    pub hash_shift_: libc::c_int,
    pub block_mask_: uint32_t,
    pub block_bits_: libc::c_int,
    pub num_last_distances_to_check_: libc::c_int,
    pub common_: *mut HasherCommon,
    pub num_: *mut uint16_t,
    pub buckets_: *mut uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H6 {
    pub bucket_size_: size_t,
    pub block_size_: size_t,
    pub hash_shift_: libc::c_int,
    pub hash_mask_: uint64_t,
    pub block_mask_: uint32_t,
    pub block_bits_: libc::c_int,
    pub num_last_distances_to_check_: libc::c_int,
    pub common_: *mut HasherCommon,
    pub num_: *mut uint16_t,
    pub buckets_: *mut uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SlotH40 {
    pub delta: uint16_t,
    pub next: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BankH40 {
    pub slots: [SlotH40; 65536],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H40 {
    pub free_slot_idx: [uint16_t; 1],
    pub max_hops: size_t,
    pub extra: *mut libc::c_void,
    pub common: *mut HasherCommon,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SlotH41 {
    pub delta: uint16_t,
    pub next: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BankH41 {
    pub slots: [SlotH41; 65536],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H41 {
    pub free_slot_idx: [uint16_t; 1],
    pub max_hops: size_t,
    pub extra: *mut libc::c_void,
    pub common: *mut HasherCommon,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SlotH42 {
    pub delta: uint16_t,
    pub next: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BankH42 {
    pub slots: [SlotH42; 512],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H42 {
    pub free_slot_idx: [uint16_t; 512],
    pub max_hops: size_t,
    pub extra: *mut libc::c_void,
    pub common: *mut HasherCommon,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H54 {
    pub common: *mut HasherCommon,
    pub buckets_: *mut uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HROLLING_FAST {
    pub state: uint32_t,
    pub table: *mut uint32_t,
    pub next_ix: size_t,
    pub chunk_len: uint32_t,
    pub factor: uint32_t,
    pub factor_remove: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HROLLING {
    pub state: uint32_t,
    pub table: *mut uint32_t,
    pub next_ix: size_t,
    pub chunk_len: uint32_t,
    pub factor: uint32_t,
    pub factor_remove: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H35 {
    pub ha: H3,
    pub hb: HROLLING_FAST,
    pub hb_common: HasherCommon,
    pub extra: *mut libc::c_void,
    pub common: *mut HasherCommon,
    pub fresh: libc::c_int,
    pub params: *const BrotliEncoderParams,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H55 {
    pub ha: H54,
    pub hb: HROLLING_FAST,
    pub hb_common: HasherCommon,
    pub extra: *mut libc::c_void,
    pub common: *mut HasherCommon,
    pub fresh: libc::c_int,
    pub params: *const BrotliEncoderParams,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H65 {
    pub ha: H6,
    pub hb: HROLLING,
    pub hb_common: HasherCommon,
    pub extra: *mut libc::c_void,
    pub common: *mut HasherCommon,
    pub fresh: libc::c_int,
    pub params: *const BrotliEncoderParams,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Hasher {
    pub common: HasherCommon,
    pub privat: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub _H2: H2,
    pub _H3: H3,
    pub _H4: H4,
    pub _H5: H5,
    pub _H6: H6,
    pub _H40: H40,
    pub _H41: H41,
    pub _H42: H42,
    pub _H54: H54,
    pub _H35: H35,
    pub _H55: H55,
    pub _H65: H65,
    pub _H10: H10,
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
unsafe extern "C" fn BrotliUnalignedRead64(mut p: *const libc::c_void) -> uint64_t {
    return *(p as *const uint64_t);
}
#[inline(always)]
unsafe extern "C" fn BrotliUnalignedRead32(mut p: *const libc::c_void) -> uint32_t {
    return *(p as *const uint32_t);
}
#[inline(always)]
unsafe extern "C" fn Log2FloorNonZero(mut n: size_t) -> uint32_t {
    return 31 as libc::c_uint ^ (n as uint32_t).leading_zeros() as i32 as uint32_t;
}
#[inline(always)]
unsafe extern "C" fn PrefixEncodeCopyDistance(
    mut distance_code: size_t,
    mut num_direct_codes: size_t,
    mut postfix_bits: size_t,
    mut code: *mut uint16_t,
    mut extra_bits: *mut uint32_t,
) {
    if distance_code
        < (16 as libc::c_int as libc::c_ulong).wrapping_add(num_direct_codes)
    {
        *code = distance_code as uint16_t;
        *extra_bits = 0 as libc::c_int as uint32_t;
        return;
    } else {
        let mut dist = ((1 as libc::c_int as size_t)
            << postfix_bits.wrapping_add(2 as libc::c_uint as libc::c_ulong))
            .wrapping_add(
                distance_code
                    .wrapping_sub(16 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(num_direct_codes),
            );
        let mut bucket = (Log2FloorNonZero(dist))
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t;
        let mut postfix_mask = ((1 as libc::c_uint) << postfix_bits)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t;
        let mut postfix = dist & postfix_mask;
        let mut prefix = dist >> bucket & 1 as libc::c_int as libc::c_ulong;
        let mut offset = (2 as libc::c_int as libc::c_ulong).wrapping_add(prefix)
            << bucket;
        let mut nbits = bucket.wrapping_sub(postfix_bits);
        *code = (nbits << 10 as libc::c_int
            | (16 as libc::c_int as libc::c_ulong)
                .wrapping_add(num_direct_codes)
                .wrapping_add(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(
                            nbits.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        )
                        .wrapping_add(prefix) << postfix_bits,
                )
                .wrapping_add(postfix)) as uint16_t;
        *extra_bits = (dist.wrapping_sub(offset) >> postfix_bits) as uint32_t;
    };
}
#[inline(always)]
unsafe extern "C" fn GetInsertLengthCode(mut insertlen: size_t) -> uint16_t {
    if insertlen < 6 as libc::c_int as libc::c_ulong {
        return insertlen as uint16_t
    } else if insertlen < 130 as libc::c_int as libc::c_ulong {
        let mut nbits = (Log2FloorNonZero(
            insertlen.wrapping_sub(2 as libc::c_int as libc::c_ulong),
        ))
            .wrapping_sub(1 as libc::c_uint);
        return ((nbits << 1 as libc::c_int) as libc::c_ulong)
            .wrapping_add(
                insertlen.wrapping_sub(2 as libc::c_int as libc::c_ulong) >> nbits,
            )
            .wrapping_add(2 as libc::c_int as libc::c_ulong) as uint16_t;
    } else if insertlen < 2114 as libc::c_int as libc::c_ulong {
        return (Log2FloorNonZero(
            insertlen.wrapping_sub(66 as libc::c_int as libc::c_ulong),
        ))
            .wrapping_add(10 as libc::c_int as libc::c_uint) as uint16_t
    } else if insertlen < 6210 as libc::c_int as libc::c_ulong {
        return 21 as libc::c_uint as uint16_t
    } else if insertlen < 22594 as libc::c_int as libc::c_ulong {
        return 22 as libc::c_uint as uint16_t
    } else {
        return 23 as libc::c_uint as uint16_t
    };
}
#[inline(always)]
unsafe extern "C" fn GetCopyLengthCode(mut copylen: size_t) -> uint16_t {
    if copylen < 10 as libc::c_int as libc::c_ulong {
        return copylen.wrapping_sub(2 as libc::c_int as libc::c_ulong) as uint16_t
    } else if copylen < 134 as libc::c_int as libc::c_ulong {
        let mut nbits = (Log2FloorNonZero(
            copylen.wrapping_sub(6 as libc::c_int as libc::c_ulong),
        ))
            .wrapping_sub(1 as libc::c_uint);
        return ((nbits << 1 as libc::c_int) as libc::c_ulong)
            .wrapping_add(
                copylen.wrapping_sub(6 as libc::c_int as libc::c_ulong) >> nbits,
            )
            .wrapping_add(4 as libc::c_int as libc::c_ulong) as uint16_t;
    } else if copylen < 2118 as libc::c_int as libc::c_ulong {
        return (Log2FloorNonZero(
            copylen.wrapping_sub(70 as libc::c_int as libc::c_ulong),
        ))
            .wrapping_add(12 as libc::c_int as libc::c_uint) as uint16_t
    } else {
        return 23 as libc::c_uint as uint16_t
    };
}
#[inline(always)]
unsafe extern "C" fn CombineLengthCodes(
    mut inscode: uint16_t,
    mut copycode: uint16_t,
    mut use_last_distance: libc::c_int,
) -> uint16_t {
    let mut bits64 = (copycode as libc::c_uint & 0x7 as libc::c_uint
        | (inscode as libc::c_uint & 0x7 as libc::c_uint) << 3 as libc::c_uint)
        as uint16_t;
    if use_last_distance != 0 && (inscode as libc::c_uint) < 8 as libc::c_uint
        && (copycode as libc::c_uint) < 16 as libc::c_uint
    {
        return (if (copycode as libc::c_uint) < 8 as libc::c_uint {
            bits64 as libc::c_uint
        } else {
            bits64 as libc::c_uint | 64 as libc::c_uint
        }) as uint16_t
    } else {
        let mut offset = (2 as libc::c_uint)
            .wrapping_mul(
                ((copycode as libc::c_int >> 3 as libc::c_uint) as libc::c_uint)
                    .wrapping_add(
                        (3 as libc::c_uint)
                            .wrapping_mul(
                                (inscode as libc::c_int >> 3 as libc::c_uint)
                                    as libc::c_uint,
                            ),
                    ),
            );
        offset = (offset << 5 as libc::c_uint)
            .wrapping_add(0x40 as libc::c_uint)
            .wrapping_add(0x520d40 as libc::c_uint >> offset & 0xc0 as libc::c_uint);
        return (offset | bits64 as libc::c_uint) as uint16_t;
    };
}
#[inline(always)]
unsafe extern "C" fn GetLengthCode(
    mut insertlen: size_t,
    mut copylen: size_t,
    mut use_last_distance: libc::c_int,
    mut code: *mut uint16_t,
) {
    let mut inscode = GetInsertLengthCode(insertlen);
    let mut copycode = GetCopyLengthCode(copylen);
    *code = CombineLengthCodes(inscode, copycode, use_last_distance);
}
#[inline(always)]
unsafe extern "C" fn InitCommand(
    mut self_0: *mut Command,
    mut dist: *const BrotliDistanceParams,
    mut insertlen: size_t,
    mut copylen: size_t,
    mut copylen_code_delta: libc::c_int,
    mut distance_code: size_t,
) {
    let mut delta = copylen_code_delta as int8_t as uint8_t as uint32_t;
    (*self_0).insert_len_ = insertlen as uint32_t;
    (*self_0)
        .copy_len_ = (copylen | (delta << 25 as libc::c_int) as libc::c_ulong)
        as uint32_t;
    PrefixEncodeCopyDistance(
        distance_code,
        (*dist).num_direct_distance_codes as size_t,
        (*dist).distance_postfix_bits as size_t,
        &mut (*self_0).dist_prefix_,
        &mut (*self_0).dist_extra_,
    );
    GetLengthCode(
        insertlen,
        (copylen as libc::c_int + copylen_code_delta) as size_t,
        if (*self_0).dist_prefix_ as libc::c_int & 0x3ff as libc::c_int
            == 0 as libc::c_int
        {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        },
        &mut (*self_0).cmd_prefix_,
    );
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
        limit2 = limit2.wrapping_sub(1);
        if !((limit2 != 0) as libc::c_int as libc::c_long != 0) {
            break;
        }
        if (BrotliUnalignedRead64(s2 as *const libc::c_void)
            == BrotliUnalignedRead64(s1.offset(matched as isize) as *const libc::c_void))
            as libc::c_int as libc::c_long != 0
        {
            s2 = s2.offset(8 as libc::c_int as isize);
            matched = (matched as libc::c_ulong)
                .wrapping_add(8 as libc::c_int as libc::c_ulong) as size_t as size_t;
        } else {
            let mut x = BrotliUnalignedRead64(s2 as *const libc::c_void)
                ^ BrotliUnalignedRead64(
                    s1.offset(matched as isize) as *const libc::c_void,
                );
            let mut matching_bits = (x as libc::c_ulonglong).trailing_zeros() as i32
                as size_t;
            matched = (matched as libc::c_ulong)
                .wrapping_add(matching_bits >> 3 as libc::c_int) as size_t as size_t;
            return matched;
        }
    }
    limit = (limit & 7 as libc::c_int as libc::c_ulong)
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    loop {
        limit = limit.wrapping_sub(1);
        if !(limit != 0) {
            break;
        }
        if (*s1.offset(matched as isize) as libc::c_int == *s2 as libc::c_int)
            as libc::c_int as libc::c_long != 0
        {
            s2 = s2.offset(1);
            matched = matched.wrapping_add(1);
        } else {
            return matched
        }
    }
    return matched;
}
#[inline(always)]
unsafe extern "C" fn LiteralSpreeLengthForSparseSearch(
    mut params: *const BrotliEncoderParams,
) -> size_t {
    return (if (*params).quality < 9 as libc::c_int {
        64 as libc::c_int
    } else {
        512 as libc::c_int
    }) as size_t;
}
static mut kHashMul32: uint32_t = 0x1e35a7bd as libc::c_int as uint32_t;
static mut kHashMul64: uint64_t = (0x1e35a7bd as libc::c_int as uint64_t)
    << 32 as libc::c_int | 0x1e35a7bd as libc::c_int as libc::c_ulong;
static mut kHashMul64Long: uint64_t = (0x1fe35a7b as libc::c_uint as uint64_t)
    << 32 as libc::c_int | 0xd3579bd3 as libc::c_uint as libc::c_ulong;
#[inline(always)]
unsafe extern "C" fn Hash14(mut data: *const uint8_t) -> uint32_t {
    let mut h = (BrotliUnalignedRead32(data as *const libc::c_void))
        .wrapping_mul(kHashMul32);
    return h >> 32 as libc::c_int - 14 as libc::c_int;
}
#[inline(always)]
unsafe extern "C" fn PrepareDistanceCache(
    mut distance_cache: *mut libc::c_int,
    num_distances: libc::c_int,
) {
    if num_distances > 4 as libc::c_int {
        let mut last_distance = *distance_cache.offset(0 as libc::c_int as isize);
        *distance_cache
            .offset(4 as libc::c_int as isize) = last_distance - 1 as libc::c_int;
        *distance_cache
            .offset(5 as libc::c_int as isize) = last_distance + 1 as libc::c_int;
        *distance_cache
            .offset(6 as libc::c_int as isize) = last_distance - 2 as libc::c_int;
        *distance_cache
            .offset(7 as libc::c_int as isize) = last_distance + 2 as libc::c_int;
        *distance_cache
            .offset(8 as libc::c_int as isize) = last_distance - 3 as libc::c_int;
        *distance_cache
            .offset(9 as libc::c_int as isize) = last_distance + 3 as libc::c_int;
        if num_distances > 10 as libc::c_int {
            let mut next_last_distance = *distance_cache
                .offset(1 as libc::c_int as isize);
            *distance_cache
                .offset(
                    10 as libc::c_int as isize,
                ) = next_last_distance - 1 as libc::c_int;
            *distance_cache
                .offset(
                    11 as libc::c_int as isize,
                ) = next_last_distance + 1 as libc::c_int;
            *distance_cache
                .offset(
                    12 as libc::c_int as isize,
                ) = next_last_distance - 2 as libc::c_int;
            *distance_cache
                .offset(
                    13 as libc::c_int as isize,
                ) = next_last_distance + 2 as libc::c_int;
            *distance_cache
                .offset(
                    14 as libc::c_int as isize,
                ) = next_last_distance - 3 as libc::c_int;
            *distance_cache
                .offset(
                    15 as libc::c_int as isize,
                ) = next_last_distance + 3 as libc::c_int;
        }
    }
}
#[inline(always)]
unsafe extern "C" fn BackwardReferenceScore(
    mut copy_length: size_t,
    mut backward_reference_offset: size_t,
) -> size_t {
    return ((30 as libc::c_int * 8 as libc::c_int) as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<size_t>() as libc::c_ulong)
        .wrapping_add((135 as libc::c_int as libc::c_ulong).wrapping_mul(copy_length))
        .wrapping_sub(
            (30 as libc::c_int as libc::c_uint)
                .wrapping_mul(Log2FloorNonZero(backward_reference_offset))
                as libc::c_ulong,
        );
}
#[inline(always)]
unsafe extern "C" fn BackwardReferenceScoreUsingLastDistance(
    mut copy_length: size_t,
) -> size_t {
    return (135 as libc::c_int as libc::c_ulong)
        .wrapping_mul(copy_length)
        .wrapping_add(
            ((30 as libc::c_int * 8 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<size_t>() as libc::c_ulong),
        )
        .wrapping_add(15 as libc::c_int as libc::c_ulong);
}
#[inline(always)]
unsafe extern "C" fn BackwardReferencePenaltyUsingLastDistance(
    mut distance_short_code: size_t,
) -> size_t {
    return (39 as libc::c_int as size_t)
        .wrapping_add(
            (0x1ca10 as libc::c_int
                >> (distance_short_code & 0xe as libc::c_int as libc::c_ulong)
                & 0xe as libc::c_int) as libc::c_ulong,
        );
}
#[inline(always)]
unsafe extern "C" fn TestStaticDictionaryItem(
    mut dictionary: *const BrotliEncoderDictionary,
    mut len: size_t,
    mut word_idx: size_t,
    mut data: *const uint8_t,
    mut max_length: size_t,
    mut max_backward: size_t,
    mut max_distance: size_t,
    mut out: *mut HasherSearchResult,
) -> libc::c_int {
    let mut offset: size_t = 0;
    let mut matchlen: size_t = 0;
    let mut backward: size_t = 0;
    let mut score: size_t = 0;
    offset = ((*(*dictionary).words).offsets_by_length[len as usize] as libc::c_ulong)
        .wrapping_add(len.wrapping_mul(word_idx));
    if len > max_length {
        return 0 as libc::c_int;
    }
    matchlen = FindMatchLengthWithLimit(
        data,
        &*((*(*dictionary).words).data).offset(offset as isize),
        len,
    );
    if matchlen.wrapping_add((*dictionary).cutoffTransformsCount as libc::c_ulong) <= len
        || matchlen == 0 as libc::c_int as libc::c_ulong
    {
        return 0 as libc::c_int;
    }
    let mut cut = len.wrapping_sub(matchlen);
    let mut transform_id = (cut << 2 as libc::c_int)
        .wrapping_add(
            (*dictionary).cutoffTransforms
                >> cut.wrapping_mul(6 as libc::c_int as libc::c_ulong)
                & 0x3f as libc::c_int as libc::c_ulong,
        );
    backward = max_backward
        .wrapping_add(1 as libc::c_int as libc::c_ulong)
        .wrapping_add(word_idx)
        .wrapping_add(
            transform_id
                << (*(*dictionary).words).size_bits_by_length[len as usize]
                    as libc::c_int,
        );
    if backward > max_distance {
        return 0 as libc::c_int;
    }
    score = BackwardReferenceScore(matchlen, backward);
    if score < (*out).score {
        return 0 as libc::c_int;
    }
    (*out).len = matchlen;
    (*out).len_code_delta = len as libc::c_int - matchlen as libc::c_int;
    (*out).distance = backward;
    (*out).score = score;
    return 1 as libc::c_int;
}
#[inline(always)]
unsafe extern "C" fn SearchInStaticDictionary(
    mut dictionary: *const BrotliEncoderDictionary,
    mut common: *mut HasherCommon,
    mut data: *const uint8_t,
    mut max_length: size_t,
    mut max_backward: size_t,
    mut max_distance: size_t,
    mut out: *mut HasherSearchResult,
    mut shallow: libc::c_int,
) {
    let mut key: size_t = 0;
    let mut i: size_t = 0;
    if (*common).dict_num_matches < (*common).dict_num_lookups >> 7 as libc::c_int {
        return;
    }
    key = (Hash14(data) << 1 as libc::c_int) as size_t;
    i = 0 as libc::c_int as size_t;
    while i
        < (if shallow != 0 { 1 as libc::c_uint } else { 2 as libc::c_uint })
            as libc::c_ulong
    {
        let ref mut fresh0 = (*common).dict_num_lookups;
        *fresh0 = (*fresh0).wrapping_add(1);
        if *((*dictionary).hash_table_lengths).offset(key as isize) as libc::c_int
            != 0 as libc::c_int
        {
            let mut item_matches = TestStaticDictionaryItem(
                dictionary,
                *((*dictionary).hash_table_lengths).offset(key as isize) as size_t,
                *((*dictionary).hash_table_words).offset(key as isize) as size_t,
                data,
                max_length,
                max_backward,
                max_distance,
                out,
            );
            if item_matches != 0 {
                let ref mut fresh1 = (*common).dict_num_matches;
                *fresh1 = (*fresh1).wrapping_add(1);
            }
        }
        i = i.wrapping_add(1);
        key = key.wrapping_add(1);
    }
}
#[inline(always)]
unsafe extern "C" fn HashTypeLengthH2() -> size_t {
    return 8 as libc::c_int as size_t;
}
#[inline(always)]
unsafe extern "C" fn StoreLookaheadH2() -> size_t {
    return 8 as libc::c_int as size_t;
}
unsafe extern "C" fn HashBytesH2(mut data: *const uint8_t) -> uint32_t {
    let h = (BrotliUnalignedRead64(data as *const libc::c_void)
        << 64 as libc::c_int - 8 as libc::c_int * 5 as libc::c_int)
        .wrapping_mul(kHashMul64);
    return (h >> 64 as libc::c_int - 16 as libc::c_int) as uint32_t;
}
#[inline(always)]
unsafe extern "C" fn StoreH2(
    mut self_0: *mut H2,
    mut data: *const uint8_t,
    mask: size_t,
    ix: size_t,
) {
    let key = HashBytesH2(&*data.offset((ix & mask) as isize));
    if (1 as libc::c_int) << 0 as libc::c_int == 1 as libc::c_int {
        *((*self_0).buckets_).offset(key as isize) = ix as uint32_t;
    } else {
        let off = (ix
            & ((((1 as libc::c_int) << 0 as libc::c_int) - 1 as libc::c_int)
                << 3 as libc::c_int) as libc::c_ulong) as uint32_t;
        *((*self_0).buckets_)
            .offset(
                (key.wrapping_add(off)
                    & (((1 as libc::c_int) << 16 as libc::c_int) - 1 as libc::c_int)
                        as libc::c_uint) as isize,
            ) = ix as uint32_t;
    };
}
#[inline(always)]
unsafe extern "C" fn StoreRangeH2(
    mut self_0: *mut H2,
    mut data: *const uint8_t,
    mask: size_t,
    ix_start: size_t,
    ix_end: size_t,
) {
    let mut i: size_t = 0;
    i = ix_start;
    while i < ix_end {
        StoreH2(self_0, data, mask, i);
        i = i.wrapping_add(1);
    }
}
#[inline(always)]
unsafe extern "C" fn PrepareDistanceCacheH2(
    mut self_0: *mut H2,
    mut distance_cache: *mut libc::c_int,
) {}
#[inline(always)]
unsafe extern "C" fn FindLongestMatchH2(
    mut self_0: *mut H2,
    mut dictionary: *const BrotliEncoderDictionary,
    mut data: *const uint8_t,
    ring_buffer_mask: size_t,
    mut distance_cache: *const libc::c_int,
    cur_ix: size_t,
    max_length: size_t,
    max_backward: size_t,
    dictionary_distance: size_t,
    max_distance: size_t,
    mut out: *mut HasherSearchResult,
) {
    let mut buckets = (*self_0).buckets_;
    let best_len_in = (*out).len;
    let cur_ix_masked = cur_ix & ring_buffer_mask;
    let mut compare_char = *data.offset(cur_ix_masked.wrapping_add(best_len_in) as isize)
        as libc::c_int;
    let mut key = HashBytesH2(&*data.offset(cur_ix_masked as isize)) as size_t;
    let mut key_out: size_t = 0;
    let mut min_score = (*out).score;
    let mut best_score = (*out).score;
    let mut best_len = best_len_in;
    let mut cached_backward = *distance_cache.offset(0 as libc::c_int as isize)
        as size_t;
    let mut prev_ix = cur_ix.wrapping_sub(cached_backward);
    (*out).len_code_delta = 0 as libc::c_int;
    if prev_ix < cur_ix {
        prev_ix &= ring_buffer_mask as uint32_t as libc::c_ulong;
        if compare_char
            == *data.offset(prev_ix.wrapping_add(best_len) as isize) as libc::c_int
        {
            let len = FindMatchLengthWithLimit(
                &*data.offset(prev_ix as isize),
                &*data.offset(cur_ix_masked as isize),
                max_length,
            );
            if len >= 4 as libc::c_int as libc::c_ulong {
                let score = BackwardReferenceScoreUsingLastDistance(len);
                if best_score < score {
                    (*out).len = len;
                    (*out).distance = cached_backward;
                    (*out).score = score;
                    if (1 as libc::c_int) << 0 as libc::c_int == 1 as libc::c_int {
                        *buckets.offset(key as isize) = cur_ix as uint32_t;
                        return;
                    } else {
                        best_len = len;
                        best_score = score;
                        compare_char = *data
                            .offset(cur_ix_masked.wrapping_add(len) as isize)
                            as libc::c_int;
                    }
                }
            }
        }
    }
    if (1 as libc::c_int) << 0 as libc::c_int == 1 as libc::c_int {
        let mut backward: size_t = 0;
        let mut len_0: size_t = 0;
        prev_ix = *buckets.offset(key as isize) as size_t;
        *buckets.offset(key as isize) = cur_ix as uint32_t;
        backward = cur_ix.wrapping_sub(prev_ix);
        prev_ix &= ring_buffer_mask as uint32_t as libc::c_ulong;
        if compare_char
            != *data.offset(prev_ix.wrapping_add(best_len_in) as isize) as libc::c_int
        {
            return;
        }
        if (backward == 0 as libc::c_int as libc::c_ulong || backward > max_backward)
            as libc::c_int as libc::c_long != 0
        {
            return;
        }
        len_0 = FindMatchLengthWithLimit(
            &*data.offset(prev_ix as isize),
            &*data.offset(cur_ix_masked as isize),
            max_length,
        );
        if len_0 >= 4 as libc::c_int as libc::c_ulong {
            let score_0 = BackwardReferenceScore(len_0, backward);
            if best_score < score_0 {
                (*out).len = len_0;
                (*out).distance = backward;
                (*out).score = score_0;
                return;
            }
        }
    }
    if 1 as libc::c_int != 0 && min_score == (*out).score {
        SearchInStaticDictionary(
            dictionary,
            (*self_0).common,
            &*data.offset(cur_ix_masked as isize),
            max_length,
            dictionary_distance,
            max_distance,
            out,
            1 as libc::c_int,
        );
    }
    if (1 as libc::c_int) << 0 as libc::c_int != 1 as libc::c_int {
        *buckets.offset(key_out as isize) = cur_ix as uint32_t;
    }
}
#[inline(always)]
unsafe extern "C" fn HashTypeLengthH3() -> size_t {
    return 8 as libc::c_int as size_t;
}
#[inline(always)]
unsafe extern "C" fn StoreLookaheadH3() -> size_t {
    return 8 as libc::c_int as size_t;
}
unsafe extern "C" fn HashBytesH3(mut data: *const uint8_t) -> uint32_t {
    let h = (BrotliUnalignedRead64(data as *const libc::c_void)
        << 64 as libc::c_int - 8 as libc::c_int * 5 as libc::c_int)
        .wrapping_mul(kHashMul64);
    return (h >> 64 as libc::c_int - 16 as libc::c_int) as uint32_t;
}
#[inline(always)]
unsafe extern "C" fn StoreH3(
    mut self_0: *mut H3,
    mut data: *const uint8_t,
    mask: size_t,
    ix: size_t,
) {
    let key = HashBytesH3(&*data.offset((ix & mask) as isize));
    if (1 as libc::c_int) << 1 as libc::c_int == 1 as libc::c_int {
        *((*self_0).buckets_).offset(key as isize) = ix as uint32_t;
    } else {
        let off = (ix
            & ((((1 as libc::c_int) << 1 as libc::c_int) - 1 as libc::c_int)
                << 3 as libc::c_int) as libc::c_ulong) as uint32_t;
        *((*self_0).buckets_)
            .offset(
                (key.wrapping_add(off)
                    & (((1 as libc::c_int) << 16 as libc::c_int) - 1 as libc::c_int)
                        as libc::c_uint) as isize,
            ) = ix as uint32_t;
    };
}
#[inline(always)]
unsafe extern "C" fn StoreRangeH3(
    mut self_0: *mut H3,
    mut data: *const uint8_t,
    mask: size_t,
    ix_start: size_t,
    ix_end: size_t,
) {
    let mut i: size_t = 0;
    i = ix_start;
    while i < ix_end {
        StoreH3(self_0, data, mask, i);
        i = i.wrapping_add(1);
    }
}
#[inline(always)]
unsafe extern "C" fn PrepareDistanceCacheH3(
    mut self_0: *mut H3,
    mut distance_cache: *mut libc::c_int,
) {}
#[inline(always)]
unsafe extern "C" fn FindLongestMatchH3(
    mut self_0: *mut H3,
    mut dictionary: *const BrotliEncoderDictionary,
    mut data: *const uint8_t,
    ring_buffer_mask: size_t,
    mut distance_cache: *const libc::c_int,
    cur_ix: size_t,
    max_length: size_t,
    max_backward: size_t,
    dictionary_distance: size_t,
    max_distance: size_t,
    mut out: *mut HasherSearchResult,
) {
    let mut buckets = (*self_0).buckets_;
    let best_len_in = (*out).len;
    let cur_ix_masked = cur_ix & ring_buffer_mask;
    let mut compare_char = *data.offset(cur_ix_masked.wrapping_add(best_len_in) as isize)
        as libc::c_int;
    let mut key = HashBytesH3(&*data.offset(cur_ix_masked as isize)) as size_t;
    let mut key_out: size_t = 0;
    let mut min_score = (*out).score;
    let mut best_score = (*out).score;
    let mut best_len = best_len_in;
    let mut cached_backward = *distance_cache.offset(0 as libc::c_int as isize)
        as size_t;
    let mut prev_ix = cur_ix.wrapping_sub(cached_backward);
    (*out).len_code_delta = 0 as libc::c_int;
    if prev_ix < cur_ix {
        prev_ix &= ring_buffer_mask as uint32_t as libc::c_ulong;
        if compare_char
            == *data.offset(prev_ix.wrapping_add(best_len) as isize) as libc::c_int
        {
            let len = FindMatchLengthWithLimit(
                &*data.offset(prev_ix as isize),
                &*data.offset(cur_ix_masked as isize),
                max_length,
            );
            if len >= 4 as libc::c_int as libc::c_ulong {
                let score = BackwardReferenceScoreUsingLastDistance(len);
                if best_score < score {
                    (*out).len = len;
                    (*out).distance = cached_backward;
                    (*out).score = score;
                    if (1 as libc::c_int) << 1 as libc::c_int == 1 as libc::c_int {
                        *buckets.offset(key as isize) = cur_ix as uint32_t;
                        return;
                    } else {
                        best_len = len;
                        best_score = score;
                        compare_char = *data
                            .offset(cur_ix_masked.wrapping_add(len) as isize)
                            as libc::c_int;
                    }
                }
            }
        }
    }
    {
        let mut keys: [size_t; 2] = [0; 2];
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_ulong {
            keys[i
                as usize] = key.wrapping_add(i << 3 as libc::c_int)
                & (((1 as libc::c_int) << 16 as libc::c_int) - 1 as libc::c_int)
                    as libc::c_ulong;
            i = i.wrapping_add(1);
        }
        key_out = keys[((cur_ix
            & ((((1 as libc::c_int) << 1 as libc::c_int) - 1 as libc::c_int)
                << 3 as libc::c_int) as libc::c_ulong) >> 3 as libc::c_int) as usize];
        i = 0 as libc::c_int as size_t;
        while i < ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_ulong {
            let mut len_1: size_t = 0;
            let mut backward_0: size_t = 0;
            prev_ix = *buckets.offset(keys[i as usize] as isize) as size_t;
            backward_0 = cur_ix.wrapping_sub(prev_ix);
            prev_ix &= ring_buffer_mask as uint32_t as libc::c_ulong;
            if !(compare_char
                != *data.offset(prev_ix.wrapping_add(best_len) as isize) as libc::c_int)
            {
                if !((backward_0 == 0 as libc::c_int as libc::c_ulong
                    || backward_0 > max_backward) as libc::c_int as libc::c_long != 0)
                {
                    len_1 = FindMatchLengthWithLimit(
                        &*data.offset(prev_ix as isize),
                        &*data.offset(cur_ix_masked as isize),
                        max_length,
                    );
                    if len_1 >= 4 as libc::c_int as libc::c_ulong {
                        let score_1 = BackwardReferenceScore(len_1, backward_0);
                        if best_score < score_1 {
                            best_len = len_1;
                            (*out).len = len_1;
                            compare_char = *data
                                .offset(cur_ix_masked.wrapping_add(len_1) as isize)
                                as libc::c_int;
                            best_score = score_1;
                            (*out).score = score_1;
                            (*out).distance = backward_0;
                        }
                    }
                }
            }
            i = i.wrapping_add(1);
        }
    }
    if (1 as libc::c_int) << 1 as libc::c_int != 1 as libc::c_int {
        *buckets.offset(key_out as isize) = cur_ix as uint32_t;
    }
}
#[inline(always)]
unsafe extern "C" fn HashTypeLengthH4() -> size_t {
    return 8 as libc::c_int as size_t;
}
#[inline(always)]
unsafe extern "C" fn StoreLookaheadH4() -> size_t {
    return 8 as libc::c_int as size_t;
}
unsafe extern "C" fn HashBytesH4(mut data: *const uint8_t) -> uint32_t {
    let h = (BrotliUnalignedRead64(data as *const libc::c_void)
        << 64 as libc::c_int - 8 as libc::c_int * 5 as libc::c_int)
        .wrapping_mul(kHashMul64);
    return (h >> 64 as libc::c_int - 17 as libc::c_int) as uint32_t;
}
#[inline(always)]
unsafe extern "C" fn StoreH4(
    mut self_0: *mut H4,
    mut data: *const uint8_t,
    mask: size_t,
    ix: size_t,
) {
    let key = HashBytesH4(&*data.offset((ix & mask) as isize));
    if (1 as libc::c_int) << 2 as libc::c_int == 1 as libc::c_int {
        *((*self_0).buckets_).offset(key as isize) = ix as uint32_t;
    } else {
        let off = (ix
            & ((((1 as libc::c_int) << 2 as libc::c_int) - 1 as libc::c_int)
                << 3 as libc::c_int) as libc::c_ulong) as uint32_t;
        *((*self_0).buckets_)
            .offset(
                (key.wrapping_add(off)
                    & (((1 as libc::c_int) << 17 as libc::c_int) - 1 as libc::c_int)
                        as libc::c_uint) as isize,
            ) = ix as uint32_t;
    };
}
#[inline(always)]
unsafe extern "C" fn StoreRangeH4(
    mut self_0: *mut H4,
    mut data: *const uint8_t,
    mask: size_t,
    ix_start: size_t,
    ix_end: size_t,
) {
    let mut i: size_t = 0;
    i = ix_start;
    while i < ix_end {
        StoreH4(self_0, data, mask, i);
        i = i.wrapping_add(1);
    }
}
#[inline(always)]
unsafe extern "C" fn PrepareDistanceCacheH4(
    mut self_0: *mut H4,
    mut distance_cache: *mut libc::c_int,
) {}
#[inline(always)]
unsafe extern "C" fn FindLongestMatchH4(
    mut self_0: *mut H4,
    mut dictionary: *const BrotliEncoderDictionary,
    mut data: *const uint8_t,
    ring_buffer_mask: size_t,
    mut distance_cache: *const libc::c_int,
    cur_ix: size_t,
    max_length: size_t,
    max_backward: size_t,
    dictionary_distance: size_t,
    max_distance: size_t,
    mut out: *mut HasherSearchResult,
) {
    let mut buckets = (*self_0).buckets_;
    let best_len_in = (*out).len;
    let cur_ix_masked = cur_ix & ring_buffer_mask;
    let mut compare_char = *data.offset(cur_ix_masked.wrapping_add(best_len_in) as isize)
        as libc::c_int;
    let mut key = HashBytesH4(&*data.offset(cur_ix_masked as isize)) as size_t;
    let mut key_out: size_t = 0;
    let mut min_score = (*out).score;
    let mut best_score = (*out).score;
    let mut best_len = best_len_in;
    let mut cached_backward = *distance_cache.offset(0 as libc::c_int as isize)
        as size_t;
    let mut prev_ix = cur_ix.wrapping_sub(cached_backward);
    (*out).len_code_delta = 0 as libc::c_int;
    if prev_ix < cur_ix {
        prev_ix &= ring_buffer_mask as uint32_t as libc::c_ulong;
        if compare_char
            == *data.offset(prev_ix.wrapping_add(best_len) as isize) as libc::c_int
        {
            let len = FindMatchLengthWithLimit(
                &*data.offset(prev_ix as isize),
                &*data.offset(cur_ix_masked as isize),
                max_length,
            );
            if len >= 4 as libc::c_int as libc::c_ulong {
                let score = BackwardReferenceScoreUsingLastDistance(len);
                if best_score < score {
                    (*out).len = len;
                    (*out).distance = cached_backward;
                    (*out).score = score;
                    if (1 as libc::c_int) << 2 as libc::c_int == 1 as libc::c_int {
                        *buckets.offset(key as isize) = cur_ix as uint32_t;
                        return;
                    } else {
                        best_len = len;
                        best_score = score;
                        compare_char = *data
                            .offset(cur_ix_masked.wrapping_add(len) as isize)
                            as libc::c_int;
                    }
                }
            }
        }
    }
    {
        let mut keys: [size_t; 4] = [0; 4];
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_ulong {
            keys[i
                as usize] = key.wrapping_add(i << 3 as libc::c_int)
                & (((1 as libc::c_int) << 17 as libc::c_int) - 1 as libc::c_int)
                    as libc::c_ulong;
            i = i.wrapping_add(1);
        }
        key_out = keys[((cur_ix
            & ((((1 as libc::c_int) << 2 as libc::c_int) - 1 as libc::c_int)
                << 3 as libc::c_int) as libc::c_ulong) >> 3 as libc::c_int) as usize];
        i = 0 as libc::c_int as size_t;
        while i < ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_ulong {
            let mut len_1: size_t = 0;
            let mut backward_0: size_t = 0;
            prev_ix = *buckets.offset(keys[i as usize] as isize) as size_t;
            backward_0 = cur_ix.wrapping_sub(prev_ix);
            prev_ix &= ring_buffer_mask as uint32_t as libc::c_ulong;
            if !(compare_char
                != *data.offset(prev_ix.wrapping_add(best_len) as isize) as libc::c_int)
            {
                if !((backward_0 == 0 as libc::c_int as libc::c_ulong
                    || backward_0 > max_backward) as libc::c_int as libc::c_long != 0)
                {
                    len_1 = FindMatchLengthWithLimit(
                        &*data.offset(prev_ix as isize),
                        &*data.offset(cur_ix_masked as isize),
                        max_length,
                    );
                    if len_1 >= 4 as libc::c_int as libc::c_ulong {
                        let score_1 = BackwardReferenceScore(len_1, backward_0);
                        if best_score < score_1 {
                            best_len = len_1;
                            (*out).len = len_1;
                            compare_char = *data
                                .offset(cur_ix_masked.wrapping_add(len_1) as isize)
                                as libc::c_int;
                            best_score = score_1;
                            (*out).score = score_1;
                            (*out).distance = backward_0;
                        }
                    }
                }
            }
            i = i.wrapping_add(1);
        }
    }
    if 1 as libc::c_int != 0 && min_score == (*out).score {
        SearchInStaticDictionary(
            dictionary,
            (*self_0).common,
            &*data.offset(cur_ix_masked as isize),
            max_length,
            dictionary_distance,
            max_distance,
            out,
            1 as libc::c_int,
        );
    }
    if (1 as libc::c_int) << 2 as libc::c_int != 1 as libc::c_int {
        *buckets.offset(key_out as isize) = cur_ix as uint32_t;
    }
}
#[inline(always)]
unsafe extern "C" fn HashTypeLengthH5() -> size_t {
    return 4 as libc::c_int as size_t;
}
#[inline(always)]
unsafe extern "C" fn StoreLookaheadH5() -> size_t {
    return 4 as libc::c_int as size_t;
}
unsafe extern "C" fn HashBytesH5(
    mut data: *const uint8_t,
    shift: libc::c_int,
) -> uint32_t {
    let mut h = (BrotliUnalignedRead32(data as *const libc::c_void))
        .wrapping_mul(kHashMul32);
    return h >> shift;
}
#[inline(always)]
unsafe extern "C" fn StoreH5(
    mut self_0: *mut H5,
    mut data: *const uint8_t,
    mask: size_t,
    ix: size_t,
) {
    let key = HashBytesH5(&*data.offset((ix & mask) as isize), (*self_0).hash_shift_);
    let minor_ix = (*((*self_0).num_).offset(key as isize) as libc::c_uint
        & (*self_0).block_mask_) as size_t;
    let offset = minor_ix.wrapping_add((key << (*self_0).block_bits_) as libc::c_ulong);
    *((*self_0).buckets_).offset(offset as isize) = ix as uint32_t;
    let ref mut fresh2 = *((*self_0).num_).offset(key as isize);
    *fresh2 = (*fresh2).wrapping_add(1);
}
#[inline(always)]
unsafe extern "C" fn StoreRangeH5(
    mut self_0: *mut H5,
    mut data: *const uint8_t,
    mask: size_t,
    ix_start: size_t,
    ix_end: size_t,
) {
    let mut i: size_t = 0;
    i = ix_start;
    while i < ix_end {
        StoreH5(self_0, data, mask, i);
        i = i.wrapping_add(1);
    }
}
#[inline(always)]
unsafe extern "C" fn PrepareDistanceCacheH5(
    mut self_0: *mut H5,
    mut distance_cache: *mut libc::c_int,
) {
    PrepareDistanceCache(distance_cache, (*self_0).num_last_distances_to_check_);
}
#[inline(always)]
unsafe extern "C" fn FindLongestMatchH5(
    mut self_0: *mut H5,
    mut dictionary: *const BrotliEncoderDictionary,
    mut data: *const uint8_t,
    ring_buffer_mask: size_t,
    mut distance_cache: *const libc::c_int,
    cur_ix: size_t,
    max_length: size_t,
    max_backward: size_t,
    dictionary_distance: size_t,
    max_distance: size_t,
    mut out: *mut HasherSearchResult,
) {
    let mut num = (*self_0).num_;
    let mut buckets = (*self_0).buckets_;
    let cur_ix_masked = cur_ix & ring_buffer_mask;
    let mut min_score = (*out).score;
    let mut best_score = (*out).score;
    let mut best_len = (*out).len;
    let mut i: size_t = 0;
    (*out).len = 0 as libc::c_int as size_t;
    (*out).len_code_delta = 0 as libc::c_int;
    i = 0 as libc::c_int as size_t;
    while i < (*self_0).num_last_distances_to_check_ as size_t {
        let backward = *distance_cache.offset(i as isize) as size_t;
        let mut prev_ix = cur_ix.wrapping_sub(backward);
        if !(prev_ix >= cur_ix) {
            if !((backward > max_backward) as libc::c_int as libc::c_long != 0) {
                prev_ix &= ring_buffer_mask;
                if !(cur_ix_masked.wrapping_add(best_len) > ring_buffer_mask
                    || prev_ix.wrapping_add(best_len) > ring_buffer_mask
                    || *data.offset(cur_ix_masked.wrapping_add(best_len) as isize)
                        as libc::c_int
                        != *data.offset(prev_ix.wrapping_add(best_len) as isize)
                            as libc::c_int)
                {
                    let len = FindMatchLengthWithLimit(
                        &*data.offset(prev_ix as isize),
                        &*data.offset(cur_ix_masked as isize),
                        max_length,
                    );
                    if len >= 3 as libc::c_int as libc::c_ulong
                        || len == 2 as libc::c_int as libc::c_ulong
                            && i < 2 as libc::c_int as libc::c_ulong
                    {
                        let mut score = BackwardReferenceScoreUsingLastDistance(len);
                        if best_score < score {
                            if i != 0 as libc::c_int as libc::c_ulong {
                                score = (score as libc::c_ulong)
                                    .wrapping_sub(BackwardReferencePenaltyUsingLastDistance(i))
                                    as size_t as size_t;
                            }
                            if best_score < score {
                                best_score = score;
                                best_len = len;
                                (*out).len = best_len;
                                (*out).distance = backward;
                                (*out).score = best_score;
                            }
                        }
                    }
                }
            }
        }
        i = i.wrapping_add(1);
    }
    let key = HashBytesH5(&*data.offset(cur_ix_masked as isize), (*self_0).hash_shift_);
    let mut bucket: *mut uint32_t = &mut *buckets
        .offset((key << (*self_0).block_bits_) as isize) as *mut uint32_t;
    let down = if *num.offset(key as isize) as libc::c_ulong > (*self_0).block_size_ {
        (*num.offset(key as isize) as libc::c_ulong).wrapping_sub((*self_0).block_size_)
    } else {
        0 as libc::c_uint as libc::c_ulong
    };
    i = *num.offset(key as isize) as size_t;
    while i > down {
        i = i.wrapping_sub(1);
        let mut prev_ix_0 = *bucket
            .offset((i & (*self_0).block_mask_ as libc::c_ulong) as isize) as size_t;
        let backward_0 = cur_ix.wrapping_sub(prev_ix_0);
        if (backward_0 > max_backward) as libc::c_int as libc::c_long != 0 {
            break;
        }
        prev_ix_0 &= ring_buffer_mask;
        if cur_ix_masked.wrapping_add(best_len) > ring_buffer_mask
            || prev_ix_0.wrapping_add(best_len) > ring_buffer_mask
            || *data.offset(cur_ix_masked.wrapping_add(best_len) as isize) as libc::c_int
                != *data.offset(prev_ix_0.wrapping_add(best_len) as isize) as libc::c_int
        {
            continue;
        }
        let len_0 = FindMatchLengthWithLimit(
            &*data.offset(prev_ix_0 as isize),
            &*data.offset(cur_ix_masked as isize),
            max_length,
        );
        if len_0 >= 4 as libc::c_int as libc::c_ulong {
            let mut score_0 = BackwardReferenceScore(len_0, backward_0);
            if best_score < score_0 {
                best_score = score_0;
                best_len = len_0;
                (*out).len = best_len;
                (*out).distance = backward_0;
                (*out).score = best_score;
            }
        }
    }
    *bucket
        .offset(
            (*num.offset(key as isize) as libc::c_uint & (*self_0).block_mask_) as isize,
        ) = cur_ix as uint32_t;
    let ref mut fresh3 = *num.offset(key as isize);
    *fresh3 = (*fresh3).wrapping_add(1);
    if min_score == (*out).score {
        SearchInStaticDictionary(
            dictionary,
            (*self_0).common_,
            &*data.offset(cur_ix_masked as isize),
            max_length,
            dictionary_distance,
            max_distance,
            out,
            0 as libc::c_int,
        );
    }
}
#[inline(always)]
unsafe extern "C" fn HashTypeLengthH6() -> size_t {
    return 8 as libc::c_int as size_t;
}
#[inline(always)]
unsafe extern "C" fn StoreLookaheadH6() -> size_t {
    return 8 as libc::c_int as size_t;
}
#[inline(always)]
unsafe extern "C" fn HashBytesH6(
    mut data: *const uint8_t,
    mask: uint64_t,
    shift: libc::c_int,
) -> uint32_t {
    let h = (BrotliUnalignedRead64(data as *const libc::c_void) & mask)
        .wrapping_mul(kHashMul64Long);
    return (h >> shift) as uint32_t;
}
#[inline(always)]
unsafe extern "C" fn StoreH6(
    mut self_0: *mut H6,
    mut data: *const uint8_t,
    mask: size_t,
    ix: size_t,
) {
    let mut num = (*self_0).num_;
    let mut buckets = (*self_0).buckets_;
    let key = HashBytesH6(
        &*data.offset((ix & mask) as isize),
        (*self_0).hash_mask_,
        (*self_0).hash_shift_,
    );
    let minor_ix = (*num.offset(key as isize) as libc::c_uint & (*self_0).block_mask_)
        as size_t;
    let offset = minor_ix.wrapping_add((key << (*self_0).block_bits_) as libc::c_ulong);
    let ref mut fresh4 = *num.offset(key as isize);
    *fresh4 = (*fresh4).wrapping_add(1);
    *buckets.offset(offset as isize) = ix as uint32_t;
}
#[inline(always)]
unsafe extern "C" fn StoreRangeH6(
    mut self_0: *mut H6,
    mut data: *const uint8_t,
    mask: size_t,
    ix_start: size_t,
    ix_end: size_t,
) {
    let mut i: size_t = 0;
    i = ix_start;
    while i < ix_end {
        StoreH6(self_0, data, mask, i);
        i = i.wrapping_add(1);
    }
}
#[inline(always)]
unsafe extern "C" fn PrepareDistanceCacheH6(
    mut self_0: *mut H6,
    mut distance_cache: *mut libc::c_int,
) {
    PrepareDistanceCache(distance_cache, (*self_0).num_last_distances_to_check_);
}
#[inline(always)]
unsafe extern "C" fn FindLongestMatchH6(
    mut self_0: *mut H6,
    mut dictionary: *const BrotliEncoderDictionary,
    mut data: *const uint8_t,
    ring_buffer_mask: size_t,
    mut distance_cache: *const libc::c_int,
    cur_ix: size_t,
    max_length: size_t,
    max_backward: size_t,
    dictionary_distance: size_t,
    max_distance: size_t,
    mut out: *mut HasherSearchResult,
) {
    let mut num = (*self_0).num_;
    let mut buckets = (*self_0).buckets_;
    let cur_ix_masked = cur_ix & ring_buffer_mask;
    let mut min_score = (*out).score;
    let mut best_score = (*out).score;
    let mut best_len = (*out).len;
    let mut i: size_t = 0;
    (*out).len = 0 as libc::c_int as size_t;
    (*out).len_code_delta = 0 as libc::c_int;
    i = 0 as libc::c_int as size_t;
    while i < (*self_0).num_last_distances_to_check_ as size_t {
        let backward = *distance_cache.offset(i as isize) as size_t;
        let mut prev_ix = cur_ix.wrapping_sub(backward);
        if !(prev_ix >= cur_ix) {
            if !((backward > max_backward) as libc::c_int as libc::c_long != 0) {
                prev_ix &= ring_buffer_mask;
                if !(cur_ix_masked.wrapping_add(best_len) > ring_buffer_mask
                    || prev_ix.wrapping_add(best_len) > ring_buffer_mask
                    || *data.offset(cur_ix_masked.wrapping_add(best_len) as isize)
                        as libc::c_int
                        != *data.offset(prev_ix.wrapping_add(best_len) as isize)
                            as libc::c_int)
                {
                    let len = FindMatchLengthWithLimit(
                        &*data.offset(prev_ix as isize),
                        &*data.offset(cur_ix_masked as isize),
                        max_length,
                    );
                    if len >= 3 as libc::c_int as libc::c_ulong
                        || len == 2 as libc::c_int as libc::c_ulong
                            && i < 2 as libc::c_int as libc::c_ulong
                    {
                        let mut score = BackwardReferenceScoreUsingLastDistance(len);
                        if best_score < score {
                            if i != 0 as libc::c_int as libc::c_ulong {
                                score = (score as libc::c_ulong)
                                    .wrapping_sub(BackwardReferencePenaltyUsingLastDistance(i))
                                    as size_t as size_t;
                            }
                            if best_score < score {
                                best_score = score;
                                best_len = len;
                                (*out).len = best_len;
                                (*out).distance = backward;
                                (*out).score = best_score;
                            }
                        }
                    }
                }
            }
        }
        i = i.wrapping_add(1);
    }
    let key = HashBytesH6(
        &*data.offset(cur_ix_masked as isize),
        (*self_0).hash_mask_,
        (*self_0).hash_shift_,
    );
    let mut bucket: *mut uint32_t = &mut *buckets
        .offset((key << (*self_0).block_bits_) as isize) as *mut uint32_t;
    let down = if *num.offset(key as isize) as libc::c_ulong > (*self_0).block_size_ {
        (*num.offset(key as isize) as libc::c_ulong).wrapping_sub((*self_0).block_size_)
    } else {
        0 as libc::c_uint as libc::c_ulong
    };
    i = *num.offset(key as isize) as size_t;
    while i > down {
        i = i.wrapping_sub(1);
        let mut prev_ix_0 = *bucket
            .offset((i & (*self_0).block_mask_ as libc::c_ulong) as isize) as size_t;
        let backward_0 = cur_ix.wrapping_sub(prev_ix_0);
        if (backward_0 > max_backward) as libc::c_int as libc::c_long != 0 {
            break;
        }
        prev_ix_0 &= ring_buffer_mask;
        if cur_ix_masked.wrapping_add(best_len) > ring_buffer_mask
            || prev_ix_0.wrapping_add(best_len) > ring_buffer_mask
            || *data.offset(cur_ix_masked.wrapping_add(best_len) as isize) as libc::c_int
                != *data.offset(prev_ix_0.wrapping_add(best_len) as isize) as libc::c_int
        {
            continue;
        }
        let len_0 = FindMatchLengthWithLimit(
            &*data.offset(prev_ix_0 as isize),
            &*data.offset(cur_ix_masked as isize),
            max_length,
        );
        if len_0 >= 4 as libc::c_int as libc::c_ulong {
            let mut score_0 = BackwardReferenceScore(len_0, backward_0);
            if best_score < score_0 {
                best_score = score_0;
                best_len = len_0;
                (*out).len = best_len;
                (*out).distance = backward_0;
                (*out).score = best_score;
            }
        }
    }
    *bucket
        .offset(
            (*num.offset(key as isize) as libc::c_uint & (*self_0).block_mask_) as isize,
        ) = cur_ix as uint32_t;
    let ref mut fresh5 = *num.offset(key as isize);
    *fresh5 = (*fresh5).wrapping_add(1);
    if min_score == (*out).score {
        SearchInStaticDictionary(
            dictionary,
            (*self_0).common_,
            &*data.offset(cur_ix_masked as isize),
            max_length,
            dictionary_distance,
            max_distance,
            out,
            0 as libc::c_int,
        );
    }
}
#[inline(always)]
unsafe extern "C" fn HashTypeLengthH40() -> size_t {
    return 4 as libc::c_int as size_t;
}
#[inline(always)]
unsafe extern "C" fn StoreLookaheadH40() -> size_t {
    return 4 as libc::c_int as size_t;
}
#[inline(always)]
unsafe extern "C" fn HashBytesH40(mut data: *const uint8_t) -> size_t {
    let h = (BrotliUnalignedRead32(data as *const libc::c_void))
        .wrapping_mul(kHashMul32);
    return (h >> 32 as libc::c_int - 15 as libc::c_int) as size_t;
}
unsafe extern "C" fn AddrH40(mut extra: *mut libc::c_void) -> *mut uint32_t {
    return extra as *mut uint32_t;
}
unsafe extern "C" fn HeadH40(mut extra: *mut libc::c_void) -> *mut uint16_t {
    return &mut *((AddrH40
        as unsafe extern "C" fn(*mut libc::c_void) -> *mut uint32_t)(extra))
        .offset(((1 as libc::c_int) << 15 as libc::c_int) as isize) as *mut uint32_t
        as *mut uint16_t;
}
unsafe extern "C" fn TinyHashH40(mut extra: *mut libc::c_void) -> *mut uint8_t {
    return &mut *((HeadH40
        as unsafe extern "C" fn(*mut libc::c_void) -> *mut uint16_t)(extra))
        .offset(((1 as libc::c_int) << 15 as libc::c_int) as isize) as *mut uint16_t
        as *mut uint8_t;
}
unsafe extern "C" fn BanksH40(mut extra: *mut libc::c_void) -> *mut BankH40 {
    return &mut *((TinyHashH40
        as unsafe extern "C" fn(*mut libc::c_void) -> *mut uint8_t)(extra))
        .offset(65536 as libc::c_int as isize) as *mut uint8_t as *mut BankH40;
}
#[inline(always)]
unsafe extern "C" fn StoreH40(
    mut self_0: *mut H40,
    mut data: *const uint8_t,
    mask: size_t,
    ix: size_t,
) {
    let mut addr = AddrH40((*self_0).extra);
    let mut head = HeadH40((*self_0).extra);
    let mut tiny_hash = TinyHashH40((*self_0).extra);
    let mut banks = BanksH40((*self_0).extra);
    let key = HashBytesH40(&*data.offset((ix & mask) as isize));
    let bank = key & (1 as libc::c_int - 1 as libc::c_int) as libc::c_ulong;
    let ref mut fresh6 = (*self_0).free_slot_idx[bank as usize];
    let fresh7 = *fresh6;
    *fresh6 = (*fresh6).wrapping_add(1);
    let idx = (fresh7 as libc::c_int
        & ((1 as libc::c_int) << 16 as libc::c_int) - 1 as libc::c_int) as size_t;
    let mut delta = ix.wrapping_sub(*addr.offset(key as isize) as libc::c_ulong);
    *tiny_hash.offset(ix as uint16_t as isize) = key as uint8_t;
    if delta > 0xffff as libc::c_int as libc::c_ulong {
        delta = (if 0 as libc::c_int != 0 {
            0 as libc::c_int
        } else {
            0xffff as libc::c_int
        }) as size_t;
    }
    (*banks.offset(bank as isize)).slots[idx as usize].delta = delta as uint16_t;
    (*banks.offset(bank as isize)).slots[idx as usize].next = *head.offset(key as isize);
    *addr.offset(key as isize) = ix as uint32_t;
    *head.offset(key as isize) = idx as uint16_t;
}
#[inline(always)]
unsafe extern "C" fn StoreRangeH40(
    mut self_0: *mut H40,
    mut data: *const uint8_t,
    mask: size_t,
    ix_start: size_t,
    ix_end: size_t,
) {
    let mut i: size_t = 0;
    i = ix_start;
    while i < ix_end {
        StoreH40(self_0, data, mask, i);
        i = i.wrapping_add(1);
    }
}
#[inline(always)]
unsafe extern "C" fn PrepareDistanceCacheH40(
    mut self_0: *mut H40,
    mut distance_cache: *mut libc::c_int,
) {
    PrepareDistanceCache(distance_cache, 4 as libc::c_int);
}
#[inline(always)]
unsafe extern "C" fn FindLongestMatchH40(
    mut self_0: *mut H40,
    mut dictionary: *const BrotliEncoderDictionary,
    mut data: *const uint8_t,
    ring_buffer_mask: size_t,
    mut distance_cache: *const libc::c_int,
    cur_ix: size_t,
    max_length: size_t,
    max_backward: size_t,
    dictionary_distance: size_t,
    max_distance: size_t,
    mut out: *mut HasherSearchResult,
) {
    let mut addr = AddrH40((*self_0).extra);
    let mut head = HeadH40((*self_0).extra);
    let mut tiny_hashes = TinyHashH40((*self_0).extra);
    let mut banks = BanksH40((*self_0).extra);
    let cur_ix_masked = cur_ix & ring_buffer_mask;
    let mut min_score = (*out).score;
    let mut best_score = (*out).score;
    let mut best_len = (*out).len;
    let mut i: size_t = 0;
    let key = HashBytesH40(&*data.offset(cur_ix_masked as isize));
    let tiny_hash = key as uint8_t;
    (*out).len = 0 as libc::c_int as size_t;
    (*out).len_code_delta = 0 as libc::c_int;
    i = 0 as libc::c_int as size_t;
    while i < 4 as libc::c_int as libc::c_ulong {
        let backward = *distance_cache.offset(i as isize) as size_t;
        let mut prev_ix = cur_ix.wrapping_sub(backward);
        if !(i > 0 as libc::c_int as libc::c_ulong
            && *tiny_hashes.offset(prev_ix as uint16_t as isize) as libc::c_int
                != tiny_hash as libc::c_int)
        {
            if !(prev_ix >= cur_ix || backward > max_backward) {
                prev_ix &= ring_buffer_mask;
                let len = FindMatchLengthWithLimit(
                    &*data.offset(prev_ix as isize),
                    &*data.offset(cur_ix_masked as isize),
                    max_length,
                );
                if len >= 2 as libc::c_int as libc::c_ulong {
                    let mut score = BackwardReferenceScoreUsingLastDistance(len);
                    if best_score < score {
                        if i != 0 as libc::c_int as libc::c_ulong {
                            score = (score as libc::c_ulong)
                                .wrapping_sub(BackwardReferencePenaltyUsingLastDistance(i))
                                as size_t as size_t;
                        }
                        if best_score < score {
                            best_score = score;
                            best_len = len;
                            (*out).len = best_len;
                            (*out).distance = backward;
                            (*out).score = best_score;
                        }
                    }
                }
            }
        }
        i = i.wrapping_add(1);
    }
    let bank = key & (1 as libc::c_int - 1 as libc::c_int) as libc::c_ulong;
    let mut backward_0 = 0 as libc::c_int as size_t;
    let mut hops = (*self_0).max_hops;
    let mut delta = cur_ix.wrapping_sub(*addr.offset(key as isize) as libc::c_ulong);
    let mut slot = *head.offset(key as isize) as size_t;
    loop {
        let fresh8 = hops;
        hops = hops.wrapping_sub(1);
        if !(fresh8 != 0) {
            break;
        }
        let mut prev_ix_0: size_t = 0;
        let mut last = slot;
        backward_0 = (backward_0 as libc::c_ulong).wrapping_add(delta) as size_t
            as size_t;
        if backward_0 > max_backward || 0 as libc::c_int != 0 && delta == 0 {
            break;
        }
        prev_ix_0 = cur_ix.wrapping_sub(backward_0) & ring_buffer_mask;
        slot = (*banks.offset(bank as isize)).slots[last as usize].next as size_t;
        delta = (*banks.offset(bank as isize)).slots[last as usize].delta as size_t;
        if cur_ix_masked.wrapping_add(best_len) > ring_buffer_mask
            || prev_ix_0.wrapping_add(best_len) > ring_buffer_mask
            || *data.offset(cur_ix_masked.wrapping_add(best_len) as isize) as libc::c_int
                != *data.offset(prev_ix_0.wrapping_add(best_len) as isize) as libc::c_int
        {
            continue;
        }
        let len_0 = FindMatchLengthWithLimit(
            &*data.offset(prev_ix_0 as isize),
            &*data.offset(cur_ix_masked as isize),
            max_length,
        );
        if len_0 >= 4 as libc::c_int as libc::c_ulong {
            let mut score_0 = BackwardReferenceScore(len_0, backward_0);
            if best_score < score_0 {
                best_score = score_0;
                best_len = len_0;
                (*out).len = best_len;
                (*out).distance = backward_0;
                (*out).score = best_score;
            }
        }
    }
    StoreH40(self_0, data, ring_buffer_mask, cur_ix);
    if (*out).score == min_score {
        SearchInStaticDictionary(
            dictionary,
            (*self_0).common,
            &*data.offset(cur_ix_masked as isize),
            max_length,
            dictionary_distance,
            max_distance,
            out,
            0 as libc::c_int,
        );
    }
}
#[inline(always)]
unsafe extern "C" fn HashTypeLengthH41() -> size_t {
    return 4 as libc::c_int as size_t;
}
#[inline(always)]
unsafe extern "C" fn StoreLookaheadH41() -> size_t {
    return 4 as libc::c_int as size_t;
}
#[inline(always)]
unsafe extern "C" fn HashBytesH41(mut data: *const uint8_t) -> size_t {
    let h = (BrotliUnalignedRead32(data as *const libc::c_void))
        .wrapping_mul(kHashMul32);
    return (h >> 32 as libc::c_int - 15 as libc::c_int) as size_t;
}
unsafe extern "C" fn AddrH41(mut extra: *mut libc::c_void) -> *mut uint32_t {
    return extra as *mut uint32_t;
}
unsafe extern "C" fn HeadH41(mut extra: *mut libc::c_void) -> *mut uint16_t {
    return &mut *((AddrH41
        as unsafe extern "C" fn(*mut libc::c_void) -> *mut uint32_t)(extra))
        .offset(((1 as libc::c_int) << 15 as libc::c_int) as isize) as *mut uint32_t
        as *mut uint16_t;
}
unsafe extern "C" fn TinyHashH41(mut extra: *mut libc::c_void) -> *mut uint8_t {
    return &mut *((HeadH41
        as unsafe extern "C" fn(*mut libc::c_void) -> *mut uint16_t)(extra))
        .offset(((1 as libc::c_int) << 15 as libc::c_int) as isize) as *mut uint16_t
        as *mut uint8_t;
}
unsafe extern "C" fn BanksH41(mut extra: *mut libc::c_void) -> *mut BankH41 {
    return &mut *((TinyHashH41
        as unsafe extern "C" fn(*mut libc::c_void) -> *mut uint8_t)(extra))
        .offset(65536 as libc::c_int as isize) as *mut uint8_t as *mut BankH41;
}
#[inline(always)]
unsafe extern "C" fn StoreH41(
    mut self_0: *mut H41,
    mut data: *const uint8_t,
    mask: size_t,
    ix: size_t,
) {
    let mut addr = AddrH41((*self_0).extra);
    let mut head = HeadH41((*self_0).extra);
    let mut tiny_hash = TinyHashH41((*self_0).extra);
    let mut banks = BanksH41((*self_0).extra);
    let key = HashBytesH41(&*data.offset((ix & mask) as isize));
    let bank = key & (1 as libc::c_int - 1 as libc::c_int) as libc::c_ulong;
    let ref mut fresh9 = (*self_0).free_slot_idx[bank as usize];
    let fresh10 = *fresh9;
    *fresh9 = (*fresh9).wrapping_add(1);
    let idx = (fresh10 as libc::c_int
        & ((1 as libc::c_int) << 16 as libc::c_int) - 1 as libc::c_int) as size_t;
    let mut delta = ix.wrapping_sub(*addr.offset(key as isize) as libc::c_ulong);
    *tiny_hash.offset(ix as uint16_t as isize) = key as uint8_t;
    if delta > 0xffff as libc::c_int as libc::c_ulong {
        delta = (if 0 as libc::c_int != 0 {
            0 as libc::c_int
        } else {
            0xffff as libc::c_int
        }) as size_t;
    }
    (*banks.offset(bank as isize)).slots[idx as usize].delta = delta as uint16_t;
    (*banks.offset(bank as isize)).slots[idx as usize].next = *head.offset(key as isize);
    *addr.offset(key as isize) = ix as uint32_t;
    *head.offset(key as isize) = idx as uint16_t;
}
#[inline(always)]
unsafe extern "C" fn StoreRangeH41(
    mut self_0: *mut H41,
    mut data: *const uint8_t,
    mask: size_t,
    ix_start: size_t,
    ix_end: size_t,
) {
    let mut i: size_t = 0;
    i = ix_start;
    while i < ix_end {
        StoreH41(self_0, data, mask, i);
        i = i.wrapping_add(1);
    }
}
#[inline(always)]
unsafe extern "C" fn PrepareDistanceCacheH41(
    mut self_0: *mut H41,
    mut distance_cache: *mut libc::c_int,
) {
    PrepareDistanceCache(distance_cache, 10 as libc::c_int);
}
#[inline(always)]
unsafe extern "C" fn FindLongestMatchH41(
    mut self_0: *mut H41,
    mut dictionary: *const BrotliEncoderDictionary,
    mut data: *const uint8_t,
    ring_buffer_mask: size_t,
    mut distance_cache: *const libc::c_int,
    cur_ix: size_t,
    max_length: size_t,
    max_backward: size_t,
    dictionary_distance: size_t,
    max_distance: size_t,
    mut out: *mut HasherSearchResult,
) {
    let mut addr = AddrH41((*self_0).extra);
    let mut head = HeadH41((*self_0).extra);
    let mut tiny_hashes = TinyHashH41((*self_0).extra);
    let mut banks = BanksH41((*self_0).extra);
    let cur_ix_masked = cur_ix & ring_buffer_mask;
    let mut min_score = (*out).score;
    let mut best_score = (*out).score;
    let mut best_len = (*out).len;
    let mut i: size_t = 0;
    let key = HashBytesH41(&*data.offset(cur_ix_masked as isize));
    let tiny_hash = key as uint8_t;
    (*out).len = 0 as libc::c_int as size_t;
    (*out).len_code_delta = 0 as libc::c_int;
    i = 0 as libc::c_int as size_t;
    while i < 10 as libc::c_int as libc::c_ulong {
        let backward = *distance_cache.offset(i as isize) as size_t;
        let mut prev_ix = cur_ix.wrapping_sub(backward);
        if !(i > 0 as libc::c_int as libc::c_ulong
            && *tiny_hashes.offset(prev_ix as uint16_t as isize) as libc::c_int
                != tiny_hash as libc::c_int)
        {
            if !(prev_ix >= cur_ix || backward > max_backward) {
                prev_ix &= ring_buffer_mask;
                let len = FindMatchLengthWithLimit(
                    &*data.offset(prev_ix as isize),
                    &*data.offset(cur_ix_masked as isize),
                    max_length,
                );
                if len >= 2 as libc::c_int as libc::c_ulong {
                    let mut score = BackwardReferenceScoreUsingLastDistance(len);
                    if best_score < score {
                        if i != 0 as libc::c_int as libc::c_ulong {
                            score = (score as libc::c_ulong)
                                .wrapping_sub(BackwardReferencePenaltyUsingLastDistance(i))
                                as size_t as size_t;
                        }
                        if best_score < score {
                            best_score = score;
                            best_len = len;
                            (*out).len = best_len;
                            (*out).distance = backward;
                            (*out).score = best_score;
                        }
                    }
                }
            }
        }
        i = i.wrapping_add(1);
    }
    let bank = key & (1 as libc::c_int - 1 as libc::c_int) as libc::c_ulong;
    let mut backward_0 = 0 as libc::c_int as size_t;
    let mut hops = (*self_0).max_hops;
    let mut delta = cur_ix.wrapping_sub(*addr.offset(key as isize) as libc::c_ulong);
    let mut slot = *head.offset(key as isize) as size_t;
    loop {
        let fresh11 = hops;
        hops = hops.wrapping_sub(1);
        if !(fresh11 != 0) {
            break;
        }
        let mut prev_ix_0: size_t = 0;
        let mut last = slot;
        backward_0 = (backward_0 as libc::c_ulong).wrapping_add(delta) as size_t
            as size_t;
        if backward_0 > max_backward || 0 as libc::c_int != 0 && delta == 0 {
            break;
        }
        prev_ix_0 = cur_ix.wrapping_sub(backward_0) & ring_buffer_mask;
        slot = (*banks.offset(bank as isize)).slots[last as usize].next as size_t;
        delta = (*banks.offset(bank as isize)).slots[last as usize].delta as size_t;
        if cur_ix_masked.wrapping_add(best_len) > ring_buffer_mask
            || prev_ix_0.wrapping_add(best_len) > ring_buffer_mask
            || *data.offset(cur_ix_masked.wrapping_add(best_len) as isize) as libc::c_int
                != *data.offset(prev_ix_0.wrapping_add(best_len) as isize) as libc::c_int
        {
            continue;
        }
        let len_0 = FindMatchLengthWithLimit(
            &*data.offset(prev_ix_0 as isize),
            &*data.offset(cur_ix_masked as isize),
            max_length,
        );
        if len_0 >= 4 as libc::c_int as libc::c_ulong {
            let mut score_0 = BackwardReferenceScore(len_0, backward_0);
            if best_score < score_0 {
                best_score = score_0;
                best_len = len_0;
                (*out).len = best_len;
                (*out).distance = backward_0;
                (*out).score = best_score;
            }
        }
    }
    StoreH41(self_0, data, ring_buffer_mask, cur_ix);
    if (*out).score == min_score {
        SearchInStaticDictionary(
            dictionary,
            (*self_0).common,
            &*data.offset(cur_ix_masked as isize),
            max_length,
            dictionary_distance,
            max_distance,
            out,
            0 as libc::c_int,
        );
    }
}
#[inline(always)]
unsafe extern "C" fn HashTypeLengthH42() -> size_t {
    return 4 as libc::c_int as size_t;
}
#[inline(always)]
unsafe extern "C" fn StoreLookaheadH42() -> size_t {
    return 4 as libc::c_int as size_t;
}
#[inline(always)]
unsafe extern "C" fn HashBytesH42(mut data: *const uint8_t) -> size_t {
    let h = (BrotliUnalignedRead32(data as *const libc::c_void))
        .wrapping_mul(kHashMul32);
    return (h >> 32 as libc::c_int - 15 as libc::c_int) as size_t;
}
unsafe extern "C" fn AddrH42(mut extra: *mut libc::c_void) -> *mut uint32_t {
    return extra as *mut uint32_t;
}
unsafe extern "C" fn HeadH42(mut extra: *mut libc::c_void) -> *mut uint16_t {
    return &mut *((AddrH42
        as unsafe extern "C" fn(*mut libc::c_void) -> *mut uint32_t)(extra))
        .offset(((1 as libc::c_int) << 15 as libc::c_int) as isize) as *mut uint32_t
        as *mut uint16_t;
}
unsafe extern "C" fn TinyHashH42(mut extra: *mut libc::c_void) -> *mut uint8_t {
    return &mut *((HeadH42
        as unsafe extern "C" fn(*mut libc::c_void) -> *mut uint16_t)(extra))
        .offset(((1 as libc::c_int) << 15 as libc::c_int) as isize) as *mut uint16_t
        as *mut uint8_t;
}
unsafe extern "C" fn BanksH42(mut extra: *mut libc::c_void) -> *mut BankH42 {
    return &mut *((TinyHashH42
        as unsafe extern "C" fn(*mut libc::c_void) -> *mut uint8_t)(extra))
        .offset(65536 as libc::c_int as isize) as *mut uint8_t as *mut BankH42;
}
#[inline(always)]
unsafe extern "C" fn StoreH42(
    mut self_0: *mut H42,
    mut data: *const uint8_t,
    mask: size_t,
    ix: size_t,
) {
    let mut addr = AddrH42((*self_0).extra);
    let mut head = HeadH42((*self_0).extra);
    let mut tiny_hash = TinyHashH42((*self_0).extra);
    let mut banks = BanksH42((*self_0).extra);
    let key = HashBytesH42(&*data.offset((ix & mask) as isize));
    let bank = key & (512 as libc::c_int - 1 as libc::c_int) as libc::c_ulong;
    let ref mut fresh12 = (*self_0).free_slot_idx[bank as usize];
    let fresh13 = *fresh12;
    *fresh12 = (*fresh12).wrapping_add(1);
    let idx = (fresh13 as libc::c_int
        & ((1 as libc::c_int) << 9 as libc::c_int) - 1 as libc::c_int) as size_t;
    let mut delta = ix.wrapping_sub(*addr.offset(key as isize) as libc::c_ulong);
    *tiny_hash.offset(ix as uint16_t as isize) = key as uint8_t;
    if delta > 0xffff as libc::c_int as libc::c_ulong {
        delta = (if 0 as libc::c_int != 0 {
            0 as libc::c_int
        } else {
            0xffff as libc::c_int
        }) as size_t;
    }
    (*banks.offset(bank as isize)).slots[idx as usize].delta = delta as uint16_t;
    (*banks.offset(bank as isize)).slots[idx as usize].next = *head.offset(key as isize);
    *addr.offset(key as isize) = ix as uint32_t;
    *head.offset(key as isize) = idx as uint16_t;
}
#[inline(always)]
unsafe extern "C" fn StoreRangeH42(
    mut self_0: *mut H42,
    mut data: *const uint8_t,
    mask: size_t,
    ix_start: size_t,
    ix_end: size_t,
) {
    let mut i: size_t = 0;
    i = ix_start;
    while i < ix_end {
        StoreH42(self_0, data, mask, i);
        i = i.wrapping_add(1);
    }
}
#[inline(always)]
unsafe extern "C" fn PrepareDistanceCacheH42(
    mut self_0: *mut H42,
    mut distance_cache: *mut libc::c_int,
) {
    PrepareDistanceCache(distance_cache, 16 as libc::c_int);
}
#[inline(always)]
unsafe extern "C" fn FindLongestMatchH42(
    mut self_0: *mut H42,
    mut dictionary: *const BrotliEncoderDictionary,
    mut data: *const uint8_t,
    ring_buffer_mask: size_t,
    mut distance_cache: *const libc::c_int,
    cur_ix: size_t,
    max_length: size_t,
    max_backward: size_t,
    dictionary_distance: size_t,
    max_distance: size_t,
    mut out: *mut HasherSearchResult,
) {
    let mut addr = AddrH42((*self_0).extra);
    let mut head = HeadH42((*self_0).extra);
    let mut tiny_hashes = TinyHashH42((*self_0).extra);
    let mut banks = BanksH42((*self_0).extra);
    let cur_ix_masked = cur_ix & ring_buffer_mask;
    let mut min_score = (*out).score;
    let mut best_score = (*out).score;
    let mut best_len = (*out).len;
    let mut i: size_t = 0;
    let key = HashBytesH42(&*data.offset(cur_ix_masked as isize));
    let tiny_hash = key as uint8_t;
    (*out).len = 0 as libc::c_int as size_t;
    (*out).len_code_delta = 0 as libc::c_int;
    i = 0 as libc::c_int as size_t;
    while i < 16 as libc::c_int as libc::c_ulong {
        let backward = *distance_cache.offset(i as isize) as size_t;
        let mut prev_ix = cur_ix.wrapping_sub(backward);
        if !(i > 0 as libc::c_int as libc::c_ulong
            && *tiny_hashes.offset(prev_ix as uint16_t as isize) as libc::c_int
                != tiny_hash as libc::c_int)
        {
            if !(prev_ix >= cur_ix || backward > max_backward) {
                prev_ix &= ring_buffer_mask;
                let len = FindMatchLengthWithLimit(
                    &*data.offset(prev_ix as isize),
                    &*data.offset(cur_ix_masked as isize),
                    max_length,
                );
                if len >= 2 as libc::c_int as libc::c_ulong {
                    let mut score = BackwardReferenceScoreUsingLastDistance(len);
                    if best_score < score {
                        if i != 0 as libc::c_int as libc::c_ulong {
                            score = (score as libc::c_ulong)
                                .wrapping_sub(BackwardReferencePenaltyUsingLastDistance(i))
                                as size_t as size_t;
                        }
                        if best_score < score {
                            best_score = score;
                            best_len = len;
                            (*out).len = best_len;
                            (*out).distance = backward;
                            (*out).score = best_score;
                        }
                    }
                }
            }
        }
        i = i.wrapping_add(1);
    }
    let bank = key & (512 as libc::c_int - 1 as libc::c_int) as libc::c_ulong;
    let mut backward_0 = 0 as libc::c_int as size_t;
    let mut hops = (*self_0).max_hops;
    let mut delta = cur_ix.wrapping_sub(*addr.offset(key as isize) as libc::c_ulong);
    let mut slot = *head.offset(key as isize) as size_t;
    loop {
        let fresh14 = hops;
        hops = hops.wrapping_sub(1);
        if !(fresh14 != 0) {
            break;
        }
        let mut prev_ix_0: size_t = 0;
        let mut last = slot;
        backward_0 = (backward_0 as libc::c_ulong).wrapping_add(delta) as size_t
            as size_t;
        if backward_0 > max_backward || 0 as libc::c_int != 0 && delta == 0 {
            break;
        }
        prev_ix_0 = cur_ix.wrapping_sub(backward_0) & ring_buffer_mask;
        slot = (*banks.offset(bank as isize)).slots[last as usize].next as size_t;
        delta = (*banks.offset(bank as isize)).slots[last as usize].delta as size_t;
        if cur_ix_masked.wrapping_add(best_len) > ring_buffer_mask
            || prev_ix_0.wrapping_add(best_len) > ring_buffer_mask
            || *data.offset(cur_ix_masked.wrapping_add(best_len) as isize) as libc::c_int
                != *data.offset(prev_ix_0.wrapping_add(best_len) as isize) as libc::c_int
        {
            continue;
        }
        let len_0 = FindMatchLengthWithLimit(
            &*data.offset(prev_ix_0 as isize),
            &*data.offset(cur_ix_masked as isize),
            max_length,
        );
        if len_0 >= 4 as libc::c_int as libc::c_ulong {
            let mut score_0 = BackwardReferenceScore(len_0, backward_0);
            if best_score < score_0 {
                best_score = score_0;
                best_len = len_0;
                (*out).len = best_len;
                (*out).distance = backward_0;
                (*out).score = best_score;
            }
        }
    }
    StoreH42(self_0, data, ring_buffer_mask, cur_ix);
    if (*out).score == min_score {
        SearchInStaticDictionary(
            dictionary,
            (*self_0).common,
            &*data.offset(cur_ix_masked as isize),
            max_length,
            dictionary_distance,
            max_distance,
            out,
            0 as libc::c_int,
        );
    }
}
#[inline(always)]
unsafe extern "C" fn HashTypeLengthH54() -> size_t {
    return 8 as libc::c_int as size_t;
}
#[inline(always)]
unsafe extern "C" fn StoreLookaheadH54() -> size_t {
    return 8 as libc::c_int as size_t;
}
unsafe extern "C" fn HashBytesH54(mut data: *const uint8_t) -> uint32_t {
    let h = (BrotliUnalignedRead64(data as *const libc::c_void)
        << 64 as libc::c_int - 8 as libc::c_int * 7 as libc::c_int)
        .wrapping_mul(kHashMul64);
    return (h >> 64 as libc::c_int - 20 as libc::c_int) as uint32_t;
}
#[inline(always)]
unsafe extern "C" fn StoreH54(
    mut self_0: *mut H54,
    mut data: *const uint8_t,
    mask: size_t,
    ix: size_t,
) {
    let key = HashBytesH54(&*data.offset((ix & mask) as isize));
    if (1 as libc::c_int) << 2 as libc::c_int == 1 as libc::c_int {
        *((*self_0).buckets_).offset(key as isize) = ix as uint32_t;
    } else {
        let off = (ix
            & ((((1 as libc::c_int) << 2 as libc::c_int) - 1 as libc::c_int)
                << 3 as libc::c_int) as libc::c_ulong) as uint32_t;
        *((*self_0).buckets_)
            .offset(
                (key.wrapping_add(off)
                    & (((1 as libc::c_int) << 20 as libc::c_int) - 1 as libc::c_int)
                        as libc::c_uint) as isize,
            ) = ix as uint32_t;
    };
}
#[inline(always)]
unsafe extern "C" fn StoreRangeH54(
    mut self_0: *mut H54,
    mut data: *const uint8_t,
    mask: size_t,
    ix_start: size_t,
    ix_end: size_t,
) {
    let mut i: size_t = 0;
    i = ix_start;
    while i < ix_end {
        StoreH54(self_0, data, mask, i);
        i = i.wrapping_add(1);
    }
}
#[inline(always)]
unsafe extern "C" fn PrepareDistanceCacheH54(
    mut self_0: *mut H54,
    mut distance_cache: *mut libc::c_int,
) {}
#[inline(always)]
unsafe extern "C" fn FindLongestMatchH54(
    mut self_0: *mut H54,
    mut dictionary: *const BrotliEncoderDictionary,
    mut data: *const uint8_t,
    ring_buffer_mask: size_t,
    mut distance_cache: *const libc::c_int,
    cur_ix: size_t,
    max_length: size_t,
    max_backward: size_t,
    dictionary_distance: size_t,
    max_distance: size_t,
    mut out: *mut HasherSearchResult,
) {
    let mut buckets = (*self_0).buckets_;
    let best_len_in = (*out).len;
    let cur_ix_masked = cur_ix & ring_buffer_mask;
    let mut compare_char = *data.offset(cur_ix_masked.wrapping_add(best_len_in) as isize)
        as libc::c_int;
    let mut key = HashBytesH54(&*data.offset(cur_ix_masked as isize)) as size_t;
    let mut key_out: size_t = 0;
    let mut min_score = (*out).score;
    let mut best_score = (*out).score;
    let mut best_len = best_len_in;
    let mut cached_backward = *distance_cache.offset(0 as libc::c_int as isize)
        as size_t;
    let mut prev_ix = cur_ix.wrapping_sub(cached_backward);
    (*out).len_code_delta = 0 as libc::c_int;
    if prev_ix < cur_ix {
        prev_ix &= ring_buffer_mask as uint32_t as libc::c_ulong;
        if compare_char
            == *data.offset(prev_ix.wrapping_add(best_len) as isize) as libc::c_int
        {
            let len = FindMatchLengthWithLimit(
                &*data.offset(prev_ix as isize),
                &*data.offset(cur_ix_masked as isize),
                max_length,
            );
            if len >= 4 as libc::c_int as libc::c_ulong {
                let score = BackwardReferenceScoreUsingLastDistance(len);
                if best_score < score {
                    (*out).len = len;
                    (*out).distance = cached_backward;
                    (*out).score = score;
                    if (1 as libc::c_int) << 2 as libc::c_int == 1 as libc::c_int {
                        *buckets.offset(key as isize) = cur_ix as uint32_t;
                        return;
                    } else {
                        best_len = len;
                        best_score = score;
                        compare_char = *data
                            .offset(cur_ix_masked.wrapping_add(len) as isize)
                            as libc::c_int;
                    }
                }
            }
        }
    }
    {
        let mut keys: [size_t; 4] = [0; 4];
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_ulong {
            keys[i
                as usize] = key.wrapping_add(i << 3 as libc::c_int)
                & (((1 as libc::c_int) << 20 as libc::c_int) - 1 as libc::c_int)
                    as libc::c_ulong;
            i = i.wrapping_add(1);
        }
        key_out = keys[((cur_ix
            & ((((1 as libc::c_int) << 2 as libc::c_int) - 1 as libc::c_int)
                << 3 as libc::c_int) as libc::c_ulong) >> 3 as libc::c_int) as usize];
        i = 0 as libc::c_int as size_t;
        while i < ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_ulong {
            let mut len_1: size_t = 0;
            let mut backward_0: size_t = 0;
            prev_ix = *buckets.offset(keys[i as usize] as isize) as size_t;
            backward_0 = cur_ix.wrapping_sub(prev_ix);
            prev_ix &= ring_buffer_mask as uint32_t as libc::c_ulong;
            if !(compare_char
                != *data.offset(prev_ix.wrapping_add(best_len) as isize) as libc::c_int)
            {
                if !((backward_0 == 0 as libc::c_int as libc::c_ulong
                    || backward_0 > max_backward) as libc::c_int as libc::c_long != 0)
                {
                    len_1 = FindMatchLengthWithLimit(
                        &*data.offset(prev_ix as isize),
                        &*data.offset(cur_ix_masked as isize),
                        max_length,
                    );
                    if len_1 >= 4 as libc::c_int as libc::c_ulong {
                        let score_1 = BackwardReferenceScore(len_1, backward_0);
                        if best_score < score_1 {
                            best_len = len_1;
                            (*out).len = len_1;
                            compare_char = *data
                                .offset(cur_ix_masked.wrapping_add(len_1) as isize)
                                as libc::c_int;
                            best_score = score_1;
                            (*out).score = score_1;
                            (*out).distance = backward_0;
                        }
                    }
                }
            }
            i = i.wrapping_add(1);
        }
    }
    if (1 as libc::c_int) << 2 as libc::c_int != 1 as libc::c_int {
        *buckets.offset(key_out as isize) = cur_ix as uint32_t;
    }
}
static mut kInvalidPosHROLLING_FAST: uint32_t = 0xffffffff as libc::c_uint;
#[inline(always)]
unsafe extern "C" fn HashTypeLengthHROLLING_FAST() -> size_t {
    return 4 as libc::c_int as size_t;
}
#[inline(always)]
unsafe extern "C" fn StoreLookaheadHROLLING_FAST() -> size_t {
    return 4 as libc::c_int as size_t;
}
unsafe extern "C" fn HashByteHROLLING_FAST(mut byte: uint8_t) -> uint32_t {
    return (byte as uint32_t).wrapping_add(1 as libc::c_uint);
}
unsafe extern "C" fn HashRollingFunctionHROLLING_FAST(
    mut state: uint32_t,
    mut add: uint8_t,
    mut rem: uint8_t,
    mut factor: uint32_t,
    mut factor_remove: uint32_t,
) -> uint32_t {
    return factor
        .wrapping_mul(state)
        .wrapping_add(HashByteHROLLING_FAST(add))
        .wrapping_sub(factor_remove.wrapping_mul(HashByteHROLLING_FAST(rem)));
}
#[inline(always)]
unsafe extern "C" fn StoreHROLLING_FAST(
    mut self_0: *mut HROLLING_FAST,
    mut data: *const uint8_t,
    mask: size_t,
    ix: size_t,
) {}
#[inline(always)]
unsafe extern "C" fn StoreRangeHROLLING_FAST(
    mut self_0: *mut HROLLING_FAST,
    mut data: *const uint8_t,
    mask: size_t,
    ix_start: size_t,
    ix_end: size_t,
) {}
#[inline(always)]
unsafe extern "C" fn PrepareDistanceCacheHROLLING_FAST(
    mut self_0: *mut HROLLING_FAST,
    mut distance_cache: *mut libc::c_int,
) {}
#[inline(always)]
unsafe extern "C" fn FindLongestMatchHROLLING_FAST(
    mut self_0: *mut HROLLING_FAST,
    mut dictionary: *const BrotliEncoderDictionary,
    mut data: *const uint8_t,
    ring_buffer_mask: size_t,
    mut distance_cache: *const libc::c_int,
    cur_ix: size_t,
    max_length: size_t,
    max_backward: size_t,
    dictionary_distance: size_t,
    max_distance: size_t,
    mut out: *mut HasherSearchResult,
) {
    let cur_ix_masked = cur_ix & ring_buffer_mask;
    let mut pos: size_t = 0;
    if cur_ix & (4 as libc::c_int - 1 as libc::c_int) as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        return;
    }
    if max_length < 32 as libc::c_int as libc::c_ulong {
        return;
    }
    pos = (*self_0).next_ix;
    while pos <= cur_ix {
        let mut code = (*self_0).state
            & (16777216 as libc::c_int * 64 as libc::c_int - 1 as libc::c_int)
                as libc::c_uint;
        let mut rem = *data.offset((pos & ring_buffer_mask) as isize);
        let mut add = *data
            .offset(
                (pos.wrapping_add(32 as libc::c_int as libc::c_ulong) & ring_buffer_mask)
                    as isize,
            );
        let mut found_ix = kInvalidPosHROLLING_FAST as size_t;
        (*self_0)
            .state = HashRollingFunctionHROLLING_FAST(
            (*self_0).state,
            add,
            rem,
            (*self_0).factor,
            (*self_0).factor_remove,
        );
        if code < 16777216 as libc::c_int as libc::c_uint {
            found_ix = *((*self_0).table).offset(code as isize) as size_t;
            *((*self_0).table).offset(code as isize) = pos as uint32_t;
            if pos == cur_ix && found_ix != kInvalidPosHROLLING_FAST as libc::c_ulong {
                let mut backward = cur_ix.wrapping_sub(found_ix) as uint32_t as size_t;
                if backward <= max_backward {
                    let found_ix_masked = found_ix & ring_buffer_mask;
                    let len = FindMatchLengthWithLimit(
                        &*data.offset(found_ix_masked as isize),
                        &*data.offset(cur_ix_masked as isize),
                        max_length,
                    );
                    if len >= 4 as libc::c_int as libc::c_ulong && len > (*out).len {
                        let mut score = BackwardReferenceScore(len, backward);
                        if score > (*out).score {
                            (*out).len = len;
                            (*out).distance = backward;
                            (*out).score = score;
                            (*out).len_code_delta = 0 as libc::c_int;
                        }
                    }
                }
            }
        }
        pos = (pos as libc::c_ulong).wrapping_add(4 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
    }
    (*self_0).next_ix = cur_ix.wrapping_add(4 as libc::c_int as libc::c_ulong);
}
static mut kInvalidPosHROLLING: uint32_t = 0xffffffff as libc::c_uint;
#[inline(always)]
unsafe extern "C" fn HashTypeLengthHROLLING() -> size_t {
    return 4 as libc::c_int as size_t;
}
#[inline(always)]
unsafe extern "C" fn StoreLookaheadHROLLING() -> size_t {
    return 4 as libc::c_int as size_t;
}
unsafe extern "C" fn HashByteHROLLING(mut byte: uint8_t) -> uint32_t {
    return (byte as uint32_t).wrapping_add(1 as libc::c_uint);
}
unsafe extern "C" fn HashRollingFunctionHROLLING(
    mut state: uint32_t,
    mut add: uint8_t,
    mut rem: uint8_t,
    mut factor: uint32_t,
    mut factor_remove: uint32_t,
) -> uint32_t {
    return factor
        .wrapping_mul(state)
        .wrapping_add(HashByteHROLLING(add))
        .wrapping_sub(factor_remove.wrapping_mul(HashByteHROLLING(rem)));
}
#[inline(always)]
unsafe extern "C" fn StoreHROLLING(
    mut self_0: *mut HROLLING,
    mut data: *const uint8_t,
    mask: size_t,
    ix: size_t,
) {}
#[inline(always)]
unsafe extern "C" fn StoreRangeHROLLING(
    mut self_0: *mut HROLLING,
    mut data: *const uint8_t,
    mask: size_t,
    ix_start: size_t,
    ix_end: size_t,
) {}
#[inline(always)]
unsafe extern "C" fn PrepareDistanceCacheHROLLING(
    mut self_0: *mut HROLLING,
    mut distance_cache: *mut libc::c_int,
) {}
#[inline(always)]
unsafe extern "C" fn FindLongestMatchHROLLING(
    mut self_0: *mut HROLLING,
    mut dictionary: *const BrotliEncoderDictionary,
    mut data: *const uint8_t,
    ring_buffer_mask: size_t,
    mut distance_cache: *const libc::c_int,
    cur_ix: size_t,
    max_length: size_t,
    max_backward: size_t,
    dictionary_distance: size_t,
    max_distance: size_t,
    mut out: *mut HasherSearchResult,
) {
    let cur_ix_masked = cur_ix & ring_buffer_mask;
    let mut pos: size_t = 0;
    if cur_ix & (1 as libc::c_int - 1 as libc::c_int) as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        return;
    }
    if max_length < 32 as libc::c_int as libc::c_ulong {
        return;
    }
    pos = (*self_0).next_ix;
    while pos <= cur_ix {
        let mut code = (*self_0).state
            & (16777216 as libc::c_int * 64 as libc::c_int - 1 as libc::c_int)
                as libc::c_uint;
        let mut rem = *data.offset((pos & ring_buffer_mask) as isize);
        let mut add = *data
            .offset(
                (pos.wrapping_add(32 as libc::c_int as libc::c_ulong) & ring_buffer_mask)
                    as isize,
            );
        let mut found_ix = kInvalidPosHROLLING as size_t;
        (*self_0)
            .state = HashRollingFunctionHROLLING(
            (*self_0).state,
            add,
            rem,
            (*self_0).factor,
            (*self_0).factor_remove,
        );
        if code < 16777216 as libc::c_int as libc::c_uint {
            found_ix = *((*self_0).table).offset(code as isize) as size_t;
            *((*self_0).table).offset(code as isize) = pos as uint32_t;
            if pos == cur_ix && found_ix != kInvalidPosHROLLING as libc::c_ulong {
                let mut backward = cur_ix.wrapping_sub(found_ix) as uint32_t as size_t;
                if backward <= max_backward {
                    let found_ix_masked = found_ix & ring_buffer_mask;
                    let len = FindMatchLengthWithLimit(
                        &*data.offset(found_ix_masked as isize),
                        &*data.offset(cur_ix_masked as isize),
                        max_length,
                    );
                    if len >= 4 as libc::c_int as libc::c_ulong && len > (*out).len {
                        let mut score = BackwardReferenceScore(len, backward);
                        if score > (*out).score {
                            (*out).len = len;
                            (*out).distance = backward;
                            (*out).score = score;
                            (*out).len_code_delta = 0 as libc::c_int;
                        }
                    }
                }
            }
        }
        pos = (pos as libc::c_ulong).wrapping_add(1 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
    }
    (*self_0).next_ix = cur_ix.wrapping_add(1 as libc::c_int as libc::c_ulong);
}
#[inline(always)]
unsafe extern "C" fn HashTypeLengthH35() -> size_t {
    let mut a = HashTypeLengthH3();
    let mut b = HashTypeLengthHROLLING_FAST();
    return if a > b { a } else { b };
}
#[inline(always)]
unsafe extern "C" fn StoreLookaheadH35() -> size_t {
    let mut a = StoreLookaheadH3();
    let mut b = StoreLookaheadHROLLING_FAST();
    return if a > b { a } else { b };
}
#[inline(always)]
unsafe extern "C" fn StoreH35(
    mut self_0: *mut H35,
    mut data: *const uint8_t,
    mask: size_t,
    ix: size_t,
) {
    StoreH3(&mut (*self_0).ha, data, mask, ix);
    StoreHROLLING_FAST(&mut (*self_0).hb, data, mask, ix);
}
#[inline(always)]
unsafe extern "C" fn StoreRangeH35(
    mut self_0: *mut H35,
    mut data: *const uint8_t,
    mask: size_t,
    ix_start: size_t,
    ix_end: size_t,
) {
    StoreRangeH3(&mut (*self_0).ha, data, mask, ix_start, ix_end);
    StoreRangeHROLLING_FAST(&mut (*self_0).hb, data, mask, ix_start, ix_end);
}
#[inline(always)]
unsafe extern "C" fn PrepareDistanceCacheH35(
    mut self_0: *mut H35,
    mut distance_cache: *mut libc::c_int,
) {
    PrepareDistanceCacheH3(&mut (*self_0).ha, distance_cache);
    PrepareDistanceCacheHROLLING_FAST(&mut (*self_0).hb, distance_cache);
}
#[inline(always)]
unsafe extern "C" fn FindLongestMatchH35(
    mut self_0: *mut H35,
    mut dictionary: *const BrotliEncoderDictionary,
    mut data: *const uint8_t,
    ring_buffer_mask: size_t,
    mut distance_cache: *const libc::c_int,
    cur_ix: size_t,
    max_length: size_t,
    max_backward: size_t,
    dictionary_distance: size_t,
    max_distance: size_t,
    mut out: *mut HasherSearchResult,
) {
    FindLongestMatchH3(
        &mut (*self_0).ha,
        dictionary,
        data,
        ring_buffer_mask,
        distance_cache,
        cur_ix,
        max_length,
        max_backward,
        dictionary_distance,
        max_distance,
        out,
    );
    FindLongestMatchHROLLING_FAST(
        &mut (*self_0).hb,
        dictionary,
        data,
        ring_buffer_mask,
        distance_cache,
        cur_ix,
        max_length,
        max_backward,
        dictionary_distance,
        max_distance,
        out,
    );
}
#[inline(always)]
unsafe extern "C" fn HashTypeLengthH55() -> size_t {
    let mut a = HashTypeLengthH54();
    let mut b = HashTypeLengthHROLLING_FAST();
    return if a > b { a } else { b };
}
#[inline(always)]
unsafe extern "C" fn StoreLookaheadH55() -> size_t {
    let mut a = StoreLookaheadH54();
    let mut b = StoreLookaheadHROLLING_FAST();
    return if a > b { a } else { b };
}
#[inline(always)]
unsafe extern "C" fn StoreH55(
    mut self_0: *mut H55,
    mut data: *const uint8_t,
    mask: size_t,
    ix: size_t,
) {
    StoreH54(&mut (*self_0).ha, data, mask, ix);
    StoreHROLLING_FAST(&mut (*self_0).hb, data, mask, ix);
}
#[inline(always)]
unsafe extern "C" fn StoreRangeH55(
    mut self_0: *mut H55,
    mut data: *const uint8_t,
    mask: size_t,
    ix_start: size_t,
    ix_end: size_t,
) {
    StoreRangeH54(&mut (*self_0).ha, data, mask, ix_start, ix_end);
    StoreRangeHROLLING_FAST(&mut (*self_0).hb, data, mask, ix_start, ix_end);
}
#[inline(always)]
unsafe extern "C" fn PrepareDistanceCacheH55(
    mut self_0: *mut H55,
    mut distance_cache: *mut libc::c_int,
) {
    PrepareDistanceCacheH54(&mut (*self_0).ha, distance_cache);
    PrepareDistanceCacheHROLLING_FAST(&mut (*self_0).hb, distance_cache);
}
#[inline(always)]
unsafe extern "C" fn FindLongestMatchH55(
    mut self_0: *mut H55,
    mut dictionary: *const BrotliEncoderDictionary,
    mut data: *const uint8_t,
    ring_buffer_mask: size_t,
    mut distance_cache: *const libc::c_int,
    cur_ix: size_t,
    max_length: size_t,
    max_backward: size_t,
    dictionary_distance: size_t,
    max_distance: size_t,
    mut out: *mut HasherSearchResult,
) {
    FindLongestMatchH54(
        &mut (*self_0).ha,
        dictionary,
        data,
        ring_buffer_mask,
        distance_cache,
        cur_ix,
        max_length,
        max_backward,
        dictionary_distance,
        max_distance,
        out,
    );
    FindLongestMatchHROLLING_FAST(
        &mut (*self_0).hb,
        dictionary,
        data,
        ring_buffer_mask,
        distance_cache,
        cur_ix,
        max_length,
        max_backward,
        dictionary_distance,
        max_distance,
        out,
    );
}
#[inline(always)]
unsafe extern "C" fn HashTypeLengthH65() -> size_t {
    let mut a = HashTypeLengthH6();
    let mut b = HashTypeLengthHROLLING();
    return if a > b { a } else { b };
}
#[inline(always)]
unsafe extern "C" fn StoreLookaheadH65() -> size_t {
    let mut a = StoreLookaheadH6();
    let mut b = StoreLookaheadHROLLING();
    return if a > b { a } else { b };
}
#[inline(always)]
unsafe extern "C" fn StoreH65(
    mut self_0: *mut H65,
    mut data: *const uint8_t,
    mask: size_t,
    ix: size_t,
) {
    StoreH6(&mut (*self_0).ha, data, mask, ix);
    StoreHROLLING(&mut (*self_0).hb, data, mask, ix);
}
#[inline(always)]
unsafe extern "C" fn StoreRangeH65(
    mut self_0: *mut H65,
    mut data: *const uint8_t,
    mask: size_t,
    ix_start: size_t,
    ix_end: size_t,
) {
    StoreRangeH6(&mut (*self_0).ha, data, mask, ix_start, ix_end);
    StoreRangeHROLLING(&mut (*self_0).hb, data, mask, ix_start, ix_end);
}
#[inline(always)]
unsafe extern "C" fn PrepareDistanceCacheH65(
    mut self_0: *mut H65,
    mut distance_cache: *mut libc::c_int,
) {
    PrepareDistanceCacheH6(&mut (*self_0).ha, distance_cache);
    PrepareDistanceCacheHROLLING(&mut (*self_0).hb, distance_cache);
}
#[inline(always)]
unsafe extern "C" fn FindLongestMatchH65(
    mut self_0: *mut H65,
    mut dictionary: *const BrotliEncoderDictionary,
    mut data: *const uint8_t,
    ring_buffer_mask: size_t,
    mut distance_cache: *const libc::c_int,
    cur_ix: size_t,
    max_length: size_t,
    max_backward: size_t,
    dictionary_distance: size_t,
    max_distance: size_t,
    mut out: *mut HasherSearchResult,
) {
    FindLongestMatchH6(
        &mut (*self_0).ha,
        dictionary,
        data,
        ring_buffer_mask,
        distance_cache,
        cur_ix,
        max_length,
        max_backward,
        dictionary_distance,
        max_distance,
        out,
    );
    FindLongestMatchHROLLING(
        &mut (*self_0).hb,
        dictionary,
        data,
        ring_buffer_mask,
        distance_cache,
        cur_ix,
        max_length,
        max_backward,
        dictionary_distance,
        max_distance,
        out,
    );
}
#[inline(always)]
unsafe extern "C" fn ComputeDistanceCode(
    mut distance: size_t,
    mut max_distance: size_t,
    mut dist_cache: *const libc::c_int,
) -> size_t {
    if distance <= max_distance {
        let mut distance_plus_3 = distance
            .wrapping_add(3 as libc::c_int as libc::c_ulong);
        let mut offset0 = distance_plus_3
            .wrapping_sub(*dist_cache.offset(0 as libc::c_int as isize) as size_t);
        let mut offset1 = distance_plus_3
            .wrapping_sub(*dist_cache.offset(1 as libc::c_int as isize) as size_t);
        if distance == *dist_cache.offset(0 as libc::c_int as isize) as size_t {
            return 0 as libc::c_int as size_t
        } else {
            if distance == *dist_cache.offset(1 as libc::c_int as isize) as size_t {
                return 1 as libc::c_int as size_t
            } else {
                if offset0 < 7 as libc::c_int as libc::c_ulong {
                    return (0x9750468 as libc::c_int
                        >> (4 as libc::c_int as libc::c_ulong).wrapping_mul(offset0)
                        & 0xf as libc::c_int) as size_t
                } else {
                    if offset1 < 7 as libc::c_int as libc::c_ulong {
                        return (0xfdb1ace as libc::c_int
                            >> (4 as libc::c_int as libc::c_ulong).wrapping_mul(offset1)
                            & 0xf as libc::c_int) as size_t
                    } else {
                        if distance
                            == *dist_cache.offset(2 as libc::c_int as isize) as size_t
                        {
                            return 2 as libc::c_int as size_t
                        } else {
                            if distance
                                == *dist_cache.offset(3 as libc::c_int as isize) as size_t
                            {
                                return 3 as libc::c_int as size_t;
                            }
                        }
                    }
                }
            }
        }
    }
    return distance
        .wrapping_add(16 as libc::c_int as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong);
}
#[inline(never)]
unsafe extern "C" fn CreateBackwardReferencesNH54(
    mut num_bytes: size_t,
    mut position: size_t,
    mut ringbuffer: *const uint8_t,
    mut ringbuffer_mask: size_t,
    mut literal_context_lut: ContextLut,
    mut params: *const BrotliEncoderParams,
    mut hasher: *mut Hasher,
    mut dist_cache: *mut libc::c_int,
    mut last_insert_len: *mut size_t,
    mut commands: *mut Command,
    mut num_commands: *mut size_t,
    mut num_literals: *mut size_t,
) {
    let mut privat: *mut H54 = &mut (*hasher).privat._H54;
    let max_backward_limit = ((1 as libc::c_int as size_t) << (*params).lgwin)
        .wrapping_sub(16 as libc::c_int as libc::c_ulong);
    let position_offset = (*params).stream_offset;
    let orig_commands: *const Command = commands;
    let mut insert_length = *last_insert_len;
    let pos_end = position.wrapping_add(num_bytes);
    let store_end = if num_bytes >= StoreLookaheadH54() {
        position
            .wrapping_add(num_bytes)
            .wrapping_sub(StoreLookaheadH54())
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
    } else {
        position
    };
    let random_heuristics_window_size = LiteralSpreeLengthForSparseSearch(params);
    let mut apply_random_heuristics = position
        .wrapping_add(random_heuristics_window_size);
    let gap = 0 as libc::c_int as size_t;
    let kMinScore = ((30 as libc::c_int * 8 as libc::c_int) as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<size_t>() as libc::c_ulong)
        .wrapping_add(100 as libc::c_int as libc::c_ulong);
    PrepareDistanceCacheH54(privat, dist_cache);
    while position.wrapping_add(HashTypeLengthH54()) < pos_end {
        let mut max_length = pos_end.wrapping_sub(position);
        let mut max_distance = brotli_min_size_t(position, max_backward_limit);
        let mut dictionary_start = brotli_min_size_t(
            position.wrapping_add(position_offset),
            max_backward_limit,
        );
        let mut sr = HasherSearchResult {
            len: 0,
            distance: 0,
            score: 0,
            len_code_delta: 0,
        };
        sr.len = 0 as libc::c_int as size_t;
        sr.len_code_delta = 0 as libc::c_int;
        sr.distance = 0 as libc::c_int as size_t;
        sr.score = kMinScore;
        FindLongestMatchH54(
            privat,
            &(*params).dictionary,
            ringbuffer,
            ringbuffer_mask,
            dist_cache,
            position,
            max_length,
            max_distance,
            dictionary_start.wrapping_add(gap),
            (*params).dist.max_distance,
            &mut sr,
        );
        if sr.score > kMinScore {
            let mut delayed_backward_references_in_row = 0 as libc::c_int;
            max_length = max_length.wrapping_sub(1);
            loop {
                let cost_diff_lazy = 175 as libc::c_int as size_t;
                let mut sr2 = HasherSearchResult {
                    len: 0,
                    distance: 0,
                    score: 0,
                    len_code_delta: 0,
                };
                sr2
                    .len = if (*params).quality < 5 as libc::c_int {
                    brotli_min_size_t(
                        (sr.len).wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        max_length,
                    )
                } else {
                    0 as libc::c_int as libc::c_ulong
                };
                sr2.len_code_delta = 0 as libc::c_int;
                sr2.distance = 0 as libc::c_int as size_t;
                sr2.score = kMinScore;
                max_distance = brotli_min_size_t(
                    position.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    max_backward_limit,
                );
                dictionary_start = brotli_min_size_t(
                    position
                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(position_offset),
                    max_backward_limit,
                );
                FindLongestMatchH54(
                    privat,
                    &(*params).dictionary,
                    ringbuffer,
                    ringbuffer_mask,
                    dist_cache,
                    position.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    max_length,
                    max_distance,
                    dictionary_start.wrapping_add(gap),
                    (*params).dist.max_distance,
                    &mut sr2,
                );
                if !(sr2.score >= (sr.score).wrapping_add(cost_diff_lazy)) {
                    break;
                }
                position = position.wrapping_add(1);
                insert_length = insert_length.wrapping_add(1);
                sr = sr2;
                delayed_backward_references_in_row += 1;
                if !(delayed_backward_references_in_row < 4 as libc::c_int
                    && position.wrapping_add(HashTypeLengthH54()) < pos_end)
                {
                    break;
                }
                max_length = max_length.wrapping_sub(1);
            }
            apply_random_heuristics = position
                .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(sr.len))
                .wrapping_add(random_heuristics_window_size);
            dictionary_start = brotli_min_size_t(
                position.wrapping_add(position_offset),
                max_backward_limit,
            );
            let mut distance_code = ComputeDistanceCode(
                sr.distance,
                dictionary_start.wrapping_add(gap),
                dist_cache,
            );
            if sr.distance <= dictionary_start.wrapping_add(gap)
                && distance_code > 0 as libc::c_int as libc::c_ulong
            {
                *dist_cache
                    .offset(
                        3 as libc::c_int as isize,
                    ) = *dist_cache.offset(2 as libc::c_int as isize);
                *dist_cache
                    .offset(
                        2 as libc::c_int as isize,
                    ) = *dist_cache.offset(1 as libc::c_int as isize);
                *dist_cache
                    .offset(
                        1 as libc::c_int as isize,
                    ) = *dist_cache.offset(0 as libc::c_int as isize);
                *dist_cache
                    .offset(0 as libc::c_int as isize) = sr.distance as libc::c_int;
                PrepareDistanceCacheH54(privat, dist_cache);
            }
            let fresh15 = commands;
            commands = commands.offset(1);
            InitCommand(
                fresh15,
                &(*params).dist,
                insert_length,
                sr.len,
                sr.len_code_delta,
                distance_code,
            );
            *num_literals = (*num_literals as libc::c_ulong).wrapping_add(insert_length)
                as size_t as size_t;
            insert_length = 0 as libc::c_int as size_t;
            let mut range_start = position
                .wrapping_add(2 as libc::c_int as libc::c_ulong);
            let mut range_end = brotli_min_size_t(
                position.wrapping_add(sr.len),
                store_end,
            );
            if sr.distance < sr.len >> 2 as libc::c_int {
                range_start = brotli_min_size_t(
                    range_end,
                    brotli_max_size_t(
                        range_start,
                        position
                            .wrapping_add(sr.len)
                            .wrapping_sub(sr.distance << 2 as libc::c_int),
                    ),
                );
            }
            StoreRangeH54(privat, ringbuffer, ringbuffer_mask, range_start, range_end);
            position = (position as libc::c_ulong).wrapping_add(sr.len) as size_t
                as size_t;
        } else {
            insert_length = insert_length.wrapping_add(1);
            position = position.wrapping_add(1);
            if position > apply_random_heuristics {
                if position
                    > apply_random_heuristics
                        .wrapping_add(
                            (4 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(random_heuristics_window_size),
                        )
                {
                    let kMargin = brotli_max_size_t(
                        (StoreLookaheadH54())
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        4 as libc::c_int as size_t,
                    );
                    let mut pos_jump = brotli_min_size_t(
                        position.wrapping_add(16 as libc::c_int as libc::c_ulong),
                        pos_end.wrapping_sub(kMargin),
                    );
                    while position < pos_jump {
                        StoreH54(privat, ringbuffer, ringbuffer_mask, position);
                        insert_length = (insert_length as libc::c_ulong)
                            .wrapping_add(4 as libc::c_int as libc::c_ulong) as size_t
                            as size_t;
                        position = (position as libc::c_ulong)
                            .wrapping_add(4 as libc::c_int as libc::c_ulong) as size_t
                            as size_t;
                    }
                } else {
                    let kMargin_0 = brotli_max_size_t(
                        (StoreLookaheadH54())
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        2 as libc::c_int as size_t,
                    );
                    let mut pos_jump_0 = brotli_min_size_t(
                        position.wrapping_add(8 as libc::c_int as libc::c_ulong),
                        pos_end.wrapping_sub(kMargin_0),
                    );
                    while position < pos_jump_0 {
                        StoreH54(privat, ringbuffer, ringbuffer_mask, position);
                        insert_length = (insert_length as libc::c_ulong)
                            .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t
                            as size_t;
                        position = (position as libc::c_ulong)
                            .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t
                            as size_t;
                    }
                }
            }
        }
    }
    insert_length = (insert_length as libc::c_ulong)
        .wrapping_add(pos_end.wrapping_sub(position)) as size_t as size_t;
    *last_insert_len = insert_length;
    *num_commands = (*num_commands as libc::c_ulong)
        .wrapping_add(commands.offset_from(orig_commands) as libc::c_long as size_t)
        as size_t as size_t;
}
#[inline(never)]
unsafe extern "C" fn CreateBackwardReferencesNH35(
    mut num_bytes: size_t,
    mut position: size_t,
    mut ringbuffer: *const uint8_t,
    mut ringbuffer_mask: size_t,
    mut literal_context_lut: ContextLut,
    mut params: *const BrotliEncoderParams,
    mut hasher: *mut Hasher,
    mut dist_cache: *mut libc::c_int,
    mut last_insert_len: *mut size_t,
    mut commands: *mut Command,
    mut num_commands: *mut size_t,
    mut num_literals: *mut size_t,
) {
    let mut privat: *mut H35 = &mut (*hasher).privat._H35;
    let max_backward_limit = ((1 as libc::c_int as size_t) << (*params).lgwin)
        .wrapping_sub(16 as libc::c_int as libc::c_ulong);
    let position_offset = (*params).stream_offset;
    let orig_commands: *const Command = commands;
    let mut insert_length = *last_insert_len;
    let pos_end = position.wrapping_add(num_bytes);
    let store_end = if num_bytes >= StoreLookaheadH35() {
        position
            .wrapping_add(num_bytes)
            .wrapping_sub(StoreLookaheadH35())
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
    } else {
        position
    };
    let random_heuristics_window_size = LiteralSpreeLengthForSparseSearch(params);
    let mut apply_random_heuristics = position
        .wrapping_add(random_heuristics_window_size);
    let gap = 0 as libc::c_int as size_t;
    let kMinScore = ((30 as libc::c_int * 8 as libc::c_int) as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<size_t>() as libc::c_ulong)
        .wrapping_add(100 as libc::c_int as libc::c_ulong);
    PrepareDistanceCacheH35(privat, dist_cache);
    while position.wrapping_add(HashTypeLengthH35()) < pos_end {
        let mut max_length = pos_end.wrapping_sub(position);
        let mut max_distance = brotli_min_size_t(position, max_backward_limit);
        let mut dictionary_start = brotli_min_size_t(
            position.wrapping_add(position_offset),
            max_backward_limit,
        );
        let mut sr = HasherSearchResult {
            len: 0,
            distance: 0,
            score: 0,
            len_code_delta: 0,
        };
        sr.len = 0 as libc::c_int as size_t;
        sr.len_code_delta = 0 as libc::c_int;
        sr.distance = 0 as libc::c_int as size_t;
        sr.score = kMinScore;
        FindLongestMatchH35(
            privat,
            &(*params).dictionary,
            ringbuffer,
            ringbuffer_mask,
            dist_cache,
            position,
            max_length,
            max_distance,
            dictionary_start.wrapping_add(gap),
            (*params).dist.max_distance,
            &mut sr,
        );
        if sr.score > kMinScore {
            let mut delayed_backward_references_in_row = 0 as libc::c_int;
            max_length = max_length.wrapping_sub(1);
            loop {
                let cost_diff_lazy = 175 as libc::c_int as size_t;
                let mut sr2 = HasherSearchResult {
                    len: 0,
                    distance: 0,
                    score: 0,
                    len_code_delta: 0,
                };
                sr2
                    .len = if (*params).quality < 5 as libc::c_int {
                    brotli_min_size_t(
                        (sr.len).wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        max_length,
                    )
                } else {
                    0 as libc::c_int as libc::c_ulong
                };
                sr2.len_code_delta = 0 as libc::c_int;
                sr2.distance = 0 as libc::c_int as size_t;
                sr2.score = kMinScore;
                max_distance = brotli_min_size_t(
                    position.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    max_backward_limit,
                );
                dictionary_start = brotli_min_size_t(
                    position
                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(position_offset),
                    max_backward_limit,
                );
                FindLongestMatchH35(
                    privat,
                    &(*params).dictionary,
                    ringbuffer,
                    ringbuffer_mask,
                    dist_cache,
                    position.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    max_length,
                    max_distance,
                    dictionary_start.wrapping_add(gap),
                    (*params).dist.max_distance,
                    &mut sr2,
                );
                if !(sr2.score >= (sr.score).wrapping_add(cost_diff_lazy)) {
                    break;
                }
                position = position.wrapping_add(1);
                insert_length = insert_length.wrapping_add(1);
                sr = sr2;
                delayed_backward_references_in_row += 1;
                if !(delayed_backward_references_in_row < 4 as libc::c_int
                    && position.wrapping_add(HashTypeLengthH35()) < pos_end)
                {
                    break;
                }
                max_length = max_length.wrapping_sub(1);
            }
            apply_random_heuristics = position
                .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(sr.len))
                .wrapping_add(random_heuristics_window_size);
            dictionary_start = brotli_min_size_t(
                position.wrapping_add(position_offset),
                max_backward_limit,
            );
            let mut distance_code = ComputeDistanceCode(
                sr.distance,
                dictionary_start.wrapping_add(gap),
                dist_cache,
            );
            if sr.distance <= dictionary_start.wrapping_add(gap)
                && distance_code > 0 as libc::c_int as libc::c_ulong
            {
                *dist_cache
                    .offset(
                        3 as libc::c_int as isize,
                    ) = *dist_cache.offset(2 as libc::c_int as isize);
                *dist_cache
                    .offset(
                        2 as libc::c_int as isize,
                    ) = *dist_cache.offset(1 as libc::c_int as isize);
                *dist_cache
                    .offset(
                        1 as libc::c_int as isize,
                    ) = *dist_cache.offset(0 as libc::c_int as isize);
                *dist_cache
                    .offset(0 as libc::c_int as isize) = sr.distance as libc::c_int;
                PrepareDistanceCacheH35(privat, dist_cache);
            }
            let fresh16 = commands;
            commands = commands.offset(1);
            InitCommand(
                fresh16,
                &(*params).dist,
                insert_length,
                sr.len,
                sr.len_code_delta,
                distance_code,
            );
            *num_literals = (*num_literals as libc::c_ulong).wrapping_add(insert_length)
                as size_t as size_t;
            insert_length = 0 as libc::c_int as size_t;
            let mut range_start = position
                .wrapping_add(2 as libc::c_int as libc::c_ulong);
            let mut range_end = brotli_min_size_t(
                position.wrapping_add(sr.len),
                store_end,
            );
            if sr.distance < sr.len >> 2 as libc::c_int {
                range_start = brotli_min_size_t(
                    range_end,
                    brotli_max_size_t(
                        range_start,
                        position
                            .wrapping_add(sr.len)
                            .wrapping_sub(sr.distance << 2 as libc::c_int),
                    ),
                );
            }
            StoreRangeH35(privat, ringbuffer, ringbuffer_mask, range_start, range_end);
            position = (position as libc::c_ulong).wrapping_add(sr.len) as size_t
                as size_t;
        } else {
            insert_length = insert_length.wrapping_add(1);
            position = position.wrapping_add(1);
            if position > apply_random_heuristics {
                if position
                    > apply_random_heuristics
                        .wrapping_add(
                            (4 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(random_heuristics_window_size),
                        )
                {
                    let kMargin = brotli_max_size_t(
                        (StoreLookaheadH35())
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        4 as libc::c_int as size_t,
                    );
                    let mut pos_jump = brotli_min_size_t(
                        position.wrapping_add(16 as libc::c_int as libc::c_ulong),
                        pos_end.wrapping_sub(kMargin),
                    );
                    while position < pos_jump {
                        StoreH35(privat, ringbuffer, ringbuffer_mask, position);
                        insert_length = (insert_length as libc::c_ulong)
                            .wrapping_add(4 as libc::c_int as libc::c_ulong) as size_t
                            as size_t;
                        position = (position as libc::c_ulong)
                            .wrapping_add(4 as libc::c_int as libc::c_ulong) as size_t
                            as size_t;
                    }
                } else {
                    let kMargin_0 = brotli_max_size_t(
                        (StoreLookaheadH35())
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        2 as libc::c_int as size_t,
                    );
                    let mut pos_jump_0 = brotli_min_size_t(
                        position.wrapping_add(8 as libc::c_int as libc::c_ulong),
                        pos_end.wrapping_sub(kMargin_0),
                    );
                    while position < pos_jump_0 {
                        StoreH35(privat, ringbuffer, ringbuffer_mask, position);
                        insert_length = (insert_length as libc::c_ulong)
                            .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t
                            as size_t;
                        position = (position as libc::c_ulong)
                            .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t
                            as size_t;
                    }
                }
            }
        }
    }
    insert_length = (insert_length as libc::c_ulong)
        .wrapping_add(pos_end.wrapping_sub(position)) as size_t as size_t;
    *last_insert_len = insert_length;
    *num_commands = (*num_commands as libc::c_ulong)
        .wrapping_add(commands.offset_from(orig_commands) as libc::c_long as size_t)
        as size_t as size_t;
}
#[inline(never)]
unsafe extern "C" fn CreateBackwardReferencesNH42(
    mut num_bytes: size_t,
    mut position: size_t,
    mut ringbuffer: *const uint8_t,
    mut ringbuffer_mask: size_t,
    mut literal_context_lut: ContextLut,
    mut params: *const BrotliEncoderParams,
    mut hasher: *mut Hasher,
    mut dist_cache: *mut libc::c_int,
    mut last_insert_len: *mut size_t,
    mut commands: *mut Command,
    mut num_commands: *mut size_t,
    mut num_literals: *mut size_t,
) {
    let mut privat: *mut H42 = &mut (*hasher).privat._H42;
    let max_backward_limit = ((1 as libc::c_int as size_t) << (*params).lgwin)
        .wrapping_sub(16 as libc::c_int as libc::c_ulong);
    let position_offset = (*params).stream_offset;
    let orig_commands: *const Command = commands;
    let mut insert_length = *last_insert_len;
    let pos_end = position.wrapping_add(num_bytes);
    let store_end = if num_bytes >= StoreLookaheadH42() {
        position
            .wrapping_add(num_bytes)
            .wrapping_sub(StoreLookaheadH42())
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
    } else {
        position
    };
    let random_heuristics_window_size = LiteralSpreeLengthForSparseSearch(params);
    let mut apply_random_heuristics = position
        .wrapping_add(random_heuristics_window_size);
    let gap = 0 as libc::c_int as size_t;
    let kMinScore = ((30 as libc::c_int * 8 as libc::c_int) as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<size_t>() as libc::c_ulong)
        .wrapping_add(100 as libc::c_int as libc::c_ulong);
    PrepareDistanceCacheH42(privat, dist_cache);
    while position.wrapping_add(HashTypeLengthH42()) < pos_end {
        let mut max_length = pos_end.wrapping_sub(position);
        let mut max_distance = brotli_min_size_t(position, max_backward_limit);
        let mut dictionary_start = brotli_min_size_t(
            position.wrapping_add(position_offset),
            max_backward_limit,
        );
        let mut sr = HasherSearchResult {
            len: 0,
            distance: 0,
            score: 0,
            len_code_delta: 0,
        };
        sr.len = 0 as libc::c_int as size_t;
        sr.len_code_delta = 0 as libc::c_int;
        sr.distance = 0 as libc::c_int as size_t;
        sr.score = kMinScore;
        FindLongestMatchH42(
            privat,
            &(*params).dictionary,
            ringbuffer,
            ringbuffer_mask,
            dist_cache,
            position,
            max_length,
            max_distance,
            dictionary_start.wrapping_add(gap),
            (*params).dist.max_distance,
            &mut sr,
        );
        if sr.score > kMinScore {
            let mut delayed_backward_references_in_row = 0 as libc::c_int;
            max_length = max_length.wrapping_sub(1);
            loop {
                let cost_diff_lazy = 175 as libc::c_int as size_t;
                let mut sr2 = HasherSearchResult {
                    len: 0,
                    distance: 0,
                    score: 0,
                    len_code_delta: 0,
                };
                sr2
                    .len = if (*params).quality < 5 as libc::c_int {
                    brotli_min_size_t(
                        (sr.len).wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        max_length,
                    )
                } else {
                    0 as libc::c_int as libc::c_ulong
                };
                sr2.len_code_delta = 0 as libc::c_int;
                sr2.distance = 0 as libc::c_int as size_t;
                sr2.score = kMinScore;
                max_distance = brotli_min_size_t(
                    position.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    max_backward_limit,
                );
                dictionary_start = brotli_min_size_t(
                    position
                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(position_offset),
                    max_backward_limit,
                );
                FindLongestMatchH42(
                    privat,
                    &(*params).dictionary,
                    ringbuffer,
                    ringbuffer_mask,
                    dist_cache,
                    position.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    max_length,
                    max_distance,
                    dictionary_start.wrapping_add(gap),
                    (*params).dist.max_distance,
                    &mut sr2,
                );
                if !(sr2.score >= (sr.score).wrapping_add(cost_diff_lazy)) {
                    break;
                }
                position = position.wrapping_add(1);
                insert_length = insert_length.wrapping_add(1);
                sr = sr2;
                delayed_backward_references_in_row += 1;
                if !(delayed_backward_references_in_row < 4 as libc::c_int
                    && position.wrapping_add(HashTypeLengthH42()) < pos_end)
                {
                    break;
                }
                max_length = max_length.wrapping_sub(1);
            }
            apply_random_heuristics = position
                .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(sr.len))
                .wrapping_add(random_heuristics_window_size);
            dictionary_start = brotli_min_size_t(
                position.wrapping_add(position_offset),
                max_backward_limit,
            );
            let mut distance_code = ComputeDistanceCode(
                sr.distance,
                dictionary_start.wrapping_add(gap),
                dist_cache,
            );
            if sr.distance <= dictionary_start.wrapping_add(gap)
                && distance_code > 0 as libc::c_int as libc::c_ulong
            {
                *dist_cache
                    .offset(
                        3 as libc::c_int as isize,
                    ) = *dist_cache.offset(2 as libc::c_int as isize);
                *dist_cache
                    .offset(
                        2 as libc::c_int as isize,
                    ) = *dist_cache.offset(1 as libc::c_int as isize);
                *dist_cache
                    .offset(
                        1 as libc::c_int as isize,
                    ) = *dist_cache.offset(0 as libc::c_int as isize);
                *dist_cache
                    .offset(0 as libc::c_int as isize) = sr.distance as libc::c_int;
                PrepareDistanceCacheH42(privat, dist_cache);
            }
            let fresh17 = commands;
            commands = commands.offset(1);
            InitCommand(
                fresh17,
                &(*params).dist,
                insert_length,
                sr.len,
                sr.len_code_delta,
                distance_code,
            );
            *num_literals = (*num_literals as libc::c_ulong).wrapping_add(insert_length)
                as size_t as size_t;
            insert_length = 0 as libc::c_int as size_t;
            let mut range_start = position
                .wrapping_add(2 as libc::c_int as libc::c_ulong);
            let mut range_end = brotli_min_size_t(
                position.wrapping_add(sr.len),
                store_end,
            );
            if sr.distance < sr.len >> 2 as libc::c_int {
                range_start = brotli_min_size_t(
                    range_end,
                    brotli_max_size_t(
                        range_start,
                        position
                            .wrapping_add(sr.len)
                            .wrapping_sub(sr.distance << 2 as libc::c_int),
                    ),
                );
            }
            StoreRangeH42(privat, ringbuffer, ringbuffer_mask, range_start, range_end);
            position = (position as libc::c_ulong).wrapping_add(sr.len) as size_t
                as size_t;
        } else {
            insert_length = insert_length.wrapping_add(1);
            position = position.wrapping_add(1);
            if position > apply_random_heuristics {
                if position
                    > apply_random_heuristics
                        .wrapping_add(
                            (4 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(random_heuristics_window_size),
                        )
                {
                    let kMargin = brotli_max_size_t(
                        (StoreLookaheadH42())
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        4 as libc::c_int as size_t,
                    );
                    let mut pos_jump = brotli_min_size_t(
                        position.wrapping_add(16 as libc::c_int as libc::c_ulong),
                        pos_end.wrapping_sub(kMargin),
                    );
                    while position < pos_jump {
                        StoreH42(privat, ringbuffer, ringbuffer_mask, position);
                        insert_length = (insert_length as libc::c_ulong)
                            .wrapping_add(4 as libc::c_int as libc::c_ulong) as size_t
                            as size_t;
                        position = (position as libc::c_ulong)
                            .wrapping_add(4 as libc::c_int as libc::c_ulong) as size_t
                            as size_t;
                    }
                } else {
                    let kMargin_0 = brotli_max_size_t(
                        (StoreLookaheadH42())
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        2 as libc::c_int as size_t,
                    );
                    let mut pos_jump_0 = brotli_min_size_t(
                        position.wrapping_add(8 as libc::c_int as libc::c_ulong),
                        pos_end.wrapping_sub(kMargin_0),
                    );
                    while position < pos_jump_0 {
                        StoreH42(privat, ringbuffer, ringbuffer_mask, position);
                        insert_length = (insert_length as libc::c_ulong)
                            .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t
                            as size_t;
                        position = (position as libc::c_ulong)
                            .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t
                            as size_t;
                    }
                }
            }
        }
    }
    insert_length = (insert_length as libc::c_ulong)
        .wrapping_add(pos_end.wrapping_sub(position)) as size_t as size_t;
    *last_insert_len = insert_length;
    *num_commands = (*num_commands as libc::c_ulong)
        .wrapping_add(commands.offset_from(orig_commands) as libc::c_long as size_t)
        as size_t as size_t;
}
#[inline(never)]
unsafe extern "C" fn CreateBackwardReferencesNH41(
    mut num_bytes: size_t,
    mut position: size_t,
    mut ringbuffer: *const uint8_t,
    mut ringbuffer_mask: size_t,
    mut literal_context_lut: ContextLut,
    mut params: *const BrotliEncoderParams,
    mut hasher: *mut Hasher,
    mut dist_cache: *mut libc::c_int,
    mut last_insert_len: *mut size_t,
    mut commands: *mut Command,
    mut num_commands: *mut size_t,
    mut num_literals: *mut size_t,
) {
    let mut privat: *mut H41 = &mut (*hasher).privat._H41;
    let max_backward_limit = ((1 as libc::c_int as size_t) << (*params).lgwin)
        .wrapping_sub(16 as libc::c_int as libc::c_ulong);
    let position_offset = (*params).stream_offset;
    let orig_commands: *const Command = commands;
    let mut insert_length = *last_insert_len;
    let pos_end = position.wrapping_add(num_bytes);
    let store_end = if num_bytes >= StoreLookaheadH41() {
        position
            .wrapping_add(num_bytes)
            .wrapping_sub(StoreLookaheadH41())
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
    } else {
        position
    };
    let random_heuristics_window_size = LiteralSpreeLengthForSparseSearch(params);
    let mut apply_random_heuristics = position
        .wrapping_add(random_heuristics_window_size);
    let gap = 0 as libc::c_int as size_t;
    let kMinScore = ((30 as libc::c_int * 8 as libc::c_int) as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<size_t>() as libc::c_ulong)
        .wrapping_add(100 as libc::c_int as libc::c_ulong);
    PrepareDistanceCacheH41(privat, dist_cache);
    while position.wrapping_add(HashTypeLengthH41()) < pos_end {
        let mut max_length = pos_end.wrapping_sub(position);
        let mut max_distance = brotli_min_size_t(position, max_backward_limit);
        let mut dictionary_start = brotli_min_size_t(
            position.wrapping_add(position_offset),
            max_backward_limit,
        );
        let mut sr = HasherSearchResult {
            len: 0,
            distance: 0,
            score: 0,
            len_code_delta: 0,
        };
        sr.len = 0 as libc::c_int as size_t;
        sr.len_code_delta = 0 as libc::c_int;
        sr.distance = 0 as libc::c_int as size_t;
        sr.score = kMinScore;
        FindLongestMatchH41(
            privat,
            &(*params).dictionary,
            ringbuffer,
            ringbuffer_mask,
            dist_cache,
            position,
            max_length,
            max_distance,
            dictionary_start.wrapping_add(gap),
            (*params).dist.max_distance,
            &mut sr,
        );
        if sr.score > kMinScore {
            let mut delayed_backward_references_in_row = 0 as libc::c_int;
            max_length = max_length.wrapping_sub(1);
            loop {
                let cost_diff_lazy = 175 as libc::c_int as size_t;
                let mut sr2 = HasherSearchResult {
                    len: 0,
                    distance: 0,
                    score: 0,
                    len_code_delta: 0,
                };
                sr2
                    .len = if (*params).quality < 5 as libc::c_int {
                    brotli_min_size_t(
                        (sr.len).wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        max_length,
                    )
                } else {
                    0 as libc::c_int as libc::c_ulong
                };
                sr2.len_code_delta = 0 as libc::c_int;
                sr2.distance = 0 as libc::c_int as size_t;
                sr2.score = kMinScore;
                max_distance = brotli_min_size_t(
                    position.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    max_backward_limit,
                );
                dictionary_start = brotli_min_size_t(
                    position
                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(position_offset),
                    max_backward_limit,
                );
                FindLongestMatchH41(
                    privat,
                    &(*params).dictionary,
                    ringbuffer,
                    ringbuffer_mask,
                    dist_cache,
                    position.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    max_length,
                    max_distance,
                    dictionary_start.wrapping_add(gap),
                    (*params).dist.max_distance,
                    &mut sr2,
                );
                if !(sr2.score >= (sr.score).wrapping_add(cost_diff_lazy)) {
                    break;
                }
                position = position.wrapping_add(1);
                insert_length = insert_length.wrapping_add(1);
                sr = sr2;
                delayed_backward_references_in_row += 1;
                if !(delayed_backward_references_in_row < 4 as libc::c_int
                    && position.wrapping_add(HashTypeLengthH41()) < pos_end)
                {
                    break;
                }
                max_length = max_length.wrapping_sub(1);
            }
            apply_random_heuristics = position
                .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(sr.len))
                .wrapping_add(random_heuristics_window_size);
            dictionary_start = brotli_min_size_t(
                position.wrapping_add(position_offset),
                max_backward_limit,
            );
            let mut distance_code = ComputeDistanceCode(
                sr.distance,
                dictionary_start.wrapping_add(gap),
                dist_cache,
            );
            if sr.distance <= dictionary_start.wrapping_add(gap)
                && distance_code > 0 as libc::c_int as libc::c_ulong
            {
                *dist_cache
                    .offset(
                        3 as libc::c_int as isize,
                    ) = *dist_cache.offset(2 as libc::c_int as isize);
                *dist_cache
                    .offset(
                        2 as libc::c_int as isize,
                    ) = *dist_cache.offset(1 as libc::c_int as isize);
                *dist_cache
                    .offset(
                        1 as libc::c_int as isize,
                    ) = *dist_cache.offset(0 as libc::c_int as isize);
                *dist_cache
                    .offset(0 as libc::c_int as isize) = sr.distance as libc::c_int;
                PrepareDistanceCacheH41(privat, dist_cache);
            }
            let fresh18 = commands;
            commands = commands.offset(1);
            InitCommand(
                fresh18,
                &(*params).dist,
                insert_length,
                sr.len,
                sr.len_code_delta,
                distance_code,
            );
            *num_literals = (*num_literals as libc::c_ulong).wrapping_add(insert_length)
                as size_t as size_t;
            insert_length = 0 as libc::c_int as size_t;
            let mut range_start = position
                .wrapping_add(2 as libc::c_int as libc::c_ulong);
            let mut range_end = brotli_min_size_t(
                position.wrapping_add(sr.len),
                store_end,
            );
            if sr.distance < sr.len >> 2 as libc::c_int {
                range_start = brotli_min_size_t(
                    range_end,
                    brotli_max_size_t(
                        range_start,
                        position
                            .wrapping_add(sr.len)
                            .wrapping_sub(sr.distance << 2 as libc::c_int),
                    ),
                );
            }
            StoreRangeH41(privat, ringbuffer, ringbuffer_mask, range_start, range_end);
            position = (position as libc::c_ulong).wrapping_add(sr.len) as size_t
                as size_t;
        } else {
            insert_length = insert_length.wrapping_add(1);
            position = position.wrapping_add(1);
            if position > apply_random_heuristics {
                if position
                    > apply_random_heuristics
                        .wrapping_add(
                            (4 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(random_heuristics_window_size),
                        )
                {
                    let kMargin = brotli_max_size_t(
                        (StoreLookaheadH41())
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        4 as libc::c_int as size_t,
                    );
                    let mut pos_jump = brotli_min_size_t(
                        position.wrapping_add(16 as libc::c_int as libc::c_ulong),
                        pos_end.wrapping_sub(kMargin),
                    );
                    while position < pos_jump {
                        StoreH41(privat, ringbuffer, ringbuffer_mask, position);
                        insert_length = (insert_length as libc::c_ulong)
                            .wrapping_add(4 as libc::c_int as libc::c_ulong) as size_t
                            as size_t;
                        position = (position as libc::c_ulong)
                            .wrapping_add(4 as libc::c_int as libc::c_ulong) as size_t
                            as size_t;
                    }
                } else {
                    let kMargin_0 = brotli_max_size_t(
                        (StoreLookaheadH41())
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        2 as libc::c_int as size_t,
                    );
                    let mut pos_jump_0 = brotli_min_size_t(
                        position.wrapping_add(8 as libc::c_int as libc::c_ulong),
                        pos_end.wrapping_sub(kMargin_0),
                    );
                    while position < pos_jump_0 {
                        StoreH41(privat, ringbuffer, ringbuffer_mask, position);
                        insert_length = (insert_length as libc::c_ulong)
                            .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t
                            as size_t;
                        position = (position as libc::c_ulong)
                            .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t
                            as size_t;
                    }
                }
            }
        }
    }
    insert_length = (insert_length as libc::c_ulong)
        .wrapping_add(pos_end.wrapping_sub(position)) as size_t as size_t;
    *last_insert_len = insert_length;
    *num_commands = (*num_commands as libc::c_ulong)
        .wrapping_add(commands.offset_from(orig_commands) as libc::c_long as size_t)
        as size_t as size_t;
}
#[inline(never)]
unsafe extern "C" fn CreateBackwardReferencesNH40(
    mut num_bytes: size_t,
    mut position: size_t,
    mut ringbuffer: *const uint8_t,
    mut ringbuffer_mask: size_t,
    mut literal_context_lut: ContextLut,
    mut params: *const BrotliEncoderParams,
    mut hasher: *mut Hasher,
    mut dist_cache: *mut libc::c_int,
    mut last_insert_len: *mut size_t,
    mut commands: *mut Command,
    mut num_commands: *mut size_t,
    mut num_literals: *mut size_t,
) {
    let mut privat: *mut H40 = &mut (*hasher).privat._H40;
    let max_backward_limit = ((1 as libc::c_int as size_t) << (*params).lgwin)
        .wrapping_sub(16 as libc::c_int as libc::c_ulong);
    let position_offset = (*params).stream_offset;
    let orig_commands: *const Command = commands;
    let mut insert_length = *last_insert_len;
    let pos_end = position.wrapping_add(num_bytes);
    let store_end = if num_bytes >= StoreLookaheadH40() {
        position
            .wrapping_add(num_bytes)
            .wrapping_sub(StoreLookaheadH40())
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
    } else {
        position
    };
    let random_heuristics_window_size = LiteralSpreeLengthForSparseSearch(params);
    let mut apply_random_heuristics = position
        .wrapping_add(random_heuristics_window_size);
    let gap = 0 as libc::c_int as size_t;
    let kMinScore = ((30 as libc::c_int * 8 as libc::c_int) as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<size_t>() as libc::c_ulong)
        .wrapping_add(100 as libc::c_int as libc::c_ulong);
    PrepareDistanceCacheH40(privat, dist_cache);
    while position.wrapping_add(HashTypeLengthH40()) < pos_end {
        let mut max_length = pos_end.wrapping_sub(position);
        let mut max_distance = brotli_min_size_t(position, max_backward_limit);
        let mut dictionary_start = brotli_min_size_t(
            position.wrapping_add(position_offset),
            max_backward_limit,
        );
        let mut sr = HasherSearchResult {
            len: 0,
            distance: 0,
            score: 0,
            len_code_delta: 0,
        };
        sr.len = 0 as libc::c_int as size_t;
        sr.len_code_delta = 0 as libc::c_int;
        sr.distance = 0 as libc::c_int as size_t;
        sr.score = kMinScore;
        FindLongestMatchH40(
            privat,
            &(*params).dictionary,
            ringbuffer,
            ringbuffer_mask,
            dist_cache,
            position,
            max_length,
            max_distance,
            dictionary_start.wrapping_add(gap),
            (*params).dist.max_distance,
            &mut sr,
        );
        if sr.score > kMinScore {
            let mut delayed_backward_references_in_row = 0 as libc::c_int;
            max_length = max_length.wrapping_sub(1);
            loop {
                let cost_diff_lazy = 175 as libc::c_int as size_t;
                let mut sr2 = HasherSearchResult {
                    len: 0,
                    distance: 0,
                    score: 0,
                    len_code_delta: 0,
                };
                sr2
                    .len = if (*params).quality < 5 as libc::c_int {
                    brotli_min_size_t(
                        (sr.len).wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        max_length,
                    )
                } else {
                    0 as libc::c_int as libc::c_ulong
                };
                sr2.len_code_delta = 0 as libc::c_int;
                sr2.distance = 0 as libc::c_int as size_t;
                sr2.score = kMinScore;
                max_distance = brotli_min_size_t(
                    position.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    max_backward_limit,
                );
                dictionary_start = brotli_min_size_t(
                    position
                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(position_offset),
                    max_backward_limit,
                );
                FindLongestMatchH40(
                    privat,
                    &(*params).dictionary,
                    ringbuffer,
                    ringbuffer_mask,
                    dist_cache,
                    position.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    max_length,
                    max_distance,
                    dictionary_start.wrapping_add(gap),
                    (*params).dist.max_distance,
                    &mut sr2,
                );
                if !(sr2.score >= (sr.score).wrapping_add(cost_diff_lazy)) {
                    break;
                }
                position = position.wrapping_add(1);
                insert_length = insert_length.wrapping_add(1);
                sr = sr2;
                delayed_backward_references_in_row += 1;
                if !(delayed_backward_references_in_row < 4 as libc::c_int
                    && position.wrapping_add(HashTypeLengthH40()) < pos_end)
                {
                    break;
                }
                max_length = max_length.wrapping_sub(1);
            }
            apply_random_heuristics = position
                .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(sr.len))
                .wrapping_add(random_heuristics_window_size);
            dictionary_start = brotli_min_size_t(
                position.wrapping_add(position_offset),
                max_backward_limit,
            );
            let mut distance_code = ComputeDistanceCode(
                sr.distance,
                dictionary_start.wrapping_add(gap),
                dist_cache,
            );
            if sr.distance <= dictionary_start.wrapping_add(gap)
                && distance_code > 0 as libc::c_int as libc::c_ulong
            {
                *dist_cache
                    .offset(
                        3 as libc::c_int as isize,
                    ) = *dist_cache.offset(2 as libc::c_int as isize);
                *dist_cache
                    .offset(
                        2 as libc::c_int as isize,
                    ) = *dist_cache.offset(1 as libc::c_int as isize);
                *dist_cache
                    .offset(
                        1 as libc::c_int as isize,
                    ) = *dist_cache.offset(0 as libc::c_int as isize);
                *dist_cache
                    .offset(0 as libc::c_int as isize) = sr.distance as libc::c_int;
                PrepareDistanceCacheH40(privat, dist_cache);
            }
            let fresh19 = commands;
            commands = commands.offset(1);
            InitCommand(
                fresh19,
                &(*params).dist,
                insert_length,
                sr.len,
                sr.len_code_delta,
                distance_code,
            );
            *num_literals = (*num_literals as libc::c_ulong).wrapping_add(insert_length)
                as size_t as size_t;
            insert_length = 0 as libc::c_int as size_t;
            let mut range_start = position
                .wrapping_add(2 as libc::c_int as libc::c_ulong);
            let mut range_end = brotli_min_size_t(
                position.wrapping_add(sr.len),
                store_end,
            );
            if sr.distance < sr.len >> 2 as libc::c_int {
                range_start = brotli_min_size_t(
                    range_end,
                    brotli_max_size_t(
                        range_start,
                        position
                            .wrapping_add(sr.len)
                            .wrapping_sub(sr.distance << 2 as libc::c_int),
                    ),
                );
            }
            StoreRangeH40(privat, ringbuffer, ringbuffer_mask, range_start, range_end);
            position = (position as libc::c_ulong).wrapping_add(sr.len) as size_t
                as size_t;
        } else {
            insert_length = insert_length.wrapping_add(1);
            position = position.wrapping_add(1);
            if position > apply_random_heuristics {
                if position
                    > apply_random_heuristics
                        .wrapping_add(
                            (4 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(random_heuristics_window_size),
                        )
                {
                    let kMargin = brotli_max_size_t(
                        (StoreLookaheadH40())
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        4 as libc::c_int as size_t,
                    );
                    let mut pos_jump = brotli_min_size_t(
                        position.wrapping_add(16 as libc::c_int as libc::c_ulong),
                        pos_end.wrapping_sub(kMargin),
                    );
                    while position < pos_jump {
                        StoreH40(privat, ringbuffer, ringbuffer_mask, position);
                        insert_length = (insert_length as libc::c_ulong)
                            .wrapping_add(4 as libc::c_int as libc::c_ulong) as size_t
                            as size_t;
                        position = (position as libc::c_ulong)
                            .wrapping_add(4 as libc::c_int as libc::c_ulong) as size_t
                            as size_t;
                    }
                } else {
                    let kMargin_0 = brotli_max_size_t(
                        (StoreLookaheadH40())
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        2 as libc::c_int as size_t,
                    );
                    let mut pos_jump_0 = brotli_min_size_t(
                        position.wrapping_add(8 as libc::c_int as libc::c_ulong),
                        pos_end.wrapping_sub(kMargin_0),
                    );
                    while position < pos_jump_0 {
                        StoreH40(privat, ringbuffer, ringbuffer_mask, position);
                        insert_length = (insert_length as libc::c_ulong)
                            .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t
                            as size_t;
                        position = (position as libc::c_ulong)
                            .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t
                            as size_t;
                    }
                }
            }
        }
    }
    insert_length = (insert_length as libc::c_ulong)
        .wrapping_add(pos_end.wrapping_sub(position)) as size_t as size_t;
    *last_insert_len = insert_length;
    *num_commands = (*num_commands as libc::c_ulong)
        .wrapping_add(commands.offset_from(orig_commands) as libc::c_long as size_t)
        as size_t as size_t;
}
#[inline(never)]
unsafe extern "C" fn CreateBackwardReferencesNH6(
    mut num_bytes: size_t,
    mut position: size_t,
    mut ringbuffer: *const uint8_t,
    mut ringbuffer_mask: size_t,
    mut literal_context_lut: ContextLut,
    mut params: *const BrotliEncoderParams,
    mut hasher: *mut Hasher,
    mut dist_cache: *mut libc::c_int,
    mut last_insert_len: *mut size_t,
    mut commands: *mut Command,
    mut num_commands: *mut size_t,
    mut num_literals: *mut size_t,
) {
    let mut privat: *mut H6 = &mut (*hasher).privat._H6;
    let max_backward_limit = ((1 as libc::c_int as size_t) << (*params).lgwin)
        .wrapping_sub(16 as libc::c_int as libc::c_ulong);
    let position_offset = (*params).stream_offset;
    let orig_commands: *const Command = commands;
    let mut insert_length = *last_insert_len;
    let pos_end = position.wrapping_add(num_bytes);
    let store_end = if num_bytes >= StoreLookaheadH6() {
        position
            .wrapping_add(num_bytes)
            .wrapping_sub(StoreLookaheadH6())
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
    } else {
        position
    };
    let random_heuristics_window_size = LiteralSpreeLengthForSparseSearch(params);
    let mut apply_random_heuristics = position
        .wrapping_add(random_heuristics_window_size);
    let gap = 0 as libc::c_int as size_t;
    let kMinScore = ((30 as libc::c_int * 8 as libc::c_int) as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<size_t>() as libc::c_ulong)
        .wrapping_add(100 as libc::c_int as libc::c_ulong);
    PrepareDistanceCacheH6(privat, dist_cache);
    while position.wrapping_add(HashTypeLengthH6()) < pos_end {
        let mut max_length = pos_end.wrapping_sub(position);
        let mut max_distance = brotli_min_size_t(position, max_backward_limit);
        let mut dictionary_start = brotli_min_size_t(
            position.wrapping_add(position_offset),
            max_backward_limit,
        );
        let mut sr = HasherSearchResult {
            len: 0,
            distance: 0,
            score: 0,
            len_code_delta: 0,
        };
        sr.len = 0 as libc::c_int as size_t;
        sr.len_code_delta = 0 as libc::c_int;
        sr.distance = 0 as libc::c_int as size_t;
        sr.score = kMinScore;
        FindLongestMatchH6(
            privat,
            &(*params).dictionary,
            ringbuffer,
            ringbuffer_mask,
            dist_cache,
            position,
            max_length,
            max_distance,
            dictionary_start.wrapping_add(gap),
            (*params).dist.max_distance,
            &mut sr,
        );
        if sr.score > kMinScore {
            let mut delayed_backward_references_in_row = 0 as libc::c_int;
            max_length = max_length.wrapping_sub(1);
            loop {
                let cost_diff_lazy = 175 as libc::c_int as size_t;
                let mut sr2 = HasherSearchResult {
                    len: 0,
                    distance: 0,
                    score: 0,
                    len_code_delta: 0,
                };
                sr2
                    .len = if (*params).quality < 5 as libc::c_int {
                    brotli_min_size_t(
                        (sr.len).wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        max_length,
                    )
                } else {
                    0 as libc::c_int as libc::c_ulong
                };
                sr2.len_code_delta = 0 as libc::c_int;
                sr2.distance = 0 as libc::c_int as size_t;
                sr2.score = kMinScore;
                max_distance = brotli_min_size_t(
                    position.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    max_backward_limit,
                );
                dictionary_start = brotli_min_size_t(
                    position
                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(position_offset),
                    max_backward_limit,
                );
                FindLongestMatchH6(
                    privat,
                    &(*params).dictionary,
                    ringbuffer,
                    ringbuffer_mask,
                    dist_cache,
                    position.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    max_length,
                    max_distance,
                    dictionary_start.wrapping_add(gap),
                    (*params).dist.max_distance,
                    &mut sr2,
                );
                if !(sr2.score >= (sr.score).wrapping_add(cost_diff_lazy)) {
                    break;
                }
                position = position.wrapping_add(1);
                insert_length = insert_length.wrapping_add(1);
                sr = sr2;
                delayed_backward_references_in_row += 1;
                if !(delayed_backward_references_in_row < 4 as libc::c_int
                    && position.wrapping_add(HashTypeLengthH6()) < pos_end)
                {
                    break;
                }
                max_length = max_length.wrapping_sub(1);
            }
            apply_random_heuristics = position
                .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(sr.len))
                .wrapping_add(random_heuristics_window_size);
            dictionary_start = brotli_min_size_t(
                position.wrapping_add(position_offset),
                max_backward_limit,
            );
            let mut distance_code = ComputeDistanceCode(
                sr.distance,
                dictionary_start.wrapping_add(gap),
                dist_cache,
            );
            if sr.distance <= dictionary_start.wrapping_add(gap)
                && distance_code > 0 as libc::c_int as libc::c_ulong
            {
                *dist_cache
                    .offset(
                        3 as libc::c_int as isize,
                    ) = *dist_cache.offset(2 as libc::c_int as isize);
                *dist_cache
                    .offset(
                        2 as libc::c_int as isize,
                    ) = *dist_cache.offset(1 as libc::c_int as isize);
                *dist_cache
                    .offset(
                        1 as libc::c_int as isize,
                    ) = *dist_cache.offset(0 as libc::c_int as isize);
                *dist_cache
                    .offset(0 as libc::c_int as isize) = sr.distance as libc::c_int;
                PrepareDistanceCacheH6(privat, dist_cache);
            }
            let fresh20 = commands;
            commands = commands.offset(1);
            InitCommand(
                fresh20,
                &(*params).dist,
                insert_length,
                sr.len,
                sr.len_code_delta,
                distance_code,
            );
            *num_literals = (*num_literals as libc::c_ulong).wrapping_add(insert_length)
                as size_t as size_t;
            insert_length = 0 as libc::c_int as size_t;
            let mut range_start = position
                .wrapping_add(2 as libc::c_int as libc::c_ulong);
            let mut range_end = brotli_min_size_t(
                position.wrapping_add(sr.len),
                store_end,
            );
            if sr.distance < sr.len >> 2 as libc::c_int {
                range_start = brotli_min_size_t(
                    range_end,
                    brotli_max_size_t(
                        range_start,
                        position
                            .wrapping_add(sr.len)
                            .wrapping_sub(sr.distance << 2 as libc::c_int),
                    ),
                );
            }
            StoreRangeH6(privat, ringbuffer, ringbuffer_mask, range_start, range_end);
            position = (position as libc::c_ulong).wrapping_add(sr.len) as size_t
                as size_t;
        } else {
            insert_length = insert_length.wrapping_add(1);
            position = position.wrapping_add(1);
            if position > apply_random_heuristics {
                if position
                    > apply_random_heuristics
                        .wrapping_add(
                            (4 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(random_heuristics_window_size),
                        )
                {
                    let kMargin = brotli_max_size_t(
                        (StoreLookaheadH6())
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        4 as libc::c_int as size_t,
                    );
                    let mut pos_jump = brotli_min_size_t(
                        position.wrapping_add(16 as libc::c_int as libc::c_ulong),
                        pos_end.wrapping_sub(kMargin),
                    );
                    while position < pos_jump {
                        StoreH6(privat, ringbuffer, ringbuffer_mask, position);
                        insert_length = (insert_length as libc::c_ulong)
                            .wrapping_add(4 as libc::c_int as libc::c_ulong) as size_t
                            as size_t;
                        position = (position as libc::c_ulong)
                            .wrapping_add(4 as libc::c_int as libc::c_ulong) as size_t
                            as size_t;
                    }
                } else {
                    let kMargin_0 = brotli_max_size_t(
                        (StoreLookaheadH6())
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        2 as libc::c_int as size_t,
                    );
                    let mut pos_jump_0 = brotli_min_size_t(
                        position.wrapping_add(8 as libc::c_int as libc::c_ulong),
                        pos_end.wrapping_sub(kMargin_0),
                    );
                    while position < pos_jump_0 {
                        StoreH6(privat, ringbuffer, ringbuffer_mask, position);
                        insert_length = (insert_length as libc::c_ulong)
                            .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t
                            as size_t;
                        position = (position as libc::c_ulong)
                            .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t
                            as size_t;
                    }
                }
            }
        }
    }
    insert_length = (insert_length as libc::c_ulong)
        .wrapping_add(pos_end.wrapping_sub(position)) as size_t as size_t;
    *last_insert_len = insert_length;
    *num_commands = (*num_commands as libc::c_ulong)
        .wrapping_add(commands.offset_from(orig_commands) as libc::c_long as size_t)
        as size_t as size_t;
}
#[inline(never)]
unsafe extern "C" fn CreateBackwardReferencesNH5(
    mut num_bytes: size_t,
    mut position: size_t,
    mut ringbuffer: *const uint8_t,
    mut ringbuffer_mask: size_t,
    mut literal_context_lut: ContextLut,
    mut params: *const BrotliEncoderParams,
    mut hasher: *mut Hasher,
    mut dist_cache: *mut libc::c_int,
    mut last_insert_len: *mut size_t,
    mut commands: *mut Command,
    mut num_commands: *mut size_t,
    mut num_literals: *mut size_t,
) {
    let mut privat: *mut H5 = &mut (*hasher).privat._H5;
    let max_backward_limit = ((1 as libc::c_int as size_t) << (*params).lgwin)
        .wrapping_sub(16 as libc::c_int as libc::c_ulong);
    let position_offset = (*params).stream_offset;
    let orig_commands: *const Command = commands;
    let mut insert_length = *last_insert_len;
    let pos_end = position.wrapping_add(num_bytes);
    let store_end = if num_bytes >= StoreLookaheadH5() {
        position
            .wrapping_add(num_bytes)
            .wrapping_sub(StoreLookaheadH5())
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
    } else {
        position
    };
    let random_heuristics_window_size = LiteralSpreeLengthForSparseSearch(params);
    let mut apply_random_heuristics = position
        .wrapping_add(random_heuristics_window_size);
    let gap = 0 as libc::c_int as size_t;
    let kMinScore = ((30 as libc::c_int * 8 as libc::c_int) as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<size_t>() as libc::c_ulong)
        .wrapping_add(100 as libc::c_int as libc::c_ulong);
    PrepareDistanceCacheH5(privat, dist_cache);
    while position.wrapping_add(HashTypeLengthH5()) < pos_end {
        let mut max_length = pos_end.wrapping_sub(position);
        let mut max_distance = brotli_min_size_t(position, max_backward_limit);
        let mut dictionary_start = brotli_min_size_t(
            position.wrapping_add(position_offset),
            max_backward_limit,
        );
        let mut sr = HasherSearchResult {
            len: 0,
            distance: 0,
            score: 0,
            len_code_delta: 0,
        };
        sr.len = 0 as libc::c_int as size_t;
        sr.len_code_delta = 0 as libc::c_int;
        sr.distance = 0 as libc::c_int as size_t;
        sr.score = kMinScore;
        FindLongestMatchH5(
            privat,
            &(*params).dictionary,
            ringbuffer,
            ringbuffer_mask,
            dist_cache,
            position,
            max_length,
            max_distance,
            dictionary_start.wrapping_add(gap),
            (*params).dist.max_distance,
            &mut sr,
        );
        if sr.score > kMinScore {
            let mut delayed_backward_references_in_row = 0 as libc::c_int;
            max_length = max_length.wrapping_sub(1);
            loop {
                let cost_diff_lazy = 175 as libc::c_int as size_t;
                let mut sr2 = HasherSearchResult {
                    len: 0,
                    distance: 0,
                    score: 0,
                    len_code_delta: 0,
                };
                sr2
                    .len = if (*params).quality < 5 as libc::c_int {
                    brotli_min_size_t(
                        (sr.len).wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        max_length,
                    )
                } else {
                    0 as libc::c_int as libc::c_ulong
                };
                sr2.len_code_delta = 0 as libc::c_int;
                sr2.distance = 0 as libc::c_int as size_t;
                sr2.score = kMinScore;
                max_distance = brotli_min_size_t(
                    position.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    max_backward_limit,
                );
                dictionary_start = brotli_min_size_t(
                    position
                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(position_offset),
                    max_backward_limit,
                );
                FindLongestMatchH5(
                    privat,
                    &(*params).dictionary,
                    ringbuffer,
                    ringbuffer_mask,
                    dist_cache,
                    position.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    max_length,
                    max_distance,
                    dictionary_start.wrapping_add(gap),
                    (*params).dist.max_distance,
                    &mut sr2,
                );
                if !(sr2.score >= (sr.score).wrapping_add(cost_diff_lazy)) {
                    break;
                }
                position = position.wrapping_add(1);
                insert_length = insert_length.wrapping_add(1);
                sr = sr2;
                delayed_backward_references_in_row += 1;
                if !(delayed_backward_references_in_row < 4 as libc::c_int
                    && position.wrapping_add(HashTypeLengthH5()) < pos_end)
                {
                    break;
                }
                max_length = max_length.wrapping_sub(1);
            }
            apply_random_heuristics = position
                .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(sr.len))
                .wrapping_add(random_heuristics_window_size);
            dictionary_start = brotli_min_size_t(
                position.wrapping_add(position_offset),
                max_backward_limit,
            );
            let mut distance_code = ComputeDistanceCode(
                sr.distance,
                dictionary_start.wrapping_add(gap),
                dist_cache,
            );
            if sr.distance <= dictionary_start.wrapping_add(gap)
                && distance_code > 0 as libc::c_int as libc::c_ulong
            {
                *dist_cache
                    .offset(
                        3 as libc::c_int as isize,
                    ) = *dist_cache.offset(2 as libc::c_int as isize);
                *dist_cache
                    .offset(
                        2 as libc::c_int as isize,
                    ) = *dist_cache.offset(1 as libc::c_int as isize);
                *dist_cache
                    .offset(
                        1 as libc::c_int as isize,
                    ) = *dist_cache.offset(0 as libc::c_int as isize);
                *dist_cache
                    .offset(0 as libc::c_int as isize) = sr.distance as libc::c_int;
                PrepareDistanceCacheH5(privat, dist_cache);
            }
            let fresh21 = commands;
            commands = commands.offset(1);
            InitCommand(
                fresh21,
                &(*params).dist,
                insert_length,
                sr.len,
                sr.len_code_delta,
                distance_code,
            );
            *num_literals = (*num_literals as libc::c_ulong).wrapping_add(insert_length)
                as size_t as size_t;
            insert_length = 0 as libc::c_int as size_t;
            let mut range_start = position
                .wrapping_add(2 as libc::c_int as libc::c_ulong);
            let mut range_end = brotli_min_size_t(
                position.wrapping_add(sr.len),
                store_end,
            );
            if sr.distance < sr.len >> 2 as libc::c_int {
                range_start = brotli_min_size_t(
                    range_end,
                    brotli_max_size_t(
                        range_start,
                        position
                            .wrapping_add(sr.len)
                            .wrapping_sub(sr.distance << 2 as libc::c_int),
                    ),
                );
            }
            StoreRangeH5(privat, ringbuffer, ringbuffer_mask, range_start, range_end);
            position = (position as libc::c_ulong).wrapping_add(sr.len) as size_t
                as size_t;
        } else {
            insert_length = insert_length.wrapping_add(1);
            position = position.wrapping_add(1);
            if position > apply_random_heuristics {
                if position
                    > apply_random_heuristics
                        .wrapping_add(
                            (4 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(random_heuristics_window_size),
                        )
                {
                    let kMargin = brotli_max_size_t(
                        (StoreLookaheadH5())
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        4 as libc::c_int as size_t,
                    );
                    let mut pos_jump = brotli_min_size_t(
                        position.wrapping_add(16 as libc::c_int as libc::c_ulong),
                        pos_end.wrapping_sub(kMargin),
                    );
                    while position < pos_jump {
                        StoreH5(privat, ringbuffer, ringbuffer_mask, position);
                        insert_length = (insert_length as libc::c_ulong)
                            .wrapping_add(4 as libc::c_int as libc::c_ulong) as size_t
                            as size_t;
                        position = (position as libc::c_ulong)
                            .wrapping_add(4 as libc::c_int as libc::c_ulong) as size_t
                            as size_t;
                    }
                } else {
                    let kMargin_0 = brotli_max_size_t(
                        (StoreLookaheadH5())
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        2 as libc::c_int as size_t,
                    );
                    let mut pos_jump_0 = brotli_min_size_t(
                        position.wrapping_add(8 as libc::c_int as libc::c_ulong),
                        pos_end.wrapping_sub(kMargin_0),
                    );
                    while position < pos_jump_0 {
                        StoreH5(privat, ringbuffer, ringbuffer_mask, position);
                        insert_length = (insert_length as libc::c_ulong)
                            .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t
                            as size_t;
                        position = (position as libc::c_ulong)
                            .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t
                            as size_t;
                    }
                }
            }
        }
    }
    insert_length = (insert_length as libc::c_ulong)
        .wrapping_add(pos_end.wrapping_sub(position)) as size_t as size_t;
    *last_insert_len = insert_length;
    *num_commands = (*num_commands as libc::c_ulong)
        .wrapping_add(commands.offset_from(orig_commands) as libc::c_long as size_t)
        as size_t as size_t;
}
#[inline(never)]
unsafe extern "C" fn CreateBackwardReferencesNH4(
    mut num_bytes: size_t,
    mut position: size_t,
    mut ringbuffer: *const uint8_t,
    mut ringbuffer_mask: size_t,
    mut literal_context_lut: ContextLut,
    mut params: *const BrotliEncoderParams,
    mut hasher: *mut Hasher,
    mut dist_cache: *mut libc::c_int,
    mut last_insert_len: *mut size_t,
    mut commands: *mut Command,
    mut num_commands: *mut size_t,
    mut num_literals: *mut size_t,
) {
    let mut privat: *mut H4 = &mut (*hasher).privat._H4;
    let max_backward_limit = ((1 as libc::c_int as size_t) << (*params).lgwin)
        .wrapping_sub(16 as libc::c_int as libc::c_ulong);
    let position_offset = (*params).stream_offset;
    let orig_commands: *const Command = commands;
    let mut insert_length = *last_insert_len;
    let pos_end = position.wrapping_add(num_bytes);
    let store_end = if num_bytes >= StoreLookaheadH4() {
        position
            .wrapping_add(num_bytes)
            .wrapping_sub(StoreLookaheadH4())
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
    } else {
        position
    };
    let random_heuristics_window_size = LiteralSpreeLengthForSparseSearch(params);
    let mut apply_random_heuristics = position
        .wrapping_add(random_heuristics_window_size);
    let gap = 0 as libc::c_int as size_t;
    let kMinScore = ((30 as libc::c_int * 8 as libc::c_int) as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<size_t>() as libc::c_ulong)
        .wrapping_add(100 as libc::c_int as libc::c_ulong);
    PrepareDistanceCacheH4(privat, dist_cache);
    while position.wrapping_add(HashTypeLengthH4()) < pos_end {
        let mut max_length = pos_end.wrapping_sub(position);
        let mut max_distance = brotli_min_size_t(position, max_backward_limit);
        let mut dictionary_start = brotli_min_size_t(
            position.wrapping_add(position_offset),
            max_backward_limit,
        );
        let mut sr = HasherSearchResult {
            len: 0,
            distance: 0,
            score: 0,
            len_code_delta: 0,
        };
        sr.len = 0 as libc::c_int as size_t;
        sr.len_code_delta = 0 as libc::c_int;
        sr.distance = 0 as libc::c_int as size_t;
        sr.score = kMinScore;
        FindLongestMatchH4(
            privat,
            &(*params).dictionary,
            ringbuffer,
            ringbuffer_mask,
            dist_cache,
            position,
            max_length,
            max_distance,
            dictionary_start.wrapping_add(gap),
            (*params).dist.max_distance,
            &mut sr,
        );
        if sr.score > kMinScore {
            let mut delayed_backward_references_in_row = 0 as libc::c_int;
            max_length = max_length.wrapping_sub(1);
            loop {
                let cost_diff_lazy = 175 as libc::c_int as size_t;
                let mut sr2 = HasherSearchResult {
                    len: 0,
                    distance: 0,
                    score: 0,
                    len_code_delta: 0,
                };
                sr2
                    .len = if (*params).quality < 5 as libc::c_int {
                    brotli_min_size_t(
                        (sr.len).wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        max_length,
                    )
                } else {
                    0 as libc::c_int as libc::c_ulong
                };
                sr2.len_code_delta = 0 as libc::c_int;
                sr2.distance = 0 as libc::c_int as size_t;
                sr2.score = kMinScore;
                max_distance = brotli_min_size_t(
                    position.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    max_backward_limit,
                );
                dictionary_start = brotli_min_size_t(
                    position
                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(position_offset),
                    max_backward_limit,
                );
                FindLongestMatchH4(
                    privat,
                    &(*params).dictionary,
                    ringbuffer,
                    ringbuffer_mask,
                    dist_cache,
                    position.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    max_length,
                    max_distance,
                    dictionary_start.wrapping_add(gap),
                    (*params).dist.max_distance,
                    &mut sr2,
                );
                if !(sr2.score >= (sr.score).wrapping_add(cost_diff_lazy)) {
                    break;
                }
                position = position.wrapping_add(1);
                insert_length = insert_length.wrapping_add(1);
                sr = sr2;
                delayed_backward_references_in_row += 1;
                if !(delayed_backward_references_in_row < 4 as libc::c_int
                    && position.wrapping_add(HashTypeLengthH4()) < pos_end)
                {
                    break;
                }
                max_length = max_length.wrapping_sub(1);
            }
            apply_random_heuristics = position
                .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(sr.len))
                .wrapping_add(random_heuristics_window_size);
            dictionary_start = brotli_min_size_t(
                position.wrapping_add(position_offset),
                max_backward_limit,
            );
            let mut distance_code = ComputeDistanceCode(
                sr.distance,
                dictionary_start.wrapping_add(gap),
                dist_cache,
            );
            if sr.distance <= dictionary_start.wrapping_add(gap)
                && distance_code > 0 as libc::c_int as libc::c_ulong
            {
                *dist_cache
                    .offset(
                        3 as libc::c_int as isize,
                    ) = *dist_cache.offset(2 as libc::c_int as isize);
                *dist_cache
                    .offset(
                        2 as libc::c_int as isize,
                    ) = *dist_cache.offset(1 as libc::c_int as isize);
                *dist_cache
                    .offset(
                        1 as libc::c_int as isize,
                    ) = *dist_cache.offset(0 as libc::c_int as isize);
                *dist_cache
                    .offset(0 as libc::c_int as isize) = sr.distance as libc::c_int;
                PrepareDistanceCacheH4(privat, dist_cache);
            }
            let fresh22 = commands;
            commands = commands.offset(1);
            InitCommand(
                fresh22,
                &(*params).dist,
                insert_length,
                sr.len,
                sr.len_code_delta,
                distance_code,
            );
            *num_literals = (*num_literals as libc::c_ulong).wrapping_add(insert_length)
                as size_t as size_t;
            insert_length = 0 as libc::c_int as size_t;
            let mut range_start = position
                .wrapping_add(2 as libc::c_int as libc::c_ulong);
            let mut range_end = brotli_min_size_t(
                position.wrapping_add(sr.len),
                store_end,
            );
            if sr.distance < sr.len >> 2 as libc::c_int {
                range_start = brotli_min_size_t(
                    range_end,
                    brotli_max_size_t(
                        range_start,
                        position
                            .wrapping_add(sr.len)
                            .wrapping_sub(sr.distance << 2 as libc::c_int),
                    ),
                );
            }
            StoreRangeH4(privat, ringbuffer, ringbuffer_mask, range_start, range_end);
            position = (position as libc::c_ulong).wrapping_add(sr.len) as size_t
                as size_t;
        } else {
            insert_length = insert_length.wrapping_add(1);
            position = position.wrapping_add(1);
            if position > apply_random_heuristics {
                if position
                    > apply_random_heuristics
                        .wrapping_add(
                            (4 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(random_heuristics_window_size),
                        )
                {
                    let kMargin = brotli_max_size_t(
                        (StoreLookaheadH4())
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        4 as libc::c_int as size_t,
                    );
                    let mut pos_jump = brotli_min_size_t(
                        position.wrapping_add(16 as libc::c_int as libc::c_ulong),
                        pos_end.wrapping_sub(kMargin),
                    );
                    while position < pos_jump {
                        StoreH4(privat, ringbuffer, ringbuffer_mask, position);
                        insert_length = (insert_length as libc::c_ulong)
                            .wrapping_add(4 as libc::c_int as libc::c_ulong) as size_t
                            as size_t;
                        position = (position as libc::c_ulong)
                            .wrapping_add(4 as libc::c_int as libc::c_ulong) as size_t
                            as size_t;
                    }
                } else {
                    let kMargin_0 = brotli_max_size_t(
                        (StoreLookaheadH4())
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        2 as libc::c_int as size_t,
                    );
                    let mut pos_jump_0 = brotli_min_size_t(
                        position.wrapping_add(8 as libc::c_int as libc::c_ulong),
                        pos_end.wrapping_sub(kMargin_0),
                    );
                    while position < pos_jump_0 {
                        StoreH4(privat, ringbuffer, ringbuffer_mask, position);
                        insert_length = (insert_length as libc::c_ulong)
                            .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t
                            as size_t;
                        position = (position as libc::c_ulong)
                            .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t
                            as size_t;
                    }
                }
            }
        }
    }
    insert_length = (insert_length as libc::c_ulong)
        .wrapping_add(pos_end.wrapping_sub(position)) as size_t as size_t;
    *last_insert_len = insert_length;
    *num_commands = (*num_commands as libc::c_ulong)
        .wrapping_add(commands.offset_from(orig_commands) as libc::c_long as size_t)
        as size_t as size_t;
}
#[inline(never)]
unsafe extern "C" fn CreateBackwardReferencesNH3(
    mut num_bytes: size_t,
    mut position: size_t,
    mut ringbuffer: *const uint8_t,
    mut ringbuffer_mask: size_t,
    mut literal_context_lut: ContextLut,
    mut params: *const BrotliEncoderParams,
    mut hasher: *mut Hasher,
    mut dist_cache: *mut libc::c_int,
    mut last_insert_len: *mut size_t,
    mut commands: *mut Command,
    mut num_commands: *mut size_t,
    mut num_literals: *mut size_t,
) {
    let mut privat: *mut H3 = &mut (*hasher).privat._H3;
    let max_backward_limit = ((1 as libc::c_int as size_t) << (*params).lgwin)
        .wrapping_sub(16 as libc::c_int as libc::c_ulong);
    let position_offset = (*params).stream_offset;
    let orig_commands: *const Command = commands;
    let mut insert_length = *last_insert_len;
    let pos_end = position.wrapping_add(num_bytes);
    let store_end = if num_bytes >= StoreLookaheadH3() {
        position
            .wrapping_add(num_bytes)
            .wrapping_sub(StoreLookaheadH3())
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
    } else {
        position
    };
    let random_heuristics_window_size = LiteralSpreeLengthForSparseSearch(params);
    let mut apply_random_heuristics = position
        .wrapping_add(random_heuristics_window_size);
    let gap = 0 as libc::c_int as size_t;
    let kMinScore = ((30 as libc::c_int * 8 as libc::c_int) as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<size_t>() as libc::c_ulong)
        .wrapping_add(100 as libc::c_int as libc::c_ulong);
    PrepareDistanceCacheH3(privat, dist_cache);
    while position.wrapping_add(HashTypeLengthH3()) < pos_end {
        let mut max_length = pos_end.wrapping_sub(position);
        let mut max_distance = brotli_min_size_t(position, max_backward_limit);
        let mut dictionary_start = brotli_min_size_t(
            position.wrapping_add(position_offset),
            max_backward_limit,
        );
        let mut sr = HasherSearchResult {
            len: 0,
            distance: 0,
            score: 0,
            len_code_delta: 0,
        };
        sr.len = 0 as libc::c_int as size_t;
        sr.len_code_delta = 0 as libc::c_int;
        sr.distance = 0 as libc::c_int as size_t;
        sr.score = kMinScore;
        FindLongestMatchH3(
            privat,
            &(*params).dictionary,
            ringbuffer,
            ringbuffer_mask,
            dist_cache,
            position,
            max_length,
            max_distance,
            dictionary_start.wrapping_add(gap),
            (*params).dist.max_distance,
            &mut sr,
        );
        if sr.score > kMinScore {
            let mut delayed_backward_references_in_row = 0 as libc::c_int;
            max_length = max_length.wrapping_sub(1);
            loop {
                let cost_diff_lazy = 175 as libc::c_int as size_t;
                let mut sr2 = HasherSearchResult {
                    len: 0,
                    distance: 0,
                    score: 0,
                    len_code_delta: 0,
                };
                sr2
                    .len = if (*params).quality < 5 as libc::c_int {
                    brotli_min_size_t(
                        (sr.len).wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        max_length,
                    )
                } else {
                    0 as libc::c_int as libc::c_ulong
                };
                sr2.len_code_delta = 0 as libc::c_int;
                sr2.distance = 0 as libc::c_int as size_t;
                sr2.score = kMinScore;
                max_distance = brotli_min_size_t(
                    position.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    max_backward_limit,
                );
                dictionary_start = brotli_min_size_t(
                    position
                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(position_offset),
                    max_backward_limit,
                );
                FindLongestMatchH3(
                    privat,
                    &(*params).dictionary,
                    ringbuffer,
                    ringbuffer_mask,
                    dist_cache,
                    position.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    max_length,
                    max_distance,
                    dictionary_start.wrapping_add(gap),
                    (*params).dist.max_distance,
                    &mut sr2,
                );
                if !(sr2.score >= (sr.score).wrapping_add(cost_diff_lazy)) {
                    break;
                }
                position = position.wrapping_add(1);
                insert_length = insert_length.wrapping_add(1);
                sr = sr2;
                delayed_backward_references_in_row += 1;
                if !(delayed_backward_references_in_row < 4 as libc::c_int
                    && position.wrapping_add(HashTypeLengthH3()) < pos_end)
                {
                    break;
                }
                max_length = max_length.wrapping_sub(1);
            }
            apply_random_heuristics = position
                .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(sr.len))
                .wrapping_add(random_heuristics_window_size);
            dictionary_start = brotli_min_size_t(
                position.wrapping_add(position_offset),
                max_backward_limit,
            );
            let mut distance_code = ComputeDistanceCode(
                sr.distance,
                dictionary_start.wrapping_add(gap),
                dist_cache,
            );
            if sr.distance <= dictionary_start.wrapping_add(gap)
                && distance_code > 0 as libc::c_int as libc::c_ulong
            {
                *dist_cache
                    .offset(
                        3 as libc::c_int as isize,
                    ) = *dist_cache.offset(2 as libc::c_int as isize);
                *dist_cache
                    .offset(
                        2 as libc::c_int as isize,
                    ) = *dist_cache.offset(1 as libc::c_int as isize);
                *dist_cache
                    .offset(
                        1 as libc::c_int as isize,
                    ) = *dist_cache.offset(0 as libc::c_int as isize);
                *dist_cache
                    .offset(0 as libc::c_int as isize) = sr.distance as libc::c_int;
                PrepareDistanceCacheH3(privat, dist_cache);
            }
            let fresh23 = commands;
            commands = commands.offset(1);
            InitCommand(
                fresh23,
                &(*params).dist,
                insert_length,
                sr.len,
                sr.len_code_delta,
                distance_code,
            );
            *num_literals = (*num_literals as libc::c_ulong).wrapping_add(insert_length)
                as size_t as size_t;
            insert_length = 0 as libc::c_int as size_t;
            let mut range_start = position
                .wrapping_add(2 as libc::c_int as libc::c_ulong);
            let mut range_end = brotli_min_size_t(
                position.wrapping_add(sr.len),
                store_end,
            );
            if sr.distance < sr.len >> 2 as libc::c_int {
                range_start = brotli_min_size_t(
                    range_end,
                    brotli_max_size_t(
                        range_start,
                        position
                            .wrapping_add(sr.len)
                            .wrapping_sub(sr.distance << 2 as libc::c_int),
                    ),
                );
            }
            StoreRangeH3(privat, ringbuffer, ringbuffer_mask, range_start, range_end);
            position = (position as libc::c_ulong).wrapping_add(sr.len) as size_t
                as size_t;
        } else {
            insert_length = insert_length.wrapping_add(1);
            position = position.wrapping_add(1);
            if position > apply_random_heuristics {
                if position
                    > apply_random_heuristics
                        .wrapping_add(
                            (4 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(random_heuristics_window_size),
                        )
                {
                    let kMargin = brotli_max_size_t(
                        (StoreLookaheadH3())
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        4 as libc::c_int as size_t,
                    );
                    let mut pos_jump = brotli_min_size_t(
                        position.wrapping_add(16 as libc::c_int as libc::c_ulong),
                        pos_end.wrapping_sub(kMargin),
                    );
                    while position < pos_jump {
                        StoreH3(privat, ringbuffer, ringbuffer_mask, position);
                        insert_length = (insert_length as libc::c_ulong)
                            .wrapping_add(4 as libc::c_int as libc::c_ulong) as size_t
                            as size_t;
                        position = (position as libc::c_ulong)
                            .wrapping_add(4 as libc::c_int as libc::c_ulong) as size_t
                            as size_t;
                    }
                } else {
                    let kMargin_0 = brotli_max_size_t(
                        (StoreLookaheadH3())
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        2 as libc::c_int as size_t,
                    );
                    let mut pos_jump_0 = brotli_min_size_t(
                        position.wrapping_add(8 as libc::c_int as libc::c_ulong),
                        pos_end.wrapping_sub(kMargin_0),
                    );
                    while position < pos_jump_0 {
                        StoreH3(privat, ringbuffer, ringbuffer_mask, position);
                        insert_length = (insert_length as libc::c_ulong)
                            .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t
                            as size_t;
                        position = (position as libc::c_ulong)
                            .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t
                            as size_t;
                    }
                }
            }
        }
    }
    insert_length = (insert_length as libc::c_ulong)
        .wrapping_add(pos_end.wrapping_sub(position)) as size_t as size_t;
    *last_insert_len = insert_length;
    *num_commands = (*num_commands as libc::c_ulong)
        .wrapping_add(commands.offset_from(orig_commands) as libc::c_long as size_t)
        as size_t as size_t;
}
#[inline(never)]
unsafe extern "C" fn CreateBackwardReferencesNH2(
    mut num_bytes: size_t,
    mut position: size_t,
    mut ringbuffer: *const uint8_t,
    mut ringbuffer_mask: size_t,
    mut literal_context_lut: ContextLut,
    mut params: *const BrotliEncoderParams,
    mut hasher: *mut Hasher,
    mut dist_cache: *mut libc::c_int,
    mut last_insert_len: *mut size_t,
    mut commands: *mut Command,
    mut num_commands: *mut size_t,
    mut num_literals: *mut size_t,
) {
    let mut privat: *mut H2 = &mut (*hasher).privat._H2;
    let max_backward_limit = ((1 as libc::c_int as size_t) << (*params).lgwin)
        .wrapping_sub(16 as libc::c_int as libc::c_ulong);
    let position_offset = (*params).stream_offset;
    let orig_commands: *const Command = commands;
    let mut insert_length = *last_insert_len;
    let pos_end = position.wrapping_add(num_bytes);
    let store_end = if num_bytes >= StoreLookaheadH2() {
        position
            .wrapping_add(num_bytes)
            .wrapping_sub(StoreLookaheadH2())
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
    } else {
        position
    };
    let random_heuristics_window_size = LiteralSpreeLengthForSparseSearch(params);
    let mut apply_random_heuristics = position
        .wrapping_add(random_heuristics_window_size);
    let gap = 0 as libc::c_int as size_t;
    let kMinScore = ((30 as libc::c_int * 8 as libc::c_int) as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<size_t>() as libc::c_ulong)
        .wrapping_add(100 as libc::c_int as libc::c_ulong);
    PrepareDistanceCacheH2(privat, dist_cache);
    while position.wrapping_add(HashTypeLengthH2()) < pos_end {
        let mut max_length = pos_end.wrapping_sub(position);
        let mut max_distance = brotli_min_size_t(position, max_backward_limit);
        let mut dictionary_start = brotli_min_size_t(
            position.wrapping_add(position_offset),
            max_backward_limit,
        );
        let mut sr = HasherSearchResult {
            len: 0,
            distance: 0,
            score: 0,
            len_code_delta: 0,
        };
        sr.len = 0 as libc::c_int as size_t;
        sr.len_code_delta = 0 as libc::c_int;
        sr.distance = 0 as libc::c_int as size_t;
        sr.score = kMinScore;
        FindLongestMatchH2(
            privat,
            &(*params).dictionary,
            ringbuffer,
            ringbuffer_mask,
            dist_cache,
            position,
            max_length,
            max_distance,
            dictionary_start.wrapping_add(gap),
            (*params).dist.max_distance,
            &mut sr,
        );
        if sr.score > kMinScore {
            let mut delayed_backward_references_in_row = 0 as libc::c_int;
            max_length = max_length.wrapping_sub(1);
            loop {
                let cost_diff_lazy = 175 as libc::c_int as size_t;
                let mut sr2 = HasherSearchResult {
                    len: 0,
                    distance: 0,
                    score: 0,
                    len_code_delta: 0,
                };
                sr2
                    .len = if (*params).quality < 5 as libc::c_int {
                    brotli_min_size_t(
                        (sr.len).wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        max_length,
                    )
                } else {
                    0 as libc::c_int as libc::c_ulong
                };
                sr2.len_code_delta = 0 as libc::c_int;
                sr2.distance = 0 as libc::c_int as size_t;
                sr2.score = kMinScore;
                max_distance = brotli_min_size_t(
                    position.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    max_backward_limit,
                );
                dictionary_start = brotli_min_size_t(
                    position
                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(position_offset),
                    max_backward_limit,
                );
                FindLongestMatchH2(
                    privat,
                    &(*params).dictionary,
                    ringbuffer,
                    ringbuffer_mask,
                    dist_cache,
                    position.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    max_length,
                    max_distance,
                    dictionary_start.wrapping_add(gap),
                    (*params).dist.max_distance,
                    &mut sr2,
                );
                if !(sr2.score >= (sr.score).wrapping_add(cost_diff_lazy)) {
                    break;
                }
                position = position.wrapping_add(1);
                insert_length = insert_length.wrapping_add(1);
                sr = sr2;
                delayed_backward_references_in_row += 1;
                if !(delayed_backward_references_in_row < 4 as libc::c_int
                    && position.wrapping_add(HashTypeLengthH2()) < pos_end)
                {
                    break;
                }
                max_length = max_length.wrapping_sub(1);
            }
            apply_random_heuristics = position
                .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(sr.len))
                .wrapping_add(random_heuristics_window_size);
            dictionary_start = brotli_min_size_t(
                position.wrapping_add(position_offset),
                max_backward_limit,
            );
            let mut distance_code = ComputeDistanceCode(
                sr.distance,
                dictionary_start.wrapping_add(gap),
                dist_cache,
            );
            if sr.distance <= dictionary_start.wrapping_add(gap)
                && distance_code > 0 as libc::c_int as libc::c_ulong
            {
                *dist_cache
                    .offset(
                        3 as libc::c_int as isize,
                    ) = *dist_cache.offset(2 as libc::c_int as isize);
                *dist_cache
                    .offset(
                        2 as libc::c_int as isize,
                    ) = *dist_cache.offset(1 as libc::c_int as isize);
                *dist_cache
                    .offset(
                        1 as libc::c_int as isize,
                    ) = *dist_cache.offset(0 as libc::c_int as isize);
                *dist_cache
                    .offset(0 as libc::c_int as isize) = sr.distance as libc::c_int;
                PrepareDistanceCacheH2(privat, dist_cache);
            }
            let fresh24 = commands;
            commands = commands.offset(1);
            InitCommand(
                fresh24,
                &(*params).dist,
                insert_length,
                sr.len,
                sr.len_code_delta,
                distance_code,
            );
            *num_literals = (*num_literals as libc::c_ulong).wrapping_add(insert_length)
                as size_t as size_t;
            insert_length = 0 as libc::c_int as size_t;
            let mut range_start = position
                .wrapping_add(2 as libc::c_int as libc::c_ulong);
            let mut range_end = brotli_min_size_t(
                position.wrapping_add(sr.len),
                store_end,
            );
            if sr.distance < sr.len >> 2 as libc::c_int {
                range_start = brotli_min_size_t(
                    range_end,
                    brotli_max_size_t(
                        range_start,
                        position
                            .wrapping_add(sr.len)
                            .wrapping_sub(sr.distance << 2 as libc::c_int),
                    ),
                );
            }
            StoreRangeH2(privat, ringbuffer, ringbuffer_mask, range_start, range_end);
            position = (position as libc::c_ulong).wrapping_add(sr.len) as size_t
                as size_t;
        } else {
            insert_length = insert_length.wrapping_add(1);
            position = position.wrapping_add(1);
            if position > apply_random_heuristics {
                if position
                    > apply_random_heuristics
                        .wrapping_add(
                            (4 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(random_heuristics_window_size),
                        )
                {
                    let kMargin = brotli_max_size_t(
                        (StoreLookaheadH2())
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        4 as libc::c_int as size_t,
                    );
                    let mut pos_jump = brotli_min_size_t(
                        position.wrapping_add(16 as libc::c_int as libc::c_ulong),
                        pos_end.wrapping_sub(kMargin),
                    );
                    while position < pos_jump {
                        StoreH2(privat, ringbuffer, ringbuffer_mask, position);
                        insert_length = (insert_length as libc::c_ulong)
                            .wrapping_add(4 as libc::c_int as libc::c_ulong) as size_t
                            as size_t;
                        position = (position as libc::c_ulong)
                            .wrapping_add(4 as libc::c_int as libc::c_ulong) as size_t
                            as size_t;
                    }
                } else {
                    let kMargin_0 = brotli_max_size_t(
                        (StoreLookaheadH2())
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        2 as libc::c_int as size_t,
                    );
                    let mut pos_jump_0 = brotli_min_size_t(
                        position.wrapping_add(8 as libc::c_int as libc::c_ulong),
                        pos_end.wrapping_sub(kMargin_0),
                    );
                    while position < pos_jump_0 {
                        StoreH2(privat, ringbuffer, ringbuffer_mask, position);
                        insert_length = (insert_length as libc::c_ulong)
                            .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t
                            as size_t;
                        position = (position as libc::c_ulong)
                            .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t
                            as size_t;
                    }
                }
            }
        }
    }
    insert_length = (insert_length as libc::c_ulong)
        .wrapping_add(pos_end.wrapping_sub(position)) as size_t as size_t;
    *last_insert_len = insert_length;
    *num_commands = (*num_commands as libc::c_ulong)
        .wrapping_add(commands.offset_from(orig_commands) as libc::c_long as size_t)
        as size_t as size_t;
}
#[inline(never)]
unsafe extern "C" fn CreateBackwardReferencesNH55(
    mut num_bytes: size_t,
    mut position: size_t,
    mut ringbuffer: *const uint8_t,
    mut ringbuffer_mask: size_t,
    mut literal_context_lut: ContextLut,
    mut params: *const BrotliEncoderParams,
    mut hasher: *mut Hasher,
    mut dist_cache: *mut libc::c_int,
    mut last_insert_len: *mut size_t,
    mut commands: *mut Command,
    mut num_commands: *mut size_t,
    mut num_literals: *mut size_t,
) {
    let mut privat: *mut H55 = &mut (*hasher).privat._H55;
    let max_backward_limit = ((1 as libc::c_int as size_t) << (*params).lgwin)
        .wrapping_sub(16 as libc::c_int as libc::c_ulong);
    let position_offset = (*params).stream_offset;
    let orig_commands: *const Command = commands;
    let mut insert_length = *last_insert_len;
    let pos_end = position.wrapping_add(num_bytes);
    let store_end = if num_bytes >= StoreLookaheadH55() {
        position
            .wrapping_add(num_bytes)
            .wrapping_sub(StoreLookaheadH55())
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
    } else {
        position
    };
    let random_heuristics_window_size = LiteralSpreeLengthForSparseSearch(params);
    let mut apply_random_heuristics = position
        .wrapping_add(random_heuristics_window_size);
    let gap = 0 as libc::c_int as size_t;
    let kMinScore = ((30 as libc::c_int * 8 as libc::c_int) as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<size_t>() as libc::c_ulong)
        .wrapping_add(100 as libc::c_int as libc::c_ulong);
    PrepareDistanceCacheH55(privat, dist_cache);
    while position.wrapping_add(HashTypeLengthH55()) < pos_end {
        let mut max_length = pos_end.wrapping_sub(position);
        let mut max_distance = brotli_min_size_t(position, max_backward_limit);
        let mut dictionary_start = brotli_min_size_t(
            position.wrapping_add(position_offset),
            max_backward_limit,
        );
        let mut sr = HasherSearchResult {
            len: 0,
            distance: 0,
            score: 0,
            len_code_delta: 0,
        };
        sr.len = 0 as libc::c_int as size_t;
        sr.len_code_delta = 0 as libc::c_int;
        sr.distance = 0 as libc::c_int as size_t;
        sr.score = kMinScore;
        FindLongestMatchH55(
            privat,
            &(*params).dictionary,
            ringbuffer,
            ringbuffer_mask,
            dist_cache,
            position,
            max_length,
            max_distance,
            dictionary_start.wrapping_add(gap),
            (*params).dist.max_distance,
            &mut sr,
        );
        if sr.score > kMinScore {
            let mut delayed_backward_references_in_row = 0 as libc::c_int;
            max_length = max_length.wrapping_sub(1);
            loop {
                let cost_diff_lazy = 175 as libc::c_int as size_t;
                let mut sr2 = HasherSearchResult {
                    len: 0,
                    distance: 0,
                    score: 0,
                    len_code_delta: 0,
                };
                sr2
                    .len = if (*params).quality < 5 as libc::c_int {
                    brotli_min_size_t(
                        (sr.len).wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        max_length,
                    )
                } else {
                    0 as libc::c_int as libc::c_ulong
                };
                sr2.len_code_delta = 0 as libc::c_int;
                sr2.distance = 0 as libc::c_int as size_t;
                sr2.score = kMinScore;
                max_distance = brotli_min_size_t(
                    position.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    max_backward_limit,
                );
                dictionary_start = brotli_min_size_t(
                    position
                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(position_offset),
                    max_backward_limit,
                );
                FindLongestMatchH55(
                    privat,
                    &(*params).dictionary,
                    ringbuffer,
                    ringbuffer_mask,
                    dist_cache,
                    position.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    max_length,
                    max_distance,
                    dictionary_start.wrapping_add(gap),
                    (*params).dist.max_distance,
                    &mut sr2,
                );
                if !(sr2.score >= (sr.score).wrapping_add(cost_diff_lazy)) {
                    break;
                }
                position = position.wrapping_add(1);
                insert_length = insert_length.wrapping_add(1);
                sr = sr2;
                delayed_backward_references_in_row += 1;
                if !(delayed_backward_references_in_row < 4 as libc::c_int
                    && position.wrapping_add(HashTypeLengthH55()) < pos_end)
                {
                    break;
                }
                max_length = max_length.wrapping_sub(1);
            }
            apply_random_heuristics = position
                .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(sr.len))
                .wrapping_add(random_heuristics_window_size);
            dictionary_start = brotli_min_size_t(
                position.wrapping_add(position_offset),
                max_backward_limit,
            );
            let mut distance_code = ComputeDistanceCode(
                sr.distance,
                dictionary_start.wrapping_add(gap),
                dist_cache,
            );
            if sr.distance <= dictionary_start.wrapping_add(gap)
                && distance_code > 0 as libc::c_int as libc::c_ulong
            {
                *dist_cache
                    .offset(
                        3 as libc::c_int as isize,
                    ) = *dist_cache.offset(2 as libc::c_int as isize);
                *dist_cache
                    .offset(
                        2 as libc::c_int as isize,
                    ) = *dist_cache.offset(1 as libc::c_int as isize);
                *dist_cache
                    .offset(
                        1 as libc::c_int as isize,
                    ) = *dist_cache.offset(0 as libc::c_int as isize);
                *dist_cache
                    .offset(0 as libc::c_int as isize) = sr.distance as libc::c_int;
                PrepareDistanceCacheH55(privat, dist_cache);
            }
            let fresh25 = commands;
            commands = commands.offset(1);
            InitCommand(
                fresh25,
                &(*params).dist,
                insert_length,
                sr.len,
                sr.len_code_delta,
                distance_code,
            );
            *num_literals = (*num_literals as libc::c_ulong).wrapping_add(insert_length)
                as size_t as size_t;
            insert_length = 0 as libc::c_int as size_t;
            let mut range_start = position
                .wrapping_add(2 as libc::c_int as libc::c_ulong);
            let mut range_end = brotli_min_size_t(
                position.wrapping_add(sr.len),
                store_end,
            );
            if sr.distance < sr.len >> 2 as libc::c_int {
                range_start = brotli_min_size_t(
                    range_end,
                    brotli_max_size_t(
                        range_start,
                        position
                            .wrapping_add(sr.len)
                            .wrapping_sub(sr.distance << 2 as libc::c_int),
                    ),
                );
            }
            StoreRangeH55(privat, ringbuffer, ringbuffer_mask, range_start, range_end);
            position = (position as libc::c_ulong).wrapping_add(sr.len) as size_t
                as size_t;
        } else {
            insert_length = insert_length.wrapping_add(1);
            position = position.wrapping_add(1);
            if position > apply_random_heuristics {
                if position
                    > apply_random_heuristics
                        .wrapping_add(
                            (4 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(random_heuristics_window_size),
                        )
                {
                    let kMargin = brotli_max_size_t(
                        (StoreLookaheadH55())
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        4 as libc::c_int as size_t,
                    );
                    let mut pos_jump = brotli_min_size_t(
                        position.wrapping_add(16 as libc::c_int as libc::c_ulong),
                        pos_end.wrapping_sub(kMargin),
                    );
                    while position < pos_jump {
                        StoreH55(privat, ringbuffer, ringbuffer_mask, position);
                        insert_length = (insert_length as libc::c_ulong)
                            .wrapping_add(4 as libc::c_int as libc::c_ulong) as size_t
                            as size_t;
                        position = (position as libc::c_ulong)
                            .wrapping_add(4 as libc::c_int as libc::c_ulong) as size_t
                            as size_t;
                    }
                } else {
                    let kMargin_0 = brotli_max_size_t(
                        (StoreLookaheadH55())
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        2 as libc::c_int as size_t,
                    );
                    let mut pos_jump_0 = brotli_min_size_t(
                        position.wrapping_add(8 as libc::c_int as libc::c_ulong),
                        pos_end.wrapping_sub(kMargin_0),
                    );
                    while position < pos_jump_0 {
                        StoreH55(privat, ringbuffer, ringbuffer_mask, position);
                        insert_length = (insert_length as libc::c_ulong)
                            .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t
                            as size_t;
                        position = (position as libc::c_ulong)
                            .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t
                            as size_t;
                    }
                }
            }
        }
    }
    insert_length = (insert_length as libc::c_ulong)
        .wrapping_add(pos_end.wrapping_sub(position)) as size_t as size_t;
    *last_insert_len = insert_length;
    *num_commands = (*num_commands as libc::c_ulong)
        .wrapping_add(commands.offset_from(orig_commands) as libc::c_long as size_t)
        as size_t as size_t;
}
#[inline(never)]
unsafe extern "C" fn CreateBackwardReferencesNH65(
    mut num_bytes: size_t,
    mut position: size_t,
    mut ringbuffer: *const uint8_t,
    mut ringbuffer_mask: size_t,
    mut literal_context_lut: ContextLut,
    mut params: *const BrotliEncoderParams,
    mut hasher: *mut Hasher,
    mut dist_cache: *mut libc::c_int,
    mut last_insert_len: *mut size_t,
    mut commands: *mut Command,
    mut num_commands: *mut size_t,
    mut num_literals: *mut size_t,
) {
    let mut privat: *mut H65 = &mut (*hasher).privat._H65;
    let max_backward_limit = ((1 as libc::c_int as size_t) << (*params).lgwin)
        .wrapping_sub(16 as libc::c_int as libc::c_ulong);
    let position_offset = (*params).stream_offset;
    let orig_commands: *const Command = commands;
    let mut insert_length = *last_insert_len;
    let pos_end = position.wrapping_add(num_bytes);
    let store_end = if num_bytes >= StoreLookaheadH65() {
        position
            .wrapping_add(num_bytes)
            .wrapping_sub(StoreLookaheadH65())
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
    } else {
        position
    };
    let random_heuristics_window_size = LiteralSpreeLengthForSparseSearch(params);
    let mut apply_random_heuristics = position
        .wrapping_add(random_heuristics_window_size);
    let gap = 0 as libc::c_int as size_t;
    let kMinScore = ((30 as libc::c_int * 8 as libc::c_int) as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<size_t>() as libc::c_ulong)
        .wrapping_add(100 as libc::c_int as libc::c_ulong);
    PrepareDistanceCacheH65(privat, dist_cache);
    while position.wrapping_add(HashTypeLengthH65()) < pos_end {
        let mut max_length = pos_end.wrapping_sub(position);
        let mut max_distance = brotli_min_size_t(position, max_backward_limit);
        let mut dictionary_start = brotli_min_size_t(
            position.wrapping_add(position_offset),
            max_backward_limit,
        );
        let mut sr = HasherSearchResult {
            len: 0,
            distance: 0,
            score: 0,
            len_code_delta: 0,
        };
        sr.len = 0 as libc::c_int as size_t;
        sr.len_code_delta = 0 as libc::c_int;
        sr.distance = 0 as libc::c_int as size_t;
        sr.score = kMinScore;
        FindLongestMatchH65(
            privat,
            &(*params).dictionary,
            ringbuffer,
            ringbuffer_mask,
            dist_cache,
            position,
            max_length,
            max_distance,
            dictionary_start.wrapping_add(gap),
            (*params).dist.max_distance,
            &mut sr,
        );
        if sr.score > kMinScore {
            let mut delayed_backward_references_in_row = 0 as libc::c_int;
            max_length = max_length.wrapping_sub(1);
            loop {
                let cost_diff_lazy = 175 as libc::c_int as size_t;
                let mut sr2 = HasherSearchResult {
                    len: 0,
                    distance: 0,
                    score: 0,
                    len_code_delta: 0,
                };
                sr2
                    .len = if (*params).quality < 5 as libc::c_int {
                    brotli_min_size_t(
                        (sr.len).wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        max_length,
                    )
                } else {
                    0 as libc::c_int as libc::c_ulong
                };
                sr2.len_code_delta = 0 as libc::c_int;
                sr2.distance = 0 as libc::c_int as size_t;
                sr2.score = kMinScore;
                max_distance = brotli_min_size_t(
                    position.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    max_backward_limit,
                );
                dictionary_start = brotli_min_size_t(
                    position
                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(position_offset),
                    max_backward_limit,
                );
                FindLongestMatchH65(
                    privat,
                    &(*params).dictionary,
                    ringbuffer,
                    ringbuffer_mask,
                    dist_cache,
                    position.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    max_length,
                    max_distance,
                    dictionary_start.wrapping_add(gap),
                    (*params).dist.max_distance,
                    &mut sr2,
                );
                if !(sr2.score >= (sr.score).wrapping_add(cost_diff_lazy)) {
                    break;
                }
                position = position.wrapping_add(1);
                insert_length = insert_length.wrapping_add(1);
                sr = sr2;
                delayed_backward_references_in_row += 1;
                if !(delayed_backward_references_in_row < 4 as libc::c_int
                    && position.wrapping_add(HashTypeLengthH65()) < pos_end)
                {
                    break;
                }
                max_length = max_length.wrapping_sub(1);
            }
            apply_random_heuristics = position
                .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(sr.len))
                .wrapping_add(random_heuristics_window_size);
            dictionary_start = brotli_min_size_t(
                position.wrapping_add(position_offset),
                max_backward_limit,
            );
            let mut distance_code = ComputeDistanceCode(
                sr.distance,
                dictionary_start.wrapping_add(gap),
                dist_cache,
            );
            if sr.distance <= dictionary_start.wrapping_add(gap)
                && distance_code > 0 as libc::c_int as libc::c_ulong
            {
                *dist_cache
                    .offset(
                        3 as libc::c_int as isize,
                    ) = *dist_cache.offset(2 as libc::c_int as isize);
                *dist_cache
                    .offset(
                        2 as libc::c_int as isize,
                    ) = *dist_cache.offset(1 as libc::c_int as isize);
                *dist_cache
                    .offset(
                        1 as libc::c_int as isize,
                    ) = *dist_cache.offset(0 as libc::c_int as isize);
                *dist_cache
                    .offset(0 as libc::c_int as isize) = sr.distance as libc::c_int;
                PrepareDistanceCacheH65(privat, dist_cache);
            }
            let fresh26 = commands;
            commands = commands.offset(1);
            InitCommand(
                fresh26,
                &(*params).dist,
                insert_length,
                sr.len,
                sr.len_code_delta,
                distance_code,
            );
            *num_literals = (*num_literals as libc::c_ulong).wrapping_add(insert_length)
                as size_t as size_t;
            insert_length = 0 as libc::c_int as size_t;
            let mut range_start = position
                .wrapping_add(2 as libc::c_int as libc::c_ulong);
            let mut range_end = brotli_min_size_t(
                position.wrapping_add(sr.len),
                store_end,
            );
            if sr.distance < sr.len >> 2 as libc::c_int {
                range_start = brotli_min_size_t(
                    range_end,
                    brotli_max_size_t(
                        range_start,
                        position
                            .wrapping_add(sr.len)
                            .wrapping_sub(sr.distance << 2 as libc::c_int),
                    ),
                );
            }
            StoreRangeH65(privat, ringbuffer, ringbuffer_mask, range_start, range_end);
            position = (position as libc::c_ulong).wrapping_add(sr.len) as size_t
                as size_t;
        } else {
            insert_length = insert_length.wrapping_add(1);
            position = position.wrapping_add(1);
            if position > apply_random_heuristics {
                if position
                    > apply_random_heuristics
                        .wrapping_add(
                            (4 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(random_heuristics_window_size),
                        )
                {
                    let kMargin = brotli_max_size_t(
                        (StoreLookaheadH65())
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        4 as libc::c_int as size_t,
                    );
                    let mut pos_jump = brotli_min_size_t(
                        position.wrapping_add(16 as libc::c_int as libc::c_ulong),
                        pos_end.wrapping_sub(kMargin),
                    );
                    while position < pos_jump {
                        StoreH65(privat, ringbuffer, ringbuffer_mask, position);
                        insert_length = (insert_length as libc::c_ulong)
                            .wrapping_add(4 as libc::c_int as libc::c_ulong) as size_t
                            as size_t;
                        position = (position as libc::c_ulong)
                            .wrapping_add(4 as libc::c_int as libc::c_ulong) as size_t
                            as size_t;
                    }
                } else {
                    let kMargin_0 = brotli_max_size_t(
                        (StoreLookaheadH65())
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        2 as libc::c_int as size_t,
                    );
                    let mut pos_jump_0 = brotli_min_size_t(
                        position.wrapping_add(8 as libc::c_int as libc::c_ulong),
                        pos_end.wrapping_sub(kMargin_0),
                    );
                    while position < pos_jump_0 {
                        StoreH65(privat, ringbuffer, ringbuffer_mask, position);
                        insert_length = (insert_length as libc::c_ulong)
                            .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t
                            as size_t;
                        position = (position as libc::c_ulong)
                            .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t
                            as size_t;
                    }
                }
            }
        }
    }
    insert_length = (insert_length as libc::c_ulong)
        .wrapping_add(pos_end.wrapping_sub(position)) as size_t as size_t;
    *last_insert_len = insert_length;
    *num_commands = (*num_commands as libc::c_ulong)
        .wrapping_add(commands.offset_from(orig_commands) as libc::c_long as size_t)
        as size_t as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn BrotliCreateBackwardReferences(
    mut num_bytes: size_t,
    mut position: size_t,
    mut ringbuffer: *const uint8_t,
    mut ringbuffer_mask: size_t,
    mut literal_context_lut: ContextLut,
    mut params: *const BrotliEncoderParams,
    mut hasher: *mut Hasher,
    mut dist_cache: *mut libc::c_int,
    mut last_insert_len: *mut size_t,
    mut commands: *mut Command,
    mut num_commands: *mut size_t,
    mut num_literals: *mut size_t,
) {
    match (*params).hasher.type_0 {
        2 => {
            CreateBackwardReferencesNH2(
                num_bytes,
                position,
                ringbuffer,
                ringbuffer_mask,
                literal_context_lut,
                params,
                hasher,
                dist_cache,
                last_insert_len,
                commands,
                num_commands,
                num_literals,
            );
            return;
        }
        3 => {
            CreateBackwardReferencesNH3(
                num_bytes,
                position,
                ringbuffer,
                ringbuffer_mask,
                literal_context_lut,
                params,
                hasher,
                dist_cache,
                last_insert_len,
                commands,
                num_commands,
                num_literals,
            );
            return;
        }
        4 => {
            CreateBackwardReferencesNH4(
                num_bytes,
                position,
                ringbuffer,
                ringbuffer_mask,
                literal_context_lut,
                params,
                hasher,
                dist_cache,
                last_insert_len,
                commands,
                num_commands,
                num_literals,
            );
            return;
        }
        5 => {
            CreateBackwardReferencesNH5(
                num_bytes,
                position,
                ringbuffer,
                ringbuffer_mask,
                literal_context_lut,
                params,
                hasher,
                dist_cache,
                last_insert_len,
                commands,
                num_commands,
                num_literals,
            );
            return;
        }
        6 => {
            CreateBackwardReferencesNH6(
                num_bytes,
                position,
                ringbuffer,
                ringbuffer_mask,
                literal_context_lut,
                params,
                hasher,
                dist_cache,
                last_insert_len,
                commands,
                num_commands,
                num_literals,
            );
            return;
        }
        40 => {
            CreateBackwardReferencesNH40(
                num_bytes,
                position,
                ringbuffer,
                ringbuffer_mask,
                literal_context_lut,
                params,
                hasher,
                dist_cache,
                last_insert_len,
                commands,
                num_commands,
                num_literals,
            );
            return;
        }
        41 => {
            CreateBackwardReferencesNH41(
                num_bytes,
                position,
                ringbuffer,
                ringbuffer_mask,
                literal_context_lut,
                params,
                hasher,
                dist_cache,
                last_insert_len,
                commands,
                num_commands,
                num_literals,
            );
            return;
        }
        42 => {
            CreateBackwardReferencesNH42(
                num_bytes,
                position,
                ringbuffer,
                ringbuffer_mask,
                literal_context_lut,
                params,
                hasher,
                dist_cache,
                last_insert_len,
                commands,
                num_commands,
                num_literals,
            );
            return;
        }
        54 => {
            CreateBackwardReferencesNH54(
                num_bytes,
                position,
                ringbuffer,
                ringbuffer_mask,
                literal_context_lut,
                params,
                hasher,
                dist_cache,
                last_insert_len,
                commands,
                num_commands,
                num_literals,
            );
            return;
        }
        35 => {
            CreateBackwardReferencesNH35(
                num_bytes,
                position,
                ringbuffer,
                ringbuffer_mask,
                literal_context_lut,
                params,
                hasher,
                dist_cache,
                last_insert_len,
                commands,
                num_commands,
                num_literals,
            );
            return;
        }
        55 => {
            CreateBackwardReferencesNH55(
                num_bytes,
                position,
                ringbuffer,
                ringbuffer_mask,
                literal_context_lut,
                params,
                hasher,
                dist_cache,
                last_insert_len,
                commands,
                num_commands,
                num_literals,
            );
            return;
        }
        65 => {
            CreateBackwardReferencesNH65(
                num_bytes,
                position,
                ringbuffer,
                ringbuffer_mask,
                literal_context_lut,
                params,
                hasher,
                dist_cache,
                last_insert_len,
                commands,
                num_commands,
                num_literals,
            );
            return;
        }
        _ => {}
    };
}
