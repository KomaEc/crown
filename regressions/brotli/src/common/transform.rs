use ::libc;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type int16_t = __int16_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type BrotliWordTransformType = libc::c_uint;
pub const BROTLI_NUM_TRANSFORM_TYPES: BrotliWordTransformType = 23;
pub const BROTLI_TRANSFORM_SHIFT_ALL: BrotliWordTransformType = 22;
pub const BROTLI_TRANSFORM_SHIFT_FIRST: BrotliWordTransformType = 21;
pub const BROTLI_TRANSFORM_OMIT_FIRST_9: BrotliWordTransformType = 20;
pub const BROTLI_TRANSFORM_OMIT_FIRST_8: BrotliWordTransformType = 19;
pub const BROTLI_TRANSFORM_OMIT_FIRST_7: BrotliWordTransformType = 18;
pub const BROTLI_TRANSFORM_OMIT_FIRST_6: BrotliWordTransformType = 17;
pub const BROTLI_TRANSFORM_OMIT_FIRST_5: BrotliWordTransformType = 16;
pub const BROTLI_TRANSFORM_OMIT_FIRST_4: BrotliWordTransformType = 15;
pub const BROTLI_TRANSFORM_OMIT_FIRST_3: BrotliWordTransformType = 14;
pub const BROTLI_TRANSFORM_OMIT_FIRST_2: BrotliWordTransformType = 13;
pub const BROTLI_TRANSFORM_OMIT_FIRST_1: BrotliWordTransformType = 12;
pub const BROTLI_TRANSFORM_UPPERCASE_ALL: BrotliWordTransformType = 11;
pub const BROTLI_TRANSFORM_UPPERCASE_FIRST: BrotliWordTransformType = 10;
pub const BROTLI_TRANSFORM_OMIT_LAST_9: BrotliWordTransformType = 9;
pub const BROTLI_TRANSFORM_OMIT_LAST_8: BrotliWordTransformType = 8;
pub const BROTLI_TRANSFORM_OMIT_LAST_7: BrotliWordTransformType = 7;
pub const BROTLI_TRANSFORM_OMIT_LAST_6: BrotliWordTransformType = 6;
pub const BROTLI_TRANSFORM_OMIT_LAST_5: BrotliWordTransformType = 5;
pub const BROTLI_TRANSFORM_OMIT_LAST_4: BrotliWordTransformType = 4;
pub const BROTLI_TRANSFORM_OMIT_LAST_3: BrotliWordTransformType = 3;
pub const BROTLI_TRANSFORM_OMIT_LAST_2: BrotliWordTransformType = 2;
pub const BROTLI_TRANSFORM_OMIT_LAST_1: BrotliWordTransformType = 1;
pub const BROTLI_TRANSFORM_IDENTITY: BrotliWordTransformType = 0;
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
static mut kPrefixSuffix: [libc::c_char; 217] = unsafe {
    *::std::mem::transmute::<
        &[u8; 217],
        &[libc::c_char; 217],
    >(
        b"\x01 \x02, \x08 of the \x04 of \x02s \x01.\x05 and \x04 in \x01\"\x04 to \x02\">\x01\n\x02. \x01]\x05 for \x03 a \x06 that \x01'\x06 with \x06 from \x04 by \x01(\x06. The \x04 on \x04 as \x04 is \x04ing \x02\n\t\x01:\x03ed \x02=\"\x04 at \x03ly \x01,\x02='\x05.com/\x07. This \x05 not \x03er \x03al \x04ful \x04ive \x05less \x04est \x04ize \x02\xC2\xA0\x04ous \x05 the \x02e \0",
    )
};
static mut kPrefixSuffixMap: [uint16_t; 50] = [
    0 as libc::c_int as uint16_t,
    0x2 as libc::c_int as uint16_t,
    0x5 as libc::c_int as uint16_t,
    0xe as libc::c_int as uint16_t,
    0x13 as libc::c_int as uint16_t,
    0x16 as libc::c_int as uint16_t,
    0x18 as libc::c_int as uint16_t,
    0x1e as libc::c_int as uint16_t,
    0x23 as libc::c_int as uint16_t,
    0x25 as libc::c_int as uint16_t,
    0x2a as libc::c_int as uint16_t,
    0x2d as libc::c_int as uint16_t,
    0x2f as libc::c_int as uint16_t,
    0x32 as libc::c_int as uint16_t,
    0x34 as libc::c_int as uint16_t,
    0x3a as libc::c_int as uint16_t,
    0x3e as libc::c_int as uint16_t,
    0x45 as libc::c_int as uint16_t,
    0x47 as libc::c_int as uint16_t,
    0x4e as libc::c_int as uint16_t,
    0x55 as libc::c_int as uint16_t,
    0x5a as libc::c_int as uint16_t,
    0x5c as libc::c_int as uint16_t,
    0x63 as libc::c_int as uint16_t,
    0x68 as libc::c_int as uint16_t,
    0x6d as libc::c_int as uint16_t,
    0x72 as libc::c_int as uint16_t,
    0x77 as libc::c_int as uint16_t,
    0x7a as libc::c_int as uint16_t,
    0x7c as libc::c_int as uint16_t,
    0x80 as libc::c_int as uint16_t,
    0x83 as libc::c_int as uint16_t,
    0x88 as libc::c_int as uint16_t,
    0x8c as libc::c_int as uint16_t,
    0x8e as libc::c_int as uint16_t,
    0x91 as libc::c_int as uint16_t,
    0x97 as libc::c_int as uint16_t,
    0x9f as libc::c_int as uint16_t,
    0xa5 as libc::c_int as uint16_t,
    0xa9 as libc::c_int as uint16_t,
    0xad as libc::c_int as uint16_t,
    0xb2 as libc::c_int as uint16_t,
    0xb7 as libc::c_int as uint16_t,
    0xbd as libc::c_int as uint16_t,
    0xc2 as libc::c_int as uint16_t,
    0xc7 as libc::c_int as uint16_t,
    0xca as libc::c_int as uint16_t,
    0xcf as libc::c_int as uint16_t,
    0xd5 as libc::c_int as uint16_t,
    0xd8 as libc::c_int as uint16_t,
];
static mut kTransformsData: [uint8_t; 363] = [
    49 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_OMIT_FIRST_1 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_UPPERCASE_FIRST as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as libc::c_int as uint8_t,
    47 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    4 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_UPPERCASE_FIRST as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_OMIT_FIRST_2 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_OMIT_LAST_1 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_UPPERCASE_FIRST as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as libc::c_int as uint8_t,
    7 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    48 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as libc::c_int as uint8_t,
    8 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_OMIT_LAST_3 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as libc::c_int as uint8_t,
    14 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_OMIT_FIRST_3 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_OMIT_LAST_2 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as libc::c_int as uint8_t,
    16 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_UPPERCASE_FIRST as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as libc::c_int as uint8_t,
    12 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_OMIT_FIRST_4 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as libc::c_int as uint8_t,
    18 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as libc::c_int as uint8_t,
    17 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as libc::c_int as uint8_t,
    19 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as libc::c_int as uint8_t,
    20 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_OMIT_FIRST_5 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_OMIT_FIRST_6 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    47 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_OMIT_LAST_4 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as libc::c_int as uint8_t,
    22 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_UPPERCASE_ALL as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as libc::c_int as uint8_t,
    23 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as libc::c_int as uint8_t,
    24 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as libc::c_int as uint8_t,
    25 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_OMIT_LAST_7 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_OMIT_LAST_1 as libc::c_int as uint8_t,
    26 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as libc::c_int as uint8_t,
    27 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as libc::c_int as uint8_t,
    28 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as libc::c_int as uint8_t,
    12 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as libc::c_int as uint8_t,
    29 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_OMIT_FIRST_9 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_OMIT_FIRST_7 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_OMIT_LAST_6 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as libc::c_int as uint8_t,
    21 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_UPPERCASE_FIRST as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_OMIT_LAST_8 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as libc::c_int as uint8_t,
    31 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as libc::c_int as uint8_t,
    32 as libc::c_int as uint8_t,
    47 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_OMIT_LAST_5 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_OMIT_LAST_9 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_UPPERCASE_FIRST as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_UPPERCASE_FIRST as libc::c_int as uint8_t,
    8 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as libc::c_int as uint8_t,
    21 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_UPPERCASE_ALL as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_UPPERCASE_FIRST as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as libc::c_int as uint8_t,
    30 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    35 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    47 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_UPPERCASE_FIRST as libc::c_int as uint8_t,
    17 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as libc::c_int as uint8_t,
    36 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as libc::c_int as uint8_t,
    33 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_UPPERCASE_FIRST as libc::c_int as uint8_t,
    21 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_UPPERCASE_FIRST as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as libc::c_int as uint8_t,
    37 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as libc::c_int as uint8_t,
    30 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as libc::c_int as uint8_t,
    38 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_UPPERCASE_ALL as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as libc::c_int as uint8_t,
    39 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_UPPERCASE_ALL as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as libc::c_int as uint8_t,
    34 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_UPPERCASE_ALL as libc::c_int as uint8_t,
    8 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_UPPERCASE_FIRST as libc::c_int as uint8_t,
    12 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as libc::c_int as uint8_t,
    21 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as libc::c_int as uint8_t,
    40 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_UPPERCASE_FIRST as libc::c_int as uint8_t,
    12 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as libc::c_int as uint8_t,
    41 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as libc::c_int as uint8_t,
    42 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_UPPERCASE_ALL as libc::c_int as uint8_t,
    17 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as libc::c_int as uint8_t,
    43 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_UPPERCASE_FIRST as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_UPPERCASE_ALL as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as libc::c_int as uint8_t,
    34 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_UPPERCASE_FIRST as libc::c_int as uint8_t,
    33 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as libc::c_int as uint8_t,
    44 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_UPPERCASE_ALL as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    45 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as libc::c_int as uint8_t,
    33 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_UPPERCASE_FIRST as libc::c_int as uint8_t,
    30 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_UPPERCASE_ALL as libc::c_int as uint8_t,
    30 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as libc::c_int as uint8_t,
    46 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_UPPERCASE_ALL as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_UPPERCASE_FIRST as libc::c_int as uint8_t,
    34 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_UPPERCASE_FIRST as libc::c_int as uint8_t,
    33 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_UPPERCASE_ALL as libc::c_int as uint8_t,
    30 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_UPPERCASE_ALL as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_UPPERCASE_ALL as libc::c_int as uint8_t,
    33 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_UPPERCASE_ALL as libc::c_int as uint8_t,
    21 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_UPPERCASE_ALL as libc::c_int as uint8_t,
    12 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_UPPERCASE_ALL as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_UPPERCASE_ALL as libc::c_int as uint8_t,
    34 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_UPPERCASE_ALL as libc::c_int as uint8_t,
    12 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_UPPERCASE_FIRST as libc::c_int as uint8_t,
    30 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_UPPERCASE_ALL as libc::c_int as uint8_t,
    34 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    BROTLI_TRANSFORM_UPPERCASE_FIRST as libc::c_int as uint8_t,
    34 as libc::c_int as uint8_t,
];
static mut kBrotliTransforms: BrotliTransforms = BrotliTransforms {
    prefix_suffix_size: 0,
    prefix_suffix: 0 as *const uint8_t,
    prefix_suffix_map: 0 as *const uint16_t,
    num_transforms: 0,
    transforms: 0 as *const uint8_t,
    params: 0 as *const uint8_t,
    cutOffTransforms: [0; 10],
};
#[no_mangle]
pub unsafe extern "C" fn BrotliGetTransforms() -> *const BrotliTransforms {
    return &kBrotliTransforms;
}
unsafe extern "C" fn ToUpperCase(mut p: *mut uint8_t) -> libc::c_int {
    if (*p.offset(0 as libc::c_int as isize) as libc::c_int) < 0xc0 as libc::c_int {
        if *p.offset(0 as libc::c_int as isize) as libc::c_int >= 'a' as i32
            && *p.offset(0 as libc::c_int as isize) as libc::c_int <= 'z' as i32
        {
            *p.offset(0 as libc::c_int as isize) = (*p.offset(0 as libc::c_int as isize) as libc::c_int ^ 32 as libc::c_int) as uint8_t;
        }
        return 1 as libc::c_int;
    }
    if (*p.offset(0 as libc::c_int as isize) as libc::c_int) < 0xe0 as libc::c_int {
        *p.offset(1 as libc::c_int as isize) = (*p.offset(1 as libc::c_int as isize) as libc::c_int ^ 32 as libc::c_int) as uint8_t;
        return 2 as libc::c_int;
    }
    *p.offset(2 as libc::c_int as isize) = (*p.offset(2 as libc::c_int as isize) as libc::c_int ^ 5 as libc::c_int) as uint8_t;
    return 3 as libc::c_int;
}
unsafe extern "C" fn Shift(
    mut word: *mut uint8_t,
    mut word_len: libc::c_int,
    mut parameter: uint16_t,
) -> libc::c_int {
    let mut scalar = (parameter as libc::c_uint & 0x7fff as libc::c_uint)
        .wrapping_add(
            (0x1000000 as libc::c_uint)
                .wrapping_sub(parameter as libc::c_uint & 0x8000 as libc::c_uint),
        );
    if (*word.offset(0 as libc::c_int as isize) as libc::c_int) < 0x80 as libc::c_int {
        scalar= (scalar as libc::c_uint)
            .wrapping_add(*word.offset(0 as libc::c_int as isize) as uint32_t)
            as uint32_t as uint32_t;
        *word
            .offset(
                0 as libc::c_int as isize,
            ) = (scalar & 0x7f as libc::c_uint) as uint8_t;
        return 1 as libc::c_int;
    } else {
        if (*word.offset(0 as libc::c_int as isize) as libc::c_int) < 0xc0 as libc::c_int
        {
            return 1 as libc::c_int
        } else {
            if (*word.offset(0 as libc::c_int as isize) as libc::c_int)
                < 0xe0 as libc::c_int
            {
                if word_len < 2 as libc::c_int {
                    return 1 as libc::c_int;
                }
                scalar= (scalar as libc::c_uint)
                    .wrapping_add(
                        *word.offset(1 as libc::c_int as isize) as libc::c_uint
                            & 0x3f as libc::c_uint
                            | (*word.offset(0 as libc::c_int as isize) as libc::c_uint
                                & 0x1f as libc::c_uint) << 6 as libc::c_uint,
                    ) as uint32_t as uint32_t;
                *word
                    .offset(
                        0 as libc::c_int as isize,
                    ) = (0xc0 as libc::c_int as libc::c_uint
                    | scalar >> 6 as libc::c_uint & 0x1f as libc::c_int as libc::c_uint)
                    as uint8_t;
                *word
                    .offset(
                        1 as libc::c_int as isize,
                    ) = ((*word.offset(1 as libc::c_int as isize) as libc::c_int
                    & 0xc0 as libc::c_int) as libc::c_uint
                    | scalar & 0x3f as libc::c_int as libc::c_uint) as uint8_t;
                return 2 as libc::c_int;
            } else {
                if (*word.offset(0 as libc::c_int as isize) as libc::c_int)
                    < 0xf0 as libc::c_int
                {
                    if word_len < 3 as libc::c_int {
                        return word_len;
                    }
                    scalar= (scalar as libc::c_uint)
                        .wrapping_add(
                            *word.offset(2 as libc::c_int as isize) as libc::c_uint
                                & 0x3f as libc::c_uint
                                | (*word.offset(1 as libc::c_int as isize) as libc::c_uint
                                    & 0x3f as libc::c_uint) << 6 as libc::c_uint
                                | (*word.offset(0 as libc::c_int as isize) as libc::c_uint
                                    & 0xf as libc::c_uint) << 12 as libc::c_uint,
                        ) as uint32_t as uint32_t;
                    *word
                        .offset(
                            0 as libc::c_int as isize,
                        ) = (0xe0 as libc::c_int as libc::c_uint
                        | scalar >> 12 as libc::c_uint
                            & 0xf as libc::c_int as libc::c_uint) as uint8_t;
                    *word
                        .offset(
                            1 as libc::c_int as isize,
                        ) = ((*word.offset(1 as libc::c_int as isize) as libc::c_int
                        & 0xc0 as libc::c_int) as libc::c_uint
                        | scalar >> 6 as libc::c_uint
                            & 0x3f as libc::c_int as libc::c_uint) as uint8_t;
                    *word
                        .offset(
                            2 as libc::c_int as isize,
                        ) = ((*word.offset(2 as libc::c_int as isize) as libc::c_int
                        & 0xc0 as libc::c_int) as libc::c_uint
                        | scalar & 0x3f as libc::c_int as libc::c_uint) as uint8_t;
                    return 3 as libc::c_int;
                } else {
                    if (*word.offset(0 as libc::c_int as isize) as libc::c_int)
                        < 0xf8 as libc::c_int
                    {
                        if word_len < 4 as libc::c_int {
                            return word_len;
                        }
                        scalar= (scalar as libc::c_uint)
                            .wrapping_add(
                                *word.offset(3 as libc::c_int as isize) as libc::c_uint
                                    & 0x3f as libc::c_uint
                                    | (*word.offset(2 as libc::c_int as isize) as libc::c_uint
                                        & 0x3f as libc::c_uint) << 6 as libc::c_uint
                                    | (*word.offset(1 as libc::c_int as isize) as libc::c_uint
                                        & 0x3f as libc::c_uint) << 12 as libc::c_uint
                                    | (*word.offset(0 as libc::c_int as isize) as libc::c_uint
                                        & 0x7 as libc::c_uint) << 18 as libc::c_uint,
                            ) as uint32_t as uint32_t;
                        *word
                            .offset(
                                0 as libc::c_int as isize,
                            ) = (0xf0 as libc::c_int as libc::c_uint
                            | scalar >> 18 as libc::c_uint
                                & 0x7 as libc::c_int as libc::c_uint) as uint8_t;
                        *word
                            .offset(
                                1 as libc::c_int as isize,
                            ) = ((*word.offset(1 as libc::c_int as isize) as libc::c_int
                            & 0xc0 as libc::c_int) as libc::c_uint
                            | scalar >> 12 as libc::c_uint
                                & 0x3f as libc::c_int as libc::c_uint) as uint8_t;
                        *word
                            .offset(
                                2 as libc::c_int as isize,
                            ) = ((*word.offset(2 as libc::c_int as isize) as libc::c_int
                            & 0xc0 as libc::c_int) as libc::c_uint
                            | scalar >> 6 as libc::c_uint
                                & 0x3f as libc::c_int as libc::c_uint) as uint8_t;
                        *word
                            .offset(
                                3 as libc::c_int as isize,
                            ) = ((*word.offset(3 as libc::c_int as isize) as libc::c_int
                            & 0xc0 as libc::c_int) as libc::c_uint
                            | scalar & 0x3f as libc::c_int as libc::c_uint) as uint8_t;
                        return 4 as libc::c_int;
                    }
                }
            }
        }
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn BrotliTransformDictionaryWord(
    mut dst: *mut uint8_t,
    mut word: *const uint8_t,
    mut len: libc::c_int,
    mut transforms: *const BrotliTransforms,
    mut transform_idx: libc::c_int,
) -> libc::c_int {
    let mut idx = 0 as libc::c_int;
    let mut prefix: *const uint8_t = &*(*transforms).prefix_suffix
        .offset(
            *(*transforms).prefix_suffix_map
                .offset(
                    *(*transforms).transforms
                        .offset(
                            (transform_idx * 3 as libc::c_int + 0 as libc::c_int)
                                as isize,
                        ) as isize,
                ) as isize,
        ) as *const uint8_t;
    let mut type_0 = *(*transforms).transforms
        .offset((transform_idx * 3 as libc::c_int + 1 as libc::c_int) as isize);
    let mut suffix: *const uint8_t = &*(*transforms).prefix_suffix
        .offset(
            *(*transforms).prefix_suffix_map
                .offset(
                    *(*transforms).transforms
                        .offset(
                            (transform_idx * 3 as libc::c_int + 2 as libc::c_int)
                                as isize,
                        ) as isize,
                ) as isize,
        ) as *const uint8_t;
    let fresh3 = prefix;
    prefix= prefix.offset(1);
    let mut prefix_len = (*fresh3) as libc::c_int;
    loop {
        let fresh4 = prefix_len;
        prefix_len= prefix_len - 1;
        if !(fresh4 != 0) {
            break;
        }
        let fresh5 = prefix;
        prefix= prefix.offset(1);
        let fresh6 = idx;
        idx= idx + 1;
        *dst.offset(fresh6 as isize) = (*fresh5);
    }
    let t = type_0 as libc::c_int;
    let mut i = 0 as libc::c_int;
    if t <= BROTLI_TRANSFORM_OMIT_LAST_9 as libc::c_int {
        len-= t;
    } else if t >= BROTLI_TRANSFORM_OMIT_FIRST_1 as libc::c_int
        && t <= BROTLI_TRANSFORM_OMIT_FIRST_9 as libc::c_int
    {
        let mut skip = t
            - (BROTLI_TRANSFORM_OMIT_FIRST_1 as libc::c_int - 1 as libc::c_int);
        word= word.offset(skip as isize);
        len-= skip;
    }
    while i < len {
        let fresh7 = i;
        i= i + 1;
        let fresh8 = idx;
        idx= idx + 1;
        *dst.offset(fresh8 as isize) = *word.offset(fresh7 as isize);
    }
    if t == BROTLI_TRANSFORM_UPPERCASE_FIRST as libc::c_int {
        ToUpperCase(core::ptr::addr_of_mut!(*dst.offset((idx - len) as isize)));
    } else if t == BROTLI_TRANSFORM_UPPERCASE_ALL as libc::c_int {
        let mut uppercase: *mut uint8_t = core::ptr::addr_of_mut!(*dst.offset((idx - len) as isize))
            as *mut uint8_t;
        while len > 0 as libc::c_int {
            let mut step = ToUpperCase(uppercase);
            uppercase= uppercase.offset(step as isize);
            len-= step;
        }
    } else if t == BROTLI_TRANSFORM_SHIFT_FIRST as libc::c_int {
        let mut param = (*(*transforms).params
            .offset((transform_idx * 2 as libc::c_int) as isize) as libc::c_int
            + ((*(*transforms).params
                .offset((transform_idx * 2 as libc::c_int + 1 as libc::c_int) as isize)
                as libc::c_int) << 8 as libc::c_uint)) as uint16_t;
        Shift(core::ptr::addr_of_mut!(*dst.offset((idx - len) as isize)), len, param);
    } else if t == BROTLI_TRANSFORM_SHIFT_ALL as libc::c_int {
        let mut param_0 = (*(*transforms).params
            .offset((transform_idx * 2 as libc::c_int) as isize) as libc::c_int
            + ((*(*transforms).params
                .offset((transform_idx * 2 as libc::c_int + 1 as libc::c_int) as isize)
                as libc::c_int) << 8 as libc::c_uint)) as uint16_t;
        let mut shift: *mut uint8_t = core::ptr::addr_of_mut!(*dst.offset((idx - len) as isize))
            as *mut uint8_t;
        while len > 0 as libc::c_int {
            let mut step_0 = Shift(shift, len, param_0);
            shift= shift.offset(step_0 as isize);
            len-= step_0;
        }
    }
    let fresh9 = suffix;
    suffix= suffix.offset(1);
    let mut suffix_len = (*fresh9) as libc::c_int;
    loop {
        let fresh10 = suffix_len;
        suffix_len= suffix_len - 1;
        if !(fresh10 != 0) {
            break;
        }
        let fresh11 = suffix;
        suffix= suffix.offset(1);
        let fresh12 = idx;
        idx= idx + 1;
        *dst.offset(fresh12 as isize) = (*fresh11);
    }
    return idx;
}
unsafe extern "C" fn run_static_initializers() {
    crate::src::common::transform::kBrotliTransforms= {
        let mut init = BrotliTransforms {
            prefix_suffix_size: ::std::mem::size_of::<[libc::c_char; 217]>()
                as libc::c_ulong as uint16_t,
            prefix_suffix: kPrefixSuffix.as_ptr() as *const uint8_t,
            prefix_suffix_map: kPrefixSuffixMap.as_ptr(),
            num_transforms: (::std::mem::size_of::<[uint8_t; 363]>() as libc::c_ulong)
                .wrapping_div(
                    (3 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<uint8_t>() as libc::c_ulong),
                ) as uint32_t,
            transforms: kTransformsData.as_ptr(),
            params: 0 as *const uint8_t,
            cutOffTransforms: [
                0 as libc::c_int as int16_t,
                12 as libc::c_int as int16_t,
                27 as libc::c_int as int16_t,
                23 as libc::c_int as int16_t,
                42 as libc::c_int as int16_t,
                63 as libc::c_int as int16_t,
                56 as libc::c_int as int16_t,
                48 as libc::c_int as int16_t,
                59 as libc::c_int as int16_t,
                64 as libc::c_int as int16_t,
            ],
        };
        init
    };
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
