use :: libc;
extern "C" {
    #[no_mangle]
    fn strtol(_: *const i8, _: *mut *mut i8, _: i32) -> i64;
    #[no_mangle]
    fn malloc(_: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    #[no_mangle]
    fn snprintf(_: *mut i8, _: u64, _: *const i8, _: ...) -> i32;
    #[no_mangle]
    fn vsnprintf(_: *mut i8, _: u64, _: *const i8, _: ::std::ffi::VaList) -> i32;
    #[no_mangle]
    fn fread(_: *mut libc::c_void, _: u64, _: u64, _: *mut FILE) -> u64;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    #[no_mangle]
    fn strncpy(_: *mut i8, _: *const i8, _: u64) -> *mut i8;
    #[no_mangle]
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    #[no_mangle]
    fn strncmp(_: *const i8, _: *const i8, _: u64) -> i32;
    #[no_mangle]
    fn strdup(_: *const i8) -> *mut i8;
    #[no_mangle]
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    #[no_mangle]
    fn strcspn(_: *const i8, _: *const i8) -> u64;
    #[no_mangle]
    fn strspn(_: *const i8, _: *const i8) -> u64;
    #[no_mangle]
    fn strstr(_: *const i8, _: *const i8) -> *mut i8;
    #[no_mangle]
    fn strlen(_: *const i8) -> u64;
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const u16;
    #[no_mangle]
    fn close(__fd: i32) -> i32;
    #[no_mangle]
    fn read(__fd: i32, __buf: *mut libc::c_void, __nbytes: u64) -> i64;
    #[no_mangle]
    fn sysconf(__name: i32) -> i64;
    #[no_mangle]
    fn mmap(
        __addr: *mut libc::c_void,
        __len: u64,
        __prot: i32,
        __flags: i32,
        __fd: i32,
        __offset: i64,
    ) -> *mut libc::c_void;
    #[no_mangle]
    fn munmap(__addr: *mut libc::c_void, __len: u64) -> i32;
    #[no_mangle]
    fn madvise(__addr: *mut libc::c_void, __len: u64, __advice: i32) -> i32;
    #[no_mangle]
    fn __fxstat(__ver: i32, __fildes: i32, __stat_buf: *mut stat) -> i32;
    #[no_mangle]
    fn open(__file: *const i8, __oflag: i32, _: ...) -> i32;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: u32,
    pub fp_offset: u32,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: i32,
    pub _IO_read_ptr: *mut i8,
    pub _IO_read_end: *mut i8,
    pub _IO_read_base: *mut i8,
    pub _IO_write_base: *mut i8,
    pub _IO_write_ptr: *mut i8,
    pub _IO_write_end: *mut i8,
    pub _IO_buf_base: *mut i8,
    pub _IO_buf_end: *mut i8,
    pub _IO_save_base: *mut i8,
    pub _IO_backup_base: *mut i8,
    pub _IO_save_end: *mut i8,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: i32,
    pub _flags2: i32,
    pub _old_offset: i64,
    pub _cur_column: u16,
    pub _vtable_offset: i8,
    pub _shortbuf: [i8; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: i64,
    pub __pad1: *mut libc::c_void,
    pub __pad2: *mut libc::c_void,
    pub __pad3: *mut libc::c_void,
    pub __pad4: *mut libc::c_void,
    pub __pad5: u64,
    pub _mode: i32,
    pub _unused2: [i8; 20],
}
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: i32,
}
pub type FILE = _IO_FILE;
pub type va_list = __builtin_va_list;
pub const _ISalnum: u32 = 8;
pub const _ISpunct: u32 = 4;
pub const _IScntrl: u32 = 2;
pub const _ISblank: u32 = 1;
pub const _ISgraph: u32 = 32768;
pub const _ISprint: u32 = 16384;
pub const _ISspace: u32 = 8192;
pub const _ISxdigit: u32 = 4096;
pub const _ISdigit: u32 = 2048;
pub const _ISalpha: u32 = 1024;
pub const _ISlower: u32 = 512;
pub const _ISupper: u32 = 256;
pub const _SC_THREAD_ROBUST_PRIO_PROTECT: u32 = 248;
pub const _SC_THREAD_ROBUST_PRIO_INHERIT: u32 = 247;
pub const _SC_XOPEN_STREAMS: u32 = 246;
pub const _SC_TRACE_USER_EVENT_MAX: u32 = 245;
pub const _SC_TRACE_SYS_MAX: u32 = 244;
pub const _SC_TRACE_NAME_MAX: u32 = 243;
pub const _SC_TRACE_EVENT_NAME_MAX: u32 = 242;
pub const _SC_SS_REPL_MAX: u32 = 241;
pub const _SC_V7_LPBIG_OFFBIG: u32 = 240;
pub const _SC_V7_LP64_OFF64: u32 = 239;
pub const _SC_V7_ILP32_OFFBIG: u32 = 238;
pub const _SC_V7_ILP32_OFF32: u32 = 237;
pub const _SC_RAW_SOCKETS: u32 = 236;
pub const _SC_IPV6: u32 = 235;
pub const _SC_LEVEL4_CACHE_LINESIZE: u32 = 199;
pub const _SC_LEVEL4_CACHE_ASSOC: u32 = 198;
pub const _SC_LEVEL4_CACHE_SIZE: u32 = 197;
pub const _SC_LEVEL3_CACHE_LINESIZE: u32 = 196;
pub const _SC_LEVEL3_CACHE_ASSOC: u32 = 195;
pub const _SC_LEVEL3_CACHE_SIZE: u32 = 194;
pub const _SC_LEVEL2_CACHE_LINESIZE: u32 = 193;
pub const _SC_LEVEL2_CACHE_ASSOC: u32 = 192;
pub const _SC_LEVEL2_CACHE_SIZE: u32 = 191;
pub const _SC_LEVEL1_DCACHE_LINESIZE: u32 = 190;
pub const _SC_LEVEL1_DCACHE_ASSOC: u32 = 189;
pub const _SC_LEVEL1_DCACHE_SIZE: u32 = 188;
pub const _SC_LEVEL1_ICACHE_LINESIZE: u32 = 187;
pub const _SC_LEVEL1_ICACHE_ASSOC: u32 = 186;
pub const _SC_LEVEL1_ICACHE_SIZE: u32 = 185;
pub const _SC_TRACE_LOG: u32 = 184;
pub const _SC_TRACE_INHERIT: u32 = 183;
pub const _SC_TRACE_EVENT_FILTER: u32 = 182;
pub const _SC_TRACE: u32 = 181;
pub const _SC_HOST_NAME_MAX: u32 = 180;
pub const _SC_V6_LPBIG_OFFBIG: u32 = 179;
pub const _SC_V6_LP64_OFF64: u32 = 178;
pub const _SC_V6_ILP32_OFFBIG: u32 = 177;
pub const _SC_V6_ILP32_OFF32: u32 = 176;
pub const _SC_2_PBS_CHECKPOINT: u32 = 175;
pub const _SC_STREAMS: u32 = 174;
pub const _SC_SYMLOOP_MAX: u32 = 173;
pub const _SC_2_PBS_TRACK: u32 = 172;
pub const _SC_2_PBS_MESSAGE: u32 = 171;
pub const _SC_2_PBS_LOCATE: u32 = 170;
pub const _SC_2_PBS_ACCOUNTING: u32 = 169;
pub const _SC_2_PBS: u32 = 168;
pub const _SC_USER_GROUPS_R: u32 = 167;
pub const _SC_USER_GROUPS: u32 = 166;
pub const _SC_TYPED_MEMORY_OBJECTS: u32 = 165;
pub const _SC_TIMEOUTS: u32 = 164;
pub const _SC_SYSTEM_DATABASE_R: u32 = 163;
pub const _SC_SYSTEM_DATABASE: u32 = 162;
pub const _SC_THREAD_SPORADIC_SERVER: u32 = 161;
pub const _SC_SPORADIC_SERVER: u32 = 160;
pub const _SC_SPAWN: u32 = 159;
pub const _SC_SIGNALS: u32 = 158;
pub const _SC_SHELL: u32 = 157;
pub const _SC_REGEX_VERSION: u32 = 156;
pub const _SC_REGEXP: u32 = 155;
pub const _SC_SPIN_LOCKS: u32 = 154;
pub const _SC_READER_WRITER_LOCKS: u32 = 153;
pub const _SC_NETWORKING: u32 = 152;
pub const _SC_SINGLE_PROCESS: u32 = 151;
pub const _SC_MULTI_PROCESS: u32 = 150;
pub const _SC_MONOTONIC_CLOCK: u32 = 149;
pub const _SC_FILE_SYSTEM: u32 = 148;
pub const _SC_FILE_LOCKING: u32 = 147;
pub const _SC_FILE_ATTRIBUTES: u32 = 146;
pub const _SC_PIPE: u32 = 145;
pub const _SC_FIFO: u32 = 144;
pub const _SC_FD_MGMT: u32 = 143;
pub const _SC_DEVICE_SPECIFIC_R: u32 = 142;
pub const _SC_DEVICE_SPECIFIC: u32 = 141;
pub const _SC_DEVICE_IO: u32 = 140;
pub const _SC_THREAD_CPUTIME: u32 = 139;
pub const _SC_CPUTIME: u32 = 138;
pub const _SC_CLOCK_SELECTION: u32 = 137;
pub const _SC_C_LANG_SUPPORT_R: u32 = 136;
pub const _SC_C_LANG_SUPPORT: u32 = 135;
pub const _SC_BASE: u32 = 134;
pub const _SC_BARRIERS: u32 = 133;
pub const _SC_ADVISORY_INFO: u32 = 132;
pub const _SC_XOPEN_REALTIME_THREADS: u32 = 131;
pub const _SC_XOPEN_REALTIME: u32 = 130;
pub const _SC_XOPEN_LEGACY: u32 = 129;
pub const _SC_XBS5_LPBIG_OFFBIG: u32 = 128;
pub const _SC_XBS5_LP64_OFF64: u32 = 127;
pub const _SC_XBS5_ILP32_OFFBIG: u32 = 126;
pub const _SC_XBS5_ILP32_OFF32: u32 = 125;
pub const _SC_NL_TEXTMAX: u32 = 124;
pub const _SC_NL_SETMAX: u32 = 123;
pub const _SC_NL_NMAX: u32 = 122;
pub const _SC_NL_MSGMAX: u32 = 121;
pub const _SC_NL_LANGMAX: u32 = 120;
pub const _SC_NL_ARGMAX: u32 = 119;
pub const _SC_USHRT_MAX: u32 = 118;
pub const _SC_ULONG_MAX: u32 = 117;
pub const _SC_UINT_MAX: u32 = 116;
pub const _SC_UCHAR_MAX: u32 = 115;
pub const _SC_SHRT_MIN: u32 = 114;
pub const _SC_SHRT_MAX: u32 = 113;
pub const _SC_SCHAR_MIN: u32 = 112;
pub const _SC_SCHAR_MAX: u32 = 111;
pub const _SC_SSIZE_MAX: u32 = 110;
pub const _SC_NZERO: u32 = 109;
pub const _SC_MB_LEN_MAX: u32 = 108;
pub const _SC_WORD_BIT: u32 = 107;
pub const _SC_LONG_BIT: u32 = 106;
pub const _SC_INT_MIN: u32 = 105;
pub const _SC_INT_MAX: u32 = 104;
pub const _SC_CHAR_MIN: u32 = 103;
pub const _SC_CHAR_MAX: u32 = 102;
pub const _SC_CHAR_BIT: u32 = 101;
pub const _SC_XOPEN_XPG4: u32 = 100;
pub const _SC_XOPEN_XPG3: u32 = 99;
pub const _SC_XOPEN_XPG2: u32 = 98;
pub const _SC_2_UPE: u32 = 97;
pub const _SC_2_C_VERSION: u32 = 96;
pub const _SC_2_CHAR_TERM: u32 = 95;
pub const _SC_XOPEN_SHM: u32 = 94;
pub const _SC_XOPEN_ENH_I18N: u32 = 93;
pub const _SC_XOPEN_CRYPT: u32 = 92;
pub const _SC_XOPEN_UNIX: u32 = 91;
pub const _SC_XOPEN_XCU_VERSION: u32 = 90;
pub const _SC_XOPEN_VERSION: u32 = 89;
pub const _SC_PASS_MAX: u32 = 88;
pub const _SC_ATEXIT_MAX: u32 = 87;
pub const _SC_AVPHYS_PAGES: u32 = 86;
pub const _SC_PHYS_PAGES: u32 = 85;
pub const _SC_NPROCESSORS_ONLN: u32 = 84;
pub const _SC_NPROCESSORS_CONF: u32 = 83;
pub const _SC_THREAD_PROCESS_SHARED: u32 = 82;
pub const _SC_THREAD_PRIO_PROTECT: u32 = 81;
pub const _SC_THREAD_PRIO_INHERIT: u32 = 80;
pub const _SC_THREAD_PRIORITY_SCHEDULING: u32 = 79;
pub const _SC_THREAD_ATTR_STACKSIZE: u32 = 78;
pub const _SC_THREAD_ATTR_STACKADDR: u32 = 77;
pub const _SC_THREAD_THREADS_MAX: u32 = 76;
pub const _SC_THREAD_STACK_MIN: u32 = 75;
pub const _SC_THREAD_KEYS_MAX: u32 = 74;
pub const _SC_THREAD_DESTRUCTOR_ITERATIONS: u32 = 73;
pub const _SC_TTY_NAME_MAX: u32 = 72;
pub const _SC_LOGIN_NAME_MAX: u32 = 71;
pub const _SC_GETPW_R_SIZE_MAX: u32 = 70;
pub const _SC_GETGR_R_SIZE_MAX: u32 = 69;
pub const _SC_THREAD_SAFE_FUNCTIONS: u32 = 68;
pub const _SC_THREADS: u32 = 67;
pub const _SC_T_IOV_MAX: u32 = 66;
pub const _SC_PII_OSI_M: u32 = 65;
pub const _SC_PII_OSI_CLTS: u32 = 64;
pub const _SC_PII_OSI_COTS: u32 = 63;
pub const _SC_PII_INTERNET_DGRAM: u32 = 62;
pub const _SC_PII_INTERNET_STREAM: u32 = 61;
pub const _SC_IOV_MAX: u32 = 60;
pub const _SC_UIO_MAXIOV: u32 = 60;
pub const _SC_SELECT: u32 = 59;
pub const _SC_POLL: u32 = 58;
pub const _SC_PII_OSI: u32 = 57;
pub const _SC_PII_INTERNET: u32 = 56;
pub const _SC_PII_SOCKET: u32 = 55;
pub const _SC_PII_XTI: u32 = 54;
pub const _SC_PII: u32 = 53;
pub const _SC_2_LOCALEDEF: u32 = 52;
pub const _SC_2_SW_DEV: u32 = 51;
pub const _SC_2_FORT_RUN: u32 = 50;
pub const _SC_2_FORT_DEV: u32 = 49;
pub const _SC_2_C_DEV: u32 = 48;
pub const _SC_2_C_BIND: u32 = 47;
pub const _SC_2_VERSION: u32 = 46;
pub const _SC_CHARCLASS_NAME_MAX: u32 = 45;
pub const _SC_RE_DUP_MAX: u32 = 44;
pub const _SC_LINE_MAX: u32 = 43;
pub const _SC_EXPR_NEST_MAX: u32 = 42;
pub const _SC_EQUIV_CLASS_MAX: u32 = 41;
pub const _SC_COLL_WEIGHTS_MAX: u32 = 40;
pub const _SC_BC_STRING_MAX: u32 = 39;
pub const _SC_BC_SCALE_MAX: u32 = 38;
pub const _SC_BC_DIM_MAX: u32 = 37;
pub const _SC_BC_BASE_MAX: u32 = 36;
pub const _SC_TIMER_MAX: u32 = 35;
pub const _SC_SIGQUEUE_MAX: u32 = 34;
pub const _SC_SEM_VALUE_MAX: u32 = 33;
pub const _SC_SEM_NSEMS_MAX: u32 = 32;
pub const _SC_RTSIG_MAX: u32 = 31;
pub const _SC_PAGESIZE: u32 = 30;
pub const _SC_VERSION: u32 = 29;
pub const _SC_MQ_PRIO_MAX: u32 = 28;
pub const _SC_MQ_OPEN_MAX: u32 = 27;
pub const _SC_DELAYTIMER_MAX: u32 = 26;
pub const _SC_AIO_PRIO_DELTA_MAX: u32 = 25;
pub const _SC_AIO_MAX: u32 = 24;
pub const _SC_AIO_LISTIO_MAX: u32 = 23;
pub const _SC_SHARED_MEMORY_OBJECTS: u32 = 22;
pub const _SC_SEMAPHORES: u32 = 21;
pub const _SC_MESSAGE_PASSING: u32 = 20;
pub const _SC_MEMORY_PROTECTION: u32 = 19;
pub const _SC_MEMLOCK_RANGE: u32 = 18;
pub const _SC_MEMLOCK: u32 = 17;
pub const _SC_MAPPED_FILES: u32 = 16;
pub const _SC_FSYNC: u32 = 15;
pub const _SC_SYNCHRONIZED_IO: u32 = 14;
pub const _SC_PRIORITIZED_IO: u32 = 13;
pub const _SC_ASYNCHRONOUS_IO: u32 = 12;
pub const _SC_TIMERS: u32 = 11;
pub const _SC_PRIORITY_SCHEDULING: u32 = 10;
pub const _SC_REALTIME_SIGNALS: u32 = 9;
pub const _SC_SAVED_IDS: u32 = 8;
pub const _SC_JOB_CONTROL: u32 = 7;
pub const _SC_TZNAME_MAX: u32 = 6;
pub const _SC_STREAM_MAX: u32 = 5;
pub const _SC_OPEN_MAX: u32 = 4;
pub const _SC_NGROUPS_MAX: u32 = 3;
pub const _SC_CLK_TCK: u32 = 2;
pub const _SC_CHILD_MAX: u32 = 1;
pub const _SC_ARG_MAX: u32 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: u64,
    pub st_ino: u64,
    pub st_nlink: u64,
    pub st_mode: u32,
    pub st_uid: u32,
    pub st_gid: u32,
    pub __pad0: i32,
    pub st_rdev: u64,
    pub st_size: i64,
    pub st_blksize: i64,
    pub st_blocks: i64,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [i64; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ezxml {
    pub name: *mut i8,
    pub attr: *mut *mut i8,
    pub txt: *mut i8,
    pub off: u64,
    pub next: ezxml_t,
    pub sibling: ezxml_t,
    pub ordered: ezxml_t,
    pub child: ezxml_t,
    pub parent: ezxml_t,
    pub flags: i16,
}
pub type ezxml_t = *mut ezxml;
pub type ezxml_root_t = *mut ezxml_root;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ezxml_root {
    pub xml: ezxml,
    pub cur: ezxml_t,
    pub m: *mut i8,
    pub len: u64,
    pub u: *mut i8,
    pub s: *mut i8,
    pub e: *mut i8,
    pub ent: *mut *mut i8,
    pub attr: *mut *mut *mut i8,
    pub pi: *mut *mut *mut i8,
    pub standalone: i16,
    pub err: [i8; 128],
}
#[inline]
extern "C" fn fstat(mut __fd: i32, mut __statbuf: *mut stat) -> i32 {
    unsafe {
        return __fxstat(1, __fd, __statbuf);
    }
}

#[no_mangle]
pub static mut EZXML_NIL: [*mut i8; 1] = [0 as *const i8 as *mut i8];
#[no_mangle]
pub extern "C" fn ezxml_child(mut xml: ezxml_t, mut name: *const i8) -> ezxml_t {
    unsafe {
        xml = if !xml.is_null() {
            (*xml).child
        } else {
            0 as ezxml_t
        };
        while !xml.is_null() && strcmp(name, (*xml).name) != 0 {
            xml = (*xml).sibling
        }
    }
    return xml;
}

#[no_mangle]
pub extern "C" fn ezxml_idx(mut xml: ezxml_t, mut idx: i32) -> ezxml_t {
    unsafe {
        while !xml.is_null() && idx != 0 {
            xml = (*xml).next;
            idx -= 1
        }
    }
    return xml;
}

#[no_mangle]
pub extern "C" fn ezxml_attr(mut xml: ezxml_t, mut attr: *const i8) -> *const i8 {
    let mut i: i32 = 0;
    let mut j: i32 = 1;
    let mut root: ezxml_root_t = xml as ezxml_root_t;
    unsafe {
        if xml.is_null() || (*xml).attr.is_null() {
            return 0 as *const i8;
        }
        while !(*(*xml).attr.offset(i as isize)).is_null()
            && strcmp(attr, *(*xml).attr.offset(i as isize)) != 0
        {
            i += 2
        }
        if !(*(*xml).attr.offset(i as isize)).is_null() {
            return *(*xml).attr.offset((i + 1i32) as isize);
        }
        while !(*root).xml.parent.is_null() {
            root = (*root).xml.parent as ezxml_root_t
        }
    }
    i = 0;
    unsafe {
        while !(*(*root).attr.offset(i as isize)).is_null()
            && strcmp(
                (*xml).name,
                *(*(*root).attr.offset(i as isize)).offset(0 as isize),
            ) != 0
        {
            i += 1
        }
        if (*(*root).attr.offset(i as isize)).is_null() {
            return 0 as *const i8;
        }
        while !(*(*(*root).attr.offset(i as isize)).offset(j as isize)).is_null()
            && strcmp(attr, *(*(*root).attr.offset(i as isize)).offset(j as isize)) != 0
        {
            j += 3
        }
        return if !(*(*(*root).attr.offset(i as isize)).offset(j as isize)).is_null() {
            *(*(*root).attr.offset(i as isize)).offset((j + 1i32) as isize)
        } else {
            0 as *mut i8
        };
    }
}

#[no_mangle]
pub extern "C" fn ezxml_vget(mut xml: ezxml_t, mut ap: ::std::ffi::VaList) -> ezxml_t {
    unsafe {
        let mut name: *mut i8 = ap.arg::<*mut i8>();
        let mut idx: i32 = -1;
        if !name.is_null() && *name as i32 != 0 {
            idx = ap.arg::<i32>();
            xml = ezxml_child(xml, name)
        }
        return if idx < 0 {
            xml
        } else {
            ezxml_vget(ezxml_idx(xml, idx), ap.as_va_list())
        };
    }
}

#[no_mangle]
pub unsafe extern "C" fn ezxml_get(mut xml: ezxml_t, mut args: ...) -> ezxml_t {
    let mut ap: ::std::ffi::VaListImpl;
    let mut r: ezxml_t = 0 as *mut ezxml;
    ap = args.clone();
    r = ezxml_vget(xml, ap.as_va_list());
    return r;
}

#[no_mangle]
pub extern "C" fn ezxml_pi(mut xml: ezxml_t, mut target: *const i8) -> *mut *const i8 {
    let mut root: ezxml_root_t = xml as ezxml_root_t;
    let mut i: i32 = 0;
    unsafe {
        if root.is_null() {
            return EZXML_NIL.as_mut_ptr() as *mut *const i8;
        }
        while !(*root).xml.parent.is_null() {
            root = (*root).xml.parent as ezxml_root_t
        }
        while !(*(*root).pi.offset(i as isize)).is_null()
            && strcmp(target, *(*(*root).pi.offset(i as isize)).offset(0 as isize)) != 0
        {
            i += 1
        }
        return if !(*(*root).pi.offset(i as isize)).is_null() {
            (*(*root).pi.offset(i as isize)).offset(1 as isize)
        } else {
            EZXML_NIL.as_mut_ptr()
        } as *mut *const i8;
    }
}

#[no_mangle]
pub unsafe extern "C" fn ezxml_err(
    mut root: ezxml_root_t,
    mut s: *mut i8,
    mut err: *const i8,
    mut args: ...
) -> ezxml_t {
    let mut ap: ::std::ffi::VaListImpl;
    let mut line: i32 = 1;
    let mut t: *mut i8 = 0 as *mut i8;
    let mut fmt: [i8; 128] = [0; 128];
    t = (*root).s;
    while t < s {
        if *t as i32 == '\n' as i32 {
            line += 1
        }
        t = t.offset(1)
    }
    snprintf(
        fmt.as_mut_ptr(),
        128,
        b"[error near line %d]: %s\x00" as *const u8 as *const i8,
        line,
        err,
    );
    ap = args.clone();
    vsnprintf(
        (*root).err.as_mut_ptr(),
        128,
        fmt.as_mut_ptr(),
        ap.as_va_list(),
    );
    return &mut (*root).xml;
}

#[no_mangle]
pub extern "C" fn ezxml_decode(mut s: *mut i8, mut ent: *mut *mut i8, mut t: i8) -> *mut i8 {
    let mut e: *mut i8 = 0 as *mut i8;
    unsafe {
        let mut r: *mut i8 = s;
        let mut m: *mut i8 = s;
        let mut b: i64 = 0;
        let mut c: i64 = 0;
        let mut d: i64 = 0;
        let mut l: i64 = 0;
        while *s != 0 {
            while *s as i32 == '\r' as i32 {
                let fresh0 = s;
                s = s.offset(1);
                *fresh0 = '\n' as i8;
                if *s as i32 == '\n' as i32 {
                    memmove(
                        s as *mut libc::c_void,
                        s.offset(1 as isize) as *const libc::c_void,
                        strlen(s),
                    );
                }
            }
            s = s.offset(1)
        }
        s = r;
        loop {
            while *s as i32 != 0
                && *s as i32 != '&' as i32
                && (*s as i32 != '%' as i32 || t as i32 != '%' as i32)
                && *(*__ctype_b_loc()).offset(*s as i32 as isize) as i32 & _ISspace as i32 == 0
            {
                s = s.offset(1)
            }
            if *s == 0 {
                break;
            }
            if t as i32 != 'c' as i32 && strncmp(s, b"&#\x00" as *const u8 as *const i8, 2) == 0 {
                if *s.offset(2 as isize) as i32 == 'x' as i32 {
                    c = strtol(s.offset(3 as isize), &mut e, 16)
                } else {
                    c = strtol(s.offset(2 as isize), &mut e, 10)
                }
                if c == 0 || *e as i32 != ';' as i32 {
                    s = s.offset(1)
                } else {
                    if c < 0x80 {
                        let fresh1 = s;
                        s = s.offset(1);
                        *fresh1 = c as i8
                    } else {
                        b = 0;
                        d = c;
                        while d != 0 {
                            b += 1;
                            d /= 2
                        }
                        b = (b - 2) / 5;
                        let fresh2 = s;
                        s = s.offset(1);
                        *fresh2 = ((0xffi32 << 7 - b) as i64 | c >> 6 * b) as i8;
                        while b != 0 {
                            b -= 1;
                            let fresh3 = s;
                            s = s.offset(1);
                            *fresh3 = (0x80 | c >> 6 * b & 0x3fi64) as i8
                        }
                    }
                    memmove(
                        s as *mut libc::c_void,
                        strchr(s, ';' as i32).offset(1 as isize) as *const libc::c_void,
                        strlen(strchr(s, ';' as i32)),
                    );
                }
            } else if *s as i32 == '&' as i32
                && (t as i32 == '&' as i32 || t as i32 == ' ' as i32 || t as i32 == '*' as i32)
                || *s as i32 == '%' as i32 && t as i32 == '%' as i32
            {
                b = 0;
                while !(*ent.offset(b as isize)).is_null()
                    && strncmp(
                        s.offset(1 as isize),
                        *ent.offset(b as isize),
                        strlen(*ent.offset(b as isize)),
                    ) != 0
                {
                    b += 2
                }
                let fresh4 = b;
                b = b + 1;
                if !(*ent.offset(fresh4 as isize)).is_null() {
                    c = strlen(*ent.offset(b as isize)) as i64;
                    e = strchr(s, ';' as i32);
                    if c - 1 > e.wrapping_offset_from(s) as i64 {
                        d = s.wrapping_offset_from(r) as i64;
                        l = ((d + c) as u64).wrapping_add(strlen(e)) as i64;
                        r = if r == m {
                            strcpy(malloc(l as u64) as *mut i8, r) as *mut libc::c_void
                        } else {
                            realloc(r as *mut libc::c_void, l as u64)
                        } as *mut i8;
                        s = r.offset(d as isize);
                        e = strchr(s, ';' as i32)
                    }
                    memmove(
                        s.offset(c as isize) as *mut libc::c_void,
                        e.offset(1 as isize) as *const libc::c_void,
                        strlen(e),
                    );
                    strncpy(s, *ent.offset(b as isize), c as u64);
                } else {
                    s = s.offset(1)
                }
            } else if (t as i32 == ' ' as i32 || t as i32 == '*' as i32)
                && *(*__ctype_b_loc()).offset(*s as i32 as isize) as i32 & _ISspace as i32 != 0
            {
                let fresh5 = s;
                s = s.offset(1);
                *fresh5 = ' ' as i8
            } else {
                s = s.offset(1)
            }
        }
        if t as i32 == '*' as i32 {
            s = r;
            while *s != 0 {
                l = strspn(s, b" \x00" as *const u8 as *const i8) as i64;
                if l != 0 {
                    memmove(
                        s as *mut libc::c_void,
                        s.offset(l as isize) as *const libc::c_void,
                        strlen(s.offset(l as isize)).wrapping_add(1),
                    );
                }
                while *s as i32 != 0 && *s as i32 != ' ' as i32 {
                    s = s.offset(1)
                }
                s = s.offset(1)
            }
            s = s.offset(-1);
            if s >= r && *s as i32 == ' ' as i32 {
                *s = '\u{0}' as i8
            }
        }
        return r;
    }
}

#[no_mangle]
pub extern "C" fn ezxml_open_tag(
    mut root: ezxml_root_t,
    mut name: *mut i8,
    mut attr: *mut *mut i8,
) {
    unsafe {
        let mut xml: ezxml_t = (*root).cur;
        if !(*xml).name.is_null() {
            xml = ezxml_add_child(xml, name, strlen((*xml).txt))
        } else {
            (*xml).name = name
        };
        (*xml).attr = attr;
        (*root).cur = xml;
    }
}

#[no_mangle]
pub extern "C" fn ezxml_char_content(
    mut root: ezxml_root_t,
    mut s: *mut i8,
    mut len: u64,
    mut t: i8,
) {
    unsafe {
        let mut xml: ezxml_t = (*root).cur;
        let mut m: *mut i8 = s;
        let mut l: u64 = 0;
        if xml.is_null() || (*xml).name.is_null() || len == 0 {
            return;
        };
        *s.offset(len as isize) = '\u{0}' as i8;
        s = ezxml_decode(s, (*root).ent, t);
        len = strlen(s).wrapping_add(1);
        if *(*xml).txt == 0 {
            (*xml).txt = s
        } else {
            (*xml).txt = if (*xml).flags as i32 & 0x40 != 0 {
                l = strlen((*xml).txt);
                realloc((*xml).txt as *mut libc::c_void, l.wrapping_add(len))
            } else {
                l = strlen((*xml).txt);
                strcpy(malloc(l.wrapping_add(len)) as *mut i8, (*xml).txt) as *mut libc::c_void
            } as *mut i8;
            strcpy((*xml).txt.offset(l as isize), s);
            if s != m {
                free(s as *mut libc::c_void);
            }
        }
        if (*xml).txt != m {
            ezxml_set_flag(xml, 0x40);
        };
    }
}

#[no_mangle]
pub extern "C" fn ezxml_close_tag(
    mut root: ezxml_root_t,
    mut name: *mut i8,
    mut s: *mut i8,
) -> ezxml_t {
    unsafe {
        if (*root).cur.is_null()
            || (*(*root).cur).name.is_null()
            || strcmp(name, (*(*root).cur).name) != 0
        {
            return ezxml_err(
                root,
                s,
                b"unexpected closing tag </%s>\x00" as *const u8 as *const i8,
                name,
            );
        };
        (*root).cur = (*(*root).cur).parent;
    }
    return 0 as ezxml_t;
}

#[no_mangle]
pub extern "C" fn ezxml_ent_ok(mut name: *mut i8, mut s: *mut i8, mut ent: *mut *mut i8) -> i32 {
    let mut i: i32 = 0;
    unsafe {
        loop {
            while *s as i32 != 0 && *s as i32 != '&' as i32 {
                s = s.offset(1)
            }
            if *s == 0 {
                return 1;
            }
            if strncmp(s.offset(1 as isize), name, strlen(name)) == 0 {
                return 0;
            }
            i = 0;
            while !(*ent.offset(i as isize)).is_null()
                && strncmp(
                    *ent.offset(i as isize),
                    s.offset(1 as isize),
                    strlen(*ent.offset(i as isize)),
                ) != 0
            {
                i += 2
            }
            if !(*ent.offset(i as isize)).is_null()
                && ezxml_ent_ok(name, *ent.offset((i + 1i32) as isize), ent) == 0
            {
                return 0;
            }
            s = s.offset(1)
        }
    }
}

#[no_mangle]
pub extern "C" fn ezxml_proc_inst(mut root: ezxml_root_t, mut s: *mut i8, mut len: u64) {
    let mut i: i32 = 0;
    let mut j: i32 = 1;
    unsafe {
        let mut target: *mut i8 = s;
        *s.offset(len as isize) = '\u{0}' as i8;
        s = s.offset(strcspn(s, b"\t\r\n \x00" as *const u8 as *const i8) as isize);
        if *s != 0 {
            *s = '\u{0}' as i8;
            s = s.offset(
                strspn(
                    s.offset(1 as isize),
                    b"\t\r\n \x00" as *const u8 as *const i8,
                )
                .wrapping_add(1) as isize,
            )
        }
        if strcmp(target, b"xml\x00" as *const u8 as *const i8) == 0 {
            s = strstr(s, b"standalone\x00" as *const u8 as *const i8);
            if !s.is_null()
                && strncmp(
                    s.offset(strspn(
                        s.offset(10 as isize),
                        b"\t\r\n =\'\"\x00" as *const u8 as *const i8,
                    ) as isize)
                        .offset(10 as isize),
                    b"yes\x00" as *const u8 as *const i8,
                    3,
                ) == 0
            {
                (*root).standalone = 1
            }
            return;
        }
        if (*(*root).pi.offset(0 as isize)).is_null() {
            (*root).pi = malloc(::std::mem::size_of::<*mut *mut i8>() as u64) as *mut *mut *mut i8;
            *(*root).pi = 0 as *mut *mut i8
        }
        while !(*(*root).pi.offset(i as isize)).is_null()
            && strcmp(target, *(*(*root).pi.offset(i as isize)).offset(0 as isize)) != 0
        {
            i += 1
        }
        if (*(*root).pi.offset(i as isize)).is_null() {
            (*root).pi = realloc(
                (*root).pi as *mut libc::c_void,
                (::std::mem::size_of::<*mut *mut i8>() as u64).wrapping_mul((i + 2i32) as u64),
            ) as *mut *mut *mut i8;
            let ref mut fresh6 = *(*root).pi.offset(i as isize);
            *fresh6 =
                malloc((::std::mem::size_of::<*mut i8>() as u64).wrapping_mul(3)) as *mut *mut i8;
            let ref mut fresh7 = *(*(*root).pi.offset(i as isize)).offset(0 as isize);
            *fresh7 = target;
            let ref mut fresh8 = *(*root).pi.offset((i + 1i32) as isize);
            *fresh8 = 0 as *mut *mut i8;
            let ref mut fresh9 = *(*(*root).pi.offset(i as isize)).offset(1 as isize);
            *fresh9 = *fresh8 as *mut i8;
            let ref mut fresh10 = *(*(*root).pi.offset(i as isize)).offset(2 as isize);
            *fresh10 = strdup(b"\x00" as *const u8 as *const i8)
        }
        while !(*(*(*root).pi.offset(i as isize)).offset(j as isize)).is_null() {
            j += 1
        }
        let ref mut fresh11 = *(*root).pi.offset(i as isize);
        *fresh11 = realloc(
            *(*root).pi.offset(i as isize) as *mut libc::c_void,
            (::std::mem::size_of::<*mut i8>() as u64).wrapping_mul((j + 3i32) as u64),
        ) as *mut *mut i8;
        let ref mut fresh12 = *(*(*root).pi.offset(i as isize)).offset((j + 2i32) as isize);
        *fresh12 = realloc(
            *(*(*root).pi.offset(i as isize)).offset((j + 1i32) as isize) as *mut libc::c_void,
            (j + 1i32) as u64,
        ) as *mut i8;
        strcpy(
            (*(*(*root).pi.offset(i as isize)).offset((j + 2i32) as isize))
                .offset(j as isize)
                .offset(-(1 as isize)),
            if !(*root).xml.name.is_null() {
                b">\x00" as *const u8 as *const i8
            } else {
                b"<\x00" as *const u8 as *const i8
            },
        );
        let ref mut fresh13 = *(*(*root).pi.offset(i as isize)).offset((j + 1i32) as isize);
        *fresh13 = 0 as *mut i8;
        let ref mut fresh14 = *(*(*root).pi.offset(i as isize)).offset(j as isize);
        *fresh14 = s;
    }
}

#[no_mangle]
pub extern "C" fn ezxml_internal_dtd(mut root: ezxml_root_t, mut s: *mut i8, mut len: u64) -> i16 {
    let mut q: i8 = 0;
    let mut c: *mut i8 = 0 as *mut i8;
    let mut t: *mut i8 = 0 as *mut i8;
    let mut n: *mut i8 = 0 as *mut i8;
    let mut v: *mut i8 = 0 as *mut i8;
    unsafe {
        let mut ent: *mut *mut i8 = 0 as *mut *mut i8;
        let mut pe: *mut *mut i8 = 0 as *mut *mut i8;
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        pe = memcpy(
            malloc(::std::mem::size_of::<[*mut i8; 1]>() as u64),
            EZXML_NIL.as_mut_ptr() as *const libc::c_void,
            ::std::mem::size_of::<[*mut i8; 1]>() as u64,
        ) as *mut *mut i8;
        *s.offset(len as isize) = '\u{0}' as i8;
        while !s.is_null() {
            while *s as i32 != 0 && *s as i32 != '<' as i32 && *s as i32 != '%' as i32 {
                s = s.offset(1)
            }
            if *s == 0 {
                break;
            }
            if strncmp(s, b"<!ENTITY\x00" as *const u8 as *const i8, 8) == 0 {
                s = s.offset(
                    strspn(
                        s.offset(8 as isize),
                        b"\t\r\n \x00" as *const u8 as *const i8,
                    )
                    .wrapping_add(8) as isize,
                );
                c = s;
                n = s.offset(strspn(s, b"\t\r\n %\x00" as *const u8 as *const i8) as isize);
                s = n.offset(strcspn(n, b"\t\r\n \x00" as *const u8 as *const i8) as isize);
                *s = ';' as i8;
                v = s
                    .offset(strspn(
                        s.offset(1 as isize),
                        b"\t\r\n \x00" as *const u8 as *const i8,
                    ) as isize)
                    .offset(1 as isize);
                let fresh15 = v;
                v = v.offset(1);
                q = *fresh15;
                if q as i32 != '\"' as i32 && q as i32 != '\'' as i32 {
                    s = strchr(s, '>' as i32)
                } else {
                    i = 0;
                    ent = (if *c as i32 == '%' as i32 {
                        pe
                    } else {
                        (*root).ent
                    });
                    while !(*ent.offset(i as isize)).is_null() {
                        i += 1
                    }
                    ent = realloc(
                        ent as *mut libc::c_void,
                        ((i + 3i32) as u64).wrapping_mul(::std::mem::size_of::<*mut i8>() as u64),
                    ) as *mut *mut i8;
                    if *c as i32 == '%' as i32 {
                        pe = ent
                    } else {
                        (*root).ent = ent
                    }
                    s = s.offset(1);
                    *s = '\u{0}' as i8;
                    s = strchr(v, q as i32);
                    if !s.is_null() {
                        let fresh16 = s;
                        s = s.offset(1);
                        *fresh16 = '\u{0}' as i8
                    }
                    let ref mut fresh17 = *ent.offset((i + 1i32) as isize);
                    *fresh17 = ezxml_decode(v, pe, '%' as i8);
                    let ref mut fresh18 = *ent.offset((i + 2i32) as isize);
                    *fresh18 = 0 as *mut i8;
                    if ezxml_ent_ok(n, *ent.offset((i + 1i32) as isize), ent) == 0 {
                        if *ent.offset((i + 1i32) as isize) != v {
                            free(*ent.offset((i + 1i32) as isize) as *mut libc::c_void);
                        }
                        ezxml_err(
                            root,
                            v,
                            b"circular entity declaration &%s\x00" as *const u8 as *const i8,
                            n,
                        );
                        break;
                    } else {
                        let ref mut fresh19 = *ent.offset(i as isize);
                        *fresh19 = n
                    }
                }
            } else if strncmp(s, b"<!ATTLIST\x00" as *const u8 as *const i8, 9) == 0 {
                t = s
                    .offset(strspn(
                        s.offset(9 as isize),
                        b"\t\r\n \x00" as *const u8 as *const i8,
                    ) as isize)
                    .offset(9 as isize);
                if *t == 0 {
                    ezxml_err(root, t, b"unclosed <!ATTLIST\x00" as *const u8 as *const i8);
                    break;
                } else {
                    s = t.offset(strcspn(t, b"\t\r\n >\x00" as *const u8 as *const i8) as isize);
                    if *s as i32 == '>' as i32 {
                        continue;
                    }
                    *s = '\u{0}' as i8;
                    i = 0;
                    while !(*(*root).attr.offset(i as isize)).is_null()
                        && strcmp(n, *(*(*root).attr.offset(i as isize)).offset(0 as isize)) != 0
                    {
                        i += 1
                    }
                    loop {
                        s = s.offset(1);
                        n = s.offset(strspn(s, b"\t\r\n \x00" as *const u8 as *const i8) as isize);
                        if !(*n as i32 != 0 && *n as i32 != '>' as i32) {
                            break;
                        }
                        s = n.offset(strcspn(n, b"\t\r\n \x00" as *const u8 as *const i8) as isize);
                        if *s != 0 {
                            *s = '\u{0}' as i8;
                            s = s.offset(
                                strspn(
                                    s.offset(1 as isize),
                                    b"\t\r\n \x00" as *const u8 as *const i8,
                                )
                                .wrapping_add(1) as isize,
                            );
                            c = if strncmp(s, b"CDATA\x00" as *const u8 as *const i8, 5) != 0 {
                                b"*\x00" as *const u8 as *const i8
                            } else {
                                b" \x00" as *const u8 as *const i8
                            } as *mut i8;
                            if strncmp(s, b"NOTATION\x00" as *const u8 as *const i8, 8) == 0 {
                                s = s.offset(
                                    strspn(
                                        s.offset(8 as isize),
                                        b"\t\r\n \x00" as *const u8 as *const i8,
                                    )
                                    .wrapping_add(8) as isize,
                                )
                            }
                            s = if *s as i32 == '(' as i32 {
                                strchr(s, ')' as i32)
                            } else {
                                s.offset(
                                    strcspn(s, b"\t\r\n \x00" as *const u8 as *const i8) as isize
                                )
                            };
                            if s.is_null() {
                                ezxml_err(
                                    root,
                                    t,
                                    b"malformed <!ATTLIST\x00" as *const u8 as *const i8,
                                );
                                break;
                            } else {
                                s = s
                                    .offset(strspn(s, b"\t\r\n )\x00" as *const u8 as *const i8)
                                        as isize);
                                if strncmp(s, b"#FIXED\x00" as *const u8 as *const i8, 6) == 0 {
                                    s = s.offset(
                                        strspn(
                                            s.offset(6 as isize),
                                            b"\t\r\n \x00" as *const u8 as *const i8,
                                        )
                                        .wrapping_add(6)
                                            as isize,
                                    )
                                }
                                if *s as i32 == '#' as i32 {
                                    s = s.offset(
                                        strcspn(s, b"\t\r\n >\x00" as *const u8 as *const i8)
                                            .wrapping_sub(1)
                                            as isize,
                                    );
                                    if *c as i32 == ' ' as i32 {
                                        continue;
                                    }
                                    v = 0 as *mut i8
                                } else if (*s as i32 == '\"' as i32 || *s as i32 == '\'' as i32)
                                    && {
                                        v = s.offset(1 as isize);
                                        s = strchr(v, *s as i32);
                                        !s.is_null()
                                    }
                                {
                                    *s = '\u{0}' as i8
                                } else {
                                    ezxml_err(
                                        root,
                                        t,
                                        b"malformed <!ATTLIST\x00" as *const u8 as *const i8,
                                    );
                                    break;
                                }
                                if (*(*root).attr.offset(i as isize)).is_null() {
                                    (*root).attr = if i == 0 {
                                        malloc(2u64.wrapping_mul(
                                            ::std::mem::size_of::<*mut *mut i8>() as u64,
                                        ))
                                    } else {
                                        realloc(
                                            (*root).attr as *mut libc::c_void,
                                            ((i + 2i32) as u64).wrapping_mul(
                                                ::std::mem::size_of::<*mut *mut i8>() as u64,
                                            ),
                                        )
                                    }
                                        as *mut *mut *mut i8;
                                    let ref mut fresh20 = *(*root).attr.offset(i as isize);
                                    *fresh20 = malloc(
                                        2u64.wrapping_mul(::std::mem::size_of::<*mut i8>() as u64),
                                    )
                                        as *mut *mut i8;
                                    let ref mut fresh21 =
                                        *(*(*root).attr.offset(i as isize)).offset(0 as isize);
                                    *fresh21 = t;
                                    let ref mut fresh22 = *(*root).attr.offset((i + 1i32) as isize);
                                    *fresh22 = 0 as *mut *mut i8;
                                    let ref mut fresh23 =
                                        *(*(*root).attr.offset(i as isize)).offset(1 as isize);
                                    *fresh23 = *fresh22 as *mut i8
                                }
                                j = 1;
                                while !(*(*(*root).attr.offset(i as isize)).offset(j as isize))
                                    .is_null()
                                {
                                    j += 3
                                }
                                let ref mut fresh24 = *(*root).attr.offset(i as isize);
                                *fresh24 = realloc(
                                    *(*root).attr.offset(i as isize) as *mut libc::c_void,
                                    ((j + 4i32) as u64)
                                        .wrapping_mul(::std::mem::size_of::<*mut i8>() as u64),
                                ) as *mut *mut i8;
                                let ref mut fresh25 =
                                    *(*(*root).attr.offset(i as isize)).offset((j + 3i32) as isize);
                                *fresh25 = 0 as *mut i8;
                                let ref mut fresh26 =
                                    *(*(*root).attr.offset(i as isize)).offset((j + 2i32) as isize);
                                *fresh26 = c;
                                let ref mut fresh27 =
                                    *(*(*root).attr.offset(i as isize)).offset((j + 1i32) as isize);
                                *fresh27 = if !v.is_null() {
                                    ezxml_decode(v, (*root).ent, *c)
                                } else {
                                    0 as *mut i8
                                };
                                let ref mut fresh28 =
                                    *(*(*root).attr.offset(i as isize)).offset(j as isize);
                                *fresh28 = n
                            }
                        } else {
                            ezxml_err(
                                root,
                                t,
                                b"malformed <!ATTLIST\x00" as *const u8 as *const i8,
                            );
                            break;
                        }
                    }
                }
            } else if strncmp(s, b"<!--\x00" as *const u8 as *const i8, 4) == 0 {
                s = strstr(s.offset(4 as isize), b"-->\x00" as *const u8 as *const i8)
            } else if strncmp(s, b"<?\x00" as *const u8 as *const i8, 2) == 0 {
                c = s.offset(2 as isize);
                s = strstr(c, b"?>\x00" as *const u8 as *const i8);
                if !s.is_null() {
                    let fresh29 = s;
                    s = s.offset(1);
                    ezxml_proc_inst(root, c, fresh29.wrapping_offset_from(c) as u64);
                }
            } else if *s as i32 == '<' as i32 {
                s = strchr(s, '>' as i32)
            } else {
                let fresh30 = s;
                s = s.offset(1);
                if *fresh30 as i32 == '%' as i32 && (*root).standalone == 0 {
                    break;
                }
            }
        }
        free(pe as *mut libc::c_void);
        return (*(*root).err.as_mut_ptr() == 0) as i16;
    }
}

#[no_mangle]
pub extern "C" fn ezxml_str2utf8(mut s: *mut *mut i8, mut len: *mut u64) -> *mut i8 {
    let mut u: *mut i8 = 0 as *mut i8;
    let mut l: u64 = 0;
    let mut sl: u64 = 0;
    unsafe {
        let mut max: u64 = *len;
        let mut c: i64 = 0;
        let mut d: i64 = 0;
        let mut b: i32 = 0;
        let mut be: i32 = if **s as i32 == -2i32 {
            1
        } else if **s as i32 == -1i32 {
            0
        } else {
            -1
        };
        if be == -1 {
            return 0 as *mut i8;
        }
        u = malloc(max) as *mut i8;
        sl = 2;
        while sl < (*len).wrapping_sub(1) {
            c = if be != 0 {
                ((*(*s).offset(sl as isize) as i32 & 0xffi32) << 8)
                    | *(*s).offset(sl.wrapping_add(1) as isize) as i32 & 0xff
            } else {
                ((*(*s).offset(sl.wrapping_add(1) as isize) as i32 & 0xff) << 8)
                    | *(*s).offset(sl as isize) as i32 & 0xff
            } as i64;
            if c >= 0xd800 && c <= 0xdfff && {
                sl = (sl).wrapping_add(2) as u64;
                (sl) < (*len).wrapping_sub(1)
            } {
                d = if be != 0 {
                    ((*(*s).offset(sl as isize) as i32 & 0xffi32) << 8)
                        | *(*s).offset(sl.wrapping_add(1) as isize) as i32 & 0xff
                } else {
                    ((*(*s).offset(sl.wrapping_add(1) as isize) as i32 & 0xff) << 8)
                        | *(*s).offset(sl as isize) as i32 & 0xff
                } as i64;
                c = ((c & 0x3ff) << 10 | d & 0x3ff) + 0x10000
            }
            while l.wrapping_add(6) > max {
                max = (max).wrapping_add(1024) as u64;
                u = realloc(u as *mut libc::c_void, max) as *mut i8
            }
            if c < 0x80 {
                let fresh31 = l;
                l = l.wrapping_add(1);
                *u.offset(fresh31 as isize) = c as i8
            } else {
                b = 0;
                d = c;
                while d != 0 {
                    b += 1;
                    d /= 2
                }
                b = (b - 2) / 5;
                let fresh32 = l;
                l = l.wrapping_add(1);
                *u.offset(fresh32 as isize) = ((0xffi32 << 7 - b) as i64 | c >> 6 * b) as i8;
                while b != 0 {
                    b -= 1;
                    let fresh33 = l;
                    l = l.wrapping_add(1);
                    *u.offset(fresh33 as isize) = (0x80 | c >> 6 * b & 0x3fi64) as i8
                }
            }
            sl = (sl).wrapping_add(2) as u64
        }
        *len = l;
        *s = realloc(u as *mut libc::c_void, *len) as *mut i8;
        return *s;
    }
}

#[no_mangle]
pub extern "C" fn ezxml_free_attr(mut attr: *mut *mut i8) {
    let mut i: i32 = 0;
    let mut m: *mut i8 = 0 as *mut i8;
    unsafe {
        if attr.is_null() || attr == EZXML_NIL.as_mut_ptr() {
            return;
        }
        while !(*attr.offset(i as isize)).is_null() {
            i += 2
        }
        m = *attr.offset((i + 1i32) as isize);
    }
    i = 0;
    unsafe {
        while *m.offset(i as isize) != 0 {
            if *m.offset(i as isize) as i32 & 0x80 != 0 {
                free(*attr.offset((i * 2i32) as isize) as *mut libc::c_void);
            }
            if *m.offset(i as isize) as i32 & 0x40 != 0 {
                free(*attr.offset((i * 2 + 1i32) as isize) as *mut libc::c_void);
            }
            i += 1
        }
        free(m as *mut libc::c_void);
        free(attr as *mut libc::c_void);
    }
}

#[no_mangle]
pub extern "C" fn ezxml_parse_str(mut s: *mut i8, mut len: u64) -> ezxml_t {
    let mut root: ezxml_root_t = ezxml_new(0 as *const i8) as ezxml_root_t;
    let mut q: i8 = 0;
    let mut e: i8 = 0;
    let mut d: *mut i8 = 0 as *mut i8;
    let mut attr: *mut *mut i8 = 0 as *mut *mut i8;
    let mut a: *mut *mut i8 = 0 as *mut *mut i8;
    let mut l: i32 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    unsafe {
        (*root).m = s;
        if len == 0 {
            return ezxml_err(
                root,
                0 as *mut i8,
                b"root tag missing\x00" as *const u8 as *const i8,
            );
        };
        (*root).u = ezxml_str2utf8(&mut s, &mut len);
        (*root).s = s;
        (*root).e = (*root).s.offset(len as isize);
        e = *s.offset(len.wrapping_sub(1) as isize);
        *s.offset(len.wrapping_sub(1) as isize) = '\u{0}' as i8;
        while *s as i32 != 0 && *s as i32 != '<' as i32 {
            s = s.offset(1)
        }
        if *s == 0 {
            return ezxml_err(root, s, b"root tag missing\x00" as *const u8 as *const i8);
        }
        loop {
            attr = EZXML_NIL.as_mut_ptr();
            s = s.offset(1);
            d = s;
            if *(*__ctype_b_loc()).offset(*s as i32 as isize) as i32 & _ISalpha as i32 != 0
                || *s as i32 == '_' as i32
                || *s as i32 == ':' as i32
                || (*s as i32) < '\u{0}' as i32
            {
                if (*root).cur.is_null() {
                    return ezxml_err(
                        root,
                        d,
                        b"markup outside of root element\x00" as *const u8 as *const i8,
                    );
                }
                s = s.offset(strcspn(s, b"\t\r\n />\x00" as *const u8 as *const i8) as isize);
                while *(*__ctype_b_loc()).offset(*s as i32 as isize) as i32 & _ISspace as i32 != 0 {
                    let fresh34 = s;
                    s = s.offset(1);
                    *fresh34 = '\u{0}' as i8
                }
                if *s as i32 != 0 && *s as i32 != '/' as i32 && *s as i32 != '>' as i32 {
                    i = 0;
                    loop {
                        a = *(*root).attr.offset(i as isize);
                        if !(!a.is_null() && strcmp(*a.offset(0 as isize), d) != 0) {
                            break;
                        }
                        i += 1
                    }
                }
                l = 0;
                while *s as i32 != 0 && *s as i32 != '/' as i32 && *s as i32 != '>' as i32 {
                    attr = if l != 0 {
                        realloc(
                            attr as *mut libc::c_void,
                            ((l + 4i32) as u64)
                                .wrapping_mul(::std::mem::size_of::<*mut i8>() as u64),
                        )
                    } else {
                        malloc(4u64.wrapping_mul(::std::mem::size_of::<*mut i8>() as u64))
                    } as *mut *mut i8;
                    let ref mut fresh35 = *attr.offset((l + 3i32) as isize);
                    *fresh35 = if l != 0 {
                        realloc(
                            *attr.offset((l + 1i32) as isize) as *mut libc::c_void,
                            (l / 2 + 2i32) as u64,
                        )
                    } else {
                        malloc(2)
                    } as *mut i8;
                    strcpy(
                        (*attr.offset((l + 3i32) as isize)).offset((l / 2i32) as isize),
                        b" \x00" as *const u8 as *const i8,
                    );
                    let ref mut fresh36 = *attr.offset((l + 2i32) as isize);
                    *fresh36 = 0 as *mut i8;
                    let ref mut fresh37 = *attr.offset((l + 1i32) as isize);
                    *fresh37 = b"\x00" as *const u8 as *const i8 as *mut i8;
                    let ref mut fresh38 = *attr.offset(l as isize);
                    *fresh38 = s;
                    s = s.offset(strcspn(s, b"\t\r\n =/>\x00" as *const u8 as *const i8) as isize);
                    if *s as i32 == '=' as i32
                        || *(*__ctype_b_loc()).offset(*s as i32 as isize) as i32 & _ISspace as i32
                            != 0
                    {
                        let fresh39 = s;
                        s = s.offset(1);
                        *fresh39 = '\u{0}' as i8;
                        s = s.offset(strspn(s, b"\t\r\n =\x00" as *const u8 as *const i8) as isize);
                        q = *s;
                        if q as i32 == '\"' as i32 || q as i32 == '\'' as i32 {
                            s = s.offset(1);
                            let ref mut fresh40 = *attr.offset((l + 1i32) as isize);
                            *fresh40 = s;
                            while *s as i32 != 0 && *s as i32 != q as i32 {
                                s = s.offset(1)
                            }
                            if *s != 0 {
                                let fresh41 = s;
                                s = s.offset(1);
                                *fresh41 = '\u{0}' as i8
                            } else {
                                ezxml_free_attr(attr);
                                return ezxml_err(
                                    root,
                                    d,
                                    b"missing %c\x00" as *const u8 as *const i8,
                                    q as i32,
                                );
                            }
                            j = 1;
                            while !a.is_null()
                                && !(*a.offset(j as isize)).is_null()
                                && strcmp(*a.offset(j as isize), *attr.offset(l as isize)) != 0
                            {
                                j += 3
                            }
                            let ref mut fresh42 = *attr.offset((l + 1i32) as isize);
                            *fresh42 = ezxml_decode(
                                *attr.offset((l + 1i32) as isize),
                                (*root).ent,
                                if !a.is_null() && !(*a.offset(j as isize)).is_null() {
                                    **a.offset((j + 2i32) as isize) as i32
                                } else {
                                    ' ' as i32
                                } as i8,
                            );
                            if *attr.offset((l + 1i32) as isize) < d
                                || *attr.offset((l + 1i32) as isize) > s
                            {
                                *(*attr.offset((l + 3i32) as isize)).offset((l / 2i32) as isize) =
                                    0x40
                            }
                        }
                    }
                    while *(*__ctype_b_loc()).offset(*s as i32 as isize) as i32 & _ISspace as i32
                        != 0
                    {
                        s = s.offset(1)
                    }
                    l += 2
                }
                if *s as i32 == '/' as i32 {
                    let fresh43 = s;
                    s = s.offset(1);
                    *fresh43 = '\u{0}' as i8;
                    if *s as i32 != 0 && *s as i32 != '>' as i32
                        || *s == 0 && e as i32 != '>' as i32
                    {
                        if l != 0 {
                            ezxml_free_attr(attr);
                        }
                        return ezxml_err(root, d, b"missing >\x00" as *const u8 as *const i8);
                    }
                    ezxml_open_tag(root, d, attr);
                    ezxml_close_tag(root, d, s);
                } else {
                    q = *s;
                    if q as i32 == '>' as i32 || *s == 0 && e as i32 == '>' as i32 {
                        *s = '\u{0}' as i8;
                        ezxml_open_tag(root, d, attr);
                        *s = q
                    } else {
                        if l != 0 {
                            ezxml_free_attr(attr);
                        }
                        return ezxml_err(root, d, b"missing >\x00" as *const u8 as *const i8);
                    }
                }
            } else if *s as i32 == '/' as i32 {
                d = s.offset(1 as isize);
                s = s.offset(
                    strcspn(d, b"\t\r\n >\x00" as *const u8 as *const i8).wrapping_add(1) as isize,
                );
                q = *s;
                if q == 0 && e as i32 != '>' as i32 {
                    return ezxml_err(root, d, b"missing >\x00" as *const u8 as *const i8);
                }
                *s = '\u{0}' as i8;
                if !ezxml_close_tag(root, d, s).is_null() {
                    return &mut (*root).xml;
                }
                *s = q;
                if *(*__ctype_b_loc()).offset(*s as i32 as isize) as i32 & _ISspace as i32 != 0 {
                    s = s.offset(strspn(s, b"\t\r\n \x00" as *const u8 as *const i8) as isize)
                }
            } else if strncmp(s, b"!--\x00" as *const u8 as *const i8, 3) == 0 {
                s = strstr(s.offset(3 as isize), b"--\x00" as *const u8 as *const i8);
                if s.is_null()
                    || {
                        s = s.offset(2 as isize);
                        (*s as i32 != '>' as i32) && *s as i32 != 0
                    }
                    || *s == 0 && e as i32 != '>' as i32
                {
                    return ezxml_err(root, d, b"unclosed <!--\x00" as *const u8 as *const i8);
                }
            } else if strncmp(s, b"![CDATA[\x00" as *const u8 as *const i8, 8) == 0 {
                s = strstr(s, b"]]>\x00" as *const u8 as *const i8);
                if !s.is_null() {
                    s = s.offset(2 as isize);
                    ezxml_char_content(
                        root,
                        d.offset(8 as isize),
                        (s.wrapping_offset_from(d) as i64 - 10i64) as u64,
                        'c' as i8,
                    );
                } else {
                    return ezxml_err(root, d, b"unclosed <![CDATA[\x00" as *const u8 as *const i8);
                }
            } else if strncmp(s, b"!DOCTYPE\x00" as *const u8 as *const i8, 8) == 0 {
                l = 0;
                while *s as i32 != 0
                    && (l == 0 && *s as i32 != '>' as i32
                        || l != 0
                            && (*s as i32 != ']' as i32
                                || *s
                                    .offset(strspn(
                                        s.offset(1 as isize),
                                        b"\t\r\n \x00" as *const u8 as *const i8,
                                    ) as isize)
                                    .offset(1 as isize) as i32
                                    != '>' as i32))
                {
                    s = s.offset(
                        strcspn(s.offset(1 as isize), b"[]>\x00" as *const u8 as *const i8)
                            .wrapping_add(1) as isize,
                    );
                    l = if *s as i32 == '[' as i32 { 1 } else { l }
                }
                if *s == 0 && e as i32 != '>' as i32 {
                    return ezxml_err(root, d, b"unclosed <!DOCTYPE\x00" as *const u8 as *const i8);
                }
                d = if l != 0 {
                    strchr(d, '[' as i32).offset(1 as isize)
                } else {
                    d
                };
                if l != 0 && {
                    let fresh44 = s;
                    s = s.offset(1);
                    (ezxml_internal_dtd(root, d, fresh44.wrapping_offset_from(d) as u64)) == 0
                } {
                    return &mut (*root).xml;
                }
            } else if *s as i32 == '?' as i32 {
                loop {
                    s = strchr(s, '?' as i32);
                    if !(!s.is_null()
                        && {
                            s = s.offset(1);
                            (*s as i32) != 0
                        }
                        && *s as i32 != '>' as i32)
                    {
                        break;
                    }
                }
                if s.is_null() || *s == 0 && e as i32 != '>' as i32 {
                    return ezxml_err(root, d, b"unclosed <?\x00" as *const u8 as *const i8);
                } else {
                    ezxml_proc_inst(
                        root,
                        d.offset(1 as isize),
                        (s.wrapping_offset_from(d) as i64 - 2i64) as u64,
                    );
                }
            } else {
                return ezxml_err(root, d, b"unexpected <\x00" as *const u8 as *const i8);
            }
            if s.is_null() || *s == 0 {
                break;
            }
            *s = '\u{0}' as i8;
            s = s.offset(1);
            d = s;
            if *s as i32 != 0 && *s as i32 != '<' as i32 {
                while *s as i32 != 0 && *s as i32 != '<' as i32 {
                    s = s.offset(1)
                }
                if !(*s != 0) {
                    break;
                }
                ezxml_char_content(root, d, s.wrapping_offset_from(d) as u64, '&' as i8);
            } else if *s == 0 {
                break;
            }
        }
        if (*root).cur.is_null() {
            return &mut (*root).xml;
        } else if (*(*root).cur).name.is_null() {
            return ezxml_err(root, d, b"root tag missing\x00" as *const u8 as *const i8);
        } else {
            return ezxml_err(
                root,
                d,
                b"unclosed tag <%s>\x00" as *const u8 as *const i8,
                (*(*root).cur).name,
            );
        };
    }
}

#[no_mangle]
pub extern "C" fn ezxml_parse_fp(mut fp: *mut FILE) -> ezxml_t {
    let mut root: ezxml_root_t = 0 as *mut ezxml_root;
    let mut l: u64 = 0;
    let mut len: u64 = 0;
    let mut s: *mut i8 = 0 as *mut i8;
    unsafe {
        s = malloc(1024) as *mut i8;
    }
    if s.is_null() {
        return 0 as ezxml_t;
    }
    unsafe {
        loop {
            l = fread(s.offset(len as isize) as *mut libc::c_void, 1, 1024, fp);
            len = (len).wrapping_add(l) as u64;
            if l == 1024 {
                s = realloc(s as *mut libc::c_void, len.wrapping_add(1024)) as *mut i8
            }
            if !(!s.is_null() && l == 1024) {
                break;
            }
        }
    }
    if s.is_null() {
        return 0 as ezxml_t;
    }
    root = ezxml_parse_str(s, len) as ezxml_root_t;
    unsafe {
        (*root).len = (-1) as i64 as u64;
        return &mut (*root).xml;
    }
}

#[no_mangle]
pub extern "C" fn ezxml_parse_fd(mut fd: i32) -> ezxml_t {
    let mut root: ezxml_root_t = 0 as *mut ezxml_root;
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
    let mut l: u64 = 0;
    let mut m: *mut libc::c_void = 0 as *mut libc::c_void;
    if fd < 0 {
        return 0 as ezxml_t;
    }
    fstat(fd, &mut st);
    unsafe {
        l = (st.st_size + sysconf(_SC_PAGESIZE as i32) - 1 & !(sysconf(_SC_PAGESIZE as i32) - 1i64))
            as u64;
        m = mmap(0 as *mut libc::c_void, l, 0x1 | 0x2, 0x2, fd, 0);
        if m != -1i32 as *mut libc::c_void {
            madvise(m, l, 2);
            root = ezxml_parse_str(m as *mut i8, st.st_size as u64) as ezxml_root_t;
            (*root).len = l;
            madvise(m, (*root).len, 0);
        } else {
            m = malloc(st.st_size as u64);
            l = read(fd, m, st.st_size as u64) as u64;
            root = ezxml_parse_str(m as *mut i8, l) as ezxml_root_t;
            (*root).len = (-1) as i64 as u64
        }
        return &mut (*root).xml;
    }
}

#[no_mangle]
pub extern "C" fn ezxml_parse_file(mut file: *const i8) -> ezxml_t {
    unsafe {
        let mut fd: i32 = open(file, 0, 0);
        let mut xml: ezxml_t = ezxml_parse_fd(fd);
        if fd >= 0 {
            close(fd);
        }
        return xml;
    }
}

#[no_mangle]
pub extern "C" fn ezxml_ampencode(
    mut s: *const i8,
    mut len: u64,
    mut dst: *mut *mut i8,
    mut dlen: *mut u64,
    mut max: *mut u64,
    mut a: i16,
) -> *mut i8 {
    let mut e: *const i8 = 0 as *const i8;
    unsafe {
        e = s.offset(len as isize);
        while s != e {
            while (*dlen).wrapping_add(10) > *max {
                *max = (*max as u64).wrapping_add(1024) as u64;
                *dst = realloc(*dst as *mut libc::c_void, *max) as *mut i8
            }
            match *s as i32 {
                0 => return *dst,
                38 => {
                    *dlen = (*dlen as u64).wrapping_add(sprintf(
                        (*dst).offset(*dlen as isize),
                        b"&amp;\x00" as *const u8 as *const i8,
                    ) as u64) as u64
                }
                60 => {
                    *dlen = (*dlen as u64).wrapping_add(sprintf(
                        (*dst).offset(*dlen as isize),
                        b"&lt;\x00" as *const u8 as *const i8,
                    ) as u64) as u64
                }
                62 => {
                    *dlen = (*dlen as u64).wrapping_add(sprintf(
                        (*dst).offset(*dlen as isize),
                        b"&gt;\x00" as *const u8 as *const i8,
                    ) as u64) as u64
                }
                34 => {
                    *dlen = (*dlen as u64).wrapping_add(sprintf(
                        (*dst).offset(*dlen as isize),
                        if a as i32 != 0 {
                            b"&quot;\x00" as *const u8 as *const i8
                        } else {
                            b"\"\x00" as *const u8 as *const i8
                        },
                    ) as u64) as u64
                }
                10 => {
                    *dlen = (*dlen as u64).wrapping_add(sprintf(
                        (*dst).offset(*dlen as isize),
                        if a as i32 != 0 {
                            b"&#xA;\x00" as *const u8 as *const i8
                        } else {
                            b"\n\x00" as *const u8 as *const i8
                        },
                    ) as u64) as u64
                }
                9 => {
                    *dlen = (*dlen as u64).wrapping_add(sprintf(
                        (*dst).offset(*dlen as isize),
                        if a as i32 != 0 {
                            b"&#x9;\x00" as *const u8 as *const i8
                        } else {
                            b"\t\x00" as *const u8 as *const i8
                        },
                    ) as u64) as u64
                }
                13 => {
                    *dlen = (*dlen as u64).wrapping_add(sprintf(
                        (*dst).offset(*dlen as isize),
                        b"&#xD;\x00" as *const u8 as *const i8,
                    ) as u64) as u64
                }
                _ => {
                    let fresh45 = *dlen;
                    *dlen = (*dlen).wrapping_add(1);
                    *(*dst).offset(fresh45 as isize) = *s
                }
            }
            s = s.offset(1)
        }
        return *dst;
    }
}

#[no_mangle]
pub extern "C" fn ezxml_toxml_r(
    mut xml: ezxml_t,
    mut s: *mut *mut i8,
    mut len: *mut u64,
    mut max: *mut u64,
    mut start: u64,
    mut attr: *mut *mut *mut i8,
) -> *mut i8 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    unsafe {
        let mut txt: *mut i8 = if !(*xml).parent.is_null() {
            (*(*xml).parent).txt
        } else {
            b"\x00" as *const u8 as *const i8
        } as *mut i8;
        let mut off: u64 = 0;
        *s = ezxml_ampencode(
            txt.offset(start as isize),
            (*xml).off.wrapping_sub(start),
            s,
            len,
            max,
            0,
        );
        while (*len).wrapping_add(strlen((*xml).name)).wrapping_add(4) > *max {
            *max = (*max as u64).wrapping_add(1024) as u64;
            *s = realloc(*s as *mut libc::c_void, *max) as *mut i8
        }
        *len = (*len as u64).wrapping_add(sprintf(
            (*s).offset(*len as isize),
            b"<%s\x00" as *const u8 as *const i8,
            (*xml).name,
        ) as u64) as u64;
        i = 0;
        while !(*(*xml).attr.offset(i as isize)).is_null() {
            if !(ezxml_attr(xml, *(*xml).attr.offset(i as isize))
                != *(*xml).attr.offset((i + 1i32) as isize))
            {
                while (*len)
                    .wrapping_add(strlen(*(*xml).attr.offset(i as isize)))
                    .wrapping_add(7)
                    > *max
                {
                    *max = (*max as u64).wrapping_add(1024) as u64;
                    *s = realloc(*s as *mut libc::c_void, *max) as *mut i8
                }
                *len = (*len as u64).wrapping_add(sprintf(
                    (*s).offset(*len as isize),
                    b" %s=\"\x00" as *const u8 as *const i8,
                    *(*xml).attr.offset(i as isize),
                ) as u64) as u64;
                ezxml_ampencode(
                    *(*xml).attr.offset((i + 1i32) as isize),
                    (-1) as i64 as u64,
                    s,
                    len,
                    max,
                    1,
                );
                *len = (*len as u64).wrapping_add(sprintf(
                    (*s).offset(*len as isize),
                    b"\"\x00" as *const u8 as *const i8,
                ) as u64) as u64
            }
            i += 2
        }
        i = 0;
        while !(*attr.offset(i as isize)).is_null()
            && strcmp(*(*attr.offset(i as isize)).offset(0 as isize), (*xml).name) != 0
        {
            i += 1
        }
        j = 1;
        while !(*attr.offset(i as isize)).is_null()
            && !(*(*attr.offset(i as isize)).offset(j as isize)).is_null()
        {
            if !((*(*attr.offset(i as isize)).offset((j + 1i32) as isize)).is_null()
                || ezxml_attr(xml, *(*attr.offset(i as isize)).offset(j as isize))
                    != *(*attr.offset(i as isize)).offset((j + 1i32) as isize))
            {
                while (*len)
                    .wrapping_add(strlen(*(*attr.offset(i as isize)).offset(j as isize)))
                    .wrapping_add(7)
                    > *max
                {
                    *max = (*max as u64).wrapping_add(1024) as u64;
                    *s = realloc(*s as *mut libc::c_void, *max) as *mut i8
                }
                *len = (*len as u64).wrapping_add(sprintf(
                    (*s).offset(*len as isize),
                    b" %s=\"\x00" as *const u8 as *const i8,
                    *(*attr.offset(i as isize)).offset(j as isize),
                ) as u64) as u64;
                ezxml_ampencode(
                    *(*attr.offset(i as isize)).offset((j + 1i32) as isize),
                    (-1) as i64 as u64,
                    s,
                    len,
                    max,
                    1,
                );
                *len = (*len as u64).wrapping_add(sprintf(
                    (*s).offset(*len as isize),
                    b"\"\x00" as *const u8 as *const i8,
                ) as u64) as u64
            }
            j += 3
        }
        *len = (*len as u64).wrapping_add(sprintf(
            (*s).offset(*len as isize),
            b">\x00" as *const u8 as *const i8,
        ) as u64) as u64;
        *s = if !(*xml).child.is_null() {
            ezxml_toxml_r((*xml).child, s, len, max, 0, attr)
        } else {
            ezxml_ampencode((*xml).txt, (-1) as i64 as u64, s, len, max, 0)
        };
        while (*len).wrapping_add(strlen((*xml).name)).wrapping_add(4) > *max {
            *max = (*max as u64).wrapping_add(1024) as u64;
            *s = realloc(*s as *mut libc::c_void, *max) as *mut i8
        }
        *len = (*len as u64).wrapping_add(sprintf(
            (*s).offset(*len as isize),
            b"</%s>\x00" as *const u8 as *const i8,
            (*xml).name,
        ) as u64) as u64;
        while *txt.offset(off as isize) as i32 != 0 && off < (*xml).off {
            off = off.wrapping_add(1)
        }
        return if !(*xml).ordered.is_null() {
            ezxml_toxml_r((*xml).ordered, s, len, max, off, attr)
        } else {
            ezxml_ampencode(txt.offset(off as isize), (-1) as i64 as u64, s, len, max, 0)
        };
    }
}

#[no_mangle]
pub extern "C" fn ezxml_toxml(mut xml: ezxml_t) -> *mut i8 {
    unsafe {
        let mut p: ezxml_t = if !xml.is_null() {
            (*xml).parent
        } else {
            0 as ezxml_t
        };
        let mut o: ezxml_t = if !xml.is_null() {
            (*xml).ordered
        } else {
            0 as ezxml_t
        };
        let mut root: ezxml_root_t = xml as ezxml_root_t;
        let mut len: u64 = 0;
        let mut max: u64 = 1024;
        let mut s: *mut i8 = strcpy(malloc(max) as *mut i8, b"\x00" as *const u8 as *const i8);
        let mut t: *mut i8 = 0 as *mut i8;
        let mut n: *mut i8 = 0 as *mut i8;
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut k: i32 = 0;
        if xml.is_null() || (*xml).name.is_null() {
            return realloc(s as *mut libc::c_void, len.wrapping_add(1)) as *mut i8;
        }
        while !(*root).xml.parent.is_null() {
            root = (*root).xml.parent as ezxml_root_t
        }
        i = 0;
        while p.is_null() && !(*(*root).pi.offset(i as isize)).is_null() {
            k = 2;
            while !(*(*(*root).pi.offset(i as isize)).offset((k - 1i32) as isize)).is_null() {
                k += 1
            }
            j = 1;
            loop {
                n = *(*(*root).pi.offset(i as isize)).offset(j as isize);
                if n.is_null() {
                    break;
                }
                if !(*(*(*(*root).pi.offset(i as isize)).offset(k as isize))
                    .offset((j - 1i32) as isize) as i32
                    == '>' as i32)
                {
                    loop {
                        t = *(*(*root).pi.offset(i as isize)).offset(0 as isize);
                        if !(len
                            .wrapping_add(strlen(t))
                            .wrapping_add(strlen(n))
                            .wrapping_add(7)
                            > max)
                        {
                            break;
                        }
                        max = (max).wrapping_add(1024) as u64;
                        s = realloc(s as *mut libc::c_void, max) as *mut i8
                    }
                    len = (len).wrapping_add(sprintf(
                        s.offset(len as isize),
                        b"<?%s%s%s?>\n\x00" as *const u8 as *const i8,
                        t,
                        if *n as i32 != 0 {
                            b" \x00" as *const u8 as *const i8
                        } else {
                            b"\x00" as *const u8 as *const i8
                        },
                        n,
                    ) as u64) as u64
                }
                j += 1
            }
            i += 1
        }
        (*xml).ordered = 0 as ezxml_t;
        (*xml).parent = (*xml).ordered;
        s = ezxml_toxml_r(xml, &mut s, &mut len, &mut max, 0, (*root).attr);
        (*xml).parent = p;
        (*xml).ordered = o;
        i = 0;
        while p.is_null() && !(*(*root).pi.offset(i as isize)).is_null() {
            k = 2;
            while !(*(*(*root).pi.offset(i as isize)).offset((k - 1i32) as isize)).is_null() {
                k += 1
            }
            j = 1;
            loop {
                n = *(*(*root).pi.offset(i as isize)).offset(j as isize);
                if n.is_null() {
                    break;
                }
                if !(*(*(*(*root).pi.offset(i as isize)).offset(k as isize))
                    .offset((j - 1i32) as isize) as i32
                    == '<' as i32)
                {
                    loop {
                        t = *(*(*root).pi.offset(i as isize)).offset(0 as isize);
                        if !(len
                            .wrapping_add(strlen(t))
                            .wrapping_add(strlen(n))
                            .wrapping_add(7)
                            > max)
                        {
                            break;
                        }
                        max = (max).wrapping_add(1024) as u64;
                        s = realloc(s as *mut libc::c_void, max) as *mut i8
                    }
                    len = (len).wrapping_add(sprintf(
                        s.offset(len as isize),
                        b"\n<?%s%s%s?>\x00" as *const u8 as *const i8,
                        t,
                        if *n as i32 != 0 {
                            b" \x00" as *const u8 as *const i8
                        } else {
                            b"\x00" as *const u8 as *const i8
                        },
                        n,
                    ) as u64) as u64
                }
                j += 1
            }
            i += 1
        }
        return realloc(s as *mut libc::c_void, len.wrapping_add(1)) as *mut i8;
    }
}

#[no_mangle]
pub extern "C" fn ezxml_free(mut xml: ezxml_t) {
    let mut root: ezxml_root_t = xml as ezxml_root_t;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut a: *mut *mut i8 = 0 as *mut *mut i8;
    let mut s: *mut i8 = 0 as *mut i8;
    if xml.is_null() {
        return;
    }
    unsafe {
        ezxml_free((*xml).child);
        ezxml_free((*xml).ordered);
        if (*xml).parent.is_null() {
            i = 10;
            while !(*(*root).ent.offset(i as isize)).is_null() {
                s = *(*root).ent.offset((i + 1i32) as isize);
                if s < (*root).s || s > (*root).e {
                    free(s as *mut libc::c_void);
                }
                i += 2
            }
            free((*root).ent as *mut libc::c_void);
            i = 0;
            loop {
                a = *(*root).attr.offset(i as isize);
                if a.is_null() {
                    break;
                }
                j = 1;
                loop {
                    let fresh46 = j;
                    j = j + 1;
                    if (*a.offset(fresh46 as isize)).is_null() {
                        break;
                    }
                    if !(*a.offset(j as isize)).is_null()
                        && (*a.offset(j as isize) < (*root).s || *a.offset(j as isize) > (*root).e)
                    {
                        free(*a.offset(j as isize) as *mut libc::c_void);
                    }
                    j += 2
                }
                free(a as *mut libc::c_void);
                i += 1
            }
            if !(*(*root).attr.offset(0 as isize)).is_null() {
                free((*root).attr as *mut libc::c_void);
            }
            i = 0;
            while !(*(*root).pi.offset(i as isize)).is_null() {
                j = 1;
                while !(*(*(*root).pi.offset(i as isize)).offset(j as isize)).is_null() {
                    j += 1
                }
                free(
                    *(*(*root).pi.offset(i as isize)).offset((j + 1i32) as isize)
                        as *mut libc::c_void,
                );
                free(*(*root).pi.offset(i as isize) as *mut libc::c_void);
                i += 1
            }
            if !(*(*root).pi.offset(0 as isize)).is_null() {
                free((*root).pi as *mut libc::c_void);
            }
            if (*root).len == (-1) as i64 as u64 {
                free((*root).m as *mut libc::c_void);
            } else if (*root).len != 0 {
                munmap((*root).m as *mut libc::c_void, (*root).len);
            }
            if !(*root).u.is_null() {
                free((*root).u as *mut libc::c_void);
            }
        }
        ezxml_free_attr((*xml).attr);
        if (*xml).flags as i32 & 0x40 != 0 {
            free((*xml).txt as *mut libc::c_void);
        }
        if (*xml).flags as i32 & 0x80 != 0 {
            free((*xml).name as *mut libc::c_void);
        }
        free(xml as *mut libc::c_void);
    }
}

#[no_mangle]
pub extern "C" fn ezxml_error(mut xml: ezxml_t) -> *const i8 {
    unsafe {
        while !xml.is_null() && !(*xml).parent.is_null() {
            xml = (*xml).parent
        }
        return if !xml.is_null() {
            (*(xml as ezxml_root_t)).err.as_mut_ptr()
        } else {
            b"\x00" as *const u8 as *const i8
        };
    }
}

#[no_mangle]
pub extern "C" fn ezxml_new(mut name: *const i8) -> ezxml_t {
    unsafe {
        static mut ent: [*mut i8; 11] = [
            b"lt;\x00" as *const u8 as *const i8 as *mut i8,
            b"&#60;\x00" as *const u8 as *const i8 as *mut i8,
            b"gt;\x00" as *const u8 as *const i8 as *mut i8,
            b"&#62;\x00" as *const u8 as *const i8 as *mut i8,
            b"quot;\x00" as *const u8 as *const i8 as *mut i8,
            b"&#34;\x00" as *const u8 as *const i8 as *mut i8,
            b"apos;\x00" as *const u8 as *const i8 as *mut i8,
            b"&#39;\x00" as *const u8 as *const i8 as *mut i8,
            b"amp;\x00" as *const u8 as *const i8 as *mut i8,
            b"&#38;\x00" as *const u8 as *const i8 as *mut i8,
            0 as *const i8 as *mut i8,
        ];
        let mut root: ezxml_root_t = memset(
            malloc(::std::mem::size_of::<ezxml_root>() as u64),
            '\u{0}' as i32,
            ::std::mem::size_of::<ezxml_root>() as u64,
        ) as ezxml_root_t;
        (*root).xml.name = name as *mut i8;
        (*root).cur = &mut (*root).xml;
        (*root).xml.txt = b"\x00" as *const u8 as *const i8 as *mut i8;
        strcpy((*root).err.as_mut_ptr(), (*root).xml.txt);
        (*root).ent = memcpy(
            malloc(::std::mem::size_of::<[*mut i8; 11]>() as u64),
            ent.as_mut_ptr() as *const libc::c_void,
            ::std::mem::size_of::<[*mut i8; 11]>() as u64,
        ) as *mut *mut i8;
        (*root).xml.attr = EZXML_NIL.as_mut_ptr();
        (*root).pi = (*root).xml.attr as *mut *mut *mut i8;
        (*root).attr = (*root).pi;
        return &mut (*root).xml;
    }
}

#[no_mangle]
pub extern "C" fn ezxml_insert(mut xml: ezxml_t, mut dest: ezxml_t, mut off: u64) -> ezxml_t {
    let mut cur: ezxml_t = 0 as *mut ezxml;
    let mut prev: ezxml_t = 0 as *mut ezxml;
    let mut head: ezxml_t = 0 as *mut ezxml;
    unsafe {
        (*xml).ordered = 0 as ezxml_t;
        (*xml).sibling = (*xml).ordered;
        (*xml).next = (*xml).sibling;
        (*xml).off = off;
        (*xml).parent = dest;
        head = (*dest).child;
        if !head.is_null() {
            if (*head).off <= off {
                cur = head;
                while !(*cur).ordered.is_null() && (*(*cur).ordered).off <= off {
                    cur = (*cur).ordered
                }
                (*xml).ordered = (*cur).ordered;
                (*cur).ordered = xml
            } else {
                (*xml).ordered = head;
                (*dest).child = xml
            }
            cur = head;
            prev = 0 as ezxml_t;
            while !cur.is_null() && strcmp((*cur).name, (*xml).name) != 0 {
                prev = cur;
                cur = (*cur).sibling
            }
            if !cur.is_null() && (*cur).off <= off {
                while !(*cur).next.is_null() && (*(*cur).next).off <= off {
                    cur = (*cur).next
                }
                (*xml).next = (*cur).next;
                (*cur).next = xml
            } else {
                if !prev.is_null() && !cur.is_null() {
                    (*prev).sibling = (*cur).sibling
                };
                (*xml).next = cur;
                cur = head;
                prev = 0 as ezxml_t;
                while !cur.is_null() && (*cur).off <= off {
                    prev = cur;
                    cur = (*cur).sibling
                }
                (*xml).sibling = cur;
                if !prev.is_null() {
                    (*prev).sibling = xml
                }
            }
        } else {
            (*dest).child = xml
        }
    }
    return xml;
}

#[no_mangle]
pub extern "C" fn ezxml_add_child(mut xml: ezxml_t, mut name: *const i8, mut off: u64) -> ezxml_t {
    let mut child: ezxml_t = 0 as *mut ezxml;
    if xml.is_null() {
        return 0 as ezxml_t;
    }
    unsafe {
        child = memset(
            malloc(::std::mem::size_of::<ezxml>() as u64),
            '\u{0}' as i32,
            ::std::mem::size_of::<ezxml>() as u64,
        ) as ezxml_t;
        (*child).name = name as *mut i8;
        (*child).attr = EZXML_NIL.as_mut_ptr();
        (*child).txt = b"\x00" as *const u8 as *const i8 as *mut i8;
    }
    return ezxml_insert(child, xml, off);
}

#[no_mangle]
pub extern "C" fn ezxml_set_txt(mut xml: ezxml_t, mut txt: *const i8) -> ezxml_t {
    if xml.is_null() {
        return 0 as ezxml_t;
    }
    unsafe {
        if (*xml).flags as i32 & 0x40 != 0 {
            free((*xml).txt as *mut libc::c_void);
        };
        (*xml).flags = ((*xml).flags as i32 & !0x40i32) as i16;
        (*xml).txt = txt as *mut i8;
    }
    return xml;
}

#[no_mangle]
pub extern "C" fn ezxml_set_attr(
    mut xml: ezxml_t,
    mut name: *const i8,
    mut value: *const i8,
) -> ezxml_t {
    let mut l: i32 = 0;
    let mut c: i32 = 0;
    if xml.is_null() {
        return 0 as ezxml_t;
    }
    unsafe {
        while !(*(*xml).attr.offset(l as isize)).is_null()
            && strcmp(*(*xml).attr.offset(l as isize), name) != 0
        {
            l += 2
        }
        if (*(*xml).attr.offset(l as isize)).is_null() {
            if value.is_null() {
                return xml;
            }
            if (*xml).attr == EZXML_NIL.as_mut_ptr() {
                (*xml).attr = malloc(4u64.wrapping_mul(::std::mem::size_of::<*mut i8>() as u64))
                    as *mut *mut i8;
                let ref mut fresh47 = *(*xml).attr.offset(1 as isize);
                *fresh47 = strdup(b"\x00" as *const u8 as *const i8)
            } else {
                (*xml).attr = realloc(
                    (*xml).attr as *mut libc::c_void,
                    ((l + 4i32) as u64).wrapping_mul(::std::mem::size_of::<*mut i8>() as u64),
                ) as *mut *mut i8
            }
            let ref mut fresh48 = *(*xml).attr.offset(l as isize);
            *fresh48 = name as *mut i8;
            let ref mut fresh49 = *(*xml).attr.offset((l + 2i32) as isize);
            *fresh49 = 0 as *mut i8;
            c = strlen(*(*xml).attr.offset((l + 1i32) as isize)) as i32;
            let ref mut fresh50 = *(*xml).attr.offset((l + 3i32) as isize);
            *fresh50 = realloc(
                *(*xml).attr.offset((l + 1i32) as isize) as *mut libc::c_void,
                (c + 2i32) as u64,
            ) as *mut i8;
            strcpy(
                (*(*xml).attr.offset((l + 3i32) as isize)).offset(c as isize),
                b" \x00" as *const u8 as *const i8,
            );
            if (*xml).flags as i32 & 0x20 != 0 {
                *(*(*xml).attr.offset((l + 3i32) as isize)).offset(c as isize) = 0x80
            }
        } else if (*xml).flags as i32 & 0x20 != 0 {
            free(name as *mut i8 as *mut libc::c_void);
        }
    }
    c = l;
    unsafe {
        while !(*(*xml).attr.offset(c as isize)).is_null() {
            c += 2
        }
        if *(*(*xml).attr.offset((c + 1i32) as isize)).offset((l / 2i32) as isize) as i32 & 0x40
            != 0
        {
            free(*(*xml).attr.offset((l + 1i32) as isize) as *mut libc::c_void);
        }
        if (*xml).flags as i32 & 0x20 != 0 {
            let ref mut fresh51 =
                *(*(*xml).attr.offset((c + 1i32) as isize)).offset((l / 2i32) as isize);
            *fresh51 = (*fresh51 as i32 | 0x40i32) as i8
        } else {
            let ref mut fresh52 =
                *(*(*xml).attr.offset((c + 1i32) as isize)).offset((l / 2i32) as isize);
            *fresh52 = (*fresh52 as i32 & !0x40i32) as i8
        }
        if !value.is_null() {
            let ref mut fresh53 = *(*xml).attr.offset((l + 1i32) as isize);
            *fresh53 = value as *mut i8
        } else {
            if *(*(*xml).attr.offset((c + 1i32) as isize)).offset((l / 2i32) as isize) as i32 & 0x80
                != 0
            {
                free(*(*xml).attr.offset(l as isize) as *mut libc::c_void);
            }
            memmove(
                (*xml).attr.offset(l as isize) as *mut libc::c_void,
                (*xml).attr.offset(l as isize).offset(2 as isize) as *const libc::c_void,
                ((c - l + 2i32) as u64).wrapping_mul(::std::mem::size_of::<*mut i8>() as u64),
            );
            (*xml).attr = realloc(
                (*xml).attr as *mut libc::c_void,
                ((c + 2i32) as u64).wrapping_mul(::std::mem::size_of::<*mut i8>() as u64),
            ) as *mut *mut i8;
            memmove(
                (*(*xml).attr.offset((c + 1i32) as isize)).offset((l / 2i32) as isize)
                    as *mut libc::c_void,
                (*(*xml).attr.offset((c + 1i32) as isize))
                    .offset((l / 2i32) as isize)
                    .offset(1 as isize) as *const libc::c_void,
                (c / 2 - l / 2i32) as u64,
            );
        };
        (*xml).flags = ((*xml).flags as i32 & !0x20i32) as i16;
    }
    return xml;
}

#[no_mangle]
pub extern "C" fn ezxml_set_flag(mut xml: ezxml_t, mut flag: i16) -> ezxml_t {
    unsafe {
        if !xml.is_null() {
            (*xml).flags = ((*xml).flags as i32 | flag as i32) as i16
        }
    }
    return xml;
}

#[no_mangle]
pub extern "C" fn ezxml_cut(mut xml: ezxml_t) -> ezxml_t {
    let mut cur: ezxml_t = 0 as *mut ezxml;
    if xml.is_null() {
        return 0 as ezxml_t;
    }
    unsafe {
        if !(*xml).next.is_null() {
            (*(*xml).next).sibling = (*xml).sibling
        }
        if !(*xml).parent.is_null() {
            cur = (*(*xml).parent).child;
            if cur == xml {
                (*(*xml).parent).child = (*xml).ordered
            } else {
                while (*cur).ordered != xml {
                    cur = (*cur).ordered
                }
                (*cur).ordered = (*(*cur).ordered).ordered;
                cur = (*(*xml).parent).child;
                if strcmp((*cur).name, (*xml).name) != 0 {
                    while strcmp((*(*cur).sibling).name, (*xml).name) != 0 {
                        cur = (*cur).sibling
                    }
                    if (*cur).sibling == xml {
                        (*cur).sibling = if !(*xml).next.is_null() {
                            (*xml).next
                        } else {
                            (*(*cur).sibling).sibling
                        }
                    } else {
                        cur = (*cur).sibling
                    }
                }
                while !(*cur).next.is_null() && (*cur).next != xml {
                    cur = (*cur).next
                }
                if !(*cur).next.is_null() {
                    (*cur).next = (*(*cur).next).next
                }
            }
        };
        (*xml).next = 0 as ezxml_t;
        (*xml).sibling = (*xml).next;
        (*xml).ordered = (*xml).sibling;
    }
    return xml;
}
