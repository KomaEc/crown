use crate::bitfields::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type sym_version;
    #[no_mangle]
    fn strrchr(_: *const std::os::raw::c_char, _: std::os::raw::c_int)
    -> *mut std::os::raw::c_char;
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
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    fn fflush(__stream: *mut FILE) -> std::os::raw::c_int;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    static mut stdout: *mut FILE;
    #[no_mangle]
    fn strtoull(
        _: *const std::os::raw::c_char,
        _: *mut *mut std::os::raw::c_char,
        _: std::os::raw::c_int,
    ) -> std::os::raw::c_ulonglong;
    #[no_mangle]
    fn strtoul(
        _: *const std::os::raw::c_char,
        _: *mut *mut std::os::raw::c_char,
        _: std::os::raw::c_int,
    ) -> std::os::raw::c_ulong;
    #[no_mangle]
    fn strtol(
        _: *const std::os::raw::c_char,
        _: *mut *mut std::os::raw::c_char,
        _: std::os::raw::c_int,
    ) -> std::os::raw::c_long;
    #[no_mangle]
    fn open(
        __file: *const std::os::raw::c_char,
        __oflag: std::os::raw::c_int,
        _: ...
    ) -> std::os::raw::c_int;
    #[no_mangle]
    fn _setjmp(_: *mut __jmp_buf_tag) -> std::os::raw::c_int;
    #[no_mangle]
    fn longjmp(_: *mut __jmp_buf_tag, _: std::os::raw::c_int) -> !;
    #[no_mangle]
    fn lseek(
        __fd: std::os::raw::c_int,
        __offset: __off_t,
        __whence: std::os::raw::c_int,
    ) -> __off_t;
    #[no_mangle]
    fn close(__fd: std::os::raw::c_int) -> std::os::raw::c_int;
    #[no_mangle]
    fn read(
        __fd: std::os::raw::c_int,
        __buf: *mut std::os::raw::c_void,
        __nbytes: size_t,
    ) -> ssize_t;
    #[no_mangle]
    fn dlopen(
        __file: *const std::os::raw::c_char,
        __mode: std::os::raw::c_int,
    ) -> *mut std::os::raw::c_void;
    #[no_mangle]
    static mut tok_flags: std::os::raw::c_int;
    #[no_mangle]
    static mut file: *mut BufferedFile;
    #[no_mangle]
    fn tccelf_new(s: *mut TCCState);
    #[no_mangle]
    fn tcc_run_free(s1: *mut TCCState);
    #[no_mangle]
    fn cstr_free(cstr: *mut CString);
    #[no_mangle]
    fn tccelf_delete(s: *mut TCCState);
    #[no_mangle]
    fn cstr_cat(cstr: *mut CString, str: *const std::os::raw::c_char, len: std::os::raw::c_int);
    #[no_mangle]
    fn cstr_ccat(cstr: *mut CString, ch: std::os::raw::c_int);
    #[no_mangle]
    fn cstr_printf(
        cs: *mut CString,
        fmt: *const std::os::raw::c_char,
        _: ...
    ) -> std::os::raw::c_int;
    #[no_mangle]
    fn cstr_new(cstr: *mut CString);
    #[no_mangle]
    fn tccelf_end_file(s1: *mut TCCState);
    #[no_mangle]
    fn preprocess_end(s1: *mut TCCState);
    #[no_mangle]
    fn tccgen_finish(s1: *mut TCCState);
    #[no_mangle]
    fn tccgen_compile(s1: *mut TCCState) -> std::os::raw::c_int;
    #[no_mangle]
    fn tcc_assemble(s1: *mut TCCState, do_preprocess: std::os::raw::c_int) -> std::os::raw::c_int;
    #[no_mangle]
    fn tcc_preprocess(s1: *mut TCCState) -> std::os::raw::c_int;
    #[no_mangle]
    fn tccgen_init(s1: *mut TCCState);
    #[no_mangle]
    fn preprocess_start(s1: *mut TCCState, filetype: std::os::raw::c_int);
    #[no_mangle]
    fn tccelf_begin_file(s1: *mut TCCState);
    #[no_mangle]
    fn tcc_load_ldscript(s1: *mut TCCState, fd: std::os::raw::c_int) -> std::os::raw::c_int;
    #[no_mangle]
    fn tcc_load_dll(
        s1: *mut TCCState,
        fd: std::os::raw::c_int,
        filename: *const std::os::raw::c_char,
        level: std::os::raw::c_int,
    ) -> std::os::raw::c_int;
    #[no_mangle]
    fn tcc_load_archive(
        s1: *mut TCCState,
        fd: std::os::raw::c_int,
        alacarte: std::os::raw::c_int,
    ) -> std::os::raw::c_int;
    #[no_mangle]
    fn tcc_load_object_file(
        s1: *mut TCCState,
        fd: std::os::raw::c_int,
        file_offset: std::os::raw::c_ulong,
    ) -> std::os::raw::c_int;
    #[no_mangle]
    fn tcc_object_type(fd: std::os::raw::c_int, h: *mut Elf64_Ehdr) -> std::os::raw::c_int;
    #[no_mangle]
    fn tccelf_stab_new(s: *mut TCCState);
    #[no_mangle]
    fn tccelf_bounds_new(s: *mut TCCState);
    #[no_mangle]
    fn set_global_sym(
        s1: *mut TCCState,
        name: *const std::os::raw::c_char,
        sec: *mut Section,
        offs: Elf64_Addr,
    ) -> std::os::raw::c_int;
    #[no_mangle]
    fn realloc(_: *mut std::os::raw::c_void, _: std::os::raw::c_ulong)
    -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn exit(_: std::os::raw::c_int) -> !;
    #[no_mangle]
    fn free(__ptr: *mut std::os::raw::c_void);
    #[no_mangle]
    fn malloc(_: std::os::raw::c_ulong) -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn sem_init(
        __sem: *mut sem_t,
        __pshared: std::os::raw::c_int,
        __value: std::os::raw::c_uint,
    ) -> std::os::raw::c_int;
    #[no_mangle]
    fn sem_wait(__sem: *mut sem_t) -> std::os::raw::c_int;
    #[no_mangle]
    fn sem_post(__sem: *mut sem_t) -> std::os::raw::c_int;
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
pub type TCCErrorFunc = Option<
    unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const std::os::raw::c_char) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub union sem_t {
    pub __size: [std::os::raw::c_char; 32],
    pub __align: std::os::raw::c_long,
}
pub const TCC_OPTION_C: C2RustUnnamed_4 = 52;
pub const TCC_OPTION_s: C2RustUnnamed_4 = 24;
pub const TCC_OPTION_pipe: C2RustUnnamed_4 = 45;
pub const TCC_OPTION_pedantic: C2RustUnnamed_4 = 41;
pub const TCC_OPTION_traditional: C2RustUnnamed_4 = 25;
pub const TCC_OPTION_ar: C2RustUnnamed_4 = 50;
pub const TCC_OPTION_impdef: C2RustUnnamed_4 = 51;
pub const TCC_OPTION_print_search_dirs: C2RustUnnamed_4 = 38;
pub const TCC_OPTION_O: C2RustUnnamed_4 = 29;
pub const TCC_OPTION_x: C2RustUnnamed_4 = 49;
pub const TCC_OPTION_dumpversion: C2RustUnnamed_4 = 16;
pub const TCC_OPTION_MF: C2RustUnnamed_4 = 48;
pub const TCC_OPTION_MD: C2RustUnnamed_4 = 47;
pub const TCC_OPTION_P: C2RustUnnamed_4 = 6;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TCCOption {
    pub name: *const std::os::raw::c_char,
    pub index: uint16_t,
    pub flags: uint16_t,
}
pub type uint16_t = __uint16_t;
pub const TCC_OPTION_E: C2RustUnnamed_4 = 46;
pub const TCC_OPTION_Wp: C2RustUnnamed_4 = 27;
pub const TCC_OPTION_Wl: C2RustUnnamed_4 = 26;
pub const TCC_OPTION_rdynamic: C2RustUnnamed_4 = 39;
pub const TCC_OPTION_w: C2RustUnnamed_4 = 44;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FlagDef {
    pub offset: uint16_t,
    pub flags: uint16_t,
    pub name: *const std::os::raw::c_char,
}
pub const TCC_OPTION_W: C2RustUnnamed_4 = 28;
pub const TCC_OPTION_m: C2RustUnnamed_4 = 31;
pub const TCC_OPTION_f: C2RustUnnamed_4 = 32;
pub const TCC_OPTION_v: C2RustUnnamed_4 = 2;
pub const TCC_OPTION_run: C2RustUnnamed_4 = 43;
pub const TCC_OPTION_nostdlib: C2RustUnnamed_4 = 37;
pub const TCC_OPTION_nostdinc: C2RustUnnamed_4 = 36;
pub const TCC_OPTION_include: C2RustUnnamed_4 = 35;
pub const TCC_OPTION_isystem: C2RustUnnamed_4 = 33;
pub const TCC_OPTION_r: C2RustUnnamed_4 = 23;
pub const TCC_OPTION_o: C2RustUnnamed_4 = 22;
pub const TCC_OPTION_soname: C2RustUnnamed_4 = 21;
pub const TCC_OPTION_shared: C2RustUnnamed_4 = 20;
pub const TCC_OPTION_std: C2RustUnnamed_4 = 19;
pub const TCC_OPTION_static: C2RustUnnamed_4 = 18;
pub const TCC_OPTION_d: C2RustUnnamed_4 = 17;
pub const TCC_OPTION_c: C2RustUnnamed_4 = 15;
pub const TCC_OPTION_g: C2RustUnnamed_4 = 14;
pub const TCC_OPTION_b: C2RustUnnamed_4 = 12;
pub const TCC_OPTION_bt: C2RustUnnamed_4 = 11;
pub const TCC_OPTION_bench: C2RustUnnamed_4 = 10;
pub const TCC_OPTION_pthread: C2RustUnnamed_4 = 42;
pub const TCC_OPTION_l: C2RustUnnamed_4 = 9;
pub const TCC_OPTION_B: C2RustUnnamed_4 = 8;
pub const TCC_OPTION_L: C2RustUnnamed_4 = 7;
pub const TCC_OPTION_U: C2RustUnnamed_4 = 5;
pub const TCC_OPTION_D: C2RustUnnamed_4 = 4;
pub const TCC_OPTION_I: C2RustUnnamed_4 = 3;
pub const TCC_OPTION_HELP2: C2RustUnnamed_4 = 1;
pub const TCC_OPTION_HELP: C2RustUnnamed_4 = 0;
pub const TCC_OPTION_param: C2RustUnnamed_4 = 40;
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
pub type Elf64_Word = uint32_t;
pub type uint32_t = __uint32_t;
pub type Elf64_Off = uint64_t;
pub type uintptr_t = std::os::raw::c_ulong;
pub type C2RustUnnamed_4 = std::os::raw::c_uint;
pub const TCC_OPTION_iwithprefix: C2RustUnnamed_4 = 34;
pub const TCC_OPTION_mfloat_abi: C2RustUnnamed_4 = 30;
pub const TCC_OPTION_ba: C2RustUnnamed_4 = 13;
#[inline]
unsafe extern "C" fn isnum(mut c: std::os::raw::c_int) -> std::os::raw::c_int {
    return (c >= '0' as i32 && c <= '9' as i32) as std::os::raw::c_int;
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const std::os::raw::c_char) -> std::os::raw::c_int {
    return strtol(
        __nptr,
        0 as *mut std::os::raw::c_void as *mut *mut std::os::raw::c_char,
        10 as std::os::raw::c_int,
    ) as std::os::raw::c_int;
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
/* ONE_SOURCE */
/* *******************************************************/
/* global variables */
/* XXX: get rid of this ASAP (or maybe not) */
#[no_mangle]
pub static mut tcc_state: *mut TCCState = 0 as *const TCCState as *mut TCCState;
/* *******************************************************/
/* *******************************************************/
static mut tcc_sem_init: std::os::raw::c_int = 0;
static mut tcc_sem: sem_t = sem_t { __size: [0; 32] };
unsafe extern "C" fn wait_sem() {
    if tcc_sem_init == 0 {
        sem_init(
            &mut tcc_sem,
            0 as std::os::raw::c_int,
            1 as std::os::raw::c_int as std::os::raw::c_uint,
        );
        tcc_sem_init = 1 as std::os::raw::c_int
    }
    while sem_wait(&mut tcc_sem) < 0 as std::os::raw::c_int
        && *__errno_location() == 4 as std::os::raw::c_int
    {}
}
/* *******************************************************/
/* copy a string and truncate it. */
#[no_mangle]
pub unsafe extern "C" fn pstrcpy(
    mut buf: *mut std::os::raw::c_char,
    mut buf_size: size_t,
    mut s: *const std::os::raw::c_char,
) -> *mut std::os::raw::c_char {
    let mut q: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut q_end: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut c: std::os::raw::c_int = 0;
    if buf_size > 0 as std::os::raw::c_int as std::os::raw::c_ulong {
        q = buf;
        q_end = buf
            .offset(buf_size as isize)
            .offset(-(1 as std::os::raw::c_int as isize));
        while q < q_end {
            let fresh0 = s;
            s = s.offset(1);
            c = *fresh0 as std::os::raw::c_int;
            if c == '\u{0}' as i32 {
                break;
            }
            let fresh1 = q;
            q = q.offset(1);
            *fresh1 = c as std::os::raw::c_char
        }
        *q = '\u{0}' as i32 as std::os::raw::c_char
    }
    return buf;
}
/* strcat and truncate. */
#[no_mangle]
pub unsafe extern "C" fn pstrcat(
    mut buf: *mut std::os::raw::c_char,
    mut buf_size: size_t,
    mut s: *const std::os::raw::c_char,
) -> *mut std::os::raw::c_char {
    let mut len: size_t = 0;
    len = strlen(buf);
    if len < buf_size {
        pstrcpy(buf.offset(len as isize), buf_size.wrapping_sub(len), s);
    }
    return buf;
}
#[no_mangle]
pub unsafe extern "C" fn pstrncpy(
    mut out: *mut std::os::raw::c_char,
    mut in_0: *const std::os::raw::c_char,
    mut num: size_t,
) -> *mut std::os::raw::c_char {
    memcpy(
        out as *mut std::os::raw::c_void,
        in_0 as *const std::os::raw::c_void,
        num,
    );
    *out.offset(num as isize) = '\u{0}' as i32 as std::os::raw::c_char;
    return out;
}
/* extract the basename of a file */
#[no_mangle]
pub unsafe extern "C" fn tcc_basename(
    mut name: *const std::os::raw::c_char,
) -> *mut std::os::raw::c_char {
    let mut p: *mut std::os::raw::c_char = strchr(name, 0 as std::os::raw::c_int);
    while p > name as *mut std::os::raw::c_char
        && !(*p.offset(-(1 as std::os::raw::c_int) as isize) as std::os::raw::c_int == '/' as i32)
    {
        p = p.offset(-1)
    }
    return p;
}
/* extract extension part of a file
 *
 * (if no extension, return pointer to end-of-string)
 */
#[no_mangle]
pub unsafe extern "C" fn tcc_fileextension(
    mut name: *const std::os::raw::c_char,
) -> *mut std::os::raw::c_char {
    let mut b: *mut std::os::raw::c_char = tcc_basename(name);
    let mut e: *mut std::os::raw::c_char = strrchr(b, '.' as i32);
    return if !e.is_null() {
        e
    } else {
        strchr(b, 0 as std::os::raw::c_int)
    };
}
/* *******************************************************/
/* memory management */
#[no_mangle]
pub unsafe extern "C" fn tcc_free(mut ptr: *mut std::os::raw::c_void) {
    free(ptr);
}
#[no_mangle]
pub unsafe extern "C" fn tcc_malloc(mut size: std::os::raw::c_ulong) -> *mut std::os::raw::c_void {
    let mut ptr: *mut std::os::raw::c_void = 0 as *mut std::os::raw::c_void;
    ptr = malloc(size);
    if ptr.is_null() && size != 0 {
        _tcc_error(b"memory full (malloc)\x00" as *const u8 as *const std::os::raw::c_char);
    }
    return ptr;
}
#[no_mangle]
pub unsafe extern "C" fn tcc_mallocz(mut size: std::os::raw::c_ulong) -> *mut std::os::raw::c_void {
    let mut ptr: *mut std::os::raw::c_void = 0 as *mut std::os::raw::c_void;
    ptr = tcc_malloc(size);
    if size != 0 {
        memset(ptr, 0 as std::os::raw::c_int, size);
    }
    return ptr;
}
#[no_mangle]
pub unsafe extern "C" fn tcc_realloc(
    mut ptr: *mut std::os::raw::c_void,
    mut size: std::os::raw::c_ulong,
) -> *mut std::os::raw::c_void {
    let mut ptr1: *mut std::os::raw::c_void = 0 as *mut std::os::raw::c_void;
    ptr1 = realloc(ptr, size);
    if ptr1.is_null() && size != 0 {
        _tcc_error(b"memory full (realloc)\x00" as *const u8 as *const std::os::raw::c_char);
    }
    return ptr1;
}
#[no_mangle]
pub unsafe extern "C" fn tcc_strdup(
    mut str: *const std::os::raw::c_char,
) -> *mut std::os::raw::c_char {
    let mut ptr: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    ptr = tcc_malloc(strlen(str).wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong))
        as *mut std::os::raw::c_char;
    strcpy(ptr, str);
    return ptr;
}
/* MEM_DEBUG */
/* *******************************************************/
/* dynarrays */
#[no_mangle]
pub unsafe extern "C" fn dynarray_add(
    mut ptab: *mut std::os::raw::c_void,
    mut nb_ptr: *mut std::os::raw::c_int,
    mut data: *mut std::os::raw::c_void,
) {
    let mut nb: std::os::raw::c_int = 0;
    let mut nb_alloc: std::os::raw::c_int = 0;
    let mut pp: *mut *mut std::os::raw::c_void = 0 as *mut *mut std::os::raw::c_void;
    nb = *nb_ptr;
    pp = *(ptab as *mut *mut *mut std::os::raw::c_void);
    /* every power of two we double array size */
    if nb & nb - 1 as std::os::raw::c_int == 0 as std::os::raw::c_int {
        if nb == 0 {
            nb_alloc = 1 as std::os::raw::c_int
        } else {
            nb_alloc = nb * 2 as std::os::raw::c_int
        }
        pp = tcc_realloc(
            pp as *mut std::os::raw::c_void,
            (nb_alloc as std::os::raw::c_ulong)
                .wrapping_mul(
                    ::std::mem::size_of::<*mut std::os::raw::c_void>() as std::os::raw::c_ulong
                ),
        ) as *mut *mut std::os::raw::c_void;
        let ref mut fresh2 = *(ptab as *mut *mut *mut std::os::raw::c_void);
        *fresh2 = pp
    }
    let fresh3 = nb;
    nb = nb + 1;
    let ref mut fresh4 = *pp.offset(fresh3 as isize);
    *fresh4 = data;
    *nb_ptr = nb;
}
#[no_mangle]
pub unsafe extern "C" fn dynarray_reset(
    mut pp: *mut std::os::raw::c_void,
    mut n: *mut std::os::raw::c_int,
) {
    let mut p: *mut *mut std::os::raw::c_void = 0 as *mut *mut std::os::raw::c_void;
    p = *(pp as *mut *mut *mut std::os::raw::c_void);
    while *n != 0 {
        if !(*p).is_null() {
            tcc_free(*p);
        }
        p = p.offset(1);
        *n -= 1
    }
    tcc_free(*(pp as *mut *mut std::os::raw::c_void));
    let ref mut fresh5 = *(pp as *mut *mut std::os::raw::c_void);
    *fresh5 = 0 as *mut std::os::raw::c_void;
}
unsafe extern "C" fn tcc_split_path(
    mut s: *mut TCCState,
    mut p_ary: *mut std::os::raw::c_void,
    mut p_nb_ary: *mut std::os::raw::c_int,
    mut in_0: *const std::os::raw::c_char,
) {
    let mut p: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    loop {
        let mut c: std::os::raw::c_int = 0;
        let mut str: CString = CString {
            size: 0,
            data: 0 as *mut std::os::raw::c_void,
            size_allocated: 0,
        };
        cstr_new(&mut str);
        p = in_0;
        loop {
            c = *p as std::os::raw::c_int;
            if !(c != '\u{0}' as i32
                && c != (*::std::mem::transmute::<&[u8; 2], &[std::os::raw::c_char; 2]>(b":\x00"))
                    [0 as std::os::raw::c_int as usize]
                    as std::os::raw::c_int)
            {
                break;
            }
            if c == '{' as i32
                && *p.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int != 0
                && *p.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int == '}' as i32
            {
                c = *p.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int;
                p = p.offset(2 as std::os::raw::c_int as isize);
                if c == 'B' as i32 {
                    cstr_cat(&mut str, (*s).tcc_lib_path, -(1 as std::os::raw::c_int));
                }
                if c == 'f' as i32 && !file.is_null() {
                    /* substitute current file's dir */
                    let mut f: *const std::os::raw::c_char = (*file).true_filename;
                    let mut b: *const std::os::raw::c_char = tcc_basename(f);
                    if b > f {
                        cstr_cat(
                            &mut str,
                            f,
                            (b.offset_from(f) as std::os::raw::c_long
                                - 1 as std::os::raw::c_int as std::os::raw::c_long)
                                as std::os::raw::c_int,
                        );
                    } else {
                        cstr_cat(
                            &mut str,
                            b".\x00" as *const u8 as *const std::os::raw::c_char,
                            1 as std::os::raw::c_int,
                        );
                    }
                }
            } else {
                cstr_ccat(&mut str, c);
            }
            p = p.offset(1)
        }
        if str.size != 0 {
            cstr_ccat(&mut str, '\u{0}' as i32);
            dynarray_add(
                p_ary,
                p_nb_ary,
                tcc_strdup(str.data as *const std::os::raw::c_char) as *mut std::os::raw::c_void,
            );
        }
        cstr_free(&mut str);
        in_0 = p.offset(1 as std::os::raw::c_int as isize);
        if !(*p != 0) {
            break;
        }
    }
}
/* *******************************************************/
unsafe extern "C" fn strcat_vprintf(
    mut buf: *mut std::os::raw::c_char,
    mut buf_size: std::os::raw::c_int,
    mut fmt: *const std::os::raw::c_char,
    mut ap: ::std::ffi::VaList,
) {
    let mut len: std::os::raw::c_int = 0;
    len = strlen(buf) as std::os::raw::c_int;
    vsnprintf(
        buf.offset(len as isize),
        (buf_size - len) as std::os::raw::c_ulong,
        fmt,
        ap.as_va_list(),
    );
}
unsafe extern "C" fn strcat_printf(
    mut buf: *mut std::os::raw::c_char,
    mut buf_size: std::os::raw::c_int,
    mut fmt: *const std::os::raw::c_char,
    mut args: ...
) {
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    strcat_vprintf(buf, buf_size, fmt, ap.as_va_list());
}
#[no_mangle]
pub unsafe extern "C" fn tcc_enter_state(mut s1: *mut TCCState) {
    wait_sem();
    tcc_state = s1;
}
#[no_mangle]
pub unsafe extern "C" fn tcc_exit_state() {
    tcc_state = 0 as *mut TCCState;
    sem_post(&mut tcc_sem);
}
unsafe extern "C" fn error1(
    mut mode: std::os::raw::c_int,
    mut fmt: *const std::os::raw::c_char,
    mut ap: ::std::ffi::VaList,
) {
    let mut buf: [std::os::raw::c_char; 2048] = [0; 2048];
    let mut pf: *mut *mut BufferedFile = 0 as *mut *mut BufferedFile;
    let mut f: *mut BufferedFile = 0 as *mut BufferedFile;
    let mut s1: *mut TCCState = tcc_state;
    buf[0 as std::os::raw::c_int as usize] = '\u{0}' as i32 as std::os::raw::c_char;
    if !s1.is_null() {
        if !s1.is_null() && (*s1).error_set_jmp_enabled == 0 {
            /* tcc_state just was set by tcc_enter_state() */
            tcc_exit_state();
        }
        if mode == 0 as std::os::raw::c_int {
            if (*s1).warn_none != 0 {
                return;
            }
            if (*s1).warn_error != 0 {
                mode = 2 as std::os::raw::c_int
            }
        }
        f = 0 as *mut BufferedFile;
        if (*s1).error_set_jmp_enabled != 0 {
            /* we're called while parsing a file */
            /* use upper file if inline ":asm:" or token ":paste:" */
            f = file;
            while !f.is_null()
                && (*f).filename[0 as std::os::raw::c_int as usize] as std::os::raw::c_int
                    == ':' as i32
            {
                f = (*f).prev
            }
        }
        if !f.is_null() {
            pf = (*s1).include_stack.as_mut_ptr();
            while pf < (*s1).include_stack_ptr {
                strcat_printf(
                    buf.as_mut_ptr(),
                    ::std::mem::size_of::<[std::os::raw::c_char; 2048]>() as std::os::raw::c_ulong
                        as std::os::raw::c_int,
                    b"In file included from %s:%d:\n\x00" as *const u8
                        as *const std::os::raw::c_char,
                    (**pf).filename.as_mut_ptr(),
                    (**pf).line_num,
                );
                pf = pf.offset(1)
            }
            strcat_printf(
                buf.as_mut_ptr(),
                ::std::mem::size_of::<[std::os::raw::c_char; 2048]>() as std::os::raw::c_ulong
                    as std::os::raw::c_int,
                b"%s:%d: \x00" as *const u8 as *const std::os::raw::c_char,
                (*f).filename.as_mut_ptr(),
                (*f).line_num
                    - (tok_flags & 0x1 as std::os::raw::c_int != 0) as std::os::raw::c_int,
            );
        } else if !(*s1).current_filename.is_null() {
            strcat_printf(
                buf.as_mut_ptr(),
                ::std::mem::size_of::<[std::os::raw::c_char; 2048]>() as std::os::raw::c_ulong
                    as std::os::raw::c_int,
                b"%s: \x00" as *const u8 as *const std::os::raw::c_char,
                (*s1).current_filename,
            );
        }
    }
    /* can happen only if called from tcc_malloc(): 'out of memory' */
    if 0 as std::os::raw::c_int == buf[0 as std::os::raw::c_int as usize] as std::os::raw::c_int {
        strcat_printf(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[std::os::raw::c_char; 2048]>() as std::os::raw::c_ulong
                as std::os::raw::c_int,
            b"tcc: \x00" as *const u8 as *const std::os::raw::c_char,
        );
    }
    if mode == 0 as std::os::raw::c_int {
        strcat_printf(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[std::os::raw::c_char; 2048]>() as std::os::raw::c_ulong
                as std::os::raw::c_int,
            b"warning: \x00" as *const u8 as *const std::os::raw::c_char,
        );
    } else {
        strcat_printf(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[std::os::raw::c_char; 2048]>() as std::os::raw::c_ulong
                as std::os::raw::c_int,
            b"error: \x00" as *const u8 as *const std::os::raw::c_char,
        );
    }
    strcat_vprintf(
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[std::os::raw::c_char; 2048]>() as std::os::raw::c_ulong
            as std::os::raw::c_int,
        fmt,
        ap.as_va_list(),
    );
    if s1.is_null() || (*s1).error_func.is_none() {
        /* default case: stderr */
        if !s1.is_null() && (*s1).output_type == 5 as std::os::raw::c_int && (*s1).ppfp == stdout {
            /* print a newline during tcc -E */
            printf(b"\n\x00" as *const u8 as *const std::os::raw::c_char);
            fflush(stdout);
        }
        /* print error/warning now (win32) */
        fflush(stdout); /* flush -v output */
        fprintf(
            stderr,
            b"%s\n\x00" as *const u8 as *const std::os::raw::c_char,
            buf.as_mut_ptr(),
        );
        fflush(stderr);
    } else {
        (*s1).error_func.expect("non-null function pointer")((*s1).error_opaque, buf.as_mut_ptr());
    }
    if !s1.is_null() {
        if mode != 0 as std::os::raw::c_int {
            (*s1).nb_errors += 1
        }
        if mode != 2 as std::os::raw::c_int {
            return;
        }
        if (*s1).error_set_jmp_enabled != 0 {
            longjmp((*s1).error_jmp_buf.as_mut_ptr(), 1 as std::os::raw::c_int);
        }
    }
    exit(1 as std::os::raw::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn tcc_set_error_func(
    mut s: *mut TCCState,
    mut error_opaque: *mut std::os::raw::c_void,
    mut error_func: TCCErrorFunc,
) {
    (*s).error_opaque = error_opaque;
    (*s).error_func = error_func;
}
#[no_mangle]
pub unsafe extern "C" fn tcc_get_error_func(mut s: *mut TCCState) -> TCCErrorFunc {
    return (*s).error_func;
}
#[no_mangle]
pub unsafe extern "C" fn tcc_get_error_opaque(mut s: *mut TCCState) -> *mut std::os::raw::c_void {
    return (*s).error_opaque;
}
/* error without aborting current compilation */
#[no_mangle]
pub unsafe extern "C" fn _tcc_error_noabort(mut fmt: *const std::os::raw::c_char, mut args: ...) {
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    error1(1 as std::os::raw::c_int, fmt, ap.as_va_list());
}
#[no_mangle]
pub unsafe extern "C" fn _tcc_error(mut fmt: *const std::os::raw::c_char, mut args: ...) -> ! {
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    loop {
        error1(2 as std::os::raw::c_int, fmt, ap.as_va_list());
    }
}
#[no_mangle]
pub unsafe extern "C" fn _tcc_warning(mut fmt: *const std::os::raw::c_char, mut args: ...) {
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    error1(0 as std::os::raw::c_int, fmt, ap.as_va_list());
}
/* *******************************************************/
/* I/O layer */
#[no_mangle]
pub unsafe extern "C" fn tcc_open_bf(
    mut s1: *mut TCCState,
    mut filename: *const std::os::raw::c_char,
    mut initlen: std::os::raw::c_int,
) {
    let mut bf: *mut BufferedFile = 0 as *mut BufferedFile; /* put eob symbol */
    let mut buflen: std::os::raw::c_int = if initlen != 0 {
        initlen
    } else {
        8192 as std::os::raw::c_int
    };
    bf = tcc_mallocz(
        (::std::mem::size_of::<BufferedFile>() as std::os::raw::c_ulong)
            .wrapping_add(buflen as std::os::raw::c_ulong),
    ) as *mut BufferedFile;
    (*bf).buf_ptr = (*bf).buffer.as_mut_ptr();
    (*bf).buf_end = (*bf).buffer.as_mut_ptr().offset(initlen as isize);
    *(*bf).buf_end.offset(0 as std::os::raw::c_int as isize) = '\\' as i32 as uint8_t;
    pstrcpy(
        (*bf).filename.as_mut_ptr(),
        ::std::mem::size_of::<[std::os::raw::c_char; 1024]>() as std::os::raw::c_ulong,
        filename,
    );
    (*bf).true_filename = (*bf).filename.as_mut_ptr();
    (*bf).line_num = 1 as std::os::raw::c_int;
    (*bf).ifdef_stack_ptr = (*s1).ifdef_stack_ptr;
    (*bf).fd = -(1 as std::os::raw::c_int);
    (*bf).prev = file;
    file = bf;
    tok_flags = 0x1 as std::os::raw::c_int | 0x2 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn tcc_close() {
    let mut s1: *mut TCCState = tcc_state;
    let mut bf: *mut BufferedFile = file;
    if (*bf).fd > 0 as std::os::raw::c_int {
        close((*bf).fd);
        (*s1).total_lines += (*bf).line_num
    }
    if (*bf).true_filename != (*bf).filename.as_mut_ptr() {
        tcc_free((*bf).true_filename as *mut std::os::raw::c_void);
    }
    file = (*bf).prev;
    tcc_free(bf as *mut std::os::raw::c_void);
}
unsafe extern "C" fn _tcc_open(
    mut s1: *mut TCCState,
    mut filename: *const std::os::raw::c_char,
) -> std::os::raw::c_int {
    let mut fd: std::os::raw::c_int = 0;
    if strcmp(
        filename,
        b"-\x00" as *const u8 as *const std::os::raw::c_char,
    ) == 0 as std::os::raw::c_int
    {
        fd = 0 as std::os::raw::c_int;
        filename = b"<stdin>\x00" as *const u8 as *const std::os::raw::c_char
    } else {
        fd = open(
            filename,
            0 as std::os::raw::c_int | 0 as std::os::raw::c_int,
        )
    }
    if (*s1).verbose as std::os::raw::c_int == 2 as std::os::raw::c_int
        && fd >= 0 as std::os::raw::c_int
        || (*s1).verbose as std::os::raw::c_int == 3 as std::os::raw::c_int
    {
        printf(
            b"%s %*s%s\n\x00" as *const u8 as *const std::os::raw::c_char,
            if fd < 0 as std::os::raw::c_int {
                b"nf\x00" as *const u8 as *const std::os::raw::c_char
            } else {
                b"->\x00" as *const u8 as *const std::os::raw::c_char
            },
            (*s1)
                .include_stack_ptr
                .offset_from((*s1).include_stack.as_mut_ptr()) as std::os::raw::c_long
                as std::os::raw::c_int,
            b"\x00" as *const u8 as *const std::os::raw::c_char,
            filename,
        );
    }
    return fd;
}
#[no_mangle]
pub unsafe extern "C" fn tcc_open(
    mut s1: *mut TCCState,
    mut filename: *const std::os::raw::c_char,
) -> std::os::raw::c_int {
    let mut fd: std::os::raw::c_int = _tcc_open(s1, filename);
    if fd < 0 as std::os::raw::c_int {
        return -(1 as std::os::raw::c_int);
    }
    tcc_open_bf(s1, filename, 0 as std::os::raw::c_int);
    (*file).fd = fd;
    return 0 as std::os::raw::c_int;
}
/* compile the file opened in 'file'. Return non zero if errors. */
unsafe extern "C" fn tcc_compile(
    mut s1: *mut TCCState,
    mut filetype: std::os::raw::c_int,
    mut str: *const std::os::raw::c_char,
    mut fd: std::os::raw::c_int,
) -> std::os::raw::c_int {
    /* Here we enter the code section where we use the global variables for
    parsing and code generation (tccpp.c, tccgen.c, <target>-gen.c).
    Other threads need to wait until we're done.

    Alternatively we could use thread local storage for those global
    variables, which may or may not have advantages */
    tcc_enter_state(s1);
    if _setjmp((*s1).error_jmp_buf.as_mut_ptr()) == 0 as std::os::raw::c_int {
        (*s1).error_set_jmp_enabled = 1 as std::os::raw::c_int;
        (*s1).nb_errors = 0 as std::os::raw::c_int;
        if fd == -(1 as std::os::raw::c_int) {
            let mut len: std::os::raw::c_int = strlen(str) as std::os::raw::c_int;
            tcc_open_bf(
                s1,
                b"<string>\x00" as *const u8 as *const std::os::raw::c_char,
                len,
            );
            memcpy(
                (*file).buffer.as_mut_ptr() as *mut std::os::raw::c_void,
                str as *const std::os::raw::c_void,
                len as std::os::raw::c_ulong,
            );
        } else {
            tcc_open_bf(s1, str, 0 as std::os::raw::c_int);
            (*file).fd = fd
        }
        tccelf_begin_file(s1);
        preprocess_start(s1, filetype);
        tccgen_init(s1);
        if (*s1).output_type == 5 as std::os::raw::c_int {
            tcc_preprocess(s1);
        } else if filetype & (2 as std::os::raw::c_int | 4 as std::os::raw::c_int) != 0 {
            tcc_assemble(
                s1,
                (filetype & 4 as std::os::raw::c_int != 0) as std::os::raw::c_int,
            );
        } else {
            tccgen_compile(s1);
        }
    }
    (*s1).error_set_jmp_enabled = 0 as std::os::raw::c_int;
    tccgen_finish(s1);
    preprocess_end(s1);
    tcc_exit_state();
    tccelf_end_file(s1);
    return if (*s1).nb_errors != 0 as std::os::raw::c_int {
        -(1 as std::os::raw::c_int)
    } else {
        0 as std::os::raw::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn tcc_compile_string(
    mut s: *mut TCCState,
    mut str: *const std::os::raw::c_char,
) -> std::os::raw::c_int {
    return tcc_compile(
        s,
        (*s).filetype as std::os::raw::c_int,
        str,
        -(1 as std::os::raw::c_int),
    );
}
/* define a preprocessor symbol. value can be NULL, sym can be "sym=val" */
#[no_mangle]
pub unsafe extern "C" fn tcc_define_symbol(
    mut s1: *mut TCCState,
    mut sym: *const std::os::raw::c_char,
    mut value: *const std::os::raw::c_char,
) {
    let mut eq: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    eq = strchr(sym, '=' as i32);
    if eq.is_null() {
        eq = strchr(sym, 0 as std::os::raw::c_int)
    }
    if value.is_null() {
        value = if *eq as std::os::raw::c_int != 0 {
            eq.offset(1 as std::os::raw::c_int as isize)
        } else {
            b"1\x00" as *const u8 as *const std::os::raw::c_char
        }
    }
    cstr_printf(
        &mut (*s1).cmdline_defs as *mut CString,
        b"#define %.*s %s\n\x00" as *const u8 as *const std::os::raw::c_char,
        eq.offset_from(sym) as std::os::raw::c_long as std::os::raw::c_int,
        sym,
        value,
    );
}
/* undefine a preprocessor symbol */
#[no_mangle]
pub unsafe extern "C" fn tcc_undefine_symbol(
    mut s1: *mut TCCState,
    mut sym: *const std::os::raw::c_char,
) {
    cstr_printf(
        &mut (*s1).cmdline_defs as *mut CString,
        b"#undef %s\n\x00" as *const u8 as *const std::os::raw::c_char,
        sym,
    ); /*on by default like in gcc/clang*/
}
#[no_mangle]
pub unsafe extern "C" fn tcc_new() -> *mut TCCState {
    let mut s: *mut TCCState = 0 as *mut TCCState; /* default unless -std=c11 is supplied */
    s = tcc_mallocz(::std::mem::size_of::<TCCState>() as std::os::raw::c_ulong) as *mut TCCState;
    if s.is_null() {
        return 0 as *mut TCCState;
    }
    (*s).gnu_ext = 1 as std::os::raw::c_int as std::os::raw::c_uchar;
    (*s).tcc_ext = 1 as std::os::raw::c_int as std::os::raw::c_uchar;
    (*s).nocommon = 1 as std::os::raw::c_int as std::os::raw::c_uchar;
    (*s).dollars_in_identifiers = 1 as std::os::raw::c_int as std::os::raw::c_uchar;
    (*s).cversion = 199901 as std::os::raw::c_int as std::os::raw::c_uint;
    (*s).warn_implicit_function_declaration = 1 as std::os::raw::c_int as std::os::raw::c_uchar;
    (*s).ms_extensions = 1 as std::os::raw::c_int as std::os::raw::c_uchar;
    /* enable this if you want symbols with leading underscore on windows: */
    /* || defined TCC_TARGET_PE */
    (*s).ppfp = stdout;
    /* might be used in error() before preprocess_start() */
    (*s).include_stack_ptr = (*s).include_stack.as_mut_ptr();
    tccelf_new(s);
    tcc_set_lib_path(
        s,
        b"/usr/local/lib/tcc\x00" as *const u8 as *const std::os::raw::c_char,
    );
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn tcc_delete(mut s1: *mut TCCState) {
    /* free sections */
    tccelf_delete(s1);
    /* free library paths */
    dynarray_reset(
        &mut (*s1).library_paths as *mut *mut *mut std::os::raw::c_char
            as *mut std::os::raw::c_void,
        &mut (*s1).nb_library_paths,
    );
    dynarray_reset(
        &mut (*s1).crt_paths as *mut *mut *mut std::os::raw::c_char as *mut std::os::raw::c_void,
        &mut (*s1).nb_crt_paths,
    );
    /* free include paths */
    dynarray_reset(
        &mut (*s1).include_paths as *mut *mut *mut std::os::raw::c_char
            as *mut std::os::raw::c_void,
        &mut (*s1).nb_include_paths,
    );
    dynarray_reset(
        &mut (*s1).sysinclude_paths as *mut *mut *mut std::os::raw::c_char
            as *mut std::os::raw::c_void,
        &mut (*s1).nb_sysinclude_paths,
    );
    tcc_free((*s1).tcc_lib_path as *mut std::os::raw::c_void);
    tcc_free((*s1).soname as *mut std::os::raw::c_void);
    tcc_free((*s1).rpath as *mut std::os::raw::c_void);
    tcc_free((*s1).init_symbol as *mut std::os::raw::c_void);
    tcc_free((*s1).fini_symbol as *mut std::os::raw::c_void);
    tcc_free((*s1).outfile as *mut std::os::raw::c_void);
    tcc_free((*s1).deps_outfile as *mut std::os::raw::c_void);
    dynarray_reset(
        &mut (*s1).files as *mut *mut *mut filespec as *mut std::os::raw::c_void,
        &mut (*s1).nb_files,
    );
    dynarray_reset(
        &mut (*s1).target_deps as *mut *mut *mut std::os::raw::c_char as *mut std::os::raw::c_void,
        &mut (*s1).nb_target_deps,
    );
    dynarray_reset(
        &mut (*s1).pragma_libs as *mut *mut *mut std::os::raw::c_char as *mut std::os::raw::c_void,
        &mut (*s1).nb_pragma_libs,
    );
    dynarray_reset(
        &mut (*s1).argv as *mut *mut *mut std::os::raw::c_char as *mut std::os::raw::c_void,
        &mut (*s1).argc,
    );
    cstr_free(&mut (*s1).cmdline_defs);
    cstr_free(&mut (*s1).cmdline_incl);
    /* free runtime memory */
    tcc_run_free(s1);
    tcc_free(s1 as *mut std::os::raw::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn tcc_set_output_type(
    mut s: *mut TCCState,
    mut output_type: std::os::raw::c_int,
) -> std::os::raw::c_int {
    (*s).output_type = output_type;
    /* always elf for objects */
    if output_type == 4 as std::os::raw::c_int {
        (*s).output_format = 0 as std::os::raw::c_int
    }
    if (*s).nostdinc == 0 {
        /* default include paths */
        /* -isystem paths have already been handled */
        tcc_add_sysinclude_path(
            s,
            b"{B}/include:/usr/local/include:/usr/include\x00" as *const u8
                as *const std::os::raw::c_char,
        );
    }
    if (*s).do_bounds_check != 0 {
        /* if bound checking, then add corresponding sections */
        tccelf_bounds_new(s);
    }
    if (*s).do_debug != 0 {
        /* add debug sections */
        tccelf_stab_new(s);
    }
    tcc_add_library_path(
        s,
        b"/usr/lib64:/lib64:/usr/local/lib64\x00" as *const u8 as *const std::os::raw::c_char,
    );
    /* paths for crt objects */
    tcc_split_path(
        s,
        &mut (*s).crt_paths as *mut *mut *mut std::os::raw::c_char as *mut std::os::raw::c_void,
        &mut (*s).nb_crt_paths,
        b"/usr/lib64\x00" as *const u8 as *const std::os::raw::c_char,
    );
    /* add libc crt1/crti objects */
    if (output_type == 2 as std::os::raw::c_int || output_type == 3 as std::os::raw::c_int)
        && (*s).nostdlib == 0
    {
        if output_type != 3 as std::os::raw::c_int {
            tcc_add_crt(s, b"crt1.o\x00" as *const u8 as *const std::os::raw::c_char);
        }
        tcc_add_crt(s, b"crti.o\x00" as *const u8 as *const std::os::raw::c_char);
    }
    return 0 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn tcc_add_include_path(
    mut s: *mut TCCState,
    mut pathname: *const std::os::raw::c_char,
) -> std::os::raw::c_int {
    tcc_split_path(
        s,
        &mut (*s).include_paths as *mut *mut *mut std::os::raw::c_char as *mut std::os::raw::c_void,
        &mut (*s).nb_include_paths,
        pathname,
    );
    return 0 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn tcc_add_sysinclude_path(
    mut s: *mut TCCState,
    mut pathname: *const std::os::raw::c_char,
) -> std::os::raw::c_int {
    tcc_split_path(
        s,
        &mut (*s).sysinclude_paths as *mut *mut *mut std::os::raw::c_char
            as *mut std::os::raw::c_void,
        &mut (*s).nb_sysinclude_paths,
        pathname,
    );
    return 0 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn tcc_add_dllref(
    mut s1: *mut TCCState,
    mut dllname: *const std::os::raw::c_char,
) -> *mut DLLReference {
    let mut ref_0: *mut DLLReference = tcc_mallocz(
        (::std::mem::size_of::<DLLReference>() as std::os::raw::c_ulong)
            .wrapping_add(strlen(dllname)),
    ) as *mut DLLReference;
    strcpy((*ref_0).name.as_mut_ptr(), dllname);
    dynarray_add(
        &mut (*s1).loaded_dlls as *mut *mut *mut DLLReference as *mut std::os::raw::c_void,
        &mut (*s1).nb_loaded_dlls,
        ref_0 as *mut std::os::raw::c_void,
    );
    return ref_0;
}
/* OpenBSD: choose latest from libxxx.so.x.y versions */
#[no_mangle]
pub unsafe extern "C" fn tcc_add_file_internal(
    mut s1: *mut TCCState,
    mut filename: *const std::os::raw::c_char,
    mut flags: std::os::raw::c_int,
) -> std::os::raw::c_int {
    let mut fd: std::os::raw::c_int = 0;
    let mut ret: std::os::raw::c_int = -(1 as std::os::raw::c_int);
    /* open the file */
    fd = _tcc_open(s1, filename);
    if fd < 0 as std::os::raw::c_int {
        if flags & 0x10 as std::os::raw::c_int != 0 {
            tcc_enter_state(s1);
            Some(
                _tcc_error_noabort
                    as unsafe extern "C" fn(_: *const std::os::raw::c_char, _: ...) -> (),
            )
            .expect("non-null function pointer")(
                b"file \'%s\' not found\x00" as *const u8 as *const std::os::raw::c_char,
                filename,
            );
        }
        return ret;
    }
    (*s1).current_filename = filename;
    if flags & 0x40 as std::os::raw::c_int != 0 {
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
        let mut obj_type: std::os::raw::c_int = 0;
        obj_type = tcc_object_type(fd, &mut ehdr);
        lseek(
            fd,
            0 as std::os::raw::c_int as __off_t,
            0 as std::os::raw::c_int,
        );
        match obj_type {
            1 => {
                ret =
                    tcc_load_object_file(s1, fd, 0 as std::os::raw::c_int as std::os::raw::c_ulong)
            },
            3 => {
                ret = tcc_load_archive(
                    s1,
                    fd,
                    (flags & 0x80 as std::os::raw::c_int == 0) as std::os::raw::c_int,
                )
            },
            2 => {
                if (*s1).output_type == 1 as std::os::raw::c_int {
                    let mut dl: *mut std::os::raw::c_void = dlopen(
                        filename,
                        0x100 as std::os::raw::c_int | 0x1 as std::os::raw::c_int,
                    );
                    if !dl.is_null() {
                        let ref mut fresh6 = (*tcc_add_dllref(s1, filename)).handle;
                        *fresh6 = dl;
                        ret = 0 as std::os::raw::c_int
                    }
                } else {
                    ret = tcc_load_dll(
                        s1,
                        fd,
                        filename,
                        (flags & 0x20 as std::os::raw::c_int != 0 as std::os::raw::c_int)
                            as std::os::raw::c_int,
                    )
                }
            },
            _ => {
                /* as GNU ld, consider it is an ld script if not recognized */
                ret = tcc_load_ldscript(s1, fd);
                /* !TCC_TARGET_PE */
                if ret < 0 as std::os::raw::c_int {
                    tcc_enter_state(s1);
                    Some(
                        _tcc_error_noabort
                            as unsafe extern "C" fn(_: *const std::os::raw::c_char, _: ...) -> (),
                    )
                    .expect("non-null function pointer")(
                        b"%s: unrecognized file type\x00" as *const u8
                            as *const std::os::raw::c_char,
                        filename,
                    );
                }
            },
        }
        close(fd);
    } else {
        /* update target deps */
        dynarray_add(
            &mut (*s1).target_deps as *mut *mut *mut std::os::raw::c_char
                as *mut std::os::raw::c_void,
            &mut (*s1).nb_target_deps,
            tcc_strdup(filename) as *mut std::os::raw::c_void,
        );
        ret = tcc_compile(s1, flags, filename, fd)
    }
    (*s1).current_filename = 0 as *const std::os::raw::c_char;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn tcc_add_file(
    mut s: *mut TCCState,
    mut filename: *const std::os::raw::c_char,
) -> std::os::raw::c_int {
    let mut filetype: std::os::raw::c_int = (*s).filetype as std::os::raw::c_int;
    if 0 as std::os::raw::c_int
        == filetype & (15 as std::os::raw::c_int | 0x40 as std::os::raw::c_int)
    {
        /* use a file extension to detect a filetype */
        let mut ext: *const std::os::raw::c_char = tcc_fileextension(filename);
        if *ext.offset(0 as std::os::raw::c_int as isize) != 0 {
            ext = ext.offset(1);
            if strcmp(ext, b"S\x00" as *const u8 as *const std::os::raw::c_char) == 0 {
                filetype = 4 as std::os::raw::c_int
            } else if strcmp(ext, b"s\x00" as *const u8 as *const std::os::raw::c_char) == 0 {
                filetype = 2 as std::os::raw::c_int
            } else if strcmp(ext, b"c\x00" as *const u8 as *const std::os::raw::c_char) == 0
                || strcmp(ext, b"i\x00" as *const u8 as *const std::os::raw::c_char) == 0
            {
                filetype = 1 as std::os::raw::c_int
            } else {
                filetype |= 0x40 as std::os::raw::c_int
            }
        } else {
            filetype = 1 as std::os::raw::c_int
        }
    }
    return tcc_add_file_internal(s, filename, filetype | 0x10 as std::os::raw::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn tcc_add_library_path(
    mut s: *mut TCCState,
    mut pathname: *const std::os::raw::c_char,
) -> std::os::raw::c_int {
    tcc_split_path(
        s,
        &mut (*s).library_paths as *mut *mut *mut std::os::raw::c_char as *mut std::os::raw::c_void,
        &mut (*s).nb_library_paths,
        pathname,
    );
    return 0 as std::os::raw::c_int;
}
unsafe extern "C" fn tcc_add_library_internal(
    mut s: *mut TCCState,
    mut fmt: *const std::os::raw::c_char,
    mut filename: *const std::os::raw::c_char,
    mut flags: std::os::raw::c_int,
    mut paths: *mut *mut std::os::raw::c_char,
    mut nb_paths: std::os::raw::c_int,
) -> std::os::raw::c_int {
    let mut buf: [std::os::raw::c_char; 1024] = [0; 1024];
    let mut i: std::os::raw::c_int = 0;
    i = 0 as std::os::raw::c_int;
    while i < nb_paths {
        snprintf(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[std::os::raw::c_char; 1024]>() as std::os::raw::c_ulong,
            fmt,
            *paths.offset(i as isize),
            filename,
        );
        if tcc_add_file_internal(s, buf.as_mut_ptr(), flags | 0x40 as std::os::raw::c_int)
            == 0 as std::os::raw::c_int
        {
            return 0 as std::os::raw::c_int;
        }
        i += 1
    }
    return -(1 as std::os::raw::c_int);
}
/* find and load a dll. Return non zero if not found */
/* XXX: add '-rpath' option support ? */
#[no_mangle]
pub unsafe extern "C" fn tcc_add_dll(
    mut s: *mut TCCState,
    mut filename: *const std::os::raw::c_char,
    mut flags: std::os::raw::c_int,
) -> std::os::raw::c_int {
    return tcc_add_library_internal(
        s,
        b"%s/%s\x00" as *const u8 as *const std::os::raw::c_char,
        filename,
        flags,
        (*s).library_paths,
        (*s).nb_library_paths,
    );
}
#[no_mangle]
pub unsafe extern "C" fn tcc_add_crt(
    mut s1: *mut TCCState,
    mut filename: *const std::os::raw::c_char,
) -> std::os::raw::c_int {
    if -(1 as std::os::raw::c_int)
        == tcc_add_library_internal(
            s1,
            b"%s/%s\x00" as *const u8 as *const std::os::raw::c_char,
            filename,
            0 as std::os::raw::c_int,
            (*s1).crt_paths,
            (*s1).nb_crt_paths,
        )
    {
        tcc_enter_state(s1);
        Some(
            _tcc_error_noabort
                as unsafe extern "C" fn(_: *const std::os::raw::c_char, _: ...) -> (),
        )
        .expect("non-null function pointer")(
            b"file \'%s\' not found\x00" as *const u8 as *const std::os::raw::c_char,
            filename,
        );
    }
    return 0 as std::os::raw::c_int;
}
/* the library name is the same as the argument of the '-l' option */
#[no_mangle]
pub unsafe extern "C" fn tcc_add_library(
    mut s: *mut TCCState,
    mut libraryname: *const std::os::raw::c_char,
) -> std::os::raw::c_int {
    static mut libs: [*const std::os::raw::c_char; 3] = [
        b"%s/lib%s.so\x00" as *const u8 as *const std::os::raw::c_char,
        b"%s/lib%s.a\x00" as *const u8 as *const std::os::raw::c_char,
        0 as *const std::os::raw::c_char,
    ];
    let mut pp: *const *const std::os::raw::c_char = if (*s).static_link as std::os::raw::c_int != 0
    {
        libs.as_ptr().offset(1 as std::os::raw::c_int as isize)
    } else {
        libs.as_ptr()
    };
    let mut flags: std::os::raw::c_int =
        (*s).filetype as std::os::raw::c_int & 0x80 as std::os::raw::c_int;
    while !(*pp).is_null() {
        if 0 as std::os::raw::c_int
            == tcc_add_library_internal(
                s,
                *pp,
                libraryname,
                flags,
                (*s).library_paths,
                (*s).nb_library_paths,
            )
        {
            return 0 as std::os::raw::c_int;
        }
        pp = pp.offset(1)
    }
    return -(1 as std::os::raw::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn tcc_add_library_err(
    mut s1: *mut TCCState,
    mut libname: *const std::os::raw::c_char,
) -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = tcc_add_library(s1, libname);
    if ret < 0 as std::os::raw::c_int {
        tcc_enter_state(s1);
        Some(
            _tcc_error_noabort
                as unsafe extern "C" fn(_: *const std::os::raw::c_char, _: ...) -> (),
        )
        .expect("non-null function pointer")(
            b"library \'%s\' not found\x00" as *const u8 as *const std::os::raw::c_char,
            libname,
        );
    }
    return ret;
}
/* handle #pragma comment(lib,) */
#[no_mangle]
pub unsafe extern "C" fn tcc_add_pragma_libs(mut s1: *mut TCCState) {
    let mut i: std::os::raw::c_int = 0; /* NULL: SHN_ABS */
    i = 0 as std::os::raw::c_int;
    while i < (*s1).nb_pragma_libs {
        tcc_add_library_err(s1, *(*s1).pragma_libs.offset(i as isize));
        i += 1
    }
}
#[no_mangle]
pub unsafe extern "C" fn tcc_add_symbol(
    mut s1: *mut TCCState,
    mut name: *const std::os::raw::c_char,
    mut val: *const std::os::raw::c_void,
) -> std::os::raw::c_int {
    let mut buf: [std::os::raw::c_char; 256] = [0; 256];
    if (*s1).leading_underscore != 0 {
        buf[0 as std::os::raw::c_int as usize] = '_' as i32 as std::os::raw::c_char;
        pstrcpy(
            buf.as_mut_ptr().offset(1 as std::os::raw::c_int as isize),
            (::std::mem::size_of::<[std::os::raw::c_char; 256]>() as std::os::raw::c_ulong)
                .wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong),
            name,
        );
        name = buf.as_mut_ptr()
    }
    set_global_sym(s1, name, 0 as *mut Section, val as uintptr_t);
    return 0 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn tcc_set_lib_path(
    mut s: *mut TCCState,
    mut path: *const std::os::raw::c_char,
) {
    tcc_free((*s).tcc_lib_path as *mut std::os::raw::c_void);
    (*s).tcc_lib_path = tcc_strdup(path);
}
unsafe extern "C" fn no_flag(mut pp: *mut *const std::os::raw::c_char) -> std::os::raw::c_int {
    let mut p: *const std::os::raw::c_char = *pp;
    if *p as std::os::raw::c_int != 'n' as i32
        || {
            p = p.offset(1);
            (*p as std::os::raw::c_int) != 'o' as i32
        }
        || {
            p = p.offset(1);
            (*p as std::os::raw::c_int) != '-' as i32
        }
    {
        return 0 as std::os::raw::c_int;
    }
    *pp = p.offset(1 as std::os::raw::c_int as isize);
    return 1 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn set_flag(
    mut s: *mut TCCState,
    mut flags: *const FlagDef,
    mut name: *const std::os::raw::c_char,
) -> std::os::raw::c_int {
    let mut value: std::os::raw::c_int = 0;
    let mut ret: std::os::raw::c_int = 0;
    let mut p: *const FlagDef = 0 as *const FlagDef;
    let mut r: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    value = 1 as std::os::raw::c_int;
    r = name;
    if no_flag(&mut r) != 0 {
        value = 0 as std::os::raw::c_int
    }
    let mut current_block_11: u64;
    ret = -(1 as std::os::raw::c_int);
    p = flags;
    while !(*p).name.is_null() {
        if ret != 0 {
            if strcmp(r, (*p).name) != 0 {
                current_block_11 = 11875828834189669668;
            } else {
                current_block_11 = 12209867499936983673;
            }
        } else if 0 as std::os::raw::c_int
            == (*p).flags as std::os::raw::c_int & 0x1 as std::os::raw::c_int
        {
            current_block_11 = 11875828834189669668;
        } else {
            current_block_11 = 12209867499936983673;
        }
        match current_block_11 {
            12209867499936983673 => {
                if (*p).offset != 0 {
                    *(s as *mut std::os::raw::c_uchar)
                        .offset((*p).offset as std::os::raw::c_int as isize) =
                        if (*p).flags as std::os::raw::c_int & 0x2 as std::os::raw::c_int != 0 {
                            (value == 0) as std::os::raw::c_int
                        } else {
                            value
                        } as std::os::raw::c_uchar;
                    if ret != 0 {
                        return 0 as std::os::raw::c_int;
                    }
                } else {
                    ret = 0 as std::os::raw::c_int
                }
            },
            _ => {},
        }
        p = p.offset(1)
    }
    return ret;
}
unsafe extern "C" fn strstart(
    mut val: *const std::os::raw::c_char,
    mut str: *mut *const std::os::raw::c_char,
) -> std::os::raw::c_int {
    let mut p: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut q: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    p = *str;
    q = val;
    while *q != 0 {
        if *p as std::os::raw::c_int != *q as std::os::raw::c_int {
            return 0 as std::os::raw::c_int;
        }
        p = p.offset(1);
        q = q.offset(1)
    }
    *str = p;
    return 1 as std::os::raw::c_int;
}
/* Like strstart, but automatically takes into account that ld options can
 *
 * - start with double or single dash (e.g. '--soname' or '-soname')
 * - arguments can be given as separate or after '=' (e.g. '-Wl,-soname,x.so'
 *   or '-Wl,-soname=x.so')
 *
 * you provide `val` always in 'option[=]' form (no leading -)
 */
unsafe extern "C" fn link_option(
    mut str: *const std::os::raw::c_char,
    mut val: *const std::os::raw::c_char,
    mut ptr: *mut *const std::os::raw::c_char,
) -> std::os::raw::c_int {
    let mut p: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut q: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut ret: std::os::raw::c_int = 0;
    /* there should be 1 or 2 dashes */
    let fresh7 = str;
    str = str.offset(1);
    if *fresh7 as std::os::raw::c_int != '-' as i32 {
        return 0 as std::os::raw::c_int;
    }
    if *str as std::os::raw::c_int == '-' as i32 {
        str = str.offset(1)
    }
    /* then str & val should match (potentially up to '=') */
    p = str;
    q = val;
    ret = 1 as std::os::raw::c_int;
    if *q.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int == '?' as i32 {
        q = q.offset(1);
        if no_flag(&mut p) != 0 {
            ret = -(1 as std::os::raw::c_int)
        }
    }
    while *q as std::os::raw::c_int != '\u{0}' as i32 && *q as std::os::raw::c_int != '=' as i32 {
        if *p as std::os::raw::c_int != *q as std::os::raw::c_int {
            return 0 as std::os::raw::c_int;
        }
        p = p.offset(1);
        q = q.offset(1)
    }
    /* '=' near eos means ',' or '=' is ok */
    if *q as std::os::raw::c_int == '=' as i32 {
        if *p as std::os::raw::c_int == 0 as std::os::raw::c_int {
            *ptr = p
        }
        if *p as std::os::raw::c_int != ',' as i32 && *p as std::os::raw::c_int != '=' as i32 {
            return 0 as std::os::raw::c_int;
        }
        p = p.offset(1)
    } else if *p != 0 {
        return 0 as std::os::raw::c_int;
    }
    *ptr = p;
    return ret;
}
unsafe extern "C" fn skip_linker_arg(
    mut str: *mut *const std::os::raw::c_char,
) -> *const std::os::raw::c_char {
    let mut s1: *const std::os::raw::c_char = *str;
    let mut s2: *const std::os::raw::c_char = strchr(s1, ',' as i32);
    *str = if !s2.is_null() {
        let fresh8 = s2;
        s2 = s2.offset(1);
        fresh8
    } else {
        s2 = s1.offset(strlen(s1) as isize);
        s2
    };
    return s2;
}
unsafe extern "C" fn copy_linker_arg(
    mut pp: *mut *mut std::os::raw::c_char,
    mut s: *const std::os::raw::c_char,
    mut sep: std::os::raw::c_int,
) {
    let mut q: *const std::os::raw::c_char = s;
    let mut p: *mut std::os::raw::c_char = *pp;
    let mut l: std::os::raw::c_int = 0 as std::os::raw::c_int;
    if !p.is_null() && sep != 0 {
        l = strlen(p) as std::os::raw::c_int;
        *p.offset(l as isize) = sep as std::os::raw::c_char;
        l += 1
    }
    skip_linker_arg(&mut q);
    *pp = tcc_realloc(
        p as *mut std::os::raw::c_void,
        (q.offset_from(s) as std::os::raw::c_long
            + l as std::os::raw::c_long
            + 1 as std::os::raw::c_int as std::os::raw::c_long) as std::os::raw::c_ulong,
    ) as *mut std::os::raw::c_char;
    pstrncpy(
        (*pp).offset(l as isize),
        s,
        q.offset_from(s) as std::os::raw::c_long as size_t,
    );
}
/* set linker options */
unsafe extern "C" fn tcc_set_linker(
    mut s: *mut TCCState,
    mut option: *const std::os::raw::c_char,
) -> std::os::raw::c_int {
    let mut s1: *mut TCCState = s;
    while *option != 0 {
        let mut p: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
        let mut end: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
        let mut ignoring: std::os::raw::c_int = 0 as std::os::raw::c_int;
        let mut ret: std::os::raw::c_int = 0;
        if link_option(
            option,
            b"Bsymbolic\x00" as *const u8 as *const std::os::raw::c_char,
            &mut p,
        ) != 0
        {
            (*s).symbolic = 1 as std::os::raw::c_int as std::os::raw::c_uchar
        } else if link_option(
            option,
            b"nostdlib\x00" as *const u8 as *const std::os::raw::c_char,
            &mut p,
        ) != 0
        {
            (*s).nostdlib = 1 as std::os::raw::c_int as std::os::raw::c_uchar
        } else if link_option(
            option,
            b"fini=\x00" as *const u8 as *const std::os::raw::c_char,
            &mut p,
        ) != 0
        {
            copy_linker_arg(&mut (*s).fini_symbol, p, 0 as std::os::raw::c_int);
            ignoring = 1 as std::os::raw::c_int
        } else if link_option(
            option,
            b"image-base=\x00" as *const u8 as *const std::os::raw::c_char,
            &mut p,
        ) != 0
            || link_option(
                option,
                b"Ttext=\x00" as *const u8 as *const std::os::raw::c_char,
                &mut p,
            ) != 0
        {
            (*s).text_addr = strtoull(p, &mut end, 16 as std::os::raw::c_int) as Elf64_Addr;
            (*s).has_text_addr = 1 as std::os::raw::c_int as std::os::raw::c_uchar
        } else if link_option(
            option,
            b"init=\x00" as *const u8 as *const std::os::raw::c_char,
            &mut p,
        ) != 0
        {
            copy_linker_arg(&mut (*s).init_symbol, p, 0 as std::os::raw::c_int);
            ignoring = 1 as std::os::raw::c_int
        } else {
            let mut current_block_41: u64;
            if link_option(
                option,
                b"oformat=\x00" as *const u8 as *const std::os::raw::c_char,
                &mut p,
            ) != 0
            {
                if strstart(
                    b"elf64-\x00" as *const u8 as *const std::os::raw::c_char,
                    &mut p,
                ) != 0
                {
                    (*s).output_format = 0 as std::os::raw::c_int;
                    current_block_41 = 12829669402821218572;
                } else if strcmp(p, b"binary\x00" as *const u8 as *const std::os::raw::c_char) == 0
                {
                    (*s).output_format = 1 as std::os::raw::c_int;
                    current_block_41 = 12829669402821218572;
                } else {
                    current_block_41 = 3988677119214552796;
                }
            } else if link_option(
                option,
                b"as-needed\x00" as *const u8 as *const std::os::raw::c_char,
                &mut p,
            ) != 0
            {
                ignoring = 1 as std::os::raw::c_int;
                current_block_41 = 12829669402821218572;
            } else if link_option(
                option,
                b"O\x00" as *const u8 as *const std::os::raw::c_char,
                &mut p,
            ) != 0
            {
                ignoring = 1 as std::os::raw::c_int;
                current_block_41 = 12829669402821218572;
            } else if link_option(
                option,
                b"export-all-symbols\x00" as *const u8 as *const std::os::raw::c_char,
                &mut p,
            ) != 0
            {
                (*s).rdynamic = 1 as std::os::raw::c_int as std::os::raw::c_uchar;
                current_block_41 = 12829669402821218572;
            } else if link_option(
                option,
                b"export-dynamic\x00" as *const u8 as *const std::os::raw::c_char,
                &mut p,
            ) != 0
            {
                (*s).rdynamic = 1 as std::os::raw::c_int as std::os::raw::c_uchar;
                current_block_41 = 12829669402821218572;
            } else if link_option(
                option,
                b"rpath=\x00" as *const u8 as *const std::os::raw::c_char,
                &mut p,
            ) != 0
            {
                copy_linker_arg(&mut (*s).rpath, p, ':' as i32);
                current_block_41 = 12829669402821218572;
            } else if link_option(
                option,
                b"enable-new-dtags\x00" as *const u8 as *const std::os::raw::c_char,
                &mut p,
            ) != 0
            {
                (*s).enable_new_dtags = 1 as std::os::raw::c_int as std::os::raw::c_uchar;
                current_block_41 = 12829669402821218572;
            } else if link_option(
                option,
                b"section-alignment=\x00" as *const u8 as *const std::os::raw::c_char,
                &mut p,
            ) != 0
            {
                (*s).section_align =
                    strtoul(p, &mut end, 16 as std::os::raw::c_int) as std::os::raw::c_uint;
                current_block_41 = 12829669402821218572;
            } else if link_option(
                option,
                b"soname=\x00" as *const u8 as *const std::os::raw::c_char,
                &mut p,
            ) != 0
            {
                copy_linker_arg(&mut (*s).soname, p, 0 as std::os::raw::c_int);
                current_block_41 = 12829669402821218572;
            } else {
                ret = link_option(
                    option,
                    b"?whole-archive\x00" as *const u8 as *const std::os::raw::c_char,
                    &mut p,
                );
                if ret != 0 {
                    if ret > 0 as std::os::raw::c_int {
                        (*s).filetype = ((*s).filetype as std::os::raw::c_int
                            | 0x80 as std::os::raw::c_int)
                            as std::os::raw::c_uchar
                    } else {
                        (*s).filetype = ((*s).filetype as std::os::raw::c_int
                            & !(0x80 as std::os::raw::c_int))
                            as std::os::raw::c_uchar
                    }
                    current_block_41 = 12829669402821218572;
                } else if link_option(
                    option,
                    b"z=\x00" as *const u8 as *const std::os::raw::c_char,
                    &mut p,
                ) != 0
                {
                    ignoring = 1 as std::os::raw::c_int;
                    current_block_41 = 12829669402821218572;
                } else if !p.is_null() {
                    return 0 as std::os::raw::c_int;
                } else {
                    current_block_41 = 3988677119214552796;
                }
            }
            match current_block_41 {
                3988677119214552796 => {
                    tcc_enter_state(s1);
                    Some(
                        _tcc_error
                            as unsafe extern "C" fn(_: *const std::os::raw::c_char, _: ...) -> !,
                    )
                    .expect("non-null function pointer")(
                        b"unsupported linker option \'%s\'\x00" as *const u8
                            as *const std::os::raw::c_char,
                        option,
                    );
                },
                _ => {},
            }
        }
        if ignoring != 0 && (*s).warn_unsupported as std::os::raw::c_int != 0 {
            tcc_enter_state(s1);
            Some(
                _tcc_warning as unsafe extern "C" fn(_: *const std::os::raw::c_char, _: ...) -> (),
            )
            .expect("non-null function pointer")(
                b"unsupported linker option \'%s\'\x00" as *const u8 as *const std::os::raw::c_char,
                option,
            );
        }
        option = skip_linker_arg(&mut p)
    }
    return 1 as std::os::raw::c_int;
}
/* cannot have space before option and arg */
static mut tcc_options: [TCCOption; 53] = [
    {
        let mut init = TCCOption {
            name: b"h\x00" as *const u8 as *const std::os::raw::c_char,
            index: TCC_OPTION_HELP as std::os::raw::c_int as uint16_t,
            flags: 0 as std::os::raw::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = TCCOption {
            name: b"-help\x00" as *const u8 as *const std::os::raw::c_char,
            index: TCC_OPTION_HELP as std::os::raw::c_int as uint16_t,
            flags: 0 as std::os::raw::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = TCCOption {
            name: b"?\x00" as *const u8 as *const std::os::raw::c_char,
            index: TCC_OPTION_HELP as std::os::raw::c_int as uint16_t,
            flags: 0 as std::os::raw::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = TCCOption {
            name: b"hh\x00" as *const u8 as *const std::os::raw::c_char,
            index: TCC_OPTION_HELP2 as std::os::raw::c_int as uint16_t,
            flags: 0 as std::os::raw::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = TCCOption {
            name: b"v\x00" as *const u8 as *const std::os::raw::c_char,
            index: TCC_OPTION_v as std::os::raw::c_int as uint16_t,
            flags: (0x1 as std::os::raw::c_int | 0x2 as std::os::raw::c_int) as uint16_t,
        };
        init
    },
    {
        let mut init = TCCOption {
            name: b"-version\x00" as *const u8 as *const std::os::raw::c_char,
            index: TCC_OPTION_v as std::os::raw::c_int as uint16_t,
            flags: 0 as std::os::raw::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = TCCOption {
            name: b"I\x00" as *const u8 as *const std::os::raw::c_char,
            index: TCC_OPTION_I as std::os::raw::c_int as uint16_t,
            flags: 0x1 as std::os::raw::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = TCCOption {
            name: b"D\x00" as *const u8 as *const std::os::raw::c_char,
            index: TCC_OPTION_D as std::os::raw::c_int as uint16_t,
            flags: 0x1 as std::os::raw::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = TCCOption {
            name: b"U\x00" as *const u8 as *const std::os::raw::c_char,
            index: TCC_OPTION_U as std::os::raw::c_int as uint16_t,
            flags: 0x1 as std::os::raw::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = TCCOption {
            name: b"P\x00" as *const u8 as *const std::os::raw::c_char,
            index: TCC_OPTION_P as std::os::raw::c_int as uint16_t,
            flags: (0x1 as std::os::raw::c_int | 0x2 as std::os::raw::c_int) as uint16_t,
        };
        init
    },
    {
        let mut init = TCCOption {
            name: b"L\x00" as *const u8 as *const std::os::raw::c_char,
            index: TCC_OPTION_L as std::os::raw::c_int as uint16_t,
            flags: 0x1 as std::os::raw::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = TCCOption {
            name: b"B\x00" as *const u8 as *const std::os::raw::c_char,
            index: TCC_OPTION_B as std::os::raw::c_int as uint16_t,
            flags: 0x1 as std::os::raw::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = TCCOption {
            name: b"l\x00" as *const u8 as *const std::os::raw::c_char,
            index: TCC_OPTION_l as std::os::raw::c_int as uint16_t,
            flags: 0x1 as std::os::raw::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = TCCOption {
            name: b"bench\x00" as *const u8 as *const std::os::raw::c_char,
            index: TCC_OPTION_bench as std::os::raw::c_int as uint16_t,
            flags: 0 as std::os::raw::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = TCCOption {
            name: b"bt\x00" as *const u8 as *const std::os::raw::c_char,
            index: TCC_OPTION_bt as std::os::raw::c_int as uint16_t,
            flags: (0x1 as std::os::raw::c_int | 0x2 as std::os::raw::c_int) as uint16_t,
        };
        init
    },
    {
        let mut init = TCCOption {
            name: b"b\x00" as *const u8 as *const std::os::raw::c_char,
            index: TCC_OPTION_b as std::os::raw::c_int as uint16_t,
            flags: 0 as std::os::raw::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = TCCOption {
            name: b"g\x00" as *const u8 as *const std::os::raw::c_char,
            index: TCC_OPTION_g as std::os::raw::c_int as uint16_t,
            flags: (0x1 as std::os::raw::c_int | 0x2 as std::os::raw::c_int) as uint16_t,
        };
        init
    },
    {
        let mut init = TCCOption {
            name: b"c\x00" as *const u8 as *const std::os::raw::c_char,
            index: TCC_OPTION_c as std::os::raw::c_int as uint16_t,
            flags: 0 as std::os::raw::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = TCCOption {
            name: b"dumpversion\x00" as *const u8 as *const std::os::raw::c_char,
            index: TCC_OPTION_dumpversion as std::os::raw::c_int as uint16_t,
            flags: 0 as std::os::raw::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = TCCOption {
            name: b"d\x00" as *const u8 as *const std::os::raw::c_char,
            index: TCC_OPTION_d as std::os::raw::c_int as uint16_t,
            flags: (0x1 as std::os::raw::c_int | 0x2 as std::os::raw::c_int) as uint16_t,
        };
        init
    },
    {
        let mut init = TCCOption {
            name: b"static\x00" as *const u8 as *const std::os::raw::c_char,
            index: TCC_OPTION_static as std::os::raw::c_int as uint16_t,
            flags: 0 as std::os::raw::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = TCCOption {
            name: b"std\x00" as *const u8 as *const std::os::raw::c_char,
            index: TCC_OPTION_std as std::os::raw::c_int as uint16_t,
            flags: (0x1 as std::os::raw::c_int | 0x2 as std::os::raw::c_int) as uint16_t,
        };
        init
    },
    {
        let mut init = TCCOption {
            name: b"shared\x00" as *const u8 as *const std::os::raw::c_char,
            index: TCC_OPTION_shared as std::os::raw::c_int as uint16_t,
            flags: 0 as std::os::raw::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = TCCOption {
            name: b"soname\x00" as *const u8 as *const std::os::raw::c_char,
            index: TCC_OPTION_soname as std::os::raw::c_int as uint16_t,
            flags: 0x1 as std::os::raw::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = TCCOption {
            name: b"o\x00" as *const u8 as *const std::os::raw::c_char,
            index: TCC_OPTION_o as std::os::raw::c_int as uint16_t,
            flags: 0x1 as std::os::raw::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = TCCOption {
            name: b"-param\x00" as *const u8 as *const std::os::raw::c_char,
            index: TCC_OPTION_param as std::os::raw::c_int as uint16_t,
            flags: 0x1 as std::os::raw::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = TCCOption {
            name: b"pedantic\x00" as *const u8 as *const std::os::raw::c_char,
            index: TCC_OPTION_pedantic as std::os::raw::c_int as uint16_t,
            flags: 0 as std::os::raw::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = TCCOption {
            name: b"pthread\x00" as *const u8 as *const std::os::raw::c_char,
            index: TCC_OPTION_pthread as std::os::raw::c_int as uint16_t,
            flags: 0 as std::os::raw::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = TCCOption {
            name: b"run\x00" as *const u8 as *const std::os::raw::c_char,
            index: TCC_OPTION_run as std::os::raw::c_int as uint16_t,
            flags: (0x1 as std::os::raw::c_int | 0x2 as std::os::raw::c_int) as uint16_t,
        };
        init
    },
    {
        let mut init = TCCOption {
            name: b"rdynamic\x00" as *const u8 as *const std::os::raw::c_char,
            index: TCC_OPTION_rdynamic as std::os::raw::c_int as uint16_t,
            flags: 0 as std::os::raw::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = TCCOption {
            name: b"r\x00" as *const u8 as *const std::os::raw::c_char,
            index: TCC_OPTION_r as std::os::raw::c_int as uint16_t,
            flags: 0 as std::os::raw::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = TCCOption {
            name: b"s\x00" as *const u8 as *const std::os::raw::c_char,
            index: TCC_OPTION_s as std::os::raw::c_int as uint16_t,
            flags: 0 as std::os::raw::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = TCCOption {
            name: b"traditional\x00" as *const u8 as *const std::os::raw::c_char,
            index: TCC_OPTION_traditional as std::os::raw::c_int as uint16_t,
            flags: 0 as std::os::raw::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = TCCOption {
            name: b"Wl,\x00" as *const u8 as *const std::os::raw::c_char,
            index: TCC_OPTION_Wl as std::os::raw::c_int as uint16_t,
            flags: (0x1 as std::os::raw::c_int | 0x2 as std::os::raw::c_int) as uint16_t,
        };
        init
    },
    {
        let mut init = TCCOption {
            name: b"Wp,\x00" as *const u8 as *const std::os::raw::c_char,
            index: TCC_OPTION_Wp as std::os::raw::c_int as uint16_t,
            flags: (0x1 as std::os::raw::c_int | 0x2 as std::os::raw::c_int) as uint16_t,
        };
        init
    },
    {
        let mut init = TCCOption {
            name: b"W\x00" as *const u8 as *const std::os::raw::c_char,
            index: TCC_OPTION_W as std::os::raw::c_int as uint16_t,
            flags: (0x1 as std::os::raw::c_int | 0x2 as std::os::raw::c_int) as uint16_t,
        };
        init
    },
    {
        let mut init = TCCOption {
            name: b"O\x00" as *const u8 as *const std::os::raw::c_char,
            index: TCC_OPTION_O as std::os::raw::c_int as uint16_t,
            flags: (0x1 as std::os::raw::c_int | 0x2 as std::os::raw::c_int) as uint16_t,
        };
        init
    },
    {
        let mut init = TCCOption {
            name: b"m\x00" as *const u8 as *const std::os::raw::c_char,
            index: TCC_OPTION_m as std::os::raw::c_int as uint16_t,
            flags: (0x1 as std::os::raw::c_int | 0x2 as std::os::raw::c_int) as uint16_t,
        };
        init
    },
    {
        let mut init = TCCOption {
            name: b"f\x00" as *const u8 as *const std::os::raw::c_char,
            index: TCC_OPTION_f as std::os::raw::c_int as uint16_t,
            flags: (0x1 as std::os::raw::c_int | 0x2 as std::os::raw::c_int) as uint16_t,
        };
        init
    },
    {
        let mut init = TCCOption {
            name: b"isystem\x00" as *const u8 as *const std::os::raw::c_char,
            index: TCC_OPTION_isystem as std::os::raw::c_int as uint16_t,
            flags: 0x1 as std::os::raw::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = TCCOption {
            name: b"include\x00" as *const u8 as *const std::os::raw::c_char,
            index: TCC_OPTION_include as std::os::raw::c_int as uint16_t,
            flags: 0x1 as std::os::raw::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = TCCOption {
            name: b"nostdinc\x00" as *const u8 as *const std::os::raw::c_char,
            index: TCC_OPTION_nostdinc as std::os::raw::c_int as uint16_t,
            flags: 0 as std::os::raw::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = TCCOption {
            name: b"nostdlib\x00" as *const u8 as *const std::os::raw::c_char,
            index: TCC_OPTION_nostdlib as std::os::raw::c_int as uint16_t,
            flags: 0 as std::os::raw::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = TCCOption {
            name: b"print-search-dirs\x00" as *const u8 as *const std::os::raw::c_char,
            index: TCC_OPTION_print_search_dirs as std::os::raw::c_int as uint16_t,
            flags: 0 as std::os::raw::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = TCCOption {
            name: b"w\x00" as *const u8 as *const std::os::raw::c_char,
            index: TCC_OPTION_w as std::os::raw::c_int as uint16_t,
            flags: 0 as std::os::raw::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = TCCOption {
            name: b"pipe\x00" as *const u8 as *const std::os::raw::c_char,
            index: TCC_OPTION_pipe as std::os::raw::c_int as uint16_t,
            flags: 0 as std::os::raw::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = TCCOption {
            name: b"E\x00" as *const u8 as *const std::os::raw::c_char,
            index: TCC_OPTION_E as std::os::raw::c_int as uint16_t,
            flags: 0 as std::os::raw::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = TCCOption {
            name: b"MD\x00" as *const u8 as *const std::os::raw::c_char,
            index: TCC_OPTION_MD as std::os::raw::c_int as uint16_t,
            flags: 0 as std::os::raw::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = TCCOption {
            name: b"MF\x00" as *const u8 as *const std::os::raw::c_char,
            index: TCC_OPTION_MF as std::os::raw::c_int as uint16_t,
            flags: 0x1 as std::os::raw::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = TCCOption {
            name: b"x\x00" as *const u8 as *const std::os::raw::c_char,
            index: TCC_OPTION_x as std::os::raw::c_int as uint16_t,
            flags: 0x1 as std::os::raw::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = TCCOption {
            name: b"ar\x00" as *const u8 as *const std::os::raw::c_char,
            index: TCC_OPTION_ar as std::os::raw::c_int as uint16_t,
            flags: 0 as std::os::raw::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = TCCOption {
            name: b"C\x00" as *const u8 as *const std::os::raw::c_char,
            index: TCC_OPTION_C as std::os::raw::c_int as uint16_t,
            flags: 0 as std::os::raw::c_int as uint16_t,
        };
        init
    },
    {
        let mut init = TCCOption {
            name: 0 as *const std::os::raw::c_char,
            index: 0 as std::os::raw::c_int as uint16_t,
            flags: 0 as std::os::raw::c_int as uint16_t,
        };
        init
    },
];
// Initialized in run_static_initializers
static mut options_W: [FlagDef; 7] = [FlagDef {
    offset: 0,
    flags: 0,
    name: 0 as *const std::os::raw::c_char,
}; 7];
// Initialized in run_static_initializers
static mut options_f: [FlagDef; 8] = [FlagDef {
    offset: 0,
    flags: 0,
    name: 0 as *const std::os::raw::c_char,
}; 8];
// Initialized in run_static_initializers
static mut options_m: [FlagDef; 3] = [FlagDef {
    offset: 0,
    flags: 0,
    name: 0 as *const std::os::raw::c_char,
}; 3];
unsafe extern "C" fn args_parser_add_file(
    mut s: *mut TCCState,
    mut filename: *const std::os::raw::c_char,
    mut filetype: std::os::raw::c_int,
) {
    let mut f: *mut filespec = tcc_malloc(
        (::std::mem::size_of::<filespec>() as std::os::raw::c_ulong).wrapping_add(strlen(filename)),
    ) as *mut filespec;
    (*f).type_0 = filetype as std::os::raw::c_char;
    strcpy((*f).name.as_mut_ptr(), filename);
    dynarray_add(
        &mut (*s).files as *mut *mut *mut filespec as *mut std::os::raw::c_void,
        &mut (*s).nb_files,
        f as *mut std::os::raw::c_void,
    );
}
unsafe extern "C" fn args_parser_make_argv(
    mut r: *const std::os::raw::c_char,
    mut argc: *mut std::os::raw::c_int,
    mut argv: *mut *mut *mut std::os::raw::c_char,
) -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut q: std::os::raw::c_int = 0;
    let mut c: std::os::raw::c_int = 0;
    let mut str: CString = CString {
        size: 0,
        data: 0 as *mut std::os::raw::c_void,
        size_allocated: 0,
    };
    loop {
        loop {
            c = *r as std::os::raw::c_uchar as std::os::raw::c_int;
            if !(c != 0 && c <= ' ' as i32) {
                break;
            }
            r = r.offset(1)
        }
        if c == 0 as std::os::raw::c_int {
            break;
        }
        q = 0 as std::os::raw::c_int;
        cstr_new(&mut str);
        loop {
            c = *r as std::os::raw::c_uchar as std::os::raw::c_int;
            if !(c != 0) {
                break;
            }
            r = r.offset(1);
            if c == '\\' as i32
                && (*r as std::os::raw::c_int == '\"' as i32
                    || *r as std::os::raw::c_int == '\\' as i32)
            {
                let fresh9 = r;
                r = r.offset(1);
                c = *fresh9 as std::os::raw::c_int
            } else if c == '\"' as i32 {
                q = (q == 0) as std::os::raw::c_int;
                continue;
            } else if q == 0 as std::os::raw::c_int && c <= ' ' as i32 {
                break;
            }
            cstr_ccat(&mut str, c);
        }
        cstr_ccat(&mut str, 0 as std::os::raw::c_int);
        //printf("<%s>\n", str.data), fflush(stdout);
        dynarray_add(
            argv as *mut std::os::raw::c_void,
            argc,
            tcc_strdup(str.data as *const std::os::raw::c_char) as *mut std::os::raw::c_void,
        );
        cstr_free(&mut str);
        ret += 1
    }
    return ret;
}
/* read list file */
unsafe extern "C" fn args_parser_listfile(
    mut s: *mut TCCState,
    mut filename: *const std::os::raw::c_char,
    mut optind: std::os::raw::c_int,
    mut pargc: *mut std::os::raw::c_int,
    mut pargv: *mut *mut *mut std::os::raw::c_char,
) {
    let mut s1: *mut TCCState = s; /* collect -Wl options */
    let mut fd: std::os::raw::c_int = 0;
    let mut i: std::os::raw::c_int = 0;
    let mut len: size_t = 0;
    let mut p: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut argc: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut argv: *mut *mut std::os::raw::c_char = 0 as *mut *mut std::os::raw::c_char;
    fd = open(
        filename,
        0 as std::os::raw::c_int | 0 as std::os::raw::c_int,
    );
    if fd < 0 as std::os::raw::c_int {
        tcc_enter_state(s1);
        Some(_tcc_error as unsafe extern "C" fn(_: *const std::os::raw::c_char, _: ...) -> !)
            .expect("non-null function pointer")(
            b"listfile \'%s\' not found\x00" as *const u8 as *const std::os::raw::c_char,
            filename,
        );
    }
    len = lseek(
        fd,
        0 as std::os::raw::c_int as __off_t,
        2 as std::os::raw::c_int,
    ) as size_t;
    p = tcc_malloc(len.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong))
        as *mut std::os::raw::c_char;
    *p.offset(len as isize) = 0 as std::os::raw::c_int as std::os::raw::c_char;
    lseek(
        fd,
        0 as std::os::raw::c_int as __off_t,
        0 as std::os::raw::c_int,
    );
    read(fd, p as *mut std::os::raw::c_void, len);
    close(fd);
    i = 0 as std::os::raw::c_int;
    while i < *pargc {
        if i == optind {
            args_parser_make_argv(p, &mut argc, &mut argv);
        } else {
            dynarray_add(
                &mut argv as *mut *mut *mut std::os::raw::c_char as *mut std::os::raw::c_void,
                &mut argc,
                tcc_strdup(*(*pargv).offset(i as isize)) as *mut std::os::raw::c_void,
            );
        }
        i += 1
    }
    tcc_free(p as *mut std::os::raw::c_void);
    dynarray_reset(
        &mut (*s).argv as *mut *mut *mut std::os::raw::c_char as *mut std::os::raw::c_void,
        &mut (*s).argc,
    );
    (*s).argc = argc;
    *pargc = (*s).argc;
    (*s).argv = argv;
    *pargv = (*s).argv;
}
#[no_mangle]
pub unsafe extern "C" fn tcc_parse_args(
    mut s: *mut TCCState,
    mut pargc: *mut std::os::raw::c_int,
    mut pargv: *mut *mut *mut std::os::raw::c_char,
    mut optind: std::os::raw::c_int,
) -> std::os::raw::c_int {
    let mut p1: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut r1: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut current_block: u64;
    let mut s1: *mut TCCState = s;
    let mut popt: *const TCCOption = 0 as *const TCCOption;
    let mut optarg: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut r: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut run: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut x: std::os::raw::c_int = 0;
    let mut linker_arg: CString = CString {
        size: 0,
        data: 0 as *mut std::os::raw::c_void,
        size_allocated: 0,
    };
    let mut tool: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut arg_start: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut noaction: std::os::raw::c_int = optind;
    let mut argv: *mut *mut std::os::raw::c_char = *pargv;
    let mut argc: std::os::raw::c_int = *pargc;
    cstr_new(&mut linker_arg);
    's_33: loop
    /* ignored */
    {
        if !(optind < argc) {
            current_block = 15514437232607373049;
            break;
        }
        r = *argv.offset(optind as isize);
        if *r.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int == '@' as i32
            && *r.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int != '\u{0}' as i32
        {
            args_parser_listfile(
                s,
                r.offset(1 as std::os::raw::c_int as isize),
                optind,
                &mut argc,
                &mut argv,
            );
        } else {
            optind += 1;
            if tool != 0 {
                if *r.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int == '-' as i32
                    && *r.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int
                        == 'v' as i32
                    && *r.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int
                        == 0 as std::os::raw::c_int
                {
                    (*s).verbose = (*s).verbose.wrapping_add(1)
                }
            } else {
                loop {
                    if *r.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int
                        != '-' as i32
                        || *r.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int
                            == '\u{0}' as i32
                    {
                        if *r.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int
                            != '@' as i32
                        {
                            /* allow "tcc file(s) -run @ args ..." */
                            args_parser_add_file(s, r, (*s).filetype as std::os::raw::c_int);
                        }
                        if run.is_null() {
                            continue 's_33;
                        }
                        tcc_set_options(s, run);
                        arg_start = optind - 1 as std::os::raw::c_int;
                        current_block = 15514437232607373049;
                        break 's_33;
                    } else {
                        /* find option in table */
                        popt = tcc_options.as_ptr();
                        loop {
                            p1 = (*popt).name;
                            r1 = r.offset(1 as std::os::raw::c_int as isize);
                            if p1.is_null() {
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
                                    b"invalid option -- \'%s\'\x00" as *const u8
                                        as *const std::os::raw::c_char,
                                    r,
                                );
                            }
                            if !(strstart(p1, &mut r1) == 0) {
                                optarg = r1;
                                if (*popt).flags as std::os::raw::c_int & 0x1 as std::os::raw::c_int
                                    != 0
                                {
                                    if *r1 as std::os::raw::c_int == '\u{0}' as i32
                                        && (*popt).flags as std::os::raw::c_int
                                            & 0x2 as std::os::raw::c_int
                                            == 0
                                    {
                                        current_block = 11459959175219260272;
                                        break;
                                    } else {
                                        current_block = 11048769245176032998;
                                        break;
                                    }
                                } else if !(*r1 as std::os::raw::c_int != '\u{0}' as i32) {
                                    current_block = 11048769245176032998;
                                    break;
                                }
                            }
                            popt = popt.offset(1)
                        }
                        match current_block {
                            11459959175219260272 => {
                                if optind >= argc {
                                    current_block = 10146975503812143990;
                                    break 's_33;
                                }
                                let fresh10 = optind;
                                optind = optind + 1;
                                optarg = *argv.offset(fresh10 as isize)
                            },
                            _ => {},
                        }
                        match (*popt).index as std::os::raw::c_int {
                            0 => {
                                x = 1 as std::os::raw::c_int;
                                current_block = 10931776671445766452;
                                break;
                            },
                            1 => {
                                x = 2 as std::os::raw::c_int;
                                current_block = 10931776671445766452;
                                break;
                            },
                            3 => {
                                tcc_add_include_path(s, optarg);
                                continue 's_33;
                            },
                            4 => {
                                tcc_define_symbol(s, optarg, 0 as *const std::os::raw::c_char);
                                continue 's_33;
                            },
                            5 => {
                                tcc_undefine_symbol(s, optarg);
                                continue 's_33;
                            },
                            7 => {
                                tcc_add_library_path(s, optarg);
                                continue 's_33;
                            },
                            8 => {
                                /* set tcc utilities path (mainly for tcc development) */
                                tcc_set_lib_path(s, optarg);
                                continue 's_33;
                            },
                            9 => {
                                args_parser_add_file(
                                    s,
                                    optarg,
                                    8 as std::os::raw::c_int
                                        | (*s).filetype as std::os::raw::c_int
                                            & !(15 as std::os::raw::c_int
                                                | 0x40 as std::os::raw::c_int),
                                );
                                (*s).nb_libraries += 1;
                                continue 's_33;
                            },
                            42 => {
                                (*s).option_pthread =
                                    1 as std::os::raw::c_int as std::os::raw::c_uchar;
                                continue 's_33;
                            },
                            10 => {
                                (*s).do_bench = 1 as std::os::raw::c_int as std::os::raw::c_uchar;
                                continue 's_33;
                            },
                            11 => {
                                (*s).rt_num_callers = atoi(optarg);
                                (*s).do_backtrace =
                                    1 as std::os::raw::c_int as std::os::raw::c_uchar;
                                (*s).do_debug = 1 as std::os::raw::c_int as std::os::raw::c_uchar;
                                continue 's_33;
                            },
                            12 => {
                                (*s).do_bounds_check =
                                    1 as std::os::raw::c_int as std::os::raw::c_uchar;
                                (*s).do_backtrace =
                                    1 as std::os::raw::c_int as std::os::raw::c_uchar;
                                (*s).do_debug = 1 as std::os::raw::c_int as std::os::raw::c_uchar;
                                continue 's_33;
                            },
                            14 => {
                                (*s).do_debug = 1 as std::os::raw::c_int as std::os::raw::c_uchar;
                                continue 's_33;
                            },
                            15 => {
                                x = 4 as std::os::raw::c_int;
                                current_block = 2823955538708594695;
                                break;
                            },
                            17 => {
                                if *optarg as std::os::raw::c_int == 'D' as i32 {
                                    current_block = 13660591889533726445;
                                    break;
                                } else {
                                    current_block = 5028470053297453708;
                                    break;
                                }
                            },
                            18 => {
                                (*s).static_link =
                                    1 as std::os::raw::c_int as std::os::raw::c_uchar;
                                continue 's_33;
                            },
                            19 => {
                                if strcmp(
                                    optarg,
                                    b"=c11\x00" as *const u8 as *const std::os::raw::c_char,
                                ) == 0 as std::os::raw::c_int
                                {
                                    (*s).cversion =
                                        201112 as std::os::raw::c_int as std::os::raw::c_uint
                                }
                                continue 's_33;
                            },
                            20 => {
                                x = 3 as std::os::raw::c_int;
                                current_block = 2823955538708594695;
                                break;
                            },
                            21 => {
                                (*s).soname = tcc_strdup(optarg);
                                continue 's_33;
                            },
                            22 => {
                                if !(*s).outfile.is_null() {
                                    tcc_enter_state(s1);
                                    Some(
                                        _tcc_warning
                                            as unsafe extern "C" fn(
                                                _: *const std::os::raw::c_char,
                                                _: ...
                                            )
                                                -> (),
                                    )
                                    .expect("non-null function pointer")(
                                        b"multiple -o option\x00" as *const u8
                                            as *const std::os::raw::c_char,
                                    );
                                    tcc_free((*s).outfile as *mut std::os::raw::c_void);
                                }
                                (*s).outfile = tcc_strdup(optarg);
                                continue 's_33;
                            },
                            23 => {
                                /* generate a .o merging several output files */
                                (*s).option_r = 1 as std::os::raw::c_int as std::os::raw::c_uchar;
                                x = 4 as std::os::raw::c_int;
                                current_block = 2823955538708594695;
                                break;
                            },
                            33 => {
                                tcc_add_sysinclude_path(s, optarg);
                                continue 's_33;
                            },
                            35 => {
                                cstr_printf(
                                    &mut (*s).cmdline_incl as *mut CString,
                                    b"#include \"%s\"\n\x00" as *const u8
                                        as *const std::os::raw::c_char,
                                    optarg,
                                );
                                continue 's_33;
                            },
                            36 => {
                                (*s).nostdinc = 1 as std::os::raw::c_int as std::os::raw::c_uchar;
                                continue 's_33;
                            },
                            37 => {
                                (*s).nostdlib = 1 as std::os::raw::c_int as std::os::raw::c_uchar;
                                continue 's_33;
                            },
                            43 => {
                                run = optarg;
                                x = 1 as std::os::raw::c_int;
                                current_block = 2823955538708594695;
                                break;
                            },
                            2 => {
                                loop {
                                    (*s).verbose = (*s).verbose.wrapping_add(1);
                                    let fresh11 = optarg;
                                    optarg = optarg.offset(1);
                                    if !(*fresh11 as std::os::raw::c_int == 'v' as i32) {
                                        break;
                                    }
                                }
                                noaction += 1;
                                continue 's_33;
                            },
                            32 => {
                                if set_flag(s, options_f.as_ptr(), optarg)
                                    < 0 as std::os::raw::c_int
                                {
                                    current_block = 9902503279498310510;
                                    break;
                                } else {
                                    continue 's_33;
                                }
                            },
                            31 => {
                                if !(set_flag(s, options_m.as_ptr(), optarg)
                                    < 0 as std::os::raw::c_int)
                                {
                                    continue 's_33;
                                }
                                x = atoi(optarg);
                                if x != 32 as std::os::raw::c_int && x != 64 as std::os::raw::c_int
                                {
                                    current_block = 9902503279498310510;
                                    break;
                                } else {
                                    current_block = 14851765859726653900;
                                    break;
                                }
                            },
                            28 => {
                                (*s).warn_none = 0 as std::os::raw::c_int as std::os::raw::c_uchar;
                                if *optarg.offset(0 as std::os::raw::c_int as isize)
                                    as std::os::raw::c_int
                                    != 0
                                    && set_flag(s, options_W.as_ptr(), optarg)
                                        < 0 as std::os::raw::c_int
                                {
                                    current_block = 9902503279498310510;
                                    break;
                                } else {
                                    continue 's_33;
                                }
                            },
                            44 => {
                                (*s).warn_none = 1 as std::os::raw::c_int as std::os::raw::c_uchar;
                                continue 's_33;
                            },
                            39 => {
                                (*s).rdynamic = 1 as std::os::raw::c_int as std::os::raw::c_uchar;
                                continue 's_33;
                            },
                            26 => {
                                if linker_arg.size != 0 {
                                    linker_arg.size -= 1;
                                    cstr_ccat(&mut linker_arg, ',' as i32);
                                }
                                cstr_cat(&mut linker_arg, optarg, 0 as std::os::raw::c_int);
                                if tcc_set_linker(s, linker_arg.data as *const std::os::raw::c_char)
                                    != 0
                                {
                                    cstr_free(&mut linker_arg);
                                }
                                continue 's_33;
                            },
                            27 => r = optarg,
                            46 => {
                                x = 5 as std::os::raw::c_int;
                                current_block = 2823955538708594695;
                                break;
                            },
                            6 => {
                                (*s).Pflag =
                                    (atoi(optarg) + 1 as std::os::raw::c_int) as C2RustUnnamed_3;
                                continue 's_33;
                            },
                            47 => {
                                (*s).gen_deps = 1 as std::os::raw::c_int;
                                continue 's_33;
                            },
                            48 => {
                                (*s).deps_outfile = tcc_strdup(optarg);
                                continue 's_33;
                            },
                            16 => {
                                printf(
                                    b"%s\n\x00" as *const u8 as *const std::os::raw::c_char,
                                    b"0.9.27\x00" as *const u8 as *const std::os::raw::c_char,
                                );
                                exit(0 as std::os::raw::c_int);
                            },
                            49 => {
                                x = 0 as std::os::raw::c_int;
                                if *optarg as std::os::raw::c_int == 'c' as i32 {
                                    x = 1 as std::os::raw::c_int
                                } else if *optarg as std::os::raw::c_int == 'a' as i32 {
                                    x = 4 as std::os::raw::c_int
                                } else if *optarg as std::os::raw::c_int == 'b' as i32 {
                                    x = 0x40 as std::os::raw::c_int
                                } else if *optarg as std::os::raw::c_int == 'n' as i32 {
                                    x = 0 as std::os::raw::c_int
                                } else {
                                    tcc_enter_state(s1);
                                    Some(
                                        _tcc_warning
                                            as unsafe extern "C" fn(
                                                _: *const std::os::raw::c_char,
                                                _: ...
                                            )
                                                -> (),
                                    )
                                    .expect("non-null function pointer")(
                                        b"unsupported language \'%s\'\x00" as *const u8
                                            as *const std::os::raw::c_char,
                                        optarg,
                                    );
                                }
                                (*s).filetype = (x
                                    | (*s).filetype as std::os::raw::c_int
                                        & !(15 as std::os::raw::c_int
                                            | 0x40 as std::os::raw::c_int))
                                    as std::os::raw::c_uchar;
                                continue 's_33;
                            },
                            29 => {
                                (*s).optimize = atoi(optarg) as std::os::raw::c_uchar;
                                continue 's_33;
                            },
                            38 => {
                                x = 4 as std::os::raw::c_int;
                                current_block = 10931776671445766452;
                                break;
                            },
                            51 => {
                                x = 6 as std::os::raw::c_int;
                                current_block = 10931776671445766452;
                                break;
                            },
                            50 => {
                                x = 5 as std::os::raw::c_int;
                                current_block = 10931776671445766452;
                                break;
                            },
                            25 | 41 | 45 | 24 | 52 => {
                                continue 's_33;
                            },
                            _ => {
                                current_block = 9902503279498310510;
                                break;
                            },
                        }
                    }
                }
                match current_block {
                    14851765859726653900 => {
                        if 8 as std::os::raw::c_int != x / 8 as std::os::raw::c_int {
                            return x;
                        }
                        noaction += 1;
                        continue;
                    },
                    5028470053297453708 => {
                        if *optarg as std::os::raw::c_int == 'M' as i32 {
                            (*s).dflag = 7 as std::os::raw::c_int as std::os::raw::c_char;
                            continue;
                        } else if *optarg as std::os::raw::c_int == 't' as i32 {
                            (*s).dflag = 16 as std::os::raw::c_int as std::os::raw::c_char;
                            continue;
                        } else if isnum(*optarg as std::os::raw::c_int) != 0 {
                            (*s).g_debug |= atoi(optarg);
                            continue;
                        }
                    },
                    10931776671445766452 => {
                        arg_start = optind - 1 as std::os::raw::c_int;
                        if arg_start != noaction {
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
                                b"cannot parse %s here\x00" as *const u8
                                    as *const std::os::raw::c_char,
                                r,
                            );
                        }
                        tool = x;
                        continue;
                    },
                    2823955538708594695 => {
                        if (*s).output_type != 0 {
                            tcc_enter_state(s1);
                            Some(
                                _tcc_warning
                                    as unsafe extern "C" fn(
                                        _: *const std::os::raw::c_char,
                                        _: ...
                                    )
                                        -> (),
                            )
                            .expect("non-null function pointer")(
                                b"-%s: overriding compiler action already specified\x00"
                                    as *const u8
                                    as *const std::os::raw::c_char,
                                (*popt).name,
                            );
                        }
                        (*s).output_type = x;
                        continue;
                    },
                    13660591889533726445 => {
                        (*s).dflag = 3 as std::os::raw::c_int as std::os::raw::c_char;
                        continue;
                    },
                    _ => {},
                }
                if (*s).warn_unsupported != 0 {
                    tcc_enter_state(s1);
                    Some(
                        _tcc_warning
                            as unsafe extern "C" fn(_: *const std::os::raw::c_char, _: ...) -> (),
                    )
                    .expect("non-null function pointer")(
                        b"unsupported option \'%s\'\x00" as *const u8
                            as *const std::os::raw::c_char,
                        r,
                    );
                }
            }
        }
    }
    match current_block {
        15514437232607373049 => {
            if linker_arg.size != 0 {
                r = linker_arg.data as *const std::os::raw::c_char
            } else {
                *pargc = argc - arg_start;
                *pargv = argv.offset(arg_start as isize);
                if tool != 0 {
                    return tool;
                }
                if optind != noaction {
                    return 0 as std::os::raw::c_int;
                }
                if (*s).verbose as std::os::raw::c_int == 2 as std::os::raw::c_int {
                    return 4 as std::os::raw::c_int;
                }
                if (*s).verbose != 0 {
                    return 3 as std::os::raw::c_int;
                }
                return 1 as std::os::raw::c_int;
            }
        },
        _ => {},
    }
    tcc_enter_state(s1);
    Some(_tcc_error as unsafe extern "C" fn(_: *const std::os::raw::c_char, _: ...) -> !)
        .expect("non-null function pointer")(
        b"argument to \'%s\' is missing\x00" as *const u8 as *const std::os::raw::c_char,
        r,
    );
}
#[no_mangle]
pub unsafe extern "C" fn tcc_set_options(mut s: *mut TCCState, mut r: *const std::os::raw::c_char) {
    let mut argv: *mut *mut std::os::raw::c_char = 0 as *mut *mut std::os::raw::c_char;
    let mut argc: std::os::raw::c_int = 0 as std::os::raw::c_int;
    args_parser_make_argv(r, &mut argc, &mut argv);
    tcc_parse_args(s, &mut argc, &mut argv, 0 as std::os::raw::c_int);
    dynarray_reset(
        &mut argv as *mut *mut *mut std::os::raw::c_char as *mut std::os::raw::c_void,
        &mut argc,
    );
}
#[no_mangle]
pub unsafe extern "C" fn tcc_print_stats(
    mut s1: *mut TCCState,
    mut total_time: std::os::raw::c_uint,
) {
    if total_time < 1 as std::os::raw::c_int as std::os::raw::c_uint {
        total_time = 1 as std::os::raw::c_int as std::os::raw::c_uint
    }
    if (*s1).total_bytes < 1 as std::os::raw::c_int {
        (*s1).total_bytes = 1 as std::os::raw::c_int
    }
    fprintf(
        stderr,
        b"* %d idents, %d lines, %d bytes\n* %0.3f s, %u lines/s, %0.1f MB/s\n\x00" as *const u8
            as *const std::os::raw::c_char,
        (*s1).total_idents,
        (*s1).total_lines,
        (*s1).total_bytes,
        total_time as std::os::raw::c_double
            / 1000 as std::os::raw::c_int as std::os::raw::c_double,
        ((*s1).total_lines as std::os::raw::c_uint)
            .wrapping_mul(1000 as std::os::raw::c_int as std::os::raw::c_uint)
            .wrapping_div(total_time),
        (*s1).total_bytes as std::os::raw::c_double
            / 1000 as std::os::raw::c_int as std::os::raw::c_double
            / total_time as std::os::raw::c_double,
    );
    fprintf(
        stderr,
        b"* text %d, data.rw %d, data.ro %d, bss %d bytes\n\x00" as *const u8
            as *const std::os::raw::c_char,
        (*s1).total_output[0 as std::os::raw::c_int as usize],
        (*s1).total_output[1 as std::os::raw::c_int as usize],
        (*s1).total_output[2 as std::os::raw::c_int as usize],
        (*s1).total_output[3 as std::os::raw::c_int as usize],
    );
}
unsafe extern "C" fn run_static_initializers() {
    options_W = [
        {
            let mut init = FlagDef {
                offset: 0 as std::os::raw::c_int as uint16_t,
                flags: 0 as std::os::raw::c_int as uint16_t,
                name: b"all\x00" as *const u8 as *const std::os::raw::c_char,
            };
            init
        },
        {
            let mut init = FlagDef {
                offset: &mut (*(0 as *mut TCCState)).warn_unsupported as *mut std::os::raw::c_uchar
                    as size_t as uint16_t,
                flags: 0 as std::os::raw::c_int as uint16_t,
                name: b"unsupported\x00" as *const u8 as *const std::os::raw::c_char,
            };
            init
        },
        {
            let mut init = FlagDef {
                offset: &mut (*(0 as *mut TCCState)).warn_write_strings
                    as *mut std::os::raw::c_uchar as size_t as uint16_t,
                flags: 0x1 as std::os::raw::c_int as uint16_t,
                name: b"write-strings\x00" as *const u8 as *const std::os::raw::c_char,
            };
            init
        },
        {
            let mut init = FlagDef {
                offset: &mut (*(0 as *mut TCCState)).warn_error as *mut std::os::raw::c_uchar
                    as size_t as uint16_t,
                flags: 0 as std::os::raw::c_int as uint16_t,
                name: b"error\x00" as *const u8 as *const std::os::raw::c_char,
            };
            init
        },
        {
            let mut init = FlagDef {
                offset: &mut (*(0 as *mut TCCState)).warn_gcc_compat as *mut std::os::raw::c_uchar
                    as size_t as uint16_t,
                flags: 0 as std::os::raw::c_int as uint16_t,
                name: b"gcc-compat\x00" as *const u8 as *const std::os::raw::c_char,
            };
            init
        },
        {
            let mut init = FlagDef {
                offset: &mut (*(0 as *mut TCCState)).warn_implicit_function_declaration
                    as *mut std::os::raw::c_uchar as size_t as uint16_t,
                flags: 0x1 as std::os::raw::c_int as uint16_t,
                name: b"implicit-function-declaration\x00" as *const u8
                    as *const std::os::raw::c_char,
            };
            init
        },
        {
            let mut init = FlagDef {
                offset: 0 as std::os::raw::c_int as uint16_t,
                flags: 0 as std::os::raw::c_int as uint16_t,
                name: 0 as *const std::os::raw::c_char,
            };
            init
        },
    ];
    options_f = [
        {
            let mut init = FlagDef {
                offset: &mut (*(0 as *mut TCCState)).char_is_unsigned as *mut std::os::raw::c_uchar
                    as size_t as uint16_t,
                flags: 0 as std::os::raw::c_int as uint16_t,
                name: b"unsigned-char\x00" as *const u8 as *const std::os::raw::c_char,
            };
            init
        },
        {
            let mut init = FlagDef {
                offset: &mut (*(0 as *mut TCCState)).char_is_unsigned as *mut std::os::raw::c_uchar
                    as size_t as uint16_t,
                flags: 0x2 as std::os::raw::c_int as uint16_t,
                name: b"signed-char\x00" as *const u8 as *const std::os::raw::c_char,
            };
            init
        },
        {
            let mut init = FlagDef {
                offset: &mut (*(0 as *mut TCCState)).nocommon as *mut std::os::raw::c_uchar
                    as size_t as uint16_t,
                flags: 0x2 as std::os::raw::c_int as uint16_t,
                name: b"common\x00" as *const u8 as *const std::os::raw::c_char,
            };
            init
        },
        {
            let mut init = FlagDef {
                offset: &mut (*(0 as *mut TCCState)).leading_underscore
                    as *mut std::os::raw::c_uchar as size_t as uint16_t,
                flags: 0 as std::os::raw::c_int as uint16_t,
                name: b"leading-underscore\x00" as *const u8 as *const std::os::raw::c_char,
            };
            init
        },
        {
            let mut init = FlagDef {
                offset: &mut (*(0 as *mut TCCState)).ms_extensions as *mut std::os::raw::c_uchar
                    as size_t as uint16_t,
                flags: 0 as std::os::raw::c_int as uint16_t,
                name: b"ms-extensions\x00" as *const u8 as *const std::os::raw::c_char,
            };
            init
        },
        {
            let mut init = FlagDef {
                offset: &mut (*(0 as *mut TCCState)).dollars_in_identifiers
                    as *mut std::os::raw::c_uchar as size_t as uint16_t,
                flags: 0 as std::os::raw::c_int as uint16_t,
                name: b"dollars-in-identifiers\x00" as *const u8 as *const std::os::raw::c_char,
            };
            init
        },
        {
            let mut init = FlagDef {
                offset: &mut (*(0 as *mut TCCState)).test_coverage as *mut std::os::raw::c_uchar
                    as size_t as uint16_t,
                flags: 0 as std::os::raw::c_int as uint16_t,
                name: b"test-coverage\x00" as *const u8 as *const std::os::raw::c_char,
            };
            init
        },
        {
            let mut init = FlagDef {
                offset: 0 as std::os::raw::c_int as uint16_t,
                flags: 0 as std::os::raw::c_int as uint16_t,
                name: 0 as *const std::os::raw::c_char,
            };
            init
        },
    ];
    options_m = [
        {
            let mut init = FlagDef {
                offset: &mut (*(0 as *mut TCCState)).ms_bitfields as *mut std::os::raw::c_uchar
                    as size_t as uint16_t,
                flags: 0 as std::os::raw::c_int as uint16_t,
                name: b"ms-bitfields\x00" as *const u8 as *const std::os::raw::c_char,
            };
            init
        },
        {
            let mut init = FlagDef {
                offset: &mut (*(0 as *mut TCCState)).nosse as *mut std::os::raw::c_uchar as size_t
                    as uint16_t,
                flags: 0x2 as std::os::raw::c_int as uint16_t,
                name: b"sse\x00" as *const u8 as *const std::os::raw::c_char,
            };
            init
        },
        {
            let mut init = FlagDef {
                offset: 0 as std::os::raw::c_int as uint16_t,
                flags: 0 as std::os::raw::c_int as uint16_t,
                name: 0 as *const std::os::raw::c_char,
            };
            init
        },
    ]
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
