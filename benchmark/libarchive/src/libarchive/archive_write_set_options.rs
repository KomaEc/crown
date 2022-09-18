use ::libc;
extern "C" {
    pub type archive_string_conv;
    /* Declare our basic types. */
    pub type archive_entry;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn _archive_set_option(
        a: *mut archive,
        mod_0: *const libc::c_char,
        opt: *const libc::c_char,
        val: *const libc::c_char,
        magic: libc::c_int,
        fn_0: *const libc::c_char,
        use_option: option_handler,
    ) -> libc::c_int;
    #[no_mangle]
    fn _archive_set_options(
        a: *mut archive,
        options: *const libc::c_char,
        magic: libc::c_int,
        fn_0: *const libc::c_char,
        use_option: option_handler,
    ) -> libc::c_int;
    #[no_mangle]
    fn _archive_set_either_option(
        a: *mut archive,
        m: *const libc::c_char,
        o: *const libc::c_char,
        v: *const libc::c_char,
        use_format_option: option_handler,
        use_filter_option: option_handler,
    ) -> libc::c_int;
}
pub type __int64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type int64_t = __int64_t;
pub type size_t = libc::c_ulong;
pub type ssize_t = __ssize_t;
/* The la_ssize_t should match the type used in 'struct stat' */
/* Older code relied on the __LA_SSIZE_T macro; after 4.0 we'll switch to the typedef exclusively. */
/* ssize_t */
pub type la_ssize_t = ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive {
    pub magic: libc::c_uint,
    pub state: libc::c_uint,
    pub vtable: *mut archive_vtable,
    pub archive_format: libc::c_int,
    pub archive_format_name: *const libc::c_char,
    pub compression_code: libc::c_int,
    pub compression_name: *const libc::c_char,
    pub file_count: libc::c_int,
    pub archive_error_number: libc::c_int,
    pub error: *const libc::c_char,
    pub error_string: archive_string,
    pub current_code: *mut libc::c_char,
    pub current_codepage: libc::c_uint,
    pub current_oemcp: libc::c_uint,
    pub sconv: *mut archive_string_conv,
    pub read_data_block: *const libc::c_char,
    pub read_data_offset: int64_t,
    pub read_data_output_offset: int64_t,
    pub read_data_remaining: size_t,
    pub read_data_is_posix_read: libc::c_char,
    pub read_data_requested: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_string {
    pub s: *mut libc::c_char,
    pub length: size_t,
    pub buffer_length: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_vtable {
    pub archive_close: Option<unsafe extern "C" fn(_: *mut archive) -> libc::c_int>,
    pub archive_free: Option<unsafe extern "C" fn(_: *mut archive) -> libc::c_int>,
    pub archive_write_header:
        Option<unsafe extern "C" fn(_: *mut archive, _: *mut archive_entry) -> libc::c_int>,
    pub archive_write_finish_entry: Option<unsafe extern "C" fn(_: *mut archive) -> libc::c_int>,
    pub archive_write_data:
        Option<unsafe extern "C" fn(_: *mut archive, _: *const libc::c_void, _: size_t) -> ssize_t>,
    pub archive_write_data_block: Option<
        unsafe extern "C" fn(
            _: *mut archive,
            _: *const libc::c_void,
            _: size_t,
            _: int64_t,
        ) -> ssize_t,
    >,
    pub archive_read_next_header:
        Option<unsafe extern "C" fn(_: *mut archive, _: *mut *mut archive_entry) -> libc::c_int>,
    pub archive_read_next_header2:
        Option<unsafe extern "C" fn(_: *mut archive, _: *mut archive_entry) -> libc::c_int>,
    pub archive_read_data_block: Option<
        unsafe extern "C" fn(
            _: *mut archive,
            _: *mut *const libc::c_void,
            _: *mut size_t,
            _: *mut int64_t,
        ) -> libc::c_int,
    >,
    pub archive_filter_count: Option<unsafe extern "C" fn(_: *mut archive) -> libc::c_int>,
    pub archive_filter_bytes:
        Option<unsafe extern "C" fn(_: *mut archive, _: libc::c_int) -> int64_t>,
    pub archive_filter_code:
        Option<unsafe extern "C" fn(_: *mut archive, _: libc::c_int) -> libc::c_int>,
    pub archive_filter_name:
        Option<unsafe extern "C" fn(_: *mut archive, _: libc::c_int) -> *const libc::c_char>,
}
/* Returns size actually written, zero on EOF, -1 on error. */
pub type archive_write_callback = unsafe extern "C" fn(
    _: *mut archive,
    _: *mut libc::c_void,
    _: *const libc::c_void,
    _: size_t,
) -> la_ssize_t;
pub type archive_open_callback =
    unsafe extern "C" fn(_: *mut archive, _: *mut libc::c_void) -> libc::c_int;
pub type archive_close_callback =
    unsafe extern "C" fn(_: *mut archive, _: *mut libc::c_void) -> libc::c_int;
/*
 * Returns a passphrase used for encryption or decryption, NULL on nothing
 * to do and give it up.
 */
pub type archive_passphrase_callback =
    unsafe extern "C" fn(_: *mut archive, _: *mut libc::c_void) -> *const libc::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_write {
    pub archive: archive,
    pub skip_file_set: libc::c_int,
    pub skip_file_dev: int64_t,
    pub skip_file_ino: int64_t,
    pub nulls: *const libc::c_uchar,
    pub null_length: size_t,
    pub client_opener: Option<archive_open_callback>,
    pub client_writer: Option<archive_write_callback>,
    pub client_closer: Option<archive_close_callback>,
    pub client_data: *mut libc::c_void,
    pub bytes_per_block: libc::c_int,
    pub bytes_in_last_block: libc::c_int,
    pub filter_first: *mut archive_write_filter,
    pub filter_last: *mut archive_write_filter,
    pub format_data: *mut libc::c_void,
    pub format_name: *const libc::c_char,
    pub format_init: Option<unsafe extern "C" fn(_: *mut archive_write) -> libc::c_int>,
    pub format_options: Option<
        unsafe extern "C" fn(
            _: *mut archive_write,
            _: *const libc::c_char,
            _: *const libc::c_char,
        ) -> libc::c_int,
    >,
    pub format_finish_entry: Option<unsafe extern "C" fn(_: *mut archive_write) -> libc::c_int>,
    pub format_write_header:
        Option<unsafe extern "C" fn(_: *mut archive_write, _: *mut archive_entry) -> libc::c_int>,
    pub format_write_data: Option<
        unsafe extern "C" fn(_: *mut archive_write, _: *const libc::c_void, _: size_t) -> ssize_t,
    >,
    pub format_close: Option<unsafe extern "C" fn(_: *mut archive_write) -> libc::c_int>,
    pub format_free: Option<unsafe extern "C" fn(_: *mut archive_write) -> libc::c_int>,
    pub passphrase: *mut libc::c_char,
    pub passphrase_callback: Option<archive_passphrase_callback>,
    pub passphrase_client_data: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_write_filter {
    pub bytes_written: int64_t,
    pub archive: *mut archive,
    pub next_filter: *mut archive_write_filter,
    pub options: Option<
        unsafe extern "C" fn(
            _: *mut archive_write_filter,
            _: *const libc::c_char,
            _: *const libc::c_char,
        ) -> libc::c_int,
    >,
    pub open: Option<unsafe extern "C" fn(_: *mut archive_write_filter) -> libc::c_int>,
    pub write: Option<
        unsafe extern "C" fn(
            _: *mut archive_write_filter,
            _: *const libc::c_void,
            _: size_t,
        ) -> libc::c_int,
    >,
    pub close: Option<unsafe extern "C" fn(_: *mut archive_write_filter) -> libc::c_int>,
    pub free: Option<unsafe extern "C" fn(_: *mut archive_write_filter) -> libc::c_int>,
    pub data: *mut libc::c_void,
    pub name: *const libc::c_char,
    pub code: libc::c_int,
    pub bytes_per_block: libc::c_int,
    pub bytes_in_last_block: libc::c_int,
    pub state: libc::c_int,
}
pub type option_handler = Option<
    unsafe extern "C" fn(
        _: *mut archive,
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: *const libc::c_char,
    ) -> libc::c_int,
>;
/*
 * Error codes: Use archive_errno() and archive_error_string()
 * to retrieve details.  Unless specified otherwise, all functions
 * that return 'int' use these codes.
 */
/* Found end of archive. */
pub const ARCHIVE_OK: libc::c_int = 0 as libc::c_int;
/* Operation was successful. */
/* Retry might succeed. */
pub const ARCHIVE_WARN: libc::c_int = -(20 as libc::c_int);
/* Partial success. */
/* For example, if write_header "fails", then you can't push data. */
pub const ARCHIVE_FAILED: libc::c_int = -(25 as libc::c_int);
/* Current operation cannot complete. */
/* But if write_header is "fatal," then this archive is dead and useless. */
pub const ARCHIVE_FATAL: libc::c_int = -(30 as libc::c_int);
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const ARCHIVE_WRITE_MAGIC: libc::c_uint = 0xb0c5c0de as libc::c_uint;
/*
 * Set write options.
 */
/* Apply option to the format only. */
#[no_mangle]
pub unsafe extern "C" fn archive_write_set_format_option(
    mut a: *mut archive,
    mut m: *const libc::c_char,
    mut o: *const libc::c_char,
    mut v: *const libc::c_char,
) -> libc::c_int {
    return _archive_set_option(
        a,
        m,
        o,
        v,
        ARCHIVE_WRITE_MAGIC as libc::c_int,
        b"archive_write_set_format_option\x00" as *const u8 as *const libc::c_char,
        Some(
            archive_set_format_option
                as unsafe extern "C" fn(
                    _: *mut archive,
                    _: *const libc::c_char,
                    _: *const libc::c_char,
                    _: *const libc::c_char,
                ) -> libc::c_int,
        ),
    );
}
/* Apply option to the filter only. */
#[no_mangle]
pub unsafe extern "C" fn archive_write_set_filter_option(
    mut a: *mut archive,
    mut m: *const libc::c_char,
    mut o: *const libc::c_char,
    mut v: *const libc::c_char,
) -> libc::c_int {
    return _archive_set_option(
        a,
        m,
        o,
        v,
        ARCHIVE_WRITE_MAGIC as libc::c_int,
        b"archive_write_set_filter_option\x00" as *const u8 as *const libc::c_char,
        Some(
            archive_set_filter_option
                as unsafe extern "C" fn(
                    _: *mut archive,
                    _: *const libc::c_char,
                    _: *const libc::c_char,
                    _: *const libc::c_char,
                ) -> libc::c_int,
        ),
    );
}
/* Apply option to both the format and the filter. */
#[no_mangle]
pub unsafe extern "C" fn archive_write_set_option(
    mut a: *mut archive,
    mut m: *const libc::c_char,
    mut o: *const libc::c_char,
    mut v: *const libc::c_char,
) -> libc::c_int {
    return _archive_set_option(
        a,
        m,
        o,
        v,
        ARCHIVE_WRITE_MAGIC as libc::c_int,
        b"archive_write_set_option\x00" as *const u8 as *const libc::c_char,
        Some(
            archive_set_option
                as unsafe extern "C" fn(
                    _: *mut archive,
                    _: *const libc::c_char,
                    _: *const libc::c_char,
                    _: *const libc::c_char,
                ) -> libc::c_int,
        ),
    );
}
/* Apply option string to both the format and the filter. */
#[no_mangle]
pub unsafe extern "C" fn archive_write_set_options(
    mut a: *mut archive,
    mut options: *const libc::c_char,
) -> libc::c_int {
    return _archive_set_options(
        a,
        options,
        ARCHIVE_WRITE_MAGIC as libc::c_int,
        b"archive_write_set_options\x00" as *const u8 as *const libc::c_char,
        Some(
            archive_set_option
                as unsafe extern "C" fn(
                    _: *mut archive,
                    _: *const libc::c_char,
                    _: *const libc::c_char,
                    _: *const libc::c_char,
                ) -> libc::c_int,
        ),
    );
}
unsafe extern "C" fn archive_set_format_option(
    mut _a: *mut archive,
    mut m: *const libc::c_char,
    mut o: *const libc::c_char,
    mut v: *const libc::c_char,
) -> libc::c_int {
    let mut a: *mut archive_write = _a as *mut archive_write;
    if (*a).format_name.is_null() {
        return if m.is_null() {
            ARCHIVE_FAILED
        } else {
            (ARCHIVE_WARN) - 1 as libc::c_int
        };
    }
    /* If the format name didn't match, return a special code for
     * _archive_set_option[s]. */
    if !m.is_null() && strcmp(m, (*a).format_name) != 0 as libc::c_int {
        return ARCHIVE_WARN - 1 as libc::c_int;
    }
    if (*a).format_options.is_none() {
        return -(20 as libc::c_int);
    }
    return (*a).format_options.expect("non-null function pointer")(a, o, v);
}
unsafe extern "C" fn archive_set_filter_option(
    mut _a: *mut archive,
    mut m: *const libc::c_char,
    mut o: *const libc::c_char,
    mut v: *const libc::c_char,
) -> libc::c_int {
    let mut a: *mut archive_write = _a as *mut archive_write;
    let mut filter: *mut archive_write_filter = 0 as *mut archive_write_filter;
    let mut r: libc::c_int = 0;
    let mut rv: libc::c_int = ARCHIVE_WARN;
    filter = (*a).filter_first;
    while !filter.is_null() {
        if !(*filter).options.is_none() {
            if !(!m.is_null() && strcmp((*filter).name, m) != 0 as libc::c_int) {
                r = (*filter).options.expect("non-null function pointer")(filter, o, v);
                if r == ARCHIVE_FATAL {
                    return -(30 as libc::c_int);
                }
                if !m.is_null() {
                    return r;
                }
                if r == ARCHIVE_OK {
                    rv = ARCHIVE_OK
                }
            }
        }
        filter = (*filter).next_filter
    }
    /* If the filter name didn't match, return a special code for
     * _archive_set_option[s]. */
    if rv == ARCHIVE_WARN && !m.is_null() {
        rv = ARCHIVE_WARN - 1 as libc::c_int
    }
    return rv;
}
unsafe extern "C" fn archive_set_option(
    mut a: *mut archive,
    mut m: *const libc::c_char,
    mut o: *const libc::c_char,
    mut v: *const libc::c_char,
) -> libc::c_int {
    return _archive_set_either_option(
        a,
        m,
        o,
        v,
        Some(
            archive_set_format_option
                as unsafe extern "C" fn(
                    _: *mut archive,
                    _: *const libc::c_char,
                    _: *const libc::c_char,
                    _: *const libc::c_char,
                ) -> libc::c_int,
        ),
        Some(
            archive_set_filter_option
                as unsafe extern "C" fn(
                    _: *mut archive,
                    _: *const libc::c_char,
                    _: *const libc::c_char,
                    _: *const libc::c_char,
                ) -> libc::c_int,
        ),
    );
}
