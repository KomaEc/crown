use crate::bitfields::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    fn getenv(__name: *const std::os::raw::c_char) -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> std::os::raw::c_int;
    #[no_mangle]
    fn snprintf(
        _: *mut std::os::raw::c_char,
        _: std::os::raw::c_ulong,
        _: *const std::os::raw::c_char,
        _: ...
    ) -> std::os::raw::c_int;
    #[no_mangle]
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    fn fputc(__c: std::os::raw::c_int, __stream: *mut FILE) -> std::os::raw::c_int;
    #[no_mangle]
    fn fwrite(
        _: *const std::os::raw::c_void,
        _: std::os::raw::c_ulong,
        _: std::os::raw::c_ulong,
        _: *mut FILE,
    ) -> std::os::raw::c_ulong;
    #[no_mangle]
    fn fdopen(__fd: std::os::raw::c_int, __modes: *const std::os::raw::c_char) -> *mut FILE;
    #[no_mangle]
    fn memcpy(
        _: *mut std::os::raw::c_void,
        _: *const std::os::raw::c_void,
        _: std::os::raw::c_ulong,
    ) -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn memmove(
        _: *mut std::os::raw::c_void,
        _: *const std::os::raw::c_void,
        _: std::os::raw::c_ulong,
    ) -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn memset(
        _: *mut std::os::raw::c_void,
        _: std::os::raw::c_int,
        _: std::os::raw::c_ulong,
    ) -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn memcmp(
        _: *const std::os::raw::c_void,
        _: *const std::os::raw::c_void,
        _: std::os::raw::c_ulong,
    ) -> std::os::raw::c_int;
    #[no_mangle]
    fn strcpy(
        _: *mut std::os::raw::c_char,
        _: *const std::os::raw::c_char,
    ) -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn strncpy(
        _: *mut std::os::raw::c_char,
        _: *const std::os::raw::c_char,
        _: std::os::raw::c_ulong,
    ) -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn strcmp(
        _: *const std::os::raw::c_char,
        _: *const std::os::raw::c_char,
    ) -> std::os::raw::c_int;
    #[no_mangle]
    fn strncmp(
        _: *const std::os::raw::c_char,
        _: *const std::os::raw::c_char,
        _: std::os::raw::c_ulong,
    ) -> std::os::raw::c_int;
    #[no_mangle]
    fn strtol(
        _: *const std::os::raw::c_char,
        _: *mut *mut std::os::raw::c_char,
        _: std::os::raw::c_int,
    ) -> std::os::raw::c_long;
    #[no_mangle]
    fn strchr(_: *const std::os::raw::c_char, _: std::os::raw::c_int) -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn strstr(
        _: *const std::os::raw::c_char,
        _: *const std::os::raw::c_char,
    ) -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn strlen(_: *const std::os::raw::c_char) -> std::os::raw::c_ulong;
    #[no_mangle]
    fn open(
        __file: *const std::os::raw::c_char,
        __oflag: std::os::raw::c_int,
        _: ...
    ) -> std::os::raw::c_int;
    #[no_mangle]
    fn lseek(
        __fd: std::os::raw::c_int,
        __offset: __off_t,
        __whence: std::os::raw::c_int,
    ) -> __off_t;
    #[no_mangle]
    fn read(
        __fd: std::os::raw::c_int,
        __buf: *mut std::os::raw::c_void,
        __nbytes: size_t,
    ) -> ssize_t;
    #[no_mangle]
    fn getcwd(__buf: *mut std::os::raw::c_char, __size: size_t) -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn dlclose(__handle: *mut std::os::raw::c_void) -> std::os::raw::c_int;
    #[no_mangle]
    fn dlsym(
        __handle: *mut std::os::raw::c_void,
        __name: *const std::os::raw::c_char,
    ) -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn tcc_add_file(s: *mut TCCState, filename: *const std::os::raw::c_char)
    -> std::os::raw::c_int;
    #[no_mangle]
    fn tcc_compile_string(
        s: *mut TCCState,
        buf: *const std::os::raw::c_char,
    ) -> std::os::raw::c_int;
    #[no_mangle]
    fn tcc_free(ptr: *mut std::os::raw::c_void);
    #[no_mangle]
    fn tcc_malloc(size: std::os::raw::c_ulong) -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn _tcc_error(fmt: *const std::os::raw::c_char, _: ...) -> !;
    #[no_mangle]
    fn tcc_enter_state(s1: *mut TCCState);
    #[no_mangle]
    fn pstrcpy(
        buf: *mut std::os::raw::c_char,
        buf_size: size_t,
        s: *const std::os::raw::c_char,
    ) -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn _tcc_error_noabort(fmt: *const std::os::raw::c_char, _: ...);
    #[no_mangle]
    fn tcc_realloc(
        ptr: *mut std::os::raw::c_void,
        size: std::os::raw::c_ulong,
    ) -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn dynarray_add(
        ptab: *mut std::os::raw::c_void,
        nb_ptr: *mut std::os::raw::c_int,
        data: *mut std::os::raw::c_void,
    );
    #[no_mangle]
    fn tcc_mallocz(size: std::os::raw::c_ulong) -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn relocate(
        s1: *mut TCCState,
        rel: *mut Elf64_Rela,
        type_0: std::os::raw::c_int,
        ptr: *mut std::os::raw::c_uchar,
        addr: Elf64_Addr,
        val: Elf64_Addr,
    );
    #[no_mangle]
    fn relocate_plt(s1: *mut TCCState);
    #[no_mangle]
    fn create_plt_entry(
        s1: *mut TCCState,
        got_offset: std::os::raw::c_uint,
        attr: *mut sym_attr,
    ) -> std::os::raw::c_uint;
    #[no_mangle]
    fn code_reloc(reloc_type: std::os::raw::c_int) -> std::os::raw::c_int;
    #[no_mangle]
    fn gotplt_entry_type(reloc_type: std::os::raw::c_int) -> std::os::raw::c_int;
    #[no_mangle]
    fn _tcc_warning(fmt: *const std::os::raw::c_char, _: ...);
    #[no_mangle]
    fn tcc_add_crt(s: *mut TCCState, filename: *const std::os::raw::c_char) -> std::os::raw::c_int;
    #[no_mangle]
    fn cstr_free(cstr: *mut CString);
    #[no_mangle]
    fn cstr_printf(
        cs: *mut CString,
        fmt: *const std::os::raw::c_char,
        _: ...
    ) -> std::os::raw::c_int;
    #[no_mangle]
    fn cstr_new(cstr: *mut CString);
    #[no_mangle]
    fn tcc_add_library_err(s: *mut TCCState, f: *const std::os::raw::c_char)
    -> std::os::raw::c_int;
    #[no_mangle]
    fn tcc_add_pragma_libs(s1: *mut TCCState);
    #[no_mangle]
    fn tcc_basename(name: *const std::os::raw::c_char) -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn tcc_strdup(str: *const std::os::raw::c_char) -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn dynarray_reset(pp: *mut std::os::raw::c_void, n: *mut std::os::raw::c_int);
    #[no_mangle]
    fn tcc_add_file_internal(
        s1: *mut TCCState,
        filename: *const std::os::raw::c_char,
        flags: std::os::raw::c_int,
    ) -> std::os::raw::c_int;
    #[no_mangle]
    fn tcc_add_dll(
        s: *mut TCCState,
        filename: *const std::os::raw::c_char,
        flags: std::os::raw::c_int,
    ) -> std::os::raw::c_int;
    #[no_mangle]
    fn tcc_add_dllref(s1: *mut TCCState, dllname: *const std::os::raw::c_char)
    -> *mut DLLReference;
    #[no_mangle]
    fn sysconf(__name: std::os::raw::c_int) -> std::os::raw::c_long;
    #[no_mangle]
    fn unlink(__name: *const std::os::raw::c_char) -> std::os::raw::c_int;
}
pub type size_t = std::os::raw::c_ulong;
pub type __uint8_t = std::os::raw::c_uchar;
pub type __uint16_t = std::os::raw::c_ushort;
pub type __uint32_t = std::os::raw::c_uint;
pub type __int64_t = std::os::raw::c_long;
pub type __uint64_t = std::os::raw::c_ulong;
pub type __off_t = std::os::raw::c_long;
pub type __off64_t = std::os::raw::c_long;
pub type __ssize_t = std::os::raw::c_long;
pub type ssize_t = __ssize_t;
pub type int64_t = __int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [std::os::raw::c_ulong; 16],
}
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
/*
 *  ELF file handling for TCC
 *
 *  Copyright (c) 2001-2004 Fabrice Bellard
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library; if not, write to the Free Software
 * Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA  02111-1307  USA
 */
