use ::libc;

extern "C" {
    #[no_mangle]
    fn strtol(_: *const libc::c_char, _: *mut *mut libc::c_char,
              _: libc::c_int) -> libc::c_long;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn vsnprintf(_: *mut libc::c_char, _: libc::c_ulong,
                 _: *const libc::c_char, _: ::std::ffi::VaList)
     -> libc::c_int;
    #[no_mangle]
    fn fread(_: *mut libc::c_void, _: libc::c_ulong, _: libc::c_ulong,
             _: *mut FILE) -> libc::c_ulong;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strncpy(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
               _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char)
     -> libc::c_ulong;
    #[no_mangle]
    fn strspn(_: *const libc::c_char, _: *const libc::c_char)
     -> libc::c_ulong;
    #[no_mangle]
    fn strstr(_: *const libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t)
     -> ssize_t;
    #[no_mangle]
    fn sysconf(__name: libc::c_int) -> libc::c_long;
    #[no_mangle]
    fn mmap(__addr: *mut libc::c_void, __len: size_t, __prot: libc::c_int,
            __flags: libc::c_int, __fd: libc::c_int, __offset: __off_t)
     -> *mut libc::c_void;
    #[no_mangle]
    fn munmap(__addr: *mut libc::c_void, __len: size_t) -> libc::c_int;
    #[no_mangle]
    fn madvise(__addr: *mut libc::c_void, __len: size_t,
               __advice: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn __fxstat(__ver: libc::c_int, __fildes: libc::c_int,
                __stat_buf: *mut stat) -> libc::c_int;
    #[no_mangle]
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...)
     -> libc::c_int;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: Option<crustr_ptr::NonNullRawMut<libc::c_void>,
    pub reg_save_area: Option<crustr_ptr::NonNullRawMut<libc::c_void>,
}
pub type size_t = libc::c_ulong;
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
pub type ssize_t = __ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: Option<crustr_ptr::NonNullRawMut<libc::c_char>,
    pub _IO_read_end: Option<crustr_ptr::NonNullRawMut<libc::c_char>,
    pub _IO_read_base: Option<crustr_ptr::NonNullRawMut<libc::c_char>,
    pub _IO_write_base: Option<crustr_ptr::NonNullRawMut<libc::c_char>,
    pub _IO_write_ptr: Option<crustr_ptr::NonNullRawMut<libc::c_char>,
    pub _IO_write_end: Option<crustr_ptr::NonNullRawMut<libc::c_char>,
    pub _IO_buf_base: Option<crustr_ptr::NonNullRawMut<libc::c_char>,
    pub _IO_buf_end: Option<crustr_ptr::NonNullRawMut<libc::c_char>,
    pub _IO_save_base: Option<crustr_ptr::NonNullRawMut<libc::c_char>,
    pub _IO_backup_base: Option<crustr_ptr::NonNullRawMut<libc::c_char>,
    pub _IO_save_end: Option<crustr_ptr::NonNullRawMut<libc::c_char>,
    pub _markers: Option<crustr_ptr::NonNullRawMut<_IO_marker>,
    pub _chain: Option<crustr_ptr::NonNullRawMut<_IO_FILE>,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: Option<crustr_ptr::NonNullRawMut<libc::c_void>,
    pub _offset: __off64_t,
    pub __pad1: Option<crustr_ptr::NonNullRawMut<libc::c_void>,
    pub __pad2: Option<crustr_ptr::NonNullRawMut<libc::c_void>,
    pub __pad3: Option<crustr_ptr::NonNullRawMut<libc::c_void>,
    pub __pad4: Option<crustr_ptr::NonNullRawMut<libc::c_void>,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: Option<crustr_ptr::NonNullRawMut<_IO_marker>,
    pub _sbuf: Option<crustr_ptr::NonNullRawMut<_IO_FILE>,
    pub _pos: libc::c_int,
}
pub type FILE = _IO_FILE;
pub type va_list = __builtin_va_list;
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
pub type C2RustUnnamed_0 = libc::c_uint;
pub const _SC_THREAD_ROBUST_PRIO_PROTECT: C2RustUnnamed_0 = 248;
pub const _SC_THREAD_ROBUST_PRIO_INHERIT: C2RustUnnamed_0 = 247;
pub const _SC_XOPEN_STREAMS: C2RustUnnamed_0 = 246;
pub const _SC_TRACE_USER_EVENT_MAX: C2RustUnnamed_0 = 245;
pub const _SC_TRACE_SYS_MAX: C2RustUnnamed_0 = 244;
pub const _SC_TRACE_NAME_MAX: C2RustUnnamed_0 = 243;
pub const _SC_TRACE_EVENT_NAME_MAX: C2RustUnnamed_0 = 242;
pub const _SC_SS_REPL_MAX: C2RustUnnamed_0 = 241;
pub const _SC_V7_LPBIG_OFFBIG: C2RustUnnamed_0 = 240;
pub const _SC_V7_LP64_OFF64: C2RustUnnamed_0 = 239;
pub const _SC_V7_ILP32_OFFBIG: C2RustUnnamed_0 = 238;
pub const _SC_V7_ILP32_OFF32: C2RustUnnamed_0 = 237;
pub const _SC_RAW_SOCKETS: C2RustUnnamed_0 = 236;
pub const _SC_IPV6: C2RustUnnamed_0 = 235;
pub const _SC_LEVEL4_CACHE_LINESIZE: C2RustUnnamed_0 = 199;
pub const _SC_LEVEL4_CACHE_ASSOC: C2RustUnnamed_0 = 198;
pub const _SC_LEVEL4_CACHE_SIZE: C2RustUnnamed_0 = 197;
pub const _SC_LEVEL3_CACHE_LINESIZE: C2RustUnnamed_0 = 196;
pub const _SC_LEVEL3_CACHE_ASSOC: C2RustUnnamed_0 = 195;
pub const _SC_LEVEL3_CACHE_SIZE: C2RustUnnamed_0 = 194;
pub const _SC_LEVEL2_CACHE_LINESIZE: C2RustUnnamed_0 = 193;
pub const _SC_LEVEL2_CACHE_ASSOC: C2RustUnnamed_0 = 192;
pub const _SC_LEVEL2_CACHE_SIZE: C2RustUnnamed_0 = 191;
pub const _SC_LEVEL1_DCACHE_LINESIZE: C2RustUnnamed_0 = 190;
pub const _SC_LEVEL1_DCACHE_ASSOC: C2RustUnnamed_0 = 189;
pub const _SC_LEVEL1_DCACHE_SIZE: C2RustUnnamed_0 = 188;
pub const _SC_LEVEL1_ICACHE_LINESIZE: C2RustUnnamed_0 = 187;
pub const _SC_LEVEL1_ICACHE_ASSOC: C2RustUnnamed_0 = 186;
pub const _SC_LEVEL1_ICACHE_SIZE: C2RustUnnamed_0 = 185;
pub const _SC_TRACE_LOG: C2RustUnnamed_0 = 184;
pub const _SC_TRACE_INHERIT: C2RustUnnamed_0 = 183;
pub const _SC_TRACE_EVENT_FILTER: C2RustUnnamed_0 = 182;
pub const _SC_TRACE: C2RustUnnamed_0 = 181;
pub const _SC_HOST_NAME_MAX: C2RustUnnamed_0 = 180;
pub const _SC_V6_LPBIG_OFFBIG: C2RustUnnamed_0 = 179;
pub const _SC_V6_LP64_OFF64: C2RustUnnamed_0 = 178;
pub const _SC_V6_ILP32_OFFBIG: C2RustUnnamed_0 = 177;
pub const _SC_V6_ILP32_OFF32: C2RustUnnamed_0 = 176;
pub const _SC_2_PBS_CHECKPOINT: C2RustUnnamed_0 = 175;
pub const _SC_STREAMS: C2RustUnnamed_0 = 174;
pub const _SC_SYMLOOP_MAX: C2RustUnnamed_0 = 173;
pub const _SC_2_PBS_TRACK: C2RustUnnamed_0 = 172;
pub const _SC_2_PBS_MESSAGE: C2RustUnnamed_0 = 171;
pub const _SC_2_PBS_LOCATE: C2RustUnnamed_0 = 170;
pub const _SC_2_PBS_ACCOUNTING: C2RustUnnamed_0 = 169;
pub const _SC_2_PBS: C2RustUnnamed_0 = 168;
pub const _SC_USER_GROUPS_R: C2RustUnnamed_0 = 167;
pub const _SC_USER_GROUPS: C2RustUnnamed_0 = 166;
pub const _SC_TYPED_MEMORY_OBJECTS: C2RustUnnamed_0 = 165;
pub const _SC_TIMEOUTS: C2RustUnnamed_0 = 164;
pub const _SC_SYSTEM_DATABASE_R: C2RustUnnamed_0 = 163;
pub const _SC_SYSTEM_DATABASE: C2RustUnnamed_0 = 162;
pub const _SC_THREAD_SPORADIC_SERVER: C2RustUnnamed_0 = 161;
pub const _SC_SPORADIC_SERVER: C2RustUnnamed_0 = 160;
pub const _SC_SPAWN: C2RustUnnamed_0 = 159;
pub const _SC_SIGNALS: C2RustUnnamed_0 = 158;
pub const _SC_SHELL: C2RustUnnamed_0 = 157;
pub const _SC_REGEX_VERSION: C2RustUnnamed_0 = 156;
pub const _SC_REGEXP: C2RustUnnamed_0 = 155;
pub const _SC_SPIN_LOCKS: C2RustUnnamed_0 = 154;
pub const _SC_READER_WRITER_LOCKS: C2RustUnnamed_0 = 153;
pub const _SC_NETWORKING: C2RustUnnamed_0 = 152;
pub const _SC_SINGLE_PROCESS: C2RustUnnamed_0 = 151;
pub const _SC_MULTI_PROCESS: C2RustUnnamed_0 = 150;
pub const _SC_MONOTONIC_CLOCK: C2RustUnnamed_0 = 149;
pub const _SC_FILE_SYSTEM: C2RustUnnamed_0 = 148;
pub const _SC_FILE_LOCKING: C2RustUnnamed_0 = 147;
pub const _SC_FILE_ATTRIBUTES: C2RustUnnamed_0 = 146;
pub const _SC_PIPE: C2RustUnnamed_0 = 145;
pub const _SC_FIFO: C2RustUnnamed_0 = 144;
pub const _SC_FD_MGMT: C2RustUnnamed_0 = 143;
pub const _SC_DEVICE_SPECIFIC_R: C2RustUnnamed_0 = 142;
pub const _SC_DEVICE_SPECIFIC: C2RustUnnamed_0 = 141;
pub const _SC_DEVICE_IO: C2RustUnnamed_0 = 140;
pub const _SC_THREAD_CPUTIME: C2RustUnnamed_0 = 139;
pub const _SC_CPUTIME: C2RustUnnamed_0 = 138;
pub const _SC_CLOCK_SELECTION: C2RustUnnamed_0 = 137;
pub const _SC_C_LANG_SUPPORT_R: C2RustUnnamed_0 = 136;
pub const _SC_C_LANG_SUPPORT: C2RustUnnamed_0 = 135;
pub const _SC_BASE: C2RustUnnamed_0 = 134;
pub const _SC_BARRIERS: C2RustUnnamed_0 = 133;
pub const _SC_ADVISORY_INFO: C2RustUnnamed_0 = 132;
pub const _SC_XOPEN_REALTIME_THREADS: C2RustUnnamed_0 = 131;
pub const _SC_XOPEN_REALTIME: C2RustUnnamed_0 = 130;
pub const _SC_XOPEN_LEGACY: C2RustUnnamed_0 = 129;
pub const _SC_XBS5_LPBIG_OFFBIG: C2RustUnnamed_0 = 128;
pub const _SC_XBS5_LP64_OFF64: C2RustUnnamed_0 = 127;
pub const _SC_XBS5_ILP32_OFFBIG: C2RustUnnamed_0 = 126;
pub const _SC_XBS5_ILP32_OFF32: C2RustUnnamed_0 = 125;
pub const _SC_NL_TEXTMAX: C2RustUnnamed_0 = 124;
pub const _SC_NL_SETMAX: C2RustUnnamed_0 = 123;
pub const _SC_NL_NMAX: C2RustUnnamed_0 = 122;
pub const _SC_NL_MSGMAX: C2RustUnnamed_0 = 121;
pub const _SC_NL_LANGMAX: C2RustUnnamed_0 = 120;
pub const _SC_NL_ARGMAX: C2RustUnnamed_0 = 119;
pub const _SC_USHRT_MAX: C2RustUnnamed_0 = 118;
pub const _SC_ULONG_MAX: C2RustUnnamed_0 = 117;
pub const _SC_UINT_MAX: C2RustUnnamed_0 = 116;
pub const _SC_UCHAR_MAX: C2RustUnnamed_0 = 115;
pub const _SC_SHRT_MIN: C2RustUnnamed_0 = 114;
pub const _SC_SHRT_MAX: C2RustUnnamed_0 = 113;
pub const _SC_SCHAR_MIN: C2RustUnnamed_0 = 112;
pub const _SC_SCHAR_MAX: C2RustUnnamed_0 = 111;
pub const _SC_SSIZE_MAX: C2RustUnnamed_0 = 110;
pub const _SC_NZERO: C2RustUnnamed_0 = 109;
pub const _SC_MB_LEN_MAX: C2RustUnnamed_0 = 108;
pub const _SC_WORD_BIT: C2RustUnnamed_0 = 107;
pub const _SC_LONG_BIT: C2RustUnnamed_0 = 106;
pub const _SC_INT_MIN: C2RustUnnamed_0 = 105;
pub const _SC_INT_MAX: C2RustUnnamed_0 = 104;
pub const _SC_CHAR_MIN: C2RustUnnamed_0 = 103;
pub const _SC_CHAR_MAX: C2RustUnnamed_0 = 102;
pub const _SC_CHAR_BIT: C2RustUnnamed_0 = 101;
pub const _SC_XOPEN_XPG4: C2RustUnnamed_0 = 100;
pub const _SC_XOPEN_XPG3: C2RustUnnamed_0 = 99;
pub const _SC_XOPEN_XPG2: C2RustUnnamed_0 = 98;
pub const _SC_2_UPE: C2RustUnnamed_0 = 97;
pub const _SC_2_C_VERSION: C2RustUnnamed_0 = 96;
pub const _SC_2_CHAR_TERM: C2RustUnnamed_0 = 95;
pub const _SC_XOPEN_SHM: C2RustUnnamed_0 = 94;
pub const _SC_XOPEN_ENH_I18N: C2RustUnnamed_0 = 93;
pub const _SC_XOPEN_CRYPT: C2RustUnnamed_0 = 92;
pub const _SC_XOPEN_UNIX: C2RustUnnamed_0 = 91;
pub const _SC_XOPEN_XCU_VERSION: C2RustUnnamed_0 = 90;
pub const _SC_XOPEN_VERSION: C2RustUnnamed_0 = 89;
pub const _SC_PASS_MAX: C2RustUnnamed_0 = 88;
pub const _SC_ATEXIT_MAX: C2RustUnnamed_0 = 87;
pub const _SC_AVPHYS_PAGES: C2RustUnnamed_0 = 86;
pub const _SC_PHYS_PAGES: C2RustUnnamed_0 = 85;
pub const _SC_NPROCESSORS_ONLN: C2RustUnnamed_0 = 84;
pub const _SC_NPROCESSORS_CONF: C2RustUnnamed_0 = 83;
pub const _SC_THREAD_PROCESS_SHARED: C2RustUnnamed_0 = 82;
pub const _SC_THREAD_PRIO_PROTECT: C2RustUnnamed_0 = 81;
pub const _SC_THREAD_PRIO_INHERIT: C2RustUnnamed_0 = 80;
pub const _SC_THREAD_PRIORITY_SCHEDULING: C2RustUnnamed_0 = 79;
pub const _SC_THREAD_ATTR_STACKSIZE: C2RustUnnamed_0 = 78;
pub const _SC_THREAD_ATTR_STACKADDR: C2RustUnnamed_0 = 77;
pub const _SC_THREAD_THREADS_MAX: C2RustUnnamed_0 = 76;
pub const _SC_THREAD_STACK_MIN: C2RustUnnamed_0 = 75;
pub const _SC_THREAD_KEYS_MAX: C2RustUnnamed_0 = 74;
pub const _SC_THREAD_DESTRUCTOR_ITERATIONS: C2RustUnnamed_0 = 73;
pub const _SC_TTY_NAME_MAX: C2RustUnnamed_0 = 72;
pub const _SC_LOGIN_NAME_MAX: C2RustUnnamed_0 = 71;
pub const _SC_GETPW_R_SIZE_MAX: C2RustUnnamed_0 = 70;
pub const _SC_GETGR_R_SIZE_MAX: C2RustUnnamed_0 = 69;
pub const _SC_THREAD_SAFE_FUNCTIONS: C2RustUnnamed_0 = 68;
pub const _SC_THREADS: C2RustUnnamed_0 = 67;
pub const _SC_T_IOV_MAX: C2RustUnnamed_0 = 66;
pub const _SC_PII_OSI_M: C2RustUnnamed_0 = 65;
pub const _SC_PII_OSI_CLTS: C2RustUnnamed_0 = 64;
pub const _SC_PII_OSI_COTS: C2RustUnnamed_0 = 63;
pub const _SC_PII_INTERNET_DGRAM: C2RustUnnamed_0 = 62;
pub const _SC_PII_INTERNET_STREAM: C2RustUnnamed_0 = 61;
pub const _SC_IOV_MAX: C2RustUnnamed_0 = 60;
pub const _SC_UIO_MAXIOV: C2RustUnnamed_0 = 60;
pub const _SC_SELECT: C2RustUnnamed_0 = 59;
pub const _SC_POLL: C2RustUnnamed_0 = 58;
pub const _SC_PII_OSI: C2RustUnnamed_0 = 57;
pub const _SC_PII_INTERNET: C2RustUnnamed_0 = 56;
pub const _SC_PII_SOCKET: C2RustUnnamed_0 = 55;
pub const _SC_PII_XTI: C2RustUnnamed_0 = 54;
pub const _SC_PII: C2RustUnnamed_0 = 53;
pub const _SC_2_LOCALEDEF: C2RustUnnamed_0 = 52;
pub const _SC_2_SW_DEV: C2RustUnnamed_0 = 51;
pub const _SC_2_FORT_RUN: C2RustUnnamed_0 = 50;
pub const _SC_2_FORT_DEV: C2RustUnnamed_0 = 49;
pub const _SC_2_C_DEV: C2RustUnnamed_0 = 48;
pub const _SC_2_C_BIND: C2RustUnnamed_0 = 47;
pub const _SC_2_VERSION: C2RustUnnamed_0 = 46;
pub const _SC_CHARCLASS_NAME_MAX: C2RustUnnamed_0 = 45;
pub const _SC_RE_DUP_MAX: C2RustUnnamed_0 = 44;
pub const _SC_LINE_MAX: C2RustUnnamed_0 = 43;
pub const _SC_EXPR_NEST_MAX: C2RustUnnamed_0 = 42;
pub const _SC_EQUIV_CLASS_MAX: C2RustUnnamed_0 = 41;
pub const _SC_COLL_WEIGHTS_MAX: C2RustUnnamed_0 = 40;
pub const _SC_BC_STRING_MAX: C2RustUnnamed_0 = 39;
pub const _SC_BC_SCALE_MAX: C2RustUnnamed_0 = 38;
pub const _SC_BC_DIM_MAX: C2RustUnnamed_0 = 37;
pub const _SC_BC_BASE_MAX: C2RustUnnamed_0 = 36;
pub const _SC_TIMER_MAX: C2RustUnnamed_0 = 35;
pub const _SC_SIGQUEUE_MAX: C2RustUnnamed_0 = 34;
pub const _SC_SEM_VALUE_MAX: C2RustUnnamed_0 = 33;
pub const _SC_SEM_NSEMS_MAX: C2RustUnnamed_0 = 32;
pub const _SC_RTSIG_MAX: C2RustUnnamed_0 = 31;
pub const _SC_PAGESIZE: C2RustUnnamed_0 = 30;
pub const _SC_VERSION: C2RustUnnamed_0 = 29;
pub const _SC_MQ_PRIO_MAX: C2RustUnnamed_0 = 28;
pub const _SC_MQ_OPEN_MAX: C2RustUnnamed_0 = 27;
pub const _SC_DELAYTIMER_MAX: C2RustUnnamed_0 = 26;
pub const _SC_AIO_PRIO_DELTA_MAX: C2RustUnnamed_0 = 25;
pub const _SC_AIO_MAX: C2RustUnnamed_0 = 24;
pub const _SC_AIO_LISTIO_MAX: C2RustUnnamed_0 = 23;
pub const _SC_SHARED_MEMORY_OBJECTS: C2RustUnnamed_0 = 22;
pub const _SC_SEMAPHORES: C2RustUnnamed_0 = 21;
pub const _SC_MESSAGE_PASSING: C2RustUnnamed_0 = 20;
pub const _SC_MEMORY_PROTECTION: C2RustUnnamed_0 = 19;
pub const _SC_MEMLOCK_RANGE: C2RustUnnamed_0 = 18;
pub const _SC_MEMLOCK: C2RustUnnamed_0 = 17;
pub const _SC_MAPPED_FILES: C2RustUnnamed_0 = 16;
pub const _SC_FSYNC: C2RustUnnamed_0 = 15;
pub const _SC_SYNCHRONIZED_IO: C2RustUnnamed_0 = 14;
pub const _SC_PRIORITIZED_IO: C2RustUnnamed_0 = 13;
pub const _SC_ASYNCHRONOUS_IO: C2RustUnnamed_0 = 12;
pub const _SC_TIMERS: C2RustUnnamed_0 = 11;
pub const _SC_PRIORITY_SCHEDULING: C2RustUnnamed_0 = 10;
pub const _SC_REALTIME_SIGNALS: C2RustUnnamed_0 = 9;
pub const _SC_SAVED_IDS: C2RustUnnamed_0 = 8;
pub const _SC_JOB_CONTROL: C2RustUnnamed_0 = 7;
pub const _SC_TZNAME_MAX: C2RustUnnamed_0 = 6;
pub const _SC_STREAM_MAX: C2RustUnnamed_0 = 5;
pub const _SC_OPEN_MAX: C2RustUnnamed_0 = 4;
pub const _SC_NGROUPS_MAX: C2RustUnnamed_0 = 3;
pub const _SC_CLK_TCK: C2RustUnnamed_0 = 2;
pub const _SC_CHILD_MAX: C2RustUnnamed_0 = 1;
pub const _SC_ARG_MAX: C2RustUnnamed_0 = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ezxml {
    pub name: Option<crustr_ptr::NonNullRawMut<libc::c_char>,
    pub attr: Option<crustr_ptr::NonNullRawMut<Option<crustr_ptr::NonNullRawMut<libc::c_char>>,
    pub txt: Option<crustr_ptr::NonNullRawMut<libc::c_char>,
    pub off: size_t,
    pub next: ezxml_t,
    pub sibling: ezxml_t,
    pub ordered: ezxml_t,
    pub child: ezxml_t,
    pub parent: ezxml_t,
    pub flags: libc::c_short,
}
pub type ezxml_t = *mut ezxml;
// maximum error string length
pub type ezxml_root_t = *mut ezxml_root;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ezxml_root {
    pub xml: ezxml,
    pub cur: ezxml_t,
    pub m: Option<crustr_ptr::NonNullRawMut<libc::c_char>,
    pub len: size_t,
    pub u: Option<crustr_ptr::NonNullRawMut<libc::c_char>,
    pub s: Option<crustr_ptr::NonNullRawMut<libc::c_char>,
    pub e: Option<crustr_ptr::NonNullRawMut<libc::c_char>,
    pub ent: Option<crustr_ptr::NonNullRawMut<Option<crustr_ptr::NonNullRawMut<libc::c_char>>,
    pub attr: Option<crustr_ptr::NonNullRawMut<Option<crustr_ptr::NonNullRawMut<Option<crustr_ptr::NonNullRawMut<libc::c_char>>>,
    pub pi: Option<crustr_ptr::NonNullRawMut<Option<crustr_ptr::NonNullRawMut<Option<crustr_ptr::NonNullRawMut<libc::c_char>>>,
    pub standalone: libc::c_short,
    pub err: [libc::c_char; 128],
}
#[inline]
unsafe extern "C" fn fstat(mut __fd: libc::c_int, mut __statbuf: *mut stat)
 -> libc::c_int {
    return __fxstat(1 as libc::c_int, __fd, __statbuf);
}
// error string
#[no_mangle]
pub static mut EZXML_NIL: [*mut libc::c_char; 1] =
    [0 as *const libc::c_char as *mut libc::c_char];
