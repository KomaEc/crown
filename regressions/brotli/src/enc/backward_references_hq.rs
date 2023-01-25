use ::libc;
extern "C" {
    fn log2(_: libc::c_double) -> libc::c_double;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    static kBrotliLog2Table: [libc::c_double; 256];
    static kBrotliInsExtra: [uint32_t; 24];
    static kBrotliCopyExtra: [uint32_t; 24];
    
    
    
    
}
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
pub type brotli_alloc_func = Option::<
    unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void,
>;
pub type brotli_free_func = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
>;
pub type ContextLut = *const uint8_t;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor17 { dummy: () }
pub type BrotliEncoderMode = libc::c_uint;
pub const BROTLI_MODE_FONT: BrotliEncoderMode = 2;
pub const BROTLI_MODE_TEXT: BrotliEncoderMode = 1;
pub const BROTLI_MODE_GENERIC: BrotliEncoderMode = 0;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor18 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor19 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor20 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor21 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor22 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor23 { dummy: () }
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MemoryManager {
    pub alloc_func: brotli_alloc_func,
    pub free_func: brotli_free_func,
    pub opaque: *mut libc::c_void,
}
#[derive(Copy, Clone)]

struct ErasedByPreprocessor24 { dummy: () }
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BackwardMatch {
    pub distance: uint32_t,
    pub length_and_code: uint32_t,
}
#[derive(Copy, Clone)]

