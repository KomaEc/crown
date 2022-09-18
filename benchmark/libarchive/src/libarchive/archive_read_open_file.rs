use ::libc;
extern "C" {
    pub type archive;
    #[no_mangle]
    fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    #[no_mangle]
    fn fseeko(__stream: *mut FILE, __off: __off_t, __whence: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn archive_read_set_read_callback(
        _: *mut archive,
        _: Option<archive_read_callback>,
    ) -> libc::c_int;
    #[no_mangle]
    fn archive_read_set_skip_callback(
        _: *mut archive,
        _: Option<archive_skip_callback>,
    ) -> libc::c_int;
    #[no_mangle]
    fn archive_read_set_close_callback(
        _: *mut archive,
        _: Option<archive_close_callback>,
    ) -> libc::c_int;
    #[no_mangle]
    fn archive_read_set_callback_data(_: *mut archive, _: *mut libc::c_void) -> libc::c_int;
    #[no_mangle]
    fn archive_read_open1(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_read_extract_set_skip_file(_: *mut archive, _: la_int64_t, _: la_int64_t);
    #[no_mangle]
    fn archive_set_error(_: *mut archive, _err: libc::c_int, fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn archive_clear_error(_: *mut archive);
}
pub type __int64_t = libc::c_long;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type int64_t = __int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type off_t = __off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
pub type size_t = libc::c_ulong;
pub type ssize_t = __ssize_t;
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
    pub __pad1: *mut libc::c_void,
    pub __pad2: *mut libc::c_void,
    pub __pad3: *mut libc::c_void,
    pub __pad4: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
pub type FILE = _IO_FILE;
pub type la_int64_t = int64_t;
pub type la_ssize_t = ssize_t;
pub type archive_read_callback = unsafe extern "C" fn(
    _: *mut archive,
    _: *mut libc::c_void,
    _: *mut *const libc::c_void,
) -> la_ssize_t;
pub type archive_skip_callback =
    unsafe extern "C" fn(_: *mut archive, _: *mut libc::c_void, _: la_int64_t) -> la_int64_t;
pub type archive_close_callback =
    unsafe extern "C" fn(_: *mut archive, _: *mut libc::c_void) -> libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct read_FILE_data {
    pub f: *mut FILE,
    pub block_size: size_t,
    pub buffer: *mut libc::c_void,
    pub can_skip: libc::c_char,
}
pub const __S_IFMT: libc::c_int = 0o170000 as libc::c_int;
pub const errno: libc::c_int = *__errno_location();
pub const ENOMEM: libc::c_int = 12 as libc::c_int;
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const SEEK_CUR: libc::c_int = 1 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn archive_read_open_FILE(
    mut a: *mut archive,
    mut f: *mut FILE,
) -> libc::c_int {
    let mut st: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    let mut mine: *mut read_FILE_data = 0 as *mut read_FILE_data;
    let mut block_size: size_t = (128 as libc::c_int * 1024 as libc::c_int) as size_t;
    let mut b: *mut libc::c_void = 0 as *mut libc::c_void;
    archive_clear_error(a);
    mine = malloc(::std::mem::size_of::<read_FILE_data>() as libc::c_ulong) as *mut read_FILE_data;
    b = malloc(block_size);
    if mine.is_null() || b.is_null() {
        archive_set_error(
            a,
            ENOMEM,
            b"No memory\x00" as *const u8 as *const libc::c_char,
        );
        free(mine as *mut libc::c_void);
        free(b);
        return -(30 as libc::c_int);
    }
    (*mine).block_size = block_size;
    (*mine).buffer = b;
    (*mine).f = f;
    /*
     * If we can't fstat() the file, it may just be that it's not
     * a file.  (On some platforms, FILE * objects can wrap I/O
     * streams that don't support fileno()).  As a result, fileno()
     * should be used cautiously.)
     */
    if fstat(fileno((*mine).f), &mut st) == 0 as libc::c_int
        && st.st_mode & __S_IFMT as libc::c_uint == 0o100000 as libc::c_int as libc::c_uint
    {
        archive_read_extract_set_skip_file(a, st.st_dev as la_int64_t, st.st_ino as la_int64_t);
        /* Enable the seek optimization only for regular files. */
        (*mine).can_skip = 1 as libc::c_int as libc::c_char
    } else {
        (*mine).can_skip = 0 as libc::c_int as libc::c_char
    }
    archive_read_set_read_callback(
        a,
        Some(
            file_read
                as unsafe extern "C" fn(
                    _: *mut archive,
                    _: *mut libc::c_void,
                    _: *mut *const libc::c_void,
                ) -> ssize_t,
        ),
    );
    archive_read_set_skip_callback(
        a,
        Some(
            file_skip
                as unsafe extern "C" fn(
                    _: *mut archive,
                    _: *mut libc::c_void,
                    _: int64_t,
                ) -> int64_t,
        ),
    );
    archive_read_set_close_callback(
        a,
        Some(
            file_close
                as unsafe extern "C" fn(_: *mut archive, _: *mut libc::c_void) -> libc::c_int,
        ),
    );
    archive_read_set_callback_data(a, mine as *mut libc::c_void);
    return archive_read_open1(a);
}
unsafe extern "C" fn file_read(
    mut a: *mut archive,
    mut client_data: *mut libc::c_void,
    mut buff: *mut *const libc::c_void,
) -> ssize_t {
    let mut mine: *mut read_FILE_data = client_data as *mut read_FILE_data;
    let mut bytes_read: size_t = 0;
    *buff = (*mine).buffer;
    bytes_read = fread(
        (*mine).buffer,
        1 as libc::c_int as libc::c_ulong,
        (*mine).block_size,
        (*mine).f,
    );
    if bytes_read < (*mine).block_size && ferror((*mine).f) != 0 {
        archive_set_error(
            a,
            errno,
            b"Error reading file\x00" as *const u8 as *const libc::c_char,
        );
    }
    return bytes_read as ssize_t;
}
unsafe extern "C" fn file_skip(
    mut a: *mut archive,
    mut client_data: *mut libc::c_void,
    mut request: int64_t,
) -> int64_t {
    let mut mine: *mut read_FILE_data = client_data as *mut read_FILE_data;
    let mut skip: off_t = request;
    let mut skip_bits: libc::c_int = (::std::mem::size_of::<off_t>() as libc::c_ulong)
        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        as libc::c_int;
    /* UNUSED */
    /*
     * If we can't skip, return 0 as the amount we did step and
     * the caller will work around by reading and discarding.
     */
    if (*mine).can_skip == 0 {
        return 0 as libc::c_int as int64_t;
    }
    if request == 0 as libc::c_int as libc::c_long {
        return 0 as libc::c_int as int64_t;
    }
    /* If request is too big for a long or an off_t, reduce it. */
    if ::std::mem::size_of::<int64_t>() as libc::c_ulong
        > ::std::mem::size_of::<off_t>() as libc::c_ulong
    {
        let mut max_skip: int64_t = (((1 as libc::c_int as int64_t)
            << skip_bits - 1 as libc::c_int)
            - 1 as libc::c_int as libc::c_long)
            * 2 as libc::c_int as libc::c_long
            + 1 as libc::c_int as libc::c_long;
        if request > max_skip {
            skip = max_skip
        }
    }
    if fseeko((*mine).f, skip, SEEK_CUR) != 0 as libc::c_int {
        (*mine).can_skip = 0 as libc::c_int as libc::c_char;
        return 0 as libc::c_int as int64_t;
    }
    return request;
}
unsafe extern "C" fn file_close(
    mut a: *mut archive,
    mut client_data: *mut libc::c_void,
) -> libc::c_int {
    let mut mine: *mut read_FILE_data = client_data as *mut read_FILE_data;
    /* UNUSED */
    free((*mine).buffer);
    free(mine as *mut libc::c_void);
    return 0 as libc::c_int;
}