// empty, null terminated array of strings
// returns the first child tag with the given name or NULL if not found
#[no_mangle]
pub unsafe extern "C" fn ezxml_child(mut xml: ezxml_t,
                                     mut name: *const libc::c_char)
 -> ezxml_t {
    xml = if !xml.is_null() { (*xml).child } else { 0 as ezxml_t };
    while !xml.is_null() && strcmp(name, (*xml).name) != 0 {
        xml = (*xml).sibling
    }
    return xml;
}
// returns the Nth tag with the same name in the same subsection or NULL if not
// found
#[no_mangle]
pub unsafe extern "C" fn ezxml_idx(mut xml: ezxml_t, mut idx: libc::c_int)
 -> ezxml_t {
    while !xml.is_null() && idx != 0 { xml = (*xml).next; idx -= 1 }
    return xml;
}
// returns the value of the requested tag attribute or NULL if not found
#[no_mangle]
pub unsafe extern "C" fn ezxml_attr(mut xml: ezxml_t,
                                    mut attr: *const libc::c_char)
 -> *const libc::c_char {
    let mut i: libc::c_int = 0 as libc::c_int; // found attribute
    let mut j: libc::c_int = 1 as libc::c_int; // root tag
    let mut root: ezxml_root_t =
        xml as ezxml_root_t; // no matching default attributes
    if xml.is_null() || (*xml).attr.is_null() {
        return 0 as *const libc::c_char
    }
    while !(*(*xml).attr.offset(i as isize)).is_null() &&
              strcmp(attr, *(*xml).attr.offset(i as isize)) != 0 {
        i += 2 as libc::c_int
    }
    if !(*(*xml).attr.offset(i as isize)).is_null() {
        return *(*xml).attr.offset((i + 1 as libc::c_int) as isize)
    }
    while !(*root).xml.parent.is_null() {
        root = (*root).xml.parent as ezxml_root_t
    }
    i = 0 as libc::c_int;
    while !(*(*root).attr.offset(i as isize)).is_null() &&
              strcmp((*xml).name,
                     *(*(*root).attr.offset(i as
                                                isize)).offset(0 as
                                                                   libc::c_int
                                                                   as isize))
                  != 0 {
        i += 1
    }
    if (*(*root).attr.offset(i as isize)).is_null() {
        return 0 as *const libc::c_char
    }
    while !(*(*(*root).attr.offset(i as isize)).offset(j as isize)).is_null()
              &&
              strcmp(attr,
                     *(*(*root).attr.offset(i as isize)).offset(j as isize))
                  != 0 {
        j += 3 as libc::c_int
    }
    return if !(*(*(*root).attr.offset(i as
                                           isize)).offset(j as
                                                              isize)).is_null()
              {
               *(*(*root).attr.offset(i as
                                          isize)).offset((j +
                                                              1 as
                                                                  libc::c_int)
                                                             as isize)
           } else { 0 as *mut libc::c_char };
    // found default
}
// same as ezxml_get but takes an already initialized va_list
#[no_mangle]
pub unsafe extern "C" fn ezxml_vget(mut xml: ezxml_t,
                                    mut ap: ::std::ffi::VaList) -> ezxml_t {
    let mut name: *mut libc::c_char = ap.arg::<*mut libc::c_char>();
    let mut idx: libc::c_int = -(1 as libc::c_int);
    if !name.is_null() && *name as libc::c_int != 0 {
        idx = ap.arg::<libc::c_int>();
        xml = ezxml_child(xml, name)
    }
    return if idx < 0 as libc::c_int {
               xml
           } else { ezxml_vget(ezxml_idx(xml, idx), ap.as_va_list()) };
}
// Traverses the xml tree to retrieve a specific subtag. Takes a variable
// length list of tag names and indexes. The argument list must be terminated
// by either an index of -1 or an empty string tag name. Example: 
// title = ezxml_get(library, "shelf", 0, "book", 2, "title", -1);
// This retrieves the title of the 3rd book on the 1st shelf of library.
// Returns NULL if not found.
#[no_mangle]
pub unsafe extern "C" fn ezxml_get(mut xml: ezxml_t, mut args: ...)
 -> ezxml_t {
    let mut ap: ::std::ffi::VaListImpl;
    let mut r: ezxml_t = 0 as *mut ezxml;
    ap = args.clone();
    r = ezxml_vget(xml, ap.as_va_list());
    return r;
}
// returns a null terminated array of processing instructions for the given
// target
#[no_mangle]
pub unsafe extern "C" fn ezxml_pi(mut xml: ezxml_t,
                                  mut target: *const libc::c_char)
 -> *mut *const libc::c_char {
    let mut root: ezxml_root_t = xml as ezxml_root_t; // root tag
    let mut i: libc::c_int = 0 as libc::c_int; // find target
    if root.is_null() {
        return EZXML_NIL.as_mut_ptr() as *mut *const libc::c_char
    }
    while !(*root).xml.parent.is_null() {
        root = (*root).xml.parent as ezxml_root_t
    }
    while !(*(*root).pi.offset(i as isize)).is_null() &&
              strcmp(target,
                     *(*(*root).pi.offset(i as
                                              isize)).offset(0 as libc::c_int
                                                                 as isize)) !=
                  0 {
        i += 1
    }
    return if !(*(*root).pi.offset(i as isize)).is_null() {
               (*(*root).pi.offset(i as
                                       isize)).offset(1 as libc::c_int as
                                                          isize)
           } else { EZXML_NIL.as_mut_ptr() } as *mut *const libc::c_char;
}
// set an error string and return root
#[no_mangle]
pub unsafe extern "C" fn ezxml_err(mut root: ezxml_root_t,
                                   mut s: *mut libc::c_char,
                                   mut err: *const libc::c_char,
                                   mut args: ...) -> ezxml_t {
    let mut ap: ::std::ffi::VaListImpl;
    let mut line: libc::c_int = 1 as libc::c_int;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fmt: [libc::c_char; 128] = [0; 128];
    t = (*root).s;
    while t < s {
        if *t as libc::c_int == '\n' as i32 { line += 1 }
        t = t.offset(1)
    }
    snprintf(fmt.as_mut_ptr(), 128 as libc::c_int as libc::c_ulong,
             b"[error near line %d]: %s\x00" as *const u8 as
                 *const libc::c_char, line, err);
    ap = args.clone();
    vsnprintf((*root).err.as_mut_ptr(), 128 as libc::c_int as libc::c_ulong,
              fmt.as_mut_ptr(), ap.as_va_list());
    return &mut (*root).xml;
}
// Recursively decodes entity and character references and normalizes new lines
// ent is a null terminated array of alternating entity names and values. set t
// to '&' for general entity decoding, '%' for parameter entity decoding, 'c'
// for cdata sections, ' ' for attribute normalization, or '*' for non-cdata
// attribute normalization. Returns s, or if the decoded string is longer than
// s, returns a malloced string that must be freed.
#[no_mangle]
pub unsafe extern "C" fn ezxml_decode(mut s: *mut libc::c_char,
                                      mut ent: *mut *mut libc::c_char,
                                      mut t: libc::c_char)
 -> *mut libc::c_char {
    let mut e: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut r: *mut libc::c_char = s;
    let mut m: *mut libc::c_char = s;
    let mut b: libc::c_long = 0;
    let mut c: libc::c_long = 0;
    let mut d: libc::c_long = 0;
    let mut l: libc::c_long = 0;
    while *s != 0 {
        // normalize line endings
        while *s as libc::c_int == '\r' as i32 {
            let fresh0 = s;
            s = s.offset(1);
            *fresh0 = '\n' as i32 as libc::c_char;
            if *s as libc::c_int == '\n' as i32 {
                memmove(s as *mut libc::c_void,
                        s.offset(1 as libc::c_int as isize) as
                            *const libc::c_void, strlen(s));
            }
        }
        s = s.offset(1)
    }
    s = r;
    loop  {
        while *s as libc::c_int != 0 && *s as libc::c_int != '&' as i32 &&
                  (*s as libc::c_int != '%' as i32 ||
                       t as libc::c_int != '%' as i32) &&
                  *(*__ctype_b_loc()).offset(*s as libc::c_int as isize) as
                      libc::c_int &
                      _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                      == 0 {
            s = s.offset(1)
        }
        if *s == 0 { break ; }
        if t as libc::c_int != 'c' as i32 &&
               strncmp(s, b"&#\x00" as *const u8 as *const libc::c_char,
                       2 as libc::c_int as libc::c_ulong) == 0 {
            // no decoding needed
            // character reference
            if *s.offset(2 as libc::c_int as isize) as libc::c_int ==
                   'x' as i32 { // base 10
                c =
                    strtol(s.offset(3 as libc::c_int as isize), &mut e,
                           16 as libc::c_int)
            } else {
                c =
                    strtol(s.offset(2 as libc::c_int as isize), &mut e,
                           10 as libc::c_int)
            } // base 16
            if c == 0 || *e as libc::c_int != ';' as i32 {
                s = s.offset(1)
            } else {
                if c < 0x80 as libc::c_int as libc::c_long
                   { // not a character ref
                    let fresh1 = s; // US-ASCII subset
                    s = s.offset(1);
                    *fresh1 = c as libc::c_char
                } else {
                    // multi-byte UTF-8 sequence
                    b =
                        0 as libc::c_int as
                            libc::c_long; // number of bits in c
                    d = c;
                    while d != 0 {
                        b += 1;
                        d /= 2 as libc::c_int as libc::c_long
                    }
                    // payload
                    b =
                        (b - 2 as libc::c_int as libc::c_long) /
                            5 as libc::c_int as
                                libc::c_long; // number of bytes in payload
                    let fresh2 = s; // head
                    s = s.offset(1);
                    *fresh2 =
                        (((0xff as libc::c_int) <<
                              7 as libc::c_int as libc::c_long - b) as
                             libc::c_long |
                             c >> 6 as libc::c_int as libc::c_long * b) as
                            libc::c_char;
                    while b != 0 {
                        b -= 1;
                        let fresh3 = s;
                        s = s.offset(1);
                        *fresh3 =
                            (0x80 as libc::c_int as libc::c_long |
                                 c >> 6 as libc::c_int as libc::c_long * b &
                                     0x3f as libc::c_int as libc::c_long) as
                                libc::c_char
                    }
                }
                memmove(s as *mut libc::c_void,
                        strchr(s,
                               ';' as i32).offset(1 as libc::c_int as isize)
                            as *const libc::c_void,
                        strlen(strchr(s, ';' as i32)));
            }
        } else if *s as libc::c_int == '&' as i32 &&
                      (t as libc::c_int == '&' as i32 ||
                           t as libc::c_int == ' ' as i32 ||
                           t as libc::c_int == '*' as i32) ||
                      *s as libc::c_int == '%' as i32 &&
                          t as libc::c_int == '%' as i32 {
            // entity reference
            b =
                0 as libc::c_int as
                    libc::c_long; // find entity in entity list
            while !(*ent.offset(b as isize)).is_null() &&
                      strncmp(s.offset(1 as libc::c_int as isize),
                              *ent.offset(b as isize),
                              strlen(*ent.offset(b as isize))) != 0 {
                b += 2 as libc::c_int as libc::c_long
            }
            let fresh4 = b;
            b = b + 1;
            if !(*ent.offset(fresh4 as isize)).is_null() {
                // not a known entity
                // found a match
                c =
                    strlen(*ent.offset(b as isize)) as
                        libc::c_long; // new length
                e = strchr(s, ';' as i32);
                if c - 1 as libc::c_int as libc::c_long >
                       e.offset_from(s) as libc::c_long {
                    d = s.offset_from(r) as libc::c_long;
                    l =
                        ((d + c) as libc::c_ulong).wrapping_add(strlen(e)) as
                            libc::c_long;
                    r =
                        if r == m {
                            strcpy(malloc(l as libc::c_ulong) as
                                       *mut libc::c_char, r) as
                                *mut libc::c_void
                        } else {
                            realloc(r as *mut libc::c_void,
                                    l as libc::c_ulong)
                        } as *mut libc::c_char;
                    s = r.offset(d as isize);
                    e = strchr(s, ';' as i32)
                    // fix up pointers
                }
                // copy in replacement text
                memmove(s.offset(c as isize) as *mut libc::c_void,
                        e.offset(1 as libc::c_int as isize) as
                            *const libc::c_void,
                        strlen(e)); // shift rest of string
                strncpy(s, *ent.offset(b as isize), c as libc::c_ulong);
            } else { s = s.offset(1) }
        } else if (t as libc::c_int == ' ' as i32 ||
                       t as libc::c_int == '*' as i32) &&
                      *(*__ctype_b_loc()).offset(*s as libc::c_int as isize)
                          as libc::c_int &
                          _ISspace as libc::c_int as libc::c_ushort as
                              libc::c_int != 0 {
            let fresh5 = s;
            s = s.offset(1);
            *fresh5 = ' ' as i32 as libc::c_char
        } else { s = s.offset(1) }
    }
    if t as libc::c_int == '*' as i32 {
        // normalize spaces for non-cdata attributes
        s = r;
        while *s != 0 {
            l =
                strspn(s, b" \x00" as *const u8 as *const libc::c_char) as
                    libc::c_long;
            if l != 0 {
                memmove(s as *mut libc::c_void,
                        s.offset(l as isize) as *const libc::c_void,
                        strlen(s.offset(l as
                                            isize)).wrapping_add(1 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_ulong));
            }
            while *s as libc::c_int != 0 && *s as libc::c_int != ' ' as i32 {
                s = s.offset(1)
            }
            s = s.offset(1)
        }
        s = s.offset(-1);
        if s >= r && *s as libc::c_int == ' ' as i32 {
            *s = '\u{0}' as i32 as libc::c_char
        }
        // trim any trailing space
    }
    return r;
}
// called when parser finds start of new tag
#[no_mangle]
pub unsafe extern "C" fn ezxml_open_tag(mut root: ezxml_root_t,
                                        mut name: *mut libc::c_char,
                                        mut attr: *mut *mut libc::c_char) {
    let mut xml: ezxml_t = (*root).cur; // first open tag
    if !(*xml).name.is_null() {
        xml = ezxml_add_child(xml, name, strlen((*xml).txt))
    } else { (*xml).name = name }
    (*xml).attr = attr;
    (*root).cur = xml;
    // update tag insertion point
}
// called when parser finds character content between open and closing tag
#[no_mangle]
pub unsafe extern "C" fn ezxml_char_content(mut root: ezxml_root_t,
                                            mut s: *mut libc::c_char,
                                            mut len: size_t,
                                            mut t: libc::c_char) {
    let mut xml: ezxml_t = (*root).cur; // sanity check
    let mut m: *mut libc::c_char =
        s; // null terminate text (calling functions anticipate this)
    let mut l: size_t = 0; // initial character content
    if xml.is_null() || (*xml).name.is_null() || len == 0 { return }
    *s.offset(len as isize) = '\u{0}' as i32 as libc::c_char;
    s = ezxml_decode(s, (*root).ent, t);
    len = strlen(s).wrapping_add(1 as libc::c_int as libc::c_ulong);
    if *(*xml).txt == 0 {
        (*xml).txt = s
    } else {
        // allocate our own memory and make a copy
        (*xml).txt =
            if (*xml).flags as libc::c_int & 0x40 as libc::c_int != 0 {
                l = strlen((*xml).txt);
                realloc((*xml).txt as *mut libc::c_void, l.wrapping_add(len))
            } else {
                l = strlen((*xml).txt);
                strcpy(malloc(l.wrapping_add(len)) as *mut libc::c_char,
                       (*xml).txt) as *mut libc::c_void
            } as *mut libc::c_char;
        // free s if it was malloced by ezxml_decode()
        strcpy((*xml).txt.offset(l as isize), s); // add new char content
        if s != m { free(s as *mut libc::c_void); }
    }
    if (*xml).txt != m {
        ezxml_set_flag(xml, 0x40 as libc::c_int as libc::c_short);
    };
}
// called when parser finds closing tag
#[no_mangle]
pub unsafe extern "C" fn ezxml_close_tag(mut root: ezxml_root_t,
                                         mut name: *mut libc::c_char,
                                         mut s: *mut libc::c_char)
 -> ezxml_t {
    if (*root).cur.is_null() || (*(*root).cur).name.is_null() ||
           strcmp(name, (*(*root).cur).name) != 0 {
        return ezxml_err(root, s,
                         b"unexpected closing tag </%s>\x00" as *const u8 as
                             *const libc::c_char, name)
    }
    (*root).cur = (*(*root).cur).parent;
    return 0 as ezxml_t;
}
// checks for circular entity references, returns non-zero if no circular
// references are found, zero otherwise
#[no_mangle]
pub unsafe extern "C" fn ezxml_ent_ok(mut name: *mut libc::c_char,
                                      mut s: *mut libc::c_char,
                                      mut ent: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut i: libc::c_int = 0; // find next entity reference
    loop  {
        while *s as libc::c_int != 0 && *s as libc::c_int != '&' as i32 {
            s = s.offset(1)
        } // circular ref.
        if *s == 0 { return 1 as libc::c_int }
        if strncmp(s.offset(1 as libc::c_int as isize), name, strlen(name)) ==
               0 {
            return 0 as libc::c_int
        }
        i = 0 as libc::c_int;
        while !(*ent.offset(i as isize)).is_null() &&
                  strncmp(*ent.offset(i as isize),
                          s.offset(1 as libc::c_int as isize),
                          strlen(*ent.offset(i as isize))) != 0 {
            i += 2 as libc::c_int
        }
        if !(*ent.offset(i as isize)).is_null() &&
               ezxml_ent_ok(name,
                            *ent.offset((i + 1 as libc::c_int) as isize), ent)
                   == 0 {
            return 0 as libc::c_int
        }
        s = s.offset(1)
    };
}
// called when the parser finds a processing instruction
#[no_mangle]
pub unsafe extern "C" fn ezxml_proc_inst(mut root: ezxml_root_t,
                                         mut s: *mut libc::c_char,
                                         mut len: size_t) {
    let mut i: libc::c_int = 0 as libc::c_int; // null terminate instruction
    let mut j: libc::c_int = 1 as libc::c_int; // null terminate target
    let mut target: *mut libc::c_char = s;
    *s.offset(len as isize) = '\u{0}' as i32 as libc::c_char;
    s =
        s.offset(strcspn(s,
                         b"\t\r\n \x00" as *const u8 as *const libc::c_char)
                     as isize);
    if *s != 0 {
        *s = '\u{0}' as i32 as libc::c_char;
        s =
            s.offset(strspn(s.offset(1 as libc::c_int as isize),
                            b"\t\r\n \x00" as *const u8 as
                                *const libc::c_char).wrapping_add(1 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_ulong)
                         as isize)
        // skip whitespace after target
    }
    if strcmp(target, b"xml\x00" as *const u8 as *const libc::c_char) == 0 {
        // <?xml ... ?>
        s =
            strstr(s,
                   b"standalone\x00" as *const u8 as
                       *const libc::c_char); //first pi
        if !s.is_null() &&
               strncmp(s.offset(strspn(s.offset(10 as libc::c_int as isize),
                                       b"\t\r\n =\'\"\x00" as *const u8 as
                                           *const libc::c_char) as
                                    isize).offset(10 as libc::c_int as isize),
                       b"yes\x00" as *const u8 as *const libc::c_char,
                       3 as libc::c_int as libc::c_ulong) == 0 {
            (*root).standalone = 1 as libc::c_int as libc::c_short
        } // find target
        return
    }
    if (*(*root).pi.offset(0 as libc::c_int as isize)).is_null() {
        (*root).pi =
            malloc(::std::mem::size_of::<*mut *mut libc::c_char>() as
                       libc::c_ulong) as *mut *mut *mut libc::c_char;
        *(*root).pi = 0 as *mut *mut libc::c_char
    }
    while !(*(*root).pi.offset(i as isize)).is_null() &&
              strcmp(target,
                     *(*(*root).pi.offset(i as
                                              isize)).offset(0 as libc::c_int
                                                                 as isize)) !=
                  0 {
        i += 1
    }
    if (*(*root).pi.offset(i as isize)).is_null() {
        // new target
        (*root).pi =
            realloc((*root).pi as *mut libc::c_void,
                    (::std::mem::size_of::<*mut *mut libc::c_char>() as
                         libc::c_ulong).wrapping_mul((i + 2 as libc::c_int) as
                                                         libc::c_ulong)) as
                *mut *mut *mut libc::c_char;
        let ref mut fresh6 = *(*root).pi.offset(i as isize);
        *fresh6 =
            malloc((::std::mem::size_of::<*mut libc::c_char>() as
                        libc::c_ulong).wrapping_mul(3 as libc::c_int as
                                                        libc::c_ulong)) as
                *mut *mut libc::c_char;
        let ref mut fresh7 =
            *(*(*root).pi.offset(i as
                                     isize)).offset(0 as libc::c_int as
                                                        isize);
        *fresh7 = target;
        // empty document position list
        let ref mut fresh8 =
            *(*root).pi.offset((i + 1 as libc::c_int) as
                                   isize); // terminate pi list
        *fresh8 =
            0 as
                *mut *mut libc::c_char; // find end of instruction list for this target
        let ref mut fresh9 =
            *(*(*root).pi.offset(i as
                                     isize)).offset(1 as libc::c_int as
                                                        isize); // null terminate pi list for this target
        *fresh9 = *fresh8 as *mut libc::c_char;
        let ref mut fresh10 =
            *(*(*root).pi.offset(i as
                                     isize)).offset(2 as libc::c_int as
                                                        isize);
        *fresh10 = strdup(b"\x00" as *const u8 as *const libc::c_char)
    }
    while !(*(*(*root).pi.offset(i as isize)).offset(j as isize)).is_null() {
        j += 1
    }
    let ref mut fresh11 = *(*root).pi.offset(i as isize);
    *fresh11 =
        realloc(*(*root).pi.offset(i as isize) as *mut libc::c_void,
                (::std::mem::size_of::<*mut libc::c_char>() as
                     libc::c_ulong).wrapping_mul((j + 3 as libc::c_int) as
                                                     libc::c_ulong)) as
            *mut *mut libc::c_char;
    let ref mut fresh12 =
        *(*(*root).pi.offset(i as
                                 isize)).offset((j + 2 as libc::c_int) as
                                                    isize);
    *fresh12 =
        realloc(*(*(*root).pi.offset(i as
                                         isize)).offset((j + 1 as libc::c_int)
                                                            as isize) as
                    *mut libc::c_void,
                (j + 1 as libc::c_int) as libc::c_ulong) as *mut libc::c_char;
    strcpy((*(*(*root).pi.offset(i as
                                     isize)).offset((j + 2 as libc::c_int) as
                                                        isize)).offset(j as
                                                                           isize).offset(-(1
                                                                                               as
                                                                                               libc::c_int
                                                                                               as
                                                                                               isize)),
           if !(*root).xml.name.is_null() {
               b">\x00" as *const u8 as *const libc::c_char
           } else { b"<\x00" as *const u8 as *const libc::c_char });
    let ref mut fresh13 =
        *(*(*root).pi.offset(i as
                                 isize)).offset((j + 1 as libc::c_int) as
                                                    isize);
    *fresh13 = 0 as *mut libc::c_char;
    let ref mut fresh14 =
        *(*(*root).pi.offset(i as isize)).offset(j as isize);
    *fresh14 = s;
    // set instruction
}
// called when the parser finds an internal doctype subset
#[no_mangle]
pub unsafe extern "C" fn ezxml_internal_dtd(mut root: ezxml_root_t,
                                            mut s: *mut libc::c_char,
                                            mut len: size_t)
 -> libc::c_short {
    let mut q: libc::c_char = 0; // find next declaration
    let mut c: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut v: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ent: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut pe: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    pe =
        memcpy(malloc(::std::mem::size_of::<[*mut libc::c_char; 1]>() as
                          libc::c_ulong),
               EZXML_NIL.as_mut_ptr() as *const libc::c_void,
               ::std::mem::size_of::<[*mut libc::c_char; 1]>() as
                   libc::c_ulong) as *mut *mut libc::c_char;
    *s.offset(len as isize) = '\u{0}' as i32 as libc::c_char;
    while !s.is_null() {
        while *s as libc::c_int != 0 && *s as libc::c_int != '<' as i32 &&
                  *s as libc::c_int != '%' as i32 {
            s = s.offset(1)
        }
        if *s == 0 { break ; }
        if strncmp(s, b"<!ENTITY\x00" as *const u8 as *const libc::c_char,
                   8 as libc::c_int as libc::c_ulong) == 0 {
            // parse entity definitions
            s =
                s.offset(strspn(s.offset(8 as libc::c_int as isize),
                                b"\t\r\n \x00" as *const u8 as
                                    *const libc::c_char).wrapping_add(8 as
                                                                          libc::c_int
                                                                          as
                                                                          libc::c_ulong)
                             as isize); // skip white space separator
            c = s;
            // set entity name
            n =
                s.offset(strspn(s,
                                b"\t\r\n %\x00" as *const u8 as
                                    *const libc::c_char) as
                             isize); // find name
            s =
                n.offset(strcspn(n,
                                 b"\t\r\n \x00" as *const u8 as
                                     *const libc::c_char) as
                             isize); // append ; to name
            *s = ';' as i32 as libc::c_char; // find value
            v =
                s.offset(strspn(s.offset(1 as libc::c_int as isize),
                                b"\t\r\n \x00" as *const u8 as
                                    *const libc::c_char) as
                             isize).offset(1 as libc::c_int as isize);
            let fresh15 = v;
            v = v.offset(1);
            q = *fresh15;
            if q as libc::c_int != '\"' as i32 &&
                   q as libc::c_int != '\'' as i32 {
                // skip externals
                s = strchr(s, '>' as i32)
            } else {
                i = 0 as libc::c_int; // space for next ent
                ent =
                    (if *c as libc::c_int == '%' as i32 {
                         pe
                     } else { (*root).ent }); // null terminate name
                while !(*ent.offset(i as isize)).is_null() {
                    i += 1
                } // null terminate value
                ent =
                    realloc(ent as *mut libc::c_void,
                            ((i + 3 as libc::c_int) as
                                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut libc::c_char>()
                                                                 as
                                                                 libc::c_ulong))
                        as *mut *mut libc::c_char; // set value
                if *c as libc::c_int == '%' as i32 {
                    pe = ent
                } else { (*root).ent = ent } // null terminate entity list
                s = s.offset(1);
                *s = '\u{0}' as i32 as libc::c_char;
                s = strchr(v, q as libc::c_int);
                if !s.is_null() {
                    let fresh16 = s;
                    s = s.offset(1);
                    *fresh16 = '\u{0}' as i32 as libc::c_char
                }
                let ref mut fresh17 =
                    *ent.offset((i + 1 as libc::c_int) as isize);
                *fresh17 = ezxml_decode(v, pe, '%' as i32 as libc::c_char);
                let ref mut fresh18 =
                    *ent.offset((i + 2 as libc::c_int) as isize);
                *fresh18 = 0 as *mut libc::c_char;
                if ezxml_ent_ok(n,
                                *ent.offset((i + 1 as libc::c_int) as isize),
                                ent) == 0 {
                    // circular reference
                    if *ent.offset((i + 1 as libc::c_int) as isize) != v {
                        free(*ent.offset((i + 1 as libc::c_int) as isize) as
                                 *mut libc::c_void);
                    }
                    ezxml_err(root, v,
                              b"circular entity declaration &%s\x00" as
                                  *const u8 as *const libc::c_char, n);
                    break ;
                } else {
                    let ref mut fresh19 = *ent.offset(i as isize);
                    *fresh19 = n
                }
            }
        } else if strncmp(s,
                          b"<!ATTLIST\x00" as *const u8 as
                              *const libc::c_char,
                          9 as libc::c_int as libc::c_ulong) == 0 {
            // parse default attributes
            t =
                s.offset(strspn(s.offset(9 as libc::c_int as isize),
                                b"\t\r\n \x00" as *const u8 as
                                    *const libc::c_char) as
                             isize).offset(9 as libc::c_int as
                                               isize); // skip whitespace separator
            if *t == 0 {
                ezxml_err(root, t,
                          b"unclosed <!ATTLIST\x00" as *const u8 as
                              *const libc::c_char); // null terminate tag name
                break ;
            } else {
                s =
                    t.offset(strcspn(t,
                                     b"\t\r\n >\x00" as *const u8 as
                                         *const libc::c_char) as isize);
                if *s as libc::c_int == '>' as i32 { continue ; }
                *s = '\u{0}' as i32 as libc::c_char;
                i = 0 as libc::c_int;
                while !(*(*root).attr.offset(i as isize)).is_null() &&
                          strcmp(n,
                                 *(*(*root).attr.offset(i as
                                                            isize)).offset(0
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize))
                              != 0 {
                    i += 1
                }
                loop  {
                    s = s.offset(1);
                    n =
                        s.offset(strspn(s,
                                        b"\t\r\n \x00" as *const u8 as
                                            *const libc::c_char) as isize);
                    if !(*n as libc::c_int != 0 &&
                             *n as libc::c_int != '>' as i32) {
                        break ;
                    }
                    s =
                        n.offset(strcspn(n,
                                         b"\t\r\n \x00" as *const u8 as
                                             *const libc::c_char) as isize);
                    if *s != 0 {
                        // attribute name 
                        *s = '\u{0}' as i32 as libc::c_char; // attr name
                        s =
                            s.offset(strspn(s.offset(1 as libc::c_int as
                                                         isize),
                                            b"\t\r\n \x00" as *const u8 as
                                                *const libc::c_char).wrapping_add(1
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      libc::c_ulong)
                                         as isize); // find next token
                        c =
                            if strncmp(s,
                                       b"CDATA\x00" as *const u8 as
                                           *const libc::c_char,
                                       5 as libc::c_int as libc::c_ulong) != 0
                               {
                                b"*\x00" as *const u8 as *const libc::c_char
                            } else {
                                b" \x00" as *const u8 as *const libc::c_char
                            } as *mut libc::c_char; // is it cdata?
                        if strncmp(s,
                                   b"NOTATION\x00" as *const u8 as
                                       *const libc::c_char,
                                   8 as libc::c_int as libc::c_ulong) == 0 {
                            s =
                                s.offset(strspn(s.offset(8 as libc::c_int as
                                                             isize),
                                                b"\t\r\n \x00" as *const u8 as
                                                    *const libc::c_char).wrapping_add(8
                                                                                          as
                                                                                          libc::c_int
                                                                                          as
                                                                                          libc::c_ulong)
                                             as isize)
                        } // skip white space separator
                        s =
                            if *s as libc::c_int == '(' as i32 {
                                strchr(s, ')' as i32)
                            } else {
                                s.offset(strcspn(s,
                                                 b"\t\r\n \x00" as *const u8
                                                     as *const libc::c_char)
                                             as isize)
                            };
                        if s.is_null() {
                            ezxml_err(root, t,
                                      b"malformed <!ATTLIST\x00" as *const u8
                                          as *const libc::c_char);
                            break ;
                        } else {
                            s =
                                s.offset(strspn(s,
                                                b"\t\r\n )\x00" as *const u8
                                                    as *const libc::c_char) as
                                             isize);
                            if strncmp(s,
                                       b"#FIXED\x00" as *const u8 as
                                           *const libc::c_char,
                                       6 as libc::c_int as libc::c_ulong) == 0
                               {
                                s =
                                    s.offset(strspn(s.offset(6 as libc::c_int
                                                                 as isize),
                                                    b"\t\r\n \x00" as
                                                        *const u8 as
                                                        *const libc::c_char).wrapping_add(6
                                                                                              as
                                                                                              libc::c_int
                                                                                              as
                                                                                              libc::c_ulong)
                                                 as isize)
                            }
                            if *s as libc::c_int == '#' as i32 {
                                // no default value
                                s =
                                    s.offset(strcspn(s,
                                                     b"\t\r\n >\x00" as
                                                         *const u8 as
                                                         *const libc::c_char).wrapping_sub(1
                                                                                               as
                                                                                               libc::c_int
                                                                                               as
                                                                                               libc::c_ulong)
                                                 as
                                                 isize); // cdata is default, nothing to do
                                if *c as libc::c_int == ' ' as i32 {
                                    continue ;
                                }
                                v = 0 as *mut libc::c_char
                            } else if (*s as libc::c_int == '\"' as i32 ||
                                           *s as libc::c_int == '\'' as i32)
                                          &&
                                          {
                                              v =
                                                  s.offset(1 as libc::c_int as
                                                               isize);
                                              s =
                                                  strchr(v,
                                                         *s as libc::c_int);
                                              !s.is_null()
                                          } {
                                *s = '\u{0}' as i32 as libc::c_char
                            } else {
                                ezxml_err(root, t,
                                          b"malformed <!ATTLIST\x00" as
                                              *const u8 as
                                              *const libc::c_char);
                                break ;
                            }
                            if (*(*root).attr.offset(i as isize)).is_null() {
                                // new tag name
                                (*root).attr =
                                    if i == 0 {
                                        malloc((2 as libc::c_int as
                                                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut *mut libc::c_char>()
                                                                                    as
                                                                                    libc::c_ulong))
                                    } else {
                                        realloc((*root).attr as
                                                    *mut libc::c_void,
                                                ((i + 2 as libc::c_int) as
                                                     libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut *mut libc::c_char>()
                                                                                     as
                                                                                     libc::c_ulong))
                                    } as
                                        *mut *mut *mut libc::c_char; // set tag name
                                let ref mut fresh20 =
                                    *(*root).attr.offset(i as
                                                             isize); // find end of list
                                *fresh20 =
                                    malloc((2 as libc::c_int as
                                                libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut libc::c_char>()
                                                                                as
                                                                                libc::c_ulong))
                                        as
                                        *mut *mut libc::c_char; // null terminate list
                                let ref mut fresh21 =
                                    *(*(*root).attr.offset(i as
                                                               isize)).offset(0
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize); // is it cdata?
                                *fresh21 = t; // comments
                                let ref mut fresh22 =
                                    *(*root).attr.offset((i +
                                                              1 as
                                                                  libc::c_int)
                                                             as isize);
                                *fresh22 = 0 as *mut *mut libc::c_char;
                                let ref mut fresh23 =
                                    *(*(*root).attr.offset(i as
                                                               isize)).offset(1
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize);
                                *fresh23 = *fresh22 as *mut libc::c_char
                            }
                            j = 1 as libc::c_int;
                            while !(*(*(*root).attr.offset(i as
                                                               isize)).offset(j
                                                                                  as
                                                                                  isize)).is_null()
                                  {
                                j += 3 as libc::c_int
                            }
                            let ref mut fresh24 =
                                *(*root).attr.offset(i as isize);
                            *fresh24 =
                                realloc(*(*root).attr.offset(i as isize) as
                                            *mut libc::c_void,
                                        ((j + 4 as libc::c_int) as
                                             libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut libc::c_char>()
                                                                             as
                                                                             libc::c_ulong))
                                    as *mut *mut libc::c_char;
                            let ref mut fresh25 =
                                *(*(*root).attr.offset(i as
                                                           isize)).offset((j +
                                                                               3
                                                                                   as
                                                                                   libc::c_int)
                                                                              as
                                                                              isize);
                            *fresh25 = 0 as *mut libc::c_char;
                            let ref mut fresh26 =
                                *(*(*root).attr.offset(i as
                                                           isize)).offset((j +
                                                                               2
                                                                                   as
                                                                                   libc::c_int)
                                                                              as
                                                                              isize);
                            *fresh26 = c;
                            let ref mut fresh27 =
                                *(*(*root).attr.offset(i as
                                                           isize)).offset((j +
                                                                               1
                                                                                   as
                                                                                   libc::c_int)
                                                                              as
                                                                              isize);
                            *fresh27 =
                                if !v.is_null() {
                                    ezxml_decode(v, (*root).ent, *c)
                                } else { 0 as *mut libc::c_char };
                            let ref mut fresh28 =
                                *(*(*root).attr.offset(i as
                                                           isize)).offset(j as
                                                                              isize);
                            *fresh28 = n
                        }
                    } else {
                        ezxml_err(root, t,
                                  b"malformed <!ATTLIST\x00" as *const u8 as
                                      *const libc::c_char);
                        break ;
                    }
                }
            }
        } else if strncmp(s, b"<!--\x00" as *const u8 as *const libc::c_char,
                          4 as libc::c_int as libc::c_ulong) == 0 {
            s =
                strstr(s.offset(4 as libc::c_int as isize),
                       b"-->\x00" as *const u8 as *const libc::c_char)
        } else if strncmp(s, b"<?\x00" as *const u8 as *const libc::c_char,
                          2 as libc::c_int as libc::c_ulong) == 0 {
            // processing instructions
            c =
                s.offset(2 as libc::c_int as
                             isize); // skip other declarations
            s = strstr(c, b"?>\x00" as *const u8 as *const libc::c_char);
            if !s.is_null() {
                let fresh29 = s;
                s = s.offset(1);
                ezxml_proc_inst(root, c,
                                fresh29.offset_from(c) as
                                    libc::c_long as size_t);
            }
        } else if *s as libc::c_int == '<' as i32 {
            s = strchr(s, '>' as i32)
        } else {
            let fresh30 = s;
            s = s.offset(1);
            if *fresh30 as libc::c_int == '%' as i32 &&
                   (*root).standalone == 0 {
                break ;
            }
        }
    }
    free(pe as *mut libc::c_void);
    return (*(*root).err.as_mut_ptr() == 0) as libc::c_int as libc::c_short;
}
// Converts a UTF-16 string to UTF-8. Returns a new string that must be freed
// or NULL if no conversion was needed.
#[no_mangle]
pub unsafe extern "C" fn ezxml_str2utf8(mut s: *mut *mut libc::c_char,
                                        mut len: *mut size_t)
 -> *mut libc::c_char {
    let mut u: *mut libc::c_char = 0 as *mut libc::c_char; // not UTF-16
    let mut l: size_t = 0 as libc::c_int as size_t; //UTF-16LE
    let mut sl: size_t = 0;
    let mut max: size_t = *len;
    let mut c: libc::c_long = 0;
    let mut d: libc::c_long = 0;
    let mut b: libc::c_int = 0;
    let mut be: libc::c_int =
        if **s as libc::c_int == -2i32 {
            1 as libc::c_int
        } else if **s as libc::c_int == -1i32 {
            0 as libc::c_int
        } else { -(1 as libc::c_int) };
    if be == -(1 as libc::c_int) { return 0 as *mut libc::c_char }
    u = malloc(max) as *mut libc::c_char;
    sl = 2 as libc::c_int as size_t;
    while sl < (*len).wrapping_sub(1 as libc::c_int as libc::c_ulong) {
        c =
            if be != 0 {
                ((*(*s).offset(sl as isize) as libc::c_int &
                      0xff as libc::c_int) << 8 as libc::c_int) |
                    *(*s).offset(sl.wrapping_add(1 as libc::c_int as
                                                     libc::c_ulong) as isize)
                        as libc::c_int & 0xff as libc::c_int
            } else {
                ((*(*s).offset(sl.wrapping_add(1 as libc::c_int as
                                                   libc::c_ulong) as isize) as
                      libc::c_int & 0xff as libc::c_int) << 8 as libc::c_int)
                    |
                    *(*s).offset(sl as isize) as libc::c_int &
                        0xff as libc::c_int
            } as libc::c_long;
        if c >= 0xd800 as libc::c_int as libc::c_long &&
               c <= 0xdfff as libc::c_int as libc::c_long &&
               {
                   sl =
                       (sl as
                            libc::c_ulong).wrapping_add(2 as libc::c_int as
                                                            libc::c_ulong) as
                           size_t as size_t;
                   (sl) <
                       (*len).wrapping_sub(1 as libc::c_int as libc::c_ulong)
               } {
            // high-half
            d =
                if be != 0 {
                    ((*(*s).offset(sl as isize) as libc::c_int &
                          0xff as libc::c_int) << 8 as libc::c_int) |
                        *(*s).offset(sl.wrapping_add(1 as libc::c_int as
                                                         libc::c_ulong) as
                                         isize) as libc::c_int &
                            0xff as libc::c_int
                } else {
                    ((*(*s).offset(sl.wrapping_add(1 as libc::c_int as
                                                       libc::c_ulong) as
                                       isize) as libc::c_int &
                          0xff as libc::c_int) << 8 as libc::c_int) |
                        *(*s).offset(sl as isize) as libc::c_int &
                            0xff as libc::c_int
                } as libc::c_long; // US-ASCII subset
            c =
                ((c & 0x3ff as libc::c_int as libc::c_long) <<
                     10 as libc::c_int |
                     d & 0x3ff as libc::c_int as libc::c_long) +
                    0x10000 as libc::c_int as libc::c_long
        }
        while l.wrapping_add(6 as libc::c_int as libc::c_ulong) > max {
            max =
                (max as
                     libc::c_ulong).wrapping_add(1024 as libc::c_int as
                                                     libc::c_ulong) as size_t
                    as size_t;
            u = realloc(u as *mut libc::c_void, max) as *mut libc::c_char
        }
        if c < 0x80 as libc::c_int as libc::c_long {
            let fresh31 = l;
            l = l.wrapping_add(1);
            *u.offset(fresh31 as isize) = c as libc::c_char
        } else {
            // multi-byte UTF-8 sequence
            b = 0 as libc::c_int; // bits in c
            d = c;
            while d != 0 { b += 1; d /= 2 as libc::c_int as libc::c_long }
            // payload
            b = (b - 2 as libc::c_int) / 5 as libc::c_int; // bytes in payload
            let fresh32 = l; // head
            l = l.wrapping_add(1);
            *u.offset(fresh32 as isize) =
                (((0xff as libc::c_int) << 7 as libc::c_int - b) as
                     libc::c_long | c >> 6 as libc::c_int * b) as
                    libc::c_char;
            while b != 0 {
                b -= 1;
                let fresh33 = l;
                l = l.wrapping_add(1);
                *u.offset(fresh33 as isize) =
                    (0x80 as libc::c_int as libc::c_long |
                         c >> 6 as libc::c_int * b &
                             0x3f as libc::c_int as libc::c_long) as
                        libc::c_char
            }
        }
        sl =
            (sl as
                 libc::c_ulong).wrapping_add(2 as libc::c_int as
                                                 libc::c_ulong) as size_t as
                size_t
    }
    *len = l;
    *s = realloc(u as *mut libc::c_void, *len) as *mut libc::c_char;
    return *s;
}
// frees a tag attribute list
#[no_mangle]
pub unsafe extern "C" fn ezxml_free_attr(mut attr: *mut *mut libc::c_char) {
    let mut i: libc::c_int = 0 as libc::c_int; // nothing to free
    let mut m: *mut libc::c_char =
        0 as *mut libc::c_char; // find end of attribute list
    if attr.is_null() || attr == EZXML_NIL.as_mut_ptr() {
        return
    } // list of which names and values are malloced
    while !(*attr.offset(i as isize)).is_null() { i += 2 as libc::c_int }
    m = *attr.offset((i + 1 as libc::c_int) as isize);
    i = 0 as libc::c_int;
    while *m.offset(i as isize) != 0 {
        if *m.offset(i as isize) as libc::c_int & 0x80 as libc::c_int != 0 {
            free(*attr.offset((i * 2 as libc::c_int) as isize) as
                     *mut libc::c_void);
        }
        if *m.offset(i as isize) as libc::c_int & 0x40 as libc::c_int != 0 {
            free(*attr.offset((i * 2 as libc::c_int + 1 as libc::c_int) as
                                  isize) as *mut libc::c_void);
        }
        i += 1
    }
    free(m as *mut libc::c_void);
    free(attr as *mut libc::c_void);
}
// parse the given xml string and return an ezxml structure
#[no_mangle]
pub unsafe extern "C" fn ezxml_parse_str(mut s: *mut libc::c_char,
                                         mut len: size_t) -> ezxml_t {
    let mut root: ezxml_root_t =
        ezxml_new(0 as *const libc::c_char) as
            ezxml_root_t; // initialize a to avoid compile warning
    let mut q: libc::c_char = 0; // convert utf-16 to utf-8
    let mut e: libc::c_char = 0; // record start and end of work area
    let mut d: *mut libc::c_char = 0 as *mut libc::c_char; // save end char
    let mut attr: *mut *mut libc::c_char =
        0 as *mut *mut libc::c_char; // turn end char into null terminator
    let mut a: *mut *mut libc::c_char =
        0 as *mut *mut libc::c_char; // find first tag
    let mut l: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    (*root).m = s;
    if len == 0 {
        return ezxml_err(root, 0 as *mut libc::c_char,
                         b"root tag missing\x00" as *const u8 as
                             *const libc::c_char)
    }
    (*root).u = ezxml_str2utf8(&mut s, &mut len);
    (*root).s = s;
    (*root).e = (*root).s.offset(len as isize);
    e =
        *s.offset(len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as
                      isize);
    *s.offset(len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize) =
        '\u{0}' as i32 as libc::c_char;
    while *s as libc::c_int != 0 && *s as libc::c_int != '<' as i32 {
        s = s.offset(1)
    }
    if *s == 0 {
        return ezxml_err(root, s,
                         b"root tag missing\x00" as *const u8 as
                             *const libc::c_char)
    }
    loop  {
        attr = EZXML_NIL.as_mut_ptr();
        s = s.offset(1);
        d = s;
        if *(*__ctype_b_loc()).offset(*s as libc::c_int as isize) as
               libc::c_int &
               _ISalpha as libc::c_int as libc::c_ushort as libc::c_int != 0
               || *s as libc::c_int == '_' as i32 ||
               *s as libc::c_int == ':' as i32 ||
               (*s as libc::c_int) < '\u{0}' as i32 {
            // new tag
            if (*root).cur.is_null() {
                return ezxml_err(root, d,
                                 b"markup outside of root element\x00" as
                                     *const u8 as *const libc::c_char)
            } // null terminate tag name
            s =
                s.offset(strcspn(s,
                                 b"\t\r\n />\x00" as *const u8 as
                                     *const libc::c_char) as isize);
            while *(*__ctype_b_loc()).offset(*s as libc::c_int as isize) as
                      libc::c_int &
                      _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                      != 0 {
                let fresh34 = s;
                s = s.offset(1);
                *fresh34 = '\u{0}' as i32 as libc::c_char
            }
            if *s as libc::c_int != 0 && *s as libc::c_int != '/' as i32 &&
                   *s as libc::c_int != '>' as i32 {
                // find tag in default attr list
                i = 0 as libc::c_int;
                loop  {
                    a = *(*root).attr.offset(i as isize);
                    if !(!a.is_null() &&
                             strcmp(*a.offset(0 as libc::c_int as isize), d)
                                 != 0) {
                        break ;
                    }
                    i += 1
                }
            }
            l = 0 as libc::c_int;
            while *s as libc::c_int != 0 && *s as libc::c_int != '/' as i32 &&
                      *s as libc::c_int != '>' as i32 {
                // new attrib
                attr =
                    if l != 0 {
                        realloc(attr as *mut libc::c_void,
                                ((l + 4 as libc::c_int) as
                                     libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut libc::c_char>()
                                                                     as
                                                                     libc::c_ulong))
                    } else {
                        malloc((4 as libc::c_int as
                                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut libc::c_char>()
                                                                    as
                                                                    libc::c_ulong))
                    } as *mut *mut libc::c_char; // allocate space
                let ref mut fresh35 =
                    *attr.offset((l + 3 as libc::c_int) as
                                     isize); // mem for list of maloced vals
                *fresh35 =
                    if l != 0 {
                        realloc(*attr.offset((l + 1 as libc::c_int) as isize)
                                    as *mut libc::c_void,
                                (l / 2 as libc::c_int + 2 as libc::c_int) as
                                    libc::c_ulong)
                    } else { malloc(2 as libc::c_int as libc::c_ulong) } as
                        *mut libc::c_char; // value is not malloced
                strcpy((*attr.offset((l + 3 as libc::c_int) as
                                         isize)).offset((l / 2 as libc::c_int)
                                                            as isize),
                       b" \x00" as *const u8 as
                           *const libc::c_char); // null terminate list
                let ref mut fresh36 =
                    *attr.offset((l + 2 as libc::c_int) as
                                     isize); // temporary attribute value
                *fresh36 = 0 as *mut libc::c_char; // set attribute name
                let ref mut fresh37 =
                    *attr.offset((l + 1 as libc::c_int) as
                                     isize); // null terminate tag attribute name
                *fresh37 =
                    b"\x00" as *const u8 as *const libc::c_char as
                        *mut libc::c_char;
                let ref mut fresh38 = *attr.offset(l as isize);
                *fresh38 = s;
                s =
                    s.offset(strcspn(s,
                                     b"\t\r\n =/>\x00" as *const u8 as
                                         *const libc::c_char) as isize);
                if *s as libc::c_int == '=' as i32 ||
                       *(*__ctype_b_loc()).offset(*s as libc::c_int as isize)
                           as libc::c_int &
                           _ISspace as libc::c_int as libc::c_ushort as
                               libc::c_int != 0 {
                    let fresh39 = s;
                    s = s.offset(1);
                    *fresh39 = '\u{0}' as i32 as libc::c_char;
                    s =
                        s.offset(strspn(s,
                                        b"\t\r\n =\x00" as *const u8 as
                                            *const libc::c_char) as isize);
                    q = *s;
                    if q as libc::c_int == '\"' as i32 ||
                           q as libc::c_int == '\'' as i32 {
                        // attribute value
                        s = s.offset(1);
                        let ref mut fresh40 =
                            *attr.offset((l + 1 as libc::c_int) as isize);
                        *fresh40 = s;
                        while *s as libc::c_int != 0 &&
                                  *s as libc::c_int != q as libc::c_int {
                            s = s.offset(1)
                        }
                        if *s != 0 {
                            // value malloced
                            let fresh41 = s; // null terminate attribute val
                            s = s.offset(1);
                            *fresh41 = '\u{0}' as i32 as libc::c_char
                        } else {
                            ezxml_free_attr(attr);
                            return ezxml_err(root, d,
                                             b"missing %c\x00" as *const u8 as
                                                 *const libc::c_char,
                                             q as libc::c_int)
                        }
                        j = 1 as libc::c_int;
                        while !a.is_null() &&
                                  !(*a.offset(j as isize)).is_null() &&
                                  strcmp(*a.offset(j as isize),
                                         *attr.offset(l as isize)) != 0 {
                            j += 3 as libc::c_int
                        }
                        let ref mut fresh42 =
                            *attr.offset((l + 1 as libc::c_int) as isize);
                        *fresh42 =
                            ezxml_decode(*attr.offset((l + 1 as libc::c_int)
                                                          as isize),
                                         (*root).ent,
                                         if !a.is_null() &&
                                                !(*a.offset(j as
                                                                isize)).is_null()
                                            {
                                             **a.offset((j + 2 as libc::c_int)
                                                            as isize) as
                                                 libc::c_int
                                         } else { ' ' as i32 } as
                                             libc::c_char);
                        if *attr.offset((l + 1 as libc::c_int) as isize) < d
                               ||
                               *attr.offset((l + 1 as libc::c_int) as isize) >
                                   s {
                            *(*attr.offset((l + 3 as libc::c_int) as
                                               isize)).offset((l /
                                                                   2 as
                                                                       libc::c_int)
                                                                  as isize) =
                                0x40 as libc::c_int as libc::c_char
                        }
                    }
                }
                while *(*__ctype_b_loc()).offset(*s as libc::c_int as isize)
                          as libc::c_int &
                          _ISspace as libc::c_int as libc::c_ushort as
                              libc::c_int != 0 {
                    s = s.offset(1)
                }
                l += 2 as libc::c_int
            }
            if *s as libc::c_int == '/' as i32 {
                // self closing tag
                let fresh43 = s;
                s = s.offset(1);
                *fresh43 = '\u{0}' as i32 as libc::c_char;
                if *s as libc::c_int != 0 && *s as libc::c_int != '>' as i32
                       || *s == 0 && e as libc::c_int != '>' as i32 {
                    if l != 0 { ezxml_free_attr(attr); }
                    return ezxml_err(root, d,
                                     b"missing >\x00" as *const u8 as
                                         *const libc::c_char)
                }
                ezxml_open_tag(root, d, attr);
                ezxml_close_tag(root, d, s);
            } else {
                q = *s;
                if q as libc::c_int == '>' as i32 ||
                       *s == 0 && e as libc::c_int == '>' as i32 {
                    // open tag
                    *s =
                        '\u{0}' as i32 as
                            libc::c_char; // temporarily null terminate tag name
                    ezxml_open_tag(root, d, attr);
                    *s = q
                } else {
                    if l != 0 { ezxml_free_attr(attr); }
                    return ezxml_err(root, d,
                                     b"missing >\x00" as *const u8 as
                                         *const libc::c_char)
                }
            }
        } else if *s as libc::c_int == '/' as i32 {
            // close tag
            d =
                s.offset(1 as libc::c_int as
                             isize); // temporarily null terminate tag name
            s =
                s.offset(strcspn(d,
                                 b"\t\r\n >\x00" as *const u8 as
                                     *const libc::c_char).wrapping_add(1 as
                                                                           libc::c_int
                                                                           as
                                                                           libc::c_ulong)
                             as isize);
            q = *s;
            if q == 0 && e as libc::c_int != '>' as i32 {
                return ezxml_err(root, d,
                                 b"missing >\x00" as *const u8 as
                                     *const libc::c_char)
            }
            *s = '\u{0}' as i32 as libc::c_char;
            if !ezxml_close_tag(root, d, s).is_null() {
                return &mut (*root).xml
            }
            *s = q;
            if *(*__ctype_b_loc()).offset(*s as libc::c_int as isize) as
                   libc::c_int &
                   _ISspace as libc::c_int as libc::c_ushort as libc::c_int !=
                   0 {
                s =
                    s.offset(strspn(s,
                                    b"\t\r\n \x00" as *const u8 as
                                        *const libc::c_char) as isize)
            }
        } else if strncmp(s, b"!--\x00" as *const u8 as *const libc::c_char,
                          3 as libc::c_int as libc::c_ulong) == 0 {
            // xml comment
            s =
                strstr(s.offset(3 as libc::c_int as isize),
                       b"--\x00" as *const u8 as *const libc::c_char);
            if s.is_null() ||
                   {
                       s = s.offset(2 as libc::c_int as isize);
                       (*s as libc::c_int != '>' as i32) &&
                           *s as libc::c_int != 0
                   } || *s == 0 && e as libc::c_int != '>' as i32 {
                return ezxml_err(root, d,
                                 b"unclosed <!--\x00" as *const u8 as
                                     *const libc::c_char)
            }
        } else if strncmp(s,
                          b"![CDATA[\x00" as *const u8 as *const libc::c_char,
                          8 as libc::c_int as libc::c_ulong) == 0 {
            // cdata
            s = strstr(s, b"]]>\x00" as *const u8 as *const libc::c_char);
            if !s.is_null() {
                s = s.offset(2 as libc::c_int as isize);
                ezxml_char_content(root, d.offset(8 as libc::c_int as isize),
                                   (s.offset_from(d) as libc::c_long
                                        - 10 as libc::c_int as libc::c_long)
                                       as size_t, 'c' as i32 as libc::c_char);
            } else {
                return ezxml_err(root, d,
                                 b"unclosed <![CDATA[\x00" as *const u8 as
                                     *const libc::c_char)
            }
        } else if strncmp(s,
                          b"!DOCTYPE\x00" as *const u8 as *const libc::c_char,
                          8 as libc::c_int as libc::c_ulong) == 0 {
            // dtd
            l = 0 as libc::c_int;
            while *s as libc::c_int != 0 &&
                      (l == 0 && *s as libc::c_int != '>' as i32 ||
                           l != 0 &&
                               (*s as libc::c_int != ']' as i32 ||
                                    *s.offset(strspn(s.offset(1 as libc::c_int
                                                                  as isize),
                                                     b"\t\r\n \x00" as
                                                         *const u8 as
                                                         *const libc::c_char)
                                                  as
                                                  isize).offset(1 as
                                                                    libc::c_int
                                                                    as isize)
                                        as libc::c_int != '>' as i32)) {
                s =
                    s.offset(strcspn(s.offset(1 as libc::c_int as isize),
                                     b"[]>\x00" as *const u8 as
                                         *const libc::c_char).wrapping_add(1
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               libc::c_ulong)
                                 as isize);
                l =
                    if *s as libc::c_int == '[' as i32 {
                        1 as libc::c_int
                    } else { l }
            }
            if *s == 0 && e as libc::c_int != '>' as i32 {
                return ezxml_err(root, d,
                                 b"unclosed <!DOCTYPE\x00" as *const u8 as
                                     *const libc::c_char)
            }
            d =
                if l != 0 {
                    strchr(d, '[' as i32).offset(1 as libc::c_int as isize)
                } else { d };
            if l != 0 &&
                   {
                       let fresh44 = s;
                       s = s.offset(1);
                       (ezxml_internal_dtd(root, d,
                                           fresh44.offset_from(d) as
                                               libc::c_long as size_t)) == 0
                   } {
                return &mut (*root).xml
            }
        } else if *s as libc::c_int == '?' as i32 {
            loop 
                 // <?...?> processing instructions
                 {
                s = strchr(s, '?' as i32);
                if !(!s.is_null() &&
                         { s = s.offset(1); (*s as libc::c_int) != 0 } &&
                         *s as libc::c_int != '>' as i32) {
                    break ;
                }
            }
            if s.is_null() || *s == 0 && e as libc::c_int != '>' as i32 {
                return ezxml_err(root, d,
                                 b"unclosed <?\x00" as *const u8 as
                                     *const libc::c_char)
            } else {
                ezxml_proc_inst(root, d.offset(1 as libc::c_int as isize),
                                (s.offset_from(d) as libc::c_long -
                                     2 as libc::c_int as libc::c_long) as
                                    size_t);
            }
        } else {
            return ezxml_err(root, d,
                             b"unexpected <\x00" as *const u8 as
                                 *const libc::c_char)
        }
        if s.is_null() || *s == 0 { break ; }
        *s = '\u{0}' as i32 as libc::c_char;
        s = s.offset(1);
        d = s;
        if *s as libc::c_int != 0 && *s as libc::c_int != '<' as i32 {
            // tag character content
            while *s as libc::c_int != 0 && *s as libc::c_int != '<' as i32 {
                s = s.offset(1)
            }
            if !(*s != 0) { break ; }
            ezxml_char_content(root, d,
                               s.offset_from(d) as libc::c_long as
                                   size_t, '&' as i32 as libc::c_char);
        } else if *s == 0 { break ; }
    }
    if (*root).cur.is_null() {
        return &mut (*root).xml
    } else if (*(*root).cur).name.is_null() {
        return ezxml_err(root, d,
                         b"root tag missing\x00" as *const u8 as
                             *const libc::c_char)
    } else {
        return ezxml_err(root, d,
                         b"unclosed tag <%s>\x00" as *const u8 as
                             *const libc::c_char, (*(*root).cur).name)
    };
}
// Wrapper for ezxml_parse_str() that accepts a file stream. Reads the entire
// stream into memory and then parses it. For xml files, use ezxml_parse_file()
// or ezxml_parse_fd()
#[no_mangle]
pub unsafe extern "C" fn ezxml_parse_fp(mut fp: *mut FILE) -> ezxml_t {
    let mut root: ezxml_root_t =
        0 as *mut ezxml_root; // so we know to free s in ezxml_free()
    let mut l: size_t = 0;
    let mut len: size_t = 0 as libc::c_int as size_t;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    s = malloc(1024 as libc::c_int as libc::c_ulong) as *mut libc::c_char;
    if s.is_null() { return 0 as ezxml_t }
    loop  {
        l =
            fread(s.offset(len as isize) as *mut libc::c_void,
                  1 as libc::c_int as libc::c_ulong,
                  1024 as libc::c_int as libc::c_ulong, fp);
        len = (len as libc::c_ulong).wrapping_add(l) as size_t as size_t;
        if l == 1024 as libc::c_int as libc::c_ulong {
            s =
                realloc(s as *mut libc::c_void,
                        len.wrapping_add(1024 as libc::c_int as
                                             libc::c_ulong)) as
                    *mut libc::c_char
        }
        if !(!s.is_null() && l == 1024 as libc::c_int as libc::c_ulong) {
            break ;
        }
    }
    if s.is_null() { return 0 as ezxml_t }
    root = ezxml_parse_str(s, len) as ezxml_root_t;
    (*root).len = -(1 as libc::c_int) as size_t;
    return &mut (*root).xml;
}
// A wrapper for ezxml_parse_str() that accepts a file descriptor. First
// attempts to mem map the file. Failing that, reads the file into memory.
// Returns NULL on failure.
#[no_mangle]
pub unsafe extern "C" fn ezxml_parse_fd(mut fd: libc::c_int) -> ezxml_t {
    let mut root: ezxml_root_t =
        0 as *mut ezxml_root; // optimize for sequential access
    let mut st: stat =
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
    let mut l: size_t = 0;
    let mut m: *mut libc::c_void = 0 as *mut libc::c_void;
    if fd < 0 as libc::c_int { return 0 as ezxml_t }
    fstat(fd, &mut st);
    l =
        (st.st_size + sysconf(_SC_PAGESIZE as libc::c_int) -
             1 as libc::c_int as libc::c_long &
             !(sysconf(_SC_PAGESIZE as libc::c_int) -
                   1 as libc::c_int as libc::c_long)) as size_t;
    m =
        mmap(0 as *mut libc::c_void, l,
             0x1 as libc::c_int | 0x2 as libc::c_int, 0x2 as libc::c_int, fd,
             0 as libc::c_int as __off_t);
    if m != -(1 as libc::c_int) as *mut libc::c_void {
        madvise(m, l, 2 as libc::c_int);
        root =
            ezxml_parse_str(m as *mut libc::c_char, st.st_size as size_t) as
                ezxml_root_t;
        (*root).len = l;
        madvise(m, (*root).len, 0 as libc::c_int);
        // put it back to normal
    } else {
        // mmap failed, read file into memory
        // EZXML_NOMMAP
        m = malloc(st.st_size as libc::c_ulong);
        l = read(fd, m, st.st_size as size_t) as size_t;
        root = ezxml_parse_str(m as *mut libc::c_char, l) as ezxml_root_t;
        (*root).len = -(1 as libc::c_int) as size_t
        // so we know to free s in ezxml_free()
    }
    // EZXML_NOMMAP
    return &mut (*root).xml;
}
// a wrapper for ezxml_parse_fd that accepts a file name
#[no_mangle]
pub unsafe extern "C" fn ezxml_parse_file(mut file: *const libc::c_char)
 -> ezxml_t {
    let mut fd: libc::c_int = open(file, 0 as libc::c_int, 0 as libc::c_int);
    let mut xml: ezxml_t = ezxml_parse_fd(fd);
    if fd >= 0 as libc::c_int { close(fd); }
    return xml;
}
// Encodes ampersand sequences appending the results to *dst, reallocating *dst
// if length excedes max. a is non-zero for attribute encoding. Returns *dst
#[no_mangle]
pub unsafe extern "C" fn ezxml_ampencode(mut s: *const libc::c_char,
                                         mut len: size_t,
                                         mut dst: *mut *mut libc::c_char,
                                         mut dlen: *mut size_t,
                                         mut max: *mut size_t,
                                         mut a: libc::c_short)
 -> *mut libc::c_char {
    let mut e: *const libc::c_char = 0 as *const libc::c_char;
    e = s.offset(len as isize);
    while s != e {
        while (*dlen).wrapping_add(10 as libc::c_int as libc::c_ulong) > *max
              {
            *max =
                (*max as
                     libc::c_ulong).wrapping_add(1024 as libc::c_int as
                                                     libc::c_ulong) as size_t
                    as size_t;
            *dst =
                realloc(*dst as *mut libc::c_void, *max) as *mut libc::c_char
        }
        match *s as libc::c_int {
            0 => { return *dst }
            38 => {
                *dlen =
                    (*dlen as
                         libc::c_ulong).wrapping_add(sprintf((*dst).offset(*dlen
                                                                               as
                                                                               isize),
                                                             b"&amp;\x00" as
                                                                 *const u8 as
                                                                 *const libc::c_char)
                                                         as libc::c_ulong) as
                        size_t as size_t
            }
            60 => {
                *dlen =
                    (*dlen as
                         libc::c_ulong).wrapping_add(sprintf((*dst).offset(*dlen
                                                                               as
                                                                               isize),
                                                             b"&lt;\x00" as
                                                                 *const u8 as
                                                                 *const libc::c_char)
                                                         as libc::c_ulong) as
                        size_t as size_t
            }
            62 => {
                *dlen =
                    (*dlen as
                         libc::c_ulong).wrapping_add(sprintf((*dst).offset(*dlen
                                                                               as
                                                                               isize),
                                                             b"&gt;\x00" as
                                                                 *const u8 as
                                                                 *const libc::c_char)
                                                         as libc::c_ulong) as
                        size_t as size_t
            }
            34 => {
                *dlen =
                    (*dlen as
                         libc::c_ulong).wrapping_add(sprintf((*dst).offset(*dlen
                                                                               as
                                                                               isize),
                                                             if a as
                                                                    libc::c_int
                                                                    != 0 {
                                                                 b"&quot;\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char
                                                             } else {
                                                                 b"\"\x00" as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char
                                                             }) as
                                                         libc::c_ulong) as
                        size_t as size_t
            }
            10 => {
                *dlen =
                    (*dlen as
                         libc::c_ulong).wrapping_add(sprintf((*dst).offset(*dlen
                                                                               as
                                                                               isize),
                                                             if a as
                                                                    libc::c_int
                                                                    != 0 {
                                                                 b"&#xA;\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char
                                                             } else {
                                                                 b"\n\x00" as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char
                                                             }) as
                                                         libc::c_ulong) as
                        size_t as size_t
            }
            9 => {
                *dlen =
                    (*dlen as
                         libc::c_ulong).wrapping_add(sprintf((*dst).offset(*dlen
                                                                               as
                                                                               isize),
                                                             if a as
                                                                    libc::c_int
                                                                    != 0 {
                                                                 b"&#x9;\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char
                                                             } else {
                                                                 b"\t\x00" as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char
                                                             }) as
                                                         libc::c_ulong) as
                        size_t as size_t
            }
            13 => {
                *dlen =
                    (*dlen as
                         libc::c_ulong).wrapping_add(sprintf((*dst).offset(*dlen
                                                                               as
                                                                               isize),
                                                             b"&#xD;\x00" as
                                                                 *const u8 as
                                                                 *const libc::c_char)
                                                         as libc::c_ulong) as
                        size_t as size_t
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
// Recursively converts each tag to xml appending it to *s. Reallocates *s if
// its length excedes max. start is the location of the previous tag in the
// parent tag's character content. Returns *s.
#[no_mangle]
pub unsafe extern "C" fn ezxml_toxml_r(mut xml: ezxml_t,
                                       mut s: *mut *mut libc::c_char,
                                       mut len: *mut size_t,
                                       mut max: *mut size_t,
                                       mut start: size_t,
                                       mut attr: *mut *mut *mut libc::c_char)
 -> *mut libc::c_char {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut txt: *mut libc::c_char =
        if !(*xml).parent.is_null() {
            (*(*xml).parent).txt
        } else { b"\x00" as *const u8 as *const libc::c_char } as
            *mut libc::c_char;
    let mut off: size_t = 0 as libc::c_int as size_t;
    // parent character content up to this tag
    *s =
        ezxml_ampencode(txt.offset(start as isize),
                        (*xml).off.wrapping_sub(start), s, len, max,
                        0 as libc::c_int as libc::c_short);
    while (*len).wrapping_add(strlen((*xml).name)).wrapping_add(4 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_ulong)
              > *max {
        // reallocate s
        *max =
            (*max as
                 libc::c_ulong).wrapping_add(1024 as libc::c_int as
                                                 libc::c_ulong) as size_t as
                size_t; // open tag
        *s = realloc(*s as *mut libc::c_void, *max) as *mut libc::c_char
    }
    *len =
        (*len as
             libc::c_ulong).wrapping_add(sprintf((*s).offset(*len as isize),
                                                 b"<%s\x00" as *const u8 as
                                                     *const libc::c_char,
                                                 (*xml).name) as
                                             libc::c_ulong) as size_t as
            size_t;
    i = 0 as libc::c_int;
    while !(*(*xml).attr.offset(i as isize)).is_null() {
        // tag attributes
        if !(ezxml_attr(xml, *(*xml).attr.offset(i as isize)) !=
                 *(*xml).attr.offset((i + 1 as libc::c_int) as isize)) {
            while (*len).wrapping_add(strlen(*(*xml).attr.offset(i as
                                                                     isize))).wrapping_add(7
                                                                                               as
                                                                                               libc::c_int
                                                                                               as
                                                                                               libc::c_ulong)
                      > *max {
                // reallocate s
                *max =
                    (*max as
                         libc::c_ulong).wrapping_add(1024 as libc::c_int as
                                                         libc::c_ulong) as
                        size_t as size_t;
                *s =
                    realloc(*s as *mut libc::c_void, *max) as
                        *mut libc::c_char
            }
            *len =
                (*len as
                     libc::c_ulong).wrapping_add(sprintf((*s).offset(*len as
                                                                         isize),
                                                         b" %s=\"\x00" as
                                                             *const u8 as
                                                             *const libc::c_char,
                                                         *(*xml).attr.offset(i
                                                                                 as
                                                                                 isize))
                                                     as libc::c_ulong) as
                    size_t as size_t;
            ezxml_ampencode(*(*xml).attr.offset((i + 1 as libc::c_int) as
                                                    isize),
                            -(1 as libc::c_int) as size_t, s, len, max,
                            1 as libc::c_int as libc::c_short);
            *len =
                (*len as
                     libc::c_ulong).wrapping_add(sprintf((*s).offset(*len as
                                                                         isize),
                                                         b"\"\x00" as
                                                             *const u8 as
                                                             *const libc::c_char)
                                                     as libc::c_ulong) as
                    size_t as size_t
        }
        i += 2 as libc::c_int
    }
    i = 0 as libc::c_int;
    while !(*attr.offset(i as isize)).is_null() &&
              strcmp(*(*attr.offset(i as
                                        isize)).offset(0 as libc::c_int as
                                                           isize),
                     (*xml).name) != 0 {
        i += 1
    }
    j = 1 as libc::c_int;
    while !(*attr.offset(i as isize)).is_null() &&
              !(*(*attr.offset(i as isize)).offset(j as isize)).is_null() {
        // default attributes
        if !((*(*attr.offset(i as
                                 isize)).offset((j + 1 as libc::c_int) as
                                                    isize)).is_null() ||
                 ezxml_attr(xml,
                            *(*attr.offset(i as isize)).offset(j as isize)) !=
                     *(*attr.offset(i as
                                        isize)).offset((j + 1 as libc::c_int)
                                                           as isize)) {
            while (*len).wrapping_add(strlen(*(*attr.offset(i as
                                                                isize)).offset(j
                                                                                   as
                                                                                   isize))).wrapping_add(7
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             libc::c_ulong)
                      > *max { // skip duplicates and non-values
                // reallocate s
                *max =
                    (*max as
                         libc::c_ulong).wrapping_add(1024 as libc::c_int as
                                                         libc::c_ulong) as
                        size_t as size_t; //data
                *s =
                    realloc(*s as *mut libc::c_void, *max) as
                        *mut libc::c_char
            }
            *len =
                (*len as
                     libc::c_ulong).wrapping_add(sprintf((*s).offset(*len as
                                                                         isize),
                                                         b" %s=\"\x00" as
                                                             *const u8 as
                                                             *const libc::c_char,
                                                         *(*attr.offset(i as
                                                                            isize)).offset(j
                                                                                               as
                                                                                               isize))
                                                     as libc::c_ulong) as
                    size_t as size_t;
            ezxml_ampencode(*(*attr.offset(i as
                                               isize)).offset((j +
                                                                   1 as
                                                                       libc::c_int)
                                                                  as isize),
                            -(1 as libc::c_int) as size_t, s, len, max,
                            1 as libc::c_int as libc::c_short);
            *len =
                (*len as
                     libc::c_ulong).wrapping_add(sprintf((*s).offset(*len as
                                                                         isize),
                                                         b"\"\x00" as
                                                             *const u8 as
                                                             *const libc::c_char)
                                                     as libc::c_ulong) as
                    size_t as size_t
        }
        j += 3 as libc::c_int
    }
    *len =
        (*len as
             libc::c_ulong).wrapping_add(sprintf((*s).offset(*len as isize),
                                                 b">\x00" as *const u8 as
                                                     *const libc::c_char) as
                                             libc::c_ulong) as size_t as
            size_t;
    *s =
        if !(*xml).child.is_null() {
            ezxml_toxml_r((*xml).child, s, len, max,
                          0 as libc::c_int as size_t, attr)
        } else {
            ezxml_ampencode((*xml).txt, -(1 as libc::c_int) as size_t, s, len,
                            max, 0 as libc::c_int as libc::c_short)
        };
    while (*len).wrapping_add(strlen((*xml).name)).wrapping_add(4 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_ulong)
              > *max {
        // reallocate s
        *max =
            (*max as
                 libc::c_ulong).wrapping_add(1024 as libc::c_int as
                                                 libc::c_ulong) as size_t as
                size_t; // close tag
        *s = realloc(*s as *mut libc::c_void, *max) as *mut libc::c_char
    } // make sure off is within bounds
    *len =
        (*len as
             libc::c_ulong).wrapping_add(sprintf((*s).offset(*len as isize),
                                                 b"</%s>\x00" as *const u8 as
                                                     *const libc::c_char,
                                                 (*xml).name) as
                                             libc::c_ulong) as size_t as
            size_t;
    while *txt.offset(off as isize) as libc::c_int != 0 && off < (*xml).off {
        off = off.wrapping_add(1)
    }
    return if !(*xml).ordered.is_null() {
               ezxml_toxml_r((*xml).ordered, s, len, max, off, attr)
           } else {
               ezxml_ampencode(txt.offset(off as isize),
                               -(1 as libc::c_int) as size_t, s, len, max,
                               0 as libc::c_int as libc::c_short)
           };
}
// Converts an ezxml structure back to xml. Returns a string of xml data that
// must be freed.
#[no_mangle]
pub unsafe extern "C" fn ezxml_toxml(mut xml: ezxml_t) -> *mut libc::c_char {
    let mut p: ezxml_t =
        if !xml.is_null() { (*xml).parent } else { 0 as ezxml_t }; // root tag
    let mut o: ezxml_t =
        if !xml.is_null() { (*xml).ordered } else { 0 as ezxml_t };
    let mut root: ezxml_root_t = xml as ezxml_root_t;
    let mut len: size_t = 0 as libc::c_int as size_t;
    let mut max: size_t = 1024 as libc::c_int as size_t;
    let mut s: *mut libc::c_char =
        strcpy(malloc(max) as *mut libc::c_char,
               b"\x00" as *const u8 as *const libc::c_char);
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    if xml.is_null() || (*xml).name.is_null() {
        return realloc(s as *mut libc::c_void,
                       len.wrapping_add(1 as libc::c_int as libc::c_ulong)) as
                   *mut libc::c_char
    }
    while !(*root).xml.parent.is_null() {
        root = (*root).xml.parent as ezxml_root_t
    }
    i = 0 as libc::c_int;
    while p.is_null() && !(*(*root).pi.offset(i as isize)).is_null() {
        // pre-root processing instructions
        k = 2 as libc::c_int; // not pre-root
        while !(*(*(*root).pi.offset(i as
                                         isize)).offset((k - 1 as libc::c_int)
                                                            as
                                                            isize)).is_null()
              {
            k += 1
        }
        j = 1 as libc::c_int;
        loop  {
            n = *(*(*root).pi.offset(i as isize)).offset(j as isize);
            if n.is_null() { break ; }
            if !(*(*(*(*root).pi.offset(i as
                                            isize)).offset(k as
                                                               isize)).offset((j
                                                                                   -
                                                                                   1
                                                                                       as
                                                                                       libc::c_int)
                                                                                  as
                                                                                  isize)
                     as libc::c_int == '>' as i32) {
                loop  {
                    t =
                        *(*(*root).pi.offset(i as
                                                 isize)).offset(0 as
                                                                    libc::c_int
                                                                    as isize);
                    if !(len.wrapping_add(strlen(t)).wrapping_add(strlen(n)).wrapping_add(7
                                                                                              as
                                                                                              libc::c_int
                                                                                              as
                                                                                              libc::c_ulong)
                             > max) {
                        break ;
                    }
                    max =
                        (max as
                             libc::c_ulong).wrapping_add(1024 as libc::c_int
                                                             as libc::c_ulong)
                            as size_t as size_t;
                    s =
                        realloc(s as *mut libc::c_void, max) as
                            *mut libc::c_char
                }
                len =
                    (len as
                         libc::c_ulong).wrapping_add(sprintf(s.offset(len as
                                                                          isize),
                                                             b"<?%s%s%s?>\n\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char,
                                                             t,
                                                             if *n as
                                                                    libc::c_int
                                                                    != 0 {
                                                                 b" \x00" as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char
                                                             } else {
                                                                 b"\x00" as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char
                                                             }, n) as
                                                         libc::c_ulong) as
                        size_t as size_t
            }
            j += 1
        }
        i += 1
    }
    (*xml).ordered = 0 as ezxml_t;
    (*xml).parent = (*xml).ordered;
    s =
        ezxml_toxml_r(xml, &mut s, &mut len, &mut max,
                      0 as libc::c_int as size_t, (*root).attr);
    (*xml).parent = p;
    (*xml).ordered = o;
    i = 0 as libc::c_int;
    while p.is_null() && !(*(*root).pi.offset(i as isize)).is_null() {
        // post-root processing instructions
        k = 2 as libc::c_int; // not post-root
        while !(*(*(*root).pi.offset(i as
                                         isize)).offset((k - 1 as libc::c_int)
                                                            as
                                                            isize)).is_null()
              {
            k += 1
        }
        j = 1 as libc::c_int;
        loop  {
            n = *(*(*root).pi.offset(i as isize)).offset(j as isize);
            if n.is_null() { break ; }
            if !(*(*(*(*root).pi.offset(i as
                                            isize)).offset(k as
                                                               isize)).offset((j
                                                                                   -
                                                                                   1
                                                                                       as
                                                                                       libc::c_int)
                                                                                  as
                                                                                  isize)
                     as libc::c_int == '<' as i32) {
                loop  {
                    t =
                        *(*(*root).pi.offset(i as
                                                 isize)).offset(0 as
                                                                    libc::c_int
                                                                    as isize);
                    if !(len.wrapping_add(strlen(t)).wrapping_add(strlen(n)).wrapping_add(7
                                                                                              as
                                                                                              libc::c_int
                                                                                              as
                                                                                              libc::c_ulong)
                             > max) {
                        break ;
                    }
                    max =
                        (max as
                             libc::c_ulong).wrapping_add(1024 as libc::c_int
                                                             as libc::c_ulong)
                            as size_t as size_t;
                    s =
                        realloc(s as *mut libc::c_void, max) as
                            *mut libc::c_char
                }
                len =
                    (len as
                         libc::c_ulong).wrapping_add(sprintf(s.offset(len as
                                                                          isize),
                                                             b"\n<?%s%s%s?>\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char,
                                                             t,
                                                             if *n as
                                                                    libc::c_int
                                                                    != 0 {
                                                                 b" \x00" as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char
                                                             } else {
                                                                 b"\x00" as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char
                                                             }, n) as
                                                         libc::c_ulong) as
                        size_t as size_t
            }
            j += 1
        }
        i += 1
    }
    return realloc(s as *mut libc::c_void,
                   len.wrapping_add(1 as libc::c_int as libc::c_ulong)) as
               *mut libc::c_char;
}
/* ezxml.h
 *
 * Copyright 2004-2006 Aaron Voisine <aaron@voisine.org>
 *
 * Permission is hereby granted, free of charge, to any person obtaining
 * a copy of this software and associated documentation files (the
 * "Software"), to deal in the Software without restriction, including
 * without limitation the rights to use, copy, modify, merge, publish,
 * distribute, sublicense, and/or sell copies of the Software, and to
 * permit persons to whom the Software is furnished to do so, subject to
 * the following conditions:
 *
 * The above copyright notice and this permission notice shall be included
 * in all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
 * EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
 * MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
 * IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
 * CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
 * TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
 * SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */
// size of internal memory buffers
// name is malloced
// txt is malloced
// attribute name and value are strduped
// tag name
// tag attributes { name, value, name, value, ... NULL }
// tag character content, empty string if none
// tag offset from start of parent tag character content
// next tag with same name in this section at this depth
// next tag with different name in same section and depth
// next tag, same section and depth, in original order
// head of sub tag list, NULL if none
// parent tag, NULL if current tag is root tag
// additional information
// Given a string of xml data and its length, parses it and creates an ezxml
// structure. For efficiency, modifies the data by adding null terminators
// and decoding ampersand sequences. If you don't want this, copy the data and
// pass in the copy. Returns NULL on failure.
// A wrapper for ezxml_parse_str() that accepts a file descriptor. First
// attempts to mem map the file. Failing that, reads the file into memory.
// Returns NULL on failure.
// a wrapper for ezxml_parse_fd() that accepts a file name
// Wrapper for ezxml_parse_str() that accepts a file stream. Reads the entire
// stream into memory and then parses it. For xml files, use ezxml_parse_file()
// or ezxml_parse_fd()
// returns the first child tag (one level deeper) with the given name or NULL
// if not found
// returns the next tag of the same name in the same section and depth or NULL
// if not found
// Returns the Nth tag with the same name in the same section at the same depth
// or NULL if not found. An index of 0 returns the tag given.
// returns the name of the given tag
// returns the given tag's character content or empty string if none
// returns the value of the requested tag attribute, or NULL if not found
// Traverses the ezxml sturcture to retrieve a specific subtag. Takes a
// variable length list of tag names and indexes. The argument list must be
// terminated by either an index of -1 or an empty string tag name. Example: 
// title = ezxml_get(library, "shelf", 0, "book", 2, "title", -1);
// This retrieves the title of the 3rd book on the 1st shelf of library.
// Returns NULL if not found.
// Converts an ezxml structure back to xml. Returns a string of xml data that
// must be freed.
// returns a NULL terminated array of processing instructions for the given
// target
// frees the memory allocated for an ezxml structure
// free the memory allocated for the ezxml structure
#[no_mangle]
pub unsafe extern "C" fn ezxml_free(mut xml: ezxml_t) {
    let mut root: ezxml_root_t = xml as ezxml_root_t;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut a: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    if xml.is_null() { return }
    ezxml_free((*xml).child);
    ezxml_free((*xml).ordered);
    if (*xml).parent.is_null() {
        // free root tag allocations
        i = 10 as libc::c_int;
        while !(*(*root).ent.offset(i as isize)).is_null() {
            // 0 - 9 are default entites (<>&"')
            s = *(*root).ent.offset((i + 1 as libc::c_int) as isize);
            if s < (*root).s || s > (*root).e {
                free(s as *mut libc::c_void);
            }
            i += 2 as libc::c_int
        }
        // utf8 conversion
        free((*root).ent as
                 *mut libc::c_void); // free list of general entities
        i = 0 as libc::c_int;
        loop  {
            a = *(*root).attr.offset(i as isize);
            if a.is_null() { break ; }
            j = 1 as libc::c_int;
            loop  {
                let fresh46 = j;
                j = j + 1;
                if (*a.offset(fresh46 as isize)).is_null() { break ; }
                // free malloced attribute values
                if !(*a.offset(j as isize)).is_null() &&
                       (*a.offset(j as isize) < (*root).s ||
                            *a.offset(j as isize) > (*root).e) {
                    free(*a.offset(j as isize) as
                             *mut libc::c_void); // free default attribute list
                } // free processing instructions
                j += 2 as libc::c_int
            } // mem mapped xml data
            free(a as *mut libc::c_void); // malloced xml data
            i += 1
        }
        if !(*(*root).attr.offset(0 as libc::c_int as isize)).is_null() {
            free((*root).attr as *mut libc::c_void);
        }
        i = 0 as libc::c_int;
        while !(*(*root).pi.offset(i as isize)).is_null() {
            j = 1 as libc::c_int;
            while !(*(*(*root).pi.offset(i as
                                             isize)).offset(j as
                                                                isize)).is_null()
                  {
                j += 1
            }
            free(*(*(*root).pi.offset(i as
                                          isize)).offset((j +
                                                              1 as
                                                                  libc::c_int)
                                                             as isize) as
                     *mut libc::c_void);
            free(*(*root).pi.offset(i as isize) as *mut libc::c_void);
            i += 1
        }
        if !(*(*root).pi.offset(0 as libc::c_int as isize)).is_null() {
            free((*root).pi as *mut libc::c_void);
        }
        if (*root).len == -(1 as libc::c_int) as libc::c_ulong {
            free((*root).m as *mut libc::c_void);
        } else if (*root).len != 0 {
            munmap((*root).m as *mut libc::c_void, (*root).len);
        }
        if !(*root).u.is_null() { free((*root).u as *mut libc::c_void); }
    }
    // EZXML_NOMMAP
    ezxml_free_attr((*xml).attr); // tag attributes
    if (*xml).flags as libc::c_int & 0x40 as libc::c_int != 0 {
        free((*xml).txt as *mut libc::c_void); // character content
    } // tag name
    if (*xml).flags as libc::c_int & 0x80 as libc::c_int != 0 {
        free((*xml).name as *mut libc::c_void);
    }
    free(xml as *mut libc::c_void);
}
// returns parser error message or empty string if none
// return parser error message or empty string if none
#[no_mangle]
pub unsafe extern "C" fn ezxml_error(mut xml: ezxml_t)
 -> *const libc::c_char {
    while !xml.is_null() && !(*xml).parent.is_null() {
        xml = (*xml).parent
    } // find root tag
    return if !xml.is_null() {
               (*(xml as ezxml_root_t)).err.as_mut_ptr()
           } else { b"\x00" as *const u8 as *const libc::c_char };
}
// returns a new empty ezxml structure with the given root tag name
// returns a new empty ezxml structure with the given root tag name
#[no_mangle]
pub unsafe extern "C" fn ezxml_new(mut name: *const libc::c_char) -> ezxml_t {
    static mut ent: [*mut libc::c_char; 11] =
        [b"lt;\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
         b"&#60;\x00" as *const u8 as *const libc::c_char as
             *mut libc::c_char,
         b"gt;\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
         b"&#62;\x00" as *const u8 as *const libc::c_char as
             *mut libc::c_char,
         b"quot;\x00" as *const u8 as *const libc::c_char as
             *mut libc::c_char,
         b"&#34;\x00" as *const u8 as *const libc::c_char as
             *mut libc::c_char,
         b"apos;\x00" as *const u8 as *const libc::c_char as
             *mut libc::c_char,
         b"&#39;\x00" as *const u8 as *const libc::c_char as
             *mut libc::c_char,
         b"amp;\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
         b"&#38;\x00" as *const u8 as *const libc::c_char as
             *mut libc::c_char,
         0 as *const libc::c_char as *mut libc::c_char];
    let mut root: ezxml_root_t =
        memset(malloc(::std::mem::size_of::<ezxml_root>() as libc::c_ulong),
               '\u{0}' as i32,
               ::std::mem::size_of::<ezxml_root>() as libc::c_ulong) as
            ezxml_root_t;
    (*root).xml.name = name as *mut libc::c_char;
    (*root).cur = &mut (*root).xml;
    (*root).xml.txt =
        b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    strcpy((*root).err.as_mut_ptr(), (*root).xml.txt);
    (*root).ent =
        memcpy(malloc(::std::mem::size_of::<[*mut libc::c_char; 11]>() as
                          libc::c_ulong),
               ent.as_mut_ptr() as *const libc::c_void,
               ::std::mem::size_of::<[*mut libc::c_char; 11]>() as
                   libc::c_ulong) as *mut *mut libc::c_char;
    (*root).xml.attr = EZXML_NIL.as_mut_ptr();
    (*root).pi = (*root).xml.attr as *mut *mut *mut libc::c_char;
    (*root).attr = (*root).pi;
    return &mut (*root).xml;
}
// inserts an existing tag into an ezxml structure
// inserts an existing tag into an ezxml structure
#[no_mangle]
pub unsafe extern "C" fn ezxml_insert(mut xml: ezxml_t, mut dest: ezxml_t,
                                      mut off: size_t) -> ezxml_t {
    let mut cur: ezxml_t = 0 as *mut ezxml; // only sub tag
    let mut prev: ezxml_t = 0 as *mut ezxml;
    let mut head: ezxml_t = 0 as *mut ezxml;
    (*xml).ordered = 0 as ezxml_t;
    (*xml).sibling = (*xml).ordered;
    (*xml).next = (*xml).sibling;
    (*xml).off = off;
    (*xml).parent = dest;
    head = (*dest).child;
    if !head.is_null() {
        // already have sub tags
        if (*head).off <= off {
            // not first subtag
            cur = head;
            while !(*cur).ordered.is_null() && (*(*cur).ordered).off <= off {
                cur = (*cur).ordered
            }
            (*xml).ordered = (*cur).ordered;
            (*cur).ordered = xml
        } else {
            // first subtag
            (*xml).ordered = head; // find tag type
            (*dest).child = xml
        }
        cur = head;
        prev = 0 as ezxml_t;
        while !cur.is_null() && strcmp((*cur).name, (*xml).name) != 0 {
            prev = cur;
            cur = (*cur).sibling
        }
        if !cur.is_null() && (*cur).off <= off {
            // not first of type
            while !(*cur).next.is_null() && (*(*cur).next).off <= off {
                cur = (*cur).next
            }
            (*xml).next = (*cur).next;
            (*cur).next = xml
        } else {
            // first tag of this type
            if !prev.is_null() && !cur.is_null() {
                (*prev).sibling = (*cur).sibling
            } // remove old first
            (*xml).next = cur; // old first tag is now next
            cur = head; // new sibling insert point
            prev = 0 as ezxml_t;
            while !cur.is_null() && (*cur).off <= off {
                prev = cur;
                cur = (*cur).sibling
            }
            (*xml).sibling = cur;
            if !prev.is_null() { (*prev).sibling = xml }
        }
    } else { (*dest).child = xml }
    return xml;
}
// wrapper for ezxml_new() that strdup()s name
// Adds a child tag. off is the offset of the child tag relative to the start
// of the parent tag's character content. Returns the child tag.
// Adds a child tag. off is the offset of the child tag relative to the start
// of the parent tag's character content. Returns the child tag.
#[no_mangle]
pub unsafe extern "C" fn ezxml_add_child(mut xml: ezxml_t,
                                         mut name: *const libc::c_char,
                                         mut off: size_t) -> ezxml_t {
    let mut child: ezxml_t = 0 as *mut ezxml;
    if xml.is_null() { return 0 as ezxml_t }
    child =
        memset(malloc(::std::mem::size_of::<ezxml>() as libc::c_ulong),
               '\u{0}' as i32,
               ::std::mem::size_of::<ezxml>() as libc::c_ulong) as ezxml_t;
    (*child).name = name as *mut libc::c_char;
    (*child).attr = EZXML_NIL.as_mut_ptr();
    (*child).txt =
        b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    return ezxml_insert(child, xml, off);
}
// wrapper for ezxml_add_child() that strdup()s name
// sets the character content for the given tag and returns the tag
// sets the character content for the given tag and returns the tag
#[no_mangle]
pub unsafe extern "C" fn ezxml_set_txt(mut xml: ezxml_t,
                                       mut txt: *const libc::c_char)
 -> ezxml_t {
    if xml.is_null() { return 0 as ezxml_t } // existing txt was malloced
    if (*xml).flags as libc::c_int & 0x40 as libc::c_int != 0 {
        free((*xml).txt as *mut libc::c_void);
    }
    (*xml).flags =
        ((*xml).flags as libc::c_int & !(0x40 as libc::c_int)) as
            libc::c_short;
    (*xml).txt = txt as *mut libc::c_char;
    return xml;
}
// wrapper for ezxml_set_txt() that strdup()s txt
// Sets the given tag attribute or adds a new attribute if not found. A value
// of NULL will remove the specified attribute. Returns the tag given.
// Sets the given tag attribute or adds a new attribute if not found. A value
// of NULL will remove the specified attribute. Returns the tag given.
#[no_mangle]
pub unsafe extern "C" fn ezxml_set_attr(mut xml: ezxml_t,
                                        mut name: *const libc::c_char,
                                        mut value: *const libc::c_char)
 -> ezxml_t {
    let mut l: libc::c_int = 0 as libc::c_int; // name was strduped
    let mut c: libc::c_int = 0;
    if xml.is_null() { return 0 as ezxml_t }
    while !(*(*xml).attr.offset(l as isize)).is_null() &&
              strcmp(*(*xml).attr.offset(l as isize), name) != 0 {
        l += 2 as libc::c_int
    }
    if (*(*xml).attr.offset(l as isize)).is_null() {
        // not found, add as new attribute
        if value.is_null() { return xml } // nothing to do
        if (*xml).attr == EZXML_NIL.as_mut_ptr() {
            // first attribute
            (*xml).attr =
                malloc((4 as libc::c_int as
                            libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut libc::c_char>()
                                                            as libc::c_ulong))
                    as *mut *mut libc::c_char;
            let ref mut fresh47 =
                *(*xml).attr.offset(1 as libc::c_int as isize);
            *fresh47 = strdup(b"\x00" as *const u8 as *const libc::c_char)
            // empty list of malloced names/vals
        } else {
            (*xml).attr =
                realloc((*xml).attr as *mut libc::c_void,
                        ((l + 4 as libc::c_int) as
                             libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut libc::c_char>()
                                                             as
                                                             libc::c_ulong))
                    as *mut *mut libc::c_char
        } // set attribute name
        let ref mut fresh48 =
            *(*xml).attr.offset(l as isize); // null terminate attribute list
        *fresh48 =
            name as *mut libc::c_char; // set name/value as not malloced
        let ref mut fresh49 =
            *(*xml).attr.offset((l + 2 as libc::c_int) as
                                    isize); // find end of attribute list
        *fresh49 = 0 as *mut libc::c_char; //old val
        c =
            strlen(*(*xml).attr.offset((l + 1 as libc::c_int) as isize)) as
                libc::c_int; // set attribute value
        let ref mut fresh50 =
            *(*xml).attr.offset((l + 3 as libc::c_int) as isize);
        *fresh50 =
            realloc(*(*xml).attr.offset((l + 1 as libc::c_int) as isize) as
                        *mut libc::c_void,
                    (c + 2 as libc::c_int) as libc::c_ulong) as
                *mut libc::c_char;
        strcpy((*(*xml).attr.offset((l + 3 as libc::c_int) as
                                        isize)).offset(c as isize),
               b" \x00" as *const u8 as *const libc::c_char);
        if (*xml).flags as libc::c_int & 0x20 as libc::c_int != 0 {
            *(*(*xml).attr.offset((l + 3 as libc::c_int) as
                                      isize)).offset(c as isize) =
                0x80 as libc::c_int as libc::c_char
        }
    } else if (*xml).flags as libc::c_int & 0x20 as libc::c_int != 0 {
        free(name as *mut libc::c_char as *mut libc::c_void);
    }
    c = l;
    while !(*(*xml).attr.offset(c as isize)).is_null() {
        c += 2 as libc::c_int
    }
    if *(*(*xml).attr.offset((c + 1 as libc::c_int) as
                                 isize)).offset((l / 2 as libc::c_int) as
                                                    isize) as libc::c_int &
           0x40 as libc::c_int != 0 {
        free(*(*xml).attr.offset((l + 1 as libc::c_int) as isize) as
                 *mut libc::c_void);
    }
    if (*xml).flags as libc::c_int & 0x20 as libc::c_int != 0 {
        let ref mut fresh51 =
            *(*(*xml).attr.offset((c + 1 as libc::c_int) as
                                      isize)).offset((l / 2 as libc::c_int) as
                                                         isize);
        *fresh51 =
            (*fresh51 as libc::c_int | 0x40 as libc::c_int) as libc::c_char
    } else {
        let ref mut fresh52 =
            *(*(*xml).attr.offset((c + 1 as libc::c_int) as
                                      isize)).offset((l / 2 as libc::c_int) as
                                                         isize);
        *fresh52 =
            (*fresh52 as libc::c_int & !(0x40 as libc::c_int)) as libc::c_char
    }
    if !value.is_null() {
        let ref mut fresh53 =
            *(*xml).attr.offset((l + 1 as libc::c_int) as isize);
        *fresh53 = value as *mut libc::c_char
    } else {
        // remove attribute
        if *(*(*xml).attr.offset((c + 1 as libc::c_int) as
                                     isize)).offset((l / 2 as libc::c_int) as
                                                        isize) as libc::c_int
               & 0x80 as libc::c_int != 0 {
            free(*(*xml).attr.offset(l as isize) as *mut libc::c_void);
        }
        memmove((*xml).attr.offset(l as isize) as *mut libc::c_void,
                (*xml).attr.offset(l as
                                       isize).offset(2 as libc::c_int as
                                                         isize) as
                    *const libc::c_void,
                ((c - l + 2 as libc::c_int) as
                     libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut libc::c_char>()
                                                     as libc::c_ulong));
        (*xml).attr =
            realloc((*xml).attr as *mut libc::c_void,
                    ((c + 2 as libc::c_int) as
                         libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut libc::c_char>()
                                                         as libc::c_ulong)) as
                *mut *mut libc::c_char;
        memmove((*(*xml).attr.offset((c + 1 as libc::c_int) as
                                         isize)).offset((l / 2 as libc::c_int)
                                                            as isize) as
                    *mut libc::c_void,
                (*(*xml).attr.offset((c + 1 as libc::c_int) as
                                         isize)).offset((l / 2 as libc::c_int)
                                                            as
                                                            isize).offset(1 as
                                                                              libc::c_int
                                                                              as
                                                                              isize)
                    as *const libc::c_void,
                (c / 2 as libc::c_int - l / 2 as libc::c_int) as
                    libc::c_ulong);
        // fix list of which name/vals are malloced
    } // clear strdup() flag
    (*xml).flags =
        ((*xml).flags as libc::c_int & !(0x20 as libc::c_int)) as
            libc::c_short;
    return xml;
}
// Wrapper for ezxml_set_attr() that strdup()s name/value. Value cannot be NULL
// sets a flag for the given tag and returns the tag
// sets a flag for the given tag and returns the tag
#[no_mangle]
pub unsafe extern "C" fn ezxml_set_flag(mut xml: ezxml_t,
                                        mut flag: libc::c_short) -> ezxml_t {
    if !xml.is_null() {
        (*xml).flags =
            ((*xml).flags as libc::c_int | flag as libc::c_int) as
                libc::c_short
    }
    return xml;
}
// removes a tag along with its subtags without freeing its memory
// removes a tag along with its subtags without freeing its memory
#[no_mangle]
pub unsafe extern "C" fn ezxml_cut(mut xml: ezxml_t) -> ezxml_t {
    let mut cur: ezxml_t = 0 as *mut ezxml; // nothing to do
    if xml.is_null() { return 0 as ezxml_t } // patch sibling list
    if !(*xml).next.is_null() { (*(*xml).next).sibling = (*xml).sibling }
    if !(*xml).parent.is_null() {
        // not root tag
        cur = (*(*xml).parent).child; // find head of subtag list
        if cur == xml {
            (*(*xml).parent).child = (*xml).ordered
        } else { // first subtag
            // not first subtag
            while (*cur).ordered != xml { cur = (*cur).ordered }
            // patch next list
            (*cur).ordered = (*(*cur).ordered).ordered; // patch ordered list
            cur = (*(*xml).parent).child; // go back to head of subtag list
            if strcmp((*cur).name, (*xml).name) != 0 {
                // not in first sibling list
                while strcmp((*(*cur).sibling).name, (*xml).name) != 0 {
                    cur = (*cur).sibling
                }
                if (*cur).sibling == xml {
                    // not first of a sibling list
                    // first of a sibling list
                    (*cur).sibling =
                        if !(*xml).next.is_null() {
                            (*xml).next
                        } else { (*(*cur).sibling).sibling }
                } else { cur = (*cur).sibling }
            }
            while !(*cur).next.is_null() && (*cur).next != xml {
                cur = (*cur).next
            }
            if !(*cur).next.is_null() { (*cur).next = (*(*cur).next).next }
        }
    }
    (*xml).next = 0 as ezxml_t;
    (*xml).sibling = (*xml).next;
    (*xml).ordered = (*xml).sibling;
    return xml;
}
// EZXML_TEST
// test harness
