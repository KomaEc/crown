use ::libc;
extern "C" {
    static _kBrotliPrefixCodeRanges: [crate::src::common::constants::BrotliPrefixCodeRange; 26];
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
    static _kBrotliContextLookupTable: [uint8_t; 2048];
    static kBrotliInsBase: [uint32_t; 24];
    static kBrotliInsExtra: [uint32_t; 24];
    static kBrotliCopyBase: [uint32_t; 24];
    static kBrotliCopyExtra: [uint32_t; 24];
    
    
    
    
    static kBrotliShellGaps: [size_t; 6];
    
    
}
pub type size_t = libc::c_ulong;
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
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
pub type ContextType = libc::c_uint;
pub const CONTEXT_SIGNED: ContextType = 3;
pub const CONTEXT_UTF8: ContextType = 2;
pub const CONTEXT_MSB6: ContextType = 1;
pub const CONTEXT_LSB6: ContextType = 0;
pub type ContextLut = *const uint8_t;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor53 { dummy: () }
pub type BrotliEncoderMode = libc::c_uint;
pub const BROTLI_MODE_FONT: BrotliEncoderMode = 2;
pub const BROTLI_MODE_TEXT: BrotliEncoderMode = 1;
pub const BROTLI_MODE_GENERIC: BrotliEncoderMode = 0;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor54 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor55 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor56 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor57 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor58 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor59 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor60 { dummy: () }
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HuffmanTree {
    pub total_count_: uint32_t,
    pub index_left_: int16_t,
    pub index_right_or_value_: int16_t,
}
pub type HuffmanTreeComparator = Option::<
    unsafe extern "C" fn(*const HuffmanTree, *const HuffmanTree) -> libc::c_int,
