use ::libc;
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type uint8_t = __uint8_t;
unsafe extern "C" fn BrotliParseAsUTF8(
    mut symbol: Option<&mut libc::c_int>,
    mut input: *const uint8_t,
    mut size: size_t,
) -> size_t {
    if *input.offset(0 as libc::c_int as isize) as libc::c_int & 0x80 as libc::c_int
        == 0 as libc::c_int
    {
        *symbol.as_deref_mut().unwrap()= *input.offset(0 as libc::c_int as isize) as libc::c_int;
        if (*symbol.as_deref().unwrap()) > 0 as libc::c_int {
            return 1 as libc::c_int as size_t;
        }
    }
    if size > 1 as libc::c_uint as libc::c_ulong
        && *input.offset(0 as libc::c_int as isize) as libc::c_int & 0xe0 as libc::c_int
            == 0xc0 as libc::c_int
        && *input.offset(1 as libc::c_int as isize) as libc::c_int & 0xc0 as libc::c_int
            == 0x80 as libc::c_int
    {
        *symbol.as_deref_mut().unwrap()= (*input.offset(0 as libc::c_int as isize) as libc::c_int
            & 0x1f as libc::c_int) << 6 as libc::c_int
            | *input.offset(1 as libc::c_int as isize) as libc::c_int
                & 0x3f as libc::c_int;
        if (*symbol.as_deref().unwrap()) > 0x7f as libc::c_int {
            return 2 as libc::c_int as size_t;
        }
    }
    if size > 2 as libc::c_uint as libc::c_ulong
        && *input.offset(0 as libc::c_int as isize) as libc::c_int & 0xf0 as libc::c_int
            == 0xe0 as libc::c_int
        && *input.offset(1 as libc::c_int as isize) as libc::c_int & 0xc0 as libc::c_int
            == 0x80 as libc::c_int
        && *input.offset(2 as libc::c_int as isize) as libc::c_int & 0xc0 as libc::c_int
            == 0x80 as libc::c_int
    {
        *symbol.as_deref_mut().unwrap()= (*input.offset(0 as libc::c_int as isize) as libc::c_int
            & 0xf as libc::c_int) << 12 as libc::c_int
            | (*input.offset(1 as libc::c_int as isize) as libc::c_int
                & 0x3f as libc::c_int) << 6 as libc::c_int
            | *input.offset(2 as libc::c_int as isize) as libc::c_int
                & 0x3f as libc::c_int;
        if (*symbol.as_deref().unwrap()) > 0x7ff as libc::c_int {
            return 3 as libc::c_int as size_t;
        }
    }
    if size > 3 as libc::c_uint as libc::c_ulong
        && *input.offset(0 as libc::c_int as isize) as libc::c_int & 0xf8 as libc::c_int
            == 0xf0 as libc::c_int
        && *input.offset(1 as libc::c_int as isize) as libc::c_int & 0xc0 as libc::c_int
            == 0x80 as libc::c_int
        && *input.offset(2 as libc::c_int as isize) as libc::c_int & 0xc0 as libc::c_int
            == 0x80 as libc::c_int
        && *input.offset(3 as libc::c_int as isize) as libc::c_int & 0xc0 as libc::c_int
            == 0x80 as libc::c_int
    {
        *symbol.as_deref_mut().unwrap()= (*input.offset(0 as libc::c_int as isize) as libc::c_int
            & 0x7 as libc::c_int) << 18 as libc::c_int
            | (*input.offset(1 as libc::c_int as isize) as libc::c_int
                & 0x3f as libc::c_int) << 12 as libc::c_int
            | (*input.offset(2 as libc::c_int as isize) as libc::c_int
                & 0x3f as libc::c_int) << 6 as libc::c_int
            | *input.offset(3 as libc::c_int as isize) as libc::c_int
                & 0x3f as libc::c_int;
        if (*symbol.as_deref().unwrap()) > 0xffff as libc::c_int && (*symbol.as_deref().unwrap()) <= 0x10ffff as libc::c_int {
            return 4 as libc::c_int as size_t;
        }
    }
    *symbol.as_deref_mut().unwrap()= 0x110000 as libc::c_int
        | *input.offset(0 as libc::c_int as isize) as libc::c_int;
    return 1 as libc::c_int as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn BrotliIsMostlyUTF8(
    mut data: *const uint8_t,
    mut pos: size_t,
    mut mask: size_t,
    mut length: size_t,
    mut min_fraction: libc::c_double,
) -> libc::c_int {
    let mut size_utf8 = 0 as libc::c_int as size_t;
    let mut i = 0 as libc::c_int as size_t;
    while i < length {
        let mut symbol: libc::c_int = 0;
        let mut bytes_read = BrotliParseAsUTF8(
            Some(&mut symbol),
            &*data.offset((pos.wrapping_add(i) & mask) as isize),
            length.wrapping_sub(i),
        );
        i= (i as libc::c_ulong).wrapping_add(bytes_read) as size_t as size_t;
        if symbol < 0x110000 as libc::c_int {
            size_utf8= (size_utf8 as libc::c_ulong).wrapping_add(bytes_read) as size_t
                as size_t;
        }
    }
    return if size_utf8 as libc::c_double > min_fraction * length as libc::c_double {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
}
