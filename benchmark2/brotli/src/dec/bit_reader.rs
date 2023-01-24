use ::libc;
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
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
pub struct BrotliBitReaderState {
    pub val_: uint64_t,
    pub bit_pos_: uint32_t,
    pub next_in: *const uint8_t,
    pub avail_in: size_t,
}
#[inline(always)]
unsafe extern "C" fn BrotliBitReaderSaveState(
    from: *mut BrotliBitReader,
    mut to: *mut BrotliBitReaderState,
) {
    (*to).val_ = (*from).val_;
    (*to).bit_pos_ = (*from).bit_pos_;
    let ref mut fresh0 = (*to).next_in;
    *fresh0 = (*from).next_in;
    (*to).avail_in = (*from).avail_in;
}
#[inline(always)]
unsafe extern "C" fn BrotliGetBitsUnmasked(br: *mut BrotliBitReader) -> uint64_t {
    return (*br).val_ >> (*br).bit_pos_;
}
#[inline(always)]
unsafe extern "C" fn BrotliDropBits(br: *mut BrotliBitReader, mut n_bits: uint32_t) {
    let ref mut fresh1 = (*br).bit_pos_;
    *fresh1 = (*fresh1 as libc::c_uint).wrapping_add(n_bits) as uint32_t as uint32_t;
}
#[inline(always)]
unsafe extern "C" fn BrotliTakeBits(
    br: *mut BrotliBitReader,
    mut n_bits: uint32_t,
    mut val: *mut uint32_t,
) {
    *val = BrotliGetBitsUnmasked(br) as uint32_t & BitMask(n_bits);
    BrotliDropBits(br, n_bits);
}
#[inline(always)]
unsafe extern "C" fn BrotliSafeReadBits(
    br: *mut BrotliBitReader,
    mut n_bits: uint32_t,
    mut val: *mut uint32_t,
) -> libc::c_int {
    while BrotliGetAvailableBits(br) < n_bits {
        if BrotliPullByte(br) == 0 {
            return 0 as libc::c_int;
        }
    }
    BrotliTakeBits(br, n_bits, val);
    return 1 as libc::c_int;
}
#[inline(always)]
unsafe extern "C" fn BrotliBitReaderRestoreState(
    to: *mut BrotliBitReader,
    mut from: *mut BrotliBitReaderState,
) {
    (*to).val_ = (*from).val_;
    (*to).bit_pos_ = (*from).bit_pos_;
    let ref mut fresh2 = (*to).next_in;
    *fresh2 = (*from).next_in;
    (*to).avail_in = (*from).avail_in;
}
#[inline(always)]
unsafe extern "C" fn BrotliGetAvailableBits(mut br: *const BrotliBitReader) -> uint32_t {
    return ((if 1 as libc::c_int != 0 { 64 as libc::c_int } else { 32 as libc::c_int })
        as libc::c_uint)
        .wrapping_sub((*br).bit_pos_);
}
#[inline(always)]
unsafe extern "C" fn BrotliPullByte(br: *mut BrotliBitReader) -> libc::c_int {
    if (*br).avail_in == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int;
    }
    (*br).val_ >>= 8 as libc::c_int;
    let ref mut fresh3 = (*br).val_;
    *fresh3 |= (*(*br).next_in as uint64_t) << 56 as libc::c_int;
    let ref mut fresh4 = (*br).bit_pos_;
    *fresh4 = (*fresh4 as libc::c_uint).wrapping_sub(8 as libc::c_int as libc::c_uint)
        as uint32_t as uint32_t;
    let ref mut fresh5 = (*br).avail_in;
    *fresh5 = (*fresh5).wrapping_sub(1);
    let ref mut fresh6 = (*br).next_in;
    *fresh6 = (*fresh6).offset(1);
    return 1 as libc::c_int;
}
#[inline(always)]
unsafe extern "C" fn BitMask(mut n: uint32_t) -> uint32_t {
    if 0 != 0 || 0 as libc::c_int != 0 {
        return !((0xffffffff as libc::c_uint) << n)
    } else {
        return kBrotliBitMask[n as usize]
    };
}
#[no_mangle]
pub static mut kBrotliBitMask: [uint32_t; 33] = [
    0 as libc::c_int as uint32_t,
    0x1 as libc::c_int as uint32_t,
    0x3 as libc::c_int as uint32_t,
    0x7 as libc::c_int as uint32_t,
    0xf as libc::c_int as uint32_t,
    0x1f as libc::c_int as uint32_t,
    0x3f as libc::c_int as uint32_t,
    0x7f as libc::c_int as uint32_t,
    0xff as libc::c_int as uint32_t,
    0x1ff as libc::c_int as uint32_t,
    0x3ff as libc::c_int as uint32_t,
    0x7ff as libc::c_int as uint32_t,
    0xfff as libc::c_int as uint32_t,
    0x1fff as libc::c_int as uint32_t,
    0x3fff as libc::c_int as uint32_t,
    0x7fff as libc::c_int as uint32_t,
    0xffff as libc::c_int as uint32_t,
    0x1ffff as libc::c_int as uint32_t,
    0x3ffff as libc::c_int as uint32_t,
    0x7ffff as libc::c_int as uint32_t,
    0xfffff as libc::c_int as uint32_t,
    0x1fffff as libc::c_int as uint32_t,
    0x3fffff as libc::c_int as uint32_t,
    0x7fffff as libc::c_int as uint32_t,
    0xffffff as libc::c_int as uint32_t,
    0x1ffffff as libc::c_int as uint32_t,
    0x3ffffff as libc::c_int as uint32_t,
    0x7ffffff as libc::c_int as uint32_t,
    0xfffffff as libc::c_int as uint32_t,
    0x1fffffff as libc::c_int as uint32_t,
    0x3fffffff as libc::c_int as uint32_t,
    0x7fffffff as libc::c_int as uint32_t,
    0xffffffff as libc::c_uint,
];
#[no_mangle]
pub unsafe extern "C" fn BrotliInitBitReader(br: *mut BrotliBitReader) {
    (*br).val_ = 0 as libc::c_int as uint64_t;
    (*br)
        .bit_pos_ = ((::std::mem::size_of::<uint64_t>() as libc::c_ulong)
        << 3 as libc::c_int) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn BrotliWarmupBitReader(br: *mut BrotliBitReader) -> libc::c_int {
    let mut aligned_read_mask = (::std::mem::size_of::<uint64_t>() as libc::c_ulong
        >> 1 as libc::c_int)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong);
    if 0 as libc::c_int == 0 {
        aligned_read_mask = 0 as libc::c_int as size_t;
    }
    if BrotliGetAvailableBits(br) == 0 as libc::c_int as libc::c_uint {
        if BrotliPullByte(br) == 0 {
            return 0 as libc::c_int;
        }
    }
    while (*br).next_in as size_t & aligned_read_mask
        != 0 as libc::c_int as libc::c_ulong
    {
        if BrotliPullByte(br) == 0 {
            return 1 as libc::c_int;
        }
    }
    return 1 as libc::c_int;
}
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn BrotliSafeReadBits32Slow(
    br: *mut BrotliBitReader,
    mut n_bits: uint32_t,
    mut val: *mut uint32_t,
) -> libc::c_int {
    let mut low_val: uint32_t = 0;
    let mut high_val: uint32_t = 0;
    let mut memento = BrotliBitReaderState {
        val_: 0,
        bit_pos_: 0,
        next_in: 0 as *const uint8_t,
        avail_in: 0,
    };
    BrotliBitReaderSaveState(br, &mut memento);
    if BrotliSafeReadBits(br, 16 as libc::c_int as uint32_t, &mut low_val) == 0
        || BrotliSafeReadBits(
            br,
            n_bits.wrapping_sub(16 as libc::c_int as libc::c_uint),
            &mut high_val,
        ) == 0
    {
        BrotliBitReaderRestoreState(br, &mut memento);
        return 0 as libc::c_int;
    }
    *val = low_val | high_val << 16 as libc::c_int;
    return 1 as libc::c_int;
}