>;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor61 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor62 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor63 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor64 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor65 { dummy: () }
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MetaBlockSplit {
    pub literal_split: crate::src::enc::block_splitter::BlockSplit,
    pub command_split: crate::src::enc::block_splitter::BlockSplit,
    pub distance_split: crate::src::enc::block_splitter::BlockSplit,
    pub literal_context_map: *mut uint32_t,
    pub literal_context_map_size: size_t,
    pub distance_context_map: *mut uint32_t,
    pub distance_context_map_size: size_t,
    pub literal_histograms: *mut crate::src::enc::bit_cost::HistogramLiteral,
    pub literal_histograms_size: size_t,
    pub command_histograms: *mut crate::src::enc::bit_cost::HistogramCommand,
    pub command_histograms_size: size_t,
    pub distance_histograms: *mut crate::src::enc::bit_cost::HistogramDistance,
    pub distance_histograms_size: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BlockEncoder {
    pub histogram_length_: size_t,
    pub num_block_types_: size_t,
    pub block_types_: *const uint8_t,
    pub block_lengths_: *const uint32_t,
    pub num_blocks_: size_t,
    pub block_split_code_: BlockSplitCode,
    pub block_ix_: size_t,
    pub block_len_: size_t,
    pub entropy_ix_: size_t,
    pub depths_: *mut uint8_t,
    pub bits_: *mut uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BlockSplitCode {
    pub type_code_calculator: BlockTypeCodeCalculator,
    pub type_depths: [uint8_t; 258],
    pub type_bits: [uint16_t; 258],
    pub length_depths: [uint8_t; 26],
    pub length_bits: [uint16_t; 26],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BlockTypeCodeCalculator {
    pub last_type: size_t,
    pub second_last_type: size_t,
}
#[inline(always)]
unsafe extern "C" fn brotli_max_uint32_t(mut a: uint32_t, mut b: uint32_t) -> uint32_t {
    return if a > b { a } else { b };
}
#[inline(always)]
unsafe extern "C" fn brotli_min_uint32_t(mut a: uint32_t, mut b: uint32_t) -> uint32_t {
    return if a < b { a } else { b };
}
#[inline(always)]
unsafe extern "C" fn BrotliUnalignedWrite64(mut p: *mut libc::c_void, mut v: uint64_t) {
    *(p as *mut uint64_t) = v;
}
#[inline(always)]
unsafe extern "C" fn Log2FloorNonZero(mut n: size_t) -> uint32_t {
    return 31 as libc::c_uint ^ (n as uint32_t).leading_zeros() as i32 as uint32_t;
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
unsafe extern "C" fn GetInsertBase(mut inscode: uint16_t) -> uint32_t {
    return crate::src::enc::brotli_bit_stream::kBrotliInsBase[inscode as usize];
}
#[inline(always)]
unsafe extern "C" fn GetInsertExtra(mut inscode: uint16_t) -> uint32_t {
    return crate::src::enc::brotli_bit_stream::kBrotliInsExtra[inscode as usize];
}
#[inline(always)]
unsafe extern "C" fn GetCopyBase(mut copycode: uint16_t) -> uint32_t {
    return crate::src::enc::brotli_bit_stream::kBrotliCopyBase[copycode as usize];
}
#[inline(always)]
unsafe extern "C" fn GetCopyExtra(mut copycode: uint16_t) -> uint32_t {
    return crate::src::enc::brotli_bit_stream::kBrotliCopyExtra[copycode as usize];
}
#[inline(always)]
unsafe extern "C" fn CommandDistanceContext(mut self_0: *const crate::src::enc::backward_references::Command) -> uint32_t {
    let mut r = ((*self_0).cmd_prefix_ as libc::c_int >> 6 as libc::c_int) as uint32_t;
    let mut c = ((*self_0).cmd_prefix_ as libc::c_int & 7 as libc::c_int) as uint32_t;
    if (r == 0 as libc::c_int as libc::c_uint || r == 2 as libc::c_int as libc::c_uint
        || r == 4 as libc::c_int as libc::c_uint
        || r == 7 as libc::c_int as libc::c_uint)
        && c <= 2 as libc::c_int as libc::c_uint
    {
        return c;
    }
    return 3 as libc::c_int as uint32_t;
}
#[inline(always)]
unsafe extern "C" fn CommandCopyLen(mut self_0: *const crate::src::enc::backward_references::Command) -> uint32_t {
    return (*self_0).copy_len_ & 0x1ffffff as libc::c_int as libc::c_uint;
}
#[inline(always)]
unsafe extern "C" fn CommandCopyLenCode(mut self_0: *const crate::src::enc::backward_references::Command) -> uint32_t {
    let mut modifier = (*self_0).copy_len_ >> 25 as libc::c_int;
    let mut delta = (modifier
        | (modifier & 0x40 as libc::c_int as libc::c_uint) << 1 as libc::c_int)
        as uint8_t as int8_t as int32_t;
    return (((*self_0).copy_len_ & 0x1ffffff as libc::c_int as libc::c_uint) as int32_t
        + delta) as uint32_t;
}
#[inline(always)]
unsafe extern "C" fn InitHuffmanTree(
    mut self_0: Option<&mut HuffmanTree>,
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
    mut items: *mut HuffmanTree,
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
            let mut gap = crate::src::enc::brotli_bit_stream::kBrotliShellGaps[g as usize];
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
unsafe extern "C" fn HistogramAddLiteral(
    mut self_0: Option<&mut crate::src::enc::bit_cost::HistogramLiteral>,
    mut val: size_t,
) {
    (*self_0.as_deref_mut().unwrap()).data_[val as usize]= (*self_0.as_deref().unwrap()).data_[val as usize].wrapping_add(1);
    (*self_0.as_deref_mut().unwrap()).total_count_= (*self_0.as_deref().unwrap()).total_count_.wrapping_add(1);
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
unsafe extern "C" fn HistogramAddCommand(
    mut self_0: Option<&mut crate::src::enc::bit_cost::HistogramCommand>,
    mut val: size_t,
) {
    (*self_0.as_deref_mut().unwrap()).data_[val as usize]= (*self_0.as_deref().unwrap()).data_[val as usize].wrapping_add(1);
    (*self_0.as_deref_mut().unwrap()).total_count_= (*self_0.as_deref().unwrap()).total_count_.wrapping_add(1);
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
unsafe extern "C" fn HistogramAddDistance(
    mut self_0: Option<&mut crate::src::enc::bit_cost::HistogramDistance>,
    mut val: size_t,
) {
    (*self_0.as_deref_mut().unwrap()).data_[val as usize]= (*self_0.as_deref().unwrap()).data_[val as usize].wrapping_add(1);
    (*self_0.as_deref_mut().unwrap()).total_count_= (*self_0.as_deref().unwrap()).total_count_.wrapping_add(1);
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
static mut kZeroRepsDepth: [uint32_t; 704] = [
    0 as libc::c_int as uint32_t,
    4 as libc::c_int as uint32_t,
    8 as libc::c_int as uint32_t,
    7 as libc::c_int as uint32_t,
    7 as libc::c_int as uint32_t,
    7 as libc::c_int as uint32_t,
    7 as libc::c_int as uint32_t,
    7 as libc::c_int as uint32_t,
    7 as libc::c_int as uint32_t,
    7 as libc::c_int as uint32_t,
    7 as libc::c_int as uint32_t,
    11 as libc::c_int as uint32_t,
    14 as libc::c_int as uint32_t,
    14 as libc::c_int as uint32_t,
    14 as libc::c_int as uint32_t,
    14 as libc::c_int as uint32_t,
    14 as libc::c_int as uint32_t,
    14 as libc::c_int as uint32_t,
    14 as libc::c_int as uint32_t,
    14 as libc::c_int as uint32_t,
    14 as libc::c_int as uint32_t,
    14 as libc::c_int as uint32_t,
    14 as libc::c_int as uint32_t,
    14 as libc::c_int as uint32_t,
    14 as libc::c_int as uint32_t,
    14 as libc::c_int as uint32_t,
    14 as libc::c_int as uint32_t,
    14 as libc::c_int as uint32_t,
    14 as libc::c_int as uint32_t,
    14 as libc::c_int as uint32_t,
    14 as libc::c_int as uint32_t,
    14 as libc::c_int as uint32_t,
    14 as libc::c_int as uint32_t,
    14 as libc::c_int as uint32_t,
    14 as libc::c_int as uint32_t,
    14 as libc::c_int as uint32_t,
    14 as libc::c_int as uint32_t,
    14 as libc::c_int as uint32_t,
    14 as libc::c_int as uint32_t,
    14 as libc::c_int as uint32_t,
    14 as libc::c_int as uint32_t,
    14 as libc::c_int as uint32_t,
    14 as libc::c_int as uint32_t,
    14 as libc::c_int as uint32_t,
    14 as libc::c_int as uint32_t,
    14 as libc::c_int as uint32_t,
    14 as libc::c_int as uint32_t,
    14 as libc::c_int as uint32_t,
    14 as libc::c_int as uint32_t,
    14 as libc::c_int as uint32_t,
    14 as libc::c_int as uint32_t,
    14 as libc::c_int as uint32_t,
    14 as libc::c_int as uint32_t,
    14 as libc::c_int as uint32_t,
    14 as libc::c_int as uint32_t,
    14 as libc::c_int as uint32_t,
    14 as libc::c_int as uint32_t,
    14 as libc::c_int as uint32_t,
    14 as libc::c_int as uint32_t,
    14 as libc::c_int as uint32_t,
    14 as libc::c_int as uint32_t,
    14 as libc::c_int as uint32_t,
    14 as libc::c_int as uint32_t,
    14 as libc::c_int as uint32_t,
    14 as libc::c_int as uint32_t,
    14 as libc::c_int as uint32_t,
    14 as libc::c_int as uint32_t,
    14 as libc::c_int as uint32_t,
    14 as libc::c_int as uint32_t,
    14 as libc::c_int as uint32_t,
    14 as libc::c_int as uint32_t,
    14 as libc::c_int as uint32_t,
    14 as libc::c_int as uint32_t,
    14 as libc::c_int as uint32_t,
    14 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    21 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
    28 as libc::c_int as uint32_t,
];
static mut kZeroRepsBits: [uint64_t; 704] = [
    0 as libc::c_int as uint64_t,
    0 as libc::c_int as uint64_t,
    0 as libc::c_int as uint64_t,
    0x7 as libc::c_int as uint64_t,
    0x17 as libc::c_int as uint64_t,
    0x27 as libc::c_int as uint64_t,
    0x37 as libc::c_int as uint64_t,
    0x47 as libc::c_int as uint64_t,
    0x57 as libc::c_int as uint64_t,
    0x67 as libc::c_int as uint64_t,
    0x77 as libc::c_int as uint64_t,
    0x770 as libc::c_int as uint64_t,
    0xb87 as libc::c_int as uint64_t,
    0x1387 as libc::c_int as uint64_t,
    0x1b87 as libc::c_int as uint64_t,
    0x2387 as libc::c_int as uint64_t,
    0x2b87 as libc::c_int as uint64_t,
    0x3387 as libc::c_int as uint64_t,
    0x3b87 as libc::c_int as uint64_t,
    0x397 as libc::c_int as uint64_t,
    0xb97 as libc::c_int as uint64_t,
    0x1397 as libc::c_int as uint64_t,
    0x1b97 as libc::c_int as uint64_t,
    0x2397 as libc::c_int as uint64_t,
    0x2b97 as libc::c_int as uint64_t,
    0x3397 as libc::c_int as uint64_t,
    0x3b97 as libc::c_int as uint64_t,
    0x3a7 as libc::c_int as uint64_t,
    0xba7 as libc::c_int as uint64_t,
    0x13a7 as libc::c_int as uint64_t,
    0x1ba7 as libc::c_int as uint64_t,
    0x23a7 as libc::c_int as uint64_t,
    0x2ba7 as libc::c_int as uint64_t,
    0x33a7 as libc::c_int as uint64_t,
    0x3ba7 as libc::c_int as uint64_t,
    0x3b7 as libc::c_int as uint64_t,
    0xbb7 as libc::c_int as uint64_t,
    0x13b7 as libc::c_int as uint64_t,
    0x1bb7 as libc::c_int as uint64_t,
    0x23b7 as libc::c_int as uint64_t,
    0x2bb7 as libc::c_int as uint64_t,
    0x33b7 as libc::c_int as uint64_t,
    0x3bb7 as libc::c_int as uint64_t,
    0x3c7 as libc::c_int as uint64_t,
    0xbc7 as libc::c_int as uint64_t,
    0x13c7 as libc::c_int as uint64_t,
    0x1bc7 as libc::c_int as uint64_t,
    0x23c7 as libc::c_int as uint64_t,
    0x2bc7 as libc::c_int as uint64_t,
    0x33c7 as libc::c_int as uint64_t,
    0x3bc7 as libc::c_int as uint64_t,
    0x3d7 as libc::c_int as uint64_t,
    0xbd7 as libc::c_int as uint64_t,
    0x13d7 as libc::c_int as uint64_t,
    0x1bd7 as libc::c_int as uint64_t,
    0x23d7 as libc::c_int as uint64_t,
    0x2bd7 as libc::c_int as uint64_t,
    0x33d7 as libc::c_int as uint64_t,
    0x3bd7 as libc::c_int as uint64_t,
    0x3e7 as libc::c_int as uint64_t,
    0xbe7 as libc::c_int as uint64_t,
    0x13e7 as libc::c_int as uint64_t,
    0x1be7 as libc::c_int as uint64_t,
    0x23e7 as libc::c_int as uint64_t,
    0x2be7 as libc::c_int as uint64_t,
    0x33e7 as libc::c_int as uint64_t,
    0x3be7 as libc::c_int as uint64_t,
    0x3f7 as libc::c_int as uint64_t,
    0xbf7 as libc::c_int as uint64_t,
    0x13f7 as libc::c_int as uint64_t,
    0x1bf7 as libc::c_int as uint64_t,
    0x23f7 as libc::c_int as uint64_t,
    0x2bf7 as libc::c_int as uint64_t,
    0x33f7 as libc::c_int as uint64_t,
    0x3bf7 as libc::c_int as uint64_t,
    0x1c387 as libc::c_int as uint64_t,
    0x5c387 as libc::c_int as uint64_t,
    0x9c387 as libc::c_int as uint64_t,
    0xdc387 as libc::c_int as uint64_t,
    0x11c387 as libc::c_int as uint64_t,
    0x15c387 as libc::c_int as uint64_t,
    0x19c387 as libc::c_int as uint64_t,
    0x1dc387 as libc::c_int as uint64_t,
    0x1cb87 as libc::c_int as uint64_t,
    0x5cb87 as libc::c_int as uint64_t,
    0x9cb87 as libc::c_int as uint64_t,
    0xdcb87 as libc::c_int as uint64_t,
    0x11cb87 as libc::c_int as uint64_t,
    0x15cb87 as libc::c_int as uint64_t,
    0x19cb87 as libc::c_int as uint64_t,
    0x1dcb87 as libc::c_int as uint64_t,
    0x1d387 as libc::c_int as uint64_t,
    0x5d387 as libc::c_int as uint64_t,
    0x9d387 as libc::c_int as uint64_t,
    0xdd387 as libc::c_int as uint64_t,
    0x11d387 as libc::c_int as uint64_t,
    0x15d387 as libc::c_int as uint64_t,
    0x19d387 as libc::c_int as uint64_t,
    0x1dd387 as libc::c_int as uint64_t,
    0x1db87 as libc::c_int as uint64_t,
    0x5db87 as libc::c_int as uint64_t,
    0x9db87 as libc::c_int as uint64_t,
    0xddb87 as libc::c_int as uint64_t,
    0x11db87 as libc::c_int as uint64_t,
    0x15db87 as libc::c_int as uint64_t,
    0x19db87 as libc::c_int as uint64_t,
    0x1ddb87 as libc::c_int as uint64_t,
    0x1e387 as libc::c_int as uint64_t,
    0x5e387 as libc::c_int as uint64_t,
    0x9e387 as libc::c_int as uint64_t,
    0xde387 as libc::c_int as uint64_t,
    0x11e387 as libc::c_int as uint64_t,
    0x15e387 as libc::c_int as uint64_t,
    0x19e387 as libc::c_int as uint64_t,
    0x1de387 as libc::c_int as uint64_t,
    0x1eb87 as libc::c_int as uint64_t,
    0x5eb87 as libc::c_int as uint64_t,
    0x9eb87 as libc::c_int as uint64_t,
    0xdeb87 as libc::c_int as uint64_t,
    0x11eb87 as libc::c_int as uint64_t,
    0x15eb87 as libc::c_int as uint64_t,
    0x19eb87 as libc::c_int as uint64_t,
    0x1deb87 as libc::c_int as uint64_t,
    0x1f387 as libc::c_int as uint64_t,
    0x5f387 as libc::c_int as uint64_t,
    0x9f387 as libc::c_int as uint64_t,
    0xdf387 as libc::c_int as uint64_t,
    0x11f387 as libc::c_int as uint64_t,
    0x15f387 as libc::c_int as uint64_t,
    0x19f387 as libc::c_int as uint64_t,
    0x1df387 as libc::c_int as uint64_t,
    0x1fb87 as libc::c_int as uint64_t,
    0x5fb87 as libc::c_int as uint64_t,
    0x9fb87 as libc::c_int as uint64_t,
    0xdfb87 as libc::c_int as uint64_t,
    0x11fb87 as libc::c_int as uint64_t,
    0x15fb87 as libc::c_int as uint64_t,
    0x19fb87 as libc::c_int as uint64_t,
    0x1dfb87 as libc::c_int as uint64_t,
    0x1c397 as libc::c_int as uint64_t,
    0x5c397 as libc::c_int as uint64_t,
    0x9c397 as libc::c_int as uint64_t,
    0xdc397 as libc::c_int as uint64_t,
    0x11c397 as libc::c_int as uint64_t,
    0x15c397 as libc::c_int as uint64_t,
    0x19c397 as libc::c_int as uint64_t,
    0x1dc397 as libc::c_int as uint64_t,
    0x1cb97 as libc::c_int as uint64_t,
    0x5cb97 as libc::c_int as uint64_t,
    0x9cb97 as libc::c_int as uint64_t,
    0xdcb97 as libc::c_int as uint64_t,
    0x11cb97 as libc::c_int as uint64_t,
    0x15cb97 as libc::c_int as uint64_t,
    0x19cb97 as libc::c_int as uint64_t,
    0x1dcb97 as libc::c_int as uint64_t,
    0x1d397 as libc::c_int as uint64_t,
    0x5d397 as libc::c_int as uint64_t,
    0x9d397 as libc::c_int as uint64_t,
    0xdd397 as libc::c_int as uint64_t,
    0x11d397 as libc::c_int as uint64_t,
    0x15d397 as libc::c_int as uint64_t,
    0x19d397 as libc::c_int as uint64_t,
    0x1dd397 as libc::c_int as uint64_t,
    0x1db97 as libc::c_int as uint64_t,
    0x5db97 as libc::c_int as uint64_t,
    0x9db97 as libc::c_int as uint64_t,
    0xddb97 as libc::c_int as uint64_t,
    0x11db97 as libc::c_int as uint64_t,
    0x15db97 as libc::c_int as uint64_t,
    0x19db97 as libc::c_int as uint64_t,
    0x1ddb97 as libc::c_int as uint64_t,
    0x1e397 as libc::c_int as uint64_t,
    0x5e397 as libc::c_int as uint64_t,
    0x9e397 as libc::c_int as uint64_t,
    0xde397 as libc::c_int as uint64_t,
    0x11e397 as libc::c_int as uint64_t,
    0x15e397 as libc::c_int as uint64_t,
    0x19e397 as libc::c_int as uint64_t,
    0x1de397 as libc::c_int as uint64_t,
    0x1eb97 as libc::c_int as uint64_t,
    0x5eb97 as libc::c_int as uint64_t,
    0x9eb97 as libc::c_int as uint64_t,
    0xdeb97 as libc::c_int as uint64_t,
    0x11eb97 as libc::c_int as uint64_t,
    0x15eb97 as libc::c_int as uint64_t,
    0x19eb97 as libc::c_int as uint64_t,
    0x1deb97 as libc::c_int as uint64_t,
    0x1f397 as libc::c_int as uint64_t,
    0x5f397 as libc::c_int as uint64_t,
    0x9f397 as libc::c_int as uint64_t,
    0xdf397 as libc::c_int as uint64_t,
    0x11f397 as libc::c_int as uint64_t,
    0x15f397 as libc::c_int as uint64_t,
    0x19f397 as libc::c_int as uint64_t,
    0x1df397 as libc::c_int as uint64_t,
    0x1fb97 as libc::c_int as uint64_t,
    0x5fb97 as libc::c_int as uint64_t,
    0x9fb97 as libc::c_int as uint64_t,
    0xdfb97 as libc::c_int as uint64_t,
    0x11fb97 as libc::c_int as uint64_t,
    0x15fb97 as libc::c_int as uint64_t,
    0x19fb97 as libc::c_int as uint64_t,
    0x1dfb97 as libc::c_int as uint64_t,
    0x1c3a7 as libc::c_int as uint64_t,
    0x5c3a7 as libc::c_int as uint64_t,
    0x9c3a7 as libc::c_int as uint64_t,
    0xdc3a7 as libc::c_int as uint64_t,
    0x11c3a7 as libc::c_int as uint64_t,
    0x15c3a7 as libc::c_int as uint64_t,
    0x19c3a7 as libc::c_int as uint64_t,
    0x1dc3a7 as libc::c_int as uint64_t,
    0x1cba7 as libc::c_int as uint64_t,
    0x5cba7 as libc::c_int as uint64_t,
    0x9cba7 as libc::c_int as uint64_t,
    0xdcba7 as libc::c_int as uint64_t,
    0x11cba7 as libc::c_int as uint64_t,
    0x15cba7 as libc::c_int as uint64_t,
    0x19cba7 as libc::c_int as uint64_t,
    0x1dcba7 as libc::c_int as uint64_t,
    0x1d3a7 as libc::c_int as uint64_t,
    0x5d3a7 as libc::c_int as uint64_t,
    0x9d3a7 as libc::c_int as uint64_t,
    0xdd3a7 as libc::c_int as uint64_t,
    0x11d3a7 as libc::c_int as uint64_t,
    0x15d3a7 as libc::c_int as uint64_t,
    0x19d3a7 as libc::c_int as uint64_t,
    0x1dd3a7 as libc::c_int as uint64_t,
    0x1dba7 as libc::c_int as uint64_t,
    0x5dba7 as libc::c_int as uint64_t,
    0x9dba7 as libc::c_int as uint64_t,
    0xddba7 as libc::c_int as uint64_t,
    0x11dba7 as libc::c_int as uint64_t,
    0x15dba7 as libc::c_int as uint64_t,
    0x19dba7 as libc::c_int as uint64_t,
    0x1ddba7 as libc::c_int as uint64_t,
    0x1e3a7 as libc::c_int as uint64_t,
    0x5e3a7 as libc::c_int as uint64_t,
    0x9e3a7 as libc::c_int as uint64_t,
    0xde3a7 as libc::c_int as uint64_t,
    0x11e3a7 as libc::c_int as uint64_t,
    0x15e3a7 as libc::c_int as uint64_t,
    0x19e3a7 as libc::c_int as uint64_t,
    0x1de3a7 as libc::c_int as uint64_t,
    0x1eba7 as libc::c_int as uint64_t,
    0x5eba7 as libc::c_int as uint64_t,
    0x9eba7 as libc::c_int as uint64_t,
    0xdeba7 as libc::c_int as uint64_t,
    0x11eba7 as libc::c_int as uint64_t,
    0x15eba7 as libc::c_int as uint64_t,
    0x19eba7 as libc::c_int as uint64_t,
    0x1deba7 as libc::c_int as uint64_t,
    0x1f3a7 as libc::c_int as uint64_t,
    0x5f3a7 as libc::c_int as uint64_t,
    0x9f3a7 as libc::c_int as uint64_t,
    0xdf3a7 as libc::c_int as uint64_t,
    0x11f3a7 as libc::c_int as uint64_t,
    0x15f3a7 as libc::c_int as uint64_t,
    0x19f3a7 as libc::c_int as uint64_t,
    0x1df3a7 as libc::c_int as uint64_t,
    0x1fba7 as libc::c_int as uint64_t,
    0x5fba7 as libc::c_int as uint64_t,
    0x9fba7 as libc::c_int as uint64_t,
    0xdfba7 as libc::c_int as uint64_t,
    0x11fba7 as libc::c_int as uint64_t,
    0x15fba7 as libc::c_int as uint64_t,
    0x19fba7 as libc::c_int as uint64_t,
    0x1dfba7 as libc::c_int as uint64_t,
    0x1c3b7 as libc::c_int as uint64_t,
    0x5c3b7 as libc::c_int as uint64_t,
    0x9c3b7 as libc::c_int as uint64_t,
    0xdc3b7 as libc::c_int as uint64_t,
    0x11c3b7 as libc::c_int as uint64_t,
    0x15c3b7 as libc::c_int as uint64_t,
    0x19c3b7 as libc::c_int as uint64_t,
    0x1dc3b7 as libc::c_int as uint64_t,
    0x1cbb7 as libc::c_int as uint64_t,
    0x5cbb7 as libc::c_int as uint64_t,
    0x9cbb7 as libc::c_int as uint64_t,
    0xdcbb7 as libc::c_int as uint64_t,
    0x11cbb7 as libc::c_int as uint64_t,
    0x15cbb7 as libc::c_int as uint64_t,
    0x19cbb7 as libc::c_int as uint64_t,
    0x1dcbb7 as libc::c_int as uint64_t,
    0x1d3b7 as libc::c_int as uint64_t,
    0x5d3b7 as libc::c_int as uint64_t,
    0x9d3b7 as libc::c_int as uint64_t,
    0xdd3b7 as libc::c_int as uint64_t,
    0x11d3b7 as libc::c_int as uint64_t,
    0x15d3b7 as libc::c_int as uint64_t,
    0x19d3b7 as libc::c_int as uint64_t,
    0x1dd3b7 as libc::c_int as uint64_t,
    0x1dbb7 as libc::c_int as uint64_t,
    0x5dbb7 as libc::c_int as uint64_t,
    0x9dbb7 as libc::c_int as uint64_t,
    0xddbb7 as libc::c_int as uint64_t,
    0x11dbb7 as libc::c_int as uint64_t,
    0x15dbb7 as libc::c_int as uint64_t,
    0x19dbb7 as libc::c_int as uint64_t,
    0x1ddbb7 as libc::c_int as uint64_t,
    0x1e3b7 as libc::c_int as uint64_t,
    0x5e3b7 as libc::c_int as uint64_t,
    0x9e3b7 as libc::c_int as uint64_t,
    0xde3b7 as libc::c_int as uint64_t,
    0x11e3b7 as libc::c_int as uint64_t,
    0x15e3b7 as libc::c_int as uint64_t,
    0x19e3b7 as libc::c_int as uint64_t,
    0x1de3b7 as libc::c_int as uint64_t,
    0x1ebb7 as libc::c_int as uint64_t,
    0x5ebb7 as libc::c_int as uint64_t,
    0x9ebb7 as libc::c_int as uint64_t,
    0xdebb7 as libc::c_int as uint64_t,
    0x11ebb7 as libc::c_int as uint64_t,
    0x15ebb7 as libc::c_int as uint64_t,
    0x19ebb7 as libc::c_int as uint64_t,
    0x1debb7 as libc::c_int as uint64_t,
    0x1f3b7 as libc::c_int as uint64_t,
    0x5f3b7 as libc::c_int as uint64_t,
    0x9f3b7 as libc::c_int as uint64_t,
    0xdf3b7 as libc::c_int as uint64_t,
    0x11f3b7 as libc::c_int as uint64_t,
    0x15f3b7 as libc::c_int as uint64_t,
    0x19f3b7 as libc::c_int as uint64_t,
    0x1df3b7 as libc::c_int as uint64_t,
    0x1fbb7 as libc::c_int as uint64_t,
    0x5fbb7 as libc::c_int as uint64_t,
    0x9fbb7 as libc::c_int as uint64_t,
    0xdfbb7 as libc::c_int as uint64_t,
    0x11fbb7 as libc::c_int as uint64_t,
    0x15fbb7 as libc::c_int as uint64_t,
    0x19fbb7 as libc::c_int as uint64_t,
    0x1dfbb7 as libc::c_int as uint64_t,
    0x1c3c7 as libc::c_int as uint64_t,
    0x5c3c7 as libc::c_int as uint64_t,
    0x9c3c7 as libc::c_int as uint64_t,
    0xdc3c7 as libc::c_int as uint64_t,
    0x11c3c7 as libc::c_int as uint64_t,
    0x15c3c7 as libc::c_int as uint64_t,
    0x19c3c7 as libc::c_int as uint64_t,
    0x1dc3c7 as libc::c_int as uint64_t,
    0x1cbc7 as libc::c_int as uint64_t,
    0x5cbc7 as libc::c_int as uint64_t,
    0x9cbc7 as libc::c_int as uint64_t,
    0xdcbc7 as libc::c_int as uint64_t,
    0x11cbc7 as libc::c_int as uint64_t,
    0x15cbc7 as libc::c_int as uint64_t,
    0x19cbc7 as libc::c_int as uint64_t,
    0x1dcbc7 as libc::c_int as uint64_t,
    0x1d3c7 as libc::c_int as uint64_t,
    0x5d3c7 as libc::c_int as uint64_t,
    0x9d3c7 as libc::c_int as uint64_t,
    0xdd3c7 as libc::c_int as uint64_t,
    0x11d3c7 as libc::c_int as uint64_t,
    0x15d3c7 as libc::c_int as uint64_t,
    0x19d3c7 as libc::c_int as uint64_t,
    0x1dd3c7 as libc::c_int as uint64_t,
    0x1dbc7 as libc::c_int as uint64_t,
    0x5dbc7 as libc::c_int as uint64_t,
    0x9dbc7 as libc::c_int as uint64_t,
    0xddbc7 as libc::c_int as uint64_t,
    0x11dbc7 as libc::c_int as uint64_t,
    0x15dbc7 as libc::c_int as uint64_t,
    0x19dbc7 as libc::c_int as uint64_t,
    0x1ddbc7 as libc::c_int as uint64_t,
    0x1e3c7 as libc::c_int as uint64_t,
    0x5e3c7 as libc::c_int as uint64_t,
    0x9e3c7 as libc::c_int as uint64_t,
    0xde3c7 as libc::c_int as uint64_t,
    0x11e3c7 as libc::c_int as uint64_t,
    0x15e3c7 as libc::c_int as uint64_t,
    0x19e3c7 as libc::c_int as uint64_t,
    0x1de3c7 as libc::c_int as uint64_t,
    0x1ebc7 as libc::c_int as uint64_t,
    0x5ebc7 as libc::c_int as uint64_t,
    0x9ebc7 as libc::c_int as uint64_t,
    0xdebc7 as libc::c_int as uint64_t,
    0x11ebc7 as libc::c_int as uint64_t,
    0x15ebc7 as libc::c_int as uint64_t,
    0x19ebc7 as libc::c_int as uint64_t,
    0x1debc7 as libc::c_int as uint64_t,
    0x1f3c7 as libc::c_int as uint64_t,
    0x5f3c7 as libc::c_int as uint64_t,
    0x9f3c7 as libc::c_int as uint64_t,
    0xdf3c7 as libc::c_int as uint64_t,
    0x11f3c7 as libc::c_int as uint64_t,
    0x15f3c7 as libc::c_int as uint64_t,
    0x19f3c7 as libc::c_int as uint64_t,
    0x1df3c7 as libc::c_int as uint64_t,
    0x1fbc7 as libc::c_int as uint64_t,
    0x5fbc7 as libc::c_int as uint64_t,
    0x9fbc7 as libc::c_int as uint64_t,
    0xdfbc7 as libc::c_int as uint64_t,
    0x11fbc7 as libc::c_int as uint64_t,
    0x15fbc7 as libc::c_int as uint64_t,
    0x19fbc7 as libc::c_int as uint64_t,
    0x1dfbc7 as libc::c_int as uint64_t,
    0x1c3d7 as libc::c_int as uint64_t,
    0x5c3d7 as libc::c_int as uint64_t,
    0x9c3d7 as libc::c_int as uint64_t,
    0xdc3d7 as libc::c_int as uint64_t,
    0x11c3d7 as libc::c_int as uint64_t,
    0x15c3d7 as libc::c_int as uint64_t,
    0x19c3d7 as libc::c_int as uint64_t,
    0x1dc3d7 as libc::c_int as uint64_t,
    0x1cbd7 as libc::c_int as uint64_t,
    0x5cbd7 as libc::c_int as uint64_t,
    0x9cbd7 as libc::c_int as uint64_t,
    0xdcbd7 as libc::c_int as uint64_t,
    0x11cbd7 as libc::c_int as uint64_t,
    0x15cbd7 as libc::c_int as uint64_t,
    0x19cbd7 as libc::c_int as uint64_t,
    0x1dcbd7 as libc::c_int as uint64_t,
    0x1d3d7 as libc::c_int as uint64_t,
    0x5d3d7 as libc::c_int as uint64_t,
    0x9d3d7 as libc::c_int as uint64_t,
    0xdd3d7 as libc::c_int as uint64_t,
    0x11d3d7 as libc::c_int as uint64_t,
    0x15d3d7 as libc::c_int as uint64_t,
    0x19d3d7 as libc::c_int as uint64_t,
    0x1dd3d7 as libc::c_int as uint64_t,
    0x1dbd7 as libc::c_int as uint64_t,
    0x5dbd7 as libc::c_int as uint64_t,
    0x9dbd7 as libc::c_int as uint64_t,
    0xddbd7 as libc::c_int as uint64_t,
    0x11dbd7 as libc::c_int as uint64_t,
    0x15dbd7 as libc::c_int as uint64_t,
    0x19dbd7 as libc::c_int as uint64_t,
    0x1ddbd7 as libc::c_int as uint64_t,
    0x1e3d7 as libc::c_int as uint64_t,
    0x5e3d7 as libc::c_int as uint64_t,
    0x9e3d7 as libc::c_int as uint64_t,
    0xde3d7 as libc::c_int as uint64_t,
    0x11e3d7 as libc::c_int as uint64_t,
    0x15e3d7 as libc::c_int as uint64_t,
    0x19e3d7 as libc::c_int as uint64_t,
    0x1de3d7 as libc::c_int as uint64_t,
    0x1ebd7 as libc::c_int as uint64_t,
    0x5ebd7 as libc::c_int as uint64_t,
    0x9ebd7 as libc::c_int as uint64_t,
    0xdebd7 as libc::c_int as uint64_t,
    0x11ebd7 as libc::c_int as uint64_t,
    0x15ebd7 as libc::c_int as uint64_t,
    0x19ebd7 as libc::c_int as uint64_t,
    0x1debd7 as libc::c_int as uint64_t,
    0x1f3d7 as libc::c_int as uint64_t,
    0x5f3d7 as libc::c_int as uint64_t,
    0x9f3d7 as libc::c_int as uint64_t,
    0xdf3d7 as libc::c_int as uint64_t,
    0x11f3d7 as libc::c_int as uint64_t,
    0x15f3d7 as libc::c_int as uint64_t,
    0x19f3d7 as libc::c_int as uint64_t,
    0x1df3d7 as libc::c_int as uint64_t,
    0x1fbd7 as libc::c_int as uint64_t,
    0x5fbd7 as libc::c_int as uint64_t,
    0x9fbd7 as libc::c_int as uint64_t,
    0xdfbd7 as libc::c_int as uint64_t,
    0x11fbd7 as libc::c_int as uint64_t,
    0x15fbd7 as libc::c_int as uint64_t,
    0x19fbd7 as libc::c_int as uint64_t,
    0x1dfbd7 as libc::c_int as uint64_t,
    0x1c3e7 as libc::c_int as uint64_t,
    0x5c3e7 as libc::c_int as uint64_t,
    0x9c3e7 as libc::c_int as uint64_t,
    0xdc3e7 as libc::c_int as uint64_t,
    0x11c3e7 as libc::c_int as uint64_t,
    0x15c3e7 as libc::c_int as uint64_t,
    0x19c3e7 as libc::c_int as uint64_t,
    0x1dc3e7 as libc::c_int as uint64_t,
    0x1cbe7 as libc::c_int as uint64_t,
    0x5cbe7 as libc::c_int as uint64_t,
    0x9cbe7 as libc::c_int as uint64_t,
    0xdcbe7 as libc::c_int as uint64_t,
    0x11cbe7 as libc::c_int as uint64_t,
    0x15cbe7 as libc::c_int as uint64_t,
    0x19cbe7 as libc::c_int as uint64_t,
    0x1dcbe7 as libc::c_int as uint64_t,
    0x1d3e7 as libc::c_int as uint64_t,
    0x5d3e7 as libc::c_int as uint64_t,
    0x9d3e7 as libc::c_int as uint64_t,
    0xdd3e7 as libc::c_int as uint64_t,
    0x11d3e7 as libc::c_int as uint64_t,
    0x15d3e7 as libc::c_int as uint64_t,
    0x19d3e7 as libc::c_int as uint64_t,
    0x1dd3e7 as libc::c_int as uint64_t,
    0x1dbe7 as libc::c_int as uint64_t,
    0x5dbe7 as libc::c_int as uint64_t,
    0x9dbe7 as libc::c_int as uint64_t,
    0xddbe7 as libc::c_int as uint64_t,
    0x11dbe7 as libc::c_int as uint64_t,
    0x15dbe7 as libc::c_int as uint64_t,
    0x19dbe7 as libc::c_int as uint64_t,
    0x1ddbe7 as libc::c_int as uint64_t,
    0x1e3e7 as libc::c_int as uint64_t,
    0x5e3e7 as libc::c_int as uint64_t,
    0x9e3e7 as libc::c_int as uint64_t,
    0xde3e7 as libc::c_int as uint64_t,
    0x11e3e7 as libc::c_int as uint64_t,
    0x15e3e7 as libc::c_int as uint64_t,
    0x19e3e7 as libc::c_int as uint64_t,
    0x1de3e7 as libc::c_int as uint64_t,
    0x1ebe7 as libc::c_int as uint64_t,
    0x5ebe7 as libc::c_int as uint64_t,
    0x9ebe7 as libc::c_int as uint64_t,
    0xdebe7 as libc::c_int as uint64_t,
    0x11ebe7 as libc::c_int as uint64_t,
    0x15ebe7 as libc::c_int as uint64_t,
    0x19ebe7 as libc::c_int as uint64_t,
    0x1debe7 as libc::c_int as uint64_t,
    0x1f3e7 as libc::c_int as uint64_t,
    0x5f3e7 as libc::c_int as uint64_t,
    0x9f3e7 as libc::c_int as uint64_t,
    0xdf3e7 as libc::c_int as uint64_t,
    0x11f3e7 as libc::c_int as uint64_t,
    0x15f3e7 as libc::c_int as uint64_t,
    0x19f3e7 as libc::c_int as uint64_t,
    0x1df3e7 as libc::c_int as uint64_t,
    0x1fbe7 as libc::c_int as uint64_t,
    0x5fbe7 as libc::c_int as uint64_t,
    0x9fbe7 as libc::c_int as uint64_t,
    0xdfbe7 as libc::c_int as uint64_t,
    0x11fbe7 as libc::c_int as uint64_t,
    0x15fbe7 as libc::c_int as uint64_t,
    0x19fbe7 as libc::c_int as uint64_t,
    0x1dfbe7 as libc::c_int as uint64_t,
    0x1c3f7 as libc::c_int as uint64_t,
    0x5c3f7 as libc::c_int as uint64_t,
    0x9c3f7 as libc::c_int as uint64_t,
    0xdc3f7 as libc::c_int as uint64_t,
    0x11c3f7 as libc::c_int as uint64_t,
    0x15c3f7 as libc::c_int as uint64_t,
    0x19c3f7 as libc::c_int as uint64_t,
    0x1dc3f7 as libc::c_int as uint64_t,
    0x1cbf7 as libc::c_int as uint64_t,
    0x5cbf7 as libc::c_int as uint64_t,
    0x9cbf7 as libc::c_int as uint64_t,
    0xdcbf7 as libc::c_int as uint64_t,
    0x11cbf7 as libc::c_int as uint64_t,
    0x15cbf7 as libc::c_int as uint64_t,
    0x19cbf7 as libc::c_int as uint64_t,
    0x1dcbf7 as libc::c_int as uint64_t,
    0x1d3f7 as libc::c_int as uint64_t,
    0x5d3f7 as libc::c_int as uint64_t,
    0x9d3f7 as libc::c_int as uint64_t,
    0xdd3f7 as libc::c_int as uint64_t,
    0x11d3f7 as libc::c_int as uint64_t,
    0x15d3f7 as libc::c_int as uint64_t,
    0x19d3f7 as libc::c_int as uint64_t,
    0x1dd3f7 as libc::c_int as uint64_t,
    0x1dbf7 as libc::c_int as uint64_t,
    0x5dbf7 as libc::c_int as uint64_t,
    0x9dbf7 as libc::c_int as uint64_t,
    0xddbf7 as libc::c_int as uint64_t,
    0x11dbf7 as libc::c_int as uint64_t,
    0x15dbf7 as libc::c_int as uint64_t,
    0x19dbf7 as libc::c_int as uint64_t,
    0x1ddbf7 as libc::c_int as uint64_t,
    0x1e3f7 as libc::c_int as uint64_t,
    0x5e3f7 as libc::c_int as uint64_t,
    0x9e3f7 as libc::c_int as uint64_t,
    0xde3f7 as libc::c_int as uint64_t,
    0x11e3f7 as libc::c_int as uint64_t,
    0x15e3f7 as libc::c_int as uint64_t,
    0x19e3f7 as libc::c_int as uint64_t,
    0x1de3f7 as libc::c_int as uint64_t,
    0x1ebf7 as libc::c_int as uint64_t,
    0x5ebf7 as libc::c_int as uint64_t,
    0x9ebf7 as libc::c_int as uint64_t,
    0xdebf7 as libc::c_int as uint64_t,
    0x11ebf7 as libc::c_int as uint64_t,
    0x15ebf7 as libc::c_int as uint64_t,
    0x19ebf7 as libc::c_int as uint64_t,
    0x1debf7 as libc::c_int as uint64_t,
    0x1f3f7 as libc::c_int as uint64_t,
    0x5f3f7 as libc::c_int as uint64_t,
    0x9f3f7 as libc::c_int as uint64_t,
    0xdf3f7 as libc::c_int as uint64_t,
    0x11f3f7 as libc::c_int as uint64_t,
    0x15f3f7 as libc::c_int as uint64_t,
    0x19f3f7 as libc::c_int as uint64_t,
    0x1df3f7 as libc::c_int as uint64_t,
    0x1fbf7 as libc::c_int as uint64_t,
    0x5fbf7 as libc::c_int as uint64_t,
    0x9fbf7 as libc::c_int as uint64_t,
    0xdfbf7 as libc::c_int as uint64_t,
    0x11fbf7 as libc::c_int as uint64_t,
    0x15fbf7 as libc::c_int as uint64_t,
    0x19fbf7 as libc::c_int as uint64_t,
    0x1dfbf7 as libc::c_int as uint64_t,
    0xe1c387 as libc::c_int as uint64_t,
    0x2e1c387 as libc::c_int as uint64_t,
    0x4e1c387 as libc::c_int as uint64_t,
    0x6e1c387 as libc::c_int as uint64_t,
    0x8e1c387 as libc::c_int as uint64_t,
    0xae1c387 as libc::c_int as uint64_t,
    0xce1c387 as libc::c_int as uint64_t,
    0xee1c387 as libc::c_int as uint64_t,
    0xe5c387 as libc::c_int as uint64_t,
    0x2e5c387 as libc::c_int as uint64_t,
    0x4e5c387 as libc::c_int as uint64_t,
    0x6e5c387 as libc::c_int as uint64_t,
    0x8e5c387 as libc::c_int as uint64_t,
    0xae5c387 as libc::c_int as uint64_t,
    0xce5c387 as libc::c_int as uint64_t,
    0xee5c387 as libc::c_int as uint64_t,
    0xe9c387 as libc::c_int as uint64_t,
    0x2e9c387 as libc::c_int as uint64_t,
    0x4e9c387 as libc::c_int as uint64_t,
    0x6e9c387 as libc::c_int as uint64_t,
    0x8e9c387 as libc::c_int as uint64_t,
    0xae9c387 as libc::c_int as uint64_t,
    0xce9c387 as libc::c_int as uint64_t,
    0xee9c387 as libc::c_int as uint64_t,
    0xedc387 as libc::c_int as uint64_t,
    0x2edc387 as libc::c_int as uint64_t,
    0x4edc387 as libc::c_int as uint64_t,
    0x6edc387 as libc::c_int as uint64_t,
    0x8edc387 as libc::c_int as uint64_t,
    0xaedc387 as libc::c_int as uint64_t,
    0xcedc387 as libc::c_int as uint64_t,
    0xeedc387 as libc::c_int as uint64_t,
    0xf1c387 as libc::c_int as uint64_t,
    0x2f1c387 as libc::c_int as uint64_t,
    0x4f1c387 as libc::c_int as uint64_t,
    0x6f1c387 as libc::c_int as uint64_t,
    0x8f1c387 as libc::c_int as uint64_t,
    0xaf1c387 as libc::c_int as uint64_t,
    0xcf1c387 as libc::c_int as uint64_t,
    0xef1c387 as libc::c_int as uint64_t,
    0xf5c387 as libc::c_int as uint64_t,
    0x2f5c387 as libc::c_int as uint64_t,
    0x4f5c387 as libc::c_int as uint64_t,
    0x6f5c387 as libc::c_int as uint64_t,
    0x8f5c387 as libc::c_int as uint64_t,
    0xaf5c387 as libc::c_int as uint64_t,
    0xcf5c387 as libc::c_int as uint64_t,
    0xef5c387 as libc::c_int as uint64_t,
    0xf9c387 as libc::c_int as uint64_t,
    0x2f9c387 as libc::c_int as uint64_t,
    0x4f9c387 as libc::c_int as uint64_t,
    0x6f9c387 as libc::c_int as uint64_t,
    0x8f9c387 as libc::c_int as uint64_t,
    0xaf9c387 as libc::c_int as uint64_t,
    0xcf9c387 as libc::c_int as uint64_t,
    0xef9c387 as libc::c_int as uint64_t,
    0xfdc387 as libc::c_int as uint64_t,
    0x2fdc387 as libc::c_int as uint64_t,
    0x4fdc387 as libc::c_int as uint64_t,
    0x6fdc387 as libc::c_int as uint64_t,
    0x8fdc387 as libc::c_int as uint64_t,
    0xafdc387 as libc::c_int as uint64_t,
    0xcfdc387 as libc::c_int as uint64_t,
    0xefdc387 as libc::c_int as uint64_t,
    0xe1cb87 as libc::c_int as uint64_t,
    0x2e1cb87 as libc::c_int as uint64_t,
    0x4e1cb87 as libc::c_int as uint64_t,
    0x6e1cb87 as libc::c_int as uint64_t,
    0x8e1cb87 as libc::c_int as uint64_t,
    0xae1cb87 as libc::c_int as uint64_t,
    0xce1cb87 as libc::c_int as uint64_t,
    0xee1cb87 as libc::c_int as uint64_t,
    0xe5cb87 as libc::c_int as uint64_t,
    0x2e5cb87 as libc::c_int as uint64_t,
    0x4e5cb87 as libc::c_int as uint64_t,
    0x6e5cb87 as libc::c_int as uint64_t,
    0x8e5cb87 as libc::c_int as uint64_t,
    0xae5cb87 as libc::c_int as uint64_t,
    0xce5cb87 as libc::c_int as uint64_t,
    0xee5cb87 as libc::c_int as uint64_t,
    0xe9cb87 as libc::c_int as uint64_t,
    0x2e9cb87 as libc::c_int as uint64_t,
    0x4e9cb87 as libc::c_int as uint64_t,
    0x6e9cb87 as libc::c_int as uint64_t,
    0x8e9cb87 as libc::c_int as uint64_t,
    0xae9cb87 as libc::c_int as uint64_t,
    0xce9cb87 as libc::c_int as uint64_t,
    0xee9cb87 as libc::c_int as uint64_t,
    0xedcb87 as libc::c_int as uint64_t,
    0x2edcb87 as libc::c_int as uint64_t,
    0x4edcb87 as libc::c_int as uint64_t,
    0x6edcb87 as libc::c_int as uint64_t,
    0x8edcb87 as libc::c_int as uint64_t,
    0xaedcb87 as libc::c_int as uint64_t,
    0xcedcb87 as libc::c_int as uint64_t,
    0xeedcb87 as libc::c_int as uint64_t,
    0xf1cb87 as libc::c_int as uint64_t,
    0x2f1cb87 as libc::c_int as uint64_t,
    0x4f1cb87 as libc::c_int as uint64_t,
    0x6f1cb87 as libc::c_int as uint64_t,
    0x8f1cb87 as libc::c_int as uint64_t,
    0xaf1cb87 as libc::c_int as uint64_t,
    0xcf1cb87 as libc::c_int as uint64_t,
    0xef1cb87 as libc::c_int as uint64_t,
    0xf5cb87 as libc::c_int as uint64_t,
    0x2f5cb87 as libc::c_int as uint64_t,
    0x4f5cb87 as libc::c_int as uint64_t,
    0x6f5cb87 as libc::c_int as uint64_t,
    0x8f5cb87 as libc::c_int as uint64_t,
    0xaf5cb87 as libc::c_int as uint64_t,
    0xcf5cb87 as libc::c_int as uint64_t,
    0xef5cb87 as libc::c_int as uint64_t,
    0xf9cb87 as libc::c_int as uint64_t,
    0x2f9cb87 as libc::c_int as uint64_t,
    0x4f9cb87 as libc::c_int as uint64_t,
    0x6f9cb87 as libc::c_int as uint64_t,
    0x8f9cb87 as libc::c_int as uint64_t,
];
static mut kCodeLengthDepth: [uint8_t; 18] = [
    4 as libc::c_int as uint8_t,
    4 as libc::c_int as uint8_t,
    4 as libc::c_int as uint8_t,
    4 as libc::c_int as uint8_t,
    4 as libc::c_int as uint8_t,
    4 as libc::c_int as uint8_t,
    4 as libc::c_int as uint8_t,
    4 as libc::c_int as uint8_t,
    4 as libc::c_int as uint8_t,
    4 as libc::c_int as uint8_t,
    4 as libc::c_int as uint8_t,
    4 as libc::c_int as uint8_t,
    4 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    4 as libc::c_int as uint8_t,
    4 as libc::c_int as uint8_t,
];
static mut kCodeLengthBits: [uint32_t; 18] = [
    0 as libc::c_int as uint32_t,
    8 as libc::c_int as uint32_t,
    4 as libc::c_int as uint32_t,
    12 as libc::c_int as uint32_t,
    2 as libc::c_int as uint32_t,
    10 as libc::c_int as uint32_t,
    6 as libc::c_int as uint32_t,
    14 as libc::c_int as uint32_t,
    1 as libc::c_int as uint32_t,
    9 as libc::c_int as uint32_t,
    5 as libc::c_int as uint32_t,
    13 as libc::c_int as uint32_t,
    3 as libc::c_int as uint32_t,
    15 as libc::c_int as uint32_t,
    31 as libc::c_int as uint32_t,
    0 as libc::c_int as uint32_t,
    11 as libc::c_int as uint32_t,
    7 as libc::c_int as uint32_t,
];
#[inline(always)]
unsafe extern "C" fn BrotliWriteBitsPrepareStorage(
    mut pos: size_t,
    mut array: *mut uint8_t,
) {
    *array.offset((pos >> 3 as libc::c_int) as isize) = 0 as libc::c_int as uint8_t;
}
static mut kStaticCommandCodeDepth: [uint8_t; 704] = [
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
];
#[inline(always)]
unsafe extern "C" fn StoreStaticCodeLengthCode(
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    BrotliWriteBits(
        40 as libc::c_int as size_t,
        (0xff as libc::c_uint as uint64_t) << 32 as libc::c_int
            | 0x55555554 as libc::c_uint as libc::c_ulong,
        storage_ix,
        storage,
    );
}
static mut kStaticDistanceCodeDepth: [uint8_t; 64] = [
    6 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
];
static mut kNonZeroRepsBits: [uint64_t; 704] = [
    0xb as libc::c_int as uint64_t,
    0x1b as libc::c_int as uint64_t,
    0x2b as libc::c_int as uint64_t,
    0x3b as libc::c_int as uint64_t,
    0x2cb as libc::c_int as uint64_t,
    0x6cb as libc::c_int as uint64_t,
    0xacb as libc::c_int as uint64_t,
    0xecb as libc::c_int as uint64_t,
    0x2db as libc::c_int as uint64_t,
    0x6db as libc::c_int as uint64_t,
    0xadb as libc::c_int as uint64_t,
    0xedb as libc::c_int as uint64_t,
    0x2eb as libc::c_int as uint64_t,
    0x6eb as libc::c_int as uint64_t,
    0xaeb as libc::c_int as uint64_t,
    0xeeb as libc::c_int as uint64_t,
    0x2fb as libc::c_int as uint64_t,
    0x6fb as libc::c_int as uint64_t,
    0xafb as libc::c_int as uint64_t,
    0xefb as libc::c_int as uint64_t,
    0xb2cb as libc::c_int as uint64_t,
    0x1b2cb as libc::c_int as uint64_t,
    0x2b2cb as libc::c_int as uint64_t,
    0x3b2cb as libc::c_int as uint64_t,
    0xb6cb as libc::c_int as uint64_t,
    0x1b6cb as libc::c_int as uint64_t,
    0x2b6cb as libc::c_int as uint64_t,
    0x3b6cb as libc::c_int as uint64_t,
    0xbacb as libc::c_int as uint64_t,
    0x1bacb as libc::c_int as uint64_t,
    0x2bacb as libc::c_int as uint64_t,
    0x3bacb as libc::c_int as uint64_t,
    0xbecb as libc::c_int as uint64_t,
    0x1becb as libc::c_int as uint64_t,
    0x2becb as libc::c_int as uint64_t,
    0x3becb as libc::c_int as uint64_t,
    0xb2db as libc::c_int as uint64_t,
    0x1b2db as libc::c_int as uint64_t,
    0x2b2db as libc::c_int as uint64_t,
    0x3b2db as libc::c_int as uint64_t,
    0xb6db as libc::c_int as uint64_t,
    0x1b6db as libc::c_int as uint64_t,
    0x2b6db as libc::c_int as uint64_t,
    0x3b6db as libc::c_int as uint64_t,
    0xbadb as libc::c_int as uint64_t,
    0x1badb as libc::c_int as uint64_t,
    0x2badb as libc::c_int as uint64_t,
    0x3badb as libc::c_int as uint64_t,
    0xbedb as libc::c_int as uint64_t,
    0x1bedb as libc::c_int as uint64_t,
    0x2bedb as libc::c_int as uint64_t,
    0x3bedb as libc::c_int as uint64_t,
    0xb2eb as libc::c_int as uint64_t,
    0x1b2eb as libc::c_int as uint64_t,
    0x2b2eb as libc::c_int as uint64_t,
    0x3b2eb as libc::c_int as uint64_t,
    0xb6eb as libc::c_int as uint64_t,
    0x1b6eb as libc::c_int as uint64_t,
    0x2b6eb as libc::c_int as uint64_t,
    0x3b6eb as libc::c_int as uint64_t,
    0xbaeb as libc::c_int as uint64_t,
    0x1baeb as libc::c_int as uint64_t,
    0x2baeb as libc::c_int as uint64_t,
    0x3baeb as libc::c_int as uint64_t,
    0xbeeb as libc::c_int as uint64_t,
    0x1beeb as libc::c_int as uint64_t,
    0x2beeb as libc::c_int as uint64_t,
    0x3beeb as libc::c_int as uint64_t,
    0xb2fb as libc::c_int as uint64_t,
    0x1b2fb as libc::c_int as uint64_t,
    0x2b2fb as libc::c_int as uint64_t,
    0x3b2fb as libc::c_int as uint64_t,
    0xb6fb as libc::c_int as uint64_t,
    0x1b6fb as libc::c_int as uint64_t,
    0x2b6fb as libc::c_int as uint64_t,
    0x3b6fb as libc::c_int as uint64_t,
    0xbafb as libc::c_int as uint64_t,
    0x1bafb as libc::c_int as uint64_t,
    0x2bafb as libc::c_int as uint64_t,
    0x3bafb as libc::c_int as uint64_t,
    0xbefb as libc::c_int as uint64_t,
    0x1befb as libc::c_int as uint64_t,
    0x2befb as libc::c_int as uint64_t,
    0x3befb as libc::c_int as uint64_t,
    0x2cb2cb as libc::c_int as uint64_t,
    0x6cb2cb as libc::c_int as uint64_t,
    0xacb2cb as libc::c_int as uint64_t,
    0xecb2cb as libc::c_int as uint64_t,
    0x2db2cb as libc::c_int as uint64_t,
    0x6db2cb as libc::c_int as uint64_t,
    0xadb2cb as libc::c_int as uint64_t,
    0xedb2cb as libc::c_int as uint64_t,
    0x2eb2cb as libc::c_int as uint64_t,
    0x6eb2cb as libc::c_int as uint64_t,
    0xaeb2cb as libc::c_int as uint64_t,
    0xeeb2cb as libc::c_int as uint64_t,
    0x2fb2cb as libc::c_int as uint64_t,
    0x6fb2cb as libc::c_int as uint64_t,
    0xafb2cb as libc::c_int as uint64_t,
    0xefb2cb as libc::c_int as uint64_t,
    0x2cb6cb as libc::c_int as uint64_t,
    0x6cb6cb as libc::c_int as uint64_t,
    0xacb6cb as libc::c_int as uint64_t,
    0xecb6cb as libc::c_int as uint64_t,
    0x2db6cb as libc::c_int as uint64_t,
    0x6db6cb as libc::c_int as uint64_t,
    0xadb6cb as libc::c_int as uint64_t,
    0xedb6cb as libc::c_int as uint64_t,
    0x2eb6cb as libc::c_int as uint64_t,
    0x6eb6cb as libc::c_int as uint64_t,
    0xaeb6cb as libc::c_int as uint64_t,
    0xeeb6cb as libc::c_int as uint64_t,
    0x2fb6cb as libc::c_int as uint64_t,
    0x6fb6cb as libc::c_int as uint64_t,
    0xafb6cb as libc::c_int as uint64_t,
    0xefb6cb as libc::c_int as uint64_t,
    0x2cbacb as libc::c_int as uint64_t,
    0x6cbacb as libc::c_int as uint64_t,
    0xacbacb as libc::c_int as uint64_t,
    0xecbacb as libc::c_int as uint64_t,
    0x2dbacb as libc::c_int as uint64_t,
    0x6dbacb as libc::c_int as uint64_t,
    0xadbacb as libc::c_int as uint64_t,
    0xedbacb as libc::c_int as uint64_t,
    0x2ebacb as libc::c_int as uint64_t,
    0x6ebacb as libc::c_int as uint64_t,
    0xaebacb as libc::c_int as uint64_t,
    0xeebacb as libc::c_int as uint64_t,
    0x2fbacb as libc::c_int as uint64_t,
    0x6fbacb as libc::c_int as uint64_t,
    0xafbacb as libc::c_int as uint64_t,
    0xefbacb as libc::c_int as uint64_t,
    0x2cbecb as libc::c_int as uint64_t,
    0x6cbecb as libc::c_int as uint64_t,
    0xacbecb as libc::c_int as uint64_t,
    0xecbecb as libc::c_int as uint64_t,
    0x2dbecb as libc::c_int as uint64_t,
    0x6dbecb as libc::c_int as uint64_t,
    0xadbecb as libc::c_int as uint64_t,
    0xedbecb as libc::c_int as uint64_t,
    0x2ebecb as libc::c_int as uint64_t,
    0x6ebecb as libc::c_int as uint64_t,
    0xaebecb as libc::c_int as uint64_t,
    0xeebecb as libc::c_int as uint64_t,
    0x2fbecb as libc::c_int as uint64_t,
    0x6fbecb as libc::c_int as uint64_t,
    0xafbecb as libc::c_int as uint64_t,
    0xefbecb as libc::c_int as uint64_t,
    0x2cb2db as libc::c_int as uint64_t,
    0x6cb2db as libc::c_int as uint64_t,
    0xacb2db as libc::c_int as uint64_t,
    0xecb2db as libc::c_int as uint64_t,
    0x2db2db as libc::c_int as uint64_t,
    0x6db2db as libc::c_int as uint64_t,
    0xadb2db as libc::c_int as uint64_t,
    0xedb2db as libc::c_int as uint64_t,
    0x2eb2db as libc::c_int as uint64_t,
    0x6eb2db as libc::c_int as uint64_t,
    0xaeb2db as libc::c_int as uint64_t,
    0xeeb2db as libc::c_int as uint64_t,
    0x2fb2db as libc::c_int as uint64_t,
    0x6fb2db as libc::c_int as uint64_t,
    0xafb2db as libc::c_int as uint64_t,
    0xefb2db as libc::c_int as uint64_t,
    0x2cb6db as libc::c_int as uint64_t,
    0x6cb6db as libc::c_int as uint64_t,
    0xacb6db as libc::c_int as uint64_t,
    0xecb6db as libc::c_int as uint64_t,
    0x2db6db as libc::c_int as uint64_t,
    0x6db6db as libc::c_int as uint64_t,
    0xadb6db as libc::c_int as uint64_t,
    0xedb6db as libc::c_int as uint64_t,
    0x2eb6db as libc::c_int as uint64_t,
    0x6eb6db as libc::c_int as uint64_t,
    0xaeb6db as libc::c_int as uint64_t,
    0xeeb6db as libc::c_int as uint64_t,
    0x2fb6db as libc::c_int as uint64_t,
    0x6fb6db as libc::c_int as uint64_t,
    0xafb6db as libc::c_int as uint64_t,
    0xefb6db as libc::c_int as uint64_t,
    0x2cbadb as libc::c_int as uint64_t,
    0x6cbadb as libc::c_int as uint64_t,
    0xacbadb as libc::c_int as uint64_t,
    0xecbadb as libc::c_int as uint64_t,
    0x2dbadb as libc::c_int as uint64_t,
    0x6dbadb as libc::c_int as uint64_t,
    0xadbadb as libc::c_int as uint64_t,
    0xedbadb as libc::c_int as uint64_t,
    0x2ebadb as libc::c_int as uint64_t,
    0x6ebadb as libc::c_int as uint64_t,
    0xaebadb as libc::c_int as uint64_t,
    0xeebadb as libc::c_int as uint64_t,
    0x2fbadb as libc::c_int as uint64_t,
    0x6fbadb as libc::c_int as uint64_t,
    0xafbadb as libc::c_int as uint64_t,
    0xefbadb as libc::c_int as uint64_t,
    0x2cbedb as libc::c_int as uint64_t,
    0x6cbedb as libc::c_int as uint64_t,
    0xacbedb as libc::c_int as uint64_t,
    0xecbedb as libc::c_int as uint64_t,
    0x2dbedb as libc::c_int as uint64_t,
    0x6dbedb as libc::c_int as uint64_t,
    0xadbedb as libc::c_int as uint64_t,
    0xedbedb as libc::c_int as uint64_t,
    0x2ebedb as libc::c_int as uint64_t,
    0x6ebedb as libc::c_int as uint64_t,
    0xaebedb as libc::c_int as uint64_t,
    0xeebedb as libc::c_int as uint64_t,
    0x2fbedb as libc::c_int as uint64_t,
    0x6fbedb as libc::c_int as uint64_t,
    0xafbedb as libc::c_int as uint64_t,
    0xefbedb as libc::c_int as uint64_t,
    0x2cb2eb as libc::c_int as uint64_t,
    0x6cb2eb as libc::c_int as uint64_t,
    0xacb2eb as libc::c_int as uint64_t,
    0xecb2eb as libc::c_int as uint64_t,
    0x2db2eb as libc::c_int as uint64_t,
    0x6db2eb as libc::c_int as uint64_t,
    0xadb2eb as libc::c_int as uint64_t,
    0xedb2eb as libc::c_int as uint64_t,
    0x2eb2eb as libc::c_int as uint64_t,
    0x6eb2eb as libc::c_int as uint64_t,
    0xaeb2eb as libc::c_int as uint64_t,
    0xeeb2eb as libc::c_int as uint64_t,
    0x2fb2eb as libc::c_int as uint64_t,
    0x6fb2eb as libc::c_int as uint64_t,
    0xafb2eb as libc::c_int as uint64_t,
    0xefb2eb as libc::c_int as uint64_t,
    0x2cb6eb as libc::c_int as uint64_t,
    0x6cb6eb as libc::c_int as uint64_t,
    0xacb6eb as libc::c_int as uint64_t,
    0xecb6eb as libc::c_int as uint64_t,
    0x2db6eb as libc::c_int as uint64_t,
    0x6db6eb as libc::c_int as uint64_t,
    0xadb6eb as libc::c_int as uint64_t,
    0xedb6eb as libc::c_int as uint64_t,
    0x2eb6eb as libc::c_int as uint64_t,
    0x6eb6eb as libc::c_int as uint64_t,
    0xaeb6eb as libc::c_int as uint64_t,
    0xeeb6eb as libc::c_int as uint64_t,
    0x2fb6eb as libc::c_int as uint64_t,
    0x6fb6eb as libc::c_int as uint64_t,
    0xafb6eb as libc::c_int as uint64_t,
    0xefb6eb as libc::c_int as uint64_t,
    0x2cbaeb as libc::c_int as uint64_t,
    0x6cbaeb as libc::c_int as uint64_t,
    0xacbaeb as libc::c_int as uint64_t,
    0xecbaeb as libc::c_int as uint64_t,
    0x2dbaeb as libc::c_int as uint64_t,
    0x6dbaeb as libc::c_int as uint64_t,
    0xadbaeb as libc::c_int as uint64_t,
    0xedbaeb as libc::c_int as uint64_t,
    0x2ebaeb as libc::c_int as uint64_t,
    0x6ebaeb as libc::c_int as uint64_t,
    0xaebaeb as libc::c_int as uint64_t,
    0xeebaeb as libc::c_int as uint64_t,
    0x2fbaeb as libc::c_int as uint64_t,
    0x6fbaeb as libc::c_int as uint64_t,
    0xafbaeb as libc::c_int as uint64_t,
    0xefbaeb as libc::c_int as uint64_t,
    0x2cbeeb as libc::c_int as uint64_t,
    0x6cbeeb as libc::c_int as uint64_t,
    0xacbeeb as libc::c_int as uint64_t,
    0xecbeeb as libc::c_int as uint64_t,
    0x2dbeeb as libc::c_int as uint64_t,
    0x6dbeeb as libc::c_int as uint64_t,
    0xadbeeb as libc::c_int as uint64_t,
    0xedbeeb as libc::c_int as uint64_t,
    0x2ebeeb as libc::c_int as uint64_t,
    0x6ebeeb as libc::c_int as uint64_t,
    0xaebeeb as libc::c_int as uint64_t,
    0xeebeeb as libc::c_int as uint64_t,
    0x2fbeeb as libc::c_int as uint64_t,
    0x6fbeeb as libc::c_int as uint64_t,
    0xafbeeb as libc::c_int as uint64_t,
    0xefbeeb as libc::c_int as uint64_t,
    0x2cb2fb as libc::c_int as uint64_t,
    0x6cb2fb as libc::c_int as uint64_t,
    0xacb2fb as libc::c_int as uint64_t,
    0xecb2fb as libc::c_int as uint64_t,
    0x2db2fb as libc::c_int as uint64_t,
    0x6db2fb as libc::c_int as uint64_t,
    0xadb2fb as libc::c_int as uint64_t,
    0xedb2fb as libc::c_int as uint64_t,
    0x2eb2fb as libc::c_int as uint64_t,
    0x6eb2fb as libc::c_int as uint64_t,
    0xaeb2fb as libc::c_int as uint64_t,
    0xeeb2fb as libc::c_int as uint64_t,
    0x2fb2fb as libc::c_int as uint64_t,
    0x6fb2fb as libc::c_int as uint64_t,
    0xafb2fb as libc::c_int as uint64_t,
    0xefb2fb as libc::c_int as uint64_t,
    0x2cb6fb as libc::c_int as uint64_t,
    0x6cb6fb as libc::c_int as uint64_t,
    0xacb6fb as libc::c_int as uint64_t,
    0xecb6fb as libc::c_int as uint64_t,
    0x2db6fb as libc::c_int as uint64_t,
    0x6db6fb as libc::c_int as uint64_t,
    0xadb6fb as libc::c_int as uint64_t,
    0xedb6fb as libc::c_int as uint64_t,
    0x2eb6fb as libc::c_int as uint64_t,
    0x6eb6fb as libc::c_int as uint64_t,
    0xaeb6fb as libc::c_int as uint64_t,
    0xeeb6fb as libc::c_int as uint64_t,
    0x2fb6fb as libc::c_int as uint64_t,
    0x6fb6fb as libc::c_int as uint64_t,
    0xafb6fb as libc::c_int as uint64_t,
    0xefb6fb as libc::c_int as uint64_t,
    0x2cbafb as libc::c_int as uint64_t,
    0x6cbafb as libc::c_int as uint64_t,
    0xacbafb as libc::c_int as uint64_t,
    0xecbafb as libc::c_int as uint64_t,
    0x2dbafb as libc::c_int as uint64_t,
    0x6dbafb as libc::c_int as uint64_t,
    0xadbafb as libc::c_int as uint64_t,
    0xedbafb as libc::c_int as uint64_t,
    0x2ebafb as libc::c_int as uint64_t,
    0x6ebafb as libc::c_int as uint64_t,
    0xaebafb as libc::c_int as uint64_t,
    0xeebafb as libc::c_int as uint64_t,
    0x2fbafb as libc::c_int as uint64_t,
    0x6fbafb as libc::c_int as uint64_t,
    0xafbafb as libc::c_int as uint64_t,
    0xefbafb as libc::c_int as uint64_t,
    0x2cbefb as libc::c_int as uint64_t,
    0x6cbefb as libc::c_int as uint64_t,
    0xacbefb as libc::c_int as uint64_t,
    0xecbefb as libc::c_int as uint64_t,
    0x2dbefb as libc::c_int as uint64_t,
    0x6dbefb as libc::c_int as uint64_t,
    0xadbefb as libc::c_int as uint64_t,
    0xedbefb as libc::c_int as uint64_t,
    0x2ebefb as libc::c_int as uint64_t,
    0x6ebefb as libc::c_int as uint64_t,
    0xaebefb as libc::c_int as uint64_t,
    0xeebefb as libc::c_int as uint64_t,
    0x2fbefb as libc::c_int as uint64_t,
    0x6fbefb as libc::c_int as uint64_t,
    0xafbefb as libc::c_int as uint64_t,
    0xefbefb as libc::c_int as uint64_t,
    0xb2cb2cb as libc::c_int as uint64_t,
    0x1b2cb2cb as libc::c_int as uint64_t,
    0x2b2cb2cb as libc::c_int as uint64_t,
    0x3b2cb2cb as libc::c_int as uint64_t,
    0xb6cb2cb as libc::c_int as uint64_t,
    0x1b6cb2cb as libc::c_int as uint64_t,
    0x2b6cb2cb as libc::c_int as uint64_t,
    0x3b6cb2cb as libc::c_int as uint64_t,
    0xbacb2cb as libc::c_int as uint64_t,
    0x1bacb2cb as libc::c_int as uint64_t,
    0x2bacb2cb as libc::c_int as uint64_t,
    0x3bacb2cb as libc::c_int as uint64_t,
    0xbecb2cb as libc::c_int as uint64_t,
    0x1becb2cb as libc::c_int as uint64_t,
    0x2becb2cb as libc::c_int as uint64_t,
    0x3becb2cb as libc::c_int as uint64_t,
    0xb2db2cb as libc::c_int as uint64_t,
    0x1b2db2cb as libc::c_int as uint64_t,
    0x2b2db2cb as libc::c_int as uint64_t,
    0x3b2db2cb as libc::c_int as uint64_t,
    0xb6db2cb as libc::c_int as uint64_t,
    0x1b6db2cb as libc::c_int as uint64_t,
    0x2b6db2cb as libc::c_int as uint64_t,
    0x3b6db2cb as libc::c_int as uint64_t,
    0xbadb2cb as libc::c_int as uint64_t,
    0x1badb2cb as libc::c_int as uint64_t,
    0x2badb2cb as libc::c_int as uint64_t,
    0x3badb2cb as libc::c_int as uint64_t,
    0xbedb2cb as libc::c_int as uint64_t,
    0x1bedb2cb as libc::c_int as uint64_t,
    0x2bedb2cb as libc::c_int as uint64_t,
    0x3bedb2cb as libc::c_int as uint64_t,
    0xb2eb2cb as libc::c_int as uint64_t,
    0x1b2eb2cb as libc::c_int as uint64_t,
    0x2b2eb2cb as libc::c_int as uint64_t,
    0x3b2eb2cb as libc::c_int as uint64_t,
    0xb6eb2cb as libc::c_int as uint64_t,
    0x1b6eb2cb as libc::c_int as uint64_t,
    0x2b6eb2cb as libc::c_int as uint64_t,
    0x3b6eb2cb as libc::c_int as uint64_t,
    0xbaeb2cb as libc::c_int as uint64_t,
    0x1baeb2cb as libc::c_int as uint64_t,
    0x2baeb2cb as libc::c_int as uint64_t,
    0x3baeb2cb as libc::c_int as uint64_t,
    0xbeeb2cb as libc::c_int as uint64_t,
    0x1beeb2cb as libc::c_int as uint64_t,
    0x2beeb2cb as libc::c_int as uint64_t,
    0x3beeb2cb as libc::c_int as uint64_t,
    0xb2fb2cb as libc::c_int as uint64_t,
    0x1b2fb2cb as libc::c_int as uint64_t,
    0x2b2fb2cb as libc::c_int as uint64_t,
    0x3b2fb2cb as libc::c_int as uint64_t,
    0xb6fb2cb as libc::c_int as uint64_t,
    0x1b6fb2cb as libc::c_int as uint64_t,
    0x2b6fb2cb as libc::c_int as uint64_t,
    0x3b6fb2cb as libc::c_int as uint64_t,
    0xbafb2cb as libc::c_int as uint64_t,
    0x1bafb2cb as libc::c_int as uint64_t,
    0x2bafb2cb as libc::c_int as uint64_t,
    0x3bafb2cb as libc::c_int as uint64_t,
    0xbefb2cb as libc::c_int as uint64_t,
    0x1befb2cb as libc::c_int as uint64_t,
    0x2befb2cb as libc::c_int as uint64_t,
    0x3befb2cb as libc::c_int as uint64_t,
    0xb2cb6cb as libc::c_int as uint64_t,
    0x1b2cb6cb as libc::c_int as uint64_t,
    0x2b2cb6cb as libc::c_int as uint64_t,
    0x3b2cb6cb as libc::c_int as uint64_t,
    0xb6cb6cb as libc::c_int as uint64_t,
    0x1b6cb6cb as libc::c_int as uint64_t,
    0x2b6cb6cb as libc::c_int as uint64_t,
    0x3b6cb6cb as libc::c_int as uint64_t,
    0xbacb6cb as libc::c_int as uint64_t,
    0x1bacb6cb as libc::c_int as uint64_t,
    0x2bacb6cb as libc::c_int as uint64_t,
    0x3bacb6cb as libc::c_int as uint64_t,
    0xbecb6cb as libc::c_int as uint64_t,
    0x1becb6cb as libc::c_int as uint64_t,
    0x2becb6cb as libc::c_int as uint64_t,
    0x3becb6cb as libc::c_int as uint64_t,
    0xb2db6cb as libc::c_int as uint64_t,
    0x1b2db6cb as libc::c_int as uint64_t,
    0x2b2db6cb as libc::c_int as uint64_t,
    0x3b2db6cb as libc::c_int as uint64_t,
    0xb6db6cb as libc::c_int as uint64_t,
    0x1b6db6cb as libc::c_int as uint64_t,
    0x2b6db6cb as libc::c_int as uint64_t,
    0x3b6db6cb as libc::c_int as uint64_t,
    0xbadb6cb as libc::c_int as uint64_t,
    0x1badb6cb as libc::c_int as uint64_t,
    0x2badb6cb as libc::c_int as uint64_t,
    0x3badb6cb as libc::c_int as uint64_t,
    0xbedb6cb as libc::c_int as uint64_t,
    0x1bedb6cb as libc::c_int as uint64_t,
    0x2bedb6cb as libc::c_int as uint64_t,
    0x3bedb6cb as libc::c_int as uint64_t,
    0xb2eb6cb as libc::c_int as uint64_t,
    0x1b2eb6cb as libc::c_int as uint64_t,
    0x2b2eb6cb as libc::c_int as uint64_t,
    0x3b2eb6cb as libc::c_int as uint64_t,
    0xb6eb6cb as libc::c_int as uint64_t,
    0x1b6eb6cb as libc::c_int as uint64_t,
    0x2b6eb6cb as libc::c_int as uint64_t,
    0x3b6eb6cb as libc::c_int as uint64_t,
    0xbaeb6cb as libc::c_int as uint64_t,
    0x1baeb6cb as libc::c_int as uint64_t,
    0x2baeb6cb as libc::c_int as uint64_t,
    0x3baeb6cb as libc::c_int as uint64_t,
    0xbeeb6cb as libc::c_int as uint64_t,
    0x1beeb6cb as libc::c_int as uint64_t,
    0x2beeb6cb as libc::c_int as uint64_t,
    0x3beeb6cb as libc::c_int as uint64_t,
    0xb2fb6cb as libc::c_int as uint64_t,
    0x1b2fb6cb as libc::c_int as uint64_t,
    0x2b2fb6cb as libc::c_int as uint64_t,
    0x3b2fb6cb as libc::c_int as uint64_t,
    0xb6fb6cb as libc::c_int as uint64_t,
    0x1b6fb6cb as libc::c_int as uint64_t,
    0x2b6fb6cb as libc::c_int as uint64_t,
    0x3b6fb6cb as libc::c_int as uint64_t,
    0xbafb6cb as libc::c_int as uint64_t,
    0x1bafb6cb as libc::c_int as uint64_t,
    0x2bafb6cb as libc::c_int as uint64_t,
    0x3bafb6cb as libc::c_int as uint64_t,
    0xbefb6cb as libc::c_int as uint64_t,
    0x1befb6cb as libc::c_int as uint64_t,
    0x2befb6cb as libc::c_int as uint64_t,
    0x3befb6cb as libc::c_int as uint64_t,
    0xb2cbacb as libc::c_int as uint64_t,
    0x1b2cbacb as libc::c_int as uint64_t,
    0x2b2cbacb as libc::c_int as uint64_t,
    0x3b2cbacb as libc::c_int as uint64_t,
    0xb6cbacb as libc::c_int as uint64_t,
    0x1b6cbacb as libc::c_int as uint64_t,
    0x2b6cbacb as libc::c_int as uint64_t,
    0x3b6cbacb as libc::c_int as uint64_t,
    0xbacbacb as libc::c_int as uint64_t,
    0x1bacbacb as libc::c_int as uint64_t,
    0x2bacbacb as libc::c_int as uint64_t,
    0x3bacbacb as libc::c_int as uint64_t,
    0xbecbacb as libc::c_int as uint64_t,
    0x1becbacb as libc::c_int as uint64_t,
    0x2becbacb as libc::c_int as uint64_t,
    0x3becbacb as libc::c_int as uint64_t,
    0xb2dbacb as libc::c_int as uint64_t,
    0x1b2dbacb as libc::c_int as uint64_t,
    0x2b2dbacb as libc::c_int as uint64_t,
    0x3b2dbacb as libc::c_int as uint64_t,
    0xb6dbacb as libc::c_int as uint64_t,
    0x1b6dbacb as libc::c_int as uint64_t,
    0x2b6dbacb as libc::c_int as uint64_t,
    0x3b6dbacb as libc::c_int as uint64_t,
    0xbadbacb as libc::c_int as uint64_t,
    0x1badbacb as libc::c_int as uint64_t,
    0x2badbacb as libc::c_int as uint64_t,
    0x3badbacb as libc::c_int as uint64_t,
    0xbedbacb as libc::c_int as uint64_t,
    0x1bedbacb as libc::c_int as uint64_t,
    0x2bedbacb as libc::c_int as uint64_t,
    0x3bedbacb as libc::c_int as uint64_t,
    0xb2ebacb as libc::c_int as uint64_t,
    0x1b2ebacb as libc::c_int as uint64_t,
    0x2b2ebacb as libc::c_int as uint64_t,
    0x3b2ebacb as libc::c_int as uint64_t,
    0xb6ebacb as libc::c_int as uint64_t,
    0x1b6ebacb as libc::c_int as uint64_t,
    0x2b6ebacb as libc::c_int as uint64_t,
    0x3b6ebacb as libc::c_int as uint64_t,
    0xbaebacb as libc::c_int as uint64_t,
    0x1baebacb as libc::c_int as uint64_t,
    0x2baebacb as libc::c_int as uint64_t,
    0x3baebacb as libc::c_int as uint64_t,
    0xbeebacb as libc::c_int as uint64_t,
    0x1beebacb as libc::c_int as uint64_t,
    0x2beebacb as libc::c_int as uint64_t,
    0x3beebacb as libc::c_int as uint64_t,
    0xb2fbacb as libc::c_int as uint64_t,
    0x1b2fbacb as libc::c_int as uint64_t,
    0x2b2fbacb as libc::c_int as uint64_t,
    0x3b2fbacb as libc::c_int as uint64_t,
    0xb6fbacb as libc::c_int as uint64_t,
    0x1b6fbacb as libc::c_int as uint64_t,
    0x2b6fbacb as libc::c_int as uint64_t,
    0x3b6fbacb as libc::c_int as uint64_t,
    0xbafbacb as libc::c_int as uint64_t,
    0x1bafbacb as libc::c_int as uint64_t,
    0x2bafbacb as libc::c_int as uint64_t,
    0x3bafbacb as libc::c_int as uint64_t,
    0xbefbacb as libc::c_int as uint64_t,
    0x1befbacb as libc::c_int as uint64_t,
    0x2befbacb as libc::c_int as uint64_t,
    0x3befbacb as libc::c_int as uint64_t,
    0xb2cbecb as libc::c_int as uint64_t,
    0x1b2cbecb as libc::c_int as uint64_t,
    0x2b2cbecb as libc::c_int as uint64_t,
    0x3b2cbecb as libc::c_int as uint64_t,
    0xb6cbecb as libc::c_int as uint64_t,
    0x1b6cbecb as libc::c_int as uint64_t,
    0x2b6cbecb as libc::c_int as uint64_t,
    0x3b6cbecb as libc::c_int as uint64_t,
    0xbacbecb as libc::c_int as uint64_t,
    0x1bacbecb as libc::c_int as uint64_t,
    0x2bacbecb as libc::c_int as uint64_t,
    0x3bacbecb as libc::c_int as uint64_t,
    0xbecbecb as libc::c_int as uint64_t,
    0x1becbecb as libc::c_int as uint64_t,
    0x2becbecb as libc::c_int as uint64_t,
    0x3becbecb as libc::c_int as uint64_t,
    0xb2dbecb as libc::c_int as uint64_t,
    0x1b2dbecb as libc::c_int as uint64_t,
    0x2b2dbecb as libc::c_int as uint64_t,
    0x3b2dbecb as libc::c_int as uint64_t,
    0xb6dbecb as libc::c_int as uint64_t,
    0x1b6dbecb as libc::c_int as uint64_t,
    0x2b6dbecb as libc::c_int as uint64_t,
    0x3b6dbecb as libc::c_int as uint64_t,
    0xbadbecb as libc::c_int as uint64_t,
    0x1badbecb as libc::c_int as uint64_t,
    0x2badbecb as libc::c_int as uint64_t,
    0x3badbecb as libc::c_int as uint64_t,
    0xbedbecb as libc::c_int as uint64_t,
    0x1bedbecb as libc::c_int as uint64_t,
    0x2bedbecb as libc::c_int as uint64_t,
    0x3bedbecb as libc::c_int as uint64_t,
    0xb2ebecb as libc::c_int as uint64_t,
    0x1b2ebecb as libc::c_int as uint64_t,
    0x2b2ebecb as libc::c_int as uint64_t,
    0x3b2ebecb as libc::c_int as uint64_t,
    0xb6ebecb as libc::c_int as uint64_t,
    0x1b6ebecb as libc::c_int as uint64_t,
    0x2b6ebecb as libc::c_int as uint64_t,
    0x3b6ebecb as libc::c_int as uint64_t,
    0xbaebecb as libc::c_int as uint64_t,
    0x1baebecb as libc::c_int as uint64_t,
    0x2baebecb as libc::c_int as uint64_t,
    0x3baebecb as libc::c_int as uint64_t,
    0xbeebecb as libc::c_int as uint64_t,
    0x1beebecb as libc::c_int as uint64_t,
    0x2beebecb as libc::c_int as uint64_t,
    0x3beebecb as libc::c_int as uint64_t,
    0xb2fbecb as libc::c_int as uint64_t,
    0x1b2fbecb as libc::c_int as uint64_t,
    0x2b2fbecb as libc::c_int as uint64_t,
    0x3b2fbecb as libc::c_int as uint64_t,
    0xb6fbecb as libc::c_int as uint64_t,
    0x1b6fbecb as libc::c_int as uint64_t,
    0x2b6fbecb as libc::c_int as uint64_t,
    0x3b6fbecb as libc::c_int as uint64_t,
    0xbafbecb as libc::c_int as uint64_t,
    0x1bafbecb as libc::c_int as uint64_t,
    0x2bafbecb as libc::c_int as uint64_t,
    0x3bafbecb as libc::c_int as uint64_t,
    0xbefbecb as libc::c_int as uint64_t,
    0x1befbecb as libc::c_int as uint64_t,
    0x2befbecb as libc::c_int as uint64_t,
    0x3befbecb as libc::c_int as uint64_t,
    0xb2cb2db as libc::c_int as uint64_t,
    0x1b2cb2db as libc::c_int as uint64_t,
    0x2b2cb2db as libc::c_int as uint64_t,
    0x3b2cb2db as libc::c_int as uint64_t,
    0xb6cb2db as libc::c_int as uint64_t,
    0x1b6cb2db as libc::c_int as uint64_t,
    0x2b6cb2db as libc::c_int as uint64_t,
    0x3b6cb2db as libc::c_int as uint64_t,
    0xbacb2db as libc::c_int as uint64_t,
    0x1bacb2db as libc::c_int as uint64_t,
    0x2bacb2db as libc::c_int as uint64_t,
    0x3bacb2db as libc::c_int as uint64_t,
    0xbecb2db as libc::c_int as uint64_t,
    0x1becb2db as libc::c_int as uint64_t,
    0x2becb2db as libc::c_int as uint64_t,
    0x3becb2db as libc::c_int as uint64_t,
    0xb2db2db as libc::c_int as uint64_t,
    0x1b2db2db as libc::c_int as uint64_t,
    0x2b2db2db as libc::c_int as uint64_t,
    0x3b2db2db as libc::c_int as uint64_t,
    0xb6db2db as libc::c_int as uint64_t,
    0x1b6db2db as libc::c_int as uint64_t,
    0x2b6db2db as libc::c_int as uint64_t,
    0x3b6db2db as libc::c_int as uint64_t,
    0xbadb2db as libc::c_int as uint64_t,
    0x1badb2db as libc::c_int as uint64_t,
    0x2badb2db as libc::c_int as uint64_t,
    0x3badb2db as libc::c_int as uint64_t,
    0xbedb2db as libc::c_int as uint64_t,
    0x1bedb2db as libc::c_int as uint64_t,
    0x2bedb2db as libc::c_int as uint64_t,
    0x3bedb2db as libc::c_int as uint64_t,
    0xb2eb2db as libc::c_int as uint64_t,
    0x1b2eb2db as libc::c_int as uint64_t,
    0x2b2eb2db as libc::c_int as uint64_t,
    0x3b2eb2db as libc::c_int as uint64_t,
    0xb6eb2db as libc::c_int as uint64_t,
    0x1b6eb2db as libc::c_int as uint64_t,
    0x2b6eb2db as libc::c_int as uint64_t,
    0x3b6eb2db as libc::c_int as uint64_t,
    0xbaeb2db as libc::c_int as uint64_t,
    0x1baeb2db as libc::c_int as uint64_t,
    0x2baeb2db as libc::c_int as uint64_t,
    0x3baeb2db as libc::c_int as uint64_t,
    0xbeeb2db as libc::c_int as uint64_t,
    0x1beeb2db as libc::c_int as uint64_t,
    0x2beeb2db as libc::c_int as uint64_t,
    0x3beeb2db as libc::c_int as uint64_t,
    0xb2fb2db as libc::c_int as uint64_t,
    0x1b2fb2db as libc::c_int as uint64_t,
    0x2b2fb2db as libc::c_int as uint64_t,
    0x3b2fb2db as libc::c_int as uint64_t,
    0xb6fb2db as libc::c_int as uint64_t,
    0x1b6fb2db as libc::c_int as uint64_t,
    0x2b6fb2db as libc::c_int as uint64_t,
    0x3b6fb2db as libc::c_int as uint64_t,
    0xbafb2db as libc::c_int as uint64_t,
    0x1bafb2db as libc::c_int as uint64_t,
    0x2bafb2db as libc::c_int as uint64_t,
    0x3bafb2db as libc::c_int as uint64_t,
    0xbefb2db as libc::c_int as uint64_t,
    0x1befb2db as libc::c_int as uint64_t,
    0x2befb2db as libc::c_int as uint64_t,
    0x3befb2db as libc::c_int as uint64_t,
    0xb2cb6db as libc::c_int as uint64_t,
    0x1b2cb6db as libc::c_int as uint64_t,
    0x2b2cb6db as libc::c_int as uint64_t,
    0x3b2cb6db as libc::c_int as uint64_t,
    0xb6cb6db as libc::c_int as uint64_t,
    0x1b6cb6db as libc::c_int as uint64_t,
    0x2b6cb6db as libc::c_int as uint64_t,
    0x3b6cb6db as libc::c_int as uint64_t,
    0xbacb6db as libc::c_int as uint64_t,
    0x1bacb6db as libc::c_int as uint64_t,
    0x2bacb6db as libc::c_int as uint64_t,
    0x3bacb6db as libc::c_int as uint64_t,
    0xbecb6db as libc::c_int as uint64_t,
    0x1becb6db as libc::c_int as uint64_t,
    0x2becb6db as libc::c_int as uint64_t,
    0x3becb6db as libc::c_int as uint64_t,
    0xb2db6db as libc::c_int as uint64_t,
    0x1b2db6db as libc::c_int as uint64_t,
    0x2b2db6db as libc::c_int as uint64_t,
    0x3b2db6db as libc::c_int as uint64_t,
    0xb6db6db as libc::c_int as uint64_t,
    0x1b6db6db as libc::c_int as uint64_t,
    0x2b6db6db as libc::c_int as uint64_t,
    0x3b6db6db as libc::c_int as uint64_t,
    0xbadb6db as libc::c_int as uint64_t,
    0x1badb6db as libc::c_int as uint64_t,
    0x2badb6db as libc::c_int as uint64_t,
    0x3badb6db as libc::c_int as uint64_t,
    0xbedb6db as libc::c_int as uint64_t,
    0x1bedb6db as libc::c_int as uint64_t,
    0x2bedb6db as libc::c_int as uint64_t,
    0x3bedb6db as libc::c_int as uint64_t,
    0xb2eb6db as libc::c_int as uint64_t,
    0x1b2eb6db as libc::c_int as uint64_t,
    0x2b2eb6db as libc::c_int as uint64_t,
    0x3b2eb6db as libc::c_int as uint64_t,
    0xb6eb6db as libc::c_int as uint64_t,
    0x1b6eb6db as libc::c_int as uint64_t,
    0x2b6eb6db as libc::c_int as uint64_t,
    0x3b6eb6db as libc::c_int as uint64_t,
    0xbaeb6db as libc::c_int as uint64_t,
    0x1baeb6db as libc::c_int as uint64_t,
    0x2baeb6db as libc::c_int as uint64_t,
    0x3baeb6db as libc::c_int as uint64_t,
];
static mut kNonZeroRepsDepth: [uint32_t; 704] = [
    6 as libc::c_int as uint32_t,
    6 as libc::c_int as uint32_t,
    6 as libc::c_int as uint32_t,
    6 as libc::c_int as uint32_t,
    12 as libc::c_int as uint32_t,
    12 as libc::c_int as uint32_t,
    12 as libc::c_int as uint32_t,
    12 as libc::c_int as uint32_t,
    12 as libc::c_int as uint32_t,
    12 as libc::c_int as uint32_t,
    12 as libc::c_int as uint32_t,
    12 as libc::c_int as uint32_t,
    12 as libc::c_int as uint32_t,
    12 as libc::c_int as uint32_t,
    12 as libc::c_int as uint32_t,
    12 as libc::c_int as uint32_t,
    12 as libc::c_int as uint32_t,
    12 as libc::c_int as uint32_t,
    12 as libc::c_int as uint32_t,
    12 as libc::c_int as uint32_t,
    18 as libc::c_int as uint32_t,
    18 as libc::c_int as uint32_t,
    18 as libc::c_int as uint32_t,
    18 as libc::c_int as uint32_t,
    18 as libc::c_int as uint32_t,
    18 as libc::c_int as uint32_t,
    18 as libc::c_int as uint32_t,
    18 as libc::c_int as uint32_t,
    18 as libc::c_int as uint32_t,
    18 as libc::c_int as uint32_t,
    18 as libc::c_int as uint32_t,
    18 as libc::c_int as uint32_t,
    18 as libc::c_int as uint32_t,
    18 as libc::c_int as uint32_t,
    18 as libc::c_int as uint32_t,
    18 as libc::c_int as uint32_t,
    18 as libc::c_int as uint32_t,
    18 as libc::c_int as uint32_t,
    18 as libc::c_int as uint32_t,
    18 as libc::c_int as uint32_t,
    18 as libc::c_int as uint32_t,
    18 as libc::c_int as uint32_t,
    18 as libc::c_int as uint32_t,
    18 as libc::c_int as uint32_t,
    18 as libc::c_int as uint32_t,
    18 as libc::c_int as uint32_t,
    18 as libc::c_int as uint32_t,
    18 as libc::c_int as uint32_t,
    18 as libc::c_int as uint32_t,
    18 as libc::c_int as uint32_t,
    18 as libc::c_int as uint32_t,
    18 as libc::c_int as uint32_t,
    18 as libc::c_int as uint32_t,
    18 as libc::c_int as uint32_t,
    18 as libc::c_int as uint32_t,
    18 as libc::c_int as uint32_t,
    18 as libc::c_int as uint32_t,
    18 as libc::c_int as uint32_t,
    18 as libc::c_int as uint32_t,
    18 as libc::c_int as uint32_t,
    18 as libc::c_int as uint32_t,
    18 as libc::c_int as uint32_t,
    18 as libc::c_int as uint32_t,
    18 as libc::c_int as uint32_t,
    18 as libc::c_int as uint32_t,
    18 as libc::c_int as uint32_t,
    18 as libc::c_int as uint32_t,
    18 as libc::c_int as uint32_t,
    18 as libc::c_int as uint32_t,
    18 as libc::c_int as uint32_t,
    18 as libc::c_int as uint32_t,
    18 as libc::c_int as uint32_t,
    18 as libc::c_int as uint32_t,
    18 as libc::c_int as uint32_t,
    18 as libc::c_int as uint32_t,
    18 as libc::c_int as uint32_t,
    18 as libc::c_int as uint32_t,
    18 as libc::c_int as uint32_t,
    18 as libc::c_int as uint32_t,
    18 as libc::c_int as uint32_t,
    18 as libc::c_int as uint32_t,
    18 as libc::c_int as uint32_t,
    18 as libc::c_int as uint32_t,
    18 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    24 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
    30 as libc::c_int as uint32_t,
];
static mut kStaticCommandCodeBits: [uint16_t; 704] = [
    0 as libc::c_int as uint16_t,
    256 as libc::c_int as uint16_t,
    128 as libc::c_int as uint16_t,
    384 as libc::c_int as uint16_t,
    64 as libc::c_int as uint16_t,
    320 as libc::c_int as uint16_t,
    192 as libc::c_int as uint16_t,
    448 as libc::c_int as uint16_t,
    32 as libc::c_int as uint16_t,
    288 as libc::c_int as uint16_t,
    160 as libc::c_int as uint16_t,
    416 as libc::c_int as uint16_t,
    96 as libc::c_int as uint16_t,
    352 as libc::c_int as uint16_t,
    224 as libc::c_int as uint16_t,
    480 as libc::c_int as uint16_t,
    16 as libc::c_int as uint16_t,
    272 as libc::c_int as uint16_t,
    144 as libc::c_int as uint16_t,
    400 as libc::c_int as uint16_t,
    80 as libc::c_int as uint16_t,
    336 as libc::c_int as uint16_t,
    208 as libc::c_int as uint16_t,
    464 as libc::c_int as uint16_t,
    48 as libc::c_int as uint16_t,
    304 as libc::c_int as uint16_t,
    176 as libc::c_int as uint16_t,
    432 as libc::c_int as uint16_t,
    112 as libc::c_int as uint16_t,
    368 as libc::c_int as uint16_t,
    240 as libc::c_int as uint16_t,
    496 as libc::c_int as uint16_t,
    8 as libc::c_int as uint16_t,
    264 as libc::c_int as uint16_t,
    136 as libc::c_int as uint16_t,
    392 as libc::c_int as uint16_t,
    72 as libc::c_int as uint16_t,
    328 as libc::c_int as uint16_t,
    200 as libc::c_int as uint16_t,
    456 as libc::c_int as uint16_t,
    40 as libc::c_int as uint16_t,
    296 as libc::c_int as uint16_t,
    168 as libc::c_int as uint16_t,
    424 as libc::c_int as uint16_t,
    104 as libc::c_int as uint16_t,
    360 as libc::c_int as uint16_t,
    232 as libc::c_int as uint16_t,
    488 as libc::c_int as uint16_t,
    24 as libc::c_int as uint16_t,
    280 as libc::c_int as uint16_t,
    152 as libc::c_int as uint16_t,
    408 as libc::c_int as uint16_t,
    88 as libc::c_int as uint16_t,
    344 as libc::c_int as uint16_t,
    216 as libc::c_int as uint16_t,
    472 as libc::c_int as uint16_t,
    56 as libc::c_int as uint16_t,
    312 as libc::c_int as uint16_t,
    184 as libc::c_int as uint16_t,
    440 as libc::c_int as uint16_t,
    120 as libc::c_int as uint16_t,
    376 as libc::c_int as uint16_t,
    248 as libc::c_int as uint16_t,
    504 as libc::c_int as uint16_t,
    4 as libc::c_int as uint16_t,
    260 as libc::c_int as uint16_t,
    132 as libc::c_int as uint16_t,
    388 as libc::c_int as uint16_t,
    68 as libc::c_int as uint16_t,
    324 as libc::c_int as uint16_t,
    196 as libc::c_int as uint16_t,
    452 as libc::c_int as uint16_t,
    36 as libc::c_int as uint16_t,
    292 as libc::c_int as uint16_t,
    164 as libc::c_int as uint16_t,
    420 as libc::c_int as uint16_t,
    100 as libc::c_int as uint16_t,
    356 as libc::c_int as uint16_t,
    228 as libc::c_int as uint16_t,
    484 as libc::c_int as uint16_t,
    20 as libc::c_int as uint16_t,
    276 as libc::c_int as uint16_t,
    148 as libc::c_int as uint16_t,
    404 as libc::c_int as uint16_t,
    84 as libc::c_int as uint16_t,
    340 as libc::c_int as uint16_t,
    212 as libc::c_int as uint16_t,
    468 as libc::c_int as uint16_t,
    52 as libc::c_int as uint16_t,
    308 as libc::c_int as uint16_t,
    180 as libc::c_int as uint16_t,
    436 as libc::c_int as uint16_t,
    116 as libc::c_int as uint16_t,
    372 as libc::c_int as uint16_t,
    244 as libc::c_int as uint16_t,
    500 as libc::c_int as uint16_t,
    12 as libc::c_int as uint16_t,
    268 as libc::c_int as uint16_t,
    140 as libc::c_int as uint16_t,
    396 as libc::c_int as uint16_t,
    76 as libc::c_int as uint16_t,
    332 as libc::c_int as uint16_t,
    204 as libc::c_int as uint16_t,
    460 as libc::c_int as uint16_t,
    44 as libc::c_int as uint16_t,
    300 as libc::c_int as uint16_t,
    172 as libc::c_int as uint16_t,
    428 as libc::c_int as uint16_t,
    108 as libc::c_int as uint16_t,
    364 as libc::c_int as uint16_t,
    236 as libc::c_int as uint16_t,
    492 as libc::c_int as uint16_t,
    28 as libc::c_int as uint16_t,
    284 as libc::c_int as uint16_t,
    156 as libc::c_int as uint16_t,
    412 as libc::c_int as uint16_t,
    92 as libc::c_int as uint16_t,
    348 as libc::c_int as uint16_t,
    220 as libc::c_int as uint16_t,
    476 as libc::c_int as uint16_t,
    60 as libc::c_int as uint16_t,
    316 as libc::c_int as uint16_t,
    188 as libc::c_int as uint16_t,
    444 as libc::c_int as uint16_t,
    124 as libc::c_int as uint16_t,
    380 as libc::c_int as uint16_t,
    252 as libc::c_int as uint16_t,
    508 as libc::c_int as uint16_t,
    2 as libc::c_int as uint16_t,
    258 as libc::c_int as uint16_t,
    130 as libc::c_int as uint16_t,
    386 as libc::c_int as uint16_t,
    66 as libc::c_int as uint16_t,
    322 as libc::c_int as uint16_t,
    194 as libc::c_int as uint16_t,
    450 as libc::c_int as uint16_t,
    34 as libc::c_int as uint16_t,
    290 as libc::c_int as uint16_t,
    162 as libc::c_int as uint16_t,
    418 as libc::c_int as uint16_t,
    98 as libc::c_int as uint16_t,
    354 as libc::c_int as uint16_t,
    226 as libc::c_int as uint16_t,
    482 as libc::c_int as uint16_t,
    18 as libc::c_int as uint16_t,
    274 as libc::c_int as uint16_t,
    146 as libc::c_int as uint16_t,
    402 as libc::c_int as uint16_t,
    82 as libc::c_int as uint16_t,
    338 as libc::c_int as uint16_t,
    210 as libc::c_int as uint16_t,
    466 as libc::c_int as uint16_t,
    50 as libc::c_int as uint16_t,
    306 as libc::c_int as uint16_t,
    178 as libc::c_int as uint16_t,
    434 as libc::c_int as uint16_t,
    114 as libc::c_int as uint16_t,
    370 as libc::c_int as uint16_t,
    242 as libc::c_int as uint16_t,
    498 as libc::c_int as uint16_t,
    10 as libc::c_int as uint16_t,
    266 as libc::c_int as uint16_t,
    138 as libc::c_int as uint16_t,
    394 as libc::c_int as uint16_t,
    74 as libc::c_int as uint16_t,
    330 as libc::c_int as uint16_t,
    202 as libc::c_int as uint16_t,
    458 as libc::c_int as uint16_t,
    42 as libc::c_int as uint16_t,
    298 as libc::c_int as uint16_t,
    170 as libc::c_int as uint16_t,
    426 as libc::c_int as uint16_t,
    106 as libc::c_int as uint16_t,
    362 as libc::c_int as uint16_t,
    234 as libc::c_int as uint16_t,
    490 as libc::c_int as uint16_t,
    26 as libc::c_int as uint16_t,
    282 as libc::c_int as uint16_t,
    154 as libc::c_int as uint16_t,
    410 as libc::c_int as uint16_t,
    90 as libc::c_int as uint16_t,
    346 as libc::c_int as uint16_t,
    218 as libc::c_int as uint16_t,
    474 as libc::c_int as uint16_t,
    58 as libc::c_int as uint16_t,
    314 as libc::c_int as uint16_t,
    186 as libc::c_int as uint16_t,
    442 as libc::c_int as uint16_t,
    122 as libc::c_int as uint16_t,
    378 as libc::c_int as uint16_t,
    250 as libc::c_int as uint16_t,
    506 as libc::c_int as uint16_t,
    6 as libc::c_int as uint16_t,
    262 as libc::c_int as uint16_t,
    134 as libc::c_int as uint16_t,
    390 as libc::c_int as uint16_t,
    70 as libc::c_int as uint16_t,
    326 as libc::c_int as uint16_t,
    198 as libc::c_int as uint16_t,
    454 as libc::c_int as uint16_t,
    38 as libc::c_int as uint16_t,
    294 as libc::c_int as uint16_t,
    166 as libc::c_int as uint16_t,
    422 as libc::c_int as uint16_t,
    102 as libc::c_int as uint16_t,
    358 as libc::c_int as uint16_t,
    230 as libc::c_int as uint16_t,
    486 as libc::c_int as uint16_t,
    22 as libc::c_int as uint16_t,
    278 as libc::c_int as uint16_t,
    150 as libc::c_int as uint16_t,
    406 as libc::c_int as uint16_t,
    86 as libc::c_int as uint16_t,
    342 as libc::c_int as uint16_t,
    214 as libc::c_int as uint16_t,
    470 as libc::c_int as uint16_t,
    54 as libc::c_int as uint16_t,
    310 as libc::c_int as uint16_t,
    182 as libc::c_int as uint16_t,
    438 as libc::c_int as uint16_t,
    118 as libc::c_int as uint16_t,
    374 as libc::c_int as uint16_t,
    246 as libc::c_int as uint16_t,
    502 as libc::c_int as uint16_t,
    14 as libc::c_int as uint16_t,
    270 as libc::c_int as uint16_t,
    142 as libc::c_int as uint16_t,
    398 as libc::c_int as uint16_t,
    78 as libc::c_int as uint16_t,
    334 as libc::c_int as uint16_t,
    206 as libc::c_int as uint16_t,
    462 as libc::c_int as uint16_t,
    46 as libc::c_int as uint16_t,
    302 as libc::c_int as uint16_t,
    174 as libc::c_int as uint16_t,
    430 as libc::c_int as uint16_t,
    110 as libc::c_int as uint16_t,
    366 as libc::c_int as uint16_t,
    238 as libc::c_int as uint16_t,
    494 as libc::c_int as uint16_t,
    30 as libc::c_int as uint16_t,
    286 as libc::c_int as uint16_t,
    158 as libc::c_int as uint16_t,
    414 as libc::c_int as uint16_t,
    94 as libc::c_int as uint16_t,
    350 as libc::c_int as uint16_t,
    222 as libc::c_int as uint16_t,
    478 as libc::c_int as uint16_t,
    62 as libc::c_int as uint16_t,
    318 as libc::c_int as uint16_t,
    190 as libc::c_int as uint16_t,
    446 as libc::c_int as uint16_t,
    126 as libc::c_int as uint16_t,
    382 as libc::c_int as uint16_t,
    254 as libc::c_int as uint16_t,
    510 as libc::c_int as uint16_t,
    1 as libc::c_int as uint16_t,
    257 as libc::c_int as uint16_t,
    129 as libc::c_int as uint16_t,
    385 as libc::c_int as uint16_t,
    65 as libc::c_int as uint16_t,
    321 as libc::c_int as uint16_t,
    193 as libc::c_int as uint16_t,
    449 as libc::c_int as uint16_t,
    33 as libc::c_int as uint16_t,
    289 as libc::c_int as uint16_t,
    161 as libc::c_int as uint16_t,
    417 as libc::c_int as uint16_t,
    97 as libc::c_int as uint16_t,
    353 as libc::c_int as uint16_t,
    225 as libc::c_int as uint16_t,
    481 as libc::c_int as uint16_t,
    17 as libc::c_int as uint16_t,
    273 as libc::c_int as uint16_t,
    145 as libc::c_int as uint16_t,
    401 as libc::c_int as uint16_t,
    81 as libc::c_int as uint16_t,
    337 as libc::c_int as uint16_t,
    209 as libc::c_int as uint16_t,
    465 as libc::c_int as uint16_t,
    49 as libc::c_int as uint16_t,
    305 as libc::c_int as uint16_t,
    177 as libc::c_int as uint16_t,
    433 as libc::c_int as uint16_t,
    113 as libc::c_int as uint16_t,
    369 as libc::c_int as uint16_t,
    241 as libc::c_int as uint16_t,
    497 as libc::c_int as uint16_t,
    9 as libc::c_int as uint16_t,
    265 as libc::c_int as uint16_t,
    137 as libc::c_int as uint16_t,
    393 as libc::c_int as uint16_t,
    73 as libc::c_int as uint16_t,
    329 as libc::c_int as uint16_t,
    201 as libc::c_int as uint16_t,
    457 as libc::c_int as uint16_t,
    41 as libc::c_int as uint16_t,
    297 as libc::c_int as uint16_t,
    169 as libc::c_int as uint16_t,
    425 as libc::c_int as uint16_t,
    105 as libc::c_int as uint16_t,
    361 as libc::c_int as uint16_t,
    233 as libc::c_int as uint16_t,
    489 as libc::c_int as uint16_t,
    25 as libc::c_int as uint16_t,
    281 as libc::c_int as uint16_t,
    153 as libc::c_int as uint16_t,
    409 as libc::c_int as uint16_t,
    89 as libc::c_int as uint16_t,
    345 as libc::c_int as uint16_t,
    217 as libc::c_int as uint16_t,
    473 as libc::c_int as uint16_t,
    57 as libc::c_int as uint16_t,
    313 as libc::c_int as uint16_t,
    185 as libc::c_int as uint16_t,
    441 as libc::c_int as uint16_t,
    121 as libc::c_int as uint16_t,
    377 as libc::c_int as uint16_t,
    249 as libc::c_int as uint16_t,
    505 as libc::c_int as uint16_t,
    5 as libc::c_int as uint16_t,
    261 as libc::c_int as uint16_t,
    133 as libc::c_int as uint16_t,
    389 as libc::c_int as uint16_t,
    69 as libc::c_int as uint16_t,
    325 as libc::c_int as uint16_t,
    197 as libc::c_int as uint16_t,
    453 as libc::c_int as uint16_t,
    37 as libc::c_int as uint16_t,
    293 as libc::c_int as uint16_t,
    165 as libc::c_int as uint16_t,
    421 as libc::c_int as uint16_t,
    101 as libc::c_int as uint16_t,
    357 as libc::c_int as uint16_t,
    229 as libc::c_int as uint16_t,
    485 as libc::c_int as uint16_t,
    21 as libc::c_int as uint16_t,
    277 as libc::c_int as uint16_t,
    149 as libc::c_int as uint16_t,
    405 as libc::c_int as uint16_t,
    85 as libc::c_int as uint16_t,
    341 as libc::c_int as uint16_t,
    213 as libc::c_int as uint16_t,
    469 as libc::c_int as uint16_t,
    53 as libc::c_int as uint16_t,
    309 as libc::c_int as uint16_t,
    181 as libc::c_int as uint16_t,
    437 as libc::c_int as uint16_t,
    117 as libc::c_int as uint16_t,
    373 as libc::c_int as uint16_t,
    245 as libc::c_int as uint16_t,
    501 as libc::c_int as uint16_t,
    13 as libc::c_int as uint16_t,
    269 as libc::c_int as uint16_t,
    141 as libc::c_int as uint16_t,
    397 as libc::c_int as uint16_t,
    77 as libc::c_int as uint16_t,
    333 as libc::c_int as uint16_t,
    205 as libc::c_int as uint16_t,
    461 as libc::c_int as uint16_t,
    45 as libc::c_int as uint16_t,
    301 as libc::c_int as uint16_t,
    173 as libc::c_int as uint16_t,
    429 as libc::c_int as uint16_t,
    109 as libc::c_int as uint16_t,
    365 as libc::c_int as uint16_t,
    237 as libc::c_int as uint16_t,
    493 as libc::c_int as uint16_t,
    29 as libc::c_int as uint16_t,
    285 as libc::c_int as uint16_t,
    157 as libc::c_int as uint16_t,
    413 as libc::c_int as uint16_t,
    93 as libc::c_int as uint16_t,
    349 as libc::c_int as uint16_t,
    221 as libc::c_int as uint16_t,
    477 as libc::c_int as uint16_t,
    61 as libc::c_int as uint16_t,
    317 as libc::c_int as uint16_t,
    189 as libc::c_int as uint16_t,
    445 as libc::c_int as uint16_t,
    125 as libc::c_int as uint16_t,
    381 as libc::c_int as uint16_t,
    253 as libc::c_int as uint16_t,
    509 as libc::c_int as uint16_t,
    3 as libc::c_int as uint16_t,
    259 as libc::c_int as uint16_t,
    131 as libc::c_int as uint16_t,
    387 as libc::c_int as uint16_t,
    67 as libc::c_int as uint16_t,
    323 as libc::c_int as uint16_t,
    195 as libc::c_int as uint16_t,
    451 as libc::c_int as uint16_t,
    35 as libc::c_int as uint16_t,
    291 as libc::c_int as uint16_t,
    163 as libc::c_int as uint16_t,
    419 as libc::c_int as uint16_t,
    99 as libc::c_int as uint16_t,
    355 as libc::c_int as uint16_t,
    227 as libc::c_int as uint16_t,
    483 as libc::c_int as uint16_t,
    19 as libc::c_int as uint16_t,
    275 as libc::c_int as uint16_t,
    147 as libc::c_int as uint16_t,
    403 as libc::c_int as uint16_t,
    83 as libc::c_int as uint16_t,
    339 as libc::c_int as uint16_t,
    211 as libc::c_int as uint16_t,
    467 as libc::c_int as uint16_t,
    51 as libc::c_int as uint16_t,
    307 as libc::c_int as uint16_t,
    179 as libc::c_int as uint16_t,
    435 as libc::c_int as uint16_t,
    115 as libc::c_int as uint16_t,
    371 as libc::c_int as uint16_t,
    243 as libc::c_int as uint16_t,
    499 as libc::c_int as uint16_t,
    11 as libc::c_int as uint16_t,
    267 as libc::c_int as uint16_t,
    139 as libc::c_int as uint16_t,
    395 as libc::c_int as uint16_t,
    75 as libc::c_int as uint16_t,
    331 as libc::c_int as uint16_t,
    203 as libc::c_int as uint16_t,
    459 as libc::c_int as uint16_t,
    43 as libc::c_int as uint16_t,
    299 as libc::c_int as uint16_t,
    171 as libc::c_int as uint16_t,
    427 as libc::c_int as uint16_t,
    107 as libc::c_int as uint16_t,
    363 as libc::c_int as uint16_t,
    235 as libc::c_int as uint16_t,
    491 as libc::c_int as uint16_t,
    27 as libc::c_int as uint16_t,
    283 as libc::c_int as uint16_t,
    155 as libc::c_int as uint16_t,
    411 as libc::c_int as uint16_t,
    91 as libc::c_int as uint16_t,
    347 as libc::c_int as uint16_t,
    219 as libc::c_int as uint16_t,
    475 as libc::c_int as uint16_t,
    59 as libc::c_int as uint16_t,
    315 as libc::c_int as uint16_t,
    187 as libc::c_int as uint16_t,
    443 as libc::c_int as uint16_t,
    123 as libc::c_int as uint16_t,
    379 as libc::c_int as uint16_t,
    251 as libc::c_int as uint16_t,
    507 as libc::c_int as uint16_t,
    7 as libc::c_int as uint16_t,
    1031 as libc::c_int as uint16_t,
    519 as libc::c_int as uint16_t,
    1543 as libc::c_int as uint16_t,
    263 as libc::c_int as uint16_t,
    1287 as libc::c_int as uint16_t,
    775 as libc::c_int as uint16_t,
    1799 as libc::c_int as uint16_t,
    135 as libc::c_int as uint16_t,
    1159 as libc::c_int as uint16_t,
    647 as libc::c_int as uint16_t,
    1671 as libc::c_int as uint16_t,
    391 as libc::c_int as uint16_t,
    1415 as libc::c_int as uint16_t,
    903 as libc::c_int as uint16_t,
    1927 as libc::c_int as uint16_t,
    71 as libc::c_int as uint16_t,
    1095 as libc::c_int as uint16_t,
    583 as libc::c_int as uint16_t,
    1607 as libc::c_int as uint16_t,
    327 as libc::c_int as uint16_t,
    1351 as libc::c_int as uint16_t,
    839 as libc::c_int as uint16_t,
    1863 as libc::c_int as uint16_t,
    199 as libc::c_int as uint16_t,
    1223 as libc::c_int as uint16_t,
    711 as libc::c_int as uint16_t,
    1735 as libc::c_int as uint16_t,
    455 as libc::c_int as uint16_t,
    1479 as libc::c_int as uint16_t,
    967 as libc::c_int as uint16_t,
    1991 as libc::c_int as uint16_t,
    39 as libc::c_int as uint16_t,
    1063 as libc::c_int as uint16_t,
    551 as libc::c_int as uint16_t,
    1575 as libc::c_int as uint16_t,
    295 as libc::c_int as uint16_t,
    1319 as libc::c_int as uint16_t,
    807 as libc::c_int as uint16_t,
    1831 as libc::c_int as uint16_t,
    167 as libc::c_int as uint16_t,
    1191 as libc::c_int as uint16_t,
    679 as libc::c_int as uint16_t,
    1703 as libc::c_int as uint16_t,
    423 as libc::c_int as uint16_t,
    1447 as libc::c_int as uint16_t,
    935 as libc::c_int as uint16_t,
    1959 as libc::c_int as uint16_t,
    103 as libc::c_int as uint16_t,
    1127 as libc::c_int as uint16_t,
    615 as libc::c_int as uint16_t,
    1639 as libc::c_int as uint16_t,
    359 as libc::c_int as uint16_t,
    1383 as libc::c_int as uint16_t,
    871 as libc::c_int as uint16_t,
    1895 as libc::c_int as uint16_t,
    231 as libc::c_int as uint16_t,
    1255 as libc::c_int as uint16_t,
    743 as libc::c_int as uint16_t,
    1767 as libc::c_int as uint16_t,
    487 as libc::c_int as uint16_t,
    1511 as libc::c_int as uint16_t,
    999 as libc::c_int as uint16_t,
    2023 as libc::c_int as uint16_t,
    23 as libc::c_int as uint16_t,
    1047 as libc::c_int as uint16_t,
    535 as libc::c_int as uint16_t,
    1559 as libc::c_int as uint16_t,
    279 as libc::c_int as uint16_t,
    1303 as libc::c_int as uint16_t,
    791 as libc::c_int as uint16_t,
    1815 as libc::c_int as uint16_t,
    151 as libc::c_int as uint16_t,
    1175 as libc::c_int as uint16_t,
    663 as libc::c_int as uint16_t,
    1687 as libc::c_int as uint16_t,
    407 as libc::c_int as uint16_t,
    1431 as libc::c_int as uint16_t,
    919 as libc::c_int as uint16_t,
    1943 as libc::c_int as uint16_t,
    87 as libc::c_int as uint16_t,
    1111 as libc::c_int as uint16_t,
    599 as libc::c_int as uint16_t,
    1623 as libc::c_int as uint16_t,
    343 as libc::c_int as uint16_t,
    1367 as libc::c_int as uint16_t,
    855 as libc::c_int as uint16_t,
    1879 as libc::c_int as uint16_t,
    215 as libc::c_int as uint16_t,
    1239 as libc::c_int as uint16_t,
    727 as libc::c_int as uint16_t,
    1751 as libc::c_int as uint16_t,
    471 as libc::c_int as uint16_t,
    1495 as libc::c_int as uint16_t,
    983 as libc::c_int as uint16_t,
    2007 as libc::c_int as uint16_t,
    55 as libc::c_int as uint16_t,
    1079 as libc::c_int as uint16_t,
    567 as libc::c_int as uint16_t,
    1591 as libc::c_int as uint16_t,
    311 as libc::c_int as uint16_t,
    1335 as libc::c_int as uint16_t,
    823 as libc::c_int as uint16_t,
    1847 as libc::c_int as uint16_t,
    183 as libc::c_int as uint16_t,
    1207 as libc::c_int as uint16_t,
    695 as libc::c_int as uint16_t,
    1719 as libc::c_int as uint16_t,
    439 as libc::c_int as uint16_t,
    1463 as libc::c_int as uint16_t,
    951 as libc::c_int as uint16_t,
    1975 as libc::c_int as uint16_t,
    119 as libc::c_int as uint16_t,
    1143 as libc::c_int as uint16_t,
    631 as libc::c_int as uint16_t,
    1655 as libc::c_int as uint16_t,
    375 as libc::c_int as uint16_t,
    1399 as libc::c_int as uint16_t,
    887 as libc::c_int as uint16_t,
    1911 as libc::c_int as uint16_t,
    247 as libc::c_int as uint16_t,
    1271 as libc::c_int as uint16_t,
    759 as libc::c_int as uint16_t,
    1783 as libc::c_int as uint16_t,
    503 as libc::c_int as uint16_t,
    1527 as libc::c_int as uint16_t,
    1015 as libc::c_int as uint16_t,
    2039 as libc::c_int as uint16_t,
    15 as libc::c_int as uint16_t,
    1039 as libc::c_int as uint16_t,
    527 as libc::c_int as uint16_t,
    1551 as libc::c_int as uint16_t,
    271 as libc::c_int as uint16_t,
    1295 as libc::c_int as uint16_t,
    783 as libc::c_int as uint16_t,
    1807 as libc::c_int as uint16_t,
    143 as libc::c_int as uint16_t,
    1167 as libc::c_int as uint16_t,
    655 as libc::c_int as uint16_t,
    1679 as libc::c_int as uint16_t,
    399 as libc::c_int as uint16_t,
    1423 as libc::c_int as uint16_t,
    911 as libc::c_int as uint16_t,
    1935 as libc::c_int as uint16_t,
    79 as libc::c_int as uint16_t,
    1103 as libc::c_int as uint16_t,
    591 as libc::c_int as uint16_t,
    1615 as libc::c_int as uint16_t,
    335 as libc::c_int as uint16_t,
    1359 as libc::c_int as uint16_t,
    847 as libc::c_int as uint16_t,
    1871 as libc::c_int as uint16_t,
    207 as libc::c_int as uint16_t,
    1231 as libc::c_int as uint16_t,
    719 as libc::c_int as uint16_t,
    1743 as libc::c_int as uint16_t,
    463 as libc::c_int as uint16_t,
    1487 as libc::c_int as uint16_t,
    975 as libc::c_int as uint16_t,
    1999 as libc::c_int as uint16_t,
    47 as libc::c_int as uint16_t,
    1071 as libc::c_int as uint16_t,
    559 as libc::c_int as uint16_t,
    1583 as libc::c_int as uint16_t,
    303 as libc::c_int as uint16_t,
    1327 as libc::c_int as uint16_t,
    815 as libc::c_int as uint16_t,
    1839 as libc::c_int as uint16_t,
    175 as libc::c_int as uint16_t,
    1199 as libc::c_int as uint16_t,
    687 as libc::c_int as uint16_t,
    1711 as libc::c_int as uint16_t,
    431 as libc::c_int as uint16_t,
    1455 as libc::c_int as uint16_t,
    943 as libc::c_int as uint16_t,
    1967 as libc::c_int as uint16_t,
    111 as libc::c_int as uint16_t,
    1135 as libc::c_int as uint16_t,
    623 as libc::c_int as uint16_t,
    1647 as libc::c_int as uint16_t,
    367 as libc::c_int as uint16_t,
    1391 as libc::c_int as uint16_t,
    879 as libc::c_int as uint16_t,
    1903 as libc::c_int as uint16_t,
    239 as libc::c_int as uint16_t,
    1263 as libc::c_int as uint16_t,
    751 as libc::c_int as uint16_t,
    1775 as libc::c_int as uint16_t,
    495 as libc::c_int as uint16_t,
    1519 as libc::c_int as uint16_t,
    1007 as libc::c_int as uint16_t,
    2031 as libc::c_int as uint16_t,
    31 as libc::c_int as uint16_t,
    1055 as libc::c_int as uint16_t,
    543 as libc::c_int as uint16_t,
    1567 as libc::c_int as uint16_t,
    287 as libc::c_int as uint16_t,
    1311 as libc::c_int as uint16_t,
    799 as libc::c_int as uint16_t,
    1823 as libc::c_int as uint16_t,
    159 as libc::c_int as uint16_t,
    1183 as libc::c_int as uint16_t,
    671 as libc::c_int as uint16_t,
    1695 as libc::c_int as uint16_t,
    415 as libc::c_int as uint16_t,
    1439 as libc::c_int as uint16_t,
    927 as libc::c_int as uint16_t,
    1951 as libc::c_int as uint16_t,
    95 as libc::c_int as uint16_t,
    1119 as libc::c_int as uint16_t,
    607 as libc::c_int as uint16_t,
    1631 as libc::c_int as uint16_t,
    351 as libc::c_int as uint16_t,
    1375 as libc::c_int as uint16_t,
    863 as libc::c_int as uint16_t,
    1887 as libc::c_int as uint16_t,
    223 as libc::c_int as uint16_t,
    1247 as libc::c_int as uint16_t,
    735 as libc::c_int as uint16_t,
    1759 as libc::c_int as uint16_t,
    479 as libc::c_int as uint16_t,
    1503 as libc::c_int as uint16_t,
    991 as libc::c_int as uint16_t,
    2015 as libc::c_int as uint16_t,
    63 as libc::c_int as uint16_t,
    1087 as libc::c_int as uint16_t,
    575 as libc::c_int as uint16_t,
    1599 as libc::c_int as uint16_t,
    319 as libc::c_int as uint16_t,
    1343 as libc::c_int as uint16_t,
    831 as libc::c_int as uint16_t,
    1855 as libc::c_int as uint16_t,
    191 as libc::c_int as uint16_t,
    1215 as libc::c_int as uint16_t,
    703 as libc::c_int as uint16_t,
    1727 as libc::c_int as uint16_t,
    447 as libc::c_int as uint16_t,
    1471 as libc::c_int as uint16_t,
    959 as libc::c_int as uint16_t,
    1983 as libc::c_int as uint16_t,
    127 as libc::c_int as uint16_t,
    1151 as libc::c_int as uint16_t,
    639 as libc::c_int as uint16_t,
    1663 as libc::c_int as uint16_t,
    383 as libc::c_int as uint16_t,
    1407 as libc::c_int as uint16_t,
    895 as libc::c_int as uint16_t,
    1919 as libc::c_int as uint16_t,
    255 as libc::c_int as uint16_t,
    1279 as libc::c_int as uint16_t,
    767 as libc::c_int as uint16_t,
    1791 as libc::c_int as uint16_t,
    511 as libc::c_int as uint16_t,
    1535 as libc::c_int as uint16_t,
    1023 as libc::c_int as uint16_t,
    2047 as libc::c_int as uint16_t,
];
#[inline(always)]
unsafe extern "C" fn StoreStaticCommandHuffmanTree(
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    BrotliWriteBits(
        56 as libc::c_int as size_t,
        (0x926244 as libc::c_uint as uint64_t) << 32 as libc::c_int
            | 0x16307003 as libc::c_uint as libc::c_ulong,
        storage_ix,
        storage,
    );
    BrotliWriteBits(
        3 as libc::c_int as size_t,
        0 as libc::c_uint as uint64_t,
        storage_ix,
        storage,
    );
}
static mut kStaticDistanceCodeBits: [uint16_t; 64] = [
    0 as libc::c_int as uint16_t,
    32 as libc::c_int as uint16_t,
    16 as libc::c_int as uint16_t,
    48 as libc::c_int as uint16_t,
    8 as libc::c_int as uint16_t,
    40 as libc::c_int as uint16_t,
    24 as libc::c_int as uint16_t,
    56 as libc::c_int as uint16_t,
    4 as libc::c_int as uint16_t,
    36 as libc::c_int as uint16_t,
    20 as libc::c_int as uint16_t,
    52 as libc::c_int as uint16_t,
    12 as libc::c_int as uint16_t,
    44 as libc::c_int as uint16_t,
    28 as libc::c_int as uint16_t,
    60 as libc::c_int as uint16_t,
    2 as libc::c_int as uint16_t,
    34 as libc::c_int as uint16_t,
    18 as libc::c_int as uint16_t,
    50 as libc::c_int as uint16_t,
    10 as libc::c_int as uint16_t,
    42 as libc::c_int as uint16_t,
    26 as libc::c_int as uint16_t,
    58 as libc::c_int as uint16_t,
    6 as libc::c_int as uint16_t,
    38 as libc::c_int as uint16_t,
    22 as libc::c_int as uint16_t,
    54 as libc::c_int as uint16_t,
    14 as libc::c_int as uint16_t,
    46 as libc::c_int as uint16_t,
    30 as libc::c_int as uint16_t,
    62 as libc::c_int as uint16_t,
    1 as libc::c_int as uint16_t,
    33 as libc::c_int as uint16_t,
    17 as libc::c_int as uint16_t,
    49 as libc::c_int as uint16_t,
    9 as libc::c_int as uint16_t,
    41 as libc::c_int as uint16_t,
    25 as libc::c_int as uint16_t,
    57 as libc::c_int as uint16_t,
    5 as libc::c_int as uint16_t,
    37 as libc::c_int as uint16_t,
    21 as libc::c_int as uint16_t,
    53 as libc::c_int as uint16_t,
    13 as libc::c_int as uint16_t,
    45 as libc::c_int as uint16_t,
    29 as libc::c_int as uint16_t,
    61 as libc::c_int as uint16_t,
    3 as libc::c_int as uint16_t,
    35 as libc::c_int as uint16_t,
    19 as libc::c_int as uint16_t,
    51 as libc::c_int as uint16_t,
    11 as libc::c_int as uint16_t,
    43 as libc::c_int as uint16_t,
    27 as libc::c_int as uint16_t,
    59 as libc::c_int as uint16_t,
    7 as libc::c_int as uint16_t,
    39 as libc::c_int as uint16_t,
    23 as libc::c_int as uint16_t,
    55 as libc::c_int as uint16_t,
    15 as libc::c_int as uint16_t,
    47 as libc::c_int as uint16_t,
    31 as libc::c_int as uint16_t,
    63 as libc::c_int as uint16_t,
];
#[inline(always)]
unsafe extern "C" fn StoreStaticDistanceHuffmanTree(
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    BrotliWriteBits(
        28 as libc::c_int as size_t,
        0x369dc03 as libc::c_uint as uint64_t,
        storage_ix,
        storage,
    );
}
#[inline(always)]
unsafe extern "C" fn BlockLengthPrefixCode(mut len: uint32_t) -> uint32_t {
    let mut code = (if len >= 177 as libc::c_int as libc::c_uint {
        if len >= 753 as libc::c_int as libc::c_uint {
            20 as libc::c_int
        } else {
            14 as libc::c_int
        }
    } else if len >= 41 as libc::c_int as libc::c_uint {
        7 as libc::c_int
    } else {
        0 as libc::c_int
    }) as uint32_t;
    while code < (26 as libc::c_int - 1 as libc::c_int) as libc::c_uint
        && len
            >= crate::src::enc::brotli_bit_stream::_kBrotliPrefixCodeRanges[code
                    .wrapping_add(1 as libc::c_int as libc::c_uint) as usize].offset as libc::c_uint
    {
        code= code.wrapping_add(1);
    }
    return code;
}
#[inline(always)]
unsafe extern "C" fn GetBlockLengthPrefixCode(
    mut len: uint32_t,
    mut code: Option<&mut size_t>,
    mut n_extra: Option<&mut uint32_t>,
    mut extra: Option<&mut uint32_t>,
) {
    *code.as_deref_mut().unwrap()= BlockLengthPrefixCode(len) as size_t;
    *n_extra.as_deref_mut().unwrap()= crate::src::enc::brotli_bit_stream::_kBrotliPrefixCodeRanges[(*code.as_deref().unwrap()) as usize].nbits as uint32_t;
    *extra.as_deref_mut().unwrap()= len
        .wrapping_sub(crate::src::enc::brotli_bit_stream::_kBrotliPrefixCodeRanges[(*code.as_deref().unwrap()) as usize].offset as libc::c_uint);
}
unsafe extern "C" fn InitBlockTypeCodeCalculator(
    mut self_0: Option<&mut BlockTypeCodeCalculator>,
) {
    (*self_0.as_deref_mut().unwrap()).last_type= 1 as libc::c_int as size_t;
    (*self_0.as_deref_mut().unwrap()).second_last_type= 0 as libc::c_int as size_t;
}
#[inline(always)]
unsafe extern "C" fn NextBlockTypeCode(
    mut calculator: Option<&mut BlockTypeCodeCalculator>,
    mut type_0: uint8_t,
) -> size_t {
    let mut type_code = (if type_0 as libc::c_ulong
        == (*calculator.as_deref().unwrap()).last_type.wrapping_add(1 as libc::c_int as libc::c_ulong)
    {
        1 as libc::c_uint
    } else if type_0 as libc::c_ulong == (*calculator.as_deref().unwrap()).second_last_type {
        0 as libc::c_uint
    } else {
        (type_0 as libc::c_uint).wrapping_add(2 as libc::c_uint)
    }) as size_t;
    (*calculator.as_deref_mut().unwrap()).second_last_type= (*calculator.as_deref().unwrap()).last_type;
    (*calculator.as_deref_mut().unwrap()).last_type= type_0 as size_t;
    return type_code;
}
unsafe extern "C" fn BrotliEncodeMlen(
    mut length: size_t,
    mut bits: Option<&mut uint64_t>,
    mut numbits: Option<&mut size_t>,
    mut nibblesbits: Option<&mut uint64_t>,
) {
    let mut lg = (if length == 1 as libc::c_int as libc::c_ulong {
        1 as libc::c_int as libc::c_uint
    } else {
        (Log2FloorNonZero(
            length.wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t as size_t,
        ))
            .wrapping_add(1 as libc::c_int as libc::c_uint)
    }) as size_t;
    let mut mnibbles = (if lg < 16 as libc::c_int as libc::c_ulong {
        16 as libc::c_int as libc::c_ulong
    } else {
        lg.wrapping_add(3 as libc::c_int as libc::c_ulong)
    })
        .wrapping_div(4 as libc::c_int as libc::c_ulong);
    *nibblesbits.as_deref_mut().unwrap()= mnibbles.wrapping_sub(4 as libc::c_int as libc::c_ulong);
    *numbits.as_deref_mut().unwrap()= mnibbles.wrapping_mul(4 as libc::c_int as libc::c_ulong);
    *bits.as_deref_mut().unwrap()= length.wrapping_sub(1 as libc::c_int as libc::c_ulong);
}
#[inline(always)]
unsafe extern "C" fn StoreCommandExtra(
    mut cmd: *const crate::src::enc::backward_references::Command,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let mut copylen_code = CommandCopyLenCode(cmd);
    let mut inscode = GetInsertLengthCode((*cmd).insert_len_ as size_t);
    let mut copycode = GetCopyLengthCode(copylen_code as size_t);
    let mut insnumextra = GetInsertExtra(inscode);
    let mut insextraval = (*cmd).insert_len_.wrapping_sub(GetInsertBase(inscode))
        as uint64_t;
    let mut copyextraval = copylen_code.wrapping_sub(GetCopyBase(copycode)) as uint64_t;
    let mut bits = copyextraval << insnumextra | insextraval;
    BrotliWriteBits(
        insnumextra.wrapping_add(GetCopyExtra(copycode)) as size_t,
        bits,
        storage_ix,
        storage,
    );
}
unsafe extern "C" fn StoreVarLenUint8(
    mut n: size_t,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    if n == 0 as libc::c_int as libc::c_ulong {
        BrotliWriteBits(
            1 as libc::c_int as size_t,
            0 as libc::c_int as uint64_t,
            storage_ix,
            storage,
        );
    } else {
        let mut nbits = Log2FloorNonZero(n) as size_t;
        BrotliWriteBits(
            1 as libc::c_int as size_t,
            1 as libc::c_int as uint64_t,
            storage_ix,
            storage,
        );
        BrotliWriteBits(3 as libc::c_int as size_t, nbits, storage_ix, storage);
        BrotliWriteBits(
            nbits,
            n.wrapping_sub((1 as libc::c_int as size_t) << nbits),
            storage_ix,
            storage,
        );
    };
}
unsafe extern "C" fn StoreCompressedMetaBlockHeader(
    mut is_final_block: libc::c_int,
    mut length: size_t,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let mut lenbits: uint64_t = 0;
    let mut nlenbits: size_t = 0;
    let mut nibblesbits: uint64_t = 0;
    BrotliWriteBits(
        1 as libc::c_int as size_t,
        is_final_block as uint64_t,
        storage_ix,
        storage,
    );
    if is_final_block != 0 {
        BrotliWriteBits(
            1 as libc::c_int as size_t,
            0 as libc::c_int as uint64_t,
            storage_ix,
            storage,
        );
    }
    BrotliEncodeMlen(length, Some(&mut lenbits), Some(&mut nlenbits), Some(&mut nibblesbits));
    BrotliWriteBits(2 as libc::c_int as size_t, nibblesbits, storage_ix, storage);
    BrotliWriteBits(nlenbits, lenbits, storage_ix, storage);
    if is_final_block == 0 {
        BrotliWriteBits(
            1 as libc::c_int as size_t,
            0 as libc::c_int as uint64_t,
            storage_ix,
            storage,
        );
    }
}
unsafe extern "C" fn BrotliStoreUncompressedMetaBlockHeader(
    mut length: size_t,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let mut lenbits: uint64_t = 0;
    let mut nlenbits: size_t = 0;
    let mut nibblesbits: uint64_t = 0;
    BrotliWriteBits(
        1 as libc::c_int as size_t,
        0 as libc::c_int as uint64_t,
        storage_ix,
        storage,
    );
    BrotliEncodeMlen(length, Some(&mut lenbits), Some(&mut nlenbits), Some(&mut nibblesbits));
    BrotliWriteBits(2 as libc::c_int as size_t, nibblesbits, storage_ix, storage);
    BrotliWriteBits(nlenbits, lenbits, storage_ix, storage);
    BrotliWriteBits(
        1 as libc::c_int as size_t,
        1 as libc::c_int as uint64_t,
        storage_ix,
        storage,
    );
}
unsafe extern "C" fn BrotliStoreHuffmanTreeOfHuffmanTreeToBitMask(
    mut num_codes: libc::c_int,
    mut code_length_bitdepth: *const uint8_t,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    static mut kStorageOrder: [uint8_t; 18] = [
        1 as libc::c_int as uint8_t,
        2 as libc::c_int as uint8_t,
        3 as libc::c_int as uint8_t,
        4 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        5 as libc::c_int as uint8_t,
        17 as libc::c_int as uint8_t,
        6 as libc::c_int as uint8_t,
        16 as libc::c_int as uint8_t,
        7 as libc::c_int as uint8_t,
        8 as libc::c_int as uint8_t,
        9 as libc::c_int as uint8_t,
        10 as libc::c_int as uint8_t,
        11 as libc::c_int as uint8_t,
        12 as libc::c_int as uint8_t,
        13 as libc::c_int as uint8_t,
        14 as libc::c_int as uint8_t,
        15 as libc::c_int as uint8_t,
    ];
    static mut kHuffmanBitLengthHuffmanCodeSymbols: [uint8_t; 6] = [
        0 as libc::c_int as uint8_t,
        7 as libc::c_int as uint8_t,
        3 as libc::c_int as uint8_t,
        2 as libc::c_int as uint8_t,
        1 as libc::c_int as uint8_t,
        15 as libc::c_int as uint8_t,
    ];
    static mut kHuffmanBitLengthHuffmanCodeBitLengths: [uint8_t; 6] = [
        2 as libc::c_int as uint8_t,
        4 as libc::c_int as uint8_t,
        3 as libc::c_int as uint8_t,
        2 as libc::c_int as uint8_t,
        2 as libc::c_int as uint8_t,
        4 as libc::c_int as uint8_t,
    ];
    let mut skip_some = 0 as libc::c_int as size_t;
    let mut codes_to_store = (17 as libc::c_int + 1 as libc::c_int) as size_t;
    if num_codes > 1 as libc::c_int {
        while codes_to_store > 0 as libc::c_int as libc::c_ulong {
            if *code_length_bitdepth
                .offset(
                    kStorageOrder[codes_to_store
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as usize]
                        as isize,
                ) as libc::c_int != 0 as libc::c_int
            {
                break;
            }
            codes_to_store= codes_to_store.wrapping_sub(1);
        }
    }
    if *code_length_bitdepth.offset(kStorageOrder[0 as libc::c_int as usize] as isize)
        as libc::c_int == 0 as libc::c_int
        && *code_length_bitdepth
            .offset(kStorageOrder[1 as libc::c_int as usize] as isize) as libc::c_int
            == 0 as libc::c_int
    {
        skip_some= 2 as libc::c_int as size_t;
        if *code_length_bitdepth
            .offset(kStorageOrder[2 as libc::c_int as usize] as isize) as libc::c_int
            == 0 as libc::c_int
        {
            skip_some= 3 as libc::c_int as size_t;
        }
    }
    BrotliWriteBits(2 as libc::c_int as size_t, skip_some, storage_ix, storage);
    let mut i: size_t = 0;
    i= skip_some;
    while i < codes_to_store {
        let mut l = *code_length_bitdepth.offset(kStorageOrder[i as usize] as isize)
            as size_t;
        BrotliWriteBits(
            kHuffmanBitLengthHuffmanCodeBitLengths[l as usize] as size_t,
            kHuffmanBitLengthHuffmanCodeSymbols[l as usize] as uint64_t,
            storage_ix,
            storage,
        );
        i= i.wrapping_add(1);
    }
}
unsafe extern "C" fn BrotliStoreHuffmanTreeToBitMask(
    mut huffman_tree_size: size_t,
    mut huffman_tree: *const uint8_t,
    mut huffman_tree_extra_bits: *const uint8_t,
    mut code_length_bitdepth: *const uint8_t,
    mut code_length_bitdepth_symbols: *const uint16_t,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let mut i: size_t = 0;
    i= 0 as libc::c_int as size_t;
    while i < huffman_tree_size {
        let mut ix = *huffman_tree.offset(i as isize) as size_t;
        BrotliWriteBits(
            *code_length_bitdepth.offset(ix as isize) as size_t,
            *code_length_bitdepth_symbols.offset(ix as isize) as uint64_t,
            storage_ix,
            storage,
        );
        match ix {
            16 => {
                BrotliWriteBits(
                    2 as libc::c_int as size_t,
                    *huffman_tree_extra_bits.offset(i as isize) as uint64_t,
                    storage_ix,
                    storage,
                );
            }
            17 => {
                BrotliWriteBits(
                    3 as libc::c_int as size_t,
                    *huffman_tree_extra_bits.offset(i as isize) as uint64_t,
                    storage_ix,
                    storage,
                );
            }
            _ => {}
        }
        i= i.wrapping_add(1);
    }
}
unsafe extern "C" fn StoreSimpleHuffmanTree(
    mut depths: *const uint8_t,
    mut symbols: *mut size_t,
    mut num_symbols: size_t,
    mut max_bits: size_t,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    BrotliWriteBits(
        2 as libc::c_int as size_t,
        1 as libc::c_int as uint64_t,
        storage_ix,
        storage,
    );
    BrotliWriteBits(
        2 as libc::c_int as size_t,
        num_symbols.wrapping_sub(1 as libc::c_int as libc::c_ulong),
        storage_ix,
        storage,
    );
    let mut i: size_t = 0;
    i= 0 as libc::c_int as size_t;
    while i < num_symbols {
        let mut j: size_t = 0;
        j= i.wrapping_add(1 as libc::c_int as libc::c_ulong);
        while j < num_symbols {
            if (*depths.offset(*symbols.offset(j as isize) as isize) as libc::c_int)
                < *depths.offset(*symbols.offset(i as isize) as isize) as libc::c_int
            {
                let mut __brotli_swap_tmp = *symbols.offset(j as isize);
                *symbols.offset(j as isize) = *symbols.offset(i as isize);
                *symbols.offset(i as isize) = __brotli_swap_tmp;
            }
            j= j.wrapping_add(1);
        }
        i= i.wrapping_add(1);
    }
    if num_symbols == 2 as libc::c_int as libc::c_ulong {
        BrotliWriteBits(
            max_bits,
            *symbols.offset(0 as libc::c_int as isize),
            storage_ix,
            storage,
        );
        BrotliWriteBits(
            max_bits,
            *symbols.offset(1 as libc::c_int as isize),
            storage_ix,
            storage,
        );
    } else if num_symbols == 3 as libc::c_int as libc::c_ulong {
        BrotliWriteBits(
            max_bits,
            *symbols.offset(0 as libc::c_int as isize),
            storage_ix,
            storage,
        );
        BrotliWriteBits(
            max_bits,
            *symbols.offset(1 as libc::c_int as isize),
            storage_ix,
            storage,
        );
        BrotliWriteBits(
            max_bits,
            *symbols.offset(2 as libc::c_int as isize),
            storage_ix,
            storage,
        );
    } else {
        BrotliWriteBits(
            max_bits,
            *symbols.offset(0 as libc::c_int as isize),
            storage_ix,
            storage,
        );
        BrotliWriteBits(
            max_bits,
            *symbols.offset(1 as libc::c_int as isize),
            storage_ix,
            storage,
        );
        BrotliWriteBits(
            max_bits,
            *symbols.offset(2 as libc::c_int as isize),
            storage_ix,
            storage,
        );
        BrotliWriteBits(
            max_bits,
            *symbols.offset(3 as libc::c_int as isize),
            storage_ix,
            storage,
        );
        BrotliWriteBits(
            1 as libc::c_int as size_t,
            (if *depths.offset(*symbols.offset(0 as libc::c_int as isize) as isize)
                as libc::c_int == 1 as libc::c_int
            {
                1 as libc::c_int
            } else {
                0 as libc::c_int
            }) as uint64_t,
            storage_ix,
            storage,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn BrotliStoreHuffmanTree(
    mut depths: *const uint8_t,
    mut num: size_t,
    mut tree: *mut HuffmanTree,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let mut huffman_tree: [uint8_t; 704] = [0; 704];
    let mut huffman_tree_extra_bits: [uint8_t; 704] = [0; 704];
    let mut huffman_tree_size = 0 as libc::c_int as size_t;
    let mut code_length_bitdepth: [uint8_t; 18] = [
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
    ];
    let mut code_length_bitdepth_symbols: [uint16_t; 18] = [0; 18];
    let mut huffman_tree_histogram: [uint32_t; 18] = [
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
    let mut i: size_t = 0;
    let mut num_codes = 0 as libc::c_int;
    let mut code = 0 as libc::c_int as size_t;
    crate::src::enc::entropy_encode::BrotliWriteHuffmanTree(
        depths,
        num,
        Some(&mut huffman_tree_size),
        huffman_tree.as_mut_ptr(),
        huffman_tree_extra_bits.as_mut_ptr(),
    );
    i= 0 as libc::c_int as size_t;
    while i < huffman_tree_size {
        huffman_tree_histogram[huffman_tree[i as usize]
            as usize]= huffman_tree_histogram[huffman_tree[i as usize] as usize]
            .wrapping_add(1);
        i= i.wrapping_add(1);
    }
    i= 0 as libc::c_int as size_t;
    while i < (17 as libc::c_int + 1 as libc::c_int) as libc::c_ulong {
        if huffman_tree_histogram[i as usize] != 0 {
            if num_codes == 0 as libc::c_int {
                code= i;
                num_codes= 1 as libc::c_int;
            } else if num_codes == 1 as libc::c_int {
                num_codes= 2 as libc::c_int;
                break;
            }
        }
        i= i.wrapping_add(1);
    }
    crate::src::enc::entropy_encode::BrotliCreateHuffmanTree(
        huffman_tree_histogram.as_mut_ptr(),
        (17 as libc::c_int + 1 as libc::c_int) as size_t,
        5 as libc::c_int,
        tree,
        code_length_bitdepth.as_mut_ptr(),
    );
    crate::src::enc::entropy_encode::BrotliConvertBitDepthsToSymbols(
        code_length_bitdepth.as_mut_ptr(),
        (17 as libc::c_int + 1 as libc::c_int) as size_t,
        code_length_bitdepth_symbols.as_mut_ptr(),
    );
    BrotliStoreHuffmanTreeOfHuffmanTreeToBitMask(
        num_codes,
        code_length_bitdepth.as_mut_ptr(),
        storage_ix,
        storage,
    );
    if num_codes == 1 as libc::c_int {
        code_length_bitdepth[code as usize]= 0 as libc::c_int as uint8_t;
    }
    BrotliStoreHuffmanTreeToBitMask(
        huffman_tree_size,
        huffman_tree.as_mut_ptr(),
        huffman_tree_extra_bits.as_mut_ptr(),
        code_length_bitdepth.as_mut_ptr(),
        code_length_bitdepth_symbols.as_mut_ptr(),
        storage_ix,
        storage,
    );
}
unsafe extern "C" fn BuildAndStoreHuffmanTree(
    mut histogram: *const uint32_t,
    mut histogram_length: size_t,
    mut alphabet_size: size_t,
    mut tree: *mut HuffmanTree,
    mut depth: *mut uint8_t,
    mut bits: *mut uint16_t,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let mut count = 0 as libc::c_int as size_t;
    let mut s4: [size_t; 4] = [0 as libc::c_int as size_t, 0, 0, 0];
    let mut i: size_t = 0;
    let mut max_bits = 0 as libc::c_int as size_t;
    i= 0 as libc::c_int as size_t;
    while i < histogram_length {
        if *histogram.offset(i as isize) != 0 {
            if count < 4 as libc::c_int as libc::c_ulong {
                s4[count as usize]= i;
            } else if count > 4 as libc::c_int as libc::c_ulong {
                break;
            }
            count= count.wrapping_add(1);
        }
        i= i.wrapping_add(1);
    }
    let mut max_bits_counter = alphabet_size
        .wrapping_sub(1 as libc::c_int as libc::c_ulong);
    while max_bits_counter != 0 {
        max_bits_counter>>= 1 as libc::c_int;
        max_bits= max_bits.wrapping_add(1);
    }
    if count <= 1 as libc::c_int as libc::c_ulong {
        BrotliWriteBits(
            4 as libc::c_int as size_t,
            1 as libc::c_int as uint64_t,
            storage_ix,
            storage,
        );
        BrotliWriteBits(max_bits, s4[0 as libc::c_int as usize], storage_ix, storage);
        *depth
            .offset(
                s4[0 as libc::c_int as usize] as isize,
            ) = 0 as libc::c_int as uint8_t;
        *bits
            .offset(
                s4[0 as libc::c_int as usize] as isize,
            ) = 0 as libc::c_int as uint16_t;
        return;
    }
    memset(
        depth as *mut libc::c_void,
        0 as libc::c_int,
        histogram_length.wrapping_mul(::std::mem::size_of::<uint8_t>() as libc::c_ulong),
    );
    crate::src::enc::entropy_encode::BrotliCreateHuffmanTree(histogram, histogram_length, 15 as libc::c_int, tree, depth);
    crate::src::enc::entropy_encode::BrotliConvertBitDepthsToSymbols(depth, histogram_length, bits);
    if count <= 4 as libc::c_int as libc::c_ulong {
        StoreSimpleHuffmanTree(
            depth,
            s4.as_mut_ptr(),
            count,
            max_bits,
            storage_ix,
            storage,
        );
    } else {
        BrotliStoreHuffmanTree(depth, histogram_length, tree, storage_ix, storage);
    };
}
#[inline(always)]
unsafe extern "C" fn SortHuffmanTree(
    mut v0: *const HuffmanTree,
    mut v1: *const HuffmanTree,
) -> libc::c_int {
    return if (*v0).total_count_ < (*v1).total_count_ {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn BrotliBuildAndStoreHuffmanTreeFast(
    mut m: *mut crate::src::enc::backward_references_hq::MemoryManager,
    mut histogram: *const uint32_t,
    mut histogram_total: size_t,
    mut max_bits: size_t,
    mut depth: *mut uint8_t,
    mut bits: *mut uint16_t,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let mut count = 0 as libc::c_int as size_t;
    let mut symbols: [size_t; 4] = [0 as libc::c_int as size_t, 0, 0, 0];
    let mut length = 0 as libc::c_int as size_t;
    let mut total = histogram_total;
    while total != 0 as libc::c_int as libc::c_ulong {
        if *histogram.offset(length as isize) != 0 {
            if count < 4 as libc::c_int as libc::c_ulong {
                symbols[count as usize]= length;
            }
            count= count.wrapping_add(1);
            total= (total as libc::c_ulong)
                .wrapping_sub(*histogram.offset(length as isize) as libc::c_ulong)
                as size_t as size_t;
        }
        length= length.wrapping_add(1);
    }
    if count <= 1 as libc::c_int as libc::c_ulong {
        BrotliWriteBits(
            4 as libc::c_int as size_t,
            1 as libc::c_int as uint64_t,
            storage_ix,
            storage,
        );
        BrotliWriteBits(
            max_bits,
            symbols[0 as libc::c_int as usize],
            storage_ix,
            storage,
        );
        *depth
            .offset(
                symbols[0 as libc::c_int as usize] as isize,
            ) = 0 as libc::c_int as uint8_t;
        *bits
            .offset(
                symbols[0 as libc::c_int as usize] as isize,
            ) = 0 as libc::c_int as uint16_t;
        return;
    }
    memset(
        depth as *mut libc::c_void,
        0 as libc::c_int,
        length.wrapping_mul(::std::mem::size_of::<uint8_t>() as libc::c_ulong),
    );
    let max_tree_size = (2 as libc::c_int as libc::c_ulong)
        .wrapping_mul(length)
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    let mut tree = if max_tree_size > 0 as libc::c_int as libc::c_ulong {
        crate::src::enc::memory::BrotliAllocate(
            m,
            max_tree_size
                .wrapping_mul(::std::mem::size_of::<HuffmanTree>() as libc::c_ulong),
        ) as *mut HuffmanTree
    } else {
        0 as *mut HuffmanTree
    };
    let mut count_limit: uint32_t = 0;
    if 0 as libc::c_int != 0 || 0 as libc::c_int != 0 {
        return;
    }
    count_limit= 1 as libc::c_int as uint32_t;
    loop {
        let mut node = tree;
        let mut l: size_t = 0;
        l= length;
        while l != 0 as libc::c_int as libc::c_ulong {
            l= l.wrapping_sub(1);
            if *histogram.offset(l as isize) != 0 {
                if (*histogram.offset(l as isize) >= count_limit) as libc::c_int
                    as libc::c_long != 0
                {
                    InitHuffmanTree(
                        node.as_mut(),
                        *histogram.offset(l as isize),
                        -(1 as libc::c_int) as int16_t,
                        l as int16_t,
                    );
                } else {
                    InitHuffmanTree(
                        node.as_mut(),
                        count_limit,
                        -(1 as libc::c_int) as int16_t,
                        l as int16_t,
                    );
                }
                node= node.offset(1);
            }
        }
        let n = node.offset_from(tree) as libc::c_long as libc::c_int;
        let mut sentinel = HuffmanTree {
            total_count_: 0,
            index_left_: 0,
            index_right_or_value_: 0,
        };
        let mut i = 0 as libc::c_int;
        let mut j = n + 1 as libc::c_int;
        let mut k: libc::c_int = 0;
        SortHuffmanTreeItems(
            tree,
            n as size_t,
            Some(
                SortHuffmanTree
                    as unsafe extern "C" fn(
                        *const HuffmanTree,
                        *const HuffmanTree,
                    ) -> libc::c_int,
            ),
        );
        InitHuffmanTree(
            Some(&mut sentinel),
            !(0 as libc::c_int as uint32_t),
            -(1 as libc::c_int) as int16_t,
            -(1 as libc::c_int) as int16_t,
        );
        let fresh7 = node;
        node= node.offset(1);
        *fresh7= sentinel;
        let fresh8 = node;
        node= node.offset(1);
        *fresh8= sentinel;
        k= n - 1 as libc::c_int;
        while k > 0 as libc::c_int {
            let mut left: libc::c_int = 0;
            let mut right: libc::c_int = 0;
            if (*tree.offset(i as isize)).total_count_
                <= (*tree.offset(j as isize)).total_count_
            {
                left= i;
                i+= 1;
            } else {
                left= j;
                j+= 1;
            }
            if (*tree.offset(i as isize)).total_count_
                <= (*tree.offset(j as isize)).total_count_
            {
                right= i;
                i+= 1;
            } else {
                right= j;
                j+= 1;
            }
            (*node.offset(-(1 as libc::c_int) as isize))
                .total_count_ = ((*tree.offset(left as isize)).total_count_)
                .wrapping_add((*tree.offset(right as isize)).total_count_);
            (*node.offset(-(1 as libc::c_int) as isize)).index_left_ = left as int16_t;
            (*node.offset(-(1 as libc::c_int) as isize))
                .index_right_or_value_ = right as int16_t;
            let fresh9 = node;
            node= node.offset(1);
            *fresh9= sentinel;
            k-= 1;
        }
        if crate::src::enc::entropy_encode::BrotliSetDepth(
            2 as libc::c_int * n - 1 as libc::c_int,
            tree,
            depth,
            14 as libc::c_int,
        ) != 0
        {
            break;
        }
        count_limit= (count_limit as libc::c_uint)
            .wrapping_mul(2 as libc::c_int as libc::c_uint) as uint32_t as uint32_t;
    }
    crate::src::enc::memory::BrotliFree(m, tree as *mut libc::c_void);
    tree= 0 as *mut HuffmanTree;
    crate::src::enc::entropy_encode::BrotliConvertBitDepthsToSymbols(depth, length, bits);
    if count <= 4 as libc::c_int as libc::c_ulong {
        let mut i_0: size_t = 0;
        BrotliWriteBits(
            2 as libc::c_int as size_t,
            1 as libc::c_int as uint64_t,
            storage_ix,
            storage,
        );
        BrotliWriteBits(
            2 as libc::c_int as size_t,
            count.wrapping_sub(1 as libc::c_int as libc::c_ulong),
            storage_ix,
            storage,
        );
        i_0= 0 as libc::c_int as size_t;
        while i_0 < count {
            let mut j_0: size_t = 0;
            j_0= i_0.wrapping_add(1 as libc::c_int as libc::c_ulong);
            while j_0 < count {
                if (*depth.offset(symbols[j_0 as usize] as isize) as libc::c_int)
                    < *depth.offset(symbols[i_0 as usize] as isize) as libc::c_int
                {
                    let mut __brotli_swap_tmp = symbols[j_0 as usize];
                    symbols[j_0 as usize]= symbols[i_0 as usize];
                    symbols[i_0 as usize]= __brotli_swap_tmp;
                }
                j_0= j_0.wrapping_add(1);
            }
            i_0= i_0.wrapping_add(1);
        }
        if count == 2 as libc::c_int as libc::c_ulong {
            BrotliWriteBits(
                max_bits,
                symbols[0 as libc::c_int as usize],
                storage_ix,
                storage,
            );
            BrotliWriteBits(
                max_bits,
                symbols[1 as libc::c_int as usize],
                storage_ix,
                storage,
            );
        } else if count == 3 as libc::c_int as libc::c_ulong {
            BrotliWriteBits(
                max_bits,
                symbols[0 as libc::c_int as usize],
                storage_ix,
                storage,
            );
            BrotliWriteBits(
                max_bits,
                symbols[1 as libc::c_int as usize],
                storage_ix,
                storage,
            );
            BrotliWriteBits(
                max_bits,
                symbols[2 as libc::c_int as usize],
                storage_ix,
                storage,
            );
        } else {
            BrotliWriteBits(
                max_bits,
                symbols[0 as libc::c_int as usize],
                storage_ix,
                storage,
            );
            BrotliWriteBits(
                max_bits,
                symbols[1 as libc::c_int as usize],
                storage_ix,
                storage,
            );
            BrotliWriteBits(
                max_bits,
                symbols[2 as libc::c_int as usize],
                storage_ix,
                storage,
            );
            BrotliWriteBits(
                max_bits,
                symbols[3 as libc::c_int as usize],
                storage_ix,
                storage,
            );
            BrotliWriteBits(
                1 as libc::c_int as size_t,
                (if *depth.offset(symbols[0 as libc::c_int as usize] as isize)
                    as libc::c_int == 1 as libc::c_int
                {
                    1 as libc::c_int
                } else {
                    0 as libc::c_int
                }) as uint64_t,
                storage_ix,
                storage,
            );
        }
    } else {
        let mut previous_value = 8 as libc::c_int as uint8_t;
        let mut i_1: size_t = 0;
        StoreStaticCodeLengthCode(storage_ix, storage);
        i_1= 0 as libc::c_int as size_t;
        while i_1 < length {
            let value = *depth.offset(i_1 as isize);
            let mut reps = 1 as libc::c_int as size_t;
            let mut k_0: size_t = 0;
            k_0= i_1.wrapping_add(1 as libc::c_int as libc::c_ulong);
            while k_0 < length
                && *depth.offset(k_0 as isize) as libc::c_int == value as libc::c_int
            {
                reps= reps.wrapping_add(1);
                k_0= k_0.wrapping_add(1);
            }
            i_1= (i_1 as libc::c_ulong).wrapping_add(reps) as size_t as size_t;
            if value as libc::c_int == 0 as libc::c_int {
                BrotliWriteBits(
                    crate::src::enc::brotli_bit_stream::kZeroRepsDepth[reps as usize] as size_t,
                    crate::src::enc::brotli_bit_stream::kZeroRepsBits[reps as usize],
                    storage_ix,
                    storage,
                );
            } else {
                if previous_value as libc::c_int != value as libc::c_int {
                    BrotliWriteBits(
                        crate::src::enc::brotli_bit_stream::kCodeLengthDepth[value as usize] as size_t,
                        crate::src::enc::brotli_bit_stream::kCodeLengthBits[value as usize] as uint64_t,
                        storage_ix,
                        storage,
                    );
                    reps= reps.wrapping_sub(1);
                }
                if reps < 3 as libc::c_int as libc::c_ulong {
                    while reps != 0 as libc::c_int as libc::c_ulong {
                        reps= reps.wrapping_sub(1);
                        BrotliWriteBits(
                            crate::src::enc::brotli_bit_stream::kCodeLengthDepth[value as usize] as size_t,
                            crate::src::enc::brotli_bit_stream::kCodeLengthBits[value as usize] as uint64_t,
                            storage_ix,
                            storage,
                        );
                    }
                } else {
                    reps= (reps as libc::c_ulong)
                        .wrapping_sub(3 as libc::c_int as libc::c_ulong) as size_t
                        as size_t;
                    BrotliWriteBits(
                        crate::src::enc::brotli_bit_stream::kNonZeroRepsDepth[reps as usize] as size_t,
                        crate::src::enc::brotli_bit_stream::kNonZeroRepsBits[reps as usize],
                        storage_ix,
                        storage,
                    );
                }
                previous_value= value;
            }
        }
    };
}
unsafe extern "C" fn IndexOf(
    mut v: *const uint8_t,
    mut v_size: size_t,
    mut value: uint8_t,
) -> size_t {
    let mut i = 0 as libc::c_int as size_t;
    while i < v_size {
        if *v.offset(i as isize) as libc::c_int == value as libc::c_int {
            return i;
        }
        i= i.wrapping_add(1);
    }
    return i;
}
unsafe extern "C" fn MoveToFront(mut v: *mut uint8_t, mut index: size_t) {
    let mut value = *v.offset(index as isize);
    let mut i: size_t = 0;
    i= index;
    while i != 0 as libc::c_int as libc::c_ulong {
        *v
            .offset(
                i as isize,
            ) = *v.offset(i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
        i= i.wrapping_sub(1);
    }
    *v.offset(0 as libc::c_int as isize) = value;
}
unsafe extern "C" fn MoveToFrontTransform(
    mut v_in: *const uint32_t,
    mut v_size: size_t,
    mut v_out: *mut uint32_t,
) {
    let mut i: size_t = 0;
    let mut mtf: [uint8_t; 256] = [0; 256];
    let mut max_value: uint32_t = 0;
    if v_size == 0 as libc::c_int as libc::c_ulong {
        return;
    }
    max_value= *v_in.offset(0 as libc::c_int as isize);
    i= 1 as libc::c_int as size_t;
    while i < v_size {
        if *v_in.offset(i as isize) > max_value {
            max_value= *v_in.offset(i as isize);
        }
        i= i.wrapping_add(1);
    }
    i= 0 as libc::c_int as size_t;
    while i <= max_value as libc::c_ulong {
        mtf[i as usize]= i as uint8_t;
        i= i.wrapping_add(1);
    }
    let mut mtf_size = max_value.wrapping_add(1 as libc::c_int as libc::c_uint)
        as size_t;
    i= 0 as libc::c_int as size_t;
    while i < v_size {
        let mut index = IndexOf(
            mtf.as_mut_ptr(),
            mtf_size,
            *v_in.offset(i as isize) as uint8_t,
        );
        *v_out.offset(i as isize) = index as uint32_t;
        MoveToFront(mtf.as_mut_ptr(), index);
        i= i.wrapping_add(1);
    }
}
unsafe extern "C" fn RunLengthCodeZeros(
    mut in_size: size_t,
    mut v: *mut uint32_t,
    mut out_size: Option<&mut size_t>,
    mut max_run_length_prefix: Option<&mut uint32_t>,
) {
    let mut max_reps = 0 as libc::c_int as uint32_t;
    let mut i: size_t = 0;
    let mut max_prefix: uint32_t = 0;
    i= 0 as libc::c_int as size_t;
    while i < in_size {
        let mut reps = 0 as libc::c_int as uint32_t;
        while i < in_size && *v.offset(i as isize) != 0 as libc::c_int as libc::c_uint {
            i= i.wrapping_add(1);
        }
        while i < in_size && *v.offset(i as isize) == 0 as libc::c_int as libc::c_uint {
            reps= reps.wrapping_add(1);
            i= i.wrapping_add(1);
        }
        max_reps= brotli_max_uint32_t(reps, max_reps);
    }
    max_prefix= if max_reps > 0 as libc::c_int as libc::c_uint {
        Log2FloorNonZero(max_reps as size_t)
    } else {
        0 as libc::c_int as libc::c_uint
    };
    max_prefix= brotli_min_uint32_t(max_prefix, (*max_run_length_prefix.as_deref().unwrap()));
    *max_run_length_prefix.as_deref_mut().unwrap()= max_prefix;
    *out_size.as_deref_mut().unwrap()= 0 as libc::c_int as size_t;
    i= 0 as libc::c_int as size_t;
    while i < in_size {
        if *v.offset(i as isize) != 0 as libc::c_int as libc::c_uint {
            *v
                .offset(
                    (*out_size.as_deref().unwrap()) as isize,
                ) = (*v.offset(i as isize)).wrapping_add((*max_run_length_prefix.as_deref().unwrap()));
            i= i.wrapping_add(1);
            *out_size.as_deref_mut().unwrap()= (*out_size.as_deref().unwrap()).wrapping_add(1);
        } else {
            let mut reps_0 = 1 as libc::c_int as uint32_t;
            let mut k: size_t = 0;
            k= i.wrapping_add(1 as libc::c_int as libc::c_ulong);
            while k < in_size
                && *v.offset(k as isize) == 0 as libc::c_int as libc::c_uint
            {
                reps_0= reps_0.wrapping_add(1);
                k= k.wrapping_add(1);
            }
            i= (i as libc::c_ulong).wrapping_add(reps_0 as libc::c_ulong) as size_t
                as size_t;
            while reps_0 != 0 as libc::c_int as libc::c_uint {
                if reps_0 < (2 as libc::c_uint) << max_prefix {
                    let mut run_length_prefix = Log2FloorNonZero(reps_0 as size_t);
                    let extra_bits = reps_0
                        .wrapping_sub((1 as libc::c_uint) << run_length_prefix);
                    *v
                        .offset(
                            (*out_size.as_deref().unwrap()) as isize,
                        ) = run_length_prefix
                        .wrapping_add(extra_bits << 9 as libc::c_int);
                    *out_size.as_deref_mut().unwrap()= (*out_size.as_deref().unwrap()).wrapping_add(1);
                    break;
                } else {
                    let extra_bits_0 = ((1 as libc::c_uint) << max_prefix)
                        .wrapping_sub(1 as libc::c_uint);
                    *v
                        .offset(
                            (*out_size.as_deref().unwrap()) as isize,
                        ) = max_prefix.wrapping_add(extra_bits_0 << 9 as libc::c_int);
                    reps_0= (reps_0 as libc::c_uint)
                        .wrapping_sub(
                            ((2 as libc::c_uint) << max_prefix)
                                .wrapping_sub(1 as libc::c_uint),
                        ) as uint32_t as uint32_t;
                    *out_size.as_deref_mut().unwrap()= (*out_size.as_deref().unwrap()).wrapping_add(1);
                }
            }
        }
    }
}
static mut kSymbolMask: uint32_t = 0;
unsafe extern "C" fn EncodeContextMap(
    mut m: *mut crate::src::enc::backward_references_hq::MemoryManager,
    mut context_map: *const uint32_t,
    mut context_map_size: size_t,
    mut num_clusters: size_t,
    mut tree: *mut HuffmanTree,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let mut i: size_t = 0;
    let mut rle_symbols = 0 as *mut uint32_t;
    let mut max_run_length_prefix = 6 as libc::c_int as uint32_t;
    let mut num_rle_symbols = 0 as libc::c_int as size_t;
    let mut histogram: [uint32_t; 272] = [0; 272];
    let mut depths: [uint8_t; 272] = [0; 272];
    let mut bits: [uint16_t; 272] = [0; 272];
    StoreVarLenUint8(
        num_clusters.wrapping_sub(1 as libc::c_int as libc::c_ulong),
        storage_ix,
        storage,
    );
    if num_clusters == 1 as libc::c_int as libc::c_ulong {
        return;
    }
    rle_symbols= if context_map_size > 0 as libc::c_int as libc::c_ulong {
        crate::src::enc::memory::BrotliAllocate(
            m,
            context_map_size
                .wrapping_mul(::std::mem::size_of::<uint32_t>() as libc::c_ulong),
        ) as *mut uint32_t
    } else {
        0 as *mut uint32_t
    };
    if 0 as libc::c_int != 0 || 0 as libc::c_int != 0 {
        return;
    }
    MoveToFrontTransform(context_map, context_map_size, rle_symbols);
    RunLengthCodeZeros(
        context_map_size,
        rle_symbols,
        Some(&mut num_rle_symbols),
        Some(&mut max_run_length_prefix),
    );
    memset(
        histogram.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[uint32_t; 272]>() as libc::c_ulong,
    );
    i= 0 as libc::c_int as size_t;
    while i < num_rle_symbols {
        histogram[(*rle_symbols.offset(i as isize) & crate::src::enc::brotli_bit_stream::kSymbolMask)
            as usize]= histogram[(*rle_symbols.offset(i as isize) & crate::src::enc::brotli_bit_stream::kSymbolMask)
            as usize]
            .wrapping_add(1);
        i= i.wrapping_add(1);
    }
    let mut use_rle = if max_run_length_prefix > 0 as libc::c_int as libc::c_uint {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
    BrotliWriteBits(
        1 as libc::c_int as size_t,
        use_rle as uint64_t,
        storage_ix,
        storage,
    );
    if use_rle != 0 {
        BrotliWriteBits(
            4 as libc::c_int as size_t,
            max_run_length_prefix.wrapping_sub(1 as libc::c_int as libc::c_uint)
                as uint64_t,
            storage_ix,
            storage,
        );
    }
    BuildAndStoreHuffmanTree(
        histogram.as_mut_ptr(),
        num_clusters.wrapping_add(max_run_length_prefix as libc::c_ulong),
        num_clusters.wrapping_add(max_run_length_prefix as libc::c_ulong),
        tree,
        depths.as_mut_ptr(),
        bits.as_mut_ptr(),
        storage_ix,
        storage,
    );
    i= 0 as libc::c_int as size_t;
    while i < num_rle_symbols {
        let rle_symbol = *rle_symbols.offset(i as isize) & crate::src::enc::brotli_bit_stream::kSymbolMask;
        let extra_bits_val = *rle_symbols.offset(i as isize) >> 9 as libc::c_int;
        BrotliWriteBits(
            depths[rle_symbol as usize] as size_t,
            bits[rle_symbol as usize] as uint64_t,
            storage_ix,
            storage,
        );
        if rle_symbol > 0 as libc::c_int as libc::c_uint
            && rle_symbol <= max_run_length_prefix
        {
            BrotliWriteBits(
                rle_symbol as size_t,
                extra_bits_val as uint64_t,
                storage_ix,
                storage,
            );
        }
        i= i.wrapping_add(1);
    }
    BrotliWriteBits(
        1 as libc::c_int as size_t,
        1 as libc::c_int as uint64_t,
        storage_ix,
        storage,
    );
    crate::src::enc::memory::BrotliFree(m, rle_symbols as *mut libc::c_void);
    rle_symbols= 0 as *mut uint32_t;
}
#[inline(always)]
unsafe extern "C" fn StoreBlockSwitch(
    mut code: *mut BlockSplitCode,
    mut block_len: uint32_t,
    mut block_type: uint8_t,
    mut is_first_block: libc::c_int,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let mut typecode = NextBlockTypeCode(Some(&mut (*code).type_code_calculator), block_type);
    let mut lencode: size_t = 0;
    let mut len_nextra: uint32_t = 0;
    let mut len_extra: uint32_t = 0;
    if is_first_block == 0 {
        BrotliWriteBits(
            (*code).type_depths[typecode as usize] as size_t,
            (*code).type_bits[typecode as usize] as uint64_t,
            storage_ix,
            storage,
        );
    }
    GetBlockLengthPrefixCode(block_len, Some(&mut lencode), Some(&mut len_nextra), Some(&mut len_extra));
    BrotliWriteBits(
        (*code).length_depths[lencode as usize] as size_t,
        (*code).length_bits[lencode as usize] as uint64_t,
        storage_ix,
        storage,
    );
    BrotliWriteBits(len_nextra as size_t, len_extra as uint64_t, storage_ix, storage);
}
unsafe extern "C" fn BuildAndStoreBlockSplitCode(
    mut types: *const uint8_t,
    mut lengths: *const uint32_t,
    mut num_blocks: size_t,
    mut num_types: size_t,
    mut tree: *mut HuffmanTree,
    mut code: *mut BlockSplitCode,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let mut type_histo: [uint32_t; 258] = [0; 258];
    let mut length_histo: [uint32_t; 26] = [0; 26];
    let mut i: size_t = 0;
    let mut type_code_calculator = BlockTypeCodeCalculator {
        last_type: 0,
        second_last_type: 0,
    };
    memset(
        type_histo.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        num_types
            .wrapping_add(2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<uint32_t>() as libc::c_ulong),
    );
    memset(
        length_histo.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[uint32_t; 26]>() as libc::c_ulong,
    );
    InitBlockTypeCodeCalculator(Some(&mut type_code_calculator));
    i= 0 as libc::c_int as size_t;
    while i < num_blocks {
        let mut type_code = NextBlockTypeCode(
            Some(&mut type_code_calculator),
            *types.offset(i as isize),
        );
        if i != 0 as libc::c_int as libc::c_ulong {
            type_histo[type_code
                as usize]= type_histo[type_code as usize].wrapping_add(1);
        }
        length_histo[BlockLengthPrefixCode(*lengths.offset(i as isize))
            as usize]= length_histo[BlockLengthPrefixCode(*lengths.offset(i as isize))
            as usize]
            .wrapping_add(1);
        i= i.wrapping_add(1);
    }
    StoreVarLenUint8(
        num_types.wrapping_sub(1 as libc::c_int as libc::c_ulong),
        storage_ix,
        storage,
    );
    if num_types > 1 as libc::c_int as libc::c_ulong {
        BuildAndStoreHuffmanTree(
            core::ptr::addr_of_mut!(*type_histo.as_mut_ptr().offset(0 as libc::c_int as isize)),
            num_types.wrapping_add(2 as libc::c_int as libc::c_ulong),
            num_types.wrapping_add(2 as libc::c_int as libc::c_ulong),
            tree,
            core::ptr::addr_of_mut!(*(*code).type_depths.as_mut_ptr().offset(0 as libc::c_int as isize)),
            core::ptr::addr_of_mut!(*(*code).type_bits.as_mut_ptr().offset(0 as libc::c_int as isize)),
            storage_ix,
            storage,
        );
        BuildAndStoreHuffmanTree(
            core::ptr::addr_of_mut!(*length_histo.as_mut_ptr().offset(0 as libc::c_int as isize)),
            26 as libc::c_int as size_t,
            26 as libc::c_int as size_t,
            tree,
            core::ptr::addr_of_mut!(*(*code).length_depths.as_mut_ptr().offset(0 as libc::c_int as isize)),
            core::ptr::addr_of_mut!(*(*code).length_bits.as_mut_ptr().offset(0 as libc::c_int as isize)),
            storage_ix,
            storage,
        );
        StoreBlockSwitch(
            code,
            *lengths.offset(0 as libc::c_int as isize),
            *types.offset(0 as libc::c_int as isize),
            1 as libc::c_int,
            storage_ix,
            storage,
        );
    }
}
unsafe extern "C" fn StoreTrivialContextMap(
    mut num_types: size_t,
    mut context_bits: size_t,
    mut tree: *mut HuffmanTree,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    StoreVarLenUint8(
        num_types.wrapping_sub(1 as libc::c_int as libc::c_ulong),
        storage_ix,
        storage,
    );
    if num_types > 1 as libc::c_int as libc::c_ulong {
        let mut repeat_code = context_bits
            .wrapping_sub(1 as libc::c_uint as libc::c_ulong);
        let mut repeat_bits = ((1 as libc::c_uint) << repeat_code)
            .wrapping_sub(1 as libc::c_uint) as size_t;
        let mut alphabet_size = num_types.wrapping_add(repeat_code);
        let mut histogram: [uint32_t; 272] = [0; 272];
        let mut depths: [uint8_t; 272] = [0; 272];
        let mut bits: [uint16_t; 272] = [0; 272];
        let mut i: size_t = 0;
        memset(
            histogram.as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            alphabet_size
                .wrapping_mul(::std::mem::size_of::<uint32_t>() as libc::c_ulong),
        );
        BrotliWriteBits(
            1 as libc::c_int as size_t,
            1 as libc::c_int as uint64_t,
            storage_ix,
            storage,
        );
        BrotliWriteBits(
            4 as libc::c_int as size_t,
            repeat_code.wrapping_sub(1 as libc::c_int as libc::c_ulong),
            storage_ix,
            storage,
        );
        histogram[repeat_code as usize]= num_types as uint32_t;
        histogram[0 as libc::c_int as usize]= 1 as libc::c_int as uint32_t;
        i= context_bits;
        while i < alphabet_size {
            histogram[i as usize]= 1 as libc::c_int as uint32_t;
            i= i.wrapping_add(1);
        }
        BuildAndStoreHuffmanTree(
            histogram.as_mut_ptr(),
            alphabet_size,
            alphabet_size,
            tree,
            depths.as_mut_ptr(),
            bits.as_mut_ptr(),
            storage_ix,
            storage,
        );
        i= 0 as libc::c_int as size_t;
        while i < num_types {
            let mut code = if i == 0 as libc::c_int as libc::c_ulong {
                0 as libc::c_int as libc::c_ulong
            } else {
                i.wrapping_add(context_bits)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            };
            BrotliWriteBits(
                depths[code as usize] as size_t,
                bits[code as usize] as uint64_t,
                storage_ix,
                storage,
            );
            BrotliWriteBits(
                depths[repeat_code as usize] as size_t,
                bits[repeat_code as usize] as uint64_t,
                storage_ix,
                storage,
            );
            BrotliWriteBits(repeat_code, repeat_bits, storage_ix, storage);
            i= i.wrapping_add(1);
        }
        BrotliWriteBits(
            1 as libc::c_int as size_t,
            1 as libc::c_int as uint64_t,
            storage_ix,
            storage,
        );
    }
}
unsafe extern "C" fn InitBlockEncoder(
    mut self_0: *mut BlockEncoder,
    mut histogram_length: size_t,
    mut num_block_types: size_t,
    mut block_types: *const uint8_t,
    mut block_lengths: *const uint32_t,
    mut num_blocks: size_t,
) {
    (*self_0).histogram_length_= histogram_length;
    (*self_0).num_block_types_= num_block_types;
    (*self_0).block_types_= block_types;
    (*self_0).block_lengths_= block_lengths;
    (*self_0).num_blocks_= num_blocks;
    InitBlockTypeCodeCalculator(Some(&mut (*self_0).block_split_code_.type_code_calculator));
    (*self_0).block_ix_= 0 as libc::c_int as size_t;
    (*self_0).block_len_= (if num_blocks == 0 as libc::c_int as libc::c_ulong {
        0 as libc::c_int as libc::c_uint
    } else {
        *block_lengths.offset(0 as libc::c_int as isize)
    }) as size_t;
    (*self_0).entropy_ix_= 0 as libc::c_int as size_t;
    (*self_0).depths_= 0 as *mut uint8_t;
    (*self_0).bits_= 0 as *mut uint16_t;
}
unsafe extern "C" fn CleanupBlockEncoder(
    mut m: *mut crate::src::enc::backward_references_hq::MemoryManager,
    mut self_0: *mut BlockEncoder,
) {
    crate::src::enc::memory::BrotliFree(m, (*self_0).depths_ as *mut libc::c_void);
    (*self_0).depths_= 0 as *mut uint8_t;
    crate::src::enc::memory::BrotliFree(m, (*self_0).bits_ as *mut libc::c_void);
    (*self_0).bits_= 0 as *mut uint16_t;
}
unsafe extern "C" fn BuildAndStoreBlockSwitchEntropyCodes(
    mut self_0: *mut BlockEncoder,
    mut tree: *mut HuffmanTree,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    BuildAndStoreBlockSplitCode(
        (*self_0).block_types_,
        (*self_0).block_lengths_,
        (*self_0).num_blocks_,
        (*self_0).num_block_types_,
        tree,
        core::ptr::addr_of_mut!((*self_0).block_split_code_),
        storage_ix,
        storage,
    );
}
unsafe extern "C" fn StoreSymbol(
    mut self_0: *mut BlockEncoder,
    mut symbol: size_t,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    if (*self_0).block_len_ == 0 as libc::c_int as libc::c_ulong {
        (*self_0).block_ix_= (*self_0).block_ix_.wrapping_add(1); let mut block_ix  = (*self_0).block_ix_;
        let mut block_len = *(*self_0).block_lengths_.offset(block_ix as isize);
        let mut block_type = *(*self_0).block_types_.offset(block_ix as isize);
        (*self_0).block_len_= block_len as size_t;
        (*self_0).entropy_ix_= (block_type as libc::c_ulong)
            .wrapping_mul((*self_0).histogram_length_);
        StoreBlockSwitch(
            core::ptr::addr_of_mut!((*self_0).block_split_code_),
            block_len,
            block_type,
            0 as libc::c_int,
            storage_ix,
            storage,
        );
    }
    (*self_0).block_len_= (*self_0).block_len_.wrapping_sub(1);
    let mut ix = (*self_0).entropy_ix_.wrapping_add(symbol);
    BrotliWriteBits(
        *(*self_0).depths_.offset(ix as isize) as size_t,
        *(*self_0).bits_.offset(ix as isize) as uint64_t,
        storage_ix,
        storage,
    );
}
unsafe extern "C" fn StoreSymbolWithContext(
    mut self_0: *mut BlockEncoder,
    mut symbol: size_t,
    mut context: size_t,
    mut context_map: *const uint32_t,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
    mut context_bits: size_t,
) {
    if (*self_0).block_len_ == 0 as libc::c_int as libc::c_ulong {
        (*self_0).block_ix_= (*self_0).block_ix_.wrapping_add(1); let mut block_ix  = (*self_0).block_ix_;
        let mut block_len = *(*self_0).block_lengths_.offset(block_ix as isize);
        let mut block_type = *(*self_0).block_types_.offset(block_ix as isize);
        (*self_0).block_len_= block_len as size_t;
        (*self_0).entropy_ix_= (block_type as size_t) << context_bits;
        StoreBlockSwitch(
            core::ptr::addr_of_mut!((*self_0).block_split_code_),
            block_len,
            block_type,
            0 as libc::c_int,
            storage_ix,
            storage,
        );
    }
    (*self_0).block_len_= (*self_0).block_len_.wrapping_sub(1);
    let mut histo_ix = *context_map
        .offset((*self_0).entropy_ix_.wrapping_add(context) as isize) as size_t;
    let mut ix = histo_ix.wrapping_mul((*self_0).histogram_length_).wrapping_add(symbol);
    BrotliWriteBits(
        *(*self_0).depths_.offset(ix as isize) as size_t,
        *(*self_0).bits_.offset(ix as isize) as uint64_t,
        storage_ix,
        storage,
    );
}
unsafe extern "C" fn BuildAndStoreEntropyCodesCommand(
    mut m: *mut crate::src::enc::backward_references_hq::MemoryManager,
    mut self_0: *mut BlockEncoder,
    mut histograms: *const crate::src::enc::bit_cost::HistogramCommand,
    mut histograms_size: size_t,
    mut alphabet_size: size_t,
    mut tree: *mut HuffmanTree,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let table_size = histograms_size.wrapping_mul((*self_0).histogram_length_);
    (*self_0).depths_= if table_size > 0 as libc::c_int as libc::c_ulong {
        crate::src::enc::memory::BrotliAllocate(
            m,
            table_size.wrapping_mul(::std::mem::size_of::<uint8_t>() as libc::c_ulong),
        ) as *mut uint8_t
    } else {
        0 as *mut uint8_t
    };
    (*self_0).bits_= if table_size > 0 as libc::c_int as libc::c_ulong {
        crate::src::enc::memory::BrotliAllocate(
            m,
            table_size.wrapping_mul(::std::mem::size_of::<uint16_t>() as libc::c_ulong),
        ) as *mut uint16_t
    } else {
        0 as *mut uint16_t
    };
    if 0 as libc::c_int != 0 {
        return;
    }
    let mut i: size_t = 0;
    i= 0 as libc::c_int as size_t;
    while i < histograms_size {
        let mut ix = i.wrapping_mul((*self_0).histogram_length_);
        BuildAndStoreHuffmanTree(
            &*((*histograms.offset(i as isize)).data_)
                .as_ptr()
                .offset(0 as libc::c_int as isize),
            (*self_0).histogram_length_,
            alphabet_size,
            tree,
            core::ptr::addr_of_mut!(*(*self_0).depths_.offset(ix as isize)),
            core::ptr::addr_of_mut!(*(*self_0).bits_.offset(ix as isize)),
            storage_ix,
            storage,
        );
        i= i.wrapping_add(1);
    }
}
unsafe extern "C" fn BuildAndStoreEntropyCodesLiteral(
    mut m: *mut crate::src::enc::backward_references_hq::MemoryManager,
    mut self_0: *mut BlockEncoder,
    mut histograms: *const crate::src::enc::bit_cost::HistogramLiteral,
    mut histograms_size: size_t,
    mut alphabet_size: size_t,
    mut tree: *mut HuffmanTree,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let table_size = histograms_size.wrapping_mul((*self_0).histogram_length_);
    (*self_0).depths_= if table_size > 0 as libc::c_int as libc::c_ulong {
        crate::src::enc::memory::BrotliAllocate(
            m,
            table_size.wrapping_mul(::std::mem::size_of::<uint8_t>() as libc::c_ulong),
        ) as *mut uint8_t
    } else {
        0 as *mut uint8_t
    };
    (*self_0).bits_= if table_size > 0 as libc::c_int as libc::c_ulong {
        crate::src::enc::memory::BrotliAllocate(
            m,
            table_size.wrapping_mul(::std::mem::size_of::<uint16_t>() as libc::c_ulong),
        ) as *mut uint16_t
    } else {
        0 as *mut uint16_t
    };
    if 0 as libc::c_int != 0 {
        return;
    }
    let mut i: size_t = 0;
    i= 0 as libc::c_int as size_t;
    while i < histograms_size {
        let mut ix = i.wrapping_mul((*self_0).histogram_length_);
        BuildAndStoreHuffmanTree(
            &*((*histograms.offset(i as isize)).data_)
                .as_ptr()
                .offset(0 as libc::c_int as isize),
            (*self_0).histogram_length_,
            alphabet_size,
            tree,
            core::ptr::addr_of_mut!(*(*self_0).depths_.offset(ix as isize)),
            core::ptr::addr_of_mut!(*(*self_0).bits_.offset(ix as isize)),
            storage_ix,
            storage,
        );
        i= i.wrapping_add(1);
    }
}
unsafe extern "C" fn BuildAndStoreEntropyCodesDistance(
    mut m: *mut crate::src::enc::backward_references_hq::MemoryManager,
    mut self_0: *mut BlockEncoder,
    mut histograms: *const crate::src::enc::bit_cost::HistogramDistance,
    mut histograms_size: size_t,
    mut alphabet_size: size_t,
    mut tree: *mut HuffmanTree,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let table_size = histograms_size.wrapping_mul((*self_0).histogram_length_);
    (*self_0).depths_= if table_size > 0 as libc::c_int as libc::c_ulong {
        crate::src::enc::memory::BrotliAllocate(
            m,
            table_size.wrapping_mul(::std::mem::size_of::<uint8_t>() as libc::c_ulong),
        ) as *mut uint8_t
    } else {
        0 as *mut uint8_t
    };
    (*self_0).bits_= if table_size > 0 as libc::c_int as libc::c_ulong {
        crate::src::enc::memory::BrotliAllocate(
            m,
            table_size.wrapping_mul(::std::mem::size_of::<uint16_t>() as libc::c_ulong),
        ) as *mut uint16_t
    } else {
        0 as *mut uint16_t
    };
    if 0 as libc::c_int != 0 {
        return;
    }
    let mut i: size_t = 0;
    i= 0 as libc::c_int as size_t;
    while i < histograms_size {
        let mut ix = i.wrapping_mul((*self_0).histogram_length_);
        BuildAndStoreHuffmanTree(
            &*((*histograms.offset(i as isize)).data_)
                .as_ptr()
                .offset(0 as libc::c_int as isize),
            (*self_0).histogram_length_,
            alphabet_size,
            tree,
            core::ptr::addr_of_mut!(*(*self_0).depths_.offset(ix as isize)),
            core::ptr::addr_of_mut!(*(*self_0).bits_.offset(ix as isize)),
            storage_ix,
            storage,
        );
        i= i.wrapping_add(1);
    }
}
unsafe extern "C" fn JumpToByteBoundary(
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    *storage_ix= (*storage_ix).wrapping_add(7 as libc::c_uint as libc::c_ulong)
        & !(7 as libc::c_uint) as libc::c_ulong;
    *storage
        .offset(
            ((*storage_ix) >> 3 as libc::c_int) as isize,
        ) = 0 as libc::c_int as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn BrotliStoreMetaBlock(
    mut m: *mut crate::src::enc::backward_references_hq::MemoryManager,
    mut input: *const uint8_t,
    mut start_pos: size_t,
    mut length: size_t,
    mut mask: size_t,
    mut prev_byte: uint8_t,
    mut prev_byte2: uint8_t,
    mut is_last: libc::c_int,
    mut params: *const crate::src::enc::backward_references::BrotliEncoderParams,
    mut literal_context_mode: ContextType,
    mut commands: *const crate::src::enc::backward_references::Command,
    mut n_commands: size_t,
    mut mb: *const MetaBlockSplit,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let mut pos = start_pos;
    let mut i: size_t = 0;
    let mut num_distance_symbols = (*params).dist.alphabet_size_max;
    let mut num_effective_distance_symbols = (*params).dist.alphabet_size_limit;
    let mut tree = 0 as *mut HuffmanTree;
    let mut literal_context_lut: ContextLut = &*_kBrotliContextLookupTable
        .as_ptr()
        .offset(((literal_context_mode as libc::c_uint) << 9 as libc::c_int) as isize)
        as *const uint8_t;
    let mut literal_enc = BlockEncoder {
        histogram_length_: 0,
        num_block_types_: 0,
        block_types_: 0 as *const uint8_t,
        block_lengths_: 0 as *const uint32_t,
        num_blocks_: 0,
        block_split_code_: BlockSplitCode {
            type_code_calculator: BlockTypeCodeCalculator {
                last_type: 0,
                second_last_type: 0,
            },
            type_depths: [0; 258],
            type_bits: [0; 258],
            length_depths: [0; 26],
            length_bits: [0; 26],
        },
        block_ix_: 0,
        block_len_: 0,
        entropy_ix_: 0,
        depths_: 0 as *mut uint8_t,
        bits_: 0 as *mut uint16_t,
    };
    let mut command_enc = BlockEncoder {
        histogram_length_: 0,
        num_block_types_: 0,
        block_types_: 0 as *const uint8_t,
        block_lengths_: 0 as *const uint32_t,
        num_blocks_: 0,
        block_split_code_: BlockSplitCode {
            type_code_calculator: BlockTypeCodeCalculator {
                last_type: 0,
                second_last_type: 0,
            },
            type_depths: [0; 258],
            type_bits: [0; 258],
            length_depths: [0; 26],
            length_bits: [0; 26],
        },
        block_ix_: 0,
        block_len_: 0,
        entropy_ix_: 0,
        depths_: 0 as *mut uint8_t,
        bits_: 0 as *mut uint16_t,
    };
    let mut distance_enc = BlockEncoder {
        histogram_length_: 0,
        num_block_types_: 0,
        block_types_: 0 as *const uint8_t,
        block_lengths_: 0 as *const uint32_t,
        num_blocks_: 0,
        block_split_code_: BlockSplitCode {
            type_code_calculator: BlockTypeCodeCalculator {
                last_type: 0,
                second_last_type: 0,
            },
            type_depths: [0; 258],
            type_bits: [0; 258],
            length_depths: [0; 26],
            length_bits: [0; 26],
        },
        block_ix_: 0,
        block_len_: 0,
        entropy_ix_: 0,
        depths_: 0 as *mut uint8_t,
        bits_: 0 as *mut uint16_t,
    };
    let mut dist: *const crate::src::enc::backward_references::BrotliDistanceParams = &(*params).dist;
    StoreCompressedMetaBlockHeader(is_last, length, storage_ix, storage);
    tree= if 2 as libc::c_int * 704 as libc::c_int + 1 as libc::c_int > 0 as libc::c_int
    {
        crate::src::enc::memory::BrotliAllocate(
            m,
            ((2 as libc::c_int * 704 as libc::c_int + 1 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<HuffmanTree>() as libc::c_ulong),
        ) as *mut HuffmanTree
    } else {
        0 as *mut HuffmanTree
    };
    if 0 as libc::c_int != 0 || 0 as libc::c_int != 0 {
        return;
    }
    InitBlockEncoder(
        core::ptr::addr_of_mut!(literal_enc),
        256 as libc::c_int as size_t,
        (*mb).literal_split.num_types,
        (*mb).literal_split.types,
        (*mb).literal_split.lengths,
        (*mb).literal_split.num_blocks,
    );
    InitBlockEncoder(
        core::ptr::addr_of_mut!(command_enc),
        704 as libc::c_int as size_t,
        (*mb).command_split.num_types,
        (*mb).command_split.types,
        (*mb).command_split.lengths,
        (*mb).command_split.num_blocks,
    );
    InitBlockEncoder(
        core::ptr::addr_of_mut!(distance_enc),
        num_effective_distance_symbols as size_t,
        (*mb).distance_split.num_types,
        (*mb).distance_split.types,
        (*mb).distance_split.lengths,
        (*mb).distance_split.num_blocks,
    );
    BuildAndStoreBlockSwitchEntropyCodes(core::ptr::addr_of_mut!(literal_enc), tree, storage_ix, storage);
    BuildAndStoreBlockSwitchEntropyCodes(core::ptr::addr_of_mut!(command_enc), tree, storage_ix, storage);
    BuildAndStoreBlockSwitchEntropyCodes(core::ptr::addr_of_mut!(distance_enc), tree, storage_ix, storage);
    BrotliWriteBits(
        2 as libc::c_int as size_t,
        (*dist).distance_postfix_bits as uint64_t,
        storage_ix,
        storage,
    );
    BrotliWriteBits(
        4 as libc::c_int as size_t,
        ((*dist).num_direct_distance_codes >> (*dist).distance_postfix_bits) as uint64_t,
        storage_ix,
        storage,
    );
    i= 0 as libc::c_int as size_t;
    while i < (*mb).literal_split.num_types {
        BrotliWriteBits(
            2 as libc::c_int as size_t,
            literal_context_mode as uint64_t,
            storage_ix,
            storage,
        );
        i= i.wrapping_add(1);
    }
    if (*mb).literal_context_map_size == 0 as libc::c_int as libc::c_ulong {
        StoreTrivialContextMap(
            (*mb).literal_histograms_size,
            6 as libc::c_int as size_t,
            tree,
            storage_ix,
            storage,
        );
    } else {
        EncodeContextMap(
            m,
            (*mb).literal_context_map,
            (*mb).literal_context_map_size,
            (*mb).literal_histograms_size,
            tree,
            storage_ix,
            storage,
        );
        if 0 as libc::c_int != 0 {
            return;
        }
    }
    if (*mb).distance_context_map_size == 0 as libc::c_int as libc::c_ulong {
        StoreTrivialContextMap(
            (*mb).distance_histograms_size,
            2 as libc::c_int as size_t,
            tree,
            storage_ix,
            storage,
        );
    } else {
        EncodeContextMap(
            m,
            (*mb).distance_context_map,
            (*mb).distance_context_map_size,
            (*mb).distance_histograms_size,
            tree,
            storage_ix,
            storage,
        );
        if 0 as libc::c_int != 0 {
            return;
        }
    }
    BuildAndStoreEntropyCodesLiteral(
        m,
        core::ptr::addr_of_mut!(literal_enc),
        (*mb).literal_histograms,
        (*mb).literal_histograms_size,
        256 as libc::c_int as size_t,
        tree,
        storage_ix,
        storage,
    );
    if 0 as libc::c_int != 0 {
        return;
    }
    BuildAndStoreEntropyCodesCommand(
        m,
        core::ptr::addr_of_mut!(command_enc),
        (*mb).command_histograms,
        (*mb).command_histograms_size,
        704 as libc::c_int as size_t,
        tree,
        storage_ix,
        storage,
    );
    if 0 as libc::c_int != 0 {
        return;
    }
    BuildAndStoreEntropyCodesDistance(
        m,
        core::ptr::addr_of_mut!(distance_enc),
        (*mb).distance_histograms,
        (*mb).distance_histograms_size,
        num_distance_symbols as size_t,
        tree,
        storage_ix,
        storage,
    );
    if 0 as libc::c_int != 0 {
        return;
    }
    crate::src::enc::memory::BrotliFree(m, tree as *mut libc::c_void);
    tree= 0 as *mut HuffmanTree;
    i= 0 as libc::c_int as size_t;
    while i < n_commands {
        let cmd = *commands.offset(i as isize);
        let mut cmd_code = cmd.cmd_prefix_ as size_t;
        StoreSymbol(core::ptr::addr_of_mut!(command_enc), cmd_code, storage_ix, storage);
        StoreCommandExtra(&cmd, storage_ix, storage);
        if (*mb).literal_context_map_size == 0 as libc::c_int as libc::c_ulong {
            let mut j: size_t = 0;
            j= cmd.insert_len_ as size_t;
            while j != 0 as libc::c_int as libc::c_ulong {
                StoreSymbol(
                    core::ptr::addr_of_mut!(literal_enc),
                    *input.offset((pos & mask) as isize) as size_t,
                    storage_ix,
                    storage,
                );
                pos= pos.wrapping_add(1);
                j= j.wrapping_sub(1);
            }
        } else {
            let mut j_0: size_t = 0;
            j_0= cmd.insert_len_ as size_t;
            while j_0 != 0 as libc::c_int as libc::c_ulong {
                let mut context = (*literal_context_lut.offset(prev_byte as isize)
                    as libc::c_int
                    | *literal_context_lut
                        .offset(256 as libc::c_int as isize)
                        .offset(prev_byte2 as isize) as libc::c_int) as size_t;
                let mut literal = *input.offset((pos & mask) as isize);
                StoreSymbolWithContext(
                    core::ptr::addr_of_mut!(literal_enc),
                    literal as size_t,
                    context,
                    (*mb).literal_context_map,
                    storage_ix,
                    storage,
                    6 as libc::c_int as size_t,
                );
                prev_byte2= prev_byte;
                prev_byte= literal;
                pos= pos.wrapping_add(1);
                j_0= j_0.wrapping_sub(1);
            }
        }
        pos= (pos as libc::c_ulong).wrapping_add(CommandCopyLen(&cmd) as libc::c_ulong)
            as size_t as size_t;
        if CommandCopyLen(&cmd) != 0 {
            prev_byte2= *input
                .offset(
                    (pos.wrapping_sub(2 as libc::c_int as libc::c_ulong) & mask) as isize,
                );
            prev_byte= *input
                .offset(
                    (pos.wrapping_sub(1 as libc::c_int as libc::c_ulong) & mask) as isize,
                );
            if cmd.cmd_prefix_ as libc::c_int >= 128 as libc::c_int {
                let mut dist_code = (cmd.dist_prefix_ as libc::c_int
                    & 0x3ff as libc::c_int) as size_t;
                let mut distnumextra = (cmd.dist_prefix_ as libc::c_int
                    >> 10 as libc::c_int) as uint32_t;
                let mut distextra = cmd.dist_extra_ as uint64_t;
                if (*mb).distance_context_map_size == 0 as libc::c_int as libc::c_ulong {
                    StoreSymbol(core::ptr::addr_of_mut!(distance_enc), dist_code, storage_ix, storage);
                } else {
                    let mut context_0 = CommandDistanceContext(&cmd) as size_t;
                    StoreSymbolWithContext(
                        core::ptr::addr_of_mut!(distance_enc),
                        dist_code,
                        context_0,
                        (*mb).distance_context_map,
                        storage_ix,
                        storage,
                        2 as libc::c_int as size_t,
                    );
                }
                BrotliWriteBits(distnumextra as size_t, distextra, storage_ix, storage);
            }
        }
        i= i.wrapping_add(1);
    }
    CleanupBlockEncoder(m, core::ptr::addr_of_mut!(distance_enc));
    CleanupBlockEncoder(m, core::ptr::addr_of_mut!(command_enc));
    CleanupBlockEncoder(m, core::ptr::addr_of_mut!(literal_enc));
    if is_last != 0 {
        JumpToByteBoundary(storage_ix, storage);
    }
}
unsafe extern "C" fn BuildHistograms(
    mut input: *const uint8_t,
    mut start_pos: size_t,
    mut mask: size_t,
    mut commands: *const crate::src::enc::backward_references::Command,
    mut n_commands: size_t,
    mut lit_histo: *mut crate::src::enc::bit_cost::HistogramLiteral,
    mut cmd_histo: *mut crate::src::enc::bit_cost::HistogramCommand,
    mut dist_histo: *mut crate::src::enc::bit_cost::HistogramDistance,
) {
    let mut pos = start_pos;
    let mut i: size_t = 0;
    i= 0 as libc::c_int as size_t;
    while i < n_commands {
        let cmd = *commands.offset(i as isize);
        let mut j: size_t = 0;
        HistogramAddCommand(cmd_histo.as_mut(), cmd.cmd_prefix_ as size_t);
        j= cmd.insert_len_ as size_t;
        while j != 0 as libc::c_int as libc::c_ulong {
            HistogramAddLiteral(
                lit_histo.as_mut(),
                *input.offset((pos & mask) as isize) as size_t,
            );
            pos= pos.wrapping_add(1);
            j= j.wrapping_sub(1);
        }
        pos= (pos as libc::c_ulong).wrapping_add(CommandCopyLen(&cmd) as libc::c_ulong)
            as size_t as size_t;
        if CommandCopyLen(&cmd) != 0
            && cmd.cmd_prefix_ as libc::c_int >= 128 as libc::c_int
        {
            HistogramAddDistance(
                dist_histo.as_mut(),
                (cmd.dist_prefix_ as libc::c_int & 0x3ff as libc::c_int) as size_t,
            );
        }
        i= i.wrapping_add(1);
    }
}
unsafe extern "C" fn StoreDataWithHuffmanCodes(
    mut input: *const uint8_t,
    mut start_pos: size_t,
    mut mask: size_t,
    mut commands: *const crate::src::enc::backward_references::Command,
    mut n_commands: size_t,
    mut lit_depth: *const uint8_t,
    mut lit_bits: *const uint16_t,
    mut cmd_depth: *const uint8_t,
    mut cmd_bits: *const uint16_t,
    mut dist_depth: *const uint8_t,
    mut dist_bits: *const uint16_t,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let mut pos = start_pos;
    let mut i: size_t = 0;
    i= 0 as libc::c_int as size_t;
    while i < n_commands {
        let cmd = *commands.offset(i as isize);
        let cmd_code = cmd.cmd_prefix_ as size_t;
        let mut j: size_t = 0;
        BrotliWriteBits(
            *cmd_depth.offset(cmd_code as isize) as size_t,
            *cmd_bits.offset(cmd_code as isize) as uint64_t,
            storage_ix,
            storage,
        );
        StoreCommandExtra(&cmd, storage_ix, storage);
        j= cmd.insert_len_ as size_t;
        while j != 0 as libc::c_int as libc::c_ulong {
            let literal = *input.offset((pos & mask) as isize);
            BrotliWriteBits(
                *lit_depth.offset(literal as isize) as size_t,
                *lit_bits.offset(literal as isize) as uint64_t,
                storage_ix,
                storage,
            );
            pos= pos.wrapping_add(1);
            j= j.wrapping_sub(1);
        }
        pos= (pos as libc::c_ulong).wrapping_add(CommandCopyLen(&cmd) as libc::c_ulong)
            as size_t as size_t;
        if CommandCopyLen(&cmd) != 0
            && cmd.cmd_prefix_ as libc::c_int >= 128 as libc::c_int
        {
            let dist_code = (cmd.dist_prefix_ as libc::c_int & 0x3ff as libc::c_int)
                as size_t;
            let distnumextra = (cmd.dist_prefix_ as libc::c_int >> 10 as libc::c_int)
                as uint32_t;
            let distextra = cmd.dist_extra_;
            BrotliWriteBits(
                *dist_depth.offset(dist_code as isize) as size_t,
                *dist_bits.offset(dist_code as isize) as uint64_t,
                storage_ix,
                storage,
            );
            BrotliWriteBits(
                distnumextra as size_t,
                distextra as uint64_t,
                storage_ix,
                storage,
            );
        }
        i= i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn BrotliStoreMetaBlockTrivial(
    mut m: *mut crate::src::enc::backward_references_hq::MemoryManager,
    mut input: *const uint8_t,
    mut start_pos: size_t,
    mut length: size_t,
    mut mask: size_t,
    mut is_last: libc::c_int,
    mut params: *const crate::src::enc::backward_references::BrotliEncoderParams,
    mut commands: *const crate::src::enc::backward_references::Command,
    mut n_commands: size_t,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let mut lit_histo = crate::src::enc::bit_cost::HistogramLiteral {
        data_: [0; 256],
        total_count_: 0,
        bit_cost_: 0.,
    };
    let mut cmd_histo = crate::src::enc::bit_cost::HistogramCommand {
        data_: [0; 704],
        total_count_: 0,
        bit_cost_: 0.,
    };
    let mut dist_histo = crate::src::enc::bit_cost::HistogramDistance {
        data_: [0; 544],
        total_count_: 0,
        bit_cost_: 0.,
    };
    let mut lit_depth: [uint8_t; 256] = [0; 256];
    let mut lit_bits: [uint16_t; 256] = [0; 256];
    let mut cmd_depth: [uint8_t; 704] = [0; 704];
    let mut cmd_bits: [uint16_t; 704] = [0; 704];
    let mut dist_depth: [uint8_t; 140] = [0; 140];
    let mut dist_bits: [uint16_t; 140] = [0; 140];
    let mut tree = 0 as *mut HuffmanTree;
    let mut num_distance_symbols = (*params).dist.alphabet_size_max;
    StoreCompressedMetaBlockHeader(is_last, length, storage_ix, storage);
    HistogramClearLiteral(core::ptr::addr_of_mut!(lit_histo));
    HistogramClearCommand(core::ptr::addr_of_mut!(cmd_histo));
    HistogramClearDistance(core::ptr::addr_of_mut!(dist_histo));
    BuildHistograms(
        input,
        start_pos,
        mask,
        commands,
        n_commands,
        core::ptr::addr_of_mut!(lit_histo),
        core::ptr::addr_of_mut!(cmd_histo),
        core::ptr::addr_of_mut!(dist_histo),
    );
    BrotliWriteBits(
        13 as libc::c_int as size_t,
        0 as libc::c_int as uint64_t,
        storage_ix,
        storage,
    );
    tree= if 2 as libc::c_int * 704 as libc::c_int + 1 as libc::c_int > 0 as libc::c_int
    {
        crate::src::enc::memory::BrotliAllocate(
            m,
            ((2 as libc::c_int * 704 as libc::c_int + 1 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<HuffmanTree>() as libc::c_ulong),
        ) as *mut HuffmanTree
    } else {
        0 as *mut HuffmanTree
    };
    if 0 as libc::c_int != 0 || 0 as libc::c_int != 0 {
        return;
    }
    BuildAndStoreHuffmanTree(
        lit_histo.data_.as_mut_ptr(),
        256 as libc::c_int as size_t,
        256 as libc::c_int as size_t,
        tree,
        lit_depth.as_mut_ptr(),
        lit_bits.as_mut_ptr(),
        storage_ix,
        storage,
    );
    BuildAndStoreHuffmanTree(
        cmd_histo.data_.as_mut_ptr(),
        704 as libc::c_int as size_t,
        704 as libc::c_int as size_t,
        tree,
        cmd_depth.as_mut_ptr(),
        cmd_bits.as_mut_ptr(),
        storage_ix,
        storage,
    );
    BuildAndStoreHuffmanTree(
        dist_histo.data_.as_mut_ptr(),
        ((16 as libc::c_int + 0 as libc::c_int) as libc::c_uint)
            .wrapping_add((62 as libc::c_uint) << 0 as libc::c_int + 1 as libc::c_int)
            as size_t,
        num_distance_symbols as size_t,
        tree,
        dist_depth.as_mut_ptr(),
        dist_bits.as_mut_ptr(),
        storage_ix,
        storage,
    );
    crate::src::enc::memory::BrotliFree(m, tree as *mut libc::c_void);
    tree= 0 as *mut HuffmanTree;
    StoreDataWithHuffmanCodes(
        input,
        start_pos,
        mask,
        commands,
        n_commands,
        lit_depth.as_mut_ptr(),
        lit_bits.as_mut_ptr(),
        cmd_depth.as_mut_ptr(),
        cmd_bits.as_mut_ptr(),
        dist_depth.as_mut_ptr(),
        dist_bits.as_mut_ptr(),
        storage_ix,
        storage,
    );
    if is_last != 0 {
        JumpToByteBoundary(storage_ix, storage);
    }
}
#[no_mangle]
pub unsafe extern "C" fn BrotliStoreMetaBlockFast(
    mut m: *mut crate::src::enc::backward_references_hq::MemoryManager,
    mut input: *const uint8_t,
    mut start_pos: size_t,
    mut length: size_t,
    mut mask: size_t,
    mut is_last: libc::c_int,
    mut params: *const crate::src::enc::backward_references::BrotliEncoderParams,
    mut commands: *const crate::src::enc::backward_references::Command,
    mut n_commands: size_t,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let mut num_distance_symbols = (*params).dist.alphabet_size_max;
    let mut distance_alphabet_bits = (Log2FloorNonZero(
        num_distance_symbols.wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    ))
        .wrapping_add(1 as libc::c_int as libc::c_uint);
    StoreCompressedMetaBlockHeader(is_last, length, storage_ix, storage);
    BrotliWriteBits(
        13 as libc::c_int as size_t,
        0 as libc::c_int as uint64_t,
        storage_ix,
        storage,
    );
    if n_commands <= 128 as libc::c_int as libc::c_ulong {
        let mut histogram: [uint32_t; 256] = [
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
        let mut pos = start_pos;
        let mut num_literals = 0 as libc::c_int as size_t;
        let mut i: size_t = 0;
        let mut lit_depth: [uint8_t; 256] = [0; 256];
        let mut lit_bits: [uint16_t; 256] = [0; 256];
        i= 0 as libc::c_int as size_t;
        while i < n_commands {
            let cmd = *commands.offset(i as isize);
            let mut j: size_t = 0;
            j= cmd.insert_len_ as size_t;
            while j != 0 as libc::c_int as libc::c_ulong {
                histogram[*input.offset((pos & mask) as isize)
                    as usize]= histogram[*input.offset((pos & mask) as isize)
                    as usize]
                    .wrapping_add(1);
                pos= pos.wrapping_add(1);
                j= j.wrapping_sub(1);
            }
            num_literals= (num_literals as libc::c_ulong)
                .wrapping_add(cmd.insert_len_ as libc::c_ulong) as size_t as size_t;
            pos= (pos as libc::c_ulong)
                .wrapping_add(CommandCopyLen(&cmd) as libc::c_ulong) as size_t as size_t;
            i= i.wrapping_add(1);
        }
        BrotliBuildAndStoreHuffmanTreeFast(
            m,
            histogram.as_mut_ptr(),
            num_literals,
            8 as libc::c_int as size_t,
            lit_depth.as_mut_ptr(),
            lit_bits.as_mut_ptr(),
            storage_ix,
            storage,
        );
        if 0 as libc::c_int != 0 {
            return;
        }
        StoreStaticCommandHuffmanTree(storage_ix, storage);
        StoreStaticDistanceHuffmanTree(storage_ix, storage);
        StoreDataWithHuffmanCodes(
            input,
            start_pos,
            mask,
            commands,
            n_commands,
            lit_depth.as_mut_ptr(),
            lit_bits.as_mut_ptr(),
            kStaticCommandCodeDepth.as_ptr(),
            kStaticCommandCodeBits.as_ptr(),
            kStaticDistanceCodeDepth.as_ptr(),
            kStaticDistanceCodeBits.as_ptr(),
            storage_ix,
            storage,
        );
    } else {
        let mut lit_histo = crate::src::enc::bit_cost::HistogramLiteral {
            data_: [0; 256],
            total_count_: 0,
            bit_cost_: 0.,
        };
        let mut cmd_histo = crate::src::enc::bit_cost::HistogramCommand {
            data_: [0; 704],
            total_count_: 0,
            bit_cost_: 0.,
        };
        let mut dist_histo = crate::src::enc::bit_cost::HistogramDistance {
            data_: [0; 544],
            total_count_: 0,
            bit_cost_: 0.,
        };
        let mut lit_depth_0: [uint8_t; 256] = [0; 256];
        let mut lit_bits_0: [uint16_t; 256] = [0; 256];
        let mut cmd_depth: [uint8_t; 704] = [0; 704];
        let mut cmd_bits: [uint16_t; 704] = [0; 704];
        let mut dist_depth: [uint8_t; 140] = [0; 140];
        let mut dist_bits: [uint16_t; 140] = [0; 140];
        HistogramClearLiteral(core::ptr::addr_of_mut!(lit_histo));
        HistogramClearCommand(core::ptr::addr_of_mut!(cmd_histo));
        HistogramClearDistance(core::ptr::addr_of_mut!(dist_histo));
        BuildHistograms(
            input,
            start_pos,
            mask,
            commands,
            n_commands,
            core::ptr::addr_of_mut!(lit_histo),
            core::ptr::addr_of_mut!(cmd_histo),
            core::ptr::addr_of_mut!(dist_histo),
        );
        BrotliBuildAndStoreHuffmanTreeFast(
            m,
            lit_histo.data_.as_mut_ptr(),
            lit_histo.total_count_,
            8 as libc::c_int as size_t,
            lit_depth_0.as_mut_ptr(),
            lit_bits_0.as_mut_ptr(),
            storage_ix,
            storage,
        );
        if 0 as libc::c_int != 0 {
            return;
        }
        BrotliBuildAndStoreHuffmanTreeFast(
            m,
            cmd_histo.data_.as_mut_ptr(),
            cmd_histo.total_count_,
            10 as libc::c_int as size_t,
            cmd_depth.as_mut_ptr(),
            cmd_bits.as_mut_ptr(),
            storage_ix,
            storage,
        );
        if 0 as libc::c_int != 0 {
            return;
        }
        BrotliBuildAndStoreHuffmanTreeFast(
            m,
            dist_histo.data_.as_mut_ptr(),
            dist_histo.total_count_,
            distance_alphabet_bits as size_t,
            dist_depth.as_mut_ptr(),
            dist_bits.as_mut_ptr(),
            storage_ix,
            storage,
        );
        if 0 as libc::c_int != 0 {
            return;
        }
        StoreDataWithHuffmanCodes(
            input,
            start_pos,
            mask,
            commands,
            n_commands,
            lit_depth_0.as_mut_ptr(),
            lit_bits_0.as_mut_ptr(),
            cmd_depth.as_mut_ptr(),
            cmd_bits.as_mut_ptr(),
            dist_depth.as_mut_ptr(),
            dist_bits.as_mut_ptr(),
            storage_ix,
            storage,
        );
    }
    if is_last != 0 {
        JumpToByteBoundary(storage_ix, storage);
    }
}
#[no_mangle]
pub unsafe extern "C" fn BrotliStoreUncompressedMetaBlock(
    mut is_final_block: libc::c_int,
    mut input: *const uint8_t,
    mut position: size_t,
    mut mask: size_t,
    mut len: size_t,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let mut masked_pos = position & mask;
    BrotliStoreUncompressedMetaBlockHeader(len, storage_ix, storage);
    JumpToByteBoundary(storage_ix, storage);
    if masked_pos.wrapping_add(len)
        > mask.wrapping_add(1 as libc::c_int as libc::c_ulong)
    {
        let mut len1 = mask
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_sub(masked_pos);
        memcpy(
            core::ptr::addr_of_mut!(*storage.offset(((*storage_ix) >> 3 as libc::c_int) as isize))
                as *mut uint8_t as *mut libc::c_void,
            &*input.offset(masked_pos as isize) as *const uint8_t as *const libc::c_void,
            len1,
        );
        *storage_ix= ((*storage_ix) as libc::c_ulong)
            .wrapping_add(len1 << 3 as libc::c_int) as size_t as size_t;
        len= (len as libc::c_ulong).wrapping_sub(len1) as size_t as size_t;
        masked_pos= 0 as libc::c_int as size_t;
    }
    memcpy(
        core::ptr::addr_of_mut!(*storage.offset(((*storage_ix) >> 3 as libc::c_int) as isize)) as *mut uint8_t
            as *mut libc::c_void,
        &*input.offset(masked_pos as isize) as *const uint8_t as *const libc::c_void,
        len,
    );
    *storage_ix= ((*storage_ix) as libc::c_ulong).wrapping_add(len << 3 as libc::c_int)
        as size_t as size_t;
    BrotliWriteBitsPrepareStorage((*storage_ix), storage);
    if is_final_block != 0 {
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
        JumpToByteBoundary(storage_ix, storage);
    }
}
unsafe extern "C" fn run_static_initializers() {
    crate::src::enc::brotli_bit_stream::kSymbolMask= ((1 as libc::c_uint) << 9 as libc::c_int)
        .wrapping_sub(1 as libc::c_uint);
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
