use ::libc;
extern "C" {
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
    fn log2(_: libc::c_double) -> libc::c_double;
    static kBrotliLog2Table: [libc::c_double; 256];
    
    
    
    
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type int16_t = __int16_t;
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
#[derive(Copy, Clone)]

struct ErasedByPreprocessor73 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor74 { dummy: () }
#[inline(always)]
unsafe extern "C" fn BrotliUnalignedRead32(mut p: *const libc::c_void) -> uint32_t {
    return *(p as *const uint32_t);
}
#[inline(always)]
unsafe extern "C" fn BrotliUnalignedRead64(mut p: *const libc::c_void) -> uint64_t {
    return *(p as *const uint64_t);
}
#[inline(always)]
unsafe extern "C" fn BrotliUnalignedWrite64(mut p: *mut libc::c_void, mut v: uint64_t) {
    *(p as *mut uint64_t) = v;
}
#[inline(always)]
unsafe extern "C" fn brotli_min_size_t(mut a: size_t, mut b: size_t) -> size_t {
    return if a < b { a } else { b };
}
static mut kCompressFragmentTwoPassBlockSize: size_t = ((1 as libc::c_int)
    << 17 as libc::c_int) as size_t;
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
        current_block= 18167736425092750904;
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
                current_block= 18167736425092750904;
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
unsafe extern "C" fn Log2FloorNonZero(mut n: size_t) -> uint32_t {
    return 31 as libc::c_uint ^ (n as uint32_t).leading_zeros() as i32 as uint32_t;
}
#[inline(always)]
unsafe extern "C" fn FastLog2(mut v: size_t) -> libc::c_double {
    if v < 256 as libc::c_int as libc::c_ulong {
        return crate::src::enc::compress_fragment_two_pass::kBrotliLog2Table[v as usize];
    }
    return log2(v as libc::c_double);
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
unsafe extern "C" fn BrotliWriteBits(
    mut n_bits: size_t,
    mut bits: uint64_t,
    mut pos: *mut size_t,
    mut array: *mut uint8_t,
) {
    let mut p: *mut uint8_t = core::ptr::addr_of_mut!(*array.offset(((*pos) >> 3 as libc::c_int) as isize))
        as *mut uint8_t;
    let mut v = (*p) as uint64_t;
    v|= bits << ((*pos) & 7 as libc::c_int as libc::c_ulong);
    BrotliUnalignedWrite64(p as *mut libc::c_void, v);
    *pos= ((*pos) as libc::c_ulong).wrapping_add(n_bits) as size_t as size_t;
}
static mut kHashMul32: uint32_t = 0x1e35a7bd as libc::c_int as uint32_t;
#[inline(always)]
unsafe extern "C" fn Hash(
    mut p: *const uint8_t,
    mut shift: size_t,
    mut length: size_t,
) -> uint32_t {
    let h = (BrotliUnalignedRead64(p as *const libc::c_void)
        << (8 as libc::c_int as libc::c_ulong)
            .wrapping_sub(length)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong))
        .wrapping_mul(crate::src::enc::compress_fragment_two_pass::kHashMul32 as libc::c_ulong);
    return (h >> shift) as uint32_t;
}
#[inline(always)]
unsafe extern "C" fn HashBytesAtOffset(
    mut v: uint64_t,
    mut offset: size_t,
    mut shift: size_t,
    mut length: size_t,
) -> uint32_t {
    let h = (v >> (8 as libc::c_int as libc::c_ulong).wrapping_mul(offset)
        << (8 as libc::c_int as libc::c_ulong)
            .wrapping_sub(length)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong))
        .wrapping_mul(crate::src::enc::compress_fragment_two_pass::kHashMul32 as libc::c_ulong);
    return (h >> shift) as uint32_t;
}
#[inline(always)]
unsafe extern "C" fn IsMatch(
    mut p1: *const uint8_t,
    mut p2: *const uint8_t,
    mut length: size_t,
) -> libc::c_int {
    if BrotliUnalignedRead32(p1 as *const libc::c_void)
        == BrotliUnalignedRead32(p2 as *const libc::c_void)
    {
        if length == 4 as libc::c_int as libc::c_ulong {
            return 1 as libc::c_int;
        }
        return if *p1.offset(4 as libc::c_int as isize) as libc::c_int
            == *p2.offset(4 as libc::c_int as isize) as libc::c_int
            && *p1.offset(5 as libc::c_int as isize) as libc::c_int
                == *p2.offset(5 as libc::c_int as isize) as libc::c_int
        {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        };
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn BuildAndStoreCommandPrefixCode(
    mut histogram: *const uint32_t,
    mut depth: *mut uint8_t,
    mut bits: *mut uint16_t,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let mut tree: [crate::src::enc::brotli_bit_stream::HuffmanTree; 129] = [crate::src::enc::brotli_bit_stream::HuffmanTree {
        total_count_: 0,
        index_left_: 0,
        index_right_or_value_: 0,
    }; 129];
    let mut cmd_depth: [uint8_t; 704] = [
        0 as libc::c_int as uint8_t,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
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
    let mut cmd_bits: [uint16_t; 64] = [0; 64];
    crate::src::enc::entropy_encode::BrotliCreateHuffmanTree(
        histogram,
        64 as libc::c_int as size_t,
        15 as libc::c_int,
        tree.as_mut_ptr(),
        depth,
    );
    crate::src::enc::entropy_encode::BrotliCreateHuffmanTree(
        &*histogram.offset(64 as libc::c_int as isize),
        64 as libc::c_int as size_t,
        14 as libc::c_int,
        tree.as_mut_ptr(),
        core::ptr::addr_of_mut!(*depth.offset(64 as libc::c_int as isize)),
    );
    memcpy(
        cmd_depth.as_mut_ptr() as *mut libc::c_void,
        depth.offset(24 as libc::c_int as isize) as *const libc::c_void,
        24 as libc::c_int as libc::c_ulong,
    );
    memcpy(
        cmd_depth.as_mut_ptr().offset(24 as libc::c_int as isize) as *mut libc::c_void,
        depth as *const libc::c_void,
        8 as libc::c_int as libc::c_ulong,
    );
    memcpy(
        cmd_depth.as_mut_ptr().offset(32 as libc::c_int as isize) as *mut libc::c_void,
        depth.offset(48 as libc::c_int as isize) as *const libc::c_void,
        8 as libc::c_int as libc::c_ulong,
    );
    memcpy(
        cmd_depth.as_mut_ptr().offset(40 as libc::c_int as isize) as *mut libc::c_void,
        depth.offset(8 as libc::c_int as isize) as *const libc::c_void,
        8 as libc::c_int as libc::c_ulong,
    );
    memcpy(
        cmd_depth.as_mut_ptr().offset(48 as libc::c_int as isize) as *mut libc::c_void,
        depth.offset(56 as libc::c_int as isize) as *const libc::c_void,
        8 as libc::c_int as libc::c_ulong,
    );
    memcpy(
        cmd_depth.as_mut_ptr().offset(56 as libc::c_int as isize) as *mut libc::c_void,
        depth.offset(16 as libc::c_int as isize) as *const libc::c_void,
        8 as libc::c_int as libc::c_ulong,
    );
    crate::src::enc::entropy_encode::BrotliConvertBitDepthsToSymbols(
        cmd_depth.as_mut_ptr(),
        64 as libc::c_int as size_t,
        cmd_bits.as_mut_ptr(),
    );
    memcpy(
        bits as *mut libc::c_void,
        cmd_bits.as_mut_ptr().offset(24 as libc::c_int as isize) as *const libc::c_void,
        16 as libc::c_int as libc::c_ulong,
    );
    memcpy(
        bits.offset(8 as libc::c_int as isize) as *mut libc::c_void,
        cmd_bits.as_mut_ptr().offset(40 as libc::c_int as isize) as *const libc::c_void,
        16 as libc::c_int as libc::c_ulong,
    );
    memcpy(
        bits.offset(16 as libc::c_int as isize) as *mut libc::c_void,
        cmd_bits.as_mut_ptr().offset(56 as libc::c_int as isize) as *const libc::c_void,
        16 as libc::c_int as libc::c_ulong,
    );
    memcpy(
        bits.offset(24 as libc::c_int as isize) as *mut libc::c_void,
        cmd_bits.as_mut_ptr() as *const libc::c_void,
        48 as libc::c_int as libc::c_ulong,
    );
    memcpy(
        bits.offset(48 as libc::c_int as isize) as *mut libc::c_void,
        cmd_bits.as_mut_ptr().offset(32 as libc::c_int as isize) as *const libc::c_void,
        16 as libc::c_int as libc::c_ulong,
    );
    memcpy(
        bits.offset(56 as libc::c_int as isize) as *mut libc::c_void,
        cmd_bits.as_mut_ptr().offset(48 as libc::c_int as isize) as *const libc::c_void,
        16 as libc::c_int as libc::c_ulong,
    );
    crate::src::enc::entropy_encode::BrotliConvertBitDepthsToSymbols(
        core::ptr::addr_of_mut!(*depth.offset(64 as libc::c_int as isize)),
        64 as libc::c_int as size_t,
        core::ptr::addr_of_mut!(*bits.offset(64 as libc::c_int as isize)),
    );
    let mut i: size_t = 0;
    memset(
        cmd_depth.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        64 as libc::c_int as libc::c_ulong,
    );
    memcpy(
        cmd_depth.as_mut_ptr() as *mut libc::c_void,
        depth.offset(24 as libc::c_int as isize) as *const libc::c_void,
        8 as libc::c_int as libc::c_ulong,
    );
    memcpy(
        cmd_depth.as_mut_ptr().offset(64 as libc::c_int as isize) as *mut libc::c_void,
        depth.offset(32 as libc::c_int as isize) as *const libc::c_void,
        8 as libc::c_int as libc::c_ulong,
    );
    memcpy(
        cmd_depth.as_mut_ptr().offset(128 as libc::c_int as isize) as *mut libc::c_void,
        depth.offset(40 as libc::c_int as isize) as *const libc::c_void,
        8 as libc::c_int as libc::c_ulong,
    );
    memcpy(
        cmd_depth.as_mut_ptr().offset(192 as libc::c_int as isize) as *mut libc::c_void,
        depth.offset(48 as libc::c_int as isize) as *const libc::c_void,
        8 as libc::c_int as libc::c_ulong,
    );
    memcpy(
        cmd_depth.as_mut_ptr().offset(384 as libc::c_int as isize) as *mut libc::c_void,
        depth.offset(56 as libc::c_int as isize) as *const libc::c_void,
        8 as libc::c_int as libc::c_ulong,
    );
    i= 0 as libc::c_int as size_t;
    while i < 8 as libc::c_int as libc::c_ulong {
        cmd_depth[(128 as libc::c_int as libc::c_ulong)
            .wrapping_add((8 as libc::c_int as libc::c_ulong).wrapping_mul(i))
            as usize]= *depth.offset(i as isize);
        cmd_depth[(256 as libc::c_int as libc::c_ulong)
            .wrapping_add((8 as libc::c_int as libc::c_ulong).wrapping_mul(i))
            as usize]= *depth
            .offset((8 as libc::c_int as libc::c_ulong).wrapping_add(i) as isize);
        cmd_depth[(448 as libc::c_int as libc::c_ulong)
            .wrapping_add((8 as libc::c_int as libc::c_ulong).wrapping_mul(i))
            as usize]= *depth
            .offset((16 as libc::c_int as libc::c_ulong).wrapping_add(i) as isize);
        i= i.wrapping_add(1);
    }
    crate::src::enc::brotli_bit_stream::BrotliStoreHuffmanTree(
        cmd_depth.as_mut_ptr(),
        704 as libc::c_int as size_t,
        tree.as_mut_ptr(),
        storage_ix,
        storage,
    );
    crate::src::enc::brotli_bit_stream::BrotliStoreHuffmanTree(
        core::ptr::addr_of_mut!(*depth.offset(64 as libc::c_int as isize)),
        64 as libc::c_int as size_t,
        tree.as_mut_ptr(),
        storage_ix,
        storage,
    );
}
#[inline(always)]
unsafe extern "C" fn EmitInsertLen(
    mut insertlen: uint32_t,
    mut commands: Option<&mut *mut uint32_t>,
) {
    if insertlen < 6 as libc::c_int as libc::c_uint {
        *(*commands.as_deref_mut().unwrap())= insertlen;
    } else if insertlen < 130 as libc::c_int as libc::c_uint {
        let tail = insertlen.wrapping_sub(2 as libc::c_int as libc::c_uint);
        let nbits = (Log2FloorNonZero(tail as size_t)).wrapping_sub(1 as libc::c_uint);
        let prefix = tail >> nbits;
        let inscode = (nbits << 1 as libc::c_int)
            .wrapping_add(prefix)
            .wrapping_add(2 as libc::c_int as libc::c_uint);
        let extra = tail.wrapping_sub(prefix << nbits);
        *(*commands.as_deref_mut().unwrap())= inscode | extra << 8 as libc::c_int;
    } else if insertlen < 2114 as libc::c_int as libc::c_uint {
        let tail_0 = insertlen.wrapping_sub(66 as libc::c_int as libc::c_uint);
        let nbits_0 = Log2FloorNonZero(tail_0 as size_t);
        let code = nbits_0.wrapping_add(10 as libc::c_int as libc::c_uint);
        let extra_0 = tail_0.wrapping_sub((1 as libc::c_uint) << nbits_0);
        *(*commands.as_deref_mut().unwrap())= code | extra_0 << 8 as libc::c_int;
    } else if insertlen < 6210 as libc::c_int as libc::c_uint {
        let extra_1 = insertlen.wrapping_sub(2114 as libc::c_int as libc::c_uint);
        *(*commands.as_deref_mut().unwrap())= 21 as libc::c_int as libc::c_uint | extra_1 << 8 as libc::c_int;
    } else if insertlen < 22594 as libc::c_int as libc::c_uint {
        let extra_2 = insertlen.wrapping_sub(6210 as libc::c_int as libc::c_uint);
        *(*commands.as_deref_mut().unwrap())= 22 as libc::c_int as libc::c_uint | extra_2 << 8 as libc::c_int;
    } else {
        let extra_3 = insertlen.wrapping_sub(22594 as libc::c_int as libc::c_uint);
        *(*commands.as_deref_mut().unwrap())= 23 as libc::c_int as libc::c_uint | extra_3 << 8 as libc::c_int;
    }
    *commands.as_deref_mut().unwrap()= (*commands.as_deref().unwrap()).offset(1);
}
#[inline(always)]
unsafe extern "C" fn EmitCopyLen(mut copylen: size_t, mut commands: Option<&mut *mut uint32_t>) {
    if copylen < 10 as libc::c_int as libc::c_ulong {
        *(*commands.as_deref_mut().unwrap())= copylen.wrapping_add(38 as libc::c_int as libc::c_ulong)
            as uint32_t;
    } else if copylen < 134 as libc::c_int as libc::c_ulong {
        let tail = copylen.wrapping_sub(6 as libc::c_int as libc::c_ulong);
        let nbits = (Log2FloorNonZero(tail))
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t;
        let prefix = tail >> nbits;
        let code = (nbits << 1 as libc::c_int)
            .wrapping_add(prefix)
            .wrapping_add(44 as libc::c_int as libc::c_ulong);
        let extra = tail.wrapping_sub(prefix << nbits);
        *(*commands.as_deref_mut().unwrap())= (code | extra << 8 as libc::c_int) as uint32_t;
    } else if copylen < 2118 as libc::c_int as libc::c_ulong {
        let tail_0 = copylen.wrapping_sub(70 as libc::c_int as libc::c_ulong);
        let nbits_0 = Log2FloorNonZero(tail_0) as size_t;
        let code_0 = nbits_0.wrapping_add(52 as libc::c_int as libc::c_ulong);
        let extra_0 = tail_0.wrapping_sub((1 as libc::c_int as size_t) << nbits_0);
        *(*commands.as_deref_mut().unwrap())= (code_0 | extra_0 << 8 as libc::c_int) as uint32_t;
    } else {
        let extra_1 = copylen.wrapping_sub(2118 as libc::c_int as libc::c_ulong);
        *(*commands.as_deref_mut().unwrap())= (63 as libc::c_int as libc::c_ulong | extra_1 << 8 as libc::c_int)
            as uint32_t;
    }
    *commands.as_deref_mut().unwrap()= (*commands.as_deref().unwrap()).offset(1);
}
#[inline(always)]
unsafe extern "C" fn EmitCopyLenLastDistance(
    mut copylen: size_t,
    mut commands: Option<&mut *mut uint32_t>,
) {
    if copylen < 12 as libc::c_int as libc::c_ulong {
        *(*commands.as_deref_mut().unwrap())= copylen.wrapping_add(20 as libc::c_int as libc::c_ulong)
            as uint32_t;
        *commands.as_deref_mut().unwrap()= (*commands.as_deref().unwrap()).offset(1);
    } else if copylen < 72 as libc::c_int as libc::c_ulong {
        let tail = copylen.wrapping_sub(8 as libc::c_int as libc::c_ulong);
        let nbits = (Log2FloorNonZero(tail))
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t;
        let prefix = tail >> nbits;
        let code = (nbits << 1 as libc::c_int)
            .wrapping_add(prefix)
            .wrapping_add(28 as libc::c_int as libc::c_ulong);
        let extra = tail.wrapping_sub(prefix << nbits);
        *(*commands.as_deref_mut().unwrap())= (code | extra << 8 as libc::c_int) as uint32_t;
        *commands.as_deref_mut().unwrap()= (*commands.as_deref().unwrap()).offset(1);
    } else if copylen < 136 as libc::c_int as libc::c_ulong {
        let tail_0 = copylen.wrapping_sub(8 as libc::c_int as libc::c_ulong);
        let code_0 = (tail_0 >> 5 as libc::c_int)
            .wrapping_add(54 as libc::c_int as libc::c_ulong);
        let extra_0 = tail_0 & 31 as libc::c_int as libc::c_ulong;
        *(*commands.as_deref_mut().unwrap())= (code_0 | extra_0 << 8 as libc::c_int) as uint32_t;
        *commands.as_deref_mut().unwrap()= (*commands.as_deref().unwrap()).offset(1);
        *(*commands.as_deref_mut().unwrap())= 64 as libc::c_int as uint32_t;
        *commands.as_deref_mut().unwrap()= (*commands.as_deref().unwrap()).offset(1);
    } else if copylen < 2120 as libc::c_int as libc::c_ulong {
        let tail_1 = copylen.wrapping_sub(72 as libc::c_int as libc::c_ulong);
        let nbits_0 = Log2FloorNonZero(tail_1) as size_t;
        let code_1 = nbits_0.wrapping_add(52 as libc::c_int as libc::c_ulong);
        let extra_1 = tail_1.wrapping_sub((1 as libc::c_int as size_t) << nbits_0);
        *(*commands.as_deref_mut().unwrap())= (code_1 | extra_1 << 8 as libc::c_int) as uint32_t;
        *commands.as_deref_mut().unwrap()= (*commands.as_deref().unwrap()).offset(1);
        *(*commands.as_deref_mut().unwrap())= 64 as libc::c_int as uint32_t;
        *commands.as_deref_mut().unwrap()= (*commands.as_deref().unwrap()).offset(1);
    } else {
        let extra_2 = copylen.wrapping_sub(2120 as libc::c_int as libc::c_ulong);
        *(*commands.as_deref_mut().unwrap())= (63 as libc::c_int as libc::c_ulong | extra_2 << 8 as libc::c_int)
            as uint32_t;
        *commands.as_deref_mut().unwrap()= (*commands.as_deref().unwrap()).offset(1);
        *(*commands.as_deref_mut().unwrap())= 64 as libc::c_int as uint32_t;
        *commands.as_deref_mut().unwrap()= (*commands.as_deref().unwrap()).offset(1);
    };
}
#[inline(always)]
unsafe extern "C" fn EmitDistance(
    mut distance: uint32_t,
    mut commands: Option<&mut *mut uint32_t>,
) {
    let mut d = distance.wrapping_add(3 as libc::c_int as libc::c_uint);
    let mut nbits = (Log2FloorNonZero(d as size_t))
        .wrapping_sub(1 as libc::c_int as libc::c_uint);
    let prefix = d >> nbits & 1 as libc::c_int as libc::c_uint;
    let offset = (2 as libc::c_int as libc::c_uint).wrapping_add(prefix) << nbits;
    let distcode = (2 as libc::c_int as libc::c_uint)
        .wrapping_mul(nbits.wrapping_sub(1 as libc::c_int as libc::c_uint))
        .wrapping_add(prefix)
        .wrapping_add(80 as libc::c_int as libc::c_uint);
    let mut extra = d.wrapping_sub(offset);
    *(*commands.as_deref_mut().unwrap())= distcode | extra << 8 as libc::c_int;
    *commands.as_deref_mut().unwrap()= (*commands.as_deref().unwrap()).offset(1);
}
unsafe extern "C" fn BrotliStoreMetaBlockHeader(
    mut len: size_t,
    mut is_uncompressed: libc::c_int,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let mut nibbles = 6 as libc::c_int as size_t;
    BrotliWriteBits(
        1 as libc::c_int as size_t,
        0 as libc::c_int as uint64_t,
        storage_ix,
        storage,
    );
    if len <= ((1 as libc::c_uint) << 16 as libc::c_int) as libc::c_ulong {
        nibbles= 4 as libc::c_int as size_t;
    } else if len <= ((1 as libc::c_uint) << 20 as libc::c_int) as libc::c_ulong {
        nibbles= 5 as libc::c_int as size_t;
    }
    BrotliWriteBits(
        2 as libc::c_int as size_t,
        nibbles.wrapping_sub(4 as libc::c_int as libc::c_ulong),
        storage_ix,
        storage,
    );
    BrotliWriteBits(
        nibbles.wrapping_mul(4 as libc::c_int as libc::c_ulong),
        len.wrapping_sub(1 as libc::c_int as libc::c_ulong),
        storage_ix,
        storage,
    );
    BrotliWriteBits(
        1 as libc::c_int as size_t,
        is_uncompressed as uint64_t,
        storage_ix,
        storage,
    );
}
#[inline(always)]
unsafe extern "C" fn CreateCommands(
    mut input: *const uint8_t,
    mut block_size: size_t,
    mut input_size: size_t,
    mut base_ip: *const uint8_t,
    mut table: *mut libc::c_int,
    mut table_bits: size_t,
    mut min_match: size_t,
    mut literals: Option<&mut *mut uint8_t>,
    mut commands: Option<&mut *mut uint32_t>,
) {
    let mut current_block: u64;
    let mut ip = input;
    let shift = (64 as libc::c_uint as libc::c_ulong).wrapping_sub(table_bits);
    let mut ip_end = input.offset(block_size as isize);
    let mut next_emit = input;
    let mut last_distance = -(1 as libc::c_int);
    let kInputMarginBytes = 16 as libc::c_int as size_t;
    if (block_size >= kInputMarginBytes) as libc::c_int as libc::c_long != 0 {
        let len_limit = brotli_min_size_t(
            block_size.wrapping_sub(min_match),
            input_size.wrapping_sub(kInputMarginBytes),
        );
        let mut ip_limit = input.offset(len_limit as isize);
        let mut next_hash: uint32_t = 0;
        ip= ip.offset(1);
        next_hash= Hash(ip, shift, min_match);
        's_41: loop {
            let mut skip = 32 as libc::c_int as uint32_t;
            let mut next_ip = ip;
            let mut candidate = 0 as *const uint8_t;
            loop {
                let mut hash = next_hash;
                let fresh2 = skip;
                skip= skip.wrapping_add(1);
                let mut bytes_between_hash_lookups = fresh2 >> 5 as libc::c_int;
                ip= next_ip;
                next_ip= ip.offset(bytes_between_hash_lookups as isize);
                if (next_ip > ip_limit) as libc::c_int as libc::c_long != 0 {
                    break 's_41;
                }
                next_hash= Hash(next_ip, shift, min_match);
                candidate= ip.offset(-(last_distance as isize));
                if IsMatch(ip, candidate, min_match) != 0 {
                    if (candidate < ip) as libc::c_int as libc::c_long != 0 {
                        *table
                            .offset(
                                hash as isize,
                            ) = ip.offset_from(base_ip) as libc::c_long as libc::c_int;
                        current_block= 14818589718467733107;
                    } else {
                        current_block= 1109700713171191020;
                    }
                } else {
                    current_block= 1109700713171191020;
                }
                match current_block {
                    1109700713171191020 => {
                        candidate= base_ip
                            .offset(*table.offset(hash as isize) as isize);
                        *table
                            .offset(
                                hash as isize,
                            ) = ip.offset_from(base_ip) as libc::c_long as libc::c_int;
                        if (IsMatch(ip, candidate, min_match) == 0) as libc::c_int
                            as libc::c_long != 0
                        {
                            continue;
                        }
                    }
                    _ => {}
                }
                if !(ip.offset_from(candidate) as libc::c_long
                    > ((1 as libc::c_int as size_t) << 18 as libc::c_int)
                        .wrapping_sub(16 as libc::c_int as libc::c_ulong)
                        as libc::c_long)
                {
                    break;
                }
            }
            let mut base = ip;
            let mut matched = min_match
                .wrapping_add(
                    FindMatchLengthWithLimit(
                        candidate.offset(min_match as isize),
                        ip.offset(min_match as isize),
                        (ip_end.offset_from(ip) as libc::c_long as size_t)
                            .wrapping_sub(min_match),
                    ),
                );
            let mut distance = base.offset_from(candidate) as libc::c_long
                as libc::c_int;
            let mut insert = base.offset_from(next_emit) as libc::c_long as libc::c_int;
            ip= ip.offset(matched as isize);
            EmitInsertLen(insert as uint32_t, commands.as_deref_mut());
            memcpy(
                (*literals.as_deref().unwrap()) as *mut libc::c_void,
                next_emit as *const libc::c_void,
                insert as size_t,
            );
            *literals.as_deref_mut().unwrap()= (*literals.as_deref().unwrap()).offset(insert as isize);
            if distance == last_distance {
                *(*commands.as_deref_mut().unwrap())= 64 as libc::c_int as uint32_t;
                *commands.as_deref_mut().unwrap()= (*commands.as_deref().unwrap()).offset(1);
            } else {
                EmitDistance(distance as uint32_t, commands.as_deref_mut());
                last_distance= distance;
            }
            EmitCopyLenLastDistance(matched, commands.as_deref_mut());
            next_emit= ip;
            if (ip >= ip_limit) as libc::c_int as libc::c_long != 0 {
                break;
            }
            let mut input_bytes: uint64_t = 0;
            let mut cur_hash: uint32_t = 0;
            let mut prev_hash: uint32_t = 0;
            if min_match == 4 as libc::c_int as libc::c_ulong {
                input_bytes= BrotliUnalignedRead64(
                    ip.offset(-(3 as libc::c_int as isize)) as *const libc::c_void,
                );
                cur_hash= HashBytesAtOffset(
                    input_bytes,
                    3 as libc::c_int as size_t,
                    shift,
                    min_match,
                );
                prev_hash= HashBytesAtOffset(
                    input_bytes,
                    0 as libc::c_int as size_t,
                    shift,
                    min_match,
                );
                *table
                    .offset(
                        prev_hash as isize,
                    ) = (ip.offset_from(base_ip) as libc::c_long
                    - 3 as libc::c_int as libc::c_long) as libc::c_int;
                prev_hash= HashBytesAtOffset(
                    input_bytes,
                    1 as libc::c_int as size_t,
                    shift,
                    min_match,
                );
                *table
                    .offset(
                        prev_hash as isize,
                    ) = (ip.offset_from(base_ip) as libc::c_long
                    - 2 as libc::c_int as libc::c_long) as libc::c_int;
                prev_hash= HashBytesAtOffset(
                    input_bytes,
                    0 as libc::c_int as size_t,
                    shift,
                    min_match,
                );
                *table
                    .offset(
                        prev_hash as isize,
                    ) = (ip.offset_from(base_ip) as libc::c_long
                    - 1 as libc::c_int as libc::c_long) as libc::c_int;
            } else {
                input_bytes= BrotliUnalignedRead64(
                    ip.offset(-(5 as libc::c_int as isize)) as *const libc::c_void,
                );
                prev_hash= HashBytesAtOffset(
                    input_bytes,
                    0 as libc::c_int as size_t,
                    shift,
                    min_match,
                );
                *table
                    .offset(
                        prev_hash as isize,
                    ) = (ip.offset_from(base_ip) as libc::c_long
                    - 5 as libc::c_int as libc::c_long) as libc::c_int;
                prev_hash= HashBytesAtOffset(
                    input_bytes,
                    1 as libc::c_int as size_t,
                    shift,
                    min_match,
                );
                *table
                    .offset(
                        prev_hash as isize,
                    ) = (ip.offset_from(base_ip) as libc::c_long
                    - 4 as libc::c_int as libc::c_long) as libc::c_int;
                prev_hash= HashBytesAtOffset(
                    input_bytes,
                    2 as libc::c_int as size_t,
                    shift,
                    min_match,
                );
                *table
                    .offset(
                        prev_hash as isize,
                    ) = (ip.offset_from(base_ip) as libc::c_long
                    - 3 as libc::c_int as libc::c_long) as libc::c_int;
                input_bytes= BrotliUnalignedRead64(
                    ip.offset(-(2 as libc::c_int as isize)) as *const libc::c_void,
                );
                cur_hash= HashBytesAtOffset(
                    input_bytes,
                    2 as libc::c_int as size_t,
                    shift,
                    min_match,
                );
                prev_hash= HashBytesAtOffset(
                    input_bytes,
                    0 as libc::c_int as size_t,
                    shift,
                    min_match,
                );
                *table
                    .offset(
                        prev_hash as isize,
                    ) = (ip.offset_from(base_ip) as libc::c_long
                    - 2 as libc::c_int as libc::c_long) as libc::c_int;
                prev_hash= HashBytesAtOffset(
                    input_bytes,
                    1 as libc::c_int as size_t,
                    shift,
                    min_match,
                );
                *table
                    .offset(
                        prev_hash as isize,
                    ) = (ip.offset_from(base_ip) as libc::c_long
                    - 1 as libc::c_int as libc::c_long) as libc::c_int;
            }
            candidate= base_ip.offset(*table.offset(cur_hash as isize) as isize);
            *table
                .offset(
                    cur_hash as isize,
                ) = ip.offset_from(base_ip) as libc::c_long as libc::c_int;
            while ip.offset_from(candidate) as libc::c_long
                <= ((1 as libc::c_int as size_t) << 18 as libc::c_int)
                    .wrapping_sub(16 as libc::c_int as libc::c_ulong) as libc::c_long
                && IsMatch(ip, candidate, min_match) != 0
            {
                let mut base_0 = ip;
                let mut matched_0 = min_match
                    .wrapping_add(
                        FindMatchLengthWithLimit(
                            candidate.offset(min_match as isize),
                            ip.offset(min_match as isize),
                            (ip_end.offset_from(ip) as libc::c_long as size_t)
                                .wrapping_sub(min_match),
                        ),
                    );
                ip= ip.offset(matched_0 as isize);
                last_distance= base_0.offset_from(candidate) as libc::c_long
                    as libc::c_int;
                EmitCopyLen(matched_0, commands.as_deref_mut());
                EmitDistance(last_distance as uint32_t, commands.as_deref_mut());
                next_emit= ip;
                if (ip >= ip_limit) as libc::c_int as libc::c_long != 0 {
                    break 's_41;
                }
                let mut input_bytes_0: uint64_t = 0;
                let mut cur_hash_0: uint32_t = 0;
                let mut prev_hash_0: uint32_t = 0;
                if min_match == 4 as libc::c_int as libc::c_ulong {
                    input_bytes_0= BrotliUnalignedRead64(
                        ip.offset(-(3 as libc::c_int as isize)) as *const libc::c_void,
                    );
                    cur_hash_0= HashBytesAtOffset(
                        input_bytes_0,
                        3 as libc::c_int as size_t,
                        shift,
                        min_match,
                    );
                    prev_hash_0= HashBytesAtOffset(
                        input_bytes_0,
                        0 as libc::c_int as size_t,
                        shift,
                        min_match,
                    );
                    *table
                        .offset(
                            prev_hash_0 as isize,
                        ) = (ip.offset_from(base_ip) as libc::c_long
                        - 3 as libc::c_int as libc::c_long) as libc::c_int;
                    prev_hash_0= HashBytesAtOffset(
                        input_bytes_0,
                        1 as libc::c_int as size_t,
                        shift,
                        min_match,
                    );
                    *table
                        .offset(
                            prev_hash_0 as isize,
                        ) = (ip.offset_from(base_ip) as libc::c_long
                        - 2 as libc::c_int as libc::c_long) as libc::c_int;
                    prev_hash_0= HashBytesAtOffset(
                        input_bytes_0,
                        2 as libc::c_int as size_t,
                        shift,
                        min_match,
                    );
                    *table
                        .offset(
                            prev_hash_0 as isize,
                        ) = (ip.offset_from(base_ip) as libc::c_long
                        - 1 as libc::c_int as libc::c_long) as libc::c_int;
                } else {
                    input_bytes_0= BrotliUnalignedRead64(
                        ip.offset(-(5 as libc::c_int as isize)) as *const libc::c_void,
                    );
                    prev_hash_0= HashBytesAtOffset(
                        input_bytes_0,
                        0 as libc::c_int as size_t,
                        shift,
                        min_match,
                    );
                    *table
                        .offset(
                            prev_hash_0 as isize,
                        ) = (ip.offset_from(base_ip) as libc::c_long
                        - 5 as libc::c_int as libc::c_long) as libc::c_int;
                    prev_hash_0= HashBytesAtOffset(
                        input_bytes_0,
                        1 as libc::c_int as size_t,
                        shift,
                        min_match,
                    );
                    *table
                        .offset(
                            prev_hash_0 as isize,
                        ) = (ip.offset_from(base_ip) as libc::c_long
                        - 4 as libc::c_int as libc::c_long) as libc::c_int;
                    prev_hash_0= HashBytesAtOffset(
                        input_bytes_0,
                        2 as libc::c_int as size_t,
                        shift,
                        min_match,
                    );
                    *table
                        .offset(
                            prev_hash_0 as isize,
                        ) = (ip.offset_from(base_ip) as libc::c_long
                        - 3 as libc::c_int as libc::c_long) as libc::c_int;
                    input_bytes_0= BrotliUnalignedRead64(
                        ip.offset(-(2 as libc::c_int as isize)) as *const libc::c_void,
                    );
                    cur_hash_0= HashBytesAtOffset(
                        input_bytes_0,
                        2 as libc::c_int as size_t,
                        shift,
                        min_match,
                    );
                    prev_hash_0= HashBytesAtOffset(
                        input_bytes_0,
                        0 as libc::c_int as size_t,
                        shift,
                        min_match,
                    );
                    *table
                        .offset(
                            prev_hash_0 as isize,
                        ) = (ip.offset_from(base_ip) as libc::c_long
                        - 2 as libc::c_int as libc::c_long) as libc::c_int;
                    prev_hash_0= HashBytesAtOffset(
                        input_bytes_0,
                        1 as libc::c_int as size_t,
                        shift,
                        min_match,
                    );
                    *table
                        .offset(
                            prev_hash_0 as isize,
                        ) = (ip.offset_from(base_ip) as libc::c_long
                        - 1 as libc::c_int as libc::c_long) as libc::c_int;
                }
                candidate= base_ip.offset(*table.offset(cur_hash_0 as isize) as isize);
                *table
                    .offset(
                        cur_hash_0 as isize,
                    ) = ip.offset_from(base_ip) as libc::c_long as libc::c_int;
            }
            ip= ip.offset(1);
            next_hash= Hash(ip, shift, min_match);
        }
    }
    if next_emit < ip_end {
        let insert_0 = ip_end.offset_from(next_emit) as libc::c_long as uint32_t;
        EmitInsertLen(insert_0, commands.as_deref_mut());
        memcpy(
            (*literals.as_deref().unwrap()) as *mut libc::c_void,
            next_emit as *const libc::c_void,
            insert_0 as libc::c_ulong,
        );
        *literals.as_deref_mut().unwrap()= (*literals.as_deref().unwrap()).offset(insert_0 as isize);
    }
}
unsafe extern "C" fn StoreCommands(
    mut m: *mut crate::src::enc::backward_references_hq::MemoryManager,
    mut literals: *const uint8_t,
    mut num_literals: size_t,
    mut commands: *const uint32_t,
    mut num_commands: size_t,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    static mut kNumExtraBits: [uint32_t; 128] = [
        0 as libc::c_int as uint32_t,
        0 as libc::c_int as uint32_t,
        0 as libc::c_int as uint32_t,
        0 as libc::c_int as uint32_t,
        0 as libc::c_int as uint32_t,
        0 as libc::c_int as uint32_t,
        1 as libc::c_int as uint32_t,
        1 as libc::c_int as uint32_t,
        2 as libc::c_int as uint32_t,
        2 as libc::c_int as uint32_t,
        3 as libc::c_int as uint32_t,
        3 as libc::c_int as uint32_t,
        4 as libc::c_int as uint32_t,
        4 as libc::c_int as uint32_t,
        5 as libc::c_int as uint32_t,
        5 as libc::c_int as uint32_t,
        6 as libc::c_int as uint32_t,
        7 as libc::c_int as uint32_t,
        8 as libc::c_int as uint32_t,
        9 as libc::c_int as uint32_t,
        10 as libc::c_int as uint32_t,
        12 as libc::c_int as uint32_t,
        14 as libc::c_int as uint32_t,
        24 as libc::c_int as uint32_t,
        0 as libc::c_int as uint32_t,
        0 as libc::c_int as uint32_t,
        0 as libc::c_int as uint32_t,
        0 as libc::c_int as uint32_t,
        0 as libc::c_int as uint32_t,
        0 as libc::c_int as uint32_t,
        0 as libc::c_int as uint32_t,
        0 as libc::c_int as uint32_t,
        1 as libc::c_int as uint32_t,
        1 as libc::c_int as uint32_t,
        2 as libc::c_int as uint32_t,
        2 as libc::c_int as uint32_t,
        3 as libc::c_int as uint32_t,
        3 as libc::c_int as uint32_t,
        4 as libc::c_int as uint32_t,
        4 as libc::c_int as uint32_t,
        0 as libc::c_int as uint32_t,
        0 as libc::c_int as uint32_t,
        0 as libc::c_int as uint32_t,
        0 as libc::c_int as uint32_t,
        0 as libc::c_int as uint32_t,
        0 as libc::c_int as uint32_t,
        0 as libc::c_int as uint32_t,
        0 as libc::c_int as uint32_t,
        1 as libc::c_int as uint32_t,
        1 as libc::c_int as uint32_t,
        2 as libc::c_int as uint32_t,
        2 as libc::c_int as uint32_t,
        3 as libc::c_int as uint32_t,
        3 as libc::c_int as uint32_t,
        4 as libc::c_int as uint32_t,
        4 as libc::c_int as uint32_t,
        5 as libc::c_int as uint32_t,
        5 as libc::c_int as uint32_t,
        6 as libc::c_int as uint32_t,
        7 as libc::c_int as uint32_t,
        8 as libc::c_int as uint32_t,
        9 as libc::c_int as uint32_t,
        10 as libc::c_int as uint32_t,
        24 as libc::c_int as uint32_t,
        0 as libc::c_int as uint32_t,
        0 as libc::c_int as uint32_t,
        0 as libc::c_int as uint32_t,
        0 as libc::c_int as uint32_t,
        0 as libc::c_int as uint32_t,
        0 as libc::c_int as uint32_t,
        0 as libc::c_int as uint32_t,
        0 as libc::c_int as uint32_t,
        0 as libc::c_int as uint32_t,
        0 as libc::c_int as uint32_t,
        0 as libc::c_int as uint32_t,
        0 as libc::c_int as uint32_t,
        0 as libc::c_int as uint32_t,
        0 as libc::c_int as uint32_t,
        0 as libc::c_int as uint32_t,
        0 as libc::c_int as uint32_t,
        1 as libc::c_int as uint32_t,
        1 as libc::c_int as uint32_t,
        2 as libc::c_int as uint32_t,
        2 as libc::c_int as uint32_t,
        3 as libc::c_int as uint32_t,
        3 as libc::c_int as uint32_t,
        4 as libc::c_int as uint32_t,
        4 as libc::c_int as uint32_t,
        5 as libc::c_int as uint32_t,
        5 as libc::c_int as uint32_t,
        6 as libc::c_int as uint32_t,
        6 as libc::c_int as uint32_t,
        7 as libc::c_int as uint32_t,
        7 as libc::c_int as uint32_t,
        8 as libc::c_int as uint32_t,
        8 as libc::c_int as uint32_t,
        9 as libc::c_int as uint32_t,
        9 as libc::c_int as uint32_t,
        10 as libc::c_int as uint32_t,
        10 as libc::c_int as uint32_t,
        11 as libc::c_int as uint32_t,
        11 as libc::c_int as uint32_t,
        12 as libc::c_int as uint32_t,
        12 as libc::c_int as uint32_t,
        13 as libc::c_int as uint32_t,
        13 as libc::c_int as uint32_t,
        14 as libc::c_int as uint32_t,
        14 as libc::c_int as uint32_t,
        15 as libc::c_int as uint32_t,
        15 as libc::c_int as uint32_t,
        16 as libc::c_int as uint32_t,
        16 as libc::c_int as uint32_t,
        17 as libc::c_int as uint32_t,
        17 as libc::c_int as uint32_t,
        18 as libc::c_int as uint32_t,
        18 as libc::c_int as uint32_t,
        19 as libc::c_int as uint32_t,
        19 as libc::c_int as uint32_t,
        20 as libc::c_int as uint32_t,
        20 as libc::c_int as uint32_t,
        21 as libc::c_int as uint32_t,
        21 as libc::c_int as uint32_t,
        22 as libc::c_int as uint32_t,
        22 as libc::c_int as uint32_t,
        23 as libc::c_int as uint32_t,
        23 as libc::c_int as uint32_t,
        24 as libc::c_int as uint32_t,
        24 as libc::c_int as uint32_t,
    ];
    static mut kInsertOffset: [uint32_t; 24] = [
        0 as libc::c_int as uint32_t,
        1 as libc::c_int as uint32_t,
        2 as libc::c_int as uint32_t,
        3 as libc::c_int as uint32_t,
        4 as libc::c_int as uint32_t,
        5 as libc::c_int as uint32_t,
        6 as libc::c_int as uint32_t,
        8 as libc::c_int as uint32_t,
        10 as libc::c_int as uint32_t,
        14 as libc::c_int as uint32_t,
        18 as libc::c_int as uint32_t,
        26 as libc::c_int as uint32_t,
        34 as libc::c_int as uint32_t,
        50 as libc::c_int as uint32_t,
        66 as libc::c_int as uint32_t,
        98 as libc::c_int as uint32_t,
        130 as libc::c_int as uint32_t,
        194 as libc::c_int as uint32_t,
        322 as libc::c_int as uint32_t,
        578 as libc::c_int as uint32_t,
        1090 as libc::c_int as uint32_t,
        2114 as libc::c_int as uint32_t,
        6210 as libc::c_int as uint32_t,
        22594 as libc::c_int as uint32_t,
    ];
    let mut lit_depths: [uint8_t; 256] = [0; 256];
    let mut lit_bits: [uint16_t; 256] = [0; 256];
    let mut lit_histo: [uint32_t; 256] = [
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
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
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
    let mut cmd_depths: [uint8_t; 128] = [
        0 as libc::c_int as uint8_t,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
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
    let mut cmd_bits: [uint16_t; 128] = [
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
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
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
    let mut cmd_histo: [uint32_t; 128] = [
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
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
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
    let mut i: size_t = 0;
    i= 0 as libc::c_int as size_t;
    while i < num_literals {
        lit_histo[*literals.offset(i as isize)
            as usize]= lit_histo[*literals.offset(i as isize) as usize]
            .wrapping_add(1);
        i= i.wrapping_add(1);
    }
    crate::src::enc::brotli_bit_stream::BrotliBuildAndStoreHuffmanTreeFast(
        m,
        lit_histo.as_mut_ptr(),
        num_literals,
        8 as libc::c_int as size_t,
        lit_depths.as_mut_ptr(),
        lit_bits.as_mut_ptr(),
        storage_ix,
        storage,
    );
    if 0 as libc::c_int != 0 {
        return;
    }
    i= 0 as libc::c_int as size_t;
    while i < num_commands {
        let code = *commands.offset(i as isize) & 0xff as libc::c_int as libc::c_uint;
        cmd_histo[code as usize]= cmd_histo[code as usize].wrapping_add(1);
        i= i.wrapping_add(1);
    }
    cmd_histo[1 as libc::c_int
        as usize]= (cmd_histo[1 as libc::c_int as usize] as libc::c_uint)
        .wrapping_add(1 as libc::c_int as libc::c_uint) as uint32_t as uint32_t;
    cmd_histo[2 as libc::c_int
        as usize]= (cmd_histo[2 as libc::c_int as usize] as libc::c_uint)
        .wrapping_add(1 as libc::c_int as libc::c_uint) as uint32_t as uint32_t;
    cmd_histo[64 as libc::c_int
        as usize]= (cmd_histo[64 as libc::c_int as usize] as libc::c_uint)
        .wrapping_add(1 as libc::c_int as libc::c_uint) as uint32_t as uint32_t;
    cmd_histo[84 as libc::c_int
        as usize]= (cmd_histo[84 as libc::c_int as usize] as libc::c_uint)
        .wrapping_add(1 as libc::c_int as libc::c_uint) as uint32_t as uint32_t;
    BuildAndStoreCommandPrefixCode(
        cmd_histo.as_mut_ptr() as *const uint32_t,
        cmd_depths.as_mut_ptr(),
        cmd_bits.as_mut_ptr(),
        storage_ix,
        storage,
    );
    i= 0 as libc::c_int as size_t;
    while i < num_commands {
        let cmd = *commands.offset(i as isize);
        let code_0 = cmd & 0xff as libc::c_int as libc::c_uint;
        let extra = cmd >> 8 as libc::c_int;
        BrotliWriteBits(
            cmd_depths[code_0 as usize] as size_t,
            cmd_bits[code_0 as usize] as uint64_t,
            storage_ix,
            storage,
        );
        BrotliWriteBits(
            kNumExtraBits[code_0 as usize] as size_t,
            extra as uint64_t,
            storage_ix,
            storage,
        );
        if code_0 < 24 as libc::c_int as libc::c_uint {
            let insert = kInsertOffset[code_0 as usize].wrapping_add(extra);
            let mut j: uint32_t = 0;
            j= 0 as libc::c_int as uint32_t;
            while j < insert {
                let lit = (*literals);
                BrotliWriteBits(
                    lit_depths[lit as usize] as size_t,
                    lit_bits[lit as usize] as uint64_t,
                    storage_ix,
                    storage,
                );
                literals= literals.offset(1);
                j= j.wrapping_add(1);
            }
        }
        i= i.wrapping_add(1);
    }
}
unsafe extern "C" fn ShouldCompress(
    mut input: *const uint8_t,
    mut input_size: size_t,
    mut num_literals: size_t,
) -> libc::c_int {
    let mut corpus_size = input_size as libc::c_double;
    if (num_literals as libc::c_double) < 0.98f64 * corpus_size {
        return 1 as libc::c_int
    } else {
        let mut literal_histo: [uint32_t; 256] = [
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
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
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
        let max_total_bit_cost = corpus_size * 8 as libc::c_int as libc::c_double
            * 0.98f64 / 43 as libc::c_int as libc::c_double;
        let mut i: size_t = 0;
        i= 0 as libc::c_int as size_t;
        while i < input_size {
            literal_histo[*input.offset(i as isize)
                as usize]= literal_histo[*input.offset(i as isize) as usize]
                .wrapping_add(1);
            i= (i as libc::c_ulong).wrapping_add(43 as libc::c_int as libc::c_ulong)
                as size_t as size_t;
        }
        return if BitsEntropy(literal_histo.as_mut_ptr(), 256 as libc::c_int as size_t)
            < max_total_bit_cost
        {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        };
    };
}
unsafe extern "C" fn RewindBitPosition(
    mut new_storage_ix: size_t,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let bitpos = new_storage_ix & 7 as libc::c_int as libc::c_ulong;
    let mask = ((1 as libc::c_uint) << bitpos)
        .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t;
    *storage.offset((new_storage_ix >> 3 as libc::c_int) as isize) = (*storage.offset((new_storage_ix >> 3 as libc::c_int) as isize) as libc::c_int & mask as uint8_t as libc::c_int) as uint8_t;
    *storage_ix= new_storage_ix;
}
unsafe extern "C" fn EmitUncompressedMetaBlock(
    mut input: *const uint8_t,
    mut input_size: size_t,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    BrotliStoreMetaBlockHeader(input_size, 1 as libc::c_int, storage_ix, storage);
    *storage_ix= (*storage_ix).wrapping_add(7 as libc::c_uint as libc::c_ulong)
        & !(7 as libc::c_uint) as libc::c_ulong;
    memcpy(
        core::ptr::addr_of_mut!(*storage.offset(((*storage_ix) >> 3 as libc::c_int) as isize)) as *mut uint8_t
            as *mut libc::c_void,
        input as *const libc::c_void,
        input_size,
    );
    *storage_ix= ((*storage_ix) as libc::c_ulong)
        .wrapping_add(input_size << 3 as libc::c_int) as size_t as size_t;
    *storage
        .offset(
            ((*storage_ix) >> 3 as libc::c_int) as isize,
        ) = 0 as libc::c_int as uint8_t;
}
#[inline(always)]
unsafe extern "C" fn BrotliCompressFragmentTwoPassImpl(
    mut m: *mut crate::src::enc::backward_references_hq::MemoryManager,
    mut input: *const uint8_t,
    mut input_size: size_t,
    mut is_last: libc::c_int,
    mut command_buf: *mut uint32_t,
    mut literal_buf: *mut uint8_t,
    mut table: *mut libc::c_int,
    mut table_bits: size_t,
    mut min_match: size_t,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let mut base_ip = input;
    while input_size > 0 as libc::c_int as libc::c_ulong {
        let mut block_size = brotli_min_size_t(
            input_size,
            crate::src::enc::compress_fragment_two_pass::kCompressFragmentTwoPassBlockSize,
        );
        let mut commands = command_buf;
        let mut literals = literal_buf;
        let mut num_literals: size_t = 0;
        CreateCommands(
            input,
            block_size,
            input_size,
            base_ip,
            table,
            table_bits,
            min_match,
            Some(&mut literals),
            Some(&mut commands),
        );
        num_literals= literals.offset_from(literal_buf) as libc::c_long as size_t;
        if ShouldCompress(input, block_size, num_literals) != 0 {
            let num_commands = commands.offset_from(command_buf) as libc::c_long
                as size_t;
            BrotliStoreMetaBlockHeader(
                block_size,
                0 as libc::c_int,
                storage_ix,
                storage,
            );
            BrotliWriteBits(
                13 as libc::c_int as size_t,
                0 as libc::c_int as uint64_t,
                storage_ix,
                storage,
            );
            StoreCommands(
                m,
                literal_buf,
                num_literals,
                command_buf,
                num_commands,
                storage_ix,
                storage,
            );
            if 0 as libc::c_int != 0 {
                return;
            }
        } else {
            EmitUncompressedMetaBlock(input, block_size, storage_ix, storage);
        }
        input= input.offset(block_size as isize);
        input_size= (input_size as libc::c_ulong).wrapping_sub(block_size) as size_t
            as size_t;
    }
}
#[inline(never)]
unsafe extern "C" fn BrotliCompressFragmentTwoPassImpl10(
    mut m: *mut crate::src::enc::backward_references_hq::MemoryManager,
    mut input: *const uint8_t,
    mut input_size: size_t,
    mut is_last: libc::c_int,
    mut command_buf: *mut uint32_t,
    mut literal_buf: *mut uint8_t,
    mut table: *mut libc::c_int,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let mut min_match = (if 10 as libc::c_int <= 15 as libc::c_int {
        4 as libc::c_int
    } else {
        6 as libc::c_int
    }) as size_t;
    BrotliCompressFragmentTwoPassImpl(
        m,
        input,
        input_size,
        is_last,
        command_buf,
        literal_buf,
        table,
        10 as libc::c_int as size_t,
        min_match,
        storage_ix,
        storage,
    );
}
#[inline(never)]
unsafe extern "C" fn BrotliCompressFragmentTwoPassImpl16(
    mut m: *mut crate::src::enc::backward_references_hq::MemoryManager,
    mut input: *const uint8_t,
    mut input_size: size_t,
    mut is_last: libc::c_int,
    mut command_buf: *mut uint32_t,
    mut literal_buf: *mut uint8_t,
    mut table: *mut libc::c_int,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let mut min_match = (if 16 as libc::c_int <= 15 as libc::c_int {
        4 as libc::c_int
    } else {
        6 as libc::c_int
    }) as size_t;
    BrotliCompressFragmentTwoPassImpl(
        m,
        input,
        input_size,
        is_last,
        command_buf,
        literal_buf,
        table,
        16 as libc::c_int as size_t,
        min_match,
        storage_ix,
        storage,
    );
}
#[inline(never)]
unsafe extern "C" fn BrotliCompressFragmentTwoPassImpl15(
    mut m: *mut crate::src::enc::backward_references_hq::MemoryManager,
    mut input: *const uint8_t,
    mut input_size: size_t,
    mut is_last: libc::c_int,
    mut command_buf: *mut uint32_t,
    mut literal_buf: *mut uint8_t,
    mut table: *mut libc::c_int,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let mut min_match = (if 15 as libc::c_int <= 15 as libc::c_int {
        4 as libc::c_int
    } else {
        6 as libc::c_int
    }) as size_t;
    BrotliCompressFragmentTwoPassImpl(
        m,
        input,
        input_size,
        is_last,
        command_buf,
        literal_buf,
        table,
        15 as libc::c_int as size_t,
        min_match,
        storage_ix,
        storage,
    );
}
#[inline(never)]
unsafe extern "C" fn BrotliCompressFragmentTwoPassImpl14(
    mut m: *mut crate::src::enc::backward_references_hq::MemoryManager,
    mut input: *const uint8_t,
    mut input_size: size_t,
    mut is_last: libc::c_int,
    mut command_buf: *mut uint32_t,
    mut literal_buf: *mut uint8_t,
    mut table: *mut libc::c_int,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let mut min_match = (if 14 as libc::c_int <= 15 as libc::c_int {
        4 as libc::c_int
    } else {
        6 as libc::c_int
    }) as size_t;
    BrotliCompressFragmentTwoPassImpl(
        m,
        input,
        input_size,
        is_last,
        command_buf,
        literal_buf,
        table,
        14 as libc::c_int as size_t,
        min_match,
        storage_ix,
        storage,
    );
}
#[inline(never)]
unsafe extern "C" fn BrotliCompressFragmentTwoPassImpl13(
    mut m: *mut crate::src::enc::backward_references_hq::MemoryManager,
    mut input: *const uint8_t,
    mut input_size: size_t,
    mut is_last: libc::c_int,
    mut command_buf: *mut uint32_t,
    mut literal_buf: *mut uint8_t,
    mut table: *mut libc::c_int,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let mut min_match = (if 13 as libc::c_int <= 15 as libc::c_int {
        4 as libc::c_int
    } else {
        6 as libc::c_int
    }) as size_t;
    BrotliCompressFragmentTwoPassImpl(
        m,
        input,
        input_size,
        is_last,
        command_buf,
        literal_buf,
        table,
        13 as libc::c_int as size_t,
        min_match,
        storage_ix,
        storage,
    );
}
#[inline(never)]
unsafe extern "C" fn BrotliCompressFragmentTwoPassImpl12(
    mut m: *mut crate::src::enc::backward_references_hq::MemoryManager,
    mut input: *const uint8_t,
    mut input_size: size_t,
    mut is_last: libc::c_int,
    mut command_buf: *mut uint32_t,
    mut literal_buf: *mut uint8_t,
    mut table: *mut libc::c_int,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let mut min_match = (if 12 as libc::c_int <= 15 as libc::c_int {
        4 as libc::c_int
    } else {
        6 as libc::c_int
    }) as size_t;
    BrotliCompressFragmentTwoPassImpl(
        m,
        input,
        input_size,
        is_last,
        command_buf,
        literal_buf,
        table,
        12 as libc::c_int as size_t,
        min_match,
        storage_ix,
        storage,
    );
}
#[inline(never)]
unsafe extern "C" fn BrotliCompressFragmentTwoPassImpl11(
    mut m: *mut crate::src::enc::backward_references_hq::MemoryManager,
    mut input: *const uint8_t,
    mut input_size: size_t,
    mut is_last: libc::c_int,
    mut command_buf: *mut uint32_t,
    mut literal_buf: *mut uint8_t,
    mut table: *mut libc::c_int,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let mut min_match = (if 11 as libc::c_int <= 15 as libc::c_int {
        4 as libc::c_int
    } else {
        6 as libc::c_int
    }) as size_t;
    BrotliCompressFragmentTwoPassImpl(
        m,
        input,
        input_size,
        is_last,
        command_buf,
        literal_buf,
        table,
        11 as libc::c_int as size_t,
        min_match,
        storage_ix,
        storage,
    );
}
#[inline(never)]
unsafe extern "C" fn BrotliCompressFragmentTwoPassImpl17(
    mut m: *mut crate::src::enc::backward_references_hq::MemoryManager,
    mut input: *const uint8_t,
    mut input_size: size_t,
    mut is_last: libc::c_int,
    mut command_buf: *mut uint32_t,
    mut literal_buf: *mut uint8_t,
    mut table: *mut libc::c_int,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let mut min_match = (if 17 as libc::c_int <= 15 as libc::c_int {
        4 as libc::c_int
    } else {
        6 as libc::c_int
    }) as size_t;
    BrotliCompressFragmentTwoPassImpl(
        m,
        input,
        input_size,
        is_last,
        command_buf,
        literal_buf,
        table,
        17 as libc::c_int as size_t,
        min_match,
        storage_ix,
        storage,
    );
}
#[inline(never)]
unsafe extern "C" fn BrotliCompressFragmentTwoPassImpl9(
    mut m: *mut crate::src::enc::backward_references_hq::MemoryManager,
    mut input: *const uint8_t,
    mut input_size: size_t,
    mut is_last: libc::c_int,
    mut command_buf: *mut uint32_t,
    mut literal_buf: *mut uint8_t,
    mut table: *mut libc::c_int,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let mut min_match = (if 9 as libc::c_int <= 15 as libc::c_int {
        4 as libc::c_int
    } else {
        6 as libc::c_int
    }) as size_t;
    BrotliCompressFragmentTwoPassImpl(
        m,
        input,
        input_size,
        is_last,
        command_buf,
        literal_buf,
        table,
        9 as libc::c_int as size_t,
        min_match,
        storage_ix,
        storage,
    );
}
#[inline(never)]
unsafe extern "C" fn BrotliCompressFragmentTwoPassImpl8(
    mut m: *mut crate::src::enc::backward_references_hq::MemoryManager,
    mut input: *const uint8_t,
    mut input_size: size_t,
    mut is_last: libc::c_int,
    mut command_buf: *mut uint32_t,
    mut literal_buf: *mut uint8_t,
    mut table: *mut libc::c_int,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let mut min_match = (if 8 as libc::c_int <= 15 as libc::c_int {
        4 as libc::c_int
    } else {
        6 as libc::c_int
    }) as size_t;
    BrotliCompressFragmentTwoPassImpl(
        m,
        input,
        input_size,
        is_last,
        command_buf,
        literal_buf,
        table,
        8 as libc::c_int as size_t,
        min_match,
        storage_ix,
        storage,
    );
}
#[no_mangle]
pub unsafe extern "C" fn BrotliCompressFragmentTwoPass(
    mut m: *mut crate::src::enc::backward_references_hq::MemoryManager,
    mut input: *const uint8_t,
    mut input_size: size_t,
    mut is_last: libc::c_int,
    mut command_buf: *mut uint32_t,
    mut literal_buf: *mut uint8_t,
    mut table: *mut libc::c_int,
    mut table_size: size_t,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let initial_storage_ix = (*storage_ix);
    let table_bits = Log2FloorNonZero(table_size) as size_t;
    match table_bits {
        8 => {
            BrotliCompressFragmentTwoPassImpl8(
                m,
                input,
                input_size,
                is_last,
                command_buf,
                literal_buf,
                table,
                storage_ix,
                storage,
            );
        }
        9 => {
            BrotliCompressFragmentTwoPassImpl9(
                m,
                input,
                input_size,
                is_last,
                command_buf,
                literal_buf,
                table,
                storage_ix,
                storage,
            );
        }
        10 => {
            BrotliCompressFragmentTwoPassImpl10(
                m,
                input,
                input_size,
                is_last,
                command_buf,
                literal_buf,
                table,
                storage_ix,
                storage,
            );
        }
        11 => {
            BrotliCompressFragmentTwoPassImpl11(
                m,
                input,
                input_size,
                is_last,
                command_buf,
                literal_buf,
                table,
                storage_ix,
                storage,
            );
        }
        12 => {
            BrotliCompressFragmentTwoPassImpl12(
                m,
                input,
                input_size,
                is_last,
                command_buf,
                literal_buf,
                table,
                storage_ix,
                storage,
            );
        }
        13 => {
            BrotliCompressFragmentTwoPassImpl13(
                m,
                input,
                input_size,
                is_last,
                command_buf,
                literal_buf,
                table,
                storage_ix,
                storage,
            );
        }
        14 => {
            BrotliCompressFragmentTwoPassImpl14(
                m,
                input,
                input_size,
                is_last,
                command_buf,
                literal_buf,
                table,
                storage_ix,
                storage,
            );
        }
        15 => {
            BrotliCompressFragmentTwoPassImpl15(
                m,
                input,
                input_size,
                is_last,
                command_buf,
                literal_buf,
                table,
                storage_ix,
                storage,
            );
        }
        16 => {
            BrotliCompressFragmentTwoPassImpl16(
                m,
                input,
                input_size,
                is_last,
                command_buf,
                literal_buf,
                table,
                storage_ix,
                storage,
            );
        }
        17 => {
            BrotliCompressFragmentTwoPassImpl17(
                m,
                input,
                input_size,
                is_last,
                command_buf,
                literal_buf,
                table,
                storage_ix,
                storage,
            );
        }
        _ => {}
    }
    if (*storage_ix).wrapping_sub(initial_storage_ix)
        > (31 as libc::c_int as libc::c_ulong)
            .wrapping_add(input_size << 3 as libc::c_int)
    {
        RewindBitPosition(initial_storage_ix, storage_ix, storage);
        EmitUncompressedMetaBlock(input, input_size, storage_ix, storage);
    }
    if is_last != 0 {
        BrotliWriteBits(
            1 as libc::c_int as size_t,
            1 as libc::c_int as uint64_t,
            storage_ix,
            storage,
        );
        BrotliWriteBits(
            1 as libc::c_int as size_t,
            1 as libc::c_int as uint64_t,
            storage_ix,
            storage,
        );
        *storage_ix= (*storage_ix).wrapping_add(7 as libc::c_uint as libc::c_ulong)
            & !(7 as libc::c_uint) as libc::c_ulong;
    }
}
