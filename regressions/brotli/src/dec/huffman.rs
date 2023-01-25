use ::libc;
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
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
#[derive(Copy, Clone)]

struct ErasedByPreprocessor5 { dummy: () }
#[inline(always)]
unsafe extern "C" fn ConstructHuffmanCode(
    mut bits: uint8_t,
    mut value: uint16_t,
) -> crate::src::dec::decode::HuffmanCode {
    let mut h = crate::src::dec::decode::HuffmanCode { bits: 0, value: 0 };
    h.bits= bits;
    h.value= value;
    return h;
}
static mut kReverseBits: [uint8_t; 256] = [
    0 as libc::c_int as uint8_t,
    0x80 as libc::c_int as uint8_t,
    0x40 as libc::c_int as uint8_t,
    0xc0 as libc::c_int as uint8_t,
    0x20 as libc::c_int as uint8_t,
    0xa0 as libc::c_int as uint8_t,
    0x60 as libc::c_int as uint8_t,
    0xe0 as libc::c_int as uint8_t,
    0x10 as libc::c_int as uint8_t,
    0x90 as libc::c_int as uint8_t,
    0x50 as libc::c_int as uint8_t,
    0xd0 as libc::c_int as uint8_t,
    0x30 as libc::c_int as uint8_t,
    0xb0 as libc::c_int as uint8_t,
    0x70 as libc::c_int as uint8_t,
    0xf0 as libc::c_int as uint8_t,
    0x8 as libc::c_int as uint8_t,
    0x88 as libc::c_int as uint8_t,
    0x48 as libc::c_int as uint8_t,
    0xc8 as libc::c_int as uint8_t,
    0x28 as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0x68 as libc::c_int as uint8_t,
    0xe8 as libc::c_int as uint8_t,
    0x18 as libc::c_int as uint8_t,
    0x98 as libc::c_int as uint8_t,
    0x58 as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0x38 as libc::c_int as uint8_t,
    0xb8 as libc::c_int as uint8_t,
    0x78 as libc::c_int as uint8_t,
    0xf8 as libc::c_int as uint8_t,
    0x4 as libc::c_int as uint8_t,
    0x84 as libc::c_int as uint8_t,
    0x44 as libc::c_int as uint8_t,
    0xc4 as libc::c_int as uint8_t,
    0x24 as libc::c_int as uint8_t,
    0xa4 as libc::c_int as uint8_t,
    0x64 as libc::c_int as uint8_t,
    0xe4 as libc::c_int as uint8_t,
    0x14 as libc::c_int as uint8_t,
    0x94 as libc::c_int as uint8_t,
    0x54 as libc::c_int as uint8_t,
    0xd4 as libc::c_int as uint8_t,
    0x34 as libc::c_int as uint8_t,
    0xb4 as libc::c_int as uint8_t,
    0x74 as libc::c_int as uint8_t,
    0xf4 as libc::c_int as uint8_t,
    0xc as libc::c_int as uint8_t,
    0x8c as libc::c_int as uint8_t,
    0x4c as libc::c_int as uint8_t,
    0xcc as libc::c_int as uint8_t,
    0x2c as libc::c_int as uint8_t,
    0xac as libc::c_int as uint8_t,
    0x6c as libc::c_int as uint8_t,
    0xec as libc::c_int as uint8_t,
    0x1c as libc::c_int as uint8_t,
    0x9c as libc::c_int as uint8_t,
    0x5c as libc::c_int as uint8_t,
    0xdc as libc::c_int as uint8_t,
    0x3c as libc::c_int as uint8_t,
    0xbc as libc::c_int as uint8_t,
    0x7c as libc::c_int as uint8_t,
    0xfc as libc::c_int as uint8_t,
    0x2 as libc::c_int as uint8_t,
    0x82 as libc::c_int as uint8_t,
    0x42 as libc::c_int as uint8_t,
    0xc2 as libc::c_int as uint8_t,
    0x22 as libc::c_int as uint8_t,
    0xa2 as libc::c_int as uint8_t,
    0x62 as libc::c_int as uint8_t,
    0xe2 as libc::c_int as uint8_t,
    0x12 as libc::c_int as uint8_t,
    0x92 as libc::c_int as uint8_t,
    0x52 as libc::c_int as uint8_t,
    0xd2 as libc::c_int as uint8_t,
    0x32 as libc::c_int as uint8_t,
    0xb2 as libc::c_int as uint8_t,
    0x72 as libc::c_int as uint8_t,
    0xf2 as libc::c_int as uint8_t,
    0xa as libc::c_int as uint8_t,
    0x8a as libc::c_int as uint8_t,
    0x4a as libc::c_int as uint8_t,
    0xca as libc::c_int as uint8_t,
    0x2a as libc::c_int as uint8_t,
    0xaa as libc::c_int as uint8_t,
    0x6a as libc::c_int as uint8_t,
    0xea as libc::c_int as uint8_t,
    0x1a as libc::c_int as uint8_t,
    0x9a as libc::c_int as uint8_t,
    0x5a as libc::c_int as uint8_t,
    0xda as libc::c_int as uint8_t,
    0x3a as libc::c_int as uint8_t,
    0xba as libc::c_int as uint8_t,
    0x7a as libc::c_int as uint8_t,
    0xfa as libc::c_int as uint8_t,
    0x6 as libc::c_int as uint8_t,
    0x86 as libc::c_int as uint8_t,
    0x46 as libc::c_int as uint8_t,
    0xc6 as libc::c_int as uint8_t,
    0x26 as libc::c_int as uint8_t,
    0xa6 as libc::c_int as uint8_t,
    0x66 as libc::c_int as uint8_t,
    0xe6 as libc::c_int as uint8_t,
    0x16 as libc::c_int as uint8_t,
    0x96 as libc::c_int as uint8_t,
    0x56 as libc::c_int as uint8_t,
    0xd6 as libc::c_int as uint8_t,
    0x36 as libc::c_int as uint8_t,
    0xb6 as libc::c_int as uint8_t,
    0x76 as libc::c_int as uint8_t,
    0xf6 as libc::c_int as uint8_t,
    0xe as libc::c_int as uint8_t,
    0x8e as libc::c_int as uint8_t,
    0x4e as libc::c_int as uint8_t,
    0xce as libc::c_int as uint8_t,
    0x2e as libc::c_int as uint8_t,
    0xae as libc::c_int as uint8_t,
    0x6e as libc::c_int as uint8_t,
    0xee as libc::c_int as uint8_t,
    0x1e as libc::c_int as uint8_t,
    0x9e as libc::c_int as uint8_t,
    0x5e as libc::c_int as uint8_t,
    0xde as libc::c_int as uint8_t,
    0x3e as libc::c_int as uint8_t,
    0xbe as libc::c_int as uint8_t,
    0x7e as libc::c_int as uint8_t,
    0xfe as libc::c_int as uint8_t,
    0x1 as libc::c_int as uint8_t,
    0x81 as libc::c_int as uint8_t,
    0x41 as libc::c_int as uint8_t,
    0xc1 as libc::c_int as uint8_t,
    0x21 as libc::c_int as uint8_t,
    0xa1 as libc::c_int as uint8_t,
    0x61 as libc::c_int as uint8_t,
    0xe1 as libc::c_int as uint8_t,
    0x11 as libc::c_int as uint8_t,
    0x91 as libc::c_int as uint8_t,
    0x51 as libc::c_int as uint8_t,
    0xd1 as libc::c_int as uint8_t,
    0x31 as libc::c_int as uint8_t,
    0xb1 as libc::c_int as uint8_t,
    0x71 as libc::c_int as uint8_t,
    0xf1 as libc::c_int as uint8_t,
    0x9 as libc::c_int as uint8_t,
    0x89 as libc::c_int as uint8_t,
    0x49 as libc::c_int as uint8_t,
    0xc9 as libc::c_int as uint8_t,
    0x29 as libc::c_int as uint8_t,
    0xa9 as libc::c_int as uint8_t,
    0x69 as libc::c_int as uint8_t,
    0xe9 as libc::c_int as uint8_t,
    0x19 as libc::c_int as uint8_t,
    0x99 as libc::c_int as uint8_t,
    0x59 as libc::c_int as uint8_t,
    0xd9 as libc::c_int as uint8_t,
    0x39 as libc::c_int as uint8_t,
    0xb9 as libc::c_int as uint8_t,
    0x79 as libc::c_int as uint8_t,
    0xf9 as libc::c_int as uint8_t,
    0x5 as libc::c_int as uint8_t,
    0x85 as libc::c_int as uint8_t,
    0x45 as libc::c_int as uint8_t,
    0xc5 as libc::c_int as uint8_t,
    0x25 as libc::c_int as uint8_t,
    0xa5 as libc::c_int as uint8_t,
    0x65 as libc::c_int as uint8_t,
    0xe5 as libc::c_int as uint8_t,
    0x15 as libc::c_int as uint8_t,
    0x95 as libc::c_int as uint8_t,
    0x55 as libc::c_int as uint8_t,
    0xd5 as libc::c_int as uint8_t,
    0x35 as libc::c_int as uint8_t,
    0xb5 as libc::c_int as uint8_t,
    0x75 as libc::c_int as uint8_t,
    0xf5 as libc::c_int as uint8_t,
    0xd as libc::c_int as uint8_t,
    0x8d as libc::c_int as uint8_t,
    0x4d as libc::c_int as uint8_t,
    0xcd as libc::c_int as uint8_t,
    0x2d as libc::c_int as uint8_t,
    0xad as libc::c_int as uint8_t,
    0x6d as libc::c_int as uint8_t,
    0xed as libc::c_int as uint8_t,
    0x1d as libc::c_int as uint8_t,
    0x9d as libc::c_int as uint8_t,
    0x5d as libc::c_int as uint8_t,
    0xdd as libc::c_int as uint8_t,
    0x3d as libc::c_int as uint8_t,
    0xbd as libc::c_int as uint8_t,
    0x7d as libc::c_int as uint8_t,
    0xfd as libc::c_int as uint8_t,
    0x3 as libc::c_int as uint8_t,
    0x83 as libc::c_int as uint8_t,
    0x43 as libc::c_int as uint8_t,
    0xc3 as libc::c_int as uint8_t,
    0x23 as libc::c_int as uint8_t,
    0xa3 as libc::c_int as uint8_t,
    0x63 as libc::c_int as uint8_t,
    0xe3 as libc::c_int as uint8_t,
    0x13 as libc::c_int as uint8_t,
    0x93 as libc::c_int as uint8_t,
    0x53 as libc::c_int as uint8_t,
    0xd3 as libc::c_int as uint8_t,
    0x33 as libc::c_int as uint8_t,
    0xb3 as libc::c_int as uint8_t,
    0x73 as libc::c_int as uint8_t,
    0xf3 as libc::c_int as uint8_t,
    0xb as libc::c_int as uint8_t,
    0x8b as libc::c_int as uint8_t,
    0x4b as libc::c_int as uint8_t,
    0xcb as libc::c_int as uint8_t,
    0x2b as libc::c_int as uint8_t,
    0xab as libc::c_int as uint8_t,
    0x6b as libc::c_int as uint8_t,
    0xeb as libc::c_int as uint8_t,
    0x1b as libc::c_int as uint8_t,
    0x9b as libc::c_int as uint8_t,
    0x5b as libc::c_int as uint8_t,
    0xdb as libc::c_int as uint8_t,
    0x3b as libc::c_int as uint8_t,
    0xbb as libc::c_int as uint8_t,
    0x7b as libc::c_int as uint8_t,
    0xfb as libc::c_int as uint8_t,
    0x7 as libc::c_int as uint8_t,
    0x87 as libc::c_int as uint8_t,
    0x47 as libc::c_int as uint8_t,
    0xc7 as libc::c_int as uint8_t,
    0x27 as libc::c_int as uint8_t,
    0xa7 as libc::c_int as uint8_t,
    0x67 as libc::c_int as uint8_t,
    0xe7 as libc::c_int as uint8_t,
    0x17 as libc::c_int as uint8_t,
    0x97 as libc::c_int as uint8_t,
    0x57 as libc::c_int as uint8_t,
    0xd7 as libc::c_int as uint8_t,
    0x37 as libc::c_int as uint8_t,
    0xb7 as libc::c_int as uint8_t,
    0x77 as libc::c_int as uint8_t,
    0xf7 as libc::c_int as uint8_t,
    0xf as libc::c_int as uint8_t,
    0x8f as libc::c_int as uint8_t,
    0x4f as libc::c_int as uint8_t,
    0xcf as libc::c_int as uint8_t,
    0x2f as libc::c_int as uint8_t,
    0xaf as libc::c_int as uint8_t,
    0x6f as libc::c_int as uint8_t,
    0xef as libc::c_int as uint8_t,
    0x1f as libc::c_int as uint8_t,
    0x9f as libc::c_int as uint8_t,
    0x5f as libc::c_int as uint8_t,
    0xdf as libc::c_int as uint8_t,
    0x3f as libc::c_int as uint8_t,
    0xbf as libc::c_int as uint8_t,
    0x7f as libc::c_int as uint8_t,
    0xff as libc::c_int as uint8_t,
];
#[inline(always)]
unsafe extern "C" fn BrotliReverseBits(mut num: uint64_t) -> uint64_t {
    return crate::src::dec::huffman::kReverseBits[num as usize] as uint64_t;
}
#[inline(always)]
unsafe extern "C" fn ReplicateValue(
    mut table: *mut crate::src::dec::decode::HuffmanCode,
    mut step: libc::c_int,
    mut end: libc::c_int,
    mut code: crate::src::dec::decode::HuffmanCode,
) {
    loop {
        end-= step;
        *table.offset(end as isize) = code;
        if !(end > 0 as libc::c_int) {
            break;
        }
    };
}
#[inline(always)]
unsafe extern "C" fn NextTableBitSize(
    mut count: *const uint16_t,
    mut len: libc::c_int,
    mut root_bits: libc::c_int,
) -> libc::c_int {
    let mut left = (1 as libc::c_int) << len - root_bits;
    while len < 15 as libc::c_int {
        left-= *count.offset(len as isize) as libc::c_int;
        if left <= 0 as libc::c_int {
            break;
        }
        len+= 1;
        left<<= 1 as libc::c_int;
    }
    return len - root_bits;
}
#[no_mangle]
pub unsafe extern "C" fn BrotliBuildCodeLengthsHuffmanTable(
    mut table: *mut crate::src::dec::decode::HuffmanCode,
    mut code_lengths: *const uint8_t,
    mut count: *mut uint16_t,
) {
    let mut code = crate::src::dec::decode::HuffmanCode { bits: 0, value: 0 };
    let mut symbol: libc::c_int = 0;
    let mut key: uint64_t = 0;
    let mut key_step: uint64_t = 0;
    let mut step: libc::c_int = 0;
    let mut table_size: libc::c_int = 0;
    let mut sorted: [libc::c_int; 18] = [0; 18];
    let mut offset: [libc::c_int; 6] = [0; 6];
    let mut bits: libc::c_int = 0;
    let mut bits_count: libc::c_int = 0;
    symbol= -(1 as libc::c_int);
    bits= 1 as libc::c_int;
    if 5 as libc::c_int & 1 as libc::c_int != 0 as libc::c_int {
        symbol+= *count.offset(bits as isize) as libc::c_int;
        offset[bits as usize]= symbol;
        bits+= 1;
    }
    if 5 as libc::c_int & 2 as libc::c_int != 0 as libc::c_int {
        symbol += *count.offset(bits as isize) as libc::c_int;
        offset[bits as usize] = symbol;
        bits += 1;
        symbol += *count.offset(bits as isize) as libc::c_int;
        offset[bits as usize] = symbol;
        bits += 1;
    }
    if 5 as libc::c_int & 4 as libc::c_int != 0 as libc::c_int {
        symbol+= *count.offset(bits as isize) as libc::c_int;
        offset[bits as usize]= symbol;
        bits+= 1;
        symbol+= *count.offset(bits as isize) as libc::c_int;
        offset[bits as usize]= symbol;
        bits+= 1;
        symbol+= *count.offset(bits as isize) as libc::c_int;
        offset[bits as usize]= symbol;
        bits+= 1;
        symbol+= *count.offset(bits as isize) as libc::c_int;
        offset[bits as usize]= symbol;
        bits+= 1;
    }
    offset[0 as libc::c_int
        as usize]= 17 as libc::c_int + 1 as libc::c_int - 1 as libc::c_int;
    symbol= 17 as libc::c_int + 1 as libc::c_int;
    loop {
        if 6 as libc::c_int & 1 as libc::c_int != 0 as libc::c_int {
            symbol -= 1;
            let fresh0 = offset[*code_lengths.offset(symbol as isize) as usize];
            offset[*code_lengths.offset(symbol as isize)
                as usize] = offset[*code_lengths.offset(symbol as isize) as usize] - 1;
            sorted[fresh0 as usize] = symbol;
        }
        if 6 as libc::c_int & 2 as libc::c_int != 0 as libc::c_int {
            symbol-= 1;
            let fresh1 = offset[*code_lengths.offset(symbol as isize) as usize];
            offset[*code_lengths.offset(symbol as isize)
                as usize]= offset[*code_lengths.offset(symbol as isize) as usize] - 1;
            sorted[fresh1 as usize]= symbol;
            symbol-= 1;
            let fresh2 = offset[*code_lengths.offset(symbol as isize) as usize];
            offset[*code_lengths.offset(symbol as isize)
                as usize]= offset[*code_lengths.offset(symbol as isize) as usize] - 1;
            sorted[fresh2 as usize]= symbol;
        }
        if 6 as libc::c_int & 4 as libc::c_int != 0 as libc::c_int {
            symbol-= 1;
            let fresh3 = offset[*code_lengths.offset(symbol as isize) as usize];
            offset[*code_lengths.offset(symbol as isize)
                as usize]= offset[*code_lengths.offset(symbol as isize) as usize] - 1;
            sorted[fresh3 as usize]= symbol;
            symbol-= 1;
            let fresh4 = offset[*code_lengths.offset(symbol as isize) as usize];
            offset[*code_lengths.offset(symbol as isize)
                as usize]= offset[*code_lengths.offset(symbol as isize) as usize] - 1;
            sorted[fresh4 as usize]= symbol;
            symbol-= 1;
            let fresh5 = offset[*code_lengths.offset(symbol as isize) as usize];
            offset[*code_lengths.offset(symbol as isize)
                as usize]= offset[*code_lengths.offset(symbol as isize) as usize] - 1;
            sorted[fresh5 as usize]= symbol;
            symbol-= 1;
            let fresh6 = offset[*code_lengths.offset(symbol as isize) as usize];
            offset[*code_lengths.offset(symbol as isize)
                as usize]= offset[*code_lengths.offset(symbol as isize) as usize] - 1;
            sorted[fresh6 as usize]= symbol;
        }
        if !(symbol != 0 as libc::c_int) {
            break;
        }
    }
    table_size= (1 as libc::c_int) << 5 as libc::c_int;
    if offset[0 as libc::c_int as usize] == 0 as libc::c_int {
        code= ConstructHuffmanCode(
            0 as libc::c_int as uint8_t,
            sorted[0 as libc::c_int as usize] as uint16_t,
        );
        key= 0 as libc::c_int as uint64_t;
        while key < table_size as uint64_t {
            *table.offset(key as isize) = code;
            key= key.wrapping_add(1);
        }
        return;
    }
    key= 0 as libc::c_int as uint64_t;
    key_step= (1 as libc::c_int as uint64_t)
        << 8 as libc::c_int - 1 as libc::c_int + 0 as libc::c_int;
    symbol= 0 as libc::c_int;
    bits= 1 as libc::c_int;
    step= 2 as libc::c_int;
    loop {
        bits_count= *count.offset(bits as isize) as libc::c_int;
        while bits_count != 0 as libc::c_int {
            let fresh7 = symbol;
            symbol= symbol + 1;
            code= ConstructHuffmanCode(
                bits as uint8_t,
                sorted[fresh7 as usize] as uint16_t,
            );
            ReplicateValue(
                core::ptr::addr_of_mut!(*table
                    .offset(
                        (BrotliReverseBits
                            as unsafe extern "C" fn(uint64_t) -> uint64_t)(key) as isize,
                    )),
                step,
                table_size,
                code,
            );
            key= (key as libc::c_ulong).wrapping_add(key_step) as uint64_t as uint64_t;
            bits_count-= 1;
        }
        step<<= 1 as libc::c_int;
        key_step>>= 1 as libc::c_int;
        bits+= 1;
        if !(bits <= 5 as libc::c_int) {
            break;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn BrotliBuildHuffmanTable(
    mut root_table: *mut crate::src::dec::decode::HuffmanCode,
    mut root_bits: libc::c_int,
    mut symbol_lists: *const uint16_t,
    mut count: *mut uint16_t,
) -> uint32_t {
    let mut code = crate::src::dec::decode::HuffmanCode { bits: 0, value: 0 };
    let mut table = 0 as *mut crate::src::dec::decode::HuffmanCode;
    let mut len: libc::c_int = 0;
    let mut symbol: libc::c_int = 0;
    let mut key: uint64_t = 0;
    let mut key_step: uint64_t = 0;
    let mut sub_key: uint64_t = 0;
    let mut sub_key_step: uint64_t = 0;
    let mut step: libc::c_int = 0;
    let mut table_bits: libc::c_int = 0;
    let mut table_size: libc::c_int = 0;
    let mut total_size: libc::c_int = 0;
    let mut max_length = -(1 as libc::c_int);
    let mut bits: libc::c_int = 0;
    let mut bits_count: libc::c_int = 0;
    while *symbol_lists.offset(max_length as isize) as libc::c_int
        == 0xffff as libc::c_int
    {
        max_length-= 1;
    }
    max_length+= 15 as libc::c_int + 1 as libc::c_int;
    table= root_table;
    table_bits= root_bits;
    table_size= (1 as libc::c_int) << table_bits;
    total_size= table_size;
    if table_bits > max_length {
        table_bits= max_length;
        table_size= (1 as libc::c_int) << table_bits;
    }
    key= 0 as libc::c_int as uint64_t;
    key_step= (1 as libc::c_int as uint64_t)
        << 8 as libc::c_int - 1 as libc::c_int + 0 as libc::c_int;
    bits= 1 as libc::c_int;
    step= 2 as libc::c_int;
    loop {
        symbol= bits - (15 as libc::c_int + 1 as libc::c_int);
        bits_count= *count.offset(bits as isize) as libc::c_int;
        while bits_count != 0 as libc::c_int {
            symbol= *symbol_lists.offset(symbol as isize) as libc::c_int;
            code= ConstructHuffmanCode(bits as uint8_t, symbol as uint16_t);
            ReplicateValue(
                core::ptr::addr_of_mut!(*table
                    .offset(
                        (BrotliReverseBits
                            as unsafe extern "C" fn(uint64_t) -> uint64_t)(key) as isize,
                    )),
                step,
                table_size,
                code,
            );
            key= (key as libc::c_ulong).wrapping_add(key_step) as uint64_t as uint64_t;
            bits_count-= 1;
        }
        step<<= 1 as libc::c_int;
        key_step>>= 1 as libc::c_int;
        bits+= 1;
        if !(bits <= table_bits) {
            break;
        }
    }
    while total_size != table_size {
        memcpy(
            core::ptr::addr_of_mut!(*table.offset(table_size as isize)) as *mut crate::src::dec::decode::HuffmanCode
                as *mut libc::c_void,
            core::ptr::addr_of_mut!(*table.offset(0 as libc::c_int as isize)) as *mut crate::src::dec::decode::HuffmanCode
                as *const libc::c_void,
            (table_size as size_t)
                .wrapping_mul(::std::mem::size_of::<crate::src::dec::decode::HuffmanCode>() as libc::c_ulong),
        );
        table_size<<= 1 as libc::c_int;
    }
    key_step= (1 as libc::c_int as uint64_t)
        << 8 as libc::c_int - 1 as libc::c_int + 0 as libc::c_int
        >> root_bits - 1 as libc::c_int;
    sub_key= ((1 as libc::c_int as uint64_t)
        << 8 as libc::c_int - 1 as libc::c_int + 0 as libc::c_int) << 1 as libc::c_int;
    sub_key_step= (1 as libc::c_int as uint64_t)
        << 8 as libc::c_int - 1 as libc::c_int + 0 as libc::c_int;
    len= root_bits + 1 as libc::c_int;
    step= 2 as libc::c_int;
    while len <= max_length {
        symbol= len - (15 as libc::c_int + 1 as libc::c_int);
        while *count.offset(len as isize) as libc::c_int != 0 as libc::c_int {
            if sub_key
                == ((1 as libc::c_int as uint64_t)
                    << 8 as libc::c_int - 1 as libc::c_int + 0 as libc::c_int)
                    << 1 as libc::c_uint
            {
                table= table.offset(table_size as isize);
                table_bits= NextTableBitSize(count, len, root_bits);
                table_size= (1 as libc::c_int) << table_bits;
                total_size+= table_size;
                sub_key= BrotliReverseBits(key);
                key= (key as libc::c_ulong).wrapping_add(key_step) as uint64_t
                    as uint64_t;
                *root_table
                    .offset(
                        sub_key as isize,
                    ) = ConstructHuffmanCode(
                    (table_bits + root_bits) as uint8_t,
                    (table.offset_from(root_table) as libc::c_long as size_t)
                        .wrapping_sub(sub_key) as uint16_t,
                );
                sub_key= 0 as libc::c_int as uint64_t;
            }
            symbol= *symbol_lists.offset(symbol as isize) as libc::c_int;
            code= ConstructHuffmanCode(
                (len - root_bits) as uint8_t,
                symbol as uint16_t,
            );
            ReplicateValue(
                core::ptr::addr_of_mut!(*table
                    .offset(
                        (BrotliReverseBits
                            as unsafe extern "C" fn(uint64_t) -> uint64_t)(sub_key)
                            as isize,
                    )),
                step,
                table_size,
                code,
            );
            sub_key= (sub_key as libc::c_ulong).wrapping_add(sub_key_step) as uint64_t
                as uint64_t;
            *count.offset(len as isize) = (*count.offset(len as isize)).wrapping_sub(1);
        }
        step<<= 1 as libc::c_int;
        sub_key_step>>= 1 as libc::c_int;
        len+= 1;
    }
    return total_size as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn BrotliBuildSimpleHuffmanTable(
    mut table: *mut crate::src::dec::decode::HuffmanCode,
    mut root_bits: libc::c_int,
    mut val: *mut uint16_t,
    mut num_symbols: uint32_t,
) -> uint32_t {
    let mut table_size = 1 as libc::c_int as uint32_t;
    let goal_size = (1 as libc::c_uint) << root_bits;
    match num_symbols {
        0 => {
            *table
                .offset(
                    0 as libc::c_int as isize,
                ) = ConstructHuffmanCode(
                0 as libc::c_int as uint8_t,
                *val.offset(0 as libc::c_int as isize),
            );
        }
        1 => {
            if *val.offset(1 as libc::c_int as isize) as libc::c_int
                > *val.offset(0 as libc::c_int as isize) as libc::c_int
            {
                *table
                    .offset(
                        0 as libc::c_int as isize,
                    ) = ConstructHuffmanCode(
                    1 as libc::c_int as uint8_t,
                    *val.offset(0 as libc::c_int as isize),
                );
                *table
                    .offset(
                        1 as libc::c_int as isize,
                    ) = ConstructHuffmanCode(
                    1 as libc::c_int as uint8_t,
                    *val.offset(1 as libc::c_int as isize),
                );
            } else {
                *table
                    .offset(
                        0 as libc::c_int as isize,
                    ) = ConstructHuffmanCode(
                    1 as libc::c_int as uint8_t,
                    *val.offset(1 as libc::c_int as isize),
                );
                *table
                    .offset(
                        1 as libc::c_int as isize,
                    ) = ConstructHuffmanCode(
                    1 as libc::c_int as uint8_t,
                    *val.offset(0 as libc::c_int as isize),
                );
            }
            table_size= 2 as libc::c_int as uint32_t;
        }
        2 => {
            *table
                .offset(
                    0 as libc::c_int as isize,
                ) = ConstructHuffmanCode(
                1 as libc::c_int as uint8_t,
                *val.offset(0 as libc::c_int as isize),
            );
            *table
                .offset(
                    2 as libc::c_int as isize,
                ) = ConstructHuffmanCode(
                1 as libc::c_int as uint8_t,
                *val.offset(0 as libc::c_int as isize),
            );
            if *val.offset(2 as libc::c_int as isize) as libc::c_int
                > *val.offset(1 as libc::c_int as isize) as libc::c_int
            {
                *table
                    .offset(
                        1 as libc::c_int as isize,
                    ) = ConstructHuffmanCode(
                    2 as libc::c_int as uint8_t,
                    *val.offset(1 as libc::c_int as isize),
                );
                *table
                    .offset(
                        3 as libc::c_int as isize,
                    ) = ConstructHuffmanCode(
                    2 as libc::c_int as uint8_t,
                    *val.offset(2 as libc::c_int as isize),
                );
            } else {
                *table
                    .offset(
                        1 as libc::c_int as isize,
                    ) = ConstructHuffmanCode(
                    2 as libc::c_int as uint8_t,
                    *val.offset(2 as libc::c_int as isize),
                );
                *table
                    .offset(
                        3 as libc::c_int as isize,
                    ) = ConstructHuffmanCode(
                    2 as libc::c_int as uint8_t,
                    *val.offset(1 as libc::c_int as isize),
                );
            }
            table_size= 4 as libc::c_int as uint32_t;
        }
        3 => {
            let mut i: libc::c_int = 0;
            let mut k: libc::c_int = 0;
            i= 0 as libc::c_int;
            while i < 3 as libc::c_int {
                k= i + 1 as libc::c_int;
                while k < 4 as libc::c_int {
                    if (*val.offset(k as isize) as libc::c_int)
                        < *val.offset(i as isize) as libc::c_int
                    {
                        let mut t = *val.offset(k as isize);
                        *val.offset(k as isize) = *val.offset(i as isize);
                        *val.offset(i as isize) = t;
                    }
                    k+= 1;
                }
                i+= 1;
            }
            *table
                .offset(
                    0 as libc::c_int as isize,
                ) = ConstructHuffmanCode(
                2 as libc::c_int as uint8_t,
                *val.offset(0 as libc::c_int as isize),
            );
            *table
                .offset(
                    2 as libc::c_int as isize,
                ) = ConstructHuffmanCode(
                2 as libc::c_int as uint8_t,
                *val.offset(1 as libc::c_int as isize),
            );
            *table
                .offset(
                    1 as libc::c_int as isize,
                ) = ConstructHuffmanCode(
                2 as libc::c_int as uint8_t,
                *val.offset(2 as libc::c_int as isize),
            );
            *table
                .offset(
                    3 as libc::c_int as isize,
                ) = ConstructHuffmanCode(
                2 as libc::c_int as uint8_t,
                *val.offset(3 as libc::c_int as isize),
            );
            table_size= 4 as libc::c_int as uint32_t;
        }
        4 => {
            if (*val.offset(3 as libc::c_int as isize) as libc::c_int)
                < *val.offset(2 as libc::c_int as isize) as libc::c_int
            {
                let mut t_0 = *val.offset(3 as libc::c_int as isize);
                *val
                    .offset(
                        3 as libc::c_int as isize,
                    ) = *val.offset(2 as libc::c_int as isize);
                *val.offset(2 as libc::c_int as isize) = t_0;
            }
            *table
                .offset(
                    0 as libc::c_int as isize,
                ) = ConstructHuffmanCode(
                1 as libc::c_int as uint8_t,
                *val.offset(0 as libc::c_int as isize),
            );
            *table
                .offset(
                    1 as libc::c_int as isize,
                ) = ConstructHuffmanCode(
                2 as libc::c_int as uint8_t,
                *val.offset(1 as libc::c_int as isize),
            );
            *table
                .offset(
                    2 as libc::c_int as isize,
                ) = ConstructHuffmanCode(
                1 as libc::c_int as uint8_t,
                *val.offset(0 as libc::c_int as isize),
            );
            *table
                .offset(
                    3 as libc::c_int as isize,
                ) = ConstructHuffmanCode(
                3 as libc::c_int as uint8_t,
                *val.offset(2 as libc::c_int as isize),
            );
            *table
                .offset(
                    4 as libc::c_int as isize,
                ) = ConstructHuffmanCode(
                1 as libc::c_int as uint8_t,
                *val.offset(0 as libc::c_int as isize),
            );
            *table
                .offset(
                    5 as libc::c_int as isize,
                ) = ConstructHuffmanCode(
                2 as libc::c_int as uint8_t,
                *val.offset(1 as libc::c_int as isize),
            );
            *table
                .offset(
                    6 as libc::c_int as isize,
                ) = ConstructHuffmanCode(
                1 as libc::c_int as uint8_t,
                *val.offset(0 as libc::c_int as isize),
            );
            *table
                .offset(
                    7 as libc::c_int as isize,
                ) = ConstructHuffmanCode(
                3 as libc::c_int as uint8_t,
                *val.offset(3 as libc::c_int as isize),
            );
            table_size= 8 as libc::c_int as uint32_t;
        }
        _ => {}
    }
    while table_size != goal_size {
        memcpy(
            core::ptr::addr_of_mut!(*table.offset(table_size as isize)) as *mut crate::src::dec::decode::HuffmanCode
                as *mut libc::c_void,
            core::ptr::addr_of_mut!(*table.offset(0 as libc::c_int as isize)) as *mut crate::src::dec::decode::HuffmanCode
                as *const libc::c_void,
            (table_size as size_t)
                .wrapping_mul(::std::mem::size_of::<crate::src::dec::decode::HuffmanCode>() as libc::c_ulong),
        );
        table_size<<= 1 as libc::c_int;
    }
    return goal_size;
}
