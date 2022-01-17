use crate::bitfields::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type sym_version;
    #[no_mangle]
    fn memset(
        _: *mut std::os::raw::c_void,
        _: std::os::raw::c_int,
        _: std::os::raw::c_ulong,
    ) -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn strcpy(
        _: *mut std::os::raw::c_char,
        _: *const std::os::raw::c_char,
    ) -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn strcmp(
        _: *const std::os::raw::c_char,
        _: *const std::os::raw::c_char,
    ) -> std::os::raw::c_int;
    #[no_mangle]
    fn strchr(_: *const std::os::raw::c_char, _: std::os::raw::c_int) -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn memcpy(
        _: *mut std::os::raw::c_void,
        _: *const std::os::raw::c_void,
        _: std::os::raw::c_ulong,
    ) -> *mut std::os::raw::c_void;
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
    fn asm_clobber(clobber_regs: *mut uint8_t, str: *const std::os::raw::c_char);
    #[no_mangle]
    fn asm_compute_constraints(
        operands: *mut ASMOperand,
        nb_operands: std::os::raw::c_int,
        nb_outputs: std::os::raw::c_int,
        clobber_regs: *const uint8_t,
        pout_reg: *mut std::os::raw::c_int,
    );
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
    static mut tcc_state: *mut TCCState;
    #[no_mangle]
    fn pstrcat(
        buf: *mut std::os::raw::c_char,
        buf_size: size_t,
        s: *const std::os::raw::c_char,
    ) -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn tcc_free(ptr: *mut std::os::raw::c_void);
    #[no_mangle]
    fn tcc_malloc(size: std::os::raw::c_ulong) -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn _tcc_error(fmt: *const std::os::raw::c_char, _: ...) -> !;
    #[no_mangle]
    fn _tcc_warning(fmt: *const std::os::raw::c_char, _: ...);
    #[no_mangle]
    fn cstr_ccat(cstr: *mut CString, ch: std::os::raw::c_int);
    #[no_mangle]
    fn cstr_new(cstr: *mut CString);
    #[no_mangle]
    fn cstr_free(cstr: *mut CString);
    #[no_mangle]
    fn tcc_open_bf(
        s1: *mut TCCState,
        filename: *const std::os::raw::c_char,
        initlen: std::os::raw::c_int,
    );
    #[no_mangle]
    fn tcc_close();
    #[no_mangle]
    static mut file: *mut BufferedFile;
    #[no_mangle]
    static mut tok: std::os::raw::c_int;
    #[no_mangle]
    static mut tokc: CValue;
    #[no_mangle]
    static mut macro_ptr: *const std::os::raw::c_int;
    #[no_mangle]
    static mut parse_flags: std::os::raw::c_int;
    #[no_mangle]
    fn tok_alloc(str: *const std::os::raw::c_char, len: std::os::raw::c_int) -> *mut TokenSym;
    #[no_mangle]
    fn tok_alloc_const(str: *const std::os::raw::c_char) -> std::os::raw::c_int;
    #[no_mangle]
    fn get_tok_str(v: std::os::raw::c_int, cv: *mut CValue) -> *const std::os::raw::c_char;
    #[no_mangle]
    fn begin_macro(str: *mut TokenString, alloc: std::os::raw::c_int);
    #[no_mangle]
    fn end_macro();
    #[no_mangle]
    fn set_idnum(c: std::os::raw::c_int, val: std::os::raw::c_int) -> std::os::raw::c_int;
    #[no_mangle]
    fn tok_str_alloc() -> *mut TokenString;
    #[no_mangle]
    fn tok_str_add(s: *mut TokenString, t: std::os::raw::c_int);
    #[no_mangle]
    fn tok_str_add_tok(s: *mut TokenString);
    #[no_mangle]
    fn next();
    #[no_mangle]
    fn skip(c: std::os::raw::c_int);
    #[no_mangle]
    fn expect(msg: *const std::os::raw::c_char) -> !;
    #[no_mangle]
    static mut vtop: *mut SValue;
    #[no_mangle]
    static mut ind: std::os::raw::c_int;
    #[no_mangle]
    static mut nocode_wanted: std::os::raw::c_int;
    #[no_mangle]
    fn tcc_debug_start(s1: *mut TCCState);
    #[no_mangle]
    fn tcc_debug_end(s1: *mut TCCState);
    #[no_mangle]
    fn test_lvalue();
    #[no_mangle]
    fn elfsym(_: *mut Sym) -> *mut Elf64_Sym;
    #[no_mangle]
    fn update_storage(sym: *mut Sym);
    #[no_mangle]
    fn put_extern_sym2(
        sym: *mut Sym,
        sh_num: std::os::raw::c_int,
        value: Elf64_Addr,
        size: std::os::raw::c_ulong,
        can_add_underscore: std::os::raw::c_int,
    );
    #[no_mangle]
    fn sym_find(v: std::os::raw::c_int) -> *mut Sym;
    #[no_mangle]
    fn global_identifier_push(
        v: std::os::raw::c_int,
        t: std::os::raw::c_int,
        c: std::os::raw::c_int,
    ) -> *mut Sym;
    #[no_mangle]
    fn vpop();
    #[no_mangle]
    fn save_regs(n: std::os::raw::c_int);
    #[no_mangle]
    fn gv(rc: std::os::raw::c_int) -> std::os::raw::c_int;
    #[no_mangle]
    fn parse_mult_str(astr: *mut CString, msg: *const std::os::raw::c_char);
    #[no_mangle]
    fn parse_asm_str(astr: *mut CString);
    #[no_mangle]
    fn gexpr();
    #[no_mangle]
    fn section_ptr_add(sec: *mut Section, size: Elf64_Addr) -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn find_section(s1: *mut TCCState, name: *const std::os::raw::c_char) -> *mut Section;
    #[no_mangle]
    fn g(c: std::os::raw::c_int);
    #[no_mangle]
    fn gen_le16(c: std::os::raw::c_int);
    #[no_mangle]
    fn asm_gen_code(
        operands: *mut ASMOperand,
        nb_operands: std::os::raw::c_int,
        nb_outputs: std::os::raw::c_int,
        is_output: std::os::raw::c_int,
        clobber_regs: *mut uint8_t,
        out_reg: std::os::raw::c_int,
    );
    #[no_mangle]
    fn subst_asm_operand(add_str: *mut CString, sv: *mut SValue, modifier: std::os::raw::c_int);
    #[no_mangle]
    fn asm_opcode(s1: *mut TCCState, opcode: std::os::raw::c_int);
    #[no_mangle]
    fn gen_expr32(pe: *mut ExprValue);
    #[no_mangle]
    fn gen_expr64(pe: *mut ExprValue);
}
pub type size_t = std::os::raw::c_ulong;
pub type __uint8_t = std::os::raw::c_uchar;
pub type __uint16_t = std::os::raw::c_ushort;
pub type __uint32_t = std::os::raw::c_uint;
pub type __int64_t = std::os::raw::c_long;
pub type __uint64_t = std::os::raw::c_ulong;
pub type __off_t = std::os::raw::c_long;
pub type __off64_t = std::os::raw::c_long;
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
    pub str_0: C2RustUnnamed_4,
    pub tab: [std::os::raw::c_int; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub data: *const std::os::raw::c_void,
    pub size: std::os::raw::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SValue {
    pub type_0: CType,
    pub r: std::os::raw::c_ushort,
    pub r2: std::os::raw::c_ushort,
    pub c2rust_unnamed: C2RustUnnamed_7,
    pub c2rust_unnamed_0: C2RustUnnamed_5,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub c2rust_unnamed: C2RustUnnamed_6,
    pub sym: *mut Sym,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub cmp_op: std::os::raw::c_ushort,
    pub cmp_r: std::os::raw::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
    pub c2rust_unnamed: C2RustUnnamed_8,
    pub c: CValue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
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
#[inline]
unsafe extern "C" fn isnum(mut c: std::os::raw::c_int) -> std::os::raw::c_int {
    return (c >= '0' as i32 && c <= '9' as i32) as std::os::raw::c_int;
}
/*
 *  GAS like assembler for TCC
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
static mut last_text_section: *mut Section = 0 as *const Section as *mut Section;
/* to handle .previous asm directive */
#[no_mangle]
pub unsafe extern "C" fn asm_get_local_label_name(
    mut s1: *mut TCCState,
    mut n: std::os::raw::c_uint,
) -> std::os::raw::c_int {
    let mut buf: [std::os::raw::c_char; 64] = [0; 64];
    snprintf(
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[std::os::raw::c_char; 64]>() as std::os::raw::c_ulong,
        b"L..%u\x00" as *const u8 as *const std::os::raw::c_char,
        n,
    );
    return tok_alloc_const(buf.as_mut_ptr());
}
/* If a C name has an _ prepended then only asm labels that start
with _ are representable in C, by removing the first _.  ASM names
without _ at the beginning don't correspond to C names, but we use
the global C symbol table to track ASM names as well, so we need to
transform those into ones that don't conflict with a C name,
so prepend a '.' for them, but force the ELF asm name to be set.  */
unsafe extern "C" fn asm2cname(
    mut v: std::os::raw::c_int,
    mut addeddot: *mut std::os::raw::c_int,
) -> std::os::raw::c_int {
    let mut name: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    *addeddot = 0 as std::os::raw::c_int;
    if (*tcc_state).leading_underscore == 0 {
        return v;
    }
    name = get_tok_str(v, 0 as *mut CValue);
    if name.is_null() {
        return v;
    }
    if *name.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int == '_' as i32 {
        v = tok_alloc_const(name.offset(1 as std::os::raw::c_int as isize))
    } else if strchr(name, '.' as i32).is_null() {
        let mut newname: [std::os::raw::c_char; 256] = [0; 256];
        snprintf(
            newname.as_mut_ptr(),
            ::std::mem::size_of::<[std::os::raw::c_char; 256]>() as std::os::raw::c_ulong,
            b".%s\x00" as *const u8 as *const std::os::raw::c_char,
            name,
        );
        v = tok_alloc_const(newname.as_mut_ptr());
        *addeddot = 1 as std::os::raw::c_int
    }
    return v;
}
unsafe extern "C" fn asm_label_find(mut v: std::os::raw::c_int) -> *mut Sym {
    let mut sym: *mut Sym = 0 as *mut Sym;
    let mut addeddot: std::os::raw::c_int = 0;
    v = asm2cname(v, &mut addeddot);
    sym = sym_find(v);
    while !sym.is_null()
        && (*sym)
            .c2rust_unnamed
            .c2rust_unnamed
            .c2rust_unnamed
            .sym_scope
            != 0
        && (*sym).type_0.t & 0x2000 as std::os::raw::c_int == 0
    {
        sym = (*sym).prev_tok
    }
    return sym;
}
unsafe extern "C" fn asm_label_push(mut v: std::os::raw::c_int) -> *mut Sym {
    let mut addeddot: std::os::raw::c_int = 0;
    let mut v2: std::os::raw::c_int = asm2cname(v, &mut addeddot);
    /* We always add VT_EXTERN, for sym definition that's tentative
    (for .set, removed for real defs), for mere references it's correct
    as is.  */
    let mut sym: *mut Sym = global_identifier_push(
        v2,
        0 as std::os::raw::c_int
            | (1 as std::os::raw::c_int) << 20 as std::os::raw::c_int
            | 0x1000 as std::os::raw::c_int
            | 0x2000 as std::os::raw::c_int,
        0 as std::os::raw::c_int,
    );
    if addeddot != 0 {
        (*sym).c2rust_unnamed_0.asm_label = v
    }
    return sym;
}
/* Return a symbol we can use inside the assembler, having name NAME.
Symbols from asm and C source share a namespace.  If we generate
an asm symbol it's also a (file-global) C symbol, but it's
either not accessible by name (like "L.123"), or its type information
is such that it's not usable without a proper C declaration.

Sometimes we need symbols accessible by name from asm, which
are anonymous in C, in this case CSYM can be used to transfer
all information from that symbol to the (possibly newly created)
asm symbol.  */
#[no_mangle]
pub unsafe extern "C" fn get_asm_sym(
    mut name: std::os::raw::c_int,
    mut csym: *mut Sym,
) -> *mut Sym {
    let mut sym: *mut Sym = asm_label_find(name);
    if sym.is_null() {
        sym = asm_label_push(name);
        if !csym.is_null() {
            (*sym).c2rust_unnamed.c2rust_unnamed.c = (*csym).c2rust_unnamed.c2rust_unnamed.c
        }
    }
    return sym;
}
unsafe extern "C" fn asm_section_sym(mut s1: *mut TCCState, mut sec: *mut Section) -> *mut Sym {
    let mut buf: [std::os::raw::c_char; 100] = [0; 100];
    let mut label: std::os::raw::c_int = 0;
    let mut sym: *mut Sym = 0 as *mut Sym;
    snprintf(
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[std::os::raw::c_char; 100]>() as std::os::raw::c_ulong,
        b"L.%s\x00" as *const u8 as *const std::os::raw::c_char,
        (*sec).name.as_mut_ptr(),
    );
    label = tok_alloc_const(buf.as_mut_ptr());
    sym = asm_label_find(label);
    return if !sym.is_null() {
        sym
    } else {
        asm_new_label1(
            s1,
            label,
            1 as std::os::raw::c_int,
            (*sec).sh_num,
            0 as std::os::raw::c_int,
        )
    };
}
/* We do not use the C expression parser to handle symbols. Maybe the
C expression parser could be tweaked to do so. */
unsafe extern "C" fn asm_expr_unary(mut s1: *mut TCCState, mut pe: *mut ExprValue) {
    let mut sym: *mut Sym = 0 as *mut Sym;
    let mut op: std::os::raw::c_int = 0;
    let mut label: std::os::raw::c_int = 0;
    let mut n: uint64_t = 0;
    let mut p: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    match tok {
        205 => {
            p = tokc.str_0.data as *const std::os::raw::c_char;
            n = strtoull(
                p,
                &mut p as *mut *const std::os::raw::c_char as *mut *mut std::os::raw::c_char,
                0 as std::os::raw::c_int,
            ) as uint64_t;
            if *p as std::os::raw::c_int == 'b' as i32 || *p as std::os::raw::c_int == 'f' as i32 {
                /* backward or forward label */
                label = asm_get_local_label_name(s1, n as std::os::raw::c_uint);
                sym = asm_label_find(label);
                if *p as std::os::raw::c_int == 'b' as i32 {
                    /* backward : find the last corresponding defined label */
                    if !sym.is_null()
                        && ((*sym).c2rust_unnamed.c2rust_unnamed.c == 0
                            || (*elfsym(sym)).st_shndx as std::os::raw::c_int
                                == 0 as std::os::raw::c_int)
                    {
                        sym = (*sym).prev_tok
                    }
                    if sym.is_null() {
                        _tcc_error(
                            b"local label \'%d\' not found backward\x00" as *const u8
                                as *const std::os::raw::c_char,
                            n as std::os::raw::c_int,
                        );
                    }
                } else if sym.is_null()
                    || (*sym).c2rust_unnamed.c2rust_unnamed.c != 0
                        && (*elfsym(sym)).st_shndx as std::os::raw::c_int
                            != 0 as std::os::raw::c_int
                {
                    /* forward */
                    /* if the last label is defined, then define a new one */
                    sym = asm_label_push(label)
                }
                (*pe).v = 0 as std::os::raw::c_int as uint64_t;
                (*pe).sym = sym;
                (*pe).pcrel = 0 as std::os::raw::c_int
            } else if *p as std::os::raw::c_int == '\u{0}' as i32 {
                (*pe).v = n;
                (*pe).sym = 0 as *mut Sym;
                (*pe).pcrel = 0 as std::os::raw::c_int
            } else {
                _tcc_error(
                    b"invalid number syntax\x00" as *const u8 as *const std::os::raw::c_char,
                );
            }
            next();
        },
        43 => {
            next();
            asm_expr_unary(s1, pe);
        },
        45 | 126 => {
            op = tok;
            next();
            asm_expr_unary(s1, pe);
            if !(*pe).sym.is_null() {
                _tcc_error(
                    b"invalid operation with label\x00" as *const u8 as *const std::os::raw::c_char,
                );
            }
            if op == '-' as i32 {
                (*pe).v = (*pe).v.wrapping_neg()
            } else {
                (*pe).v = !(*pe).v
            }
        },
        192 | 193 => {
            (*pe).v = tokc.i;
            (*pe).sym = 0 as *mut Sym;
            (*pe).pcrel = 0 as std::os::raw::c_int;
            next();
        },
        40 => {
            next();
            asm_expr(s1, pe);
            skip(')' as i32);
        },
        46 => {
            (*pe).v = ind as uint64_t;
            (*pe).sym = asm_section_sym(s1, (*tcc_state).cur_text_section);
            (*pe).pcrel = 0 as std::os::raw::c_int;
            next();
        },
        _ => {
            if tok >= 256 as std::os::raw::c_int {
                let mut esym: *mut Elf64_Sym = 0 as *mut Elf64_Sym;
                /* label case : if the label was not found, add one */
                sym = get_asm_sym(tok, 0 as *mut Sym);
                esym = elfsym(sym);
                if !esym.is_null()
                    && (*esym).st_shndx as std::os::raw::c_int == 0xfff1 as std::os::raw::c_int
                {
                    /* if absolute symbol, no need to put a symbol value */
                    (*pe).v = (*esym).st_value;
                    (*pe).sym = 0 as *mut Sym;
                    (*pe).pcrel = 0 as std::os::raw::c_int
                } else {
                    (*pe).v = 0 as std::os::raw::c_int as uint64_t;
                    (*pe).sym = sym;
                    (*pe).pcrel = 0 as std::os::raw::c_int
                }
                next();
            } else {
                _tcc_error(
                    b"bad expression syntax [%s]\x00" as *const u8 as *const std::os::raw::c_char,
                    get_tok_str(tok, &mut tokc),
                );
            }
        },
    };
}
unsafe extern "C" fn asm_expr_prod(mut s1: *mut TCCState, mut pe: *mut ExprValue) {
    let mut op: std::os::raw::c_int = 0;
    let mut e2: ExprValue = ExprValue {
        v: 0,
        sym: 0 as *mut Sym,
        pcrel: 0,
    };
    asm_expr_unary(s1, pe);
    loop {
        op = tok;
        if op != '*' as i32
            && op != '/' as i32
            && op != '%' as i32
            && op != '<' as i32
            && op != '>' as i32
        {
            break;
        }
        next();
        asm_expr_unary(s1, &mut e2);
        if !(*pe).sym.is_null() || !e2.sym.is_null() {
            _tcc_error(
                b"invalid operation with label\x00" as *const u8 as *const std::os::raw::c_char,
            );
        }
        let mut current_block_11: u64;
        match op {
            42 => {
                (*pe).v =
                    ((*pe).v as std::os::raw::c_ulong).wrapping_mul(e2.v) as uint64_t as uint64_t;
                current_block_11 = 4495394744059808450;
            },
            47 => {
                if e2.v == 0 as std::os::raw::c_int as std::os::raw::c_ulong {
                    current_block_11 = 15317039509691223228;
                } else {
                    (*pe).v = ((*pe).v as std::os::raw::c_ulong).wrapping_div(e2.v) as uint64_t
                        as uint64_t;
                    current_block_11 = 4495394744059808450;
                }
            },
            37 => {
                if e2.v == 0 as std::os::raw::c_int as std::os::raw::c_ulong {
                    current_block_11 = 15317039509691223228;
                } else {
                    (*pe).v = ((*pe).v as std::os::raw::c_ulong).wrapping_rem(e2.v) as uint64_t
                        as uint64_t;
                    current_block_11 = 4495394744059808450;
                }
            },
            60 => {
                (*pe).v <<= e2.v;
                current_block_11 = 4495394744059808450;
            },
            62 | _ => {
                (*pe).v >>= e2.v;
                current_block_11 = 4495394744059808450;
            },
        }
        match current_block_11 {
            4495394744059808450 => {},
            _ => {
                _tcc_error(b"division by zero\x00" as *const u8 as *const std::os::raw::c_char);
            },
        }
    }
}
unsafe extern "C" fn asm_expr_logic(mut s1: *mut TCCState, mut pe: *mut ExprValue) {
    let mut op: std::os::raw::c_int = 0;
    let mut e2: ExprValue = ExprValue {
        v: 0,
        sym: 0 as *mut Sym,
        pcrel: 0,
    };
    asm_expr_prod(s1, pe);
    loop {
        op = tok;
        if op != '&' as i32 && op != '|' as i32 && op != '^' as i32 {
            break;
        }
        next();
        asm_expr_prod(s1, &mut e2);
        if !(*pe).sym.is_null() || !e2.sym.is_null() {
            _tcc_error(
                b"invalid operation with label\x00" as *const u8 as *const std::os::raw::c_char,
            );
        }
        match op {
            38 => (*pe).v &= e2.v,
            124 => (*pe).v |= e2.v,
            94 | _ => (*pe).v ^= e2.v,
        }
    }
}
#[inline]
unsafe extern "C" fn asm_expr_sum(mut s1: *mut TCCState, mut pe: *mut ExprValue) {
    let mut op: std::os::raw::c_int = 0;
    let mut e2: ExprValue = ExprValue {
        v: 0,
        sym: 0 as *mut Sym,
        pcrel: 0,
    };
    asm_expr_logic(s1, pe);
    loop {
        op = tok;
        if op != '+' as i32 && op != '-' as i32 {
            break;
        }
        next();
        asm_expr_logic(s1, &mut e2);
        let mut esym1: *mut Elf64_Sym = 0 as *mut Elf64_Sym;
        let mut esym2: *mut Elf64_Sym = 0 as *mut Elf64_Sym;
        let mut current_block_20: u64;
        if op == '+' as i32 {
            if !(*pe).sym.is_null() && !e2.sym.is_null() {
                current_block_20 = 5950501356679075927;
            } else {
                (*pe).v =
                    ((*pe).v as std::os::raw::c_ulong).wrapping_add(e2.v) as uint64_t as uint64_t;
                if (*pe).sym.is_null() && !e2.sym.is_null() {
                    (*pe).sym = e2.sym
                }
                current_block_20 = 7205609094909031804;
            }
        } else {
            (*pe).v = ((*pe).v as std::os::raw::c_ulong).wrapping_sub(e2.v) as uint64_t as uint64_t;
            /* NOTE: we are less powerful than gas in that case
            because we store only one symbol in the expression */
            if e2.sym.is_null() {
                current_block_20 = 7205609094909031804;
            } else if (*pe).sym == e2.sym {
                /* OK */
                (*pe).sym = 0 as *mut Sym;
                current_block_20 = 7205609094909031804;
            /* same symbols can be subtracted to NULL */
            } else {
                esym1 = 0 as *mut Elf64_Sym;
                esym2 = 0 as *mut Elf64_Sym;
                esym1 = elfsym((*pe).sym);
                esym2 = elfsym(e2.sym);
                if !esym1.is_null()
                    && (*esym1).st_shndx as std::os::raw::c_int
                        == (*esym2).st_shndx as std::os::raw::c_int
                    && (*esym1).st_shndx as std::os::raw::c_int != 0 as std::os::raw::c_int
                {
                    /* we also accept defined symbols in the same section */
                    (*pe).v = ((*pe).v as std::os::raw::c_ulong)
                        .wrapping_add((*esym1).st_value.wrapping_sub((*esym2).st_value))
                        as uint64_t as uint64_t;
                    (*pe).sym = 0 as *mut Sym;
                    current_block_20 = 7205609094909031804;
                } else if (*esym2).st_shndx as std::os::raw::c_int
                    == (*(*tcc_state).cur_text_section).sh_num
                {
                    /* When subtracting a defined symbol in current section
                    this actually makes the value PC-relative.  */
                    (*pe).v = ((*pe).v as std::os::raw::c_ulong).wrapping_sub(
                        (*esym2)
                            .st_value
                            .wrapping_sub(ind as std::os::raw::c_ulong)
                            .wrapping_sub(4 as std::os::raw::c_int as std::os::raw::c_ulong),
                    ) as uint64_t as uint64_t;
                    (*pe).pcrel = 1 as std::os::raw::c_int;
                    e2.sym = 0 as *mut Sym;
                    current_block_20 = 7205609094909031804;
                } else {
                    current_block_20 = 5950501356679075927;
                }
            }
        }
        match current_block_20 {
            5950501356679075927 => {
                _tcc_error(
                    b"invalid operation with label\x00" as *const u8 as *const std::os::raw::c_char,
                );
            },
            _ =>
                /* OK */
                {}
        }
    }
}
#[inline]
unsafe extern "C" fn asm_expr_cmp(mut s1: *mut TCCState, mut pe: *mut ExprValue) {
    let mut op: std::os::raw::c_int = 0;
    let mut e2: ExprValue = ExprValue {
        v: 0,
        sym: 0 as *mut Sym,
        pcrel: 0,
    };
    asm_expr_sum(s1, pe);
    loop {
        op = tok;
        if op != 0x94 as std::os::raw::c_int
            && op != 0x95 as std::os::raw::c_int
            && (op > 0x9f as std::os::raw::c_int || op < 0x96 as std::os::raw::c_int)
        {
            break;
        }
        next();
        asm_expr_sum(s1, &mut e2);
        if !(*pe).sym.is_null() || !e2.sym.is_null() {
            _tcc_error(
                b"invalid operation with label\x00" as *const u8 as *const std::os::raw::c_char,
            );
        }
        match op {
            148 => (*pe).v = ((*pe).v == e2.v) as std::os::raw::c_int as uint64_t,
            149 => (*pe).v = ((*pe).v != e2.v) as std::os::raw::c_int as uint64_t,
            156 => {
                (*pe).v =
                    (((*pe).v as int64_t) < e2.v as int64_t) as std::os::raw::c_int as uint64_t
            },
            157 => {
                (*pe).v = ((*pe).v as int64_t >= e2.v as int64_t) as std::os::raw::c_int as uint64_t
            },
            158 => {
                (*pe).v = ((*pe).v as int64_t <= e2.v as int64_t) as std::os::raw::c_int as uint64_t
            },
            159 => {
                (*pe).v = ((*pe).v as int64_t > e2.v as int64_t) as std::os::raw::c_int as uint64_t
            },
            _ => {},
        }
        /* GAS compare results are -1/0 not 1/0.  */
        (*pe).v = -((*pe).v as int64_t) as uint64_t
    }
}
#[no_mangle]
pub unsafe extern "C" fn asm_expr(mut s1: *mut TCCState, mut pe: *mut ExprValue) {
    asm_expr_cmp(s1, pe);
}
#[no_mangle]
pub unsafe extern "C" fn asm_int_expr(mut s1: *mut TCCState) -> std::os::raw::c_int {
    let mut e: ExprValue = ExprValue {
        v: 0,
        sym: 0 as *mut Sym,
        pcrel: 0,
    };
    asm_expr(s1, &mut e);
    if !e.sym.is_null() {
        expect(b"constant\x00" as *const u8 as *const std::os::raw::c_char);
    }
    return e.v as std::os::raw::c_int;
}
unsafe extern "C" fn asm_new_label1(
    mut s1: *mut TCCState,
    mut label: std::os::raw::c_int,
    mut is_local: std::os::raw::c_int,
    mut sh_num: std::os::raw::c_int,
    mut value: std::os::raw::c_int,
) -> *mut Sym {
    let mut sym: *mut Sym = 0 as *mut Sym;
    let mut esym: *mut Elf64_Sym = 0 as *mut Elf64_Sym;
    sym = asm_label_find(label);
    let mut current_block_4: u64;
    if !sym.is_null() {
        esym = elfsym(sym);
        /* A VT_EXTERN symbol, even if it has a section is considered
        overridable.  This is how we "define" .set targets.  Real
        definitions won't have VT_EXTERN set.  */
        if !esym.is_null() && (*esym).st_shndx as std::os::raw::c_int != 0 as std::os::raw::c_int {
            /* the label is already defined */
            if (*sym).type_0.t
                & (0xf as std::os::raw::c_int
                    | (0 as std::os::raw::c_int
                        | (1 as std::os::raw::c_int) << 20 as std::os::raw::c_int))
                == 0 as std::os::raw::c_int
                    | (1 as std::os::raw::c_int) << 20 as std::os::raw::c_int
                && (is_local == 1 as std::os::raw::c_int
                    || (*sym).type_0.t & 0x1000 as std::os::raw::c_int != 0)
            {
                current_block_4 = 3640402440696253106;
            } else {
                if (*sym).type_0.t & 0x1000 as std::os::raw::c_int == 0 {
                    _tcc_error(
                        b"assembler label \'%s\' already defined\x00" as *const u8
                            as *const std::os::raw::c_char,
                        get_tok_str(label, 0 as *mut CValue),
                    );
                }
                current_block_4 = 13109137661213826276;
            }
        } else {
            current_block_4 = 13109137661213826276;
        }
    } else {
        current_block_4 = 3640402440696253106;
    }
    match current_block_4 {
        3640402440696253106 => sym = asm_label_push(label),
        _ => {},
    }
    if (*sym).c2rust_unnamed.c2rust_unnamed.c == 0 {
        put_extern_sym2(
            sym,
            0 as std::os::raw::c_int,
            0 as std::os::raw::c_int as Elf64_Addr,
            0 as std::os::raw::c_int as std::os::raw::c_ulong,
            1 as std::os::raw::c_int,
        );
    }
    esym = elfsym(sym);
    (*esym).st_shndx = sh_num as Elf64_Section;
    (*esym).st_value = value as Elf64_Addr;
    if is_local != 2 as std::os::raw::c_int {
        (*sym).type_0.t &= !(0x1000 as std::os::raw::c_int)
    }
    return sym;
}
unsafe extern "C" fn asm_new_label(
    mut s1: *mut TCCState,
    mut label: std::os::raw::c_int,
    mut is_local: std::os::raw::c_int,
) -> *mut Sym {
    return asm_new_label1(
        s1,
        label,
        is_local,
        (*(*tcc_state).cur_text_section).sh_num,
        ind,
    );
}
/* Set the value of LABEL to that of some expression (possibly
involving other symbols).  LABEL can be overwritten later still.  */
unsafe extern "C" fn set_symbol(mut s1: *mut TCCState, mut label: std::os::raw::c_int) -> *mut Sym {
    let mut n: std::os::raw::c_long = 0;
    let mut e: ExprValue = ExprValue {
        v: 0,
        sym: 0 as *mut Sym,
        pcrel: 0,
    };
    let mut sym: *mut Sym = 0 as *mut Sym;
    let mut esym: *mut Elf64_Sym = 0 as *mut Elf64_Sym;
    next();
    asm_expr(s1, &mut e);
    n = e.v as std::os::raw::c_long;
    esym = elfsym(e.sym);
    if !esym.is_null() {
        n = (n as std::os::raw::c_ulong).wrapping_add((*esym).st_value) as std::os::raw::c_long
            as std::os::raw::c_long
    }
    sym = asm_new_label1(
        s1,
        label,
        2 as std::os::raw::c_int,
        if !esym.is_null() {
            (*esym).st_shndx as std::os::raw::c_int
        } else {
            0xfff1 as std::os::raw::c_int
        },
        n as std::os::raw::c_int,
    );
    let ref mut fresh0 = (*elfsym(sym)).st_other;
    *fresh0 =
        (*fresh0 as std::os::raw::c_int | 0x4 as std::os::raw::c_int) as std::os::raw::c_uchar;
    return sym;
}
unsafe extern "C" fn use_section1(mut s1: *mut TCCState, mut sec: *mut Section) {
    (*(*tcc_state).cur_text_section).data_offset = ind as std::os::raw::c_ulong;
    (*tcc_state).cur_text_section = sec;
    ind = (*(*tcc_state).cur_text_section).data_offset as std::os::raw::c_int;
}
unsafe extern "C" fn use_section(mut s1: *mut TCCState, mut name: *const std::os::raw::c_char) {
    let mut sec: *mut Section = 0 as *mut Section;
    sec = find_section(s1, name);
    use_section1(s1, sec);
}
unsafe extern "C" fn push_section(mut s1: *mut TCCState, mut name: *const std::os::raw::c_char) {
    let mut sec: *mut Section = find_section(s1, name);
    (*sec).prev = (*tcc_state).cur_text_section;
    use_section1(s1, sec);
}
unsafe extern "C" fn pop_section(mut s1: *mut TCCState) {
    let mut prev: *mut Section = (*(*tcc_state).cur_text_section).prev;
    if prev.is_null() {
        _tcc_error(
            b".popsection without .pushsection\x00" as *const u8 as *const std::os::raw::c_char,
        );
    }
    (*(*tcc_state).cur_text_section).prev = 0 as *mut Section;
    use_section1(s1, prev);
}
unsafe extern "C" fn asm_parse_directive(mut s1: *mut TCCState, mut global: std::os::raw::c_int) {
    let mut current_block: u64;
    let mut n: std::os::raw::c_int = 0;
    let mut offset: std::os::raw::c_int = 0;
    let mut v: std::os::raw::c_int = 0;
    let mut size: std::os::raw::c_int = 0;
    let mut tok1: std::os::raw::c_int = 0;
    let mut sec: *mut Section = 0 as *mut Section;
    let mut ptr: *mut uint8_t = 0 as *mut uint8_t;
    /* assembler directive */
    sec = (*tcc_state).cur_text_section;
    match tok {
        487 | 488 | 489 | 491 | 492 => {
            tok1 = tok;
            next();
            n = asm_int_expr(s1);
            if tok1 == TOK_ASMDIR_p2align as std::os::raw::c_int {
                if n < 0 as std::os::raw::c_int || n > 30 as std::os::raw::c_int {
                    _tcc_error(
                        b"invalid p2align, must be between 0 and 30\x00" as *const u8
                            as *const std::os::raw::c_char,
                    );
                }
                n = (1 as std::os::raw::c_int) << n;
                tok1 = TOK_ASMDIR_align as std::os::raw::c_int
            }
            if tok1 == TOK_ASMDIR_align as std::os::raw::c_int
                || tok1 == TOK_ASMDIR_balign as std::os::raw::c_int
            {
                if n < 0 as std::os::raw::c_int
                    || n & n - 1 as std::os::raw::c_int != 0 as std::os::raw::c_int
                {
                    _tcc_error(
                        b"alignment must be a positive power of two\x00" as *const u8
                            as *const std::os::raw::c_char,
                    );
                }
                offset = ind + n - 1 as std::os::raw::c_int & -n;
                size = offset - ind;
                /* the section must have a compatible alignment */
                if (*sec).sh_addralign < n {
                    (*sec).sh_addralign = n
                }
            } else {
                if n < 0 as std::os::raw::c_int {
                    n = 0 as std::os::raw::c_int
                }
                size = n
            }
            v = 0 as std::os::raw::c_int;
            if tok == ',' as i32 {
                next();
                v = asm_int_expr(s1)
            }
            current_block = 15295264098288563722;
        },
        514 => {
            size = 8 as std::os::raw::c_int;
            current_block = 17625080126422795864;
        },
        485 => {
            size = 1 as std::os::raw::c_int;
            current_block = 17625080126422795864;
        },
        486 | 516 => {
            size = 2 as std::os::raw::c_int;
            current_block = 17625080126422795864;
        },
        517 | 518 => {
            size = 4 as std::os::raw::c_int;
            current_block = 17625080126422795864;
        },
        510 => {
            let mut repeat: std::os::raw::c_int = 0;
            let mut size_0: std::os::raw::c_int = 0;
            let mut val: std::os::raw::c_int = 0;
            let mut i: std::os::raw::c_int = 0;
            let mut j: std::os::raw::c_int = 0;
            let mut repeat_buf: [uint8_t; 8] = [0; 8];
            next();
            repeat = asm_int_expr(s1);
            if repeat < 0 as std::os::raw::c_int {
                _tcc_error(
                    b"repeat < 0; .fill ignored\x00" as *const u8 as *const std::os::raw::c_char,
                );
            } else {
                size_0 = 1 as std::os::raw::c_int;
                val = 0 as std::os::raw::c_int;
                if tok == ',' as i32 {
                    next();
                    size_0 = asm_int_expr(s1);
                    if size_0 < 0 as std::os::raw::c_int {
                        _tcc_error(
                            b"size < 0; .fill ignored\x00" as *const u8
                                as *const std::os::raw::c_char,
                        );
                    } else {
                        if size_0 > 8 as std::os::raw::c_int {
                            size_0 = 8 as std::os::raw::c_int
                        }
                        if tok == ',' as i32 {
                            next();
                            val = asm_int_expr(s1)
                        }
                    }
                }
                /* XXX: endianness */
                repeat_buf[0 as std::os::raw::c_int as usize] = val as uint8_t;
                repeat_buf[1 as std::os::raw::c_int as usize] =
                    (val >> 8 as std::os::raw::c_int) as uint8_t;
                repeat_buf[2 as std::os::raw::c_int as usize] =
                    (val >> 16 as std::os::raw::c_int) as uint8_t;
                repeat_buf[3 as std::os::raw::c_int as usize] =
                    (val >> 24 as std::os::raw::c_int) as uint8_t;
                repeat_buf[4 as std::os::raw::c_int as usize] = 0 as std::os::raw::c_int as uint8_t;
                repeat_buf[5 as std::os::raw::c_int as usize] = 0 as std::os::raw::c_int as uint8_t;
                repeat_buf[6 as std::os::raw::c_int as usize] = 0 as std::os::raw::c_int as uint8_t;
                repeat_buf[7 as std::os::raw::c_int as usize] = 0 as std::os::raw::c_int as uint8_t;
                i = 0 as std::os::raw::c_int;
                while i < repeat {
                    j = 0 as std::os::raw::c_int;
                    while j < size_0 {
                        g(repeat_buf[j as usize] as std::os::raw::c_int);
                        j += 1
                    }
                    i += 1
                }
            }
            current_block = 573237344297737569;
        },
        511 => {
            let mut repeat_0: std::os::raw::c_int = 0;
            let mut init_str: *mut TokenString = 0 as *mut TokenString;
            next();
            repeat_0 = asm_int_expr(s1);
            init_str = tok_str_alloc();
            loop {
                next();
                if !(tok != TOK_ASMDIR_endr as std::os::raw::c_int) {
                    break;
                }
                if tok == -(1 as std::os::raw::c_int) {
                    _tcc_error(
                        b"we at end of file, .endr not found\x00" as *const u8
                            as *const std::os::raw::c_char,
                    );
                }
                tok_str_add_tok(init_str);
            }
            tok_str_add(init_str, -(1 as std::os::raw::c_int));
            tok_str_add(init_str, 0 as std::os::raw::c_int);
            begin_macro(init_str, 1 as std::os::raw::c_int);
            loop {
                let fresh1 = repeat_0;
                repeat_0 = repeat_0 - 1;
                if !(fresh1 > 0 as std::os::raw::c_int) {
                    break;
                }
                tcc_assemble_internal(s1, parse_flags & 0x1 as std::os::raw::c_int, global);
                macro_ptr = (*init_str).str_0
            }
            end_macro();
            next();
            current_block = 573237344297737569;
        },
        513 => {
            let mut n_0: std::os::raw::c_ulong = 0;
            let mut e_0: ExprValue = ExprValue {
                v: 0,
                sym: 0 as *mut Sym,
                pcrel: 0,
            };
            let mut esym: *mut Elf64_Sym = 0 as *mut Elf64_Sym;
            next();
            asm_expr(s1, &mut e_0);
            n_0 = e_0.v;
            esym = elfsym(e_0.sym);
            if !esym.is_null() {
                if (*esym).st_shndx as std::os::raw::c_int
                    != (*(*tcc_state).cur_text_section).sh_num
                {
                    expect(
                        b"constant or same-section symbol\x00" as *const u8
                            as *const std::os::raw::c_char,
                    );
                }
                n_0 = n_0.wrapping_add((*esym).st_value)
            }
            if n_0 < ind as std::os::raw::c_ulong {
                _tcc_error(
                    b"attempt to .org backwards\x00" as *const u8 as *const std::os::raw::c_char,
                );
            }
            v = 0 as std::os::raw::c_int;
            size = n_0.wrapping_sub(ind as std::os::raw::c_ulong) as std::os::raw::c_int;
            current_block = 15295264098288563722;
        },
        490 => {
            next();
            tok1 = tok;
            next();
            /* Also accept '.set stuff', but don't do anything with this.
            It's used in GAS to set various features like '.set mips16'.  */
            if tok == ',' as i32 {
                set_symbol(s1, tok1);
            }
            current_block = 573237344297737569;
        },
        497 | 498 | 499 | 500 => {
            tok1 = tok;
            loop {
                let mut sym: *mut Sym = 0 as *mut Sym;
                next();
                sym = get_asm_sym(tok, 0 as *mut Sym);
                if tok1 != TOK_ASMDIR_hidden as std::os::raw::c_int {
                    (*sym).type_0.t &= !(0x2000 as std::os::raw::c_int)
                }
                if tok1 == TOK_ASMDIR_weak as std::os::raw::c_int {
                    (*sym)
                        .a
                        .set_weak(1 as std::os::raw::c_int as std::os::raw::c_ushort)
                } else if tok1 == TOK_ASMDIR_hidden as std::os::raw::c_int {
                    (*sym)
                        .a
                        .set_visibility(2 as std::os::raw::c_int as std::os::raw::c_ushort)
                }
                update_storage(sym);
                next();
                if !(tok == ',' as i32) {
                    break;
                }
            }
            current_block = 573237344297737569;
        },
        493 | 495 | 494 => {
            let mut p: *const uint8_t = 0 as *const uint8_t;
            let mut i_0: std::os::raw::c_int = 0;
            let mut size_1: std::os::raw::c_int = 0;
            let mut t: std::os::raw::c_int = 0;
            t = tok;
            next();
            loop {
                if tok != 0xc8 as std::os::raw::c_int {
                    expect(b"string constant\x00" as *const u8 as *const std::os::raw::c_char);
                }
                p = tokc.str_0.data as *const uint8_t;
                size_1 = tokc.str_0.size;
                if t == TOK_ASMDIR_ascii as std::os::raw::c_int && size_1 > 0 as std::os::raw::c_int
                {
                    size_1 -= 1
                }
                i_0 = 0 as std::os::raw::c_int;
                while i_0 < size_1 {
                    g(*p.offset(i_0 as isize) as std::os::raw::c_int);
                    i_0 += 1
                }
                next();
                if tok == ',' as i32 {
                    next();
                } else if tok != 0xc8 as std::os::raw::c_int {
                    break;
                }
            }
            current_block = 573237344297737569;
        },
        504 | 505 | 506 => {
            let mut sname: [std::os::raw::c_char; 64] = [0; 64];
            tok1 = tok;
            n = 0 as std::os::raw::c_int;
            next();
            if tok != ';' as i32 && tok != 10 as std::os::raw::c_int {
                n = asm_int_expr(s1);
                next();
            }
            if n != 0 {
                sprintf(
                    sname.as_mut_ptr(),
                    b"%s%d\x00" as *const u8 as *const std::os::raw::c_char,
                    get_tok_str(tok1, 0 as *mut CValue),
                    n,
                );
            } else {
                sprintf(
                    sname.as_mut_ptr(),
                    b"%s\x00" as *const u8 as *const std::os::raw::c_char,
                    get_tok_str(tok1, 0 as *mut CValue),
                );
            }
            use_section(s1, sname.as_mut_ptr());
            current_block = 573237344297737569;
        },
        496 => {
            let mut filename: [std::os::raw::c_char; 512] = [0; 512];
            filename[0 as std::os::raw::c_int as usize] = '\u{0}' as i32 as std::os::raw::c_char;
            next();
            if tok == 0xc8 as std::os::raw::c_int {
                pstrcat(
                    filename.as_mut_ptr(),
                    ::std::mem::size_of::<[std::os::raw::c_char; 512]>() as std::os::raw::c_ulong,
                    tokc.str_0.data as *const std::os::raw::c_char,
                );
            } else {
                pstrcat(
                    filename.as_mut_ptr(),
                    ::std::mem::size_of::<[std::os::raw::c_char; 512]>() as std::os::raw::c_ulong,
                    get_tok_str(tok, 0 as *mut CValue),
                );
            }
            if (*s1).warn_unsupported != 0 {
                _tcc_warning(
                    b"ignoring .file %s\x00" as *const u8 as *const std::os::raw::c_char,
                    filename.as_mut_ptr(),
                );
            }
            next();
            current_block = 573237344297737569;
        },
        501 => {
            let mut ident: [std::os::raw::c_char; 256] = [0; 256];
            ident[0 as std::os::raw::c_int as usize] = '\u{0}' as i32 as std::os::raw::c_char;
            next();
            if tok == 0xc8 as std::os::raw::c_int {
                pstrcat(
                    ident.as_mut_ptr(),
                    ::std::mem::size_of::<[std::os::raw::c_char; 256]>() as std::os::raw::c_ulong,
                    tokc.str_0.data as *const std::os::raw::c_char,
                );
            } else {
                pstrcat(
                    ident.as_mut_ptr(),
                    ::std::mem::size_of::<[std::os::raw::c_char; 256]>() as std::os::raw::c_ulong,
                    get_tok_str(tok, 0 as *mut CValue),
                );
            }
            if (*s1).warn_unsupported != 0 {
                _tcc_warning(
                    b"ignoring .ident %s\x00" as *const u8 as *const std::os::raw::c_char,
                    ident.as_mut_ptr(),
                );
            }
            next();
            current_block = 573237344297737569;
        },
        502 => {
            let mut sym_0: *mut Sym = 0 as *mut Sym;
            next();
            sym_0 = asm_label_find(tok);
            if sym_0.is_null() {
                _tcc_error(
                    b"label not found: %s\x00" as *const u8 as *const std::os::raw::c_char,
                    get_tok_str(tok, 0 as *mut CValue),
                );
            }
            /* XXX .size name,label2-label1 */
            if (*s1).warn_unsupported != 0 {
                _tcc_warning(
                    b"ignoring .size %s,*\x00" as *const u8 as *const std::os::raw::c_char,
                    get_tok_str(tok, 0 as *mut CValue),
                );
            }
            next();
            skip(',' as i32);
            while tok != 10 as std::os::raw::c_int
                && tok != ';' as i32
                && tok != -(1 as std::os::raw::c_int)
            {
                next();
            }
            current_block = 573237344297737569;
        },
        503 => {
            let mut sym_1: *mut Sym = 0 as *mut Sym;
            let mut newtype: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
            next();
            sym_1 = get_asm_sym(tok, 0 as *mut Sym);
            next();
            skip(',' as i32);
            if tok == 0xc8 as std::os::raw::c_int {
                newtype = tokc.str_0.data as *const std::os::raw::c_char
            } else {
                if tok == '@' as i32 || tok == '%' as i32 {
                    next();
                }
                newtype = get_tok_str(tok, 0 as *mut CValue)
            }
            if strcmp(
                newtype,
                b"function\x00" as *const u8 as *const std::os::raw::c_char,
            ) == 0
                || strcmp(
                    newtype,
                    b"STT_FUNC\x00" as *const u8 as *const std::os::raw::c_char,
                ) == 0
            {
                (*sym_1).type_0.t =
                    (*sym_1).type_0.t & !(0xf as std::os::raw::c_int) | 6 as std::os::raw::c_int
            } else if (*s1).warn_unsupported != 0 {
                _tcc_warning(
                    b"change type of \'%s\' from 0x%x to \'%s\' ignored\x00" as *const u8
                        as *const std::os::raw::c_char,
                    get_tok_str((*sym_1).v, 0 as *mut CValue),
                    (*sym_1).type_0.t,
                    newtype,
                );
            }
            next();
            current_block = 573237344297737569;
        },
        508 | 519 => {
            let mut sname_0: [std::os::raw::c_char; 256] = [0; 256];
            let mut old_nb_section: std::os::raw::c_int = (*s1).nb_sections;
            tok1 = tok;
            /* XXX: support more options */
            next();
            sname_0[0 as std::os::raw::c_int as usize] = '\u{0}' as i32 as std::os::raw::c_char;
            while tok != ';' as i32 && tok != 10 as std::os::raw::c_int && tok != ',' as i32 {
                if tok == 0xc8 as std::os::raw::c_int {
                    pstrcat(
                        sname_0.as_mut_ptr(),
                        ::std::mem::size_of::<[std::os::raw::c_char; 256]>()
                            as std::os::raw::c_ulong,
                        tokc.str_0.data as *const std::os::raw::c_char,
                    );
                } else {
                    pstrcat(
                        sname_0.as_mut_ptr(),
                        ::std::mem::size_of::<[std::os::raw::c_char; 256]>()
                            as std::os::raw::c_ulong,
                        get_tok_str(tok, 0 as *mut CValue),
                    );
                }
                next();
            }
            if tok == ',' as i32 {
                /* skip section options */
                next();
                if tok != 0xc8 as std::os::raw::c_int {
                    expect(b"string constant\x00" as *const u8 as *const std::os::raw::c_char);
                }
                next();
                if tok == ',' as i32 {
                    next();
                    if tok == '@' as i32 || tok == '%' as i32 {
                        next();
                    }
                    next();
                }
            }
            last_text_section = (*tcc_state).cur_text_section;
            if tok1 == TOK_ASMDIR_section as std::os::raw::c_int {
                use_section(s1, sname_0.as_mut_ptr());
            } else {
                push_section(s1, sname_0.as_mut_ptr());
            }
            /* If we just allocated a new section reset its alignment to
            1.  new_section normally acts for GCC compatibility and
            sets alignment to PTR_SIZE.  The assembler behaves different. */
            if old_nb_section != (*s1).nb_sections {
                (*(*tcc_state).cur_text_section).sh_addralign = 1 as std::os::raw::c_int
            }
            current_block = 573237344297737569;
        },
        507 => {
            let mut sec_0: *mut Section = 0 as *mut Section;
            next();
            if last_text_section.is_null() {
                _tcc_error(
                    b"no previous section referenced\x00" as *const u8
                        as *const std::os::raw::c_char,
                );
            }
            sec_0 = (*tcc_state).cur_text_section;
            use_section1(s1, last_text_section);
            last_text_section = sec_0;
            current_block = 573237344297737569;
        },
        509 => {
            next();
            pop_section(s1);
            current_block = 573237344297737569;
        },
        515 => {
            /* added for compatibility with GAS */
            next();
            current_block = 573237344297737569;
        },
        _ => {
            _tcc_error(
                b"unknown assembler directive \'.%s\'\x00" as *const u8
                    as *const std::os::raw::c_char,
                get_tok_str(tok, 0 as *mut CValue),
            );
        },
    }
    match current_block {
        17625080126422795864 => {
            next();
            loop {
                let mut e: ExprValue = ExprValue {
                    v: 0,
                    sym: 0 as *mut Sym,
                    pcrel: 0,
                };
                asm_expr(s1, &mut e);
                if (*sec).sh_type != 8 as std::os::raw::c_int {
                    if size == 4 as std::os::raw::c_int {
                        gen_expr32(&mut e);
                    } else if size == 8 as std::os::raw::c_int {
                        gen_expr64(&mut e);
                    } else {
                        if !e.sym.is_null() {
                            expect(b"constant\x00" as *const u8 as *const std::os::raw::c_char);
                        }
                        if size == 1 as std::os::raw::c_int {
                            g(e.v as std::os::raw::c_int);
                        } else {
                            gen_le16(e.v as std::os::raw::c_int);
                        }
                    }
                } else {
                    ind += size
                }
                if tok != ',' as i32 {
                    break;
                }
                next();
            }
        },
        15295264098288563722 => {
            if (*sec).sh_type != 8 as std::os::raw::c_int {
                (*sec).data_offset = ind as std::os::raw::c_ulong;
                ptr = section_ptr_add(sec, size as Elf64_Addr) as *mut uint8_t;
                memset(
                    ptr as *mut std::os::raw::c_void,
                    v,
                    size as std::os::raw::c_ulong,
                );
            }
            ind += size
        },
        _ => {},
    };
}
/* assemble a file */
unsafe extern "C" fn tcc_assemble_internal(
    mut s1: *mut TCCState,
    mut do_preprocess: std::os::raw::c_int,
    mut global: std::os::raw::c_int,
) -> std::os::raw::c_int {
    let mut opcode: std::os::raw::c_int = 0;
    let mut saved_parse_flags: std::os::raw::c_int = parse_flags;
    parse_flags = 0x8 as std::os::raw::c_int | 0x40 as std::os::raw::c_int;
    if do_preprocess != 0 {
        parse_flags |= 0x1 as std::os::raw::c_int
    }
    loop {
        next();
        if tok == -(1 as std::os::raw::c_int) {
            break;
        }
        /* XXX: suppress that hack */
        parse_flags |= 0x4 as std::os::raw::c_int; /* XXX: suppress that hack */
        loop {
            if tok == '#' as i32 {
                /* horrible gas comment */
                while tok != 10 as std::os::raw::c_int {
                    next();
                }
                break;
            } else if tok >= TOK_ASMDIR_byte as std::os::raw::c_int
                && tok <= TOK_ASMDIR_section as std::os::raw::c_int
            {
                asm_parse_directive(s1, global);
                break;
            } else if tok == 0xcd as std::os::raw::c_int {
                let mut p: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
                let mut n: std::os::raw::c_int = 0;
                p = tokc.str_0.data as *const std::os::raw::c_char;
                n = strtoul(
                    p,
                    &mut p as *mut *const std::os::raw::c_char as *mut *mut std::os::raw::c_char,
                    10 as std::os::raw::c_int,
                ) as std::os::raw::c_int;
                if *p as std::os::raw::c_int != '\u{0}' as i32 {
                    expect(b"\':\'\x00" as *const u8 as *const std::os::raw::c_char);
                }
                /* new local label */
                asm_new_label(
                    s1,
                    asm_get_local_label_name(s1, n as std::os::raw::c_uint),
                    1 as std::os::raw::c_int,
                );
                next();
                skip(':' as i32);
            } else {
                if !(tok >= 256 as std::os::raw::c_int) {
                    break;
                }
                /* instruction or label */
                opcode = tok;
                next();
                if tok == ':' as i32 {
                    /* new label */
                    asm_new_label(s1, opcode, 0 as std::os::raw::c_int);
                    next();
                } else if tok == '=' as i32 {
                    set_symbol(s1, opcode);
                } else {
                    asm_opcode(s1, opcode);
                    break;
                }
            }
        }
        /* end of line */
        if tok != ';' as i32 && tok != 10 as std::os::raw::c_int {
            expect(b"end of line\x00" as *const u8 as *const std::os::raw::c_char);
        }
        parse_flags &= !(0x4 as std::os::raw::c_int)
    }
    parse_flags = saved_parse_flags;
    return 0 as std::os::raw::c_int;
}
/* Assemble the current file */
#[no_mangle]
pub unsafe extern "C" fn tcc_assemble(
    mut s1: *mut TCCState,
    mut do_preprocess: std::os::raw::c_int,
) -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0;
    tcc_debug_start(s1);
    /* default section is text */
    (*tcc_state).cur_text_section = (*tcc_state).text_section;
    ind = (*(*tcc_state).cur_text_section).data_offset as std::os::raw::c_int;
    nocode_wanted = 0 as std::os::raw::c_int;
    ret = tcc_assemble_internal(s1, do_preprocess, 1 as std::os::raw::c_int);
    (*(*tcc_state).cur_text_section).data_offset = ind as std::os::raw::c_ulong;
    tcc_debug_end(s1);
    return ret;
}
/* *******************************************************************/
/* GCC inline asm support */
/* assemble the string 'str' in the current C compilation unit without
C preprocessing. NOTE: str is modified by modifying the '\0' at the
end */
unsafe extern "C" fn tcc_assemble_inline(
    mut s1: *mut TCCState,
    mut str: *mut std::os::raw::c_char,
    mut len: std::os::raw::c_int,
    mut global: std::os::raw::c_int,
) {
    let mut saved_macro_ptr: *const std::os::raw::c_int = macro_ptr;
    let mut dotid: std::os::raw::c_int = set_idnum('.' as i32, 2 as std::os::raw::c_int);
    let mut dolid: std::os::raw::c_int = set_idnum('$' as i32, 0 as std::os::raw::c_int);
    tcc_open_bf(
        s1,
        b":asm:\x00" as *const u8 as *const std::os::raw::c_char,
        len,
    );
    memcpy(
        (*file).buffer.as_mut_ptr() as *mut std::os::raw::c_void,
        str as *const std::os::raw::c_void,
        len as std::os::raw::c_ulong,
    );
    macro_ptr = 0 as *const std::os::raw::c_int;
    tcc_assemble_internal(s1, 0 as std::os::raw::c_int, global);
    tcc_close();
    set_idnum('$' as i32, dolid);
    set_idnum('.' as i32, dotid);
    macro_ptr = saved_macro_ptr;
}
/* find a constraint by its number or id (gcc 3 extended
syntax). return -1 if not found. Return in *pp in char after the
constraint */
#[no_mangle]
pub unsafe extern "C" fn find_constraint(
    mut operands: *mut ASMOperand,
    mut nb_operands: std::os::raw::c_int,
    mut name: *const std::os::raw::c_char,
    mut pp: *mut *const std::os::raw::c_char,
) -> std::os::raw::c_int {
    let mut index: std::os::raw::c_int = 0;
    let mut ts: *mut TokenSym = 0 as *mut TokenSym;
    let mut p: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    if isnum(*name as std::os::raw::c_int) != 0 {
        index = 0 as std::os::raw::c_int;
        while isnum(*name as std::os::raw::c_int) != 0 {
            index = index * 10 as std::os::raw::c_int + *name as std::os::raw::c_int - '0' as i32;
            name = name.offset(1)
        }
        if index as std::os::raw::c_uint >= nb_operands as std::os::raw::c_uint {
            index = -(1 as std::os::raw::c_int)
        }
    } else if *name as std::os::raw::c_int == '[' as i32 {
        name = name.offset(1);
        p = strchr(name, ']' as i32);
        if !p.is_null() {
            let mut current_block_13: u64;
            ts = tok_alloc(
                name,
                p.offset_from(name) as std::os::raw::c_long as std::os::raw::c_int,
            );
            index = 0 as std::os::raw::c_int;
            loop {
                if !(index < nb_operands) {
                    current_block_13 = 8457315219000651999;
                    break;
                }
                if (*operands.offset(index as isize)).id == (*ts).tok {
                    current_block_13 = 2763335091496215905;
                    break;
                }
                index += 1
            }
            match current_block_13 {
                8457315219000651999 => index = -(1 as std::os::raw::c_int),
                _ => {},
            }
            name = p.offset(1 as std::os::raw::c_int as isize)
        } else {
            index = -(1 as std::os::raw::c_int)
        }
    } else {
        index = -(1 as std::os::raw::c_int)
    }
    if !pp.is_null() {
        *pp = name
    }
    return index;
}
unsafe extern "C" fn subst_asm_operands(
    mut operands: *mut ASMOperand,
    mut nb_operands: std::os::raw::c_int,
    mut out_str: *mut CString,
    mut in_str: *mut CString,
) {
    let mut c: std::os::raw::c_int = 0;
    let mut index: std::os::raw::c_int = 0;
    let mut modifier: std::os::raw::c_int = 0;
    let mut str: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut op: *mut ASMOperand = 0 as *mut ASMOperand;
    let mut sv: SValue = SValue {
        type_0: CType {
            t: 0,
            ref_0: 0 as *mut Sym,
        },
        r: 0,
        r2: 0,
        c2rust_unnamed: C2RustUnnamed_7 {
            c2rust_unnamed: C2RustUnnamed_8 {
                jtrue: 0,
                jfalse: 0,
            },
        },
        c2rust_unnamed_0: C2RustUnnamed_5 {
            c2rust_unnamed: C2RustUnnamed_6 {
                cmp_op: 0,
                cmp_r: 0,
            },
        },
    };
    cstr_new(out_str);
    str = (*in_str).data as *const std::os::raw::c_char;
    loop {
        let fresh2 = str;
        str = str.offset(1);
        c = *fresh2 as std::os::raw::c_int;
        if c == '%' as i32 {
            if *str as std::os::raw::c_int == '%' as i32 {
                str = str.offset(1)
            } else {
                modifier = 0 as std::os::raw::c_int;
                if *str as std::os::raw::c_int == 'c' as i32
                    || *str as std::os::raw::c_int == 'n' as i32
                    || *str as std::os::raw::c_int == 'b' as i32
                    || *str as std::os::raw::c_int == 'w' as i32
                    || *str as std::os::raw::c_int == 'h' as i32
                    || *str as std::os::raw::c_int == 'k' as i32
                    || *str as std::os::raw::c_int == 'q' as i32
                    || *str as std::os::raw::c_int == 'P' as i32
                {
                    let fresh3 = str;
                    str = str.offset(1);
                    modifier = *fresh3 as std::os::raw::c_int
                }
                index = find_constraint(operands, nb_operands, str, &mut str);
                if index < 0 as std::os::raw::c_int {
                    _tcc_error(
                        b"invalid operand reference after %%\x00" as *const u8
                            as *const std::os::raw::c_char,
                    );
                }
                op = &mut *operands.offset(index as isize) as *mut ASMOperand;
                sv = *(*op).vt;
                if (*op).reg >= 0 as std::os::raw::c_int {
                    sv.r = (*op).reg as std::os::raw::c_ushort;
                    if (*(*op).vt).r as std::os::raw::c_int & 0x3f as std::os::raw::c_int
                        == 0x31 as std::os::raw::c_int
                        && (*op).is_memory != 0
                    {
                        sv.r = (sv.r as std::os::raw::c_int | 0x100 as std::os::raw::c_int)
                            as std::os::raw::c_ushort
                    }
                }
                subst_asm_operand(out_str, &mut sv, modifier);
                continue;
            }
        }
        cstr_ccat(out_str, c);
        if c == '\u{0}' as i32 {
            break;
        }
    }
}
unsafe extern "C" fn parse_asm_operands(
    mut operands: *mut ASMOperand,
    mut nb_operands_ptr: *mut std::os::raw::c_int,
    mut is_output: std::os::raw::c_int,
) {
    let mut op: *mut ASMOperand = 0 as *mut ASMOperand;
    let mut nb_operands: std::os::raw::c_int = 0;
    if tok != ':' as i32 {
        nb_operands = *nb_operands_ptr;
        loop {
            let mut astr: CString = CString {
                size: 0,
                data: 0 as *mut std::os::raw::c_void,
                size_allocated: 0,
            };
            if nb_operands >= 30 as std::os::raw::c_int {
                _tcc_error(
                    b"too many asm operands\x00" as *const u8 as *const std::os::raw::c_char,
                );
            }
            let fresh4 = nb_operands;
            nb_operands = nb_operands + 1;
            op = &mut *operands.offset(fresh4 as isize) as *mut ASMOperand;
            (*op).id = 0 as std::os::raw::c_int;
            if tok == '[' as i32 {
                next();
                if tok < 256 as std::os::raw::c_int {
                    expect(b"identifier\x00" as *const u8 as *const std::os::raw::c_char);
                }
                (*op).id = tok;
                next();
                skip(']' as i32);
            }
            parse_mult_str(
                &mut astr,
                b"string constant\x00" as *const u8 as *const std::os::raw::c_char,
            );
            (*op).constraint =
                tcc_malloc(astr.size as std::os::raw::c_ulong) as *mut std::os::raw::c_char;
            strcpy((*op).constraint, astr.data as *const std::os::raw::c_char);
            cstr_free(&mut astr);
            skip('(' as i32);
            gexpr();
            if is_output != 0 {
                if (*vtop).type_0.t & 0x40 as std::os::raw::c_int == 0 {
                    test_lvalue();
                }
            } else if (*vtop).r as std::os::raw::c_int & 0x100 as std::os::raw::c_int != 0
                && ((*vtop).r as std::os::raw::c_int & 0x3f as std::os::raw::c_int
                    == 0x31 as std::os::raw::c_int
                    || ((*vtop).r as std::os::raw::c_int & 0x3f as std::os::raw::c_int)
                        < 0x30 as std::os::raw::c_int)
                && strchr((*op).constraint, 'm' as i32).is_null()
            {
                gv(0x1 as std::os::raw::c_int);
            }
            (*op).vt = vtop;
            skip(')' as i32);
            if !(tok == ',' as i32) {
                break;
            }
            next();
        }
        *nb_operands_ptr = nb_operands
    };
}
/* we want to avoid LLOCAL case, except when the 'm'
constraint is used. Note that it may come from
register storage, so we need to convert (reg)
case */
/* parse the GCC asm() instruction */
#[no_mangle]
pub unsafe extern "C" fn asm_instr() {
    let mut astr: CString = CString {
        size: 0,
        data: 0 as *mut std::os::raw::c_void,
        size_allocated: 0,
    };
    let mut astr1: CString = CString {
        size: 0,
        data: 0 as *mut std::os::raw::c_void,
        size_allocated: 0,
    };
    let mut operands: [ASMOperand; 30] = [ASMOperand {
        id: 0,
        constraint: 0 as *mut std::os::raw::c_char,
        asm_str: [0; 16],
        vt: 0 as *mut SValue,
        ref_index: 0,
        input_index: 0,
        priority: 0,
        reg: 0,
        is_llong: 0,
        is_memory: 0,
        is_rw: 0,
    }; 30];
    let mut nb_outputs: std::os::raw::c_int = 0;
    let mut nb_operands: std::os::raw::c_int = 0;
    let mut i: std::os::raw::c_int = 0;
    let mut must_subst: std::os::raw::c_int = 0;
    let mut out_reg: std::os::raw::c_int = 0;
    let mut clobber_regs: [uint8_t; 16] = [0; 16];
    let mut sec: *mut Section = 0 as *mut Section;
    /* since we always generate the asm() instruction, we can ignore
    volatile */
    if tok == TOK_VOLATILE1 as std::os::raw::c_int
        || tok == TOK_VOLATILE2 as std::os::raw::c_int
        || tok == TOK_VOLATILE3 as std::os::raw::c_int
    {
        next();
    }
    parse_asm_str(&mut astr);
    nb_operands = 0 as std::os::raw::c_int;
    nb_outputs = 0 as std::os::raw::c_int;
    must_subst = 0 as std::os::raw::c_int;
    memset(
        clobber_regs.as_mut_ptr() as *mut std::os::raw::c_void,
        0 as std::os::raw::c_int,
        ::std::mem::size_of::<[uint8_t; 16]>() as std::os::raw::c_ulong,
    );
    if tok == ':' as i32 {
        next();
        must_subst = 1 as std::os::raw::c_int;
        /* output args */
        parse_asm_operands(
            operands.as_mut_ptr(),
            &mut nb_operands,
            1 as std::os::raw::c_int,
        );
        nb_outputs = nb_operands;
        if tok == ':' as i32 {
            next();
            if tok != ')' as i32 {
                /* input args */
                parse_asm_operands(
                    operands.as_mut_ptr(),
                    &mut nb_operands,
                    0 as std::os::raw::c_int,
                );
                if tok == ':' as i32 {
                    /* clobber list */
                    /* XXX: handle registers */
                    next();
                    loop {
                        if tok != 0xc8 as std::os::raw::c_int {
                            expect(
                                b"string constant\x00" as *const u8 as *const std::os::raw::c_char,
                            );
                        }
                        asm_clobber(
                            clobber_regs.as_mut_ptr(),
                            tokc.str_0.data as *const std::os::raw::c_char,
                        );
                        next();
                        if !(tok == ',' as i32) {
                            break;
                        }
                        next();
                    }
                }
            }
        }
    }
    skip(')' as i32);
    /* NOTE: we do not eat the ';' so that we can restore the current
    token after the assembler parsing */
    if tok != ';' as i32 {
        expect(b"\';\'\x00" as *const u8 as *const std::os::raw::c_char);
    }
    /* save all values in the memory */
    save_regs(0 as std::os::raw::c_int);
    /* compute constraints */
    asm_compute_constraints(
        operands.as_mut_ptr(),
        nb_operands,
        nb_outputs,
        clobber_regs.as_mut_ptr(),
        &mut out_reg,
    );
    /* substitute the operands in the asm string. No substitution is
    done if no operands (GCC behaviour) */
    if must_subst != 0 {
        subst_asm_operands(operands.as_mut_ptr(), nb_operands, &mut astr1, &mut astr);
        cstr_free(&mut astr);
    } else {
        astr1 = astr
    }
    /* generate loads */
    asm_gen_code(
        operands.as_mut_ptr(),
        nb_operands,
        nb_outputs,
        0 as std::os::raw::c_int,
        clobber_regs.as_mut_ptr(),
        out_reg,
    );
    /* We don't allow switching section within inline asm to
    bleed out to surrounding code.  */
    sec = (*tcc_state).cur_text_section;
    /* assemble the string with tcc internal assembler */
    tcc_assemble_inline(
        tcc_state,
        astr1.data as *mut std::os::raw::c_char,
        astr1.size - 1 as std::os::raw::c_int,
        0 as std::os::raw::c_int,
    );
    if sec != (*tcc_state).cur_text_section {
        _tcc_warning(
            b"inline asm tries to change current section\x00" as *const u8
                as *const std::os::raw::c_char,
        );
        use_section1(tcc_state, sec);
    }
    /* restore the current C token */
    next();
    /* store the output values if needed */
    asm_gen_code(
        operands.as_mut_ptr(),
        nb_operands,
        nb_outputs,
        1 as std::os::raw::c_int,
        clobber_regs.as_mut_ptr(),
        out_reg,
    );
    /* free everything */
    i = 0 as std::os::raw::c_int;
    while i < nb_operands {
        let mut op: *mut ASMOperand = 0 as *mut ASMOperand;
        op = &mut *operands.as_mut_ptr().offset(i as isize) as *mut ASMOperand;
        tcc_free((*op).constraint as *mut std::os::raw::c_void);
        vpop();
        i += 1
    }
    cstr_free(&mut astr1);
}
#[no_mangle]
pub unsafe extern "C" fn asm_global_instr() {
    let mut astr: CString = CString {
        size: 0,
        data: 0 as *mut std::os::raw::c_void,
        size_allocated: 0,
    };
    let mut saved_nocode_wanted: std::os::raw::c_int = nocode_wanted;
    /* Global asm blocks are always emitted.  */
    nocode_wanted = 0 as std::os::raw::c_int;
    next();
    parse_asm_str(&mut astr);
    skip(')' as i32);
    /* NOTE: we do not eat the ';' so that we can restore the current
    token after the assembler parsing */
    if tok != ';' as i32 {
        expect(b"\';\'\x00" as *const u8 as *const std::os::raw::c_char);
    }
    (*tcc_state).cur_text_section = (*tcc_state).text_section;
    ind = (*(*tcc_state).cur_text_section).data_offset as std::os::raw::c_int;
    /* assemble the string with tcc internal assembler */
    tcc_assemble_inline(
        tcc_state,
        astr.data as *mut std::os::raw::c_char,
        astr.size - 1 as std::os::raw::c_int,
        1 as std::os::raw::c_int,
    );
    (*(*tcc_state).cur_text_section).data_offset = ind as std::os::raw::c_ulong;
    /* restore the current C token */
    next();
    cstr_free(&mut astr);
    nocode_wanted = saved_nocode_wanted;
}
/* CONFIG_TCC_ASM */
