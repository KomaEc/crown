
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    fn rename(__old: *const std::os::raw::c_char, __new: *const std::os::raw::c_char)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn fflush(__stream: *mut FILE) -> std::os::raw::c_int;
    #[no_mangle]
    fn fread(_: *mut std::os::raw::c_void, _: std::os::raw::c_ulong, _: std::os::raw::c_ulong,
             _: *mut FILE) -> std::os::raw::c_ulong;
    #[no_mangle]
    fn fwrite(_: *const std::os::raw::c_void, _: std::os::raw::c_ulong, _: std::os::raw::c_ulong,
              _: *mut FILE) -> std::os::raw::c_ulong;
    #[no_mangle]
    fn fseek(__stream: *mut FILE, __off: std::os::raw::c_long, __whence: std::os::raw::c_int)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn ftell(__stream: *mut FILE) -> std::os::raw::c_long;
    #[no_mangle]
    fn fgetpos(__stream: *mut FILE, __pos: *mut fpos_t) -> std::os::raw::c_int;
    #[no_mangle]
    fn fsetpos(__stream: *mut FILE, __pos: *const fpos_t) -> std::os::raw::c_int;
    #[no_mangle]
    fn fileno(__stream: *mut FILE) -> std::os::raw::c_int;
    #[no_mangle]
    fn strpbrk(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char)
     -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn strcpy(_: *mut std::os::raw::c_char, _: *const std::os::raw::c_char)
     -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn strchr(_: *const std::os::raw::c_char, _: std::os::raw::c_int) -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn strlen(_: *const std::os::raw::c_char) -> std::os::raw::c_ulong;
    #[no_mangle]
    fn strcat(_: *mut std::os::raw::c_char, _: *const std::os::raw::c_char)
     -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn access(__name: *const std::os::raw::c_char, __type: std::os::raw::c_int)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn chown(__file: *const std::os::raw::c_char, __owner: __uid_t, __group: __gid_t)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn unlink(__name: *const std::os::raw::c_char) -> std::os::raw::c_int;
    #[no_mangle]
    fn stat(__file: *const std::os::raw::c_char, __buf: *mut stat) -> std::os::raw::c_int;
    #[no_mangle]
    fn fstat(__fd: std::os::raw::c_int, __buf: *mut stat) -> std::os::raw::c_int;
    #[no_mangle]
    fn chmod(__file: *const std::os::raw::c_char, __mode: __mode_t) -> std::os::raw::c_int;
    #[no_mangle]
    fn mkdir(__path: *const std::os::raw::c_char, __mode: __mode_t) -> std::os::raw::c_int;
    #[no_mangle]
    fn utimensat(__fd: std::os::raw::c_int, __path: *const std::os::raw::c_char,
                 __times: *const timespec, __flags: std::os::raw::c_int)
     -> std::os::raw::c_int;
}
pub type size_t = std::os::raw::c_ulong;
pub type __dev_t = std::os::raw::c_ulong;
pub type __uid_t = std::os::raw::c_uint;
pub type __gid_t = std::os::raw::c_uint;
pub type __ino_t = std::os::raw::c_ulong;
pub type __mode_t = std::os::raw::c_uint;
pub type __nlink_t = std::os::raw::c_ulong;
pub type __off_t = std::os::raw::c_long;
pub type __off64_t = std::os::raw::c_long;
pub type __time_t = std::os::raw::c_long;
pub type __blksize_t = std::os::raw::c_long;
pub type __blkcnt_t = std::os::raw::c_long;
pub type __syscall_slong_t = std::os::raw::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __mbstate_t {
    pub __count: std::os::raw::c_int,
    pub __value: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __wch: std::os::raw::c_uint,
    pub __wchb: [std::os::raw::c_char; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _G_fpos_t {
    pub __pos: __off_t,
    pub __state: __mbstate_t,
}
pub type __fpos_t = _G_fpos_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: std::os::raw::c_int,
    pub _IO_read_ptr: *mut std::os::raw::c_char,
    pub _IO_read_end: *mut std::os::raw::c_char,
    pub _IO_read_base: *mut std::os::raw::c_char,
    pub _IO_write_base: *mut std::os::raw::c_char,
    pub _IO_write_ptr: *mut std::os::raw::c_char,
    pub _IO_write_end: *mut std::os::raw::c_char,
    pub _IO_buf_base: *mut std::os::raw::c_char,
    pub _IO_buf_end: *mut std::os::raw::c_char,
    pub _IO_save_base: *mut std::os::raw::c_char,
    pub _IO_backup_base: *mut std::os::raw::c_char,
    pub _IO_save_end: *mut std::os::raw::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: std::os::raw::c_int,
    pub _flags2: std::os::raw::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: std::os::raw::c_ushort,
    pub _vtable_offset: std::os::raw::c_schar,
    pub _shortbuf: [std::os::raw::c_char; 1],
    pub _lock: *mut std::os::raw::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut std::os::raw::c_void,
    pub __pad5: size_t,
    pub _mode: std::os::raw::c_int,
    pub _unused2: [std::os::raw::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type fpos_t = __fpos_t;
pub type opng_foffset_t = std::os::raw::c_long;
pub type opng_fsize_t = std::os::raw::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: std::os::raw::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
/*
 * Utility macros.
 */
/*
 * Returns the current value of the file position indicator.
 */
#[no_mangle]
pub unsafe extern "C" fn opng_ftello(mut stream: *mut FILE)
 -> opng_foffset_t {
    /* generic */
    return ftell(stream);
}
/*
 * Sets the file position indicator at the specified file offset.
 */
#[no_mangle]
pub unsafe extern "C" fn opng_fseeko(mut stream: *mut FILE,
                                     mut offset: opng_foffset_t,
                                     mut whence: std::os::raw::c_int) -> std::os::raw::c_int {
    return fseek(stream, offset, whence);
    /* generic */
}
/*
 * Reads a block of data from the specified file offset.
 */
#[no_mangle]
pub unsafe extern "C" fn opng_freado(mut stream: *mut FILE,
                                     mut offset: opng_foffset_t,
                                     mut whence: std::os::raw::c_int,
                                     mut block: *mut std::os::raw::c_void,
                                     mut blocksize: size_t) -> size_t {
    let mut pos: fpos_t =
        fpos_t{__pos: 0,
               __state:
                   __mbstate_t{__count: 0,
                               __value: C2RustUnnamed{__wch: 0,},},};
    let mut result: size_t = 0;
    if fgetpos(stream, &mut pos) != 0 as std::os::raw::c_int {
        return 0 as std::os::raw::c_int as size_t
    }
    if opng_fseeko(stream, offset, whence) == 0 as std::os::raw::c_int {
        result =
            fread(block, 1 as std::os::raw::c_int as std::os::raw::c_ulong, blocksize, stream)
    } else { result = 0 as std::os::raw::c_int as size_t }
    if fsetpos(stream, &mut pos) != 0 as std::os::raw::c_int {
        result = 0 as std::os::raw::c_int as size_t
    }
    return result;
}
/*
 * Writes a block of data at the specified file offset.
 */
#[no_mangle]
pub unsafe extern "C" fn opng_fwriteo(mut stream: *mut FILE,
                                      mut offset: opng_foffset_t,
                                      mut whence: std::os::raw::c_int,
                                      mut block: *const std::os::raw::c_void,
                                      mut blocksize: size_t) -> size_t {
    let mut pos: fpos_t =
        fpos_t{__pos: 0,
               __state:
                   __mbstate_t{__count: 0,
                               __value: C2RustUnnamed{__wch: 0,},},};
    let mut result: size_t = 0;
    if fgetpos(stream, &mut pos) != 0 as std::os::raw::c_int ||
           fflush(stream) != 0 as std::os::raw::c_int {
        return 0 as std::os::raw::c_int as size_t
    }
    if opng_fseeko(stream, offset, whence) == 0 as std::os::raw::c_int {
        result =
            fwrite(block, 1 as std::os::raw::c_int as std::os::raw::c_ulong, blocksize,
                   stream)
    } else { result = 0 as std::os::raw::c_int as size_t }
    if fflush(stream) != 0 as std::os::raw::c_int {
        result = 0 as std::os::raw::c_int as size_t
    }
    if fsetpos(stream, &mut pos) != 0 as std::os::raw::c_int {
        result = 0 as std::os::raw::c_int as size_t
    }
    return result;
}
/*
 * Gets the size of the specified file stream.
 */
#[no_mangle]
pub unsafe extern "C" fn opng_fgetsize(mut stream: *mut FILE,
                                       mut size: *mut opng_fsize_t)
 -> std::os::raw::c_int {
    let mut sbuf: stat =
        stat{st_dev: 0,
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
             st_atim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_mtim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_ctim: timespec{tv_sec: 0, tv_nsec: 0,},
             __glibc_reserved: [0; 3],};
    if fstat(fileno(stream), &mut sbuf) != 0 as std::os::raw::c_int {
        return -(1 as std::os::raw::c_int)
    }
    if sbuf.st_size < 0 as std::os::raw::c_int as std::os::raw::c_long {
        return -(1 as std::os::raw::c_int)
    }
    *size = sbuf.st_size as opng_fsize_t;
    return 0 as std::os::raw::c_int;
    /* generic */
}
/*
 * Makes a new path name by replacing the directory component of
 * a specified path name.
 */
#[no_mangle]
pub unsafe extern "C" fn opng_path_replace_dir(mut buffer: *mut std::os::raw::c_char,
                                               mut bufsize: size_t,
                                               mut old_path:
                                                   *const std::os::raw::c_char,
                                               mut new_dirname:
                                                   *const std::os::raw::c_char)
 -> *mut std::os::raw::c_char {
    let mut path: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut ptr: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut dirlen: size_t = 0;
    /* Extract file name from old_path. */
    path = old_path;
    loop  {
        ptr = strpbrk(path, b"/\x00" as *const u8 as *const std::os::raw::c_char);
        if ptr.is_null() { break ; }
        path = ptr.offset(1 as std::os::raw::c_int as isize)
    }
    /* Make sure the buffer is large enough. */
    dirlen = strlen(new_dirname);
    if dirlen.wrapping_add(strlen(path)).wrapping_add(2 as std::os::raw::c_int as
                                                          std::os::raw::c_ulong) >=
           bufsize {
        /* overflow */
        return 0 as *mut std::os::raw::c_char
    }
    /* Copy the new directory name. Also append a slash if necessary. */
    if dirlen > 0 as std::os::raw::c_int as std::os::raw::c_ulong {
        strcpy(buffer, new_dirname);
        if strchr(b"/\x00" as *const u8 as *const std::os::raw::c_char,
                  *buffer.offset(dirlen.wrapping_sub(1 as std::os::raw::c_int as
                                                         std::os::raw::c_ulong) as
                                     isize) as std::os::raw::c_int).is_null() {
            let fresh0 = dirlen;
            dirlen = dirlen.wrapping_add(1);
            *buffer.offset(fresh0 as isize) = '/' as i32 as std::os::raw::c_char
        }
    }
    /* Append the file name. */
    strcpy(buffer.offset(dirlen as isize), path);
    return buffer;
}
/*
 * Makes a new path name by changing the extension component of
 * a specified path name.
 */
#[no_mangle]
pub unsafe extern "C" fn opng_path_replace_ext(mut buffer: *mut std::os::raw::c_char,
                                               mut bufsize: size_t,
                                               mut old_path:
                                                   *const std::os::raw::c_char,
                                               mut new_extname:
                                                   *const std::os::raw::c_char)
 -> *mut std::os::raw::c_char {
    let mut i: size_t = 0;
    let mut pos: size_t = 0;
    if *new_extname.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int !=
           '.' as i32 {
        /* invalid argument */
        return 0 as *mut std::os::raw::c_char
    }
    i = 0 as std::os::raw::c_int as size_t;
    pos = -(1 as std::os::raw::c_int) as size_t;
    while *old_path.offset(i as isize) as std::os::raw::c_int != '\u{0}' as i32 {
        if i >= bufsize {
            /* overflow */
            return 0 as *mut std::os::raw::c_char
        }
        let ref mut fresh1 = *buffer.offset(i as isize);
        *fresh1 = *old_path.offset(i as isize);
        if *fresh1 as std::os::raw::c_int == '.' as i32 { pos = i }
        i = i.wrapping_add(1)
    }
    if i > pos {
        /* An extension already exists in old_path. Go back. */
        i = pos
    }
    loop  {
        if i >= bufsize {
            /* overflow */
            return 0 as *mut std::os::raw::c_char
        }
        let ref mut fresh2 = *buffer.offset(i as isize);
        *fresh2 = *new_extname;
        if *fresh2 as std::os::raw::c_int == '\u{0}' as i32 { return buffer }
        i = i.wrapping_add(1);
        new_extname = new_extname.offset(1)
    };
}
/*
 * Makes a backup path name.
 */
#[no_mangle]
pub unsafe extern "C" fn opng_path_make_backup(mut buffer: *mut std::os::raw::c_char,
                                               mut bufsize: size_t,
                                               mut path: *const std::os::raw::c_char)
 -> *mut std::os::raw::c_char {
    static mut bak_extname: [std::os::raw::c_char; 5] =
        unsafe {
            *::std::mem::transmute::<&[u8; 5],
                                     &[std::os::raw::c_char; 5]>(b".bak\x00")
        };
    if strlen(path).wrapping_add(::std::mem::size_of::<[std::os::raw::c_char; 5]>() as
                                     std::os::raw::c_ulong) > bufsize {
        return 0 as *mut std::os::raw::c_char
    }
    /* OPNG_OS_UNIX and others */
    strcpy(buffer, path);
    strcat(buffer, bak_extname.as_ptr());
    return buffer;
}
/*
 * Changes the name of a file system object.
 */
#[no_mangle]
pub unsafe extern "C" fn opng_os_rename(mut src_path: *const std::os::raw::c_char,
                                        mut dest_path: *const std::os::raw::c_char,
                                        mut clobber: std::os::raw::c_int)
 -> std::os::raw::c_int {
    if clobber == 0 {
        if access(dest_path, 0 as std::os::raw::c_int) >= 0 as std::os::raw::c_int {
            return -(1 as std::os::raw::c_int)
        }
    }
    return rename(src_path, dest_path);
    /* generic */
}
/*
 * Copies the attributes (access mode, time stamp, etc.) of a file system
 * object.
 */
#[no_mangle]
pub unsafe extern "C" fn opng_os_copy_attr(mut src_path: *const std::os::raw::c_char,
                                           mut dest_path: *const std::os::raw::c_char)
 -> std::os::raw::c_int {
    let mut sbuf: stat =
        stat{st_dev: 0,
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
             st_atim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_mtim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_ctim: timespec{tv_sec: 0, tv_nsec: 0,},
             __glibc_reserved: [0; 3],};
    let mut result: std::os::raw::c_int = 0;
    if stat(src_path, &mut sbuf) != 0 as std::os::raw::c_int {
        return -(1 as std::os::raw::c_int)
    }
    result = 0 as std::os::raw::c_int;
    (chown(dest_path, sbuf.st_uid, sbuf.st_gid)) != 0 as std::os::raw::c_int;
    if chmod(dest_path, sbuf.st_mode) != 0 as std::os::raw::c_int {
        result = -(1 as std::os::raw::c_int)
    }
    let mut times: [timespec; 2] = [timespec{tv_sec: 0, tv_nsec: 0,}; 2];
    times[0 as std::os::raw::c_int as usize] = sbuf.st_atim;
    times[1 as std::os::raw::c_int as usize] = sbuf.st_mtim;
    if utimensat(-(100 as std::os::raw::c_int), dest_path,
                 times.as_mut_ptr() as *const timespec, 0 as std::os::raw::c_int) !=
           0 as std::os::raw::c_int {
        result = -(1 as std::os::raw::c_int)
    }
    /* legacy utime */
    return result;
    /* generic */
}
/*
 * Creates a new directory.
 */
#[no_mangle]
pub unsafe extern "C" fn opng_os_create_dir(mut dirname: *const std::os::raw::c_char)
 -> std::os::raw::c_int {
    /* Exit early if there is no directory name. */
    if *dirname.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
           '\u{0}' as i32 {
        return 0 as std::os::raw::c_int
    }
    let mut sbuf: stat =
        stat{st_dev: 0,
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
             st_atim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_mtim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_ctim: timespec{tv_sec: 0, tv_nsec: 0,},
             __glibc_reserved: [0; 3],};
    if stat(dirname, &mut sbuf) == 0 as std::os::raw::c_int {
        return if sbuf.st_mode & 0o40000 as std::os::raw::c_int as std::os::raw::c_uint != 0 {
                   0 as std::os::raw::c_int
               } else { -(1 as std::os::raw::c_int) }
    }
    /* There is no directory, so create one now. */
    return mkdir(dirname, 0o777 as std::os::raw::c_int as __mode_t);
    /* generic */
}
/*
 * Determines if the accessibility of the specified file system object
 * satisfies the specified access mode.
 */
#[no_mangle]
pub unsafe extern "C" fn opng_os_test(mut path: *const std::os::raw::c_char,
                                      mut mode: *const std::os::raw::c_char)
 -> std::os::raw::c_int {
    let mut faccess: std::os::raw::c_int = 0;
    let mut freg: std::os::raw::c_int = 0;
    freg = 0 as std::os::raw::c_int;
    faccess = freg;
    if !strchr(mode, 'f' as i32).is_null() { freg = 1 as std::os::raw::c_int }
    if !strchr(mode, 'r' as i32).is_null() { faccess |= 4 as std::os::raw::c_int }
    if !strchr(mode, 'w' as i32).is_null() { faccess |= 2 as std::os::raw::c_int }
    if !strchr(mode, 'x' as i32).is_null() { faccess |= 1 as std::os::raw::c_int }
    if faccess == 0 as std::os::raw::c_int && freg == 0 {
        if strchr(mode, 'e' as i32).is_null() { return 0 as std::os::raw::c_int }
    }
    let mut sbuf: stat =
        stat{st_dev: 0,
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
             st_atim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_mtim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_ctim: timespec{tv_sec: 0, tv_nsec: 0,},
             __glibc_reserved: [0; 3],};
    if stat(path, &mut sbuf) != 0 as std::os::raw::c_int {
        return -(1 as std::os::raw::c_int)
    }
    if freg != 0 &&
           sbuf.st_mode & 0o100000 as std::os::raw::c_int as std::os::raw::c_uint !=
               0o100000 as std::os::raw::c_int as std::os::raw::c_uint {
        return -(1 as std::os::raw::c_int)
    }
    if faccess == 0 as std::os::raw::c_int { return 0 as std::os::raw::c_int }
    return access(path, faccess);
    /* generic */
}
/*
 * Determines if two accessible paths are equivalent, i.e. they
 * refer to the same file system object.
 */
#[no_mangle]
pub unsafe extern "C" fn opng_os_test_eq(mut path1: *const std::os::raw::c_char,
                                         mut path2: *const std::os::raw::c_char)
 -> std::os::raw::c_int {
    let mut sbuf1: stat =
        stat{st_dev: 0,
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
             st_atim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_mtim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_ctim: timespec{tv_sec: 0, tv_nsec: 0,},
             __glibc_reserved: [0; 3],};
    let mut sbuf2: stat =
        stat{st_dev: 0,
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
             st_atim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_mtim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_ctim: timespec{tv_sec: 0, tv_nsec: 0,},
             __glibc_reserved: [0; 3],};
    if stat(path1, &mut sbuf1) != 0 as std::os::raw::c_int ||
           stat(path2, &mut sbuf2) != 0 as std::os::raw::c_int {
        /* Can't stat the paths. */
        return -(1 as std::os::raw::c_int)
    }
    if sbuf1.st_dev == sbuf2.st_dev && sbuf1.st_ino == sbuf2.st_ino {
        /* The two paths have the same device and inode numbers. */
        /* The inode numbers are reliable only if they're not 0. */
        return if sbuf1.st_ino != 0 as std::os::raw::c_int as std::os::raw::c_ulong {
                   1 as std::os::raw::c_int
               } else { -(1 as std::os::raw::c_int) }
    } else {
        /* The two paths have different device or inode numbers. */
        return 0 as std::os::raw::c_int
    };
    /* generic */
}
/*
 * Removes a directory entry.
 */
#[no_mangle]
pub unsafe extern "C" fn opng_os_unlink(mut path: *const std::os::raw::c_char)
 -> std::os::raw::c_int {
    return unlink(path);
    /* generic */
}
