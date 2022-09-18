use ::libc;
extern "C" {
    pub type archive;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    #[no_mangle]
    fn archive_write_open(
        _: *mut archive,
        _: *mut libc::c_void,
        _: Option<archive_open_callback>,
        _: Option<archive_write_callback>,
        _: Option<archive_close_callback>,
    ) -> libc::c_int;
    #[no_mangle]
    fn archive_set_error(_: *mut archive, _err: libc::c_int, fmt: *const libc::c_char, _: ...);
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
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
pub type la_ssize_t = ssize_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct write_FILE_data {
    pub f: *mut FILE,
}
pub const errno: libc::c_int = *__errno_location();
pub const EINTR: libc::c_int = 4 as libc::c_int;
pub const ENOMEM: libc::c_int = 12 as libc::c_int;
pub const NULL: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn archive_write_open_FILE(
    mut a: *mut archive,
    mut f: *mut FILE,
) -> libc::c_int {
    let mut mine: *mut write_FILE_data = 0 as *mut write_FILE_data;
    mine =
        malloc(::std::mem::size_of::<write_FILE_data>() as libc::c_ulong) as *mut write_FILE_data;
    if mine.is_null() {
        archive_set_error(
            a,
            ENOMEM,
            b"No memory\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    (*mine).f = f;
    return archive_write_open(
        a,
        mine as *mut libc::c_void,
        Some(
            file_open as unsafe extern "C" fn(_: *mut archive, _: *mut libc::c_void) -> libc::c_int,
        ),
        Some(
            file_write
                as unsafe extern "C" fn(
                    _: *mut archive,
                    _: *mut libc::c_void,
                    _: *const libc::c_void,
                    _: size_t,
                ) -> ssize_t,
        ),
        Some(
            file_close
                as unsafe extern "C" fn(_: *mut archive, _: *mut libc::c_void) -> libc::c_int,
        ),
    );
}
unsafe extern "C" fn file_open(
    mut a: *mut archive,
    mut client_data: *mut libc::c_void,
) -> libc::c_int {
    /* UNUSED */
    return 0 as libc::c_int;
}
unsafe extern "C" fn file_write(
    mut a: *mut archive,
    mut client_data: *mut libc::c_void,
    mut buff: *const libc::c_void,
    mut length: size_t,
) -> ssize_t {
    let mut mine: *mut write_FILE_data = 0 as *mut write_FILE_data;
    let mut bytesWritten: size_t = 0;
    mine = client_data as *mut write_FILE_data;
    loop {
        bytesWritten = fwrite(buff, 1 as libc::c_int as libc::c_ulong, length, (*mine).f);
        if bytesWritten <= 0 as libc::c_int as libc::c_ulong {
            if errno == EINTR {
                continue;
            }
            archive_set_error(
                a,
                errno,
                b"Write error\x00" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int) as ssize_t;
        } else {
            return bytesWritten as ssize_t;
        }
    }
}
unsafe extern "C" fn file_close(
    mut a: *mut archive,
    mut client_data: *mut libc::c_void,
) -> libc::c_int {
    let mut mine: *mut write_FILE_data = client_data as *mut write_FILE_data;
    /* UNUSED */
    free(mine as *mut libc::c_void);
    return 0 as libc::c_int;
}
