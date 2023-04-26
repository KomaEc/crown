use ::libc;
extern "C" {
    static mut stdout: *mut _IO_FILE;
    static mut stderr: *mut _IO_FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(__filename: *const libc::c_char, __modes: *const libc::c_char) -> *mut FILE;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn _IO_getc(__fp: *mut _IO_FILE) -> libc::c_int;
    fn _IO_putc(__c: libc::c_int, __fp: *mut _IO_FILE) -> libc::c_int;
    fn puts(__s: *const libc::c_char) -> libc::c_int;
    fn fread(
        __ptr: *mut libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn fwrite(
        __ptr: *const libc::c_void,
        __size: size_t,
        __n: size_t,
        __s: *mut FILE,
    ) -> size_t;
    fn fseek(
        __stream: *mut FILE,
        __off: libc::c_long,
        __whence: libc::c_int,
    ) -> libc::c_int;
    fn strtoul(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
        __base: libc::c_int,
    ) -> libc::c_ulong;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn glob(
        __pattern: *const libc::c_char,
        __flags: libc::c_int,
        __errfunc: Option::<
            unsafe extern "C" fn(*const libc::c_char, libc::c_int) -> libc::c_int,
        >,
        __pglob: *mut glob_t,
    ) -> libc::c_int;
    fn globfree(__pglob: *mut glob_t);
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn uname(__name: *mut utsname) -> libc::c_int;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __ino64_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type __syscall_ulong_t = libc::c_ulong;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
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
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type __size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glob_t {
    pub gl_pathc: __size_t,
    pub gl_pathv: *mut *mut libc::c_char,
    pub gl_offs: __size_t,
    pub gl_flags: libc::c_int,
    pub gl_closedir: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub gl_readdir: Option::<
        unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
    >,
    pub gl_opendir: Option::<
        unsafe extern "C" fn(*const libc::c_char) -> *mut libc::c_void,
    >,
    pub gl_lstat: Option::<
        unsafe extern "C" fn(*const libc::c_char, *mut libc::c_void) -> libc::c_int,
    >,
    pub gl_stat: Option::<
        unsafe extern "C" fn(*const libc::c_char, *mut libc::c_void) -> libc::c_int,
    >,
}
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
    pub st_atime: __time_t,
    pub st_atimensec: __syscall_ulong_t,
    pub st_mtime: __time_t,
    pub st_mtimensec: __syscall_ulong_t,
    pub st_ctime: __time_t,
    pub st_ctimensec: __syscall_ulong_t,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
pub type ino_t = __ino64_t;
pub type dev_t = __dev_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct utsname {
    pub sysname: [libc::c_char; 65],
    pub nodename: [libc::c_char; 65],
    pub release: [libc::c_char; 65],
    pub version: [libc::c_char; 65],
    pub machine: [libc::c_char; 65],
    pub __domainname: [libc::c_char; 65],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct header_64_t {
    pub e_type: uint16_t,
    pub e_machine: uint16_t,
    pub e_version: uint32_t,
    pub e_entry: uint64_t,
    pub e_phoff: uint64_t,
    pub e_shoff: uint64_t,
    pub e_flags: uint32_t,
    pub e_ehsize: uint16_t,
    pub e_phentsize: uint16_t,
    pub e_phnum: uint16_t,
    pub e_shentsize: uint16_t,
    pub e_shnum: uint16_t,
    pub e_shstrndx: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct header_32_t {
    pub e_type: uint16_t,
    pub e_machine: uint16_t,
    pub e_version: uint32_t,
    pub e_entry: uint32_t,
    pub e_phoff: uint32_t,
    pub e_shoff: uint32_t,
    pub e_flags: uint32_t,
    pub e_ehsize: uint16_t,
    pub e_phentsize: uint16_t,
    pub e_phnum: uint16_t,
    pub e_shentsize: uint16_t,
    pub e_shnum: uint16_t,
    pub e_shstrndx: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct prog_64_t {
    pub p_type: uint32_t,
    pub p_flags: uint32_t,
    pub p_offset: uint64_t,
    pub p_vaddr: uint64_t,
    pub p_paddr: uint64_t,
    pub p_filesz: uint64_t,
    pub p_memsz: uint64_t,
    pub p_align: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct prog_32_t {
    pub p_type: uint32_t,
    pub p_offset: uint32_t,
    pub p_vaddr: uint32_t,
    pub p_paddr: uint32_t,
    pub p_filesz: uint32_t,
    pub p_memsz: uint32_t,
    pub p_flags: uint32_t,
    pub p_align: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dyn_64_t {
    pub d_tag: int64_t,
    pub d_val: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dyn_32_t {
    pub d_tag: int32_t,
    pub d_val: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct compat_t {
    pub any: libc::c_char,
    pub class: uint8_t,
    pub machine: uint16_t,
}
pub type how_t = libc::c_uint;
pub const DEFAULT: how_t = 6;
pub const LD_SO_CONF: how_t = 5;
pub const RUNPATH: how_t = 4;
pub const LD_LIBRARY_PATH: how_t = 3;
pub const RPATH: how_t = 2;
pub const DIRECT: how_t = 1;
pub const INPUT: how_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct found_t {
    pub how: how_t,
    pub depth: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
struct ErasedByRefactorer0;
#[repr(C)]
pub struct string_table_t {
    pub arr: *mut /* owning */ libc::c_char,
    pub n: size_t,
    pub capacity: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct visited_file_t {
    pub st_dev: dev_t,
    pub st_ino: ino_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
struct ErasedByRefactorer1;
#[repr(C)]
pub struct visited_file_array_t {
    pub arr: *mut /* owning */ visited_file_t,
    pub n: size_t,
    pub capacity: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
struct ErasedByRefactorer2;
#[repr(C)]
pub struct libtree_state_t {
    pub verbosity: libc::c_int,
    pub path: libc::c_int,
    pub color: libc::c_int,
    pub ld_conf_file: *mut libc::c_char,
    pub max_depth: libc::c_ulong,
    pub string_table: string_table_t,
    pub visited: visited_file_array_t,
    pub PLATFORM: *mut libc::c_char,
    pub LIB: *mut libc::c_char,
    pub OSNAME: *mut libc::c_char,
    pub OSREL: *mut libc::c_char,
    pub rpath_offsets: [size_t; 32],
    pub ld_library_path_offset: size_t,
    pub default_paths_offset: size_t,
    pub ld_so_conf_offset: size_t,
    pub found_all_needed: [libc::c_char; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct small_vec_u64_t {
    pub buf: [uint64_t; 16],
    pub p: *mut uint64_t,
    pub n: size_t,
    pub capacity: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub p64: prog_64_t,
    pub p32: prog_32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub h64: header_64_t,
    pub h32: header_32_t,
}
#[inline]
unsafe extern "C" fn putchar(mut __c: libc::c_int) -> libc::c_int {
    return _IO_putc(__c, crate::src::libtree::stdout);
}
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
#[no_mangle]
pub static mut exclude_list: [*const libc::c_char; 14] = [
    b"ld-linux-aarch64.so\0" as *const u8 as *const libc::c_char,
    b"ld-linux-armhf.so\0" as *const u8 as *const libc::c_char,
    b"ld-linux-x86-64.so\0" as *const u8 as *const libc::c_char,
    b"ld-linux.so\0" as *const u8 as *const libc::c_char,
    b"ld64.so\0" as *const u8 as *const libc::c_char,
    b"libc.musl-aarch64.so\0" as *const u8 as *const libc::c_char,
    b"libc.musl-armhf.so\0" as *const u8 as *const libc::c_char,
    b"libc.musl-i386.so\0" as *const u8 as *const libc::c_char,
    b"libc.musl-x86_64.so\0" as *const u8 as *const libc::c_char,
    b"libc.so\0" as *const u8 as *const libc::c_char,
    b"libdl.so\0" as *const u8 as *const libc::c_char,
    b"libgcc_s.so\0" as *const u8 as *const libc::c_char,
    b"libm.so\0" as *const u8 as *const libc::c_char,
    b"libstdc++.so\0" as *const u8 as *const libc::c_char,
];
#[inline]
unsafe extern "C" fn utoa(mut str: *mut libc::c_char, mut v: size_t) {
    let mut p = str;
    loop {
        let fresh0 = p;
        p= p.offset(1);
        *fresh0= ('0' as i32 as libc::c_ulong)
            .wrapping_add(v.wrapping_rem(10 as libc::c_int as libc::c_ulong))
            as libc::c_char;
        v= (v as libc::c_ulong).wrapping_div(10 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
        if !(v > 0 as libc::c_int as libc::c_ulong) {
            break;
        }
    }
    let mut len = p.offset_from(str) as libc::c_long as size_t;
    let mut i = 0 as libc::c_int as size_t;
    while i < len.wrapping_div(2 as libc::c_int as libc::c_ulong) {
        let mut tmp = *str.offset(i as isize);
        *str
            .offset(
                i as isize,
            ) = *str
            .offset(
                len.wrapping_sub(i).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    as isize,
            );
        *str
            .offset(
                len.wrapping_sub(i).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    as isize,
            ) = tmp;
        i= i.wrapping_add(1);
    }
    *str.offset(len as isize) = '\0' as i32 as libc::c_char;
}
#[inline]
unsafe extern "C" fn small_vec_u64_init(mut v: *mut small_vec_u64_t) {
    memset(
        v as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<small_vec_u64_t>() as libc::c_ulong,
    );
    (*v).p= (*v).buf.as_mut_ptr();
}
unsafe extern "C" fn small_vec_u64_append(
    mut v: *mut small_vec_u64_t,
    mut val: uint64_t,
) {
    if (*v).n < 16 as libc::c_int as libc::c_ulong {
        let fresh3 = (*v).n;(*v).n= (*v).n.wrapping_add(1);
        *(*v).p.offset(fresh3 as isize) = val;
        return;
    }
    if (*v).n == 16 as libc::c_int as libc::c_ulong {
        (*v).capacity= (2 as libc::c_int * 16 as libc::c_int) as size_t;
        (*v).p= malloc(
            (*v).capacity
                .wrapping_mul(::std::mem::size_of::<uint64_t>() as libc::c_ulong),
        ) as *mut uint64_t;
        if (*v).p.is_null() {();
            exit(1 as libc::c_int);
        }
        memcpy(
            (*v).p as *mut libc::c_void,
            (*v).buf.as_mut_ptr() as *const libc::c_void,
            (16 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<uint64_t>() as libc::c_ulong),
        );
    } else if (*v).n == (*v).capacity {
        (*v).capacity= ((*v).capacity as libc::c_ulong)
            .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
        let mut p = realloc(
            (*v).p as *mut libc::c_void,
            (*v).capacity
                .wrapping_mul(::std::mem::size_of::<uint64_t>() as libc::c_ulong),
        ) as *mut uint64_t;
        if p.is_null() {();
            exit(1 as libc::c_int);
        }
        (*v).p= p;
    }
    let fresh8 = (*v).n;(*v).n= (*v).n.wrapping_add(1);
    *(*v).p.offset(fresh8 as isize) = val;
}
unsafe extern "C" fn small_vec_u64_free(mut v: *mut small_vec_u64_t) {
    if (*v).n <= 16 as libc::c_int as libc::c_ulong {
        return;
    }
    free((*v).p as *mut libc::c_void);
    (*v).p= 0 as *mut uint64_t;
}
#[inline]
unsafe extern "C" fn host_is_little_endian() -> libc::c_int {
    let mut test = 1 as libc::c_int;
    let mut bytes = core::ptr::addr_of_mut!(test) as *mut libc::c_int as *mut libc::c_char;
    return (*bytes.offset(0 as libc::c_int as isize) as libc::c_int == 1 as libc::c_int)
        as libc::c_int;
}
unsafe extern "C" fn is_ascending_order(
    mut v: *mut uint64_t,
    mut n: size_t,
) -> libc::c_int {
    let mut j = 1 as libc::c_int as size_t;
    while j < n {
        if *v.offset(j.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            >= *v.offset(j as isize)
        {
            return 0 as libc::c_int;
        }
        j= j.wrapping_add(1);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn string_table_maybe_grow(mut t: Option<&mut string_table_t>, mut n: size_t) {
    if (*t.as_deref().unwrap()).n.wrapping_add(n) <= (*t.as_deref().unwrap()).capacity {
        return;
    }
    (*t.as_deref_mut().unwrap()).capacity= (2 as libc::c_int as libc::c_ulong)
        .wrapping_mul((*t.as_deref().unwrap()).n.wrapping_add(n));
    let mut arr = realloc(
        (*t.as_deref().unwrap()).arr as *mut libc::c_void,
        (*t.as_deref().unwrap()).capacity
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
    ) as *mut libc::c_char;
    if arr.is_null() {();
        exit(1 as libc::c_int);
    }
    (*t.as_deref_mut().unwrap()).arr= arr;
}
unsafe extern "C" fn string_table_store(
    mut t: Option<&mut string_table_t>,
    mut str: *const libc::c_char,
) {
    let mut n = (strlen(str)).wrapping_add(1 as libc::c_int as libc::c_ulong);
    string_table_maybe_grow(t.as_deref_mut(), n);
    memcpy(
        (*t.as_deref().unwrap()).arr.offset((*t.as_deref().unwrap()).n as isize) as *mut libc::c_void,
        str as *const libc::c_void,
        n,
    );
    (*t.as_deref_mut().unwrap()).n= ((*t.as_deref().unwrap()).n as libc::c_ulong).wrapping_add(n) as size_t as size_t;
}
unsafe extern "C" fn string_table_copy_from_file(
    mut t: Option<&mut string_table_t>,
    mut fptr: *mut FILE,
) {
    let mut c: libc::c_int = 0;
    loop {
        c= _IO_getc(fptr);
        if !(c != '\0' as i32 && c != -(1 as libc::c_int)) {
            break;
        }
        string_table_maybe_grow(t.as_deref_mut(), 1 as libc::c_int as size_t);
        let fresh13 = (*t.as_deref().unwrap()).n;(*t.as_deref_mut().unwrap()).n= (*t.as_deref().unwrap()).n.wrapping_add(1);
        *(*t.as_deref().unwrap()).arr.offset(fresh13 as isize) = c as libc::c_char;
    }
    string_table_maybe_grow(t.as_deref_mut(), 1 as libc::c_int as size_t);
    let fresh15 = (*t.as_deref().unwrap()).n;(*t.as_deref_mut().unwrap()).n= (*t.as_deref().unwrap()).n.wrapping_add(1);
    *(*t.as_deref().unwrap()).arr.offset(fresh15 as isize) = '\0' as i32 as libc::c_char;
}
unsafe extern "C" fn is_in_exclude_list(mut soname: *mut libc::c_char) -> libc::c_int {
    let mut start = soname;
    let mut end = strrchr(start, '\0' as i32);
    if start == end {
        return 0 as libc::c_int;
    }
    end= end.offset(-1);
    while end != start
        && ((*end) as libc::c_int >= '0' as i32 && (*end) as libc::c_int <= '9' as i32
            || (*end) as libc::c_int == '.' as i32)
    {
        end= end.offset(-1);
    }
    let mut j = 0 as libc::c_int as size_t;
    while j
        < (::std::mem::size_of::<[*const libc::c_char; 14]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
    {
        let mut len = strlen(crate::src::libtree::exclude_list[j as usize]);
        if strncmp(start, crate::src::libtree::exclude_list[j as usize], len) != 0 as libc::c_int {
            j= j.wrapping_add(1);
        } else {
            return 1 as libc::c_int
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn tree_preamble(mut s: *mut libtree_state_t, mut depth: size_t) {
    if depth == 0 as libc::c_int as libc::c_ulong {
        return;
    }
    let mut i = 0 as libc::c_int as size_t;
    while i < depth.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
        fputs(
            if (*s).found_all_needed[i as usize] as libc::c_int != 0 {
                b"    \0" as *const u8 as *const libc::c_char
            } else {
                b"\xE2\x94\x82   \0" as *const u8 as *const libc::c_char
            },
            crate::src::libtree::stdout,
        );
        i= i.wrapping_add(1);
    }
    fputs(
        if (*s).found_all_needed[depth.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            as usize] as libc::c_int != 0
        {
            b"\xE2\x94\x94\xE2\x94\x80\xE2\x94\x80 \0" as *const u8
                as *const libc::c_char
        } else {
            b"\xE2\x94\x9C\xE2\x94\x80\xE2\x94\x80 \0" as *const u8
                as *const libc::c_char
        },
        crate::src::libtree::stdout,
    );
}
unsafe extern "C" fn apply_exclude_list(
    mut needed_not_found: Option<&mut size_t>,
    mut needed_buf_offsets: *mut small_vec_u64_t,
    mut s: *mut libtree_state_t,
) {
    let mut i = 0 as libc::c_int as size_t;
    while i < (*needed_not_found.as_deref().unwrap()) {
        if is_in_exclude_list(
            (*s).string_table.arr
                .offset(*(*needed_buf_offsets).p.offset(i as isize) as isize),
        ) != 0
        {
            let mut tmp = *(*needed_buf_offsets).p.offset(i as isize);
            *(*needed_buf_offsets).p
                .offset(
                    i as isize,
                ) = *(*needed_buf_offsets).p
                .offset(
                    (*needed_not_found.as_deref().unwrap()).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        as isize,
                );
            *needed_not_found.as_deref_mut().unwrap()= (*needed_not_found.as_deref().unwrap()).wrapping_sub(1);
            *(*needed_buf_offsets).p.offset((*needed_not_found.as_deref().unwrap()) as isize) = tmp;
        } else {
            i= i.wrapping_add(1);
        }
    }
}
unsafe extern "C" fn check_absolute_paths(
    mut needed_not_found: Option<&mut size_t>,
    mut needed_buf_offsets: *mut small_vec_u64_t,
    mut depth: size_t,
    mut s: Option<&mut libtree_state_t>,
    mut compat: compat_t,
) -> libc::c_int {
    let mut exit_code = 0 as libc::c_int;
    let mut i = 0 as libc::c_int as size_t;
    while i < (*needed_not_found.as_deref().unwrap()) {
        let mut st: *const string_table_t = core::ptr::addr_of!((*s.as_deref_mut().unwrap()).string_table);
        if (strchr(
            (*st).arr.offset(*(*needed_buf_offsets).p.offset(i as isize) as isize),
            '/' as i32,
        ))
            .is_null()
        {();
            i= i.wrapping_add(1);
        } else {
            let mut path: [libc::c_char; 4096] = [0; 4096];
            let mut len = strlen(
                (*st).arr
                    .offset(*(*needed_buf_offsets).p.offset(i as isize) as isize),
            );
            if len >= 4096 as libc::c_int as libc::c_ulong {
                continue;
            }
            memcpy(
                path.as_mut_ptr() as *mut libc::c_void,
                (*st).arr
                    .offset(*(*needed_buf_offsets).p.offset(i as isize) as isize)
                    as *const libc::c_void,
                len.wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
            (*s.as_deref_mut().unwrap()).found_all_needed[depth
                as usize]= ((*needed_not_found.as_deref().unwrap()) <= 1 as libc::c_int as libc::c_ulong)
                as libc::c_int as libc::c_char;
            let mut err = 0 as *mut libc::c_char;
            if path[0 as libc::c_int as usize] as libc::c_int != '/' as i32 {
                err= b" is not absolute\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
                exit_code= 28 as libc::c_int;
            } else {
                let mut code = recurse(
                    path.as_mut_ptr(),
                    depth.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    s.as_deref_mut(),
                    compat,
                    {
                        let mut init = found_t { how: DIRECT, depth: 0 };
                        init
                    },
                );
                if code == 28 as libc::c_int {
                    exit_code= 28 as libc::c_int;
                }
                if code != 0 as libc::c_int && code != 28 as libc::c_int {
                    err= b" not found\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                }
            }
            if !err.is_null() {
                tree_preamble(s.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()), depth.wrapping_add(1 as libc::c_int as libc::c_ulong));
                if (*s.as_deref().unwrap()).color != 0 {
                    fputs(b"\x1B[1;31m\0" as *const u8 as *const libc::c_char, crate::src::libtree::stdout);
                }
                fputs(path.as_mut_ptr(), crate::src::libtree::stdout);
                fputs(b" is not absolute\0" as *const u8 as *const libc::c_char, crate::src::libtree::stdout);
                fputs(
                    if (*s.as_deref().unwrap()).color != 0 {
                        b"\x1B[0m\n\0" as *const u8 as *const libc::c_char
                    } else {
                        b"\n\0" as *const u8 as *const libc::c_char
                    },
                    crate::src::libtree::stdout,
                );
            }else { (); }
            let mut tmp = *(*needed_buf_offsets).p.offset(i as isize);
            *(*needed_buf_offsets).p
                .offset(
                    i as isize,
                ) = *(*needed_buf_offsets).p
                .offset(
                    (*needed_not_found.as_deref().unwrap()).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        as isize,
                );
            *needed_not_found.as_deref_mut().unwrap()= (*needed_not_found.as_deref().unwrap()).wrapping_sub(1);
            *(*needed_buf_offsets).p.offset((*needed_not_found.as_deref().unwrap()) as isize) = tmp;
        }
    }
    return exit_code;
}
unsafe extern "C" fn check_search_paths(
    mut reason: found_t,
    mut offset: size_t,
    mut needed_not_found: Option<&mut size_t>,
    mut needed_buf_offsets: *mut small_vec_u64_t,
    mut depth: size_t,
    mut s: Option<&mut libtree_state_t>,
    mut compat: compat_t,
) -> libc::c_int {
    let mut exit_code = 0 as libc::c_int;
    let mut path: [libc::c_char; 4096] = [0; 4096];
    let mut path_end = path.as_mut_ptr().offset(4096 as libc::c_int as isize);
    let mut st: *const string_table_t = core::ptr::addr_of!((*s.as_deref_mut().unwrap()).string_table);
    while *(*st).arr.offset(offset as isize) as libc::c_int != '\0' as i32 {
        while *(*st).arr.offset(offset as isize) as libc::c_int == ':' as i32
            && *(*st).arr.offset(offset as isize) as libc::c_int != '\0' as i32
        {
            offset= offset.wrapping_add(1);
        }
        if *(*st).arr.offset(offset as isize) as libc::c_int == '\0' as i32 {
            return exit_code;
        }
        let mut dest = path.as_mut_ptr();
        while *(*st).arr.offset(offset as isize) as libc::c_int != '\0' as i32
            && *(*st).arr.offset(offset as isize) as libc::c_int != ':' as i32
            && dest != path_end
        {
            let fresh16 = offset;
            offset= offset.wrapping_add(1);
            let fresh17 = dest;
            dest= dest.offset(1);
            *fresh17= *(*st).arr.offset(fresh16 as isize);
        }
        if dest.offset(1 as libc::c_int as isize) >= path_end {
            continue;
        }
        if *dest.offset(-(1 as libc::c_int as isize)) as libc::c_int != '/' as i32 {
            let fresh18 = dest;
            dest= dest.offset(1);
            *fresh18= '/' as i32 as libc::c_char;
        }
        let mut search_path_end = dest;
        let mut i = 0 as libc::c_int as size_t;
        while i < (*needed_not_found.as_deref().unwrap()) {
            let mut soname_len = strlen(
                (*st).arr
                    .offset(*(*needed_buf_offsets).p.offset(i as isize) as isize),
            );
            if search_path_end
                .offset(soname_len as isize)
                .offset(1 as libc::c_int as isize) >= path_end
            {
                continue;
            }
            memcpy(
                search_path_end as *mut libc::c_void,
                (*st).arr
                    .offset(*(*needed_buf_offsets).p.offset(i as isize) as isize)
                    as *const libc::c_void,
                soname_len.wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
            (*s.as_deref_mut().unwrap()).found_all_needed[depth
                as usize]= ((*needed_not_found.as_deref().unwrap()) <= 1 as libc::c_int as libc::c_ulong)
                as libc::c_int as libc::c_char;
            let mut code = recurse(
                path.as_mut_ptr(),
                depth.wrapping_add(1 as libc::c_int as libc::c_ulong),
                s.as_deref_mut(),
                compat,
                reason,
            );
            if code == 28 as libc::c_int {
                exit_code= 28 as libc::c_int;
            }
            if code == 0 as libc::c_int || code == 28 as libc::c_int {
                let mut tmp = *(*needed_buf_offsets).p.offset(i as isize);
                *(*needed_buf_offsets).p
                    .offset(
                        i as isize,
                    ) = *(*needed_buf_offsets).p
                    .offset(
                        (*needed_not_found.as_deref().unwrap())
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                    );
                *needed_not_found.as_deref_mut().unwrap()= (*needed_not_found.as_deref().unwrap()).wrapping_sub(1);
                *(*needed_buf_offsets).p.offset((*needed_not_found.as_deref().unwrap()) as isize) = tmp;
            } else {
                i= i.wrapping_add(1);
            }
        }
    }
    return exit_code;
}
unsafe extern "C" fn interpolate_variables(
    mut s: *mut libtree_state_t,
    mut src: size_t,
    mut ORIGIN: *const libc::c_char,
) -> libc::c_int {
    let mut prev_src = src;
    let mut curr_src = src;
    let mut st: *mut string_table_t = core::ptr::addr_of_mut!((*s).string_table);
    loop {
        let mut dollar = strchr((*st).arr.offset(curr_src as isize), '$' as i32);
        if dollar.is_null() {();
            break;
        }
        curr_src= dollar.offset_from((*st).arr as *const i8) as libc::c_long as size_t;
        let mut bytes_to_dollar = curr_src.wrapping_sub(prev_src);
        curr_src= curr_src.wrapping_add(1);
        let mut curly = 0 as libc::c_int;
        if *(*st).arr.offset(curr_src as isize) as libc::c_int == '{' as i32 {
            curly= 1 as libc::c_int;
            curr_src= curr_src.wrapping_add(1);
        }
        let mut var_val = 0 as *const libc::c_char;
        if strncmp(
            core::ptr::addr_of_mut!(*(*st).arr.offset(curr_src as isize)),
            b"ORIGIN\0" as *const u8 as *const libc::c_char,
            6 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            var_val= ORIGIN;
            curr_src= (curr_src as libc::c_ulong)
                .wrapping_add(6 as libc::c_int as libc::c_ulong) as size_t as size_t;
        } else if strncmp(
            core::ptr::addr_of_mut!(*(*st).arr.offset(curr_src as isize)),
            b"LIB\0" as *const u8 as *const libc::c_char,
            3 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            var_val= (*s).LIB;
            curr_src= (curr_src as libc::c_ulong)
                .wrapping_add(3 as libc::c_int as libc::c_ulong) as size_t as size_t;
        } else if strncmp(
            core::ptr::addr_of_mut!(*(*st).arr.offset(curr_src as isize)),
            b"PLATFORM\0" as *const u8 as *const libc::c_char,
            8 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            var_val= (*s).PLATFORM;
            curr_src= (curr_src as libc::c_ulong)
                .wrapping_add(8 as libc::c_int as libc::c_ulong) as size_t as size_t;
        } else if strncmp(
            core::ptr::addr_of_mut!(*(*st).arr.offset(curr_src as isize)),
            b"OSNAME\0" as *const u8 as *const libc::c_char,
            6 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            var_val= (*s).OSNAME;
            curr_src= (curr_src as libc::c_ulong)
                .wrapping_add(6 as libc::c_int as libc::c_ulong) as size_t as size_t;
        } else {
            if !(strncmp(
                core::ptr::addr_of_mut!(*(*st).arr.offset(curr_src as isize)),
                b"OSREL\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int)
            {
                continue;
            }
            var_val= (*s).OSREL;
            curr_src= (curr_src as libc::c_ulong)
                .wrapping_add(5 as libc::c_int as libc::c_ulong) as size_t as size_t;
        }
        if curly != 0 {
            if *(*st).arr.offset(curr_src as isize) as libc::c_int != '}' as i32 {
                continue;
            }
            curr_src= curr_src.wrapping_add(1);
        }
        let mut var_len = strlen(var_val);
        string_table_maybe_grow(st.as_mut(), bytes_to_dollar.wrapping_add(var_len));
        memcpy(
            core::ptr::addr_of_mut!(*(*st).arr.offset((*s).string_table.n as isize)) as *mut libc::c_char
                as *mut libc::c_void,
            core::ptr::addr_of_mut!(*(*st).arr.offset(prev_src as isize)) as *mut libc::c_char
                as *const libc::c_void,
            bytes_to_dollar,
        );
        (*s).string_table.n= ((*s).string_table.n as libc::c_ulong).wrapping_add(bytes_to_dollar) as size_t
            as size_t;
        prev_src= curr_src;
        memcpy(
            core::ptr::addr_of_mut!(*(*st).arr.offset((*s).string_table.n as isize)) as *mut libc::c_char
                as *mut libc::c_void,
            var_val as *const libc::c_void,
            var_len,
        );
        (*s).string_table.n= ((*s).string_table.n as libc::c_ulong).wrapping_add(var_len) as size_t as size_t;
    }
    if prev_src != src {
        let mut n = (strlen((*st).arr.offset(prev_src as isize)))
            .wrapping_add(1 as libc::c_int as libc::c_ulong);
        string_table_maybe_grow(st.as_mut(), n);
        memcpy(
            (*st).arr.offset((*st).n as isize) as *mut libc::c_void,
            (*st).arr.offset(prev_src as isize) as *const libc::c_void,
            n,
        );
        (*st).n= ((*st).n as libc::c_ulong).wrapping_add(n) as size_t as size_t;
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn print_colon_delimited_paths(
    mut start: *const libc::c_char,
    mut indent: *const libc::c_char,
) {
    while !((*start) as libc::c_int == '\0' as i32) {
        let mut next = strchr(start, ':' as i32);
        if start == next {
            start= start.offset(1);
        } else {
            fputs(indent, crate::src::libtree::stdout);
            fputs(b"    \0" as *const u8 as *const libc::c_char, crate::src::libtree::stdout);
            if next.is_null() {();
                puts(start);
            } else {
                fwrite(
                    start as *const libc::c_void,
                    1 as libc::c_int as size_t,
                    next.offset_from(start) as libc::c_long as size_t,
                    crate::src::libtree::stdout,
                );
                putchar('\n' as i32);
            }
            if next.is_null() {();
                break;
            }
            start= next.offset(1 as libc::c_int as isize);
        }
    }
}
unsafe extern "C" fn print_line(
    mut depth: size_t,
    mut name: *mut libc::c_char,
    mut color_bold: *mut libc::c_char,
    mut color_regular: *mut libc::c_char,
    mut highlight: libc::c_int,
    mut reason: found_t,
    mut s: Option<&mut libtree_state_t>,
) {
    tree_preamble(s.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()), depth);
    let mut slash = 0 as *mut libc::c_char;
    if (*s.as_deref().unwrap()).color != 0 && highlight != 0
        && {
            slash= strrchr(name, '/' as i32);
            !slash.is_null()
        }
    {
        fputs(color_regular, crate::src::libtree::stdout);
        fwrite(
            name as *const libc::c_void,
            1 as libc::c_int as size_t,
            slash.offset(1 as libc::c_int as isize).offset_from(name) as libc::c_long
                as size_t,
            crate::src::libtree::stdout,
        );
        fputs(color_bold, crate::src::libtree::stdout);
        fputs(slash.offset(1 as libc::c_int as isize), crate::src::libtree::stdout);
    } else {
        if (*s.as_deref().unwrap()).color != 0 {
            fputs(color_bold, crate::src::libtree::stdout);
        }
        fputs(name, crate::src::libtree::stdout);
    }
    if (*s.as_deref().unwrap()).color != 0 && highlight != 0 {
        fputs(b"\x1B[0m \x1B[33m\0" as *const u8 as *const libc::c_char, crate::src::libtree::stdout);
    } else {
        putchar(' ' as i32);
    }
    let mut conf_name: *mut libc::c_char = 0 as *mut libc::c_char;
    match  reason.how as libc::c_uint {
        2 => {
            if reason.depth.wrapping_add(1 as libc::c_int as libc::c_ulong) >= depth {
                fputs(b"[rpath]\0" as *const u8 as *const libc::c_char, crate::src::libtree::stdout);
            } else {
                let mut num: [libc::c_char; 8] = [0; 8];
                utoa(
                    num.as_mut_ptr(),
                    reason.depth.wrapping_add(1 as libc::c_int as libc::c_ulong),
                );
                fputs(b"[rpath of \0" as *const u8 as *const libc::c_char, crate::src::libtree::stdout);
                fputs(num.as_mut_ptr(), crate::src::libtree::stdout);
                putchar(']' as i32);
            }
        }
        3 => {
            fputs(b"[LD_LIBRARY_PATH]\0" as *const u8 as *const libc::c_char, crate::src::libtree::stdout);
        }
        4 => {
            fputs(b"[runpath]\0" as *const u8 as *const libc::c_char, crate::src::libtree::stdout);
        }
        5 => {
            putchar('[' as i32);
            conf_name= strrchr((*s.as_deref().unwrap()).ld_conf_file, '/' as i32);
            conf_name= if conf_name.is_null() {();
                (*s.as_deref().unwrap()).ld_conf_file
            } else {
                conf_name.offset(1 as libc::c_int as isize)
            };
            fputs(conf_name, crate::src::libtree::stdout);
            putchar(']' as i32);
        }
        1 => {
            fputs(b"[direct]\0" as *const u8 as *const libc::c_char, crate::src::libtree::stdout);
        }
        6 => {
            fputs(b"[default path]\0" as *const u8 as *const libc::c_char, crate::src::libtree::stdout);
        }
        _ => {}
    }
    if (*s.as_deref().unwrap()).color != 0 {
        fputs(b"\x1B[0m\n\0" as *const u8 as *const libc::c_char, crate::src::libtree::stdout);
    } else {
        putchar('\n' as i32);
    };
}
unsafe extern "C" fn print_error(
    mut depth: size_t,
    mut needed_not_found: size_t,
    mut needed_buf_offsets: *mut small_vec_u64_t,
    mut runpath: *mut libc::c_char,
    mut s: Option<&mut libtree_state_t>,
    mut no_def_lib: libc::c_int,
) {
    let mut i = 0 as libc::c_int as size_t;
    while i < needed_not_found {
        (*s.as_deref_mut().unwrap()).found_all_needed[depth
            as usize]= (i.wrapping_add(1 as libc::c_int as libc::c_ulong)
            >= needed_not_found) as libc::c_int as libc::c_char;
        tree_preamble(s.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()), depth.wrapping_add(1 as libc::c_int as libc::c_ulong));
        if (*s.as_deref().unwrap()).color != 0 {
            fputs(b"\x1B[1;31m\0" as *const u8 as *const libc::c_char, crate::src::libtree::stdout);
        }
        fputs(
            (*s.as_deref().unwrap()).string_table.arr
                .offset(*(*needed_buf_offsets).p.offset(i as isize) as isize),
            crate::src::libtree::stdout,
        );
        fputs(b" not found\n\0" as *const u8 as *const libc::c_char, crate::src::libtree::stdout);
        if (*s.as_deref().unwrap()).color != 0 {
            fputs(b"\x1B[0m\0" as *const u8 as *const libc::c_char, crate::src::libtree::stdout);
        }
        i= i.wrapping_add(1);
    }
    let mut box_vertical = (if (*s.as_deref().unwrap()).color != 0 {
        b"    \x1B[0;31m\xE2\x94\x8A\x1B[0m\0" as *const u8 as *const libc::c_char
    } else {
        b"    \xE2\x94\x8A\0" as *const u8 as *const libc::c_char
    }) as *mut libc::c_char;
    let mut indent = malloc(
        (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
            .wrapping_mul(depth)
            .wrapping_add(strlen(box_vertical))
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    let mut p = indent;
    let mut i_0 = 0 as libc::c_int as size_t;
    while i_0 < depth {
        if (*s.as_deref().unwrap()).found_all_needed[i_0 as usize] != 0 {
            let mut len = (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int;
            memcpy(
                p as *mut libc::c_void,
                b"    \0" as *const u8 as *const libc::c_char as *const libc::c_void,
                len as libc::c_ulong,
            );
            p= p.offset(len as isize);
        } else {
            let mut len_0 = (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int;
            memcpy(
                p as *mut libc::c_void,
                b"\xE2\x94\x82   \0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                len_0 as libc::c_ulong,
            );
            p= p.offset(len_0 as isize);
        }
        i_0= i_0.wrapping_add(1);
    }
    strcpy(p, box_vertical);
    fputs(indent as *const i8, crate::src::libtree::stdout);
    if (*s.as_deref().unwrap()).color != 0 {
        fputs(b"\x1B[0;90m\0" as *const u8 as *const libc::c_char, crate::src::libtree::stdout);
    }
    fputs(
        b" Paths considered in this order:\n\0" as *const u8 as *const libc::c_char,
        crate::src::libtree::stdout,
    );
    if (*s.as_deref().unwrap()).color != 0 {
        fputs(b"\x1B[0m\0" as *const u8 as *const libc::c_char, crate::src::libtree::stdout);
    }
    fputs(indent as *const i8, crate::src::libtree::stdout);
    if !runpath.is_null() {
        if (*s.as_deref().unwrap()).color != 0 {
            fputs(b"\x1B[0;90m\0" as *const u8 as *const libc::c_char, crate::src::libtree::stdout);
        }
        fputs(
            b" 1. rpath is skipped because runpath was set\n\0" as *const u8
                as *const libc::c_char,
            crate::src::libtree::stdout,
        );
        if (*s.as_deref().unwrap()).color != 0 {
            fputs(b"\x1B[0m\0" as *const u8 as *const libc::c_char, crate::src::libtree::stdout);
        }
    } else {();
        if (*s.as_deref().unwrap()).color != 0 {
            fputs(b"\x1B[0;90m\0" as *const u8 as *const libc::c_char, crate::src::libtree::stdout);
        }
        fputs(b" 1. rpath:\n\0" as *const u8 as *const libc::c_char, crate::src::libtree::stdout);
        if (*s.as_deref().unwrap()).color != 0 {
            fputs(b"\x1B[0m\0" as *const u8 as *const libc::c_char, crate::src::libtree::stdout);
        }
        let mut j = depth as libc::c_int;
        while j >= 0 as libc::c_int {
            if (*s.as_deref().unwrap()).rpath_offsets[j as usize] != 18446744073709551615 as libc::c_ulong {
                let mut num: [libc::c_char; 8] = [0; 8];
                utoa(num.as_mut_ptr(), (j + 1 as libc::c_int) as size_t);
                fputs(indent as *const i8, crate::src::libtree::stdout);
                if (*s.as_deref().unwrap()).color != 0 {
                    fputs(b"\x1B[0;90m\0" as *const u8 as *const libc::c_char, crate::src::libtree::stdout);
                }
                fputs(b"    depth \0" as *const u8 as *const libc::c_char, crate::src::libtree::stdout);
                fputs(num.as_mut_ptr(), crate::src::libtree::stdout);
                if (*s.as_deref().unwrap()).color != 0 {
                    fputs(b"\x1B[0m\0" as *const u8 as *const libc::c_char, crate::src::libtree::stdout);
                }
                putchar('\n' as i32);
                print_colon_delimited_paths(
                    (*s.as_deref().unwrap()).string_table.arr
                        .offset((*s.as_deref().unwrap()).rpath_offsets[j as usize] as isize),
                    indent as *const i8,
                );
            }
            j-= 1;
        }
    }
    fputs(indent as *const i8, crate::src::libtree::stdout);
    if (*s.as_deref().unwrap()).color != 0 {
        fputs(b"\x1B[0;90m\0" as *const u8 as *const libc::c_char, crate::src::libtree::stdout);
    }
    fputs(
        if (*s.as_deref().unwrap()).ld_library_path_offset == 18446744073709551615 as libc::c_ulong {
            b" 2. LD_LIBRARY_PATH was not set\n\0" as *const u8 as *const libc::c_char
        } else {
            b" 2. LD_LIBRARY_PATH:\n\0" as *const u8 as *const libc::c_char
        },
        crate::src::libtree::stdout,
    );
    if (*s.as_deref().unwrap()).color != 0 {
        fputs(b"\x1B[0m\0" as *const u8 as *const libc::c_char, crate::src::libtree::stdout);
    }
    if (*s.as_deref().unwrap()).ld_library_path_offset != 18446744073709551615 as libc::c_ulong {
        print_colon_delimited_paths(
            (*s.as_deref().unwrap()).string_table.arr.offset((*s.as_deref().unwrap()).ld_library_path_offset as isize),
            indent as *const i8,
        );
    }
    fputs(indent as *const i8, crate::src::libtree::stdout);
    if (*s.as_deref().unwrap()).color != 0 {
        fputs(b"\x1B[0;90m\0" as *const u8 as *const libc::c_char, crate::src::libtree::stdout);
    }
    fputs(
        if runpath.is_null() {();
            b" 3. runpath was not set\n\0" as *const u8 as *const libc::c_char
        } else {
            b" 3. runpath:\n\0" as *const u8 as *const libc::c_char
        },
        crate::src::libtree::stdout,
    );
    if (*s.as_deref().unwrap()).color != 0 {
        fputs(b"\x1B[0m\0" as *const u8 as *const libc::c_char, crate::src::libtree::stdout);
    }
    if !runpath.is_null() {
        print_colon_delimited_paths(runpath, indent as *const i8);
    }else { (); }
    fputs(indent as *const i8, crate::src::libtree::stdout);
    if (*s.as_deref().unwrap()).color != 0 {
        fputs(b"\x1B[0;90m\0" as *const u8 as *const libc::c_char, crate::src::libtree::stdout);
    }
    fputs(
        if no_def_lib != 0 {
            b" 4. ld config files not considered due to NODEFLIB flag\n\0" as *const u8
                as *const libc::c_char
        } else {
            b" 4. ld config files:\n\0" as *const u8 as *const libc::c_char
        },
        crate::src::libtree::stdout,
    );
    if (*s.as_deref().unwrap()).color != 0 {
        fputs(b"\x1B[0m\0" as *const u8 as *const libc::c_char, crate::src::libtree::stdout);
    }
    print_colon_delimited_paths(
        (*s.as_deref().unwrap()).string_table.arr.offset((*s.as_deref().unwrap()).ld_so_conf_offset as isize),
        indent as *const i8,
    );
    fputs(indent as *const i8, crate::src::libtree::stdout);
    if (*s.as_deref().unwrap()).color != 0 {
        fputs(b"\x1B[0;90m\0" as *const u8 as *const libc::c_char, crate::src::libtree::stdout);
    }
    fputs(
        if no_def_lib != 0 {
            b" 5. Standard paths not considered due to NODEFLIB flag\n\0" as *const u8
                as *const libc::c_char
        } else {
            b" 5. Standard paths:\n\0" as *const u8 as *const libc::c_char
        },
        crate::src::libtree::stdout,
    );
    if (*s.as_deref().unwrap()).color != 0 {
        fputs(b"\x1B[0m\0" as *const u8 as *const libc::c_char, crate::src::libtree::stdout);
    }
    print_colon_delimited_paths(
        (*s.as_deref().unwrap()).string_table.arr.offset((*s.as_deref().unwrap()).default_paths_offset as isize),
        indent as *const i8,
    );
    free(indent as *mut libc::c_void);
}
unsafe extern "C" fn visited_files_contains(
    mut files: *mut visited_file_array_t,
    mut needle: *mut stat,
) -> libc::c_int {
    let mut i = 0 as libc::c_int as size_t;
    while i < (*files).n {
        let mut f: *mut visited_file_t = core::ptr::addr_of_mut!(*(*files).arr.offset(i as isize))
            as *mut visited_file_t;
        if (*f).st_dev == (*needle).st_dev && (*f).st_ino == (*needle).st_ino {
            return 1 as libc::c_int;
        }
        i= i.wrapping_add(1);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn visited_files_append(
    mut files: Option<&mut visited_file_array_t>,
    mut new: *mut stat,
) {
    if (*files.as_deref().unwrap()).n == (*files.as_deref().unwrap()).capacity {
        (*files.as_deref_mut().unwrap()).capacity= ((*files.as_deref().unwrap()).capacity as libc::c_ulong)
            .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
        (*files.as_deref_mut().unwrap()).arr= realloc(
            (*files.as_deref().unwrap()).arr as *mut libc::c_void,
            (*files.as_deref().unwrap()).capacity
                .wrapping_mul(::std::mem::size_of::<visited_file_t>() as libc::c_ulong),
        ) as *mut visited_file_t;
        if (*files.as_deref().unwrap()).arr.is_null() {();
            exit(1 as libc::c_int);
        }
    }
    (*(*files.as_deref().unwrap()).arr.offset((*files.as_deref().unwrap()).n as isize)).st_dev = (*new).st_dev;
    (*(*files.as_deref().unwrap()).arr.offset((*files.as_deref().unwrap()).n as isize)).st_ino = (*new).st_ino;
    (*files.as_deref_mut().unwrap()).n= (*files.as_deref().unwrap()).n.wrapping_add(1);
}
unsafe extern "C" fn recurse(
    mut current_file: *mut libc::c_char,
    mut depth: size_t,
    mut s: Option<&mut libtree_state_t>,
    mut compat: compat_t,
    mut reason: found_t,
) -> libc::c_int {
    let mut fptr = fopen(current_file, b"rb\0" as *const u8 as *const libc::c_char);
    if fptr.is_null() {();
        return 31 as libc::c_int;
    }
    let mut old_buf_size = (*s.as_deref().unwrap()).string_table.n;
    let mut e_ident: [libc::c_char; 16] = [0; 16];
    if fread(
        core::ptr::addr_of_mut!(e_ident) as *mut [libc::c_char; 16] as *mut libc::c_void,
        16 as libc::c_int as size_t,
        1 as libc::c_int as size_t,
        fptr,
    ) != 1 as libc::c_int as libc::c_ulong
    {
        fclose(fptr);
        return 11 as libc::c_int;
    }
    if e_ident[0 as libc::c_int as usize] as libc::c_int != 0x7f as libc::c_int
        || e_ident[1 as libc::c_int as usize] as libc::c_int != 'E' as i32
        || e_ident[2 as libc::c_int as usize] as libc::c_int != 'L' as i32
        || e_ident[3 as libc::c_int as usize] as libc::c_int != 'F' as i32
    {
        fclose(fptr);
        return 11 as libc::c_int;
    }
    if e_ident[4 as libc::c_int as usize] as libc::c_int != 1 as libc::c_int
        && e_ident[4 as libc::c_int as usize] as libc::c_int != 2 as libc::c_int
    {
        fclose(fptr);
        return 12 as libc::c_int;
    }
    if e_ident[5 as libc::c_int as usize] as libc::c_int != '\u{1}' as i32
        && e_ident[5 as libc::c_int as usize] as libc::c_int != '\u{2}' as i32
    {
        fclose(fptr);
        return 13 as libc::c_int;
    }
    let mut curr_type = {
        let mut init = compat_t {
            any: 0 as libc::c_int as libc::c_char,
            class: e_ident[4 as libc::c_int as usize] as uint8_t,
            machine: 0,
        };
        init
    };
    let mut is_little_endian = (e_ident[5 as libc::c_int as usize] as libc::c_int
        == '\u{1}' as i32) as libc::c_int;
    if compat.any == 0 && compat.class as libc::c_int != curr_type.class as libc::c_int {
        fclose(fptr);
        return 15 as libc::c_int;
    }
    if is_little_endian ^ host_is_little_endian() != 0 {
        fclose(fptr);
        return 16 as libc::c_int;
    }
    let mut header = C2RustUnnamed_1 {
        h64: header_64_t {
            e_type: 0,
            e_machine: 0,
            e_version: 0,
            e_entry: 0,
            e_phoff: 0,
            e_shoff: 0,
            e_flags: 0,
            e_ehsize: 0,
            e_phentsize: 0,
            e_phnum: 0,
            e_shentsize: 0,
            e_shnum: 0,
            e_shstrndx: 0,
        },
    };
    if curr_type.class as libc::c_int == 2 as libc::c_int {
        if fread(
            core::ptr::addr_of_mut!(header.h64) as *mut header_64_t as *mut libc::c_void,
            ::std::mem::size_of::<header_64_t>() as libc::c_ulong,
            1 as libc::c_int as size_t,
            fptr,
        ) != 1 as libc::c_int as libc::c_ulong
        {
            fclose(fptr);
            return 14 as libc::c_int;
        }
        if header.h64.e_type as libc::c_int != 2 as libc::c_int
            && header.h64.e_type as libc::c_int != 3 as libc::c_int
        {
            fclose(fptr);
            return 17 as libc::c_int;
        }
        curr_type.machine= header.h64.e_machine;
        if compat.any == 0
            && compat.machine as libc::c_int != curr_type.machine as libc::c_int
        {
            fclose(fptr);
            return 32 as libc::c_int;
        }
        if fseek(fptr, header.h64.e_phoff as libc::c_long, 0 as libc::c_int)
            != 0 as libc::c_int
        {
            fclose(fptr);
            return 18 as libc::c_int;
        }
    } else {
        if fread(
            core::ptr::addr_of_mut!(header.h32) as *mut header_32_t as *mut libc::c_void,
            ::std::mem::size_of::<header_32_t>() as libc::c_ulong,
            1 as libc::c_int as size_t,
            fptr,
        ) != 1 as libc::c_int as libc::c_ulong
        {
            fclose(fptr);
            return 14 as libc::c_int;
        }
        if header.h32.e_type as libc::c_int != 2 as libc::c_int
            && header.h32.e_type as libc::c_int != 3 as libc::c_int
        {
            fclose(fptr);
            return 17 as libc::c_int;
        }
        curr_type.machine= header.h32.e_machine;
        if compat.any == 0
            && compat.machine as libc::c_int != curr_type.machine as libc::c_int
        {
            fclose(fptr);
            return 32 as libc::c_int;
        }
        if fseek(fptr, header.h32.e_phoff as libc::c_long, 0 as libc::c_int)
            != 0 as libc::c_int
        {
            fclose(fptr);
            return 18 as libc::c_int;
        }
    }
    let mut prog = C2RustUnnamed_0 {
        p64: prog_64_t {
            p_type: 0,
            p_flags: 0,
            p_offset: 0,
            p_vaddr: 0,
            p_paddr: 0,
            p_filesz: 0,
            p_memsz: 0,
            p_align: 0,
        },
    };
    let mut pt_load_offset = small_vec_u64_t {
        buf: [0; 16],
        p: 0 as *mut uint64_t,
        n: 0,
        capacity: 0,
    };
    let mut pt_load_vaddr = small_vec_u64_t {
        buf: [0; 16],
        p: 0 as *mut uint64_t,
        n: 0,
        capacity: 0,
    };
    small_vec_u64_init(core::ptr::addr_of_mut!(pt_load_offset));
    small_vec_u64_init(core::ptr::addr_of_mut!(pt_load_vaddr));
    let mut p_offset = 0xffffffffffffffff as libc::c_ulong;
    if curr_type.class as libc::c_int == 2 as libc::c_int {
        let mut i = 0 as libc::c_int as uint64_t;
        while i < header.h64.e_phnum as libc::c_ulong {
            if fread(
                core::ptr::addr_of_mut!(prog.p64) as *mut prog_64_t as *mut libc::c_void,
                ::std::mem::size_of::<prog_64_t>() as libc::c_ulong,
                1 as libc::c_int as size_t,
                fptr,
            ) != 1 as libc::c_int as libc::c_ulong
            {
                fclose(fptr);
                small_vec_u64_free(core::ptr::addr_of_mut!(pt_load_offset));
                small_vec_u64_free(core::ptr::addr_of_mut!(pt_load_vaddr));
                return 19 as libc::c_int;
            }
            if prog.p64.p_type == 1 as libc::c_int as libc::c_uint {
                small_vec_u64_append(core::ptr::addr_of_mut!(pt_load_offset), prog.p64.p_offset);
                small_vec_u64_append(core::ptr::addr_of_mut!(pt_load_vaddr), prog.p64.p_vaddr);
            } else if prog.p64.p_type == 2 as libc::c_int as libc::c_uint {
                p_offset= prog.p64.p_offset;
            }
            i= i.wrapping_add(1);
        }
    } else {
        let mut i_0 = 0 as libc::c_int as uint32_t;
        while i_0 < header.h32.e_phnum as libc::c_uint {
            if fread(
                core::ptr::addr_of_mut!(prog.p32) as *mut prog_32_t as *mut libc::c_void,
                ::std::mem::size_of::<prog_32_t>() as libc::c_ulong,
                1 as libc::c_int as size_t,
                fptr,
            ) != 1 as libc::c_int as libc::c_ulong
            {
                fclose(fptr);
                small_vec_u64_free(core::ptr::addr_of_mut!(pt_load_offset));
                small_vec_u64_free(core::ptr::addr_of_mut!(pt_load_vaddr));
                return 19 as libc::c_int;
            }
            if prog.p32.p_type == 1 as libc::c_int as libc::c_uint {
                small_vec_u64_append(core::ptr::addr_of_mut!(pt_load_offset), prog.p32.p_offset as uint64_t);
                small_vec_u64_append(core::ptr::addr_of_mut!(pt_load_vaddr), prog.p32.p_vaddr as uint64_t);
            } else if prog.p32.p_type == 2 as libc::c_int as libc::c_uint {
                p_offset= prog.p32.p_offset as uint64_t;
            }
            i_0= i_0.wrapping_add(1);
        }
    }
    let mut finfo = stat {
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
        st_atime: 0,
        st_atimensec: 0,
        st_mtime: 0,
        st_mtimensec: 0,
        st_ctime: 0,
        st_ctimensec: 0,
        __glibc_reserved: [0; 3],
    };
    if stat(current_file, core::ptr::addr_of_mut!(finfo)) != 0 as libc::c_int {
        fclose(fptr);
        small_vec_u64_free(core::ptr::addr_of_mut!(pt_load_offset));
        small_vec_u64_free(core::ptr::addr_of_mut!(pt_load_vaddr));
        return 20 as libc::c_int;
    }
    let mut seen_before = visited_files_contains(core::ptr::addr_of_mut!((*s.as_deref_mut().unwrap()).visited), core::ptr::addr_of_mut!(finfo));
    if seen_before == 0 {
        visited_files_append(Some(&mut (*s.as_deref_mut().unwrap()).visited), core::ptr::addr_of_mut!(finfo));
    }
    if p_offset == 0xffffffffffffffff as libc::c_ulong {
        print_line(
            depth,
            current_file,
            b"\x1B[1;36m\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\x1B[0;36m\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            1 as libc::c_int,
            reason,
            s.as_deref_mut(),
        );
        fclose(fptr);
        small_vec_u64_free(core::ptr::addr_of_mut!(pt_load_offset));
        small_vec_u64_free(core::ptr::addr_of_mut!(pt_load_vaddr));
        return 0 as libc::c_int;
    }
    if pt_load_offset.n == 0 as libc::c_int as libc::c_ulong {
        fclose(fptr);
        small_vec_u64_free(core::ptr::addr_of_mut!(pt_load_offset));
        small_vec_u64_free(core::ptr::addr_of_mut!(pt_load_vaddr));
        return 29 as libc::c_int;
    }
    if fseek(fptr, p_offset as libc::c_long, 0 as libc::c_int) != 0 as libc::c_int {
        fclose(fptr);
        small_vec_u64_free(core::ptr::addr_of_mut!(pt_load_offset));
        small_vec_u64_free(core::ptr::addr_of_mut!(pt_load_vaddr));
        return 21 as libc::c_int;
    }
    let mut no_def_lib = 0 as libc::c_int;
    let mut strtab = 0xffffffffffffffff as libc::c_ulong;
    let mut rpath = 0xffffffffffffffff as libc::c_ulong;
    let mut runpath = 0xffffffffffffffff as libc::c_ulong;
    let mut soname = 0xffffffffffffffff as libc::c_ulong;
    let mut needed = small_vec_u64_t {
        buf: [0; 16],
        p: 0 as *mut uint64_t,
        n: 0,
        capacity: 0,
    };
    small_vec_u64_init(core::ptr::addr_of_mut!(needed));
    let mut cont = 1 as libc::c_int;
    while cont != 0 {
        let mut d_tag: uint64_t = 0;
        let mut d_val: uint64_t = 0;
        if curr_type.class as libc::c_int == 2 as libc::c_int {
            let mut dyn_0 = dyn_64_t { d_tag: 0, d_val: 0 };
            if fread(
                core::ptr::addr_of_mut!(dyn_0) as *mut dyn_64_t as *mut libc::c_void,
                ::std::mem::size_of::<dyn_64_t>() as libc::c_ulong,
                1 as libc::c_int as size_t,
                fptr,
            ) != 1 as libc::c_int as libc::c_ulong
            {
                fclose(fptr);
                small_vec_u64_free(core::ptr::addr_of_mut!(pt_load_offset));
                small_vec_u64_free(core::ptr::addr_of_mut!(pt_load_vaddr));
                small_vec_u64_free(core::ptr::addr_of_mut!(needed));
                return 22 as libc::c_int;
            }
            d_tag= dyn_0.d_tag as uint64_t;
            d_val= dyn_0.d_val;
        } else {
            let mut dyn_1 = dyn_32_t { d_tag: 0, d_val: 0 };
            if fread(
                core::ptr::addr_of_mut!(dyn_1) as *mut dyn_32_t as *mut libc::c_void,
                ::std::mem::size_of::<dyn_32_t>() as libc::c_ulong,
                1 as libc::c_int as size_t,
                fptr,
            ) != 1 as libc::c_int as libc::c_ulong
            {
                fclose(fptr);
                small_vec_u64_free(core::ptr::addr_of_mut!(pt_load_offset));
                small_vec_u64_free(core::ptr::addr_of_mut!(pt_load_vaddr));
                small_vec_u64_free(core::ptr::addr_of_mut!(needed));
                return 22 as libc::c_int;
            }
            d_tag= dyn_1.d_tag as uint64_t;
            d_val= dyn_1.d_val as uint64_t;
        }
        match d_tag {
            0 => {
                cont= 0 as libc::c_int;
            }
            5 => {
                strtab= d_val;
            }
            15 => {
                rpath= d_val;
            }
            29 => {
                runpath= d_val;
            }
            1 => {
                small_vec_u64_append(core::ptr::addr_of_mut!(needed), d_val);
            }
            14 => {
                soname= d_val;
            }
            1879048187 => {
                no_def_lib|= (0x800 as libc::c_int as libc::c_ulong & d_val
                        == 0x800 as libc::c_int as libc::c_ulong) as libc::c_int;
            }
            _ => {}
        }
    }
    if strtab == 0xffffffffffffffff as libc::c_ulong {
        fclose(fptr);
        small_vec_u64_free(core::ptr::addr_of_mut!(pt_load_offset));
        small_vec_u64_free(core::ptr::addr_of_mut!(pt_load_vaddr));
        small_vec_u64_free(core::ptr::addr_of_mut!(needed));
        return 23 as libc::c_int;
    }
    if is_ascending_order(pt_load_vaddr.p, pt_load_vaddr.n) == 0 {
        fclose(fptr);
        small_vec_u64_free(core::ptr::addr_of_mut!(pt_load_vaddr));
        small_vec_u64_free(core::ptr::addr_of_mut!(pt_load_offset));
        small_vec_u64_free(core::ptr::addr_of_mut!(needed));
        return 30 as libc::c_int;
    }
    let mut vaddr_idx = 0 as libc::c_int as size_t;
    while vaddr_idx.wrapping_add(1 as libc::c_int as libc::c_ulong) != pt_load_vaddr.n
        && strtab
            >= *pt_load_vaddr.p
                .offset(
                    vaddr_idx.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                )
    {
        vaddr_idx= vaddr_idx.wrapping_add(1);
    }
    let mut strtab_offset = (*pt_load_offset.p.offset(vaddr_idx as isize))
        .wrapping_add(strtab)
        .wrapping_sub(*pt_load_vaddr.p.offset(vaddr_idx as isize));
    small_vec_u64_free(core::ptr::addr_of_mut!(pt_load_vaddr));
    small_vec_u64_free(core::ptr::addr_of_mut!(pt_load_offset));
    let mut soname_buf_offset = (*s.as_deref().unwrap()).string_table.n;
    if soname != 0xffffffffffffffff as libc::c_ulong {
        if fseek(
            fptr,
            strtab_offset.wrapping_add(soname) as libc::c_long,
            0 as libc::c_int,
        ) != 0 as libc::c_int
        {
            (*s.as_deref_mut().unwrap()).string_table.n= old_buf_size;
            fclose(fptr);
            small_vec_u64_free(core::ptr::addr_of_mut!(needed));
            return 24 as libc::c_int;
        }
        string_table_copy_from_file(Some(&mut (*s.as_deref_mut().unwrap()).string_table), fptr);
    }
    let mut in_exclude_list = (soname != 0xffffffffffffffff as libc::c_ulong
        && is_in_exclude_list((*s.as_deref().unwrap()).string_table.arr.offset(soname_buf_offset as isize))
            != 0) as libc::c_int;
    let mut should_recurse = (depth < (*s.as_deref().unwrap()).max_depth
        && (seen_before == 0 && in_exclude_list == 0
            || seen_before == 0 && in_exclude_list != 0
                && (*s.as_deref().unwrap()).verbosity >= 2 as libc::c_int
            || (*s.as_deref().unwrap()).verbosity >= 3 as libc::c_int)) as libc::c_int;
    if should_recurse == 0 {
        let mut print_name = if soname == 0xffffffffffffffff as libc::c_ulong
            || (*s.as_deref().unwrap()).path != 0
        {
            current_file
        } else {
            (*s.as_deref().unwrap()).string_table.arr.offset(soname_buf_offset as isize)
        };
        let mut bold_color = (if in_exclude_list != 0 {
            b"\x1B[0;35m\0" as *const u8 as *const libc::c_char
        } else if seen_before != 0 {
            b"\x1B[0;34m\0" as *const u8 as *const libc::c_char
        } else {
            b"\x1B[1;36m\0" as *const u8 as *const libc::c_char
        }) as *mut libc::c_char;
        let mut regular_color = (if in_exclude_list != 0 {
            b"\x1B[0;35m\0" as *const u8 as *const libc::c_char
        } else if seen_before != 0 {
            b"\x1B[0;34m\0" as *const u8 as *const libc::c_char
        } else {
            b"\x1B[0;36m\0" as *const u8 as *const libc::c_char
        }) as *mut libc::c_char;
        let mut highlight = (seen_before == 0 && in_exclude_list == 0) as libc::c_int;
        print_line(depth, print_name, bold_color, regular_color, highlight, reason, s.as_deref_mut());
        (*s.as_deref_mut().unwrap()).string_table.n= old_buf_size;
        fclose(fptr);
        small_vec_u64_free(core::ptr::addr_of_mut!(needed));
        return 0 as libc::c_int;
    }
    let mut origin: [libc::c_char; 4096] = [0; 4096];
    let mut last_slash = strrchr(current_file, '/' as i32);
    if !last_slash.is_null() {
        let mut bytes = last_slash.offset_from(current_file) as libc::c_long as size_t;
        memcpy(
            origin.as_mut_ptr() as *mut libc::c_void,
            current_file as *const libc::c_void,
            bytes,
        );
        origin[bytes as usize]= '\0' as i32 as libc::c_char;
    } else {();
        memcpy(
            origin.as_mut_ptr() as *mut libc::c_void,
            b"./\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            3 as libc::c_int as libc::c_ulong,
        );
    }
    if rpath == 0xffffffffffffffff as libc::c_ulong {
        (*s.as_deref_mut().unwrap()).rpath_offsets[depth as usize]= 18446744073709551615 as libc::c_ulong;
    } else {
        (*s.as_deref_mut().unwrap()).rpath_offsets[depth as usize]= (*s.as_deref().unwrap()).string_table.n;
        if fseek(
            fptr,
            strtab_offset.wrapping_add(rpath) as libc::c_long,
            0 as libc::c_int,
        ) != 0 as libc::c_int
        {
            (*s.as_deref_mut().unwrap()).string_table.n= old_buf_size;
            fclose(fptr);
            small_vec_u64_free(core::ptr::addr_of_mut!(needed));
            return 25 as libc::c_int;
        }
        string_table_copy_from_file(Some(&mut (*s.as_deref_mut().unwrap()).string_table), fptr);
        let mut curr_buf_size = (*s.as_deref().unwrap()).string_table.n;
        if {let crown_promoted_local_0 = (*s.as_deref().unwrap()).rpath_offsets[depth as usize];interpolate_variables(
            s.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()),
            crown_promoted_local_0,
            origin.as_mut_ptr(),
        )} != 0
        {
            (*s.as_deref_mut().unwrap()).rpath_offsets[depth as usize]= curr_buf_size;
        }
    }
    let mut runpath_buf_offset = (*s.as_deref().unwrap()).string_table.n;
    if runpath != 0xffffffffffffffff as libc::c_ulong {
        if fseek(
            fptr,
            strtab_offset.wrapping_add(runpath) as libc::c_long,
            0 as libc::c_int,
        ) != 0 as libc::c_int
        {
            (*s.as_deref_mut().unwrap()).string_table.n= old_buf_size;
            fclose(fptr);
            small_vec_u64_free(core::ptr::addr_of_mut!(needed));
            return 26 as libc::c_int;
        }
        string_table_copy_from_file(Some(&mut (*s.as_deref_mut().unwrap()).string_table), fptr);
        let mut curr_buf_size_0 = (*s.as_deref().unwrap()).string_table.n;
        if interpolate_variables(s.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()), runpath_buf_offset, origin.as_mut_ptr()) != 0 {
            runpath_buf_offset= curr_buf_size_0;
        }
    }
    let mut needed_buf_offsets = small_vec_u64_t {
        buf: [0; 16],
        p: 0 as *mut uint64_t,
        n: 0,
        capacity: 0,
    };
    small_vec_u64_init(core::ptr::addr_of_mut!(needed_buf_offsets));
    let mut i_1 = 0 as libc::c_int as size_t;
    while i_1 < needed.n {
        small_vec_u64_append(core::ptr::addr_of_mut!(needed_buf_offsets), (*s.as_deref().unwrap()).string_table.n);
        if fseek(
            fptr,
            strtab_offset.wrapping_add(*needed.p.offset(i_1 as isize)) as libc::c_long,
            0 as libc::c_int,
        ) != 0 as libc::c_int
        {
            (*s.as_deref_mut().unwrap()).string_table.n= old_buf_size;
            fclose(fptr);
            small_vec_u64_free(core::ptr::addr_of_mut!(needed_buf_offsets));
            small_vec_u64_free(core::ptr::addr_of_mut!(needed));
            return 27 as libc::c_int;
        }
        string_table_copy_from_file(Some(&mut (*s.as_deref_mut().unwrap()).string_table), fptr);
        i_1= i_1.wrapping_add(1);
    }
    fclose(fptr);
    let mut print_name_0 = if soname == 0xffffffffffffffff as libc::c_ulong
        || (*s.as_deref().unwrap()).path != 0
    {
        current_file
    } else {
        (*s.as_deref().unwrap()).string_table.arr.offset(soname_buf_offset as isize)
    };
    let mut bold_color_0 = (if in_exclude_list != 0 {
        b"\x1B[0;35m\0" as *const u8 as *const libc::c_char
    } else if seen_before != 0 {
        b"\x1B[0;34m\0" as *const u8 as *const libc::c_char
    } else {
        b"\x1B[1;36m\0" as *const u8 as *const libc::c_char
    }) as *mut libc::c_char;
    let mut regular_color_0 = (if in_exclude_list != 0 {
        b"\x1B[0;35m\0" as *const u8 as *const libc::c_char
    } else if seen_before != 0 {
        b"\x1B[0;34m\0" as *const u8 as *const libc::c_char
    } else {
        b"\x1B[0;36m\0" as *const u8 as *const libc::c_char
    }) as *mut libc::c_char;
    let mut highlight_0 = (seen_before == 0 && in_exclude_list == 0) as libc::c_int;
    print_line(
        depth,
        print_name_0,
        bold_color_0,
        regular_color_0,
        highlight_0,
        reason,
        s.as_deref_mut(),
    );
    let mut exit_code = 0 as libc::c_int;
    let mut needed_not_found = needed_buf_offsets.n;
    if needed_not_found != 0 && (*s.as_deref().unwrap()).verbosity == 0 as libc::c_int {
        apply_exclude_list(Some(&mut needed_not_found), core::ptr::addr_of_mut!(needed_buf_offsets), s.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()));
    }
    if needed_not_found != 0 {
        exit_code|= check_absolute_paths(
                Some(&mut needed_not_found),
                core::ptr::addr_of_mut!(needed_buf_offsets),
                depth,
                s.as_deref_mut(),
                curr_type,
            );
    }
    if runpath == 0xffffffffffffffff as libc::c_ulong {
        let mut j = depth as libc::c_int;
        while j >= 0 as libc::c_int && needed_not_found != 0 {
            if !((*s.as_deref().unwrap()).rpath_offsets[j as usize] == 18446744073709551615 as libc::c_ulong)
            {
                exit_code|= check_search_paths(
                        {
                            let mut init = found_t {
                                how: RPATH,
                                depth: j as size_t,
                            };
                            init
                        },
                        (*s.as_deref().unwrap()).rpath_offsets[j as usize],
                        Some(&mut needed_not_found),
                        core::ptr::addr_of_mut!(needed_buf_offsets),
                        depth,
                        s.as_deref_mut(),
                        curr_type,
                    );
            }
            j-= 1;
        }
    }
    if needed_not_found != 0
        && (*s.as_deref().unwrap()).ld_library_path_offset != 18446744073709551615 as libc::c_ulong
    {
        exit_code|= check_search_paths(
                {
                    let mut init = found_t {
                        how: LD_LIBRARY_PATH,
                        depth: 0,
                    };
                    init
                },
                (*s.as_deref().unwrap()).ld_library_path_offset,
                Some(&mut needed_not_found),
                core::ptr::addr_of_mut!(needed_buf_offsets),
                depth,
                s.as_deref_mut(),
                curr_type,
            );
    }
    if needed_not_found != 0 && runpath != 0xffffffffffffffff as libc::c_ulong {
        exit_code|= check_search_paths(
                {
                    let mut init = found_t { how: RUNPATH, depth: 0 };
                    init
                },
                runpath_buf_offset,
                Some(&mut needed_not_found),
                core::ptr::addr_of_mut!(needed_buf_offsets),
                depth,
                s.as_deref_mut(),
                curr_type,
            );
    }
    if needed_not_found != 0 && no_def_lib == 0 {
        exit_code|= check_search_paths(
                {
                    let mut init = found_t {
                        how: LD_SO_CONF,
                        depth: 0,
                    };
                    init
                },
                (*s.as_deref().unwrap()).ld_so_conf_offset,
                Some(&mut needed_not_found),
                core::ptr::addr_of_mut!(needed_buf_offsets),
                depth,
                s.as_deref_mut(),
                curr_type,
            );
    }
    if needed_not_found != 0 && no_def_lib == 0 {
        exit_code|= check_search_paths(
                {
                    let mut init = found_t { how: DEFAULT, depth: 0 };
                    init
                },
                (*s.as_deref().unwrap()).default_paths_offset,
                Some(&mut needed_not_found),
                core::ptr::addr_of_mut!(needed_buf_offsets),
                depth,
                s.as_deref_mut(),
                curr_type,
            );
    }
    if needed_not_found != 0 {
        print_error(
            depth,
            needed_not_found,
            core::ptr::addr_of_mut!(needed_buf_offsets),
            if runpath == 0xffffffffffffffff as libc::c_ulong {
                0 as *mut libc::c_char
            } else {
                (*s.as_deref().unwrap()).string_table.arr.offset(runpath_buf_offset as isize)
            },
            s.as_deref_mut(),
            no_def_lib,
        );
        (*s.as_deref_mut().unwrap()).string_table.n= old_buf_size;
        small_vec_u64_free(core::ptr::addr_of_mut!(needed_buf_offsets));
        small_vec_u64_free(core::ptr::addr_of_mut!(needed));
        return 28 as libc::c_int;
    }
    (*s.as_deref_mut().unwrap()).string_table.n= old_buf_size;
    small_vec_u64_free(core::ptr::addr_of_mut!(needed_buf_offsets));
    small_vec_u64_free(core::ptr::addr_of_mut!(needed));
    return exit_code;
}
unsafe extern "C" fn ld_conf_globbing(
    mut st: Option<&mut string_table_t>,
    mut pattern: *mut libc::c_char,
) -> libc::c_int {
    let mut result = glob_t {
        gl_pathc: 0,
        gl_pathv: 0 as *mut *mut libc::c_char,
        gl_offs: 0,
        gl_flags: 0,
        gl_closedir: None,
        gl_readdir: None,
        gl_opendir: None,
        gl_lstat: None,
        gl_stat: None,
    };
    memset(
        core::ptr::addr_of_mut!(result) as *mut glob_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<glob_t>() as libc::c_ulong,
    );
    let mut status = glob(pattern, 0 as libc::c_int, None, core::ptr::addr_of_mut!(result));
    match status {
        1 | 2 => {
            globfree(core::ptr::addr_of_mut!(result));
            return 1 as libc::c_int;
        }
        3 => {
            globfree(core::ptr::addr_of_mut!(result));
            return 0 as libc::c_int;
        }
        _ => {}
    }
    let mut code = 0 as libc::c_int;
    let mut i = 0 as libc::c_int as size_t;
    while i < result.gl_pathc {
        code|= parse_ld_config_file(st.as_deref_mut(), *result.gl_pathv.offset(i as isize));
        i= i.wrapping_add(1);
    }
    globfree(core::ptr::addr_of_mut!(result));
    return code;
}
unsafe extern "C" fn parse_ld_config_file(
    mut st: Option<&mut string_table_t>,
    mut path: *mut libc::c_char,
) -> libc::c_int {
    let mut fptr = fopen(path, b"r\0" as *const u8 as *const libc::c_char);
    if fptr.is_null() {();
        return 1 as libc::c_int;
    }
    let mut c = 0 as libc::c_int;
    let mut line: [libc::c_char; 4096] = [0; 4096];
    let mut tmp: [libc::c_char; 4096] = [0; 4096];
    while c != -(1 as libc::c_int) {
        let mut line_len = 0 as libc::c_int as size_t;
        loop {
            c= _IO_getc(fptr);
            if !(c != '\n' as i32 && c != -(1 as libc::c_int)) {
                break;
            }
            if line_len < (4096 as libc::c_int - 1 as libc::c_int) as libc::c_ulong {
                let fresh25 = line_len;
                line_len= line_len.wrapping_add(1);
                line[fresh25 as usize]= c as libc::c_char;
            }
        }
        line[line_len as usize]= '\0' as i32 as libc::c_char;
        let mut begin = line.as_mut_ptr();
        let mut end = line.as_mut_ptr().offset(line_len as isize);
        while *(*__ctype_b_loc()).offset((*begin) as libc::c_int as isize) as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            begin= begin.offset(1);
        }
        let mut comment = strchr(begin, '#' as i32);
        if !comment.is_null() {
            *comment= '\0' as i32 as libc::c_char;
        }else { (); }
        while end != begin {
            end= end.offset(-1);
            if *(*__ctype_b_loc()).offset((*end) as libc::c_int as isize) as libc::c_int
                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int == 0
            {
                break;
            }
        }
        if begin == end {
            continue;
        }
        *end.offset(1 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
        if strncmp(
            begin,
            b"include\0" as *const u8 as *const libc::c_char,
            7 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
            && *(*__ctype_b_loc())
                .offset(*begin.offset(7 as libc::c_int as isize) as libc::c_int as isize)
                as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                != 0
        {
            begin= begin.offset(8 as libc::c_int as isize);
            while *(*__ctype_b_loc()).offset((*begin) as libc::c_int as isize)
                as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                != 0
            {
                begin= begin.offset(1);
            }
            if (*begin) as libc::c_int != '/' as i32 {
                let mut wd = strrchr(path, '/' as i32);
                wd= if wd.is_null() {(); strrchr(path, '\0' as i32) } else { wd };
                let mut wd_len = wd.offset_from(path) as libc::c_long as size_t;
                let mut include_len = (end.offset_from(begin) as libc::c_long
                    + 1 as libc::c_int as libc::c_long) as size_t;
                if wd_len
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(include_len) >= 4096 as libc::c_int as libc::c_ulong
                {
                    continue;
                }
                memcpy(
                    tmp.as_mut_ptr() as *mut libc::c_void,
                    path as *const libc::c_void,
                    wd_len,
                );
                tmp[wd_len as usize]= '/' as i32 as libc::c_char;
                memcpy(
                    tmp.as_mut_ptr()
                        .offset(wd_len as isize)
                        .offset(1 as libc::c_int as isize) as *mut libc::c_void,
                    begin as *const libc::c_void,
                    include_len,
                );
                tmp[wd_len
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(include_len) as usize]= '\0' as i32 as libc::c_char;
                begin= tmp.as_mut_ptr();
            }
            ld_conf_globbing(st.as_deref_mut(), begin);
        } else {
            string_table_store(st.as_deref_mut(), begin);
            *(*st.as_deref().unwrap()).arr
                .offset(
                    (*st.as_deref().unwrap()).n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                ) = ':' as i32 as libc::c_char;
        }
    }
    fclose(fptr);
    return 0 as libc::c_int;
}
unsafe extern "C" fn parse_ld_so_conf(mut s: *mut libtree_state_t) {
    let mut st: *mut string_table_t = core::ptr::addr_of_mut!((*s).string_table);
    (*s).ld_so_conf_offset= (*st).n;
    parse_ld_config_file(st.as_mut(), (*s).ld_conf_file);
    if (*st).n > (*s).ld_so_conf_offset {
        *(*st).arr
            .offset(
                (*st).n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
            ) = '\0' as i32 as libc::c_char;
    } else {
        string_table_store(st.as_mut(), b"\0" as *const u8 as *const libc::c_char);
    };
}
unsafe extern "C" fn parse_ld_library_path(mut s: Option<&mut libtree_state_t>) {
    (*s.as_deref_mut().unwrap()).ld_library_path_offset= 18446744073709551615 as libc::c_ulong;
    let mut val = getenv(b"LD_LIBRARY_PATH\0" as *const u8 as *const libc::c_char);
    if val.is_null() {();
        return;
    }
    (*s.as_deref_mut().unwrap()).ld_library_path_offset= (*s.as_deref().unwrap()).string_table.n;
    string_table_store(Some(&mut (*s.as_deref_mut().unwrap()).string_table), val);
    let mut search = (*s.as_deref().unwrap()).string_table.arr
        .offset((*s.as_deref().unwrap()).ld_library_path_offset as isize);
    loop {
        search= strchr(search, ';' as i32);
        if search.is_null() {();
            break;
        }
        let fresh26 = search;
        search= search.offset(1);
        *fresh26= ':' as i32 as libc::c_char;
    };
}
unsafe extern "C" fn set_default_paths(mut s: Option<&mut libtree_state_t>) {
    (*s.as_deref_mut().unwrap()).default_paths_offset= (*s.as_deref().unwrap()).string_table.n;
    string_table_store(
        Some(&mut (*s.as_deref_mut().unwrap()).string_table),
        b"/lib:/lib64:/usr/lib:/usr/lib64\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn libtree_state_init(mut s: Option<&mut libtree_state_t>) {
    (*s.as_deref_mut().unwrap()).string_table.n= 0 as libc::c_int as size_t;
    (*s.as_deref_mut().unwrap()).string_table.capacity= 1024 as libc::c_int as size_t;
    (*s.as_deref_mut().unwrap()).string_table.arr= malloc(
        (*s.as_deref().unwrap()).string_table.capacity
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
    ) as *mut libc::c_char;
    (*s.as_deref_mut().unwrap()).visited.n= 0 as libc::c_int as size_t;
    (*s.as_deref_mut().unwrap()).visited.capacity= 256 as libc::c_int as size_t;
    (*s.as_deref_mut().unwrap()).visited.arr= malloc(
        (*s.as_deref().unwrap()).visited.capacity
            .wrapping_mul(::std::mem::size_of::<visited_file_t>() as libc::c_ulong),
    ) as *mut visited_file_t;
}
unsafe extern "C" fn libtree_state_free(mut s: Option<&mut libtree_state_t>) {
    free((*s.as_deref().unwrap()).string_table.arr as *mut libc::c_void);
    free((*s.as_deref().unwrap()).visited.arr as *mut libc::c_void);
}
unsafe extern "C" fn print_tree(
    mut pathc: libc::c_int,
    mut pathv: *mut *mut libc::c_char,
    mut s: Option<&mut libtree_state_t>,
) -> libc::c_int {
    libtree_state_init(s.as_deref_mut());
    parse_ld_so_conf(s.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()));
    parse_ld_library_path(s.as_deref_mut());
    set_default_paths(s.as_deref_mut());
    let mut exit_code = 0 as libc::c_int;
    let mut i = 0 as libc::c_int;
    while i < pathc {
        let mut code = recurse(
            *pathv.offset(i as isize),
            0 as libc::c_int as size_t,
            s.as_deref_mut(),
            {
                let mut init = compat_t {
                    any: 1 as libc::c_int as libc::c_char,
                    class: 0,
                    machine: 0,
                };
                init
            },
            {
                let mut init = found_t { how: INPUT, depth: 0 };
                init
            },
        );
        fflush(crate::src::libtree::stdout);
        if code != 0 as libc::c_int {
            exit_code= code;
            fputs(b"Error [\0" as *const u8 as *const libc::c_char, crate::src::libtree::stderr);
            fputs(*pathv.offset(i as isize), crate::src::libtree::stderr);
            fputs(b"]: \0" as *const u8 as *const libc::c_char, crate::src::libtree::stderr);
        }
        let mut msg = 0 as *mut libc::c_char;
        match code {
            11 => {
                msg= b"Invalid ELF magic bytes\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
            }
            12 => {
                msg= b"Invalid ELF class\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
            }
            13 => {
                msg= b"Invalid ELF data\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
            }
            14 => {
                msg= b"Invalid ELF header\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
            }
            15 => {
                msg= b"Invalid bits\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
            }
            16 => {
                msg= b"Invalid endianness\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
            }
            17 => {
                msg= b"Not an ET_EXEC or ET_DYN ELF file\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char;
            }
            18 => {
                msg= b"Invalid ELF program header offset\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char;
            }
            19 => {
                msg= b"Invalid ELF program header\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char;
            }
            20 => {
                msg= b"Can't stat file\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
            }
            21 => {
                msg= b"Invalid ELF dynamic section\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char;
            }
            22 => {
                msg= b"Invalid ELF dynamic array entry\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char;
            }
            23 => {
                msg= b"No ELF string table found\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char;
            }
            24 => {
                msg= b"Can't read DT_SONAME\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
            }
            25 => {
                msg= b"Can't read DT_RPATH\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
            }
            26 => {
                msg= b"Can't read DT_RUNPATH\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
            }
            27 => {
                msg= b"Can't read DT_NEEDED\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
            }
            28 => {
                msg= b"Not all dependencies were found\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char;
            }
            29 => {
                msg= b"No PT_LOAD found in ELF file\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char;
            }
            30 => {
                msg= b"Virtual addresses are not ordered\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char;
            }
            31 => {
                msg= b"Could not open file\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
            }
            32 => {
                msg= b"Incompatible ISA\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
            }
            _ => {}
        }
        if !msg.is_null() {
            fputs(msg, crate::src::libtree::stderr);
        }else { (); }
        fflush(crate::src::libtree::stderr);
        i+= 1;
    }
    libtree_state_free(s.as_deref_mut());
    return exit_code;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut s = libtree_state_t {
        verbosity: 0,
        path: 0,
        color: 0,
        ld_conf_file: 0 as *mut libc::c_char,
        max_depth: 0,
        string_table: string_table_t {
            arr: 0 as *mut libc::c_char,
            n: 0,
            capacity: 0,
        },
        visited: visited_file_array_t {
            arr: 0 as *mut visited_file_t,
            n: 0,
            capacity: 0,
        },
        PLATFORM: 0 as *mut libc::c_char,
        LIB: 0 as *mut libc::c_char,
        OSNAME: 0 as *mut libc::c_char,
        OSREL: 0 as *mut libc::c_char,
        rpath_offsets: [0; 32],
        ld_library_path_offset: 0,
        default_paths_offset: 0,
        ld_so_conf_offset: 0,
        found_all_needed: [0; 32],
    };
    s.color= ((getenv(b"NO_COLOR\0" as *const u8 as *const libc::c_char)).is_null()
        && isatty(1 as libc::c_int) != 0) as libc::c_int;
    s.verbosity= 0 as libc::c_int;
    s.path= 0 as libc::c_int;
    s.max_depth= 32 as libc::c_int as libc::c_ulong;
    let mut positional = 1 as libc::c_int;
    let mut uname_val = utsname {
        sysname: [0; 65],
        nodename: [0; 65],
        release: [0; 65],
        version: [0; 65],
        machine: [0; 65],
        __domainname: [0; 65],
    };
    if uname(core::ptr::addr_of_mut!(uname_val)) != 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    s.PLATFORM= uname_val.machine.as_mut_ptr();
    s.OSNAME= uname_val.sysname.as_mut_ptr();
    s.OSREL= uname_val.release.as_mut_ptr();
    s.ld_conf_file= b"/etc/ld.so.conf\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    if strcmp(
        uname_val.sysname.as_mut_ptr(),
        b"FreeBSD\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        s.ld_conf_file= b"/etc/ld-elf.so.conf\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
    }
    s.LIB= b"lib\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    let mut opt_help = 0 as libc::c_int;
    let mut opt_version = 0 as libc::c_int;
    let mut opt_raw = 0 as libc::c_int;
    let mut i = 1 as libc::c_int;
    while i < argc {
        let mut arg = *argv.offset(i as isize);
        if opt_raw != 0 || (*arg) as libc::c_int != '-' as i32
            || *arg.offset(1 as libc::c_int as isize) as libc::c_int == '\0' as i32
        {
            let fresh29 = positional;
            positional= positional + 1;
            *argv.offset(fresh29 as isize) = arg;
        } else {
            arg= arg.offset(1);
            if (*arg) as libc::c_int == '-' as i32 {
                arg= arg.offset(1);
                if (*arg) as libc::c_int == '\0' as i32 {
                    opt_raw= 1 as libc::c_int;
                } else if strcmp(arg, b"version\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    opt_version= 1 as libc::c_int;
                } else if strcmp(arg, b"path\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    s.path= 1 as libc::c_int;
                } else if strcmp(arg, b"verbose\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    s.verbosity+= 1;
                } else if strcmp(arg, b"help\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    opt_help= 1 as libc::c_int;
                } else if strcmp(arg, b"ldconf\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    if i + 1 as libc::c_int == argc {
                        fputs(
                            b"Expected value after `--ldconf`\n\0" as *const u8
                                as *const libc::c_char,
                            crate::src::libtree::stderr,
                        );
                        return 1 as libc::c_int;
                    }
                    i+= 1;
                    s.ld_conf_file= *argv.offset(i as isize);
                } else if strcmp(arg, b"max-depth\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    if i + 1 as libc::c_int == argc {
                        fputs(
                            b"Expected value after `--max-depth`\n\0" as *const u8
                                as *const libc::c_char,
                            crate::src::libtree::stderr,
                        );
                        return 1 as libc::c_int;
                    }
                    let mut ptr = 0 as *mut libc::c_char;
                    i+= 1;
                    s.max_depth= strtoul(
                        *argv.offset(i as isize),
                        core::ptr::addr_of_mut!(ptr),
                        10 as libc::c_int,
                    );
                    if s.max_depth > 32 as libc::c_int as libc::c_ulong {
                        s.max_depth= 32 as libc::c_int as libc::c_ulong;
                    }
                } else {
                    fputs(
                        b"Unrecognized flag `--\0" as *const u8 as *const libc::c_char,
                        crate::src::libtree::stderr,
                    );
                    fputs(arg, crate::src::libtree::stderr);
                    fputs(b"`\n\0" as *const u8 as *const libc::c_char, crate::src::libtree::stderr);
                    return 1 as libc::c_int;
                }
            } else {
                while (*arg) as libc::c_int != '\0' as i32 {
                    match  (*arg) as libc::c_int {
                        104 => {
                            opt_help= 1 as libc::c_int;
                        }
                        112 => {
                            s.path= 1 as libc::c_int;
                        }
                        118 => {
                            s.verbosity+= 1;
                        }
                        _ => {
                            fputs(
                                b"Unrecognized flag `-\0" as *const u8
                                    as *const libc::c_char,
                                crate::src::libtree::stderr,
                            );
                            fputs(arg, crate::src::libtree::stderr);
                            fputs(b"`\n\0" as *const u8 as *const libc::c_char, crate::src::libtree::stderr);
                            return 1 as libc::c_int;
                        }
                    }
                    arg= arg.offset(1);
                }
            }
        }
        i+= 1;
    }
    argv= argv.offset(1);
    positional-= 1;
    if opt_help != 0 || opt_version == 0 && positional == 0 as libc::c_int {
        fputs(
            b"Show the dynamic dependency tree of ELF files\nUsage: libtree [OPTION]... [--] FILE [FILES]...\n\n  -h, --help     Print help info\n      --version  Print version info\n\nFile names starting with '-', for example '-.so', can be specified as follows:\n  libtree -- -.so\n\nLocating libs options:\n  -p, --path       Show the path of libraries instead of the soname\n  -v               Show libraries skipped by default*\n  -vv              Show dependencies of libraries skipped by default*\n  -vvv             Show dependencies of already encountered libraries\n  --ldconf <path>  Config file for extra search paths [\0"
                as *const u8 as *const libc::c_char,
            crate::src::libtree::stdout,
        );
        fputs(s.ld_conf_file, crate::src::libtree::stdout);
        fputs(
            b"]\n  --max-depth <n>  Limit library traversal to at most n levels of depth\n\n* For brevity, the following libraries are not shown by default:\n  \0"
                as *const u8 as *const libc::c_char,
            crate::src::libtree::stdout,
        );
        let mut num_excluded = (::std::mem::size_of::<[*const libc::c_char; 14]>()
            as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong);
        let mut cursor_x = 3 as libc::c_int as size_t;
        let mut j = 0 as libc::c_int as size_t;
        while j < num_excluded {
            cursor_x= (cursor_x as libc::c_ulong)
                .wrapping_add(strlen(crate::src::libtree::exclude_list[j as usize])) as size_t as size_t;
            if cursor_x > 60 as libc::c_int as libc::c_ulong {
                cursor_x= 3 as libc::c_int as size_t;
                fputs(b"\n  \0" as *const u8 as *const libc::c_char, crate::src::libtree::stdout);
            }
            fputs(crate::src::libtree::exclude_list[j as usize], crate::src::libtree::stdout);
            if j.wrapping_add(1 as libc::c_int as libc::c_ulong) != num_excluded {
                fputs(b", \0" as *const u8 as *const libc::c_char, crate::src::libtree::stdout);
            }
            j= j.wrapping_add(1);
        }
        fputs(
            b".\n\nThe following rpath/runpath substitutions are used:\n\0" as *const u8
                as *const libc::c_char,
            crate::src::libtree::stdout,
        );
        fputs(b"  PLATFORM       \0" as *const u8 as *const libc::c_char, crate::src::libtree::stdout);
        fputs(s.PLATFORM, crate::src::libtree::stdout);
        fputs(b"\n  LIB            \0" as *const u8 as *const libc::c_char, crate::src::libtree::stdout);
        fputs(s.LIB, crate::src::libtree::stdout);
        fputs(b"\n  OSNAME         \0" as *const u8 as *const libc::c_char, crate::src::libtree::stdout);
        fputs(s.OSNAME, crate::src::libtree::stdout);
        fputs(b"\n  OSREL          \0" as *const u8 as *const libc::c_char, crate::src::libtree::stdout);
        fputs(s.OSREL, crate::src::libtree::stdout);
        putchar('\n' as i32);
        return (opt_help == 0) as libc::c_int;
    }
    if opt_version != 0 {
        puts(b"3.1.1\0" as *const u8 as *const libc::c_char);
        return 0 as libc::c_int;
    }
    return print_tree(positional, argv, Some(&mut s));
}
// pub fn main() {
//     let mut args: Vec::<*mut libc::c_char> = Vec::new();
//     for arg in ::std::env::args() {
//         args.push(
//             (::std::ffi::CString::new(arg))
//                 .expect("Failed to convert argument into CString.")
//                 .into_raw(),
//         );
//     }
//     args.push(::std::ptr::null_mut());
//     unsafe {
//         ::std::process::exit(
//             main_0(
//                 (args.len() - 1) as libc::c_int,
//                 args.as_mut_ptr() as *mut *mut libc::c_char,
//             ) as i32,
//         )
//     }
// }
