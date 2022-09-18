use ::libc;
extern "C" {
    /* Declare our basic types. */
    pub type archive;
    #[no_mangle]
    fn archive_read_support_filter_program(
        _: *mut archive,
        command: *const libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn archive_write_add_filter_by_name(_: *mut archive, name: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn archive_write_add_filter_program(_: *mut archive, cmd: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn lafe_errc(eval: libc::c_int, code: libc::c_int, fmt: *const libc::c_char, _: ...) -> !;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct creation_set {
    pub create_format: *mut libc::c_char,
    pub filters: *mut filter_set,
    pub filter_count: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct filter_set {
    pub program: libc::c_int,
    pub filter_name: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct suffix_code_t {
    pub suffix: *const libc::c_char,
    pub form: *const libc::c_char,
}
/*
 * Error codes: Use archive_errno() and archive_error_string()
 * to retrieve details.  Unless specified otherwise, all functions
 * that return 'int' use these codes.
 */
/* Found end of archive. */
/* Operation was successful. */
/* Retry might succeed. */
pub const ARCHIVE_WARN: libc::c_int = -(20 as libc::c_int);
pub const NULL: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn get_suffix_code(
    mut tbl: *const suffix_code_t,
    mut suffix: *const libc::c_char,
) -> *const libc::c_char {
    let mut i: libc::c_int = 0;
    if suffix.is_null() {
        return 0 as *const libc::c_char;
    }
    i = 0 as libc::c_int;
    while !(*tbl.offset(i as isize)).suffix.is_null() {
        if strcmp((*tbl.offset(i as isize)).suffix, suffix) == 0 as libc::c_int {
            return (*tbl.offset(i as isize)).form;
        }
        i += 1
    }
    return 0 as *const libc::c_char;
}
unsafe extern "C" fn get_filter_code(mut suffix: *const libc::c_char) -> *const libc::c_char {
    /* A pair of suffix and compression/filter. */
    static mut filters: [suffix_code_t; 13] = [
        {
            let mut init = suffix_code_t {
                suffix: b".Z\x00" as *const u8 as *const libc::c_char,
                form: b"compress\x00" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = suffix_code_t {
                suffix: b".bz2\x00" as *const u8 as *const libc::c_char,
                form: b"bzip2\x00" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = suffix_code_t {
                suffix: b".gz\x00" as *const u8 as *const libc::c_char,
                form: b"gzip\x00" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = suffix_code_t {
                suffix: b".grz\x00" as *const u8 as *const libc::c_char,
                form: b"grzip\x00" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = suffix_code_t {
                suffix: b".lrz\x00" as *const u8 as *const libc::c_char,
                form: b"lrzip\x00" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = suffix_code_t {
                suffix: b".lz\x00" as *const u8 as *const libc::c_char,
                form: b"lzip\x00" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = suffix_code_t {
                suffix: b".lz4\x00" as *const u8 as *const libc::c_char,
                form: b"lz4\x00" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = suffix_code_t {
                suffix: b".lzo\x00" as *const u8 as *const libc::c_char,
                form: b"lzop\x00" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = suffix_code_t {
                suffix: b".lzma\x00" as *const u8 as *const libc::c_char,
                form: b"lzma\x00" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = suffix_code_t {
                suffix: b".uu\x00" as *const u8 as *const libc::c_char,
                form: b"uuencode\x00" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = suffix_code_t {
                suffix: b".xz\x00" as *const u8 as *const libc::c_char,
                form: b"xz\x00" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = suffix_code_t {
                suffix: b".zst\x00" as *const u8 as *const libc::c_char,
                form: b"zstd\x00" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = suffix_code_t {
                suffix: NULL as *const libc::c_char,
                form: NULL as *const libc::c_char,
            };
            init
        },
    ];
    return get_suffix_code(filters.as_ptr(), suffix);
}
unsafe extern "C" fn get_format_code(mut suffix: *const libc::c_char) -> *const libc::c_char {
    /* A pair of suffix and format. */
    static mut formats: [suffix_code_t; 11] = [
        {
            let mut init = suffix_code_t {
                suffix: b".7z\x00" as *const u8 as *const libc::c_char,
                form: b"7zip\x00" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = suffix_code_t {
                suffix: b".ar\x00" as *const u8 as *const libc::c_char,
                form: b"arbsd\x00" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = suffix_code_t {
                suffix: b".cpio\x00" as *const u8 as *const libc::c_char,
                form: b"cpio\x00" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = suffix_code_t {
                suffix: b".iso\x00" as *const u8 as *const libc::c_char,
                form: b"iso9960\x00" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = suffix_code_t {
                suffix: b".mtree\x00" as *const u8 as *const libc::c_char,
                form: b"mtree\x00" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = suffix_code_t {
                suffix: b".shar\x00" as *const u8 as *const libc::c_char,
                form: b"shar\x00" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = suffix_code_t {
                suffix: b".tar\x00" as *const u8 as *const libc::c_char,
                form: b"paxr\x00" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = suffix_code_t {
                suffix: b".warc\x00" as *const u8 as *const libc::c_char,
                form: b"warc\x00" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = suffix_code_t {
                suffix: b".xar\x00" as *const u8 as *const libc::c_char,
                form: b"xar\x00" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = suffix_code_t {
                suffix: b".zip\x00" as *const u8 as *const libc::c_char,
                form: b"zip\x00" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = suffix_code_t {
                suffix: NULL as *const libc::c_char,
                form: NULL as *const libc::c_char,
            };
            init
        },
    ];
    return get_suffix_code(formats.as_ptr(), suffix);
}
unsafe extern "C" fn decompose_alias(mut suffix: *const libc::c_char) -> *const libc::c_char {
    static mut alias: [suffix_code_t; 12] = [
        {
            let mut init = suffix_code_t {
                suffix: b".taz\x00" as *const u8 as *const libc::c_char,
                form: b".tar.gz\x00" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = suffix_code_t {
                suffix: b".tgz\x00" as *const u8 as *const libc::c_char,
                form: b".tar.gz\x00" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = suffix_code_t {
                suffix: b".tbz\x00" as *const u8 as *const libc::c_char,
                form: b".tar.bz2\x00" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = suffix_code_t {
                suffix: b".tbz2\x00" as *const u8 as *const libc::c_char,
                form: b".tar.bz2\x00" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = suffix_code_t {
                suffix: b".tz2\x00" as *const u8 as *const libc::c_char,
                form: b".tar.bz2\x00" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = suffix_code_t {
                suffix: b".tlz\x00" as *const u8 as *const libc::c_char,
                form: b".tar.lzma\x00" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = suffix_code_t {
                suffix: b".txz\x00" as *const u8 as *const libc::c_char,
                form: b".tar.xz\x00" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = suffix_code_t {
                suffix: b".tzo\x00" as *const u8 as *const libc::c_char,
                form: b".tar.lzo\x00" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = suffix_code_t {
                suffix: b".taZ\x00" as *const u8 as *const libc::c_char,
                form: b".tar.Z\x00" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = suffix_code_t {
                suffix: b".tZ\x00" as *const u8 as *const libc::c_char,
                form: b".tar.Z\x00" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = suffix_code_t {
                suffix: b".tzst\x00" as *const u8 as *const libc::c_char,
                form: b".tar.zst\x00" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = suffix_code_t {
                suffix: NULL as *const libc::c_char,
                form: NULL as *const libc::c_char,
            };
            init
        },
    ];
    return get_suffix_code(alias.as_ptr(), suffix);
}
unsafe extern "C" fn _cset_add_filter(
    mut cset: *mut creation_set,
    mut program: libc::c_int,
    mut filter: *const libc::c_char,
) {
    let mut new_ptr: *mut filter_set = 0 as *mut filter_set;
    let mut new_filter: *mut libc::c_char = 0 as *mut libc::c_char;
    new_ptr = realloc(
        (*cset).filters as *mut libc::c_void,
        (::std::mem::size_of::<filter_set>() as libc::c_ulong)
            .wrapping_mul(((*cset).filter_count + 1 as libc::c_int) as libc::c_ulong),
    ) as *mut filter_set;
    if new_ptr.is_null() {
        lafe_errc(
            1 as libc::c_int,
            0 as libc::c_int,
            b"No memory\x00" as *const u8 as *const libc::c_char,
        );
    }
    new_filter = strdup(filter);
    if new_filter.is_null() {
        lafe_errc(
            1 as libc::c_int,
            0 as libc::c_int,
            b"No memory\x00" as *const u8 as *const libc::c_char,
        );
    }
    (*cset).filters = new_ptr;
    (*(*cset).filters.offset((*cset).filter_count as isize)).program = program;
    let ref mut fresh0 = (*(*cset).filters.offset((*cset).filter_count as isize)).filter_name;
    *fresh0 = new_filter;
    (*cset).filter_count += 1;
}
#[no_mangle]
pub unsafe extern "C" fn cset_add_filter(
    mut cset: *mut creation_set,
    mut filter: *const libc::c_char,
) {
    _cset_add_filter(cset, 0 as libc::c_int, filter);
}
#[no_mangle]
pub unsafe extern "C" fn cset_add_filter_program(
    mut cset: *mut creation_set,
    mut filter: *const libc::c_char,
) {
    _cset_add_filter(cset, 1 as libc::c_int, filter);
}
#[no_mangle]
pub unsafe extern "C" fn cset_read_support_filter_program(
    mut cset: *mut creation_set,
    mut a: *mut archive,
) -> libc::c_int {
    let mut cnt: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*cset).filter_count {
        if (*(*cset).filters.offset(i as isize)).program != 0 {
            archive_read_support_filter_program(
                a,
                (*(*cset).filters.offset(i as isize)).filter_name,
            );
            cnt += 1
        }
        i += 1
    }
    return cnt;
}
#[no_mangle]
pub unsafe extern "C" fn cset_write_add_filters(
    mut cset: *mut creation_set,
    mut a: *mut archive,
    mut filter_name: *mut *const libc::c_void,
) -> libc::c_int {
    let mut cnt: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*cset).filter_count {
        if (*(*cset).filters.offset(i as isize)).program != 0 {
            r = archive_write_add_filter_program(
                a,
                (*(*cset).filters.offset(i as isize)).filter_name,
            )
        } else {
            r = archive_write_add_filter_by_name(
                a,
                (*(*cset).filters.offset(i as isize)).filter_name,
            )
        }
        if r < ARCHIVE_WARN {
            *filter_name = (*(*cset).filters.offset(i as isize)).filter_name as *const libc::c_void;
            return r;
        }
        cnt += 1;
        i += 1
    }
    return cnt;
}
#[no_mangle]
pub unsafe extern "C" fn cset_set_format(
    mut cset: *mut creation_set,
    mut format: *const libc::c_char,
) {
    let mut f: *mut libc::c_char = 0 as *mut libc::c_char;
    f = strdup(format);
    if f.is_null() {
        lafe_errc(
            1 as libc::c_int,
            0 as libc::c_int,
            b"No memory\x00" as *const u8 as *const libc::c_char,
        );
    }
    free((*cset).create_format as *mut libc::c_void);
    (*cset).create_format = f;
}
#[no_mangle]
pub unsafe extern "C" fn cset_get_format(mut cset: *mut creation_set) -> *const libc::c_char {
    return (*cset).create_format;
}
unsafe extern "C" fn _cleanup_filters(mut filters: *mut filter_set, mut count: libc::c_int) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < count {
        free((*filters.offset(i as isize)).filter_name as *mut libc::c_void);
        i += 1
    }
    free(filters as *mut libc::c_void);
}
/*
 * Clean up a creation set.
 */
#[no_mangle]
pub unsafe extern "C" fn cset_free(mut cset: *mut creation_set) {
    _cleanup_filters((*cset).filters, (*cset).filter_count);
    free((*cset).create_format as *mut libc::c_void);
    free(cset as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn cset_new() -> *mut creation_set {
    return calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<creation_set>() as libc::c_ulong,
    ) as *mut creation_set;
}
/*
 * Build a creation set by a file name suffix.
 */
#[no_mangle]
pub unsafe extern "C" fn cset_auto_compress(
    mut cset: *mut creation_set,
    mut filename: *const libc::c_char,
) -> libc::c_int {
    let mut old_filters: *mut filter_set = 0 as *mut filter_set;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut code: *const libc::c_char = 0 as *const libc::c_char;
    let mut old_filter_count: libc::c_int = 0;
    name = strdup(filename);
    if name.is_null() {
        lafe_errc(
            1 as libc::c_int,
            0 as libc::c_int,
            b"No memory\x00" as *const u8 as *const libc::c_char,
        );
    }
    /* Save previous filters. */
    old_filters = (*cset).filters;
    old_filter_count = (*cset).filter_count;
    (*cset).filters = NULL as *mut filter_set;
    (*cset).filter_count = 0 as libc::c_int;
    loop {
        /* Get the suffix. */
        p = strrchr(name, '.' as i32);
        if p.is_null() {
            break;
        }
        /* Suppose it indicates compression/filter type
         * such as ".gz". */
        code = get_filter_code(p);
        if !code.is_null() {
            cset_add_filter(cset, code);
            *p = '\u{0}' as i32 as libc::c_char
        } else {
            /* Suppose it indicates format type such as ".tar". */
            code = get_format_code(p);
            if !code.is_null() {
                cset_set_format(cset, code);
                break;
            } else {
                /* Suppose it indicates alias such as ".tgz". */
                code = decompose_alias(p);
                if code.is_null() {
                    break;
                }
                /* Replace the suffix. */
                *p = '\u{0}' as i32 as libc::c_char;
                name = realloc(
                    name as *mut libc::c_void,
                    strlen(name)
                        .wrapping_add(strlen(code))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                ) as *mut libc::c_char;
                if name.is_null() {
                    lafe_errc(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        b"No memory\x00" as *const u8 as *const libc::c_char,
                    );
                }
                strcat(name, code);
            }
        }
    }
    free(name as *mut libc::c_void);
    if !(*cset).filters.is_null() {
        let mut v: *mut filter_set = 0 as *mut filter_set;
        let mut i: libc::c_int = 0;
        let mut r: libc::c_int = 0;
        /* Release previous filters. */
        _cleanup_filters(old_filters, old_filter_count);
        v = malloc(
            (::std::mem::size_of::<filter_set>() as libc::c_ulong)
                .wrapping_mul((*cset).filter_count as libc::c_ulong),
        ) as *mut filter_set;
        if v.is_null() {
            lafe_errc(
                1 as libc::c_int,
                0 as libc::c_int,
                b"No memory\x00" as *const u8 as *const libc::c_char,
            );
        }
        /* Reverse filter sequence. */
        i = 0 as libc::c_int;
        r = (*cset).filter_count;
        while r > 0 as libc::c_int {
            r -= 1;
            let fresh1 = i;
            i = i + 1;
            *v.offset(fresh1 as isize) = *(*cset).filters.offset(r as isize)
        }
        free((*cset).filters as *mut libc::c_void);
        (*cset).filters = v;
        return 1 as libc::c_int;
    } else {
        /* Put previous filters back. */
        (*cset).filters = old_filters;
        (*cset).filter_count = old_filter_count;
        return 0 as libc::c_int;
    };
}
