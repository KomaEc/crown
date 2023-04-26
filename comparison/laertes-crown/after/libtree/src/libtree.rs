
extern "C" {
    static mut stdout: * mut crate::src::libtree::_IO_FILE;
    static mut stderr: * mut crate::src::libtree::_IO_FILE;
    fn fclose(__stream: * mut crate::src::libtree::_IO_FILE) -> std::os::raw::c_int;
    fn fflush(__stream: * mut crate::src::libtree::_IO_FILE) -> std::os::raw::c_int;
    fn fopen(__filename: * const std::os::raw::c_char, __modes: * const std::os::raw::c_char) -> * mut crate::src::libtree::_IO_FILE;
    fn fputs(__s: * const std::os::raw::c_char, __stream: * mut crate::src::libtree::_IO_FILE) -> std::os::raw::c_int;
    fn _IO_getc(__fp: * mut crate::src::libtree::_IO_FILE) -> std::os::raw::c_int;
    fn _IO_putc(__c: std::os::raw::c_int, __fp: * mut crate::src::libtree::_IO_FILE) -> std::os::raw::c_int;
    fn puts(__s: * const std::os::raw::c_char) -> std::os::raw::c_int;
    fn fread(
        __ptr: * mut core::ffi::c_void,
        __size: std::os::raw::c_ulong,
        __n: std::os::raw::c_ulong,
        __stream: * mut crate::src::libtree::_IO_FILE,
    ) -> std::os::raw::c_ulong;
    fn fwrite(
        __ptr: * const core::ffi::c_void,
        __size: std::os::raw::c_ulong,
        __n: std::os::raw::c_ulong,
        __s: * mut crate::src::libtree::_IO_FILE,
    ) -> std::os::raw::c_ulong;
    fn fseek(
        __stream: * mut crate::src::libtree::_IO_FILE,
        __off: std::os::raw::c_long,
        __whence: std::os::raw::c_int,
    ) -> std::os::raw::c_int;
    fn strtoul(
        __nptr: * const std::os::raw::c_char,
        __endptr: * mut * mut std::os::raw::c_char,
        __base: std::os::raw::c_int,
    ) -> std::os::raw::c_ulong;
    fn malloc(_: std::os::raw::c_ulong) -> * mut core::ffi::c_void;
    fn realloc(_: * mut core::ffi::c_void, _: std::os::raw::c_ulong) -> * mut core::ffi::c_void;
    fn free(__ptr: * mut core::ffi::c_void);
    fn exit(_: std::os::raw::c_int) -> !;
    fn getenv(__name: * const std::os::raw::c_char) -> * mut std::os::raw::c_char;
    fn memcpy(
        _: * mut core::ffi::c_void,
        _: * const core::ffi::c_void,
        _: std::os::raw::c_ulong,
    ) -> * mut core::ffi::c_void;
    fn memset(
        _: * mut core::ffi::c_void,
        _: std::os::raw::c_int,
        _: std::os::raw::c_ulong,
    ) -> * mut core::ffi::c_void;
    fn strcpy(_: * mut std::os::raw::c_char, _: * const std::os::raw::c_char) -> * mut std::os::raw::c_char;
    fn strcmp(_: * const std::os::raw::c_char, _: * const std::os::raw::c_char) -> std::os::raw::c_int;
    fn strncmp(
        _: * const std::os::raw::c_char,
        _: * const std::os::raw::c_char,
        _: std::os::raw::c_ulong,
    ) -> std::os::raw::c_int;
    fn strchr(_: * const std::os::raw::c_char, _: std::os::raw::c_int) -> * mut std::os::raw::c_char;
    fn strrchr(_: * const std::os::raw::c_char, _: std::os::raw::c_int) -> * mut std::os::raw::c_char;
    fn strlen(_: * const std::os::raw::c_char) -> std::os::raw::c_ulong;
    fn __ctype_b_loc() -> * mut * const std::os::raw::c_ushort;
    fn glob(
        __pattern: * const std::os::raw::c_char,
        __flags: std::os::raw::c_int,
        __errfunc: Option<unsafe extern "C"  fn(_: * const std::os::raw::c_char,_: std::os::raw::c_int,) -> std::os::raw::c_int>,
        __pglob: * mut crate::src::libtree::glob_t,
    ) -> std::os::raw::c_int;
    fn globfree(__pglob: * mut crate::src::libtree::glob_t);
    fn __xstat(
        __ver: std::os::raw::c_int,
        __filename: * const std::os::raw::c_char,
        __stat_buf: * mut crate::src::libtree::stat,
    ) -> std::os::raw::c_int;
    fn uname(__name: * mut crate::src::libtree::utsname) -> std::os::raw::c_int;
    fn isatty(__fd: std::os::raw::c_int) -> std::os::raw::c_int;
}
pub type __uint8_t = std::os::raw::c_uchar;
pub type __uint16_t = std::os::raw::c_ushort;
pub type __int32_t = std::os::raw::c_int;
pub type __uint32_t = std::os::raw::c_uint;
pub type __int64_t = std::os::raw::c_long;
pub type __uint64_t = std::os::raw::c_ulong;
pub type __dev_t = std::os::raw::c_ulong;
pub type __uid_t = std::os::raw::c_uint;
pub type __gid_t = std::os::raw::c_uint;
pub type __ino_t = std::os::raw::c_ulong;
pub type __ino64_t = std::os::raw::c_ulong;
pub type __mode_t = std::os::raw::c_uint;
pub type __nlink_t = std::os::raw::c_ulong;
pub type __off_t = std::os::raw::c_long;
pub type __off64_t = std::os::raw::c_long;
pub type __time_t = std::os::raw::c_long;
pub type __blksize_t = std::os::raw::c_long;
pub type __blkcnt_t = std::os::raw::c_long;
pub type __syscall_slong_t = std::os::raw::c_long;
pub type __syscall_ulong_t = std::os::raw::c_ulong;
pub type int32_t = std::os::raw::c_int;
pub type int64_t = std::os::raw::c_long;
pub type uint8_t = std::os::raw::c_uchar;
pub type uint16_t = std::os::raw::c_ushort;
pub type uint32_t = std::os::raw::c_uint;
pub type uint64_t = std::os::raw::c_ulong;
pub type size_t = std::os::raw::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: std::os::raw::c_int,
    pub _IO_read_ptr: * mut std::os::raw::c_char,
    pub _IO_read_end: * mut std::os::raw::c_char,
    pub _IO_read_base: * mut std::os::raw::c_char,
    pub _IO_write_base: * mut std::os::raw::c_char,
    pub _IO_write_ptr: * mut std::os::raw::c_char,
    pub _IO_write_end: * mut std::os::raw::c_char,
    pub _IO_buf_base: * mut std::os::raw::c_char,
    pub _IO_buf_end: * mut std::os::raw::c_char,
    pub _IO_save_base: * mut std::os::raw::c_char,
    pub _IO_backup_base: * mut std::os::raw::c_char,
    pub _IO_save_end: * mut std::os::raw::c_char,
    pub _markers: * mut crate::src::libtree::_IO_marker,
    pub _chain: * mut crate::src::libtree::_IO_FILE,
    pub _fileno: std::os::raw::c_int,
    pub _flags2: std::os::raw::c_int,
    pub _old_offset: std::os::raw::c_long,
    pub _cur_column: std::os::raw::c_ushort,
    pub _vtable_offset: std::os::raw::c_schar,
    pub _shortbuf: [std::os::raw::c_char; 1],
    pub _lock: * mut core::ffi::c_void,
    pub _offset: std::os::raw::c_long,
    pub __pad1: * mut core::ffi::c_void,
    pub __pad2: * mut core::ffi::c_void,
    pub __pad3: * mut core::ffi::c_void,
    pub __pad4: * mut core::ffi::c_void,
    pub __pad5: std::os::raw::c_ulong,
    pub _mode: std::os::raw::c_int,
    pub _unused2: [std::os::raw::c_char; 20],
}
impl _IO_FILE {
    pub const fn new() -> Self {
        _IO_FILE {
        _flags: 0,
        _IO_read_ptr: (0 as * mut std::os::raw::c_char),
        _IO_read_end: (0 as * mut std::os::raw::c_char),
        _IO_read_base: (0 as * mut std::os::raw::c_char),
        _IO_write_base: (0 as * mut std::os::raw::c_char),
        _IO_write_ptr: (0 as * mut std::os::raw::c_char),
        _IO_write_end: (0 as * mut std::os::raw::c_char),
        _IO_buf_base: (0 as * mut std::os::raw::c_char),
        _IO_buf_end: (0 as * mut std::os::raw::c_char),
        _IO_save_base: (0 as * mut std::os::raw::c_char),
        _IO_backup_base: (0 as * mut std::os::raw::c_char),
        _IO_save_end: (0 as * mut std::os::raw::c_char),
        _markers: (0 as * mut crate::src::libtree::_IO_marker),
        _chain: (0 as * mut crate::src::libtree::_IO_FILE),
        _fileno: 0,
        _flags2: 0,
        _old_offset: 0,
        _cur_column: 0,
        _vtable_offset: 0,
        _shortbuf: [0,],
        _lock: (0 as * mut core::ffi::c_void),
        _offset: 0,
        __pad1: (0 as * mut core::ffi::c_void),
        __pad2: (0 as * mut core::ffi::c_void),
        __pad3: (0 as * mut core::ffi::c_void),
        __pad4: (0 as * mut core::ffi::c_void),
        __pad5: 0,
        _mode: 0,
        _unused2: [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,]
        }
    }
}

impl std::default::Default for _IO_FILE {
    fn default() -> Self { _IO_FILE::new() }
}

pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: * mut crate::src::libtree::_IO_marker,
    pub _sbuf: * mut crate::src::libtree::_IO_FILE,
    pub _pos: std::os::raw::c_int,
}
impl _IO_marker {
    pub const fn new() -> Self {
        _IO_marker {
        _next: (0 as * mut crate::src::libtree::_IO_marker),
        _sbuf: (0 as * mut crate::src::libtree::_IO_FILE),
        _pos: 0
        }
    }
}

impl std::default::Default for _IO_marker {
    fn default() -> Self { _IO_marker::new() }
}

pub type FILE = crate::src::libtree::_IO_FILE;
pub type C2RustUnnamed = std::os::raw::c_uint;
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
pub type __size_t = std::os::raw::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glob_t {
    pub gl_pathc: std::os::raw::c_ulong,
    pub gl_pathv: * mut * mut std::os::raw::c_char,
    pub gl_offs: std::os::raw::c_ulong,
    pub gl_flags: std::os::raw::c_int,
    pub gl_closedir: Option<unsafe extern "C"  fn(_: * mut core::ffi::c_void,) -> ()>,
    pub gl_readdir: Option<unsafe extern "C"  fn(_: * mut core::ffi::c_void,) -> * mut core::ffi::c_void>,
    pub gl_opendir: Option<unsafe extern "C"  fn(_: * const std::os::raw::c_char,) -> * mut core::ffi::c_void>,
    pub gl_lstat: Option<unsafe extern "C"  fn(_: * const std::os::raw::c_char,_: * mut core::ffi::c_void,) -> std::os::raw::c_int>,
    pub gl_stat: Option<unsafe extern "C"  fn(_: * const std::os::raw::c_char,_: * mut core::ffi::c_void,) -> std::os::raw::c_int>,
}
impl glob_t {
    pub const fn new() -> Self {
        glob_t {
        gl_pathc: 0,
        gl_pathv: (0 as * mut * mut std::os::raw::c_char),
        gl_offs: 0,
        gl_flags: 0,
        gl_closedir: None,
        gl_readdir: None,
        gl_opendir: None,
        gl_lstat: None,
        gl_stat: None
        }
    }
}

impl std::default::Default for glob_t {
    fn default() -> Self { glob_t::new() }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: std::os::raw::c_ulong,
    pub st_ino: std::os::raw::c_ulong,
    pub st_nlink: std::os::raw::c_ulong,
    pub st_mode: std::os::raw::c_uint,
    pub st_uid: std::os::raw::c_uint,
    pub st_gid: std::os::raw::c_uint,
    pub __pad0: std::os::raw::c_int,
    pub st_rdev: std::os::raw::c_ulong,
    pub st_size: std::os::raw::c_long,
    pub st_blksize: std::os::raw::c_long,
    pub st_blocks: std::os::raw::c_long,
    pub st_atime: std::os::raw::c_long,
    pub st_atimensec: std::os::raw::c_ulong,
    pub st_mtime: std::os::raw::c_long,
    pub st_mtimensec: std::os::raw::c_ulong,
    pub st_ctime: std::os::raw::c_long,
    pub st_ctimensec: std::os::raw::c_ulong,
    pub __glibc_reserved: [std::os::raw::c_long; 3],
}
impl stat {
    pub const fn new() -> Self {
        stat {
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
        __glibc_reserved: [0,0,0,]
        }
    }
}

impl std::default::Default for stat {
    fn default() -> Self { stat::new() }
}

