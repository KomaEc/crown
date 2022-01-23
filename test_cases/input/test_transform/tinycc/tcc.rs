use crate::bitfields::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type sym_version;
    #[no_mangle]
    fn strchr(_: *const std::os::raw::c_char, _: std::os::raw::c_int) -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn strcmp(
        _: *const std::os::raw::c_char,
        _: *const std::os::raw::c_char,
    ) -> std::os::raw::c_int;
    #[no_mangle]
    fn strstr(
        _: *const std::os::raw::c_char,
        _: *const std::os::raw::c_char,
    ) -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn strlen(_: *const std::os::raw::c_char) -> std::os::raw::c_ulong;
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
    fn fputs(__s: *const std::os::raw::c_char, __stream: *mut FILE) -> std::os::raw::c_int;
    #[no_mangle]
    fn ftell(__stream: *mut FILE) -> std::os::raw::c_long;
    #[no_mangle]
    fn fseek(
        __stream: *mut FILE,
        __off: std::os::raw::c_long,
        __whence: std::os::raw::c_int,
    ) -> std::os::raw::c_int;
    #[no_mangle]
    fn fwrite(
        _: *const std::os::raw::c_void,
        _: std::os::raw::c_ulong,
        _: std::os::raw::c_ulong,
        _: *mut FILE,
    ) -> std::os::raw::c_ulong;
    #[no_mangle]
    fn fread(
        _: *mut std::os::raw::c_void,
        _: std::os::raw::c_ulong,
        _: std::os::raw::c_ulong,
        _: *mut FILE,
    ) -> std::os::raw::c_ulong;
    #[no_mangle]
    fn fopen(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char) -> *mut FILE;
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> std::os::raw::c_int;
    #[no_mangle]
    fn remove(__filename: *const std::os::raw::c_char) -> std::os::raw::c_int;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    static mut stdout: *mut FILE;
    #[no_mangle]
    fn getenv(__name: *const std::os::raw::c_char) -> *mut std::os::raw::c_char;
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
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    fn execvp(
        __file: *const std::os::raw::c_char,
        __argv: *const *mut std::os::raw::c_char,
    ) -> std::os::raw::c_int;
    #[no_mangle]
    fn gettimeofday(__tv: *mut timeval, __tz: *mut std::os::raw::c_void) -> std::os::raw::c_int;
    #[no_mangle]
    fn tcc_new() -> *mut TCCState;
    #[no_mangle]
    fn tcc_delete(s: *mut TCCState);
    #[no_mangle]
    fn tcc_add_include_path(
        s: *mut TCCState,
        pathname: *const std::os::raw::c_char,
    ) -> std::os::raw::c_int;
    #[no_mangle]
    fn tcc_add_sysinclude_path(
        s: *mut TCCState,
        pathname: *const std::os::raw::c_char,
    ) -> std::os::raw::c_int;
    #[no_mangle]
    fn tcc_add_file(s: *mut TCCState, filename: *const std::os::raw::c_char)
    -> std::os::raw::c_int;
    #[no_mangle]
    fn tcc_set_output_type(
        s: *mut TCCState,
        output_type: std::os::raw::c_int,
    ) -> std::os::raw::c_int;
    #[no_mangle]
    fn tcc_add_library_path(
        s: *mut TCCState,
        pathname: *const std::os::raw::c_char,
    ) -> std::os::raw::c_int;
    #[no_mangle]
    fn tcc_output_file(
        s: *mut TCCState,
        filename: *const std::os::raw::c_char,
    ) -> std::os::raw::c_int;
    #[no_mangle]
    fn tcc_run(
        s: *mut TCCState,
        argc: std::os::raw::c_int,
        argv: *mut *mut std::os::raw::c_char,
    ) -> std::os::raw::c_int;
    #[no_mangle]
    fn tcc_basename(name: *const std::os::raw::c_char) -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn tcc_fileextension(name: *const std::os::raw::c_char) -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn tcc_free(ptr: *mut std::os::raw::c_void);
    #[no_mangle]
    fn tcc_malloc(size: std::os::raw::c_ulong) -> *mut std::os::raw::c_void;
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
    fn tcc_add_library_err(s: *mut TCCState, f: *const std::os::raw::c_char)
    -> std::os::raw::c_int;
    #[no_mangle]
    fn tcc_print_stats(s: *mut TCCState, total_time: std::os::raw::c_uint);
    #[no_mangle]
    fn tcc_parse_args(
        s: *mut TCCState,
        argc: *mut std::os::raw::c_int,
        argv: *mut *mut *mut std::os::raw::c_char,
        optind: std::os::raw::c_int,
    ) -> std::os::raw::c_int;
    #[no_mangle]
    fn tcc_enter_state(s1: *mut TCCState);
}
pub type size_t = std::os::raw::c_ulong;
pub type __uint8_t = std::os::raw::c_uchar;
pub type __uint16_t = std::os::raw::c_ushort;
pub type __uint32_t = std::os::raw::c_uint;
pub type __int64_t = std::os::raw::c_long;
pub type __uint64_t = std::os::raw::c_ulong;
pub type __off_t = std::os::raw::c_long;
pub type __off64_t = std::os::raw::c_long;
pub type __time_t = std::os::raw::c_long;
pub type __suseconds_t = std::os::raw::c_long;
pub type int64_t = __int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [std::os::raw::c_ulong; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
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
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type Elf64_Half = uint16_t;
pub type Elf64_Word = uint32_t;
pub type Elf64_Off = uint64_t;
pub type Elf64_Section = uint16_t;
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
pub struct Elf64_Sym {
    pub st_name: Elf64_Word,
    pub st_info: std::os::raw::c_uchar,
    pub st_other: std::os::raw::c_uchar,
    pub st_shndx: Elf64_Section,
    pub st_value: Elf64_Addr,
    pub st_size: Elf64_Xword,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ArHdr {
    pub ar_name: [std::os::raw::c_char; 16],
    pub ar_date: [std::os::raw::c_char; 12],
    pub ar_uid: [std::os::raw::c_char; 6],
    pub ar_gid: [std::os::raw::c_char; 6],
    pub ar_mode: [std::os::raw::c_char; 8],
    pub ar_size: [std::os::raw::c_char; 10],
    pub ar_fmag: [std::os::raw::c_char; 2],
}
#[inline]
unsafe extern "C" fn is_space(mut ch: std::os::raw::c_int) -> std::os::raw::c_int {
    return (ch == ' ' as i32
        || ch == '\t' as i32
        || ch == '\u{b}' as i32
        || ch == '\u{c}' as i32
        || ch == '\r' as i32) as std::os::raw::c_int;
}
unsafe extern "C" fn le2belong(mut ul: std::os::raw::c_ulong) -> std::os::raw::c_ulong {
    return ((ul & 0xff0000 as std::os::raw::c_int as std::os::raw::c_ulong)
        >> 8 as std::os::raw::c_int)
        .wrapping_add(
            (ul & 0xff000000 as std::os::raw::c_uint as std::os::raw::c_ulong)
                >> 24 as std::os::raw::c_int,
        )
        .wrapping_add(
            (ul & 0xff as std::os::raw::c_int as std::os::raw::c_ulong)
                << 24 as std::os::raw::c_int,
        )
        .wrapping_add(
            (ul & 0xff00 as std::os::raw::c_int as std::os::raw::c_ulong)
                << 8 as std::os::raw::c_int,
        );
}
unsafe extern "C" fn contains_any(
    mut s: *const std::os::raw::c_char,
    mut list: *const std::os::raw::c_char,
) -> std::os::raw::c_int {
    let mut l: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    while *s != 0 {
        l = list;
        while *l != 0 {
            if *s as std::os::raw::c_int == *l as std::os::raw::c_int {
                return 1 as std::os::raw::c_int;
            }
            l = l.offset(1)
        }
        s = s.offset(1)
    }
    return 0 as std::os::raw::c_int;
}
unsafe extern "C" fn ar_usage(mut ret: std::os::raw::c_int) -> std::os::raw::c_int {
    fprintf(
        stderr,
        b"usage: tcc -ar [rcsv] lib file...\n\x00" as *const u8 as *const std::os::raw::c_char,
    );
    fprintf(
        stderr,
        b"create library ([abdioptxN] not supported).\n\x00" as *const u8
            as *const std::os::raw::c_char,
    );
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn tcc_tool_ar(
    mut s1: *mut TCCState,
    mut argc: std::os::raw::c_int,
    mut argv: *mut *mut std::os::raw::c_char,
) -> std::os::raw::c_int {
    let mut current_block: u64;
    static mut arhdr_init: ArHdr = unsafe {
        {
            let mut init = ArHdr {
                ar_name: *::std::mem::transmute::<&[u8; 16], &mut [std::os::raw::c_char; 16]>(
                    b"/               ",
                ),
                ar_date: *::std::mem::transmute::<&[u8; 12], &mut [std::os::raw::c_char; 12]>(
                    b"            ",
                ),
                ar_uid: *::std::mem::transmute::<&[u8; 6], &mut [std::os::raw::c_char; 6]>(
                    b"0     ",
                ),
                ar_gid: *::std::mem::transmute::<&[u8; 6], &mut [std::os::raw::c_char; 6]>(
                    b"0     ",
                ),
                ar_mode: *::std::mem::transmute::<&[u8; 8], &mut [std::os::raw::c_char; 8]>(
                    b"0       ",
                ),
                ar_size: *::std::mem::transmute::<&[u8; 10], &mut [std::os::raw::c_char; 10]>(
                    b"          ",
                ),
                ar_fmag: *::std::mem::transmute::<&[u8; 2], &mut [std::os::raw::c_char; 2]>(b"`\n"),
            };
            init
        }
    };
    let mut arhdr: ArHdr = arhdr_init;
    let mut arhdro: ArHdr = arhdr_init;
    let mut fi: *mut FILE = 0 as *mut FILE;
    let mut fh: *mut FILE = 0 as *mut FILE;
    let mut fo: *mut FILE = 0 as *mut FILE;
    let mut ehdr: *mut Elf64_Ehdr = 0 as *mut Elf64_Ehdr;
    let mut shdr: *mut Elf64_Shdr = 0 as *mut Elf64_Shdr;
    let mut sym: *mut Elf64_Sym = 0 as *mut Elf64_Sym;
    let mut i: std::os::raw::c_int = 0;
    let mut fsize: std::os::raw::c_int = 0;
    let mut i_lib: std::os::raw::c_int = 0;
    let mut i_obj: std::os::raw::c_int = 0;
    let mut buf: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut shstr: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut symtab: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut strtab: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut symtabsize: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut anames: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut afpos: *mut std::os::raw::c_int = 0 as *mut std::os::raw::c_int;
    let mut istrlen: std::os::raw::c_int = 0;
    let mut strpos: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut fpos: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut funccnt: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut funcmax: std::os::raw::c_int = 0;
    let mut hofs: std::os::raw::c_int = 0;
    let mut tfile: [std::os::raw::c_char; 260] = [0; 260];
    let mut stmp: [std::os::raw::c_char; 20] = [0; 20];
    let mut file: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut name: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut ret: std::os::raw::c_int = 2 as std::os::raw::c_int;
    let mut ops_conflict: *const std::os::raw::c_char =
        b"habdioptxN\x00" as *const u8 as *const std::os::raw::c_char;
    let mut verbose: std::os::raw::c_int = 0 as std::os::raw::c_int;
    i_lib = 0 as std::os::raw::c_int;
    i_obj = 0 as std::os::raw::c_int;
    i = 1 as std::os::raw::c_int;
    while i < argc {
        let mut a: *const std::os::raw::c_char = *argv.offset(i as isize);
        if *a as std::os::raw::c_int == '-' as i32
            && !strstr(a, b".\x00" as *const u8 as *const std::os::raw::c_char).is_null()
        {
            ret = 1 as std::os::raw::c_int
        }
        if *a as std::os::raw::c_int == '-' as i32
            || i == 1 as std::os::raw::c_int
                && strstr(a, b".\x00" as *const u8 as *const std::os::raw::c_char).is_null()
        {
            if contains_any(a, ops_conflict) != 0 {
                ret = 1 as std::os::raw::c_int
            }
            if !strstr(a, b"v\x00" as *const u8 as *const std::os::raw::c_char).is_null() {
                verbose = 1 as std::os::raw::c_int
            }
        } else if i_lib == 0 {
            i_lib = i
        } else if i_obj == 0 {
            i_obj = i
        }
        i += 1
    }
    if i_obj == 0 {
        ret = 1 as std::os::raw::c_int
    }
    if ret == 1 as std::os::raw::c_int {
        return ar_usage(ret);
    }
    fh = fopen(
        *argv.offset(i_lib as isize),
        b"wb\x00" as *const u8 as *const std::os::raw::c_char,
    );
    if fh.is_null() {
        fprintf(
            stderr,
            b"tcc: ar: can\'t open file %s \n\x00" as *const u8 as *const std::os::raw::c_char,
            *argv.offset(i_lib as isize),
        );
    } else {
        sprintf(
            tfile.as_mut_ptr(),
            b"%s.tmp\x00" as *const u8 as *const std::os::raw::c_char,
            *argv.offset(i_lib as isize),
        );
        fo = fopen(
            tfile.as_mut_ptr(),
            b"wb+\x00" as *const u8 as *const std::os::raw::c_char,
        );
        if fo.is_null() {
            fprintf(
                stderr,
                b"tcc: ar: can\'t create temporary file %s\n\x00" as *const u8
                    as *const std::os::raw::c_char,
                tfile.as_mut_ptr(),
            );
        } else {
            funcmax = 250 as std::os::raw::c_int;
            afpos = tcc_realloc(
                0 as *mut std::os::raw::c_void,
                (funcmax as std::os::raw::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong
                    ),
            ) as *mut std::os::raw::c_int;
            memcpy(
                &mut arhdro.ar_mode as *mut [std::os::raw::c_char; 8] as *mut std::os::raw::c_void,
                b"100666\x00" as *const u8 as *const std::os::raw::c_char
                    as *const std::os::raw::c_void,
                6 as std::os::raw::c_int as std::os::raw::c_ulong,
            );
            loop {
                if !(i_obj < argc) {
                    current_block = 5706507068631705000;
                    break;
                }
                if **argv.offset(i_obj as isize) as std::os::raw::c_int == '-' as i32 {
                    i_obj += 1
                } else {
                    fi = fopen(
                        *argv.offset(i_obj as isize),
                        b"rb\x00" as *const u8 as *const std::os::raw::c_char,
                    );
                    if fi.is_null() {
                        fprintf(
                            stderr,
                            b"tcc: ar: can\'t open file %s \n\x00" as *const u8
                                as *const std::os::raw::c_char,
                            *argv.offset(i_obj as isize),
                        );
                        current_block = 295062140193088423;
                        break;
                    } else {
                        if verbose != 0 {
                            printf(
                                b"a - %s\n\x00" as *const u8 as *const std::os::raw::c_char,
                                *argv.offset(i_obj as isize),
                            );
                        }
                        fseek(
                            fi,
                            0 as std::os::raw::c_int as std::os::raw::c_long,
                            2 as std::os::raw::c_int,
                        );
                        fsize = ftell(fi) as std::os::raw::c_int;
                        fseek(
                            fi,
                            0 as std::os::raw::c_int as std::os::raw::c_long,
                            0 as std::os::raw::c_int,
                        );
                        buf =
                            tcc_malloc((fsize + 1 as std::os::raw::c_int) as std::os::raw::c_ulong)
                                as *mut std::os::raw::c_char;
                        fread(
                            buf as *mut std::os::raw::c_void,
                            fsize as std::os::raw::c_ulong,
                            1 as std::os::raw::c_int as std::os::raw::c_ulong,
                            fi,
                        );
                        fclose(fi);
                        ehdr = buf as *mut Elf64_Ehdr;
                        if (*ehdr).e_ident[4 as std::os::raw::c_int as usize] as std::os::raw::c_int
                            != 2 as std::os::raw::c_int
                        {
                            fprintf(
                                stderr,
                                b"tcc: ar: Unsupported Elf Class: %s\n\x00" as *const u8
                                    as *const std::os::raw::c_char,
                                *argv.offset(i_obj as isize),
                            );
                            current_block = 295062140193088423;
                            break;
                        } else {
                            shdr = buf.offset((*ehdr).e_shoff as isize).offset(
                                ((*ehdr).e_shstrndx as std::os::raw::c_int
                                    * (*ehdr).e_shentsize as std::os::raw::c_int)
                                    as isize,
                            ) as *mut Elf64_Shdr;
                            shstr = buf.offset((*shdr).sh_offset as isize);
                            i = 0 as std::os::raw::c_int;
                            while i < (*ehdr).e_shnum as std::os::raw::c_int {
                                shdr = buf.offset((*ehdr).e_shoff as isize).offset(
                                    (i * (*ehdr).e_shentsize as std::os::raw::c_int) as isize,
                                ) as *mut Elf64_Shdr;
                                if !((*shdr).sh_offset == 0) {
                                    if (*shdr).sh_type
                                        == 2 as std::os::raw::c_int as std::os::raw::c_uint
                                    {
                                        symtab = buf.offset((*shdr).sh_offset as isize);
                                        symtabsize = (*shdr).sh_size as std::os::raw::c_int
                                    }
                                    if (*shdr).sh_type
                                        == 3 as std::os::raw::c_int as std::os::raw::c_uint
                                    {
                                        if strcmp(
                                            shstr.offset((*shdr).sh_name as isize),
                                            b".strtab\x00" as *const u8
                                                as *const std::os::raw::c_char,
                                        ) == 0
                                        {
                                            strtab = buf.offset((*shdr).sh_offset as isize)
                                        }
                                    }
                                }
                                i += 1
                            }
                            if !symtab.is_null() && symtabsize != 0 {
                                let mut nsym: std::os::raw::c_int =
                                    (symtabsize as std::os::raw::c_ulong)
                                        .wrapping_div(::std::mem::size_of::<Elf64_Sym>()
                                            as std::os::raw::c_ulong)
                                        as std::os::raw::c_int;
                                i = 1 as std::os::raw::c_int;
                                while i < nsym {
                                    sym = symtab.offset(
                                        (i as std::os::raw::c_ulong)
                                            .wrapping_mul(::std::mem::size_of::<Elf64_Sym>()
                                                as std::os::raw::c_ulong)
                                            as isize,
                                    ) as *mut Elf64_Sym;
                                    if (*sym).st_shndx as std::os::raw::c_int != 0
                                        && ((*sym).st_info as std::os::raw::c_int
                                            == 0x10 as std::os::raw::c_int
                                            || (*sym).st_info as std::os::raw::c_int
                                                == 0x11 as std::os::raw::c_int
                                            || (*sym).st_info as std::os::raw::c_int
                                                == 0x12 as std::os::raw::c_int)
                                    {
                                        istrlen = strlen(strtab.offset((*sym).st_name as isize))
                                            .wrapping_add(
                                                1 as std::os::raw::c_int as std::os::raw::c_ulong,
                                            )
                                            as std::os::raw::c_int;
                                        anames = tcc_realloc(
                                            anames as *mut std::os::raw::c_void,
                                            (strpos + istrlen) as std::os::raw::c_ulong,
                                        )
                                            as *mut std::os::raw::c_char;
                                        strcpy(
                                            anames.offset(strpos as isize),
                                            strtab.offset((*sym).st_name as isize),
                                        );
                                        strpos += istrlen;
                                        funccnt += 1;
                                        if funccnt >= funcmax {
                                            funcmax += 250 as std::os::raw::c_int;
                                            afpos = tcc_realloc(
                                                afpos as *mut std::os::raw::c_void,
                                                (funcmax as std::os::raw::c_ulong).wrapping_mul(
                                                    ::std::mem::size_of::<std::os::raw::c_int>()
                                                        as std::os::raw::c_ulong,
                                                ),
                                            )
                                                as *mut std::os::raw::c_int
                                        }
                                        *afpos.offset(funccnt as isize) = fpos
                                    }
                                    i += 1
                                }
                            }
                            file = *argv.offset(i_obj as isize);
                            name = strchr(file, 0 as std::os::raw::c_int);
                            while name > file
                                && *name.offset(-(1 as std::os::raw::c_int) as isize)
                                    as std::os::raw::c_int
                                    != '/' as i32
                                && *name.offset(-(1 as std::os::raw::c_int) as isize)
                                    as std::os::raw::c_int
                                    != '\\' as i32
                            {
                                name = name.offset(-1)
                            }
                            istrlen = strlen(name) as std::os::raw::c_int;
                            if istrlen as std::os::raw::c_ulong
                                >= ::std::mem::size_of::<[std::os::raw::c_char; 16]>()
                                    as std::os::raw::c_ulong
                            {
                                istrlen = (::std::mem::size_of::<[std::os::raw::c_char; 16]>()
                                    as std::os::raw::c_ulong)
                                    .wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong)
                                    as std::os::raw::c_int
                            }
                            memset(
                                arhdro.ar_name.as_mut_ptr() as *mut std::os::raw::c_void,
                                ' ' as i32,
                                ::std::mem::size_of::<[std::os::raw::c_char; 16]>()
                                    as std::os::raw::c_ulong,
                            );
                            memcpy(
                                arhdro.ar_name.as_mut_ptr() as *mut std::os::raw::c_void,
                                name as *const std::os::raw::c_void,
                                istrlen as std::os::raw::c_ulong,
                            );
                            arhdro.ar_name[istrlen as usize] = '/' as i32 as std::os::raw::c_char;
                            sprintf(
                                stmp.as_mut_ptr(),
                                b"%-10d\x00" as *const u8 as *const std::os::raw::c_char,
                                fsize,
                            );
                            memcpy(
                                &mut arhdro.ar_size as *mut [std::os::raw::c_char; 10]
                                    as *mut std::os::raw::c_void,
                                stmp.as_mut_ptr() as *const std::os::raw::c_void,
                                10 as std::os::raw::c_int as std::os::raw::c_ulong,
                            );
                            fwrite(
                                &mut arhdro as *mut ArHdr as *const std::os::raw::c_void,
                                ::std::mem::size_of::<ArHdr>() as std::os::raw::c_ulong,
                                1 as std::os::raw::c_int as std::os::raw::c_ulong,
                                fo,
                            );
                            fwrite(
                                buf as *const std::os::raw::c_void,
                                fsize as std::os::raw::c_ulong,
                                1 as std::os::raw::c_int as std::os::raw::c_ulong,
                                fo,
                            );
                            tcc_free(buf as *mut std::os::raw::c_void);
                            i_obj += 1;
                            fpos = (fpos as std::os::raw::c_ulong).wrapping_add(
                                (fsize as std::os::raw::c_ulong).wrapping_add(
                                    ::std::mem::size_of::<ArHdr>() as std::os::raw::c_ulong,
                                ),
                            ) as std::os::raw::c_int
                                as std::os::raw::c_int
                        }
                    }
                }
            }
            match current_block {
                295062140193088423 => {},
                _ => {
                    hofs = (8 as std::os::raw::c_int as std::os::raw::c_ulong)
                        .wrapping_add(::std::mem::size_of::<ArHdr>() as std::os::raw::c_ulong)
                        .wrapping_add(strpos as std::os::raw::c_ulong)
                        .wrapping_add(
                            ((funccnt + 1 as std::os::raw::c_int) as std::os::raw::c_ulong)
                                .wrapping_mul(::std::mem::size_of::<std::os::raw::c_int>()
                                    as std::os::raw::c_ulong),
                        ) as std::os::raw::c_int;
                    fpos = 0 as std::os::raw::c_int;
                    if hofs & 1 as std::os::raw::c_int != 0 {
                        hofs += 1;
                        fpos = 1 as std::os::raw::c_int
                    }
                    fwrite(
                        b"!<arch>\n\x00" as *const u8 as *const std::os::raw::c_char
                            as *const std::os::raw::c_void,
                        8 as std::os::raw::c_int as std::os::raw::c_ulong,
                        1 as std::os::raw::c_int as std::os::raw::c_ulong,
                        fh,
                    );
                    sprintf(
                        stmp.as_mut_ptr(),
                        b"%-10d\x00" as *const u8 as *const std::os::raw::c_char,
                        (strpos as std::os::raw::c_ulong).wrapping_add(
                            ((funccnt + 1 as std::os::raw::c_int) as std::os::raw::c_ulong)
                                .wrapping_mul(::std::mem::size_of::<std::os::raw::c_int>()
                                    as std::os::raw::c_ulong),
                        ) as std::os::raw::c_int,
                    );
                    memcpy(
                        &mut arhdr.ar_size as *mut [std::os::raw::c_char; 10]
                            as *mut std::os::raw::c_void,
                        stmp.as_mut_ptr() as *const std::os::raw::c_void,
                        10 as std::os::raw::c_int as std::os::raw::c_ulong,
                    );
                    fwrite(
                        &mut arhdr as *mut ArHdr as *const std::os::raw::c_void,
                        ::std::mem::size_of::<ArHdr>() as std::os::raw::c_ulong,
                        1 as std::os::raw::c_int as std::os::raw::c_ulong,
                        fh,
                    );
                    *afpos.offset(0 as std::os::raw::c_int as isize) =
                        le2belong(funccnt as std::os::raw::c_ulong) as std::os::raw::c_int;
                    i = 1 as std::os::raw::c_int;
                    while i <= funccnt {
                        *afpos.offset(i as isize) =
                            le2belong((*afpos.offset(i as isize) + hofs) as std::os::raw::c_ulong)
                                as std::os::raw::c_int;
                        i += 1
                    }
                    fwrite(
                        afpos as *const std::os::raw::c_void,
                        ((funccnt + 1 as std::os::raw::c_int) as std::os::raw::c_ulong)
                            .wrapping_mul(::std::mem::size_of::<std::os::raw::c_int>()
                                as std::os::raw::c_ulong),
                        1 as std::os::raw::c_int as std::os::raw::c_ulong,
                        fh,
                    );
                    fwrite(
                        anames as *const std::os::raw::c_void,
                        strpos as std::os::raw::c_ulong,
                        1 as std::os::raw::c_int as std::os::raw::c_ulong,
                        fh,
                    );
                    if fpos != 0 {
                        fwrite(
                            b"\x00" as *const u8 as *const std::os::raw::c_char
                                as *const std::os::raw::c_void,
                            1 as std::os::raw::c_int as std::os::raw::c_ulong,
                            1 as std::os::raw::c_int as std::os::raw::c_ulong,
                            fh,
                        );
                    }
                    fseek(
                        fo,
                        0 as std::os::raw::c_int as std::os::raw::c_long,
                        2 as std::os::raw::c_int,
                    );
                    fsize = ftell(fo) as std::os::raw::c_int;
                    fseek(
                        fo,
                        0 as std::os::raw::c_int as std::os::raw::c_long,
                        0 as std::os::raw::c_int,
                    );
                    buf = tcc_malloc((fsize + 1 as std::os::raw::c_int) as std::os::raw::c_ulong)
                        as *mut std::os::raw::c_char;
                    fread(
                        buf as *mut std::os::raw::c_void,
                        fsize as std::os::raw::c_ulong,
                        1 as std::os::raw::c_int as std::os::raw::c_ulong,
                        fo,
                    );
                    fwrite(
                        buf as *const std::os::raw::c_void,
                        fsize as std::os::raw::c_ulong,
                        1 as std::os::raw::c_int as std::os::raw::c_ulong,
                        fh,
                    );
                    tcc_free(buf as *mut std::os::raw::c_void);
                    ret = 0 as std::os::raw::c_int
                },
            }
        }
    }
    if !anames.is_null() {
        tcc_free(anames as *mut std::os::raw::c_void);
    }
    if !afpos.is_null() {
        tcc_free(afpos as *mut std::os::raw::c_void);
    }
    if !fh.is_null() {
        fclose(fh);
    }
    if !fo.is_null() {
        fclose(fo);
        remove(tfile.as_mut_ptr());
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn tcc_tool_cross(
    mut s1: *mut TCCState,
    mut argv: *mut *mut std::os::raw::c_char,
    mut target: std::os::raw::c_int,
) {
    let mut program: [std::os::raw::c_char; 4096] = [0; 4096];
    let mut a0: *mut std::os::raw::c_char = *argv.offset(0 as std::os::raw::c_int as isize);
    let mut prefix: std::os::raw::c_int =
        tcc_basename(a0).offset_from(a0) as std::os::raw::c_long as std::os::raw::c_int;
    snprintf(
        program.as_mut_ptr(),
        ::std::mem::size_of::<[std::os::raw::c_char; 4096]>() as std::os::raw::c_ulong,
        b"%.*s%s-tcc\x00" as *const u8 as *const std::os::raw::c_char,
        prefix,
        a0,
        if target == 64 as std::os::raw::c_int {
            b"x86_64\x00" as *const u8 as *const std::os::raw::c_char
        } else {
            b"i386\x00" as *const u8 as *const std::os::raw::c_char
        },
    );
    if strcmp(a0, program.as_mut_ptr()) != 0 {
        let ref mut fresh0 = *argv.offset(0 as std::os::raw::c_int as isize);
        *fresh0 = program.as_mut_ptr();
        execvp(*fresh0, argv as *const *mut std::os::raw::c_char);
    }
    tcc_enter_state(s1);
    Some(_tcc_error as unsafe extern "C" fn(_: *const std::os::raw::c_char, _: ...) -> !)
        .expect("non-null function pointer")(
        b"could not run \'%s\'\x00" as *const u8 as *const std::os::raw::c_char,
        program.as_mut_ptr(),
    );
}
unsafe extern "C" fn escape_target_dep(
    mut s: *const std::os::raw::c_char,
) -> *mut std::os::raw::c_char {
    let mut res: *mut std::os::raw::c_char = tcc_malloc(
        strlen(s)
            .wrapping_mul(2 as std::os::raw::c_int as std::os::raw::c_ulong)
            .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong),
    ) as *mut std::os::raw::c_char;
    let mut j: std::os::raw::c_int = 0;
    j = 0 as std::os::raw::c_int;
    while *s != 0 {
        if is_space(*s as std::os::raw::c_int) != 0 {
            let fresh1 = j;
            j = j + 1;
            *res.offset(fresh1 as isize) = '\\' as i32 as std::os::raw::c_char
        }
        *res.offset(j as isize) = *s;
        s = s.offset(1);
        j += 1
    }
    *res.offset(j as isize) = '\u{0}' as i32 as std::os::raw::c_char;
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn gen_makedeps(
    mut s1: *mut TCCState,
    mut target: *const std::os::raw::c_char,
    mut filename: *const std::os::raw::c_char,
) {
    let mut depout: *mut FILE = 0 as *mut FILE;
    let mut buf: [std::os::raw::c_char; 1024] = [0; 1024];
    let mut escaped_target: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut i: std::os::raw::c_int = 0;
    let mut k: std::os::raw::c_int = 0;
    if filename.is_null() {
        snprintf(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[std::os::raw::c_char; 1024]>() as std::os::raw::c_ulong,
            b"%.*s.d\x00" as *const u8 as *const std::os::raw::c_char,
            tcc_fileextension(target).offset_from(target) as std::os::raw::c_long
                as std::os::raw::c_int,
            target,
        );
        filename = buf.as_mut_ptr()
    }
    if (*s1).verbose != 0 {
        printf(
            b"<- %s\n\x00" as *const u8 as *const std::os::raw::c_char,
            filename,
        );
    }
    depout = fopen(
        filename,
        b"w\x00" as *const u8 as *const std::os::raw::c_char,
    );
    if depout.is_null() {
        tcc_enter_state(s1);
        Some(_tcc_error as unsafe extern "C" fn(_: *const std::os::raw::c_char, _: ...) -> !)
            .expect("non-null function pointer")(
            b"could not open \'%s\'\x00" as *const u8 as *const std::os::raw::c_char,
            filename,
        );
    }
    fprintf(
        depout,
        b"%s:\x00" as *const u8 as *const std::os::raw::c_char,
        target,
    );
    i = 0 as std::os::raw::c_int;
    while i < (*s1).nb_target_deps {
        let mut current_block_15: u64;
        k = 0 as std::os::raw::c_int;
        loop {
            if !(k < i) {
                current_block_15 = 9606288038608642794;
                break;
            }
            if 0 as std::os::raw::c_int
                == strcmp(
                    *(*s1).target_deps.offset(i as isize),
                    *(*s1).target_deps.offset(k as isize),
                )
            {
                current_block_15 = 13242334135786603907;
                break;
            }
            k += 1
        }
        match current_block_15 {
            9606288038608642794 => {
                escaped_target = escape_target_dep(*(*s1).target_deps.offset(i as isize));
                fprintf(
                    depout,
                    b" \\\n  %s\x00" as *const u8 as *const std::os::raw::c_char,
                    escaped_target,
                );
                tcc_free(escaped_target as *mut std::os::raw::c_void);
            },
            _ => {},
        }
        i += 1
    }
    fprintf(
        depout,
        b"\n\x00" as *const u8 as *const std::os::raw::c_char,
    );
    fclose(depout);
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
static mut help: [std::os::raw::c_char; 2103] = unsafe {
    *::std::mem::transmute::<&[u8; 2103],
                                 &[std::os::raw::c_char; 2103]>(b"Tiny C Compiler 0.9.27 - Copyright (C) 2001-2006 Fabrice Bellard\nUsage: tcc [options...] [-o outfile] [-c] infile(s)...\n       tcc [options...] -run infile [arguments...]\nGeneral options:\n  -c           compile only - generate an object file\n  -o outfile   set output filename\n  -run         run compiled source\n  -fflag       set or reset (with \'no-\' prefix) \'flag\' (see tcc -hh)\n  -std=c99     Conform to the ISO 1999 C standard (default).\n  -std=c11     Conform to the ISO 2011 C standard.\n  -Wwarning    set or reset (with \'no-\' prefix) \'warning\' (see tcc -hh)\n  -w           disable all warnings\n  --version -v show version\n  -vv          show search paths or loaded files\n  -h -hh       show this, show more help\n  -bench       show compilation statistics\n  -            use stdin pipe as infile\n  @listfile    read arguments from listfile\nPreprocessor options:\n  -Idir        add include path \'dir\'\n  -Dsym[=val]  define \'sym\' with value \'val\'\n  -Usym        undefine \'sym\'\n  -E           preprocess only\n  -C           keep comments (not yet implemented)\nLinker options:\n  -Ldir        add library path \'dir\'\n  -llib        link with dynamic or static library \'lib\'\n  -r           generate (relocatable) object file\n  -shared      generate a shared library/dll\n  -rdynamic    export all global symbols to dynamic linker\n  -soname      set name for shared library to be used at runtime\n  -Wl,-opt[=val]  set linker option (see tcc -hh)\nDebugger options:\n  -g           generate runtime debug info\n  -b           compile with built-in memory and bounds checker (implies -g)\n  -bt[N]       link with backtrace (stack dump) support [show max N callers]\nMisc. options:\n  -x[c|a|b|n]  specify type of the next infile (C,ASM,BIN,NONE)\n  -nostdinc    do not use standard system include paths\n  -nostdlib    do not link with standard crt and libraries\n  -Bdir        set tcc\'s private include/library dir\n  -MD          generate dependency file for make\n  -MF file     specify dependency file name\n  -m32/64      defer to i386/x86_64 cross compiler\nTools:\n  create library  : tcc -ar [rcsv] lib.a files\n\x00")
};
static mut help2: [std::os::raw::c_char; 2522] = unsafe {
    *::std::mem::transmute::<&[u8; 2522],
                                 &[std::os::raw::c_char; 2522]>(b"Tiny C Compiler 0.9.27 - More Options\nSpecial options:\n  -P -P1                        with -E: no/alternative #line output\n  -dD -dM                       with -E: output #define directives\n  -pthread                      same as -D_REENTRANT and -lpthread\n  -On                           same as -D__OPTIMIZE__ for n > 0\n  -Wp,-opt                      same as -opt\n  -include file                 include \'file\' above each input file\n  -isystem dir                  add \'dir\' to system include path\n  -static                       link to static libraries (not recommended)\n  -dumpversion                  print version\n  -print-search-dirs            print search paths\n  -dt                           with -run/-E: auto-define \'test_...\' macros\nIgnored options:\n  --param  -pedantic  -pipe  -s  -traditional\n-W... warnings:\n  all                           turn on some (*) warnings\n  error                         stop after first warning\n  unsupported                   warn about ignored options, pragmas, etc.\n  write-strings                 strings are const\n  implicit-function-declaration warn for missing prototype (*)\n-f[no-]... flags:\n  unsigned-char                 default char is unsigned\n  signed-char                   default char is signed\n  common                        use common section instead of bss\n  leading-underscore            decorate extern symbols\n  ms-extensions                 allow anonymous struct in struct\n  dollars-in-identifiers        allow \'$\' in C symbols\n  test-coverage                 create code coverage code\n-m... target specific options:\n  ms-bitfields                  use MSVC bitfield layout\n  no-sse                        disable floats on x86_64\n-Wl,... linker options:\n  -nostdlib                     do not link with standard crt/libs\n  -[no-]whole-archive           load lib(s) fully/only as needed\n  -export-all-symbols           same as -rdynamic\n  -export-dynamic               same as -rdynamic\n  -image-base= -Ttext=          set base address of executable\n  -section-alignment=           set section alignment in executable\n  -rpath=                       set dynamic library search path\n  -enable-new-dtags             set DT_RUNPATH instead of DT_RPATH\n  -soname=                      set DT_SONAME elf tag\n  -Bsymbolic                    set DT_SYMBOLIC elf tag\n  -oformat=[elf32/64-* binary]  set executable output format\n  -init= -fini= -as-needed -O   (ignored)\nPredefined macros:\n  tcc -E -dM - < /dev/null\nSee also the manual for more details.\n\x00")
};
static mut version: [std::os::raw::c_char; 35] = unsafe {
    *::std::mem::transmute::<&[u8; 35], &[std::os::raw::c_char; 35]>(
        b"tcc version 0.9.27 (x86_64 Linux)\n\x00",
    )
};
unsafe extern "C" fn print_dirs(
    mut msg: *const std::os::raw::c_char,
    mut paths: *mut *mut std::os::raw::c_char,
    mut nb_paths: std::os::raw::c_int,
) {
    let mut i: std::os::raw::c_int = 0;
    printf(
        b"%s:\n%s\x00" as *const u8 as *const std::os::raw::c_char,
        msg,
        if nb_paths != 0 {
            b"\x00" as *const u8 as *const std::os::raw::c_char
        } else {
            b"  -\n\x00" as *const u8 as *const std::os::raw::c_char
        },
    );
    i = 0 as std::os::raw::c_int;
    while i < nb_paths {
        printf(
            b"  %s\n\x00" as *const u8 as *const std::os::raw::c_char,
            *paths.offset(i as isize),
        );
        i += 1
    }
}
unsafe extern "C" fn print_search_dirs(mut s: *mut TCCState) {
    printf(
        b"install: %s\n\x00" as *const u8 as *const std::os::raw::c_char,
        (*s).tcc_lib_path,
    );
    /* print_dirs("programs", NULL, 0); */
    print_dirs(
        b"include\x00" as *const u8 as *const std::os::raw::c_char,
        (*s).sysinclude_paths,
        (*s).nb_sysinclude_paths,
    ); /* never returns */
    print_dirs(
        b"libraries\x00" as *const u8 as *const std::os::raw::c_char,
        (*s).library_paths,
        (*s).nb_library_paths,
    );
    printf(
        b"libtcc1:\n  %s/libtcc1.a\n\x00" as *const u8 as *const std::os::raw::c_char,
        (*s).tcc_lib_path,
    );
    print_dirs(
        b"crt\x00" as *const u8 as *const std::os::raw::c_char,
        (*s).crt_paths,
        (*s).nb_crt_paths,
    );
    printf(
        b"elfinterp:\n  %s\n\x00" as *const u8 as *const std::os::raw::c_char,
        b"/lib64/ld-linux-x86-64.so.2\x00" as *const u8 as *const std::os::raw::c_char,
    );
}
unsafe extern "C" fn set_environment(mut s: *mut TCCState) {
    let mut path: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    path = getenv(b"C_INCLUDE_PATH\x00" as *const u8 as *const std::os::raw::c_char);
    if !path.is_null() {
        tcc_add_sysinclude_path(s, path);
    }
    path = getenv(b"CPATH\x00" as *const u8 as *const std::os::raw::c_char);
    if !path.is_null() {
        tcc_add_include_path(s, path);
    }
    path = getenv(b"LIBRARY_PATH\x00" as *const u8 as *const std::os::raw::c_char);
    if !path.is_null() {
        tcc_add_library_path(s, path);
    };
}
unsafe extern "C" fn default_outputfile(
    mut s: *mut TCCState,
    mut first_file: *const std::os::raw::c_char,
) -> *mut std::os::raw::c_char {
    let mut buf: [std::os::raw::c_char; 1024] = [0; 1024];
    let mut ext: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut name: *const std::os::raw::c_char =
        b"a\x00" as *const u8 as *const std::os::raw::c_char;
    if !first_file.is_null()
        && strcmp(
            first_file,
            b"-\x00" as *const u8 as *const std::os::raw::c_char,
        ) != 0
    {
        name = tcc_basename(first_file)
    }
    snprintf(
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[std::os::raw::c_char; 1024]>() as std::os::raw::c_ulong,
        b"%s\x00" as *const u8 as *const std::os::raw::c_char,
        name,
    );
    ext = tcc_fileextension(buf.as_mut_ptr());
    if (*s).output_type == 4 as std::os::raw::c_int
        && (*s).option_r == 0
        && *ext as std::os::raw::c_int != 0
    {
        strcpy(ext, b".o\x00" as *const u8 as *const std::os::raw::c_char);
    } else {
        strcpy(
            buf.as_mut_ptr(),
            b"a.out\x00" as *const u8 as *const std::os::raw::c_char,
        );
    }
    return tcc_strdup(buf.as_mut_ptr());
}
unsafe extern "C" fn getclock_ms() -> std::os::raw::c_uint {
    let mut tv: timeval = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    gettimeofday(&mut tv, 0 as *mut std::os::raw::c_void);
    return (tv.tv_sec * 1000 as std::os::raw::c_int as std::os::raw::c_long
        + (tv.tv_usec + 500 as std::os::raw::c_int as std::os::raw::c_long)
            / 1000 as std::os::raw::c_int as std::os::raw::c_long)
        as std::os::raw::c_uint;
}
unsafe fn main_0(
    mut argc0: std::os::raw::c_int,
    mut argv0: *mut *mut std::os::raw::c_char,
) -> std::os::raw::c_int {
    let mut s: *mut TCCState = 0 as *mut TCCState;
    let mut s1: *mut TCCState = 0 as *mut TCCState;
    let mut ret: std::os::raw::c_int = 0;
    let mut opt: std::os::raw::c_int = 0;
    let mut n: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut t: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut done: std::os::raw::c_int = 0;
    let mut start_time: std::os::raw::c_uint = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    let mut first_file: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut argc: std::os::raw::c_int = 0;
    let mut argv: *mut *mut std::os::raw::c_char = 0 as *mut *mut std::os::raw::c_char;
    let mut ppfp: *mut FILE = stdout;
    loop {
        argc = argc0;
        argv = argv0;
        s1 = tcc_new();
        s = s1;
        opt = tcc_parse_args(s, &mut argc, &mut argv, 1 as std::os::raw::c_int);
        if n == 0 as std::os::raw::c_int {
            if opt == 1 as std::os::raw::c_int {
                fputs(help.as_ptr(), stdout);
                if (*s).verbose == 0 {
                    return 0 as std::os::raw::c_int;
                }
                opt += 1
            }
            if opt == 2 as std::os::raw::c_int {
                fputs(help2.as_ptr(), stdout);
                return 0 as std::os::raw::c_int;
            }
            if opt == 32 as std::os::raw::c_int || opt == 64 as std::os::raw::c_int {
                tcc_tool_cross(s, argv, opt);
            }
            if (*s).verbose != 0 {
                printf(version.as_ptr());
            }
            if opt == 5 as std::os::raw::c_int {
                return tcc_tool_ar(s, argc, argv);
            }
            if opt == 3 as std::os::raw::c_int {
                return 0 as std::os::raw::c_int;
            }
            if opt == 4 as std::os::raw::c_int {
                /* initialize search dirs */
                set_environment(s);
                tcc_set_output_type(s, 1 as std::os::raw::c_int);
                print_search_dirs(s);
                return 0 as std::os::raw::c_int;
            }
            if (*s).nb_files == 0 as std::os::raw::c_int {
                tcc_enter_state(s1);
                Some(
                    _tcc_error as unsafe extern "C" fn(_: *const std::os::raw::c_char, _: ...) -> !,
                )
                .expect("non-null function pointer")(
                    b"no input files\n\x00" as *const u8 as *const std::os::raw::c_char,
                );
            }
            if (*s).output_type == 5 as std::os::raw::c_int {
                if !(*s).outfile.is_null()
                    && 0 as std::os::raw::c_int
                        != strcmp(
                            b"-\x00" as *const u8 as *const std::os::raw::c_char,
                            (*s).outfile,
                        )
                {
                    ppfp = fopen(
                        (*s).outfile,
                        b"w\x00" as *const u8 as *const std::os::raw::c_char,
                    );
                    if ppfp.is_null() {
                        tcc_enter_state(s1);
                        Some(
                            _tcc_error
                                as unsafe extern "C" fn(
                                    _: *const std::os::raw::c_char,
                                    _: ...
                                ) -> !,
                        )
                        .expect("non-null function pointer")(
                            b"could not write \'%s\'\x00" as *const u8
                                as *const std::os::raw::c_char,
                            (*s).outfile,
                        );
                    }
                }
            } else if (*s).output_type == 4 as std::os::raw::c_int && (*s).option_r == 0 {
                if (*s).nb_libraries != 0 {
                    tcc_enter_state(s1);
                    Some(
                        _tcc_error
                            as unsafe extern "C" fn(_: *const std::os::raw::c_char, _: ...) -> !,
                    )
                    .expect("non-null function pointer")(
                        b"cannot specify libraries with -c\x00" as *const u8
                            as *const std::os::raw::c_char,
                    );
                }
                if (*s).nb_files > 1 as std::os::raw::c_int && !(*s).outfile.is_null() {
                    tcc_enter_state(s1);
                    Some(
                        _tcc_error
                            as unsafe extern "C" fn(_: *const std::os::raw::c_char, _: ...) -> !,
                    )
                    .expect("non-null function pointer")(
                        b"cannot specify output file with -c many files\x00" as *const u8
                            as *const std::os::raw::c_char,
                    );
                }
            }
            if (*s).do_bench != 0 {
                start_time = getclock_ms()
            }
        }
        set_environment(s);
        if (*s).output_type == 0 as std::os::raw::c_int {
            (*s).output_type = 2 as std::os::raw::c_int
        }
        tcc_set_output_type(s, (*s).output_type);
        (*s).ppfp = ppfp;
        if ((*s).output_type == 1 as std::os::raw::c_int
            || (*s).output_type == 5 as std::os::raw::c_int)
            && (*s).dflag as std::os::raw::c_int & 16 as std::os::raw::c_int != 0
        {
            /* -dt option */
            if t != 0 {
                (*s).dflag = ((*s).dflag as std::os::raw::c_int | 32 as std::os::raw::c_int)
                    as std::os::raw::c_char
            }
            t += 1;
            (*s).run_test = t;
            if n != 0 {
                n -= 1
            }
        }
        /* compile or add each files or library */
        first_file = 0 as *const std::os::raw::c_char; /* compile more files with -c */
        ret = 0 as std::os::raw::c_int; /* run more tests with -dt -run */
        loop {
            let mut f: *mut filespec = *(*s).files.offset(n as isize);
            (*s).filetype = (*f).type_0 as std::os::raw::c_uchar;
            if (*f).type_0 as std::os::raw::c_int & 8 as std::os::raw::c_int != 0 {
                if tcc_add_library_err(s, (*f).name.as_mut_ptr()) < 0 as std::os::raw::c_int {
                    ret = 1 as std::os::raw::c_int
                }
            } else {
                if 1 as std::os::raw::c_int == (*s).verbose as std::os::raw::c_int {
                    printf(
                        b"-> %s\n\x00" as *const u8 as *const std::os::raw::c_char,
                        (*f).name.as_mut_ptr(),
                    );
                }
                if first_file.is_null() {
                    first_file = (*f).name.as_mut_ptr()
                }
                if tcc_add_file(s, (*f).name.as_mut_ptr()) < 0 as std::os::raw::c_int {
                    ret = 1 as std::os::raw::c_int
                }
            }
            done = (ret != 0 || {
                n += 1;
                (n) >= (*s).nb_files
            }) as std::os::raw::c_int;
            if !(done == 0
                && ((*s).output_type != 4 as std::os::raw::c_int
                    || (*s).option_r as std::os::raw::c_int != 0))
            {
                break;
            }
        }
        if (*s).run_test != 0 {
            t = 0 as std::os::raw::c_int
        } else if !((*s).output_type == 5 as std::os::raw::c_int) {
            if 0 as std::os::raw::c_int == ret {
                if (*s).output_type == 1 as std::os::raw::c_int {
                    ret = tcc_run(s, argc, argv)
                } else {
                    if (*s).outfile.is_null() {
                        (*s).outfile = default_outputfile(s, first_file)
                    }
                    if tcc_output_file(s, (*s).outfile) != 0 {
                        ret = 1 as std::os::raw::c_int
                    } else if (*s).gen_deps != 0 {
                        gen_makedeps(s, (*s).outfile, (*s).deps_outfile);
                    }
                }
            }
        }
        if (*s).do_bench as std::os::raw::c_int != 0 && done != 0 && t | ret == 0 {
            tcc_print_stats(s, getclock_ms().wrapping_sub(start_time));
        }
        tcc_delete(s);
        if done == 0 {
            continue;
        }
        if !(t != 0) {
            break;
        }
    }
    if !ppfp.is_null() && ppfp != stdout {
        fclose(ppfp);
    }
    return ret;
}
#[main]
pub fn main() {
    let mut args: Vec<*mut std::os::raw::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            ::std::ffi::CString::new(arg)
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0(
            (args.len() - 1) as std::os::raw::c_int,
            args.as_mut_ptr() as *mut *mut std::os::raw::c_char,
        ) as i32)
    }
}
