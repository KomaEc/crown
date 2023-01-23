use ::libc;
use ::c2rust_bitfields;
extern "C" {
    fn BrotliInitBitReader(br: *mut BrotliBitReader);
    fn BrotliDefaultAllocFunc(
        opaque: *mut libc::c_void,
        size: size_t,
    ) -> *mut libc::c_void;
    fn BrotliDefaultFreeFunc(opaque: *mut libc::c_void, address: *mut libc::c_void);
    fn BrotliGetDictionary() -> *const BrotliDictionary;
    fn BrotliGetTransforms() -> *const BrotliTransforms;
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
#[repr(C)]
pub struct BrotliDictionary {
    pub size_bits_by_length: [uint8_t; 32],
    pub offsets_by_length: [uint32_t; 32],
    pub data_size: size_t,
    pub data: *const uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BrotliTransforms {
    pub prefix_suffix_size: uint16_t,
    pub prefix_suffix: *const uint8_t,
    pub prefix_suffix_map: *const uint16_t,
    pub num_transforms: uint32_t,
    pub transforms: *const uint8_t,
    pub params: *const uint8_t,
    pub cutOffTransforms: [int16_t; 10],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BrotliBitReader {
    pub val_: uint64_t,
    pub bit_pos_: uint32_t,
    pub next_in: *const uint8_t,
    pub avail_in: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HuffmanCode {
    pub bits: uint8_t,
    pub value: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HuffmanTreeGroup {
    pub htrees: *mut *mut HuffmanCode,
    pub codes: *mut HuffmanCode,
    pub alphabet_size_max: uint16_t,
    pub alphabet_size_limit: uint16_t,
    pub num_htrees: uint16_t,
}
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
#[repr(C)]
pub struct BrotliMetablockHeaderArena {
    pub substate_tree_group: BrotliRunningTreeGroupState,
    pub substate_context_map: BrotliRunningContextMapState,
    pub substate_huffman: BrotliRunningHuffmanState,
    pub sub_loop_counter: uint32_t,
    pub repeat_code_len: uint32_t,
    pub prev_code_len: uint32_t,
    pub symbol: uint32_t,
    pub repeat: uint32_t,
    pub space: uint32_t,
    pub table: [HuffmanCode; 32],
    pub symbol_lists: *mut uint16_t,
    pub symbols_lists_array: [uint16_t; 720],
    pub next_symbol: [libc::c_int; 32],
    pub code_length_code_lengths: [uint8_t; 18],
    pub code_length_histo: [uint16_t; 16],
    pub htree_index: libc::c_int,
    pub next: *mut HuffmanCode,
    pub context_index: uint32_t,
    pub max_run_length_prefix: uint32_t,
    pub code: uint32_t,
    pub context_map_table: [HuffmanCode; 646],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BrotliMetablockBodyArena {
    pub dist_extra_bits: [uint8_t; 544],
    pub dist_offset: [uint32_t; 544],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct BrotliDecoderStateStruct {
    pub state: BrotliRunningState,
    pub loop_counter: libc::c_int,
    pub br: BrotliBitReader,
    pub alloc_func: brotli_alloc_func,
    pub free_func: brotli_free_func,
    pub memory_manager_opaque: *mut libc::c_void,
    pub buffer: C2RustUnnamed_0,
    pub buffer_length: uint32_t,
    pub pos: libc::c_int,
    pub max_backward_distance: libc::c_int,
    pub max_distance: libc::c_int,
    pub ringbuffer_size: libc::c_int,
    pub ringbuffer_mask: libc::c_int,
    pub dist_rb_idx: libc::c_int,
    pub dist_rb: [libc::c_int; 4],
    pub error_code: libc::c_int,
    pub ringbuffer: *mut uint8_t,
    pub ringbuffer_end: *mut uint8_t,
    pub htree_command: *mut HuffmanCode,
    pub context_lookup: *const uint8_t,
    pub context_map_slice: *mut uint8_t,
    pub dist_context_map_slice: *mut uint8_t,
    pub literal_hgroup: HuffmanTreeGroup,
    pub insert_copy_hgroup: HuffmanTreeGroup,
    pub distance_hgroup: HuffmanTreeGroup,
    pub block_type_trees: *mut HuffmanCode,
    pub block_len_trees: *mut HuffmanCode,
    pub trivial_literal_context: libc::c_int,
    pub distance_context: libc::c_int,
    pub meta_block_remaining_len: libc::c_int,
    pub block_length_index: uint32_t,
    pub block_length: [uint32_t; 3],
    pub num_block_types: [uint32_t; 3],
    pub block_type_rb: [uint32_t; 6],
    pub distance_postfix_bits: uint32_t,
    pub num_direct_distance_codes: uint32_t,
    pub num_dist_htrees: uint32_t,
    pub dist_context_map: *mut uint8_t,
    pub literal_htree: *mut HuffmanCode,
    pub dist_htree_index: uint8_t,
    pub copy_length: libc::c_int,
    pub distance_code: libc::c_int,
    pub rb_roundtrips: size_t,
    pub partial_pos_out: size_t,
    pub mtf_upper_bound: uint32_t,
    pub mtf: [uint32_t; 65],
    pub substate_metablock_header: BrotliRunningMetablockHeaderState,
    pub substate_uncompressed: BrotliRunningUncompressedState,
    pub substate_decode_uint8: BrotliRunningDecodeUint8State,
    pub substate_read_block_length: BrotliRunningReadBlockLengthState,
    #[bitfield(name = "is_last_metablock", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "is_uncompressed", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "is_metadata", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "should_wrap_ringbuffer", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "canny_ringbuffer_allocation", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "large_window", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "size_nibbles", ty = "libc::c_uint", bits = "6..=13")]
    pub is_last_metablock_is_uncompressed_is_metadata_should_wrap_ringbuffer_canny_ringbuffer_allocation_large_window_size_nibbles: [u8; 2],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 2],
    pub window_bits: uint32_t,
    pub new_ringbuffer_size: libc::c_int,
    pub num_literal_htrees: uint32_t,
    pub context_map: *mut uint8_t,
    pub context_modes: *mut uint8_t,
    pub dictionary: *const BrotliDictionary,
    pub transforms: *const BrotliTransforms,
    pub trivial_literal_contexts: [uint32_t; 8],
    pub arena: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub header: BrotliMetablockHeaderArena,
    pub body: BrotliMetablockBodyArena,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub u64_0: uint64_t,
    pub u8_0: [uint8_t; 8],
}
pub type BrotliDecoderStateInternal = BrotliDecoderStateStruct;
#[no_mangle]
pub unsafe extern "C" fn BrotliDecoderStateInit(
    mut s: *mut BrotliDecoderStateInternal,
    mut alloc_func: brotli_alloc_func,
    mut free_func: brotli_free_func,
    mut opaque: *mut libc::c_void,
) -> libc::c_int {
    if alloc_func.is_none() {
        let ref mut fresh0 = (*s).alloc_func;
        *fresh0 = Some(
            BrotliDefaultAllocFunc
                as unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void,
        );
        let ref mut fresh1 = (*s).free_func;
        *fresh1 = Some(
            BrotliDefaultFreeFunc
                as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
        );
        let ref mut fresh2 = (*s).memory_manager_opaque;
        *fresh2 = 0 as *mut libc::c_void;
    } else {
        let ref mut fresh3 = (*s).alloc_func;
        *fresh3 = alloc_func;
        let ref mut fresh4 = (*s).free_func;
        *fresh4 = free_func;
        let ref mut fresh5 = (*s).memory_manager_opaque;
        *fresh5 = opaque;
    }
    (*s).error_code = 0 as libc::c_int;
    BrotliInitBitReader(&mut (*s).br);
    (*s).state = BROTLI_STATE_UNINITED;
    (*s).set_large_window(0 as libc::c_int as libc::c_uint);
    (*s).substate_metablock_header = BROTLI_STATE_METABLOCK_HEADER_NONE;
    (*s).substate_uncompressed = BROTLI_STATE_UNCOMPRESSED_NONE;
    (*s).substate_decode_uint8 = BROTLI_STATE_DECODE_UINT8_NONE;
    (*s).substate_read_block_length = BROTLI_STATE_READ_BLOCK_LENGTH_NONE;
    (*s).buffer_length = 0 as libc::c_int as uint32_t;
    (*s).loop_counter = 0 as libc::c_int;
    (*s).pos = 0 as libc::c_int;
    (*s).rb_roundtrips = 0 as libc::c_int as size_t;
    (*s).partial_pos_out = 0 as libc::c_int as size_t;
    let ref mut fresh6 = (*s).block_type_trees;
    *fresh6 = 0 as *mut HuffmanCode;
    let ref mut fresh7 = (*s).block_len_trees;
    *fresh7 = 0 as *mut HuffmanCode;
    let ref mut fresh8 = (*s).ringbuffer;
    *fresh8 = 0 as *mut uint8_t;
    (*s).ringbuffer_size = 0 as libc::c_int;
    (*s).new_ringbuffer_size = 0 as libc::c_int;
    (*s).ringbuffer_mask = 0 as libc::c_int;
    let ref mut fresh9 = (*s).context_map;
    *fresh9 = 0 as *mut uint8_t;
    let ref mut fresh10 = (*s).context_modes;
    *fresh10 = 0 as *mut uint8_t;
    let ref mut fresh11 = (*s).dist_context_map;
    *fresh11 = 0 as *mut uint8_t;
    let ref mut fresh12 = (*s).context_map_slice;
    *fresh12 = 0 as *mut uint8_t;
    let ref mut fresh13 = (*s).dist_context_map_slice;
    *fresh13 = 0 as *mut uint8_t;
    let ref mut fresh14 = (*s).literal_hgroup.codes;
    *fresh14 = 0 as *mut HuffmanCode;
    let ref mut fresh15 = (*s).literal_hgroup.htrees;
    *fresh15 = 0 as *mut *mut HuffmanCode;
    let ref mut fresh16 = (*s).insert_copy_hgroup.codes;
    *fresh16 = 0 as *mut HuffmanCode;
    let ref mut fresh17 = (*s).insert_copy_hgroup.htrees;
    *fresh17 = 0 as *mut *mut HuffmanCode;
    let ref mut fresh18 = (*s).distance_hgroup.codes;
    *fresh18 = 0 as *mut HuffmanCode;
    let ref mut fresh19 = (*s).distance_hgroup.htrees;
    *fresh19 = 0 as *mut *mut HuffmanCode;
    (*s).set_is_last_metablock(0 as libc::c_int as libc::c_uint);
    (*s).set_is_uncompressed(0 as libc::c_int as libc::c_uint);
    (*s).set_is_metadata(0 as libc::c_int as libc::c_uint);
    (*s).set_should_wrap_ringbuffer(0 as libc::c_int as libc::c_uint);
    (*s).set_canny_ringbuffer_allocation(1 as libc::c_int as libc::c_uint);
    (*s).window_bits = 0 as libc::c_int as uint32_t;
    (*s).max_distance = 0 as libc::c_int;
    (*s).dist_rb[0 as libc::c_int as usize] = 16 as libc::c_int;
    (*s).dist_rb[1 as libc::c_int as usize] = 15 as libc::c_int;
    (*s).dist_rb[2 as libc::c_int as usize] = 11 as libc::c_int;
    (*s).dist_rb[3 as libc::c_int as usize] = 4 as libc::c_int;
    (*s).dist_rb_idx = 0 as libc::c_int;
    let ref mut fresh20 = (*s).block_type_trees;
    *fresh20 = 0 as *mut HuffmanCode;
    let ref mut fresh21 = (*s).block_len_trees;
    *fresh21 = 0 as *mut HuffmanCode;
    (*s).mtf_upper_bound = 63 as libc::c_int as uint32_t;
    let ref mut fresh22 = (*s).dictionary;
    *fresh22 = BrotliGetDictionary();
    let ref mut fresh23 = (*s).transforms;
    *fresh23 = BrotliGetTransforms();
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn BrotliDecoderStateMetablockBegin(
    mut s: *mut BrotliDecoderStateInternal,
) {
    (*s).meta_block_remaining_len = 0 as libc::c_int;
    (*s)
        .block_length[0 as libc::c_int
        as usize] = (1 as libc::c_uint) << 24 as libc::c_int;
    (*s)
        .block_length[1 as libc::c_int
        as usize] = (1 as libc::c_uint) << 24 as libc::c_int;
    (*s)
        .block_length[2 as libc::c_int
        as usize] = (1 as libc::c_uint) << 24 as libc::c_int;
    (*s).num_block_types[0 as libc::c_int as usize] = 1 as libc::c_int as uint32_t;
    (*s).num_block_types[1 as libc::c_int as usize] = 1 as libc::c_int as uint32_t;
    (*s).num_block_types[2 as libc::c_int as usize] = 1 as libc::c_int as uint32_t;
    (*s).block_type_rb[0 as libc::c_int as usize] = 1 as libc::c_int as uint32_t;
    (*s).block_type_rb[1 as libc::c_int as usize] = 0 as libc::c_int as uint32_t;
    (*s).block_type_rb[2 as libc::c_int as usize] = 1 as libc::c_int as uint32_t;
    (*s).block_type_rb[3 as libc::c_int as usize] = 0 as libc::c_int as uint32_t;
    (*s).block_type_rb[4 as libc::c_int as usize] = 1 as libc::c_int as uint32_t;
    (*s).block_type_rb[5 as libc::c_int as usize] = 0 as libc::c_int as uint32_t;
    let ref mut fresh24 = (*s).context_map;
    *fresh24 = 0 as *mut uint8_t;
    let ref mut fresh25 = (*s).context_modes;
    *fresh25 = 0 as *mut uint8_t;
    let ref mut fresh26 = (*s).dist_context_map;
    *fresh26 = 0 as *mut uint8_t;
    let ref mut fresh27 = (*s).context_map_slice;
    *fresh27 = 0 as *mut uint8_t;
    let ref mut fresh28 = (*s).literal_htree;
    *fresh28 = 0 as *mut HuffmanCode;
    let ref mut fresh29 = (*s).dist_context_map_slice;
    *fresh29 = 0 as *mut uint8_t;
    (*s).dist_htree_index = 0 as libc::c_int as uint8_t;
    let ref mut fresh30 = (*s).context_lookup;
    *fresh30 = 0 as *const uint8_t;
    let ref mut fresh31 = (*s).literal_hgroup.codes;
    *fresh31 = 0 as *mut HuffmanCode;
    let ref mut fresh32 = (*s).literal_hgroup.htrees;
    *fresh32 = 0 as *mut *mut HuffmanCode;
    let ref mut fresh33 = (*s).insert_copy_hgroup.codes;
    *fresh33 = 0 as *mut HuffmanCode;
    let ref mut fresh34 = (*s).insert_copy_hgroup.htrees;
    *fresh34 = 0 as *mut *mut HuffmanCode;
    let ref mut fresh35 = (*s).distance_hgroup.codes;
    *fresh35 = 0 as *mut HuffmanCode;
    let ref mut fresh36 = (*s).distance_hgroup.htrees;
    *fresh36 = 0 as *mut *mut HuffmanCode;
}
#[no_mangle]
pub unsafe extern "C" fn BrotliDecoderStateCleanupAfterMetablock(
    mut s: *mut BrotliDecoderStateInternal,
) {
    ((*s).free_func)
        .expect(
            "non-null function pointer",
        )((*s).memory_manager_opaque, (*s).context_modes as *mut libc::c_void);
    let ref mut fresh37 = (*s).context_modes;
    *fresh37 = 0 as *mut uint8_t;
    ((*s).free_func)
        .expect(
            "non-null function pointer",
        )((*s).memory_manager_opaque, (*s).context_map as *mut libc::c_void);
    let ref mut fresh38 = (*s).context_map;
    *fresh38 = 0 as *mut uint8_t;
    ((*s).free_func)
        .expect(
            "non-null function pointer",
        )((*s).memory_manager_opaque, (*s).dist_context_map as *mut libc::c_void);
    let ref mut fresh39 = (*s).dist_context_map;
    *fresh39 = 0 as *mut uint8_t;
    ((*s).free_func)
        .expect(
            "non-null function pointer",
        )((*s).memory_manager_opaque, (*s).literal_hgroup.htrees as *mut libc::c_void);
    let ref mut fresh40 = (*s).literal_hgroup.htrees;
    *fresh40 = 0 as *mut *mut HuffmanCode;
    ((*s).free_func)
        .expect(
            "non-null function pointer",
        )(
        (*s).memory_manager_opaque,
        (*s).insert_copy_hgroup.htrees as *mut libc::c_void,
    );
    let ref mut fresh41 = (*s).insert_copy_hgroup.htrees;
    *fresh41 = 0 as *mut *mut HuffmanCode;
    ((*s).free_func)
        .expect(
            "non-null function pointer",
        )((*s).memory_manager_opaque, (*s).distance_hgroup.htrees as *mut libc::c_void);
    let ref mut fresh42 = (*s).distance_hgroup.htrees;
    *fresh42 = 0 as *mut *mut HuffmanCode;
}
#[no_mangle]
pub unsafe extern "C" fn BrotliDecoderStateCleanup(
    mut s: *mut BrotliDecoderStateInternal,
) {
    BrotliDecoderStateCleanupAfterMetablock(s);
    ((*s).free_func)
        .expect(
            "non-null function pointer",
        )((*s).memory_manager_opaque, (*s).ringbuffer as *mut libc::c_void);
    let ref mut fresh43 = (*s).ringbuffer;
    *fresh43 = 0 as *mut uint8_t;
    ((*s).free_func)
        .expect(
            "non-null function pointer",
        )((*s).memory_manager_opaque, (*s).block_type_trees as *mut libc::c_void);
    let ref mut fresh44 = (*s).block_type_trees;
    *fresh44 = 0 as *mut HuffmanCode;
}
#[no_mangle]
pub unsafe extern "C" fn BrotliDecoderHuffmanTreeGroupInit(
    mut s: *mut BrotliDecoderStateInternal,
    mut group: *mut HuffmanTreeGroup,
    mut alphabet_size_max: uint32_t,
    mut alphabet_size_limit: uint32_t,
    mut ntrees: uint32_t,
) -> libc::c_int {
    let max_table_size = alphabet_size_limit
        .wrapping_add(376 as libc::c_int as libc::c_uint) as size_t;
    let code_size = (::std::mem::size_of::<HuffmanCode>() as libc::c_ulong)
        .wrapping_mul(ntrees as libc::c_ulong)
        .wrapping_mul(max_table_size);
    let htree_size = (::std::mem::size_of::<*mut HuffmanCode>() as libc::c_ulong)
        .wrapping_mul(ntrees as libc::c_ulong);
    let mut p = ((*s).alloc_func)
        .expect(
            "non-null function pointer",
        )((*s).memory_manager_opaque, code_size.wrapping_add(htree_size))
        as *mut *mut HuffmanCode;
    (*group).alphabet_size_max = alphabet_size_max as uint16_t;
    (*group).alphabet_size_limit = alphabet_size_limit as uint16_t;
    (*group).num_htrees = ntrees as uint16_t;
    let ref mut fresh45 = (*group).htrees;
    *fresh45 = p;
    let ref mut fresh46 = (*group).codes;
    *fresh46 = &mut *p.offset(ntrees as isize) as *mut *mut HuffmanCode
        as *mut HuffmanCode;
    return !p.is_null() as libc::c_int;
}
