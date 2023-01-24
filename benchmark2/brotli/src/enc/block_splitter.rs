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
    fn BrotliAllocate(m: *mut MemoryManager, n: size_t) -> *mut libc::c_void;
    fn BrotliFree(m: *mut MemoryManager, p: *mut libc::c_void);
    fn BrotliPopulationCostLiteral(_: *const HistogramLiteral) -> libc::c_double;
    fn BrotliPopulationCostCommand(_: *const HistogramCommand) -> libc::c_double;
    fn BrotliPopulationCostDistance(_: *const HistogramDistance) -> libc::c_double;
    fn BrotliHistogramCombineLiteral(
        out: *mut HistogramLiteral,
        cluster_size: *mut uint32_t,
        symbols: *mut uint32_t,
        clusters: *mut uint32_t,
        pairs: *mut HistogramPair,
        num_clusters: size_t,
        symbols_size: size_t,
        max_clusters: size_t,
        max_num_pairs: size_t,
    ) -> size_t;
    fn BrotliHistogramCombineDistance(
        out: *mut HistogramDistance,
        cluster_size: *mut uint32_t,
        symbols: *mut uint32_t,
        clusters: *mut uint32_t,
        pairs: *mut HistogramPair,
        num_clusters: size_t,
        symbols_size: size_t,
        max_clusters: size_t,
        max_num_pairs: size_t,
    ) -> size_t;
    fn BrotliHistogramCombineCommand(
        out: *mut HistogramCommand,
        cluster_size: *mut uint32_t,
        symbols: *mut uint32_t,
        clusters: *mut uint32_t,
        pairs: *mut HistogramPair,
        num_clusters: size_t,
        symbols_size: size_t,
        max_clusters: size_t,
        max_num_pairs: size_t,
    ) -> size_t;
    fn BrotliHistogramBitCostDistanceLiteral(
        histogram: *const HistogramLiteral,
        candidate: *const HistogramLiteral,
    ) -> libc::c_double;
    fn BrotliHistogramBitCostDistanceDistance(
        histogram: *const HistogramDistance,
        candidate: *const HistogramDistance,
    ) -> libc::c_double;
    fn BrotliHistogramBitCostDistanceCommand(
        histogram: *const HistogramCommand,
        candidate: *const HistogramCommand,
    ) -> libc::c_double;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
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
pub type BrotliEncoderMode = libc::c_uint;
pub const BROTLI_MODE_FONT: BrotliEncoderMode = 2;
pub const BROTLI_MODE_TEXT: BrotliEncoderMode = 1;
pub const BROTLI_MODE_GENERIC: BrotliEncoderMode = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BrotliDictionary {
    pub size_bits_by_length: [uint8_t; 32],
    pub offsets_by_length: [uint32_t; 32],
    pub data_size: size_t,
    pub data: *const uint8_t,
}
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
pub struct MemoryManager {
    pub alloc_func: brotli_alloc_func,
    pub free_func: brotli_free_func,
    pub opaque: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BlockSplit {
    pub num_types: size_t,
    pub num_blocks: size_t,
    pub types: *mut uint8_t,
    pub lengths: *mut uint32_t,
    pub types_alloc_size: size_t,
    pub lengths_alloc_size: size_t,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HistogramCommand {
    pub data_: [uint32_t; 704],
    pub total_count_: size_t,
    pub bit_cost_: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HistogramLiteral {
    pub data_: [uint32_t; 256],
    pub total_count_: size_t,
    pub bit_cost_: libc::c_double,
}
#[inline(always)]
unsafe extern "C" fn brotli_max_uint8_t(mut a: uint8_t, mut b: uint8_t) -> uint8_t {
    return (if a as libc::c_int > b as libc::c_int {
        a as libc::c_int
    } else {
        b as libc::c_int
    }) as uint8_t;
}
#[inline(always)]
unsafe extern "C" fn brotli_min_size_t(mut a: size_t, mut b: size_t) -> size_t {
    return if a < b { a } else { b };
}
#[inline(always)]
unsafe extern "C" fn FastLog2(mut v: size_t) -> libc::c_double {
    if v < 256 as libc::c_int as libc::c_ulong {
        return kBrotliLog2Table[v as usize];
    }
    return log2(v as libc::c_double);
}
#[inline(always)]
unsafe extern "C" fn CommandCopyLen(mut self_0: *const Command) -> uint32_t {
    return (*self_0).copy_len_ & 0x1ffffff as libc::c_int as libc::c_uint;
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
unsafe extern "C" fn ClearHistogramsDistance(
    mut array: *mut HistogramDistance,
    mut length: size_t,
) {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < length {
        HistogramClearDistance(array.offset(i as isize));
        i = i.wrapping_add(1);
    }
}
#[inline(always)]
unsafe extern "C" fn ClearHistogramsLiteral(
    mut array: *mut HistogramLiteral,
    mut length: size_t,
) {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < length {
        HistogramClearLiteral(array.offset(i as isize));
        i = i.wrapping_add(1);
    }
}
#[inline(always)]
unsafe extern "C" fn ClearHistogramsCommand(
    mut array: *mut HistogramCommand,
    mut length: size_t,
) {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < length {
        HistogramClearCommand(array.offset(i as isize));
        i = i.wrapping_add(1);
    }
}
#[inline(always)]
unsafe extern "C" fn HistogramAddLiteral(
    mut self_0: *mut HistogramLiteral,
    mut val: size_t,
) {
    let ref mut fresh0 = (*self_0).data_[val as usize];
    *fresh0 = (*fresh0).wrapping_add(1);
    let ref mut fresh1 = (*self_0).total_count_;
    *fresh1 = (*fresh1).wrapping_add(1);
}
#[inline(always)]
unsafe extern "C" fn HistogramAddCommand(
    mut self_0: *mut HistogramCommand,
    mut val: size_t,
) {
    let ref mut fresh2 = (*self_0).data_[val as usize];
    *fresh2 = (*fresh2).wrapping_add(1);
    let ref mut fresh3 = (*self_0).total_count_;
    *fresh3 = (*fresh3).wrapping_add(1);
}
#[inline(always)]
unsafe extern "C" fn HistogramAddDistance(
    mut self_0: *mut HistogramDistance,
    mut val: size_t,
) {
    let ref mut fresh4 = (*self_0).data_[val as usize];
    *fresh4 = (*fresh4).wrapping_add(1);
    let ref mut fresh5 = (*self_0).total_count_;
    *fresh5 = (*fresh5).wrapping_add(1);
}
#[inline(always)]
unsafe extern "C" fn HistogramAddVectorLiteral(
    mut self_0: *mut HistogramLiteral,
    mut p: *const uint8_t,
    mut n: size_t,
) {
    let ref mut fresh6 = (*self_0).total_count_;
    *fresh6 = (*fresh6 as libc::c_ulong).wrapping_add(n) as size_t as size_t;
    n = (n as libc::c_ulong).wrapping_add(1 as libc::c_int as libc::c_ulong) as size_t
        as size_t;
    loop {
        n = n.wrapping_sub(1);
        if !(n != 0) {
            break;
        }
        let fresh7 = p;
        p = p.offset(1);
        let ref mut fresh8 = (*self_0).data_[*fresh7 as usize];
        *fresh8 = (*fresh8).wrapping_add(1);
    };
}
#[inline(always)]
unsafe extern "C" fn HistogramAddHistogramCommand(
    mut self_0: *mut HistogramCommand,
    mut v: *const HistogramCommand,
) {
    let mut i: size_t = 0;
    let ref mut fresh9 = (*self_0).total_count_;
    *fresh9 = (*fresh9 as libc::c_ulong).wrapping_add((*v).total_count_) as size_t
        as size_t;
    i = 0 as libc::c_int as size_t;
    while i < 704 as libc::c_int as libc::c_ulong {
        let ref mut fresh10 = (*self_0).data_[i as usize];
        *fresh10 = (*fresh10 as libc::c_uint).wrapping_add((*v).data_[i as usize])
            as uint32_t as uint32_t;
        i = i.wrapping_add(1);
    }
}
#[inline(always)]
unsafe extern "C" fn HistogramAddHistogramLiteral(
    mut self_0: *mut HistogramLiteral,
    mut v: *const HistogramLiteral,
) {
    let mut i: size_t = 0;
    let ref mut fresh11 = (*self_0).total_count_;
    *fresh11 = (*fresh11 as libc::c_ulong).wrapping_add((*v).total_count_) as size_t
        as size_t;
    i = 0 as libc::c_int as size_t;
    while i < 256 as libc::c_int as libc::c_ulong {
        let ref mut fresh12 = (*self_0).data_[i as usize];
        *fresh12 = (*fresh12 as libc::c_uint).wrapping_add((*v).data_[i as usize])
            as uint32_t as uint32_t;
        i = i.wrapping_add(1);
    }
}
#[inline(always)]
unsafe extern "C" fn HistogramAddHistogramDistance(
    mut self_0: *mut HistogramDistance,
    mut v: *const HistogramDistance,
) {
    let mut i: size_t = 0;
    let ref mut fresh13 = (*self_0).total_count_;
    *fresh13 = (*fresh13 as libc::c_ulong).wrapping_add((*v).total_count_) as size_t
        as size_t;
    i = 0 as libc::c_int as size_t;
    while i < 544 as libc::c_int as libc::c_ulong {
        let ref mut fresh14 = (*self_0).data_[i as usize];
        *fresh14 = (*fresh14 as libc::c_uint).wrapping_add((*v).data_[i as usize])
            as uint32_t as uint32_t;
        i = i.wrapping_add(1);
    }
}
#[inline(always)]
unsafe extern "C" fn HistogramAddVectorDistance(
    mut self_0: *mut HistogramDistance,
    mut p: *const uint16_t,
    mut n: size_t,
) {
    let ref mut fresh15 = (*self_0).total_count_;
    *fresh15 = (*fresh15 as libc::c_ulong).wrapping_add(n) as size_t as size_t;
    n = (n as libc::c_ulong).wrapping_add(1 as libc::c_int as libc::c_ulong) as size_t
        as size_t;
    loop {
        n = n.wrapping_sub(1);
        if !(n != 0) {
            break;
        }
        let fresh16 = p;
        p = p.offset(1);
        let ref mut fresh17 = (*self_0).data_[*fresh16 as usize];
        *fresh17 = (*fresh17).wrapping_add(1);
    };
}
#[inline(always)]
unsafe extern "C" fn HistogramAddVectorCommand(
    mut self_0: *mut HistogramCommand,
    mut p: *const uint16_t,
    mut n: size_t,
) {
    let ref mut fresh18 = (*self_0).total_count_;
    *fresh18 = (*fresh18 as libc::c_ulong).wrapping_add(n) as size_t as size_t;
    n = (n as libc::c_ulong).wrapping_add(1 as libc::c_int as libc::c_ulong) as size_t
        as size_t;
    loop {
        n = n.wrapping_sub(1);
        if !(n != 0) {
            break;
        }
        let fresh19 = p;
        p = p.offset(1);
        let ref mut fresh20 = (*self_0).data_[*fresh19 as usize];
        *fresh20 = (*fresh20).wrapping_add(1);
    };
}
#[inline(always)]
unsafe extern "C" fn HistogramDataSizeDistance() -> size_t {
    return 544 as libc::c_int as size_t;
}
#[inline(always)]
unsafe extern "C" fn HistogramDataSizeLiteral() -> size_t {
    return 256 as libc::c_int as size_t;
}
#[inline(always)]
unsafe extern "C" fn HistogramDataSizeCommand() -> size_t {
    return 704 as libc::c_int as size_t;
}
static mut kMaxLiteralHistograms: size_t = 100 as libc::c_int as size_t;
static mut kMaxCommandHistograms: size_t = 50 as libc::c_int as size_t;
static mut kLiteralBlockSwitchCost: libc::c_double = 28.1f64;
static mut kCommandBlockSwitchCost: libc::c_double = 13.5f64;
static mut kDistanceBlockSwitchCost: libc::c_double = 14.6f64;
static mut kLiteralStrideLength: size_t = 70 as libc::c_int as size_t;
static mut kCommandStrideLength: size_t = 40 as libc::c_int as size_t;
static mut kSymbolsPerLiteralHistogram: size_t = 544 as libc::c_int as size_t;
static mut kSymbolsPerCommandHistogram: size_t = 530 as libc::c_int as size_t;
static mut kSymbolsPerDistanceHistogram: size_t = 544 as libc::c_int as size_t;
static mut kMinLengthForBlockSplitting: size_t = 128 as libc::c_int as size_t;
static mut kIterMulForRefining: size_t = 2 as libc::c_int as size_t;
static mut kMinItersForRefining: size_t = 100 as libc::c_int as size_t;
unsafe extern "C" fn CountLiterals(
    mut cmds: *const Command,
    num_commands: size_t,
) -> size_t {
    let mut total_length = 0 as libc::c_int as size_t;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < num_commands {
        total_length = (total_length as libc::c_ulong)
            .wrapping_add((*cmds.offset(i as isize)).insert_len_ as libc::c_ulong)
            as size_t as size_t;
        i = i.wrapping_add(1);
    }
    return total_length;
}
unsafe extern "C" fn CopyLiteralsToByteArray(
    mut cmds: *const Command,
    num_commands: size_t,
    mut data: *const uint8_t,
    offset: size_t,
    mask: size_t,
    mut literals: *mut uint8_t,
) {
    let mut pos = 0 as libc::c_int as size_t;
    let mut from_pos = offset & mask;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < num_commands {
        let mut insert_len = (*cmds.offset(i as isize)).insert_len_ as size_t;
        if from_pos.wrapping_add(insert_len) > mask {
            let mut head_size = mask
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_sub(from_pos);
            memcpy(
                literals.offset(pos as isize) as *mut libc::c_void,
                data.offset(from_pos as isize) as *const libc::c_void,
                head_size,
            );
            from_pos = 0 as libc::c_int as size_t;
            pos = (pos as libc::c_ulong).wrapping_add(head_size) as size_t as size_t;
            insert_len = (insert_len as libc::c_ulong).wrapping_sub(head_size) as size_t
                as size_t;
        }
        if insert_len > 0 as libc::c_int as libc::c_ulong {
            memcpy(
                literals.offset(pos as isize) as *mut libc::c_void,
                data.offset(from_pos as isize) as *const libc::c_void,
                insert_len,
            );
            pos = (pos as libc::c_ulong).wrapping_add(insert_len) as size_t as size_t;
        }
        from_pos = from_pos
            .wrapping_add(insert_len)
            .wrapping_add(CommandCopyLen(&*cmds.offset(i as isize)) as libc::c_ulong)
            & mask;
        i = i.wrapping_add(1);
    }
}
#[inline(always)]
unsafe extern "C" fn MyRand(mut seed: *mut uint32_t) -> uint32_t {
    *seed = (*seed as libc::c_uint).wrapping_mul(16807 as libc::c_uint) as uint32_t
        as uint32_t;
    return *seed;
}
#[inline(always)]
unsafe extern "C" fn BitCost(mut count: size_t) -> libc::c_double {
    return if count == 0 as libc::c_int as libc::c_ulong {
        -2.0f64
    } else {
        FastLog2(count)
    };
}
unsafe extern "C" fn InitialEntropyCodesLiteral(
    mut data: *const uint8_t,
    mut length: size_t,
    mut stride: size_t,
    mut num_histograms: size_t,
    mut histograms: *mut HistogramLiteral,
) {
    let mut seed = 7 as libc::c_int as uint32_t;
    let mut block_length = length.wrapping_div(num_histograms);
    let mut i: size_t = 0;
    ClearHistogramsLiteral(histograms, num_histograms);
    i = 0 as libc::c_int as size_t;
    while i < num_histograms {
        let mut pos = length.wrapping_mul(i).wrapping_div(num_histograms);
        if i != 0 as libc::c_int as libc::c_ulong {
            pos = (pos as libc::c_ulong)
                .wrapping_add(
                    (MyRand(&mut seed) as libc::c_ulong).wrapping_rem(block_length),
                ) as size_t as size_t;
        }
        if pos.wrapping_add(stride) >= length {
            pos = length
                .wrapping_sub(stride)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        }
        HistogramAddVectorLiteral(
            &mut *histograms.offset(i as isize),
            data.offset(pos as isize),
            stride,
        );
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn InitialEntropyCodesDistance(
    mut data: *const uint16_t,
    mut length: size_t,
    mut stride: size_t,
    mut num_histograms: size_t,
    mut histograms: *mut HistogramDistance,
) {
    let mut seed = 7 as libc::c_int as uint32_t;
    let mut block_length = length.wrapping_div(num_histograms);
    let mut i: size_t = 0;
    ClearHistogramsDistance(histograms, num_histograms);
    i = 0 as libc::c_int as size_t;
    while i < num_histograms {
        let mut pos = length.wrapping_mul(i).wrapping_div(num_histograms);
        if i != 0 as libc::c_int as libc::c_ulong {
            pos = (pos as libc::c_ulong)
                .wrapping_add(
                    (MyRand(&mut seed) as libc::c_ulong).wrapping_rem(block_length),
                ) as size_t as size_t;
        }
        if pos.wrapping_add(stride) >= length {
            pos = length
                .wrapping_sub(stride)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        }
        HistogramAddVectorDistance(
            &mut *histograms.offset(i as isize),
            data.offset(pos as isize),
            stride,
        );
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn InitialEntropyCodesCommand(
    mut data: *const uint16_t,
    mut length: size_t,
    mut stride: size_t,
    mut num_histograms: size_t,
    mut histograms: *mut HistogramCommand,
) {
    let mut seed = 7 as libc::c_int as uint32_t;
    let mut block_length = length.wrapping_div(num_histograms);
    let mut i: size_t = 0;
    ClearHistogramsCommand(histograms, num_histograms);
    i = 0 as libc::c_int as size_t;
    while i < num_histograms {
        let mut pos = length.wrapping_mul(i).wrapping_div(num_histograms);
        if i != 0 as libc::c_int as libc::c_ulong {
            pos = (pos as libc::c_ulong)
                .wrapping_add(
                    (MyRand(&mut seed) as libc::c_ulong).wrapping_rem(block_length),
                ) as size_t as size_t;
        }
        if pos.wrapping_add(stride) >= length {
            pos = length
                .wrapping_sub(stride)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        }
        HistogramAddVectorCommand(
            &mut *histograms.offset(i as isize),
            data.offset(pos as isize),
            stride,
        );
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn RandomSampleDistance(
    mut seed: *mut uint32_t,
    mut data: *const uint16_t,
    mut length: size_t,
    mut stride: size_t,
    mut sample: *mut HistogramDistance,
) {
    let mut pos = 0 as libc::c_int as size_t;
    if stride >= length {
        stride = length;
    } else {
        pos = (MyRand(seed) as libc::c_ulong)
            .wrapping_rem(
                length
                    .wrapping_sub(stride)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
    }
    HistogramAddVectorDistance(sample, data.offset(pos as isize), stride);
}
unsafe extern "C" fn RandomSampleLiteral(
    mut seed: *mut uint32_t,
    mut data: *const uint8_t,
    mut length: size_t,
    mut stride: size_t,
    mut sample: *mut HistogramLiteral,
) {
    let mut pos = 0 as libc::c_int as size_t;
    if stride >= length {
        stride = length;
    } else {
        pos = (MyRand(seed) as libc::c_ulong)
            .wrapping_rem(
                length
                    .wrapping_sub(stride)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
    }
    HistogramAddVectorLiteral(sample, data.offset(pos as isize), stride);
}
unsafe extern "C" fn RandomSampleCommand(
    mut seed: *mut uint32_t,
    mut data: *const uint16_t,
    mut length: size_t,
    mut stride: size_t,
    mut sample: *mut HistogramCommand,
) {
    let mut pos = 0 as libc::c_int as size_t;
    if stride >= length {
        stride = length;
    } else {
        pos = (MyRand(seed) as libc::c_ulong)
            .wrapping_rem(
                length
                    .wrapping_sub(stride)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
    }
    HistogramAddVectorCommand(sample, data.offset(pos as isize), stride);
}
unsafe extern "C" fn RefineEntropyCodesLiteral(
    mut data: *const uint8_t,
    mut length: size_t,
    mut stride: size_t,
    mut num_histograms: size_t,
    mut histograms: *mut HistogramLiteral,
) {
    let mut iters = kIterMulForRefining
        .wrapping_mul(length)
        .wrapping_div(stride)
        .wrapping_add(kMinItersForRefining);
    let mut seed = 7 as libc::c_int as uint32_t;
    let mut iter: size_t = 0;
    iters = iters
        .wrapping_add(num_histograms)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        .wrapping_div(num_histograms)
        .wrapping_mul(num_histograms);
    iter = 0 as libc::c_int as size_t;
    while iter < iters {
        let mut sample = HistogramLiteral {
            data_: [0; 256],
            total_count_: 0,
            bit_cost_: 0.,
        };
        HistogramClearLiteral(&mut sample);
        RandomSampleLiteral(&mut seed, data, length, stride, &mut sample);
        HistogramAddHistogramLiteral(
            &mut *histograms.offset(iter.wrapping_rem(num_histograms) as isize),
            &mut sample,
        );
        iter = iter.wrapping_add(1);
    }
}
unsafe extern "C" fn RefineEntropyCodesDistance(
    mut data: *const uint16_t,
    mut length: size_t,
    mut stride: size_t,
    mut num_histograms: size_t,
    mut histograms: *mut HistogramDistance,
) {
    let mut iters = kIterMulForRefining
        .wrapping_mul(length)
        .wrapping_div(stride)
        .wrapping_add(kMinItersForRefining);
    let mut seed = 7 as libc::c_int as uint32_t;
    let mut iter: size_t = 0;
    iters = iters
        .wrapping_add(num_histograms)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        .wrapping_div(num_histograms)
        .wrapping_mul(num_histograms);
    iter = 0 as libc::c_int as size_t;
    while iter < iters {
        let mut sample = HistogramDistance {
            data_: [0; 544],
            total_count_: 0,
            bit_cost_: 0.,
        };
        HistogramClearDistance(&mut sample);
        RandomSampleDistance(&mut seed, data, length, stride, &mut sample);
        HistogramAddHistogramDistance(
            &mut *histograms.offset(iter.wrapping_rem(num_histograms) as isize),
            &mut sample,
        );
        iter = iter.wrapping_add(1);
    }
}
unsafe extern "C" fn RefineEntropyCodesCommand(
    mut data: *const uint16_t,
    mut length: size_t,
    mut stride: size_t,
    mut num_histograms: size_t,
    mut histograms: *mut HistogramCommand,
) {
    let mut iters = kIterMulForRefining
        .wrapping_mul(length)
        .wrapping_div(stride)
        .wrapping_add(kMinItersForRefining);
    let mut seed = 7 as libc::c_int as uint32_t;
    let mut iter: size_t = 0;
    iters = iters
        .wrapping_add(num_histograms)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        .wrapping_div(num_histograms)
        .wrapping_mul(num_histograms);
    iter = 0 as libc::c_int as size_t;
    while iter < iters {
        let mut sample = HistogramCommand {
            data_: [0; 704],
            total_count_: 0,
            bit_cost_: 0.,
        };
        HistogramClearCommand(&mut sample);
        RandomSampleCommand(&mut seed, data, length, stride, &mut sample);
        HistogramAddHistogramCommand(
            &mut *histograms.offset(iter.wrapping_rem(num_histograms) as isize),
            &mut sample,
        );
        iter = iter.wrapping_add(1);
    }
}
unsafe extern "C" fn FindBlocksCommand(
    mut data: *const uint16_t,
    length: size_t,
    block_switch_bitcost: libc::c_double,
    num_histograms: size_t,
    mut histograms: *const HistogramCommand,
    mut insert_cost: *mut libc::c_double,
    mut cost: *mut libc::c_double,
    mut switch_signal: *mut uint8_t,
    mut block_id: *mut uint8_t,
) -> size_t {
    let data_size = HistogramDataSizeCommand();
    let bitmaplen = num_histograms.wrapping_add(7 as libc::c_int as libc::c_ulong)
        >> 3 as libc::c_int;
    let mut num_blocks = 1 as libc::c_int as size_t;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    if num_histograms <= 1 as libc::c_int as libc::c_ulong {
        i = 0 as libc::c_int as size_t;
        while i < length {
            *block_id.offset(i as isize) = 0 as libc::c_int as uint8_t;
            i = i.wrapping_add(1);
        }
        return 1 as libc::c_int as size_t;
    }
    memset(
        insert_cost as *mut libc::c_void,
        0 as libc::c_int,
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(data_size)
            .wrapping_mul(num_histograms),
    );
    i = 0 as libc::c_int as size_t;
    while i < num_histograms {
        *insert_cost
            .offset(
                i as isize,
            ) = FastLog2(
            (*histograms.offset(i as isize)).total_count_ as uint32_t as size_t,
        );
        i = i.wrapping_add(1);
    }
    i = data_size;
    while i != 0 as libc::c_int as libc::c_ulong {
        i = i.wrapping_sub(1);
        j = 0 as libc::c_int as size_t;
        while j < num_histograms {
            *insert_cost
                .offset(
                    i.wrapping_mul(num_histograms).wrapping_add(j) as isize,
                ) = *insert_cost.offset(j as isize)
                - BitCost((*histograms.offset(j as isize)).data_[i as usize] as size_t);
            j = j.wrapping_add(1);
        }
    }
    memset(
        cost as *mut libc::c_void,
        0 as libc::c_int,
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(num_histograms),
    );
    memset(
        switch_signal as *mut libc::c_void,
        0 as libc::c_int,
        (::std::mem::size_of::<uint8_t>() as libc::c_ulong)
            .wrapping_mul(length)
            .wrapping_mul(bitmaplen),
    );
    i = 0 as libc::c_int as size_t;
    while i < length {
        let byte_ix = i;
        let mut ix = byte_ix.wrapping_mul(bitmaplen);
        let mut insert_cost_ix = (*data.offset(byte_ix as isize) as libc::c_ulong)
            .wrapping_mul(num_histograms);
        let mut min_cost = 1e99f64;
        let mut block_switch_cost = block_switch_bitcost;
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < num_histograms {
            *cost.offset(k as isize)
                += *insert_cost.offset(insert_cost_ix.wrapping_add(k) as isize);
            if *cost.offset(k as isize) < min_cost {
                min_cost = *cost.offset(k as isize);
                *block_id.offset(byte_ix as isize) = k as uint8_t;
            }
            k = k.wrapping_add(1);
        }
        if byte_ix < 2000 as libc::c_int as libc::c_ulong {
            block_switch_cost
                *= 0.77f64
                    + 0.07f64 * byte_ix as libc::c_double
                        / 2000 as libc::c_int as libc::c_double;
        }
        k = 0 as libc::c_int as size_t;
        while k < num_histograms {
            *cost.offset(k as isize) -= min_cost;
            if *cost.offset(k as isize) >= block_switch_cost {
                let mask = ((1 as libc::c_uint)
                    << (k & 7 as libc::c_int as libc::c_ulong)) as uint8_t;
                *cost.offset(k as isize) = block_switch_cost;
                let ref mut fresh21 = *switch_signal
                    .offset(ix.wrapping_add(k >> 3 as libc::c_int) as isize);
                *fresh21 = (*fresh21 as libc::c_int | mask as libc::c_int) as uint8_t;
            }
            k = k.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    let mut byte_ix_0 = length.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    let mut ix_0 = byte_ix_0.wrapping_mul(bitmaplen);
    let mut cur_id = *block_id.offset(byte_ix_0 as isize);
    while byte_ix_0 > 0 as libc::c_int as libc::c_ulong {
        let mask_0 = ((1 as libc::c_uint) << (cur_id as libc::c_int & 7 as libc::c_int))
            as uint8_t;
        byte_ix_0 = byte_ix_0.wrapping_sub(1);
        ix_0 = (ix_0 as libc::c_ulong).wrapping_sub(bitmaplen) as size_t as size_t;
        if *switch_signal
            .offset(
                ix_0
                    .wrapping_add(
                        (cur_id as libc::c_int >> 3 as libc::c_int) as libc::c_ulong,
                    ) as isize,
            ) as libc::c_int & mask_0 as libc::c_int != 0
        {
            if cur_id as libc::c_int
                != *block_id.offset(byte_ix_0 as isize) as libc::c_int
            {
                cur_id = *block_id.offset(byte_ix_0 as isize);
                num_blocks = num_blocks.wrapping_add(1);
            }
        }
        *block_id.offset(byte_ix_0 as isize) = cur_id;
    }
    return num_blocks;
}
unsafe extern "C" fn FindBlocksLiteral(
    mut data: *const uint8_t,
    length: size_t,
    block_switch_bitcost: libc::c_double,
    num_histograms: size_t,
    mut histograms: *const HistogramLiteral,
    mut insert_cost: *mut libc::c_double,
    mut cost: *mut libc::c_double,
    mut switch_signal: *mut uint8_t,
    mut block_id: *mut uint8_t,
) -> size_t {
    let data_size = HistogramDataSizeLiteral();
    let bitmaplen = num_histograms.wrapping_add(7 as libc::c_int as libc::c_ulong)
        >> 3 as libc::c_int;
    let mut num_blocks = 1 as libc::c_int as size_t;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    if num_histograms <= 1 as libc::c_int as libc::c_ulong {
        i = 0 as libc::c_int as size_t;
        while i < length {
            *block_id.offset(i as isize) = 0 as libc::c_int as uint8_t;
            i = i.wrapping_add(1);
        }
        return 1 as libc::c_int as size_t;
    }
    memset(
        insert_cost as *mut libc::c_void,
        0 as libc::c_int,
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(data_size)
            .wrapping_mul(num_histograms),
    );
    i = 0 as libc::c_int as size_t;
    while i < num_histograms {
        *insert_cost
            .offset(
                i as isize,
            ) = FastLog2(
            (*histograms.offset(i as isize)).total_count_ as uint32_t as size_t,
        );
        i = i.wrapping_add(1);
    }
    i = data_size;
    while i != 0 as libc::c_int as libc::c_ulong {
        i = i.wrapping_sub(1);
        j = 0 as libc::c_int as size_t;
        while j < num_histograms {
            *insert_cost
                .offset(
                    i.wrapping_mul(num_histograms).wrapping_add(j) as isize,
                ) = *insert_cost.offset(j as isize)
                - BitCost((*histograms.offset(j as isize)).data_[i as usize] as size_t);
            j = j.wrapping_add(1);
        }
    }
    memset(
        cost as *mut libc::c_void,
        0 as libc::c_int,
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(num_histograms),
    );
    memset(
        switch_signal as *mut libc::c_void,
        0 as libc::c_int,
        (::std::mem::size_of::<uint8_t>() as libc::c_ulong)
            .wrapping_mul(length)
            .wrapping_mul(bitmaplen),
    );
    i = 0 as libc::c_int as size_t;
    while i < length {
        let byte_ix = i;
        let mut ix = byte_ix.wrapping_mul(bitmaplen);
        let mut insert_cost_ix = (*data.offset(byte_ix as isize) as libc::c_ulong)
            .wrapping_mul(num_histograms);
        let mut min_cost = 1e99f64;
        let mut block_switch_cost = block_switch_bitcost;
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < num_histograms {
            *cost.offset(k as isize)
                += *insert_cost.offset(insert_cost_ix.wrapping_add(k) as isize);
            if *cost.offset(k as isize) < min_cost {
                min_cost = *cost.offset(k as isize);
                *block_id.offset(byte_ix as isize) = k as uint8_t;
            }
            k = k.wrapping_add(1);
        }
        if byte_ix < 2000 as libc::c_int as libc::c_ulong {
            block_switch_cost
                *= 0.77f64
                    + 0.07f64 * byte_ix as libc::c_double
                        / 2000 as libc::c_int as libc::c_double;
        }
        k = 0 as libc::c_int as size_t;
        while k < num_histograms {
            *cost.offset(k as isize) -= min_cost;
            if *cost.offset(k as isize) >= block_switch_cost {
                let mask = ((1 as libc::c_uint)
                    << (k & 7 as libc::c_int as libc::c_ulong)) as uint8_t;
                *cost.offset(k as isize) = block_switch_cost;
                let ref mut fresh22 = *switch_signal
                    .offset(ix.wrapping_add(k >> 3 as libc::c_int) as isize);
                *fresh22 = (*fresh22 as libc::c_int | mask as libc::c_int) as uint8_t;
            }
            k = k.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    let mut byte_ix_0 = length.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    let mut ix_0 = byte_ix_0.wrapping_mul(bitmaplen);
    let mut cur_id = *block_id.offset(byte_ix_0 as isize);
    while byte_ix_0 > 0 as libc::c_int as libc::c_ulong {
        let mask_0 = ((1 as libc::c_uint) << (cur_id as libc::c_int & 7 as libc::c_int))
            as uint8_t;
        byte_ix_0 = byte_ix_0.wrapping_sub(1);
        ix_0 = (ix_0 as libc::c_ulong).wrapping_sub(bitmaplen) as size_t as size_t;
        if *switch_signal
            .offset(
                ix_0
                    .wrapping_add(
                        (cur_id as libc::c_int >> 3 as libc::c_int) as libc::c_ulong,
                    ) as isize,
            ) as libc::c_int & mask_0 as libc::c_int != 0
        {
            if cur_id as libc::c_int
                != *block_id.offset(byte_ix_0 as isize) as libc::c_int
            {
                cur_id = *block_id.offset(byte_ix_0 as isize);
                num_blocks = num_blocks.wrapping_add(1);
            }
        }
        *block_id.offset(byte_ix_0 as isize) = cur_id;
    }
    return num_blocks;
}
unsafe extern "C" fn FindBlocksDistance(
    mut data: *const uint16_t,
    length: size_t,
    block_switch_bitcost: libc::c_double,
    num_histograms: size_t,
    mut histograms: *const HistogramDistance,
    mut insert_cost: *mut libc::c_double,
    mut cost: *mut libc::c_double,
    mut switch_signal: *mut uint8_t,
    mut block_id: *mut uint8_t,
) -> size_t {
    let data_size = HistogramDataSizeDistance();
    let bitmaplen = num_histograms.wrapping_add(7 as libc::c_int as libc::c_ulong)
        >> 3 as libc::c_int;
    let mut num_blocks = 1 as libc::c_int as size_t;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    if num_histograms <= 1 as libc::c_int as libc::c_ulong {
        i = 0 as libc::c_int as size_t;
        while i < length {
            *block_id.offset(i as isize) = 0 as libc::c_int as uint8_t;
            i = i.wrapping_add(1);
        }
        return 1 as libc::c_int as size_t;
    }
    memset(
        insert_cost as *mut libc::c_void,
        0 as libc::c_int,
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(data_size)
            .wrapping_mul(num_histograms),
    );
    i = 0 as libc::c_int as size_t;
    while i < num_histograms {
        *insert_cost
            .offset(
                i as isize,
            ) = FastLog2(
            (*histograms.offset(i as isize)).total_count_ as uint32_t as size_t,
        );
        i = i.wrapping_add(1);
    }
    i = data_size;
    while i != 0 as libc::c_int as libc::c_ulong {
        i = i.wrapping_sub(1);
        j = 0 as libc::c_int as size_t;
        while j < num_histograms {
            *insert_cost
                .offset(
                    i.wrapping_mul(num_histograms).wrapping_add(j) as isize,
                ) = *insert_cost.offset(j as isize)
                - BitCost((*histograms.offset(j as isize)).data_[i as usize] as size_t);
            j = j.wrapping_add(1);
        }
    }
    memset(
        cost as *mut libc::c_void,
        0 as libc::c_int,
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(num_histograms),
    );
    memset(
        switch_signal as *mut libc::c_void,
        0 as libc::c_int,
        (::std::mem::size_of::<uint8_t>() as libc::c_ulong)
            .wrapping_mul(length)
            .wrapping_mul(bitmaplen),
    );
    i = 0 as libc::c_int as size_t;
    while i < length {
        let byte_ix = i;
        let mut ix = byte_ix.wrapping_mul(bitmaplen);
        let mut insert_cost_ix = (*data.offset(byte_ix as isize) as libc::c_ulong)
            .wrapping_mul(num_histograms);
        let mut min_cost = 1e99f64;
        let mut block_switch_cost = block_switch_bitcost;
        let mut k: size_t = 0;
        k = 0 as libc::c_int as size_t;
        while k < num_histograms {
            *cost.offset(k as isize)
                += *insert_cost.offset(insert_cost_ix.wrapping_add(k) as isize);
            if *cost.offset(k as isize) < min_cost {
                min_cost = *cost.offset(k as isize);
                *block_id.offset(byte_ix as isize) = k as uint8_t;
            }
            k = k.wrapping_add(1);
        }
        if byte_ix < 2000 as libc::c_int as libc::c_ulong {
            block_switch_cost
                *= 0.77f64
                    + 0.07f64 * byte_ix as libc::c_double
                        / 2000 as libc::c_int as libc::c_double;
        }
        k = 0 as libc::c_int as size_t;
        while k < num_histograms {
            *cost.offset(k as isize) -= min_cost;
            if *cost.offset(k as isize) >= block_switch_cost {
                let mask = ((1 as libc::c_uint)
                    << (k & 7 as libc::c_int as libc::c_ulong)) as uint8_t;
                *cost.offset(k as isize) = block_switch_cost;
                let ref mut fresh23 = *switch_signal
                    .offset(ix.wrapping_add(k >> 3 as libc::c_int) as isize);
                *fresh23 = (*fresh23 as libc::c_int | mask as libc::c_int) as uint8_t;
            }
            k = k.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    let mut byte_ix_0 = length.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    let mut ix_0 = byte_ix_0.wrapping_mul(bitmaplen);
    let mut cur_id = *block_id.offset(byte_ix_0 as isize);
    while byte_ix_0 > 0 as libc::c_int as libc::c_ulong {
        let mask_0 = ((1 as libc::c_uint) << (cur_id as libc::c_int & 7 as libc::c_int))
            as uint8_t;
        byte_ix_0 = byte_ix_0.wrapping_sub(1);
        ix_0 = (ix_0 as libc::c_ulong).wrapping_sub(bitmaplen) as size_t as size_t;
        if *switch_signal
            .offset(
                ix_0
                    .wrapping_add(
                        (cur_id as libc::c_int >> 3 as libc::c_int) as libc::c_ulong,
                    ) as isize,
            ) as libc::c_int & mask_0 as libc::c_int != 0
        {
            if cur_id as libc::c_int
                != *block_id.offset(byte_ix_0 as isize) as libc::c_int
            {
                cur_id = *block_id.offset(byte_ix_0 as isize);
                num_blocks = num_blocks.wrapping_add(1);
            }
        }
        *block_id.offset(byte_ix_0 as isize) = cur_id;
    }
    return num_blocks;
}
unsafe extern "C" fn RemapBlockIdsLiteral(
    mut block_ids: *mut uint8_t,
    length: size_t,
    mut new_id: *mut uint16_t,
    num_histograms: size_t,
) -> size_t {
    static mut kInvalidId: uint16_t = 256 as libc::c_int as uint16_t;
    let mut next_id = 0 as libc::c_int as uint16_t;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < num_histograms {
        *new_id.offset(i as isize) = kInvalidId;
        i = i.wrapping_add(1);
    }
    i = 0 as libc::c_int as size_t;
    while i < length {
        if *new_id.offset(*block_ids.offset(i as isize) as isize) as libc::c_int
            == kInvalidId as libc::c_int
        {
            let fresh24 = next_id;
            next_id = next_id.wrapping_add(1);
            *new_id.offset(*block_ids.offset(i as isize) as isize) = fresh24;
        }
        i = i.wrapping_add(1);
    }
    i = 0 as libc::c_int as size_t;
    while i < length {
        *block_ids
            .offset(
                i as isize,
            ) = *new_id.offset(*block_ids.offset(i as isize) as isize) as uint8_t;
        i = i.wrapping_add(1);
    }
    return next_id as size_t;
}
unsafe extern "C" fn RemapBlockIdsCommand(
    mut block_ids: *mut uint8_t,
    length: size_t,
    mut new_id: *mut uint16_t,
    num_histograms: size_t,
) -> size_t {
    static mut kInvalidId: uint16_t = 256 as libc::c_int as uint16_t;
    let mut next_id = 0 as libc::c_int as uint16_t;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < num_histograms {
        *new_id.offset(i as isize) = kInvalidId;
        i = i.wrapping_add(1);
    }
    i = 0 as libc::c_int as size_t;
    while i < length {
        if *new_id.offset(*block_ids.offset(i as isize) as isize) as libc::c_int
            == kInvalidId as libc::c_int
        {
            let fresh25 = next_id;
            next_id = next_id.wrapping_add(1);
            *new_id.offset(*block_ids.offset(i as isize) as isize) = fresh25;
        }
        i = i.wrapping_add(1);
    }
    i = 0 as libc::c_int as size_t;
    while i < length {
        *block_ids
            .offset(
                i as isize,
            ) = *new_id.offset(*block_ids.offset(i as isize) as isize) as uint8_t;
        i = i.wrapping_add(1);
    }
    return next_id as size_t;
}
unsafe extern "C" fn RemapBlockIdsDistance(
    mut block_ids: *mut uint8_t,
    length: size_t,
    mut new_id: *mut uint16_t,
    num_histograms: size_t,
) -> size_t {
    static mut kInvalidId: uint16_t = 256 as libc::c_int as uint16_t;
    let mut next_id = 0 as libc::c_int as uint16_t;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < num_histograms {
        *new_id.offset(i as isize) = kInvalidId;
        i = i.wrapping_add(1);
    }
    i = 0 as libc::c_int as size_t;
    while i < length {
        if *new_id.offset(*block_ids.offset(i as isize) as isize) as libc::c_int
            == kInvalidId as libc::c_int
        {
            let fresh26 = next_id;
            next_id = next_id.wrapping_add(1);
            *new_id.offset(*block_ids.offset(i as isize) as isize) = fresh26;
        }
        i = i.wrapping_add(1);
    }
    i = 0 as libc::c_int as size_t;
    while i < length {
        *block_ids
            .offset(
                i as isize,
            ) = *new_id.offset(*block_ids.offset(i as isize) as isize) as uint8_t;
        i = i.wrapping_add(1);
    }
    return next_id as size_t;
}
unsafe extern "C" fn BuildBlockHistogramsLiteral(
    mut data: *const uint8_t,
    length: size_t,
    mut block_ids: *const uint8_t,
    num_histograms: size_t,
    mut histograms: *mut HistogramLiteral,
) {
    let mut i: size_t = 0;
    ClearHistogramsLiteral(histograms, num_histograms);
    i = 0 as libc::c_int as size_t;
    while i < length {
        HistogramAddLiteral(
            &mut *histograms.offset(*block_ids.offset(i as isize) as isize),
            *data.offset(i as isize) as size_t,
        );
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn BuildBlockHistogramsCommand(
    mut data: *const uint16_t,
    length: size_t,
    mut block_ids: *const uint8_t,
    num_histograms: size_t,
    mut histograms: *mut HistogramCommand,
) {
    let mut i: size_t = 0;
    ClearHistogramsCommand(histograms, num_histograms);
    i = 0 as libc::c_int as size_t;
    while i < length {
        HistogramAddCommand(
            &mut *histograms.offset(*block_ids.offset(i as isize) as isize),
            *data.offset(i as isize) as size_t,
        );
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn BuildBlockHistogramsDistance(
    mut data: *const uint16_t,
    length: size_t,
    mut block_ids: *const uint8_t,
    num_histograms: size_t,
    mut histograms: *mut HistogramDistance,
) {
    let mut i: size_t = 0;
    ClearHistogramsDistance(histograms, num_histograms);
    i = 0 as libc::c_int as size_t;
    while i < length {
        HistogramAddDistance(
            &mut *histograms.offset(*block_ids.offset(i as isize) as isize),
            *data.offset(i as isize) as size_t,
        );
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn ClusterBlocksLiteral(
    mut m: *mut MemoryManager,
    mut data: *const uint8_t,
    length: size_t,
    num_blocks: size_t,
    mut block_ids: *mut uint8_t,
    mut split: *mut BlockSplit,
) {
    let mut histogram_symbols = if num_blocks > 0 as libc::c_int as libc::c_ulong {
        BrotliAllocate(
            m,
            num_blocks.wrapping_mul(::std::mem::size_of::<uint32_t>() as libc::c_ulong),
        ) as *mut uint32_t
    } else {
        0 as *mut uint32_t
    };
    let mut block_lengths = if num_blocks > 0 as libc::c_int as libc::c_ulong {
        BrotliAllocate(
            m,
            num_blocks.wrapping_mul(::std::mem::size_of::<uint32_t>() as libc::c_ulong),
        ) as *mut uint32_t
    } else {
        0 as *mut uint32_t
    };
    let expected_num_clusters = (16 as libc::c_int as libc::c_ulong)
        .wrapping_mul(
            num_blocks
                .wrapping_add(64 as libc::c_int as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        )
        .wrapping_div(64 as libc::c_int as libc::c_ulong);
    let mut all_histograms_size = 0 as libc::c_int as size_t;
    let mut all_histograms_capacity = expected_num_clusters;
    let mut all_histograms = if all_histograms_capacity
        > 0 as libc::c_int as libc::c_ulong
    {
        BrotliAllocate(
            m,
            all_histograms_capacity
                .wrapping_mul(::std::mem::size_of::<HistogramLiteral>() as libc::c_ulong),
        ) as *mut HistogramLiteral
    } else {
        0 as *mut HistogramLiteral
    };
    let mut cluster_size_size = 0 as libc::c_int as size_t;
    let mut cluster_size_capacity = expected_num_clusters;
    let mut cluster_size = if cluster_size_capacity > 0 as libc::c_int as libc::c_ulong {
        BrotliAllocate(
            m,
            cluster_size_capacity
                .wrapping_mul(::std::mem::size_of::<uint32_t>() as libc::c_ulong),
        ) as *mut uint32_t
    } else {
        0 as *mut uint32_t
    };
    let mut num_clusters = 0 as libc::c_int as size_t;
    let mut histograms = if brotli_min_size_t(num_blocks, 64 as libc::c_int as size_t)
        > 0 as libc::c_int as libc::c_ulong
    {
        BrotliAllocate(
            m,
            (brotli_min_size_t(num_blocks, 64 as libc::c_int as size_t))
                .wrapping_mul(::std::mem::size_of::<HistogramLiteral>() as libc::c_ulong),
        ) as *mut HistogramLiteral
    } else {
        0 as *mut HistogramLiteral
    };
    let mut max_num_pairs = (64 as libc::c_int * 64 as libc::c_int / 2 as libc::c_int)
        as size_t;
    let mut pairs_capacity = max_num_pairs
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    let mut pairs = if pairs_capacity > 0 as libc::c_int as libc::c_ulong {
        BrotliAllocate(
            m,
            pairs_capacity
                .wrapping_mul(::std::mem::size_of::<HistogramPair>() as libc::c_ulong),
        ) as *mut HistogramPair
    } else {
        0 as *mut HistogramPair
    };
    let mut pos = 0 as libc::c_int as size_t;
    let mut clusters = 0 as *mut uint32_t;
    let mut num_final_clusters: size_t = 0;
    static mut kInvalidIndex: uint32_t = !(0 as libc::c_int as uint32_t);
    let mut new_index = 0 as *mut uint32_t;
    let mut i: size_t = 0;
    let mut sizes: [uint32_t; 64] = [
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
    ];
    let mut new_clusters: [uint32_t; 64] = [
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
    ];
    let mut symbols: [uint32_t; 64] = [
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
    ];
    let mut remap: [uint32_t; 64] = [
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
    ];
    if 0 as libc::c_int != 0 || 0 as libc::c_int != 0 || 0 as libc::c_int != 0
        || 0 as libc::c_int != 0 || 0 as libc::c_int != 0 || 0 as libc::c_int != 0
        || 0 as libc::c_int != 0
    {
        return;
    }
    memset(
        block_lengths as *mut libc::c_void,
        0 as libc::c_int,
        num_blocks.wrapping_mul(::std::mem::size_of::<uint32_t>() as libc::c_ulong),
    );
    let mut block_idx = 0 as libc::c_int as size_t;
    i = 0 as libc::c_int as size_t;
    while i < length {
        let ref mut fresh27 = *block_lengths.offset(block_idx as isize);
        *fresh27 = (*fresh27).wrapping_add(1);
        if i.wrapping_add(1 as libc::c_int as libc::c_ulong) == length
            || *block_ids.offset(i as isize) as libc::c_int
                != *block_ids
                    .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
                    as libc::c_int
        {
            block_idx = block_idx.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    i = 0 as libc::c_int as size_t;
    while i < num_blocks {
        let num_to_combine = brotli_min_size_t(
            num_blocks.wrapping_sub(i),
            64 as libc::c_int as size_t,
        );
        let mut num_new_clusters: size_t = 0;
        let mut j: size_t = 0;
        j = 0 as libc::c_int as size_t;
        while j < num_to_combine {
            let mut k: size_t = 0;
            HistogramClearLiteral(&mut *histograms.offset(j as isize));
            k = 0 as libc::c_int as size_t;
            while k < *block_lengths.offset(i.wrapping_add(j) as isize) as libc::c_ulong
            {
                let fresh28 = pos;
                pos = pos.wrapping_add(1);
                HistogramAddLiteral(
                    &mut *histograms.offset(j as isize),
                    *data.offset(fresh28 as isize) as size_t,
                );
                k = k.wrapping_add(1);
            }
            (*histograms.offset(j as isize))
                .bit_cost_ = BrotliPopulationCostLiteral(
                &mut *histograms.offset(j as isize),
            );
            new_clusters[j as usize] = j as uint32_t;
            symbols[j as usize] = j as uint32_t;
            sizes[j as usize] = 1 as libc::c_int as uint32_t;
            j = j.wrapping_add(1);
        }
        num_new_clusters = BrotliHistogramCombineLiteral(
            histograms,
            sizes.as_mut_ptr(),
            symbols.as_mut_ptr(),
            new_clusters.as_mut_ptr(),
            pairs,
            num_to_combine,
            num_to_combine,
            64 as libc::c_int as size_t,
            max_num_pairs,
        );
        if all_histograms_capacity < all_histograms_size.wrapping_add(num_new_clusters) {
            let mut _new_size = if all_histograms_capacity
                == 0 as libc::c_int as libc::c_ulong
            {
                all_histograms_size.wrapping_add(num_new_clusters)
            } else {
                all_histograms_capacity
            };
            let mut new_array = 0 as *mut HistogramLiteral;
            while _new_size < all_histograms_size.wrapping_add(num_new_clusters) {
                _new_size = (_new_size as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
            }
            new_array = if _new_size > 0 as libc::c_int as libc::c_ulong {
                BrotliAllocate(
                    m,
                    _new_size
                        .wrapping_mul(
                            ::std::mem::size_of::<HistogramLiteral>() as libc::c_ulong,
                        ),
                ) as *mut HistogramLiteral
            } else {
                0 as *mut HistogramLiteral
            };
            if 0 as libc::c_int == 0 && 0 as libc::c_int == 0
                && all_histograms_capacity != 0 as libc::c_int as libc::c_ulong
            {
                memcpy(
                    new_array as *mut libc::c_void,
                    all_histograms as *const libc::c_void,
                    all_histograms_capacity
                        .wrapping_mul(
                            ::std::mem::size_of::<HistogramLiteral>() as libc::c_ulong,
                        ),
                );
            }
            BrotliFree(m, all_histograms as *mut libc::c_void);
            all_histograms = 0 as *mut HistogramLiteral;
            all_histograms = new_array;
            all_histograms_capacity = _new_size;
        }
        if cluster_size_capacity < cluster_size_size.wrapping_add(num_new_clusters) {
            let mut _new_size_0 = if cluster_size_capacity
                == 0 as libc::c_int as libc::c_ulong
            {
                cluster_size_size.wrapping_add(num_new_clusters)
            } else {
                cluster_size_capacity
            };
            let mut new_array_0 = 0 as *mut uint32_t;
            while _new_size_0 < cluster_size_size.wrapping_add(num_new_clusters) {
                _new_size_0 = (_new_size_0 as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
            }
            new_array_0 = if _new_size_0 > 0 as libc::c_int as libc::c_ulong {
                BrotliAllocate(
                    m,
                    _new_size_0
                        .wrapping_mul(::std::mem::size_of::<uint32_t>() as libc::c_ulong),
                ) as *mut uint32_t
            } else {
                0 as *mut uint32_t
            };
            if 0 as libc::c_int == 0 && 0 as libc::c_int == 0
                && cluster_size_capacity != 0 as libc::c_int as libc::c_ulong
            {
                memcpy(
                    new_array_0 as *mut libc::c_void,
                    cluster_size as *const libc::c_void,
                    cluster_size_capacity
                        .wrapping_mul(::std::mem::size_of::<uint32_t>() as libc::c_ulong),
                );
            }
            BrotliFree(m, cluster_size as *mut libc::c_void);
            cluster_size = 0 as *mut uint32_t;
            cluster_size = new_array_0;
            cluster_size_capacity = _new_size_0;
        }
        if 0 as libc::c_int != 0 {
            return;
        }
        j = 0 as libc::c_int as size_t;
        while j < num_new_clusters {
            let fresh29 = all_histograms_size;
            all_histograms_size = all_histograms_size.wrapping_add(1);
            *all_histograms
                .offset(
                    fresh29 as isize,
                ) = *histograms.offset(new_clusters[j as usize] as isize);
            let fresh30 = cluster_size_size;
            cluster_size_size = cluster_size_size.wrapping_add(1);
            *cluster_size
                .offset(fresh30 as isize) = sizes[new_clusters[j as usize] as usize];
            remap[new_clusters[j as usize] as usize] = j as uint32_t;
            j = j.wrapping_add(1);
        }
        j = 0 as libc::c_int as size_t;
        while j < num_to_combine {
            *histogram_symbols
                .offset(
                    i.wrapping_add(j) as isize,
                ) = (num_clusters as uint32_t)
                .wrapping_add(remap[symbols[j as usize] as usize]);
            j = j.wrapping_add(1);
        }
        num_clusters = (num_clusters as libc::c_ulong).wrapping_add(num_new_clusters)
            as size_t as size_t;
        i = (i as libc::c_ulong).wrapping_add(64 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
    }
    BrotliFree(m, histograms as *mut libc::c_void);
    histograms = 0 as *mut HistogramLiteral;
    max_num_pairs = brotli_min_size_t(
        (64 as libc::c_int as libc::c_ulong).wrapping_mul(num_clusters),
        num_clusters
            .wrapping_div(2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(num_clusters),
    );
    if pairs_capacity < max_num_pairs.wrapping_add(1 as libc::c_int as libc::c_ulong) {
        BrotliFree(m, pairs as *mut libc::c_void);
        pairs = 0 as *mut HistogramPair;
        pairs = if max_num_pairs.wrapping_add(1 as libc::c_int as libc::c_ulong)
            > 0 as libc::c_int as libc::c_ulong
        {
            BrotliAllocate(
                m,
                max_num_pairs
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<HistogramPair>() as libc::c_ulong,
                    ),
            ) as *mut HistogramPair
        } else {
            0 as *mut HistogramPair
        };
        if 0 as libc::c_int != 0 || 0 as libc::c_int != 0 {
            return;
        }
    }
    clusters = if num_clusters > 0 as libc::c_int as libc::c_ulong {
        BrotliAllocate(
            m,
            num_clusters.wrapping_mul(::std::mem::size_of::<uint32_t>() as libc::c_ulong),
        ) as *mut uint32_t
    } else {
        0 as *mut uint32_t
    };
    if 0 as libc::c_int != 0 || 0 as libc::c_int != 0 {
        return;
    }
    i = 0 as libc::c_int as size_t;
    while i < num_clusters {
        *clusters.offset(i as isize) = i as uint32_t;
        i = i.wrapping_add(1);
    }
    num_final_clusters = BrotliHistogramCombineLiteral(
        all_histograms,
        cluster_size,
        histogram_symbols,
        clusters,
        pairs,
        num_clusters,
        num_blocks,
        256 as libc::c_int as size_t,
        max_num_pairs,
    );
    BrotliFree(m, pairs as *mut libc::c_void);
    pairs = 0 as *mut HistogramPair;
    BrotliFree(m, cluster_size as *mut libc::c_void);
    cluster_size = 0 as *mut uint32_t;
    new_index = if num_clusters > 0 as libc::c_int as libc::c_ulong {
        BrotliAllocate(
            m,
            num_clusters.wrapping_mul(::std::mem::size_of::<uint32_t>() as libc::c_ulong),
        ) as *mut uint32_t
    } else {
        0 as *mut uint32_t
    };
    if 0 as libc::c_int != 0 || 0 as libc::c_int != 0 {
        return;
    }
    i = 0 as libc::c_int as size_t;
    while i < num_clusters {
        *new_index.offset(i as isize) = kInvalidIndex;
        i = i.wrapping_add(1);
    }
    pos = 0 as libc::c_int as size_t;
    let mut next_index = 0 as libc::c_int as uint32_t;
    i = 0 as libc::c_int as size_t;
    while i < num_blocks {
        let mut histo = HistogramLiteral {
            data_: [0; 256],
            total_count_: 0,
            bit_cost_: 0.,
        };
        let mut j_0: size_t = 0;
        let mut best_out: uint32_t = 0;
        let mut best_bits: libc::c_double = 0.;
        HistogramClearLiteral(&mut histo);
        j_0 = 0 as libc::c_int as size_t;
        while j_0 < *block_lengths.offset(i as isize) as libc::c_ulong {
            let fresh31 = pos;
            pos = pos.wrapping_add(1);
            HistogramAddLiteral(&mut histo, *data.offset(fresh31 as isize) as size_t);
            j_0 = j_0.wrapping_add(1);
        }
        best_out = if i == 0 as libc::c_int as libc::c_ulong {
            *histogram_symbols.offset(0 as libc::c_int as isize)
        } else {
            *histogram_symbols
                .offset(i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
        };
        best_bits = BrotliHistogramBitCostDistanceLiteral(
            &mut histo,
            &mut *all_histograms.offset(best_out as isize),
        );
        j_0 = 0 as libc::c_int as size_t;
        while j_0 < num_final_clusters {
            let cur_bits = BrotliHistogramBitCostDistanceLiteral(
                &mut histo,
                &mut *all_histograms.offset(*clusters.offset(j_0 as isize) as isize),
            );
            if cur_bits < best_bits {
                best_bits = cur_bits;
                best_out = *clusters.offset(j_0 as isize);
            }
            j_0 = j_0.wrapping_add(1);
        }
        *histogram_symbols.offset(i as isize) = best_out;
        if *new_index.offset(best_out as isize) == kInvalidIndex {
            let fresh32 = next_index;
            next_index = next_index.wrapping_add(1);
            *new_index.offset(best_out as isize) = fresh32;
        }
        i = i.wrapping_add(1);
    }
    BrotliFree(m, clusters as *mut libc::c_void);
    clusters = 0 as *mut uint32_t;
    BrotliFree(m, all_histograms as *mut libc::c_void);
    all_histograms = 0 as *mut HistogramLiteral;
    if (*split).types_alloc_size < num_blocks {
        let mut _new_size_1 = if (*split).types_alloc_size
            == 0 as libc::c_int as libc::c_ulong
        {
            num_blocks
        } else {
            (*split).types_alloc_size
        };
        let mut new_array_1 = 0 as *mut uint8_t;
        while _new_size_1 < num_blocks {
            _new_size_1 = (_new_size_1 as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
        }
        new_array_1 = if _new_size_1 > 0 as libc::c_int as libc::c_ulong {
            BrotliAllocate(
                m,
                _new_size_1
                    .wrapping_mul(::std::mem::size_of::<uint8_t>() as libc::c_ulong),
            ) as *mut uint8_t
        } else {
            0 as *mut uint8_t
        };
        if 0 as libc::c_int == 0 && 0 as libc::c_int == 0
            && (*split).types_alloc_size != 0 as libc::c_int as libc::c_ulong
        {
            memcpy(
                new_array_1 as *mut libc::c_void,
                (*split).types as *const libc::c_void,
                ((*split).types_alloc_size)
                    .wrapping_mul(::std::mem::size_of::<uint8_t>() as libc::c_ulong),
            );
        }
        BrotliFree(m, (*split).types as *mut libc::c_void);
        let ref mut fresh33 = (*split).types;
        *fresh33 = 0 as *mut uint8_t;
        let ref mut fresh34 = (*split).types;
        *fresh34 = new_array_1;
        (*split).types_alloc_size = _new_size_1;
    }
    if (*split).lengths_alloc_size < num_blocks {
        let mut _new_size_2 = if (*split).lengths_alloc_size
            == 0 as libc::c_int as libc::c_ulong
        {
            num_blocks
        } else {
            (*split).lengths_alloc_size
        };
        let mut new_array_2 = 0 as *mut uint32_t;
        while _new_size_2 < num_blocks {
            _new_size_2 = (_new_size_2 as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
        }
        new_array_2 = if _new_size_2 > 0 as libc::c_int as libc::c_ulong {
            BrotliAllocate(
                m,
                _new_size_2
                    .wrapping_mul(::std::mem::size_of::<uint32_t>() as libc::c_ulong),
            ) as *mut uint32_t
        } else {
            0 as *mut uint32_t
        };
        if 0 as libc::c_int == 0 && 0 as libc::c_int == 0
            && (*split).lengths_alloc_size != 0 as libc::c_int as libc::c_ulong
        {
            memcpy(
                new_array_2 as *mut libc::c_void,
                (*split).lengths as *const libc::c_void,
                ((*split).lengths_alloc_size)
                    .wrapping_mul(::std::mem::size_of::<uint32_t>() as libc::c_ulong),
            );
        }
        BrotliFree(m, (*split).lengths as *mut libc::c_void);
        let ref mut fresh35 = (*split).lengths;
        *fresh35 = 0 as *mut uint32_t;
        let ref mut fresh36 = (*split).lengths;
        *fresh36 = new_array_2;
        (*split).lengths_alloc_size = _new_size_2;
    }
    if 0 as libc::c_int != 0 {
        return;
    }
    let mut cur_length = 0 as libc::c_int as uint32_t;
    let mut block_idx_0 = 0 as libc::c_int as size_t;
    let mut max_type = 0 as libc::c_int as uint8_t;
    i = 0 as libc::c_int as size_t;
    while i < num_blocks {
        cur_length = (cur_length as libc::c_uint)
            .wrapping_add(*block_lengths.offset(i as isize)) as uint32_t as uint32_t;
        if i.wrapping_add(1 as libc::c_int as libc::c_ulong) == num_blocks
            || *histogram_symbols.offset(i as isize)
                != *histogram_symbols
                    .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
        {
            let id = *new_index.offset(*histogram_symbols.offset(i as isize) as isize)
                as uint8_t;
            *((*split).types).offset(block_idx_0 as isize) = id;
            *((*split).lengths).offset(block_idx_0 as isize) = cur_length;
            max_type = brotli_max_uint8_t(max_type, id);
            cur_length = 0 as libc::c_int as uint32_t;
            block_idx_0 = block_idx_0.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    (*split).num_blocks = block_idx_0;
    (*split)
        .num_types = (max_type as size_t)
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    BrotliFree(m, new_index as *mut libc::c_void);
    new_index = 0 as *mut uint32_t;
    BrotliFree(m, block_lengths as *mut libc::c_void);
    block_lengths = 0 as *mut uint32_t;
    BrotliFree(m, histogram_symbols as *mut libc::c_void);
    histogram_symbols = 0 as *mut uint32_t;
}
unsafe extern "C" fn ClusterBlocksDistance(
    mut m: *mut MemoryManager,
    mut data: *const uint16_t,
    length: size_t,
    num_blocks: size_t,
    mut block_ids: *mut uint8_t,
    mut split: *mut BlockSplit,
) {
    let mut histogram_symbols = if num_blocks > 0 as libc::c_int as libc::c_ulong {
        BrotliAllocate(
            m,
            num_blocks.wrapping_mul(::std::mem::size_of::<uint32_t>() as libc::c_ulong),
        ) as *mut uint32_t
    } else {
        0 as *mut uint32_t
    };
    let mut block_lengths = if num_blocks > 0 as libc::c_int as libc::c_ulong {
        BrotliAllocate(
            m,
            num_blocks.wrapping_mul(::std::mem::size_of::<uint32_t>() as libc::c_ulong),
        ) as *mut uint32_t
    } else {
        0 as *mut uint32_t
    };
    let expected_num_clusters = (16 as libc::c_int as libc::c_ulong)
        .wrapping_mul(
            num_blocks
                .wrapping_add(64 as libc::c_int as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        )
        .wrapping_div(64 as libc::c_int as libc::c_ulong);
    let mut all_histograms_size = 0 as libc::c_int as size_t;
    let mut all_histograms_capacity = expected_num_clusters;
    let mut all_histograms = if all_histograms_capacity
        > 0 as libc::c_int as libc::c_ulong
    {
        BrotliAllocate(
            m,
            all_histograms_capacity
                .wrapping_mul(
                    ::std::mem::size_of::<HistogramDistance>() as libc::c_ulong,
                ),
        ) as *mut HistogramDistance
    } else {
        0 as *mut HistogramDistance
    };
    let mut cluster_size_size = 0 as libc::c_int as size_t;
    let mut cluster_size_capacity = expected_num_clusters;
    let mut cluster_size = if cluster_size_capacity > 0 as libc::c_int as libc::c_ulong {
        BrotliAllocate(
            m,
            cluster_size_capacity
                .wrapping_mul(::std::mem::size_of::<uint32_t>() as libc::c_ulong),
        ) as *mut uint32_t
    } else {
        0 as *mut uint32_t
    };
    let mut num_clusters = 0 as libc::c_int as size_t;
    let mut histograms = if brotli_min_size_t(num_blocks, 64 as libc::c_int as size_t)
        > 0 as libc::c_int as libc::c_ulong
    {
        BrotliAllocate(
            m,
            (brotli_min_size_t(num_blocks, 64 as libc::c_int as size_t))
                .wrapping_mul(
                    ::std::mem::size_of::<HistogramDistance>() as libc::c_ulong,
                ),
        ) as *mut HistogramDistance
    } else {
        0 as *mut HistogramDistance
    };
    let mut max_num_pairs = (64 as libc::c_int * 64 as libc::c_int / 2 as libc::c_int)
        as size_t;
    let mut pairs_capacity = max_num_pairs
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    let mut pairs = if pairs_capacity > 0 as libc::c_int as libc::c_ulong {
        BrotliAllocate(
            m,
            pairs_capacity
                .wrapping_mul(::std::mem::size_of::<HistogramPair>() as libc::c_ulong),
        ) as *mut HistogramPair
    } else {
        0 as *mut HistogramPair
    };
    let mut pos = 0 as libc::c_int as size_t;
    let mut clusters = 0 as *mut uint32_t;
    let mut num_final_clusters: size_t = 0;
    static mut kInvalidIndex: uint32_t = !(0 as libc::c_int as uint32_t);
    let mut new_index = 0 as *mut uint32_t;
    let mut i: size_t = 0;
    let mut sizes: [uint32_t; 64] = [
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
    ];
    let mut new_clusters: [uint32_t; 64] = [
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
    ];
    let mut symbols: [uint32_t; 64] = [
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
    ];
    let mut remap: [uint32_t; 64] = [
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
    ];
    if 0 as libc::c_int != 0 || 0 as libc::c_int != 0 || 0 as libc::c_int != 0
        || 0 as libc::c_int != 0 || 0 as libc::c_int != 0 || 0 as libc::c_int != 0
        || 0 as libc::c_int != 0
    {
        return;
    }
    memset(
        block_lengths as *mut libc::c_void,
        0 as libc::c_int,
        num_blocks.wrapping_mul(::std::mem::size_of::<uint32_t>() as libc::c_ulong),
    );
    let mut block_idx = 0 as libc::c_int as size_t;
    i = 0 as libc::c_int as size_t;
    while i < length {
        let ref mut fresh37 = *block_lengths.offset(block_idx as isize);
        *fresh37 = (*fresh37).wrapping_add(1);
        if i.wrapping_add(1 as libc::c_int as libc::c_ulong) == length
            || *block_ids.offset(i as isize) as libc::c_int
                != *block_ids
                    .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
                    as libc::c_int
        {
            block_idx = block_idx.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    i = 0 as libc::c_int as size_t;
    while i < num_blocks {
        let num_to_combine = brotli_min_size_t(
            num_blocks.wrapping_sub(i),
            64 as libc::c_int as size_t,
        );
        let mut num_new_clusters: size_t = 0;
        let mut j: size_t = 0;
        j = 0 as libc::c_int as size_t;
        while j < num_to_combine {
            let mut k: size_t = 0;
            HistogramClearDistance(&mut *histograms.offset(j as isize));
            k = 0 as libc::c_int as size_t;
            while k < *block_lengths.offset(i.wrapping_add(j) as isize) as libc::c_ulong
            {
                let fresh38 = pos;
                pos = pos.wrapping_add(1);
                HistogramAddDistance(
                    &mut *histograms.offset(j as isize),
                    *data.offset(fresh38 as isize) as size_t,
                );
                k = k.wrapping_add(1);
            }
            (*histograms.offset(j as isize))
                .bit_cost_ = BrotliPopulationCostDistance(
                &mut *histograms.offset(j as isize),
            );
            new_clusters[j as usize] = j as uint32_t;
            symbols[j as usize] = j as uint32_t;
            sizes[j as usize] = 1 as libc::c_int as uint32_t;
            j = j.wrapping_add(1);
        }
        num_new_clusters = BrotliHistogramCombineDistance(
            histograms,
            sizes.as_mut_ptr(),
            symbols.as_mut_ptr(),
            new_clusters.as_mut_ptr(),
            pairs,
            num_to_combine,
            num_to_combine,
            64 as libc::c_int as size_t,
            max_num_pairs,
        );
        if all_histograms_capacity < all_histograms_size.wrapping_add(num_new_clusters) {
            let mut _new_size = if all_histograms_capacity
                == 0 as libc::c_int as libc::c_ulong
            {
                all_histograms_size.wrapping_add(num_new_clusters)
            } else {
                all_histograms_capacity
            };
            let mut new_array = 0 as *mut HistogramDistance;
            while _new_size < all_histograms_size.wrapping_add(num_new_clusters) {
                _new_size = (_new_size as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
            }
            new_array = if _new_size > 0 as libc::c_int as libc::c_ulong {
                BrotliAllocate(
                    m,
                    _new_size
                        .wrapping_mul(
                            ::std::mem::size_of::<HistogramDistance>() as libc::c_ulong,
                        ),
                ) as *mut HistogramDistance
            } else {
                0 as *mut HistogramDistance
            };
            if 0 as libc::c_int == 0 && 0 as libc::c_int == 0
                && all_histograms_capacity != 0 as libc::c_int as libc::c_ulong
            {
                memcpy(
                    new_array as *mut libc::c_void,
                    all_histograms as *const libc::c_void,
                    all_histograms_capacity
                        .wrapping_mul(
                            ::std::mem::size_of::<HistogramDistance>() as libc::c_ulong,
                        ),
                );
            }
            BrotliFree(m, all_histograms as *mut libc::c_void);
            all_histograms = 0 as *mut HistogramDistance;
            all_histograms = new_array;
            all_histograms_capacity = _new_size;
        }
        if cluster_size_capacity < cluster_size_size.wrapping_add(num_new_clusters) {
            let mut _new_size_0 = if cluster_size_capacity
                == 0 as libc::c_int as libc::c_ulong
            {
                cluster_size_size.wrapping_add(num_new_clusters)
            } else {
                cluster_size_capacity
            };
            let mut new_array_0 = 0 as *mut uint32_t;
            while _new_size_0 < cluster_size_size.wrapping_add(num_new_clusters) {
                _new_size_0 = (_new_size_0 as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
            }
            new_array_0 = if _new_size_0 > 0 as libc::c_int as libc::c_ulong {
                BrotliAllocate(
                    m,
                    _new_size_0
                        .wrapping_mul(::std::mem::size_of::<uint32_t>() as libc::c_ulong),
                ) as *mut uint32_t
            } else {
                0 as *mut uint32_t
            };
            if 0 as libc::c_int == 0 && 0 as libc::c_int == 0
                && cluster_size_capacity != 0 as libc::c_int as libc::c_ulong
            {
                memcpy(
                    new_array_0 as *mut libc::c_void,
                    cluster_size as *const libc::c_void,
                    cluster_size_capacity
                        .wrapping_mul(::std::mem::size_of::<uint32_t>() as libc::c_ulong),
                );
            }
            BrotliFree(m, cluster_size as *mut libc::c_void);
            cluster_size = 0 as *mut uint32_t;
            cluster_size = new_array_0;
            cluster_size_capacity = _new_size_0;
        }
        if 0 as libc::c_int != 0 {
            return;
        }
        j = 0 as libc::c_int as size_t;
        while j < num_new_clusters {
            let fresh39 = all_histograms_size;
            all_histograms_size = all_histograms_size.wrapping_add(1);
            *all_histograms
                .offset(
                    fresh39 as isize,
                ) = *histograms.offset(new_clusters[j as usize] as isize);
            let fresh40 = cluster_size_size;
            cluster_size_size = cluster_size_size.wrapping_add(1);
            *cluster_size
                .offset(fresh40 as isize) = sizes[new_clusters[j as usize] as usize];
            remap[new_clusters[j as usize] as usize] = j as uint32_t;
            j = j.wrapping_add(1);
        }
        j = 0 as libc::c_int as size_t;
        while j < num_to_combine {
            *histogram_symbols
                .offset(
                    i.wrapping_add(j) as isize,
                ) = (num_clusters as uint32_t)
                .wrapping_add(remap[symbols[j as usize] as usize]);
            j = j.wrapping_add(1);
        }
        num_clusters = (num_clusters as libc::c_ulong).wrapping_add(num_new_clusters)
            as size_t as size_t;
        i = (i as libc::c_ulong).wrapping_add(64 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
    }
    BrotliFree(m, histograms as *mut libc::c_void);
    histograms = 0 as *mut HistogramDistance;
    max_num_pairs = brotli_min_size_t(
        (64 as libc::c_int as libc::c_ulong).wrapping_mul(num_clusters),
        num_clusters
            .wrapping_div(2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(num_clusters),
    );
    if pairs_capacity < max_num_pairs.wrapping_add(1 as libc::c_int as libc::c_ulong) {
        BrotliFree(m, pairs as *mut libc::c_void);
        pairs = 0 as *mut HistogramPair;
        pairs = if max_num_pairs.wrapping_add(1 as libc::c_int as libc::c_ulong)
            > 0 as libc::c_int as libc::c_ulong
        {
            BrotliAllocate(
                m,
                max_num_pairs
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<HistogramPair>() as libc::c_ulong,
                    ),
            ) as *mut HistogramPair
        } else {
            0 as *mut HistogramPair
        };
        if 0 as libc::c_int != 0 || 0 as libc::c_int != 0 {
            return;
        }
    }
    clusters = if num_clusters > 0 as libc::c_int as libc::c_ulong {
        BrotliAllocate(
            m,
            num_clusters.wrapping_mul(::std::mem::size_of::<uint32_t>() as libc::c_ulong),
        ) as *mut uint32_t
    } else {
        0 as *mut uint32_t
    };
    if 0 as libc::c_int != 0 || 0 as libc::c_int != 0 {
        return;
    }
    i = 0 as libc::c_int as size_t;
    while i < num_clusters {
        *clusters.offset(i as isize) = i as uint32_t;
        i = i.wrapping_add(1);
    }
    num_final_clusters = BrotliHistogramCombineDistance(
        all_histograms,
        cluster_size,
        histogram_symbols,
        clusters,
        pairs,
        num_clusters,
        num_blocks,
        256 as libc::c_int as size_t,
        max_num_pairs,
    );
    BrotliFree(m, pairs as *mut libc::c_void);
    pairs = 0 as *mut HistogramPair;
    BrotliFree(m, cluster_size as *mut libc::c_void);
    cluster_size = 0 as *mut uint32_t;
    new_index = if num_clusters > 0 as libc::c_int as libc::c_ulong {
        BrotliAllocate(
            m,
            num_clusters.wrapping_mul(::std::mem::size_of::<uint32_t>() as libc::c_ulong),
        ) as *mut uint32_t
    } else {
        0 as *mut uint32_t
    };
    if 0 as libc::c_int != 0 || 0 as libc::c_int != 0 {
        return;
    }
    i = 0 as libc::c_int as size_t;
    while i < num_clusters {
        *new_index.offset(i as isize) = kInvalidIndex;
        i = i.wrapping_add(1);
    }
    pos = 0 as libc::c_int as size_t;
    let mut next_index = 0 as libc::c_int as uint32_t;
    i = 0 as libc::c_int as size_t;
    while i < num_blocks {
        let mut histo = HistogramDistance {
            data_: [0; 544],
            total_count_: 0,
            bit_cost_: 0.,
        };
        let mut j_0: size_t = 0;
        let mut best_out: uint32_t = 0;
        let mut best_bits: libc::c_double = 0.;
        HistogramClearDistance(&mut histo);
        j_0 = 0 as libc::c_int as size_t;
        while j_0 < *block_lengths.offset(i as isize) as libc::c_ulong {
            let fresh41 = pos;
            pos = pos.wrapping_add(1);
            HistogramAddDistance(&mut histo, *data.offset(fresh41 as isize) as size_t);
            j_0 = j_0.wrapping_add(1);
        }
        best_out = if i == 0 as libc::c_int as libc::c_ulong {
            *histogram_symbols.offset(0 as libc::c_int as isize)
        } else {
            *histogram_symbols
                .offset(i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
        };
        best_bits = BrotliHistogramBitCostDistanceDistance(
            &mut histo,
            &mut *all_histograms.offset(best_out as isize),
        );
        j_0 = 0 as libc::c_int as size_t;
        while j_0 < num_final_clusters {
            let cur_bits = BrotliHistogramBitCostDistanceDistance(
                &mut histo,
                &mut *all_histograms.offset(*clusters.offset(j_0 as isize) as isize),
            );
            if cur_bits < best_bits {
                best_bits = cur_bits;
                best_out = *clusters.offset(j_0 as isize);
            }
            j_0 = j_0.wrapping_add(1);
        }
        *histogram_symbols.offset(i as isize) = best_out;
        if *new_index.offset(best_out as isize) == kInvalidIndex {
            let fresh42 = next_index;
            next_index = next_index.wrapping_add(1);
            *new_index.offset(best_out as isize) = fresh42;
        }
        i = i.wrapping_add(1);
    }
    BrotliFree(m, clusters as *mut libc::c_void);
    clusters = 0 as *mut uint32_t;
    BrotliFree(m, all_histograms as *mut libc::c_void);
    all_histograms = 0 as *mut HistogramDistance;
    if (*split).types_alloc_size < num_blocks {
        let mut _new_size_1 = if (*split).types_alloc_size
            == 0 as libc::c_int as libc::c_ulong
        {
            num_blocks
        } else {
            (*split).types_alloc_size
        };
        let mut new_array_1 = 0 as *mut uint8_t;
        while _new_size_1 < num_blocks {
            _new_size_1 = (_new_size_1 as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
        }
        new_array_1 = if _new_size_1 > 0 as libc::c_int as libc::c_ulong {
            BrotliAllocate(
                m,
                _new_size_1
                    .wrapping_mul(::std::mem::size_of::<uint8_t>() as libc::c_ulong),
            ) as *mut uint8_t
        } else {
            0 as *mut uint8_t
        };
        if 0 as libc::c_int == 0 && 0 as libc::c_int == 0
            && (*split).types_alloc_size != 0 as libc::c_int as libc::c_ulong
        {
            memcpy(
                new_array_1 as *mut libc::c_void,
                (*split).types as *const libc::c_void,
                ((*split).types_alloc_size)
                    .wrapping_mul(::std::mem::size_of::<uint8_t>() as libc::c_ulong),
            );
        }
        BrotliFree(m, (*split).types as *mut libc::c_void);
        let ref mut fresh43 = (*split).types;
        *fresh43 = 0 as *mut uint8_t;
        let ref mut fresh44 = (*split).types;
        *fresh44 = new_array_1;
        (*split).types_alloc_size = _new_size_1;
    }
    if (*split).lengths_alloc_size < num_blocks {
        let mut _new_size_2 = if (*split).lengths_alloc_size
            == 0 as libc::c_int as libc::c_ulong
        {
            num_blocks
        } else {
            (*split).lengths_alloc_size
        };
        let mut new_array_2 = 0 as *mut uint32_t;
        while _new_size_2 < num_blocks {
            _new_size_2 = (_new_size_2 as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
        }
        new_array_2 = if _new_size_2 > 0 as libc::c_int as libc::c_ulong {
            BrotliAllocate(
                m,
                _new_size_2
                    .wrapping_mul(::std::mem::size_of::<uint32_t>() as libc::c_ulong),
            ) as *mut uint32_t
        } else {
            0 as *mut uint32_t
        };
        if 0 as libc::c_int == 0 && 0 as libc::c_int == 0
            && (*split).lengths_alloc_size != 0 as libc::c_int as libc::c_ulong
        {
            memcpy(
                new_array_2 as *mut libc::c_void,
                (*split).lengths as *const libc::c_void,
                ((*split).lengths_alloc_size)
                    .wrapping_mul(::std::mem::size_of::<uint32_t>() as libc::c_ulong),
            );
        }
        BrotliFree(m, (*split).lengths as *mut libc::c_void);
        let ref mut fresh45 = (*split).lengths;
        *fresh45 = 0 as *mut uint32_t;
        let ref mut fresh46 = (*split).lengths;
        *fresh46 = new_array_2;
        (*split).lengths_alloc_size = _new_size_2;
    }
    if 0 as libc::c_int != 0 {
        return;
    }
    let mut cur_length = 0 as libc::c_int as uint32_t;
    let mut block_idx_0 = 0 as libc::c_int as size_t;
    let mut max_type = 0 as libc::c_int as uint8_t;
    i = 0 as libc::c_int as size_t;
    while i < num_blocks {
        cur_length = (cur_length as libc::c_uint)
            .wrapping_add(*block_lengths.offset(i as isize)) as uint32_t as uint32_t;
        if i.wrapping_add(1 as libc::c_int as libc::c_ulong) == num_blocks
            || *histogram_symbols.offset(i as isize)
                != *histogram_symbols
                    .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
        {
            let id = *new_index.offset(*histogram_symbols.offset(i as isize) as isize)
                as uint8_t;
            *((*split).types).offset(block_idx_0 as isize) = id;
            *((*split).lengths).offset(block_idx_0 as isize) = cur_length;
            max_type = brotli_max_uint8_t(max_type, id);
            cur_length = 0 as libc::c_int as uint32_t;
            block_idx_0 = block_idx_0.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    (*split).num_blocks = block_idx_0;
    (*split)
        .num_types = (max_type as size_t)
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    BrotliFree(m, new_index as *mut libc::c_void);
    new_index = 0 as *mut uint32_t;
    BrotliFree(m, block_lengths as *mut libc::c_void);
    block_lengths = 0 as *mut uint32_t;
    BrotliFree(m, histogram_symbols as *mut libc::c_void);
    histogram_symbols = 0 as *mut uint32_t;
}
unsafe extern "C" fn ClusterBlocksCommand(
    mut m: *mut MemoryManager,
    mut data: *const uint16_t,
    length: size_t,
    num_blocks: size_t,
    mut block_ids: *mut uint8_t,
    mut split: *mut BlockSplit,
) {
    let mut histogram_symbols = if num_blocks > 0 as libc::c_int as libc::c_ulong {
        BrotliAllocate(
            m,
            num_blocks.wrapping_mul(::std::mem::size_of::<uint32_t>() as libc::c_ulong),
        ) as *mut uint32_t
    } else {
        0 as *mut uint32_t
    };
    let mut block_lengths = if num_blocks > 0 as libc::c_int as libc::c_ulong {
        BrotliAllocate(
            m,
            num_blocks.wrapping_mul(::std::mem::size_of::<uint32_t>() as libc::c_ulong),
        ) as *mut uint32_t
    } else {
        0 as *mut uint32_t
    };
    let expected_num_clusters = (16 as libc::c_int as libc::c_ulong)
        .wrapping_mul(
            num_blocks
                .wrapping_add(64 as libc::c_int as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        )
        .wrapping_div(64 as libc::c_int as libc::c_ulong);
    let mut all_histograms_size = 0 as libc::c_int as size_t;
    let mut all_histograms_capacity = expected_num_clusters;
    let mut all_histograms = if all_histograms_capacity
        > 0 as libc::c_int as libc::c_ulong
    {
        BrotliAllocate(
            m,
            all_histograms_capacity
                .wrapping_mul(::std::mem::size_of::<HistogramCommand>() as libc::c_ulong),
        ) as *mut HistogramCommand
    } else {
        0 as *mut HistogramCommand
    };
    let mut cluster_size_size = 0 as libc::c_int as size_t;
    let mut cluster_size_capacity = expected_num_clusters;
    let mut cluster_size = if cluster_size_capacity > 0 as libc::c_int as libc::c_ulong {
        BrotliAllocate(
            m,
            cluster_size_capacity
                .wrapping_mul(::std::mem::size_of::<uint32_t>() as libc::c_ulong),
        ) as *mut uint32_t
    } else {
        0 as *mut uint32_t
    };
    let mut num_clusters = 0 as libc::c_int as size_t;
    let mut histograms = if brotli_min_size_t(num_blocks, 64 as libc::c_int as size_t)
        > 0 as libc::c_int as libc::c_ulong
    {
        BrotliAllocate(
            m,
            (brotli_min_size_t(num_blocks, 64 as libc::c_int as size_t))
                .wrapping_mul(::std::mem::size_of::<HistogramCommand>() as libc::c_ulong),
        ) as *mut HistogramCommand
    } else {
        0 as *mut HistogramCommand
    };
    let mut max_num_pairs = (64 as libc::c_int * 64 as libc::c_int / 2 as libc::c_int)
        as size_t;
    let mut pairs_capacity = max_num_pairs
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    let mut pairs = if pairs_capacity > 0 as libc::c_int as libc::c_ulong {
        BrotliAllocate(
            m,
            pairs_capacity
                .wrapping_mul(::std::mem::size_of::<HistogramPair>() as libc::c_ulong),
        ) as *mut HistogramPair
    } else {
        0 as *mut HistogramPair
    };
    let mut pos = 0 as libc::c_int as size_t;
    let mut clusters = 0 as *mut uint32_t;
    let mut num_final_clusters: size_t = 0;
    static mut kInvalidIndex: uint32_t = !(0 as libc::c_int as uint32_t);
    let mut new_index = 0 as *mut uint32_t;
    let mut i: size_t = 0;
    let mut sizes: [uint32_t; 64] = [
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
    ];
    let mut new_clusters: [uint32_t; 64] = [
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
    ];
    let mut symbols: [uint32_t; 64] = [
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
    ];
    let mut remap: [uint32_t; 64] = [
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
    ];
    if 0 as libc::c_int != 0 || 0 as libc::c_int != 0 || 0 as libc::c_int != 0
        || 0 as libc::c_int != 0 || 0 as libc::c_int != 0 || 0 as libc::c_int != 0
        || 0 as libc::c_int != 0
    {
        return;
    }
    memset(
        block_lengths as *mut libc::c_void,
        0 as libc::c_int,
        num_blocks.wrapping_mul(::std::mem::size_of::<uint32_t>() as libc::c_ulong),
    );
    let mut block_idx = 0 as libc::c_int as size_t;
    i = 0 as libc::c_int as size_t;
    while i < length {
        let ref mut fresh47 = *block_lengths.offset(block_idx as isize);
        *fresh47 = (*fresh47).wrapping_add(1);
        if i.wrapping_add(1 as libc::c_int as libc::c_ulong) == length
            || *block_ids.offset(i as isize) as libc::c_int
                != *block_ids
                    .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
                    as libc::c_int
        {
            block_idx = block_idx.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    i = 0 as libc::c_int as size_t;
    while i < num_blocks {
        let num_to_combine = brotli_min_size_t(
            num_blocks.wrapping_sub(i),
            64 as libc::c_int as size_t,
        );
        let mut num_new_clusters: size_t = 0;
        let mut j: size_t = 0;
        j = 0 as libc::c_int as size_t;
        while j < num_to_combine {
            let mut k: size_t = 0;
            HistogramClearCommand(&mut *histograms.offset(j as isize));
            k = 0 as libc::c_int as size_t;
            while k < *block_lengths.offset(i.wrapping_add(j) as isize) as libc::c_ulong
            {
                let fresh48 = pos;
                pos = pos.wrapping_add(1);
                HistogramAddCommand(
                    &mut *histograms.offset(j as isize),
                    *data.offset(fresh48 as isize) as size_t,
                );
                k = k.wrapping_add(1);
            }
            (*histograms.offset(j as isize))
                .bit_cost_ = BrotliPopulationCostCommand(
                &mut *histograms.offset(j as isize),
            );
            new_clusters[j as usize] = j as uint32_t;
            symbols[j as usize] = j as uint32_t;
            sizes[j as usize] = 1 as libc::c_int as uint32_t;
            j = j.wrapping_add(1);
        }
        num_new_clusters = BrotliHistogramCombineCommand(
            histograms,
            sizes.as_mut_ptr(),
            symbols.as_mut_ptr(),
            new_clusters.as_mut_ptr(),
            pairs,
            num_to_combine,
            num_to_combine,
            64 as libc::c_int as size_t,
            max_num_pairs,
        );
        if all_histograms_capacity < all_histograms_size.wrapping_add(num_new_clusters) {
            let mut _new_size = if all_histograms_capacity
                == 0 as libc::c_int as libc::c_ulong
            {
                all_histograms_size.wrapping_add(num_new_clusters)
            } else {
                all_histograms_capacity
            };
            let mut new_array = 0 as *mut HistogramCommand;
            while _new_size < all_histograms_size.wrapping_add(num_new_clusters) {
                _new_size = (_new_size as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
            }
            new_array = if _new_size > 0 as libc::c_int as libc::c_ulong {
                BrotliAllocate(
                    m,
                    _new_size
                        .wrapping_mul(
                            ::std::mem::size_of::<HistogramCommand>() as libc::c_ulong,
                        ),
                ) as *mut HistogramCommand
            } else {
                0 as *mut HistogramCommand
            };
            if 0 as libc::c_int == 0 && 0 as libc::c_int == 0
                && all_histograms_capacity != 0 as libc::c_int as libc::c_ulong
            {
                memcpy(
                    new_array as *mut libc::c_void,
                    all_histograms as *const libc::c_void,
                    all_histograms_capacity
                        .wrapping_mul(
                            ::std::mem::size_of::<HistogramCommand>() as libc::c_ulong,
                        ),
                );
            }
            BrotliFree(m, all_histograms as *mut libc::c_void);
            all_histograms = 0 as *mut HistogramCommand;
            all_histograms = new_array;
            all_histograms_capacity = _new_size;
        }
        if cluster_size_capacity < cluster_size_size.wrapping_add(num_new_clusters) {
            let mut _new_size_0 = if cluster_size_capacity
                == 0 as libc::c_int as libc::c_ulong
            {
                cluster_size_size.wrapping_add(num_new_clusters)
            } else {
                cluster_size_capacity
            };
            let mut new_array_0 = 0 as *mut uint32_t;
            while _new_size_0 < cluster_size_size.wrapping_add(num_new_clusters) {
                _new_size_0 = (_new_size_0 as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
            }
            new_array_0 = if _new_size_0 > 0 as libc::c_int as libc::c_ulong {
                BrotliAllocate(
                    m,
                    _new_size_0
                        .wrapping_mul(::std::mem::size_of::<uint32_t>() as libc::c_ulong),
                ) as *mut uint32_t
            } else {
                0 as *mut uint32_t
            };
            if 0 as libc::c_int == 0 && 0 as libc::c_int == 0
                && cluster_size_capacity != 0 as libc::c_int as libc::c_ulong
            {
                memcpy(
                    new_array_0 as *mut libc::c_void,
                    cluster_size as *const libc::c_void,
                    cluster_size_capacity
                        .wrapping_mul(::std::mem::size_of::<uint32_t>() as libc::c_ulong),
                );
            }
            BrotliFree(m, cluster_size as *mut libc::c_void);
            cluster_size = 0 as *mut uint32_t;
            cluster_size = new_array_0;
            cluster_size_capacity = _new_size_0;
        }
        if 0 as libc::c_int != 0 {
            return;
        }
        j = 0 as libc::c_int as size_t;
        while j < num_new_clusters {
            let fresh49 = all_histograms_size;
            all_histograms_size = all_histograms_size.wrapping_add(1);
            *all_histograms
                .offset(
                    fresh49 as isize,
                ) = *histograms.offset(new_clusters[j as usize] as isize);
            let fresh50 = cluster_size_size;
            cluster_size_size = cluster_size_size.wrapping_add(1);
            *cluster_size
                .offset(fresh50 as isize) = sizes[new_clusters[j as usize] as usize];
            remap[new_clusters[j as usize] as usize] = j as uint32_t;
            j = j.wrapping_add(1);
        }
        j = 0 as libc::c_int as size_t;
        while j < num_to_combine {
            *histogram_symbols
                .offset(
                    i.wrapping_add(j) as isize,
                ) = (num_clusters as uint32_t)
                .wrapping_add(remap[symbols[j as usize] as usize]);
            j = j.wrapping_add(1);
        }
        num_clusters = (num_clusters as libc::c_ulong).wrapping_add(num_new_clusters)
            as size_t as size_t;
        i = (i as libc::c_ulong).wrapping_add(64 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
    }
    BrotliFree(m, histograms as *mut libc::c_void);
    histograms = 0 as *mut HistogramCommand;
    max_num_pairs = brotli_min_size_t(
        (64 as libc::c_int as libc::c_ulong).wrapping_mul(num_clusters),
        num_clusters
            .wrapping_div(2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(num_clusters),
    );
    if pairs_capacity < max_num_pairs.wrapping_add(1 as libc::c_int as libc::c_ulong) {
        BrotliFree(m, pairs as *mut libc::c_void);
        pairs = 0 as *mut HistogramPair;
        pairs = if max_num_pairs.wrapping_add(1 as libc::c_int as libc::c_ulong)
            > 0 as libc::c_int as libc::c_ulong
        {
            BrotliAllocate(
                m,
                max_num_pairs
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<HistogramPair>() as libc::c_ulong,
                    ),
            ) as *mut HistogramPair
        } else {
            0 as *mut HistogramPair
        };
        if 0 as libc::c_int != 0 || 0 as libc::c_int != 0 {
            return;
        }
    }
    clusters = if num_clusters > 0 as libc::c_int as libc::c_ulong {
        BrotliAllocate(
            m,
            num_clusters.wrapping_mul(::std::mem::size_of::<uint32_t>() as libc::c_ulong),
        ) as *mut uint32_t
    } else {
        0 as *mut uint32_t
    };
    if 0 as libc::c_int != 0 || 0 as libc::c_int != 0 {
        return;
    }
    i = 0 as libc::c_int as size_t;
    while i < num_clusters {
        *clusters.offset(i as isize) = i as uint32_t;
        i = i.wrapping_add(1);
    }
    num_final_clusters = BrotliHistogramCombineCommand(
        all_histograms,
        cluster_size,
        histogram_symbols,
        clusters,
        pairs,
        num_clusters,
        num_blocks,
        256 as libc::c_int as size_t,
        max_num_pairs,
    );
    BrotliFree(m, pairs as *mut libc::c_void);
    pairs = 0 as *mut HistogramPair;
    BrotliFree(m, cluster_size as *mut libc::c_void);
    cluster_size = 0 as *mut uint32_t;
    new_index = if num_clusters > 0 as libc::c_int as libc::c_ulong {
        BrotliAllocate(
            m,
            num_clusters.wrapping_mul(::std::mem::size_of::<uint32_t>() as libc::c_ulong),
        ) as *mut uint32_t
    } else {
        0 as *mut uint32_t
    };
    if 0 as libc::c_int != 0 || 0 as libc::c_int != 0 {
        return;
    }
    i = 0 as libc::c_int as size_t;
    while i < num_clusters {
        *new_index.offset(i as isize) = kInvalidIndex;
        i = i.wrapping_add(1);
    }
    pos = 0 as libc::c_int as size_t;
    let mut next_index = 0 as libc::c_int as uint32_t;
    i = 0 as libc::c_int as size_t;
    while i < num_blocks {
        let mut histo = HistogramCommand {
            data_: [0; 704],
            total_count_: 0,
            bit_cost_: 0.,
        };
        let mut j_0: size_t = 0;
        let mut best_out: uint32_t = 0;
        let mut best_bits: libc::c_double = 0.;
        HistogramClearCommand(&mut histo);
        j_0 = 0 as libc::c_int as size_t;
        while j_0 < *block_lengths.offset(i as isize) as libc::c_ulong {
            let fresh51 = pos;
            pos = pos.wrapping_add(1);
            HistogramAddCommand(&mut histo, *data.offset(fresh51 as isize) as size_t);
            j_0 = j_0.wrapping_add(1);
        }
        best_out = if i == 0 as libc::c_int as libc::c_ulong {
            *histogram_symbols.offset(0 as libc::c_int as isize)
        } else {
            *histogram_symbols
                .offset(i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
        };
        best_bits = BrotliHistogramBitCostDistanceCommand(
            &mut histo,
            &mut *all_histograms.offset(best_out as isize),
        );
        j_0 = 0 as libc::c_int as size_t;
        while j_0 < num_final_clusters {
            let cur_bits = BrotliHistogramBitCostDistanceCommand(
                &mut histo,
                &mut *all_histograms.offset(*clusters.offset(j_0 as isize) as isize),
            );
            if cur_bits < best_bits {
                best_bits = cur_bits;
                best_out = *clusters.offset(j_0 as isize);
            }
            j_0 = j_0.wrapping_add(1);
        }
        *histogram_symbols.offset(i as isize) = best_out;
        if *new_index.offset(best_out as isize) == kInvalidIndex {
            let fresh52 = next_index;
            next_index = next_index.wrapping_add(1);
            *new_index.offset(best_out as isize) = fresh52;
        }
        i = i.wrapping_add(1);
    }
    BrotliFree(m, clusters as *mut libc::c_void);
    clusters = 0 as *mut uint32_t;
    BrotliFree(m, all_histograms as *mut libc::c_void);
    all_histograms = 0 as *mut HistogramCommand;
    if (*split).types_alloc_size < num_blocks {
        let mut _new_size_1 = if (*split).types_alloc_size
            == 0 as libc::c_int as libc::c_ulong
        {
            num_blocks
        } else {
            (*split).types_alloc_size
        };
        let mut new_array_1 = 0 as *mut uint8_t;
        while _new_size_1 < num_blocks {
            _new_size_1 = (_new_size_1 as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
        }
        new_array_1 = if _new_size_1 > 0 as libc::c_int as libc::c_ulong {
            BrotliAllocate(
                m,
                _new_size_1
                    .wrapping_mul(::std::mem::size_of::<uint8_t>() as libc::c_ulong),
            ) as *mut uint8_t
        } else {
            0 as *mut uint8_t
        };
        if 0 as libc::c_int == 0 && 0 as libc::c_int == 0
            && (*split).types_alloc_size != 0 as libc::c_int as libc::c_ulong
        {
            memcpy(
                new_array_1 as *mut libc::c_void,
                (*split).types as *const libc::c_void,
                ((*split).types_alloc_size)
                    .wrapping_mul(::std::mem::size_of::<uint8_t>() as libc::c_ulong),
            );
        }
        BrotliFree(m, (*split).types as *mut libc::c_void);
        let ref mut fresh53 = (*split).types;
        *fresh53 = 0 as *mut uint8_t;
        let ref mut fresh54 = (*split).types;
        *fresh54 = new_array_1;
        (*split).types_alloc_size = _new_size_1;
    }
    if (*split).lengths_alloc_size < num_blocks {
        let mut _new_size_2 = if (*split).lengths_alloc_size
            == 0 as libc::c_int as libc::c_ulong
        {
            num_blocks
        } else {
            (*split).lengths_alloc_size
        };
        let mut new_array_2 = 0 as *mut uint32_t;
        while _new_size_2 < num_blocks {
            _new_size_2 = (_new_size_2 as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
        }
        new_array_2 = if _new_size_2 > 0 as libc::c_int as libc::c_ulong {
            BrotliAllocate(
                m,
                _new_size_2
                    .wrapping_mul(::std::mem::size_of::<uint32_t>() as libc::c_ulong),
            ) as *mut uint32_t
        } else {
            0 as *mut uint32_t
        };
        if 0 as libc::c_int == 0 && 0 as libc::c_int == 0
            && (*split).lengths_alloc_size != 0 as libc::c_int as libc::c_ulong
        {
            memcpy(
                new_array_2 as *mut libc::c_void,
                (*split).lengths as *const libc::c_void,
                ((*split).lengths_alloc_size)
                    .wrapping_mul(::std::mem::size_of::<uint32_t>() as libc::c_ulong),
            );
        }
        BrotliFree(m, (*split).lengths as *mut libc::c_void);
        let ref mut fresh55 = (*split).lengths;
        *fresh55 = 0 as *mut uint32_t;
        let ref mut fresh56 = (*split).lengths;
        *fresh56 = new_array_2;
        (*split).lengths_alloc_size = _new_size_2;
    }
    if 0 as libc::c_int != 0 {
        return;
    }
    let mut cur_length = 0 as libc::c_int as uint32_t;
    let mut block_idx_0 = 0 as libc::c_int as size_t;
    let mut max_type = 0 as libc::c_int as uint8_t;
    i = 0 as libc::c_int as size_t;
    while i < num_blocks {
        cur_length = (cur_length as libc::c_uint)
            .wrapping_add(*block_lengths.offset(i as isize)) as uint32_t as uint32_t;
        if i.wrapping_add(1 as libc::c_int as libc::c_ulong) == num_blocks
            || *histogram_symbols.offset(i as isize)
                != *histogram_symbols
                    .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
        {
            let id = *new_index.offset(*histogram_symbols.offset(i as isize) as isize)
                as uint8_t;
            *((*split).types).offset(block_idx_0 as isize) = id;
            *((*split).lengths).offset(block_idx_0 as isize) = cur_length;
            max_type = brotli_max_uint8_t(max_type, id);
            cur_length = 0 as libc::c_int as uint32_t;
            block_idx_0 = block_idx_0.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    (*split).num_blocks = block_idx_0;
    (*split)
        .num_types = (max_type as size_t)
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    BrotliFree(m, new_index as *mut libc::c_void);
    new_index = 0 as *mut uint32_t;
    BrotliFree(m, block_lengths as *mut libc::c_void);
    block_lengths = 0 as *mut uint32_t;
    BrotliFree(m, histogram_symbols as *mut libc::c_void);
    histogram_symbols = 0 as *mut uint32_t;
}
unsafe extern "C" fn SplitByteVectorLiteral(
    mut m: *mut MemoryManager,
    mut data: *const uint8_t,
    length: size_t,
    literals_per_histogram: size_t,
    max_histograms: size_t,
    sampling_stride_length: size_t,
    block_switch_cost: libc::c_double,
    mut params: *const BrotliEncoderParams,
    mut split: *mut BlockSplit,
) {
    let data_size = HistogramDataSizeLiteral();
    let mut num_histograms = length
        .wrapping_div(literals_per_histogram)
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    let mut histograms = 0 as *mut HistogramLiteral;
    if num_histograms > max_histograms {
        num_histograms = max_histograms;
    }
    if length == 0 as libc::c_int as libc::c_ulong {
        (*split).num_types = 1 as libc::c_int as size_t;
        return;
    } else {
        if length < kMinLengthForBlockSplitting {
            if (*split).types_alloc_size
                < ((*split).num_blocks).wrapping_add(1 as libc::c_int as libc::c_ulong)
            {
                let mut _new_size = if (*split).types_alloc_size
                    == 0 as libc::c_int as libc::c_ulong
                {
                    ((*split).num_blocks).wrapping_add(1 as libc::c_int as libc::c_ulong)
                } else {
                    (*split).types_alloc_size
                };
                let mut new_array = 0 as *mut uint8_t;
                while _new_size
                    < ((*split).num_blocks)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                {
                    _new_size = (_new_size as libc::c_ulong)
                        .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t
                        as size_t;
                }
                new_array = if _new_size > 0 as libc::c_int as libc::c_ulong {
                    BrotliAllocate(
                        m,
                        _new_size
                            .wrapping_mul(
                                ::std::mem::size_of::<uint8_t>() as libc::c_ulong,
                            ),
                    ) as *mut uint8_t
                } else {
                    0 as *mut uint8_t
                };
                if 0 as libc::c_int == 0 && 0 as libc::c_int == 0
                    && (*split).types_alloc_size != 0 as libc::c_int as libc::c_ulong
                {
                    memcpy(
                        new_array as *mut libc::c_void,
                        (*split).types as *const libc::c_void,
                        ((*split).types_alloc_size)
                            .wrapping_mul(
                                ::std::mem::size_of::<uint8_t>() as libc::c_ulong,
                            ),
                    );
                }
                BrotliFree(m, (*split).types as *mut libc::c_void);
                let ref mut fresh57 = (*split).types;
                *fresh57 = 0 as *mut uint8_t;
                let ref mut fresh58 = (*split).types;
                *fresh58 = new_array;
                (*split).types_alloc_size = _new_size;
            }
            if (*split).lengths_alloc_size
                < ((*split).num_blocks).wrapping_add(1 as libc::c_int as libc::c_ulong)
            {
                let mut _new_size_0 = if (*split).lengths_alloc_size
                    == 0 as libc::c_int as libc::c_ulong
                {
                    ((*split).num_blocks).wrapping_add(1 as libc::c_int as libc::c_ulong)
                } else {
                    (*split).lengths_alloc_size
                };
                let mut new_array_0 = 0 as *mut uint32_t;
                while _new_size_0
                    < ((*split).num_blocks)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                {
                    _new_size_0 = (_new_size_0 as libc::c_ulong)
                        .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t
                        as size_t;
                }
                new_array_0 = if _new_size_0 > 0 as libc::c_int as libc::c_ulong {
                    BrotliAllocate(
                        m,
                        _new_size_0
                            .wrapping_mul(
                                ::std::mem::size_of::<uint32_t>() as libc::c_ulong,
                            ),
                    ) as *mut uint32_t
                } else {
                    0 as *mut uint32_t
                };
                if 0 as libc::c_int == 0 && 0 as libc::c_int == 0
                    && (*split).lengths_alloc_size != 0 as libc::c_int as libc::c_ulong
                {
                    memcpy(
                        new_array_0 as *mut libc::c_void,
                        (*split).lengths as *const libc::c_void,
                        ((*split).lengths_alloc_size)
                            .wrapping_mul(
                                ::std::mem::size_of::<uint32_t>() as libc::c_ulong,
                            ),
                    );
                }
                BrotliFree(m, (*split).lengths as *mut libc::c_void);
                let ref mut fresh59 = (*split).lengths;
                *fresh59 = 0 as *mut uint32_t;
                let ref mut fresh60 = (*split).lengths;
                *fresh60 = new_array_0;
                (*split).lengths_alloc_size = _new_size_0;
            }
            if 0 as libc::c_int != 0 {
                return;
            }
            (*split).num_types = 1 as libc::c_int as size_t;
            *((*split).types)
                .offset((*split).num_blocks as isize) = 0 as libc::c_int as uint8_t;
            *((*split).lengths)
                .offset((*split).num_blocks as isize) = length as uint32_t;
            let ref mut fresh61 = (*split).num_blocks;
            *fresh61 = (*fresh61).wrapping_add(1);
            return;
        }
    }
    histograms = if num_histograms > 0 as libc::c_int as libc::c_ulong {
        BrotliAllocate(
            m,
            num_histograms
                .wrapping_mul(::std::mem::size_of::<HistogramLiteral>() as libc::c_ulong),
        ) as *mut HistogramLiteral
    } else {
        0 as *mut HistogramLiteral
    };
    if 0 as libc::c_int != 0 || 0 as libc::c_int != 0 {
        return;
    }
    InitialEntropyCodesLiteral(
        data,
        length,
        sampling_stride_length,
        num_histograms,
        histograms,
    );
    RefineEntropyCodesLiteral(
        data,
        length,
        sampling_stride_length,
        num_histograms,
        histograms,
    );
    let mut block_ids = if length > 0 as libc::c_int as libc::c_ulong {
        BrotliAllocate(
            m,
            length.wrapping_mul(::std::mem::size_of::<uint8_t>() as libc::c_ulong),
        ) as *mut uint8_t
    } else {
        0 as *mut uint8_t
    };
    let mut num_blocks = 0 as libc::c_int as size_t;
    let bitmaplen = num_histograms.wrapping_add(7 as libc::c_int as libc::c_ulong)
        >> 3 as libc::c_int;
    let mut insert_cost = if data_size.wrapping_mul(num_histograms)
        > 0 as libc::c_int as libc::c_ulong
    {
        BrotliAllocate(
            m,
            data_size
                .wrapping_mul(num_histograms)
                .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
        ) as *mut libc::c_double
    } else {
        0 as *mut libc::c_double
    };
    let mut cost = if num_histograms > 0 as libc::c_int as libc::c_ulong {
        BrotliAllocate(
            m,
            num_histograms
                .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
        ) as *mut libc::c_double
    } else {
        0 as *mut libc::c_double
    };
    let mut switch_signal = if length.wrapping_mul(bitmaplen)
        > 0 as libc::c_int as libc::c_ulong
    {
        BrotliAllocate(
            m,
            length
                .wrapping_mul(bitmaplen)
                .wrapping_mul(::std::mem::size_of::<uint8_t>() as libc::c_ulong),
        ) as *mut uint8_t
    } else {
        0 as *mut uint8_t
    };
    let mut new_id = if num_histograms > 0 as libc::c_int as libc::c_ulong {
        BrotliAllocate(
            m,
            num_histograms
                .wrapping_mul(::std::mem::size_of::<uint16_t>() as libc::c_ulong),
        ) as *mut uint16_t
    } else {
        0 as *mut uint16_t
    };
    let iters = (if (*params).quality < 11 as libc::c_int {
        3 as libc::c_int
    } else {
        10 as libc::c_int
    }) as size_t;
    let mut i: size_t = 0;
    if 0 as libc::c_int != 0 || 0 as libc::c_int != 0 || 0 as libc::c_int != 0
        || 0 as libc::c_int != 0 || 0 as libc::c_int != 0 || 0 as libc::c_int != 0
    {
        return;
    }
    i = 0 as libc::c_int as size_t;
    while i < iters {
        num_blocks = FindBlocksLiteral(
            data,
            length,
            block_switch_cost,
            num_histograms,
            histograms,
            insert_cost,
            cost,
            switch_signal,
            block_ids,
        );
        num_histograms = RemapBlockIdsLiteral(block_ids, length, new_id, num_histograms);
        BuildBlockHistogramsLiteral(data, length, block_ids, num_histograms, histograms);
        i = i.wrapping_add(1);
    }
    BrotliFree(m, insert_cost as *mut libc::c_void);
    insert_cost = 0 as *mut libc::c_double;
    BrotliFree(m, cost as *mut libc::c_void);
    cost = 0 as *mut libc::c_double;
    BrotliFree(m, switch_signal as *mut libc::c_void);
    switch_signal = 0 as *mut uint8_t;
    BrotliFree(m, new_id as *mut libc::c_void);
    new_id = 0 as *mut uint16_t;
    BrotliFree(m, histograms as *mut libc::c_void);
    histograms = 0 as *mut HistogramLiteral;
    ClusterBlocksLiteral(m, data, length, num_blocks, block_ids, split);
    if 0 as libc::c_int != 0 {
        return;
    }
    BrotliFree(m, block_ids as *mut libc::c_void);
    block_ids = 0 as *mut uint8_t;
}
unsafe extern "C" fn SplitByteVectorDistance(
    mut m: *mut MemoryManager,
    mut data: *const uint16_t,
    length: size_t,
    literals_per_histogram: size_t,
    max_histograms: size_t,
    sampling_stride_length: size_t,
    block_switch_cost: libc::c_double,
    mut params: *const BrotliEncoderParams,
    mut split: *mut BlockSplit,
) {
    let data_size = HistogramDataSizeDistance();
    let mut num_histograms = length
        .wrapping_div(literals_per_histogram)
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    let mut histograms = 0 as *mut HistogramDistance;
    if num_histograms > max_histograms {
        num_histograms = max_histograms;
    }
    if length == 0 as libc::c_int as libc::c_ulong {
        (*split).num_types = 1 as libc::c_int as size_t;
        return;
    } else {
        if length < kMinLengthForBlockSplitting {
            if (*split).types_alloc_size
                < ((*split).num_blocks).wrapping_add(1 as libc::c_int as libc::c_ulong)
            {
                let mut _new_size = if (*split).types_alloc_size
                    == 0 as libc::c_int as libc::c_ulong
                {
                    ((*split).num_blocks).wrapping_add(1 as libc::c_int as libc::c_ulong)
                } else {
                    (*split).types_alloc_size
                };
                let mut new_array = 0 as *mut uint8_t;
                while _new_size
                    < ((*split).num_blocks)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                {
                    _new_size = (_new_size as libc::c_ulong)
                        .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t
                        as size_t;
                }
                new_array = if _new_size > 0 as libc::c_int as libc::c_ulong {
                    BrotliAllocate(
                        m,
                        _new_size
                            .wrapping_mul(
                                ::std::mem::size_of::<uint8_t>() as libc::c_ulong,
                            ),
                    ) as *mut uint8_t
                } else {
                    0 as *mut uint8_t
                };
                if 0 as libc::c_int == 0 && 0 as libc::c_int == 0
                    && (*split).types_alloc_size != 0 as libc::c_int as libc::c_ulong
                {
                    memcpy(
                        new_array as *mut libc::c_void,
                        (*split).types as *const libc::c_void,
                        ((*split).types_alloc_size)
                            .wrapping_mul(
                                ::std::mem::size_of::<uint8_t>() as libc::c_ulong,
                            ),
                    );
                }
                BrotliFree(m, (*split).types as *mut libc::c_void);
                let ref mut fresh62 = (*split).types;
                *fresh62 = 0 as *mut uint8_t;
                let ref mut fresh63 = (*split).types;
                *fresh63 = new_array;
                (*split).types_alloc_size = _new_size;
            }
            if (*split).lengths_alloc_size
                < ((*split).num_blocks).wrapping_add(1 as libc::c_int as libc::c_ulong)
            {
                let mut _new_size_0 = if (*split).lengths_alloc_size
                    == 0 as libc::c_int as libc::c_ulong
                {
                    ((*split).num_blocks).wrapping_add(1 as libc::c_int as libc::c_ulong)
                } else {
                    (*split).lengths_alloc_size
                };
                let mut new_array_0 = 0 as *mut uint32_t;
                while _new_size_0
                    < ((*split).num_blocks)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                {
                    _new_size_0 = (_new_size_0 as libc::c_ulong)
                        .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t
                        as size_t;
                }
                new_array_0 = if _new_size_0 > 0 as libc::c_int as libc::c_ulong {
                    BrotliAllocate(
                        m,
                        _new_size_0
                            .wrapping_mul(
                                ::std::mem::size_of::<uint32_t>() as libc::c_ulong,
                            ),
                    ) as *mut uint32_t
                } else {
                    0 as *mut uint32_t
                };
                if 0 as libc::c_int == 0 && 0 as libc::c_int == 0
                    && (*split).lengths_alloc_size != 0 as libc::c_int as libc::c_ulong
                {
                    memcpy(
                        new_array_0 as *mut libc::c_void,
                        (*split).lengths as *const libc::c_void,
                        ((*split).lengths_alloc_size)
                            .wrapping_mul(
                                ::std::mem::size_of::<uint32_t>() as libc::c_ulong,
                            ),
                    );
                }
                BrotliFree(m, (*split).lengths as *mut libc::c_void);
                let ref mut fresh64 = (*split).lengths;
                *fresh64 = 0 as *mut uint32_t;
                let ref mut fresh65 = (*split).lengths;
                *fresh65 = new_array_0;
                (*split).lengths_alloc_size = _new_size_0;
            }
            if 0 as libc::c_int != 0 {
                return;
            }
            (*split).num_types = 1 as libc::c_int as size_t;
            *((*split).types)
                .offset((*split).num_blocks as isize) = 0 as libc::c_int as uint8_t;
            *((*split).lengths)
                .offset((*split).num_blocks as isize) = length as uint32_t;
            let ref mut fresh66 = (*split).num_blocks;
            *fresh66 = (*fresh66).wrapping_add(1);
            return;
        }
    }
    histograms = if num_histograms > 0 as libc::c_int as libc::c_ulong {
        BrotliAllocate(
            m,
            num_histograms
                .wrapping_mul(
                    ::std::mem::size_of::<HistogramDistance>() as libc::c_ulong,
                ),
        ) as *mut HistogramDistance
    } else {
        0 as *mut HistogramDistance
    };
    if 0 as libc::c_int != 0 || 0 as libc::c_int != 0 {
        return;
    }
    InitialEntropyCodesDistance(
        data,
        length,
        sampling_stride_length,
        num_histograms,
        histograms,
    );
    RefineEntropyCodesDistance(
        data,
        length,
        sampling_stride_length,
        num_histograms,
        histograms,
    );
    let mut block_ids = if length > 0 as libc::c_int as libc::c_ulong {
        BrotliAllocate(
            m,
            length.wrapping_mul(::std::mem::size_of::<uint8_t>() as libc::c_ulong),
        ) as *mut uint8_t
    } else {
        0 as *mut uint8_t
    };
    let mut num_blocks = 0 as libc::c_int as size_t;
    let bitmaplen = num_histograms.wrapping_add(7 as libc::c_int as libc::c_ulong)
        >> 3 as libc::c_int;
    let mut insert_cost = if data_size.wrapping_mul(num_histograms)
        > 0 as libc::c_int as libc::c_ulong
    {
        BrotliAllocate(
            m,
            data_size
                .wrapping_mul(num_histograms)
                .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
        ) as *mut libc::c_double
    } else {
        0 as *mut libc::c_double
    };
    let mut cost = if num_histograms > 0 as libc::c_int as libc::c_ulong {
        BrotliAllocate(
            m,
            num_histograms
                .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
        ) as *mut libc::c_double
    } else {
        0 as *mut libc::c_double
    };
    let mut switch_signal = if length.wrapping_mul(bitmaplen)
        > 0 as libc::c_int as libc::c_ulong
    {
        BrotliAllocate(
            m,
            length
                .wrapping_mul(bitmaplen)
                .wrapping_mul(::std::mem::size_of::<uint8_t>() as libc::c_ulong),
        ) as *mut uint8_t
    } else {
        0 as *mut uint8_t
    };
    let mut new_id = if num_histograms > 0 as libc::c_int as libc::c_ulong {
        BrotliAllocate(
            m,
            num_histograms
                .wrapping_mul(::std::mem::size_of::<uint16_t>() as libc::c_ulong),
        ) as *mut uint16_t
    } else {
        0 as *mut uint16_t
    };
    let iters = (if (*params).quality < 11 as libc::c_int {
        3 as libc::c_int
    } else {
        10 as libc::c_int
    }) as size_t;
    let mut i: size_t = 0;
    if 0 as libc::c_int != 0 || 0 as libc::c_int != 0 || 0 as libc::c_int != 0
        || 0 as libc::c_int != 0 || 0 as libc::c_int != 0 || 0 as libc::c_int != 0
    {
        return;
    }
    i = 0 as libc::c_int as size_t;
    while i < iters {
        num_blocks = FindBlocksDistance(
            data,
            length,
            block_switch_cost,
            num_histograms,
            histograms,
            insert_cost,
            cost,
            switch_signal,
            block_ids,
        );
        num_histograms = RemapBlockIdsDistance(
            block_ids,
            length,
            new_id,
            num_histograms,
        );
        BuildBlockHistogramsDistance(
            data,
            length,
            block_ids,
            num_histograms,
            histograms,
        );
        i = i.wrapping_add(1);
    }
    BrotliFree(m, insert_cost as *mut libc::c_void);
    insert_cost = 0 as *mut libc::c_double;
    BrotliFree(m, cost as *mut libc::c_void);
    cost = 0 as *mut libc::c_double;
    BrotliFree(m, switch_signal as *mut libc::c_void);
    switch_signal = 0 as *mut uint8_t;
    BrotliFree(m, new_id as *mut libc::c_void);
    new_id = 0 as *mut uint16_t;
    BrotliFree(m, histograms as *mut libc::c_void);
    histograms = 0 as *mut HistogramDistance;
    ClusterBlocksDistance(m, data, length, num_blocks, block_ids, split);
    if 0 as libc::c_int != 0 {
        return;
    }
    BrotliFree(m, block_ids as *mut libc::c_void);
    block_ids = 0 as *mut uint8_t;
}
unsafe extern "C" fn SplitByteVectorCommand(
    mut m: *mut MemoryManager,
    mut data: *const uint16_t,
    length: size_t,
    literals_per_histogram: size_t,
    max_histograms: size_t,
    sampling_stride_length: size_t,
    block_switch_cost: libc::c_double,
    mut params: *const BrotliEncoderParams,
    mut split: *mut BlockSplit,
) {
    let data_size = HistogramDataSizeCommand();
    let mut num_histograms = length
        .wrapping_div(literals_per_histogram)
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    let mut histograms = 0 as *mut HistogramCommand;
    if num_histograms > max_histograms {
        num_histograms = max_histograms;
    }
    if length == 0 as libc::c_int as libc::c_ulong {
        (*split).num_types = 1 as libc::c_int as size_t;
        return;
    } else {
        if length < kMinLengthForBlockSplitting {
            if (*split).types_alloc_size
                < ((*split).num_blocks).wrapping_add(1 as libc::c_int as libc::c_ulong)
            {
                let mut _new_size = if (*split).types_alloc_size
                    == 0 as libc::c_int as libc::c_ulong
                {
                    ((*split).num_blocks).wrapping_add(1 as libc::c_int as libc::c_ulong)
                } else {
                    (*split).types_alloc_size
                };
                let mut new_array = 0 as *mut uint8_t;
                while _new_size
                    < ((*split).num_blocks)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                {
                    _new_size = (_new_size as libc::c_ulong)
                        .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t
                        as size_t;
                }
                new_array = if _new_size > 0 as libc::c_int as libc::c_ulong {
                    BrotliAllocate(
                        m,
                        _new_size
                            .wrapping_mul(
                                ::std::mem::size_of::<uint8_t>() as libc::c_ulong,
                            ),
                    ) as *mut uint8_t
                } else {
                    0 as *mut uint8_t
                };
                if 0 as libc::c_int == 0 && 0 as libc::c_int == 0
                    && (*split).types_alloc_size != 0 as libc::c_int as libc::c_ulong
                {
                    memcpy(
                        new_array as *mut libc::c_void,
                        (*split).types as *const libc::c_void,
                        ((*split).types_alloc_size)
                            .wrapping_mul(
                                ::std::mem::size_of::<uint8_t>() as libc::c_ulong,
                            ),
                    );
                }
                BrotliFree(m, (*split).types as *mut libc::c_void);
                let ref mut fresh67 = (*split).types;
                *fresh67 = 0 as *mut uint8_t;
                let ref mut fresh68 = (*split).types;
                *fresh68 = new_array;
                (*split).types_alloc_size = _new_size;
            }
            if (*split).lengths_alloc_size
                < ((*split).num_blocks).wrapping_add(1 as libc::c_int as libc::c_ulong)
            {
                let mut _new_size_0 = if (*split).lengths_alloc_size
                    == 0 as libc::c_int as libc::c_ulong
                {
                    ((*split).num_blocks).wrapping_add(1 as libc::c_int as libc::c_ulong)
                } else {
                    (*split).lengths_alloc_size
                };
                let mut new_array_0 = 0 as *mut uint32_t;
                while _new_size_0
                    < ((*split).num_blocks)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                {
                    _new_size_0 = (_new_size_0 as libc::c_ulong)
                        .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t
                        as size_t;
                }
                new_array_0 = if _new_size_0 > 0 as libc::c_int as libc::c_ulong {
                    BrotliAllocate(
                        m,
                        _new_size_0
                            .wrapping_mul(
                                ::std::mem::size_of::<uint32_t>() as libc::c_ulong,
                            ),
                    ) as *mut uint32_t
                } else {
                    0 as *mut uint32_t
                };
                if 0 as libc::c_int == 0 && 0 as libc::c_int == 0
                    && (*split).lengths_alloc_size != 0 as libc::c_int as libc::c_ulong
                {
                    memcpy(
                        new_array_0 as *mut libc::c_void,
                        (*split).lengths as *const libc::c_void,
                        ((*split).lengths_alloc_size)
                            .wrapping_mul(
                                ::std::mem::size_of::<uint32_t>() as libc::c_ulong,
                            ),
                    );
                }
                BrotliFree(m, (*split).lengths as *mut libc::c_void);
                let ref mut fresh69 = (*split).lengths;
                *fresh69 = 0 as *mut uint32_t;
                let ref mut fresh70 = (*split).lengths;
                *fresh70 = new_array_0;
                (*split).lengths_alloc_size = _new_size_0;
            }
            if 0 as libc::c_int != 0 {
                return;
            }
            (*split).num_types = 1 as libc::c_int as size_t;
            *((*split).types)
                .offset((*split).num_blocks as isize) = 0 as libc::c_int as uint8_t;
            *((*split).lengths)
                .offset((*split).num_blocks as isize) = length as uint32_t;
            let ref mut fresh71 = (*split).num_blocks;
            *fresh71 = (*fresh71).wrapping_add(1);
            return;
        }
    }
    histograms = if num_histograms > 0 as libc::c_int as libc::c_ulong {
        BrotliAllocate(
            m,
            num_histograms
                .wrapping_mul(::std::mem::size_of::<HistogramCommand>() as libc::c_ulong),
        ) as *mut HistogramCommand
    } else {
        0 as *mut HistogramCommand
    };
    if 0 as libc::c_int != 0 || 0 as libc::c_int != 0 {
        return;
    }
    InitialEntropyCodesCommand(
        data,
        length,
        sampling_stride_length,
        num_histograms,
        histograms,
    );
    RefineEntropyCodesCommand(
        data,
        length,
        sampling_stride_length,
        num_histograms,
        histograms,
    );
    let mut block_ids = if length > 0 as libc::c_int as libc::c_ulong {
        BrotliAllocate(
            m,
            length.wrapping_mul(::std::mem::size_of::<uint8_t>() as libc::c_ulong),
        ) as *mut uint8_t
    } else {
        0 as *mut uint8_t
    };
    let mut num_blocks = 0 as libc::c_int as size_t;
    let bitmaplen = num_histograms.wrapping_add(7 as libc::c_int as libc::c_ulong)
        >> 3 as libc::c_int;
    let mut insert_cost = if data_size.wrapping_mul(num_histograms)
        > 0 as libc::c_int as libc::c_ulong
    {
        BrotliAllocate(
            m,
            data_size
                .wrapping_mul(num_histograms)
                .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
        ) as *mut libc::c_double
    } else {
        0 as *mut libc::c_double
    };
    let mut cost = if num_histograms > 0 as libc::c_int as libc::c_ulong {
        BrotliAllocate(
            m,
            num_histograms
                .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
        ) as *mut libc::c_double
    } else {
        0 as *mut libc::c_double
    };
    let mut switch_signal = if length.wrapping_mul(bitmaplen)
        > 0 as libc::c_int as libc::c_ulong
    {
        BrotliAllocate(
            m,
            length
                .wrapping_mul(bitmaplen)
                .wrapping_mul(::std::mem::size_of::<uint8_t>() as libc::c_ulong),
        ) as *mut uint8_t
    } else {
        0 as *mut uint8_t
    };
    let mut new_id = if num_histograms > 0 as libc::c_int as libc::c_ulong {
        BrotliAllocate(
            m,
            num_histograms
                .wrapping_mul(::std::mem::size_of::<uint16_t>() as libc::c_ulong),
        ) as *mut uint16_t
    } else {
        0 as *mut uint16_t
    };
    let iters = (if (*params).quality < 11 as libc::c_int {
        3 as libc::c_int
    } else {
        10 as libc::c_int
    }) as size_t;
    let mut i: size_t = 0;
    if 0 as libc::c_int != 0 || 0 as libc::c_int != 0 || 0 as libc::c_int != 0
        || 0 as libc::c_int != 0 || 0 as libc::c_int != 0 || 0 as libc::c_int != 0
    {
        return;
    }
    i = 0 as libc::c_int as size_t;
    while i < iters {
        num_blocks = FindBlocksCommand(
            data,
            length,
            block_switch_cost,
            num_histograms,
            histograms,
            insert_cost,
            cost,
            switch_signal,
            block_ids,
        );
        num_histograms = RemapBlockIdsCommand(block_ids, length, new_id, num_histograms);
        BuildBlockHistogramsCommand(data, length, block_ids, num_histograms, histograms);
        i = i.wrapping_add(1);
    }
    BrotliFree(m, insert_cost as *mut libc::c_void);
    insert_cost = 0 as *mut libc::c_double;
    BrotliFree(m, cost as *mut libc::c_void);
    cost = 0 as *mut libc::c_double;
    BrotliFree(m, switch_signal as *mut libc::c_void);
    switch_signal = 0 as *mut uint8_t;
    BrotliFree(m, new_id as *mut libc::c_void);
    new_id = 0 as *mut uint16_t;
    BrotliFree(m, histograms as *mut libc::c_void);
    histograms = 0 as *mut HistogramCommand;
    ClusterBlocksCommand(m, data, length, num_blocks, block_ids, split);
    if 0 as libc::c_int != 0 {
        return;
    }
    BrotliFree(m, block_ids as *mut libc::c_void);
    block_ids = 0 as *mut uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn BrotliInitBlockSplit(mut self_0: *mut BlockSplit) {
    (*self_0).num_types = 0 as libc::c_int as size_t;
    (*self_0).num_blocks = 0 as libc::c_int as size_t;
    let ref mut fresh72 = (*self_0).types;
    *fresh72 = 0 as *mut uint8_t;
    let ref mut fresh73 = (*self_0).lengths;
    *fresh73 = 0 as *mut uint32_t;
    (*self_0).types_alloc_size = 0 as libc::c_int as size_t;
    (*self_0).lengths_alloc_size = 0 as libc::c_int as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn BrotliDestroyBlockSplit(
    mut m: *mut MemoryManager,
    mut self_0: *mut BlockSplit,
) {
    BrotliFree(m, (*self_0).types as *mut libc::c_void);
    let ref mut fresh74 = (*self_0).types;
    *fresh74 = 0 as *mut uint8_t;
    BrotliFree(m, (*self_0).lengths as *mut libc::c_void);
    let ref mut fresh75 = (*self_0).lengths;
    *fresh75 = 0 as *mut uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn BrotliSplitBlock(
    mut m: *mut MemoryManager,
    mut cmds: *const Command,
    num_commands: size_t,
    mut data: *const uint8_t,
    pos: size_t,
    mask: size_t,
    mut params: *const BrotliEncoderParams,
    mut literal_split: *mut BlockSplit,
    mut insert_and_copy_split: *mut BlockSplit,
    mut dist_split: *mut BlockSplit,
) {
    let mut literals_count = CountLiterals(cmds, num_commands);
    let mut literals = if literals_count > 0 as libc::c_int as libc::c_ulong {
        BrotliAllocate(
            m,
            literals_count
                .wrapping_mul(::std::mem::size_of::<uint8_t>() as libc::c_ulong),
        ) as *mut uint8_t
    } else {
        0 as *mut uint8_t
    };
    if 0 as libc::c_int != 0 || 0 as libc::c_int != 0 {
        return;
    }
    CopyLiteralsToByteArray(cmds, num_commands, data, pos, mask, literals);
    SplitByteVectorLiteral(
        m,
        literals,
        literals_count,
        kSymbolsPerLiteralHistogram,
        kMaxLiteralHistograms,
        kLiteralStrideLength,
        kLiteralBlockSwitchCost,
        params,
        literal_split,
    );
    if 0 as libc::c_int != 0 {
        return;
    }
    BrotliFree(m, literals as *mut libc::c_void);
    literals = 0 as *mut uint8_t;
    let mut insert_and_copy_codes = if num_commands > 0 as libc::c_int as libc::c_ulong {
        BrotliAllocate(
            m,
            num_commands.wrapping_mul(::std::mem::size_of::<uint16_t>() as libc::c_ulong),
        ) as *mut uint16_t
    } else {
        0 as *mut uint16_t
    };
    let mut i: size_t = 0;
    if 0 as libc::c_int != 0 || 0 as libc::c_int != 0 {
        return;
    }
    i = 0 as libc::c_int as size_t;
    while i < num_commands {
        *insert_and_copy_codes
            .offset(i as isize) = (*cmds.offset(i as isize)).cmd_prefix_;
        i = i.wrapping_add(1);
    }
    SplitByteVectorCommand(
        m,
        insert_and_copy_codes,
        num_commands,
        kSymbolsPerCommandHistogram,
        kMaxCommandHistograms,
        kCommandStrideLength,
        kCommandBlockSwitchCost,
        params,
        insert_and_copy_split,
    );
    if 0 as libc::c_int != 0 {
        return;
    }
    BrotliFree(m, insert_and_copy_codes as *mut libc::c_void);
    insert_and_copy_codes = 0 as *mut uint16_t;
    let mut distance_prefixes = if num_commands > 0 as libc::c_int as libc::c_ulong {
        BrotliAllocate(
            m,
            num_commands.wrapping_mul(::std::mem::size_of::<uint16_t>() as libc::c_ulong),
        ) as *mut uint16_t
    } else {
        0 as *mut uint16_t
    };
    let mut j = 0 as libc::c_int as size_t;
    let mut i_0: size_t = 0;
    if 0 as libc::c_int != 0 || 0 as libc::c_int != 0 {
        return;
    }
    i_0 = 0 as libc::c_int as size_t;
    while i_0 < num_commands {
        let mut cmd: *const Command = &*cmds.offset(i_0 as isize) as *const Command;
        if CommandCopyLen(cmd) != 0
            && (*cmd).cmd_prefix_ as libc::c_int >= 128 as libc::c_int
        {
            let fresh76 = j;
            j = j.wrapping_add(1);
            *distance_prefixes
                .offset(
                    fresh76 as isize,
                ) = ((*cmd).dist_prefix_ as libc::c_int & 0x3ff as libc::c_int)
                as uint16_t;
        }
        i_0 = i_0.wrapping_add(1);
    }
    SplitByteVectorDistance(
        m,
        distance_prefixes,
        j,
        kSymbolsPerDistanceHistogram,
        kMaxCommandHistograms,
        kCommandStrideLength,
        kDistanceBlockSwitchCost,
        params,
        dist_split,
    );
    if 0 as libc::c_int != 0 {
        return;
    }
    BrotliFree(m, distance_prefixes as *mut libc::c_void);
    distance_prefixes = 0 as *mut uint16_t;
}
