use crate::bitfields::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type sym_version;
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
    fn memcpy(
        _: *mut std::os::raw::c_void,
        _: *const std::os::raw::c_void,
        _: std::os::raw::c_ulong,
    ) -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn strcmp(
        _: *const std::os::raw::c_char,
        _: *const std::os::raw::c_char,
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
    fn strchr(_: *const std::os::raw::c_char, _: std::os::raw::c_int) -> *mut std::os::raw::c_char;
    #[no_mangle]
    static mut tcc_state: *mut TCCState;
    #[no_mangle]
    fn _tcc_error(fmt: *const std::os::raw::c_char, _: ...) -> !;
    #[no_mangle]
    fn cstr_ccat(cstr: *mut CString, ch: std::os::raw::c_int);
    #[no_mangle]
    fn cstr_cat(cstr: *mut CString, str: *const std::os::raw::c_char, len: std::os::raw::c_int);
    #[no_mangle]
    static mut tok: std::os::raw::c_int;
    #[no_mangle]
    static mut tokc: CValue;
    #[no_mangle]
    static mut tok_ident: std::os::raw::c_int;
    #[no_mangle]
    static mut table_ident: *mut *mut TokenSym;
    #[no_mangle]
    fn tok_alloc(str: *const std::os::raw::c_char, len: std::os::raw::c_int) -> *mut TokenSym;
    #[no_mangle]
    fn tok_alloc_const(str: *const std::os::raw::c_char) -> std::os::raw::c_int;
    #[no_mangle]
    fn get_tok_str(v: std::os::raw::c_int, cv: *mut CValue) -> *const std::os::raw::c_char;
    #[no_mangle]
    fn next();
    #[no_mangle]
    fn unget_tok(last_tok: std::os::raw::c_int);
    #[no_mangle]
    fn skip(c: std::os::raw::c_int);
    #[no_mangle]
    fn expect(msg: *const std::os::raw::c_char) -> !;
    #[no_mangle]
    static mut ind: std::os::raw::c_int;
    #[no_mangle]
    fn elfsym(_: *mut Sym) -> *mut Elf64_Sym;
    #[no_mangle]
    fn load(r: std::os::raw::c_int, sv: *mut SValue);
    #[no_mangle]
    fn store(r: std::os::raw::c_int, v: *mut SValue);
    #[no_mangle]
    fn g(c: std::os::raw::c_int);
    #[no_mangle]
    fn gen_le16(c: std::os::raw::c_int);
    #[no_mangle]
    fn gen_le32(c: std::os::raw::c_int);
    #[no_mangle]
    fn gen_addr32(r: std::os::raw::c_int, sym: *mut Sym, c: std::os::raw::c_int);
    #[no_mangle]
    fn gen_addrpc32(r: std::os::raw::c_int, sym: *mut Sym, c: std::os::raw::c_int);
    #[no_mangle]
    fn gen_addr64(r: std::os::raw::c_int, sym: *mut Sym, c: int64_t);
    #[no_mangle]
    fn find_constraint(
        operands: *mut ASMOperand,
        nb_operands: std::os::raw::c_int,
        name: *const std::os::raw::c_char,
        pp: *mut *const std::os::raw::c_char,
    ) -> std::os::raw::c_int;
    #[no_mangle]
    fn get_asm_sym(name: std::os::raw::c_int, csym: *mut Sym) -> *mut Sym;
    #[no_mangle]
    fn asm_expr(s1: *mut TCCState, pe: *mut ExprValue);
    #[no_mangle]
    fn asm_int_expr(s1: *mut TCCState) -> std::os::raw::c_int;
}
pub type size_t = std::os::raw::c_ulong;
pub type __int8_t = std::os::raw::c_schar;
pub type __uint8_t = std::os::raw::c_uchar;
pub type __uint16_t = std::os::raw::c_ushort;
pub type __int32_t = std::os::raw::c_int;
pub type __uint32_t = std::os::raw::c_uint;
pub type __int64_t = std::os::raw::c_long;
pub type __uint64_t = std::os::raw::c_ulong;
pub type __off_t = std::os::raw::c_long;
pub type __off64_t = std::os::raw::c_long;
pub type int8_t = __int8_t;
pub type int32_t = __int32_t;
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
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type Elf64_Word = uint32_t;
pub type Elf64_Section = uint16_t;
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
pub type C2RustUnnamed_4 = std::os::raw::c_uint;
pub const TREG_MEM: C2RustUnnamed_4 = 32;
pub const TREG_ST0: C2RustUnnamed_4 = 24;
pub const TREG_XMM7: C2RustUnnamed_4 = 23;
pub const TREG_XMM6: C2RustUnnamed_4 = 22;
pub const TREG_XMM5: C2RustUnnamed_4 = 21;
pub const TREG_XMM4: C2RustUnnamed_4 = 20;
pub const TREG_XMM3: C2RustUnnamed_4 = 19;
pub const TREG_XMM2: C2RustUnnamed_4 = 18;
pub const TREG_XMM1: C2RustUnnamed_4 = 17;
pub const TREG_XMM0: C2RustUnnamed_4 = 16;
pub const TREG_R11: C2RustUnnamed_4 = 11;
pub const TREG_R10: C2RustUnnamed_4 = 10;
pub const TREG_R9: C2RustUnnamed_4 = 9;
pub const TREG_R8: C2RustUnnamed_4 = 8;
pub const TREG_RDI: C2RustUnnamed_4 = 7;
pub const TREG_RSI: C2RustUnnamed_4 = 6;
pub const TREG_RSP: C2RustUnnamed_4 = 4;
pub const TREG_RDX: C2RustUnnamed_4 = 2;
pub const TREG_RCX: C2RustUnnamed_4 = 1;
pub const TREG_RAX: C2RustUnnamed_4 = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union CValue {
    pub ld: f128::f128,
    pub d: std::os::raw::c_double,
    pub f: std::os::raw::c_float,
    pub i: uint64_t,
    pub str_0: C2RustUnnamed_5,
    pub tab: [std::os::raw::c_int; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub data: *const std::os::raw::c_void,
    pub size: std::os::raw::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SValue {
    pub type_0: CType,
    pub r: std::os::raw::c_ushort,
    pub r2: std::os::raw::c_ushort,
    pub c2rust_unnamed: C2RustUnnamed_8,
    pub c2rust_unnamed_0: C2RustUnnamed_6,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub c2rust_unnamed: C2RustUnnamed_7,
    pub sym: *mut Sym,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub cmp_op: std::os::raw::c_ushort,
    pub cmp_r: std::os::raw::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
    pub c2rust_unnamed: C2RustUnnamed_9,
    pub c: CValue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub jtrue: std::os::raw::c_int,
    pub jfalse: std::os::raw::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExprValue {
    pub v: uint64_t,
    pub sym: *mut Sym,
    pub pcrel: std::os::raw::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ASMOperand {
    pub id: std::os::raw::c_int,
    pub constraint: *mut std::os::raw::c_char,
    pub asm_str: [std::os::raw::c_char; 16],
    pub vt: *mut SValue,
    pub ref_index: std::os::raw::c_int,
    pub input_index: std::os::raw::c_int,
    pub priority: std::os::raw::c_int,
    pub reg: std::os::raw::c_int,
    pub is_llong: std::os::raw::c_int,
    pub is_memory: std::os::raw::c_int,
    pub is_rw: std::os::raw::c_int,
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
pub struct Operand {
    pub type_0: uint32_t,
    pub reg: int8_t,
    pub reg2: int8_t,
    pub shift: uint8_t,
    pub e: ExprValue,
}
/* Like OPT_ADDR, but emitted as displacement (for jumps) */
pub const OPT_DISP8: C2RustUnnamed_10 = 30;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ASMInstr {
    pub sym: uint16_t,
    pub opcode: uint16_t,
    pub instr_type: uint16_t,
    pub nb_ops: uint8_t,
    pub op_type: [uint8_t; 3],
}
/* MMX | SSE */
pub const OPT_DISP: C2RustUnnamed_10 = 29;
pub const OPT_IM64: C2RustUnnamed_10 = 16;
pub const OPT_IM16: C2RustUnnamed_10 = 14;
pub const OPT_IM8S: C2RustUnnamed_10 = 13;
/* %spl,%bpl,%sil,%dil, encoded like ah,ch,dh,bh, but
with REX prefix, not used in insn templates */
pub const OPT_IM8: C2RustUnnamed_10 = 12;
pub const OPT_IM32: C2RustUnnamed_10 = 15;
/* %dx register */
pub const OPT_ADDR: C2RustUnnamed_10 = 21;
/* warning: value is hardcoded from TOK_ASM_xxx */
pub const OPT_SSE: C2RustUnnamed_10 = 5;
/* warning: value is hardcoded from TOK_ASM_xxx */
pub const OPT_MMX: C2RustUnnamed_10 = 4;
/* warning: value is hardcoded from TOK_ASM_xxx */
pub const OPT_REG64: C2RustUnnamed_10 = 3;
/* warning: value is hardcoded from TOK_ASM_xxx */
pub const OPT_REG32: C2RustUnnamed_10 = 2;
/* warning: value is hardcoded from TOK_ASM_xxx */
pub const OPT_REG16: C2RustUnnamed_10 = 1;
pub const OPT_REG8: C2RustUnnamed_10 = 0;
pub const OPT_ST: C2RustUnnamed_10 = 10;
pub const OPT_REG8_LOW: C2RustUnnamed_10 = 11;
/* warning: value is hardcoded from TOK_ASM_xxx */
pub const OPT_CR: C2RustUnnamed_10 = 6;
/* warning: value is hardcoded from TOK_ASM_xxx */
pub const OPT_SEG: C2RustUnnamed_10 = 9;
/* warning: value is hardcoded from TOK_ASM_xxx */
pub const OPT_DB: C2RustUnnamed_10 = 8;
/* warning: value is hardcoded from TOK_ASM_xxx */
pub const OPT_TR: C2RustUnnamed_10 = 7;
/* OP_EA with only offset */
pub const OPT_INDIR: C2RustUnnamed_10 = 22;
/* %cl register */
pub const OPT_DX: C2RustUnnamed_10 = 20;
/* %st(0) register */
pub const OPT_CL: C2RustUnnamed_10 = 19;
/* Like OPT_ADDR, but only 8bit (short jumps) */
/* can be ored with any OPT_xxx */
pub const OPT_EA: C2RustUnnamed_10 = 128;
/* IM16 | IM32 */
pub const OPT_MMXSSE: C2RustUnnamed_10 = 28;
/* REG16 | REG32 | REG64 */
pub const OPT_IMW: C2RustUnnamed_10 = 27;
/* REG8 | REG16 | REG32 | REG64 */
pub const OPT_REGW: C2RustUnnamed_10 = 26;
/* IM8 | IM16 | IM32 */
pub const OPT_REG: C2RustUnnamed_10 = 25;
pub const OPT_IM: C2RustUnnamed_10 = 24;
/* %al, %ax, %eax or %rax register */
pub const OPT_ST0: C2RustUnnamed_10 = 18;
pub const OPT_EAX: C2RustUnnamed_10 = 17;
/* in order to compress the operand type, we use specific operands and
we or only with EA  */
pub type C2RustUnnamed_10 = std::os::raw::c_uint;
/* *(expr) */
/* composite types */
pub const OPT_COMPOSITE_FIRST: C2RustUnnamed_10 = 23;
#[inline]
unsafe extern "C" fn isnum(mut c: std::os::raw::c_int) -> std::os::raw::c_int {
    return (c >= '0' as i32 && c <= '9' as i32) as std::os::raw::c_int;
}
#[inline]
unsafe extern "C" fn read16le(mut p: *mut std::os::raw::c_uchar) -> uint16_t {
    return (*p.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int
        | (*p.offset(1 as std::os::raw::c_int as isize) as uint16_t as std::os::raw::c_int)
            << 8 as std::os::raw::c_int) as uint16_t;
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
unsafe extern "C" fn read32le(mut p: *mut std::os::raw::c_uchar) -> uint32_t {
    return read16le(p) as std::os::raw::c_uint
        | (read16le(p.offset(2 as std::os::raw::c_int as isize)) as uint32_t)
            << 16 as std::os::raw::c_int;
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
unsafe extern "C" fn add32le(mut p: *mut std::os::raw::c_uchar, mut x: int32_t) {
    write32le(p, read32le(p).wrapping_add(x as std::os::raw::c_uint));
}
static mut reg_to_size: [uint8_t; 9] = [
    0 as std::os::raw::c_int as uint8_t,
    0 as std::os::raw::c_int as uint8_t,
    1 as std::os::raw::c_int as uint8_t,
    0 as std::os::raw::c_int as uint8_t,
    2 as std::os::raw::c_int as uint8_t,
    0 as std::os::raw::c_int as uint8_t,
    0 as std::os::raw::c_int as uint8_t,
    0 as std::os::raw::c_int as uint8_t,
    3 as std::os::raw::c_int as uint8_t,
];
static mut test_bits: [uint8_t; 30] = [
    0 as std::os::raw::c_int as uint8_t,
    0x1 as std::os::raw::c_int as uint8_t,
    0x2 as std::os::raw::c_int as uint8_t,
    0x2 as std::os::raw::c_int as uint8_t,
    0x2 as std::os::raw::c_int as uint8_t,
    0x3 as std::os::raw::c_int as uint8_t,
    0x3 as std::os::raw::c_int as uint8_t,
    0x3 as std::os::raw::c_int as uint8_t,
    0x4 as std::os::raw::c_int as uint8_t,
    0x4 as std::os::raw::c_int as uint8_t,
    0x5 as std::os::raw::c_int as uint8_t,
    0x5 as std::os::raw::c_int as uint8_t,
    0x6 as std::os::raw::c_int as uint8_t,
    0x6 as std::os::raw::c_int as uint8_t,
    0x7 as std::os::raw::c_int as uint8_t,
    0x7 as std::os::raw::c_int as uint8_t,
    0x8 as std::os::raw::c_int as uint8_t,
    0x9 as std::os::raw::c_int as uint8_t,
    0xa as std::os::raw::c_int as uint8_t,
    0xa as std::os::raw::c_int as uint8_t,
    0xb as std::os::raw::c_int as uint8_t,
    0xb as std::os::raw::c_int as uint8_t,
    0xc as std::os::raw::c_int as uint8_t,
    0xc as std::os::raw::c_int as uint8_t,
    0xd as std::os::raw::c_int as uint8_t,
    0xd as std::os::raw::c_int as uint8_t,
    0xe as std::os::raw::c_int as uint8_t,
    0xe as std::os::raw::c_int as uint8_t,
    0xf as std::os::raw::c_int as uint8_t,
    0xf as std::os::raw::c_int as uint8_t,
];
static mut segment_prefixes: [uint8_t; 6] = [
    0x26 as std::os::raw::c_int as uint8_t,
    0x2e as std::os::raw::c_int as uint8_t,
    0x36 as std::os::raw::c_int as uint8_t,
    0x3e as std::os::raw::c_int as uint8_t,
    0x64 as std::os::raw::c_int as uint8_t,
    0x65 as std::os::raw::c_int as uint8_t,
];
// Initialized in run_static_initializers
static mut asm_instrs: [ASMInstr; 332] = [ASMInstr {
    sym: 0,
    opcode: 0,
    instr_type: 0,
    nb_ops: 0,
    op_type: [0; 3],
}; 332];
static mut op0_codes: [uint16_t; 93] = [
    0xf8 as std::os::raw::c_int as uint16_t,
    0xfc as std::os::raw::c_int as uint16_t,
    0xfa as std::os::raw::c_int as uint16_t,
    0xf06 as std::os::raw::c_int as uint16_t,
    0xf5 as std::os::raw::c_int as uint16_t,
    0x9f as std::os::raw::c_int as uint16_t,
    0x9e as std::os::raw::c_int as uint16_t,
    0x9c as std::os::raw::c_int as uint16_t,
    0x9d as std::os::raw::c_int as uint16_t,
    0x9c as std::os::raw::c_int as uint16_t,
    0x9d as std::os::raw::c_int as uint16_t,
    0xf9 as std::os::raw::c_int as uint16_t,
    0xfd as std::os::raw::c_int as uint16_t,
    0xfb as std::os::raw::c_int as uint16_t,
    0x37 as std::os::raw::c_int as uint16_t,
    0x3f as std::os::raw::c_int as uint16_t,
    0x27 as std::os::raw::c_int as uint16_t,
    0x2f as std::os::raw::c_int as uint16_t,
    0xd50a as std::os::raw::c_int as uint16_t,
    0xd40a as std::os::raw::c_int as uint16_t,
    0x6698 as std::os::raw::c_int as uint16_t,
    0x6699 as std::os::raw::c_int as uint16_t,
    0x98 as std::os::raw::c_int as uint16_t,
    0x99 as std::os::raw::c_int as uint16_t,
    0x6698 as std::os::raw::c_int as uint16_t,
    0x98 as std::os::raw::c_int as uint16_t,
    0x6699 as std::os::raw::c_int as uint16_t,
    0x99 as std::os::raw::c_int as uint16_t,
    0x4899 as std::os::raw::c_int as uint16_t,
    0xcc as std::os::raw::c_int as uint16_t,
    0xce as std::os::raw::c_int as uint16_t,
    0xcf as std::os::raw::c_int as uint16_t,
    0xfaa as std::os::raw::c_int as uint16_t,
    0xf4 as std::os::raw::c_int as uint16_t,
    0x9b as std::os::raw::c_int as uint16_t,
    0x90 as std::os::raw::c_int as uint16_t,
    0xf390 as std::os::raw::c_int as uint16_t,
    0xd7 as std::os::raw::c_int as uint16_t,
    0xf0 as std::os::raw::c_int as uint16_t,
    0xf3 as std::os::raw::c_int as uint16_t,
    0xf3 as std::os::raw::c_int as uint16_t,
    0xf3 as std::os::raw::c_int as uint16_t,
    0xf2 as std::os::raw::c_int as uint16_t,
    0xf2 as std::os::raw::c_int as uint16_t,
    0xf08 as std::os::raw::c_int as uint16_t,
    0xf09 as std::os::raw::c_int as uint16_t,
    0xfa2 as std::os::raw::c_int as uint16_t,
    0xf30 as std::os::raw::c_int as uint16_t,
    0xf31 as std::os::raw::c_int as uint16_t,
    0xf32 as std::os::raw::c_int as uint16_t,
    0xf33 as std::os::raw::c_int as uint16_t,
    0xf05 as std::os::raw::c_int as uint16_t,
    0xf07 as std::os::raw::c_int as uint16_t,
    0xf0b as std::os::raw::c_int as uint16_t,
    0xc9 as std::os::raw::c_int as uint16_t,
    0xc3 as std::os::raw::c_int as uint16_t,
    0xc3 as std::os::raw::c_int as uint16_t,
    0xcb as std::os::raw::c_int as uint16_t,
    0xdae9 as std::os::raw::c_int as uint16_t,
    0xd9e4 as std::os::raw::c_int as uint16_t,
    0xd9e5 as std::os::raw::c_int as uint16_t,
    0xd9e8 as std::os::raw::c_int as uint16_t,
    0xd9e9 as std::os::raw::c_int as uint16_t,
    0xd9ea as std::os::raw::c_int as uint16_t,
    0xd9eb as std::os::raw::c_int as uint16_t,
    0xd9ec as std::os::raw::c_int as uint16_t,
    0xd9ed as std::os::raw::c_int as uint16_t,
    0xd9ee as std::os::raw::c_int as uint16_t,
    0xd9f0 as std::os::raw::c_int as uint16_t,
    0xd9f1 as std::os::raw::c_int as uint16_t,
    0xd9f2 as std::os::raw::c_int as uint16_t,
    0xd9f3 as std::os::raw::c_int as uint16_t,
    0xd9f4 as std::os::raw::c_int as uint16_t,
    0xd9f5 as std::os::raw::c_int as uint16_t,
    0xd9f6 as std::os::raw::c_int as uint16_t,
    0xd9f7 as std::os::raw::c_int as uint16_t,
    0xd9f8 as std::os::raw::c_int as uint16_t,
    0xd9f9 as std::os::raw::c_int as uint16_t,
    0xd9fa as std::os::raw::c_int as uint16_t,
    0xd9fb as std::os::raw::c_int as uint16_t,
    0xd9fc as std::os::raw::c_int as uint16_t,
    0xd9fd as std::os::raw::c_int as uint16_t,
    0xd9fe as std::os::raw::c_int as uint16_t,
    0xd9ff as std::os::raw::c_int as uint16_t,
    0xd9e0 as std::os::raw::c_int as uint16_t,
    0xd9e1 as std::os::raw::c_int as uint16_t,
    0xdbe3 as std::os::raw::c_int as uint16_t,
    0xdbe2 as std::os::raw::c_int as uint16_t,
    0xd9d0 as std::os::raw::c_int as uint16_t,
    0x9b as std::os::raw::c_int as uint16_t,
    0xd9c9 as std::os::raw::c_int as uint16_t,
    0xdfe0 as std::os::raw::c_int as uint16_t,
    0xf77 as std::os::raw::c_int as uint16_t,
];
#[inline]
unsafe extern "C" fn get_reg_shift(mut s1: *mut TCCState) -> std::os::raw::c_int {
    let mut shift: std::os::raw::c_int = 0;
    let mut v: std::os::raw::c_int = 0;
    v = asm_int_expr(s1);
    match v {
        1 => shift = 0 as std::os::raw::c_int,
        2 => shift = 1 as std::os::raw::c_int,
        4 => shift = 2 as std::os::raw::c_int,
        8 => shift = 3 as std::os::raw::c_int,
        _ => {
            expect(b"1, 2, 4 or 8 constant\x00" as *const u8 as *const std::os::raw::c_char);
        },
    }
    return shift;
}
unsafe extern "C" fn asm_parse_numeric_reg(
    mut t: std::os::raw::c_int,
    mut type_0: *mut std::os::raw::c_uint,
) -> std::os::raw::c_int {
    let mut reg: std::os::raw::c_int = -(1 as std::os::raw::c_int);
    if t >= 256 as std::os::raw::c_int && t < tok_ident {
        let mut s: *const std::os::raw::c_char = (**table_ident
            .offset((t - 256 as std::os::raw::c_int) as isize))
        .str_0
        .as_mut_ptr();
        let mut c: std::os::raw::c_char = 0;
        *type_0 = ((1 as std::os::raw::c_int) << OPT_REG64 as std::os::raw::c_int)
            as std::os::raw::c_uint;
        if *s as std::os::raw::c_int == 'c' as i32 {
            s = s.offset(1);
            *type_0 = ((1 as std::os::raw::c_int) << OPT_CR as std::os::raw::c_int)
                as std::os::raw::c_uint
        }
        let fresh0 = s;
        s = s.offset(1);
        if *fresh0 as std::os::raw::c_int != 'r' as i32 {
            return -(1 as std::os::raw::c_int);
        }
        /* Don't allow leading '0'.  */
        let fresh1 = s; /* Probably should use different escape code. */
        s = s.offset(1); /* WARNING: do not change constant order */
        c = *fresh1;
        if c as std::os::raw::c_int >= '1' as i32 && c as std::os::raw::c_int <= '9' as i32 {
            reg = c as std::os::raw::c_int - '0' as i32
        } else {
            return -(1 as std::os::raw::c_int);
        }
        c = *s;
        if c as std::os::raw::c_int >= '0' as i32 && c as std::os::raw::c_int <= '5' as i32 {
            s = s.offset(1);
            reg = reg * 10 as std::os::raw::c_int + c as std::os::raw::c_int - '0' as i32
        }
        if reg > 15 as std::os::raw::c_int {
            return -(1 as std::os::raw::c_int);
        }
        c = *s;
        if !(c as std::os::raw::c_int == 0 as std::os::raw::c_int) {
            if *type_0
                != ((1 as std::os::raw::c_int) << OPT_REG64 as std::os::raw::c_int)
                    as std::os::raw::c_uint
            {
                return -(1 as std::os::raw::c_int);
            } else {
                if c as std::os::raw::c_int == 'b' as i32
                    && *s.offset(1 as std::os::raw::c_int as isize) == 0
                {
                    *type_0 = ((1 as std::os::raw::c_int) << OPT_REG8 as std::os::raw::c_int)
                        as std::os::raw::c_uint
                } else if c as std::os::raw::c_int == 'w' as i32
                    && *s.offset(1 as std::os::raw::c_int as isize) == 0
                {
                    *type_0 = ((1 as std::os::raw::c_int) << OPT_REG16 as std::os::raw::c_int)
                        as std::os::raw::c_uint
                } else if c as std::os::raw::c_int == 'd' as i32
                    && *s.offset(1 as std::os::raw::c_int as isize) == 0
                {
                    *type_0 = ((1 as std::os::raw::c_int) << OPT_REG32 as std::os::raw::c_int)
                        as std::os::raw::c_uint
                } else {
                    return -(1 as std::os::raw::c_int);
                }
            }
        }
    }
    return reg;
}
unsafe extern "C" fn asm_parse_reg(mut type_0: *mut std::os::raw::c_uint) -> std::os::raw::c_int {
    let mut current_block: u64;
    let mut reg: std::os::raw::c_int = 0 as std::os::raw::c_int;
    *type_0 = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    if !(tok != '%' as i32) {
        next();
        if tok >= TOK_ASM_eax as std::os::raw::c_int && tok <= TOK_ASM_edi as std::os::raw::c_int {
            reg = tok - TOK_ASM_eax as std::os::raw::c_int;
            *type_0 = ((1 as std::os::raw::c_int) << OPT_REG32 as std::os::raw::c_int)
                as std::os::raw::c_uint;
            current_block = 15652330335145281839;
        } else if tok >= TOK_ASM_rax as std::os::raw::c_int
            && tok <= TOK_ASM_rdi as std::os::raw::c_int
        {
            reg = tok - TOK_ASM_rax as std::os::raw::c_int;
            *type_0 = ((1 as std::os::raw::c_int) << OPT_REG64 as std::os::raw::c_int)
                as std::os::raw::c_uint;
            current_block = 15652330335145281839;
        } else if tok == TOK_ASM_rip as std::os::raw::c_int {
            reg = -(2 as std::os::raw::c_int);
            *type_0 = ((1 as std::os::raw::c_int) << OPT_REG64 as std::os::raw::c_int)
                as std::os::raw::c_uint;
            current_block = 15652330335145281839;
        } else {
            reg = asm_parse_numeric_reg(tok, type_0);
            if reg >= 0 as std::os::raw::c_int
                && (*type_0
                    == ((1 as std::os::raw::c_int) << OPT_REG32 as std::os::raw::c_int)
                        as std::os::raw::c_uint
                    || *type_0
                        == ((1 as std::os::raw::c_int) << OPT_REG64 as std::os::raw::c_int)
                            as std::os::raw::c_uint)
            {
                current_block = 15652330335145281839;
            } else {
                current_block = 4256701872681613420;
            }
        }
        match current_block {
            4256701872681613420 => {},
            _ => {
                next();
                return reg;
            },
        }
    }
    expect(b"register\x00" as *const u8 as *const std::os::raw::c_char);
}
unsafe extern "C" fn parse_operand(mut s1: *mut TCCState, mut op: *mut Operand) {
    let mut current_block: u64;
    let mut e: ExprValue = ExprValue {
        v: 0,
        sym: 0 as *mut Sym,
        pcrel: 0,
    };
    let mut reg: std::os::raw::c_int = 0;
    let mut indir: std::os::raw::c_int = 0;
    let mut p: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    indir = 0 as std::os::raw::c_int;
    if tok == '*' as i32 {
        next();
        indir = (1 as std::os::raw::c_int) << OPT_INDIR as std::os::raw::c_int
    }
    if tok == '%' as i32 {
        next();
        if tok >= TOK_ASM_al as std::os::raw::c_int && tok <= TOK_ASM_db7 as std::os::raw::c_int {
            reg = tok - TOK_ASM_al as std::os::raw::c_int;
            (*op).type_0 =
                ((1 as std::os::raw::c_int) << (reg >> 3 as std::os::raw::c_int)) as uint32_t;
            (*op).reg = (reg & 7 as std::os::raw::c_int) as int8_t;
            if (*op).type_0
                & ((1 as std::os::raw::c_int) << OPT_REG8 as std::os::raw::c_int
                    | (1 as std::os::raw::c_int) << OPT_REG16 as std::os::raw::c_int
                    | (1 as std::os::raw::c_int) << OPT_REG32 as std::os::raw::c_int
                    | (1 as std::os::raw::c_int) << OPT_REG64 as std::os::raw::c_int)
                    as std::os::raw::c_uint
                != 0
                && (*op).reg as std::os::raw::c_int == TREG_RAX as std::os::raw::c_int
            {
                (*op).type_0 |= ((1 as std::os::raw::c_int) << OPT_EAX as std::os::raw::c_int)
                    as std::os::raw::c_uint
            } else if (*op).type_0
                == ((1 as std::os::raw::c_int) << OPT_REG8 as std::os::raw::c_int)
                    as std::os::raw::c_uint
                && (*op).reg as std::os::raw::c_int == TREG_RCX as std::os::raw::c_int
            {
                (*op).type_0 |= ((1 as std::os::raw::c_int) << OPT_CL as std::os::raw::c_int)
                    as std::os::raw::c_uint
            } else if (*op).type_0
                == ((1 as std::os::raw::c_int) << OPT_REG16 as std::os::raw::c_int)
                    as std::os::raw::c_uint
                && (*op).reg as std::os::raw::c_int == TREG_RDX as std::os::raw::c_int
            {
                (*op).type_0 |= ((1 as std::os::raw::c_int) << OPT_DX as std::os::raw::c_int)
                    as std::os::raw::c_uint
            }
            current_block = 1434579379687443766;
        } else if tok >= TOK_ASM_dr0 as std::os::raw::c_int
            && tok <= TOK_ASM_dr7 as std::os::raw::c_int
        {
            (*op).type_0 =
                ((1 as std::os::raw::c_int) << OPT_DB as std::os::raw::c_int) as uint32_t;
            (*op).reg = (tok - TOK_ASM_dr0 as std::os::raw::c_int) as int8_t;
            current_block = 1434579379687443766;
        } else if tok >= TOK_ASM_es as std::os::raw::c_int
            && tok <= TOK_ASM_gs as std::os::raw::c_int
        {
            (*op).type_0 =
                ((1 as std::os::raw::c_int) << OPT_SEG as std::os::raw::c_int) as uint32_t;
            (*op).reg = (tok - TOK_ASM_es as std::os::raw::c_int) as int8_t;
            current_block = 1434579379687443766;
        } else {
            if tok == TOK_ASM_st as std::os::raw::c_int {
                (*op).type_0 =
                    ((1 as std::os::raw::c_int) << OPT_ST as std::os::raw::c_int) as uint32_t;
                (*op).reg = 0 as std::os::raw::c_int as int8_t;
                next();
                if tok == '(' as i32 {
                    next();
                    if tok != 0xcd as std::os::raw::c_int {
                        current_block = 16253299934934335700;
                    } else {
                        p = tokc.str_0.data as *const std::os::raw::c_char;
                        reg = *p.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int
                            - '0' as i32;
                        if reg as std::os::raw::c_uint
                            >= 8 as std::os::raw::c_int as std::os::raw::c_uint
                            || *p.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int
                                != '\u{0}' as i32
                        {
                            current_block = 16253299934934335700;
                        } else {
                            (*op).reg = reg as int8_t;
                            next();
                            skip(')' as i32);
                            current_block = 2873832966593178012;
                        }
                    }
                } else {
                    current_block = 2873832966593178012;
                }
                match current_block {
                    16253299934934335700 => {},
                    _ => {
                        if (*op).reg as std::os::raw::c_int == 0 as std::os::raw::c_int {
                            (*op).type_0 |= ((1 as std::os::raw::c_int)
                                << OPT_ST0 as std::os::raw::c_int)
                                as std::os::raw::c_uint
                        }
                        current_block = 17372050596571538954;
                    },
                }
            } else if tok >= TOK_ASM_spl as std::os::raw::c_int
                && tok <= TOK_ASM_dil as std::os::raw::c_int
            {
                (*op).type_0 = ((1 as std::os::raw::c_int) << OPT_REG8 as std::os::raw::c_int
                    | (1 as std::os::raw::c_int) << OPT_REG8_LOW as std::os::raw::c_int)
                    as uint32_t;
                (*op).reg =
                    (4 as std::os::raw::c_int + tok - TOK_ASM_spl as std::os::raw::c_int) as int8_t;
                current_block = 1434579379687443766;
            } else {
                (*op).reg = asm_parse_numeric_reg(tok, &mut (*op).type_0) as int8_t;
                if (*op).reg as std::os::raw::c_int >= 0 as std::os::raw::c_int {
                    current_block = 1434579379687443766;
                } else {
                    current_block = 16253299934934335700;
                }
            }
            match current_block {
                17372050596571538954 => {},
                1434579379687443766 => {},
                _ => {
                    _tcc_error(
                        b"unknown register %%%s\x00" as *const u8 as *const std::os::raw::c_char,
                        get_tok_str(tok, &mut tokc),
                    );
                },
            }
        }
        match current_block {
            17372050596571538954 => {},
            _ => {
                next();
            },
        }
    } else if tok == '$' as i32 {
        /* constant value */
        next();
        asm_expr(s1, &mut e);
        (*op).type_0 = ((1 as std::os::raw::c_int) << OPT_IM32 as std::os::raw::c_int) as uint32_t;
        (*op).e = e;
        if (*op).e.sym.is_null() {
            if (*op).e.v == (*op).e.v as uint8_t as std::os::raw::c_ulong {
                (*op).type_0 |= ((1 as std::os::raw::c_int) << OPT_IM8 as std::os::raw::c_int)
                    as std::os::raw::c_uint
            }
            if (*op).e.v == (*op).e.v as int8_t as std::os::raw::c_ulong {
                (*op).type_0 |= ((1 as std::os::raw::c_int) << OPT_IM8S as std::os::raw::c_int)
                    as std::os::raw::c_uint
            }
            if (*op).e.v == (*op).e.v as uint16_t as std::os::raw::c_ulong {
                (*op).type_0 |= ((1 as std::os::raw::c_int) << OPT_IM16 as std::os::raw::c_int)
                    as std::os::raw::c_uint
            }
            if (*op).e.v != (*op).e.v as int32_t as std::os::raw::c_ulong
                && (*op).e.v != (*op).e.v as uint32_t as std::os::raw::c_ulong
            {
                (*op).type_0 =
                    ((1 as std::os::raw::c_int) << OPT_IM64 as std::os::raw::c_int) as uint32_t
            }
        }
    } else {
        /* address(reg,reg2,shift) with all variants */
        (*op).type_0 = 0x40000000 as std::os::raw::c_int as uint32_t;
        (*op).reg = -(1 as std::os::raw::c_int) as int8_t;
        (*op).reg2 = -(1 as std::os::raw::c_int) as int8_t;
        (*op).shift = 0 as std::os::raw::c_int as uint8_t;
        if tok != '(' as i32 {
            asm_expr(s1, &mut e);
            (*op).e = e
        } else {
            next();
            if tok == '%' as i32 {
                unget_tok('(' as i32);
                (*op).e.v = 0 as std::os::raw::c_int as uint64_t;
                (*op).e.sym = 0 as *mut Sym
            } else {
                /* bracketed offset expression */
                asm_expr(s1, &mut e);
                if tok != ')' as i32 {
                    expect(b")\x00" as *const u8 as *const std::os::raw::c_char);
                }
                next();
                (*op).e.v = e.v;
                (*op).e.sym = e.sym
            }
            (*op).e.pcrel = 0 as std::os::raw::c_int
        }
        if tok == '(' as i32 {
            let mut type_0: std::os::raw::c_uint = 0 as std::os::raw::c_int as std::os::raw::c_uint;
            next();
            if tok != ',' as i32 {
                (*op).reg = asm_parse_reg(&mut type_0) as int8_t
            }
            if tok == ',' as i32 {
                next();
                if tok != ',' as i32 {
                    (*op).reg2 = asm_parse_reg(&mut type_0) as int8_t
                }
                if tok == ',' as i32 {
                    next();
                    (*op).shift = get_reg_shift(s1) as uint8_t
                }
            }
            if type_0
                & ((1 as std::os::raw::c_int) << OPT_REG32 as std::os::raw::c_int)
                    as std::os::raw::c_uint
                != 0
            {
                (*op).type_0 |= ((0x40000000 as std::os::raw::c_int) << 1 as std::os::raw::c_int)
                    as std::os::raw::c_uint
            }
            skip(')' as i32);
        }
        if (*op).reg as std::os::raw::c_int == -(1 as std::os::raw::c_int)
            && (*op).reg2 as std::os::raw::c_int == -(1 as std::os::raw::c_int)
        {
            (*op).type_0 |= ((1 as std::os::raw::c_int) << OPT_ADDR as std::os::raw::c_int)
                as std::os::raw::c_uint
        }
    }
    (*op).type_0 |= indir as std::os::raw::c_uint;
}
/* XXX: unify with C code output ? */
#[no_mangle]
pub unsafe extern "C" fn gen_expr32(mut pe: *mut ExprValue) {
    if (*pe).pcrel != 0 {
        /* If PC-relative, always set VT_SYM, even without symbol,
        so as to force a relocation to be emitted.  */
        gen_addrpc32(
            0x200 as std::os::raw::c_int,
            (*pe).sym,
            (*pe).v as std::os::raw::c_int,
        );
    } else {
        gen_addr32(
            if !(*pe).sym.is_null() {
                0x200 as std::os::raw::c_int
            } else {
                0 as std::os::raw::c_int
            },
            (*pe).sym,
            (*pe).v as std::os::raw::c_int,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn gen_expr64(mut pe: *mut ExprValue) {
    gen_addr64(
        if !(*pe).sym.is_null() {
            0x200 as std::os::raw::c_int
        } else {
            0 as std::os::raw::c_int
        },
        (*pe).sym,
        (*pe).v as int64_t,
    );
}
/* XXX: unify with C code output ? */
unsafe extern "C" fn gen_disp32(mut pe: *mut ExprValue) {
    let mut sym: *mut Sym = (*pe).sym;
    let mut esym: *mut Elf64_Sym = elfsym(sym);
    if !esym.is_null()
        && (*esym).st_shndx as std::os::raw::c_int == (*(*tcc_state).cur_text_section).sh_num
    {
        /* same section: we can output an absolute value. Note
        that the TCC compiler behaves differently here because
        it always outputs a relocation to ease (future) code
        elimination in the linker */
        gen_le32(
            (*pe)
                .v
                .wrapping_add((*esym).st_value)
                .wrapping_sub(ind as std::os::raw::c_ulong)
                .wrapping_sub(4 as std::os::raw::c_int as std::os::raw::c_ulong)
                as std::os::raw::c_int,
        );
    } else {
        if !sym.is_null() && (*sym).type_0.t == 0 as std::os::raw::c_int {
            (*sym).type_0.t = 6 as std::os::raw::c_int;
            (*sym).type_0.ref_0 = 0 as *mut Sym
        }
        gen_addrpc32(
            0x200 as std::os::raw::c_int,
            sym,
            (*pe).v as std::os::raw::c_int,
        );
    };
}
/* generate the modrm operand */
#[inline]
unsafe extern "C" fn asm_modrm(
    mut reg: std::os::raw::c_int,
    mut op: *mut Operand,
) -> std::os::raw::c_int {
    let mut mod_0: std::os::raw::c_int = 0;
    let mut reg1: std::os::raw::c_int = 0;
    let mut reg2: std::os::raw::c_int = 0;
    let mut sib_reg1: std::os::raw::c_int = 0;
    if (*op).type_0
        & ((1 as std::os::raw::c_int) << OPT_REG8 as std::os::raw::c_int
            | (1 as std::os::raw::c_int) << OPT_REG16 as std::os::raw::c_int
            | (1 as std::os::raw::c_int) << OPT_REG32 as std::os::raw::c_int
            | (1 as std::os::raw::c_int) << OPT_REG64 as std::os::raw::c_int
            | (1 as std::os::raw::c_int) << OPT_MMX as std::os::raw::c_int
            | (1 as std::os::raw::c_int) << OPT_SSE as std::os::raw::c_int)
            as std::os::raw::c_uint
        != 0
    {
        g(0xc0 as std::os::raw::c_int
            + (reg << 3 as std::os::raw::c_int)
            + (*op).reg as std::os::raw::c_int);
    } else if (*op).reg as std::os::raw::c_int == -(1 as std::os::raw::c_int)
        && (*op).reg2 as std::os::raw::c_int == -(1 as std::os::raw::c_int)
    {
        /* displacement only */
        g(0x4 as std::os::raw::c_int + (reg << 3 as std::os::raw::c_int));
        g(0x25 as std::os::raw::c_int);
        gen_expr32(&mut (*op).e);
    } else if (*op).reg as std::os::raw::c_int == -(2 as std::os::raw::c_int) {
        let mut pe: *mut ExprValue = &mut (*op).e;
        g(0x5 as std::os::raw::c_int + (reg << 3 as std::os::raw::c_int));
        gen_addrpc32(
            if !(*pe).sym.is_null() {
                0x200 as std::os::raw::c_int
            } else {
                0 as std::os::raw::c_int
            },
            (*pe).sym,
            (*pe).v as std::os::raw::c_int,
        );
        return ind;
    } else {
        sib_reg1 = (*op).reg as std::os::raw::c_int;
        /* fist compute displacement encoding */
        if sib_reg1 == -(1 as std::os::raw::c_int) {
            sib_reg1 = 5 as std::os::raw::c_int;
            mod_0 = 0 as std::os::raw::c_int
        } else if (*op).e.v == 0 as std::os::raw::c_int as std::os::raw::c_ulong
            && (*op).e.sym.is_null()
            && (*op).reg as std::os::raw::c_int != 5 as std::os::raw::c_int
        {
            mod_0 = 0 as std::os::raw::c_int
        } else if (*op).e.v == (*op).e.v as int8_t as std::os::raw::c_ulong && (*op).e.sym.is_null()
        {
            mod_0 = 0x40 as std::os::raw::c_int
        } else {
            mod_0 = 0x80 as std::os::raw::c_int
        }
        /* compute if sib byte needed */
        reg1 = (*op).reg as std::os::raw::c_int;
        if (*op).reg2 as std::os::raw::c_int != -(1 as std::os::raw::c_int) {
            reg1 = 4 as std::os::raw::c_int
        }
        g(mod_0 + (reg << 3 as std::os::raw::c_int) + reg1);
        if reg1 == 4 as std::os::raw::c_int {
            /* add sib byte */
            reg2 = (*op).reg2 as std::os::raw::c_int; /* indicate no index */
            if reg2 == -(1 as std::os::raw::c_int) {
                reg2 = 4 as std::os::raw::c_int
            }
            g(
                (((*op).shift as std::os::raw::c_int) << 6 as std::os::raw::c_int)
                    + (reg2 << 3 as std::os::raw::c_int)
                    + sib_reg1,
            );
        }
        /* add offset */
        if mod_0 == 0x40 as std::os::raw::c_int {
            g((*op).e.v as std::os::raw::c_int);
        } else if mod_0 == 0x80 as std::os::raw::c_int
            || (*op).reg as std::os::raw::c_int == -(1 as std::os::raw::c_int)
        {
            gen_expr32(&mut (*op).e);
        }
    }
    return 0 as std::os::raw::c_int;
}
unsafe extern "C" fn asm_rex(
    mut width64: std::os::raw::c_int,
    mut ops: *mut Operand,
    mut nb_ops: std::os::raw::c_int,
    mut op_type: *mut std::os::raw::c_int,
    mut regi: std::os::raw::c_int,
    mut rmi: std::os::raw::c_int,
) {
    let mut rex: std::os::raw::c_uchar = if width64 != 0 {
        0x48 as std::os::raw::c_int
    } else {
        0 as std::os::raw::c_int
    } as std::os::raw::c_uchar;
    let mut saw_high_8bit: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut i: std::os::raw::c_int = 0;
    if rmi == -(1 as std::os::raw::c_int) {
        /* No mod/rm byte, but we might have a register op nevertheless
        (we will add it to the opcode later).  */
        i = 0 as std::os::raw::c_int;
        while i < nb_ops {
            if *op_type.offset(i as isize)
                & ((1 as std::os::raw::c_int) << OPT_REG8 as std::os::raw::c_int
                    | (1 as std::os::raw::c_int) << OPT_REG16 as std::os::raw::c_int
                    | (1 as std::os::raw::c_int) << OPT_REG32 as std::os::raw::c_int
                    | (1 as std::os::raw::c_int) << OPT_REG64 as std::os::raw::c_int
                    | (1 as std::os::raw::c_int) << OPT_ST as std::os::raw::c_int)
                != 0
            {
                if (*ops.offset(i as isize)).reg as std::os::raw::c_int >= 8 as std::os::raw::c_int
                {
                    rex = (rex as std::os::raw::c_int | 0x41 as std::os::raw::c_int)
                        as std::os::raw::c_uchar;
                    let ref mut fresh2 = (*ops.offset(i as isize)).reg;
                    *fresh2 = (*fresh2 as std::os::raw::c_int - 8 as std::os::raw::c_int) as int8_t
                } else if (*ops.offset(i as isize)).type_0
                    & ((1 as std::os::raw::c_int) << OPT_REG8_LOW as std::os::raw::c_int)
                        as std::os::raw::c_uint
                    != 0
                {
                    rex = (rex as std::os::raw::c_int | 0x40 as std::os::raw::c_int)
                        as std::os::raw::c_uchar
                } else if (*ops.offset(i as isize)).type_0
                    & ((1 as std::os::raw::c_int) << OPT_REG8 as std::os::raw::c_int)
                        as std::os::raw::c_uint
                    != 0
                    && (*ops.offset(i as isize)).reg as std::os::raw::c_int
                        >= 4 as std::os::raw::c_int
                {
                    /* An 8 bit reg >= 4 without REG8 is ah/ch/dh/bh */
                    saw_high_8bit = (*ops.offset(i as isize)).reg as std::os::raw::c_int
                }
                break;
            } else {
                i += 1
            }
        }
    } else {
        if regi != -(1 as std::os::raw::c_int) {
            if (*ops.offset(regi as isize)).reg as std::os::raw::c_int >= 8 as std::os::raw::c_int {
                rex = (rex as std::os::raw::c_int | 0x44 as std::os::raw::c_int)
                    as std::os::raw::c_uchar;
                let ref mut fresh3 = (*ops.offset(regi as isize)).reg;
                *fresh3 = (*fresh3 as std::os::raw::c_int - 8 as std::os::raw::c_int) as int8_t
            } else if (*ops.offset(regi as isize)).type_0
                & ((1 as std::os::raw::c_int) << OPT_REG8_LOW as std::os::raw::c_int)
                    as std::os::raw::c_uint
                != 0
            {
                rex = (rex as std::os::raw::c_int | 0x40 as std::os::raw::c_int)
                    as std::os::raw::c_uchar
            } else if (*ops.offset(regi as isize)).type_0
                & ((1 as std::os::raw::c_int) << OPT_REG8 as std::os::raw::c_int)
                    as std::os::raw::c_uint
                != 0
                && (*ops.offset(regi as isize)).reg as std::os::raw::c_int
                    >= 4 as std::os::raw::c_int
            {
                /* An 8 bit reg >= 4 without REG8 is ah/ch/dh/bh */
                saw_high_8bit = (*ops.offset(regi as isize)).reg as std::os::raw::c_int
            }
        }
        if (*ops.offset(rmi as isize)).type_0
            & ((1 as std::os::raw::c_int) << OPT_REG8 as std::os::raw::c_int
                | (1 as std::os::raw::c_int) << OPT_REG16 as std::os::raw::c_int
                | (1 as std::os::raw::c_int) << OPT_REG32 as std::os::raw::c_int
                | (1 as std::os::raw::c_int) << OPT_REG64 as std::os::raw::c_int
                | (1 as std::os::raw::c_int) << OPT_MMX as std::os::raw::c_int
                | (1 as std::os::raw::c_int) << OPT_SSE as std::os::raw::c_int
                | (1 as std::os::raw::c_int) << OPT_CR as std::os::raw::c_int
                | 0x40000000 as std::os::raw::c_int) as std::os::raw::c_uint
            != 0
        {
            if (*ops.offset(rmi as isize)).reg as std::os::raw::c_int >= 8 as std::os::raw::c_int {
                rex = (rex as std::os::raw::c_int | 0x41 as std::os::raw::c_int)
                    as std::os::raw::c_uchar;
                let ref mut fresh4 = (*ops.offset(rmi as isize)).reg;
                *fresh4 = (*fresh4 as std::os::raw::c_int - 8 as std::os::raw::c_int) as int8_t
            } else if (*ops.offset(rmi as isize)).type_0
                & ((1 as std::os::raw::c_int) << OPT_REG8_LOW as std::os::raw::c_int)
                    as std::os::raw::c_uint
                != 0
            {
                rex = (rex as std::os::raw::c_int | 0x40 as std::os::raw::c_int)
                    as std::os::raw::c_uchar
            } else if (*ops.offset(rmi as isize)).type_0
                & ((1 as std::os::raw::c_int) << OPT_REG8 as std::os::raw::c_int)
                    as std::os::raw::c_uint
                != 0
                && (*ops.offset(rmi as isize)).reg as std::os::raw::c_int
                    >= 4 as std::os::raw::c_int
            {
                /* An 8 bit reg >= 4 without REG8 is ah/ch/dh/bh */
                saw_high_8bit = (*ops.offset(rmi as isize)).reg as std::os::raw::c_int
            }
        }
        if (*ops.offset(rmi as isize)).type_0
            & 0x40000000 as std::os::raw::c_int as std::os::raw::c_uint
            != 0
            && (*ops.offset(rmi as isize)).reg2 as std::os::raw::c_int >= 8 as std::os::raw::c_int
        {
            rex =
                (rex as std::os::raw::c_int | 0x42 as std::os::raw::c_int) as std::os::raw::c_uchar;
            let ref mut fresh5 = (*ops.offset(rmi as isize)).reg2;
            *fresh5 = (*fresh5 as std::os::raw::c_int - 8 as std::os::raw::c_int) as int8_t
        }
    }
    if rex != 0 {
        if saw_high_8bit != 0 {
            _tcc_error(
                b"can\'t encode register %%%ch when REX prefix is required\x00" as *const u8
                    as *const std::os::raw::c_char,
                (*::std::mem::transmute::<&[u8; 5], &[std::os::raw::c_char; 5]>(b"acdb\x00"))
                    [(saw_high_8bit - 4 as std::os::raw::c_int) as usize]
                    as std::os::raw::c_int,
            );
        }
        g(rex as std::os::raw::c_int);
    };
}
unsafe extern "C" fn maybe_print_stats() {
    static mut already: std::os::raw::c_int = 0;
    if 0 as std::os::raw::c_int != 0 && already == 0 {
        /* print stats about opcodes */
        let mut pa: *const ASMInstr = 0 as *const ASMInstr;
        let mut freq: [std::os::raw::c_int; 4] = [0; 4];
        let mut op_vals: [std::os::raw::c_int; 500] = [0; 500];
        let mut nb_op_vals: std::os::raw::c_int = 0;
        let mut i: std::os::raw::c_int = 0;
        let mut j: std::os::raw::c_int = 0;
        already = 1 as std::os::raw::c_int;
        nb_op_vals = 0 as std::os::raw::c_int;
        memset(
            freq.as_mut_ptr() as *mut std::os::raw::c_void,
            0 as std::os::raw::c_int,
            ::std::mem::size_of::<[std::os::raw::c_int; 4]>() as std::os::raw::c_ulong,
        );
        pa = asm_instrs.as_ptr();
        while (*pa).sym as std::os::raw::c_int != 0 as std::os::raw::c_int {
            let mut current_block_7: u64;
            freq[(*pa).nb_ops as usize] += 1;
            //}
            j = 0 as std::os::raw::c_int;
            loop {
                if !(j < nb_op_vals) {
                    current_block_7 = 2979737022853876585;
                    break;
                }
                //for(i=0;i<pa->nb_ops;i++) {
                //if (pa->op_type[i] == op_vals[j])
                if (*pa).instr_type as std::os::raw::c_int == op_vals[j as usize] {
                    current_block_7 = 17860125682698302841;
                    break;
                }
                j += 1
            }
            match current_block_7 {
                2979737022853876585 => {
                    //op_vals[nb_op_vals++] = pa->op_type[i];
                    let fresh6 = nb_op_vals;
                    nb_op_vals = nb_op_vals + 1;
                    op_vals[fresh6 as usize] = (*pa).instr_type as std::os::raw::c_int
                },
                _ => {},
            }
            pa = pa.offset(1)
        }
        i = 0 as std::os::raw::c_int;
        while i < nb_op_vals {
            let mut v: std::os::raw::c_int = op_vals[i as usize];
            //if ((v & (v - 1)) != 0)
            printf(
                b"%3d: %08x\n\x00" as *const u8 as *const std::os::raw::c_char,
                i,
                v,
            ); /* decoded op type */
            i += 1
        } /* OR of all operand types */
        printf(
            b"size=%d nb=%d f0=%d f1=%d f2=%d f3=%d\n\x00" as *const u8
                as *const std::os::raw::c_char,
            ::std::mem::size_of::<[ASMInstr; 332]>() as std::os::raw::c_ulong
                as std::os::raw::c_int,
            ::std::mem::size_of::<[ASMInstr; 332]>() as std::os::raw::c_ulong
                as std::os::raw::c_int
                / ::std::mem::size_of::<ASMInstr>() as std::os::raw::c_ulong as std::os::raw::c_int,
            freq[0 as std::os::raw::c_int as usize],
            freq[1 as std::os::raw::c_int as usize],
            freq[2 as std::os::raw::c_int as usize],
            freq[3 as std::os::raw::c_int as usize],
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn asm_opcode(mut s1: *mut TCCState, mut opcode: std::os::raw::c_int) {
    let mut current_block: u64;
    let mut pa: *const ASMInstr = 0 as *const ASMInstr;
    let mut i: std::os::raw::c_int = 0;
    let mut modrm_index: std::os::raw::c_int = 0;
    let mut modreg_index: std::os::raw::c_int = 0;
    let mut reg: std::os::raw::c_int = 0;
    let mut v: std::os::raw::c_int = 0;
    let mut op1: std::os::raw::c_int = 0;
    let mut seg_prefix: std::os::raw::c_int = 0;
    let mut pc: std::os::raw::c_int = 0;
    let mut nb_ops: std::os::raw::c_int = 0;
    let mut s: std::os::raw::c_int = 0;
    let mut ops: [Operand; 3] = [Operand {
        type_0: 0,
        reg: 0,
        reg2: 0,
        shift: 0,
        e: ExprValue {
            v: 0,
            sym: 0 as *mut Sym,
            pcrel: 0,
        },
    }; 3];
    let mut pop: *mut Operand = 0 as *mut Operand;
    let mut op_type: [std::os::raw::c_int; 3] = [0; 3];
    let mut alltypes: std::os::raw::c_int = 0;
    let mut autosize: std::os::raw::c_int = 0;
    let mut p66: std::os::raw::c_int = 0;
    let mut rex64: std::os::raw::c_int = 0;
    maybe_print_stats();
    /* force synthetic ';' after prefix instruction, so we can handle */
    /* one-line things like "rep stosb" instead of only "rep\nstosb" */
    if opcode >= TOK_ASM_wait as std::os::raw::c_int
        && opcode <= TOK_ASM_repnz as std::os::raw::c_int
    {
        unget_tok(';' as i32);
    }
    /* get operands */
    pop = ops.as_mut_ptr(); /* avoid warning */
    nb_ops = 0 as std::os::raw::c_int;
    seg_prefix = 0 as std::os::raw::c_int;
    alltypes = 0 as std::os::raw::c_int;
    while !(tok == ';' as i32 || tok == 10 as std::os::raw::c_int) {
        if nb_ops >= 3 as std::os::raw::c_int {
            _tcc_error(
                b"incorrect number of operands\x00" as *const u8 as *const std::os::raw::c_char,
            );
        }
        parse_operand(s1, pop);
        if tok == ':' as i32 {
            if (*pop).type_0
                != ((1 as std::os::raw::c_int) << OPT_SEG as std::os::raw::c_int)
                    as std::os::raw::c_uint
                || seg_prefix != 0
            {
                _tcc_error(b"incorrect prefix\x00" as *const u8 as *const std::os::raw::c_char);
            }
            seg_prefix = segment_prefixes[(*pop).reg as usize] as std::os::raw::c_int;
            next();
            parse_operand(s1, pop);
            if (*pop).type_0 & 0x40000000 as std::os::raw::c_int as std::os::raw::c_uint == 0 {
                _tcc_error(
                    b"segment prefix must be followed by memory reference\x00" as *const u8
                        as *const std::os::raw::c_char,
                );
            }
        }
        pop = pop.offset(1);
        nb_ops += 1;
        if tok != ',' as i32 {
            break;
        }
        next();
    }
    s = 0 as std::os::raw::c_int;
    loop {
        let mut current_block_52: u64;
        /* optimize matching by using a lookup table (no hashing is needed
        !) */
        pa = asm_instrs.as_ptr();
        's_157: while (*pa).sym as std::os::raw::c_int != 0 as std::os::raw::c_int {
            let mut it: std::os::raw::c_int =
                (*pa).instr_type as std::os::raw::c_int & 0x70 as std::os::raw::c_int;
            s = 0 as std::os::raw::c_int;
            if it == 0x40 as std::os::raw::c_int {
                v = opcode - (*pa).sym as std::os::raw::c_int;
                if !((v as std::os::raw::c_uint)
                    < (8 as std::os::raw::c_int * 6 as std::os::raw::c_int) as std::os::raw::c_uint
                    && v % 6 as std::os::raw::c_int == 0 as std::os::raw::c_int)
                {
                    current_block_52 = 15897653523371991391;
                } else {
                    current_block_52 = 14001958660280927786;
                }
            } else if it == 0x30 as std::os::raw::c_int {
                if !(opcode >= (*pa).sym as std::os::raw::c_int
                    && opcode
                        < (*pa).sym as std::os::raw::c_int
                            + 8 as std::os::raw::c_int * 5 as std::os::raw::c_int)
                {
                    current_block_52 = 15897653523371991391;
                } else {
                    s = (opcode - (*pa).sym as std::os::raw::c_int) % 5 as std::os::raw::c_int;
                    if (*pa).instr_type as std::os::raw::c_int
                        & (0x1 as std::os::raw::c_int | 0x1000 as std::os::raw::c_int)
                        == 0x1000 as std::os::raw::c_int
                    {
                        /* We need to reject the xxxb opcodes that we accepted above.
                        Note that pa->sym for WLX opcodes is the 'w' token,
                        to get the 'b' token subtract one.  */
                        if (opcode - (*pa).sym as std::os::raw::c_int + 1 as std::os::raw::c_int)
                            % 5 as std::os::raw::c_int
                            == 0 as std::os::raw::c_int
                        {
                            current_block_52 = 15897653523371991391;
                        } else {
                            s += 1;
                            current_block_52 = 14001958660280927786;
                        }
                    } else {
                        current_block_52 = 14001958660280927786;
                    }
                }
            } else if it == 0x20 as std::os::raw::c_int {
                if !(opcode >= (*pa).sym as std::os::raw::c_int
                    && opcode
                        < (*pa).sym as std::os::raw::c_int
                            + 7 as std::os::raw::c_int * 5 as std::os::raw::c_int)
                {
                    current_block_52 = 15897653523371991391;
                } else {
                    s = (opcode - (*pa).sym as std::os::raw::c_int) % 5 as std::os::raw::c_int;
                    current_block_52 = 14001958660280927786;
                }
            } else if it == 0x50 as std::os::raw::c_int {
                if !(opcode >= (*pa).sym as std::os::raw::c_int
                    && opcode < (*pa).sym as std::os::raw::c_int + 30 as std::os::raw::c_int)
                {
                    current_block_52 = 15897653523371991391;
                } else {
                    /* cmovxx is a test opcode but accepts multiple sizes.
                    The suffixes aren't encoded in the table, instead we
                    simply force size autodetection always and deal with suffixed
                    variants below when we don't find e.g. "cmovzl".  */
                    if (*pa).instr_type as std::os::raw::c_int & 0x1000 as std::os::raw::c_int != 0
                    {
                        s = 5 as std::os::raw::c_int - 1 as std::os::raw::c_int
                    }
                    current_block_52 = 14001958660280927786;
                }
            } else if (*pa).instr_type as std::os::raw::c_int & 0x1 as std::os::raw::c_int != 0 {
                /* Some instructions don't have the full size but only
                bwl form.  insb e.g. */
                if (*pa).instr_type as std::os::raw::c_int & 0x1000 as std::os::raw::c_int
                    != 0x1000 as std::os::raw::c_int
                    && !(opcode >= (*pa).sym as std::os::raw::c_int
                        && opcode
                            < (*pa).sym as std::os::raw::c_int + 5 as std::os::raw::c_int
                                - 1 as std::os::raw::c_int)
                {
                    current_block_52 = 15897653523371991391;
                } else if !(opcode >= (*pa).sym as std::os::raw::c_int
                    && opcode < (*pa).sym as std::os::raw::c_int + 5 as std::os::raw::c_int)
                {
                    current_block_52 = 15897653523371991391;
                } else {
                    s = opcode - (*pa).sym as std::os::raw::c_int;
                    current_block_52 = 14001958660280927786;
                }
            } else if (*pa).instr_type as std::os::raw::c_int & 0x1000 as std::os::raw::c_int != 0 {
                if !(opcode >= (*pa).sym as std::os::raw::c_int
                    && opcode
                        < (*pa).sym as std::os::raw::c_int + 5 as std::os::raw::c_int
                            - 1 as std::os::raw::c_int)
                {
                    current_block_52 = 15897653523371991391;
                } else {
                    s = opcode - (*pa).sym as std::os::raw::c_int + 1 as std::os::raw::c_int;
                    current_block_52 = 14001958660280927786;
                }
            } else if (*pa).sym as std::os::raw::c_int != opcode {
                current_block_52 = 15897653523371991391;
            } else {
                current_block_52 = 14001958660280927786;
            }
            match current_block_52 {
                14001958660280927786 => {
                    if !((*pa).nb_ops as std::os::raw::c_int != nb_ops) {
                        /* Special case for moves.  Selecting the IM64->REG64 form
                        should only be done if we really have an >32bit imm64, and that
                        is hardcoded.  Ignore it here.  */
                        if !((*pa).opcode as std::os::raw::c_int == 0xb0 as std::os::raw::c_int
                            && ops[0 as std::os::raw::c_int as usize].type_0
                                != ((1 as std::os::raw::c_int) << OPT_IM64 as std::os::raw::c_int)
                                    as std::os::raw::c_uint
                            && ops[1 as std::os::raw::c_int as usize].type_0
                                & ((1 as std::os::raw::c_int) << OPT_REG8 as std::os::raw::c_int
                                    | (1 as std::os::raw::c_int)
                                        << OPT_REG16 as std::os::raw::c_int
                                    | (1 as std::os::raw::c_int)
                                        << OPT_REG32 as std::os::raw::c_int
                                    | (1 as std::os::raw::c_int)
                                        << OPT_REG64 as std::os::raw::c_int)
                                    as std::os::raw::c_uint
                                == ((1 as std::os::raw::c_int) << OPT_REG64 as std::os::raw::c_int)
                                    as std::os::raw::c_uint
                            && (*pa).instr_type as std::os::raw::c_int
                                & 0x100 as std::os::raw::c_int
                                == 0)
                        {
                            /* now decode and check each operand */
                            alltypes = 0 as std::os::raw::c_int;
                            i = 0 as std::os::raw::c_int;
                            loop {
                                if !(i < nb_ops) {
                                    break 's_157;
                                }
                                let mut op1_0: std::os::raw::c_int = 0;
                                let mut op2: std::os::raw::c_int = 0;
                                op1_0 = (*pa).op_type[i as usize] as std::os::raw::c_int;
                                op2 = op1_0 & 0x1f as std::os::raw::c_int;
                                match op2 {
                                    24 => {
                                        v = (1 as std::os::raw::c_int)
                                            << OPT_IM8 as std::os::raw::c_int
                                            | (1 as std::os::raw::c_int)
                                                << OPT_IM16 as std::os::raw::c_int
                                            | (1 as std::os::raw::c_int)
                                                << OPT_IM32 as std::os::raw::c_int
                                    },
                                    25 => {
                                        v = (1 as std::os::raw::c_int)
                                            << OPT_REG8 as std::os::raw::c_int
                                            | (1 as std::os::raw::c_int)
                                                << OPT_REG16 as std::os::raw::c_int
                                            | (1 as std::os::raw::c_int)
                                                << OPT_REG32 as std::os::raw::c_int
                                            | (1 as std::os::raw::c_int)
                                                << OPT_REG64 as std::os::raw::c_int
                                    },
                                    26 => {
                                        v = (1 as std::os::raw::c_int)
                                            << OPT_REG16 as std::os::raw::c_int
                                            | (1 as std::os::raw::c_int)
                                                << OPT_REG32 as std::os::raw::c_int
                                            | (1 as std::os::raw::c_int)
                                                << OPT_REG64 as std::os::raw::c_int
                                    },
                                    27 => {
                                        v = (1 as std::os::raw::c_int)
                                            << OPT_IM16 as std::os::raw::c_int
                                            | (1 as std::os::raw::c_int)
                                                << OPT_IM32 as std::os::raw::c_int
                                    },
                                    28 => {
                                        v = (1 as std::os::raw::c_int)
                                            << OPT_MMX as std::os::raw::c_int
                                            | (1 as std::os::raw::c_int)
                                                << OPT_SSE as std::os::raw::c_int
                                    },
                                    29 | 30 => {
                                        v = (1 as std::os::raw::c_int)
                                            << OPT_ADDR as std::os::raw::c_int
                                    },
                                    _ => v = (1 as std::os::raw::c_int) << op2,
                                }
                                if op1_0 & OPT_EA as std::os::raw::c_int != 0 {
                                    v |= 0x40000000 as std::os::raw::c_int
                                }
                                op_type[i as usize] = v;
                                if ops[i as usize].type_0 & v as std::os::raw::c_uint
                                    == 0 as std::os::raw::c_int as std::os::raw::c_uint
                                {
                                    break;
                                }
                                alltypes = (alltypes as std::os::raw::c_uint
                                    | ops[i as usize].type_0)
                                    as std::os::raw::c_int;
                                i += 1
                            }
                        }
                    }
                },
                _ => {},
            }
            pa = pa.offset(1)
        }
        if (*pa).sym as std::os::raw::c_int == 0 as std::os::raw::c_int {
            if opcode >= TOK_ASM_clc as std::os::raw::c_int
                && opcode <= TOK_ASM_emms as std::os::raw::c_int
            {
                let mut b: std::os::raw::c_int = 0;
                b = op0_codes[(opcode - TOK_ASM_clc as std::os::raw::c_int) as usize]
                    as std::os::raw::c_int;
                if b & 0xff00 as std::os::raw::c_int != 0 {
                    g(b >> 8 as std::os::raw::c_int);
                }
                g(b);
                return;
            } else if opcode <= TOK_ASM_subps as std::os::raw::c_int {
                _tcc_error(
                    b"bad operand with opcode \'%s\'\x00" as *const u8
                        as *const std::os::raw::c_char,
                    get_tok_str(opcode, 0 as *mut CValue),
                );
            } else {
                /* Special case for cmovcc, we accept size suffixes but ignore
                them, but we don't want them to blow up our tables.  */
                let mut ts: *mut TokenSym =
                    *table_ident.offset((opcode - 256 as std::os::raw::c_int) as isize);
                if (*ts).len >= 6 as std::os::raw::c_int
                    && !strchr(
                        b"wlq\x00" as *const u8 as *const std::os::raw::c_char,
                        *(*ts)
                            .str_0
                            .as_mut_ptr()
                            .offset(((*ts).len - 1 as std::os::raw::c_int) as isize)
                            as std::os::raw::c_int,
                    )
                    .is_null()
                    && memcmp(
                        (*ts).str_0.as_mut_ptr() as *const std::os::raw::c_void,
                        b"cmov\x00" as *const u8 as *const std::os::raw::c_char
                            as *const std::os::raw::c_void,
                        4 as std::os::raw::c_int as std::os::raw::c_ulong,
                    ) == 0
                {
                    opcode = (*tok_alloc(
                        (*ts).str_0.as_mut_ptr(),
                        (*ts).len - 1 as std::os::raw::c_int,
                    ))
                    .tok
                } else {
                    _tcc_error(
                        b"unknown opcode \'%s\'\x00" as *const u8 as *const std::os::raw::c_char,
                        (*ts).str_0.as_mut_ptr(),
                    );
                }
            }
        } else {
            /* if the size is unknown, then evaluate it (OPC_B or OPC_WL case) */
            autosize = 5 as std::os::raw::c_int - 1 as std::os::raw::c_int;
            /* XXX the autosize should rather be zero, to not have to adjust this
            all the time.  */
            if (*pa).instr_type as std::os::raw::c_int
                & (0x1 as std::os::raw::c_int | 0x1000 as std::os::raw::c_int)
                == 0x1 as std::os::raw::c_int
            {
                autosize = 5 as std::os::raw::c_int - 2 as std::os::raw::c_int
            }
            if s == autosize {
                /* Check for register operands providing hints about the size.
                Start from the end, i.e. destination operands.  This matters
                only for opcodes accepting different sized registers, lar and lsl
                are such opcodes.  */
                i = nb_ops - 1 as std::os::raw::c_int;
                while s == autosize && i >= 0 as std::os::raw::c_int {
                    if ops[i as usize].type_0
                        & ((1 as std::os::raw::c_int) << OPT_REG8 as std::os::raw::c_int
                            | (1 as std::os::raw::c_int) << OPT_REG16 as std::os::raw::c_int
                            | (1 as std::os::raw::c_int) << OPT_REG32 as std::os::raw::c_int
                            | (1 as std::os::raw::c_int) << OPT_REG64 as std::os::raw::c_int)
                            as std::os::raw::c_uint
                        != 0
                        && op_type[i as usize]
                            & ((1 as std::os::raw::c_int) << OPT_CL as std::os::raw::c_int
                                | (1 as std::os::raw::c_int) << OPT_DX as std::os::raw::c_int)
                            == 0
                    {
                        s = reg_to_size[(ops[i as usize].type_0
                            & ((1 as std::os::raw::c_int) << OPT_REG8 as std::os::raw::c_int
                                | (1 as std::os::raw::c_int) << OPT_REG16 as std::os::raw::c_int
                                | (1 as std::os::raw::c_int) << OPT_REG32 as std::os::raw::c_int
                                | (1 as std::os::raw::c_int) << OPT_REG64 as std::os::raw::c_int)
                                as std::os::raw::c_uint)
                            as usize] as std::os::raw::c_int
                    }
                    i -= 1
                }
                if s == autosize {
                    if (opcode == TOK_ASM_push as std::os::raw::c_int
                        || opcode == TOK_ASM_pop as std::os::raw::c_int)
                        && ops[0 as std::os::raw::c_int as usize].type_0
                            & ((1 as std::os::raw::c_int) << OPT_SEG as std::os::raw::c_int
                                | (1 as std::os::raw::c_int) << OPT_IM8S as std::os::raw::c_int
                                | (1 as std::os::raw::c_int) << OPT_IM32 as std::os::raw::c_int)
                                as std::os::raw::c_uint
                            != 0
                    {
                        s = 2 as std::os::raw::c_int
                    } else if (opcode == TOK_ASM_push as std::os::raw::c_int
                        || opcode == TOK_ASM_pop as std::os::raw::c_int)
                        && ops[0 as std::os::raw::c_int as usize].type_0
                            & 0x40000000 as std::os::raw::c_int as std::os::raw::c_uint
                            != 0
                    {
                        s = 5 as std::os::raw::c_int - 2 as std::os::raw::c_int
                    } else {
                        _tcc_error(
                            b"cannot infer opcode suffix\x00" as *const u8
                                as *const std::os::raw::c_char,
                        );
                    }
                }
            }
            /* Generate addr32 prefix if needed */
            i = 0 as std::os::raw::c_int;
            while i < nb_ops {
                if ops[i as usize].type_0
                    & ((0x40000000 as std::os::raw::c_int) << 1 as std::os::raw::c_int)
                        as std::os::raw::c_uint
                    != 0
                {
                    g(0x67 as std::os::raw::c_int);
                    break;
                } else {
                    i += 1
                }
            }
            /* generate data16 prefix if needed */
            p66 = 0 as std::os::raw::c_int;
            if s == 1 as std::os::raw::c_int {
                p66 = 1 as std::os::raw::c_int
            } else {
                /* accepting mmx+sse in all operands --> needs 0x66 to
                switch to sse mode.  Accepting only sse in an operand --> is
                already SSE insn and needs 0x66/f2/f3 handling.  */
                i = 0 as std::os::raw::c_int;
                while i < nb_ops {
                    if op_type[i as usize]
                        & ((1 as std::os::raw::c_int) << OPT_MMX as std::os::raw::c_int
                            | (1 as std::os::raw::c_int) << OPT_SSE as std::os::raw::c_int)
                        == (1 as std::os::raw::c_int) << OPT_MMX as std::os::raw::c_int
                            | (1 as std::os::raw::c_int) << OPT_SSE as std::os::raw::c_int
                        && ops[i as usize].type_0
                            & ((1 as std::os::raw::c_int) << OPT_SSE as std::os::raw::c_int)
                                as std::os::raw::c_uint
                            != 0
                    {
                        p66 = 1 as std::os::raw::c_int
                    }
                    i += 1
                }
            }
            if p66 != 0 {
                g(0x66 as std::os::raw::c_int);
            }
            rex64 = 0 as std::os::raw::c_int;
            if (*pa).instr_type as std::os::raw::c_int & 0x200 as std::os::raw::c_int != 0 {
                rex64 = 1 as std::os::raw::c_int
            } else if s == 3 as std::os::raw::c_int
                || alltypes & (1 as std::os::raw::c_int) << OPT_REG64 as std::os::raw::c_int != 0
            {
                /* generate REX prefix */
                let mut default64: std::os::raw::c_int = 0 as std::os::raw::c_int;
                i = 0 as std::os::raw::c_int;
                while i < nb_ops {
                    if op_type[i as usize]
                        == (1 as std::os::raw::c_int) << OPT_REG64 as std::os::raw::c_int
                        && (*pa).opcode as std::os::raw::c_int != 0xb8 as std::os::raw::c_int
                    {
                        /* If only 64bit regs are accepted in one operand
                        this is a default64 instruction without need for
                        REX prefixes, except for movabs(0xb8).  */
                        default64 = 1 as std::os::raw::c_int;
                        break;
                    } else {
                        i += 1
                    }
                }
                /* XXX find better encoding for the default64 instructions.  */
                if opcode != TOK_ASM_push as std::os::raw::c_int
                    && opcode != TOK_ASM_pop as std::os::raw::c_int
                    && opcode != TOK_ASM_pushw as std::os::raw::c_int
                    && opcode != TOK_ASM_pushl as std::os::raw::c_int
                    && opcode != TOK_ASM_pushq as std::os::raw::c_int
                    && opcode != TOK_ASM_popw as std::os::raw::c_int
                    && opcode != TOK_ASM_popl as std::os::raw::c_int
                    && opcode != TOK_ASM_popq as std::os::raw::c_int
                    && opcode != TOK_ASM_call as std::os::raw::c_int
                    && opcode != TOK_ASM_jmp as std::os::raw::c_int
                    && default64 == 0
                {
                    rex64 = 1 as std::os::raw::c_int
                }
            }
            /* now generates the operation */
            if (*pa).instr_type as std::os::raw::c_int & 0x70 as std::os::raw::c_int
                == 0x10 as std::os::raw::c_int
            {
                g(0x9b as std::os::raw::c_int);
            }
            if seg_prefix != 0 {
                g(seg_prefix);
            }
            v = (*pa).opcode as std::os::raw::c_int;
            if (*pa).instr_type as std::os::raw::c_int & 0x100 as std::os::raw::c_int != 0 {
                v = (v & !(0xff as std::os::raw::c_int)) << 8 as std::os::raw::c_int
                    | 0xf00 as std::os::raw::c_int
                    | v & 0xff as std::os::raw::c_int
            }
            if (v == 0x69 as std::os::raw::c_int || v == 0x6b as std::os::raw::c_int)
                && nb_ops == 2 as std::os::raw::c_int
            {
                /* kludge for imul $im, %reg */
                nb_ops = 3 as std::os::raw::c_int; /* int $3 case */
                ops[2 as std::os::raw::c_int as usize] = ops[1 as std::os::raw::c_int as usize];
                op_type[2 as std::os::raw::c_int as usize] =
                    op_type[1 as std::os::raw::c_int as usize]
            } else if v == 0xcd as std::os::raw::c_int
                && ops[0 as std::os::raw::c_int as usize].e.v
                    == 3 as std::os::raw::c_int as std::os::raw::c_ulong
                && ops[0 as std::os::raw::c_int as usize].e.sym.is_null()
            {
                v -= 1;
                nb_ops = 0 as std::os::raw::c_int
            } else if v == 0x6 as std::os::raw::c_int || v == 0x7 as std::os::raw::c_int {
                if ops[0 as std::os::raw::c_int as usize].reg as std::os::raw::c_int
                    >= 4 as std::os::raw::c_int
                {
                    /* push/pop %fs or %gs */
                    v = 0xfa0 as std::os::raw::c_int
                        + (v - 0x6 as std::os::raw::c_int)
                        + ((ops[0 as std::os::raw::c_int as usize].reg as std::os::raw::c_int
                            - 4 as std::os::raw::c_int)
                            << 3 as std::os::raw::c_int)
                } else {
                    v += (ops[0 as std::os::raw::c_int as usize].reg as std::os::raw::c_int)
                        << 3 as std::os::raw::c_int
                }
                nb_ops = 0 as std::os::raw::c_int
            } else if v <= 0x5 as std::os::raw::c_int {
                /* arith case */
                v += ((opcode - TOK_ASM_addb as std::os::raw::c_int) / 5 as std::os::raw::c_int)
                    << 3 as std::os::raw::c_int
            } else if (*pa).instr_type as std::os::raw::c_int
                & (0x70 as std::os::raw::c_int | 0x8 as std::os::raw::c_int)
                == 0x40 as std::os::raw::c_int
            {
                /* fpu arith case */
                v += ((opcode - (*pa).sym as std::os::raw::c_int) / 6 as std::os::raw::c_int)
                    << 3 as std::os::raw::c_int
            }
            /* search which operand will be used for modrm */
            modrm_index = -(1 as std::os::raw::c_int);
            modreg_index = -(1 as std::os::raw::c_int);
            if (*pa).instr_type as std::os::raw::c_int & 0x8 as std::os::raw::c_int != 0 {
                current_block = 7238532450961708898;
                break;
            } else {
                current_block = 17170253997621722914;
                break;
            }
        }
    }
    match current_block {
        7238532450961708898 => {
            if nb_ops == 0 {
                /* A modrm opcode without operands is a special case (e.g. mfence).
                It has a group and acts as if there's an register operand 0
                (ax).  */
                i = 0 as std::os::raw::c_int;
                ops[i as usize].type_0 = ((1 as std::os::raw::c_int)
                    << OPT_REG8 as std::os::raw::c_int
                    | (1 as std::os::raw::c_int) << OPT_REG16 as std::os::raw::c_int
                    | (1 as std::os::raw::c_int) << OPT_REG32 as std::os::raw::c_int
                    | (1 as std::os::raw::c_int) << OPT_REG64 as std::os::raw::c_int)
                    as uint32_t;
                ops[i as usize].reg = 0 as std::os::raw::c_int as int8_t
            } else {
                /* first look for an ea operand */
                i = 0 as std::os::raw::c_int;
                loop {
                    if !(i < nb_ops) {
                        current_block = 7549100979104075880;
                        break;
                    }
                    if op_type[i as usize] & 0x40000000 as std::os::raw::c_int != 0 {
                        current_block = 17422450969180240714;
                        break;
                    }
                    i += 1
                }
                match current_block {
                    17422450969180240714 => {},
                    _ =>
                    /* then if not found, a register or indirection (shift instructions) */
                    {
                        i = 0 as std::os::raw::c_int;
                        while i < nb_ops {
                            if op_type[i as usize]
                                & ((1 as std::os::raw::c_int) << OPT_REG8 as std::os::raw::c_int
                                    | (1 as std::os::raw::c_int)
                                        << OPT_REG16 as std::os::raw::c_int
                                    | (1 as std::os::raw::c_int)
                                        << OPT_REG32 as std::os::raw::c_int
                                    | (1 as std::os::raw::c_int)
                                        << OPT_REG64 as std::os::raw::c_int
                                    | (1 as std::os::raw::c_int) << OPT_MMX as std::os::raw::c_int
                                    | (1 as std::os::raw::c_int) << OPT_SSE as std::os::raw::c_int
                                    | (1 as std::os::raw::c_int)
                                        << OPT_INDIR as std::os::raw::c_int)
                                != 0
                            {
                                break;
                            }
                            i += 1
                        }
                    }
                }
            }
            modrm_index = i;
            /* if a register is used in another operand then it is
            used instead of group */
            i = 0 as std::os::raw::c_int;
            while i < nb_ops {
                let mut t: std::os::raw::c_int = op_type[i as usize];
                if i != modrm_index
                    && t & ((1 as std::os::raw::c_int) << OPT_REG8 as std::os::raw::c_int
                        | (1 as std::os::raw::c_int) << OPT_REG16 as std::os::raw::c_int
                        | (1 as std::os::raw::c_int) << OPT_REG32 as std::os::raw::c_int
                        | (1 as std::os::raw::c_int) << OPT_REG64 as std::os::raw::c_int
                        | (1 as std::os::raw::c_int) << OPT_MMX as std::os::raw::c_int
                        | (1 as std::os::raw::c_int) << OPT_SSE as std::os::raw::c_int
                        | (1 as std::os::raw::c_int) << OPT_CR as std::os::raw::c_int
                        | (1 as std::os::raw::c_int) << OPT_TR as std::os::raw::c_int
                        | (1 as std::os::raw::c_int) << OPT_DB as std::os::raw::c_int
                        | (1 as std::os::raw::c_int) << OPT_SEG as std::os::raw::c_int)
                        != 0
                {
                    modreg_index = i;
                    break;
                } else {
                    i += 1
                }
            }
        },
        _ => {},
    }
    asm_rex(
        rex64,
        ops.as_mut_ptr(),
        nb_ops,
        op_type.as_mut_ptr(),
        modreg_index,
        modrm_index,
    );
    if (*pa).instr_type as std::os::raw::c_int & 0x4 as std::os::raw::c_int != 0 {
        /* mov $im, %reg case */
        if v == 0xb0 as std::os::raw::c_int && s >= 1 as std::os::raw::c_int {
            v += 7 as std::os::raw::c_int
        }
        i = 0 as std::os::raw::c_int;
        while i < nb_ops {
            if op_type[i as usize]
                & ((1 as std::os::raw::c_int) << OPT_REG8 as std::os::raw::c_int
                    | (1 as std::os::raw::c_int) << OPT_REG16 as std::os::raw::c_int
                    | (1 as std::os::raw::c_int) << OPT_REG32 as std::os::raw::c_int
                    | (1 as std::os::raw::c_int) << OPT_REG64 as std::os::raw::c_int
                    | (1 as std::os::raw::c_int) << OPT_ST as std::os::raw::c_int)
                != 0
            {
                v += ops[i as usize].reg as std::os::raw::c_int;
                break;
            } else {
                i += 1
            }
        }
    }
    if (*pa).instr_type as std::os::raw::c_int & 0x1 as std::os::raw::c_int != 0 {
        v += (s >= 1 as std::os::raw::c_int) as std::os::raw::c_int
    }
    if nb_ops == 1 as std::os::raw::c_int
        && (*pa).op_type[0 as std::os::raw::c_int as usize] as std::os::raw::c_int
            == OPT_DISP8 as std::os::raw::c_int
    {
        let mut current_block_165: u64;
        let mut esym: *mut Elf64_Sym = 0 as *mut Elf64_Sym;
        let mut jmp_disp: std::os::raw::c_int = 0;
        /* see if we can really generate the jump with a byte offset */
        esym = elfsym(ops[0 as std::os::raw::c_int as usize].e.sym);
        if esym.is_null()
            || (*esym).st_shndx as std::os::raw::c_int != (*(*tcc_state).cur_text_section).sh_num
        {
            current_block_165 = 1904967287505027374;
        } else {
            jmp_disp = ops[0 as std::os::raw::c_int as usize]
                .e
                .v
                .wrapping_add((*esym).st_value)
                .wrapping_sub(ind as std::os::raw::c_ulong)
                .wrapping_sub(2 as std::os::raw::c_int as std::os::raw::c_ulong)
                .wrapping_sub(
                    (v >= 0xff as std::os::raw::c_int) as std::os::raw::c_int
                        as std::os::raw::c_ulong,
                ) as std::os::raw::c_int;
            if jmp_disp == jmp_disp as int8_t as std::os::raw::c_int {
                /* OK to generate jump */
                ops[0 as std::os::raw::c_int as usize].e.sym = 0 as *mut Sym;
                ops[0 as std::os::raw::c_int as usize].e.v = jmp_disp as uint64_t;
                op_type[0 as std::os::raw::c_int as usize] =
                    (1 as std::os::raw::c_int) << OPT_IM8S as std::os::raw::c_int;
                current_block_165 = 9675306770437543583;
            } else {
                current_block_165 = 1904967287505027374;
            }
        }
        match current_block_165 {
            1904967287505027374 => {
                /* long jump will be allowed. need to modify the
                opcode slightly */
                if v == 0xeb as std::os::raw::c_int {
                    /* jmp */
                    v = 0xe9 as std::os::raw::c_int
                } else if v == 0x70 as std::os::raw::c_int {
                    /* jcc */
                    v += 0xf10 as std::os::raw::c_int
                } else {
                    _tcc_error(
                        b"invalid displacement\x00" as *const u8 as *const std::os::raw::c_char,
                    );
                }
            },
            _ => {},
        }
    }
    if (*pa).instr_type as std::os::raw::c_int & 0x70 as std::os::raw::c_int
        == 0x50 as std::os::raw::c_int
    {
        v += test_bits[(opcode - (*pa).sym as std::os::raw::c_int) as usize] as std::os::raw::c_int
    }
    op1 = v >> 16 as std::os::raw::c_int;
    if op1 != 0 {
        g(op1);
    }
    op1 = v >> 8 as std::os::raw::c_int & 0xff as std::os::raw::c_int;
    if op1 != 0 {
        g(op1);
    }
    g(v);
    if (*pa).instr_type as std::os::raw::c_int & 0x70 as std::os::raw::c_int
        == 0x20 as std::os::raw::c_int
    {
        reg = (opcode - (*pa).sym as std::os::raw::c_int) / 5 as std::os::raw::c_int;
        if reg == 6 as std::os::raw::c_int {
            reg = 7 as std::os::raw::c_int
        }
    } else if (*pa).instr_type as std::os::raw::c_int & 0x70 as std::os::raw::c_int
        == 0x30 as std::os::raw::c_int
    {
        reg = (opcode - (*pa).sym as std::os::raw::c_int) / 5 as std::os::raw::c_int
    } else if (*pa).instr_type as std::os::raw::c_int & 0x70 as std::os::raw::c_int
        == 0x40 as std::os::raw::c_int
    {
        reg = (opcode - (*pa).sym as std::os::raw::c_int) / 6 as std::os::raw::c_int
    } else {
        reg = (*pa).instr_type as std::os::raw::c_int >> 13 as std::os::raw::c_int
            & 7 as std::os::raw::c_int
    }
    pc = 0 as std::os::raw::c_int;
    if (*pa).instr_type as std::os::raw::c_int & 0x8 as std::os::raw::c_int != 0 {
        /* if a register is used in another operand then it is
        used instead of group */
        if modreg_index >= 0 as std::os::raw::c_int {
            reg = ops[modreg_index as usize].reg as std::os::raw::c_int
        }
        pc = asm_modrm(reg, &mut *ops.as_mut_ptr().offset(modrm_index as isize))
    }
    /* emit constants */
    i = 0 as std::os::raw::c_int;
    while i < nb_ops {
        v = op_type[i as usize];
        if v & ((1 as std::os::raw::c_int) << OPT_IM8 as std::os::raw::c_int
            | (1 as std::os::raw::c_int) << OPT_IM16 as std::os::raw::c_int
            | (1 as std::os::raw::c_int) << OPT_IM32 as std::os::raw::c_int
            | (1 as std::os::raw::c_int) << OPT_IM64 as std::os::raw::c_int
            | (1 as std::os::raw::c_int) << OPT_IM8S as std::os::raw::c_int
            | (1 as std::os::raw::c_int) << OPT_ADDR as std::os::raw::c_int)
            != 0
        {
            /* if multiple sizes are given it means we must look
            at the op size */
            if v | (1 as std::os::raw::c_int) << OPT_IM8 as std::os::raw::c_int
                | (1 as std::os::raw::c_int) << OPT_IM64 as std::os::raw::c_int
                == (1 as std::os::raw::c_int) << OPT_IM8 as std::os::raw::c_int
                    | (1 as std::os::raw::c_int) << OPT_IM16 as std::os::raw::c_int
                    | (1 as std::os::raw::c_int) << OPT_IM32 as std::os::raw::c_int
                    | (1 as std::os::raw::c_int) << OPT_IM64 as std::os::raw::c_int
            {
                if s == 0 as std::os::raw::c_int {
                    v = (1 as std::os::raw::c_int) << OPT_IM8 as std::os::raw::c_int
                } else if s == 1 as std::os::raw::c_int {
                    v = (1 as std::os::raw::c_int) << OPT_IM16 as std::os::raw::c_int
                } else if s == 2 as std::os::raw::c_int
                    || v & (1 as std::os::raw::c_int) << OPT_IM64 as std::os::raw::c_int
                        == 0 as std::os::raw::c_int
                {
                    v = (1 as std::os::raw::c_int) << OPT_IM32 as std::os::raw::c_int
                } else {
                    v = (1 as std::os::raw::c_int) << OPT_IM64 as std::os::raw::c_int
                }
            }
            if v & ((1 as std::os::raw::c_int) << OPT_IM8 as std::os::raw::c_int
                | (1 as std::os::raw::c_int) << OPT_IM8S as std::os::raw::c_int
                | (1 as std::os::raw::c_int) << OPT_IM16 as std::os::raw::c_int)
                != 0
                && !ops[i as usize].e.sym.is_null()
            {
                _tcc_error(b"cannot relocate\x00" as *const u8 as *const std::os::raw::c_char);
            }
            if v & ((1 as std::os::raw::c_int) << OPT_IM8 as std::os::raw::c_int
                | (1 as std::os::raw::c_int) << OPT_IM8S as std::os::raw::c_int)
                != 0
            {
                g(ops[i as usize].e.v as std::os::raw::c_int);
            } else if v & (1 as std::os::raw::c_int) << OPT_IM16 as std::os::raw::c_int != 0 {
                gen_le16(ops[i as usize].e.v as std::os::raw::c_int);
            } else if v & (1 as std::os::raw::c_int) << OPT_IM64 as std::os::raw::c_int != 0 {
                gen_expr64(&mut (*ops.as_mut_ptr().offset(i as isize)).e);
            } else if (*pa).op_type[i as usize] as std::os::raw::c_int
                == OPT_DISP as std::os::raw::c_int
                || (*pa).op_type[i as usize] as std::os::raw::c_int
                    == OPT_DISP8 as std::os::raw::c_int
            {
                gen_disp32(&mut (*ops.as_mut_ptr().offset(i as isize)).e);
            } else {
                gen_expr32(&mut (*ops.as_mut_ptr().offset(i as isize)).e);
            }
        }
        i += 1
    }
    /* after immediate operands, adjust pc-relative address */
    if pc != 0 {
        add32le(
            (*(*tcc_state).cur_text_section)
                .data
                .offset(pc as isize)
                .offset(-(4 as std::os::raw::c_int as isize)),
            pc - ind,
        );
    };
}
/* return the constraint priority (we allocate first the lowest
numbered constraints) */
#[inline]
unsafe extern "C" fn constraint_priority(
    mut str: *const std::os::raw::c_char,
) -> std::os::raw::c_int {
    let mut priority: std::os::raw::c_int = 0;
    let mut c: std::os::raw::c_int = 0;
    let mut pr: std::os::raw::c_int = 0;
    /* we take the lowest priority */
    priority = 0 as std::os::raw::c_int;
    loop {
        c = *str as std::os::raw::c_int;
        if c == '\u{0}' as i32 {
            break;
        }
        str = str.offset(1);
        match c {
            65 => pr = 0 as std::os::raw::c_int,
            97 | 98 | 99 | 100 | 83 | 68 => pr = 1 as std::os::raw::c_int,
            113 => pr = 2 as std::os::raw::c_int,
            114 | 82 | 112 => pr = 3 as std::os::raw::c_int,
            78 | 77 | 73 | 101 | 105 | 109 | 103 => pr = 4 as std::os::raw::c_int,
            _ => {
                _tcc_error(
                    b"unknown constraint \'%c\'\x00" as *const u8 as *const std::os::raw::c_char,
                    c,
                );
            },
        }
        if pr > priority {
            priority = pr
        }
    }
    return priority;
}
unsafe extern "C" fn skip_constraint_modifiers(
    mut p: *const std::os::raw::c_char,
) -> *const std::os::raw::c_char {
    while *p as std::os::raw::c_int == '=' as i32
        || *p as std::os::raw::c_int == '&' as i32
        || *p as std::os::raw::c_int == '+' as i32
        || *p as std::os::raw::c_int == '%' as i32
    {
        p = p.offset(1)
    }
    return p;
}
/* If T (a token) is of the form "%reg" returns the register
number and type, otherwise return -1.  */
#[no_mangle]
pub unsafe extern "C" fn asm_parse_regvar(mut t: std::os::raw::c_int) -> std::os::raw::c_int {
    let mut s: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut op: Operand = Operand {
        type_0: 0,
        reg: 0,
        reg2: 0,
        shift: 0,
        e: ExprValue {
            v: 0,
            sym: 0 as *mut Sym,
            pcrel: 0,
        },
    };
    if t < 256 as std::os::raw::c_int || t & 0x20000000 as std::os::raw::c_int != 0 {
        return -(1 as std::os::raw::c_int);
    }
    s = (**table_ident.offset((t - 256 as std::os::raw::c_int) as isize))
        .str_0
        .as_mut_ptr();
    if *s.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int != '%' as i32 {
        return -(1 as std::os::raw::c_int);
    }
    t = tok_alloc_const(s.offset(1 as std::os::raw::c_int as isize));
    unget_tok(t);
    unget_tok('%' as i32);
    parse_operand(tcc_state, &mut op);
    /* Accept only integer regs for now.  */
    if op.type_0
        & ((1 as std::os::raw::c_int) << OPT_REG8 as std::os::raw::c_int
            | (1 as std::os::raw::c_int) << OPT_REG16 as std::os::raw::c_int
            | (1 as std::os::raw::c_int) << OPT_REG32 as std::os::raw::c_int
            | (1 as std::os::raw::c_int) << OPT_REG64 as std::os::raw::c_int)
            as std::os::raw::c_uint
        != 0
    {
        return op.reg as std::os::raw::c_int;
    } else {
        return -(1 as std::os::raw::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn asm_compute_constraints(
    mut operands: *mut ASMOperand,
    mut nb_operands: std::os::raw::c_int,
    mut nb_outputs: std::os::raw::c_int,
    mut clobber_regs: *const uint8_t,
    mut pout_reg: *mut std::os::raw::c_int,
) {
    let mut current_block: u64;
    let mut op: *mut ASMOperand = 0 as *mut ASMOperand;
    let mut sorted_op: [std::os::raw::c_int; 30] = [0; 30];
    let mut i: std::os::raw::c_int = 0;
    let mut j: std::os::raw::c_int = 0;
    let mut k: std::os::raw::c_int = 0;
    let mut p1: std::os::raw::c_int = 0;
    let mut p2: std::os::raw::c_int = 0;
    let mut tmp: std::os::raw::c_int = 0;
    let mut reg: std::os::raw::c_int = 0;
    let mut c: std::os::raw::c_int = 0;
    let mut reg_mask: std::os::raw::c_int = 0;
    let mut str: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut regs_allocated: [uint8_t; 16] = [0; 16];
    /* init fields */
    i = 0 as std::os::raw::c_int;
    while i < nb_operands {
        op = &mut *operands.offset(i as isize) as *mut ASMOperand;
        (*op).input_index = -(1 as std::os::raw::c_int);
        (*op).ref_index = -(1 as std::os::raw::c_int);
        (*op).reg = -(1 as std::os::raw::c_int);
        (*op).is_memory = 0 as std::os::raw::c_int;
        (*op).is_rw = 0 as std::os::raw::c_int;
        i += 1
    }
    /* compute constraint priority and evaluate references to output
    constraints if input constraints */
    i = 0 as std::os::raw::c_int;
    while i < nb_operands {
        op = &mut *operands.offset(i as isize) as *mut ASMOperand;
        str = (*op).constraint;
        str = skip_constraint_modifiers(str);
        if isnum(*str as std::os::raw::c_int) != 0 || *str as std::os::raw::c_int == '[' as i32 {
            /* this is a reference to another constraint */
            k = find_constraint(
                operands,
                nb_operands,
                str,
                0 as *mut *const std::os::raw::c_char,
            );
            if k as std::os::raw::c_uint >= i as std::os::raw::c_uint || i < nb_outputs {
                _tcc_error(
                    b"invalid reference in constraint %d (\'%s\')\x00" as *const u8
                        as *const std::os::raw::c_char,
                    i,
                    str,
                );
            }
            (*op).ref_index = k;
            if (*operands.offset(k as isize)).input_index >= 0 as std::os::raw::c_int {
                _tcc_error(
                    b"cannot reference twice the same operand\x00" as *const u8
                        as *const std::os::raw::c_char,
                );
            }
            (*operands.offset(k as isize)).input_index = i;
            (*op).priority = 5 as std::os::raw::c_int
        } else if (*(*op).vt).r as std::os::raw::c_int & 0x3f as std::os::raw::c_int
            == 0x32 as std::os::raw::c_int
            && !(*(*op).vt).c2rust_unnamed_0.sym.is_null()
            && {
                reg = (*(*(*op).vt).c2rust_unnamed_0.sym).r as std::os::raw::c_int
                    & 0x3f as std::os::raw::c_int;
                (reg) < 0x30 as std::os::raw::c_int
            }
        {
            (*op).priority = 1 as std::os::raw::c_int;
            (*op).reg = reg
        } else {
            (*op).priority = constraint_priority(str)
        }
        i += 1
    }
    /* sort operands according to their priority */
    i = 0 as std::os::raw::c_int;
    while i < nb_operands {
        sorted_op[i as usize] = i;
        i += 1
    }
    i = 0 as std::os::raw::c_int;
    while i < nb_operands - 1 as std::os::raw::c_int {
        j = i + 1 as std::os::raw::c_int;
        while j < nb_operands {
            p1 = (*operands.offset(sorted_op[i as usize] as isize)).priority;
            p2 = (*operands.offset(sorted_op[j as usize] as isize)).priority;
            if p2 < p1 {
                tmp = sorted_op[i as usize];
                sorted_op[i as usize] = sorted_op[j as usize];
                sorted_op[j as usize] = tmp
            }
            j += 1
        }
        i += 1
    }
    i = 0 as std::os::raw::c_int;
    while i < 16 as std::os::raw::c_int {
        if *clobber_regs.offset(i as isize) != 0 {
            regs_allocated[i as usize] =
                (0x2 as std::os::raw::c_int | 0x1 as std::os::raw::c_int) as uint8_t
        } else {
            regs_allocated[i as usize] = 0 as std::os::raw::c_int as uint8_t
        }
        i += 1
    }
    /* esp cannot be used */
    regs_allocated[4 as std::os::raw::c_int as usize] =
        (0x2 as std::os::raw::c_int | 0x1 as std::os::raw::c_int) as uint8_t;
    /* ebp cannot be used yet */
    regs_allocated[5 as std::os::raw::c_int as usize] =
        (0x2 as std::os::raw::c_int | 0x1 as std::os::raw::c_int) as uint8_t;
    /* allocate registers and generate corresponding asm moves */
    i = 0 as std::os::raw::c_int;
    while i < nb_operands {
        j = sorted_op[i as usize];
        op = &mut *operands.offset(j as isize) as *mut ASMOperand;
        str = (*op).constraint;
        /* no need to allocate references */
        if !((*op).ref_index >= 0 as std::os::raw::c_int) {
            /* select if register is used for output, input or both */
            if (*op).input_index >= 0 as std::os::raw::c_int {
                reg_mask = 0x2 as std::os::raw::c_int | 0x1 as std::os::raw::c_int
            } else if j < nb_outputs {
                reg_mask = 0x1 as std::os::raw::c_int
            } else {
                reg_mask = 0x2 as std::os::raw::c_int
            }
            if (*op).reg >= 0 as std::os::raw::c_int {
                if regs_allocated[(*op).reg as usize] as std::os::raw::c_int & reg_mask != 0 {
                    _tcc_error(
                        b"asm regvar requests register that\'s taken already\x00" as *const u8
                            as *const std::os::raw::c_char,
                    );
                }
                reg = (*op).reg;
                current_block = 5280412841289695757;
            } else {
                'c_38506: loop {
                    let fresh7 = str;
                    str = str.offset(1);
                    c = *fresh7 as std::os::raw::c_int;
                    match c {
                        61 => {
                            continue;
                        },
                        43 => {
                            (*op).is_rw = 1 as std::os::raw::c_int;
                            current_block = 12671336273659955656;
                        },
                        38 => {
                            current_block = 12671336273659955656;
                        },
                        65 => {
                            /* allocate both eax and edx */
                            if regs_allocated[TREG_RAX as std::os::raw::c_int as usize]
                                as std::os::raw::c_int
                                & reg_mask
                                != 0
                                || regs_allocated[TREG_RDX as std::os::raw::c_int as usize]
                                    as std::os::raw::c_int
                                    & reg_mask
                                    != 0
                            {
                                continue;
                            }
                            (*op).is_llong = 1 as std::os::raw::c_int;
                            (*op).reg = TREG_RAX as std::os::raw::c_int;
                            regs_allocated[TREG_RAX as std::os::raw::c_int as usize] =
                                (regs_allocated[TREG_RAX as std::os::raw::c_int as usize]
                                    as std::os::raw::c_int
                                    | reg_mask) as uint8_t;
                            regs_allocated[TREG_RDX as std::os::raw::c_int as usize] =
                                (regs_allocated[TREG_RDX as std::os::raw::c_int as usize]
                                    as std::os::raw::c_int
                                    | reg_mask) as uint8_t;
                            current_block = 9430418855388998878;
                            break;
                        },
                        97 => {
                            reg = TREG_RAX as std::os::raw::c_int;
                            current_block = 7888318297499932645;
                        },
                        98 => {
                            reg = 3 as std::os::raw::c_int;
                            current_block = 7888318297499932645;
                        },
                        99 => {
                            reg = TREG_RCX as std::os::raw::c_int;
                            current_block = 7888318297499932645;
                        },
                        100 => {
                            reg = TREG_RDX as std::os::raw::c_int;
                            current_block = 7888318297499932645;
                        },
                        83 => {
                            reg = 6 as std::os::raw::c_int;
                            current_block = 7888318297499932645;
                        },
                        68 => {
                            reg = 7 as std::os::raw::c_int;
                            current_block = 7888318297499932645;
                        },
                        113 => {
                            /* eax, ebx, ecx or edx */
                            reg = 0 as std::os::raw::c_int;
                            loop {
                                if !(reg < 4 as std::os::raw::c_int) {
                                    continue 'c_38506;
                                }
                                if regs_allocated[reg as usize] as std::os::raw::c_int & reg_mask
                                    == 0
                                {
                                    current_block = 5280412841289695757;
                                    break 'c_38506;
                                }
                                reg += 1
                            }
                        },
                        114 | 82 | 112 => {
                            /* A general address, for x86(64) any register is acceptable*/
                            /* any general register */
                            reg = 0 as std::os::raw::c_int;
                            loop {
                                if !(reg < 8 as std::os::raw::c_int) {
                                    continue 'c_38506;
                                }
                                if regs_allocated[reg as usize] as std::os::raw::c_int & reg_mask
                                    == 0
                                {
                                    current_block = 5280412841289695757;
                                    break 'c_38506;
                                }
                                reg += 1
                            }
                        },
                        101 | 105 => {
                            if !((*(*op).vt).r as std::os::raw::c_int
                                & (0x3f as std::os::raw::c_int | 0x100 as std::os::raw::c_int)
                                == 0x30 as std::os::raw::c_int)
                            {
                                continue;
                            } else {
                                current_block = 9430418855388998878;
                                break;
                            }
                        },
                        73 | 78 | 77 => {
                            if !((*(*op).vt).r as std::os::raw::c_int
                                & (0x3f as std::os::raw::c_int
                                    | 0x100 as std::os::raw::c_int
                                    | 0x200 as std::os::raw::c_int)
                                == 0x30 as std::os::raw::c_int)
                            {
                                continue;
                            } else {
                                current_block = 9430418855388998878;
                                break;
                            }
                        },
                        109 | 103 => {
                            /* nothing special to do because the operand is already in
                            memory, except if the pointer itself is stored in a
                            memory variable (VT_LLOCAL case) */
                            /* XXX: fix constant case */
                            /* if it is a reference to a memory zone, it must lie
                            in a register, so we reserve the register in the
                            input registers and a load will be generated
                            later */
                            if !(j < nb_outputs || c == 'm' as i32) {
                                current_block = 9430418855388998878;
                                break;
                            }
                            if !((*(*op).vt).r as std::os::raw::c_int & 0x3f as std::os::raw::c_int
                                == 0x31 as std::os::raw::c_int)
                            {
                                current_block = 9430418855388998878;
                                break;
                            }
                            /* any general register */
                            reg = 0 as std::os::raw::c_int;
                            loop {
                                if !(reg < 8 as std::os::raw::c_int) {
                                    continue 'c_38506;
                                }
                                if regs_allocated[reg as usize] as std::os::raw::c_int
                                    & 0x2 as std::os::raw::c_int
                                    == 0
                                {
                                    break;
                                }
                                reg += 1
                            }
                            /* now we can reload in the register */
                            regs_allocated[reg as usize] = (regs_allocated[reg as usize]
                                as std::os::raw::c_int
                                | 0x2 as std::os::raw::c_int)
                                as uint8_t;
                            (*op).reg = reg;
                            (*op).is_memory = 1 as std::os::raw::c_int;
                            current_block = 9430418855388998878;
                            break;
                        },
                        _ => {
                            _tcc_error(
                                b"asm constraint %d (\'%s\') could not be satisfied\x00"
                                    as *const u8
                                    as *const std::os::raw::c_char,
                                j,
                                (*op).constraint,
                            );
                        },
                    }
                    match current_block {
                        7888318297499932645 => {
                            if !(regs_allocated[reg as usize] as std::os::raw::c_int & reg_mask
                                != 0)
                            {
                                current_block = 5280412841289695757;
                                break;
                            }
                        },
                        _ =>
                        /* FALL THRU */
                        {
                            if j >= nb_outputs {
                                _tcc_error(
                                    b"\'%c\' modifier can only be applied to outputs\x00"
                                        as *const u8
                                        as *const std::os::raw::c_char,
                                    c,
                                );
                            }
                            reg_mask = 0x2 as std::os::raw::c_int | 0x1 as std::os::raw::c_int
                        }
                    }
                }
            }
            match current_block {
                5280412841289695757 => {
                    /* now we can reload in the register */
                    (*op).is_llong = 0 as std::os::raw::c_int;
                    (*op).reg = reg;
                    regs_allocated[reg as usize] =
                        (regs_allocated[reg as usize] as std::os::raw::c_int | reg_mask) as uint8_t
                },
                _ => {},
            }
            /* if a reference is present for that operand, we assign it too */
            if (*op).input_index >= 0 as std::os::raw::c_int {
                (*operands.offset((*op).input_index as isize)).reg = (*op).reg;
                (*operands.offset((*op).input_index as isize)).is_llong = (*op).is_llong
            }
        }
        i += 1
    }
    /* compute out_reg. It is used to store outputs registers to memory
    locations references by pointers (VT_LLOCAL case) */
    *pout_reg = -(1 as std::os::raw::c_int);
    let mut current_block_104: u64;
    i = 0 as std::os::raw::c_int;
    while i < nb_operands {
        op = &mut *operands.offset(i as isize) as *mut ASMOperand;
        if (*op).reg >= 0 as std::os::raw::c_int
            && (*(*op).vt).r as std::os::raw::c_int & 0x3f as std::os::raw::c_int
                == 0x31 as std::os::raw::c_int
            && (*op).is_memory == 0
        {
            reg = 0 as std::os::raw::c_int;
            loop {
                if !(reg < 8 as std::os::raw::c_int) {
                    current_block_104 = 18221534353613080499;
                    break;
                }
                if regs_allocated[reg as usize] as std::os::raw::c_int & 0x1 as std::os::raw::c_int
                    == 0
                {
                    current_block_104 = 6791187058007702249;
                    break;
                }
                reg += 1
            }
            match current_block_104 {
                18221534353613080499 => {
                    _tcc_error(
                        b"could not find free output register for reloading\x00" as *const u8
                            as *const std::os::raw::c_char,
                    );
                },
                _ => {
                    *pout_reg = reg;
                    break;
                },
            }
        } else {
            i += 1
        }
    }
    /* print sorted constraints */
}
#[no_mangle]
pub unsafe extern "C" fn subst_asm_operand(
    mut add_str: *mut CString,
    mut sv: *mut SValue,
    mut modifier: std::os::raw::c_int,
) {
    let mut r: std::os::raw::c_int = 0;
    let mut reg: std::os::raw::c_int = 0;
    let mut size: std::os::raw::c_int = 0;
    let mut val: std::os::raw::c_int = 0;
    let mut buf: [std::os::raw::c_char; 64] = [0; 64];
    r = (*sv).r as std::os::raw::c_int;
    if r & 0x3f as std::os::raw::c_int == 0x30 as std::os::raw::c_int {
        let mut current_block_17: u64;
        if r & 0x100 as std::os::raw::c_int == 0
            && modifier != 'c' as i32
            && modifier != 'n' as i32
            && modifier != 'P' as i32
        {
            cstr_ccat(add_str, '$' as i32);
        }
        if r & 0x200 as std::os::raw::c_int != 0 {
            let mut name: *const std::os::raw::c_char =
                get_tok_str((*(*sv).c2rust_unnamed_0.sym).v, 0 as *mut CValue);
            if (*(*sv).c2rust_unnamed_0.sym).v >= 0x10000000 as std::os::raw::c_int {
                /* In case of anonymous symbols ("L.42", used
                for static data labels) we can't find them
                in the C symbol table when later looking up
                this name.  So enter them now into the asm label
                list when we still know the symbol.  */
                get_asm_sym(tok_alloc_const(name), (*sv).c2rust_unnamed_0.sym);
            }
            if (*tcc_state).leading_underscore != 0 {
                cstr_ccat(add_str, '_' as i32);
            }
            cstr_cat(add_str, name, -(1 as std::os::raw::c_int));
            if (*sv).c2rust_unnamed.c.i as uint32_t
                == 0 as std::os::raw::c_int as std::os::raw::c_uint
            {
                current_block_17 = 12124785117276362961;
            } else {
                cstr_ccat(add_str, '+' as i32);
                current_block_17 = 12599329904712511516;
            }
        } else {
            current_block_17 = 12599329904712511516;
        }
        match current_block_17 {
            12599329904712511516 => {
                val = (*sv).c2rust_unnamed.c.i as std::os::raw::c_int;
                if modifier == 'n' as i32 {
                    val = -val
                }
                snprintf(
                    buf.as_mut_ptr(),
                    ::std::mem::size_of::<[std::os::raw::c_char; 64]>() as std::os::raw::c_ulong,
                    b"%d\x00" as *const u8 as *const std::os::raw::c_char,
                    (*sv).c2rust_unnamed.c.i as std::os::raw::c_int,
                );
                cstr_cat(add_str, buf.as_mut_ptr(), -(1 as std::os::raw::c_int));
            },
            _ => {},
        }
        if r & 0x100 as std::os::raw::c_int != 0 {
            cstr_cat(
                add_str,
                b"(%rip)\x00" as *const u8 as *const std::os::raw::c_char,
                -(1 as std::os::raw::c_int),
            );
        }
    } else if r & 0x3f as std::os::raw::c_int == 0x32 as std::os::raw::c_int {
        snprintf(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[std::os::raw::c_char; 64]>() as std::os::raw::c_ulong,
            b"%d(%%rbp)\x00" as *const u8 as *const std::os::raw::c_char,
            (*sv).c2rust_unnamed.c.i as std::os::raw::c_int,
        );
        cstr_cat(add_str, buf.as_mut_ptr(), -(1 as std::os::raw::c_int));
    } else if r & 0x100 as std::os::raw::c_int != 0 {
        reg = r & 0x3f as std::os::raw::c_int;
        if reg >= 0x30 as std::os::raw::c_int {
            _tcc_error(
                b"internal compiler error\n%s:%d: in %s(): \x00" as *const u8
                    as *const std::os::raw::c_char,
                b"i386-asm.c\x00" as *const u8 as *const std::os::raw::c_char,
                1522 as std::os::raw::c_int,
                (*::std::mem::transmute::<&[u8; 18], &[std::os::raw::c_char; 18]>(
                    b"subst_asm_operand\x00",
                ))
                .as_ptr(),
            );
        }
        snprintf(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[std::os::raw::c_char; 64]>() as std::os::raw::c_ulong,
            b"(%%%s)\x00" as *const u8 as *const std::os::raw::c_char,
            get_tok_str(TOK_ASM_rax as std::os::raw::c_int + reg, 0 as *mut CValue),
        );
        cstr_cat(add_str, buf.as_mut_ptr(), -(1 as std::os::raw::c_int));
    } else {
        /* register case */
        reg = r & 0x3f as std::os::raw::c_int;
        if reg >= 0x30 as std::os::raw::c_int {
            _tcc_error(
                b"internal compiler error\n%s:%d: in %s(): \x00" as *const u8
                    as *const std::os::raw::c_char,
                b"i386-asm.c\x00" as *const u8 as *const std::os::raw::c_char,
                1535 as std::os::raw::c_int,
                (*::std::mem::transmute::<&[u8; 18], &[std::os::raw::c_char; 18]>(
                    b"subst_asm_operand\x00",
                ))
                .as_ptr(),
            );
        }
        /* choose register operand size */
        if (*sv).type_0.t & 0xf as std::os::raw::c_int == 1 as std::os::raw::c_int
            || (*sv).type_0.t & 0xf as std::os::raw::c_int == 11 as std::os::raw::c_int
        {
            size = 1 as std::os::raw::c_int
        } else if (*sv).type_0.t & 0xf as std::os::raw::c_int == 2 as std::os::raw::c_int {
            size = 2 as std::os::raw::c_int
        } else if (*sv).type_0.t & 0xf as std::os::raw::c_int == 4 as std::os::raw::c_int
            || (*sv).type_0.t & 0xf as std::os::raw::c_int == 5 as std::os::raw::c_int
        {
            size = 8 as std::os::raw::c_int
        } else {
            size = 4 as std::os::raw::c_int
        }
        if size == 1 as std::os::raw::c_int && reg >= 4 as std::os::raw::c_int {
            size = 4 as std::os::raw::c_int
        }
        if modifier == 'b' as i32 {
            if reg >= 4 as std::os::raw::c_int {
                _tcc_error(
                    b"cannot use byte register\x00" as *const u8 as *const std::os::raw::c_char,
                );
            }
            size = 1 as std::os::raw::c_int
        } else if modifier == 'h' as i32 {
            if reg >= 4 as std::os::raw::c_int {
                _tcc_error(
                    b"cannot use byte register\x00" as *const u8 as *const std::os::raw::c_char,
                );
            }
            size = -(1 as std::os::raw::c_int)
        } else if modifier == 'w' as i32 {
            size = 2 as std::os::raw::c_int
        } else if modifier == 'k' as i32 {
            size = 4 as std::os::raw::c_int
        } else if modifier == 'q' as i32 {
            size = 8 as std::os::raw::c_int
        }
        match size {
            -1 => reg = TOK_ASM_ah as std::os::raw::c_int + reg,
            1 => reg = TOK_ASM_al as std::os::raw::c_int + reg,
            2 => reg = TOK_ASM_ax as std::os::raw::c_int + reg,
            8 => reg = TOK_ASM_rax as std::os::raw::c_int + reg,
            _ => reg = TOK_ASM_eax as std::os::raw::c_int + reg,
        }
        snprintf(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[std::os::raw::c_char; 64]>() as std::os::raw::c_ulong,
            b"%%%s\x00" as *const u8 as *const std::os::raw::c_char,
            get_tok_str(reg, 0 as *mut CValue),
        );
        cstr_cat(add_str, buf.as_mut_ptr(), -(1 as std::os::raw::c_int));
    };
}
/* generate prolog and epilog code for asm statement */
#[no_mangle]
pub unsafe extern "C" fn asm_gen_code(
    mut operands: *mut ASMOperand,
    mut nb_operands: std::os::raw::c_int,
    mut nb_outputs: std::os::raw::c_int,
    mut is_output: std::os::raw::c_int,
    mut clobber_regs: *mut uint8_t,
    mut out_reg: std::os::raw::c_int,
) {
    let mut regs_allocated: [uint8_t; 16] = [0; 16];
    let mut op: *mut ASMOperand = 0 as *mut ASMOperand;
    let mut i: std::os::raw::c_int = 0;
    let mut reg: std::os::raw::c_int = 0;
    /* Strictly speaking %Xbp and %Xsp should be included in the
    call-preserved registers, but currently it doesn't matter.  */
    static mut reg_saved: [uint8_t; 5] = [
        3 as std::os::raw::c_int as uint8_t,
        12 as std::os::raw::c_int as uint8_t,
        13 as std::os::raw::c_int as uint8_t,
        14 as std::os::raw::c_int as uint8_t,
        15 as std::os::raw::c_int as uint8_t,
    ];
    /* mark all used registers */
    memcpy(
        regs_allocated.as_mut_ptr() as *mut std::os::raw::c_void,
        clobber_regs as *const std::os::raw::c_void,
        ::std::mem::size_of::<[uint8_t; 16]>() as std::os::raw::c_ulong,
    );
    i = 0 as std::os::raw::c_int;
    while i < nb_operands {
        op = &mut *operands.offset(i as isize) as *mut ASMOperand;
        if (*op).reg >= 0 as std::os::raw::c_int {
            regs_allocated[(*op).reg as usize] = 1 as std::os::raw::c_int as uint8_t
        }
        i += 1
    }
    if is_output == 0 {
        /* generate reg save code */
        i = 0 as std::os::raw::c_int;
        while (i as std::os::raw::c_ulong)
            < (::std::mem::size_of::<[uint8_t; 5]>() as std::os::raw::c_ulong)
                .wrapping_div(::std::mem::size_of::<uint8_t>() as std::os::raw::c_ulong)
        {
            reg = reg_saved[i as usize] as std::os::raw::c_int;
            if regs_allocated[reg as usize] != 0 {
                if reg >= 8 as std::os::raw::c_int {
                    g(0x41 as std::os::raw::c_int);
                    reg -= 8 as std::os::raw::c_int
                }
                g(0x50 as std::os::raw::c_int + reg);
            }
            i += 1
        }
        /* generate load code */
        i = 0 as std::os::raw::c_int;
        while i < nb_operands {
            op = &mut *operands.offset(i as isize) as *mut ASMOperand;
            if (*op).reg >= 0 as std::os::raw::c_int {
                if (*(*op).vt).r as std::os::raw::c_int & 0x3f as std::os::raw::c_int
                    == 0x31 as std::os::raw::c_int
                    && (*op).is_memory != 0
                {
                    /* memory reference case (for both input and
                    output cases) */
                    let mut sv: SValue = SValue {
                        type_0: CType {
                            t: 0,
                            ref_0: 0 as *mut Sym,
                        },
                        r: 0,
                        r2: 0,
                        c2rust_unnamed: C2RustUnnamed_8 {
                            c2rust_unnamed: C2RustUnnamed_9 {
                                jtrue: 0,
                                jfalse: 0,
                            },
                        },
                        c2rust_unnamed_0: C2RustUnnamed_6 {
                            c2rust_unnamed: C2RustUnnamed_7 {
                                cmp_op: 0,
                                cmp_r: 0,
                            },
                        },
                    };
                    sv = *(*op).vt;
                    sv.r = (sv.r as std::os::raw::c_int & !(0x3f as std::os::raw::c_int)
                        | 0x32 as std::os::raw::c_int
                        | 0x100 as std::os::raw::c_int)
                        as std::os::raw::c_ushort;
                    sv.type_0.t = 5 as std::os::raw::c_int;
                    load((*op).reg, &mut sv);
                } else if i >= nb_outputs || (*op).is_rw != 0 {
                    /* load value in register */
                    load((*op).reg, (*op).vt);
                    if (*op).is_llong != 0 {
                        let mut sv_0: SValue = SValue {
                            type_0: CType {
                                t: 0,
                                ref_0: 0 as *mut Sym,
                            },
                            r: 0,
                            r2: 0,
                            c2rust_unnamed: C2RustUnnamed_8 {
                                c2rust_unnamed: C2RustUnnamed_9 {
                                    jtrue: 0,
                                    jfalse: 0,
                                },
                            },
                            c2rust_unnamed_0: C2RustUnnamed_6 {
                                c2rust_unnamed: C2RustUnnamed_7 {
                                    cmp_op: 0,
                                    cmp_r: 0,
                                },
                            },
                        };
                        sv_0 = *(*op).vt;
                        sv_0.c2rust_unnamed.c.i = (sv_0.c2rust_unnamed.c.i as std::os::raw::c_ulong)
                            .wrapping_add(4 as std::os::raw::c_int as std::os::raw::c_ulong)
                            as uint64_t
                            as uint64_t;
                        load(TREG_RDX as std::os::raw::c_int, &mut sv_0);
                    }
                }
            }
            i += 1
        }
    } else {
        /* generate save code */
        i = 0 as std::os::raw::c_int;
        while i < nb_outputs {
            op = &mut *operands.offset(i as isize) as *mut ASMOperand;
            if (*op).reg >= 0 as std::os::raw::c_int {
                if (*(*op).vt).r as std::os::raw::c_int & 0x3f as std::os::raw::c_int
                    == 0x31 as std::os::raw::c_int
                {
                    if (*op).is_memory == 0 {
                        let mut sv_1: SValue = SValue {
                            type_0: CType {
                                t: 0,
                                ref_0: 0 as *mut Sym,
                            },
                            r: 0,
                            r2: 0,
                            c2rust_unnamed: C2RustUnnamed_8 {
                                c2rust_unnamed: C2RustUnnamed_9 {
                                    jtrue: 0,
                                    jfalse: 0,
                                },
                            },
                            c2rust_unnamed_0: C2RustUnnamed_6 {
                                c2rust_unnamed: C2RustUnnamed_7 {
                                    cmp_op: 0,
                                    cmp_r: 0,
                                },
                            },
                        };
                        sv_1 = *(*op).vt;
                        sv_1.r = (sv_1.r as std::os::raw::c_int & !(0x3f as std::os::raw::c_int)
                            | 0x32 as std::os::raw::c_int)
                            as std::os::raw::c_ushort;
                        sv_1.type_0.t = 5 as std::os::raw::c_int;
                        load(out_reg, &mut sv_1);
                        sv_1 = *(*op).vt;
                        sv_1.r = (sv_1.r as std::os::raw::c_int & !(0x3f as std::os::raw::c_int)
                            | out_reg) as std::os::raw::c_ushort;
                        store((*op).reg, &mut sv_1);
                    }
                } else {
                    store((*op).reg, (*op).vt);
                    if (*op).is_llong != 0 {
                        let mut sv_2: SValue = SValue {
                            type_0: CType {
                                t: 0,
                                ref_0: 0 as *mut Sym,
                            },
                            r: 0,
                            r2: 0,
                            c2rust_unnamed: C2RustUnnamed_8 {
                                c2rust_unnamed: C2RustUnnamed_9 {
                                    jtrue: 0,
                                    jfalse: 0,
                                },
                            },
                            c2rust_unnamed_0: C2RustUnnamed_6 {
                                c2rust_unnamed: C2RustUnnamed_7 {
                                    cmp_op: 0,
                                    cmp_r: 0,
                                },
                            },
                        };
                        sv_2 = *(*op).vt;
                        sv_2.c2rust_unnamed.c.i = (sv_2.c2rust_unnamed.c.i as std::os::raw::c_ulong)
                            .wrapping_add(4 as std::os::raw::c_int as std::os::raw::c_ulong)
                            as uint64_t
                            as uint64_t;
                        store(TREG_RDX as std::os::raw::c_int, &mut sv_2);
                    }
                }
            }
            i += 1
        }
        /* generate reg restore code */
        i = (::std::mem::size_of::<[uint8_t; 5]>() as std::os::raw::c_ulong)
            .wrapping_div(::std::mem::size_of::<uint8_t>() as std::os::raw::c_ulong)
            .wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong)
            as std::os::raw::c_int;
        while i >= 0 as std::os::raw::c_int {
            reg = reg_saved[i as usize] as std::os::raw::c_int;
            if regs_allocated[reg as usize] != 0 {
                if reg >= 8 as std::os::raw::c_int {
                    g(0x41 as std::os::raw::c_int);
                    reg -= 8 as std::os::raw::c_int
                }
                g(0x58 as std::os::raw::c_int + reg);
            }
            i -= 1
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn asm_clobber(
    mut clobber_regs: *mut uint8_t,
    mut str: *const std::os::raw::c_char,
) {
    let mut reg: std::os::raw::c_int = 0;
    let mut type_0: std::os::raw::c_uint = 0;
    if strcmp(
        str,
        b"memory\x00" as *const u8 as *const std::os::raw::c_char,
    ) == 0
        || strcmp(str, b"cc\x00" as *const u8 as *const std::os::raw::c_char) == 0
        || strcmp(
            str,
            b"flags\x00" as *const u8 as *const std::os::raw::c_char,
        ) == 0
    {
        return;
    }
    reg = tok_alloc_const(str);
    if reg >= TOK_ASM_eax as std::os::raw::c_int && reg <= TOK_ASM_edi as std::os::raw::c_int {
        reg -= TOK_ASM_eax as std::os::raw::c_int
    } else if reg >= TOK_ASM_ax as std::os::raw::c_int && reg <= TOK_ASM_di as std::os::raw::c_int {
        reg -= TOK_ASM_ax as std::os::raw::c_int
    } else if reg >= TOK_ASM_rax as std::os::raw::c_int && reg <= TOK_ASM_rdi as std::os::raw::c_int
    {
        reg -= TOK_ASM_rax as std::os::raw::c_int
    } else {
        reg = asm_parse_numeric_reg(reg, &mut type_0);
        if reg >= 0 as std::os::raw::c_int {
        } else {
            _tcc_error(
                b"invalid clobber register \'%s\'\x00" as *const u8 as *const std::os::raw::c_char,
                str,
            );
        }
    }
    *clobber_regs.offset(reg as isize) = 1 as std::os::raw::c_int as uint8_t;
}
unsafe extern "C" fn run_static_initializers() {
    asm_instrs = [
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_cmpsb as std::os::raw::c_int as uint16_t,
                opcode: if 0xa6 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xa6 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xa6 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xa6 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x1 as std::os::raw::c_int
                    | 0x1000 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xa6 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 0 as std::os::raw::c_int as uint8_t,
                op_type: [0 as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_scmpb as std::os::raw::c_int as uint16_t,
                opcode: if 0xa6 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xa6 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xa6 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xa6 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x1 as std::os::raw::c_int
                    | 0x1000 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xa6 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 0 as std::os::raw::c_int as uint8_t,
                op_type: [0 as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_insb as std::os::raw::c_int as uint16_t,
                opcode: if 0x6c as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0x6c as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0x6c as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0x6c as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x1 as std::os::raw::c_int
                    | 0x2 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0x6c as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 0 as std::os::raw::c_int as uint8_t,
                op_type: [0 as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_outsb as std::os::raw::c_int as uint16_t,
                opcode: if 0x6e as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0x6e as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0x6e as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0x6e as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x1 as std::os::raw::c_int
                    | 0x2 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0x6e as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 0 as std::os::raw::c_int as uint8_t,
                op_type: [0 as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_lodsb as std::os::raw::c_int as uint16_t,
                opcode: if 0xac as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xac as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xac as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xac as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x1 as std::os::raw::c_int
                    | 0x1000 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xac as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 0 as std::os::raw::c_int as uint8_t,
                op_type: [0 as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_slodb as std::os::raw::c_int as uint16_t,
                opcode: if 0xac as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xac as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xac as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xac as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x1 as std::os::raw::c_int
                    | 0x1000 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xac as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 0 as std::os::raw::c_int as uint8_t,
                op_type: [0 as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_movsb as std::os::raw::c_int as uint16_t,
                opcode: if 0xa4 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xa4 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xa4 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xa4 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x1 as std::os::raw::c_int
                    | 0x1000 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xa4 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 0 as std::os::raw::c_int as uint8_t,
                op_type: [0 as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_smovb as std::os::raw::c_int as uint16_t,
                opcode: if 0xa4 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xa4 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xa4 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xa4 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x1 as std::os::raw::c_int
                    | 0x1000 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xa4 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 0 as std::os::raw::c_int as uint8_t,
                op_type: [0 as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_scasb as std::os::raw::c_int as uint16_t,
                opcode: if 0xae as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xae as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xae as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xae as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x1 as std::os::raw::c_int
                    | 0x1000 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xae as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 0 as std::os::raw::c_int as uint8_t,
                op_type: [0 as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_sscab as std::os::raw::c_int as uint16_t,
                opcode: if 0xae as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xae as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xae as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xae as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x1 as std::os::raw::c_int
                    | 0x1000 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xae as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 0 as std::os::raw::c_int as uint8_t,
                op_type: [0 as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_stosb as std::os::raw::c_int as uint16_t,
                opcode: if 0xaa as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xaa as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xaa as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xaa as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x1 as std::os::raw::c_int
                    | 0x1000 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xaa as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 0 as std::os::raw::c_int as uint8_t,
                op_type: [0 as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_sstob as std::os::raw::c_int as uint16_t,
                opcode: if 0xaa as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xaa as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xaa as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xaa as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x1 as std::os::raw::c_int
                    | 0x1000 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xaa as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 0 as std::os::raw::c_int as uint8_t,
                op_type: [0 as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_bsfw as std::os::raw::c_int as uint16_t,
                opcode: if 0xfbc as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xfbc as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xfbc as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xfbc as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | 0x1000 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xfbc as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_REGW as std::os::raw::c_int | OPT_EA as std::os::raw::c_int) as uint8_t,
                    OPT_REGW as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_bsrw as std::os::raw::c_int as uint16_t,
                opcode: if 0xfbd as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xfbd as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xfbd as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xfbd as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | 0x1000 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xfbd as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_REGW as std::os::raw::c_int | OPT_EA as std::os::raw::c_int) as uint8_t,
                    OPT_REGW as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_btw as std::os::raw::c_int as uint16_t,
                opcode: if 0xfa3 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xfa3 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xfa3 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xfa3 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | 0x1000 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xfa3 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_REGW as std::os::raw::c_int as uint8_t,
                    (OPT_REGW as std::os::raw::c_int | OPT_EA as std::os::raw::c_int) as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_btw as std::os::raw::c_int as uint16_t,
                opcode: if 0xfba as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xfba as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xfba as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xfba as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | 0x1000 as std::os::raw::c_int
                    | (4 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xfba as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_IM8 as std::os::raw::c_int as uint8_t,
                    (OPT_REGW as std::os::raw::c_int | OPT_EA as std::os::raw::c_int) as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_btsw as std::os::raw::c_int as uint16_t,
                opcode: if 0xfab as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xfab as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xfab as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xfab as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | 0x1000 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xfab as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_REGW as std::os::raw::c_int as uint8_t,
                    (OPT_REGW as std::os::raw::c_int | OPT_EA as std::os::raw::c_int) as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_btsw as std::os::raw::c_int as uint16_t,
                opcode: if 0xfba as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xfba as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xfba as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xfba as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | 0x1000 as std::os::raw::c_int
                    | (5 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xfba as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_IM8 as std::os::raw::c_int as uint8_t,
                    (OPT_REGW as std::os::raw::c_int | OPT_EA as std::os::raw::c_int) as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_btrw as std::os::raw::c_int as uint16_t,
                opcode: if 0xfb3 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xfb3 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xfb3 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xfb3 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | 0x1000 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xfb3 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_REGW as std::os::raw::c_int as uint8_t,
                    (OPT_REGW as std::os::raw::c_int | OPT_EA as std::os::raw::c_int) as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_btrw as std::os::raw::c_int as uint16_t,
                opcode: if 0xfba as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xfba as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xfba as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xfba as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | 0x1000 as std::os::raw::c_int
                    | (6 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xfba as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_IM8 as std::os::raw::c_int as uint8_t,
                    (OPT_REGW as std::os::raw::c_int | OPT_EA as std::os::raw::c_int) as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_btcw as std::os::raw::c_int as uint16_t,
                opcode: if 0xfbb as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xfbb as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xfbb as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xfbb as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | 0x1000 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xfbb as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_REGW as std::os::raw::c_int as uint8_t,
                    (OPT_REGW as std::os::raw::c_int | OPT_EA as std::os::raw::c_int) as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_btcw as std::os::raw::c_int as uint16_t,
                opcode: if 0xfba as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xfba as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xfba as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xfba as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | 0x1000 as std::os::raw::c_int
                    | (7 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xfba as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_IM8 as std::os::raw::c_int as uint8_t,
                    (OPT_REGW as std::os::raw::c_int | OPT_EA as std::os::raw::c_int) as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_sysretq as std::os::raw::c_int as uint16_t,
                opcode: if 0x480f07 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0x480f07 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0x480f07 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0x480f07 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0x480f07 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 0 as std::os::raw::c_int as uint8_t,
                op_type: [0 as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_movb as std::os::raw::c_int as uint16_t,
                opcode: if 0x88 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0x88 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0x88 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0x88 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0x1 as std::os::raw::c_int | 0x1000 as std::os::raw::c_int)
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0x88 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_REG as std::os::raw::c_int as uint8_t,
                    (OPT_EA as std::os::raw::c_int | OPT_REG as std::os::raw::c_int) as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_movb as std::os::raw::c_int as uint16_t,
                opcode: if 0x8a as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0x8a as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0x8a as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0x8a as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0x1 as std::os::raw::c_int | 0x1000 as std::os::raw::c_int)
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0x8a as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_EA as std::os::raw::c_int | OPT_REG as std::os::raw::c_int) as uint8_t,
                    OPT_REG as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_movb as std::os::raw::c_int as uint16_t,
                opcode: if 0xb0 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xb0 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xb0 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xb0 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x4 as std::os::raw::c_int
                    | (0x1 as std::os::raw::c_int | 0x1000 as std::os::raw::c_int)
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xb0 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_IM as std::os::raw::c_int as uint8_t,
                    OPT_REG as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_mov as std::os::raw::c_int as uint16_t,
                opcode: if 0xb8 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xb8 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xb8 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xb8 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x4 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xb8 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_IM64 as std::os::raw::c_int as uint8_t,
                    OPT_REG64 as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_movq as std::os::raw::c_int as uint16_t,
                opcode: if 0xb8 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xb8 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xb8 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xb8 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x4 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xb8 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_IM64 as std::os::raw::c_int as uint8_t,
                    OPT_REG64 as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_movb as std::os::raw::c_int as uint16_t,
                opcode: if 0xc6 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xc6 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xc6 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xc6 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0x1 as std::os::raw::c_int | 0x1000 as std::os::raw::c_int)
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xc6 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_IM as std::os::raw::c_int as uint8_t,
                    (OPT_REG as std::os::raw::c_int | OPT_EA as std::os::raw::c_int) as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_movw as std::os::raw::c_int as uint16_t,
                opcode: if 0x8c as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0x8c as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0x8c as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0x8c as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | 0x1000 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0x8c as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_SEG as std::os::raw::c_int as uint8_t,
                    (OPT_EA as std::os::raw::c_int | OPT_REG as std::os::raw::c_int) as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_movw as std::os::raw::c_int as uint16_t,
                opcode: if 0x8e as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0x8e as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0x8e as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0x8e as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | 0x1000 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0x8e as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_EA as std::os::raw::c_int | OPT_REG as std::os::raw::c_int) as uint8_t,
                    OPT_SEG as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_movw as std::os::raw::c_int as uint16_t,
                opcode: if 0xf20 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf20 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf20 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf20 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | 0x1000 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf20 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_CR as std::os::raw::c_int as uint8_t,
                    OPT_REG64 as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_movw as std::os::raw::c_int as uint16_t,
                opcode: if 0xf21 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf21 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf21 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf21 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | 0x1000 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf21 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_DB as std::os::raw::c_int as uint8_t,
                    OPT_REG64 as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_movw as std::os::raw::c_int as uint16_t,
                opcode: if 0xf22 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf22 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf22 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf22 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | 0x1000 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf22 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_REG64 as std::os::raw::c_int as uint8_t,
                    OPT_CR as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_movw as std::os::raw::c_int as uint16_t,
                opcode: if 0xf23 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf23 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf23 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf23 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | 0x1000 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf23 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_REG64 as std::os::raw::c_int as uint8_t,
                    OPT_DB as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_movsbw as std::os::raw::c_int as uint16_t,
                opcode: if 0x660fbe as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0x660fbe as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0x660fbe as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0x660fbe as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0x660fbe as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_REG8 as std::os::raw::c_int | OPT_EA as std::os::raw::c_int) as uint8_t,
                    OPT_REG16 as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_movsbl as std::os::raw::c_int as uint16_t,
                opcode: if 0xfbe as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xfbe as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xfbe as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xfbe as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xfbe as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_REG8 as std::os::raw::c_int | OPT_EA as std::os::raw::c_int) as uint8_t,
                    OPT_REG32 as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_movsbq as std::os::raw::c_int as uint16_t,
                opcode: if 0xfbe as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xfbe as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xfbe as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xfbe as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xfbe as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_REG8 as std::os::raw::c_int | OPT_EA as std::os::raw::c_int) as uint8_t,
                    OPT_REGW as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_movswl as std::os::raw::c_int as uint16_t,
                opcode: if 0xfbf as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xfbf as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xfbf as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xfbf as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xfbf as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_REG16 as std::os::raw::c_int | OPT_EA as std::os::raw::c_int) as uint8_t,
                    OPT_REG32 as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_movswq as std::os::raw::c_int as uint16_t,
                opcode: if 0xfbf as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xfbf as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xfbf as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xfbf as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xfbf as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_REG16 as std::os::raw::c_int | OPT_EA as std::os::raw::c_int) as uint8_t,
                    OPT_REG as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_movslq as std::os::raw::c_int as uint16_t,
                opcode: if 0x63 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0x63 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0x63 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0x63 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0x63 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_REG32 as std::os::raw::c_int | OPT_EA as std::os::raw::c_int) as uint8_t,
                    OPT_REG as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_movzbw as std::os::raw::c_int as uint16_t,
                opcode: if 0xfb6 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xfb6 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xfb6 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xfb6 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | 0x1000 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xfb6 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_REG8 as std::os::raw::c_int | OPT_EA as std::os::raw::c_int) as uint8_t,
                    OPT_REGW as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_movzwl as std::os::raw::c_int as uint16_t,
                opcode: if 0xfb7 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xfb7 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xfb7 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xfb7 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xfb7 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_REG16 as std::os::raw::c_int | OPT_EA as std::os::raw::c_int) as uint8_t,
                    OPT_REG32 as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_movzwq as std::os::raw::c_int as uint16_t,
                opcode: if 0xfb7 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xfb7 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xfb7 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xfb7 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xfb7 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_REG16 as std::os::raw::c_int | OPT_EA as std::os::raw::c_int) as uint8_t,
                    OPT_REG as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_pushq as std::os::raw::c_int as uint16_t,
                opcode: if 0x6a as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0x6a as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0x6a as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0x6a as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0x6a as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_IM8S as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_push as std::os::raw::c_int as uint16_t,
                opcode: if 0x6a as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0x6a as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0x6a as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0x6a as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0x6a as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_IM8S as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_pushw as std::os::raw::c_int as uint16_t,
                opcode: if 0x666a as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0x666a as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0x666a as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0x666a as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0x666a as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_IM8S as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_pushw as std::os::raw::c_int as uint16_t,
                opcode: if 0x50 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0x50 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0x50 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0x50 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x4 as std::os::raw::c_int
                    | 0x1000 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0x50 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_REG64 as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_pushw as std::os::raw::c_int as uint16_t,
                opcode: if 0x50 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0x50 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0x50 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0x50 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x4 as std::os::raw::c_int
                    | 0x1000 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0x50 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_REG16 as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_pushw as std::os::raw::c_int as uint16_t,
                opcode: if 0xff as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xff as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xff as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xff as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | 0x1000 as std::os::raw::c_int
                    | (6 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xff as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_REG64 as std::os::raw::c_int | OPT_EA as std::os::raw::c_int) as uint8_t,
                    0,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_pushw as std::os::raw::c_int as uint16_t,
                opcode: if 0x6668 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0x6668 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0x6668 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0x6668 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0x6668 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_IM16 as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_pushw as std::os::raw::c_int as uint16_t,
                opcode: if 0x68 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0x68 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0x68 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0x68 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x1000 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0x68 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_IM32 as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_pushw as std::os::raw::c_int as uint16_t,
                opcode: if 0x6 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0x6 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0x6 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0x6 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x1000 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0x6 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_SEG as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_popw as std::os::raw::c_int as uint16_t,
                opcode: if 0x58 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0x58 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0x58 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0x58 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x4 as std::os::raw::c_int
                    | 0x1000 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0x58 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_REG64 as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_popw as std::os::raw::c_int as uint16_t,
                opcode: if 0x58 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0x58 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0x58 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0x58 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x4 as std::os::raw::c_int
                    | 0x1000 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0x58 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_REG16 as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_popw as std::os::raw::c_int as uint16_t,
                opcode: if 0x8f as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0x8f as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0x8f as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0x8f as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | 0x1000 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0x8f as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_REGW as std::os::raw::c_int | OPT_EA as std::os::raw::c_int) as uint8_t,
                    0,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_popw as std::os::raw::c_int as uint16_t,
                opcode: if 0x7 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0x7 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0x7 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0x7 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x1000 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0x7 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_SEG as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_xchgw as std::os::raw::c_int as uint16_t,
                opcode: if 0x90 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0x90 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0x90 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0x90 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x4 as std::os::raw::c_int
                    | 0x1000 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0x90 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_REGW as std::os::raw::c_int as uint8_t,
                    OPT_EAX as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_xchgw as std::os::raw::c_int as uint16_t,
                opcode: if 0x90 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0x90 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0x90 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0x90 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x4 as std::os::raw::c_int
                    | 0x1000 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0x90 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_EAX as std::os::raw::c_int as uint8_t,
                    OPT_REGW as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_xchgb as std::os::raw::c_int as uint16_t,
                opcode: if 0x86 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0x86 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0x86 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0x86 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0x1 as std::os::raw::c_int | 0x1000 as std::os::raw::c_int)
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0x86 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_REG as std::os::raw::c_int as uint8_t,
                    (OPT_EA as std::os::raw::c_int | OPT_REG as std::os::raw::c_int) as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_xchgb as std::os::raw::c_int as uint16_t,
                opcode: if 0x86 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0x86 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0x86 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0x86 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0x1 as std::os::raw::c_int | 0x1000 as std::os::raw::c_int)
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0x86 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_EA as std::os::raw::c_int | OPT_REG as std::os::raw::c_int) as uint8_t,
                    OPT_REG as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_inb as std::os::raw::c_int as uint16_t,
                opcode: if 0xe4 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xe4 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xe4 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xe4 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x1 as std::os::raw::c_int
                    | 0x2 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xe4 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_IM8 as std::os::raw::c_int as uint8_t,
                    OPT_EAX as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_inb as std::os::raw::c_int as uint16_t,
                opcode: if 0xe4 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xe4 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xe4 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xe4 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x1 as std::os::raw::c_int
                    | 0x2 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xe4 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_IM8 as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_inb as std::os::raw::c_int as uint16_t,
                opcode: if 0xec as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xec as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xec as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xec as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x1 as std::os::raw::c_int
                    | 0x2 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xec as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_DX as std::os::raw::c_int as uint8_t,
                    OPT_EAX as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_inb as std::os::raw::c_int as uint16_t,
                opcode: if 0xec as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xec as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xec as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xec as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x1 as std::os::raw::c_int
                    | 0x2 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xec as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_DX as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_outb as std::os::raw::c_int as uint16_t,
                opcode: if 0xe6 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xe6 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xe6 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xe6 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x1 as std::os::raw::c_int
                    | 0x2 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xe6 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_EAX as std::os::raw::c_int as uint8_t,
                    OPT_IM8 as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_outb as std::os::raw::c_int as uint16_t,
                opcode: if 0xe6 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xe6 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xe6 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xe6 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x1 as std::os::raw::c_int
                    | 0x2 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xe6 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_IM8 as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_outb as std::os::raw::c_int as uint16_t,
                opcode: if 0xee as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xee as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xee as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xee as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x1 as std::os::raw::c_int
                    | 0x2 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xee as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_EAX as std::os::raw::c_int as uint8_t,
                    OPT_DX as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_outb as std::os::raw::c_int as uint16_t,
                opcode: if 0xee as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xee as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xee as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xee as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x1 as std::os::raw::c_int
                    | 0x2 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xee as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_DX as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_leaw as std::os::raw::c_int as uint16_t,
                opcode: if 0x8d as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0x8d as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0x8d as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0x8d as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | 0x1000 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0x8d as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_EA as std::os::raw::c_int as uint8_t,
                    OPT_REG as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_les as std::os::raw::c_int as uint16_t,
                opcode: if 0xc4 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xc4 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xc4 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xc4 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xc4 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_EA as std::os::raw::c_int as uint8_t,
                    OPT_REG32 as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_lds as std::os::raw::c_int as uint16_t,
                opcode: if 0xc5 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xc5 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xc5 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xc5 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xc5 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_EA as std::os::raw::c_int as uint8_t,
                    OPT_REG32 as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_lss as std::os::raw::c_int as uint16_t,
                opcode: if 0xfb2 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xfb2 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xfb2 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xfb2 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xfb2 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_EA as std::os::raw::c_int as uint8_t,
                    OPT_REG32 as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_lfs as std::os::raw::c_int as uint16_t,
                opcode: if 0xfb4 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xfb4 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xfb4 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xfb4 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xfb4 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_EA as std::os::raw::c_int as uint8_t,
                    OPT_REG32 as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_lgs as std::os::raw::c_int as uint16_t,
                opcode: if 0xfb5 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xfb5 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xfb5 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xfb5 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xfb5 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_EA as std::os::raw::c_int as uint8_t,
                    OPT_REG32 as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_addb as std::os::raw::c_int as uint16_t,
                opcode: if 0 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x30 as std::os::raw::c_int
                    | 0x8 as std::os::raw::c_int
                    | (0x1 as std::os::raw::c_int | 0x1000 as std::os::raw::c_int)
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_REG as std::os::raw::c_int as uint8_t,
                    (OPT_EA as std::os::raw::c_int | OPT_REG as std::os::raw::c_int) as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_addb as std::os::raw::c_int as uint16_t,
                opcode: if 0x2 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0x2 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0x2 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0x2 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x30 as std::os::raw::c_int
                    | 0x8 as std::os::raw::c_int
                    | (0x1 as std::os::raw::c_int | 0x1000 as std::os::raw::c_int)
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0x2 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_EA as std::os::raw::c_int | OPT_REG as std::os::raw::c_int) as uint8_t,
                    OPT_REG as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_addb as std::os::raw::c_int as uint16_t,
                opcode: if 0x4 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0x4 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0x4 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0x4 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x30 as std::os::raw::c_int
                    | (0x1 as std::os::raw::c_int | 0x1000 as std::os::raw::c_int)
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0x4 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_IM as std::os::raw::c_int as uint8_t,
                    OPT_EAX as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_addw as std::os::raw::c_int as uint16_t,
                opcode: if 0x83 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0x83 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0x83 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0x83 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x30 as std::os::raw::c_int
                    | 0x8 as std::os::raw::c_int
                    | 0x1000 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0x83 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_IM8S as std::os::raw::c_int as uint8_t,
                    (OPT_EA as std::os::raw::c_int | OPT_REGW as std::os::raw::c_int) as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_addb as std::os::raw::c_int as uint16_t,
                opcode: if 0x80 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0x80 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0x80 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0x80 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x30 as std::os::raw::c_int
                    | 0x8 as std::os::raw::c_int
                    | (0x1 as std::os::raw::c_int | 0x1000 as std::os::raw::c_int)
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0x80 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_IM as std::os::raw::c_int as uint8_t,
                    (OPT_EA as std::os::raw::c_int | OPT_REG as std::os::raw::c_int) as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_testb as std::os::raw::c_int as uint16_t,
                opcode: if 0x84 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0x84 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0x84 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0x84 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0x1 as std::os::raw::c_int | 0x1000 as std::os::raw::c_int)
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0x84 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_REG as std::os::raw::c_int as uint8_t,
                    (OPT_EA as std::os::raw::c_int | OPT_REG as std::os::raw::c_int) as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_testb as std::os::raw::c_int as uint16_t,
                opcode: if 0x84 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0x84 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0x84 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0x84 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0x1 as std::os::raw::c_int | 0x1000 as std::os::raw::c_int)
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0x84 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_EA as std::os::raw::c_int | OPT_REG as std::os::raw::c_int) as uint8_t,
                    OPT_REG as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_testb as std::os::raw::c_int as uint16_t,
                opcode: if 0xa8 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xa8 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xa8 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xa8 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x1 as std::os::raw::c_int
                    | 0x1000 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xa8 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_IM as std::os::raw::c_int as uint8_t,
                    OPT_EAX as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_testb as std::os::raw::c_int as uint16_t,
                opcode: if 0xf6 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf6 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf6 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf6 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0x1 as std::os::raw::c_int | 0x1000 as std::os::raw::c_int)
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf6 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_IM as std::os::raw::c_int as uint8_t,
                    (OPT_EA as std::os::raw::c_int | OPT_REG as std::os::raw::c_int) as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_incb as std::os::raw::c_int as uint16_t,
                opcode: if 0xfe as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xfe as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xfe as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xfe as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0x1 as std::os::raw::c_int | 0x1000 as std::os::raw::c_int)
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xfe as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_REG as std::os::raw::c_int | OPT_EA as std::os::raw::c_int) as uint8_t,
                    0,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_decb as std::os::raw::c_int as uint16_t,
                opcode: if 0xfe as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xfe as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xfe as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xfe as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0x1 as std::os::raw::c_int | 0x1000 as std::os::raw::c_int)
                    | (1 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xfe as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_REG as std::os::raw::c_int | OPT_EA as std::os::raw::c_int) as uint8_t,
                    0,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_notb as std::os::raw::c_int as uint16_t,
                opcode: if 0xf6 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf6 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf6 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf6 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0x1 as std::os::raw::c_int | 0x1000 as std::os::raw::c_int)
                    | (2 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf6 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_REG as std::os::raw::c_int | OPT_EA as std::os::raw::c_int) as uint8_t,
                    0,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_negb as std::os::raw::c_int as uint16_t,
                opcode: if 0xf6 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf6 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf6 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf6 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0x1 as std::os::raw::c_int | 0x1000 as std::os::raw::c_int)
                    | (3 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf6 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_REG as std::os::raw::c_int | OPT_EA as std::os::raw::c_int) as uint8_t,
                    0,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_mulb as std::os::raw::c_int as uint16_t,
                opcode: if 0xf6 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf6 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf6 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf6 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0x1 as std::os::raw::c_int | 0x1000 as std::os::raw::c_int)
                    | (4 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf6 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_REG as std::os::raw::c_int | OPT_EA as std::os::raw::c_int) as uint8_t,
                    0,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_imulb as std::os::raw::c_int as uint16_t,
                opcode: if 0xf6 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf6 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf6 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf6 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0x1 as std::os::raw::c_int | 0x1000 as std::os::raw::c_int)
                    | (5 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf6 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_REG as std::os::raw::c_int | OPT_EA as std::os::raw::c_int) as uint8_t,
                    0,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_imulw as std::os::raw::c_int as uint16_t,
                opcode: if 0xfaf as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xfaf as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xfaf as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xfaf as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | 0x1000 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xfaf as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_REG as std::os::raw::c_int | OPT_EA as std::os::raw::c_int) as uint8_t,
                    OPT_REG as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_imulw as std::os::raw::c_int as uint16_t,
                opcode: if 0x6b as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0x6b as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0x6b as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0x6b as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | 0x1000 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0x6b as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 3 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_IM8S as std::os::raw::c_int as uint8_t,
                    (OPT_REGW as std::os::raw::c_int | OPT_EA as std::os::raw::c_int) as uint8_t,
                    OPT_REGW as std::os::raw::c_int as uint8_t,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_imulw as std::os::raw::c_int as uint16_t,
                opcode: if 0x6b as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0x6b as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0x6b as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0x6b as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | 0x1000 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0x6b as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_IM8S as std::os::raw::c_int as uint8_t,
                    OPT_REGW as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_imulw as std::os::raw::c_int as uint16_t,
                opcode: if 0x69 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0x69 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0x69 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0x69 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | 0x1000 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0x69 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 3 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_IMW as std::os::raw::c_int as uint8_t,
                    (OPT_REGW as std::os::raw::c_int | OPT_EA as std::os::raw::c_int) as uint8_t,
                    OPT_REGW as std::os::raw::c_int as uint8_t,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_imulw as std::os::raw::c_int as uint16_t,
                opcode: if 0x69 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0x69 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0x69 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0x69 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | 0x1000 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0x69 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_IMW as std::os::raw::c_int as uint8_t,
                    OPT_REGW as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_divb as std::os::raw::c_int as uint16_t,
                opcode: if 0xf6 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf6 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf6 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf6 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0x1 as std::os::raw::c_int | 0x1000 as std::os::raw::c_int)
                    | (6 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf6 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_REG as std::os::raw::c_int | OPT_EA as std::os::raw::c_int) as uint8_t,
                    0,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_divb as std::os::raw::c_int as uint16_t,
                opcode: if 0xf6 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf6 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf6 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf6 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0x1 as std::os::raw::c_int | 0x1000 as std::os::raw::c_int)
                    | (6 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf6 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_REG as std::os::raw::c_int | OPT_EA as std::os::raw::c_int) as uint8_t,
                    OPT_EAX as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_idivb as std::os::raw::c_int as uint16_t,
                opcode: if 0xf6 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf6 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf6 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf6 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0x1 as std::os::raw::c_int | 0x1000 as std::os::raw::c_int)
                    | (7 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf6 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_REG as std::os::raw::c_int | OPT_EA as std::os::raw::c_int) as uint8_t,
                    0,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_idivb as std::os::raw::c_int as uint16_t,
                opcode: if 0xf6 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf6 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf6 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf6 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0x1 as std::os::raw::c_int | 0x1000 as std::os::raw::c_int)
                    | (7 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf6 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_REG as std::os::raw::c_int | OPT_EA as std::os::raw::c_int) as uint8_t,
                    OPT_EAX as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_rolb as std::os::raw::c_int as uint16_t,
                opcode: if 0xc0 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xc0 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xc0 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xc0 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0x1 as std::os::raw::c_int | 0x1000 as std::os::raw::c_int)
                    | 0x20 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xc0 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_IM8 as std::os::raw::c_int as uint8_t,
                    (OPT_EA as std::os::raw::c_int | OPT_REG as std::os::raw::c_int) as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_rolb as std::os::raw::c_int as uint16_t,
                opcode: if 0xd2 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xd2 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xd2 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xd2 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0x1 as std::os::raw::c_int | 0x1000 as std::os::raw::c_int)
                    | 0x20 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xd2 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_CL as std::os::raw::c_int as uint8_t,
                    (OPT_EA as std::os::raw::c_int | OPT_REG as std::os::raw::c_int) as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_rolb as std::os::raw::c_int as uint16_t,
                opcode: if 0xd0 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xd0 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xd0 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xd0 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0x1 as std::os::raw::c_int | 0x1000 as std::os::raw::c_int)
                    | 0x20 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xd0 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_EA as std::os::raw::c_int | OPT_REG as std::os::raw::c_int) as uint8_t,
                    0,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_shldw as std::os::raw::c_int as uint16_t,
                opcode: if 0xfa4 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xfa4 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xfa4 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xfa4 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | 0x1000 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xfa4 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 3 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_IM8 as std::os::raw::c_int as uint8_t,
                    OPT_REGW as std::os::raw::c_int as uint8_t,
                    (OPT_EA as std::os::raw::c_int | OPT_REGW as std::os::raw::c_int) as uint8_t,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_shldw as std::os::raw::c_int as uint16_t,
                opcode: if 0xfa5 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xfa5 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xfa5 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xfa5 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | 0x1000 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xfa5 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 3 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_CL as std::os::raw::c_int as uint8_t,
                    OPT_REGW as std::os::raw::c_int as uint8_t,
                    (OPT_EA as std::os::raw::c_int | OPT_REGW as std::os::raw::c_int) as uint8_t,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_shldw as std::os::raw::c_int as uint16_t,
                opcode: if 0xfa5 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xfa5 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xfa5 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xfa5 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | 0x1000 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xfa5 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_REGW as std::os::raw::c_int as uint8_t,
                    (OPT_EA as std::os::raw::c_int | OPT_REGW as std::os::raw::c_int) as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_shrdw as std::os::raw::c_int as uint16_t,
                opcode: if 0xfac as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xfac as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xfac as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xfac as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | 0x1000 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xfac as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 3 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_IM8 as std::os::raw::c_int as uint8_t,
                    OPT_REGW as std::os::raw::c_int as uint8_t,
                    (OPT_EA as std::os::raw::c_int | OPT_REGW as std::os::raw::c_int) as uint8_t,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_shrdw as std::os::raw::c_int as uint16_t,
                opcode: if 0xfad as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xfad as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xfad as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xfad as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | 0x1000 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xfad as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 3 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_CL as std::os::raw::c_int as uint8_t,
                    OPT_REGW as std::os::raw::c_int as uint8_t,
                    (OPT_EA as std::os::raw::c_int | OPT_REGW as std::os::raw::c_int) as uint8_t,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_shrdw as std::os::raw::c_int as uint16_t,
                opcode: if 0xfad as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xfad as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xfad as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xfad as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | 0x1000 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xfad as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_REGW as std::os::raw::c_int as uint8_t,
                    (OPT_EA as std::os::raw::c_int | OPT_REGW as std::os::raw::c_int) as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_call as std::os::raw::c_int as uint16_t,
                opcode: if 0xff as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xff as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xff as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xff as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (2 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xff as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_INDIR as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_call as std::os::raw::c_int as uint16_t,
                opcode: if 0xe8 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xe8 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xe8 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xe8 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xe8 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_DISP as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_jmp as std::os::raw::c_int as uint16_t,
                opcode: if 0xff as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xff as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xff as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xff as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (4 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xff as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_INDIR as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_jmp as std::os::raw::c_int as uint16_t,
                opcode: if 0xeb as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xeb as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xeb as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xeb as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xeb as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_DISP8 as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_lcall as std::os::raw::c_int as uint16_t,
                opcode: if 0xff as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xff as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xff as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xff as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (3 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xff as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_EA as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_ljmp as std::os::raw::c_int as uint16_t,
                opcode: if 0xff as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xff as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xff as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xff as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (5 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xff as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_EA as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_ljmpw as std::os::raw::c_int as uint16_t,
                opcode: if 0x66ff as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0x66ff as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0x66ff as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0x66ff as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (5 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0x66ff as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_EA as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_ljmpl as std::os::raw::c_int as uint16_t,
                opcode: if 0xff as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xff as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xff as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xff as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (5 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xff as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_EA as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_INT as std::os::raw::c_int as uint16_t,
                opcode: if 0xcd as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xcd as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xcd as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xcd as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xcd as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_IM8 as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_seto as std::os::raw::c_int as uint16_t,
                opcode: if 0xf90 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf90 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf90 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf90 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | 0x50 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf90 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_REG8 as std::os::raw::c_int | OPT_EA as std::os::raw::c_int) as uint8_t,
                    0,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_setob as std::os::raw::c_int as uint16_t,
                opcode: if 0xf90 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf90 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf90 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf90 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | 0x50 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf90 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_REG8 as std::os::raw::c_int | OPT_EA as std::os::raw::c_int) as uint8_t,
                    0,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_enter as std::os::raw::c_int as uint16_t,
                opcode: if 0xc8 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xc8 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xc8 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xc8 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xc8 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_IM16 as std::os::raw::c_int as uint8_t,
                    OPT_IM8 as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_retq as std::os::raw::c_int as uint16_t,
                opcode: if 0xc2 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xc2 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xc2 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xc2 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xc2 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_IM16 as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_ret as std::os::raw::c_int as uint16_t,
                opcode: if 0xc2 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xc2 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xc2 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xc2 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xc2 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_IM16 as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_lret as std::os::raw::c_int as uint16_t,
                opcode: if 0xca as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xca as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xca as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xca as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xca as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_IM16 as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_jo as std::os::raw::c_int as uint16_t,
                opcode: if 0x70 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0x70 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0x70 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0x70 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x50 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0x70 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_DISP8 as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_loopne as std::os::raw::c_int as uint16_t,
                opcode: if 0xe0 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xe0 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xe0 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xe0 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xe0 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_DISP8 as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_loopnz as std::os::raw::c_int as uint16_t,
                opcode: if 0xe0 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xe0 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xe0 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xe0 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xe0 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_DISP8 as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_loope as std::os::raw::c_int as uint16_t,
                opcode: if 0xe1 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xe1 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xe1 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xe1 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xe1 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_DISP8 as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_loopz as std::os::raw::c_int as uint16_t,
                opcode: if 0xe1 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xe1 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xe1 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xe1 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xe1 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_DISP8 as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_loop as std::os::raw::c_int as uint16_t,
                opcode: if 0xe2 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xe2 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xe2 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xe2 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xe2 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_DISP8 as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_jecxz as std::os::raw::c_int as uint16_t,
                opcode: if 0x67e3 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0x67e3 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0x67e3 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0x67e3 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0x67e3 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_DISP8 as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_fcomp as std::os::raw::c_int as uint16_t,
                opcode: if 0xd8d9 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xd8d9 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xd8d9 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xd8d9 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xd8d9 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 0 as std::os::raw::c_int as uint8_t,
                op_type: [0 as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_fadd as std::os::raw::c_int as uint16_t,
                opcode: if 0xd8c0 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xd8c0 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xd8c0 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xd8c0 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x40 as std::os::raw::c_int
                    | 0x4 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xd8c0 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_ST as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_fadd as std::os::raw::c_int as uint16_t,
                opcode: if 0xd8c0 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xd8c0 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xd8c0 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xd8c0 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x40 as std::os::raw::c_int
                    | 0x4 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xd8c0 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_ST as std::os::raw::c_int as uint8_t,
                    OPT_ST0 as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_fadd as std::os::raw::c_int as uint16_t,
                opcode: if 0xdcc0 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xdcc0 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xdcc0 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xdcc0 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x40 as std::os::raw::c_int
                    | 0x4 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xdcc0 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_ST0 as std::os::raw::c_int as uint8_t,
                    OPT_ST as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_fmul as std::os::raw::c_int as uint16_t,
                opcode: if 0xdcc8 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xdcc8 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xdcc8 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xdcc8 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x40 as std::os::raw::c_int
                    | 0x4 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xdcc8 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_ST0 as std::os::raw::c_int as uint8_t,
                    OPT_ST as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_fadd as std::os::raw::c_int as uint16_t,
                opcode: if 0xdec1 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xdec1 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xdec1 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xdec1 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x40 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xdec1 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 0 as std::os::raw::c_int as uint8_t,
                op_type: [0 as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_faddp as std::os::raw::c_int as uint16_t,
                opcode: if 0xdec0 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xdec0 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xdec0 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xdec0 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x40 as std::os::raw::c_int
                    | 0x4 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xdec0 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_ST as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_faddp as std::os::raw::c_int as uint16_t,
                opcode: if 0xdec0 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xdec0 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xdec0 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xdec0 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x40 as std::os::raw::c_int
                    | 0x4 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xdec0 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_ST as std::os::raw::c_int as uint8_t,
                    OPT_ST0 as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_faddp as std::os::raw::c_int as uint16_t,
                opcode: if 0xdec0 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xdec0 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xdec0 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xdec0 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x40 as std::os::raw::c_int
                    | 0x4 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xdec0 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_ST0 as std::os::raw::c_int as uint8_t,
                    OPT_ST as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_faddp as std::os::raw::c_int as uint16_t,
                opcode: if 0xdec1 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xdec1 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xdec1 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xdec1 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x40 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xdec1 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 0 as std::os::raw::c_int as uint8_t,
                op_type: [0 as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_fadds as std::os::raw::c_int as uint16_t,
                opcode: if 0xd8 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xd8 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xd8 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xd8 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x40 as std::os::raw::c_int
                    | 0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xd8 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_EA as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_fiaddl as std::os::raw::c_int as uint16_t,
                opcode: if 0xda as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xda as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xda as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xda as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x40 as std::os::raw::c_int
                    | 0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xda as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_EA as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_faddl as std::os::raw::c_int as uint16_t,
                opcode: if 0xdc as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xdc as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xdc as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xdc as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x40 as std::os::raw::c_int
                    | 0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xdc as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_EA as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_fiadds as std::os::raw::c_int as uint16_t,
                opcode: if 0xde as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xde as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xde as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xde as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x40 as std::os::raw::c_int
                    | 0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xde as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_EA as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_fld as std::os::raw::c_int as uint16_t,
                opcode: if 0xd9c0 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xd9c0 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xd9c0 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xd9c0 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x4 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xd9c0 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_ST as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_fldl as std::os::raw::c_int as uint16_t,
                opcode: if 0xd9c0 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xd9c0 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xd9c0 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xd9c0 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x4 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xd9c0 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_ST as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_flds as std::os::raw::c_int as uint16_t,
                opcode: if 0xd9 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xd9 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xd9 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xd9 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xd9 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_EA as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_fldl as std::os::raw::c_int as uint16_t,
                opcode: if 0xdd as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xdd as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xdd as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xdd as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xdd as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_EA as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_fildl as std::os::raw::c_int as uint16_t,
                opcode: if 0xdb as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xdb as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xdb as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xdb as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xdb as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_EA as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_fildq as std::os::raw::c_int as uint16_t,
                opcode: if 0xdf as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xdf as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xdf as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xdf as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (5 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xdf as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_EA as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_fildll as std::os::raw::c_int as uint16_t,
                opcode: if 0xdf as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xdf as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xdf as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xdf as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (5 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xdf as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_EA as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_fldt as std::os::raw::c_int as uint16_t,
                opcode: if 0xdb as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xdb as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xdb as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xdb as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (5 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xdb as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_EA as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_fbld as std::os::raw::c_int as uint16_t,
                opcode: if 0xdf as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xdf as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xdf as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xdf as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (4 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xdf as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_EA as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_fst as std::os::raw::c_int as uint16_t,
                opcode: if 0xddd0 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xddd0 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xddd0 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xddd0 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x4 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xddd0 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_ST as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_fstl as std::os::raw::c_int as uint16_t,
                opcode: if 0xddd0 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xddd0 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xddd0 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xddd0 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x4 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xddd0 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_ST as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_fsts as std::os::raw::c_int as uint16_t,
                opcode: if 0xd9 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xd9 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xd9 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xd9 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (2 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xd9 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_EA as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_fstps as std::os::raw::c_int as uint16_t,
                opcode: if 0xd9 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xd9 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xd9 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xd9 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (3 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xd9 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_EA as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_fstl as std::os::raw::c_int as uint16_t,
                opcode: if 0xdd as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xdd as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xdd as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xdd as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (2 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xdd as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_EA as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_fstpl as std::os::raw::c_int as uint16_t,
                opcode: if 0xdd as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xdd as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xdd as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xdd as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (3 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xdd as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_EA as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_fist as std::os::raw::c_int as uint16_t,
                opcode: if 0xdf as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xdf as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xdf as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xdf as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (2 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xdf as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_EA as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_fistp as std::os::raw::c_int as uint16_t,
                opcode: if 0xdf as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xdf as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xdf as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xdf as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (3 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xdf as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_EA as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_fistl as std::os::raw::c_int as uint16_t,
                opcode: if 0xdb as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xdb as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xdb as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xdb as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (2 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xdb as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_EA as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_fistpl as std::os::raw::c_int as uint16_t,
                opcode: if 0xdb as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xdb as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xdb as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xdb as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (3 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xdb as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_EA as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_fstp as std::os::raw::c_int as uint16_t,
                opcode: if 0xddd8 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xddd8 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xddd8 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xddd8 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x4 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xddd8 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_ST as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_fistpq as std::os::raw::c_int as uint16_t,
                opcode: if 0xdf as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xdf as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xdf as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xdf as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (7 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xdf as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_EA as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_fistpll as std::os::raw::c_int as uint16_t,
                opcode: if 0xdf as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xdf as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xdf as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xdf as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (7 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xdf as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_EA as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_fstpt as std::os::raw::c_int as uint16_t,
                opcode: if 0xdb as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xdb as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xdb as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xdb as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (7 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xdb as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_EA as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_fbstp as std::os::raw::c_int as uint16_t,
                opcode: if 0xdf as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xdf as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xdf as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xdf as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (6 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xdf as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_EA as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_fxch as std::os::raw::c_int as uint16_t,
                opcode: if 0xd9c8 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xd9c8 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xd9c8 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xd9c8 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x4 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xd9c8 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_ST as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_fucom as std::os::raw::c_int as uint16_t,
                opcode: if 0xdde0 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xdde0 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xdde0 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xdde0 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x4 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xdde0 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_ST as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_fucomp as std::os::raw::c_int as uint16_t,
                opcode: if 0xdde8 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xdde8 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xdde8 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xdde8 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x4 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xdde8 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_ST as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_finit as std::os::raw::c_int as uint16_t,
                opcode: if 0xdbe3 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xdbe3 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xdbe3 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xdbe3 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x10 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xdbe3 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 0 as std::os::raw::c_int as uint8_t,
                op_type: [0 as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_fldcw as std::os::raw::c_int as uint16_t,
                opcode: if 0xd9 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xd9 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xd9 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xd9 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (5 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xd9 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_EA as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_fnstcw as std::os::raw::c_int as uint16_t,
                opcode: if 0xd9 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xd9 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xd9 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xd9 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (7 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xd9 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_EA as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_fstcw as std::os::raw::c_int as uint16_t,
                opcode: if 0xd9 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xd9 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xd9 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xd9 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | 0x10 as std::os::raw::c_int
                    | (7 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xd9 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_EA as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_fnstsw as std::os::raw::c_int as uint16_t,
                opcode: if 0xdfe0 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xdfe0 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xdfe0 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xdfe0 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xdfe0 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_EAX as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_fnstsw as std::os::raw::c_int as uint16_t,
                opcode: if 0xdd as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xdd as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xdd as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xdd as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (7 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xdd as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_EA as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_fstsw as std::os::raw::c_int as uint16_t,
                opcode: if 0xdfe0 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xdfe0 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xdfe0 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xdfe0 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x10 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xdfe0 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_EAX as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_fstsw as std::os::raw::c_int as uint16_t,
                opcode: if 0xdfe0 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xdfe0 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xdfe0 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xdfe0 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x10 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xdfe0 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 0 as std::os::raw::c_int as uint8_t,
                op_type: [0 as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_fstsw as std::os::raw::c_int as uint16_t,
                opcode: if 0xdd as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xdd as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xdd as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xdd as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | 0x10 as std::os::raw::c_int
                    | (7 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xdd as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_EA as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_fclex as std::os::raw::c_int as uint16_t,
                opcode: if 0xdbe2 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xdbe2 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xdbe2 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xdbe2 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x10 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xdbe2 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 0 as std::os::raw::c_int as uint8_t,
                op_type: [0 as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_fnstenv as std::os::raw::c_int as uint16_t,
                opcode: if 0xd9 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xd9 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xd9 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xd9 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (6 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xd9 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_EA as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_fstenv as std::os::raw::c_int as uint16_t,
                opcode: if 0xd9 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xd9 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xd9 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xd9 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | 0x10 as std::os::raw::c_int
                    | (6 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xd9 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_EA as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_fldenv as std::os::raw::c_int as uint16_t,
                opcode: if 0xd9 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xd9 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xd9 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xd9 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (4 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xd9 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_EA as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_fnsave as std::os::raw::c_int as uint16_t,
                opcode: if 0xdd as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xdd as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xdd as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xdd as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (6 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xdd as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_EA as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_fsave as std::os::raw::c_int as uint16_t,
                opcode: if 0xdd as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xdd as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xdd as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xdd as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | 0x10 as std::os::raw::c_int
                    | (6 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xdd as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_EA as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_frstor as std::os::raw::c_int as uint16_t,
                opcode: if 0xdd as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xdd as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xdd as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xdd as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (4 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xdd as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_EA as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_ffree as std::os::raw::c_int as uint16_t,
                opcode: if 0xddc0 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xddc0 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xddc0 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xddc0 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x4 as std::os::raw::c_int
                    | (4 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xddc0 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_ST as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_ffreep as std::os::raw::c_int as uint16_t,
                opcode: if 0xdfc0 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xdfc0 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xdfc0 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xdfc0 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x4 as std::os::raw::c_int
                    | (4 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xdfc0 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_ST as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_fxsave as std::os::raw::c_int as uint16_t,
                opcode: if 0xfae as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xfae as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xfae as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xfae as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xfae as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_EA as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_fxrstor as std::os::raw::c_int as uint16_t,
                opcode: if 0xfae as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xfae as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xfae as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xfae as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (1 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xfae as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_EA as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_fxsaveq as std::os::raw::c_int as uint16_t,
                opcode: if 0xfae as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xfae as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xfae as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xfae as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | 0x200 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xfae as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_EA as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_fxrstorq as std::os::raw::c_int as uint16_t,
                opcode: if 0xfae as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xfae as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xfae as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xfae as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | 0x200 as std::os::raw::c_int
                    | (1 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xfae as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_EA as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_arpl as std::os::raw::c_int as uint16_t,
                opcode: if 0x63 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0x63 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0x63 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0x63 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0x63 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_REG16 as std::os::raw::c_int as uint8_t,
                    (OPT_REG16 as std::os::raw::c_int | OPT_EA as std::os::raw::c_int) as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_larw as std::os::raw::c_int as uint16_t,
                opcode: if 0xf02 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf02 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf02 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf02 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | 0x1000 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf02 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_REG as std::os::raw::c_int | OPT_EA as std::os::raw::c_int) as uint8_t,
                    OPT_REG as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_lgdt as std::os::raw::c_int as uint16_t,
                opcode: if 0xf01 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf01 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf01 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf01 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (2 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf01 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_EA as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_lgdtq as std::os::raw::c_int as uint16_t,
                opcode: if 0xf01 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf01 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf01 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf01 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (2 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf01 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_EA as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_lidt as std::os::raw::c_int as uint16_t,
                opcode: if 0xf01 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf01 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf01 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf01 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (3 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf01 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_EA as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_lidtq as std::os::raw::c_int as uint16_t,
                opcode: if 0xf01 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf01 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf01 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf01 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (3 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf01 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_EA as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_lldt as std::os::raw::c_int as uint16_t,
                opcode: if 0xf00 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf00 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf00 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf00 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (2 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf00 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_EA as std::os::raw::c_int | OPT_REG as std::os::raw::c_int) as uint8_t,
                    0,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_lmsw as std::os::raw::c_int as uint16_t,
                opcode: if 0xf01 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf01 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf01 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf01 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (6 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf01 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_EA as std::os::raw::c_int | OPT_REG as std::os::raw::c_int) as uint8_t,
                    0,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_lslw as std::os::raw::c_int as uint16_t,
                opcode: if 0xf03 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf03 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf03 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf03 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | 0x1000 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf03 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_EA as std::os::raw::c_int | OPT_REG as std::os::raw::c_int) as uint8_t,
                    OPT_REG as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_ltr as std::os::raw::c_int as uint16_t,
                opcode: if 0xf00 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf00 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf00 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf00 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (3 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf00 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_EA as std::os::raw::c_int | OPT_REG16 as std::os::raw::c_int) as uint8_t,
                    0,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_sgdt as std::os::raw::c_int as uint16_t,
                opcode: if 0xf01 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf01 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf01 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf01 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf01 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_EA as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_sgdtq as std::os::raw::c_int as uint16_t,
                opcode: if 0xf01 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf01 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf01 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf01 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf01 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_EA as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_sidt as std::os::raw::c_int as uint16_t,
                opcode: if 0xf01 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf01 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf01 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf01 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (1 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf01 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_EA as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_sidtq as std::os::raw::c_int as uint16_t,
                opcode: if 0xf01 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf01 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf01 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf01 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (1 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf01 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_EA as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_sldt as std::os::raw::c_int as uint16_t,
                opcode: if 0xf00 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf00 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf00 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf00 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf00 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_REG as std::os::raw::c_int | OPT_EA as std::os::raw::c_int) as uint8_t,
                    0,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_smsw as std::os::raw::c_int as uint16_t,
                opcode: if 0xf01 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf01 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf01 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf01 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (4 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf01 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_REG as std::os::raw::c_int | OPT_EA as std::os::raw::c_int) as uint8_t,
                    0,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_str as std::os::raw::c_int as uint16_t,
                opcode: if 0xf00 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf00 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf00 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf00 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (1 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf00 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_REG32 as std::os::raw::c_int | OPT_EA as std::os::raw::c_int) as uint8_t,
                    0,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_str as std::os::raw::c_int as uint16_t,
                opcode: if 0x660f00 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0x660f00 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0x660f00 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0x660f00 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (1 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0x660f00 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_REG16 as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_str as std::os::raw::c_int as uint16_t,
                opcode: if 0xf00 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf00 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf00 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf00 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | 0x200 as std::os::raw::c_int
                    | (1 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf00 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_REG64 as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_verr as std::os::raw::c_int as uint16_t,
                opcode: if 0xf00 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf00 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf00 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf00 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (4 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf00 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_REG as std::os::raw::c_int | OPT_EA as std::os::raw::c_int) as uint8_t,
                    0,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_verw as std::os::raw::c_int as uint16_t,
                opcode: if 0xf00 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf00 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf00 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf00 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (5 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf00 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_REG as std::os::raw::c_int | OPT_EA as std::os::raw::c_int) as uint8_t,
                    0,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_swapgs as std::os::raw::c_int as uint16_t,
                opcode: if 0xf01 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf01 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf01 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf01 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (7 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf01 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 0 as std::os::raw::c_int as uint8_t,
                op_type: [0 as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_bswap as std::os::raw::c_int as uint16_t,
                opcode: if 0xfc8 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xfc8 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xfc8 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xfc8 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x4 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xfc8 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_REG32 as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_bswapl as std::os::raw::c_int as uint16_t,
                opcode: if 0xfc8 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xfc8 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xfc8 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xfc8 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x4 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xfc8 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_REG32 as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_bswapq as std::os::raw::c_int as uint16_t,
                opcode: if 0xfc8 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xfc8 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xfc8 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xfc8 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x4 as std::os::raw::c_int
                    | 0x200 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xfc8 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_REG64 as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_xaddb as std::os::raw::c_int as uint16_t,
                opcode: if 0xfc0 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xfc0 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xfc0 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xfc0 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0x1 as std::os::raw::c_int | 0x1000 as std::os::raw::c_int)
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xfc0 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_REG as std::os::raw::c_int as uint8_t,
                    (OPT_REG as std::os::raw::c_int | OPT_EA as std::os::raw::c_int) as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_cmpxchgb as std::os::raw::c_int as uint16_t,
                opcode: if 0xfb0 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xfb0 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xfb0 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xfb0 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0x1 as std::os::raw::c_int | 0x1000 as std::os::raw::c_int)
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xfb0 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_REG as std::os::raw::c_int as uint8_t,
                    (OPT_REG as std::os::raw::c_int | OPT_EA as std::os::raw::c_int) as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_invlpg as std::os::raw::c_int as uint16_t,
                opcode: if 0xf01 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf01 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf01 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf01 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (7 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf01 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_EA as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_cmpxchg8b as std::os::raw::c_int as uint16_t,
                opcode: if 0xfc7 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xfc7 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xfc7 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xfc7 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (1 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xfc7 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_EA as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_cmpxchg16b as std::os::raw::c_int as uint16_t,
                opcode: if 0xfc7 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xfc7 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xfc7 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xfc7 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | 0x200 as std::os::raw::c_int
                    | (1 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xfc7 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_EA as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_cmovo as std::os::raw::c_int as uint16_t,
                opcode: if 0xf40 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf40 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf40 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf40 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | 0x50 as std::os::raw::c_int
                    | 0x1000 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf40 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_REGW as std::os::raw::c_int | OPT_EA as std::os::raw::c_int) as uint8_t,
                    OPT_REGW as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_fcmovb as std::os::raw::c_int as uint16_t,
                opcode: if 0xdac0 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xdac0 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xdac0 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xdac0 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x4 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xdac0 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_ST as std::os::raw::c_int as uint8_t,
                    OPT_ST0 as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_fcmove as std::os::raw::c_int as uint16_t,
                opcode: if 0xdac8 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xdac8 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xdac8 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xdac8 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x4 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xdac8 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_ST as std::os::raw::c_int as uint8_t,
                    OPT_ST0 as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_fcmovbe as std::os::raw::c_int as uint16_t,
                opcode: if 0xdad0 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xdad0 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xdad0 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xdad0 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x4 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xdad0 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_ST as std::os::raw::c_int as uint8_t,
                    OPT_ST0 as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_fcmovu as std::os::raw::c_int as uint16_t,
                opcode: if 0xdad8 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xdad8 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xdad8 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xdad8 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x4 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xdad8 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_ST as std::os::raw::c_int as uint8_t,
                    OPT_ST0 as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_fcmovnb as std::os::raw::c_int as uint16_t,
                opcode: if 0xdbc0 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xdbc0 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xdbc0 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xdbc0 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x4 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xdbc0 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_ST as std::os::raw::c_int as uint8_t,
                    OPT_ST0 as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_fcmovne as std::os::raw::c_int as uint16_t,
                opcode: if 0xdbc8 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xdbc8 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xdbc8 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xdbc8 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x4 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xdbc8 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_ST as std::os::raw::c_int as uint8_t,
                    OPT_ST0 as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_fcmovnbe as std::os::raw::c_int as uint16_t,
                opcode: if 0xdbd0 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xdbd0 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xdbd0 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xdbd0 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x4 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xdbd0 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_ST as std::os::raw::c_int as uint8_t,
                    OPT_ST0 as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_fcmovnu as std::os::raw::c_int as uint16_t,
                opcode: if 0xdbd8 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xdbd8 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xdbd8 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xdbd8 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x4 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xdbd8 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_ST as std::os::raw::c_int as uint8_t,
                    OPT_ST0 as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_fucomi as std::os::raw::c_int as uint16_t,
                opcode: if 0xdbe8 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xdbe8 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xdbe8 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xdbe8 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x4 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xdbe8 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_ST as std::os::raw::c_int as uint8_t,
                    OPT_ST0 as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_fcomi as std::os::raw::c_int as uint16_t,
                opcode: if 0xdbf0 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xdbf0 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xdbf0 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xdbf0 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x4 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xdbf0 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_ST as std::os::raw::c_int as uint8_t,
                    OPT_ST0 as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_fucomip as std::os::raw::c_int as uint16_t,
                opcode: if 0xdfe8 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xdfe8 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xdfe8 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xdfe8 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x4 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xdfe8 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_ST as std::os::raw::c_int as uint8_t,
                    OPT_ST0 as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_fcomip as std::os::raw::c_int as uint16_t,
                opcode: if 0xdff0 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xdff0 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xdff0 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xdff0 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x4 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xdff0 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_ST as std::os::raw::c_int as uint8_t,
                    OPT_ST0 as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_movd as std::os::raw::c_int as uint16_t,
                opcode: if 0xf6e as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf6e as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf6e as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf6e as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf6e as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_EA as std::os::raw::c_int | OPT_REG32 as std::os::raw::c_int) as uint8_t,
                    OPT_MMXSSE as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_movd as std::os::raw::c_int as uint16_t,
                opcode: if 0xf6e as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf6e as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf6e as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf6e as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf6e as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_EA as std::os::raw::c_int | OPT_REG64 as std::os::raw::c_int) as uint8_t,
                    OPT_MMXSSE as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_movq as std::os::raw::c_int as uint16_t,
                opcode: if 0xf6e as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf6e as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf6e as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf6e as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | 0x200 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf6e as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_REG64 as std::os::raw::c_int as uint8_t,
                    OPT_MMXSSE as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_movq as std::os::raw::c_int as uint16_t,
                opcode: if 0xf6f as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf6f as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf6f as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf6f as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf6f as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_EA as std::os::raw::c_int | OPT_MMX as std::os::raw::c_int) as uint8_t,
                    OPT_MMX as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_movd as std::os::raw::c_int as uint16_t,
                opcode: if 0xf7e as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf7e as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf7e as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf7e as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf7e as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_MMXSSE as std::os::raw::c_int as uint8_t,
                    (OPT_EA as std::os::raw::c_int | OPT_REG32 as std::os::raw::c_int) as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_movd as std::os::raw::c_int as uint16_t,
                opcode: if 0xf7e as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf7e as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf7e as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf7e as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf7e as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_MMXSSE as std::os::raw::c_int as uint8_t,
                    (OPT_EA as std::os::raw::c_int | OPT_REG64 as std::os::raw::c_int) as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_movq as std::os::raw::c_int as uint16_t,
                opcode: if 0xf7f as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf7f as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf7f as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf7f as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf7f as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_MMX as std::os::raw::c_int as uint8_t,
                    (OPT_EA as std::os::raw::c_int | OPT_MMX as std::os::raw::c_int) as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_movq as std::os::raw::c_int as uint16_t,
                opcode: if 0x660fd6 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0x660fd6 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0x660fd6 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0x660fd6 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0x660fd6 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_SSE as std::os::raw::c_int as uint8_t,
                    (OPT_EA as std::os::raw::c_int | OPT_SSE as std::os::raw::c_int) as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_movq as std::os::raw::c_int as uint16_t,
                opcode: if 0xf30f7e as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf30f7e as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf30f7e as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf30f7e as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf30f7e as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_EA as std::os::raw::c_int | OPT_SSE as std::os::raw::c_int) as uint8_t,
                    OPT_SSE as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_movq as std::os::raw::c_int as uint16_t,
                opcode: if 0xf7e as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf7e as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf7e as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf7e as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf7e as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_MMXSSE as std::os::raw::c_int as uint8_t,
                    (OPT_EA as std::os::raw::c_int | OPT_REG64 as std::os::raw::c_int) as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_packssdw as std::os::raw::c_int as uint16_t,
                opcode: if 0xf6b as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf6b as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf6b as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf6b as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf6b as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_EA as std::os::raw::c_int | OPT_MMXSSE as std::os::raw::c_int) as uint8_t,
                    OPT_MMXSSE as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_packsswb as std::os::raw::c_int as uint16_t,
                opcode: if 0xf63 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf63 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf63 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf63 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf63 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_EA as std::os::raw::c_int | OPT_MMXSSE as std::os::raw::c_int) as uint8_t,
                    OPT_MMXSSE as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_packuswb as std::os::raw::c_int as uint16_t,
                opcode: if 0xf67 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf67 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf67 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf67 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf67 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_EA as std::os::raw::c_int | OPT_MMXSSE as std::os::raw::c_int) as uint8_t,
                    OPT_MMXSSE as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_paddb as std::os::raw::c_int as uint16_t,
                opcode: if 0xffc as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xffc as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xffc as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xffc as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xffc as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_EA as std::os::raw::c_int | OPT_MMXSSE as std::os::raw::c_int) as uint8_t,
                    OPT_MMXSSE as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_paddw as std::os::raw::c_int as uint16_t,
                opcode: if 0xffd as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xffd as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xffd as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xffd as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xffd as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_EA as std::os::raw::c_int | OPT_MMXSSE as std::os::raw::c_int) as uint8_t,
                    OPT_MMXSSE as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_paddd as std::os::raw::c_int as uint16_t,
                opcode: if 0xffe as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xffe as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xffe as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xffe as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xffe as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_EA as std::os::raw::c_int | OPT_MMXSSE as std::os::raw::c_int) as uint8_t,
                    OPT_MMXSSE as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_paddsb as std::os::raw::c_int as uint16_t,
                opcode: if 0xfec as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xfec as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xfec as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xfec as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xfec as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_EA as std::os::raw::c_int | OPT_MMXSSE as std::os::raw::c_int) as uint8_t,
                    OPT_MMXSSE as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_paddsw as std::os::raw::c_int as uint16_t,
                opcode: if 0xfed as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xfed as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xfed as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xfed as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xfed as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_EA as std::os::raw::c_int | OPT_MMXSSE as std::os::raw::c_int) as uint8_t,
                    OPT_MMXSSE as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_paddusb as std::os::raw::c_int as uint16_t,
                opcode: if 0xfdc as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xfdc as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xfdc as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xfdc as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xfdc as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_EA as std::os::raw::c_int | OPT_MMXSSE as std::os::raw::c_int) as uint8_t,
                    OPT_MMXSSE as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_paddusw as std::os::raw::c_int as uint16_t,
                opcode: if 0xfdd as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xfdd as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xfdd as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xfdd as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xfdd as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_EA as std::os::raw::c_int | OPT_MMXSSE as std::os::raw::c_int) as uint8_t,
                    OPT_MMXSSE as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_pand as std::os::raw::c_int as uint16_t,
                opcode: if 0xfdb as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xfdb as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xfdb as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xfdb as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xfdb as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_EA as std::os::raw::c_int | OPT_MMXSSE as std::os::raw::c_int) as uint8_t,
                    OPT_MMXSSE as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_pandn as std::os::raw::c_int as uint16_t,
                opcode: if 0xfdf as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xfdf as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xfdf as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xfdf as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xfdf as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_EA as std::os::raw::c_int | OPT_MMXSSE as std::os::raw::c_int) as uint8_t,
                    OPT_MMXSSE as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_pcmpeqb as std::os::raw::c_int as uint16_t,
                opcode: if 0xf74 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf74 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf74 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf74 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf74 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_EA as std::os::raw::c_int | OPT_MMXSSE as std::os::raw::c_int) as uint8_t,
                    OPT_MMXSSE as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_pcmpeqw as std::os::raw::c_int as uint16_t,
                opcode: if 0xf75 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf75 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf75 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf75 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf75 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_EA as std::os::raw::c_int | OPT_MMXSSE as std::os::raw::c_int) as uint8_t,
                    OPT_MMXSSE as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_pcmpeqd as std::os::raw::c_int as uint16_t,
                opcode: if 0xf76 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf76 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf76 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf76 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf76 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_EA as std::os::raw::c_int | OPT_MMXSSE as std::os::raw::c_int) as uint8_t,
                    OPT_MMXSSE as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_pcmpgtb as std::os::raw::c_int as uint16_t,
                opcode: if 0xf64 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf64 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf64 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf64 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf64 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_EA as std::os::raw::c_int | OPT_MMXSSE as std::os::raw::c_int) as uint8_t,
                    OPT_MMXSSE as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_pcmpgtw as std::os::raw::c_int as uint16_t,
                opcode: if 0xf65 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf65 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf65 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf65 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf65 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_EA as std::os::raw::c_int | OPT_MMXSSE as std::os::raw::c_int) as uint8_t,
                    OPT_MMXSSE as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_pcmpgtd as std::os::raw::c_int as uint16_t,
                opcode: if 0xf66 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf66 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf66 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf66 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf66 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_EA as std::os::raw::c_int | OPT_MMXSSE as std::os::raw::c_int) as uint8_t,
                    OPT_MMXSSE as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_pmaddwd as std::os::raw::c_int as uint16_t,
                opcode: if 0xff5 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xff5 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xff5 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xff5 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xff5 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_EA as std::os::raw::c_int | OPT_MMXSSE as std::os::raw::c_int) as uint8_t,
                    OPT_MMXSSE as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_pmulhw as std::os::raw::c_int as uint16_t,
                opcode: if 0xfe5 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xfe5 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xfe5 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xfe5 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xfe5 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_EA as std::os::raw::c_int | OPT_MMXSSE as std::os::raw::c_int) as uint8_t,
                    OPT_MMXSSE as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_pmullw as std::os::raw::c_int as uint16_t,
                opcode: if 0xfd5 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xfd5 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xfd5 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xfd5 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xfd5 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_EA as std::os::raw::c_int | OPT_MMXSSE as std::os::raw::c_int) as uint8_t,
                    OPT_MMXSSE as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_por as std::os::raw::c_int as uint16_t,
                opcode: if 0xfeb as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xfeb as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xfeb as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xfeb as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xfeb as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_EA as std::os::raw::c_int | OPT_MMXSSE as std::os::raw::c_int) as uint8_t,
                    OPT_MMXSSE as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_psllw as std::os::raw::c_int as uint16_t,
                opcode: if 0xff1 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xff1 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xff1 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xff1 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xff1 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_EA as std::os::raw::c_int | OPT_MMXSSE as std::os::raw::c_int) as uint8_t,
                    OPT_MMXSSE as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_psllw as std::os::raw::c_int as uint16_t,
                opcode: if 0xf71 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf71 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf71 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf71 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (6 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf71 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_IM8 as std::os::raw::c_int as uint8_t,
                    OPT_MMXSSE as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_pslld as std::os::raw::c_int as uint16_t,
                opcode: if 0xff2 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xff2 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xff2 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xff2 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xff2 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_EA as std::os::raw::c_int | OPT_MMXSSE as std::os::raw::c_int) as uint8_t,
                    OPT_MMXSSE as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_pslld as std::os::raw::c_int as uint16_t,
                opcode: if 0xf72 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf72 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf72 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf72 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (6 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf72 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_IM8 as std::os::raw::c_int as uint8_t,
                    OPT_MMXSSE as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_psllq as std::os::raw::c_int as uint16_t,
                opcode: if 0xff3 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xff3 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xff3 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xff3 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xff3 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_EA as std::os::raw::c_int | OPT_MMXSSE as std::os::raw::c_int) as uint8_t,
                    OPT_MMXSSE as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_psllq as std::os::raw::c_int as uint16_t,
                opcode: if 0xf73 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf73 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf73 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf73 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (6 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf73 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_IM8 as std::os::raw::c_int as uint8_t,
                    OPT_MMXSSE as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_psraw as std::os::raw::c_int as uint16_t,
                opcode: if 0xfe1 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xfe1 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xfe1 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xfe1 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xfe1 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_EA as std::os::raw::c_int | OPT_MMXSSE as std::os::raw::c_int) as uint8_t,
                    OPT_MMXSSE as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_psraw as std::os::raw::c_int as uint16_t,
                opcode: if 0xf71 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf71 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf71 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf71 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (4 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf71 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_IM8 as std::os::raw::c_int as uint8_t,
                    OPT_MMXSSE as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_psrad as std::os::raw::c_int as uint16_t,
                opcode: if 0xfe2 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xfe2 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xfe2 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xfe2 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xfe2 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_EA as std::os::raw::c_int | OPT_MMXSSE as std::os::raw::c_int) as uint8_t,
                    OPT_MMXSSE as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_psrad as std::os::raw::c_int as uint16_t,
                opcode: if 0xf72 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf72 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf72 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf72 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (4 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf72 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_IM8 as std::os::raw::c_int as uint8_t,
                    OPT_MMXSSE as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_psrlw as std::os::raw::c_int as uint16_t,
                opcode: if 0xfd1 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xfd1 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xfd1 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xfd1 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xfd1 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_EA as std::os::raw::c_int | OPT_MMXSSE as std::os::raw::c_int) as uint8_t,
                    OPT_MMXSSE as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_psrlw as std::os::raw::c_int as uint16_t,
                opcode: if 0xf71 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf71 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf71 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf71 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (2 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf71 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_IM8 as std::os::raw::c_int as uint8_t,
                    OPT_MMXSSE as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_psrld as std::os::raw::c_int as uint16_t,
                opcode: if 0xfd2 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xfd2 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xfd2 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xfd2 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xfd2 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_EA as std::os::raw::c_int | OPT_MMXSSE as std::os::raw::c_int) as uint8_t,
                    OPT_MMXSSE as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_psrld as std::os::raw::c_int as uint16_t,
                opcode: if 0xf72 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf72 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf72 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf72 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (2 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf72 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_IM8 as std::os::raw::c_int as uint8_t,
                    OPT_MMXSSE as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_psrlq as std::os::raw::c_int as uint16_t,
                opcode: if 0xfd3 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xfd3 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xfd3 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xfd3 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xfd3 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_EA as std::os::raw::c_int | OPT_MMXSSE as std::os::raw::c_int) as uint8_t,
                    OPT_MMXSSE as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_psrlq as std::os::raw::c_int as uint16_t,
                opcode: if 0xf73 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf73 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf73 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf73 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (2 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf73 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_IM8 as std::os::raw::c_int as uint8_t,
                    OPT_MMXSSE as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_psubb as std::os::raw::c_int as uint16_t,
                opcode: if 0xff8 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xff8 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xff8 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xff8 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xff8 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_EA as std::os::raw::c_int | OPT_MMXSSE as std::os::raw::c_int) as uint8_t,
                    OPT_MMXSSE as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_psubw as std::os::raw::c_int as uint16_t,
                opcode: if 0xff9 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xff9 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xff9 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xff9 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xff9 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_EA as std::os::raw::c_int | OPT_MMXSSE as std::os::raw::c_int) as uint8_t,
                    OPT_MMXSSE as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_psubd as std::os::raw::c_int as uint16_t,
                opcode: if 0xffa as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xffa as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xffa as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xffa as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xffa as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_EA as std::os::raw::c_int | OPT_MMXSSE as std::os::raw::c_int) as uint8_t,
                    OPT_MMXSSE as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_psubsb as std::os::raw::c_int as uint16_t,
                opcode: if 0xfe8 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xfe8 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xfe8 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xfe8 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xfe8 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_EA as std::os::raw::c_int | OPT_MMXSSE as std::os::raw::c_int) as uint8_t,
                    OPT_MMXSSE as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_psubsw as std::os::raw::c_int as uint16_t,
                opcode: if 0xfe9 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xfe9 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xfe9 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xfe9 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xfe9 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_EA as std::os::raw::c_int | OPT_MMXSSE as std::os::raw::c_int) as uint8_t,
                    OPT_MMXSSE as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_psubusb as std::os::raw::c_int as uint16_t,
                opcode: if 0xfd8 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xfd8 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xfd8 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xfd8 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xfd8 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_EA as std::os::raw::c_int | OPT_MMXSSE as std::os::raw::c_int) as uint8_t,
                    OPT_MMXSSE as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_psubusw as std::os::raw::c_int as uint16_t,
                opcode: if 0xfd9 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xfd9 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xfd9 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xfd9 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xfd9 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_EA as std::os::raw::c_int | OPT_MMXSSE as std::os::raw::c_int) as uint8_t,
                    OPT_MMXSSE as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_punpckhbw as std::os::raw::c_int as uint16_t,
                opcode: if 0xf68 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf68 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf68 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf68 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf68 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_EA as std::os::raw::c_int | OPT_MMXSSE as std::os::raw::c_int) as uint8_t,
                    OPT_MMXSSE as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_punpckhwd as std::os::raw::c_int as uint16_t,
                opcode: if 0xf69 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf69 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf69 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf69 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf69 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_EA as std::os::raw::c_int | OPT_MMXSSE as std::os::raw::c_int) as uint8_t,
                    OPT_MMXSSE as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_punpckhdq as std::os::raw::c_int as uint16_t,
                opcode: if 0xf6a as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf6a as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf6a as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf6a as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf6a as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_EA as std::os::raw::c_int | OPT_MMXSSE as std::os::raw::c_int) as uint8_t,
                    OPT_MMXSSE as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_punpcklbw as std::os::raw::c_int as uint16_t,
                opcode: if 0xf60 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf60 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf60 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf60 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf60 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_EA as std::os::raw::c_int | OPT_MMXSSE as std::os::raw::c_int) as uint8_t,
                    OPT_MMXSSE as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_punpcklwd as std::os::raw::c_int as uint16_t,
                opcode: if 0xf61 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf61 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf61 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf61 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf61 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_EA as std::os::raw::c_int | OPT_MMXSSE as std::os::raw::c_int) as uint8_t,
                    OPT_MMXSSE as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_punpckldq as std::os::raw::c_int as uint16_t,
                opcode: if 0xf62 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf62 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf62 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf62 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf62 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_EA as std::os::raw::c_int | OPT_MMXSSE as std::os::raw::c_int) as uint8_t,
                    OPT_MMXSSE as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_pxor as std::os::raw::c_int as uint16_t,
                opcode: if 0xfef as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xfef as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xfef as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xfef as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xfef as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_EA as std::os::raw::c_int | OPT_MMXSSE as std::os::raw::c_int) as uint8_t,
                    OPT_MMXSSE as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_movups as std::os::raw::c_int as uint16_t,
                opcode: if 0xf10 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf10 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf10 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf10 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf10 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_EA as std::os::raw::c_int | OPT_REG32 as std::os::raw::c_int) as uint8_t,
                    OPT_SSE as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_movups as std::os::raw::c_int as uint16_t,
                opcode: if 0xf11 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf11 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf11 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf11 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf11 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_SSE as std::os::raw::c_int as uint8_t,
                    (OPT_EA as std::os::raw::c_int | OPT_REG32 as std::os::raw::c_int) as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_movaps as std::os::raw::c_int as uint16_t,
                opcode: if 0xf28 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf28 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf28 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf28 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf28 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_EA as std::os::raw::c_int | OPT_REG32 as std::os::raw::c_int) as uint8_t,
                    OPT_SSE as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_movaps as std::os::raw::c_int as uint16_t,
                opcode: if 0xf29 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf29 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf29 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf29 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf29 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_SSE as std::os::raw::c_int as uint8_t,
                    (OPT_EA as std::os::raw::c_int | OPT_REG32 as std::os::raw::c_int) as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_movhps as std::os::raw::c_int as uint16_t,
                opcode: if 0xf16 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf16 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf16 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf16 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf16 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_EA as std::os::raw::c_int | OPT_REG32 as std::os::raw::c_int) as uint8_t,
                    OPT_SSE as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_movhps as std::os::raw::c_int as uint16_t,
                opcode: if 0xf17 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf17 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf17 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf17 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf17 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    OPT_SSE as std::os::raw::c_int as uint8_t,
                    (OPT_EA as std::os::raw::c_int | OPT_REG32 as std::os::raw::c_int) as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_addps as std::os::raw::c_int as uint16_t,
                opcode: if 0xf58 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf58 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf58 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf58 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf58 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_EA as std::os::raw::c_int | OPT_SSE as std::os::raw::c_int) as uint8_t,
                    OPT_SSE as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_cvtpi2ps as std::os::raw::c_int as uint16_t,
                opcode: if 0xf2a as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf2a as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf2a as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf2a as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf2a as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_EA as std::os::raw::c_int | OPT_MMX as std::os::raw::c_int) as uint8_t,
                    OPT_SSE as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_cvtps2pi as std::os::raw::c_int as uint16_t,
                opcode: if 0xf2d as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf2d as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf2d as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf2d as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf2d as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_EA as std::os::raw::c_int | OPT_SSE as std::os::raw::c_int) as uint8_t,
                    OPT_MMX as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_cvttps2pi as std::os::raw::c_int as uint16_t,
                opcode: if 0xf2c as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf2c as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf2c as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf2c as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf2c as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_EA as std::os::raw::c_int | OPT_SSE as std::os::raw::c_int) as uint8_t,
                    OPT_MMX as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_divps as std::os::raw::c_int as uint16_t,
                opcode: if 0xf5e as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf5e as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf5e as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf5e as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf5e as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_EA as std::os::raw::c_int | OPT_SSE as std::os::raw::c_int) as uint8_t,
                    OPT_SSE as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_maxps as std::os::raw::c_int as uint16_t,
                opcode: if 0xf5f as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf5f as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf5f as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf5f as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf5f as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_EA as std::os::raw::c_int | OPT_SSE as std::os::raw::c_int) as uint8_t,
                    OPT_SSE as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_minps as std::os::raw::c_int as uint16_t,
                opcode: if 0xf5d as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf5d as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf5d as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf5d as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf5d as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_EA as std::os::raw::c_int | OPT_SSE as std::os::raw::c_int) as uint8_t,
                    OPT_SSE as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_mulps as std::os::raw::c_int as uint16_t,
                opcode: if 0xf59 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf59 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf59 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf59 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf59 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_EA as std::os::raw::c_int | OPT_SSE as std::os::raw::c_int) as uint8_t,
                    OPT_SSE as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_pavgb as std::os::raw::c_int as uint16_t,
                opcode: if 0xfe0 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xfe0 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xfe0 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xfe0 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xfe0 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_EA as std::os::raw::c_int | OPT_SSE as std::os::raw::c_int) as uint8_t,
                    OPT_SSE as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_pavgw as std::os::raw::c_int as uint16_t,
                opcode: if 0xfe3 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xfe3 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xfe3 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xfe3 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xfe3 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_EA as std::os::raw::c_int | OPT_SSE as std::os::raw::c_int) as uint8_t,
                    OPT_SSE as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_pmaxsw as std::os::raw::c_int as uint16_t,
                opcode: if 0xfee as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xfee as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xfee as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xfee as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xfee as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_EA as std::os::raw::c_int | OPT_MMXSSE as std::os::raw::c_int) as uint8_t,
                    OPT_MMXSSE as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_pmaxub as std::os::raw::c_int as uint16_t,
                opcode: if 0xfde as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xfde as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xfde as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xfde as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xfde as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_EA as std::os::raw::c_int | OPT_MMXSSE as std::os::raw::c_int) as uint8_t,
                    OPT_MMXSSE as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_pminsw as std::os::raw::c_int as uint16_t,
                opcode: if 0xfea as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xfea as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xfea as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xfea as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xfea as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_EA as std::os::raw::c_int | OPT_MMXSSE as std::os::raw::c_int) as uint8_t,
                    OPT_MMXSSE as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_pminub as std::os::raw::c_int as uint16_t,
                opcode: if 0xfda as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xfda as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xfda as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xfda as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xfda as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_EA as std::os::raw::c_int | OPT_MMXSSE as std::os::raw::c_int) as uint8_t,
                    OPT_MMXSSE as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_rcpss as std::os::raw::c_int as uint16_t,
                opcode: if 0xf53 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf53 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf53 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf53 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf53 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_EA as std::os::raw::c_int | OPT_SSE as std::os::raw::c_int) as uint8_t,
                    OPT_SSE as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_rsqrtps as std::os::raw::c_int as uint16_t,
                opcode: if 0xf52 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf52 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf52 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf52 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf52 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_EA as std::os::raw::c_int | OPT_SSE as std::os::raw::c_int) as uint8_t,
                    OPT_SSE as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_sqrtps as std::os::raw::c_int as uint16_t,
                opcode: if 0xf51 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf51 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf51 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf51 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf51 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_EA as std::os::raw::c_int | OPT_SSE as std::os::raw::c_int) as uint8_t,
                    OPT_SSE as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_subps as std::os::raw::c_int as uint16_t,
                opcode: if 0xf5c as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf5c as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf5c as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf5c as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf5c as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 2 as std::os::raw::c_int as uint8_t,
                op_type: [
                    (OPT_EA as std::os::raw::c_int | OPT_SSE as std::os::raw::c_int) as uint8_t,
                    OPT_SSE as std::os::raw::c_int as uint8_t,
                    0,
                ],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_prefetchnta as std::os::raw::c_int as uint16_t,
                opcode: if 0xf18 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf18 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf18 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf18 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (0 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf18 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_EA as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_prefetcht0 as std::os::raw::c_int as uint16_t,
                opcode: if 0xf18 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf18 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf18 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf18 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (1 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf18 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_EA as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_prefetcht1 as std::os::raw::c_int as uint16_t,
                opcode: if 0xf18 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf18 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf18 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf18 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (2 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf18 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_EA as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_prefetcht2 as std::os::raw::c_int as uint16_t,
                opcode: if 0xf18 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf18 as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf18 as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf18 as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (3 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf18 as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_EA as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_prefetchw as std::os::raw::c_int as uint16_t,
                opcode: if 0xf0d as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xf0d as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xf0d as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xf0d as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (1 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xf0d as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_EA as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_lfence as std::os::raw::c_int as uint16_t,
                opcode: if 0xfae as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xfae as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xfae as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xfae as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (5 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xfae as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 0 as std::os::raw::c_int as uint8_t,
                op_type: [0 as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_mfence as std::os::raw::c_int as uint16_t,
                opcode: if 0xfae as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xfae as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xfae as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xfae as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (6 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xfae as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 0 as std::os::raw::c_int as uint8_t,
                op_type: [0 as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_sfence as std::os::raw::c_int as uint16_t,
                opcode: if 0xfae as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xfae as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xfae as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xfae as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (7 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xfae as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 0 as std::os::raw::c_int as uint8_t,
                op_type: [0 as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: TOK_ASM_clflush as std::os::raw::c_int as uint16_t,
                opcode: if 0xfae as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                    == 0xf00 as std::os::raw::c_int
                {
                    (0xfae as std::os::raw::c_int >> 8 as std::os::raw::c_int
                        & !(0xff as std::os::raw::c_int))
                        | 0xfae as std::os::raw::c_int & 0xff as std::os::raw::c_int
                } else {
                    0xfae as std::os::raw::c_int
                } as uint64_t as uint16_t,
                instr_type: (0x8 as std::os::raw::c_int
                    | (7 as std::os::raw::c_int) << 13 as std::os::raw::c_int
                    | (if 0xfae as std::os::raw::c_int & 0xff00 as std::os::raw::c_int
                        == 0xf00 as std::os::raw::c_int
                    {
                        0x100 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    })) as uint16_t,
                nb_ops: 1 as std::os::raw::c_int as uint8_t,
                op_type: [OPT_EA as std::os::raw::c_int as uint8_t, 0, 0],
            };
            init
        },
        {
            let mut init = ASMInstr {
                sym: 0 as std::os::raw::c_int as uint16_t,
                opcode: 0,
                instr_type: 0,
                nb_ops: 0,
                op_type: [0; 3],
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
