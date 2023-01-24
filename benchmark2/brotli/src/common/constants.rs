use ::libc;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BrotliPrefixCodeRange {
    pub offset: uint16_t,
    pub nbits: uint8_t,
}
#[no_mangle]
pub static mut _kBrotliPrefixCodeRanges: [BrotliPrefixCodeRange; 26] = [
    {
        let mut init = BrotliPrefixCodeRange {
            offset: 1 as libc::c_int as uint16_t,
            nbits: 2 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = BrotliPrefixCodeRange {
            offset: 5 as libc::c_int as uint16_t,
            nbits: 2 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = BrotliPrefixCodeRange {
            offset: 9 as libc::c_int as uint16_t,
            nbits: 2 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = BrotliPrefixCodeRange {
            offset: 13 as libc::c_int as uint16_t,
            nbits: 2 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = BrotliPrefixCodeRange {
            offset: 17 as libc::c_int as uint16_t,
            nbits: 3 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = BrotliPrefixCodeRange {
            offset: 25 as libc::c_int as uint16_t,
            nbits: 3 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = BrotliPrefixCodeRange {
            offset: 33 as libc::c_int as uint16_t,
            nbits: 3 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = BrotliPrefixCodeRange {
            offset: 41 as libc::c_int as uint16_t,
            nbits: 3 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = BrotliPrefixCodeRange {
            offset: 49 as libc::c_int as uint16_t,
            nbits: 4 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = BrotliPrefixCodeRange {
            offset: 65 as libc::c_int as uint16_t,
            nbits: 4 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = BrotliPrefixCodeRange {
            offset: 81 as libc::c_int as uint16_t,
            nbits: 4 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = BrotliPrefixCodeRange {
            offset: 97 as libc::c_int as uint16_t,
            nbits: 4 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = BrotliPrefixCodeRange {
            offset: 113 as libc::c_int as uint16_t,
            nbits: 5 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = BrotliPrefixCodeRange {
            offset: 145 as libc::c_int as uint16_t,
            nbits: 5 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = BrotliPrefixCodeRange {
            offset: 177 as libc::c_int as uint16_t,
            nbits: 5 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = BrotliPrefixCodeRange {
            offset: 209 as libc::c_int as uint16_t,
            nbits: 5 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = BrotliPrefixCodeRange {
            offset: 241 as libc::c_int as uint16_t,
            nbits: 6 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = BrotliPrefixCodeRange {
            offset: 305 as libc::c_int as uint16_t,
            nbits: 6 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = BrotliPrefixCodeRange {
            offset: 369 as libc::c_int as uint16_t,
            nbits: 7 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = BrotliPrefixCodeRange {
            offset: 497 as libc::c_int as uint16_t,
            nbits: 8 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = BrotliPrefixCodeRange {
            offset: 753 as libc::c_int as uint16_t,
            nbits: 9 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = BrotliPrefixCodeRange {
            offset: 1265 as libc::c_int as uint16_t,
            nbits: 10 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = BrotliPrefixCodeRange {
            offset: 2289 as libc::c_int as uint16_t,
            nbits: 11 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = BrotliPrefixCodeRange {
            offset: 4337 as libc::c_int as uint16_t,
            nbits: 12 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = BrotliPrefixCodeRange {
            offset: 8433 as libc::c_int as uint16_t,
            nbits: 13 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = BrotliPrefixCodeRange {
            offset: 16625 as libc::c_int as uint16_t,
            nbits: 24 as libc::c_int as uint8_t,
        };
        init
    },
];
