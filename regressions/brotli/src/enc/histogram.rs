use ::libc;
extern "C" {
    static _kBrotliContextLookupTable: [uint8_t; 2048];
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type ContextType = libc::c_uint;
pub const CONTEXT_SIGNED: ContextType = 3;
pub const CONTEXT_UTF8: ContextType = 2;
pub const CONTEXT_MSB6: ContextType = 1;
pub const CONTEXT_LSB6: ContextType = 0;
pub type ContextLut = *const uint8_t;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor121 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor122 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor123 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor124 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor125 { dummy: () }
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BlockSplitIterator {
    pub split_: *const crate::src::enc::block_splitter::BlockSplit,
    pub idx_: size_t,
    pub type_: size_t,
    pub length_: size_t,
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
unsafe extern "C" fn HistogramAddCommand(
    mut self_0: Option<&mut crate::src::enc::bit_cost::HistogramCommand>,
    mut val: size_t,
) {
    (*self_0.as_deref_mut().unwrap()).data_[val as usize]= (*self_0.as_deref().unwrap()).data_[val as usize].wrapping_add(1);
    (*self_0.as_deref_mut().unwrap()).total_count_= (*self_0.as_deref().unwrap()).total_count_.wrapping_add(1);
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
unsafe extern "C" fn InitBlockSplitIterator(
    mut self_0: Option<&mut BlockSplitIterator>,
    mut split: *const crate::src::enc::block_splitter::BlockSplit,
) {
    (*self_0.as_deref_mut().unwrap()).split_= split;
    (*self_0.as_deref_mut().unwrap()).idx_= 0 as libc::c_int as size_t;
    (*self_0.as_deref_mut().unwrap()).type_= 0 as libc::c_int as size_t;
    (*self_0.as_deref_mut().unwrap()).length_= (if !(*split).lengths.is_null() {
        *(*split).lengths.offset(0 as libc::c_int as isize)
    } else {();
        0 as libc::c_int as libc::c_uint
    }) as size_t;
}
unsafe extern "C" fn BlockSplitIteratorNext(mut self_0: Option<&mut BlockSplitIterator>) {
    if (*self_0.as_deref().unwrap()).length_ == 0 as libc::c_int as libc::c_ulong {
        (*self_0.as_deref_mut().unwrap()).idx_= (*self_0.as_deref().unwrap()).idx_.wrapping_add(1);
        (*self_0.as_deref_mut().unwrap()).type_= *(*(*self_0.as_deref().unwrap()).split_).types.offset((*self_0.as_deref().unwrap()).idx_ as isize)
            as size_t;
        (*self_0.as_deref_mut().unwrap()).length_= *(*(*self_0.as_deref().unwrap()).split_).lengths.offset((*self_0.as_deref().unwrap()).idx_ as isize)
            as size_t;
    }
    (*self_0.as_deref_mut().unwrap()).length_= (*self_0.as_deref().unwrap()).length_.wrapping_sub(1);
}
#[no_mangle]
pub unsafe extern "C" fn BrotliBuildHistogramsWithContext(
    mut cmds: *const crate::src::enc::backward_references::Command,
    mut num_commands: size_t,
    mut literal_split: *const crate::src::enc::block_splitter::BlockSplit,
    mut insert_and_copy_split: *const crate::src::enc::block_splitter::BlockSplit,
    mut dist_split: *const crate::src::enc::block_splitter::BlockSplit,
    mut ringbuffer: *const uint8_t,
    mut start_pos: size_t,
    mut mask: size_t,
    mut prev_byte: uint8_t,
    mut prev_byte2: uint8_t,
    mut context_modes: *const ContextType,
    mut literal_histograms: *mut crate::src::enc::bit_cost::HistogramLiteral,
    mut insert_and_copy_histograms: *mut crate::src::enc::bit_cost::HistogramCommand,
    mut copy_dist_histograms: *mut crate::src::enc::bit_cost::HistogramDistance,
) {
    let mut pos = start_pos;
    let mut literal_it = BlockSplitIterator {
        split_: 0 as *const crate::src::enc::block_splitter::BlockSplit,
        idx_: 0,
        type_: 0,
        length_: 0,
    };
    let mut insert_and_copy_it = BlockSplitIterator {
        split_: 0 as *const crate::src::enc::block_splitter::BlockSplit,
        idx_: 0,
        type_: 0,
        length_: 0,
    };
    let mut dist_it = BlockSplitIterator {
        split_: 0 as *const crate::src::enc::block_splitter::BlockSplit,
        idx_: 0,
        type_: 0,
        length_: 0,
    };
    let mut i: size_t = 0;
    InitBlockSplitIterator(Some(&mut literal_it), literal_split);
    InitBlockSplitIterator(Some(&mut insert_and_copy_it), insert_and_copy_split);
    InitBlockSplitIterator(Some(&mut dist_it), dist_split);
    i= 0 as libc::c_int as size_t;
    while i < num_commands {
        let mut cmd: *const crate::src::enc::backward_references::Command = &*cmds.offset(i as isize) as *const crate::src::enc::backward_references::Command;
        let mut j: size_t = 0;
        BlockSplitIteratorNext(Some(&mut insert_and_copy_it));
        HistogramAddCommand(
            Some(&mut *insert_and_copy_histograms.offset(insert_and_copy_it.type_ as isize)),
            (*cmd).cmd_prefix_ as size_t,
        );
        j= (*cmd).insert_len_ as size_t;
        while j != 0 as libc::c_int as libc::c_ulong {
            let mut context: size_t = 0;
            BlockSplitIteratorNext(Some(&mut literal_it));
            context= literal_it.type_;
            if !context_modes.is_null() {
                let mut lut: ContextLut = &*_kBrotliContextLookupTable
                    .as_ptr()
                    .offset(
                        ((*context_modes.offset(context as isize) as libc::c_uint)
                            << 9 as libc::c_int) as isize,
                    ) as *const uint8_t;
                context= (context << 6 as libc::c_int)
                    .wrapping_add(
                        (*lut.offset(prev_byte as isize) as libc::c_int
                            | *lut
                                .offset(256 as libc::c_int as isize)
                                .offset(prev_byte2 as isize) as libc::c_int)
                            as libc::c_ulong,
                    );
            }else { (); }
            HistogramAddLiteral(
                Some(&mut *literal_histograms.offset(context as isize)),
                *ringbuffer.offset((pos & mask) as isize) as size_t,
            );
            prev_byte2= prev_byte;
            prev_byte= *ringbuffer.offset((pos & mask) as isize);
            pos= pos.wrapping_add(1);
            j= j.wrapping_sub(1);
        }
        pos= (pos as libc::c_ulong).wrapping_add(CommandCopyLen(cmd) as libc::c_ulong)
            as size_t as size_t;
        if CommandCopyLen(cmd) != 0 {
            prev_byte2= *ringbuffer
                .offset(
                    (pos.wrapping_sub(2 as libc::c_int as libc::c_ulong) & mask) as isize,
                );
            prev_byte= *ringbuffer
                .offset(
                    (pos.wrapping_sub(1 as libc::c_int as libc::c_ulong) & mask) as isize,
                );
            if (*cmd).cmd_prefix_ as libc::c_int >= 128 as libc::c_int {
                let mut context_0: size_t = 0;
                BlockSplitIteratorNext(Some(&mut dist_it));
                context_0= (dist_it.type_ << 2 as libc::c_int)
                    .wrapping_add(CommandDistanceContext(cmd) as libc::c_ulong);
                HistogramAddDistance(
                    Some(&mut *copy_dist_histograms.offset(context_0 as isize)),
                    ((*cmd).dist_prefix_ as libc::c_int & 0x3ff as libc::c_int) as size_t,
                );
            }
        }
        i= i.wrapping_add(1);
    }
}
