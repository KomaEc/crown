use crate::bitfields::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type sym_version;
    #[no_mangle]
    fn __errno_location() -> *mut std::os::raw::c_int;
    #[no_mangle]
    fn strlen(_: *const std::os::raw::c_char) -> std::os::raw::c_ulong;
    #[no_mangle]
    fn strstr(
        _: *const std::os::raw::c_char,
        _: *const std::os::raw::c_char,
    ) -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn strchr(_: *const std::os::raw::c_char, _: std::os::raw::c_int) -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn memset(
        _: *mut std::os::raw::c_void,
        _: std::os::raw::c_int,
        _: std::os::raw::c_ulong,
    ) -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn memcpy(
        _: *mut std::os::raw::c_void,
        _: *const std::os::raw::c_void,
        _: std::os::raw::c_ulong,
    ) -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn vfprintf(
        _: *mut FILE,
        _: *const std::os::raw::c_char,
        _: ::std::ffi::VaList,
    ) -> std::os::raw::c_int;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn fflush(__stream: *mut FILE) -> std::os::raw::c_int;
    #[no_mangle]
    static mut stdout: *mut FILE;
    #[no_mangle]
    fn exit(_: std::os::raw::c_int) -> !;
    #[no_mangle]
    fn _setjmp(_: *mut __jmp_buf_tag) -> std::os::raw::c_int;
    #[no_mangle]
    fn longjmp(_: *mut __jmp_buf_tag, _: std::os::raw::c_int) -> !;
    #[no_mangle]
    static mut environ: *mut *mut std::os::raw::c_char;
    #[no_mangle]
    fn sysconf(__name: std::os::raw::c_int) -> std::os::raw::c_long;
    #[no_mangle]
    fn tcc_add_symbol(
        s: *mut TCCState,
        name: *const std::os::raw::c_char,
        val: *const std::os::raw::c_void,
    ) -> std::os::raw::c_int;
    #[no_mangle]
    fn get_sym_addr(
        s: *mut TCCState,
        name: *const std::os::raw::c_char,
        err: std::os::raw::c_int,
        forc: std::os::raw::c_int,
    ) -> Elf64_Addr;
    #[no_mangle]
    fn pstrcpy(
        buf: *mut std::os::raw::c_char,
        buf_size: size_t,
        s: *const std::os::raw::c_char,
    ) -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn tcc_malloc(size: std::os::raw::c_ulong) -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn tcc_add_runtime(s1: *mut TCCState);
    #[no_mangle]
    fn resolve_common_syms(s1: *mut TCCState);
    #[no_mangle]
    fn build_got_entries(s1: *mut TCCState);
    #[no_mangle]
    fn relocate_syms(s1: *mut TCCState, symtab: *mut Section, do_resolve: std::os::raw::c_int);
    #[no_mangle]
    fn relocate_plt(s1: *mut TCCState);
    #[no_mangle]
    fn relocate_sections(s1: *mut TCCState);
    #[no_mangle]
    fn tcc_free(ptr: *mut std::os::raw::c_void);
    #[no_mangle]
    fn tcc_enter_state(s1: *mut TCCState);
    #[no_mangle]
    fn _tcc_error(fmt: *const std::os::raw::c_char, _: ...) -> !;
    #[no_mangle]
    fn tcc_get_symbol(
        s: *mut TCCState,
        name: *const std::os::raw::c_char,
    ) -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn dynarray_add(
        ptab: *mut std::os::raw::c_void,
        nb_ptr: *mut std::os::raw::c_int,
        data: *mut std::os::raw::c_void,
    );
    #[no_mangle]
    fn mprotect(
        __addr: *mut std::os::raw::c_void,
        __len: size_t,
        __prot: std::os::raw::c_int,
    ) -> std::os::raw::c_int;
    #[no_mangle]
    fn sigemptyset(__set: *mut sigset_t) -> std::os::raw::c_int;
    #[no_mangle]
    fn sigaction(
        __sig: std::os::raw::c_int,
        __act: *const sigaction,
        __oact: *mut sigaction,
    ) -> std::os::raw::c_int;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: std::os::raw::c_uint,
    pub fp_offset: std::os::raw::c_uint,
    pub overflow_arg_area: *mut std::os::raw::c_void,
    pub reg_save_area: *mut std::os::raw::c_void,
}
pub type va_list = __builtin_va_list;
pub type size_t = std::os::raw::c_ulong;
pub type __uint8_t = std::os::raw::c_uchar;
pub type __uint16_t = std::os::raw::c_ushort;
pub type __uint32_t = std::os::raw::c_uint;
pub type __int64_t = std::os::raw::c_long;
pub type __uint64_t = std::os::raw::c_ulong;
pub type __uid_t = std::os::raw::c_uint;
pub type __off_t = std::os::raw::c_long;
pub type __off64_t = std::os::raw::c_long;
pub type __pid_t = std::os::raw::c_int;
pub type __clock_t = std::os::raw::c_long;
pub type int64_t = __int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [std::os::raw::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
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
pub type __jmp_buf = [std::os::raw::c_long; 8];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: std::os::raw::c_int,
    pub __saved_mask: __sigset_t,
}
pub type jmp_buf = [__jmp_buf_tag; 1];
pub type __sigval_t = sigval;
#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
    pub sival_int: std::os::raw::c_int,
    pub sival_ptr: *mut std::os::raw::c_void,
}
pub type C2RustUnnamed = std::os::raw::c_uint;
pub const _SC_THREAD_ROBUST_PRIO_PROTECT: C2RustUnnamed = 248;
pub const _SC_THREAD_ROBUST_PRIO_INHERIT: C2RustUnnamed = 247;
pub const _SC_XOPEN_STREAMS: C2RustUnnamed = 246;
pub const _SC_TRACE_USER_EVENT_MAX: C2RustUnnamed = 245;
pub const _SC_TRACE_SYS_MAX: C2RustUnnamed = 244;
pub const _SC_TRACE_NAME_MAX: C2RustUnnamed = 243;
pub const _SC_TRACE_EVENT_NAME_MAX: C2RustUnnamed = 242;
pub const _SC_SS_REPL_MAX: C2RustUnnamed = 241;
pub const _SC_V7_LPBIG_OFFBIG: C2RustUnnamed = 240;
pub const _SC_V7_LP64_OFF64: C2RustUnnamed = 239;
pub const _SC_V7_ILP32_OFFBIG: C2RustUnnamed = 238;
pub const _SC_V7_ILP32_OFF32: C2RustUnnamed = 237;
pub const _SC_RAW_SOCKETS: C2RustUnnamed = 236;
pub const _SC_IPV6: C2RustUnnamed = 235;
pub const _SC_LEVEL4_CACHE_LINESIZE: C2RustUnnamed = 199;
pub const _SC_LEVEL4_CACHE_ASSOC: C2RustUnnamed = 198;
pub const _SC_LEVEL4_CACHE_SIZE: C2RustUnnamed = 197;
pub const _SC_LEVEL3_CACHE_LINESIZE: C2RustUnnamed = 196;
pub const _SC_LEVEL3_CACHE_ASSOC: C2RustUnnamed = 195;
pub const _SC_LEVEL3_CACHE_SIZE: C2RustUnnamed = 194;
pub const _SC_LEVEL2_CACHE_LINESIZE: C2RustUnnamed = 193;
pub const _SC_LEVEL2_CACHE_ASSOC: C2RustUnnamed = 192;
pub const _SC_LEVEL2_CACHE_SIZE: C2RustUnnamed = 191;
pub const _SC_LEVEL1_DCACHE_LINESIZE: C2RustUnnamed = 190;
pub const _SC_LEVEL1_DCACHE_ASSOC: C2RustUnnamed = 189;
pub const _SC_LEVEL1_DCACHE_SIZE: C2RustUnnamed = 188;
pub const _SC_LEVEL1_ICACHE_LINESIZE: C2RustUnnamed = 187;
pub const _SC_LEVEL1_ICACHE_ASSOC: C2RustUnnamed = 186;
pub const _SC_LEVEL1_ICACHE_SIZE: C2RustUnnamed = 185;
pub const _SC_TRACE_LOG: C2RustUnnamed = 184;
pub const _SC_TRACE_INHERIT: C2RustUnnamed = 183;
pub const _SC_TRACE_EVENT_FILTER: C2RustUnnamed = 182;
pub const _SC_TRACE: C2RustUnnamed = 181;
pub const _SC_HOST_NAME_MAX: C2RustUnnamed = 180;
pub const _SC_V6_LPBIG_OFFBIG: C2RustUnnamed = 179;
pub const _SC_V6_LP64_OFF64: C2RustUnnamed = 178;
pub const _SC_V6_ILP32_OFFBIG: C2RustUnnamed = 177;
pub const _SC_V6_ILP32_OFF32: C2RustUnnamed = 176;
pub const _SC_2_PBS_CHECKPOINT: C2RustUnnamed = 175;
pub const _SC_STREAMS: C2RustUnnamed = 174;
pub const _SC_SYMLOOP_MAX: C2RustUnnamed = 173;
pub const _SC_2_PBS_TRACK: C2RustUnnamed = 172;
pub const _SC_2_PBS_MESSAGE: C2RustUnnamed = 171;
pub const _SC_2_PBS_LOCATE: C2RustUnnamed = 170;
pub const _SC_2_PBS_ACCOUNTING: C2RustUnnamed = 169;
pub const _SC_2_PBS: C2RustUnnamed = 168;
pub const _SC_USER_GROUPS_R: C2RustUnnamed = 167;
pub const _SC_USER_GROUPS: C2RustUnnamed = 166;
pub const _SC_TYPED_MEMORY_OBJECTS: C2RustUnnamed = 165;
pub const _SC_TIMEOUTS: C2RustUnnamed = 164;
pub const _SC_SYSTEM_DATABASE_R: C2RustUnnamed = 163;
pub const _SC_SYSTEM_DATABASE: C2RustUnnamed = 162;
pub const _SC_THREAD_SPORADIC_SERVER: C2RustUnnamed = 161;
pub const _SC_SPORADIC_SERVER: C2RustUnnamed = 160;
pub const _SC_SPAWN: C2RustUnnamed = 159;
pub const _SC_SIGNALS: C2RustUnnamed = 158;
pub const _SC_SHELL: C2RustUnnamed = 157;
pub const _SC_REGEX_VERSION: C2RustUnnamed = 156;
pub const _SC_REGEXP: C2RustUnnamed = 155;
pub const _SC_SPIN_LOCKS: C2RustUnnamed = 154;
pub const _SC_READER_WRITER_LOCKS: C2RustUnnamed = 153;
pub const _SC_NETWORKING: C2RustUnnamed = 152;
pub const _SC_SINGLE_PROCESS: C2RustUnnamed = 151;
pub const _SC_MULTI_PROCESS: C2RustUnnamed = 150;
pub const _SC_MONOTONIC_CLOCK: C2RustUnnamed = 149;
pub const _SC_FILE_SYSTEM: C2RustUnnamed = 148;
pub const _SC_FILE_LOCKING: C2RustUnnamed = 147;
pub const _SC_FILE_ATTRIBUTES: C2RustUnnamed = 146;
pub const _SC_PIPE: C2RustUnnamed = 145;
pub const _SC_FIFO: C2RustUnnamed = 144;
pub const _SC_FD_MGMT: C2RustUnnamed = 143;
pub const _SC_DEVICE_SPECIFIC_R: C2RustUnnamed = 142;
pub const _SC_DEVICE_SPECIFIC: C2RustUnnamed = 141;
pub const _SC_DEVICE_IO: C2RustUnnamed = 140;
pub const _SC_THREAD_CPUTIME: C2RustUnnamed = 139;
pub const _SC_CPUTIME: C2RustUnnamed = 138;
pub const _SC_CLOCK_SELECTION: C2RustUnnamed = 137;
pub const _SC_C_LANG_SUPPORT_R: C2RustUnnamed = 136;
pub const _SC_C_LANG_SUPPORT: C2RustUnnamed = 135;
pub const _SC_BASE: C2RustUnnamed = 134;
pub const _SC_BARRIERS: C2RustUnnamed = 133;
pub const _SC_ADVISORY_INFO: C2RustUnnamed = 132;
pub const _SC_XOPEN_REALTIME_THREADS: C2RustUnnamed = 131;
pub const _SC_XOPEN_REALTIME: C2RustUnnamed = 130;
pub const _SC_XOPEN_LEGACY: C2RustUnnamed = 129;
pub const _SC_XBS5_LPBIG_OFFBIG: C2RustUnnamed = 128;
pub const _SC_XBS5_LP64_OFF64: C2RustUnnamed = 127;
pub const _SC_XBS5_ILP32_OFFBIG: C2RustUnnamed = 126;
pub const _SC_XBS5_ILP32_OFF32: C2RustUnnamed = 125;
pub const _SC_NL_TEXTMAX: C2RustUnnamed = 124;
pub const _SC_NL_SETMAX: C2RustUnnamed = 123;
pub const _SC_NL_NMAX: C2RustUnnamed = 122;
pub const _SC_NL_MSGMAX: C2RustUnnamed = 121;
pub const _SC_NL_LANGMAX: C2RustUnnamed = 120;
pub const _SC_NL_ARGMAX: C2RustUnnamed = 119;
pub const _SC_USHRT_MAX: C2RustUnnamed = 118;
pub const _SC_ULONG_MAX: C2RustUnnamed = 117;
pub const _SC_UINT_MAX: C2RustUnnamed = 116;
pub const _SC_UCHAR_MAX: C2RustUnnamed = 115;
pub const _SC_SHRT_MIN: C2RustUnnamed = 114;
pub const _SC_SHRT_MAX: C2RustUnnamed = 113;
pub const _SC_SCHAR_MIN: C2RustUnnamed = 112;
pub const _SC_SCHAR_MAX: C2RustUnnamed = 111;
pub const _SC_SSIZE_MAX: C2RustUnnamed = 110;
pub const _SC_NZERO: C2RustUnnamed = 109;
pub const _SC_MB_LEN_MAX: C2RustUnnamed = 108;
pub const _SC_WORD_BIT: C2RustUnnamed = 107;
pub const _SC_LONG_BIT: C2RustUnnamed = 106;
pub const _SC_INT_MIN: C2RustUnnamed = 105;
pub const _SC_INT_MAX: C2RustUnnamed = 104;
pub const _SC_CHAR_MIN: C2RustUnnamed = 103;
pub const _SC_CHAR_MAX: C2RustUnnamed = 102;
pub const _SC_CHAR_BIT: C2RustUnnamed = 101;
pub const _SC_XOPEN_XPG4: C2RustUnnamed = 100;
pub const _SC_XOPEN_XPG3: C2RustUnnamed = 99;
pub const _SC_XOPEN_XPG2: C2RustUnnamed = 98;
pub const _SC_2_UPE: C2RustUnnamed = 97;
pub const _SC_2_C_VERSION: C2RustUnnamed = 96;
pub const _SC_2_CHAR_TERM: C2RustUnnamed = 95;
pub const _SC_XOPEN_SHM: C2RustUnnamed = 94;
pub const _SC_XOPEN_ENH_I18N: C2RustUnnamed = 93;
pub const _SC_XOPEN_CRYPT: C2RustUnnamed = 92;
pub const _SC_XOPEN_UNIX: C2RustUnnamed = 91;
pub const _SC_XOPEN_XCU_VERSION: C2RustUnnamed = 90;
pub const _SC_XOPEN_VERSION: C2RustUnnamed = 89;
pub const _SC_PASS_MAX: C2RustUnnamed = 88;
pub const _SC_ATEXIT_MAX: C2RustUnnamed = 87;
pub const _SC_AVPHYS_PAGES: C2RustUnnamed = 86;
pub const _SC_PHYS_PAGES: C2RustUnnamed = 85;
pub const _SC_NPROCESSORS_ONLN: C2RustUnnamed = 84;
pub const _SC_NPROCESSORS_CONF: C2RustUnnamed = 83;
pub const _SC_THREAD_PROCESS_SHARED: C2RustUnnamed = 82;
pub const _SC_THREAD_PRIO_PROTECT: C2RustUnnamed = 81;
pub const _SC_THREAD_PRIO_INHERIT: C2RustUnnamed = 80;
pub const _SC_THREAD_PRIORITY_SCHEDULING: C2RustUnnamed = 79;
pub const _SC_THREAD_ATTR_STACKSIZE: C2RustUnnamed = 78;
pub const _SC_THREAD_ATTR_STACKADDR: C2RustUnnamed = 77;
pub const _SC_THREAD_THREADS_MAX: C2RustUnnamed = 76;
pub const _SC_THREAD_STACK_MIN: C2RustUnnamed = 75;
pub const _SC_THREAD_KEYS_MAX: C2RustUnnamed = 74;
pub const _SC_THREAD_DESTRUCTOR_ITERATIONS: C2RustUnnamed = 73;
pub const _SC_TTY_NAME_MAX: C2RustUnnamed = 72;
pub const _SC_LOGIN_NAME_MAX: C2RustUnnamed = 71;
pub const _SC_GETPW_R_SIZE_MAX: C2RustUnnamed = 70;
pub const _SC_GETGR_R_SIZE_MAX: C2RustUnnamed = 69;
pub const _SC_THREAD_SAFE_FUNCTIONS: C2RustUnnamed = 68;
pub const _SC_THREADS: C2RustUnnamed = 67;
pub const _SC_T_IOV_MAX: C2RustUnnamed = 66;
pub const _SC_PII_OSI_M: C2RustUnnamed = 65;
pub const _SC_PII_OSI_CLTS: C2RustUnnamed = 64;
pub const _SC_PII_OSI_COTS: C2RustUnnamed = 63;
pub const _SC_PII_INTERNET_DGRAM: C2RustUnnamed = 62;
pub const _SC_PII_INTERNET_STREAM: C2RustUnnamed = 61;
pub const _SC_IOV_MAX: C2RustUnnamed = 60;
pub const _SC_UIO_MAXIOV: C2RustUnnamed = 60;
pub const _SC_SELECT: C2RustUnnamed = 59;
pub const _SC_POLL: C2RustUnnamed = 58;
pub const _SC_PII_OSI: C2RustUnnamed = 57;
pub const _SC_PII_INTERNET: C2RustUnnamed = 56;
pub const _SC_PII_SOCKET: C2RustUnnamed = 55;
pub const _SC_PII_XTI: C2RustUnnamed = 54;
pub const _SC_PII: C2RustUnnamed = 53;
pub const _SC_2_LOCALEDEF: C2RustUnnamed = 52;
pub const _SC_2_SW_DEV: C2RustUnnamed = 51;
pub const _SC_2_FORT_RUN: C2RustUnnamed = 50;
pub const _SC_2_FORT_DEV: C2RustUnnamed = 49;
pub const _SC_2_C_DEV: C2RustUnnamed = 48;
pub const _SC_2_C_BIND: C2RustUnnamed = 47;
pub const _SC_2_VERSION: C2RustUnnamed = 46;
pub const _SC_CHARCLASS_NAME_MAX: C2RustUnnamed = 45;
pub const _SC_RE_DUP_MAX: C2RustUnnamed = 44;
pub const _SC_LINE_MAX: C2RustUnnamed = 43;
pub const _SC_EXPR_NEST_MAX: C2RustUnnamed = 42;
pub const _SC_EQUIV_CLASS_MAX: C2RustUnnamed = 41;
pub const _SC_COLL_WEIGHTS_MAX: C2RustUnnamed = 40;
pub const _SC_BC_STRING_MAX: C2RustUnnamed = 39;
pub const _SC_BC_SCALE_MAX: C2RustUnnamed = 38;
pub const _SC_BC_DIM_MAX: C2RustUnnamed = 37;
pub const _SC_BC_BASE_MAX: C2RustUnnamed = 36;
pub const _SC_TIMER_MAX: C2RustUnnamed = 35;
pub const _SC_SIGQUEUE_MAX: C2RustUnnamed = 34;
pub const _SC_SEM_VALUE_MAX: C2RustUnnamed = 33;
pub const _SC_SEM_NSEMS_MAX: C2RustUnnamed = 32;
pub const _SC_RTSIG_MAX: C2RustUnnamed = 31;
pub const _SC_PAGESIZE: C2RustUnnamed = 30;
pub const _SC_VERSION: C2RustUnnamed = 29;
pub const _SC_MQ_PRIO_MAX: C2RustUnnamed = 28;
pub const _SC_MQ_OPEN_MAX: C2RustUnnamed = 27;
pub const _SC_DELAYTIMER_MAX: C2RustUnnamed = 26;
pub const _SC_AIO_PRIO_DELTA_MAX: C2RustUnnamed = 25;
pub const _SC_AIO_MAX: C2RustUnnamed = 24;
pub const _SC_AIO_LISTIO_MAX: C2RustUnnamed = 23;
pub const _SC_SHARED_MEMORY_OBJECTS: C2RustUnnamed = 22;
pub const _SC_SEMAPHORES: C2RustUnnamed = 21;
pub const _SC_MESSAGE_PASSING: C2RustUnnamed = 20;
pub const _SC_MEMORY_PROTECTION: C2RustUnnamed = 19;
pub const _SC_MEMLOCK_RANGE: C2RustUnnamed = 18;
pub const _SC_MEMLOCK: C2RustUnnamed = 17;
pub const _SC_MAPPED_FILES: C2RustUnnamed = 16;
pub const _SC_FSYNC: C2RustUnnamed = 15;
pub const _SC_SYNCHRONIZED_IO: C2RustUnnamed = 14;
pub const _SC_PRIORITIZED_IO: C2RustUnnamed = 13;
pub const _SC_ASYNCHRONOUS_IO: C2RustUnnamed = 12;
pub const _SC_TIMERS: C2RustUnnamed = 11;
pub const _SC_PRIORITY_SCHEDULING: C2RustUnnamed = 10;
pub const _SC_REALTIME_SIGNALS: C2RustUnnamed = 9;
pub const _SC_SAVED_IDS: C2RustUnnamed = 8;
pub const _SC_JOB_CONTROL: C2RustUnnamed = 7;
pub const _SC_TZNAME_MAX: C2RustUnnamed = 6;
pub const _SC_STREAM_MAX: C2RustUnnamed = 5;
pub const _SC_OPEN_MAX: C2RustUnnamed = 4;
pub const _SC_NGROUPS_MAX: C2RustUnnamed = 3;
pub const _SC_CLK_TCK: C2RustUnnamed = 2;
pub const _SC_CHILD_MAX: C2RustUnnamed = 1;
pub const _SC_ARG_MAX: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TCCState {
    pub verbose: std::os::raw::c_uchar,
    pub nostdinc: std::os::raw::c_uchar,
    pub nostdlib: std::os::raw::c_uchar,
    pub nocommon: std::os::raw::c_uchar,
    pub static_link: std::os::raw::c_uchar,
    pub rdynamic: std::os::raw::c_uchar,
    pub symbolic: std::os::raw::c_uchar,
    pub filetype: std::os::raw::c_uchar,
    pub optimize: std::os::raw::c_uchar,
    pub option_pthread: std::os::raw::c_uchar,
    pub enable_new_dtags: std::os::raw::c_uchar,
    pub cversion: std::os::raw::c_uint,
    pub tcc_lib_path: *mut std::os::raw::c_char,
    pub soname: *mut std::os::raw::c_char,
    pub rpath: *mut std::os::raw::c_char,
    pub output_type: std::os::raw::c_int,
    pub output_format: std::os::raw::c_int,
    pub char_is_unsigned: std::os::raw::c_uchar,
    pub leading_underscore: std::os::raw::c_uchar,
    pub ms_extensions: std::os::raw::c_uchar,
    pub dollars_in_identifiers: std::os::raw::c_uchar,
    pub ms_bitfields: std::os::raw::c_uchar,
    pub warn_write_strings: std::os::raw::c_uchar,
    pub warn_unsupported: std::os::raw::c_uchar,
    pub warn_error: std::os::raw::c_uchar,
    pub warn_none: std::os::raw::c_uchar,
    pub warn_implicit_function_declaration: std::os::raw::c_uchar,
    pub warn_gcc_compat: std::os::raw::c_uchar,
    pub do_debug: std::os::raw::c_uchar,
    pub do_backtrace: std::os::raw::c_uchar,
    pub do_bounds_check: std::os::raw::c_uchar,
    pub test_coverage: std::os::raw::c_uchar,
    pub run_test: std::os::raw::c_int,
    pub text_addr: Elf64_Addr,
    pub has_text_addr: std::os::raw::c_uchar,
    pub section_align: std::os::raw::c_uint,
    pub gnu_ext: std::os::raw::c_uchar,
    pub tcc_ext: std::os::raw::c_uchar,
    pub init_symbol: *mut std::os::raw::c_char,
    pub fini_symbol: *mut std::os::raw::c_char,
    pub nosse: std::os::raw::c_uchar,
    pub loaded_dlls: *mut *mut DLLReference,
    pub nb_loaded_dlls: std::os::raw::c_int,
    pub include_paths: *mut *mut std::os::raw::c_char,
    pub nb_include_paths: std::os::raw::c_int,
    pub sysinclude_paths: *mut *mut std::os::raw::c_char,
    pub nb_sysinclude_paths: std::os::raw::c_int,
    pub library_paths: *mut *mut std::os::raw::c_char,
    pub nb_library_paths: std::os::raw::c_int,
    pub crt_paths: *mut *mut std::os::raw::c_char,
    pub nb_crt_paths: std::os::raw::c_int,
    pub cmdline_defs: CString,
    pub cmdline_incl: CString,
    pub error_opaque: *mut std::os::raw::c_void,
    pub error_func: Option<
        unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const std::os::raw::c_char) -> (),
    >,
    pub error_set_jmp_enabled: std::os::raw::c_int,
    pub error_jmp_buf: jmp_buf,
    pub nb_errors: std::os::raw::c_int,
    pub ppfp: *mut FILE,
    pub Pflag: C2RustUnnamed_4,
    pub dflag: std::os::raw::c_char,
    pub target_deps: *mut *mut std::os::raw::c_char,
    pub nb_target_deps: std::os::raw::c_int,
    pub include_stack: [*mut BufferedFile; 32],
    pub include_stack_ptr: *mut *mut BufferedFile,
    pub ifdef_stack: [std::os::raw::c_int; 64],
    pub ifdef_stack_ptr: *mut std::os::raw::c_int,
    pub cached_includes_hash: [std::os::raw::c_int; 32],
    pub cached_includes: *mut *mut CachedInclude,
    pub nb_cached_includes: std::os::raw::c_int,
    pub pack_stack: [std::os::raw::c_int; 8],
    pub pack_stack_ptr: *mut std::os::raw::c_int,
    pub pragma_libs: *mut *mut std::os::raw::c_char,
    pub nb_pragma_libs: std::os::raw::c_int,
    pub inline_fns: *mut *mut InlineFunc,
    pub nb_inline_fns: std::os::raw::c_int,
    pub sections: *mut *mut Section,
    pub nb_sections: std::os::raw::c_int,
    pub priv_sections: *mut *mut Section,
    pub nb_priv_sections: std::os::raw::c_int,
    pub got: *mut Section,
    pub plt: *mut Section,
    pub text_section: *mut Section,
    pub data_section: *mut Section,
    pub rodata_section: *mut Section,
    pub bss_section: *mut Section,
    pub common_section: *mut Section,
    pub cur_text_section: *mut Section,
    pub bounds_section: *mut Section,
    pub lbounds_section: *mut Section,
    pub tcov_section: *mut Section,
    pub symtab_section: *mut Section,
    pub stab_section: *mut Section,
    pub new_undef_sym: std::os::raw::c_int,
    pub dynsymtab_section: *mut Section,
    pub dynsym: *mut Section,
    pub symtab: *mut Section,
    pub sym_attrs: *mut sym_attr,
    pub nb_sym_attrs: std::os::raw::c_int,
    pub qrel: *mut Elf64_Rela,
    pub nb_sym_versions: std::os::raw::c_int,
    pub sym_versions: *mut sym_version,
    pub nb_sym_to_version: std::os::raw::c_int,
    pub sym_to_version: *mut std::os::raw::c_int,
    pub dt_verneednum: std::os::raw::c_int,
    pub versym_section: *mut Section,
    pub verneed_section: *mut Section,
    pub runtime_main: *const std::os::raw::c_char,
    pub runtime_mem: *mut *mut std::os::raw::c_void,
    pub nb_runtime_mem: std::os::raw::c_int,
    pub rt_num_callers: std::os::raw::c_int,
    pub fd: std::os::raw::c_int,
    pub cc: std::os::raw::c_int,
    pub total_idents: std::os::raw::c_int,
    pub total_lines: std::os::raw::c_int,
    pub total_bytes: std::os::raw::c_int,
    pub total_output: [std::os::raw::c_int; 4],
    pub g_debug: std::os::raw::c_int,
    pub current_filename: *const std::os::raw::c_char,
    pub files: *mut *mut filespec,
    pub nb_files: std::os::raw::c_int,
    pub nb_libraries: std::os::raw::c_int,
    pub outfile: *mut std::os::raw::c_char,
    pub option_r: std::os::raw::c_uchar,
    pub do_bench: std::os::raw::c_uchar,
    pub gen_deps: std::os::raw::c_int,
    pub deps_outfile: *mut std::os::raw::c_char,
    pub argc: std::os::raw::c_int,
    pub argv: *mut *mut std::os::raw::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct filespec {
    pub type_0: std::os::raw::c_char,
    pub name: [std::os::raw::c_char; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Section {
    pub data_offset: std::os::raw::c_ulong,
    pub data: *mut std::os::raw::c_uchar,
    pub data_allocated: std::os::raw::c_ulong,
    pub s1: *mut TCCState,
    pub sh_name: std::os::raw::c_int,
    pub sh_num: std::os::raw::c_int,
    pub sh_type: std::os::raw::c_int,
    pub sh_flags: std::os::raw::c_int,
    pub sh_info: std::os::raw::c_int,
    pub sh_addralign: std::os::raw::c_int,
    pub sh_entsize: std::os::raw::c_int,
    pub sh_size: std::os::raw::c_ulong,
    pub sh_addr: Elf64_Addr,
    pub sh_offset: std::os::raw::c_ulong,
    pub nb_hashed_syms: std::os::raw::c_int,
    pub link: *mut Section,
    pub reloc: *mut Section,
    pub hash: *mut Section,
    pub prev: *mut Section,
    pub name: [std::os::raw::c_char; 1],
}
pub type Elf64_Addr = uint64_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Elf64_Rela {
    pub r_offset: Elf64_Addr,
    pub r_info: Elf64_Xword,
    pub r_addend: Elf64_Sxword,
}
pub type Elf64_Sxword = int64_t;
pub type Elf64_Xword = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sym_attr {
    pub got_offset: std::os::raw::c_uint,
    pub plt_offset: std::os::raw::c_uint,
    pub plt_sym: std::os::raw::c_int,
    pub dyn_index: std::os::raw::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct InlineFunc {
    pub func_str: *mut TokenString,
    pub sym: *mut Sym,
    pub filename: [std::os::raw::c_char; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Sym {
    pub v: std::os::raw::c_int,
    pub r: std::os::raw::c_ushort,
    pub a: SymAttr,
    pub c2rust_unnamed: C2RustUnnamed_1,
    pub type_0: CType,
    pub c2rust_unnamed_0: C2RustUnnamed_0,
    pub prev: *mut Sym,
    pub prev_tok: *mut Sym,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub next: *mut Sym,
    pub cleanupstate: *mut Sym,
    pub asm_label: std::os::raw::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CType {
    pub t: std::os::raw::c_int,
    pub ref_0: *mut Sym,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub c2rust_unnamed: C2RustUnnamed_2,
    pub enum_val: std::os::raw::c_longlong,
    pub d: *mut std::os::raw::c_int,
    pub ncl: *mut Sym,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub c: std::os::raw::c_int,
    pub c2rust_unnamed: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub sym_scope: std::os::raw::c_int,
    pub jnext: std::os::raw::c_int,
    pub f: FuncAttr,
    pub auxtype: std::os::raw::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TokenString {
    pub str_0: *mut std::os::raw::c_int,
    pub len: std::os::raw::c_int,
    pub lastlen: std::os::raw::c_int,
    pub allocated_len: std::os::raw::c_int,
    pub last_line_num: std::os::raw::c_int,
    pub save_line_num: std::os::raw::c_int,
    pub prev: *mut TokenString,
    pub prev_ptr: *const std::os::raw::c_int,
    pub alloc: std::os::raw::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CachedInclude {
    pub ifndef_macro: std::os::raw::c_int,
    pub once: std::os::raw::c_int,
    pub hash_next: std::os::raw::c_int,
    pub filename: [std::os::raw::c_char; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BufferedFile {
    pub buf_ptr: *mut uint8_t,
    pub buf_end: *mut uint8_t,
    pub fd: std::os::raw::c_int,
    pub prev: *mut BufferedFile,
    pub line_num: std::os::raw::c_int,
    pub line_ref: std::os::raw::c_int,
    pub ifndef_macro: std::os::raw::c_int,
    pub ifndef_macro_saved: std::os::raw::c_int,
    pub ifdef_stack_ptr: *mut std::os::raw::c_int,
    pub include_next_index: std::os::raw::c_int,
    pub filename: [std::os::raw::c_char; 1024],
    pub true_filename: *mut std::os::raw::c_char,
    pub unget: [std::os::raw::c_uchar; 4],
    pub buffer: [std::os::raw::c_uchar; 1],
}
pub type uint8_t = __uint8_t;
pub type C2RustUnnamed_4 = std::os::raw::c_uint;
pub const LINE_MACRO_OUTPUT_FORMAT_P10: C2RustUnnamed_4 = 11;
pub const LINE_MACRO_OUTPUT_FORMAT_STD: C2RustUnnamed_4 = 2;
pub const LINE_MACRO_OUTPUT_FORMAT_NONE: C2RustUnnamed_4 = 1;
pub const LINE_MACRO_OUTPUT_FORMAT_GCC: C2RustUnnamed_4 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CString {
    pub size: std::os::raw::c_int,
    pub data: *mut std::os::raw::c_void,
    pub size_allocated: std::os::raw::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DLLReference {
    pub level: std::os::raw::c_int,
    pub handle: *mut std::os::raw::c_void,
    pub name: [std::os::raw::c_char; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rt_context {
    pub stab_sym: *mut Stab_Sym,
    pub stab_sym_end: *mut Stab_Sym,
    pub stab_str: *mut std::os::raw::c_char,
    pub esym_start: *mut Elf64_Sym,
    pub esym_end: *mut Elf64_Sym,
    pub elf_str: *mut std::os::raw::c_char,
    pub prog_base: Elf64_Addr,
    pub bounds_start: *mut std::os::raw::c_void,
    pub next: *mut rt_context,
    pub num_callers: std::os::raw::c_int,
    pub ip: Elf64_Addr,
    pub fp: Elf64_Addr,
    pub sp: Elf64_Addr,
    pub top_func: *mut std::os::raw::c_void,
    pub jmp_buf: jmp_buf,
    pub do_jmp: std::os::raw::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Elf64_Sym {
    pub st_name: Elf64_Word,
    pub st_info: std::os::raw::c_uchar,
    pub st_other: std::os::raw::c_uchar,
    pub st_shndx: Elf64_Section,
    pub st_value: Elf64_Addr,
    pub st_size: Elf64_Xword,
}
pub type Elf64_Section = uint16_t;
pub type uint16_t = __uint16_t;
pub type Elf64_Word = uint32_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Stab_Sym {
    pub n_strx: std::os::raw::c_uint,
    pub n_type: std::os::raw::c_uchar,
    pub n_other: std::os::raw::c_uchar,
    pub n_desc: std::os::raw::c_ushort,
    pub n_value: std::os::raw::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
    pub __sigaction_handler: C2RustUnnamed_5,
    pub sa_mask: __sigset_t,
    pub sa_flags: std::os::raw::c_int,
    pub sa_restorer: Option<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub sa_handler: __sighandler_t,
    pub sa_sigaction: Option<
        unsafe extern "C" fn(
            _: std::os::raw::c_int,
            _: *mut siginfo_t,
            _: *mut std::os::raw::c_void,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct siginfo_t {
    pub si_signo: std::os::raw::c_int,
    pub si_errno: std::os::raw::c_int,
    pub si_code: std::os::raw::c_int,
    pub __pad0: std::os::raw::c_int,
    pub _sifields: C2RustUnnamed_6,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub _pad: [std::os::raw::c_int; 28],
    pub _kill: C2RustUnnamed_15,
    pub _timer: C2RustUnnamed_14,
    pub _rt: C2RustUnnamed_13,
    pub _sigchld: C2RustUnnamed_12,
    pub _sigfault: C2RustUnnamed_9,
    pub _sigpoll: C2RustUnnamed_8,
    pub _sigsys: C2RustUnnamed_7,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub _call_addr: *mut std::os::raw::c_void,
    pub _syscall: std::os::raw::c_int,
    pub _arch: std::os::raw::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub si_band: std::os::raw::c_long,
    pub si_fd: std::os::raw::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub si_addr: *mut std::os::raw::c_void,
    pub si_addr_lsb: std::os::raw::c_short,
    pub _bounds: C2RustUnnamed_10,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
    pub _addr_bnd: C2RustUnnamed_11,
    pub _pkey: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub _lower: *mut std::os::raw::c_void,
    pub _upper: *mut std::os::raw::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: std::os::raw::c_int,
    pub si_utime: __clock_t,
    pub si_stime: __clock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
    pub si_tid: std::os::raw::c_int,
    pub si_overrun: std::os::raw::c_int,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_15 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}
pub type __sighandler_t = Option<unsafe extern "C" fn(_: std::os::raw::c_int) -> ()>;
pub const N_SOL: __stab_debug_code = 132;
pub const N_SO: __stab_debug_code = 100;
pub const N_EINCL: __stab_debug_code = 162;
pub const N_BINCL: __stab_debug_code = 130;
pub const N_SLINE: __stab_debug_code = 68;
pub const N_FUN: __stab_debug_code = 36;
pub const FPE_FLTDIV: C2RustUnnamed_16 = 3;
pub const FPE_INTDIV: C2RustUnnamed_16 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ucontext_t {
    pub uc_flags: std::os::raw::c_ulong,
    pub uc_link: *mut ucontext_t,
    pub uc_stack: stack_t,
    pub uc_mcontext: mcontext_t,
    pub uc_sigmask: sigset_t,
    pub __fpregs_mem: _libc_fpstate,
    pub __ssp: [std::os::raw::c_ulonglong; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _libc_fpstate {
    pub cwd: __uint16_t,
    pub swd: __uint16_t,
    pub ftw: __uint16_t,
    pub fop: __uint16_t,
    pub rip: __uint64_t,
    pub rdp: __uint64_t,
    pub mxcsr: __uint32_t,
    pub mxcr_mask: __uint32_t,
    pub _st: [_libc_fpxreg; 8],
    pub _xmm: [_libc_xmmreg; 16],
    pub __glibc_reserved1: [__uint32_t; 24],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _libc_xmmreg {
    pub element: [__uint32_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _libc_fpxreg {
    pub significand: [std::os::raw::c_ushort; 4],
    pub exponent: std::os::raw::c_ushort,
    pub __glibc_reserved1: [std::os::raw::c_ushort; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mcontext_t {
    pub gregs: gregset_t,
    pub fpregs: fpregset_t,
    pub __reserved1: [std::os::raw::c_ulonglong; 8],
}
pub type fpregset_t = *mut _libc_fpstate;
pub type gregset_t = [greg_t; 23];
pub type greg_t = std::os::raw::c_longlong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stack_t {
    pub ss_sp: *mut std::os::raw::c_void,
    pub ss_flags: std::os::raw::c_int,
    pub ss_size: size_t,
}
pub const REG_RBP: C2RustUnnamed_17 = 10;
pub const REG_RIP: C2RustUnnamed_17 = 16;
pub type __stab_debug_code = std::os::raw::c_uint;
pub const LAST_UNUSED_STAB_CODE: __stab_debug_code = 255;
pub const N_LENG: __stab_debug_code = 254;
pub const N_NBLCS: __stab_debug_code = 248;
pub const N_NBSTS: __stab_debug_code = 246;
pub const N_NBBSS: __stab_debug_code = 244;
pub const N_NBDATA: __stab_debug_code = 242;
pub const N_NBTEXT: __stab_debug_code = 240;
pub const N_ECOML: __stab_debug_code = 232;
pub const N_ECOMM: __stab_debug_code = 228;
pub const N_BCOMM: __stab_debug_code = 226;
pub const N_RBRAC: __stab_debug_code = 224;
pub const N_SCOPE: __stab_debug_code = 196;
pub const N_EXCL: __stab_debug_code = 194;
pub const N_LBRAC: __stab_debug_code = 192;
pub const N_ENTRY: __stab_debug_code = 164;
pub const N_PSYM: __stab_debug_code = 160;
pub const N_LSYM: __stab_debug_code = 128;
pub const N_SSYM: __stab_debug_code = 96;
pub const N_CATCH: __stab_debug_code = 84;
pub const N_MOD2: __stab_debug_code = 80;
pub const N_EHDECL: __stab_debug_code = 80;
pub const N_DEFD: __stab_debug_code = 74;
pub const N_BROWS: __stab_debug_code = 72;
pub const N_BSLINE: __stab_debug_code = 72;
pub const N_DSLINE: __stab_debug_code = 70;
pub const N_M2C: __stab_debug_code = 66;
pub const N_RSYM: __stab_debug_code = 64;
pub const N_OPT: __stab_debug_code = 60;
pub const N_OBJ: __stab_debug_code = 56;
pub const N_NOMAP: __stab_debug_code = 52;
pub const N_NSYMS: __stab_debug_code = 50;
pub const N_PC: __stab_debug_code = 48;
pub const N_MAIN: __stab_debug_code = 42;
pub const N_LCSYM: __stab_debug_code = 40;
pub const N_STSYM: __stab_debug_code = 38;
pub const N_FNAME: __stab_debug_code = 34;
pub const N_GSYM: __stab_debug_code = 32;
pub type C2RustUnnamed_16 = std::os::raw::c_uint;
pub const FPE_CONDTRAP: C2RustUnnamed_16 = 15;
pub const FPE_FLTUNK: C2RustUnnamed_16 = 14;
pub const FPE_FLTSUB: C2RustUnnamed_16 = 8;
pub const FPE_FLTINV: C2RustUnnamed_16 = 7;
pub const FPE_FLTRES: C2RustUnnamed_16 = 6;
pub const FPE_FLTUND: C2RustUnnamed_16 = 5;
pub const FPE_FLTOVF: C2RustUnnamed_16 = 4;
pub const FPE_INTOVF: C2RustUnnamed_16 = 2;
pub type C2RustUnnamed_17 = std::os::raw::c_uint;
pub const REG_CR2: C2RustUnnamed_17 = 22;
pub const REG_OLDMASK: C2RustUnnamed_17 = 21;
pub const REG_TRAPNO: C2RustUnnamed_17 = 20;
pub const REG_ERR: C2RustUnnamed_17 = 19;
pub const REG_CSGSFS: C2RustUnnamed_17 = 18;
pub const REG_EFL: C2RustUnnamed_17 = 17;
pub const REG_RSP: C2RustUnnamed_17 = 15;
pub const REG_RCX: C2RustUnnamed_17 = 14;
pub const REG_RAX: C2RustUnnamed_17 = 13;
pub const REG_RDX: C2RustUnnamed_17 = 12;
pub const REG_RBX: C2RustUnnamed_17 = 11;
pub const REG_RSI: C2RustUnnamed_17 = 9;
pub const REG_RDI: C2RustUnnamed_17 = 8;
pub const REG_R15: C2RustUnnamed_17 = 7;
pub const REG_R14: C2RustUnnamed_17 = 6;
pub const REG_R13: C2RustUnnamed_17 = 5;
pub const REG_R12: C2RustUnnamed_17 = 4;
pub const REG_R11: C2RustUnnamed_17 = 3;
pub const REG_R10: C2RustUnnamed_17 = 2;
pub const REG_R9: C2RustUnnamed_17 = 1;
pub const REG_R8: C2RustUnnamed_17 = 0;
static mut g_rtctxt: rt_context = rt_context {
    stab_sym: 0 as *const Stab_Sym as *mut Stab_Sym,
    stab_sym_end: 0 as *const Stab_Sym as *mut Stab_Sym,
    stab_str: 0 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
    esym_start: 0 as *const Elf64_Sym as *mut Elf64_Sym,
    esym_end: 0 as *const Elf64_Sym as *mut Elf64_Sym,
    elf_str: 0 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
    prog_base: 0,
    bounds_start: 0 as *const std::os::raw::c_void as *mut std::os::raw::c_void,
    next: 0 as *const rt_context as *mut rt_context,
    num_callers: 0,
    ip: 0,
    fp: 0,
    sp: 0,
    top_func: 0 as *const std::os::raw::c_void as *mut std::os::raw::c_void,
    jmp_buf: [__jmp_buf_tag {
        __jmpbuf: [0; 8],
        __mask_was_saved: 0,
        __saved_mask: __sigset_t { __val: [0; 16] },
    }; 1],
    do_jmp: 0,
};
/* ------------------------------------------------------------- */
/* Do all relocations (needed before using tcc_get_symbol())
Returns -1 on error. */
#[no_mangle]
pub unsafe extern "C" fn tcc_relocate(
    mut s1: *mut TCCState,
    mut ptr: *mut std::os::raw::c_void,
) -> std::os::raw::c_int {
    let mut size: std::os::raw::c_int = 0; /* no more errors expected */
    let mut ptr_diff: Elf64_Addr = 0 as std::os::raw::c_int as Elf64_Addr;
    if 1 as std::os::raw::c_int as *mut std::os::raw::c_void != ptr {
        return tcc_relocate_ex(s1, ptr, 0 as std::os::raw::c_int as Elf64_Addr);
    }
    size = tcc_relocate_ex(
        s1,
        0 as *mut std::os::raw::c_void,
        0 as std::os::raw::c_int as Elf64_Addr,
    );
    if size < 0 as std::os::raw::c_int {
        return -(1 as std::os::raw::c_int);
    }
    ptr = tcc_malloc(size as std::os::raw::c_ulong);
    tcc_relocate_ex(s1, ptr, ptr_diff);
    dynarray_add(
        &mut (*s1).runtime_mem as *mut *mut *mut std::os::raw::c_void as *mut std::os::raw::c_void,
        &mut (*s1).nb_runtime_mem,
        size as Elf64_Addr as *mut std::os::raw::c_void,
    );
    dynarray_add(
        &mut (*s1).runtime_mem as *mut *mut *mut std::os::raw::c_void as *mut std::os::raw::c_void,
        &mut (*s1).nb_runtime_mem,
        ptr,
    );
    return 0 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn tcc_run_free(mut s1: *mut TCCState) {
    let mut i: std::os::raw::c_int = 0;
    i = 0 as std::os::raw::c_int;
    while i < (*s1).nb_runtime_mem {
        let mut size: std::os::raw::c_uint =
            *(*s1).runtime_mem.offset(i as isize) as Elf64_Addr as std::os::raw::c_uint;
        let mut ptr: *mut std::os::raw::c_void = *(*s1)
            .runtime_mem
            .offset((i + 1 as std::os::raw::c_int) as isize);
        /* unprotect memory to make it usable for malloc again */
        set_pages_executable(
            s1,
            2 as std::os::raw::c_int,
            ptr,
            size as std::os::raw::c_ulong,
        );
        tcc_free(ptr);
        i += 2 as std::os::raw::c_int
    }
    tcc_free((*s1).runtime_mem as *mut std::os::raw::c_void);
}
unsafe extern "C" fn run_cdtors(
    mut s1: *mut TCCState,
    mut start: *const std::os::raw::c_char,
    mut end: *const std::os::raw::c_char,
    mut argc: std::os::raw::c_int,
    mut argv: *mut *mut std::os::raw::c_char,
    mut envp: *mut *mut std::os::raw::c_char,
) {
    let mut a: *mut *mut std::os::raw::c_void = get_sym_addr(
        s1,
        start,
        0 as std::os::raw::c_int,
        0 as std::os::raw::c_int,
    ) as *mut *mut std::os::raw::c_void;
    let mut b: *mut *mut std::os::raw::c_void =
        get_sym_addr(s1, end, 0 as std::os::raw::c_int, 0 as std::os::raw::c_int)
            as *mut *mut std::os::raw::c_void;
    while a != b {
        let fresh0 = a;
        a = a.offset(1);
        ::std::mem::transmute::<
            *mut std::os::raw::c_void,
            Option<
                unsafe extern "C" fn(
                    _: std::os::raw::c_int,
                    _: *mut *mut std::os::raw::c_char,
                    _: *mut *mut std::os::raw::c_char,
                ) -> (),
            >,
        >(*fresh0)
        .expect("non-null function pointer")(argc, argv, envp);
    }
}
/* launch the compiled program with the given arguments */
#[no_mangle]
pub unsafe extern "C" fn tcc_run(
    mut s1: *mut TCCState,
    mut argc: std::os::raw::c_int,
    mut argv: *mut *mut std::os::raw::c_char,
) -> std::os::raw::c_int {
    let mut prog_main: Option<
        unsafe extern "C" fn(
            _: std::os::raw::c_int,
            _: *mut *mut std::os::raw::c_char,
            _: *mut *mut std::os::raw::c_char,
        ) -> std::os::raw::c_int,
    > = None; /* clean errno value */
    let mut ret: std::os::raw::c_int = 0;
    let mut rc: *mut rt_context = &mut g_rtctxt;
    let mut envp: *mut *mut std::os::raw::c_char = environ;
    (*s1).runtime_main = if (*s1).nostdlib as std::os::raw::c_int != 0 {
        b"_start\x00" as *const u8 as *const std::os::raw::c_char
    } else {
        b"main\x00" as *const u8 as *const std::os::raw::c_char
    };
    if (*s1).dflag as std::os::raw::c_int & 16 as std::os::raw::c_int != 0
        && -(1 as std::os::raw::c_int) as Elf64_Addr
            == get_sym_addr(
                s1,
                (*s1).runtime_main,
                0 as std::os::raw::c_int,
                1 as std::os::raw::c_int,
            )
    {
        return 0 as std::os::raw::c_int;
    }
    if (*s1).do_debug != 0 {
        tcc_add_symbol(
            s1,
            b"exit\x00" as *const u8 as *const std::os::raw::c_char,
            ::std::mem::transmute::<
                Option<unsafe extern "C" fn(_: std::os::raw::c_int) -> ()>,
                *const std::os::raw::c_void,
            >(Some(
                rt_exit as unsafe extern "C" fn(_: std::os::raw::c_int) -> (),
            )),
        );
    }
    if tcc_relocate(s1, 1 as std::os::raw::c_int as *mut std::os::raw::c_void)
        < 0 as std::os::raw::c_int
    {
        return -(1 as std::os::raw::c_int);
    }
    prog_main = ::std::mem::transmute::<
        *mut std::os::raw::c_void,
        Option<
            unsafe extern "C" fn(
                _: std::os::raw::c_int,
                _: *mut *mut std::os::raw::c_char,
                _: *mut *mut std::os::raw::c_char,
            ) -> std::os::raw::c_int,
        >,
    >(get_sym_addr(
        s1,
        (*s1).runtime_main,
        1 as std::os::raw::c_int,
        1 as std::os::raw::c_int,
    ) as *mut std::os::raw::c_void);
    memset(
        rc as *mut std::os::raw::c_void,
        0 as std::os::raw::c_int,
        ::std::mem::size_of::<rt_context>() as std::os::raw::c_ulong,
    );
    if (*s1).do_debug != 0 {
        let mut p: *mut std::os::raw::c_void = 0 as *mut std::os::raw::c_void;
        (*rc).stab_sym = (*(*s1).stab_section).data as *mut Stab_Sym;
        (*rc).stab_sym_end = (*(*s1).stab_section)
            .data
            .offset((*(*s1).stab_section).data_offset as isize)
            as *mut Stab_Sym;
        (*rc).stab_str = (*(*(*s1).stab_section).link).data as *mut std::os::raw::c_char;
        (*rc).esym_start = (*(*s1).symtab_section).data as *mut Elf64_Sym;
        (*rc).esym_end = (*(*s1).symtab_section)
            .data
            .offset((*(*s1).symtab_section).data_offset as isize)
            as *mut Elf64_Sym;
        (*rc).elf_str = (*(*(*s1).symtab_section).link).data as *mut std::os::raw::c_char;
        (*rc).prog_base = ((*(*s1).text_section).sh_addr as std::os::raw::c_ulonglong
            & 0xffffffff00000000 as std::os::raw::c_ulonglong)
            as Elf64_Addr;
        (*rc).top_func =
            tcc_get_symbol(s1, b"main\x00" as *const u8 as *const std::os::raw::c_char);
        (*rc).num_callers = (*s1).rt_num_callers;
        (*rc).do_jmp = 1 as std::os::raw::c_int as std::os::raw::c_char;
        p = tcc_get_symbol(
            s1,
            b"__rt_error\x00" as *const u8 as *const std::os::raw::c_char,
        );
        if !p.is_null() {
            let ref mut fresh1 = *(p as *mut *mut std::os::raw::c_void);
            *fresh1 = ::std::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        _: *mut std::os::raw::c_void,
                        _: *mut std::os::raw::c_void,
                        _: *const std::os::raw::c_char,
                        _: ::std::ffi::VaList,
                    ) -> std::os::raw::c_int,
                >,
                *mut std::os::raw::c_void,
            >(Some(
                _rt_error
                    as unsafe extern "C" fn(
                        _: *mut std::os::raw::c_void,
                        _: *mut std::os::raw::c_void,
                        _: *const std::os::raw::c_char,
                        _: ::std::ffi::VaList,
                    ) -> std::os::raw::c_int,
            ))
        }
        if (*s1).do_bounds_check != 0 {
            (*rc).bounds_start = (*(*s1).bounds_section).sh_addr as *mut std::os::raw::c_void;
            p = tcc_get_symbol(
                s1,
                b"__bound_init\x00" as *const u8 as *const std::os::raw::c_char,
            );
            if !p.is_null() {
                ::std::mem::transmute::<
                    *mut std::os::raw::c_void,
                    Option<
                        unsafe extern "C" fn(
                            _: *mut std::os::raw::c_void,
                            _: std::os::raw::c_int,
                        ) -> (),
                    >,
                >(p)
                .expect("non-null function pointer")(
                    (*rc).bounds_start, 1 as std::os::raw::c_int
                );
            }
        }
        set_exception_handler();
    }
    *__errno_location() = 0 as std::os::raw::c_int;
    fflush(stdout);
    fflush(stderr);
    /* These aren't C symbols, so don't need leading underscore handling.  */
    run_cdtors(
        s1,
        b"__init_array_start\x00" as *const u8 as *const std::os::raw::c_char,
        b"__init_array_end\x00" as *const u8 as *const std::os::raw::c_char,
        argc,
        argv,
        envp,
    );
    if (*rc).do_jmp == 0 || {
        ret = _setjmp((*rc).jmp_buf.as_mut_ptr());
        (ret) == 0
    } {
        ret = prog_main.expect("non-null function pointer")(argc, argv, envp)
    }
    run_cdtors(
        s1,
        b"__fini_array_start\x00" as *const u8 as *const std::os::raw::c_char,
        b"__fini_array_end\x00" as *const u8 as *const std::os::raw::c_char,
        0 as std::os::raw::c_int,
        0 as *mut *mut std::os::raw::c_char,
        0 as *mut *mut std::os::raw::c_char,
    );
    if (*s1).dflag as std::os::raw::c_int & 16 as std::os::raw::c_int != 0 && ret != 0 {
        fprintf(
            (*s1).ppfp,
            b"[returns %d]\n\x00" as *const u8 as *const std::os::raw::c_char,
            ret,
        );
        fflush((*s1).ppfp);
    }
    return ret;
}
/* relocate code. Return -1 on error, required size if ptr is NULL,
otherwise copy code into buffer passed by the caller */
unsafe extern "C" fn tcc_relocate_ex(
    mut s1: *mut TCCState,
    mut ptr: *mut std::os::raw::c_void,
    mut ptr_diff: Elf64_Addr,
) -> std::os::raw::c_int {
    let mut s: *mut Section = 0 as *mut Section;
    let mut offset: std::os::raw::c_uint = 0;
    let mut length: std::os::raw::c_uint = 0;
    let mut align: std::os::raw::c_uint = 0;
    let mut max_align: std::os::raw::c_uint = 0;
    let mut i: std::os::raw::c_uint = 0;
    let mut k: std::os::raw::c_uint = 0;
    let mut f: std::os::raw::c_uint = 0;
    let mut n: std::os::raw::c_uint = 0;
    let mut copy: std::os::raw::c_uint = 0;
    let mut mem: Elf64_Addr = 0;
    let mut addr: Elf64_Addr = 0;
    if ptr.is_null() {
        (*s1).nb_errors = 0 as std::os::raw::c_int;
        tcc_add_runtime(s1);
        resolve_common_syms(s1);
        build_got_entries(s1);
        if (*s1).nb_errors != 0 {
            return -(1 as std::os::raw::c_int);
        }
    }
    max_align = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    offset = max_align;
    mem = ptr as Elf64_Addr;
    copy = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    loop {
        k = 0 as std::os::raw::c_int as std::os::raw::c_uint;
        while k < 3 as std::os::raw::c_int as std::os::raw::c_uint {
            /* 0:rx, 1:ro, 2:rw sections */
            n = 0 as std::os::raw::c_int as std::os::raw::c_uint;
            addr = 0 as std::os::raw::c_int as Elf64_Addr;
            i = 1 as std::os::raw::c_int as std::os::raw::c_uint;
            while i < (*s1).nb_sections as std::os::raw::c_uint {
                static mut shf: [std::os::raw::c_char; 3] = [
                    ((1 as std::os::raw::c_int) << 1 as std::os::raw::c_int
                        | (1 as std::os::raw::c_int) << 2 as std::os::raw::c_int)
                        as std::os::raw::c_char,
                    ((1 as std::os::raw::c_int) << 1 as std::os::raw::c_int)
                        as std::os::raw::c_char,
                    ((1 as std::os::raw::c_int) << 1 as std::os::raw::c_int
                        | (1 as std::os::raw::c_int) << 0 as std::os::raw::c_int)
                        as std::os::raw::c_char,
                ];
                s = *(*s1).sections.offset(i as isize);
                if !(shf[k as usize] as std::os::raw::c_int
                    != (*s).sh_flags
                        & ((1 as std::os::raw::c_int) << 1 as std::os::raw::c_int
                            | (1 as std::os::raw::c_int) << 0 as std::os::raw::c_int
                            | (1 as std::os::raw::c_int) << 2 as std::os::raw::c_int))
                {
                    length = (*s).data_offset as std::os::raw::c_uint;
                    if copy != 0 {
                        if addr == 0 as std::os::raw::c_int as std::os::raw::c_ulong {
                            addr = (*s).sh_addr
                        }
                        n = (*s)
                            .sh_addr
                            .wrapping_sub(addr)
                            .wrapping_add(length as std::os::raw::c_ulong)
                            as std::os::raw::c_uint;
                        ptr = (*s).sh_addr as *mut std::os::raw::c_void;
                        if k == 0 as std::os::raw::c_int as std::os::raw::c_uint {
                            ptr = (*s).sh_addr.wrapping_sub(ptr_diff) as *mut std::os::raw::c_void
                        }
                        if (*s).data.is_null() || (*s).sh_type == 8 as std::os::raw::c_int {
                            memset(
                                ptr,
                                0 as std::os::raw::c_int,
                                length as std::os::raw::c_ulong,
                            );
                        } else {
                            memcpy(
                                ptr,
                                (*s).data as *const std::os::raw::c_void,
                                length as std::os::raw::c_ulong,
                            );
                        }
                        if !(*s).data.is_null() {
                            tcc_free((*s).data as *mut std::os::raw::c_void);
                            (*s).data = 0 as *mut std::os::raw::c_uchar;
                            (*s).data_allocated = 0 as std::os::raw::c_int as std::os::raw::c_ulong
                        }
                        (*s).data_offset = 0 as std::os::raw::c_int as std::os::raw::c_ulong
                    } else {
                        align =
                            ((*s).sh_addralign - 1 as std::os::raw::c_int) as std::os::raw::c_uint;
                        n = n.wrapping_add(1);
                        if n == 1 as std::os::raw::c_int as std::os::raw::c_uint
                            && (align as std::os::raw::c_long)
                                < sysconf(_SC_PAGESIZE as std::os::raw::c_int)
                                    - 1 as std::os::raw::c_int as std::os::raw::c_long
                        {
                            align = (sysconf(_SC_PAGESIZE as std::os::raw::c_int)
                                - 1 as std::os::raw::c_int as std::os::raw::c_long)
                                as std::os::raw::c_uint
                        }
                        if max_align < align {
                            max_align = align
                        }
                        addr = if k != 0 {
                            mem
                        } else {
                            mem.wrapping_add(ptr_diff)
                        };
                        offset = (offset as std::os::raw::c_ulong).wrapping_add(
                            addr.wrapping_add(offset as std::os::raw::c_ulong)
                                .wrapping_neg()
                                & align as std::os::raw::c_ulong,
                        ) as std::os::raw::c_uint
                            as std::os::raw::c_uint;
                        (*s).sh_addr = if mem != 0 {
                            addr.wrapping_add(offset as std::os::raw::c_ulong)
                        } else {
                            0 as std::os::raw::c_int as std::os::raw::c_ulong
                        };
                        offset = offset.wrapping_add(length)
                    }
                }
                i = i.wrapping_add(1)
            }
            if copy != 0 {
                /* set permissions */
                if !(k == 0 as std::os::raw::c_int as std::os::raw::c_uint && ptr_diff != 0) {
                    f = k; /* not with HAVE_SELINUX */
                    if n != 0 {
                        set_pages_executable(
                            s1,
                            f as std::os::raw::c_int,
                            addr as *mut std::os::raw::c_void,
                            n as std::os::raw::c_ulong,
                        );
                    }
                }
            }
            k = k.wrapping_add(1)
        }
        if copy != 0 {
            return 0 as std::os::raw::c_int;
        }
        /* relocate symbols */
        relocate_syms(
            s1,
            (*s1).symtab,
            ((*s1).nostdlib == 0) as std::os::raw::c_int,
        );
        if (*s1).nb_errors != 0 {
            return -(1 as std::os::raw::c_int);
        }
        if 0 as std::os::raw::c_int as std::os::raw::c_ulong == mem {
            return offset.wrapping_add(max_align) as std::os::raw::c_int;
        }
        /* relocate sections */
        relocate_plt(s1);
        relocate_sections(s1);
        copy = 1 as std::os::raw::c_int as std::os::raw::c_uint
    }
}
/* CONFIG_TCC_BACKTRACE */
/* defined when included from lib/bt-exe.c */
/* ------------------------------------------------------------- */
/* allow to run code in memory */
unsafe extern "C" fn set_pages_executable(
    mut s1: *mut TCCState,
    mut mode: std::os::raw::c_int,
    mut ptr: *mut std::os::raw::c_void,
    mut length: std::os::raw::c_ulong,
) {
    static mut protect: [std::os::raw::c_uchar; 4] = [
        (0x1 as std::os::raw::c_int | 0x4 as std::os::raw::c_int) as std::os::raw::c_uchar,
        0x1 as std::os::raw::c_int as std::os::raw::c_uchar,
        (0x1 as std::os::raw::c_int | 0x2 as std::os::raw::c_int) as std::os::raw::c_uchar,
        (0x1 as std::os::raw::c_int | 0x2 as std::os::raw::c_int | 0x4 as std::os::raw::c_int)
            as std::os::raw::c_uchar,
    ];
    let mut start: Elf64_Addr = 0;
    let mut end: Elf64_Addr = 0;
    start = ptr as Elf64_Addr
        & !(sysconf(_SC_PAGESIZE as std::os::raw::c_int)
            - 1 as std::os::raw::c_int as std::os::raw::c_long) as std::os::raw::c_ulong;
    end = (ptr as Elf64_Addr).wrapping_add(length);
    end = end
        .wrapping_add(sysconf(_SC_PAGESIZE as std::os::raw::c_int) as std::os::raw::c_ulong)
        .wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong)
        & !(sysconf(_SC_PAGESIZE as std::os::raw::c_int)
            - 1 as std::os::raw::c_int as std::os::raw::c_long) as std::os::raw::c_ulong;
    if mprotect(
        start as *mut std::os::raw::c_void,
        end.wrapping_sub(start),
        protect[mode as usize] as std::os::raw::c_int,
    ) != 0
    {
        tcc_enter_state(s1);
        Some(_tcc_error as unsafe extern "C" fn(_: *const std::os::raw::c_char, _: ...) -> !)
            .expect("non-null function pointer")(
            b"mprotect failed: did you mean to configure --with-selinux?\x00" as *const u8
                as *const std::os::raw::c_char,
        );
    };
    /* XXX: BSD sometimes dump core with bad system call */
}
//ndef CONFIG_TCC_BACKTRACE_ONLY
/* ------------------------------------------------------------- */
unsafe extern "C" fn rt_vprintf(
    mut fmt: *const std::os::raw::c_char,
    mut ap: ::std::ffi::VaList,
) -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = vfprintf(stderr, fmt, ap.as_va_list());
    fflush(stderr);
    return ret;
}
unsafe extern "C" fn rt_printf(
    mut fmt: *const std::os::raw::c_char,
    mut args: ...
) -> std::os::raw::c_int {
    let mut ap: ::std::ffi::VaListImpl;
    let mut r: std::os::raw::c_int = 0;
    ap = args.clone();
    r = rt_vprintf(fmt, ap.as_va_list());
    return r;
}
/* print the position in the source file of PC value 'pc' by reading
the stabs debug information */
unsafe extern "C" fn rt_printline(
    mut rc: *mut rt_context,
    mut wanted_pc: Elf64_Addr,
    mut msg: *const std::os::raw::c_char,
    mut skip: *const std::os::raw::c_char,
) -> Elf64_Addr {
    let mut current_block: u64;
    let mut func_name: [std::os::raw::c_char; 128] = [0; 128];
    let mut func_addr: Elf64_Addr = 0;
    let mut last_pc: Elf64_Addr = 0;
    let mut pc: Elf64_Addr = 0;
    let mut incl_files: [*const std::os::raw::c_char; 32] = [0 as *const std::os::raw::c_char; 32];
    let mut incl_index: std::os::raw::c_int = 0;
    let mut last_incl_index: std::os::raw::c_int = 0;
    let mut len: std::os::raw::c_int = 0;
    let mut last_line_num: std::os::raw::c_int = 0;
    let mut i: std::os::raw::c_int = 0;
    let mut str: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut p: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut esym: *mut Elf64_Sym = 0 as *mut Elf64_Sym;
    let mut sym: *mut Stab_Sym = 0 as *mut Stab_Sym;
    'c_8446: loop {
        func_name[0 as std::os::raw::c_int as usize] = '\u{0}' as i32 as std::os::raw::c_char;
        func_addr = 0 as std::os::raw::c_int as Elf64_Addr;
        incl_index = 0 as std::os::raw::c_int;
        last_pc = -(1 as std::os::raw::c_int) as Elf64_Addr;
        last_line_num = 1 as std::os::raw::c_int;
        last_incl_index = 0 as std::os::raw::c_int;
        sym = (*rc).stab_sym.offset(1 as std::os::raw::c_int as isize);
        while sym < (*rc).stab_sym_end {
            str = (*rc).stab_str.offset((*sym).n_strx as isize);
            pc = (*sym).n_value as Elf64_Addr;
            match (*sym).n_type as std::os::raw::c_int {
                68 => {
                    if func_addr != 0 {
                        current_block = 8096298871650922110;
                    } else {
                        current_block = 2455223256940590701;
                    }
                },
                100 | 132 => {
                    current_block = 2455223256940590701;
                },
                36 => {
                    if (*sym).n_strx == 0 as std::os::raw::c_int as std::os::raw::c_uint {
                        current_block = 8096298871650922110;
                    } else {
                        current_block = 2455223256940590701;
                    }
                },
                _ => {
                    current_block = 14401909646449704462;
                },
            }
            match current_block {
                2455223256940590701 => {
                    /* Stab_Sym.n_value is only 32bits */
                    pc = (pc as std::os::raw::c_ulong).wrapping_add((*rc).prog_base) as Elf64_Addr
                        as Elf64_Addr;
                    current_block = 14981113974269438036;
                },
                8096298871650922110 =>
                /* end of function */
                {
                    pc = (pc as std::os::raw::c_ulong).wrapping_add(func_addr) as Elf64_Addr
                        as Elf64_Addr;
                    current_block = 14981113974269438036;
                }
                _ => {},
            }
            match current_block {
                14981113974269438036 => {
                    if pc >= wanted_pc && wanted_pc >= last_pc {
                        break 'c_8446;
                    }
                },
                _ => {},
            }
            let mut current_block_33: u64;
            match (*sym).n_type as std::os::raw::c_int {
                36 => {
                    /* function start or end */
                    if (*sym).n_strx == 0 as std::os::raw::c_int as std::os::raw::c_uint {
                        current_block_33 = 16190497586199869627;
                    } else {
                        p = strchr(str, ':' as i32);
                        if p.is_null() || {
                            len = (p.offset_from(str) as std::os::raw::c_long
                                + 1 as std::os::raw::c_int as std::os::raw::c_long)
                                as std::os::raw::c_int;
                            (len as std::os::raw::c_ulong)
                                > ::std::mem::size_of::<[std::os::raw::c_char; 128]>()
                                    as std::os::raw::c_ulong
                        } {
                            len = ::std::mem::size_of::<[std::os::raw::c_char; 128]>()
                                as std::os::raw::c_ulong
                                as std::os::raw::c_int
                        }
                        pstrcpy(func_name.as_mut_ptr(), len as size_t, str);
                        func_addr = pc;
                        current_block_33 = 14945149239039849694;
                    }
                },
                68 => {
                    /* line number info */
                    last_pc = pc;
                    last_line_num = (*sym).n_desc as std::os::raw::c_int;
                    last_incl_index = incl_index;
                    current_block_33 = 14945149239039849694;
                },
                130 => {
                    /* include files */
                    if incl_index < 32 as std::os::raw::c_int {
                        let fresh2 = incl_index;
                        incl_index = incl_index + 1;
                        incl_files[fresh2 as usize] = str
                    }
                    current_block_33 = 14945149239039849694;
                },
                162 => {
                    if incl_index > 1 as std::os::raw::c_int {
                        incl_index -= 1
                    }
                    current_block_33 = 14945149239039849694;
                },
                100 => {
                    /* start/end of translation unit */
                    incl_index = 0 as std::os::raw::c_int;
                    if (*sym).n_strx != 0 {
                        /* do not add path */
                        len = strlen(str) as std::os::raw::c_int;
                        if len > 0 as std::os::raw::c_int
                            && *str.offset((len - 1 as std::os::raw::c_int) as isize)
                                as std::os::raw::c_int
                                != '/' as i32
                        {
                            let fresh3 = incl_index;
                            incl_index = incl_index + 1;
                            incl_files[fresh3 as usize] = str
                        }
                    }
                    current_block_33 = 16190497586199869627;
                },
                132 => {
                    /* alternative file name (from #line or #include directives) */
                    if incl_index != 0 {
                        incl_files[(incl_index - 1 as std::os::raw::c_int) as usize] = str
                    }
                    current_block_33 = 14945149239039849694;
                },
                _ => {
                    current_block_33 = 14945149239039849694;
                },
            }
            match current_block_33 {
                16190497586199869627 => {
                    func_name[0 as std::os::raw::c_int as usize] =
                        '\u{0}' as i32 as std::os::raw::c_char;
                    func_addr = 0 as std::os::raw::c_int as Elf64_Addr;
                    last_pc = -(1 as std::os::raw::c_int) as Elf64_Addr
                },
                _ => {},
            }
            sym = sym.offset(1)
        }
        func_name[0 as std::os::raw::c_int as usize] = '\u{0}' as i32 as std::os::raw::c_char;
        func_addr = 0 as std::os::raw::c_int as Elf64_Addr;
        last_incl_index = 0 as std::os::raw::c_int;
        /* we try symtab symbols (no line number info) */
        esym = (*rc).esym_start.offset(1 as std::os::raw::c_int as isize);
        while esym < (*rc).esym_end {
            let mut type_0: std::os::raw::c_int =
                (*esym).st_info as std::os::raw::c_int & 0xf as std::os::raw::c_int;
            if type_0 == 2 as std::os::raw::c_int || type_0 == 10 as std::os::raw::c_int {
                if wanted_pc >= (*esym).st_value
                    && wanted_pc < (*esym).st_value.wrapping_add((*esym).st_size)
                {
                    pstrcpy(
                        func_name.as_mut_ptr(),
                        ::std::mem::size_of::<[std::os::raw::c_char; 128]>()
                            as std::os::raw::c_ulong,
                        (*rc).elf_str.offset((*esym).st_name as isize),
                    );
                    func_addr = (*esym).st_value;
                    break 'c_8446;
                }
            }
            esym = esym.offset(1)
        }
        rc = (*rc).next;
        if rc.is_null() {
            break;
        }
    }
    i = last_incl_index;
    if i > 0 as std::os::raw::c_int {
        i -= 1;
        str = incl_files[i as usize];
        if *skip.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int != 0
            && !strstr(str, skip).is_null()
        {
            return -(1 as std::os::raw::c_int) as Elf64_Addr;
        }
        rt_printf(
            b"%s:%d: \x00" as *const u8 as *const std::os::raw::c_char,
            str,
            last_line_num,
        );
    } else {
        rt_printf(
            b"%08llx : \x00" as *const u8 as *const std::os::raw::c_char,
            wanted_pc as std::os::raw::c_longlong,
        );
    }
    rt_printf(
        b"%s %s\x00" as *const u8 as *const std::os::raw::c_char,
        msg,
        if func_name[0 as std::os::raw::c_int as usize] as std::os::raw::c_int != 0 {
            func_name.as_mut_ptr() as *const std::os::raw::c_char
        } else {
            b"???\x00" as *const u8 as *const std::os::raw::c_char
        },
    );
    return func_addr;
}
unsafe extern "C" fn _rt_error(
    mut fp: *mut std::os::raw::c_void,
    mut ip: *mut std::os::raw::c_void,
    mut fmt: *const std::os::raw::c_char,
    mut ap: ::std::ffi::VaList,
) -> std::os::raw::c_int {
    let mut rc: *mut rt_context = &mut g_rtctxt;
    let mut pc: Elf64_Addr = 0 as std::os::raw::c_int as Elf64_Addr;
    let mut skip: [std::os::raw::c_char; 100] = [0; 100];
    let mut i: std::os::raw::c_int = 0;
    let mut level: std::os::raw::c_int = 0;
    let mut ret: std::os::raw::c_int = 0;
    let mut n: std::os::raw::c_int = 0;
    let mut a: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut b: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut msg: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    if !fp.is_null() {
        /* we're called from tcc_backtrace. */
        (*rc).fp = fp as Elf64_Addr;
        (*rc).ip = ip as Elf64_Addr;
        msg = b"\x00" as *const u8 as *const std::os::raw::c_char
    } else {
        /* we're called from signal/exception handler */
        msg = b"RUNTIME ERROR: \x00" as *const u8 as *const std::os::raw::c_char
    }
    skip[0 as std::os::raw::c_int as usize] = 0 as std::os::raw::c_int as std::os::raw::c_char;
    /* If fmt is like "^file.c^..." then skip calls from 'file.c' */
    if *fmt.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int == '^' as i32 && {
        a = fmt.offset(1 as std::os::raw::c_int as isize);
        b = strchr(
            a,
            *fmt.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int,
        );
        !b.is_null()
    } {
        memcpy(
            skip.as_mut_ptr() as *mut std::os::raw::c_void,
            a as *const std::os::raw::c_void,
            b.offset_from(a) as std::os::raw::c_long as std::os::raw::c_ulong,
        );
        skip[b.offset_from(a) as std::os::raw::c_long as usize] =
            0 as std::os::raw::c_int as std::os::raw::c_char;
        fmt = b.offset(1 as std::os::raw::c_int as isize)
    }
    n = if (*rc).num_callers != 0 {
        (*rc).num_callers
    } else {
        6 as std::os::raw::c_int
    };
    let mut current_block_22: u64;
    level = 0 as std::os::raw::c_int;
    i = level;
    while level < n {
        ret = rt_get_caller_pc(&mut pc, rc, i);
        a = b"%s\x00" as *const u8 as *const std::os::raw::c_char;
        if ret != -(1 as std::os::raw::c_int) {
            pc = rt_printline(
                rc,
                pc,
                if level != 0 {
                    b"by\x00" as *const u8 as *const std::os::raw::c_char
                } else {
                    b"at\x00" as *const u8 as *const std::os::raw::c_char
                },
                skip.as_mut_ptr(),
            );
            if pc == -(1 as std::os::raw::c_int) as Elf64_Addr {
                current_block_22 = 7976072742316086414;
            } else {
                a = b": %s\x00" as *const u8 as *const std::os::raw::c_char;
                current_block_22 = 11042950489265723346;
            }
        } else {
            current_block_22 = 11042950489265723346;
        }
        match current_block_22 {
            11042950489265723346 => {
                if level == 0 as std::os::raw::c_int {
                    rt_printf(a, msg);
                    rt_vprintf(fmt, ap.as_va_list());
                } else if ret == -(1 as std::os::raw::c_int) {
                    break;
                }
                rt_printf(b"\n\x00" as *const u8 as *const std::os::raw::c_char);
                if ret == -(1 as std::os::raw::c_int)
                    || pc == (*rc).top_func as Elf64_Addr && pc != 0
                {
                    break;
                }
                level += 1
            },
            _ => {},
        }
        i += 1
    }
    (*rc).fp = 0 as std::os::raw::c_int as Elf64_Addr;
    (*rc).ip = (*rc).fp;
    return 0 as std::os::raw::c_int;
}
/* emit a run time error at position 'pc' */
unsafe extern "C" fn rt_error(
    mut fmt: *const std::os::raw::c_char,
    mut args: ...
) -> std::os::raw::c_int {
    let mut ap: ::std::ffi::VaListImpl;
    let mut ret: std::os::raw::c_int = 0;
    ap = args.clone();
    ret = _rt_error(
        0 as *mut std::os::raw::c_void,
        0 as *mut std::os::raw::c_void,
        fmt,
        ap.as_va_list(),
    );
    return ret;
}
unsafe extern "C" fn rt_exit(mut code: std::os::raw::c_int) {
    let mut rc: *mut rt_context = &mut g_rtctxt;
    if (*rc).do_jmp != 0 {
        longjmp(
            (*rc).jmp_buf.as_mut_ptr(),
            if code != 0 {
                code
            } else {
                256 as std::os::raw::c_int
            },
        );
    }
    exit(code);
}
/* ------------------------------------------------------------- */
/* translate from ucontext_t* to internal rt_context * */
unsafe extern "C" fn rt_getcontext(mut uc: *mut ucontext_t, mut rc: *mut rt_context) {
    (*rc).ip = (*uc).uc_mcontext.gregs[REG_RIP as std::os::raw::c_int as usize] as Elf64_Addr;
    (*rc).fp = (*uc).uc_mcontext.gregs[REG_RBP as std::os::raw::c_int as usize] as Elf64_Addr;
}
/* ------------------------------------------------------------- */
/* signal handler for fatal errors */
unsafe extern "C" fn sig_error(
    mut signum: std::os::raw::c_int,
    mut siginf: *mut siginfo_t,
    mut puc: *mut std::os::raw::c_void,
) {
    let mut rc: *mut rt_context = &mut g_rtctxt;
    rt_getcontext(puc as *mut ucontext_t, rc);
    match signum {
        8 => match (*siginf).si_code {
            1 | 3 => {
                rt_error(b"division by zero\x00" as *const u8 as *const std::os::raw::c_char);
            },
            _ => {
                rt_error(
                    b"floating point exception\x00" as *const u8 as *const std::os::raw::c_char,
                );
            },
        },
        7 | 11 => {
            rt_error(b"invalid memory access\x00" as *const u8 as *const std::os::raw::c_char);
        },
        4 => {
            rt_error(b"illegal instruction\x00" as *const u8 as *const std::os::raw::c_char);
        },
        6 => {
            rt_error(b"abort() called\x00" as *const u8 as *const std::os::raw::c_char);
        },
        _ => {
            rt_error(
                b"caught signal %d\x00" as *const u8 as *const std::os::raw::c_char,
                signum,
            );
        },
    }
    rt_exit(255 as std::os::raw::c_int);
}
/* Generate a stack backtrace when a CPU exception occurs. */
unsafe extern "C" fn set_exception_handler() {
    let mut sigact: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed_5 { sa_handler: None },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    /* install TCC signal handlers to print debug info on fatal
    runtime errors */
    sigemptyset(&mut sigact.sa_mask);
    sigact.sa_flags = (4 as std::os::raw::c_int as std::os::raw::c_uint
        | 0x80000000 as std::os::raw::c_uint) as std::os::raw::c_int;
    //def SIGSTKSZ // this causes signals not to work at all on some (older) linuxes
    sigact.__sigaction_handler.sa_sigaction = Some(
        sig_error
            as unsafe extern "C" fn(
                _: std::os::raw::c_int,
                _: *mut siginfo_t,
                _: *mut std::os::raw::c_void,
            ) -> (),
    );
    sigemptyset(&mut sigact.sa_mask);
    sigaction(8 as std::os::raw::c_int, &mut sigact, 0 as *mut sigaction);
    sigaction(4 as std::os::raw::c_int, &mut sigact, 0 as *mut sigaction);
    sigaction(11 as std::os::raw::c_int, &mut sigact, 0 as *mut sigaction);
    sigaction(7 as std::os::raw::c_int, &mut sigact, 0 as *mut sigaction);
    sigaction(6 as std::os::raw::c_int, &mut sigact, 0 as *mut sigaction);
    //def SIGSTKSZ
}
/* WIN32 */
/* ------------------------------------------------------------- */
/* return the PC at frame level 'level'. Return negative if not found */
unsafe extern "C" fn rt_get_caller_pc(
    mut paddr: *mut Elf64_Addr,
    mut rc: *mut rt_context,
    mut level: std::os::raw::c_int,
) -> std::os::raw::c_int {
    let mut ip: Elf64_Addr = 0;
    let mut fp: Elf64_Addr = 0;
    if level == 0 as std::os::raw::c_int {
        ip = (*rc).ip
    } else {
        ip = 0 as std::os::raw::c_int as Elf64_Addr;
        fp = (*rc).fp;
        loop {
            level -= 1;
            if !(level != 0) {
                break;
            }
            /* XXX: check address validity with program info */
            if fp <= 0x1000 as std::os::raw::c_int as std::os::raw::c_ulong {
                break;
            }
            fp = *(fp as *mut Elf64_Addr).offset(0 as std::os::raw::c_int as isize)
        }
        if fp > 0x1000 as std::os::raw::c_int as std::os::raw::c_ulong {
            ip = *(fp as *mut Elf64_Addr).offset(1 as std::os::raw::c_int as isize)
        }
    }
    if ip <= 0x1000 as std::os::raw::c_int as std::os::raw::c_ulong {
        return -(1 as std::os::raw::c_int);
    }
    *paddr = ip;
    return 0 as std::os::raw::c_int;
}
/* ------------------------------------------------------------- */
/* TCC_IS_NATIVE */
/* CONFIG_TCC_STATIC */
/* ------------------------------------------------------------- */
/* CONFIG_TCC_BACKTRACE */
