use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    static _kBrotliContextLookupTable: [uint8_t; 2048];
    static kBrotliLog2Table: [libc::c_double; 256];
    
    fn log2(_: libc::c_double) -> libc::c_double;
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
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
pub type BrotliEncoderMode = libc::c_uint;
pub const BROTLI_MODE_FONT: BrotliEncoderMode = 2;
pub const BROTLI_MODE_TEXT: BrotliEncoderMode = 1;
pub const BROTLI_MODE_GENERIC: BrotliEncoderMode = 0;
pub type BrotliEncoderOperation = libc::c_uint;
pub const BROTLI_OPERATION_EMIT_METADATA: BrotliEncoderOperation = 3;
pub const BROTLI_OPERATION_FINISH: BrotliEncoderOperation = 2;
pub const BROTLI_OPERATION_FLUSH: BrotliEncoderOperation = 1;
pub const BROTLI_OPERATION_PROCESS: BrotliEncoderOperation = 0;
pub type BrotliEncoderParameter = libc::c_uint;
pub const BROTLI_PARAM_STREAM_OFFSET: BrotliEncoderParameter = 9;
pub const BROTLI_PARAM_NDIRECT: BrotliEncoderParameter = 8;
pub const BROTLI_PARAM_NPOSTFIX: BrotliEncoderParameter = 7;
pub const BROTLI_PARAM_LARGE_WINDOW: BrotliEncoderParameter = 6;
pub const BROTLI_PARAM_SIZE_HINT: BrotliEncoderParameter = 5;
pub const BROTLI_PARAM_DISABLE_LITERAL_CONTEXT_MODELING: BrotliEncoderParameter = 4;
pub const BROTLI_PARAM_LGBLOCK: BrotliEncoderParameter = 3;
pub const BROTLI_PARAM_LGWIN: BrotliEncoderParameter = 2;
pub const BROTLI_PARAM_QUALITY: BrotliEncoderParameter = 1;
pub const BROTLI_PARAM_MODE: BrotliEncoderParameter = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BrotliEncoderStateStruct {
    pub params: crate::src::enc::backward_references::BrotliEncoderParams,
    pub memory_manager_: crate::src::enc::backward_references_hq::MemoryManager,
    pub input_pos_: uint64_t,
    pub ringbuffer_: RingBuffer,
    pub cmd_alloc_size_: size_t,
    pub commands_: *mut crate::src::enc::backward_references::Command,
    pub num_commands_: size_t,
    pub num_literals_: size_t,
    pub last_insert_len_: size_t,
    pub last_flush_pos_: uint64_t,
    pub last_processed_pos_: uint64_t,
    pub dist_cache_: [libc::c_int; 16],
    pub saved_dist_cache_: [libc::c_int; 4],
    pub last_bytes_: uint16_t,
    pub last_bytes_bits_: uint8_t,
    pub flint_: int8_t,
    pub prev_byte_: uint8_t,
    pub prev_byte2_: uint8_t,
    pub storage_size_: size_t,
    pub storage_: *mut uint8_t,
    pub hasher_: crate::src::enc::backward_references::Hasher,
    pub small_table_: [libc::c_int; 1024],
    pub large_table_: *mut libc::c_int,
    pub large_table_size_: size_t,
    pub cmd_depths_: [uint8_t; 128],
    pub cmd_bits_: [uint16_t; 128],
    pub cmd_code_: [uint8_t; 512],
    pub cmd_code_numbits_: size_t,
    pub command_buf_: *mut uint32_t,
    pub literal_buf_: *mut uint8_t,
    pub next_out_: *mut uint8_t,
    pub available_out_: size_t,
    pub total_out_: size_t,
    pub tiny_buf_: crate::src::dec::decode::C2RustUnnamed_0,
    pub remaining_metadata_bytes_: uint32_t,
    pub stream_state_: BrotliEncoderStreamState,
    pub is_last_block_emitted_: libc::c_int,
    pub is_initialized_: libc::c_int,
}
pub type BrotliEncoderStreamState = libc::c_uint;
pub const BROTLI_STREAM_METADATA_BODY: BrotliEncoderStreamState = 4;
pub const BROTLI_STREAM_METADATA_HEAD: BrotliEncoderStreamState = 3;
pub const BROTLI_STREAM_FINISHED: BrotliEncoderStreamState = 2;
pub const BROTLI_STREAM_FLUSH_REQUESTED: BrotliEncoderStreamState = 1;
pub const BROTLI_STREAM_PROCESSING: BrotliEncoderStreamState = 0;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor75 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor76 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor77 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor78 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor79 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor80 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor81 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor82 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor83 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor84 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor85 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor86 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor87 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor88 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor89 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor90 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor91 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor92 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor93 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor94 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor95 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor96 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor97 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor98 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor99 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor100 { dummy: () }
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RingBuffer {
    pub size_: uint32_t,
    pub mask_: uint32_t,
    pub tail_size_: uint32_t,
    pub total_size_: uint32_t,
    pub cur_size_: uint32_t,
    pub pos_: uint32_t,
    pub data_: *mut uint8_t,
    pub buffer_: *mut uint8_t,
}
#[derive(Copy, Clone)]