pub type ino_t = std::os::raw::c_ulong;
pub type dev_t = std::os::raw::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct utsname {
    pub sysname: [std::os::raw::c_char; 65],
    pub nodename: [std::os::raw::c_char; 65],
    pub release: [std::os::raw::c_char; 65],
    pub version: [std::os::raw::c_char; 65],
    pub machine: [std::os::raw::c_char; 65],
    pub __domainname: [std::os::raw::c_char; 65],
}
impl utsname {
    pub const fn new() -> Self {
        utsname {
        sysname: [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
        nodename: [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
        release: [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
        version: [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
        machine: [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
        __domainname: [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,]
        }
    }
}

impl std::default::Default for utsname {
    fn default() -> Self { utsname::new() }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct header_64_t {
    pub e_type: std::os::raw::c_ushort,
    pub e_machine: std::os::raw::c_ushort,
    pub e_version: std::os::raw::c_uint,
    pub e_entry: std::os::raw::c_ulong,
    pub e_phoff: std::os::raw::c_ulong,
    pub e_shoff: std::os::raw::c_ulong,
    pub e_flags: std::os::raw::c_uint,
    pub e_ehsize: std::os::raw::c_ushort,
    pub e_phentsize: std::os::raw::c_ushort,
    pub e_phnum: std::os::raw::c_ushort,
    pub e_shentsize: std::os::raw::c_ushort,
    pub e_shnum: std::os::raw::c_ushort,
    pub e_shstrndx: std::os::raw::c_ushort,
}
impl header_64_t {
    pub const fn new() -> Self {
        header_64_t {
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
        e_shstrndx: 0
        }
    }
}

impl std::default::Default for header_64_t {
    fn default() -> Self { header_64_t::new() }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct header_32_t {
    pub e_type: std::os::raw::c_ushort,
    pub e_machine: std::os::raw::c_ushort,
    pub e_version: std::os::raw::c_uint,
    pub e_entry: std::os::raw::c_uint,
    pub e_phoff: std::os::raw::c_uint,
    pub e_shoff: std::os::raw::c_uint,
    pub e_flags: std::os::raw::c_uint,
    pub e_ehsize: std::os::raw::c_ushort,
    pub e_phentsize: std::os::raw::c_ushort,
    pub e_phnum: std::os::raw::c_ushort,
    pub e_shentsize: std::os::raw::c_ushort,
    pub e_shnum: std::os::raw::c_ushort,
    pub e_shstrndx: std::os::raw::c_ushort,
}
impl header_32_t {
    pub const fn new() -> Self {
        header_32_t {
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
        e_shstrndx: 0
        }
    }
}

impl std::default::Default for header_32_t {
    fn default() -> Self { header_32_t::new() }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct prog_64_t {
    pub p_type: std::os::raw::c_uint,
    pub p_flags: std::os::raw::c_uint,
    pub p_offset: std::os::raw::c_ulong,
    pub p_vaddr: std::os::raw::c_ulong,
    pub p_paddr: std::os::raw::c_ulong,
    pub p_filesz: std::os::raw::c_ulong,
    pub p_memsz: std::os::raw::c_ulong,
    pub p_align: std::os::raw::c_ulong,
}
impl prog_64_t {
    pub const fn new() -> Self {
        prog_64_t {
        p_type: 0,
        p_flags: 0,
        p_offset: 0,
        p_vaddr: 0,
        p_paddr: 0,
        p_filesz: 0,
        p_memsz: 0,
        p_align: 0
        }
    }
}

impl std::default::Default for prog_64_t {
    fn default() -> Self { prog_64_t::new() }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct prog_32_t {
    pub p_type: std::os::raw::c_uint,
    pub p_offset: std::os::raw::c_uint,
    pub p_vaddr: std::os::raw::c_uint,
    pub p_paddr: std::os::raw::c_uint,
    pub p_filesz: std::os::raw::c_uint,
    pub p_memsz: std::os::raw::c_uint,
    pub p_flags: std::os::raw::c_uint,
    pub p_align: std::os::raw::c_uint,
}
impl prog_32_t {
    pub const fn new() -> Self {
        prog_32_t {
        p_type: 0,
        p_offset: 0,
        p_vaddr: 0,
        p_paddr: 0,
        p_filesz: 0,
        p_memsz: 0,
        p_flags: 0,
        p_align: 0
        }
    }
}

impl std::default::Default for prog_32_t {
    fn default() -> Self { prog_32_t::new() }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct dyn_64_t {
    pub d_tag: std::os::raw::c_long,
    pub d_val: std::os::raw::c_ulong,
}
impl dyn_64_t {
    pub const fn new() -> Self {
        dyn_64_t {
        d_tag: 0,
        d_val: 0
        }
    }
}

impl std::default::Default for dyn_64_t {
    fn default() -> Self { dyn_64_t::new() }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct dyn_32_t {
    pub d_tag: std::os::raw::c_int,
    pub d_val: std::os::raw::c_uint,
}
impl dyn_32_t {
    pub const fn new() -> Self {
        dyn_32_t {
        d_tag: 0,
        d_val: 0
        }
    }
}

impl std::default::Default for dyn_32_t {
    fn default() -> Self { dyn_32_t::new() }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct compat_t {
    pub any: std::os::raw::c_char,
    pub class: std::os::raw::c_uchar,
    pub machine: std::os::raw::c_ushort,
}
impl compat_t {
    pub const fn new() -> Self {
        compat_t {
        any: 0,
        class: 0,
        machine: 0
        }
    }
}

impl std::default::Default for compat_t {
    fn default() -> Self { compat_t::new() }
}

pub type how_t = std::os::raw::c_uint;
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
    pub how: std::os::raw::c_uint,
    pub depth: std::os::raw::c_ulong,
}
impl found_t {
    pub const fn new() -> Self {
        found_t {
        how: 0,
        depth: 0
        }
    }
}

impl std::default::Default for found_t {
    fn default() -> Self { found_t::new() }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct string_table_t {
    pub arr: * mut std::os::raw::c_char,
    pub n: std::os::raw::c_ulong,
    pub capacity: std::os::raw::c_ulong,
}
impl string_table_t {
    pub const fn new() -> Self {
        string_table_t {
        arr: (0 as * mut std::os::raw::c_char),
        n: 0,
        capacity: 0
        }
    }
}

impl std::default::Default for string_table_t {
    fn default() -> Self { string_table_t::new() }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct visited_file_t {
    pub st_dev: std::os::raw::c_ulong,
    pub st_ino: std::os::raw::c_ulong,
}
impl visited_file_t {
    pub const fn new() -> Self {
        visited_file_t {
        st_dev: 0,
        st_ino: 0
        }
    }
}

impl std::default::Default for visited_file_t {
    fn default() -> Self { visited_file_t::new() }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct visited_file_array_t {
    pub arr: * mut crate::src::libtree::visited_file_t,
    pub n: std::os::raw::c_ulong,
    pub capacity: std::os::raw::c_ulong,
}
impl visited_file_array_t {
    pub const fn new() -> Self {
        visited_file_array_t {
        arr: (0 as * mut crate::src::libtree::visited_file_t),
        n: 0,
        capacity: 0
        }
    }
}

impl std::default::Default for visited_file_array_t {
    fn default() -> Self { visited_file_array_t::new() }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct libtree_state_t {
    pub verbosity: std::os::raw::c_int,
    pub path: std::os::raw::c_int,
    pub color: std::os::raw::c_int,
    pub ld_conf_file: * mut std::os::raw::c_char,
    pub max_depth: std::os::raw::c_ulong,
    pub string_table: crate::src::libtree::string_table_t,
    pub visited: crate::src::libtree::visited_file_array_t,
    pub PLATFORM: * mut std::os::raw::c_char,
    pub LIB: * mut std::os::raw::c_char,
    pub OSNAME: * mut std::os::raw::c_char,
    pub OSREL: * mut std::os::raw::c_char,
    pub rpath_offsets: [std::os::raw::c_ulong; 32],
    pub ld_library_path_offset: std::os::raw::c_ulong,
    pub default_paths_offset: std::os::raw::c_ulong,
    pub ld_so_conf_offset: std::os::raw::c_ulong,
    pub found_all_needed: [std::os::raw::c_char; 32],
}
impl libtree_state_t {
    pub const fn new() -> Self {
        libtree_state_t {
        verbosity: 0,
        path: 0,
        color: 0,
        ld_conf_file: (0 as * mut std::os::raw::c_char),
        max_depth: 0,
        string_table: crate::src::libtree::string_table_t::new(),
        visited: crate::src::libtree::visited_file_array_t::new(),
        PLATFORM: (0 as * mut std::os::raw::c_char),
        LIB: (0 as * mut std::os::raw::c_char),
        OSNAME: (0 as * mut std::os::raw::c_char),
        OSREL: (0 as * mut std::os::raw::c_char),
        rpath_offsets: [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
        ld_library_path_offset: 0,
        default_paths_offset: 0,
        ld_so_conf_offset: 0,
        found_all_needed: [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,]
        }
    }
}

impl std::default::Default for libtree_state_t {
    fn default() -> Self { libtree_state_t::new() }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct small_vec_u64_t {
    pub buf: [std::os::raw::c_ulong; 16],
    pub p: * mut std::os::raw::c_ulong,
    pub n: std::os::raw::c_ulong,
    pub capacity: std::os::raw::c_ulong,
}
impl small_vec_u64_t {
    pub const fn new() -> Self {
        small_vec_u64_t {
        buf: [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
        p: (0 as * mut std::os::raw::c_ulong),
        n: 0,
        capacity: 0
        }
    }
}

impl std::default::Default for small_vec_u64_t {
    fn default() -> Self { small_vec_u64_t::new() }
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
unsafe extern "C" fn putchar(mut __c: std::os::raw::c_int) -> std::os::raw::c_int {
    return _IO_putc(__c, stdout);
}
#[inline]
unsafe extern "C" fn stat(
    mut __path: * const std::os::raw::c_char,
    mut __statbuf: * mut crate::src::libtree::stat,
) -> std::os::raw::c_int {
    return __xstat(1 as std::os::raw::c_int, __path, __statbuf);
}
#[no_mangle]
pub static mut exclude_list: [* const std::os::raw::c_char; 14] = [(0 as * const std::os::raw::c_char),(0 as * const std::os::raw::c_char),(0 as * const std::os::raw::c_char),(0 as * const std::os::raw::c_char),(0 as * const std::os::raw::c_char),(0 as * const std::os::raw::c_char),(0 as * const std::os::raw::c_char),(0 as * const std::os::raw::c_char),(0 as * const std::os::raw::c_char),(0 as * const std::os::raw::c_char),(0 as * const std::os::raw::c_char),(0 as * const std::os::raw::c_char),(0 as * const std::os::raw::c_char),(0 as * const std::os::raw::c_char),]; unsafe fn laertes_init_exclude_list() {
exclude_list = [
    b"ld-linux-aarch64.so\0" as *const u8 as *const std::os::raw::c_char,
    b"ld-linux-armhf.so\0" as *const u8 as *const std::os::raw::c_char,
    b"ld-linux-x86-64.so\0" as *const u8 as *const std::os::raw::c_char,
    b"ld-linux.so\0" as *const u8 as *const std::os::raw::c_char,
    b"ld64.so\0" as *const u8 as *const std::os::raw::c_char,
    b"libc.musl-aarch64.so\0" as *const u8 as *const std::os::raw::c_char,
    b"libc.musl-armhf.so\0" as *const u8 as *const std::os::raw::c_char,
    b"libc.musl-i386.so\0" as *const u8 as *const std::os::raw::c_char,
    b"libc.musl-x86_64.so\0" as *const u8 as *const std::os::raw::c_char,
    b"libc.so\0" as *const u8 as *const std::os::raw::c_char,
    b"libdl.so\0" as *const u8 as *const std::os::raw::c_char,
    b"libgcc_s.so\0" as *const u8 as *const std::os::raw::c_char,
    b"libm.so\0" as *const u8 as *const std::os::raw::c_char,
    b"libstdc++.so\0" as *const u8 as *const std::os::raw::c_char,
];}//;
#[inline]
unsafe extern "C" fn utoa(mut str: * mut std::os::raw::c_char, mut v: std::os::raw::c_ulong) {
    let mut p = str;
    loop {
        let mut fresh0 = p;
        p = p.offset(1);
        *fresh0 = ('0' as i32 as std::os::raw::c_ulong)
            .wrapping_add(v.wrapping_rem(10 as std::os::raw::c_int as std::os::raw::c_ulong))
            as std::os::raw::c_char;
        v = (v as std::os::raw::c_ulong).wrapping_div(10 as std::os::raw::c_int as std::os::raw::c_ulong)
            as size_t as size_t;
        if !(v > 0 as std::os::raw::c_int as std::os::raw::c_ulong) {
            break;
        }
    }
    let mut len = p.offset_from(str) as std::os::raw::c_long as size_t;
    let mut i = 0 as std::os::raw::c_int as size_t;
    while i < len.wrapping_div(2 as std::os::raw::c_int as std::os::raw::c_ulong) {
        let mut tmp = *str.offset(i as isize);
        *str
            .offset(
                i as isize,
            ) = *str
            .offset(
                len.wrapping_sub(i).wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong)
                    as isize,
            );
        *str
            .offset(
                len.wrapping_sub(i).wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong)
                    as isize,
            ) = tmp;
        i = i.wrapping_add(1);
    }
    *str.offset(len as isize) = '\0' as i32 as std::os::raw::c_char;
}
#[inline]
unsafe extern "C" fn small_vec_u64_init(mut v: * mut crate::src::libtree::small_vec_u64_t) {
    memset(
        v as *mut std::os::raw::c_void,
        0 as std::os::raw::c_int,
        ::std::mem::size_of::<small_vec_u64_t>() as std::os::raw::c_ulong,
    );
    let ref mut fresh1 = (*v).p;
    *fresh1 = ((*v).buf).as_mut_ptr();
}
unsafe extern "C" fn small_vec_u64_append(
    mut v: * mut crate::src::libtree::small_vec_u64_t,
    mut val: std::os::raw::c_ulong,
) {
    if (*v).n < 16 as std::os::raw::c_int as std::os::raw::c_ulong {
        let ref mut fresh2 = (*v).n;
        let mut fresh3 = *fresh2;
        *fresh2 = (*fresh2).wrapping_add(1);
        *((*v).p).offset(fresh3 as isize) = val;
        return;
    }
    if (*v).n == 16 as std::os::raw::c_int as std::os::raw::c_ulong {
        (*v).capacity = (2 as std::os::raw::c_int * 16 as std::os::raw::c_int) as size_t;
        let ref mut fresh4 = (*v).p;
        *fresh4 = malloc(
            ((*v).capacity)
                .wrapping_mul(::std::mem::size_of::<uint64_t>() as std::os::raw::c_ulong),
        ) as *mut uint64_t;
        if ((*v).p).is_null() {
            exit(1 as std::os::raw::c_int);
        }
        memcpy(
            (*v).p as *mut std::os::raw::c_void,
            ((*v).buf).as_mut_ptr() as *const std::os::raw::c_void,
            (16 as std::os::raw::c_int as std::os::raw::c_ulong)
                .wrapping_mul(::std::mem::size_of::<uint64_t>() as std::os::raw::c_ulong),
        );
    } else if (*v).n == (*v).capacity {
        let ref mut fresh5 = (*v).capacity;
        *fresh5 = (*fresh5 as std::os::raw::c_ulong)
            .wrapping_mul(2 as std::os::raw::c_int as std::os::raw::c_ulong) as size_t as size_t;
        let mut p = realloc(
            (*v).p as *mut std::os::raw::c_void,
            ((*v).capacity)
                .wrapping_mul(::std::mem::size_of::<uint64_t>() as std::os::raw::c_ulong),
        ) as *mut uint64_t;
        if p.is_null() {
            exit(1 as std::os::raw::c_int);
        }
        let ref mut fresh6 = (*v).p;
        *fresh6 = p;
    }
    let ref mut fresh7 = (*v).n;
    let mut fresh8 = *fresh7;
    *fresh7 = (*fresh7).wrapping_add(1);
    *((*v).p).offset(fresh8 as isize) = val;
}
unsafe extern "C" fn small_vec_u64_free<'a1>(mut v: Option<&'a1 mut crate::src::libtree::small_vec_u64_t>) {
    if (*(borrow(& v)).unwrap()).n <= 16 as std::os::raw::c_int as std::os::raw::c_ulong {
        return;
    }
    free((*(borrow_mut(&mut v)).unwrap()).p as *mut std::os::raw::c_void);
    let ref mut fresh9 = (*(borrow_mut(&mut v)).unwrap()).p;
    *fresh9 = 0 as *mut uint64_t;
}
#[inline]
unsafe extern "C" fn host_is_little_endian() -> std::os::raw::c_int {
    let mut test = 1 as std::os::raw::c_int;
    let mut bytes = &mut test as *mut std::os::raw::c_int as *mut std::os::raw::c_char;
    return (*bytes.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int == 1 as std::os::raw::c_int)
        as std::os::raw::c_int;
}
unsafe extern "C" fn is_ascending_order(
    mut v: * mut std::os::raw::c_ulong,
    mut n: std::os::raw::c_ulong,
) -> std::os::raw::c_int {
    let mut j = 1 as std::os::raw::c_int as size_t;
    while j < n {
        if *v.offset(j.wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize)
            >= *v.offset(j as isize)
        {
            return 0 as std::os::raw::c_int;
        }
        j = j.wrapping_add(1);
    }
    return 1 as std::os::raw::c_int;
}
unsafe extern "C" fn string_table_maybe_grow<'a1>(mut t: Option<&'a1 mut crate::src::libtree::string_table_t>, mut n: std::os::raw::c_ulong) {
    if ((*(borrow(& t)).unwrap()).n).wrapping_add(n) <= (*(borrow(& t)).unwrap()).capacity {
        return;
    }
    (*(borrow_mut(&mut t)).unwrap())
        .capacity = (2 as std::os::raw::c_int as std::os::raw::c_ulong)
        .wrapping_mul(((*(borrow(& t)).unwrap()).n).wrapping_add(n));
    let mut arr = realloc(
        (*(borrow_mut(&mut t)).unwrap()).arr as *mut std::os::raw::c_void,
        ((*(borrow(& t)).unwrap()).capacity)
            .wrapping_mul(::std::mem::size_of::<std::os::raw::c_char>() as std::os::raw::c_ulong),
    ) as *mut std::os::raw::c_char;
    if arr.is_null() {
        exit(1 as std::os::raw::c_int);
    }
    let ref mut fresh10 = (*(borrow_mut(&mut t)).unwrap()).arr;
    *fresh10 = arr;
}
unsafe extern "C" fn string_table_store<'a1>(
    mut t: Option<&'a1 mut crate::src::libtree::string_table_t>,
    mut str: * const std::os::raw::c_char,
) {
    let mut n = (strlen(str)).wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong);
    string_table_maybe_grow(borrow_mut(&mut t), n);
    memcpy(
        ((*(borrow(& t)).unwrap()).arr).offset((*(borrow(& t)).unwrap()).n as isize) as *mut std::os::raw::c_void,
        str as *const std::os::raw::c_void,
        n,
    );
    let ref mut fresh11 = (*(borrow_mut(&mut t)).unwrap()).n;
    *fresh11 = (*fresh11 as std::os::raw::c_ulong).wrapping_add(n) as size_t as size_t;
}
unsafe extern "C" fn string_table_copy_from_file<'a1>(
    mut t: Option<&'a1 mut crate::src::libtree::string_table_t>,
    mut fptr: * mut crate::src::libtree::_IO_FILE,
) {
    let mut c: i32 = 0;
    loop {
        c = _IO_getc(fptr);
        if !(c != '\0' as i32 && c != -(1 as std::os::raw::c_int)) {
            break;
        }
        string_table_maybe_grow(borrow_mut(&mut t), 1 as std::os::raw::c_int as size_t);
        let ref mut fresh12 = (*(borrow_mut(&mut t)).unwrap()).n;
        let mut fresh13 = *fresh12;
        *fresh12 = (*fresh12).wrapping_add(1);
        *((*(borrow(& t)).unwrap()).arr).offset(fresh13 as isize) = c as std::os::raw::c_char;
    }
    string_table_maybe_grow(borrow_mut(&mut t), 1 as std::os::raw::c_int as size_t);
    let ref mut fresh14 = (*(borrow_mut(&mut t)).unwrap()).n;
    let mut fresh15 = *fresh14;
    *fresh14 = (*fresh14).wrapping_add(1);
    *((*(borrow(& t)).unwrap()).arr).offset(fresh15 as isize) = '\0' as i32 as std::os::raw::c_char;
}
unsafe extern "C" fn is_in_exclude_list(mut soname: * mut std::os::raw::c_char) -> std::os::raw::c_int {
    let mut start = soname;
    let mut end = strrchr(start, '\0' as i32);
    if start == end {
        return 0 as std::os::raw::c_int;
    }
    end = end.offset(-1);
    while end != start
        && (*end as std::os::raw::c_int >= '0' as i32 && *end as std::os::raw::c_int <= '9' as i32
            || *end as std::os::raw::c_int == '.' as i32)
    {
        end = end.offset(-1);
    }
    let mut j = 0 as std::os::raw::c_int as size_t;
    while j
        < (::std::mem::size_of::<[*const std::os::raw::c_char; 14]>() as std::os::raw::c_ulong)
            .wrapping_div(::std::mem::size_of::<*mut std::os::raw::c_char>() as std::os::raw::c_ulong)
    {
        let mut len = strlen(exclude_list[j as usize]);
        if strncmp(start, exclude_list[j as usize], len) != 0 as std::os::raw::c_int {
            j = j.wrapping_add(1);
        } else {
            return 1 as std::os::raw::c_int
        }
    }
    return 0 as std::os::raw::c_int;
}
unsafe extern "C" fn tree_preamble(mut s: * mut crate::src::libtree::libtree_state_t, mut depth: std::os::raw::c_ulong) {
    if depth == 0 as std::os::raw::c_int as std::os::raw::c_ulong {
        return;
    }
    let mut i = 0 as std::os::raw::c_int as size_t;
    while i < depth.wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong) {
        fputs(
            if (*s).found_all_needed[i as usize] as std::os::raw::c_int != 0 {
                b"    \0" as *const u8 as *const std::os::raw::c_char
            } else {
                b"\xE2\x94\x82   \0" as *const u8 as *const std::os::raw::c_char
            },
            stdout,
        );
        i = i.wrapping_add(1);
    }
    fputs(
        if (*s)
            .found_all_needed[depth.wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong)
            as usize] as std::os::raw::c_int != 0
        {
            b"\xE2\x94\x94\xE2\x94\x80\xE2\x94\x80 \0" as *const u8
                as *const std::os::raw::c_char
        } else {
            b"\xE2\x94\x9C\xE2\x94\x80\xE2\x94\x80 \0" as *const u8
                as *const std::os::raw::c_char
        },
        stdout,
    );
}
unsafe extern "C" fn apply_exclude_list<'a1, 'a2>(
    mut needed_not_found: Option<&'a1 mut std::os::raw::c_ulong>,
    mut needed_buf_offsets: Option<&'a2 mut crate::src::libtree::small_vec_u64_t>,
    mut s: * mut crate::src::libtree::libtree_state_t,
) {
    let mut i = 0 as std::os::raw::c_int as size_t;
    while i < *(borrow(& needed_not_found)).unwrap() {
        if is_in_exclude_list(
            ((*s).string_table.arr)
                .offset(*((*(borrow(& needed_buf_offsets)).unwrap()).p).offset(i as isize) as isize),
        ) != 0
        {
            let mut tmp = *((*(borrow(& needed_buf_offsets)).unwrap()).p).offset(i as isize);
            *((*(borrow(& needed_buf_offsets)).unwrap()).p)
                .offset(
                    i as isize,
                ) = *((*(borrow(& needed_buf_offsets)).unwrap()).p)
                .offset(
                    (*(borrow(& needed_not_found)).unwrap()).wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong)
                        as isize,
                );
            *(borrow_mut(&mut needed_not_found)).unwrap() = (*(borrow(& needed_not_found)).unwrap()).wrapping_sub(1);
            *((*(borrow(& needed_buf_offsets)).unwrap()).p).offset(*(borrow(& needed_not_found)).unwrap() as isize) = tmp;
        } else {
            i = i.wrapping_add(1);
        }
    }
}
unsafe extern "C" fn check_absolute_paths<'a1, 'a2>(
    mut needed_not_found: Option<&'a1 mut std::os::raw::c_ulong>,
    mut needed_buf_offsets: Option<&'a2 mut crate::src::libtree::small_vec_u64_t>,
    mut depth: std::os::raw::c_ulong,
    mut s: * mut crate::src::libtree::libtree_state_t,
    mut compat: crate::src::libtree::compat_t,
) -> std::os::raw::c_int {
    let mut exit_code = 0 as std::os::raw::c_int;
    let mut i = 0 as std::os::raw::c_int as size_t;
    while i < *(borrow(& needed_not_found)).unwrap() {
        let mut st: Option<&'_ crate::src::libtree::string_table_t> = Some(&mut (*s).string_table);
        if (strchr(
            ((*((st).clone()).unwrap()).arr).offset(*((*(borrow(& needed_buf_offsets)).unwrap()).p).offset(i as isize) as isize),
            '/' as i32,
        ))
            .is_null()
        {
            i = i.wrapping_add(1);
        } else {
            let mut path: [i8; 4096] = [0; 4096];
            let mut len = strlen(
                ((*((st).clone()).unwrap()).arr)
                    .offset(*((*(borrow(& needed_buf_offsets)).unwrap()).p).offset(i as isize) as isize),
            );
            if len >= 4096 as std::os::raw::c_int as std::os::raw::c_ulong {
                continue;
            }
            memcpy(
                path.as_mut_ptr() as *mut std::os::raw::c_void,
                ((*((st).clone()).unwrap()).arr)
                    .offset(*((*(borrow(& needed_buf_offsets)).unwrap()).p).offset(i as isize) as isize)
                    as *const std::os::raw::c_void,
                len.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong),
            );
            (*s)
                .found_all_needed[depth
                as usize] = (*(borrow(& needed_not_found)).unwrap() <= 1 as std::os::raw::c_int as std::os::raw::c_ulong)
                as std::os::raw::c_int as std::os::raw::c_char;
            let mut err = (0 as * mut i8);
            if path[0 as std::os::raw::c_int as usize] as std::os::raw::c_int != '/' as i32 {
                err = b" is not absolute\0" as *const u8 as *const std::os::raw::c_char
                    as *mut std::os::raw::c_char;
                exit_code = 28 as std::os::raw::c_int;
            } else {
                let mut code = recurse(
                    path.as_mut_ptr(),
                    depth.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong),
                    s,
                    compat,
                    {
                        let mut init = found_t { how: DIRECT, depth: 0 };
                        init
                    },
                );
                if code == 28 as std::os::raw::c_int {
                    exit_code = 28 as std::os::raw::c_int;
                }
                if code != 0 as std::os::raw::c_int && code != 28 as std::os::raw::c_int {
                    err = b" not found\0" as *const u8 as *const std::os::raw::c_char
                        as *mut std::os::raw::c_char;
                }
            }
            if !err.is_null() {
                tree_preamble(s, depth.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong));
                if (*s).color != 0 {
                    fputs(b"\x1B[1;31m\0" as *const u8 as *const std::os::raw::c_char, stdout);
                }
                fputs(path.as_mut_ptr(), stdout);
                fputs(b" is not absolute\0" as *const u8 as *const std::os::raw::c_char, stdout);
                fputs(
                    if (*s).color != 0 {
                        b"\x1B[0m\n\0" as *const u8 as *const std::os::raw::c_char
                    } else {
                        b"\n\0" as *const u8 as *const std::os::raw::c_char
                    },
                    stdout,
                );
            }
            let mut tmp = *((*(borrow(& needed_buf_offsets)).unwrap()).p).offset(i as isize);
            *((*(borrow(& needed_buf_offsets)).unwrap()).p)
                .offset(
                    i as isize,
                ) = *((*(borrow(& needed_buf_offsets)).unwrap()).p)
                .offset(
                    (*(borrow(& needed_not_found)).unwrap()).wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong)
                        as isize,
                );
            *(borrow_mut(&mut needed_not_found)).unwrap() = (*(borrow(& needed_not_found)).unwrap()).wrapping_sub(1);
            *((*(borrow(& needed_buf_offsets)).unwrap()).p).offset(*(borrow(& needed_not_found)).unwrap() as isize) = tmp;
        }
    }
    return exit_code;
}
unsafe extern "C" fn check_search_paths<'a1, 'a2>(
    mut reason: crate::src::libtree::found_t,
    mut offset: std::os::raw::c_ulong,
    mut needed_not_found: Option<&'a1 mut std::os::raw::c_ulong>,
    mut needed_buf_offsets: Option<&'a2 mut crate::src::libtree::small_vec_u64_t>,
    mut depth: std::os::raw::c_ulong,
    mut s: * mut crate::src::libtree::libtree_state_t,
    mut compat: crate::src::libtree::compat_t,
) -> std::os::raw::c_int {
    let mut exit_code = 0 as std::os::raw::c_int;
    let mut path: [i8; 4096] = [0; 4096];
    let mut path_end = path.as_mut_ptr().offset(4096 as std::os::raw::c_int as isize);
    let mut st: Option<&'_ crate::src::libtree::string_table_t> = Some(&mut (*s).string_table);
    while *((*((st).clone()).unwrap()).arr).offset(offset as isize) as std::os::raw::c_int != '\0' as i32 {
        while *((*((st).clone()).unwrap()).arr).offset(offset as isize) as std::os::raw::c_int == ':' as i32
            && *((*((st).clone()).unwrap()).arr).offset(offset as isize) as std::os::raw::c_int != '\0' as i32
        {
            offset = offset.wrapping_add(1);
        }
        if *((*((st).clone()).unwrap()).arr).offset(offset as isize) as std::os::raw::c_int == '\0' as i32 {
            return exit_code;
        }
        let mut dest = path.as_mut_ptr();
        while *((*((st).clone()).unwrap()).arr).offset(offset as isize) as std::os::raw::c_int != '\0' as i32
            && *((*((st).clone()).unwrap()).arr).offset(offset as isize) as std::os::raw::c_int != ':' as i32
            && dest != path_end
        {
            let mut fresh16 = offset;
            offset = offset.wrapping_add(1);
            let mut fresh17 = dest;
            dest = dest.offset(1);
            *fresh17 = *((*((st).clone()).unwrap()).arr).offset(fresh16 as isize);
        }
        if dest.offset(1 as std::os::raw::c_int as isize) >= path_end {
            continue;
        }
        if *dest.offset(-(1 as std::os::raw::c_int as isize)) as std::os::raw::c_int != '/' as i32 {
            let mut fresh18 = dest;
            dest = dest.offset(1);
            *fresh18 = '/' as i32 as std::os::raw::c_char;
        }
        let mut search_path_end = dest;
        let mut i = 0 as std::os::raw::c_int as size_t;
        while i < *(borrow(& needed_not_found)).unwrap() {
            let mut soname_len = strlen(
                ((*((st).clone()).unwrap()).arr)
                    .offset(*((*(borrow(& needed_buf_offsets)).unwrap()).p).offset(i as isize) as isize),
            );
            if search_path_end
                .offset(soname_len as isize)
                .offset(1 as std::os::raw::c_int as isize) >= path_end
            {
                continue;
            }
            memcpy(
                search_path_end as *mut std::os::raw::c_void,
                ((*((st).clone()).unwrap()).arr)
                    .offset(*((*(borrow(& needed_buf_offsets)).unwrap()).p).offset(i as isize) as isize)
                    as *const std::os::raw::c_void,
                soname_len.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong),
            );
            (*s)
                .found_all_needed[depth
                as usize] = (*(borrow(& needed_not_found)).unwrap() <= 1 as std::os::raw::c_int as std::os::raw::c_ulong)
                as std::os::raw::c_int as std::os::raw::c_char;
            let mut code = recurse(
                path.as_mut_ptr(),
                depth.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong),
                s,
                compat,
                reason,
            );
            if code == 28 as std::os::raw::c_int {
                exit_code = 28 as std::os::raw::c_int;
            }
            if code == 0 as std::os::raw::c_int || code == 28 as std::os::raw::c_int {
                let mut tmp = *((*(borrow(& needed_buf_offsets)).unwrap()).p).offset(i as isize);
                *((*(borrow(& needed_buf_offsets)).unwrap()).p)
                    .offset(
                        i as isize,
                    ) = *((*(borrow(& needed_buf_offsets)).unwrap()).p)
                    .offset(
                        (*(borrow(& needed_not_found)).unwrap())
                            .wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                    );
                *(borrow_mut(&mut needed_not_found)).unwrap() = (*(borrow(& needed_not_found)).unwrap()).wrapping_sub(1);
                *((*(borrow(& needed_buf_offsets)).unwrap()).p).offset(*(borrow(& needed_not_found)).unwrap() as isize) = tmp;
            } else {
                i = i.wrapping_add(1);
            }
        }
    }
    return exit_code;
}
unsafe extern "C" fn interpolate_variables(
    mut s: * mut crate::src::libtree::libtree_state_t,
    mut src: std::os::raw::c_ulong,
    mut ORIGIN: * const std::os::raw::c_char,
) -> std::os::raw::c_int {
    let mut prev_src = src;
    let mut curr_src = src;
    let mut st: Option<&'_ mut crate::src::libtree::string_table_t> = Some(&mut (*s).string_table);
    loop {
        let mut dollar = strchr(((*(borrow(& st)).unwrap()).arr).offset(curr_src as isize), '$' as i32);
        if dollar.is_null() {
            break;
        }
        curr_src = dollar.offset_from((*(borrow(& st)).unwrap()).arr) as std::os::raw::c_long as size_t;
        let mut bytes_to_dollar = curr_src.wrapping_sub(prev_src);
        curr_src = curr_src.wrapping_add(1);
        let mut curly = 0 as std::os::raw::c_int;
        if *((*(borrow(& st)).unwrap()).arr).offset(curr_src as isize) as std::os::raw::c_int == '{' as i32 {
            curly = 1 as std::os::raw::c_int;
            curr_src = curr_src.wrapping_add(1);
        }
        let mut var_val = 0 as *const std::os::raw::c_char;
        if strncmp(
            &mut *((*(borrow(& st)).unwrap()).arr).offset(curr_src as isize),
            b"ORIGIN\0" as *const u8 as *const std::os::raw::c_char,
            6 as std::os::raw::c_int as std::os::raw::c_ulong,
        ) == 0 as std::os::raw::c_int
        {
            var_val = ORIGIN;
            curr_src = (curr_src as std::os::raw::c_ulong)
                .wrapping_add(6 as std::os::raw::c_int as std::os::raw::c_ulong) as size_t as size_t;
        } else if strncmp(
            &mut *((*(borrow(& st)).unwrap()).arr).offset(curr_src as isize),
            b"LIB\0" as *const u8 as *const std::os::raw::c_char,
            3 as std::os::raw::c_int as std::os::raw::c_ulong,
        ) == 0 as std::os::raw::c_int
        {
            var_val = (*s).LIB;
            curr_src = (curr_src as std::os::raw::c_ulong)
                .wrapping_add(3 as std::os::raw::c_int as std::os::raw::c_ulong) as size_t as size_t;
        } else if strncmp(
            &mut *((*(borrow(& st)).unwrap()).arr).offset(curr_src as isize),
            b"PLATFORM\0" as *const u8 as *const std::os::raw::c_char,
            8 as std::os::raw::c_int as std::os::raw::c_ulong,
        ) == 0 as std::os::raw::c_int
        {
            var_val = (*s).PLATFORM;
            curr_src = (curr_src as std::os::raw::c_ulong)
                .wrapping_add(8 as std::os::raw::c_int as std::os::raw::c_ulong) as size_t as size_t;
        } else if strncmp(
            &mut *((*(borrow(& st)).unwrap()).arr).offset(curr_src as isize),
            b"OSNAME\0" as *const u8 as *const std::os::raw::c_char,
            6 as std::os::raw::c_int as std::os::raw::c_ulong,
        ) == 0 as std::os::raw::c_int
        {
            var_val = (*s).OSNAME;
            curr_src = (curr_src as std::os::raw::c_ulong)
                .wrapping_add(6 as std::os::raw::c_int as std::os::raw::c_ulong) as size_t as size_t;
        } else {
            if !(strncmp(
                &mut *((*(borrow(& st)).unwrap()).arr).offset(curr_src as isize),
                b"OSREL\0" as *const u8 as *const std::os::raw::c_char,
                5 as std::os::raw::c_int as std::os::raw::c_ulong,
            ) == 0 as std::os::raw::c_int)
            {
                continue;
            }
            var_val = (*s).OSREL;
            curr_src = (curr_src as std::os::raw::c_ulong)
                .wrapping_add(5 as std::os::raw::c_int as std::os::raw::c_ulong) as size_t as size_t;
        }
        if curly != 0 {
            if *((*(borrow(& st)).unwrap()).arr).offset(curr_src as isize) as std::os::raw::c_int != '}' as i32 {
                continue;
            }
            curr_src = curr_src.wrapping_add(1);
        }
        let mut var_len = strlen(var_val);
        string_table_maybe_grow(borrow_mut(&mut st), bytes_to_dollar.wrapping_add(var_len));
        memcpy(
            &mut *((*(borrow(& st)).unwrap()).arr).offset((*s).string_table.n as isize) as *mut std::os::raw::c_char
                as *mut std::os::raw::c_void,
            &mut *((*(borrow(& st)).unwrap()).arr).offset(prev_src as isize) as *mut std::os::raw::c_char
                as *const std::os::raw::c_void,
            bytes_to_dollar,
        );
        let ref mut fresh19 = (*s).string_table.n;
        *fresh19 = (*fresh19 as std::os::raw::c_ulong).wrapping_add(bytes_to_dollar) as size_t
            as size_t;
        prev_src = curr_src;
        memcpy(
            &mut *((*(borrow(& st)).unwrap()).arr).offset((*s).string_table.n as isize) as *mut std::os::raw::c_char
                as *mut std::os::raw::c_void,
            var_val as *const std::os::raw::c_void,
            var_len,
        );
        let ref mut fresh20 = (*s).string_table.n;
        *fresh20 = (*fresh20 as std::os::raw::c_ulong).wrapping_add(var_len) as size_t as size_t;
    }
    if prev_src != src {
        let mut n = (strlen(((*(borrow(& st)).unwrap()).arr).offset(prev_src as isize)))
            .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong);
        string_table_maybe_grow(borrow_mut(&mut st), n);
        memcpy(
            ((*(borrow(& st)).unwrap()).arr).offset((*(borrow(& st)).unwrap()).n as isize) as *mut std::os::raw::c_void,
            ((*(borrow(& st)).unwrap()).arr).offset(prev_src as isize) as *const std::os::raw::c_void,
            n,
        );
        let ref mut fresh21 = (*(borrow_mut(&mut st)).unwrap()).n;
        *fresh21 = (*fresh21 as std::os::raw::c_ulong).wrapping_add(n) as size_t as size_t;
        return 1 as std::os::raw::c_int;
    }
    return 0 as std::os::raw::c_int;
}
unsafe extern "C" fn print_colon_delimited_paths(
    mut start: * const std::os::raw::c_char,
    mut indent: * const std::os::raw::c_char,
) {
    while !(*start as std::os::raw::c_int == '\0' as i32) {
        let mut next = strchr(start, ':' as i32);
        if start == next {
            start = start.offset(1);
        } else {
            fputs(indent, stdout);
            fputs(b"    \0" as *const u8 as *const std::os::raw::c_char, stdout);
            if next.is_null() {
                puts(start);
            } else {
                fwrite(
                    start as *const std::os::raw::c_void,
                    1 as std::os::raw::c_int as size_t,
                    next.offset_from(start) as std::os::raw::c_long as size_t,
                    stdout,
                );
                putchar('\n' as i32);
            }
            if next.is_null() {
                break;
            }
            start = next.offset(1 as std::os::raw::c_int as isize);
        }
    }
}
unsafe extern "C" fn print_line(
    mut depth: std::os::raw::c_ulong,
    mut name: * mut std::os::raw::c_char,
    mut color_bold: * mut std::os::raw::c_char,
    mut color_regular: * mut std::os::raw::c_char,
    mut highlight: std::os::raw::c_int,
    mut reason: crate::src::libtree::found_t,
    mut s: * mut crate::src::libtree::libtree_state_t,
) {
    tree_preamble(s, depth);
    let mut slash = 0 as *mut std::os::raw::c_char;
    if (*s).color != 0 && highlight != 0
        && {
            slash = strrchr(name, '/' as i32);
            !slash.is_null()
        }
    {
        fputs(color_regular, stdout);
        fwrite(
            name as *const std::os::raw::c_void,
            1 as std::os::raw::c_int as size_t,
            slash.offset(1 as std::os::raw::c_int as isize).offset_from(name) as std::os::raw::c_long
                as size_t,
            stdout,
        );
        fputs(color_bold, stdout);
        fputs(slash.offset(1 as std::os::raw::c_int as isize), stdout);
    } else {
        if (*s).color != 0 {
            fputs(color_bold, stdout);
        }
        fputs(name, stdout);
    }
    if (*s).color != 0 && highlight != 0 {
        fputs(b"\x1B[0m \x1B[33m\0" as *const u8 as *const std::os::raw::c_char, stdout);
    } else {
        putchar(' ' as i32);
    }
    let mut conf_name: * mut i8 = 0 as *mut std::os::raw::c_char;
    match reason.how as std::os::raw::c_uint {
        2 => {
            if (reason.depth).wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) >= depth {
                fputs(b"[rpath]\0" as *const u8 as *const std::os::raw::c_char, stdout);
            } else {
                let mut num: [i8; 8] = [0; 8];
                utoa(
                    num.as_mut_ptr(),
                    (reason.depth).wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong),
                );
                fputs(b"[rpath of \0" as *const u8 as *const std::os::raw::c_char, stdout);
                fputs(num.as_mut_ptr(), stdout);
                putchar(']' as i32);
            }
        }
        3 => {
            fputs(b"[LD_LIBRARY_PATH]\0" as *const u8 as *const std::os::raw::c_char, stdout);
        }
        4 => {
            fputs(b"[runpath]\0" as *const u8 as *const std::os::raw::c_char, stdout);
        }
        5 => {
            putchar('[' as i32);
            conf_name = strrchr((*s).ld_conf_file, '/' as i32);
            conf_name = if conf_name.is_null() {
                (*s).ld_conf_file
            } else {
                conf_name.offset(1 as std::os::raw::c_int as isize)
            };
            fputs(conf_name, stdout);
            putchar(']' as i32);
        }
        1 => {
            fputs(b"[direct]\0" as *const u8 as *const std::os::raw::c_char, stdout);
        }
        6 => {
            fputs(b"[default path]\0" as *const u8 as *const std::os::raw::c_char, stdout);
        }
        _ => {}
    }
    if (*s).color != 0 {
        fputs(b"\x1B[0m\n\0" as *const u8 as *const std::os::raw::c_char, stdout);
    } else {
        putchar('\n' as i32);
    };
}
unsafe extern "C" fn print_error<'a1>(
    mut depth: std::os::raw::c_ulong,
    mut needed_not_found: std::os::raw::c_ulong,
    mut needed_buf_offsets: Option<&'a1 mut crate::src::libtree::small_vec_u64_t>,
    mut runpath: * mut std::os::raw::c_char,
    mut s: * mut crate::src::libtree::libtree_state_t,
    mut no_def_lib: std::os::raw::c_int,
) {
    let mut i = 0 as std::os::raw::c_int as size_t;
    while i < needed_not_found {
        (*s)
            .found_all_needed[depth
            as usize] = (i.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong)
            >= needed_not_found) as std::os::raw::c_int as std::os::raw::c_char;
        tree_preamble(s, depth.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong));
        if (*s).color != 0 {
            fputs(b"\x1B[1;31m\0" as *const u8 as *const std::os::raw::c_char, stdout);
        }
        fputs(
            ((*s).string_table.arr)
                .offset(*((*(borrow(& needed_buf_offsets)).unwrap()).p).offset(i as isize) as isize),
            stdout,
        );
        fputs(b" not found\n\0" as *const u8 as *const std::os::raw::c_char, stdout);
        if (*s).color != 0 {
            fputs(b"\x1B[0m\0" as *const u8 as *const std::os::raw::c_char, stdout);
        }
        i = i.wrapping_add(1);
    }
    let mut box_vertical = (if (*s).color != 0 {
        b"    \x1B[0;31m\xE2\x94\x8A\x1B[0m\0" as *const u8 as *const std::os::raw::c_char
    } else {
        b"    \xE2\x94\x8A\0" as *const u8 as *const std::os::raw::c_char
    }) as *mut std::os::raw::c_char;
    let mut indent = malloc(
        (::std::mem::size_of::<[std::os::raw::c_char; 7]>() as std::os::raw::c_ulong)
            .wrapping_mul(depth)
            .wrapping_add(strlen(box_vertical))
            .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong),
    ) as *mut std::os::raw::c_char;
    let mut p = indent;
    let mut i_0 = 0 as std::os::raw::c_int as size_t;
    while i_0 < depth {
        if (*s).found_all_needed[i_0 as usize] != 0 {
            let mut len = (::std::mem::size_of::<[std::os::raw::c_char; 5]>() as std::os::raw::c_ulong)
                .wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong) as std::os::raw::c_int;
            memcpy(
                p as *mut std::os::raw::c_void,
                b"    \0" as *const u8 as *const std::os::raw::c_char as *const std::os::raw::c_void,
                len as std::os::raw::c_ulong,
            );
            p = p.offset(len as isize);
        } else {
            let mut len_0 = (::std::mem::size_of::<[std::os::raw::c_char; 7]>() as std::os::raw::c_ulong)
                .wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong) as std::os::raw::c_int;
            memcpy(
                p as *mut std::os::raw::c_void,
                b"\xE2\x94\x82   \0" as *const u8 as *const std::os::raw::c_char
                    as *const std::os::raw::c_void,
                len_0 as std::os::raw::c_ulong,
            );
            p = p.offset(len_0 as isize);
        }
        i_0 = i_0.wrapping_add(1);
    }
    strcpy(p, box_vertical);
    fputs(indent, stdout);
    if (*s).color != 0 {
        fputs(b"\x1B[0;90m\0" as *const u8 as *const std::os::raw::c_char, stdout);
    }
    fputs(
        b" Paths considered in this order:\n\0" as *const u8 as *const std::os::raw::c_char,
        stdout,
    );
    if (*s).color != 0 {
        fputs(b"\x1B[0m\0" as *const u8 as *const std::os::raw::c_char, stdout);
    }
    fputs(indent, stdout);
    if !runpath.is_null() {
        if (*s).color != 0 {
            fputs(b"\x1B[0;90m\0" as *const u8 as *const std::os::raw::c_char, stdout);
        }
        fputs(
            b" 1. rpath is skipped because runpath was set\n\0" as *const u8
                as *const std::os::raw::c_char,
            stdout,
        );
        if (*s).color != 0 {
            fputs(b"\x1B[0m\0" as *const u8 as *const std::os::raw::c_char, stdout);
        }
    } else {
        if (*s).color != 0 {
            fputs(b"\x1B[0;90m\0" as *const u8 as *const std::os::raw::c_char, stdout);
        }
        fputs(b" 1. rpath:\n\0" as *const u8 as *const std::os::raw::c_char, stdout);
        if (*s).color != 0 {
            fputs(b"\x1B[0m\0" as *const u8 as *const std::os::raw::c_char, stdout);
        }
        let mut j = depth as std::os::raw::c_int;
        while j >= 0 as std::os::raw::c_int {
            if (*s).rpath_offsets[j as usize] != 18446744073709551615 as std::os::raw::c_ulong {
                let mut num: [i8; 8] = [0; 8];
                utoa(num.as_mut_ptr(), (j + 1 as std::os::raw::c_int) as size_t);
                fputs(indent, stdout);
                if (*s).color != 0 {
                    fputs(b"\x1B[0;90m\0" as *const u8 as *const std::os::raw::c_char, stdout);
                }
                fputs(b"    depth \0" as *const u8 as *const std::os::raw::c_char, stdout);
                fputs(num.as_mut_ptr(), stdout);
                if (*s).color != 0 {
                    fputs(b"\x1B[0m\0" as *const u8 as *const std::os::raw::c_char, stdout);
                }
                putchar('\n' as i32);
                print_colon_delimited_paths(
                    ((*s).string_table.arr)
                        .offset((*s).rpath_offsets[j as usize] as isize),
                    indent,
                );
            }
            j -= 1;
        }
    }
    fputs(indent, stdout);
    if (*s).color != 0 {
        fputs(b"\x1B[0;90m\0" as *const u8 as *const std::os::raw::c_char, stdout);
    }
    fputs(
        if (*s).ld_library_path_offset == 18446744073709551615 as std::os::raw::c_ulong {
            b" 2. LD_LIBRARY_PATH was not set\n\0" as *const u8 as *const std::os::raw::c_char
        } else {
            b" 2. LD_LIBRARY_PATH:\n\0" as *const u8 as *const std::os::raw::c_char
        },
        stdout,
    );
    if (*s).color != 0 {
        fputs(b"\x1B[0m\0" as *const u8 as *const std::os::raw::c_char, stdout);
    }
    if (*s).ld_library_path_offset != 18446744073709551615 as std::os::raw::c_ulong {
        print_colon_delimited_paths(
            ((*s).string_table.arr).offset((*s).ld_library_path_offset as isize),
            indent,
        );
    }
    fputs(indent, stdout);
    if (*s).color != 0 {
        fputs(b"\x1B[0;90m\0" as *const u8 as *const std::os::raw::c_char, stdout);
    }
    fputs(
        if runpath.is_null() {
            b" 3. runpath was not set\n\0" as *const u8 as *const std::os::raw::c_char
        } else {
            b" 3. runpath:\n\0" as *const u8 as *const std::os::raw::c_char
        },
        stdout,
    );
    if (*s).color != 0 {
        fputs(b"\x1B[0m\0" as *const u8 as *const std::os::raw::c_char, stdout);
    }
    if !runpath.is_null() {
        print_colon_delimited_paths(runpath, indent);
    }
    fputs(indent, stdout);
    if (*s).color != 0 {
        fputs(b"\x1B[0;90m\0" as *const u8 as *const std::os::raw::c_char, stdout);
    }
    fputs(
        if no_def_lib != 0 {
            b" 4. ld config files not considered due to NODEFLIB flag\n\0" as *const u8
                as *const std::os::raw::c_char
        } else {
            b" 4. ld config files:\n\0" as *const u8 as *const std::os::raw::c_char
        },
        stdout,
    );
    if (*s).color != 0 {
        fputs(b"\x1B[0m\0" as *const u8 as *const std::os::raw::c_char, stdout);
    }
    print_colon_delimited_paths(
        ((*s).string_table.arr).offset((*s).ld_so_conf_offset as isize),
        indent,
    );
    fputs(indent, stdout);
    if (*s).color != 0 {
        fputs(b"\x1B[0;90m\0" as *const u8 as *const std::os::raw::c_char, stdout);
    }
    fputs(
        if no_def_lib != 0 {
            b" 5. Standard paths not considered due to NODEFLIB flag\n\0" as *const u8
                as *const std::os::raw::c_char
        } else {
            b" 5. Standard paths:\n\0" as *const u8 as *const std::os::raw::c_char
        },
        stdout,
    );
    if (*s).color != 0 {
        fputs(b"\x1B[0m\0" as *const u8 as *const std::os::raw::c_char, stdout);
    }
    print_colon_delimited_paths(
        ((*s).string_table.arr).offset((*s).default_paths_offset as isize),
        indent,
    );
    free(indent as *mut std::os::raw::c_void);
}
unsafe extern "C" fn visited_files_contains<'a1, 'a2>(
    mut files: Option<&'a1 mut crate::src::libtree::visited_file_array_t>,
    mut needle: Option<&'a2 mut crate::src::libtree::stat>,
) -> std::os::raw::c_int {
    let mut i = 0 as std::os::raw::c_int as size_t;
    while i < (*(borrow(& files)).unwrap()).n {
        let mut f: Option<&'_ mut crate::src::libtree::visited_file_t> = Some(&mut *((*(borrow(& files)).unwrap()).arr).offset(i as isize));
        if (*(borrow(& f)).unwrap()).st_dev == (*(borrow(& needle)).unwrap()).st_dev && (*(borrow(& f)).unwrap()).st_ino == (*(borrow(& needle)).unwrap()).st_ino {
            return 1 as std::os::raw::c_int;
        }
        i = i.wrapping_add(1);
    }
    return 0 as std::os::raw::c_int;
}
unsafe extern "C" fn visited_files_append<'a1>(
    mut files: * mut crate::src::libtree::visited_file_array_t,
    mut new: Option<&'a1 mut crate::src::libtree::stat>,
) {
    if (*files).n == (*files).capacity {
        let ref mut fresh22 = (*files).capacity;
        *fresh22 = (*fresh22 as std::os::raw::c_ulong)
            .wrapping_mul(2 as std::os::raw::c_int as std::os::raw::c_ulong) as size_t as size_t;
        let ref mut fresh23 = (*files).arr;
        *fresh23 = realloc(
            (*files).arr as *mut std::os::raw::c_void,
            ((*files).capacity)
                .wrapping_mul(::std::mem::size_of::<visited_file_t>() as std::os::raw::c_ulong),
        ) as *mut visited_file_t;
        if ((*files).arr).is_null() {
            exit(1 as std::os::raw::c_int);
        }
    }
    (*((*files).arr).offset((*files).n as isize)).st_dev = (*(borrow_mut(&mut new)).unwrap()).st_dev;
    (*((*files).arr).offset((*files).n as isize)).st_ino = (*(borrow_mut(&mut new)).unwrap()).st_ino;
    let ref mut fresh24 = (*files).n;
    *fresh24 = (*fresh24).wrapping_add(1);
}
unsafe extern "C" fn recurse(
    mut current_file: * mut std::os::raw::c_char,
    mut depth: std::os::raw::c_ulong,
    mut s: * mut crate::src::libtree::libtree_state_t,
    mut compat: crate::src::libtree::compat_t,
    mut reason: crate::src::libtree::found_t,
) -> std::os::raw::c_int {
    let mut fptr = fopen(current_file, b"rb\0" as *const u8 as *const std::os::raw::c_char);
    if fptr.is_null() {
        return 31 as std::os::raw::c_int;
    }
    let mut old_buf_size = (*s).string_table.n;
    let mut e_ident: [i8; 16] = [0; 16];
    if fread(
        &mut e_ident as *mut [std::os::raw::c_char; 16] as *mut std::os::raw::c_void,
        16 as std::os::raw::c_int as size_t,
        1 as std::os::raw::c_int as size_t,
        fptr,
    ) != 1 as std::os::raw::c_int as std::os::raw::c_ulong
    {
        fclose(fptr);
        return 11 as std::os::raw::c_int;
    }
    if e_ident[0 as std::os::raw::c_int as usize] as std::os::raw::c_int != 0x7f as std::os::raw::c_int
        || e_ident[1 as std::os::raw::c_int as usize] as std::os::raw::c_int != 'E' as i32
        || e_ident[2 as std::os::raw::c_int as usize] as std::os::raw::c_int != 'L' as i32
        || e_ident[3 as std::os::raw::c_int as usize] as std::os::raw::c_int != 'F' as i32
    {
        fclose(fptr);
        return 11 as std::os::raw::c_int;
    }
    if e_ident[4 as std::os::raw::c_int as usize] as std::os::raw::c_int != 1 as std::os::raw::c_int
        && e_ident[4 as std::os::raw::c_int as usize] as std::os::raw::c_int != 2 as std::os::raw::c_int
    {
        fclose(fptr);
        return 12 as std::os::raw::c_int;
    }
    if e_ident[5 as std::os::raw::c_int as usize] as std::os::raw::c_int != '\u{1}' as i32
        && e_ident[5 as std::os::raw::c_int as usize] as std::os::raw::c_int != '\u{2}' as i32
    {
        fclose(fptr);
        return 13 as std::os::raw::c_int;
    }
    let mut curr_type = {
        let mut init = compat_t {
            any: 0 as std::os::raw::c_int as std::os::raw::c_char,
            class: e_ident[4 as std::os::raw::c_int as usize] as uint8_t,
            machine: 0,
        };
        init
    };
    let mut is_little_endian = (e_ident[5 as std::os::raw::c_int as usize] as std::os::raw::c_int
        == '\u{1}' as i32) as std::os::raw::c_int;
    if compat.any == 0 && compat.class as std::os::raw::c_int != curr_type.class as std::os::raw::c_int {
        fclose(fptr);
        return 15 as std::os::raw::c_int;
    }
    if is_little_endian ^ host_is_little_endian() != 0 {
        fclose(fptr);
        return 16 as std::os::raw::c_int;
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
    if curr_type.class as std::os::raw::c_int == 2 as std::os::raw::c_int {
        if fread(
            &mut header.h64 as *mut header_64_t as *mut std::os::raw::c_void,
            ::std::mem::size_of::<header_64_t>() as std::os::raw::c_ulong,
            1 as std::os::raw::c_int as size_t,
            fptr,
        ) != 1 as std::os::raw::c_int as std::os::raw::c_ulong
        {
            fclose(fptr);
            return 14 as std::os::raw::c_int;
        }
        if header.h64.e_type as std::os::raw::c_int != 2 as std::os::raw::c_int
            && header.h64.e_type as std::os::raw::c_int != 3 as std::os::raw::c_int
        {
            fclose(fptr);
            return 17 as std::os::raw::c_int;
        }
        curr_type.machine = header.h64.e_machine;
        if compat.any == 0
            && compat.machine as std::os::raw::c_int != curr_type.machine as std::os::raw::c_int
        {
            fclose(fptr);
            return 32 as std::os::raw::c_int;
        }
        if fseek(fptr, header.h64.e_phoff as std::os::raw::c_long, 0 as std::os::raw::c_int)
            != 0 as std::os::raw::c_int
        {
            fclose(fptr);
            return 18 as std::os::raw::c_int;
        }
    } else {
        if fread(
            &mut header.h32 as *mut header_32_t as *mut std::os::raw::c_void,
            ::std::mem::size_of::<header_32_t>() as std::os::raw::c_ulong,
            1 as std::os::raw::c_int as size_t,
            fptr,
        ) != 1 as std::os::raw::c_int as std::os::raw::c_ulong
        {
            fclose(fptr);
            return 14 as std::os::raw::c_int;
        }
        if header.h32.e_type as std::os::raw::c_int != 2 as std::os::raw::c_int
            && header.h32.e_type as std::os::raw::c_int != 3 as std::os::raw::c_int
        {
            fclose(fptr);
            return 17 as std::os::raw::c_int;
        }
        curr_type.machine = header.h32.e_machine;
        if compat.any == 0
            && compat.machine as std::os::raw::c_int != curr_type.machine as std::os::raw::c_int
        {
            fclose(fptr);
            return 32 as std::os::raw::c_int;
        }
        if fseek(fptr, header.h32.e_phoff as std::os::raw::c_long, 0 as std::os::raw::c_int)
            != 0 as std::os::raw::c_int
        {
            fclose(fptr);
            return 18 as std::os::raw::c_int;
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
    small_vec_u64_init(&mut pt_load_offset);
    small_vec_u64_init(&mut pt_load_vaddr);
    let mut p_offset = 0xffffffffffffffff as std::os::raw::c_ulong;
    if curr_type.class as std::os::raw::c_int == 2 as std::os::raw::c_int {
        let mut i = 0 as std::os::raw::c_int as uint64_t;
        while i < header.h64.e_phnum as std::os::raw::c_ulong {
            if fread(
                &mut prog.p64 as *mut prog_64_t as *mut std::os::raw::c_void,
                ::std::mem::size_of::<prog_64_t>() as std::os::raw::c_ulong,
                1 as std::os::raw::c_int as size_t,
                fptr,
            ) != 1 as std::os::raw::c_int as std::os::raw::c_ulong
            {
                fclose(fptr);
                small_vec_u64_free(Some(&mut pt_load_offset));
                small_vec_u64_free(Some(&mut pt_load_vaddr));
                return 19 as std::os::raw::c_int;
            }
            if prog.p64.p_type == 1 as std::os::raw::c_int as std::os::raw::c_uint {
                small_vec_u64_append(&mut pt_load_offset, prog.p64.p_offset);
                small_vec_u64_append(&mut pt_load_vaddr, prog.p64.p_vaddr);
            } else if prog.p64.p_type == 2 as std::os::raw::c_int as std::os::raw::c_uint {
                p_offset = prog.p64.p_offset;
            }
            i = i.wrapping_add(1);
        }
    } else {
        let mut i_0 = 0 as std::os::raw::c_int as uint32_t;
        while i_0 < header.h32.e_phnum as std::os::raw::c_uint {
            if fread(
                &mut prog.p32 as *mut prog_32_t as *mut std::os::raw::c_void,
                ::std::mem::size_of::<prog_32_t>() as std::os::raw::c_ulong,
                1 as std::os::raw::c_int as size_t,
                fptr,
            ) != 1 as std::os::raw::c_int as std::os::raw::c_ulong
            {
                fclose(fptr);
                small_vec_u64_free(Some(&mut pt_load_offset));
                small_vec_u64_free(Some(&mut pt_load_vaddr));
                return 19 as std::os::raw::c_int;
            }
            if prog.p32.p_type == 1 as std::os::raw::c_int as std::os::raw::c_uint {
                small_vec_u64_append(&mut pt_load_offset, prog.p32.p_offset as uint64_t);
                small_vec_u64_append(&mut pt_load_vaddr, prog.p32.p_vaddr as uint64_t);
            } else if prog.p32.p_type == 2 as std::os::raw::c_int as std::os::raw::c_uint {
                p_offset = prog.p32.p_offset as uint64_t;
            }
            i_0 = i_0.wrapping_add(1);
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
    if stat(current_file, &mut finfo) != 0 as std::os::raw::c_int {
        fclose(fptr);
        small_vec_u64_free(Some(&mut pt_load_offset));
        small_vec_u64_free(Some(&mut pt_load_vaddr));
        return 20 as std::os::raw::c_int;
    }
    let mut seen_before = visited_files_contains(Some(&mut (*s).visited), Some(&mut finfo));
    if seen_before == 0 {
        visited_files_append(&mut (*s).visited, Some(&mut finfo));
    }
    if p_offset == 0xffffffffffffffff as std::os::raw::c_ulong {
        print_line(
            depth,
            current_file,
            b"\x1B[1;36m\0" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
            b"\x1B[0;36m\0" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
            1 as std::os::raw::c_int,
            reason,
            s,
        );
        fclose(fptr);
        small_vec_u64_free(Some(&mut pt_load_offset));
        small_vec_u64_free(Some(&mut pt_load_vaddr));
        return 0 as std::os::raw::c_int;
    }
    if pt_load_offset.n == 0 as std::os::raw::c_int as std::os::raw::c_ulong {
        fclose(fptr);
        small_vec_u64_free(Some(&mut pt_load_offset));
        small_vec_u64_free(Some(&mut pt_load_vaddr));
        return 29 as std::os::raw::c_int;
    }
    if fseek(fptr, p_offset as std::os::raw::c_long, 0 as std::os::raw::c_int) != 0 as std::os::raw::c_int {
        fclose(fptr);
        small_vec_u64_free(Some(&mut pt_load_offset));
        small_vec_u64_free(Some(&mut pt_load_vaddr));
        return 21 as std::os::raw::c_int;
    }
    let mut no_def_lib = 0 as std::os::raw::c_int;
    let mut strtab = 0xffffffffffffffff as std::os::raw::c_ulong;
    let mut rpath = 0xffffffffffffffff as std::os::raw::c_ulong;
    let mut runpath = 0xffffffffffffffff as std::os::raw::c_ulong;
    let mut soname = 0xffffffffffffffff as std::os::raw::c_ulong;
    let mut needed = small_vec_u64_t {
        buf: [0; 16],
        p: 0 as *mut uint64_t,
        n: 0,
        capacity: 0,
    };
    small_vec_u64_init(&mut needed);
    let mut cont = 1 as std::os::raw::c_int;
    while cont != 0 {
        let mut d_tag: u64 = 0;
        let mut d_val: u64 = 0;
        if curr_type.class as std::os::raw::c_int == 2 as std::os::raw::c_int {
            let mut dyn_0 = dyn_64_t { d_tag: 0, d_val: 0 };
            if fread(
                &mut dyn_0 as *mut dyn_64_t as *mut std::os::raw::c_void,
                ::std::mem::size_of::<dyn_64_t>() as std::os::raw::c_ulong,
                1 as std::os::raw::c_int as size_t,
                fptr,
            ) != 1 as std::os::raw::c_int as std::os::raw::c_ulong
            {
                fclose(fptr);
                small_vec_u64_free(Some(&mut pt_load_offset));
                small_vec_u64_free(Some(&mut pt_load_vaddr));
                small_vec_u64_free(Some(&mut needed));
                return 22 as std::os::raw::c_int;
            }
            d_tag = dyn_0.d_tag as uint64_t;
            d_val = dyn_0.d_val;
        } else {
            let mut dyn_1 = dyn_32_t { d_tag: 0, d_val: 0 };
            if fread(
                &mut dyn_1 as *mut dyn_32_t as *mut std::os::raw::c_void,
                ::std::mem::size_of::<dyn_32_t>() as std::os::raw::c_ulong,
                1 as std::os::raw::c_int as size_t,
                fptr,
            ) != 1 as std::os::raw::c_int as std::os::raw::c_ulong
            {
                fclose(fptr);
                small_vec_u64_free(Some(&mut pt_load_offset));
                small_vec_u64_free(Some(&mut pt_load_vaddr));
                small_vec_u64_free(Some(&mut needed));
                return 22 as std::os::raw::c_int;
            }
            d_tag = dyn_1.d_tag as uint64_t;
            d_val = dyn_1.d_val as uint64_t;
        }
        match d_tag {
            0 => {
                cont = 0 as std::os::raw::c_int;
            }
            5 => {
                strtab = d_val;
            }
            15 => {
                rpath = d_val;
            }
            29 => {
                runpath = d_val;
            }
            1 => {
                small_vec_u64_append(&mut needed, d_val);
            }
            14 => {
                soname = d_val;
            }
            1879048187 => {
                no_def_lib
                    |= (0x800 as std::os::raw::c_int as std::os::raw::c_ulong & d_val
                        == 0x800 as std::os::raw::c_int as std::os::raw::c_ulong) as std::os::raw::c_int;
            }
            _ => {}
        }
    }
    if strtab == 0xffffffffffffffff as std::os::raw::c_ulong {
        fclose(fptr);
        small_vec_u64_free(Some(&mut pt_load_offset));
        small_vec_u64_free(Some(&mut pt_load_vaddr));
        small_vec_u64_free(Some(&mut needed));
        return 23 as std::os::raw::c_int;
    }
    if is_ascending_order(pt_load_vaddr.p, pt_load_vaddr.n) == 0 {
        fclose(fptr);
        small_vec_u64_free(Some(&mut pt_load_vaddr));
        small_vec_u64_free(Some(&mut pt_load_offset));
        small_vec_u64_free(Some(&mut needed));
        return 30 as std::os::raw::c_int;
    }
    let mut vaddr_idx = 0 as std::os::raw::c_int as size_t;
    while vaddr_idx.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) != pt_load_vaddr.n
        && strtab
            >= *(pt_load_vaddr.p)
                .offset(
                    vaddr_idx.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                )
    {
        vaddr_idx = vaddr_idx.wrapping_add(1);
    }
    let mut strtab_offset = (*(pt_load_offset.p).offset(vaddr_idx as isize))
        .wrapping_add(strtab)
        .wrapping_sub(*(pt_load_vaddr.p).offset(vaddr_idx as isize));
    small_vec_u64_free(Some(&mut pt_load_vaddr));
    small_vec_u64_free(Some(&mut pt_load_offset));
    let mut soname_buf_offset = (*s).string_table.n;
    if soname != 0xffffffffffffffff as std::os::raw::c_ulong {
        if fseek(
            fptr,
            strtab_offset.wrapping_add(soname) as std::os::raw::c_long,
            0 as std::os::raw::c_int,
        ) != 0 as std::os::raw::c_int
        {
            (*s).string_table.n = old_buf_size;
            fclose(fptr);
            small_vec_u64_free(Some(&mut needed));
            return 24 as std::os::raw::c_int;
        }
        string_table_copy_from_file(Some(&mut (*s).string_table), fptr);
    }
    let mut in_exclude_list = (soname != 0xffffffffffffffff as std::os::raw::c_ulong
        && is_in_exclude_list(((*s).string_table.arr).offset(soname_buf_offset as isize))
            != 0) as std::os::raw::c_int;
    let mut should_recurse = (depth < (*s).max_depth
        && (seen_before == 0 && in_exclude_list == 0
            || seen_before == 0 && in_exclude_list != 0
                && (*s).verbosity >= 2 as std::os::raw::c_int
            || (*s).verbosity >= 3 as std::os::raw::c_int)) as std::os::raw::c_int;
    if should_recurse == 0 {
        let mut print_name = if soname == 0xffffffffffffffff as std::os::raw::c_ulong
            || (*s).path != 0
        {
            current_file
        } else {
            ((*s).string_table.arr).offset(soname_buf_offset as isize)
        };
        let mut bold_color = (if in_exclude_list != 0 {
            b"\x1B[0;35m\0" as *const u8 as *const std::os::raw::c_char
        } else if seen_before != 0 {
            b"\x1B[0;34m\0" as *const u8 as *const std::os::raw::c_char
        } else {
            b"\x1B[1;36m\0" as *const u8 as *const std::os::raw::c_char
        }) as *mut std::os::raw::c_char;
        let mut regular_color = (if in_exclude_list != 0 {
            b"\x1B[0;35m\0" as *const u8 as *const std::os::raw::c_char
        } else if seen_before != 0 {
            b"\x1B[0;34m\0" as *const u8 as *const std::os::raw::c_char
        } else {
            b"\x1B[0;36m\0" as *const u8 as *const std::os::raw::c_char
        }) as *mut std::os::raw::c_char;
        let mut highlight = (seen_before == 0 && in_exclude_list == 0) as std::os::raw::c_int;
        print_line(depth, print_name, bold_color, regular_color, highlight, reason, s);
        (*s).string_table.n = old_buf_size;
        fclose(fptr);
        small_vec_u64_free(Some(&mut needed));
        return 0 as std::os::raw::c_int;
    }
    let mut origin: [i8; 4096] = [0; 4096];
    let mut last_slash = strrchr(current_file, '/' as i32);
    if !last_slash.is_null() {
        let mut bytes = last_slash.offset_from(current_file) as std::os::raw::c_long as size_t;
        memcpy(
            origin.as_mut_ptr() as *mut std::os::raw::c_void,
            current_file as *const std::os::raw::c_void,
            bytes,
        );
        origin[bytes as usize] = '\0' as i32 as std::os::raw::c_char;
    } else {
        memcpy(
            origin.as_mut_ptr() as *mut std::os::raw::c_void,
            b"./\0" as *const u8 as *const std::os::raw::c_char as *const std::os::raw::c_void,
            3 as std::os::raw::c_int as std::os::raw::c_ulong,
        );
    }
    if rpath == 0xffffffffffffffff as std::os::raw::c_ulong {
        (*s).rpath_offsets[depth as usize] = 18446744073709551615 as std::os::raw::c_ulong;
    } else {
        (*s).rpath_offsets[depth as usize] = (*s).string_table.n;
        if fseek(
            fptr,
            strtab_offset.wrapping_add(rpath) as std::os::raw::c_long,
            0 as std::os::raw::c_int,
        ) != 0 as std::os::raw::c_int
        {
            (*s).string_table.n = old_buf_size;
            fclose(fptr);
            small_vec_u64_free(Some(&mut needed));
            return 25 as std::os::raw::c_int;
        }
        string_table_copy_from_file(Some(&mut (*s).string_table), fptr);
        let mut curr_buf_size = (*s).string_table.n;
        if interpolate_variables(
            s,
            (*s).rpath_offsets[depth as usize],
            origin.as_mut_ptr(),
        ) != 0
        {
            (*s).rpath_offsets[depth as usize] = curr_buf_size;
        }
    }
    let mut runpath_buf_offset = (*s).string_table.n;
    if runpath != 0xffffffffffffffff as std::os::raw::c_ulong {
        if fseek(
            fptr,
            strtab_offset.wrapping_add(runpath) as std::os::raw::c_long,
            0 as std::os::raw::c_int,
        ) != 0 as std::os::raw::c_int
        {
            (*s).string_table.n = old_buf_size;
            fclose(fptr);
            small_vec_u64_free(Some(&mut needed));
            return 26 as std::os::raw::c_int;
        }
        string_table_copy_from_file(Some(&mut (*s).string_table), fptr);
        let mut curr_buf_size_0 = (*s).string_table.n;
        if interpolate_variables(s, runpath_buf_offset, origin.as_mut_ptr()) != 0 {
            runpath_buf_offset = curr_buf_size_0;
        }
    }
    let mut needed_buf_offsets = small_vec_u64_t {
        buf: [0; 16],
        p: 0 as *mut uint64_t,
        n: 0,
        capacity: 0,
    };
    small_vec_u64_init(&mut needed_buf_offsets);
    let mut i_1 = 0 as std::os::raw::c_int as size_t;
    while i_1 < needed.n {
        small_vec_u64_append(&mut needed_buf_offsets, (*s).string_table.n);
        if fseek(
            fptr,
            strtab_offset.wrapping_add(*(needed.p).offset(i_1 as isize)) as std::os::raw::c_long,
            0 as std::os::raw::c_int,
        ) != 0 as std::os::raw::c_int
        {
            (*s).string_table.n = old_buf_size;
            fclose(fptr);
            small_vec_u64_free(Some(&mut needed_buf_offsets));
            small_vec_u64_free(Some(&mut needed));
            return 27 as std::os::raw::c_int;
        }
        string_table_copy_from_file(Some(&mut (*s).string_table), fptr);
        i_1 = i_1.wrapping_add(1);
    }
    fclose(fptr);
    let mut print_name_0 = if soname == 0xffffffffffffffff as std::os::raw::c_ulong
        || (*s).path != 0
    {
        current_file
    } else {
        ((*s).string_table.arr).offset(soname_buf_offset as isize)
    };
    let mut bold_color_0 = (if in_exclude_list != 0 {
        b"\x1B[0;35m\0" as *const u8 as *const std::os::raw::c_char
    } else if seen_before != 0 {
        b"\x1B[0;34m\0" as *const u8 as *const std::os::raw::c_char
    } else {
        b"\x1B[1;36m\0" as *const u8 as *const std::os::raw::c_char
    }) as *mut std::os::raw::c_char;
    let mut regular_color_0 = (if in_exclude_list != 0 {
        b"\x1B[0;35m\0" as *const u8 as *const std::os::raw::c_char
    } else if seen_before != 0 {
        b"\x1B[0;34m\0" as *const u8 as *const std::os::raw::c_char
    } else {
        b"\x1B[0;36m\0" as *const u8 as *const std::os::raw::c_char
    }) as *mut std::os::raw::c_char;
    let mut highlight_0 = (seen_before == 0 && in_exclude_list == 0) as std::os::raw::c_int;
    print_line(
        depth,
        print_name_0,
        bold_color_0,
        regular_color_0,
        highlight_0,
        reason,
        s,
    );
    let mut exit_code = 0 as std::os::raw::c_int;
    let mut needed_not_found = needed_buf_offsets.n;
    if needed_not_found != 0 && (*s).verbosity == 0 as std::os::raw::c_int {
        apply_exclude_list(Some(&mut needed_not_found), Some(&mut needed_buf_offsets), s);
    }
    if needed_not_found != 0 {
        exit_code
            |= check_absolute_paths(
                Some(&mut needed_not_found),
                Some(&mut needed_buf_offsets),
                depth,
                s,
                curr_type,
            );
    }
    if runpath == 0xffffffffffffffff as std::os::raw::c_ulong {
        let mut j = depth as std::os::raw::c_int;
        while j >= 0 as std::os::raw::c_int && needed_not_found != 0 {
            if !((*s).rpath_offsets[j as usize] == 18446744073709551615 as std::os::raw::c_ulong)
            {
                exit_code
                    |= check_search_paths(
                        {
                            let mut init = found_t {
                                how: RPATH,
                                depth: j as size_t,
                            };
                            init
                        },
                        (*s).rpath_offsets[j as usize],
                        Some(&mut needed_not_found),
                        Some(&mut needed_buf_offsets),
                        depth,
                        s,
                        curr_type,
                    );
            }
            j -= 1;
        }
    }
    if needed_not_found != 0
        && (*s).ld_library_path_offset != 18446744073709551615 as std::os::raw::c_ulong
    {
        exit_code
            |= check_search_paths(
                {
                    let mut init = found_t {
                        how: LD_LIBRARY_PATH,
                        depth: 0,
                    };
                    init
                },
                (*s).ld_library_path_offset,
                Some(&mut needed_not_found),
                Some(&mut needed_buf_offsets),
                depth,
                s,
                curr_type,
            );
    }
    if needed_not_found != 0 && runpath != 0xffffffffffffffff as std::os::raw::c_ulong {
        exit_code
            |= check_search_paths(
                {
                    let mut init = found_t { how: RUNPATH, depth: 0 };
                    init
                },
                runpath_buf_offset,
                Some(&mut needed_not_found),
                Some(&mut needed_buf_offsets),
                depth,
                s,
                curr_type,
            );
    }
    if needed_not_found != 0 && no_def_lib == 0 {
        exit_code
            |= check_search_paths(
                {
                    let mut init = found_t {
                        how: LD_SO_CONF,
                        depth: 0,
                    };
                    init
                },
                (*s).ld_so_conf_offset,
                Some(&mut needed_not_found),
                Some(&mut needed_buf_offsets),
                depth,
                s,
                curr_type,
            );
    }
    if needed_not_found != 0 && no_def_lib == 0 {
        exit_code
            |= check_search_paths(
                {
                    let mut init = found_t { how: DEFAULT, depth: 0 };
                    init
                },
                (*s).default_paths_offset,
                Some(&mut needed_not_found),
                Some(&mut needed_buf_offsets),
                depth,
                s,
                curr_type,
            );
    }
    if needed_not_found != 0 {
        print_error(
            depth,
            needed_not_found,
            Some(&mut needed_buf_offsets),
            if runpath == 0xffffffffffffffff as std::os::raw::c_ulong {
                0 as *mut std::os::raw::c_char
            } else {
                ((*s).string_table.arr).offset(runpath_buf_offset as isize)
            },
            s,
            no_def_lib,
        );
        (*s).string_table.n = old_buf_size;
        small_vec_u64_free(Some(&mut needed_buf_offsets));
        small_vec_u64_free(Some(&mut needed));
        return 28 as std::os::raw::c_int;
    }
    (*s).string_table.n = old_buf_size;
    small_vec_u64_free(Some(&mut needed_buf_offsets));
    small_vec_u64_free(Some(&mut needed));
    return exit_code;
}
unsafe extern "C" fn ld_conf_globbing<'a1>(
    mut st: Option<&'a1 mut crate::src::libtree::string_table_t>,
    mut pattern: * mut std::os::raw::c_char,
) -> std::os::raw::c_int {
    let mut result = glob_t {
        gl_pathc: 0,
        gl_pathv: 0 as *mut *mut std::os::raw::c_char,
        gl_offs: 0,
        gl_flags: 0,
        gl_closedir: None,
        gl_readdir: None,
        gl_opendir: None,
        gl_lstat: None,
        gl_stat: None,
    };
    memset(
        &mut result as *mut glob_t as *mut std::os::raw::c_void,
        0 as std::os::raw::c_int,
        ::std::mem::size_of::<glob_t>() as std::os::raw::c_ulong,
    );
    let mut status = glob(pattern, 0 as std::os::raw::c_int, None, &mut result);
    match status {
        1 | 2 => {
            globfree(&mut result);
            return 1 as std::os::raw::c_int;
        }
        3 => {
            globfree(&mut result);
            return 0 as std::os::raw::c_int;
        }
        _ => {}
    }
    let mut code = 0 as std::os::raw::c_int;
    let mut i = 0 as std::os::raw::c_int as size_t;
    while i < result.gl_pathc {
        code |= parse_ld_config_file(borrow_mut(&mut st), *(result.gl_pathv).offset(i as isize));
        i = i.wrapping_add(1);
    }
    globfree(&mut result);
    return code;
}
unsafe extern "C" fn parse_ld_config_file<'a1>(
    mut st: Option<&'a1 mut crate::src::libtree::string_table_t>,
    mut path: * mut std::os::raw::c_char,
) -> std::os::raw::c_int {
    let mut fptr = fopen(path, b"r\0" as *const u8 as *const std::os::raw::c_char);
    if fptr.is_null() {
        return 1 as std::os::raw::c_int;
    }
    let mut c = 0 as std::os::raw::c_int;
    let mut line: [i8; 4096] = [0; 4096];
    let mut tmp: [i8; 4096] = [0; 4096];
    while c != -(1 as std::os::raw::c_int) {
        let mut line_len = 0 as std::os::raw::c_int as size_t;
        loop {
            c = _IO_getc(fptr);
            if !(c != '\n' as i32 && c != -(1 as std::os::raw::c_int)) {
                break;
            }
            if line_len < (4096 as std::os::raw::c_int - 1 as std::os::raw::c_int) as std::os::raw::c_ulong {
                let mut fresh25 = line_len;
                line_len = line_len.wrapping_add(1);
                line[fresh25 as usize] = c as std::os::raw::c_char;
            }
        }
        line[line_len as usize] = '\0' as i32 as std::os::raw::c_char;
        let mut begin = line.as_mut_ptr();
        let mut end = line.as_mut_ptr().offset(line_len as isize);
        while *(*__ctype_b_loc()).offset(*begin as std::os::raw::c_int as isize) as std::os::raw::c_int
            & _ISspace as std::os::raw::c_int as std::os::raw::c_ushort as std::os::raw::c_int != 0
        {
            begin = begin.offset(1);
        }
        let mut comment = strchr(begin, '#' as i32);
        if !comment.is_null() {
            *comment = '\0' as i32 as std::os::raw::c_char;
        }
        while end != begin {
            end = end.offset(-1);
            if *(*__ctype_b_loc()).offset(*end as std::os::raw::c_int as isize) as std::os::raw::c_int
                & _ISspace as std::os::raw::c_int as std::os::raw::c_ushort as std::os::raw::c_int == 0
            {
                break;
            }
        }
        if begin == end {
            continue;
        }
        *end.offset(1 as std::os::raw::c_int as isize) = '\0' as i32 as std::os::raw::c_char;
        if strncmp(
            begin,
            b"include\0" as *const u8 as *const std::os::raw::c_char,
            7 as std::os::raw::c_int as std::os::raw::c_ulong,
        ) == 0 as std::os::raw::c_int
            && *(*__ctype_b_loc())
                .offset(*begin.offset(7 as std::os::raw::c_int as isize) as std::os::raw::c_int as isize)
                as std::os::raw::c_int & _ISspace as std::os::raw::c_int as std::os::raw::c_ushort as std::os::raw::c_int
                != 0
        {
            begin = begin.offset(8 as std::os::raw::c_int as isize);
            while *(*__ctype_b_loc()).offset(*begin as std::os::raw::c_int as isize)
                as std::os::raw::c_int & _ISspace as std::os::raw::c_int as std::os::raw::c_ushort as std::os::raw::c_int
                != 0
            {
                begin = begin.offset(1);
            }
            if *begin as std::os::raw::c_int != '/' as i32 {
                let mut wd = strrchr(path, '/' as i32);
                wd = if wd.is_null() { strrchr(path, '\0' as i32) } else { wd };
                let mut wd_len = wd.offset_from(path) as std::os::raw::c_long as size_t;
                let mut include_len = (end.offset_from(begin) as std::os::raw::c_long
                    + 1 as std::os::raw::c_int as std::os::raw::c_long) as size_t;
                if wd_len
                    .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong)
                    .wrapping_add(include_len) >= 4096 as std::os::raw::c_int as std::os::raw::c_ulong
                {
                    continue;
                }
                memcpy(
                    tmp.as_mut_ptr() as *mut std::os::raw::c_void,
                    path as *const std::os::raw::c_void,
                    wd_len,
                );
                tmp[wd_len as usize] = '/' as i32 as std::os::raw::c_char;
                memcpy(
                    tmp
                        .as_mut_ptr()
                        .offset(wd_len as isize)
                        .offset(1 as std::os::raw::c_int as isize) as *mut std::os::raw::c_void,
                    begin as *const std::os::raw::c_void,
                    include_len,
                );
                tmp[wd_len
                    .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong)
                    .wrapping_add(include_len) as usize] = '\0' as i32 as std::os::raw::c_char;
                begin = tmp.as_mut_ptr();
            }
            ld_conf_globbing(borrow_mut(&mut st), begin);
        } else {
            string_table_store(borrow_mut(&mut st), begin);
            *((*(borrow(& st)).unwrap()).arr)
                .offset(
                    ((*(borrow(& st)).unwrap()).n).wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                ) = ':' as i32 as std::os::raw::c_char;
        }
    }
    fclose(fptr);
    return 0 as std::os::raw::c_int;
}
unsafe extern "C" fn parse_ld_so_conf(mut s: * mut crate::src::libtree::libtree_state_t) {
    let mut st: Option<&'_ mut crate::src::libtree::string_table_t> = Some(&mut (*s).string_table);
    (*s).ld_so_conf_offset = (*(borrow_mut(&mut st)).unwrap()).n;
    parse_ld_config_file(borrow_mut(&mut st), (*s).ld_conf_file);
    if (*(borrow(& st)).unwrap()).n > (*s).ld_so_conf_offset {
        *((*(borrow(& st)).unwrap()).arr)
            .offset(
                ((*(borrow(& st)).unwrap()).n).wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
            ) = '\0' as i32 as std::os::raw::c_char;
    } else {
        string_table_store(borrow_mut(&mut st), b"\0" as *const u8 as *const std::os::raw::c_char);
    };
}
unsafe extern "C" fn parse_ld_library_path(mut s: * mut crate::src::libtree::libtree_state_t) {
    (*s).ld_library_path_offset = 18446744073709551615 as std::os::raw::c_ulong;
    let mut val = getenv(b"LD_LIBRARY_PATH\0" as *const u8 as *const std::os::raw::c_char);
    if val.is_null() {
        return;
    }
    (*s).ld_library_path_offset = (*s).string_table.n;
    string_table_store(Some(&mut (*s).string_table), val);
    let mut search = ((*s).string_table.arr)
        .offset((*s).ld_library_path_offset as isize);
    loop {
        search = strchr(search, ';' as i32);
        if search.is_null() {
            break;
        }
        let mut fresh26 = search;
        search = search.offset(1);
        *fresh26 = ':' as i32 as std::os::raw::c_char;
    };
}
unsafe extern "C" fn set_default_paths(mut s: * mut crate::src::libtree::libtree_state_t) {
    (*s).default_paths_offset = (*s).string_table.n;
    string_table_store(
        Some(&mut (*s).string_table),
        b"/lib:/lib64:/usr/lib:/usr/lib64\0" as *const u8 as *const std::os::raw::c_char,
    );
}
unsafe extern "C" fn libtree_state_init(mut s: * mut crate::src::libtree::libtree_state_t) {
    (*s).string_table.n = 0 as std::os::raw::c_int as size_t;
    (*s).string_table.capacity = 1024 as std::os::raw::c_int as size_t;
    let ref mut fresh27 = (*s).string_table.arr;
    *fresh27 = malloc(
        ((*s).string_table.capacity)
            .wrapping_mul(::std::mem::size_of::<std::os::raw::c_char>() as std::os::raw::c_ulong),
    ) as *mut std::os::raw::c_char;
    (*s).visited.n = 0 as std::os::raw::c_int as size_t;
    (*s).visited.capacity = 256 as std::os::raw::c_int as size_t;
    let ref mut fresh28 = (*s).visited.arr;
    *fresh28 = malloc(
        ((*s).visited.capacity)
            .wrapping_mul(::std::mem::size_of::<visited_file_t>() as std::os::raw::c_ulong),
    ) as *mut visited_file_t;
}
unsafe extern "C" fn libtree_state_free(mut s: * mut crate::src::libtree::libtree_state_t) {
    free((*s).string_table.arr as *mut std::os::raw::c_void);
    free((*s).visited.arr as *mut std::os::raw::c_void);
}
unsafe extern "C" fn print_tree(
    mut pathc: std::os::raw::c_int,
    mut pathv: * mut * mut std::os::raw::c_char,
    mut s: * mut crate::src::libtree::libtree_state_t,
) -> std::os::raw::c_int {
    libtree_state_init(s);
    parse_ld_so_conf(s);
    parse_ld_library_path(s);
    set_default_paths(s);
    let mut exit_code = 0 as std::os::raw::c_int;
    let mut i = 0 as std::os::raw::c_int;
    while i < pathc {
        let mut code = recurse(
            *pathv.offset(i as isize),
            0 as std::os::raw::c_int as size_t,
            s,
            {
                let mut init = compat_t {
                    any: 1 as std::os::raw::c_int as std::os::raw::c_char,
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
        fflush(stdout);
        if code != 0 as std::os::raw::c_int {
            exit_code = code;
            fputs(b"Error [\0" as *const u8 as *const std::os::raw::c_char, stderr);
            fputs(*pathv.offset(i as isize), stderr);
            fputs(b"]: \0" as *const u8 as *const std::os::raw::c_char, stderr);
        }
        let mut msg = 0 as *mut std::os::raw::c_char;
        match code {
            11 => {
                msg = b"Invalid ELF magic bytes\n\0" as *const u8 as *const std::os::raw::c_char
                    as *mut std::os::raw::c_char;
            }
            12 => {
                msg = b"Invalid ELF class\n\0" as *const u8 as *const std::os::raw::c_char
                    as *mut std::os::raw::c_char;
            }
            13 => {
                msg = b"Invalid ELF data\n\0" as *const u8 as *const std::os::raw::c_char
                    as *mut std::os::raw::c_char;
            }
            14 => {
                msg = b"Invalid ELF header\n\0" as *const u8 as *const std::os::raw::c_char
                    as *mut std::os::raw::c_char;
            }
            15 => {
                msg = b"Invalid bits\n\0" as *const u8 as *const std::os::raw::c_char
                    as *mut std::os::raw::c_char;
            }
            16 => {
                msg = b"Invalid endianness\n\0" as *const u8 as *const std::os::raw::c_char
                    as *mut std::os::raw::c_char;
            }
            17 => {
                msg = b"Not an ET_EXEC or ET_DYN ELF file\n\0" as *const u8
                    as *const std::os::raw::c_char as *mut std::os::raw::c_char;
            }
            18 => {
                msg = b"Invalid ELF program header offset\n\0" as *const u8
                    as *const std::os::raw::c_char as *mut std::os::raw::c_char;
            }
            19 => {
                msg = b"Invalid ELF program header\n\0" as *const u8
                    as *const std::os::raw::c_char as *mut std::os::raw::c_char;
            }
            20 => {
                msg = b"Can't stat file\n\0" as *const u8 as *const std::os::raw::c_char
                    as *mut std::os::raw::c_char;
            }
            21 => {
                msg = b"Invalid ELF dynamic section\n\0" as *const u8
                    as *const std::os::raw::c_char as *mut std::os::raw::c_char;
            }
            22 => {
                msg = b"Invalid ELF dynamic array entry\n\0" as *const u8
                    as *const std::os::raw::c_char as *mut std::os::raw::c_char;
            }
            23 => {
                msg = b"No ELF string table found\n\0" as *const u8
                    as *const std::os::raw::c_char as *mut std::os::raw::c_char;
            }
            24 => {
                msg = b"Can't read DT_SONAME\n\0" as *const u8 as *const std::os::raw::c_char
                    as *mut std::os::raw::c_char;
            }
            25 => {
                msg = b"Can't read DT_RPATH\n\0" as *const u8 as *const std::os::raw::c_char
                    as *mut std::os::raw::c_char;
            }
            26 => {
                msg = b"Can't read DT_RUNPATH\n\0" as *const u8 as *const std::os::raw::c_char
                    as *mut std::os::raw::c_char;
            }
            27 => {
                msg = b"Can't read DT_NEEDED\n\0" as *const u8 as *const std::os::raw::c_char
                    as *mut std::os::raw::c_char;
            }
            28 => {
                msg = b"Not all dependencies were found\n\0" as *const u8
                    as *const std::os::raw::c_char as *mut std::os::raw::c_char;
            }
            29 => {
                msg = b"No PT_LOAD found in ELF file\n\0" as *const u8
                    as *const std::os::raw::c_char as *mut std::os::raw::c_char;
            }
            30 => {
                msg = b"Virtual addresses are not ordered\n\0" as *const u8
                    as *const std::os::raw::c_char as *mut std::os::raw::c_char;
            }
            31 => {
                msg = b"Could not open file\n\0" as *const u8 as *const std::os::raw::c_char
                    as *mut std::os::raw::c_char;
            }
            32 => {
                msg = b"Incompatible ISA\n\0" as *const u8 as *const std::os::raw::c_char
                    as *mut std::os::raw::c_char;
            }
            _ => {}
        }
        if !msg.is_null() {
            fputs(msg, stderr);
        }
        fflush(stderr);
        i += 1;
    }
    libtree_state_free(s);
    return exit_code;
}
unsafe fn main_0(
    mut argc: std::os::raw::c_int,
    mut argv: * mut * mut std::os::raw::c_char,
) -> std::os::raw::c_int {
    let mut s = libtree_state_t {
        verbosity: 0,
        path: 0,
        color: 0,
        ld_conf_file: 0 as *mut std::os::raw::c_char,
        max_depth: 0,
        string_table: string_table_t {
            arr: 0 as *mut std::os::raw::c_char,
            n: 0,
            capacity: 0,
        },
        visited: visited_file_array_t {
            arr: 0 as *mut visited_file_t,
            n: 0,
            capacity: 0,
        },
        PLATFORM: 0 as *mut std::os::raw::c_char,
        LIB: 0 as *mut std::os::raw::c_char,
        OSNAME: 0 as *mut std::os::raw::c_char,
        OSREL: 0 as *mut std::os::raw::c_char,
        rpath_offsets: [0; 32],
        ld_library_path_offset: 0,
        default_paths_offset: 0,
        ld_so_conf_offset: 0,
        found_all_needed: [0; 32],
    };
    s
        .color = ((getenv(b"NO_COLOR\0" as *const u8 as *const std::os::raw::c_char)).is_null()
        && isatty(1 as std::os::raw::c_int) != 0) as std::os::raw::c_int;
    s.verbosity = 0 as std::os::raw::c_int;
    s.path = 0 as std::os::raw::c_int;
    s.max_depth = 32 as std::os::raw::c_int as std::os::raw::c_ulong;
    let mut positional = 1 as std::os::raw::c_int;
    let mut uname_val = utsname {
        sysname: [0; 65],
        nodename: [0; 65],
        release: [0; 65],
        version: [0; 65],
        machine: [0; 65],
        __domainname: [0; 65],
    };
    if uname(&mut uname_val) != 0 as std::os::raw::c_int {
        return 1 as std::os::raw::c_int;
    }
    s.PLATFORM = (uname_val.machine).as_mut_ptr();
    s.OSNAME = (uname_val.sysname).as_mut_ptr();
    s.OSREL = (uname_val.release).as_mut_ptr();
    s
        .ld_conf_file = b"/etc/ld.so.conf\0" as *const u8 as *const std::os::raw::c_char
        as *mut std::os::raw::c_char;
    if strcmp(
        (uname_val.sysname).as_mut_ptr(),
        b"FreeBSD\0" as *const u8 as *const std::os::raw::c_char,
    ) == 0 as std::os::raw::c_int
    {
        s
            .ld_conf_file = b"/etc/ld-elf.so.conf\0" as *const u8 as *const std::os::raw::c_char
            as *mut std::os::raw::c_char;
    }
    s.LIB = b"lib\0" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char;
    let mut opt_help = 0 as std::os::raw::c_int;
    let mut opt_version = 0 as std::os::raw::c_int;
    let mut opt_raw = 0 as std::os::raw::c_int;
    let mut i = 1 as std::os::raw::c_int;
    while i < argc {
        let mut arg = *argv.offset(i as isize);
        if opt_raw != 0 || *arg as std::os::raw::c_int != '-' as i32
            || *arg.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int == '\0' as i32
        {
            let mut fresh29 = positional;
            positional = positional + 1;
            let ref mut fresh30 = *argv.offset(fresh29 as isize);
            *fresh30 = arg;
        } else {
            arg = arg.offset(1);
            if *arg as std::os::raw::c_int == '-' as i32 {
                arg = arg.offset(1);
                if *arg as std::os::raw::c_int == '\0' as i32 {
                    opt_raw = 1 as std::os::raw::c_int;
                } else if strcmp(arg, b"version\0" as *const u8 as *const std::os::raw::c_char)
                    == 0 as std::os::raw::c_int
                {
                    opt_version = 1 as std::os::raw::c_int;
                } else if strcmp(arg, b"path\0" as *const u8 as *const std::os::raw::c_char)
                    == 0 as std::os::raw::c_int
                {
                    s.path = 1 as std::os::raw::c_int;
                } else if strcmp(arg, b"verbose\0" as *const u8 as *const std::os::raw::c_char)
                    == 0 as std::os::raw::c_int
                {
                    s.verbosity += 1;
                } else if strcmp(arg, b"help\0" as *const u8 as *const std::os::raw::c_char)
                    == 0 as std::os::raw::c_int
                {
                    opt_help = 1 as std::os::raw::c_int;
                } else if strcmp(arg, b"ldconf\0" as *const u8 as *const std::os::raw::c_char)
                    == 0 as std::os::raw::c_int
                {
                    if i + 1 as std::os::raw::c_int == argc {
                        fputs(
                            b"Expected value after `--ldconf`\n\0" as *const u8
                                as *const std::os::raw::c_char,
                            stderr,
                        );
                        return 1 as std::os::raw::c_int;
                    }
                    i += 1;
                    s.ld_conf_file = *argv.offset(i as isize);
                } else if strcmp(arg, b"max-depth\0" as *const u8 as *const std::os::raw::c_char)
                    == 0 as std::os::raw::c_int
                {
                    if i + 1 as std::os::raw::c_int == argc {
                        fputs(
                            b"Expected value after `--max-depth`\n\0" as *const u8
                                as *const std::os::raw::c_char,
                            stderr,
                        );
                        return 1 as std::os::raw::c_int;
                    }
                    let mut ptr = 0 as *mut std::os::raw::c_char;
                    i += 1;
                    s
                        .max_depth = strtoul(
                        *argv.offset(i as isize),
                        &mut ptr,
                        10 as std::os::raw::c_int,
                    );
                    if s.max_depth > 32 as std::os::raw::c_int as std::os::raw::c_ulong {
                        s.max_depth = 32 as std::os::raw::c_int as std::os::raw::c_ulong;
                    }
                } else {
                    fputs(
                        b"Unrecognized flag `--\0" as *const u8 as *const std::os::raw::c_char,
                        stderr,
                    );
                    fputs(arg, stderr);
                    fputs(b"`\n\0" as *const u8 as *const std::os::raw::c_char, stderr);
                    return 1 as std::os::raw::c_int;
                }
            } else {
                while *arg as std::os::raw::c_int != '\0' as i32 {
                    match *arg as std::os::raw::c_int {
                        104 => {
                            opt_help = 1 as std::os::raw::c_int;
                        }
                        112 => {
                            s.path = 1 as std::os::raw::c_int;
                        }
                        118 => {
                            s.verbosity += 1;
                        }
                        _ => {
                            fputs(
                                b"Unrecognized flag `-\0" as *const u8
                                    as *const std::os::raw::c_char,
                                stderr,
                            );
                            fputs(arg, stderr);
                            fputs(b"`\n\0" as *const u8 as *const std::os::raw::c_char, stderr);
                            return 1 as std::os::raw::c_int;
                        }
                    }
                    arg = arg.offset(1);
                }
            }
        }
        i += 1;
    }
    argv = argv.offset(1);
    positional -= 1;
    if opt_help != 0 || opt_version == 0 && positional == 0 as std::os::raw::c_int {
        fputs(
            b"Show the dynamic dependency tree of ELF files\nUsage: libtree [OPTION]... [--] FILE [FILES]...\n\n  -h, --help     Print help info\n      --version  Print version info\n\nFile names starting with '-', for example '-.so', can be specified as follows:\n  libtree -- -.so\n\nLocating libs options:\n  -p, --path       Show the path of libraries instead of the soname\n  -v               Show libraries skipped by default*\n  -vv              Show dependencies of libraries skipped by default*\n  -vvv             Show dependencies of already encountered libraries\n  --ldconf <path>  Config file for extra search paths [\0"
                as *const u8 as *const std::os::raw::c_char,
            stdout,
        );
        fputs(s.ld_conf_file, stdout);
        fputs(
            b"]\n  --max-depth <n>  Limit library traversal to at most n levels of depth\n\n* For brevity, the following libraries are not shown by default:\n  \0"
                as *const u8 as *const std::os::raw::c_char,
            stdout,
        );
        let mut num_excluded = (::std::mem::size_of::<[*const std::os::raw::c_char; 14]>()
            as std::os::raw::c_ulong)
            .wrapping_div(::std::mem::size_of::<*mut std::os::raw::c_char>() as std::os::raw::c_ulong);
        let mut cursor_x = 3 as std::os::raw::c_int as size_t;
        let mut j = 0 as std::os::raw::c_int as size_t;
        while j < num_excluded {
            cursor_x = (cursor_x as std::os::raw::c_ulong)
                .wrapping_add(strlen(exclude_list[j as usize])) as size_t as size_t;
            if cursor_x > 60 as std::os::raw::c_int as std::os::raw::c_ulong {
                cursor_x = 3 as std::os::raw::c_int as size_t;
                fputs(b"\n  \0" as *const u8 as *const std::os::raw::c_char, stdout);
            }
            fputs(exclude_list[j as usize], stdout);
            if j.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) != num_excluded {
                fputs(b", \0" as *const u8 as *const std::os::raw::c_char, stdout);
            }
            j = j.wrapping_add(1);
        }
        fputs(
            b".\n\nThe following rpath/runpath substitutions are used:\n\0" as *const u8
                as *const std::os::raw::c_char,
            stdout,
        );
        fputs(b"  PLATFORM       \0" as *const u8 as *const std::os::raw::c_char, stdout);
        fputs(s.PLATFORM, stdout);
        fputs(b"\n  LIB            \0" as *const u8 as *const std::os::raw::c_char, stdout);
        fputs(s.LIB, stdout);
        fputs(b"\n  OSNAME         \0" as *const u8 as *const std::os::raw::c_char, stdout);
        fputs(s.OSNAME, stdout);
        fputs(b"\n  OSREL          \0" as *const u8 as *const std::os::raw::c_char, stdout);
        fputs(s.OSREL, stdout);
        putchar('\n' as i32);
        return (opt_help == 0) as std::os::raw::c_int;
    }
    if opt_version != 0 {
        puts(b"3.1.1\0" as *const u8 as *const std::os::raw::c_char);
        return 0 as std::os::raw::c_int;
    }
    return print_tree(positional, argv, &mut s);
}
pub fn main() {
    let mut args: Vec::<*mut std::os::raw::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as std::os::raw::c_int,
                args.as_mut_ptr() as *mut *mut std::os::raw::c_char,
            ) as i32,
        )
    }
}
use crate::laertes_rt::*;
