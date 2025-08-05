use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char
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
    pub _unused2: [libc::c_char; 20]
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
    pub is_space: Option<unsafe extern "C" fn(libc::c_uchar) -> libc::c_int>,
    pub is_term: Option<unsafe extern "C" fn(libc::c_uchar) -> libc::c_int>,
    pub blk_size: size_t,
    pub malloc_func: Option<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
    pub realloc_func: Option<
        unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void
    >,
    pub free_func: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>
}
static mut csv_errors: [*const libc::c_char; 5] = [
    b"success\0" as *const u8 as *const libc::c_char,
    b"error parsing data while strict checking enabled\0" as *const u8
        as *const libc::c_char,
    b"memory exhausted while increasing buffer size\0" as *const u8
        as *const libc::c_char,
    b"data size too large\0" as *const u8 as *const libc::c_char,
    b"invalid status code\0" as *const u8 as *const libc::c_char
];
#[no_mangle]
pub unsafe extern "C" fn csv_error(mut p: *const csv_parser) -> libc::c_int {
    if !p.is_null()
        && !(b"received null csv_parser\0" as *const u8 as *const libc::c_char)
            .is_null()
    {
    } else {
        __assert_fail(
            b"p && \"received null csv_parser\"\0" as *const u8
                as *const libc::c_char,
            b"libcsv.c\0" as *const u8 as *const libc::c_char,
            82 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 41], &[libc::c_char; 41]>(
                b"int csv_error(const struct csv_parser *)\0"
            ))
            .as_ptr()
        );
    }
    'c_2500: {
        if !p.is_null()
            && !(b"received null csv_parser\0" as *const u8
                as *const libc::c_char)
                .is_null()
        {
        } else {
            __assert_fail(
                b"p && \"received null csv_parser\"\0" as *const u8
                    as *const libc::c_char,
                b"libcsv.c\0" as *const u8 as *const libc::c_char,
                82 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 41], &[libc::c_char; 41]>(
                    b"int csv_error(const struct csv_parser *)\0"
                ))
                .as_ptr()
            );
        }
    };
    return (*p).status;
}
#[no_mangle]
pub unsafe extern "C" fn csv_strerror(
    mut status: libc::c_int
) -> *const libc::c_char {
    if status >= 4 as libc::c_int || status < 0 as libc::c_int {
        return csv_errors[4 as libc::c_int as usize];
    } else {
        return csv_errors[status as usize];
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
    mut options: libc::c_uchar
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
    mut options: libc::c_uchar
) -> libc::c_int {
    if p.is_null() {
        return -(1 as libc::c_int);
    }
    (*p).entry_buf = 0 as *mut libc::c_uchar;
    (*p).pstate = 0 as libc::c_int;
    (*p).quoted = 0 as libc::c_int;
    (*p).spaces = 0 as libc::c_int as size_t;
    (*p).entry_pos = 0 as libc::c_int as size_t;
    (*p).entry_size = 0 as libc::c_int as size_t;
    (*p).status = 0 as libc::c_int;
    (*p).options = options;
    (*p).quote_char = 0x22 as libc::c_int as libc::c_uchar;
    (*p).delim_char = 0x2c as libc::c_int as libc::c_uchar;
    (*p).is_space = None;
    (*p).is_term = None;
    (*p).blk_size = 128 as libc::c_int as size_t;
    (*p).malloc_func = None;
    (*p).realloc_func = Some(
        realloc
            as unsafe extern "C" fn(
                *mut libc::c_void,
                libc::c_ulong
            ) -> *mut libc::c_void
    );
    (*p).free_func =
        Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ());
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn csv_free(mut p: *mut csv_parser) {
    if p.is_null() {
        return;
    }
    if !((*p).entry_buf).is_null() && ((*p).free_func).is_some() {
        ((*p).free_func).expect("non-null function pointer")(
            (*p).entry_buf as *mut libc::c_void
        );
    }
    (*p).entry_buf = 0 as *mut libc::c_uchar;
    (*p).entry_size = 0 as libc::c_int as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn csv_fini(
    mut p: *mut csv_parser,
    mut cb1: Option<
        unsafe extern "C" fn(
            *mut libc::c_void,
            size_t,
            *mut libc::c_void
        ) -> ()
    >,
    mut cb2: Option<unsafe extern "C" fn(libc::c_int, *mut libc::c_void) -> ()>,
    mut data: *mut libc::c_void
) -> libc::c_int {
    if p.is_null() {
        return -(1 as libc::c_int);
    }
    let mut quoted: libc::c_int = (*p).quoted;
    let mut pstate: libc::c_int = (*p).pstate;
    let mut spaces: size_t = (*p).spaces;
    let mut entry_pos: size_t = (*p).entry_pos;
    if pstate == 2 as libc::c_int
        && (*p).quoted != 0
        && (*p).options as libc::c_int & 1 as libc::c_int != 0
        && (*p).options as libc::c_int & 4 as libc::c_int != 0
    {
        (*p).status = 1 as libc::c_int;
        return -(1 as libc::c_int);
    }
    let mut current_block_26: u64;
    match pstate {
        3 => {
            (*p).entry_pos = ((*p).entry_pos as libc::c_ulong).wrapping_sub(
                ((*p).spaces).wrapping_add(1 as libc::c_int as libc::c_ulong)
            ) as size_t as size_t;
            entry_pos = (*p).entry_pos;
            current_block_26 = 13803647377203188467;
        }
        1 | 2 => {
            current_block_26 = 13803647377203188467;
        }
        0 | _ => {
            current_block_26 = 15768484401365413375;
        }
    }
    match current_block_26 {
        13803647377203188467 => {
            if quoted == 0 {
                entry_pos = (entry_pos as libc::c_ulong).wrapping_sub(spaces)
                    as size_t as size_t;
            }
            if (*p).options as libc::c_int & 8 as libc::c_int != 0 {
                *((*p).entry_buf).offset(entry_pos as isize) =
                    '\0' as i32 as libc::c_uchar;
            }
            if cb1.is_some()
                && (*p).options as libc::c_int & 16 as libc::c_int != 0
                && quoted == 0
                && entry_pos == 0 as libc::c_int as libc::c_ulong
            {
                cb1.expect("non-null function pointer")(
                    0 as *mut libc::c_void,
                    entry_pos,
                    data
                );
            } else if cb1.is_some() {
                cb1.expect("non-null function pointer")(
                    (*p).entry_buf as *mut libc::c_void,
                    entry_pos,
                    data
                );
            }
            pstate = 1 as libc::c_int;
            spaces = 0 as libc::c_int as size_t;
            quoted = spaces as libc::c_int;
            entry_pos = quoted as size_t;
            if cb2.is_some() {
                cb2.expect("non-null function pointer")(
                    -(1 as libc::c_int),
                    data
                );
            }
            pstate = 0 as libc::c_int;
            spaces = 0 as libc::c_int as size_t;
            quoted = spaces as libc::c_int;
            entry_pos = quoted as size_t;
        }
        _ => {}
    }
    (*p).status = 0 as libc::c_int;
    (*p).entry_pos = (*p).status as size_t;
    (*p).quoted = (*p).entry_pos as libc::c_int;
    (*p).spaces = (*p).quoted as size_t;
    (*p).pstate = 0 as libc::c_int;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn csv_set_delim(
    mut p: *mut csv_parser,
    mut c: libc::c_uchar
) {
    if !p.is_null() {
        (*p).delim_char = c;
    }
}
#[no_mangle]
pub unsafe extern "C" fn csv_set_quote(
    mut p: *mut csv_parser,
    mut c: libc::c_uchar
) {
    if !p.is_null() {
        (*p).quote_char = c;
    }
}
#[no_mangle]
pub unsafe extern "C" fn csv_get_delim(
    mut p: *const csv_parser
) -> libc::c_uchar {
    if !p.is_null()
        && !(b"received null csv_parser\0" as *const u8 as *const libc::c_char)
            .is_null()
    {
    } else {
        __assert_fail(
            b"p && \"received null csv_parser\"\0" as *const u8
                as *const libc::c_char,
            b"libcsv.c\0" as *const u8 as *const libc::c_char,
            222 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 55], &[libc::c_char; 55]>(
                b"unsigned char csv_get_delim(const struct csv_parser *)\0"
            ))
            .as_ptr()
        );
    }
    'c_4736: {
        if !p.is_null()
            && !(b"received null csv_parser\0" as *const u8
                as *const libc::c_char)
                .is_null()
        {
        } else {
            __assert_fail(
                b"p && \"received null csv_parser\"\0" as *const u8
                    as *const libc::c_char,
                b"libcsv.c\0" as *const u8 as *const libc::c_char,
                222 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 55], &[libc::c_char; 55]>(
                    b"unsigned char csv_get_delim(const struct csv_parser *)\0"
                ))
                .as_ptr()
            );
        }
    };
    return (*p).delim_char;
}
#[no_mangle]
pub unsafe extern "C" fn csv_get_quote(
    mut p: *const csv_parser
) -> libc::c_uchar {
    if !p.is_null()
        && !(b"received null csv_parser\0" as *const u8 as *const libc::c_char)
            .is_null()
    {
    } else {
        __assert_fail(
            b"p && \"received null csv_parser\"\0" as *const u8
                as *const libc::c_char,
            b"libcsv.c\0" as *const u8 as *const libc::c_char,
            231 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 55], &[libc::c_char; 55]>(
                b"unsigned char csv_get_quote(const struct csv_parser *)\0"
            ))
            .as_ptr()
        );
    }
    'c_4784: {
        if !p.is_null()
            && !(b"received null csv_parser\0" as *const u8
                as *const libc::c_char)
                .is_null()
        {
        } else {
            __assert_fail(
                b"p && \"received null csv_parser\"\0" as *const u8
                    as *const libc::c_char,
                b"libcsv.c\0" as *const u8 as *const libc::c_char,
                231 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 55], &[libc::c_char; 55]>(
                    b"unsigned char csv_get_quote(const struct csv_parser *)\0"
                ))
                .as_ptr()
            );
        }
    };
    return (*p).quote_char;
}
#[no_mangle]
pub unsafe extern "C" fn csv_set_space_func(
    mut p: *mut csv_parser,
    mut f: Option<unsafe extern "C" fn(libc::c_uchar) -> libc::c_int>
) {
    if !p.is_null() {
        (*p).is_space = f;
    }
}
#[no_mangle]
pub unsafe extern "C" fn csv_set_term_func(
    mut p: *mut csv_parser,
    mut f: Option<unsafe extern "C" fn(libc::c_uchar) -> libc::c_int>
) {
    if !p.is_null() {
        (*p).is_term = f;
    }
}
#[no_mangle]
pub unsafe extern "C" fn csv_set_realloc_func(
    mut p: *mut csv_parser,
    mut f: Option<
        unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void
    >
) {
    if !p.is_null() && f.is_some() {
        (*p).realloc_func = f;
    }
}
#[no_mangle]
pub unsafe extern "C" fn csv_set_free_func(
    mut p: *mut csv_parser,
    mut f: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>
) {
    if !p.is_null() && f.is_some() {
        (*p).free_func = f;
    }
}
#[no_mangle]
pub unsafe extern "C" fn csv_set_blk_size(
    mut p: *mut csv_parser,
    mut size: size_t
) {
    if !p.is_null() {
        (*p).blk_size = size;
    }
}
#[no_mangle]
pub unsafe extern "C" fn csv_get_buffer_size(
    mut p: *const csv_parser
) -> size_t {
    if !p.is_null() {
        return (*p).entry_size;
    }
    return 0 as libc::c_int as size_t;
}
unsafe extern "C" fn csv_increase_buffer(
    mut p: *mut csv_parser
) -> libc::c_int {
    if p.is_null() {
        return 0 as libc::c_int;
    }
    if ((*p).realloc_func).is_none() {
        return 0 as libc::c_int;
    }
    let mut to_add: size_t = (*p).blk_size;
    let mut vp: *mut libc::c_void = 0 as *mut libc::c_void;
    if (*p).entry_size
        >= (18446744073709551615 as libc::c_ulong).wrapping_sub(to_add)
    {
        to_add = (18446744073709551615 as libc::c_ulong)
            .wrapping_sub((*p).entry_size);
    }
    if to_add == 0 {
        (*p).status = 3 as libc::c_int;
        return -(1 as libc::c_int);
    }
    loop {
        vp = ((*p).realloc_func).expect("non-null function pointer")(
            (*p).entry_buf as *mut libc::c_void,
            ((*p).entry_size).wrapping_add(to_add)
        );
        if !vp.is_null() {
            break;
        }
        to_add = (to_add as libc::c_ulong)
            .wrapping_div(2 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
        if to_add == 0 {
            (*p).status = 2 as libc::c_int;
            return -(1 as libc::c_int);
        }
    }
    (*p).entry_buf = vp as *mut libc::c_uchar;
    (*p).entry_size = ((*p).entry_size as libc::c_ulong).wrapping_add(to_add)
        as size_t as size_t;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn csv_parse(
    mut p: *mut csv_parser,
    mut s: *const libc::c_void,
    mut len: size_t,
    mut cb1: Option<
        unsafe extern "C" fn(
            *mut libc::c_void,
            size_t,
            *mut libc::c_void
        ) -> ()
    >,
    mut cb2: Option<unsafe extern "C" fn(libc::c_int, *mut libc::c_void) -> ()>,
    mut data: *mut libc::c_void
) -> size_t {
    if !p.is_null()
        && !(b"received null csv_parser\0" as *const u8 as *const libc::c_char)
            .is_null()
    {
    } else {
        __assert_fail(
            b"p && \"received null csv_parser\"\0" as *const u8 as *const libc::c_char,
            b"libcsv.c\0" as *const u8 as *const libc::c_char,
            321 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 125],
                &[libc::c_char; 125],
            >(
                b"size_t csv_parse(struct csv_parser *, const void *, size_t, void (*)(void *, size_t, void *), void (*)(int, void *), void *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_4295: {
        if !p.is_null()
            && !(b"received null csv_parser\0" as *const u8
                as *const libc::c_char)
                .is_null()
        {
        } else {
            __assert_fail(
                b"p && \"received null csv_parser\"\0" as *const u8
                    as *const libc::c_char,
                b"libcsv.c\0" as *const u8 as *const libc::c_char,
                321 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 125],
                    &[libc::c_char; 125],
                >(
                    b"size_t csv_parse(struct csv_parser *, const void *, size_t, void (*)(void *, size_t, void *), void (*)(int, void *), void *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if s.is_null() {
        return 0 as libc::c_int as size_t;
    }
    let mut us: *const libc::c_uchar = s as *const libc::c_uchar;
    let mut c: libc::c_uchar = 0;
    let mut pos: size_t = 0 as libc::c_int as size_t;
    let mut delim: libc::c_uchar = (*p).delim_char;
    let mut quote: libc::c_uchar = (*p).quote_char;
    let mut is_space: Option<
        unsafe extern "C" fn(libc::c_uchar) -> libc::c_int
    > = (*p).is_space;
    let mut is_term: Option<
        unsafe extern "C" fn(libc::c_uchar) -> libc::c_int
    > = (*p).is_term;
    let mut quoted: libc::c_int = (*p).quoted;
    let mut pstate: libc::c_int = (*p).pstate;
    let mut spaces: size_t = (*p).spaces;
    let mut entry_pos: size_t = (*p).entry_pos;
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
                ((*p).entry_size)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
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
        let fresh0 = pos;
        pos = pos.wrapping_add(1);
        c = *us.offset(fresh0 as isize);
        match pstate {
            0 | 1 => {
                if (if is_space.is_some() {
                    is_space.expect("non-null function pointer")(c)
                } else {
                    (c as libc::c_int == 0x20 as libc::c_int
                        || c as libc::c_int == 0x9 as libc::c_int)
                        as libc::c_int
                }) != 0
                    && c as libc::c_int != delim as libc::c_int
                {
                    continue;
                }
                if if is_term.is_some() {
                    is_term.expect("non-null function pointer")(c)
                } else {
                    (c as libc::c_int == 0xd as libc::c_int
                        || c as libc::c_int == 0xa as libc::c_int)
                        as libc::c_int
                } != 0
                {
                    if pstate == 1 as libc::c_int {
                        if quoted == 0 {
                            entry_pos = (entry_pos as libc::c_ulong)
                                .wrapping_sub(spaces)
                                as size_t
                                as size_t;
                        }
                        if (*p).options as libc::c_int & 8 as libc::c_int != 0 {
                            *((*p).entry_buf).offset(entry_pos as isize) =
                                '\0' as i32 as libc::c_uchar;
                        }
                        if cb1.is_some()
                            && (*p).options as libc::c_int & 16 as libc::c_int
                                != 0
                            && quoted == 0
                            && entry_pos == 0 as libc::c_int as libc::c_ulong
                        {
                            cb1.expect("non-null function pointer")(
                                0 as *mut libc::c_void,
                                entry_pos,
                                data
                            );
                        } else if cb1.is_some() {
                            cb1.expect("non-null function pointer")(
                                (*p).entry_buf as *mut libc::c_void,
                                entry_pos,
                                data
                            );
                        }
                        pstate = 1 as libc::c_int;
                        spaces = 0 as libc::c_int as size_t;
                        quoted = spaces as libc::c_int;
                        entry_pos = quoted as size_t;
                        if cb2.is_some() {
                            cb2.expect("non-null function pointer")(
                                c as libc::c_int,
                                data
                            );
                        }
                        pstate = 0 as libc::c_int;
                        spaces = 0 as libc::c_int as size_t;
                        quoted = spaces as libc::c_int;
                        entry_pos = quoted as size_t;
                    } else if (*p).options as libc::c_int & 2 as libc::c_int
                        != 0
                    {
                        if cb2.is_some() {
                            cb2.expect("non-null function pointer")(
                                c as libc::c_int,
                                data
                            );
                        }
                        pstate = 0 as libc::c_int;
                        spaces = 0 as libc::c_int as size_t;
                        quoted = spaces as libc::c_int;
                        entry_pos = quoted as size_t;
                    }
                } else if c as libc::c_int == delim as libc::c_int {
                    if quoted == 0 {
                        entry_pos =
                            (entry_pos as libc::c_ulong).wrapping_sub(spaces)
                                as size_t as size_t;
                    }
                    if (*p).options as libc::c_int & 8 as libc::c_int != 0 {
                        *((*p).entry_buf).offset(entry_pos as isize) =
                            '\0' as i32 as libc::c_uchar;
                    }
                    if cb1.is_some()
                        && (*p).options as libc::c_int & 16 as libc::c_int != 0
                        && quoted == 0
                        && entry_pos == 0 as libc::c_int as libc::c_ulong
                    {
                        cb1.expect("non-null function pointer")(
                            0 as *mut libc::c_void,
                            entry_pos,
                            data
                        );
                    } else if cb1.is_some() {
                        cb1.expect("non-null function pointer")(
                            (*p).entry_buf as *mut libc::c_void,
                            entry_pos,
                            data
                        );
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
                    let fresh1 = entry_pos;
                    entry_pos = entry_pos.wrapping_add(1);
                    *((*p).entry_buf).offset(fresh1 as isize) = c;
                }
            }
            2 => {
                if c as libc::c_int == quote as libc::c_int {
                    if quoted != 0 {
                        let fresh2 = entry_pos;
                        entry_pos = entry_pos.wrapping_add(1);
                        *((*p).entry_buf).offset(fresh2 as isize) = c;
                        pstate = 3 as libc::c_int;
                    } else {
                        if (*p).options as libc::c_int & 1 as libc::c_int != 0 {
                            (*p).status = 1 as libc::c_int;
                            (*p).quoted = quoted;
                            (*p).pstate = pstate;
                            (*p).spaces = spaces;
                            (*p).entry_pos = entry_pos;
                            return pos.wrapping_sub(
                                1 as libc::c_int as libc::c_ulong
                            );
                        }
                        let fresh3 = entry_pos;
                        entry_pos = entry_pos.wrapping_add(1);
                        *((*p).entry_buf).offset(fresh3 as isize) = c;
                        spaces = 0 as libc::c_int as size_t;
                    }
                } else if c as libc::c_int == delim as libc::c_int {
                    if quoted != 0 {
                        let fresh4 = entry_pos;
                        entry_pos = entry_pos.wrapping_add(1);
                        *((*p).entry_buf).offset(fresh4 as isize) = c;
                    } else {
                        if quoted == 0 {
                            entry_pos = (entry_pos as libc::c_ulong)
                                .wrapping_sub(spaces)
                                as size_t
                                as size_t;
                        }
                        if (*p).options as libc::c_int & 8 as libc::c_int != 0 {
                            *((*p).entry_buf).offset(entry_pos as isize) =
                                '\0' as i32 as libc::c_uchar;
                        }
                        if cb1.is_some()
                            && (*p).options as libc::c_int & 16 as libc::c_int
                                != 0
                            && quoted == 0
                            && entry_pos == 0 as libc::c_int as libc::c_ulong
                        {
                            cb1.expect("non-null function pointer")(
                                0 as *mut libc::c_void,
                                entry_pos,
                                data
                            );
                        } else if cb1.is_some() {
                            cb1.expect("non-null function pointer")(
                                (*p).entry_buf as *mut libc::c_void,
                                entry_pos,
                                data
                            );
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
                        || c as libc::c_int == 0xa as libc::c_int)
                        as libc::c_int
                } != 0
                {
                    if quoted == 0 {
                        if quoted == 0 {
                            entry_pos = (entry_pos as libc::c_ulong)
                                .wrapping_sub(spaces)
                                as size_t
                                as size_t;
                        }
                        if (*p).options as libc::c_int & 8 as libc::c_int != 0 {
                            *((*p).entry_buf).offset(entry_pos as isize) =
                                '\0' as i32 as libc::c_uchar;
                        }
                        if cb1.is_some()
                            && (*p).options as libc::c_int & 16 as libc::c_int
                                != 0
                            && quoted == 0
                            && entry_pos == 0 as libc::c_int as libc::c_ulong
                        {
                            cb1.expect("non-null function pointer")(
                                0 as *mut libc::c_void,
                                entry_pos,
                                data
                            );
                        } else if cb1.is_some() {
                            cb1.expect("non-null function pointer")(
                                (*p).entry_buf as *mut libc::c_void,
                                entry_pos,
                                data
                            );
                        }
                        pstate = 1 as libc::c_int;
                        spaces = 0 as libc::c_int as size_t;
                        quoted = spaces as libc::c_int;
                        entry_pos = quoted as size_t;
                        if cb2.is_some() {
                            cb2.expect("non-null function pointer")(
                                c as libc::c_int,
                                data
                            );
                        }
                        pstate = 0 as libc::c_int;
                        spaces = 0 as libc::c_int as size_t;
                        quoted = spaces as libc::c_int;
                        entry_pos = quoted as size_t;
                    } else {
                        let fresh5 = entry_pos;
                        entry_pos = entry_pos.wrapping_add(1);
                        *((*p).entry_buf).offset(fresh5 as isize) = c;
                    }
                } else if quoted == 0
                    && (if is_space.is_some() {
                        is_space.expect("non-null function pointer")(c)
                    } else {
                        (c as libc::c_int == 0x20 as libc::c_int
                            || c as libc::c_int == 0x9 as libc::c_int)
                            as libc::c_int
                    }) != 0
                {
                    let fresh6 = entry_pos;
                    entry_pos = entry_pos.wrapping_add(1);
                    *((*p).entry_buf).offset(fresh6 as isize) = c;
                    spaces = spaces.wrapping_add(1);
                    spaces;
                } else {
                    let fresh7 = entry_pos;
                    entry_pos = entry_pos.wrapping_add(1);
                    *((*p).entry_buf).offset(fresh7 as isize) = c;
                    spaces = 0 as libc::c_int as size_t;
                }
            }
            3 => {
                if c as libc::c_int == delim as libc::c_int {
                    entry_pos = (entry_pos as libc::c_ulong).wrapping_sub(
                        spaces.wrapping_add(1 as libc::c_int as libc::c_ulong)
                    ) as size_t as size_t;
                    if quoted == 0 {
                        entry_pos =
                            (entry_pos as libc::c_ulong).wrapping_sub(spaces)
                                as size_t as size_t;
                    }
                    if (*p).options as libc::c_int & 8 as libc::c_int != 0 {
                        *((*p).entry_buf).offset(entry_pos as isize) =
                            '\0' as i32 as libc::c_uchar;
                    }
                    if cb1.is_some()
                        && (*p).options as libc::c_int & 16 as libc::c_int != 0
                        && quoted == 0
                        && entry_pos == 0 as libc::c_int as libc::c_ulong
                    {
                        cb1.expect("non-null function pointer")(
                            0 as *mut libc::c_void,
                            entry_pos,
                            data
                        );
                    } else if cb1.is_some() {
                        cb1.expect("non-null function pointer")(
                            (*p).entry_buf as *mut libc::c_void,
                            entry_pos,
                            data
                        );
                    }
                    pstate = 1 as libc::c_int;
                    spaces = 0 as libc::c_int as size_t;
                    quoted = spaces as libc::c_int;
                    entry_pos = quoted as size_t;
                } else if if is_term.is_some() {
                    is_term.expect("non-null function pointer")(c)
                } else {
                    (c as libc::c_int == 0xd as libc::c_int
                        || c as libc::c_int == 0xa as libc::c_int)
                        as libc::c_int
                } != 0
                {
                    entry_pos = (entry_pos as libc::c_ulong).wrapping_sub(
                        spaces.wrapping_add(1 as libc::c_int as libc::c_ulong)
                    ) as size_t as size_t;
                    if quoted == 0 {
                        entry_pos =
                            (entry_pos as libc::c_ulong).wrapping_sub(spaces)
                                as size_t as size_t;
                    }
                    if (*p).options as libc::c_int & 8 as libc::c_int != 0 {
                        *((*p).entry_buf).offset(entry_pos as isize) =
                            '\0' as i32 as libc::c_uchar;
                    }
                    if cb1.is_some()
                        && (*p).options as libc::c_int & 16 as libc::c_int != 0
                        && quoted == 0
                        && entry_pos == 0 as libc::c_int as libc::c_ulong
                    {
                        cb1.expect("non-null function pointer")(
                            0 as *mut libc::c_void,
                            entry_pos,
                            data
                        );
                    } else if cb1.is_some() {
                        cb1.expect("non-null function pointer")(
                            (*p).entry_buf as *mut libc::c_void,
                            entry_pos,
                            data
                        );
                    }
                    pstate = 1 as libc::c_int;
                    spaces = 0 as libc::c_int as size_t;
                    quoted = spaces as libc::c_int;
                    entry_pos = quoted as size_t;
                    if cb2.is_some() {
                        cb2.expect("non-null function pointer")(
                            c as libc::c_int,
                            data
                        );
                    }
                    pstate = 0 as libc::c_int;
                    spaces = 0 as libc::c_int as size_t;
                    quoted = spaces as libc::c_int;
                    entry_pos = quoted as size_t;
                } else if if is_space.is_some() {
                    is_space.expect("non-null function pointer")(c)
                } else {
                    (c as libc::c_int == 0x20 as libc::c_int
                        || c as libc::c_int == 0x9 as libc::c_int)
                        as libc::c_int
                } != 0
                {
                    let fresh8 = entry_pos;
                    entry_pos = entry_pos.wrapping_add(1);
                    *((*p).entry_buf).offset(fresh8 as isize) = c;
                    spaces = spaces.wrapping_add(1);
                    spaces;
                } else if c as libc::c_int == quote as libc::c_int {
                    if spaces != 0 {
                        if (*p).options as libc::c_int & 1 as libc::c_int != 0 {
                            (*p).status = 1 as libc::c_int;
                            (*p).quoted = quoted;
                            (*p).pstate = pstate;
                            (*p).spaces = spaces;
                            (*p).entry_pos = entry_pos;
                            return pos.wrapping_sub(
                                1 as libc::c_int as libc::c_ulong
                            );
                        }
                        spaces = 0 as libc::c_int as size_t;
                        let fresh9 = entry_pos;
                        entry_pos = entry_pos.wrapping_add(1);
                        *((*p).entry_buf).offset(fresh9 as isize) = c;
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
                        return pos
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
                    }
                    pstate = 2 as libc::c_int;
                    spaces = 0 as libc::c_int as size_t;
                    let fresh10 = entry_pos;
                    entry_pos = entry_pos.wrapping_add(1);
                    *((*p).entry_buf).offset(fresh10 as isize) = c;
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
    mut src_size: size_t
) -> size_t {
    return csv_write2(
        dest,
        dest_size,
        src,
        src_size,
        0x22 as libc::c_int as libc::c_uchar
    );
}
#[no_mangle]
pub unsafe extern "C" fn csv_fwrite(
    mut fp: *mut FILE,
    mut src: *const libc::c_void,
    mut src_size: size_t
) -> libc::c_int {
    return csv_fwrite2(
        fp,
        src,
        src_size,
        0x22 as libc::c_int as libc::c_uchar
    );
}
#[no_mangle]
pub unsafe extern "C" fn csv_write2(
    mut dest: *mut libc::c_void,
    mut dest_size: size_t,
    mut src: *const libc::c_void,
    mut src_size: size_t,
    mut quote: libc::c_uchar
) -> size_t {
    let mut cdest: *mut libc::c_uchar = dest as *mut libc::c_uchar;
    let mut csrc: *const libc::c_uchar = src as *const libc::c_uchar;
    let mut chars: size_t = 0 as libc::c_int as size_t;
    if src.is_null() {
        return 0 as libc::c_int as size_t;
    }
    if dest.is_null() {
        dest_size = 0 as libc::c_int as size_t;
    }
    if dest_size > 0 as libc::c_int as libc::c_ulong {
        let fresh11 = cdest;
        cdest = cdest.offset(1);
        *fresh11 = quote;
    }
    chars = chars.wrapping_add(1);
    chars;
    while src_size != 0 {
        if *csrc as libc::c_int == quote as libc::c_int {
            if dest_size > chars {
                let fresh12 = cdest;
                cdest = cdest.offset(1);
                *fresh12 = quote;
            }
            if chars < 18446744073709551615 as libc::c_ulong {
                chars = chars.wrapping_add(1);
                chars;
            }
        }
        if dest_size > chars {
            let fresh13 = cdest;
            cdest = cdest.offset(1);
            *fresh13 = *csrc;
        }
        if chars < 18446744073709551615 as libc::c_ulong {
            chars = chars.wrapping_add(1);
            chars;
        }
        src_size = src_size.wrapping_sub(1);
        src_size;
        csrc = csrc.offset(1);
        csrc;
    }
    if dest_size > chars {
        *cdest = quote;
    }
    if chars < 18446744073709551615 as libc::c_ulong {
        chars = chars.wrapping_add(1);
        chars;
    }
    return chars;
}
#[no_mangle]
pub unsafe extern "C" fn csv_fwrite2(
    mut fp: *mut FILE,
    mut src: *const libc::c_void,
    mut src_size: size_t,
    mut quote: libc::c_uchar
) -> libc::c_int {
    let mut csrc: *const libc::c_uchar = src as *const libc::c_uchar;
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
        src_size;
        csrc = csrc.offset(1);
        csrc;
    }
    if fputc(quote as libc::c_int, fp) == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