struct ErasedByPreprocessor101 { dummy: () }
pub type BrotliEncoderState = BrotliEncoderStateStruct;
pub type ContextType = libc::c_uint;
pub const CONTEXT_SIGNED: ContextType = 3;
pub const CONTEXT_UTF8: ContextType = 2;
pub const CONTEXT_MSB6: ContextType = 1;
pub const CONTEXT_LSB6: ContextType = 0;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor102 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor103 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor104 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor105 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor106 { dummy: () }
pub type ContextLut = *const uint8_t;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor107 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor108 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor109 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor110 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor111 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor112 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor113 { dummy: () }
pub const BROTLI_FLINT_WAITING_FOR_FLUSHING: BrotliEncoderFlintState = -1;
pub const BROTLI_FLINT_DONE: BrotliEncoderFlintState = -2;
pub const BROTLI_FLINT_NEEDS_2_BYTES: BrotliEncoderFlintState = 2;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor114 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor115 { dummy: () }
pub type BrotliEncoderFlintState = libc::c_int;
pub const BROTLI_FLINT_WAITING_FOR_PROCESSING: BrotliEncoderFlintState = 0;
pub const BROTLI_FLINT_NEEDS_1_BYTE: BrotliEncoderFlintState = 1;
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
unsafe extern "C" fn brotli_max_int(
    mut a: libc::c_int,
    mut b: libc::c_int,
) -> libc::c_int {
    return if a > b { a } else { b };
}
#[inline(always)]
unsafe extern "C" fn brotli_min_int(
    mut a: libc::c_int,
    mut b: libc::c_int,
) -> libc::c_int {
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
unsafe extern "C" fn brotli_min_uint32_t(mut a: uint32_t, mut b: uint32_t) -> uint32_t {
    return if a < b { a } else { b };
}
#[inline(always)]
unsafe extern "C" fn HashTypeLengthH4() -> size_t {
    return 8 as libc::c_int as size_t;
}
#[inline(always)]
unsafe extern "C" fn ComputeLgBlock(
    mut params: *const crate::src::enc::backward_references::BrotliEncoderParams,
) -> libc::c_int {
    let mut lgblock = (*params).lgblock;
    if (*params).quality == 0 as libc::c_int || (*params).quality == 1 as libc::c_int {
        lgblock= (*params).lgwin;
    } else if (*params).quality < 4 as libc::c_int {
        lgblock= 14 as libc::c_int;
    } else if lgblock == 0 as libc::c_int {
        lgblock= 16 as libc::c_int;
        if (*params).quality >= 9 as libc::c_int && (*params).lgwin > lgblock {
            lgblock= brotli_min_int(18 as libc::c_int, (*params).lgwin);
        }
    } else {
        lgblock= brotli_min_int(
            24 as libc::c_int,
            brotli_max_int(16 as libc::c_int, lgblock),
        );
    }
    return lgblock;
}
#[inline(always)]
unsafe extern "C" fn SanitizeParams(mut params: Option<&mut crate::src::enc::backward_references::BrotliEncoderParams>) {
    (*params.as_deref_mut().unwrap()).quality= brotli_min_int(
        11 as libc::c_int,
        brotli_max_int(0 as libc::c_int, (*params.as_deref().unwrap()).quality),
    );
    if (*params.as_deref().unwrap()).quality <= 2 as libc::c_int {
        (*params.as_deref_mut().unwrap()).large_window= 0 as libc::c_int;
    }
    if (*params.as_deref().unwrap()).lgwin < 10 as libc::c_int {
        (*params.as_deref_mut().unwrap()).lgwin= 10 as libc::c_int;
    } else {
        let mut max_lgwin = if (*params.as_deref().unwrap()).large_window != 0 {
            30 as libc::c_int
        } else {
            24 as libc::c_int
        };
        if (*params.as_deref().unwrap()).lgwin > max_lgwin {
            (*params.as_deref_mut().unwrap()).lgwin= max_lgwin;
        }
    };
}
#[inline(always)]
unsafe extern "C" fn HashBytesH40(mut data: *const uint8_t) -> size_t {
    let h = (BrotliUnalignedRead32(data as *const libc::c_void))
        .wrapping_mul(crate::src::enc::encode::kHashMul32);
    return (h >> 32 as libc::c_int - 15 as libc::c_int) as size_t;
}
#[inline(always)]
unsafe extern "C" fn MaxHashTableSize(mut quality: libc::c_int) -> size_t {
    return (if quality == 0 as libc::c_int {
        (1 as libc::c_int) << 15 as libc::c_int
    } else {
        (1 as libc::c_int) << 17 as libc::c_int
    }) as size_t;
}
#[inline(always)]
unsafe extern "C" fn ChooseHasher(
    mut params: *const crate::src::enc::backward_references::BrotliEncoderParams,
    mut hparams: *mut crate::src::enc::backward_references::BrotliHasherParams,
) {
    if (*params).quality > 9 as libc::c_int {
        (*hparams).type_0= 10 as libc::c_int;
    } else if (*params).quality == 4 as libc::c_int
        && (*params).size_hint
            >= ((1 as libc::c_int) << 20 as libc::c_int) as libc::c_ulong
    {
        (*hparams).type_0= 54 as libc::c_int;
    } else if (*params).quality < 5 as libc::c_int {
        (*hparams).type_0= (*params).quality;
    } else if (*params).lgwin <= 16 as libc::c_int {
        (*hparams).type_0= if (*params).quality < 7 as libc::c_int {
            40 as libc::c_int
        } else if (*params).quality < 9 as libc::c_int {
            41 as libc::c_int
        } else {
            42 as libc::c_int
        };
    } else if (*params).size_hint
        >= ((1 as libc::c_int) << 20 as libc::c_int) as libc::c_ulong
        && (*params).lgwin >= 19 as libc::c_int
    {
        (*hparams).type_0= 6 as libc::c_int;
        (*hparams).block_bits= (*params).quality - 1 as libc::c_int;
        (*hparams).bucket_bits= 15 as libc::c_int;
        (*hparams).hash_len= 5 as libc::c_int;
        (*hparams).num_last_distances_to_check= if (*params).quality < 7 as libc::c_int {
            4 as libc::c_int
        } else if (*params).quality < 9 as libc::c_int {
            10 as libc::c_int
        } else {
            16 as libc::c_int
        };
    } else {
        (*hparams).type_0= 5 as libc::c_int;
        (*hparams).block_bits= (*params).quality - 1 as libc::c_int;
        (*hparams).bucket_bits= if (*params).quality < 7 as libc::c_int {
            14 as libc::c_int
        } else {
            15 as libc::c_int
        };
        (*hparams).num_last_distances_to_check= if (*params).quality < 7 as libc::c_int {
            4 as libc::c_int
        } else if (*params).quality < 9 as libc::c_int {
            10 as libc::c_int
        } else {
            16 as libc::c_int
        };
    }
    if (*params).lgwin > 24 as libc::c_int {
        if (*hparams).type_0 == 3 as libc::c_int {
            (*hparams).type_0= 35 as libc::c_int;
        }
        if (*hparams).type_0 == 54 as libc::c_int {
            (*hparams).type_0= 55 as libc::c_int;
        }
        if (*hparams).type_0 == 6 as libc::c_int {
            (*hparams).type_0= 65 as libc::c_int;
        }
    }
}
#[inline(always)]
unsafe extern "C" fn HashMemAllocInBytesH2(
    mut params: *const crate::src::enc::backward_references::BrotliEncoderParams,
    mut one_shot: libc::c_int,
    mut input_size: size_t,
) -> size_t {
    return (::std::mem::size_of::<uint32_t>() as libc::c_ulong)
        .wrapping_mul(((1 as libc::c_int) << 16 as libc::c_int) as libc::c_ulong);
}
#[inline(always)]
unsafe extern "C" fn HashMemAllocInBytesH4(
    mut params: *const crate::src::enc::backward_references::BrotliEncoderParams,
    mut one_shot: libc::c_int,
    mut input_size: size_t,
) -> size_t {
    return (::std::mem::size_of::<uint32_t>() as libc::c_ulong)
        .wrapping_mul(((1 as libc::c_int) << 17 as libc::c_int) as libc::c_ulong);
}
#[inline(always)]
unsafe extern "C" fn HashMemAllocInBytesH5(
    mut params: *const crate::src::enc::backward_references::BrotliEncoderParams,
    mut one_shot: libc::c_int,
    mut input_size: size_t,
) -> size_t {
    let mut bucket_size = (1 as libc::c_int as size_t) << (*params).hasher.bucket_bits;
    let mut block_size = (1 as libc::c_int as size_t) << (*params).hasher.block_bits;
    return (::std::mem::size_of::<uint16_t>() as libc::c_ulong)
        .wrapping_mul(bucket_size)
        .wrapping_add(
            (::std::mem::size_of::<uint32_t>() as libc::c_ulong)
                .wrapping_mul(bucket_size)
                .wrapping_mul(block_size),
        );
}
#[inline(always)]
unsafe extern "C" fn HashMemAllocInBytesH40(
    mut params: *const crate::src::enc::backward_references::BrotliEncoderParams,
    mut one_shot: libc::c_int,
    mut input_size: size_t,
) -> size_t {
    return (::std::mem::size_of::<uint32_t>() as libc::c_ulong)
        .wrapping_mul(((1 as libc::c_int) << 15 as libc::c_int) as libc::c_ulong)
        .wrapping_add(
            (::std::mem::size_of::<uint16_t>() as libc::c_ulong)
                .wrapping_mul(((1 as libc::c_int) << 15 as libc::c_int) as libc::c_ulong),
        )
        .wrapping_add(
            (::std::mem::size_of::<uint8_t>() as libc::c_ulong)
                .wrapping_mul(65536 as libc::c_int as libc::c_ulong),
        )
        .wrapping_add(
            (::std::mem::size_of::<crate::src::enc::backward_references::BankH40>() as libc::c_ulong)
                .wrapping_mul(1 as libc::c_int as libc::c_ulong),
        );
}
#[inline(always)]
unsafe extern "C" fn HashMemAllocInBytesH41(
    mut params: *const crate::src::enc::backward_references::BrotliEncoderParams,
    mut one_shot: libc::c_int,
    mut input_size: size_t,
) -> size_t {
    return (::std::mem::size_of::<uint32_t>() as libc::c_ulong)
        .wrapping_mul(((1 as libc::c_int) << 15 as libc::c_int) as libc::c_ulong)
        .wrapping_add(
            (::std::mem::size_of::<uint16_t>() as libc::c_ulong)
                .wrapping_mul(((1 as libc::c_int) << 15 as libc::c_int) as libc::c_ulong),
        )
        .wrapping_add(
            (::std::mem::size_of::<uint8_t>() as libc::c_ulong)
                .wrapping_mul(65536 as libc::c_int as libc::c_ulong),
        )
        .wrapping_add(
            (::std::mem::size_of::<crate::src::enc::backward_references::BankH41>() as libc::c_ulong)
                .wrapping_mul(1 as libc::c_int as libc::c_ulong),
        );
}
#[inline(always)]
unsafe extern "C" fn HashMemAllocInBytesH42(
    mut params: *const crate::src::enc::backward_references::BrotliEncoderParams,
    mut one_shot: libc::c_int,
    mut input_size: size_t,
) -> size_t {
    return (::std::mem::size_of::<uint32_t>() as libc::c_ulong)
        .wrapping_mul(((1 as libc::c_int) << 15 as libc::c_int) as libc::c_ulong)
        .wrapping_add(
            (::std::mem::size_of::<uint16_t>() as libc::c_ulong)
                .wrapping_mul(((1 as libc::c_int) << 15 as libc::c_int) as libc::c_ulong),
        )
        .wrapping_add(
            (::std::mem::size_of::<uint8_t>() as libc::c_ulong)
                .wrapping_mul(65536 as libc::c_int as libc::c_ulong),
        )
        .wrapping_add(
            (::std::mem::size_of::<crate::src::enc::backward_references::BankH42>() as libc::c_ulong)
                .wrapping_mul(512 as libc::c_int as libc::c_ulong),
        );
}
#[inline(always)]
unsafe extern "C" fn HashMemAllocInBytesH35(
    mut params: *const crate::src::enc::backward_references::BrotliEncoderParams,
    mut one_shot: libc::c_int,
    mut input_size: size_t,
) -> size_t {
    return (HashMemAllocInBytesH3(params, one_shot, input_size))
        .wrapping_add(HashMemAllocInBytesHROLLING_FAST(params, one_shot, input_size));
}
#[inline(always)]
unsafe extern "C" fn HashMemAllocInBytesHROLLING_FAST(
    mut params: *const crate::src::enc::backward_references::BrotliEncoderParams,
    mut one_shot: libc::c_int,
    mut input_size: size_t,
) -> size_t {
    return (16777216 as libc::c_int as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<uint32_t>() as libc::c_ulong);
}
#[inline(always)]
unsafe extern "C" fn HashMemAllocInBytesH55(
    mut params: *const crate::src::enc::backward_references::BrotliEncoderParams,
    mut one_shot: libc::c_int,
    mut input_size: size_t,
) -> size_t {
    return (HashMemAllocInBytesH54(params, one_shot, input_size))
        .wrapping_add(HashMemAllocInBytesHROLLING_FAST(params, one_shot, input_size));
}
#[inline(always)]
unsafe extern "C" fn HashMemAllocInBytesHROLLING(
    mut params: *const crate::src::enc::backward_references::BrotliEncoderParams,
    mut one_shot: libc::c_int,
    mut input_size: size_t,
) -> size_t {
    return (16777216 as libc::c_int as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<uint32_t>() as libc::c_ulong);
}
#[inline(always)]
unsafe extern "C" fn HashMemAllocInBytesH65(
    mut params: *const crate::src::enc::backward_references::BrotliEncoderParams,
    mut one_shot: libc::c_int,
    mut input_size: size_t,
) -> size_t {
    return (HashMemAllocInBytesH6(params, one_shot, input_size))
        .wrapping_add(HashMemAllocInBytesHROLLING(params, one_shot, input_size));
}
#[inline(always)]
unsafe extern "C" fn HashMemAllocInBytesH10(
    mut params: *const crate::src::enc::backward_references::BrotliEncoderParams,
    mut one_shot: libc::c_int,
    mut input_size: size_t,
) -> size_t {
    let mut num_nodes = (1 as libc::c_int as size_t) << (*params).lgwin;
    if one_shot != 0 && input_size < num_nodes {
        num_nodes= input_size;
    }
    return (::std::mem::size_of::<uint32_t>() as libc::c_ulong)
        .wrapping_mul(((1 as libc::c_int) << 17 as libc::c_int) as libc::c_ulong)
        .wrapping_add(
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<uint32_t>() as libc::c_ulong)
                .wrapping_mul(num_nodes),
        );
}
#[inline(always)]
unsafe extern "C" fn HasherSize(
    mut params: *const crate::src::enc::backward_references::BrotliEncoderParams,
    mut one_shot: libc::c_int,
    mut input_size: size_t,
) -> size_t {
    match (*params).hasher.type_0 {
        2 => return HashMemAllocInBytesH2(params, one_shot, input_size),
        3 => return HashMemAllocInBytesH3(params, one_shot, input_size),
        4 => return HashMemAllocInBytesH4(params, one_shot, input_size),
        5 => return HashMemAllocInBytesH5(params, one_shot, input_size),
        6 => return HashMemAllocInBytesH6(params, one_shot, input_size),
        40 => return HashMemAllocInBytesH40(params, one_shot, input_size),
        41 => return HashMemAllocInBytesH41(params, one_shot, input_size),
        42 => return HashMemAllocInBytesH42(params, one_shot, input_size),
        54 => return HashMemAllocInBytesH54(params, one_shot, input_size),
        35 => return HashMemAllocInBytesH35(params, one_shot, input_size),
        55 => return HashMemAllocInBytesH55(params, one_shot, input_size),
        65 => return HashMemAllocInBytesH65(params, one_shot, input_size),
        10 => return HashMemAllocInBytesH10(params, one_shot, input_size),
        _ => {}
    }
    return 0 as libc::c_int as size_t;
}
unsafe extern "C" fn InitializeH2(
    mut common: *mut crate::src::enc::backward_references::HasherCommon,
    mut self_0: *mut crate::src::enc::backward_references::H2,
    mut params: *const crate::src::enc::backward_references::BrotliEncoderParams,
) {
    (*self_0).common= common;
    (*self_0).buckets_= (*common).extra as *mut uint32_t;
}
unsafe extern "C" fn InitializeH4(
    mut common: *mut crate::src::enc::backward_references::HasherCommon,
    mut self_0: *mut crate::src::enc::backward_references::H4,
    mut params: *const crate::src::enc::backward_references::BrotliEncoderParams,
) {
    (*self_0).common= common;
    (*self_0).buckets_= (*common).extra as *mut uint32_t;
}
unsafe extern "C" fn InitializeH5(
    mut common: *mut crate::src::enc::backward_references::HasherCommon,
    mut self_0: *mut crate::src::enc::backward_references::H5,
    mut params: *const crate::src::enc::backward_references::BrotliEncoderParams,
) {
    (*self_0).common_= common;
    (*self_0).hash_shift_= 32 as libc::c_int - (*common).params.bucket_bits;
    (*self_0).bucket_size_= (1 as libc::c_int as size_t) << (*common).params.bucket_bits;
    (*self_0).block_size_= (1 as libc::c_int as size_t) << (*common).params.block_bits;
    (*self_0).block_mask_= (*self_0).block_size_
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t;
    (*self_0).num_= (*common).extra as *mut uint16_t;
    (*self_0).buckets_= core::ptr::addr_of_mut!(*(*self_0).num_.offset((*self_0).bucket_size_ as isize))
        as *mut uint16_t as *mut uint32_t;
    (*self_0).block_bits_= (*common).params.block_bits;
    (*self_0).num_last_distances_to_check_= (*common).params.num_last_distances_to_check;
}
unsafe extern "C" fn InitializeH40(
    mut common: *mut crate::src::enc::backward_references::HasherCommon,
    mut self_0: *mut crate::src::enc::backward_references::H40,
    mut params: *const crate::src::enc::backward_references::BrotliEncoderParams,
) {
    (*self_0).common= common;
    (*self_0).extra= (*common).extra;
    (*self_0).max_hops= ((if (*params).quality > 6 as libc::c_int {
        7 as libc::c_uint
    } else {
        8 as libc::c_uint
    }) << (*params).quality - 4 as libc::c_int) as size_t;
}
unsafe extern "C" fn InitializeH41(
    mut common: *mut crate::src::enc::backward_references::HasherCommon,
    mut self_0: *mut crate::src::enc::backward_references::H41,
    mut params: *const crate::src::enc::backward_references::BrotliEncoderParams,
) {
    (*self_0).common= common;
    (*self_0).extra= (*common).extra;
    (*self_0).max_hops= ((if (*params).quality > 6 as libc::c_int {
        7 as libc::c_uint
    } else {
        8 as libc::c_uint
    }) << (*params).quality - 4 as libc::c_int) as size_t;
}
unsafe extern "C" fn InitializeH42(
    mut common: *mut crate::src::enc::backward_references::HasherCommon,
    mut self_0: *mut crate::src::enc::backward_references::H42,
    mut params: *const crate::src::enc::backward_references::BrotliEncoderParams,
) {
    (*self_0).common= common;
    (*self_0).extra= (*common).extra;
    (*self_0).max_hops= ((if (*params).quality > 6 as libc::c_int {
        7 as libc::c_uint
    } else {
        8 as libc::c_uint
    }) << (*params).quality - 4 as libc::c_int) as size_t;
}
unsafe extern "C" fn InitializeH35(
    mut common: *mut crate::src::enc::backward_references::HasherCommon,
    mut self_0: *mut crate::src::enc::backward_references::H35,
    mut params: *const crate::src::enc::backward_references::BrotliEncoderParams,
) {
    (*self_0).common= common;
    (*self_0).extra= (*common).extra;
    (*self_0).hb_common= (*(*self_0).common);
    (*self_0).fresh= 1 as libc::c_int;
    (*self_0).params= params;
}
unsafe extern "C" fn InitializeH55(
    mut common: *mut crate::src::enc::backward_references::HasherCommon,
    mut self_0: *mut crate::src::enc::backward_references::H55,
    mut params: *const crate::src::enc::backward_references::BrotliEncoderParams,
) {
    (*self_0).common= common;
    (*self_0).extra= (*common).extra;
    (*self_0).hb_common= (*(*self_0).common);
    (*self_0).fresh= 1 as libc::c_int;
    (*self_0).params= params;
}
unsafe extern "C" fn InitializeH65(
    mut common: *mut crate::src::enc::backward_references::HasherCommon,
    mut self_0: *mut crate::src::enc::backward_references::H65,
    mut params: *const crate::src::enc::backward_references::BrotliEncoderParams,
) {
    (*self_0).common= common;
    (*self_0).extra= (*common).extra;
    (*self_0).hb_common= (*(*self_0).common);
    (*self_0).fresh= 1 as libc::c_int;
    (*self_0).params= params;
}
unsafe extern "C" fn InitializeH10(
    mut common: *mut crate::src::enc::backward_references::HasherCommon,
    mut self_0: *mut crate::src::enc::backward_references::H10,
    mut params: *const crate::src::enc::backward_references::BrotliEncoderParams,
) {
    (*self_0).buckets_= (*common).extra as *mut uint32_t;
    (*self_0).forest_= core::ptr::addr_of_mut!(*(*self_0).buckets_
        .offset(((1 as libc::c_int) << 17 as libc::c_int) as isize)) as *mut uint32_t;
    (*self_0).window_mask_= ((1 as libc::c_uint) << (*params).lgwin)
        .wrapping_sub(1 as libc::c_uint) as size_t;
    (*self_0).invalid_pos_= (0 as libc::c_int as libc::c_ulong)
        .wrapping_sub((*self_0).window_mask_) as uint32_t;
}
unsafe extern "C" fn PrepareH2(
    mut self_0: *mut crate::src::enc::backward_references::H2,
    mut one_shot: libc::c_int,
    mut input_size: size_t,
    mut data: *const uint8_t,
) {
    let mut buckets = (*self_0).buckets_;
    let mut partial_prepare_threshold = ((1 as libc::c_int) << 16 as libc::c_int
        >> 5 as libc::c_int) as size_t;
    if one_shot != 0 && input_size <= partial_prepare_threshold {
        let mut i: size_t = 0;
        i= 0 as libc::c_int as size_t;
        while i < input_size {
            let key = HashBytesH2(&*data.offset(i as isize));
            if (1 as libc::c_int) << 0 as libc::c_int == 1 as libc::c_int {
                *buckets.offset(key as isize) = 0 as libc::c_int as uint32_t;
            } else {
                let mut j: uint32_t = 0;
                j = 0 as libc::c_int as uint32_t;
                while j < ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint {
                    *buckets
                        .offset(
                            (key.wrapping_add(j << 3 as libc::c_int)
                                & (((1 as libc::c_int) << 16 as libc::c_int)
                                    - 1 as libc::c_int) as libc::c_uint) as isize,
                        ) = 0 as libc::c_int as uint32_t;
                    j = j.wrapping_add(1);
                }
            }
            i= i.wrapping_add(1);
        }
    } else {
        memset(
            buckets as *mut libc::c_void,
            0 as libc::c_int,
            (::std::mem::size_of::<uint32_t>() as libc::c_ulong)
                .wrapping_mul(((1 as libc::c_int) << 16 as libc::c_int) as libc::c_ulong),
        );
    };
}
unsafe extern "C" fn PrepareH4(
    mut self_0: *mut crate::src::enc::backward_references::H4,
    mut one_shot: libc::c_int,
    mut input_size: size_t,
    mut data: *const uint8_t,
) {
    let mut buckets = (*self_0).buckets_;
    let mut partial_prepare_threshold = ((1 as libc::c_int) << 17 as libc::c_int
        >> 5 as libc::c_int) as size_t;
    if one_shot != 0 && input_size <= partial_prepare_threshold {
        let mut i: size_t = 0;
        i= 0 as libc::c_int as size_t;
        while i < input_size {
            let key = HashBytesH4(&*data.offset(i as isize));
            if (1 as libc::c_int) << 2 as libc::c_int == 1 as libc::c_int {
                *buckets.offset(key as isize) = 0 as libc::c_int as uint32_t;
            } else {
                let mut j: uint32_t = 0;
                j= 0 as libc::c_int as uint32_t;
                while j < ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint {
                    *buckets
                        .offset(
                            (key.wrapping_add(j << 3 as libc::c_int)
                                & (((1 as libc::c_int) << 17 as libc::c_int)
                                    - 1 as libc::c_int) as libc::c_uint) as isize,
                        ) = 0 as libc::c_int as uint32_t;
                    j= j.wrapping_add(1);
                }
            }
            i= i.wrapping_add(1);
        }
    } else {
        memset(
            buckets as *mut libc::c_void,
            0 as libc::c_int,
            (::std::mem::size_of::<uint32_t>() as libc::c_ulong)
                .wrapping_mul(((1 as libc::c_int) << 17 as libc::c_int) as libc::c_ulong),
        );
    };
}
unsafe extern "C" fn PrepareH5(
    mut self_0: *mut crate::src::enc::backward_references::H5,
    mut one_shot: libc::c_int,
    mut input_size: size_t,
    mut data: *const uint8_t,
) {
    let mut num = (*self_0).num_;
    let mut partial_prepare_threshold = (*self_0).bucket_size_ >> 6 as libc::c_int;
    if one_shot != 0 && input_size <= partial_prepare_threshold {
        let mut i: size_t = 0;
        i= 0 as libc::c_int as size_t;
        while i < input_size {
            let key = HashBytesH5(&*data.offset(i as isize), (*self_0).hash_shift_);
            *num.offset(key as isize) = 0 as libc::c_int as uint16_t;
            i= i.wrapping_add(1);
        }
    } else {
        memset(
            num as *mut libc::c_void,
            0 as libc::c_int,
            (*self_0).bucket_size_
                .wrapping_mul(::std::mem::size_of::<uint16_t>() as libc::c_ulong),
        );
    };
}
unsafe extern "C" fn PrepareH40(
    mut self_0: *mut crate::src::enc::backward_references::H40,
    mut one_shot: libc::c_int,
    mut input_size: size_t,
    mut data: *const uint8_t,
) {
    let mut addr = AddrH40((*self_0).extra);
    let mut head = HeadH40((*self_0).extra);
    let mut tiny_hash = TinyHashH40((*self_0).extra);
    let mut partial_prepare_threshold = ((1 as libc::c_int) << 15 as libc::c_int
        >> 6 as libc::c_int) as size_t;
    if one_shot != 0 && input_size <= partial_prepare_threshold {
        let mut i: size_t = 0;
        i= 0 as libc::c_int as size_t;
        while i < input_size {
            let mut bucket = HashBytesH40(&*data.offset(i as isize));
            *addr.offset(bucket as isize) = 0xcccccccc as libc::c_uint;
            *head.offset(bucket as isize) = 0xcccc as libc::c_int as uint16_t;
            i= i.wrapping_add(1);
        }
    } else {
        memset(
            addr as *mut libc::c_void,
            0xcc as libc::c_int,
            (::std::mem::size_of::<uint32_t>() as libc::c_ulong)
                .wrapping_mul(((1 as libc::c_int) << 15 as libc::c_int) as libc::c_ulong),
        );
        memset(
            head as *mut libc::c_void,
            0 as libc::c_int,
            (::std::mem::size_of::<uint16_t>() as libc::c_ulong)
                .wrapping_mul(((1 as libc::c_int) << 15 as libc::c_int) as libc::c_ulong),
        );
    }
    memset(
        tiny_hash as *mut libc::c_void,
        0 as libc::c_int,
        (::std::mem::size_of::<uint8_t>() as libc::c_ulong)
            .wrapping_mul(65536 as libc::c_int as libc::c_ulong),
    );
    memset(
        (*self_0).free_slot_idx.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[uint16_t; 1]>() as libc::c_ulong,
    );
}
unsafe extern "C" fn PrepareH41(
    mut self_0: *mut crate::src::enc::backward_references::H41,
    mut one_shot: libc::c_int,
    mut input_size: size_t,
    mut data: *const uint8_t,
) {
    let mut addr = AddrH41((*self_0).extra);
    let mut head = HeadH41((*self_0).extra);
    let mut tiny_hash = TinyHashH41((*self_0).extra);
    let mut partial_prepare_threshold = ((1 as libc::c_int) << 15 as libc::c_int
        >> 6 as libc::c_int) as size_t;
    if one_shot != 0 && input_size <= partial_prepare_threshold {
        let mut i: size_t = 0;
        i= 0 as libc::c_int as size_t;
        while i < input_size {
            let mut bucket = HashBytesH41(&*data.offset(i as isize));
            *addr.offset(bucket as isize) = 0xcccccccc as libc::c_uint;
            *head.offset(bucket as isize) = 0xcccc as libc::c_int as uint16_t;
            i= i.wrapping_add(1);
        }
    } else {
        memset(
            addr as *mut libc::c_void,
            0xcc as libc::c_int,
            (::std::mem::size_of::<uint32_t>() as libc::c_ulong)
                .wrapping_mul(((1 as libc::c_int) << 15 as libc::c_int) as libc::c_ulong),
        );
        memset(
            head as *mut libc::c_void,
            0 as libc::c_int,
            (::std::mem::size_of::<uint16_t>() as libc::c_ulong)
                .wrapping_mul(((1 as libc::c_int) << 15 as libc::c_int) as libc::c_ulong),
        );
    }
    memset(
        tiny_hash as *mut libc::c_void,
        0 as libc::c_int,
        (::std::mem::size_of::<uint8_t>() as libc::c_ulong)
            .wrapping_mul(65536 as libc::c_int as libc::c_ulong),
    );
    memset(
        (*self_0).free_slot_idx.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[uint16_t; 1]>() as libc::c_ulong,
    );
}
unsafe extern "C" fn PrepareH42(
    mut self_0: *mut crate::src::enc::backward_references::H42,
    mut one_shot: libc::c_int,
    mut input_size: size_t,
    mut data: *const uint8_t,
) {
    let mut addr = AddrH42((*self_0).extra);
    let mut head = HeadH42((*self_0).extra);
    let mut tiny_hash = TinyHashH42((*self_0).extra);
    let mut partial_prepare_threshold = ((1 as libc::c_int) << 15 as libc::c_int
        >> 6 as libc::c_int) as size_t;
    if one_shot != 0 && input_size <= partial_prepare_threshold {
        let mut i: size_t = 0;
        i= 0 as libc::c_int as size_t;
        while i < input_size {
            let mut bucket = HashBytesH42(&*data.offset(i as isize));
            *addr.offset(bucket as isize) = 0xcccccccc as libc::c_uint;
            *head.offset(bucket as isize) = 0xcccc as libc::c_int as uint16_t;
            i= i.wrapping_add(1);
        }
    } else {
        memset(
            addr as *mut libc::c_void,
            0xcc as libc::c_int,
            (::std::mem::size_of::<uint32_t>() as libc::c_ulong)
                .wrapping_mul(((1 as libc::c_int) << 15 as libc::c_int) as libc::c_ulong),
        );
        memset(
            head as *mut libc::c_void,
            0 as libc::c_int,
            (::std::mem::size_of::<uint16_t>() as libc::c_ulong)
                .wrapping_mul(((1 as libc::c_int) << 15 as libc::c_int) as libc::c_ulong),
        );
    }
    memset(
        tiny_hash as *mut libc::c_void,
        0 as libc::c_int,
        (::std::mem::size_of::<uint8_t>() as libc::c_ulong)
            .wrapping_mul(65536 as libc::c_int as libc::c_ulong),
    );
    memset(
        (*self_0).free_slot_idx.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[uint16_t; 512]>() as libc::c_ulong,
    );
}
#[inline(always)]
unsafe extern "C" fn HashMemAllocInBytesH3(
    mut params: *const crate::src::enc::backward_references::BrotliEncoderParams,
    mut one_shot: libc::c_int,
    mut input_size: size_t,
) -> size_t {
    return (::std::mem::size_of::<uint32_t>() as libc::c_ulong)
        .wrapping_mul(((1 as libc::c_int) << 16 as libc::c_int) as libc::c_ulong);
}
unsafe extern "C" fn InitializeH3(
    mut common: *mut crate::src::enc::backward_references::HasherCommon,
    mut self_0: *mut crate::src::enc::backward_references::H3,
    mut params: *const crate::src::enc::backward_references::BrotliEncoderParams,
) {
    (*self_0).common= common;
    (*self_0).buckets_= (*common).extra as *mut uint32_t;
}
unsafe extern "C" fn PrepareH3(
    mut self_0: *mut crate::src::enc::backward_references::H3,
    mut one_shot: libc::c_int,
    mut input_size: size_t,
    mut data: *const uint8_t,
) {
    let mut buckets = (*self_0).buckets_;
    let mut partial_prepare_threshold = ((1 as libc::c_int) << 16 as libc::c_int
        >> 5 as libc::c_int) as size_t;
    if one_shot != 0 && input_size <= partial_prepare_threshold {
        let mut i: size_t = 0;
        i= 0 as libc::c_int as size_t;
        while i < input_size {
            let key = HashBytesH3(&*data.offset(i as isize));
            if (1 as libc::c_int) << 1 as libc::c_int == 1 as libc::c_int {
                *buckets.offset(key as isize) = 0 as libc::c_int as uint32_t;
            } else {
                let mut j: uint32_t = 0;
                j= 0 as libc::c_int as uint32_t;
                while j < ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint {
                    *buckets
                        .offset(
                            (key.wrapping_add(j << 3 as libc::c_int)
                                & (((1 as libc::c_int) << 16 as libc::c_int)
                                    - 1 as libc::c_int) as libc::c_uint) as isize,
                        ) = 0 as libc::c_int as uint32_t;
                    j= j.wrapping_add(1);
                }
            }
            i= i.wrapping_add(1);
        }
    } else {
        memset(
            buckets as *mut libc::c_void,
            0 as libc::c_int,
            (::std::mem::size_of::<uint32_t>() as libc::c_ulong)
                .wrapping_mul(((1 as libc::c_int) << 16 as libc::c_int) as libc::c_ulong),
        );
    };
}
unsafe extern "C" fn PrepareH35(
    mut self_0: *mut crate::src::enc::backward_references::H35,
    mut one_shot: libc::c_int,
    mut input_size: size_t,
    mut data: *const uint8_t,
) {
    if (*self_0).fresh != 0 {
        (*self_0).fresh= 0 as libc::c_int;
        (*self_0).hb_common.extra= ((*self_0).extra as *mut uint8_t)
            .offset(
                HashMemAllocInBytesH3((*self_0).params, one_shot, input_size) as isize,
            ) as *mut libc::c_void;
        InitializeH3((*self_0).common, core::ptr::addr_of_mut!((*self_0).ha), (*self_0).params);
        InitializeHROLLING_FAST(
            core::ptr::addr_of_mut!((*self_0).hb_common),
            core::ptr::addr_of_mut!((*self_0).hb),
            (*self_0).params,
        );
    }
    PrepareH3(core::ptr::addr_of_mut!((*self_0).ha), one_shot, input_size, data);
    PrepareHROLLING_FAST(core::ptr::addr_of_mut!((*self_0).hb), one_shot, input_size, data);
}
#[inline(always)]
unsafe extern "C" fn HashMemAllocInBytesH54(
    mut params: *const crate::src::enc::backward_references::BrotliEncoderParams,
    mut one_shot: libc::c_int,
    mut input_size: size_t,
) -> size_t {
    return (::std::mem::size_of::<uint32_t>() as libc::c_ulong)
        .wrapping_mul(((1 as libc::c_int) << 20 as libc::c_int) as libc::c_ulong);
}
unsafe extern "C" fn InitializeH54(
    mut common: *mut crate::src::enc::backward_references::HasherCommon,
    mut self_0: *mut crate::src::enc::backward_references::H54,
    mut params: *const crate::src::enc::backward_references::BrotliEncoderParams,
) {
    (*self_0).common= common;
    (*self_0).buckets_= (*common).extra as *mut uint32_t;
}
static mut kRollingHashMul32HROLLING_FAST: uint32_t = 69069 as libc::c_int as uint32_t;
static mut kInvalidPosHROLLING_FAST: uint32_t = 0xffffffff as libc::c_uint;
unsafe extern "C" fn InitializeHROLLING_FAST(
    mut common: *mut crate::src::enc::backward_references::HasherCommon,
    mut self_0: *mut crate::src::enc::backward_references::HROLLING_FAST,
    mut params: *const crate::src::enc::backward_references::BrotliEncoderParams,
) {
    let mut i: size_t = 0;
    (*self_0).state= 0 as libc::c_int as uint32_t;
    (*self_0).next_ix= 0 as libc::c_int as size_t;
    (*self_0).factor= crate::src::enc::encode::kRollingHashMul32HROLLING_FAST;
    (*self_0).factor_remove= 1 as libc::c_int as uint32_t;
    i= 0 as libc::c_int as size_t;
    while i < 32 as libc::c_int as libc::c_ulong {
        (*self_0).factor_remove= ((*self_0).factor_remove as libc::c_uint).wrapping_mul((*self_0).factor) as uint32_t
            as uint32_t;
        i= (i as libc::c_ulong).wrapping_add(4 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
    }
    (*self_0).table= (*common).extra as *mut uint32_t;
    i= 0 as libc::c_int as size_t;
    while i < 16777216 as libc::c_int as libc::c_ulong {
        *(*self_0).table.offset(i as isize) = crate::src::enc::encode::kInvalidPosHROLLING_FAST;
        i= i.wrapping_add(1);
    }
}
unsafe extern "C" fn PrepareH54(
    mut self_0: *mut crate::src::enc::backward_references::H54,
    mut one_shot: libc::c_int,
    mut input_size: size_t,
    mut data: *const uint8_t,
) {
    let mut buckets = (*self_0).buckets_;
    let mut partial_prepare_threshold = ((1 as libc::c_int) << 20 as libc::c_int
        >> 5 as libc::c_int) as size_t;
    if one_shot != 0 && input_size <= partial_prepare_threshold {
        let mut i: size_t = 0;
        i= 0 as libc::c_int as size_t;
        while i < input_size {
            let key = HashBytesH54(&*data.offset(i as isize));
            if (1 as libc::c_int) << 2 as libc::c_int == 1 as libc::c_int {
                *buckets.offset(key as isize) = 0 as libc::c_int as uint32_t;
            } else {
                let mut j: uint32_t = 0;
                j= 0 as libc::c_int as uint32_t;
                while j < ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint {
                    *buckets
                        .offset(
                            (key.wrapping_add(j << 3 as libc::c_int)
                                & (((1 as libc::c_int) << 20 as libc::c_int)
                                    - 1 as libc::c_int) as libc::c_uint) as isize,
                        ) = 0 as libc::c_int as uint32_t;
                    j= j.wrapping_add(1);
                }
            }
            i= i.wrapping_add(1);
        }
    } else {
        memset(
            buckets as *mut libc::c_void,
            0 as libc::c_int,
            (::std::mem::size_of::<uint32_t>() as libc::c_ulong)
                .wrapping_mul(((1 as libc::c_int) << 20 as libc::c_int) as libc::c_ulong),
        );
    };
}
unsafe extern "C" fn PrepareH55(
    mut self_0: *mut crate::src::enc::backward_references::H55,
    mut one_shot: libc::c_int,
    mut input_size: size_t,
    mut data: *const uint8_t,
) {
    if (*self_0).fresh != 0 {
        (*self_0).fresh= 0 as libc::c_int;
        (*self_0).hb_common.extra= ((*self_0).extra as *mut uint8_t)
            .offset(
                HashMemAllocInBytesH54((*self_0).params, one_shot, input_size) as isize,
            ) as *mut libc::c_void;
        InitializeH54((*self_0).common, core::ptr::addr_of_mut!((*self_0).ha), (*self_0).params);
        InitializeHROLLING_FAST(
            core::ptr::addr_of_mut!((*self_0).hb_common),
            core::ptr::addr_of_mut!((*self_0).hb),
            (*self_0).params,
        );
    }
    PrepareH54(core::ptr::addr_of_mut!((*self_0).ha), one_shot, input_size, data);
    PrepareHROLLING_FAST(core::ptr::addr_of_mut!((*self_0).hb), one_shot, input_size, data);
}
#[inline(always)]
unsafe extern "C" fn HashMemAllocInBytesH6(
    mut params: *const crate::src::enc::backward_references::BrotliEncoderParams,
    mut one_shot: libc::c_int,
    mut input_size: size_t,
) -> size_t {
    let mut bucket_size = (1 as libc::c_int as size_t) << (*params).hasher.bucket_bits;
    let mut block_size = (1 as libc::c_int as size_t) << (*params).hasher.block_bits;
    return (::std::mem::size_of::<uint16_t>() as libc::c_ulong)
        .wrapping_mul(bucket_size)
        .wrapping_add(
            (::std::mem::size_of::<uint32_t>() as libc::c_ulong)
                .wrapping_mul(bucket_size)
                .wrapping_mul(block_size),
        );
}
unsafe extern "C" fn InitializeH6(
    mut common: *mut crate::src::enc::backward_references::HasherCommon,
    mut self_0: *mut crate::src::enc::backward_references::H6,
    mut params: *const crate::src::enc::backward_references::BrotliEncoderParams,
) {
    (*self_0).common_= common;
    (*self_0).hash_shift_= 64 as libc::c_int - (*common).params.bucket_bits;
    (*self_0).hash_mask_= !(0 as libc::c_uint as uint64_t)
        >> 64 as libc::c_int - 8 as libc::c_int * (*common).params.hash_len;
    (*self_0).bucket_size_= (1 as libc::c_int as size_t) << (*common).params.bucket_bits;
    (*self_0).block_bits_= (*common).params.block_bits;
    (*self_0).block_size_= (1 as libc::c_int as size_t) << (*common).params.block_bits;
    (*self_0).block_mask_= (*self_0).block_size_
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t;
    (*self_0).num_last_distances_to_check_= (*common).params.num_last_distances_to_check;
    (*self_0).num_= (*common).extra as *mut uint16_t;
    (*self_0).buckets_= core::ptr::addr_of_mut!(*(*self_0).num_.offset((*self_0).bucket_size_ as isize))
        as *mut uint16_t as *mut uint32_t;
}
static mut kRollingHashMul32HROLLING: uint32_t = 69069 as libc::c_int as uint32_t;
static mut kInvalidPosHROLLING: uint32_t = 0xffffffff as libc::c_uint;
unsafe extern "C" fn InitializeHROLLING(
    mut common: *mut crate::src::enc::backward_references::HasherCommon,
    mut self_0: *mut crate::src::enc::backward_references::HROLLING,
    mut params: *const crate::src::enc::backward_references::BrotliEncoderParams,
) {
    let mut i: size_t = 0;
    (*self_0).state= 0 as libc::c_int as uint32_t;
    (*self_0).next_ix= 0 as libc::c_int as size_t;
    (*self_0).factor= crate::src::enc::encode::kRollingHashMul32HROLLING;
    (*self_0).factor_remove= 1 as libc::c_int as uint32_t;
    i= 0 as libc::c_int as size_t;
    while i < 32 as libc::c_int as libc::c_ulong {
        (*self_0).factor_remove= ((*self_0).factor_remove as libc::c_uint).wrapping_mul((*self_0).factor) as uint32_t
            as uint32_t;
        i= (i as libc::c_ulong).wrapping_add(1 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
    }
    (*self_0).table= (*common).extra as *mut uint32_t;
    i= 0 as libc::c_int as size_t;
    while i < 16777216 as libc::c_int as libc::c_ulong {
        *(*self_0).table.offset(i as isize) = crate::src::enc::encode::kInvalidPosHROLLING;
        i= i.wrapping_add(1);
    }
}
unsafe extern "C" fn PrepareH6(
    mut self_0: *mut crate::src::enc::backward_references::H6,
    mut one_shot: libc::c_int,
    mut input_size: size_t,
    mut data: *const uint8_t,
) {
    let mut num = (*self_0).num_;
    let mut partial_prepare_threshold = (*self_0).bucket_size_ >> 6 as libc::c_int;
    if one_shot != 0 && input_size <= partial_prepare_threshold {
        let mut i: size_t = 0;
        i= 0 as libc::c_int as size_t;
        while i < input_size {
            let key = HashBytesH6(
                &*data.offset(i as isize),
                (*self_0).hash_mask_,
                (*self_0).hash_shift_,
            );
            *num.offset(key as isize) = 0 as libc::c_int as uint16_t;
            i= i.wrapping_add(1);
        }
    } else {
        memset(
            num as *mut libc::c_void,
            0 as libc::c_int,
            (*self_0).bucket_size_
                .wrapping_mul(::std::mem::size_of::<uint16_t>() as libc::c_ulong),
        );
    };
}
unsafe extern "C" fn PrepareH65(
    mut self_0: *mut crate::src::enc::backward_references::H65,
    mut one_shot: libc::c_int,
    mut input_size: size_t,
    mut data: *const uint8_t,
) {
    if (*self_0).fresh != 0 {
        (*self_0).fresh= 0 as libc::c_int;
        (*self_0).hb_common.extra= ((*self_0).extra as *mut uint8_t)
            .offset(
                HashMemAllocInBytesH6((*self_0).params, one_shot, input_size) as isize,
            ) as *mut libc::c_void;
        InitializeH6((*self_0).common, core::ptr::addr_of_mut!((*self_0).ha), (*self_0).params);
        InitializeHROLLING(
            core::ptr::addr_of_mut!((*self_0).hb_common),
            core::ptr::addr_of_mut!((*self_0).hb),
            (*self_0).params,
        );
    }
    PrepareH6(core::ptr::addr_of_mut!((*self_0).ha), one_shot, input_size, data);
    PrepareHROLLING(core::ptr::addr_of_mut!((*self_0).hb), one_shot, input_size, data);
}
unsafe extern "C" fn PrepareH10(
    mut self_0: *mut crate::src::enc::backward_references::H10,
    mut one_shot: libc::c_int,
    mut input_size: size_t,
    mut data: *const uint8_t,
) {
    let mut invalid_pos = (*self_0).invalid_pos_;
    let mut i: uint32_t = 0;
    let mut buckets = (*self_0).buckets_;
    i= 0 as libc::c_int as uint32_t;
    while i < ((1 as libc::c_int) << 17 as libc::c_int) as libc::c_uint {
        *buckets.offset(i as isize) = invalid_pos;
        i= i.wrapping_add(1);
    }
}
#[inline(always)]
unsafe extern "C" fn HasherSetup(
    mut m: *mut crate::src::enc::backward_references_hq::MemoryManager,
    mut hasher: *mut crate::src::enc::backward_references::Hasher,
    mut params: *mut crate::src::enc::backward_references::BrotliEncoderParams,
    mut data: *const uint8_t,
    mut position: size_t,
    mut input_size: size_t,
    mut is_last: libc::c_int,
) {
    let mut one_shot = (position == 0 as libc::c_int as libc::c_ulong && is_last != 0)
        as libc::c_int;
    if (*hasher).common.extra.is_null() {();
        let mut alloc_size: size_t = 0;
        ChooseHasher(params, core::ptr::addr_of_mut!((*params).hasher));
        alloc_size= HasherSize(params, one_shot, input_size);
        (*hasher).common.extra= (if alloc_size > 0 as libc::c_int as libc::c_ulong {
            crate::src::enc::memory::BrotliAllocate(
                m,
                alloc_size
                    .wrapping_mul(::std::mem::size_of::<uint8_t>() as libc::c_ulong),
            ) as *mut uint8_t
        } else {
            0 as *mut uint8_t
        }) as *mut libc::c_void;
        if 0 as libc::c_int != 0 || 0 as libc::c_int != 0 {
            return;
        }
        (*hasher).common.params= (*params).hasher;
        match (*hasher).common.params.type_0 {
            2 => {
                InitializeH2(core::ptr::addr_of_mut!((*hasher).common), core::ptr::addr_of_mut!((*hasher).privat._H2), params);
            }
            3 => {
                InitializeH3(core::ptr::addr_of_mut!((*hasher).common), core::ptr::addr_of_mut!((*hasher).privat._H3), params);
            }
            4 => {
                InitializeH4(core::ptr::addr_of_mut!((*hasher).common), core::ptr::addr_of_mut!((*hasher).privat._H4), params);
            }
            5 => {
                InitializeH5(core::ptr::addr_of_mut!((*hasher).common), core::ptr::addr_of_mut!((*hasher).privat._H5), params);
            }
            6 => {
                InitializeH6(core::ptr::addr_of_mut!((*hasher).common), core::ptr::addr_of_mut!((*hasher).privat._H6), params);
            }
            40 => {
                InitializeH40(core::ptr::addr_of_mut!((*hasher).common), core::ptr::addr_of_mut!((*hasher).privat._H40), params);
            }
            41 => {
                InitializeH41(core::ptr::addr_of_mut!((*hasher).common), core::ptr::addr_of_mut!((*hasher).privat._H41), params);
            }
            42 => {
                InitializeH42(core::ptr::addr_of_mut!((*hasher).common), core::ptr::addr_of_mut!((*hasher).privat._H42), params);
            }
            54 => {
                InitializeH54(core::ptr::addr_of_mut!((*hasher).common), core::ptr::addr_of_mut!((*hasher).privat._H54), params);
            }
            35 => {
                InitializeH35(core::ptr::addr_of_mut!((*hasher).common), core::ptr::addr_of_mut!((*hasher).privat._H35), params);
            }
            55 => {
                InitializeH55(core::ptr::addr_of_mut!((*hasher).common), core::ptr::addr_of_mut!((*hasher).privat._H55), params);
            }
            65 => {
                InitializeH65(core::ptr::addr_of_mut!((*hasher).common), core::ptr::addr_of_mut!((*hasher).privat._H65), params);
            }
            10 => {
                InitializeH10(core::ptr::addr_of_mut!((*hasher).common), core::ptr::addr_of_mut!((*hasher).privat._H10), params);
            }
            _ => {}
        }
        HasherReset(hasher.as_mut());
    }
    if (*hasher).common.is_prepared_ == 0 {
        match (*hasher).common.params.type_0 {
            2 => {
                PrepareH2(core::ptr::addr_of_mut!((*hasher).privat._H2), one_shot, input_size, data);
            }
            3 => {
                PrepareH3(core::ptr::addr_of_mut!((*hasher).privat._H3), one_shot, input_size, data);
            }
            4 => {
                PrepareH4(core::ptr::addr_of_mut!((*hasher).privat._H4), one_shot, input_size, data);
            }
            5 => {
                PrepareH5(core::ptr::addr_of_mut!((*hasher).privat._H5), one_shot, input_size, data);
            }
            6 => {
                PrepareH6(core::ptr::addr_of_mut!((*hasher).privat._H6), one_shot, input_size, data);
            }
            40 => {
                PrepareH40(core::ptr::addr_of_mut!((*hasher).privat._H40), one_shot, input_size, data);
            }
            41 => {
                PrepareH41(core::ptr::addr_of_mut!((*hasher).privat._H41), one_shot, input_size, data);
            }
            42 => {
                PrepareH42(core::ptr::addr_of_mut!((*hasher).privat._H42), one_shot, input_size, data);
            }
            54 => {
                PrepareH54(core::ptr::addr_of_mut!((*hasher).privat._H54), one_shot, input_size, data);
            }
            35 => {
                PrepareH35(core::ptr::addr_of_mut!((*hasher).privat._H35), one_shot, input_size, data);
            }
            55 => {
                PrepareH55(core::ptr::addr_of_mut!((*hasher).privat._H55), one_shot, input_size, data);
            }
            65 => {
                PrepareH65(core::ptr::addr_of_mut!((*hasher).privat._H65), one_shot, input_size, data);
            }
            10 => {
                PrepareH10(core::ptr::addr_of_mut!((*hasher).privat._H10), one_shot, input_size, data);
            }
            _ => {}
        }
        if position == 0 as libc::c_int as libc::c_ulong {
            (*hasher).common.dict_num_lookups= 0 as libc::c_int as size_t;
            (*hasher).common.dict_num_matches= 0 as libc::c_int as size_t;
        }
        (*hasher).common.is_prepared_= 1 as libc::c_int;
    }
}
#[inline(always)]
unsafe extern "C" fn HashTypeLengthH2() -> size_t {
    return 8 as libc::c_int as size_t;
}
unsafe extern "C" fn HashBytesH2(mut data: *const uint8_t) -> uint32_t {
    let h = (BrotliUnalignedRead64(data as *const libc::c_void)
        << 64 as libc::c_int - 8 as libc::c_int * 5 as libc::c_int)
        .wrapping_mul(crate::src::enc::encode::kHashMul64);
    return (h >> 64 as libc::c_int - 16 as libc::c_int) as uint32_t;
}
#[inline(always)]
unsafe extern "C" fn StoreH2(
    mut self_0: *mut crate::src::enc::backward_references::H2,
    mut data: *const uint8_t,
    mut mask: size_t,
    mut ix: size_t,
) {
    let key = HashBytesH2(&*data.offset((ix & mask) as isize));
    if (1 as libc::c_int) << 0 as libc::c_int == 1 as libc::c_int {
        *(*self_0).buckets_.offset(key as isize) = ix as uint32_t;
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
unsafe extern "C" fn StitchToPreviousBlockH2(
    mut self_0: *mut crate::src::enc::backward_references::H2,
    mut num_bytes: size_t,
    mut position: size_t,
    mut ringbuffer: *const uint8_t,
    mut ringbuffer_mask: size_t,
) {
    if num_bytes >= (HashTypeLengthH2()).wrapping_sub(1 as libc::c_int as libc::c_ulong)
        && position >= 3 as libc::c_int as libc::c_ulong
    {
        StoreH2(
            self_0,
            ringbuffer,
            ringbuffer_mask,
            position.wrapping_sub(3 as libc::c_int as libc::c_ulong),
        );
        StoreH2(
            self_0,
            ringbuffer,
            ringbuffer_mask,
            position.wrapping_sub(2 as libc::c_int as libc::c_ulong),
        );
        StoreH2(
            self_0,
            ringbuffer,
            ringbuffer_mask,
            position.wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
    }
}
unsafe extern "C" fn HashBytesH4(mut data: *const uint8_t) -> uint32_t {
    let h = (BrotliUnalignedRead64(data as *const libc::c_void)
        << 64 as libc::c_int - 8 as libc::c_int * 5 as libc::c_int)
        .wrapping_mul(crate::src::enc::encode::kHashMul64);
    return (h >> 64 as libc::c_int - 17 as libc::c_int) as uint32_t;
}
#[inline(always)]
unsafe extern "C" fn StoreH4(
    mut self_0: *mut crate::src::enc::backward_references::H4,
    mut data: *const uint8_t,
    mut mask: size_t,
    mut ix: size_t,
) {
    let key = HashBytesH4(&*data.offset((ix & mask) as isize));
    if (1 as libc::c_int) << 2 as libc::c_int == 1 as libc::c_int {
        *((*self_0).buckets_).offset(key as isize) = ix as uint32_t;
    } else {
        let off = (ix
            & ((((1 as libc::c_int) << 2 as libc::c_int) - 1 as libc::c_int)
                << 3 as libc::c_int) as libc::c_ulong) as uint32_t;
        *(*self_0).buckets_
            .offset(
                (key.wrapping_add(off)
                    & (((1 as libc::c_int) << 17 as libc::c_int) - 1 as libc::c_int)
                        as libc::c_uint) as isize,
            ) = ix as uint32_t;
    };
}
#[inline(always)]
unsafe extern "C" fn StitchToPreviousBlockH4(
    mut self_0: *mut crate::src::enc::backward_references::H4,
    mut num_bytes: size_t,
    mut position: size_t,
    mut ringbuffer: *const uint8_t,
    mut ringbuffer_mask: size_t,
) {
    if num_bytes >= (HashTypeLengthH4()).wrapping_sub(1 as libc::c_int as libc::c_ulong)
        && position >= 3 as libc::c_int as libc::c_ulong
    {
        StoreH4(
            self_0,
            ringbuffer,
            ringbuffer_mask,
            position.wrapping_sub(3 as libc::c_int as libc::c_ulong),
        );
        StoreH4(
            self_0,
            ringbuffer,
            ringbuffer_mask,
            position.wrapping_sub(2 as libc::c_int as libc::c_ulong),
        );
        StoreH4(
            self_0,
            ringbuffer,
            ringbuffer_mask,
            position.wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
    }
}
#[inline(always)]
unsafe extern "C" fn HashTypeLengthH5() -> size_t {
    return 4 as libc::c_int as size_t;
}
unsafe extern "C" fn HashBytesH5(
    mut data: *const uint8_t,
    mut shift: libc::c_int,
) -> uint32_t {
    let mut h = (BrotliUnalignedRead32(data as *const libc::c_void))
        .wrapping_mul(crate::src::enc::encode::kHashMul32);
    return h >> shift;
}
#[inline(always)]
unsafe extern "C" fn StoreH5(
    mut self_0: *mut crate::src::enc::backward_references::H5,
    mut data: *const uint8_t,
    mut mask: size_t,
    mut ix: size_t,
) {
    let key = HashBytesH5(&*data.offset((ix & mask) as isize), (*self_0).hash_shift_);
    let minor_ix = (*(*self_0).num_.offset(key as isize) as libc::c_uint
        & (*self_0).block_mask_) as size_t;
    let offset = minor_ix.wrapping_add((key << (*self_0).block_bits_) as libc::c_ulong);
    *(*self_0).buckets_.offset(offset as isize) = ix as uint32_t;
    *(*self_0).num_.offset(key as isize) = (*(*self_0).num_.offset(key as isize)).wrapping_add(1);
}
#[inline(always)]
unsafe extern "C" fn StitchToPreviousBlockH5(
    mut self_0: *mut crate::src::enc::backward_references::H5,
    mut num_bytes: size_t,
    mut position: size_t,
    mut ringbuffer: *const uint8_t,
    mut ringbuffer_mask: size_t,
) {
    if num_bytes >= (HashTypeLengthH5()).wrapping_sub(1 as libc::c_int as libc::c_ulong)
        && position >= 3 as libc::c_int as libc::c_ulong
    {
        StoreH5(
            self_0,
            ringbuffer,
            ringbuffer_mask,
            position.wrapping_sub(3 as libc::c_int as libc::c_ulong),
        );
        StoreH5(
            self_0,
            ringbuffer,
            ringbuffer_mask,
            position.wrapping_sub(2 as libc::c_int as libc::c_ulong),
        );
        StoreH5(
            self_0,
            ringbuffer,
            ringbuffer_mask,
            position.wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
    }
}
#[inline(always)]
unsafe extern "C" fn HashTypeLengthH40() -> size_t {
    return 4 as libc::c_int as size_t;
}
unsafe extern "C" fn TinyHashH40(mut extra: *mut libc::c_void) -> *mut uint8_t {
    return core::ptr::addr_of_mut!(*((HeadH40
        as unsafe extern "C" fn(*mut libc::c_void) -> *mut uint16_t)(extra))
        .offset(((1 as libc::c_int) << 15 as libc::c_int) as isize)) as *mut uint16_t
        as *mut uint8_t;
}
unsafe extern "C" fn BanksH40(mut extra: *mut libc::c_void) -> *mut crate::src::enc::backward_references::BankH40 {
    return core::ptr::addr_of_mut!(*((TinyHashH40
        as unsafe extern "C" fn(*mut libc::c_void) -> *mut uint8_t)(extra))
        .offset(65536 as libc::c_int as isize)) as *mut uint8_t as *mut crate::src::enc::backward_references::BankH40;
}
unsafe extern "C" fn AddrH40(mut extra: *mut libc::c_void) -> *mut uint32_t {
    return extra as *mut uint32_t;
}
unsafe extern "C" fn HeadH40(mut extra: *mut libc::c_void) -> *mut uint16_t {
    return core::ptr::addr_of_mut!(*((AddrH40
        as unsafe extern "C" fn(*mut libc::c_void) -> *mut uint32_t)(extra))
        .offset(((1 as libc::c_int) << 15 as libc::c_int) as isize)) as *mut uint32_t
        as *mut uint16_t;
}
unsafe extern "C" fn TinyHashH42(mut extra: *mut libc::c_void) -> *mut uint8_t {
    return core::ptr::addr_of_mut!(*((HeadH42
        as unsafe extern "C" fn(*mut libc::c_void) -> *mut uint16_t)(extra))
        .offset(((1 as libc::c_int) << 15 as libc::c_int) as isize)) as *mut uint16_t
        as *mut uint8_t;
}
#[inline(always)]
unsafe extern "C" fn StoreH40(
    mut self_0: *mut crate::src::enc::backward_references::H40,
    mut data: *const uint8_t,
    mut mask: size_t,
    mut ix: size_t,
) {
    let mut addr = AddrH40((*self_0).extra);
    let mut head = HeadH40((*self_0).extra);
    let mut tiny_hash = TinyHashH40((*self_0).extra);
    let mut banks = BanksH40((*self_0).extra);
    let key = HashBytesH40(&*data.offset((ix & mask) as isize));
    let bank = key & (1 as libc::c_int - 1 as libc::c_int) as libc::c_ulong;
    let fresh41 = (*self_0).free_slot_idx[bank as usize];(*self_0).free_slot_idx[bank as usize]= (*self_0).free_slot_idx[bank as usize].wrapping_add(1);
    let idx = (fresh41 as libc::c_int
        & ((1 as libc::c_int) << 16 as libc::c_int) - 1 as libc::c_int) as size_t;
    let mut delta = ix.wrapping_sub(*addr.offset(key as isize) as libc::c_ulong);
    *tiny_hash.offset(ix as uint16_t as isize) = key as uint8_t;
    if delta > 0xffff as libc::c_int as libc::c_ulong {
        delta= (if 0 as libc::c_int != 0 {
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
unsafe extern "C" fn StitchToPreviousBlockH40(
    mut self_0: *mut crate::src::enc::backward_references::H40,
    mut num_bytes: size_t,
    mut position: size_t,
    mut ringbuffer: *const uint8_t,
    mut ring_buffer_mask: size_t,
) {
    if num_bytes >= (HashTypeLengthH40()).wrapping_sub(1 as libc::c_int as libc::c_ulong)
        && position >= 3 as libc::c_int as libc::c_ulong
    {
        StoreH40(
            self_0,
            ringbuffer,
            ring_buffer_mask,
            position.wrapping_sub(3 as libc::c_int as libc::c_ulong),
        );
        StoreH40(
            self_0,
            ringbuffer,
            ring_buffer_mask,
            position.wrapping_sub(2 as libc::c_int as libc::c_ulong),
        );
        StoreH40(
            self_0,
            ringbuffer,
            ring_buffer_mask,
            position.wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
    }
}
#[inline(always)]
unsafe extern "C" fn HashTypeLengthH41() -> size_t {
    return 4 as libc::c_int as size_t;
}
unsafe extern "C" fn TinyHashH41(mut extra: *mut libc::c_void) -> *mut uint8_t {
    return core::ptr::addr_of_mut!(*((HeadH41
        as unsafe extern "C" fn(*mut libc::c_void) -> *mut uint16_t)(extra))
        .offset(((1 as libc::c_int) << 15 as libc::c_int) as isize)) as *mut uint16_t
        as *mut uint8_t;
}
unsafe extern "C" fn BanksH41(mut extra: *mut libc::c_void) -> *mut crate::src::enc::backward_references::BankH41 {
    return core::ptr::addr_of_mut!(*((TinyHashH41
        as unsafe extern "C" fn(*mut libc::c_void) -> *mut uint8_t)(extra))
        .offset(65536 as libc::c_int as isize)) as *mut uint8_t as *mut crate::src::enc::backward_references::BankH41;
}
unsafe extern "C" fn AddrH41(mut extra: *mut libc::c_void) -> *mut uint32_t {
    return extra as *mut uint32_t;
}
unsafe extern "C" fn HeadH41(mut extra: *mut libc::c_void) -> *mut uint16_t {
    return core::ptr::addr_of_mut!(*((AddrH41
        as unsafe extern "C" fn(*mut libc::c_void) -> *mut uint32_t)(extra))
        .offset(((1 as libc::c_int) << 15 as libc::c_int) as isize)) as *mut uint32_t
        as *mut uint16_t;
}
#[inline(always)]
unsafe extern "C" fn HashBytesH41(mut data: *const uint8_t) -> size_t {
    let h = (BrotliUnalignedRead32(data as *const libc::c_void))
        .wrapping_mul(crate::src::enc::encode::kHashMul32);
    return (h >> 32 as libc::c_int - 15 as libc::c_int) as size_t;
}
#[inline(always)]
unsafe extern "C" fn StoreH41(
    mut self_0: *mut crate::src::enc::backward_references::H41,
    mut data: *const uint8_t,
    mut mask: size_t,
    mut ix: size_t,
) {
    let mut addr = AddrH41((*self_0).extra);
    let mut head = HeadH41((*self_0).extra);
    let mut tiny_hash = TinyHashH41((*self_0).extra);
    let mut banks = BanksH41((*self_0).extra);
    let key = HashBytesH41(&*data.offset((ix & mask) as isize));
    let bank = key & (1 as libc::c_int - 1 as libc::c_int) as libc::c_ulong;
    let fresh43 = (*self_0).free_slot_idx[bank as usize];(*self_0).free_slot_idx[bank as usize]= (*self_0).free_slot_idx[bank as usize].wrapping_add(1);
    let idx = (fresh43 as libc::c_int
        & ((1 as libc::c_int) << 16 as libc::c_int) - 1 as libc::c_int) as size_t;
    let mut delta = ix.wrapping_sub(*addr.offset(key as isize) as libc::c_ulong);
    *tiny_hash.offset(ix as uint16_t as isize) = key as uint8_t;
    if delta > 0xffff as libc::c_int as libc::c_ulong {
        delta= (if 0 as libc::c_int != 0 {
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
unsafe extern "C" fn StitchToPreviousBlockH41(
    mut self_0: *mut crate::src::enc::backward_references::H41,
    mut num_bytes: size_t,
    mut position: size_t,
    mut ringbuffer: *const uint8_t,
    mut ring_buffer_mask: size_t,
) {
    if num_bytes >= (HashTypeLengthH41()).wrapping_sub(1 as libc::c_int as libc::c_ulong)
        && position >= 3 as libc::c_int as libc::c_ulong
    {
        StoreH41(
            self_0,
            ringbuffer,
            ring_buffer_mask,
            position.wrapping_sub(3 as libc::c_int as libc::c_ulong),
        );
        StoreH41(
            self_0,
            ringbuffer,
            ring_buffer_mask,
            position.wrapping_sub(2 as libc::c_int as libc::c_ulong),
        );
        StoreH41(
            self_0,
            ringbuffer,
            ring_buffer_mask,
            position.wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
    }
}
#[inline(always)]
unsafe extern "C" fn HashTypeLengthH42() -> size_t {
    return 4 as libc::c_int as size_t;
}
unsafe extern "C" fn BanksH42(mut extra: *mut libc::c_void) -> *mut crate::src::enc::backward_references::BankH42 {
    return core::ptr::addr_of_mut!(*((TinyHashH42
        as unsafe extern "C" fn(*mut libc::c_void) -> *mut uint8_t)(extra))
        .offset(65536 as libc::c_int as isize)) as *mut uint8_t as *mut crate::src::enc::backward_references::BankH42;
}
unsafe extern "C" fn AddrH42(mut extra: *mut libc::c_void) -> *mut uint32_t {
    return extra as *mut uint32_t;
}
unsafe extern "C" fn HeadH42(mut extra: *mut libc::c_void) -> *mut uint16_t {
    return core::ptr::addr_of_mut!(*((AddrH42
        as unsafe extern "C" fn(*mut libc::c_void) -> *mut uint32_t)(extra))
        .offset(((1 as libc::c_int) << 15 as libc::c_int) as isize)) as *mut uint32_t
        as *mut uint16_t;
}
#[inline(always)]
unsafe extern "C" fn HashBytesH42(mut data: *const uint8_t) -> size_t {
    let h = (BrotliUnalignedRead32(data as *const libc::c_void))
        .wrapping_mul(crate::src::enc::encode::kHashMul32);
    return (h >> 32 as libc::c_int - 15 as libc::c_int) as size_t;
}
#[inline(always)]
unsafe extern "C" fn StoreH42(
    mut self_0: *mut crate::src::enc::backward_references::H42,
    mut data: *const uint8_t,
    mut mask: size_t,
    mut ix: size_t,
) {
    let mut addr = AddrH42((*self_0).extra);
    let mut head = HeadH42((*self_0).extra);
    let mut tiny_hash = TinyHashH42((*self_0).extra);
    let mut banks = BanksH42((*self_0).extra);
    let key = HashBytesH42(&*data.offset((ix & mask) as isize));
    let bank = key & (512 as libc::c_int - 1 as libc::c_int) as libc::c_ulong;
    let fresh45 = (*self_0).free_slot_idx[bank as usize];(*self_0).free_slot_idx[bank as usize]= (*self_0).free_slot_idx[bank as usize].wrapping_add(1);
    let idx = (fresh45 as libc::c_int
        & ((1 as libc::c_int) << 9 as libc::c_int) - 1 as libc::c_int) as size_t;
    let mut delta = ix.wrapping_sub(*addr.offset(key as isize) as libc::c_ulong);
    *tiny_hash.offset(ix as uint16_t as isize) = key as uint8_t;
    if delta > 0xffff as libc::c_int as libc::c_ulong {
        delta= (if 0 as libc::c_int != 0 {
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
unsafe extern "C" fn StitchToPreviousBlockH42(
    mut self_0: *mut crate::src::enc::backward_references::H42,
    mut num_bytes: size_t,
    mut position: size_t,
    mut ringbuffer: *const uint8_t,
    mut ring_buffer_mask: size_t,
) {
    if num_bytes >= (HashTypeLengthH42()).wrapping_sub(1 as libc::c_int as libc::c_ulong)
        && position >= 3 as libc::c_int as libc::c_ulong
    {
        StoreH42(
            self_0,
            ringbuffer,
            ring_buffer_mask,
            position.wrapping_sub(3 as libc::c_int as libc::c_ulong),
        );
        StoreH42(
            self_0,
            ringbuffer,
            ring_buffer_mask,
            position.wrapping_sub(2 as libc::c_int as libc::c_ulong),
        );
        StoreH42(
            self_0,
            ringbuffer,
            ring_buffer_mask,
            position.wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
    }
}
#[inline(always)]
unsafe extern "C" fn HashTypeLengthH3() -> size_t {
    return 8 as libc::c_int as size_t;
}
unsafe extern "C" fn HashBytesH3(mut data: *const uint8_t) -> uint32_t {
    let h = (BrotliUnalignedRead64(data as *const libc::c_void)
        << 64 as libc::c_int - 8 as libc::c_int * 5 as libc::c_int)
        .wrapping_mul(crate::src::enc::encode::kHashMul64);
    return (h >> 64 as libc::c_int - 16 as libc::c_int) as uint32_t;
}
#[inline(always)]
unsafe extern "C" fn StoreH3(
    mut self_0: *mut crate::src::enc::backward_references::H3,
    mut data: *const uint8_t,
    mut mask: size_t,
    mut ix: size_t,
) {
    let key = HashBytesH3(&*data.offset((ix & mask) as isize));
    if (1 as libc::c_int) << 1 as libc::c_int == 1 as libc::c_int {
        *((*self_0).buckets_).offset(key as isize) = ix as uint32_t;
    } else {
        let off = (ix
            & ((((1 as libc::c_int) << 1 as libc::c_int) - 1 as libc::c_int)
                << 3 as libc::c_int) as libc::c_ulong) as uint32_t;
        *(*self_0).buckets_
            .offset(
                (key.wrapping_add(off)
                    & (((1 as libc::c_int) << 16 as libc::c_int) - 1 as libc::c_int)
                        as libc::c_uint) as isize,
            ) = ix as uint32_t;
    };
}
#[inline(always)]
unsafe extern "C" fn StitchToPreviousBlockH3(
    mut self_0: *mut crate::src::enc::backward_references::H3,
    mut num_bytes: size_t,
    mut position: size_t,
    mut ringbuffer: *const uint8_t,
    mut ringbuffer_mask: size_t,
) {
    if num_bytes >= (HashTypeLengthH3()).wrapping_sub(1 as libc::c_int as libc::c_ulong)
        && position >= 3 as libc::c_int as libc::c_ulong
    {
        StoreH3(
            self_0,
            ringbuffer,
            ringbuffer_mask,
            position.wrapping_sub(3 as libc::c_int as libc::c_ulong),
        );
        StoreH3(
            self_0,
            ringbuffer,
            ringbuffer_mask,
            position.wrapping_sub(2 as libc::c_int as libc::c_ulong),
        );
        StoreH3(
            self_0,
            ringbuffer,
            ringbuffer_mask,
            position.wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
    }
}
#[inline(always)]
unsafe extern "C" fn StitchToPreviousBlockH35(
    mut self_0: *mut crate::src::enc::backward_references::H35,
    mut num_bytes: size_t,
    mut position: size_t,
    mut ringbuffer: *const uint8_t,
    mut ring_buffer_mask: size_t,
) {
    StitchToPreviousBlockH3(
        core::ptr::addr_of_mut!((*self_0).ha),
        num_bytes,
        position,
        ringbuffer,
        ring_buffer_mask,
    );
    StitchToPreviousBlockHROLLING_FAST(
        core::ptr::addr_of_mut!((*self_0).hb),
        num_bytes,
        position,
        ringbuffer,
        ring_buffer_mask,
    );
}
#[inline(always)]
unsafe extern "C" fn HashTypeLengthH54() -> size_t {
    return 8 as libc::c_int as size_t;
}
static mut kHashMul64: uint64_t = (0x1e35a7bd as libc::c_int as uint64_t)
    << 32 as libc::c_int | 0x1e35a7bd as libc::c_int as libc::c_ulong;
unsafe extern "C" fn HashBytesH54(mut data: *const uint8_t) -> uint32_t {
    let h = (BrotliUnalignedRead64(data as *const libc::c_void)
        << 64 as libc::c_int - 8 as libc::c_int * 7 as libc::c_int)
        .wrapping_mul(crate::src::enc::encode::kHashMul64);
    return (h >> 64 as libc::c_int - 20 as libc::c_int) as uint32_t;
}
#[inline(always)]
unsafe extern "C" fn StoreH54(
    mut self_0: *mut crate::src::enc::backward_references::H54,
    mut data: *const uint8_t,
    mut mask: size_t,
    mut ix: size_t,
) {
    let key = HashBytesH54(&*data.offset((ix & mask) as isize));
    if (1 as libc::c_int) << 2 as libc::c_int == 1 as libc::c_int {
        *((*self_0).buckets_).offset(key as isize) = ix as uint32_t;
    } else {
        let off = (ix
            & ((((1 as libc::c_int) << 2 as libc::c_int) - 1 as libc::c_int)
                << 3 as libc::c_int) as libc::c_ulong) as uint32_t;
        *(*self_0).buckets_
            .offset(
                (key.wrapping_add(off)
                    & (((1 as libc::c_int) << 20 as libc::c_int) - 1 as libc::c_int)
                        as libc::c_uint) as isize,
            ) = ix as uint32_t;
    };
}
#[inline(always)]
unsafe extern "C" fn StitchToPreviousBlockH54(
    mut self_0: *mut crate::src::enc::backward_references::H54,
    mut num_bytes: size_t,
    mut position: size_t,
    mut ringbuffer: *const uint8_t,
    mut ringbuffer_mask: size_t,
) {
    if num_bytes >= (HashTypeLengthH54()).wrapping_sub(1 as libc::c_int as libc::c_ulong)
        && position >= 3 as libc::c_int as libc::c_ulong
    {
        StoreH54(
            self_0,
            ringbuffer,
            ringbuffer_mask,
            position.wrapping_sub(3 as libc::c_int as libc::c_ulong),
        );
        StoreH54(
            self_0,
            ringbuffer,
            ringbuffer_mask,
            position.wrapping_sub(2 as libc::c_int as libc::c_ulong),
        );
        StoreH54(
            self_0,
            ringbuffer,
            ringbuffer_mask,
            position.wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
    }
}
unsafe extern "C" fn HashByteHROLLING_FAST(mut byte: uint8_t) -> uint32_t {
    return (byte as uint32_t).wrapping_add(1 as libc::c_uint);
}
unsafe extern "C" fn HashRollingFunctionInitialHROLLING_FAST(
    mut state: uint32_t,
    mut add: uint8_t,
    mut factor: uint32_t,
) -> uint32_t {
    return factor.wrapping_mul(state).wrapping_add(HashByteHROLLING_FAST(add));
}
unsafe extern "C" fn PrepareHROLLING_FAST(
    mut self_0: *mut crate::src::enc::backward_references::HROLLING_FAST,
    mut one_shot: libc::c_int,
    mut input_size: size_t,
    mut data: *const uint8_t,
) {
    let mut i: size_t = 0;
    if input_size < 32 as libc::c_int as libc::c_ulong {
        return;
    }
    (*self_0).state= 0 as libc::c_int as uint32_t;
    i= 0 as libc::c_int as size_t;
    while i < 32 as libc::c_int as libc::c_ulong {
        (*self_0).state= HashRollingFunctionInitialHROLLING_FAST(
            (*self_0).state,
            *data.offset(i as isize),
            (*self_0).factor,
        );
        i= (i as libc::c_ulong).wrapping_add(4 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
    }
}
#[inline(always)]
unsafe extern "C" fn StitchToPreviousBlockHROLLING_FAST(
    mut self_0: *mut crate::src::enc::backward_references::HROLLING_FAST,
    mut num_bytes: size_t,
    mut position: size_t,
    mut ringbuffer: *const uint8_t,
    mut ring_buffer_mask: size_t,
) {
    let mut position_masked: size_t = 0;
    let mut available = num_bytes;
    if position & (4 as libc::c_int - 1 as libc::c_int) as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        let mut diff = (4 as libc::c_int as libc::c_ulong)
            .wrapping_sub(
                position & (4 as libc::c_int - 1 as libc::c_int) as libc::c_ulong,
            );
        available= if diff > available {
            0 as libc::c_int as libc::c_ulong
        } else {
            available.wrapping_sub(diff)
        };
        position= (position as libc::c_ulong).wrapping_add(diff) as size_t as size_t;
    }
    position_masked= position & ring_buffer_mask;
    if available > ring_buffer_mask.wrapping_sub(position_masked) {
        available= ring_buffer_mask.wrapping_sub(position_masked);
    }
    PrepareHROLLING_FAST(
        self_0,
        0 as libc::c_int,
        available,
        ringbuffer.offset((position & ring_buffer_mask) as isize),
    );
    (*self_0).next_ix= position;
}
#[inline(always)]
unsafe extern "C" fn StitchToPreviousBlockH55(
    mut self_0: *mut crate::src::enc::backward_references::H55,
    mut num_bytes: size_t,
    mut position: size_t,
    mut ringbuffer: *const uint8_t,
    mut ring_buffer_mask: size_t,
) {
    StitchToPreviousBlockH54(
        core::ptr::addr_of_mut!((*self_0).ha),
        num_bytes,
        position,
        ringbuffer,
        ring_buffer_mask,
    );
    StitchToPreviousBlockHROLLING_FAST(
        core::ptr::addr_of_mut!((*self_0).hb),
        num_bytes,
        position,
        ringbuffer,
        ring_buffer_mask,
    );
}
#[inline(always)]
unsafe extern "C" fn HashTypeLengthH6() -> size_t {
    return 8 as libc::c_int as size_t;
}
static mut kHashMul64Long: uint64_t = (0x1fe35a7b as libc::c_uint as uint64_t)
    << 32 as libc::c_int | 0xd3579bd3 as libc::c_uint as libc::c_ulong;
#[inline(always)]
unsafe extern "C" fn HashBytesH6(
    mut data: *const uint8_t,
    mut mask: uint64_t,
    mut shift: libc::c_int,
) -> uint32_t {
    let h = (BrotliUnalignedRead64(data as *const libc::c_void) & mask)
        .wrapping_mul(crate::src::enc::encode::kHashMul64Long);
    return (h >> shift) as uint32_t;
}
#[inline(always)]
unsafe extern "C" fn StoreH6(
    mut self_0: *mut crate::src::enc::backward_references::H6,
    mut data: *const uint8_t,
    mut mask: size_t,
    mut ix: size_t,
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
    *num.offset(key as isize) = (*num.offset(key as isize)).wrapping_add(1);
    *buckets.offset(offset as isize) = ix as uint32_t;
}
#[inline(always)]
unsafe extern "C" fn StitchToPreviousBlockH6(
    mut self_0: *mut crate::src::enc::backward_references::H6,
    mut num_bytes: size_t,
    mut position: size_t,
    mut ringbuffer: *const uint8_t,
    mut ringbuffer_mask: size_t,
) {
    if num_bytes >= (HashTypeLengthH6()).wrapping_sub(1 as libc::c_int as libc::c_ulong)
        && position >= 3 as libc::c_int as libc::c_ulong
    {
        StoreH6(
            self_0,
            ringbuffer,
            ringbuffer_mask,
            position.wrapping_sub(3 as libc::c_int as libc::c_ulong),
        );
        StoreH6(
            self_0,
            ringbuffer,
            ringbuffer_mask,
            position.wrapping_sub(2 as libc::c_int as libc::c_ulong),
        );
        StoreH6(
            self_0,
            ringbuffer,
            ringbuffer_mask,
            position.wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
    }
}
unsafe extern "C" fn HashByteHROLLING(mut byte: uint8_t) -> uint32_t {
    return (byte as uint32_t).wrapping_add(1 as libc::c_uint);
}
unsafe extern "C" fn HashRollingFunctionInitialHROLLING(
    mut state: uint32_t,
    mut add: uint8_t,
    mut factor: uint32_t,
) -> uint32_t {
    return factor.wrapping_mul(state).wrapping_add(HashByteHROLLING(add));
}
unsafe extern "C" fn PrepareHROLLING(
    mut self_0: *mut crate::src::enc::backward_references::HROLLING,
    mut one_shot: libc::c_int,
    mut input_size: size_t,
    mut data: *const uint8_t,
) {
    let mut i: size_t = 0;
    if input_size < 32 as libc::c_int as libc::c_ulong {
        return;
    }
    (*self_0).state= 0 as libc::c_int as uint32_t;
    i= 0 as libc::c_int as size_t;
    while i < 32 as libc::c_int as libc::c_ulong {
        (*self_0).state= HashRollingFunctionInitialHROLLING(
            (*self_0).state,
            *data.offset(i as isize),
            (*self_0).factor,
        );
        i= (i as libc::c_ulong).wrapping_add(1 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
    }
}
#[inline(always)]
unsafe extern "C" fn StitchToPreviousBlockHROLLING(
    mut self_0: *mut crate::src::enc::backward_references::HROLLING,
    mut num_bytes: size_t,
    mut position: size_t,
    mut ringbuffer: *const uint8_t,
    mut ring_buffer_mask: size_t,
) {
    let mut position_masked: size_t = 0;
    let mut available = num_bytes;
    if position & (1 as libc::c_int - 1 as libc::c_int) as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        let mut diff = (1 as libc::c_int as libc::c_ulong)
            .wrapping_sub(
                position & (1 as libc::c_int - 1 as libc::c_int) as libc::c_ulong,
            );
        available= if diff > available {
            0 as libc::c_int as libc::c_ulong
        } else {
            available.wrapping_sub(diff)
        };
        position= (position as libc::c_ulong).wrapping_add(diff) as size_t as size_t;
    }
    position_masked= position & ring_buffer_mask;
    if available > ring_buffer_mask.wrapping_sub(position_masked) {
        available= ring_buffer_mask.wrapping_sub(position_masked);
    }
    PrepareHROLLING(
        self_0,
        0 as libc::c_int,
        available,
        ringbuffer.offset((position & ring_buffer_mask) as isize),
    );
    (*self_0).next_ix= position;
}
#[inline(always)]
unsafe extern "C" fn StitchToPreviousBlockH65(
    mut self_0: *mut crate::src::enc::backward_references::H65,
    mut num_bytes: size_t,
    mut position: size_t,
    mut ringbuffer: *const uint8_t,
    mut ring_buffer_mask: size_t,
) {
    StitchToPreviousBlockH6(
        core::ptr::addr_of_mut!((*self_0).ha),
        num_bytes,
        position,
        ringbuffer,
        ring_buffer_mask,
    );
    StitchToPreviousBlockHROLLING(
        core::ptr::addr_of_mut!((*self_0).hb),
        num_bytes,
        position,
        ringbuffer,
        ring_buffer_mask,
    );
}
#[inline(always)]
unsafe extern "C" fn HashTypeLengthH10() -> size_t {
    return 4 as libc::c_int as size_t;
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
unsafe extern "C" fn InitBackwardMatch(
    mut self_0: Option<&mut crate::src::enc::backward_references_hq::BackwardMatch>,
    mut dist: size_t,
    mut len: size_t,
) {
    (*self_0.as_deref_mut().unwrap()).distance= dist as uint32_t;
    (*self_0.as_deref_mut().unwrap()).length_and_code= (len << 5 as libc::c_int) as uint32_t;
}
#[inline(always)]
unsafe extern "C" fn LeftChildIndexH10(mut self_0: *mut crate::src::enc::backward_references::H10, mut pos: size_t) -> size_t {
    return (2 as libc::c_int as libc::c_ulong)
        .wrapping_mul(pos & (*self_0).window_mask_);
}
static mut kHashMul32: uint32_t = 0x1e35a7bd as libc::c_int as uint32_t;
unsafe extern "C" fn HashBytesH10(mut data: *const uint8_t) -> uint32_t {
    let mut h = (BrotliUnalignedRead32(data as *const libc::c_void))
        .wrapping_mul(crate::src::enc::encode::kHashMul32);
    return h >> 32 as libc::c_int - 17 as libc::c_int;
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
    mut matches: *mut crate::src::enc::backward_references_hq::BackwardMatch,
) -> *mut crate::src::enc::backward_references_hq::BackwardMatch {
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
                let fresh47 = matches;
                matches= matches.offset(1);
                InitBackwardMatch(fresh47.as_mut(), backward, len);
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
unsafe extern "C" fn StitchToPreviousBlockH10(
    mut self_0: *mut crate::src::enc::backward_references::H10,
    mut num_bytes: size_t,
    mut position: size_t,
    mut ringbuffer: *const uint8_t,
    mut ringbuffer_mask: size_t,
) {
    if num_bytes >= (HashTypeLengthH10()).wrapping_sub(1 as libc::c_int as libc::c_ulong)
        && position >= 128 as libc::c_int as libc::c_ulong
    {
        let i_start = position
            .wrapping_sub(128 as libc::c_int as libc::c_ulong)
            .wrapping_add(1 as libc::c_int as libc::c_ulong);
        let i_end = brotli_min_size_t(position, i_start.wrapping_add(num_bytes));
        let mut i: size_t = 0;
        i= i_start;
        while i < i_end {
            let max_backward = (*self_0).window_mask_
                .wrapping_sub(
                    brotli_max_size_t(
                        (16 as libc::c_int - 1 as libc::c_int) as size_t,
                        position.wrapping_sub(i),
                    ),
                );
            StoreAndFindMatchesH10(
                self_0,
                ringbuffer,
                i,
                ringbuffer_mask,
                128 as libc::c_int as size_t,
                max_backward,
                0 as *mut size_t,
                0 as *mut crate::src::enc::backward_references_hq::BackwardMatch,
            );
            i= i.wrapping_add(1);
        }
    }
}
#[inline(always)]
unsafe extern "C" fn InitOrStitchToPreviousBlock(
    mut m: *mut crate::src::enc::backward_references_hq::MemoryManager,
    mut hasher: *mut crate::src::enc::backward_references::Hasher,
    mut data: *const uint8_t,
    mut mask: size_t,
    mut params: *mut crate::src::enc::backward_references::BrotliEncoderParams,
    mut position: size_t,
    mut input_size: size_t,
    mut is_last: libc::c_int,
) {
    HasherSetup(m, hasher, params, data, position, input_size, is_last);
    if 0 as libc::c_int != 0 {
        return;
    }
    match (*hasher).common.params.type_0 {
        2 => {
            StitchToPreviousBlockH2(
                core::ptr::addr_of_mut!((*hasher).privat._H2),
                input_size,
                position,
                data,
                mask,
            );
        }
        3 => {
            StitchToPreviousBlockH3(
                core::ptr::addr_of_mut!((*hasher).privat._H3),
                input_size,
                position,
                data,
                mask,
            );
        }
        4 => {
            StitchToPreviousBlockH4(
                core::ptr::addr_of_mut!((*hasher).privat._H4),
                input_size,
                position,
                data,
                mask,
            );
        }
        5 => {
            StitchToPreviousBlockH5(
                core::ptr::addr_of_mut!((*hasher).privat._H5),
                input_size,
                position,
                data,
                mask,
            );
        }
        6 => {
            StitchToPreviousBlockH6(
                core::ptr::addr_of_mut!((*hasher).privat._H6),
                input_size,
                position,
                data,
                mask,
            );
        }
        40 => {
            StitchToPreviousBlockH40(
                core::ptr::addr_of_mut!((*hasher).privat._H40),
                input_size,
                position,
                data,
                mask,
            );
        }
        41 => {
            StitchToPreviousBlockH41(
                core::ptr::addr_of_mut!((*hasher).privat._H41),
                input_size,
                position,
                data,
                mask,
            );
        }
        42 => {
            StitchToPreviousBlockH42(
                core::ptr::addr_of_mut!((*hasher).privat._H42),
                input_size,
                position,
                data,
                mask,
            );
        }
        54 => {
            StitchToPreviousBlockH54(
                core::ptr::addr_of_mut!((*hasher).privat._H54),
                input_size,
                position,
                data,
                mask,
            );
        }
        35 => {
            StitchToPreviousBlockH35(
                core::ptr::addr_of_mut!((*hasher).privat._H35),
                input_size,
                position,
                data,
                mask,
            );
        }
        55 => {
            StitchToPreviousBlockH55(
                core::ptr::addr_of_mut!((*hasher).privat._H55),
                input_size,
                position,
                data,
                mask,
            );
        }
        65 => {
            StitchToPreviousBlockH65(
                core::ptr::addr_of_mut!((*hasher).privat._H65),
                input_size,
                position,
                data,
                mask,
            );
        }
        10 => {
            StitchToPreviousBlockH10(
                core::ptr::addr_of_mut!((*hasher).privat._H10),
                input_size,
                position,
                data,
                mask,
            );
        }
        _ => {}
    };
}
#[inline(always)]
unsafe extern "C" fn CommandRestoreDistanceCode(
    mut self_0: *const crate::src::enc::backward_references::Command,
    mut dist: *const crate::src::enc::backward_references::BrotliDistanceParams,
) -> uint32_t {
    if ((*self_0).dist_prefix_ as libc::c_uint & 0x3ff as libc::c_uint)
        < (16 as libc::c_int as libc::c_uint)
            .wrapping_add((*dist).num_direct_distance_codes)
    {
        return (*self_0).dist_prefix_ as libc::c_uint & 0x3ff as libc::c_uint
    } else {
        let mut dcode = (*self_0).dist_prefix_ as libc::c_uint & 0x3ff as libc::c_uint;
        let mut nbits = ((*self_0).dist_prefix_ as libc::c_int >> 10 as libc::c_int)
            as uint32_t;
        let mut extra = (*self_0).dist_extra_;
        let mut postfix_mask = ((1 as libc::c_uint) << (*dist).distance_postfix_bits)
            .wrapping_sub(1 as libc::c_uint);
        let mut hcode = dcode
            .wrapping_sub((*dist).num_direct_distance_codes)
            .wrapping_sub(16 as libc::c_int as libc::c_uint)
            >> (*dist).distance_postfix_bits;
        let mut lcode = dcode
            .wrapping_sub((*dist).num_direct_distance_codes)
            .wrapping_sub(16 as libc::c_int as libc::c_uint) & postfix_mask;
        let mut offset = ((2 as libc::c_uint).wrapping_add(hcode & 1 as libc::c_uint)
            << nbits)
            .wrapping_sub(4 as libc::c_uint);
        return (offset.wrapping_add(extra) << (*dist).distance_postfix_bits)
            .wrapping_add(lcode)
            .wrapping_add((*dist).num_direct_distance_codes)
            .wrapping_add(16 as libc::c_int as libc::c_uint);
    };
}
#[inline(always)]
unsafe extern "C" fn ComputeRbBits(
    mut params: *const crate::src::enc::backward_references::BrotliEncoderParams,
) -> libc::c_int {
    return 1 as libc::c_int + brotli_max_int((*params).lgwin, (*params).lgblock);
}
#[inline(always)]
unsafe extern "C" fn MaxMetablockSize(mut params: *const crate::src::enc::backward_references::BrotliEncoderParams) -> size_t {
    let mut bits = brotli_min_int(ComputeRbBits(params), 24 as libc::c_int);
    return (1 as libc::c_int as size_t) << bits;
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
unsafe extern "C" fn Log2FloorNonZero(mut n: size_t) -> uint32_t {
    return 31 as libc::c_uint ^ (n as uint32_t).leading_zeros() as i32 as uint32_t;
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
unsafe extern "C" fn InitInsertCommand(mut self_0: Option<&mut crate::src::enc::backward_references::Command>, mut insertlen: size_t) {
    (*self_0.as_deref_mut().unwrap()).insert_len_= insertlen as uint32_t;
    (*self_0.as_deref_mut().unwrap()).copy_len_= ((4 as libc::c_int) << 25 as libc::c_int) as uint32_t;
    (*self_0.as_deref_mut().unwrap()).dist_extra_= 0 as libc::c_int as uint32_t;
    (*self_0.as_deref_mut().unwrap()).dist_prefix_= 16 as libc::c_int as uint16_t;
    GetLengthCode(
        insertlen,
        4 as libc::c_int as size_t,
        0 as libc::c_int,
        Some(&mut (*self_0.as_deref_mut().unwrap()).cmd_prefix_),
    );
}
#[inline(always)]
unsafe extern "C" fn FastLog2(mut v: size_t) -> libc::c_double {
    if v < 256 as libc::c_int as libc::c_ulong {
        return crate::src::enc::encode::kBrotliLog2Table[v as usize];
    }
    return log2(v as libc::c_double);
}
#[inline(always)]
unsafe extern "C" fn HasherReset(mut hasher: Option<&mut crate::src::enc::backward_references::Hasher>) {
    (*hasher.as_deref_mut().unwrap()).common.is_prepared_= 0 as libc::c_int;
}
#[inline(always)]
unsafe extern "C" fn DestroyHasher(mut m: *mut crate::src::enc::backward_references_hq::MemoryManager, mut hasher: *mut crate::src::enc::backward_references::Hasher) {
    if (*hasher).common.extra.is_null() {();
        return;
    }
    crate::src::enc::memory::BrotliFree(m, (*hasher).common.extra);
    (*hasher).common.extra= 0 as *mut libc::c_void;
}
#[inline(always)]
unsafe extern "C" fn HasherInit(mut hasher: Option<&mut crate::src::enc::backward_references::Hasher>) {
    (*hasher.as_deref_mut().unwrap()).common.extra= 0 as *mut libc::c_void;
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
        current_block= 7572130911416793741;
    } else {
        current_block= 715039052867723359;
    }
    loop {
        match current_block {
            715039052867723359 => {
                if !(population < population_end) {
                    break;
                }
                let fresh50 = population;
                population= population.offset(1);
                p= (*fresh50) as size_t;
                sum= (sum as libc::c_ulong).wrapping_add(p) as size_t as size_t;
                retval-= p as libc::c_double * FastLog2(p);
                current_block= 7572130911416793741;
            }
            _ => {
                let fresh51 = population;
                population= population.offset(1);
                p= (*fresh51) as size_t;
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
unsafe extern "C" fn InitMetaBlockSplit(mut mb: Option<&mut crate::src::enc::brotli_bit_stream::MetaBlockSplit>) {
    crate::src::enc::block_splitter::BrotliInitBlockSplit(Some(&mut (*mb.as_deref_mut().unwrap()).literal_split));
    crate::src::enc::block_splitter::BrotliInitBlockSplit(Some(&mut (*mb.as_deref_mut().unwrap()).command_split));
    crate::src::enc::block_splitter::BrotliInitBlockSplit(Some(&mut (*mb.as_deref_mut().unwrap()).distance_split));
    (*mb.as_deref_mut().unwrap()).literal_context_map= 0 as *mut uint32_t;
    (*mb.as_deref_mut().unwrap()).literal_context_map_size= 0 as libc::c_int as size_t;
    (*mb.as_deref_mut().unwrap()).distance_context_map= 0 as *mut uint32_t;
    (*mb.as_deref_mut().unwrap()).distance_context_map_size= 0 as libc::c_int as size_t;
    (*mb.as_deref_mut().unwrap()).literal_histograms= 0 as *mut crate::src::enc::bit_cost::HistogramLiteral;
    (*mb.as_deref_mut().unwrap()).literal_histograms_size= 0 as libc::c_int as size_t;
    (*mb.as_deref_mut().unwrap()).command_histograms= 0 as *mut crate::src::enc::bit_cost::HistogramCommand;
    (*mb.as_deref_mut().unwrap()).command_histograms_size= 0 as libc::c_int as size_t;
    (*mb.as_deref_mut().unwrap()).distance_histograms= 0 as *mut crate::src::enc::bit_cost::HistogramDistance;
    (*mb.as_deref_mut().unwrap()).distance_histograms_size= 0 as libc::c_int as size_t;
}
#[inline(always)]
unsafe extern "C" fn DestroyMetaBlockSplit(
    mut m: *mut crate::src::enc::backward_references_hq::MemoryManager,
    mut mb: *mut crate::src::enc::brotli_bit_stream::MetaBlockSplit,
) {
    crate::src::enc::block_splitter::BrotliDestroyBlockSplit(m, core::ptr::addr_of_mut!((*mb).literal_split));
    crate::src::enc::block_splitter::BrotliDestroyBlockSplit(m, core::ptr::addr_of_mut!((*mb).command_split));
    crate::src::enc::block_splitter::BrotliDestroyBlockSplit(m, core::ptr::addr_of_mut!((*mb).distance_split));
    crate::src::enc::memory::BrotliFree(m, (*mb).literal_context_map as *mut libc::c_void);
    (*mb).literal_context_map= 0 as *mut uint32_t;
    crate::src::enc::memory::BrotliFree(m, (*mb).distance_context_map as *mut libc::c_void);
    (*mb).distance_context_map= 0 as *mut uint32_t;
    crate::src::enc::memory::BrotliFree(m, (*mb).literal_histograms as *mut libc::c_void);
    (*mb).literal_histograms= 0 as *mut crate::src::enc::bit_cost::HistogramLiteral;
    crate::src::enc::memory::BrotliFree(m, (*mb).command_histograms as *mut libc::c_void);
    (*mb).command_histograms= 0 as *mut crate::src::enc::bit_cost::HistogramCommand;
    crate::src::enc::memory::BrotliFree(m, (*mb).distance_histograms as *mut libc::c_void);
    (*mb).distance_histograms= 0 as *mut crate::src::enc::bit_cost::HistogramDistance;
}
static mut kCompressFragmentTwoPassBlockSize: size_t = ((1 as libc::c_int)
    << 17 as libc::c_int) as size_t;
#[inline(always)]
unsafe extern "C" fn RingBufferInit(mut rb: Option<&mut RingBuffer>) {
    (*rb.as_deref_mut().unwrap()).cur_size_= 0 as libc::c_int as uint32_t;
    (*rb.as_deref_mut().unwrap()).pos_= 0 as libc::c_int as uint32_t;
    (*rb.as_deref_mut().unwrap()).data_= 0 as *mut uint8_t;
    (*rb.as_deref_mut().unwrap()).buffer_= 0 as *mut uint8_t;
}
#[inline(always)]
unsafe extern "C" fn RingBufferSetup(
    mut params: *const crate::src::enc::backward_references::BrotliEncoderParams,
    mut rb: *mut RingBuffer,
) {
    let mut window_bits = ComputeRbBits(params);
    let mut tail_bits = (*params).lgblock;
    *(&(*rb).size_ as *const uint32_t
        as *mut uint32_t) = (1 as libc::c_uint) << window_bits;
    *(&(*rb).mask_ as *const uint32_t
        as *mut uint32_t) = ((1 as libc::c_uint) << window_bits)
        .wrapping_sub(1 as libc::c_int as libc::c_uint);
    *(&(*rb).tail_size_ as *const uint32_t
        as *mut uint32_t) = (1 as libc::c_uint) << tail_bits;
    *(&(*rb).total_size_ as *const uint32_t
        as *mut uint32_t) = (*rb).size_.wrapping_add((*rb).tail_size_);
}
#[inline(always)]
unsafe extern "C" fn RingBufferFree(mut m: *mut crate::src::enc::backward_references_hq::MemoryManager, mut rb: *mut RingBuffer) {
    crate::src::enc::memory::BrotliFree(m, (*rb).data_ as *mut libc::c_void);
    (*rb).data_= 0 as *mut uint8_t;
}
#[inline(always)]
unsafe extern "C" fn RingBufferInitBuffer(
    mut m: *mut crate::src::enc::backward_references_hq::MemoryManager,
    mut buflen: uint32_t,
    mut rb: *mut RingBuffer,
) {
    static mut kSlackForEightByteHashingEverywhere: size_t = 7 as libc::c_int as size_t;
    let mut new_data = if ((2 as libc::c_int as libc::c_uint).wrapping_add(buflen)
        as libc::c_ulong)
        .wrapping_add(kSlackForEightByteHashingEverywhere)
        > 0 as libc::c_int as libc::c_ulong
    {
        crate::src::enc::memory::BrotliAllocate(
            m,
            ((2 as libc::c_int as libc::c_uint).wrapping_add(buflen) as libc::c_ulong)
                .wrapping_add(kSlackForEightByteHashingEverywhere)
                .wrapping_mul(::std::mem::size_of::<uint8_t>() as libc::c_ulong),
        ) as *mut uint8_t
    } else {
        0 as *mut uint8_t
    };
    let mut i: size_t = 0;
    if 0 as libc::c_int != 0 || 0 as libc::c_int != 0 {
        return;
    }
    if !(*rb).data_.is_null() {
        memcpy(
            new_data as *mut libc::c_void,
            (*rb).data_ as *const libc::c_void,
            ((2 as libc::c_int as libc::c_uint).wrapping_add((*rb).cur_size_)
                as libc::c_ulong)
                .wrapping_add(kSlackForEightByteHashingEverywhere),
        );
        crate::src::enc::memory::BrotliFree(m, (*rb).data_ as *mut libc::c_void);
        (*rb).data_= 0 as *mut uint8_t;
    }else { (); }
    (*rb).data_= new_data;
    (*rb).cur_size_= buflen;
    (*rb).buffer_= (*rb).data_.offset(2 as libc::c_int as isize);
    *(*rb).buffer_.offset(-(1 as libc::c_int) as isize) = 0 as libc::c_int as uint8_t; *(*rb).buffer_.offset(-(2 as libc::c_int) as isize)  = *(*rb).buffer_.offset(-(1 as libc::c_int) as isize);
    i= 0 as libc::c_int as size_t;
    while i < kSlackForEightByteHashingEverywhere {
        *(*rb).buffer_
            .offset(
                ((*rb).cur_size_ as libc::c_ulong).wrapping_add(i) as isize,
            ) = 0 as libc::c_int as uint8_t;
        i= i.wrapping_add(1);
    }
}
#[inline(always)]
unsafe extern "C" fn RingBufferWriteTail(
    mut bytes: *const uint8_t,
    mut n: size_t,
    mut rb: *mut RingBuffer,
) {
    let masked_pos = ((*rb).pos_ & (*rb).mask_) as size_t;
    if (masked_pos < (*rb).tail_size_ as libc::c_ulong) as libc::c_int as libc::c_long
        != 0
    {
        let p = ((*rb).size_ as libc::c_ulong).wrapping_add(masked_pos);
        memcpy(
            core::ptr::addr_of_mut!(*(*rb).buffer_.offset(p as isize)) as *mut uint8_t
                as *mut libc::c_void,
            bytes as *const libc::c_void,
            brotli_min_size_t(
                n,
                ((*rb).tail_size_ as libc::c_ulong).wrapping_sub(masked_pos),
            ),
        );
    }
}
#[inline(always)]
unsafe extern "C" fn RingBufferWrite(
    mut m: *mut crate::src::enc::backward_references_hq::MemoryManager,
    mut bytes: *const uint8_t,
    mut n: size_t,
    mut rb: *mut RingBuffer,
) {
    if (*rb).pos_ == 0 as libc::c_int as libc::c_uint
        && n < (*rb).tail_size_ as libc::c_ulong
    {
        (*rb).pos_= n as uint32_t;
        RingBufferInitBuffer(m, (*rb).pos_, rb);
        if 0 as libc::c_int != 0 {
            return;
        }
        memcpy((*rb).buffer_ as *mut libc::c_void, bytes as *const libc::c_void, n);
        return;
    }
    if (*rb).cur_size_ < (*rb).total_size_ {
        RingBufferInitBuffer(m, (*rb).total_size_, rb);
        if 0 as libc::c_int != 0 {
            return;
        }
        *(*rb).buffer_
            .offset(
                (*rb).size_.wrapping_sub(2 as libc::c_int as libc::c_uint) as isize,
            ) = 0 as libc::c_int as uint8_t;
        *(*rb).buffer_
            .offset(
                (*rb).size_.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
            ) = 0 as libc::c_int as uint8_t;
        *(*rb).buffer_.offset((*rb).size_ as isize) = 241 as libc::c_int as uint8_t;
    }
    let masked_pos = ((*rb).pos_ & (*rb).mask_) as size_t;
    RingBufferWriteTail(bytes, n, rb);
    if (masked_pos.wrapping_add(n) <= (*rb).size_ as libc::c_ulong) as libc::c_int
        as libc::c_long != 0
    {
        memcpy(
            core::ptr::addr_of_mut!(*(*rb).buffer_.offset(masked_pos as isize)) as *mut uint8_t
                as *mut libc::c_void,
            bytes as *const libc::c_void,
            n,
        );
    } else {
        memcpy(
            core::ptr::addr_of_mut!(*(*rb).buffer_.offset(masked_pos as isize)) as *mut uint8_t
                as *mut libc::c_void,
            bytes as *const libc::c_void,
            brotli_min_size_t(
                n,
                ((*rb).total_size_ as libc::c_ulong).wrapping_sub(masked_pos),
            ),
        );
        memcpy(
            core::ptr::addr_of_mut!(*(*rb).buffer_.offset(0 as libc::c_int as isize)) as *mut uint8_t
                as *mut libc::c_void,
            bytes
                .offset(((*rb).size_ as libc::c_ulong).wrapping_sub(masked_pos) as isize)
                as *const libc::c_void,
            n.wrapping_sub(((*rb).size_ as libc::c_ulong).wrapping_sub(masked_pos)),
        );
    }
    let mut not_first_lap = ((*rb).pos_ & (1 as libc::c_uint) << 31 as libc::c_int
        != 0 as libc::c_int as libc::c_uint) as libc::c_int;
    let mut rb_pos_mask = ((1 as libc::c_uint) << 31 as libc::c_int)
        .wrapping_sub(1 as libc::c_int as libc::c_uint);
    *(*rb).buffer_
        .offset(
            -(2 as libc::c_int) as isize,
        ) = *(*rb).buffer_
        .offset((*rb).size_.wrapping_sub(2 as libc::c_int as libc::c_uint) as isize);
    *(*rb).buffer_
        .offset(
            -(1 as libc::c_int) as isize,
        ) = *(*rb).buffer_
        .offset((*rb).size_.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize);
    (*rb).pos_= ((*rb).pos_ & rb_pos_mask)
        .wrapping_add((n & rb_pos_mask as libc::c_ulong) as uint32_t);
    if not_first_lap != 0 {
        (*rb).pos_= (1 as libc::c_uint) << 31 as libc::c_int;
    }
}
static mut kMinUTF8Ratio: libc::c_double = 0.75f64;
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
unsafe extern "C" fn InputBlockSize(mut s: *mut BrotliEncoderState) -> size_t {
    return (1 as libc::c_int as size_t) << (*s).params.lgblock;
}
unsafe extern "C" fn UnprocessedInputSize(mut s: *mut BrotliEncoderState) -> uint64_t {
    return (*s).input_pos_.wrapping_sub((*s).last_processed_pos_);
}
unsafe extern "C" fn RemainingInputBlockSize(mut s: *mut BrotliEncoderState) -> size_t {
    let delta = UnprocessedInputSize(s);
    let mut block_size = InputBlockSize(s);
    if delta >= block_size {
        return 0 as libc::c_int as size_t;
    }
    return block_size.wrapping_sub(delta);
}
#[no_mangle]
pub unsafe extern "C" fn BrotliEncoderSetParameter(
    mut state: Option<&mut BrotliEncoderState>,
    mut p: BrotliEncoderParameter,
    mut value: uint32_t,
) -> libc::c_int {
    if (*state.as_deref().unwrap()).is_initialized_ != 0 {
        return 0 as libc::c_int;
    }
    match  p as libc::c_uint {
        0 => {
            (*state.as_deref_mut().unwrap()).params.mode= value as BrotliEncoderMode;
            return 1 as libc::c_int;
        }
        1 => {
            (*state.as_deref_mut().unwrap()).params.quality= value as libc::c_int;
            return 1 as libc::c_int;
        }
        2 => {
            (*state.as_deref_mut().unwrap()).params.lgwin= value as libc::c_int;
            return 1 as libc::c_int;
        }
        3 => {
            (*state.as_deref_mut().unwrap()).params.lgblock= value as libc::c_int;
            return 1 as libc::c_int;
        }
        4 => {
            if value != 0 as libc::c_int as libc::c_uint
                && value != 1 as libc::c_int as libc::c_uint
            {
                return 0 as libc::c_int;
            }
            (*state.as_deref_mut().unwrap()).params.disable_literal_context_modeling= if value != 0 {
                1 as libc::c_int
            } else {
                0 as libc::c_int
            };
            return 1 as libc::c_int;
        }
        5 => {
            (*state.as_deref_mut().unwrap()).params.size_hint= value as size_t;
            return 1 as libc::c_int;
        }
        6 => {
            (*state.as_deref_mut().unwrap()).params.large_window= if value != 0 {
                1 as libc::c_int
            } else {
                0 as libc::c_int
            };
            return 1 as libc::c_int;
        }
        7 => {
            (*state.as_deref_mut().unwrap()).params.dist.distance_postfix_bits= value;
            return 1 as libc::c_int;
        }
        8 => {
            (*state.as_deref_mut().unwrap()).params.dist.num_direct_distance_codes= value;
            return 1 as libc::c_int;
        }
        9 => {
            if value > (1 as libc::c_uint) << 30 as libc::c_int {
                return 0 as libc::c_int;
            }
            (*state.as_deref_mut().unwrap()).params.stream_offset= value as size_t;
            return 1 as libc::c_int;
        }
        _ => return 0 as libc::c_int,
    };
}
unsafe extern "C" fn WrapPosition(mut position: uint64_t) -> uint32_t {
    let mut result = position as uint32_t;
    let mut gb = position >> 30 as libc::c_int;
    if gb > 2 as libc::c_int as libc::c_ulong {
        result= result
            & ((1 as libc::c_uint) << 30 as libc::c_int)
                .wrapping_sub(1 as libc::c_int as libc::c_uint)
            | ((gb.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                & 1 as libc::c_int as libc::c_ulong) as uint32_t)
                .wrapping_add(1 as libc::c_int as libc::c_uint) << 30 as libc::c_int;
    }
    return result;
}
unsafe extern "C" fn GetBrotliStorage(
    mut s: *mut BrotliEncoderState,
    mut size: size_t,
) -> *mut uint8_t {
    let mut m: *mut crate::src::enc::backward_references_hq::MemoryManager = core::ptr::addr_of_mut!((*s).memory_manager_);
    if (*s).storage_size_ < size {
        crate::src::enc::memory::BrotliFree(m, (*s).storage_ as *mut libc::c_void);
        (*s).storage_= 0 as *mut uint8_t;
        (*s).storage_= if size > 0 as libc::c_int as libc::c_ulong {
            crate::src::enc::memory::BrotliAllocate(
                m,
                size.wrapping_mul(::std::mem::size_of::<uint8_t>() as libc::c_ulong),
            ) as *mut uint8_t
        } else {
            0 as *mut uint8_t
        };
        if 0 as libc::c_int != 0 || 0 as libc::c_int != 0 {
            return 0 as *mut uint8_t;
        }
        (*s).storage_size_= size;
    }
    return (*s).storage_;
}
unsafe extern "C" fn HashTableSize(
    mut max_table_size: size_t,
    mut input_size: size_t,
) -> size_t {
    let mut htsize = 256 as libc::c_int as size_t;
    while htsize < max_table_size && htsize < input_size {
        htsize<<= 1 as libc::c_int;
    }
    return htsize;
}
unsafe extern "C" fn GetHashTable(
    mut s: *mut BrotliEncoderState,
    mut quality: libc::c_int,
    mut input_size: size_t,
    mut table_size: Option<&mut size_t>,
) -> *mut libc::c_int {
    let mut m: *mut crate::src::enc::backward_references_hq::MemoryManager = core::ptr::addr_of_mut!((*s).memory_manager_);
    let max_table_size = MaxHashTableSize(quality);
    let mut htsize = HashTableSize(max_table_size, input_size);
    let mut table = 0 as *mut libc::c_int;
    if quality == 0 as libc::c_int {
        if htsize & 0xaaaaa as libc::c_int as libc::c_ulong
            == 0 as libc::c_int as libc::c_ulong
        {
            htsize<<= 1 as libc::c_int;
        }
    }
    if htsize
        <= (::std::mem::size_of::<[libc::c_int; 1024]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
    {
        table= (*s).small_table_.as_mut_ptr();
    } else {
        if htsize > (*s).large_table_size_ {
            (*s).large_table_size_= htsize;
            crate::src::enc::memory::BrotliFree(m, (*s).large_table_ as *mut libc::c_void);
            (*s).large_table_= 0 as *mut libc::c_int;
            (*s).large_table_= if htsize > 0 as libc::c_int as libc::c_ulong {
                crate::src::enc::memory::BrotliAllocate(
                    m,
                    htsize
                        .wrapping_mul(
                            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                        ),
                ) as *mut libc::c_int
            } else {
                0 as *mut libc::c_int
            };
            if 0 as libc::c_int != 0 || 0 as libc::c_int != 0 {
                return 0 as *mut libc::c_int;
            }
        }
        table= (*s).large_table_;
    }
    *table_size.as_deref_mut().unwrap()= htsize;
    memset(
        table as *mut libc::c_void,
        0 as libc::c_int,
        htsize.wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    return table;
}
unsafe extern "C" fn EncodeWindowBits(
    mut lgwin: libc::c_int,
    mut large_window: libc::c_int,
    mut last_bytes: *mut uint16_t,
    mut last_bytes_bits: *mut uint8_t,
) {
    if large_window != 0 {
        *last_bytes= ((lgwin & 0x3f as libc::c_int) << 8 as libc::c_int
            | 0x11 as libc::c_int) as uint16_t;
        *last_bytes_bits= 14 as libc::c_int as uint8_t;
    } else if lgwin == 16 as libc::c_int {
        *last_bytes= 0 as libc::c_int as uint16_t;
        *last_bytes_bits= 1 as libc::c_int as uint8_t;
    } else if lgwin == 17 as libc::c_int {
        *last_bytes= 1 as libc::c_int as uint16_t;
        *last_bytes_bits= 7 as libc::c_int as uint8_t;
    } else if lgwin > 17 as libc::c_int {
        *last_bytes= ((lgwin - 17 as libc::c_int) << 1 as libc::c_int
            | 0x1 as libc::c_int) as uint16_t;
        *last_bytes_bits= 4 as libc::c_int as uint8_t;
    } else {
        *last_bytes= ((lgwin - 8 as libc::c_int) << 4 as libc::c_int
            | 0x1 as libc::c_int) as uint16_t;
        *last_bytes_bits= 7 as libc::c_int as uint8_t;
    };
}
unsafe extern "C" fn InitCommandPrefixCodes(
    mut cmd_depths: *mut uint8_t,
    mut cmd_bits: *mut uint16_t,
    mut cmd_code: *mut uint8_t,
    mut cmd_code_numbits: *mut size_t,
) {
    static mut kDefaultCommandDepths: [uint8_t; 128] = [
        0 as libc::c_int as uint8_t,
        4 as libc::c_int as uint8_t,
        4 as libc::c_int as uint8_t,
        5 as libc::c_int as uint8_t,
        6 as libc::c_int as uint8_t,
        6 as libc::c_int as uint8_t,
        7 as libc::c_int as uint8_t,
        7 as libc::c_int as uint8_t,
        7 as libc::c_int as uint8_t,
        7 as libc::c_int as uint8_t,
        7 as libc::c_int as uint8_t,
        8 as libc::c_int as uint8_t,
        8 as libc::c_int as uint8_t,
        8 as libc::c_int as uint8_t,
        8 as libc::c_int as uint8_t,
        8 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        4 as libc::c_int as uint8_t,
        4 as libc::c_int as uint8_t,
        4 as libc::c_int as uint8_t,
        4 as libc::c_int as uint8_t,
        4 as libc::c_int as uint8_t,
        5 as libc::c_int as uint8_t,
        5 as libc::c_int as uint8_t,
        6 as libc::c_int as uint8_t,
        6 as libc::c_int as uint8_t,
        6 as libc::c_int as uint8_t,
        6 as libc::c_int as uint8_t,
        7 as libc::c_int as uint8_t,
        7 as libc::c_int as uint8_t,
        7 as libc::c_int as uint8_t,
        7 as libc::c_int as uint8_t,
        10 as libc::c_int as uint8_t,
        10 as libc::c_int as uint8_t,
        10 as libc::c_int as uint8_t,
        10 as libc::c_int as uint8_t,
        10 as libc::c_int as uint8_t,
        10 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        4 as libc::c_int as uint8_t,
        4 as libc::c_int as uint8_t,
        5 as libc::c_int as uint8_t,
        5 as libc::c_int as uint8_t,
        5 as libc::c_int as uint8_t,
        6 as libc::c_int as uint8_t,
        6 as libc::c_int as uint8_t,
        7 as libc::c_int as uint8_t,
        8 as libc::c_int as uint8_t,
        8 as libc::c_int as uint8_t,
        9 as libc::c_int as uint8_t,
        10 as libc::c_int as uint8_t,
        10 as libc::c_int as uint8_t,
        10 as libc::c_int as uint8_t,
        10 as libc::c_int as uint8_t,
        10 as libc::c_int as uint8_t,
        10 as libc::c_int as uint8_t,
        10 as libc::c_int as uint8_t,
        10 as libc::c_int as uint8_t,
        10 as libc::c_int as uint8_t,
        10 as libc::c_int as uint8_t,
        10 as libc::c_int as uint8_t,
        10 as libc::c_int as uint8_t,
        5 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        6 as libc::c_int as uint8_t,
        6 as libc::c_int as uint8_t,
        6 as libc::c_int as uint8_t,
        6 as libc::c_int as uint8_t,
        6 as libc::c_int as uint8_t,
        6 as libc::c_int as uint8_t,
        5 as libc::c_int as uint8_t,
        5 as libc::c_int as uint8_t,
        5 as libc::c_int as uint8_t,
        5 as libc::c_int as uint8_t,
        5 as libc::c_int as uint8_t,
        5 as libc::c_int as uint8_t,
        4 as libc::c_int as uint8_t,
        4 as libc::c_int as uint8_t,
        4 as libc::c_int as uint8_t,
        4 as libc::c_int as uint8_t,
        4 as libc::c_int as uint8_t,
        4 as libc::c_int as uint8_t,
        4 as libc::c_int as uint8_t,
        5 as libc::c_int as uint8_t,
        5 as libc::c_int as uint8_t,
        5 as libc::c_int as uint8_t,
        5 as libc::c_int as uint8_t,
        5 as libc::c_int as uint8_t,
        5 as libc::c_int as uint8_t,
        6 as libc::c_int as uint8_t,
        6 as libc::c_int as uint8_t,
        7 as libc::c_int as uint8_t,
        7 as libc::c_int as uint8_t,
        7 as libc::c_int as uint8_t,
        8 as libc::c_int as uint8_t,
        10 as libc::c_int as uint8_t,
        12 as libc::c_int as uint8_t,
        12 as libc::c_int as uint8_t,
        12 as libc::c_int as uint8_t,
        12 as libc::c_int as uint8_t,
        12 as libc::c_int as uint8_t,
        12 as libc::c_int as uint8_t,
        12 as libc::c_int as uint8_t,
        12 as libc::c_int as uint8_t,
        12 as libc::c_int as uint8_t,
        12 as libc::c_int as uint8_t,
        12 as libc::c_int as uint8_t,
        12 as libc::c_int as uint8_t,
        0,
        0,
        0,
        0,
    ];
    static mut kDefaultCommandBits: [uint16_t; 128] = [
        0 as libc::c_int as uint16_t,
        0 as libc::c_int as uint16_t,
        8 as libc::c_int as uint16_t,
        9 as libc::c_int as uint16_t,
        3 as libc::c_int as uint16_t,
        35 as libc::c_int as uint16_t,
        7 as libc::c_int as uint16_t,
        71 as libc::c_int as uint16_t,
        39 as libc::c_int as uint16_t,
        103 as libc::c_int as uint16_t,
        23 as libc::c_int as uint16_t,
        47 as libc::c_int as uint16_t,
        175 as libc::c_int as uint16_t,
        111 as libc::c_int as uint16_t,
        239 as libc::c_int as uint16_t,
        31 as libc::c_int as uint16_t,
        0 as libc::c_int as uint16_t,
        0 as libc::c_int as uint16_t,
        0 as libc::c_int as uint16_t,
        4 as libc::c_int as uint16_t,
        12 as libc::c_int as uint16_t,
        2 as libc::c_int as uint16_t,
        10 as libc::c_int as uint16_t,
        6 as libc::c_int as uint16_t,
        13 as libc::c_int as uint16_t,
        29 as libc::c_int as uint16_t,
        11 as libc::c_int as uint16_t,
        43 as libc::c_int as uint16_t,
        27 as libc::c_int as uint16_t,
        59 as libc::c_int as uint16_t,
        87 as libc::c_int as uint16_t,
        55 as libc::c_int as uint16_t,
        15 as libc::c_int as uint16_t,
        79 as libc::c_int as uint16_t,
        319 as libc::c_int as uint16_t,
        831 as libc::c_int as uint16_t,
        191 as libc::c_int as uint16_t,
        703 as libc::c_int as uint16_t,
        447 as libc::c_int as uint16_t,
        959 as libc::c_int as uint16_t,
        0 as libc::c_int as uint16_t,
        14 as libc::c_int as uint16_t,
        1 as libc::c_int as uint16_t,
        25 as libc::c_int as uint16_t,
        5 as libc::c_int as uint16_t,
        21 as libc::c_int as uint16_t,
        19 as libc::c_int as uint16_t,
        51 as libc::c_int as uint16_t,
        119 as libc::c_int as uint16_t,
        159 as libc::c_int as uint16_t,
        95 as libc::c_int as uint16_t,
        223 as libc::c_int as uint16_t,
        479 as libc::c_int as uint16_t,
        991 as libc::c_int as uint16_t,
        63 as libc::c_int as uint16_t,
        575 as libc::c_int as uint16_t,
        127 as libc::c_int as uint16_t,
        639 as libc::c_int as uint16_t,
        383 as libc::c_int as uint16_t,
        895 as libc::c_int as uint16_t,
        255 as libc::c_int as uint16_t,
        767 as libc::c_int as uint16_t,
        511 as libc::c_int as uint16_t,
        1023 as libc::c_int as uint16_t,
        14 as libc::c_int as uint16_t,
        0 as libc::c_int as uint16_t,
        0 as libc::c_int as uint16_t,
        0 as libc::c_int as uint16_t,
        0 as libc::c_int as uint16_t,
        0 as libc::c_int as uint16_t,
        0 as libc::c_int as uint16_t,
        0 as libc::c_int as uint16_t,
        0 as libc::c_int as uint16_t,
        0 as libc::c_int as uint16_t,
        0 as libc::c_int as uint16_t,
        0 as libc::c_int as uint16_t,
        0 as libc::c_int as uint16_t,
        0 as libc::c_int as uint16_t,
        0 as libc::c_int as uint16_t,
        0 as libc::c_int as uint16_t,
        27 as libc::c_int as uint16_t,
        59 as libc::c_int as uint16_t,
        7 as libc::c_int as uint16_t,
        39 as libc::c_int as uint16_t,
        23 as libc::c_int as uint16_t,
        55 as libc::c_int as uint16_t,
        30 as libc::c_int as uint16_t,
        1 as libc::c_int as uint16_t,
        17 as libc::c_int as uint16_t,
        9 as libc::c_int as uint16_t,
        25 as libc::c_int as uint16_t,
        5 as libc::c_int as uint16_t,
        0 as libc::c_int as uint16_t,
        8 as libc::c_int as uint16_t,
        4 as libc::c_int as uint16_t,
        12 as libc::c_int as uint16_t,
        2 as libc::c_int as uint16_t,
        10 as libc::c_int as uint16_t,
        6 as libc::c_int as uint16_t,
        21 as libc::c_int as uint16_t,
        13 as libc::c_int as uint16_t,
        29 as libc::c_int as uint16_t,
        3 as libc::c_int as uint16_t,
        19 as libc::c_int as uint16_t,
        11 as libc::c_int as uint16_t,
        15 as libc::c_int as uint16_t,
        47 as libc::c_int as uint16_t,
        31 as libc::c_int as uint16_t,
        95 as libc::c_int as uint16_t,
        63 as libc::c_int as uint16_t,
        127 as libc::c_int as uint16_t,
        255 as libc::c_int as uint16_t,
        767 as libc::c_int as uint16_t,
        2815 as libc::c_int as uint16_t,
        1791 as libc::c_int as uint16_t,
        3839 as libc::c_int as uint16_t,
        511 as libc::c_int as uint16_t,
        2559 as libc::c_int as uint16_t,
        1535 as libc::c_int as uint16_t,
        3583 as libc::c_int as uint16_t,
        1023 as libc::c_int as uint16_t,
        3071 as libc::c_int as uint16_t,
        2047 as libc::c_int as uint16_t,
        4095 as libc::c_int as uint16_t,
        0,
        0,
        0,
        0,
    ];
    static mut kDefaultCommandCode: [uint8_t; 57] = [
        0xff as libc::c_int as uint8_t,
        0x77 as libc::c_int as uint8_t,
        0xd5 as libc::c_int as uint8_t,
        0xbf as libc::c_int as uint8_t,
        0xe7 as libc::c_int as uint8_t,
        0xde as libc::c_int as uint8_t,
        0xea as libc::c_int as uint8_t,
        0x9e as libc::c_int as uint8_t,
        0x51 as libc::c_int as uint8_t,
        0x5d as libc::c_int as uint8_t,
        0xde as libc::c_int as uint8_t,
        0xc6 as libc::c_int as uint8_t,
        0x70 as libc::c_int as uint8_t,
        0x57 as libc::c_int as uint8_t,
        0xbc as libc::c_int as uint8_t,
        0x58 as libc::c_int as uint8_t,
        0x58 as libc::c_int as uint8_t,
        0x58 as libc::c_int as uint8_t,
        0xd8 as libc::c_int as uint8_t,
        0xd8 as libc::c_int as uint8_t,
        0x58 as libc::c_int as uint8_t,
        0xd5 as libc::c_int as uint8_t,
        0xcb as libc::c_int as uint8_t,
        0x8c as libc::c_int as uint8_t,
        0xea as libc::c_int as uint8_t,
        0xe0 as libc::c_int as uint8_t,
        0xc3 as libc::c_int as uint8_t,
        0x87 as libc::c_int as uint8_t,
        0x1f as libc::c_int as uint8_t,
        0x83 as libc::c_int as uint8_t,
        0xc1 as libc::c_int as uint8_t,
        0x60 as libc::c_int as uint8_t,
        0x1c as libc::c_int as uint8_t,
        0x67 as libc::c_int as uint8_t,
        0xb2 as libc::c_int as uint8_t,
        0xaa as libc::c_int as uint8_t,
        0x6 as libc::c_int as uint8_t,
        0x83 as libc::c_int as uint8_t,
        0xc1 as libc::c_int as uint8_t,
        0x60 as libc::c_int as uint8_t,
        0x30 as libc::c_int as uint8_t,
        0x18 as libc::c_int as uint8_t,
        0xcc as libc::c_int as uint8_t,
        0xa1 as libc::c_int as uint8_t,
        0xce as libc::c_int as uint8_t,
        0x88 as libc::c_int as uint8_t,
        0x54 as libc::c_int as uint8_t,
        0x94 as libc::c_int as uint8_t,
        0x46 as libc::c_int as uint8_t,
        0xe1 as libc::c_int as uint8_t,
        0xb0 as libc::c_int as uint8_t,
        0xd0 as libc::c_int as uint8_t,
        0x4e as libc::c_int as uint8_t,
        0xb2 as libc::c_int as uint8_t,
        0xf7 as libc::c_int as uint8_t,
        0x4 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
    ];
    static mut kDefaultCommandCodeNumBits: size_t = 448 as libc::c_int as size_t;
    memcpy(
        cmd_depths as *mut libc::c_void,
        kDefaultCommandDepths.as_ptr() as *const libc::c_void,
        ::std::mem::size_of::<[uint8_t; 128]>() as libc::c_ulong,
    );
    memcpy(
        cmd_bits as *mut libc::c_void,
        kDefaultCommandBits.as_ptr() as *const libc::c_void,
        ::std::mem::size_of::<[uint16_t; 128]>() as libc::c_ulong,
    );
    memcpy(
        cmd_code as *mut libc::c_void,
        kDefaultCommandCode.as_ptr() as *const libc::c_void,
        ::std::mem::size_of::<[uint8_t; 57]>() as libc::c_ulong,
    );
    *cmd_code_numbits= kDefaultCommandCodeNumBits;
}
unsafe extern "C" fn ChooseContextMap(
    mut quality: libc::c_int,
    mut bigram_histo: *mut uint32_t,
    mut num_literal_contexts: Option<&mut size_t>,
    mut literal_context_map: Option<&mut *const uint32_t>,
) {
    static mut kStaticContextMapContinuation: [uint32_t; 64] = [
        1 as libc::c_int as uint32_t,
        1 as libc::c_int as uint32_t,
        2 as libc::c_int as uint32_t,
        2 as libc::c_int as uint32_t,
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
    ];
    static mut kStaticContextMapSimpleUTF8: [uint32_t; 64] = [
        0 as libc::c_int as uint32_t,
        0 as libc::c_int as uint32_t,
        1 as libc::c_int as uint32_t,
        1 as libc::c_int as uint32_t,
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
    ];
    let mut monogram_histo: [uint32_t; 3] = [0 as libc::c_int as uint32_t, 0, 0];
    let mut two_prefix_histo: [uint32_t; 6] = [
        0 as libc::c_int as uint32_t,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut total: size_t = 0;
    let mut i: size_t = 0;
    let mut dummy: size_t = 0;
    let mut entropy: [libc::c_double; 4] = [0.; 4];
    i= 0 as libc::c_int as size_t;
    while i < 9 as libc::c_int as libc::c_ulong {
        monogram_histo[i.wrapping_rem(3 as libc::c_int as libc::c_ulong)
            as usize]= (monogram_histo[i.wrapping_rem(3 as libc::c_int as libc::c_ulong)
            as usize] as libc::c_uint)
            .wrapping_add(*bigram_histo.offset(i as isize)) as uint32_t as uint32_t;
        two_prefix_histo[i.wrapping_rem(6 as libc::c_int as libc::c_ulong)
            as usize]= (two_prefix_histo[i
            .wrapping_rem(6 as libc::c_int as libc::c_ulong) as usize] as libc::c_uint)
            .wrapping_add(*bigram_histo.offset(i as isize)) as uint32_t as uint32_t;
        i= i.wrapping_add(1);
    }
    entropy[1 as libc::c_int
        as usize]= ShannonEntropy(
        monogram_histo.as_mut_ptr(),
        3 as libc::c_int as size_t,
        Some(&mut dummy),
    );
    entropy[2 as libc::c_int
        as usize]= ShannonEntropy(
        two_prefix_histo.as_mut_ptr(),
        3 as libc::c_int as size_t,
        Some(&mut dummy),
    )
        + ShannonEntropy(
            two_prefix_histo.as_mut_ptr().offset(3 as libc::c_int as isize),
            3 as libc::c_int as size_t,
            Some(&mut dummy),
        );
    entropy[3 as libc::c_int as usize]= 0 as libc::c_int as libc::c_double;
    i= 0 as libc::c_int as size_t;
    while i < 3 as libc::c_int as libc::c_ulong {
        entropy[3 as libc::c_int as usize]+= ShannonEntropy(
                bigram_histo
                    .offset(
                        (3 as libc::c_int as libc::c_ulong).wrapping_mul(i) as isize,
                    ),
                3 as libc::c_int as size_t,
                Some(&mut dummy),
            );
        i= i.wrapping_add(1);
    }
    total= monogram_histo[0 as libc::c_int as usize]
        .wrapping_add(monogram_histo[1 as libc::c_int as usize])
        .wrapping_add(monogram_histo[2 as libc::c_int as usize]) as size_t;
    entropy[0 as libc::c_int as usize]= 1.0f64 / total as libc::c_double;
    entropy[1 as libc::c_int as usize]*= entropy[0 as libc::c_int as usize];
    entropy[2 as libc::c_int as usize]*= entropy[0 as libc::c_int as usize];
    entropy[3 as libc::c_int as usize]*= entropy[0 as libc::c_int as usize];
    if quality < 7 as libc::c_int {
        entropy[3 as libc::c_int
            as usize]= entropy[1 as libc::c_int as usize]
            * 10 as libc::c_int as libc::c_double;
    }
    if entropy[1 as libc::c_int as usize] - entropy[2 as libc::c_int as usize] < 0.2f64
        && entropy[1 as libc::c_int as usize] - entropy[3 as libc::c_int as usize]
            < 0.2f64
    {
        *num_literal_contexts.as_deref_mut().unwrap()= 1 as libc::c_int as size_t;
    } else if entropy[2 as libc::c_int as usize] - entropy[3 as libc::c_int as usize]
        < 0.02f64
    {
        *num_literal_contexts.as_deref_mut().unwrap()= 2 as libc::c_int as size_t;
        *literal_context_map.as_deref_mut().unwrap()= kStaticContextMapSimpleUTF8.as_ptr();
    } else {
        *num_literal_contexts.as_deref_mut().unwrap()= 3 as libc::c_int as size_t;
        *literal_context_map.as_deref_mut().unwrap()= kStaticContextMapContinuation.as_ptr();
    };
}
unsafe extern "C" fn ShouldUseComplexStaticContextMap(
    mut input: *const uint8_t,
    mut start_pos: size_t,
    mut length: size_t,
    mut mask: size_t,
    mut quality: libc::c_int,
    mut size_hint: size_t,
    mut num_literal_contexts: Option<&mut size_t>,
    mut literal_context_map: Option<&mut *const uint32_t>,
) -> libc::c_int {
    static mut kStaticContextMapComplexUTF8: [uint32_t; 64] = [
        11 as libc::c_int as uint32_t,
        11 as libc::c_int as uint32_t,
        12 as libc::c_int as uint32_t,
        12 as libc::c_int as uint32_t,
        0 as libc::c_int as uint32_t,
        0 as libc::c_int as uint32_t,
        0 as libc::c_int as uint32_t,
        0 as libc::c_int as uint32_t,
        1 as libc::c_int as uint32_t,
        1 as libc::c_int as uint32_t,
        9 as libc::c_int as uint32_t,
        9 as libc::c_int as uint32_t,
        2 as libc::c_int as uint32_t,
        2 as libc::c_int as uint32_t,
        2 as libc::c_int as uint32_t,
        2 as libc::c_int as uint32_t,
        1 as libc::c_int as uint32_t,
        1 as libc::c_int as uint32_t,
        1 as libc::c_int as uint32_t,
        1 as libc::c_int as uint32_t,
        8 as libc::c_int as uint32_t,
        3 as libc::c_int as uint32_t,
        3 as libc::c_int as uint32_t,
        3 as libc::c_int as uint32_t,
        1 as libc::c_int as uint32_t,
        1 as libc::c_int as uint32_t,
        1 as libc::c_int as uint32_t,
        1 as libc::c_int as uint32_t,
        2 as libc::c_int as uint32_t,
        2 as libc::c_int as uint32_t,
        2 as libc::c_int as uint32_t,
        2 as libc::c_int as uint32_t,
        8 as libc::c_int as uint32_t,
        4 as libc::c_int as uint32_t,
        4 as libc::c_int as uint32_t,
        4 as libc::c_int as uint32_t,
        8 as libc::c_int as uint32_t,
        7 as libc::c_int as uint32_t,
        4 as libc::c_int as uint32_t,
        4 as libc::c_int as uint32_t,
        8 as libc::c_int as uint32_t,
        0 as libc::c_int as uint32_t,
        0 as libc::c_int as uint32_t,
        0 as libc::c_int as uint32_t,
        3 as libc::c_int as uint32_t,
        3 as libc::c_int as uint32_t,
        3 as libc::c_int as uint32_t,
        3 as libc::c_int as uint32_t,
        5 as libc::c_int as uint32_t,
        5 as libc::c_int as uint32_t,
        10 as libc::c_int as uint32_t,
        5 as libc::c_int as uint32_t,
        5 as libc::c_int as uint32_t,
        5 as libc::c_int as uint32_t,
        10 as libc::c_int as uint32_t,
        5 as libc::c_int as uint32_t,
        6 as libc::c_int as uint32_t,
        6 as libc::c_int as uint32_t,
        6 as libc::c_int as uint32_t,
        6 as libc::c_int as uint32_t,
        6 as libc::c_int as uint32_t,
        6 as libc::c_int as uint32_t,
        6 as libc::c_int as uint32_t,
        6 as libc::c_int as uint32_t,
    ];
    if size_hint < ((1 as libc::c_int) << 20 as libc::c_int) as libc::c_ulong {
        return 0 as libc::c_int
    } else {
        let end_pos = start_pos.wrapping_add(length);
        let mut combined_histo: [uint32_t; 32] = [
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
        ];
        let mut context_histo: [[uint32_t; 32]; 13] = [
            [
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
            ],
            [0; 32],
            [0; 32],
            [0; 32],
            [0; 32],
            [0; 32],
            [0; 32],
            [0; 32],
            [0; 32],
            [0; 32],
            [0; 32],
            [0; 32],
            [0; 32],
        ];
        let mut total = 0 as libc::c_int as uint32_t;
        let mut entropy: [libc::c_double; 3] = [0.; 3];
        let mut dummy: size_t = 0;
        let mut i: size_t = 0;
        let mut utf8_lut: ContextLut = &*_kBrotliContextLookupTable
            .as_ptr()
            .offset(((CONTEXT_UTF8 as libc::c_int) << 9 as libc::c_int) as isize)
            as *const uint8_t;
        while start_pos.wrapping_add(64 as libc::c_int as libc::c_ulong) <= end_pos {
            let stride_end_pos = start_pos
                .wrapping_add(64 as libc::c_int as libc::c_ulong);
            let mut prev2 = *input.offset((start_pos & mask) as isize);
            let mut prev1 = *input
                .offset(
                    (start_pos.wrapping_add(1 as libc::c_int as libc::c_ulong) & mask)
                        as isize,
                );
            let mut pos: size_t = 0;
            pos= start_pos.wrapping_add(2 as libc::c_int as libc::c_ulong);
            while pos < stride_end_pos {
                let literal = *input.offset((pos & mask) as isize);
                let context = kStaticContextMapComplexUTF8[(*utf8_lut
                    .offset(prev1 as isize) as libc::c_int
                    | *utf8_lut
                        .offset(256 as libc::c_int as isize)
                        .offset(prev2 as isize) as libc::c_int) as usize] as uint8_t;
                total= total.wrapping_add(1);
                combined_histo[(literal as libc::c_int >> 3 as libc::c_int)
                    as usize]= combined_histo[(literal as libc::c_int
                    >> 3 as libc::c_int) as usize]
                    .wrapping_add(1);
                context_histo[context
                    as usize][(literal as libc::c_int >> 3 as libc::c_int)
                    as usize]= context_histo[context
                    as usize][(literal as libc::c_int >> 3 as libc::c_int) as usize]
                    .wrapping_add(1);
                prev2= prev1;
                prev1= literal;
                pos= pos.wrapping_add(1);
            }
            start_pos= (start_pos as libc::c_ulong)
                .wrapping_add(4096 as libc::c_int as libc::c_ulong) as size_t as size_t;
        }
        entropy[1 as libc::c_int
            as usize]= ShannonEntropy(
            combined_histo.as_mut_ptr(),
            32 as libc::c_int as size_t,
            Some(&mut dummy),
        );
        entropy[2 as libc::c_int as usize]= 0 as libc::c_int as libc::c_double;
        i= 0 as libc::c_int as size_t;
        while i < 13 as libc::c_int as libc::c_ulong {
            entropy[2 as libc::c_int as usize]+= ShannonEntropy(
                    core::ptr::addr_of_mut!(*(*context_histo.as_mut_ptr().offset(i as isize))
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize)),
                    32 as libc::c_int as size_t,
                    Some(&mut dummy),
                );
            i= i.wrapping_add(1);
        }
        entropy[0 as libc::c_int as usize]= 1.0f64 / total as libc::c_double;
        entropy[1 as libc::c_int as usize]*= entropy[0 as libc::c_int as usize];
        entropy[2 as libc::c_int as usize]*= entropy[0 as libc::c_int as usize];
        if entropy[2 as libc::c_int as usize] > 3.0f64
            || entropy[1 as libc::c_int as usize] - entropy[2 as libc::c_int as usize]
                < 0.2f64
        {
            return 0 as libc::c_int
        } else {
            *num_literal_contexts.as_deref_mut().unwrap()= 13 as libc::c_int as size_t;
            *literal_context_map.as_deref_mut().unwrap()= kStaticContextMapComplexUTF8.as_ptr();
            return 1 as libc::c_int;
        }
    };
}
unsafe extern "C" fn DecideOverLiteralContextModeling(
    mut input: *const uint8_t,
    mut start_pos: size_t,
    mut length: size_t,
    mut mask: size_t,
    mut quality: libc::c_int,
    mut size_hint: size_t,
    mut num_literal_contexts: Option<&mut size_t>,
    mut literal_context_map: Option<&mut *const uint32_t>,
) {
    if quality < 5 as libc::c_int || length < 64 as libc::c_int as libc::c_ulong {
        return
    } else {
        if !(ShouldUseComplexStaticContextMap(
            input,
            start_pos,
            length,
            mask,
            quality,
            size_hint,
            num_literal_contexts.as_deref_mut(),
            literal_context_map.as_deref_mut(),
        ) != 0)
        {
            let end_pos = start_pos.wrapping_add(length);
            let mut bigram_prefix_histo: [uint32_t; 9] = [
                0 as libc::c_int as uint32_t,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ];
            while start_pos.wrapping_add(64 as libc::c_int as libc::c_ulong) <= end_pos {
                static mut lut: [libc::c_int; 4] = [
                    0 as libc::c_int,
                    0 as libc::c_int,
                    1 as libc::c_int,
                    2 as libc::c_int,
                ];
                let stride_end_pos = start_pos
                    .wrapping_add(64 as libc::c_int as libc::c_ulong);
                let mut prev = lut[(*input.offset((start_pos & mask) as isize)
                    as libc::c_int >> 6 as libc::c_int) as usize] * 3 as libc::c_int;
                let mut pos: size_t = 0;
                pos= start_pos.wrapping_add(1 as libc::c_int as libc::c_ulong);
                while pos < stride_end_pos {
                    let literal = *input.offset((pos & mask) as isize);
                    bigram_prefix_histo[(prev
                        + lut[(literal as libc::c_int >> 6 as libc::c_int) as usize])
                        as usize]= bigram_prefix_histo[(prev
                        + lut[(literal as libc::c_int >> 6 as libc::c_int) as usize])
                        as usize]
                        .wrapping_add(1);
                    prev= lut[(literal as libc::c_int >> 6 as libc::c_int) as usize]
                        * 3 as libc::c_int;
                    pos= pos.wrapping_add(1);
                }
                start_pos= (start_pos as libc::c_ulong)
                    .wrapping_add(4096 as libc::c_int as libc::c_ulong) as size_t
                    as size_t;
            }
            ChooseContextMap(
                quality,
                core::ptr::addr_of_mut!(*bigram_prefix_histo.as_mut_ptr().offset(0 as libc::c_int as isize)),
                num_literal_contexts.as_deref_mut(),
                literal_context_map.as_deref_mut(),
            );
        }
    };
}
unsafe extern "C" fn ShouldCompress(
    mut data: *const uint8_t,
    mut mask: size_t,
    mut last_flush_pos: uint64_t,
    mut bytes: size_t,
    mut num_literals: size_t,
    mut num_commands: size_t,
) -> libc::c_int {
    if bytes <= 2 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int;
    }
    if num_commands
        < (bytes >> 8 as libc::c_int).wrapping_add(2 as libc::c_int as libc::c_ulong)
    {
        if num_literals as libc::c_double > 0.99f64 * bytes as libc::c_double {
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
            static mut kSampleRate: uint32_t = 13 as libc::c_int as uint32_t;
            static mut kMinEntropy: libc::c_double = 7.92f64;
            let bit_cost_threshold = bytes as libc::c_double * kMinEntropy
                / kSampleRate as libc::c_double;
            let mut t = bytes
                .wrapping_add(kSampleRate as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                .wrapping_div(kSampleRate as libc::c_ulong);
            let mut pos = last_flush_pos as uint32_t;
            let mut i: size_t = 0;
            i= 0 as libc::c_int as size_t;
            while i < t {
                literal_histo[*data.offset((pos as libc::c_ulong & mask) as isize)
                    as usize]= literal_histo[*data
                    .offset((pos as libc::c_ulong & mask) as isize) as usize]
                    .wrapping_add(1);
                pos= (pos as libc::c_uint).wrapping_add(kSampleRate) as uint32_t
                    as uint32_t;
                i= i.wrapping_add(1);
            }
            if BitsEntropy(literal_histo.as_mut_ptr(), 256 as libc::c_int as size_t)
                > bit_cost_threshold
            {
                return 0 as libc::c_int;
            }
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn ChooseContextMode(
    mut params: *const crate::src::enc::backward_references::BrotliEncoderParams,
    mut data: *const uint8_t,
    mut pos: size_t,
    mut mask: size_t,
    mut length: size_t,
) -> ContextType {
    if (*params).quality >= 10 as libc::c_int
        && crate::src::enc::utf8_util::BrotliIsMostlyUTF8(data, pos, mask, length, crate::src::enc::encode::kMinUTF8Ratio) == 0
    {
        return CONTEXT_SIGNED;
    }
    return CONTEXT_UTF8;
}
unsafe extern "C" fn WriteMetaBlockInternal(
    mut m: *mut crate::src::enc::backward_references_hq::MemoryManager,
    mut data: *const uint8_t,
    mut mask: size_t,
    mut last_flush_pos: uint64_t,
    mut bytes: size_t,
    mut is_last: libc::c_int,
    mut literal_context_mode: ContextType,
    mut params: *const crate::src::enc::backward_references::BrotliEncoderParams,
    mut prev_byte: uint8_t,
    mut prev_byte2: uint8_t,
    mut num_literals: size_t,
    mut num_commands: size_t,
    mut commands: *mut crate::src::enc::backward_references::Command,
    mut saved_dist_cache: *const libc::c_int,
    mut dist_cache: *mut libc::c_int,
    mut storage_ix: *mut size_t,
    mut storage: *mut uint8_t,
) {
    let wrapped_last_flush_pos = WrapPosition(last_flush_pos);
    let mut last_bytes: uint16_t = 0;
    let mut last_bytes_bits: uint8_t = 0;
    let mut literal_context_lut: ContextLut = &*_kBrotliContextLookupTable
        .as_ptr()
        .offset(((literal_context_mode as libc::c_uint) << 9 as libc::c_int) as isize)
        as *const uint8_t;
    let mut block_params = (*params);
    if bytes == 0 as libc::c_int as libc::c_ulong {
        BrotliWriteBits(
            2 as libc::c_int as size_t,
            3 as libc::c_int as uint64_t,
            storage_ix,
            storage,
        );
        *storage_ix= (*storage_ix).wrapping_add(7 as libc::c_uint as libc::c_ulong)
            & !(7 as libc::c_uint) as libc::c_ulong;
        return;
    }
    if ShouldCompress(data, mask, last_flush_pos, bytes, num_literals, num_commands) == 0
    {
        memcpy(
            dist_cache as *mut libc::c_void,
            saved_dist_cache as *const libc::c_void,
            (4 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
        crate::src::enc::brotli_bit_stream::BrotliStoreUncompressedMetaBlock(
            is_last,
            data,
            wrapped_last_flush_pos as size_t,
            mask,
            bytes,
            storage_ix,
            storage,
        );
        return;
    }
    last_bytes= ((*storage.offset(1 as libc::c_int as isize) as libc::c_int)
        << 8 as libc::c_int | *storage.offset(0 as libc::c_int as isize) as libc::c_int)
        as uint16_t;
    last_bytes_bits= (*storage_ix) as uint8_t;
    if (*params).quality <= 2 as libc::c_int {
        crate::src::enc::brotli_bit_stream::BrotliStoreMetaBlockFast(
            m,
            data,
            wrapped_last_flush_pos as size_t,
            bytes,
            mask,
            is_last,
            params,
            commands,
            num_commands,
            storage_ix,
            storage,
        );
        if 0 as libc::c_int != 0 {
            return;
        }
    } else if (*params).quality < 4 as libc::c_int {
        crate::src::enc::brotli_bit_stream::BrotliStoreMetaBlockTrivial(
            m,
            data,
            wrapped_last_flush_pos as size_t,
            bytes,
            mask,
            is_last,
            params,
            commands,
            num_commands,
            storage_ix,
            storage,
        );
        if 0 as libc::c_int != 0 {
            return;
        }
    } else {
        let mut mb = crate::src::enc::brotli_bit_stream::MetaBlockSplit {
            literal_split: crate::src::enc::block_splitter::BlockSplit {
                num_types: 0,
                num_blocks: 0,
                types: 0 as *mut uint8_t,
                lengths: 0 as *mut uint32_t,
                types_alloc_size: 0,
                lengths_alloc_size: 0,
            },
            command_split: crate::src::enc::block_splitter::BlockSplit {
                num_types: 0,
                num_blocks: 0,
                types: 0 as *mut uint8_t,
                lengths: 0 as *mut uint32_t,
                types_alloc_size: 0,
                lengths_alloc_size: 0,
            },
            distance_split: crate::src::enc::block_splitter::BlockSplit {
                num_types: 0,
                num_blocks: 0,
                types: 0 as *mut uint8_t,
                lengths: 0 as *mut uint32_t,
                types_alloc_size: 0,
                lengths_alloc_size: 0,
            },
            literal_context_map: 0 as *mut uint32_t,
            literal_context_map_size: 0,
            distance_context_map: 0 as *mut uint32_t,
            distance_context_map_size: 0,
            literal_histograms: 0 as *mut crate::src::enc::bit_cost::HistogramLiteral,
            literal_histograms_size: 0,
            command_histograms: 0 as *mut crate::src::enc::bit_cost::HistogramCommand,
            command_histograms_size: 0,
            distance_histograms: 0 as *mut crate::src::enc::bit_cost::HistogramDistance,
            distance_histograms_size: 0,
        };
        InitMetaBlockSplit(Some(&mut mb));
        if (*params).quality < 10 as libc::c_int {
            let mut num_literal_contexts = 1 as libc::c_int as size_t;
            let mut literal_context_map = 0 as *const uint32_t;
            if (*params).disable_literal_context_modeling == 0 {
                DecideOverLiteralContextModeling(
                    data,
                    wrapped_last_flush_pos as size_t,
                    bytes,
                    mask,
                    (*params).quality,
                    (*params).size_hint,
                    Some(&mut num_literal_contexts),
                    Some(&mut literal_context_map),
                );
            }
            crate::src::enc::metablock::BrotliBuildMetaBlockGreedy(
                m,
                data,
                wrapped_last_flush_pos as size_t,
                mask,
                prev_byte,
                prev_byte2,
                literal_context_lut,
                num_literal_contexts,
                literal_context_map,
                commands,
                num_commands,
                core::ptr::addr_of_mut!(mb),
            );
            if 0 as libc::c_int != 0 {
                return;
            }
        } else {
            crate::src::enc::metablock::BrotliBuildMetaBlock(
                m,
                data,
                wrapped_last_flush_pos as size_t,
                mask,
                Some(&mut block_params),
                prev_byte,
                prev_byte2,
                commands,
                num_commands,
                literal_context_mode,
                core::ptr::addr_of_mut!(mb),
            );
            if 0 as libc::c_int != 0 {
                return;
            }
        }
        if (*params).quality >= 4 as libc::c_int {
            crate::src::enc::metablock::BrotliOptimizeHistograms(block_params.dist.alphabet_size_limit, Some(&mut mb));
        }
        crate::src::enc::brotli_bit_stream::BrotliStoreMetaBlock(
            m,
            data,
            wrapped_last_flush_pos as size_t,
            bytes,
            mask,
            prev_byte,
            prev_byte2,
            is_last,
            core::ptr::addr_of!(block_params),
            literal_context_mode,
            commands,
            num_commands,
            core::ptr::addr_of!(mb),
            storage_ix,
            storage,
        );
        if 0 as libc::c_int != 0 {
            return;
        }
        DestroyMetaBlockSplit(m, core::ptr::addr_of_mut!(mb));
    }
    if bytes.wrapping_add(4 as libc::c_int as libc::c_ulong)
        < (*storage_ix) >> 3 as libc::c_int
    {
        memcpy(
            dist_cache as *mut libc::c_void,
            saved_dist_cache as *const libc::c_void,
            (4 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
        *storage.offset(0 as libc::c_int as isize) = last_bytes as uint8_t;
        *storage
            .offset(
                1 as libc::c_int as isize,
            ) = (last_bytes as libc::c_int >> 8 as libc::c_int) as uint8_t;
        *storage_ix= last_bytes_bits as size_t;
        crate::src::enc::brotli_bit_stream::BrotliStoreUncompressedMetaBlock(
            is_last,
            data,
            wrapped_last_flush_pos as size_t,
            mask,
            bytes,
            storage_ix,
            storage,
        );
    }
}
unsafe extern "C" fn ChooseDistanceParams(mut params: Option<&mut crate::src::enc::backward_references::BrotliEncoderParams>) {
    let mut distance_postfix_bits = 0 as libc::c_int as uint32_t;
    let mut num_direct_distance_codes = 0 as libc::c_int as uint32_t;
    if (*params.as_deref().unwrap()).quality >= 4 as libc::c_int {
        let mut ndirect_msb: uint32_t = 0;
        if (*params.as_deref().unwrap()).mode as libc::c_uint
            == BROTLI_MODE_FONT as libc::c_int as libc::c_uint
        {
            distance_postfix_bits= 1 as libc::c_int as uint32_t;
            num_direct_distance_codes= 12 as libc::c_int as uint32_t;
        } else {
            distance_postfix_bits= (*params.as_deref().unwrap()).dist.distance_postfix_bits;
            num_direct_distance_codes= (*params.as_deref().unwrap()).dist.num_direct_distance_codes;
        }
        ndirect_msb= num_direct_distance_codes >> distance_postfix_bits
            & 0xf as libc::c_int as libc::c_uint;
        if distance_postfix_bits > 3 as libc::c_int as libc::c_uint
            || num_direct_distance_codes > 120 as libc::c_int as libc::c_uint
            || ndirect_msb << distance_postfix_bits != num_direct_distance_codes
        {
            distance_postfix_bits= 0 as libc::c_int as uint32_t;
            num_direct_distance_codes= 0 as libc::c_int as uint32_t;
        }
    }
    crate::src::enc::metablock::BrotliInitDistanceParams(params.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()), distance_postfix_bits, num_direct_distance_codes);
}
unsafe extern "C" fn EnsureInitialized(mut s: *mut BrotliEncoderState) -> libc::c_int {
    if 0 as libc::c_int != 0 {
        return 0 as libc::c_int;
    }
    if (*s).is_initialized_ != 0 {
        return 1 as libc::c_int;
    }
    (*s).last_bytes_bits_= 0 as libc::c_int as uint8_t;
    (*s).last_bytes_= 0 as libc::c_int as uint16_t;
    (*s).flint_= BROTLI_FLINT_DONE as libc::c_int as int8_t;
    (*s).remaining_metadata_bytes_= !(0 as libc::c_int as uint32_t);
    SanitizeParams(Some(&mut (*s).params));
    (*s).params.lgblock= ComputeLgBlock(core::ptr::addr_of!((*s).params));
    ChooseDistanceParams(Some(&mut (*s).params));
    if (*s).params.stream_offset != 0 as libc::c_int as libc::c_ulong {
        (*s).flint_= BROTLI_FLINT_NEEDS_2_BYTES as libc::c_int as int8_t;
        (*s).dist_cache_[0 as libc::c_int as usize]= -(16 as libc::c_int);
        (*s).dist_cache_[1 as libc::c_int as usize]= -(16 as libc::c_int);
        (*s).dist_cache_[2 as libc::c_int as usize]= -(16 as libc::c_int);
        (*s).dist_cache_[3 as libc::c_int as usize]= -(16 as libc::c_int);
        memcpy(
            (*s).saved_dist_cache_.as_mut_ptr() as *mut libc::c_void,
            (*s).dist_cache_.as_mut_ptr() as *const libc::c_void,
            ::std::mem::size_of::<[libc::c_int; 4]>() as libc::c_ulong,
        );
    }
    RingBufferSetup(core::ptr::addr_of!((*s).params), core::ptr::addr_of_mut!((*s).ringbuffer_));
    let mut lgwin = (*s).params.lgwin;
    if (*s).params.quality == 0 as libc::c_int || (*s).params.quality == 1 as libc::c_int
    {
        lgwin= brotli_max_int(lgwin, 18 as libc::c_int);
    }
    if (*s).params.stream_offset == 0 as libc::c_int as libc::c_ulong {
        EncodeWindowBits(
            lgwin,
            (*s).params.large_window,
            core::ptr::addr_of_mut!((*s).last_bytes_),
            core::ptr::addr_of_mut!((*s).last_bytes_bits_),
        );
    } else {
        (*s).params.stream_offset= brotli_min_size_t(
            (*s).params.stream_offset,
            ((1 as libc::c_int as size_t) << lgwin)
                .wrapping_sub(16 as libc::c_int as libc::c_ulong),
        );
    }
    if (*s).params.quality == 0 as libc::c_int {
        InitCommandPrefixCodes(
            (*s).cmd_depths_.as_mut_ptr(),
            (*s).cmd_bits_.as_mut_ptr(),
            (*s).cmd_code_.as_mut_ptr(),
            core::ptr::addr_of_mut!((*s).cmd_code_numbits_),
        );
    }
    (*s).is_initialized_= 1 as libc::c_int;
    return 1 as libc::c_int;
}
unsafe extern "C" fn BrotliEncoderInitParams(mut params: Option<&mut crate::src::enc::backward_references::BrotliEncoderParams>) {
    (*params.as_deref_mut().unwrap()).mode= BROTLI_MODE_GENERIC;
    (*params.as_deref_mut().unwrap()).large_window= 0 as libc::c_int;
    (*params.as_deref_mut().unwrap()).quality= 11 as libc::c_int;
    (*params.as_deref_mut().unwrap()).lgwin= 22 as libc::c_int;
    (*params.as_deref_mut().unwrap()).lgblock= 0 as libc::c_int;
    (*params.as_deref_mut().unwrap()).stream_offset= 0 as libc::c_int as size_t;
    (*params.as_deref_mut().unwrap()).size_hint= 0 as libc::c_int as size_t;
    (*params.as_deref_mut().unwrap()).disable_literal_context_modeling= 0 as libc::c_int;
    crate::src::enc::encoder_dict::BrotliInitEncoderDictionary(core::ptr::addr_of_mut!((*params.as_deref_mut().unwrap()).dictionary));
    (*params.as_deref_mut().unwrap()).dist.distance_postfix_bits= 0 as libc::c_int as uint32_t;
    (*params.as_deref_mut().unwrap()).dist.num_direct_distance_codes= 0 as libc::c_int as uint32_t;
    (*params.as_deref_mut().unwrap()).dist.alphabet_size_max= ((16 as libc::c_int + 0 as libc::c_int) as libc::c_uint)
        .wrapping_add((24 as libc::c_uint) << 0 as libc::c_int + 1 as libc::c_int);
    (*params.as_deref_mut().unwrap()).dist.alphabet_size_limit= (*params.as_deref().unwrap()).dist.alphabet_size_max;
    (*params.as_deref_mut().unwrap()).dist.max_distance= 0x3fffffc as libc::c_int as size_t;
}
unsafe extern "C" fn BrotliEncoderInitState(mut s: *mut BrotliEncoderState) {
    BrotliEncoderInitParams(Some(&mut (*s).params));
    (*s).input_pos_= 0 as libc::c_int as uint64_t;
    (*s).num_commands_= 0 as libc::c_int as size_t;
    (*s).num_literals_= 0 as libc::c_int as size_t;
    (*s).last_insert_len_= 0 as libc::c_int as size_t;
    (*s).last_flush_pos_= 0 as libc::c_int as uint64_t;
    (*s).last_processed_pos_= 0 as libc::c_int as uint64_t;
    (*s).prev_byte_= 0 as libc::c_int as uint8_t;
    (*s).prev_byte2_= 0 as libc::c_int as uint8_t;
    (*s).storage_size_= 0 as libc::c_int as size_t;
    (*s).storage_= 0 as *mut uint8_t;
    HasherInit(Some(&mut (*s).hasher_));
    (*s).large_table_= 0 as *mut libc::c_int;
    (*s).large_table_size_= 0 as libc::c_int as size_t;
    (*s).cmd_code_numbits_= 0 as libc::c_int as size_t;
    (*s).command_buf_= 0 as *mut uint32_t;
    (*s).literal_buf_= 0 as *mut uint8_t;
    (*s).next_out_= 0 as *mut uint8_t;
    (*s).available_out_= 0 as libc::c_int as size_t;
    (*s).total_out_= 0 as libc::c_int as size_t;
    (*s).stream_state_= BROTLI_STREAM_PROCESSING;
    (*s).is_last_block_emitted_= 0 as libc::c_int;
    (*s).is_initialized_= 0 as libc::c_int;
    RingBufferInit(Some(&mut (*s).ringbuffer_));
    (*s).commands_= 0 as *mut crate::src::enc::backward_references::Command;
    (*s).cmd_alloc_size_= 0 as libc::c_int as size_t;
    (*s).dist_cache_[0 as libc::c_int as usize]= 4 as libc::c_int;
    (*s).dist_cache_[1 as libc::c_int as usize]= 11 as libc::c_int;
    (*s).dist_cache_[2 as libc::c_int as usize]= 15 as libc::c_int;
    (*s).dist_cache_[3 as libc::c_int as usize]= 16 as libc::c_int;
    memcpy(
        (*s).saved_dist_cache_.as_mut_ptr() as *mut libc::c_void,
        (*s).dist_cache_.as_mut_ptr() as *const libc::c_void,
        ::std::mem::size_of::<[libc::c_int; 4]>() as libc::c_ulong,
    );
}
#[no_mangle]
pub unsafe extern "C" fn BrotliEncoderCreateInstance(
    mut alloc_func: brotli_alloc_func,
    mut free_func: brotli_free_func,
    mut opaque: *mut libc::c_void,
) -> *mut BrotliEncoderState {
    let mut state = 0 as *mut BrotliEncoderState;
    if alloc_func.is_none() && free_func.is_none() {
        state= malloc(::std::mem::size_of::<BrotliEncoderState>() as libc::c_ulong)
            as *mut BrotliEncoderState;
    } else if alloc_func.is_some() && free_func.is_some() {
        state= alloc_func
            .expect(
                "non-null function pointer",
            )(opaque, ::std::mem::size_of::<BrotliEncoderState>() as libc::c_ulong)
            as *mut BrotliEncoderState;
    }
    if state.is_null() {();
        return 0 as *mut BrotliEncoderState;
    }
    crate::src::enc::memory::BrotliInitMemoryManager(
        core::ptr::addr_of_mut!((*state).memory_manager_),
        alloc_func,
        free_func,
        opaque,
    );
    BrotliEncoderInitState(state);
    return state;
}
unsafe extern "C" fn BrotliEncoderCleanupState(mut s: *mut BrotliEncoderState) {
    let mut m: *mut crate::src::enc::backward_references_hq::MemoryManager = core::ptr::addr_of_mut!((*s).memory_manager_);
    if 0 as libc::c_int != 0 {
        crate::src::enc::memory::BrotliWipeOutMemoryManager(m);
        return;
    }
    crate::src::enc::memory::BrotliFree(m, (*s).storage_ as *mut libc::c_void);
    (*s).storage_= 0 as *mut uint8_t;
    crate::src::enc::memory::BrotliFree(m, (*s).commands_ as *mut libc::c_void);
    (*s).commands_= 0 as *mut crate::src::enc::backward_references::Command;
    RingBufferFree(m, core::ptr::addr_of_mut!((*s).ringbuffer_));
    DestroyHasher(m, core::ptr::addr_of_mut!((*s).hasher_));
    crate::src::enc::memory::BrotliFree(m, (*s).large_table_ as *mut libc::c_void);
    (*s).large_table_= 0 as *mut libc::c_int;
    crate::src::enc::memory::BrotliFree(m, (*s).command_buf_ as *mut libc::c_void);
    (*s).command_buf_= 0 as *mut uint32_t;
    crate::src::enc::memory::BrotliFree(m, (*s).literal_buf_ as *mut libc::c_void);
    (*s).literal_buf_= 0 as *mut uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn BrotliEncoderDestroyInstance(
    mut state: *mut /* owning */ BrotliEncoderState,
) {
    if state.is_null() {();
        return
    } else {
        let mut m: *mut crate::src::enc::backward_references_hq::MemoryManager = core::ptr::addr_of_mut!((*state).memory_manager_);
        let mut free_func: brotli_free_func = (*m).free_func;
        let mut opaque = (*m).opaque;
        BrotliEncoderCleanupState(state);
        free_func
            .expect("non-null function pointer")(opaque, state as *mut libc::c_void);
    };
}
unsafe extern "C" fn CopyInputToRingBuffer(
    mut s: *mut BrotliEncoderState,
    mut input_size: size_t,
    mut input_buffer: *const uint8_t,
) {
    let mut ringbuffer_: *mut RingBuffer = core::ptr::addr_of_mut!((*s).ringbuffer_);
    let mut m: *mut crate::src::enc::backward_references_hq::MemoryManager = core::ptr::addr_of_mut!((*s).memory_manager_);
    RingBufferWrite(m, input_buffer, input_size, ringbuffer_);
    if 0 as libc::c_int != 0 {
        return;
    }
    (*s).input_pos_= ((*s).input_pos_ as libc::c_ulong).wrapping_add(input_size) as uint64_t
        as uint64_t;
    if (*ringbuffer_).pos_ <= (*ringbuffer_).mask_ {
        memset(
            (*ringbuffer_).buffer_.offset((*ringbuffer_).pos_ as isize)
                as *mut libc::c_void,
            0 as libc::c_int,
            7 as libc::c_int as libc::c_ulong,
        );
    }
}
unsafe extern "C" fn UpdateLastProcessedPos(
    mut s: Option<&mut BrotliEncoderState>,
) -> libc::c_int {
    let mut wrapped_last_processed_pos = WrapPosition((*s.as_deref().unwrap()).last_processed_pos_);
    let mut wrapped_input_pos = WrapPosition((*s.as_deref().unwrap()).input_pos_);
    (*s.as_deref_mut().unwrap()).last_processed_pos_= (*s.as_deref().unwrap()).input_pos_;
    return if wrapped_input_pos < wrapped_last_processed_pos {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
}
unsafe extern "C" fn ExtendLastCommand(
    mut s: *mut BrotliEncoderState,
    mut bytes: Option<&mut uint32_t>,
    mut wrapped_last_processed_pos: Option<&mut uint32_t>,
) {
    let mut last_command: *mut crate::src::enc::backward_references::Command = core::ptr::addr_of_mut!(*(*s).commands_
        .offset(
            (*s).num_commands_.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
        )) as *mut crate::src::enc::backward_references::Command;
    let mut data: *const uint8_t = (*s).ringbuffer_.buffer_;
    let mask = (*s).ringbuffer_.mask_;
    let mut max_backward_distance = ((1 as libc::c_int as uint64_t) << (*s).params.lgwin)
        .wrapping_sub(16 as libc::c_int as libc::c_ulong);
    let mut last_copy_len = ((*last_command).copy_len_
        & 0x1ffffff as libc::c_int as libc::c_uint) as uint64_t;
    let mut last_processed_pos = (*s).last_processed_pos_.wrapping_sub(last_copy_len);
    let mut max_distance = if last_processed_pos < max_backward_distance {
        last_processed_pos
    } else {
        max_backward_distance
    };
    let mut cmd_dist = (*s).dist_cache_[0 as libc::c_int as usize] as uint64_t;
    let mut distance_code = CommandRestoreDistanceCode(
        last_command,
        core::ptr::addr_of!((*s).params.dist),
    );
    if distance_code < 16 as libc::c_int as libc::c_uint
        || distance_code
            .wrapping_sub((16 as libc::c_int - 1 as libc::c_int) as libc::c_uint)
            as libc::c_ulong == cmd_dist
    {
        if cmd_dist <= max_distance {
            while (*bytes.as_deref().unwrap()) != 0 as libc::c_int as libc::c_uint
                && *data.offset(((*wrapped_last_processed_pos.as_deref().unwrap()) & mask) as isize)
                    as libc::c_int
                    == *data
                        .offset(
                            (((*wrapped_last_processed_pos.as_deref().unwrap()) as libc::c_ulong)
                                .wrapping_sub(cmd_dist) & mask as libc::c_ulong) as isize,
                        ) as libc::c_int
            {
                (*last_command).copy_len_= (*last_command).copy_len_.wrapping_add(1);
                *bytes.as_deref_mut().unwrap()= (*bytes.as_deref().unwrap()).wrapping_sub(1);
                *wrapped_last_processed_pos.as_deref_mut().unwrap()= (*wrapped_last_processed_pos.as_deref().unwrap())
                    .wrapping_add(1);
            }
        }
        GetLengthCode(
            (*last_command).insert_len_ as size_t,
            (((*last_command).copy_len_ & 0x1ffffff as libc::c_int as libc::c_uint)
                as libc::c_int
                + ((*last_command).copy_len_ >> 25 as libc::c_int) as libc::c_int)
                as size_t,
            if (*last_command).dist_prefix_ as libc::c_int & 0x3ff as libc::c_int
                == 0 as libc::c_int
            {
                1 as libc::c_int
            } else {
                0 as libc::c_int
            },
            Some(&mut (*last_command).cmd_prefix_),
        );
    }
}
unsafe extern "C" fn EncodeData(
    mut s: *mut BrotliEncoderState,
    mut is_last: libc::c_int,
    mut force_flush: libc::c_int,
    mut out_size: *mut size_t,
    mut output: *mut *mut uint8_t,
) -> libc::c_int {
    let delta = UnprocessedInputSize(s);
    let mut bytes = delta as uint32_t;
    let mut wrapped_last_processed_pos = WrapPosition((*s).last_processed_pos_);
    let mut data = 0 as *mut uint8_t;
    let mut mask: uint32_t = 0;
    let mut m: *mut crate::src::enc::backward_references_hq::MemoryManager = core::ptr::addr_of_mut!((*s).memory_manager_);
    let mut literal_context_mode = CONTEXT_LSB6;
    let mut literal_context_lut = 0 as *const uint8_t;
    data= (*s).ringbuffer_.buffer_;
    mask= (*s).ringbuffer_.mask_;
    if (*s).is_last_block_emitted_ != 0 {
        return 0 as libc::c_int;
    }
    if is_last != 0 {
        (*s).is_last_block_emitted_= 1 as libc::c_int;
    }
    if delta > InputBlockSize(s) {
        return 0 as libc::c_int;
    }
    if (*s).params.quality == 1 as libc::c_int && (*s).command_buf_.is_null() {
        (*s).command_buf_= if crate::src::enc::encode::kCompressFragmentTwoPassBlockSize
            > 0 as libc::c_int as libc::c_ulong
        {
            crate::src::enc::memory::BrotliAllocate(
                m,
                crate::src::enc::encode::kCompressFragmentTwoPassBlockSize
                    .wrapping_mul(::std::mem::size_of::<uint32_t>() as libc::c_ulong),
            ) as *mut uint32_t
        } else {
            0 as *mut uint32_t
        };
        (*s).literal_buf_= if crate::src::enc::encode::kCompressFragmentTwoPassBlockSize
            > 0 as libc::c_int as libc::c_ulong
        {
            crate::src::enc::memory::BrotliAllocate(
                m,
                crate::src::enc::encode::kCompressFragmentTwoPassBlockSize
                    .wrapping_mul(::std::mem::size_of::<uint8_t>() as libc::c_ulong),
            ) as *mut uint8_t
        } else {
            0 as *mut uint8_t
        };
        if 0 as libc::c_int != 0 || 0 as libc::c_int != 0 || 0 as libc::c_int != 0 {
            return 0 as libc::c_int;
        }
    }
    if (*s).params.quality == 0 as libc::c_int || (*s).params.quality == 1 as libc::c_int
    {
        let mut storage = 0 as *mut uint8_t;
        let mut storage_ix = (*s).last_bytes_bits_ as size_t;
        let mut table_size: size_t = 0;
        let mut table = 0 as *mut libc::c_int;
        if delta == 0 as libc::c_int as libc::c_ulong && is_last == 0 {
            *out_size= 0 as libc::c_int as size_t;
            return 1 as libc::c_int;
        }
        storage= GetBrotliStorage(
            s,
            (2 as libc::c_int as libc::c_uint)
                .wrapping_mul(bytes)
                .wrapping_add(503 as libc::c_int as libc::c_uint) as size_t,
        );
        if 0 as libc::c_int != 0 {
            return 0 as libc::c_int;
        }
        *storage.offset(0 as libc::c_int as isize) = (*s).last_bytes_ as uint8_t;
        *storage
            .offset(
                1 as libc::c_int as isize,
            ) = ((*s).last_bytes_ as libc::c_int >> 8 as libc::c_int) as uint8_t;
        table= GetHashTable(s, (*s).params.quality, bytes as size_t, Some(&mut table_size));
        if 0 as libc::c_int != 0 {
            return 0 as libc::c_int;
        }
        if (*s).params.quality == 0 as libc::c_int {
            crate::src::enc::compress_fragment::BrotliCompressFragmentFast(
                m,
                core::ptr::addr_of_mut!(*data.offset((wrapped_last_processed_pos & mask) as isize)),
                bytes as size_t,
                is_last,
                table,
                table_size,
                (*s).cmd_depths_.as_mut_ptr(),
                (*s).cmd_bits_.as_mut_ptr(),
                core::ptr::addr_of_mut!((*s).cmd_code_numbits_),
                (*s).cmd_code_.as_mut_ptr(),
                core::ptr::addr_of_mut!(storage_ix),
                storage,
            );
            if 0 as libc::c_int != 0 {
                return 0 as libc::c_int;
            }
        } else {
            crate::src::enc::compress_fragment_two_pass::BrotliCompressFragmentTwoPass(
                m,
                core::ptr::addr_of_mut!(*data.offset((wrapped_last_processed_pos & mask) as isize)),
                bytes as size_t,
                is_last,
                (*s).command_buf_,
                (*s).literal_buf_,
                table,
                table_size,
                core::ptr::addr_of_mut!(storage_ix),
                storage,
            );
            if 0 as libc::c_int != 0 {
                return 0 as libc::c_int;
            }
        }
        (*s).last_bytes_= *storage.offset((storage_ix >> 3 as libc::c_int) as isize)
            as uint16_t;
        (*s).last_bytes_bits_= (storage_ix & 7 as libc::c_uint as libc::c_ulong)
            as uint8_t;
        UpdateLastProcessedPos(s.as_mut());
        *output= core::ptr::addr_of_mut!(*storage.offset(0 as libc::c_int as isize)) as *mut uint8_t;
        *out_size= storage_ix >> 3 as libc::c_int;
        return 1 as libc::c_int;
    }
    let mut newsize = (*s).num_commands_
        .wrapping_add(
            bytes.wrapping_div(2 as libc::c_int as libc::c_uint) as libc::c_ulong,
        )
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    if newsize > (*s).cmd_alloc_size_ {
        let mut new_commands = 0 as *mut crate::src::enc::backward_references::Command;
        newsize= (newsize as libc::c_ulong)
            .wrapping_add(
                bytes
                    .wrapping_div(4 as libc::c_int as libc::c_uint)
                    .wrapping_add(16 as libc::c_int as libc::c_uint) as libc::c_ulong,
            ) as size_t as size_t;
        (*s).cmd_alloc_size_= newsize;
        new_commands= if newsize > 0 as libc::c_int as libc::c_ulong {
            crate::src::enc::memory::BrotliAllocate(
                m,
                newsize.wrapping_mul(::std::mem::size_of::<crate::src::enc::backward_references::Command>() as libc::c_ulong),
            ) as *mut crate::src::enc::backward_references::Command
        } else {
            0 as *mut crate::src::enc::backward_references::Command
        };
        if 0 as libc::c_int != 0 || 0 as libc::c_int != 0 {
            return 0 as libc::c_int;
        }
        if !(*s).commands_.is_null() {
            memcpy(
                new_commands as *mut libc::c_void,
                (*s).commands_ as *const libc::c_void,
                (::std::mem::size_of::<crate::src::enc::backward_references::Command>() as libc::c_ulong)
                    .wrapping_mul((*s).num_commands_),
            );
            crate::src::enc::memory::BrotliFree(m, (*s).commands_ as *mut libc::c_void);
            (*s).commands_= 0 as *mut crate::src::enc::backward_references::Command;
        }else { (); }
        (*s).commands_= new_commands;
    }
    InitOrStitchToPreviousBlock(
        m,
        core::ptr::addr_of_mut!((*s).hasher_),
        data,
        mask as size_t,
        core::ptr::addr_of_mut!((*s).params),
        wrapped_last_processed_pos as size_t,
        bytes as size_t,
        is_last,
    );
    literal_context_mode= ChooseContextMode(
        core::ptr::addr_of!((*s).params),
        data,
        WrapPosition((*s).last_flush_pos_) as size_t,
        mask as size_t,
        (*s).input_pos_.wrapping_sub((*s).last_flush_pos_),
    );
    literal_context_lut= &*_kBrotliContextLookupTable
        .as_ptr()
        .offset(((literal_context_mode as libc::c_uint) << 9 as libc::c_int) as isize)
        as *const uint8_t;
    if 0 as libc::c_int != 0 {
        return 0 as libc::c_int;
    }
    if (*s).num_commands_ != 0
        && (*s).last_insert_len_ == 0 as libc::c_int as libc::c_ulong
    {
        ExtendLastCommand(s, Some(&mut bytes), Some(&mut wrapped_last_processed_pos));
    }
    if (*s).params.quality == 10 as libc::c_int {
        crate::src::enc::backward_references_hq::BrotliCreateZopfliBackwardReferences(
            m,
            bytes as size_t,
            wrapped_last_processed_pos as size_t,
            data,
            mask as size_t,
            literal_context_lut,
            core::ptr::addr_of!((*s).params),
            core::ptr::addr_of_mut!((*s).hasher_),
            (*s).dist_cache_.as_mut_ptr(),
            core::ptr::addr_of_mut!((*s).last_insert_len_),
            core::ptr::addr_of_mut!(*(*s).commands_.offset((*s).num_commands_ as isize)),
            core::ptr::addr_of_mut!((*s).num_commands_),
            core::ptr::addr_of_mut!((*s).num_literals_),
        );
        if 0 as libc::c_int != 0 {
            return 0 as libc::c_int;
        }
    } else if (*s).params.quality == 11 as libc::c_int {
        crate::src::enc::backward_references_hq::BrotliCreateHqZopfliBackwardReferences(
            m,
            bytes as size_t,
            wrapped_last_processed_pos as size_t,
            data,
            mask as size_t,
            literal_context_lut,
            core::ptr::addr_of!((*s).params),
            core::ptr::addr_of_mut!((*s).hasher_),
            (*s).dist_cache_.as_mut_ptr(),
            core::ptr::addr_of_mut!((*s).last_insert_len_),
            core::ptr::addr_of_mut!(*(*s).commands_.offset((*s).num_commands_ as isize)),
            core::ptr::addr_of_mut!((*s).num_commands_),
            core::ptr::addr_of_mut!((*s).num_literals_),
        );
        if 0 as libc::c_int != 0 {
            return 0 as libc::c_int;
        }
    } else {
        crate::src::enc::backward_references::BrotliCreateBackwardReferences(
            bytes as size_t,
            wrapped_last_processed_pos as size_t,
            data,
            mask as size_t,
            literal_context_lut,
            core::ptr::addr_of!((*s).params),
            core::ptr::addr_of_mut!((*s).hasher_),
            (*s).dist_cache_.as_mut_ptr(),
            core::ptr::addr_of_mut!((*s).last_insert_len_),
            core::ptr::addr_of_mut!(*(*s).commands_.offset((*s).num_commands_ as isize)),
            core::ptr::addr_of_mut!((*s).num_commands_),
            core::ptr::addr_of_mut!((*s).num_literals_),
        );
    }
    let max_length = MaxMetablockSize(core::ptr::addr_of!((*s).params));
    let max_literals = max_length.wrapping_div(8 as libc::c_int as libc::c_ulong);
    let max_commands = max_length.wrapping_div(8 as libc::c_int as libc::c_ulong);
    let processed_bytes = (*s).input_pos_.wrapping_sub((*s).last_flush_pos_);
    let next_input_fits_metablock = if processed_bytes.wrapping_add(InputBlockSize(s))
        <= max_length
    {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
    let should_flush = if (*s).params.quality < 4 as libc::c_int
        && (*s).num_literals_.wrapping_add((*s).num_commands_)
            >= 0x2fff as libc::c_int as libc::c_ulong
    {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
    if is_last == 0 && force_flush == 0 && should_flush == 0
        && next_input_fits_metablock != 0 && (*s).num_literals_ < max_literals
        && (*s).num_commands_ < max_commands
    {
        if UpdateLastProcessedPos(s.as_mut()) != 0 {
            HasherReset(Some(&mut (*s).hasher_));
        }
        *out_size= 0 as libc::c_int as size_t;
        return 1 as libc::c_int;
    }
    if (*s).last_insert_len_ > 0 as libc::c_int as libc::c_ulong {
        let fresh92 = (*s).num_commands_;(*s).num_commands_= (*s).num_commands_.wrapping_add(1);
        InitInsertCommand(
            Some(&mut *(*s).commands_.offset(fresh92 as isize)),
            (*s).last_insert_len_,
        );
        (*s).num_literals_= ((*s).num_literals_ as libc::c_ulong).wrapping_add((*s).last_insert_len_)
            as size_t as size_t;
        (*s).last_insert_len_= 0 as libc::c_int as size_t;
    }
    if is_last == 0 && (*s).input_pos_ == (*s).last_flush_pos_ {
        *out_size= 0 as libc::c_int as size_t;
        return 1 as libc::c_int;
    }
    let metablock_size = (*s).input_pos_.wrapping_sub((*s).last_flush_pos_)
        as uint32_t;
    let mut storage_0 = GetBrotliStorage(
        s,
        (2 as libc::c_int as libc::c_uint)
            .wrapping_mul(metablock_size)
            .wrapping_add(503 as libc::c_int as libc::c_uint) as size_t,
    );
    let mut storage_ix_0 = (*s).last_bytes_bits_ as size_t;
    if 0 as libc::c_int != 0 {
        return 0 as libc::c_int;
    }
    *storage_0.offset(0 as libc::c_int as isize) = (*s).last_bytes_ as uint8_t;
    *storage_0
        .offset(
            1 as libc::c_int as isize,
        ) = ((*s).last_bytes_ as libc::c_int >> 8 as libc::c_int) as uint8_t;
    WriteMetaBlockInternal(
        m,
        data,
        mask as size_t,
        (*s).last_flush_pos_,
        metablock_size as size_t,
        is_last,
        literal_context_mode,
        core::ptr::addr_of!((*s).params),
        (*s).prev_byte_,
        (*s).prev_byte2_,
        (*s).num_literals_,
        (*s).num_commands_,
        (*s).commands_,
        (*s).saved_dist_cache_.as_mut_ptr(),
        (*s).dist_cache_.as_mut_ptr(),
        core::ptr::addr_of_mut!(storage_ix_0),
        storage_0,
    );
    if 0 as libc::c_int != 0 {
        return 0 as libc::c_int;
    }
    (*s).last_bytes_= *storage_0.offset((storage_ix_0 >> 3 as libc::c_int) as isize)
        as uint16_t;
    (*s).last_bytes_bits_= (storage_ix_0 & 7 as libc::c_uint as libc::c_ulong)
        as uint8_t;
    (*s).last_flush_pos_= (*s).input_pos_;
    if UpdateLastProcessedPos(s.as_mut()) != 0 {
        HasherReset(Some(&mut (*s).hasher_));
    }
    if (*s).last_flush_pos_ > 0 as libc::c_int as libc::c_ulong {
        (*s).prev_byte_= *data
            .offset(
                (((*s).last_flush_pos_ as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) & mask) as isize,
            );
    }
    if (*s).last_flush_pos_ > 1 as libc::c_int as libc::c_ulong {
        (*s).prev_byte2_= *data
            .offset(
                ((*s).last_flush_pos_.wrapping_sub(2 as libc::c_int as libc::c_ulong)
                    as uint32_t & mask) as isize,
            );
    }
    (*s).num_commands_= 0 as libc::c_int as size_t;
    (*s).num_literals_= 0 as libc::c_int as size_t;
    memcpy(
        (*s).saved_dist_cache_.as_mut_ptr() as *mut libc::c_void,
        (*s).dist_cache_.as_mut_ptr() as *const libc::c_void,
        ::std::mem::size_of::<[libc::c_int; 4]>() as libc::c_ulong,
    );
    *output= core::ptr::addr_of_mut!(*storage_0.offset(0 as libc::c_int as isize)) as *mut uint8_t;
    *out_size= storage_ix_0 >> 3 as libc::c_int;
    return 1 as libc::c_int;
}
unsafe extern "C" fn WriteMetadataHeader(
    mut s: *mut BrotliEncoderState,
    mut block_size: size_t,
    mut header: *mut uint8_t,
) -> size_t {
    let mut storage_ix: size_t = 0;
    storage_ix= (*s).last_bytes_bits_ as size_t;
    *header.offset(0 as libc::c_int as isize) = (*s).last_bytes_ as uint8_t;
    *header
        .offset(
            1 as libc::c_int as isize,
        ) = ((*s).last_bytes_ as libc::c_int >> 8 as libc::c_int) as uint8_t;
    (*s).last_bytes_= 0 as libc::c_int as uint16_t;
    (*s).last_bytes_bits_= 0 as libc::c_int as uint8_t;
    BrotliWriteBits(
        1 as libc::c_int as size_t,
        0 as libc::c_int as uint64_t,
        core::ptr::addr_of_mut!(storage_ix),
        header,
    );
    BrotliWriteBits(
        2 as libc::c_int as size_t,
        3 as libc::c_int as uint64_t,
        core::ptr::addr_of_mut!(storage_ix),
        header,
    );
    BrotliWriteBits(
        1 as libc::c_int as size_t,
        0 as libc::c_int as uint64_t,
        core::ptr::addr_of_mut!(storage_ix),
        header,
    );
    if block_size == 0 as libc::c_int as libc::c_ulong {
        BrotliWriteBits(
            2 as libc::c_int as size_t,
            0 as libc::c_int as uint64_t,
            core::ptr::addr_of_mut!(storage_ix),
            header,
        );
    } else {
        let mut nbits = if block_size == 1 as libc::c_int as libc::c_ulong {
            0 as libc::c_int as libc::c_uint
        } else {
            (Log2FloorNonZero(
                (block_size as uint32_t).wrapping_sub(1 as libc::c_int as libc::c_uint)
                    as size_t,
            ))
                .wrapping_add(1 as libc::c_int as libc::c_uint)
        };
        let mut nbytes = nbits
            .wrapping_add(7 as libc::c_int as libc::c_uint)
            .wrapping_div(8 as libc::c_int as libc::c_uint);
        BrotliWriteBits(
            2 as libc::c_int as size_t,
            nbytes as uint64_t,
            core::ptr::addr_of_mut!(storage_ix),
            header,
        );
        BrotliWriteBits(
            (8 as libc::c_int as libc::c_uint).wrapping_mul(nbytes) as size_t,
            block_size.wrapping_sub(1 as libc::c_int as libc::c_ulong),
            core::ptr::addr_of_mut!(storage_ix),
            header,
        );
    }
    return storage_ix.wrapping_add(7 as libc::c_uint as libc::c_ulong)
        >> 3 as libc::c_int;
}
unsafe extern "C" fn BrotliCompressBufferQuality10(
    mut lgwin: libc::c_int,
    mut input_size: size_t,
    mut input_buffer: *const uint8_t,
    mut encoded_size: Option<&mut size_t>,
    mut encoded_buffer: *mut uint8_t,
) -> libc::c_int {
    let mut current_block: u64;
    let mut memory_manager = crate::src::enc::backward_references_hq::MemoryManager {
        alloc_func: None,
        free_func: None,
        opaque: 0 as *mut libc::c_void,
    };
    let mut m: *mut crate::src::enc::backward_references_hq::MemoryManager = core::ptr::addr_of_mut!(memory_manager);
    let mask = !(0 as libc::c_int as size_t) >> 1 as libc::c_int;
    let mut dist_cache: [libc::c_int; 4] = [
        4 as libc::c_int,
        11 as libc::c_int,
        15 as libc::c_int,
        16 as libc::c_int,
    ];
    let mut saved_dist_cache: [libc::c_int; 4] = [
        4 as libc::c_int,
        11 as libc::c_int,
        15 as libc::c_int,
        16 as libc::c_int,
    ];
    let mut ok = 1 as libc::c_int;
    let max_out_size = (*encoded_size.as_deref().unwrap());
    let mut total_out_size = 0 as libc::c_int as size_t;
    let mut last_bytes: uint16_t = 0;
    let mut last_bytes_bits: uint8_t = 0;
    let hasher_eff_size = brotli_min_size_t(
        input_size,
        ((1 as libc::c_int as size_t) << lgwin)
            .wrapping_sub(16 as libc::c_int as libc::c_ulong)
            .wrapping_add(16 as libc::c_int as libc::c_ulong),
    );
    let mut params = crate::src::enc::backward_references::BrotliEncoderParams {
        mode: BROTLI_MODE_GENERIC,
        quality: 0,
        lgwin: 0,
        lgblock: 0,
        stream_offset: 0,
        size_hint: 0,
        disable_literal_context_modeling: 0,
        large_window: 0,
        hasher: crate::src::enc::backward_references::BrotliHasherParams {
            type_0: 0,
            bucket_bits: 0,
            block_bits: 0,
            hash_len: 0,
            num_last_distances_to_check: 0,
        },
        dist: crate::src::enc::backward_references::BrotliDistanceParams {
            distance_postfix_bits: 0,
            num_direct_distance_codes: 0,
            alphabet_size_max: 0,
            alphabet_size_limit: 0,
            max_distance: 0,
        },
        dictionary: crate::src::enc::backward_references::BrotliEncoderDictionary {
            words: 0 as *const crate::src::common::dictionary::BrotliDictionary,
            num_transforms: 0,
            cutoffTransformsCount: 0,
            cutoffTransforms: 0,
            hash_table_words: 0 as *const uint16_t,
            hash_table_lengths: 0 as *const uint8_t,
            buckets: 0 as *const uint16_t,
            dict_words: 0 as *const crate::src::enc::backward_references::DictWord,
        },
    };
    let lgmetablock = brotli_min_int(24 as libc::c_int, lgwin + 1 as libc::c_int);
    let mut max_block_size: size_t = 0;
    let max_metablock_size = (1 as libc::c_int as size_t) << lgmetablock;
    let max_literals_per_metablock = max_metablock_size
        .wrapping_div(8 as libc::c_int as libc::c_ulong);
    let max_commands_per_metablock = max_metablock_size
        .wrapping_div(8 as libc::c_int as libc::c_ulong);
    let mut metablock_start = 0 as libc::c_int as size_t;
    let mut prev_byte = 0 as libc::c_int as uint8_t;
    let mut prev_byte2 = 0 as libc::c_int as uint8_t;
    let mut hasher = crate::src::enc::backward_references::Hasher {
        common: crate::src::enc::backward_references::HasherCommon {
            extra: 0 as *mut libc::c_void,
            dict_num_lookups: 0,
            dict_num_matches: 0,
            params: crate::src::enc::backward_references::BrotliHasherParams {
                type_0: 0,
                bucket_bits: 0,
                block_bits: 0,
                hash_len: 0,
                num_last_distances_to_check: 0,
            },
            is_prepared_: 0,
        },
        privat: crate::src::enc::backward_references::C2RustUnnamed {
            _H2: crate::src::enc::backward_references::H2 {
                common: 0 as *mut crate::src::enc::backward_references::HasherCommon,
                buckets_: 0 as *mut uint32_t,
            },
        },
    };
    HasherInit(Some(&mut hasher));
    BrotliEncoderInitParams(Some(&mut params));
    params.quality= 10 as libc::c_int;
    params.lgwin= lgwin;
    if lgwin > 24 as libc::c_int {
        params.large_window= 1 as libc::c_int;
    }
    SanitizeParams(Some(&mut params));
    params.lgblock= ComputeLgBlock(core::ptr::addr_of!(params));
    ChooseDistanceParams(Some(&mut params));
    max_block_size= (1 as libc::c_int as size_t) << params.lgblock;
    crate::src::enc::memory::BrotliInitMemoryManager(m, None, None, 0 as *mut libc::c_void);
    EncodeWindowBits(lgwin, params.large_window, core::ptr::addr_of_mut!(last_bytes), core::ptr::addr_of_mut!(last_bytes_bits));
    InitOrStitchToPreviousBlock(
        m,
        core::ptr::addr_of_mut!(hasher),
        input_buffer,
        mask,
        core::ptr::addr_of_mut!(params),
        0 as libc::c_int as size_t,
        hasher_eff_size,
        1 as libc::c_int,
    );
    if 0 as libc::c_int != 0 {
        current_block = 16891123271837848903;
    } else {
        current_block= 4068382217303356765;
    }
    'c_20643: loop {
        match current_block {
            16891123271837848903 => {
                crate::src::enc::memory::BrotliWipeOutMemoryManager(m);
                return 0 as libc::c_int;
            }
            _ => {
                if ok != 0 && metablock_start < input_size {
                    let metablock_end = brotli_min_size_t(
                        input_size,
                        metablock_start.wrapping_add(max_metablock_size),
                    );
                    let expected_num_commands = metablock_end
                        .wrapping_sub(metablock_start)
                        .wrapping_div(12 as libc::c_int as libc::c_ulong)
                        .wrapping_add(16 as libc::c_int as libc::c_ulong);
                    let mut commands = 0 as *mut crate::src::enc::backward_references::Command;
                    let mut num_commands = 0 as libc::c_int as size_t;
                    let mut last_insert_len = 0 as libc::c_int as size_t;
                    let mut num_literals = 0 as libc::c_int as size_t;
                    let mut metablock_size = 0 as libc::c_int as size_t;
                    let mut cmd_alloc_size = 0 as libc::c_int as size_t;
                    let mut is_last: libc::c_int = 0;
                    let mut storage = 0 as *mut uint8_t;
                    let mut storage_ix: size_t = 0;
                    let mut literal_context_mode = ChooseContextMode(
                        core::ptr::addr_of!(params),
                        input_buffer,
                        metablock_start,
                        mask,
                        metablock_end.wrapping_sub(metablock_start),
                    );
                    let mut literal_context_lut: ContextLut = &*_kBrotliContextLookupTable
                        .as_ptr()
                        .offset(
                            ((literal_context_mode as libc::c_uint) << 9 as libc::c_int)
                                as isize,
                        ) as *const uint8_t;
                    let mut block_start: size_t = 0;
                    block_start= metablock_start;
                    while block_start < metablock_end {
                        let mut block_size = brotli_min_size_t(
                            metablock_end.wrapping_sub(block_start),
                            max_block_size,
                        );
                        let mut nodes = if block_size
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            > 0 as libc::c_int as libc::c_ulong
                        {
                            crate::src::enc::memory::BrotliAllocate(
                                m,
                                block_size
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(
                                        ::std::mem::size_of::<crate::src::enc::backward_references_hq::ZopfliNode>() as libc::c_ulong,
                                    ),
                            ) as *mut crate::src::enc::backward_references_hq::ZopfliNode
                        } else {
                            0 as *mut crate::src::enc::backward_references_hq::ZopfliNode
                        };
                        let mut path_size: size_t = 0;
                        let mut new_cmd_alloc_size: size_t = 0;
                        if 0 as libc::c_int != 0 || 0 as libc::c_int != 0 {
                            current_block= 16891123271837848903;
                            continue 'c_20643;
                        }
                        crate::src::enc::backward_references_hq::BrotliInitZopfliNodes(
                            nodes,
                            block_size.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        );
                        StitchToPreviousBlockH10(
                            core::ptr::addr_of_mut!(hasher.privat._H10),
                            block_size,
                            block_start,
                            input_buffer,
                            mask,
                        );
                        path_size= crate::src::enc::backward_references_hq::BrotliZopfliComputeShortestPath(
                            m,
                            block_size,
                            block_start,
                            input_buffer,
                            mask,
                            literal_context_lut,
                            core::ptr::addr_of!(params),
                            dist_cache.as_mut_ptr(),
                            core::ptr::addr_of_mut!(hasher),
                            nodes,
                        );
                        if 0 as libc::c_int != 0 {
                            current_block = 16891123271837848903;
                            continue 'c_20643;
                        }
                        new_cmd_alloc_size= brotli_max_size_t(
                            expected_num_commands,
                            num_commands
                                .wrapping_add(path_size)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        );
                        if cmd_alloc_size != new_cmd_alloc_size {
                            let mut new_commands = if new_cmd_alloc_size
                                > 0 as libc::c_int as libc::c_ulong
                            {
                                crate::src::enc::memory::BrotliAllocate(
                                    m,
                                    new_cmd_alloc_size
                                        .wrapping_mul(
                                            ::std::mem::size_of::<crate::src::enc::backward_references::Command>() as libc::c_ulong,
                                        ),
                                ) as *mut crate::src::enc::backward_references::Command
                            } else {
                                0 as *mut crate::src::enc::backward_references::Command
                            };
                            if 0 as libc::c_int != 0 || 0 as libc::c_int != 0 {
                                current_block= 16891123271837848903;
                                continue 'c_20643;
                            }
                            cmd_alloc_size= new_cmd_alloc_size;
                            if !commands.is_null() {
                                memcpy(
                                    new_commands as *mut libc::c_void,
                                    commands as *const libc::c_void,
                                    (::std::mem::size_of::<crate::src::enc::backward_references::Command>() as libc::c_ulong)
                                        .wrapping_mul(num_commands),
                                );
                                crate::src::enc::memory::BrotliFree(m, commands as *mut libc::c_void);
                                commands= 0 as *mut crate::src::enc::backward_references::Command;
                            }else { (); }
                            commands= new_commands;
                        }
                        crate::src::enc::backward_references_hq::BrotliZopfliCreateCommands(
                            block_size,
                            block_start,
                            core::ptr::addr_of_mut!(*nodes.offset(0 as libc::c_int as isize)),
                            dist_cache.as_mut_ptr(),
                            core::ptr::addr_of_mut!(last_insert_len),
                            core::ptr::addr_of!(params),
                            core::ptr::addr_of_mut!(*commands.offset(num_commands as isize)),
                            core::ptr::addr_of_mut!(num_literals),
                        );
                        num_commands= (num_commands as libc::c_ulong)
                            .wrapping_add(path_size) as size_t as size_t;
                        block_start= (block_start as libc::c_ulong)
                            .wrapping_add(block_size) as size_t as size_t;
                        metablock_size= (metablock_size as libc::c_ulong)
                            .wrapping_add(block_size) as size_t as size_t;
                        crate::src::enc::memory::BrotliFree(m, nodes as *mut libc::c_void);
                        nodes= 0 as *mut crate::src::enc::backward_references_hq::ZopfliNode;
                        if num_literals > max_literals_per_metablock
                            || num_commands > max_commands_per_metablock
                        {
                            break;
                        }
                    }
                    if last_insert_len > 0 as libc::c_int as libc::c_ulong {
                        let fresh94 = num_commands;
                        num_commands= num_commands.wrapping_add(1);
                        InitInsertCommand(
                            Some(&mut *commands.offset(fresh94 as isize)),
                            last_insert_len,
                        );
                        num_literals= (num_literals as libc::c_ulong)
                            .wrapping_add(last_insert_len) as size_t as size_t;
                    }
                    is_last= if metablock_start.wrapping_add(metablock_size)
                        == input_size
                    {
                        1 as libc::c_int
                    } else {
                        0 as libc::c_int
                    };
                    storage= 0 as *mut uint8_t;
                    storage_ix= last_bytes_bits as size_t;
                    if metablock_size == 0 as libc::c_int as libc::c_ulong {
                        storage= if 16 as libc::c_int > 0 as libc::c_int {
                            crate::src::enc::memory::BrotliAllocate(
                                m,
                                (16 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(
                                        ::std::mem::size_of::<uint8_t>() as libc::c_ulong,
                                    ),
                            ) as *mut uint8_t
                        } else {
                            0 as *mut uint8_t
                        };
                        if 0 as libc::c_int != 0 || 0 as libc::c_int != 0 {
                            current_block= 16891123271837848903;
                            continue;
                        }
                        *storage
                            .offset(0 as libc::c_int as isize) = last_bytes as uint8_t;
                        *storage
                            .offset(
                                1 as libc::c_int as isize,
                            ) = (last_bytes as libc::c_int >> 8 as libc::c_int)
                            as uint8_t;
                        BrotliWriteBits(
                            2 as libc::c_int as size_t,
                            3 as libc::c_int as uint64_t,
                            core::ptr::addr_of_mut!(storage_ix),
                            storage,
                        );
                        storage_ix= storage_ix
                            .wrapping_add(7 as libc::c_uint as libc::c_ulong)
                            & !(7 as libc::c_uint) as libc::c_ulong;
                    } else if ShouldCompress(
                        input_buffer,
                        mask,
                        metablock_start,
                        metablock_size,
                        num_literals,
                        num_commands,
                    ) == 0
                    {
                        memcpy(
                            dist_cache.as_mut_ptr() as *mut libc::c_void,
                            saved_dist_cache.as_mut_ptr() as *const libc::c_void,
                            (4 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(
                                    ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                                ),
                        );
                        storage= if metablock_size
                            .wrapping_add(16 as libc::c_int as libc::c_ulong)
                            > 0 as libc::c_int as libc::c_ulong
                        {
                            crate::src::enc::memory::BrotliAllocate(
                                m,
                                metablock_size
                                    .wrapping_add(16 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(
                                        ::std::mem::size_of::<uint8_t>() as libc::c_ulong,
                                    ),
                            ) as *mut uint8_t
                        } else {
                            0 as *mut uint8_t
                        };
                        if 0 as libc::c_int != 0 || 0 as libc::c_int != 0 {
                            current_block= 16891123271837848903;
                            continue;
                        }
                        *storage
                            .offset(0 as libc::c_int as isize) = last_bytes as uint8_t;
                        *storage
                            .offset(
                                1 as libc::c_int as isize,
                            ) = (last_bytes as libc::c_int >> 8 as libc::c_int)
                            as uint8_t;
                        crate::src::enc::brotli_bit_stream::BrotliStoreUncompressedMetaBlock(
                            is_last,
                            input_buffer,
                            metablock_start,
                            mask,
                            metablock_size,
                            core::ptr::addr_of_mut!(storage_ix),
                            storage,
                        );
                    } else {
                        let mut mb = crate::src::enc::brotli_bit_stream::MetaBlockSplit {
                            literal_split: crate::src::enc::block_splitter::BlockSplit {
                                num_types: 0,
                                num_blocks: 0,
                                types: 0 as *mut uint8_t,
                                lengths: 0 as *mut uint32_t,
                                types_alloc_size: 0,
                                lengths_alloc_size: 0,
                            },
                            command_split: crate::src::enc::block_splitter::BlockSplit {
                                num_types: 0,
                                num_blocks: 0,
                                types: 0 as *mut uint8_t,
                                lengths: 0 as *mut uint32_t,
                                types_alloc_size: 0,
                                lengths_alloc_size: 0,
                            },
                            distance_split: crate::src::enc::block_splitter::BlockSplit {
                                num_types: 0,
                                num_blocks: 0,
                                types: 0 as *mut uint8_t,
                                lengths: 0 as *mut uint32_t,
                                types_alloc_size: 0,
                                lengths_alloc_size: 0,
                            },
                            literal_context_map: 0 as *mut uint32_t,
                            literal_context_map_size: 0,
                            distance_context_map: 0 as *mut uint32_t,
                            distance_context_map_size: 0,
                            literal_histograms: 0 as *mut crate::src::enc::bit_cost::HistogramLiteral,
                            literal_histograms_size: 0,
                            command_histograms: 0 as *mut crate::src::enc::bit_cost::HistogramCommand,
                            command_histograms_size: 0,
                            distance_histograms: 0 as *mut crate::src::enc::bit_cost::HistogramDistance,
                            distance_histograms_size: 0,
                        };
                        let mut block_params = params;
                        InitMetaBlockSplit(Some(&mut mb));
                        crate::src::enc::metablock::BrotliBuildMetaBlock(
                            m,
                            input_buffer,
                            metablock_start,
                            mask,
                            Some(&mut block_params),
                            prev_byte,
                            prev_byte2,
                            commands,
                            num_commands,
                            literal_context_mode,
                            core::ptr::addr_of_mut!(mb),
                        );
                        if 0 as libc::c_int != 0 {
                            current_block = 16891123271837848903;
                            continue;
                        }
                        crate::src::enc::metablock::BrotliOptimizeHistograms(
                            block_params.dist.alphabet_size_limit,
                            Some(&mut mb),
                        );
                        storage= if (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(metablock_size)
                            .wrapping_add(503 as libc::c_int as libc::c_ulong)
                            > 0 as libc::c_int as libc::c_ulong
                        {
                            crate::src::enc::memory::BrotliAllocate(
                                m,
                                (2 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(metablock_size)
                                    .wrapping_add(503 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(
                                        ::std::mem::size_of::<uint8_t>() as libc::c_ulong,
                                    ),
                            ) as *mut uint8_t
                        } else {
                            0 as *mut uint8_t
                        };
                        if 0 as libc::c_int != 0 || 0 as libc::c_int != 0 {
                            current_block= 16891123271837848903;
                            continue;
                        }
                        *storage
                            .offset(0 as libc::c_int as isize) = last_bytes as uint8_t;
                        *storage
                            .offset(
                                1 as libc::c_int as isize,
                            ) = (last_bytes as libc::c_int >> 8 as libc::c_int)
                            as uint8_t;
                        crate::src::enc::brotli_bit_stream::BrotliStoreMetaBlock(
                            m,
                            input_buffer,
                            metablock_start,
                            metablock_size,
                            mask,
                            prev_byte,
                            prev_byte2,
                            is_last,
                            core::ptr::addr_of!(block_params),
                            literal_context_mode,
                            commands,
                            num_commands,
                            core::ptr::addr_of!(mb),
                            core::ptr::addr_of_mut!(storage_ix),
                            storage,
                        );
                        if 0 as libc::c_int != 0 {
                            current_block = 16891123271837848903;
                            continue;
                        }
                        if metablock_size.wrapping_add(4 as libc::c_int as libc::c_ulong)
                            < storage_ix >> 3 as libc::c_int
                        {
                            memcpy(
                                dist_cache.as_mut_ptr() as *mut libc::c_void,
                                saved_dist_cache.as_mut_ptr() as *const libc::c_void,
                                (4 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(
                                        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                                    ),
                            );
                            *storage
                                .offset(0 as libc::c_int as isize) = last_bytes as uint8_t;
                            *storage
                                .offset(
                                    1 as libc::c_int as isize,
                                ) = (last_bytes as libc::c_int >> 8 as libc::c_int)
                                as uint8_t;
                            storage_ix= last_bytes_bits as size_t;
                            crate::src::enc::brotli_bit_stream::BrotliStoreUncompressedMetaBlock(
                                is_last,
                                input_buffer,
                                metablock_start,
                                mask,
                                metablock_size,
                                core::ptr::addr_of_mut!(storage_ix),
                                storage,
                            );
                        }
                        DestroyMetaBlockSplit(m, core::ptr::addr_of_mut!(mb));
                    }
                    last_bytes= *storage
                        .offset((storage_ix >> 3 as libc::c_int) as isize) as uint16_t;
                    last_bytes_bits= (storage_ix & 7 as libc::c_uint as libc::c_ulong)
                        as uint8_t;
                    metablock_start= (metablock_start as libc::c_ulong)
                        .wrapping_add(metablock_size) as size_t as size_t;
                    if metablock_start < input_size {
                        prev_byte= *input_buffer
                            .offset(
                                metablock_start
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                            );
                        prev_byte2= *input_buffer
                            .offset(
                                metablock_start
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize,
                            );
                    }
                    memcpy(
                        saved_dist_cache.as_mut_ptr() as *mut libc::c_void,
                        dist_cache.as_mut_ptr() as *const libc::c_void,
                        (4 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    );
                    let out_size = storage_ix >> 3 as libc::c_int;
                    total_out_size= (total_out_size as libc::c_ulong)
                        .wrapping_add(out_size) as size_t as size_t;
                    if total_out_size <= max_out_size {
                        memcpy(
                            encoded_buffer as *mut libc::c_void,
                            storage as *const libc::c_void,
                            out_size,
                        );
                        encoded_buffer= encoded_buffer.offset(out_size as isize);
                    } else {
                        ok= 0 as libc::c_int;
                    }
                    crate::src::enc::memory::BrotliFree(m, storage as *mut libc::c_void);
                    storage= 0 as *mut uint8_t;
                    crate::src::enc::memory::BrotliFree(m, commands as *mut libc::c_void);
                    commands= 0 as *mut crate::src::enc::backward_references::Command;
                    current_block= 4068382217303356765;
                } else {
                    *encoded_size.as_deref_mut().unwrap()= total_out_size;
                    DestroyHasher(m, core::ptr::addr_of_mut!(hasher));
                    return ok;
                }
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn BrotliEncoderMaxCompressedSize(
    mut input_size: size_t,
) -> size_t {
    let mut num_large_blocks = input_size >> 14 as libc::c_int;
    let mut overhead = (2 as libc::c_int as libc::c_ulong)
        .wrapping_add((4 as libc::c_int as libc::c_ulong).wrapping_mul(num_large_blocks))
        .wrapping_add(3 as libc::c_int as libc::c_ulong)
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    let mut result = input_size.wrapping_add(overhead);
    if input_size == 0 as libc::c_int as libc::c_ulong {
        return 2 as libc::c_int as size_t;
    }
    return if result < input_size { 0 as libc::c_int as libc::c_ulong } else { result };
}
unsafe extern "C" fn MakeUncompressedStream(
    mut input: *const uint8_t,
    mut input_size: size_t,
    mut output: *mut uint8_t,
) -> size_t {
    let mut size = input_size;
    let mut result = 0 as libc::c_int as size_t;
    let mut offset = 0 as libc::c_int as size_t;
    if input_size == 0 as libc::c_int as libc::c_ulong {
        *output.offset(0 as libc::c_int as isize) = 6 as libc::c_int as uint8_t;
        return 1 as libc::c_int as size_t;
    }
    let fresh95 = result;
    result= result.wrapping_add(1);
    *output.offset(fresh95 as isize) = 0x21 as libc::c_int as uint8_t;
    let fresh96 = result;
    result= result.wrapping_add(1);
    *output.offset(fresh96 as isize) = 0x3 as libc::c_int as uint8_t;
    while size > 0 as libc::c_int as libc::c_ulong {
        let mut nibbles = 0 as libc::c_int as uint32_t;
        let mut chunk_size: uint32_t = 0;
        let mut bits: uint32_t = 0;
        chunk_size= if size
            > ((1 as libc::c_uint) << 24 as libc::c_int) as libc::c_ulong
        {
            (1 as libc::c_uint) << 24 as libc::c_int
        } else {
            size as uint32_t
        };
        if chunk_size > (1 as libc::c_uint) << 16 as libc::c_int {
            nibbles= (if chunk_size > (1 as libc::c_uint) << 20 as libc::c_int {
                2 as libc::c_int
            } else {
                1 as libc::c_int
            }) as uint32_t;
        }
        bits= nibbles << 1 as libc::c_int
            | chunk_size.wrapping_sub(1 as libc::c_int as libc::c_uint)
                << 3 as libc::c_int
            | (1 as libc::c_uint)
                << (19 as libc::c_int as libc::c_uint)
                    .wrapping_add(
                        (4 as libc::c_int as libc::c_uint).wrapping_mul(nibbles),
                    );
        let fresh97 = result;
        result= result.wrapping_add(1);
        *output.offset(fresh97 as isize) = bits as uint8_t;
        let fresh98 = result;
        result= result.wrapping_add(1);
        *output.offset(fresh98 as isize) = (bits >> 8 as libc::c_int) as uint8_t;
        let fresh99 = result;
        result= result.wrapping_add(1);
        *output.offset(fresh99 as isize) = (bits >> 16 as libc::c_int) as uint8_t;
        if nibbles == 2 as libc::c_int as libc::c_uint {
            let fresh100 = result;
            result= result.wrapping_add(1);
            *output.offset(fresh100 as isize) = (bits >> 24 as libc::c_int) as uint8_t;
        }
        memcpy(
            core::ptr::addr_of_mut!(*output.offset(result as isize)) as *mut uint8_t as *mut libc::c_void,
            &*input.offset(offset as isize) as *const uint8_t as *const libc::c_void,
            chunk_size as libc::c_ulong,
        );
        result= (result as libc::c_ulong).wrapping_add(chunk_size as libc::c_ulong)
            as size_t as size_t;
        offset= (offset as libc::c_ulong).wrapping_add(chunk_size as libc::c_ulong)
            as size_t as size_t;
        size= (size as libc::c_ulong).wrapping_sub(chunk_size as libc::c_ulong)
            as size_t as size_t;
    }
    let fresh101 = result;
    result= result.wrapping_add(1);
    *output.offset(fresh101 as isize) = 3 as libc::c_int as uint8_t;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn BrotliEncoderCompress(
    mut quality: libc::c_int,
    mut lgwin: libc::c_int,
    mut mode: BrotliEncoderMode,
    mut input_size: size_t,
    mut input_buffer: *const uint8_t,
    mut encoded_size: Option<&mut size_t>,
    mut encoded_buffer: *mut uint8_t,
) -> libc::c_int {
    let mut s = 0 as *mut BrotliEncoderState;
    let mut out_size = (*encoded_size.as_deref().unwrap());
    let mut input_start = input_buffer;
    let mut output_start = encoded_buffer;
    let mut max_out_size = BrotliEncoderMaxCompressedSize(input_size);
    if out_size == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int;
    }
    if input_size == 0 as libc::c_int as libc::c_ulong {
        *encoded_size.as_deref_mut().unwrap()= 1 as libc::c_int as size_t;
        *encoded_buffer= 6 as libc::c_int as uint8_t;
        return 1 as libc::c_int;
    }
    if quality == 10 as libc::c_int {
        let lg_win = brotli_min_int(
            30 as libc::c_int,
            brotli_max_int(16 as libc::c_int, lgwin),
        );
        let mut ok = BrotliCompressBufferQuality10(
            lg_win,
            input_size,
            input_buffer,
            encoded_size.as_deref_mut(),
            encoded_buffer,
        );
        if !(ok == 0 || max_out_size != 0 && (*encoded_size.as_deref().unwrap()) > max_out_size) {
            return 1 as libc::c_int;
        }
    } else {
        s= BrotliEncoderCreateInstance(None, None, 0 as *mut libc::c_void);
        if s.is_null() {();
            return 0 as libc::c_int
        } else {
            let mut available_in = input_size;
            let mut next_in = input_buffer;
            let mut available_out = (*encoded_size.as_deref().unwrap());
            let mut next_out = encoded_buffer;
            let mut total_out = 0 as libc::c_int as size_t;
            let mut result = 0 as libc::c_int;
            BrotliEncoderSetParameter(s.as_mut(), BROTLI_PARAM_QUALITY, quality as uint32_t);
            BrotliEncoderSetParameter(s.as_mut(), BROTLI_PARAM_LGWIN, lgwin as uint32_t);
            BrotliEncoderSetParameter(s.as_mut(), BROTLI_PARAM_MODE, mode as uint32_t);
            BrotliEncoderSetParameter(s.as_mut(), BROTLI_PARAM_SIZE_HINT, input_size as uint32_t);
            if lgwin > 24 as libc::c_int {
                BrotliEncoderSetParameter(
                    s.as_mut(),
                    BROTLI_PARAM_LARGE_WINDOW,
                    1 as libc::c_int as uint32_t,
                );
            }
            result= BrotliEncoderCompressStream(
                s.as_mut(),
                BROTLI_OPERATION_FINISH,
                core::ptr::addr_of_mut!(available_in),
                core::ptr::addr_of_mut!(next_in),
                core::ptr::addr_of_mut!(available_out),
                core::ptr::addr_of_mut!(next_out),
                Some(&mut total_out),
            );
            if BrotliEncoderIsFinished(s) == 0 {
                result= 0 as libc::c_int;
            }
            *encoded_size.as_deref_mut().unwrap()= total_out;
            BrotliEncoderDestroyInstance(s);
            if !(result == 0 || max_out_size != 0 && (*encoded_size.as_deref().unwrap()) > max_out_size) {
                return 1 as libc::c_int;
            }
        }
    }
    *encoded_size.as_deref_mut().unwrap()= 0 as libc::c_int as size_t;
    if max_out_size == 0 {
        return 0 as libc::c_int;
    }
    if out_size >= max_out_size {
        *encoded_size.as_deref_mut().unwrap()= MakeUncompressedStream(input_start, input_size, output_start);
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn InjectBytePaddingBlock(mut s: *mut BrotliEncoderState) {
    let mut seal = (*s).last_bytes_ as uint32_t;
    let mut seal_bits = (*s).last_bytes_bits_ as size_t;
    let mut destination = 0 as *mut uint8_t;
    (*s).last_bytes_= 0 as libc::c_int as uint16_t;
    (*s).last_bytes_bits_= 0 as libc::c_int as uint8_t;
    seal|= (0x6 as libc::c_uint) << seal_bits;
    seal_bits= (seal_bits as libc::c_ulong)
        .wrapping_add(6 as libc::c_int as libc::c_ulong) as size_t as size_t;
    if !(*s).next_out_.is_null() {
        destination= (*s).next_out_.offset((*s).available_out_ as isize);
    } else {();
        destination= (*s).tiny_buf_.u8_0.as_mut_ptr();
        (*s).next_out_= destination;
    }
    *destination.offset(0 as libc::c_int as isize) = seal as uint8_t;
    if seal_bits > 8 as libc::c_int as libc::c_ulong {
        *destination
            .offset(1 as libc::c_int as isize) = (seal >> 8 as libc::c_int) as uint8_t;
    }
    if seal_bits > 16 as libc::c_int as libc::c_ulong {
        *destination
            .offset(2 as libc::c_int as isize) = (seal >> 16 as libc::c_int) as uint8_t;
    }
    (*s).available_out_= ((*s).available_out_ as libc::c_ulong)
        .wrapping_add(
            seal_bits.wrapping_add(7 as libc::c_int as libc::c_ulong) >> 3 as libc::c_int,
        ) as size_t as size_t;
}
unsafe extern "C" fn InjectFlushOrPushOutput(
    mut s: *mut BrotliEncoderState,
    mut available_out: *mut size_t,
    mut next_out: *mut *mut uint8_t,
    mut total_out: Option<&mut size_t>,
) -> libc::c_int {
    if (*s).stream_state_ as libc::c_uint
        == BROTLI_STREAM_FLUSH_REQUESTED as libc::c_int as libc::c_uint
        && (*s).last_bytes_bits_ as libc::c_int != 0 as libc::c_int
    {
        InjectBytePaddingBlock(s);
        return 1 as libc::c_int;
    }
    if (*s).available_out_ != 0 as libc::c_int as libc::c_ulong
        && (*available_out) != 0 as libc::c_int as libc::c_ulong
    {
        let mut copy_output_size = brotli_min_size_t(
            (*s).available_out_,
            (*available_out),
        );
        memcpy(
            (*next_out) as *mut libc::c_void,
            (*s).next_out_ as *const libc::c_void,
            copy_output_size,
        );
        *next_out= (*next_out).offset(copy_output_size as isize);
        *available_out= ((*available_out) as libc::c_ulong).wrapping_sub(copy_output_size)
            as size_t as size_t;
        (*s).next_out_= (*s).next_out_.offset(copy_output_size as isize);
        (*s).available_out_= ((*s).available_out_ as libc::c_ulong).wrapping_sub(copy_output_size) as size_t
            as size_t;
        (*s).total_out_= ((*s).total_out_ as libc::c_ulong).wrapping_add(copy_output_size) as size_t
            as size_t;
        if !total_out.as_deref().is_none() {
            *total_out.as_deref_mut().unwrap()= (*s).total_out_;
        }else { (); }
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn CheckFlushComplete(mut s: Option<&mut BrotliEncoderState>) {
    if (*s.as_deref().unwrap()).stream_state_ as libc::c_uint
        == BROTLI_STREAM_FLUSH_REQUESTED as libc::c_int as libc::c_uint
        && (*s.as_deref().unwrap()).available_out_ == 0 as libc::c_int as libc::c_ulong
    {
        (*s.as_deref_mut().unwrap()).stream_state_= BROTLI_STREAM_PROCESSING;
        (*s.as_deref_mut().unwrap()).next_out_= 0 as *mut uint8_t;
    }
}
unsafe extern "C" fn BrotliEncoderCompressStreamFast(
    mut s: *mut BrotliEncoderState,
    mut op: BrotliEncoderOperation,
    mut available_in: *mut size_t,
    mut next_in: *mut *const uint8_t,
    mut available_out: *mut size_t,
    mut next_out: *mut *mut uint8_t,
    mut total_out: *mut size_t,
) -> libc::c_int {
    let block_size_limit = (1 as libc::c_int as size_t) << (*s).params.lgwin;
    let buf_size = brotli_min_size_t(
        crate::src::enc::encode::kCompressFragmentTwoPassBlockSize,
        brotli_min_size_t((*available_in), block_size_limit),
    );
    let mut tmp_command_buf = 0 as *mut uint32_t;
    let mut command_buf = 0 as *mut uint32_t;
    let mut tmp_literal_buf = 0 as *mut uint8_t;
    let mut literal_buf = 0 as *mut uint8_t;
    let mut m: *mut crate::src::enc::backward_references_hq::MemoryManager = core::ptr::addr_of_mut!((*s).memory_manager_);
    if (*s).params.quality != 0 as libc::c_int && (*s).params.quality != 1 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if (*s).params.quality == 1 as libc::c_int {
        if (*s).command_buf_.is_null() && buf_size == crate::src::enc::encode::kCompressFragmentTwoPassBlockSize
        {
            (*s).command_buf_= if crate::src::enc::encode::kCompressFragmentTwoPassBlockSize
                > 0 as libc::c_int as libc::c_ulong
            {
                crate::src::enc::memory::BrotliAllocate(
                    m,
                    crate::src::enc::encode::kCompressFragmentTwoPassBlockSize
                        .wrapping_mul(::std::mem::size_of::<uint32_t>() as libc::c_ulong),
                ) as *mut uint32_t
            } else {
                0 as *mut uint32_t
            };
            (*s).literal_buf_= if crate::src::enc::encode::kCompressFragmentTwoPassBlockSize
                > 0 as libc::c_int as libc::c_ulong
            {
                crate::src::enc::memory::BrotliAllocate(
                    m,
                    crate::src::enc::encode::kCompressFragmentTwoPassBlockSize
                        .wrapping_mul(::std::mem::size_of::<uint8_t>() as libc::c_ulong),
                ) as *mut uint8_t
            } else {
                0 as *mut uint8_t
            };
            if 0 as libc::c_int != 0 || 0 as libc::c_int != 0 || 0 as libc::c_int != 0 {
                return 0 as libc::c_int;
            }
        }
        if !(*s).command_buf_.is_null() {
            command_buf= (*s).command_buf_;
            literal_buf= (*s).literal_buf_;
        } else {();
            tmp_command_buf= if buf_size > 0 as libc::c_int as libc::c_ulong {
                crate::src::enc::memory::BrotliAllocate(
                    m,
                    buf_size
                        .wrapping_mul(::std::mem::size_of::<uint32_t>() as libc::c_ulong),
                ) as *mut uint32_t
            } else {
                0 as *mut uint32_t
            };
            tmp_literal_buf= if buf_size > 0 as libc::c_int as libc::c_ulong {
                crate::src::enc::memory::BrotliAllocate(
                    m,
                    buf_size
                        .wrapping_mul(::std::mem::size_of::<uint8_t>() as libc::c_ulong),
                ) as *mut uint8_t
            } else {
                0 as *mut uint8_t
            };
            if 0 as libc::c_int != 0 || 0 as libc::c_int != 0 || 0 as libc::c_int != 0 {
                return 0 as libc::c_int;
            }
            command_buf= tmp_command_buf;
            literal_buf= tmp_literal_buf;
        }
    }
    loop {
        if InjectFlushOrPushOutput(s, available_out, next_out, total_out.as_mut()) != 0 {
            continue;
        }
        if !((*s).available_out_ == 0 as libc::c_int as libc::c_ulong
            && (*s).stream_state_ as libc::c_uint
                == BROTLI_STREAM_PROCESSING as libc::c_int as libc::c_uint
            && ((*available_in) != 0 as libc::c_int as libc::c_ulong
                || op as libc::c_uint
                    != BROTLI_OPERATION_PROCESS as libc::c_int as libc::c_uint))
        {
            break;
        }
        let mut block_size = brotli_min_size_t(block_size_limit, (*available_in));
        let mut is_last = ((*available_in) == block_size
            && op as libc::c_uint
                == BROTLI_OPERATION_FINISH as libc::c_int as libc::c_uint)
            as libc::c_int;
        let mut force_flush = ((*available_in) == block_size
            && op as libc::c_uint
                == BROTLI_OPERATION_FLUSH as libc::c_int as libc::c_uint) as libc::c_int;
        let mut max_out_size = (2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(block_size)
            .wrapping_add(503 as libc::c_int as libc::c_ulong);
        let mut inplace = 1 as libc::c_int;
        let mut storage = 0 as *mut uint8_t;
        let mut storage_ix = (*s).last_bytes_bits_ as size_t;
        let mut table_size: size_t = 0;
        let mut table = 0 as *mut libc::c_int;
        if force_flush != 0 && block_size == 0 as libc::c_int as libc::c_ulong {
            (*s).stream_state_= BROTLI_STREAM_FLUSH_REQUESTED;
        } else {
            if max_out_size <= (*available_out) {
                storage= (*next_out);
            } else {
                inplace= 0 as libc::c_int;
                storage= GetBrotliStorage(s, max_out_size);
                if 0 as libc::c_int != 0 {
                    return 0 as libc::c_int;
                }
            }
            *storage.offset(0 as libc::c_int as isize) = (*s).last_bytes_ as uint8_t;
            *storage
                .offset(
                    1 as libc::c_int as isize,
                ) = ((*s).last_bytes_ as libc::c_int >> 8 as libc::c_int) as uint8_t;
            table= GetHashTable(s, (*s).params.quality, block_size, Some(&mut table_size));
            if 0 as libc::c_int != 0 {
                return 0 as libc::c_int;
            }
            if (*s).params.quality == 0 as libc::c_int {
                crate::src::enc::compress_fragment::BrotliCompressFragmentFast(
                    m,
                    (*next_in),
                    block_size,
                    is_last,
                    table,
                    table_size,
                    (*s).cmd_depths_.as_mut_ptr(),
                    (*s).cmd_bits_.as_mut_ptr(),
                    core::ptr::addr_of_mut!((*s).cmd_code_numbits_),
                    (*s).cmd_code_.as_mut_ptr(),
                    core::ptr::addr_of_mut!(storage_ix),
                    storage,
                );
                if 0 as libc::c_int != 0 {
                    return 0 as libc::c_int;
                }
            } else {
                crate::src::enc::compress_fragment_two_pass::BrotliCompressFragmentTwoPass(
                    m,
                    (*next_in),
                    block_size,
                    is_last,
                    command_buf,
                    literal_buf,
                    table,
                    table_size,
                    core::ptr::addr_of_mut!(storage_ix),
                    storage,
                );
                if 0 as libc::c_int != 0 {
                    return 0 as libc::c_int;
                }
            }
            if block_size != 0 as libc::c_int as libc::c_ulong {
                *next_in= (*next_in).offset(block_size as isize);
                *available_in= ((*available_in) as libc::c_ulong).wrapping_sub(block_size)
                    as size_t as size_t;
            }
            if inplace != 0 {
                let mut out_bytes = storage_ix >> 3 as libc::c_int;
                *next_out= (*next_out).offset(out_bytes as isize);
                *available_out= ((*available_out) as libc::c_ulong)
                    .wrapping_sub(out_bytes) as size_t as size_t;
                (*s).total_out_= ((*s).total_out_ as libc::c_ulong).wrapping_add(out_bytes)
                    as size_t as size_t;
                if !total_out.is_null() {
                    *total_out= (*s).total_out_;
                }else { (); }
            } else {
                let mut out_bytes_0 = storage_ix >> 3 as libc::c_int;
                (*s).next_out_= storage;
                (*s).available_out_= out_bytes_0;
            }
            (*s).last_bytes_= *storage.offset((storage_ix >> 3 as libc::c_int) as isize)
                as uint16_t;
            (*s).last_bytes_bits_= (storage_ix & 7 as libc::c_uint as libc::c_ulong)
                as uint8_t;
            if force_flush != 0 {
                (*s).stream_state_= BROTLI_STREAM_FLUSH_REQUESTED;
            }
            if is_last != 0 {
                (*s).stream_state_= BROTLI_STREAM_FINISHED;
            }
        }
    }
    crate::src::enc::memory::BrotliFree(m, tmp_command_buf as *mut libc::c_void);
    tmp_command_buf= 0 as *mut uint32_t;
    crate::src::enc::memory::BrotliFree(m, tmp_literal_buf as *mut libc::c_void);
    tmp_literal_buf= 0 as *mut uint8_t;
    CheckFlushComplete(s.as_mut());
    return 1 as libc::c_int;
}
unsafe extern "C" fn ProcessMetadata(
    mut s: *mut BrotliEncoderState,
    mut available_in: *mut size_t,
    mut next_in: *mut *const uint8_t,
    mut available_out: *mut size_t,
    mut next_out: *mut *mut uint8_t,
    mut total_out: Option<&mut size_t>,
) -> libc::c_int {
    if (*available_in) > ((1 as libc::c_uint) << 24 as libc::c_int) as libc::c_ulong {
        return 0 as libc::c_int;
    }
    if (*s).stream_state_ as libc::c_uint
        == BROTLI_STREAM_PROCESSING as libc::c_int as libc::c_uint
    {
        (*s).remaining_metadata_bytes_= (*available_in) as uint32_t;
        (*s).stream_state_= BROTLI_STREAM_METADATA_HEAD;
    }
    if (*s).stream_state_ as libc::c_uint
        != BROTLI_STREAM_METADATA_HEAD as libc::c_int as libc::c_uint
        && (*s).stream_state_ as libc::c_uint
            != BROTLI_STREAM_METADATA_BODY as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    loop {
        if InjectFlushOrPushOutput(s, available_out, next_out, total_out.as_deref_mut()) != 0 {
            continue;
        }
        if (*s).available_out_ != 0 as libc::c_int as libc::c_ulong {
            break;
        }
        if (*s).input_pos_ != (*s).last_flush_pos_ {
            let mut result = EncodeData(
                s,
                0 as libc::c_int,
                1 as libc::c_int,
                core::ptr::addr_of_mut!((*s).available_out_),
                core::ptr::addr_of_mut!((*s).next_out_),
            );
            if result == 0 {
                return 0 as libc::c_int;
            }
        } else if (*s).stream_state_ as libc::c_uint
            == BROTLI_STREAM_METADATA_HEAD as libc::c_int as libc::c_uint
        {
            (*s).next_out_= (*s).tiny_buf_.u8_0.as_mut_ptr();
            (*s).available_out_= WriteMetadataHeader(
                s,
                (*s).remaining_metadata_bytes_ as size_t,
                (*s).next_out_,
            );
            (*s).stream_state_= BROTLI_STREAM_METADATA_BODY;
        } else if (*s).remaining_metadata_bytes_ == 0 as libc::c_int as libc::c_uint {
            (*s).remaining_metadata_bytes_= !(0 as libc::c_int as uint32_t);
            (*s).stream_state_= BROTLI_STREAM_PROCESSING;
            break;
        } else if (*available_out) != 0 {
            let mut copy = brotli_min_size_t(
                (*s).remaining_metadata_bytes_ as size_t,
                (*available_out),
            ) as uint32_t;
            memcpy(
                (*next_out) as *mut libc::c_void,
                (*next_in) as *const libc::c_void,
                copy as libc::c_ulong,
            );
            *next_in= (*next_in).offset(copy as isize);
            *available_in= ((*available_in) as libc::c_ulong)
                .wrapping_sub(copy as libc::c_ulong) as size_t as size_t;
            (*s).remaining_metadata_bytes_= ((*s).remaining_metadata_bytes_ as libc::c_uint).wrapping_sub(copy) as uint32_t
                as uint32_t;
            *next_out= (*next_out).offset(copy as isize);
            *available_out= ((*available_out) as libc::c_ulong)
                .wrapping_sub(copy as libc::c_ulong) as size_t as size_t;
        } else {
            let mut copy_0 = brotli_min_uint32_t(
                (*s).remaining_metadata_bytes_,
                16 as libc::c_int as uint32_t,
            );
            (*s).next_out_= (*s).tiny_buf_.u8_0.as_mut_ptr();
            memcpy(
                (*s).next_out_ as *mut libc::c_void,
                (*next_in) as *const libc::c_void,
                copy_0 as libc::c_ulong,
            );
            *next_in= (*next_in).offset(copy_0 as isize);
            *available_in= ((*available_in) as libc::c_ulong)
                .wrapping_sub(copy_0 as libc::c_ulong) as size_t as size_t;
            (*s).remaining_metadata_bytes_= ((*s).remaining_metadata_bytes_ as libc::c_uint).wrapping_sub(copy_0) as uint32_t
                as uint32_t;
            (*s).available_out_= copy_0 as size_t;
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn UpdateSizeHint(
    mut s: Option<&mut BrotliEncoderState>,
    mut available_in: size_t,
) {
    if (*s.as_deref().unwrap()).params.size_hint == 0 as libc::c_int as libc::c_ulong {
        let mut delta = UnprocessedInputSize(s.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()));
        let mut tail = available_in;
        let mut limit = (1 as libc::c_uint) << 30 as libc::c_int;
        let mut total: uint32_t = 0;
        if delta >= limit as libc::c_ulong || tail >= limit as libc::c_ulong
            || delta.wrapping_add(tail) >= limit as libc::c_ulong
        {
            total= limit;
        } else {
            total= delta.wrapping_add(tail) as uint32_t;
        }
        (*s.as_deref_mut().unwrap()).params.size_hint= total as size_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn BrotliEncoderCompressStream(
    mut s: Option<&mut BrotliEncoderState>,
    mut op: BrotliEncoderOperation,
    mut available_in: *mut size_t,
    mut next_in: *mut *const uint8_t,
    mut available_out: *mut size_t,
    mut next_out: *mut *mut uint8_t,
    mut total_out: Option<&mut size_t>,
) -> libc::c_int {
    if EnsureInitialized(s.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut())) == 0 {
        return 0 as libc::c_int;
    }
    if (*s.as_deref().unwrap()).remaining_metadata_bytes_ != !(0 as libc::c_int as uint32_t) {
        if (*available_in) != (*s.as_deref().unwrap()).remaining_metadata_bytes_ as libc::c_ulong {
            return 0 as libc::c_int;
        }
        if op as libc::c_uint
            != BROTLI_OPERATION_EMIT_METADATA as libc::c_int as libc::c_uint
        {
            return 0 as libc::c_int;
        }
    }
    if op as libc::c_uint
        == BROTLI_OPERATION_EMIT_METADATA as libc::c_int as libc::c_uint
    {
        UpdateSizeHint(s.as_deref_mut(), 0 as libc::c_int as size_t);
        return ProcessMetadata(
            s.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()),
            available_in,
            next_in,
            available_out,
            next_out,
            total_out.as_deref_mut(),
        );
    }
    if (*s.as_deref().unwrap()).stream_state_ as libc::c_uint
        == BROTLI_STREAM_METADATA_HEAD as libc::c_int as libc::c_uint
        || (*s.as_deref().unwrap()).stream_state_ as libc::c_uint
            == BROTLI_STREAM_METADATA_BODY as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    if (*s.as_deref().unwrap()).stream_state_ as libc::c_uint
        != BROTLI_STREAM_PROCESSING as libc::c_int as libc::c_uint
        && (*available_in) != 0 as libc::c_int as libc::c_ulong
    {
        return 0 as libc::c_int;
    }
    if (*s.as_deref().unwrap()).params.quality == 0 as libc::c_int || (*s.as_deref().unwrap()).params.quality == 1 as libc::c_int
    {
        return BrotliEncoderCompressStreamFast(
            s.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()),
            op,
            available_in,
            next_in,
            available_out,
            next_out,
            total_out.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()),
        );
    }
    loop {
        let mut remaining_block_size = RemainingInputBlockSize(s.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()));
        if (*s.as_deref().unwrap()).flint_ as libc::c_int >= 0 as libc::c_int
            && remaining_block_size > (*s.as_deref().unwrap()).flint_ as size_t
        {
            remaining_block_size= (*s.as_deref().unwrap()).flint_ as size_t;
        }
        if remaining_block_size != 0 as libc::c_int as libc::c_ulong
            && (*available_in) != 0 as libc::c_int as libc::c_ulong
        {
            let mut copy_input_size = brotli_min_size_t(
                remaining_block_size,
                (*available_in),
            );
            CopyInputToRingBuffer(s.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()), copy_input_size, (*next_in));
            *next_in= (*next_in).offset(copy_input_size as isize);
            *available_in= ((*available_in) as libc::c_ulong)
                .wrapping_sub(copy_input_size) as size_t as size_t;
            if (*s.as_deref().unwrap()).flint_ as libc::c_int > 0 as libc::c_int {
                (*s.as_deref_mut().unwrap()).flint_= ((*s.as_deref().unwrap()).flint_ as libc::c_int
                    - copy_input_size as libc::c_int) as int8_t;
            }
        } else if InjectFlushOrPushOutput(s.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()), available_out, next_out, total_out.as_deref_mut()) != 0 {
            if (*s.as_deref().unwrap()).flint_ as libc::c_int
                == BROTLI_FLINT_WAITING_FOR_FLUSHING as libc::c_int
            {
                CheckFlushComplete(s.as_deref_mut());
                if (*s.as_deref().unwrap()).stream_state_ as libc::c_uint
                    == BROTLI_STREAM_PROCESSING as libc::c_int as libc::c_uint
                {
                    (*s.as_deref_mut().unwrap()).flint_= BROTLI_FLINT_DONE as libc::c_int as int8_t;
                }
            }
        } else {
            if !((*s.as_deref().unwrap()).available_out_ == 0 as libc::c_int as libc::c_ulong
                && (*s.as_deref().unwrap()).stream_state_ as libc::c_uint
                    == BROTLI_STREAM_PROCESSING as libc::c_int as libc::c_uint)
            {
                break;
            }
            if !(remaining_block_size == 0 as libc::c_int as libc::c_ulong
                || op as libc::c_uint
                    != BROTLI_OPERATION_PROCESS as libc::c_int as libc::c_uint)
            {
                break;
            }
            let mut is_last = if (*available_in) == 0 as libc::c_int as libc::c_ulong
                && op as libc::c_uint
                    == BROTLI_OPERATION_FINISH as libc::c_int as libc::c_uint
            {
                1 as libc::c_int
            } else {
                0 as libc::c_int
            };
            let mut force_flush = if (*available_in) == 0 as libc::c_int as libc::c_ulong
                && op as libc::c_uint
                    == BROTLI_OPERATION_FLUSH as libc::c_int as libc::c_uint
            {
                1 as libc::c_int
            } else {
                0 as libc::c_int
            };
            let mut result: libc::c_int = 0;
            if is_last == 0 && (*s.as_deref().unwrap()).flint_ as libc::c_int == 0 as libc::c_int {
                (*s.as_deref_mut().unwrap()).flint_= BROTLI_FLINT_WAITING_FOR_FLUSHING as libc::c_int as int8_t;
                force_flush= 1 as libc::c_int;
            }
            UpdateSizeHint(s.as_deref_mut(), (*available_in));
            result= EncodeData(
                s.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()),
                is_last,
                force_flush,
                core::ptr::addr_of_mut!((*s.as_deref_mut().unwrap()).available_out_),
                core::ptr::addr_of_mut!((*s.as_deref_mut().unwrap()).next_out_),
            );
            if result == 0 {
                return 0 as libc::c_int;
            }
            if force_flush != 0 {
                (*s.as_deref_mut().unwrap()).stream_state_= BROTLI_STREAM_FLUSH_REQUESTED;
            }
            if is_last != 0 {
                (*s.as_deref_mut().unwrap()).stream_state_= BROTLI_STREAM_FINISHED;
            }
        }
    }
    CheckFlushComplete(s.as_deref_mut());
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn BrotliEncoderIsFinished(
    mut s: *mut BrotliEncoderState,
) -> libc::c_int {
    return if (*s).stream_state_ as libc::c_uint
        == BROTLI_STREAM_FINISHED as libc::c_int as libc::c_uint
        && BrotliEncoderHasMoreOutput(s) == 0
    {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn BrotliEncoderHasMoreOutput(
    mut s: *mut BrotliEncoderState,
) -> libc::c_int {
    return if (*s).available_out_ != 0 as libc::c_int as libc::c_ulong {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn BrotliEncoderTakeOutput(
    mut s: *mut BrotliEncoderState,
    mut size: *mut size_t,
) -> *const uint8_t {
    let mut consumed_size = (*s).available_out_;
    let mut result = (*s).next_out_;
    if (*size) != 0 {
        consumed_size= brotli_min_size_t((*size), (*s).available_out_);
    }
    if consumed_size != 0 {
        (*s).next_out_= (*s).next_out_.offset(consumed_size as isize);
        (*s).available_out_= ((*s).available_out_ as libc::c_ulong).wrapping_sub(consumed_size) as size_t
            as size_t;
        (*s).total_out_= ((*s).total_out_ as libc::c_ulong).wrapping_add(consumed_size) as size_t
            as size_t;
        CheckFlushComplete(s.as_mut());
        *size= consumed_size;
    } else {
        *size= 0 as libc::c_int as size_t;
        result= 0 as *mut uint8_t;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn BrotliEncoderVersion() -> uint32_t {
    return 0x1000009 as libc::c_int as uint32_t;
}