/* Define this to get some debug output during relocation processing.  */
/* *******************************************************/
/* global variables */
/* elf version information */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sym_version {
    pub lib: *mut std::os::raw::c_char,
    pub version: *mut std::os::raw::c_char,
    pub out_index: std::os::raw::c_int,
    pub prev_same_lib: std::os::raw::c_int,
}
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
pub struct Elf64_Phdr {
    pub p_type: Elf64_Word,
    pub p_flags: Elf64_Word,
    pub p_offset: Elf64_Off,
    pub p_vaddr: Elf64_Addr,
    pub p_paddr: Elf64_Addr,
    pub p_filesz: Elf64_Xword,
    pub p_memsz: Elf64_Xword,
    pub p_align: Elf64_Xword,
}
pub type Elf64_Off = uint64_t;
pub type Elf64_Word = uint32_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Elf64_Shdr {
    pub sh_name: Elf64_Word,
    pub sh_type: Elf64_Word,
    pub sh_flags: Elf64_Xword,
    pub sh_addr: Elf64_Addr,
    pub sh_offset: Elf64_Off,
    pub sh_size: Elf64_Xword,
    pub sh_link: Elf64_Word,
    pub sh_info: Elf64_Word,
    pub sh_addralign: Elf64_Xword,
    pub sh_entsize: Elf64_Xword,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Elf64_Ehdr {
    pub e_ident: [std::os::raw::c_uchar; 16],
    pub e_type: Elf64_Half,
    pub e_machine: Elf64_Half,
    pub e_version: Elf64_Word,
    pub e_entry: Elf64_Addr,
    pub e_phoff: Elf64_Off,
    pub e_shoff: Elf64_Off,
    pub e_flags: Elf64_Word,
    pub e_ehsize: Elf64_Half,
    pub e_phentsize: Elf64_Half,
    pub e_phnum: Elf64_Half,
    pub e_shentsize: Elf64_Half,
    pub e_shnum: Elf64_Half,
    pub e_shstrndx: Elf64_Half,
}
pub type Elf64_Half = uint16_t;
pub type uint16_t = __uint16_t;
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
/* Info to be copied in dynamic section */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dyn_inf {
    pub dynamic: *mut Section,
    pub dynstr: *mut Section,
    pub data_offset: std::os::raw::c_ulong,
    pub rel_addr: Elf64_Addr,
    pub rel_size: Elf64_Addr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub d_val: Elf64_Xword,
    pub d_ptr: Elf64_Addr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Elf64_Dyn {
    pub d_tag: Elf64_Sxword,
    pub d_un: C2RustUnnamed_5,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Stab_Sym {
    pub n_strx: std::os::raw::c_uint,
    pub n_type: std::os::raw::c_uchar,
    pub n_other: std::os::raw::c_uchar,
    pub n_desc: std::os::raw::c_ushort,
    pub n_value: std::os::raw::c_uint,
}
/* Info for GNU_RELRO */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ro_inf {
    pub sh_offset: Elf64_Addr,
    pub sh_addr: Elf64_Addr,
    pub sh_size: Elf64_Addr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Elf64_Verneed {
    pub vn_version: Elf64_Half,
    pub vn_cnt: Elf64_Half,
    pub vn_file: Elf64_Word,
    pub vn_aux: Elf64_Word,
    pub vn_next: Elf64_Word,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Elf64_Vernaux {
    pub vna_hash: Elf64_Word,
    pub vna_flags: Elf64_Half,
    pub vna_other: Elf64_Half,
    pub vna_name: Elf64_Word,
    pub vna_next: Elf64_Word,
}
pub const BUILD_GOT_ONLY: gotplt_entry = 1;
pub const AUTO_GOTPLT_ENTRY: gotplt_entry = 2;
pub const NO_GOTPLT_ENTRY: gotplt_entry = 0;
pub type uintptr_t = std::os::raw::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Elf64_Verdef {
    pub vd_version: Elf64_Half,
    pub vd_flags: Elf64_Half,
    pub vd_ndx: Elf64_Half,
    pub vd_cnt: Elf64_Half,
    pub vd_hash: Elf64_Word,
    pub vd_aux: Elf64_Word,
    pub vd_next: Elf64_Word,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Elf64_Verdaux {
    pub vda_name: Elf64_Word,
    pub vda_next: Elf64_Word,
}
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
pub const N_EINCL: __stab_debug_code = 162;
pub const N_PSYM: __stab_debug_code = 160;
pub const N_SOL: __stab_debug_code = 132;
pub const N_BINCL: __stab_debug_code = 130;
pub const N_LSYM: __stab_debug_code = 128;
pub const N_SO: __stab_debug_code = 100;
pub const N_SSYM: __stab_debug_code = 96;
pub const N_CATCH: __stab_debug_code = 84;
pub const N_MOD2: __stab_debug_code = 80;
pub const N_EHDECL: __stab_debug_code = 80;
pub const N_DEFD: __stab_debug_code = 74;
pub const N_BROWS: __stab_debug_code = 72;
pub const N_BSLINE: __stab_debug_code = 72;
pub const N_DSLINE: __stab_debug_code = 70;
pub const N_SLINE: __stab_debug_code = 68;
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
pub const N_FUN: __stab_debug_code = 36;
pub const N_FNAME: __stab_debug_code = 34;
pub const N_GSYM: __stab_debug_code = 32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SectionMergeInfo {
    pub s: *mut Section,
    pub offset: std::os::raw::c_ulong,
    pub new_section: uint8_t,
    pub link_once: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ArchiveHeader {
    pub ar_name: [std::os::raw::c_char; 16],
    pub ar_date: [std::os::raw::c_char; 12],
    pub ar_uid: [std::os::raw::c_char; 6],
    pub ar_gid: [std::os::raw::c_char; 6],
    pub ar_mode: [std::os::raw::c_char; 8],
    pub ar_size: [std::os::raw::c_char; 10],
    pub ar_fmag: [std::os::raw::c_char; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct versym_info {
    pub nb_versyms: std::os::raw::c_int,
    pub verdef: *mut Elf64_Verdef,
    pub verneed: *mut Elf64_Verneed,
    pub versym: *mut Elf64_Half,
    pub nb_local_ver: std::os::raw::c_int,
    pub local_ver: *mut std::os::raw::c_int,
}
pub type gotplt_entry = std::os::raw::c_uint;
pub const ALWAYS_GOTPLT_ENTRY: gotplt_entry = 3;
#[inline]
unsafe extern "C" fn write64le(mut p: *mut std::os::raw::c_uchar, mut x: uint64_t) {
    write32le(p, x as uint32_t);
    write32le(
        p.offset(4 as std::os::raw::c_int as isize),
        (x >> 32 as std::os::raw::c_int) as uint32_t,
    );
}
#[inline]
unsafe extern "C" fn write32le(mut p: *mut std::os::raw::c_uchar, mut x: uint32_t) {
    write16le(p, x as uint16_t);
    write16le(
        p.offset(2 as std::os::raw::c_int as isize),
        (x >> 16 as std::os::raw::c_int) as uint16_t,
    );
}
#[inline]
unsafe extern "C" fn write16le(mut p: *mut std::os::raw::c_uchar, mut x: uint16_t) {
    *p.offset(0 as std::os::raw::c_int as isize) =
        (x as std::os::raw::c_int & 255 as std::os::raw::c_int) as std::os::raw::c_uchar;
    *p.offset(1 as std::os::raw::c_int as isize) =
        (x as std::os::raw::c_int >> 8 as std::os::raw::c_int & 255 as std::os::raw::c_int)
            as std::os::raw::c_uchar;
}
#[inline]
unsafe extern "C" fn isnum(mut c: std::os::raw::c_int) -> std::os::raw::c_int {
    return (c >= '0' as i32 && c <= '9' as i32) as std::os::raw::c_int;
}
#[inline]
unsafe extern "C" fn isid(mut c: std::os::raw::c_int) -> std::os::raw::c_int {
    return (c >= 'a' as i32 && c <= 'z' as i32
        || c >= 'A' as i32 && c <= 'Z' as i32
        || c == '_' as i32) as std::os::raw::c_int;
}
/* ------------------------------------------------------------------------- */
#[no_mangle]
pub unsafe extern "C" fn tccelf_new(mut s: *mut TCCState) {
    let mut s1: *mut TCCState = s;
    /* no section zero */
    dynarray_add(
        &mut (*s).sections as *mut *mut *mut Section as *mut std::os::raw::c_void,
        &mut (*s).nb_sections,
        0 as *mut std::os::raw::c_void,
    );
    /* create standard sections */
    (*s1).text_section = new_section(
        s,
        b".text\x00" as *const u8 as *const std::os::raw::c_char,
        1 as std::os::raw::c_int,
        (1 as std::os::raw::c_int) << 1 as std::os::raw::c_int
            | (1 as std::os::raw::c_int) << 2 as std::os::raw::c_int,
    );
    (*s1).data_section = new_section(
        s,
        b".data\x00" as *const u8 as *const std::os::raw::c_char,
        1 as std::os::raw::c_int,
        (1 as std::os::raw::c_int) << 1 as std::os::raw::c_int
            | (1 as std::os::raw::c_int) << 0 as std::os::raw::c_int,
    );
    /* create ro data section (make ro after relocation done with GNU_RELRO) */
    (*s1).rodata_section = new_section(
        s,
        b".data.ro\x00" as *const u8 as *const std::os::raw::c_char,
        1 as std::os::raw::c_int,
        (1 as std::os::raw::c_int) << 1 as std::os::raw::c_int
            | (1 as std::os::raw::c_int) << 0 as std::os::raw::c_int,
    );
    (*s1).bss_section = new_section(
        s,
        b".bss\x00" as *const u8 as *const std::os::raw::c_char,
        8 as std::os::raw::c_int,
        (1 as std::os::raw::c_int) << 1 as std::os::raw::c_int
            | (1 as std::os::raw::c_int) << 0 as std::os::raw::c_int,
    );
    (*s1).common_section = new_section(
        s,
        b".common\x00" as *const u8 as *const std::os::raw::c_char,
        8 as std::os::raw::c_int,
        0x80000000 as std::os::raw::c_uint as std::os::raw::c_int,
    );
    (*(*s1).common_section).sh_num = 0xfff2 as std::os::raw::c_int;
    /* symbols are always generated for linking stage */
    (*s1).symtab_section = new_symtab(
        s,
        b".symtab\x00" as *const u8 as *const std::os::raw::c_char,
        2 as std::os::raw::c_int,
        0 as std::os::raw::c_int,
        b".strtab\x00" as *const u8 as *const std::os::raw::c_char,
        b".hashtab\x00" as *const u8 as *const std::os::raw::c_char,
        0x80000000 as std::os::raw::c_uint as std::os::raw::c_int,
    );
    (*s).symtab = (*s1).symtab_section;
    /* private symbol table for dynamic symbols */
    (*s).dynsymtab_section = new_symtab(
        s,
        b".dynsymtab\x00" as *const u8 as *const std::os::raw::c_char,
        2 as std::os::raw::c_int,
        (0x80000000 as std::os::raw::c_uint
            | 0x40000000 as std::os::raw::c_int as std::os::raw::c_uint)
            as std::os::raw::c_int,
        b".dynstrtab\x00" as *const u8 as *const std::os::raw::c_char,
        b".dynhashtab\x00" as *const u8 as *const std::os::raw::c_char,
        0x80000000 as std::os::raw::c_uint as std::os::raw::c_int,
    );
    get_sym_attr(s, 0 as std::os::raw::c_int, 1 as std::os::raw::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn tccelf_bounds_new(mut s: *mut TCCState) {
    let mut s1: *mut TCCState = s;
    /* create bounds sections (make ro after relocation done with GNU_RELRO) */
    (*s1).bounds_section = new_section(
        s,
        b".bounds\x00" as *const u8 as *const std::os::raw::c_char,
        1 as std::os::raw::c_int,
        (1 as std::os::raw::c_int) << 1 as std::os::raw::c_int
            | (1 as std::os::raw::c_int) << 0 as std::os::raw::c_int,
    );
    (*s1).lbounds_section = new_section(
        s,
        b".lbounds\x00" as *const u8 as *const std::os::raw::c_char,
        1 as std::os::raw::c_int,
        (1 as std::os::raw::c_int) << 1 as std::os::raw::c_int
            | (1 as std::os::raw::c_int) << 0 as std::os::raw::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn tccelf_stab_new(mut s: *mut TCCState) {
    let mut s1: *mut TCCState = s;
    let mut shf: std::os::raw::c_int = 0 as std::os::raw::c_int;
    /* include stab info with standalone backtrace support */
    if (*s).do_backtrace as std::os::raw::c_int != 0 && (*s).output_type != 1 as std::os::raw::c_int
    {
        shf = (1 as std::os::raw::c_int) << 1 as std::os::raw::c_int
            | (1 as std::os::raw::c_int) << 0 as std::os::raw::c_int
    } // SHF_WRITE needed for musl/SELINUX
    (*s1).stab_section = new_section(
        s,
        b".stab\x00" as *const u8 as *const std::os::raw::c_char,
        1 as std::os::raw::c_int,
        shf,
    );
    (*(*s1).stab_section).sh_entsize =
        ::std::mem::size_of::<Stab_Sym>() as std::os::raw::c_ulong as std::os::raw::c_int;
    (*(*s1).stab_section).sh_addralign = ::std::mem::size_of::<std::os::raw::c_uint>()
        as std::os::raw::c_ulong as std::os::raw::c_int;
    (*(*s1).stab_section).link = new_section(
        s,
        b".stabstr\x00" as *const u8 as *const std::os::raw::c_char,
        3 as std::os::raw::c_int,
        shf,
    );
    /* put first entry */
    put_stabs(
        s,
        b"\x00" as *const u8 as *const std::os::raw::c_char,
        0 as std::os::raw::c_int,
        0 as std::os::raw::c_int,
        0 as std::os::raw::c_int,
        0 as std::os::raw::c_int as std::os::raw::c_ulong,
    );
}
unsafe extern "C" fn free_section(mut s: *mut Section) {
    tcc_free((*s).data as *mut std::os::raw::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn tccelf_delete(mut s1: *mut TCCState) {
    let mut i: std::os::raw::c_int = 0;
    /* free symbol versions */
    i = 0 as std::os::raw::c_int;
    while i < (*s1).nb_sym_versions {
        tcc_free((*(*s1).sym_versions.offset(i as isize)).version as *mut std::os::raw::c_void);
        tcc_free((*(*s1).sym_versions.offset(i as isize)).lib as *mut std::os::raw::c_void);
        i += 1
    }
    tcc_free((*s1).sym_versions as *mut std::os::raw::c_void);
    tcc_free((*s1).sym_to_version as *mut std::os::raw::c_void);
    /* free all sections */
    i = 1 as std::os::raw::c_int;
    while i < (*s1).nb_sections {
        free_section(*(*s1).sections.offset(i as isize));
        i += 1
    }
    dynarray_reset(
        &mut (*s1).sections as *mut *mut *mut Section as *mut std::os::raw::c_void,
        &mut (*s1).nb_sections,
    );
    i = 0 as std::os::raw::c_int;
    while i < (*s1).nb_priv_sections {
        free_section(*(*s1).priv_sections.offset(i as isize));
        i += 1
    }
    dynarray_reset(
        &mut (*s1).priv_sections as *mut *mut *mut Section as *mut std::os::raw::c_void,
        &mut (*s1).nb_priv_sections,
    );
    /* free any loaded DLLs */
    i = 0 as std::os::raw::c_int;
    while i < (*s1).nb_loaded_dlls {
        let mut ref_0: *mut DLLReference = *(*s1).loaded_dlls.offset(i as isize);
        if !(*ref_0).handle.is_null() {
            dlclose((*ref_0).handle);
        }
        i += 1
    }
    /* free loaded dlls array */
    dynarray_reset(
        &mut (*s1).loaded_dlls as *mut *mut *mut DLLReference as *mut std::os::raw::c_void,
        &mut (*s1).nb_loaded_dlls,
    );
    tcc_free((*s1).sym_attrs as *mut std::os::raw::c_void);
    (*s1).symtab_section = 0 as *mut Section;
    /* for tccrun.c:rt_printline() */
}
/* save section data state */
#[no_mangle]
pub unsafe extern "C" fn tccelf_begin_file(mut s1: *mut TCCState) {
    let mut s: *mut Section = 0 as *mut Section;
    let mut i: std::os::raw::c_int = 0;
    i = 1 as std::os::raw::c_int;
    while i < (*s1).nb_sections {
        s = *(*s1).sections.offset(i as isize);
        (*s).sh_offset = (*s).data_offset;
        i += 1
    }
    /* disable symbol hashing during compilation */
    s = (*s1).symtab;
    (*s).reloc = (*s).hash;
    (*s).hash = 0 as *mut Section;
}
/* At the end of compilation, convert any UNDEF syms to global, and merge
with previously existing symbols */
#[no_mangle]
pub unsafe extern "C" fn tccelf_end_file(mut s1: *mut TCCState) {
    let mut s: *mut Section = (*s1).symtab;
    let mut first_sym: std::os::raw::c_int = 0;
    let mut nb_syms: std::os::raw::c_int = 0;
    let mut tr: *mut std::os::raw::c_int = 0 as *mut std::os::raw::c_int;
    let mut i: std::os::raw::c_int = 0;
    first_sym = (*s)
        .sh_offset
        .wrapping_div(::std::mem::size_of::<Elf64_Sym>() as std::os::raw::c_ulong)
        as std::os::raw::c_int;
    nb_syms = (*s)
        .data_offset
        .wrapping_div(::std::mem::size_of::<Elf64_Sym>() as std::os::raw::c_ulong)
        .wrapping_sub(first_sym as std::os::raw::c_ulong) as std::os::raw::c_int;
    (*s).data_offset = (*s).sh_offset;
    (*(*s).link).data_offset = (*(*s).link).sh_offset;
    (*s).hash = (*s).reloc;
    (*s).reloc = 0 as *mut Section;
    tr = tcc_mallocz(
        (nb_syms as std::os::raw::c_ulong)
            .wrapping_mul(::std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong),
    ) as *mut std::os::raw::c_int;
    i = 0 as std::os::raw::c_int;
    while i < nb_syms {
        let mut sym: *mut Elf64_Sym = ((*s).data as *mut Elf64_Sym)
            .offset(first_sym as isize)
            .offset(i as isize);
        if (*sym).st_shndx as std::os::raw::c_int == 0 as std::os::raw::c_int
            && (*sym).st_info as std::os::raw::c_int >> 4 as std::os::raw::c_int
                == 0 as std::os::raw::c_int
        {
            (*sym).st_info = (((1 as std::os::raw::c_int) << 4 as std::os::raw::c_int)
                + ((*sym).st_info as std::os::raw::c_int
                    & 0xf as std::os::raw::c_int
                    & 0xf as std::os::raw::c_int))
                as std::os::raw::c_uchar
        }
        *tr.offset(i as isize) = set_elf_sym(
            s,
            (*sym).st_value,
            (*sym).st_size,
            (*sym).st_info as std::os::raw::c_int,
            (*sym).st_other as std::os::raw::c_int,
            (*sym).st_shndx as std::os::raw::c_int,
            ((*(*s).link).data as *mut std::os::raw::c_char).offset((*sym).st_name as isize),
        );
        i += 1
    }
    /* now update relocations */
    i = 1 as std::os::raw::c_int;
    while i < (*s1).nb_sections {
        let mut sr: *mut Section = *(*s1).sections.offset(i as isize);
        if (*sr).sh_type == 4 as std::os::raw::c_int && (*sr).link == s {
            let mut rel: *mut Elf64_Rela =
                (*sr).data.offset((*sr).sh_offset as isize) as *mut Elf64_Rela;
            let mut rel_end: *mut Elf64_Rela =
                (*sr).data.offset((*sr).data_offset as isize) as *mut Elf64_Rela;
            while rel < rel_end {
                let mut n: std::os::raw::c_int = ((*rel).r_info >> 32 as std::os::raw::c_int)
                    .wrapping_sub(first_sym as std::os::raw::c_ulong)
                    as std::os::raw::c_int;
                //if (n < 0) tcc_error("internal: invalid symbol index in relocation");
                (*rel).r_info = ((*tr.offset(n as isize) as Elf64_Xword)
                    << 32 as std::os::raw::c_int)
                    .wrapping_add(
                        (*rel).r_info & 0xffffffff as std::os::raw::c_uint as std::os::raw::c_ulong,
                    );
                rel = rel.offset(1)
            }
        }
        i += 1
    }
    tcc_free(tr as *mut std::os::raw::c_void);
    /* record text/data/bss output for -bench info */
    i = 0 as std::os::raw::c_int; /* gcc/pcc default alignment */
    while i < 4 as std::os::raw::c_int {
        s = *(*s1)
            .sections
            .offset((i + 1 as std::os::raw::c_int) as isize);
        (*s1).total_output[i as usize] = ((*s1).total_output[i as usize] as std::os::raw::c_ulong)
            .wrapping_add((*s).data_offset.wrapping_sub((*s).sh_offset))
            as std::os::raw::c_int as std::os::raw::c_int;
        i += 1
    }
}
#[no_mangle]
pub unsafe extern "C" fn new_section(
    mut s1: *mut TCCState,
    mut name: *const std::os::raw::c_char,
    mut sh_type: std::os::raw::c_int,
    mut sh_flags: std::os::raw::c_int,
) -> *mut Section {
    let mut sec: *mut Section = 0 as *mut Section;
    sec = tcc_mallocz(
        (::std::mem::size_of::<Section>() as std::os::raw::c_ulong).wrapping_add(strlen(name)),
    ) as *mut Section;
    (*sec).s1 = s1;
    strcpy((*sec).name.as_mut_ptr(), name);
    (*sec).sh_type = sh_type;
    (*sec).sh_flags = sh_flags;
    match sh_type {
        1879048191 => (*sec).sh_addralign = 2 as std::os::raw::c_int,
        5 | 9 | 4 | 11 | 2 | 6 | 1879048190 | 1879048189 => {
            (*sec).sh_addralign = 8 as std::os::raw::c_int
        },
        3 => (*sec).sh_addralign = 1 as std::os::raw::c_int,
        _ => (*sec).sh_addralign = 8 as std::os::raw::c_int,
    }
    if sh_flags as std::os::raw::c_uint & 0x80000000 as std::os::raw::c_uint != 0 {
        dynarray_add(
            &mut (*s1).priv_sections as *mut *mut *mut Section as *mut std::os::raw::c_void,
            &mut (*s1).nb_priv_sections,
            sec as *mut std::os::raw::c_void,
        );
    } else {
        (*sec).sh_num = (*s1).nb_sections;
        dynarray_add(
            &mut (*s1).sections as *mut *mut *mut Section as *mut std::os::raw::c_void,
            &mut (*s1).nb_sections,
            sec as *mut std::os::raw::c_void,
        );
    }
    return sec;
}
#[no_mangle]
pub unsafe extern "C" fn new_symtab(
    mut s1: *mut TCCState,
    mut symtab_name: *const std::os::raw::c_char,
    mut sh_type: std::os::raw::c_int,
    mut sh_flags: std::os::raw::c_int,
    mut strtab_name: *const std::os::raw::c_char,
    mut hash_name: *const std::os::raw::c_char,
    mut hash_sh_flags: std::os::raw::c_int,
) -> *mut Section {
    let mut symtab: *mut Section = 0 as *mut Section;
    let mut strtab: *mut Section = 0 as *mut Section;
    let mut hash: *mut Section = 0 as *mut Section;
    let mut ptr: *mut std::os::raw::c_int = 0 as *mut std::os::raw::c_int;
    let mut nb_buckets: std::os::raw::c_int = 0;
    symtab = new_section(s1, symtab_name, sh_type, sh_flags);
    (*symtab).sh_entsize =
        ::std::mem::size_of::<Elf64_Sym>() as std::os::raw::c_ulong as std::os::raw::c_int;
    strtab = new_section(s1, strtab_name, 3 as std::os::raw::c_int, sh_flags);
    put_elf_str(strtab, b"\x00" as *const u8 as *const std::os::raw::c_char);
    (*symtab).link = strtab;
    put_elf_sym(
        symtab,
        0 as std::os::raw::c_int as Elf64_Addr,
        0 as std::os::raw::c_int as std::os::raw::c_ulong,
        0 as std::os::raw::c_int,
        0 as std::os::raw::c_int,
        0 as std::os::raw::c_int,
        0 as *const std::os::raw::c_char,
    );
    nb_buckets = 1 as std::os::raw::c_int;
    hash = new_section(s1, hash_name, 5 as std::os::raw::c_int, hash_sh_flags);
    (*hash).sh_entsize = ::std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong
        as std::os::raw::c_int;
    (*symtab).hash = hash;
    (*hash).link = symtab;
    ptr = section_ptr_add(
        hash,
        ((2 as std::os::raw::c_int + nb_buckets + 1 as std::os::raw::c_int)
            as std::os::raw::c_ulong)
            .wrapping_mul(::std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong),
    ) as *mut std::os::raw::c_int;
    *ptr.offset(0 as std::os::raw::c_int as isize) = nb_buckets;
    *ptr.offset(1 as std::os::raw::c_int as isize) = 1 as std::os::raw::c_int;
    memset(
        ptr.offset(2 as std::os::raw::c_int as isize) as *mut std::os::raw::c_void,
        0 as std::os::raw::c_int,
        ((nb_buckets + 1 as std::os::raw::c_int) as std::os::raw::c_ulong)
            .wrapping_mul(::std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong),
    );
    return symtab;
}
/* realloc section and set its content to zero */
#[no_mangle]
pub unsafe extern "C" fn section_realloc(
    mut sec: *mut Section,
    mut new_size: std::os::raw::c_ulong,
) {
    let mut size: std::os::raw::c_ulong = 0;
    let mut data: *mut std::os::raw::c_uchar = 0 as *mut std::os::raw::c_uchar;
    size = (*sec).data_allocated;
    if size == 0 as std::os::raw::c_int as std::os::raw::c_ulong {
        size = 1 as std::os::raw::c_int as std::os::raw::c_ulong
    }
    while size < new_size {
        size = size.wrapping_mul(2 as std::os::raw::c_int as std::os::raw::c_ulong)
    }
    data =
        tcc_realloc((*sec).data as *mut std::os::raw::c_void, size) as *mut std::os::raw::c_uchar;
    memset(
        data.offset((*sec).data_allocated as isize) as *mut std::os::raw::c_void,
        0 as std::os::raw::c_int,
        size.wrapping_sub((*sec).data_allocated),
    );
    (*sec).data = data;
    (*sec).data_allocated = size;
}
/* reserve at least 'size' bytes aligned per 'align' in section
'sec' from current offset, and return the aligned offset */
#[no_mangle]
pub unsafe extern "C" fn section_add(
    mut sec: *mut Section,
    mut size: Elf64_Addr,
    mut align: std::os::raw::c_int,
) -> size_t {
    let mut offset: size_t = 0;
    let mut offset1: size_t = 0;
    offset = (*sec)
        .data_offset
        .wrapping_add(align as std::os::raw::c_ulong)
        .wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong)
        & -align as std::os::raw::c_ulong;
    offset1 = offset.wrapping_add(size);
    if (*sec).sh_type != 8 as std::os::raw::c_int && offset1 > (*sec).data_allocated {
        section_realloc(sec, offset1);
    }
    (*sec).data_offset = offset1;
    if align > (*sec).sh_addralign {
        (*sec).sh_addralign = align
    }
    return offset;
}
/* reserve at least 'size' bytes in section 'sec' from
sec->data_offset. */
#[no_mangle]
pub unsafe extern "C" fn section_ptr_add(
    mut sec: *mut Section,
    mut size: Elf64_Addr,
) -> *mut std::os::raw::c_void {
    let mut offset: size_t = section_add(sec, size, 1 as std::os::raw::c_int);
    return (*sec).data.offset(offset as isize) as *mut std::os::raw::c_void;
}
/* reserve at least 'size' bytes from section start */
unsafe extern "C" fn section_reserve(mut sec: *mut Section, mut size: std::os::raw::c_ulong) {
    if size > (*sec).data_allocated {
        section_realloc(sec, size);
    }
    if size > (*sec).data_offset {
        (*sec).data_offset = size
    };
}
unsafe extern "C" fn find_section_create(
    mut s1: *mut TCCState,
    mut name: *const std::os::raw::c_char,
    mut create: std::os::raw::c_int,
) -> *mut Section {
    let mut sec: *mut Section = 0 as *mut Section;
    let mut i: std::os::raw::c_int = 0;
    i = 1 as std::os::raw::c_int;
    while i < (*s1).nb_sections {
        sec = *(*s1).sections.offset(i as isize);
        if strcmp(name, (*sec).name.as_mut_ptr()) == 0 {
            return sec;
        }
        i += 1
    }
    /* sections are created as PROGBITS */
    return if create != 0 {
        new_section(
            s1,
            name,
            1 as std::os::raw::c_int,
            (1 as std::os::raw::c_int) << 1 as std::os::raw::c_int,
        )
    } else {
        0 as *mut Section
    };
}
/* return a reference to a section, and create it if it does not
exists */
#[no_mangle]
pub unsafe extern "C" fn find_section(
    mut s1: *mut TCCState,
    mut name: *const std::os::raw::c_char,
) -> *mut Section {
    return find_section_create(s1, name, 1 as std::os::raw::c_int);
}
/* ------------------------------------------------------------------------- */
#[no_mangle]
pub unsafe extern "C" fn put_elf_str(
    mut s: *mut Section,
    mut sym: *const std::os::raw::c_char,
) -> std::os::raw::c_int {
    let mut offset: std::os::raw::c_int = 0;
    let mut len: std::os::raw::c_int = 0;
    let mut ptr: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    len = strlen(sym).wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong)
        as std::os::raw::c_int;
    offset = (*s).data_offset as std::os::raw::c_int;
    ptr = section_ptr_add(s, len as Elf64_Addr) as *mut std::os::raw::c_char;
    memmove(
        ptr as *mut std::os::raw::c_void,
        sym as *const std::os::raw::c_void,
        len as std::os::raw::c_ulong,
    );
    return offset;
}
/* elf symbol hashing function */
unsafe extern "C" fn elf_hash(mut name: *const std::os::raw::c_uchar) -> std::os::raw::c_ulong {
    let mut h: std::os::raw::c_ulong = 0 as std::os::raw::c_int as std::os::raw::c_ulong;
    let mut g: std::os::raw::c_ulong = 0;
    while *name != 0 {
        let fresh0 = name;
        name = name.offset(1);
        h = (h << 4 as std::os::raw::c_int).wrapping_add(*fresh0 as std::os::raw::c_ulong);
        g = h & 0xf0000000 as std::os::raw::c_uint as std::os::raw::c_ulong;
        if g != 0 {
            h ^= g >> 24 as std::os::raw::c_int
        }
        h &= !g
    }
    return h;
}
/* rebuild hash table of section s */
/* NOTE: we do factorize the hash table code to go faster */
unsafe extern "C" fn rebuild_hash(mut s: *mut Section, mut nb_buckets: std::os::raw::c_uint) {
    let mut sym: *mut Elf64_Sym = 0 as *mut Elf64_Sym;
    let mut ptr: *mut std::os::raw::c_int = 0 as *mut std::os::raw::c_int;
    let mut hash: *mut std::os::raw::c_int = 0 as *mut std::os::raw::c_int;
    let mut nb_syms: std::os::raw::c_int = 0;
    let mut sym_index: std::os::raw::c_int = 0;
    let mut h: std::os::raw::c_int = 0;
    let mut strtab: *mut std::os::raw::c_uchar = 0 as *mut std::os::raw::c_uchar;
    strtab = (*(*s).link).data;
    nb_syms = (*s)
        .data_offset
        .wrapping_div(::std::mem::size_of::<Elf64_Sym>() as std::os::raw::c_ulong)
        as std::os::raw::c_int;
    if nb_buckets == 0 {
        nb_buckets = *((*(*s).hash).data as *mut std::os::raw::c_int)
            .offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_uint
    }
    (*(*s).hash).data_offset = 0 as std::os::raw::c_int as std::os::raw::c_ulong;
    ptr = section_ptr_add(
        (*s).hash,
        ((2 as std::os::raw::c_int as std::os::raw::c_uint)
            .wrapping_add(nb_buckets)
            .wrapping_add(nb_syms as std::os::raw::c_uint) as std::os::raw::c_ulong)
            .wrapping_mul(::std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong),
    ) as *mut std::os::raw::c_int;
    *ptr.offset(0 as std::os::raw::c_int as isize) = nb_buckets as std::os::raw::c_int;
    *ptr.offset(1 as std::os::raw::c_int as isize) = nb_syms;
    ptr = ptr.offset(2 as std::os::raw::c_int as isize);
    hash = ptr;
    memset(
        hash as *mut std::os::raw::c_void,
        0 as std::os::raw::c_int,
        (nb_buckets.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_uint)
            as std::os::raw::c_ulong)
            .wrapping_mul(::std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong),
    );
    ptr = ptr
        .offset(nb_buckets.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_uint) as isize);
    sym = ((*s).data as *mut Elf64_Sym).offset(1 as std::os::raw::c_int as isize);
    sym_index = 1 as std::os::raw::c_int;
    while sym_index < nb_syms {
        if (*sym).st_info as std::os::raw::c_int >> 4 as std::os::raw::c_int
            != 0 as std::os::raw::c_int
        {
            h = elf_hash(strtab.offset((*sym).st_name as isize))
                .wrapping_rem(nb_buckets as std::os::raw::c_ulong)
                as std::os::raw::c_int;
            *ptr = *hash.offset(h as isize);
            *hash.offset(h as isize) = sym_index
        } else {
            *ptr = 0 as std::os::raw::c_int
        }
        ptr = ptr.offset(1);
        sym = sym.offset(1);
        sym_index += 1
    }
}
/* return the symbol number */
#[no_mangle]
pub unsafe extern "C" fn put_elf_sym(
    mut s: *mut Section,
    mut value: Elf64_Addr,
    mut size: std::os::raw::c_ulong,
    mut info: std::os::raw::c_int,
    mut other: std::os::raw::c_int,
    mut shndx: std::os::raw::c_int,
    mut name: *const std::os::raw::c_char,
) -> std::os::raw::c_int {
    let mut name_offset: std::os::raw::c_int = 0;
    let mut sym_index: std::os::raw::c_int = 0;
    let mut nbuckets: std::os::raw::c_int = 0;
    let mut h: std::os::raw::c_int = 0;
    let mut sym: *mut Elf64_Sym = 0 as *mut Elf64_Sym;
    let mut hs: *mut Section = 0 as *mut Section;
    sym = section_ptr_add(
        s,
        ::std::mem::size_of::<Elf64_Sym>() as std::os::raw::c_ulong,
    ) as *mut Elf64_Sym;
    if !name.is_null()
        && *name.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int != 0
    {
        name_offset = put_elf_str((*s).link, name)
    } else {
        name_offset = 0 as std::os::raw::c_int
    }
    /* XXX: endianness */
    (*sym).st_name = name_offset as Elf64_Word;
    (*sym).st_value = value;
    (*sym).st_size = size;
    (*sym).st_info = info as std::os::raw::c_uchar;
    (*sym).st_other = other as std::os::raw::c_uchar;
    (*sym).st_shndx = shndx as Elf64_Section;
    sym_index =
        sym.offset_from((*s).data as *mut Elf64_Sym) as std::os::raw::c_long as std::os::raw::c_int;
    hs = (*s).hash;
    if !hs.is_null() {
        let mut ptr: *mut std::os::raw::c_int = 0 as *mut std::os::raw::c_int;
        let mut base: *mut std::os::raw::c_int = 0 as *mut std::os::raw::c_int;
        ptr = section_ptr_add(
            hs,
            ::std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong,
        ) as *mut std::os::raw::c_int;
        base = (*hs).data as *mut std::os::raw::c_int;
        /* only add global or weak symbols. */
        if info as std::os::raw::c_uchar as std::os::raw::c_int >> 4 as std::os::raw::c_int
            != 0 as std::os::raw::c_int
        {
            /* add another hashing entry */
            nbuckets = *base.offset(0 as std::os::raw::c_int as isize);
            h = elf_hash((*(*s).link).data.offset(name_offset as isize))
                .wrapping_rem(nbuckets as std::os::raw::c_ulong)
                as std::os::raw::c_int;
            *ptr = *base.offset((2 as std::os::raw::c_int + h) as isize);
            *base.offset((2 as std::os::raw::c_int + h) as isize) = sym_index;
            let ref mut fresh1 = *base.offset(1 as std::os::raw::c_int as isize);
            *fresh1 += 1;
            /* we resize the hash table */
            (*hs).nb_hashed_syms += 1;
            if (*hs).nb_hashed_syms > 2 as std::os::raw::c_int * nbuckets {
                rebuild_hash(
                    s,
                    (2 as std::os::raw::c_int * nbuckets) as std::os::raw::c_uint,
                );
            }
        } else {
            *ptr = 0 as std::os::raw::c_int;
            let ref mut fresh2 = *base.offset(1 as std::os::raw::c_int as isize);
            *fresh2 += 1
        }
    }
    return sym_index;
}
#[no_mangle]
pub unsafe extern "C" fn find_elf_sym(
    mut s: *mut Section,
    mut name: *const std::os::raw::c_char,
) -> std::os::raw::c_int {
    let mut sym: *mut Elf64_Sym = 0 as *mut Elf64_Sym;
    let mut hs: *mut Section = 0 as *mut Section;
    let mut nbuckets: std::os::raw::c_int = 0;
    let mut sym_index: std::os::raw::c_int = 0;
    let mut h: std::os::raw::c_int = 0;
    let mut name1: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    hs = (*s).hash;
    if hs.is_null() {
        return 0 as std::os::raw::c_int;
    }
    nbuckets = *((*hs).data as *mut std::os::raw::c_int).offset(0 as std::os::raw::c_int as isize);
    h = elf_hash(name as *mut std::os::raw::c_uchar).wrapping_rem(nbuckets as std::os::raw::c_ulong)
        as std::os::raw::c_int;
    sym_index =
        *((*hs).data as *mut std::os::raw::c_int).offset((2 as std::os::raw::c_int + h) as isize);
    while sym_index != 0 as std::os::raw::c_int {
        sym = &mut *((*s).data as *mut Elf64_Sym).offset(sym_index as isize) as *mut Elf64_Sym;
        name1 = ((*(*s).link).data as *mut std::os::raw::c_char).offset((*sym).st_name as isize);
        if strcmp(name, name1) == 0 {
            return sym_index;
        }
        sym_index = *((*hs).data as *mut std::os::raw::c_int)
            .offset((2 as std::os::raw::c_int + nbuckets + sym_index) as isize)
    }
    return 0 as std::os::raw::c_int;
}
/* return elf symbol value, signal error if 'err' is nonzero, decorate
name if FORC */
#[no_mangle]
pub unsafe extern "C" fn get_sym_addr(
    mut s1: *mut TCCState,
    mut name: *const std::os::raw::c_char,
    mut err: std::os::raw::c_int,
    mut forc: std::os::raw::c_int,
) -> Elf64_Addr {
    let mut sym_index: std::os::raw::c_int = 0;
    let mut sym: *mut Elf64_Sym = 0 as *mut Elf64_Sym;
    let mut buf: [std::os::raw::c_char; 256] = [0; 256];
    if forc != 0 && (*s1).leading_underscore as std::os::raw::c_int != 0 {
        buf[0 as std::os::raw::c_int as usize] = '_' as i32 as std::os::raw::c_char;
        pstrcpy(
            buf.as_mut_ptr().offset(1 as std::os::raw::c_int as isize),
            (::std::mem::size_of::<[std::os::raw::c_char; 256]>() as std::os::raw::c_ulong)
                .wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong),
            name,
        );
        name = buf.as_mut_ptr()
    }
    sym_index = find_elf_sym((*s1).symtab, name);
    sym =
        &mut *((*(*s1).symtab).data as *mut Elf64_Sym).offset(sym_index as isize) as *mut Elf64_Sym;
    if sym_index == 0 || (*sym).st_shndx as std::os::raw::c_int == 0 as std::os::raw::c_int {
        if err != 0 {
            tcc_enter_state(s1);
            Some(_tcc_error as unsafe extern "C" fn(_: *const std::os::raw::c_char, _: ...) -> !)
                .expect("non-null function pointer")(
                b"%s not defined\x00" as *const u8 as *const std::os::raw::c_char,
                name,
            );
        }
        return -(1 as std::os::raw::c_int) as Elf64_Addr;
    }
    return (*sym).st_value;
}
/* return elf symbol value */
#[no_mangle]
pub unsafe extern "C" fn tcc_get_symbol(
    mut s: *mut TCCState,
    mut name: *const std::os::raw::c_char,
) -> *mut std::os::raw::c_void {
    let mut addr: Elf64_Addr =
        get_sym_addr(s, name, 0 as std::os::raw::c_int, 1 as std::os::raw::c_int);
    return if addr == -(1 as std::os::raw::c_int) as std::os::raw::c_ulong {
        0 as *mut std::os::raw::c_void
    } else {
        addr as *mut std::os::raw::c_void
    };
}
/* list elf symbol names and values */
#[no_mangle]
pub unsafe extern "C" fn list_elf_symbols(
    mut s: *mut TCCState,
    mut ctx: *mut std::os::raw::c_void,
    mut symbol_cb: Option<
        unsafe extern "C" fn(
            _: *mut std::os::raw::c_void,
            _: *const std::os::raw::c_char,
            _: *const std::os::raw::c_void,
        ) -> (),
    >,
) {
    let mut sym: *mut Elf64_Sym = 0 as *mut Elf64_Sym;
    let mut symtab: *mut Section = 0 as *mut Section;
    let mut sym_index: std::os::raw::c_int = 0;
    let mut end_sym: std::os::raw::c_int = 0;
    let mut name: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut sym_vis: std::os::raw::c_uchar = 0;
    let mut sym_bind: std::os::raw::c_uchar = 0;
    symtab = (*s).symtab;
    end_sym = (*symtab)
        .data_offset
        .wrapping_div(::std::mem::size_of::<Elf64_Sym>() as std::os::raw::c_ulong)
        as std::os::raw::c_int;
    sym_index = 0 as std::os::raw::c_int;
    while sym_index < end_sym {
        sym = &mut *((*symtab).data as *mut Elf64_Sym).offset(sym_index as isize) as *mut Elf64_Sym;
        if (*sym).st_value != 0 {
            name = ((*(*symtab).link).data as *mut std::os::raw::c_char)
                .offset((*sym).st_name as isize);
            sym_bind = ((*sym).st_info as std::os::raw::c_int >> 4 as std::os::raw::c_int)
                as std::os::raw::c_uchar;
            sym_vis = ((*sym).st_other as std::os::raw::c_int & 0x3 as std::os::raw::c_int)
                as std::os::raw::c_uchar;
            if sym_bind as std::os::raw::c_int == 1 as std::os::raw::c_int
                && sym_vis as std::os::raw::c_int == 0 as std::os::raw::c_int
            {
                symbol_cb.expect("non-null function pointer")(
                    ctx,
                    name,
                    (*sym).st_value as *mut std::os::raw::c_void,
                );
            }
        }
        sym_index += 1
    }
}
/* list elf symbol names and values */
#[no_mangle]
pub unsafe extern "C" fn tcc_list_symbols(
    mut s: *mut TCCState,
    mut ctx: *mut std::os::raw::c_void,
    mut symbol_cb: Option<
        unsafe extern "C" fn(
            _: *mut std::os::raw::c_void,
            _: *const std::os::raw::c_char,
            _: *const std::os::raw::c_void,
        ) -> (),
    >,
) {
    list_elf_symbols(s, ctx, symbol_cb);
}
unsafe extern "C" fn version_add(mut s1: *mut TCCState) {
    let mut i: std::os::raw::c_int = 0;
    let mut sym: *mut Elf64_Sym = 0 as *mut Elf64_Sym;
    let mut vn: *mut Elf64_Verneed = 0 as *mut Elf64_Verneed;
    let mut symtab: *mut Section = 0 as *mut Section;
    let mut sym_index: std::os::raw::c_int = 0;
    let mut end_sym: std::os::raw::c_int = 0;
    let mut nb_versions: std::os::raw::c_int = 2 as std::os::raw::c_int;
    let mut nb_entries: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut versym: *mut Elf64_Half = 0 as *mut Elf64_Half;
    let mut name: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    if 0 as std::os::raw::c_int == (*s1).nb_sym_versions {
        return;
    }
    (*s1).versym_section = new_section(
        s1,
        b".gnu.version\x00" as *const u8 as *const std::os::raw::c_char,
        0x6fffffff as std::os::raw::c_int,
        (1 as std::os::raw::c_int) << 1 as std::os::raw::c_int,
    );
    (*(*s1).versym_section).sh_entsize =
        ::std::mem::size_of::<Elf64_Half>() as std::os::raw::c_ulong as std::os::raw::c_int;
    (*(*s1).versym_section).link = (*s1).dynsym;
    /* add needed symbols */
    symtab = (*s1).dynsym;
    end_sym = (*symtab)
        .data_offset
        .wrapping_div(::std::mem::size_of::<Elf64_Sym>() as std::os::raw::c_ulong)
        as std::os::raw::c_int;
    versym = section_ptr_add(
        (*s1).versym_section,
        (end_sym as std::os::raw::c_ulong)
            .wrapping_mul(::std::mem::size_of::<Elf64_Half>() as std::os::raw::c_ulong),
    ) as *mut Elf64_Half;
    sym_index = 0 as std::os::raw::c_int;
    while sym_index < end_sym {
        let mut dllindex: std::os::raw::c_int = 0;
        let mut verndx: std::os::raw::c_int = 0;
        sym = &mut *((*symtab).data as *mut Elf64_Sym).offset(sym_index as isize) as *mut Elf64_Sym;
        name =
            ((*(*symtab).link).data as *mut std::os::raw::c_char).offset((*sym).st_name as isize);
        dllindex = find_elf_sym((*s1).dynsymtab_section, name);
        verndx = if dllindex != 0 && dllindex < (*s1).nb_sym_to_version {
            *(*s1).sym_to_version.offset(dllindex as isize)
        } else {
            -(1 as std::os::raw::c_int)
        };
        if verndx >= 0 as std::os::raw::c_int {
            if (*(*s1).sym_versions.offset(verndx as isize)).out_index == 0 {
                let fresh3 = nb_versions;
                nb_versions = nb_versions + 1;
                (*(*s1).sym_versions.offset(verndx as isize)).out_index = fresh3
            }
            *versym.offset(sym_index as isize) =
                (*(*s1).sym_versions.offset(verndx as isize)).out_index as Elf64_Half
        } else {
            *versym.offset(sym_index as isize) = 0 as std::os::raw::c_int as Elf64_Half
        }
        sym_index += 1
    }
    /* generate verneed section, but not when it will be empty.  Some
    dynamic linkers look at their contents even when DTVERNEEDNUM and
    section size is zero.  */
    if nb_versions > 2 as std::os::raw::c_int {
        (*s1).verneed_section = new_section(
            s1,
            b".gnu.version_r\x00" as *const u8 as *const std::os::raw::c_char,
            0x6ffffffe as std::os::raw::c_int,
            (1 as std::os::raw::c_int) << 1 as std::os::raw::c_int,
        );
        (*(*s1).verneed_section).link = (*(*s1).dynsym).link;
        i = (*s1).nb_sym_versions;
        loop {
            let fresh4 = i;
            i = i - 1;
            if !(fresh4 > 0 as std::os::raw::c_int) {
                break;
            }
            let mut sv: *mut sym_version =
                &mut *(*s1).sym_versions.offset(i as isize) as *mut sym_version;
            let mut n_same_libs: std::os::raw::c_int = 0 as std::os::raw::c_int;
            let mut prev: std::os::raw::c_int = 0;
            let mut vnofs: size_t = 0;
            let mut vna: *mut Elf64_Vernaux = 0 as *mut Elf64_Vernaux;
            if (*sv).out_index < 1 as std::os::raw::c_int {
                continue;
            }
            vnofs = section_add(
                (*s1).verneed_section,
                ::std::mem::size_of::<Elf64_Verneed>() as std::os::raw::c_ulong,
                1 as std::os::raw::c_int,
            );
            vn = (*(*s1).verneed_section).data.offset(vnofs as isize) as *mut Elf64_Verneed;
            (*vn).vn_version = 1 as std::os::raw::c_int as Elf64_Half;
            (*vn).vn_file = put_elf_str((*(*s1).verneed_section).link, (*sv).lib) as Elf64_Word;
            (*vn).vn_aux =
                ::std::mem::size_of::<Elf64_Verneed>() as std::os::raw::c_ulong as Elf64_Word;
            loop {
                prev = (*sv).prev_same_lib;
                if (*sv).out_index > 0 as std::os::raw::c_int {
                    vna = section_ptr_add(
                        (*s1).verneed_section,
                        ::std::mem::size_of::<Elf64_Vernaux>() as std::os::raw::c_ulong,
                    ) as *mut Elf64_Vernaux;
                    (*vna).vna_hash =
                        elf_hash((*sv).version as *const std::os::raw::c_uchar) as Elf64_Word;
                    (*vna).vna_flags = 0 as std::os::raw::c_int as Elf64_Half;
                    (*vna).vna_other = (*sv).out_index as Elf64_Half;
                    (*sv).out_index = -(2 as std::os::raw::c_int);
                    (*vna).vna_name =
                        put_elf_str((*(*s1).verneed_section).link, (*sv).version) as Elf64_Word;
                    (*vna).vna_next = ::std::mem::size_of::<Elf64_Vernaux>()
                        as std::os::raw::c_ulong
                        as Elf64_Word;
                    n_same_libs += 1
                }
                if prev >= 0 as std::os::raw::c_int {
                    sv = &mut *(*s1).sym_versions.offset(prev as isize) as *mut sym_version
                }
                if !(prev >= 0 as std::os::raw::c_int) {
                    break;
                }
            }
            (*vna).vna_next = 0 as std::os::raw::c_int as Elf64_Word;
            vn = (*(*s1).verneed_section).data.offset(vnofs as isize) as *mut Elf64_Verneed;
            (*vn).vn_cnt = n_same_libs as Elf64_Half;
            (*vn).vn_next = (::std::mem::size_of::<Elf64_Verneed>() as std::os::raw::c_ulong)
                .wrapping_add(
                    (n_same_libs as std::os::raw::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<Elf64_Vernaux>() as std::os::raw::c_ulong
                        ),
                ) as Elf64_Word;
            nb_entries += 1
        }
        if !vn.is_null() {
            (*vn).vn_next = 0 as std::os::raw::c_int as Elf64_Word
        }
        (*(*s1).verneed_section).sh_info = nb_entries
    }
    (*s1).dt_verneednum = nb_entries;
}
/* add an elf symbol : check if it is already defined and patch
it. Return symbol index. NOTE that sh_num can be SHN_UNDEF. */
#[no_mangle]
pub unsafe extern "C" fn set_elf_sym(
    mut s: *mut Section,
    mut value: Elf64_Addr,
    mut size: std::os::raw::c_ulong,
    mut info: std::os::raw::c_int,
    mut other: std::os::raw::c_int,
    mut shndx: std::os::raw::c_int,
    mut name: *const std::os::raw::c_char,
) -> std::os::raw::c_int {
    let mut current_block: u64;
    let mut s1: *mut TCCState = (*s).s1;
    let mut esym: *mut Elf64_Sym = 0 as *mut Elf64_Sym;
    let mut sym_bind: std::os::raw::c_int = 0;
    let mut sym_index: std::os::raw::c_int = 0;
    let mut sym_type: std::os::raw::c_int = 0;
    let mut esym_bind: std::os::raw::c_int = 0;
    let mut sym_vis: std::os::raw::c_uchar = 0;
    let mut esym_vis: std::os::raw::c_uchar = 0;
    let mut new_vis: std::os::raw::c_uchar = 0;
    sym_bind = info as std::os::raw::c_uchar as std::os::raw::c_int >> 4 as std::os::raw::c_int;
    sym_type = info & 0xf as std::os::raw::c_int;
    sym_vis = (other & 0x3 as std::os::raw::c_int) as std::os::raw::c_uchar;
    if sym_bind != 0 as std::os::raw::c_int {
        /* we search global or weak symbols */
        sym_index = find_elf_sym(s, name);
        if sym_index == 0 {
            current_block = 9997610163788566778;
        } else {
            esym = &mut *((*s).data as *mut Elf64_Sym).offset(sym_index as isize) as *mut Elf64_Sym;
            if (*esym).st_value == value
                && (*esym).st_size == size
                && (*esym).st_info as std::os::raw::c_int == info
                && (*esym).st_other as std::os::raw::c_int == other
                && (*esym).st_shndx as std::os::raw::c_int == shndx
            {
                return sym_index;
            }
            if (*esym).st_shndx as std::os::raw::c_int != 0 as std::os::raw::c_int {
                esym_bind = (*esym).st_info as std::os::raw::c_int >> 4 as std::os::raw::c_int;
                /* propagate the most constraining visibility */
                /* STV_DEFAULT(0)<STV_PROTECTED(3)<STV_HIDDEN(2)<STV_INTERNAL(1) */
                esym_vis = ((*esym).st_other as std::os::raw::c_int & 0x3 as std::os::raw::c_int)
                    as std::os::raw::c_uchar; /* in case we have to patch esym */
                if esym_vis as std::os::raw::c_int == 0 as std::os::raw::c_int {
                    new_vis = sym_vis
                } else if sym_vis as std::os::raw::c_int == 0 as std::os::raw::c_int {
                    new_vis = esym_vis
                } else {
                    new_vis = if (esym_vis as std::os::raw::c_int) < sym_vis as std::os::raw::c_int
                    {
                        esym_vis as std::os::raw::c_int
                    } else {
                        sym_vis as std::os::raw::c_int
                    } as std::os::raw::c_uchar
                }
                (*esym).st_other = ((*esym).st_other as std::os::raw::c_int
                    & !(-(1 as std::os::raw::c_int) & 0x3 as std::os::raw::c_int)
                    | new_vis as std::os::raw::c_int)
                    as std::os::raw::c_uchar;
                other = (*esym).st_other as std::os::raw::c_int;
                if shndx == 0 as std::os::raw::c_int {
                    /* ignore adding of undefined symbol if the
                    corresponding symbol is already defined */
                    current_block = 6476622998065200121;
                } else if sym_bind == 1 as std::os::raw::c_int
                    && esym_bind == 2 as std::os::raw::c_int
                {
                    /* global overrides weak, so patch */
                    current_block = 188970668311456969;
                } else if sym_bind == 2 as std::os::raw::c_int
                    && esym_bind == 1 as std::os::raw::c_int
                {
                    /* weak is ignored if already global */
                    current_block = 6476622998065200121;
                } else if sym_bind == 2 as std::os::raw::c_int
                    && esym_bind == 2 as std::os::raw::c_int
                {
                    /* keep first-found weak definition, ignore subsequents */
                    current_block = 6476622998065200121;
                } else if sym_vis as std::os::raw::c_int == 2 as std::os::raw::c_int
                    || sym_vis as std::os::raw::c_int == 1 as std::os::raw::c_int
                {
                    /* ignore hidden symbols after */
                    current_block = 6476622998065200121;
                } else if ((*esym).st_shndx as std::os::raw::c_int == 0xfff2 as std::os::raw::c_int
                    || (*esym).st_shndx as std::os::raw::c_int == (*(*s1).bss_section).sh_num)
                    && (shndx < 0xff00 as std::os::raw::c_int
                        && shndx != (*(*s1).bss_section).sh_num)
                {
                    /* data symbol gets precedence over common/bss */
                    current_block = 188970668311456969;
                } else if shndx == 0xfff2 as std::os::raw::c_int
                    || shndx == (*(*s1).bss_section).sh_num
                {
                    /* data symbol keeps precedence over common/bss */
                    current_block = 6476622998065200121;
                } else if (*s).sh_flags & 0x40000000 as std::os::raw::c_int != 0 {
                    current_block = 6476622998065200121;
                } else if (*esym).st_other as std::os::raw::c_int & 0x4 as std::os::raw::c_int != 0
                {
                    current_block = 188970668311456969;
                } else {
                    tcc_enter_state(s1);
                    Some(
                        _tcc_error_noabort
                            as unsafe extern "C" fn(_: *const std::os::raw::c_char, _: ...) -> (),
                    )
                    .expect("non-null function pointer")(
                        b"\'%s\' defined twice\x00" as *const u8 as *const std::os::raw::c_char,
                        name,
                    );
                    current_block = 6476622998065200121;
                }
            } else {
                current_block = 188970668311456969;
            }
            match current_block {
                6476622998065200121 => {},
                _ =>
                /* If the existing symbol came from an asm .set
                we can override.  */
                {
                    (*esym).st_info = ((sym_bind << 4 as std::os::raw::c_int)
                        + (sym_type & 0xf as std::os::raw::c_int))
                        as std::os::raw::c_uchar;
                    (*esym).st_shndx = shndx as Elf64_Section;
                    (*s1).new_undef_sym = 1 as std::os::raw::c_int;
                    (*esym).st_value = value;
                    (*esym).st_size = size;
                    (*esym).st_other = other as std::os::raw::c_uchar;
                    current_block = 6476622998065200121;
                }
            }
        }
    } else {
        current_block = 9997610163788566778;
    }
    match current_block {
        9997610163788566778 => {
            sym_index = put_elf_sym(
                s,
                value,
                size,
                (sym_bind << 4 as std::os::raw::c_int) + (sym_type & 0xf as std::os::raw::c_int),
                other,
                shndx,
                name,
            )
        },
        _ => {},
    }
    /* we accept that two DLL define the same symbol */
    return sym_index;
}
/* put relocation */
#[no_mangle]
pub unsafe extern "C" fn put_elf_reloca(
    mut symtab: *mut Section,
    mut s: *mut Section,
    mut offset: std::os::raw::c_ulong,
    mut type_0: std::os::raw::c_int,
    mut symbol: std::os::raw::c_int,
    mut addend: Elf64_Addr,
) {
    let mut s1: *mut TCCState = (*s).s1;
    let mut buf: [std::os::raw::c_char; 256] = [0; 256];
    let mut sr: *mut Section = 0 as *mut Section;
    let mut rel: *mut Elf64_Rela = 0 as *mut Elf64_Rela;
    sr = (*s).reloc;
    if sr.is_null() {
        /* if no relocation section, create it */
        snprintf(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[std::os::raw::c_char; 256]>() as std::os::raw::c_ulong,
            b".rela%s\x00" as *const u8 as *const std::os::raw::c_char,
            (*s).name.as_mut_ptr(),
        );
        /* if the symtab is allocated, then we consider the relocation
        are also */
        sr = new_section(
            (*s).s1,
            buf.as_mut_ptr(),
            4 as std::os::raw::c_int,
            (*symtab).sh_flags,
        );
        (*sr).sh_entsize =
            ::std::mem::size_of::<Elf64_Rela>() as std::os::raw::c_ulong as std::os::raw::c_int;
        (*sr).link = symtab;
        (*sr).sh_info = (*s).sh_num;
        (*s).reloc = sr
    }
    rel = section_ptr_add(
        sr,
        ::std::mem::size_of::<Elf64_Rela>() as std::os::raw::c_ulong,
    ) as *mut Elf64_Rela;
    (*rel).r_offset = offset;
    (*rel).r_info = ((symbol as Elf64_Xword) << 32 as std::os::raw::c_int)
        .wrapping_add(type_0 as std::os::raw::c_ulong);
    (*rel).r_addend = addend as Elf64_Sxword;
    if 4 as std::os::raw::c_int != 4 as std::os::raw::c_int && addend != 0 {
        tcc_enter_state(s1);
        Some(_tcc_error as unsafe extern "C" fn(_: *const std::os::raw::c_char, _: ...) -> !)
            .expect("non-null function pointer")(
            b"non-zero addend on REL architecture\x00" as *const u8 as *const std::os::raw::c_char,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn put_elf_reloc(
    mut symtab: *mut Section,
    mut s: *mut Section,
    mut offset: std::os::raw::c_ulong,
    mut type_0: std::os::raw::c_int,
    mut symbol: std::os::raw::c_int,
) {
    put_elf_reloca(
        symtab,
        s,
        offset,
        type_0,
        symbol,
        0 as std::os::raw::c_int as Elf64_Addr,
    );
}
/* put stab debug information */
#[no_mangle]
pub unsafe extern "C" fn put_stabs(
    mut s1: *mut TCCState,
    mut str: *const std::os::raw::c_char,
    mut type_0: std::os::raw::c_int,
    mut other: std::os::raw::c_int,
    mut desc: std::os::raw::c_int,
    mut value: std::os::raw::c_ulong,
) {
    let mut sym: *mut Stab_Sym = 0 as *mut Stab_Sym;
    let mut offset: std::os::raw::c_uint = 0;
    if type_0 == N_SLINE as std::os::raw::c_int
        && {
            offset = (*(*s1).stab_section).data_offset as std::os::raw::c_uint;
            (offset) != 0
        }
        && {
            sym = ((*(*s1).stab_section).data.offset(offset as isize) as *mut Stab_Sym)
                .offset(-(1 as std::os::raw::c_int as isize));
            !sym.is_null()
        }
        && (*sym).n_type as std::os::raw::c_int == type_0
        && (*sym).n_value as std::os::raw::c_ulong == value
    {
        /* just update line_number in previous entry */
        (*sym).n_desc = desc as std::os::raw::c_ushort;
        return;
    }
    sym = section_ptr_add(
        (*s1).stab_section,
        ::std::mem::size_of::<Stab_Sym>() as std::os::raw::c_ulong,
    ) as *mut Stab_Sym;
    if !str.is_null() {
        (*sym).n_strx = put_elf_str((*(*s1).stab_section).link, str) as std::os::raw::c_uint
    } else {
        (*sym).n_strx = 0 as std::os::raw::c_int as std::os::raw::c_uint
    }
    (*sym).n_type = type_0 as std::os::raw::c_uchar;
    (*sym).n_other = other as std::os::raw::c_uchar;
    (*sym).n_desc = desc as std::os::raw::c_ushort;
    (*sym).n_value = value as std::os::raw::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn put_stabs_r(
    mut s1: *mut TCCState,
    mut str: *const std::os::raw::c_char,
    mut type_0: std::os::raw::c_int,
    mut other: std::os::raw::c_int,
    mut desc: std::os::raw::c_int,
    mut value: std::os::raw::c_ulong,
    mut sec: *mut Section,
    mut sym_index: std::os::raw::c_int,
) {
    put_elf_reloc(
        (*s1).symtab_section,
        (*s1).stab_section,
        (*(*s1).stab_section)
            .data_offset
            .wrapping_add(8 as std::os::raw::c_int as std::os::raw::c_ulong),
        if ::std::mem::size_of::<std::os::raw::c_uint>() as std::os::raw::c_ulong
            == 8 as std::os::raw::c_int as std::os::raw::c_ulong
        {
            1 as std::os::raw::c_int
        } else {
            11 as std::os::raw::c_int
        },
        sym_index,
    );
    put_stabs(s1, str, type_0, other, desc, value);
}
#[no_mangle]
pub unsafe extern "C" fn put_stabn(
    mut s1: *mut TCCState,
    mut type_0: std::os::raw::c_int,
    mut other: std::os::raw::c_int,
    mut desc: std::os::raw::c_int,
    mut value: std::os::raw::c_int,
) {
    put_stabs(
        s1,
        0 as *const std::os::raw::c_char,
        type_0,
        other,
        desc,
        value as std::os::raw::c_ulong,
    );
}
#[no_mangle]
pub unsafe extern "C" fn get_sym_attr(
    mut s1: *mut TCCState,
    mut index: std::os::raw::c_int,
    mut alloc: std::os::raw::c_int,
) -> *mut sym_attr {
    let mut n: std::os::raw::c_int = 0;
    let mut tab: *mut sym_attr = 0 as *mut sym_attr;
    if index >= (*s1).nb_sym_attrs {
        if alloc == 0 {
            return (*s1).sym_attrs;
        }
        /* find immediately bigger power of 2 and reallocate array */
        n = 1 as std::os::raw::c_int;
        while index >= n {
            n *= 2 as std::os::raw::c_int
        }
        tab = tcc_realloc(
            (*s1).sym_attrs as *mut std::os::raw::c_void,
            (n as std::os::raw::c_ulong)
                .wrapping_mul(::std::mem::size_of::<sym_attr>() as std::os::raw::c_ulong),
        ) as *mut sym_attr;
        (*s1).sym_attrs = tab;
        memset(
            (*s1).sym_attrs.offset((*s1).nb_sym_attrs as isize) as *mut std::os::raw::c_void,
            0 as std::os::raw::c_int,
            ((n - (*s1).nb_sym_attrs) as std::os::raw::c_ulong)
                .wrapping_mul(::std::mem::size_of::<sym_attr>() as std::os::raw::c_ulong),
        );
        (*s1).nb_sym_attrs = n
    }
    return &mut *(*s1).sym_attrs.offset(index as isize) as *mut sym_attr;
}
/* In an ELF file symbol table, the local symbols must appear below
the global and weak ones. Since TCC cannot sort it while generating
the code, we must do it after. All the relocation tables are also
modified to take into account the symbol table sorting */
unsafe extern "C" fn sort_syms(mut s1: *mut TCCState, mut s: *mut Section) {
    let mut old_to_new_syms: *mut std::os::raw::c_int = 0 as *mut std::os::raw::c_int;
    let mut new_syms: *mut Elf64_Sym = 0 as *mut Elf64_Sym;
    let mut nb_syms: std::os::raw::c_int = 0;
    let mut i: std::os::raw::c_int = 0;
    let mut p: *mut Elf64_Sym = 0 as *mut Elf64_Sym;
    let mut q: *mut Elf64_Sym = 0 as *mut Elf64_Sym;
    let mut rel: *mut Elf64_Rela = 0 as *mut Elf64_Rela;
    let mut sr: *mut Section = 0 as *mut Section;
    let mut type_0: std::os::raw::c_int = 0;
    let mut sym_index: std::os::raw::c_int = 0;
    nb_syms = (*s)
        .data_offset
        .wrapping_div(::std::mem::size_of::<Elf64_Sym>() as std::os::raw::c_ulong)
        as std::os::raw::c_int;
    new_syms = tcc_malloc(
        (nb_syms as std::os::raw::c_ulong)
            .wrapping_mul(::std::mem::size_of::<Elf64_Sym>() as std::os::raw::c_ulong),
    ) as *mut Elf64_Sym;
    old_to_new_syms = tcc_malloc(
        (nb_syms as std::os::raw::c_ulong)
            .wrapping_mul(::std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong),
    ) as *mut std::os::raw::c_int;
    /* first pass for local symbols */
    p = (*s).data as *mut Elf64_Sym;
    q = new_syms;
    i = 0 as std::os::raw::c_int;
    while i < nb_syms {
        if (*p).st_info as std::os::raw::c_int >> 4 as std::os::raw::c_int
            == 0 as std::os::raw::c_int
        {
            *old_to_new_syms.offset(i as isize) =
                q.offset_from(new_syms) as std::os::raw::c_long as std::os::raw::c_int;
            let fresh5 = q;
            q = q.offset(1);
            *fresh5 = *p
        }
        p = p.offset(1);
        i += 1
    }
    /* save the number of local symbols in section header */
    if (*s).sh_size != 0 {
        /* this 'if' makes IDA happy */
        (*s).sh_info = q.offset_from(new_syms) as std::os::raw::c_long as std::os::raw::c_int
    }
    /* then second pass for non local symbols */
    p = (*s).data as *mut Elf64_Sym;
    i = 0 as std::os::raw::c_int;
    while i < nb_syms {
        if (*p).st_info as std::os::raw::c_int >> 4 as std::os::raw::c_int
            != 0 as std::os::raw::c_int
        {
            *old_to_new_syms.offset(i as isize) =
                q.offset_from(new_syms) as std::os::raw::c_long as std::os::raw::c_int;
            let fresh6 = q;
            q = q.offset(1);
            *fresh6 = *p
        }
        p = p.offset(1);
        i += 1
    }
    /* we copy the new symbols to the old */
    memcpy(
        (*s).data as *mut std::os::raw::c_void,
        new_syms as *const std::os::raw::c_void,
        (nb_syms as std::os::raw::c_ulong)
            .wrapping_mul(::std::mem::size_of::<Elf64_Sym>() as std::os::raw::c_ulong),
    );
    tcc_free(new_syms as *mut std::os::raw::c_void);
    /* now we modify all the relocations */
    i = 1 as std::os::raw::c_int;
    while i < (*s1).nb_sections {
        sr = *(*s1).sections.offset(i as isize);
        if (*sr).sh_type == 4 as std::os::raw::c_int && (*sr).link == s {
            rel = ((*sr).data as *mut Elf64_Rela).offset(0 as std::os::raw::c_int as isize);
            while rel < (*sr).data.offset((*sr).data_offset as isize) as *mut Elf64_Rela {
                sym_index = ((*rel).r_info >> 32 as std::os::raw::c_int) as std::os::raw::c_int;
                type_0 = ((*rel).r_info
                    & 0xffffffff as std::os::raw::c_uint as std::os::raw::c_ulong)
                    as std::os::raw::c_int;
                sym_index = *old_to_new_syms.offset(sym_index as isize);
                (*rel).r_info = ((sym_index as Elf64_Xword) << 32 as std::os::raw::c_int)
                    .wrapping_add(type_0 as std::os::raw::c_ulong);
                rel = rel.offset(1)
            }
        }
        i += 1
    }
    tcc_free(old_to_new_syms as *mut std::os::raw::c_void);
}
/* relocate symbol table, resolve undefined symbols if do_resolve is
true and output error if undefined symbol. */
#[no_mangle]
pub unsafe extern "C" fn relocate_syms(
    mut s1: *mut TCCState,
    mut symtab: *mut Section,
    mut do_resolve: std::os::raw::c_int,
) {
    let mut current_block: u64;
    let mut sym: *mut Elf64_Sym = 0 as *mut Elf64_Sym;
    let mut sym_bind: std::os::raw::c_int = 0;
    let mut sh_num: std::os::raw::c_int = 0;
    let mut name: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    sym = ((*symtab).data as *mut Elf64_Sym).offset(1 as std::os::raw::c_int as isize);
    while sym < (*symtab).data.offset((*symtab).data_offset as isize) as *mut Elf64_Sym {
        sh_num = (*sym).st_shndx as std::os::raw::c_int;
        if sh_num == 0 as std::os::raw::c_int {
            name = ((*(*(*s1).symtab).link).data as *mut std::os::raw::c_char)
                .offset((*sym).st_name as isize);
            /* Use ld.so to resolve symbol for us (for tcc -run) */
            if do_resolve != 0 {
                /* dlsym() needs the undecorated name.  */
                let mut addr: *mut std::os::raw::c_void = dlsym(
                    0 as *mut std::os::raw::c_void,
                    &*name.offset((*s1).leading_underscore as isize),
                );
                if !addr.is_null() {
                    (*sym).st_value = addr as Elf64_Addr;
                    current_block = 14916268686031723178;
                } else {
                    current_block = 5399440093318478209;
                }
            /* if dynamic symbol exist, it will be used in relocate_section */
            } else if !(*s1).dynsym.is_null() && find_elf_sym((*s1).dynsym, name) != 0 {
                current_block = 14916268686031723178;
            } else {
                current_block = 5399440093318478209;
            }
            match current_block {
                14916268686031723178 => {},
                _ =>
                /* XXX: _fp_hw seems to be part of the ABI, so we ignore
                it */
                {
                    if !(strcmp(
                        name,
                        b"_fp_hw\x00" as *const u8 as *const std::os::raw::c_char,
                    ) == 0)
                    {
                        /* only weak symbols are accepted to be undefined. Their
                        value is zero */
                        sym_bind =
                            (*sym).st_info as std::os::raw::c_int >> 4 as std::os::raw::c_int;
                        if sym_bind == 2 as std::os::raw::c_int {
                            (*sym).st_value = 0 as std::os::raw::c_int as Elf64_Addr
                        } else {
                            tcc_enter_state(s1);
                            Some(
                                _tcc_error_noabort
                                    as unsafe extern "C" fn(
                                        _: *const std::os::raw::c_char,
                                        _: ...
                                    )
                                        -> (),
                            )
                            .expect("non-null function pointer")(
                                b"undefined symbol \'%s\'\x00" as *const u8
                                    as *const std::os::raw::c_char,
                                name,
                            );
                        }
                    }
                }
            }
        } else if sh_num < 0xff00 as std::os::raw::c_int {
            /* add section base */
            (*sym).st_value = ((*sym).st_value as std::os::raw::c_ulong)
                .wrapping_add((**(*s1).sections.offset((*sym).st_shndx as isize)).sh_addr)
                as Elf64_Addr as Elf64_Addr
        }
        sym = sym.offset(1)
    }
}
/* relocate a given section (CPU dependent) by applying the relocations
in the associated relocation section */
unsafe extern "C" fn relocate_section(
    mut s1: *mut TCCState,
    mut s: *mut Section,
    mut sr: *mut Section,
) {
    let mut rel: *mut Elf64_Rela = 0 as *mut Elf64_Rela;
    let mut sym: *mut Elf64_Sym = 0 as *mut Elf64_Sym;
    let mut type_0: std::os::raw::c_int = 0;
    let mut sym_index: std::os::raw::c_int = 0;
    let mut ptr: *mut std::os::raw::c_uchar = 0 as *mut std::os::raw::c_uchar;
    let mut tgt: Elf64_Addr = 0;
    let mut addr: Elf64_Addr = 0;
    (*s1).qrel = (*sr).data as *mut Elf64_Rela;
    rel = ((*sr).data as *mut Elf64_Rela).offset(0 as std::os::raw::c_int as isize);
    while rel < (*sr).data.offset((*sr).data_offset as isize) as *mut Elf64_Rela {
        ptr = (*s).data.offset((*rel).r_offset as isize);
        sym_index = ((*rel).r_info >> 32 as std::os::raw::c_int) as std::os::raw::c_int;
        sym = &mut *((*(*s1).symtab_section).data as *mut Elf64_Sym).offset(sym_index as isize)
            as *mut Elf64_Sym;
        type_0 = ((*rel).r_info & 0xffffffff as std::os::raw::c_uint as std::os::raw::c_ulong)
            as std::os::raw::c_int;
        tgt = (*sym).st_value;
        tgt = (tgt as std::os::raw::c_ulong).wrapping_add((*rel).r_addend as std::os::raw::c_ulong)
            as Elf64_Addr as Elf64_Addr;
        addr = (*s).sh_addr.wrapping_add((*rel).r_offset);
        relocate(s1, rel, type_0, ptr, addr, tgt);
        rel = rel.offset(1)
    }
    /* if the relocation is allocated, we change its symbol table */
    if (*sr).sh_flags & (1 as std::os::raw::c_int) << 1 as std::os::raw::c_int != 0 {
        (*sr).link = (*s1).dynsym; /* cannot apply 64bit relocation to 32bit value */
        if (*s1).output_type == 3 as std::os::raw::c_int {
            let mut r: size_t = ((*s1).qrel as *mut uint8_t).offset_from((*sr).data)
                as std::os::raw::c_long as size_t;
            if (::std::mem::size_of::<std::os::raw::c_uint>() as std::os::raw::c_ulong)
                < 8 as std::os::raw::c_int as std::os::raw::c_ulong
                && 0 as std::os::raw::c_int
                    == strcmp(
                        (*s).name.as_mut_ptr(),
                        b".stab\x00" as *const u8 as *const std::os::raw::c_char,
                    )
            {
                r = 0 as std::os::raw::c_int as size_t
            }
            (*sr).sh_size = r;
            (*sr).data_offset = (*sr).sh_size
        }
    };
}
/* relocate all sections */
#[no_mangle]
pub unsafe extern "C" fn relocate_sections(mut s1: *mut TCCState) {
    let mut i: std::os::raw::c_int = 0;
    let mut s: *mut Section = 0 as *mut Section;
    let mut sr: *mut Section = 0 as *mut Section;
    i = 1 as std::os::raw::c_int;
    while i < (*s1).nb_sections {
        sr = *(*s1).sections.offset(i as isize);
        if !((*sr).sh_type != 4 as std::os::raw::c_int) {
            s = *(*s1).sections.offset((*sr).sh_info as isize);
            if s != (*s1).got
                || (*s1).static_link as std::os::raw::c_int != 0
                || (*s1).output_type == 1 as std::os::raw::c_int
            {
                relocate_section(s1, s, sr);
            }
            if (*sr).sh_flags & (1 as std::os::raw::c_int) << 1 as std::os::raw::c_int != 0 {
                let mut rel: *mut Elf64_Rela = 0 as *mut Elf64_Rela;
                /* relocate relocation table in 'sr' */
                rel = ((*sr).data as *mut Elf64_Rela).offset(0 as std::os::raw::c_int as isize);
                while rel < (*sr).data.offset((*sr).data_offset as isize) as *mut Elf64_Rela {
                    (*rel).r_offset = ((*rel).r_offset as std::os::raw::c_ulong)
                        .wrapping_add((*s).sh_addr)
                        as Elf64_Addr as Elf64_Addr;
                    rel = rel.offset(1)
                }
            }
        }
        i += 1
    }
}
/* count the number of dynamic relocations so that we can reserve
their space */
unsafe extern "C" fn prepare_dynamic_rel(
    mut s1: *mut TCCState,
    mut sr: *mut Section,
) -> std::os::raw::c_int {
    let mut count: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut rel: *mut Elf64_Rela = 0 as *mut Elf64_Rela;
    rel = ((*sr).data as *mut Elf64_Rela).offset(0 as std::os::raw::c_int as isize);
    while rel < (*sr).data.offset((*sr).data_offset as isize) as *mut Elf64_Rela {
        let mut sym_index: std::os::raw::c_int =
            ((*rel).r_info >> 32 as std::os::raw::c_int) as std::os::raw::c_int;
        let mut type_0: std::os::raw::c_int = ((*rel).r_info
            & 0xffffffff as std::os::raw::c_uint as std::os::raw::c_ulong)
            as std::os::raw::c_int;
        match type_0 {
            10 | 11 | 1 => count += 1,
            2 => {
                let mut sym: *mut Elf64_Sym = &mut *((*(*s1).symtab_section).data as *mut Elf64_Sym)
                    .offset(sym_index as isize)
                    as *mut Elf64_Sym;
                /* Hidden defined symbols can and must be resolved locally.
                We're misusing a PLT32 reloc for this, as that's always
                resolved to its address even in shared libs.  */
                if (*sym).st_shndx as std::os::raw::c_int != 0 as std::os::raw::c_int
                    && (*sym).st_other as std::os::raw::c_int & 0x3 as std::os::raw::c_int
                        == 2 as std::os::raw::c_int
                {
                    (*rel).r_info = ((sym_index as Elf64_Xword) << 32 as std::os::raw::c_int)
                        .wrapping_add(4 as std::os::raw::c_int as std::os::raw::c_ulong)
                } else if (*get_sym_attr(s1, sym_index, 0 as std::os::raw::c_int)).dyn_index != 0 {
                    count += 1
                }
            },
            _ => {},
        }
        rel = rel.offset(1)
    }
    return count;
}
unsafe extern "C" fn build_got(mut s1: *mut TCCState) {
    /* if no got, then create it */
    (*s1).got = new_section(
        s1,
        b".got\x00" as *const u8 as *const std::os::raw::c_char,
        1 as std::os::raw::c_int,
        (1 as std::os::raw::c_int) << 1 as std::os::raw::c_int
            | (1 as std::os::raw::c_int) << 0 as std::os::raw::c_int,
    );
    (*(*s1).got).sh_entsize = 4 as std::os::raw::c_int;
    set_elf_sym(
        (*s1).symtab_section,
        0 as std::os::raw::c_int as Elf64_Addr,
        4 as std::os::raw::c_int as std::os::raw::c_ulong,
        ((1 as std::os::raw::c_int) << 4 as std::os::raw::c_int)
            + (1 as std::os::raw::c_int & 0xf as std::os::raw::c_int),
        0 as std::os::raw::c_int,
        (*(*s1).got).sh_num,
        b"_GLOBAL_OFFSET_TABLE_\x00" as *const u8 as *const std::os::raw::c_char,
    );
    /* keep space for _DYNAMIC pointer and two dummy got entries */
    section_ptr_add(
        (*s1).got,
        (3 as std::os::raw::c_int * 8 as std::os::raw::c_int) as Elf64_Addr,
    );
}
/* Create a GOT and (for function call) a PLT entry corresponding to a symbol
in s1->symtab. When creating the dynamic symbol table entry for the GOT
relocation, use 'size' and 'info' for the corresponding symbol metadata.
Returns the offset of the GOT or (if any) PLT entry. */
unsafe extern "C" fn put_got_entry(
    mut s1: *mut TCCState,
    mut dyn_reloc_type: std::os::raw::c_int,
    mut sym_index: std::os::raw::c_int,
) -> *mut sym_attr {
    let mut need_plt_entry: std::os::raw::c_int = 0;
    let mut name: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut sym: *mut Elf64_Sym = 0 as *mut Elf64_Sym;
    let mut attr: *mut sym_attr = 0 as *mut sym_attr;
    let mut got_offset: std::os::raw::c_uint = 0;
    let mut plt_name: [std::os::raw::c_char; 100] = [0; 100];
    let mut len: std::os::raw::c_int = 0;
    let mut s_rel: *mut Section = 0 as *mut Section;
    need_plt_entry = (dyn_reloc_type == 7 as std::os::raw::c_int) as std::os::raw::c_int;
    attr = get_sym_attr(s1, sym_index, 1 as std::os::raw::c_int);
    /* In case a function is both called and its address taken 2 GOT entries
    are created, one for taking the address (GOT) and the other for the PLT
    entry (PLTGOT).  */
    if if need_plt_entry != 0 {
        (*attr).plt_offset
    } else {
        (*attr).got_offset
    } != 0
    {
        return attr;
    }
    s_rel = (*s1).got;
    if need_plt_entry != 0 {
        if (*s1).plt.is_null() {
            (*s1).plt = new_section(
                s1,
                b".plt\x00" as *const u8 as *const std::os::raw::c_char,
                1 as std::os::raw::c_int,
                (1 as std::os::raw::c_int) << 1 as std::os::raw::c_int
                    | (1 as std::os::raw::c_int) << 2 as std::os::raw::c_int,
            );
            (*(*s1).plt).sh_entsize = 4 as std::os::raw::c_int
        }
        s_rel = (*s1).plt
    }
    /* create the GOT entry */
    got_offset = (*(*s1).got).data_offset as std::os::raw::c_uint;
    section_ptr_add((*s1).got, 8 as std::os::raw::c_int as Elf64_Addr);
    /* Create the GOT relocation that will insert the address of the object or
    function of interest in the GOT entry. This is a static relocation for
    memory output (dlsym will give us the address of symbols) and dynamic
    relocation otherwise (executable and DLLs). The relocation should be
    done lazily for GOT entry with *_JUMP_SLOT relocation type (the one
    associated to a PLT entry) but is currently done at load time for an
    unknown reason. */
    sym = &mut *((*(*s1).symtab_section).data as *mut Elf64_Sym).offset(sym_index as isize)
        as *mut Elf64_Sym;
    name = ((*(*(*s1).symtab_section).link).data as *mut std::os::raw::c_char)
        .offset((*sym).st_name as isize);
    //printf("sym %d %s\n", need_plt_entry, name);
    if !(*s1).dynsym.is_null() {
        if (*sym).st_info as std::os::raw::c_int >> 4 as std::os::raw::c_int
            == 0 as std::os::raw::c_int
        {
            /* Hack alarm.  We don't want to emit dynamic symbols
            and symbol based relocs for STB_LOCAL symbols, but rather
            want to resolve them directly.  At this point the symbol
            values aren't final yet, so we must defer this.  We will later
            have to create a RELATIVE reloc anyway, so we misuse the
            relocation slot to smuggle the symbol reference until
            fill_local_got_entries.  Not that the sym_index is
            relative to symtab_section, not s1->dynsym!  Nevertheless
            we use s1->dyn_sym so that if this is the first call
            that got->reloc is correctly created.  Also note that
            RELATIVE relocs are not normally created for the .got,
            so the types serves as a marker for later (and is retained
            also for the final output, which is okay because then the
            got is just normal data).  */
            put_elf_reloc(
                (*s1).dynsym,
                (*s1).got,
                got_offset as std::os::raw::c_ulong,
                8 as std::os::raw::c_int,
                sym_index,
            );
        } else {
            if 0 as std::os::raw::c_int == (*attr).dyn_index {
                (*attr).dyn_index = set_elf_sym(
                    (*s1).dynsym,
                    (*sym).st_value,
                    (*sym).st_size,
                    (*sym).st_info as std::os::raw::c_int,
                    0 as std::os::raw::c_int,
                    (*sym).st_shndx as std::os::raw::c_int,
                    name,
                )
            }
            put_elf_reloc(
                (*s1).dynsym,
                s_rel,
                got_offset as std::os::raw::c_ulong,
                dyn_reloc_type,
                (*attr).dyn_index,
            );
        }
    } else {
        put_elf_reloc(
            (*s1).symtab_section,
            (*s1).got,
            got_offset as std::os::raw::c_ulong,
            dyn_reloc_type,
            sym_index,
        );
    }
    if need_plt_entry != 0 {
        (*attr).plt_offset = create_plt_entry(s1, got_offset, attr);
        /* create a symbol 'sym@plt' for the PLT jump vector */
        len = strlen(name) as std::os::raw::c_int;
        if len as std::os::raw::c_ulong
            > (::std::mem::size_of::<[std::os::raw::c_char; 100]>() as std::os::raw::c_ulong)
                .wrapping_sub(5 as std::os::raw::c_int as std::os::raw::c_ulong)
        {
            len = (::std::mem::size_of::<[std::os::raw::c_char; 100]>() as std::os::raw::c_ulong)
                .wrapping_sub(5 as std::os::raw::c_int as std::os::raw::c_ulong)
                as std::os::raw::c_int
        }
        memcpy(
            plt_name.as_mut_ptr() as *mut std::os::raw::c_void,
            name as *const std::os::raw::c_void,
            len as std::os::raw::c_ulong,
        );
        strcpy(
            plt_name.as_mut_ptr().offset(len as isize),
            b"@plt\x00" as *const u8 as *const std::os::raw::c_char,
        );
        (*attr).plt_sym = put_elf_sym(
            (*s1).symtab,
            (*attr).plt_offset as Elf64_Addr,
            (*sym).st_size,
            ((1 as std::os::raw::c_int) << 4 as std::os::raw::c_int)
                + (2 as std::os::raw::c_int & 0xf as std::os::raw::c_int),
            0 as std::os::raw::c_int,
            (*(*s1).plt).sh_num,
            plt_name.as_mut_ptr(),
        )
    } else {
        (*attr).got_offset = got_offset
    }
    return attr;
}
/* build GOT and PLT entries */
/* Two passes because R_JMP_SLOT should become first. Some targets
(arm, arm64) do not allow mixing R_JMP_SLOT and R_GLOB_DAT. */
#[no_mangle]
pub unsafe extern "C" fn build_got_entries(mut s1: *mut TCCState) {
    let mut s: *mut Section = 0 as *mut Section;
    let mut rel: *mut Elf64_Rela = 0 as *mut Elf64_Rela;
    let mut sym: *mut Elf64_Sym = 0 as *mut Elf64_Sym;
    let mut i: std::os::raw::c_int = 0;
    let mut type_0: std::os::raw::c_int = 0;
    let mut gotplt_entry: std::os::raw::c_int = 0;
    let mut reloc_type: std::os::raw::c_int = 0;
    let mut sym_index: std::os::raw::c_int = 0;
    let mut attr: *mut sym_attr = 0 as *mut sym_attr;
    let mut pass: std::os::raw::c_int = 0 as std::os::raw::c_int;
    loop {
        i = 1 as std::os::raw::c_int;
        while i < (*s1).nb_sections {
            s = *(*s1).sections.offset(i as isize);
            if !((*s).sh_type != 4 as std::os::raw::c_int) {
                /* no need to handle got relocations */
                if !((*s).link != (*s1).symtab_section) {
                    let mut current_block_21: u64;
                    rel = ((*s).data as *mut Elf64_Rela).offset(0 as std::os::raw::c_int as isize);
                    while rel < (*s).data.offset((*s).data_offset as isize) as *mut Elf64_Rela {
                        type_0 = ((*rel).r_info
                            & 0xffffffff as std::os::raw::c_uint as std::os::raw::c_ulong)
                            as std::os::raw::c_int;
                        gotplt_entry = gotplt_entry_type(type_0);
                        if gotplt_entry == -(1 as std::os::raw::c_int) {
                            tcc_enter_state(s1);
                            Some(
                                _tcc_error
                                    as unsafe extern "C" fn(
                                        _: *const std::os::raw::c_char,
                                        _: ...
                                    )
                                        -> !,
                            )
                            .expect("non-null function pointer")(
                                b"Unknown relocation type for got: %d\x00" as *const u8
                                    as *const std::os::raw::c_char,
                                type_0,
                            );
                        }
                        sym_index =
                            ((*rel).r_info >> 32 as std::os::raw::c_int) as std::os::raw::c_int;
                        sym = &mut *((*(*s1).symtab_section).data as *mut Elf64_Sym)
                            .offset(sym_index as isize)
                            as *mut Elf64_Sym;
                        if !(gotplt_entry == NO_GOTPLT_ENTRY as std::os::raw::c_int) {
                            /* Automatically create PLT/GOT [entry] if it is an undefined
                            reference (resolved at runtime), or the symbol is absolute,
                            probably created by tcc_add_symbol, and thus on 64-bit
                            targets might be too far from application code.  */
                            if gotplt_entry == AUTO_GOTPLT_ENTRY as std::os::raw::c_int {
                                if (*sym).st_shndx as std::os::raw::c_int
                                    == 0 as std::os::raw::c_int
                                {
                                    let mut esym: *mut Elf64_Sym = 0 as *mut Elf64_Sym;
                                    let mut dynindex: std::os::raw::c_int = 0;
                                    if (*s1).output_type == 3 as std::os::raw::c_int
                                        && 1 as std::os::raw::c_int == 0
                                    {
                                        current_block_21 = 1394248824506584008;
                                    } else if !(*s1).dynsym.is_null() {
                                        /* Relocations for UNDEF symbols would normally need
                                        to be transferred into the executable or shared object.
                                        If that were done AUTO_GOTPLT_ENTRY wouldn't exist.
                                        But TCC doesn't do that (at least for exes), so we
                                        need to resolve all such relocs locally.  And that
                                        means PLT slots for functions in DLLs and COPY relocs for
                                        data symbols.  COPY relocs were generated in
                                        bind_exe_dynsyms (and the symbol adjusted to be defined),
                                        and for functions we were generated a dynamic symbol
                                        of function type.  */
                                        /* dynsym isn't set for -run :-/  */
                                        dynindex = (*get_sym_attr(
                                            s1,
                                            sym_index,
                                            0 as std::os::raw::c_int,
                                        ))
                                        .dyn_index;
                                        esym = ((*(*s1).dynsym).data as *mut Elf64_Sym)
                                            .offset(dynindex as isize);
                                        if dynindex != 0
                                            && ((*esym).st_info as std::os::raw::c_int
                                                & 0xf as std::os::raw::c_int
                                                == 2 as std::os::raw::c_int
                                                || (*esym).st_info as std::os::raw::c_int
                                                    & 0xf as std::os::raw::c_int
                                                    == 0 as std::os::raw::c_int
                                                    && (*sym).st_info as std::os::raw::c_int
                                                        & 0xf as std::os::raw::c_int
                                                        == 2 as std::os::raw::c_int)
                                        {
                                            current_block_21 = 13126051560704620981;
                                        } else {
                                            current_block_21 = 15897653523371991391;
                                        }
                                    } else {
                                        current_block_21 = 15897653523371991391;
                                    }
                                } else if (*sym).st_shndx as std::os::raw::c_int
                                    == 0xfff1 as std::os::raw::c_int
                                {
                                    if (*sym).st_value
                                        == 0 as std::os::raw::c_int as std::os::raw::c_ulong
                                    {
                                        current_block_21 = 1394248824506584008;
                                    } else if 8 as std::os::raw::c_int != 8 as std::os::raw::c_int {
                                        current_block_21 = 1394248824506584008;
                                    } else {
                                        current_block_21 = 15897653523371991391;
                                    }
                                /* from tcc_add_symbol(): on 64 bit platforms these
                                need to go through .got */
                                } else {
                                    current_block_21 = 1394248824506584008;
                                }
                            } else {
                                current_block_21 = 15897653523371991391;
                            }
                            match current_block_21 {
                                1394248824506584008 => {},
                                _ => {
                                    match current_block_21 {
                                        15897653523371991391 => {
                                            if (type_0 == 4 as std::os::raw::c_int
                                                || type_0 == 2 as std::os::raw::c_int)
                                                && (*sym).st_shndx as std::os::raw::c_int
                                                    != 0 as std::os::raw::c_int
                                                && ((*sym).st_other as std::os::raw::c_int
                                                    & 0x3 as std::os::raw::c_int
                                                    != 0 as std::os::raw::c_int
                                                    || (*sym).st_info as std::os::raw::c_int
                                                        >> 4 as std::os::raw::c_int
                                                        == 0 as std::os::raw::c_int
                                                    || (*s1).output_type
                                                        == 2 as std::os::raw::c_int)
                                            {
                                                if pass != 0 as std::os::raw::c_int {
                                                    current_block_21 = 1394248824506584008;
                                                } else {
                                                    (*rel).r_info = ((sym_index as Elf64_Xword)
                                                        << 32 as std::os::raw::c_int)
                                                        .wrapping_add(
                                                            2 as std::os::raw::c_int
                                                                as std::os::raw::c_ulong,
                                                        );
                                                    current_block_21 = 1394248824506584008;
                                                }
                                            } else {
                                                reloc_type = code_reloc(type_0);
                                                if reloc_type == -(1 as std::os::raw::c_int) {
                                                    tcc_enter_state(s1);
                                                    Some(
                                                        _tcc_error
                                                            as unsafe extern "C" fn(
                                                                _: *const std::os::raw::c_char,
                                                                _: ...
                                                            )
                                                                -> !,
                                                    )
                                                    .expect("non-null function pointer")(
                                                        b"Unknown relocation type: %d\x00"
                                                            as *const u8
                                                            as *const std::os::raw::c_char,
                                                        type_0,
                                                    );
                                                }
                                                if reloc_type != 0 as std::os::raw::c_int {
                                                    current_block_21 = 13126051560704620981;
                                                } else if pass != 1 as std::os::raw::c_int {
                                                    current_block_21 = 1394248824506584008;
                                                } else {
                                                    reloc_type = 6 as std::os::raw::c_int;
                                                    current_block_21 = 1622411330066726685;
                                                }
                                            }
                                        },
                                        _ => {},
                                    }
                                    match current_block_21 {
                                        1394248824506584008 => {},
                                        _ => {
                                            match current_block_21 {
                                                13126051560704620981 => {
                                                    if pass != 0 as std::os::raw::c_int {
                                                        current_block_21 = 1394248824506584008;
                                                    } else {
                                                        reloc_type = 7 as std::os::raw::c_int;
                                                        current_block_21 = 1622411330066726685;
                                                    }
                                                },
                                                _ => {},
                                            }
                                            match current_block_21 {
                                                1394248824506584008 => {},
                                                _ => {
                                                    if (*s1).got.is_null() {
                                                        build_got(s1);
                                                    }
                                                    if !(gotplt_entry
                                                        == BUILD_GOT_ONLY as std::os::raw::c_int)
                                                    {
                                                        attr = put_got_entry(
                                                            s1, reloc_type, sym_index,
                                                        );
                                                        if reloc_type == 7 as std::os::raw::c_int {
                                                            (*rel).r_info = (((*attr).plt_sym
                                                                as Elf64_Xword)
                                                                << 32 as std::os::raw::c_int)
                                                                .wrapping_add(
                                                                    type_0 as std::os::raw::c_ulong,
                                                                )
                                                        }
                                                    }
                                                },
                                            }
                                        },
                                    }
                                },
                            }
                        }
                        /* from tcc_add_btstub() */
                        rel = rel.offset(1)
                    }
                }
            }
            i += 1
        }
        pass += 1;
        if !(pass < 2 as std::os::raw::c_int) {
            break;
        }
    }
    /* .rel.plt refers to .got actually */
    if !(*s1).plt.is_null() && !(*(*s1).plt).reloc.is_null() {
        (*(*(*s1).plt).reloc).sh_info = (*(*s1).got).sh_num
    };
}
#[no_mangle]
pub unsafe extern "C" fn set_global_sym(
    mut s1: *mut TCCState,
    mut name: *const std::os::raw::c_char,
    mut sec: *mut Section,
    mut offs: Elf64_Addr,
) -> std::os::raw::c_int {
    let mut shn: std::os::raw::c_int = if !sec.is_null() {
        (*sec).sh_num
    } else if offs != 0 || name.is_null() {
        0xfff1 as std::os::raw::c_int
    } else {
        0 as std::os::raw::c_int
    };
    if !sec.is_null() && offs == -(1 as std::os::raw::c_int) as std::os::raw::c_ulong {
        offs = (*sec).data_offset
    }
    return set_elf_sym(
        (*s1).symtab_section,
        offs,
        0 as std::os::raw::c_int as std::os::raw::c_ulong,
        ((if !name.is_null() {
            1 as std::os::raw::c_int
        } else {
            0 as std::os::raw::c_int
        }) << 4 as std::os::raw::c_int)
            + (0 as std::os::raw::c_int & 0xf as std::os::raw::c_int),
        0 as std::os::raw::c_int,
        shn,
        name,
    );
}
unsafe extern "C" fn add_init_array_defines(
    mut s1: *mut TCCState,
    mut section_name: *const std::os::raw::c_char,
) {
    let mut s: *mut Section = 0 as *mut Section;
    let mut end_offset: Elf64_Addr = 0;
    let mut buf: [std::os::raw::c_char; 1024] = [0; 1024];
    s = find_section_create(s1, section_name, 0 as std::os::raw::c_int);
    if s.is_null() {
        end_offset = 0 as std::os::raw::c_int as Elf64_Addr;
        s = (*s1).data_section
    } else {
        end_offset = (*s).data_offset
    }
    snprintf(
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[std::os::raw::c_char; 1024]>() as std::os::raw::c_ulong,
        b"__%s_start\x00" as *const u8 as *const std::os::raw::c_char,
        section_name.offset(1 as std::os::raw::c_int as isize),
    );
    set_global_sym(
        s1,
        buf.as_mut_ptr(),
        s,
        0 as std::os::raw::c_int as Elf64_Addr,
    );
    snprintf(
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[std::os::raw::c_char; 1024]>() as std::os::raw::c_ulong,
        b"__%s_end\x00" as *const u8 as *const std::os::raw::c_char,
        section_name.offset(1 as std::os::raw::c_int as isize),
    );
    set_global_sym(s1, buf.as_mut_ptr(), s, end_offset);
}
unsafe extern "C" fn tcc_add_support(
    mut s1: *mut TCCState,
    mut filename: *const std::os::raw::c_char,
) {
    let mut buf: [std::os::raw::c_char; 1024] = [0; 1024];
    snprintf(
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[std::os::raw::c_char; 1024]>() as std::os::raw::c_ulong,
        b"%s/%s\x00" as *const u8 as *const std::os::raw::c_char,
        (*s1).tcc_lib_path,
        filename,
    );
    tcc_add_file(s1, buf.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn add_array(
    mut s1: *mut TCCState,
    mut sec: *const std::os::raw::c_char,
    mut c: std::os::raw::c_int,
) {
    let mut s: *mut Section = 0 as *mut Section;
    s = find_section(s1, sec);
    (*s).sh_flags |= (1 as std::os::raw::c_int) << 0 as std::os::raw::c_int;
    (*s).sh_type =
        if *sec.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int == 'i' as i32 {
            14 as std::os::raw::c_int
        } else {
            15 as std::os::raw::c_int
        };
    put_elf_reloc(
        (*s1).symtab,
        s,
        (*s).data_offset,
        1 as std::os::raw::c_int,
        c,
    );
    section_ptr_add(s, 8 as std::os::raw::c_int as Elf64_Addr);
}
#[no_mangle]
pub unsafe extern "C" fn tcc_add_bcheck(mut s1: *mut TCCState) {
    if 0 as std::os::raw::c_int == (*s1).do_bounds_check as std::os::raw::c_int {
        return;
    }
    section_ptr_add(
        (*s1).bounds_section,
        ::std::mem::size_of::<Elf64_Addr>() as std::os::raw::c_ulong,
    );
}
/* set symbol to STB_LOCAL and resolve. The point is to not export it as
a dynamic symbol to allow so's to have one each with a different value. */
unsafe extern "C" fn set_local_sym(
    mut s1: *mut TCCState,
    mut name: *const std::os::raw::c_char,
    mut s: *mut Section,
    mut offset: std::os::raw::c_int,
) {
    let mut c: std::os::raw::c_int = find_elf_sym((*s1).symtab, name);
    if c != 0 {
        let mut esym: *mut Elf64_Sym = ((*(*s1).symtab).data as *mut Elf64_Sym).offset(c as isize);
        (*esym).st_info = (((0 as std::os::raw::c_int) << 4 as std::os::raw::c_int)
            + (0 as std::os::raw::c_int & 0xf as std::os::raw::c_int))
            as std::os::raw::c_uchar;
        (*esym).st_value = offset as Elf64_Addr;
        (*esym).st_shndx = (*s).sh_num as Elf64_Section
    };
}
unsafe extern "C" fn put_ptr(
    mut s1: *mut TCCState,
    mut s: *mut Section,
    mut offs: std::os::raw::c_int,
) {
    let mut c: std::os::raw::c_int = 0;
    c = set_global_sym(s1, 0 as *const std::os::raw::c_char, s, offs as Elf64_Addr);
    s = (*s1).data_section;
    put_elf_reloc(
        (*s1).symtab,
        s,
        (*s).data_offset,
        1 as std::os::raw::c_int,
        c,
    );
    section_ptr_add(s, 8 as std::os::raw::c_int as Elf64_Addr);
}
#[no_mangle]
pub unsafe extern "C" fn tcc_add_btstub(mut s1: *mut TCCState) {
    let mut s: *mut Section = 0 as *mut Section;
    let mut n: std::os::raw::c_int = 0;
    let mut o: std::os::raw::c_int = 0;
    let mut cstr: CString = CString {
        size: 0,
        data: 0 as *mut std::os::raw::c_void,
        size_allocated: 0,
    };
    s = (*s1).data_section;
    /* Align to PTR_SIZE */
    section_ptr_add(
        s,
        (*s).data_offset.wrapping_neg()
            & (8 as std::os::raw::c_int - 1 as std::os::raw::c_int) as std::os::raw::c_ulong,
    );
    o = (*s).data_offset as std::os::raw::c_int;
    /* create (part of) a struct rt_context (see tccrun.c) */
    put_ptr(s1, (*s1).stab_section, 0 as std::os::raw::c_int);
    put_ptr(s1, (*s1).stab_section, -(1 as std::os::raw::c_int));
    put_ptr(s1, (*(*s1).stab_section).link, 0 as std::os::raw::c_int);
    section_ptr_add(
        s,
        (3 as std::os::raw::c_int * 8 as std::os::raw::c_int) as Elf64_Addr,
    );
    /* prog_base : local nameless symbol with offset 0 at SHN_ABS */
    put_ptr(s1, 0 as *mut Section, 0 as std::os::raw::c_int);
    n = 2 as std::os::raw::c_int * 8 as std::os::raw::c_int;
    if (*s1).do_bounds_check != 0 {
        put_ptr(s1, (*s1).bounds_section, 0 as std::os::raw::c_int);
        n -= 8 as std::os::raw::c_int
    }
    section_ptr_add(s, n as Elf64_Addr);
    cstr_new(&mut cstr);
    cstr_printf(&mut cstr as *mut CString,
                b"extern void __bt_init(),__bt_init_dll();static void *__rt_info[];__attribute__((constructor)) static void __bt_init_rt(){\x00"
                    as *const u8 as *const std::os::raw::c_char);
    cstr_printf(
        &mut cstr as *mut CString,
        b"__bt_init(__rt_info,%d);}\x00" as *const u8 as *const std::os::raw::c_char,
        if (*s1).output_type == 3 as std::os::raw::c_int {
            0 as std::os::raw::c_int
        } else {
            ((*s1).rt_num_callers) + 1 as std::os::raw::c_int
        },
    );
    tcc_compile_string(s1, cstr.data as *const std::os::raw::c_char);
    cstr_free(&mut cstr);
    set_local_sym(
        s1,
        &*(b"___rt_info\x00" as *const u8 as *const std::os::raw::c_char)
            .offset(((*s1).leading_underscore == 0) as std::os::raw::c_int as isize),
        s,
        o,
    );
}
unsafe extern "C" fn tcc_tcov_add_file(
    mut s1: *mut TCCState,
    mut filename: *const std::os::raw::c_char,
) {
    let mut cstr: CString = CString {
        size: 0,
        data: 0 as *mut std::os::raw::c_void,
        size_allocated: 0,
    };
    let mut ptr: *mut std::os::raw::c_void = 0 as *mut std::os::raw::c_void;
    let mut wd: [std::os::raw::c_char; 1024] = [0; 1024];
    if (*s1).tcov_section.is_null() {
        return;
    }
    section_ptr_add((*s1).tcov_section, 1 as std::os::raw::c_int as Elf64_Addr);
    write32le(
        (*(*s1).tcov_section).data,
        (*(*s1).tcov_section).data_offset as uint32_t,
    );
    cstr_new(&mut cstr);
    if *filename.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int == '/' as i32 {
        cstr_printf(
            &mut cstr as *mut CString,
            b"%s.tcov\x00" as *const u8 as *const std::os::raw::c_char,
            filename,
        );
    } else {
        getcwd(
            wd.as_mut_ptr(),
            ::std::mem::size_of::<[std::os::raw::c_char; 1024]>() as std::os::raw::c_ulong,
        );
        cstr_printf(
            &mut cstr as *mut CString,
            b"%s/%s.tcov\x00" as *const u8 as *const std::os::raw::c_char,
            wd.as_mut_ptr(),
            filename,
        );
    }
    ptr = section_ptr_add(
        (*s1).tcov_section,
        (cstr.size + 1 as std::os::raw::c_int) as Elf64_Addr,
    );
    strncpy(
        ptr as *mut std::os::raw::c_char,
        cstr.data as *const std::os::raw::c_char,
        cstr.size as std::os::raw::c_ulong,
    );
    unlink(ptr as *mut std::os::raw::c_char);
    cstr_free(&mut cstr);
    cstr_new(&mut cstr);
    cstr_printf(&mut cstr as *mut CString,
                b"extern char *__tcov_data[];extern void __store_test_coverage ();__attribute__((destructor)) static void __tcov_exit() {__store_test_coverage(__tcov_data);}\x00"
                    as *const u8 as *const std::os::raw::c_char);
    tcc_compile_string(s1, cstr.data as *const std::os::raw::c_char);
    cstr_free(&mut cstr);
    set_local_sym(
        s1,
        &*(b"___tcov_data\x00" as *const u8 as *const std::os::raw::c_char)
            .offset(((*s1).leading_underscore == 0) as std::os::raw::c_int as isize),
        (*s1).tcov_section,
        0 as std::os::raw::c_int,
    );
}
/* add tcc runtime libraries */
#[no_mangle]
pub unsafe extern "C" fn tcc_add_runtime(mut s1: *mut TCCState) {
    (*s1).filetype = 0 as std::os::raw::c_int as std::os::raw::c_uchar;
    tcc_add_bcheck(s1);
    tcc_add_pragma_libs(s1);
    /* add libc */
    if (*s1).nostdlib == 0 {
        if (*s1).option_pthread != 0 {
            tcc_add_library_err(
                s1,
                b"pthread\x00" as *const u8 as *const std::os::raw::c_char,
            );
        }
        tcc_add_library_err(s1, b"c\x00" as *const u8 as *const std::os::raw::c_char);
        if (*s1).do_bounds_check as std::os::raw::c_int != 0
            && (*s1).output_type != 3 as std::os::raw::c_int
        {
            tcc_add_library_err(
                s1,
                b"pthread\x00" as *const u8 as *const std::os::raw::c_char,
            );
            tcc_add_library_err(s1, b"dl\x00" as *const u8 as *const std::os::raw::c_char);
            tcc_add_support(
                s1,
                b"bcheck.o\x00" as *const u8 as *const std::os::raw::c_char,
            );
            if (*s1).static_link != 0 {
                tcc_add_library_err(s1, b"c\x00" as *const u8 as *const std::os::raw::c_char);
            }
        }
        if (*s1).do_backtrace != 0 {
            if (*s1).output_type == 2 as std::os::raw::c_int {
                tcc_add_support(
                    s1,
                    b"bt-exe.o\x00" as *const u8 as *const std::os::raw::c_char,
                );
            }
            if (*s1).output_type != 3 as std::os::raw::c_int {
                tcc_add_support(
                    s1,
                    b"bt-log.o\x00" as *const u8 as *const std::os::raw::c_char,
                );
            }
            if (*s1).output_type != 1 as std::os::raw::c_int {
                tcc_add_btstub(s1);
            }
        }
        if (*::std::mem::transmute::<&[u8; 10], &[std::os::raw::c_char; 10]>(b"libtcc1.a\x00"))
            [0 as std::os::raw::c_int as usize]
            != 0
        {
            tcc_add_support(
                s1,
                b"libtcc1.a\x00" as *const u8 as *const std::os::raw::c_char,
            );
        }
        /* add crt end if not memory output */
        if (*s1).output_type != 1 as std::os::raw::c_int {
            tcc_add_crt(
                s1,
                b"crtn.o\x00" as *const u8 as *const std::os::raw::c_char,
            );
        }
    };
}
/* add various standard linker symbols (must be done after the
sections are filled (for example after allocating common
symbols)) */
unsafe extern "C" fn tcc_add_linker_symbols(mut s1: *mut TCCState) {
    let mut buf: [std::os::raw::c_char; 1024] = [0; 1024];
    let mut i: std::os::raw::c_int = 0;
    let mut s: *mut Section = 0 as *mut Section;
    set_global_sym(
        s1,
        b"_etext\x00" as *const u8 as *const std::os::raw::c_char,
        (*s1).text_section,
        -(1 as std::os::raw::c_int) as Elf64_Addr,
    );
    set_global_sym(
        s1,
        b"_edata\x00" as *const u8 as *const std::os::raw::c_char,
        (*s1).data_section,
        -(1 as std::os::raw::c_int) as Elf64_Addr,
    );
    set_global_sym(
        s1,
        b"_end\x00" as *const u8 as *const std::os::raw::c_char,
        (*s1).bss_section,
        -(1 as std::os::raw::c_int) as Elf64_Addr,
    );
    /* horrible new standard ldscript defines */
    add_init_array_defines(
        s1,
        b".preinit_array\x00" as *const u8 as *const std::os::raw::c_char,
    );
    add_init_array_defines(
        s1,
        b".init_array\x00" as *const u8 as *const std::os::raw::c_char,
    );
    add_init_array_defines(
        s1,
        b".fini_array\x00" as *const u8 as *const std::os::raw::c_char,
    );
    /* add start and stop symbols for sections whose name can be
    expressed in C */
    i = 1 as std::os::raw::c_int;
    while i < (*s1).nb_sections {
        let mut current_block_14: u64;
        s = *(*s1).sections.offset(i as isize);
        if (*s).sh_flags & (1 as std::os::raw::c_int) << 1 as std::os::raw::c_int != 0
            && ((*s).sh_type == 1 as std::os::raw::c_int
                || (*s).sh_type == 3 as std::os::raw::c_int)
        {
            let mut p: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
            /* check if section name can be expressed in C */
            p = (*s).name.as_mut_ptr();
            loop {
                let mut c: std::os::raw::c_int = *p as std::os::raw::c_int;
                if c == 0 {
                    current_block_14 = 15652330335145281839;
                    break;
                }
                if isid(c) == 0 && isnum(c) == 0 {
                    current_block_14 = 1109700713171191020;
                    break;
                }
                p = p.offset(1)
            }
            match current_block_14 {
                1109700713171191020 => {},
                _ => {
                    snprintf(
                        buf.as_mut_ptr(),
                        ::std::mem::size_of::<[std::os::raw::c_char; 1024]>()
                            as std::os::raw::c_ulong,
                        b"__start_%s\x00" as *const u8 as *const std::os::raw::c_char,
                        (*s).name.as_mut_ptr(),
                    );
                    set_global_sym(
                        s1,
                        buf.as_mut_ptr(),
                        s,
                        0 as std::os::raw::c_int as Elf64_Addr,
                    );
                    snprintf(
                        buf.as_mut_ptr(),
                        ::std::mem::size_of::<[std::os::raw::c_char; 1024]>()
                            as std::os::raw::c_ulong,
                        b"__stop_%s\x00" as *const u8 as *const std::os::raw::c_char,
                        (*s).name.as_mut_ptr(),
                    );
                    set_global_sym(
                        s1,
                        buf.as_mut_ptr(),
                        s,
                        -(1 as std::os::raw::c_int) as Elf64_Addr,
                    );
                },
            }
        }
        i += 1
    }
}
#[no_mangle]
pub unsafe extern "C" fn resolve_common_syms(mut s1: *mut TCCState) {
    let mut sym: *mut Elf64_Sym = 0 as *mut Elf64_Sym;
    /* Allocate common symbols in BSS.  */
    sym =
        ((*(*s1).symtab_section).data as *mut Elf64_Sym).offset(1 as std::os::raw::c_int as isize);
    while sym
        < (*(*s1).symtab_section)
            .data
            .offset((*(*s1).symtab_section).data_offset as isize) as *mut Elf64_Sym
    {
        if (*sym).st_shndx as std::os::raw::c_int == 0xfff2 as std::os::raw::c_int {
            /* symbol alignment is in st_value for SHN_COMMONs */
            (*sym).st_value = section_add(
                (*s1).bss_section,
                (*sym).st_size,
                (*sym).st_value as std::os::raw::c_int,
            );
            (*sym).st_shndx = (*(*s1).bss_section).sh_num as Elf64_Section
        }
        sym = sym.offset(1)
    }
    /* Now assign linker provided symbols their value.  */
    tcc_add_linker_symbols(s1);
}
#[no_mangle]
pub unsafe extern "C" fn fill_got_entry(mut s1: *mut TCCState, mut rel: *mut Elf64_Rela) {
    let mut sym_index: std::os::raw::c_int =
        ((*rel).r_info >> 32 as std::os::raw::c_int) as std::os::raw::c_int;
    let mut sym: *mut Elf64_Sym = &mut *((*(*s1).symtab_section).data as *mut Elf64_Sym)
        .offset(sym_index as isize) as *mut Elf64_Sym;
    let mut attr: *mut sym_attr = get_sym_attr(s1, sym_index, 0 as std::os::raw::c_int);
    let mut offset: std::os::raw::c_uint = (*attr).got_offset;
    if 0 as std::os::raw::c_int as std::os::raw::c_uint == offset {
        return;
    }
    section_reserve(
        (*s1).got,
        offset.wrapping_add(8 as std::os::raw::c_int as std::os::raw::c_uint)
            as std::os::raw::c_ulong,
    );
    write64le((*(*s1).got).data.offset(offset as isize), (*sym).st_value);
}
/* Perform relocation to GOT or PLT entries */
#[no_mangle]
pub unsafe extern "C" fn fill_got(mut s1: *mut TCCState) {
    let mut s: *mut Section = 0 as *mut Section;
    let mut rel: *mut Elf64_Rela = 0 as *mut Elf64_Rela;
    let mut i: std::os::raw::c_int = 0;
    i = 1 as std::os::raw::c_int;
    while i < (*s1).nb_sections {
        s = *(*s1).sections.offset(i as isize);
        if !((*s).sh_type != 4 as std::os::raw::c_int) {
            /* no need to handle got relocations */
            if !((*s).link != (*s1).symtab_section) {
                rel = ((*s).data as *mut Elf64_Rela).offset(0 as std::os::raw::c_int as isize);
                while rel < (*s).data.offset((*s).data_offset as isize) as *mut Elf64_Rela {
                    match (*rel).r_info
                        & 0xffffffff as std::os::raw::c_uint as std::os::raw::c_ulong
                    {
                        3 | 9 | 41 | 42 | 4 => {
                            fill_got_entry(s1, rel);
                        },
                        _ => {},
                    }
                    rel = rel.offset(1)
                }
            }
        }
        i += 1
    }
}
/* See put_got_entry for a description.  This is the second stage
where GOT references to local defined symbols are rewritten.  */
unsafe extern "C" fn fill_local_got_entries(mut s1: *mut TCCState) {
    let mut rel: *mut Elf64_Rela = 0 as *mut Elf64_Rela;
    if (*(*s1).got).reloc.is_null() {
        return;
    }
    rel = ((*(*(*s1).got).reloc).data as *mut Elf64_Rela).offset(0 as std::os::raw::c_int as isize);
    while rel
        < (*(*(*s1).got).reloc)
            .data
            .offset((*(*(*s1).got).reloc).data_offset as isize) as *mut Elf64_Rela
    {
        if (*rel).r_info & 0xffffffff as std::os::raw::c_uint as std::os::raw::c_ulong
            == 8 as std::os::raw::c_int as std::os::raw::c_ulong
        {
            let mut sym_index: std::os::raw::c_int =
                ((*rel).r_info >> 32 as std::os::raw::c_int) as std::os::raw::c_int;
            let mut sym: *mut Elf64_Sym = &mut *((*(*s1).symtab_section).data as *mut Elf64_Sym)
                .offset(sym_index as isize)
                as *mut Elf64_Sym;
            let mut attr: *mut sym_attr = get_sym_attr(s1, sym_index, 0 as std::os::raw::c_int);
            let mut offset: std::os::raw::c_uint = (*attr).got_offset;
            if offset as std::os::raw::c_ulong != (*rel).r_offset.wrapping_sub((*(*s1).got).sh_addr)
            {
                tcc_enter_state(s1);
                Some(
                    _tcc_error_noabort
                        as unsafe extern "C" fn(_: *const std::os::raw::c_char, _: ...) -> (),
                )
                .expect("non-null function pointer")(
                    b"huh\x00" as *const u8 as *const std::os::raw::c_char,
                );
            }
            (*rel).r_info = ((0 as std::os::raw::c_int as Elf64_Xword)
                << 32 as std::os::raw::c_int)
                .wrapping_add(8 as std::os::raw::c_int as std::os::raw::c_ulong);
            (*rel).r_addend = (*sym).st_value as Elf64_Sxword
        }
        rel = rel.offset(1)
    }
}
/* Bind symbols of executable: resolve undefined symbols from exported symbols
in shared libraries and export non local defined symbols to shared libraries
if -rdynamic switch was given on command line */
unsafe extern "C" fn bind_exe_dynsyms(mut s1: *mut TCCState) {
    let mut name: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut sym_index: std::os::raw::c_int = 0;
    let mut index: std::os::raw::c_int = 0;
    let mut sym: *mut Elf64_Sym = 0 as *mut Elf64_Sym;
    let mut esym: *mut Elf64_Sym = 0 as *mut Elf64_Sym;
    let mut type_0: std::os::raw::c_int = 0;
    /* Resolve undefined symbols from dynamic symbols. When there is a match:
    - if STT_FUNC or STT_GNU_IFUNC symbol -> add it in PLT
    - if STT_OBJECT symbol -> add it in .bss section with suitable reloc */
    sym =
        ((*(*s1).symtab_section).data as *mut Elf64_Sym).offset(1 as std::os::raw::c_int as isize);
    while sym
        < (*(*s1).symtab_section)
            .data
            .offset((*(*s1).symtab_section).data_offset as isize) as *mut Elf64_Sym
    {
        if (*sym).st_shndx as std::os::raw::c_int == 0 as std::os::raw::c_int {
            name = ((*(*(*s1).symtab_section).link).data as *mut std::os::raw::c_char)
                .offset((*sym).st_name as isize);
            sym_index = find_elf_sym((*s1).dynsymtab_section, name);
            if sym_index != 0 {
                esym = &mut *((*(*s1).dynsymtab_section).data as *mut Elf64_Sym)
                    .offset(sym_index as isize) as *mut Elf64_Sym;
                type_0 = (*esym).st_info as std::os::raw::c_int & 0xf as std::os::raw::c_int;
                if type_0 == 2 as std::os::raw::c_int || type_0 == 10 as std::os::raw::c_int {
                    /* Indirect functions shall have STT_FUNC type in executable
                     * dynsym section. Indeed, a dlsym call following a lazy
                     * resolution would pick the symbol value from the
                     * executable dynsym entry which would contain the address
                     * of the function wanted by the caller of dlsym instead of
                     * the address of the function that would return that
                     * address */
                    let mut dynindex: std::os::raw::c_int = put_elf_sym(
                        (*s1).dynsym,
                        0 as std::os::raw::c_int as Elf64_Addr,
                        (*esym).st_size,
                        ((1 as std::os::raw::c_int) << 4 as std::os::raw::c_int)
                            + (2 as std::os::raw::c_int & 0xf as std::os::raw::c_int),
                        0 as std::os::raw::c_int,
                        0 as std::os::raw::c_int,
                        name,
                    );
                    let mut index_0: std::os::raw::c_int =
                        sym.offset_from((*(*s1).symtab_section).data as *mut Elf64_Sym)
                            as std::os::raw::c_long as std::os::raw::c_int;
                    (*get_sym_attr(s1, index_0, 1 as std::os::raw::c_int)).dyn_index = dynindex
                } else if type_0 == 1 as std::os::raw::c_int {
                    let mut offset: std::os::raw::c_ulong = 0;
                    let mut dynsym: *mut Elf64_Sym = 0 as *mut Elf64_Sym;
                    offset = (*(*s1).bss_section).data_offset;
                    /* XXX: which alignment ? */
                    offset = offset
                        .wrapping_add(16 as std::os::raw::c_int as std::os::raw::c_ulong)
                        .wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong)
                        & -(16 as std::os::raw::c_int) as std::os::raw::c_ulong;
                    set_elf_sym(
                        (*s1).symtab,
                        offset,
                        (*esym).st_size,
                        (*esym).st_info as std::os::raw::c_int,
                        0 as std::os::raw::c_int,
                        (*(*s1).bss_section).sh_num,
                        name,
                    );
                    index = put_elf_sym(
                        (*s1).dynsym,
                        offset,
                        (*esym).st_size,
                        (*esym).st_info as std::os::raw::c_int,
                        0 as std::os::raw::c_int,
                        (*(*s1).bss_section).sh_num,
                        name,
                    );
                    /* Ensure R_COPY works for weak symbol aliases */
                    if (*esym).st_info as std::os::raw::c_int >> 4 as std::os::raw::c_int
                        == 2 as std::os::raw::c_int
                    {
                        dynsym = ((*(*s1).dynsymtab_section).data as *mut Elf64_Sym)
                            .offset(1 as std::os::raw::c_int as isize);
                        while dynsym
                            < (*(*s1).dynsymtab_section)
                                .data
                                .offset((*(*s1).dynsymtab_section).data_offset as isize)
                                as *mut Elf64_Sym
                        {
                            if (*dynsym).st_value == (*esym).st_value
                                && (*dynsym).st_info as std::os::raw::c_int
                                    >> 4 as std::os::raw::c_int
                                    == 1 as std::os::raw::c_int
                            {
                                let mut dynname: *mut std::os::raw::c_char =
                                    ((*(*(*s1).dynsymtab_section).link).data
                                        as *mut std::os::raw::c_char)
                                        .offset((*dynsym).st_name as isize);
                                put_elf_sym(
                                    (*s1).dynsym,
                                    offset,
                                    (*dynsym).st_size,
                                    (*dynsym).st_info as std::os::raw::c_int,
                                    0 as std::os::raw::c_int,
                                    (*(*s1).bss_section).sh_num,
                                    dynname,
                                );
                                break;
                            } else {
                                dynsym = dynsym.offset(1)
                            }
                        }
                    }
                    put_elf_reloc(
                        (*s1).dynsym,
                        (*s1).bss_section,
                        offset,
                        5 as std::os::raw::c_int,
                        index,
                    );
                    offset = offset.wrapping_add((*esym).st_size);
                    (*(*s1).bss_section).data_offset = offset
                }
            } else if !((*sym).st_info as std::os::raw::c_int >> 4 as std::os::raw::c_int
                == 2 as std::os::raw::c_int
                || strcmp(
                    name,
                    b"_fp_hw\x00" as *const u8 as *const std::os::raw::c_char,
                ) == 0)
            {
                tcc_enter_state(s1);
                Some(
                    _tcc_error_noabort
                        as unsafe extern "C" fn(_: *const std::os::raw::c_char, _: ...) -> (),
                )
                .expect("non-null function pointer")(
                    b"undefined symbol \'%s\'\x00" as *const u8 as *const std::os::raw::c_char,
                    name,
                );
            }
        } else if (*s1).rdynamic as std::os::raw::c_int != 0
            && (*sym).st_info as std::os::raw::c_int >> 4 as std::os::raw::c_int
                != 0 as std::os::raw::c_int
        {
            /* STB_WEAK undefined symbols are accepted */
            /* XXX: _fp_hw seems to be part of the ABI, so we ignore it */
            /* if -rdynamic option, then export all non local symbols */
            name = ((*(*(*s1).symtab_section).link).data as *mut std::os::raw::c_char)
                .offset((*sym).st_name as isize);
            set_elf_sym(
                (*s1).dynsym,
                (*sym).st_value,
                (*sym).st_size,
                (*sym).st_info as std::os::raw::c_int,
                0 as std::os::raw::c_int,
                (*sym).st_shndx as std::os::raw::c_int,
                name,
            );
        }
        sym = sym.offset(1)
    }
}
/* Bind symbols of libraries: export all non local symbols of executable that
are referenced by shared libraries. The reason is that the dynamic loader
search symbol first in executable and then in libraries. Therefore a
reference to a symbol already defined by a library can still be resolved by
a symbol in the executable. */
unsafe extern "C" fn bind_libs_dynsyms(mut s1: *mut TCCState) {
    let mut name: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut sym_index: std::os::raw::c_int = 0;
    let mut sym: *mut Elf64_Sym = 0 as *mut Elf64_Sym;
    let mut esym: *mut Elf64_Sym = 0 as *mut Elf64_Sym;
    esym = ((*(*s1).dynsymtab_section).data as *mut Elf64_Sym)
        .offset(1 as std::os::raw::c_int as isize);
    while esym
        < (*(*s1).dynsymtab_section)
            .data
            .offset((*(*s1).dynsymtab_section).data_offset as isize) as *mut Elf64_Sym
    {
        name = ((*(*(*s1).dynsymtab_section).link).data as *mut std::os::raw::c_char)
            .offset((*esym).st_name as isize);
        sym_index = find_elf_sym((*s1).symtab_section, name);
        sym = &mut *((*(*s1).symtab_section).data as *mut Elf64_Sym).offset(sym_index as isize)
            as *mut Elf64_Sym;
        if sym_index != 0
            && (*sym).st_shndx as std::os::raw::c_int != 0 as std::os::raw::c_int
            && (*sym).st_info as std::os::raw::c_int >> 4 as std::os::raw::c_int
                != 0 as std::os::raw::c_int
        {
            set_elf_sym(
                (*s1).dynsym,
                (*sym).st_value,
                (*sym).st_size,
                (*sym).st_info as std::os::raw::c_int,
                0 as std::os::raw::c_int,
                (*sym).st_shndx as std::os::raw::c_int,
                name,
            );
        } else if (*esym).st_shndx as std::os::raw::c_int == 0 as std::os::raw::c_int {
            /* weak symbols can stay undefined */
            if (*esym).st_info as std::os::raw::c_int >> 4 as std::os::raw::c_int
                != 2 as std::os::raw::c_int
            {
                tcc_enter_state(s1);
                Some(
                    _tcc_warning
                        as unsafe extern "C" fn(_: *const std::os::raw::c_char, _: ...) -> (),
                )
                .expect("non-null function pointer")(
                    b"undefined dynamic symbol \'%s\'\x00" as *const u8
                        as *const std::os::raw::c_char,
                    name,
                );
            }
        }
        esym = esym.offset(1)
    }
}
/* Export all non local symbols. This is used by shared libraries so that the
non local symbols they define can resolve a reference in another shared
library or in the executable. Correspondingly, it allows undefined local
symbols to be resolved by other shared libraries or by the executable. */
unsafe extern "C" fn export_global_syms(mut s1: *mut TCCState) {
    let mut dynindex: std::os::raw::c_int = 0;
    let mut index: std::os::raw::c_int = 0;
    let mut name: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut sym: *mut Elf64_Sym = 0 as *mut Elf64_Sym;
    sym =
        ((*(*s1).symtab_section).data as *mut Elf64_Sym).offset(1 as std::os::raw::c_int as isize);
    while sym
        < (*(*s1).symtab_section)
            .data
            .offset((*(*s1).symtab_section).data_offset as isize) as *mut Elf64_Sym
    {
        if (*sym).st_info as std::os::raw::c_int >> 4 as std::os::raw::c_int
            != 0 as std::os::raw::c_int
        {
            name = ((*(*(*s1).symtab_section).link).data as *mut std::os::raw::c_char)
                .offset((*sym).st_name as isize);
            dynindex = put_elf_sym(
                (*s1).dynsym,
                (*sym).st_value,
                (*sym).st_size,
                (*sym).st_info as std::os::raw::c_int,
                0 as std::os::raw::c_int,
                (*sym).st_shndx as std::os::raw::c_int,
                name,
            );
            index = sym.offset_from((*(*s1).symtab_section).data as *mut Elf64_Sym)
                as std::os::raw::c_long as std::os::raw::c_int;
            (*get_sym_attr(s1, index, 1 as std::os::raw::c_int)).dyn_index = dynindex
        }
        sym = sym.offset(1)
    }
}
/* decide if an unallocated section should be output. */
unsafe extern "C" fn set_sec_sizes(mut s1: *mut TCCState) -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    let mut s: *mut Section = 0 as *mut Section;
    let mut textrel: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut file_type: std::os::raw::c_int = (*s1).output_type;
    /* Allocate strings for section names */
    i = 1 as std::os::raw::c_int;
    while i < (*s1).nb_sections {
        s = *(*s1).sections.offset(i as isize);
        if (*s).sh_type == 4 as std::os::raw::c_int
            && (*s).sh_flags & (1 as std::os::raw::c_int) << 1 as std::os::raw::c_int == 0
        {
            /* when generating a DLL, we include relocations but
            we may patch them */
            if file_type == 3 as std::os::raw::c_int
                && (**(*s1).sections.offset((*s).sh_info as isize)).sh_flags
                    & (1 as std::os::raw::c_int) << 1 as std::os::raw::c_int
                    != 0
            {
                let mut count: std::os::raw::c_int = prepare_dynamic_rel(s1, s);
                if count != 0 {
                    /* allocate the section */
                    (*s).sh_flags |= (1 as std::os::raw::c_int) << 1 as std::os::raw::c_int;
                    (*s).sh_size = (count as std::os::raw::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<Elf64_Rela>() as std::os::raw::c_ulong);
                    if (**(*s1).sections.offset((*s).sh_info as isize)).sh_flags
                        & (1 as std::os::raw::c_int) << 0 as std::os::raw::c_int
                        == 0
                    {
                        textrel = 1 as std::os::raw::c_int
                    }
                }
            }
        } else if (*s).sh_flags & (1 as std::os::raw::c_int) << 1 as std::os::raw::c_int != 0
            || (*s1).do_debug as std::os::raw::c_int != 0
        {
            (*s).sh_size = (*s).data_offset
        }
        i += 1
    }
    return textrel;
}
/* Assign sections to segments and decide how are sections laid out when loaded
in memory. This function also fills corresponding program headers. */
unsafe extern "C" fn layout_sections(
    mut s1: *mut TCCState,
    mut phdr: *mut Elf64_Phdr,
    mut phnum: std::os::raw::c_int,
    mut phfill: std::os::raw::c_int,
    mut interp: *mut Section,
    mut roinf: *mut ro_inf,
    mut sec_order: *mut std::os::raw::c_int,
) -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    let mut file_offset: std::os::raw::c_int = 0;
    let mut s: *mut Section = 0 as *mut Section;
    file_offset = 0 as std::os::raw::c_int;
    if (*s1).output_format == 0 as std::os::raw::c_int {
        file_offset = (::std::mem::size_of::<Elf64_Ehdr>() as std::os::raw::c_ulong).wrapping_add(
            (phnum as std::os::raw::c_ulong)
                .wrapping_mul(::std::mem::size_of::<Elf64_Phdr>() as std::os::raw::c_ulong),
        ) as std::os::raw::c_int
    }
    let mut s_align: std::os::raw::c_ulong = 0;
    let mut tmp: std::os::raw::c_longlong = 0;
    let mut addr: Elf64_Addr = 0;
    let mut ph: *mut Elf64_Phdr = 0 as *mut Elf64_Phdr;
    let mut j: std::os::raw::c_int = 0;
    let mut k: std::os::raw::c_int = 0;
    let mut f: std::os::raw::c_int = 0;
    let mut file_type: std::os::raw::c_int = (*s1).output_type;
    s_align = 0x200000 as std::os::raw::c_int as std::os::raw::c_ulong;
    if (*s1).section_align != 0 {
        s_align = (*s1).section_align as std::os::raw::c_ulong
    }
    if (*s1).has_text_addr != 0 {
        let mut a_offset: std::os::raw::c_int = 0;
        let mut p_offset: std::os::raw::c_int = 0;
        addr = (*s1).text_addr;
        /* we ensure that (addr % ELF_PAGE_SIZE) == file_offset %
        ELF_PAGE_SIZE */
        a_offset = (addr & s_align.wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong))
            as std::os::raw::c_int;
        p_offset = (file_offset as std::os::raw::c_ulong
            & s_align.wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong))
            as std::os::raw::c_int;
        if a_offset < p_offset {
            a_offset = (a_offset as std::os::raw::c_ulong).wrapping_add(s_align)
                as std::os::raw::c_int as std::os::raw::c_int
        }
        file_offset += a_offset - p_offset
    } else {
        if file_type == 3 as std::os::raw::c_int {
            addr = 0 as std::os::raw::c_int as Elf64_Addr
        } else {
            addr = 0x400000 as std::os::raw::c_int as Elf64_Addr
        }
        /* compute address after headers */
        addr = (addr as std::os::raw::c_ulong).wrapping_add(
            file_offset as std::os::raw::c_ulong
                & s_align.wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong),
        ) as Elf64_Addr as Elf64_Addr
    }
    ph = &mut *phdr.offset(0 as std::os::raw::c_int as isize) as *mut Elf64_Phdr;
    /* Leave one program headers for the program interpreter and one for
    the program header table itself if needed. These are done later as
    they require section layout to be done first. */
    if !interp.is_null() {
        ph = ph.offset(2 as std::os::raw::c_int as isize)
    }
    /* read only segment mapping for GNU_RELRO */
    (*roinf).sh_size = 0 as std::os::raw::c_int as Elf64_Addr;
    (*roinf).sh_addr = (*roinf).sh_size;
    (*roinf).sh_offset = (*roinf).sh_addr;
    j = 0 as std::os::raw::c_int;
    while j < phfill {
        (*ph).p_type = if j == 2 as std::os::raw::c_int {
            7 as std::os::raw::c_int
        } else {
            1 as std::os::raw::c_int
        } as Elf64_Word;
        if j == 0 as std::os::raw::c_int {
            (*ph).p_flags = ((1 as std::os::raw::c_int) << 2 as std::os::raw::c_int
                | (1 as std::os::raw::c_int) << 0 as std::os::raw::c_int)
                as Elf64_Word
        } else {
            (*ph).p_flags = ((1 as std::os::raw::c_int) << 2 as std::os::raw::c_int
                | (1 as std::os::raw::c_int) << 1 as std::os::raw::c_int)
                as Elf64_Word
        }
        (*ph).p_align = if j == 2 as std::os::raw::c_int {
            4 as std::os::raw::c_int as std::os::raw::c_ulong
        } else {
            s_align
        };
        /* Decide the layout of sections loaded in memory. This must
        be done before program headers are filled since they contain
        info about the layout. We do the following ordering: interp,
        symbol tables, relocations, progbits, nobits */
        /* XXX: do faster and simpler sorting */
        f = -(1 as std::os::raw::c_int);
        k = 0 as std::os::raw::c_int;
        while k < 7 as std::os::raw::c_int {
            let mut current_block_56: u64;
            i = 1 as std::os::raw::c_int;
            while i < (*s1).nb_sections {
                s = *(*s1).sections.offset(i as isize);
                /* compute if section should be included */
                if j == 0 as std::os::raw::c_int {
                    if (*s).sh_flags
                        & ((1 as std::os::raw::c_int) << 1 as std::os::raw::c_int
                            | (1 as std::os::raw::c_int) << 0 as std::os::raw::c_int
                            | (1 as std::os::raw::c_int) << 10 as std::os::raw::c_int)
                        != (1 as std::os::raw::c_int) << 1 as std::os::raw::c_int
                    {
                        current_block_56 = 7333393191927787629;
                    } else {
                        current_block_56 = 1356832168064818221;
                    }
                } else if j == 1 as std::os::raw::c_int {
                    if (*s).sh_flags
                        & ((1 as std::os::raw::c_int) << 1 as std::os::raw::c_int
                            | (1 as std::os::raw::c_int) << 0 as std::os::raw::c_int
                            | (1 as std::os::raw::c_int) << 10 as std::os::raw::c_int)
                        != (1 as std::os::raw::c_int) << 1 as std::os::raw::c_int
                            | (1 as std::os::raw::c_int) << 0 as std::os::raw::c_int
                    {
                        current_block_56 = 7333393191927787629;
                    } else {
                        current_block_56 = 1356832168064818221;
                    }
                } else if (*s).sh_flags
                    & ((1 as std::os::raw::c_int) << 1 as std::os::raw::c_int
                        | (1 as std::os::raw::c_int) << 0 as std::os::raw::c_int
                        | (1 as std::os::raw::c_int) << 10 as std::os::raw::c_int)
                    != (1 as std::os::raw::c_int) << 1 as std::os::raw::c_int
                        | (1 as std::os::raw::c_int) << 0 as std::os::raw::c_int
                        | (1 as std::os::raw::c_int) << 10 as std::os::raw::c_int
                {
                    current_block_56 = 7333393191927787629;
                } else {
                    current_block_56 = 1356832168064818221;
                }
                match current_block_56 {
                    1356832168064818221 => {
                        if s == interp {
                            if k != 0 as std::os::raw::c_int {
                                current_block_56 = 7333393191927787629;
                            } else {
                                current_block_56 = 1134115459065347084;
                            }
                        } else if ((*s).sh_type == 11 as std::os::raw::c_int
                            || (*s).sh_type == 3 as std::os::raw::c_int
                            || (*s).sh_type == 5 as std::os::raw::c_int)
                            && strstr(
                                (*s).name.as_mut_ptr(),
                                b".stab\x00" as *const u8 as *const std::os::raw::c_char,
                            )
                            .is_null()
                        {
                            if k != 1 as std::os::raw::c_int {
                                current_block_56 = 7333393191927787629;
                            } else {
                                current_block_56 = 1134115459065347084;
                            }
                        } else if (*s).sh_type == 4 as std::os::raw::c_int {
                            if !(*s1).plt.is_null() && s == (*(*s1).plt).reloc {
                                if k != 3 as std::os::raw::c_int {
                                    current_block_56 = 7333393191927787629;
                                } else {
                                    current_block_56 = 1134115459065347084;
                                }
                            } else if k != 2 as std::os::raw::c_int {
                                current_block_56 = 7333393191927787629;
                            } else {
                                current_block_56 = 1134115459065347084;
                            }
                        } else if (*s).sh_type == 8 as std::os::raw::c_int {
                            if k != 6 as std::os::raw::c_int {
                                current_block_56 = 7333393191927787629;
                            } else {
                                current_block_56 = 1134115459065347084;
                            }
                        } else if (s == (*s1).rodata_section
                            || s == (*s1).bounds_section
                            || s == (*s1).lbounds_section)
                            && (*s).sh_flags
                                & (1 as std::os::raw::c_int) << 0 as std::os::raw::c_int
                                != 0
                        {
                            if k != 4 as std::os::raw::c_int {
                                current_block_56 = 7333393191927787629;
                            } else {
                                /* Align next section on page size.
                                This is needed to remap roinf section ro. */
                                f = 1 as std::os::raw::c_int;
                                current_block_56 = 1134115459065347084;
                            }
                        } else if k != 5 as std::os::raw::c_int {
                            current_block_56 = 7333393191927787629;
                        } else {
                            current_block_56 = 1134115459065347084;
                        }
                        match current_block_56 {
                            7333393191927787629 => {},
                            _ => {
                                let fresh7 = sec_order;
                                sec_order = sec_order.offset(1);
                                *fresh7 = i;
                                /* section matches: we align it and add its size */
                                tmp = addr as std::os::raw::c_longlong;
                                let fresh8 = f;
                                f = f - 1;
                                if fresh8 == 0 as std::os::raw::c_int {
                                    (*s).sh_addralign = sysconf(_SC_PAGESIZE as std::os::raw::c_int)
                                        as std::os::raw::c_int
                                }
                                addr = addr
                                    .wrapping_add((*s).sh_addralign as std::os::raw::c_ulong)
                                    .wrapping_sub(
                                        1 as std::os::raw::c_int as std::os::raw::c_ulong,
                                    )
                                    & !((*s).sh_addralign - 1 as std::os::raw::c_int)
                                        as std::os::raw::c_ulong;
                                file_offset += (addr as std::os::raw::c_ulonglong)
                                    .wrapping_sub(tmp as std::os::raw::c_ulonglong)
                                    as std::os::raw::c_int;
                                (*s).sh_offset = file_offset as std::os::raw::c_ulong;
                                (*s).sh_addr = addr;
                                /* update program header infos */
                                if (*ph).p_offset
                                    == 0 as std::os::raw::c_int as std::os::raw::c_ulong
                                {
                                    (*ph).p_offset = file_offset as Elf64_Off;
                                    (*ph).p_vaddr = addr;
                                    (*ph).p_paddr = (*ph).p_vaddr
                                }
                                if k == 4 as std::os::raw::c_int {
                                    if (*roinf).sh_size
                                        == 0 as std::os::raw::c_int as std::os::raw::c_ulong
                                    {
                                        (*roinf).sh_offset = (*s).sh_offset;
                                        (*roinf).sh_addr = (*s).sh_addr
                                    }
                                    (*roinf).sh_size = addr
                                        .wrapping_sub((*roinf).sh_addr)
                                        .wrapping_add((*s).sh_size)
                                }
                                addr = (addr as std::os::raw::c_ulong).wrapping_add((*s).sh_size)
                                    as Elf64_Addr
                                    as Elf64_Addr;
                                if (*s).sh_type != 8 as std::os::raw::c_int {
                                    file_offset = (file_offset as std::os::raw::c_ulong)
                                        .wrapping_add((*s).sh_size)
                                        as std::os::raw::c_int
                                        as std::os::raw::c_int
                                }
                            },
                        }
                    },
                    _ => {},
                }
                i += 1
            }
            k += 1
        }
        if j == 0 as std::os::raw::c_int {
            /* Make the first PT_LOAD segment include the program
            headers itself (and the ELF header as well), it'll
            come out with same memory use but will make various
            tools like binutils strip work better.  */
            (*ph).p_offset &= !(*ph)
                .p_align
                .wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong);
            (*ph).p_vaddr &= !(*ph)
                .p_align
                .wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong);
            (*ph).p_paddr &= !(*ph)
                .p_align
                .wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong)
        }
        (*ph).p_filesz = (file_offset as std::os::raw::c_ulong).wrapping_sub((*ph).p_offset);
        (*ph).p_memsz = addr.wrapping_sub((*ph).p_vaddr);
        ph = ph.offset(1);
        if j == 0 as std::os::raw::c_int {
            if (*s1).output_format == 0 as std::os::raw::c_int {
                /* if in the middle of a page, we duplicate the page in
                memory so that one copy is RX and the other is RW */
                if addr & s_align.wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong)
                    != 0 as std::os::raw::c_int as std::os::raw::c_ulong
                {
                    addr = (addr as std::os::raw::c_ulong).wrapping_add(s_align) as Elf64_Addr
                        as Elf64_Addr
                }
            } else {
                addr = addr
                    .wrapping_add(s_align)
                    .wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong)
                    & !s_align.wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong);
                file_offset = ((file_offset as std::os::raw::c_ulong)
                    .wrapping_add(s_align)
                    .wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong)
                    & !s_align.wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong))
                    as std::os::raw::c_int
            }
        }
        j += 1
    }
    /* all other sections come after */
    return layout_any_sections(s1, file_offset, sec_order, 0 as std::os::raw::c_int);
}
/* put dynamic tag */
unsafe extern "C" fn put_dt(
    mut dynamic: *mut Section,
    mut dt: std::os::raw::c_int,
    mut val: Elf64_Addr,
) {
    let mut dyn_0: *mut Elf64_Dyn = 0 as *mut Elf64_Dyn;
    dyn_0 = section_ptr_add(
        dynamic,
        ::std::mem::size_of::<Elf64_Dyn>() as std::os::raw::c_ulong,
    ) as *mut Elf64_Dyn;
    (*dyn_0).d_tag = dt as Elf64_Sxword;
    (*dyn_0).d_un.d_val = val;
}
unsafe extern "C" fn fill_unloadable_phdr(
    mut phdr: *mut Elf64_Phdr,
    mut phnum: std::os::raw::c_int,
    mut interp: *mut Section,
    mut dynamic: *mut Section,
    mut note: *mut Section,
    mut roinf: *mut ro_inf,
) {
    let mut ph: *mut Elf64_Phdr = 0 as *mut Elf64_Phdr;
    /* if interpreter, then add corresponding program header */
    if !interp.is_null() {
        ph = &mut *phdr.offset(0 as std::os::raw::c_int as isize) as *mut Elf64_Phdr; /* interp->sh_addralign; */
        (*ph).p_type = 6 as std::os::raw::c_int as Elf64_Word;
        (*ph).p_offset = ::std::mem::size_of::<Elf64_Ehdr>() as std::os::raw::c_ulong;
        (*ph).p_memsz = (phnum as std::os::raw::c_ulong)
            .wrapping_mul(::std::mem::size_of::<Elf64_Phdr>() as std::os::raw::c_ulong);
        (*ph).p_filesz = (*ph).p_memsz;
        (*ph).p_vaddr = (*interp).sh_addr.wrapping_sub((*ph).p_filesz);
        (*ph).p_paddr = (*ph).p_vaddr;
        (*ph).p_flags = ((1 as std::os::raw::c_int) << 2 as std::os::raw::c_int
            | (1 as std::os::raw::c_int) << 0 as std::os::raw::c_int)
            as Elf64_Word;
        (*ph).p_align = 4 as std::os::raw::c_int as Elf64_Xword;
        ph = ph.offset(1);
        (*ph).p_type = 3 as std::os::raw::c_int as Elf64_Word;
        (*ph).p_offset = (*interp).sh_offset;
        (*ph).p_vaddr = (*interp).sh_addr;
        (*ph).p_paddr = (*ph).p_vaddr;
        (*ph).p_filesz = (*interp).sh_size;
        (*ph).p_memsz = (*interp).sh_size;
        (*ph).p_flags = ((1 as std::os::raw::c_int) << 2 as std::os::raw::c_int) as Elf64_Word;
        (*ph).p_align = (*interp).sh_addralign as Elf64_Xword
    }
    if !note.is_null() {
        ph = &mut *phdr.offset(
            (phnum
                - 2 as std::os::raw::c_int
                - (roinf != 0 as *mut std::os::raw::c_void as *mut ro_inf) as std::os::raw::c_int)
                as isize,
        ) as *mut Elf64_Phdr;
        (*ph).p_type = 4 as std::os::raw::c_int as Elf64_Word;
        (*ph).p_offset = (*note).sh_offset;
        (*ph).p_vaddr = (*note).sh_addr;
        (*ph).p_paddr = (*ph).p_vaddr;
        (*ph).p_filesz = (*note).sh_size;
        (*ph).p_memsz = (*note).sh_size;
        (*ph).p_flags = ((1 as std::os::raw::c_int) << 2 as std::os::raw::c_int) as Elf64_Word;
        (*ph).p_align = (*note).sh_addralign as Elf64_Xword
    }
    /* if dynamic section, then add corresponding program header */
    if !dynamic.is_null() {
        ph = &mut *phdr.offset(
            (phnum
                - 1 as std::os::raw::c_int
                - (roinf != 0 as *mut std::os::raw::c_void as *mut ro_inf) as std::os::raw::c_int)
                as isize,
        ) as *mut Elf64_Phdr;
        (*ph).p_type = 2 as std::os::raw::c_int as Elf64_Word;
        (*ph).p_offset = (*dynamic).sh_offset;
        (*ph).p_vaddr = (*dynamic).sh_addr;
        (*ph).p_paddr = (*ph).p_vaddr;
        (*ph).p_filesz = (*dynamic).sh_size;
        (*ph).p_memsz = (*dynamic).sh_size;
        (*ph).p_flags = ((1 as std::os::raw::c_int) << 2 as std::os::raw::c_int
            | (1 as std::os::raw::c_int) << 1 as std::os::raw::c_int)
            as Elf64_Word;
        (*ph).p_align = (*dynamic).sh_addralign as Elf64_Xword
    }
    if !roinf.is_null() {
        ph = &mut *phdr.offset((phnum - 1 as std::os::raw::c_int) as isize) as *mut Elf64_Phdr;
        (*ph).p_type = 0x6474e552 as std::os::raw::c_int as Elf64_Word;
        (*ph).p_offset = (*roinf).sh_offset;
        (*ph).p_vaddr = (*roinf).sh_addr;
        (*ph).p_paddr = (*ph).p_vaddr;
        (*ph).p_filesz = (*roinf).sh_size;
        (*ph).p_memsz = (*roinf).sh_size;
        (*ph).p_flags = ((1 as std::os::raw::c_int) << 2 as std::os::raw::c_int) as Elf64_Word;
        (*ph).p_align = 1 as std::os::raw::c_int as Elf64_Xword
    };
}
/* Fill the dynamic section with tags describing the address and size of
sections */
unsafe extern "C" fn fill_dynamic(mut s1: *mut TCCState, mut dyninf: *mut dyn_inf) {
    let mut dynamic: *mut Section = (*dyninf).dynamic;
    let mut s: *mut Section = 0 as *mut Section;
    /* put dynamic section entries */
    put_dt(
        dynamic,
        4 as std::os::raw::c_int,
        (*(*(*s1).dynsym).hash).sh_addr,
    );
    put_dt(
        dynamic,
        5 as std::os::raw::c_int,
        (*(*dyninf).dynstr).sh_addr,
    );
    put_dt(dynamic, 6 as std::os::raw::c_int, (*(*s1).dynsym).sh_addr);
    put_dt(
        dynamic,
        10 as std::os::raw::c_int,
        (*(*dyninf).dynstr).data_offset,
    );
    put_dt(
        dynamic,
        11 as std::os::raw::c_int,
        ::std::mem::size_of::<Elf64_Sym>() as std::os::raw::c_ulong,
    );
    put_dt(dynamic, 7 as std::os::raw::c_int, (*dyninf).rel_addr);
    put_dt(dynamic, 8 as std::os::raw::c_int, (*dyninf).rel_size);
    put_dt(
        dynamic,
        9 as std::os::raw::c_int,
        ::std::mem::size_of::<Elf64_Rela>() as std::os::raw::c_ulong,
    );
    if !(*s1).plt.is_null() && !(*(*s1).plt).reloc.is_null() {
        put_dt(dynamic, 3 as std::os::raw::c_int, (*(*s1).got).sh_addr);
        put_dt(
            dynamic,
            2 as std::os::raw::c_int,
            (*(*(*s1).plt).reloc).data_offset,
        );
        put_dt(
            dynamic,
            23 as std::os::raw::c_int,
            (*(*(*s1).plt).reloc).sh_addr,
        );
        put_dt(
            dynamic,
            20 as std::os::raw::c_int,
            7 as std::os::raw::c_int as Elf64_Addr,
        );
    }
    put_dt(
        dynamic,
        0x6ffffff9 as std::os::raw::c_int,
        0 as std::os::raw::c_int as Elf64_Addr,
    );
    if !(*s1).versym_section.is_null() && !(*s1).verneed_section.is_null() {
        /* The dynamic linker can not handle VERSYM without VERNEED */
        put_dt(
            dynamic,
            0x6ffffff0 as std::os::raw::c_int,
            (*(*s1).versym_section).sh_addr,
        );
        put_dt(
            dynamic,
            0x6ffffffe as std::os::raw::c_int,
            (*(*s1).verneed_section).sh_addr,
        );
        put_dt(
            dynamic,
            0x6fffffff as std::os::raw::c_int,
            (*s1).dt_verneednum as Elf64_Addr,
        );
    }
    s = find_section_create(
        s1,
        b".preinit_array\x00" as *const u8 as *const std::os::raw::c_char,
        0 as std::os::raw::c_int,
    );
    if !s.is_null() && (*s).data_offset != 0 {
        put_dt(dynamic, 32 as std::os::raw::c_int, (*s).sh_addr);
        put_dt(dynamic, 33 as std::os::raw::c_int, (*s).data_offset);
    }
    s = find_section_create(
        s1,
        b".init_array\x00" as *const u8 as *const std::os::raw::c_char,
        0 as std::os::raw::c_int,
    );
    if !s.is_null() && (*s).data_offset != 0 {
        put_dt(dynamic, 25 as std::os::raw::c_int, (*s).sh_addr);
        put_dt(dynamic, 27 as std::os::raw::c_int, (*s).data_offset);
    }
    s = find_section_create(
        s1,
        b".fini_array\x00" as *const u8 as *const std::os::raw::c_char,
        0 as std::os::raw::c_int,
    );
    if !s.is_null() && (*s).data_offset != 0 {
        put_dt(dynamic, 26 as std::os::raw::c_int, (*s).sh_addr);
        put_dt(dynamic, 28 as std::os::raw::c_int, (*s).data_offset);
    }
    s = find_section_create(
        s1,
        b".init\x00" as *const u8 as *const std::os::raw::c_char,
        0 as std::os::raw::c_int,
    );
    if !s.is_null() && (*s).data_offset != 0 {
        put_dt(dynamic, 12 as std::os::raw::c_int, (*s).sh_addr);
    }
    s = find_section_create(
        s1,
        b".fini\x00" as *const u8 as *const std::os::raw::c_char,
        0 as std::os::raw::c_int,
    );
    if !s.is_null() && (*s).data_offset != 0 {
        put_dt(dynamic, 13 as std::os::raw::c_int, (*s).sh_addr);
    }
    if (*s1).do_debug != 0 {
        put_dt(
            dynamic,
            21 as std::os::raw::c_int,
            0 as std::os::raw::c_int as Elf64_Addr,
        );
    }
    put_dt(
        dynamic,
        0 as std::os::raw::c_int,
        0 as std::os::raw::c_int as Elf64_Addr,
    );
}
/* Remove gaps between RELX sections.
These gaps are a result of final_sections_reloc. Here some relocs are removed.
The gaps are then filled with 0 in tcc_output_elf. The 0 is intepreted as
R_...NONE reloc. This does work on most targets but on OpenBSD/arm64 this
is illegal. OpenBSD/arm64 does not support R_...NONE reloc. */
unsafe extern "C" fn update_reloc_sections(mut s1: *mut TCCState, mut dyninf: *mut dyn_inf) {
    let mut i: std::os::raw::c_int = 0;
    let mut file_offset: std::os::raw::c_ulong = 0 as std::os::raw::c_int as std::os::raw::c_ulong;
    let mut s: *mut Section = 0 as *mut Section;
    let mut relocplt: *mut Section = if !(*s1).plt.is_null() {
        (*(*s1).plt).reloc
    } else {
        0 as *mut Section
    };
    /* dynamic relocation table information, for .dynamic section */
    (*dyninf).rel_size = 0 as std::os::raw::c_int as Elf64_Addr;
    (*dyninf).rel_addr = (*dyninf).rel_size;
    i = 1 as std::os::raw::c_int;
    while i < (*s1).nb_sections {
        s = *(*s1).sections.offset(i as isize);
        if (*s).sh_type == 4 as std::os::raw::c_int && s != relocplt {
            if (*dyninf).rel_size == 0 as std::os::raw::c_int as std::os::raw::c_ulong {
                (*dyninf).rel_addr = (*s).sh_addr;
                file_offset = (*s).sh_offset
            } else {
                (*s).sh_addr = (*dyninf).rel_addr.wrapping_add((*dyninf).rel_size);
                (*s).sh_offset = file_offset.wrapping_add((*dyninf).rel_size)
            }
            (*dyninf).rel_size = ((*dyninf).rel_size as std::os::raw::c_ulong)
                .wrapping_add((*s).sh_size) as Elf64_Addr
                as Elf64_Addr
        }
        i += 1
    }
}
/* ndef ELF_OBJ_ONLY */
/* Create an ELF file on disk.
This function handle ELF specific layout requirements */
unsafe extern "C" fn tcc_output_elf(
    mut s1: *mut TCCState,
    mut f: *mut FILE,
    mut phnum: std::os::raw::c_int,
    mut phdr: *mut Elf64_Phdr,
    mut file_offset: std::os::raw::c_int,
    mut sec_order: *mut std::os::raw::c_int,
) {
    let mut i: std::os::raw::c_int = 0;
    let mut shnum: std::os::raw::c_int = 0;
    let mut offset: std::os::raw::c_int = 0;
    let mut size: std::os::raw::c_int = 0;
    let mut file_type: std::os::raw::c_int = 0;
    let mut s: *mut Section = 0 as *mut Section;
    let mut ehdr: Elf64_Ehdr = Elf64_Ehdr {
        e_ident: [0; 16],
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
    };
    let mut shdr: Elf64_Shdr = Elf64_Shdr {
        sh_name: 0,
        sh_type: 0,
        sh_flags: 0,
        sh_addr: 0,
        sh_offset: 0,
        sh_size: 0,
        sh_link: 0,
        sh_info: 0,
        sh_addralign: 0,
        sh_entsize: 0,
    };
    let mut sh: *mut Elf64_Shdr = 0 as *mut Elf64_Shdr;
    file_type = (*s1).output_type;
    shnum = (*s1).nb_sections;
    memset(
        &mut ehdr as *mut Elf64_Ehdr as *mut std::os::raw::c_void,
        0 as std::os::raw::c_int,
        ::std::mem::size_of::<Elf64_Ehdr>() as std::os::raw::c_ulong,
    );
    if phnum > 0 as std::os::raw::c_int {
        ehdr.e_phentsize =
            ::std::mem::size_of::<Elf64_Phdr>() as std::os::raw::c_ulong as Elf64_Half;
        ehdr.e_phnum = phnum as Elf64_Half;
        ehdr.e_phoff = ::std::mem::size_of::<Elf64_Ehdr>() as std::os::raw::c_ulong
    }
    /* align to 4 */
    file_offset = file_offset + 3 as std::os::raw::c_int & -(4 as std::os::raw::c_int);
    /* fill header */
    ehdr.e_ident[0 as std::os::raw::c_int as usize] =
        0x7f as std::os::raw::c_int as std::os::raw::c_uchar; /* XXX: is it correct ? */
    ehdr.e_ident[1 as std::os::raw::c_int as usize] = 'E' as i32 as std::os::raw::c_uchar;
    ehdr.e_ident[2 as std::os::raw::c_int as usize] = 'L' as i32 as std::os::raw::c_uchar;
    ehdr.e_ident[3 as std::os::raw::c_int as usize] = 'F' as i32 as std::os::raw::c_uchar;
    ehdr.e_ident[4 as std::os::raw::c_int as usize] =
        2 as std::os::raw::c_int as std::os::raw::c_uchar;
    ehdr.e_ident[5 as std::os::raw::c_int as usize] =
        1 as std::os::raw::c_int as std::os::raw::c_uchar;
    ehdr.e_ident[6 as std::os::raw::c_int as usize] =
        1 as std::os::raw::c_int as std::os::raw::c_uchar;
    match file_type {
        3 => {
            ehdr.e_type = 3 as std::os::raw::c_int as Elf64_Half;
            ehdr.e_entry = (*(*s1).text_section).sh_addr
        },
        4 => ehdr.e_type = 1 as std::os::raw::c_int as Elf64_Half,
        2 | _ => {
            ehdr.e_type = 2 as std::os::raw::c_int as Elf64_Half;
            ehdr.e_entry = get_sym_addr(
                s1,
                b"_start\x00" as *const u8 as *const std::os::raw::c_char,
                1 as std::os::raw::c_int,
                0 as std::os::raw::c_int,
            )
        },
    }
    ehdr.e_machine = 62 as std::os::raw::c_int as Elf64_Half;
    ehdr.e_version = 1 as std::os::raw::c_int as Elf64_Word;
    ehdr.e_shoff = file_offset as Elf64_Off;
    ehdr.e_ehsize = ::std::mem::size_of::<Elf64_Ehdr>() as std::os::raw::c_ulong as Elf64_Half;
    ehdr.e_shentsize = ::std::mem::size_of::<Elf64_Shdr>() as std::os::raw::c_ulong as Elf64_Half;
    ehdr.e_shnum = shnum as Elf64_Half;
    ehdr.e_shstrndx = (shnum - 1 as std::os::raw::c_int) as Elf64_Half;
    fwrite(
        &mut ehdr as *mut Elf64_Ehdr as *const std::os::raw::c_void,
        1 as std::os::raw::c_int as std::os::raw::c_ulong,
        ::std::mem::size_of::<Elf64_Ehdr>() as std::os::raw::c_ulong,
        f,
    );
    fwrite(
        phdr as *const std::os::raw::c_void,
        1 as std::os::raw::c_int as std::os::raw::c_ulong,
        (phnum as std::os::raw::c_ulong)
            .wrapping_mul(::std::mem::size_of::<Elf64_Phdr>() as std::os::raw::c_ulong),
        f,
    );
    offset = (::std::mem::size_of::<Elf64_Ehdr>() as std::os::raw::c_ulong).wrapping_add(
        (phnum as std::os::raw::c_ulong)
            .wrapping_mul(::std::mem::size_of::<Elf64_Phdr>() as std::os::raw::c_ulong),
    ) as std::os::raw::c_int;
    sort_syms(s1, (*s1).symtab_section);
    i = 1 as std::os::raw::c_int;
    while i < (*s1).nb_sections {
        s = *(*s1)
            .sections
            .offset(*sec_order.offset(i as isize) as isize);
        if (*s).sh_type != 8 as std::os::raw::c_int {
            while (offset as std::os::raw::c_ulong) < (*s).sh_offset {
                fputc(0 as std::os::raw::c_int, f);
                offset += 1
            }
            size = (*s).sh_size as std::os::raw::c_int;
            if size != 0 {
                fwrite(
                    (*s).data as *const std::os::raw::c_void,
                    1 as std::os::raw::c_int as std::os::raw::c_ulong,
                    size as std::os::raw::c_ulong,
                    f,
                );
            }
            offset += size
        }
        i += 1
    }
    /* output section headers */
    while (offset as std::os::raw::c_ulong) < ehdr.e_shoff {
        fputc(0 as std::os::raw::c_int, f);
        offset += 1
    }
    i = 0 as std::os::raw::c_int;
    while i < (*s1).nb_sections {
        sh = &mut shdr;
        memset(
            sh as *mut std::os::raw::c_void,
            0 as std::os::raw::c_int,
            ::std::mem::size_of::<Elf64_Shdr>() as std::os::raw::c_ulong,
        );
        s = *(*s1).sections.offset(i as isize);
        if !s.is_null() {
            (*sh).sh_name = (*s).sh_name as Elf64_Word;
            (*sh).sh_type = (*s).sh_type as Elf64_Word;
            (*sh).sh_flags = (*s).sh_flags as Elf64_Xword;
            (*sh).sh_entsize = (*s).sh_entsize as Elf64_Xword;
            (*sh).sh_info = (*s).sh_info as Elf64_Word;
            if !(*s).link.is_null() {
                (*sh).sh_link = (*(*s).link).sh_num as Elf64_Word
            }
            (*sh).sh_addralign = (*s).sh_addralign as Elf64_Xword;
            (*sh).sh_addr = (*s).sh_addr;
            (*sh).sh_offset = (*s).sh_offset;
            (*sh).sh_size = (*s).sh_size
        }
        fwrite(
            sh as *const std::os::raw::c_void,
            1 as std::os::raw::c_int as std::os::raw::c_ulong,
            ::std::mem::size_of::<Elf64_Shdr>() as std::os::raw::c_ulong,
            f,
        );
        i += 1
    }
}
unsafe extern "C" fn tcc_output_binary(
    mut s1: *mut TCCState,
    mut f: *mut FILE,
    mut sec_order: *const std::os::raw::c_int,
) {
    let mut s: *mut Section = 0 as *mut Section;
    let mut i: std::os::raw::c_int = 0;
    let mut offset: std::os::raw::c_int = 0;
    let mut size: std::os::raw::c_int = 0;
    offset = 0 as std::os::raw::c_int;
    i = 1 as std::os::raw::c_int;
    while i < (*s1).nb_sections {
        s = *(*s1)
            .sections
            .offset(*sec_order.offset(i as isize) as isize);
        if (*s).sh_type != 8 as std::os::raw::c_int
            && (*s).sh_flags & (1 as std::os::raw::c_int) << 1 as std::os::raw::c_int != 0
        {
            while (offset as std::os::raw::c_ulong) < (*s).sh_offset {
                fputc(0 as std::os::raw::c_int, f);
                offset += 1
            }
            size = (*s).sh_size as std::os::raw::c_int;
            fwrite(
                (*s).data as *const std::os::raw::c_void,
                1 as std::os::raw::c_int as std::os::raw::c_ulong,
                size as std::os::raw::c_ulong,
                f,
            );
            offset += size
        }
        i += 1
    }
}
/* Write an elf, coff or "binary" file */
unsafe extern "C" fn tcc_write_elf_file(
    mut s1: *mut TCCState,
    mut filename: *const std::os::raw::c_char,
    mut phnum: std::os::raw::c_int,
    mut phdr: *mut Elf64_Phdr,
    mut file_offset: std::os::raw::c_int,
    mut sec_order: *mut std::os::raw::c_int,
) -> std::os::raw::c_int {
    let mut fd: std::os::raw::c_int = 0;
    let mut mode: std::os::raw::c_int = 0;
    let mut file_type: std::os::raw::c_int = 0;
    let mut f: *mut FILE = 0 as *mut FILE;
    file_type = (*s1).output_type;
    if file_type == 4 as std::os::raw::c_int {
        mode = 0o666 as std::os::raw::c_int
    } else {
        mode = 0o777 as std::os::raw::c_int
    }
    unlink(filename);
    fd = open(
        filename,
        0o1 as std::os::raw::c_int
            | 0o100 as std::os::raw::c_int
            | 0o1000 as std::os::raw::c_int
            | 0 as std::os::raw::c_int,
        mode,
    );
    if fd < 0 as std::os::raw::c_int {
        tcc_enter_state(s1);
        Some(
            _tcc_error_noabort
                as unsafe extern "C" fn(_: *const std::os::raw::c_char, _: ...) -> (),
        )
        .expect("non-null function pointer")(
            b"could not write \'%s\'\x00" as *const u8 as *const std::os::raw::c_char,
            filename,
        );
        return -(1 as std::os::raw::c_int);
    }
    f = fdopen(fd, b"wb\x00" as *const u8 as *const std::os::raw::c_char);
    if (*s1).verbose != 0 {
        printf(
            b"<- %s\n\x00" as *const u8 as *const std::os::raw::c_char,
            filename,
        );
    }
    if (*s1).output_format == 0 as std::os::raw::c_int {
        tcc_output_elf(s1, f, phnum, phdr, file_offset, sec_order);
    } else {
        tcc_output_binary(s1, f, sec_order);
    }
    fclose(f);
    return 0 as std::os::raw::c_int;
}
/* Sort section headers by assigned sh_addr, remove sections
that we aren't going to output.  */
unsafe extern "C" fn tidy_section_headers(
    mut s1: *mut TCCState,
    mut sec_order: *mut std::os::raw::c_int,
) {
    let mut i: std::os::raw::c_int = 0;
    let mut nnew: std::os::raw::c_int = 0;
    let mut l: std::os::raw::c_int = 0;
    let mut backmap: *mut std::os::raw::c_int = 0 as *mut std::os::raw::c_int;
    let mut snew: *mut *mut Section = 0 as *mut *mut Section;
    let mut s: *mut Section = 0 as *mut Section;
    let mut sym: *mut Elf64_Sym = 0 as *mut Elf64_Sym;
    snew = tcc_malloc(
        ((*s1).nb_sections as std::os::raw::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut Section>() as std::os::raw::c_ulong),
    ) as *mut *mut Section;
    backmap = tcc_malloc(
        ((*s1).nb_sections as std::os::raw::c_ulong)
            .wrapping_mul(::std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong),
    ) as *mut std::os::raw::c_int;
    i = 0 as std::os::raw::c_int;
    nnew = 0 as std::os::raw::c_int;
    l = (*s1).nb_sections;
    while i < (*s1).nb_sections {
        s = *(*s1)
            .sections
            .offset(*sec_order.offset(i as isize) as isize);
        if i == 0 || (*s).sh_name != 0 {
            *backmap.offset(*sec_order.offset(i as isize) as isize) = nnew;
            let ref mut fresh9 = *snew.offset(nnew as isize);
            *fresh9 = s;
            nnew += 1
        } else {
            *backmap.offset(*sec_order.offset(i as isize) as isize) = 0 as std::os::raw::c_int;
            l -= 1;
            let ref mut fresh10 = *snew.offset(l as isize);
            *fresh10 = s
        }
        i += 1
    }
    i = 0 as std::os::raw::c_int;
    while i < nnew {
        s = *snew.offset(i as isize);
        if !s.is_null() {
            (*s).sh_num = i;
            if (*s).sh_type == 4 as std::os::raw::c_int {
                (*s).sh_info = *backmap.offset((*s).sh_info as isize)
            }
        }
        i += 1
    }
    sym =
        ((*(*s1).symtab_section).data as *mut Elf64_Sym).offset(1 as std::os::raw::c_int as isize);
    while sym
        < (*(*s1).symtab_section)
            .data
            .offset((*(*s1).symtab_section).data_offset as isize) as *mut Elf64_Sym
    {
        if (*sym).st_shndx as std::os::raw::c_int != 0 as std::os::raw::c_int
            && ((*sym).st_shndx as std::os::raw::c_int) < 0xff00 as std::os::raw::c_int
        {
            (*sym).st_shndx = *backmap.offset((*sym).st_shndx as isize) as Elf64_Section
        }
        sym = sym.offset(1)
    }
    if (*s1).static_link == 0 {
        sym = ((*(*s1).dynsym).data as *mut Elf64_Sym).offset(1 as std::os::raw::c_int as isize);
        while sym
            < (*(*s1).dynsym)
                .data
                .offset((*(*s1).dynsym).data_offset as isize) as *mut Elf64_Sym
        {
            if (*sym).st_shndx as std::os::raw::c_int != 0 as std::os::raw::c_int
                && ((*sym).st_shndx as std::os::raw::c_int) < 0xff00 as std::os::raw::c_int
            {
                (*sym).st_shndx = *backmap.offset((*sym).st_shndx as isize) as Elf64_Section
            }
            sym = sym.offset(1)
        }
    }
    i = 0 as std::os::raw::c_int;
    while i < (*s1).nb_sections {
        *sec_order.offset(i as isize) = i;
        i += 1
    }
    tcc_free((*s1).sections as *mut std::os::raw::c_void);
    (*s1).sections = snew;
    (*s1).nb_sections = nnew;
    tcc_free(backmap as *mut std::os::raw::c_void);
}
/* Output an elf, coff or binary file */
/* XXX: suppress unneeded sections */
unsafe extern "C" fn elf_output_file(
    mut s1: *mut TCCState,
    mut filename: *const std::os::raw::c_char,
) -> std::os::raw::c_int {
    let mut current_block: u64;
    let mut i: std::os::raw::c_int = 0;
    let mut ret: std::os::raw::c_int = 0;
    let mut phnum: std::os::raw::c_int = 0;
    let mut phfill: std::os::raw::c_int = 0;
    let mut shnum: std::os::raw::c_int = 0;
    let mut file_type: std::os::raw::c_int = 0;
    let mut file_offset: std::os::raw::c_int = 0;
    let mut sec_order: *mut std::os::raw::c_int = 0 as *mut std::os::raw::c_int;
    let mut dyninf: dyn_inf = {
        let mut init = dyn_inf {
            dynamic: 0 as *mut Section,
            dynstr: 0 as *mut Section,
            data_offset: 0,
            rel_addr: 0,
            rel_size: 0,
        };
        init
    };
    let mut roinf: ro_inf = ro_inf {
        sh_offset: 0,
        sh_addr: 0,
        sh_size: 0,
    };
    let mut phdr: *mut Elf64_Phdr = 0 as *mut Elf64_Phdr;
    let mut interp: *mut Section = 0 as *mut Section;
    let mut dynamic: *mut Section = 0 as *mut Section;
    let mut dynstr: *mut Section = 0 as *mut Section;
    let mut note: *mut Section = 0 as *mut Section;
    let mut roinf_use: *mut ro_inf = 0 as *mut ro_inf;
    let mut textrel: std::os::raw::c_int = 0;
    file_type = (*s1).output_type;
    (*s1).nb_errors = 0 as std::os::raw::c_int;
    ret = -(1 as std::os::raw::c_int);
    phdr = 0 as *mut Elf64_Phdr;
    sec_order = 0 as *mut std::os::raw::c_int;
    note = 0 as *mut Section;
    dynstr = note;
    dynamic = dynstr;
    interp = dynamic;
    /* if linking, also link in runtime libraries (libc, libgcc, etc.) */
    tcc_add_runtime(s1);
    resolve_common_syms(s1);
    if (*s1).static_link == 0 {
        if file_type == 2 as std::os::raw::c_int {
            let mut ptr: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
            /* allow override the dynamic loader */
            let mut elfint: *const std::os::raw::c_char =
                getenv(b"LD_SO\x00" as *const u8 as *const std::os::raw::c_char);
            if elfint.is_null() {
                elfint =
                    b"/lib64/ld-linux-x86-64.so.2\x00" as *const u8 as *const std::os::raw::c_char
            }
            /* add interpreter section only if executable */
            interp = new_section(
                s1,
                b".interp\x00" as *const u8 as *const std::os::raw::c_char,
                1 as std::os::raw::c_int,
                (1 as std::os::raw::c_int) << 1 as std::os::raw::c_int,
            );
            (*interp).sh_addralign = 1 as std::os::raw::c_int;
            ptr = section_ptr_add(
                interp,
                (1 as std::os::raw::c_int as std::os::raw::c_ulong).wrapping_add(strlen(elfint)),
            ) as *mut std::os::raw::c_char;
            strcpy(ptr, elfint);
        }
        /* add dynamic symbol table */
        (*s1).dynsym = new_symtab(
            s1,
            b".dynsym\x00" as *const u8 as *const std::os::raw::c_char,
            11 as std::os::raw::c_int,
            (1 as std::os::raw::c_int) << 1 as std::os::raw::c_int,
            b".dynstr\x00" as *const u8 as *const std::os::raw::c_char,
            b".hash\x00" as *const u8 as *const std::os::raw::c_char,
            (1 as std::os::raw::c_int) << 1 as std::os::raw::c_int,
        );
        /* Number of local symbols (readelf complains if not set) */
        (*(*s1).dynsym).sh_info = 1 as std::os::raw::c_int;
        dynstr = (*(*s1).dynsym).link;
        /* add dynamic section */
        dynamic = new_section(
            s1,
            b".dynamic\x00" as *const u8 as *const std::os::raw::c_char,
            6 as std::os::raw::c_int,
            (1 as std::os::raw::c_int) << 1 as std::os::raw::c_int
                | (1 as std::os::raw::c_int) << 0 as std::os::raw::c_int,
        );
        (*dynamic).link = dynstr;
        (*dynamic).sh_entsize =
            ::std::mem::size_of::<Elf64_Dyn>() as std::os::raw::c_ulong as std::os::raw::c_int;
        build_got(s1);
        if file_type == 2 as std::os::raw::c_int {
            bind_exe_dynsyms(s1);
            if (*s1).nb_errors != 0 {
                current_block = 12051599741940511589;
            } else {
                bind_libs_dynsyms(s1);
                current_block = 3275366147856559585;
            }
        } else {
            /* shared library case: simply export all global symbols */
            export_global_syms(s1);
            current_block = 3275366147856559585;
        }
    } else {
        current_block = 3275366147856559585;
    }
    match current_block {
        3275366147856559585 => {
            build_got_entries(s1);
            version_add(s1);
            textrel = set_sec_sizes(s1);
            alloc_sec_names(s1, 0 as std::os::raw::c_int);
            if (*s1).static_link == 0 {
                let mut i_0: std::os::raw::c_int = 0;
                /* add a list of needed dlls */
                i_0 = 0 as std::os::raw::c_int;
                while i_0 < (*s1).nb_loaded_dlls {
                    let mut dllref: *mut DLLReference = *(*s1).loaded_dlls.offset(i_0 as isize);
                    if (*dllref).level == 0 as std::os::raw::c_int {
                        put_dt(
                            dynamic,
                            1 as std::os::raw::c_int,
                            put_elf_str(dynstr, (*dllref).name.as_mut_ptr()) as Elf64_Addr,
                        );
                    }
                    i_0 += 1
                }
                if !(*s1).rpath.is_null() {
                    put_dt(
                        dynamic,
                        if (*s1).enable_new_dtags as std::os::raw::c_int != 0 {
                            29 as std::os::raw::c_int
                        } else {
                            15 as std::os::raw::c_int
                        },
                        put_elf_str(dynstr, (*s1).rpath) as Elf64_Addr,
                    );
                }
                if file_type == 3 as std::os::raw::c_int {
                    if !(*s1).soname.is_null() {
                        put_dt(
                            dynamic,
                            14 as std::os::raw::c_int,
                            put_elf_str(dynstr, (*s1).soname) as Elf64_Addr,
                        );
                    }
                    /* XXX: currently, since we do not handle PIC code, we
                    must relocate the readonly segments */
                    if textrel != 0 {
                        put_dt(
                            dynamic,
                            22 as std::os::raw::c_int,
                            0 as std::os::raw::c_int as Elf64_Addr,
                        );
                    }
                }
                if (*s1).symbolic != 0 {
                    put_dt(
                        dynamic,
                        16 as std::os::raw::c_int,
                        0 as std::os::raw::c_int as Elf64_Addr,
                    );
                }
                dyninf.dynamic = dynamic;
                dyninf.dynstr = dynstr;
                /* remember offset and reserve space for 2nd call below */
                dyninf.data_offset = (*dynamic).data_offset;
                fill_dynamic(s1, &mut dyninf);
                (*dynamic).sh_size = (*dynamic).data_offset;
                (*dynstr).sh_size = (*dynstr).data_offset
            }
            i = 1 as std::os::raw::c_int;
            while i < (*s1).nb_sections
                && (**(*s1).sections.offset(i as isize)).sh_flags
                    & (1 as std::os::raw::c_int) << 10 as std::os::raw::c_int
                    == 0
            {
                i += 1
            }
            phfill = 2 as std::os::raw::c_int + (i < (*s1).nb_sections) as std::os::raw::c_int;
            /* compute number of program headers */
            if file_type == 3 as std::os::raw::c_int {
                phnum = 3 as std::os::raw::c_int
            } else if (*s1).static_link != 0 {
                phnum = 3 as std::os::raw::c_int
            } else {
                phnum = 5 as std::os::raw::c_int + (i < (*s1).nb_sections) as std::os::raw::c_int
            }
            phnum +=
                (note != 0 as *mut std::os::raw::c_void as *mut Section) as std::os::raw::c_int;
            /* GNU_RELRO */
            phnum += 1;
            roinf_use = &mut roinf;
            /* allocate program segment headers */
            phdr = tcc_mallocz(
                (phnum as std::os::raw::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<Elf64_Phdr>() as std::os::raw::c_ulong),
            ) as *mut Elf64_Phdr;
            /* compute number of sections */
            shnum = (*s1).nb_sections;
            /* this array is used to reorder sections in the output file */
            sec_order = tcc_malloc(
                (::std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong)
                    .wrapping_mul(shnum as std::os::raw::c_ulong),
            ) as *mut std::os::raw::c_int;
            *sec_order.offset(0 as std::os::raw::c_int as isize) = 0 as std::os::raw::c_int;
            /* compute section to program header mapping */
            file_offset = layout_sections(
                s1,
                phdr,
                phnum,
                phfill,
                interp,
                &mut roinf,
                sec_order.offset(1 as std::os::raw::c_int as isize),
            );
            /* Fill remaining program header and finalize relocation related to dynamic
            linking. */
            fill_unloadable_phdr(phdr, phnum, interp, dynamic, note, roinf_use);
            if !dynamic.is_null() {
                let mut sym: *mut Elf64_Sym = 0 as *mut Elf64_Sym;
                /* put in GOT the dynamic section address and relocate PLT */
                write32le((*(*s1).got).data, (*dynamic).sh_addr as uint32_t);
                if file_type == 2 as std::os::raw::c_int
                    || 1 as std::os::raw::c_int != 0 && file_type == 3 as std::os::raw::c_int
                {
                    relocate_plt(s1);
                }
                /* relocate symbols in .dynsym now that final addresses are known */
                sym = ((*(*s1).dynsym).data as *mut Elf64_Sym)
                    .offset(1 as std::os::raw::c_int as isize);
                while sym
                    < (*(*s1).dynsym)
                        .data
                        .offset((*(*s1).dynsym).data_offset as isize)
                        as *mut Elf64_Sym
                {
                    if (*sym).st_shndx as std::os::raw::c_int != 0 as std::os::raw::c_int
                        && ((*sym).st_shndx as std::os::raw::c_int) < 0xff00 as std::os::raw::c_int
                    {
                        /* do symbol relocation */
                        (*sym).st_value = ((*sym).st_value as std::os::raw::c_ulong).wrapping_add(
                            (**(*s1).sections.offset((*sym).st_shndx as isize)).sh_addr,
                        ) as Elf64_Addr as Elf64_Addr
                    }
                    sym = sym.offset(1)
                }
            }
            /* if building executable or DLL, then relocate each section
            except the GOT which is already relocated */
            relocate_syms(s1, (*s1).symtab, 0 as std::os::raw::c_int);
            ret = -(1 as std::os::raw::c_int);
            if !((*s1).nb_errors != 0 as std::os::raw::c_int) {
                relocate_sections(s1);
                if !dynamic.is_null() {
                    update_reloc_sections(s1, &mut dyninf);
                    (*dynamic).data_offset = dyninf.data_offset;
                    fill_dynamic(s1, &mut dyninf);
                }
                tidy_section_headers(s1, sec_order);
                /* Perform relocation to GOT or PLT entries */
                if file_type == 2 as std::os::raw::c_int
                    && (*s1).static_link as std::os::raw::c_int != 0
                {
                    fill_got(s1);
                } else if !(*s1).got.is_null() {
                    fill_local_got_entries(s1);
                }
                /* Create the ELF file with name 'filename' */
                ret = tcc_write_elf_file(s1, filename, phnum, phdr, file_offset, sec_order);
                (*s1).nb_sections = shnum
            }
        },
        _ => {},
    }
    tcc_free(sec_order as *mut std::os::raw::c_void);
    tcc_free(phdr as *mut std::os::raw::c_void);
    return ret;
}
/* ndef ELF_OBJ_ONLY */
/* Allocate strings for section names */
unsafe extern "C" fn alloc_sec_names(mut s1: *mut TCCState, mut is_obj: std::os::raw::c_int) {
    let mut i: std::os::raw::c_int = 0;
    let mut s: *mut Section = 0 as *mut Section;
    let mut strsec: *mut Section = 0 as *mut Section;
    strsec = new_section(
        s1,
        b".shstrtab\x00" as *const u8 as *const std::os::raw::c_char,
        3 as std::os::raw::c_int,
        0 as std::os::raw::c_int,
    );
    put_elf_str(strsec, b"\x00" as *const u8 as *const std::os::raw::c_char);
    i = 1 as std::os::raw::c_int;
    while i < (*s1).nb_sections {
        s = *(*s1).sections.offset(i as isize);
        if is_obj != 0 {
            (*s).sh_size = (*s).data_offset
        }
        if s == strsec
            || (*s).sh_size != 0
            || (*s).sh_flags & (1 as std::os::raw::c_int) << 1 as std::os::raw::c_int != 0
        {
            (*s).sh_name = put_elf_str(strsec, (*s).name.as_mut_ptr())
        }
        i += 1
    }
    (*strsec).sh_size = (*strsec).data_offset;
}
unsafe extern "C" fn layout_any_sections(
    mut s1: *mut TCCState,
    mut file_offset: std::os::raw::c_int,
    mut sec_order: *mut std::os::raw::c_int,
    mut is_obj: std::os::raw::c_int,
) -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    let mut s: *mut Section = 0 as *mut Section;
    i = 1 as std::os::raw::c_int;
    while i < (*s1).nb_sections {
        s = *(*s1).sections.offset(i as isize);
        if !(is_obj == 0
            && (*s).sh_flags & (1 as std::os::raw::c_int) << 1 as std::os::raw::c_int != 0)
        {
            let fresh11 = sec_order;
            sec_order = sec_order.offset(1);
            *fresh11 = i;
            file_offset = file_offset + (*s).sh_addralign - 1 as std::os::raw::c_int
                & !((*s).sh_addralign - 1 as std::os::raw::c_int);
            (*s).sh_offset = file_offset as std::os::raw::c_ulong;
            if (*s).sh_type != 8 as std::os::raw::c_int {
                file_offset = (file_offset as std::os::raw::c_ulong).wrapping_add((*s).sh_size)
                    as std::os::raw::c_int as std::os::raw::c_int
            }
        }
        i += 1
    }
    return file_offset;
}
/* Output an elf .o file */
unsafe extern "C" fn elf_output_obj(
    mut s1: *mut TCCState,
    mut filename: *const std::os::raw::c_char,
) -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0;
    let mut file_offset: std::os::raw::c_int = 0;
    let mut sec_order: *mut std::os::raw::c_int = 0 as *mut std::os::raw::c_int;
    (*s1).nb_errors = 0 as std::os::raw::c_int;
    /* Allocate strings for section names */
    alloc_sec_names(s1, 1 as std::os::raw::c_int);
    /* this array is used to reorder sections in the output file */
    sec_order = tcc_malloc(
        (::std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong)
            .wrapping_mul((*s1).nb_sections as std::os::raw::c_ulong),
    ) as *mut std::os::raw::c_int;
    *sec_order.offset(0 as std::os::raw::c_int as isize) = 0 as std::os::raw::c_int;
    file_offset = layout_any_sections(
        s1,
        ::std::mem::size_of::<Elf64_Ehdr>() as std::os::raw::c_ulong as std::os::raw::c_int,
        sec_order.offset(1 as std::os::raw::c_int as isize),
        1 as std::os::raw::c_int,
    );
    /* Create the ELF file with name 'filename' */
    ret = tcc_write_elf_file(
        s1,
        filename,
        0 as std::os::raw::c_int,
        0 as *mut Elf64_Phdr,
        file_offset,
        sec_order,
    );
    tcc_free(sec_order as *mut std::os::raw::c_void);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn tcc_output_file(
    mut s: *mut TCCState,
    mut filename: *const std::os::raw::c_char,
) -> std::os::raw::c_int {
    if (*s).test_coverage != 0 {
        tcc_tcov_add_file(s, filename);
    }
    if (*s).output_type == 4 as std::os::raw::c_int {
        return elf_output_obj(s, filename);
    }
    return elf_output_file(s, filename);
}
#[no_mangle]
pub unsafe extern "C" fn full_read(
    mut fd: std::os::raw::c_int,
    mut buf: *mut std::os::raw::c_void,
    mut count: size_t,
) -> ssize_t {
    let mut cbuf: *mut std::os::raw::c_char = buf as *mut std::os::raw::c_char;
    let mut rnum: size_t = 0 as std::os::raw::c_int as size_t;
    loop {
        let mut num: ssize_t = read(
            fd,
            cbuf as *mut std::os::raw::c_void,
            count.wrapping_sub(rnum),
        );
        if num < 0 as std::os::raw::c_int as std::os::raw::c_long {
            return num;
        }
        if num == 0 as std::os::raw::c_int as std::os::raw::c_long {
            return rnum as ssize_t;
        }
        rnum = (rnum as std::os::raw::c_ulong).wrapping_add(num as std::os::raw::c_ulong) as size_t
            as size_t;
        cbuf = cbuf.offset(num as isize)
    }
}
#[no_mangle]
pub unsafe extern "C" fn load_data(
    mut fd: std::os::raw::c_int,
    mut file_offset: std::os::raw::c_ulong,
    mut size: std::os::raw::c_ulong,
) -> *mut std::os::raw::c_void {
    let mut data: *mut std::os::raw::c_void = 0 as *mut std::os::raw::c_void;
    data = tcc_malloc(size);
    lseek(fd, file_offset as __off_t, 0 as std::os::raw::c_int);
    full_read(fd, data, size);
    return data;
}
#[no_mangle]
pub unsafe extern "C" fn tcc_object_type(
    mut fd: std::os::raw::c_int,
    mut h: *mut Elf64_Ehdr,
) -> std::os::raw::c_int {
    let mut size: std::os::raw::c_int = full_read(
        fd,
        h as *mut std::os::raw::c_void,
        ::std::mem::size_of::<Elf64_Ehdr>() as std::os::raw::c_ulong,
    ) as std::os::raw::c_int;
    if size as std::os::raw::c_ulong == ::std::mem::size_of::<Elf64_Ehdr>() as std::os::raw::c_ulong
        && 0 as std::os::raw::c_int
            == memcmp(
                h as *const std::os::raw::c_void,
                b"\x7fELF\x00" as *const u8 as *const std::os::raw::c_char
                    as *const std::os::raw::c_void,
                4 as std::os::raw::c_int as std::os::raw::c_ulong,
            )
    {
        if (*h).e_type as std::os::raw::c_int == 1 as std::os::raw::c_int {
            return 1 as std::os::raw::c_int;
        }
        if (*h).e_type as std::os::raw::c_int == 3 as std::os::raw::c_int {
            return 2 as std::os::raw::c_int;
        }
    } else if size >= 8 as std::os::raw::c_int {
        if 0 as std::os::raw::c_int
            == memcmp(
                h as *const std::os::raw::c_void,
                b"!<arch>\n\x00" as *const u8 as *const std::os::raw::c_char
                    as *const std::os::raw::c_void,
                8 as std::os::raw::c_int as std::os::raw::c_ulong,
            )
        {
            return 3 as std::os::raw::c_int;
        }
    }
    return 0 as std::os::raw::c_int;
}
/* load an object file and merge it with current files */
/* XXX: handle correctly stab (debug) info */
#[no_mangle]
pub unsafe extern "C" fn tcc_load_object_file(
    mut s1: *mut TCCState,
    mut fd: std::os::raw::c_int,
    mut file_offset: std::os::raw::c_ulong,
) -> std::os::raw::c_int {
    let mut current_block: u64;
    let mut ehdr: Elf64_Ehdr = Elf64_Ehdr {
        e_ident: [0; 16],
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
    };
    let mut shdr: *mut Elf64_Shdr = 0 as *mut Elf64_Shdr;
    let mut sh: *mut Elf64_Shdr = 0 as *mut Elf64_Shdr;
    let mut size: std::os::raw::c_int = 0;
    let mut i: std::os::raw::c_int = 0;
    let mut j: std::os::raw::c_int = 0;
    let mut offset: std::os::raw::c_int = 0;
    let mut offseti: std::os::raw::c_int = 0;
    let mut nb_syms: std::os::raw::c_int = 0;
    let mut sym_index: std::os::raw::c_int = 0;
    let mut ret: std::os::raw::c_int = 0;
    let mut seencompressed: std::os::raw::c_int = 0;
    let mut strsec: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut strtab: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut stab_index: std::os::raw::c_int = 0;
    let mut stabstr_index: std::os::raw::c_int = 0;
    let mut old_to_new_syms: *mut std::os::raw::c_int = 0 as *mut std::os::raw::c_int;
    let mut sh_name: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut name: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut sm_table: *mut SectionMergeInfo = 0 as *mut SectionMergeInfo;
    let mut sm: *mut SectionMergeInfo = 0 as *mut SectionMergeInfo;
    let mut sym: *mut Elf64_Sym = 0 as *mut Elf64_Sym;
    let mut symtab: *mut Elf64_Sym = 0 as *mut Elf64_Sym;
    let mut rel: *mut Elf64_Rela = 0 as *mut Elf64_Rela;
    let mut s: *mut Section = 0 as *mut Section;
    lseek(fd, file_offset as __off_t, 0 as std::os::raw::c_int);
    if !(tcc_object_type(fd, &mut ehdr) != 1 as std::os::raw::c_int) {
        /* test CPU specific stuff */
        if !(ehdr.e_ident[5 as std::os::raw::c_int as usize] as std::os::raw::c_int
            != 1 as std::os::raw::c_int
            || ehdr.e_machine as std::os::raw::c_int != 62 as std::os::raw::c_int)
        {
            /* read sections */
            shdr = load_data(
                fd,
                file_offset.wrapping_add(ehdr.e_shoff),
                (::std::mem::size_of::<Elf64_Shdr>() as std::os::raw::c_ulong)
                    .wrapping_mul(ehdr.e_shnum as std::os::raw::c_ulong),
            ) as *mut Elf64_Shdr;
            sm_table = tcc_mallocz(
                (::std::mem::size_of::<SectionMergeInfo>() as std::os::raw::c_ulong)
                    .wrapping_mul(ehdr.e_shnum as std::os::raw::c_ulong),
            ) as *mut SectionMergeInfo;
            /* load section names */
            sh = &mut *shdr.offset(ehdr.e_shstrndx as isize) as *mut Elf64_Shdr;
            strsec = load_data(fd, file_offset.wrapping_add((*sh).sh_offset), (*sh).sh_size)
                as *mut std::os::raw::c_char;
            /* load symtab and strtab */
            old_to_new_syms = 0 as *mut std::os::raw::c_int;
            symtab = 0 as *mut Elf64_Sym;
            strtab = 0 as *mut std::os::raw::c_char;
            nb_syms = 0 as std::os::raw::c_int;
            seencompressed = 0 as std::os::raw::c_int;
            stabstr_index = 0 as std::os::raw::c_int;
            stab_index = stabstr_index;
            i = 1 as std::os::raw::c_int;
            loop {
                if i < ehdr.e_shnum as std::os::raw::c_int {
                    sh = &mut *shdr.offset(i as isize) as *mut Elf64_Shdr;
                    if (*sh).sh_type == 2 as std::os::raw::c_int as std::os::raw::c_uint {
                        if !symtab.is_null() {
                            tcc_enter_state(s1);
                            Some(
                                _tcc_error_noabort
                                    as unsafe extern "C" fn(
                                        _: *const std::os::raw::c_char,
                                        _: ...
                                    )
                                        -> (),
                            )
                            .expect("non-null function pointer")(
                                b"object must contain only one symtab\x00" as *const u8
                                    as *const std::os::raw::c_char,
                            );
                            current_block = 7392824444931029276;
                            break;
                        } else {
                            nb_syms = (*sh).sh_size.wrapping_div(
                                ::std::mem::size_of::<Elf64_Sym>() as std::os::raw::c_ulong
                            ) as std::os::raw::c_int;
                            symtab = load_data(
                                fd,
                                file_offset.wrapping_add((*sh).sh_offset),
                                (*sh).sh_size,
                            ) as *mut Elf64_Sym;
                            let ref mut fresh12 = (*sm_table.offset(i as isize)).s;
                            *fresh12 = (*s1).symtab_section;
                            /* now load strtab */
                            sh = &mut *shdr.offset((*sh).sh_link as isize) as *mut Elf64_Shdr;
                            strtab = load_data(
                                fd,
                                file_offset.wrapping_add((*sh).sh_offset),
                                (*sh).sh_size,
                            ) as *mut std::os::raw::c_char
                        }
                    }
                    if (*sh).sh_flags
                        & ((1 as std::os::raw::c_int) << 11 as std::os::raw::c_int)
                            as std::os::raw::c_ulong
                        != 0
                    {
                        seencompressed = 1 as std::os::raw::c_int
                    }
                    i += 1
                } else {
                    /* now examine each section and try to merge its content with the
                    ones in memory */
                    i = 1 as std::os::raw::c_int;
                    current_block = 7333393191927787629;
                    break;
                }
            }
            's_176: loop {
                match current_block {
                    7392824444931029276 => {
                        ret = -(1 as std::os::raw::c_int);
                        break;
                    },
                    _ => {
                        if i < ehdr.e_shnum as std::os::raw::c_int {
                            /* no need to examine section name strtab */
                            if !(i == ehdr.e_shstrndx as std::os::raw::c_int) {
                                sh = &mut *shdr.offset(i as isize) as *mut Elf64_Shdr;
                                if (*sh).sh_type == 4 as std::os::raw::c_int as std::os::raw::c_uint
                                {
                                    sh =
                                        &mut *shdr.offset((*sh).sh_info as isize) as *mut Elf64_Shdr
                                }
                                /* ignore sections types we do not handle (plus relocs to those) */
                                if !((*sh).sh_type
                                    != 1 as std::os::raw::c_int as std::os::raw::c_uint
                                    && (*sh).sh_type
                                        != 7 as std::os::raw::c_int as std::os::raw::c_uint
                                    && (*sh).sh_type
                                        != 8 as std::os::raw::c_int as std::os::raw::c_uint
                                    && (*sh).sh_type
                                        != 16 as std::os::raw::c_int as std::os::raw::c_uint
                                    && (*sh).sh_type
                                        != 14 as std::os::raw::c_int as std::os::raw::c_uint
                                    && (*sh).sh_type
                                        != 15 as std::os::raw::c_int as std::os::raw::c_uint
                                    && strcmp(
                                        strsec.offset((*sh).sh_name as isize),
                                        b".stabstr\x00" as *const u8 as *const std::os::raw::c_char,
                                    ) != 0)
                                {
                                    if !(seencompressed != 0
                                        && strncmp(
                                            strsec.offset((*sh).sh_name as isize),
                                            b".debug_\x00" as *const u8
                                                as *const std::os::raw::c_char,
                                            (::std::mem::size_of::<[std::os::raw::c_char; 8]>()
                                                as std::os::raw::c_ulong)
                                                .wrapping_sub(
                                                    1 as std::os::raw::c_int
                                                        as std::os::raw::c_ulong,
                                                ),
                                        ) == 0)
                                    {
                                        sh = &mut *shdr.offset(i as isize) as *mut Elf64_Shdr;
                                        sh_name = strsec.offset((*sh).sh_name as isize);
                                        if (*sh).sh_addralign
                                            < 1 as std::os::raw::c_int as std::os::raw::c_ulong
                                        {
                                            (*sh).sh_addralign =
                                                1 as std::os::raw::c_int as Elf64_Xword
                                        }
                                        /* find corresponding section, if any */
                                        j = 1 as std::os::raw::c_int;
                                        loop {
                                            if !(j < (*s1).nb_sections) {
                                                current_block = 4741994311446740739;
                                                break;
                                            }
                                            s = *(*s1).sections.offset(j as isize);
                                            if strcmp((*s).name.as_mut_ptr(), sh_name) == 0 {
                                                if strncmp(sh_name,
                                                               b".gnu.linkonce\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const std::os::raw::c_char,
                                                               (::std::mem::size_of::<[std::os::raw::c_char; 14]>()
                                                                    as
                                                                    std::os::raw::c_ulong).wrapping_sub(1
                                                                                                    as
                                                                                                    std::os::raw::c_int
                                                                                                    as
                                                                                                    std::os::raw::c_ulong))
                                                           == 0 {
                                                        /* if a 'linkonce' section is already present, we
                       do not add it again. It is a little tricky as
                       symbols can still be defined in
                       it. */
                                                        (*sm_table.offset(i as
                                                                              isize)).link_once
                                                            =
                                                            1 as std::os::raw::c_int
                                                                as uint8_t;
                                                        current_block =
                                                            7245201122033322888;
                                                        break ;
                                                    } else {
                                                        if !(*s1).stab_section.is_null()
                                                           {
                                                            if s ==
                                                                   (*s1).stab_section
                                                               {
                                                                stab_index = i
                                                            }
                                                            if s ==
                                                                   (*(*s1).stab_section).link
                                                               {
                                                                stabstr_index
                                                                    = i
                                                            }
                                                        }
                                                        current_block =
                                                            17985269909145287291;
                                                        break ;
                                                    }
                                            } else {
                                                j += 1
                                            }
                                        }
                                        match current_block {
                                            7245201122033322888 => {},
                                            _ => {
                                                match current_block {
                                                    4741994311446740739 => {
                                                        /* not found: create new section */
                                                        s = new_section(
                                                            s1,
                                                            sh_name,
                                                            (*sh).sh_type as std::os::raw::c_int,
                                                            ((*sh).sh_flags
                                                                & !((1 as std::os::raw::c_int)
                                                                    << 9 as std::os::raw::c_int)
                                                                    as std::os::raw::c_ulong)
                                                                as std::os::raw::c_int,
                                                        );
                                                        /* take as much info as possible from the section. sh_link and
                                                        sh_info will be updated later */
                                                        (*s).sh_addralign = (*sh).sh_addralign
                                                            as std::os::raw::c_int;
                                                        (*s).sh_entsize =
                                                            (*sh).sh_entsize as std::os::raw::c_int;
                                                        (*sm_table.offset(i as isize)).new_section =
                                                            1 as std::os::raw::c_int as uint8_t
                                                    },
                                                    _ => {},
                                                }
                                                if (*sh).sh_type
                                                    != (*s).sh_type as std::os::raw::c_uint
                                                {
                                                    tcc_enter_state(s1);
                                                    Some(
                                                        _tcc_error_noabort
                                                            as unsafe extern "C" fn(
                                                                _: *const std::os::raw::c_char,
                                                                _: ...
                                                            )
                                                                -> (),
                                                    )
                                                    .expect("non-null function pointer")(
                                                        b"invalid section type\x00" as *const u8
                                                            as *const std::os::raw::c_char,
                                                    );
                                                    current_block = 7392824444931029276;
                                                    continue;
                                                } else {
                                                    /* align start of section */
                                                    (*s).data_offset =
                                                        (*s).data_offset.wrapping_add(
                                                            (*s).data_offset.wrapping_neg()
                                                                & (*sh).sh_addralign.wrapping_sub(
                                                                    1 as std::os::raw::c_int
                                                                        as std::os::raw::c_ulong,
                                                                ),
                                                        );
                                                    if (*sh).sh_addralign
                                                        > (*s).sh_addralign as std::os::raw::c_ulong
                                                    {
                                                        (*s).sh_addralign = (*sh).sh_addralign
                                                            as std::os::raw::c_int
                                                    }
                                                    (*sm_table.offset(i as isize)).offset =
                                                        (*s).data_offset;
                                                    let ref mut fresh13 =
                                                        (*sm_table.offset(i as isize)).s;
                                                    *fresh13 = s;
                                                    /* concatenate sections */
                                                    size = (*sh).sh_size as std::os::raw::c_int;
                                                    if (*sh).sh_type
                                                        != 8 as std::os::raw::c_int
                                                            as std::os::raw::c_uint
                                                    {
                                                        let mut ptr: *mut std::os::raw::c_uchar =
                                                            0 as *mut std::os::raw::c_uchar;
                                                        lseek(
                                                            fd,
                                                            file_offset
                                                                .wrapping_add((*sh).sh_offset)
                                                                as __off_t,
                                                            0 as std::os::raw::c_int,
                                                        );
                                                        ptr = section_ptr_add(s, size as Elf64_Addr)
                                                            as *mut std::os::raw::c_uchar;
                                                        full_read(
                                                            fd,
                                                            ptr as *mut std::os::raw::c_void,
                                                            size as size_t,
                                                        );
                                                    } else {
                                                        (*s).data_offset =
                                                            (*s).data_offset.wrapping_add(
                                                                size as std::os::raw::c_ulong,
                                                            )
                                                    }
                                                }
                                            },
                                        }
                                    }
                                }
                            }
                            i += 1;
                            current_block = 7333393191927787629;
                        } else {
                            /* gr relocate stab strings */
                            if stab_index != 0 && stabstr_index != 0 {
                                let mut a: *mut Stab_Sym = 0 as *mut Stab_Sym;
                                let mut b: *mut Stab_Sym = 0 as *mut Stab_Sym;
                                let mut o: std::os::raw::c_uint = 0;
                                s = (*sm_table.offset(stab_index as isize)).s;
                                a = (*s)
                                    .data
                                    .offset((*sm_table.offset(stab_index as isize)).offset as isize)
                                    as *mut Stab_Sym;
                                b = (*s).data.offset((*s).data_offset as isize) as *mut Stab_Sym;
                                o = (*sm_table.offset(stabstr_index as isize)).offset
                                    as std::os::raw::c_uint;
                                while a < b {
                                    if (*a).n_strx != 0 {
                                        (*a).n_strx = (*a).n_strx.wrapping_add(o)
                                    }
                                    a = a.offset(1)
                                }
                            }
                            /* second short pass to update sh_link and sh_info fields of new
                            sections */
                            i = 1 as std::os::raw::c_int;
                            while i < ehdr.e_shnum as std::os::raw::c_int {
                                s = (*sm_table.offset(i as isize)).s;
                                if !(s.is_null() || (*sm_table.offset(i as isize)).new_section == 0)
                                {
                                    sh = &mut *shdr.offset(i as isize) as *mut Elf64_Shdr;
                                    if (*sh).sh_link
                                        > 0 as std::os::raw::c_int as std::os::raw::c_uint
                                    {
                                        (*s).link = (*sm_table.offset((*sh).sh_link as isize)).s
                                    }
                                    if (*sh).sh_type
                                        == 4 as std::os::raw::c_int as std::os::raw::c_uint
                                    {
                                        (*s).sh_info =
                                            (*(*sm_table.offset((*sh).sh_info as isize)).s).sh_num;
                                        /* update backward link */
                                        let ref mut fresh14 =
                                            (**(*s1).sections.offset((*s).sh_info as isize)).reloc;
                                        *fresh14 = s
                                    }
                                }
                                i += 1
                            }
                            /* resolve symbols */
                            old_to_new_syms = tcc_mallocz(
                                (nb_syms as std::os::raw::c_ulong)
                                    .wrapping_mul(::std::mem::size_of::<std::os::raw::c_int>()
                                        as std::os::raw::c_ulong),
                            )
                                as *mut std::os::raw::c_int;
                            sym = symtab.offset(1 as std::os::raw::c_int as isize);
                            let mut current_block_95: u64;
                            i = 1 as std::os::raw::c_int;
                            while i < nb_syms {
                                if (*sym).st_shndx as std::os::raw::c_int
                                    != 0 as std::os::raw::c_int
                                    && ((*sym).st_shndx as std::os::raw::c_int)
                                        < 0xff00 as std::os::raw::c_int
                                {
                                    sm = &mut *sm_table.offset((*sym).st_shndx as isize)
                                        as *mut SectionMergeInfo;
                                    if (*sm).link_once != 0 {
                                        /* if a symbol is in a link once section, we use the
                                        already defined symbol. It is very important to get
                                        correct relocations */
                                        if (*sym).st_info as std::os::raw::c_int
                                            >> 4 as std::os::raw::c_int
                                            != 0 as std::os::raw::c_int
                                        {
                                            name = strtab.offset((*sym).st_name as isize);
                                            sym_index = find_elf_sym((*s1).symtab_section, name);
                                            if sym_index != 0 {
                                                *old_to_new_syms.offset(i as isize) = sym_index
                                            }
                                        }
                                        current_block_95 = 7739940392431776979;
                                    } else if (*sm).s.is_null() {
                                        current_block_95 = 7739940392431776979;
                                    } else {
                                        /* if no corresponding section added, no need to add symbol */
                                        /* convert section number */
                                        (*sym).st_shndx = (*(*sm).s).sh_num as Elf64_Section;
                                        /* offset value */
                                        (*sym).st_value = ((*sym).st_value as std::os::raw::c_ulong)
                                            .wrapping_add((*sm).offset)
                                            as Elf64_Addr
                                            as Elf64_Addr;
                                        current_block_95 = 13253659531982233645;
                                    }
                                } else {
                                    current_block_95 = 13253659531982233645;
                                }
                                match current_block_95 {
                                    13253659531982233645 => {
                                        /* add symbol */
                                        name = strtab.offset((*sym).st_name as isize);
                                        sym_index = set_elf_sym(
                                            (*s1).symtab_section,
                                            (*sym).st_value,
                                            (*sym).st_size,
                                            (*sym).st_info as std::os::raw::c_int,
                                            (*sym).st_other as std::os::raw::c_int,
                                            (*sym).st_shndx as std::os::raw::c_int,
                                            name,
                                        );
                                        *old_to_new_syms.offset(i as isize) = sym_index
                                    },
                                    _ => {},
                                }
                                i += 1;
                                sym = sym.offset(1)
                            }
                            /* third pass to patch relocation entries */
                            i = 1 as std::os::raw::c_int;
                            while i < ehdr.e_shnum as std::os::raw::c_int {
                                s = (*sm_table.offset(i as isize)).s;
                                if !s.is_null() {
                                    sh = &mut *shdr.offset(i as isize) as *mut Elf64_Shdr;
                                    offset = (*sm_table.offset(i as isize)).offset
                                        as std::os::raw::c_int;
                                    size = (*sh).sh_size as std::os::raw::c_int;
                                    match (*s).sh_type {
                                        4 => {
                                            /* take relocation offset information */
                                            offseti = (*sm_table.offset((*sh).sh_info as isize))
                                                .offset
                                                as std::os::raw::c_int;
                                            rel = ((*s).data as *mut Elf64_Rela).offset(
                                                (offset as std::os::raw::c_ulong).wrapping_div(
                                                    ::std::mem::size_of::<Elf64_Rela>()
                                                        as std::os::raw::c_ulong,
                                                )
                                                    as isize,
                                            );
                                            while rel
                                                < ((*s).data as *mut Elf64_Rela).offset(
                                                    ((offset + size) as std::os::raw::c_ulong)
                                                        .wrapping_div(::std::mem::size_of::<
                                                            Elf64_Rela,
                                                        >(
                                                        )
                                                            as std::os::raw::c_ulong)
                                                        as isize,
                                                )
                                            {
                                                let mut type_0: std::os::raw::c_int = 0;
                                                let mut sym_index_0: std::os::raw::c_uint = 0;
                                                /* convert symbol index */
                                                type_0 = ((*rel).r_info
                                                    & 0xffffffff as std::os::raw::c_uint
                                                        as std::os::raw::c_ulong)
                                                    as std::os::raw::c_int;
                                                sym_index_0 = ((*rel).r_info
                                                    >> 32 as std::os::raw::c_int)
                                                    as std::os::raw::c_uint;
                                                /* NOTE: only one symtab assumed */
                                                if !(sym_index_0 >= nb_syms as std::os::raw::c_uint)
                                                {
                                                    sym_index_0 = *old_to_new_syms
                                                        .offset(sym_index_0 as isize)
                                                        as std::os::raw::c_uint;
                                                    /* ignore link_once in rel section. */
                                                    if !(sym_index_0 == 0
                                                        && (*sm_table
                                                            .offset((*sh).sh_info as isize))
                                                        .link_once
                                                            == 0)
                                                    {
                                                        (*rel).r_info = ((sym_index_0
                                                            as Elf64_Xword)
                                                            << 32 as std::os::raw::c_int)
                                                            .wrapping_add(
                                                                type_0 as std::os::raw::c_ulong,
                                                            );
                                                        /* offset the relocation offset */
                                                        (*rel).r_offset = ((*rel).r_offset
                                                            as std::os::raw::c_ulong)
                                                            .wrapping_add(
                                                                offseti as std::os::raw::c_ulong,
                                                            )
                                                            as Elf64_Addr
                                                            as Elf64_Addr;
                                                        rel = rel.offset(1);
                                                        continue;
                                                    }
                                                }
                                                tcc_enter_state(s1);
                                                Some(_tcc_error_noabort as
                                                             unsafe extern "C" fn(_:
                                                                                      *const std::os::raw::c_char,
                                                                                  _:
                                                                                      ...)
                                                                 ->
                                                                     ()).expect("non-null function pointer")(b"Invalid relocation entry [%2d] \'%s\' @ %.8x\x00"
                                                                                                                 as
                                                                                                                 *const u8
                                                                                                                 as
                                                                                                                 *const std::os::raw::c_char,
                                                                                                             i,
                                                                                                             strsec.offset((*sh).sh_name
                                                                                                                               as
                                                                                                                               isize),
                                                                                                             (*rel).r_offset
                                                                                                                 as
                                                                                                                 std::os::raw::c_int);
                                                current_block = 7392824444931029276;
                                                continue 's_176;
                                            }
                                        },
                                        _ => {},
                                    }
                                }
                                i += 1
                            }
                            ret = 0 as std::os::raw::c_int;
                            break;
                        }
                    },
                }
            }
            tcc_free(symtab as *mut std::os::raw::c_void);
            tcc_free(strtab as *mut std::os::raw::c_void);
            tcc_free(old_to_new_syms as *mut std::os::raw::c_void);
            tcc_free(sm_table as *mut std::os::raw::c_void);
            tcc_free(strsec as *mut std::os::raw::c_void);
            tcc_free(shdr as *mut std::os::raw::c_void);
            return ret;
        }
    }
    tcc_enter_state(s1);
    Some(_tcc_error_noabort as unsafe extern "C" fn(_: *const std::os::raw::c_char, _: ...) -> ())
        .expect("non-null function pointer")(
        b"invalid object file\x00" as *const u8 as *const std::os::raw::c_char,
    );
    return -(1 as std::os::raw::c_int);
}
unsafe extern "C" fn get_be(
    mut b: *const uint8_t,
    mut n: std::os::raw::c_int,
) -> std::os::raw::c_ulonglong {
    let mut ret: std::os::raw::c_ulonglong = 0 as std::os::raw::c_int as std::os::raw::c_ulonglong;
    while n != 0 {
        let fresh15 = b;
        b = b.offset(1);
        ret = ret << 8 as std::os::raw::c_int | *fresh15 as std::os::raw::c_ulonglong;
        n -= 1
    }
    return ret;
}
unsafe extern "C" fn read_ar_header(
    mut fd: std::os::raw::c_int,
    mut offset: std::os::raw::c_int,
    mut hdr: *mut ArchiveHeader,
) -> std::os::raw::c_int {
    let mut p: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut e: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut len: std::os::raw::c_int = 0;
    lseek(fd, offset as __off_t, 0 as std::os::raw::c_int);
    len = full_read(
        fd,
        hdr as *mut std::os::raw::c_void,
        ::std::mem::size_of::<ArchiveHeader>() as std::os::raw::c_ulong,
    ) as std::os::raw::c_int;
    if len as std::os::raw::c_ulong
        != ::std::mem::size_of::<ArchiveHeader>() as std::os::raw::c_ulong
    {
        return if len != 0 {
            -(1 as std::os::raw::c_int)
        } else {
            0 as std::os::raw::c_int
        };
    }
    p = (*hdr).ar_name.as_mut_ptr();
    e = p.offset(
        ::std::mem::size_of::<[std::os::raw::c_char; 16]>() as std::os::raw::c_ulong as isize,
    );
    while e > p
        && *e.offset(-(1 as std::os::raw::c_int) as isize) as std::os::raw::c_int == ' ' as i32
    {
        e = e.offset(-1)
    }
    *e = '\u{0}' as i32 as std::os::raw::c_char;
    (*hdr).ar_size[(::std::mem::size_of::<[std::os::raw::c_char; 10]>() as std::os::raw::c_ulong)
        .wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong)
        as usize] = 0 as std::os::raw::c_int as std::os::raw::c_char;
    return len;
}
/* load only the objects which resolve undefined symbols */
unsafe extern "C" fn tcc_load_alacarte(
    mut s1: *mut TCCState,
    mut fd: std::os::raw::c_int,
    mut size: std::os::raw::c_int,
    mut entrysize: std::os::raw::c_int,
) -> std::os::raw::c_int {
    let mut current_block: u64;
    let mut i: std::os::raw::c_int = 0;
    let mut bound: std::os::raw::c_int = 0;
    let mut nsyms: std::os::raw::c_int = 0;
    let mut sym_index: std::os::raw::c_int = 0;
    let mut len: std::os::raw::c_int = 0;
    let mut ret: std::os::raw::c_int = -(1 as std::os::raw::c_int);
    let mut off: std::os::raw::c_ulonglong = 0;
    let mut data: *mut uint8_t = 0 as *mut uint8_t;
    let mut ar_names: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut p: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut ar_index: *const uint8_t = 0 as *const uint8_t;
    let mut sym: *mut Elf64_Sym = 0 as *mut Elf64_Sym;
    let mut hdr: ArchiveHeader = ArchiveHeader {
        ar_name: [0; 16],
        ar_date: [0; 12],
        ar_uid: [0; 6],
        ar_gid: [0; 6],
        ar_mode: [0; 8],
        ar_size: [0; 10],
        ar_fmag: [0; 2],
    };
    data = tcc_malloc(size as std::os::raw::c_ulong) as *mut uint8_t;
    if !(full_read(fd, data as *mut std::os::raw::c_void, size as size_t)
        != size as std::os::raw::c_long)
    {
        nsyms = get_be(data, entrysize) as std::os::raw::c_int;
        ar_index = data.offset(entrysize as isize);
        ar_names = (ar_index as *mut std::os::raw::c_char).offset((nsyms * entrysize) as isize);
        's_48: loop {
            bound = 0 as std::os::raw::c_int;
            p = ar_names;
            i = 0 as std::os::raw::c_int;
            while i < nsyms {
                let mut s: *mut Section = (*s1).symtab_section;
                sym_index = find_elf_sym(s, p);
                if !(sym_index == 0) {
                    sym = &mut *((*s).data as *mut Elf64_Sym).offset(sym_index as isize)
                        as *mut Elf64_Sym;
                    if !((*sym).st_shndx as std::os::raw::c_int != 0 as std::os::raw::c_int) {
                        off = get_be(ar_index.offset((i * entrysize) as isize), entrysize);
                        len = read_ar_header(fd, off as std::os::raw::c_int, &mut hdr);
                        if len <= 0 as std::os::raw::c_int
                            || memcmp(
                                hdr.ar_fmag.as_mut_ptr() as *const std::os::raw::c_void,
                                b"`\n\x00" as *const u8 as *const std::os::raw::c_char
                                    as *const std::os::raw::c_void,
                                2 as std::os::raw::c_int as std::os::raw::c_ulong,
                            ) != 0
                        {
                            tcc_enter_state(s1);
                            Some(
                                _tcc_error_noabort
                                    as unsafe extern "C" fn(
                                        _: *const std::os::raw::c_char,
                                        _: ...
                                    )
                                        -> (),
                            )
                            .expect("non-null function pointer")(
                                b"invalid archive\x00" as *const u8 as *const std::os::raw::c_char,
                            );
                            current_block = 12057219424739730934;
                            break 's_48;
                        } else {
                            off = off.wrapping_add(len as std::os::raw::c_ulonglong);
                            if (*s1).verbose as std::os::raw::c_int == 2 as std::os::raw::c_int {
                                printf(
                                    b"   -> %s\n\x00" as *const u8 as *const std::os::raw::c_char,
                                    hdr.ar_name.as_mut_ptr(),
                                );
                            }
                            if tcc_load_object_file(s1, fd, off as std::os::raw::c_ulong)
                                < 0 as std::os::raw::c_int
                            {
                                current_block = 12057219424739730934;
                                break 's_48;
                            }
                            bound += 1
                        }
                    }
                }
                i += 1;
                p = p.offset(
                    strlen(p).wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong)
                        as isize,
                )
            }
            if !(bound != 0) {
                current_block = 14818589718467733107;
                break;
            }
        }
        match current_block {
            12057219424739730934 => {},
            _ => ret = 0 as std::os::raw::c_int,
        }
    }
    tcc_free(data as *mut std::os::raw::c_void);
    return ret;
}
/* load a '.a' file */
#[no_mangle]
pub unsafe extern "C" fn tcc_load_archive(
    mut s1: *mut TCCState,
    mut fd: std::os::raw::c_int,
    mut alacarte: std::os::raw::c_int,
) -> std::os::raw::c_int {
    let mut hdr: ArchiveHeader = ArchiveHeader {
        ar_name: [0; 16],
        ar_date: [0; 12],
        ar_uid: [0; 6],
        ar_gid: [0; 6],
        ar_mode: [0; 8],
        ar_size: [0; 10],
        ar_fmag: [0; 2],
    };
    /* char magic[8]; */
    let mut size: std::os::raw::c_int = 0;
    let mut len: std::os::raw::c_int = 0;
    let mut file_offset: std::os::raw::c_ulong = 0;
    let mut ehdr: Elf64_Ehdr = Elf64_Ehdr {
        e_ident: [0; 16],
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
    };
    /* skip magic which was already checked */
    /* full_read(fd, magic, sizeof(magic)); */
    file_offset = (::std::mem::size_of::<[std::os::raw::c_char; 9]>() as std::os::raw::c_ulong)
        .wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong);
    loop {
        len = read_ar_header(fd, file_offset as std::os::raw::c_int, &mut hdr);
        if len == 0 as std::os::raw::c_int {
            return 0 as std::os::raw::c_int;
        }
        if len < 0 as std::os::raw::c_int {
            tcc_enter_state(s1);
            Some(
                _tcc_error_noabort
                    as unsafe extern "C" fn(_: *const std::os::raw::c_char, _: ...) -> (),
            )
            .expect("non-null function pointer")(
                b"invalid archive\x00" as *const u8 as *const std::os::raw::c_char,
            );
            return -(1 as std::os::raw::c_int);
        }
        file_offset = file_offset.wrapping_add(len as std::os::raw::c_ulong);
        size = strtol(
            hdr.ar_size.as_mut_ptr(),
            0 as *mut *mut std::os::raw::c_char,
            0 as std::os::raw::c_int,
        ) as std::os::raw::c_int;
        /* align to even */
        size = size + 1 as std::os::raw::c_int & !(1 as std::os::raw::c_int);
        if alacarte != 0 {
            /* coff symbol table : we handle it */
            if strcmp(
                hdr.ar_name.as_mut_ptr(),
                b"/\x00" as *const u8 as *const std::os::raw::c_char,
            ) == 0
            {
                return tcc_load_alacarte(s1, fd, size, 4 as std::os::raw::c_int);
            }
            if strcmp(
                hdr.ar_name.as_mut_ptr(),
                b"/SYM64/\x00" as *const u8 as *const std::os::raw::c_char,
            ) == 0
            {
                return tcc_load_alacarte(s1, fd, size, 8 as std::os::raw::c_int);
            }
        } else if tcc_object_type(fd, &mut ehdr) == 1 as std::os::raw::c_int {
            if (*s1).verbose as std::os::raw::c_int == 2 as std::os::raw::c_int {
                printf(
                    b"   -> %s\n\x00" as *const u8 as *const std::os::raw::c_char,
                    hdr.ar_name.as_mut_ptr(),
                );
            }
            if tcc_load_object_file(s1, fd, file_offset) < 0 as std::os::raw::c_int {
                return -(1 as std::os::raw::c_int);
            }
        }
        file_offset = file_offset.wrapping_add(size as std::os::raw::c_ulong)
    }
}
/* Set LV[I] to the global index of sym-version (LIB,VERSION).  Maybe resizes
LV, maybe create a new entry for (LIB,VERSION).  */
unsafe extern "C" fn set_ver_to_ver(
    mut s1: *mut TCCState,
    mut n: *mut std::os::raw::c_int,
    mut lv: *mut *mut std::os::raw::c_int,
    mut i: std::os::raw::c_int,
    mut lib: *mut std::os::raw::c_char,
    mut version: *mut std::os::raw::c_char,
) {
    while i >= *n {
        *lv =
            tcc_realloc(
                *lv as *mut std::os::raw::c_void,
                ((*n + 1 as std::os::raw::c_int) as std::os::raw::c_ulong).wrapping_mul(
                    ::std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong,
                ),
            ) as *mut std::os::raw::c_int;
        let fresh16 = *n;
        *n = *n + 1;
        *(*lv).offset(fresh16 as isize) = -(1 as std::os::raw::c_int)
    }
    if *(*lv).offset(i as isize) == -(1 as std::os::raw::c_int) {
        let mut v: std::os::raw::c_int = 0;
        let mut prev_same_lib: std::os::raw::c_int = -(1 as std::os::raw::c_int);
        v = 0 as std::os::raw::c_int;
        while v < (*s1).nb_sym_versions {
            if !(strcmp((*(*s1).sym_versions.offset(v as isize)).lib, lib) != 0) {
                prev_same_lib = v;
                if strcmp((*(*s1).sym_versions.offset(v as isize)).version, version) == 0 {
                    break;
                }
            }
            v += 1
        }
        if v == (*s1).nb_sym_versions {
            (*s1).sym_versions = tcc_realloc(
                (*s1).sym_versions as *mut std::os::raw::c_void,
                ((v + 1 as std::os::raw::c_int) as std::os::raw::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<sym_version>() as std::os::raw::c_ulong),
            ) as *mut sym_version;
            let ref mut fresh17 = (*(*s1).sym_versions.offset(v as isize)).lib;
            *fresh17 = tcc_strdup(lib);
            let ref mut fresh18 = (*(*s1).sym_versions.offset(v as isize)).version;
            *fresh18 = tcc_strdup(version);
            (*(*s1).sym_versions.offset(v as isize)).out_index = 0 as std::os::raw::c_int;
            (*(*s1).sym_versions.offset(v as isize)).prev_same_lib = prev_same_lib;
            (*s1).nb_sym_versions += 1
        }
        *(*lv).offset(i as isize) = v
    };
}
/* Associates symbol SYM_INDEX (in dynsymtab) with sym-version index
VERNDX.  */
unsafe extern "C" fn set_sym_version(
    mut s1: *mut TCCState,
    mut sym_index: std::os::raw::c_int,
    mut verndx: std::os::raw::c_int,
) {
    if sym_index >= (*s1).nb_sym_to_version {
        let mut newelems: std::os::raw::c_int = if sym_index != 0 {
            (sym_index) * 2 as std::os::raw::c_int
        } else {
            1 as std::os::raw::c_int
        };
        (*s1).sym_to_version =
            tcc_realloc(
                (*s1).sym_to_version as *mut std::os::raw::c_void,
                (newelems as std::os::raw::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong
                    ),
            ) as *mut std::os::raw::c_int;
        memset((*s1).sym_to_version.offset((*s1).nb_sym_to_version as isize)
                   as *mut std::os::raw::c_void, -(1 as std::os::raw::c_int),
               ((newelems - (*s1).nb_sym_to_version) as
                    std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<std::os::raw::c_int>()
                                                    as std::os::raw::c_ulong));
        (*s1).nb_sym_to_version = newelems
    }
    if *(*s1).sym_to_version.offset(sym_index as isize) < 0 as std::os::raw::c_int {
        *(*s1).sym_to_version.offset(sym_index as isize) = verndx
    };
}
unsafe extern "C" fn store_version(
    mut s1: *mut TCCState,
    mut v: *mut versym_info,
    mut dynstr: *mut std::os::raw::c_char,
) {
    let mut lib: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut version: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut next: uint32_t = 0;
    let mut i: std::os::raw::c_int = 0;
    if !(*v).versym.is_null() && !(*v).verdef.is_null() {
        let mut vdef: *mut Elf64_Verdef = (*v).verdef;
        lib = 0 as *mut std::os::raw::c_char;
        loop {
            let mut verdaux: *mut Elf64_Verdaux = (vdef as *mut std::os::raw::c_char)
                .offset((*vdef).vd_aux as isize)
                as *mut Elf64_Verdaux;
            if (*vdef).vd_cnt != 0 {
                version = dynstr.offset((*verdaux).vda_name as isize);
                if lib.is_null() {
                    lib = version
                } else {
                    set_ver_to_ver(
                        s1,
                        &mut (*v).nb_local_ver,
                        &mut (*v).local_ver,
                        (*vdef).vd_ndx as std::os::raw::c_int,
                        lib,
                        version,
                    );
                }
            }
            next = (*vdef).vd_next;
            vdef = (vdef as *mut std::os::raw::c_char).offset(next as isize) as *mut Elf64_Verdef;
            if !(next != 0) {
                break;
            }
        }
    }
    if !(*v).versym.is_null() && !(*v).verneed.is_null() {
        let mut vneed: *mut Elf64_Verneed = (*v).verneed;
        loop {
            let mut vernaux: *mut Elf64_Vernaux = (vneed as *mut std::os::raw::c_char)
                .offset((*vneed).vn_aux as isize)
                as *mut Elf64_Vernaux;
            lib = dynstr.offset((*vneed).vn_file as isize);
            i = 0 as std::os::raw::c_int;
            while i < (*vneed).vn_cnt as std::os::raw::c_int {
                if (*vernaux).vna_other as std::os::raw::c_int & 0x8000 as std::os::raw::c_int
                    == 0 as std::os::raw::c_int
                {
                    /* hidden */
                    version = dynstr.offset((*vernaux).vna_name as isize);
                    set_ver_to_ver(
                        s1,
                        &mut (*v).nb_local_ver,
                        &mut (*v).local_ver,
                        (*vernaux).vna_other as std::os::raw::c_int,
                        lib,
                        version,
                    );
                }
                vernaux = (vernaux as *mut std::os::raw::c_char)
                    .offset((*vernaux).vna_next as isize)
                    as *mut Elf64_Vernaux;
                i += 1
            }
            next = (*vneed).vn_next;
            vneed =
                (vneed as *mut std::os::raw::c_char).offset(next as isize) as *mut Elf64_Verneed;
            if !(next != 0) {
                break;
            }
        }
    };
}
/* load a DLL and all referenced DLLs. 'level = 0' means that the DLL
is referenced by the user (so it should be added as DT_NEEDED in
the generated ELF file) */
#[no_mangle]
pub unsafe extern "C" fn tcc_load_dll(
    mut s1: *mut TCCState,
    mut fd: std::os::raw::c_int,
    mut filename: *const std::os::raw::c_char,
    mut level: std::os::raw::c_int,
) -> std::os::raw::c_int {
    let mut current_block: u64;
    let mut ehdr: Elf64_Ehdr = Elf64_Ehdr {
        e_ident: [0; 16],
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
    };
    let mut shdr: *mut Elf64_Shdr = 0 as *mut Elf64_Shdr;
    let mut sh: *mut Elf64_Shdr = 0 as *mut Elf64_Shdr;
    let mut sh1: *mut Elf64_Shdr = 0 as *mut Elf64_Shdr;
    let mut i: std::os::raw::c_int = 0;
    let mut j: std::os::raw::c_int = 0;
    let mut nb_syms: std::os::raw::c_int = 0;
    let mut nb_dts: std::os::raw::c_int = 0;
    let mut sym_bind: std::os::raw::c_int = 0;
    let mut ret: std::os::raw::c_int = 0;
    let mut sym: *mut Elf64_Sym = 0 as *mut Elf64_Sym;
    let mut dynsym: *mut Elf64_Sym = 0 as *mut Elf64_Sym;
    let mut dt: *mut Elf64_Dyn = 0 as *mut Elf64_Dyn;
    let mut dynamic: *mut Elf64_Dyn = 0 as *mut Elf64_Dyn;
    let mut dynstr: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut sym_index: std::os::raw::c_int = 0;
    let mut name: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut soname: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut dllref: *mut DLLReference = 0 as *mut DLLReference;
    let mut v: versym_info = versym_info {
        nb_versyms: 0,
        verdef: 0 as *mut Elf64_Verdef,
        verneed: 0 as *mut Elf64_Verneed,
        versym: 0 as *mut Elf64_Half,
        nb_local_ver: 0,
        local_ver: 0 as *mut std::os::raw::c_int,
    };
    full_read(
        fd,
        &mut ehdr as *mut Elf64_Ehdr as *mut std::os::raw::c_void,
        ::std::mem::size_of::<Elf64_Ehdr>() as std::os::raw::c_ulong,
    );
    /* test CPU specific stuff */
    if ehdr.e_ident[5 as std::os::raw::c_int as usize] as std::os::raw::c_int
        != 1 as std::os::raw::c_int
        || ehdr.e_machine as std::os::raw::c_int != 62 as std::os::raw::c_int
    {
        tcc_enter_state(s1);
        Some(
            _tcc_error_noabort
                as unsafe extern "C" fn(_: *const std::os::raw::c_char, _: ...) -> (),
        )
        .expect("non-null function pointer")(
            b"bad architecture\x00" as *const u8 as *const std::os::raw::c_char,
        );
        return -(1 as std::os::raw::c_int);
    }
    /* read sections */
    shdr = load_data(
        fd,
        ehdr.e_shoff,
        (::std::mem::size_of::<Elf64_Shdr>() as std::os::raw::c_ulong)
            .wrapping_mul(ehdr.e_shnum as std::os::raw::c_ulong),
    ) as *mut Elf64_Shdr;
    /* load dynamic section and dynamic symbols */
    nb_syms = 0 as std::os::raw::c_int; /* avoid warning */
    nb_dts = 0 as std::os::raw::c_int; /* avoid warning */
    dynamic = 0 as *mut Elf64_Dyn;
    dynsym = 0 as *mut Elf64_Sym;
    dynstr = 0 as *mut std::os::raw::c_char;
    memset(
        &mut v as *mut versym_info as *mut std::os::raw::c_void,
        0 as std::os::raw::c_int,
        ::std::mem::size_of::<versym_info>() as std::os::raw::c_ulong,
    );
    i = 0 as std::os::raw::c_int;
    sh = shdr;
    while i < ehdr.e_shnum as std::os::raw::c_int {
        match (*sh).sh_type {
            6 => {
                nb_dts = (*sh)
                    .sh_size
                    .wrapping_div(::std::mem::size_of::<Elf64_Dyn>() as std::os::raw::c_ulong)
                    as std::os::raw::c_int;
                dynamic = load_data(fd, (*sh).sh_offset, (*sh).sh_size) as *mut Elf64_Dyn
            },
            11 => {
                nb_syms = (*sh)
                    .sh_size
                    .wrapping_div(::std::mem::size_of::<Elf64_Sym>() as std::os::raw::c_ulong)
                    as std::os::raw::c_int;
                dynsym = load_data(fd, (*sh).sh_offset, (*sh).sh_size) as *mut Elf64_Sym;
                sh1 = &mut *shdr.offset((*sh).sh_link as isize) as *mut Elf64_Shdr;
                dynstr =
                    load_data(fd, (*sh1).sh_offset, (*sh1).sh_size) as *mut std::os::raw::c_char
            },
            1879048189 => {
                v.verdef = load_data(fd, (*sh).sh_offset, (*sh).sh_size) as *mut Elf64_Verdef
            },
            1879048190 => {
                v.verneed = load_data(fd, (*sh).sh_offset, (*sh).sh_size) as *mut Elf64_Verneed
            },
            1879048191 => {
                v.nb_versyms = (*sh)
                    .sh_size
                    .wrapping_div(::std::mem::size_of::<Elf64_Half>() as std::os::raw::c_ulong)
                    as std::os::raw::c_int;
                v.versym = load_data(fd, (*sh).sh_offset, (*sh).sh_size) as *mut Elf64_Half
            },
            _ => {},
        }
        i += 1;
        sh = sh.offset(1)
    }
    /* compute the real library name */
    soname = tcc_basename(filename);
    i = 0 as std::os::raw::c_int;
    dt = dynamic;
    while i < nb_dts {
        if (*dt).d_tag == 14 as std::os::raw::c_int as std::os::raw::c_long {
            soname = dynstr.offset((*dt).d_un.d_val as isize)
        }
        i += 1;
        dt = dt.offset(1)
    }
    /* if the dll is already loaded, do not load it */
    i = 0 as std::os::raw::c_int;
    loop {
        if !(i < (*s1).nb_loaded_dlls) {
            current_block = 10150597327160359210;
            break;
        }
        dllref = *(*s1).loaded_dlls.offset(i as isize);
        if strcmp(soname, (*dllref).name.as_mut_ptr()) == 0 {
            /* but update level if needed */
            if level < (*dllref).level {
                (*dllref).level = level
            }
            ret = 0 as std::os::raw::c_int;
            current_block = 12401344399603912730;
            break;
        } else {
            i += 1
        }
    }
    match current_block {
        10150597327160359210 => {
            if v.nb_versyms != nb_syms {
                tcc_free(v.versym as *mut std::os::raw::c_void);
                v.versym = 0 as *mut Elf64_Half
            } else {
                store_version(s1, &mut v, dynstr);
            }
            /* add the dll and its level */
            (*tcc_add_dllref(s1, soname)).level = level;
            /* add dynamic symbols in dynsym_section */
            i = 1 as std::os::raw::c_int;
            sym = dynsym.offset(1 as std::os::raw::c_int as isize);
            while i < nb_syms {
                sym_bind = (*sym).st_info as std::os::raw::c_int >> 4 as std::os::raw::c_int;
                if !(sym_bind == 0 as std::os::raw::c_int) {
                    name = dynstr.offset((*sym).st_name as isize);
                    sym_index = set_elf_sym(
                        (*s1).dynsymtab_section,
                        (*sym).st_value,
                        (*sym).st_size,
                        (*sym).st_info as std::os::raw::c_int,
                        (*sym).st_other as std::os::raw::c_int,
                        (*sym).st_shndx as std::os::raw::c_int,
                        name,
                    );
                    if !v.versym.is_null() {
                        let mut vsym: Elf64_Half = *v.versym.offset(i as isize);
                        if vsym as std::os::raw::c_int & 0x8000 as std::os::raw::c_int
                            == 0 as std::os::raw::c_int
                            && vsym as std::os::raw::c_int > 0 as std::os::raw::c_int
                            && (vsym as std::os::raw::c_int) < v.nb_local_ver
                        {
                            set_sym_version(s1, sym_index, *v.local_ver.offset(vsym as isize));
                        }
                    }
                }
                i += 1;
                sym = sym.offset(1)
            }
            /* load all referenced DLLs */
            i = 0 as std::os::raw::c_int;
            dt = dynamic;
            loop {
                if !(i < nb_dts) {
                    current_block = 11739054925370445424;
                    break;
                }
                match (*dt).d_tag {
                    1 => {
                        name = dynstr.offset((*dt).d_un.d_val as isize);
                        j = 0 as std::os::raw::c_int;
                        loop {
                            if !(j < (*s1).nb_loaded_dlls) {
                                current_block = 10512632378975961025;
                                break;
                            }
                            dllref = *(*s1).loaded_dlls.offset(j as isize);
                            if strcmp(name, (*dllref).name.as_mut_ptr()) == 0 {
                                current_block = 15594603006322722090;
                                break;
                            }
                            j += 1
                        }
                        match current_block {
                            15594603006322722090 => {},
                            _ => {
                                if tcc_add_dll(s1, name, 0x20 as std::os::raw::c_int)
                                    < 0 as std::os::raw::c_int
                                {
                                    tcc_enter_state(s1);
                                    Some(
                                        _tcc_error_noabort
                                            as unsafe extern "C" fn(
                                                _: *const std::os::raw::c_char,
                                                _: ...
                                            )
                                                -> (),
                                    )
                                    .expect("non-null function pointer")(
                                        b"referenced dll \'%s\' not found\x00" as *const u8
                                            as *const std::os::raw::c_char,
                                        name,
                                    );
                                    ret = -(1 as std::os::raw::c_int);
                                    current_block = 12401344399603912730;
                                    break;
                                }
                            },
                        }
                    },
                    _ => {},
                }
                i += 1;
                dt = dt.offset(1)
            }
            match current_block {
                12401344399603912730 => {},
                _ => ret = 0 as std::os::raw::c_int,
            }
        },
        _ => {},
    }
    tcc_free(dynstr as *mut std::os::raw::c_void);
    tcc_free(dynsym as *mut std::os::raw::c_void);
    tcc_free(dynamic as *mut std::os::raw::c_void);
    tcc_free(shdr as *mut std::os::raw::c_void);
    tcc_free(v.local_ver as *mut std::os::raw::c_void);
    tcc_free(v.verdef as *mut std::os::raw::c_void);
    tcc_free(v.verneed as *mut std::os::raw::c_void);
    tcc_free(v.versym as *mut std::os::raw::c_void);
    return ret;
}
unsafe extern "C" fn ld_inp(mut s1: *mut TCCState) -> std::os::raw::c_int {
    let mut b: std::os::raw::c_char = 0;
    if (*s1).cc != -(1 as std::os::raw::c_int) {
        let mut c: std::os::raw::c_int = (*s1).cc;
        (*s1).cc = -(1 as std::os::raw::c_int);
        return c;
    }
    if 1 as std::os::raw::c_int as std::os::raw::c_long
        == read(
            (*s1).fd,
            &mut b as *mut std::os::raw::c_char as *mut std::os::raw::c_void,
            1 as std::os::raw::c_int as size_t,
        )
    {
        return b as std::os::raw::c_int;
    }
    return -(1 as std::os::raw::c_int);
}
/* return next ld script token */
unsafe extern "C" fn ld_next(
    mut s1: *mut TCCState,
    mut name: *mut std::os::raw::c_char,
    mut name_size: std::os::raw::c_int,
) -> std::os::raw::c_int {
    let mut current_block: u64;
    let mut c: std::os::raw::c_int = 0;
    let mut d: std::os::raw::c_int = 0;
    let mut ch: std::os::raw::c_int = 0;
    let mut q: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    loop {
        ch = ld_inp(s1);
        match ch {
            32 | 9 | 12 | 11 | 13 | 10 => {},
            47 => {
                ch = ld_inp(s1);
                if ch == '*' as i32 {
                    /* comment */
                    d = 0 as std::os::raw::c_int;
                    loop {
                        ch = ld_inp(s1);
                        if ch == -(1 as std::os::raw::c_int) || ch == '/' as i32 && d == '*' as i32
                        {
                            break;
                        }
                        d = ch
                    }
                } else {
                    q = name;
                    let fresh19 = q;
                    q = q.offset(1);
                    *fresh19 = '/' as i32 as std::os::raw::c_char;
                    current_block = 10881366358344556924;
                    break;
                }
            },
            92 => {
                /* case 'a' ... 'z': */
                current_block = 1476413545099485561;
                break;
            },
            97 | 98 | 99 | 100 | 101 | 102 | 103 | 104 | 105 | 106 | 107 | 108 | 109 | 110
            | 111 | 112 | 113 | 114 | 115 | 116 | 117 | 118 | 119 | 120 | 121 | 122 | 65 | 66
            | 67 | 68 | 69 | 70 | 71 | 72 | 73 | 74 | 75 | 76 | 77 | 78 | 79 | 80 | 81 | 82
            | 83 | 84 | 85 | 86 | 87 | 88 | 89 | 90 | 95 | 46 | 36 | 126 => {
                current_block = 1476413545099485561;
                break;
            },
            -1 => {
                c = -(1 as std::os::raw::c_int);
                current_block = 8151474771948790331;
                break;
            },
            _ => {
                c = ch;
                current_block = 8151474771948790331;
                break;
            },
        }
    }
    match current_block {
        1476413545099485561 =>
        /* case 'A' ... 'z': */
        {
            q = name;
            current_block = 10881366358344556924;
        }
        _ => {},
    }
    match current_block {
        10881366358344556924 => {
            while ch >= 'a' as i32 && ch <= 'z' as i32
                || ch >= 'A' as i32 && ch <= 'Z' as i32
                || ch >= '0' as i32 && ch <= '9' as i32
                || !strchr(
                    b"/.-_+=$:\\,~\x00" as *const u8 as *const std::os::raw::c_char,
                    ch,
                )
                .is_null()
            {
                if (q.offset_from(name) as std::os::raw::c_long)
                    < (name_size - 1 as std::os::raw::c_int) as std::os::raw::c_long
                {
                    let fresh20 = q;
                    q = q.offset(1);
                    *fresh20 = ch as std::os::raw::c_char
                }
                ch = ld_inp(s1)
            }
            (*s1).cc = ch;
            *q = '\u{0}' as i32 as std::os::raw::c_char;
            c = 256 as std::os::raw::c_int
        },
        _ => {},
    }
    return c;
}
unsafe extern "C" fn ld_add_file(
    mut s1: *mut TCCState,
    mut filename: *const std::os::raw::c_char,
) -> std::os::raw::c_int {
    if *filename.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int == '/' as i32 {
        if (*::std::mem::transmute::<&[u8; 1], &[std::os::raw::c_char; 1]>(b"\x00"))
            [0 as std::os::raw::c_int as usize] as std::os::raw::c_int
            == '\u{0}' as i32
            && tcc_add_file_internal(s1, filename, 0x40 as std::os::raw::c_int)
                == 0 as std::os::raw::c_int
        {
            return 0 as std::os::raw::c_int;
        }
        filename = tcc_basename(filename) as *const std::os::raw::c_char
    }
    return tcc_add_dll(s1, filename, 0 as std::os::raw::c_int);
}
unsafe extern "C" fn ld_add_file_list(
    mut s1: *mut TCCState,
    mut cmd: *const std::os::raw::c_char,
    mut as_needed: std::os::raw::c_int,
) -> std::os::raw::c_int {
    let mut current_block: u64;
    let mut filename: [std::os::raw::c_char; 1024] = [0; 1024];
    let mut libname: [std::os::raw::c_char; 1024] = [0; 1024];
    let mut t: std::os::raw::c_int = 0;
    let mut group: std::os::raw::c_int = 0;
    let mut nblibs: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut ret: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut libs: *mut *mut std::os::raw::c_char = 0 as *mut *mut std::os::raw::c_char;
    group = (strcmp(
        cmd,
        b"GROUP\x00" as *const u8 as *const std::os::raw::c_char,
    ) == 0) as std::os::raw::c_int;
    if as_needed == 0 {
        (*s1).new_undef_sym = 0 as std::os::raw::c_int
    }
    t = ld_next(
        s1,
        filename.as_mut_ptr(),
        ::std::mem::size_of::<[std::os::raw::c_char; 1024]>() as std::os::raw::c_ulong
            as std::os::raw::c_int,
    );
    if t != '(' as i32 {
        tcc_enter_state(s1);
        Some(
            _tcc_error_noabort
                as unsafe extern "C" fn(_: *const std::os::raw::c_char, _: ...) -> (),
        )
        .expect("non-null function pointer")(
            b"( expected\x00" as *const u8 as *const std::os::raw::c_char,
        );
        ret = -(1 as std::os::raw::c_int)
    } else {
        t = ld_next(
            s1,
            filename.as_mut_ptr(),
            ::std::mem::size_of::<[std::os::raw::c_char; 1024]>() as std::os::raw::c_ulong
                as std::os::raw::c_int,
        );
        loop {
            libname[0 as std::os::raw::c_int as usize] = '\u{0}' as i32 as std::os::raw::c_char;
            if t == -(1 as std::os::raw::c_int) {
                tcc_enter_state(s1);
                Some(
                    _tcc_error_noabort
                        as unsafe extern "C" fn(_: *const std::os::raw::c_char, _: ...) -> (),
                )
                .expect("non-null function pointer")(
                    b"unexpected end of file\x00" as *const u8 as *const std::os::raw::c_char,
                );
                ret = -(1 as std::os::raw::c_int);
                current_block = 394432901522681494;
                break;
            } else {
                if t == ')' as i32 {
                    current_block = 11763295167351361500;
                    break;
                }
                if t == '-' as i32 {
                    t = ld_next(
                        s1,
                        filename.as_mut_ptr(),
                        ::std::mem::size_of::<[std::os::raw::c_char; 1024]>()
                            as std::os::raw::c_ulong as std::os::raw::c_int,
                    );
                    if t != 256 as std::os::raw::c_int
                        || filename[0 as std::os::raw::c_int as usize] as std::os::raw::c_int
                            != 'l' as i32
                    {
                        tcc_enter_state(s1);
                        Some(
                            _tcc_error_noabort
                                as unsafe extern "C" fn(
                                    _: *const std::os::raw::c_char,
                                    _: ...
                                ) -> (),
                        )
                        .expect("non-null function pointer")(
                            b"library name expected\x00" as *const u8
                                as *const std::os::raw::c_char,
                        );
                        ret = -(1 as std::os::raw::c_int);
                        current_block = 394432901522681494;
                        break;
                    } else {
                        pstrcpy(
                            libname.as_mut_ptr(),
                            ::std::mem::size_of::<[std::os::raw::c_char; 1024]>()
                                as std::os::raw::c_ulong,
                            &mut *filename
                                .as_mut_ptr()
                                .offset(1 as std::os::raw::c_int as isize),
                        );
                        if (*s1).static_link != 0 {
                            snprintf(
                                filename.as_mut_ptr(),
                                ::std::mem::size_of::<[std::os::raw::c_char; 1024]>()
                                    as std::os::raw::c_ulong,
                                b"lib%s.a\x00" as *const u8 as *const std::os::raw::c_char,
                                libname.as_mut_ptr(),
                            );
                        } else {
                            snprintf(
                                filename.as_mut_ptr(),
                                ::std::mem::size_of::<[std::os::raw::c_char; 1024]>()
                                    as std::os::raw::c_ulong,
                                b"lib%s.so\x00" as *const u8 as *const std::os::raw::c_char,
                                libname.as_mut_ptr(),
                            );
                        }
                    }
                } else if t != 256 as std::os::raw::c_int {
                    tcc_enter_state(s1);
                    Some(
                        _tcc_error_noabort
                            as unsafe extern "C" fn(_: *const std::os::raw::c_char, _: ...) -> (),
                    )
                    .expect("non-null function pointer")(
                        b"filename expected\x00" as *const u8 as *const std::os::raw::c_char,
                    );
                    ret = -(1 as std::os::raw::c_int);
                    current_block = 394432901522681494;
                    break;
                }
                if strcmp(
                    filename.as_mut_ptr(),
                    b"AS_NEEDED\x00" as *const u8 as *const std::os::raw::c_char,
                ) == 0
                {
                    ret = ld_add_file_list(s1, cmd, 1 as std::os::raw::c_int);
                    if ret != 0 {
                        current_block = 394432901522681494;
                        break;
                    }
                } else if as_needed == 0 {
                    ret = ld_add_file(s1, filename.as_mut_ptr() as *const std::os::raw::c_char);
                    if ret != 0 {
                        current_block = 394432901522681494;
                        break;
                    }
                    if group != 0 {
                        /* TODO: Implement AS_NEEDED support. Ignore it for now */
                        /* Add the filename *and* the libname to avoid future conversions */
                        dynarray_add(
                            &mut libs as *mut *mut *mut std::os::raw::c_char
                                as *mut std::os::raw::c_void,
                            &mut nblibs,
                            tcc_strdup(filename.as_mut_ptr()) as *mut std::os::raw::c_void,
                        );
                        if libname[0 as std::os::raw::c_int as usize] as std::os::raw::c_int
                            != '\u{0}' as i32
                        {
                            dynarray_add(
                                &mut libs as *mut *mut *mut std::os::raw::c_char
                                    as *mut std::os::raw::c_void,
                                &mut nblibs,
                                tcc_strdup(libname.as_mut_ptr()) as *mut std::os::raw::c_void,
                            );
                        }
                    }
                }
                t = ld_next(
                    s1,
                    filename.as_mut_ptr(),
                    ::std::mem::size_of::<[std::os::raw::c_char; 1024]>() as std::os::raw::c_ulong
                        as std::os::raw::c_int,
                );
                if t == ',' as i32 {
                    t = ld_next(
                        s1,
                        filename.as_mut_ptr(),
                        ::std::mem::size_of::<[std::os::raw::c_char; 1024]>()
                            as std::os::raw::c_ulong as std::os::raw::c_int,
                    )
                }
            }
        }
        match current_block {
            394432901522681494 => {},
            _ => {
                if group != 0 && as_needed == 0 {
                    while (*s1).new_undef_sym != 0 {
                        let mut i: std::os::raw::c_int = 0;
                        (*s1).new_undef_sym = 0 as std::os::raw::c_int;
                        i = 0 as std::os::raw::c_int;
                        while i < nblibs {
                            ld_add_file(
                                s1,
                                *libs.offset(i as isize) as *const std::os::raw::c_char,
                            );
                            i += 1
                        }
                    }
                }
            },
        }
    }
    dynarray_reset(
        &mut libs as *mut *mut *mut std::os::raw::c_char as *mut std::os::raw::c_void,
        &mut nblibs,
    );
    return ret;
}
/* interpret a subset of GNU ldscripts to handle the dummy libc.so
files */
#[no_mangle]
pub unsafe extern "C" fn tcc_load_ldscript(
    mut s1: *mut TCCState,
    mut fd: std::os::raw::c_int,
) -> std::os::raw::c_int {
    let mut cmd: [std::os::raw::c_char; 64] = [0; 64];
    let mut filename: [std::os::raw::c_char; 1024] = [0; 1024];
    let mut t: std::os::raw::c_int = 0;
    let mut ret: std::os::raw::c_int = 0;
    (*s1).fd = fd;
    (*s1).cc = -(1 as std::os::raw::c_int);
    loop {
        t = ld_next(
            s1,
            cmd.as_mut_ptr(),
            ::std::mem::size_of::<[std::os::raw::c_char; 64]>() as std::os::raw::c_ulong
                as std::os::raw::c_int,
        );
        if t == -(1 as std::os::raw::c_int) {
            return 0 as std::os::raw::c_int;
        } else {
            if t != 256 as std::os::raw::c_int {
                return -(1 as std::os::raw::c_int);
            }
        }
        if strcmp(
            cmd.as_mut_ptr(),
            b"INPUT\x00" as *const u8 as *const std::os::raw::c_char,
        ) == 0
            || strcmp(
                cmd.as_mut_ptr(),
                b"GROUP\x00" as *const u8 as *const std::os::raw::c_char,
            ) == 0
        {
            ret = ld_add_file_list(s1, cmd.as_mut_ptr(), 0 as std::os::raw::c_int);
            if ret != 0 {
                return ret;
            }
        } else if strcmp(
            cmd.as_mut_ptr(),
            b"OUTPUT_FORMAT\x00" as *const u8 as *const std::os::raw::c_char,
        ) == 0
            || strcmp(
                cmd.as_mut_ptr(),
                b"TARGET\x00" as *const u8 as *const std::os::raw::c_char,
            ) == 0
        {
            /* ignore some commands */
            t = ld_next(
                s1,
                cmd.as_mut_ptr(),
                ::std::mem::size_of::<[std::os::raw::c_char; 64]>() as std::os::raw::c_ulong
                    as std::os::raw::c_int,
            );
            if t != '(' as i32 {
                tcc_enter_state(s1);
                Some(
                    _tcc_error_noabort
                        as unsafe extern "C" fn(_: *const std::os::raw::c_char, _: ...) -> (),
                )
                .expect("non-null function pointer")(
                    b"( expected\x00" as *const u8 as *const std::os::raw::c_char,
                );
                return -(1 as std::os::raw::c_int);
            }
            loop {
                t = ld_next(
                    s1,
                    filename.as_mut_ptr(),
                    ::std::mem::size_of::<[std::os::raw::c_char; 1024]>() as std::os::raw::c_ulong
                        as std::os::raw::c_int,
                );
                if t == -(1 as std::os::raw::c_int) {
                    tcc_enter_state(s1);
                    Some(
                        _tcc_error_noabort
                            as unsafe extern "C" fn(_: *const std::os::raw::c_char, _: ...) -> (),
                    )
                    .expect("non-null function pointer")(
                        b"unexpected end of file\x00" as *const u8 as *const std::os::raw::c_char,
                    );
                    return -(1 as std::os::raw::c_int);
                } else if t == ')' as i32 {
                    break;
                }
            }
        } else {
            return -(1 as std::os::raw::c_int);
        }
    }
}
/* !ELF_OBJ_ONLY */
