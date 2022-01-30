use crate::bitfields::*;
use ::f128;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type sym_version;
    #[no_mangle]
    fn ldexp(_: std::os::raw::c_double, _: std::os::raw::c_int) -> std::os::raw::c_double;
    #[no_mangle]
    fn __errno_location() -> *mut std::os::raw::c_int;
    #[no_mangle]
    fn strlen(_: *const std::os::raw::c_char) -> std::os::raw::c_ulong;
    #[no_mangle]
    fn strchr(_: *const std::os::raw::c_char, _: std::os::raw::c_int) -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn strcmp(
        _: *const std::os::raw::c_char,
        _: *const std::os::raw::c_char,
    ) -> std::os::raw::c_int;
    #[no_mangle]
    fn strcpy(
        _: *mut std::os::raw::c_char,
        _: *const std::os::raw::c_char,
    ) -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn memcmp(
        _: *const std::os::raw::c_void,
        _: *const std::os::raw::c_void,
        _: std::os::raw::c_ulong,
    ) -> std::os::raw::c_int;
    #[no_mangle]
    fn memset(
        _: *mut std::os::raw::c_void,
        _: std::os::raw::c_int,
        _: std::os::raw::c_ulong,
    ) -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn memmove(
        _: *mut std::os::raw::c_void,
        _: *const std::os::raw::c_void,
        _: std::os::raw::c_ulong,
    ) -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn memcpy(
        _: *mut std::os::raw::c_void,
        _: *const std::os::raw::c_void,
        _: std::os::raw::c_ulong,
    ) -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn fputs(__s: *const std::os::raw::c_char, __stream: *mut FILE) -> std::os::raw::c_int;
    #[no_mangle]
    fn sscanf(
        _: *const std::os::raw::c_char,
        _: *const std::os::raw::c_char,
        _: ...
    ) -> std::os::raw::c_int;
    #[no_mangle]
    fn vsnprintf(
        _: *mut std::os::raw::c_char,
        _: std::os::raw::c_ulong,
        _: *const std::os::raw::c_char,
        _: ::std::ffi::VaList,
    ) -> std::os::raw::c_int;
    #[no_mangle]
    fn snprintf(
        _: *mut std::os::raw::c_char,
        _: std::os::raw::c_ulong,
        _: *const std::os::raw::c_char,
        _: ...
    ) -> std::os::raw::c_int;
    #[no_mangle]
    fn sprintf(
        _: *mut std::os::raw::c_char,
        _: *const std::os::raw::c_char,
        _: ...
    ) -> std::os::raw::c_int;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    fn fflush(__stream: *mut FILE) -> std::os::raw::c_int;
    #[no_mangle]
    fn strtoul(
        _: *const std::os::raw::c_char,
        _: *mut *mut std::os::raw::c_char,
        _: std::os::raw::c_int,
    ) -> std::os::raw::c_ulong;
    #[no_mangle]
    fn strtold(_: *const std::os::raw::c_char, _: *mut *mut std::os::raw::c_char) -> f128::f128;
    #[no_mangle]
    fn strtof(
        _: *const std::os::raw::c_char,
        _: *mut *mut std::os::raw::c_char,
    ) -> std::os::raw::c_float;
    #[no_mangle]
    fn strtod(
        _: *const std::os::raw::c_char,
        _: *mut *mut std::os::raw::c_char,
    ) -> std::os::raw::c_double;
    #[no_mangle]
    fn time(__timer: *mut time_t) -> time_t;
    #[no_mangle]
    fn localtime(__timer: *const time_t) -> *mut tm;
    #[no_mangle]
    fn read(
        __fd: std::os::raw::c_int,
        __buf: *mut std::os::raw::c_void,
        __nbytes: size_t,
    ) -> ssize_t;
    #[no_mangle]
    fn tcc_set_options(s: *mut TCCState, str: *const std::os::raw::c_char);
    #[no_mangle]
    static mut tcc_state: *mut TCCState;
    #[no_mangle]
    fn pstrcpy(
        buf: *mut std::os::raw::c_char,
        buf_size: size_t,
        s: *const std::os::raw::c_char,
    ) -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn pstrcat(
        buf: *mut std::os::raw::c_char,
        buf_size: size_t,
        s: *const std::os::raw::c_char,
    ) -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn pstrncpy(
        out: *mut std::os::raw::c_char,
        in_0: *const std::os::raw::c_char,
        num: size_t,
    ) -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn tcc_basename(name: *const std::os::raw::c_char) -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn tcc_free(ptr: *mut std::os::raw::c_void);
    #[no_mangle]
    fn tcc_malloc(size: std::os::raw::c_ulong) -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn tcc_mallocz(size: std::os::raw::c_ulong) -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn tcc_realloc(
        ptr: *mut std::os::raw::c_void,
        size: std::os::raw::c_ulong,
    ) -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn tcc_strdup(str: *const std::os::raw::c_char) -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn _tcc_error(fmt: *const std::os::raw::c_char, _: ...) -> !;
    #[no_mangle]
    fn _tcc_warning(fmt: *const std::os::raw::c_char, _: ...);
    #[no_mangle]
    fn dynarray_add(
        ptab: *mut std::os::raw::c_void,
        nb_ptr: *mut std::os::raw::c_int,
        data: *mut std::os::raw::c_void,
    );
    #[no_mangle]
    fn dynarray_reset(pp: *mut std::os::raw::c_void, n: *mut std::os::raw::c_int);
    #[no_mangle]
    fn tcc_open_bf(
        s1: *mut TCCState,
        filename: *const std::os::raw::c_char,
        initlen: std::os::raw::c_int,
    );
    #[no_mangle]
    fn tcc_open(s1: *mut TCCState, filename: *const std::os::raw::c_char) -> std::os::raw::c_int;
    #[no_mangle]
    fn tcc_close();
    #[no_mangle]
    static mut define_stack: *mut Sym;
    #[no_mangle]
    fn sym_push2(
        ps: *mut *mut Sym,
        v: std::os::raw::c_int,
        t: std::os::raw::c_int,
        c: std::os::raw::c_int,
    ) -> *mut Sym;
    #[no_mangle]
    fn sym_free(sym: *mut Sym);
    #[no_mangle]
    static mut global_label_stack: *mut Sym;
    #[no_mangle]
    fn put_extern_sym(
        sym: *mut Sym,
        section: *mut Section,
        value: Elf64_Addr,
        size: std::os::raw::c_ulong,
    );
    #[no_mangle]
    fn sym_find2(s: *mut Sym, v: std::os::raw::c_int) -> *mut Sym;
    #[no_mangle]
    fn tcc_debug_putfile(s1: *mut TCCState, filename: *const std::os::raw::c_char);
    #[no_mangle]
    fn expr_const() -> std::os::raw::c_int;
    #[no_mangle]
    fn tcc_debug_bincl(s1: *mut TCCState);
    #[no_mangle]
    fn tcc_debug_eincl(s1: *mut TCCState);
    #[no_mangle]
    static target_machine_defs: *const std::os::raw::c_char;
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
pub type __uint32_t = std::os::raw::c_uint;
pub type __int64_t = std::os::raw::c_long;
pub type __uint64_t = std::os::raw::c_ulong;
pub type __off_t = std::os::raw::c_long;
pub type __off64_t = std::os::raw::c_long;
pub type __time_t = std::os::raw::c_long;
pub type __ssize_t = std::os::raw::c_long;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: std::os::raw::c_int,
    pub tm_min: std::os::raw::c_int,
    pub tm_hour: std::os::raw::c_int,
    pub tm_mday: std::os::raw::c_int,
    pub tm_mon: std::os::raw::c_int,
    pub tm_year: std::os::raw::c_int,
    pub tm_wday: std::os::raw::c_int,
    pub tm_yday: std::os::raw::c_int,
    pub tm_isdst: std::os::raw::c_int,
    pub tm_gmtoff: std::os::raw::c_long,
    pub tm_zone: *const std::os::raw::c_char,
}
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
    pub Pflag: C2RustUnnamed_3,
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
    pub c2rust_unnamed: C2RustUnnamed_0,
    pub type_0: CType,
    pub c2rust_unnamed_0: C2RustUnnamed,
    pub prev: *mut Sym,
    pub prev_tok: *mut Sym,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
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
pub union C2RustUnnamed_0 {
    pub c2rust_unnamed: C2RustUnnamed_1,
    pub enum_val: std::os::raw::c_longlong,
    pub d: *mut std::os::raw::c_int,
    pub ncl: *mut Sym,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub c: std::os::raw::c_int,
    pub c2rust_unnamed: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
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
pub type C2RustUnnamed_3 = std::os::raw::c_uint;
pub const LINE_MACRO_OUTPUT_FORMAT_P10: C2RustUnnamed_3 = 11;
pub const LINE_MACRO_OUTPUT_FORMAT_STD: C2RustUnnamed_3 = 2;
pub const LINE_MACRO_OUTPUT_FORMAT_NONE: C2RustUnnamed_3 = 1;
pub const LINE_MACRO_OUTPUT_FORMAT_GCC: C2RustUnnamed_3 = 0;
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
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TokenSym {
    pub hash_next: *mut TokenSym,
    pub sym_define: *mut Sym,
    pub sym_label: *mut Sym,
    pub sym_struct: *mut Sym,
    pub sym_identifier: *mut Sym,
    pub tok: std::os::raw::c_int,
    pub len: std::os::raw::c_int,
    pub str_0: [std::os::raw::c_char; 1],
}
pub type nwchar_t = std::os::raw::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub union CValue {
    pub ld: f128::f128,
    pub d: std::os::raw::c_double,
    pub f: std::os::raw::c_float,
    pub i: uint64_t,
    pub str_0: C2RustUnnamed_4,
    pub tab: [std::os::raw::c_int; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub data: *const std::os::raw::c_void,
    pub size: std::os::raw::c_int,
}
pub type tcc_token = std::os::raw::c_uint;
pub const TOK_ASM_clflush: tcc_token = 1314;
pub const TOK_ASM_sfence: tcc_token = 1313;
pub const TOK_ASM_mfence: tcc_token = 1312;
pub const TOK_ASM_lfence: tcc_token = 1311;
pub const TOK_ASM_prefetchw: tcc_token = 1310;
pub const TOK_ASM_prefetcht2: tcc_token = 1309;
pub const TOK_ASM_prefetcht1: tcc_token = 1308;
pub const TOK_ASM_prefetcht0: tcc_token = 1307;
pub const TOK_ASM_prefetchnta: tcc_token = 1306;
pub const TOK_ASM_subps: tcc_token = 1305;
pub const TOK_ASM_sqrtps: tcc_token = 1304;
pub const TOK_ASM_rsqrtps: tcc_token = 1303;
pub const TOK_ASM_rcpss: tcc_token = 1302;
pub const TOK_ASM_pminub: tcc_token = 1301;
pub const TOK_ASM_pminsw: tcc_token = 1300;
pub const TOK_ASM_pmaxub: tcc_token = 1299;
pub const TOK_ASM_pmaxsw: tcc_token = 1298;
pub const TOK_ASM_pavgw: tcc_token = 1297;
pub const TOK_ASM_pavgb: tcc_token = 1296;
pub const TOK_ASM_mulps: tcc_token = 1295;
pub const TOK_ASM_minps: tcc_token = 1294;
pub const TOK_ASM_maxps: tcc_token = 1293;
pub const TOK_ASM_divps: tcc_token = 1292;
pub const TOK_ASM_cvttps2pi: tcc_token = 1291;
pub const TOK_ASM_cvtps2pi: tcc_token = 1290;
pub const TOK_ASM_cvtpi2ps: tcc_token = 1289;
pub const TOK_ASM_addps: tcc_token = 1288;
pub const TOK_ASM_movhps: tcc_token = 1287;
pub const TOK_ASM_movaps: tcc_token = 1286;
pub const TOK_ASM_movups: tcc_token = 1285;
pub const TOK_ASM_pxor: tcc_token = 1284;
pub const TOK_ASM_punpckldq: tcc_token = 1283;
pub const TOK_ASM_punpcklwd: tcc_token = 1282;
pub const TOK_ASM_punpcklbw: tcc_token = 1281;
pub const TOK_ASM_punpckhdq: tcc_token = 1280;
pub const TOK_ASM_punpckhwd: tcc_token = 1279;
pub const TOK_ASM_punpckhbw: tcc_token = 1278;
pub const TOK_ASM_psubusw: tcc_token = 1277;
pub const TOK_ASM_psubusb: tcc_token = 1276;
pub const TOK_ASM_psubsw: tcc_token = 1275;
pub const TOK_ASM_psubsb: tcc_token = 1274;
pub const TOK_ASM_psubd: tcc_token = 1273;
pub const TOK_ASM_psubw: tcc_token = 1272;
pub const TOK_ASM_psubb: tcc_token = 1271;
pub const TOK_ASM_psrlq: tcc_token = 1270;
pub const TOK_ASM_psrld: tcc_token = 1269;
pub const TOK_ASM_psrlw: tcc_token = 1268;
pub const TOK_ASM_psrad: tcc_token = 1267;
pub const TOK_ASM_psraw: tcc_token = 1266;
pub const TOK_ASM_psllq: tcc_token = 1265;
pub const TOK_ASM_pslld: tcc_token = 1264;
pub const TOK_ASM_psllw: tcc_token = 1263;
pub const TOK_ASM_por: tcc_token = 1262;
pub const TOK_ASM_pmullw: tcc_token = 1261;
pub const TOK_ASM_pmulhw: tcc_token = 1260;
pub const TOK_ASM_pmaddwd: tcc_token = 1259;
pub const TOK_ASM_pcmpgtd: tcc_token = 1258;
pub const TOK_ASM_pcmpgtw: tcc_token = 1257;
pub const TOK_ASM_pcmpgtb: tcc_token = 1256;
pub const TOK_ASM_pcmpeqd: tcc_token = 1255;
pub const TOK_ASM_pcmpeqw: tcc_token = 1254;
pub const TOK_ASM_pcmpeqb: tcc_token = 1253;
pub const TOK_ASM_pandn: tcc_token = 1252;
pub const TOK_ASM_pand: tcc_token = 1251;
pub const TOK_ASM_paddusw: tcc_token = 1250;
pub const TOK_ASM_paddusb: tcc_token = 1249;
pub const TOK_ASM_paddsw: tcc_token = 1248;
pub const TOK_ASM_paddsb: tcc_token = 1247;
pub const TOK_ASM_paddd: tcc_token = 1246;
pub const TOK_ASM_paddw: tcc_token = 1245;
pub const TOK_ASM_paddb: tcc_token = 1244;
pub const TOK_ASM_packuswb: tcc_token = 1243;
pub const TOK_ASM_packsswb: tcc_token = 1242;
pub const TOK_ASM_packssdw: tcc_token = 1241;
pub const TOK_ASM_movd: tcc_token = 1240;
pub const TOK_ASM_fcomip: tcc_token = 1239;
pub const TOK_ASM_fucomip: tcc_token = 1238;
pub const TOK_ASM_fcomi: tcc_token = 1237;
pub const TOK_ASM_fucomi: tcc_token = 1236;
pub const TOK_ASM_fcmovnu: tcc_token = 1235;
pub const TOK_ASM_fcmovnbe: tcc_token = 1234;
pub const TOK_ASM_fcmovne: tcc_token = 1233;
pub const TOK_ASM_fcmovnb: tcc_token = 1232;
pub const TOK_ASM_fcmovu: tcc_token = 1231;
pub const TOK_ASM_fcmovbe: tcc_token = 1230;
pub const TOK_ASM_fcmove: tcc_token = 1229;
pub const TOK_ASM_fcmovb: tcc_token = 1228;
pub const TOK_ASM_cmpxchg16b: tcc_token = 1227;
pub const TOK_ASM_cmpxchg8b: tcc_token = 1226;
pub const TOK_ASM_invlpg: tcc_token = 1225;
pub const TOK_ASM_bswapq: tcc_token = 1224;
pub const TOK_ASM_bswapl: tcc_token = 1223;
pub const TOK_ASM_bswap: tcc_token = 1222;
pub const TOK_ASM_swapgs: tcc_token = 1221;
pub const TOK_ASM_verw: tcc_token = 1220;
pub const TOK_ASM_verr: tcc_token = 1219;
pub const TOK_ASM_str: tcc_token = 1218;
pub const TOK_ASM_smsw: tcc_token = 1217;
pub const TOK_ASM_sldt: tcc_token = 1216;
pub const TOK_ASM_sidtq: tcc_token = 1215;
pub const TOK_ASM_sidt: tcc_token = 1214;
pub const TOK_ASM_sgdtq: tcc_token = 1213;
pub const TOK_ASM_sgdt: tcc_token = 1212;
pub const TOK_ASM_ltr: tcc_token = 1211;
pub const TOK_ASM_lmsw: tcc_token = 1210;
pub const TOK_ASM_lldt: tcc_token = 1209;
pub const TOK_ASM_lidtq: tcc_token = 1208;
pub const TOK_ASM_lidt: tcc_token = 1207;
pub const TOK_ASM_lgdtq: tcc_token = 1206;
pub const TOK_ASM_lgdt: tcc_token = 1205;
pub const TOK_ASM_arpl: tcc_token = 1204;
pub const TOK_ASM_fxrstorq: tcc_token = 1203;
pub const TOK_ASM_fxsaveq: tcc_token = 1202;
pub const TOK_ASM_fxrstor: tcc_token = 1201;
pub const TOK_ASM_fxsave: tcc_token = 1200;
pub const TOK_ASM_ffreep: tcc_token = 1199;
pub const TOK_ASM_ffree: tcc_token = 1198;
pub const TOK_ASM_frstor: tcc_token = 1197;
pub const TOK_ASM_fsave: tcc_token = 1196;
pub const TOK_ASM_fnsave: tcc_token = 1195;
pub const TOK_ASM_fldenv: tcc_token = 1194;
pub const TOK_ASM_fstenv: tcc_token = 1193;
pub const TOK_ASM_fnstenv: tcc_token = 1192;
pub const TOK_ASM_fclex: tcc_token = 1191;
pub const TOK_ASM_fstsw: tcc_token = 1190;
pub const TOK_ASM_fstcw: tcc_token = 1189;
pub const TOK_ASM_fnstcw: tcc_token = 1188;
pub const TOK_ASM_fldcw: tcc_token = 1187;
pub const TOK_ASM_finit: tcc_token = 1186;
pub const TOK_ASM_fucomp: tcc_token = 1185;
pub const TOK_ASM_fucom: tcc_token = 1184;
pub const TOK_ASM_fbstp: tcc_token = 1183;
pub const TOK_ASM_fstpt: tcc_token = 1182;
pub const TOK_ASM_fistpll: tcc_token = 1181;
pub const TOK_ASM_fistpq: tcc_token = 1180;
pub const TOK_ASM_fstp: tcc_token = 1179;
pub const TOK_ASM_fistpl: tcc_token = 1178;
pub const TOK_ASM_fistl: tcc_token = 1177;
pub const TOK_ASM_fistp: tcc_token = 1176;
pub const TOK_ASM_fist: tcc_token = 1175;
pub const TOK_ASM_fstpl: tcc_token = 1174;
pub const TOK_ASM_fstps: tcc_token = 1173;
pub const TOK_ASM_fsts: tcc_token = 1172;
pub const TOK_ASM_fstl: tcc_token = 1171;
pub const TOK_ASM_fst: tcc_token = 1170;
pub const TOK_ASM_fbld: tcc_token = 1169;
pub const TOK_ASM_fldt: tcc_token = 1168;
pub const TOK_ASM_fildll: tcc_token = 1167;
pub const TOK_ASM_fildq: tcc_token = 1166;
pub const TOK_ASM_fildl: tcc_token = 1165;
pub const TOK_ASM_flds: tcc_token = 1164;
pub const TOK_ASM_fldl: tcc_token = 1163;
pub const TOK_ASM_fld: tcc_token = 1162;
pub const TOK_ASM_jecxz: tcc_token = 1161;
pub const TOK_ASM_loop: tcc_token = 1160;
pub const TOK_ASM_loopz: tcc_token = 1159;
pub const TOK_ASM_loope: tcc_token = 1158;
pub const TOK_ASM_loopnz: tcc_token = 1157;
pub const TOK_ASM_loopne: tcc_token = 1156;
pub const TOK_ASM_enter: tcc_token = 1155;
pub const TOK_ASM_ljmpl: tcc_token = 1154;
pub const TOK_ASM_ljmpw: tcc_token = 1153;
pub const TOK_ASM_sysretq: tcc_token = 1152;
pub const TOK_ASM_emms: tcc_token = 1151;
pub const TOK_ASM_fnstsw: tcc_token = 1150;
pub const TOK_ASM_fxch: tcc_token = 1149;
pub const TOK_ASM_fwait: tcc_token = 1148;
pub const TOK_ASM_fnop: tcc_token = 1147;
pub const TOK_ASM_fnclex: tcc_token = 1146;
pub const TOK_ASM_fninit: tcc_token = 1145;
pub const TOK_ASM_fabs: tcc_token = 1144;
pub const TOK_ASM_fchs: tcc_token = 1143;
pub const TOK_ASM_fcos: tcc_token = 1142;
pub const TOK_ASM_fsin: tcc_token = 1141;
pub const TOK_ASM_fscale: tcc_token = 1140;
pub const TOK_ASM_frndint: tcc_token = 1139;
pub const TOK_ASM_fsincos: tcc_token = 1138;
pub const TOK_ASM_fsqrt: tcc_token = 1137;
pub const TOK_ASM_fyl2xp1: tcc_token = 1136;
pub const TOK_ASM_fprem: tcc_token = 1135;
pub const TOK_ASM_fincstp: tcc_token = 1134;
pub const TOK_ASM_fdecstp: tcc_token = 1133;
pub const TOK_ASM_fprem1: tcc_token = 1132;
pub const TOK_ASM_fxtract: tcc_token = 1131;
pub const TOK_ASM_fpatan: tcc_token = 1130;
pub const TOK_ASM_fptan: tcc_token = 1129;
pub const TOK_ASM_fyl2x: tcc_token = 1128;
pub const TOK_ASM_f2xm1: tcc_token = 1127;
pub const TOK_ASM_fldz: tcc_token = 1126;
pub const TOK_ASM_fldln2: tcc_token = 1125;
pub const TOK_ASM_fldlg2: tcc_token = 1124;
pub const TOK_ASM_fldpi: tcc_token = 1123;
pub const TOK_ASM_fldl2e: tcc_token = 1122;
pub const TOK_ASM_fldl2t: tcc_token = 1121;
pub const TOK_ASM_fld1: tcc_token = 1120;
pub const TOK_ASM_fxam: tcc_token = 1119;
pub const TOK_ASM_ftst: tcc_token = 1118;
pub const TOK_ASM_fucompp: tcc_token = 1117;
pub const TOK_ASM_lret: tcc_token = 1116;
pub const TOK_ASM_retq: tcc_token = 1115;
pub const TOK_ASM_ret: tcc_token = 1114;
pub const TOK_ASM_leave: tcc_token = 1113;
pub const TOK_ASM_ud2: tcc_token = 1112;
pub const TOK_ASM_sysret: tcc_token = 1111;
pub const TOK_ASM_syscall: tcc_token = 1110;
pub const TOK_ASM_rdpmc: tcc_token = 1109;
pub const TOK_ASM_rdmsr: tcc_token = 1108;
pub const TOK_ASM_rdtsc: tcc_token = 1107;
pub const TOK_ASM_wrmsr: tcc_token = 1106;
pub const TOK_ASM_cpuid: tcc_token = 1105;
pub const TOK_ASM_wbinvd: tcc_token = 1104;
pub const TOK_ASM_invd: tcc_token = 1103;
pub const TOK_ASM_repnz: tcc_token = 1102;
pub const TOK_ASM_repne: tcc_token = 1101;
pub const TOK_ASM_repz: tcc_token = 1100;
pub const TOK_ASM_repe: tcc_token = 1099;
pub const TOK_ASM_rep: tcc_token = 1098;
pub const TOK_ASM_lock: tcc_token = 1097;
pub const TOK_ASM_xlat: tcc_token = 1096;
pub const TOK_ASM_pause: tcc_token = 1095;
pub const TOK_ASM_nop: tcc_token = 1094;
pub const TOK_ASM_wait: tcc_token = 1093;
pub const TOK_ASM_hlt: tcc_token = 1092;
pub const TOK_ASM_rsm: tcc_token = 1091;
pub const TOK_ASM_iret: tcc_token = 1090;
pub const TOK_ASM_into: tcc_token = 1089;
pub const TOK_ASM_int3: tcc_token = 1088;
pub const TOK_ASM_cqto: tcc_token = 1087;
pub const TOK_ASM_cltd: tcc_token = 1086;
pub const TOK_ASM_cwtd: tcc_token = 1085;
pub const TOK_ASM_cwtl: tcc_token = 1084;
pub const TOK_ASM_cbtw: tcc_token = 1083;
pub const TOK_ASM_cdq: tcc_token = 1082;
pub const TOK_ASM_cwde: tcc_token = 1081;
pub const TOK_ASM_cwd: tcc_token = 1080;
pub const TOK_ASM_cbw: tcc_token = 1079;
pub const TOK_ASM_aam: tcc_token = 1078;
pub const TOK_ASM_aad: tcc_token = 1077;
pub const TOK_ASM_das: tcc_token = 1076;
pub const TOK_ASM_daa: tcc_token = 1075;
pub const TOK_ASM_aas: tcc_token = 1074;
pub const TOK_ASM_aaa: tcc_token = 1073;
pub const TOK_ASM_sti: tcc_token = 1072;
pub const TOK_ASM_std: tcc_token = 1071;
pub const TOK_ASM_stc: tcc_token = 1070;
pub const TOK_ASM_popf: tcc_token = 1069;
pub const TOK_ASM_pushf: tcc_token = 1068;
pub const TOK_ASM_popfq: tcc_token = 1067;
pub const TOK_ASM_pushfq: tcc_token = 1066;
pub const TOK_ASM_sahf: tcc_token = 1065;
pub const TOK_ASM_lahf: tcc_token = 1064;
pub const TOK_ASM_cmc: tcc_token = 1063;
pub const TOK_ASM_clts: tcc_token = 1062;
pub const TOK_ASM_cli: tcc_token = 1061;
pub const TOK_ASM_cld: tcc_token = 1060;
pub const TOK_ASM_clc: tcc_token = 1059;
pub const TOK_ASM_ssto: tcc_token = 1058;
pub const TOK_ASM_sstoq: tcc_token = 1057;
pub const TOK_ASM_sstol: tcc_token = 1056;
pub const TOK_ASM_sstow: tcc_token = 1055;
pub const TOK_ASM_sstob: tcc_token = 1054;
pub const TOK_ASM_stos: tcc_token = 1053;
pub const TOK_ASM_stosq: tcc_token = 1052;
pub const TOK_ASM_stosl: tcc_token = 1051;
pub const TOK_ASM_stosw: tcc_token = 1050;
pub const TOK_ASM_stosb: tcc_token = 1049;
pub const TOK_ASM_ssca: tcc_token = 1048;
pub const TOK_ASM_sscaq: tcc_token = 1047;
pub const TOK_ASM_sscal: tcc_token = 1046;
pub const TOK_ASM_sscaw: tcc_token = 1045;
pub const TOK_ASM_sscab: tcc_token = 1044;
pub const TOK_ASM_scas: tcc_token = 1043;
pub const TOK_ASM_scasq: tcc_token = 1042;
pub const TOK_ASM_scasl: tcc_token = 1041;
pub const TOK_ASM_scasw: tcc_token = 1040;
pub const TOK_ASM_scasb: tcc_token = 1039;
pub const TOK_ASM_smov: tcc_token = 1038;
pub const TOK_ASM_smovq: tcc_token = 1037;
pub const TOK_ASM_smovl: tcc_token = 1036;
pub const TOK_ASM_smovw: tcc_token = 1035;
pub const TOK_ASM_smovb: tcc_token = 1034;
pub const TOK_ASM_movs: tcc_token = 1033;
pub const TOK_ASM_movsq: tcc_token = 1032;
pub const TOK_ASM_movsl: tcc_token = 1031;
pub const TOK_ASM_movsw: tcc_token = 1030;
pub const TOK_ASM_movsb: tcc_token = 1029;
pub const TOK_ASM_slod: tcc_token = 1028;
pub const TOK_ASM_slodq: tcc_token = 1027;
pub const TOK_ASM_slodl: tcc_token = 1026;
pub const TOK_ASM_slodw: tcc_token = 1025;
pub const TOK_ASM_slodb: tcc_token = 1024;
pub const TOK_ASM_lods: tcc_token = 1023;
pub const TOK_ASM_lodsq: tcc_token = 1022;
pub const TOK_ASM_lodsl: tcc_token = 1021;
pub const TOK_ASM_lodsw: tcc_token = 1020;
pub const TOK_ASM_lodsb: tcc_token = 1019;
pub const TOK_ASM_outs: tcc_token = 1018;
pub const TOK_ASM_outsl: tcc_token = 1017;
pub const TOK_ASM_outsw: tcc_token = 1016;
pub const TOK_ASM_outsb: tcc_token = 1015;
pub const TOK_ASM_ins: tcc_token = 1014;
pub const TOK_ASM_insl: tcc_token = 1013;
pub const TOK_ASM_insw: tcc_token = 1012;
pub const TOK_ASM_insb: tcc_token = 1011;
pub const TOK_ASM_scmp: tcc_token = 1010;
pub const TOK_ASM_scmpq: tcc_token = 1009;
pub const TOK_ASM_scmpl: tcc_token = 1008;
pub const TOK_ASM_scmpw: tcc_token = 1007;
pub const TOK_ASM_scmpb: tcc_token = 1006;
pub const TOK_ASM_cmps: tcc_token = 1005;
pub const TOK_ASM_cmpsq: tcc_token = 1004;
pub const TOK_ASM_cmpsl: tcc_token = 1003;
pub const TOK_ASM_cmpsw: tcc_token = 1002;
pub const TOK_ASM_cmpsb: tcc_token = 1001;
pub const TOK_ASM_cmpxchg: tcc_token = 1000;
pub const TOK_ASM_cmpxchgq: tcc_token = 999;
pub const TOK_ASM_cmpxchgl: tcc_token = 998;
pub const TOK_ASM_cmpxchgw: tcc_token = 997;
pub const TOK_ASM_cmpxchgb: tcc_token = 996;
pub const TOK_ASM_xadd: tcc_token = 995;
pub const TOK_ASM_xaddq: tcc_token = 994;
pub const TOK_ASM_xaddl: tcc_token = 993;
pub const TOK_ASM_xaddw: tcc_token = 992;
pub const TOK_ASM_xaddb: tcc_token = 991;
pub const TOK_ASM_fidivrs: tcc_token = 990;
pub const TOK_ASM_fdivrl: tcc_token = 989;
pub const TOK_ASM_fidivrl: tcc_token = 988;
pub const TOK_ASM_fdivrs: tcc_token = 987;
pub const TOK_ASM_fdivrp: tcc_token = 986;
pub const TOK_ASM_fdivr: tcc_token = 985;
pub const TOK_ASM_fidivs: tcc_token = 984;
pub const TOK_ASM_fdivl: tcc_token = 983;
pub const TOK_ASM_fidivl: tcc_token = 982;
pub const TOK_ASM_fdivs: tcc_token = 981;
pub const TOK_ASM_fdivp: tcc_token = 980;
pub const TOK_ASM_fdiv: tcc_token = 979;
pub const TOK_ASM_fisubrs: tcc_token = 978;
pub const TOK_ASM_fsubrl: tcc_token = 977;
pub const TOK_ASM_fisubrl: tcc_token = 976;
pub const TOK_ASM_fsubrs: tcc_token = 975;
pub const TOK_ASM_fsubrp: tcc_token = 974;
pub const TOK_ASM_fsubr: tcc_token = 973;
pub const TOK_ASM_fisubs: tcc_token = 972;
pub const TOK_ASM_fsubl: tcc_token = 971;
pub const TOK_ASM_fisubl: tcc_token = 970;
pub const TOK_ASM_fsubs: tcc_token = 969;
pub const TOK_ASM_fsubp: tcc_token = 968;
pub const TOK_ASM_fsub: tcc_token = 967;
pub const TOK_ASM_ficomps: tcc_token = 966;
pub const TOK_ASM_fcompl: tcc_token = 965;
pub const TOK_ASM_ficompl: tcc_token = 964;
pub const TOK_ASM_fcomps: tcc_token = 963;
pub const TOK_ASM_fcompp: tcc_token = 962;
pub const TOK_ASM_fcomp: tcc_token = 961;
pub const TOK_ASM_ficoms: tcc_token = 960;
pub const TOK_ASM_fcoml: tcc_token = 959;
pub const TOK_ASM_ficoml: tcc_token = 958;
pub const TOK_ASM_fcoms: tcc_token = 957;
pub const TOK_ASM_fcom_1: tcc_token = 956;
pub const TOK_ASM_fcom: tcc_token = 955;
pub const TOK_ASM_fimuls: tcc_token = 954;
pub const TOK_ASM_fmull: tcc_token = 953;
pub const TOK_ASM_fimull: tcc_token = 952;
pub const TOK_ASM_fmuls: tcc_token = 951;
pub const TOK_ASM_fmulp: tcc_token = 950;
pub const TOK_ASM_fmul: tcc_token = 949;
pub const TOK_ASM_fiadds: tcc_token = 948;
pub const TOK_ASM_faddl: tcc_token = 947;
pub const TOK_ASM_fiaddl: tcc_token = 946;
pub const TOK_ASM_fadds: tcc_token = 945;
pub const TOK_ASM_faddp: tcc_token = 944;
pub const TOK_ASM_fadd: tcc_token = 943;
pub const TOK_ASM_lsl: tcc_token = 942;
pub const TOK_ASM_lslq: tcc_token = 941;
pub const TOK_ASM_lsll: tcc_token = 940;
pub const TOK_ASM_lslw: tcc_token = 939;
pub const TOK_ASM_lar: tcc_token = 938;
pub const TOK_ASM_larq: tcc_token = 937;
pub const TOK_ASM_larl: tcc_token = 936;
pub const TOK_ASM_larw: tcc_token = 935;
pub const TOK_ASM_btc: tcc_token = 934;
pub const TOK_ASM_btcq: tcc_token = 933;
pub const TOK_ASM_btcl: tcc_token = 932;
pub const TOK_ASM_btcw: tcc_token = 931;
pub const TOK_ASM_btr: tcc_token = 930;
pub const TOK_ASM_btrq: tcc_token = 929;
pub const TOK_ASM_btrl: tcc_token = 928;
pub const TOK_ASM_btrw: tcc_token = 927;
pub const TOK_ASM_bts: tcc_token = 926;
pub const TOK_ASM_btsq: tcc_token = 925;
pub const TOK_ASM_btsl: tcc_token = 924;
pub const TOK_ASM_btsw: tcc_token = 923;
pub const TOK_ASM_bt: tcc_token = 922;
pub const TOK_ASM_btq: tcc_token = 921;
pub const TOK_ASM_btl: tcc_token = 920;
pub const TOK_ASM_btw: tcc_token = 919;
pub const TOK_ASM_bsr: tcc_token = 918;
pub const TOK_ASM_bsrq: tcc_token = 917;
pub const TOK_ASM_bsrl: tcc_token = 916;
pub const TOK_ASM_bsrw: tcc_token = 915;
pub const TOK_ASM_bsf: tcc_token = 914;
pub const TOK_ASM_bsfq: tcc_token = 913;
pub const TOK_ASM_bsfl: tcc_token = 912;
pub const TOK_ASM_bsfw: tcc_token = 911;
pub const TOK_ASM_cmovg: tcc_token = 910;
pub const TOK_ASM_cmovnle: tcc_token = 909;
pub const TOK_ASM_cmovng: tcc_token = 908;
pub const TOK_ASM_cmovle: tcc_token = 907;
pub const TOK_ASM_cmovge: tcc_token = 906;
pub const TOK_ASM_cmovnl: tcc_token = 905;
pub const TOK_ASM_cmovnge: tcc_token = 904;
pub const TOK_ASM_cmovl: tcc_token = 903;
pub const TOK_ASM_cmovpo: tcc_token = 902;
pub const TOK_ASM_cmovnp: tcc_token = 901;
pub const TOK_ASM_cmovpe: tcc_token = 900;
pub const TOK_ASM_cmovp: tcc_token = 899;
pub const TOK_ASM_cmovns: tcc_token = 898;
pub const TOK_ASM_cmovs: tcc_token = 897;
pub const TOK_ASM_cmova: tcc_token = 896;
pub const TOK_ASM_cmovnbe: tcc_token = 895;
pub const TOK_ASM_cmovna: tcc_token = 894;
pub const TOK_ASM_cmovbe: tcc_token = 893;
pub const TOK_ASM_cmovnz: tcc_token = 892;
pub const TOK_ASM_cmovne: tcc_token = 891;
pub const TOK_ASM_cmovz: tcc_token = 890;
pub const TOK_ASM_cmove: tcc_token = 889;
pub const TOK_ASM_cmovae: tcc_token = 888;
pub const TOK_ASM_cmovnc: tcc_token = 887;
pub const TOK_ASM_cmovnb: tcc_token = 886;
pub const TOK_ASM_cmovnae: tcc_token = 885;
pub const TOK_ASM_cmovc: tcc_token = 884;
pub const TOK_ASM_cmovb: tcc_token = 883;
pub const TOK_ASM_cmovno: tcc_token = 882;
pub const TOK_ASM_cmovo: tcc_token = 881;
pub const TOK_ASM_setgb: tcc_token = 880;
pub const TOK_ASM_setnleb: tcc_token = 879;
pub const TOK_ASM_setngb: tcc_token = 878;
pub const TOK_ASM_setleb: tcc_token = 877;
pub const TOK_ASM_setgeb: tcc_token = 876;
pub const TOK_ASM_setnlb: tcc_token = 875;
pub const TOK_ASM_setngeb: tcc_token = 874;
pub const TOK_ASM_setlb: tcc_token = 873;
pub const TOK_ASM_setpob: tcc_token = 872;
pub const TOK_ASM_setnpb: tcc_token = 871;
pub const TOK_ASM_setpeb: tcc_token = 870;
pub const TOK_ASM_setpb: tcc_token = 869;
pub const TOK_ASM_setnsb: tcc_token = 868;
pub const TOK_ASM_setsb: tcc_token = 867;
pub const TOK_ASM_setab: tcc_token = 866;
pub const TOK_ASM_setnbeb: tcc_token = 865;
pub const TOK_ASM_setnab: tcc_token = 864;
pub const TOK_ASM_setbeb: tcc_token = 863;
pub const TOK_ASM_setnzb: tcc_token = 862;
pub const TOK_ASM_setneb: tcc_token = 861;
pub const TOK_ASM_setzb: tcc_token = 860;
pub const TOK_ASM_seteb: tcc_token = 859;
pub const TOK_ASM_setaeb: tcc_token = 858;
pub const TOK_ASM_setncb: tcc_token = 857;
pub const TOK_ASM_setnbb: tcc_token = 856;
pub const TOK_ASM_setnaeb: tcc_token = 855;
pub const TOK_ASM_setcb: tcc_token = 854;
pub const TOK_ASM_setbb: tcc_token = 853;
pub const TOK_ASM_setnob: tcc_token = 852;
pub const TOK_ASM_setob: tcc_token = 851;
pub const TOK_ASM_setg: tcc_token = 850;
pub const TOK_ASM_setnle: tcc_token = 849;
pub const TOK_ASM_setng: tcc_token = 848;
pub const TOK_ASM_setle: tcc_token = 847;
pub const TOK_ASM_setge: tcc_token = 846;
pub const TOK_ASM_setnl: tcc_token = 845;
pub const TOK_ASM_setnge: tcc_token = 844;
pub const TOK_ASM_setl: tcc_token = 843;
pub const TOK_ASM_setpo: tcc_token = 842;
pub const TOK_ASM_setnp: tcc_token = 841;
pub const TOK_ASM_setpe: tcc_token = 840;
pub const TOK_ASM_setp: tcc_token = 839;
pub const TOK_ASM_setns: tcc_token = 838;
pub const TOK_ASM_sets: tcc_token = 837;
pub const TOK_ASM_seta: tcc_token = 836;
pub const TOK_ASM_setnbe: tcc_token = 835;
pub const TOK_ASM_setna: tcc_token = 834;
pub const TOK_ASM_setbe: tcc_token = 833;
pub const TOK_ASM_setnz: tcc_token = 832;
pub const TOK_ASM_setne: tcc_token = 831;
pub const TOK_ASM_setz: tcc_token = 830;
pub const TOK_ASM_sete: tcc_token = 829;
pub const TOK_ASM_setae: tcc_token = 828;
pub const TOK_ASM_setnc: tcc_token = 827;
pub const TOK_ASM_setnb: tcc_token = 826;
pub const TOK_ASM_setnae: tcc_token = 825;
pub const TOK_ASM_setc: tcc_token = 824;
pub const TOK_ASM_setb: tcc_token = 823;
pub const TOK_ASM_setno: tcc_token = 822;
pub const TOK_ASM_seto: tcc_token = 821;
pub const TOK_ASM_jg: tcc_token = 820;
pub const TOK_ASM_jnle: tcc_token = 819;
pub const TOK_ASM_jng: tcc_token = 818;
pub const TOK_ASM_jle: tcc_token = 817;
pub const TOK_ASM_jge: tcc_token = 816;
pub const TOK_ASM_jnl: tcc_token = 815;
pub const TOK_ASM_jnge: tcc_token = 814;
pub const TOK_ASM_jl: tcc_token = 813;
pub const TOK_ASM_jpo: tcc_token = 812;
pub const TOK_ASM_jnp: tcc_token = 811;
pub const TOK_ASM_jpe: tcc_token = 810;
pub const TOK_ASM_jp: tcc_token = 809;
pub const TOK_ASM_jns: tcc_token = 808;
pub const TOK_ASM_js: tcc_token = 807;
pub const TOK_ASM_ja: tcc_token = 806;
pub const TOK_ASM_jnbe: tcc_token = 805;
pub const TOK_ASM_jna: tcc_token = 804;
pub const TOK_ASM_jbe: tcc_token = 803;
pub const TOK_ASM_jnz: tcc_token = 802;
pub const TOK_ASM_jne: tcc_token = 801;
pub const TOK_ASM_jz: tcc_token = 800;
pub const TOK_ASM_je: tcc_token = 799;
pub const TOK_ASM_jae: tcc_token = 798;
pub const TOK_ASM_jnc: tcc_token = 797;
pub const TOK_ASM_jnb: tcc_token = 796;
pub const TOK_ASM_jnae: tcc_token = 795;
pub const TOK_ASM_jc: tcc_token = 794;
pub const TOK_ASM_jb: tcc_token = 793;
pub const TOK_ASM_jno: tcc_token = 792;
pub const TOK_ASM_jo: tcc_token = 791;
pub const TOK_ASM_ljmp: tcc_token = 790;
pub const TOK_ASM_lcall: tcc_token = 789;
pub const TOK_ASM_jmp: tcc_token = 788;
pub const TOK_ASM_call: tcc_token = 787;
pub const TOK_ASM_lgs: tcc_token = 786;
pub const TOK_ASM_lfs: tcc_token = 785;
pub const TOK_ASM_lss: tcc_token = 784;
pub const TOK_ASM_lds: tcc_token = 783;
pub const TOK_ASM_les: tcc_token = 782;
pub const TOK_ASM_lea: tcc_token = 781;
pub const TOK_ASM_leaq: tcc_token = 780;
pub const TOK_ASM_leal: tcc_token = 779;
pub const TOK_ASM_leaw: tcc_token = 778;
pub const TOK_ASM_movslq: tcc_token = 777;
pub const TOK_ASM_movzwq: tcc_token = 776;
pub const TOK_ASM_movswq: tcc_token = 775;
pub const TOK_ASM_movsbq: tcc_token = 774;
pub const TOK_ASM_movswl: tcc_token = 773;
pub const TOK_ASM_movsbl: tcc_token = 772;
pub const TOK_ASM_movsbw: tcc_token = 771;
pub const TOK_ASM_movzwl: tcc_token = 770;
pub const TOK_ASM_movzb: tcc_token = 769;
pub const TOK_ASM_movzbq: tcc_token = 768;
pub const TOK_ASM_movzbl: tcc_token = 767;
pub const TOK_ASM_movzbw: tcc_token = 766;
pub const TOK_ASM_out: tcc_token = 765;
pub const TOK_ASM_outl: tcc_token = 764;
pub const TOK_ASM_outw: tcc_token = 763;
pub const TOK_ASM_outb: tcc_token = 762;
pub const TOK_ASM_in: tcc_token = 761;
pub const TOK_ASM_inl: tcc_token = 760;
pub const TOK_ASM_inw: tcc_token = 759;
pub const TOK_ASM_inb: tcc_token = 758;
pub const TOK_ASM_pop: tcc_token = 757;
pub const TOK_ASM_popq: tcc_token = 756;
pub const TOK_ASM_popl: tcc_token = 755;
pub const TOK_ASM_popw: tcc_token = 754;
pub const TOK_ASM_push: tcc_token = 753;
pub const TOK_ASM_pushq: tcc_token = 752;
pub const TOK_ASM_pushl: tcc_token = 751;
pub const TOK_ASM_pushw: tcc_token = 750;
pub const TOK_ASM_shrd: tcc_token = 749;
pub const TOK_ASM_shrdq: tcc_token = 748;
pub const TOK_ASM_shrdl: tcc_token = 747;
pub const TOK_ASM_shrdw: tcc_token = 746;
pub const TOK_ASM_shld: tcc_token = 745;
pub const TOK_ASM_shldq: tcc_token = 744;
pub const TOK_ASM_shldl: tcc_token = 743;
pub const TOK_ASM_shldw: tcc_token = 742;
pub const TOK_ASM_sar: tcc_token = 741;
pub const TOK_ASM_sarq: tcc_token = 740;
pub const TOK_ASM_sarl: tcc_token = 739;
pub const TOK_ASM_sarw: tcc_token = 738;
pub const TOK_ASM_sarb: tcc_token = 737;
pub const TOK_ASM_shr: tcc_token = 736;
pub const TOK_ASM_shrq: tcc_token = 735;
pub const TOK_ASM_shrl: tcc_token = 734;
pub const TOK_ASM_shrw: tcc_token = 733;
pub const TOK_ASM_shrb: tcc_token = 732;
pub const TOK_ASM_shl: tcc_token = 731;
pub const TOK_ASM_shlq: tcc_token = 730;
pub const TOK_ASM_shll: tcc_token = 729;
pub const TOK_ASM_shlw: tcc_token = 728;
pub const TOK_ASM_shlb: tcc_token = 727;
pub const TOK_ASM_rcr: tcc_token = 726;
pub const TOK_ASM_rcrq: tcc_token = 725;
pub const TOK_ASM_rcrl: tcc_token = 724;
pub const TOK_ASM_rcrw: tcc_token = 723;
pub const TOK_ASM_rcrb: tcc_token = 722;
pub const TOK_ASM_rcl: tcc_token = 721;
pub const TOK_ASM_rclq: tcc_token = 720;
pub const TOK_ASM_rcll: tcc_token = 719;
pub const TOK_ASM_rclw: tcc_token = 718;
pub const TOK_ASM_rclb: tcc_token = 717;
pub const TOK_ASM_ror: tcc_token = 716;
pub const TOK_ASM_rorq: tcc_token = 715;
pub const TOK_ASM_rorl: tcc_token = 714;
pub const TOK_ASM_rorw: tcc_token = 713;
pub const TOK_ASM_rorb: tcc_token = 712;
pub const TOK_ASM_rol: tcc_token = 711;
pub const TOK_ASM_rolq: tcc_token = 710;
pub const TOK_ASM_roll: tcc_token = 709;
pub const TOK_ASM_rolw: tcc_token = 708;
pub const TOK_ASM_rolb: tcc_token = 707;
pub const TOK_ASM_test: tcc_token = 706;
pub const TOK_ASM_testq: tcc_token = 705;
pub const TOK_ASM_testl: tcc_token = 704;
pub const TOK_ASM_testw: tcc_token = 703;
pub const TOK_ASM_testb: tcc_token = 702;
pub const TOK_ASM_xchg: tcc_token = 701;
pub const TOK_ASM_xchgq: tcc_token = 700;
pub const TOK_ASM_xchgl: tcc_token = 699;
pub const TOK_ASM_xchgw: tcc_token = 698;
pub const TOK_ASM_xchgb: tcc_token = 697;
pub const TOK_ASM_idiv: tcc_token = 696;
pub const TOK_ASM_idivq: tcc_token = 695;
pub const TOK_ASM_idivl: tcc_token = 694;
pub const TOK_ASM_idivw: tcc_token = 693;
pub const TOK_ASM_idivb: tcc_token = 692;
pub const TOK_ASM_div: tcc_token = 691;
pub const TOK_ASM_divq: tcc_token = 690;
pub const TOK_ASM_divl: tcc_token = 689;
pub const TOK_ASM_divw: tcc_token = 688;
pub const TOK_ASM_divb: tcc_token = 687;
pub const TOK_ASM_imul: tcc_token = 686;
pub const TOK_ASM_imulq: tcc_token = 685;
pub const TOK_ASM_imull: tcc_token = 684;
pub const TOK_ASM_imulw: tcc_token = 683;
pub const TOK_ASM_imulb: tcc_token = 682;
pub const TOK_ASM_mul: tcc_token = 681;
pub const TOK_ASM_mulq: tcc_token = 680;
pub const TOK_ASM_mull: tcc_token = 679;
pub const TOK_ASM_mulw: tcc_token = 678;
pub const TOK_ASM_mulb: tcc_token = 677;
pub const TOK_ASM_neg: tcc_token = 676;
pub const TOK_ASM_negq: tcc_token = 675;
pub const TOK_ASM_negl: tcc_token = 674;
pub const TOK_ASM_negw: tcc_token = 673;
pub const TOK_ASM_negb: tcc_token = 672;
pub const TOK_ASM_not: tcc_token = 671;
pub const TOK_ASM_notq: tcc_token = 670;
pub const TOK_ASM_notl: tcc_token = 669;
pub const TOK_ASM_notw: tcc_token = 668;
pub const TOK_ASM_notb: tcc_token = 667;
pub const TOK_ASM_dec: tcc_token = 666;
pub const TOK_ASM_decq: tcc_token = 665;
pub const TOK_ASM_decl: tcc_token = 664;
pub const TOK_ASM_decw: tcc_token = 663;
pub const TOK_ASM_decb: tcc_token = 662;
pub const TOK_ASM_inc: tcc_token = 661;
pub const TOK_ASM_incq: tcc_token = 660;
pub const TOK_ASM_incl: tcc_token = 659;
pub const TOK_ASM_incw: tcc_token = 658;
pub const TOK_ASM_incb: tcc_token = 657;
pub const TOK_ASM_cmp: tcc_token = 656;
pub const TOK_ASM_cmpq: tcc_token = 655;
pub const TOK_ASM_cmpl: tcc_token = 654;
pub const TOK_ASM_cmpw: tcc_token = 653;
pub const TOK_ASM_cmpb: tcc_token = 652;
pub const TOK_ASM_xor: tcc_token = 651;
pub const TOK_ASM_xorq: tcc_token = 650;
pub const TOK_ASM_xorl: tcc_token = 649;
pub const TOK_ASM_xorw: tcc_token = 648;
pub const TOK_ASM_xorb: tcc_token = 647;
pub const TOK_ASM_sub: tcc_token = 646;
pub const TOK_ASM_subq: tcc_token = 645;
pub const TOK_ASM_subl: tcc_token = 644;
pub const TOK_ASM_subw: tcc_token = 643;
pub const TOK_ASM_subb: tcc_token = 642;
pub const TOK_ASM_and: tcc_token = 641;
pub const TOK_ASM_andq: tcc_token = 640;
pub const TOK_ASM_andl: tcc_token = 639;
pub const TOK_ASM_andw: tcc_token = 638;
pub const TOK_ASM_andb: tcc_token = 637;
pub const TOK_ASM_sbb: tcc_token = 636;
pub const TOK_ASM_sbbq: tcc_token = 635;
pub const TOK_ASM_sbbl: tcc_token = 634;
pub const TOK_ASM_sbbw: tcc_token = 633;
pub const TOK_ASM_sbbb: tcc_token = 632;
pub const TOK_ASM_adc: tcc_token = 631;
pub const TOK_ASM_adcq: tcc_token = 630;
pub const TOK_ASM_adcl: tcc_token = 629;
pub const TOK_ASM_adcw: tcc_token = 628;
pub const TOK_ASM_adcb: tcc_token = 627;
pub const TOK_ASM_or: tcc_token = 626;
pub const TOK_ASM_orq: tcc_token = 625;
pub const TOK_ASM_orl: tcc_token = 624;
pub const TOK_ASM_orw: tcc_token = 623;
pub const TOK_ASM_orb: tcc_token = 622;
pub const TOK_ASM_add: tcc_token = 621;
pub const TOK_ASM_addq: tcc_token = 620;
pub const TOK_ASM_addl: tcc_token = 619;
pub const TOK_ASM_addw: tcc_token = 618;
pub const TOK_ASM_addb: tcc_token = 617;
pub const TOK_ASM_mov: tcc_token = 616;
pub const TOK_ASM_movq: tcc_token = 615;
pub const TOK_ASM_movl: tcc_token = 614;
pub const TOK_ASM_movw: tcc_token = 613;
pub const TOK_ASM_movb: tcc_token = 612;
pub const TOK_ASM_dil: tcc_token = 611;
pub const TOK_ASM_sil: tcc_token = 610;
pub const TOK_ASM_bpl: tcc_token = 609;
pub const TOK_ASM_spl: tcc_token = 608;
pub const TOK_ASM_rip: tcc_token = 607;
pub const TOK_ASM_st: tcc_token = 606;
pub const TOK_ASM_gs: tcc_token = 605;
pub const TOK_ASM_fs: tcc_token = 604;
pub const TOK_ASM_ds: tcc_token = 603;
pub const TOK_ASM_ss: tcc_token = 602;
pub const TOK_ASM_cs: tcc_token = 601;
pub const TOK_ASM_es: tcc_token = 600;
pub const TOK_ASM_dr7: tcc_token = 599;
pub const TOK_ASM_dr6: tcc_token = 598;
pub const TOK_ASM_dr5: tcc_token = 597;
pub const TOK_ASM_dr4: tcc_token = 596;
pub const TOK_ASM_dr3: tcc_token = 595;
pub const TOK_ASM_dr2: tcc_token = 594;
pub const TOK_ASM_dr1: tcc_token = 593;
pub const TOK_ASM_dr0: tcc_token = 592;
pub const TOK_ASM_db7: tcc_token = 591;
pub const TOK_ASM_db6: tcc_token = 590;
pub const TOK_ASM_db5: tcc_token = 589;
pub const TOK_ASM_db4: tcc_token = 588;
pub const TOK_ASM_db3: tcc_token = 587;
pub const TOK_ASM_db2: tcc_token = 586;
pub const TOK_ASM_db1: tcc_token = 585;
pub const TOK_ASM_db0: tcc_token = 584;
pub const TOK_ASM_tr7: tcc_token = 583;
pub const TOK_ASM_tr6: tcc_token = 582;
pub const TOK_ASM_tr5: tcc_token = 581;
pub const TOK_ASM_tr4: tcc_token = 580;
pub const TOK_ASM_tr3: tcc_token = 579;
pub const TOK_ASM_tr2: tcc_token = 578;
pub const TOK_ASM_tr1: tcc_token = 577;
pub const TOK_ASM_tr0: tcc_token = 576;
pub const TOK_ASM_cr7: tcc_token = 575;
pub const TOK_ASM_cr6: tcc_token = 574;
pub const TOK_ASM_cr5: tcc_token = 573;
pub const TOK_ASM_cr4: tcc_token = 572;
pub const TOK_ASM_cr3: tcc_token = 571;
pub const TOK_ASM_cr2: tcc_token = 570;
pub const TOK_ASM_cr1: tcc_token = 569;
pub const TOK_ASM_cr0: tcc_token = 568;
pub const TOK_ASM_xmm7: tcc_token = 567;
pub const TOK_ASM_xmm6: tcc_token = 566;
pub const TOK_ASM_xmm5: tcc_token = 565;
pub const TOK_ASM_xmm4: tcc_token = 564;
pub const TOK_ASM_xmm3: tcc_token = 563;
pub const TOK_ASM_xmm2: tcc_token = 562;
pub const TOK_ASM_xmm1: tcc_token = 561;
pub const TOK_ASM_xmm0: tcc_token = 560;
pub const TOK_ASM_mm7: tcc_token = 559;
pub const TOK_ASM_mm6: tcc_token = 558;
pub const TOK_ASM_mm5: tcc_token = 557;
pub const TOK_ASM_mm4: tcc_token = 556;
pub const TOK_ASM_mm3: tcc_token = 555;
pub const TOK_ASM_mm2: tcc_token = 554;
pub const TOK_ASM_mm1: tcc_token = 553;
pub const TOK_ASM_mm0: tcc_token = 552;
pub const TOK_ASM_rdi: tcc_token = 551;
pub const TOK_ASM_rsi: tcc_token = 550;
pub const TOK_ASM_rbp: tcc_token = 549;
pub const TOK_ASM_rsp: tcc_token = 548;
pub const TOK_ASM_rbx: tcc_token = 547;
pub const TOK_ASM_rdx: tcc_token = 546;
pub const TOK_ASM_rcx: tcc_token = 545;
pub const TOK_ASM_rax: tcc_token = 544;
pub const TOK_ASM_edi: tcc_token = 543;
pub const TOK_ASM_esi: tcc_token = 542;
pub const TOK_ASM_ebp: tcc_token = 541;
pub const TOK_ASM_esp: tcc_token = 540;
pub const TOK_ASM_ebx: tcc_token = 539;
pub const TOK_ASM_edx: tcc_token = 538;
pub const TOK_ASM_ecx: tcc_token = 537;
pub const TOK_ASM_eax: tcc_token = 536;
pub const TOK_ASM_di: tcc_token = 535;
pub const TOK_ASM_si: tcc_token = 534;
pub const TOK_ASM_bp: tcc_token = 533;
pub const TOK_ASM_sp: tcc_token = 532;
pub const TOK_ASM_bx: tcc_token = 531;
pub const TOK_ASM_dx: tcc_token = 530;
pub const TOK_ASM_cx: tcc_token = 529;
pub const TOK_ASM_ax: tcc_token = 528;
pub const TOK_ASM_bh: tcc_token = 527;
pub const TOK_ASM_dh: tcc_token = 526;
pub const TOK_ASM_ch: tcc_token = 525;
pub const TOK_ASM_ah: tcc_token = 524;
pub const TOK_ASM_bl: tcc_token = 523;
pub const TOK_ASM_dl: tcc_token = 522;
pub const TOK_ASM_cl: tcc_token = 521;
pub const TOK_ASM_al: tcc_token = 520;
pub const TOK_ASMDIR_section: tcc_token = 519;
pub const TOK_ASMDIR_int: tcc_token = 518;
pub const TOK_ASMDIR_long: tcc_token = 517;
pub const TOK_ASMDIR_short: tcc_token = 516;
pub const TOK_ASMDIR_code64: tcc_token = 515;
pub const TOK_ASMDIR_quad: tcc_token = 514;
pub const TOK_ASMDIR_org: tcc_token = 513;
pub const TOK_ASMDIR_endr: tcc_token = 512;
pub const TOK_ASMDIR_rept: tcc_token = 511;
pub const TOK_ASMDIR_fill: tcc_token = 510;
pub const TOK_ASMDIR_popsection: tcc_token = 509;
pub const TOK_ASMDIR_pushsection: tcc_token = 508;
pub const TOK_ASMDIR_previous: tcc_token = 507;
pub const TOK_ASMDIR_bss: tcc_token = 506;
pub const TOK_ASMDIR_data: tcc_token = 505;
pub const TOK_ASMDIR_text: tcc_token = 504;
pub const TOK_ASMDIR_type: tcc_token = 503;
pub const TOK_ASMDIR_size: tcc_token = 502;
pub const TOK_ASMDIR_ident: tcc_token = 501;
pub const TOK_ASMDIR_hidden: tcc_token = 500;
pub const TOK_ASMDIR_weak: tcc_token = 499;
pub const TOK_ASMDIR_global: tcc_token = 498;
pub const TOK_ASMDIR_globl: tcc_token = 497;
pub const TOK_ASMDIR_file: tcc_token = 496;
pub const TOK_ASMDIR_ascii: tcc_token = 495;
pub const TOK_ASMDIR_asciz: tcc_token = 494;
pub const TOK_ASMDIR_string: tcc_token = 493;
pub const TOK_ASMDIR_space: tcc_token = 492;
pub const TOK_ASMDIR_skip: tcc_token = 491;
pub const TOK_ASMDIR_set: tcc_token = 490;
pub const TOK_ASMDIR_p2align: tcc_token = 489;
pub const TOK_ASMDIR_balign: tcc_token = 488;
pub const TOK_ASMDIR_align: tcc_token = 487;
pub const TOK_ASMDIR_word: tcc_token = 486;
pub const TOK_ASMDIR_byte: tcc_token = 485;
pub const TOK_longjmp: tcc_token = 484;
pub const TOK__setjmp: tcc_token = 483;
pub const TOK_setjmp: tcc_token = 482;
pub const TOK_siglongjmp: tcc_token = 481;
pub const TOK___sigsetjmp: tcc_token = 480;
pub const TOK_sigsetjmp: tcc_token = 479;
pub const TOK___bound_new_region: tcc_token = 478;
pub const TOK___bound_longjmp: tcc_token = 477;
pub const TOK___bound_setjmp: tcc_token = 476;
pub const TOK___bound_local_delete: tcc_token = 475;
pub const TOK___bound_local_new: tcc_token = 474;
pub const TOK___bound_main_arg: tcc_token = 473;
pub const TOK___bound_ptr_indir16: tcc_token = 472;
pub const TOK___bound_ptr_indir12: tcc_token = 471;
pub const TOK___bound_ptr_indir8: tcc_token = 470;
pub const TOK___bound_ptr_indir4: tcc_token = 469;
pub const TOK___bound_ptr_indir2: tcc_token = 468;
pub const TOK___bound_ptr_indir1: tcc_token = 467;
pub const TOK___bound_ptr_add: tcc_token = 466;
pub const TOK_alloca: tcc_token = 465;
pub const TOK___fixunsdfdi: tcc_token = 464;
pub const TOK___fixunssfdi: tcc_token = 463;
pub const TOK___fixunsxfdi: tcc_token = 462;
pub const TOK___floatundixf: tcc_token = 461;
pub const TOK___floatundidf: tcc_token = 460;
pub const TOK___floatundisf: tcc_token = 459;
pub const TOK___ashldi3: tcc_token = 458;
pub const TOK___lshrdi3: tcc_token = 457;
pub const TOK___ashrdi3: tcc_token = 456;
pub const TOK___umoddi3: tcc_token = 455;
pub const TOK___udivdi3: tcc_token = 454;
pub const TOK___moddi3: tcc_token = 453;
pub const TOK___divdi3: tcc_token = 452;
pub const TOK_memset: tcc_token = 451;
pub const TOK_memmove: tcc_token = 450;
pub const TOK_memcpy: tcc_token = 449;
pub const TOK_option: tcc_token = 448;
pub const TOK_once: tcc_token = 447;
pub const TOK_pop_macro: tcc_token = 446;
pub const TOK_push_macro: tcc_token = 445;
pub const TOK_lib: tcc_token = 444;
pub const TOK_comment: tcc_token = 443;
pub const TOK_pack: tcc_token = 442;
pub const TOK___atomic_fetch_and_8: tcc_token = 441;
pub const TOK___atomic_fetch_and_4: tcc_token = 440;
pub const TOK___atomic_fetch_and_2: tcc_token = 439;
pub const TOK___atomic_fetch_and_1: tcc_token = 438;
pub const TOK___atomic_fetch_and: tcc_token = 437;
pub const TOK___atomic_fetch_xor_8: tcc_token = 436;
pub const TOK___atomic_fetch_xor_4: tcc_token = 435;
pub const TOK___atomic_fetch_xor_2: tcc_token = 434;
pub const TOK___atomic_fetch_xor_1: tcc_token = 433;
pub const TOK___atomic_fetch_xor: tcc_token = 432;
pub const TOK___atomic_fetch_or_8: tcc_token = 431;
pub const TOK___atomic_fetch_or_4: tcc_token = 430;
pub const TOK___atomic_fetch_or_2: tcc_token = 429;
pub const TOK___atomic_fetch_or_1: tcc_token = 428;
pub const TOK___atomic_fetch_or: tcc_token = 427;
pub const TOK___atomic_fetch_sub_8: tcc_token = 426;
pub const TOK___atomic_fetch_sub_4: tcc_token = 425;
pub const TOK___atomic_fetch_sub_2: tcc_token = 424;
pub const TOK___atomic_fetch_sub_1: tcc_token = 423;
pub const TOK___atomic_fetch_sub: tcc_token = 422;
pub const TOK___atomic_fetch_add_8: tcc_token = 421;
pub const TOK___atomic_fetch_add_4: tcc_token = 420;
pub const TOK___atomic_fetch_add_2: tcc_token = 419;
pub const TOK___atomic_fetch_add_1: tcc_token = 418;
pub const TOK___atomic_fetch_add: tcc_token = 417;
pub const TOK___atomic_compare_exchange_8: tcc_token = 416;
pub const TOK___atomic_compare_exchange_4: tcc_token = 415;
pub const TOK___atomic_compare_exchange_2: tcc_token = 414;
pub const TOK___atomic_compare_exchange_1: tcc_token = 413;
pub const TOK___atomic_compare_exchange: tcc_token = 412;
pub const TOK___atomic_exchange_8: tcc_token = 411;
pub const TOK___atomic_exchange_4: tcc_token = 410;
pub const TOK___atomic_exchange_2: tcc_token = 409;
pub const TOK___atomic_exchange_1: tcc_token = 408;
pub const TOK___atomic_exchange: tcc_token = 407;
pub const TOK___atomic_load_8: tcc_token = 406;
pub const TOK___atomic_load_4: tcc_token = 405;
pub const TOK___atomic_load_2: tcc_token = 404;
pub const TOK___atomic_load_1: tcc_token = 403;
pub const TOK___atomic_load: tcc_token = 402;
pub const TOK___atomic_store_8: tcc_token = 401;
pub const TOK___atomic_store_4: tcc_token = 400;
pub const TOK___atomic_store_2: tcc_token = 399;
pub const TOK___atomic_store_1: tcc_token = 398;
pub const TOK___atomic_store: tcc_token = 397;
pub const TOK_builtin_va_arg_types: tcc_token = 396;
pub const TOK_builtin_expect: tcc_token = 395;
pub const TOK_builtin_return_address: tcc_token = 394;
pub const TOK_builtin_frame_address: tcc_token = 393;
pub const TOK_builtin_constant_p: tcc_token = 392;
pub const TOK_builtin_choose_expr: tcc_token = 391;
pub const TOK_builtin_types_compatible_p: tcc_token = 390;
pub const TOK_VISIBILITY2: tcc_token = 389;
pub const TOK_VISIBILITY1: tcc_token = 388;
pub const TOK_NORETURN3: tcc_token = 387;
pub const TOK_NORETURN2: tcc_token = 386;
pub const TOK_NORETURN1: tcc_token = 385;
pub const TOK_NODECORATE: tcc_token = 384;
pub const TOK_DLLIMPORT: tcc_token = 383;
pub const TOK_DLLEXPORT: tcc_token = 382;
pub const TOK_MODE_word: tcc_token = 381;
pub const TOK_MODE_SI: tcc_token = 380;
pub const TOK_MODE_HI: tcc_token = 379;
pub const TOK_MODE_DI: tcc_token = 378;
pub const TOK_MODE_QI: tcc_token = 377;
pub const TOK_MODE: tcc_token = 376;
pub const TOK_ALWAYS_INLINE2: tcc_token = 375;
pub const TOK_ALWAYS_INLINE1: tcc_token = 374;
pub const TOK_DESTRUCTOR2: tcc_token = 373;
pub const TOK_DESTRUCTOR1: tcc_token = 372;
pub const TOK_CONSTRUCTOR2: tcc_token = 371;
pub const TOK_CONSTRUCTOR1: tcc_token = 370;
pub const TOK_CLEANUP2: tcc_token = 369;
pub const TOK_CLEANUP1: tcc_token = 368;
pub const TOK_REGPARM2: tcc_token = 367;
pub const TOK_REGPARM1: tcc_token = 366;
pub const TOK_FASTCALL3: tcc_token = 365;
pub const TOK_FASTCALL2: tcc_token = 364;
pub const TOK_FASTCALL1: tcc_token = 363;
pub const TOK_STDCALL3: tcc_token = 362;
pub const TOK_STDCALL2: tcc_token = 361;
pub const TOK_STDCALL1: tcc_token = 360;
pub const TOK_CDECL3: tcc_token = 359;
pub const TOK_CDECL2: tcc_token = 358;
pub const TOK_CDECL1: tcc_token = 357;
pub const TOK_UNUSED2: tcc_token = 356;
pub const TOK_UNUSED1: tcc_token = 355;
pub const TOK_ALIAS2: tcc_token = 354;
pub const TOK_ALIAS1: tcc_token = 353;
pub const TOK_WEAK2: tcc_token = 352;
pub const TOK_WEAK1: tcc_token = 351;
pub const TOK_PACKED2: tcc_token = 350;
pub const TOK_PACKED1: tcc_token = 349;
pub const TOK_ALIGNED2: tcc_token = 348;
pub const TOK_ALIGNED1: tcc_token = 347;
pub const TOK_SECTION2: tcc_token = 346;
pub const TOK_SECTION1: tcc_token = 345;
pub const TOK___mzerodf: tcc_token = 344;
pub const TOK___mzerosf: tcc_token = 343;
pub const TOK___INF__: tcc_token = 342;
pub const TOK___SNAN__: tcc_token = 341;
pub const TOK___NAN__: tcc_token = 340;
pub const TOK___FUNC__: tcc_token = 339;
pub const TOK___HAS_INCLUDE: tcc_token = 338;
pub const TOK___COUNTER__: tcc_token = 337;
pub const TOK___VA_ARGS__: tcc_token = 336;
pub const TOK___FUNCTION__: tcc_token = 335;
pub const TOK___TIME__: tcc_token = 334;
pub const TOK___DATE__: tcc_token = 333;
pub const TOK___FILE__: tcc_token = 332;
pub const TOK___LINE__: tcc_token = 331;
pub const TOK_PRAGMA: tcc_token = 330;
pub const TOK_LINE: tcc_token = 329;
pub const TOK_WARNING: tcc_token = 328;
pub const TOK_ERROR: tcc_token = 327;
pub const TOK_UNDEF: tcc_token = 326;
pub const TOK_DEFINED: tcc_token = 325;
pub const TOK_ENDIF: tcc_token = 324;
pub const TOK_ELIF: tcc_token = 323;
pub const TOK_IFNDEF: tcc_token = 322;
pub const TOK_IFDEF: tcc_token = 321;
pub const TOK_INCLUDE_NEXT: tcc_token = 320;
pub const TOK_INCLUDE: tcc_token = 319;
pub const TOK_DEFINE: tcc_token = 318;
pub const TOK_ASM3: tcc_token = 317;
pub const TOK_ASM2: tcc_token = 316;
pub const TOK_ASM1: tcc_token = 315;
pub const TOK_LABEL: tcc_token = 314;
pub const TOK_TYPEOF3: tcc_token = 313;
pub const TOK_TYPEOF2: tcc_token = 312;
pub const TOK_TYPEOF1: tcc_token = 311;
pub const TOK_ALIGNAS: tcc_token = 310;
pub const TOK_ALIGNOF3: tcc_token = 309;
pub const TOK_ALIGNOF2: tcc_token = 308;
pub const TOK_ALIGNOF1: tcc_token = 307;
pub const TOK_ATTRIBUTE2: tcc_token = 306;
pub const TOK_ATTRIBUTE1: tcc_token = 305;
pub const TOK_SIZEOF: tcc_token = 304;
pub const TOK_ENUM: tcc_token = 303;
pub const TOK_DEFAULT: tcc_token = 302;
pub const TOK_TYPEDEF: tcc_token = 301;
pub const TOK_UNION: tcc_token = 300;
pub const TOK_STRUCT: tcc_token = 299;
pub const TOK_SHORT: tcc_token = 298;
pub const TOK_BOOL: tcc_token = 297;
pub const TOK_DOUBLE: tcc_token = 296;
pub const TOK_FLOAT: tcc_token = 295;
pub const TOK_STATIC_ASSERT: tcc_token = 294;
pub const TOK_GENERIC: tcc_token = 293;
pub const TOK_EXTENSION: tcc_token = 292;
pub const TOK_RESTRICT3: tcc_token = 291;
pub const TOK_RESTRICT2: tcc_token = 290;
pub const TOK_RESTRICT1: tcc_token = 289;
pub const TOK_INLINE3: tcc_token = 288;
pub const TOK_INLINE2: tcc_token = 287;
pub const TOK_INLINE1: tcc_token = 286;
pub const TOK_AUTO: tcc_token = 285;
pub const TOK_SIGNED3: tcc_token = 284;
pub const TOK_SIGNED2: tcc_token = 283;
pub const TOK_SIGNED1: tcc_token = 282;
pub const TOK_REGISTER: tcc_token = 281;
pub const TOK_LONG: tcc_token = 280;
pub const TOK_VOLATILE3: tcc_token = 279;
pub const TOK_VOLATILE2: tcc_token = 278;
pub const TOK_VOLATILE1: tcc_token = 277;
pub const TOK_CONST3: tcc_token = 276;
pub const TOK_CONST2: tcc_token = 275;
pub const TOK_CONST1: tcc_token = 274;
pub const TOK__Atomic: tcc_token = 273;
pub const TOK_CASE: tcc_token = 272;
pub const TOK_SWITCH: tcc_token = 271;
pub const TOK_CONTINUE: tcc_token = 270;
pub const TOK_DO: tcc_token = 269;
pub const TOK_GOTO: tcc_token = 268;
pub const TOK_UNSIGNED: tcc_token = 267;
pub const TOK_STATIC: tcc_token = 266;
pub const TOK_EXTERN: tcc_token = 265;
pub const TOK_FOR: tcc_token = 264;
pub const TOK_RETURN: tcc_token = 263;
pub const TOK_BREAK: tcc_token = 262;
pub const TOK_WHILE: tcc_token = 261;
pub const TOK_ELSE: tcc_token = 260;
pub const TOK_IF: tcc_token = 259;
pub const TOK_CHAR: tcc_token = 258;
pub const TOK_VOID: tcc_token = 257;
pub const TOK_INT: tcc_token = 256;
pub const TOK_LAST: tcc_token = 255;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TinyAlloc {
    pub limit: std::os::raw::c_uint,
    pub size: std::os::raw::c_uint,
    pub buffer: *mut uint8_t,
    pub p: *mut uint8_t,
    pub nb_allocs: std::os::raw::c_uint,
    pub next: *mut TinyAlloc,
    pub top: *mut TinyAlloc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tal_header_t {
    pub size: std::os::raw::c_uint,
}
#[inline]
unsafe extern "C" fn isnum(mut c: std::os::raw::c_int) -> std::os::raw::c_int {
    return (c >= '0' as i32 && c <= '9' as i32) as std::os::raw::c_int;
}
#[inline]
unsafe extern "C" fn toup(mut c: std::os::raw::c_int) -> std::os::raw::c_int {
    return if c >= 'a' as i32 && c <= 'z' as i32 {
        (c - 'a' as i32) + 'A' as i32
    } else {
        c
    };
}
#[inline]
unsafe extern "C" fn isoct(mut c: std::os::raw::c_int) -> std::os::raw::c_int {
    return (c >= '0' as i32 && c <= '7' as i32) as std::os::raw::c_int;
}
#[inline]
unsafe extern "C" fn is_space(mut ch_0: std::os::raw::c_int) -> std::os::raw::c_int {
    return (ch_0 == ' ' as i32
        || ch_0 == '\t' as i32
        || ch_0 == '\u{b}' as i32
        || ch_0 == '\u{c}' as i32
        || ch_0 == '\r' as i32) as std::os::raw::c_int;
}
#[inline]
unsafe extern "C" fn isid(mut c: std::os::raw::c_int) -> std::os::raw::c_int {
    return (c >= 'a' as i32 && c <= 'z' as i32
        || c >= 'A' as i32 && c <= 'Z' as i32
        || c == '_' as i32) as std::os::raw::c_int;
}
/*
 *  TCC - Tiny C Compiler
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
/* *******************************************************/
/* global variables */
#[no_mangle]
pub static mut tok_flags: std::os::raw::c_int = 0;
#[no_mangle]
pub static mut parse_flags: std::os::raw::c_int = 0;
#[no_mangle]
pub static mut file: *mut BufferedFile = 0 as *const BufferedFile as *mut BufferedFile;
#[no_mangle]
pub static mut ch: std::os::raw::c_int = 0;
#[no_mangle]
pub static mut tok: std::os::raw::c_int = 0;
#[no_mangle]
pub static mut tokc: CValue = CValue {
    ld: f128::f128::ZERO,
};
#[no_mangle]
pub static mut macro_ptr: *const std::os::raw::c_int = 0 as *const std::os::raw::c_int;
#[no_mangle]
pub static mut tokcstr: CString = CString {
    size: 0,
    data: 0 as *const std::os::raw::c_void as *mut std::os::raw::c_void,
    size_allocated: 0,
};
/* current parsed string, if any */
/* display benchmark infos */
#[no_mangle]
pub static mut tok_ident: std::os::raw::c_int = 0;
#[no_mangle]
pub static mut table_ident: *mut *mut TokenSym = 0 as *const *mut TokenSym as *mut *mut TokenSym;
/* ------------------------------------------------------------------------- */
static mut hash_ident: [*mut TokenSym; 16384] = [0 as *const TokenSym as *mut TokenSym; 16384];
static mut token_buf: [std::os::raw::c_char; 1025] = [0; 1025];
static mut cstr_buf: CString = CString {
    size: 0,
    data: 0 as *const std::os::raw::c_void as *mut std::os::raw::c_void,
    size_allocated: 0,
};
static mut macro_equal_buf: CString = CString {
    size: 0,
    data: 0 as *const std::os::raw::c_void as *mut std::os::raw::c_void,
    size_allocated: 0,
};
static mut tokstr_buf: TokenString = TokenString {
    str_0: 0 as *const std::os::raw::c_int as *mut std::os::raw::c_int,
    len: 0,
    lastlen: 0,
    allocated_len: 0,
    last_line_num: 0,
    save_line_num: 0,
    prev: 0 as *const TokenString as *mut TokenString,
    prev_ptr: 0 as *const std::os::raw::c_int,
    alloc: 0,
};
static mut isidnum_table: [std::os::raw::c_uchar; 257] = [0; 257];
static mut pp_debug_tok: std::os::raw::c_int = 0;
static mut pp_debug_symv: std::os::raw::c_int = 0;
static mut pp_once: std::os::raw::c_int = 0;
static mut pp_expr: std::os::raw::c_int = 0;
static mut pp_counter: std::os::raw::c_int = 0;
static mut toksym_alloc: *mut TinyAlloc = 0 as *const TinyAlloc as *mut TinyAlloc;
static mut tokstr_alloc: *mut TinyAlloc = 0 as *const TinyAlloc as *mut TinyAlloc;
static mut macro_stack: *mut TokenString = 0 as *const TokenString as *mut TokenString;
static mut tcc_keywords: [std::os::raw::c_char; 7461] = unsafe {
    *::std::mem::transmute::<&[u8; 7461],
                                 &[std::os::raw::c_char; 7461]>(b"int\x00void\x00char\x00if\x00else\x00while\x00break\x00return\x00for\x00extern\x00static\x00unsigned\x00goto\x00do\x00continue\x00switch\x00case\x00_Atomic\x00const\x00__const\x00__const__\x00volatile\x00__volatile\x00__volatile__\x00long\x00register\x00signed\x00__signed\x00__signed__\x00auto\x00inline\x00__inline\x00__inline__\x00restrict\x00__restrict\x00__restrict__\x00__extension__\x00_Generic\x00_Static_assert\x00float\x00double\x00_Bool\x00short\x00struct\x00union\x00typedef\x00default\x00enum\x00sizeof\x00__attribute\x00__attribute__\x00__alignof\x00__alignof__\x00_Alignof\x00_Alignas\x00typeof\x00__typeof\x00__typeof__\x00__label__\x00asm\x00__asm\x00__asm__\x00define\x00include\x00include_next\x00ifdef\x00ifndef\x00elif\x00endif\x00defined\x00undef\x00error\x00warning\x00line\x00pragma\x00__LINE__\x00__FILE__\x00__DATE__\x00__TIME__\x00__FUNCTION__\x00__VA_ARGS__\x00__COUNTER__\x00__has_include\x00__func__\x00__nan__\x00__snan__\x00__inf__\x00__mzerosf\x00__mzerodf\x00section\x00__section__\x00aligned\x00__aligned__\x00packed\x00__packed__\x00weak\x00__weak__\x00alias\x00__alias__\x00unused\x00__unused__\x00cdecl\x00__cdecl\x00__cdecl__\x00stdcall\x00__stdcall\x00__stdcall__\x00fastcall\x00__fastcall\x00__fastcall__\x00regparm\x00__regparm__\x00cleanup\x00__cleanup__\x00constructor\x00__constructor__\x00destructor\x00__destructor__\x00always_inline\x00__always_inline__\x00__mode__\x00__QI__\x00__DI__\x00__HI__\x00__SI__\x00__word__\x00dllexport\x00dllimport\x00nodecorate\x00noreturn\x00__noreturn__\x00_Noreturn\x00visibility\x00__visibility__\x00__builtin_types_compatible_p\x00__builtin_choose_expr\x00__builtin_constant_p\x00__builtin_frame_address\x00__builtin_return_address\x00__builtin_expect\x00__builtin_va_arg_types\x00__atomic_store\x00__atomic_store_1\x00__atomic_store_2\x00__atomic_store_4\x00__atomic_store_8\x00__atomic_load\x00__atomic_load_1\x00__atomic_load_2\x00__atomic_load_4\x00__atomic_load_8\x00__atomic_exchange\x00__atomic_exchange_1\x00__atomic_exchange_2\x00__atomic_exchange_4\x00__atomic_exchange_8\x00__atomic_compare_exchange\x00__atomic_compare_exchange_1\x00__atomic_compare_exchange_2\x00__atomic_compare_exchange_4\x00__atomic_compare_exchange_8\x00__atomic_fetch_add\x00__atomic_fetch_add_1\x00__atomic_fetch_add_2\x00__atomic_fetch_add_4\x00__atomic_fetch_add_8\x00__atomic_fetch_sub\x00__atomic_fetch_sub_1\x00__atomic_fetch_sub_2\x00__atomic_fetch_sub_4\x00__atomic_fetch_sub_8\x00__atomic_fetch_or\x00__atomic_fetch_or_1\x00__atomic_fetch_or_2\x00__atomic_fetch_or_4\x00__atomic_fetch_or_8\x00__atomic_fetch_xor\x00__atomic_fetch_xor_1\x00__atomic_fetch_xor_2\x00__atomic_fetch_xor_4\x00__atomic_fetch_xor_8\x00__atomic_fetch_and\x00__atomic_fetch_and_1\x00__atomic_fetch_and_2\x00__atomic_fetch_and_4\x00__atomic_fetch_and_8\x00pack\x00comment\x00lib\x00push_macro\x00pop_macro\x00once\x00option\x00memcpy\x00memmove\x00memset\x00__divdi3\x00__moddi3\x00__udivdi3\x00__umoddi3\x00__ashrdi3\x00__lshrdi3\x00__ashldi3\x00__floatundisf\x00__floatundidf\x00__floatundixf\x00__fixunsxfdi\x00__fixunssfdi\x00__fixunsdfdi\x00alloca\x00__bound_ptr_add\x00__bound_ptr_indir1\x00__bound_ptr_indir2\x00__bound_ptr_indir4\x00__bound_ptr_indir8\x00__bound_ptr_indir12\x00__bound_ptr_indir16\x00__bound_main_arg\x00__bound_local_new\x00__bound_local_delete\x00__bound_setjmp\x00__bound_longjmp\x00__bound_new_region\x00sigsetjmp\x00__sigsetjmp\x00siglongjmp\x00setjmp\x00_setjmp\x00longjmp\x00.byte\x00.word\x00.align\x00.balign\x00.p2align\x00.set\x00.skip\x00.space\x00.string\x00.asciz\x00.ascii\x00.file\x00.globl\x00.global\x00.weak\x00.hidden\x00.ident\x00.size\x00.type\x00.text\x00.data\x00.bss\x00.previous\x00.pushsection\x00.popsection\x00.fill\x00.rept\x00.endr\x00.org\x00.quad\x00.code64\x00.short\x00.long\x00.int\x00.section\x00al\x00cl\x00dl\x00bl\x00ah\x00ch\x00dh\x00bh\x00ax\x00cx\x00dx\x00bx\x00sp\x00bp\x00si\x00di\x00eax\x00ecx\x00edx\x00ebx\x00esp\x00ebp\x00esi\x00edi\x00rax\x00rcx\x00rdx\x00rbx\x00rsp\x00rbp\x00rsi\x00rdi\x00mm0\x00mm1\x00mm2\x00mm3\x00mm4\x00mm5\x00mm6\x00mm7\x00xmm0\x00xmm1\x00xmm2\x00xmm3\x00xmm4\x00xmm5\x00xmm6\x00xmm7\x00cr0\x00cr1\x00cr2\x00cr3\x00cr4\x00cr5\x00cr6\x00cr7\x00tr0\x00tr1\x00tr2\x00tr3\x00tr4\x00tr5\x00tr6\x00tr7\x00db0\x00db1\x00db2\x00db3\x00db4\x00db5\x00db6\x00db7\x00dr0\x00dr1\x00dr2\x00dr3\x00dr4\x00dr5\x00dr6\x00dr7\x00es\x00cs\x00ss\x00ds\x00fs\x00gs\x00st\x00rip\x00spl\x00bpl\x00sil\x00dil\x00movb\x00movw\x00movl\x00movq\x00mov\x00addb\x00addw\x00addl\x00addq\x00add\x00orb\x00orw\x00orl\x00orq\x00or\x00adcb\x00adcw\x00adcl\x00adcq\x00adc\x00sbbb\x00sbbw\x00sbbl\x00sbbq\x00sbb\x00andb\x00andw\x00andl\x00andq\x00and\x00subb\x00subw\x00subl\x00subq\x00sub\x00xorb\x00xorw\x00xorl\x00xorq\x00xor\x00cmpb\x00cmpw\x00cmpl\x00cmpq\x00cmp\x00incb\x00incw\x00incl\x00incq\x00inc\x00decb\x00decw\x00decl\x00decq\x00dec\x00notb\x00notw\x00notl\x00notq\x00not\x00negb\x00negw\x00negl\x00negq\x00neg\x00mulb\x00mulw\x00mull\x00mulq\x00mul\x00imulb\x00imulw\x00imull\x00imulq\x00imul\x00divb\x00divw\x00divl\x00divq\x00div\x00idivb\x00idivw\x00idivl\x00idivq\x00idiv\x00xchgb\x00xchgw\x00xchgl\x00xchgq\x00xchg\x00testb\x00testw\x00testl\x00testq\x00test\x00rolb\x00rolw\x00roll\x00rolq\x00rol\x00rorb\x00rorw\x00rorl\x00rorq\x00ror\x00rclb\x00rclw\x00rcll\x00rclq\x00rcl\x00rcrb\x00rcrw\x00rcrl\x00rcrq\x00rcr\x00shlb\x00shlw\x00shll\x00shlq\x00shl\x00shrb\x00shrw\x00shrl\x00shrq\x00shr\x00sarb\x00sarw\x00sarl\x00sarq\x00sar\x00shldw\x00shldl\x00shldq\x00shld\x00shrdw\x00shrdl\x00shrdq\x00shrd\x00pushw\x00pushl\x00pushq\x00push\x00popw\x00popl\x00popq\x00pop\x00inb\x00inw\x00inl\x00in\x00outb\x00outw\x00outl\x00out\x00movzbw\x00movzbl\x00movzbq\x00movzb\x00movzwl\x00movsbw\x00movsbl\x00movswl\x00movsbq\x00movswq\x00movzwq\x00movslq\x00leaw\x00leal\x00leaq\x00lea\x00les\x00lds\x00lss\x00lfs\x00lgs\x00call\x00jmp\x00lcall\x00ljmp\x00jo\x00jno\x00jb\x00jc\x00jnae\x00jnb\x00jnc\x00jae\x00je\x00jz\x00jne\x00jnz\x00jbe\x00jna\x00jnbe\x00ja\x00js\x00jns\x00jp\x00jpe\x00jnp\x00jpo\x00jl\x00jnge\x00jnl\x00jge\x00jle\x00jng\x00jnle\x00jg\x00seto\x00setno\x00setb\x00setc\x00setnae\x00setnb\x00setnc\x00setae\x00sete\x00setz\x00setne\x00setnz\x00setbe\x00setna\x00setnbe\x00seta\x00sets\x00setns\x00setp\x00setpe\x00setnp\x00setpo\x00setl\x00setnge\x00setnl\x00setge\x00setle\x00setng\x00setnle\x00setg\x00setob\x00setnob\x00setbb\x00setcb\x00setnaeb\x00setnbb\x00setncb\x00setaeb\x00seteb\x00setzb\x00setneb\x00setnzb\x00setbeb\x00setnab\x00setnbeb\x00setab\x00setsb\x00setnsb\x00setpb\x00setpeb\x00setnpb\x00setpob\x00setlb\x00setngeb\x00setnlb\x00setgeb\x00setleb\x00setngb\x00setnleb\x00setgb\x00cmovo\x00cmovno\x00cmovb\x00cmovc\x00cmovnae\x00cmovnb\x00cmovnc\x00cmovae\x00cmove\x00cmovz\x00cmovne\x00cmovnz\x00cmovbe\x00cmovna\x00cmovnbe\x00cmova\x00cmovs\x00cmovns\x00cmovp\x00cmovpe\x00cmovnp\x00cmovpo\x00cmovl\x00cmovnge\x00cmovnl\x00cmovge\x00cmovle\x00cmovng\x00cmovnle\x00cmovg\x00bsfw\x00bsfl\x00bsfq\x00bsf\x00bsrw\x00bsrl\x00bsrq\x00bsr\x00btw\x00btl\x00btq\x00bt\x00btsw\x00btsl\x00btsq\x00bts\x00btrw\x00btrl\x00btrq\x00btr\x00btcw\x00btcl\x00btcq\x00btc\x00larw\x00larl\x00larq\x00lar\x00lslw\x00lsll\x00lslq\x00lsl\x00fadd\x00faddp\x00fadds\x00fiaddl\x00faddl\x00fiadds\x00fmul\x00fmulp\x00fmuls\x00fimull\x00fmull\x00fimuls\x00fcom\x00fcom_1\x00fcoms\x00ficoml\x00fcoml\x00ficoms\x00fcomp\x00fcompp\x00fcomps\x00ficompl\x00fcompl\x00ficomps\x00fsub\x00fsubp\x00fsubs\x00fisubl\x00fsubl\x00fisubs\x00fsubr\x00fsubrp\x00fsubrs\x00fisubrl\x00fsubrl\x00fisubrs\x00fdiv\x00fdivp\x00fdivs\x00fidivl\x00fdivl\x00fidivs\x00fdivr\x00fdivrp\x00fdivrs\x00fidivrl\x00fdivrl\x00fidivrs\x00xaddb\x00xaddw\x00xaddl\x00xaddq\x00xadd\x00cmpxchgb\x00cmpxchgw\x00cmpxchgl\x00cmpxchgq\x00cmpxchg\x00cmpsb\x00cmpsw\x00cmpsl\x00cmpsq\x00cmps\x00scmpb\x00scmpw\x00scmpl\x00scmpq\x00scmp\x00insb\x00insw\x00insl\x00ins\x00outsb\x00outsw\x00outsl\x00outs\x00lodsb\x00lodsw\x00lodsl\x00lodsq\x00lods\x00slodb\x00slodw\x00slodl\x00slodq\x00slod\x00movsb\x00movsw\x00movsl\x00movsq\x00movs\x00smovb\x00smovw\x00smovl\x00smovq\x00smov\x00scasb\x00scasw\x00scasl\x00scasq\x00scas\x00sscab\x00sscaw\x00sscal\x00sscaq\x00ssca\x00stosb\x00stosw\x00stosl\x00stosq\x00stos\x00sstob\x00sstow\x00sstol\x00sstoq\x00ssto\x00clc\x00cld\x00cli\x00clts\x00cmc\x00lahf\x00sahf\x00pushfq\x00popfq\x00pushf\x00popf\x00stc\x00std\x00sti\x00aaa\x00aas\x00daa\x00das\x00aad\x00aam\x00cbw\x00cwd\x00cwde\x00cdq\x00cbtw\x00cwtl\x00cwtd\x00cltd\x00cqto\x00int3\x00into\x00iret\x00rsm\x00hlt\x00wait\x00nop\x00pause\x00xlat\x00lock\x00rep\x00repe\x00repz\x00repne\x00repnz\x00invd\x00wbinvd\x00cpuid\x00wrmsr\x00rdtsc\x00rdmsr\x00rdpmc\x00syscall\x00sysret\x00ud2\x00leave\x00ret\x00retq\x00lret\x00fucompp\x00ftst\x00fxam\x00fld1\x00fldl2t\x00fldl2e\x00fldpi\x00fldlg2\x00fldln2\x00fldz\x00f2xm1\x00fyl2x\x00fptan\x00fpatan\x00fxtract\x00fprem1\x00fdecstp\x00fincstp\x00fprem\x00fyl2xp1\x00fsqrt\x00fsincos\x00frndint\x00fscale\x00fsin\x00fcos\x00fchs\x00fabs\x00fninit\x00fnclex\x00fnop\x00fwait\x00fxch\x00fnstsw\x00emms\x00sysretq\x00ljmpw\x00ljmpl\x00enter\x00loopne\x00loopnz\x00loope\x00loopz\x00loop\x00jecxz\x00fld\x00fldl\x00flds\x00fildl\x00fildq\x00fildll\x00fldt\x00fbld\x00fst\x00fstl\x00fsts\x00fstps\x00fstpl\x00fist\x00fistp\x00fistl\x00fistpl\x00fstp\x00fistpq\x00fistpll\x00fstpt\x00fbstp\x00fucom\x00fucomp\x00finit\x00fldcw\x00fnstcw\x00fstcw\x00fstsw\x00fclex\x00fnstenv\x00fstenv\x00fldenv\x00fnsave\x00fsave\x00frstor\x00ffree\x00ffreep\x00fxsave\x00fxrstor\x00fxsaveq\x00fxrstorq\x00arpl\x00lgdt\x00lgdtq\x00lidt\x00lidtq\x00lldt\x00lmsw\x00ltr\x00sgdt\x00sgdtq\x00sidt\x00sidtq\x00sldt\x00smsw\x00str\x00verr\x00verw\x00swapgs\x00bswap\x00bswapl\x00bswapq\x00invlpg\x00cmpxchg8b\x00cmpxchg16b\x00fcmovb\x00fcmove\x00fcmovbe\x00fcmovu\x00fcmovnb\x00fcmovne\x00fcmovnbe\x00fcmovnu\x00fucomi\x00fcomi\x00fucomip\x00fcomip\x00movd\x00packssdw\x00packsswb\x00packuswb\x00paddb\x00paddw\x00paddd\x00paddsb\x00paddsw\x00paddusb\x00paddusw\x00pand\x00pandn\x00pcmpeqb\x00pcmpeqw\x00pcmpeqd\x00pcmpgtb\x00pcmpgtw\x00pcmpgtd\x00pmaddwd\x00pmulhw\x00pmullw\x00por\x00psllw\x00pslld\x00psllq\x00psraw\x00psrad\x00psrlw\x00psrld\x00psrlq\x00psubb\x00psubw\x00psubd\x00psubsb\x00psubsw\x00psubusb\x00psubusw\x00punpckhbw\x00punpckhwd\x00punpckhdq\x00punpcklbw\x00punpcklwd\x00punpckldq\x00pxor\x00movups\x00movaps\x00movhps\x00addps\x00cvtpi2ps\x00cvtps2pi\x00cvttps2pi\x00divps\x00maxps\x00minps\x00mulps\x00pavgb\x00pavgw\x00pmaxsw\x00pmaxub\x00pminsw\x00pminub\x00rcpss\x00rsqrtps\x00sqrtps\x00subps\x00prefetchnta\x00prefetcht0\x00prefetcht1\x00prefetcht2\x00prefetchw\x00lfence\x00mfence\x00sfence\x00clflush\x00\x00")
};
/* WARNING: the content of this string encodes token numbers */
static mut tok_two_chars: [std::os::raw::c_uchar; 67] = [
    '<' as i32 as std::os::raw::c_uchar,
    '=' as i32 as std::os::raw::c_uchar,
    0x9e as std::os::raw::c_int as std::os::raw::c_uchar,
    '>' as i32 as std::os::raw::c_uchar,
    '=' as i32 as std::os::raw::c_uchar,
    0x9d as std::os::raw::c_int as std::os::raw::c_uchar,
    '!' as i32 as std::os::raw::c_uchar,
    '=' as i32 as std::os::raw::c_uchar,
    0x95 as std::os::raw::c_int as std::os::raw::c_uchar,
    '&' as i32 as std::os::raw::c_uchar,
    '&' as i32 as std::os::raw::c_uchar,
    0x90 as std::os::raw::c_int as std::os::raw::c_uchar,
    '|' as i32 as std::os::raw::c_uchar,
    '|' as i32 as std::os::raw::c_uchar,
    0x91 as std::os::raw::c_int as std::os::raw::c_uchar,
    '+' as i32 as std::os::raw::c_uchar,
    '+' as i32 as std::os::raw::c_uchar,
    0x82 as std::os::raw::c_int as std::os::raw::c_uchar,
    '-' as i32 as std::os::raw::c_uchar,
    '-' as i32 as std::os::raw::c_uchar,
    0x80 as std::os::raw::c_int as std::os::raw::c_uchar,
    '=' as i32 as std::os::raw::c_uchar,
    '=' as i32 as std::os::raw::c_uchar,
    0x94 as std::os::raw::c_int as std::os::raw::c_uchar,
    '<' as i32 as std::os::raw::c_uchar,
    '<' as i32 as std::os::raw::c_uchar,
    '<' as i32 as std::os::raw::c_uchar,
    '>' as i32 as std::os::raw::c_uchar,
    '>' as i32 as std::os::raw::c_uchar,
    '>' as i32 as std::os::raw::c_uchar,
    '+' as i32 as std::os::raw::c_uchar,
    '=' as i32 as std::os::raw::c_uchar,
    0xb0 as std::os::raw::c_int as std::os::raw::c_uchar,
    '-' as i32 as std::os::raw::c_uchar,
    '=' as i32 as std::os::raw::c_uchar,
    0xb1 as std::os::raw::c_int as std::os::raw::c_uchar,
    '*' as i32 as std::os::raw::c_uchar,
    '=' as i32 as std::os::raw::c_uchar,
    0xb2 as std::os::raw::c_int as std::os::raw::c_uchar,
    '/' as i32 as std::os::raw::c_uchar,
    '=' as i32 as std::os::raw::c_uchar,
    0xb3 as std::os::raw::c_int as std::os::raw::c_uchar,
    '%' as i32 as std::os::raw::c_uchar,
    '=' as i32 as std::os::raw::c_uchar,
    0xb4 as std::os::raw::c_int as std::os::raw::c_uchar,
    '&' as i32 as std::os::raw::c_uchar,
    '=' as i32 as std::os::raw::c_uchar,
    0xb5 as std::os::raw::c_int as std::os::raw::c_uchar,
    '^' as i32 as std::os::raw::c_uchar,
    '=' as i32 as std::os::raw::c_uchar,
    0xb7 as std::os::raw::c_int as std::os::raw::c_uchar,
    '|' as i32 as std::os::raw::c_uchar,
    '=' as i32 as std::os::raw::c_uchar,
    0xb6 as std::os::raw::c_int as std::os::raw::c_uchar,
    '-' as i32 as std::os::raw::c_uchar,
    '>' as i32 as std::os::raw::c_uchar,
    0xa0 as std::os::raw::c_int as std::os::raw::c_uchar,
    '.' as i32 as std::os::raw::c_uchar,
    '.' as i32 as std::os::raw::c_uchar,
    0xa2 as std::os::raw::c_int as std::os::raw::c_uchar,
    '#' as i32 as std::os::raw::c_uchar,
    '#' as i32 as std::os::raw::c_uchar,
    0xa3 as std::os::raw::c_int as std::os::raw::c_uchar,
    '#' as i32 as std::os::raw::c_uchar,
    '#' as i32 as std::os::raw::c_uchar,
    0xa6 as std::os::raw::c_int as std::os::raw::c_uchar,
    0 as std::os::raw::c_int as std::os::raw::c_uchar,
];
#[no_mangle]
pub unsafe extern "C" fn skip(mut c: std::os::raw::c_int) {
    if tok != c {
        _tcc_error(
            b"\'%c\' expected (got \"%s\")\x00" as *const u8 as *const std::os::raw::c_char,
            c,
            get_tok_str(tok, &mut tokc),
        );
    }
    next();
}
#[no_mangle]
pub unsafe extern "C" fn expect(mut msg: *const std::os::raw::c_char) -> ! {
    _tcc_error(
        b"%s expected\x00" as *const u8 as *const std::os::raw::c_char,
        msg,
    );
}
#[no_mangle]
pub unsafe extern "C" fn expect_arg(mut msg: *const std::os::raw::c_char, mut arg: size_t) -> ! {
    _tcc_error(
        b"%s expected as arg #%zu\x00" as *const u8 as *const std::os::raw::c_char,
        msg,
        arg,
    );
}
/* ------------------------------------------------------------------------- */
unsafe extern "C" fn tal_new(
    mut pal: *mut *mut TinyAlloc,
    mut limit: std::os::raw::c_uint,
    mut size: std::os::raw::c_uint,
) -> *mut TinyAlloc {
    let mut al: *mut TinyAlloc =
        tcc_mallocz(::std::mem::size_of::<TinyAlloc>() as std::os::raw::c_ulong) as *mut TinyAlloc;
    (*al).buffer = tcc_malloc(size as std::os::raw::c_ulong) as *mut uint8_t;
    (*al).p = (*al).buffer;
    (*al).limit = limit;
    (*al).size = size;
    if !pal.is_null() {
        *pal = al
    }
    return al;
}
unsafe extern "C" fn tal_delete(mut al: *mut TinyAlloc) {
    let mut next_0: *mut TinyAlloc = 0 as *mut TinyAlloc;
    loop {
        if al.is_null() {
            return;
        }
        next_0 = (*al).next;
        tcc_free((*al).buffer as *mut std::os::raw::c_void);
        tcc_free(al as *mut std::os::raw::c_void);
        al = next_0
    }
}
unsafe extern "C" fn tal_free_impl(mut al: *mut TinyAlloc, mut p: *mut std::os::raw::c_void) {
    if p.is_null() {
        return;
    }
    loop {
        if (*al).buffer <= p as *mut uint8_t
            && (p as *mut uint8_t) < (*al).buffer.offset((*al).size as isize)
        {
            (*al).nb_allocs = (*al).nb_allocs.wrapping_sub(1);
            if (*al).nb_allocs == 0 {
                (*al).p = (*al).buffer
            }
            break;
        } else if !(*al).next.is_null() {
            al = (*al).next
        } else {
            tcc_free(p);
            break;
        }
    }
}
unsafe extern "C" fn tal_realloc_impl(
    mut pal: *mut *mut TinyAlloc,
    mut p: *mut std::os::raw::c_void,
    mut size: std::os::raw::c_uint,
) -> *mut std::os::raw::c_void {
    let mut header: *mut tal_header_t = 0 as *mut tal_header_t;
    let mut ret: *mut std::os::raw::c_void = 0 as *mut std::os::raw::c_void;
    let mut is_own: std::os::raw::c_int = 0;
    let mut adj_size: std::os::raw::c_uint = size
        .wrapping_add(3 as std::os::raw::c_int as std::os::raw::c_uint)
        & -(4 as std::os::raw::c_int) as std::os::raw::c_uint;
    let mut al: *mut TinyAlloc = *pal;
    loop {
        is_own = ((*al).buffer <= p as *mut uint8_t
            && (p as *mut uint8_t) < (*al).buffer.offset((*al).size as isize))
            as std::os::raw::c_int;
        if (p.is_null() || is_own != 0) && size <= (*al).limit {
            if (((*al).p.offset_from((*al).buffer) as std::os::raw::c_long
                + adj_size as std::os::raw::c_long) as std::os::raw::c_ulong)
                .wrapping_add(::std::mem::size_of::<tal_header_t>() as std::os::raw::c_ulong)
                < (*al).size as std::os::raw::c_ulong
            {
                header = (*al).p as *mut tal_header_t;
                (*header).size = adj_size;
                ret = (*al)
                    .p
                    .offset(::std::mem::size_of::<tal_header_t>() as std::os::raw::c_ulong as isize)
                    as *mut std::os::raw::c_void;
                (*al).p =
                    (*al).p.offset(
                        (adj_size as std::os::raw::c_ulong)
                            .wrapping_add(
                                ::std::mem::size_of::<tal_header_t>() as std::os::raw::c_ulong
                            ) as isize,
                    );
                if is_own != 0 {
                    header = (p as *mut tal_header_t).offset(-(1 as std::os::raw::c_int as isize));
                    if !p.is_null() {
                        memcpy(ret, p, (*header).size as std::os::raw::c_ulong);
                    }
                } else {
                    (*al).nb_allocs = (*al).nb_allocs.wrapping_add(1)
                }
                return ret;
            } else {
                if is_own != 0 {
                    (*al).nb_allocs = (*al).nb_allocs.wrapping_sub(1);
                    ret = tal_realloc_impl(pal, 0 as *mut std::os::raw::c_void, size);
                    header = (p as *mut tal_header_t).offset(-(1 as std::os::raw::c_int as isize));
                    if !p.is_null() {
                        memcpy(ret, p, (*header).size as std::os::raw::c_ulong);
                    }
                    return ret;
                }
            }
            if !(*al).next.is_null() {
                al = (*al).next
            } else {
                let mut bottom: *mut TinyAlloc = al;
                let mut next_0: *mut TinyAlloc = if !(*al).top.is_null() { (*al).top } else { al };
                al = tal_new(
                    pal,
                    (*next_0).limit,
                    (*next_0)
                        .size
                        .wrapping_mul(2 as std::os::raw::c_int as std::os::raw::c_uint),
                );
                (*al).next = next_0;
                (*bottom).top = al
            }
        } else if is_own != 0 {
            (*al).nb_allocs = (*al).nb_allocs.wrapping_sub(1);
            ret = tcc_malloc(size as std::os::raw::c_ulong);
            header = (p as *mut tal_header_t).offset(-(1 as std::os::raw::c_int as isize));
            if !p.is_null() {
                memcpy(ret, p, (*header).size as std::os::raw::c_ulong);
            }
            break;
        } else if !(*al).next.is_null() {
            al = (*al).next
        } else {
            ret = tcc_realloc(p, size as std::os::raw::c_ulong);
            break;
        }
    }
    return ret;
}
/* USE_TAL */
/* ------------------------------------------------------------------------- */
/* CString handling */
unsafe extern "C" fn cstr_realloc(mut cstr: *mut CString, mut new_size: std::os::raw::c_int) {
    let mut size: std::os::raw::c_int = 0; /* no need to allocate a too small first string */
    size = (*cstr).size_allocated;
    if size < 8 as std::os::raw::c_int {
        size = 8 as std::os::raw::c_int
    }
    while size < new_size {
        size = size * 2 as std::os::raw::c_int
    }
    (*cstr).data = tcc_realloc((*cstr).data, size as std::os::raw::c_ulong);
    (*cstr).size_allocated = size;
}
/* add a byte */
#[no_mangle]
pub unsafe extern "C" fn cstr_ccat(mut cstr: *mut CString, mut ch_0: std::os::raw::c_int) {
    let mut size: std::os::raw::c_int = 0;
    size = (*cstr).size + 1 as std::os::raw::c_int;
    if size > (*cstr).size_allocated {
        cstr_realloc(cstr, size);
    }
    *((*cstr).data as *mut std::os::raw::c_uchar)
        .offset((size - 1 as std::os::raw::c_int) as isize) = ch_0 as std::os::raw::c_uchar;
    (*cstr).size = size;
}
#[no_mangle]
pub unsafe extern "C" fn unicode_to_utf8(
    mut b: *mut std::os::raw::c_char,
    mut Uc: uint32_t,
) -> *mut std::os::raw::c_char {
    if Uc < 0x80 as std::os::raw::c_int as std::os::raw::c_uint {
        let fresh0 = b;
        b = b.offset(1);
        *fresh0 = Uc as std::os::raw::c_char
    } else if Uc < 0x800 as std::os::raw::c_int as std::os::raw::c_uint {
        let fresh1 = b;
        b = b.offset(1);
        *fresh1 = (192 as std::os::raw::c_int as std::os::raw::c_uint)
            .wrapping_add(Uc.wrapping_div(64 as std::os::raw::c_int as std::os::raw::c_uint))
            as std::os::raw::c_char;
        let fresh2 = b;
        b = b.offset(1);
        *fresh2 = (128 as std::os::raw::c_int as std::os::raw::c_uint)
            .wrapping_add(Uc.wrapping_rem(64 as std::os::raw::c_int as std::os::raw::c_uint))
            as std::os::raw::c_char
    } else if Uc.wrapping_sub(0xd800 as std::os::raw::c_uint)
        < 0x800 as std::os::raw::c_int as std::os::raw::c_uint
    {
        return b;
    } else {
        if Uc < 0x10000 as std::os::raw::c_int as std::os::raw::c_uint {
            let fresh3 = b;
            b = b.offset(1);
            *fresh3 = (224 as std::os::raw::c_int as std::os::raw::c_uint)
                .wrapping_add(Uc.wrapping_div(4096 as std::os::raw::c_int as std::os::raw::c_uint))
                as std::os::raw::c_char;
            let fresh4 = b;
            b = b.offset(1);
            *fresh4 = (128 as std::os::raw::c_int as std::os::raw::c_uint).wrapping_add(
                Uc.wrapping_div(64 as std::os::raw::c_int as std::os::raw::c_uint)
                    .wrapping_rem(64 as std::os::raw::c_int as std::os::raw::c_uint),
            ) as std::os::raw::c_char;
            let fresh5 = b;
            b = b.offset(1);
            *fresh5 = (128 as std::os::raw::c_int as std::os::raw::c_uint)
                .wrapping_add(Uc.wrapping_rem(64 as std::os::raw::c_int as std::os::raw::c_uint))
                as std::os::raw::c_char
        } else if Uc < 0x110000 as std::os::raw::c_int as std::os::raw::c_uint {
            let fresh6 = b;
            b = b.offset(1);
            *fresh6 = (240 as std::os::raw::c_int as std::os::raw::c_uint).wrapping_add(
                Uc.wrapping_div(262144 as std::os::raw::c_int as std::os::raw::c_uint),
            ) as std::os::raw::c_char;
            let fresh7 = b;
            b = b.offset(1);
            *fresh7 = (128 as std::os::raw::c_int as std::os::raw::c_uint).wrapping_add(
                Uc.wrapping_div(4096 as std::os::raw::c_int as std::os::raw::c_uint)
                    .wrapping_rem(64 as std::os::raw::c_int as std::os::raw::c_uint),
            ) as std::os::raw::c_char;
            let fresh8 = b;
            b = b.offset(1);
            *fresh8 = (128 as std::os::raw::c_int as std::os::raw::c_uint).wrapping_add(
                Uc.wrapping_div(64 as std::os::raw::c_int as std::os::raw::c_uint)
                    .wrapping_rem(64 as std::os::raw::c_int as std::os::raw::c_uint),
            ) as std::os::raw::c_char;
            let fresh9 = b;
            b = b.offset(1);
            *fresh9 = (128 as std::os::raw::c_int as std::os::raw::c_uint)
                .wrapping_add(Uc.wrapping_rem(64 as std::os::raw::c_int as std::os::raw::c_uint))
                as std::os::raw::c_char
        }
    }
    return b;
}
/* add a unicode character expanded into utf8 */
#[no_mangle]
pub unsafe extern "C" fn cstr_u8cat(mut cstr: *mut CString, mut ch_0: std::os::raw::c_int) {
    let mut buf: [std::os::raw::c_char; 4] = [0; 4];
    let mut e: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    e = unicode_to_utf8(buf.as_mut_ptr(), ch_0 as uint32_t);
    cstr_cat(
        cstr,
        buf.as_mut_ptr(),
        e.offset_from(buf.as_mut_ptr()) as std::os::raw::c_long as std::os::raw::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cstr_cat(
    mut cstr: *mut CString,
    mut str: *const std::os::raw::c_char,
    mut len: std::os::raw::c_int,
) {
    let mut size: std::os::raw::c_int = 0;
    if len <= 0 as std::os::raw::c_int {
        len = strlen(str)
            .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong)
            .wrapping_add(len as std::os::raw::c_ulong) as std::os::raw::c_int
    }
    size = (*cstr).size + len;
    if size > (*cstr).size_allocated {
        cstr_realloc(cstr, size);
    }
    memmove(
        ((*cstr).data as *mut std::os::raw::c_uchar).offset((*cstr).size as isize)
            as *mut std::os::raw::c_void,
        str as *const std::os::raw::c_void,
        len as std::os::raw::c_ulong,
    );
    (*cstr).size = size;
}
/* add a wide char */
#[no_mangle]
pub unsafe extern "C" fn cstr_wccat(mut cstr: *mut CString, mut ch_0: std::os::raw::c_int) {
    let mut size: std::os::raw::c_int = 0;
    size = ((*cstr).size as std::os::raw::c_ulong)
        .wrapping_add(::std::mem::size_of::<nwchar_t>() as std::os::raw::c_ulong)
        as std::os::raw::c_int;
    if size > (*cstr).size_allocated {
        cstr_realloc(cstr, size);
    }
    *(((*cstr).data as *mut std::os::raw::c_uchar)
        .offset(size as isize)
        .offset(-(::std::mem::size_of::<nwchar_t>() as std::os::raw::c_ulong as isize))
        as *mut nwchar_t) = ch_0;
    (*cstr).size = size;
}
#[no_mangle]
pub unsafe extern "C" fn cstr_new(mut cstr: *mut CString) {
    memset(
        cstr as *mut std::os::raw::c_void,
        0 as std::os::raw::c_int,
        ::std::mem::size_of::<CString>() as std::os::raw::c_ulong,
    );
}
/* free string and reset it to NULL */
#[no_mangle]
pub unsafe extern "C" fn cstr_free(mut cstr: *mut CString) {
    tcc_free((*cstr).data);
    cstr_new(cstr);
}
/* reset string to empty */
#[no_mangle]
pub unsafe extern "C" fn cstr_reset(mut cstr: *mut CString) {
    (*cstr).size = 0 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn cstr_printf(
    mut cstr: *mut CString,
    mut fmt: *const std::os::raw::c_char,
    mut args: ...
) -> std::os::raw::c_int {
    let mut v: ::std::ffi::VaListImpl;
    let mut len: std::os::raw::c_int = 0;
    let mut size: std::os::raw::c_int = 80 as std::os::raw::c_int;
    loop {
        size += (*cstr).size;
        if size > (*cstr).size_allocated {
            cstr_realloc(cstr, size);
        }
        size = (*cstr).size_allocated - (*cstr).size;
        v = args.clone();
        len = vsnprintf(
            ((*cstr).data as *mut std::os::raw::c_char).offset((*cstr).size as isize),
            size as std::os::raw::c_ulong,
            fmt,
            v.as_va_list(),
        );
        if len > 0 as std::os::raw::c_int && len < size {
            break;
        }
        size *= 2 as std::os::raw::c_int
    }
    (*cstr).size += len;
    return len;
}
/* XXX: unicode ? */
unsafe extern "C" fn add_char(mut cstr: *mut CString, mut c: std::os::raw::c_int) {
    if c == '\'' as i32 || c == '\"' as i32 || c == '\\' as i32 {
        /* XXX: could be more precise if char or string */
        cstr_ccat(cstr, '\\' as i32);
    }
    if c >= 32 as std::os::raw::c_int && c <= 126 as std::os::raw::c_int {
        cstr_ccat(cstr, c);
    } else {
        cstr_ccat(cstr, '\\' as i32);
        if c == '\n' as i32 {
            cstr_ccat(cstr, 'n' as i32);
        } else {
            cstr_ccat(
                cstr,
                '0' as i32 + (c >> 6 as std::os::raw::c_int & 7 as std::os::raw::c_int),
            );
            cstr_ccat(
                cstr,
                '0' as i32 + (c >> 3 as std::os::raw::c_int & 7 as std::os::raw::c_int),
            );
            cstr_ccat(cstr, '0' as i32 + (c & 7 as std::os::raw::c_int));
        }
    };
}
/* ------------------------------------------------------------------------- */
/* allocate a new token */
unsafe extern "C" fn tok_alloc_new(
    mut pts: *mut *mut TokenSym,
    mut str: *const std::os::raw::c_char,
    mut len: std::os::raw::c_int,
) -> *mut TokenSym {
    let mut ts: *mut TokenSym = 0 as *mut TokenSym;
    let mut ptable: *mut *mut TokenSym = 0 as *mut *mut TokenSym;
    let mut i: std::os::raw::c_int = 0;
    if tok_ident >= 0x10000000 as std::os::raw::c_int {
        _tcc_error(b"memory full (symbols)\x00" as *const u8 as *const std::os::raw::c_char);
    }
    /* expand token table if needed */
    i = tok_ident - 256 as std::os::raw::c_int;
    if i % 512 as std::os::raw::c_int == 0 as std::os::raw::c_int {
        ptable = tcc_realloc(
            table_ident as *mut std::os::raw::c_void,
            ((i + 512 as std::os::raw::c_int) as std::os::raw::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut TokenSym>() as std::os::raw::c_ulong),
        ) as *mut *mut TokenSym;
        table_ident = ptable
    }
    ts = tal_realloc_impl(
        &mut toksym_alloc,
        0 as *mut std::os::raw::c_void,
        (::std::mem::size_of::<TokenSym>() as std::os::raw::c_ulong)
            .wrapping_add(len as std::os::raw::c_ulong) as std::os::raw::c_uint,
    ) as *mut TokenSym;
    let ref mut fresh10 = *table_ident.offset(i as isize);
    *fresh10 = ts;
    let fresh11 = tok_ident;
    tok_ident = tok_ident + 1;
    (*ts).tok = fresh11;
    (*ts).sym_define = 0 as *mut Sym;
    (*ts).sym_label = 0 as *mut Sym;
    (*ts).sym_struct = 0 as *mut Sym;
    (*ts).sym_identifier = 0 as *mut Sym;
    (*ts).len = len;
    (*ts).hash_next = 0 as *mut TokenSym;
    memcpy(
        (*ts).str_0.as_mut_ptr() as *mut std::os::raw::c_void,
        str as *const std::os::raw::c_void,
        len as std::os::raw::c_ulong,
    );
    *(*ts).str_0.as_mut_ptr().offset(len as isize) = '\u{0}' as i32 as std::os::raw::c_char;
    *pts = ts;
    return ts;
}
/* find a token and add it if not found */
#[no_mangle]
pub unsafe extern "C" fn tok_alloc(
    mut str: *const std::os::raw::c_char,
    mut len: std::os::raw::c_int,
) -> *mut TokenSym {
    let mut ts: *mut TokenSym = 0 as *mut TokenSym;
    let mut pts: *mut *mut TokenSym = 0 as *mut *mut TokenSym;
    let mut i: std::os::raw::c_int = 0;
    let mut h: std::os::raw::c_uint = 0;
    h = 1 as std::os::raw::c_int as std::os::raw::c_uint;
    i = 0 as std::os::raw::c_int;
    while i < len {
        h = h
            .wrapping_add(h << 5 as std::os::raw::c_int)
            .wrapping_add(h >> 27 as std::os::raw::c_int)
            .wrapping_add(
                *(str as *mut std::os::raw::c_uchar).offset(i as isize) as std::os::raw::c_uint
            );
        i += 1
    }
    h &= (16384 as std::os::raw::c_int - 1 as std::os::raw::c_int) as std::os::raw::c_uint;
    pts = &mut *hash_ident.as_mut_ptr().offset(h as isize) as *mut *mut TokenSym;
    loop {
        ts = *pts;
        if ts.is_null() {
            break;
        }
        if (*ts).len == len
            && memcmp(
                (*ts).str_0.as_mut_ptr() as *const std::os::raw::c_void,
                str as *const std::os::raw::c_void,
                len as std::os::raw::c_ulong,
            ) == 0
        {
            return ts;
        }
        pts = &mut (*ts).hash_next
    }
    return tok_alloc_new(pts, str, len);
}
#[no_mangle]
pub unsafe extern "C" fn tok_alloc_const(
    mut str: *const std::os::raw::c_char,
) -> std::os::raw::c_int {
    return (*tok_alloc(str, strlen(str) as std::os::raw::c_int)).tok;
}
/* XXX: buffer overflow */
/* XXX: float tokens */
#[no_mangle]
pub unsafe extern "C" fn get_tok_str(
    mut v: std::os::raw::c_int,
    mut cv: *mut CValue,
) -> *const std::os::raw::c_char {
    let mut q: *const std::os::raw::c_uchar = 0 as *const std::os::raw::c_uchar;
    let mut current_block: u64;
    let mut p: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut i: std::os::raw::c_int = 0;
    let mut len: std::os::raw::c_int = 0;
    cstr_reset(&mut cstr_buf);
    p = cstr_buf.data as *mut std::os::raw::c_char;
    match v {
        194 | 195 | 198 | 199 | 196 | 197 => {
            /* XXX: not quite exact, but only useful for testing  */
            sprintf(
                p,
                b"%llu\x00" as *const u8 as *const std::os::raw::c_char,
                (*cv).i as std::os::raw::c_ulonglong,
            );
            current_block = 15594839951440953787;
        },
        193 => {
            cstr_ccat(&mut cstr_buf, 'L' as i32);
            current_block = 15556716285768475671;
        },
        192 => {
            current_block = 15556716285768475671;
        },
        205 | 206 => return (*cv).str_0.data as *mut std::os::raw::c_char,
        201 => {
            cstr_ccat(&mut cstr_buf, 'L' as i32);
            current_block = 8487402319155624291;
        },
        200 => {
            current_block = 8487402319155624291;
        },
        202 => {
            cstr_cat(
                &mut cstr_buf,
                b"<float>\x00" as *const u8 as *const std::os::raw::c_char,
                0 as std::os::raw::c_int,
            );
            current_block = 15594839951440953787;
        },
        203 => {
            cstr_cat(
                &mut cstr_buf,
                b"<double>\x00" as *const u8 as *const std::os::raw::c_char,
                0 as std::os::raw::c_int,
            );
            current_block = 15594839951440953787;
        },
        204 => {
            cstr_cat(
                &mut cstr_buf,
                b"<long double>\x00" as *const u8 as *const std::os::raw::c_char,
                0 as std::os::raw::c_int,
            );
            current_block = 15594839951440953787;
        },
        207 => {
            cstr_cat(
                &mut cstr_buf,
                b"<linenumber>\x00" as *const u8 as *const std::os::raw::c_char,
                0 as std::os::raw::c_int,
            );
            current_block = 15594839951440953787;
        },
        156 => {
            /* above tokens have value, the ones below don't */
            v = '<' as i32;
            current_block = 14832005871758244535;
        },
        159 => {
            v = '>' as i32;
            current_block = 14832005871758244535;
        },
        161 => return strcpy(p, b"...\x00" as *const u8 as *const std::os::raw::c_char),
        184 => return strcpy(p, b"<<=\x00" as *const u8 as *const std::os::raw::c_char),
        185 => return strcpy(p, b">>=\x00" as *const u8 as *const std::os::raw::c_char),
        -1 => return strcpy(p, b"<eof>\x00" as *const u8 as *const std::os::raw::c_char),
        0 => {
            current_block = 1491878468859892758;
        },
        _ => {
            if v < 256 as std::os::raw::c_int {
                /* search in two bytes table */
                q = tok_two_chars.as_ptr();
                while *q != 0 {
                    if *q.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int == v {
                        let fresh12 = p;
                        p = p.offset(1);
                        *fresh12 =
                            *q.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_char;
                        let fresh13 = p;
                        p = p.offset(1);
                        *fresh13 =
                            *q.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_char;
                        *p = '\u{0}' as i32 as std::os::raw::c_char;
                        return cstr_buf.data as *const std::os::raw::c_char;
                    }
                    q = q.offset(3 as std::os::raw::c_int as isize)
                }
                if v >= 127 as std::os::raw::c_int {
                    sprintf(
                        cstr_buf.data as *mut std::os::raw::c_char,
                        b"<%02x>\x00" as *const u8 as *const std::os::raw::c_char,
                        v,
                    );
                    return cstr_buf.data as *const std::os::raw::c_char;
                }
                current_block = 14832005871758244535;
            } else {
                if v < tok_ident {
                    return (**table_ident.offset((v - 256 as std::os::raw::c_int) as isize))
                        .str_0
                        .as_mut_ptr();
                } else {
                    if v >= 0x10000000 as std::os::raw::c_int {
                        /* special name for anonymous symbol */
                        sprintf(
                            p,
                            b"L.%u\x00" as *const u8 as *const std::os::raw::c_char,
                            v - 0x10000000 as std::os::raw::c_int,
                        );
                    } else {
                        /* should never happen */
                        return 0 as *const std::os::raw::c_char;
                    }
                }
                current_block = 15594839951440953787;
            }
        },
    }
    match current_block {
        8487402319155624291 => {
            cstr_ccat(&mut cstr_buf, '\"' as i32);
            if v == 0xc8 as std::os::raw::c_int {
                len = (*cv).str_0.size - 1 as std::os::raw::c_int;
                i = 0 as std::os::raw::c_int;
                while i < len {
                    add_char(
                        &mut cstr_buf,
                        *((*cv).str_0.data as *mut std::os::raw::c_uchar).offset(i as isize)
                            as std::os::raw::c_int,
                    );
                    i += 1
                }
            } else {
                len = ((*cv).str_0.size as std::os::raw::c_ulong)
                    .wrapping_div(::std::mem::size_of::<nwchar_t>() as std::os::raw::c_ulong)
                    .wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong)
                    as std::os::raw::c_int;
                i = 0 as std::os::raw::c_int;
                while i < len {
                    add_char(
                        &mut cstr_buf,
                        *((*cv).str_0.data as *mut nwchar_t).offset(i as isize),
                    );
                    i += 1
                }
            }
            cstr_ccat(&mut cstr_buf, '\"' as i32);
            cstr_ccat(&mut cstr_buf, '\u{0}' as i32);
            current_block = 15594839951440953787;
        },
        15556716285768475671 => {
            cstr_ccat(&mut cstr_buf, '\'' as i32);
            add_char(&mut cstr_buf, (*cv).i as std::os::raw::c_int);
            cstr_ccat(&mut cstr_buf, '\'' as i32);
            cstr_ccat(&mut cstr_buf, '\u{0}' as i32);
            current_block = 15594839951440953787;
        },
        14832005871758244535 => {
            let fresh14 = p;
            p = p.offset(1);
            *fresh14 = v as std::os::raw::c_char;
            current_block = 1491878468859892758;
        },
        _ => {},
    }
    match current_block {
        1491878468859892758 => {
            /* nameless anonymous symbol */
            *p = '\u{0}' as i32 as std::os::raw::c_char
        },
        _ => {},
    }
    return cstr_buf.data as *const std::os::raw::c_char;
}
/* return the current character, handling end of block if necessary
(but not stray) */
unsafe extern "C" fn handle_eob() -> std::os::raw::c_int {
    let mut bf: *mut BufferedFile = file;
    let mut len: std::os::raw::c_int = 0;
    /* only tries to read if really end of buffer */
    if (*bf).buf_ptr >= (*bf).buf_end {
        if (*bf).fd >= 0 as std::os::raw::c_int {
            len = 8192 as std::os::raw::c_int;
            len = read(
                (*bf).fd,
                (*bf).buffer.as_mut_ptr() as *mut std::os::raw::c_void,
                len as size_t,
            ) as std::os::raw::c_int;
            if len < 0 as std::os::raw::c_int {
                len = 0 as std::os::raw::c_int
            }
        } else {
            len = 0 as std::os::raw::c_int
        }
        (*tcc_state).total_bytes += len;
        (*bf).buf_ptr = (*bf).buffer.as_mut_ptr();
        (*bf).buf_end = (*bf).buffer.as_mut_ptr().offset(len as isize);
        *(*bf).buf_end = '\\' as i32 as uint8_t
    }
    if (*bf).buf_ptr < (*bf).buf_end {
        return *(*bf).buf_ptr.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int;
    } else {
        (*bf).buf_ptr = (*bf).buf_end;
        return -(1 as std::os::raw::c_int);
    };
}
/* read next char from current input file and handle end of input buffer */
#[inline]
unsafe extern "C" fn inp() {
    (*file).buf_ptr = (*file).buf_ptr.offset(1);
    ch = *(*file).buf_ptr as std::os::raw::c_int;
    /* end of buffer/file handling */
    if ch == '\\' as i32 {
        ch = handle_eob()
    };
}
/* handle '\[\r]\n' */
unsafe extern "C" fn handle_stray_noerror() -> std::os::raw::c_int {
    while ch == '\\' as i32 {
        inp();
        if ch == '\n' as i32 {
            (*file).line_num += 1;
            inp();
        } else {
            's_57: {
                if ch == '\r' as i32 {
                    inp();
                    if !(ch != '\n' as i32) {
                        (*file).line_num += 1;
                        inp();
                        break 's_57;
                    }
                }
                return 1 as std::os::raw::c_int;
            }
        }
    }
    return 0 as std::os::raw::c_int;
}
unsafe extern "C" fn handle_stray() {
    if handle_stray_noerror() != 0 {
        _tcc_error(b"stray \'\\\' in program\x00" as *const u8 as *const std::os::raw::c_char);
    };
}
/* skip the stray and handle the \\n case. Output an error if
incorrect char after the stray */
unsafe extern "C" fn handle_stray1(mut p: *mut uint8_t) -> std::os::raw::c_int {
    let mut c: std::os::raw::c_int = 0;
    (*file).buf_ptr = p;
    if p >= (*file).buf_end {
        c = handle_eob();
        if c != '\\' as i32 {
            return c;
        }
        p = (*file).buf_ptr
    }
    ch = *p as std::os::raw::c_int;
    if handle_stray_noerror() != 0 {
        if parse_flags & 0x20 as std::os::raw::c_int == 0 {
            _tcc_error(b"stray \'\\\' in program\x00" as *const u8 as *const std::os::raw::c_char);
        }
        (*file).buf_ptr = (*file).buf_ptr.offset(-1);
        *(*file).buf_ptr = '\\' as i32 as uint8_t
    }
    p = (*file).buf_ptr;
    c = *p as std::os::raw::c_int;
    return c;
}
/* handle just the EOB case, but not stray */
/* handle the complicated stray case */
/* input with '\[\r]\n' handling. Note that this function cannot
handle other characters after '\', so you cannot call it inside
strings or comments */
unsafe extern "C" fn minp() {
    inp();
    if ch == '\\' as i32 {
        handle_stray();
    };
}
/* single line C++ comments */
unsafe extern "C" fn parse_line_comment(mut p: *mut uint8_t) -> *mut uint8_t {
    let mut c: std::os::raw::c_int = 0;
    p = p.offset(1);
    's_14: loop {
        c = *p as std::os::raw::c_int;
        loop {
            if c == '\n' as i32 || c == -(1 as std::os::raw::c_int) {
                break 's_14;
            }
            if c == '\\' as i32 {
                (*file).buf_ptr = p;
                c = handle_eob();
                p = (*file).buf_ptr;
                if !(c == '\\' as i32) {
                    continue;
                }
                p = p.offset(1);
                c = *p as std::os::raw::c_int;
                if c == '\\' as i32 {
                    (*file).buf_ptr = p;
                    c = handle_eob();
                    p = (*file).buf_ptr
                }
                if c == '\n' as i32 {
                    (*file).line_num += 1;
                    p = p.offset(1);
                    c = *p as std::os::raw::c_int;
                    if c == '\\' as i32 {
                        (*file).buf_ptr = p;
                        c = handle_eob();
                        p = (*file).buf_ptr
                    }
                } else if c == '\r' as i32 {
                    p = p.offset(1);
                    c = *p as std::os::raw::c_int;
                    if c == '\\' as i32 {
                        (*file).buf_ptr = p;
                        c = handle_eob();
                        p = (*file).buf_ptr
                    }
                    if c == '\n' as i32 {
                        (*file).line_num += 1;
                        p = p.offset(1);
                        c = *p as std::os::raw::c_int;
                        if c == '\\' as i32 {
                            (*file).buf_ptr = p;
                            c = handle_eob();
                            p = (*file).buf_ptr
                        }
                    }
                }
                break;
            } else {
                p = p.offset(1);
                break;
            }
        }
    }
    return p;
}
/* C comments */
unsafe extern "C" fn parse_comment(mut p: *mut uint8_t) -> *mut uint8_t {
    let mut c: std::os::raw::c_int = 0;
    p = p.offset(1);
    's_14: loop {
        loop
        /* fast skip loop */
        {
            c = *p as std::os::raw::c_int;
            if c == '\n' as i32 || c == '*' as i32 || c == '\\' as i32 {
                break;
            }
            p = p.offset(1);
            c = *p as std::os::raw::c_int;
            if c == '\n' as i32 || c == '*' as i32 || c == '\\' as i32 {
                break;
            }
            p = p.offset(1)
        }
        /* now we can handle all the cases */
        if c == '\n' as i32 {
            (*file).line_num += 1;
            p = p.offset(1)
        } else if c == '*' as i32 {
            p = p.offset(1);
            's_85: loop {
                c = *p as std::os::raw::c_int;
                if c == '*' as i32 {
                    p = p.offset(1)
                } else {
                    if c == '/' as i32 {
                        break 's_14;
                    }
                    if !(c == '\\' as i32) {
                        break;
                    }
                    (*file).buf_ptr = p;
                    c = handle_eob();
                    p = (*file).buf_ptr;
                    if c == -(1 as std::os::raw::c_int) {
                        _tcc_error(
                            b"unexpected end of file in comment\x00" as *const u8
                                as *const std::os::raw::c_char,
                        );
                    }
                    if !(c == '\\' as i32) {
                        continue;
                    }
                    /* skip '\[\r]\n', otherwise just skip the stray */
                    while c == '\\' as i32 {
                        p = p.offset(1);
                        c = *p as std::os::raw::c_int;
                        if c == '\\' as i32 {
                            (*file).buf_ptr = p;
                            c = handle_eob();
                            p = (*file).buf_ptr
                        }
                        if c == '\n' as i32 {
                            (*file).line_num += 1;
                            p = p.offset(1);
                            c = *p as std::os::raw::c_int;
                            if c == '\\' as i32 {
                                (*file).buf_ptr = p;
                                c = handle_eob();
                                p = (*file).buf_ptr
                            }
                        } else {
                            if !(c == '\r' as i32) {
                                break 's_85;
                            }
                            p = p.offset(1);
                            c = *p as std::os::raw::c_int;
                            if c == '\\' as i32 {
                                (*file).buf_ptr = p;
                                c = handle_eob();
                                p = (*file).buf_ptr
                            }
                            if c == '\n' as i32 {
                                (*file).line_num += 1;
                                p = p.offset(1);
                                c = *p as std::os::raw::c_int;
                                if c == '\\' as i32 {
                                    (*file).buf_ptr = p;
                                    c = handle_eob();
                                    p = (*file).buf_ptr
                                }
                            }
                        }
                    }
                }
            }
        } else {
            /* stray, eob or eof */
            (*file).buf_ptr = p;
            c = handle_eob();
            p = (*file).buf_ptr;
            if c == -(1 as std::os::raw::c_int) {
                _tcc_error(
                    b"unexpected end of file in comment\x00" as *const u8
                        as *const std::os::raw::c_char,
                );
            } else {
                if c == '\\' as i32 {
                    p = p.offset(1)
                }
            }
        }
    }
    p = p.offset(1);
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn set_idnum(
    mut c: std::os::raw::c_int,
    mut val: std::os::raw::c_int,
) -> std::os::raw::c_int {
    let mut prev: std::os::raw::c_int =
        isidnum_table[(c - -(1 as std::os::raw::c_int)) as usize] as std::os::raw::c_int;
    isidnum_table[(c - -(1 as std::os::raw::c_int)) as usize] = val as std::os::raw::c_uchar;
    return prev;
}
#[inline]
unsafe extern "C" fn skip_spaces() {
    while isidnum_table[(ch - -(1 as std::os::raw::c_int)) as usize] as std::os::raw::c_int
        & 1 as std::os::raw::c_int
        != 0
    {
        minp();
    }
}
#[inline]
unsafe extern "C" fn check_space(
    mut t: std::os::raw::c_int,
    mut spc: *mut std::os::raw::c_int,
) -> std::os::raw::c_int {
    if t < 256 as std::os::raw::c_int
        && isidnum_table[(t - -(1 as std::os::raw::c_int)) as usize] as std::os::raw::c_int
            & 1 as std::os::raw::c_int
            != 0
    {
        if *spc != 0 {
            return 1 as std::os::raw::c_int;
        }
        *spc = 1 as std::os::raw::c_int
    } else {
        *spc = 0 as std::os::raw::c_int
    }
    return 0 as std::os::raw::c_int;
}
/* parse a string without interpreting escapes */
unsafe extern "C" fn parse_pp_string(
    mut p: *mut uint8_t,
    mut sep: std::os::raw::c_int,
    mut str: *mut CString,
) -> *mut uint8_t {
    let mut c: std::os::raw::c_int = 0;
    p = p.offset(1);
    loop {
        c = *p as std::os::raw::c_int;
        if c == sep {
            break;
        }
        if c == '\\' as i32 {
            (*file).buf_ptr = p;
            c = handle_eob();
            p = (*file).buf_ptr;
            's_217: {
                let mut current_block_37: u64;
                if !(c == -(1 as std::os::raw::c_int)) {
                    if c == '\\' as i32 {
                        /* escape : just skip \[\r]\n */
                        p = p.offset(1);
                        c = *p as std::os::raw::c_int;
                        if c == '\\' as i32 {
                            (*file).buf_ptr = p;
                            c = handle_eob();
                            p = (*file).buf_ptr
                        }
                        if c == '\n' as i32 {
                            (*file).line_num += 1;
                            p = p.offset(1);
                            current_block_37 = 2480299350034459858;
                        } else if c == '\r' as i32 {
                            p = p.offset(1);
                            c = *p as std::os::raw::c_int;
                            if c == '\\' as i32 {
                                (*file).buf_ptr = p;
                                c = handle_eob();
                                p = (*file).buf_ptr
                            }
                            if c != '\n' as i32 {
                                expect(
                                    b"\'\n\' after \'\r\'\x00" as *const u8
                                        as *const std::os::raw::c_char,
                                );
                            }
                            (*file).line_num += 1;
                            p = p.offset(1);
                            current_block_37 = 2480299350034459858;
                        } else if c == -(1 as std::os::raw::c_int) {
                            current_block_37 = 4305732096819611692;
                        } else {
                            if !str.is_null() {
                                cstr_ccat(str, '\\' as i32);
                                cstr_ccat(str, c);
                            }
                            p = p.offset(1);
                            current_block_37 = 2480299350034459858;
                        }
                    } else {
                        current_block_37 = 2480299350034459858;
                    }
                    match current_block_37 {
                        4305732096819611692 => {},
                        _ => {
                            break 's_217;
                        },
                    }
                }
                /* XXX: indicate line number of start of string */
                _tcc_error(
                    b"missing terminating %c character\x00" as *const u8
                        as *const std::os::raw::c_char,
                    sep,
                );
            }
        } else {
            if c == '\n' as i32 {
                (*file).line_num += 1
            } else if c == '\r' as i32 {
                p = p.offset(1);
                c = *p as std::os::raw::c_int;
                if c == '\\' as i32 {
                    (*file).buf_ptr = p;
                    c = handle_eob();
                    p = (*file).buf_ptr
                }
                if c != '\n' as i32 {
                    if !str.is_null() {
                        cstr_ccat(str, '\r' as i32);
                    }
                    continue;
                } else {
                    (*file).line_num += 1
                }
            }
            if !str.is_null() {
                cstr_ccat(str, c);
            }
            p = p.offset(1)
        }
    }
    p = p.offset(1);
    return p;
}
/* skip block of text until #else, #elif or #endif. skip also pairs of
#if/#endif */
unsafe extern "C" fn preprocess_skip() {
    let mut current_block: u64;
    let mut a: std::os::raw::c_int = 0;
    let mut start_of_line: std::os::raw::c_int = 0;
    let mut c: std::os::raw::c_int = 0;
    let mut in_warn_or_error: std::os::raw::c_int = 0;
    let mut p: *mut uint8_t = 0 as *mut uint8_t;
    p = (*file).buf_ptr;
    a = 0 as std::os::raw::c_int;
    'c_23863: loop {
        start_of_line = 1 as std::os::raw::c_int;
        in_warn_or_error = 0 as std::os::raw::c_int;
        loop {
            c = *p as std::os::raw::c_int;
            match c {
                32 | 9 | 12 | 11 | 13 => {
                    p = p.offset(1);
                    continue;
                },
                10 => {
                    (*file).line_num += 1;
                    p = p.offset(1);
                    break;
                },
                92 => {
                    (*file).buf_ptr = p;
                    c = handle_eob();
                    if c == -(1 as std::os::raw::c_int) {
                        expect(b"#endif\x00" as *const u8 as *const std::os::raw::c_char);
                    } else {
                        if c == '\\' as i32 {
                            ch = *(*file).buf_ptr.offset(0 as std::os::raw::c_int as isize)
                                as std::os::raw::c_int;
                            handle_stray_noerror();
                        }
                    }
                    p = (*file).buf_ptr;
                    continue;
                },
                34 | 39 => {
                    /* skip strings */
                    if in_warn_or_error != 0 {
                        current_block = 4056437700417898090;
                    } else {
                        p = parse_pp_string(p, c, 0 as *mut CString);
                        current_block = 10753070352654377903;
                    }
                },
                47 => {
                    /* skip comments */
                    if in_warn_or_error != 0 {
                        current_block = 4056437700417898090;
                    } else {
                        (*file).buf_ptr = p;
                        ch = *p as std::os::raw::c_int;
                        minp();
                        p = (*file).buf_ptr;
                        if ch == '*' as i32 {
                            p = parse_comment(p)
                        } else if ch == '/' as i32 {
                            p = parse_line_comment(p)
                        }
                        current_block = 10753070352654377903;
                    }
                },
                35 => {
                    p = p.offset(1);
                    if start_of_line != 0 {
                        (*file).buf_ptr = p;
                        next_nomacro();
                        p = (*file).buf_ptr;
                        if a == 0 as std::os::raw::c_int
                            && (tok == TOK_ELSE as std::os::raw::c_int
                                || tok == TOK_ELIF as std::os::raw::c_int
                                || tok == TOK_ENDIF as std::os::raw::c_int)
                        {
                            break 'c_23863;
                        }
                        if tok == TOK_IF as std::os::raw::c_int
                            || tok == TOK_IFDEF as std::os::raw::c_int
                            || tok == TOK_IFNDEF as std::os::raw::c_int
                        {
                            a += 1
                        } else if tok == TOK_ENDIF as std::os::raw::c_int {
                            a -= 1
                        } else if tok == TOK_ERROR as std::os::raw::c_int
                            || tok == TOK_WARNING as std::os::raw::c_int
                        {
                            in_warn_or_error = 1 as std::os::raw::c_int
                        } else {
                            if tok == 10 as std::os::raw::c_int {
                                break;
                            }
                            if parse_flags & 0x8 as std::os::raw::c_int != 0 {
                                p = parse_line_comment(
                                    p.offset(-(1 as std::os::raw::c_int as isize)),
                                )
                            }
                        }
                    } else if parse_flags & 0x8 as std::os::raw::c_int != 0 {
                        p = parse_line_comment(p.offset(-(1 as std::os::raw::c_int as isize)))
                    }
                    current_block = 10753070352654377903;
                },
                _ => {
                    current_block = 4056437700417898090;
                },
            }
            match current_block {
                4056437700417898090 => p = p.offset(1),
                _ => {},
            }
            start_of_line = 0 as std::os::raw::c_int
        }
    }
    (*file).buf_ptr = p;
}
/* token string handling */
#[no_mangle]
pub unsafe extern "C" fn tok_str_new(mut s: *mut TokenString) {
    (*s).str_0 = 0 as *mut std::os::raw::c_int; /* don't free */
    (*s).lastlen = 0 as std::os::raw::c_int;
    (*s).len = (*s).lastlen;
    (*s).allocated_len = 0 as std::os::raw::c_int;
    (*s).last_line_num = -(1 as std::os::raw::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn tok_str_alloc() -> *mut TokenString {
    let mut str: *mut TokenString = tal_realloc_impl(
        &mut tokstr_alloc,
        0 as *mut std::os::raw::c_void,
        ::std::mem::size_of::<TokenString>() as std::os::raw::c_ulong as std::os::raw::c_uint,
    ) as *mut TokenString;
    tok_str_new(str);
    return str;
}
#[no_mangle]
pub unsafe extern "C" fn tok_str_dup(mut s: *mut TokenString) -> *mut std::os::raw::c_int {
    let mut str: *mut std::os::raw::c_int = 0 as *mut std::os::raw::c_int;
    str = tal_realloc_impl(
        &mut tokstr_alloc,
        0 as *mut std::os::raw::c_void,
        ((*s).len as std::os::raw::c_ulong)
            .wrapping_mul(::std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong)
            as std::os::raw::c_uint,
    ) as *mut std::os::raw::c_int;
    memcpy(
        str as *mut std::os::raw::c_void,
        (*s).str_0 as *const std::os::raw::c_void,
        ((*s).len as std::os::raw::c_ulong)
            .wrapping_mul(::std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong),
    );
    return str;
}
#[no_mangle]
pub unsafe extern "C" fn tok_str_free_str(mut str: *mut std::os::raw::c_int) {
    tal_free_impl(tokstr_alloc, str as *mut std::os::raw::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn tok_str_free(mut str: *mut TokenString) {
    tok_str_free_str((*str).str_0);
    tal_free_impl(tokstr_alloc, str as *mut std::os::raw::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn tok_str_realloc(
    mut s: *mut TokenString,
    mut new_size: std::os::raw::c_int,
) -> *mut std::os::raw::c_int {
    let mut str: *mut std::os::raw::c_int = 0 as *mut std::os::raw::c_int;
    let mut size: std::os::raw::c_int = 0;
    size = (*s).allocated_len;
    if size < 16 as std::os::raw::c_int {
        size = 16 as std::os::raw::c_int
    }
    while size < new_size {
        size = size * 2 as std::os::raw::c_int
    }
    if size > (*s).allocated_len {
        str = tal_realloc_impl(
            &mut tokstr_alloc,
            (*s).str_0 as *mut std::os::raw::c_void,
            (size as std::os::raw::c_ulong)
                .wrapping_mul(::std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong)
                as std::os::raw::c_uint,
        ) as *mut std::os::raw::c_int;
        (*s).allocated_len = size;
        (*s).str_0 = str
    }
    return (*s).str_0;
}
#[no_mangle]
pub unsafe extern "C" fn tok_str_add(mut s: *mut TokenString, mut t: std::os::raw::c_int) {
    let mut len: std::os::raw::c_int = 0;
    let mut str: *mut std::os::raw::c_int = 0 as *mut std::os::raw::c_int;
    len = (*s).len;
    str = (*s).str_0;
    if len >= (*s).allocated_len {
        str = tok_str_realloc(s, len + 1 as std::os::raw::c_int)
    }
    let fresh15 = len;
    len = len + 1;
    *str.offset(fresh15 as isize) = t;
    (*s).len = len;
}
#[no_mangle]
pub unsafe extern "C" fn begin_macro(mut str: *mut TokenString, mut alloc: std::os::raw::c_int) {
    (*str).alloc = alloc as std::os::raw::c_char;
    (*str).prev = macro_stack;
    (*str).prev_ptr = macro_ptr;
    (*str).save_line_num = (*file).line_num;
    macro_ptr = (*str).str_0;
    macro_stack = str;
}
#[no_mangle]
pub unsafe extern "C" fn end_macro() {
    let mut str: *mut TokenString = macro_stack;
    macro_stack = (*str).prev;
    macro_ptr = (*str).prev_ptr;
    (*file).line_num = (*str).save_line_num;
    if (*str).alloc as std::os::raw::c_int != 0 as std::os::raw::c_int {
        if (*str).alloc as std::os::raw::c_int == 2 as std::os::raw::c_int {
            (*str).str_0 = 0 as *mut std::os::raw::c_int
        }
        tok_str_free(str);
    };
}
unsafe extern "C" fn tok_str_add2(
    mut s: *mut TokenString,
    mut t: std::os::raw::c_int,
    mut cv: *mut CValue,
) {
    let mut len: std::os::raw::c_int = 0;
    let mut str: *mut std::os::raw::c_int = 0 as *mut std::os::raw::c_int;
    (*s).lastlen = (*s).len;
    len = (*s).lastlen;
    str = (*s).str_0;
    /* allocate space for worst case */
    if len + 4 as std::os::raw::c_int >= (*s).allocated_len {
        str = tok_str_realloc(s, len + 4 as std::os::raw::c_int + 1 as std::os::raw::c_int)
    }
    let fresh16 = len;
    len = len + 1;
    *str.offset(fresh16 as isize) = t;
    match t {
        194 | 195 | 192 | 193 | 202 | 207 => {
            let fresh17 = len;
            len = len + 1;
            *str.offset(fresh17 as isize) = (*cv).tab[0 as std::os::raw::c_int as usize]
        },
        205 | 206 | 200 | 201 => {
            /* Insert the string into the int array. */
            let mut nb_words: size_t = (1 as std::os::raw::c_int as std::os::raw::c_ulong)
                .wrapping_add(
                    ((*cv).str_0.size as std::os::raw::c_ulong)
                        .wrapping_add(
                            ::std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong
                        )
                        .wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong)
                        .wrapping_div(
                            ::std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong
                        ),
                );
            if (len as std::os::raw::c_ulong).wrapping_add(nb_words)
                >= (*s).allocated_len as std::os::raw::c_ulong
            {
                str = tok_str_realloc(
                    s,
                    (len as std::os::raw::c_ulong)
                        .wrapping_add(nb_words)
                        .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong)
                        as std::os::raw::c_int,
                )
            }
            *str.offset(len as isize) = (*cv).str_0.size;
            memcpy(
                &mut *str.offset((len + 1 as std::os::raw::c_int) as isize)
                    as *mut std::os::raw::c_int as *mut std::os::raw::c_void,
                (*cv).str_0.data,
                (*cv).str_0.size as std::os::raw::c_ulong,
            );
            len = (len as std::os::raw::c_ulong).wrapping_add(nb_words) as std::os::raw::c_int
                as std::os::raw::c_int
        },
        203 | 196 | 197 | 198 | 199 => {
            let fresh18 = len;
            len = len + 1;
            *str.offset(fresh18 as isize) = (*cv).tab[0 as std::os::raw::c_int as usize];
            let fresh19 = len;
            len = len + 1;
            *str.offset(fresh19 as isize) = (*cv).tab[1 as std::os::raw::c_int as usize]
        },
        204 => {
            let fresh20 = len;
            len = len + 1;
            *str.offset(fresh20 as isize) = (*cv).tab[0 as std::os::raw::c_int as usize];
            let fresh21 = len;
            len = len + 1;
            *str.offset(fresh21 as isize) = (*cv).tab[1 as std::os::raw::c_int as usize];
            let fresh22 = len;
            len = len + 1;
            *str.offset(fresh22 as isize) = (*cv).tab[2 as std::os::raw::c_int as usize];
            let fresh23 = len;
            len = len + 1;
            *str.offset(fresh23 as isize) = (*cv).tab[3 as std::os::raw::c_int as usize]
        },
        _ => {},
    }
    (*s).len = len;
}
/* add the current parse token in token string 's' */
#[no_mangle]
pub unsafe extern "C" fn tok_str_add_tok(mut s: *mut TokenString) {
    let mut cval: CValue = CValue {
        ld: f128::f128::ZERO,
    };
    /* save line number info */
    if (*file).line_num != (*s).last_line_num {
        (*s).last_line_num = (*file).line_num;
        cval.i = (*s).last_line_num as uint64_t;
        tok_str_add2(s, 0xcf as std::os::raw::c_int, &mut cval);
    }
    tok_str_add2(s, tok, &mut tokc);
}
/* get a token from an integer array and increment pointer. */
#[inline]
unsafe extern "C" fn tok_get(
    mut t: *mut std::os::raw::c_int,
    mut pp: *mut *const std::os::raw::c_int,
    mut cv: *mut CValue,
) {
    let mut p: *const std::os::raw::c_int = *pp;
    let mut n: std::os::raw::c_int = 0;
    let mut tab: *mut std::os::raw::c_int = 0 as *mut std::os::raw::c_int;
    tab = (*cv).tab.as_mut_ptr();
    let mut current_block_10: u64;
    let fresh24 = p;
    p = p.offset(1);
    *t = *fresh24;
    match *t {
        194 | 192 | 193 | 207 => {
            let fresh25 = p;
            p = p.offset(1);
            (*cv).i = *fresh25 as uint64_t;
            current_block_10 = 5601891728916014340;
        },
        195 => {
            let fresh26 = p;
            p = p.offset(1);
            (*cv).i = *fresh26 as std::os::raw::c_uint as uint64_t;
            current_block_10 = 5601891728916014340;
        },
        202 => {
            let fresh27 = p;
            p = p.offset(1);
            *tab.offset(0 as std::os::raw::c_int as isize) = *fresh27;
            current_block_10 = 5601891728916014340;
        },
        200 | 201 | 205 | 206 => {
            let fresh28 = p;
            p = p.offset(1);
            (*cv).str_0.size = *fresh28;
            (*cv).str_0.data = p as *const std::os::raw::c_void;
            p = p.offset(
                ((*cv).str_0.size as std::os::raw::c_ulong)
                    .wrapping_add(
                        ::std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong
                    )
                    .wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong)
                    .wrapping_div(
                        ::std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong
                    ) as isize,
            );
            current_block_10 = 5601891728916014340;
        },
        203 | 196 | 197 | 198 | 199 => {
            n = 2 as std::os::raw::c_int;
            current_block_10 = 2305958262682200376;
        },
        204 => {
            n = 4 as std::os::raw::c_int;
            current_block_10 = 2305958262682200376;
        },
        _ => {
            current_block_10 = 5601891728916014340;
        },
    }
    match current_block_10 {
        2305958262682200376 => loop {
            let fresh29 = p;
            p = p.offset(1);
            let fresh30 = tab;
            tab = tab.offset(1);
            *fresh30 = *fresh29;
            n -= 1;
            if !(n != 0) {
                break;
            }
        },
        _ => {},
    }
    *pp = p;
}
unsafe extern "C" fn macro_is_equal(
    mut a: *const std::os::raw::c_int,
    mut b: *const std::os::raw::c_int,
) -> std::os::raw::c_int {
    let mut cv: CValue = CValue {
        ld: f128::f128::ZERO,
    };
    let mut t: std::os::raw::c_int = 0;
    if a.is_null() || b.is_null() {
        return 1 as std::os::raw::c_int;
    }
    while *a != 0 && *b != 0 {
        /* first time preallocate macro_equal_buf, next time only reset position to start */
        cstr_reset(&mut macro_equal_buf);
        let mut _t: std::os::raw::c_int = *a;
        if _t >= 0xc0 as std::os::raw::c_int && _t <= 0xcf as std::os::raw::c_int {
            tok_get(&mut t, &mut a, &mut cv);
        } else {
            t = _t;
            a = a.offset(1)
        }
        cstr_cat(
            &mut macro_equal_buf,
            get_tok_str(t, &mut cv),
            0 as std::os::raw::c_int,
        );
        let mut _t_0: std::os::raw::c_int = *b;
        if _t_0 >= 0xc0 as std::os::raw::c_int && _t_0 <= 0xcf as std::os::raw::c_int {
            tok_get(&mut t, &mut b, &mut cv);
        } else {
            t = _t_0;
            b = b.offset(1)
        }
        if strcmp(
            macro_equal_buf.data as *const std::os::raw::c_char,
            get_tok_str(t, &mut cv),
        ) != 0
        {
            return 0 as std::os::raw::c_int;
        }
    }
    return !(*a != 0 || *b != 0) as std::os::raw::c_int;
}
/* defines handling */
#[no_mangle]
pub unsafe extern "C" fn define_push(
    mut v: std::os::raw::c_int,
    mut macro_type: std::os::raw::c_int,
    mut str: *mut std::os::raw::c_int,
    mut first_arg: *mut Sym,
) {
    let mut s: *mut Sym = 0 as *mut Sym;
    let mut o: *mut Sym = 0 as *mut Sym;
    o = define_find(v);
    s = sym_push2(&mut define_stack, v, macro_type, 0 as std::os::raw::c_int);
    (*s).c2rust_unnamed.d = str;
    (*s).c2rust_unnamed_0.next = first_arg;
    let ref mut fresh31 =
        (**table_ident.offset((v - 256 as std::os::raw::c_int) as isize)).sym_define;
    *fresh31 = s;
    if !o.is_null() && macro_is_equal((*o).c2rust_unnamed.d, (*s).c2rust_unnamed.d) == 0 {
        _tcc_warning(
            b"%s redefined\x00" as *const u8 as *const std::os::raw::c_char,
            get_tok_str(v, 0 as *mut CValue),
        );
    };
}
/* undefined a define symbol. Its name is just set to zero */
#[no_mangle]
pub unsafe extern "C" fn define_undef(mut s: *mut Sym) {
    let mut v: std::os::raw::c_int = (*s).v;
    if v >= 256 as std::os::raw::c_int && v < tok_ident {
        let ref mut fresh32 =
            (**table_ident.offset((v - 256 as std::os::raw::c_int) as isize)).sym_define;
        *fresh32 = 0 as *mut Sym
    };
}
#[no_mangle]
pub unsafe extern "C" fn define_find(mut v: std::os::raw::c_int) -> *mut Sym {
    v -= 256 as std::os::raw::c_int;
    if v as std::os::raw::c_uint >= (tok_ident - 256 as std::os::raw::c_int) as std::os::raw::c_uint
    {
        return 0 as *mut Sym;
    }
    return (**table_ident.offset(v as isize)).sym_define;
}
/* free define stack until top reaches 'b' */
#[no_mangle]
pub unsafe extern "C" fn free_defines(mut b: *mut Sym) {
    while define_stack != b {
        let mut top: *mut Sym = define_stack;
        define_stack = (*top).prev;
        tok_str_free_str((*top).c2rust_unnamed.d);
        define_undef(top);
        sym_free(top);
    }
}
/* label lookup */
#[no_mangle]
pub unsafe extern "C" fn label_find(mut v: std::os::raw::c_int) -> *mut Sym {
    v -= 256 as std::os::raw::c_int;
    if v as std::os::raw::c_uint >= (tok_ident - 256 as std::os::raw::c_int) as std::os::raw::c_uint
    {
        return 0 as *mut Sym;
    }
    return (**table_ident.offset(v as isize)).sym_label;
}
#[no_mangle]
pub unsafe extern "C" fn label_push(
    mut ptop: *mut *mut Sym,
    mut v: std::os::raw::c_int,
    mut flags: std::os::raw::c_int,
) -> *mut Sym {
    let mut s: *mut Sym = 0 as *mut Sym;
    let mut ps: *mut *mut Sym = 0 as *mut *mut Sym;
    s = sym_push2(ptop, v, 0 as std::os::raw::c_int, 0 as std::os::raw::c_int);
    (*s).r = flags as std::os::raw::c_ushort;
    ps = &mut (**table_ident.offset((v - 256 as std::os::raw::c_int) as isize)).sym_label;
    if ptop == &mut global_label_stack as *mut *mut Sym {
        /* modify the top most local identifier, so that
        sym_identifier will point to 's' when popped */
        while !(*ps).is_null() {
            ps = &mut (**ps).prev_tok
        }
    }
    (*s).prev_tok = *ps;
    *ps = s;
    return s;
}
/* pop labels until element last is reached. Look if any labels are
undefined. Define symbols if '&&label' was used. */
#[no_mangle]
pub unsafe extern "C" fn label_pop(
    mut ptop: *mut *mut Sym,
    mut slast: *mut Sym,
    mut keep: std::os::raw::c_int,
) {
    let mut s: *mut Sym = 0 as *mut Sym;
    let mut s1: *mut Sym = 0 as *mut Sym;
    s = *ptop;
    while s != slast {
        s1 = (*s).prev;
        if (*s).r as std::os::raw::c_int == 2 as std::os::raw::c_int {
            _tcc_warning(
                b"label \'%s\' declared but not used\x00" as *const u8
                    as *const std::os::raw::c_char,
                get_tok_str((*s).v, 0 as *mut CValue),
            );
        } else if (*s).r as std::os::raw::c_int == 1 as std::os::raw::c_int {
            _tcc_error(
                b"label \'%s\' used but not defined\x00" as *const u8
                    as *const std::os::raw::c_char,
                get_tok_str((*s).v, 0 as *mut CValue),
            );
        } else {
            if (*s).c2rust_unnamed.c2rust_unnamed.c != 0 {
                /* define corresponding symbol. A size of
                1 is put. */
                put_extern_sym(
                    s,
                    (*tcc_state).cur_text_section,
                    (*s).c2rust_unnamed.c2rust_unnamed.c2rust_unnamed.jnext as Elf64_Addr,
                    1 as std::os::raw::c_int as std::os::raw::c_ulong,
                );
            }
        }
        /* remove label */
        if (*s).r as std::os::raw::c_int != 3 as std::os::raw::c_int {
            let ref mut fresh33 =
                (**table_ident.offset(((*s).v - 256 as std::os::raw::c_int) as isize)).sym_label;
            *fresh33 = (*s).prev_tok
        }
        if keep == 0 {
            sym_free(s);
        } else {
            (*s).r = 3 as std::os::raw::c_int as std::os::raw::c_ushort
        }
        s = s1
    }
    if keep == 0 {
        *ptop = slast
    };
}
/* fake the nth "#if defined test_..." for tcc -dt -run */
unsafe extern "C" fn maybe_run_test(mut s: *mut TCCState) {
    let mut p: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    if (*s).include_stack_ptr != (*s).include_stack.as_mut_ptr() {
        return;
    }
    p = get_tok_str(tok, 0 as *mut CValue);
    if 0 as std::os::raw::c_int
        != memcmp(
            p as *const std::os::raw::c_void,
            b"test_\x00" as *const u8 as *const std::os::raw::c_char as *const std::os::raw::c_void,
            5 as std::os::raw::c_int as std::os::raw::c_ulong,
        )
    {
        return;
    }
    (*s).run_test -= 1;
    if 0 as std::os::raw::c_int != (*s).run_test {
        return;
    }
    fprintf(
        (*s).ppfp,
        &*(b"\n[%s]\n\x00" as *const u8 as *const std::os::raw::c_char).offset(
            ((*s).dflag as std::os::raw::c_int & 32 as std::os::raw::c_int == 0)
                as std::os::raw::c_int as isize,
        ) as *const std::os::raw::c_char,
        p,
    );
    fflush((*s).ppfp);
    define_push(
        tok,
        0 as std::os::raw::c_int,
        0 as *mut std::os::raw::c_int,
        0 as *mut Sym,
    );
}
/* eval an expression for #if/#elif */
unsafe extern "C" fn expr_preprocess() -> std::os::raw::c_int {
    let mut c: std::os::raw::c_int = 0; /* do macro subst */
    let mut t: std::os::raw::c_int = 0; /* XXX check if correct to use expansion */
    let mut str: *mut TokenString = 0 as *mut TokenString;
    str = tok_str_alloc();
    pp_expr = 1 as std::os::raw::c_int;
    while tok != 10 as std::os::raw::c_int && tok != -(1 as std::os::raw::c_int) {
        next();
        loop {
            if tok == TOK_DEFINED as std::os::raw::c_int {
                next_nomacro();
                t = tok;
                if t == '(' as i32 {
                    next_nomacro();
                }
                if tok < 256 as std::os::raw::c_int {
                    expect(b"identifier\x00" as *const u8 as *const std::os::raw::c_char);
                }
                if (*tcc_state).run_test != 0 {
                    maybe_run_test(tcc_state);
                }
                c = (define_find(tok) != 0 as *mut Sym) as std::os::raw::c_int;
                if t == '(' as i32 {
                    next_nomacro();
                    if tok != ')' as i32 {
                        expect(b"\')\'\x00" as *const u8 as *const std::os::raw::c_char);
                    }
                }
                tok = 0xc2 as std::os::raw::c_int;
                tokc.i = c as uint64_t;
                break;
            } else if 1 as std::os::raw::c_int != 0
                && tok == TOK___HAS_INCLUDE as std::os::raw::c_int
            {
                next();
                skip('(' as i32);
                while tok != ')' as i32 && tok != -(1 as std::os::raw::c_int) {
                    next();
                }
                if tok != ')' as i32 {
                    expect(b"\')\'\x00" as *const u8 as *const std::os::raw::c_char);
                }
                tok = 0xc2 as std::os::raw::c_int;
                tokc.i = 0 as std::os::raw::c_int as uint64_t;
                break;
            } else {
                if !(tok >= 256 as std::os::raw::c_int) {
                    break;
                }
                /* if undefined macro, replace with zero, check for func-like */
                t = tok; /* simulate end of file */
                tok = 0xc2 as std::os::raw::c_int;
                tokc.i = 0 as std::os::raw::c_int as uint64_t;
                tok_str_add_tok(str);
                next();
                if tok == '(' as i32 {
                    _tcc_error(
                        b"function-like macro \'%s\' is not defined\x00" as *const u8
                            as *const std::os::raw::c_char,
                        get_tok_str(t, 0 as *mut CValue),
                    );
                }
            }
        }
        tok_str_add_tok(str);
    }
    pp_expr = 0 as std::os::raw::c_int;
    tok_str_add(str, -(1 as std::os::raw::c_int));
    tok_str_add(str, 0 as std::os::raw::c_int);
    /* now evaluate C constant expression */
    begin_macro(str, 1 as std::os::raw::c_int);
    next();
    c = expr_const();
    end_macro();
    return (c != 0 as std::os::raw::c_int) as std::os::raw::c_int;
}
/* parse after #define */
#[no_mangle]
pub unsafe extern "C" fn parse_define() {
    let mut current_block: u64;
    let mut s: *mut Sym = 0 as *mut Sym;
    let mut first: *mut Sym = 0 as *mut Sym;
    let mut ps: *mut *mut Sym = 0 as *mut *mut Sym;
    let mut v: std::os::raw::c_int = 0;
    let mut t: std::os::raw::c_int = 0;
    let mut varg: std::os::raw::c_int = 0;
    let mut is_vaargs: std::os::raw::c_int = 0;
    let mut spc: std::os::raw::c_int = 0;
    let mut saved_parse_flags: std::os::raw::c_int = parse_flags;
    v = tok;
    if v < 256 as std::os::raw::c_int || v == TOK_DEFINED as std::os::raw::c_int {
        _tcc_error(
            b"invalid macro name \'%s\'\x00" as *const u8 as *const std::os::raw::c_char,
            get_tok_str(tok, &mut tokc),
        );
    }
    /* XXX: should check if same macro (ANSI) */
    first = 0 as *mut Sym;
    t = 0 as std::os::raw::c_int;
    /* We have to parse the whole define as if not in asm mode, in particular
    no line comment with '#' must be ignored.  Also for function
    macros the argument list must be parsed without '.' being an ID
    character.  */
    parse_flags = parse_flags & !(0x8 as std::os::raw::c_int) | 0x10 as std::os::raw::c_int;
    /* '(' must be just after macro definition for MACRO_FUNC */
    next_nomacro();
    parse_flags &= !(0x10 as std::os::raw::c_int);
    if tok == '(' as i32 {
        let mut dotid: std::os::raw::c_int = set_idnum('.' as i32, 0 as std::os::raw::c_int);
        next_nomacro();
        ps = &mut first;
        if tok != ')' as i32 {
            loop {
                varg = tok;
                next_nomacro();
                is_vaargs = 0 as std::os::raw::c_int;
                if varg == 0xa1 as std::os::raw::c_int {
                    varg = TOK___VA_ARGS__ as std::os::raw::c_int;
                    is_vaargs = 1 as std::os::raw::c_int
                } else if tok == 0xa1 as std::os::raw::c_int
                    && (*tcc_state).gnu_ext as std::os::raw::c_int != 0
                {
                    is_vaargs = 1 as std::os::raw::c_int;
                    next_nomacro();
                }
                if !(varg < 256 as std::os::raw::c_int) {
                    s = sym_push2(
                        &mut define_stack,
                        varg | 0x20000000 as std::os::raw::c_int,
                        is_vaargs,
                        0 as std::os::raw::c_int,
                    );
                    *ps = s;
                    ps = &mut (*s).c2rust_unnamed_0.next;
                    if tok == ')' as i32 {
                        break;
                    }
                    if !(tok != ',' as i32 || is_vaargs != 0) {
                        next_nomacro();
                        continue;
                    }
                }
                _tcc_error(
                    b"bad macro parameter list\x00" as *const u8 as *const std::os::raw::c_char,
                );
            }
        }
        parse_flags |= 0x10 as std::os::raw::c_int;
        next_nomacro();
        t = 1 as std::os::raw::c_int;
        set_idnum('.' as i32, dotid);
    }
    tokstr_buf.len = 0 as std::os::raw::c_int;
    spc = 2 as std::os::raw::c_int;
    parse_flags |=
        0x20 as std::os::raw::c_int | 0x10 as std::os::raw::c_int | 0x4 as std::os::raw::c_int;
    loop
    /* The body of a macro definition should be parsed such that identifiers
    are parsed like the file mode determines (i.e. with '.' being an
    ID character in asm mode).  But '#' should be retained instead of
    regarded as line comment leader, so still don't set ASM_FILE
    in parse_flags. */
    {
        if !(tok != 10 as std::os::raw::c_int && tok != -(1 as std::os::raw::c_int)) {
            current_block = 2516253395664191498;
            break;
        }
        /* remove spaces around ## and after '#' */
        if 0xa3 as std::os::raw::c_int == tok {
            if 2 as std::os::raw::c_int == spc {
                current_block = 12499197920459774240; /* remove trailing space */
                break;
            }
            if 1 as std::os::raw::c_int == spc {
                tokstr_buf.len -= 1
            }
            spc = 3 as std::os::raw::c_int;
            tok = 0xa6 as std::os::raw::c_int;
            current_block = 13619784596304402172;
        } else if '#' as i32 == tok {
            spc = 4 as std::os::raw::c_int;
            current_block = 13619784596304402172;
        } else if check_space(tok, &mut spc) != 0 {
            current_block = 12580004273960699447;
        } else {
            current_block = 13619784596304402172;
        }
        match current_block {
            13619784596304402172 => {
                tok_str_add2(&mut tokstr_buf, tok, &mut tokc);
            },
            _ => {},
        }
        next_nomacro();
    }
    match current_block {
        2516253395664191498 => {
            parse_flags = saved_parse_flags;
            if spc == 1 as std::os::raw::c_int {
                tokstr_buf.len -= 1
            }
            tok_str_add(&mut tokstr_buf, 0 as std::os::raw::c_int);
            if !(3 as std::os::raw::c_int == spc) {
                define_push(v, t, tok_str_dup(&mut tokstr_buf), first);
                return;
            }
        },
        _ => {},
    }
    _tcc_error(
        b"\'##\' cannot appear at either end of macro\x00" as *const u8
            as *const std::os::raw::c_char,
    );
}
unsafe extern "C" fn search_cached_include(
    mut s1: *mut TCCState,
    mut filename: *const std::os::raw::c_char,
    mut add: std::os::raw::c_int,
) -> *mut CachedInclude {
    let mut s: *const std::os::raw::c_uchar = 0 as *const std::os::raw::c_uchar;
    let mut h: std::os::raw::c_uint = 0;
    let mut e: *mut CachedInclude = 0 as *mut CachedInclude;
    let mut i: std::os::raw::c_int = 0;
    h = 1 as std::os::raw::c_int as std::os::raw::c_uint;
    s = filename as *mut std::os::raw::c_uchar;
    while *s != 0 {
        h = h
            .wrapping_add(h << 5 as std::os::raw::c_int)
            .wrapping_add(h >> 27 as std::os::raw::c_int)
            .wrapping_add(*s as std::os::raw::c_uint);
        s = s.offset(1)
    }
    h &= (32 as std::os::raw::c_int - 1 as std::os::raw::c_int) as std::os::raw::c_uint;
    i = (*s1).cached_includes_hash[h as usize];
    while !(i == 0 as std::os::raw::c_int) {
        e = *(*s1)
            .cached_includes
            .offset((i - 1 as std::os::raw::c_int) as isize);
        if 0 as std::os::raw::c_int == strcmp((*e).filename.as_mut_ptr(), filename) {
            return e;
        }
        i = (*e).hash_next
    }
    if add == 0 {
        return 0 as *mut CachedInclude;
    }
    e = tcc_malloc(
        (::std::mem::size_of::<CachedInclude>() as std::os::raw::c_ulong)
            .wrapping_add(strlen(filename)),
    ) as *mut CachedInclude;
    strcpy((*e).filename.as_mut_ptr(), filename);
    (*e).once = 0 as std::os::raw::c_int;
    (*e).ifndef_macro = (*e).once;
    dynarray_add(
        &mut (*s1).cached_includes as *mut *mut *mut CachedInclude as *mut std::os::raw::c_void,
        &mut (*s1).nb_cached_includes,
        e as *mut std::os::raw::c_void,
    );
    /* add in hash table */
    (*e).hash_next = (*s1).cached_includes_hash[h as usize];
    (*s1).cached_includes_hash[h as usize] = (*s1).nb_cached_includes;
    return e;
}
unsafe extern "C" fn pragma_parse(mut s1: *mut TCCState) {
    let mut current_block: u64;
    next_nomacro();
    if tok == TOK_push_macro as std::os::raw::c_int || tok == TOK_pop_macro as std::os::raw::c_int {
        let mut t: std::os::raw::c_int = tok;
        let mut v: std::os::raw::c_int = 0;
        let mut s: *mut Sym = 0 as *mut Sym;
        next();
        if tok != '(' as i32 {
            current_block = 5166041964070998680;
        } else {
            next();
            if tok != 0xc8 as std::os::raw::c_int {
                current_block = 5166041964070998680;
            } else {
                v = (*tok_alloc(
                    tokc.str_0.data as *const std::os::raw::c_char,
                    tokc.str_0.size - 1 as std::os::raw::c_int,
                ))
                .tok;
                next();
                if tok != ')' as i32 {
                    current_block = 5166041964070998680;
                } else {
                    if t == TOK_push_macro as std::os::raw::c_int {
                        loop {
                            s = define_find(v);
                            if !s.is_null() {
                                break;
                            }
                            define_push(
                                v,
                                0 as std::os::raw::c_int,
                                0 as *mut std::os::raw::c_int,
                                0 as *mut Sym,
                            );
                        }
                        (*s).type_0.ref_0 = s
                    /* set push boundary */
                    } else {
                        s = define_stack;
                        while !s.is_null() {
                            if (*s).v == v && (*s).type_0.ref_0 == s {
                                (*s).type_0.ref_0 = 0 as *mut Sym;
                                break;
                            } else {
                                s = (*s).prev
                            }
                        }
                    }
                    if !s.is_null() {
                        let ref mut fresh34 = (**table_ident
                            .offset((v - 256 as std::os::raw::c_int) as isize))
                        .sym_define;
                        *fresh34 = if !(*s).c2rust_unnamed.d.is_null() {
                            s
                        } else {
                            0 as *mut Sym
                        }
                    } else {
                        _tcc_warning(
                            b"unbalanced #pragma pop_macro\x00" as *const u8
                                as *const std::os::raw::c_char,
                        );
                    }
                    pp_debug_tok = t;
                    pp_debug_symv = v;
                    current_block = 777662472977924419;
                }
            }
        }
    } else if tok == TOK_once as std::os::raw::c_int {
        (*search_cached_include(s1, (*file).filename.as_mut_ptr(), 1 as std::os::raw::c_int))
            .once = pp_once;
        current_block = 777662472977924419;
    } else if (*s1).output_type == 5 as std::os::raw::c_int {
        /* tcc -E: keep pragmas below unchanged */
        unget_tok(' ' as i32);
        unget_tok(TOK_PRAGMA as std::os::raw::c_int);
        unget_tok('#' as i32);
        unget_tok(10 as std::os::raw::c_int);
        current_block = 777662472977924419;
    } else if tok == TOK_pack as std::os::raw::c_int {
        /* This may be:
        #pragma pack(1) // set
        #pragma pack() // reset to default
        #pragma pack(push,1) // push & set
        #pragma pack(pop) // restore previous */
        next();
        skip('(' as i32);
        if tok == TOK_ASM_pop as std::os::raw::c_int {
            next();
            if (*s1).pack_stack_ptr <= (*s1).pack_stack.as_mut_ptr() {
                current_block = 10502187178602744141;
            } else {
                (*s1).pack_stack_ptr = (*s1).pack_stack_ptr.offset(-1);
                current_block = 13460095289871124136;
            }
        } else {
            let mut val: std::os::raw::c_int = 0 as std::os::raw::c_int;
            if tok != ')' as i32 {
                if tok == TOK_ASM_push as std::os::raw::c_int {
                    next();
                    if (*s1).pack_stack_ptr
                        >= (*s1)
                            .pack_stack
                            .as_mut_ptr()
                            .offset(8 as std::os::raw::c_int as isize)
                            .offset(-(1 as std::os::raw::c_int as isize))
                    {
                        current_block = 10502187178602744141;
                    } else {
                        (*s1).pack_stack_ptr = (*s1).pack_stack_ptr.offset(1);
                        skip(',' as i32);
                        current_block = 10758786907990354186;
                    }
                } else {
                    current_block = 10758786907990354186;
                }
                match current_block {
                    10502187178602744141 => {},
                    _ => {
                        if tok != 0xc2 as std::os::raw::c_int {
                            current_block = 5166041964070998680;
                        } else {
                            val = tokc.i as std::os::raw::c_int;
                            if val < 1 as std::os::raw::c_int
                                || val > 16 as std::os::raw::c_int
                                || val & val - 1 as std::os::raw::c_int != 0 as std::os::raw::c_int
                            {
                                current_block = 5166041964070998680;
                            } else {
                                next();
                                current_block = 10150597327160359210;
                            }
                        }
                    },
                }
            } else {
                current_block = 10150597327160359210;
            }
            match current_block {
                5166041964070998680 => {},
                10502187178602744141 => {},
                _ => {
                    *(*s1).pack_stack_ptr = val;
                    current_block = 13460095289871124136;
                },
            }
        }
        match current_block {
            5166041964070998680 => {},
            _ => match current_block {
                10502187178602744141 => {
                    _tcc_error(
                        b"out of pack stack\x00" as *const u8 as *const std::os::raw::c_char,
                    );
                },
                _ => {
                    if tok != ')' as i32 {
                        current_block = 5166041964070998680;
                    } else {
                        current_block = 777662472977924419;
                    }
                },
            },
        }
    } else if tok == TOK_comment as std::os::raw::c_int {
        let mut p: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
        let mut t_0: std::os::raw::c_int = 0;
        next();
        skip('(' as i32);
        t_0 = tok;
        next();
        skip(',' as i32);
        if tok != 0xc8 as std::os::raw::c_int {
            current_block = 5166041964070998680;
        } else {
            p = tcc_strdup(tokc.str_0.data as *mut std::os::raw::c_char);
            next();
            if tok != ')' as i32 {
                current_block = 5166041964070998680;
            } else {
                if t_0 == TOK_lib as std::os::raw::c_int {
                    dynarray_add(
                        &mut (*s1).pragma_libs as *mut *mut *mut std::os::raw::c_char
                            as *mut std::os::raw::c_void,
                        &mut (*s1).nb_pragma_libs,
                        p as *mut std::os::raw::c_void,
                    );
                } else {
                    if t_0 == TOK_option as std::os::raw::c_int {
                        tcc_set_options(s1, p);
                    }
                    tcc_free(p as *mut std::os::raw::c_void);
                }
                current_block = 777662472977924419;
            }
        }
    } else {
        if (*s1).warn_unsupported != 0 {
            _tcc_warning(
                b"#pragma %s is ignored\x00" as *const u8 as *const std::os::raw::c_char,
                get_tok_str(tok, &mut tokc),
            );
        }
        current_block = 777662472977924419;
    }
    match current_block {
        5166041964070998680 => {
            _tcc_error(
                b"malformed #pragma directive\x00" as *const u8 as *const std::os::raw::c_char,
            );
        },
        _ => return,
    };
}
/* is_bof is true if first non space token at beginning of file */
#[no_mangle]
pub unsafe extern "C" fn preprocess(mut is_bof: std::os::raw::c_int) {
    let mut current_block: u64;
    let mut s1: *mut TCCState = tcc_state;
    let mut i: std::os::raw::c_int = 0;
    let mut c: std::os::raw::c_int = 0;
    let mut n: std::os::raw::c_int = 0;
    let mut saved_parse_flags: std::os::raw::c_int = 0;
    let mut buf: [std::os::raw::c_char; 1024] = [0; 1024];
    let mut q: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut s: *mut Sym = 0 as *mut Sym;
    saved_parse_flags = parse_flags;
    parse_flags = 0x1 as std::os::raw::c_int
        | 0x2 as std::os::raw::c_int
        | 0x40 as std::os::raw::c_int
        | 0x4 as std::os::raw::c_int
        | parse_flags & 0x8 as std::os::raw::c_int;
    next_nomacro();
    loop {
        match tok {
            318 => {
                pp_debug_tok = tok;
                next_nomacro();
                pp_debug_symv = tok;
                parse_define();
                current_block = 242220637564940144;
                break;
            },
            326 => {
                pp_debug_tok = tok;
                next_nomacro();
                pp_debug_symv = tok;
                s = define_find(tok);
                /* undefine symbol by putting an invalid name */
                if !s.is_null() {
                    define_undef(s);
                }
                current_block = 242220637564940144;
                break;
            },
            319 | 320 => {
                ch = *(*file).buf_ptr.offset(0 as std::os::raw::c_int as isize)
                    as std::os::raw::c_int;
                /* XXX: incorrect if comments : use next_nomacro with a special mode */
                skip_spaces();
                let mut current_block_43: u64;
                if ch == '<' as i32 {
                    c = '>' as i32;
                    current_block_43 = 9177663572014039438;
                } else if ch == '\"' as i32 {
                    c = ch;
                    current_block_43 = 9177663572014039438;
                } else {
                    let mut len: std::os::raw::c_int = 0;
                    /* computed #include : concatenate everything up to linefeed,
                    the result must be one of the two accepted forms.
                    Don't convert pp-tokens to tokens here.  */
                    parse_flags = 0x1 as std::os::raw::c_int
                        | 0x4 as std::os::raw::c_int
                        | parse_flags & 0x8 as std::os::raw::c_int;
                    next();
                    buf[0 as std::os::raw::c_int as usize] = '\u{0}' as i32 as std::os::raw::c_char;
                    while tok != 10 as std::os::raw::c_int {
                        pstrcat(
                            buf.as_mut_ptr(),
                            ::std::mem::size_of::<[std::os::raw::c_char; 1024]>()
                                as std::os::raw::c_ulong,
                            get_tok_str(tok, &mut tokc),
                        );
                        next();
                    }
                    len = strlen(buf.as_mut_ptr()) as std::os::raw::c_int;
                    /* check syntax and remove '<>|""' */
                    if len < 2 as std::os::raw::c_int
                        || (buf[0 as std::os::raw::c_int as usize] as std::os::raw::c_int
                            != '\"' as i32
                            || buf[(len - 1 as std::os::raw::c_int) as usize]
                                as std::os::raw::c_int
                                != '\"' as i32)
                            && (buf[0 as std::os::raw::c_int as usize] as std::os::raw::c_int
                                != '<' as i32
                                || buf[(len - 1 as std::os::raw::c_int) as usize]
                                    as std::os::raw::c_int
                                    != '>' as i32)
                    {
                        _tcc_error(
                            b"\'#include\' expects \"FILENAME\" or <FILENAME>\x00" as *const u8
                                as *const std::os::raw::c_char,
                        );
                    }
                    c = buf[(len - 1 as std::os::raw::c_int) as usize] as std::os::raw::c_int;
                    memmove(
                        buf.as_mut_ptr() as *mut std::os::raw::c_void,
                        buf.as_mut_ptr().offset(1 as std::os::raw::c_int as isize)
                            as *const std::os::raw::c_void,
                        (len - 2 as std::os::raw::c_int) as std::os::raw::c_ulong,
                    );
                    buf[(len - 2 as std::os::raw::c_int) as usize] =
                        '\u{0}' as i32 as std::os::raw::c_char;
                    current_block_43 = 10399321362245223758;
                }
                match current_block_43 {
                    9177663572014039438 => {
                        inp();
                        q = buf.as_mut_ptr();
                        while ch != c && ch != '\n' as i32 && ch != -(1 as std::os::raw::c_int) {
                            if (q.offset_from(buf.as_mut_ptr()) as std::os::raw::c_long
                                as std::os::raw::c_ulong)
                                < (::std::mem::size_of::<[std::os::raw::c_char; 1024]>()
                                    as std::os::raw::c_ulong)
                                    .wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong)
                            {
                                let fresh35 = q;
                                q = q.offset(1);
                                *fresh35 = ch as std::os::raw::c_char
                            }
                            if ch == '\\' as i32 {
                                if handle_stray_noerror() == 0 as std::os::raw::c_int {
                                    q = q.offset(-1)
                                }
                            } else {
                                inp();
                            }
                        }
                        *q = '\u{0}' as i32 as std::os::raw::c_char;
                        minp();
                    },
                    _ => {},
                }
                if (*s1).include_stack_ptr
                    >= (*s1)
                        .include_stack
                        .as_mut_ptr()
                        .offset(32 as std::os::raw::c_int as isize)
                {
                    _tcc_error(
                        b"#include recursion too deep\x00" as *const u8
                            as *const std::os::raw::c_char,
                    );
                }
                i = if tok == TOK_INCLUDE_NEXT as std::os::raw::c_int {
                    ((*file).include_next_index) + 1 as std::os::raw::c_int
                } else {
                    0 as std::os::raw::c_int
                };
                n = 2 as std::os::raw::c_int + (*s1).nb_include_paths + (*s1).nb_sysinclude_paths;
                current_block = 7990025728955927862;
                break;
            },
            322 => {
                c = 1 as std::os::raw::c_int;
                current_block = 18391292360981809970;
            },
            259 => {
                c = expr_preprocess();
                current_block = 10307739472820856892;
            },
            321 => {
                c = 0 as std::os::raw::c_int;
                current_block = 18391292360981809970;
            },
            260 => {
                if (*s1).ifdef_stack_ptr == (*s1).ifdef_stack.as_mut_ptr() {
                    _tcc_error(
                        b"#else without matching #if\x00" as *const u8
                            as *const std::os::raw::c_char,
                    );
                }
                if *(*s1)
                    .ifdef_stack_ptr
                    .offset(-(1 as std::os::raw::c_int) as isize)
                    & 2 as std::os::raw::c_int
                    != 0
                {
                    _tcc_error(
                        b"#else after #else\x00" as *const u8 as *const std::os::raw::c_char,
                    );
                }
                let ref mut fresh38 = *(*s1)
                    .ifdef_stack_ptr
                    .offset(-(1 as std::os::raw::c_int) as isize);
                *fresh38 ^= 3 as std::os::raw::c_int;
                c = *fresh38;
                current_block = 5490365314095407341;
            },
            323 => {
                if (*s1).ifdef_stack_ptr == (*s1).ifdef_stack.as_mut_ptr() {
                    _tcc_error(
                        b"#elif without matching #if\x00" as *const u8
                            as *const std::os::raw::c_char,
                    );
                }
                c = *(*s1)
                    .ifdef_stack_ptr
                    .offset(-(1 as std::os::raw::c_int) as isize);
                if c > 1 as std::os::raw::c_int {
                    _tcc_error(
                        b"#elif after #else\x00" as *const u8 as *const std::os::raw::c_char,
                    );
                }
                /* last #if/#elif expression was true: we skip */
                if c == 1 as std::os::raw::c_int {
                    c = 0 as std::os::raw::c_int
                } else {
                    c = expr_preprocess();
                    *(*s1)
                        .ifdef_stack_ptr
                        .offset(-(1 as std::os::raw::c_int) as isize) = c
                }
                current_block = 5490365314095407341;
            },
            324 => {
                if (*s1).ifdef_stack_ptr <= (*file).ifdef_stack_ptr {
                    _tcc_error(
                        b"#endif without matching #if\x00" as *const u8
                            as *const std::os::raw::c_char,
                    );
                }
                (*s1).ifdef_stack_ptr = (*s1).ifdef_stack_ptr.offset(-1);
                /* '#ifndef macro' was at the start of file. Now we check if
                an '#endif' is exactly at the end of file */
                if (*file).ifndef_macro != 0 && (*s1).ifdef_stack_ptr == (*file).ifdef_stack_ptr {
                    current_block = 10241167629170301496;
                    break;
                } else {
                    current_block = 242220637564940144;
                    break;
                }
            },
            205 => {
                n = strtoul(
                    tokc.str_0.data as *mut std::os::raw::c_char,
                    &mut q,
                    10 as std::os::raw::c_int,
                ) as std::os::raw::c_int;
                current_block = 1754105723715427567;
                break;
            },
            329 => {
                next();
                if tok != 0xc2 as std::os::raw::c_int {
                    current_block = 4168664641820735757;
                    break;
                } else {
                    current_block = 7923086311623215889;
                    break;
                }
            },
            327 | 328 => {
                c = tok;
                ch = *(*file).buf_ptr.offset(0 as std::os::raw::c_int as isize)
                    as std::os::raw::c_int;
                skip_spaces();
                q = buf.as_mut_ptr();
                while ch != '\n' as i32 && ch != -(1 as std::os::raw::c_int) {
                    if (q.offset_from(buf.as_mut_ptr()) as std::os::raw::c_long
                        as std::os::raw::c_ulong)
                        < (::std::mem::size_of::<[std::os::raw::c_char; 1024]>()
                            as std::os::raw::c_ulong)
                            .wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong)
                    {
                        let fresh39 = q;
                        q = q.offset(1);
                        *fresh39 = ch as std::os::raw::c_char
                    }
                    if ch == '\\' as i32 {
                        if handle_stray_noerror() == 0 as std::os::raw::c_int {
                            q = q.offset(-1)
                        }
                    } else {
                        inp();
                    }
                }
                *q = '\u{0}' as i32 as std::os::raw::c_char;
                if c == TOK_ERROR as std::os::raw::c_int {
                    _tcc_error(
                        b"#error %s\x00" as *const u8 as *const std::os::raw::c_char,
                        buf.as_mut_ptr(),
                    );
                } else {
                    _tcc_warning(
                        b"#warning %s\x00" as *const u8 as *const std::os::raw::c_char,
                        buf.as_mut_ptr(),
                    );
                }
                current_block = 242220637564940144;
                break;
            },
            330 => {
                pragma_parse(s1);
                current_block = 242220637564940144;
                break;
            },
            10 => {
                current_block = 9292038568874564400;
                break;
            },
            _ => {
                /* ignore gas line comment in an 'S' file. */
                if saved_parse_flags & 0x8 as std::os::raw::c_int != 0 {
                    current_block = 246964916090906403;
                    break;
                } else {
                    current_block = 13479157322803929894;
                    break;
                }
            },
        }
        match current_block {
            18391292360981809970 => {
                next_nomacro();
                if tok < 256 as std::os::raw::c_int {
                    _tcc_error(
                        b"invalid argument for \'#if%sdef\'\x00" as *const u8
                            as *const std::os::raw::c_char,
                        if c != 0 {
                            b"n\x00" as *const u8 as *const std::os::raw::c_char
                        } else {
                            b"\x00" as *const u8 as *const std::os::raw::c_char
                        },
                    );
                }
                if is_bof != 0 {
                    if c != 0 {
                        (*file).ifndef_macro = tok
                    }
                }
                c = (define_find(tok) != 0 as *mut Sym) as std::os::raw::c_int ^ c;
                current_block = 10307739472820856892;
            },
            5490365314095407341 => {
                if (*s1).ifdef_stack_ptr
                    == (*file)
                        .ifdef_stack_ptr
                        .offset(1 as std::os::raw::c_int as isize)
                {
                    (*file).ifndef_macro = 0 as std::os::raw::c_int
                }
                current_block = 18065791840937293542;
            },
            _ => {},
        }
        match current_block {
            10307739472820856892 => {
                if (*s1).ifdef_stack_ptr
                    >= (*s1)
                        .ifdef_stack
                        .as_mut_ptr()
                        .offset(64 as std::os::raw::c_int as isize)
                {
                    _tcc_error(
                        b"memory full (ifdef)\x00" as *const u8 as *const std::os::raw::c_char,
                    );
                }
                let fresh37 = (*s1).ifdef_stack_ptr;
                (*s1).ifdef_stack_ptr = (*s1).ifdef_stack_ptr.offset(1);
                *fresh37 = c
            },
            _ => {},
        }
        if !(c & 1 as std::os::raw::c_int == 0) {
            current_block = 242220637564940144;
            break;
        }
        preprocess_skip();
        is_bof = 0 as std::os::raw::c_int
    }
    match current_block {
        7990025728955927862 => {
            loop {
                if !(i < n) {
                    current_block = 9705665520141849625;
                    break;
                }
                let mut buf1: [std::os::raw::c_char; 1024] = [0; 1024];
                let mut e: *mut CachedInclude = 0 as *mut CachedInclude;
                let mut path: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
                if i == 0 as std::os::raw::c_int {
                    /* check absolute include path */
                    if !(buf[0 as std::os::raw::c_int as usize] as std::os::raw::c_int
                        == '/' as i32)
                    {
                        current_block = 11777552016271000781;
                    } else {
                        buf1[0 as std::os::raw::c_int as usize] =
                            0 as std::os::raw::c_int as std::os::raw::c_char;
                        current_block = 7494008139977416618;
                    }
                } else if i == 1 as std::os::raw::c_int {
                    /* search in file's dir if "header.h" */
                    if c != '\"' as i32 {
                        current_block = 11777552016271000781;
                    } else {
                        /* https://savannah.nongnu.org/bugs/index.php?50847 */
                        path = (*file).true_filename;
                        pstrncpy(
                            buf1.as_mut_ptr(),
                            path,
                            tcc_basename(path).offset_from(path) as std::os::raw::c_long as size_t,
                        );
                        current_block = 7494008139977416618;
                    }
                } else {
                    /* search in all the include paths */
                    let mut j: std::os::raw::c_int = i - 2 as std::os::raw::c_int;
                    let mut k: std::os::raw::c_int = j - (*s1).nb_include_paths;
                    path = if k < 0 as std::os::raw::c_int {
                        *(*s1).include_paths.offset(j as isize)
                    } else {
                        *(*s1).sysinclude_paths.offset(k as isize)
                    };
                    pstrcpy(
                        buf1.as_mut_ptr(),
                        ::std::mem::size_of::<[std::os::raw::c_char; 1024]>()
                            as std::os::raw::c_ulong,
                        path,
                    );
                    pstrcat(
                        buf1.as_mut_ptr(),
                        ::std::mem::size_of::<[std::os::raw::c_char; 1024]>()
                            as std::os::raw::c_ulong,
                        b"/\x00" as *const u8 as *const std::os::raw::c_char,
                    );
                    current_block = 7494008139977416618;
                }
                match current_block {
                    7494008139977416618 => {
                        pstrcat(
                            buf1.as_mut_ptr(),
                            ::std::mem::size_of::<[std::os::raw::c_char; 1024]>()
                                as std::os::raw::c_ulong,
                            buf.as_mut_ptr(),
                        );
                        e = search_cached_include(s1, buf1.as_mut_ptr(), 0 as std::os::raw::c_int);
                        if !e.is_null()
                            && (!define_find((*e).ifndef_macro).is_null() || (*e).once == pp_once)
                        {
                            current_block = 242220637564940144;
                            break;
                        }
                        if !(tcc_open(s1, buf1.as_mut_ptr()) < 0 as std::os::raw::c_int) {
                            /* push previous file on stack */
                            let fresh36 = (*s1).include_stack_ptr;
                            (*s1).include_stack_ptr = (*s1).include_stack_ptr.offset(1);
                            *fresh36 = (*file).prev;
                            (*file).include_next_index = i;
                            /* update target deps */
                            if (*s1).gen_deps != 0 {
                                let mut bf: *mut BufferedFile = file;
                                while i == 1 as std::os::raw::c_int && {
                                    bf = (*bf).prev;
                                    !bf.is_null()
                                } {
                                    i = (*bf).include_next_index
                                }
                                /* skip system include files */
                                if n - i > (*s1).nb_sysinclude_paths {
                                    dynarray_add(
                                        &mut (*s1).target_deps
                                            as *mut *mut *mut std::os::raw::c_char
                                            as *mut std::os::raw::c_void,
                                        &mut (*s1).nb_target_deps,
                                        tcc_strdup(buf1.as_mut_ptr()) as *mut std::os::raw::c_void,
                                    );
                                }
                            }
                            /* add include file debug info */
                            tcc_debug_bincl(tcc_state);
                            tok_flags |= 0x2 as std::os::raw::c_int | 0x1 as std::os::raw::c_int;
                            ch = *(*file).buf_ptr.offset(0 as std::os::raw::c_int as isize)
                                as std::os::raw::c_int;
                            current_block = 9292038568874564400;
                            break;
                        }
                    },
                    _ => {},
                }
                i += 1
            }
            match current_block {
                242220637564940144 => {},
                9292038568874564400 => {},
                _ => {
                    _tcc_error(
                        b"include file \'%s\' not found\x00" as *const u8
                            as *const std::os::raw::c_char,
                        buf.as_mut_ptr(),
                    );
                },
            }
        },
        10241167629170301496 => {
            (*file).ifndef_macro_saved = (*file).ifndef_macro;
            /* need to set to zero to avoid false matches if another
            #ifndef at middle of file */
            (*file).ifndef_macro = 0 as std::os::raw::c_int;
            while tok != 10 as std::os::raw::c_int {
                next_nomacro();
            }
            tok_flags |= 0x4 as std::os::raw::c_int;
            current_block = 9292038568874564400;
        },
        13479157322803929894 => {
            if tok == '!' as i32 && is_bof != 0 {
                current_block = 246964916090906403;
            } else {
                _tcc_warning(
                    b"Ignoring unknown preprocessing directive #%s\x00" as *const u8
                        as *const std::os::raw::c_char,
                    get_tok_str(tok, &mut tokc),
                );
                current_block = 246964916090906403;
            }
        },
        7923086311623215889 => {
            n = tokc.i as std::os::raw::c_int;
            current_block = 1754105723715427567;
        },
        _ => {},
    }
    match current_block {
        1754105723715427567 => {
            next();
            if tok != 10 as std::os::raw::c_int {
                if tok == 0xc8 as std::os::raw::c_int {
                    if (*file).true_filename == (*file).filename.as_mut_ptr() {
                        (*file).true_filename = tcc_strdup((*file).filename.as_mut_ptr())
                    }
                    /* prepend directory from real file */
                    pstrcpy(
                        buf.as_mut_ptr(),
                        ::std::mem::size_of::<[std::os::raw::c_char; 1024]>()
                            as std::os::raw::c_ulong,
                        (*file).true_filename,
                    );
                    *tcc_basename(buf.as_mut_ptr()) =
                        0 as std::os::raw::c_int as std::os::raw::c_char;
                    pstrcat(
                        buf.as_mut_ptr(),
                        ::std::mem::size_of::<[std::os::raw::c_char; 1024]>()
                            as std::os::raw::c_ulong,
                        tokc.str_0.data as *mut std::os::raw::c_char,
                    );
                    tcc_debug_putfile(s1, buf.as_mut_ptr());
                    n -= 1;
                    current_block = 4871270227279186910;
                } else if parse_flags & 0x8 as std::os::raw::c_int != 0 {
                    current_block = 242220637564940144;
                } else {
                    current_block = 4168664641820735757;
                }
            } else {
                current_block = 4871270227279186910;
            }
            match current_block {
                242220637564940144 => {},
                4168664641820735757 => {},
                _ => {
                    if (*file).fd > 0 as std::os::raw::c_int {
                        (*tcc_state).total_lines += (*file).line_num - n
                    }
                    (*file).line_num = n;
                    current_block = 242220637564940144;
                },
            }
        },
        246964916090906403 =>
        /* '!' is ignored at beginning to allow C scripts. */
        {
            (*file).buf_ptr =
                parse_line_comment((*file).buf_ptr.offset(-(1 as std::os::raw::c_int as isize)));
            current_block = 9292038568874564400;
        }
        _ => {},
    }
    match current_block {
        242220637564940144 =>
        /* no need to parse the include because the 'ifndef macro'
                   is defined (or had #pragma once) */
        /* ignore other preprocess commands or #! for C scripts */
        {
            while tok != 10 as std::os::raw::c_int {
                next_nomacro();
            }
        },
        4168664641820735757 => {
            _tcc_error(b"wrong #line format\x00" as *const u8 as *const std::os::raw::c_char);
        },
        _ => {},
    }
    parse_flags = saved_parse_flags;
}
/* evaluate escape codes in a string. */
unsafe extern "C" fn parse_escape_string(
    mut outstr: *mut CString,
    mut buf: *const uint8_t,
    mut is_long: std::os::raw::c_int,
) {
    let mut current_block: u64;
    let mut c: std::os::raw::c_int = 0;
    let mut n: std::os::raw::c_int = 0;
    let mut i: std::os::raw::c_int = 0;
    let mut p: *const uint8_t = 0 as *const uint8_t;
    p = buf;
    loop {
        c = *p as std::os::raw::c_int;
        if c == '\u{0}' as i32 {
            break;
        }
        if c == '\\' as i32 {
            p = p.offset(1);
            /* escape */
            c = *p as std::os::raw::c_int;
            match c {
                48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 => {
                    current_block = 15715630227694234064;
                    match current_block {
                        15715630227694234064 => {
                            /* at most three octal digits */
                            n = c - '0' as i32;
                            p = p.offset(1);
                            c = *p as std::os::raw::c_int;
                            if isoct(c) != 0 {
                                n = n * 8 as std::os::raw::c_int + c - '0' as i32;
                                p = p.offset(1);
                                c = *p as std::os::raw::c_int;
                                if isoct(c) != 0 {
                                    n = n * 8 as std::os::raw::c_int + c - '0' as i32;
                                    p = p.offset(1)
                                }
                            }
                            c = n;
                            current_block = 8297889085312008496;
                        },
                        3761358111973716264 => {
                            if (*tcc_state).gnu_ext == 0 {
                                current_block = 18043221310185898469;
                            } else {
                                c = 27 as std::os::raw::c_int;
                                current_block = 562309032768341766;
                            }
                        },
                        111971726257118000 => {
                            i = 0 as std::os::raw::c_int;
                            current_block = 1923476342031941186;
                        },
                        8335422270325910153 => {
                            i = 4 as std::os::raw::c_int;
                            current_block = 1923476342031941186;
                        },
                        14754185658451558945 => {
                            i = 8 as std::os::raw::c_int;
                            current_block = 1923476342031941186;
                        },
                        7388311901245426766 => {
                            c = '\u{7}' as i32;
                            current_block = 562309032768341766;
                        },
                        16704339345470113457 => {
                            c = '\u{8}' as i32;
                            current_block = 562309032768341766;
                        },
                        8070590502297169425 => {
                            c = '\u{c}' as i32;
                            current_block = 562309032768341766;
                        },
                        7068460829993434719 => {
                            c = '\n' as i32;
                            current_block = 562309032768341766;
                        },
                        5140500990525381484 => {
                            c = '\r' as i32;
                            current_block = 562309032768341766;
                        },
                        11484286114940466897 => {
                            c = '\t' as i32;
                            current_block = 562309032768341766;
                        },
                        2642875140583231469 => {
                            c = '\u{b}' as i32;
                            current_block = 562309032768341766;
                        },
                        _ => {},
                    }
                    match current_block {
                        8297889085312008496 => {},
                        562309032768341766 => {},
                        _ => match current_block {
                            18043221310185898469 => {
                                if c >= '!' as i32 && c <= '~' as i32 {
                                    _tcc_warning(
                                        b"unknown escape sequence: \'\\%c\'\x00" as *const u8
                                            as *const std::os::raw::c_char,
                                        c,
                                    );
                                } else {
                                    _tcc_warning(
                                        b"unknown escape sequence: \'\\x%x\'\x00" as *const u8
                                            as *const std::os::raw::c_char,
                                        c,
                                    );
                                }
                                current_block = 562309032768341766;
                            },
                            _ => {
                                p = p.offset(1);
                                n = 0 as std::os::raw::c_int;
                                loop {
                                    c = *p as std::os::raw::c_int;
                                    if c >= 'a' as i32 && c <= 'f' as i32 {
                                        c = c - 'a' as i32 + 10 as std::os::raw::c_int
                                    } else if c >= 'A' as i32 && c <= 'F' as i32 {
                                        c = c - 'A' as i32 + 10 as std::os::raw::c_int
                                    } else if isnum(c) != 0 {
                                        c = c - '0' as i32
                                    } else if i > 0 as std::os::raw::c_int {
                                        expect(
                                            b"more hex digits in universal-character-name\x00"
                                                as *const u8
                                                as *const std::os::raw::c_char,
                                        );
                                    } else {
                                        c = n;
                                        current_block = 8297889085312008496;
                                        break;
                                    }
                                    n = n * 16 as std::os::raw::c_int + c;
                                    p = p.offset(1);
                                    i -= 1;
                                    if !(i != 0) {
                                        current_block = 3222590281903869779;
                                        break;
                                    }
                                }
                                match current_block {
                                    8297889085312008496 => {},
                                    _ => {
                                        cstr_u8cat(outstr, n);
                                        continue;
                                    },
                                }
                            },
                        },
                    }
                },
                120 => {
                    current_block = 111971726257118000;
                    match current_block {
                        15715630227694234064 => {
                            n = c - '0' as i32;
                            p = p.offset(1);
                            c = *p as std::os::raw::c_int;
                            if isoct(c) != 0 {
                                n = n * 8 as std::os::raw::c_int + c - '0' as i32;
                                p = p.offset(1);
                                c = *p as std::os::raw::c_int;
                                if isoct(c) != 0 {
                                    n = n * 8 as std::os::raw::c_int + c - '0' as i32;
                                    p = p.offset(1)
                                }
                            }
                            c = n;
                            current_block = 8297889085312008496;
                        },
                        3761358111973716264 => {
                            if (*tcc_state).gnu_ext == 0 {
                                current_block = 18043221310185898469;
                            } else {
                                c = 27 as std::os::raw::c_int;
                                current_block = 562309032768341766;
                            }
                        },
                        111971726257118000 => {
                            i = 0 as std::os::raw::c_int;
                            current_block = 1923476342031941186;
                        },
                        8335422270325910153 => {
                            i = 4 as std::os::raw::c_int;
                            current_block = 1923476342031941186;
                        },
                        14754185658451558945 => {
                            i = 8 as std::os::raw::c_int;
                            current_block = 1923476342031941186;
                        },
                        7388311901245426766 => {
                            c = '\u{7}' as i32;
                            current_block = 562309032768341766;
                        },
                        16704339345470113457 => {
                            c = '\u{8}' as i32;
                            current_block = 562309032768341766;
                        },
                        8070590502297169425 => {
                            c = '\u{c}' as i32;
                            current_block = 562309032768341766;
                        },
                        7068460829993434719 => {
                            c = '\n' as i32;
                            current_block = 562309032768341766;
                        },
                        5140500990525381484 => {
                            c = '\r' as i32;
                            current_block = 562309032768341766;
                        },
                        11484286114940466897 => {
                            c = '\t' as i32;
                            current_block = 562309032768341766;
                        },
                        2642875140583231469 => {
                            c = '\u{b}' as i32;
                            current_block = 562309032768341766;
                        },
                        _ => {},
                    }
                    match current_block {
                        8297889085312008496 => {},
                        562309032768341766 => {},
                        _ => match current_block {
                            18043221310185898469 => {
                                if c >= '!' as i32 && c <= '~' as i32 {
                                    _tcc_warning(
                                        b"unknown escape sequence: \'\\%c\'\x00" as *const u8
                                            as *const std::os::raw::c_char,
                                        c,
                                    );
                                } else {
                                    _tcc_warning(
                                        b"unknown escape sequence: \'\\x%x\'\x00" as *const u8
                                            as *const std::os::raw::c_char,
                                        c,
                                    );
                                }
                                current_block = 562309032768341766;
                            },
                            _ => {
                                p = p.offset(1);
                                n = 0 as std::os::raw::c_int;
                                loop {
                                    c = *p as std::os::raw::c_int;
                                    if c >= 'a' as i32 && c <= 'f' as i32 {
                                        c = c - 'a' as i32 + 10 as std::os::raw::c_int
                                    } else if c >= 'A' as i32 && c <= 'F' as i32 {
                                        c = c - 'A' as i32 + 10 as std::os::raw::c_int
                                    } else if isnum(c) != 0 {
                                        c = c - '0' as i32
                                    } else if i > 0 as std::os::raw::c_int {
                                        expect(
                                            b"more hex digits in universal-character-name\x00"
                                                as *const u8
                                                as *const std::os::raw::c_char,
                                        );
                                    } else {
                                        c = n;
                                        current_block = 8297889085312008496;
                                        break;
                                    }
                                    n = n * 16 as std::os::raw::c_int + c;
                                    p = p.offset(1);
                                    i -= 1;
                                    if !(i != 0) {
                                        current_block = 3222590281903869779;
                                        break;
                                    }
                                }
                                match current_block {
                                    8297889085312008496 => {},
                                    _ => {
                                        cstr_u8cat(outstr, n);
                                        continue;
                                    },
                                }
                            },
                        },
                    }
                },
                117 => {
                    current_block = 8335422270325910153;
                    match current_block {
                        15715630227694234064 => {
                            n = c - '0' as i32;
                            p = p.offset(1);
                            c = *p as std::os::raw::c_int;
                            if isoct(c) != 0 {
                                n = n * 8 as std::os::raw::c_int + c - '0' as i32;
                                p = p.offset(1);
                                c = *p as std::os::raw::c_int;
                                if isoct(c) != 0 {
                                    n = n * 8 as std::os::raw::c_int + c - '0' as i32;
                                    p = p.offset(1)
                                }
                            }
                            c = n;
                            current_block = 8297889085312008496;
                        },
                        3761358111973716264 => {
                            if (*tcc_state).gnu_ext == 0 {
                                current_block = 18043221310185898469;
                            } else {
                                c = 27 as std::os::raw::c_int;
                                current_block = 562309032768341766;
                            }
                        },
                        111971726257118000 => {
                            i = 0 as std::os::raw::c_int;
                            current_block = 1923476342031941186;
                        },
                        8335422270325910153 => {
                            i = 4 as std::os::raw::c_int;
                            current_block = 1923476342031941186;
                        },
                        14754185658451558945 => {
                            i = 8 as std::os::raw::c_int;
                            current_block = 1923476342031941186;
                        },
                        7388311901245426766 => {
                            c = '\u{7}' as i32;
                            current_block = 562309032768341766;
                        },
                        16704339345470113457 => {
                            c = '\u{8}' as i32;
                            current_block = 562309032768341766;
                        },
                        8070590502297169425 => {
                            c = '\u{c}' as i32;
                            current_block = 562309032768341766;
                        },
                        7068460829993434719 => {
                            c = '\n' as i32;
                            current_block = 562309032768341766;
                        },
                        5140500990525381484 => {
                            c = '\r' as i32;
                            current_block = 562309032768341766;
                        },
                        11484286114940466897 => {
                            c = '\t' as i32;
                            current_block = 562309032768341766;
                        },
                        2642875140583231469 => {
                            c = '\u{b}' as i32;
                            current_block = 562309032768341766;
                        },
                        _ => {},
                    }
                    match current_block {
                        8297889085312008496 => {},
                        562309032768341766 => {},
                        _ => match current_block {
                            18043221310185898469 => {
                                if c >= '!' as i32 && c <= '~' as i32 {
                                    _tcc_warning(
                                        b"unknown escape sequence: \'\\%c\'\x00" as *const u8
                                            as *const std::os::raw::c_char,
                                        c,
                                    );
                                } else {
                                    _tcc_warning(
                                        b"unknown escape sequence: \'\\x%x\'\x00" as *const u8
                                            as *const std::os::raw::c_char,
                                        c,
                                    );
                                }
                                current_block = 562309032768341766;
                            },
                            _ => {
                                p = p.offset(1);
                                n = 0 as std::os::raw::c_int;
                                loop {
                                    c = *p as std::os::raw::c_int;
                                    if c >= 'a' as i32 && c <= 'f' as i32 {
                                        c = c - 'a' as i32 + 10 as std::os::raw::c_int
                                    } else if c >= 'A' as i32 && c <= 'F' as i32 {
                                        c = c - 'A' as i32 + 10 as std::os::raw::c_int
                                    } else if isnum(c) != 0 {
                                        c = c - '0' as i32
                                    } else if i > 0 as std::os::raw::c_int {
                                        expect(
                                            b"more hex digits in universal-character-name\x00"
                                                as *const u8
                                                as *const std::os::raw::c_char,
                                        );
                                    } else {
                                        c = n;
                                        current_block = 8297889085312008496;
                                        break;
                                    }
                                    n = n * 16 as std::os::raw::c_int + c;
                                    p = p.offset(1);
                                    i -= 1;
                                    if !(i != 0) {
                                        current_block = 3222590281903869779;
                                        break;
                                    }
                                }
                                match current_block {
                                    8297889085312008496 => {},
                                    _ => {
                                        cstr_u8cat(outstr, n);
                                        continue;
                                    },
                                }
                            },
                        },
                    }
                },
                85 => {
                    current_block = 14754185658451558945;
                    match current_block {
                        15715630227694234064 => {
                            n = c - '0' as i32;
                            p = p.offset(1);
                            c = *p as std::os::raw::c_int;
                            if isoct(c) != 0 {
                                n = n * 8 as std::os::raw::c_int + c - '0' as i32;
                                p = p.offset(1);
                                c = *p as std::os::raw::c_int;
                                if isoct(c) != 0 {
                                    n = n * 8 as std::os::raw::c_int + c - '0' as i32;
                                    p = p.offset(1)
                                }
                            }
                            c = n;
                            current_block = 8297889085312008496;
                        },
                        3761358111973716264 => {
                            if (*tcc_state).gnu_ext == 0 {
                                current_block = 18043221310185898469;
                            } else {
                                c = 27 as std::os::raw::c_int;
                                current_block = 562309032768341766;
                            }
                        },
                        111971726257118000 => {
                            i = 0 as std::os::raw::c_int;
                            current_block = 1923476342031941186;
                        },
                        8335422270325910153 => {
                            i = 4 as std::os::raw::c_int;
                            current_block = 1923476342031941186;
                        },
                        14754185658451558945 => {
                            i = 8 as std::os::raw::c_int;
                            current_block = 1923476342031941186;
                        },
                        7388311901245426766 => {
                            c = '\u{7}' as i32;
                            current_block = 562309032768341766;
                        },
                        16704339345470113457 => {
                            c = '\u{8}' as i32;
                            current_block = 562309032768341766;
                        },
                        8070590502297169425 => {
                            c = '\u{c}' as i32;
                            current_block = 562309032768341766;
                        },
                        7068460829993434719 => {
                            c = '\n' as i32;
                            current_block = 562309032768341766;
                        },
                        5140500990525381484 => {
                            c = '\r' as i32;
                            current_block = 562309032768341766;
                        },
                        11484286114940466897 => {
                            c = '\t' as i32;
                            current_block = 562309032768341766;
                        },
                        2642875140583231469 => {
                            c = '\u{b}' as i32;
                            current_block = 562309032768341766;
                        },
                        _ => {},
                    }
                    match current_block {
                        8297889085312008496 => {},
                        562309032768341766 => {},
                        _ => match current_block {
                            18043221310185898469 => {
                                if c >= '!' as i32 && c <= '~' as i32 {
                                    _tcc_warning(
                                        b"unknown escape sequence: \'\\%c\'\x00" as *const u8
                                            as *const std::os::raw::c_char,
                                        c,
                                    );
                                } else {
                                    _tcc_warning(
                                        b"unknown escape sequence: \'\\x%x\'\x00" as *const u8
                                            as *const std::os::raw::c_char,
                                        c,
                                    );
                                }
                                current_block = 562309032768341766;
                            },
                            _ => {
                                p = p.offset(1);
                                n = 0 as std::os::raw::c_int;
                                loop {
                                    c = *p as std::os::raw::c_int;
                                    if c >= 'a' as i32 && c <= 'f' as i32 {
                                        c = c - 'a' as i32 + 10 as std::os::raw::c_int
                                    } else if c >= 'A' as i32 && c <= 'F' as i32 {
                                        c = c - 'A' as i32 + 10 as std::os::raw::c_int
                                    } else if isnum(c) != 0 {
                                        c = c - '0' as i32
                                    } else if i > 0 as std::os::raw::c_int {
                                        expect(
                                            b"more hex digits in universal-character-name\x00"
                                                as *const u8
                                                as *const std::os::raw::c_char,
                                        );
                                    } else {
                                        c = n;
                                        current_block = 8297889085312008496;
                                        break;
                                    }
                                    n = n * 16 as std::os::raw::c_int + c;
                                    p = p.offset(1);
                                    i -= 1;
                                    if !(i != 0) {
                                        current_block = 3222590281903869779;
                                        break;
                                    }
                                }
                                match current_block {
                                    8297889085312008496 => {},
                                    _ => {
                                        cstr_u8cat(outstr, n);
                                        continue;
                                    },
                                }
                            },
                        },
                    }
                },
                97 => {
                    current_block = 7388311901245426766;
                    match current_block {
                        15715630227694234064 => {
                            n = c - '0' as i32;
                            p = p.offset(1);
                            c = *p as std::os::raw::c_int;
                            if isoct(c) != 0 {
                                n = n * 8 as std::os::raw::c_int + c - '0' as i32;
                                p = p.offset(1);
                                c = *p as std::os::raw::c_int;
                                if isoct(c) != 0 {
                                    n = n * 8 as std::os::raw::c_int + c - '0' as i32;
                                    p = p.offset(1)
                                }
                            }
                            c = n;
                            current_block = 8297889085312008496;
                        },
                        3761358111973716264 => {
                            if (*tcc_state).gnu_ext == 0 {
                                current_block = 18043221310185898469;
                            } else {
                                c = 27 as std::os::raw::c_int;
                                current_block = 562309032768341766;
                            }
                        },
                        111971726257118000 => {
                            i = 0 as std::os::raw::c_int;
                            current_block = 1923476342031941186;
                        },
                        8335422270325910153 => {
                            i = 4 as std::os::raw::c_int;
                            current_block = 1923476342031941186;
                        },
                        14754185658451558945 => {
                            i = 8 as std::os::raw::c_int;
                            current_block = 1923476342031941186;
                        },
                        7388311901245426766 => {
                            c = '\u{7}' as i32;
                            current_block = 562309032768341766;
                        },
                        16704339345470113457 => {
                            c = '\u{8}' as i32;
                            current_block = 562309032768341766;
                        },
                        8070590502297169425 => {
                            c = '\u{c}' as i32;
                            current_block = 562309032768341766;
                        },
                        7068460829993434719 => {
                            c = '\n' as i32;
                            current_block = 562309032768341766;
                        },
                        5140500990525381484 => {
                            c = '\r' as i32;
                            current_block = 562309032768341766;
                        },
                        11484286114940466897 => {
                            c = '\t' as i32;
                            current_block = 562309032768341766;
                        },
                        2642875140583231469 => {
                            c = '\u{b}' as i32;
                            current_block = 562309032768341766;
                        },
                        _ => {},
                    }
                    match current_block {
                        8297889085312008496 => {},
                        562309032768341766 => {},
                        _ => match current_block {
                            18043221310185898469 => {
                                if c >= '!' as i32 && c <= '~' as i32 {
                                    _tcc_warning(
                                        b"unknown escape sequence: \'\\%c\'\x00" as *const u8
                                            as *const std::os::raw::c_char,
                                        c,
                                    );
                                } else {
                                    _tcc_warning(
                                        b"unknown escape sequence: \'\\x%x\'\x00" as *const u8
                                            as *const std::os::raw::c_char,
                                        c,
                                    );
                                }
                                current_block = 562309032768341766;
                            },
                            _ => {
                                p = p.offset(1);
                                n = 0 as std::os::raw::c_int;
                                loop {
                                    c = *p as std::os::raw::c_int;
                                    if c >= 'a' as i32 && c <= 'f' as i32 {
                                        c = c - 'a' as i32 + 10 as std::os::raw::c_int
                                    } else if c >= 'A' as i32 && c <= 'F' as i32 {
                                        c = c - 'A' as i32 + 10 as std::os::raw::c_int
                                    } else if isnum(c) != 0 {
                                        c = c - '0' as i32
                                    } else if i > 0 as std::os::raw::c_int {
                                        expect(
                                            b"more hex digits in universal-character-name\x00"
                                                as *const u8
                                                as *const std::os::raw::c_char,
                                        );
                                    } else {
                                        c = n;
                                        current_block = 8297889085312008496;
                                        break;
                                    }
                                    n = n * 16 as std::os::raw::c_int + c;
                                    p = p.offset(1);
                                    i -= 1;
                                    if !(i != 0) {
                                        current_block = 3222590281903869779;
                                        break;
                                    }
                                }
                                match current_block {
                                    8297889085312008496 => {},
                                    _ => {
                                        cstr_u8cat(outstr, n);
                                        continue;
                                    },
                                }
                            },
                        },
                    }
                },
                98 => {
                    current_block = 16704339345470113457;
                    match current_block {
                        15715630227694234064 => {
                            n = c - '0' as i32;
                            p = p.offset(1);
                            c = *p as std::os::raw::c_int;
                            if isoct(c) != 0 {
                                n = n * 8 as std::os::raw::c_int + c - '0' as i32;
                                p = p.offset(1);
                                c = *p as std::os::raw::c_int;
                                if isoct(c) != 0 {
                                    n = n * 8 as std::os::raw::c_int + c - '0' as i32;
                                    p = p.offset(1)
                                }
                            }
                            c = n;
                            current_block = 8297889085312008496;
                        },
                        3761358111973716264 => {
                            if (*tcc_state).gnu_ext == 0 {
                                current_block = 18043221310185898469;
                            } else {
                                c = 27 as std::os::raw::c_int;
                                current_block = 562309032768341766;
                            }
                        },
                        111971726257118000 => {
                            i = 0 as std::os::raw::c_int;
                            current_block = 1923476342031941186;
                        },
                        8335422270325910153 => {
                            i = 4 as std::os::raw::c_int;
                            current_block = 1923476342031941186;
                        },
                        14754185658451558945 => {
                            i = 8 as std::os::raw::c_int;
                            current_block = 1923476342031941186;
                        },
                        7388311901245426766 => {
                            c = '\u{7}' as i32;
                            current_block = 562309032768341766;
                        },
                        16704339345470113457 => {
                            c = '\u{8}' as i32;
                            current_block = 562309032768341766;
                        },
                        8070590502297169425 => {
                            c = '\u{c}' as i32;
                            current_block = 562309032768341766;
                        },
                        7068460829993434719 => {
                            c = '\n' as i32;
                            current_block = 562309032768341766;
                        },
                        5140500990525381484 => {
                            c = '\r' as i32;
                            current_block = 562309032768341766;
                        },
                        11484286114940466897 => {
                            c = '\t' as i32;
                            current_block = 562309032768341766;
                        },
                        2642875140583231469 => {
                            c = '\u{b}' as i32;
                            current_block = 562309032768341766;
                        },
                        _ => {},
                    }
                    match current_block {
                        8297889085312008496 => {},
                        562309032768341766 => {},
                        _ => match current_block {
                            18043221310185898469 => {
                                if c >= '!' as i32 && c <= '~' as i32 {
                                    _tcc_warning(
                                        b"unknown escape sequence: \'\\%c\'\x00" as *const u8
                                            as *const std::os::raw::c_char,
                                        c,
                                    );
                                } else {
                                    _tcc_warning(
                                        b"unknown escape sequence: \'\\x%x\'\x00" as *const u8
                                            as *const std::os::raw::c_char,
                                        c,
                                    );
                                }
                                current_block = 562309032768341766;
                            },
                            _ => {
                                p = p.offset(1);
                                n = 0 as std::os::raw::c_int;
                                loop {
                                    c = *p as std::os::raw::c_int;
                                    if c >= 'a' as i32 && c <= 'f' as i32 {
                                        c = c - 'a' as i32 + 10 as std::os::raw::c_int
                                    } else if c >= 'A' as i32 && c <= 'F' as i32 {
                                        c = c - 'A' as i32 + 10 as std::os::raw::c_int
                                    } else if isnum(c) != 0 {
                                        c = c - '0' as i32
                                    } else if i > 0 as std::os::raw::c_int {
                                        expect(
                                            b"more hex digits in universal-character-name\x00"
                                                as *const u8
                                                as *const std::os::raw::c_char,
                                        );
                                    } else {
                                        c = n;
                                        current_block = 8297889085312008496;
                                        break;
                                    }
                                    n = n * 16 as std::os::raw::c_int + c;
                                    p = p.offset(1);
                                    i -= 1;
                                    if !(i != 0) {
                                        current_block = 3222590281903869779;
                                        break;
                                    }
                                }
                                match current_block {
                                    8297889085312008496 => {},
                                    _ => {
                                        cstr_u8cat(outstr, n);
                                        continue;
                                    },
                                }
                            },
                        },
                    }
                },
                102 => {
                    current_block = 8070590502297169425;
                    match current_block {
                        15715630227694234064 => {
                            n = c - '0' as i32;
                            p = p.offset(1);
                            c = *p as std::os::raw::c_int;
                            if isoct(c) != 0 {
                                n = n * 8 as std::os::raw::c_int + c - '0' as i32;
                                p = p.offset(1);
                                c = *p as std::os::raw::c_int;
                                if isoct(c) != 0 {
                                    n = n * 8 as std::os::raw::c_int + c - '0' as i32;
                                    p = p.offset(1)
                                }
                            }
                            c = n;
                            current_block = 8297889085312008496;
                        },
                        3761358111973716264 => {
                            if (*tcc_state).gnu_ext == 0 {
                                current_block = 18043221310185898469;
                            } else {
                                c = 27 as std::os::raw::c_int;
                                current_block = 562309032768341766;
                            }
                        },
                        111971726257118000 => {
                            i = 0 as std::os::raw::c_int;
                            current_block = 1923476342031941186;
                        },
                        8335422270325910153 => {
                            i = 4 as std::os::raw::c_int;
                            current_block = 1923476342031941186;
                        },
                        14754185658451558945 => {
                            i = 8 as std::os::raw::c_int;
                            current_block = 1923476342031941186;
                        },
                        7388311901245426766 => {
                            c = '\u{7}' as i32;
                            current_block = 562309032768341766;
                        },
                        16704339345470113457 => {
                            c = '\u{8}' as i32;
                            current_block = 562309032768341766;
                        },
                        8070590502297169425 => {
                            c = '\u{c}' as i32;
                            current_block = 562309032768341766;
                        },
                        7068460829993434719 => {
                            c = '\n' as i32;
                            current_block = 562309032768341766;
                        },
                        5140500990525381484 => {
                            c = '\r' as i32;
                            current_block = 562309032768341766;
                        },
                        11484286114940466897 => {
                            c = '\t' as i32;
                            current_block = 562309032768341766;
                        },
                        2642875140583231469 => {
                            c = '\u{b}' as i32;
                            current_block = 562309032768341766;
                        },
                        _ => {},
                    }
                    match current_block {
                        8297889085312008496 => {},
                        562309032768341766 => {},
                        _ => match current_block {
                            18043221310185898469 => {
                                if c >= '!' as i32 && c <= '~' as i32 {
                                    _tcc_warning(
                                        b"unknown escape sequence: \'\\%c\'\x00" as *const u8
                                            as *const std::os::raw::c_char,
                                        c,
                                    );
                                } else {
                                    _tcc_warning(
                                        b"unknown escape sequence: \'\\x%x\'\x00" as *const u8
                                            as *const std::os::raw::c_char,
                                        c,
                                    );
                                }
                                current_block = 562309032768341766;
                            },
                            _ => {
                                p = p.offset(1);
                                n = 0 as std::os::raw::c_int;
                                loop {
                                    c = *p as std::os::raw::c_int;
                                    if c >= 'a' as i32 && c <= 'f' as i32 {
                                        c = c - 'a' as i32 + 10 as std::os::raw::c_int
                                    } else if c >= 'A' as i32 && c <= 'F' as i32 {
                                        c = c - 'A' as i32 + 10 as std::os::raw::c_int
                                    } else if isnum(c) != 0 {
                                        c = c - '0' as i32
                                    } else if i > 0 as std::os::raw::c_int {
                                        expect(
                                            b"more hex digits in universal-character-name\x00"
                                                as *const u8
                                                as *const std::os::raw::c_char,
                                        );
                                    } else {
                                        c = n;
                                        current_block = 8297889085312008496;
                                        break;
                                    }
                                    n = n * 16 as std::os::raw::c_int + c;
                                    p = p.offset(1);
                                    i -= 1;
                                    if !(i != 0) {
                                        current_block = 3222590281903869779;
                                        break;
                                    }
                                }
                                match current_block {
                                    8297889085312008496 => {},
                                    _ => {
                                        cstr_u8cat(outstr, n);
                                        continue;
                                    },
                                }
                            },
                        },
                    }
                },
                110 => {
                    current_block = 7068460829993434719;
                    match current_block {
                        15715630227694234064 => {
                            n = c - '0' as i32;
                            p = p.offset(1);
                            c = *p as std::os::raw::c_int;
                            if isoct(c) != 0 {
                                n = n * 8 as std::os::raw::c_int + c - '0' as i32;
                                p = p.offset(1);
                                c = *p as std::os::raw::c_int;
                                if isoct(c) != 0 {
                                    n = n * 8 as std::os::raw::c_int + c - '0' as i32;
                                    p = p.offset(1)
                                }
                            }
                            c = n;
                            current_block = 8297889085312008496;
                        },
                        3761358111973716264 => {
                            if (*tcc_state).gnu_ext == 0 {
                                current_block = 18043221310185898469;
                            } else {
                                c = 27 as std::os::raw::c_int;
                                current_block = 562309032768341766;
                            }
                        },
                        111971726257118000 => {
                            i = 0 as std::os::raw::c_int;
                            current_block = 1923476342031941186;
                        },
                        8335422270325910153 => {
                            i = 4 as std::os::raw::c_int;
                            current_block = 1923476342031941186;
                        },
                        14754185658451558945 => {
                            i = 8 as std::os::raw::c_int;
                            current_block = 1923476342031941186;
                        },
                        7388311901245426766 => {
                            c = '\u{7}' as i32;
                            current_block = 562309032768341766;
                        },
                        16704339345470113457 => {
                            c = '\u{8}' as i32;
                            current_block = 562309032768341766;
                        },
                        8070590502297169425 => {
                            c = '\u{c}' as i32;
                            current_block = 562309032768341766;
                        },
                        7068460829993434719 => {
                            c = '\n' as i32;
                            current_block = 562309032768341766;
                        },
                        5140500990525381484 => {
                            c = '\r' as i32;
                            current_block = 562309032768341766;
                        },
                        11484286114940466897 => {
                            c = '\t' as i32;
                            current_block = 562309032768341766;
                        },
                        2642875140583231469 => {
                            c = '\u{b}' as i32;
                            current_block = 562309032768341766;
                        },
                        _ => {},
                    }
                    match current_block {
                        8297889085312008496 => {},
                        562309032768341766 => {},
                        _ => match current_block {
                            18043221310185898469 => {
                                if c >= '!' as i32 && c <= '~' as i32 {
                                    _tcc_warning(
                                        b"unknown escape sequence: \'\\%c\'\x00" as *const u8
                                            as *const std::os::raw::c_char,
                                        c,
                                    );
                                } else {
                                    _tcc_warning(
                                        b"unknown escape sequence: \'\\x%x\'\x00" as *const u8
                                            as *const std::os::raw::c_char,
                                        c,
                                    );
                                }
                                current_block = 562309032768341766;
                            },
                            _ => {
                                p = p.offset(1);
                                n = 0 as std::os::raw::c_int;
                                loop {
                                    c = *p as std::os::raw::c_int;
                                    if c >= 'a' as i32 && c <= 'f' as i32 {
                                        c = c - 'a' as i32 + 10 as std::os::raw::c_int
                                    } else if c >= 'A' as i32 && c <= 'F' as i32 {
                                        c = c - 'A' as i32 + 10 as std::os::raw::c_int
                                    } else if isnum(c) != 0 {
                                        c = c - '0' as i32
                                    } else if i > 0 as std::os::raw::c_int {
                                        expect(
                                            b"more hex digits in universal-character-name\x00"
                                                as *const u8
                                                as *const std::os::raw::c_char,
                                        );
                                    } else {
                                        c = n;
                                        current_block = 8297889085312008496;
                                        break;
                                    }
                                    n = n * 16 as std::os::raw::c_int + c;
                                    p = p.offset(1);
                                    i -= 1;
                                    if !(i != 0) {
                                        current_block = 3222590281903869779;
                                        break;
                                    }
                                }
                                match current_block {
                                    8297889085312008496 => {},
                                    _ => {
                                        cstr_u8cat(outstr, n);
                                        continue;
                                    },
                                }
                            },
                        },
                    }
                },
                114 => {
                    current_block = 5140500990525381484;
                    match current_block {
                        15715630227694234064 => {
                            n = c - '0' as i32;
                            p = p.offset(1);
                            c = *p as std::os::raw::c_int;
                            if isoct(c) != 0 {
                                n = n * 8 as std::os::raw::c_int + c - '0' as i32;
                                p = p.offset(1);
                                c = *p as std::os::raw::c_int;
                                if isoct(c) != 0 {
                                    n = n * 8 as std::os::raw::c_int + c - '0' as i32;
                                    p = p.offset(1)
                                }
                            }
                            c = n;
                            current_block = 8297889085312008496;
                        },
                        3761358111973716264 => {
                            if (*tcc_state).gnu_ext == 0 {
                                current_block = 18043221310185898469;
                            } else {
                                c = 27 as std::os::raw::c_int;
                                current_block = 562309032768341766;
                            }
                        },
                        111971726257118000 => {
                            i = 0 as std::os::raw::c_int;
                            current_block = 1923476342031941186;
                        },
                        8335422270325910153 => {
                            i = 4 as std::os::raw::c_int;
                            current_block = 1923476342031941186;
                        },
                        14754185658451558945 => {
                            i = 8 as std::os::raw::c_int;
                            current_block = 1923476342031941186;
                        },
                        7388311901245426766 => {
                            c = '\u{7}' as i32;
                            current_block = 562309032768341766;
                        },
                        16704339345470113457 => {
                            c = '\u{8}' as i32;
                            current_block = 562309032768341766;
                        },
                        8070590502297169425 => {
                            c = '\u{c}' as i32;
                            current_block = 562309032768341766;
                        },
                        7068460829993434719 => {
                            c = '\n' as i32;
                            current_block = 562309032768341766;
                        },
                        5140500990525381484 => {
                            c = '\r' as i32;
                            current_block = 562309032768341766;
                        },
                        11484286114940466897 => {
                            c = '\t' as i32;
                            current_block = 562309032768341766;
                        },
                        2642875140583231469 => {
                            c = '\u{b}' as i32;
                            current_block = 562309032768341766;
                        },
                        _ => {},
                    }
                    match current_block {
                        8297889085312008496 => {},
                        562309032768341766 => {},
                        _ => match current_block {
                            18043221310185898469 => {
                                if c >= '!' as i32 && c <= '~' as i32 {
                                    _tcc_warning(
                                        b"unknown escape sequence: \'\\%c\'\x00" as *const u8
                                            as *const std::os::raw::c_char,
                                        c,
                                    );
                                } else {
                                    _tcc_warning(
                                        b"unknown escape sequence: \'\\x%x\'\x00" as *const u8
                                            as *const std::os::raw::c_char,
                                        c,
                                    );
                                }
                                current_block = 562309032768341766;
                            },
                            _ => {
                                p = p.offset(1);
                                n = 0 as std::os::raw::c_int;
                                loop {
                                    c = *p as std::os::raw::c_int;
                                    if c >= 'a' as i32 && c <= 'f' as i32 {
                                        c = c - 'a' as i32 + 10 as std::os::raw::c_int
                                    } else if c >= 'A' as i32 && c <= 'F' as i32 {
                                        c = c - 'A' as i32 + 10 as std::os::raw::c_int
                                    } else if isnum(c) != 0 {
                                        c = c - '0' as i32
                                    } else if i > 0 as std::os::raw::c_int {
                                        expect(
                                            b"more hex digits in universal-character-name\x00"
                                                as *const u8
                                                as *const std::os::raw::c_char,
                                        );
                                    } else {
                                        c = n;
                                        current_block = 8297889085312008496;
                                        break;
                                    }
                                    n = n * 16 as std::os::raw::c_int + c;
                                    p = p.offset(1);
                                    i -= 1;
                                    if !(i != 0) {
                                        current_block = 3222590281903869779;
                                        break;
                                    }
                                }
                                match current_block {
                                    8297889085312008496 => {},
                                    _ => {
                                        cstr_u8cat(outstr, n);
                                        continue;
                                    },
                                }
                            },
                        },
                    }
                },
                116 => {
                    current_block = 11484286114940466897;
                    match current_block {
                        15715630227694234064 => {
                            n = c - '0' as i32;
                            p = p.offset(1);
                            c = *p as std::os::raw::c_int;
                            if isoct(c) != 0 {
                                n = n * 8 as std::os::raw::c_int + c - '0' as i32;
                                p = p.offset(1);
                                c = *p as std::os::raw::c_int;
                                if isoct(c) != 0 {
                                    n = n * 8 as std::os::raw::c_int + c - '0' as i32;
                                    p = p.offset(1)
                                }
                            }
                            c = n;
                            current_block = 8297889085312008496;
                        },
                        3761358111973716264 => {
                            if (*tcc_state).gnu_ext == 0 {
                                current_block = 18043221310185898469;
                            } else {
                                c = 27 as std::os::raw::c_int;
                                current_block = 562309032768341766;
                            }
                        },
                        111971726257118000 => {
                            i = 0 as std::os::raw::c_int;
                            current_block = 1923476342031941186;
                        },
                        8335422270325910153 => {
                            i = 4 as std::os::raw::c_int;
                            current_block = 1923476342031941186;
                        },
                        14754185658451558945 => {
                            i = 8 as std::os::raw::c_int;
                            current_block = 1923476342031941186;
                        },
                        7388311901245426766 => {
                            c = '\u{7}' as i32;
                            current_block = 562309032768341766;
                        },
                        16704339345470113457 => {
                            c = '\u{8}' as i32;
                            current_block = 562309032768341766;
                        },
                        8070590502297169425 => {
                            c = '\u{c}' as i32;
                            current_block = 562309032768341766;
                        },
                        7068460829993434719 => {
                            c = '\n' as i32;
                            current_block = 562309032768341766;
                        },
                        5140500990525381484 => {
                            c = '\r' as i32;
                            current_block = 562309032768341766;
                        },
                        11484286114940466897 => {
                            c = '\t' as i32;
                            current_block = 562309032768341766;
                        },
                        2642875140583231469 => {
                            c = '\u{b}' as i32;
                            current_block = 562309032768341766;
                        },
                        _ => {},
                    }
                    match current_block {
                        8297889085312008496 => {},
                        562309032768341766 => {},
                        _ => match current_block {
                            18043221310185898469 => {
                                if c >= '!' as i32 && c <= '~' as i32 {
                                    _tcc_warning(
                                        b"unknown escape sequence: \'\\%c\'\x00" as *const u8
                                            as *const std::os::raw::c_char,
                                        c,
                                    );
                                } else {
                                    _tcc_warning(
                                        b"unknown escape sequence: \'\\x%x\'\x00" as *const u8
                                            as *const std::os::raw::c_char,
                                        c,
                                    );
                                }
                                current_block = 562309032768341766;
                            },
                            _ => {
                                p = p.offset(1);
                                n = 0 as std::os::raw::c_int;
                                loop {
                                    c = *p as std::os::raw::c_int;
                                    if c >= 'a' as i32 && c <= 'f' as i32 {
                                        c = c - 'a' as i32 + 10 as std::os::raw::c_int
                                    } else if c >= 'A' as i32 && c <= 'F' as i32 {
                                        c = c - 'A' as i32 + 10 as std::os::raw::c_int
                                    } else if isnum(c) != 0 {
                                        c = c - '0' as i32
                                    } else if i > 0 as std::os::raw::c_int {
                                        expect(
                                            b"more hex digits in universal-character-name\x00"
                                                as *const u8
                                                as *const std::os::raw::c_char,
                                        );
                                    } else {
                                        c = n;
                                        current_block = 8297889085312008496;
                                        break;
                                    }
                                    n = n * 16 as std::os::raw::c_int + c;
                                    p = p.offset(1);
                                    i -= 1;
                                    if !(i != 0) {
                                        current_block = 3222590281903869779;
                                        break;
                                    }
                                }
                                match current_block {
                                    8297889085312008496 => {},
                                    _ => {
                                        cstr_u8cat(outstr, n);
                                        continue;
                                    },
                                }
                            },
                        },
                    }
                },
                118 => {
                    current_block = 2642875140583231469;
                    match current_block {
                        15715630227694234064 => {
                            n = c - '0' as i32;
                            p = p.offset(1);
                            c = *p as std::os::raw::c_int;
                            if isoct(c) != 0 {
                                n = n * 8 as std::os::raw::c_int + c - '0' as i32;
                                p = p.offset(1);
                                c = *p as std::os::raw::c_int;
                                if isoct(c) != 0 {
                                    n = n * 8 as std::os::raw::c_int + c - '0' as i32;
                                    p = p.offset(1)
                                }
                            }
                            c = n;
                            current_block = 8297889085312008496;
                        },
                        3761358111973716264 => {
                            if (*tcc_state).gnu_ext == 0 {
                                current_block = 18043221310185898469;
                            } else {
                                c = 27 as std::os::raw::c_int;
                                current_block = 562309032768341766;
                            }
                        },
                        111971726257118000 => {
                            i = 0 as std::os::raw::c_int;
                            current_block = 1923476342031941186;
                        },
                        8335422270325910153 => {
                            i = 4 as std::os::raw::c_int;
                            current_block = 1923476342031941186;
                        },
                        14754185658451558945 => {
                            i = 8 as std::os::raw::c_int;
                            current_block = 1923476342031941186;
                        },
                        7388311901245426766 => {
                            c = '\u{7}' as i32;
                            current_block = 562309032768341766;
                        },
                        16704339345470113457 => {
                            c = '\u{8}' as i32;
                            current_block = 562309032768341766;
                        },
                        8070590502297169425 => {
                            c = '\u{c}' as i32;
                            current_block = 562309032768341766;
                        },
                        7068460829993434719 => {
                            c = '\n' as i32;
                            current_block = 562309032768341766;
                        },
                        5140500990525381484 => {
                            c = '\r' as i32;
                            current_block = 562309032768341766;
                        },
                        11484286114940466897 => {
                            c = '\t' as i32;
                            current_block = 562309032768341766;
                        },
                        2642875140583231469 => {
                            c = '\u{b}' as i32;
                            current_block = 562309032768341766;
                        },
                        _ => {},
                    }
                    match current_block {
                        8297889085312008496 => {},
                        562309032768341766 => {},
                        _ => match current_block {
                            18043221310185898469 => {
                                if c >= '!' as i32 && c <= '~' as i32 {
                                    _tcc_warning(
                                        b"unknown escape sequence: \'\\%c\'\x00" as *const u8
                                            as *const std::os::raw::c_char,
                                        c,
                                    );
                                } else {
                                    _tcc_warning(
                                        b"unknown escape sequence: \'\\x%x\'\x00" as *const u8
                                            as *const std::os::raw::c_char,
                                        c,
                                    );
                                }
                                current_block = 562309032768341766;
                            },
                            _ => {
                                p = p.offset(1);
                                n = 0 as std::os::raw::c_int;
                                loop {
                                    c = *p as std::os::raw::c_int;
                                    if c >= 'a' as i32 && c <= 'f' as i32 {
                                        c = c - 'a' as i32 + 10 as std::os::raw::c_int
                                    } else if c >= 'A' as i32 && c <= 'F' as i32 {
                                        c = c - 'A' as i32 + 10 as std::os::raw::c_int
                                    } else if isnum(c) != 0 {
                                        c = c - '0' as i32
                                    } else if i > 0 as std::os::raw::c_int {
                                        expect(
                                            b"more hex digits in universal-character-name\x00"
                                                as *const u8
                                                as *const std::os::raw::c_char,
                                        );
                                    } else {
                                        c = n;
                                        current_block = 8297889085312008496;
                                        break;
                                    }
                                    n = n * 16 as std::os::raw::c_int + c;
                                    p = p.offset(1);
                                    i -= 1;
                                    if !(i != 0) {
                                        current_block = 3222590281903869779;
                                        break;
                                    }
                                }
                                match current_block {
                                    8297889085312008496 => {},
                                    _ => {
                                        cstr_u8cat(outstr, n);
                                        continue;
                                    },
                                }
                            },
                        },
                    }
                },
                101 => {
                    current_block = 3761358111973716264;
                    match current_block {
                        15715630227694234064 => {
                            n = c - '0' as i32;
                            p = p.offset(1);
                            c = *p as std::os::raw::c_int;
                            if isoct(c) != 0 {
                                n = n * 8 as std::os::raw::c_int + c - '0' as i32;
                                p = p.offset(1);
                                c = *p as std::os::raw::c_int;
                                if isoct(c) != 0 {
                                    n = n * 8 as std::os::raw::c_int + c - '0' as i32;
                                    p = p.offset(1)
                                }
                            }
                            c = n;
                            current_block = 8297889085312008496;
                        },
                        3761358111973716264 => {
                            if (*tcc_state).gnu_ext == 0 {
                                current_block = 18043221310185898469;
                            } else {
                                c = 27 as std::os::raw::c_int;
                                current_block = 562309032768341766;
                            }
                        },
                        111971726257118000 => {
                            i = 0 as std::os::raw::c_int;
                            current_block = 1923476342031941186;
                        },
                        8335422270325910153 => {
                            i = 4 as std::os::raw::c_int;
                            current_block = 1923476342031941186;
                        },
                        14754185658451558945 => {
                            i = 8 as std::os::raw::c_int;
                            current_block = 1923476342031941186;
                        },
                        7388311901245426766 => {
                            c = '\u{7}' as i32;
                            current_block = 562309032768341766;
                        },
                        16704339345470113457 => {
                            c = '\u{8}' as i32;
                            current_block = 562309032768341766;
                        },
                        8070590502297169425 => {
                            c = '\u{c}' as i32;
                            current_block = 562309032768341766;
                        },
                        7068460829993434719 => {
                            c = '\n' as i32;
                            current_block = 562309032768341766;
                        },
                        5140500990525381484 => {
                            c = '\r' as i32;
                            current_block = 562309032768341766;
                        },
                        11484286114940466897 => {
                            c = '\t' as i32;
                            current_block = 562309032768341766;
                        },
                        2642875140583231469 => {
                            c = '\u{b}' as i32;
                            current_block = 562309032768341766;
                        },
                        _ => {},
                    }
                    match current_block {
                        8297889085312008496 => {},
                        562309032768341766 => {},
                        _ => match current_block {
                            18043221310185898469 => {
                                if c >= '!' as i32 && c <= '~' as i32 {
                                    _tcc_warning(
                                        b"unknown escape sequence: \'\\%c\'\x00" as *const u8
                                            as *const std::os::raw::c_char,
                                        c,
                                    );
                                } else {
                                    _tcc_warning(
                                        b"unknown escape sequence: \'\\x%x\'\x00" as *const u8
                                            as *const std::os::raw::c_char,
                                        c,
                                    );
                                }
                                current_block = 562309032768341766;
                            },
                            _ => {
                                p = p.offset(1);
                                n = 0 as std::os::raw::c_int;
                                loop {
                                    c = *p as std::os::raw::c_int;
                                    if c >= 'a' as i32 && c <= 'f' as i32 {
                                        c = c - 'a' as i32 + 10 as std::os::raw::c_int
                                    } else if c >= 'A' as i32 && c <= 'F' as i32 {
                                        c = c - 'A' as i32 + 10 as std::os::raw::c_int
                                    } else if isnum(c) != 0 {
                                        c = c - '0' as i32
                                    } else if i > 0 as std::os::raw::c_int {
                                        expect(
                                            b"more hex digits in universal-character-name\x00"
                                                as *const u8
                                                as *const std::os::raw::c_char,
                                        );
                                    } else {
                                        c = n;
                                        current_block = 8297889085312008496;
                                        break;
                                    }
                                    n = n * 16 as std::os::raw::c_int + c;
                                    p = p.offset(1);
                                    i -= 1;
                                    if !(i != 0) {
                                        current_block = 3222590281903869779;
                                        break;
                                    }
                                }
                                match current_block {
                                    8297889085312008496 => {},
                                    _ => {
                                        cstr_u8cat(outstr, n);
                                        continue;
                                    },
                                }
                            },
                        },
                    }
                },
                39 | 34 | 92 | 63 => {
                    current_block = 562309032768341766;
                },
                _ => {
                    current_block = 18043221310185898469;
                    match current_block {
                        15715630227694234064 => {
                            n = c - '0' as i32;
                            p = p.offset(1);
                            c = *p as std::os::raw::c_int;
                            if isoct(c) != 0 {
                                n = n * 8 as std::os::raw::c_int + c - '0' as i32;
                                p = p.offset(1);
                                c = *p as std::os::raw::c_int;
                                if isoct(c) != 0 {
                                    n = n * 8 as std::os::raw::c_int + c - '0' as i32;
                                    p = p.offset(1)
                                }
                            }
                            c = n;
                            current_block = 8297889085312008496;
                        },
                        3761358111973716264 => {
                            if (*tcc_state).gnu_ext == 0 {
                                current_block = 18043221310185898469;
                            } else {
                                c = 27 as std::os::raw::c_int;
                                current_block = 562309032768341766;
                            }
                        },
                        111971726257118000 => {
                            i = 0 as std::os::raw::c_int;
                            current_block = 1923476342031941186;
                        },
                        8335422270325910153 => {
                            i = 4 as std::os::raw::c_int;
                            current_block = 1923476342031941186;
                        },
                        14754185658451558945 => {
                            i = 8 as std::os::raw::c_int;
                            current_block = 1923476342031941186;
                        },
                        7388311901245426766 => {
                            c = '\u{7}' as i32;
                            current_block = 562309032768341766;
                        },
                        16704339345470113457 => {
                            c = '\u{8}' as i32;
                            current_block = 562309032768341766;
                        },
                        8070590502297169425 => {
                            c = '\u{c}' as i32;
                            current_block = 562309032768341766;
                        },
                        7068460829993434719 => {
                            c = '\n' as i32;
                            current_block = 562309032768341766;
                        },
                        5140500990525381484 => {
                            c = '\r' as i32;
                            current_block = 562309032768341766;
                        },
                        11484286114940466897 => {
                            c = '\t' as i32;
                            current_block = 562309032768341766;
                        },
                        2642875140583231469 => {
                            c = '\u{b}' as i32;
                            current_block = 562309032768341766;
                        },
                        _ => {},
                    }
                    match current_block {
                        8297889085312008496 => {},
                        562309032768341766 => {},
                        _ => match current_block {
                            18043221310185898469 => {
                                if c >= '!' as i32 && c <= '~' as i32 {
                                    _tcc_warning(
                                        b"unknown escape sequence: \'\\%c\'\x00" as *const u8
                                            as *const std::os::raw::c_char,
                                        c,
                                    );
                                } else {
                                    _tcc_warning(
                                        b"unknown escape sequence: \'\\x%x\'\x00" as *const u8
                                            as *const std::os::raw::c_char,
                                        c,
                                    );
                                }
                                current_block = 562309032768341766;
                            },
                            _ => {
                                p = p.offset(1);
                                n = 0 as std::os::raw::c_int;
                                loop {
                                    c = *p as std::os::raw::c_int;
                                    if c >= 'a' as i32 && c <= 'f' as i32 {
                                        c = c - 'a' as i32 + 10 as std::os::raw::c_int
                                    } else if c >= 'A' as i32 && c <= 'F' as i32 {
                                        c = c - 'A' as i32 + 10 as std::os::raw::c_int
                                    } else if isnum(c) != 0 {
                                        c = c - '0' as i32
                                    } else if i > 0 as std::os::raw::c_int {
                                        expect(
                                            b"more hex digits in universal-character-name\x00"
                                                as *const u8
                                                as *const std::os::raw::c_char,
                                        );
                                    } else {
                                        c = n;
                                        current_block = 8297889085312008496;
                                        break;
                                    }
                                    n = n * 16 as std::os::raw::c_int + c;
                                    p = p.offset(1);
                                    i -= 1;
                                    if !(i != 0) {
                                        current_block = 3222590281903869779;
                                        break;
                                    }
                                }
                                match current_block {
                                    8297889085312008496 => {},
                                    _ => {
                                        cstr_u8cat(outstr, n);
                                        continue;
                                    },
                                }
                            },
                        },
                    }
                },
            }
        } else if is_long != 0 && c >= 0x80 as std::os::raw::c_int {
            /* assume we are processing UTF-8 sequence */
            /* reference: The Unicode Standard, Version 10.0, ch3.9 */
            let mut cont: std::os::raw::c_int = 0; /* count of continuation bytes */
            let mut skip_0: std::os::raw::c_int = 0; /* how many bytes should skip when error occurred */
            let mut i_0: std::os::raw::c_int = 0;
            /* decode leading byte */
            if c < 0xc2 as std::os::raw::c_int {
                skip_0 = 1 as std::os::raw::c_int;
                current_block = 3339529741159126203;
            } else {
                if c <= 0xdf as std::os::raw::c_int {
                    cont = 1 as std::os::raw::c_int;
                    n = c & 0x1f as std::os::raw::c_int;
                    current_block = 8464383504555462953;
                } else if c <= 0xef as std::os::raw::c_int {
                    cont = 2 as std::os::raw::c_int;
                    n = c & 0xf as std::os::raw::c_int;
                    current_block = 8464383504555462953;
                } else if c <= 0xf4 as std::os::raw::c_int {
                    cont = 3 as std::os::raw::c_int;
                    n = c & 0x7 as std::os::raw::c_int;
                    current_block = 8464383504555462953;
                } else {
                    skip_0 = 1 as std::os::raw::c_int;
                    current_block = 3339529741159126203;
                }
                match current_block {
                    3339529741159126203 => {},
                    _ =>
                    /* decode continuation bytes */
                    {
                        i_0 = 1 as std::os::raw::c_int;
                        loop {
                            if !(i_0 <= cont) {
                                current_block = 9500030526577190060;
                                break;
                            }
                            let mut l: std::os::raw::c_int = 0x80 as std::os::raw::c_int;
                            let mut h: std::os::raw::c_int = 0xbf as std::os::raw::c_int;
                            /* adjust limit for second byte */
                            if i_0 == 1 as std::os::raw::c_int {
                                match c {
                                    224 => l = 0xa0 as std::os::raw::c_int,
                                    237 => h = 0x9f as std::os::raw::c_int,
                                    240 => l = 0x90 as std::os::raw::c_int,
                                    244 => h = 0x8f as std::os::raw::c_int,
                                    _ => {},
                                }
                            }
                            if (*p.offset(i_0 as isize) as std::os::raw::c_int) < l
                                || *p.offset(i_0 as isize) as std::os::raw::c_int > h
                            {
                                skip_0 = i_0;
                                current_block = 3339529741159126203;
                                break;
                            } else {
                                n = n << 6 as std::os::raw::c_int
                                    | *p.offset(i_0 as isize) as std::os::raw::c_int
                                        & 0x3f as std::os::raw::c_int;
                                i_0 += 1
                            }
                        }
                        match current_block {
                            3339529741159126203 => {},
                            _ => {
                                /* advance pointer */
                                p = p.offset((1 as std::os::raw::c_int + cont) as isize);
                                c = n;
                                current_block = 8297889085312008496;
                            },
                        }
                    }
                }
            }
            match current_block {
                8297889085312008496 => {},
                _ =>
                /* error handling */
                {
                    _tcc_warning(
                        b"ill-formed UTF-8 subsequence starting with: \'\\x%x\'\x00" as *const u8
                            as *const std::os::raw::c_char,
                        c,
                    );
                    c = 0xfffd as std::os::raw::c_int;
                    p = p.offset(skip_0 as isize);
                    current_block = 8297889085312008496;
                }
            }
        } else {
            current_block = 562309032768341766;
        }
        match current_block {
            562309032768341766 => p = p.offset(1),
            _ => {},
        }
        if is_long == 0 {
            cstr_ccat(outstr, c);
        } else {
            cstr_wccat(outstr, c);
        }
    }
    /* add a trailing '\0' */
    if is_long == 0 {
        cstr_ccat(outstr, '\u{0}' as i32);
    } else {
        cstr_wccat(outstr, '\u{0}' as i32);
    };
}
unsafe extern "C" fn parse_string(
    mut s: *const std::os::raw::c_char,
    mut len: std::os::raw::c_int,
) {
    let mut buf: [uint8_t; 1000] = [0; 1000];
    let mut p: *mut uint8_t = buf.as_mut_ptr();
    let mut is_long: std::os::raw::c_int = 0;
    let mut sep: std::os::raw::c_int = 0;
    is_long = (*s as std::os::raw::c_int == 'L' as i32) as std::os::raw::c_int;
    if is_long != 0 {
        s = s.offset(1);
        len -= 1
    }
    let fresh40 = s;
    s = s.offset(1);
    sep = *fresh40 as std::os::raw::c_int;
    len -= 2 as std::os::raw::c_int;
    if len as std::os::raw::c_ulong
        >= ::std::mem::size_of::<[uint8_t; 1000]>() as std::os::raw::c_ulong
    {
        p = tcc_malloc((len + 1 as std::os::raw::c_int) as std::os::raw::c_ulong) as *mut uint8_t
    }
    memcpy(
        p as *mut std::os::raw::c_void,
        s as *const std::os::raw::c_void,
        len as std::os::raw::c_ulong,
    );
    *p.offset(len as isize) = 0 as std::os::raw::c_int as uint8_t;
    cstr_reset(&mut tokcstr);
    parse_escape_string(&mut tokcstr, p, is_long);
    if p != buf.as_mut_ptr() {
        tcc_free(p as *mut std::os::raw::c_void);
    }
    if sep == '\'' as i32 {
        let mut char_size: std::os::raw::c_int = 0;
        let mut i: std::os::raw::c_int = 0;
        let mut n: std::os::raw::c_int = 0;
        let mut c: std::os::raw::c_int = 0;
        /* XXX: make it portable */
        if is_long == 0 {
            tok = 0xc0 as std::os::raw::c_int;
            char_size = 1 as std::os::raw::c_int
        } else {
            tok = 0xc1 as std::os::raw::c_int;
            char_size =
                ::std::mem::size_of::<nwchar_t>() as std::os::raw::c_ulong as std::os::raw::c_int
        }
        n = tokcstr.size / char_size - 1 as std::os::raw::c_int;
        if n < 1 as std::os::raw::c_int {
            _tcc_error(b"empty character constant\x00" as *const u8 as *const std::os::raw::c_char);
        }
        if n > 1 as std::os::raw::c_int {
            _tcc_warning(
                b"multi-character character constant\x00" as *const u8
                    as *const std::os::raw::c_char,
            );
        }
        i = 0 as std::os::raw::c_int;
        c = i;
        while i < n {
            if is_long != 0 {
                c = *(tokcstr.data as *mut nwchar_t).offset(i as isize)
            } else {
                c = c << 8 as std::os::raw::c_int
                    | *(tokcstr.data as *mut std::os::raw::c_char).offset(i as isize)
                        as std::os::raw::c_int
            }
            i += 1
        }
        tokc.i = c as uint64_t
    } else {
        tokc.str_0.size = tokcstr.size;
        tokc.str_0.data = tokcstr.data;
        if is_long == 0 {
            tok = 0xc8 as std::os::raw::c_int
        } else {
            tok = 0xc9 as std::os::raw::c_int
        }
    };
}
/* bn = (bn << shift) | or_val */
unsafe extern "C" fn bn_lshift(
    mut bn: *mut std::os::raw::c_uint,
    mut shift: std::os::raw::c_int,
    mut or_val: std::os::raw::c_int,
) {
    let mut i: std::os::raw::c_int = 0;
    let mut v: std::os::raw::c_uint = 0;
    i = 0 as std::os::raw::c_int;
    while i < 2 as std::os::raw::c_int {
        v = *bn.offset(i as isize);
        *bn.offset(i as isize) = v << shift | or_val as std::os::raw::c_uint;
        or_val = (v >> 32 as std::os::raw::c_int - shift) as std::os::raw::c_int;
        i += 1
    }
}
unsafe extern "C" fn bn_zero(mut bn: *mut std::os::raw::c_uint) {
    let mut i: std::os::raw::c_int = 0;
    i = 0 as std::os::raw::c_int;
    while i < 2 as std::os::raw::c_int {
        *bn.offset(i as isize) = 0 as std::os::raw::c_int as std::os::raw::c_uint;
        i += 1
    }
}
/* parse number in null terminated string 'p' and return it in the
current token */
unsafe extern "C" fn parse_number(mut p: *const std::os::raw::c_char) {
    let mut current_block: u64;
    let mut b: std::os::raw::c_int = 0;
    let mut t: std::os::raw::c_int = 0;
    let mut shift: std::os::raw::c_int = 0;
    let mut frac_bits: std::os::raw::c_int = 0;
    let mut s: std::os::raw::c_int = 0;
    let mut exp_val: std::os::raw::c_int = 0;
    let mut ch_0: std::os::raw::c_int = 0;
    let mut q: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut bn: [std::os::raw::c_uint; 2] = [0; 2];
    let mut d: std::os::raw::c_double = 0.;
    /* number */
    q = token_buf.as_mut_ptr();
    let fresh41 = p;
    p = p.offset(1);
    ch_0 = *fresh41 as std::os::raw::c_int;
    t = ch_0;
    let fresh42 = p;
    p = p.offset(1);
    ch_0 = *fresh42 as std::os::raw::c_int;
    let fresh43 = q;
    q = q.offset(1);
    *fresh43 = t as std::os::raw::c_char;
    b = 10 as std::os::raw::c_int;
    if t == '.' as i32 {
        current_block = 8140372313878014523;
    } else {
        if t == '0' as i32 {
            if ch_0 == 'x' as i32 || ch_0 == 'X' as i32 {
                q = q.offset(-1);
                let fresh44 = p;
                p = p.offset(1);
                ch_0 = *fresh44 as std::os::raw::c_int;
                b = 16 as std::os::raw::c_int
            } else if (*tcc_state).tcc_ext as std::os::raw::c_int != 0
                && (ch_0 == 'b' as i32 || ch_0 == 'B' as i32)
            {
                q = q.offset(-1);
                let fresh45 = p;
                p = p.offset(1);
                ch_0 = *fresh45 as std::os::raw::c_int;
                b = 2 as std::os::raw::c_int
            }
        }
        loop
        /* parse all digits. cannot check octal numbers at this stage
        because of floating point constants */
        {
            if ch_0 >= 'a' as i32 && ch_0 <= 'f' as i32 {
                t = ch_0 - 'a' as i32 + 10 as std::os::raw::c_int
            } else if ch_0 >= 'A' as i32 && ch_0 <= 'F' as i32 {
                t = ch_0 - 'A' as i32 + 10 as std::os::raw::c_int
            } else {
                if !(isnum(ch_0) != 0) {
                    current_block = 15897653523371991391;
                    break;
                }
                t = ch_0 - '0' as i32
            }
            if t >= b {
                current_block = 15897653523371991391;
                break;
            }
            if q >= token_buf
                .as_mut_ptr()
                .offset(1024 as std::os::raw::c_int as isize)
            {
                current_block = 137604303107109226;
                break;
            }
            let fresh46 = q;
            q = q.offset(1);
            *fresh46 = ch_0 as std::os::raw::c_char;
            let fresh47 = p;
            p = p.offset(1);
            ch_0 = *fresh47 as std::os::raw::c_int
        }
        match current_block {
            137604303107109226 => {},
            _ => {
                if ch_0 == '.' as i32
                    || (ch_0 == 'e' as i32 || ch_0 == 'E' as i32) && b == 10 as std::os::raw::c_int
                    || (ch_0 == 'p' as i32 || ch_0 == 'P' as i32)
                        && (b == 16 as std::os::raw::c_int || b == 2 as std::os::raw::c_int)
                {
                    if b != 10 as std::os::raw::c_int {
                        /* NOTE: strtox should support that for hexa numbers, but
                        non ISOC99 libcs do not support it, so we prefer to do
                        it by hand */
                        /* hexadecimal or binary floats */
                        /* XXX: handle overflows */
                        *q = '\u{0}' as i32 as std::os::raw::c_char;
                        if b == 16 as std::os::raw::c_int {
                            shift = 4 as std::os::raw::c_int
                        } else {
                            shift = 1 as std::os::raw::c_int
                        }
                        bn_zero(bn.as_mut_ptr());
                        q = token_buf.as_mut_ptr();
                        loop {
                            let fresh48 = q;
                            q = q.offset(1);
                            t = *fresh48 as std::os::raw::c_int;
                            if t == '\u{0}' as i32 {
                                break;
                            }
                            if t >= 'a' as i32 {
                                t = t - 'a' as i32 + 10 as std::os::raw::c_int
                            } else if t >= 'A' as i32 {
                                t = t - 'A' as i32 + 10 as std::os::raw::c_int
                            } else {
                                t = t - '0' as i32
                            }
                            bn_lshift(bn.as_mut_ptr(), shift, t);
                        }
                        frac_bits = 0 as std::os::raw::c_int;
                        if ch_0 == '.' as i32 {
                            let fresh49 = p;
                            p = p.offset(1);
                            ch_0 = *fresh49 as std::os::raw::c_int;
                            loop {
                                t = ch_0;
                                if t >= 'a' as i32 && t <= 'f' as i32 {
                                    t = t - 'a' as i32 + 10 as std::os::raw::c_int
                                } else if t >= 'A' as i32 && t <= 'F' as i32 {
                                    t = t - 'A' as i32 + 10 as std::os::raw::c_int
                                } else {
                                    if !(t >= '0' as i32 && t <= '9' as i32) {
                                        break;
                                    }
                                    t = t - '0' as i32
                                }
                                if t >= b {
                                    _tcc_error(
                                        b"invalid digit\x00" as *const u8
                                            as *const std::os::raw::c_char,
                                    );
                                }
                                bn_lshift(bn.as_mut_ptr(), shift, t);
                                frac_bits += shift;
                                let fresh50 = p;
                                p = p.offset(1);
                                ch_0 = *fresh50 as std::os::raw::c_int
                            }
                        }
                        if ch_0 != 'p' as i32 && ch_0 != 'P' as i32 {
                            expect(b"exponent\x00" as *const u8 as *const std::os::raw::c_char);
                        }
                        let fresh51 = p;
                        p = p.offset(1);
                        ch_0 = *fresh51 as std::os::raw::c_int;
                        s = 1 as std::os::raw::c_int;
                        exp_val = 0 as std::os::raw::c_int;
                        if ch_0 == '+' as i32 {
                            let fresh52 = p;
                            p = p.offset(1);
                            ch_0 = *fresh52 as std::os::raw::c_int
                        } else if ch_0 == '-' as i32 {
                            s = -(1 as std::os::raw::c_int);
                            let fresh53 = p;
                            p = p.offset(1);
                            ch_0 = *fresh53 as std::os::raw::c_int
                        }
                        if ch_0 < '0' as i32 || ch_0 > '9' as i32 {
                            expect(
                                b"exponent digits\x00" as *const u8 as *const std::os::raw::c_char,
                            );
                        }
                        while ch_0 >= '0' as i32 && ch_0 <= '9' as i32 {
                            exp_val = exp_val * 10 as std::os::raw::c_int + ch_0 - '0' as i32;
                            let fresh54 = p;
                            p = p.offset(1);
                            ch_0 = *fresh54 as std::os::raw::c_int
                        }
                        exp_val = exp_val * s;
                        /* now we can generate the number */
                        /* XXX: should patch directly float number */
                        d = bn[1 as std::os::raw::c_int as usize] as std::os::raw::c_double
                            * 4294967296.0f64
                            + bn[0 as std::os::raw::c_int as usize] as std::os::raw::c_double;
                        d = ldexp(d, exp_val - frac_bits);
                        t = toup(ch_0);
                        if t == 'F' as i32 {
                            let fresh55 = p;
                            p = p.offset(1);
                            ch_0 = *fresh55 as std::os::raw::c_int;
                            tok = 0xca as std::os::raw::c_int;
                            /* float : should handle overflow */
                            tokc.f = d as std::os::raw::c_float
                        } else if t == 'L' as i32 {
                            let fresh56 = p;
                            p = p.offset(1);
                            ch_0 = *fresh56 as std::os::raw::c_int;
                            tok = 0xcc as std::os::raw::c_int;
                            /* XXX: not large enough */
                            tokc.ld = f128::f128::new(d)
                        } else {
                            tok = 0xcb as std::os::raw::c_int;
                            tokc.d = d
                        }
                        current_block = 16167632229894708628;
                    } else if ch_0 == '.' as i32 {
                        if q >= token_buf
                            .as_mut_ptr()
                            .offset(1024 as std::os::raw::c_int as isize)
                        {
                            current_block = 137604303107109226;
                        } else {
                            let fresh57 = q;
                            q = q.offset(1);
                            *fresh57 = ch_0 as std::os::raw::c_char;
                            let fresh58 = p;
                            p = p.offset(1);
                            ch_0 = *fresh58 as std::os::raw::c_int;
                            current_block = 8140372313878014523;
                        }
                    } else {
                        current_block = 11674240781755647963;
                    }
                } else {
                    let mut n: std::os::raw::c_ulonglong = 0;
                    let mut n1: std::os::raw::c_ulonglong = 0;
                    let mut lcount: std::os::raw::c_int = 0;
                    let mut ucount: std::os::raw::c_int = 0;
                    let mut ov: std::os::raw::c_int = 0 as std::os::raw::c_int;
                    let mut p1: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
                    /* decimal floats */
                    /* integer number */
                    *q = '\u{0}' as i32 as std::os::raw::c_char;
                    q = token_buf.as_mut_ptr();
                    if b == 10 as std::os::raw::c_int && *q as std::os::raw::c_int == '0' as i32 {
                        b = 8 as std::os::raw::c_int;
                        q = q.offset(1)
                    }
                    n = 0 as std::os::raw::c_int as std::os::raw::c_ulonglong;
                    loop {
                        let fresh69 = q;
                        q = q.offset(1);
                        t = *fresh69 as std::os::raw::c_int;
                        /* no need for checks except for base 10 / 8 errors */
                        if t == '\u{0}' as i32 {
                            break;
                        }
                        if t >= 'a' as i32 {
                            t = t - 'a' as i32 + 10 as std::os::raw::c_int
                        } else if t >= 'A' as i32 {
                            t = t - 'A' as i32 + 10 as std::os::raw::c_int
                        } else {
                            t = t - '0' as i32
                        }
                        if t >= b {
                            _tcc_error(
                                b"invalid digit\x00" as *const u8 as *const std::os::raw::c_char,
                            );
                        }
                        n1 = n;
                        n = n
                            .wrapping_mul(b as std::os::raw::c_ulonglong)
                            .wrapping_add(t as std::os::raw::c_ulonglong);
                        /* detect overflow */
                        if n1 >= 0x1000000000000000 as std::os::raw::c_ulonglong
                            && n.wrapping_div(b as std::os::raw::c_ulonglong) != n1
                        {
                            ov = 1 as std::os::raw::c_int
                        }
                    }
                    /* Determine the characteristics (unsigned and/or 64bit) the type of
                    the constant must have according to the constant suffix(es) */
                    ucount = 0 as std::os::raw::c_int;
                    lcount = ucount;
                    p1 = p;
                    loop {
                        t = toup(ch_0);
                        if t == 'L' as i32 {
                            if lcount >= 2 as std::os::raw::c_int {
                                _tcc_error(
                                    b"three \'l\'s in integer constant\x00" as *const u8
                                        as *const std::os::raw::c_char,
                                );
                            }
                            if lcount != 0
                                && *p.offset(-(1 as std::os::raw::c_int as isize))
                                    as std::os::raw::c_int
                                    != ch_0
                            {
                                _tcc_error(
                                    b"incorrect integer suffix: %s\x00" as *const u8
                                        as *const std::os::raw::c_char,
                                    p1,
                                );
                            }
                            lcount += 1;
                            let fresh70 = p;
                            p = p.offset(1);
                            ch_0 = *fresh70 as std::os::raw::c_int
                        } else {
                            if !(t == 'U' as i32) {
                                break;
                            }
                            if ucount >= 1 as std::os::raw::c_int {
                                _tcc_error(
                                    b"two \'u\'s in integer constant\x00" as *const u8
                                        as *const std::os::raw::c_char,
                                );
                            }
                            ucount += 1;
                            let fresh71 = p;
                            p = p.offset(1);
                            ch_0 = *fresh71 as std::os::raw::c_int
                        }
                    }
                    /* Determine if it needs 64 bits and/or unsigned in order to fit */
                    if ucount == 0 as std::os::raw::c_int && b == 10 as std::os::raw::c_int {
                        if lcount
                            <= (8 as std::os::raw::c_int == 4 as std::os::raw::c_int)
                                as std::os::raw::c_int
                        {
                            if n >= 0x80000000 as std::os::raw::c_uint as std::os::raw::c_ulonglong
                            {
                                lcount = (8 as std::os::raw::c_int == 4 as std::os::raw::c_int)
                                    as std::os::raw::c_int
                                    + 1 as std::os::raw::c_int
                            }
                        } /* TOK_CU... */
                        if n >= 0x8000000000000000 as std::os::raw::c_ulonglong {
                            ov = 1 as std::os::raw::c_int;
                            ucount = 1 as std::os::raw::c_int
                        }
                    } else {
                        if lcount
                            <= (8 as std::os::raw::c_int == 4 as std::os::raw::c_int)
                                as std::os::raw::c_int
                        {
                            if n >= 0x100000000 as std::os::raw::c_ulonglong {
                                lcount = (8 as std::os::raw::c_int == 4 as std::os::raw::c_int)
                                    as std::os::raw::c_int
                                    + 1 as std::os::raw::c_int
                            } else if n
                                >= 0x80000000 as std::os::raw::c_uint as std::os::raw::c_ulonglong
                            {
                                ucount = 1 as std::os::raw::c_int
                            }
                        }
                        if n >= 0x8000000000000000 as std::os::raw::c_ulonglong {
                            ucount = 1 as std::os::raw::c_int
                        }
                    }
                    if ov != 0 {
                        _tcc_warning(
                            b"integer constant overflow\x00" as *const u8
                                as *const std::os::raw::c_char,
                        );
                    }
                    tok = 0xc2 as std::os::raw::c_int;
                    if lcount != 0 {
                        tok = 0xc6 as std::os::raw::c_int;
                        if lcount == 2 as std::os::raw::c_int {
                            tok = 0xc4 as std::os::raw::c_int
                        }
                    }
                    if ucount != 0 {
                        tok += 1
                    }
                    tokc.i = n as uint64_t;
                    current_block = 16167632229894708628;
                }
            },
        }
    }
    's_530: loop {
        match current_block {
            137604303107109226 => {
                _tcc_error(b"number too long\x00" as *const u8 as *const std::os::raw::c_char);
            },
            8140372313878014523 => {
                if !(ch_0 >= '0' as i32 && ch_0 <= '9' as i32) {
                    current_block = 11674240781755647963;
                    continue;
                }
                if q >= token_buf
                    .as_mut_ptr()
                    .offset(1024 as std::os::raw::c_int as isize)
                {
                    current_block = 137604303107109226;
                    continue;
                }
                let fresh59 = q;
                q = q.offset(1);
                *fresh59 = ch_0 as std::os::raw::c_char;
                let fresh60 = p;
                p = p.offset(1);
                ch_0 = *fresh60 as std::os::raw::c_int;
                current_block = 8140372313878014523;
            },
            16167632229894708628 => {
                if ch_0 != 0 {
                    _tcc_error(b"invalid number\x00" as *const u8 as *const std::os::raw::c_char);
                }
                break;
            },
            _ => {
                if ch_0 == 'e' as i32 || ch_0 == 'E' as i32 {
                    if q >= token_buf
                        .as_mut_ptr()
                        .offset(1024 as std::os::raw::c_int as isize)
                    {
                        current_block = 137604303107109226;
                        continue;
                    }
                    let fresh61 = q;
                    q = q.offset(1);
                    *fresh61 = ch_0 as std::os::raw::c_char;
                    let fresh62 = p;
                    p = p.offset(1);
                    ch_0 = *fresh62 as std::os::raw::c_int;
                    if ch_0 == '-' as i32 || ch_0 == '+' as i32 {
                        if q >= token_buf
                            .as_mut_ptr()
                            .offset(1024 as std::os::raw::c_int as isize)
                        {
                            current_block = 137604303107109226;
                            continue;
                        }
                        let fresh63 = q;
                        q = q.offset(1);
                        *fresh63 = ch_0 as std::os::raw::c_char;
                        let fresh64 = p;
                        p = p.offset(1);
                        ch_0 = *fresh64 as std::os::raw::c_int
                    }
                    if ch_0 < '0' as i32 || ch_0 > '9' as i32 {
                        expect(b"exponent digits\x00" as *const u8 as *const std::os::raw::c_char);
                    }
                    while ch_0 >= '0' as i32 && ch_0 <= '9' as i32 {
                        if q >= token_buf
                            .as_mut_ptr()
                            .offset(1024 as std::os::raw::c_int as isize)
                        {
                            current_block = 137604303107109226;
                            continue 's_530;
                        }
                        let fresh65 = q;
                        q = q.offset(1);
                        *fresh65 = ch_0 as std::os::raw::c_char;
                        let fresh66 = p;
                        p = p.offset(1);
                        ch_0 = *fresh66 as std::os::raw::c_int
                    }
                }
                *q = '\u{0}' as i32 as std::os::raw::c_char;
                t = toup(ch_0);
                *__errno_location() = 0 as std::os::raw::c_int;
                if t == 'F' as i32 {
                    let fresh67 = p;
                    p = p.offset(1);
                    ch_0 = *fresh67 as std::os::raw::c_int;
                    tok = 0xca as std::os::raw::c_int;
                    tokc.f = strtof(token_buf.as_mut_ptr(), 0 as *mut *mut std::os::raw::c_char)
                } else if t == 'L' as i32 {
                    let fresh68 = p;
                    p = p.offset(1);
                    ch_0 = *fresh68 as std::os::raw::c_int;
                    tok = 0xcc as std::os::raw::c_int;
                    tokc.ld = strtold(token_buf.as_mut_ptr(), 0 as *mut *mut std::os::raw::c_char)
                } else {
                    tok = 0xcb as std::os::raw::c_int;
                    tokc.d = strtod(token_buf.as_mut_ptr(), 0 as *mut *mut std::os::raw::c_char)
                }
                current_block = 16167632229894708628;
            },
        }
    }
}
/* return next token without macro substitution */
#[inline]
unsafe extern "C" fn next_nomacro1() {
    let mut current_block: u64;
    let mut t: std::os::raw::c_int = 0;
    let mut c: std::os::raw::c_int = 0;
    let mut is_long: std::os::raw::c_int = 0;
    let mut len: std::os::raw::c_int = 0;
    let mut ts: *mut TokenSym = 0 as *mut TokenSym;
    let mut p: *mut uint8_t = 0 as *mut uint8_t;
    let mut p1: *mut uint8_t = 0 as *mut uint8_t;
    let mut h: std::os::raw::c_uint = 0;
    p = (*file).buf_ptr;
    loop {
        c = *p as std::os::raw::c_int;
        match c {
            32 | 9 => {
                tok = c;
                p = p.offset(1);
                current_block = 7072431309075361279;
            },
            12 | 11 | 13 => {
                p = p.offset(1);
                continue;
            },
            92 => {
                /* first look if it is in fact an end of buffer */
                c = handle_stray1(p);
                p = (*file).buf_ptr;
                if c == '\\' as i32 {
                    current_block = 14537325084018676057;
                    break;
                }
                if c != -(1 as std::os::raw::c_int) {
                    continue;
                }
                let mut s1: *mut TCCState = tcc_state;
                if parse_flags & 0x4 as std::os::raw::c_int != 0
                    && tok_flags & 0x8 as std::os::raw::c_int == 0
                {
                    tok_flags |= 0x8 as std::os::raw::c_int;
                    tok = 10 as std::os::raw::c_int;
                    current_block = 6164085797154605288;
                    break;
                } else if parse_flags & 0x1 as std::os::raw::c_int == 0 {
                    tok = -(1 as std::os::raw::c_int);
                    current_block = 5571491539924227058;
                    break;
                } else if (*s1).ifdef_stack_ptr != (*file).ifdef_stack_ptr {
                    _tcc_error(b"missing #endif\x00" as *const u8 as *const std::os::raw::c_char);
                } else if (*s1).include_stack_ptr == (*s1).include_stack.as_mut_ptr() {
                    /* no include left : end of file. */
                    tok = -(1 as std::os::raw::c_int);
                    current_block = 5571491539924227058;
                    break;
                } else {
                    tok_flags &= !(0x8 as std::os::raw::c_int);
                    /* pop include file */
                    /* test if previous '#endif' was after a #ifdef at
                    start of file */
                    if tok_flags & 0x4 as std::os::raw::c_int != 0 {
                        (*search_cached_include(
                            s1,
                            (*file).filename.as_mut_ptr(),
                            1 as std::os::raw::c_int,
                        ))
                        .ifndef_macro = (*file).ifndef_macro_saved;
                        tok_flags &= !(0x4 as std::os::raw::c_int)
                    }
                    /* add end of include file debug info */
                    tcc_debug_eincl(tcc_state);
                    /* pop include stack */
                    tcc_close();
                    (*s1).include_stack_ptr = (*s1).include_stack_ptr.offset(-1);
                    p = (*file).buf_ptr;
                    if p == (*file).buffer.as_mut_ptr() {
                        tok_flags = 0x2 as std::os::raw::c_int | 0x1 as std::os::raw::c_int
                    }
                    continue;
                }
            },
            10 => {
                (*file).line_num += 1;
                tok_flags |= 0x1 as std::os::raw::c_int;
                p = p.offset(1);
                current_block = 13182150892604052619;
            },
            35 => {
                /* XXX: simplify */
                p = p.offset(1);
                c = *p as std::os::raw::c_int;
                if c == '\\' as i32 {
                    c = handle_stray1(p);
                    p = (*file).buf_ptr
                }
                if tok_flags & 0x1 as std::os::raw::c_int != 0
                    && parse_flags & 0x1 as std::os::raw::c_int != 0
                {
                    (*file).buf_ptr = p;
                    preprocess(tok_flags & 0x2 as std::os::raw::c_int);
                    p = (*file).buf_ptr
                } else if c == '#' as i32 {
                    p = p.offset(1);
                    tok = 0xa3 as std::os::raw::c_int;
                    current_block = 5571491539924227058;
                    break;
                } else if parse_flags & 0x8 as std::os::raw::c_int != 0 {
                    p = parse_line_comment(p.offset(-(1 as std::os::raw::c_int as isize)));
                    continue;
                } else {
                    tok = '#' as i32;
                    current_block = 5571491539924227058;
                    break;
                }
                current_block = 13182150892604052619;
            },
            36 => {
                /* dollar is allowed to start identifiers when not parsing asm */
                if isidnum_table[(c - -(1 as std::os::raw::c_int)) as usize] as std::os::raw::c_int
                    & 2 as std::os::raw::c_int
                    == 0
                    || parse_flags & 0x8 as std::os::raw::c_int != 0
                {
                    current_block = 14537325084018676057;
                    break;
                } else {
                    current_block = 18040606447371207285;
                    break;
                }
            },
            97 | 98 | 99 | 100 | 101 | 102 | 103 | 104 | 105 | 106 | 107 | 108 | 109 | 110
            | 111 | 112 | 113 | 114 | 115 | 116 | 117 | 118 | 119 | 120 | 121 | 122 | 65 | 66
            | 67 | 68 | 69 | 70 | 71 | 72 | 73 | 74 | 75 | 77 | 78 | 79 | 80 | 81 | 82 | 83
            | 84 | 85 | 86 | 87 | 88 | 89 | 90 | 95 => {
                current_block = 18040606447371207285;
                break;
            },
            76 => {
                t = *p.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int;
                if t != '\\' as i32 && t != '\'' as i32 && t != '\"' as i32 {
                    current_block = 13454018412769612774;
                    break;
                } else {
                    current_block = 7923086311623215889;
                    break;
                }
            },
            48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                t = c;
                p = p.offset(1);
                c = *p as std::os::raw::c_int;
                if c == '\\' as i32 {
                    c = handle_stray1(p);
                    p = (*file).buf_ptr
                }
                current_block = 14730872864422895907;
                break;
            },
            46 => {
                /* special dot handling because it can also start a number */
                p = p.offset(1);
                c = *p as std::os::raw::c_int;
                if c == '\\' as i32 {
                    c = handle_stray1(p);
                    p = (*file).buf_ptr
                }
                if isnum(c) != 0 {
                    current_block = 2167674248514145403;
                    break;
                } else {
                    current_block = 18312853480645871422;
                    break;
                }
            },
            39 | 34 => {
                is_long = 0 as std::os::raw::c_int;
                current_block = 12320421619917999938;
                break;
            },
            60 => {
                p = p.offset(1);
                c = *p as std::os::raw::c_int;
                if c == '\\' as i32 {
                    c = handle_stray1(p);
                    p = (*file).buf_ptr
                }
                if c == '=' as i32 {
                    p = p.offset(1);
                    tok = 0x9e as std::os::raw::c_int
                } else if c == '<' as i32 {
                    p = p.offset(1);
                    c = *p as std::os::raw::c_int;
                    if c == '\\' as i32 {
                        c = handle_stray1(p);
                        p = (*file).buf_ptr
                    }
                    if c == '=' as i32 {
                        p = p.offset(1);
                        tok = 0xb8 as std::os::raw::c_int
                    } else {
                        tok = '<' as i32
                    }
                } else {
                    tok = 0x9c as std::os::raw::c_int
                }
                current_block = 5571491539924227058;
                break;
            },
            62 => {
                p = p.offset(1);
                c = *p as std::os::raw::c_int;
                if c == '\\' as i32 {
                    c = handle_stray1(p);
                    p = (*file).buf_ptr
                }
                if c == '=' as i32 {
                    p = p.offset(1);
                    tok = 0x9d as std::os::raw::c_int
                } else if c == '>' as i32 {
                    p = p.offset(1);
                    c = *p as std::os::raw::c_int;
                    if c == '\\' as i32 {
                        c = handle_stray1(p);
                        p = (*file).buf_ptr
                    }
                    if c == '=' as i32 {
                        p = p.offset(1);
                        tok = 0xb9 as std::os::raw::c_int
                    } else {
                        tok = '>' as i32
                    }
                } else {
                    tok = 0x9f as std::os::raw::c_int
                }
                current_block = 5571491539924227058;
                break;
            },
            38 => {
                p = p.offset(1);
                c = *p as std::os::raw::c_int;
                if c == '\\' as i32 {
                    c = handle_stray1(p);
                    p = (*file).buf_ptr
                }
                if c == '&' as i32 {
                    p = p.offset(1);
                    tok = 0x90 as std::os::raw::c_int
                } else if c == '=' as i32 {
                    p = p.offset(1);
                    tok = 0xb5 as std::os::raw::c_int
                } else {
                    tok = '&' as i32
                }
                current_block = 5571491539924227058;
                break;
            },
            124 => {
                p = p.offset(1);
                c = *p as std::os::raw::c_int;
                if c == '\\' as i32 {
                    c = handle_stray1(p);
                    p = (*file).buf_ptr
                }
                if c == '|' as i32 {
                    p = p.offset(1);
                    tok = 0x91 as std::os::raw::c_int
                } else if c == '=' as i32 {
                    p = p.offset(1);
                    tok = 0xb6 as std::os::raw::c_int
                } else {
                    tok = '|' as i32
                }
                current_block = 5571491539924227058;
                break;
            },
            43 => {
                p = p.offset(1);
                c = *p as std::os::raw::c_int;
                if c == '\\' as i32 {
                    c = handle_stray1(p);
                    p = (*file).buf_ptr
                }
                if c == '+' as i32 {
                    p = p.offset(1);
                    tok = 0x82 as std::os::raw::c_int
                } else if c == '=' as i32 {
                    p = p.offset(1);
                    tok = 0xb0 as std::os::raw::c_int
                } else {
                    tok = '+' as i32
                }
                current_block = 5571491539924227058;
                break;
            },
            45 => {
                p = p.offset(1);
                c = *p as std::os::raw::c_int;
                if c == '\\' as i32 {
                    c = handle_stray1(p);
                    p = (*file).buf_ptr
                }
                if c == '-' as i32 {
                    p = p.offset(1);
                    tok = 0x80 as std::os::raw::c_int
                } else if c == '=' as i32 {
                    p = p.offset(1);
                    tok = 0xb1 as std::os::raw::c_int
                } else if c == '>' as i32 {
                    p = p.offset(1);
                    tok = 0xa0 as std::os::raw::c_int
                } else {
                    tok = '-' as i32
                }
                current_block = 5571491539924227058;
                break;
            },
            33 => {
                p = p.offset(1);
                c = *p as std::os::raw::c_int;
                if c == '\\' as i32 {
                    c = handle_stray1(p);
                    p = (*file).buf_ptr
                }
                if c == '=' as i32 {
                    p = p.offset(1);
                    tok = 0x95 as std::os::raw::c_int
                } else {
                    tok = '!' as i32
                }
                current_block = 5571491539924227058;
                break;
            },
            61 => {
                p = p.offset(1);
                c = *p as std::os::raw::c_int;
                if c == '\\' as i32 {
                    c = handle_stray1(p);
                    p = (*file).buf_ptr
                }
                if c == '=' as i32 {
                    p = p.offset(1);
                    tok = 0x94 as std::os::raw::c_int
                } else {
                    tok = '=' as i32
                }
                current_block = 5571491539924227058;
                break;
            },
            42 => {
                p = p.offset(1);
                c = *p as std::os::raw::c_int;
                if c == '\\' as i32 {
                    c = handle_stray1(p);
                    p = (*file).buf_ptr
                }
                if c == '=' as i32 {
                    p = p.offset(1);
                    tok = 0xb2 as std::os::raw::c_int
                } else {
                    tok = '*' as i32
                }
                current_block = 5571491539924227058;
                break;
            },
            37 => {
                p = p.offset(1);
                c = *p as std::os::raw::c_int;
                if c == '\\' as i32 {
                    c = handle_stray1(p);
                    p = (*file).buf_ptr
                }
                if c == '=' as i32 {
                    p = p.offset(1);
                    tok = 0xb4 as std::os::raw::c_int
                } else {
                    tok = '%' as i32
                }
                current_block = 5571491539924227058;
                break;
            },
            94 => {
                p = p.offset(1);
                c = *p as std::os::raw::c_int;
                if c == '\\' as i32 {
                    c = handle_stray1(p);
                    p = (*file).buf_ptr
                }
                if c == '=' as i32 {
                    p = p.offset(1);
                    tok = 0xb7 as std::os::raw::c_int
                } else {
                    tok = '^' as i32
                }
                current_block = 5571491539924227058;
                break;
            },
            47 => {
                /* comments or operator */
                p = p.offset(1);
                c = *p as std::os::raw::c_int;
                if c == '\\' as i32 {
                    c = handle_stray1(p);
                    p = (*file).buf_ptr
                }
                if c == '*' as i32 {
                    p = parse_comment(p);
                    /* comments replaced by a blank */
                    tok = ' ' as i32
                } else if c == '/' as i32 {
                    p = parse_line_comment(p); /* may underflow into file->unget[] */
                    tok = ' ' as i32
                } else {
                    if c == '=' as i32 {
                        p = p.offset(1);
                        tok = 0xb3 as std::os::raw::c_int
                    } else {
                        tok = '/' as i32
                    }
                    current_block = 5571491539924227058;
                    break;
                }
                current_block = 7072431309075361279;
            },
            40 | 41 | 91 | 93 | 123 | 125 | 44 | 59 | 58 | 63 | 126 | 64 => {
                current_block = 14537325084018676057;
                break;
            },
            _ => {
                if c >= 0x80 as std::os::raw::c_int && c <= 0xff as std::os::raw::c_int {
                    current_block = 18040606447371207285;
                    break;
                } else {
                    current_block = 408978414928704595;
                    break;
                }
            },
        }
        match current_block {
            7072431309075361279 => {
                if parse_flags & 0x10 as std::os::raw::c_int != 0 {
                    current_block = 6164085797154605288;
                    break;
                }
                while isidnum_table
                    [(*p as std::os::raw::c_int - -(1 as std::os::raw::c_int)) as usize]
                    as std::os::raw::c_int
                    & 1 as std::os::raw::c_int
                    != 0
                {
                    p = p.offset(1)
                }
            },
            _ => {
                if 0 as std::os::raw::c_int == parse_flags & 0x4 as std::os::raw::c_int {
                    continue;
                }
                tok = 10 as std::os::raw::c_int;
                current_block = 6164085797154605288;
                break;
            },
        }
    }
    match current_block {
        7923086311623215889 => {
            p = p.offset(1);
            c = *p as std::os::raw::c_int;
            if c == '\\' as i32 {
                c = handle_stray1(p);
                p = (*file).buf_ptr
            }
            if c == '\'' as i32 || c == '\"' as i32 {
                is_long = 1 as std::os::raw::c_int;
                current_block = 12320421619917999938;
            } else {
                cstr_reset(&mut tokcstr);
                cstr_ccat(&mut tokcstr, 'L' as i32);
                current_block = 6782299445110405414;
            }
        },
        18312853480645871422 => {
            if isidnum_table[('.' as i32 - -(1 as std::os::raw::c_int)) as usize]
                as std::os::raw::c_int
                & 2 as std::os::raw::c_int
                != 0
                && isidnum_table[(c - -(1 as std::os::raw::c_int)) as usize] as std::os::raw::c_int
                    & (2 as std::os::raw::c_int | 4 as std::os::raw::c_int)
                    != 0
            {
                c = '.' as i32;
                p = p.offset(-1);
                *p = c as uint8_t;
                current_block = 18040606447371207285;
            } else {
                if c == '.' as i32 {
                    p = p.offset(1);
                    c = *p as std::os::raw::c_int;
                    if c == '\\' as i32 {
                        c = handle_stray1(p);
                        p = (*file).buf_ptr
                    }
                    if c == '.' as i32 {
                        p = p.offset(1);
                        tok = 0xa1 as std::os::raw::c_int
                    } else {
                        p = p.offset(-1);
                        *p = '.' as i32 as uint8_t;
                        tok = '.' as i32
                    }
                } else {
                    tok = '.' as i32
                }
                current_block = 5571491539924227058;
            }
        },
        408978414928704595 => {
            if parse_flags & 0x8 as std::os::raw::c_int != 0 {
                current_block = 14537325084018676057;
            } else {
                _tcc_error(
                    b"unrecognized character \\x%02x\x00" as *const u8
                        as *const std::os::raw::c_char,
                    c,
                );
            }
        },
        2167674248514145403 => {
            t = '.' as i32;
            current_block = 14730872864422895907;
        },
        13454018412769612774 =>
        /* fast case */
        {
            current_block = 18040606447371207285;
        }
        _ => {},
    }
    match current_block {
        12320421619917999938 => {
            cstr_reset(&mut tokcstr);
            if is_long != 0 {
                cstr_ccat(&mut tokcstr, 'L' as i32);
            }
            cstr_ccat(&mut tokcstr, c);
            p = parse_pp_string(p, c, &mut tokcstr);
            cstr_ccat(&mut tokcstr, c);
            cstr_ccat(&mut tokcstr, '\u{0}' as i32);
            tokc.str_0.size = tokcstr.size;
            tokc.str_0.data = tokcstr.data;
            tok = 0xce as std::os::raw::c_int;
            current_block = 5571491539924227058;
        },
        14730872864422895907 =>
        /* after the first digit, accept digits, alpha, '.' or sign if
        prefixed by 'eEpP' */
        {
            cstr_reset(&mut tokcstr);
            loop {
                cstr_ccat(&mut tokcstr, t);
                if !(isidnum_table[(c - -(1 as std::os::raw::c_int)) as usize]
                    as std::os::raw::c_int
                    & (2 as std::os::raw::c_int | 4 as std::os::raw::c_int)
                    != 0
                    || c == '.' as i32
                    || (c == '+' as i32 || c == '-' as i32)
                        && ((t == 'e' as i32 || t == 'E' as i32)
                            && !(parse_flags & 0x8 as std::os::raw::c_int != 0
                                && *(tokcstr.data as *mut std::os::raw::c_char)
                                    .offset(0 as std::os::raw::c_int as isize)
                                    as std::os::raw::c_int
                                    == '0' as i32
                                && toup(
                                    *(tokcstr.data as *mut std::os::raw::c_char)
                                        .offset(1 as std::os::raw::c_int as isize)
                                        as std::os::raw::c_int,
                                ) == 'X' as i32)
                            || t == 'p' as i32
                            || t == 'P' as i32))
                {
                    break;
                }
                t = c;
                p = p.offset(1);
                c = *p as std::os::raw::c_int;
                if c == '\\' as i32 {
                    c = handle_stray1(p);
                    p = (*file).buf_ptr
                }
            }
            /* We add a trailing '\0' to ease parsing */
            cstr_ccat(&mut tokcstr, '\u{0}' as i32);
            tokc.str_0.size = tokcstr.size;
            tokc.str_0.data = tokcstr.data;
            tok = 0xcd as std::os::raw::c_int;
            current_block = 5571491539924227058;
        }
        14537325084018676057 =>
        /* only used in assembler */
        {
            tok = c;
            p = p.offset(1);
            current_block = 5571491539924227058;
        }
        18040606447371207285 =>
        /* utf8 identifiers */
        {
            p1 = p;
            h = 1 as std::os::raw::c_int as std::os::raw::c_uint;
            h = h
                .wrapping_add(h << 5 as std::os::raw::c_int)
                .wrapping_add(h >> 27 as std::os::raw::c_int)
                .wrapping_add(c as std::os::raw::c_uint);
            loop {
                p = p.offset(1);
                c = *p as std::os::raw::c_int;
                if !(isidnum_table[(c - -(1 as std::os::raw::c_int)) as usize]
                    as std::os::raw::c_int
                    & (2 as std::os::raw::c_int | 4 as std::os::raw::c_int)
                    != 0)
                {
                    break;
                }
                h = h
                    .wrapping_add(h << 5 as std::os::raw::c_int)
                    .wrapping_add(h >> 27 as std::os::raw::c_int)
                    .wrapping_add(c as std::os::raw::c_uint)
            }
            len = p.offset_from(p1) as std::os::raw::c_long as std::os::raw::c_int;
            if c != '\\' as i32 {
                let mut current_block_60: u64;
                let mut pts: *mut *mut TokenSym = 0 as *mut *mut TokenSym;
                /* fast case : no stray found, so we have the full token
                and we have already hashed it */
                h &= (16384 as std::os::raw::c_int - 1 as std::os::raw::c_int)
                    as std::os::raw::c_uint;
                pts = &mut *hash_ident.as_mut_ptr().offset(h as isize) as *mut *mut TokenSym;
                loop {
                    ts = *pts;
                    if ts.is_null() {
                        current_block_60 = 14155412868135599428;
                        break;
                    }
                    if (*ts).len == len
                        && memcmp(
                            (*ts).str_0.as_mut_ptr() as *const std::os::raw::c_void,
                            p1 as *const std::os::raw::c_void,
                            len as std::os::raw::c_ulong,
                        ) == 0
                    {
                        current_block_60 = 14483658890531361756;
                        break;
                    }
                    pts = &mut (*ts).hash_next
                }
                match current_block_60 {
                    14155412868135599428 => {
                        ts = tok_alloc_new(pts, p1 as *mut std::os::raw::c_char, len)
                    },
                    _ => {},
                }
                current_block = 3267183688118022636;
            } else {
                /* slower case */
                cstr_reset(&mut tokcstr);
                cstr_cat(&mut tokcstr, p1 as *mut std::os::raw::c_char, len);
                p = p.offset(-1);
                p = p.offset(1);
                c = *p as std::os::raw::c_int;
                if c == '\\' as i32 {
                    c = handle_stray1(p);
                    p = (*file).buf_ptr
                }
                current_block = 6782299445110405414;
            }
        }
        _ => {},
    }
    match current_block {
        6782299445110405414 => {
            while isidnum_table[(c - -(1 as std::os::raw::c_int)) as usize] as std::os::raw::c_int
                & (2 as std::os::raw::c_int | 4 as std::os::raw::c_int)
                != 0
            {
                cstr_ccat(&mut tokcstr, c);
                p = p.offset(1);
                c = *p as std::os::raw::c_int;
                if c == '\\' as i32 {
                    c = handle_stray1(p);
                    p = (*file).buf_ptr
                }
            }
            ts = tok_alloc(tokcstr.data as *const std::os::raw::c_char, tokcstr.size);
            current_block = 3267183688118022636;
        },
        _ => {},
    }
    match current_block {
        3267183688118022636 => {
            tok = (*ts).tok;
            current_block = 5571491539924227058;
        },
        _ => {},
    }
    match current_block {
        5571491539924227058 => tok_flags = 0 as std::os::raw::c_int,
        _ => {},
    }
    (*file).buf_ptr = p;
}
/* substitute arguments in replacement lists in macro_str by the values in
args (field d) and return allocated string */
unsafe extern "C" fn macro_arg_subst(
    mut nested_list: *mut *mut Sym,
    mut macro_str: *const std::os::raw::c_int,
    mut args: *mut Sym,
) -> *mut std::os::raw::c_int {
    let mut t: std::os::raw::c_int = 0;
    let mut t0: std::os::raw::c_int = 0;
    let mut t1: std::os::raw::c_int = 0;
    let mut spc: std::os::raw::c_int = 0;
    let mut st: *const std::os::raw::c_int = 0 as *const std::os::raw::c_int;
    let mut s: *mut Sym = 0 as *mut Sym;
    let mut cval: CValue = CValue {
        ld: f128::f128::ZERO,
    };
    let mut str: TokenString = TokenString {
        str_0: 0 as *const std::os::raw::c_int as *mut std::os::raw::c_int,
        len: 0,
        lastlen: 0,
        allocated_len: 0,
        last_line_num: 0,
        save_line_num: 0,
        prev: 0 as *const TokenString as *mut TokenString,
        prev_ptr: 0 as *const std::os::raw::c_int,
        alloc: 0,
    };
    let mut cstr: CString = CString {
        size: 0,
        data: 0 as *const std::os::raw::c_void as *mut std::os::raw::c_void,
        size_allocated: 0,
    };
    tok_str_new(&mut str);
    t1 = 0 as std::os::raw::c_int;
    t0 = t1;
    loop {
        let mut _t: std::os::raw::c_int = *macro_str;
        if _t >= 0xc0 as std::os::raw::c_int && _t <= 0xcf as std::os::raw::c_int {
            tok_get(&mut t, &mut macro_str, &mut cval);
        } else {
            t = _t;
            macro_str = macro_str.offset(1)
        }
        if t == 0 {
            break;
        }
        if t == '#' as i32 {
            's_247: {
                /* stringize */
                let mut _t_0: std::os::raw::c_int = *macro_str;
                if _t_0 >= 0xc0 as std::os::raw::c_int && _t_0 <= 0xcf as std::os::raw::c_int {
                    tok_get(&mut t, &mut macro_str, &mut cval);
                } else {
                    t = _t_0;
                    macro_str = macro_str.offset(1)
                }
                if !(t == 0) {
                    s = sym_find2(args, t);
                    if !s.is_null() {
                        cstr_new(&mut cstr);
                        cstr_ccat(&mut cstr, '\"' as i32);
                        st = (*s).c2rust_unnamed.d;
                        spc = 0 as std::os::raw::c_int;
                        while *st >= 0 as std::os::raw::c_int {
                            let mut _t_1: std::os::raw::c_int = *st;
                            if _t_1 >= 0xc0 as std::os::raw::c_int
                                && _t_1 <= 0xcf as std::os::raw::c_int
                            {
                                tok_get(&mut t, &mut st, &mut cval);
                            } else {
                                t = _t_1;
                                st = st.offset(1)
                            }
                            if t != 0xa4 as std::os::raw::c_int
                                && t != 0xa5 as std::os::raw::c_int
                                && 0 as std::os::raw::c_int == check_space(t, &mut spc)
                            {
                                let mut s_0: *const std::os::raw::c_char =
                                    get_tok_str(t, &mut cval);
                                while *s_0 != 0 {
                                    if t == 0xce as std::os::raw::c_int
                                        && *s_0 as std::os::raw::c_int != '\'' as i32
                                    {
                                        add_char(&mut cstr, *s_0 as std::os::raw::c_int);
                                    } else {
                                        cstr_ccat(&mut cstr, *s_0 as std::os::raw::c_int);
                                    }
                                    s_0 = s_0.offset(1)
                                }
                            }
                        }
                        cstr.size -= spc;
                        cstr_ccat(&mut cstr, '\"' as i32);
                        cstr_ccat(&mut cstr, '\u{0}' as i32);
                        /* add string */
                        cval.str_0.size = cstr.size;
                        cval.str_0.data = cstr.data;
                        tok_str_add2(&mut str, 0xce as std::os::raw::c_int, &mut cval);
                        cstr_free(&mut cstr);
                        break 's_247;
                    }
                }
                expect(
                    b"macro parameter after \'#\'\x00" as *const u8 as *const std::os::raw::c_char,
                );
            }
        } else if t >= 256 as std::os::raw::c_int {
            s = sym_find2(args, t);
            if !s.is_null() {
                let mut l0: std::os::raw::c_int = str.len;
                st = (*s).c2rust_unnamed.d;
                let mut current_block_54: u64;
                /* if '##' is present before or after, no arg substitution */
                if *macro_str == 0xa6 as std::os::raw::c_int || t1 == 0xa6 as std::os::raw::c_int {
                    /* special case for var arg macros : ## eats the ','
                    if empty VA_ARGS variable. */
                    if t1 == 0xa6 as std::os::raw::c_int
                        && t0 == ',' as i32
                        && (*tcc_state).gnu_ext as std::os::raw::c_int != 0
                        && (*s).type_0.t != 0
                    {
                        if *st <= 0 as std::os::raw::c_int {
                            /* suppress ',' '##' */
                            str.len -= 2 as std::os::raw::c_int;
                            current_block_54 = 7189308829251266000;
                        } else {
                            /* suppress '##' and add variable */
                            str.len -= 1;
                            current_block_54 = 2107516719646657291;
                        }
                    } else {
                        current_block_54 = 7189308829251266000;
                    }
                } else {
                    current_block_54 = 2107516719646657291;
                }
                match current_block_54 {
                    2107516719646657291 => {
                        if (*s).c2rust_unnamed_0.next.is_null() {
                            /* Expand arguments tokens and store them.  In most
                            cases we could also re-expand each argument if
                            used multiple times, but not if the argument
                            contains the __COUNTER__ macro.  */
                            let mut str2: TokenString = TokenString {
                                str_0: 0 as *const std::os::raw::c_int as *mut std::os::raw::c_int,
                                len: 0,
                                lastlen: 0,
                                allocated_len: 0,
                                last_line_num: 0,
                                save_line_num: 0,
                                prev: 0 as *const TokenString as *mut TokenString,
                                prev_ptr: 0 as *const std::os::raw::c_int,
                                alloc: 0,
                            };
                            sym_push2(
                                &mut (*s).c2rust_unnamed_0.next,
                                (*s).v,
                                (*s).type_0.t,
                                0 as std::os::raw::c_int,
                            );
                            tok_str_new(&mut str2);
                            macro_subst(&mut str2, nested_list, st);
                            tok_str_add(&mut str2, 0 as std::os::raw::c_int);
                            (*(*s).c2rust_unnamed_0.next).c2rust_unnamed.d = str2.str_0
                        }
                        st = (*(*s).c2rust_unnamed_0.next).c2rust_unnamed.d
                    },
                    _ => {},
                }
                loop {
                    let mut t2: std::os::raw::c_int = 0;
                    let mut _t_2: std::os::raw::c_int = *st;
                    if _t_2 >= 0xc0 as std::os::raw::c_int && _t_2 <= 0xcf as std::os::raw::c_int {
                        tok_get(&mut t2, &mut st, &mut cval);
                    } else {
                        t2 = _t_2;
                        st = st.offset(1)
                    }
                    if t2 <= 0 as std::os::raw::c_int {
                        break;
                    }
                    tok_str_add2(&mut str, t2, &mut cval);
                }
                if str.len == l0 {
                    /* expanded to empty string */
                    tok_str_add(&mut str, 0xa4 as std::os::raw::c_int);
                }
            } else {
                tok_str_add(&mut str, t);
            }
        } else {
            tok_str_add2(&mut str, t, &mut cval);
        }
        t0 = t1;
        t1 = t
    }
    tok_str_add(&mut str, 0 as std::os::raw::c_int);
    return str.str_0;
}
static mut ab_month_name: [[std::os::raw::c_char; 4]; 12] = unsafe {
    [
        *::std::mem::transmute::<&[u8; 4], &[std::os::raw::c_char; 4]>(b"Jan\x00"),
        *::std::mem::transmute::<&[u8; 4], &[std::os::raw::c_char; 4]>(b"Feb\x00"),
        *::std::mem::transmute::<&[u8; 4], &[std::os::raw::c_char; 4]>(b"Mar\x00"),
        *::std::mem::transmute::<&[u8; 4], &[std::os::raw::c_char; 4]>(b"Apr\x00"),
        *::std::mem::transmute::<&[u8; 4], &[std::os::raw::c_char; 4]>(b"May\x00"),
        *::std::mem::transmute::<&[u8; 4], &[std::os::raw::c_char; 4]>(b"Jun\x00"),
        *::std::mem::transmute::<&[u8; 4], &[std::os::raw::c_char; 4]>(b"Jul\x00"),
        *::std::mem::transmute::<&[u8; 4], &[std::os::raw::c_char; 4]>(b"Aug\x00"),
        *::std::mem::transmute::<&[u8; 4], &[std::os::raw::c_char; 4]>(b"Sep\x00"),
        *::std::mem::transmute::<&[u8; 4], &[std::os::raw::c_char; 4]>(b"Oct\x00"),
        *::std::mem::transmute::<&[u8; 4], &[std::os::raw::c_char; 4]>(b"Nov\x00"),
        *::std::mem::transmute::<&[u8; 4], &[std::os::raw::c_char; 4]>(b"Dec\x00"),
    ]
};
unsafe extern "C" fn paste_tokens(
    mut t1: std::os::raw::c_int,
    mut v1: *mut CValue,
    mut t2: std::os::raw::c_int,
    mut v2: *mut CValue,
) -> std::os::raw::c_int {
    let mut cstr: CString = CString {
        size: 0,
        data: 0 as *const std::os::raw::c_void as *mut std::os::raw::c_void,
        size_allocated: 0,
    };
    let mut n: std::os::raw::c_int = 0;
    let mut ret: std::os::raw::c_int = 1 as std::os::raw::c_int;
    cstr_new(&mut cstr);
    if t1 != 0xa4 as std::os::raw::c_int {
        cstr_cat(&mut cstr, get_tok_str(t1, v1), -(1 as std::os::raw::c_int));
    }
    n = cstr.size;
    if t2 != 0xa4 as std::os::raw::c_int {
        cstr_cat(&mut cstr, get_tok_str(t2, v2), -(1 as std::os::raw::c_int));
    }
    cstr_ccat(&mut cstr, '\u{0}' as i32);
    tcc_open_bf(
        tcc_state,
        b":paste:\x00" as *const u8 as *const std::os::raw::c_char,
        cstr.size,
    );
    memcpy(
        (*file).buffer.as_mut_ptr() as *mut std::os::raw::c_void,
        cstr.data,
        cstr.size as std::os::raw::c_ulong,
    );
    tok_flags = 0 as std::os::raw::c_int;
    loop {
        next_nomacro1();
        if 0 as std::os::raw::c_int == *(*file).buf_ptr as std::os::raw::c_int {
            break;
        }
        if is_space(tok) != 0 {
            continue;
        }
        _tcc_warning(
            b"pasting \"%.*s\" and \"%s\" does not give a valid preprocessing token\x00"
                as *const u8 as *const std::os::raw::c_char,
            n,
            cstr.data as *mut std::os::raw::c_char,
            (cstr.data as *mut std::os::raw::c_char).offset(n as isize),
        );
        ret = 0 as std::os::raw::c_int;
        break;
    }
    tcc_close();
    //printf("paste <%s>\n", (char*)cstr.data);
    cstr_free(&mut cstr);
    return ret;
}
/* handle the '##' operator. Return NULL if no '##' seen. Otherwise
return the resulting string (which must be freed). */
#[inline]
unsafe extern "C" fn macro_twosharps(
    mut ptr0: *const std::os::raw::c_int,
) -> *mut std::os::raw::c_int {
    let mut t: std::os::raw::c_int = 0;
    let mut cval: CValue = CValue {
        ld: f128::f128::ZERO,
    };
    let mut macro_str1: TokenString = TokenString {
        str_0: 0 as *const std::os::raw::c_int as *mut std::os::raw::c_int,
        len: 0,
        lastlen: 0,
        allocated_len: 0,
        last_line_num: 0,
        save_line_num: 0,
        prev: 0 as *const TokenString as *mut TokenString,
        prev_ptr: 0 as *const std::os::raw::c_int,
        alloc: 0,
    };
    let mut start_of_nosubsts: std::os::raw::c_int = -(1 as std::os::raw::c_int);
    let mut ptr: *const std::os::raw::c_int = 0 as *const std::os::raw::c_int;
    /* we search the first '##' */
    ptr = ptr0;
    loop {
        let mut _t: std::os::raw::c_int = *ptr;
        if _t >= 0xc0 as std::os::raw::c_int && _t <= 0xcf as std::os::raw::c_int {
            tok_get(&mut t, &mut ptr, &mut cval);
        } else {
            t = _t;
            ptr = ptr.offset(1)
        }
        if t == 0xa6 as std::os::raw::c_int {
            break;
        }
        if t == 0 as std::os::raw::c_int {
            return 0 as *mut std::os::raw::c_int;
        }
    }
    tok_str_new(&mut macro_str1);
    //tok_print(" $$$", ptr0);
    ptr = ptr0;
    loop {
        let mut _t_0: std::os::raw::c_int = *ptr;
        if _t_0 >= 0xc0 as std::os::raw::c_int && _t_0 <= 0xcf as std::os::raw::c_int {
            tok_get(&mut t, &mut ptr, &mut cval);
        } else {
            t = _t_0;
            ptr = ptr.offset(1)
        }
        if t == 0 as std::os::raw::c_int {
            break;
        }
        if t == 0xa6 as std::os::raw::c_int {
            continue;
        }
        while *ptr == 0xa6 as std::os::raw::c_int {
            let mut t1: std::os::raw::c_int = 0;
            let mut cv1: CValue = CValue {
                ld: f128::f128::ZERO,
            };
            /* given 'a##b', remove nosubsts preceding 'a' */
            if start_of_nosubsts >= 0 as std::os::raw::c_int {
                macro_str1.len = start_of_nosubsts
            }
            loop
            /* given 'a##b', remove nosubsts preceding 'b' */
            {
                ptr = ptr.offset(1);
                t1 = *ptr;
                if !(t1 == 0xa5 as std::os::raw::c_int) {
                    break;
                }
            }
            if t1 != 0 && t1 != 0xa6 as std::os::raw::c_int {
                let mut _t_1: std::os::raw::c_int = *ptr;
                if _t_1 >= 0xc0 as std::os::raw::c_int && _t_1 <= 0xcf as std::os::raw::c_int {
                    tok_get(&mut t1, &mut ptr, &mut cv1);
                } else {
                    t1 = _t_1;
                    ptr = ptr.offset(1)
                }
                if t != 0xa4 as std::os::raw::c_int || t1 != 0xa4 as std::os::raw::c_int {
                    if paste_tokens(t, &mut cval, t1, &mut cv1) != 0 {
                        t = tok;
                        cval = tokc
                    } else {
                        tok_str_add2(&mut macro_str1, t, &mut cval);
                        t = t1;
                        cval = cv1
                    }
                }
            }
        }
        if t == 0xa5 as std::os::raw::c_int {
            if start_of_nosubsts < 0 as std::os::raw::c_int {
                start_of_nosubsts = macro_str1.len
            }
        } else {
            start_of_nosubsts = -(1 as std::os::raw::c_int)
        }
        tok_str_add2(&mut macro_str1, t, &mut cval);
    }
    tok_str_add(&mut macro_str1, 0 as std::os::raw::c_int);
    //tok_print(" ###", macro_str1.str);
    return macro_str1.str_0;
}
/* peek or read [ws_str == NULL] next token from function macro call,
walking up macro levels up to the file if necessary */
unsafe extern "C" fn next_argstream(
    mut nested_list: *mut *mut Sym,
    mut ws_str: *mut TokenString,
) -> std::os::raw::c_int {
    let mut t: std::os::raw::c_int = 0;
    let mut p: *const std::os::raw::c_int = 0 as *const std::os::raw::c_int;
    let mut sa: *mut Sym = 0 as *mut Sym;
    loop {
        if !macro_ptr.is_null() {
            p = macro_ptr;
            t = *p;
            if !ws_str.is_null() {
                while is_space(t) != 0
                    || 10 as std::os::raw::c_int == t
                    || 0xa4 as std::os::raw::c_int == t
                {
                    tok_str_add(ws_str, t);
                    p = p.offset(1);
                    t = *p
                }
            }
            if t == 0 as std::os::raw::c_int {
                end_macro();
                /* also, end of scope for nested defined symbol */
                sa = *nested_list;
                while !sa.is_null() && (*sa).v == 0 as std::os::raw::c_int {
                    sa = (*sa).prev
                }
                if !sa.is_null() {
                    (*sa).v = 0 as std::os::raw::c_int
                }
                continue;
            }
        } else {
            ch = handle_eob();
            if !ws_str.is_null() {
                while is_space(ch) != 0 || ch == '\n' as i32 || ch == '/' as i32 {
                    if ch == '/' as i32 {
                        let mut c: std::os::raw::c_int = 0;
                        let mut p_0: *mut uint8_t = (*file).buf_ptr;
                        p_0 = p_0.offset(1);
                        c = *p_0 as std::os::raw::c_int;
                        if c == '\\' as i32 {
                            c = handle_stray1(p_0);
                            p_0 = (*file).buf_ptr
                        }
                        if c == '*' as i32 {
                            p_0 = parse_comment(p_0);
                            (*file).buf_ptr = p_0.offset(-(1 as std::os::raw::c_int as isize))
                        } else {
                            if !(c == '/' as i32) {
                                break;
                            }
                            p_0 = parse_line_comment(p_0);
                            (*file).buf_ptr = p_0.offset(-(1 as std::os::raw::c_int as isize))
                        }
                        ch = ' ' as i32
                    }
                    if ch == '\n' as i32 {
                        (*file).line_num += 1
                    }
                    if !(ch == '\u{c}' as i32 || ch == '\u{b}' as i32 || ch == '\r' as i32) {
                        tok_str_add(ws_str, ch);
                    }
                    minp();
                }
            }
            t = ch
        }
        if !ws_str.is_null() {
            return t;
        }
        next_nomacro();
        return tok;
    }
}
/* do macro substitution of current token with macro 's' and add
result to (tok_str,tok_len). 'nested_list' is the list of all
macros we got inside to avoid recursing. Return non zero if no
substitution needs to be done */
unsafe extern "C" fn macro_subst_tok(
    mut tok_str: *mut TokenString,
    mut nested_list: *mut *mut Sym,
    mut s: *mut Sym,
) -> std::os::raw::c_int {
    let mut args: *mut Sym = 0 as *mut Sym;
    let mut sa: *mut Sym = 0 as *mut Sym;
    let mut sa1: *mut Sym = 0 as *mut Sym;
    let mut parlevel: std::os::raw::c_int = 0;
    let mut t: std::os::raw::c_int = 0;
    let mut t1: std::os::raw::c_int = 0;
    let mut spc: std::os::raw::c_int = 0;
    let mut str: TokenString = TokenString {
        str_0: 0 as *const std::os::raw::c_int as *mut std::os::raw::c_int,
        len: 0,
        lastlen: 0,
        allocated_len: 0,
        last_line_num: 0,
        save_line_num: 0,
        prev: 0 as *const TokenString as *mut TokenString,
        prev_ptr: 0 as *const std::os::raw::c_int,
        alloc: 0,
    };
    let mut cstrval: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut cval: CValue = CValue {
        ld: f128::f128::ZERO,
    };
    let mut cstr: CString = CString {
        size: 0,
        data: 0 as *const std::os::raw::c_void as *mut std::os::raw::c_void,
        size_allocated: 0,
    };
    let mut buf: [std::os::raw::c_char; 32] = [0; 32];
    let mut ti: time_t = 0;
    let mut tm: *mut tm = 0 as *mut tm;
    let mut current_block_100: u64;
    /* if symbol is a macro, prepare substitution */
    /* special macros */
    if tok == TOK___LINE__ as std::os::raw::c_int || tok == TOK___COUNTER__ as std::os::raw::c_int {
        t = if tok == TOK___LINE__ as std::os::raw::c_int {
            (*file).line_num
        } else {
            let fresh72 = pp_counter;
            pp_counter = pp_counter + 1;
            fresh72
        };
        snprintf(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[std::os::raw::c_char; 32]>() as std::os::raw::c_ulong,
            b"%d\x00" as *const u8 as *const std::os::raw::c_char,
            t,
        );
        cstrval = buf.as_mut_ptr();
        t1 = 0xcd as std::os::raw::c_int;
        current_block_100 = 15189240252204177498;
    } else {
        if tok == TOK___FILE__ as std::os::raw::c_int {
            cstrval = (*file).filename.as_mut_ptr();
            current_block_100 = 667138768817726457;
        } else if tok == TOK___DATE__ as std::os::raw::c_int
            || tok == TOK___TIME__ as std::os::raw::c_int
        {
            ti = 0;
            tm = 0 as *mut tm;
            time(&mut ti);
            tm = localtime(&mut ti);
            if tok == TOK___DATE__ as std::os::raw::c_int {
                snprintf(
                    buf.as_mut_ptr(),
                    ::std::mem::size_of::<[std::os::raw::c_char; 32]>() as std::os::raw::c_ulong,
                    b"%s %2d %d\x00" as *const u8 as *const std::os::raw::c_char,
                    ab_month_name[(*tm).tm_mon as usize].as_ptr(),
                    (*tm).tm_mday,
                    (*tm).tm_year + 1900 as std::os::raw::c_int,
                );
            } else {
                snprintf(
                    buf.as_mut_ptr(),
                    ::std::mem::size_of::<[std::os::raw::c_char; 32]>() as std::os::raw::c_ulong,
                    b"%02d:%02d:%02d\x00" as *const u8 as *const std::os::raw::c_char,
                    (*tm).tm_hour,
                    (*tm).tm_min,
                    (*tm).tm_sec,
                );
            }
            cstrval = buf.as_mut_ptr();
            current_block_100 = 667138768817726457;
        } else {
            if !(*s).c2rust_unnamed.d.is_null() {
                let mut saved_parse_flags: std::os::raw::c_int = parse_flags;
                let mut joined_str: *mut std::os::raw::c_int = 0 as *mut std::os::raw::c_int;
                let mut mstr: *mut std::os::raw::c_int = (*s).c2rust_unnamed.d;
                if (*s).type_0.t == 1 as std::os::raw::c_int {
                    /* whitespace between macro name and argument list */
                    let mut ws_str: TokenString = TokenString {
                        str_0: 0 as *const std::os::raw::c_int as *mut std::os::raw::c_int,
                        len: 0,
                        lastlen: 0,
                        allocated_len: 0,
                        last_line_num: 0,
                        save_line_num: 0,
                        prev: 0 as *const TokenString as *mut TokenString,
                        prev_ptr: 0 as *const std::os::raw::c_int,
                        alloc: 0,
                    };
                    tok_str_new(&mut ws_str);
                    spc = 0 as std::os::raw::c_int;
                    parse_flags |= 0x10 as std::os::raw::c_int
                        | 0x4 as std::os::raw::c_int
                        | 0x20 as std::os::raw::c_int;
                    /* get next token from argument stream */
                    t = next_argstream(nested_list, &mut ws_str);
                    if t != '(' as i32 {
                        /* not a macro substitution after all, restore the
                         * macro token plus all whitespace we've read.
                         * whitespace is intentionally not merged to preserve
                         * newlines. */
                        parse_flags = saved_parse_flags;
                        tok_str_add(tok_str, tok);
                        if parse_flags & 0x10 as std::os::raw::c_int != 0 {
                            let mut i: std::os::raw::c_int = 0;
                            i = 0 as std::os::raw::c_int;
                            while i < ws_str.len {
                                tok_str_add(tok_str, *ws_str.str_0.offset(i as isize));
                                i += 1
                            }
                        }
                        tok_str_free_str(ws_str.str_0);
                        return 0 as std::os::raw::c_int;
                    } else {
                        tok_str_free_str(ws_str.str_0);
                    }
                    loop {
                        next_nomacro();
                        if !(tok == 0xa4 as std::os::raw::c_int || is_space(tok) != 0) {
                            break;
                        }
                        /* eat '(' */
                    }
                    /* argument macro */
                    args = 0 as *mut Sym;
                    sa = (*s).c2rust_unnamed_0.next;
                    's_261: loop
                    /* NOTE: empty args are allowed, except if no args */
                    {
                        loop {
                            next_argstream(nested_list, 0 as *mut TokenString);
                            if !(is_space(tok) != 0 || 10 as std::os::raw::c_int == tok) {
                                break;
                            }
                        }
                        loop
                        /* handle '()' case */
                        {
                            if args.is_null() && sa.is_null() && tok == ')' as i32 {
                                break 's_261;
                            }
                            if sa.is_null() {
                                _tcc_error(
                                    b"macro \'%s\' used with too many args\x00" as *const u8
                                        as *const std::os::raw::c_char,
                                    get_tok_str((*s).v, 0 as *mut CValue),
                                );
                            }
                            tok_str_new(&mut str);
                            spc = 0 as std::os::raw::c_int;
                            parlevel = spc;
                            /* NOTE: non zero sa->t indicates VA_ARGS */
                            while parlevel > 0 as std::os::raw::c_int
                                || tok != ')' as i32 && (tok != ',' as i32 || (*sa).type_0.t != 0)
                            {
                                if tok == -(1 as std::os::raw::c_int)
                                    || tok == 0 as std::os::raw::c_int
                                {
                                    break;
                                }
                                if tok == '(' as i32 {
                                    parlevel += 1
                                } else if tok == ')' as i32 {
                                    parlevel -= 1
                                }
                                if tok == 10 as std::os::raw::c_int {
                                    tok = ' ' as i32
                                }
                                if check_space(tok, &mut spc) == 0 {
                                    tok_str_add2(&mut str, tok, &mut tokc);
                                }
                                next_argstream(nested_list, 0 as *mut TokenString);
                            }
                            if parlevel != 0 {
                                expect(b")\x00" as *const u8 as *const std::os::raw::c_char);
                            }
                            str.len -= spc;
                            tok_str_add(&mut str, -(1 as std::os::raw::c_int));
                            tok_str_add(&mut str, 0 as std::os::raw::c_int);
                            sa1 = sym_push2(
                                &mut args,
                                (*sa).v & !(0x20000000 as std::os::raw::c_int),
                                (*sa).type_0.t,
                                0 as std::os::raw::c_int,
                            );
                            (*sa1).c2rust_unnamed.d = str.str_0;
                            sa = (*sa).c2rust_unnamed_0.next;
                            if !(tok == ')' as i32) {
                                break;
                            }
                            /* special case for gcc var args: add an empty
                            var arg argument if it is omitted */
                            if !(!sa.is_null()
                                && (*sa).type_0.t != 0
                                && (*tcc_state).gnu_ext as std::os::raw::c_int != 0)
                            {
                                break 's_261;
                            }
                        }
                        if tok != ',' as i32 {
                            expect(b",\x00" as *const u8 as *const std::os::raw::c_char);
                        }
                    }
                    if !sa.is_null() {
                        _tcc_error(
                            b"macro \'%s\' used with too few args\x00" as *const u8
                                as *const std::os::raw::c_char,
                            get_tok_str((*s).v, 0 as *mut CValue),
                        );
                    }
                    /* now subst each arg */
                    mstr = macro_arg_subst(nested_list, mstr, args);
                    /* free memory */
                    sa = args;
                    while !sa.is_null() {
                        sa1 = (*sa).prev;
                        tok_str_free_str((*sa).c2rust_unnamed.d);
                        if !(*sa).c2rust_unnamed_0.next.is_null() {
                            tok_str_free_str((*(*sa).c2rust_unnamed_0.next).c2rust_unnamed.d);
                            sym_free((*sa).c2rust_unnamed_0.next);
                        }
                        sym_free(sa);
                        sa = sa1
                    }
                    parse_flags = saved_parse_flags
                }
                sym_push2(
                    nested_list,
                    (*s).v,
                    0 as std::os::raw::c_int,
                    0 as std::os::raw::c_int,
                );
                parse_flags = saved_parse_flags;
                joined_str = macro_twosharps(mstr);
                macro_subst(
                    tok_str,
                    nested_list,
                    if !joined_str.is_null() {
                        joined_str
                    } else {
                        mstr
                    },
                );
                /* pop nested defined symbol */
                sa1 = *nested_list;
                *nested_list = (*sa1).prev;
                sym_free(sa1);
                if !joined_str.is_null() {
                    tok_str_free_str(joined_str);
                }
                if mstr != (*s).c2rust_unnamed.d {
                    tok_str_free_str(mstr);
                }
            }
            current_block_100 = 12065775993741208975;
        }
        match current_block_100 {
            12065775993741208975 => {},
            _ => {
                t1 = 0xc8 as std::os::raw::c_int;
                current_block_100 = 15189240252204177498;
            },
        }
    }
    match current_block_100 {
        15189240252204177498 => {
            cstr_new(&mut cstr);
            cstr_cat(&mut cstr, cstrval, 0 as std::os::raw::c_int);
            cval.str_0.size = cstr.size;
            cval.str_0.data = cstr.data;
            tok_str_add2(tok_str, t1, &mut cval);
            cstr_free(&mut cstr);
        },
        _ => {},
    }
    return 0 as std::os::raw::c_int;
}
/* do macro substitution of macro_str and add result to
(tok_str,tok_len). 'nested_list' is the list of all macros we got
inside to avoid recursing. */
unsafe extern "C" fn macro_subst(
    mut tok_str: *mut TokenString,
    mut nested_list: *mut *mut Sym,
    mut macro_str: *const std::os::raw::c_int,
) {
    let mut current_block: u64;
    let mut s: *mut Sym = 0 as *mut Sym;
    let mut t: std::os::raw::c_int = 0;
    let mut spc: std::os::raw::c_int = 0;
    let mut nosubst: std::os::raw::c_int = 0;
    let mut cval: CValue = CValue {
        ld: f128::f128::ZERO,
    };
    nosubst = 0 as std::os::raw::c_int;
    spc = nosubst;
    loop {
        let mut _t: std::os::raw::c_int = *macro_str;
        if _t >= 0xc0 as std::os::raw::c_int && _t <= 0xcf as std::os::raw::c_int {
            tok_get(&mut t, &mut macro_str, &mut cval);
        } else {
            t = _t;
            macro_str = macro_str.offset(1)
        }
        if t <= 0 as std::os::raw::c_int {
            break;
        }
        if t >= 256 as std::os::raw::c_int && 0 as std::os::raw::c_int == nosubst {
            s = define_find(t);
            if s.is_null() {
                current_block = 13905390232648648507;
            } else if !sym_find2(*nested_list, t).is_null() {
                /* if nested substitution, do nothing */
                /* and mark it as TOK_NOSUBST, so it doesn't get subst'd again */
                tok_str_add2(tok_str, 0xa5 as std::os::raw::c_int, 0 as *mut CValue);
                current_block = 13905390232648648507;
            } else {
                let mut str: *mut TokenString = tok_str_alloc();
                (*str).str_0 = macro_str as *mut std::os::raw::c_int;
                begin_macro(str, 2 as std::os::raw::c_int);
                tok = t;
                macro_subst_tok(tok_str, nested_list, s);
                if macro_stack != str {
                    break;
                }
                macro_str = macro_ptr;
                end_macro();
                if (*tok_str).len != 0 {
                    t = *(*tok_str).str_0.offset((*tok_str).lastlen as isize);
                    spc = is_space(t)
                }
                current_block = 9853141518545631134;
            }
        } else {
            current_block = 13905390232648648507;
        }
        match current_block {
            13905390232648648507 => {
                if check_space(t, &mut spc) == 0 {
                    tok_str_add2(tok_str, t, &mut cval);
                }
                if nosubst != 0 {
                    if nosubst > 1 as std::os::raw::c_int
                        && (spc != 0 || {
                            nosubst += 1;
                            (nosubst == 3 as std::os::raw::c_int) && t == '(' as i32
                        })
                    {
                        continue;
                    }
                    nosubst = 0 as std::os::raw::c_int
                }
                if t == 0xa5 as std::os::raw::c_int {
                    nosubst = 1 as std::os::raw::c_int
                }
            },
            _ => {},
        }
        /* GCC supports 'defined' as result of a macro substitution */
        if t == TOK_DEFINED as std::os::raw::c_int && pp_expr != 0 {
            nosubst = 2 as std::os::raw::c_int
        }
    }
}
/* return next token without macro substitution. Can read input from
macro_ptr buffer */
unsafe extern "C" fn next_nomacro() {
    let mut current_block: u64;
    let mut t: std::os::raw::c_int = 0;
    if !macro_ptr.is_null() {
        loop {
            t = *macro_ptr;
            if t >= 0xc0 as std::os::raw::c_int && t <= 0xcf as std::os::raw::c_int {
                tok_get(&mut tok, &mut macro_ptr, &mut tokc);
                if !(t == 0xcf as std::os::raw::c_int) {
                    current_block = 7976072742316086414;
                    break;
                }
                (*file).line_num = tokc.i as std::os::raw::c_int
            } else {
                macro_ptr = macro_ptr.offset(1);
                if !(t < 256 as std::os::raw::c_int) {
                    current_block = 17965632435239708295;
                    break;
                }
                if !(parse_flags & 0x10 as std::os::raw::c_int == 0
                    && isidnum_table[(t - -(1 as std::os::raw::c_int)) as usize]
                        as std::os::raw::c_int
                        & 1 as std::os::raw::c_int
                        != 0)
                {
                    current_block = 17965632435239708295;
                    break;
                }
            }
        }
        match current_block {
            7976072742316086414 => {},
            _ => tok = t,
        }
    } else {
        next_nomacro1();
    };
}
/* return next token with macro substitution */
#[no_mangle]
pub unsafe extern "C" fn next() {
    let mut t: std::os::raw::c_int = 0;
    loop
    /* discard preprocessor markers */
    {
        next_nomacro();
        t = tok;
        if !macro_ptr.is_null() {
            if t >= 0xc0 as std::os::raw::c_int && t <= 0xcf as std::os::raw::c_int {
                break;
            }
            if t == 0xa5 as std::os::raw::c_int || t == 0xa4 as std::os::raw::c_int {
                continue;
            }
            if t == 0 as std::os::raw::c_int {
                /* end of macro or unget token string */
                end_macro();
            } else {
                if t == '\\' as i32 {
                    if parse_flags & 0x20 as std::os::raw::c_int == 0 {
                        _tcc_error(
                            b"stray \'\\\' in program\x00" as *const u8
                                as *const std::os::raw::c_char,
                        );
                    }
                }
                return;
            }
        } else {
            if !(t >= 256 as std::os::raw::c_int && parse_flags & 0x1 as std::os::raw::c_int != 0) {
                break;
            }
            /* if reading from file, try to substitute macros */
            let mut s: *mut Sym = define_find(t);
            if !s.is_null() {
                let mut nested_list: *mut Sym = 0 as *mut Sym;
                tokstr_buf.len = 0 as std::os::raw::c_int;
                macro_subst_tok(&mut tokstr_buf, &mut nested_list, s);
                tok_str_add(&mut tokstr_buf, 0 as std::os::raw::c_int);
                begin_macro(&mut tokstr_buf, 0 as std::os::raw::c_int);
            } else {
                return;
            }
        }
    }
    /* convert preprocessor tokens into C tokens */
    if t == 0xcd as std::os::raw::c_int {
        if parse_flags & 0x2 as std::os::raw::c_int != 0 {
            parse_number(tokc.str_0.data as *mut std::os::raw::c_char);
        }
    } else if t == 0xce as std::os::raw::c_int {
        if parse_flags & 0x40 as std::os::raw::c_int != 0 {
            parse_string(
                tokc.str_0.data as *mut std::os::raw::c_char,
                tokc.str_0.size - 1 as std::os::raw::c_int,
            );
        }
    };
}
/* push back current token and set current token to 'last_tok'. Only
identifier case handled for labels. */
#[no_mangle]
pub unsafe extern "C" fn unget_tok(mut last_tok: std::os::raw::c_int) {
    let mut str: *mut TokenString = tok_str_alloc();
    tok_str_add2(str, tok, &mut tokc);
    tok_str_add(str, 0 as std::os::raw::c_int);
    begin_macro(str, 1 as std::os::raw::c_int);
    tok = last_tok;
}
/* ------------------------------------------------------------------------- */
/* init preprocessor */
static mut target_os_defs: *const std::os::raw::c_char =
    b"__linux__\x00__linux\x00__unix__\x00__unix\x00\x00" as *const u8
        as *const std::os::raw::c_char;
unsafe extern "C" fn putdef(mut cs: *mut CString, mut p: *const std::os::raw::c_char) {
    cstr_printf(
        cs,
        b"#define %s%s\n\x00" as *const u8 as *const std::os::raw::c_char,
        p,
        &*(b" 1\x00" as *const u8 as *const std::os::raw::c_char).offset(
            (!(strchr
                as unsafe extern "C" fn(
                    _: *const std::os::raw::c_char,
                    _: std::os::raw::c_int,
                ) -> *mut std::os::raw::c_char)(p, ' ' as i32)
            .is_null() as std::os::raw::c_int
                * 2 as std::os::raw::c_int) as isize,
        ) as *const std::os::raw::c_char,
    );
}
unsafe extern "C" fn tcc_predefs(
    mut s1: *mut TCCState,
    mut cs: *mut CString,
    mut is_asm: std::os::raw::c_int,
) {
    let mut a: std::os::raw::c_int = 0;
    let mut b: std::os::raw::c_int = 0;
    let mut c: std::os::raw::c_int = 0;
    let mut defs: [*const std::os::raw::c_char; 3] = [
        target_machine_defs,
        target_os_defs,
        0 as *const std::os::raw::c_char,
    ];
    let mut p: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    sscanf(
        b"0.9.27\x00" as *const u8 as *const std::os::raw::c_char,
        b"%d.%d.%d\x00" as *const u8 as *const std::os::raw::c_char,
        &mut a as *mut std::os::raw::c_int,
        &mut b as *mut std::os::raw::c_int,
        &mut c as *mut std::os::raw::c_int,
    );
    cstr_printf(
        cs,
        b"#define __TINYC__ %d\n\x00" as *const u8 as *const std::os::raw::c_char,
        a * 10000 as std::os::raw::c_int + b * 100 as std::os::raw::c_int + c,
    );
    a = 0 as std::os::raw::c_int;
    while !defs[a as usize].is_null() {
        p = defs[a as usize];
        while *p != 0 {
            putdef(cs, p);
            p = strchr(p, 0 as std::os::raw::c_int).offset(1 as std::os::raw::c_int as isize)
        }
        a += 1
    }
    if is_asm != 0 {
        putdef(
            cs,
            b"__ASSEMBLER__\x00" as *const u8 as *const std::os::raw::c_char,
        );
    }
    if (*s1).output_type == 5 as std::os::raw::c_int {
        putdef(
            cs,
            b"__TCC_PP__\x00" as *const u8 as *const std::os::raw::c_char,
        );
    }
    if (*s1).output_type == 1 as std::os::raw::c_int {
        putdef(
            cs,
            b"__TCC_RUN__\x00" as *const u8 as *const std::os::raw::c_char,
        );
    }
    if (*s1).char_is_unsigned != 0 {
        putdef(
            cs,
            b"__CHAR_UNSIGNED__\x00" as *const u8 as *const std::os::raw::c_char,
        );
    }
    if (*s1).optimize as std::os::raw::c_int > 0 as std::os::raw::c_int {
        putdef(
            cs,
            b"__OPTIMIZE__\x00" as *const u8 as *const std::os::raw::c_char,
        );
    }
    if (*s1).option_pthread != 0 {
        putdef(
            cs,
            b"_REENTRANT\x00" as *const u8 as *const std::os::raw::c_char,
        );
    }
    if (*s1).leading_underscore != 0 {
        putdef(
            cs,
            b"__leading_underscore\x00" as *const u8 as *const std::os::raw::c_char,
        );
    }
    if (*s1).do_bounds_check != 0 {
        putdef(
            cs,
            b"__BOUNDS_CHECKING_ON\x00" as *const u8 as *const std::os::raw::c_char,
        );
    }
    cstr_printf(
        cs,
        b"#define __SIZEOF_POINTER__ %d\n\x00" as *const u8 as *const std::os::raw::c_char,
        8 as std::os::raw::c_int,
    );
    cstr_printf(
        cs,
        b"#define __SIZEOF_LONG__ %d\n\x00" as *const u8 as *const std::os::raw::c_char,
        8 as std::os::raw::c_int,
    );
    if is_asm == 0 {
        putdef(
            cs,
            b"__STDC__\x00" as *const u8 as *const std::os::raw::c_char,
        );
        cstr_printf(
            cs,
            b"#define __STDC_VERSION__ %dL\n\x00" as *const u8 as *const std::os::raw::c_char,
            (*s1).cversion,
        );
        cstr_cat(cs,
                 b"#define __SIZE_TYPE__ unsigned long\n#define __PTRDIFF_TYPE__ long\n#define __LP64__ 1\n#define __INT64_TYPE__ long\n#define __SIZEOF_INT__ 4\n#define __INT_MAX__ 0x7fffffff\n#define __LONG_MAX__ 0x7fffffffffffffffL\n#define __SIZEOF_LONG_LONG__ 8\n#define __LONG_LONG_MAX__ 0x7fffffffffffffffLL\n#define __CHAR_BIT__ 8\n#define __ORDER_LITTLE_ENDIAN__ 1234\n#define __ORDER_BIG_ENDIAN__ 4321\n#define __BYTE_ORDER__ __ORDER_LITTLE_ENDIAN__\n#define __WCHAR_TYPE__ int\n#define __WINT_TYPE__ unsigned int\n#if __STDC_VERSION__==201112L\n#define __STDC_NO_ATOMICS__ 1\n#define __STDC_NO_COMPLEX__ 1\n#define __STDC_NO_THREADS__ 1\n#define __STDC_UTF_16__ 1\n#define __STDC_UTF_32__ 1\n#endif\n#define __ATOMIC_RELAXED 0\n#define __ATOMIC_CONSUME 1\n#define __ATOMIC_ACQUIRE 2\n#define __ATOMIC_RELEASE 3\n#define __ATOMIC_ACQ_REL 4\n#define __ATOMIC_SEQ_CST 5\n#define __REDIRECT(name,proto,alias) name proto __asm__(#alias)\n#define __REDIRECT_NTH(name,proto,alias) name proto __asm__(#alias)__THROW\n#ifndef __TCC_PP__\n#define __builtin_offsetof(type,field) ((__SIZE_TYPE__)&((type*)0)->field)\n#define __builtin_extract_return_addr(x) x\ntypedef struct{\nunsigned gp_offset,fp_offset;\nunion{\nunsigned overflow_offset;\nchar*overflow_arg_area;\n};\nchar*reg_save_area;\n}__builtin_va_list[1];\nvoid*__va_arg(__builtin_va_list ap,int arg_type,int size,int align);\n#define __builtin_va_start(ap,last) (*(ap)=*(__builtin_va_list)((char*)__builtin_frame_address(0)-24))\n#define __builtin_va_arg(ap,t) (*(t*)(__va_arg(ap,__builtin_va_arg_types(t),sizeof(t),__alignof__(t))))\n#define __builtin_va_copy(dest,src) (*(dest)=*(src))\n#define __builtin_va_end(ap) (void)(ap)\n#ifndef __builtin_va_copy\n#define __builtin_va_copy(dest,src) (dest)=(src)\n#endif\n#ifdef __leading_underscore\n#define __RENAME(X) __asm__(\"_\"X)\n#else\n#define __RENAME(X) __asm__(X)\n#endif\n#ifdef __BOUNDS_CHECKING_ON\n#define __BUILTINBC(ret,name,params) ret __builtin_##name params __RENAME(\"__bound_\"#name);\n#define __BOUND(ret,name,params) ret name params __RENAME(\"__bound_\"#name);\n#else\n#define __BUILTINBC(ret,name,params) ret __builtin_##name params __RENAME(#name);\n#define __BOUND(ret,name,params)\n#endif\n#define __BOTH(ret,name,params) __BUILTINBC(ret,name,params)__BOUND(ret,name,params)\n#define __BUILTIN(ret,name,params) ret __builtin_##name params __RENAME(#name);\n__BOTH(void*,memcpy,(void*,const void*,__SIZE_TYPE__))\n__BOTH(void*,memmove,(void*,const void*,__SIZE_TYPE__))\n__BOTH(void*,memset,(void*,int,__SIZE_TYPE__))\n__BOTH(int,memcmp,(const void*,const void*,__SIZE_TYPE__))\n__BOTH(__SIZE_TYPE__,strlen,(const char*))\n__BOTH(char*,strcpy,(char*,const char*))\n__BOTH(char*,strncpy,(char*,const char*,__SIZE_TYPE__))\n__BOTH(int,strcmp,(const char*,const char*))\n__BOTH(int,strncmp,(const char*,const char*,__SIZE_TYPE__))\n__BOTH(char*,strcat,(char*,const char*))\n__BOTH(char*,strchr,(const char*,int))\n__BOTH(char*,strdup,(const char*))\n#define __MAYBE_REDIR __BUILTIN\n__MAYBE_REDIR(void*,malloc,(__SIZE_TYPE__))\n__MAYBE_REDIR(void*,realloc,(void*,__SIZE_TYPE__))\n__MAYBE_REDIR(void*,calloc,(__SIZE_TYPE__,__SIZE_TYPE__))\n__MAYBE_REDIR(void*,memalign,(__SIZE_TYPE__,__SIZE_TYPE__))\n__MAYBE_REDIR(void,free,(void*))\n__BOTH(void*,alloca,(__SIZE_TYPE__))\n__BUILTIN(void,abort,(void))\n__BOUND(void,longjmp,())\n__BOUND(void*,mmap,())\n__BOUND(int,munmap,())\n#undef __BUILTINBC\n#undef __BUILTIN\n#undef __BOUND\n#undef __BOTH\n#undef __MAYBE_REDIR\n#undef __RENAME\n#endif\n\x00"
                     as *const u8 as *const std::os::raw::c_char,
                 -(1 as std::os::raw::c_int));
    }
    cstr_printf(
        cs,
        b"#define __BASE_FILE__ \"%s\"\n\x00" as *const u8 as *const std::os::raw::c_char,
        (*file).filename.as_mut_ptr(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn preprocess_start(
    mut s1: *mut TCCState,
    mut filetype: std::os::raw::c_int,
) {
    let mut is_asm: std::os::raw::c_int = (filetype
        & (2 as std::os::raw::c_int | 4 as std::os::raw::c_int)
        != 0) as std::os::raw::c_int;
    let mut cstr: CString = CString {
        size: 0,
        data: 0 as *const std::os::raw::c_void as *mut std::os::raw::c_void,
        size_allocated: 0,
    };
    tccpp_new(s1);
    (*s1).include_stack_ptr = (*s1).include_stack.as_mut_ptr();
    (*s1).ifdef_stack_ptr = (*s1).ifdef_stack.as_mut_ptr();
    (*file).ifdef_stack_ptr = (*s1).ifdef_stack_ptr;
    pp_expr = 0 as std::os::raw::c_int;
    pp_counter = 0 as std::os::raw::c_int;
    pp_debug_symv = 0 as std::os::raw::c_int;
    pp_debug_tok = pp_debug_symv;
    pp_once += 1;
    (*s1).pack_stack[0 as std::os::raw::c_int as usize] = 0 as std::os::raw::c_int;
    (*s1).pack_stack_ptr = (*s1).pack_stack.as_mut_ptr();
    set_idnum(
        '$' as i32,
        if is_asm == 0 && (*s1).dollars_in_identifiers as std::os::raw::c_int != 0 {
            2 as std::os::raw::c_int
        } else {
            0 as std::os::raw::c_int
        },
    );
    set_idnum(
        '.' as i32,
        if is_asm != 0 {
            2 as std::os::raw::c_int
        } else {
            0 as std::os::raw::c_int
        },
    );
    if filetype & 2 as std::os::raw::c_int == 0 {
        cstr_new(&mut cstr);
        tcc_predefs(s1, &mut cstr, is_asm);
        if (*s1).cmdline_defs.size != 0 {
            cstr_cat(
                &mut cstr,
                (*s1).cmdline_defs.data as *const std::os::raw::c_char,
                (*s1).cmdline_defs.size,
            );
        }
        if (*s1).cmdline_incl.size != 0 {
            cstr_cat(
                &mut cstr,
                (*s1).cmdline_incl.data as *const std::os::raw::c_char,
                (*s1).cmdline_incl.size,
            );
        }
        //printf("%s\n", (char*)cstr.data);
        let fresh73 = (*s1).include_stack_ptr;
        (*s1).include_stack_ptr = (*s1).include_stack_ptr.offset(1);
        *fresh73 = file;
        tcc_open_bf(
            s1,
            b"<command line>\x00" as *const u8 as *const std::os::raw::c_char,
            cstr.size,
        );
        memcpy(
            (*file).buffer.as_mut_ptr() as *mut std::os::raw::c_void,
            cstr.data,
            cstr.size as std::os::raw::c_ulong,
        );
        cstr_free(&mut cstr);
    }
    parse_flags = if is_asm != 0 {
        0x8 as std::os::raw::c_int
    } else {
        0 as std::os::raw::c_int
    };
    tok_flags = 0x1 as std::os::raw::c_int | 0x2 as std::os::raw::c_int;
}
/* cleanup from error/setjmp */
#[no_mangle]
pub unsafe extern "C" fn preprocess_end(mut s1: *mut TCCState) {
    while !macro_stack.is_null() {
        end_macro();
    }
    macro_ptr = 0 as *const std::os::raw::c_int;
    while !file.is_null() {
        tcc_close();
    }
    tccpp_delete(s1);
}
#[no_mangle]
pub unsafe extern "C" fn tccpp_new(mut s: *mut TCCState) {
    let mut i: std::os::raw::c_int = 0;
    let mut c: std::os::raw::c_int = 0;
    let mut p: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut r: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    /* init isid table */
    i = -(1 as std::os::raw::c_int);
    while i < 128 as std::os::raw::c_int {
        set_idnum(
            i,
            if is_space(i) != 0 {
                1 as std::os::raw::c_int
            } else if isid(i) != 0 {
                2 as std::os::raw::c_int
            } else if isnum(i) != 0 {
                4 as std::os::raw::c_int
            } else {
                0 as std::os::raw::c_int
            },
        );
        i += 1
    }
    i = 128 as std::os::raw::c_int;
    while i < 256 as std::os::raw::c_int {
        set_idnum(i, 2 as std::os::raw::c_int);
        i += 1
    }
    /* init allocators */
    tal_new(
        &mut toksym_alloc,
        256 as std::os::raw::c_int as std::os::raw::c_uint,
        (768 as std::os::raw::c_int * 1024 as std::os::raw::c_int) as std::os::raw::c_uint,
    );
    tal_new(
        &mut tokstr_alloc,
        128 as std::os::raw::c_int as std::os::raw::c_uint,
        (768 as std::os::raw::c_int * 1024 as std::os::raw::c_int) as std::os::raw::c_uint,
    );
    memset(
        hash_ident.as_mut_ptr() as *mut std::os::raw::c_void,
        0 as std::os::raw::c_int,
        (16384 as std::os::raw::c_int as std::os::raw::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut TokenSym>() as std::os::raw::c_ulong),
    );
    memset(
        (*s).cached_includes_hash.as_mut_ptr() as *mut std::os::raw::c_void,
        0 as std::os::raw::c_int,
        ::std::mem::size_of::<[std::os::raw::c_int; 32]>() as std::os::raw::c_ulong,
    );
    cstr_new(&mut cstr_buf);
    cstr_realloc(&mut cstr_buf, 1024 as std::os::raw::c_int);
    tok_str_new(&mut tokstr_buf);
    tok_str_realloc(&mut tokstr_buf, 256 as std::os::raw::c_int);
    tok_ident = 256 as std::os::raw::c_int;
    p = tcc_keywords.as_ptr();
    while *p != 0 {
        r = p;
        loop {
            let fresh74 = r;
            r = r.offset(1);
            c = *fresh74 as std::os::raw::c_int;
            if c == '\u{0}' as i32 {
                break;
            }
        }
        tok_alloc(
            p,
            (r.offset_from(p) as std::os::raw::c_long
                - 1 as std::os::raw::c_int as std::os::raw::c_long)
                as std::os::raw::c_int,
        );
        p = r
    }
    /* we add dummy defines for some special macros to speed up tests
    and to have working defined() */
    define_push(
        TOK___LINE__ as std::os::raw::c_int,
        0 as std::os::raw::c_int,
        0 as *mut std::os::raw::c_int,
        0 as *mut Sym,
    );
    define_push(
        TOK___FILE__ as std::os::raw::c_int,
        0 as std::os::raw::c_int,
        0 as *mut std::os::raw::c_int,
        0 as *mut Sym,
    );
    define_push(
        TOK___DATE__ as std::os::raw::c_int,
        0 as std::os::raw::c_int,
        0 as *mut std::os::raw::c_int,
        0 as *mut Sym,
    );
    define_push(
        TOK___TIME__ as std::os::raw::c_int,
        0 as std::os::raw::c_int,
        0 as *mut std::os::raw::c_int,
        0 as *mut Sym,
    );
    define_push(
        TOK___COUNTER__ as std::os::raw::c_int,
        0 as std::os::raw::c_int,
        0 as *mut std::os::raw::c_int,
        0 as *mut Sym,
    );
}
#[no_mangle]
pub unsafe extern "C" fn tccpp_delete(mut s: *mut TCCState) {
    let mut i: std::os::raw::c_int = 0;
    let mut n: std::os::raw::c_int = 0;
    dynarray_reset(
        &mut (*s).cached_includes as *mut *mut *mut CachedInclude as *mut std::os::raw::c_void,
        &mut (*s).nb_cached_includes,
    );
    /* free tokens */
    n = tok_ident - 256 as std::os::raw::c_int;
    if n > (*tcc_state).total_idents {
        (*tcc_state).total_idents = n
    }
    i = 0 as std::os::raw::c_int;
    while i < n {
        tal_free_impl(
            toksym_alloc,
            *table_ident.offset(i as isize) as *mut std::os::raw::c_void,
        );
        i += 1
    }
    tcc_free(table_ident as *mut std::os::raw::c_void);
    table_ident = 0 as *mut *mut TokenSym;
    /* free static buffers */
    cstr_free(&mut tokcstr);
    cstr_free(&mut cstr_buf);
    cstr_free(&mut macro_equal_buf);
    tok_str_free_str(tokstr_buf.str_0);
    /* free allocators */
    tal_delete(toksym_alloc);
    toksym_alloc = 0 as *mut TinyAlloc;
    tal_delete(tokstr_alloc);
    tokstr_alloc = 0 as *mut TinyAlloc;
}
/* ------------------------------------------------------------------------- */
/* tcc -E [-P[1]] [-dD} support */
unsafe extern "C" fn tok_print(
    mut msg: *const std::os::raw::c_char,
    mut str: *const std::os::raw::c_int,
) {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut t: std::os::raw::c_int = 0;
    let mut s: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut cval: CValue = CValue {
        ld: f128::f128::ZERO,
    };
    fp = (*tcc_state).ppfp;
    fprintf(
        fp,
        b"%s\x00" as *const u8 as *const std::os::raw::c_char,
        msg,
    );
    while !str.is_null() {
        let mut _t: std::os::raw::c_int = *str;
        if _t >= 0xc0 as std::os::raw::c_int && _t <= 0xcf as std::os::raw::c_int {
            tok_get(&mut t, &mut str, &mut cval);
        } else {
            t = _t;
            str = str.offset(1)
        }
        if t == 0 {
            break;
        }
        fprintf(
            fp,
            &*(b" %s\x00" as *const u8 as *const std::os::raw::c_char).offset(s as isize)
                as *const std::os::raw::c_char,
            get_tok_str(t, &mut cval),
        );
        s = 1 as std::os::raw::c_int
    }
    fprintf(fp, b"\n\x00" as *const u8 as *const std::os::raw::c_char);
}
unsafe extern "C" fn pp_line(
    mut s1: *mut TCCState,
    mut f: *mut BufferedFile,
    mut level: std::os::raw::c_int,
) {
    let mut d: std::os::raw::c_int = (*f).line_num - (*f).line_ref;
    if (*s1).dflag as std::os::raw::c_int & 4 as std::os::raw::c_int != 0 {
        return;
    }
    if !((*s1).Pflag as std::os::raw::c_uint
        == LINE_MACRO_OUTPUT_FORMAT_NONE as std::os::raw::c_int as std::os::raw::c_uint)
    {
        if level == 0 as std::os::raw::c_int && (*f).line_ref != 0 && d < 8 as std::os::raw::c_int {
            while d > 0 as std::os::raw::c_int {
                fputs(
                    b"\n\x00" as *const u8 as *const std::os::raw::c_char,
                    (*s1).ppfp,
                );
                d -= 1
            }
        } else if (*s1).Pflag as std::os::raw::c_uint
            == LINE_MACRO_OUTPUT_FORMAT_STD as std::os::raw::c_int as std::os::raw::c_uint
        {
            fprintf(
                (*s1).ppfp,
                b"#line %d \"%s\"\n\x00" as *const u8 as *const std::os::raw::c_char,
                (*f).line_num,
                (*f).filename.as_mut_ptr(),
            );
        } else {
            fprintf(
                (*s1).ppfp,
                b"# %d \"%s\"%s\n\x00" as *const u8 as *const std::os::raw::c_char,
                (*f).line_num,
                (*f).filename.as_mut_ptr(),
                if level > 0 as std::os::raw::c_int {
                    b" 1\x00" as *const u8 as *const std::os::raw::c_char
                } else if level < 0 as std::os::raw::c_int {
                    b" 2\x00" as *const u8 as *const std::os::raw::c_char
                } else {
                    b"\x00" as *const u8 as *const std::os::raw::c_char
                },
            );
        }
    }
    (*f).line_ref = (*f).line_num;
}
unsafe extern "C" fn define_print(mut s1: *mut TCCState, mut v: std::os::raw::c_int) {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut s: *mut Sym = 0 as *mut Sym;
    s = define_find(v);
    if s.is_null() || (*s).c2rust_unnamed.d.is_null() {
        return;
    }
    fp = (*s1).ppfp;
    fprintf(
        fp,
        b"#define %s\x00" as *const u8 as *const std::os::raw::c_char,
        get_tok_str(v, 0 as *mut CValue),
    );
    if (*s).type_0.t == 1 as std::os::raw::c_int {
        let mut a: *mut Sym = (*s).c2rust_unnamed_0.next;
        fprintf(fp, b"(\x00" as *const u8 as *const std::os::raw::c_char);
        if !a.is_null() {
            loop {
                fprintf(
                    fp,
                    b"%s\x00" as *const u8 as *const std::os::raw::c_char,
                    get_tok_str(
                        (*a).v & !(0x20000000 as std::os::raw::c_int),
                        0 as *mut CValue,
                    ),
                );
                a = (*a).c2rust_unnamed_0.next;
                if a.is_null() {
                    break;
                }
                fprintf(fp, b",\x00" as *const u8 as *const std::os::raw::c_char);
            }
        }
        fprintf(fp, b")\x00" as *const u8 as *const std::os::raw::c_char);
    }
    tok_print(
        b"\x00" as *const u8 as *const std::os::raw::c_char,
        (*s).c2rust_unnamed.d,
    );
}
unsafe extern "C" fn pp_debug_defines(mut s1: *mut TCCState) {
    let mut v: std::os::raw::c_int = 0;
    let mut t: std::os::raw::c_int = 0;
    let mut vs: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut fp: *mut FILE = 0 as *mut FILE;
    t = pp_debug_tok;
    if t == 0 as std::os::raw::c_int {
        return;
    }
    (*file).line_num -= 1;
    pp_line(s1, file, 0 as std::os::raw::c_int);
    (*file).line_num += 1;
    (*file).line_ref = (*file).line_num;
    fp = (*s1).ppfp;
    v = pp_debug_symv;
    vs = get_tok_str(v, 0 as *mut CValue);
    if t == TOK_DEFINE as std::os::raw::c_int {
        define_print(s1, v);
    } else if t == TOK_UNDEF as std::os::raw::c_int {
        fprintf(
            fp,
            b"#undef %s\n\x00" as *const u8 as *const std::os::raw::c_char,
            vs,
        );
    } else if t == TOK_push_macro as std::os::raw::c_int {
        fprintf(
            fp,
            b"#pragma push_macro(\"%s\")\n\x00" as *const u8 as *const std::os::raw::c_char,
            vs,
        );
    } else if t == TOK_pop_macro as std::os::raw::c_int {
        fprintf(
            fp,
            b"#pragma pop_macro(\"%s\")\n\x00" as *const u8 as *const std::os::raw::c_char,
            vs,
        );
    }
    pp_debug_tok = 0 as std::os::raw::c_int;
}
unsafe extern "C" fn pp_debug_builtins(mut s1: *mut TCCState) {
    let mut v: std::os::raw::c_int = 0;
    v = 256 as std::os::raw::c_int;
    while v < tok_ident {
        define_print(s1, v);
        v += 1
    }
}
/* Add a space between tokens a and b to avoid unwanted textual pasting */
unsafe extern "C" fn pp_need_space(
    mut a: std::os::raw::c_int,
    mut b: std::os::raw::c_int,
) -> std::os::raw::c_int {
    return if 'E' as i32 == a {
        ('+' as i32 == b || '-' as i32 == b) as std::os::raw::c_int
    } else if '+' as i32 == a {
        (0x82 as std::os::raw::c_int == b || '+' as i32 == b) as std::os::raw::c_int
    } else if '-' as i32 == a {
        (0x80 as std::os::raw::c_int == b || '-' as i32 == b) as std::os::raw::c_int
    } else if a >= 256 as std::os::raw::c_int {
        (b >= 256 as std::os::raw::c_int) as std::os::raw::c_int
    } else if a == 0xcd as std::os::raw::c_int {
        (b >= 256 as std::os::raw::c_int) as std::os::raw::c_int
    } else {
        0 as std::os::raw::c_int
    };
}
/* maybe hex like 0x1e */
unsafe extern "C" fn pp_check_he0xE(
    mut t: std::os::raw::c_int,
    mut p: *const std::os::raw::c_char,
) -> std::os::raw::c_int {
    if t == 0xcd as std::os::raw::c_int
        && toup(
            *strchr(p, 0 as std::os::raw::c_int).offset(-(1 as std::os::raw::c_int) as isize)
                as std::os::raw::c_int,
        ) == 'E' as i32
    {
        return 'E' as i32;
    }
    return t;
}
/* Preprocess the current file */
#[no_mangle]
pub unsafe extern "C" fn tcc_preprocess(mut s1: *mut TCCState) -> std::os::raw::c_int {
    let mut iptr: *mut *mut BufferedFile = 0 as *mut *mut BufferedFile;
    let mut token_seen: std::os::raw::c_int = 0;
    let mut spcs: std::os::raw::c_int = 0;
    let mut level: std::os::raw::c_int = 0;
    let mut p: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut white: [std::os::raw::c_char; 400] = [0; 400];
    parse_flags = 0x1 as std::os::raw::c_int
        | parse_flags & 0x8 as std::os::raw::c_int
        | 0x4 as std::os::raw::c_int
        | 0x10 as std::os::raw::c_int
        | 0x20 as std::os::raw::c_int;
    /* Credits to Fabrice Bellard's initial revision to demonstrate its
    capability to compile and run itself, provided all numbers are
    given as decimals. tcc -E -P10 will do. */
    if (*s1).Pflag as std::os::raw::c_uint
        == LINE_MACRO_OUTPUT_FORMAT_P10 as std::os::raw::c_int as std::os::raw::c_uint
    {
        parse_flags |= 0x2 as std::os::raw::c_int;
        (*s1).Pflag = LINE_MACRO_OUTPUT_FORMAT_NONE
    }
    if (*s1).do_bench != 0 {
        loop
        /* for PP benchmarks */
        {
            next();
            if !(tok != -(1 as std::os::raw::c_int)) {
                break;
            }
        }
        return 0 as std::os::raw::c_int;
    }
    if (*s1).dflag as std::os::raw::c_int & 1 as std::os::raw::c_int != 0 {
        pp_debug_builtins(s1);
        (*s1).dflag = ((*s1).dflag as std::os::raw::c_int & !(1 as std::os::raw::c_int))
            as std::os::raw::c_char
    }
    token_seen = 10 as std::os::raw::c_int;
    spcs = 0 as std::os::raw::c_int;
    level = 0 as std::os::raw::c_int;
    if !(*file).prev.is_null() {
        let fresh75 = level;
        level = level + 1;
        pp_line(s1, (*file).prev, fresh75);
    }
    pp_line(s1, file, level);
    loop {
        iptr = (*s1).include_stack_ptr;
        next();
        if tok == -(1 as std::os::raw::c_int) {
            break;
        }
        level = (*s1).include_stack_ptr.offset_from(iptr) as std::os::raw::c_long
            as std::os::raw::c_int;
        if level != 0 {
            if level > 0 as std::os::raw::c_int {
                pp_line(s1, *iptr, 0 as std::os::raw::c_int);
            }
            pp_line(s1, file, level);
        }
        if (*s1).dflag as std::os::raw::c_int & 7 as std::os::raw::c_int != 0 {
            pp_debug_defines(s1);
            if (*s1).dflag as std::os::raw::c_int & 4 as std::os::raw::c_int != 0 {
                continue;
            }
        }
        if is_space(tok) != 0 {
            if (spcs as std::os::raw::c_ulong)
                < (::std::mem::size_of::<[std::os::raw::c_char; 400]>() as std::os::raw::c_ulong)
                    .wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong)
            {
                let fresh76 = spcs;
                spcs = spcs + 1;
                white[fresh76 as usize] = tok as std::os::raw::c_char
            }
        } else {
            if tok == 10 as std::os::raw::c_int {
                spcs = 0 as std::os::raw::c_int;
                if token_seen == 10 as std::os::raw::c_int {
                    continue;
                }
                (*file).line_ref += 1
            } else if token_seen == 10 as std::os::raw::c_int {
                pp_line(s1, file, 0 as std::os::raw::c_int);
            } else if spcs == 0 as std::os::raw::c_int && pp_need_space(token_seen, tok) != 0 {
                let fresh77 = spcs;
                spcs = spcs + 1;
                white[fresh77 as usize] = ' ' as i32 as std::os::raw::c_char
            }
            white[spcs as usize] = 0 as std::os::raw::c_int as std::os::raw::c_char;
            fputs(white.as_mut_ptr(), (*s1).ppfp);
            spcs = 0 as std::os::raw::c_int;
            p = get_tok_str(tok, &mut tokc);
            fputs(p, (*s1).ppfp);
            token_seen = pp_check_he0xE(tok, p)
        }
    }
    return 0 as std::os::raw::c_int;
}
/* ------------------------------------------------------------------------- */
