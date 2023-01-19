use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct csv_parser {
    pub pstate: libc::c_int,
    pub quoted: libc::c_int,
    pub spaces: size_t,
    pub entry_buf: *mut libc::c_uchar,
    pub entry_pos: size_t,
    pub entry_size: size_t,
    pub status: libc::c_int,
    pub options: libc::c_uchar,
    pub quote_char: libc::c_uchar,
    pub delim_char: libc::c_uchar,
    pub is_space: Option::<unsafe extern "C" fn(libc::c_uchar) -> libc::c_int>,
    pub is_term: Option::<unsafe extern "C" fn(libc::c_uchar) -> libc::c_int>,
    pub blk_size: size_t,
    pub malloc_func: Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
    pub realloc_func: Option::<
        unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void,
    >,
    pub free_func: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
}
static mut csv_errors: [*const libc::c_char; 5] = [
    b"success\0" as *const u8 as *const libc::c_char,
    b"error parsing data while strict checking enabled\0" as *const u8
        as *const libc::c_char,
    b"memory exhausted while increasing buffer size\0" as *const u8
        as *const libc::c_char,
    b"data size too large\0" as *const u8 as *const libc::c_char,
    b"invalid status code\0" as *const u8 as *const libc::c_char,
];
#[no_mangle]
pub unsafe extern "C" fn csv_error(mut p: *const csv_parser) -> libc::c_int {
    if !p.is_null()
        && !(b"received null csv_parser\0" as *const u8 as *const libc::c_char).is_null()
    {} else {
        __assert_fail(
            b"p && \"received null csv_parser\"\0" as *const u8 as *const libc::c_char,
            b"libcsv.c\0" as *const u8 as *const libc::c_char,
            82 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 41],
                &[libc::c_char; 41],
            >(b"int csv_error(const struct csv_parser *)\0"))
                .as_ptr(),
        );
    }
    return (*p).status;
}
#[no_mangle]
pub unsafe extern "C" fn csv_strerror(mut status: libc::c_int) -> *const libc::c_char {
    if status >= 4 as libc::c_int || status < 0 as libc::c_int {
        return csv_errors[4 as libc::c_int as usize]
    } else {
        return csv_errors[status as usize]
    };
}
#[no_mangle]
pub unsafe extern "C" fn csv_get_opts(mut p: *const csv_parser) -> libc::c_int {
    if p.is_null() {
        return -(1 as libc::c_int);
    }
    return (*p).options as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn csv_set_opts(
    mut p: *mut csv_parser,
    mut options: libc::c_uchar,
) -> libc::c_int {
    if p.is_null() {
        return -(1 as libc::c_int);
    }
    (*p).options = options;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn csv_init(
    mut p: *mut csv_parser,
    mut options: libc::c_uchar,
) -> libc::c_int {
    if p.is_null() {
        return -(1 as libc::c_int);
    }
    let ref mut fresh0 = (*p).entry_buf;
    *fresh0 = 0 as *mut libc::c_uchar;
    (*p).pstate = 0 as libc::c_int;
    (*p).quoted = 0 as libc::c_int;
    (*p).spaces = 0 as libc::c_int as size_t;
    (*p).entry_pos = 0 as libc::c_int as size_t;
    (*p).entry_size = 0 as libc::c_int as size_t;
    (*p).status = 0 as libc::c_int;
    (*p).options = options;
    (*p).quote_char = 0x22 as libc::c_int as libc::c_uchar;
    (*p).delim_char = 0x2c as libc::c_int as libc::c_uchar;
    let ref mut fresh1 = (*p).is_space;
    *fresh1 = None;
    let ref mut fresh2 = (*p).is_term;
    *fresh2 = None;
    (*p).blk_size = 128 as libc::c_int as size_t;
    let ref mut fresh3 = (*p).malloc_func;
    *fresh3 = None;
    let ref mut fresh4 = (*p).realloc_func;
    *fresh4 = Some(
        realloc
            as unsafe extern "C" fn(
                *mut libc::c_void,
                libc::c_ulong,
            ) -> *mut libc::c_void,
    );
    let ref mut fresh5 = (*p).free_func;
    *fresh5 = Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ());
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn csv_free(mut p: *mut csv_parser) {
    if p.is_null() {
        return;
    }
    if !((*p).entry_buf).is_null() && ((*p).free_func).is_some() {
        ((*p).free_func)
            .expect("non-null function pointer")((*p).entry_buf as *mut libc::c_void);
    }
    let ref mut fresh6 = (*p).entry_buf;
    *fresh6 = 0 as *mut libc::c_uchar;
    (*p).entry_size = 0 as libc::c_int as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn csv_fini(
    mut p: *mut csv_parser,
    mut cb1: Option::<
        unsafe extern "C" fn(*mut libc::c_void, size_t, *mut libc::c_void) -> (),
    >,
    mut cb2: Option::<unsafe extern "C" fn(libc::c_int, *mut libc::c_void) -> ()>,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    if p.is_null() {
        return -(1 as libc::c_int);
    }
    let mut quoted = (*p).quoted;
    let mut pstate = (*p).pstate;
    let mut spaces = (*p).spaces;
    let mut entry_pos = (*p).entry_pos;
    if pstate == 2 as libc::c_int && (*p).quoted != 0
        && (*p).options as libc::c_int & 1 as libc::c_int != 0
        && (*p).options as libc::c_int & 4 as libc::c_int != 0
    {
        (*p).status = 1 as libc::c_int;
        return -(1 as libc::c_int);
    }
    let mut current_block_26: u64;
    match pstate {
        3 => {
            let ref mut fresh7 = (*p).entry_pos;
            *fresh7 = (*fresh7 as libc::c_ulong)
                .wrapping_sub(
                    ((*p).spaces).wrapping_add(1 as libc::c_int as libc::c_ulong),
                ) as size_t as size_t;
            entry_pos = (*p).entry_pos;
            current_block_26 = 5901124769980541461;
        }
        1 | 2 => {
            current_block_26 = 5901124769980541461;
        }
        0 | _ => {
            current_block_26 = 4761528863920922185;
        }
    }
    match current_block_26 {
        5901124769980541461 => {
            if quoted == 0 {
                entry_pos = (entry_pos as libc::c_ulong).wrapping_sub(spaces) as size_t
                    as size_t;
            }
            if (*p).options as libc::c_int & 8 as libc::c_int != 0 {
                *((*p).entry_buf)
                    .offset(entry_pos as isize) = '\0' as i32 as libc::c_uchar;
            }
            if cb1.is_some() && (*p).options as libc::c_int & 16 as libc::c_int != 0
                && quoted == 0 && entry_pos == 0 as libc::c_int as libc::c_ulong
            {
                cb1
                    .expect(
                        "non-null function pointer",
                    )(0 as *mut libc::c_void, entry_pos, data);
            } else if cb1.is_some() {
                cb1
                    .expect(
                        "non-null function pointer",
                    )((*p).entry_buf as *mut libc::c_void, entry_pos, data);
            }
            pstate = 1 as libc::c_int;
            spaces = 0 as libc::c_int as size_t;
            quoted = spaces as libc::c_int;
            entry_pos = quoted as size_t;
            if cb2.is_some() {
                cb2.expect("non-null function pointer")(-(1 as libc::c_int), data);
            }
            pstate = 0 as libc::c_int;
            spaces = 0 as libc::c_int as size_t;
            quoted = spaces as libc::c_int;
            entry_pos = quoted as size_t;
        }
        _ => {}
    }
    let ref mut fresh8 = (*p).status;
    *fresh8 = 0 as libc::c_int;
    let ref mut fresh9 = (*p).entry_pos;
    *fresh9 = *fresh8 as size_t;
    let ref mut fresh10 = (*p).quoted;
    *fresh10 = *fresh9 as libc::c_int;
    (*p).spaces = *fresh10 as size_t;
    (*p).pstate = 0 as libc::c_int;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn csv_set_delim(mut p: *mut csv_parser, mut c: libc::c_uchar) {
    if !p.is_null() {
        (*p).delim_char = c;
    }
}
#[no_mangle]
pub unsafe extern "C" fn csv_set_quote(mut p: *mut csv_parser, mut c: libc::c_uchar) {
    if !p.is_null() {
        (*p).quote_char = c;
    }
}
#[no_mangle]
pub unsafe extern "C" fn csv_get_delim(mut p: *const csv_parser) -> libc::c_uchar {
    if !p.is_null()
        && !(b"received null csv_parser\0" as *const u8 as *const libc::c_char).is_null()
    {} else {
        __assert_fail(
            b"p && \"received null csv_parser\"\0" as *const u8 as *const libc::c_char,
            b"libcsv.c\0" as *const u8 as *const libc::c_char,
            222 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 55],
                &[libc::c_char; 55],
            >(b"unsigned char csv_get_delim(const struct csv_parser *)\0"))
                .as_ptr(),
        );
    }
    return (*p).delim_char;
}
#[no_mangle]
pub unsafe extern "C" fn csv_get_quote(mut p: *const csv_parser) -> libc::c_uchar {
    if !p.is_null()
        && !(b"received null csv_parser\0" as *const u8 as *const libc::c_char).is_null()
    {} else {
        __assert_fail(
            b"p && \"received null csv_parser\"\0" as *const u8 as *const libc::c_char,
            b"libcsv.c\0" as *const u8 as *const libc::c_char,
            231 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 55],
                &[libc::c_char; 55],
            >(b"unsigned char csv_get_quote(const struct csv_parser *)\0"))
                .as_ptr(),
        );
    }
    return (*p).quote_char;
}
#[no_mangle]
pub unsafe extern "C" fn csv_set_space_func(
    mut p: *mut csv_parser,
    mut f: Option::<unsafe extern "C" fn(libc::c_uchar) -> libc::c_int>,
) {
    if !p.is_null() {
        let ref mut fresh11 = (*p).is_space;
        *fresh11 = f;
    }
}
#[no_mangle]
pub unsafe extern "C" fn csv_set_term_func(
    mut p: *mut csv_parser,
    mut f: Option::<unsafe extern "C" fn(libc::c_uchar) -> libc::c_int>,
) {
    if !p.is_null() {
        let ref mut fresh12 = (*p).is_term;
        *fresh12 = f;
    }
}
#[no_mangle]
pub unsafe extern "C" fn csv_set_realloc_func(
    mut p: *mut csv_parser,
    mut f: Option::<unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void>,
) {
    if !p.is_null() && f.is_some() {
        let ref mut fresh13 = (*p).realloc_func;
        *fresh13 = f;
    }
}
#[no_mangle]
pub unsafe extern "C" fn csv_set_free_func(
    mut p: *mut csv_parser,
    mut f: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
) {
    if !p.is_null() && f.is_some() {
        let ref mut fresh14 = (*p).free_func;
        *fresh14 = f;
    }
}
#[no_mangle]
pub unsafe extern "C" fn csv_set_blk_size(mut p: *mut csv_parser, mut size: size_t) {
    if !p.is_null() {
        (*p).blk_size = size;
    }
}
#[no_mangle]
pub unsafe extern "C" fn csv_get_buffer_size(mut p: *const csv_parser) -> size_t {
    if !p.is_null() {
        return (*p).entry_size;
    }
    return 0 as libc::c_int as size_t;
}
unsafe extern "C" fn csv_increase_buffer(mut p: *mut csv_parser) -> libc::c_int {
    if p.is_null() {
        return 0 as libc::c_int;
    }
    if ((*p).realloc_func).is_none() {
        return 0 as libc::c_int;
    }
    let mut to_add = (*p).blk_size;
    let mut vp = 0 as *mut libc::c_void;
    if (*p).entry_size >= (18446744073709551615 as libc::c_ulong).wrapping_sub(to_add) {
        to_add = (18446744073709551615 as libc::c_ulong).wrapping_sub((*p).entry_size);
    }
    if to_add == 0 {
        (*p).status = 3 as libc::c_int;
        return -(1 as libc::c_int);
    }
    loop {
        vp = ((*p).realloc_func)
            .expect(
                "non-null function pointer",
            )(
            (*p).entry_buf as *mut libc::c_void,
            ((*p).entry_size).wrapping_add(to_add),
        );
        if !vp.is_null() {
            break;
        }
        to_add = (to_add as libc::c_ulong)
            .wrapping_div(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
        if to_add == 0 {
            (*p).status = 2 as libc::c_int;
            return -(1 as libc::c_int);
        }
    }
    let ref mut fresh15 = (*p).entry_buf;
    *fresh15 = vp as *mut libc::c_uchar;
    let ref mut fresh16 = (*p).entry_size;
    *fresh16 = (*fresh16 as libc::c_ulong).wrapping_add(to_add) as size_t as size_t;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn csv_parse(
    mut p: *mut csv_parser,
    mut s: *const libc::c_void,
    mut len: size_t,
    mut cb1: Option::<
        unsafe extern "C" fn(*mut libc::c_void, size_t, *mut libc::c_void) -> (),
    >,
    mut cb2: Option::<unsafe extern "C" fn(libc::c_int, *mut libc::c_void) -> ()>,
    mut data: *mut libc::c_void,
) -> size_t {
    if !p.is_null()
        && !(b"received null csv_parser\0" as *const u8 as *const libc::c_char).is_null()
    {} else {
        __assert_fail(
            b"p && \"received null csv_parser\"\0" as *const u8 as *const libc::c_char,
            b"libcsv.c\0" as *const u8 as *const libc::c_char,
            321 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 125],
                &[libc::c_char; 125],
            >(
                b"size_t csv_parse(struct csv_parser *, const void *, size_t, void (*)(void *, size_t, void *), void (*)(int, void *), void *)\0",
            ))
                .as_ptr(),
        );
    }
    if s.is_null() {
        return 0 as libc::c_int as size_t;
    }
    let mut us = s as *const libc::c_uchar;
    let mut c: libc::c_uchar = 0;
    let mut pos = 0 as libc::c_int as size_t;
    let mut delim = (*p).delim_char;
    let mut quote = (*p).quote_char;
    let mut is_space: Option::<unsafe extern "C" fn(libc::c_uchar) -> libc::c_int> = (*p)
        .is_space;
    let mut is_term: Option::<unsafe extern "C" fn(libc::c_uchar) -> libc::c_int> = (*p)
        .is_term;
    let mut quoted = (*p).quoted;
    let mut pstate = (*p).pstate;
    let mut spaces = (*p).spaces;
    let mut entry_pos = (*p).entry_pos;
    if ((*p).entry_buf).is_null() && pos < len {
        if csv_increase_buffer(p) != 0 as libc::c_int {
            (*p).quoted = quoted;
            (*p).pstate = pstate;
            (*p).spaces = spaces;
            (*p).entry_pos = entry_pos;
            return pos;
        }
    }
    while pos < len {
        if entry_pos
            == (if (*p).options as libc::c_int & 8 as libc::c_int != 0 {
                ((*p).entry_size).wrapping_sub(1 as libc::c_int as libc::c_ulong)
            } else {
                (*p).entry_size
            })
        {
            if csv_increase_buffer(p) != 0 as libc::c_int {
                (*p).quoted = quoted;
                (*p).pstate = pstate;
                (*p).spaces = spaces;
                (*p).entry_pos = entry_pos;
                return pos;
            }
        }
        let fresh17 = pos;
        pos = pos.wrapping_add(1);
        c = *us.offset(fresh17 as isize);
        match pstate {
            0 | 1 => {
                if (if is_space.is_some() {
                    is_space.expect("non-null function pointer")(c)
                } else {
                    (c as libc::c_int == 0x20 as libc::c_int
                        || c as libc::c_int == 0x9 as libc::c_int) as libc::c_int
                }) != 0 && c as libc::c_int != delim as libc::c_int
                {
                    continue;
                }
                if if is_term.is_some() {
                    is_term.expect("non-null function pointer")(c)
                } else {
                    (c as libc::c_int == 0xd as libc::c_int
                        || c as libc::c_int == 0xa as libc::c_int) as libc::c_int
                } != 0
                {
                    if pstate == 1 as libc::c_int {
                        if quoted == 0 {
                            entry_pos = (entry_pos as libc::c_ulong).wrapping_sub(spaces)
                                as size_t as size_t;
                        }
                        if (*p).options as libc::c_int & 8 as libc::c_int != 0 {
                            *((*p).entry_buf)
                                .offset(entry_pos as isize) = '\0' as i32 as libc::c_uchar;
                        }
                        if cb1.is_some()
                            && (*p).options as libc::c_int & 16 as libc::c_int != 0
                            && quoted == 0
                            && entry_pos == 0 as libc::c_int as libc::c_ulong
                        {
                            cb1
                                .expect(
                                    "non-null function pointer",
                                )(0 as *mut libc::c_void, entry_pos, data);
                        } else if cb1.is_some() {
                            cb1
                                .expect(
                                    "non-null function pointer",
                                )((*p).entry_buf as *mut libc::c_void, entry_pos, data);
                        }
                        pstate = 1 as libc::c_int;
                        spaces = 0 as libc::c_int as size_t;
                        quoted = spaces as libc::c_int;
                        entry_pos = quoted as size_t;
                        if cb2.is_some() {
                            cb2
                                .expect(
                                    "non-null function pointer",
                                )(c as libc::c_int, data);
                        }
                        pstate = 0 as libc::c_int;
                        spaces = 0 as libc::c_int as size_t;
                        quoted = spaces as libc::c_int;
                        entry_pos = quoted as size_t;
                    } else if (*p).options as libc::c_int & 2 as libc::c_int != 0 {
                        if cb2.is_some() {
                            cb2
                                .expect(
                                    "non-null function pointer",
                                )(c as libc::c_int, data);
                        }
                        pstate = 0 as libc::c_int;
                        spaces = 0 as libc::c_int as size_t;
                        quoted = spaces as libc::c_int;
                        entry_pos = quoted as size_t;
                    }
                } else if c as libc::c_int == delim as libc::c_int {
                    if quoted == 0 {
                        entry_pos = (entry_pos as libc::c_ulong).wrapping_sub(spaces)
                            as size_t as size_t;
                    }
                    if (*p).options as libc::c_int & 8 as libc::c_int != 0 {
                        *((*p).entry_buf)
                            .offset(entry_pos as isize) = '\0' as i32 as libc::c_uchar;
                    }
                    if cb1.is_some()
                        && (*p).options as libc::c_int & 16 as libc::c_int != 0
                        && quoted == 0 && entry_pos == 0 as libc::c_int as libc::c_ulong
                    {
                        cb1
                            .expect(
                                "non-null function pointer",
                            )(0 as *mut libc::c_void, entry_pos, data);
                    } else if cb1.is_some() {
                        cb1
                            .expect(
                                "non-null function pointer",
                            )((*p).entry_buf as *mut libc::c_void, entry_pos, data);
                    }
                    pstate = 1 as libc::c_int;
                    spaces = 0 as libc::c_int as size_t;
                    quoted = spaces as libc::c_int;
                    entry_pos = quoted as size_t;
                } else if c as libc::c_int == quote as libc::c_int {
                    pstate = 2 as libc::c_int;
                    quoted = 1 as libc::c_int;
                } else {
                    pstate = 2 as libc::c_int;
                    quoted = 0 as libc::c_int;
                    let fresh18 = entry_pos;
                    entry_pos = entry_pos.wrapping_add(1);
                    *((*p).entry_buf).offset(fresh18 as isize) = c;
                }
            }
            2 => {
                if c as libc::c_int == quote as libc::c_int {
                    if quoted != 0 {
                        let fresh19 = entry_pos;
                        entry_pos = entry_pos.wrapping_add(1);
                        *((*p).entry_buf).offset(fresh19 as isize) = c;
                        pstate = 3 as libc::c_int;
                    } else {
                        if (*p).options as libc::c_int & 1 as libc::c_int != 0 {
                            (*p).status = 1 as libc::c_int;
                            (*p).quoted = quoted;
                            (*p).pstate = pstate;
                            (*p).spaces = spaces;
                            (*p).entry_pos = entry_pos;
                            return pos.wrapping_sub(1 as libc::c_int as libc::c_ulong);
                        }
                        let fresh20 = entry_pos;
                        entry_pos = entry_pos.wrapping_add(1);
                        *((*p).entry_buf).offset(fresh20 as isize) = c;
                        spaces = 0 as libc::c_int as size_t;
                    }
                } else if c as libc::c_int == delim as libc::c_int {
                    if quoted != 0 {
                        let fresh21 = entry_pos;
                        entry_pos = entry_pos.wrapping_add(1);
                        *((*p).entry_buf).offset(fresh21 as isize) = c;
                    } else {
                        if quoted == 0 {
                            entry_pos = (entry_pos as libc::c_ulong).wrapping_sub(spaces)
                                as size_t as size_t;
                        }
                        if (*p).options as libc::c_int & 8 as libc::c_int != 0 {
                            *((*p).entry_buf)
                                .offset(entry_pos as isize) = '\0' as i32 as libc::c_uchar;
                        }
                        if cb1.is_some()
                            && (*p).options as libc::c_int & 16 as libc::c_int != 0
                            && quoted == 0
                            && entry_pos == 0 as libc::c_int as libc::c_ulong
                        {
                            cb1
                                .expect(
                                    "non-null function pointer",
                                )(0 as *mut libc::c_void, entry_pos, data);
                        } else if cb1.is_some() {
                            cb1
                                .expect(
                                    "non-null function pointer",
                                )((*p).entry_buf as *mut libc::c_void, entry_pos, data);
                        }
                        pstate = 1 as libc::c_int;
                        spaces = 0 as libc::c_int as size_t;
                        quoted = spaces as libc::c_int;
                        entry_pos = quoted as size_t;
                    }
                } else if if is_term.is_some() {
                    is_term.expect("non-null function pointer")(c)
                } else {
                    (c as libc::c_int == 0xd as libc::c_int
                        || c as libc::c_int == 0xa as libc::c_int) as libc::c_int
                } != 0
                {
                    if quoted == 0 {
                        if quoted == 0 {
                            entry_pos = (entry_pos as libc::c_ulong).wrapping_sub(spaces)
                                as size_t as size_t;
                        }
                        if (*p).options as libc::c_int & 8 as libc::c_int != 0 {
                            *((*p).entry_buf)
                                .offset(entry_pos as isize) = '\0' as i32 as libc::c_uchar;
                        }
                        if cb1.is_some()
                            && (*p).options as libc::c_int & 16 as libc::c_int != 0
                            && quoted == 0
                            && entry_pos == 0 as libc::c_int as libc::c_ulong
                        {
                            cb1
                                .expect(
                                    "non-null function pointer",
                                )(0 as *mut libc::c_void, entry_pos, data);
                        } else if cb1.is_some() {
                            cb1
                                .expect(
                                    "non-null function pointer",
                                )((*p).entry_buf as *mut libc::c_void, entry_pos, data);
                        }
                        pstate = 1 as libc::c_int;
                        spaces = 0 as libc::c_int as size_t;
                        quoted = spaces as libc::c_int;
                        entry_pos = quoted as size_t;
                        if cb2.is_some() {
                            cb2
                                .expect(
                                    "non-null function pointer",
                                )(c as libc::c_int, data);
                        }
                        pstate = 0 as libc::c_int;
                        spaces = 0 as libc::c_int as size_t;
                        quoted = spaces as libc::c_int;
                        entry_pos = quoted as size_t;
                    } else {
                        let fresh22 = entry_pos;
                        entry_pos = entry_pos.wrapping_add(1);
                        *((*p).entry_buf).offset(fresh22 as isize) = c;
                    }
                } else if quoted == 0
                    && (if is_space.is_some() {
                        is_space.expect("non-null function pointer")(c)
                    } else {
                        (c as libc::c_int == 0x20 as libc::c_int
                            || c as libc::c_int == 0x9 as libc::c_int) as libc::c_int
                    }) != 0
                {
                    let fresh23 = entry_pos;
                    entry_pos = entry_pos.wrapping_add(1);
                    *((*p).entry_buf).offset(fresh23 as isize) = c;
                    spaces = spaces.wrapping_add(1);
                } else {
                    let fresh24 = entry_pos;
                    entry_pos = entry_pos.wrapping_add(1);
                    *((*p).entry_buf).offset(fresh24 as isize) = c;
                    spaces = 0 as libc::c_int as size_t;
                }
            }
            3 => {
                if c as libc::c_int == delim as libc::c_int {
                    entry_pos = (entry_pos as libc::c_ulong)
                        .wrapping_sub(
                            spaces.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as size_t as size_t;
                    if quoted == 0 {
                        entry_pos = (entry_pos as libc::c_ulong).wrapping_sub(spaces)
                            as size_t as size_t;
                    }
                    if (*p).options as libc::c_int & 8 as libc::c_int != 0 {
                        *((*p).entry_buf)
                            .offset(entry_pos as isize) = '\0' as i32 as libc::c_uchar;
                    }
                    if cb1.is_some()
                        && (*p).options as libc::c_int & 16 as libc::c_int != 0
                        && quoted == 0 && entry_pos == 0 as libc::c_int as libc::c_ulong
                    {
                        cb1
                            .expect(
                                "non-null function pointer",
                            )(0 as *mut libc::c_void, entry_pos, data);
                    } else if cb1.is_some() {
                        cb1
                            .expect(
                                "non-null function pointer",
                            )((*p).entry_buf as *mut libc::c_void, entry_pos, data);
                    }
                    pstate = 1 as libc::c_int;
                    spaces = 0 as libc::c_int as size_t;
                    quoted = spaces as libc::c_int;
                    entry_pos = quoted as size_t;
                } else if if is_term.is_some() {
                    is_term.expect("non-null function pointer")(c)
                } else {
                    (c as libc::c_int == 0xd as libc::c_int
                        || c as libc::c_int == 0xa as libc::c_int) as libc::c_int
                } != 0
                {
                    entry_pos = (entry_pos as libc::c_ulong)
                        .wrapping_sub(
                            spaces.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as size_t as size_t;
                    if quoted == 0 {
                        entry_pos = (entry_pos as libc::c_ulong).wrapping_sub(spaces)
                            as size_t as size_t;
                    }
                    if (*p).options as libc::c_int & 8 as libc::c_int != 0 {
                        *((*p).entry_buf)
                            .offset(entry_pos as isize) = '\0' as i32 as libc::c_uchar;
                    }
                    if cb1.is_some()
                        && (*p).options as libc::c_int & 16 as libc::c_int != 0
                        && quoted == 0 && entry_pos == 0 as libc::c_int as libc::c_ulong
                    {
                        cb1
                            .expect(
                                "non-null function pointer",
                            )(0 as *mut libc::c_void, entry_pos, data);
                    } else if cb1.is_some() {
                        cb1
                            .expect(
                                "non-null function pointer",
                            )((*p).entry_buf as *mut libc::c_void, entry_pos, data);
                    }
                    pstate = 1 as libc::c_int;
                    spaces = 0 as libc::c_int as size_t;
                    quoted = spaces as libc::c_int;
                    entry_pos = quoted as size_t;
                    if cb2.is_some() {
                        cb2.expect("non-null function pointer")(c as libc::c_int, data);
                    }
                    pstate = 0 as libc::c_int;
                    spaces = 0 as libc::c_int as size_t;
                    quoted = spaces as libc::c_int;
                    entry_pos = quoted as size_t;
                } else if if is_space.is_some() {
                    is_space.expect("non-null function pointer")(c)
                } else {
                    (c as libc::c_int == 0x20 as libc::c_int
                        || c as libc::c_int == 0x9 as libc::c_int) as libc::c_int
                } != 0
                {
                    let fresh25 = entry_pos;
                    entry_pos = entry_pos.wrapping_add(1);
                    *((*p).entry_buf).offset(fresh25 as isize) = c;
                    spaces = spaces.wrapping_add(1);
                } else if c as libc::c_int == quote as libc::c_int {
                    if spaces != 0 {
                        if (*p).options as libc::c_int & 1 as libc::c_int != 0 {
                            (*p).status = 1 as libc::c_int;
                            (*p).quoted = quoted;
                            (*p).pstate = pstate;
                            (*p).spaces = spaces;
                            (*p).entry_pos = entry_pos;
                            return pos.wrapping_sub(1 as libc::c_int as libc::c_ulong);
                        }
                        spaces = 0 as libc::c_int as size_t;
                        let fresh26 = entry_pos;
                        entry_pos = entry_pos.wrapping_add(1);
                        *((*p).entry_buf).offset(fresh26 as isize) = c;
                    } else {
                        pstate = 2 as libc::c_int;
                    }
                } else {
                    if (*p).options as libc::c_int & 1 as libc::c_int != 0 {
                        (*p).status = 1 as libc::c_int;
                        (*p).quoted = quoted;
                        (*p).pstate = pstate;
                        (*p).spaces = spaces;
                        (*p).entry_pos = entry_pos;
                        return pos.wrapping_sub(1 as libc::c_int as libc::c_ulong);
                    }
                    pstate = 2 as libc::c_int;
                    spaces = 0 as libc::c_int as size_t;
                    let fresh27 = entry_pos;
                    entry_pos = entry_pos.wrapping_add(1);
                    *((*p).entry_buf).offset(fresh27 as isize) = c;
                }
            }
            _ => {}
        }
    }
    (*p).quoted = quoted;
    (*p).pstate = pstate;
    (*p).spaces = spaces;
    (*p).entry_pos = entry_pos;
    return pos;
}
#[no_mangle]
pub unsafe extern "C" fn csv_write(
    mut dest: *mut libc::c_void,
    mut dest_size: size_t,
    mut src: *const libc::c_void,
    mut src_size: size_t,
) -> size_t {
    return csv_write2(
        dest,
        dest_size,
        src,
        src_size,
        0x22 as libc::c_int as libc::c_uchar,
    );
}
#[no_mangle]
pub unsafe extern "C" fn csv_fwrite(
    mut fp: *mut FILE,
    mut src: *const libc::c_void,
    mut src_size: size_t,
) -> libc::c_int {
    return csv_fwrite2(fp, src, src_size, 0x22 as libc::c_int as libc::c_uchar);
}
#[no_mangle]
pub unsafe extern "C" fn csv_write2(
    mut dest: *mut libc::c_void,
    mut dest_size: size_t,
    mut src: *const libc::c_void,
    mut src_size: size_t,
    mut quote: libc::c_uchar,
) -> size_t {
    let mut cdest = dest as *mut libc::c_uchar;
    let mut csrc = src as *const libc::c_uchar;
    let mut chars = 0 as libc::c_int as size_t;
    if src.is_null() {
        return 0 as libc::c_int as size_t;
    }
    if dest.is_null() {
        dest_size = 0 as libc::c_int as size_t;
    }
    if dest_size > 0 as libc::c_int as libc::c_ulong {
        let fresh28 = cdest;
        cdest = cdest.offset(1);
        *fresh28 = quote;
    }
    chars = chars.wrapping_add(1);
    while src_size != 0 {
        if *csrc as libc::c_int == quote as libc::c_int {
            if dest_size > chars {
                let fresh29 = cdest;
                cdest = cdest.offset(1);
                *fresh29 = quote;
            }
            if chars < 18446744073709551615 as libc::c_ulong {
                chars = chars.wrapping_add(1);
            }
        }
        if dest_size > chars {
            let fresh30 = cdest;
            cdest = cdest.offset(1);
            *fresh30 = *csrc;
        }
        if chars < 18446744073709551615 as libc::c_ulong {
            chars = chars.wrapping_add(1);
        }
        src_size = src_size.wrapping_sub(1);
        csrc = csrc.offset(1);
    }
    if dest_size > chars {
        *cdest = quote;
    }
    if chars < 18446744073709551615 as libc::c_ulong {
        chars = chars.wrapping_add(1);
    }
    return chars;
}
#[no_mangle]
pub unsafe extern "C" fn csv_fwrite2(
    mut fp: *mut FILE,
    mut src: *const libc::c_void,
    mut src_size: size_t,
    mut quote: libc::c_uchar,
) -> libc::c_int {
    let mut csrc = src as *const libc::c_uchar;
    if fp.is_null() || src.is_null() {
        return 0 as libc::c_int;
    }
    if fputc(quote as libc::c_int, fp) == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    while src_size != 0 {
        if *csrc as libc::c_int == quote as libc::c_int {
            if fputc(quote as libc::c_int, fp) == -(1 as libc::c_int) {
                return -(1 as libc::c_int);
            }
        }
        if fputc(*csrc as libc::c_int, fp) == -(1 as libc::c_int) {
            return -(1 as libc::c_int);
        }
        src_size = src_size.wrapping_sub(1);
        csrc = csrc.offset(1);
    }
    if fputc(quote as libc::c_int, fp) == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
