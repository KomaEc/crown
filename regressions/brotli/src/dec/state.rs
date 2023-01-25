use ::libc;
use ::c2rust_bitfields;
extern "C" {
    
    
    
    
    
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

struct ErasedByPreprocessor6 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor7 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor8 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor9 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor10 { dummy: () }
pub type BrotliRunningState = libc::c_uint;
pub const BROTLI_STATE_DONE: BrotliRunningState = 26;
pub const BROTLI_STATE_BEFORE_COMPRESSED_METABLOCK_BODY: BrotliRunningState = 25;
pub const BROTLI_STATE_TREE_GROUP: BrotliRunningState = 24;
pub const BROTLI_STATE_CONTEXT_MAP_2: BrotliRunningState = 23;
pub const BROTLI_STATE_CONTEXT_MAP_1: BrotliRunningState = 22;
pub const BROTLI_STATE_HUFFMAN_CODE_3: BrotliRunningState = 21;
pub const BROTLI_STATE_HUFFMAN_CODE_2: BrotliRunningState = 20;
pub const BROTLI_STATE_HUFFMAN_CODE_1: BrotliRunningState = 19;
pub const BROTLI_STATE_HUFFMAN_CODE_0: BrotliRunningState = 18;
pub const BROTLI_STATE_BEFORE_COMPRESSED_METABLOCK_HEADER: BrotliRunningState = 17;
pub const BROTLI_STATE_COMMAND_POST_WRITE_2: BrotliRunningState = 16;
pub const BROTLI_STATE_COMMAND_POST_WRITE_1: BrotliRunningState = 15;
pub const BROTLI_STATE_METABLOCK_DONE: BrotliRunningState = 14;
pub const BROTLI_STATE_COMMAND_INNER_WRITE: BrotliRunningState = 13;
pub const BROTLI_STATE_METADATA: BrotliRunningState = 12;
pub const BROTLI_STATE_UNCOMPRESSED: BrotliRunningState = 11;
pub const BROTLI_STATE_COMMAND_POST_WRAP_COPY: BrotliRunningState = 10;
pub const BROTLI_STATE_COMMAND_POST_DECODE_LITERALS: BrotliRunningState = 9;
pub const BROTLI_STATE_COMMAND_INNER: BrotliRunningState = 8;
pub const BROTLI_STATE_COMMAND_BEGIN: BrotliRunningState = 7;
pub const BROTLI_STATE_CONTEXT_MODES: BrotliRunningState = 6;
pub const BROTLI_STATE_METABLOCK_HEADER_2: BrotliRunningState = 5;
pub const BROTLI_STATE_METABLOCK_HEADER: BrotliRunningState = 4;
pub const BROTLI_STATE_METABLOCK_BEGIN: BrotliRunningState = 3;
pub const BROTLI_STATE_INITIALIZE: BrotliRunningState = 2;
pub const BROTLI_STATE_LARGE_WINDOW_BITS: BrotliRunningState = 1;
pub const BROTLI_STATE_UNINITED: BrotliRunningState = 0;
pub type BrotliRunningMetablockHeaderState = libc::c_uint;
pub const BROTLI_STATE_METABLOCK_HEADER_METADATA: BrotliRunningMetablockHeaderState = 7;
pub const BROTLI_STATE_METABLOCK_HEADER_BYTES: BrotliRunningMetablockHeaderState = 6;
pub const BROTLI_STATE_METABLOCK_HEADER_RESERVED: BrotliRunningMetablockHeaderState = 5;
pub const BROTLI_STATE_METABLOCK_HEADER_UNCOMPRESSED: BrotliRunningMetablockHeaderState = 4;
pub const BROTLI_STATE_METABLOCK_HEADER_SIZE: BrotliRunningMetablockHeaderState = 3;
pub const BROTLI_STATE_METABLOCK_HEADER_NIBBLES: BrotliRunningMetablockHeaderState = 2;
pub const BROTLI_STATE_METABLOCK_HEADER_EMPTY: BrotliRunningMetablockHeaderState = 1;
pub const BROTLI_STATE_METABLOCK_HEADER_NONE: BrotliRunningMetablockHeaderState = 0;
pub type BrotliRunningUncompressedState = libc::c_uint;
pub const BROTLI_STATE_UNCOMPRESSED_WRITE: BrotliRunningUncompressedState = 1;
pub const BROTLI_STATE_UNCOMPRESSED_NONE: BrotliRunningUncompressedState = 0;
pub type BrotliRunningTreeGroupState = libc::c_uint;
pub const BROTLI_STATE_TREE_GROUP_LOOP: BrotliRunningTreeGroupState = 1;
pub const BROTLI_STATE_TREE_GROUP_NONE: BrotliRunningTreeGroupState = 0;
pub type BrotliRunningContextMapState = libc::c_uint;
pub const BROTLI_STATE_CONTEXT_MAP_TRANSFORM: BrotliRunningContextMapState = 4;
pub const BROTLI_STATE_CONTEXT_MAP_DECODE: BrotliRunningContextMapState = 3;
pub const BROTLI_STATE_CONTEXT_MAP_HUFFMAN: BrotliRunningContextMapState = 2;
pub const BROTLI_STATE_CONTEXT_MAP_READ_PREFIX: BrotliRunningContextMapState = 1;
pub const BROTLI_STATE_CONTEXT_MAP_NONE: BrotliRunningContextMapState = 0;
pub type BrotliRunningHuffmanState = libc::c_uint;
pub const BROTLI_STATE_HUFFMAN_LENGTH_SYMBOLS: BrotliRunningHuffmanState = 5;
pub const BROTLI_STATE_HUFFMAN_COMPLEX: BrotliRunningHuffmanState = 4;
pub const BROTLI_STATE_HUFFMAN_SIMPLE_BUILD: BrotliRunningHuffmanState = 3;
pub const BROTLI_STATE_HUFFMAN_SIMPLE_READ: BrotliRunningHuffmanState = 2;
pub const BROTLI_STATE_HUFFMAN_SIMPLE_SIZE: BrotliRunningHuffmanState = 1;
pub const BROTLI_STATE_HUFFMAN_NONE: BrotliRunningHuffmanState = 0;
pub type BrotliRunningDecodeUint8State = libc::c_uint;
pub const BROTLI_STATE_DECODE_UINT8_LONG: BrotliRunningDecodeUint8State = 2;
pub const BROTLI_STATE_DECODE_UINT8_SHORT: BrotliRunningDecodeUint8State = 1;
pub const BROTLI_STATE_DECODE_UINT8_NONE: BrotliRunningDecodeUint8State = 0;
pub type BrotliRunningReadBlockLengthState = libc::c_uint;
pub const BROTLI_STATE_READ_BLOCK_LENGTH_SUFFIX: BrotliRunningReadBlockLengthState = 1;
pub const BROTLI_STATE_READ_BLOCK_LENGTH_NONE: BrotliRunningReadBlockLengthState = 0;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor11 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor12 { dummy: () }
#[derive(Copy, Clone, BitfieldStruct)]

struct ErasedByPreprocessor13 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor14 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor15 { dummy: () }
pub type BrotliDecoderStateInternal = crate::src::dec::decode::BrotliDecoderStateStruct;
#[no_mangle]
pub unsafe extern "C" fn BrotliDecoderStateInit(
    mut s: *mut BrotliDecoderStateInternal,
    mut alloc_func: brotli_alloc_func,
    mut free_func: brotli_free_func,
    mut opaque: *mut libc::c_void,
) -> libc::c_int {
    if alloc_func.is_none() {
        (*s).alloc_func= Some(
            crate::src::common::platform::BrotliDefaultAllocFunc
                as unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void,
        );
        (*s).free_func= Some(
            crate::src::common::platform::BrotliDefaultFreeFunc
                as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
        );
        (*s).memory_manager_opaque= 0 as *mut libc::c_void;
    } else {
        (*s).alloc_func= alloc_func;
        (*s).free_func= free_func;
        (*s).memory_manager_opaque= opaque;
    }
    (*s).error_code= 0 as libc::c_int;
    crate::src::dec::bit_reader::BrotliInitBitReader(Some(&mut (*s).br));
    (*s).state= BROTLI_STATE_UNINITED;
    (*s).set_large_window(0 as libc::c_int as libc::c_uint);
    (*s).substate_metablock_header= BROTLI_STATE_METABLOCK_HEADER_NONE;
    (*s).substate_uncompressed= BROTLI_STATE_UNCOMPRESSED_NONE;
    (*s).substate_decode_uint8= BROTLI_STATE_DECODE_UINT8_NONE;
    (*s).substate_read_block_length= BROTLI_STATE_READ_BLOCK_LENGTH_NONE;
    (*s).buffer_length= 0 as libc::c_int as uint32_t;
    (*s).loop_counter= 0 as libc::c_int;
    (*s).pos= 0 as libc::c_int;
    (*s).rb_roundtrips= 0 as libc::c_int as size_t;
    (*s).partial_pos_out= 0 as libc::c_int as size_t;
    (*s).block_type_trees= 0 as *mut crate::src::dec::decode::HuffmanCode;
    (*s).block_len_trees= 0 as *mut crate::src::dec::decode::HuffmanCode;
    (*s).ringbuffer= 0 as *mut uint8_t;
    (*s).ringbuffer_size= 0 as libc::c_int;
    (*s).new_ringbuffer_size= 0 as libc::c_int;
    (*s).ringbuffer_mask= 0 as libc::c_int;
    (*s).context_map= 0 as *mut uint8_t;
    (*s).context_modes= 0 as *mut uint8_t;
    (*s).dist_context_map= 0 as *mut uint8_t;
    (*s).context_map_slice= 0 as *mut uint8_t;
    (*s).dist_context_map_slice= 0 as *mut uint8_t;
    (*s).literal_hgroup.codes= 0 as *mut crate::src::dec::decode::HuffmanCode;
    (*s).literal_hgroup.htrees= 0 as *mut *mut crate::src::dec::decode::HuffmanCode;
    (*s).insert_copy_hgroup.codes= 0 as *mut crate::src::dec::decode::HuffmanCode;
    (*s).insert_copy_hgroup.htrees= 0 as *mut *mut crate::src::dec::decode::HuffmanCode;
    (*s).distance_hgroup.codes= 0 as *mut crate::src::dec::decode::HuffmanCode;
    (*s).distance_hgroup.htrees= 0 as *mut *mut crate::src::dec::decode::HuffmanCode;
    (*s).set_is_last_metablock(0 as libc::c_int as libc::c_uint);
    (*s).set_is_uncompressed(0 as libc::c_int as libc::c_uint);
    (*s).set_is_metadata(0 as libc::c_int as libc::c_uint);
    (*s).set_should_wrap_ringbuffer(0 as libc::c_int as libc::c_uint);
    (*s).set_canny_ringbuffer_allocation(1 as libc::c_int as libc::c_uint);
    (*s).window_bits= 0 as libc::c_int as uint32_t;
    (*s).max_distance= 0 as libc::c_int;
    (*s).dist_rb[0 as libc::c_int as usize]= 16 as libc::c_int;
    (*s).dist_rb[1 as libc::c_int as usize]= 15 as libc::c_int;
    (*s).dist_rb[2 as libc::c_int as usize]= 11 as libc::c_int;
    (*s).dist_rb[3 as libc::c_int as usize]= 4 as libc::c_int;
    (*s).dist_rb_idx= 0 as libc::c_int;
    (*s).block_type_trees= 0 as *mut crate::src::dec::decode::HuffmanCode;
    (*s).block_len_trees= 0 as *mut crate::src::dec::decode::HuffmanCode;
    (*s).mtf_upper_bound= 63 as libc::c_int as uint32_t;
    (*s).dictionary= crate::src::common::dictionary::BrotliGetDictionary();
    (*s).transforms= crate::src::common::transform::BrotliGetTransforms();
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn BrotliDecoderStateMetablockBegin(
    mut s: Option<&mut BrotliDecoderStateInternal>,
) {
    (*s.as_deref_mut().unwrap()).meta_block_remaining_len= 0 as libc::c_int;
    (*s.as_deref_mut().unwrap()).block_length[0 as libc::c_int
        as usize]= (1 as libc::c_uint) << 24 as libc::c_int;
    (*s.as_deref_mut().unwrap()).block_length[1 as libc::c_int
        as usize]= (1 as libc::c_uint) << 24 as libc::c_int;
    (*s.as_deref_mut().unwrap()).block_length[2 as libc::c_int
        as usize]= (1 as libc::c_uint) << 24 as libc::c_int;
    (*s.as_deref_mut().unwrap()).num_block_types[0 as libc::c_int as usize]= 1 as libc::c_int as uint32_t;
    (*s.as_deref_mut().unwrap()).num_block_types[1 as libc::c_int as usize]= 1 as libc::c_int as uint32_t;
    (*s.as_deref_mut().unwrap()).num_block_types[2 as libc::c_int as usize]= 1 as libc::c_int as uint32_t;
    (*s.as_deref_mut().unwrap()).block_type_rb[0 as libc::c_int as usize]= 1 as libc::c_int as uint32_t;
    (*s.as_deref_mut().unwrap()).block_type_rb[1 as libc::c_int as usize]= 0 as libc::c_int as uint32_t;
    (*s.as_deref_mut().unwrap()).block_type_rb[2 as libc::c_int as usize]= 1 as libc::c_int as uint32_t;
    (*s.as_deref_mut().unwrap()).block_type_rb[3 as libc::c_int as usize]= 0 as libc::c_int as uint32_t;
    (*s.as_deref_mut().unwrap()).block_type_rb[4 as libc::c_int as usize]= 1 as libc::c_int as uint32_t;
    (*s.as_deref_mut().unwrap()).block_type_rb[5 as libc::c_int as usize]= 0 as libc::c_int as uint32_t;
    (*s.as_deref_mut().unwrap()).context_map= 0 as *mut uint8_t;
    (*s.as_deref_mut().unwrap()).context_modes= 0 as *mut uint8_t;
    (*s.as_deref_mut().unwrap()).dist_context_map= 0 as *mut uint8_t;
    (*s.as_deref_mut().unwrap()).context_map_slice= 0 as *mut uint8_t;
    (*s.as_deref_mut().unwrap()).literal_htree= 0 as *mut crate::src::dec::decode::HuffmanCode;
    (*s.as_deref_mut().unwrap()).dist_context_map_slice= 0 as *mut uint8_t;
    (*s.as_deref_mut().unwrap()).dist_htree_index= 0 as libc::c_int as uint8_t;
    (*s.as_deref_mut().unwrap()).context_lookup= 0 as *const uint8_t;
    (*s.as_deref_mut().unwrap()).literal_hgroup.codes= 0 as *mut crate::src::dec::decode::HuffmanCode;
    (*s.as_deref_mut().unwrap()).literal_hgroup.htrees= 0 as *mut *mut crate::src::dec::decode::HuffmanCode;
    (*s.as_deref_mut().unwrap()).insert_copy_hgroup.codes= 0 as *mut crate::src::dec::decode::HuffmanCode;
    (*s.as_deref_mut().unwrap()).insert_copy_hgroup.htrees= 0 as *mut *mut crate::src::dec::decode::HuffmanCode;
    (*s.as_deref_mut().unwrap()).distance_hgroup.codes= 0 as *mut crate::src::dec::decode::HuffmanCode;
    (*s.as_deref_mut().unwrap()).distance_hgroup.htrees= 0 as *mut *mut crate::src::dec::decode::HuffmanCode;
}
#[no_mangle]
pub unsafe extern "C" fn BrotliDecoderStateCleanupAfterMetablock(
    mut s: Option<&mut BrotliDecoderStateInternal>,
) {
    (*s.as_deref().unwrap()).free_func
        .expect(
            "non-null function pointer",
        )((*s.as_deref().unwrap()).memory_manager_opaque, (*s.as_deref().unwrap()).context_modes as *mut libc::c_void);
    (*s.as_deref_mut().unwrap()).context_modes= 0 as *mut uint8_t;
    (*s.as_deref().unwrap()).free_func
        .expect(
            "non-null function pointer",
        )((*s.as_deref().unwrap()).memory_manager_opaque, (*s.as_deref().unwrap()).context_map as *mut libc::c_void);
    (*s.as_deref_mut().unwrap()).context_map= 0 as *mut uint8_t;
    (*s.as_deref().unwrap()).free_func
        .expect(
            "non-null function pointer",
        )((*s.as_deref().unwrap()).memory_manager_opaque, (*s.as_deref().unwrap()).dist_context_map as *mut libc::c_void);
    (*s.as_deref_mut().unwrap()).dist_context_map= 0 as *mut uint8_t;
    (*s.as_deref().unwrap()).free_func
        .expect(
            "non-null function pointer",
        )((*s.as_deref().unwrap()).memory_manager_opaque, (*s.as_deref().unwrap()).literal_hgroup.htrees as *mut libc::c_void);
    (*s.as_deref_mut().unwrap()).literal_hgroup.htrees= 0 as *mut *mut crate::src::dec::decode::HuffmanCode;
    (*s.as_deref().unwrap()).free_func
        .expect(
            "non-null function pointer",
        )(
        (*s.as_deref().unwrap()).memory_manager_opaque,
        (*s.as_deref().unwrap()).insert_copy_hgroup.htrees as *mut libc::c_void,
    );
    (*s.as_deref_mut().unwrap()).insert_copy_hgroup.htrees= 0 as *mut *mut crate::src::dec::decode::HuffmanCode;
    (*s.as_deref().unwrap()).free_func
        .expect(
            "non-null function pointer",
        )((*s.as_deref().unwrap()).memory_manager_opaque, (*s.as_deref().unwrap()).distance_hgroup.htrees as *mut libc::c_void);
    (*s.as_deref_mut().unwrap()).distance_hgroup.htrees= 0 as *mut *mut crate::src::dec::decode::HuffmanCode;
}
#[no_mangle]
pub unsafe extern "C" fn BrotliDecoderStateCleanup(
    mut s: Option<&mut BrotliDecoderStateInternal>,
) {
    BrotliDecoderStateCleanupAfterMetablock(s.as_deref_mut());
    (*s.as_deref().unwrap()).free_func
        .expect(
            "non-null function pointer",
        )((*s.as_deref().unwrap()).memory_manager_opaque, (*s.as_deref().unwrap()).ringbuffer as *mut libc::c_void);
    (*s.as_deref_mut().unwrap()).ringbuffer= 0 as *mut uint8_t;
    (*s.as_deref().unwrap()).free_func
        .expect(
            "non-null function pointer",
        )((*s.as_deref().unwrap()).memory_manager_opaque, (*s.as_deref().unwrap()).block_type_trees as *mut libc::c_void);
    (*s.as_deref_mut().unwrap()).block_type_trees= 0 as *mut crate::src::dec::decode::HuffmanCode;
}
#[no_mangle]
pub unsafe extern "C" fn BrotliDecoderHuffmanTreeGroupInit(
    mut s: *mut BrotliDecoderStateInternal,
    mut group: *mut crate::src::dec::decode::HuffmanTreeGroup,
    mut alphabet_size_max: uint32_t,
    mut alphabet_size_limit: uint32_t,
    mut ntrees: uint32_t,
) -> libc::c_int {
    let max_table_size = alphabet_size_limit
        .wrapping_add(376 as libc::c_int as libc::c_uint) as size_t;
    let code_size = (::std::mem::size_of::<crate::src::dec::decode::HuffmanCode>() as libc::c_ulong)
        .wrapping_mul(ntrees as libc::c_ulong)
        .wrapping_mul(max_table_size);
    let htree_size = (::std::mem::size_of::<*mut crate::src::dec::decode::HuffmanCode>() as libc::c_ulong)
        .wrapping_mul(ntrees as libc::c_ulong);
    let mut p = (*s).alloc_func
        .expect(
            "non-null function pointer",
        )((*s).memory_manager_opaque, code_size.wrapping_add(htree_size))
        as *mut *mut crate::src::dec::decode::HuffmanCode;
    (*group).alphabet_size_max= alphabet_size_max as uint16_t;
    (*group).alphabet_size_limit= alphabet_size_limit as uint16_t;
    (*group).num_htrees= ntrees as uint16_t;
    (*group).htrees= p;
    (*group).codes= core::ptr::addr_of_mut!(*p.offset(ntrees as isize)) as *mut *mut crate::src::dec::decode::HuffmanCode
        as *mut crate::src::dec::decode::HuffmanCode;
    return !p.is_null() as libc::c_int;
}