struct ErasedByPreprocessor25 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor26 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor27 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor28 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor29 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor30 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor31 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor32 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor33 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor34 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor35 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor36 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor37 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor38 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor39 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor40 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor41 { dummy: () }
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZopfliNode {
    pub length: uint32_t,
    pub distance: uint32_t,
    pub dcode_insert_length: uint32_t,
    pub u: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub cost: libc::c_float,
    pub next: uint32_t,
    pub shortcut: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZopfliCostModel {
    pub cost_cmd_: [libc::c_float; 704],
    pub cost_dist_: *mut libc::c_float,
    pub distance_histogram_size: uint32_t,
    pub literal_costs_: *mut libc::c_float,
    pub min_cost_cmd_: libc::c_float,
    pub num_bytes_: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StartPosQueue {
    pub q_: [PosData; 8],
    pub idx_: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PosData {
    pub pos: size_t,
    pub distance_cache: [libc::c_int; 4],
    pub costdiff: libc::c_float,
    pub cost: libc::c_float,
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
unsafe extern "C" fn brotli_min_float(
    mut a: libc::c_float,
    mut b: libc::c_float,
) -> libc::c_float {
    return if a < b { a } else { b };
}
#[inline(always)]
unsafe extern "C" fn brotli_min_size_t(mut a: size_t, mut b: size_t) -> size_t {
    return if a < b { a } else { b };
}
#[inline(always)]
unsafe extern "C" fn brotli_max_size_t(mut a: size_t, mut b: size_t) -> size_t {
    return if a > b { a } else { b };
}
#[inline(always)]
unsafe extern "C" fn Log2FloorNonZero(mut n: size_t) -> uint32_t {
    return 31 as libc::c_uint ^ (n as uint32_t).leading_zeros() as i32 as uint32_t;
}
#[inline(always)]
unsafe extern "C" fn FastLog2(mut v: size_t) -> libc::c_double {
    if v < 256 as libc::c_int as libc::c_ulong {
        return crate::src::enc::backward_references_hq::kBrotliLog2Table[v as usize];
    }
    return log2(v as libc::c_double);
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
        *code= distance_code as uint16_t;
        *extra_bits= 0 as libc::c_int as uint32_t;
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
        *code= (nbits << 10 as libc::c_int
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
        *extra_bits= (dist.wrapping_sub(offset) >> postfix_bits) as uint32_t;
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
        offset= (offset << 5 as libc::c_uint)
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
    mut code: Option<&mut uint16_t>,
) {
    let mut inscode = GetInsertLengthCode(insertlen);
    let mut copycode = GetCopyLengthCode(copylen);
    *code.as_deref_mut().unwrap()= CombineLengthCodes(inscode, copycode, use_last_distance);
}
#[inline(always)]
unsafe extern "C" fn GetInsertExtra(mut inscode: uint16_t) -> uint32_t {
    return crate::src::enc::backward_references_hq::kBrotliInsExtra[inscode as usize];
}
#[inline(always)]
unsafe extern "C" fn GetCopyExtra(mut copycode: uint16_t) -> uint32_t {
    return crate::src::enc::backward_references_hq::kBrotliCopyExtra[copycode as usize];
}
#[inline(always)]
unsafe extern "C" fn InitCommand(
    mut self_0: *mut crate::src::enc::backward_references::Command,
    mut dist: *const crate::src::enc::backward_references::BrotliDistanceParams,
    mut insertlen: size_t,
    mut copylen: size_t,
    mut copylen_code_delta: libc::c_int,
    mut distance_code: size_t,
) {
    let mut delta = copylen_code_delta as int8_t as uint8_t as uint32_t;
    (*self_0).insert_len_= insertlen as uint32_t;
    (*self_0).copy_len_= (copylen | (delta << 25 as libc::c_int) as libc::c_ulong)
        as uint32_t;
    PrefixEncodeCopyDistance(
        distance_code,
        (*dist).num_direct_distance_codes as size_t,
        (*dist).distance_postfix_bits as size_t,
        core::ptr::addr_of_mut!((*self_0).dist_prefix_),
        core::ptr::addr_of_mut!((*self_0).dist_extra_),
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
        Some(&mut (*self_0).cmd_prefix_),
    );
}
#[inline(always)]
unsafe extern "C" fn CommandCopyLen(mut self_0: *const crate::src::enc::backward_references::Command) -> uint32_t {
    return (*self_0).copy_len_ & 0x1ffffff as libc::c_int as libc::c_uint;
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
unsafe extern "C" fn MaxZopfliLen(mut params: *const crate::src::enc::backward_references::BrotliEncoderParams) -> size_t {
    return (if (*params).quality <= 10 as libc::c_int {
        150 as libc::c_int
    } else {
        325 as libc::c_int
    }) as size_t;
}
#[inline(always)]
unsafe extern "C" fn MaxZopfliCandidates(
    mut params: *const crate::src::enc::backward_references::BrotliEncoderParams,
) -> size_t {
    return (if (*params).quality <= 10 as libc::c_int {
        1 as libc::c_int
    } else {
        5 as libc::c_int
    }) as size_t;
}
static mut kInvalidMatch: uint32_t = 0xfffffff as libc::c_int as uint32_t;
static mut kHashMul32: uint32_t = 0x1e35a7bd as libc::c_int as uint32_t;
#[inline(always)]
unsafe extern "C" fn InitBackwardMatch(
    mut self_0: Option<&mut BackwardMatch>,
    mut dist: size_t,
    mut len: size_t,
) {
    (*self_0.as_deref_mut().unwrap()).distance= dist as uint32_t;
    (*self_0.as_deref_mut().unwrap()).length_and_code= (len << 5 as libc::c_int) as uint32_t;
}
#[inline(always)]
unsafe extern "C" fn InitDictionaryBackwardMatch(
    mut self_0: Option<&mut BackwardMatch>,
    mut dist: size_t,
    mut len: size_t,
    mut len_code: size_t,
) {
    (*self_0.as_deref_mut().unwrap()).distance= dist as uint32_t;
    (*self_0.as_deref_mut().unwrap()).length_and_code= (len << 5 as libc::c_int
        | (if len == len_code { 0 as libc::c_int as libc::c_ulong } else { len_code }))
        as uint32_t;
}
#[inline(always)]
unsafe extern "C" fn BackwardMatchLength(mut self_0: *const BackwardMatch) -> size_t {
    return ((*self_0).length_and_code >> 5 as libc::c_int) as size_t;
}
#[inline(always)]
unsafe extern "C" fn BackwardMatchLengthCode(
    mut self_0: *const BackwardMatch,
) -> size_t {
    let mut code = ((*self_0).length_and_code & 31 as libc::c_int as libc::c_uint)
        as size_t;
    return if code != 0 { code } else { BackwardMatchLength(self_0) };
}
#[inline(always)]
unsafe extern "C" fn HashTypeLengthH10() -> size_t {
    return 4 as libc::c_int as size_t;
}
#[inline(always)]
unsafe extern "C" fn StoreLookaheadH10() -> size_t {
    return 128 as libc::c_int as size_t;
}
unsafe extern "C" fn HashBytesH10(mut data: *const uint8_t) -> uint32_t {
    let mut h = (BrotliUnalignedRead32(data as *const libc::c_void))
        .wrapping_mul(crate::src::enc::backward_references_hq::kHashMul32);
    return h >> 32 as libc::c_int - 17 as libc::c_int;
}
#[inline(always)]
unsafe extern "C" fn LeftChildIndexH10(mut self_0: *mut crate::src::enc::backward_references::H10, mut pos: size_t) -> size_t {
    return (2 as libc::c_int as libc::c_ulong)
        .wrapping_mul(pos & (*self_0).window_mask_);
}
#[inline(always)]
unsafe extern "C" fn RightChildIndexH10(mut self_0: *mut crate::src::enc::backward_references::H10, mut pos: size_t) -> size_t {
    return (2 as libc::c_int as libc::c_ulong)
        .wrapping_mul(pos & (*self_0).window_mask_)
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
}
#[inline(always)]
unsafe extern "C" fn StoreAndFindMatchesH10(
    mut self_0: *mut crate::src::enc::backward_references::H10,
    mut data: *const uint8_t,
    mut cur_ix: size_t,
    mut ring_buffer_mask: size_t,
    mut max_length: size_t,
    mut max_backward: size_t,
    mut best_len: *mut size_t,
    mut matches: *mut BackwardMatch,
) -> *mut BackwardMatch {
    let cur_ix_masked = cur_ix & ring_buffer_mask;
    let max_comp_len = brotli_min_size_t(max_length, 128 as libc::c_int as size_t);
    let should_reroot_tree = if max_length >= 128 as libc::c_int as libc::c_ulong {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
    let key = HashBytesH10(&*data.offset(cur_ix_masked as isize));
    let mut buckets = (*self_0).buckets_;
    let mut forest = (*self_0).forest_;
    let mut prev_ix = *buckets.offset(key as isize) as size_t;
    let mut node_left = LeftChildIndexH10(self_0, cur_ix);
    let mut node_right = RightChildIndexH10(self_0, cur_ix);
    let mut best_len_left = 0 as libc::c_int as size_t;
    let mut best_len_right = 0 as libc::c_int as size_t;
    let mut depth_remaining: size_t = 0;
    if should_reroot_tree != 0 {
        *buckets.offset(key as isize) = cur_ix as uint32_t;
    }
    depth_remaining= 64 as libc::c_int as size_t;
    loop {
        let backward = cur_ix.wrapping_sub(prev_ix);
        let prev_ix_masked = prev_ix & ring_buffer_mask;
        if backward == 0 as libc::c_int as libc::c_ulong || backward > max_backward
            || depth_remaining == 0 as libc::c_int as libc::c_ulong
        {
            if should_reroot_tree != 0 {
                *forest.offset(node_left as isize) = (*self_0).invalid_pos_;
                *forest.offset(node_right as isize) = (*self_0).invalid_pos_;
            }
            break;
        } else {
            let cur_len = brotli_min_size_t(best_len_left, best_len_right);
            let mut len: size_t = 0;
            len= cur_len
                .wrapping_add(
                    FindMatchLengthWithLimit(
                        &*data.offset(cur_ix_masked.wrapping_add(cur_len) as isize),
                        &*data.offset(prev_ix_masked.wrapping_add(cur_len) as isize),
                        max_length.wrapping_sub(cur_len),
                    ),
                );
            if !matches.is_null() && len > (*best_len) {
                *best_len= len;
                let fresh0 = matches;
                matches= matches.offset(1);
                InitBackwardMatch(fresh0.as_mut(), backward, len);
            }
            if len >= max_comp_len {
                if should_reroot_tree != 0 {
                    *forest
                        .offset(
                            node_left as isize,
                        ) = *forest.offset(LeftChildIndexH10(self_0, prev_ix) as isize);
                    *forest
                        .offset(
                            node_right as isize,
                        ) = *forest.offset(RightChildIndexH10(self_0, prev_ix) as isize);
                }
                break;
            } else {
                if *data.offset(cur_ix_masked.wrapping_add(len) as isize) as libc::c_int
                    > *data.offset(prev_ix_masked.wrapping_add(len) as isize)
                        as libc::c_int
                {
                    best_len_left= len;
                    if should_reroot_tree != 0 {
                        *forest.offset(node_left as isize) = prev_ix as uint32_t;
                    }
                    node_left= RightChildIndexH10(self_0, prev_ix);
                    prev_ix= *forest.offset(node_left as isize) as size_t;
                } else {
                    best_len_right= len;
                    if should_reroot_tree != 0 {
                        *forest.offset(node_right as isize) = prev_ix as uint32_t;
                    }
                    node_right= LeftChildIndexH10(self_0, prev_ix);
                    prev_ix= *forest.offset(node_right as isize) as size_t;
                }
                depth_remaining= depth_remaining.wrapping_sub(1);
            }
        }
    }
    return matches;
}
#[inline(always)]
unsafe extern "C" fn FindAllMatchesH10(
    mut self_0: *mut crate::src::enc::backward_references::H10,
    mut dictionary: *const crate::src::enc::backward_references::BrotliEncoderDictionary,
    mut data: *const uint8_t,
    mut ring_buffer_mask: size_t,
    mut cur_ix: size_t,
    mut max_length: size_t,
    mut max_backward: size_t,
    mut dictionary_distance: size_t,
    mut params: *const crate::src::enc::backward_references::BrotliEncoderParams,
    mut matches: *mut BackwardMatch,
) -> size_t {
    let orig_matches = matches;
    let cur_ix_masked = cur_ix & ring_buffer_mask;
    let mut best_len = 1 as libc::c_int as size_t;
    let short_match_max_backward = (if (*params).quality != 11 as libc::c_int {
        16 as libc::c_int
    } else {
        64 as libc::c_int
    }) as size_t;
    let mut stop = cur_ix.wrapping_sub(short_match_max_backward);
    let mut dict_matches: [uint32_t; 38] = [0; 38];
    let mut i: size_t = 0;
    if cur_ix < short_match_max_backward {
        stop= 0 as libc::c_int as size_t;
    }
    i= cur_ix.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    while i > stop && best_len <= 2 as libc::c_int as libc::c_ulong {
        let mut prev_ix = i;
        let backward = cur_ix.wrapping_sub(prev_ix);
        if (backward > max_backward) as libc::c_int as libc::c_long != 0 {
            break;
        }
        prev_ix&= ring_buffer_mask;
        if !(*data.offset(cur_ix_masked as isize) as libc::c_int
            != *data.offset(prev_ix as isize) as libc::c_int
            || *data
                .offset(
                    cur_ix_masked.wrapping_add(1 as libc::c_int as libc::c_ulong)
                        as isize,
                ) as libc::c_int
                != *data
                    .offset(
                        prev_ix.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    ) as libc::c_int)
        {
            let len = FindMatchLengthWithLimit(
                &*data.offset(prev_ix as isize),
                &*data.offset(cur_ix_masked as isize),
                max_length,
            );
            if len > best_len {
                best_len= len;
                let fresh1 = matches;
                matches= matches.offset(1);
                InitBackwardMatch(fresh1.as_mut(), backward, len);
            }
        }
        i= i.wrapping_sub(1);
    }
    if best_len < max_length {
        matches= StoreAndFindMatchesH10(
            self_0,
            data,
            cur_ix,
            ring_buffer_mask,
            max_length,
            max_backward,
            core::ptr::addr_of_mut!(best_len),
            matches,
        );
    }
    i= 0 as libc::c_int as size_t;
    while i <= 37 as libc::c_int as libc::c_ulong {
        dict_matches[i as usize]= crate::src::enc::backward_references_hq::kInvalidMatch;
        i= i.wrapping_add(1);
    }
    let mut minlen = brotli_max_size_t(
        4 as libc::c_int as size_t,
        best_len.wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    if crate::src::enc::static_dict::BrotliFindAllStaticDictionaryMatches(
        dictionary,
        &*data.offset(cur_ix_masked as isize),
        minlen,
        max_length,
        core::ptr::addr_of_mut!(*dict_matches.as_mut_ptr().offset(0 as libc::c_int as isize)),
    ) != 0
    {
        let mut maxlen = brotli_min_size_t(37 as libc::c_int as size_t, max_length);
        let mut l: size_t = 0;
        l= minlen;
        while l <= maxlen {
            let mut dict_id = dict_matches[l as usize];
            if dict_id < crate::src::enc::backward_references_hq::kInvalidMatch {
                let mut distance = dictionary_distance
                    .wrapping_add((dict_id >> 5 as libc::c_int) as libc::c_ulong)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong);
                if distance <= (*params).dist.max_distance {
                    let fresh2 = matches;
                    matches= matches.offset(1);
                    InitDictionaryBackwardMatch(
                        fresh2.as_mut(),
                        distance,
                        l,
                        (dict_id & 31 as libc::c_int as libc::c_uint) as size_t,
                    );
                }
            }
            l= l.wrapping_add(1);
        }
    }
    return matches.offset_from(orig_matches) as libc::c_long as size_t;
}
#[inline(always)]
unsafe extern "C" fn StoreH10(
    mut self_0: *mut crate::src::enc::backward_references::H10,
    mut data: *const uint8_t,
    mut mask: size_t,
    mut ix: size_t,
) {
    let max_backward = (*self_0).window_mask_
        .wrapping_sub(16 as libc::c_int as libc::c_ulong)
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    StoreAndFindMatchesH10(
        self_0,
        data,
        ix,
        mask,
        128 as libc::c_int as size_t,
        max_backward,
        0 as *mut size_t,
        0 as *mut BackwardMatch,
    );
}
#[inline(always)]
unsafe extern "C" fn StoreRangeH10(
    mut self_0: *mut crate::src::enc::backward_references::H10,
    mut data: *const uint8_t,
    mut mask: size_t,
    mut ix_start: size_t,
    mut ix_end: size_t,
) {
    let mut i = ix_start;
    let mut j = ix_start;
    if ix_start.wrapping_add(63 as libc::c_int as libc::c_ulong) <= ix_end {
        i= ix_end.wrapping_sub(63 as libc::c_int as libc::c_ulong);
    }
    if ix_start.wrapping_add(512 as libc::c_int as libc::c_ulong) <= i {
        while j < i {
            StoreH10(self_0, data, mask, j);
            j= (j as libc::c_ulong).wrapping_add(8 as libc::c_int as libc::c_ulong)
                as size_t as size_t;
        }
    }
    while i < ix_end {
        StoreH10(self_0, data, mask, i);
        i= i.wrapping_add(1);
    }
}
static mut kInfinity: libc::c_float = 1.7e38f32;
static mut kDistanceCacheIndex: [uint32_t; 16] = [
    0 as libc::c_int as uint32_t,
    1 as libc::c_int as uint32_t,
    2 as libc::c_int as uint32_t,
    3 as libc::c_int as uint32_t,
    0 as libc::c_int as uint32_t,
    0 as libc::c_int as uint32_t,
    0 as libc::c_int as uint32_t,
    0 as libc::c_int as uint32_t,
    0 as libc::c_int as uint32_t,
    0 as libc::c_int as uint32_t,
    1 as libc::c_int as uint32_t,
    1 as libc::c_int as uint32_t,
    1 as libc::c_int as uint32_t,
    1 as libc::c_int as uint32_t,
    1 as libc::c_int as uint32_t,
    1 as libc::c_int as uint32_t,
];
static mut kDistanceCacheOffset: [libc::c_int; 16] = [
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    -(1 as libc::c_int),
    1 as libc::c_int,
    -(2 as libc::c_int),
    2 as libc::c_int,
    -(3 as libc::c_int),
    3 as libc::c_int,
    -(1 as libc::c_int),
    1 as libc::c_int,
    -(2 as libc::c_int),
    2 as libc::c_int,
    -(3 as libc::c_int),
    3 as libc::c_int,
];
#[no_mangle]
pub unsafe extern "C" fn BrotliInitZopfliNodes(
    mut array: *mut ZopfliNode,
    mut length: size_t,
) {
    let mut stub = ZopfliNode {
        length: 0,
        distance: 0,
        dcode_insert_length: 0,
        u: C2RustUnnamed_0 { cost: 0. },
    };
    let mut i: size_t = 0;
    stub.length= 1 as libc::c_int as uint32_t;
    stub.distance= 0 as libc::c_int as uint32_t;
    stub.dcode_insert_length= 0 as libc::c_int as uint32_t;
    stub.u.cost= crate::src::enc::backward_references_hq::kInfinity;
    i= 0 as libc::c_int as size_t;
    while i < length {
        *array.offset(i as isize) = stub;
        i= i.wrapping_add(1);
    }
}
#[inline(always)]
unsafe extern "C" fn ZopfliNodeCopyLength(mut self_0: *const ZopfliNode) -> uint32_t {
    return (*self_0).length & 0x1ffffff as libc::c_int as libc::c_uint;
}
#[inline(always)]
unsafe extern "C" fn ZopfliNodeLengthCode(mut self_0: *const ZopfliNode) -> uint32_t {
    let modifier = (*self_0).length >> 25 as libc::c_int;
    return (ZopfliNodeCopyLength(self_0))
        .wrapping_add(9 as libc::c_uint)
        .wrapping_sub(modifier);
}
#[inline(always)]
unsafe extern "C" fn ZopfliNodeCopyDistance(mut self_0: *const ZopfliNode) -> uint32_t {
    return (*self_0).distance;
}
#[inline(always)]
unsafe extern "C" fn ZopfliNodeDistanceCode(mut self_0: *const ZopfliNode) -> uint32_t {
    let short_code = (*self_0).dcode_insert_length >> 27 as libc::c_int;
    return if short_code == 0 as libc::c_int as libc::c_uint {
        (ZopfliNodeCopyDistance(self_0))
            .wrapping_add(16 as libc::c_int as libc::c_uint)
            .wrapping_sub(1 as libc::c_int as libc::c_uint)
    } else {
        short_code.wrapping_sub(1 as libc::c_int as libc::c_uint)
    };
}
#[inline(always)]
unsafe extern "C" fn ZopfliNodeCommandLength(mut self_0: *const ZopfliNode) -> uint32_t {
    return (ZopfliNodeCopyLength(self_0))
        .wrapping_add(
            (*self_0).dcode_insert_length & 0x7ffffff as libc::c_int as libc::c_uint,
        );
}
unsafe extern "C" fn InitZopfliCostModel(
    mut m: *mut MemoryManager,
    mut self_0: *mut ZopfliCostModel,
    mut dist: *const crate::src::enc::backward_references::BrotliDistanceParams,
    mut num_bytes: size_t,
) {
    (*self_0).num_bytes_= num_bytes;
    (*self_0).literal_costs_= if num_bytes.wrapping_add(2 as libc::c_int as libc::c_ulong)
        > 0 as libc::c_int as libc::c_ulong
    {
        crate::src::enc::memory::BrotliAllocate(
            m,
            num_bytes
                .wrapping_add(2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_float>() as libc::c_ulong),
        ) as *mut libc::c_float
    } else {
        0 as *mut libc::c_float
    };
    (*self_0).cost_dist_= if (*dist).alphabet_size_limit > 0 as libc::c_int as libc::c_uint {
        crate::src::enc::memory::BrotliAllocate(
            m,
            ((*dist).alphabet_size_limit as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_float>() as libc::c_ulong),
        ) as *mut libc::c_float
    } else {
        0 as *mut libc::c_float
    };
    (*self_0).distance_histogram_size= (*dist).alphabet_size_limit;
    if 0 as libc::c_int != 0 {
        return;
    }
}
unsafe extern "C" fn CleanupZopfliCostModel(
    mut m: *mut MemoryManager,
    mut self_0: *mut ZopfliCostModel,
) {
    crate::src::enc::memory::BrotliFree(m, (*self_0).literal_costs_ as *mut libc::c_void);
    (*self_0).literal_costs_= 0 as *mut libc::c_float;
    crate::src::enc::memory::BrotliFree(m, (*self_0).cost_dist_ as *mut libc::c_void);
    (*self_0).cost_dist_= 0 as *mut libc::c_float;
}
unsafe extern "C" fn SetCost(
    mut histogram: *const uint32_t,
    mut histogram_size: size_t,
    mut literal_histogram: libc::c_int,
    mut cost: *mut libc::c_float,
) {
    let mut sum = 0 as libc::c_int as size_t;
    let mut missing_symbol_sum: size_t = 0;
    let mut log2sum: libc::c_float = 0.;
    let mut missing_symbol_cost: libc::c_float = 0.;
    let mut i: size_t = 0;
    i= 0 as libc::c_int as size_t;
    while i < histogram_size {
        sum= (sum as libc::c_ulong)
            .wrapping_add(*histogram.offset(i as isize) as libc::c_ulong) as size_t
            as size_t;
        i= i.wrapping_add(1);
    }
    log2sum= FastLog2(sum) as libc::c_float;
    missing_symbol_sum= sum;
    if literal_histogram == 0 {
        i= 0 as libc::c_int as size_t;
        while i < histogram_size {
            if *histogram.offset(i as isize) == 0 as libc::c_int as libc::c_uint {
                missing_symbol_sum= missing_symbol_sum.wrapping_add(1);
            }
            i= i.wrapping_add(1);
        }
    }
    missing_symbol_cost= FastLog2(missing_symbol_sum) as libc::c_float
        + 2 as libc::c_int as libc::c_float;
    i= 0 as libc::c_int as size_t;
    while i < histogram_size {
        if *histogram.offset(i as isize) == 0 as libc::c_int as libc::c_uint {
            *cost.offset(i as isize) = missing_symbol_cost;
        } else {
            *cost
                .offset(
                    i as isize,
                ) = log2sum
                - FastLog2(*histogram.offset(i as isize) as size_t) as libc::c_float;
            if *cost.offset(i as isize) < 1 as libc::c_int as libc::c_float {
                *cost.offset(i as isize) = 1 as libc::c_int as libc::c_float;
            }
        }
        i= i.wrapping_add(1);
    }
}
unsafe extern "C" fn ZopfliCostModelSetFromCommands(
    mut self_0: *mut ZopfliCostModel,
    mut position: size_t,
    mut ringbuffer: *const uint8_t,
    mut ringbuffer_mask: size_t,
    mut commands: *const crate::src::enc::backward_references::Command,
    mut num_commands: size_t,
    mut last_insert_len: size_t,
) {
    let mut histogram_literal: [uint32_t; 256] = [0; 256];
    let mut histogram_cmd: [uint32_t; 704] = [0; 704];
    let mut histogram_dist: [uint32_t; 544] = [0; 544];
    let mut cost_literal: [libc::c_float; 256] = [0.; 256];
    let mut pos = position.wrapping_sub(last_insert_len);
    let mut min_cost_cmd = crate::src::enc::backward_references_hq::kInfinity;
    let mut i: size_t = 0;
    let mut cost_cmd = (*self_0).cost_cmd_.as_mut_ptr();
    memset(
        histogram_literal.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[uint32_t; 256]>() as libc::c_ulong,
    );
    memset(
        histogram_cmd.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[uint32_t; 704]>() as libc::c_ulong,
    );
    memset(
        histogram_dist.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[uint32_t; 544]>() as libc::c_ulong,
    );
    i= 0 as libc::c_int as size_t;
    while i < num_commands {
        let mut inslength = (*commands.offset(i as isize)).insert_len_ as size_t;
        let mut copylength = CommandCopyLen(&*commands.offset(i as isize)) as size_t;
        let mut distcode = ((*commands.offset(i as isize)).dist_prefix_ as libc::c_int
            & 0x3ff as libc::c_int) as size_t;
        let mut cmdcode = (*commands.offset(i as isize)).cmd_prefix_ as size_t;
        let mut j: size_t = 0;
        histogram_cmd[cmdcode
            as usize]= histogram_cmd[cmdcode as usize].wrapping_add(1);
        if cmdcode >= 128 as libc::c_int as libc::c_ulong {
            histogram_dist[distcode
                as usize]= histogram_dist[distcode as usize].wrapping_add(1);
        }
        j= 0 as libc::c_int as size_t;
        while j < inslength {
            histogram_literal[*ringbuffer
                .offset((pos.wrapping_add(j) & ringbuffer_mask) as isize)
                as usize]= histogram_literal[*ringbuffer
                .offset((pos.wrapping_add(j) & ringbuffer_mask) as isize) as usize]
                .wrapping_add(1);
            j= j.wrapping_add(1);
        }
        pos= (pos as libc::c_ulong).wrapping_add(inslength.wrapping_add(copylength))
            as size_t as size_t;
        i= i.wrapping_add(1);
    }
    SetCost(
        histogram_literal.as_mut_ptr(),
        256 as libc::c_int as size_t,
        1 as libc::c_int,
        cost_literal.as_mut_ptr(),
    );
    SetCost(
        histogram_cmd.as_mut_ptr(),
        704 as libc::c_int as size_t,
        0 as libc::c_int,
        cost_cmd,
    );
    SetCost(
        histogram_dist.as_mut_ptr(),
        (*self_0).distance_histogram_size as size_t,
        0 as libc::c_int,
        (*self_0).cost_dist_,
    );
    i= 0 as libc::c_int as size_t;
    while i < 704 as libc::c_int as libc::c_ulong {
        min_cost_cmd= brotli_min_float(min_cost_cmd, *cost_cmd.offset(i as isize));
        i= i.wrapping_add(1);
    }
    (*self_0).min_cost_cmd_= min_cost_cmd;
    let mut literal_costs = (*self_0).literal_costs_;
    let mut literal_carry = 0.0f64 as libc::c_float;
    let mut num_bytes = (*self_0).num_bytes_;
    *literal_costs.offset(0 as libc::c_int as isize) = 0.0f64 as libc::c_float;
    i= 0 as libc::c_int as size_t;
    while i < num_bytes {
        literal_carry+= cost_literal[*ringbuffer
                .offset((position.wrapping_add(i) & ringbuffer_mask) as isize) as usize];
        *literal_costs
            .offset(
                i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            ) = *literal_costs.offset(i as isize) + literal_carry;
        literal_carry-= *literal_costs
                .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
                - *literal_costs.offset(i as isize);
        i= i.wrapping_add(1);
    }
}
unsafe extern "C" fn ZopfliCostModelSetFromLiteralCosts(
    mut self_0: *mut ZopfliCostModel,
    mut position: size_t,
    mut ringbuffer: *const uint8_t,
    mut ringbuffer_mask: size_t,
) {
    let mut literal_costs = (*self_0).literal_costs_;
    let mut literal_carry = 0.0f64 as libc::c_float;
    let mut cost_dist = (*self_0).cost_dist_;
    let mut cost_cmd = (*self_0).cost_cmd_.as_mut_ptr();
    let mut num_bytes = (*self_0).num_bytes_;
    let mut i: size_t = 0;
    crate::src::enc::literal_cost::BrotliEstimateBitCostsForLiterals(
        position,
        num_bytes,
        ringbuffer_mask,
        ringbuffer,
        core::ptr::addr_of_mut!(*literal_costs.offset(1 as libc::c_int as isize)),
    );
    *literal_costs.offset(0 as libc::c_int as isize) = 0.0f64 as libc::c_float;
    i= 0 as libc::c_int as size_t;
    while i < num_bytes {
        literal_carry+= *literal_costs
                .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
        *literal_costs
            .offset(
                i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            ) = *literal_costs.offset(i as isize) + literal_carry;
        literal_carry-= *literal_costs
                .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
                - *literal_costs.offset(i as isize);
        i= i.wrapping_add(1);
    }
    i= 0 as libc::c_int as size_t;
    while i < 704 as libc::c_int as libc::c_ulong {
        *cost_cmd
            .offset(
                i as isize,
            ) = FastLog2(
            (11 as libc::c_int as libc::c_uint).wrapping_add(i as uint32_t) as size_t,
        ) as libc::c_float;
        i= i.wrapping_add(1);
    }
    i= 0 as libc::c_int as size_t;
    while i < (*self_0).distance_histogram_size as libc::c_ulong {
        *cost_dist
            .offset(
                i as isize,
            ) = FastLog2(
            (20 as libc::c_int as libc::c_uint).wrapping_add(i as uint32_t) as size_t,
        ) as libc::c_float;
        i= i.wrapping_add(1);
    }
    (*self_0).min_cost_cmd_= FastLog2(11 as libc::c_int as size_t) as libc::c_float;
}
#[inline(always)]
unsafe extern "C" fn ZopfliCostModelGetCommandCost(
    mut self_0: *const ZopfliCostModel,
    mut cmdcode: uint16_t,
) -> libc::c_float {
    return (*self_0).cost_cmd_[cmdcode as usize];
}
#[inline(always)]
unsafe extern "C" fn ZopfliCostModelGetDistanceCost(
    mut self_0: *const ZopfliCostModel,
    mut distcode: size_t,
) -> libc::c_float {
    return *(*self_0).cost_dist_.offset(distcode as isize);
}
#[inline(always)]
unsafe extern "C" fn ZopfliCostModelGetLiteralCosts(
    mut self_0: *const ZopfliCostModel,
    mut from: size_t,
    mut to: size_t,
) -> libc::c_float {
    return *(*self_0).literal_costs_.offset(to as isize)
        - *(*self_0).literal_costs_.offset(from as isize);
}
#[inline(always)]
unsafe extern "C" fn ZopfliCostModelGetMinCostCmd(
    mut self_0: *const ZopfliCostModel,
) -> libc::c_float {
    return (*self_0).min_cost_cmd_;
}
#[inline(always)]
unsafe extern "C" fn UpdateZopfliNode(
    mut nodes: *mut ZopfliNode,
    mut pos: size_t,
    mut start_pos: size_t,
    mut len: size_t,
    mut len_code: size_t,
    mut dist: size_t,
    mut short_code: size_t,
    mut cost: libc::c_float,
) {
    let mut next: *mut ZopfliNode = core::ptr::addr_of_mut!(*nodes.offset(pos.wrapping_add(len) as isize))
        as *mut ZopfliNode;
    (*next).length= (len
        | len.wrapping_add(9 as libc::c_uint as libc::c_ulong).wrapping_sub(len_code)
            << 25 as libc::c_int) as uint32_t;
    (*next).distance= dist as uint32_t;
    (*next).dcode_insert_length= (short_code << 27 as libc::c_int
        | pos.wrapping_sub(start_pos)) as uint32_t;
    (*next).u.cost= cost;
}
#[inline(always)]
unsafe extern "C" fn InitStartPosQueue(mut self_0: Option<&mut StartPosQueue>) {
    (*self_0.as_deref_mut().unwrap()).idx_= 0 as libc::c_int as size_t;
}
unsafe extern "C" fn StartPosQueueSize(mut self_0: *const StartPosQueue) -> size_t {
    return brotli_min_size_t((*self_0).idx_, 8 as libc::c_int as size_t);
}
unsafe extern "C" fn StartPosQueuePush(
    mut self_0: *mut StartPosQueue,
    mut posdata: *const PosData,
) {
    let fresh8 = (*self_0).idx_;(*self_0).idx_= (*self_0).idx_.wrapping_add(1);
    let mut offset = !fresh8 & 7 as libc::c_int as libc::c_ulong;
    let mut len = StartPosQueueSize(self_0);
    let mut i: size_t = 0;
    let mut q = (*self_0).q_.as_mut_ptr();
    *q.offset(offset as isize) = (*posdata);
    i= 1 as libc::c_int as size_t;
    while i < len {
        if (*q.offset((offset & 7 as libc::c_int as libc::c_ulong) as isize)).costdiff
            > (*q
                .offset(
                    (offset.wrapping_add(1 as libc::c_int as libc::c_ulong)
                        & 7 as libc::c_int as libc::c_ulong) as isize,
                ))
                .costdiff
        {
            let mut __brotli_swap_tmp = *q
                .offset((offset & 7 as libc::c_int as libc::c_ulong) as isize);
            *q
                .offset(
                    (offset & 7 as libc::c_int as libc::c_ulong) as isize,
                ) = *q
                .offset(
                    (offset.wrapping_add(1 as libc::c_int as libc::c_ulong)
                        & 7 as libc::c_int as libc::c_ulong) as isize,
                );
            *q
                .offset(
                    (offset.wrapping_add(1 as libc::c_int as libc::c_ulong)
                        & 7 as libc::c_int as libc::c_ulong) as isize,
                ) = __brotli_swap_tmp;
        }
        offset= offset.wrapping_add(1);
        i= i.wrapping_add(1);
    }
}
unsafe extern "C" fn StartPosQueueAt(
    mut self_0: *const StartPosQueue,
    mut k: size_t,
) -> *const PosData {
    return &*((*self_0).q_)
        .as_ptr()
        .offset(
            (k.wrapping_sub((*self_0).idx_) & 7 as libc::c_int as libc::c_ulong) as isize,
        ) as *const PosData;
}
unsafe extern "C" fn ComputeMinimumCopyLength(
    mut start_cost: libc::c_float,
    mut nodes: *const ZopfliNode,
    mut num_bytes: size_t,
    mut pos: size_t,
) -> size_t {
    let mut min_cost = start_cost;
    let mut len = 2 as libc::c_int as size_t;
    let mut next_len_bucket = 4 as libc::c_int as size_t;
    let mut next_len_offset = 10 as libc::c_int as size_t;
    while pos.wrapping_add(len) <= num_bytes
        && (*nodes.offset(pos.wrapping_add(len) as isize)).u.cost <= min_cost
    {
        len= len.wrapping_add(1);
        if len == next_len_offset {
            min_cost+= 1.0f32;
            next_len_offset= (next_len_offset as libc::c_ulong)
                .wrapping_add(next_len_bucket) as size_t as size_t;
            next_len_bucket= (next_len_bucket as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
        }
    }
    return len;
}
unsafe extern "C" fn ComputeDistanceShortcut(
    mut block_start: size_t,
    mut pos: size_t,
    mut max_backward_limit: size_t,
    mut gap: size_t,
    mut nodes: *const ZopfliNode,
) -> uint32_t {
    let clen = ZopfliNodeCopyLength(&*nodes.offset(pos as isize)) as size_t;
    let ilen = ((*nodes.offset(pos as isize)).dcode_insert_length
        & 0x7ffffff as libc::c_int as libc::c_uint) as size_t;
    let dist = ZopfliNodeCopyDistance(&*nodes.offset(pos as isize)) as size_t;
    if pos == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int as uint32_t
    } else if dist.wrapping_add(clen) <= block_start.wrapping_add(pos).wrapping_add(gap)
        && dist <= max_backward_limit.wrapping_add(gap)
        && ZopfliNodeDistanceCode(&*nodes.offset(pos as isize))
            > 0 as libc::c_int as libc::c_uint
    {
        return pos as uint32_t
    } else {
        return (*nodes.offset(pos.wrapping_sub(clen).wrapping_sub(ilen) as isize))
            .u
            .shortcut
    };
}
unsafe extern "C" fn ComputeDistanceCache(
    mut pos: size_t,
    mut starting_dist_cache: *const libc::c_int,
    mut nodes: *const ZopfliNode,
    mut dist_cache: *mut libc::c_int,
) {
    let mut idx = 0 as libc::c_int;
    let mut p = (*nodes.offset(pos as isize)).u.shortcut as size_t;
    while idx < 4 as libc::c_int && p > 0 as libc::c_int as libc::c_ulong {
        let ilen = ((*nodes.offset(p as isize)).dcode_insert_length
            & 0x7ffffff as libc::c_int as libc::c_uint) as size_t;
        let clen = ZopfliNodeCopyLength(&*nodes.offset(p as isize)) as size_t;
        let dist = ZopfliNodeCopyDistance(&*nodes.offset(p as isize)) as size_t;
        let fresh9 = idx;
        idx= idx + 1;
        *dist_cache.offset(fresh9 as isize) = dist as libc::c_int;
        p= (*nodes.offset(p.wrapping_sub(clen).wrapping_sub(ilen) as isize)).u.shortcut
            as size_t;
    }
    while idx < 4 as libc::c_int {
        let fresh10 = starting_dist_cache;
        starting_dist_cache= starting_dist_cache.offset(1);
        *dist_cache.offset(idx as isize) = (*fresh10);
        idx+= 1;
    }
}
unsafe extern "C" fn EvaluateNode(
    mut block_start: size_t,
    mut pos: size_t,
    mut max_backward_limit: size_t,
    mut gap: size_t,
    mut starting_dist_cache: *const libc::c_int,
    mut model: *const ZopfliCostModel,
    mut queue: Option<&mut StartPosQueue>,
    mut nodes: *mut ZopfliNode,
) {
    let mut node_cost = (*nodes.offset(pos as isize)).u.cost;
    (*nodes.offset(pos as isize))
        .u
        .shortcut = ComputeDistanceShortcut(
        block_start,
        pos,
        max_backward_limit,
        gap,
        nodes,
    );
    if node_cost
        <= ZopfliCostModelGetLiteralCosts(model, 0 as libc::c_int as size_t, pos)
    {
        let mut posdata = PosData {
            pos: 0,
            distance_cache: [0; 4],
            costdiff: 0.,
            cost: 0.,
        };
        posdata.pos= pos;
        posdata.cost= node_cost;
        posdata.costdiff= node_cost
            - ZopfliCostModelGetLiteralCosts(model, 0 as libc::c_int as size_t, pos);
        ComputeDistanceCache(
            pos,
            starting_dist_cache,
            nodes,
            posdata.distance_cache.as_mut_ptr(),
        );
        StartPosQueuePush(queue.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()), core::ptr::addr_of!(posdata));
    }
}
unsafe extern "C" fn UpdateNodes(
    mut num_bytes: size_t,
    mut block_start: size_t,
    mut pos: size_t,
    mut ringbuffer: *const uint8_t,
    mut ringbuffer_mask: size_t,
    mut params: *const crate::src::enc::backward_references::BrotliEncoderParams,
    mut max_backward_limit: size_t,
    mut starting_dist_cache: *const libc::c_int,
    mut num_matches: size_t,
    mut matches: *const BackwardMatch,
    mut model: *const ZopfliCostModel,
    mut queue: *mut StartPosQueue,
    mut nodes: *mut ZopfliNode,
) -> size_t {
    let stream_offset = (*params).stream_offset;
    let cur_ix = block_start.wrapping_add(pos);
    let cur_ix_masked = cur_ix & ringbuffer_mask;
    let max_distance = brotli_min_size_t(cur_ix, max_backward_limit);
    let dictionary_start = brotli_min_size_t(
        cur_ix.wrapping_add(stream_offset),
        max_backward_limit,
    );
    let max_len = num_bytes.wrapping_sub(pos);
    let max_zopfli_len = MaxZopfliLen(params);
    let max_iters = MaxZopfliCandidates(params);
    let mut min_len: size_t = 0;
    let mut result = 0 as libc::c_int as size_t;
    let mut k: size_t = 0;
    let mut gap = 0 as libc::c_int as size_t;
    EvaluateNode(
        block_start.wrapping_add(stream_offset),
        pos,
        max_backward_limit,
        gap,
        starting_dist_cache,
        model,
        queue.as_mut(),
        nodes,
    );
    let mut posdata = StartPosQueueAt(queue, 0 as libc::c_int as size_t);
    let mut min_cost = (*posdata).cost + ZopfliCostModelGetMinCostCmd(model)
        + ZopfliCostModelGetLiteralCosts(model, (*posdata).pos, pos);
    min_len= ComputeMinimumCopyLength(min_cost, nodes, num_bytes, pos);
    k= 0 as libc::c_int as size_t;
    while k < max_iters && k < StartPosQueueSize(queue) {
        let mut posdata_0 = StartPosQueueAt(queue, k);
        let start = (*posdata_0).pos;
        let inscode = GetInsertLengthCode(pos.wrapping_sub(start));
        let start_costdiff = (*posdata_0).costdiff;
        let base_cost = start_costdiff + GetInsertExtra(inscode) as libc::c_float
            + ZopfliCostModelGetLiteralCosts(model, 0 as libc::c_int as size_t, pos);
        let mut best_len = min_len.wrapping_sub(1 as libc::c_int as libc::c_ulong);
        let mut j = 0 as libc::c_int as size_t;
        while j < 16 as libc::c_int as libc::c_ulong && best_len < max_len {
            let idx = crate::src::enc::backward_references_hq::kDistanceCacheIndex[j as usize] as size_t;
            let backward = ((*posdata_0).distance_cache[idx as usize]
                + crate::src::enc::backward_references_hq::kDistanceCacheOffset[j as usize]) as size_t;
            let mut prev_ix = cur_ix.wrapping_sub(backward);
            let mut len = 0 as libc::c_int as size_t;
            let mut continuation = *ringbuffer
                .offset(cur_ix_masked.wrapping_add(best_len) as isize);
            if cur_ix_masked.wrapping_add(best_len) > ringbuffer_mask {
                break;
            }
            if !((backward > dictionary_start.wrapping_add(gap)) as libc::c_int
                as libc::c_long != 0)
            {
                if backward <= max_distance {
                    if !(prev_ix >= cur_ix) {
                        prev_ix&= ringbuffer_mask;
                        if !(prev_ix.wrapping_add(best_len) > ringbuffer_mask
                            || continuation as libc::c_int
                                != *ringbuffer
                                    .offset(prev_ix.wrapping_add(best_len) as isize)
                                    as libc::c_int)
                        {
                            len= FindMatchLengthWithLimit(
                                &*ringbuffer.offset(prev_ix as isize),
                                &*ringbuffer.offset(cur_ix_masked as isize),
                                max_len,
                            );
                            let dist_cost = base_cost
                                + ZopfliCostModelGetDistanceCost(model, j);
                            let mut l: size_t = 0;
                            l= best_len.wrapping_add(1 as libc::c_int as libc::c_ulong);
                            while l <= len {
                                let copycode = GetCopyLengthCode(l);
                                let cmdcode = CombineLengthCodes(
                                    inscode,
                                    copycode,
                                    (j == 0 as libc::c_int as libc::c_ulong) as libc::c_int,
                                );
                                let cost = (if (cmdcode as libc::c_int) < 128 as libc::c_int
                                {
                                    base_cost
                                } else {
                                    dist_cost
                                }) + GetCopyExtra(copycode) as libc::c_float
                                    + ZopfliCostModelGetCommandCost(model, cmdcode);
                                if cost
                                    < (*nodes.offset(pos.wrapping_add(l) as isize)).u.cost
                                {
                                    UpdateZopfliNode(
                                        nodes,
                                        pos,
                                        start,
                                        l,
                                        l,
                                        backward,
                                        j.wrapping_add(1 as libc::c_int as libc::c_ulong),
                                        cost,
                                    );
                                    result= brotli_max_size_t(result, l);
                                }
                                best_len= l;
                                l= l.wrapping_add(1);
                            }
                        }
                    }
                }
            }
            j= j.wrapping_add(1);
        }
        if !(k >= 2 as libc::c_int as libc::c_ulong) {
            let mut len_0 = min_len;
            j= 0 as libc::c_int as size_t;
            while j < num_matches {
                let mut match_0 = *matches.offset(j as isize);
                let mut dist = match_0.distance as size_t;
                let mut is_dictionary_match = if dist
                    > dictionary_start.wrapping_add(gap)
                {
                    1 as libc::c_int
                } else {
                    0 as libc::c_int
                };
                let mut dist_code = dist
                    .wrapping_add(16 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong);
                let mut dist_symbol: uint16_t = 0;
                let mut distextra: uint32_t = 0;
                let mut distnumextra: uint32_t = 0;
                let mut dist_cost_0: libc::c_float = 0.;
                let mut max_match_len: size_t = 0;
                PrefixEncodeCopyDistance(
                    dist_code,
                    (*params).dist.num_direct_distance_codes as size_t,
                    (*params).dist.distance_postfix_bits as size_t,
                    core::ptr::addr_of_mut!(dist_symbol),
                    core::ptr::addr_of_mut!(distextra),
                );
                distnumextra= (dist_symbol as libc::c_int >> 10 as libc::c_int)
                    as uint32_t;
                dist_cost_0= base_cost + distnumextra as libc::c_float
                    + ZopfliCostModelGetDistanceCost(
                        model,
                        (dist_symbol as libc::c_int & 0x3ff as libc::c_int) as size_t,
                    );
                max_match_len= BackwardMatchLength(core::ptr::addr_of!(match_0));
                if len_0 < max_match_len
                    && (is_dictionary_match != 0 || max_match_len > max_zopfli_len)
                {
                    len_0= max_match_len;
                }
                while len_0 <= max_match_len {
                    let len_code = if is_dictionary_match != 0 {
                        BackwardMatchLengthCode(core::ptr::addr_of!(match_0))
                    } else {
                        len_0
                    };
                    let copycode_0 = GetCopyLengthCode(len_code);
                    let cmdcode_0 = CombineLengthCodes(
                        inscode,
                        copycode_0,
                        0 as libc::c_int,
                    );
                    let cost_0 = dist_cost_0 + GetCopyExtra(copycode_0) as libc::c_float
                        + ZopfliCostModelGetCommandCost(model, cmdcode_0);
                    if cost_0 < (*nodes.offset(pos.wrapping_add(len_0) as isize)).u.cost
                    {
                        UpdateZopfliNode(
                            nodes,
                            pos,
                            start,
                            len_0,
                            len_code,
                            dist,
                            0 as libc::c_int as size_t,
                            cost_0,
                        );
                        result= brotli_max_size_t(result, len_0);
                    }
                    len_0= len_0.wrapping_add(1);
                }
                j= j.wrapping_add(1);
            }
        }
        k= k.wrapping_add(1);
    }
    return result;
}
unsafe extern "C" fn ComputeShortestPathFromNodes(
    mut num_bytes: size_t,
    mut nodes: *mut ZopfliNode,
) -> size_t {
    let mut index = num_bytes;
    let mut num_commands = 0 as libc::c_int as size_t;
    while (*nodes.offset(index as isize)).dcode_insert_length
        & 0x7ffffff as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint
        && (*nodes.offset(index as isize)).length == 1 as libc::c_int as libc::c_uint
    {
        index= index.wrapping_sub(1);
    }
    (*nodes.offset(index as isize)).u.next = !(0 as libc::c_int as uint32_t);
    while index != 0 as libc::c_int as libc::c_ulong {
        let mut len = ZopfliNodeCommandLength(core::ptr::addr_of_mut!(*nodes.offset(index as isize)))
            as size_t;
        index= (index as libc::c_ulong).wrapping_sub(len) as size_t as size_t;
        (*nodes.offset(index as isize)).u.next = len as uint32_t;
        num_commands= num_commands.wrapping_add(1);
    }
    return num_commands;
}
#[no_mangle]
pub unsafe extern "C" fn BrotliZopfliCreateCommands(
    mut num_bytes: size_t,
    mut block_start: size_t,
    mut nodes: *const ZopfliNode,
    mut dist_cache: *mut libc::c_int,
    mut last_insert_len: *mut size_t,
    mut params: *const crate::src::enc::backward_references::BrotliEncoderParams,
    mut commands: *mut crate::src::enc::backward_references::Command,
    mut num_literals: *mut size_t,
) {
    let stream_offset = (*params).stream_offset;
    let max_backward_limit = ((1 as libc::c_int as size_t) << (*params).lgwin)
        .wrapping_sub(16 as libc::c_int as libc::c_ulong);
    let mut pos = 0 as libc::c_int as size_t;
    let mut offset = (*nodes.offset(0 as libc::c_int as isize)).u.next;
    let mut i: size_t = 0;
    let mut gap = 0 as libc::c_int as size_t;
    i= 0 as libc::c_int as size_t;
    while offset != !(0 as libc::c_int as uint32_t) {
        let mut next: *const ZopfliNode = &*nodes
            .offset(pos.wrapping_add(offset as libc::c_ulong) as isize)
            as *const ZopfliNode;
        let mut copy_length = ZopfliNodeCopyLength(next) as size_t;
        let mut insert_length = ((*next).dcode_insert_length
            & 0x7ffffff as libc::c_int as libc::c_uint) as size_t;
        pos= (pos as libc::c_ulong).wrapping_add(insert_length) as size_t as size_t;
        offset= (*next).u.next;
        if i == 0 as libc::c_int as libc::c_ulong {
            insert_length= (insert_length as libc::c_ulong)
                .wrapping_add((*last_insert_len)) as size_t as size_t;
            *last_insert_len= 0 as libc::c_int as size_t;
        }
        let mut distance = ZopfliNodeCopyDistance(next) as size_t;
        let mut len_code = ZopfliNodeLengthCode(next) as size_t;
        let mut dictionary_start = brotli_min_size_t(
            block_start.wrapping_add(pos).wrapping_add(stream_offset),
            max_backward_limit,
        );
        let mut is_dictionary = if distance > dictionary_start.wrapping_add(gap) {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        };
        let mut dist_code = ZopfliNodeDistanceCode(next) as size_t;
        InitCommand(
            core::ptr::addr_of_mut!(*commands.offset(i as isize)),
            &(*params).dist,
            insert_length,
            copy_length,
            len_code as libc::c_int - copy_length as libc::c_int,
            dist_code,
        );
        if is_dictionary == 0 && dist_code > 0 as libc::c_int as libc::c_ulong {
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
            *dist_cache.offset(0 as libc::c_int as isize) = distance as libc::c_int;
        }
        *num_literals= ((*num_literals) as libc::c_ulong).wrapping_add(insert_length)
            as size_t as size_t;
        pos= (pos as libc::c_ulong).wrapping_add(copy_length) as size_t as size_t;
        i= i.wrapping_add(1);
    }
    *last_insert_len= ((*last_insert_len) as libc::c_ulong)
        .wrapping_add(num_bytes.wrapping_sub(pos)) as size_t as size_t;
}
unsafe extern "C" fn ZopfliIterate(
    mut num_bytes: size_t,
    mut position: size_t,
    mut ringbuffer: *const uint8_t,
    mut ringbuffer_mask: size_t,
    mut params: *const crate::src::enc::backward_references::BrotliEncoderParams,
    mut gap: size_t,
    mut dist_cache: *const libc::c_int,
    mut model: *const ZopfliCostModel,
    mut num_matches: *const uint32_t,
    mut matches: *const BackwardMatch,
    mut nodes: *mut ZopfliNode,
) -> size_t {
    let stream_offset = (*params).stream_offset;
    let max_backward_limit = ((1 as libc::c_int as size_t) << (*params).lgwin)
        .wrapping_sub(16 as libc::c_int as libc::c_ulong);
    let max_zopfli_len = MaxZopfliLen(params);
    let mut queue = StartPosQueue {
        q_: [PosData {
            pos: 0,
            distance_cache: [0; 4],
            costdiff: 0.,
            cost: 0.,
        }; 8],
        idx_: 0,
    };
    let mut cur_match_pos = 0 as libc::c_int as size_t;
    let mut i: size_t = 0;
    (*nodes.offset(0 as libc::c_int as isize)).length = 0 as libc::c_int as uint32_t;
    (*nodes.offset(0 as libc::c_int as isize))
        .u
        .cost = 0 as libc::c_int as libc::c_float;
    InitStartPosQueue(Some(&mut queue));
    i= 0 as libc::c_int as size_t;
    while i.wrapping_add(3 as libc::c_int as libc::c_ulong) < num_bytes {
        let mut skip = UpdateNodes(
            num_bytes,
            position,
            i,
            ringbuffer,
            ringbuffer_mask,
            params,
            max_backward_limit,
            dist_cache,
            *num_matches.offset(i as isize) as size_t,
            &*matches.offset(cur_match_pos as isize),
            model,
            core::ptr::addr_of_mut!(queue),
            nodes,
        );
        if skip < 16384 as libc::c_int as libc::c_ulong {
            skip= 0 as libc::c_int as size_t;
        }
        cur_match_pos= (cur_match_pos as libc::c_ulong)
            .wrapping_add(*num_matches.offset(i as isize) as libc::c_ulong) as size_t
            as size_t;
        if *num_matches.offset(i as isize) == 1 as libc::c_int as libc::c_uint
            && BackwardMatchLength(
                &*matches
                    .offset(
                        cur_match_pos.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            as isize,
                    ),
            ) > max_zopfli_len
        {
            skip= brotli_max_size_t(
                BackwardMatchLength(
                    &*matches
                        .offset(
                            cur_match_pos.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                as isize,
                        ),
                ),
                skip,
            );
        }
        if skip > 1 as libc::c_int as libc::c_ulong {
            skip= skip.wrapping_sub(1);
            while skip != 0 {
                i= i.wrapping_add(1);
                if i.wrapping_add(3 as libc::c_int as libc::c_ulong) >= num_bytes {
                    break;
                }
                EvaluateNode(
                    position.wrapping_add(stream_offset),
                    i,
                    max_backward_limit,
                    gap,
                    dist_cache,
                    model,
                    Some(&mut queue),
                    nodes,
                );
                cur_match_pos= (cur_match_pos as libc::c_ulong)
                    .wrapping_add(*num_matches.offset(i as isize) as libc::c_ulong)
                    as size_t as size_t;
                skip= skip.wrapping_sub(1);
            }
        }
        i= i.wrapping_add(1);
    }
    return ComputeShortestPathFromNodes(num_bytes, nodes);
}
#[no_mangle]
pub unsafe extern "C" fn BrotliZopfliComputeShortestPath(
    mut m: *mut MemoryManager,
    mut num_bytes: size_t,
    mut position: size_t,
    mut ringbuffer: *const uint8_t,
    mut ringbuffer_mask: size_t,
    mut literal_context_lut: ContextLut,
    mut params: *const crate::src::enc::backward_references::BrotliEncoderParams,
    mut dist_cache: *const libc::c_int,
    mut hasher: *mut crate::src::enc::backward_references::Hasher,
    mut nodes: *mut ZopfliNode,
) -> size_t {
    let stream_offset = (*params).stream_offset;
    let max_backward_limit = ((1 as libc::c_int as size_t) << (*params).lgwin)
        .wrapping_sub(16 as libc::c_int as libc::c_ulong);
    let max_zopfli_len = MaxZopfliLen(params);
    let mut model = ZopfliCostModel {
        cost_cmd_: [0.; 704],
        cost_dist_: 0 as *mut libc::c_float,
        distance_histogram_size: 0,
        literal_costs_: 0 as *mut libc::c_float,
        min_cost_cmd_: 0.,
        num_bytes_: 0,
    };
    let mut queue = StartPosQueue {
        q_: [PosData {
            pos: 0,
            distance_cache: [0; 4],
            costdiff: 0.,
            cost: 0.,
        }; 8],
        idx_: 0,
    };
    let mut matches: [BackwardMatch; 384] = [BackwardMatch {
        distance: 0,
        length_and_code: 0,
    }; 384];
    let store_end = if num_bytes >= StoreLookaheadH10() {
        position
            .wrapping_add(num_bytes)
            .wrapping_sub(StoreLookaheadH10())
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
    } else {
        position
    };
    let mut i: size_t = 0;
    let mut gap = 0 as libc::c_int as size_t;
    let mut lz_matches_offset = 0 as libc::c_int as size_t;
    (*nodes.offset(0 as libc::c_int as isize)).length = 0 as libc::c_int as uint32_t;
    (*nodes.offset(0 as libc::c_int as isize))
        .u
        .cost = 0 as libc::c_int as libc::c_float;
    InitZopfliCostModel(m, core::ptr::addr_of_mut!(model), &(*params).dist, num_bytes);
    if 0 as libc::c_int != 0 {
        return 0 as libc::c_int as size_t;
    }
    ZopfliCostModelSetFromLiteralCosts(
        core::ptr::addr_of_mut!(model),
        position,
        ringbuffer,
        ringbuffer_mask,
    );
    InitStartPosQueue(Some(&mut queue));
    i= 0 as libc::c_int as size_t;
    while i
        .wrapping_add(HashTypeLengthH10())
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) < num_bytes
    {
        let pos = position.wrapping_add(i);
        let max_distance = brotli_min_size_t(pos, max_backward_limit);
        let dictionary_start = brotli_min_size_t(
            pos.wrapping_add(stream_offset),
            max_backward_limit,
        );
        let mut skip: size_t = 0;
        let mut num_matches: size_t = 0;
        num_matches= FindAllMatchesH10(
            core::ptr::addr_of_mut!((*hasher).privat._H10),
            &(*params).dictionary,
            ringbuffer,
            ringbuffer_mask,
            pos,
            num_bytes.wrapping_sub(i),
            max_distance,
            dictionary_start.wrapping_add(gap),
            params,
            core::ptr::addr_of_mut!(*matches.as_mut_ptr().offset(lz_matches_offset as isize)),
        );
        if num_matches > 0 as libc::c_int as libc::c_ulong
            && BackwardMatchLength(
                core::ptr::addr_of_mut!(*matches.as_mut_ptr()
                    .offset(
                        num_matches.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            as isize,
                    )),
            ) > max_zopfli_len
        {
            matches[0 as libc::c_int
                as usize]= matches[num_matches
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as usize];
            num_matches= 1 as libc::c_int as size_t;
        }
        skip= UpdateNodes(
            num_bytes,
            position,
            i,
            ringbuffer,
            ringbuffer_mask,
            params,
            max_backward_limit,
            dist_cache,
            num_matches,
            matches.as_mut_ptr(),
            core::ptr::addr_of!(model),
            core::ptr::addr_of_mut!(queue),
            nodes,
        );
        if skip < 16384 as libc::c_int as libc::c_ulong {
            skip= 0 as libc::c_int as size_t;
        }
        if num_matches == 1 as libc::c_int as libc::c_ulong
            && BackwardMatchLength(
                core::ptr::addr_of_mut!(*matches.as_mut_ptr().offset(0 as libc::c_int as isize)),
            ) > max_zopfli_len
        {
            skip= brotli_max_size_t(
                BackwardMatchLength(
                    core::ptr::addr_of_mut!(*matches.as_mut_ptr().offset(0 as libc::c_int as isize)),
                ),
                skip,
            );
        }
        if skip > 1 as libc::c_int as libc::c_ulong {
            StoreRangeH10(
                core::ptr::addr_of_mut!((*hasher).privat._H10),
                ringbuffer,
                ringbuffer_mask,
                pos.wrapping_add(1 as libc::c_int as libc::c_ulong),
                brotli_min_size_t(pos.wrapping_add(skip), store_end),
            );
            skip= skip.wrapping_sub(1);
            while skip != 0 {
                i= i.wrapping_add(1);
                if i
                    .wrapping_add(HashTypeLengthH10())
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) >= num_bytes
                {
                    break;
                }
                EvaluateNode(
                    position.wrapping_add(stream_offset),
                    i,
                    max_backward_limit,
                    gap,
                    dist_cache,
                    core::ptr::addr_of!(model),
                    Some(&mut queue),
                    nodes,
                );
                skip= skip.wrapping_sub(1);
            }
        }
        i= i.wrapping_add(1);
    }
    CleanupZopfliCostModel(m, core::ptr::addr_of_mut!(model));
    return ComputeShortestPathFromNodes(num_bytes, nodes);
}
#[no_mangle]
pub unsafe extern "C" fn BrotliCreateZopfliBackwardReferences(
    mut m: *mut MemoryManager,
    mut num_bytes: size_t,
    mut position: size_t,
    mut ringbuffer: *const uint8_t,
    mut ringbuffer_mask: size_t,
    mut literal_context_lut: ContextLut,
    mut params: *const crate::src::enc::backward_references::BrotliEncoderParams,
    mut hasher: *mut crate::src::enc::backward_references::Hasher,
    mut dist_cache: *mut libc::c_int,
    mut last_insert_len: *mut size_t,
    mut commands: *mut crate::src::enc::backward_references::Command,
    mut num_commands: *mut size_t,
    mut num_literals: *mut size_t,
) {
    let mut nodes = if num_bytes.wrapping_add(1 as libc::c_int as libc::c_ulong)
        > 0 as libc::c_int as libc::c_ulong
    {
        crate::src::enc::memory::BrotliAllocate(
            m,
            num_bytes
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<ZopfliNode>() as libc::c_ulong),
        ) as *mut ZopfliNode
    } else {
        0 as *mut ZopfliNode
    };
    if 0 as libc::c_int != 0 || 0 as libc::c_int != 0 {
        return;
    }
    BrotliInitZopfliNodes(
        nodes,
        num_bytes.wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    *num_commands= ((*num_commands) as libc::c_ulong)
        .wrapping_add(
            BrotliZopfliComputeShortestPath(
                m,
                num_bytes,
                position,
                ringbuffer,
                ringbuffer_mask,
                literal_context_lut,
                params,
                dist_cache,
                hasher,
                nodes,
            ),
        ) as size_t as size_t;
    if 0 as libc::c_int != 0 {
        return;
    }
    BrotliZopfliCreateCommands(
        num_bytes,
        position,
        nodes,
        dist_cache,
        last_insert_len,
        params,
        commands,
        num_literals,
    );
    crate::src::enc::memory::BrotliFree(m, nodes as *mut libc::c_void);
    nodes= 0 as *mut ZopfliNode;
}
#[no_mangle]
pub unsafe extern "C" fn BrotliCreateHqZopfliBackwardReferences(
    mut m: *mut MemoryManager,
    mut num_bytes: size_t,
    mut position: size_t,
    mut ringbuffer: *const uint8_t,
    mut ringbuffer_mask: size_t,
    mut literal_context_lut: ContextLut,
    mut params: *const crate::src::enc::backward_references::BrotliEncoderParams,
    mut hasher: *mut crate::src::enc::backward_references::Hasher,
    mut dist_cache: *mut libc::c_int,
    mut last_insert_len: *mut size_t,
    mut commands: *mut crate::src::enc::backward_references::Command,
    mut num_commands: *mut size_t,
    mut num_literals: *mut size_t,
) {
    let stream_offset = (*params).stream_offset;
    let max_backward_limit = ((1 as libc::c_int as size_t) << (*params).lgwin)
        .wrapping_sub(16 as libc::c_int as libc::c_ulong);
    let mut num_matches = if num_bytes > 0 as libc::c_int as libc::c_ulong {
        crate::src::enc::memory::BrotliAllocate(
            m,
            num_bytes.wrapping_mul(::std::mem::size_of::<uint32_t>() as libc::c_ulong),
        ) as *mut uint32_t
    } else {
        0 as *mut uint32_t
    };
    let mut matches_size = (4 as libc::c_int as libc::c_ulong).wrapping_mul(num_bytes);
    let store_end = if num_bytes >= StoreLookaheadH10() {
        position
            .wrapping_add(num_bytes)
            .wrapping_sub(StoreLookaheadH10())
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
    } else {
        position
    };
    let mut cur_match_pos = 0 as libc::c_int as size_t;
    let mut i: size_t = 0;
    let mut orig_num_literals: size_t = 0;
    let mut orig_last_insert_len: size_t = 0;
    let mut orig_dist_cache: [libc::c_int; 4] = [0; 4];
    let mut orig_num_commands: size_t = 0;
    let mut model = ZopfliCostModel {
        cost_cmd_: [0.; 704],
        cost_dist_: 0 as *mut libc::c_float,
        distance_histogram_size: 0,
        literal_costs_: 0 as *mut libc::c_float,
        min_cost_cmd_: 0.,
        num_bytes_: 0,
    };
    let mut nodes = 0 as *mut ZopfliNode;
    let mut matches = if matches_size > 0 as libc::c_int as libc::c_ulong {
        crate::src::enc::memory::BrotliAllocate(
            m,
            matches_size
                .wrapping_mul(::std::mem::size_of::<BackwardMatch>() as libc::c_ulong),
        ) as *mut BackwardMatch
    } else {
        0 as *mut BackwardMatch
    };
    let mut gap = 0 as libc::c_int as size_t;
    let mut shadow_matches = 0 as libc::c_int as size_t;
    if 0 as libc::c_int != 0 || 0 as libc::c_int != 0 || 0 as libc::c_int != 0 {
        return;
    }
    i= 0 as libc::c_int as size_t;
    while i
        .wrapping_add(HashTypeLengthH10())
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) < num_bytes
    {
        let pos = position.wrapping_add(i);
        let mut max_distance = brotli_min_size_t(pos, max_backward_limit);
        let mut dictionary_start = brotli_min_size_t(
            pos.wrapping_add(stream_offset),
            max_backward_limit,
        );
        let mut max_length = num_bytes.wrapping_sub(i);
        let mut num_found_matches: size_t = 0;
        let mut cur_match_end: size_t = 0;
        let mut j: size_t = 0;
        if matches_size
            < cur_match_pos
                .wrapping_add(128 as libc::c_int as libc::c_ulong)
                .wrapping_add(shadow_matches)
        {
            let mut _new_size = if matches_size == 0 as libc::c_int as libc::c_ulong {
                cur_match_pos
                    .wrapping_add(128 as libc::c_int as libc::c_ulong)
                    .wrapping_add(shadow_matches)
            } else {
                matches_size
            };
            let mut new_array = 0 as *mut BackwardMatch;
            while _new_size
                < cur_match_pos
                    .wrapping_add(128 as libc::c_int as libc::c_ulong)
                    .wrapping_add(shadow_matches)
            {
                _new_size= (_new_size as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
            }
            new_array= if _new_size > 0 as libc::c_int as libc::c_ulong {
                crate::src::enc::memory::BrotliAllocate(
                    m,
                    _new_size
                        .wrapping_mul(
                            ::std::mem::size_of::<BackwardMatch>() as libc::c_ulong,
                        ),
                ) as *mut BackwardMatch
            } else {
                0 as *mut BackwardMatch
            };
            if 0 as libc::c_int == 0 && 0 as libc::c_int == 0
                && matches_size != 0 as libc::c_int as libc::c_ulong
            {
                memcpy(
                    new_array as *mut libc::c_void,
                    matches as *const libc::c_void,
                    matches_size
                        .wrapping_mul(
                            ::std::mem::size_of::<BackwardMatch>() as libc::c_ulong,
                        ),
                );
            }
            crate::src::enc::memory::BrotliFree(m, matches as *mut libc::c_void);
            matches= 0 as *mut BackwardMatch;
            matches= new_array;
            matches_size= _new_size;
        }
        if 0 as libc::c_int != 0 {
            return;
        }
        num_found_matches= FindAllMatchesH10(
            core::ptr::addr_of_mut!((*hasher).privat._H10),
            &(*params).dictionary,
            ringbuffer,
            ringbuffer_mask,
            pos,
            max_length,
            max_distance,
            dictionary_start.wrapping_add(gap),
            params,
            core::ptr::addr_of_mut!(*matches.offset(cur_match_pos.wrapping_add(shadow_matches) as isize)),
        );
        cur_match_end= cur_match_pos.wrapping_add(num_found_matches);
        j= cur_match_pos;
        while j.wrapping_add(1 as libc::c_int as libc::c_ulong) < cur_match_end {
            j= j.wrapping_add(1);
        }
        *num_matches.offset(i as isize) = num_found_matches as uint32_t;
        if num_found_matches > 0 as libc::c_int as libc::c_ulong {
            let match_len = BackwardMatchLength(
                core::ptr::addr_of_mut!(*matches
                    .offset(
                        cur_match_end.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            as isize,
                    )),
            );
            if match_len > 325 as libc::c_int as libc::c_ulong {
                let skip = match_len.wrapping_sub(1 as libc::c_int as libc::c_ulong);
                let fresh11 = cur_match_pos;
                cur_match_pos= cur_match_pos.wrapping_add(1);
                *matches
                    .offset(
                        fresh11 as isize,
                    ) = *matches
                    .offset(
                        cur_match_end.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            as isize,
                    );
                *num_matches.offset(i as isize) = 1 as libc::c_int as uint32_t;
                StoreRangeH10(
                    core::ptr::addr_of_mut!((*hasher).privat._H10),
                    ringbuffer,
                    ringbuffer_mask,
                    pos.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    brotli_min_size_t(pos.wrapping_add(match_len), store_end),
                );
                memset(
                    core::ptr::addr_of_mut!(*num_matches
                        .offset(
                            i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )) as *mut uint32_t as *mut libc::c_void,
                    0 as libc::c_int,
                    skip.wrapping_mul(::std::mem::size_of::<uint32_t>() as libc::c_ulong),
                );
                i= (i as libc::c_ulong).wrapping_add(skip) as size_t as size_t;
            } else {
                cur_match_pos= cur_match_end;
            }
        }
        i= i.wrapping_add(1);
    }
    orig_num_literals= (*num_literals);
    orig_last_insert_len= (*last_insert_len);
    memcpy(
        orig_dist_cache.as_mut_ptr() as *mut libc::c_void,
        dist_cache as *const libc::c_void,
        (4 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    orig_num_commands= (*num_commands);
    nodes= if num_bytes.wrapping_add(1 as libc::c_int as libc::c_ulong)
        > 0 as libc::c_int as libc::c_ulong
    {
        crate::src::enc::memory::BrotliAllocate(
            m,
            num_bytes
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<ZopfliNode>() as libc::c_ulong),
        ) as *mut ZopfliNode
    } else {
        0 as *mut ZopfliNode
    };
    if 0 as libc::c_int != 0 || 0 as libc::c_int != 0 {
        return;
    }
    InitZopfliCostModel(m, core::ptr::addr_of_mut!(model), &(*params).dist, num_bytes);
    if 0 as libc::c_int != 0 {
        return;
    }
    i= 0 as libc::c_int as size_t;
    while i < 2 as libc::c_int as libc::c_ulong {
        BrotliInitZopfliNodes(
            nodes,
            num_bytes.wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
        if i == 0 as libc::c_int as libc::c_ulong {
            ZopfliCostModelSetFromLiteralCosts(
                core::ptr::addr_of_mut!(model),
                position,
                ringbuffer,
                ringbuffer_mask,
            );
        } else {
            ZopfliCostModelSetFromCommands(
                core::ptr::addr_of_mut!(model),
                position,
                ringbuffer,
                ringbuffer_mask,
                commands,
                (*num_commands).wrapping_sub(orig_num_commands),
                orig_last_insert_len,
            );
        }
        *num_commands= orig_num_commands;
        *num_literals= orig_num_literals;
        *last_insert_len= orig_last_insert_len;
        memcpy(
            dist_cache as *mut libc::c_void,
            orig_dist_cache.as_mut_ptr() as *const libc::c_void,
            (4 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
        *num_commands= ((*num_commands) as libc::c_ulong)
            .wrapping_add(
                ZopfliIterate(
                    num_bytes,
                    position,
                    ringbuffer,
                    ringbuffer_mask,
                    params,
                    gap,
                    dist_cache,
                    core::ptr::addr_of!(model),
                    num_matches,
                    matches,
                    nodes,
                ),
            ) as size_t as size_t;
        BrotliZopfliCreateCommands(
            num_bytes,
            position,
            nodes,
            dist_cache,
            last_insert_len,
            params,
            commands,
            num_literals,
        );
        i= i.wrapping_add(1);
    }
    CleanupZopfliCostModel(m, core::ptr::addr_of_mut!(model));
    crate::src::enc::memory::BrotliFree(m, nodes as *mut libc::c_void);
    nodes= 0 as *mut ZopfliNode;
    crate::src::enc::memory::BrotliFree(m, matches as *mut libc::c_void);
    matches= 0 as *mut BackwardMatch;
    crate::src::enc::memory::BrotliFree(m, num_matches as *mut libc::c_void);
    num_matches= 0 as *mut uint32_t;
}
