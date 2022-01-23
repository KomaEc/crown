use crate::bitfields::*;
use ::f128;
use ::num_traits;
use num_traits::ToPrimitive;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type sym_version;
    #[no_mangle]
    fn strlen(_: *const std::os::raw::c_char) -> std::os::raw::c_ulong;
    #[no_mangle]
    fn strcmp(
        _: *const std::os::raw::c_char,
        _: *const std::os::raw::c_char,
    ) -> std::os::raw::c_int;
    #[no_mangle]
    fn strncpy(
        _: *mut std::os::raw::c_char,
        _: *const std::os::raw::c_char,
        _: std::os::raw::c_ulong,
    ) -> *mut std::os::raw::c_char;
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
    fn snprintf(
        _: *mut std::os::raw::c_char,
        _: std::os::raw::c_ulong,
        _: *const std::os::raw::c_char,
        _: ...
    ) -> std::os::raw::c_int;
    #[no_mangle]
    fn qsort(
        __base: *mut std::os::raw::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    #[no_mangle]
    fn getcwd(__buf: *mut std::os::raw::c_char, __size: size_t) -> *mut std::os::raw::c_char;
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
    fn _tcc_error_noabort(fmt: *const std::os::raw::c_char, _: ...);
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
    fn cstr_ccat(cstr: *mut CString, ch_0: std::os::raw::c_int);
    #[no_mangle]
    fn cstr_cat(cstr: *mut CString, str: *const std::os::raw::c_char, len: std::os::raw::c_int);
    #[no_mangle]
    fn cstr_new(cstr: *mut CString);
    #[no_mangle]
    fn cstr_free(cstr: *mut CString);
    #[no_mangle]
    fn cstr_printf(
        cs: *mut CString,
        fmt: *const std::os::raw::c_char,
        _: ...
    ) -> std::os::raw::c_int;
    #[no_mangle]
    fn cstr_reset(cstr: *mut CString);
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
    static mut ch: std::os::raw::c_int;
    #[no_mangle]
    static mut tok: std::os::raw::c_int;
    #[no_mangle]
    static mut tokc: CValue;
    #[no_mangle]
    static mut macro_ptr: *const std::os::raw::c_int;
    #[no_mangle]
    static mut parse_flags: std::os::raw::c_int;
    #[no_mangle]
    static mut tok_ident: std::os::raw::c_int;
    #[no_mangle]
    static mut table_ident: *mut *mut TokenSym;
    #[no_mangle]
    fn tok_alloc(str: *const std::os::raw::c_char, len: std::os::raw::c_int) -> *mut TokenSym;
    #[no_mangle]
    fn get_tok_str(v: std::os::raw::c_int, cv: *mut CValue) -> *const std::os::raw::c_char;
    #[no_mangle]
    fn begin_macro(str: *mut TokenString, alloc: std::os::raw::c_int);
    #[no_mangle]
    fn end_macro();
    #[no_mangle]
    fn tok_str_alloc() -> *mut TokenString;
    #[no_mangle]
    fn tok_str_free(s: *mut TokenString);
    #[no_mangle]
    fn tok_str_add(s: *mut TokenString, t: std::os::raw::c_int);
    #[no_mangle]
    fn tok_str_add_tok(s: *mut TokenString);
    #[no_mangle]
    fn free_defines(b: *mut Sym);
    #[no_mangle]
    fn label_find(v: std::os::raw::c_int) -> *mut Sym;
    #[no_mangle]
    fn label_push(
        ptop: *mut *mut Sym,
        v: std::os::raw::c_int,
        flags: std::os::raw::c_int,
    ) -> *mut Sym;
    #[no_mangle]
    fn label_pop(ptop: *mut *mut Sym, slast: *mut Sym, keep: std::os::raw::c_int);
    #[no_mangle]
    fn next();
    #[no_mangle]
    fn unget_tok(last_tok: std::os::raw::c_int);
    #[no_mangle]
    fn skip(c: std::os::raw::c_int);
    #[no_mangle]
    fn expect(msg: *const std::os::raw::c_char) -> !;
    #[no_mangle]
    fn expect_arg(msg: *const std::os::raw::c_char, arg: size_t) -> !;
    #[no_mangle]
    fn put_elf_sym(
        s: *mut Section,
        value: Elf64_Addr,
        size: std::os::raw::c_ulong,
        info: std::os::raw::c_int,
        other: std::os::raw::c_int,
        shndx: std::os::raw::c_int,
        name: *const std::os::raw::c_char,
    ) -> std::os::raw::c_int;
    #[no_mangle]
    fn put_stabs(
        s1: *mut TCCState,
        str: *const std::os::raw::c_char,
        type_0: std::os::raw::c_int,
        other: std::os::raw::c_int,
        desc: std::os::raw::c_int,
        value: std::os::raw::c_ulong,
    );
    #[no_mangle]
    fn put_stabs_r(
        s1: *mut TCCState,
        str: *const std::os::raw::c_char,
        type_0: std::os::raw::c_int,
        other: std::os::raw::c_int,
        desc: std::os::raw::c_int,
        value: std::os::raw::c_ulong,
        sec: *mut Section,
        sym_index: std::os::raw::c_int,
    );
    #[no_mangle]
    fn put_stabn(
        s1: *mut TCCState,
        type_0: std::os::raw::c_int,
        other: std::os::raw::c_int,
        desc: std::os::raw::c_int,
        value: std::os::raw::c_int,
    );
    #[no_mangle]
    fn section_ptr_add(sec: *mut Section, size: Elf64_Addr) -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn gfunc_epilog();
    #[no_mangle]
    fn gsym_addr(t: std::os::raw::c_int, a: std::os::raw::c_int);
    #[no_mangle]
    fn gen_increment_tcov(sv: *mut SValue);
    #[no_mangle]
    fn o(c: std::os::raw::c_uint);
    #[no_mangle]
    fn load(r: std::os::raw::c_int, sv: *mut SValue);
    #[no_mangle]
    fn gjmp(t: std::os::raw::c_int) -> std::os::raw::c_int;
    #[no_mangle]
    fn gjmp_cond(op: std::os::raw::c_int, t: std::os::raw::c_int) -> std::os::raw::c_int;
    #[no_mangle]
    fn gjmp_append(n: std::os::raw::c_int, t: std::os::raw::c_int) -> std::os::raw::c_int;
    #[no_mangle]
    fn gen_opi(op: std::os::raw::c_int);
    #[no_mangle]
    fn gen_opl(op: std::os::raw::c_int);
    #[no_mangle]
    fn gen_opf(op: std::os::raw::c_int);
    #[no_mangle]
    fn gen_cvt_csti(t: std::os::raw::c_int);
    #[no_mangle]
    fn gen_cvt_sxtw();
    #[no_mangle]
    static mut func_bound_add_epilog: std::os::raw::c_int;
    #[no_mangle]
    fn new_section(
        s1: *mut TCCState,
        name: *const std::os::raw::c_char,
        sh_type: std::os::raw::c_int,
        sh_flags: std::os::raw::c_int,
    ) -> *mut Section;
    #[no_mangle]
    fn gen_cvt_ftoi(t: std::os::raw::c_int);
    #[no_mangle]
    fn gfunc_call(nb_args: std::os::raw::c_int);
    #[no_mangle]
    fn gen_fill_nops(_: std::os::raw::c_int);
    #[no_mangle]
    fn add_array(s1: *mut TCCState, sec: *const std::os::raw::c_char, c: std::os::raw::c_int);
    #[no_mangle]
    fn gfunc_prolog(func_sym: *mut Sym);
    #[no_mangle]
    fn asm_global_instr();
    #[no_mangle]
    fn ggoto();
    #[no_mangle]
    fn gjmp_addr(a: std::os::raw::c_int);
    #[no_mangle]
    fn gen_cvt_itof(t: std::os::raw::c_int);
    #[no_mangle]
    fn gen_cvt_ftof(t: std::os::raw::c_int);
    #[no_mangle]
    fn asm_instr();
    #[no_mangle]
    fn gen_vla_sp_restore(addr: std::os::raw::c_int);
    #[no_mangle]
    fn asm_parse_regvar(t: std::os::raw::c_int) -> std::os::raw::c_int;
    #[no_mangle]
    fn gen_vla_alloc(type_0: *mut CType, align: std::os::raw::c_int);
    #[no_mangle]
    fn gen_vla_sp_save(addr: std::os::raw::c_int);
    #[no_mangle]
    fn classify_x86_64_va_arg(ty: *mut CType) -> std::os::raw::c_int;
    #[no_mangle]
    fn find_section(s1: *mut TCCState, name: *const std::os::raw::c_char) -> *mut Section;
    #[no_mangle]
    fn store(r: std::os::raw::c_int, v: *mut SValue);
    #[no_mangle]
    static reg_classes: [std::os::raw::c_int; 25];
    #[no_mangle]
    fn put_elf_reloca(
        symtab: *mut Section,
        s: *mut Section,
        offset: std::os::raw::c_ulong,
        type_0: std::os::raw::c_int,
        symbol: std::os::raw::c_int,
        addend: Elf64_Addr,
    );
    #[no_mangle]
    fn gfunc_sret(
        vt: *mut CType,
        variadic: std::os::raw::c_int,
        ret: *mut CType,
        align: *mut std::os::raw::c_int,
        regsize: *mut std::os::raw::c_int,
    ) -> std::os::raw::c_int;
    #[no_mangle]
    fn section_add(sec: *mut Section, size: Elf64_Addr, align: std::os::raw::c_int) -> size_t;
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
pub type __compar_fn_t = Option<
    unsafe extern "C" fn(
        _: *const std::os::raw::c_void,
        _: *const std::os::raw::c_void,
    ) -> std::os::raw::c_int,
>;
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
pub type nwchar_t = std::os::raw::c_int;
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
pub struct AttributeDef {
    pub a: SymAttr,
    pub f: FuncAttr,
    pub section: *mut Section,
    pub cleanup_func: *mut Sym,
    pub alias_target: std::os::raw::c_int,
    pub asm_label: std::os::raw::c_int,
    pub attr_mode: std::os::raw::c_char,
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
pub struct debug_hash {
    pub debug_type: std::os::raw::c_int,
    pub type_0: *mut Sym,
}
/* *******************************************************/
/* stab debug support */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub type_0: std::os::raw::c_int,
    pub name: *const std::os::raw::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub offset: std::os::raw::c_ulong,
    pub last_file_name: std::os::raw::c_ulong,
    pub last_func_name: std::os::raw::c_ulong,
    pub ind: std::os::raw::c_int,
    pub line: std::os::raw::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct debug_info {
    pub start: std::os::raw::c_int,
    pub end: std::os::raw::c_int,
    pub n_sym: std::os::raw::c_int,
    pub sym: *mut debug_sym,
    pub child: *mut debug_info,
    pub next: *mut debug_info,
    pub last: *mut debug_info,
    pub parent: *mut debug_info,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct debug_sym {
    pub type_0: std::os::raw::c_int,
    pub value: std::os::raw::c_ulong,
    pub str_0: *mut std::os::raw::c_char,
    pub sec: *mut Section,
    pub sym_index: std::os::raw::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_12 {
    pub f: std::os::raw::c_float,
    pub u: std::os::raw::c_uint,
}
/*list of temporary local variables on the stack in current function. */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct temp_local_variable {
    pub location: std::os::raw::c_int,
    pub size: std::os::raw::c_short,
    pub align: std::os::raw::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct init_params {
    pub sec: *mut Section,
    pub local_offset: std::os::raw::c_int,
    pub flex_array_ref: *mut Sym,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub loc: std::os::raw::c_int,
    pub locorig: std::os::raw::c_int,
    pub num: std::os::raw::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct scope {
    pub prev: *mut scope,
    pub vla: C2RustUnnamed_13,
    pub cl: C2RustUnnamed_14,
    pub bsym: *mut std::os::raw::c_int,
    pub csym: *mut std::os::raw::c_int,
    pub lstk: *mut Sym,
    pub llstk: *mut Sym,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
    pub s: *mut Sym,
    pub n: std::os::raw::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct switch_t {
    pub p: *mut *mut case_t,
    pub n: std::os::raw::c_int,
    pub def_sym: std::os::raw::c_int,
    pub bsym: *mut std::os::raw::c_int,
    pub scope: *mut scope,
    pub prev: *mut switch_t,
    pub sv: SValue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct case_t {
    pub v1: int64_t,
    pub v2: int64_t,
    pub sym: std::os::raw::c_int,
}
#[inline]
unsafe extern "C" fn read64le(mut p: *mut std::os::raw::c_uchar) -> uint64_t {
    return read32le(p) as std::os::raw::c_ulong
        | (read32le(p.offset(4 as std::os::raw::c_int as isize)) as uint64_t)
            << 32 as std::os::raw::c_int;
}
#[inline]
unsafe extern "C" fn read32le(mut p: *mut std::os::raw::c_uchar) -> uint32_t {
    return read16le(p) as std::os::raw::c_uint
        | (read16le(p.offset(2 as std::os::raw::c_int as isize)) as uint32_t)
            << 16 as std::os::raw::c_int;
}
#[inline]
unsafe extern "C" fn read16le(mut p: *mut std::os::raw::c_uchar) -> uint16_t {
    return (*p.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int
        | (*p.offset(1 as std::os::raw::c_int as isize) as uint16_t as std::os::raw::c_int)
            << 8 as std::os::raw::c_int) as uint16_t;
}
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
/* loc : local variable index
   ind : output code index
   rsym: return symbol
   anon_sym: anonymous symbol index
*/
#[no_mangle]
pub static mut rsym: std::os::raw::c_int = 0;
#[no_mangle]
pub static mut anon_sym: std::os::raw::c_int = 0;
#[no_mangle]
pub static mut ind: std::os::raw::c_int = 0;
#[no_mangle]
pub static mut loc: std::os::raw::c_int = 0;
#[no_mangle]
pub static mut global_stack: *mut Sym = 0 as *const Sym as *mut Sym;
#[no_mangle]
pub static mut local_stack: *mut Sym = 0 as *const Sym as *mut Sym;
#[no_mangle]
pub static mut define_stack: *mut Sym = 0 as *const Sym as *mut Sym;
#[no_mangle]
pub static mut global_label_stack: *mut Sym = 0 as *const Sym as *mut Sym;
#[no_mangle]
pub static mut local_label_stack: *mut Sym = 0 as *const Sym as *mut Sym;
static mut sym_free_first: *mut Sym = 0 as *const Sym as *mut Sym;
static mut sym_pools: *mut *mut std::os::raw::c_void =
    0 as *const *mut std::os::raw::c_void as *mut *mut std::os::raw::c_void;
static mut nb_sym_pools: std::os::raw::c_int = 0;
static mut all_cleanups: *mut Sym = 0 as *const Sym as *mut Sym;
static mut pending_gotos: *mut Sym = 0 as *const Sym as *mut Sym;
static mut local_scope: std::os::raw::c_int = 0;
static mut in_sizeof: std::os::raw::c_int = 0;
static mut in_generic: std::os::raw::c_int = 0;
static mut section_sym: std::os::raw::c_int = 0;
#[no_mangle]
pub static mut debug_modes: std::os::raw::c_char = 0;
#[no_mangle]
pub static mut vtop: *mut SValue = 0 as *const SValue as *mut SValue;
static mut _vstack: [SValue; 257] = [SValue {
    type_0: CType {
        t: 0,
        ref_0: 0 as *const Sym as *mut Sym,
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
}; 257];
#[no_mangle]
pub static mut const_wanted: std::os::raw::c_int = 0;
/* true if constant wanted */
#[no_mangle]
pub static mut nocode_wanted: std::os::raw::c_int = 0;
/* Clear 'nocode_wanted' at label if it was used */
#[no_mangle]
pub unsafe extern "C" fn gsym(mut t: std::os::raw::c_int) {
    if t != 0 {
        gsym_addr(t, ind);
        nocode_wanted &= !(0x20000000 as std::os::raw::c_int)
    };
}
unsafe extern "C" fn gind() -> std::os::raw::c_int {
    let mut t: std::os::raw::c_int = ind;
    nocode_wanted &= !(0x20000000 as std::os::raw::c_int);
    if debug_modes != 0 {
        tcc_tcov_block_begin();
    }
    return t;
}
/* Set 'nocode_wanted' after unconditional jumps */
unsafe extern "C" fn gjmp_addr_acs(mut t: std::os::raw::c_int) {
    gjmp_addr(t);
    nocode_wanted |= 0x20000000 as std::os::raw::c_int;
}
unsafe extern "C" fn gjmp_acs(mut t: std::os::raw::c_int) -> std::os::raw::c_int {
    t = gjmp(t);
    nocode_wanted |= 0x20000000 as std::os::raw::c_int;
    return t;
}
/* <---- */
#[no_mangle]
pub static mut global_expr: std::os::raw::c_int = 0;
/* true if compound literals must be allocated globally (used during initializers parsing */
#[no_mangle]
pub static mut func_vt: CType = CType {
    t: 0,
    ref_0: 0 as *const Sym as *mut Sym,
};
/* current function return type (used by return instruction) */
#[no_mangle]
pub static mut func_var: std::os::raw::c_int = 0;
/* true if current function is variadic (used by return instruction) */
#[no_mangle]
pub static mut func_vc: std::os::raw::c_int = 0;
static mut last_line_num: std::os::raw::c_int = 0;
static mut new_file: std::os::raw::c_int = 0;
static mut func_ind: std::os::raw::c_int = 0;
/* debug info control */
#[no_mangle]
pub static mut funcname: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
#[no_mangle]
pub static mut int_type: CType = CType {
    t: 0,
    ref_0: 0 as *const Sym as *mut Sym,
};
#[no_mangle]
pub static mut func_old_type: CType = CType {
    t: 0,
    ref_0: 0 as *const Sym as *mut Sym,
};
#[no_mangle]
pub static mut char_type: CType = CType {
    t: 0,
    ref_0: 0 as *const Sym as *mut Sym,
};
#[no_mangle]
pub static mut char_pointer_type: CType = CType {
    t: 0,
    ref_0: 0 as *const Sym as *mut Sym,
};
static mut initstr: CString = CString {
    size: 0,
    data: 0 as *const std::os::raw::c_void as *mut std::os::raw::c_void,
    size_allocated: 0,
};
#[no_mangle]
pub static mut cur_switch: *mut switch_t = 0 as *const switch_t as *mut switch_t;
#[no_mangle]
pub static mut arr_temp_local_vars: [temp_local_variable; 8] = [temp_local_variable {
    location: 0,
    size: 0,
    align: 0,
}; 8];
#[no_mangle]
pub static mut nb_temp_local_vars: std::os::raw::c_short = 0;
static mut cur_scope: *mut scope = 0 as *const scope as *mut scope;
static mut loop_scope: *mut scope = 0 as *const scope as *mut scope;
static mut root_scope: *mut scope = 0 as *const scope as *mut scope;
static mut default_debug: [C2RustUnnamed_10; 27] = [
    {
        let mut init = C2RustUnnamed_10 {
            type_0: 3 as std::os::raw::c_int,
            name: b"int:t1=r1;-2147483648;2147483647;\x00" as *const u8
                as *const std::os::raw::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_10 {
            type_0: 1 as std::os::raw::c_int,
            name: b"char:t2=r2;0;127;\x00" as *const u8 as *const std::os::raw::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_10 {
            type_0: 4 as std::os::raw::c_int | 0x800 as std::os::raw::c_int,
            name: b"long int:t3=r3;-9223372036854775808;9223372036854775807;\x00" as *const u8
                as *const std::os::raw::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_10 {
            type_0: 3 as std::os::raw::c_int | 0x10 as std::os::raw::c_int,
            name: b"unsigned int:t4=r4;0;037777777777;\x00" as *const u8
                as *const std::os::raw::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_10 {
            type_0: 4 as std::os::raw::c_int
                | 0x800 as std::os::raw::c_int
                | 0x10 as std::os::raw::c_int,
            name: b"long unsigned int:t5=r5;0;01777777777777777777777;\x00" as *const u8
                as *const std::os::raw::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_10 {
            type_0: 13 as std::os::raw::c_int,
            name: b"__int128:t6=r6;0;-1;\x00" as *const u8 as *const std::os::raw::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_10 {
            type_0: 13 as std::os::raw::c_int | 0x10 as std::os::raw::c_int,
            name: b"__int128 unsigned:t7=r7;0;-1;\x00" as *const u8 as *const std::os::raw::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_10 {
            type_0: 4 as std::os::raw::c_int,
            name: b"long long int:t8=r8;-9223372036854775808;9223372036854775807;\x00" as *const u8
                as *const std::os::raw::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_10 {
            type_0: 4 as std::os::raw::c_int | 0x10 as std::os::raw::c_int,
            name: b"long long unsigned int:t9=r9;0;01777777777777777777777;\x00" as *const u8
                as *const std::os::raw::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_10 {
            type_0: 2 as std::os::raw::c_int,
            name: b"short int:t10=r10;-32768;32767;\x00" as *const u8
                as *const std::os::raw::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_10 {
            type_0: 2 as std::os::raw::c_int | 0x10 as std::os::raw::c_int,
            name: b"short unsigned int:t11=r11;0;65535;\x00" as *const u8
                as *const std::os::raw::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_10 {
            type_0: 1 as std::os::raw::c_int | 0x20 as std::os::raw::c_int,
            name: b"signed char:t12=r12;-128;127;\x00" as *const u8 as *const std::os::raw::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_10 {
            type_0: 1 as std::os::raw::c_int
                | 0x20 as std::os::raw::c_int
                | 0x10 as std::os::raw::c_int,
            name: b"unsigned char:t13=r13;0;255;\x00" as *const u8 as *const std::os::raw::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_10 {
            type_0: 8 as std::os::raw::c_int,
            name: b"float:t14=r1;4;0;\x00" as *const u8 as *const std::os::raw::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_10 {
            type_0: 9 as std::os::raw::c_int,
            name: b"double:t15=r1;8;0;\x00" as *const u8 as *const std::os::raw::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_10 {
            type_0: 10 as std::os::raw::c_int,
            name: b"long double:t16=r1;16;0;\x00" as *const u8 as *const std::os::raw::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_10 {
            type_0: -(1 as std::os::raw::c_int),
            name: b"_Float32:t17=r1;4;0;\x00" as *const u8 as *const std::os::raw::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_10 {
            type_0: -(1 as std::os::raw::c_int),
            name: b"_Float64:t18=r1;8;0;\x00" as *const u8 as *const std::os::raw::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_10 {
            type_0: -(1 as std::os::raw::c_int),
            name: b"_Float128:t19=r1;16;0;\x00" as *const u8 as *const std::os::raw::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_10 {
            type_0: -(1 as std::os::raw::c_int),
            name: b"_Float32x:t20=r1;8;0;\x00" as *const u8 as *const std::os::raw::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_10 {
            type_0: -(1 as std::os::raw::c_int),
            name: b"_Float64x:t21=r1;16;0;\x00" as *const u8 as *const std::os::raw::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_10 {
            type_0: -(1 as std::os::raw::c_int),
            name: b"_Decimal32:t22=r1;4;0;\x00" as *const u8 as *const std::os::raw::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_10 {
            type_0: -(1 as std::os::raw::c_int),
            name: b"_Decimal64:t23=r1;8;0;\x00" as *const u8 as *const std::os::raw::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_10 {
            type_0: -(1 as std::os::raw::c_int),
            name: b"_Decimal128:t24=r1;16;0;\x00" as *const u8 as *const std::os::raw::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_10 {
            type_0: 1 as std::os::raw::c_int | 0x10 as std::os::raw::c_int,
            name: b"unsigned char:t25=r25;0;255;\x00" as *const u8 as *const std::os::raw::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_10 {
            type_0: 11 as std::os::raw::c_int,
            name: b"bool:t26=r26;0;255;\x00" as *const u8 as *const std::os::raw::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_10 {
            type_0: 0 as std::os::raw::c_int,
            name: b"void:t27=27\x00" as *const u8 as *const std::os::raw::c_char,
        };
        init
    },
];
static mut debug_next_type: std::os::raw::c_int = 0;
static mut debug_hash: *mut debug_hash = 0 as *const debug_hash as *mut debug_hash;
static mut n_debug_hash: std::os::raw::c_int = 0;
static mut debug_info: *mut debug_info = 0 as *const debug_info as *mut debug_info;
static mut debug_info_root: *mut debug_info = 0 as *const debug_info as *mut debug_info;
static mut tcov_data: C2RustUnnamed_11 = C2RustUnnamed_11 {
    offset: 0,
    last_file_name: 0,
    last_func_name: 0,
    ind: 0,
    line: 0,
};
#[no_mangle]
pub unsafe extern "C" fn is_float(mut t: std::os::raw::c_int) -> std::os::raw::c_int {
    let mut bt: std::os::raw::c_int = t & 0xf as std::os::raw::c_int;
    return (bt == 10 as std::os::raw::c_int
        || bt == 9 as std::os::raw::c_int
        || bt == 8 as std::os::raw::c_int
        || bt == 14 as std::os::raw::c_int) as std::os::raw::c_int;
}
#[inline]
unsafe extern "C" fn is_integer_btype(mut bt: std::os::raw::c_int) -> std::os::raw::c_int {
    return (bt == 1 as std::os::raw::c_int
        || bt == 11 as std::os::raw::c_int
        || bt == 2 as std::os::raw::c_int
        || bt == 3 as std::os::raw::c_int
        || bt == 4 as std::os::raw::c_int) as std::os::raw::c_int;
}
unsafe extern "C" fn btype_size(mut bt: std::os::raw::c_int) -> std::os::raw::c_int {
    return if bt == 1 as std::os::raw::c_int || bt == 11 as std::os::raw::c_int {
        1 as std::os::raw::c_int
    } else if bt == 2 as std::os::raw::c_int {
        2 as std::os::raw::c_int
    } else if bt == 3 as std::os::raw::c_int {
        4 as std::os::raw::c_int
    } else if bt == 4 as std::os::raw::c_int {
        8 as std::os::raw::c_int
    } else if bt == 5 as std::os::raw::c_int {
        8 as std::os::raw::c_int
    } else {
        0 as std::os::raw::c_int
    };
}
/* returns function return register from type */
unsafe extern "C" fn R_RET(mut t: std::os::raw::c_int) -> std::os::raw::c_int {
    if is_float(t) == 0 {
        return TREG_RAX as std::os::raw::c_int;
    }
    if t & 0xf as std::os::raw::c_int == 10 as std::os::raw::c_int {
        return TREG_ST0 as std::os::raw::c_int;
    }
    return TREG_XMM0 as std::os::raw::c_int;
}
/* returns 2nd function return register, if any */
unsafe extern "C" fn R2_RET(mut t: std::os::raw::c_int) -> std::os::raw::c_int {
    t &= 0xf as std::os::raw::c_int;
    if t == 13 as std::os::raw::c_int {
        return TREG_RDX as std::os::raw::c_int;
    }
    if t == 14 as std::os::raw::c_int {
        return TREG_XMM1 as std::os::raw::c_int;
    }
    return 0x30 as std::os::raw::c_int;
}
/* put function return registers to stack value */
unsafe extern "C" fn PUT_R_RET(mut sv: *mut SValue, mut t: std::os::raw::c_int) {
    (*sv).r = R_RET(t) as std::os::raw::c_ushort;
    (*sv).r2 = R2_RET(t) as std::os::raw::c_ushort;
}
/* returns function return register class for type t */
unsafe extern "C" fn RC_RET(mut t: std::os::raw::c_int) -> std::os::raw::c_int {
    return reg_classes[R_RET(t) as usize]
        & !(0x2 as std::os::raw::c_int | 0x1 as std::os::raw::c_int);
}
/* returns generic register class for type t */
unsafe extern "C" fn RC_TYPE(mut t: std::os::raw::c_int) -> std::os::raw::c_int {
    if is_float(t) == 0 {
        return 0x1 as std::os::raw::c_int;
    }
    if t & 0xf as std::os::raw::c_int == 10 as std::os::raw::c_int {
        return 0x80 as std::os::raw::c_int;
    }
    if t & 0xf as std::os::raw::c_int == 14 as std::os::raw::c_int {
        return 0x1000 as std::os::raw::c_int;
    }
    return 0x2 as std::os::raw::c_int;
}
/* returns 2nd register class corresponding to t and rc */
unsafe extern "C" fn RC2_TYPE(
    mut t: std::os::raw::c_int,
    mut rc: std::os::raw::c_int,
) -> std::os::raw::c_int {
    if !(R2_RET(t) != 0x30 as std::os::raw::c_int) {
        return 0 as std::os::raw::c_int;
    }
    if rc == 0x4 as std::os::raw::c_int {
        return 0x10 as std::os::raw::c_int;
    }
    if rc == 0x1000 as std::os::raw::c_int {
        return 0x2000 as std::os::raw::c_int;
    }
    if rc & 0x2 as std::os::raw::c_int != 0 {
        return 0x2 as std::os::raw::c_int;
    }
    return 0x1 as std::os::raw::c_int;
}
/* we use our own 'finite' function to avoid potential problems with
non standard math libs */
/* XXX: endianness dependent */
#[no_mangle]
pub unsafe extern "C" fn ieee_finite(mut d: std::os::raw::c_double) -> std::os::raw::c_int {
    let mut p: [std::os::raw::c_int; 4] = [0; 4];
    memcpy(
        p.as_mut_ptr() as *mut std::os::raw::c_void,
        &mut d as *mut std::os::raw::c_double as *const std::os::raw::c_void,
        ::std::mem::size_of::<std::os::raw::c_double>() as std::os::raw::c_ulong,
    );
    return ((p[1 as std::os::raw::c_int as usize] as std::os::raw::c_uint
        | 0x800fffff as std::os::raw::c_uint)
        .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_uint)
        >> 31 as std::os::raw::c_int) as std::os::raw::c_int;
}
/* compiling intel long double natively */
#[no_mangle]
pub unsafe extern "C" fn test_lvalue() {
    if (*vtop).r as std::os::raw::c_int & 0x100 as std::os::raw::c_int == 0 {
        expect(b"lvalue\x00" as *const u8 as *const std::os::raw::c_char);
    };
}
#[no_mangle]
pub unsafe extern "C" fn check_vstack() {
    if vtop
        != _vstack
            .as_mut_ptr()
            .offset(1 as std::os::raw::c_int as isize)
            .offset(-(1 as std::os::raw::c_int as isize))
    {
        _tcc_error(
            b"internal compiler error: vstack leak (%d)\x00" as *const u8
                as *const std::os::raw::c_char,
            (vtop.offset_from(
                _vstack
                    .as_mut_ptr()
                    .offset(1 as std::os::raw::c_int as isize),
            ) as std::os::raw::c_long
                + 1 as std::os::raw::c_int as std::os::raw::c_long)
                as std::os::raw::c_int,
        );
    };
}
/* ------------------------------------------------------------------------- */
/* vstack debugging aid */
/* ------------------------------------------------------------------------- */
/* start of translation unit info */
#[no_mangle]
pub unsafe extern "C" fn tcc_debug_start(mut s1: *mut TCCState) {
    if (*s1).do_debug != 0 {
        let mut i: std::os::raw::c_int = 0;
        let mut buf: [std::os::raw::c_char; 512] = [0; 512];
        /* file info: full path + filename */
        section_sym = put_elf_sym(
            (*tcc_state).symtab_section,
            0 as std::os::raw::c_int as Elf64_Addr,
            0 as std::os::raw::c_int as std::os::raw::c_ulong,
            ((0 as std::os::raw::c_int) << 4 as std::os::raw::c_int)
                + (3 as std::os::raw::c_int & 0xf as std::os::raw::c_int),
            0 as std::os::raw::c_int,
            (*(*tcc_state).text_section).sh_num,
            0 as *const std::os::raw::c_char,
        );
        getcwd(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[std::os::raw::c_char; 512]>() as std::os::raw::c_ulong,
        );
        pstrcat(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[std::os::raw::c_char; 512]>() as std::os::raw::c_ulong,
            b"/\x00" as *const u8 as *const std::os::raw::c_char,
        );
        put_stabs_r(
            s1,
            buf.as_mut_ptr(),
            N_SO as std::os::raw::c_int,
            0 as std::os::raw::c_int,
            0 as std::os::raw::c_int,
            (*(*tcc_state).text_section).data_offset,
            (*tcc_state).text_section,
            section_sym,
        );
        put_stabs_r(
            s1,
            if !(*file).prev.is_null() {
                (*(*file).prev).filename.as_mut_ptr()
            } else {
                (*file).filename.as_mut_ptr()
            },
            N_SO as std::os::raw::c_int,
            0 as std::os::raw::c_int,
            0 as std::os::raw::c_int,
            (*(*tcc_state).text_section).data_offset,
            (*tcc_state).text_section,
            section_sym,
        );
        i = 0 as std::os::raw::c_int;
        while (i as std::os::raw::c_ulong)
            < (::std::mem::size_of::<[C2RustUnnamed_10; 27]>() as std::os::raw::c_ulong)
                .wrapping_div(::std::mem::size_of::<C2RustUnnamed_10>() as std::os::raw::c_ulong)
        {
            put_stabs(
                s1,
                default_debug[i as usize].name,
                N_LSYM as std::os::raw::c_int,
                0 as std::os::raw::c_int,
                0 as std::os::raw::c_int,
                0 as std::os::raw::c_int as std::os::raw::c_ulong,
            );
            i += 1
        }
        last_line_num = 0 as std::os::raw::c_int;
        new_file = last_line_num;
        func_ind = -(1 as std::os::raw::c_int);
        debug_next_type = (::std::mem::size_of::<[C2RustUnnamed_10; 27]>() as std::os::raw::c_ulong)
            .wrapping_div(::std::mem::size_of::<C2RustUnnamed_10>() as std::os::raw::c_ulong)
            as std::os::raw::c_int;
        debug_hash = 0 as *mut debug_hash;
        n_debug_hash = 0 as std::os::raw::c_int;
        /* we're currently 'including' the <command line> */
        tcc_debug_bincl(s1);
    }
    /* an elf symbol of type STT_FILE must be put so that STB_LOCAL
    symbols can be safely used */
    put_elf_sym(
        (*tcc_state).symtab_section,
        0 as std::os::raw::c_int as Elf64_Addr,
        0 as std::os::raw::c_int as std::os::raw::c_ulong,
        ((0 as std::os::raw::c_int) << 4 as std::os::raw::c_int)
            + (4 as std::os::raw::c_int & 0xf as std::os::raw::c_int),
        0 as std::os::raw::c_int,
        0xfff1 as std::os::raw::c_int,
        (*file).filename.as_mut_ptr(),
    );
}
/* put end of translation unit info */
#[no_mangle]
pub unsafe extern "C" fn tcc_debug_end(mut s1: *mut TCCState) {
    if (*s1).do_debug == 0 {
        return;
    }
    put_stabs_r(
        s1,
        0 as *const std::os::raw::c_char,
        N_SO as std::os::raw::c_int,
        0 as std::os::raw::c_int,
        0 as std::os::raw::c_int,
        (*(*tcc_state).text_section).data_offset,
        (*tcc_state).text_section,
        section_sym,
    );
    tcc_free(debug_hash as *mut std::os::raw::c_void);
}
unsafe extern "C" fn put_new_file(mut s1: *mut TCCState) -> *mut BufferedFile {
    let mut f: *mut BufferedFile = file;
    /* use upper file if from inline ":asm:" */
    if (*f).filename[0 as std::os::raw::c_int as usize] as std::os::raw::c_int == ':' as i32 {
        f = (*f).prev
    }
    if !f.is_null() && new_file != 0 {
        put_stabs_r(
            s1,
            (*f).filename.as_mut_ptr(),
            N_SOL as std::os::raw::c_int,
            0 as std::os::raw::c_int,
            0 as std::os::raw::c_int,
            ind as std::os::raw::c_ulong,
            (*tcc_state).text_section,
            section_sym,
        );
        last_line_num = 0 as std::os::raw::c_int;
        new_file = last_line_num
    }
    return f;
}
/* put alternative filename */
#[no_mangle]
pub unsafe extern "C" fn tcc_debug_putfile(
    mut s1: *mut TCCState,
    mut filename: *const std::os::raw::c_char,
) {
    if 0 as std::os::raw::c_int == strcmp((*file).filename.as_mut_ptr(), filename) {
        return;
    }
    pstrcpy(
        (*file).filename.as_mut_ptr(),
        ::std::mem::size_of::<[std::os::raw::c_char; 1024]>() as std::os::raw::c_ulong,
        filename,
    );
    new_file = 1 as std::os::raw::c_int;
}
/* begin of #include */
#[no_mangle]
pub unsafe extern "C" fn tcc_debug_bincl(mut s1: *mut TCCState) {
    if (*s1).do_debug == 0 {
        return;
    }
    put_stabs(
        s1,
        (*file).filename.as_mut_ptr(),
        N_BINCL as std::os::raw::c_int,
        0 as std::os::raw::c_int,
        0 as std::os::raw::c_int,
        0 as std::os::raw::c_int as std::os::raw::c_ulong,
    );
    new_file = 1 as std::os::raw::c_int;
}
/* end of #include */
#[no_mangle]
pub unsafe extern "C" fn tcc_debug_eincl(mut s1: *mut TCCState) {
    if (*s1).do_debug == 0 {
        return;
    }
    put_stabn(
        s1,
        N_EINCL as std::os::raw::c_int,
        0 as std::os::raw::c_int,
        0 as std::os::raw::c_int,
        0 as std::os::raw::c_int,
    );
    new_file = 1 as std::os::raw::c_int;
}
/* generate line number info */
unsafe extern "C" fn tcc_debug_line(mut s1: *mut TCCState) {
    let mut f: *mut BufferedFile = 0 as *mut BufferedFile;
    if (*s1).do_debug == 0
        || (*tcc_state).cur_text_section != (*tcc_state).text_section
        || {
            f = put_new_file(s1);
            f.is_null()
        }
        || last_line_num == (*f).line_num
    {
        return;
    }
    if func_ind != -(1 as std::os::raw::c_int) {
        put_stabn(
            s1,
            N_SLINE as std::os::raw::c_int,
            0 as std::os::raw::c_int,
            (*f).line_num,
            ind - func_ind,
        );
    } else {
        /* from tcc_assemble */
        put_stabs_r(
            s1,
            0 as *const std::os::raw::c_char,
            N_SLINE as std::os::raw::c_int,
            0 as std::os::raw::c_int,
            (*f).line_num,
            ind as std::os::raw::c_ulong,
            (*tcc_state).text_section,
            section_sym,
        );
    }
    last_line_num = (*f).line_num;
}
unsafe extern "C" fn tcc_debug_stabs(
    mut s1: *mut TCCState,
    mut str: *const std::os::raw::c_char,
    mut type_0: std::os::raw::c_int,
    mut value: std::os::raw::c_ulong,
    mut sec: *mut Section,
    mut sym_index: std::os::raw::c_int,
) {
    let mut s: *mut debug_sym = 0 as *mut debug_sym;
    if !debug_info.is_null() {
        (*debug_info).sym = tcc_realloc(
            (*debug_info).sym as *mut std::os::raw::c_void,
            (::std::mem::size_of::<debug_sym>() as std::os::raw::c_ulong).wrapping_mul(
                ((*debug_info).n_sym + 1 as std::os::raw::c_int) as std::os::raw::c_ulong,
            ),
        ) as *mut debug_sym;
        let fresh0 = (*debug_info).n_sym;
        (*debug_info).n_sym = (*debug_info).n_sym + 1;
        s = (*debug_info).sym.offset(fresh0 as isize);
        (*s).type_0 = type_0;
        (*s).value = value;
        (*s).str_0 = tcc_strdup(str);
        (*s).sec = sec;
        (*s).sym_index = sym_index
    } else if !sec.is_null() {
        put_stabs_r(
            s1,
            str,
            type_0,
            0 as std::os::raw::c_int,
            0 as std::os::raw::c_int,
            value,
            sec,
            sym_index,
        );
    } else {
        put_stabs(
            s1,
            str,
            type_0,
            0 as std::os::raw::c_int,
            0 as std::os::raw::c_int,
            value,
        );
    };
}
unsafe extern "C" fn tcc_debug_stabn(
    mut s1: *mut TCCState,
    mut type_0: std::os::raw::c_int,
    mut value: std::os::raw::c_int,
) {
    if (*s1).do_debug == 0 {
        return;
    }
    if type_0 == N_LBRAC as std::os::raw::c_int {
        let mut info: *mut debug_info =
            tcc_mallocz(::std::mem::size_of::<debug_info>() as std::os::raw::c_ulong)
                as *mut debug_info;
        (*info).start = value;
        (*info).parent = debug_info;
        if !debug_info.is_null() {
            if !(*debug_info).child.is_null() {
                if !(*(*debug_info).child).last.is_null() {
                    (*(*(*debug_info).child).last).next = info
                } else {
                    (*(*debug_info).child).next = info
                }
                (*(*debug_info).child).last = info
            } else {
                (*debug_info).child = info
            }
        } else {
            debug_info_root = info
        }
        debug_info = info
    } else {
        (*debug_info).end = value;
        debug_info = (*debug_info).parent
    };
}
unsafe extern "C" fn tcc_get_debug_info(
    mut s1: *mut TCCState,
    mut s: *mut Sym,
    mut result: *mut CString,
) {
    let mut type_0: std::os::raw::c_int = 0;
    let mut n: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut debug_type: std::os::raw::c_int = -(1 as std::os::raw::c_int);
    let mut t: *mut Sym = s;
    let mut str: CString = CString {
        size: 0,
        data: 0 as *const std::os::raw::c_void as *mut std::os::raw::c_void,
        size_allocated: 0,
    };
    loop {
        type_0 = (*t).type_0.t
            & !(0x1000 as std::os::raw::c_int
                | 0x2000 as std::os::raw::c_int
                | 0x4000 as std::os::raw::c_int
                | 0x8000 as std::os::raw::c_int
                | 0x100 as std::os::raw::c_int
                | 0x200 as std::os::raw::c_int);
        if type_0 & 0xf as std::os::raw::c_int != 1 as std::os::raw::c_int {
            type_0 &= !(0x20 as std::os::raw::c_int)
        }
        if !(type_0 == 5 as std::os::raw::c_int
            || type_0 == 5 as std::os::raw::c_int | 0x40 as std::os::raw::c_int)
        {
            break;
        }
        n += 1;
        t = (*t).type_0.ref_0
    }
    if type_0 & 0xf as std::os::raw::c_int == 7 as std::os::raw::c_int {
        let mut i: std::os::raw::c_int = 0;
        t = (*t).type_0.ref_0;
        i = 0 as std::os::raw::c_int;
        while i < n_debug_hash {
            if t == (*debug_hash.offset(i as isize)).type_0 {
                debug_type = (*debug_hash.offset(i as isize)).debug_type;
                break;
            } else {
                i += 1
            }
        }
        if debug_type == -(1 as std::os::raw::c_int) {
            debug_next_type += 1;
            debug_type = debug_next_type;
            debug_hash = tcc_realloc(
                debug_hash as *mut std::os::raw::c_void,
                ((n_debug_hash + 1 as std::os::raw::c_int) as std::os::raw::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<debug_hash>() as std::os::raw::c_ulong),
            ) as *mut debug_hash;
            (*debug_hash.offset(n_debug_hash as isize)).debug_type = debug_type;
            let fresh1 = n_debug_hash;
            n_debug_hash = n_debug_hash + 1;
            let ref mut fresh2 = (*debug_hash.offset(fresh1 as isize)).type_0;
            *fresh2 = t;
            cstr_new(&mut str);
            cstr_printf(
                &mut str as *mut CString,
                b"%s:T%d=%c%d\x00" as *const u8 as *const std::os::raw::c_char,
                if (*t).v & !(0x40000000 as std::os::raw::c_int)
                    >= 0x10000000 as std::os::raw::c_int
                {
                    b"\x00" as *const u8 as *const std::os::raw::c_char
                } else {
                    get_tok_str(
                        (*t).v & !(0x40000000 as std::os::raw::c_int),
                        0 as *mut CValue,
                    )
                },
                debug_type,
                if (*t).type_0.t as std::os::raw::c_uint
                    & (((1 as std::os::raw::c_uint)
                        << 6 as std::os::raw::c_int + 6 as std::os::raw::c_int)
                        .wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_uint)
                        << 20 as std::os::raw::c_int
                        | 0x80 as std::os::raw::c_int as std::os::raw::c_uint
                        | 0xf as std::os::raw::c_int as std::os::raw::c_uint)
                    == ((1 as std::os::raw::c_int) << 20 as std::os::raw::c_int
                        | 7 as std::os::raw::c_int) as std::os::raw::c_uint
                {
                    'u' as i32
                } else {
                    's' as i32
                },
                (*t).c2rust_unnamed.c2rust_unnamed.c,
            );
            while !(*t).c2rust_unnamed_0.next.is_null() {
                let mut pos: std::os::raw::c_int = 0;
                let mut size: std::os::raw::c_int = 0;
                let mut align: std::os::raw::c_int = 0;
                t = (*t).c2rust_unnamed_0.next;
                cstr_printf(
                    &mut str as *mut CString,
                    b"%s:\x00" as *const u8 as *const std::os::raw::c_char,
                    if (*t).v & !(0x20000000 as std::os::raw::c_int)
                        >= 0x10000000 as std::os::raw::c_int
                    {
                        b"\x00" as *const u8 as *const std::os::raw::c_char
                    } else {
                        get_tok_str(
                            (*t).v & !(0x20000000 as std::os::raw::c_int),
                            0 as *mut CValue,
                        )
                    },
                );
                tcc_get_debug_info(s1, t, &mut str);
                if (*t).type_0.t & 0x80 as std::os::raw::c_int != 0 {
                    pos = (*t).c2rust_unnamed.c2rust_unnamed.c * 8 as std::os::raw::c_int
                        + ((*t).type_0.t >> 20 as std::os::raw::c_int
                            & 0x3f as std::os::raw::c_int);
                    size = (*t).type_0.t >> 20 as std::os::raw::c_int + 6 as std::os::raw::c_int
                        & 0x3f as std::os::raw::c_int
                } else {
                    pos = (*t).c2rust_unnamed.c2rust_unnamed.c * 8 as std::os::raw::c_int;
                    size = type_size(&mut (*t).type_0, &mut align) * 8 as std::os::raw::c_int
                }
                cstr_printf(
                    &mut str as *mut CString,
                    b",%d,%d;\x00" as *const u8 as *const std::os::raw::c_char,
                    pos,
                    size,
                );
            }
            cstr_printf(
                &mut str as *mut CString,
                b";\x00" as *const u8 as *const std::os::raw::c_char,
            );
            tcc_debug_stabs(
                s1,
                str.data as *const std::os::raw::c_char,
                N_LSYM as std::os::raw::c_int,
                0 as std::os::raw::c_int as std::os::raw::c_ulong,
                0 as *mut Section,
                0 as std::os::raw::c_int,
            );
            cstr_free(&mut str);
        }
    } else if type_0 as std::os::raw::c_uint
        & (((1 as std::os::raw::c_uint) << 6 as std::os::raw::c_int + 6 as std::os::raw::c_int)
            .wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_uint)
            << 20 as std::os::raw::c_int
            | 0x80 as std::os::raw::c_int as std::os::raw::c_uint)
        == ((2 as std::os::raw::c_int) << 20 as std::os::raw::c_int) as std::os::raw::c_uint
    {
        t = (*t).type_0.ref_0;
        let mut e: *mut Sym = t;
        debug_next_type += 1;
        debug_type = debug_next_type;
        cstr_new(&mut str);
        cstr_printf(
            &mut str as *mut CString,
            b"%s:T%d=e\x00" as *const u8 as *const std::os::raw::c_char,
            if (*t).v & !(0x40000000 as std::os::raw::c_int) >= 0x10000000 as std::os::raw::c_int {
                b"\x00" as *const u8 as *const std::os::raw::c_char
            } else {
                get_tok_str(
                    (*t).v & !(0x40000000 as std::os::raw::c_int),
                    0 as *mut CValue,
                )
            },
            debug_type,
        );
        while !(*t).c2rust_unnamed_0.next.is_null() {
            t = (*t).c2rust_unnamed_0.next;
            cstr_printf(
                &mut str as *mut CString,
                b"%s:\x00" as *const u8 as *const std::os::raw::c_char,
                if (*t).v & !(0x20000000 as std::os::raw::c_int)
                    >= 0x10000000 as std::os::raw::c_int
                {
                    b"\x00" as *const u8 as *const std::os::raw::c_char
                } else {
                    get_tok_str(
                        (*t).v & !(0x20000000 as std::os::raw::c_int),
                        0 as *mut CValue,
                    )
                },
            );
            cstr_printf(
                &mut str as *mut CString,
                if (*e).type_0.t & 0x10 as std::os::raw::c_int != 0 {
                    b"%u,\x00" as *const u8 as *const std::os::raw::c_char
                } else {
                    b"%d,\x00" as *const u8 as *const std::os::raw::c_char
                },
                (*t).c2rust_unnamed.enum_val as std::os::raw::c_int,
            );
        }
        cstr_printf(
            &mut str as *mut CString,
            b";\x00" as *const u8 as *const std::os::raw::c_char,
        );
        tcc_debug_stabs(
            s1,
            str.data as *const std::os::raw::c_char,
            N_LSYM as std::os::raw::c_int,
            0 as std::os::raw::c_int as std::os::raw::c_ulong,
            0 as *mut Section,
            0 as std::os::raw::c_int,
        );
        cstr_free(&mut str);
    } else if type_0 & 0xf as std::os::raw::c_int != 6 as std::os::raw::c_int {
        type_0 = (type_0 as std::os::raw::c_uint
            & !(((1 as std::os::raw::c_uint)
                << 6 as std::os::raw::c_int + 6 as std::os::raw::c_int)
                .wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_uint)
                << 20 as std::os::raw::c_int
                | 0x80 as std::os::raw::c_int as std::os::raw::c_uint))
            as std::os::raw::c_int;
        debug_type = 1 as std::os::raw::c_int;
        while debug_type as std::os::raw::c_ulong
            <= (::std::mem::size_of::<[C2RustUnnamed_10; 27]>() as std::os::raw::c_ulong)
                .wrapping_div(::std::mem::size_of::<C2RustUnnamed_10>() as std::os::raw::c_ulong)
        {
            if default_debug[(debug_type - 1 as std::os::raw::c_int) as usize].type_0 == type_0 {
                break;
            }
            debug_type += 1
        }
        if debug_type as std::os::raw::c_ulong
            > (::std::mem::size_of::<[C2RustUnnamed_10; 27]>() as std::os::raw::c_ulong)
                .wrapping_div(::std::mem::size_of::<C2RustUnnamed_10>() as std::os::raw::c_ulong)
        {
            return;
        }
    }
    if n > 0 as std::os::raw::c_int {
        debug_next_type += 1;
        cstr_printf(
            result,
            b"%d=\x00" as *const u8 as *const std::os::raw::c_char,
            debug_next_type,
        );
    }
    t = s;
    loop {
        type_0 = (*t).type_0.t
            & !(0x1000 as std::os::raw::c_int
                | 0x2000 as std::os::raw::c_int
                | 0x4000 as std::os::raw::c_int
                | 0x8000 as std::os::raw::c_int
                | 0x100 as std::os::raw::c_int
                | 0x200 as std::os::raw::c_int);
        if type_0 & 0xf as std::os::raw::c_int != 1 as std::os::raw::c_int {
            type_0 &= !(0x20 as std::os::raw::c_int)
        }
        if type_0 == 5 as std::os::raw::c_int {
            debug_next_type += 1;
            cstr_printf(
                result,
                b"%d=*\x00" as *const u8 as *const std::os::raw::c_char,
                debug_next_type,
            );
        } else if type_0 == 5 as std::os::raw::c_int | 0x40 as std::os::raw::c_int {
            debug_next_type += 1;
            cstr_printf(
                result,
                b"%d=ar1;0;%d;\x00" as *const u8 as *const std::os::raw::c_char,
                debug_next_type,
                (*(*t).type_0.ref_0).c2rust_unnamed.c2rust_unnamed.c - 1 as std::os::raw::c_int,
            );
        } else {
            if !(type_0 == 6 as std::os::raw::c_int) {
                break;
            }
            debug_next_type += 1;
            cstr_printf(
                result,
                b"%d=f\x00" as *const u8 as *const std::os::raw::c_char,
                debug_next_type,
            );
            tcc_get_debug_info(s1, (*t).type_0.ref_0, result);
            return;
        }
        t = (*t).type_0.ref_0
    }
    cstr_printf(
        result,
        b"%d\x00" as *const u8 as *const std::os::raw::c_char,
        debug_type,
    );
}
unsafe extern "C" fn tcc_debug_finish(mut s1: *mut TCCState, mut cur: *mut debug_info) {
    while !cur.is_null() {
        let mut i: std::os::raw::c_int = 0;
        let mut next_0: *mut debug_info = (*cur).next;
        i = 0 as std::os::raw::c_int;
        while i < (*cur).n_sym {
            let mut s: *mut debug_sym = &mut *(*cur).sym.offset(i as isize) as *mut debug_sym;
            if !(*s).sec.is_null() {
                put_stabs_r(
                    s1,
                    (*s).str_0,
                    (*s).type_0,
                    0 as std::os::raw::c_int,
                    0 as std::os::raw::c_int,
                    (*s).value,
                    (*s).sec,
                    (*s).sym_index,
                );
            } else {
                put_stabs(
                    s1,
                    (*s).str_0,
                    (*s).type_0,
                    0 as std::os::raw::c_int,
                    0 as std::os::raw::c_int,
                    (*s).value,
                );
            }
            tcc_free((*s).str_0 as *mut std::os::raw::c_void);
            i += 1
        }
        tcc_free((*cur).sym as *mut std::os::raw::c_void);
        put_stabn(
            s1,
            N_LBRAC as std::os::raw::c_int,
            0 as std::os::raw::c_int,
            0 as std::os::raw::c_int,
            (*cur).start,
        );
        tcc_debug_finish(s1, (*cur).child);
        put_stabn(
            s1,
            N_RBRAC as std::os::raw::c_int,
            0 as std::os::raw::c_int,
            0 as std::os::raw::c_int,
            (*cur).end,
        );
        tcc_free(cur as *mut std::os::raw::c_void);
        cur = next_0
    }
}
unsafe extern "C" fn tcc_add_debug_info(
    mut s1: *mut TCCState,
    mut param: std::os::raw::c_int,
    mut s: *mut Sym,
    mut e: *mut Sym,
) {
    let mut debug_str: CString = CString {
        size: 0,
        data: 0 as *const std::os::raw::c_void as *mut std::os::raw::c_void,
        size_allocated: 0,
    };
    if (*s1).do_debug == 0 {
        return;
    }
    cstr_new(&mut debug_str);
    while s != e {
        if !((*s).v == 0
            || (*s).r as std::os::raw::c_int & 0x3f as std::os::raw::c_int
                != 0x32 as std::os::raw::c_int)
        {
            cstr_reset(&mut debug_str);
            cstr_printf(
                &mut debug_str as *mut CString,
                b"%s:%s\x00" as *const u8 as *const std::os::raw::c_char,
                get_tok_str((*s).v, 0 as *mut CValue),
                if param != 0 {
                    b"p\x00" as *const u8 as *const std::os::raw::c_char
                } else {
                    b"\x00" as *const u8 as *const std::os::raw::c_char
                },
            );
            tcc_get_debug_info(s1, s, &mut debug_str);
            tcc_debug_stabs(
                s1,
                debug_str.data as *const std::os::raw::c_char,
                if param != 0 {
                    N_PSYM as std::os::raw::c_int
                } else {
                    N_LSYM as std::os::raw::c_int
                },
                (*s).c2rust_unnamed.c2rust_unnamed.c as std::os::raw::c_ulong,
                0 as *mut Section,
                0 as std::os::raw::c_int,
            );
        }
        s = (*s).prev
    }
    cstr_free(&mut debug_str);
}
/* put function symbol */
unsafe extern "C" fn tcc_debug_funcstart(mut s1: *mut TCCState, mut sym: *mut Sym) {
    let mut debug_str: CString = CString {
        size: 0,
        data: 0 as *const std::os::raw::c_void as *mut std::os::raw::c_void,
        size_allocated: 0,
    };
    let mut f: *mut BufferedFile = 0 as *mut BufferedFile;
    if (*s1).do_debug == 0 {
        return;
    }
    debug_info_root = 0 as *mut debug_info;
    debug_info = 0 as *mut debug_info;
    tcc_debug_stabn(s1, N_LBRAC as std::os::raw::c_int, ind - func_ind);
    f = put_new_file(s1);
    if f.is_null() {
        return;
    }
    cstr_new(&mut debug_str);
    cstr_printf(
        &mut debug_str as *mut CString,
        b"%s:%c\x00" as *const u8 as *const std::os::raw::c_char,
        funcname,
        if (*sym).type_0.t & 0x2000 as std::os::raw::c_int != 0 {
            'f' as i32
        } else {
            'F' as i32
        },
    );
    tcc_get_debug_info(s1, (*sym).type_0.ref_0, &mut debug_str);
    put_stabs_r(
        s1,
        debug_str.data as *const std::os::raw::c_char,
        N_FUN as std::os::raw::c_int,
        0 as std::os::raw::c_int,
        (*f).line_num,
        0 as std::os::raw::c_int as std::os::raw::c_ulong,
        (*tcc_state).cur_text_section,
        (*sym).c2rust_unnamed.c2rust_unnamed.c,
    );
    cstr_free(&mut debug_str);
    tcc_debug_line(s1);
}
/* put function size */
unsafe extern "C" fn tcc_debug_funcend(mut s1: *mut TCCState, mut size: std::os::raw::c_int) {
    if (*s1).do_debug == 0 {
        return;
    }
    tcc_debug_stabn(s1, N_RBRAC as std::os::raw::c_int, size);
    tcc_debug_finish(s1, debug_info_root);
}
unsafe extern "C" fn tcc_debug_extern_sym(
    mut s1: *mut TCCState,
    mut sym: *mut Sym,
    mut sh_num: std::os::raw::c_int,
    mut sym_bind: std::os::raw::c_int,
    mut sym_type: std::os::raw::c_int,
) {
    let mut s: *mut Section = 0 as *mut Section;
    let mut str: CString = CString {
        size: 0,
        data: 0 as *const std::os::raw::c_void as *mut std::os::raw::c_void,
        size_allocated: 0,
    };
    if (*s1).do_debug == 0 {
        return;
    }
    if sym_type == 2 as std::os::raw::c_int || (*sym).v >= 0x10000000 as std::os::raw::c_int {
        return;
    }
    s = *(*s1).sections.offset(sh_num as isize);
    cstr_new(&mut str);
    cstr_printf(
        &mut str as *mut CString,
        b"%s:%c\x00" as *const u8 as *const std::os::raw::c_char,
        get_tok_str((*sym).v, 0 as *mut CValue),
        if sym_bind == 1 as std::os::raw::c_int {
            'G' as i32
        } else if local_scope != 0 {
            'V' as i32
        } else {
            'S' as i32
        },
    );
    tcc_get_debug_info(s1, sym, &mut str);
    if sym_bind == 1 as std::os::raw::c_int {
        tcc_debug_stabs(
            s1,
            str.data as *const std::os::raw::c_char,
            N_GSYM as std::os::raw::c_int,
            0 as std::os::raw::c_int as std::os::raw::c_ulong,
            0 as *mut Section,
            0 as std::os::raw::c_int,
        );
    } else {
        tcc_debug_stabs(
            s1,
            str.data as *const std::os::raw::c_char,
            if (*sym).type_0.t & 0x2000 as std::os::raw::c_int != 0
                && (*tcc_state).data_section == s
            {
                N_STSYM as std::os::raw::c_int
            } else {
                N_LCSYM as std::os::raw::c_int
            },
            0 as std::os::raw::c_int as std::os::raw::c_ulong,
            s,
            (*sym).c2rust_unnamed.c2rust_unnamed.c,
        );
    }
    cstr_free(&mut str);
}
unsafe extern "C" fn tcc_debug_typedef(mut s1: *mut TCCState, mut sym: *mut Sym) {
    let mut str: CString = CString {
        size: 0,
        data: 0 as *const std::os::raw::c_void as *mut std::os::raw::c_void,
        size_allocated: 0,
    };
    if (*s1).do_debug == 0 {
        return;
    }
    cstr_new(&mut str);
    cstr_printf(
        &mut str as *mut CString,
        b"%s:t\x00" as *const u8 as *const std::os::raw::c_char,
        if (*sym).v & !(0x20000000 as std::os::raw::c_int) >= 0x10000000 as std::os::raw::c_int {
            b"\x00" as *const u8 as *const std::os::raw::c_char
        } else {
            get_tok_str(
                (*sym).v & !(0x20000000 as std::os::raw::c_int),
                0 as *mut CValue,
            )
        },
    );
    tcc_get_debug_info(s1, sym, &mut str);
    tcc_debug_stabs(
        s1,
        str.data as *const std::os::raw::c_char,
        N_LSYM as std::os::raw::c_int,
        0 as std::os::raw::c_int as std::os::raw::c_ulong,
        0 as *mut Section,
        0 as std::os::raw::c_int,
    );
    cstr_free(&mut str);
}
/* only static data output */
/* Automagical code suppression ----> */
unsafe extern "C" fn tcc_tcov_block_begin() {
    let mut sv: SValue = SValue {
        type_0: CType {
            t: 0,
            ref_0: 0 as *const Sym as *mut Sym,
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
    let mut ptr: *mut std::os::raw::c_void = 0 as *mut std::os::raw::c_void;
    let mut last_offset: std::os::raw::c_ulong = tcov_data.offset;
    tcc_tcov_block_end(0 as std::os::raw::c_int);
    if (*tcc_state).test_coverage as std::os::raw::c_int == 0 as std::os::raw::c_int
        || nocode_wanted != 0
    {
        return;
    }
    if tcov_data.last_file_name == 0 as std::os::raw::c_int as std::os::raw::c_ulong
        || strcmp(
            (*(*tcc_state).tcov_section)
                .data
                .offset(tcov_data.last_file_name as isize)
                as *const std::os::raw::c_char,
            (*file).true_filename,
        ) != 0 as std::os::raw::c_int
    {
        let mut wd: [std::os::raw::c_char; 1024] = [0; 1024];
        let mut cstr: CString = CString {
            size: 0,
            data: 0 as *const std::os::raw::c_void as *mut std::os::raw::c_void,
            size_allocated: 0,
        };
        if tcov_data.last_func_name != 0 {
            section_ptr_add(
                (*tcc_state).tcov_section,
                1 as std::os::raw::c_int as Elf64_Addr,
            );
        }
        if tcov_data.last_file_name != 0 {
            section_ptr_add(
                (*tcc_state).tcov_section,
                1 as std::os::raw::c_int as Elf64_Addr,
            );
        }
        tcov_data.last_func_name = 0 as std::os::raw::c_int as std::os::raw::c_ulong;
        cstr_new(&mut cstr);
        if *(*file)
            .true_filename
            .offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int
            == '/' as i32
        {
            tcov_data.last_file_name = (*(*tcc_state).tcov_section).data_offset;
            cstr_printf(
                &mut cstr as *mut CString,
                b"%s\x00" as *const u8 as *const std::os::raw::c_char,
                (*file).true_filename,
            );
        } else {
            getcwd(
                wd.as_mut_ptr(),
                ::std::mem::size_of::<[std::os::raw::c_char; 1024]>() as std::os::raw::c_ulong,
            );
            tcov_data.last_file_name = (*(*tcc_state).tcov_section)
                .data_offset
                .wrapping_add(strlen(wd.as_mut_ptr()))
                .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong);
            cstr_printf(
                &mut cstr as *mut CString,
                b"%s/%s\x00" as *const u8 as *const std::os::raw::c_char,
                wd.as_mut_ptr(),
                (*file).true_filename,
            );
        }
        ptr = section_ptr_add(
            (*tcc_state).tcov_section,
            (cstr.size + 1 as std::os::raw::c_int) as Elf64_Addr,
        );
        strncpy(
            ptr as *mut std::os::raw::c_char,
            cstr.data as *const std::os::raw::c_char,
            cstr.size as std::os::raw::c_ulong,
        );
        cstr_free(&mut cstr);
    }
    if tcov_data.last_func_name == 0 as std::os::raw::c_int as std::os::raw::c_ulong
        || strcmp(
            (*(*tcc_state).tcov_section)
                .data
                .offset(tcov_data.last_func_name as isize)
                as *const std::os::raw::c_char,
            funcname,
        ) != 0 as std::os::raw::c_int
    {
        let mut len: size_t = 0;
        if tcov_data.last_func_name != 0 {
            section_ptr_add(
                (*tcc_state).tcov_section,
                1 as std::os::raw::c_int as Elf64_Addr,
            );
        }
        tcov_data.last_func_name = (*(*tcc_state).tcov_section).data_offset;
        len = strlen(funcname);
        ptr = section_ptr_add(
            (*tcc_state).tcov_section,
            len.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong),
        );
        strncpy(ptr as *mut std::os::raw::c_char, funcname, len);
        section_ptr_add(
            (*tcc_state).tcov_section,
            (*(*tcc_state).tcov_section).data_offset.wrapping_neg()
                & 7 as std::os::raw::c_int as std::os::raw::c_ulong,
        );
        ptr = section_ptr_add(
            (*tcc_state).tcov_section,
            8 as std::os::raw::c_int as Elf64_Addr,
        );
        write64le(
            ptr as *mut std::os::raw::c_uchar,
            (*file).line_num as uint64_t,
        );
    }
    if ind == tcov_data.ind && tcov_data.line == (*file).line_num {
        tcov_data.offset = last_offset
    } else {
        let mut label: Sym = {
            let mut init = Sym {
                v: 0 as std::os::raw::c_int,
                r: 0,
                a: SymAttr([0; 2]),
                c2rust_unnamed: C2RustUnnamed_0 {
                    c2rust_unnamed: C2RustUnnamed_1 {
                        c: 0,
                        c2rust_unnamed: C2RustUnnamed_2 { sym_scope: 0 },
                    },
                },
                type_0: CType {
                    t: 0,
                    ref_0: 0 as *const Sym as *mut Sym,
                },
                c2rust_unnamed_0: C2RustUnnamed {
                    next: 0 as *mut Sym,
                },
                prev: 0 as *mut Sym,
                prev_tok: 0 as *mut Sym,
            };
            init
        };
        label.type_0.t = 4 as std::os::raw::c_int | 0x2000 as std::os::raw::c_int;
        ptr = section_ptr_add(
            (*tcc_state).tcov_section,
            16 as std::os::raw::c_int as Elf64_Addr,
        );
        tcov_data.line = (*file).line_num;
        write64le(
            ptr as *mut std::os::raw::c_uchar,
            (tcov_data.line << 8 as std::os::raw::c_int | 0xff as std::os::raw::c_int) as uint64_t,
        );
        put_extern_sym(
            &mut label,
            (*tcc_state).tcov_section,
            ((ptr as *mut std::os::raw::c_uchar).offset_from((*(*tcc_state).tcov_section).data)
                as std::os::raw::c_long
                + 8 as std::os::raw::c_int as std::os::raw::c_long) as Elf64_Addr,
            0 as std::os::raw::c_int as std::os::raw::c_ulong,
        );
        sv.type_0 = label.type_0;
        sv.r = (0x200 as std::os::raw::c_int
            | 0x100 as std::os::raw::c_int
            | 0x30 as std::os::raw::c_int) as std::os::raw::c_ushort;
        sv.r2 = 0x30 as std::os::raw::c_int as std::os::raw::c_ushort;
        sv.c2rust_unnamed.c.i = 0 as std::os::raw::c_int as uint64_t;
        sv.c2rust_unnamed_0.sym = &mut label;
        gen_increment_tcov(&mut sv);
        tcov_data.offset = (ptr as *mut std::os::raw::c_uchar)
            .offset_from((*(*tcc_state).tcov_section).data)
            as std::os::raw::c_long as std::os::raw::c_ulong;
        tcov_data.ind = ind
    };
}
/* ------------------------------------------------------------------------- */
/* for section layout see lib/tcov.c */
unsafe extern "C" fn tcc_tcov_block_end(mut line: std::os::raw::c_int) {
    if (*tcc_state).test_coverage as std::os::raw::c_int == 0 as std::os::raw::c_int {
        return;
    }
    if tcov_data.offset != 0 {
        let mut ptr: *mut std::os::raw::c_void = (*(*tcc_state).tcov_section)
            .data
            .offset(tcov_data.offset as isize)
            as *mut std::os::raw::c_void;
        let mut nline: std::os::raw::c_ulonglong =
            if line != 0 { line } else { (*file).line_num } as std::os::raw::c_ulonglong;
        write64le(
            ptr as *mut std::os::raw::c_uchar,
            (read64le(ptr as *mut std::os::raw::c_uchar) as std::os::raw::c_ulonglong
                & 0xfffffffff as std::os::raw::c_ulonglong
                | nline << 36 as std::os::raw::c_int) as uint64_t,
        );
        tcov_data.offset = 0 as std::os::raw::c_int as std::os::raw::c_ulong
    };
}
unsafe extern "C" fn tcc_tcov_check_line(mut start: std::os::raw::c_int) {
    if (*tcc_state).test_coverage as std::os::raw::c_int == 0 as std::os::raw::c_int {
        return;
    }
    if tcov_data.line != (*file).line_num {
        if tcov_data.line + 1 as std::os::raw::c_int != (*file).line_num {
            tcc_tcov_block_end(tcov_data.line);
            if start != 0 {
                tcc_tcov_block_begin();
            }
        } else {
            tcov_data.line = (*file).line_num
        }
    };
}
unsafe extern "C" fn tcc_tcov_start() {
    if (*tcc_state).test_coverage as std::os::raw::c_int == 0 as std::os::raw::c_int {
        return;
    }
    memset(
        &mut tcov_data as *mut C2RustUnnamed_11 as *mut std::os::raw::c_void,
        0 as std::os::raw::c_int,
        ::std::mem::size_of::<C2RustUnnamed_11>() as std::os::raw::c_ulong,
    );
    if (*tcc_state).tcov_section.is_null() {
        (*tcc_state).tcov_section = new_section(
            tcc_state,
            b".tcov\x00" as *const u8 as *const std::os::raw::c_char,
            1 as std::os::raw::c_int,
            (1 as std::os::raw::c_int) << 1 as std::os::raw::c_int
                | (1 as std::os::raw::c_int) << 0 as std::os::raw::c_int,
        );
        section_ptr_add(
            (*tcc_state).tcov_section,
            4 as std::os::raw::c_int as Elf64_Addr,
        );
        // pointer to executable name
    };
}
unsafe extern "C" fn tcc_tcov_end() {
    if (*tcc_state).test_coverage as std::os::raw::c_int == 0 as std::os::raw::c_int {
        return;
    }
    if tcov_data.last_func_name != 0 {
        section_ptr_add(
            (*tcc_state).tcov_section,
            1 as std::os::raw::c_int as Elf64_Addr,
        );
    }
    if tcov_data.last_file_name != 0 {
        section_ptr_add(
            (*tcc_state).tcov_section,
            1 as std::os::raw::c_int as Elf64_Addr,
        );
    };
}
/* ------------------------------------------------------------------------- */
/* initialize vstack and types.  This must be done also for tcc -E */
#[no_mangle]
pub unsafe extern "C" fn tccgen_init(mut s1: *mut TCCState) {
    vtop = _vstack
        .as_mut_ptr()
        .offset(1 as std::os::raw::c_int as isize)
        .offset(-(1 as std::os::raw::c_int as isize));
    memset(
        vtop as *mut std::os::raw::c_void,
        0 as std::os::raw::c_int,
        ::std::mem::size_of::<SValue>() as std::os::raw::c_ulong,
    );
    /* define some often used types */
    int_type.t = 3 as std::os::raw::c_int;
    char_type.t = 1 as std::os::raw::c_int;
    if (*s1).char_is_unsigned != 0 {
        char_type.t |= 0x10 as std::os::raw::c_int
    }
    char_pointer_type = char_type;
    mk_pointer(&mut char_pointer_type);
    func_old_type.t = 6 as std::os::raw::c_int;
    func_old_type.ref_0 = sym_push(
        0x20000000 as std::os::raw::c_int,
        &mut int_type,
        0 as std::os::raw::c_int,
        0 as std::os::raw::c_int,
    );
    (*func_old_type.ref_0)
        .c2rust_unnamed
        .c2rust_unnamed
        .c2rust_unnamed
        .f
        .set_func_call(0 as std::os::raw::c_int as std::os::raw::c_uint);
    (*func_old_type.ref_0)
        .c2rust_unnamed
        .c2rust_unnamed
        .c2rust_unnamed
        .f
        .set_func_type(2 as std::os::raw::c_int as std::os::raw::c_uint);
    init_prec();
    cstr_new(&mut initstr);
}
#[no_mangle]
pub unsafe extern "C" fn tccgen_compile(mut s1: *mut TCCState) -> std::os::raw::c_int {
    (*tcc_state).cur_text_section = 0 as *mut Section;
    funcname = b"\x00" as *const u8 as *const std::os::raw::c_char;
    anon_sym = 0x10000000 as std::os::raw::c_int;
    section_sym = 0 as std::os::raw::c_int;
    const_wanted = 0 as std::os::raw::c_int;
    nocode_wanted = 0x80000000 as std::os::raw::c_uint as std::os::raw::c_int;
    local_scope = 0 as std::os::raw::c_int;
    debug_modes = ((*s1).do_debug as std::os::raw::c_int
        | ((*s1).test_coverage as std::os::raw::c_int) << 1 as std::os::raw::c_int)
        as std::os::raw::c_char;
    tcc_debug_start(s1);
    tcc_tcov_start();
    parse_flags =
        0x1 as std::os::raw::c_int | 0x2 as std::os::raw::c_int | 0x40 as std::os::raw::c_int;
    next();
    decl(0x30 as std::os::raw::c_int);
    gen_inline_functions(s1);
    check_vstack();
    /* end of translation unit info */
    tcc_debug_end(s1);
    tcc_tcov_end();
    return 0 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn tccgen_finish(mut s1: *mut TCCState) {
    cstr_free(&mut initstr);
    free_inline_functions(s1);
    sym_pop(&mut global_stack, 0 as *mut Sym, 0 as std::os::raw::c_int);
    sym_pop(&mut local_stack, 0 as *mut Sym, 0 as std::os::raw::c_int);
    /* free preprocessor macros */
    free_defines(0 as *mut Sym);
    /* free sym_pools */
    dynarray_reset(
        &mut sym_pools as *mut *mut *mut std::os::raw::c_void as *mut std::os::raw::c_void,
        &mut nb_sym_pools,
    );
    sym_free_first = 0 as *mut Sym;
}
/* ------------------------------------------------------------------------- */
#[no_mangle]
pub unsafe extern "C" fn elfsym(mut s: *mut Sym) -> *mut Elf64_Sym {
    if s.is_null() || (*s).c2rust_unnamed.c2rust_unnamed.c == 0 {
        return 0 as *mut Elf64_Sym;
    }
    return &mut *((*(*tcc_state).symtab_section).data as *mut Elf64_Sym)
        .offset((*s).c2rust_unnamed.c2rust_unnamed.c as isize) as *mut Elf64_Sym;
}
/* apply storage attributes to Elf symbol */
#[no_mangle]
pub unsafe extern "C" fn update_storage(mut sym: *mut Sym) {
    let mut esym: *mut Elf64_Sym = 0 as *mut Elf64_Sym;
    let mut sym_bind: std::os::raw::c_int = 0;
    let mut old_sym_bind: std::os::raw::c_int = 0;
    esym = elfsym(sym);
    if esym.is_null() {
        return;
    }
    if (*sym).a.visibility() != 0 {
        (*esym).st_other = ((*esym).st_other as std::os::raw::c_int
            & !(-(1 as std::os::raw::c_int) & 0x3 as std::os::raw::c_int)
            | (*sym).a.visibility() as std::os::raw::c_int)
            as std::os::raw::c_uchar
    }
    if (*sym).type_0.t & (0x2000 as std::os::raw::c_int | 0x8000 as std::os::raw::c_int) != 0 {
        sym_bind = 0 as std::os::raw::c_int
    } else if (*sym).a.weak() != 0 {
        sym_bind = 2 as std::os::raw::c_int
    } else {
        sym_bind = 1 as std::os::raw::c_int
    }
    old_sym_bind = (*esym).st_info as std::os::raw::c_int >> 4 as std::os::raw::c_int;
    if sym_bind != old_sym_bind {
        (*esym).st_info = ((sym_bind << 4 as std::os::raw::c_int)
            + ((*esym).st_info as std::os::raw::c_int
                & 0xf as std::os::raw::c_int
                & 0xf as std::os::raw::c_int)) as std::os::raw::c_uchar
    };
}
/* ------------------------------------------------------------------------- */
/* update sym->c so that it points to an external symbol in section
'section' with value 'value' */
#[no_mangle]
pub unsafe extern "C" fn put_extern_sym2(
    mut sym: *mut Sym,
    mut sh_num: std::os::raw::c_int,
    mut value: Elf64_Addr,
    mut size: std::os::raw::c_ulong,
    mut can_add_underscore: std::os::raw::c_int,
) {
    let mut sym_type: std::os::raw::c_int = 0;
    let mut sym_bind: std::os::raw::c_int = 0;
    let mut info: std::os::raw::c_int = 0;
    let mut other: std::os::raw::c_int = 0;
    let mut t: std::os::raw::c_int = 0;
    let mut esym: *mut Elf64_Sym = 0 as *mut Elf64_Sym;
    let mut name: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut buf1: [std::os::raw::c_char; 256] = [0; 256];
    if (*sym).c2rust_unnamed.c2rust_unnamed.c == 0 {
        name = get_tok_str((*sym).v, 0 as *mut CValue);
        t = (*sym).type_0.t;
        if t & 0xf as std::os::raw::c_int == 6 as std::os::raw::c_int {
            sym_type = 2 as std::os::raw::c_int
        } else if t & 0xf as std::os::raw::c_int == 0 as std::os::raw::c_int {
            sym_type = 0 as std::os::raw::c_int;
            if t & (0xf as std::os::raw::c_int
                | (0 as std::os::raw::c_int
                    | (1 as std::os::raw::c_int) << 20 as std::os::raw::c_int
                    | (2 as std::os::raw::c_int) << 20 as std::os::raw::c_int))
                == 0 as std::os::raw::c_int
                    | (1 as std::os::raw::c_int) << 20 as std::os::raw::c_int
                    | (2 as std::os::raw::c_int) << 20 as std::os::raw::c_int
            {
                sym_type = 2 as std::os::raw::c_int
            }
        } else {
            sym_type = 1 as std::os::raw::c_int
        }
        if t & (0x2000 as std::os::raw::c_int | 0x8000 as std::os::raw::c_int) != 0 {
            sym_bind = 0 as std::os::raw::c_int
        } else {
            sym_bind = 1 as std::os::raw::c_int
        }
        other = 0 as std::os::raw::c_int;
        if (*sym).c2rust_unnamed_0.asm_label != 0 {
            name = get_tok_str((*sym).c2rust_unnamed_0.asm_label, 0 as *mut CValue);
            can_add_underscore = 0 as std::os::raw::c_int
        }
        if (*tcc_state).leading_underscore as std::os::raw::c_int != 0 && can_add_underscore != 0 {
            buf1[0 as std::os::raw::c_int as usize] = '_' as i32 as std::os::raw::c_char;
            pstrcpy(
                buf1.as_mut_ptr().offset(1 as std::os::raw::c_int as isize),
                (::std::mem::size_of::<[std::os::raw::c_char; 256]>() as std::os::raw::c_ulong)
                    .wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong),
                name,
            );
            name = buf1.as_mut_ptr()
        }
        info = (sym_bind << 4 as std::os::raw::c_int) + (sym_type & 0xf as std::os::raw::c_int);
        (*sym).c2rust_unnamed.c2rust_unnamed.c = put_elf_sym(
            (*tcc_state).symtab_section,
            value,
            size,
            info,
            other,
            sh_num,
            name,
        );
        if debug_modes != 0 {
            tcc_debug_extern_sym(tcc_state, sym, sh_num, sym_bind, sym_type);
        }
    } else {
        esym = elfsym(sym);
        (*esym).st_value = value;
        (*esym).st_size = size;
        (*esym).st_shndx = sh_num as Elf64_Section
    }
    update_storage(sym);
}
#[no_mangle]
pub unsafe extern "C" fn put_extern_sym(
    mut sym: *mut Sym,
    mut section: *mut Section,
    mut value: Elf64_Addr,
    mut size: std::os::raw::c_ulong,
) {
    let mut sh_num: std::os::raw::c_int = if !section.is_null() {
        (*section).sh_num
    } else {
        0 as std::os::raw::c_int
    };
    put_extern_sym2(sym, sh_num, value, size, 1 as std::os::raw::c_int);
}
/* add a new relocation entry to symbol 'sym' in section 's' */
#[no_mangle]
pub unsafe extern "C" fn greloca(
    mut s: *mut Section,
    mut sym: *mut Sym,
    mut offset: std::os::raw::c_ulong,
    mut type_0: std::os::raw::c_int,
    mut addend: Elf64_Addr,
) {
    let mut c: std::os::raw::c_int = 0 as std::os::raw::c_int;
    if nocode_wanted != 0 && s == (*tcc_state).cur_text_section {
        return;
    }
    if !sym.is_null() {
        if 0 as std::os::raw::c_int == (*sym).c2rust_unnamed.c2rust_unnamed.c {
            put_extern_sym(
                sym,
                0 as *mut Section,
                0 as std::os::raw::c_int as Elf64_Addr,
                0 as std::os::raw::c_int as std::os::raw::c_ulong,
            );
        }
        c = (*sym).c2rust_unnamed.c2rust_unnamed.c
    }
    /* now we can add ELF relocation info */
    put_elf_reloca((*tcc_state).symtab_section, s, offset, type_0, c, addend);
}
/* ------------------------------------------------------------------------- */
/* symbol allocator */
unsafe extern "C" fn __sym_malloc() -> *mut Sym {
    let mut sym_pool: *mut Sym = 0 as *mut Sym;
    let mut sym: *mut Sym = 0 as *mut Sym;
    let mut last_sym: *mut Sym = 0 as *mut Sym;
    let mut i: std::os::raw::c_int = 0;
    sym_pool = tcc_malloc(
        (8192 as std::os::raw::c_int as std::os::raw::c_ulong)
            .wrapping_div(::std::mem::size_of::<Sym>() as std::os::raw::c_ulong)
            .wrapping_mul(::std::mem::size_of::<Sym>() as std::os::raw::c_ulong),
    ) as *mut Sym;
    dynarray_add(
        &mut sym_pools as *mut *mut *mut std::os::raw::c_void as *mut std::os::raw::c_void,
        &mut nb_sym_pools,
        sym_pool as *mut std::os::raw::c_void,
    );
    last_sym = sym_free_first;
    sym = sym_pool;
    i = 0 as std::os::raw::c_int;
    while (i as std::os::raw::c_ulong)
        < (8192 as std::os::raw::c_int as std::os::raw::c_ulong)
            .wrapping_div(::std::mem::size_of::<Sym>() as std::os::raw::c_ulong)
    {
        (*sym).c2rust_unnamed_0.next = last_sym;
        last_sym = sym;
        sym = sym.offset(1);
        i += 1
    }
    sym_free_first = last_sym;
    return last_sym;
}
#[inline]
unsafe extern "C" fn sym_malloc() -> *mut Sym {
    let mut sym: *mut Sym = 0 as *mut Sym;
    sym = sym_free_first;
    if sym.is_null() {
        sym = __sym_malloc()
    }
    sym_free_first = (*sym).c2rust_unnamed_0.next;
    return sym;
}
#[no_mangle]
pub unsafe extern "C" fn sym_free(mut sym: *mut Sym) {
    (*sym).c2rust_unnamed_0.next = sym_free_first;
    sym_free_first = sym;
}
/* push, without hashing */
#[no_mangle]
pub unsafe extern "C" fn sym_push2(
    mut ps: *mut *mut Sym,
    mut v: std::os::raw::c_int,
    mut t: std::os::raw::c_int,
    mut c: std::os::raw::c_int,
) -> *mut Sym {
    let mut s: *mut Sym = 0 as *mut Sym;
    s = sym_malloc();
    memset(
        s as *mut std::os::raw::c_void,
        0 as std::os::raw::c_int,
        ::std::mem::size_of::<Sym>() as std::os::raw::c_ulong,
    );
    (*s).v = v;
    (*s).type_0.t = t;
    (*s).c2rust_unnamed.c2rust_unnamed.c = c;
    /* add in stack */
    (*s).prev = *ps;
    *ps = s;
    return s;
}
/* find a symbol and return its associated structure. 's' is the top
of the symbol stack */
#[no_mangle]
pub unsafe extern "C" fn sym_find2(mut s: *mut Sym, mut v: std::os::raw::c_int) -> *mut Sym {
    while !s.is_null() {
        if (*s).v == v {
            return s;
        } else {
            if (*s).v == -(1 as std::os::raw::c_int) {
                return 0 as *mut Sym;
            }
        }
        s = (*s).prev
    }
    return 0 as *mut Sym;
}
/* structure lookup */
#[no_mangle]
pub unsafe extern "C" fn struct_find(mut v: std::os::raw::c_int) -> *mut Sym {
    v -= 256 as std::os::raw::c_int;
    if v as std::os::raw::c_uint >= (tok_ident - 256 as std::os::raw::c_int) as std::os::raw::c_uint
    {
        return 0 as *mut Sym;
    }
    return (**table_ident.offset(v as isize)).sym_struct;
}
/* find an identifier */
#[no_mangle]
pub unsafe extern "C" fn sym_find(mut v: std::os::raw::c_int) -> *mut Sym {
    v -= 256 as std::os::raw::c_int;
    if v as std::os::raw::c_uint >= (tok_ident - 256 as std::os::raw::c_int) as std::os::raw::c_uint
    {
        return 0 as *mut Sym;
    }
    return (**table_ident.offset(v as isize)).sym_identifier;
}
unsafe extern "C" fn sym_scope(mut s: *mut Sym) -> std::os::raw::c_int {
    if (*s).type_0.t as std::os::raw::c_uint
        & (((1 as std::os::raw::c_uint) << 6 as std::os::raw::c_int + 6 as std::os::raw::c_int)
            .wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_uint)
            << 20 as std::os::raw::c_int
            | 0x80 as std::os::raw::c_int as std::os::raw::c_uint)
        == ((3 as std::os::raw::c_int) << 20 as std::os::raw::c_int) as std::os::raw::c_uint
    {
        return (*(*s).type_0.ref_0)
            .c2rust_unnamed
            .c2rust_unnamed
            .c2rust_unnamed
            .sym_scope;
    } else {
        return (*s).c2rust_unnamed.c2rust_unnamed.c2rust_unnamed.sym_scope;
    };
}
/* push a given symbol on the symbol stack */
#[no_mangle]
pub unsafe extern "C" fn sym_push(
    mut v: std::os::raw::c_int,
    mut type_0: *mut CType,
    mut r: std::os::raw::c_int,
    mut c: std::os::raw::c_int,
) -> *mut Sym {
    let mut s: *mut Sym = 0 as *mut Sym;
    let mut ps: *mut *mut Sym = 0 as *mut *mut Sym;
    let mut ts: *mut TokenSym = 0 as *mut TokenSym;
    if !local_stack.is_null() {
        ps = &mut local_stack
    } else {
        ps = &mut global_stack
    }
    s = sym_push2(ps, v, (*type_0).t, c);
    (*s).type_0.ref_0 = (*type_0).ref_0;
    (*s).r = r as std::os::raw::c_ushort;
    /* don't record fields or anonymous symbols */
    /* XXX: simplify */
    if v & 0x20000000 as std::os::raw::c_int == 0
        && (v & !(0x40000000 as std::os::raw::c_int)) < 0x10000000 as std::os::raw::c_int
    {
        /* record symbol in token array */
        ts = *table_ident.offset(
            ((v & !(0x40000000 as std::os::raw::c_int)) - 256 as std::os::raw::c_int) as isize,
        );
        if v & 0x40000000 as std::os::raw::c_int != 0 {
            ps = &mut (*ts).sym_struct
        } else {
            ps = &mut (*ts).sym_identifier
        }
        (*s).prev_tok = *ps;
        *ps = s;
        (*s).c2rust_unnamed.c2rust_unnamed.c2rust_unnamed.sym_scope = local_scope;
        if !(*s).prev_tok.is_null()
            && sym_scope((*s).prev_tok)
                == (*s).c2rust_unnamed.c2rust_unnamed.c2rust_unnamed.sym_scope
        {
            _tcc_error(
                b"redeclaration of \'%s\'\x00" as *const u8 as *const std::os::raw::c_char,
                get_tok_str(v & !(0x40000000 as std::os::raw::c_int), 0 as *mut CValue),
            );
        }
    }
    return s;
}
/* push a global identifier */
#[no_mangle]
pub unsafe extern "C" fn global_identifier_push(
    mut v: std::os::raw::c_int,
    mut t: std::os::raw::c_int,
    mut c: std::os::raw::c_int,
) -> *mut Sym {
    let mut s: *mut Sym = 0 as *mut Sym;
    let mut ps: *mut *mut Sym = 0 as *mut *mut Sym;
    s = sym_push2(&mut global_stack, v, t, c);
    (*s).r = (0x30 as std::os::raw::c_int | 0x200 as std::os::raw::c_int) as std::os::raw::c_ushort;
    /* don't record anonymous symbol */
    if v < 0x10000000 as std::os::raw::c_int {
        ps = &mut (**table_ident.offset((v - 256 as std::os::raw::c_int) as isize)).sym_identifier;
        /* modify the top most local identifier, so that sym_identifier will
        point to 's' when popped; happens when called from inline asm */
        while !(*ps).is_null()
            && (**ps)
                .c2rust_unnamed
                .c2rust_unnamed
                .c2rust_unnamed
                .sym_scope
                != 0
        {
            ps = &mut (**ps).prev_tok
        }
        (*s).prev_tok = *ps;
        *ps = s
    }
    return s;
}
/* pop symbols until top reaches 'b'.  If KEEP is non-zero don't really
pop them yet from the list, but do remove them from the token array.  */
#[no_mangle]
pub unsafe extern "C" fn sym_pop(
    mut ptop: *mut *mut Sym,
    mut b: *mut Sym,
    mut keep: std::os::raw::c_int,
) {
    let mut s: *mut Sym = 0 as *mut Sym;
    let mut ss: *mut Sym = 0 as *mut Sym;
    let mut ps: *mut *mut Sym = 0 as *mut *mut Sym;
    let mut ts: *mut TokenSym = 0 as *mut TokenSym;
    let mut v: std::os::raw::c_int = 0;
    s = *ptop;
    while s != b {
        ss = (*s).prev;
        v = (*s).v;
        /* remove symbol in token array */
        /* XXX: simplify */
        if v & 0x20000000 as std::os::raw::c_int == 0
            && (v & !(0x40000000 as std::os::raw::c_int)) < 0x10000000 as std::os::raw::c_int
        {
            ts = *table_ident.offset(
                ((v & !(0x40000000 as std::os::raw::c_int)) - 256 as std::os::raw::c_int) as isize,
            );
            if v & 0x40000000 as std::os::raw::c_int != 0 {
                ps = &mut (*ts).sym_struct
            } else {
                ps = &mut (*ts).sym_identifier
            }
            *ps = (*s).prev_tok
        }
        if keep == 0 {
            sym_free(s);
        }
        s = ss
    }
    if keep == 0 {
        *ptop = b
    };
}
/* ------------------------------------------------------------------------- */
unsafe extern "C" fn vcheck_cmp() {
    /* cannot let cpu flags if other instruction are generated. Also
    avoid leaving VT_JMP anywhere except on the top of the stack
    because it would complicate the code generator.

    Don't do this when nocode_wanted.  vtop might come from
    !nocode_wanted regions (see 88_codeopt.c) and transforming
    it to a register without actually generating code is wrong
    as their value might still be used for real.  All values
    we push under nocode_wanted will eventually be popped
    again, so that the VT_CMP/VT_JMP value will be in vtop
    when code is unsuppressed again. */
    if (*vtop).r as std::os::raw::c_int == 0x33 as std::os::raw::c_int && nocode_wanted == 0 {
        gv(0x1 as std::os::raw::c_int);
    };
}
unsafe extern "C" fn vsetc(
    mut type_0: *mut CType,
    mut r: std::os::raw::c_int,
    mut vc: *mut CValue,
) {
    if vtop
        >= _vstack
            .as_mut_ptr()
            .offset(1 as std::os::raw::c_int as isize)
            .offset((256 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize)
    {
        _tcc_error(b"memory full (vstack)\x00" as *const u8 as *const std::os::raw::c_char);
    }
    vcheck_cmp();
    vtop = vtop.offset(1);
    (*vtop).type_0 = *type_0;
    (*vtop).r = r as std::os::raw::c_ushort;
    (*vtop).r2 = 0x30 as std::os::raw::c_int as std::os::raw::c_ushort;
    (*vtop).c2rust_unnamed.c = *vc;
    (*vtop).c2rust_unnamed_0.sym = 0 as *mut Sym;
}
#[no_mangle]
pub unsafe extern "C" fn vswap() {
    let mut tmp: SValue = SValue {
        type_0: CType {
            t: 0,
            ref_0: 0 as *const Sym as *mut Sym,
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
    vcheck_cmp();
    tmp = *vtop.offset(0 as std::os::raw::c_int as isize);
    *vtop.offset(0 as std::os::raw::c_int as isize) =
        *vtop.offset(-(1 as std::os::raw::c_int) as isize);
    *vtop.offset(-(1 as std::os::raw::c_int) as isize) = tmp;
}
/* pop stack value */
#[no_mangle]
pub unsafe extern "C" fn vpop() {
    let mut v: std::os::raw::c_int = 0;
    v = (*vtop).r as std::os::raw::c_int & 0x3f as std::os::raw::c_int;
    /* for x86, we need to pop the FP stack */
    if v == TREG_ST0 as std::os::raw::c_int {
        o(0xd8dd as std::os::raw::c_int as std::os::raw::c_uint);
    /* fstp %st(0) */
    } else if v == 0x33 as std::os::raw::c_int {
        /* need to put correct jump if && or || without test */
        gsym((*vtop).c2rust_unnamed.c2rust_unnamed.jtrue);
        gsym((*vtop).c2rust_unnamed.c2rust_unnamed.jfalse);
    }
    vtop = vtop.offset(-1);
}
/* push constant of type "type" with useless value */
unsafe extern "C" fn vpush(mut type_0: *mut CType) {
    vset(
        type_0,
        0x30 as std::os::raw::c_int,
        0 as std::os::raw::c_int,
    );
}
/* push arbitrary 64bit constant */
unsafe extern "C" fn vpush64(mut ty: std::os::raw::c_int, mut v: std::os::raw::c_ulonglong) {
    let mut cval: CValue = CValue {
        ld: f128::f128::ZERO,
    };
    let mut ctype: CType = CType {
        t: 0,
        ref_0: 0 as *const Sym as *mut Sym,
    };
    ctype.t = ty;
    ctype.ref_0 = 0 as *mut Sym;
    cval.i = v as uint64_t;
    vsetc(&mut ctype, 0x30 as std::os::raw::c_int, &mut cval);
}
/* push integer constant */
#[no_mangle]
pub unsafe extern "C" fn vpushi(mut v: std::os::raw::c_int) {
    vpush64(3 as std::os::raw::c_int, v as std::os::raw::c_ulonglong);
}
/* push a pointer sized constant */
unsafe extern "C" fn vpushs(mut v: Elf64_Addr) {
    vpush64(
        0x800 as std::os::raw::c_int | 4 as std::os::raw::c_int | 0x10 as std::os::raw::c_int,
        v as std::os::raw::c_ulonglong,
    );
}
/* push long long constant */
#[inline]
unsafe extern "C" fn vpushll(mut v: std::os::raw::c_longlong) {
    vpush64(4 as std::os::raw::c_int, v as std::os::raw::c_ulonglong);
}
#[no_mangle]
pub unsafe extern "C" fn vset(
    mut type_0: *mut CType,
    mut r: std::os::raw::c_int,
    mut v: std::os::raw::c_int,
) {
    let mut cval: CValue = CValue {
        ld: f128::f128::ZERO,
    };
    cval.i = v as uint64_t;
    vsetc(type_0, r, &mut cval);
}
unsafe extern "C" fn vseti(mut r: std::os::raw::c_int, mut v: std::os::raw::c_int) {
    let mut type_0: CType = CType {
        t: 0,
        ref_0: 0 as *const Sym as *mut Sym,
    };
    type_0.t = 3 as std::os::raw::c_int;
    type_0.ref_0 = 0 as *mut Sym;
    vset(&mut type_0, r, v);
}
#[no_mangle]
pub unsafe extern "C" fn vpushv(mut v: *mut SValue) {
    if vtop
        >= _vstack
            .as_mut_ptr()
            .offset(1 as std::os::raw::c_int as isize)
            .offset((256 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize)
    {
        _tcc_error(b"memory full (vstack)\x00" as *const u8 as *const std::os::raw::c_char);
    }
    vtop = vtop.offset(1);
    *vtop = *v;
}
unsafe extern "C" fn vdup() {
    vpushv(vtop);
}
/* rotate n first stack elements to the bottom
   I1 ... In -> I2 ... In I1 [top is right]
*/
#[no_mangle]
pub unsafe extern "C" fn vrotb(mut n: std::os::raw::c_int) {
    let mut i: std::os::raw::c_int = 0;
    let mut tmp: SValue = SValue {
        type_0: CType {
            t: 0,
            ref_0: 0 as *const Sym as *mut Sym,
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
    vcheck_cmp();
    tmp = *vtop.offset((-n + 1 as std::os::raw::c_int) as isize);
    i = -n + 1 as std::os::raw::c_int;
    while i != 0 as std::os::raw::c_int {
        *vtop.offset(i as isize) = *vtop.offset((i + 1 as std::os::raw::c_int) as isize);
        i += 1
    }
    *vtop.offset(0 as std::os::raw::c_int as isize) = tmp;
}
/* rotate the n elements before entry e towards the top
  I1 ... In ... -> In I1 ... I(n-1) ... [top is right]
*/
#[no_mangle]
pub unsafe extern "C" fn vrote(mut e: *mut SValue, mut n: std::os::raw::c_int) {
    let mut i: std::os::raw::c_int = 0;
    let mut tmp: SValue = SValue {
        type_0: CType {
            t: 0,
            ref_0: 0 as *const Sym as *mut Sym,
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
    vcheck_cmp();
    tmp = *e;
    i = 0 as std::os::raw::c_int;
    while i < n - 1 as std::os::raw::c_int {
        *e.offset(-i as isize) = *e.offset((-i - 1 as std::os::raw::c_int) as isize);
        i += 1
    }
    *e.offset((-n + 1 as std::os::raw::c_int) as isize) = tmp;
}
/* rotate n first stack elements to the top
  I1 ... In -> In I1 ... I(n-1)  [top is right]
*/
#[no_mangle]
pub unsafe extern "C" fn vrott(mut n: std::os::raw::c_int) {
    vrote(vtop, n);
}
/* ------------------------------------------------------------------------- */
/* vtop->r = VT_CMP means CPU-flags have been set from comparison or test. */
/* called from generators to set the result from relational ops  */
#[no_mangle]
pub unsafe extern "C" fn vset_VT_CMP(mut op: std::os::raw::c_int) {
    (*vtop).r = 0x33 as std::os::raw::c_int as std::os::raw::c_ushort;
    (*vtop).c2rust_unnamed_0.c2rust_unnamed.cmp_op = op as std::os::raw::c_ushort;
    (*vtop).c2rust_unnamed.c2rust_unnamed.jfalse = 0 as std::os::raw::c_int;
    (*vtop).c2rust_unnamed.c2rust_unnamed.jtrue = 0 as std::os::raw::c_int;
}
/* called once before asking generators to load VT_CMP to a register */
unsafe extern "C" fn vset_VT_JMP() {
    let mut op: std::os::raw::c_int =
        (*vtop).c2rust_unnamed_0.c2rust_unnamed.cmp_op as std::os::raw::c_int;
    if (*vtop).c2rust_unnamed.c2rust_unnamed.jtrue != 0
        || (*vtop).c2rust_unnamed.c2rust_unnamed.jfalse != 0
    {
        /* we need to jump to 'mov $0,%R' or 'mov $1,%R' */
        let mut inv: std::os::raw::c_int =
            op & (op < 2 as std::os::raw::c_int) as std::os::raw::c_int; /* small optimization */
        vseti(
            0x34 as std::os::raw::c_int + inv,
            gvtst(inv, 0 as std::os::raw::c_int),
        );
    } else {
        /* otherwise convert flags (rsp. 0/1) to register */
        (*vtop).c2rust_unnamed.c.i = op as uint64_t;
        if op < 2 as std::os::raw::c_int {
            /* doesn't seem to happen */
            (*vtop).r = 0x30 as std::os::raw::c_int as std::os::raw::c_ushort
        }
    };
}
/* Set CPU Flags, doesn't yet jump */
unsafe extern "C" fn gvtst_set(mut inv: std::os::raw::c_int, mut t: std::os::raw::c_int) {
    let mut p: *mut std::os::raw::c_int = 0 as *mut std::os::raw::c_int;
    if (*vtop).r as std::os::raw::c_int != 0x33 as std::os::raw::c_int {
        vpushi(0 as std::os::raw::c_int);
        gen_op(0x95 as std::os::raw::c_int);
        if (*vtop).r as std::os::raw::c_int != 0x33 as std::os::raw::c_int {
            /* must be VT_CONST then */
            vset_VT_CMP(
                ((*vtop).c2rust_unnamed.c.i != 0 as std::os::raw::c_int as std::os::raw::c_ulong)
                    as std::os::raw::c_int,
            );
        }
    }
    p = if inv != 0 {
        &mut (*vtop).c2rust_unnamed.c2rust_unnamed.jfalse
    } else {
        &mut (*vtop).c2rust_unnamed.c2rust_unnamed.jtrue
    };
    *p = gjmp_append(*p, t);
}
/* Generate value test
 *
 * Generate a test for any value (jump, comparison and integers) */
unsafe extern "C" fn gvtst(
    mut inv: std::os::raw::c_int,
    mut t: std::os::raw::c_int,
) -> std::os::raw::c_int {
    let mut op: std::os::raw::c_int = 0;
    let mut x: std::os::raw::c_int = 0;
    let mut u: std::os::raw::c_int = 0;
    gvtst_set(inv, t);
    t = (*vtop).c2rust_unnamed.c2rust_unnamed.jtrue;
    u = (*vtop).c2rust_unnamed.c2rust_unnamed.jfalse;
    if inv != 0 {
        x = u;
        u = t;
        t = x
    }
    op = (*vtop).c2rust_unnamed_0.c2rust_unnamed.cmp_op as std::os::raw::c_int;
    /* jump to the wanted target */
    if op > 1 as std::os::raw::c_int {
        t = gjmp_cond(op ^ inv, t)
    } else if op != inv {
        t = gjmp_acs(t)
    }
    /* resolve complementary jumps to here */
    gsym(u);
    vtop = vtop.offset(-1);
    return t;
}
/* generate a zero or nozero test */
unsafe extern "C" fn gen_test_zero(mut op: std::os::raw::c_int) {
    if (*vtop).r as std::os::raw::c_int == 0x33 as std::os::raw::c_int {
        let mut j: std::os::raw::c_int = 0;
        if op == 0x94 as std::os::raw::c_int {
            j = (*vtop).c2rust_unnamed.c2rust_unnamed.jfalse;
            (*vtop).c2rust_unnamed.c2rust_unnamed.jfalse =
                (*vtop).c2rust_unnamed.c2rust_unnamed.jtrue;
            (*vtop).c2rust_unnamed.c2rust_unnamed.jtrue = j;
            (*vtop).c2rust_unnamed_0.c2rust_unnamed.cmp_op =
                ((*vtop).c2rust_unnamed_0.c2rust_unnamed.cmp_op as std::os::raw::c_int
                    ^ 1 as std::os::raw::c_int) as std::os::raw::c_ushort
        }
    } else {
        vpushi(0 as std::os::raw::c_int);
        gen_op(op);
    };
}
/* ------------------------------------------------------------------------- */
/* push a symbol value of TYPE */
#[no_mangle]
pub unsafe extern "C" fn vpushsym(mut type_0: *mut CType, mut sym: *mut Sym) {
    let mut cval: CValue = CValue {
        ld: f128::f128::ZERO,
    };
    cval.i = 0 as std::os::raw::c_int as uint64_t;
    vsetc(
        type_0,
        0x30 as std::os::raw::c_int | 0x200 as std::os::raw::c_int,
        &mut cval,
    );
    (*vtop).c2rust_unnamed_0.sym = sym;
}
/* Return a static symbol pointing to a section */
#[no_mangle]
pub unsafe extern "C" fn get_sym_ref(
    mut type_0: *mut CType,
    mut sec: *mut Section,
    mut offset: std::os::raw::c_ulong,
    mut size: std::os::raw::c_ulong,
) -> *mut Sym {
    let mut v: std::os::raw::c_int = 0;
    let mut sym: *mut Sym = 0 as *mut Sym;
    let fresh3 = anon_sym;
    anon_sym = anon_sym + 1;
    v = fresh3;
    sym = sym_push(
        v,
        type_0,
        0x30 as std::os::raw::c_int | 0x200 as std::os::raw::c_int,
        0 as std::os::raw::c_int,
    );
    (*sym).type_0.t |= 0x2000 as std::os::raw::c_int;
    put_extern_sym(sym, sec, offset, size);
    return sym;
}
/* push a reference to a section offset by adding a dummy symbol */
unsafe extern "C" fn vpush_ref(
    mut type_0: *mut CType,
    mut sec: *mut Section,
    mut offset: std::os::raw::c_ulong,
    mut size: std::os::raw::c_ulong,
) {
    vpushsym(type_0, get_sym_ref(type_0, sec, offset, size));
}
/* define a new external reference to a symbol 'v' of type 'u' */
#[no_mangle]
pub unsafe extern "C" fn external_global_sym(
    mut v: std::os::raw::c_int,
    mut type_0: *mut CType,
) -> *mut Sym {
    let mut s: *mut Sym = 0 as *mut Sym;
    s = sym_find(v);
    if s.is_null() {
        /* push forward reference */
        s = global_identifier_push(
            v,
            (*type_0).t | 0x1000 as std::os::raw::c_int,
            0 as std::os::raw::c_int,
        );
        (*s).type_0.ref_0 = (*type_0).ref_0
    } else if (*s).type_0.t
        & (0xf as std::os::raw::c_int
            | (0 as std::os::raw::c_int | (1 as std::os::raw::c_int) << 20 as std::os::raw::c_int))
        == 0 as std::os::raw::c_int | (1 as std::os::raw::c_int) << 20 as std::os::raw::c_int
    {
        (*s).type_0.t = (*type_0).t | (*s).type_0.t & 0x1000 as std::os::raw::c_int;
        (*s).type_0.ref_0 = (*type_0).ref_0;
        update_storage(s);
    }
    return s;
}
/* create an external reference with no specific type similar to asm labels.
This avoids type conflicts if the symbol is used from C too */
#[no_mangle]
pub unsafe extern "C" fn external_helper_sym(mut v: std::os::raw::c_int) -> *mut Sym {
    let mut ct: CType = {
        let mut init = CType {
            t: 0 as std::os::raw::c_int
                | (1 as std::os::raw::c_int) << 20 as std::os::raw::c_int
                | (2 as std::os::raw::c_int) << 20 as std::os::raw::c_int,
            ref_0: 0 as *mut Sym,
        };
        init
    };
    return external_global_sym(v, &mut ct);
}
/* push a reference to an helper function (such as memmove) */
#[no_mangle]
pub unsafe extern "C" fn vpush_helper_func(mut v: std::os::raw::c_int) {
    vpushsym(&mut func_old_type, external_helper_sym(v));
}
/* Merge symbol attributes.  */
unsafe extern "C" fn merge_symattr(mut sa: *mut SymAttr, mut sa1: *mut SymAttr) {
    if (*sa1).aligned() as std::os::raw::c_int != 0 && (*sa).aligned() == 0 {
        (*sa).set_aligned((*sa1).aligned())
    }
    (*sa).set_packed(
        (*sa).packed() | (*sa1).packed() as std::os::raw::c_int as std::os::raw::c_ushort,
    );
    (*sa).set_weak((*sa).weak() | (*sa1).weak() as std::os::raw::c_int as std::os::raw::c_ushort);
    if (*sa1).visibility() as std::os::raw::c_int != 0 as std::os::raw::c_int {
        let mut vis: std::os::raw::c_int = (*sa).visibility() as std::os::raw::c_int;
        if vis == 0 as std::os::raw::c_int || vis > (*sa1).visibility() as std::os::raw::c_int {
            vis = (*sa1).visibility() as std::os::raw::c_int
        }
        (*sa).set_visibility(vis as std::os::raw::c_ushort)
    }
    (*sa).set_dllexport(
        (*sa).dllexport() | (*sa1).dllexport() as std::os::raw::c_int as std::os::raw::c_ushort,
    );
    (*sa).set_nodecorate(
        (*sa).nodecorate() | (*sa1).nodecorate() as std::os::raw::c_int as std::os::raw::c_ushort,
    );
    (*sa).set_dllimport(
        (*sa).dllimport() | (*sa1).dllimport() as std::os::raw::c_int as std::os::raw::c_ushort,
    );
}
/* Merge function attributes.  */
unsafe extern "C" fn merge_funcattr(mut fa: *mut FuncAttr, mut fa1: *mut FuncAttr) {
    if (*fa1).func_call() as std::os::raw::c_int != 0 && (*fa).func_call() == 0 {
        (*fa).set_func_call((*fa1).func_call())
    }
    if (*fa1).func_type() as std::os::raw::c_int != 0 && (*fa).func_type() == 0 {
        (*fa).set_func_type((*fa1).func_type())
    }
    if (*fa1).func_args() as std::os::raw::c_int != 0 && (*fa).func_args() == 0 {
        (*fa).set_func_args((*fa1).func_args())
    }
    if (*fa1).func_noreturn() != 0 {
        (*fa).set_func_noreturn(1 as std::os::raw::c_int as std::os::raw::c_uint)
    }
    if (*fa1).func_ctor() != 0 {
        (*fa).set_func_ctor(1 as std::os::raw::c_int as std::os::raw::c_uint)
    }
    if (*fa1).func_dtor() != 0 {
        (*fa).set_func_dtor(1 as std::os::raw::c_int as std::os::raw::c_uint)
    };
}
/* Merge attributes.  */
unsafe extern "C" fn merge_attr(mut ad: *mut AttributeDef, mut ad1: *mut AttributeDef) {
    merge_symattr(&mut (*ad).a, &mut (*ad1).a);
    merge_funcattr(&mut (*ad).f, &mut (*ad1).f);
    if !(*ad1).section.is_null() {
        (*ad).section = (*ad1).section
    }
    if (*ad1).alias_target != 0 {
        (*ad).alias_target = (*ad1).alias_target
    }
    if (*ad1).asm_label != 0 {
        (*ad).asm_label = (*ad1).asm_label
    }
    if (*ad1).attr_mode != 0 {
        (*ad).attr_mode = (*ad1).attr_mode
    };
}
/* Merge some type attributes.  */
unsafe extern "C" fn patch_type(mut sym: *mut Sym, mut type_0: *mut CType) {
    if (*type_0).t & 0x1000 as std::os::raw::c_int == 0
        || (*sym).type_0.t as std::os::raw::c_uint
            & (((1 as std::os::raw::c_uint) << 6 as std::os::raw::c_int + 6 as std::os::raw::c_int)
                .wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_uint)
                << 20 as std::os::raw::c_int
                | 0x80 as std::os::raw::c_int as std::os::raw::c_uint)
            == ((3 as std::os::raw::c_int) << 20 as std::os::raw::c_int) as std::os::raw::c_uint
    {
        if (*sym).type_0.t & 0x1000 as std::os::raw::c_int == 0 {
            _tcc_error(
                b"redefinition of \'%s\'\x00" as *const u8 as *const std::os::raw::c_char,
                get_tok_str((*sym).v, 0 as *mut CValue),
            );
        }
        (*sym).type_0.t &= !(0x1000 as std::os::raw::c_int)
    }
    if (*sym).type_0.t
        & (0xf as std::os::raw::c_int
            | (0 as std::os::raw::c_int | (1 as std::os::raw::c_int) << 20 as std::os::raw::c_int))
        == 0 as std::os::raw::c_int | (1 as std::os::raw::c_int) << 20 as std::os::raw::c_int
    {
        /* stay static if both are static */
        (*sym).type_0.t = (*type_0).t & ((*sym).type_0.t | !(0x2000 as std::os::raw::c_int));
        (*sym).type_0.ref_0 = (*type_0).ref_0
    }
    if is_compatible_types(&mut (*sym).type_0, type_0) == 0 {
        _tcc_error(
            b"incompatible types for redefinition of \'%s\'\x00" as *const u8
                as *const std::os::raw::c_char,
            get_tok_str((*sym).v, 0 as *mut CValue),
        );
    } else {
        if (*sym).type_0.t & 0xf as std::os::raw::c_int == 6 as std::os::raw::c_int {
            let mut static_proto: std::os::raw::c_int =
                (*sym).type_0.t & 0x2000 as std::os::raw::c_int;
            /* warn if static follows non-static function declaration */
            if (*type_0).t & 0x2000 as std::os::raw::c_int != 0
                && static_proto == 0
                && ((*type_0).t | (*sym).type_0.t) & 0x8000 as std::os::raw::c_int == 0
            {
                _tcc_warning(
                    b"static storage ignored for redefinition of \'%s\'\x00" as *const u8
                        as *const std::os::raw::c_char,
                    get_tok_str((*sym).v, 0 as *mut CValue),
                );
            }
            /* set 'inline' if both agree or if one has static */
            if ((*type_0).t | (*sym).type_0.t) & 0x8000 as std::os::raw::c_int != 0 {
                if ((*type_0).t ^ (*sym).type_0.t) & 0x8000 as std::os::raw::c_int == 0
                    || ((*type_0).t | (*sym).type_0.t) & 0x2000 as std::os::raw::c_int != 0
                {
                    static_proto |= 0x8000 as std::os::raw::c_int
                }
            }
            if 0 as std::os::raw::c_int == (*type_0).t & 0x1000 as std::os::raw::c_int {
                let mut f: FuncAttr = (*(*sym).type_0.ref_0)
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .f;
                /* put complete type, use static from prototype */
                (*sym).type_0.t = (*type_0).t
                    & !(0x2000 as std::os::raw::c_int | 0x8000 as std::os::raw::c_int)
                    | static_proto;
                (*sym).type_0.ref_0 = (*type_0).ref_0;
                merge_funcattr(
                    &mut (*(*sym).type_0.ref_0)
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .f,
                    &mut f,
                );
            } else {
                (*sym).type_0.t &= !(0x8000 as std::os::raw::c_int) | static_proto
            }
            if (*(*sym).type_0.ref_0)
                .c2rust_unnamed
                .c2rust_unnamed
                .c2rust_unnamed
                .f
                .func_type() as std::os::raw::c_int
                == 2 as std::os::raw::c_int
                && (*(*type_0).ref_0)
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .f
                    .func_type() as std::os::raw::c_int
                    != 2 as std::os::raw::c_int
            {
                (*sym).type_0.ref_0 = (*type_0).ref_0
            }
        } else {
            if (*sym).type_0.t & 0x40 as std::os::raw::c_int != 0
                && (*(*type_0).ref_0).c2rust_unnamed.c2rust_unnamed.c >= 0 as std::os::raw::c_int
            {
                /* set array size if it was omitted in extern declaration */
                (*(*sym).type_0.ref_0).c2rust_unnamed.c2rust_unnamed.c =
                    (*(*type_0).ref_0).c2rust_unnamed.c2rust_unnamed.c
            }
            if ((*type_0).t ^ (*sym).type_0.t) & 0x2000 as std::os::raw::c_int != 0 {
                _tcc_warning(
                    b"storage mismatch for redefinition of \'%s\'\x00" as *const u8
                        as *const std::os::raw::c_char,
                    get_tok_str((*sym).v, 0 as *mut CValue),
                );
            }
        }
    };
}
/* Merge some storage attributes.  */
unsafe extern "C" fn patch_storage(
    mut sym: *mut Sym,
    mut ad: *mut AttributeDef,
    mut type_0: *mut CType,
) {
    if !type_0.is_null() {
        patch_type(sym, type_0);
    }
    merge_symattr(&mut (*sym).a, &mut (*ad).a);
    if (*ad).asm_label != 0 {
        (*sym).c2rust_unnamed_0.asm_label = (*ad).asm_label
    }
    update_storage(sym);
}
/* copy sym to other stack */
unsafe extern "C" fn sym_copy(mut s0: *mut Sym, mut ps: *mut *mut Sym) -> *mut Sym {
    let mut s: *mut Sym = 0 as *mut Sym;
    s = sym_malloc();
    *s = *s0;
    (*s).prev = *ps;
    *ps = s;
    if (*s).v < 0x10000000 as std::os::raw::c_int {
        ps = &mut (**table_ident.offset(((*s).v - 256 as std::os::raw::c_int) as isize))
            .sym_identifier;
        (*s).prev_tok = *ps;
        *ps = s
    }
    return s;
}
/* copy s->type.ref to stack 'ps' for VT_FUNC and VT_PTR */
unsafe extern "C" fn sym_copy_ref(mut s: *mut Sym, mut ps: *mut *mut Sym) {
    let mut bt: std::os::raw::c_int = (*s).type_0.t & 0xf as std::os::raw::c_int;
    if bt == 6 as std::os::raw::c_int
        || bt == 5 as std::os::raw::c_int
        || bt == 7 as std::os::raw::c_int
            && (*s).c2rust_unnamed.c2rust_unnamed.c2rust_unnamed.sym_scope != 0
    {
        let mut sp: *mut *mut Sym = &mut (*s).type_0.ref_0;
        s = *sp;
        *sp = 0 as *mut Sym;
        while !s.is_null() {
            let mut s2: *mut Sym = sym_copy(s, ps);
            *sp = s2;
            sp = &mut (**sp).c2rust_unnamed_0.next;
            sym_copy_ref(s2, ps);
            s = (*s).c2rust_unnamed_0.next
        }
    };
}
/* define a new external reference to a symbol 'v' */
unsafe extern "C" fn external_sym(
    mut v: std::os::raw::c_int,
    mut type_0: *mut CType,
    mut r: std::os::raw::c_int,
    mut ad: *mut AttributeDef,
) -> *mut Sym {
    let mut s: *mut Sym = 0 as *mut Sym;
    /* look for global symbol */
    s = sym_find(v);
    while !s.is_null() && (*s).c2rust_unnamed.c2rust_unnamed.c2rust_unnamed.sym_scope != 0 {
        s = (*s).prev_tok
    }
    if s.is_null() {
        /* push forward reference */
        s = global_identifier_push(v, (*type_0).t, 0 as std::os::raw::c_int);
        (*s).r = ((*s).r as std::os::raw::c_int | r) as std::os::raw::c_ushort;
        (*s).a = (*ad).a;
        (*s).c2rust_unnamed_0.asm_label = (*ad).asm_label;
        (*s).type_0.ref_0 = (*type_0).ref_0;
        /* copy type to the global stack */
        if !local_stack.is_null() {
            sym_copy_ref(s, &mut global_stack);
        }
    } else {
        patch_storage(s, ad, type_0);
    }
    /* push variables on local_stack if any */
    if !local_stack.is_null()
        && (*s).type_0.t & 0xf as std::os::raw::c_int != 6 as std::os::raw::c_int
    {
        s = sym_copy(s, &mut local_stack)
    }
    return s;
}
/* save registers up to (vtop - n) stack entry */
#[no_mangle]
pub unsafe extern "C" fn save_regs(mut n: std::os::raw::c_int) {
    let mut p: *mut SValue = 0 as *mut SValue;
    let mut p1: *mut SValue = 0 as *mut SValue;
    p = _vstack
        .as_mut_ptr()
        .offset(1 as std::os::raw::c_int as isize);
    p1 = vtop.offset(-(n as isize));
    while p <= p1 {
        save_reg((*p).r as std::os::raw::c_int);
        p = p.offset(1)
    }
}
/* save r to the memory stack, and mark it as being free */
#[no_mangle]
pub unsafe extern "C" fn save_reg(mut r: std::os::raw::c_int) {
    save_reg_upstack(r, 0 as std::os::raw::c_int);
}
/* save r to the memory stack, and mark it as being free,
if seen up to (vtop - n) stack entry */
#[no_mangle]
pub unsafe extern "C" fn save_reg_upstack(mut r: std::os::raw::c_int, mut n: std::os::raw::c_int) {
    let mut l: std::os::raw::c_int = 0;
    let mut size: std::os::raw::c_int = 0;
    let mut align: std::os::raw::c_int = 0;
    let mut bt: std::os::raw::c_int = 0;
    let mut p: *mut SValue = 0 as *mut SValue;
    let mut p1: *mut SValue = 0 as *mut SValue;
    let mut sv: SValue = SValue {
        type_0: CType {
            t: 0,
            ref_0: 0 as *const Sym as *mut Sym,
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
    r &= 0x3f as std::os::raw::c_int;
    if r >= 0x30 as std::os::raw::c_int {
        return;
    }
    if nocode_wanted != 0 {
        return;
    }
    l = 0 as std::os::raw::c_int;
    let mut current_block_29: u64;
    p = _vstack
        .as_mut_ptr()
        .offset(1 as std::os::raw::c_int as isize);
    p1 = vtop.offset(-(n as isize));
    while p <= p1 {
        if (*p).r as std::os::raw::c_int & 0x3f as std::os::raw::c_int == r
            || (*p).r2 as std::os::raw::c_int == r
        {
            /* must save value on stack if not already done */
            if l == 0 {
                bt = (*p).type_0.t & 0xf as std::os::raw::c_int;
                if bt == 0 as std::os::raw::c_int {
                    current_block_29 = 11875828834189669668;
                } else {
                    if (*p).r as std::os::raw::c_int & 0x100 as std::os::raw::c_int != 0
                        || bt == 6 as std::os::raw::c_int
                    {
                        bt = 5 as std::os::raw::c_int
                    }
                    sv.type_0.t = bt;
                    size = type_size(&mut sv.type_0, &mut align);
                    l = get_temp_local_var(size, align);
                    sv.r = (0x32 as std::os::raw::c_int | 0x100 as std::os::raw::c_int)
                        as std::os::raw::c_ushort;
                    sv.c2rust_unnamed.c.i = l as uint64_t;
                    store(
                        (*p).r as std::os::raw::c_int & 0x3f as std::os::raw::c_int,
                        &mut sv,
                    );
                    /* x86 specific: need to pop fp register ST0 if saved */
                    if r == TREG_ST0 as std::os::raw::c_int {
                        o(0xd8dd as std::os::raw::c_int as std::os::raw::c_uint);
                        /* fstp %st(0) */
                    }
                    /* special long long case */
                    if ((*p).r2 as std::os::raw::c_int) < 0x30 as std::os::raw::c_int
                        && R2_RET(bt) != 0x30 as std::os::raw::c_int
                    {
                        sv.c2rust_unnamed.c.i = (sv.c2rust_unnamed.c.i as std::os::raw::c_ulong)
                            .wrapping_add(8 as std::os::raw::c_int as std::os::raw::c_ulong)
                            as uint64_t as uint64_t;
                        store((*p).r2 as std::os::raw::c_int, &mut sv);
                    }
                    current_block_29 = 13472856163611868459;
                }
            } else {
                current_block_29 = 13472856163611868459;
            }
            match current_block_29 {
                11875828834189669668 => {},
                _ => {
                    /* mark that stack entry as being saved on the stack */
                    if (*p).r as std::os::raw::c_int & 0x100 as std::os::raw::c_int != 0 {
                        /* also clear the bounded flag because the
                        relocation address of the function was stored in
                        p->c.i */
                        (*p).r = ((*p).r as std::os::raw::c_int
                            & !(0x3f as std::os::raw::c_int | 0x8000 as std::os::raw::c_int)
                            | 0x31 as std::os::raw::c_int)
                            as std::os::raw::c_ushort
                    } else {
                        (*p).r = (0x100 as std::os::raw::c_int | 0x32 as std::os::raw::c_int)
                            as std::os::raw::c_ushort
                    }
                    (*p).c2rust_unnamed_0.sym = 0 as *mut Sym;
                    (*p).r2 = 0x30 as std::os::raw::c_int as std::os::raw::c_ushort;
                    (*p).c2rust_unnamed.c.i = l as uint64_t
                },
            }
        }
        p = p.offset(1)
    }
}
/* find a free register of class 'rc'. If none, save one register */
#[no_mangle]
pub unsafe extern "C" fn get_reg(mut rc: std::os::raw::c_int) -> std::os::raw::c_int {
    let mut r: std::os::raw::c_int = 0;
    let mut p: *mut SValue = 0 as *mut SValue;
    /* find a free register */
    r = 0 as std::os::raw::c_int;
    while r < 25 as std::os::raw::c_int {
        let mut current_block_5: u64;
        if reg_classes[r as usize] & rc != 0 {
            if nocode_wanted != 0 {
                return r;
            }
            p = _vstack
                .as_mut_ptr()
                .offset(1 as std::os::raw::c_int as isize);
            loop {
                if !(p <= vtop) {
                    current_block_5 = 13183875560443969876;
                    break;
                }
                if (*p).r as std::os::raw::c_int & 0x3f as std::os::raw::c_int == r
                    || (*p).r2 as std::os::raw::c_int == r
                {
                    current_block_5 = 7651349459974463963;
                    break;
                }
                p = p.offset(1)
            }
            match current_block_5 {
                7651349459974463963 => {},
                _ => return r,
            }
        }
        r += 1
    }
    /* no register left : free the first one on the stack (VERY
    IMPORTANT to start from the bottom to ensure that we don't
    spill registers used in gen_opi()) */
    p = _vstack
        .as_mut_ptr()
        .offset(1 as std::os::raw::c_int as isize);
    while p <= vtop {
        's_108: {
            /* look at second register (if long long) */
            r = (*p).r2 as std::os::raw::c_int;
            if !(r < 0x30 as std::os::raw::c_int && reg_classes[r as usize] & rc != 0) {
                r = (*p).r as std::os::raw::c_int & 0x3f as std::os::raw::c_int;
                if !(r < 0x30 as std::os::raw::c_int && reg_classes[r as usize] & rc != 0) {
                    break 's_108;
                }
            }
            save_reg(r);
            return r;
        }
        p = p.offset(1)
    }
    /* Should never comes here */
    return -(1 as std::os::raw::c_int);
}
/* find a free temporary local variable (return the offset on stack) match the size and align. If none, add new temporary stack variable*/
unsafe extern "C" fn get_temp_local_var(
    mut size: std::os::raw::c_int,
    mut align: std::os::raw::c_int,
) -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    let mut temp_var: *mut temp_local_variable = 0 as *mut temp_local_variable;
    let mut found_var: std::os::raw::c_int = 0;
    let mut p: *mut SValue = 0 as *mut SValue;
    let mut r: std::os::raw::c_int = 0;
    let mut free: std::os::raw::c_char = 0;
    let mut found: std::os::raw::c_char = 0;
    found = 0 as std::os::raw::c_int as std::os::raw::c_char;
    i = 0 as std::os::raw::c_int;
    while i < nb_temp_local_vars as std::os::raw::c_int {
        temp_var =
            &mut *arr_temp_local_vars.as_mut_ptr().offset(i as isize) as *mut temp_local_variable;
        if !(((*temp_var).size as std::os::raw::c_int) < size
            || align != (*temp_var).align as std::os::raw::c_int)
        {
            /*check if temp_var is free*/
            free = 1 as std::os::raw::c_int as std::os::raw::c_char;
            p = _vstack
                .as_mut_ptr()
                .offset(1 as std::os::raw::c_int as isize);
            while p <= vtop {
                r = (*p).r as std::os::raw::c_int & 0x3f as std::os::raw::c_int;
                if r == 0x32 as std::os::raw::c_int || r == 0x31 as std::os::raw::c_int {
                    if (*p).c2rust_unnamed.c.i == (*temp_var).location as std::os::raw::c_ulong {
                        free = 0 as std::os::raw::c_int as std::os::raw::c_char;
                        break;
                    }
                }
                p = p.offset(1)
            }
            if free != 0 {
                found_var = (*temp_var).location;
                found = 1 as std::os::raw::c_int as std::os::raw::c_char;
                break;
            }
        }
        i += 1
    }
    if found == 0 {
        loc = loc - size & -align;
        if (nb_temp_local_vars as std::os::raw::c_int) < 8 as std::os::raw::c_int {
            temp_var = &mut *arr_temp_local_vars.as_mut_ptr().offset(i as isize)
                as *mut temp_local_variable;
            (*temp_var).location = loc;
            (*temp_var).size = size as std::os::raw::c_short;
            (*temp_var).align = align as std::os::raw::c_short;
            nb_temp_local_vars += 1
        }
        found_var = loc
    }
    return found_var;
}
unsafe extern "C" fn clear_temp_local_var_list() {
    nb_temp_local_vars = 0 as std::os::raw::c_int as std::os::raw::c_short;
}
/* move register 's' (of type 't') to 'r', and flush previous value of r to memory
if needed */
unsafe extern "C" fn move_reg(
    mut r: std::os::raw::c_int,
    mut s: std::os::raw::c_int,
    mut t: std::os::raw::c_int,
) {
    let mut sv: SValue = SValue {
        type_0: CType {
            t: 0,
            ref_0: 0 as *const Sym as *mut Sym,
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
    if r != s {
        save_reg(r);
        sv.type_0.t = t;
        sv.type_0.ref_0 = 0 as *mut Sym;
        sv.r = s as std::os::raw::c_ushort;
        sv.c2rust_unnamed.c.i = 0 as std::os::raw::c_int as uint64_t;
        load(r, &mut sv);
    };
}
/* get address of vtop (vtop MUST BE an lvalue) */
#[no_mangle]
pub unsafe extern "C" fn gaddrof() {
    (*vtop).r = ((*vtop).r as std::os::raw::c_int & !(0x100 as std::os::raw::c_int))
        as std::os::raw::c_ushort;
    /* tricky: if saved lvalue, then we can go back to lvalue */
    if (*vtop).r as std::os::raw::c_int & 0x3f as std::os::raw::c_int == 0x31 as std::os::raw::c_int
    {
        (*vtop).r = ((*vtop).r as std::os::raw::c_int & !(0x3f as std::os::raw::c_int)
            | 0x32 as std::os::raw::c_int
            | 0x100 as std::os::raw::c_int) as std::os::raw::c_ushort
    };
}
/* generate a bounded pointer addition */
unsafe extern "C" fn gen_bounded_ptr_add() {
    let mut save: std::os::raw::c_int =
        ((*vtop.offset(-(1 as std::os::raw::c_int) as isize)).r as std::os::raw::c_int
            & 0x3f as std::os::raw::c_int
            == 0x32 as std::os::raw::c_int) as std::os::raw::c_int;
    if save != 0 {
        vpushv(&mut *vtop.offset(-(1 as std::os::raw::c_int) as isize));
        vrott(3 as std::os::raw::c_int);
    }
    vpush_helper_func(TOK___bound_ptr_add as std::os::raw::c_int);
    vrott(3 as std::os::raw::c_int);
    gfunc_call(2 as std::os::raw::c_int);
    vtop = vtop.offset(-(save as isize));
    vpushi(0 as std::os::raw::c_int);
    /* returned pointer is in REG_IRET */
    (*vtop).r =
        (TREG_RAX as std::os::raw::c_int | 0x8000 as std::os::raw::c_int) as std::os::raw::c_ushort;
    if nocode_wanted != 0 {
        return;
    }
    /* relocation offset of the bounding function call point */
    (*vtop).c2rust_unnamed.c.i = (*(*(*tcc_state).cur_text_section).reloc)
        .data_offset
        .wrapping_sub(::std::mem::size_of::<Elf64_Rela>() as std::os::raw::c_ulong);
}
/* patch pointer addition in vtop so that pointer dereferencing is
also tested */
unsafe extern "C" fn gen_bounded_ptr_deref() {
    let mut func: Elf64_Addr = 0;
    let mut size: std::os::raw::c_int = 0;
    let mut align: std::os::raw::c_int = 0;
    let mut rel: *mut Elf64_Rela = 0 as *mut Elf64_Rela;
    let mut sym: *mut Sym = 0 as *mut Sym;
    if nocode_wanted != 0 {
        return;
    }
    size = type_size(&mut (*vtop).type_0, &mut align);
    match size {
        1 => func = TOK___bound_ptr_indir1 as std::os::raw::c_int as Elf64_Addr,
        2 => func = TOK___bound_ptr_indir2 as std::os::raw::c_int as Elf64_Addr,
        4 => func = TOK___bound_ptr_indir4 as std::os::raw::c_int as Elf64_Addr,
        8 => func = TOK___bound_ptr_indir8 as std::os::raw::c_int as Elf64_Addr,
        12 => func = TOK___bound_ptr_indir12 as std::os::raw::c_int as Elf64_Addr,
        16 => func = TOK___bound_ptr_indir16 as std::os::raw::c_int as Elf64_Addr,
        _ => {
            /* may happen with struct member access */
            return;
        },
    }
    sym = external_helper_sym(func as std::os::raw::c_int);
    if (*sym).c2rust_unnamed.c2rust_unnamed.c == 0 {
        put_extern_sym(
            sym,
            0 as *mut Section,
            0 as std::os::raw::c_int as Elf64_Addr,
            0 as std::os::raw::c_int as std::os::raw::c_ulong,
        );
    }
    /* patch relocation */
    /* XXX: find a better solution ? */
    rel = (*(*(*tcc_state).cur_text_section).reloc)
        .data
        .offset((*vtop).c2rust_unnamed.c.i as isize) as *mut Elf64_Rela;
    (*rel).r_info = (((*sym).c2rust_unnamed.c2rust_unnamed.c as Elf64_Xword)
        << 32 as std::os::raw::c_int)
        .wrapping_add((*rel).r_info & 0xffffffff as std::os::raw::c_uint as std::os::raw::c_ulong);
}
/* generate lvalue bound code */
unsafe extern "C" fn gbound() {
    let mut type1: CType = CType {
        t: 0,
        ref_0: 0 as *const Sym as *mut Sym,
    };
    (*vtop).r = ((*vtop).r as std::os::raw::c_int & !(0x4000 as std::os::raw::c_int))
        as std::os::raw::c_ushort;
    /* if lvalue, then use checking code before dereferencing */
    if (*vtop).r as std::os::raw::c_int & 0x100 as std::os::raw::c_int != 0 {
        /* if not VT_BOUNDED value, then make one */
        if (*vtop).r as std::os::raw::c_int & 0x8000 as std::os::raw::c_int == 0 {
            /* must save type because we must set it to int to get pointer */
            type1 = (*vtop).type_0;
            (*vtop).type_0.t = 5 as std::os::raw::c_int;
            gaddrof();
            vpushi(0 as std::os::raw::c_int);
            gen_bounded_ptr_add();
            (*vtop).r = ((*vtop).r as std::os::raw::c_int | 0x100 as std::os::raw::c_int)
                as std::os::raw::c_ushort;
            (*vtop).type_0 = type1
        }
        /* then check for dereferencing */
        gen_bounded_ptr_deref();
    };
}
/* we need to call __bound_ptr_add before we start to load function
args into registers */
#[no_mangle]
pub unsafe extern "C" fn gbound_args(mut nb_args: std::os::raw::c_int) {
    let mut i: std::os::raw::c_int = 0;
    let mut v: std::os::raw::c_int = 0;
    let mut sv: *mut SValue = 0 as *mut SValue;
    i = 1 as std::os::raw::c_int;
    while i <= nb_args {
        if (*vtop.offset((1 as std::os::raw::c_int - i) as isize)).r as std::os::raw::c_int
            & 0x4000 as std::os::raw::c_int
            != 0
        {
            vrotb(i);
            gbound();
            vrott(i);
        }
        i += 1
    }
    sv = vtop.offset(-(nb_args as isize));
    if (*sv).r as std::os::raw::c_int & 0x200 as std::os::raw::c_int != 0 {
        v = (*(*sv).c2rust_unnamed_0.sym).v;
        if v == TOK_setjmp as std::os::raw::c_int
            || v == TOK__setjmp as std::os::raw::c_int
            || v == TOK_sigsetjmp as std::os::raw::c_int
            || v == TOK___sigsetjmp as std::os::raw::c_int
        {
            vpush_helper_func(TOK___bound_setjmp as std::os::raw::c_int);
            vpushv(sv.offset(1 as std::os::raw::c_int as isize));
            gfunc_call(1 as std::os::raw::c_int);
            func_bound_add_epilog = 1 as std::os::raw::c_int
        }
        if v == TOK_alloca as std::os::raw::c_int {
            func_bound_add_epilog = 1 as std::os::raw::c_int
        }
    };
}
/* Add bounds for local symbols from S to E (via ->prev) */
unsafe extern "C" fn add_local_bounds(mut s: *mut Sym, mut e: *mut Sym) {
    while s != e {
        if !((*s).v == 0
            || (*s).r as std::os::raw::c_int & 0x3f as std::os::raw::c_int
                != 0x32 as std::os::raw::c_int)
        {
            /* Add arrays/structs/unions because we always take address */
            if (*s).type_0.t & 0x40 as std::os::raw::c_int != 0
                || (*s).type_0.t & 0xf as std::os::raw::c_int == 7 as std::os::raw::c_int
                || (*s).a.addrtaken() as std::os::raw::c_int != 0
            {
                /* add local bound info */
                let mut align: std::os::raw::c_int = 0;
                let mut size: std::os::raw::c_int = type_size(&mut (*s).type_0, &mut align);
                let mut bounds_ptr: *mut Elf64_Addr = section_ptr_add(
                    (*tcc_state).lbounds_section,
                    (2 as std::os::raw::c_int as std::os::raw::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<Elf64_Addr>() as std::os::raw::c_ulong),
                ) as *mut Elf64_Addr;
                *bounds_ptr.offset(0 as std::os::raw::c_int as isize) =
                    (*s).c2rust_unnamed.c2rust_unnamed.c as Elf64_Addr;
                *bounds_ptr.offset(1 as std::os::raw::c_int as isize) = size as Elf64_Addr
            }
        }
        s = (*s).prev
    }
}
/* Wrapper around sym_pop, that potentially also registers local bounds.  */
unsafe extern "C" fn pop_local_syms(mut b: *mut Sym, mut keep: std::os::raw::c_int) {
    if (*tcc_state).do_bounds_check as std::os::raw::c_int != 0
        && keep == 0
        && (local_scope != 0 || func_var == 0)
    {
        add_local_bounds(local_stack, b);
    }
    if debug_modes != 0 {
        tcc_add_debug_info(
            tcc_state,
            (local_scope == 0) as std::os::raw::c_int,
            local_stack,
            b,
        );
    }
    sym_pop(&mut local_stack, b, keep);
}
unsafe extern "C" fn incr_bf_adr(mut o_0: std::os::raw::c_int) {
    (*vtop).type_0 = char_pointer_type;
    gaddrof();
    vpushs(o_0 as Elf64_Addr);
    gen_op('+' as i32);
    (*vtop).type_0.t = 1 as std::os::raw::c_int | 0x10 as std::os::raw::c_int;
    (*vtop).r =
        ((*vtop).r as std::os::raw::c_int | 0x100 as std::os::raw::c_int) as std::os::raw::c_ushort;
}
/* single-byte load mode for packed or otherwise unaligned bitfields */
unsafe extern "C" fn load_packed_bf(
    mut type_0: *mut CType,
    mut bit_pos: std::os::raw::c_int,
    mut bit_size: std::os::raw::c_int,
) {
    let mut n: std::os::raw::c_int = 0; // B X
    let mut o_0: std::os::raw::c_int = 0; // X B
    let mut bits: std::os::raw::c_int = 0; // X B B
    save_reg_upstack((*vtop).r as std::os::raw::c_int, 1 as std::os::raw::c_int); // X B Y
    vpush64(
        (*type_0).t & 0xf as std::os::raw::c_int,
        0 as std::os::raw::c_int as std::os::raw::c_ulonglong,
    ); // B Y X
    bits = 0 as std::os::raw::c_int; // B X
    o_0 = bit_pos >> 3 as std::os::raw::c_int;
    bit_pos &= 7 as std::os::raw::c_int;
    loop {
        vswap();
        incr_bf_adr(o_0);
        vdup();
        n = 8 as std::os::raw::c_int - bit_pos;
        if n > bit_size {
            n = bit_size
        }
        if bit_pos != 0 {
            vpushi(bit_pos);
            gen_op(0x8b as std::os::raw::c_int);
            bit_pos = 0 as std::os::raw::c_int
        }
        if n < 8 as std::os::raw::c_int {
            vpushi(((1 as std::os::raw::c_int) << n) - 1 as std::os::raw::c_int);
            gen_op('&' as i32);
        }
        gen_cast(type_0);
        if bits != 0 {
            vpushi(bits);
            gen_op('<' as i32);
        }
        vrotb(3 as std::os::raw::c_int);
        gen_op('|' as i32);
        bits += n;
        bit_size -= n;
        o_0 = 1 as std::os::raw::c_int;
        if !(bit_size != 0) {
            break;
        }
    }
    vswap();
    vpop();
    if (*type_0).t & 0x10 as std::os::raw::c_int == 0 {
        n = (if (*type_0).t & 0xf as std::os::raw::c_int == 4 as std::os::raw::c_int {
            64 as std::os::raw::c_int
        } else {
            32 as std::os::raw::c_int
        }) - bits;
        vpushi(n);
        gen_op('<' as i32);
        vpushi(n);
        gen_op('>' as i32);
    };
}
/* single-byte store mode for packed or otherwise unaligned bitfields */
unsafe extern "C" fn store_packed_bf(
    mut bit_pos: std::os::raw::c_int,
    mut bit_size: std::os::raw::c_int,
) {
    let mut bits: std::os::raw::c_int = 0; // X B
    let mut n: std::os::raw::c_int = 0; // X B
    let mut o_0: std::os::raw::c_int = 0; //B X
    let mut m: std::os::raw::c_int = 0; // B V X
    let mut c: std::os::raw::c_int = 0; // X B V
    c = ((*vtop).r as std::os::raw::c_int
        & (0x3f as std::os::raw::c_int
            | 0x100 as std::os::raw::c_int
            | 0x200 as std::os::raw::c_int)
        == 0x30 as std::os::raw::c_int) as std::os::raw::c_int;
    vswap();
    save_reg_upstack((*vtop).r as std::os::raw::c_int, 1 as std::os::raw::c_int);
    bits = 0 as std::os::raw::c_int;
    o_0 = bit_pos >> 3 as std::os::raw::c_int;
    bit_pos &= 7 as std::os::raw::c_int;
    loop {
        incr_bf_adr(o_0);
        vswap();
        if c != 0 {
            vdup();
        } else {
            gv_dup();
        };
        vrott(3 as std::os::raw::c_int);
        if bits != 0 {
            vpushi(bits);
            gen_op(0x8b as std::os::raw::c_int);
        }
        if bit_pos != 0 {
            vpushi(bit_pos);
            gen_op('<' as i32);
        }
        n = 8 as std::os::raw::c_int - bit_pos;
        if n > bit_size {
            n = bit_size
        }
        if n < 8 as std::os::raw::c_int {
            m = (((1 as std::os::raw::c_int) << n) - 1 as std::os::raw::c_int) << bit_pos;
            // X B V2
            vpushi(m); // X B V1
            gen_op('&' as i32); // X B V1 B
            vpushv(vtop.offset(-(1 as std::os::raw::c_int as isize))); // X B V1 B1
            vpushi(if m & 0x80 as std::os::raw::c_int != 0 {
                (!m) & 0x7f as std::os::raw::c_int
            } else {
                !m
            }); // X B B V2
            gen_op('&' as i32); // X B
            gen_op('|' as i32);
        }
        vdup();
        *vtop.offset(-(1 as std::os::raw::c_int) as isize) =
            *vtop.offset(-(2 as std::os::raw::c_int) as isize);
        vstore();
        vpop();
        bits += n;
        bit_size -= n;
        bit_pos = 0 as std::os::raw::c_int;
        o_0 = 1 as std::os::raw::c_int;
        if !(bit_size != 0) {
            break;
        }
    }
    vpop();
    vpop();
}
unsafe extern "C" fn adjust_bf(
    mut sv: *mut SValue,
    mut bit_pos: std::os::raw::c_int,
    mut bit_size: std::os::raw::c_int,
) -> std::os::raw::c_int {
    let mut t: std::os::raw::c_int = 0;
    if (*sv).type_0.ref_0.is_null() {
        return 0 as std::os::raw::c_int;
    }
    t = (*(*sv).type_0.ref_0)
        .c2rust_unnamed
        .c2rust_unnamed
        .c2rust_unnamed
        .auxtype;
    if t != -(1 as std::os::raw::c_int) && t != 7 as std::os::raw::c_int {
        (*sv).type_0.t =
            (*sv).type_0.t & !(0xf as std::os::raw::c_int | 0x800 as std::os::raw::c_int) | t;
        (*sv).r = ((*sv).r as std::os::raw::c_int | 0x100 as std::os::raw::c_int)
            as std::os::raw::c_ushort
    }
    return t;
}
/* store vtop a register belonging to class 'rc'. lvalues are
converted to values. Cannot be used if cannot be converted to
register value (such as structures). */
#[no_mangle]
pub unsafe extern "C" fn gv(mut rc: std::os::raw::c_int) -> std::os::raw::c_int {
    let mut r: std::os::raw::c_int = 0;
    let mut r2: std::os::raw::c_int = 0;
    let mut r_ok: std::os::raw::c_int = 0;
    let mut r2_ok: std::os::raw::c_int = 0;
    let mut rc2: std::os::raw::c_int = 0;
    let mut bt: std::os::raw::c_int = 0;
    let mut bit_pos: std::os::raw::c_int = 0;
    let mut bit_size: std::os::raw::c_int = 0;
    let mut size: std::os::raw::c_int = 0;
    let mut align: std::os::raw::c_int = 0;
    /* NOTE: get_reg can modify vstack[] */
    if (*vtop).type_0.t & 0x80 as std::os::raw::c_int != 0 {
        let mut type_0: CType = CType {
            t: 0,
            ref_0: 0 as *const Sym as *mut Sym,
        };
        bit_pos = (*vtop).type_0.t >> 20 as std::os::raw::c_int & 0x3f as std::os::raw::c_int;
        bit_size = (*vtop).type_0.t >> 20 as std::os::raw::c_int + 6 as std::os::raw::c_int
            & 0x3f as std::os::raw::c_int;
        /* remove bit field info to avoid loops */
        (*vtop).type_0.t = ((*vtop).type_0.t as std::os::raw::c_uint
            & !(((1 as std::os::raw::c_uint)
                << 6 as std::os::raw::c_int + 6 as std::os::raw::c_int)
                .wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_uint)
                << 20 as std::os::raw::c_int
                | 0x80 as std::os::raw::c_int as std::os::raw::c_uint))
            as std::os::raw::c_int;
        type_0.ref_0 = 0 as *mut Sym;
        type_0.t = (*vtop).type_0.t & 0x10 as std::os::raw::c_int;
        if (*vtop).type_0.t & 0xf as std::os::raw::c_int == 11 as std::os::raw::c_int {
            type_0.t |= 0x10 as std::os::raw::c_int
        }
        r = adjust_bf(vtop, bit_pos, bit_size);
        if (*vtop).type_0.t & 0xf as std::os::raw::c_int == 4 as std::os::raw::c_int {
            type_0.t |= 4 as std::os::raw::c_int
        } else {
            type_0.t |= 3 as std::os::raw::c_int
        }
        if r == 7 as std::os::raw::c_int {
            load_packed_bf(&mut type_0, bit_pos, bit_size);
        } else {
            let mut bits: std::os::raw::c_int =
                if type_0.t & 0xf as std::os::raw::c_int == 4 as std::os::raw::c_int {
                    64 as std::os::raw::c_int
                } else {
                    32 as std::os::raw::c_int
                };
            /* cast to int to propagate signedness in following ops */
            gen_cast(&mut type_0);
            /* generate shifts */
            vpushi(bits - (bit_pos + bit_size));
            gen_op('<' as i32);
            vpushi(bits - bit_size);
            /* NOTE: transformed to SHR if unsigned */
            gen_op('>' as i32);
        }
        r = gv(rc)
    } else {
        if is_float((*vtop).type_0.t) != 0
            && (*vtop).r as std::os::raw::c_int
                & (0x3f as std::os::raw::c_int | 0x100 as std::os::raw::c_int)
                == 0x30 as std::os::raw::c_int
        {
            /* CPUs usually cannot use float constants, so we store them
            generically in data segment */
            let mut p: init_params = {
                let mut init = init_params {
                    sec: (*tcc_state).rodata_section,
                    local_offset: 0,
                    flex_array_ref: 0 as *mut Sym,
                };
                init
            };
            let mut offset: std::os::raw::c_ulong = 0;
            size = type_size(&mut (*vtop).type_0, &mut align);
            if nocode_wanted > 0 as std::os::raw::c_int {
                size = 0 as std::os::raw::c_int;
                align = 1 as std::os::raw::c_int
            }
            offset = section_add(p.sec, size as Elf64_Addr, align);
            vpush_ref(
                &mut (*vtop).type_0,
                p.sec,
                offset,
                size as std::os::raw::c_ulong,
            );
            vswap();
            init_putv(&mut p, &mut (*vtop).type_0, offset);
            (*vtop).r = ((*vtop).r as std::os::raw::c_int | 0x100 as std::os::raw::c_int)
                as std::os::raw::c_ushort
        }
        if (*vtop).r as std::os::raw::c_int & 0x4000 as std::os::raw::c_int != 0 {
            gbound();
        }
        bt = (*vtop).type_0.t & 0xf as std::os::raw::c_int;
        rc2 = RC2_TYPE(bt, rc);
        /* need to reload if:
        - constant
        - lvalue (need to dereference pointer)
        - already a register, but not in the right class */
        r = (*vtop).r as std::os::raw::c_int & 0x3f as std::os::raw::c_int;
        r_ok = ((*vtop).r as std::os::raw::c_int & 0x100 as std::os::raw::c_int == 0
            && r < 0x30 as std::os::raw::c_int
            && reg_classes[r as usize] & rc != 0) as std::os::raw::c_int;
        r2_ok = (rc2 == 0
            || ((*vtop).r2 as std::os::raw::c_int) < 0x30 as std::os::raw::c_int
                && reg_classes[(*vtop).r2 as usize] & rc2 != 0)
            as std::os::raw::c_int;
        if r_ok == 0 || r2_ok == 0 {
            if r_ok == 0 {
                r = get_reg(rc)
            }
            if rc2 != 0 {
                let mut current_block_67: u64;
                let mut load_type: std::os::raw::c_int = if bt == 14 as std::os::raw::c_int {
                    9 as std::os::raw::c_int
                } else {
                    (0x800 as std::os::raw::c_int) | 4 as std::os::raw::c_int
                };
                let mut original_type: std::os::raw::c_int = (*vtop).type_0.t;
                /* two register type load :
                expand to two words temporarily */
                if (*vtop).r as std::os::raw::c_int
                    & (0x3f as std::os::raw::c_int | 0x100 as std::os::raw::c_int)
                    == 0x30 as std::os::raw::c_int
                {
                    /* load constant */
                    let mut ll: std::os::raw::c_ulonglong =
                        (*vtop).c2rust_unnamed.c.i as std::os::raw::c_ulonglong; /* first word */
                    (*vtop).c2rust_unnamed.c.i = ll as uint64_t; /* save register value */
                    load(r, vtop);
                    (*vtop).r = r as std::os::raw::c_ushort;
                    vpushi((ll >> 32 as std::os::raw::c_int) as std::os::raw::c_int);
                    current_block_67 = 15594839951440953787;
                } else if (*vtop).r as std::os::raw::c_int & 0x100 as std::os::raw::c_int != 0 {
                    /* We do not want to modifier the long long pointer here.
                    So we save any other instances down the stack */
                    save_reg_upstack((*vtop).r as std::os::raw::c_int, 1 as std::os::raw::c_int);
                    /* load from memory */
                    (*vtop).type_0.t = load_type; /* save register value */
                    load(r, vtop);
                    vdup();
                    (*vtop.offset(-(1 as std::os::raw::c_int) as isize)).r =
                        r as std::os::raw::c_ushort;
                    /* increment pointer to get second word */
                    (*vtop).type_0.t = 0x800 as std::os::raw::c_int | 4 as std::os::raw::c_int;
                    gaddrof();
                    vpushs(8 as std::os::raw::c_int as Elf64_Addr);
                    gen_op('+' as i32);
                    (*vtop).r = ((*vtop).r as std::os::raw::c_int | 0x100 as std::os::raw::c_int)
                        as std::os::raw::c_ushort;
                    (*vtop).type_0.t = load_type;
                    current_block_67 = 15594839951440953787;
                } else {
                    /* move registers */
                    if r_ok == 0 {
                        load(r, vtop); /* save register value */
                    }
                    if r2_ok != 0
                        && ((*vtop).r2 as std::os::raw::c_int) < 0x30 as std::os::raw::c_int
                    {
                        current_block_67 = 888896267959736133;
                    } else {
                        vdup();
                        (*vtop.offset(-(1 as std::os::raw::c_int) as isize)).r =
                            r as std::os::raw::c_ushort;
                        (*vtop).r = (*vtop.offset(-(1 as std::os::raw::c_int) as isize)).r2;
                        current_block_67 = 15594839951440953787;
                    }
                }
                match current_block_67 {
                    15594839951440953787 => {
                        /* Allocate second register. Here we rely on the fact that
                        get_reg() tries first to free r2 of an SValue. */
                        r2 = get_reg(rc2);
                        load(r2, vtop);
                        vpop();
                        /* write second register */
                        (*vtop).r2 = r2 as std::os::raw::c_ushort
                    },
                    _ => {},
                }
                (*vtop).type_0.t = original_type
            } else {
                if (*vtop).r as std::os::raw::c_int == 0x33 as std::os::raw::c_int {
                    vset_VT_JMP();
                }
                /* one register type load */
                load(r, vtop);
            }
        }
        (*vtop).r = r as std::os::raw::c_ushort
    }
    return r;
}
/* generate vtop[-1] and vtop[0] in resp. classes rc1 and rc2 */
#[no_mangle]
pub unsafe extern "C" fn gv2(mut rc1: std::os::raw::c_int, mut rc2: std::os::raw::c_int) {
    /* generate more generic register first. But VT_JMP or VT_CMP
    values must be generated first in all cases to avoid possible
    reload errors */
    if (*vtop).r as std::os::raw::c_int != 0x33 as std::os::raw::c_int && rc1 <= rc2 {
        vswap();
        gv(rc1);
        vswap();
        gv(rc2);
        /* test if reload is needed for first register */
        if (*vtop.offset(-(1 as std::os::raw::c_int) as isize)).r as std::os::raw::c_int
            & 0x3f as std::os::raw::c_int
            >= 0x30 as std::os::raw::c_int
        {
            vswap();
            gv(rc1);
            vswap();
        }
    } else {
        gv(rc2);
        vswap();
        gv(rc1);
        vswap();
        /* test if reload is needed for first register */
        if (*vtop.offset(0 as std::os::raw::c_int as isize)).r as std::os::raw::c_int
            & 0x3f as std::os::raw::c_int
            >= 0x30 as std::os::raw::c_int
        {
            gv(rc2);
        }
    };
}
/* convert stack entry to register and duplicate its value in another
register */
unsafe extern "C" fn gv_dup() {
    let mut t: std::os::raw::c_int = 0;
    let mut rc: std::os::raw::c_int = 0;
    let mut r: std::os::raw::c_int = 0;
    t = (*vtop).type_0.t;
    /* duplicate value */
    rc = RC_TYPE(t);
    gv(rc);
    r = get_reg(rc);
    vdup();
    load(r, vtop);
    (*vtop).r = r as std::os::raw::c_ushort;
}
unsafe extern "C" fn gen_opic_sdiv(mut a: uint64_t, mut b: uint64_t) -> uint64_t {
    let mut x: uint64_t = (if a >> 63 as std::os::raw::c_int != 0 {
        a.wrapping_neg()
    } else {
        a
    })
    .wrapping_div(
        (if b >> 63 as std::os::raw::c_int != 0 {
            b.wrapping_neg()
        } else {
            b
        }),
    );
    return if (a ^ b) >> 63 as std::os::raw::c_int != 0 {
        x.wrapping_neg()
    } else {
        x
    };
}
unsafe extern "C" fn gen_opic_lt(mut a: uint64_t, mut b: uint64_t) -> std::os::raw::c_int {
    return ((a ^ (1 as std::os::raw::c_int as uint64_t) << 63 as std::os::raw::c_int)
        < b ^ (1 as std::os::raw::c_int as uint64_t) << 63 as std::os::raw::c_int)
        as std::os::raw::c_int;
}
/* handle integer constant optimizations and various machine
independent opt */
unsafe extern "C" fn gen_opic(mut op: std::os::raw::c_int) {
    let mut current_block: u64;
    let mut v1: *mut SValue = vtop.offset(-(1 as std::os::raw::c_int as isize));
    let mut v2: *mut SValue = vtop;
    let mut t1: std::os::raw::c_int = (*v1).type_0.t & 0xf as std::os::raw::c_int;
    let mut t2: std::os::raw::c_int = (*v2).type_0.t & 0xf as std::os::raw::c_int;
    let mut c1: std::os::raw::c_int = ((*v1).r as std::os::raw::c_int
        & (0x3f as std::os::raw::c_int
            | 0x100 as std::os::raw::c_int
            | 0x200 as std::os::raw::c_int)
        == 0x30 as std::os::raw::c_int)
        as std::os::raw::c_int;
    let mut c2: std::os::raw::c_int = ((*v2).r as std::os::raw::c_int
        & (0x3f as std::os::raw::c_int
            | 0x100 as std::os::raw::c_int
            | 0x200 as std::os::raw::c_int)
        == 0x30 as std::os::raw::c_int)
        as std::os::raw::c_int;
    let mut l1: uint64_t = if c1 != 0 {
        (*v1).c2rust_unnamed.c.i
    } else {
        0 as std::os::raw::c_int as std::os::raw::c_ulong
    };
    let mut l2: uint64_t = if c2 != 0 {
        (*v2).c2rust_unnamed.c.i
    } else {
        0 as std::os::raw::c_int as std::os::raw::c_ulong
    };
    let mut shm: std::os::raw::c_int = if t1 == 4 as std::os::raw::c_int {
        63 as std::os::raw::c_int
    } else {
        31 as std::os::raw::c_int
    };
    if t1 != 4 as std::os::raw::c_int
        && (8 as std::os::raw::c_int != 8 as std::os::raw::c_int || t1 != 5 as std::os::raw::c_int)
    {
        l1 = l1 as uint32_t as std::os::raw::c_ulong
            | (if (*v1).type_0.t & 0x10 as std::os::raw::c_int != 0 {
                0 as std::os::raw::c_int as std::os::raw::c_ulong
            } else {
                (l1 & 0x80000000 as std::os::raw::c_uint as std::os::raw::c_ulong).wrapping_neg()
            })
    }
    if t2 != 4 as std::os::raw::c_int
        && (8 as std::os::raw::c_int != 8 as std::os::raw::c_int || t2 != 5 as std::os::raw::c_int)
    {
        l2 = l2 as uint32_t as std::os::raw::c_ulong
            | (if (*v2).type_0.t & 0x10 as std::os::raw::c_int != 0 {
                0 as std::os::raw::c_int as std::os::raw::c_ulong
            } else {
                (l2 & 0x80000000 as std::os::raw::c_uint as std::os::raw::c_ulong).wrapping_neg()
            })
    }
    if c1 != 0 && c2 != 0 {
        match op {
            43 => {
                current_block = 337463635748514786;
                match current_block {
                    9531126118564703076 => {
                        l1 = (l1 != 0 || l2 != 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    337463635748514786 => {
                        l1 = (l1 as std::os::raw::c_ulong).wrapping_add(l2) as uint64_t as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    14513523936503887211 => {
                        l1 = (l1 as std::os::raw::c_ulong).wrapping_sub(l2) as uint64_t as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    13264933720371784297 => {
                        l1 &= l2;
                        current_block = 2116367355679836638;
                    },
                    6266472726643588263 => {
                        l1 ^= l2;
                        current_block = 2116367355679836638;
                    },
                    4912292479518253705 => {
                        l1 |= l2;
                        current_block = 2116367355679836638;
                    },
                    6484990028451387192 => {
                        l1 = (l1 as std::os::raw::c_ulong).wrapping_mul(l2) as uint64_t as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    15319502457978536222 => {
                        l1 <<= l2 & shm as std::os::raw::c_ulong;
                        current_block = 2116367355679836638;
                    },
                    7644865018902103128 => {
                        l1 >>= l2 & shm as std::os::raw::c_ulong;
                        current_block = 2116367355679836638;
                    },
                    14360337777247149493 => {
                        l1 = if l1 >> 63 as std::os::raw::c_int != 0 {
                            !(!l1 >> (l2 & shm as std::os::raw::c_ulong))
                        } else {
                            (l1) >> (l2 & shm as std::os::raw::c_ulong)
                        };
                        current_block = 2116367355679836638;
                    },
                    7983287325140408908 =>
                    /* tests */
                    {
                        l1 = (l1 < l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    }
                    11798621572665121461 => {
                        l1 = (l1 >= l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    13932873228663550769 => {
                        l1 = (l1 == l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    1788142895051060034 => {
                        l1 = (l1 != l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    9886615242100325262 => {
                        l1 = (l1 <= l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    2041506142873075795 => {
                        l1 = (l1 > l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    6863629183173758772 => {
                        l1 = gen_opic_lt(l1, l2) as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    8913536887710889399 => {
                        l1 = (gen_opic_lt(l1, l2) == 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    4631372686411729056 => {
                        l1 = (gen_opic_lt(l2, l1) == 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    11170670477888610567 => {
                        l1 = gen_opic_lt(l2, l1) as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    18380056810314386699 =>
                    /* logical */
                    {
                        l1 = (l1 != 0 && l2 != 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    }
                    _ =>
                    /* if division by zero, generate explicit division */
                    {
                        if l2 == 0 as std::os::raw::c_int as std::os::raw::c_ulong {
                            if const_wanted != 0
                                && nocode_wanted & 0xffff as std::os::raw::c_int == 0
                            {
                                _tcc_error(
                                    b"division by zero in constant\x00" as *const u8
                                        as *const std::os::raw::c_char,
                                );
                            }
                            current_block = 10869693750139516408;
                        } else {
                            match op {
                                37 => l1 = l1.wrapping_sub(l2.wrapping_mul(gen_opic_sdiv(l1, l2))),
                                131 => l1 = l1.wrapping_div(l2),
                                132 => l1 = l1.wrapping_rem(l2),
                                _ => l1 = gen_opic_sdiv(l1, l2),
                            }
                            current_block = 2116367355679836638;
                        }
                    },
                }
                match current_block {
                    10869693750139516408 => {},
                    _ => {
                        if t1 != 4 as std::os::raw::c_int
                            && (8 as std::os::raw::c_int != 8 as std::os::raw::c_int
                                || t1 != 5 as std::os::raw::c_int)
                        {
                            l1 = l1 as uint32_t as std::os::raw::c_ulong
                                | (if (*v1).type_0.t & 0x10 as std::os::raw::c_int != 0 {
                                    0 as std::os::raw::c_int as std::os::raw::c_ulong
                                } else {
                                    (l1 & 0x80000000 as std::os::raw::c_uint
                                        as std::os::raw::c_ulong)
                                        .wrapping_neg()
                                })
                        }
                        (*v1).c2rust_unnamed.c.i = l1;
                        vtop = vtop.offset(-1);
                        current_block = 2705889988320590074;
                    },
                }
            },
            45 => {
                current_block = 14513523936503887211;
                match current_block {
                    9531126118564703076 => {
                        l1 = (l1 != 0 || l2 != 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    337463635748514786 => {
                        l1 = (l1 as std::os::raw::c_ulong).wrapping_add(l2) as uint64_t as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    14513523936503887211 => {
                        l1 = (l1 as std::os::raw::c_ulong).wrapping_sub(l2) as uint64_t as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    13264933720371784297 => {
                        l1 &= l2;
                        current_block = 2116367355679836638;
                    },
                    6266472726643588263 => {
                        l1 ^= l2;
                        current_block = 2116367355679836638;
                    },
                    4912292479518253705 => {
                        l1 |= l2;
                        current_block = 2116367355679836638;
                    },
                    6484990028451387192 => {
                        l1 = (l1 as std::os::raw::c_ulong).wrapping_mul(l2) as uint64_t as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    15319502457978536222 => {
                        l1 <<= l2 & shm as std::os::raw::c_ulong;
                        current_block = 2116367355679836638;
                    },
                    7644865018902103128 => {
                        l1 >>= l2 & shm as std::os::raw::c_ulong;
                        current_block = 2116367355679836638;
                    },
                    14360337777247149493 => {
                        l1 = if l1 >> 63 as std::os::raw::c_int != 0 {
                            !(!l1 >> (l2 & shm as std::os::raw::c_ulong))
                        } else {
                            (l1) >> (l2 & shm as std::os::raw::c_ulong)
                        };
                        current_block = 2116367355679836638;
                    },
                    7983287325140408908 => {
                        l1 = (l1 < l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    11798621572665121461 => {
                        l1 = (l1 >= l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    13932873228663550769 => {
                        l1 = (l1 == l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    1788142895051060034 => {
                        l1 = (l1 != l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    9886615242100325262 => {
                        l1 = (l1 <= l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    2041506142873075795 => {
                        l1 = (l1 > l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    6863629183173758772 => {
                        l1 = gen_opic_lt(l1, l2) as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    8913536887710889399 => {
                        l1 = (gen_opic_lt(l1, l2) == 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    4631372686411729056 => {
                        l1 = (gen_opic_lt(l2, l1) == 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    11170670477888610567 => {
                        l1 = gen_opic_lt(l2, l1) as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    18380056810314386699 => {
                        l1 = (l1 != 0 && l2 != 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    _ => {
                        if l2 == 0 as std::os::raw::c_int as std::os::raw::c_ulong {
                            if const_wanted != 0
                                && nocode_wanted & 0xffff as std::os::raw::c_int == 0
                            {
                                _tcc_error(
                                    b"division by zero in constant\x00" as *const u8
                                        as *const std::os::raw::c_char,
                                );
                            }
                            current_block = 10869693750139516408;
                        } else {
                            match op {
                                37 => l1 = l1.wrapping_sub(l2.wrapping_mul(gen_opic_sdiv(l1, l2))),
                                131 => l1 = l1.wrapping_div(l2),
                                132 => l1 = l1.wrapping_rem(l2),
                                _ => l1 = gen_opic_sdiv(l1, l2),
                            }
                            current_block = 2116367355679836638;
                        }
                    },
                }
                match current_block {
                    10869693750139516408 => {},
                    _ => {
                        if t1 != 4 as std::os::raw::c_int
                            && (8 as std::os::raw::c_int != 8 as std::os::raw::c_int
                                || t1 != 5 as std::os::raw::c_int)
                        {
                            l1 = l1 as uint32_t as std::os::raw::c_ulong
                                | (if (*v1).type_0.t & 0x10 as std::os::raw::c_int != 0 {
                                    0 as std::os::raw::c_int as std::os::raw::c_ulong
                                } else {
                                    (l1 & 0x80000000 as std::os::raw::c_uint
                                        as std::os::raw::c_ulong)
                                        .wrapping_neg()
                                })
                        }
                        (*v1).c2rust_unnamed.c.i = l1;
                        vtop = vtop.offset(-1);
                        current_block = 2705889988320590074;
                    },
                }
            },
            38 => {
                current_block = 13264933720371784297;
                match current_block {
                    9531126118564703076 => {
                        l1 = (l1 != 0 || l2 != 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    337463635748514786 => {
                        l1 = (l1 as std::os::raw::c_ulong).wrapping_add(l2) as uint64_t as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    14513523936503887211 => {
                        l1 = (l1 as std::os::raw::c_ulong).wrapping_sub(l2) as uint64_t as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    13264933720371784297 => {
                        l1 &= l2;
                        current_block = 2116367355679836638;
                    },
                    6266472726643588263 => {
                        l1 ^= l2;
                        current_block = 2116367355679836638;
                    },
                    4912292479518253705 => {
                        l1 |= l2;
                        current_block = 2116367355679836638;
                    },
                    6484990028451387192 => {
                        l1 = (l1 as std::os::raw::c_ulong).wrapping_mul(l2) as uint64_t as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    15319502457978536222 => {
                        l1 <<= l2 & shm as std::os::raw::c_ulong;
                        current_block = 2116367355679836638;
                    },
                    7644865018902103128 => {
                        l1 >>= l2 & shm as std::os::raw::c_ulong;
                        current_block = 2116367355679836638;
                    },
                    14360337777247149493 => {
                        l1 = if l1 >> 63 as std::os::raw::c_int != 0 {
                            !(!l1 >> (l2 & shm as std::os::raw::c_ulong))
                        } else {
                            (l1) >> (l2 & shm as std::os::raw::c_ulong)
                        };
                        current_block = 2116367355679836638;
                    },
                    7983287325140408908 => {
                        l1 = (l1 < l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    11798621572665121461 => {
                        l1 = (l1 >= l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    13932873228663550769 => {
                        l1 = (l1 == l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    1788142895051060034 => {
                        l1 = (l1 != l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    9886615242100325262 => {
                        l1 = (l1 <= l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    2041506142873075795 => {
                        l1 = (l1 > l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    6863629183173758772 => {
                        l1 = gen_opic_lt(l1, l2) as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    8913536887710889399 => {
                        l1 = (gen_opic_lt(l1, l2) == 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    4631372686411729056 => {
                        l1 = (gen_opic_lt(l2, l1) == 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    11170670477888610567 => {
                        l1 = gen_opic_lt(l2, l1) as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    18380056810314386699 => {
                        l1 = (l1 != 0 && l2 != 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    _ => {
                        if l2 == 0 as std::os::raw::c_int as std::os::raw::c_ulong {
                            if const_wanted != 0
                                && nocode_wanted & 0xffff as std::os::raw::c_int == 0
                            {
                                _tcc_error(
                                    b"division by zero in constant\x00" as *const u8
                                        as *const std::os::raw::c_char,
                                );
                            }
                            current_block = 10869693750139516408;
                        } else {
                            match op {
                                37 => l1 = l1.wrapping_sub(l2.wrapping_mul(gen_opic_sdiv(l1, l2))),
                                131 => l1 = l1.wrapping_div(l2),
                                132 => l1 = l1.wrapping_rem(l2),
                                _ => l1 = gen_opic_sdiv(l1, l2),
                            }
                            current_block = 2116367355679836638;
                        }
                    },
                }
                match current_block {
                    10869693750139516408 => {},
                    _ => {
                        if t1 != 4 as std::os::raw::c_int
                            && (8 as std::os::raw::c_int != 8 as std::os::raw::c_int
                                || t1 != 5 as std::os::raw::c_int)
                        {
                            l1 = l1 as uint32_t as std::os::raw::c_ulong
                                | (if (*v1).type_0.t & 0x10 as std::os::raw::c_int != 0 {
                                    0 as std::os::raw::c_int as std::os::raw::c_ulong
                                } else {
                                    (l1 & 0x80000000 as std::os::raw::c_uint
                                        as std::os::raw::c_ulong)
                                        .wrapping_neg()
                                })
                        }
                        (*v1).c2rust_unnamed.c.i = l1;
                        vtop = vtop.offset(-1);
                        current_block = 2705889988320590074;
                    },
                }
            },
            94 => {
                current_block = 6266472726643588263;
                match current_block {
                    9531126118564703076 => {
                        l1 = (l1 != 0 || l2 != 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    337463635748514786 => {
                        l1 = (l1 as std::os::raw::c_ulong).wrapping_add(l2) as uint64_t as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    14513523936503887211 => {
                        l1 = (l1 as std::os::raw::c_ulong).wrapping_sub(l2) as uint64_t as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    13264933720371784297 => {
                        l1 &= l2;
                        current_block = 2116367355679836638;
                    },
                    6266472726643588263 => {
                        l1 ^= l2;
                        current_block = 2116367355679836638;
                    },
                    4912292479518253705 => {
                        l1 |= l2;
                        current_block = 2116367355679836638;
                    },
                    6484990028451387192 => {
                        l1 = (l1 as std::os::raw::c_ulong).wrapping_mul(l2) as uint64_t as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    15319502457978536222 => {
                        l1 <<= l2 & shm as std::os::raw::c_ulong;
                        current_block = 2116367355679836638;
                    },
                    7644865018902103128 => {
                        l1 >>= l2 & shm as std::os::raw::c_ulong;
                        current_block = 2116367355679836638;
                    },
                    14360337777247149493 => {
                        l1 = if l1 >> 63 as std::os::raw::c_int != 0 {
                            !(!l1 >> (l2 & shm as std::os::raw::c_ulong))
                        } else {
                            (l1) >> (l2 & shm as std::os::raw::c_ulong)
                        };
                        current_block = 2116367355679836638;
                    },
                    7983287325140408908 => {
                        l1 = (l1 < l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    11798621572665121461 => {
                        l1 = (l1 >= l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    13932873228663550769 => {
                        l1 = (l1 == l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    1788142895051060034 => {
                        l1 = (l1 != l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    9886615242100325262 => {
                        l1 = (l1 <= l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    2041506142873075795 => {
                        l1 = (l1 > l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    6863629183173758772 => {
                        l1 = gen_opic_lt(l1, l2) as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    8913536887710889399 => {
                        l1 = (gen_opic_lt(l1, l2) == 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    4631372686411729056 => {
                        l1 = (gen_opic_lt(l2, l1) == 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    11170670477888610567 => {
                        l1 = gen_opic_lt(l2, l1) as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    18380056810314386699 => {
                        l1 = (l1 != 0 && l2 != 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    _ => {
                        if l2 == 0 as std::os::raw::c_int as std::os::raw::c_ulong {
                            if const_wanted != 0
                                && nocode_wanted & 0xffff as std::os::raw::c_int == 0
                            {
                                _tcc_error(
                                    b"division by zero in constant\x00" as *const u8
                                        as *const std::os::raw::c_char,
                                );
                            }
                            current_block = 10869693750139516408;
                        } else {
                            match op {
                                37 => l1 = l1.wrapping_sub(l2.wrapping_mul(gen_opic_sdiv(l1, l2))),
                                131 => l1 = l1.wrapping_div(l2),
                                132 => l1 = l1.wrapping_rem(l2),
                                _ => l1 = gen_opic_sdiv(l1, l2),
                            }
                            current_block = 2116367355679836638;
                        }
                    },
                }
                match current_block {
                    10869693750139516408 => {},
                    _ => {
                        if t1 != 4 as std::os::raw::c_int
                            && (8 as std::os::raw::c_int != 8 as std::os::raw::c_int
                                || t1 != 5 as std::os::raw::c_int)
                        {
                            l1 = l1 as uint32_t as std::os::raw::c_ulong
                                | (if (*v1).type_0.t & 0x10 as std::os::raw::c_int != 0 {
                                    0 as std::os::raw::c_int as std::os::raw::c_ulong
                                } else {
                                    (l1 & 0x80000000 as std::os::raw::c_uint
                                        as std::os::raw::c_ulong)
                                        .wrapping_neg()
                                })
                        }
                        (*v1).c2rust_unnamed.c.i = l1;
                        vtop = vtop.offset(-1);
                        current_block = 2705889988320590074;
                    },
                }
            },
            124 => {
                current_block = 4912292479518253705;
                match current_block {
                    9531126118564703076 => {
                        l1 = (l1 != 0 || l2 != 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    337463635748514786 => {
                        l1 = (l1 as std::os::raw::c_ulong).wrapping_add(l2) as uint64_t as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    14513523936503887211 => {
                        l1 = (l1 as std::os::raw::c_ulong).wrapping_sub(l2) as uint64_t as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    13264933720371784297 => {
                        l1 &= l2;
                        current_block = 2116367355679836638;
                    },
                    6266472726643588263 => {
                        l1 ^= l2;
                        current_block = 2116367355679836638;
                    },
                    4912292479518253705 => {
                        l1 |= l2;
                        current_block = 2116367355679836638;
                    },
                    6484990028451387192 => {
                        l1 = (l1 as std::os::raw::c_ulong).wrapping_mul(l2) as uint64_t as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    15319502457978536222 => {
                        l1 <<= l2 & shm as std::os::raw::c_ulong;
                        current_block = 2116367355679836638;
                    },
                    7644865018902103128 => {
                        l1 >>= l2 & shm as std::os::raw::c_ulong;
                        current_block = 2116367355679836638;
                    },
                    14360337777247149493 => {
                        l1 = if l1 >> 63 as std::os::raw::c_int != 0 {
                            !(!l1 >> (l2 & shm as std::os::raw::c_ulong))
                        } else {
                            (l1) >> (l2 & shm as std::os::raw::c_ulong)
                        };
                        current_block = 2116367355679836638;
                    },
                    7983287325140408908 => {
                        l1 = (l1 < l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    11798621572665121461 => {
                        l1 = (l1 >= l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    13932873228663550769 => {
                        l1 = (l1 == l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    1788142895051060034 => {
                        l1 = (l1 != l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    9886615242100325262 => {
                        l1 = (l1 <= l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    2041506142873075795 => {
                        l1 = (l1 > l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    6863629183173758772 => {
                        l1 = gen_opic_lt(l1, l2) as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    8913536887710889399 => {
                        l1 = (gen_opic_lt(l1, l2) == 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    4631372686411729056 => {
                        l1 = (gen_opic_lt(l2, l1) == 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    11170670477888610567 => {
                        l1 = gen_opic_lt(l2, l1) as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    18380056810314386699 => {
                        l1 = (l1 != 0 && l2 != 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    _ => {
                        if l2 == 0 as std::os::raw::c_int as std::os::raw::c_ulong {
                            if const_wanted != 0
                                && nocode_wanted & 0xffff as std::os::raw::c_int == 0
                            {
                                _tcc_error(
                                    b"division by zero in constant\x00" as *const u8
                                        as *const std::os::raw::c_char,
                                );
                            }
                            current_block = 10869693750139516408;
                        } else {
                            match op {
                                37 => l1 = l1.wrapping_sub(l2.wrapping_mul(gen_opic_sdiv(l1, l2))),
                                131 => l1 = l1.wrapping_div(l2),
                                132 => l1 = l1.wrapping_rem(l2),
                                _ => l1 = gen_opic_sdiv(l1, l2),
                            }
                            current_block = 2116367355679836638;
                        }
                    },
                }
                match current_block {
                    10869693750139516408 => {},
                    _ => {
                        if t1 != 4 as std::os::raw::c_int
                            && (8 as std::os::raw::c_int != 8 as std::os::raw::c_int
                                || t1 != 5 as std::os::raw::c_int)
                        {
                            l1 = l1 as uint32_t as std::os::raw::c_ulong
                                | (if (*v1).type_0.t & 0x10 as std::os::raw::c_int != 0 {
                                    0 as std::os::raw::c_int as std::os::raw::c_ulong
                                } else {
                                    (l1 & 0x80000000 as std::os::raw::c_uint
                                        as std::os::raw::c_ulong)
                                        .wrapping_neg()
                                })
                        }
                        (*v1).c2rust_unnamed.c.i = l1;
                        vtop = vtop.offset(-1);
                        current_block = 2705889988320590074;
                    },
                }
            },
            42 => {
                current_block = 6484990028451387192;
                match current_block {
                    9531126118564703076 => {
                        l1 = (l1 != 0 || l2 != 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    337463635748514786 => {
                        l1 = (l1 as std::os::raw::c_ulong).wrapping_add(l2) as uint64_t as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    14513523936503887211 => {
                        l1 = (l1 as std::os::raw::c_ulong).wrapping_sub(l2) as uint64_t as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    13264933720371784297 => {
                        l1 &= l2;
                        current_block = 2116367355679836638;
                    },
                    6266472726643588263 => {
                        l1 ^= l2;
                        current_block = 2116367355679836638;
                    },
                    4912292479518253705 => {
                        l1 |= l2;
                        current_block = 2116367355679836638;
                    },
                    6484990028451387192 => {
                        l1 = (l1 as std::os::raw::c_ulong).wrapping_mul(l2) as uint64_t as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    15319502457978536222 => {
                        l1 <<= l2 & shm as std::os::raw::c_ulong;
                        current_block = 2116367355679836638;
                    },
                    7644865018902103128 => {
                        l1 >>= l2 & shm as std::os::raw::c_ulong;
                        current_block = 2116367355679836638;
                    },
                    14360337777247149493 => {
                        l1 = if l1 >> 63 as std::os::raw::c_int != 0 {
                            !(!l1 >> (l2 & shm as std::os::raw::c_ulong))
                        } else {
                            (l1) >> (l2 & shm as std::os::raw::c_ulong)
                        };
                        current_block = 2116367355679836638;
                    },
                    7983287325140408908 => {
                        l1 = (l1 < l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    11798621572665121461 => {
                        l1 = (l1 >= l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    13932873228663550769 => {
                        l1 = (l1 == l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    1788142895051060034 => {
                        l1 = (l1 != l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    9886615242100325262 => {
                        l1 = (l1 <= l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    2041506142873075795 => {
                        l1 = (l1 > l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    6863629183173758772 => {
                        l1 = gen_opic_lt(l1, l2) as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    8913536887710889399 => {
                        l1 = (gen_opic_lt(l1, l2) == 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    4631372686411729056 => {
                        l1 = (gen_opic_lt(l2, l1) == 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    11170670477888610567 => {
                        l1 = gen_opic_lt(l2, l1) as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    18380056810314386699 => {
                        l1 = (l1 != 0 && l2 != 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    _ => {
                        if l2 == 0 as std::os::raw::c_int as std::os::raw::c_ulong {
                            if const_wanted != 0
                                && nocode_wanted & 0xffff as std::os::raw::c_int == 0
                            {
                                _tcc_error(
                                    b"division by zero in constant\x00" as *const u8
                                        as *const std::os::raw::c_char,
                                );
                            }
                            current_block = 10869693750139516408;
                        } else {
                            match op {
                                37 => l1 = l1.wrapping_sub(l2.wrapping_mul(gen_opic_sdiv(l1, l2))),
                                131 => l1 = l1.wrapping_div(l2),
                                132 => l1 = l1.wrapping_rem(l2),
                                _ => l1 = gen_opic_sdiv(l1, l2),
                            }
                            current_block = 2116367355679836638;
                        }
                    },
                }
                match current_block {
                    10869693750139516408 => {},
                    _ => {
                        if t1 != 4 as std::os::raw::c_int
                            && (8 as std::os::raw::c_int != 8 as std::os::raw::c_int
                                || t1 != 5 as std::os::raw::c_int)
                        {
                            l1 = l1 as uint32_t as std::os::raw::c_ulong
                                | (if (*v1).type_0.t & 0x10 as std::os::raw::c_int != 0 {
                                    0 as std::os::raw::c_int as std::os::raw::c_ulong
                                } else {
                                    (l1 & 0x80000000 as std::os::raw::c_uint
                                        as std::os::raw::c_ulong)
                                        .wrapping_neg()
                                })
                        }
                        (*v1).c2rust_unnamed.c.i = l1;
                        vtop = vtop.offset(-1);
                        current_block = 2705889988320590074;
                    },
                }
            },
            133 | 47 | 37 | 131 | 132 => {
                current_block = 613604850155172999;
                match current_block {
                    9531126118564703076 => {
                        l1 = (l1 != 0 || l2 != 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    337463635748514786 => {
                        l1 = (l1 as std::os::raw::c_ulong).wrapping_add(l2) as uint64_t as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    14513523936503887211 => {
                        l1 = (l1 as std::os::raw::c_ulong).wrapping_sub(l2) as uint64_t as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    13264933720371784297 => {
                        l1 &= l2;
                        current_block = 2116367355679836638;
                    },
                    6266472726643588263 => {
                        l1 ^= l2;
                        current_block = 2116367355679836638;
                    },
                    4912292479518253705 => {
                        l1 |= l2;
                        current_block = 2116367355679836638;
                    },
                    6484990028451387192 => {
                        l1 = (l1 as std::os::raw::c_ulong).wrapping_mul(l2) as uint64_t as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    15319502457978536222 => {
                        l1 <<= l2 & shm as std::os::raw::c_ulong;
                        current_block = 2116367355679836638;
                    },
                    7644865018902103128 => {
                        l1 >>= l2 & shm as std::os::raw::c_ulong;
                        current_block = 2116367355679836638;
                    },
                    14360337777247149493 => {
                        l1 = if l1 >> 63 as std::os::raw::c_int != 0 {
                            !(!l1 >> (l2 & shm as std::os::raw::c_ulong))
                        } else {
                            (l1) >> (l2 & shm as std::os::raw::c_ulong)
                        };
                        current_block = 2116367355679836638;
                    },
                    7983287325140408908 => {
                        l1 = (l1 < l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    11798621572665121461 => {
                        l1 = (l1 >= l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    13932873228663550769 => {
                        l1 = (l1 == l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    1788142895051060034 => {
                        l1 = (l1 != l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    9886615242100325262 => {
                        l1 = (l1 <= l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    2041506142873075795 => {
                        l1 = (l1 > l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    6863629183173758772 => {
                        l1 = gen_opic_lt(l1, l2) as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    8913536887710889399 => {
                        l1 = (gen_opic_lt(l1, l2) == 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    4631372686411729056 => {
                        l1 = (gen_opic_lt(l2, l1) == 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    11170670477888610567 => {
                        l1 = gen_opic_lt(l2, l1) as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    18380056810314386699 => {
                        l1 = (l1 != 0 && l2 != 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    _ => {
                        if l2 == 0 as std::os::raw::c_int as std::os::raw::c_ulong {
                            if const_wanted != 0
                                && nocode_wanted & 0xffff as std::os::raw::c_int == 0
                            {
                                _tcc_error(
                                    b"division by zero in constant\x00" as *const u8
                                        as *const std::os::raw::c_char,
                                );
                            }
                            current_block = 10869693750139516408;
                        } else {
                            match op {
                                37 => l1 = l1.wrapping_sub(l2.wrapping_mul(gen_opic_sdiv(l1, l2))),
                                131 => l1 = l1.wrapping_div(l2),
                                132 => l1 = l1.wrapping_rem(l2),
                                _ => l1 = gen_opic_sdiv(l1, l2),
                            }
                            current_block = 2116367355679836638;
                        }
                    },
                }
                match current_block {
                    10869693750139516408 => {},
                    _ => {
                        if t1 != 4 as std::os::raw::c_int
                            && (8 as std::os::raw::c_int != 8 as std::os::raw::c_int
                                || t1 != 5 as std::os::raw::c_int)
                        {
                            l1 = l1 as uint32_t as std::os::raw::c_ulong
                                | (if (*v1).type_0.t & 0x10 as std::os::raw::c_int != 0 {
                                    0 as std::os::raw::c_int as std::os::raw::c_ulong
                                } else {
                                    (l1 & 0x80000000 as std::os::raw::c_uint
                                        as std::os::raw::c_ulong)
                                        .wrapping_neg()
                                })
                        }
                        (*v1).c2rust_unnamed.c.i = l1;
                        vtop = vtop.offset(-1);
                        current_block = 2705889988320590074;
                    },
                }
            },
            60 => {
                current_block = 15319502457978536222;
                match current_block {
                    9531126118564703076 => {
                        l1 = (l1 != 0 || l2 != 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    337463635748514786 => {
                        l1 = (l1 as std::os::raw::c_ulong).wrapping_add(l2) as uint64_t as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    14513523936503887211 => {
                        l1 = (l1 as std::os::raw::c_ulong).wrapping_sub(l2) as uint64_t as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    13264933720371784297 => {
                        l1 &= l2;
                        current_block = 2116367355679836638;
                    },
                    6266472726643588263 => {
                        l1 ^= l2;
                        current_block = 2116367355679836638;
                    },
                    4912292479518253705 => {
                        l1 |= l2;
                        current_block = 2116367355679836638;
                    },
                    6484990028451387192 => {
                        l1 = (l1 as std::os::raw::c_ulong).wrapping_mul(l2) as uint64_t as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    15319502457978536222 => {
                        l1 <<= l2 & shm as std::os::raw::c_ulong;
                        current_block = 2116367355679836638;
                    },
                    7644865018902103128 => {
                        l1 >>= l2 & shm as std::os::raw::c_ulong;
                        current_block = 2116367355679836638;
                    },
                    14360337777247149493 => {
                        l1 = if l1 >> 63 as std::os::raw::c_int != 0 {
                            !(!l1 >> (l2 & shm as std::os::raw::c_ulong))
                        } else {
                            (l1) >> (l2 & shm as std::os::raw::c_ulong)
                        };
                        current_block = 2116367355679836638;
                    },
                    7983287325140408908 => {
                        l1 = (l1 < l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    11798621572665121461 => {
                        l1 = (l1 >= l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    13932873228663550769 => {
                        l1 = (l1 == l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    1788142895051060034 => {
                        l1 = (l1 != l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    9886615242100325262 => {
                        l1 = (l1 <= l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    2041506142873075795 => {
                        l1 = (l1 > l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    6863629183173758772 => {
                        l1 = gen_opic_lt(l1, l2) as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    8913536887710889399 => {
                        l1 = (gen_opic_lt(l1, l2) == 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    4631372686411729056 => {
                        l1 = (gen_opic_lt(l2, l1) == 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    11170670477888610567 => {
                        l1 = gen_opic_lt(l2, l1) as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    18380056810314386699 => {
                        l1 = (l1 != 0 && l2 != 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    _ => {
                        if l2 == 0 as std::os::raw::c_int as std::os::raw::c_ulong {
                            if const_wanted != 0
                                && nocode_wanted & 0xffff as std::os::raw::c_int == 0
                            {
                                _tcc_error(
                                    b"division by zero in constant\x00" as *const u8
                                        as *const std::os::raw::c_char,
                                );
                            }
                            current_block = 10869693750139516408;
                        } else {
                            match op {
                                37 => l1 = l1.wrapping_sub(l2.wrapping_mul(gen_opic_sdiv(l1, l2))),
                                131 => l1 = l1.wrapping_div(l2),
                                132 => l1 = l1.wrapping_rem(l2),
                                _ => l1 = gen_opic_sdiv(l1, l2),
                            }
                            current_block = 2116367355679836638;
                        }
                    },
                }
                match current_block {
                    10869693750139516408 => {},
                    _ => {
                        if t1 != 4 as std::os::raw::c_int
                            && (8 as std::os::raw::c_int != 8 as std::os::raw::c_int
                                || t1 != 5 as std::os::raw::c_int)
                        {
                            l1 = l1 as uint32_t as std::os::raw::c_ulong
                                | (if (*v1).type_0.t & 0x10 as std::os::raw::c_int != 0 {
                                    0 as std::os::raw::c_int as std::os::raw::c_ulong
                                } else {
                                    (l1 & 0x80000000 as std::os::raw::c_uint
                                        as std::os::raw::c_ulong)
                                        .wrapping_neg()
                                })
                        }
                        (*v1).c2rust_unnamed.c.i = l1;
                        vtop = vtop.offset(-1);
                        current_block = 2705889988320590074;
                    },
                }
            },
            139 => {
                current_block = 7644865018902103128;
                match current_block {
                    9531126118564703076 => {
                        l1 = (l1 != 0 || l2 != 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    337463635748514786 => {
                        l1 = (l1 as std::os::raw::c_ulong).wrapping_add(l2) as uint64_t as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    14513523936503887211 => {
                        l1 = (l1 as std::os::raw::c_ulong).wrapping_sub(l2) as uint64_t as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    13264933720371784297 => {
                        l1 &= l2;
                        current_block = 2116367355679836638;
                    },
                    6266472726643588263 => {
                        l1 ^= l2;
                        current_block = 2116367355679836638;
                    },
                    4912292479518253705 => {
                        l1 |= l2;
                        current_block = 2116367355679836638;
                    },
                    6484990028451387192 => {
                        l1 = (l1 as std::os::raw::c_ulong).wrapping_mul(l2) as uint64_t as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    15319502457978536222 => {
                        l1 <<= l2 & shm as std::os::raw::c_ulong;
                        current_block = 2116367355679836638;
                    },
                    7644865018902103128 => {
                        l1 >>= l2 & shm as std::os::raw::c_ulong;
                        current_block = 2116367355679836638;
                    },
                    14360337777247149493 => {
                        l1 = if l1 >> 63 as std::os::raw::c_int != 0 {
                            !(!l1 >> (l2 & shm as std::os::raw::c_ulong))
                        } else {
                            (l1) >> (l2 & shm as std::os::raw::c_ulong)
                        };
                        current_block = 2116367355679836638;
                    },
                    7983287325140408908 => {
                        l1 = (l1 < l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    11798621572665121461 => {
                        l1 = (l1 >= l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    13932873228663550769 => {
                        l1 = (l1 == l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    1788142895051060034 => {
                        l1 = (l1 != l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    9886615242100325262 => {
                        l1 = (l1 <= l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    2041506142873075795 => {
                        l1 = (l1 > l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    6863629183173758772 => {
                        l1 = gen_opic_lt(l1, l2) as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    8913536887710889399 => {
                        l1 = (gen_opic_lt(l1, l2) == 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    4631372686411729056 => {
                        l1 = (gen_opic_lt(l2, l1) == 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    11170670477888610567 => {
                        l1 = gen_opic_lt(l2, l1) as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    18380056810314386699 => {
                        l1 = (l1 != 0 && l2 != 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    _ => {
                        if l2 == 0 as std::os::raw::c_int as std::os::raw::c_ulong {
                            if const_wanted != 0
                                && nocode_wanted & 0xffff as std::os::raw::c_int == 0
                            {
                                _tcc_error(
                                    b"division by zero in constant\x00" as *const u8
                                        as *const std::os::raw::c_char,
                                );
                            }
                            current_block = 10869693750139516408;
                        } else {
                            match op {
                                37 => l1 = l1.wrapping_sub(l2.wrapping_mul(gen_opic_sdiv(l1, l2))),
                                131 => l1 = l1.wrapping_div(l2),
                                132 => l1 = l1.wrapping_rem(l2),
                                _ => l1 = gen_opic_sdiv(l1, l2),
                            }
                            current_block = 2116367355679836638;
                        }
                    },
                }
                match current_block {
                    10869693750139516408 => {},
                    _ => {
                        if t1 != 4 as std::os::raw::c_int
                            && (8 as std::os::raw::c_int != 8 as std::os::raw::c_int
                                || t1 != 5 as std::os::raw::c_int)
                        {
                            l1 = l1 as uint32_t as std::os::raw::c_ulong
                                | (if (*v1).type_0.t & 0x10 as std::os::raw::c_int != 0 {
                                    0 as std::os::raw::c_int as std::os::raw::c_ulong
                                } else {
                                    (l1 & 0x80000000 as std::os::raw::c_uint
                                        as std::os::raw::c_ulong)
                                        .wrapping_neg()
                                })
                        }
                        (*v1).c2rust_unnamed.c.i = l1;
                        vtop = vtop.offset(-1);
                        current_block = 2705889988320590074;
                    },
                }
            },
            62 => {
                current_block = 14360337777247149493;
                match current_block {
                    9531126118564703076 => {
                        l1 = (l1 != 0 || l2 != 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    337463635748514786 => {
                        l1 = (l1 as std::os::raw::c_ulong).wrapping_add(l2) as uint64_t as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    14513523936503887211 => {
                        l1 = (l1 as std::os::raw::c_ulong).wrapping_sub(l2) as uint64_t as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    13264933720371784297 => {
                        l1 &= l2;
                        current_block = 2116367355679836638;
                    },
                    6266472726643588263 => {
                        l1 ^= l2;
                        current_block = 2116367355679836638;
                    },
                    4912292479518253705 => {
                        l1 |= l2;
                        current_block = 2116367355679836638;
                    },
                    6484990028451387192 => {
                        l1 = (l1 as std::os::raw::c_ulong).wrapping_mul(l2) as uint64_t as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    15319502457978536222 => {
                        l1 <<= l2 & shm as std::os::raw::c_ulong;
                        current_block = 2116367355679836638;
                    },
                    7644865018902103128 => {
                        l1 >>= l2 & shm as std::os::raw::c_ulong;
                        current_block = 2116367355679836638;
                    },
                    14360337777247149493 => {
                        l1 = if l1 >> 63 as std::os::raw::c_int != 0 {
                            !(!l1 >> (l2 & shm as std::os::raw::c_ulong))
                        } else {
                            (l1) >> (l2 & shm as std::os::raw::c_ulong)
                        };
                        current_block = 2116367355679836638;
                    },
                    7983287325140408908 => {
                        l1 = (l1 < l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    11798621572665121461 => {
                        l1 = (l1 >= l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    13932873228663550769 => {
                        l1 = (l1 == l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    1788142895051060034 => {
                        l1 = (l1 != l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    9886615242100325262 => {
                        l1 = (l1 <= l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    2041506142873075795 => {
                        l1 = (l1 > l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    6863629183173758772 => {
                        l1 = gen_opic_lt(l1, l2) as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    8913536887710889399 => {
                        l1 = (gen_opic_lt(l1, l2) == 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    4631372686411729056 => {
                        l1 = (gen_opic_lt(l2, l1) == 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    11170670477888610567 => {
                        l1 = gen_opic_lt(l2, l1) as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    18380056810314386699 => {
                        l1 = (l1 != 0 && l2 != 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    _ => {
                        if l2 == 0 as std::os::raw::c_int as std::os::raw::c_ulong {
                            if const_wanted != 0
                                && nocode_wanted & 0xffff as std::os::raw::c_int == 0
                            {
                                _tcc_error(
                                    b"division by zero in constant\x00" as *const u8
                                        as *const std::os::raw::c_char,
                                );
                            }
                            current_block = 10869693750139516408;
                        } else {
                            match op {
                                37 => l1 = l1.wrapping_sub(l2.wrapping_mul(gen_opic_sdiv(l1, l2))),
                                131 => l1 = l1.wrapping_div(l2),
                                132 => l1 = l1.wrapping_rem(l2),
                                _ => l1 = gen_opic_sdiv(l1, l2),
                            }
                            current_block = 2116367355679836638;
                        }
                    },
                }
                match current_block {
                    10869693750139516408 => {},
                    _ => {
                        if t1 != 4 as std::os::raw::c_int
                            && (8 as std::os::raw::c_int != 8 as std::os::raw::c_int
                                || t1 != 5 as std::os::raw::c_int)
                        {
                            l1 = l1 as uint32_t as std::os::raw::c_ulong
                                | (if (*v1).type_0.t & 0x10 as std::os::raw::c_int != 0 {
                                    0 as std::os::raw::c_int as std::os::raw::c_ulong
                                } else {
                                    (l1 & 0x80000000 as std::os::raw::c_uint
                                        as std::os::raw::c_ulong)
                                        .wrapping_neg()
                                })
                        }
                        (*v1).c2rust_unnamed.c.i = l1;
                        vtop = vtop.offset(-1);
                        current_block = 2705889988320590074;
                    },
                }
            },
            146 => {
                current_block = 7983287325140408908;
                match current_block {
                    9531126118564703076 => {
                        l1 = (l1 != 0 || l2 != 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    337463635748514786 => {
                        l1 = (l1 as std::os::raw::c_ulong).wrapping_add(l2) as uint64_t as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    14513523936503887211 => {
                        l1 = (l1 as std::os::raw::c_ulong).wrapping_sub(l2) as uint64_t as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    13264933720371784297 => {
                        l1 &= l2;
                        current_block = 2116367355679836638;
                    },
                    6266472726643588263 => {
                        l1 ^= l2;
                        current_block = 2116367355679836638;
                    },
                    4912292479518253705 => {
                        l1 |= l2;
                        current_block = 2116367355679836638;
                    },
                    6484990028451387192 => {
                        l1 = (l1 as std::os::raw::c_ulong).wrapping_mul(l2) as uint64_t as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    15319502457978536222 => {
                        l1 <<= l2 & shm as std::os::raw::c_ulong;
                        current_block = 2116367355679836638;
                    },
                    7644865018902103128 => {
                        l1 >>= l2 & shm as std::os::raw::c_ulong;
                        current_block = 2116367355679836638;
                    },
                    14360337777247149493 => {
                        l1 = if l1 >> 63 as std::os::raw::c_int != 0 {
                            !(!l1 >> (l2 & shm as std::os::raw::c_ulong))
                        } else {
                            (l1) >> (l2 & shm as std::os::raw::c_ulong)
                        };
                        current_block = 2116367355679836638;
                    },
                    7983287325140408908 => {
                        l1 = (l1 < l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    11798621572665121461 => {
                        l1 = (l1 >= l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    13932873228663550769 => {
                        l1 = (l1 == l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    1788142895051060034 => {
                        l1 = (l1 != l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    9886615242100325262 => {
                        l1 = (l1 <= l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    2041506142873075795 => {
                        l1 = (l1 > l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    6863629183173758772 => {
                        l1 = gen_opic_lt(l1, l2) as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    8913536887710889399 => {
                        l1 = (gen_opic_lt(l1, l2) == 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    4631372686411729056 => {
                        l1 = (gen_opic_lt(l2, l1) == 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    11170670477888610567 => {
                        l1 = gen_opic_lt(l2, l1) as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    18380056810314386699 => {
                        l1 = (l1 != 0 && l2 != 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    _ => {
                        if l2 == 0 as std::os::raw::c_int as std::os::raw::c_ulong {
                            if const_wanted != 0
                                && nocode_wanted & 0xffff as std::os::raw::c_int == 0
                            {
                                _tcc_error(
                                    b"division by zero in constant\x00" as *const u8
                                        as *const std::os::raw::c_char,
                                );
                            }
                            current_block = 10869693750139516408;
                        } else {
                            match op {
                                37 => l1 = l1.wrapping_sub(l2.wrapping_mul(gen_opic_sdiv(l1, l2))),
                                131 => l1 = l1.wrapping_div(l2),
                                132 => l1 = l1.wrapping_rem(l2),
                                _ => l1 = gen_opic_sdiv(l1, l2),
                            }
                            current_block = 2116367355679836638;
                        }
                    },
                }
                match current_block {
                    10869693750139516408 => {},
                    _ => {
                        if t1 != 4 as std::os::raw::c_int
                            && (8 as std::os::raw::c_int != 8 as std::os::raw::c_int
                                || t1 != 5 as std::os::raw::c_int)
                        {
                            l1 = l1 as uint32_t as std::os::raw::c_ulong
                                | (if (*v1).type_0.t & 0x10 as std::os::raw::c_int != 0 {
                                    0 as std::os::raw::c_int as std::os::raw::c_ulong
                                } else {
                                    (l1 & 0x80000000 as std::os::raw::c_uint
                                        as std::os::raw::c_ulong)
                                        .wrapping_neg()
                                })
                        }
                        (*v1).c2rust_unnamed.c.i = l1;
                        vtop = vtop.offset(-1);
                        current_block = 2705889988320590074;
                    },
                }
            },
            147 => {
                current_block = 11798621572665121461;
                match current_block {
                    9531126118564703076 => {
                        l1 = (l1 != 0 || l2 != 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    337463635748514786 => {
                        l1 = (l1 as std::os::raw::c_ulong).wrapping_add(l2) as uint64_t as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    14513523936503887211 => {
                        l1 = (l1 as std::os::raw::c_ulong).wrapping_sub(l2) as uint64_t as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    13264933720371784297 => {
                        l1 &= l2;
                        current_block = 2116367355679836638;
                    },
                    6266472726643588263 => {
                        l1 ^= l2;
                        current_block = 2116367355679836638;
                    },
                    4912292479518253705 => {
                        l1 |= l2;
                        current_block = 2116367355679836638;
                    },
                    6484990028451387192 => {
                        l1 = (l1 as std::os::raw::c_ulong).wrapping_mul(l2) as uint64_t as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    15319502457978536222 => {
                        l1 <<= l2 & shm as std::os::raw::c_ulong;
                        current_block = 2116367355679836638;
                    },
                    7644865018902103128 => {
                        l1 >>= l2 & shm as std::os::raw::c_ulong;
                        current_block = 2116367355679836638;
                    },
                    14360337777247149493 => {
                        l1 = if l1 >> 63 as std::os::raw::c_int != 0 {
                            !(!l1 >> (l2 & shm as std::os::raw::c_ulong))
                        } else {
                            (l1) >> (l2 & shm as std::os::raw::c_ulong)
                        };
                        current_block = 2116367355679836638;
                    },
                    7983287325140408908 => {
                        l1 = (l1 < l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    11798621572665121461 => {
                        l1 = (l1 >= l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    13932873228663550769 => {
                        l1 = (l1 == l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    1788142895051060034 => {
                        l1 = (l1 != l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    9886615242100325262 => {
                        l1 = (l1 <= l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    2041506142873075795 => {
                        l1 = (l1 > l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    6863629183173758772 => {
                        l1 = gen_opic_lt(l1, l2) as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    8913536887710889399 => {
                        l1 = (gen_opic_lt(l1, l2) == 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    4631372686411729056 => {
                        l1 = (gen_opic_lt(l2, l1) == 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    11170670477888610567 => {
                        l1 = gen_opic_lt(l2, l1) as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    18380056810314386699 => {
                        l1 = (l1 != 0 && l2 != 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    _ => {
                        if l2 == 0 as std::os::raw::c_int as std::os::raw::c_ulong {
                            if const_wanted != 0
                                && nocode_wanted & 0xffff as std::os::raw::c_int == 0
                            {
                                _tcc_error(
                                    b"division by zero in constant\x00" as *const u8
                                        as *const std::os::raw::c_char,
                                );
                            }
                            current_block = 10869693750139516408;
                        } else {
                            match op {
                                37 => l1 = l1.wrapping_sub(l2.wrapping_mul(gen_opic_sdiv(l1, l2))),
                                131 => l1 = l1.wrapping_div(l2),
                                132 => l1 = l1.wrapping_rem(l2),
                                _ => l1 = gen_opic_sdiv(l1, l2),
                            }
                            current_block = 2116367355679836638;
                        }
                    },
                }
                match current_block {
                    10869693750139516408 => {},
                    _ => {
                        if t1 != 4 as std::os::raw::c_int
                            && (8 as std::os::raw::c_int != 8 as std::os::raw::c_int
                                || t1 != 5 as std::os::raw::c_int)
                        {
                            l1 = l1 as uint32_t as std::os::raw::c_ulong
                                | (if (*v1).type_0.t & 0x10 as std::os::raw::c_int != 0 {
                                    0 as std::os::raw::c_int as std::os::raw::c_ulong
                                } else {
                                    (l1 & 0x80000000 as std::os::raw::c_uint
                                        as std::os::raw::c_ulong)
                                        .wrapping_neg()
                                })
                        }
                        (*v1).c2rust_unnamed.c.i = l1;
                        vtop = vtop.offset(-1);
                        current_block = 2705889988320590074;
                    },
                }
            },
            148 => {
                current_block = 13932873228663550769;
                match current_block {
                    9531126118564703076 => {
                        l1 = (l1 != 0 || l2 != 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    337463635748514786 => {
                        l1 = (l1 as std::os::raw::c_ulong).wrapping_add(l2) as uint64_t as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    14513523936503887211 => {
                        l1 = (l1 as std::os::raw::c_ulong).wrapping_sub(l2) as uint64_t as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    13264933720371784297 => {
                        l1 &= l2;
                        current_block = 2116367355679836638;
                    },
                    6266472726643588263 => {
                        l1 ^= l2;
                        current_block = 2116367355679836638;
                    },
                    4912292479518253705 => {
                        l1 |= l2;
                        current_block = 2116367355679836638;
                    },
                    6484990028451387192 => {
                        l1 = (l1 as std::os::raw::c_ulong).wrapping_mul(l2) as uint64_t as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    15319502457978536222 => {
                        l1 <<= l2 & shm as std::os::raw::c_ulong;
                        current_block = 2116367355679836638;
                    },
                    7644865018902103128 => {
                        l1 >>= l2 & shm as std::os::raw::c_ulong;
                        current_block = 2116367355679836638;
                    },
                    14360337777247149493 => {
                        l1 = if l1 >> 63 as std::os::raw::c_int != 0 {
                            !(!l1 >> (l2 & shm as std::os::raw::c_ulong))
                        } else {
                            (l1) >> (l2 & shm as std::os::raw::c_ulong)
                        };
                        current_block = 2116367355679836638;
                    },
                    7983287325140408908 => {
                        l1 = (l1 < l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    11798621572665121461 => {
                        l1 = (l1 >= l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    13932873228663550769 => {
                        l1 = (l1 == l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    1788142895051060034 => {
                        l1 = (l1 != l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    9886615242100325262 => {
                        l1 = (l1 <= l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    2041506142873075795 => {
                        l1 = (l1 > l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    6863629183173758772 => {
                        l1 = gen_opic_lt(l1, l2) as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    8913536887710889399 => {
                        l1 = (gen_opic_lt(l1, l2) == 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    4631372686411729056 => {
                        l1 = (gen_opic_lt(l2, l1) == 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    11170670477888610567 => {
                        l1 = gen_opic_lt(l2, l1) as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    18380056810314386699 => {
                        l1 = (l1 != 0 && l2 != 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    _ => {
                        if l2 == 0 as std::os::raw::c_int as std::os::raw::c_ulong {
                            if const_wanted != 0
                                && nocode_wanted & 0xffff as std::os::raw::c_int == 0
                            {
                                _tcc_error(
                                    b"division by zero in constant\x00" as *const u8
                                        as *const std::os::raw::c_char,
                                );
                            }
                            current_block = 10869693750139516408;
                        } else {
                            match op {
                                37 => l1 = l1.wrapping_sub(l2.wrapping_mul(gen_opic_sdiv(l1, l2))),
                                131 => l1 = l1.wrapping_div(l2),
                                132 => l1 = l1.wrapping_rem(l2),
                                _ => l1 = gen_opic_sdiv(l1, l2),
                            }
                            current_block = 2116367355679836638;
                        }
                    },
                }
                match current_block {
                    10869693750139516408 => {},
                    _ => {
                        if t1 != 4 as std::os::raw::c_int
                            && (8 as std::os::raw::c_int != 8 as std::os::raw::c_int
                                || t1 != 5 as std::os::raw::c_int)
                        {
                            l1 = l1 as uint32_t as std::os::raw::c_ulong
                                | (if (*v1).type_0.t & 0x10 as std::os::raw::c_int != 0 {
                                    0 as std::os::raw::c_int as std::os::raw::c_ulong
                                } else {
                                    (l1 & 0x80000000 as std::os::raw::c_uint
                                        as std::os::raw::c_ulong)
                                        .wrapping_neg()
                                })
                        }
                        (*v1).c2rust_unnamed.c.i = l1;
                        vtop = vtop.offset(-1);
                        current_block = 2705889988320590074;
                    },
                }
            },
            149 => {
                current_block = 1788142895051060034;
                match current_block {
                    9531126118564703076 => {
                        l1 = (l1 != 0 || l2 != 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    337463635748514786 => {
                        l1 = (l1 as std::os::raw::c_ulong).wrapping_add(l2) as uint64_t as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    14513523936503887211 => {
                        l1 = (l1 as std::os::raw::c_ulong).wrapping_sub(l2) as uint64_t as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    13264933720371784297 => {
                        l1 &= l2;
                        current_block = 2116367355679836638;
                    },
                    6266472726643588263 => {
                        l1 ^= l2;
                        current_block = 2116367355679836638;
                    },
                    4912292479518253705 => {
                        l1 |= l2;
                        current_block = 2116367355679836638;
                    },
                    6484990028451387192 => {
                        l1 = (l1 as std::os::raw::c_ulong).wrapping_mul(l2) as uint64_t as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    15319502457978536222 => {
                        l1 <<= l2 & shm as std::os::raw::c_ulong;
                        current_block = 2116367355679836638;
                    },
                    7644865018902103128 => {
                        l1 >>= l2 & shm as std::os::raw::c_ulong;
                        current_block = 2116367355679836638;
                    },
                    14360337777247149493 => {
                        l1 = if l1 >> 63 as std::os::raw::c_int != 0 {
                            !(!l1 >> (l2 & shm as std::os::raw::c_ulong))
                        } else {
                            (l1) >> (l2 & shm as std::os::raw::c_ulong)
                        };
                        current_block = 2116367355679836638;
                    },
                    7983287325140408908 => {
                        l1 = (l1 < l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    11798621572665121461 => {
                        l1 = (l1 >= l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    13932873228663550769 => {
                        l1 = (l1 == l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    1788142895051060034 => {
                        l1 = (l1 != l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    9886615242100325262 => {
                        l1 = (l1 <= l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    2041506142873075795 => {
                        l1 = (l1 > l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    6863629183173758772 => {
                        l1 = gen_opic_lt(l1, l2) as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    8913536887710889399 => {
                        l1 = (gen_opic_lt(l1, l2) == 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    4631372686411729056 => {
                        l1 = (gen_opic_lt(l2, l1) == 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    11170670477888610567 => {
                        l1 = gen_opic_lt(l2, l1) as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    18380056810314386699 => {
                        l1 = (l1 != 0 && l2 != 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    _ => {
                        if l2 == 0 as std::os::raw::c_int as std::os::raw::c_ulong {
                            if const_wanted != 0
                                && nocode_wanted & 0xffff as std::os::raw::c_int == 0
                            {
                                _tcc_error(
                                    b"division by zero in constant\x00" as *const u8
                                        as *const std::os::raw::c_char,
                                );
                            }
                            current_block = 10869693750139516408;
                        } else {
                            match op {
                                37 => l1 = l1.wrapping_sub(l2.wrapping_mul(gen_opic_sdiv(l1, l2))),
                                131 => l1 = l1.wrapping_div(l2),
                                132 => l1 = l1.wrapping_rem(l2),
                                _ => l1 = gen_opic_sdiv(l1, l2),
                            }
                            current_block = 2116367355679836638;
                        }
                    },
                }
                match current_block {
                    10869693750139516408 => {},
                    _ => {
                        if t1 != 4 as std::os::raw::c_int
                            && (8 as std::os::raw::c_int != 8 as std::os::raw::c_int
                                || t1 != 5 as std::os::raw::c_int)
                        {
                            l1 = l1 as uint32_t as std::os::raw::c_ulong
                                | (if (*v1).type_0.t & 0x10 as std::os::raw::c_int != 0 {
                                    0 as std::os::raw::c_int as std::os::raw::c_ulong
                                } else {
                                    (l1 & 0x80000000 as std::os::raw::c_uint
                                        as std::os::raw::c_ulong)
                                        .wrapping_neg()
                                })
                        }
                        (*v1).c2rust_unnamed.c.i = l1;
                        vtop = vtop.offset(-1);
                        current_block = 2705889988320590074;
                    },
                }
            },
            150 => {
                current_block = 9886615242100325262;
                match current_block {
                    9531126118564703076 => {
                        l1 = (l1 != 0 || l2 != 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    337463635748514786 => {
                        l1 = (l1 as std::os::raw::c_ulong).wrapping_add(l2) as uint64_t as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    14513523936503887211 => {
                        l1 = (l1 as std::os::raw::c_ulong).wrapping_sub(l2) as uint64_t as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    13264933720371784297 => {
                        l1 &= l2;
                        current_block = 2116367355679836638;
                    },
                    6266472726643588263 => {
                        l1 ^= l2;
                        current_block = 2116367355679836638;
                    },
                    4912292479518253705 => {
                        l1 |= l2;
                        current_block = 2116367355679836638;
                    },
                    6484990028451387192 => {
                        l1 = (l1 as std::os::raw::c_ulong).wrapping_mul(l2) as uint64_t as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    15319502457978536222 => {
                        l1 <<= l2 & shm as std::os::raw::c_ulong;
                        current_block = 2116367355679836638;
                    },
                    7644865018902103128 => {
                        l1 >>= l2 & shm as std::os::raw::c_ulong;
                        current_block = 2116367355679836638;
                    },
                    14360337777247149493 => {
                        l1 = if l1 >> 63 as std::os::raw::c_int != 0 {
                            !(!l1 >> (l2 & shm as std::os::raw::c_ulong))
                        } else {
                            (l1) >> (l2 & shm as std::os::raw::c_ulong)
                        };
                        current_block = 2116367355679836638;
                    },
                    7983287325140408908 => {
                        l1 = (l1 < l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    11798621572665121461 => {
                        l1 = (l1 >= l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    13932873228663550769 => {
                        l1 = (l1 == l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    1788142895051060034 => {
                        l1 = (l1 != l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    9886615242100325262 => {
                        l1 = (l1 <= l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    2041506142873075795 => {
                        l1 = (l1 > l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    6863629183173758772 => {
                        l1 = gen_opic_lt(l1, l2) as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    8913536887710889399 => {
                        l1 = (gen_opic_lt(l1, l2) == 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    4631372686411729056 => {
                        l1 = (gen_opic_lt(l2, l1) == 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    11170670477888610567 => {
                        l1 = gen_opic_lt(l2, l1) as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    18380056810314386699 => {
                        l1 = (l1 != 0 && l2 != 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    _ => {
                        if l2 == 0 as std::os::raw::c_int as std::os::raw::c_ulong {
                            if const_wanted != 0
                                && nocode_wanted & 0xffff as std::os::raw::c_int == 0
                            {
                                _tcc_error(
                                    b"division by zero in constant\x00" as *const u8
                                        as *const std::os::raw::c_char,
                                );
                            }
                            current_block = 10869693750139516408;
                        } else {
                            match op {
                                37 => l1 = l1.wrapping_sub(l2.wrapping_mul(gen_opic_sdiv(l1, l2))),
                                131 => l1 = l1.wrapping_div(l2),
                                132 => l1 = l1.wrapping_rem(l2),
                                _ => l1 = gen_opic_sdiv(l1, l2),
                            }
                            current_block = 2116367355679836638;
                        }
                    },
                }
                match current_block {
                    10869693750139516408 => {},
                    _ => {
                        if t1 != 4 as std::os::raw::c_int
                            && (8 as std::os::raw::c_int != 8 as std::os::raw::c_int
                                || t1 != 5 as std::os::raw::c_int)
                        {
                            l1 = l1 as uint32_t as std::os::raw::c_ulong
                                | (if (*v1).type_0.t & 0x10 as std::os::raw::c_int != 0 {
                                    0 as std::os::raw::c_int as std::os::raw::c_ulong
                                } else {
                                    (l1 & 0x80000000 as std::os::raw::c_uint
                                        as std::os::raw::c_ulong)
                                        .wrapping_neg()
                                })
                        }
                        (*v1).c2rust_unnamed.c.i = l1;
                        vtop = vtop.offset(-1);
                        current_block = 2705889988320590074;
                    },
                }
            },
            151 => {
                current_block = 2041506142873075795;
                match current_block {
                    9531126118564703076 => {
                        l1 = (l1 != 0 || l2 != 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    337463635748514786 => {
                        l1 = (l1 as std::os::raw::c_ulong).wrapping_add(l2) as uint64_t as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    14513523936503887211 => {
                        l1 = (l1 as std::os::raw::c_ulong).wrapping_sub(l2) as uint64_t as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    13264933720371784297 => {
                        l1 &= l2;
                        current_block = 2116367355679836638;
                    },
                    6266472726643588263 => {
                        l1 ^= l2;
                        current_block = 2116367355679836638;
                    },
                    4912292479518253705 => {
                        l1 |= l2;
                        current_block = 2116367355679836638;
                    },
                    6484990028451387192 => {
                        l1 = (l1 as std::os::raw::c_ulong).wrapping_mul(l2) as uint64_t as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    15319502457978536222 => {
                        l1 <<= l2 & shm as std::os::raw::c_ulong;
                        current_block = 2116367355679836638;
                    },
                    7644865018902103128 => {
                        l1 >>= l2 & shm as std::os::raw::c_ulong;
                        current_block = 2116367355679836638;
                    },
                    14360337777247149493 => {
                        l1 = if l1 >> 63 as std::os::raw::c_int != 0 {
                            !(!l1 >> (l2 & shm as std::os::raw::c_ulong))
                        } else {
                            (l1) >> (l2 & shm as std::os::raw::c_ulong)
                        };
                        current_block = 2116367355679836638;
                    },
                    7983287325140408908 => {
                        l1 = (l1 < l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    11798621572665121461 => {
                        l1 = (l1 >= l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    13932873228663550769 => {
                        l1 = (l1 == l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    1788142895051060034 => {
                        l1 = (l1 != l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    9886615242100325262 => {
                        l1 = (l1 <= l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    2041506142873075795 => {
                        l1 = (l1 > l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    6863629183173758772 => {
                        l1 = gen_opic_lt(l1, l2) as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    8913536887710889399 => {
                        l1 = (gen_opic_lt(l1, l2) == 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    4631372686411729056 => {
                        l1 = (gen_opic_lt(l2, l1) == 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    11170670477888610567 => {
                        l1 = gen_opic_lt(l2, l1) as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    18380056810314386699 => {
                        l1 = (l1 != 0 && l2 != 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    _ => {
                        if l2 == 0 as std::os::raw::c_int as std::os::raw::c_ulong {
                            if const_wanted != 0
                                && nocode_wanted & 0xffff as std::os::raw::c_int == 0
                            {
                                _tcc_error(
                                    b"division by zero in constant\x00" as *const u8
                                        as *const std::os::raw::c_char,
                                );
                            }
                            current_block = 10869693750139516408;
                        } else {
                            match op {
                                37 => l1 = l1.wrapping_sub(l2.wrapping_mul(gen_opic_sdiv(l1, l2))),
                                131 => l1 = l1.wrapping_div(l2),
                                132 => l1 = l1.wrapping_rem(l2),
                                _ => l1 = gen_opic_sdiv(l1, l2),
                            }
                            current_block = 2116367355679836638;
                        }
                    },
                }
                match current_block {
                    10869693750139516408 => {},
                    _ => {
                        if t1 != 4 as std::os::raw::c_int
                            && (8 as std::os::raw::c_int != 8 as std::os::raw::c_int
                                || t1 != 5 as std::os::raw::c_int)
                        {
                            l1 = l1 as uint32_t as std::os::raw::c_ulong
                                | (if (*v1).type_0.t & 0x10 as std::os::raw::c_int != 0 {
                                    0 as std::os::raw::c_int as std::os::raw::c_ulong
                                } else {
                                    (l1 & 0x80000000 as std::os::raw::c_uint
                                        as std::os::raw::c_ulong)
                                        .wrapping_neg()
                                })
                        }
                        (*v1).c2rust_unnamed.c.i = l1;
                        vtop = vtop.offset(-1);
                        current_block = 2705889988320590074;
                    },
                }
            },
            156 => {
                current_block = 6863629183173758772;
                match current_block {
                    9531126118564703076 => {
                        l1 = (l1 != 0 || l2 != 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    337463635748514786 => {
                        l1 = (l1 as std::os::raw::c_ulong).wrapping_add(l2) as uint64_t as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    14513523936503887211 => {
                        l1 = (l1 as std::os::raw::c_ulong).wrapping_sub(l2) as uint64_t as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    13264933720371784297 => {
                        l1 &= l2;
                        current_block = 2116367355679836638;
                    },
                    6266472726643588263 => {
                        l1 ^= l2;
                        current_block = 2116367355679836638;
                    },
                    4912292479518253705 => {
                        l1 |= l2;
                        current_block = 2116367355679836638;
                    },
                    6484990028451387192 => {
                        l1 = (l1 as std::os::raw::c_ulong).wrapping_mul(l2) as uint64_t as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    15319502457978536222 => {
                        l1 <<= l2 & shm as std::os::raw::c_ulong;
                        current_block = 2116367355679836638;
                    },
                    7644865018902103128 => {
                        l1 >>= l2 & shm as std::os::raw::c_ulong;
                        current_block = 2116367355679836638;
                    },
                    14360337777247149493 => {
                        l1 = if l1 >> 63 as std::os::raw::c_int != 0 {
                            !(!l1 >> (l2 & shm as std::os::raw::c_ulong))
                        } else {
                            (l1) >> (l2 & shm as std::os::raw::c_ulong)
                        };
                        current_block = 2116367355679836638;
                    },
                    7983287325140408908 => {
                        l1 = (l1 < l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    11798621572665121461 => {
                        l1 = (l1 >= l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    13932873228663550769 => {
                        l1 = (l1 == l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    1788142895051060034 => {
                        l1 = (l1 != l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    9886615242100325262 => {
                        l1 = (l1 <= l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    2041506142873075795 => {
                        l1 = (l1 > l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    6863629183173758772 => {
                        l1 = gen_opic_lt(l1, l2) as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    8913536887710889399 => {
                        l1 = (gen_opic_lt(l1, l2) == 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    4631372686411729056 => {
                        l1 = (gen_opic_lt(l2, l1) == 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    11170670477888610567 => {
                        l1 = gen_opic_lt(l2, l1) as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    18380056810314386699 => {
                        l1 = (l1 != 0 && l2 != 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    _ => {
                        if l2 == 0 as std::os::raw::c_int as std::os::raw::c_ulong {
                            if const_wanted != 0
                                && nocode_wanted & 0xffff as std::os::raw::c_int == 0
                            {
                                _tcc_error(
                                    b"division by zero in constant\x00" as *const u8
                                        as *const std::os::raw::c_char,
                                );
                            }
                            current_block = 10869693750139516408;
                        } else {
                            match op {
                                37 => l1 = l1.wrapping_sub(l2.wrapping_mul(gen_opic_sdiv(l1, l2))),
                                131 => l1 = l1.wrapping_div(l2),
                                132 => l1 = l1.wrapping_rem(l2),
                                _ => l1 = gen_opic_sdiv(l1, l2),
                            }
                            current_block = 2116367355679836638;
                        }
                    },
                }
                match current_block {
                    10869693750139516408 => {},
                    _ => {
                        if t1 != 4 as std::os::raw::c_int
                            && (8 as std::os::raw::c_int != 8 as std::os::raw::c_int
                                || t1 != 5 as std::os::raw::c_int)
                        {
                            l1 = l1 as uint32_t as std::os::raw::c_ulong
                                | (if (*v1).type_0.t & 0x10 as std::os::raw::c_int != 0 {
                                    0 as std::os::raw::c_int as std::os::raw::c_ulong
                                } else {
                                    (l1 & 0x80000000 as std::os::raw::c_uint
                                        as std::os::raw::c_ulong)
                                        .wrapping_neg()
                                })
                        }
                        (*v1).c2rust_unnamed.c.i = l1;
                        vtop = vtop.offset(-1);
                        current_block = 2705889988320590074;
                    },
                }
            },
            157 => {
                current_block = 8913536887710889399;
                match current_block {
                    9531126118564703076 => {
                        l1 = (l1 != 0 || l2 != 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    337463635748514786 => {
                        l1 = (l1 as std::os::raw::c_ulong).wrapping_add(l2) as uint64_t as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    14513523936503887211 => {
                        l1 = (l1 as std::os::raw::c_ulong).wrapping_sub(l2) as uint64_t as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    13264933720371784297 => {
                        l1 &= l2;
                        current_block = 2116367355679836638;
                    },
                    6266472726643588263 => {
                        l1 ^= l2;
                        current_block = 2116367355679836638;
                    },
                    4912292479518253705 => {
                        l1 |= l2;
                        current_block = 2116367355679836638;
                    },
                    6484990028451387192 => {
                        l1 = (l1 as std::os::raw::c_ulong).wrapping_mul(l2) as uint64_t as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    15319502457978536222 => {
                        l1 <<= l2 & shm as std::os::raw::c_ulong;
                        current_block = 2116367355679836638;
                    },
                    7644865018902103128 => {
                        l1 >>= l2 & shm as std::os::raw::c_ulong;
                        current_block = 2116367355679836638;
                    },
                    14360337777247149493 => {
                        l1 = if l1 >> 63 as std::os::raw::c_int != 0 {
                            !(!l1 >> (l2 & shm as std::os::raw::c_ulong))
                        } else {
                            (l1) >> (l2 & shm as std::os::raw::c_ulong)
                        };
                        current_block = 2116367355679836638;
                    },
                    7983287325140408908 => {
                        l1 = (l1 < l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    11798621572665121461 => {
                        l1 = (l1 >= l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    13932873228663550769 => {
                        l1 = (l1 == l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    1788142895051060034 => {
                        l1 = (l1 != l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    9886615242100325262 => {
                        l1 = (l1 <= l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    2041506142873075795 => {
                        l1 = (l1 > l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    6863629183173758772 => {
                        l1 = gen_opic_lt(l1, l2) as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    8913536887710889399 => {
                        l1 = (gen_opic_lt(l1, l2) == 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    4631372686411729056 => {
                        l1 = (gen_opic_lt(l2, l1) == 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    11170670477888610567 => {
                        l1 = gen_opic_lt(l2, l1) as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    18380056810314386699 => {
                        l1 = (l1 != 0 && l2 != 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    _ => {
                        if l2 == 0 as std::os::raw::c_int as std::os::raw::c_ulong {
                            if const_wanted != 0
                                && nocode_wanted & 0xffff as std::os::raw::c_int == 0
                            {
                                _tcc_error(
                                    b"division by zero in constant\x00" as *const u8
                                        as *const std::os::raw::c_char,
                                );
                            }
                            current_block = 10869693750139516408;
                        } else {
                            match op {
                                37 => l1 = l1.wrapping_sub(l2.wrapping_mul(gen_opic_sdiv(l1, l2))),
                                131 => l1 = l1.wrapping_div(l2),
                                132 => l1 = l1.wrapping_rem(l2),
                                _ => l1 = gen_opic_sdiv(l1, l2),
                            }
                            current_block = 2116367355679836638;
                        }
                    },
                }
                match current_block {
                    10869693750139516408 => {},
                    _ => {
                        if t1 != 4 as std::os::raw::c_int
                            && (8 as std::os::raw::c_int != 8 as std::os::raw::c_int
                                || t1 != 5 as std::os::raw::c_int)
                        {
                            l1 = l1 as uint32_t as std::os::raw::c_ulong
                                | (if (*v1).type_0.t & 0x10 as std::os::raw::c_int != 0 {
                                    0 as std::os::raw::c_int as std::os::raw::c_ulong
                                } else {
                                    (l1 & 0x80000000 as std::os::raw::c_uint
                                        as std::os::raw::c_ulong)
                                        .wrapping_neg()
                                })
                        }
                        (*v1).c2rust_unnamed.c.i = l1;
                        vtop = vtop.offset(-1);
                        current_block = 2705889988320590074;
                    },
                }
            },
            158 => {
                current_block = 4631372686411729056;
                match current_block {
                    9531126118564703076 => {
                        l1 = (l1 != 0 || l2 != 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    337463635748514786 => {
                        l1 = (l1 as std::os::raw::c_ulong).wrapping_add(l2) as uint64_t as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    14513523936503887211 => {
                        l1 = (l1 as std::os::raw::c_ulong).wrapping_sub(l2) as uint64_t as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    13264933720371784297 => {
                        l1 &= l2;
                        current_block = 2116367355679836638;
                    },
                    6266472726643588263 => {
                        l1 ^= l2;
                        current_block = 2116367355679836638;
                    },
                    4912292479518253705 => {
                        l1 |= l2;
                        current_block = 2116367355679836638;
                    },
                    6484990028451387192 => {
                        l1 = (l1 as std::os::raw::c_ulong).wrapping_mul(l2) as uint64_t as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    15319502457978536222 => {
                        l1 <<= l2 & shm as std::os::raw::c_ulong;
                        current_block = 2116367355679836638;
                    },
                    7644865018902103128 => {
                        l1 >>= l2 & shm as std::os::raw::c_ulong;
                        current_block = 2116367355679836638;
                    },
                    14360337777247149493 => {
                        l1 = if l1 >> 63 as std::os::raw::c_int != 0 {
                            !(!l1 >> (l2 & shm as std::os::raw::c_ulong))
                        } else {
                            (l1) >> (l2 & shm as std::os::raw::c_ulong)
                        };
                        current_block = 2116367355679836638;
                    },
                    7983287325140408908 => {
                        l1 = (l1 < l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    11798621572665121461 => {
                        l1 = (l1 >= l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    13932873228663550769 => {
                        l1 = (l1 == l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    1788142895051060034 => {
                        l1 = (l1 != l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    9886615242100325262 => {
                        l1 = (l1 <= l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    2041506142873075795 => {
                        l1 = (l1 > l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    6863629183173758772 => {
                        l1 = gen_opic_lt(l1, l2) as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    8913536887710889399 => {
                        l1 = (gen_opic_lt(l1, l2) == 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    4631372686411729056 => {
                        l1 = (gen_opic_lt(l2, l1) == 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    11170670477888610567 => {
                        l1 = gen_opic_lt(l2, l1) as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    18380056810314386699 => {
                        l1 = (l1 != 0 && l2 != 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    _ => {
                        if l2 == 0 as std::os::raw::c_int as std::os::raw::c_ulong {
                            if const_wanted != 0
                                && nocode_wanted & 0xffff as std::os::raw::c_int == 0
                            {
                                _tcc_error(
                                    b"division by zero in constant\x00" as *const u8
                                        as *const std::os::raw::c_char,
                                );
                            }
                            current_block = 10869693750139516408;
                        } else {
                            match op {
                                37 => l1 = l1.wrapping_sub(l2.wrapping_mul(gen_opic_sdiv(l1, l2))),
                                131 => l1 = l1.wrapping_div(l2),
                                132 => l1 = l1.wrapping_rem(l2),
                                _ => l1 = gen_opic_sdiv(l1, l2),
                            }
                            current_block = 2116367355679836638;
                        }
                    },
                }
                match current_block {
                    10869693750139516408 => {},
                    _ => {
                        if t1 != 4 as std::os::raw::c_int
                            && (8 as std::os::raw::c_int != 8 as std::os::raw::c_int
                                || t1 != 5 as std::os::raw::c_int)
                        {
                            l1 = l1 as uint32_t as std::os::raw::c_ulong
                                | (if (*v1).type_0.t & 0x10 as std::os::raw::c_int != 0 {
                                    0 as std::os::raw::c_int as std::os::raw::c_ulong
                                } else {
                                    (l1 & 0x80000000 as std::os::raw::c_uint
                                        as std::os::raw::c_ulong)
                                        .wrapping_neg()
                                })
                        }
                        (*v1).c2rust_unnamed.c.i = l1;
                        vtop = vtop.offset(-1);
                        current_block = 2705889988320590074;
                    },
                }
            },
            159 => {
                current_block = 11170670477888610567;
                match current_block {
                    9531126118564703076 => {
                        l1 = (l1 != 0 || l2 != 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    337463635748514786 => {
                        l1 = (l1 as std::os::raw::c_ulong).wrapping_add(l2) as uint64_t as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    14513523936503887211 => {
                        l1 = (l1 as std::os::raw::c_ulong).wrapping_sub(l2) as uint64_t as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    13264933720371784297 => {
                        l1 &= l2;
                        current_block = 2116367355679836638;
                    },
                    6266472726643588263 => {
                        l1 ^= l2;
                        current_block = 2116367355679836638;
                    },
                    4912292479518253705 => {
                        l1 |= l2;
                        current_block = 2116367355679836638;
                    },
                    6484990028451387192 => {
                        l1 = (l1 as std::os::raw::c_ulong).wrapping_mul(l2) as uint64_t as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    15319502457978536222 => {
                        l1 <<= l2 & shm as std::os::raw::c_ulong;
                        current_block = 2116367355679836638;
                    },
                    7644865018902103128 => {
                        l1 >>= l2 & shm as std::os::raw::c_ulong;
                        current_block = 2116367355679836638;
                    },
                    14360337777247149493 => {
                        l1 = if l1 >> 63 as std::os::raw::c_int != 0 {
                            !(!l1 >> (l2 & shm as std::os::raw::c_ulong))
                        } else {
                            (l1) >> (l2 & shm as std::os::raw::c_ulong)
                        };
                        current_block = 2116367355679836638;
                    },
                    7983287325140408908 => {
                        l1 = (l1 < l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    11798621572665121461 => {
                        l1 = (l1 >= l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    13932873228663550769 => {
                        l1 = (l1 == l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    1788142895051060034 => {
                        l1 = (l1 != l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    9886615242100325262 => {
                        l1 = (l1 <= l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    2041506142873075795 => {
                        l1 = (l1 > l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    6863629183173758772 => {
                        l1 = gen_opic_lt(l1, l2) as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    8913536887710889399 => {
                        l1 = (gen_opic_lt(l1, l2) == 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    4631372686411729056 => {
                        l1 = (gen_opic_lt(l2, l1) == 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    11170670477888610567 => {
                        l1 = gen_opic_lt(l2, l1) as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    18380056810314386699 => {
                        l1 = (l1 != 0 && l2 != 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    _ => {
                        if l2 == 0 as std::os::raw::c_int as std::os::raw::c_ulong {
                            if const_wanted != 0
                                && nocode_wanted & 0xffff as std::os::raw::c_int == 0
                            {
                                _tcc_error(
                                    b"division by zero in constant\x00" as *const u8
                                        as *const std::os::raw::c_char,
                                );
                            }
                            current_block = 10869693750139516408;
                        } else {
                            match op {
                                37 => l1 = l1.wrapping_sub(l2.wrapping_mul(gen_opic_sdiv(l1, l2))),
                                131 => l1 = l1.wrapping_div(l2),
                                132 => l1 = l1.wrapping_rem(l2),
                                _ => l1 = gen_opic_sdiv(l1, l2),
                            }
                            current_block = 2116367355679836638;
                        }
                    },
                }
                match current_block {
                    10869693750139516408 => {},
                    _ => {
                        if t1 != 4 as std::os::raw::c_int
                            && (8 as std::os::raw::c_int != 8 as std::os::raw::c_int
                                || t1 != 5 as std::os::raw::c_int)
                        {
                            l1 = l1 as uint32_t as std::os::raw::c_ulong
                                | (if (*v1).type_0.t & 0x10 as std::os::raw::c_int != 0 {
                                    0 as std::os::raw::c_int as std::os::raw::c_ulong
                                } else {
                                    (l1 & 0x80000000 as std::os::raw::c_uint
                                        as std::os::raw::c_ulong)
                                        .wrapping_neg()
                                })
                        }
                        (*v1).c2rust_unnamed.c.i = l1;
                        vtop = vtop.offset(-1);
                        current_block = 2705889988320590074;
                    },
                }
            },
            144 => {
                current_block = 18380056810314386699;
                match current_block {
                    9531126118564703076 => {
                        l1 = (l1 != 0 || l2 != 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    337463635748514786 => {
                        l1 = (l1 as std::os::raw::c_ulong).wrapping_add(l2) as uint64_t as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    14513523936503887211 => {
                        l1 = (l1 as std::os::raw::c_ulong).wrapping_sub(l2) as uint64_t as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    13264933720371784297 => {
                        l1 &= l2;
                        current_block = 2116367355679836638;
                    },
                    6266472726643588263 => {
                        l1 ^= l2;
                        current_block = 2116367355679836638;
                    },
                    4912292479518253705 => {
                        l1 |= l2;
                        current_block = 2116367355679836638;
                    },
                    6484990028451387192 => {
                        l1 = (l1 as std::os::raw::c_ulong).wrapping_mul(l2) as uint64_t as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    15319502457978536222 => {
                        l1 <<= l2 & shm as std::os::raw::c_ulong;
                        current_block = 2116367355679836638;
                    },
                    7644865018902103128 => {
                        l1 >>= l2 & shm as std::os::raw::c_ulong;
                        current_block = 2116367355679836638;
                    },
                    14360337777247149493 => {
                        l1 = if l1 >> 63 as std::os::raw::c_int != 0 {
                            !(!l1 >> (l2 & shm as std::os::raw::c_ulong))
                        } else {
                            (l1) >> (l2 & shm as std::os::raw::c_ulong)
                        };
                        current_block = 2116367355679836638;
                    },
                    7983287325140408908 => {
                        l1 = (l1 < l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    11798621572665121461 => {
                        l1 = (l1 >= l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    13932873228663550769 => {
                        l1 = (l1 == l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    1788142895051060034 => {
                        l1 = (l1 != l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    9886615242100325262 => {
                        l1 = (l1 <= l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    2041506142873075795 => {
                        l1 = (l1 > l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    6863629183173758772 => {
                        l1 = gen_opic_lt(l1, l2) as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    8913536887710889399 => {
                        l1 = (gen_opic_lt(l1, l2) == 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    4631372686411729056 => {
                        l1 = (gen_opic_lt(l2, l1) == 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    11170670477888610567 => {
                        l1 = gen_opic_lt(l2, l1) as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    18380056810314386699 => {
                        l1 = (l1 != 0 && l2 != 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    _ => {
                        if l2 == 0 as std::os::raw::c_int as std::os::raw::c_ulong {
                            if const_wanted != 0
                                && nocode_wanted & 0xffff as std::os::raw::c_int == 0
                            {
                                _tcc_error(
                                    b"division by zero in constant\x00" as *const u8
                                        as *const std::os::raw::c_char,
                                );
                            }
                            current_block = 10869693750139516408;
                        } else {
                            match op {
                                37 => l1 = l1.wrapping_sub(l2.wrapping_mul(gen_opic_sdiv(l1, l2))),
                                131 => l1 = l1.wrapping_div(l2),
                                132 => l1 = l1.wrapping_rem(l2),
                                _ => l1 = gen_opic_sdiv(l1, l2),
                            }
                            current_block = 2116367355679836638;
                        }
                    },
                }
                match current_block {
                    10869693750139516408 => {},
                    _ => {
                        if t1 != 4 as std::os::raw::c_int
                            && (8 as std::os::raw::c_int != 8 as std::os::raw::c_int
                                || t1 != 5 as std::os::raw::c_int)
                        {
                            l1 = l1 as uint32_t as std::os::raw::c_ulong
                                | (if (*v1).type_0.t & 0x10 as std::os::raw::c_int != 0 {
                                    0 as std::os::raw::c_int as std::os::raw::c_ulong
                                } else {
                                    (l1 & 0x80000000 as std::os::raw::c_uint
                                        as std::os::raw::c_ulong)
                                        .wrapping_neg()
                                })
                        }
                        (*v1).c2rust_unnamed.c.i = l1;
                        vtop = vtop.offset(-1);
                        current_block = 2705889988320590074;
                    },
                }
            },
            145 => {
                current_block = 9531126118564703076;
                match current_block {
                    9531126118564703076 => {
                        l1 = (l1 != 0 || l2 != 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    337463635748514786 => {
                        l1 = (l1 as std::os::raw::c_ulong).wrapping_add(l2) as uint64_t as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    14513523936503887211 => {
                        l1 = (l1 as std::os::raw::c_ulong).wrapping_sub(l2) as uint64_t as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    13264933720371784297 => {
                        l1 &= l2;
                        current_block = 2116367355679836638;
                    },
                    6266472726643588263 => {
                        l1 ^= l2;
                        current_block = 2116367355679836638;
                    },
                    4912292479518253705 => {
                        l1 |= l2;
                        current_block = 2116367355679836638;
                    },
                    6484990028451387192 => {
                        l1 = (l1 as std::os::raw::c_ulong).wrapping_mul(l2) as uint64_t as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    15319502457978536222 => {
                        l1 <<= l2 & shm as std::os::raw::c_ulong;
                        current_block = 2116367355679836638;
                    },
                    7644865018902103128 => {
                        l1 >>= l2 & shm as std::os::raw::c_ulong;
                        current_block = 2116367355679836638;
                    },
                    14360337777247149493 => {
                        l1 = if l1 >> 63 as std::os::raw::c_int != 0 {
                            !(!l1 >> (l2 & shm as std::os::raw::c_ulong))
                        } else {
                            (l1) >> (l2 & shm as std::os::raw::c_ulong)
                        };
                        current_block = 2116367355679836638;
                    },
                    7983287325140408908 => {
                        l1 = (l1 < l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    11798621572665121461 => {
                        l1 = (l1 >= l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    13932873228663550769 => {
                        l1 = (l1 == l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    1788142895051060034 => {
                        l1 = (l1 != l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    9886615242100325262 => {
                        l1 = (l1 <= l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    2041506142873075795 => {
                        l1 = (l1 > l2) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    6863629183173758772 => {
                        l1 = gen_opic_lt(l1, l2) as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    8913536887710889399 => {
                        l1 = (gen_opic_lt(l1, l2) == 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    4631372686411729056 => {
                        l1 = (gen_opic_lt(l2, l1) == 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    11170670477888610567 => {
                        l1 = gen_opic_lt(l2, l1) as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    18380056810314386699 => {
                        l1 = (l1 != 0 && l2 != 0) as std::os::raw::c_int as uint64_t;
                        current_block = 2116367355679836638;
                    },
                    _ => {
                        if l2 == 0 as std::os::raw::c_int as std::os::raw::c_ulong {
                            if const_wanted != 0
                                && nocode_wanted & 0xffff as std::os::raw::c_int == 0
                            {
                                _tcc_error(
                                    b"division by zero in constant\x00" as *const u8
                                        as *const std::os::raw::c_char,
                                );
                            }
                            current_block = 10869693750139516408;
                        } else {
                            match op {
                                37 => l1 = l1.wrapping_sub(l2.wrapping_mul(gen_opic_sdiv(l1, l2))),
                                131 => l1 = l1.wrapping_div(l2),
                                132 => l1 = l1.wrapping_rem(l2),
                                _ => l1 = gen_opic_sdiv(l1, l2),
                            }
                            current_block = 2116367355679836638;
                        }
                    },
                }
                match current_block {
                    10869693750139516408 => {},
                    _ => {
                        if t1 != 4 as std::os::raw::c_int
                            && (8 as std::os::raw::c_int != 8 as std::os::raw::c_int
                                || t1 != 5 as std::os::raw::c_int)
                        {
                            l1 = l1 as uint32_t as std::os::raw::c_ulong
                                | (if (*v1).type_0.t & 0x10 as std::os::raw::c_int != 0 {
                                    0 as std::os::raw::c_int as std::os::raw::c_ulong
                                } else {
                                    (l1 & 0x80000000 as std::os::raw::c_uint
                                        as std::os::raw::c_ulong)
                                        .wrapping_neg()
                                })
                        }
                        (*v1).c2rust_unnamed.c.i = l1;
                        vtop = vtop.offset(-1);
                        current_block = 2705889988320590074;
                    },
                }
            },
            _ => {
                current_block = 10869693750139516408;
            },
        }
    } else {
        /* if commutative ops, put c2 as constant */
        if c1 != 0
            && (op == '+' as i32
                || op == '&' as i32
                || op == '^' as i32
                || op == '|' as i32
                || op == '*' as i32
                || op == 0x94 as std::os::raw::c_int
                || op == 0x95 as std::os::raw::c_int)
        {
            vswap();
            //l = l1, l1 = l2, l2 = l;
            c2 = c1; //c = c1, c1 = c2, c2 = c;
            l2 = l1
        }
        if const_wanted == 0
            && c1 != 0
            && (l1 == 0 as std::os::raw::c_int as std::os::raw::c_ulong
                && (op == '<' as i32 || op == 0x8b as std::os::raw::c_int || op == '>' as i32)
                || l1 == -(1 as std::os::raw::c_int) as std::os::raw::c_ulong && op == '>' as i32)
        {
            /* treat (0 << x), (0 >> x) and (-1 >> x) as constant */
            vtop = vtop.offset(-1);
            current_block = 2705889988320590074;
        } else if const_wanted == 0
            && c2 != 0
            && (l2 == 0 as std::os::raw::c_int as std::os::raw::c_ulong
                && (op == '&' as i32 || op == '*' as i32)
                || op == '|' as i32
                    && (l2 == -(1 as std::os::raw::c_int) as std::os::raw::c_ulong
                        || l2 == 0xffffffff as std::os::raw::c_uint as std::os::raw::c_ulong
                            && t2 != 4 as std::os::raw::c_int)
                || l2 == 1 as std::os::raw::c_int as std::os::raw::c_ulong
                    && (op == '%' as i32 || op == 0x84 as std::os::raw::c_int))
        {
            /* treat (x & 0), (x * 0), (x | -1) and (x % 1) as constant */
            if l2 == 1 as std::os::raw::c_int as std::os::raw::c_ulong {
                (*vtop).c2rust_unnamed.c.i = 0 as std::os::raw::c_int as uint64_t
            }
            vswap();
            vtop = vtop.offset(-1);
            current_block = 2705889988320590074;
        } else if c2 != 0
            && ((op == '*' as i32
                || op == '/' as i32
                || op == 0x83 as std::os::raw::c_int
                || op == 0x85 as std::os::raw::c_int)
                && l2 == 1 as std::os::raw::c_int as std::os::raw::c_ulong
                || (op == '+' as i32
                    || op == '-' as i32
                    || op == '|' as i32
                    || op == '^' as i32
                    || op == '<' as i32
                    || op == 0x8b as std::os::raw::c_int
                    || op == '>' as i32)
                    && l2 == 0 as std::os::raw::c_int as std::os::raw::c_ulong
                || op == '&' as i32
                    && (l2 == -(1 as std::os::raw::c_int) as std::os::raw::c_ulong
                        || l2 == 0xffffffff as std::os::raw::c_uint as std::os::raw::c_ulong
                            && t2 != 4 as std::os::raw::c_int))
        {
            /* filter out NOP operations like x*1, x-0, x&-1... */
            vtop = vtop.offset(-1);
            current_block = 2705889988320590074;
        } else if c2 != 0
            && (op == '*' as i32
                || op == 0x85 as std::os::raw::c_int
                || op == 0x83 as std::os::raw::c_int)
        {
            /* try to use shifts instead of muls or divs */
            if l2 > 0 as std::os::raw::c_int as std::os::raw::c_ulong
                && l2 & l2.wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong)
                    == 0 as std::os::raw::c_int as std::os::raw::c_ulong
            {
                let mut n: std::os::raw::c_int = -(1 as std::os::raw::c_int);
                while l2 != 0 {
                    l2 >>= 1 as std::os::raw::c_int;
                    n += 1
                }
                (*vtop).c2rust_unnamed.c.i = n as uint64_t;
                if op == '*' as i32 {
                    op = '<' as i32
                } else if op == 0x85 as std::os::raw::c_int {
                    op = '>' as i32
                } else {
                    op = 0x8b as std::os::raw::c_int
                }
            }
            current_block = 10869693750139516408;
        } else if c2 != 0
            && (op == '+' as i32 || op == '-' as i32)
            && ((*vtop.offset(-(1 as std::os::raw::c_int) as isize)).r as std::os::raw::c_int
                & (0x3f as std::os::raw::c_int
                    | 0x100 as std::os::raw::c_int
                    | 0x200 as std::os::raw::c_int)
                == 0x30 as std::os::raw::c_int | 0x200 as std::os::raw::c_int
                || (*vtop.offset(-(1 as std::os::raw::c_int) as isize)).r as std::os::raw::c_int
                    & (0x3f as std::os::raw::c_int | 0x100 as std::os::raw::c_int)
                    == 0x32 as std::os::raw::c_int)
        {
            /* symbol + constant case */
            if op == '-' as i32 {
                l2 = l2.wrapping_neg()
            }
            l2 = (l2 as std::os::raw::c_ulong).wrapping_add(
                (*vtop.offset(-(1 as std::os::raw::c_int) as isize))
                    .c2rust_unnamed
                    .c
                    .i,
            ) as uint64_t as uint64_t;
            /* The backends can't always deal with addends to symbols
            larger than +-1<<31.  Don't construct such.  */
            if l2 as std::os::raw::c_int as std::os::raw::c_ulong != l2 {
                current_block = 10869693750139516408;
            } else {
                vtop = vtop.offset(-1);
                (*vtop).c2rust_unnamed.c.i = l2;
                current_block = 2705889988320590074;
            }
        } else {
            current_block = 10869693750139516408;
        }
    }
    match current_block {
        10869693750139516408 => {
            /* call low level op generator */
            if t1 == 4 as std::os::raw::c_int
                || t2 == 4 as std::os::raw::c_int
                || 8 as std::os::raw::c_int == 8 as std::os::raw::c_int
                    && (t1 == 5 as std::os::raw::c_int || t2 == 5 as std::os::raw::c_int)
            {
                gen_opl(op);
            } else {
                gen_opi(op);
            }
        },
        _ => {},
    };
}
/* generate a floating point operation with constant propagation */
unsafe extern "C" fn gen_opif(mut op: std::os::raw::c_int) {
    let mut current_block: u64;
    let mut c1: std::os::raw::c_int = 0;
    let mut c2: std::os::raw::c_int = 0;
    let mut v1: *mut SValue = 0 as *mut SValue;
    let mut v2: *mut SValue = 0 as *mut SValue;
    let mut f1: f128::f128 = f128::f128::ZERO;
    let mut f2: f128::f128 = f128::f128::ZERO;
    v1 = vtop.offset(-(1 as std::os::raw::c_int as isize));
    v2 = vtop;
    if op == 0x81 as std::os::raw::c_int {
        v1 = v2
    }
    /* currently, we cannot do computations with forward symbols */
    c1 = ((*v1).r as std::os::raw::c_int
        & (0x3f as std::os::raw::c_int
            | 0x100 as std::os::raw::c_int
            | 0x200 as std::os::raw::c_int)
        == 0x30 as std::os::raw::c_int) as std::os::raw::c_int;
    c2 = ((*v2).r as std::os::raw::c_int
        & (0x3f as std::os::raw::c_int
            | 0x100 as std::os::raw::c_int
            | 0x200 as std::os::raw::c_int)
        == 0x30 as std::os::raw::c_int) as std::os::raw::c_int;
    if c1 != 0 && c2 != 0 {
        if (*v1).type_0.t == 8 as std::os::raw::c_int {
            f1 = f128::f128::new((*v1).c2rust_unnamed.c.f);
            f2 = f128::f128::new((*v2).c2rust_unnamed.c.f)
        } else if (*v1).type_0.t == 9 as std::os::raw::c_int {
            f1 = f128::f128::new((*v1).c2rust_unnamed.c.d);
            f2 = f128::f128::new((*v2).c2rust_unnamed.c.d)
        } else {
            f1 = (*v1).c2rust_unnamed.c.ld;
            f2 = (*v2).c2rust_unnamed.c.ld
        }
        /* NOTE: we only do constant propagation if finite number (not
        NaN or infinity) (ANSI spec) */
        if !(ieee_finite(f1.to_f64().unwrap()) != 0 || ieee_finite(f2.to_f64().unwrap()) == 0)
            && const_wanted == 0
        {
            current_block = 3159961340021847184;
        } else {
            match op {
                43 => {
                    current_block = 11138193540316728686;
                    match current_block {
                        9354959880478738666 => {
                            f1 = -f1;
                            current_block = 11745839857079083930;
                        },
                        11138193540316728686 => {
                            f1 += f2;
                            current_block = 6417057564578538666;
                        },
                        2863442863303056604 => {
                            f1 -= f2;
                            current_block = 6417057564578538666;
                        },
                        17213729574760675349 => {
                            f1 *= f2;
                            current_block = 6417057564578538666;
                        },
                        _ => {
                            if f2 == f128::f128::new(0.0f64) {
                                let mut x1: C2RustUnnamed_12 = C2RustUnnamed_12 { f: 0. };
                                let mut x2: C2RustUnnamed_12 = C2RustUnnamed_12 { f: 0. };
                                let mut y: C2RustUnnamed_12 = C2RustUnnamed_12 { f: 0. };
                                /* If not in initializer we need to potentially generate
                                FP exceptions at runtime, otherwise we want to fold.  */
                                if const_wanted == 0 {
                                    current_block = 3159961340021847184;
                                } else {
                                    /* the run-time result of 0.0/0.0 on x87, also of other compilers
                                    when used to compile the f1 /= f2 below, would be -nan */
                                    x1.f = f1.to_f32().unwrap(); /* infinity */
                                    x2.f = f2.to_f32().unwrap(); /* nan */
                                    if f1 == f128::f128::new(0.0f64) {
                                        y.u = 0x7fc00000 as std::os::raw::c_int
                                            as std::os::raw::c_uint
                                    } else {
                                        y.u = 0x7f800000 as std::os::raw::c_int
                                            as std::os::raw::c_uint
                                    } /* set sign */
                                    y.u |= (x1.u ^ x2.u) & 0x80000000 as std::os::raw::c_uint;
                                    f1 = f128::f128::new(y.f);
                                    current_block = 6417057564578538666;
                                }
                            } else {
                                f1 /= f2;
                                current_block = 6417057564578538666;
                            }
                        },
                    }
                    match current_block {
                        3159961340021847184 => {},
                        _ => {
                            match current_block {
                                6417057564578538666 => vtop = vtop.offset(-1),
                                _ => {},
                            }
                            /* XXX: overflow test ? */
                            if (*v1).type_0.t == 8 as std::os::raw::c_int {
                                (*v1).c2rust_unnamed.c.f = f1.to_f32().unwrap()
                            } else if (*v1).type_0.t == 9 as std::os::raw::c_int {
                                (*v1).c2rust_unnamed.c.d = f1.to_f64().unwrap()
                            } else {
                                (*v1).c2rust_unnamed.c.ld = f1
                            }
                            current_block = 7420279277351916581;
                        },
                    }
                },
                45 => {
                    current_block = 2863442863303056604;
                    match current_block {
                        9354959880478738666 => {
                            f1 = -f1;
                            current_block = 11745839857079083930;
                        },
                        11138193540316728686 => {
                            f1 += f2;
                            current_block = 6417057564578538666;
                        },
                        2863442863303056604 => {
                            f1 -= f2;
                            current_block = 6417057564578538666;
                        },
                        17213729574760675349 => {
                            f1 *= f2;
                            current_block = 6417057564578538666;
                        },
                        _ => {
                            if f2 == f128::f128::new(0.0f64) {
                                let mut x1: C2RustUnnamed_12 = C2RustUnnamed_12 { f: 0. };
                                let mut x2: C2RustUnnamed_12 = C2RustUnnamed_12 { f: 0. };
                                let mut y: C2RustUnnamed_12 = C2RustUnnamed_12 { f: 0. };
                                if const_wanted == 0 {
                                    current_block = 3159961340021847184;
                                } else {
                                    x1.f = f1.to_f32().unwrap();
                                    x2.f = f2.to_f32().unwrap();
                                    if f1 == f128::f128::new(0.0f64) {
                                        y.u = 0x7fc00000 as std::os::raw::c_int
                                            as std::os::raw::c_uint
                                    } else {
                                        y.u = 0x7f800000 as std::os::raw::c_int
                                            as std::os::raw::c_uint
                                    }
                                    y.u |= (x1.u ^ x2.u) & 0x80000000 as std::os::raw::c_uint;
                                    f1 = f128::f128::new(y.f);
                                    current_block = 6417057564578538666;
                                }
                            } else {
                                f1 /= f2;
                                current_block = 6417057564578538666;
                            }
                        },
                    }
                    match current_block {
                        3159961340021847184 => {},
                        _ => {
                            match current_block {
                                6417057564578538666 => vtop = vtop.offset(-1),
                                _ => {},
                            }
                            if (*v1).type_0.t == 8 as std::os::raw::c_int {
                                (*v1).c2rust_unnamed.c.f = f1.to_f32().unwrap()
                            } else if (*v1).type_0.t == 9 as std::os::raw::c_int {
                                (*v1).c2rust_unnamed.c.d = f1.to_f64().unwrap()
                            } else {
                                (*v1).c2rust_unnamed.c.ld = f1
                            }
                            current_block = 7420279277351916581;
                        },
                    }
                },
                42 => {
                    current_block = 17213729574760675349;
                    match current_block {
                        9354959880478738666 => {
                            f1 = -f1;
                            current_block = 11745839857079083930;
                        },
                        11138193540316728686 => {
                            f1 += f2;
                            current_block = 6417057564578538666;
                        },
                        2863442863303056604 => {
                            f1 -= f2;
                            current_block = 6417057564578538666;
                        },
                        17213729574760675349 => {
                            f1 *= f2;
                            current_block = 6417057564578538666;
                        },
                        _ => {
                            if f2 == f128::f128::new(0.0f64) {
                                let mut x1: C2RustUnnamed_12 = C2RustUnnamed_12 { f: 0. };
                                let mut x2: C2RustUnnamed_12 = C2RustUnnamed_12 { f: 0. };
                                let mut y: C2RustUnnamed_12 = C2RustUnnamed_12 { f: 0. };
                                if const_wanted == 0 {
                                    current_block = 3159961340021847184;
                                } else {
                                    x1.f = f1.to_f32().unwrap();
                                    x2.f = f2.to_f32().unwrap();
                                    if f1 == f128::f128::new(0.0f64) {
                                        y.u = 0x7fc00000 as std::os::raw::c_int
                                            as std::os::raw::c_uint
                                    } else {
                                        y.u = 0x7f800000 as std::os::raw::c_int
                                            as std::os::raw::c_uint
                                    }
                                    y.u |= (x1.u ^ x2.u) & 0x80000000 as std::os::raw::c_uint;
                                    f1 = f128::f128::new(y.f);
                                    current_block = 6417057564578538666;
                                }
                            } else {
                                f1 /= f2;
                                current_block = 6417057564578538666;
                            }
                        },
                    }
                    match current_block {
                        3159961340021847184 => {},
                        _ => {
                            match current_block {
                                6417057564578538666 => vtop = vtop.offset(-1),
                                _ => {},
                            }
                            if (*v1).type_0.t == 8 as std::os::raw::c_int {
                                (*v1).c2rust_unnamed.c.f = f1.to_f32().unwrap()
                            } else if (*v1).type_0.t == 9 as std::os::raw::c_int {
                                (*v1).c2rust_unnamed.c.d = f1.to_f64().unwrap()
                            } else {
                                (*v1).c2rust_unnamed.c.ld = f1
                            }
                            current_block = 7420279277351916581;
                        },
                    }
                },
                47 => {
                    current_block = 16679922382382824795;
                    match current_block {
                        9354959880478738666 => {
                            f1 = -f1;
                            current_block = 11745839857079083930;
                        },
                        11138193540316728686 => {
                            f1 += f2;
                            current_block = 6417057564578538666;
                        },
                        2863442863303056604 => {
                            f1 -= f2;
                            current_block = 6417057564578538666;
                        },
                        17213729574760675349 => {
                            f1 *= f2;
                            current_block = 6417057564578538666;
                        },
                        _ => {
                            if f2 == f128::f128::new(0.0f64) {
                                let mut x1: C2RustUnnamed_12 = C2RustUnnamed_12 { f: 0. };
                                let mut x2: C2RustUnnamed_12 = C2RustUnnamed_12 { f: 0. };
                                let mut y: C2RustUnnamed_12 = C2RustUnnamed_12 { f: 0. };
                                if const_wanted == 0 {
                                    current_block = 3159961340021847184;
                                } else {
                                    x1.f = f1.to_f32().unwrap();
                                    x2.f = f2.to_f32().unwrap();
                                    if f1 == f128::f128::new(0.0f64) {
                                        y.u = 0x7fc00000 as std::os::raw::c_int
                                            as std::os::raw::c_uint
                                    } else {
                                        y.u = 0x7f800000 as std::os::raw::c_int
                                            as std::os::raw::c_uint
                                    }
                                    y.u |= (x1.u ^ x2.u) & 0x80000000 as std::os::raw::c_uint;
                                    f1 = f128::f128::new(y.f);
                                    current_block = 6417057564578538666;
                                }
                            } else {
                                f1 /= f2;
                                current_block = 6417057564578538666;
                            }
                        },
                    }
                    match current_block {
                        3159961340021847184 => {},
                        _ => {
                            match current_block {
                                6417057564578538666 => vtop = vtop.offset(-1),
                                _ => {},
                            }
                            if (*v1).type_0.t == 8 as std::os::raw::c_int {
                                (*v1).c2rust_unnamed.c.f = f1.to_f32().unwrap()
                            } else if (*v1).type_0.t == 9 as std::os::raw::c_int {
                                (*v1).c2rust_unnamed.c.d = f1.to_f64().unwrap()
                            } else {
                                (*v1).c2rust_unnamed.c.ld = f1
                            }
                            current_block = 7420279277351916581;
                        },
                    }
                },
                129 => {
                    current_block = 9354959880478738666;
                    match current_block {
                        9354959880478738666 => {
                            f1 = -f1;
                            current_block = 11745839857079083930;
                        },
                        11138193540316728686 => {
                            f1 += f2;
                            current_block = 6417057564578538666;
                        },
                        2863442863303056604 => {
                            f1 -= f2;
                            current_block = 6417057564578538666;
                        },
                        17213729574760675349 => {
                            f1 *= f2;
                            current_block = 6417057564578538666;
                        },
                        _ => {
                            if f2 == f128::f128::new(0.0f64) {
                                let mut x1: C2RustUnnamed_12 = C2RustUnnamed_12 { f: 0. };
                                let mut x2: C2RustUnnamed_12 = C2RustUnnamed_12 { f: 0. };
                                let mut y: C2RustUnnamed_12 = C2RustUnnamed_12 { f: 0. };
                                if const_wanted == 0 {
                                    current_block = 3159961340021847184;
                                } else {
                                    x1.f = f1.to_f32().unwrap();
                                    x2.f = f2.to_f32().unwrap();
                                    if f1 == f128::f128::new(0.0f64) {
                                        y.u = 0x7fc00000 as std::os::raw::c_int
                                            as std::os::raw::c_uint
                                    } else {
                                        y.u = 0x7f800000 as std::os::raw::c_int
                                            as std::os::raw::c_uint
                                    }
                                    y.u |= (x1.u ^ x2.u) & 0x80000000 as std::os::raw::c_uint;
                                    f1 = f128::f128::new(y.f);
                                    current_block = 6417057564578538666;
                                }
                            } else {
                                f1 /= f2;
                                current_block = 6417057564578538666;
                            }
                        },
                    }
                    match current_block {
                        3159961340021847184 => {},
                        _ => {
                            match current_block {
                                6417057564578538666 => vtop = vtop.offset(-1),
                                _ => {},
                            }
                            if (*v1).type_0.t == 8 as std::os::raw::c_int {
                                (*v1).c2rust_unnamed.c.f = f1.to_f32().unwrap()
                            } else if (*v1).type_0.t == 9 as std::os::raw::c_int {
                                (*v1).c2rust_unnamed.c.d = f1.to_f64().unwrap()
                            } else {
                                (*v1).c2rust_unnamed.c.ld = f1
                            }
                            current_block = 7420279277351916581;
                        },
                    }
                },
                _ => {
                    current_block = 3159961340021847184;
                },
            }
        }
    } else {
        current_block = 3159961340021847184;
    }
    match current_block {
        3159961340021847184 =>
        /* XXX: also handles tests ? */
        {
            if op == 0x81 as std::os::raw::c_int {
                gen_opf(op);
            } else {
                gen_opf(op);
            }
        },
        _ => {},
    };
}
/* print a type. If 'varstr' is not NULL, then the variable is also
printed in the type */
/* XXX: union */
/* XXX: add array and function pointers */
unsafe extern "C" fn type_to_str(
    mut buf: *mut std::os::raw::c_char,
    mut buf_size: std::os::raw::c_int,
    mut type_0: *mut CType,
    mut varstr: *const std::os::raw::c_char,
) {
    let mut current_block: u64;
    let mut bt: std::os::raw::c_int = 0;
    let mut v: std::os::raw::c_int = 0;
    let mut t: std::os::raw::c_int = 0;
    let mut s: *mut Sym = 0 as *mut Sym;
    let mut sa: *mut Sym = 0 as *mut Sym;
    let mut buf1: [std::os::raw::c_char; 256] = [0; 256];
    let mut tstr: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    t = (*type_0).t;
    bt = t & 0xf as std::os::raw::c_int;
    *buf.offset(0 as std::os::raw::c_int as isize) = '\u{0}' as i32 as std::os::raw::c_char;
    if t & 0x1000 as std::os::raw::c_int != 0 {
        pstrcat(
            buf,
            buf_size as size_t,
            b"extern \x00" as *const u8 as *const std::os::raw::c_char,
        );
    }
    if t & 0x2000 as std::os::raw::c_int != 0 {
        pstrcat(
            buf,
            buf_size as size_t,
            b"static \x00" as *const u8 as *const std::os::raw::c_char,
        );
    }
    if t & 0x4000 as std::os::raw::c_int != 0 {
        pstrcat(
            buf,
            buf_size as size_t,
            b"typedef \x00" as *const u8 as *const std::os::raw::c_char,
        );
    }
    if t & 0x8000 as std::os::raw::c_int != 0 {
        pstrcat(
            buf,
            buf_size as size_t,
            b"inline \x00" as *const u8 as *const std::os::raw::c_char,
        );
    }
    if bt != 5 as std::os::raw::c_int {
        if t & 0x200 as std::os::raw::c_int != 0 {
            pstrcat(
                buf,
                buf_size as size_t,
                b"volatile \x00" as *const u8 as *const std::os::raw::c_char,
            );
        }
        if t & 0x100 as std::os::raw::c_int != 0 {
            pstrcat(
                buf,
                buf_size as size_t,
                b"const \x00" as *const u8 as *const std::os::raw::c_char,
            );
        }
    }
    if t & 0x20 as std::os::raw::c_int != 0 && bt == 1 as std::os::raw::c_int
        || t & 0x10 as std::os::raw::c_int != 0
            && (bt == 2 as std::os::raw::c_int
                || bt == 3 as std::os::raw::c_int
                || bt == 4 as std::os::raw::c_int)
            && !(t as std::os::raw::c_uint
                & (((1 as std::os::raw::c_uint)
                    << 6 as std::os::raw::c_int + 6 as std::os::raw::c_int)
                    .wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_uint)
                    << 20 as std::os::raw::c_int
                    | 0x80 as std::os::raw::c_int as std::os::raw::c_uint)
                == ((2 as std::os::raw::c_int) << 20 as std::os::raw::c_int)
                    as std::os::raw::c_uint)
    {
        pstrcat(
            buf,
            buf_size as size_t,
            if t & 0x10 as std::os::raw::c_int != 0 {
                b"unsigned \x00" as *const u8 as *const std::os::raw::c_char
            } else {
                b"signed \x00" as *const u8 as *const std::os::raw::c_char
            },
        );
    }
    buf_size = (buf_size as std::os::raw::c_ulong).wrapping_sub(strlen(buf)) as std::os::raw::c_int
        as std::os::raw::c_int;
    buf = buf.offset(strlen(buf) as isize);
    match bt {
        0 => {
            tstr = b"void\x00" as *const u8 as *const std::os::raw::c_char;
            current_block = 16180399039682407470;
        },
        11 => {
            tstr = b"_Bool\x00" as *const u8 as *const std::os::raw::c_char;
            current_block = 16180399039682407470;
        },
        1 => {
            tstr = b"char\x00" as *const u8 as *const std::os::raw::c_char;
            current_block = 16180399039682407470;
        },
        2 => {
            tstr = b"short\x00" as *const u8 as *const std::os::raw::c_char;
            current_block = 16180399039682407470;
        },
        3 => {
            tstr = b"int\x00" as *const u8 as *const std::os::raw::c_char;
            current_block = 15932415117630392578;
        },
        4 => {
            tstr = b"long long\x00" as *const u8 as *const std::os::raw::c_char;
            current_block = 15932415117630392578;
        },
        8 => {
            tstr = b"float\x00" as *const u8 as *const std::os::raw::c_char;
            current_block = 16180399039682407470;
        },
        9 => {
            tstr = b"double\x00" as *const u8 as *const std::os::raw::c_char;
            if t & 0x800 as std::os::raw::c_int == 0 {
                current_block = 16180399039682407470;
            } else {
                current_block = 7209060299604444501;
            }
        },
        10 => {
            current_block = 7209060299604444501;
        },
        7 => {
            tstr = b"struct \x00" as *const u8 as *const std::os::raw::c_char;
            if t as std::os::raw::c_uint
                & (((1 as std::os::raw::c_uint)
                    << 6 as std::os::raw::c_int + 6 as std::os::raw::c_int)
                    .wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_uint)
                    << 20 as std::os::raw::c_int
                    | 0x80 as std::os::raw::c_int as std::os::raw::c_uint
                    | 0xf as std::os::raw::c_int as std::os::raw::c_uint)
                == ((1 as std::os::raw::c_int) << 20 as std::os::raw::c_int
                    | 7 as std::os::raw::c_int) as std::os::raw::c_uint
            {
                tstr = b"union \x00" as *const u8 as *const std::os::raw::c_char
            }
            current_block = 8536721194329258740;
        },
        6 => {
            s = (*type_0).ref_0;
            buf1[0 as std::os::raw::c_int as usize] =
                0 as std::os::raw::c_int as std::os::raw::c_char;
            if !varstr.is_null() && '*' as i32 == *varstr as std::os::raw::c_int {
                pstrcat(
                    buf1.as_mut_ptr(),
                    ::std::mem::size_of::<[std::os::raw::c_char; 256]>() as std::os::raw::c_ulong,
                    b"(\x00" as *const u8 as *const std::os::raw::c_char,
                );
                pstrcat(
                    buf1.as_mut_ptr(),
                    ::std::mem::size_of::<[std::os::raw::c_char; 256]>() as std::os::raw::c_ulong,
                    varstr,
                );
                pstrcat(
                    buf1.as_mut_ptr(),
                    ::std::mem::size_of::<[std::os::raw::c_char; 256]>() as std::os::raw::c_ulong,
                    b")\x00" as *const u8 as *const std::os::raw::c_char,
                );
            }
            pstrcat(
                buf1.as_mut_ptr(),
                buf_size as size_t,
                b"(\x00" as *const u8 as *const std::os::raw::c_char,
            );
            sa = (*s).c2rust_unnamed_0.next;
            while !sa.is_null() {
                let mut buf2: [std::os::raw::c_char; 256] = [0; 256];
                type_to_str(
                    buf2.as_mut_ptr(),
                    ::std::mem::size_of::<[std::os::raw::c_char; 256]>() as std::os::raw::c_ulong
                        as std::os::raw::c_int,
                    &mut (*sa).type_0,
                    0 as *const std::os::raw::c_char,
                );
                pstrcat(
                    buf1.as_mut_ptr(),
                    ::std::mem::size_of::<[std::os::raw::c_char; 256]>() as std::os::raw::c_ulong,
                    buf2.as_mut_ptr(),
                );
                sa = (*sa).c2rust_unnamed_0.next;
                if !sa.is_null() {
                    pstrcat(
                        buf1.as_mut_ptr(),
                        ::std::mem::size_of::<[std::os::raw::c_char; 256]>()
                            as std::os::raw::c_ulong,
                        b", \x00" as *const u8 as *const std::os::raw::c_char,
                    );
                }
            }
            if (*s)
                .c2rust_unnamed
                .c2rust_unnamed
                .c2rust_unnamed
                .f
                .func_type() as std::os::raw::c_int
                == 3 as std::os::raw::c_int
            {
                pstrcat(
                    buf1.as_mut_ptr(),
                    ::std::mem::size_of::<[std::os::raw::c_char; 256]>() as std::os::raw::c_ulong,
                    b", ...\x00" as *const u8 as *const std::os::raw::c_char,
                );
            }
            pstrcat(
                buf1.as_mut_ptr(),
                ::std::mem::size_of::<[std::os::raw::c_char; 256]>() as std::os::raw::c_ulong,
                b")\x00" as *const u8 as *const std::os::raw::c_char,
            );
            type_to_str(buf, buf_size, &mut (*s).type_0, buf1.as_mut_ptr());
            current_block = 1209030638129645089;
        },
        5 => {
            s = (*type_0).ref_0;
            if t & 0x40 as std::os::raw::c_int != 0 {
                if !varstr.is_null() && '*' as i32 == *varstr as std::os::raw::c_int {
                    snprintf(
                        buf1.as_mut_ptr(),
                        ::std::mem::size_of::<[std::os::raw::c_char; 256]>()
                            as std::os::raw::c_ulong,
                        b"(%s)[%d]\x00" as *const u8 as *const std::os::raw::c_char,
                        varstr,
                        (*s).c2rust_unnamed.c2rust_unnamed.c,
                    );
                } else {
                    snprintf(
                        buf1.as_mut_ptr(),
                        ::std::mem::size_of::<[std::os::raw::c_char; 256]>()
                            as std::os::raw::c_ulong,
                        b"%s[%d]\x00" as *const u8 as *const std::os::raw::c_char,
                        if !varstr.is_null() {
                            varstr
                        } else {
                            b"\x00" as *const u8 as *const std::os::raw::c_char
                        },
                        (*s).c2rust_unnamed.c2rust_unnamed.c,
                    );
                }
                type_to_str(buf, buf_size, &mut (*s).type_0, buf1.as_mut_ptr());
            } else {
                pstrcpy(
                    buf1.as_mut_ptr(),
                    ::std::mem::size_of::<[std::os::raw::c_char; 256]>() as std::os::raw::c_ulong,
                    b"*\x00" as *const u8 as *const std::os::raw::c_char,
                );
                if t & 0x100 as std::os::raw::c_int != 0 {
                    pstrcat(
                        buf1.as_mut_ptr(),
                        buf_size as size_t,
                        b"const \x00" as *const u8 as *const std::os::raw::c_char,
                    );
                }
                if t & 0x200 as std::os::raw::c_int != 0 {
                    pstrcat(
                        buf1.as_mut_ptr(),
                        buf_size as size_t,
                        b"volatile \x00" as *const u8 as *const std::os::raw::c_char,
                    );
                }
                if !varstr.is_null() {
                    pstrcat(
                        buf1.as_mut_ptr(),
                        ::std::mem::size_of::<[std::os::raw::c_char; 256]>()
                            as std::os::raw::c_ulong,
                        varstr,
                    );
                }
                type_to_str(buf, buf_size, &mut (*s).type_0, buf1.as_mut_ptr());
            }
            current_block = 1209030638129645089;
        },
        _ => {
            current_block = 17711149709958600598;
        },
    }
    match current_block {
        15932415117630392578 => {
            if t & 0x800 as std::os::raw::c_int != 0 {
                tstr = b"long\x00" as *const u8 as *const std::os::raw::c_char
            }
            if !(t as std::os::raw::c_uint
                & (((1 as std::os::raw::c_uint)
                    << 6 as std::os::raw::c_int + 6 as std::os::raw::c_int)
                    .wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_uint)
                    << 20 as std::os::raw::c_int
                    | 0x80 as std::os::raw::c_int as std::os::raw::c_uint)
                == ((2 as std::os::raw::c_int) << 20 as std::os::raw::c_int)
                    as std::os::raw::c_uint)
            {
                current_block = 16180399039682407470;
            } else {
                tstr = b"enum \x00" as *const u8 as *const std::os::raw::c_char;
                current_block = 8536721194329258740;
            }
        },
        7209060299604444501 => {
            tstr = b"long double\x00" as *const u8 as *const std::os::raw::c_char;
            current_block = 16180399039682407470;
        },
        _ => {},
    }
    match current_block {
        8536721194329258740 => {
            pstrcat(buf, buf_size as size_t, tstr);
            v = (*(*type_0).ref_0).v & !(0x40000000 as std::os::raw::c_int);
            if v >= 0x10000000 as std::os::raw::c_int {
                pstrcat(
                    buf,
                    buf_size as size_t,
                    b"<anonymous>\x00" as *const u8 as *const std::os::raw::c_char,
                );
            } else {
                pstrcat(buf, buf_size as size_t, get_tok_str(v, 0 as *mut CValue));
            }
            current_block = 17711149709958600598;
        },
        16180399039682407470 => {
            pstrcat(buf, buf_size as size_t, tstr);
            current_block = 17711149709958600598;
        },
        _ => {},
    }
    match current_block {
        17711149709958600598 => {
            if !varstr.is_null() {
                pstrcat(
                    buf,
                    buf_size as size_t,
                    b" \x00" as *const u8 as *const std::os::raw::c_char,
                );
                pstrcat(buf, buf_size as size_t, varstr);
            }
        },
        _ => {},
    };
}
unsafe extern "C" fn type_incompatibility_error(
    mut st: *mut CType,
    mut dt: *mut CType,
    mut fmt: *const std::os::raw::c_char,
) {
    let mut buf1: [std::os::raw::c_char; 256] = [0; 256];
    let mut buf2: [std::os::raw::c_char; 256] = [0; 256];
    type_to_str(
        buf1.as_mut_ptr(),
        ::std::mem::size_of::<[std::os::raw::c_char; 256]>() as std::os::raw::c_ulong
            as std::os::raw::c_int,
        st,
        0 as *const std::os::raw::c_char,
    );
    type_to_str(
        buf2.as_mut_ptr(),
        ::std::mem::size_of::<[std::os::raw::c_char; 256]>() as std::os::raw::c_ulong
            as std::os::raw::c_int,
        dt,
        0 as *const std::os::raw::c_char,
    );
    _tcc_error(fmt, buf1.as_mut_ptr(), buf2.as_mut_ptr());
}
unsafe extern "C" fn type_incompatibility_warning(
    mut st: *mut CType,
    mut dt: *mut CType,
    mut fmt: *const std::os::raw::c_char,
) {
    let mut buf1: [std::os::raw::c_char; 256] = [0; 256];
    let mut buf2: [std::os::raw::c_char; 256] = [0; 256];
    type_to_str(
        buf1.as_mut_ptr(),
        ::std::mem::size_of::<[std::os::raw::c_char; 256]>() as std::os::raw::c_ulong
            as std::os::raw::c_int,
        st,
        0 as *const std::os::raw::c_char,
    );
    type_to_str(
        buf2.as_mut_ptr(),
        ::std::mem::size_of::<[std::os::raw::c_char; 256]>() as std::os::raw::c_ulong
            as std::os::raw::c_int,
        dt,
        0 as *const std::os::raw::c_char,
    );
    _tcc_warning(fmt, buf1.as_mut_ptr(), buf2.as_mut_ptr());
}
unsafe extern "C" fn pointed_size(mut type_0: *mut CType) -> std::os::raw::c_int {
    let mut align: std::os::raw::c_int = 0;
    return type_size(pointed_type(type_0), &mut align);
}
unsafe extern "C" fn vla_runtime_pointed_size(mut type_0: *mut CType) {
    let mut align: std::os::raw::c_int = 0;
    vla_runtime_type_size(pointed_type(type_0), &mut align);
}
#[inline]
unsafe extern "C" fn is_null_pointer(mut p: *mut SValue) -> std::os::raw::c_int {
    if (*p).r as std::os::raw::c_int
        & (0x3f as std::os::raw::c_int
            | 0x100 as std::os::raw::c_int
            | 0x200 as std::os::raw::c_int)
        != 0x30 as std::os::raw::c_int
    {
        return 0 as std::os::raw::c_int;
    }
    return ((*p).type_0.t & 0xf as std::os::raw::c_int == 3 as std::os::raw::c_int
        && (*p).c2rust_unnamed.c.i as uint32_t == 0 as std::os::raw::c_int as std::os::raw::c_uint
        || (*p).type_0.t & 0xf as std::os::raw::c_int == 4 as std::os::raw::c_int
            && (*p).c2rust_unnamed.c.i == 0 as std::os::raw::c_int as std::os::raw::c_ulong
        || (*p).type_0.t & 0xf as std::os::raw::c_int == 5 as std::os::raw::c_int
            && (if 8 as std::os::raw::c_int == 4 as std::os::raw::c_int {
                ((*p).c2rust_unnamed.c.i as uint32_t
                    == 0 as std::os::raw::c_int as std::os::raw::c_uint)
                    as std::os::raw::c_int
            } else {
                ((*p).c2rust_unnamed.c.i == 0 as std::os::raw::c_int as std::os::raw::c_ulong)
                    as std::os::raw::c_int
            }) != 0
            && (*pointed_type(&mut (*p).type_0)).t & 0xf as std::os::raw::c_int
                == 0 as std::os::raw::c_int
            && 0 as std::os::raw::c_int
                == (*pointed_type(&mut (*p).type_0)).t
                    & (0x100 as std::os::raw::c_int | 0x200 as std::os::raw::c_int))
        as std::os::raw::c_int;
}
/* compare function types. OLD functions match any new functions */
unsafe extern "C" fn is_compatible_func(
    mut type1: *mut CType,
    mut type2: *mut CType,
) -> std::os::raw::c_int {
    let mut s1: *mut Sym = 0 as *mut Sym;
    let mut s2: *mut Sym = 0 as *mut Sym;
    s1 = (*type1).ref_0;
    s2 = (*type2).ref_0;
    if (*s1)
        .c2rust_unnamed
        .c2rust_unnamed
        .c2rust_unnamed
        .f
        .func_call() as std::os::raw::c_int
        != (*s2)
            .c2rust_unnamed
            .c2rust_unnamed
            .c2rust_unnamed
            .f
            .func_call() as std::os::raw::c_int
    {
        return 0 as std::os::raw::c_int;
    }
    if (*s1)
        .c2rust_unnamed
        .c2rust_unnamed
        .c2rust_unnamed
        .f
        .func_type() as std::os::raw::c_int
        != (*s2)
            .c2rust_unnamed
            .c2rust_unnamed
            .c2rust_unnamed
            .f
            .func_type() as std::os::raw::c_int
        && (*s1)
            .c2rust_unnamed
            .c2rust_unnamed
            .c2rust_unnamed
            .f
            .func_type() as std::os::raw::c_int
            != 2 as std::os::raw::c_int
        && (*s2)
            .c2rust_unnamed
            .c2rust_unnamed
            .c2rust_unnamed
            .f
            .func_type() as std::os::raw::c_int
            != 2 as std::os::raw::c_int
    {
        return 0 as std::os::raw::c_int;
    }
    loop {
        if is_compatible_unqualified_types(&mut (*s1).type_0, &mut (*s2).type_0) == 0 {
            return 0 as std::os::raw::c_int;
        }
        if (*s1)
            .c2rust_unnamed
            .c2rust_unnamed
            .c2rust_unnamed
            .f
            .func_type() as std::os::raw::c_int
            == 2 as std::os::raw::c_int
            || (*s2)
                .c2rust_unnamed
                .c2rust_unnamed
                .c2rust_unnamed
                .f
                .func_type() as std::os::raw::c_int
                == 2 as std::os::raw::c_int
        {
            return 1 as std::os::raw::c_int;
        }
        s1 = (*s1).c2rust_unnamed_0.next;
        s2 = (*s2).c2rust_unnamed_0.next;
        if s1.is_null() {
            return s2.is_null() as std::os::raw::c_int;
        }
        if s2.is_null() {
            return 0 as std::os::raw::c_int;
        }
    }
}
/* return true if type1 and type2 are the same.  If unqualified is
  true, qualifiers on the types are ignored.
*/
unsafe extern "C" fn compare_types(
    mut type1: *mut CType,
    mut type2: *mut CType,
    mut unqualified: std::os::raw::c_int,
) -> std::os::raw::c_int {
    let mut bt1: std::os::raw::c_int = 0;
    let mut t1: std::os::raw::c_int = 0;
    let mut t2: std::os::raw::c_int = 0;
    t1 = ((*type1).t as std::os::raw::c_uint
        & !((0x1000 as std::os::raw::c_int
            | 0x2000 as std::os::raw::c_int
            | 0x4000 as std::os::raw::c_int
            | 0x8000 as std::os::raw::c_int) as std::os::raw::c_uint
            | (((1 as std::os::raw::c_uint)
                << 6 as std::os::raw::c_int + 6 as std::os::raw::c_int)
                .wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_uint)
                << 20 as std::os::raw::c_int
                | 0x80 as std::os::raw::c_int as std::os::raw::c_uint)))
        as std::os::raw::c_int;
    t2 = ((*type2).t as std::os::raw::c_uint
        & !((0x1000 as std::os::raw::c_int
            | 0x2000 as std::os::raw::c_int
            | 0x4000 as std::os::raw::c_int
            | 0x8000 as std::os::raw::c_int) as std::os::raw::c_uint
            | (((1 as std::os::raw::c_uint)
                << 6 as std::os::raw::c_int + 6 as std::os::raw::c_int)
                .wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_uint)
                << 20 as std::os::raw::c_int
                | 0x80 as std::os::raw::c_int as std::os::raw::c_uint)))
        as std::os::raw::c_int;
    if unqualified != 0 {
        /* strip qualifiers before comparing */
        t1 &= !(0x100 as std::os::raw::c_int | 0x200 as std::os::raw::c_int);
        t2 &= !(0x100 as std::os::raw::c_int | 0x200 as std::os::raw::c_int)
    }
    /* Default Vs explicit signedness only matters for char */
    if t1 & 0xf as std::os::raw::c_int != 1 as std::os::raw::c_int {
        t1 &= !(0x20 as std::os::raw::c_int);
        t2 &= !(0x20 as std::os::raw::c_int)
    }
    /* XXX: bitfields ? */
    if t1 != t2 {
        return 0 as std::os::raw::c_int;
    }
    if t1 & 0x40 as std::os::raw::c_int != 0
        && !((*(*type1).ref_0).c2rust_unnamed.c2rust_unnamed.c < 0 as std::os::raw::c_int
            || (*(*type2).ref_0).c2rust_unnamed.c2rust_unnamed.c < 0 as std::os::raw::c_int
            || (*(*type1).ref_0).c2rust_unnamed.c2rust_unnamed.c
                == (*(*type2).ref_0).c2rust_unnamed.c2rust_unnamed.c)
    {
        return 0 as std::os::raw::c_int;
    }
    /* test more complicated cases */
    bt1 = t1 & 0xf as std::os::raw::c_int;
    if bt1 == 5 as std::os::raw::c_int {
        type1 = pointed_type(type1);
        type2 = pointed_type(type2);
        return is_compatible_types(type1, type2);
    } else if bt1 == 7 as std::os::raw::c_int {
        return ((*type1).ref_0 == (*type2).ref_0) as std::os::raw::c_int;
    } else if bt1 == 6 as std::os::raw::c_int {
        return is_compatible_func(type1, type2);
    } else if (*type1).t as std::os::raw::c_uint
        & (((1 as std::os::raw::c_uint) << 6 as std::os::raw::c_int + 6 as std::os::raw::c_int)
            .wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_uint)
            << 20 as std::os::raw::c_int
            | 0x80 as std::os::raw::c_int as std::os::raw::c_uint)
        == ((2 as std::os::raw::c_int) << 20 as std::os::raw::c_int) as std::os::raw::c_uint
        && (*type2).t as std::os::raw::c_uint
            & (((1 as std::os::raw::c_uint) << 6 as std::os::raw::c_int + 6 as std::os::raw::c_int)
                .wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_uint)
                << 20 as std::os::raw::c_int
                | 0x80 as std::os::raw::c_int as std::os::raw::c_uint)
            == ((2 as std::os::raw::c_int) << 20 as std::os::raw::c_int) as std::os::raw::c_uint
    {
        /* If both are enums then they must be the same, if only one is then
        t1 and t2 must be equal, which was checked above already.  */
        return ((*type1).ref_0 == (*type2).ref_0) as std::os::raw::c_int;
    } else {
        return 1 as std::os::raw::c_int;
    };
}
/* Check if OP1 and OP2 can be "combined" with operation OP, the combined
type is stored in DEST if non-null (except for pointer plus/minus) . */
unsafe extern "C" fn combine_types(
    mut dest: *mut CType,
    mut op1: *mut SValue,
    mut op2: *mut SValue,
    mut op: std::os::raw::c_int,
) -> std::os::raw::c_int {
    let mut type1: *mut CType = &mut (*op1).type_0;
    let mut type2: *mut CType = &mut (*op2).type_0;
    let mut type_0: CType = CType {
        t: 0,
        ref_0: 0 as *const Sym as *mut Sym,
    };
    let mut t1: std::os::raw::c_int = (*type1).t;
    let mut t2: std::os::raw::c_int = (*type2).t;
    let mut bt1: std::os::raw::c_int = t1 & 0xf as std::os::raw::c_int;
    let mut bt2: std::os::raw::c_int = t2 & 0xf as std::os::raw::c_int;
    let mut ret: std::os::raw::c_int = 1 as std::os::raw::c_int;
    type_0.t = 0 as std::os::raw::c_int;
    type_0.ref_0 = 0 as *mut Sym;
    if bt1 == 0 as std::os::raw::c_int || bt2 == 0 as std::os::raw::c_int {
        ret = if op == '?' as i32 {
            1 as std::os::raw::c_int
        } else {
            0 as std::os::raw::c_int
        };
        /* NOTE: as an extension, we accept void on only one side */
        type_0.t = 0 as std::os::raw::c_int
    } else if bt1 == 5 as std::os::raw::c_int || bt2 == 5 as std::os::raw::c_int {
        if !(op == '+' as i32) {
            /* http://port70.net/~nsz/c/c99/n1256.html#6.5.15p6 */
            /* If one is a null ptr constant the result type is the other.  */
            if is_null_pointer(op2) != 0 {
                type_0 = *type1
            } else if is_null_pointer(op1) != 0 {
                type_0 = *type2
            } else if bt1 != bt2 {
                /* accept comparison or cond-expr between pointer and integer
                with a warning */
                if (op == '?' as i32
                    || op >= 0x90 as std::os::raw::c_int && op <= 0x9f as std::os::raw::c_int)
                    && (is_integer_btype(bt1) != 0 || is_integer_btype(bt2) != 0)
                {
                    _tcc_warning(
                        b"pointer/integer mismatch in %s\x00" as *const u8
                            as *const std::os::raw::c_char,
                        if op == '?' as i32 {
                            b"conditional expression\x00" as *const u8
                                as *const std::os::raw::c_char
                        } else {
                            b"comparison\x00" as *const u8 as *const std::os::raw::c_char
                        },
                    );
                } else if op != '-' as i32 || is_integer_btype(bt2) == 0 {
                    ret = 0 as std::os::raw::c_int
                }
                type_0 = *if bt1 == 5 as std::os::raw::c_int {
                    type1
                } else {
                    type2
                }
            } else {
                let mut pt1: *mut CType = pointed_type(type1);
                let mut pt2: *mut CType = pointed_type(type2);
                let mut pbt1: std::os::raw::c_int = (*pt1).t & 0xf as std::os::raw::c_int;
                let mut pbt2: std::os::raw::c_int = (*pt2).t & 0xf as std::os::raw::c_int;
                let mut newquals: std::os::raw::c_int = 0;
                let mut copied: std::os::raw::c_int = 0 as std::os::raw::c_int;
                if pbt1 != 0 as std::os::raw::c_int
                    && pbt2 != 0 as std::os::raw::c_int
                    && compare_types(pt1, pt2, 1 as std::os::raw::c_int) == 0
                {
                    if op != '?' as i32
                        && !(op >= 0x90 as std::os::raw::c_int && op <= 0x9f as std::os::raw::c_int)
                    {
                        ret = 0 as std::os::raw::c_int
                    } else {
                        type_incompatibility_warning(
                            type1,
                            type2,
                            if op == '?' as i32 {
                                b"pointer type mismatch in conditional expression (\'%s\' and \'%s\')\x00"
                                                             as *const u8 as
                                                             *const std::os::raw::c_char
                            } else {
                                b"pointer type mismatch in comparison(\'%s\' and \'%s\')\x00"
                                    as *const u8
                                    as *const std::os::raw::c_char
                            },
                        );
                    }
                }
                if op == '?' as i32 {
                    /* pointers to void get preferred, otherwise the
                    pointed to types minus qualifs should be compatible */
                    type_0 = *if pbt1 == 0 as std::os::raw::c_int {
                        type1
                    } else {
                        type2
                    };
                    /* combine qualifs */
                    newquals = ((*pt1).t | (*pt2).t)
                        & (0x100 as std::os::raw::c_int | 0x200 as std::os::raw::c_int);
                    if !(*pointed_type(&mut type_0)).t
                        & (0x100 as std::os::raw::c_int | 0x200 as std::os::raw::c_int)
                        & newquals
                        != 0
                    {
                        /* copy the pointer target symbol */
                        type_0.ref_0 = sym_push(
                            0x20000000 as std::os::raw::c_int,
                            &mut (*type_0.ref_0).type_0,
                            0 as std::os::raw::c_int,
                            (*type_0.ref_0).c2rust_unnamed.c2rust_unnamed.c,
                        );
                        copied = 1 as std::os::raw::c_int;
                        (*pointed_type(&mut type_0)).t |= newquals
                    }
                    /* pointers to incomplete arrays get converted to
                    pointers to completed ones if possible */
                    if (*pt1).t & 0x40 as std::os::raw::c_int != 0
                        && (*pt2).t & 0x40 as std::os::raw::c_int != 0
                        && (*(*pointed_type(&mut type_0)).ref_0)
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .c
                            < 0 as std::os::raw::c_int
                        && ((*(*pt1).ref_0).c2rust_unnamed.c2rust_unnamed.c
                            > 0 as std::os::raw::c_int
                            || (*(*pt2).ref_0).c2rust_unnamed.c2rust_unnamed.c
                                > 0 as std::os::raw::c_int)
                    {
                        if copied == 0 {
                            type_0.ref_0 = sym_push(
                                0x20000000 as std::os::raw::c_int,
                                &mut (*type_0.ref_0).type_0,
                                0 as std::os::raw::c_int,
                                (*type_0.ref_0).c2rust_unnamed.c2rust_unnamed.c,
                            )
                        }
                        let ref mut fresh4 = (*pointed_type(&mut type_0)).ref_0;
                        *fresh4 = sym_push(
                            0x20000000 as std::os::raw::c_int,
                            &mut (*(*(pointed_type
                                as unsafe extern "C" fn(_: *mut CType) -> *mut CType)(
                                &mut type_0,
                            ))
                            .ref_0)
                                .type_0,
                            0 as std::os::raw::c_int,
                            (*(*pointed_type(&mut type_0)).ref_0)
                                .c2rust_unnamed
                                .c2rust_unnamed
                                .c,
                        );
                        (*(*pointed_type(&mut type_0)).ref_0)
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .c = if (0 as std::os::raw::c_int)
                            < (*(*pt1).ref_0).c2rust_unnamed.c2rust_unnamed.c
                        {
                            (*(*pt1).ref_0).c2rust_unnamed.c2rust_unnamed.c
                        } else {
                            (*(*pt2).ref_0).c2rust_unnamed.c2rust_unnamed.c
                        }
                    }
                }
            }
        }
        if op >= 0x90 as std::os::raw::c_int && op <= 0x9f as std::os::raw::c_int {
            type_0.t = 0x800 as std::os::raw::c_int
                | 4 as std::os::raw::c_int
                | 0x10 as std::os::raw::c_int
        }
    } else if bt1 == 7 as std::os::raw::c_int || bt2 == 7 as std::os::raw::c_int {
        if op != '?' as i32 || compare_types(type1, type2, 1 as std::os::raw::c_int) == 0 {
            ret = 0 as std::os::raw::c_int
        }
        type_0 = *type1
    } else if is_float(bt1) != 0 || is_float(bt2) != 0 {
        if bt1 == 10 as std::os::raw::c_int || bt2 == 10 as std::os::raw::c_int {
            type_0.t = 10 as std::os::raw::c_int
        } else if bt1 == 9 as std::os::raw::c_int || bt2 == 9 as std::os::raw::c_int {
            type_0.t = 9 as std::os::raw::c_int
        } else {
            type_0.t = 8 as std::os::raw::c_int
        }
    } else if bt1 == 4 as std::os::raw::c_int || bt2 == 4 as std::os::raw::c_int {
        /* cast to biggest op */
        type_0.t = 4 as std::os::raw::c_int | 0x800 as std::os::raw::c_int;
        if bt1 == 4 as std::os::raw::c_int {
            type_0.t &= t1
        }
        if bt2 == 4 as std::os::raw::c_int {
            type_0.t &= t2
        }
        /* convert to unsigned if it does not fit in a long long */
        if t1
            & (0xf as std::os::raw::c_int
                | 0x10 as std::os::raw::c_int
                | 0x80 as std::os::raw::c_int)
            == 4 as std::os::raw::c_int | 0x10 as std::os::raw::c_int
            || t2
                & (0xf as std::os::raw::c_int
                    | 0x10 as std::os::raw::c_int
                    | 0x80 as std::os::raw::c_int)
                == 4 as std::os::raw::c_int | 0x10 as std::os::raw::c_int
        {
            type_0.t |= 0x10 as std::os::raw::c_int
        }
    } else {
        /* integer operations */
        type_0.t = 3 as std::os::raw::c_int | 0x800 as std::os::raw::c_int & (t1 | t2);
        /* convert to unsigned if it does not fit in an integer */
        if t1
            & (0xf as std::os::raw::c_int
                | 0x10 as std::os::raw::c_int
                | 0x80 as std::os::raw::c_int)
            == 3 as std::os::raw::c_int | 0x10 as std::os::raw::c_int
            || t2
                & (0xf as std::os::raw::c_int
                    | 0x10 as std::os::raw::c_int
                    | 0x80 as std::os::raw::c_int)
                == 3 as std::os::raw::c_int | 0x10 as std::os::raw::c_int
        {
            type_0.t |= 0x10 as std::os::raw::c_int
        }
    }
    if !dest.is_null() {
        *dest = type_0
    }
    return ret;
}
/* generic gen_op: handles types problems */
#[no_mangle]
pub unsafe extern "C" fn gen_op(mut op: std::os::raw::c_int) {
    let mut u: std::os::raw::c_int = 0;
    let mut t1: std::os::raw::c_int = 0;
    let mut t2: std::os::raw::c_int = 0;
    let mut bt1: std::os::raw::c_int = 0;
    let mut bt2: std::os::raw::c_int = 0;
    let mut t: std::os::raw::c_int = 0;
    let mut type1: CType = CType {
        t: 0,
        ref_0: 0 as *const Sym as *mut Sym,
    };
    let mut combtype: CType = CType {
        t: 0,
        ref_0: 0 as *const Sym as *mut Sym,
    };
    loop {
        t1 = (*vtop.offset(-(1 as std::os::raw::c_int) as isize))
            .type_0
            .t;
        t2 = (*vtop.offset(0 as std::os::raw::c_int as isize)).type_0.t;
        bt1 = t1 & 0xf as std::os::raw::c_int;
        bt2 = t2 & 0xf as std::os::raw::c_int;
        if !(bt1 == 6 as std::os::raw::c_int || bt2 == 6 as std::os::raw::c_int) {
            break;
        }
        if bt2 == 6 as std::os::raw::c_int {
            mk_pointer(&mut (*vtop).type_0);
            gaddrof();
        }
        if bt1 == 6 as std::os::raw::c_int {
            vswap();
            mk_pointer(&mut (*vtop).type_0);
            gaddrof();
            vswap();
        }
    }
    if combine_types(
        &mut combtype,
        vtop.offset(-(1 as std::os::raw::c_int as isize)),
        vtop,
        op,
    ) == 0
    {
        _tcc_error_noabort(
            b"invalid operand types for binary operation\x00" as *const u8
                as *const std::os::raw::c_char,
        );
        vpop();
    } else {
        let mut current_block_98: u64;
        if bt1 == 5 as std::os::raw::c_int || bt2 == 5 as std::os::raw::c_int {
            /* at least one operand is a pointer */
            /* relational op: must be both pointers */
            if op >= 0x90 as std::os::raw::c_int && op <= 0x9f as std::os::raw::c_int {
                current_block_98 = 14878558174716930703;
            } else {
                /* if both pointers, then it must be the '-' op */
                if bt1 == 5 as std::os::raw::c_int && bt2 == 5 as std::os::raw::c_int {
                    if op != '-' as i32 {
                        _tcc_error(
                            b"cannot use pointers here\x00" as *const u8
                                as *const std::os::raw::c_char,
                        );
                    }
                    if (*vtop.offset(-(1 as std::os::raw::c_int) as isize))
                        .type_0
                        .t
                        & 0x400 as std::os::raw::c_int
                        != 0
                    {
                        vla_runtime_pointed_size(
                            &mut (*vtop.offset(-(1 as std::os::raw::c_int) as isize)).type_0,
                        );
                    } else {
                        vpushi(pointed_size(
                            &mut (*vtop.offset(-(1 as std::os::raw::c_int) as isize)).type_0,
                        ));
                    }
                    vrott(3 as std::os::raw::c_int);
                    gen_opic(op);
                    (*vtop).type_0.t = 0x800 as std::os::raw::c_int | 4 as std::os::raw::c_int;
                    vswap();
                    gen_op(0x85 as std::os::raw::c_int);
                } else {
                    /* exactly one pointer : must be '+' or '-'. */
                    if op != '-' as i32 && op != '+' as i32 {
                        _tcc_error(
                            b"cannot use pointers here\x00" as *const u8
                                as *const std::os::raw::c_char,
                        );
                    }
                    /* Put pointer as first operand */
                    if bt2 == 5 as std::os::raw::c_int {
                        vswap();
                        t = t1;
                        t1 = t2;
                        t2 = t
                    }
                    type1 = (*vtop.offset(-(1 as std::os::raw::c_int) as isize)).type_0;
                    if (*vtop.offset(-(1 as std::os::raw::c_int) as isize))
                        .type_0
                        .t
                        & 0x400 as std::os::raw::c_int
                        != 0
                    {
                        vla_runtime_pointed_size(
                            &mut (*vtop.offset(-(1 as std::os::raw::c_int) as isize)).type_0,
                        );
                    } else {
                        u = pointed_size(
                            &mut (*vtop.offset(-(1 as std::os::raw::c_int) as isize)).type_0,
                        );
                        if u < 0 as std::os::raw::c_int {
                            _tcc_error(
                                b"unknown array element size\x00" as *const u8
                                    as *const std::os::raw::c_char,
                            );
                        }
                        vpushll(u as std::os::raw::c_longlong);
                    }
                    gen_op('*' as i32);
                    if (*tcc_state).do_bounds_check as std::os::raw::c_int != 0 && const_wanted == 0
                    {
                        /* if bounded pointers, we generate a special code to
                        test bounds */
                        if op == '-' as i32 {
                            vpushi(0 as std::os::raw::c_int);
                            vswap();
                            gen_op('-' as i32);
                        }
                        gen_bounded_ptr_add();
                    } else {
                        gen_opic(op);
                    }
                    type1.t &= !(0x40 as std::os::raw::c_int);
                    /* put again type if gen_opic() swaped operands */
                    (*vtop).type_0 = type1
                }
                current_block_98 = 3634396408142324656;
            }
        } else {
            /* floats can only be used for a few operations */
            if is_float(combtype.t) != 0
                && op != '+' as i32
                && op != '-' as i32
                && op != '*' as i32
                && op != '/' as i32
                && !(op >= 0x90 as std::os::raw::c_int && op <= 0x9f as std::os::raw::c_int)
            {
                _tcc_error(
                    b"invalid operands for binary operation\x00" as *const u8
                        as *const std::os::raw::c_char,
                );
            } else {
                if op == 0x8b as std::os::raw::c_int || op == '>' as i32 || op == '<' as i32 {
                    t = if bt1 == 4 as std::os::raw::c_int {
                        4 as std::os::raw::c_int
                    } else {
                        3 as std::os::raw::c_int
                    };
                    if t1
                        & (0xf as std::os::raw::c_int
                            | 0x10 as std::os::raw::c_int
                            | 0x80 as std::os::raw::c_int)
                        == t | 0x10 as std::os::raw::c_int
                    {
                        t |= 0x10 as std::os::raw::c_int
                    }
                    t |= 0x800 as std::os::raw::c_int & t1;
                    combtype.t = t
                }
            }
            current_block_98 = 14878558174716930703;
        }
        match current_block_98 {
            14878558174716930703 => {
                t2 = combtype.t;
                t = t2;
                /* XXX: currently, some unsigned operations are explicit, so
                we modify them here */
                if t & 0x10 as std::os::raw::c_int != 0 {
                    if op == '>' as i32 {
                        op = 0x8b as std::os::raw::c_int
                    } else if op == '/' as i32 {
                        op = 0x83 as std::os::raw::c_int
                    } else if op == '%' as i32 {
                        op = 0x84 as std::os::raw::c_int
                    } else if op == 0x9c as std::os::raw::c_int {
                        op = 0x92 as std::os::raw::c_int
                    } else if op == 0x9f as std::os::raw::c_int {
                        op = 0x97 as std::os::raw::c_int
                    } else if op == 0x9e as std::os::raw::c_int {
                        op = 0x96 as std::os::raw::c_int
                    } else if op == 0x9d as std::os::raw::c_int {
                        op = 0x93 as std::os::raw::c_int
                    }
                }
                vswap();
                gen_cast_s(t);
                vswap();
                /* special case for shifts and long long: we keep the shift as
                an integer */
                if op == 0x8b as std::os::raw::c_int || op == '>' as i32 || op == '<' as i32 {
                    t2 = 3 as std::os::raw::c_int
                }
                gen_cast_s(t2);
                if is_float(t) != 0 {
                    gen_opif(op);
                } else {
                    gen_opic(op);
                }
                if op >= 0x90 as std::os::raw::c_int && op <= 0x9f as std::os::raw::c_int {
                    /* relational op: the result is an int */
                    (*vtop).type_0.t = 3 as std::os::raw::c_int
                } else {
                    (*vtop).type_0.t = t
                }
            },
            _ => {},
        }
    }
    // Make sure that we have converted to an rvalue:
    if (*vtop).r as std::os::raw::c_int & 0x100 as std::os::raw::c_int != 0 {
        gv(
            if is_float((*vtop).type_0.t & 0xf as std::os::raw::c_int) != 0 {
                0x2 as std::os::raw::c_int
            } else {
                0x1 as std::os::raw::c_int
            },
        );
    };
}
/* generic itof for unsigned long long case */
unsafe extern "C" fn gen_cvt_itof1(mut t: std::os::raw::c_int) {
    if (*vtop).type_0.t & (0xf as std::os::raw::c_int | 0x10 as std::os::raw::c_int)
        == 4 as std::os::raw::c_int | 0x10 as std::os::raw::c_int
    {
        if t == 8 as std::os::raw::c_int {
            vpush_helper_func(TOK___floatundisf as std::os::raw::c_int);
        } else if t == 10 as std::os::raw::c_int {
            vpush_helper_func(TOK___floatundixf as std::os::raw::c_int);
        } else {
            vpush_helper_func(TOK___floatundidf as std::os::raw::c_int);
        }
        vrott(2 as std::os::raw::c_int);
        gfunc_call(1 as std::os::raw::c_int);
        vpushi(0 as std::os::raw::c_int);
        PUT_R_RET(vtop, t);
    } else {
        gen_cvt_itof(t);
    };
}
/* generic ftoi for unsigned long long case */
unsafe extern "C" fn gen_cvt_ftoi1(mut t: std::os::raw::c_int) {
    let mut st: std::os::raw::c_int = 0;
    if t == 4 as std::os::raw::c_int | 0x10 as std::os::raw::c_int {
        /* not handled natively */
        st = (*vtop).type_0.t & 0xf as std::os::raw::c_int;
        if st == 8 as std::os::raw::c_int {
            vpush_helper_func(TOK___fixunssfdi as std::os::raw::c_int);
        } else if st == 10 as std::os::raw::c_int {
            vpush_helper_func(TOK___fixunsxfdi as std::os::raw::c_int);
        } else {
            vpush_helper_func(TOK___fixunsdfdi as std::os::raw::c_int);
        }
        vrott(2 as std::os::raw::c_int);
        gfunc_call(1 as std::os::raw::c_int);
        vpushi(0 as std::os::raw::c_int);
        PUT_R_RET(vtop, t);
    } else {
        gen_cvt_ftoi(t);
    };
}
/* special delayed cast for char/short */
unsafe extern "C" fn force_charshort_cast() {
    let mut sbt: std::os::raw::c_int = if (((*vtop).r as std::os::raw::c_int
        & 0xc00 as std::os::raw::c_int)
        as std::os::raw::c_uint)
        .wrapping_div(
            ((0xc00 as std::os::raw::c_int
                & !((0xc00 as std::os::raw::c_int) << 1 as std::os::raw::c_int))
                as std::os::raw::c_uint)
                .wrapping_mul(1 as std::os::raw::c_int as std::os::raw::c_uint),
        )
        == 2 as std::os::raw::c_int as std::os::raw::c_uint
    {
        4 as std::os::raw::c_int
    } else {
        3 as std::os::raw::c_int
    };
    let mut dbt: std::os::raw::c_int = (*vtop).type_0.t;
    (*vtop).r = ((*vtop).r as std::os::raw::c_int & !(0xc00 as std::os::raw::c_int))
        as std::os::raw::c_ushort;
    (*vtop).type_0.t = sbt;
    gen_cast_s(if dbt == 11 as std::os::raw::c_int {
        (1 as std::os::raw::c_int) | 0x10 as std::os::raw::c_int
    } else {
        dbt
    });
    (*vtop).type_0.t = dbt;
}
unsafe extern "C" fn gen_cast_s(mut t: std::os::raw::c_int) {
    let mut type_0: CType = CType {
        t: 0,
        ref_0: 0 as *const Sym as *mut Sym,
    };
    type_0.t = t;
    type_0.ref_0 = 0 as *mut Sym;
    gen_cast(&mut type_0);
}
/* *******************************************************/
/* ------------------------------------------------------------------------- */
/* cast 'vtop' to 'type'. Casting to bitfields is forbidden. */
unsafe extern "C" fn gen_cast(mut type_0: *mut CType) {
    let mut current_block: u64;
    let mut sbt: std::os::raw::c_int = 0;
    let mut dbt: std::os::raw::c_int = 0;
    let mut sf: std::os::raw::c_int = 0;
    let mut df: std::os::raw::c_int = 0;
    let mut c: std::os::raw::c_int = 0;
    let mut dbt_bt: std::os::raw::c_int = 0;
    let mut sbt_bt: std::os::raw::c_int = 0;
    let mut ds: std::os::raw::c_int = 0;
    let mut ss: std::os::raw::c_int = 0;
    let mut bits: std::os::raw::c_int = 0;
    let mut trunc: std::os::raw::c_int = 0;
    /* special delayed cast for char/short */
    if (*vtop).r as std::os::raw::c_int & 0xc00 as std::os::raw::c_int != 0 {
        force_charshort_cast();
    }
    /* bitfields first get cast to ints */
    if (*vtop).type_0.t & 0x80 as std::os::raw::c_int != 0 {
        gv(0x1 as std::os::raw::c_int);
    }
    dbt = (*type_0).t & (0xf as std::os::raw::c_int | 0x10 as std::os::raw::c_int);
    sbt = (*vtop).type_0.t & (0xf as std::os::raw::c_int | 0x10 as std::os::raw::c_int);
    if sbt == 6 as std::os::raw::c_int {
        sbt = 5 as std::os::raw::c_int
    }
    'c_17414: loop {
        if !(sbt != dbt) {
            current_block = 5843665201414588178;
            break;
        }
        sf = is_float(sbt);
        df = is_float(dbt);
        dbt_bt = dbt & 0xf as std::os::raw::c_int;
        sbt_bt = sbt & 0xf as std::os::raw::c_int;
        if dbt_bt == 0 as std::os::raw::c_int {
            current_block = 5843665201414588178;
            break;
        }
        if sbt_bt == 0 as std::os::raw::c_int {
            current_block = 16236406533448175373;
        } else {
            current_block = 1054647088692577877;
        }
        loop {
            match current_block {
                16236406533448175373 => {
                    cast_error(&mut (*vtop).type_0, type_0);
                    current_block = 1054647088692577877;
                },
                _ => {
                    c = ((*vtop).r as std::os::raw::c_int
                        & (0x3f as std::os::raw::c_int
                            | 0x100 as std::os::raw::c_int
                            | 0x200 as std::os::raw::c_int)
                        == 0x30 as std::os::raw::c_int)
                        as std::os::raw::c_int;
                    if c != 0 {
                        /* constant case: we can do it now */
                        /* XXX: in ISOC, cannot do it if error in convert */
                        if sbt == 8 as std::os::raw::c_int {
                            (*vtop).c2rust_unnamed.c.ld =
                                f128::f128::new((*vtop).c2rust_unnamed.c.f)
                        } else if sbt == 9 as std::os::raw::c_int {
                            (*vtop).c2rust_unnamed.c.ld =
                                f128::f128::new((*vtop).c2rust_unnamed.c.d)
                        }
                        if df != 0 {
                            if sbt_bt == 4 as std::os::raw::c_int {
                                if sbt & 0x10 as std::os::raw::c_int != 0
                                    || (*vtop).c2rust_unnamed.c.i >> 63 as std::os::raw::c_int == 0
                                {
                                    (*vtop).c2rust_unnamed.c.ld =
                                        f128::f128::new((*vtop).c2rust_unnamed.c.i)
                                } else {
                                    (*vtop).c2rust_unnamed.c.ld =
                                        -f128::f128::new((*vtop).c2rust_unnamed.c.i.wrapping_neg())
                                }
                            } else if sf == 0 {
                                if sbt & 0x10 as std::os::raw::c_int != 0
                                    || (*vtop).c2rust_unnamed.c.i >> 31 as std::os::raw::c_int == 0
                                {
                                    (*vtop).c2rust_unnamed.c.ld =
                                        f128::f128::new((*vtop).c2rust_unnamed.c.i as uint32_t)
                                } else {
                                    (*vtop).c2rust_unnamed.c.ld = -f128::f128::new(
                                        ((*vtop).c2rust_unnamed.c.i as uint32_t).wrapping_neg(),
                                    )
                                }
                            }
                            if dbt == 8 as std::os::raw::c_int {
                                (*vtop).c2rust_unnamed.c.f =
                                    (*vtop).c2rust_unnamed.c.ld.to_f32().unwrap()
                            } else if dbt == 9 as std::os::raw::c_int {
                                (*vtop).c2rust_unnamed.c.d =
                                    (*vtop).c2rust_unnamed.c.ld.to_f64().unwrap()
                            }
                        } else if sf != 0 && dbt == 11 as std::os::raw::c_int {
                            (*vtop).c2rust_unnamed.c.i = ((*vtop).c2rust_unnamed.c.ld
                                != f128::f128::new(0 as std::os::raw::c_int))
                                as std::os::raw::c_int
                                as uint64_t
                        } else {
                            if sf != 0 {
                                (*vtop).c2rust_unnamed.c.i =
                                    (*vtop).c2rust_unnamed.c.ld.to_u64().unwrap()
                            } else if !(sbt_bt == 4 as std::os::raw::c_int
                                || 8 as std::os::raw::c_int == 8 as std::os::raw::c_int
                                    && sbt == 5 as std::os::raw::c_int)
                            {
                                if sbt & 0x10 as std::os::raw::c_int != 0 {
                                    (*vtop).c2rust_unnamed.c.i =
                                        (*vtop).c2rust_unnamed.c.i as uint32_t as uint64_t
                                } else {
                                    (*vtop).c2rust_unnamed.c.i = (*vtop).c2rust_unnamed.c.i
                                        as uint32_t
                                        as std::os::raw::c_ulong
                                        | ((*vtop).c2rust_unnamed.c.i
                                            & 0x80000000 as std::os::raw::c_uint
                                                as std::os::raw::c_ulong)
                                            .wrapping_neg()
                                }
                            }
                            if !(dbt_bt == 4 as std::os::raw::c_int
                                || 8 as std::os::raw::c_int == 8 as std::os::raw::c_int
                                    && dbt == 5 as std::os::raw::c_int)
                            {
                                if dbt == 11 as std::os::raw::c_int {
                                    (*vtop).c2rust_unnamed.c.i = ((*vtop).c2rust_unnamed.c.i
                                        != 0 as std::os::raw::c_int as std::os::raw::c_ulong)
                                        as std::os::raw::c_int
                                        as uint64_t
                                } else {
                                    let mut m: uint32_t = if dbt_bt == 1 as std::os::raw::c_int {
                                        0xff as std::os::raw::c_int as std::os::raw::c_uint
                                    } else if dbt_bt == 2 as std::os::raw::c_int {
                                        0xffff as std::os::raw::c_int as std::os::raw::c_uint
                                    } else {
                                        0xffffffff as std::os::raw::c_uint
                                    };
                                    (*vtop).c2rust_unnamed.c.i &= m as std::os::raw::c_ulong;
                                    if dbt & 0x10 as std::os::raw::c_int == 0 {
                                        (*vtop).c2rust_unnamed.c.i |= ((*vtop).c2rust_unnamed.c.i
                                            & (m >> 1 as std::os::raw::c_int).wrapping_add(
                                                1 as std::os::raw::c_int as std::os::raw::c_uint,
                                            )
                                                as std::os::raw::c_ulong)
                                            .wrapping_neg()
                                    }
                                }
                            }
                        }
                        current_block = 5843665201414588178;
                        break 'c_17414;
                    } else if dbt == 11 as std::os::raw::c_int
                        && (*vtop).r as std::os::raw::c_int
                            & (0x3f as std::os::raw::c_int
                                | 0x100 as std::os::raw::c_int
                                | 0x200 as std::os::raw::c_int)
                            == 0x30 as std::os::raw::c_int | 0x200 as std::os::raw::c_int
                    {
                        /* addresses are considered non-zero (see tcctest.c:sinit23) */
                        (*vtop).r = 0x30 as std::os::raw::c_int as std::os::raw::c_ushort;
                        (*vtop).c2rust_unnamed.c.i = 1 as std::os::raw::c_int as uint64_t;
                        current_block = 5843665201414588178;
                        break 'c_17414;
                    } else {
                        /* cannot generate code for global or static initializers */
                        if nocode_wanted as std::os::raw::c_uint
                            & 0xc0000000 as std::os::raw::c_uint
                            != 0
                        {
                            current_block = 5843665201414588178;
                            break 'c_17414;
                        }
                        /* non constant case: generate code */
                        if dbt == 11 as std::os::raw::c_int {
                            gen_test_zero(0x95 as std::os::raw::c_int);
                            current_block = 5843665201414588178;
                            break 'c_17414;
                        } else if sf != 0 || df != 0 {
                            if sf != 0 && df != 0 {
                                current_block = 14001958660280927786;
                                break;
                            } else {
                                current_block = 9437013279121998969;
                                break;
                            }
                        } else {
                            ds = btype_size(dbt_bt);
                            ss = btype_size(sbt_bt);
                            if ds == 0 as std::os::raw::c_int || ss == 0 as std::os::raw::c_int {
                                current_block = 16236406533448175373;
                                continue;
                            }
                            if (*type_0).t as std::os::raw::c_uint
                                & (((1 as std::os::raw::c_uint)
                                    << 6 as std::os::raw::c_int + 6 as std::os::raw::c_int)
                                    .wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_uint)
                                    << 20 as std::os::raw::c_int
                                    | 0x80 as std::os::raw::c_int as std::os::raw::c_uint)
                                == ((2 as std::os::raw::c_int) << 20 as std::os::raw::c_int)
                                    as std::os::raw::c_uint
                                && (*(*type_0).ref_0).c2rust_unnamed.c2rust_unnamed.c
                                    < 0 as std::os::raw::c_int
                            {
                                _tcc_error(
                                    b"cast to incomplete type\x00" as *const u8
                                        as *const std::os::raw::c_char,
                                );
                            }
                            /* same size and no sign conversion needed */
                            if ds == ss && ds >= 4 as std::os::raw::c_int {
                                current_block = 5843665201414588178;
                                break 'c_17414;
                            } else {
                                current_block = 6033931424626438518;
                                break 'c_17414;
                            }
                        }
                    }
                },
            }
        }
        match current_block {
            14001958660280927786 => {
                /* convert from fp to fp */
                gen_cvt_ftof(dbt);
                current_block = 5843665201414588178;
                break;
            },
            _ => {
                if df != 0 {
                    /* convert int to fp */
                    gen_cvt_itof1(dbt);
                    current_block = 5843665201414588178;
                    break;
                } else {
                    /* convert fp to int */
                    sbt = dbt;
                    if dbt_bt != 4 as std::os::raw::c_int && dbt_bt != 3 as std::os::raw::c_int {
                        sbt = 3 as std::os::raw::c_int
                    }
                    gen_cvt_ftoi1(sbt);
                    /* may need char/short cast */
                }
            },
        }
    }
    match current_block {
        6033931424626438518 => {
            if dbt_bt == 5 as std::os::raw::c_int || sbt_bt == 5 as std::os::raw::c_int {
                _tcc_warning(
                    b"cast between pointer and integer of different size\x00" as *const u8
                        as *const std::os::raw::c_char,
                );
                if sbt_bt == 5 as std::os::raw::c_int {
                    /* put integer type to allow logical operations below */
                    (*vtop).type_0.t = if 8 as std::os::raw::c_int == 8 as std::os::raw::c_int {
                        4 as std::os::raw::c_int
                    } else {
                        3 as std::os::raw::c_int
                    }
                }
            }
            /* processor allows { int a = 0, b = *(char*)&a; }
            That means that if we cast to less width, we can just
            change the type and read it still later. */
            if 1 as std::os::raw::c_int != 0
                && (*vtop).r as std::os::raw::c_int & 0x100 as std::os::raw::c_int != 0
            {
                /* value still in memory */
                if ds <= ss {
                    current_block = 5843665201414588178;
                } else if ds <= 4 as std::os::raw::c_int
                    && !(dbt == 2 as std::os::raw::c_int | 0x10 as std::os::raw::c_int
                        && sbt == 1 as std::os::raw::c_int)
                {
                    gv(0x1 as std::os::raw::c_int);
                    current_block = 5843665201414588178;
                /* ss <= 4 here */
                /* no 64bit envolved */
                } else {
                    current_block = 12070711452894729854;
                }
            } else {
                current_block = 12070711452894729854;
            }
            match current_block {
                5843665201414588178 => {},
                _ => {
                    gv(0x1 as std::os::raw::c_int);
                    trunc = 0 as std::os::raw::c_int;
                    if ds == 8 as std::os::raw::c_int {
                        /* need to convert from 32bit to 64bit */
                        if !(sbt & 0x10 as std::os::raw::c_int != 0) {
                            gen_cvt_sxtw();
                        }
                    } else {
                        if ss == 8 as std::os::raw::c_int {
                            /* RISC-V keeps 32bit vals in registers sign-extended.
                            So here we need a sign-extension for signed types and
                            zero-extension. for unsigned types. */
                            trunc = 32 as std::os::raw::c_int
                        /* zero upper 32 bits for non RISC-V targets */
                        } else {
                            ss = 4 as std::os::raw::c_int
                        }
                        if !(ds >= ss) {
                            if ss == 4 as std::os::raw::c_int {
                                gen_cvt_csti(dbt);
                            } else {
                                bits = (ss - ds) * 8 as std::os::raw::c_int;
                                /* for unsigned, gen_op will convert SAR to SHR */
                                (*vtop).type_0.t = (if ss == 8 as std::os::raw::c_int {
                                    4 as std::os::raw::c_int
                                } else {
                                    3 as std::os::raw::c_int
                                }) | dbt & 0x10 as std::os::raw::c_int;
                                vpushi(bits);
                                gen_op('<' as i32);
                                vpushi(bits - trunc);
                                gen_op('>' as i32);
                                vpushi(trunc);
                                gen_op(0x8b as std::os::raw::c_int);
                            }
                        }
                    }
                },
            }
        },
        _ => {},
    }
    (*vtop).type_0 = *type_0;
    (*vtop).type_0.t &= !(0x100 as std::os::raw::c_int
        | 0x200 as std::os::raw::c_int
        | 0x40 as std::os::raw::c_int);
}
/* return type size as known at compile time. Put alignment at 'a' */
#[no_mangle]
pub unsafe extern "C" fn type_size(
    mut type_0: *mut CType,
    mut a: *mut std::os::raw::c_int,
) -> std::os::raw::c_int {
    let mut s: *mut Sym = 0 as *mut Sym;
    let mut bt: std::os::raw::c_int = 0;
    bt = (*type_0).t & 0xf as std::os::raw::c_int;
    if bt == 7 as std::os::raw::c_int {
        /* struct/union */
        s = (*type_0).ref_0;
        *a = (*s).r as std::os::raw::c_int;
        return (*s).c2rust_unnamed.c2rust_unnamed.c;
    } else if bt == 5 as std::os::raw::c_int {
        if (*type_0).t & 0x40 as std::os::raw::c_int != 0 {
            let mut ts: std::os::raw::c_int = 0;
            s = (*type_0).ref_0;
            ts = type_size(&mut (*s).type_0, a);
            if ts < 0 as std::os::raw::c_int
                && (*s).c2rust_unnamed.c2rust_unnamed.c < 0 as std::os::raw::c_int
            {
                ts = -ts
            }
            return ts * (*s).c2rust_unnamed.c2rust_unnamed.c;
        } else {
            *a = 8 as std::os::raw::c_int;
            return 8 as std::os::raw::c_int;
        }
    } else if (*type_0).t as std::os::raw::c_uint
        & (((1 as std::os::raw::c_uint) << 6 as std::os::raw::c_int + 6 as std::os::raw::c_int)
            .wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_uint)
            << 20 as std::os::raw::c_int
            | 0x80 as std::os::raw::c_int as std::os::raw::c_uint)
        == ((2 as std::os::raw::c_int) << 20 as std::os::raw::c_int) as std::os::raw::c_uint
        && (*(*type_0).ref_0).c2rust_unnamed.c2rust_unnamed.c < 0 as std::os::raw::c_int
    {
        return -(1 as std::os::raw::c_int);
    /* incomplete enum */
    } else if bt == 10 as std::os::raw::c_int {
        *a = 16 as std::os::raw::c_int;
        return 16 as std::os::raw::c_int;
    } else if bt == 9 as std::os::raw::c_int || bt == 4 as std::os::raw::c_int {
        *a = 8 as std::os::raw::c_int;
        return 8 as std::os::raw::c_int;
    } else if bt == 3 as std::os::raw::c_int || bt == 8 as std::os::raw::c_int {
        *a = 4 as std::os::raw::c_int;
        return 4 as std::os::raw::c_int;
    } else if bt == 2 as std::os::raw::c_int {
        *a = 2 as std::os::raw::c_int;
        return 2 as std::os::raw::c_int;
    } else if bt == 13 as std::os::raw::c_int || bt == 14 as std::os::raw::c_int {
        *a = 8 as std::os::raw::c_int;
        return 16 as std::os::raw::c_int;
    } else {
        /* char, void, function, _Bool */
        *a = 1 as std::os::raw::c_int;
        return 1 as std::os::raw::c_int;
    };
}
/* push type size as known at runtime time on top of value stack. Put
alignment at 'a' */
unsafe extern "C" fn vla_runtime_type_size(
    mut type_0: *mut CType,
    mut a: *mut std::os::raw::c_int,
) {
    if (*type_0).t & 0x400 as std::os::raw::c_int != 0 {
        type_size(&mut (*(*type_0).ref_0).type_0, a);
        vset(
            &mut int_type,
            0x32 as std::os::raw::c_int | 0x100 as std::os::raw::c_int,
            (*(*type_0).ref_0).c2rust_unnamed.c2rust_unnamed.c,
        );
    } else {
        vpushi(type_size(type_0, a));
    };
}
/* return the pointed type of t */
#[inline]
unsafe extern "C" fn pointed_type(mut type_0: *mut CType) -> *mut CType {
    return &mut (*(*type_0).ref_0).type_0;
}
/* modify type so that its it is a pointer to type. */
#[no_mangle]
pub unsafe extern "C" fn mk_pointer(mut type_0: *mut CType) {
    let mut s: *mut Sym = 0 as *mut Sym;
    s = sym_push(
        0x20000000 as std::os::raw::c_int,
        type_0,
        0 as std::os::raw::c_int,
        -(1 as std::os::raw::c_int),
    );
    (*type_0).t = 5 as std::os::raw::c_int
        | (*type_0).t
            & (0x1000 as std::os::raw::c_int
                | 0x2000 as std::os::raw::c_int
                | 0x4000 as std::os::raw::c_int
                | 0x8000 as std::os::raw::c_int);
    (*type_0).ref_0 = s;
}
/* return true if type1 and type2 are exactly the same (including
   qualifiers).
*/
unsafe extern "C" fn is_compatible_types(
    mut type1: *mut CType,
    mut type2: *mut CType,
) -> std::os::raw::c_int {
    return compare_types(type1, type2, 0 as std::os::raw::c_int);
}
/* return true if type1 and type2 are the same (ignoring qualifiers).
*/
unsafe extern "C" fn is_compatible_unqualified_types(
    mut type1: *mut CType,
    mut type2: *mut CType,
) -> std::os::raw::c_int {
    return compare_types(type1, type2, 1 as std::os::raw::c_int);
}
unsafe extern "C" fn cast_error(mut st: *mut CType, mut dt: *mut CType) {
    type_incompatibility_error(
        st,
        dt,
        b"cannot convert \'%s\' to \'%s\'\x00" as *const u8 as *const std::os::raw::c_char,
    );
}
/* verify type compatibility to store vtop in 'dt' type */
unsafe extern "C" fn verify_assign_cast(mut dt: *mut CType) {
    let mut st: *mut CType = 0 as *mut CType; /* source type */
    let mut type1: *mut CType = 0 as *mut CType;
    let mut type2: *mut CType = 0 as *mut CType;
    let mut dbt: std::os::raw::c_int = 0;
    let mut sbt: std::os::raw::c_int = 0;
    let mut qualwarn: std::os::raw::c_int = 0;
    let mut lvl: std::os::raw::c_int = 0;
    st = &mut (*vtop).type_0;
    dbt = (*dt).t & 0xf as std::os::raw::c_int;
    sbt = (*st).t & 0xf as std::os::raw::c_int;
    if (*dt).t & 0x100 as std::os::raw::c_int != 0 {
        _tcc_warning(
            b"assignment of read-only location\x00" as *const u8 as *const std::os::raw::c_char,
        );
    }
    let mut current_block_26: u64;
    match dbt {
        0 => {
            if sbt != dbt {
                _tcc_error(
                    b"assignment to void expression\x00" as *const u8
                        as *const std::os::raw::c_char,
                );
            }
            current_block_26 = 14775119014532381840;
        },
        5 => {
            /* special cases for pointers */
            /* '0' can also be a pointer */
            if is_null_pointer(vtop) != 0 {
                current_block_26 = 14775119014532381840;
            } else if is_integer_btype(sbt) != 0 {
                _tcc_warning(
                    b"assignment makes pointer from integer without a cast\x00" as *const u8
                        as *const std::os::raw::c_char,
                );
                current_block_26 = 14775119014532381840;
            } else {
                type1 = pointed_type(dt);
                if sbt == 5 as std::os::raw::c_int {
                    type2 = pointed_type(st);
                    current_block_26 = 15976848397966268834;
                } else if sbt == 6 as std::os::raw::c_int {
                    /* accept implicit pointer to integer cast with warning */
                    type2 = st; /* a function is implicitly a function pointer */
                    current_block_26 = 15976848397966268834;
                } else {
                    current_block_26 = 9179797692622252840;
                }
                match current_block_26 {
                    9179797692622252840 => {},
                    _ => {
                        if is_compatible_types(type1, type2) != 0 {
                            current_block_26 = 14775119014532381840;
                        } else {
                            lvl = 0 as std::os::raw::c_int;
                            qualwarn = lvl;
                            loop {
                                if (*type2).t & 0x100 as std::os::raw::c_int != 0
                                    && (*type1).t & 0x100 as std::os::raw::c_int == 0
                                    || (*type2).t & 0x200 as std::os::raw::c_int != 0
                                        && (*type1).t & 0x200 as std::os::raw::c_int == 0
                                {
                                    qualwarn = 1 as std::os::raw::c_int
                                }
                                dbt = (*type1).t
                                    & (0xf as std::os::raw::c_int | 0x800 as std::os::raw::c_int);
                                sbt = (*type2).t
                                    & (0xf as std::os::raw::c_int | 0x800 as std::os::raw::c_int);
                                if dbt != 5 as std::os::raw::c_int
                                    || sbt != 5 as std::os::raw::c_int
                                {
                                    break;
                                }
                                type1 = pointed_type(type1);
                                type2 = pointed_type(type2);
                                lvl += 1
                            }
                            if is_compatible_unqualified_types(type1, type2) == 0 {
                                if (dbt == 0 as std::os::raw::c_int
                                    || sbt == 0 as std::os::raw::c_int)
                                    && lvl == 0 as std::os::raw::c_int
                                {
                                    /* void * can match anything */
                                    current_block_26 = 9853141518545631134;
                                } else if dbt == sbt
                                    && is_integer_btype(sbt & 0xf as std::os::raw::c_int) != 0
                                    && (((*type1).t as std::os::raw::c_uint
                                        & (((1 as std::os::raw::c_uint)
                                            << 6 as std::os::raw::c_int
                                                + 6 as std::os::raw::c_int)
                                            .wrapping_sub(
                                                1 as std::os::raw::c_int as std::os::raw::c_uint,
                                            )
                                            << 20 as std::os::raw::c_int
                                            | 0x80 as std::os::raw::c_int as std::os::raw::c_uint)
                                        == ((2 as std::os::raw::c_int) << 20 as std::os::raw::c_int)
                                            as std::os::raw::c_uint)
                                        as std::os::raw::c_int
                                        + ((*type2).t as std::os::raw::c_uint
                                            & (((1 as std::os::raw::c_uint)
                                                << 6 as std::os::raw::c_int
                                                    + 6 as std::os::raw::c_int)
                                                .wrapping_sub(
                                                    1 as std::os::raw::c_int
                                                        as std::os::raw::c_uint,
                                                )
                                                << 20 as std::os::raw::c_int
                                                | 0x80 as std::os::raw::c_int
                                                    as std::os::raw::c_uint)
                                            == ((2 as std::os::raw::c_int)
                                                << 20 as std::os::raw::c_int)
                                                as std::os::raw::c_uint)
                                            as std::os::raw::c_int
                                        + (((*type1).t ^ (*type2).t) & 0x10 as std::os::raw::c_int
                                            != 0)
                                            as std::os::raw::c_int)
                                        < 2 as std::os::raw::c_int
                                {
                                    current_block_26 = 9853141518545631134;
                                } else {
                                    _tcc_warning(
                                        b"assignment from incompatible pointer type\x00"
                                            as *const u8
                                            as *const std::os::raw::c_char,
                                    );
                                    current_block_26 = 14775119014532381840;
                                }
                            } else {
                                current_block_26 = 9853141518545631134;
                            }
                            match current_block_26 {
                                14775119014532381840 => {},
                                _ =>
                                /* Like GCC don't warn by default for merely changes
                                in pointer target signedness.  Do warn for different
                                base types, though, in particular for unsigned enums
                                and signed int targets.  */
                                {
                                    if qualwarn != 0 {
                                        _tcc_warning(b"assignment discards qualifiers from pointer target type\x00"
                                                         as *const u8 as
                                                         *const std::os::raw::c_char);
                                    }
                                    current_block_26 = 14775119014532381840;
                                }
                            }
                        }
                    },
                }
            }
        },
        1 | 2 | 3 | 4 => {
            if sbt == 5 as std::os::raw::c_int || sbt == 6 as std::os::raw::c_int {
                _tcc_warning(
                    b"assignment makes integer from pointer without a cast\x00" as *const u8
                        as *const std::os::raw::c_char,
                );
                current_block_26 = 14775119014532381840;
            } else if sbt == 7 as std::os::raw::c_int {
                current_block_26 = 9987932812209580714;
            } else {
                current_block_26 = 14775119014532381840;
            }
        },
        7 => {
            current_block_26 = 9987932812209580714;
        },
        _ => {
            current_block_26 = 14775119014532381840;
        },
    }
    match current_block_26 {
        9987932812209580714 => {
            if is_compatible_unqualified_types(dt, st) == 0 {
                current_block_26 = 9179797692622252840;
            } else {
                current_block_26 = 14775119014532381840;
            }
        },
        _ => {},
    }
    match current_block_26 {
        9179797692622252840 => {
            cast_error(st, dt);
        },
        _ => {},
    };
}
unsafe extern "C" fn gen_assign_cast(mut dt: *mut CType) {
    verify_assign_cast(dt);
    gen_cast(dt);
}
/* store vtop in lvalue pushed on stack */
#[no_mangle]
pub unsafe extern "C" fn vstore() {
    let mut sbt: std::os::raw::c_int = 0;
    let mut dbt: std::os::raw::c_int = 0;
    let mut ft: std::os::raw::c_int = 0;
    let mut r: std::os::raw::c_int = 0;
    let mut size: std::os::raw::c_int = 0;
    let mut align: std::os::raw::c_int = 0;
    let mut bit_size: std::os::raw::c_int = 0;
    let mut bit_pos: std::os::raw::c_int = 0;
    let mut delayed_cast: std::os::raw::c_int = 0;
    ft = (*vtop.offset(-(1 as std::os::raw::c_int) as isize))
        .type_0
        .t;
    sbt = (*vtop).type_0.t & 0xf as std::os::raw::c_int;
    dbt = ft & 0xf as std::os::raw::c_int;
    verify_assign_cast(&mut (*vtop.offset(-(1 as std::os::raw::c_int) as isize)).type_0);
    if sbt == 7 as std::os::raw::c_int {
        /* if structure, only generate pointer */
        /* structure assignment : generate memcpy */
        /* XXX: optimize if small size */
        size = type_size(&mut (*vtop).type_0, &mut align);
        /* leave source on stack */
        vswap();
        if (*vtop).r as std::os::raw::c_int & 0x4000 as std::os::raw::c_int != 0 {
            gbound();
        }
        (*vtop).type_0.t = 5 as std::os::raw::c_int;
        gaddrof();
        vpush_helper_func(TOK_memmove as std::os::raw::c_int);
        vswap();
        vpushv(vtop.offset(-(2 as std::os::raw::c_int as isize)));
        if (*vtop).r as std::os::raw::c_int & 0x4000 as std::os::raw::c_int != 0 {
            gbound();
        }
        (*vtop).type_0.t = 5 as std::os::raw::c_int;
        gaddrof();
        vpushi(size);
        gfunc_call(3 as std::os::raw::c_int);
    } else if ft & 0x80 as std::os::raw::c_int != 0 {
        /* destination */
        /* check would be wrong after gaddrof() */
        /* address of memcpy() */
        /* Use memmove, rather than memcpy, as dest and src may be same: */
        /* source */
        /* type size */
        /* bitfield store handling */
        /* save lvalue as expression result (example: s.b = s.a = n;) */
        vdup();
        *vtop.offset(-(1 as std::os::raw::c_int) as isize) =
            *vtop.offset(-(2 as std::os::raw::c_int) as isize);
        bit_pos = ft >> 20 as std::os::raw::c_int & 0x3f as std::os::raw::c_int;
        bit_size = ft >> 20 as std::os::raw::c_int + 6 as std::os::raw::c_int
            & 0x3f as std::os::raw::c_int;
        /* remove bit field info to avoid loops */
        (*vtop.offset(-(1 as std::os::raw::c_int) as isize))
            .type_0
            .t = (ft as std::os::raw::c_uint
            & !(((1 as std::os::raw::c_uint)
                << 6 as std::os::raw::c_int + 6 as std::os::raw::c_int)
                .wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_uint)
                << 20 as std::os::raw::c_int
                | 0x80 as std::os::raw::c_int as std::os::raw::c_uint))
            as std::os::raw::c_int;
        if dbt == 11 as std::os::raw::c_int {
            gen_cast(&mut (*vtop.offset(-(1 as std::os::raw::c_int) as isize)).type_0);
            (*vtop.offset(-(1 as std::os::raw::c_int) as isize))
                .type_0
                .t = (*vtop.offset(-(1 as std::os::raw::c_int) as isize))
                .type_0
                .t
                & !(0xf as std::os::raw::c_int)
                | (1 as std::os::raw::c_int | 0x10 as std::os::raw::c_int)
        }
        r = adjust_bf(
            vtop.offset(-(1 as std::os::raw::c_int as isize)),
            bit_pos,
            bit_size,
        );
        if dbt != 11 as std::os::raw::c_int {
            gen_cast(&mut (*vtop.offset(-(1 as std::os::raw::c_int) as isize)).type_0);
            dbt = (*vtop.offset(-(1 as std::os::raw::c_int) as isize))
                .type_0
                .t
                & 0xf as std::os::raw::c_int
        }
        if r == 7 as std::os::raw::c_int {
            store_packed_bf(bit_pos, bit_size);
        } else {
            let mut mask: std::os::raw::c_ulonglong = ((1 as std::os::raw::c_ulonglong)
                << bit_size)
                .wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulonglong);
            if dbt != 11 as std::os::raw::c_int {
                /* mask source */
                if dbt == 4 as std::os::raw::c_int {
                    vpushll(mask as std::os::raw::c_longlong);
                } else {
                    vpushi(mask as std::os::raw::c_uint as std::os::raw::c_int);
                }
                gen_op('&' as i32);
            }
            /* shift source */
            vpushi(bit_pos);
            gen_op('<' as i32);
            vswap();
            /* duplicate destination */
            vdup();
            vrott(3 as std::os::raw::c_int);
            /* load destination, mask and or with source */
            if dbt == 4 as std::os::raw::c_int {
                vpushll(!(mask << bit_pos) as std::os::raw::c_longlong);
            } else {
                vpushi(!((mask as std::os::raw::c_uint) << bit_pos) as std::os::raw::c_int);
            }
            gen_op('&' as i32);
            gen_op('|' as i32);
            /* store result */
            vstore();
            /* ... and discard */
            vpop();
        }
    } else if dbt == 0 as std::os::raw::c_int {
        vtop = vtop.offset(-1)
    } else {
        /* optimize char/short casts */
        delayed_cast = 0 as std::os::raw::c_int;
        if (dbt == 1 as std::os::raw::c_int || dbt == 2 as std::os::raw::c_int)
            && is_integer_btype(sbt) != 0
        {
            if (*vtop).r as std::os::raw::c_int & 0xc00 as std::os::raw::c_int != 0
                && btype_size(dbt) > btype_size(sbt)
            {
                force_charshort_cast();
            }
            delayed_cast = 1 as std::os::raw::c_int
        } else {
            gen_cast(&mut (*vtop.offset(-(1 as std::os::raw::c_int) as isize)).type_0);
        }
        /* NOT vpop() because on x86 it would flush the fp stack */
        if (*vtop.offset(-(1 as std::os::raw::c_int) as isize)).r as std::os::raw::c_int
            & 0x4000 as std::os::raw::c_int
            != 0
        {
            vswap();
            gbound();
            vswap();
        }
        gv(RC_TYPE(dbt));
        if delayed_cast != 0 {
            (*vtop).r = ((*vtop).r as std::os::raw::c_uint
                | ((0xc00 as std::os::raw::c_int
                    & !((0xc00 as std::os::raw::c_int) << 1 as std::os::raw::c_int))
                    as std::os::raw::c_uint)
                    .wrapping_mul(
                        ((sbt == 4 as std::os::raw::c_int) as std::os::raw::c_int
                            + 1 as std::os::raw::c_int)
                            as std::os::raw::c_uint,
                    )) as std::os::raw::c_ushort;
            /* bound check case */
            /* generate value */
            //tcc_warning("deley cast %x -> %x", sbt, dbt);
            (*vtop).type_0.t = (ft as std::os::raw::c_uint
                & !((0x1000 as std::os::raw::c_int
                    | 0x2000 as std::os::raw::c_int
                    | 0x4000 as std::os::raw::c_int
                    | 0x8000 as std::os::raw::c_int) as std::os::raw::c_uint
                    | (((1 as std::os::raw::c_uint)
                        << 6 as std::os::raw::c_int + 6 as std::os::raw::c_int)
                        .wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_uint)
                        << 20 as std::os::raw::c_int
                        | 0x80 as std::os::raw::c_int as std::os::raw::c_uint)))
                as std::os::raw::c_int
        }
        if (*vtop.offset(-(1 as std::os::raw::c_int) as isize)).r as std::os::raw::c_int
            & 0x3f as std::os::raw::c_int
            == 0x31 as std::os::raw::c_int
        {
            let mut sv: SValue = SValue {
                type_0: CType {
                    t: 0,
                    ref_0: 0 as *const Sym as *mut Sym,
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
            r = get_reg(0x1 as std::os::raw::c_int);
            sv.type_0.t = 0x800 as std::os::raw::c_int | 4 as std::os::raw::c_int;
            sv.r = (0x32 as std::os::raw::c_int | 0x100 as std::os::raw::c_int)
                as std::os::raw::c_ushort;
            sv.c2rust_unnamed.c.i = (*vtop.offset(-(1 as std::os::raw::c_int) as isize))
                .c2rust_unnamed
                .c
                .i;
            load(r, &mut sv);
            (*vtop.offset(-(1 as std::os::raw::c_int) as isize)).r =
                (r | 0x100 as std::os::raw::c_int) as std::os::raw::c_ushort
        }
        r = (*vtop).r as std::os::raw::c_int & 0x3f as std::os::raw::c_int;
        if R2_RET(dbt) != 0x30 as std::os::raw::c_int {
            let mut load_type: std::os::raw::c_int = if dbt == 14 as std::os::raw::c_int {
                9 as std::os::raw::c_int
            } else {
                (0x800 as std::os::raw::c_int) | 4 as std::os::raw::c_int
            };
            (*vtop.offset(-(1 as std::os::raw::c_int) as isize))
                .type_0
                .t = load_type;
            store(r, vtop.offset(-(1 as std::os::raw::c_int as isize)));
            vswap();
            /* if lvalue was saved on stack, must read it */
            /* two word case handling :
            store second register at word + 4 (or +8 for x86-64)  */
            /* convert to int to increment easily */
            (*vtop).type_0.t = 0x800 as std::os::raw::c_int | 4 as std::os::raw::c_int;
            gaddrof();
            vpushs(8 as std::os::raw::c_int as Elf64_Addr);
            gen_op('+' as i32);
            (*vtop).r = ((*vtop).r as std::os::raw::c_int | 0x100 as std::os::raw::c_int)
                as std::os::raw::c_ushort;
            vswap();
            (*vtop.offset(-(1 as std::os::raw::c_int) as isize))
                .type_0
                .t = load_type;
            /* XXX: it works because r2 is spilled last ! */
            store(
                (*vtop).r2 as std::os::raw::c_int,
                vtop.offset(-(1 as std::os::raw::c_int as isize)),
            );
        } else {
            /* single word */
            store(r, vtop.offset(-(1 as std::os::raw::c_int as isize)));
        }
        vswap();
        vtop = vtop.offset(-1)
    };
}
/* post defines POST/PRE add. c is the token ++ or -- */
#[no_mangle]
pub unsafe extern "C" fn inc(mut post: std::os::raw::c_int, mut c: std::os::raw::c_int) {
    test_lvalue(); /* save lvalue */
    vdup(); /* duplicate value */
    if post != 0 {
        gv_dup();
        vrotb(3 as std::os::raw::c_int);
        vrotb(3 as std::os::raw::c_int);
    }
    /* add constant */
    vpushi(c - 0x81 as std::os::raw::c_int); /* store value */
    gen_op('+' as i32);
    vstore();
    if post != 0 {
        vpop();
    };
    /* if post op, return saved value */
}
#[no_mangle]
pub unsafe extern "C" fn parse_mult_str(
    mut astr: *mut CString,
    mut msg: *const std::os::raw::c_char,
) {
    /* read the string */
    if tok != 0xc8 as std::os::raw::c_int {
        expect(msg);
    }
    cstr_new(astr);
    while tok == 0xc8 as std::os::raw::c_int {
        /* XXX: add \0 handling too ? */
        cstr_cat(
            astr,
            tokc.str_0.data as *const std::os::raw::c_char,
            -(1 as std::os::raw::c_int),
        );
        next();
    }
    cstr_ccat(astr, '\u{0}' as i32);
}
/* If I is >= 1 and a power of two, returns log2(i)+1.
If I is 0 returns 0.  */
#[no_mangle]
pub unsafe extern "C" fn exact_log2p1(mut i: std::os::raw::c_int) -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0;
    if i == 0 {
        return 0 as std::os::raw::c_int;
    }
    ret = 1 as std::os::raw::c_int;
    while i >= (1 as std::os::raw::c_int) << 8 as std::os::raw::c_int {
        i >>= 8 as std::os::raw::c_int;
        ret += 8 as std::os::raw::c_int
    }
    if i >= (1 as std::os::raw::c_int) << 4 as std::os::raw::c_int {
        ret += 4 as std::os::raw::c_int;
        i >>= 4 as std::os::raw::c_int
    }
    if i >= (1 as std::os::raw::c_int) << 2 as std::os::raw::c_int {
        ret += 2 as std::os::raw::c_int;
        i >>= 2 as std::os::raw::c_int
    }
    if i >= (1 as std::os::raw::c_int) << 1 as std::os::raw::c_int {
        ret += 1
    }
    return ret;
}
/* Parse __attribute__((...)) GNUC extension. */
unsafe extern "C" fn parse_attribute(mut ad: *mut AttributeDef) {
    let mut t: std::os::raw::c_int = 0;
    let mut n: std::os::raw::c_int = 0;
    let mut astr: CString = CString {
        size: 0,
        data: 0 as *const std::os::raw::c_void as *mut std::os::raw::c_void,
        size_allocated: 0,
    };
    loop {
        if tok != TOK_ATTRIBUTE1 as std::os::raw::c_int
            && tok != TOK_ATTRIBUTE2 as std::os::raw::c_int
        {
            return;
        }
        next();
        skip('(' as i32);
        skip('(' as i32);
        while tok != ')' as i32 {
            if tok < 256 as std::os::raw::c_int {
                expect(b"attribute name\x00" as *const u8 as *const std::os::raw::c_char);
            }
            t = tok;
            next();
            match t {
                368 | 369 => {
                    let mut s: *mut Sym = 0 as *mut Sym;
                    skip('(' as i32);
                    s = sym_find(tok);
                    if s.is_null() {
                        _tcc_warning(
                            b"implicit declaration of function \'%s\'\x00" as *const u8
                                as *const std::os::raw::c_char,
                            get_tok_str(tok, &mut tokc),
                        );
                        s = external_global_sym(tok, &mut func_old_type)
                    } else if (*s).type_0.t & 0xf as std::os::raw::c_int != 6 as std::os::raw::c_int
                    {
                        _tcc_error(
                            b"\'%s\' is not declared as function\x00" as *const u8
                                as *const std::os::raw::c_char,
                            get_tok_str(tok, &mut tokc),
                        );
                    }
                    (*ad).cleanup_func = s;
                    next();
                    skip(')' as i32);
                },
                370 | 371 => (*ad)
                    .f
                    .set_func_ctor(1 as std::os::raw::c_int as std::os::raw::c_uint),
                372 | 373 => (*ad)
                    .f
                    .set_func_dtor(1 as std::os::raw::c_int as std::os::raw::c_uint),
                374 | 375 => (*ad)
                    .f
                    .set_func_alwinl(1 as std::os::raw::c_int as std::os::raw::c_uint),
                345 | 346 => {
                    skip('(' as i32);
                    parse_mult_str(
                        &mut astr,
                        b"section name\x00" as *const u8 as *const std::os::raw::c_char,
                    );
                    (*ad).section = find_section(tcc_state, astr.data as *mut std::os::raw::c_char);
                    skip(')' as i32);
                    cstr_free(&mut astr);
                },
                353 | 354 => {
                    skip('(' as i32);
                    parse_mult_str(
                        &mut astr,
                        b"alias(\"target\")\x00" as *const u8 as *const std::os::raw::c_char,
                    );
                    (*ad).alias_target = (*tok_alloc(
                        astr.data as *mut std::os::raw::c_char,
                        astr.size - 1 as std::os::raw::c_int,
                    ))
                    .tok;
                    skip(')' as i32);
                    cstr_free(&mut astr);
                },
                388 | 389 => {
                    skip('(' as i32);
                    parse_mult_str(
                        &mut astr,
                        b"visibility(\"default|hidden|internal|protected\")\x00" as *const u8
                            as *const std::os::raw::c_char,
                    );
                    if strcmp(
                        astr.data as *const std::os::raw::c_char,
                        b"default\x00" as *const u8 as *const std::os::raw::c_char,
                    ) == 0
                    {
                        (*ad)
                            .a
                            .set_visibility(0 as std::os::raw::c_int as std::os::raw::c_ushort)
                    } else if strcmp(
                        astr.data as *const std::os::raw::c_char,
                        b"hidden\x00" as *const u8 as *const std::os::raw::c_char,
                    ) == 0
                    {
                        (*ad)
                            .a
                            .set_visibility(2 as std::os::raw::c_int as std::os::raw::c_ushort)
                    } else if strcmp(
                        astr.data as *const std::os::raw::c_char,
                        b"internal\x00" as *const u8 as *const std::os::raw::c_char,
                    ) == 0
                    {
                        (*ad)
                            .a
                            .set_visibility(1 as std::os::raw::c_int as std::os::raw::c_ushort)
                    } else if strcmp(
                        astr.data as *const std::os::raw::c_char,
                        b"protected\x00" as *const u8 as *const std::os::raw::c_char,
                    ) == 0
                    {
                        (*ad)
                            .a
                            .set_visibility(3 as std::os::raw::c_int as std::os::raw::c_ushort)
                    } else {
                        expect(
                            b"visibility(\"default|hidden|internal|protected\")\x00" as *const u8
                                as *const std::os::raw::c_char,
                        );
                    }
                    skip(')' as i32);
                    cstr_free(&mut astr);
                },
                347 | 348 => {
                    if tok == '(' as i32 {
                        next();
                        n = expr_const();
                        if n <= 0 as std::os::raw::c_int
                            || n & n - 1 as std::os::raw::c_int != 0 as std::os::raw::c_int
                        {
                            _tcc_error(
                                b"alignment must be a positive power of two\x00" as *const u8
                                    as *const std::os::raw::c_char,
                            );
                        }
                        skip(')' as i32);
                    } else {
                        n = 16 as std::os::raw::c_int
                    }
                    (*ad)
                        .a
                        .set_aligned(exact_log2p1(n) as std::os::raw::c_ushort);
                    if n != (1 as std::os::raw::c_int)
                        << (*ad).a.aligned() as std::os::raw::c_int - 1 as std::os::raw::c_int
                    {
                        _tcc_error(
                            b"alignment of %d is larger than implemented\x00" as *const u8
                                as *const std::os::raw::c_char,
                            n,
                        );
                    }
                },
                349 | 350 => (*ad)
                    .a
                    .set_packed(1 as std::os::raw::c_int as std::os::raw::c_ushort),
                351 | 352 => (*ad)
                    .a
                    .set_weak(1 as std::os::raw::c_int as std::os::raw::c_ushort),
                355 | 356 => {},
                385 | 386 => (*ad)
                    .f
                    .set_func_noreturn(1 as std::os::raw::c_int as std::os::raw::c_uint),
                357 | 358 | 359 => (*ad)
                    .f
                    .set_func_call(0 as std::os::raw::c_int as std::os::raw::c_uint),
                360 | 361 | 362 => (*ad)
                    .f
                    .set_func_call(1 as std::os::raw::c_int as std::os::raw::c_uint),
                376 => {
                    skip('(' as i32);
                    match tok {
                        378 => {
                            (*ad).attr_mode = (4 as std::os::raw::c_int + 1 as std::os::raw::c_int)
                                as std::os::raw::c_char
                        },
                        377 => {
                            (*ad).attr_mode = (1 as std::os::raw::c_int + 1 as std::os::raw::c_int)
                                as std::os::raw::c_char
                        },
                        379 => {
                            (*ad).attr_mode = (2 as std::os::raw::c_int + 1 as std::os::raw::c_int)
                                as std::os::raw::c_char
                        },
                        380 | 381 => {
                            (*ad).attr_mode = (3 as std::os::raw::c_int + 1 as std::os::raw::c_int)
                                as std::os::raw::c_char
                        },
                        _ => {
                            _tcc_warning(
                                b"__mode__(%s) not supported\n\x00" as *const u8
                                    as *const std::os::raw::c_char,
                                get_tok_str(tok, 0 as *mut CValue),
                            );
                        },
                    }
                    next();
                    skip(')' as i32);
                },
                382 => (*ad)
                    .a
                    .set_dllexport(1 as std::os::raw::c_int as std::os::raw::c_ushort),
                384 => (*ad)
                    .a
                    .set_nodecorate(1 as std::os::raw::c_int as std::os::raw::c_ushort),
                383 => (*ad)
                    .a
                    .set_dllimport(1 as std::os::raw::c_int as std::os::raw::c_ushort),
                _ => {
                    if (*tcc_state).warn_unsupported != 0 {
                        _tcc_warning(
                            b"\'%s\' attribute ignored\x00" as *const u8
                                as *const std::os::raw::c_char,
                            get_tok_str(t, 0 as *mut CValue),
                        );
                    }
                    /* skip parameters */
                    if tok == '(' as i32 {
                        let mut parenthesis: std::os::raw::c_int = 0 as std::os::raw::c_int; /* make it never match */
                        loop {
                            if tok == '(' as i32 {
                                parenthesis += 1
                            } else if tok == ')' as i32 {
                                parenthesis -= 1
                            }
                            next();
                            if !(parenthesis != 0 && tok != -(1 as std::os::raw::c_int)) {
                                break;
                            }
                        }
                    }
                },
            }
            if tok != ',' as i32 {
                break;
            }
            next();
        }
        skip(')' as i32);
        skip(')' as i32);
    }
}
unsafe extern "C" fn find_field(
    mut type_0: *mut CType,
    mut v: std::os::raw::c_int,
    mut cumofs: *mut std::os::raw::c_int,
) -> *mut Sym {
    let mut s: *mut Sym = (*type_0).ref_0;
    v |= 0x20000000 as std::os::raw::c_int;
    loop {
        s = (*s).c2rust_unnamed_0.next;
        if s.is_null() {
            break;
        }
        if (*s).v & 0x20000000 as std::os::raw::c_int != 0
            && (*s).type_0.t & 0xf as std::os::raw::c_int == 7 as std::os::raw::c_int
            && (*s).v & !(0x20000000 as std::os::raw::c_int) >= 0x10000000 as std::os::raw::c_int
        {
            let mut ret: *mut Sym = find_field(&mut (*s).type_0, v, cumofs);
            if !ret.is_null() {
                *cumofs += (*s).c2rust_unnamed.c2rust_unnamed.c;
                return ret;
            }
        }
        if (*s).v == v {
            break;
        }
    }
    return s;
}
unsafe extern "C" fn check_fields(mut type_0: *mut CType, mut check: std::os::raw::c_int) {
    let mut s: *mut Sym = (*type_0).ref_0;
    loop {
        s = (*s).c2rust_unnamed_0.next;
        if s.is_null() {
            break;
        }
        let mut v: std::os::raw::c_int = (*s).v & !(0x20000000 as std::os::raw::c_int);
        if v < 0x10000000 as std::os::raw::c_int {
            let mut ts: *mut TokenSym =
                *table_ident.offset((v - 256 as std::os::raw::c_int) as isize);
            if check != 0 && (*ts).tok & 0x20000000 as std::os::raw::c_int != 0 {
                _tcc_error(
                    b"duplicate member \'%s\'\x00" as *const u8 as *const std::os::raw::c_char,
                    get_tok_str(v, 0 as *mut CValue),
                );
            }
            (*ts).tok ^= 0x20000000 as std::os::raw::c_int
        } else if (*s).type_0.t & 0xf as std::os::raw::c_int == 7 as std::os::raw::c_int {
            check_fields(&mut (*s).type_0, check);
        }
    }
}
unsafe extern "C" fn struct_layout(mut type_0: *mut CType, mut ad: *mut AttributeDef) {
    let mut current_block: u64;
    let mut size: std::os::raw::c_int = 0;
    let mut align: std::os::raw::c_int = 0;
    let mut maxalign: std::os::raw::c_int = 0;
    let mut offset: std::os::raw::c_int = 0;
    let mut c: std::os::raw::c_int = 0;
    let mut bit_pos: std::os::raw::c_int = 0;
    let mut bit_size: std::os::raw::c_int = 0;
    let mut packed: std::os::raw::c_int = 0;
    let mut a: std::os::raw::c_int = 0;
    let mut bt: std::os::raw::c_int = 0;
    let mut prevbt: std::os::raw::c_int = 0;
    let mut prev_bit_size: std::os::raw::c_int = 0;
    let mut pcc: std::os::raw::c_int = ((*tcc_state).ms_bitfields == 0) as std::os::raw::c_int;
    let mut pragma_pack: std::os::raw::c_int = *(*tcc_state).pack_stack_ptr;
    let mut f: *mut Sym = 0 as *mut Sym;
    maxalign = 1 as std::os::raw::c_int;
    offset = 0 as std::os::raw::c_int;
    c = 0 as std::os::raw::c_int;
    bit_pos = 0 as std::os::raw::c_int;
    prevbt = 7 as std::os::raw::c_int;
    prev_bit_size = 0 as std::os::raw::c_int;
    //#define BF_DEBUG
    f = (*(*type_0).ref_0).c2rust_unnamed_0.next;
    while !f.is_null() {
        if (*f).type_0.t & 0x80 as std::os::raw::c_int != 0 {
            bit_size = (*f).type_0.t >> 20 as std::os::raw::c_int + 6 as std::os::raw::c_int
                & 0x3f as std::os::raw::c_int
        } else {
            bit_size = -(1 as std::os::raw::c_int)
        }
        size = type_size(&mut (*f).type_0, &mut align);
        a = if (*f).a.aligned() as std::os::raw::c_int != 0 {
            (1 as std::os::raw::c_int)
                << (*f).a.aligned() as std::os::raw::c_int - 1 as std::os::raw::c_int
        } else {
            0 as std::os::raw::c_int
        };
        packed = 0 as std::os::raw::c_int;
        if !(pcc != 0 && bit_size == 0 as std::os::raw::c_int) {
            /* in pcc mode, attribute packed overrides if set. */
            if pcc != 0
                && ((*f).a.packed() as std::os::raw::c_int != 0
                    || (*ad).a.packed() as std::os::raw::c_int != 0)
            {
                packed = 1 as std::os::raw::c_int;
                align = packed
            }
            /* pragma pack overrides align if lesser and packs bitfields always */
            if pragma_pack != 0 {
                packed = 1 as std::os::raw::c_int;
                if pragma_pack < align {
                    align = pragma_pack
                }
                /* in pcc mode pragma pack also overrides individual align */
                if pcc != 0 && pragma_pack < a {
                    a = 0 as std::os::raw::c_int
                }
            }
        }
        /* some individual align was specified */
        if a != 0 {
            align = a
        }
        if (*(*type_0).ref_0).type_0.t
            == (1 as std::os::raw::c_int) << 20 as std::os::raw::c_int | 7 as std::os::raw::c_int
        {
            if pcc != 0 && bit_size >= 0 as std::os::raw::c_int {
                size = bit_size + 7 as std::os::raw::c_int >> 3 as std::os::raw::c_int
            }
            offset = 0 as std::os::raw::c_int;
            if size > c {
                c = size
            }
        } else if bit_size < 0 as std::os::raw::c_int {
            if pcc != 0 {
                c += bit_pos + 7 as std::os::raw::c_int >> 3 as std::os::raw::c_int
            }
            c = c + align - 1 as std::os::raw::c_int & -align;
            offset = c;
            if size > 0 as std::os::raw::c_int {
                c += size
            }
            bit_pos = 0 as std::os::raw::c_int;
            prevbt = 7 as std::os::raw::c_int;
            prev_bit_size = 0 as std::os::raw::c_int
        } else {
            /* A bit-field.  Layout is more complicated.  There are two
            options: PCC (GCC) compatible and MS compatible */
            if pcc != 0 {
                /* In PCC layout a bit-field is placed adjacent to the
                preceding bit-fields, except if:
                - it has zero-width
                - an individual alignment was given
                - it would overflow its base type container and
                  there is no packing */
                if bit_size == 0 as std::os::raw::c_int {
                    current_block = 17288061161702580096;
                } else if (*f).a.aligned() != 0 {
                    current_block = 17288061161702580096;
                } else if packed == 0 {
                    let mut a8: std::os::raw::c_int = align * 8 as std::os::raw::c_int;
                    let mut ofs: std::os::raw::c_int =
                        ((c * 8 as std::os::raw::c_int + bit_pos) % a8 + bit_size + a8
                            - 1 as std::os::raw::c_int)
                            / a8;
                    if ofs > size / align {
                        current_block = 17288061161702580096;
                    } else {
                        current_block = 17747245473264231573;
                    }
                } else {
                    current_block = 17747245473264231573;
                }
                match current_block {
                    17288061161702580096 => {
                        c = c
                            + (bit_pos + 7 as std::os::raw::c_int >> 3 as std::os::raw::c_int)
                            + align
                            - 1 as std::os::raw::c_int
                            & -align;
                        bit_pos = 0 as std::os::raw::c_int
                    },
                    _ => {},
                }
                /* in pcc mode, long long bitfields have type int if they fit */
                if size == 8 as std::os::raw::c_int && bit_size <= 32 as std::os::raw::c_int {
                    (*f).type_0.t =
                        (*f).type_0.t & !(0xf as std::os::raw::c_int) | 3 as std::os::raw::c_int;
                    size = 4 as std::os::raw::c_int
                }
                while bit_pos >= align * 8 as std::os::raw::c_int {
                    c += align;
                    bit_pos -= align * 8 as std::os::raw::c_int
                }
                offset = c;
                /* In PCC layout named bit-fields influence the alignment
                of the containing struct using the base types alignment,
                except for packed fields (which here have correct align).  */
                if (*f).v & 0x10000000 as std::os::raw::c_int != 0 {
                    // && bit_size // ??? gcc on ARM/rpi does that
                    align = 1 as std::os::raw::c_int
                }
            } else {
                bt = (*f).type_0.t & 0xf as std::os::raw::c_int;
                if bit_pos + bit_size > size * 8 as std::os::raw::c_int
                    || (bit_size > 0 as std::os::raw::c_int) as std::os::raw::c_int
                        == (bt != prevbt) as std::os::raw::c_int
                {
                    c = c + align - 1 as std::os::raw::c_int & -align;
                    offset = c;
                    bit_pos = 0 as std::os::raw::c_int;
                    /* In MS bitfield mode a bit-field run always uses
                    at least as many bits as the underlying type.
                    To start a new run it's also required that this
                    or the last bit-field had non-zero width.  */
                    if bit_size != 0 || prev_bit_size != 0 {
                        c += size
                    }
                }
                /* In MS layout the records alignment is normally
                influenced by the field, except for a zero-width
                field at the start of a run (but by further zero-width
                fields it is again).  */
                if bit_size == 0 as std::os::raw::c_int && prevbt != bt {
                    align = 1 as std::os::raw::c_int
                }
                prevbt = bt;
                prev_bit_size = bit_size
            }
            (*f).type_0.t = (*f).type_0.t
                & !((0x3f as std::os::raw::c_int) << 20 as std::os::raw::c_int)
                | bit_pos << 20 as std::os::raw::c_int;
            bit_pos += bit_size
        }
        if align > maxalign {
            maxalign = align
        }
        (*f).c2rust_unnamed.c2rust_unnamed.c = offset;
        (*f).r = 0 as std::os::raw::c_int as std::os::raw::c_ushort;
        f = (*f).c2rust_unnamed_0.next
    }
    if pcc != 0 {
        c += bit_pos + 7 as std::os::raw::c_int >> 3 as std::os::raw::c_int
    }
    /* store size and alignment */
    bt = if (*ad).a.aligned() as std::os::raw::c_int != 0 {
        (1 as std::os::raw::c_int)
            << (*ad).a.aligned() as std::os::raw::c_int - 1 as std::os::raw::c_int
    } else {
        1 as std::os::raw::c_int
    };
    a = bt;
    if a < maxalign {
        a = maxalign
    }
    (*(*type_0).ref_0).r = a as std::os::raw::c_ushort;
    if pragma_pack != 0 && pragma_pack < maxalign && 0 as std::os::raw::c_int == pcc {
        /* can happen if individual align for some member was given.  In
        this case MSVC ignores maxalign when aligning the size */
        a = pragma_pack;
        if a < bt {
            a = bt
        }
    }
    c = c + a - 1 as std::os::raw::c_int & -a;
    (*(*type_0).ref_0).c2rust_unnamed.c2rust_unnamed.c = c;
    /* check whether we can access bitfields by their type */
    f = (*(*type_0).ref_0).c2rust_unnamed_0.next;
    while !f.is_null() {
        let mut s: std::os::raw::c_int = 0;
        let mut px: std::os::raw::c_int = 0;
        let mut cx: std::os::raw::c_int = 0;
        let mut c0: std::os::raw::c_int = 0;
        let mut t: CType = CType {
            t: 0,
            ref_0: 0 as *const Sym as *mut Sym,
        };
        if !(0 as std::os::raw::c_int == (*f).type_0.t & 0x80 as std::os::raw::c_int) {
            (*f).type_0.ref_0 = f;
            (*f).c2rust_unnamed.c2rust_unnamed.c2rust_unnamed.auxtype = -(1 as std::os::raw::c_int);
            bit_size = (*f).type_0.t >> 20 as std::os::raw::c_int + 6 as std::os::raw::c_int
                & 0x3f as std::os::raw::c_int;
            if !(bit_size == 0 as std::os::raw::c_int) {
                bit_pos = (*f).type_0.t >> 20 as std::os::raw::c_int & 0x3f as std::os::raw::c_int;
                size = type_size(&mut (*f).type_0, &mut align);
                if !(bit_pos + bit_size <= size * 8 as std::os::raw::c_int
                    && (*f).c2rust_unnamed.c2rust_unnamed.c + size <= c)
                {
                    /* try to access the field using a different type */
                    c0 = -(1 as std::os::raw::c_int);
                    align = 1 as std::os::raw::c_int;
                    s = align;
                    t.t = 1 as std::os::raw::c_int;
                    loop {
                        px = (*f).c2rust_unnamed.c2rust_unnamed.c * 8 as std::os::raw::c_int
                            + bit_pos;
                        cx = px >> 3 as std::os::raw::c_int & -align;
                        px = px - (cx << 3 as std::os::raw::c_int);
                        if c0 == cx {
                            break;
                        }
                        s = px + bit_size + 7 as std::os::raw::c_int >> 3 as std::os::raw::c_int;
                        if s > 4 as std::os::raw::c_int {
                            t.t = 4 as std::os::raw::c_int
                        } else if s > 2 as std::os::raw::c_int {
                            t.t = 3 as std::os::raw::c_int
                        } else if s > 1 as std::os::raw::c_int {
                            t.t = 2 as std::os::raw::c_int
                        } else {
                            t.t = 1 as std::os::raw::c_int
                        }
                        s = type_size(&mut t, &mut align);
                        c0 = cx
                    }
                    if px + bit_size <= s * 8 as std::os::raw::c_int && cx + s <= c {
                        /* update offset and bit position */
                        (*f).c2rust_unnamed.c2rust_unnamed.c = cx;
                        bit_pos = px;
                        (*f).type_0.t = (*f).type_0.t
                            & !((0x3f as std::os::raw::c_int) << 20 as std::os::raw::c_int)
                            | bit_pos << 20 as std::os::raw::c_int;
                        if s != size {
                            (*f).c2rust_unnamed.c2rust_unnamed.c2rust_unnamed.auxtype = t.t
                        }
                    } else {
                        /* fall back to load/store single-byte wise */
                        (*f).c2rust_unnamed.c2rust_unnamed.c2rust_unnamed.auxtype =
                            7 as std::os::raw::c_int
                    }
                }
            }
        }
        f = (*f).c2rust_unnamed_0.next
    }
}
/* enum/struct/union declaration. u is VT_ENUM/VT_STRUCT/VT_UNION */
unsafe extern "C" fn struct_decl(mut type_0: *mut CType, mut u: std::os::raw::c_int) {
    let mut current_block: u64;
    let mut v: std::os::raw::c_int = 0;
    let mut c: std::os::raw::c_int = 0;
    let mut size: std::os::raw::c_int = 0;
    let mut align: std::os::raw::c_int = 0;
    let mut flexible: std::os::raw::c_int = 0;
    let mut bit_size: std::os::raw::c_int = 0;
    let mut bsize: std::os::raw::c_int = 0;
    let mut bt: std::os::raw::c_int = 0;
    let mut s: *mut Sym = 0 as *mut Sym;
    let mut ss: *mut Sym = 0 as *mut Sym;
    let mut ps: *mut *mut Sym = 0 as *mut *mut Sym;
    let mut ad: AttributeDef = AttributeDef {
        a: SymAttr([0; 2]),
        f: FuncAttr([0; 4]),
        section: 0 as *mut Section,
        cleanup_func: 0 as *mut Sym,
        alias_target: 0,
        asm_label: 0,
        attr_mode: 0,
    };
    let mut ad1: AttributeDef = AttributeDef {
        a: SymAttr([0; 2]),
        f: FuncAttr([0; 4]),
        section: 0 as *mut Section,
        cleanup_func: 0 as *mut Sym,
        alias_target: 0,
        asm_label: 0,
        attr_mode: 0,
    };
    let mut type1: CType = CType {
        t: 0,
        ref_0: 0 as *const Sym as *mut Sym,
    };
    let mut btype: CType = CType {
        t: 0,
        ref_0: 0 as *const Sym as *mut Sym,
    };
    memset(
        &mut ad as *mut AttributeDef as *mut std::os::raw::c_void,
        0 as std::os::raw::c_int,
        ::std::mem::size_of::<AttributeDef>() as std::os::raw::c_ulong,
    );
    next();
    parse_attribute(&mut ad);
    if tok != '{' as i32 {
        v = tok;
        next();
        /* struct already defined ? return it */
        if v < 256 as std::os::raw::c_int {
            expect(b"struct/union/enum name\x00" as *const u8 as *const std::os::raw::c_char);
        }
        s = struct_find(v);
        if !s.is_null()
            && ((*s).c2rust_unnamed.c2rust_unnamed.c2rust_unnamed.sym_scope == local_scope
                || tok != '{' as i32)
        {
            if u == (*s).type_0.t {
                current_block = 5013019647201978700;
            } else if u == (2 as std::os::raw::c_int) << 20 as std::os::raw::c_int
                && (*s).type_0.t as std::os::raw::c_uint
                    & (((1 as std::os::raw::c_uint)
                        << 6 as std::os::raw::c_int + 6 as std::os::raw::c_int)
                        .wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_uint)
                        << 20 as std::os::raw::c_int
                        | 0x80 as std::os::raw::c_int as std::os::raw::c_uint)
                    == ((2 as std::os::raw::c_int) << 20 as std::os::raw::c_int)
                        as std::os::raw::c_uint
            {
                current_block = 5013019647201978700;
            } else {
                _tcc_error(
                    b"redefinition of \'%s\'\x00" as *const u8 as *const std::os::raw::c_char,
                    get_tok_str(v, 0 as *mut CValue),
                );
            }
        } else {
            current_block = 15976848397966268834;
        }
    } else {
        let fresh5 = anon_sym;
        anon_sym = anon_sym + 1;
        v = fresh5;
        current_block = 15976848397966268834;
    }
    match current_block {
        15976848397966268834 => {
            /* Record the original enum/struct/union token.  */
            type1.t = if u == (2 as std::os::raw::c_int) << 20 as std::os::raw::c_int {
                (u | 3 as std::os::raw::c_int) | 0x10 as std::os::raw::c_int
            } else {
                u
            };
            type1.ref_0 = 0 as *mut Sym;
            /* we put an undefined size for struct/union */
            s = sym_push(
                v | 0x40000000 as std::os::raw::c_int,
                &mut type1,
                0 as std::os::raw::c_int,
                -(1 as std::os::raw::c_int),
            ); /* default alignment is zero as gcc */
            (*s).r = 0 as std::os::raw::c_int as std::os::raw::c_ushort
        },
        _ => {},
    }
    (*type_0).t = (*s).type_0.t;
    (*type_0).ref_0 = s;
    if tok == '{' as i32 {
        next();
        if (*s).c2rust_unnamed.c2rust_unnamed.c != -(1 as std::os::raw::c_int) {
            _tcc_error(
                b"struct/union/enum already defined\x00" as *const u8
                    as *const std::os::raw::c_char,
            );
        }
        (*s).c2rust_unnamed.c2rust_unnamed.c = -(2 as std::os::raw::c_int);
        /* cannot be empty */
        /* non empty enums are not allowed */
        ps = &mut (*s).c2rust_unnamed_0.next;
        if u == (2 as std::os::raw::c_int) << 20 as std::os::raw::c_int {
            let mut ll: std::os::raw::c_longlong =
                0 as std::os::raw::c_int as std::os::raw::c_longlong;
            let mut pl: std::os::raw::c_longlong =
                0 as std::os::raw::c_int as std::os::raw::c_longlong;
            let mut nl: std::os::raw::c_longlong =
                0 as std::os::raw::c_int as std::os::raw::c_longlong;
            let mut t: CType = CType {
                t: 0,
                ref_0: 0 as *const Sym as *mut Sym,
            };
            t.ref_0 = s;
            /* enum symbols have static storage */
            t.t = 3 as std::os::raw::c_int
                | 0x2000 as std::os::raw::c_int
                | (3 as std::os::raw::c_int) << 20 as std::os::raw::c_int;
            loop {
                v = tok;
                if v < TOK_DEFINE as std::os::raw::c_int {
                    expect(b"identifier\x00" as *const u8 as *const std::os::raw::c_char);
                }
                ss = sym_find(v);
                if !ss.is_null() && local_stack.is_null() {
                    _tcc_error(
                        b"redefinition of enumerator \'%s\'\x00" as *const u8
                            as *const std::os::raw::c_char,
                        get_tok_str(v, 0 as *mut CValue),
                    );
                }
                next();
                if tok == '=' as i32 {
                    next();
                    ll = expr_const64() as std::os::raw::c_longlong
                }
                ss = sym_push(
                    v,
                    &mut t,
                    0x30 as std::os::raw::c_int,
                    0 as std::os::raw::c_int,
                );
                (*ss).c2rust_unnamed.enum_val = ll;
                *ps = ss;
                ps = &mut (*ss).c2rust_unnamed_0.next;
                if ll < nl {
                    nl = ll
                }
                if ll > pl {
                    pl = ll
                }
                if tok != ',' as i32 {
                    break;
                }
                next();
                ll += 1;
                /* NOTE: we accept a trailing comma */
                if tok == '}' as i32 {
                    break;
                }
            }
            skip('}' as i32);
            /* set integral type of the enum */
            t.t = 3 as std::os::raw::c_int;
            if nl >= 0 as std::os::raw::c_int as std::os::raw::c_longlong {
                if pl != pl as std::os::raw::c_uint as std::os::raw::c_longlong {
                    t.t = if 8 as std::os::raw::c_int == 8 as std::os::raw::c_int {
                        (4 as std::os::raw::c_int) | 0x800 as std::os::raw::c_int
                    } else {
                        4 as std::os::raw::c_int
                    }
                }
                t.t |= 0x10 as std::os::raw::c_int
            } else if pl != pl as std::os::raw::c_int as std::os::raw::c_longlong
                || nl != nl as std::os::raw::c_int as std::os::raw::c_longlong
            {
                t.t = if 8 as std::os::raw::c_int == 8 as std::os::raw::c_int {
                    (4 as std::os::raw::c_int) | 0x800 as std::os::raw::c_int
                } else {
                    4 as std::os::raw::c_int
                }
            }
            (*type_0).t = t.t | (2 as std::os::raw::c_int) << 20 as std::os::raw::c_int;
            (*s).type_0.t = (*type_0).t;
            (*s).c2rust_unnamed.c2rust_unnamed.c = 0 as std::os::raw::c_int;
            let mut current_block_59: u64;
            /* set type for enum members */
            ss = (*s).c2rust_unnamed_0.next;
            while !ss.is_null() {
                ll = (*ss).c2rust_unnamed.enum_val;
                if !(ll == ll as std::os::raw::c_int as std::os::raw::c_longlong) {
                    if t.t & 0x10 as std::os::raw::c_int != 0 {
                        (*ss).type_0.t |= 0x10 as std::os::raw::c_int;
                        if ll == ll as std::os::raw::c_uint as std::os::raw::c_longlong {
                            current_block_59 = 15594603006322722090;
                        } else {
                            current_block_59 = 2606304779496145856;
                        }
                    } else {
                        current_block_59 = 2606304779496145856;
                    }
                    match current_block_59 {
                        15594603006322722090 => {},
                        _ => {
                            (*ss).type_0.t = (*ss).type_0.t & !(0xf as std::os::raw::c_int)
                                | (if 8 as std::os::raw::c_int == 8 as std::os::raw::c_int {
                                    (4 as std::os::raw::c_int) | 0x800 as std::os::raw::c_int
                                } else {
                                    4 as std::os::raw::c_int
                                })
                        },
                    }
                }
                /* default is int if it fits */
                ss = (*ss).c2rust_unnamed_0.next
            }
        } else {
            c = 0 as std::os::raw::c_int;
            flexible = 0 as std::os::raw::c_int;
            while tok != '}' as i32 {
                if parse_btype(&mut btype, &mut ad1) == 0 {
                    skip(';' as i32);
                } else {
                    loop {
                        if flexible != 0 {
                            _tcc_error(
                                b"flexible array member \'%s\' not at the end of struct\x00"
                                    as *const u8
                                    as *const std::os::raw::c_char,
                                get_tok_str(v, 0 as *mut CValue),
                            );
                        }
                        bit_size = -(1 as std::os::raw::c_int);
                        v = 0 as std::os::raw::c_int;
                        type1 = btype;
                        if tok != ':' as i32 {
                            if tok != ';' as i32 {
                                type_decl(&mut type1, &mut ad1, &mut v, 2 as std::os::raw::c_int);
                            }
                            if v == 0 as std::os::raw::c_int {
                                if type1.t & 0xf as std::os::raw::c_int != 7 as std::os::raw::c_int
                                {
                                    expect(
                                        b"identifier\x00" as *const u8
                                            as *const std::os::raw::c_char,
                                    );
                                } else {
                                    let mut v_0: std::os::raw::c_int = (*btype.ref_0).v;
                                    if v_0 & 0x20000000 as std::os::raw::c_int == 0
                                        && (v_0 & !(0x40000000 as std::os::raw::c_int))
                                            < 0x10000000 as std::os::raw::c_int
                                    {
                                        if (*tcc_state).ms_extensions as std::os::raw::c_int
                                            == 0 as std::os::raw::c_int
                                        {
                                            expect(
                                                b"identifier\x00" as *const u8
                                                    as *const std::os::raw::c_char,
                                            );
                                        }
                                    }
                                }
                            }
                            if type_size(&mut type1, &mut align) < 0 as std::os::raw::c_int {
                                if u == 7 as std::os::raw::c_int
                                    && type1.t & 0x40 as std::os::raw::c_int != 0
                                    && c != 0
                                {
                                    flexible = 1 as std::os::raw::c_int
                                } else {
                                    _tcc_error(
                                        b"field \'%s\' has incomplete type\x00" as *const u8
                                            as *const std::os::raw::c_char,
                                        get_tok_str(v, 0 as *mut CValue),
                                    );
                                }
                            }
                            if type1.t & 0xf as std::os::raw::c_int == 6 as std::os::raw::c_int
                                || type1.t & 0xf as std::os::raw::c_int == 0 as std::os::raw::c_int
                                || type1.t
                                    & (0x1000 as std::os::raw::c_int
                                        | 0x2000 as std::os::raw::c_int
                                        | 0x4000 as std::os::raw::c_int
                                        | 0x8000 as std::os::raw::c_int)
                                    != 0
                            {
                                _tcc_error(
                                    b"invalid type for \'%s\'\x00" as *const u8
                                        as *const std::os::raw::c_char,
                                    get_tok_str(v, 0 as *mut CValue),
                                );
                            }
                        }
                        if tok == ':' as i32 {
                            next();
                            bit_size = expr_const();
                            /* XXX: handle v = 0 case for messages */
                            if bit_size < 0 as std::os::raw::c_int {
                                _tcc_error(
                                    b"negative width in bit-field \'%s\'\x00" as *const u8
                                        as *const std::os::raw::c_char,
                                    get_tok_str(v, 0 as *mut CValue),
                                );
                            }
                            if v != 0 && bit_size == 0 as std::os::raw::c_int {
                                _tcc_error(
                                    b"zero width for bit-field \'%s\'\x00" as *const u8
                                        as *const std::os::raw::c_char,
                                    get_tok_str(v, 0 as *mut CValue),
                                );
                            }
                            parse_attribute(&mut ad1);
                        }
                        size = type_size(&mut type1, &mut align);
                        if bit_size >= 0 as std::os::raw::c_int {
                            bt = type1.t & 0xf as std::os::raw::c_int;
                            if bt != 3 as std::os::raw::c_int
                                && bt != 1 as std::os::raw::c_int
                                && bt != 2 as std::os::raw::c_int
                                && bt != 11 as std::os::raw::c_int
                                && bt != 4 as std::os::raw::c_int
                            {
                                _tcc_error(
                                    b"bitfields must have scalar type\x00" as *const u8
                                        as *const std::os::raw::c_char,
                                );
                            }
                            bsize = size * 8 as std::os::raw::c_int;
                            if bit_size > bsize {
                                _tcc_error(
                                    b"width of \'%s\' exceeds its type\x00" as *const u8
                                        as *const std::os::raw::c_char,
                                    get_tok_str(v, 0 as *mut CValue),
                                );
                            } else {
                                if !(bit_size == bsize && ad.a.packed() == 0 && ad1.a.packed() == 0)
                                {
                                    if bit_size == 64 as std::os::raw::c_int {
                                        _tcc_error(
                                            b"field width 64 not implemented\x00" as *const u8
                                                as *const std::os::raw::c_char,
                                        );
                                    } else {
                                        type1.t = (type1.t as std::os::raw::c_uint
                                            & !(((1 as std::os::raw::c_uint)
                                                << 6 as std::os::raw::c_int
                                                    + 6 as std::os::raw::c_int)
                                                .wrapping_sub(
                                                    1 as std::os::raw::c_int
                                                        as std::os::raw::c_uint,
                                                )
                                                << 20 as std::os::raw::c_int
                                                | 0x80 as std::os::raw::c_int
                                                    as std::os::raw::c_uint)
                                            | 0x80 as std::os::raw::c_int as std::os::raw::c_uint
                                            | (bit_size
                                                << 20 as std::os::raw::c_int
                                                    + 6 as std::os::raw::c_int)
                                                as std::os::raw::c_uint)
                                            as std::os::raw::c_int
                                    }
                                }
                            }
                        }
                        if v != 0 as std::os::raw::c_int
                            || type1.t & 0xf as std::os::raw::c_int == 7 as std::os::raw::c_int
                        {
                            /* Remember we've seen a real field to check
                            for placement of flexible array member. */
                            c = 1 as std::os::raw::c_int
                        }
                        /* If member is a struct or bit-field, enforce
                        placing into the struct (as anonymous).  */
                        if v == 0 as std::os::raw::c_int
                            && (type1.t & 0xf as std::os::raw::c_int == 7 as std::os::raw::c_int
                                || bit_size >= 0 as std::os::raw::c_int)
                        {
                            let fresh6 = anon_sym;
                            anon_sym = anon_sym + 1;
                            v = fresh6
                        }
                        if v != 0 {
                            ss = sym_push(
                                v | 0x20000000 as std::os::raw::c_int,
                                &mut type1,
                                0 as std::os::raw::c_int,
                                0 as std::os::raw::c_int,
                            );
                            (*ss).a = ad1.a;
                            *ps = ss;
                            ps = &mut (*ss).c2rust_unnamed_0.next
                        }
                        if tok == ';' as i32 || tok == -(1 as std::os::raw::c_int) {
                            break;
                        }
                        skip(',' as i32);
                    }
                    skip(';' as i32);
                }
            }
            skip('}' as i32);
            parse_attribute(&mut ad);
            if !ad.cleanup_func.is_null() {
                _tcc_warning(
                    b"attribute \'__cleanup__\' ignored on type\x00" as *const u8
                        as *const std::os::raw::c_char,
                );
            }
            check_fields(type_0, 1 as std::os::raw::c_int);
            check_fields(type_0, 0 as std::os::raw::c_int);
            struct_layout(type_0, &mut ad);
        }
    };
}
unsafe extern "C" fn sym_to_attr(mut ad: *mut AttributeDef, mut s: *mut Sym) {
    merge_symattr(&mut (*ad).a, &mut (*s).a);
    merge_funcattr(
        &mut (*ad).f,
        &mut (*s).c2rust_unnamed.c2rust_unnamed.c2rust_unnamed.f,
    );
}
/* Add type qualifiers to a type. If the type is an array then the qualifiers
are added to the element type, copied because it could be a typedef. */
unsafe extern "C" fn parse_btype_qualify(
    mut type_0: *mut CType,
    mut qualifiers: std::os::raw::c_int,
) {
    while (*type_0).t & 0x40 as std::os::raw::c_int != 0 {
        (*type_0).ref_0 = sym_push(
            0x20000000 as std::os::raw::c_int,
            &mut (*(*type_0).ref_0).type_0,
            0 as std::os::raw::c_int,
            (*(*type_0).ref_0).c2rust_unnamed.c2rust_unnamed.c,
        );
        type_0 = &mut (*(*type_0).ref_0).type_0
    }
    (*type_0).t |= qualifiers;
}
/* return 0 if no type declaration. otherwise, return the basic type
  and skip it.
*/
unsafe extern "C" fn parse_btype(
    mut type_0: *mut CType,
    mut ad: *mut AttributeDef,
) -> std::os::raw::c_int {
    let mut current_block: u64;
    let mut t: std::os::raw::c_int = 0;
    let mut u: std::os::raw::c_int = 0;
    let mut bt: std::os::raw::c_int = 0;
    let mut st: std::os::raw::c_int = 0;
    let mut type_found: std::os::raw::c_int = 0;
    let mut typespec_found: std::os::raw::c_int = 0;
    let mut g: std::os::raw::c_int = 0;
    let mut n: std::os::raw::c_int = 0;
    let mut s: *mut Sym = 0 as *mut Sym;
    let mut type1: CType = CType {
        t: 0,
        ref_0: 0 as *const Sym as *mut Sym,
    };
    memset(
        ad as *mut std::os::raw::c_void,
        0 as std::os::raw::c_int,
        ::std::mem::size_of::<AttributeDef>() as std::os::raw::c_ulong,
    );
    type_found = 0 as std::os::raw::c_int;
    typespec_found = 0 as std::os::raw::c_int;
    t = 3 as std::os::raw::c_int;
    st = -(1 as std::os::raw::c_int);
    bt = st;
    (*type_0).ref_0 = 0 as *mut Sym;
    loop {
        match tok {
            292 => {
                /* currently, we really ignore extension */
                next();
                continue;
            },
            258 => {
                /* basic types */
                u = 1 as std::os::raw::c_int;
                current_block = 2886556943497725703;
            },
            257 => {
                u = 0 as std::os::raw::c_int;
                current_block = 2886556943497725703;
            },
            298 => {
                u = 2 as std::os::raw::c_int;
                current_block = 2886556943497725703;
            },
            256 => {
                u = 3 as std::os::raw::c_int;
                current_block = 2886556943497725703;
            },
            310 => {
                let mut n_0: std::os::raw::c_int = 0;
                let mut ad1: AttributeDef = AttributeDef {
                    a: SymAttr([0; 2]),
                    f: FuncAttr([0; 4]),
                    section: 0 as *mut Section,
                    cleanup_func: 0 as *mut Sym,
                    alias_target: 0,
                    asm_label: 0,
                    attr_mode: 0,
                };
                next();
                skip('(' as i32);
                memset(
                    &mut ad1 as *mut AttributeDef as *mut std::os::raw::c_void,
                    0 as std::os::raw::c_int,
                    ::std::mem::size_of::<AttributeDef>() as std::os::raw::c_ulong,
                );
                if parse_btype(&mut type1, &mut ad1) != 0 {
                    type_decl(&mut type1, &mut ad1, &mut n_0, 1 as std::os::raw::c_int);
                    if ad1.a.aligned() != 0 {
                        n_0 = (1 as std::os::raw::c_int)
                            << ad1.a.aligned() as std::os::raw::c_int - 1 as std::os::raw::c_int
                    } else {
                        type_size(&mut type1, &mut n_0);
                    }
                } else {
                    n_0 = expr_const();
                    if n_0 <= 0 as std::os::raw::c_int
                        || n_0 & n_0 - 1 as std::os::raw::c_int != 0 as std::os::raw::c_int
                    {
                        _tcc_error(
                            b"alignment must be a positive power of two\x00" as *const u8
                                as *const std::os::raw::c_char,
                        );
                    }
                }
                skip(')' as i32);
                (*ad)
                    .a
                    .set_aligned(exact_log2p1(n_0) as std::os::raw::c_ushort);
                continue;
            },
            280 => {
                if t & 0xf as std::os::raw::c_int == 9 as std::os::raw::c_int {
                    t = t & !(0xf as std::os::raw::c_int | 0x800 as std::os::raw::c_int)
                        | 10 as std::os::raw::c_int;
                    current_block = 5891011138178424807;
                } else if t & (0xf as std::os::raw::c_int | 0x800 as std::os::raw::c_int)
                    == 0x800 as std::os::raw::c_int
                {
                    t = t & !(0xf as std::os::raw::c_int | 0x800 as std::os::raw::c_int)
                        | 4 as std::os::raw::c_int;
                    current_block = 5891011138178424807;
                } else {
                    u = 0x800 as std::os::raw::c_int;
                    current_block = 2886556943497725703;
                }
                match current_block {
                    2886556943497725703 => {},
                    _ => {
                        next();
                        current_block = 7545150590528655645;
                    },
                }
            },
            297 => {
                u = 11 as std::os::raw::c_int;
                current_block = 2886556943497725703;
            },
            295 => {
                u = 8 as std::os::raw::c_int;
                current_block = 2886556943497725703;
            },
            296 => {
                if t & (0xf as std::os::raw::c_int | 0x800 as std::os::raw::c_int)
                    == 0x800 as std::os::raw::c_int
                {
                    t = t & !(0xf as std::os::raw::c_int | 0x800 as std::os::raw::c_int)
                        | 10 as std::os::raw::c_int;
                    next();
                    current_block = 7545150590528655645;
                } else {
                    u = 9 as std::os::raw::c_int;
                    current_block = 2886556943497725703;
                }
            },
            303 => {
                struct_decl(
                    &mut type1,
                    (2 as std::os::raw::c_int) << 20 as std::os::raw::c_int,
                );
                current_block = 16878506906597527752;
            },
            299 => {
                struct_decl(&mut type1, 7 as std::os::raw::c_int);
                current_block = 16878506906597527752;
            },
            300 => {
                struct_decl(
                    &mut type1,
                    (1 as std::os::raw::c_int) << 20 as std::os::raw::c_int
                        | 7 as std::os::raw::c_int,
                );
                current_block = 16878506906597527752;
            },
            273 => {
                /* type modifiers */
                next();
                (*type_0).t = t;
                parse_btype_qualify(type_0, 0x200 as std::os::raw::c_int);
                t = (*type_0).t;
                if tok == '(' as i32 {
                    parse_expr_type(&mut type1);
                    /* remove all storage modifiers except typedef */
                    type1.t &= !((0x1000 as std::os::raw::c_int
                        | 0x2000 as std::os::raw::c_int
                        | 0x4000 as std::os::raw::c_int
                        | 0x8000 as std::os::raw::c_int)
                        & !(0x4000 as std::os::raw::c_int));
                    if !type1.ref_0.is_null() {
                        sym_to_attr(ad, type1.ref_0);
                    }
                    current_block = 16878506906597527752;
                } else {
                    current_block = 7545150590528655645;
                }
            },
            274 | 275 | 276 => {
                (*type_0).t = t;
                parse_btype_qualify(type_0, 0x100 as std::os::raw::c_int);
                t = (*type_0).t;
                next();
                current_block = 7545150590528655645;
            },
            277 | 278 | 279 => {
                (*type_0).t = t;
                parse_btype_qualify(type_0, 0x200 as std::os::raw::c_int);
                t = (*type_0).t;
                next();
                current_block = 7545150590528655645;
            },
            282 | 283 | 284 => {
                if t & (0x20 as std::os::raw::c_int | 0x10 as std::os::raw::c_int)
                    == 0x20 as std::os::raw::c_int | 0x10 as std::os::raw::c_int
                {
                    _tcc_error(
                        b"signed and unsigned modifier\x00" as *const u8
                            as *const std::os::raw::c_char,
                    );
                }
                t |= 0x20 as std::os::raw::c_int;
                next();
                typespec_found = 1 as std::os::raw::c_int;
                current_block = 7545150590528655645;
            },
            281 | 285 | 289 | 290 | 291 => {
                next();
                current_block = 7545150590528655645;
            },
            267 => {
                if t & (0x20 as std::os::raw::c_int | 0x10 as std::os::raw::c_int)
                    == 0x20 as std::os::raw::c_int
                {
                    _tcc_error(
                        b"signed and unsigned modifier\x00" as *const u8
                            as *const std::os::raw::c_char,
                    );
                }
                t |= 0x20 as std::os::raw::c_int | 0x10 as std::os::raw::c_int;
                next();
                typespec_found = 1 as std::os::raw::c_int;
                current_block = 7545150590528655645;
            },
            265 => {
                /* storage */
                g = 0x1000 as std::os::raw::c_int;
                current_block = 15931183752769671364;
            },
            266 => {
                g = 0x2000 as std::os::raw::c_int;
                current_block = 15931183752769671364;
            },
            301 => {
                g = 0x4000 as std::os::raw::c_int;
                current_block = 15931183752769671364;
            },
            286 | 287 | 288 => {
                t |= 0x8000 as std::os::raw::c_int;
                next();
                current_block = 7545150590528655645;
            },
            387 => {
                next();
                (*ad)
                    .f
                    .set_func_noreturn(1 as std::os::raw::c_int as std::os::raw::c_uint);
                current_block = 7545150590528655645;
            },
            305 | 306 => {
                /* GNUC attribute */
                parse_attribute(ad);
                if (*ad).attr_mode != 0 {
                    u = (*ad).attr_mode as std::os::raw::c_int - 1 as std::os::raw::c_int;
                    t = t & !(0xf as std::os::raw::c_int | 0x800 as std::os::raw::c_int) | u
                }
                continue;
            },
            311 | 312 | 313 => {
                /* GNUC typeof */
                next();
                parse_expr_type(&mut type1);
                /* remove all storage modifiers except typedef */
                type1.t &= !((0x1000 as std::os::raw::c_int
                    | 0x2000 as std::os::raw::c_int
                    | 0x4000 as std::os::raw::c_int
                    | 0x8000 as std::os::raw::c_int)
                    & !(0x4000 as std::os::raw::c_int));
                if !type1.ref_0.is_null() {
                    sym_to_attr(ad, type1.ref_0);
                }
                current_block = 16878506906597527752;
            },
            _ => {
                if typespec_found != 0 {
                    break;
                }
                s = sym_find(tok);
                if s.is_null() || (*s).type_0.t & 0x4000 as std::os::raw::c_int == 0 {
                    break;
                }
                n = tok;
                next();
                if tok == ':' as i32 && in_generic == 0 {
                    /* ignore if it's a label */
                    unget_tok(n);
                    break;
                } else {
                    t &= !(0xf as std::os::raw::c_int | 0x800 as std::os::raw::c_int);
                    u = t & !(0x100 as std::os::raw::c_int | 0x200 as std::os::raw::c_int);
                    t ^= u;
                    (*type_0).t = (*s).type_0.t & !(0x4000 as std::os::raw::c_int) | u;
                    (*type_0).ref_0 = (*s).type_0.ref_0;
                    if t != 0 {
                        parse_btype_qualify(type_0, t);
                    }
                    t = (*type_0).t;
                    /* get attributes from typedef */
                    sym_to_attr(ad, s);
                    typespec_found = 1 as std::os::raw::c_int;
                    bt = -(2 as std::os::raw::c_int);
                    st = bt
                }
                current_block = 7545150590528655645;
            },
        }
        match current_block {
            15931183752769671364 => {
                if t & (0x1000 as std::os::raw::c_int
                    | 0x2000 as std::os::raw::c_int
                    | 0x4000 as std::os::raw::c_int)
                    & !g
                    != 0
                {
                    _tcc_error(
                        b"multiple storage classes\x00" as *const u8 as *const std::os::raw::c_char,
                    );
                }
                t |= g;
                next();
                current_block = 7545150590528655645;
            },
            16878506906597527752 => {
                u = type1.t;
                (*type_0).ref_0 = type1.ref_0;
                current_block = 18363084699601200251;
            },
            2886556943497725703 => {
                next();
                current_block = 18363084699601200251;
            },
            _ => {},
        }
        match current_block {
            18363084699601200251 => {
                let mut current_block_11: u64;
                if u == 2 as std::os::raw::c_int || u == 0x800 as std::os::raw::c_int {
                    if st != -(1 as std::os::raw::c_int)
                        || bt != -(1 as std::os::raw::c_int) && bt != 3 as std::os::raw::c_int
                    {
                        current_block_11 = 3541958216283374978;
                    } else {
                        st = u;
                        current_block_11 = 6057473163062296781;
                    }
                } else if bt != -(1 as std::os::raw::c_int)
                    || st != -(1 as std::os::raw::c_int) && u != 3 as std::os::raw::c_int
                {
                    current_block_11 = 3541958216283374978;
                } else {
                    bt = u;
                    current_block_11 = 6057473163062296781;
                }
                match current_block_11 {
                    6057473163062296781 => {},
                    _ => {
                        _tcc_error(
                            b"too many basic types\x00" as *const u8 as *const std::os::raw::c_char,
                        );
                    },
                }
                if u != 3 as std::os::raw::c_int {
                    t = t & !(0xf as std::os::raw::c_int | 0x800 as std::os::raw::c_int) | u
                }
                typespec_found = 1 as std::os::raw::c_int
            },
            _ => {},
        }
        type_found = 1 as std::os::raw::c_int
    }
    if (*tcc_state).char_is_unsigned != 0 {
        if t & (0x20 as std::os::raw::c_int | 0xf as std::os::raw::c_int)
            == 1 as std::os::raw::c_int
        {
            t |= 0x10 as std::os::raw::c_int
        }
    }
    /* VT_LONG is used just as a modifier for VT_INT / VT_LLONG */
    bt = t & (0xf as std::os::raw::c_int | 0x800 as std::os::raw::c_int);
    if bt == 0x800 as std::os::raw::c_int {
        t |= if 8 as std::os::raw::c_int == 8 as std::os::raw::c_int {
            4 as std::os::raw::c_int
        } else {
            3 as std::os::raw::c_int
        }
    }
    (*type_0).t = t;
    return type_found;
}
/* convert a function parameter type (array to pointer and function to
function pointer) */
#[inline]
unsafe extern "C" fn convert_parameter_type(mut pt: *mut CType) {
    /* remove const and volatile qualifiers (XXX: const could be used
    to indicate a const function parameter */
    (*pt).t &= !(0x100 as std::os::raw::c_int | 0x200 as std::os::raw::c_int);
    /* array must be transformed to pointer according to ANSI C */
    (*pt).t &= !(0x40 as std::os::raw::c_int);
    if (*pt).t & 0xf as std::os::raw::c_int == 6 as std::os::raw::c_int {
        mk_pointer(pt);
    };
}
#[no_mangle]
pub unsafe extern "C" fn parse_asm_str(mut astr: *mut CString) {
    skip('(' as i32);
    parse_mult_str(
        astr,
        b"string constant\x00" as *const u8 as *const std::os::raw::c_char,
    );
}
/* Parse an asm label and return the token */
unsafe extern "C" fn asm_label_instr() -> std::os::raw::c_int {
    let mut v: std::os::raw::c_int = 0;
    let mut astr: CString = CString {
        size: 0,
        data: 0 as *const std::os::raw::c_void as *mut std::os::raw::c_void,
        size_allocated: 0,
    };
    next();
    parse_asm_str(&mut astr);
    skip(')' as i32);
    v = (*tok_alloc(
        astr.data as *const std::os::raw::c_char,
        astr.size - 1 as std::os::raw::c_int,
    ))
    .tok;
    cstr_free(&mut astr);
    return v;
}
unsafe extern "C" fn post_type(
    mut type_0: *mut CType,
    mut ad: *mut AttributeDef,
    mut storage: std::os::raw::c_int,
    mut td: std::os::raw::c_int,
) -> std::os::raw::c_int {
    let mut n: std::os::raw::c_int = 0;
    let mut l: std::os::raw::c_int = 0;
    let mut t1: std::os::raw::c_int = 0;
    let mut arg_size: std::os::raw::c_int = 0;
    let mut align: std::os::raw::c_int = 0;
    let mut unused_align: std::os::raw::c_int = 0;
    let mut plast: *mut *mut Sym = 0 as *mut *mut Sym;
    let mut s: *mut Sym = 0 as *mut Sym;
    let mut first: *mut Sym = 0 as *mut Sym;
    let mut ad1: AttributeDef = AttributeDef {
        a: SymAttr([0; 2]),
        f: FuncAttr([0; 4]),
        section: 0 as *mut Section,
        cleanup_func: 0 as *mut Sym,
        alias_target: 0,
        asm_label: 0,
        attr_mode: 0,
    };
    let mut pt: CType = CType {
        t: 0,
        ref_0: 0 as *const Sym as *mut Sym,
    };
    if tok == '(' as i32 {
        /* function type, or recursive declarator (return if so) */
        next();
        if td != 0 && td & 1 as std::os::raw::c_int == 0 {
            return 0 as std::os::raw::c_int;
        }
        if tok == ')' as i32 {
            l = 0 as std::os::raw::c_int
        } else if parse_btype(&mut pt, &mut ad1) != 0 {
            l = 1 as std::os::raw::c_int
        } else if td != 0 {
            merge_attr(ad, &mut ad1);
            return 0 as std::os::raw::c_int;
        } else {
            l = 2 as std::os::raw::c_int
        }
        first = 0 as *mut Sym;
        plast = &mut first;
        arg_size = 0 as std::os::raw::c_int;
        if l != 0 {
            loop
            /* read param name and compute offset */
            {
                if l != 2 as std::os::raw::c_int {
                    if pt.t & 0xf as std::os::raw::c_int == 0 as std::os::raw::c_int
                        && tok == ')' as i32
                    {
                        break; /* invalid type */
                    }
                    type_decl(
                        &mut pt,
                        &mut ad1,
                        &mut n,
                        2 as std::os::raw::c_int | 1 as std::os::raw::c_int,
                    );
                    if pt.t & 0xf as std::os::raw::c_int == 0 as std::os::raw::c_int {
                        _tcc_error(
                            b"parameter declared as void\x00" as *const u8
                                as *const std::os::raw::c_char,
                        );
                    }
                } else {
                    n = tok;
                    if n < TOK_DEFINE as std::os::raw::c_int {
                        expect(b"identifier\x00" as *const u8 as *const std::os::raw::c_char);
                    }
                    pt.t = 0 as std::os::raw::c_int;
                    pt.ref_0 = 0 as *mut Sym;
                    next();
                }
                convert_parameter_type(&mut pt);
                arg_size += (type_size(&mut pt, &mut align) + 8 as std::os::raw::c_int
                    - 1 as std::os::raw::c_int)
                    / 8 as std::os::raw::c_int;
                s = sym_push(
                    n | 0x20000000 as std::os::raw::c_int,
                    &mut pt,
                    0 as std::os::raw::c_int,
                    0 as std::os::raw::c_int,
                );
                *plast = s;
                plast = &mut (*s).c2rust_unnamed_0.next;
                if tok == ')' as i32 {
                    break;
                }
                skip(',' as i32);
                if l == 1 as std::os::raw::c_int && tok == 0xa1 as std::os::raw::c_int {
                    l = 3 as std::os::raw::c_int;
                    next();
                    break;
                } else if l == 1 as std::os::raw::c_int && parse_btype(&mut pt, &mut ad1) == 0 {
                    _tcc_error(b"invalid type\x00" as *const u8 as *const std::os::raw::c_char);
                }
            }
        } else {
            /* if no parameters, then old type prototype */
            l = 2 as std::os::raw::c_int
        }
        skip(')' as i32);
        /* NOTE: const is ignored in returned type as it has a special
        meaning in gcc / C++ */
        (*type_0).t &= !(0x100 as std::os::raw::c_int);
        /* some ancient pre-K&R C allows a function to return an array
        and the array brackets to be put after the arguments, such
        that "int c()[]" means something like "int[] c()" */
        if tok == '[' as i32 {
            next(); /* only handle simple "[]" */
            skip(']' as i32);
            mk_pointer(type_0);
        }
        /* we push a anonymous symbol which will contain the function prototype */
        (*ad).f.set_func_args(arg_size as std::os::raw::c_uint);
        (*ad).f.set_func_type(l as std::os::raw::c_uint);
        s = sym_push(
            0x20000000 as std::os::raw::c_int,
            type_0,
            0 as std::os::raw::c_int,
            0 as std::os::raw::c_int,
        );
        (*s).a = (*ad).a;
        (*s).c2rust_unnamed.c2rust_unnamed.c2rust_unnamed.f = (*ad).f;
        (*s).c2rust_unnamed_0.next = first;
        (*type_0).t = 6 as std::os::raw::c_int;
        (*type_0).ref_0 = s
    } else if tok == '[' as i32 {
        let mut saved_nocode_wanted: std::os::raw::c_int = nocode_wanted;
        /* array definition */
        next();
        loop
        /* XXX The optional type-quals and static should only be accepted
        in parameter decls.  The '*' as well, and then even only
        in prototypes (not function defs).  */
        {
            match tok {
                289 | 290 | 291 | 274 | 277 | 266 | 42 => {},
                _ => {
                    break;
                },
            }
            next();
        }
        n = -(1 as std::os::raw::c_int);
        t1 = 0 as std::os::raw::c_int;
        if tok != ']' as i32 {
            if local_stack.is_null() || storage & 0x2000 as std::os::raw::c_int != 0 {
                vpushi(expr_const());
            } else {
                /* VLAs (which can only happen with local_stack && !VT_STATIC)
                length must always be evaluated, even under nocode_wanted,
                so that its size slot is initialized (e.g. under sizeof
                or typeof).  */
                nocode_wanted = 0 as std::os::raw::c_int;
                gexpr();
            }
            if (*vtop).r as std::os::raw::c_int
                & (0x3f as std::os::raw::c_int
                    | 0x100 as std::os::raw::c_int
                    | 0x200 as std::os::raw::c_int)
                == 0x30 as std::os::raw::c_int
            {
                n = (*vtop).c2rust_unnamed.c.i as std::os::raw::c_int;
                if n < 0 as std::os::raw::c_int {
                    _tcc_error(
                        b"invalid array size\x00" as *const u8 as *const std::os::raw::c_char,
                    );
                }
            } else {
                if is_integer_btype((*vtop).type_0.t & 0xf as std::os::raw::c_int) == 0 {
                    _tcc_error(
                        b"size of variable length array should be an integer\x00" as *const u8
                            as *const std::os::raw::c_char,
                    );
                }
                n = 0 as std::os::raw::c_int;
                t1 = 0x400 as std::os::raw::c_int
            }
        }
        skip(']' as i32);
        /* parse next post type */
        post_type(type_0, ad, storage, 0 as std::os::raw::c_int);
        if (*type_0).t & 0xf as std::os::raw::c_int == 6 as std::os::raw::c_int {
            _tcc_error(
                b"declaration of an array of functions\x00" as *const u8
                    as *const std::os::raw::c_char,
            );
        }
        if (*type_0).t & 0xf as std::os::raw::c_int == 0 as std::os::raw::c_int
            || type_size(type_0, &mut unused_align) < 0 as std::os::raw::c_int
        {
            _tcc_error(
                b"declaration of an array of incomplete type elements\x00" as *const u8
                    as *const std::os::raw::c_char,
            );
        }
        t1 |= (*type_0).t & 0x400 as std::os::raw::c_int;
        if t1 & 0x400 as std::os::raw::c_int != 0 {
            if n < 0 as std::os::raw::c_int {
                _tcc_error(
                    b"need explicit inner array size in VLAs\x00" as *const u8
                        as *const std::os::raw::c_char,
                );
            }
            loc -= type_size(&mut int_type, &mut align);
            loc &= -align;
            n = loc;
            vla_runtime_type_size(type_0, &mut align);
            gen_op('*' as i32);
            vset(
                &mut int_type,
                0x32 as std::os::raw::c_int | 0x100 as std::os::raw::c_int,
                n,
            );
            vswap();
            vstore();
        }
        if n != -(1 as std::os::raw::c_int) {
            vpop();
        }
        nocode_wanted = saved_nocode_wanted;
        /* we push an anonymous symbol which will contain the array
        element type */
        s = sym_push(
            0x20000000 as std::os::raw::c_int,
            type_0,
            0 as std::os::raw::c_int,
            n,
        );
        (*type_0).t = (if t1 != 0 {
            0x400 as std::os::raw::c_int
        } else {
            0x40 as std::os::raw::c_int
        }) | 5 as std::os::raw::c_int;
        (*type_0).ref_0 = s
    }
    return 1 as std::os::raw::c_int;
}
/* Parse a type declarator (except basic type), and return the type
in 'type'. 'td' is a bitmask indicating which kind of type decl is
expected. 'type' should contain the basic type. 'ad' is the
attribute definition of the basic type. It can be modified by
type_decl().  If this (possibly abstract) declarator is a pointer chain
it returns the innermost pointed to type (equals *type, but is a different
pointer), otherwise returns type itself, that's used for recursive calls.  */
unsafe extern "C" fn type_decl(
    mut type_0: *mut CType,
    mut ad: *mut AttributeDef,
    mut v: *mut std::os::raw::c_int,
    mut td: std::os::raw::c_int,
) -> *mut CType {
    let mut post: *mut CType = 0 as *mut CType;
    let mut ret: *mut CType = 0 as *mut CType;
    let mut qualifiers: std::os::raw::c_int = 0;
    let mut storage: std::os::raw::c_int = 0;
    /* recursive type, remove storage bits first, apply them later again */
    storage = (*type_0).t
        & (0x1000 as std::os::raw::c_int
            | 0x2000 as std::os::raw::c_int
            | 0x4000 as std::os::raw::c_int
            | 0x8000 as std::os::raw::c_int);
    (*type_0).t &= !(0x1000 as std::os::raw::c_int
        | 0x2000 as std::os::raw::c_int
        | 0x4000 as std::os::raw::c_int
        | 0x8000 as std::os::raw::c_int);
    ret = type_0;
    post = ret;
    while tok == '*' as i32 {
        qualifiers = 0 as std::os::raw::c_int;
        loop {
            next();
            match tok {
                273 => qualifiers |= 0x200 as std::os::raw::c_int,
                274 | 275 | 276 => qualifiers |= 0x100 as std::os::raw::c_int,
                277 | 278 | 279 => qualifiers |= 0x200 as std::os::raw::c_int,
                289 | 290 | 291 => {},
                305 | 306 => {
                    /* XXX: clarify attribute handling */
                    parse_attribute(ad);
                    break;
                },
                _ => {
                    break;
                },
            }
        }
        mk_pointer(type_0);
        (*type_0).t |= qualifiers;
        if ret == type_0 {
            /* innermost pointed to type is the one for the first derivation */
            ret = pointed_type(type_0)
        }
    }
    let mut current_block_22: u64;
    if tok == '(' as i32 {
        /* This is possibly a parameter type list for abstract declarators
        ('int ()'), use post_type for testing this.  */
        if post_type(type_0, ad, 0 as std::os::raw::c_int, td) == 0 {
            /* It's not, so it's a nested declarator, and the post operations
            apply to the innermost pointed to type (if any).  */
            /* XXX: this is not correct to modify 'ad' at this point, but
            the syntax is not clear */
            parse_attribute(ad);
            post = type_decl(type_0, ad, v, td);
            skip(')' as i32);
            current_block_22 = 9853141518545631134;
        } else {
            current_block_22 = 16090855711032207392;
        }
    } else if tok >= 256 as std::os::raw::c_int && td & 2 as std::os::raw::c_int != 0 {
        /* type identifier */
        *v = tok;
        next();
        current_block_22 = 9853141518545631134;
    } else {
        current_block_22 = 16090855711032207392;
    }
    match current_block_22 {
        16090855711032207392 => {
            if td & 1 as std::os::raw::c_int == 0 {
                expect(b"identifier\x00" as *const u8 as *const std::os::raw::c_char);
            }
            *v = 0 as std::os::raw::c_int
        },
        _ => {},
    }
    post_type(post, ad, storage, 0 as std::os::raw::c_int);
    parse_attribute(ad);
    (*type_0).t |= storage;
    return ret;
}
/* indirection with full error checking and bound check */
#[no_mangle]
pub unsafe extern "C" fn indir() {
    if (*vtop).type_0.t & 0xf as std::os::raw::c_int != 5 as std::os::raw::c_int {
        if (*vtop).type_0.t & 0xf as std::os::raw::c_int == 6 as std::os::raw::c_int {
            return;
        }
        expect(b"pointer\x00" as *const u8 as *const std::os::raw::c_char);
    }
    if (*vtop).r as std::os::raw::c_int & 0x100 as std::os::raw::c_int != 0 {
        gv(0x1 as std::os::raw::c_int);
    }
    (*vtop).type_0 = *pointed_type(&mut (*vtop).type_0);
    /* Arrays and functions are never lvalues */
    if (*vtop).type_0.t & (0x40 as std::os::raw::c_int | 0x400 as std::os::raw::c_int) == 0
        && (*vtop).type_0.t & 0xf as std::os::raw::c_int != 6 as std::os::raw::c_int
    {
        (*vtop).r = ((*vtop).r as std::os::raw::c_int | 0x100 as std::os::raw::c_int)
            as std::os::raw::c_ushort;
        /* if bound checking, the referenced pointer must be checked */
        if (*tcc_state).do_bounds_check != 0 {
            (*vtop).r = ((*vtop).r as std::os::raw::c_int | 0x4000 as std::os::raw::c_int)
                as std::os::raw::c_ushort
        }
    };
}
/* pass a parameter to a function and do type checking and casting */
unsafe extern "C" fn gfunc_param_typed(mut func: *mut Sym, mut arg: *mut Sym) {
    let mut func_type: std::os::raw::c_int = 0;
    let mut type_0: CType = CType {
        t: 0,
        ref_0: 0 as *const Sym as *mut Sym,
    };
    func_type = (*func)
        .c2rust_unnamed
        .c2rust_unnamed
        .c2rust_unnamed
        .f
        .func_type() as std::os::raw::c_int;
    if func_type == 2 as std::os::raw::c_int
        || func_type == 3 as std::os::raw::c_int && arg.is_null()
    {
        /* default casting : only need to convert float to double */
        if (*vtop).type_0.t & 0xf as std::os::raw::c_int == 8 as std::os::raw::c_int {
            gen_cast_s(9 as std::os::raw::c_int); /* need to do that to avoid false warning */
        } else if (*vtop).type_0.t & 0x80 as std::os::raw::c_int != 0 {
            type_0.t =
                (*vtop).type_0.t & (0xf as std::os::raw::c_int | 0x10 as std::os::raw::c_int);
            type_0.ref_0 = (*vtop).type_0.ref_0;
            gen_cast(&mut type_0);
        } else if (*vtop).r as std::os::raw::c_int & 0xc00 as std::os::raw::c_int != 0 {
            force_charshort_cast();
        }
    } else if arg.is_null() {
        _tcc_error(
            b"too many arguments to function\x00" as *const u8 as *const std::os::raw::c_char,
        );
    } else {
        type_0 = (*arg).type_0;
        type_0.t &= !(0x100 as std::os::raw::c_int);
        gen_assign_cast(&mut type_0);
    };
}
/* parse an expression and return its type without any side effect. */
unsafe extern "C" fn expr_type(
    mut type_0: *mut CType,
    mut expr_fn: Option<unsafe extern "C" fn() -> ()>,
) {
    nocode_wanted += 1;
    expr_fn.expect("non-null function pointer")();
    *type_0 = (*vtop).type_0;
    vpop();
    nocode_wanted -= 1;
}
/* parse an expression of the form '(type)' or '(expr)' and return its
type */
unsafe extern "C" fn parse_expr_type(mut type_0: *mut CType) {
    let mut n: std::os::raw::c_int = 0;
    let mut ad: AttributeDef = AttributeDef {
        a: SymAttr([0; 2]),
        f: FuncAttr([0; 4]),
        section: 0 as *mut Section,
        cleanup_func: 0 as *mut Sym,
        alias_target: 0,
        asm_label: 0,
        attr_mode: 0,
    };
    skip('(' as i32);
    if parse_btype(type_0, &mut ad) != 0 {
        type_decl(type_0, &mut ad, &mut n, 1 as std::os::raw::c_int);
    } else {
        expr_type(type_0, Some(gexpr as unsafe extern "C" fn() -> ()));
    }
    skip(')' as i32);
}
unsafe extern "C" fn parse_type(mut type_0: *mut CType) {
    let mut ad: AttributeDef = AttributeDef {
        a: SymAttr([0; 2]),
        f: FuncAttr([0; 4]),
        section: 0 as *mut Section,
        cleanup_func: 0 as *mut Sym,
        alias_target: 0,
        asm_label: 0,
        attr_mode: 0,
    };
    let mut n: std::os::raw::c_int = 0;
    if parse_btype(type_0, &mut ad) == 0 {
        expect(b"type\x00" as *const u8 as *const std::os::raw::c_char);
    }
    type_decl(type_0, &mut ad, &mut n, 1 as std::os::raw::c_int);
}
unsafe extern "C" fn parse_builtin_params(
    mut nc: std::os::raw::c_int,
    mut args: *const std::os::raw::c_char,
) {
    let mut c: std::os::raw::c_char = 0;
    let mut sep: std::os::raw::c_char = '(' as i32 as std::os::raw::c_char;
    let mut type_0: CType = CType {
        t: 0,
        ref_0: 0 as *const Sym as *mut Sym,
    };
    if nc != 0 {
        nocode_wanted += 1
    }
    next();
    if *args as std::os::raw::c_int == 0 as std::os::raw::c_int {
        skip(sep as std::os::raw::c_int);
    }
    let mut current_block_20: u64;
    loop {
        let fresh7 = args;
        args = args.offset(1);
        c = *fresh7;
        if !(c != 0) {
            break;
        }
        skip(sep as std::os::raw::c_int);
        sep = ',' as i32 as std::os::raw::c_char;
        if c as std::os::raw::c_int == 't' as i32 {
            parse_type(&mut type_0);
            vpush(&mut type_0);
        } else {
            expr_eq();
            type_0.ref_0 = 0 as *mut Sym;
            type_0.t = 0 as std::os::raw::c_int;
            match c as std::os::raw::c_int {
                101 => {
                    continue;
                },
                86 => {
                    type_0.t = 0x100 as std::os::raw::c_int;
                    current_block_20 = 2697510147464577642;
                },
                118 => {
                    current_block_20 = 2697510147464577642;
                },
                83 => {
                    type_0.t = 0x100 as std::os::raw::c_int;
                    current_block_20 = 12324313029698471781;
                },
                115 => {
                    current_block_20 = 12324313029698471781;
                },
                105 => {
                    type_0.t = 3 as std::os::raw::c_int;
                    current_block_20 = 11057878835866523405;
                },
                108 => {
                    type_0.t = 0x800 as std::os::raw::c_int
                        | 4 as std::os::raw::c_int
                        | 0x10 as std::os::raw::c_int;
                    current_block_20 = 11057878835866523405;
                },
                _ => {
                    current_block_20 = 11057878835866523405;
                },
            }
            match current_block_20 {
                2697510147464577642 => {
                    type_0.t |= 0 as std::os::raw::c_int;
                    mk_pointer(&mut type_0);
                },
                12324313029698471781 => {
                    type_0.t |= char_type.t;
                    mk_pointer(&mut type_0);
                },
                _ => {},
            }
            gen_assign_cast(&mut type_0);
        }
    }
    skip(')' as i32);
    if nc != 0 {
        nocode_wanted -= 1
    };
}
#[inline]
unsafe extern "C" fn is_memory_model(mut sv: *const SValue) -> std::os::raw::c_int {
    /*
     * FIXME
     * The memory models should better be backed by an enumeration.
     *
     *    const int t = sv->type.t;
     *
     *    if (!IS_ENUM_VAL(t))
     *        return 0;
     *
     *    if (!(t & VT_STATIC))
     *        return 0;
     *
     * Ideally we should check whether the model matches 1:1.
     * If it is possible, we should check by the name of the value.
     */
    return 1 as std::os::raw::c_int; /* pacify compiler */
}
unsafe extern "C" fn parse_atomic(mut atok: std::os::raw::c_int) {
    let mut mode: std::os::raw::c_int = 0;
    let mut arg: size_t = 0;
    let mut call: *mut SValue = 0 as *mut SValue;
    let mut atom: CType = {
        let mut init = CType {
            t: 0,
            ref_0: 0 as *mut Sym,
        };
        init
    };
    static mut templates: [*const std::os::raw::c_char; 41] = [
        b"avm?\x00" as *const u8 as *const std::os::raw::c_char,
        0 as *const std::os::raw::c_char,
        0 as *const std::os::raw::c_char,
        0 as *const std::os::raw::c_char,
        0 as *const std::os::raw::c_char,
        b"Amv\x00" as *const u8 as *const std::os::raw::c_char,
        0 as *const std::os::raw::c_char,
        0 as *const std::os::raw::c_char,
        0 as *const std::os::raw::c_char,
        0 as *const std::os::raw::c_char,
        b"avmv\x00" as *const u8 as *const std::os::raw::c_char,
        0 as *const std::os::raw::c_char,
        0 as *const std::os::raw::c_char,
        0 as *const std::os::raw::c_char,
        0 as *const std::os::raw::c_char,
        b"apvbmmb\x00" as *const u8 as *const std::os::raw::c_char,
        0 as *const std::os::raw::c_char,
        0 as *const std::os::raw::c_char,
        0 as *const std::os::raw::c_char,
        0 as *const std::os::raw::c_char,
        b"avmv\x00" as *const u8 as *const std::os::raw::c_char,
        0 as *const std::os::raw::c_char,
        0 as *const std::os::raw::c_char,
        0 as *const std::os::raw::c_char,
        0 as *const std::os::raw::c_char,
        b"avmv\x00" as *const u8 as *const std::os::raw::c_char,
        0 as *const std::os::raw::c_char,
        0 as *const std::os::raw::c_char,
        0 as *const std::os::raw::c_char,
        0 as *const std::os::raw::c_char,
        b"avmv\x00" as *const u8 as *const std::os::raw::c_char,
        0 as *const std::os::raw::c_char,
        0 as *const std::os::raw::c_char,
        0 as *const std::os::raw::c_char,
        0 as *const std::os::raw::c_char,
        b"avmv\x00" as *const u8 as *const std::os::raw::c_char,
        0 as *const std::os::raw::c_char,
        0 as *const std::os::raw::c_char,
        0 as *const std::os::raw::c_char,
        0 as *const std::os::raw::c_char,
        b"avmv\x00" as *const u8 as *const std::os::raw::c_char,
    ];
    let mut template: *const std::os::raw::c_char =
        templates[(atok - TOK___atomic_store as std::os::raw::c_int) as usize];
    let argc: size_t =
        strlen(template).wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong);
    next();
    mode = 0 as std::os::raw::c_int;
    vpush_helper_func(atok);
    call = vtop;
    skip('(' as i32);
    if *template as std::os::raw::c_int != 'a' as i32
        && *template as std::os::raw::c_int != 'A' as i32
    {
        expect_arg(
            b"pointer to atomic\x00" as *const u8 as *const std::os::raw::c_char,
            0 as std::os::raw::c_int as size_t,
        );
    }
    arg = 0 as std::os::raw::c_int as size_t;
    while arg < argc {
        expr_eq();
        match *template.offset(arg as isize) as std::os::raw::c_int {
            63 => {
                /* void makes sense only for return value. */
                if arg != argc.wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong) {
                    _tcc_error(
                        b"illegal atomic built-in template\x00" as *const u8
                            as *const std::os::raw::c_char,
                    );
                }
            },
            98 => {},
            97 | 65 => {
                if (*vtop).type_0.t & 0xf as std::os::raw::c_int != 5 as std::os::raw::c_int {
                    expect_arg(
                        b"pointer to atomic value\x00" as *const u8 as *const std::os::raw::c_char,
                        arg,
                    );
                }
                memcpy(
                    &mut atom as *mut CType as *mut std::os::raw::c_void,
                    pointed_type(&mut (*vtop).type_0) as *const std::os::raw::c_void,
                    ::std::mem::size_of::<CType>() as std::os::raw::c_ulong,
                );
                if atom.t & 0x200 as std::os::raw::c_int == 0 {
                    expect_arg(
                        b"qualified pointer to atomic value\x00" as *const u8
                            as *const std::os::raw::c_char,
                        arg,
                    );
                }
                if *template.offset(arg as isize) as std::os::raw::c_int == 'a' as i32
                    && atom.t & 0x100 as std::os::raw::c_int != 0
                {
                    expect_arg(
                        b"pointer to writable atomic\x00" as *const u8
                            as *const std::os::raw::c_char,
                        arg,
                    );
                }
                match btype_size(atom.t & 0xf as std::os::raw::c_int) {
                    8 => mode = 4 as std::os::raw::c_int,
                    4 => mode = 3 as std::os::raw::c_int,
                    2 => mode = 2 as std::os::raw::c_int,
                    1 => mode = 1 as std::os::raw::c_int,
                    _ => {
                        _tcc_error(
                            b"only integer-sized types are supported\x00" as *const u8
                                as *const std::os::raw::c_char,
                        );
                    },
                }
            },
            112 => {
                if (*vtop).type_0.t & 0xf as std::os::raw::c_int != 5 as std::os::raw::c_int
                    || is_compatible_unqualified_types(&mut atom, pointed_type(&mut (*vtop).type_0))
                        == 0
                {
                    expect_arg(
                        b"pointer to compatible type\x00" as *const u8
                            as *const std::os::raw::c_char,
                        arg,
                    );
                }
            },
            118 => {
                if is_integer_btype((*vtop).type_0.t & 0xf as std::os::raw::c_int) == 0 {
                    expect_arg(
                        b"integer type\x00" as *const u8 as *const std::os::raw::c_char,
                        arg,
                    );
                }
            },
            109 => {
                if is_memory_model(vtop) == 0 {
                    expect_arg(
                        b"memory model\x00" as *const u8 as *const std::os::raw::c_char,
                        arg,
                    );
                }
                (*vtop).type_0.t &= !(0x2000 as std::os::raw::c_int
                    | (3 as std::os::raw::c_int) << 20 as std::os::raw::c_int
                    | 0x4000 as std::os::raw::c_int)
            },
            _ => {
                _tcc_error(
                    b"unknown parameter type\x00" as *const u8 as *const std::os::raw::c_char,
                );
            },
        }
        if tok == ')' as i32 {
            break;
        }
        skip(',' as i32);
        arg = arg.wrapping_add(1)
    }
    if arg < argc.wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong) {
        expect(b"more parameters\x00" as *const u8 as *const std::os::raw::c_char);
    }
    if arg > argc.wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong) {
        expect(b"less parameters\x00" as *const u8 as *const std::os::raw::c_char);
    }
    skip(')' as i32);
    (*call).c2rust_unnamed_0.sym = external_helper_sym(atok + mode);
    gfunc_call(argc as std::os::raw::c_int);
    vpushi(0 as std::os::raw::c_int);
    match *template.offset(argc as isize) as std::os::raw::c_int {
        98 => {
            PUT_R_RET(vtop, 11 as std::os::raw::c_int);
        },
        118 => {
            PUT_R_RET(vtop, atom.t);
        },
        112 => {
            PUT_R_RET(
                vtop,
                0x800 as std::os::raw::c_int
                    | 4 as std::os::raw::c_int
                    | 0x10 as std::os::raw::c_int,
            );
        },
        63 => {
            PUT_R_RET(vtop, 0 as std::os::raw::c_int);
        },
        _ => {
            _tcc_error(
                b"incorrect atomic template\x00" as *const u8 as *const std::os::raw::c_char,
            );
        },
    };
}
#[no_mangle]
pub unsafe extern "C" fn unary() {
    let mut current_block: u64;
    let mut n: std::os::raw::c_int = 0;
    let mut t: std::os::raw::c_int = 0;
    let mut align: std::os::raw::c_int = 0;
    let mut size: std::os::raw::c_int = 0;
    let mut r: std::os::raw::c_int = 0;
    let mut sizeof_caller: std::os::raw::c_int = 0;
    let mut type_0: CType = CType {
        t: 0,
        ref_0: 0 as *const Sym as *mut Sym,
    };
    let mut s: *mut Sym = 0 as *mut Sym;
    let mut ad: AttributeDef = AttributeDef {
        a: SymAttr([0; 2]),
        f: FuncAttr([0; 4]),
        section: 0 as *mut Section,
        cleanup_func: 0 as *mut Sym,
        alias_target: 0,
        asm_label: 0,
        attr_mode: 0,
    };
    /* generate line number info */
    if debug_modes != 0 {
        tcc_debug_line(tcc_state);
        tcc_tcov_check_line(1 as std::os::raw::c_int);
    }
    sizeof_caller = in_sizeof;
    in_sizeof = 0 as std::os::raw::c_int;
    type_0.ref_0 = 0 as *mut Sym;
    loop
    /* XXX: GCC 2.95.3 does not generate a table although it should be
    better here */
    {
        match tok {
            292 => {
                next();
            },
            193 | 194 | 192 => {
                t = 3 as std::os::raw::c_int;
                current_block = 9632834660368191687;
                break;
            },
            195 => {
                t = 3 as std::os::raw::c_int | 0x10 as std::os::raw::c_int;
                current_block = 9632834660368191687;
                break;
            },
            196 => {
                t = 4 as std::os::raw::c_int;
                current_block = 9632834660368191687;
                break;
            },
            197 => {
                t = 4 as std::os::raw::c_int | 0x10 as std::os::raw::c_int;
                current_block = 9632834660368191687;
                break;
            },
            202 => {
                t = 8 as std::os::raw::c_int;
                current_block = 9632834660368191687;
                break;
            },
            203 => {
                t = 9 as std::os::raw::c_int;
                current_block = 9632834660368191687;
                break;
            },
            204 => {
                t = 10 as std::os::raw::c_int;
                current_block = 9632834660368191687;
                break;
            },
            198 => {
                t = (if 8 as std::os::raw::c_int == 8 as std::os::raw::c_int {
                    4 as std::os::raw::c_int
                } else {
                    3 as std::os::raw::c_int
                }) | 0x800 as std::os::raw::c_int;
                current_block = 9632834660368191687;
                break;
            },
            199 => {
                t = (if 8 as std::os::raw::c_int == 8 as std::os::raw::c_int {
                    4 as std::os::raw::c_int
                } else {
                    3 as std::os::raw::c_int
                }) | 0x800 as std::os::raw::c_int
                    | 0x10 as std::os::raw::c_int;
                current_block = 9632834660368191687;
                break;
            },
            335 => {
                if (*tcc_state).gnu_ext == 0 {
                    current_block = 9048094288174807049;
                    break;
                } else {
                    current_block = 10089449838879554637;
                    break;
                }
            },
            339 => {
                current_block = 10089449838879554637;
                break;
            },
            201 => {
                t = 3 as std::os::raw::c_int;
                current_block = 17985719102395905236;
                break;
            },
            200 => {
                /* string parsing */
                t = 1 as std::os::raw::c_int;
                if (*tcc_state).char_is_unsigned != 0 {
                    t = 1 as std::os::raw::c_int | 0x10 as std::os::raw::c_int
                }
                current_block = 17985719102395905236;
                break;
            },
            40 => {
                next();
                /* cast ? */
                if parse_btype(&mut type_0, &mut ad) != 0 {
                    type_decl(&mut type_0, &mut ad, &mut n, 1 as std::os::raw::c_int);
                    skip(')' as i32);
                    /* check ISOC99 compound literal */
                    if tok == '{' as i32 {
                        /* data is allocated locally by default */
                        if global_expr != 0 {
                            r = 0x30 as std::os::raw::c_int
                        } else {
                            r = 0x32 as std::os::raw::c_int
                        }
                        /* all except arrays are lvalues */
                        if type_0.t & 0x40 as std::os::raw::c_int == 0 {
                            r |= 0x100 as std::os::raw::c_int
                        }
                        memset(
                            &mut ad as *mut AttributeDef as *mut std::os::raw::c_void,
                            0 as std::os::raw::c_int,
                            ::std::mem::size_of::<AttributeDef>() as std::os::raw::c_ulong,
                        );
                        decl_initializer_alloc(
                            &mut type_0,
                            &mut ad,
                            r,
                            1 as std::os::raw::c_int,
                            0 as std::os::raw::c_int,
                            0 as std::os::raw::c_int,
                        );
                    } else {
                        if sizeof_caller != 0 {
                            vpush(&mut type_0);
                            return;
                        }
                        unary();
                        gen_cast(&mut type_0);
                    }
                } else if tok == '{' as i32 {
                    let mut saved_nocode_wanted: std::os::raw::c_int = nocode_wanted;
                    if const_wanted != 0 && nocode_wanted & 0xffff as std::os::raw::c_int == 0 {
                        expect(b"constant\x00" as *const u8 as *const std::os::raw::c_char);
                    }
                    if 0 as std::os::raw::c_int == local_scope {
                        _tcc_error(
                            b"statement expression outside of function\x00" as *const u8
                                as *const std::os::raw::c_char,
                        );
                    }
                    /* save all registers */
                    save_regs(0 as std::os::raw::c_int);
                    /* statement expression : we do not accept break/continue
                        inside as GCC does.  We do retain the nocode_wanted state,
                    as statement expressions can't ever be entered from the
                    outside, so any reactivation of code emission (from labels
                    or loop heads) can be disabled again after the end of it. */
                    block(1 as std::os::raw::c_int);
                    nocode_wanted = saved_nocode_wanted;
                    skip(')' as i32);
                } else {
                    gexpr();
                    skip(')' as i32);
                }
                current_block = 15568404490809570198;
                break;
            },
            42 => {
                next();
                unary();
                indir();
                current_block = 15568404490809570198;
                break;
            },
            38 => {
                next();
                unary();
                /* functions names must be treated as function pointers,
                except for unary '&' and sizeof. Since we consider that
                functions are not lvalues, we only have to handle it
                there and in function calls. */
                /* arrays can also be used although they are not lvalues */
                if (*vtop).type_0.t & 0xf as std::os::raw::c_int != 6 as std::os::raw::c_int
                    && (*vtop).type_0.t & 0x40 as std::os::raw::c_int == 0
                {
                    test_lvalue();
                }
                if !(*vtop).c2rust_unnamed_0.sym.is_null() {
                    (*(*vtop).c2rust_unnamed_0.sym)
                        .a
                        .set_addrtaken(1 as std::os::raw::c_int as std::os::raw::c_ushort)
                }
                mk_pointer(&mut (*vtop).type_0);
                gaddrof();
                current_block = 15568404490809570198;
                break;
            },
            33 => {
                next();
                unary();
                gen_test_zero(0x94 as std::os::raw::c_int);
                current_block = 15568404490809570198;
                break;
            },
            126 => {
                next();
                unary();
                vpushi(-(1 as std::os::raw::c_int));
                gen_op('^' as i32);
                current_block = 15568404490809570198;
                break;
            },
            43 => {
                next();
                unary();
                if (*vtop).type_0.t & 0xf as std::os::raw::c_int == 5 as std::os::raw::c_int {
                    _tcc_error(
                        b"pointer not accepted for unary plus\x00" as *const u8
                            as *const std::os::raw::c_char,
                    );
                }
                /* In order to force cast, we add zero, except for floating point
                where we really need an noop (otherwise -0.0 will be transformed
                into +0.0).  */
                if is_float((*vtop).type_0.t) == 0 {
                    vpushi(0 as std::os::raw::c_int); /* Perform a in_sizeof = 0; */
                    gen_op('+' as i32); /* hack: accessing previous vtop */
                }
                current_block = 15568404490809570198;
                break;
            },
            304 | 307 | 308 | 309 => {
                t = tok;
                next();
                in_sizeof += 1;
                expr_type(&mut type_0, Some(unary as unsafe extern "C" fn() -> ()));
                s = 0 as *mut Sym;
                if (*vtop.offset(1 as std::os::raw::c_int as isize)).r as std::os::raw::c_int
                    & 0x200 as std::os::raw::c_int
                    != 0
                {
                    s = (*vtop.offset(1 as std::os::raw::c_int as isize))
                        .c2rust_unnamed_0
                        .sym
                }
                size = type_size(&mut type_0, &mut align);
                if !s.is_null() && (*s).a.aligned() as std::os::raw::c_int != 0 {
                    align = (1 as std::os::raw::c_int)
                        << (*s).a.aligned() as std::os::raw::c_int - 1 as std::os::raw::c_int
                }
                if t == TOK_SIZEOF as std::os::raw::c_int {
                    if type_0.t & 0x400 as std::os::raw::c_int == 0 {
                        if size < 0 as std::os::raw::c_int {
                            _tcc_error(
                                b"sizeof applied to an incomplete type\x00" as *const u8
                                    as *const std::os::raw::c_char,
                            );
                        }
                        vpushs(size as Elf64_Addr);
                    } else {
                        vla_runtime_type_size(&mut type_0, &mut align);
                    }
                } else {
                    vpushs(align as Elf64_Addr);
                }
                (*vtop).type_0.t |= 0x10 as std::os::raw::c_int;
                current_block = 15568404490809570198;
                break;
            },
            395 => {
                /* __builtin_expect is a no-op for now */
                parse_builtin_params(
                    0 as std::os::raw::c_int,
                    b"ee\x00" as *const u8 as *const std::os::raw::c_char,
                ); /* local frame */
                vpop();
                current_block = 15568404490809570198;
                break;
            },
            390 => {
                parse_builtin_params(
                    0 as std::os::raw::c_int,
                    b"tt\x00" as *const u8 as *const std::os::raw::c_char,
                );
                (*vtop.offset(-(1 as std::os::raw::c_int) as isize))
                    .type_0
                    .t &= !(0x100 as std::os::raw::c_int | 0x200 as std::os::raw::c_int);
                (*vtop.offset(0 as std::os::raw::c_int as isize)).type_0.t &=
                    !(0x100 as std::os::raw::c_int | 0x200 as std::os::raw::c_int);
                n = is_compatible_types(
                    &mut (*vtop.offset(-(1 as std::os::raw::c_int) as isize)).type_0,
                    &mut (*vtop.offset(0 as std::os::raw::c_int as isize)).type_0,
                );
                vtop = vtop.offset(-(2 as std::os::raw::c_int as isize));
                vpushi(n);
                current_block = 15568404490809570198;
                break;
            },
            391 => {
                let mut c: int64_t = 0;
                next();
                skip('(' as i32);
                c = expr_const64();
                skip(',' as i32);
                if c == 0 {
                    nocode_wanted += 1
                }
                expr_eq();
                if c == 0 {
                    vpop();
                    nocode_wanted -= 1
                }
                skip(',' as i32);
                if c != 0 {
                    nocode_wanted += 1
                }
                expr_eq();
                if c != 0 {
                    vpop();
                    nocode_wanted -= 1
                }
                skip(')' as i32);
                current_block = 15568404490809570198;
                break;
            },
            392 => {
                parse_builtin_params(
                    1 as std::os::raw::c_int,
                    b"e\x00" as *const u8 as *const std::os::raw::c_char,
                );
                n = ((*vtop).r as std::os::raw::c_int
                    & (0x3f as std::os::raw::c_int | 0x100 as std::os::raw::c_int)
                    == 0x30 as std::os::raw::c_int
                    && !((*vtop).r as std::os::raw::c_int & 0x200 as std::os::raw::c_int != 0
                        && (*(*vtop).c2rust_unnamed_0.sym).a.addrtaken() as std::os::raw::c_int
                            != 0)) as std::os::raw::c_int;
                vtop = vtop.offset(-1);
                vpushi(n);
                current_block = 15568404490809570198;
                break;
            },
            393 | 394 => {
                let mut tok1: std::os::raw::c_int = tok;
                let mut level: std::os::raw::c_int = 0;
                next();
                skip('(' as i32);
                if tok != 0xc2 as std::os::raw::c_int {
                    _tcc_error(
                        b"%s only takes positive integers\x00" as *const u8
                            as *const std::os::raw::c_char,
                        if tok1 == TOK_builtin_return_address as std::os::raw::c_int {
                            b"__builtin_return_address\x00" as *const u8
                                as *const std::os::raw::c_char
                        } else {
                            b"__builtin_frame_address\x00" as *const u8
                                as *const std::os::raw::c_char
                        },
                    );
                }
                level = tokc.i as uint32_t as std::os::raw::c_int;
                next();
                skip(')' as i32);
                type_0.t = 0 as std::os::raw::c_int;
                mk_pointer(&mut type_0);
                vset(
                    &mut type_0,
                    0x32 as std::os::raw::c_int,
                    0 as std::os::raw::c_int,
                );
                loop {
                    let fresh8 = level;
                    level = level - 1;
                    if !(fresh8 != 0) {
                        break;
                    }
                    mk_pointer(&mut (*vtop).type_0);
                    indir();
                    /* -> parent frame */
                }
                if tok1 == TOK_builtin_return_address as std::os::raw::c_int {
                    // assume return address is just above frame pointer on stack
                    vpushi(8 as std::os::raw::c_int);
                    gen_op('+' as i32);
                    mk_pointer(&mut (*vtop).type_0);
                    indir();
                }
                current_block = 15568404490809570198;
                break;
            },
            396 => {
                parse_builtin_params(
                    0 as std::os::raw::c_int,
                    b"t\x00" as *const u8 as *const std::os::raw::c_char,
                );
                vpushi(classify_x86_64_va_arg(&mut (*vtop).type_0));
                vswap();
                vpop();
                current_block = 15568404490809570198;
                break;
            },
            397 | 402 | 407 | 412 | 417 | 422 | 427 | 432 | 437 => {
                /* atomic operations */
                parse_atomic(tok);
                current_block = 15568404490809570198;
                break;
            },
            130 | 128 => {
                /* pre operations */
                t = tok;
                next();
                unary();
                inc(0 as std::os::raw::c_int, t);
                current_block = 15568404490809570198;
                break;
            },
            45 => {
                next();
                unary();
                if is_float((*vtop).type_0.t) != 0 {
                    gen_opif(0x81 as std::os::raw::c_int);
                } else {
                    vpushi(0 as std::os::raw::c_int);
                    vswap();
                    gen_op('-' as i32);
                }
                current_block = 15568404490809570198;
                break;
            },
            144 => {
                if (*tcc_state).gnu_ext == 0 {
                    current_block = 9048094288174807049;
                    break;
                } else {
                    current_block = 9728093949049737828;
                    break;
                }
            },
            293 => {
                let mut controlling_type: CType = CType {
                    t: 0,
                    ref_0: 0 as *const Sym as *mut Sym,
                };
                let mut has_default: std::os::raw::c_int = 0 as std::os::raw::c_int;
                let mut has_match: std::os::raw::c_int = 0 as std::os::raw::c_int;
                let mut learn: std::os::raw::c_int = 0 as std::os::raw::c_int;
                let mut str: *mut TokenString = 0 as *mut TokenString;
                let mut saved_const_wanted: std::os::raw::c_int = const_wanted;
                next();
                skip('(' as i32);
                const_wanted = 0 as std::os::raw::c_int;
                expr_type(
                    &mut controlling_type,
                    Some(expr_eq as unsafe extern "C" fn() -> ()),
                );
                controlling_type.t &= !(0x100 as std::os::raw::c_int
                    | 0x200 as std::os::raw::c_int
                    | 0x40 as std::os::raw::c_int);
                if controlling_type.t & 0xf as std::os::raw::c_int == 6 as std::os::raw::c_int {
                    mk_pointer(&mut controlling_type);
                }
                const_wanted = saved_const_wanted;
                loop {
                    learn = 0 as std::os::raw::c_int;
                    skip(',' as i32);
                    if tok == TOK_DEFAULT as std::os::raw::c_int {
                        if has_default != 0 {
                            _tcc_error(
                                b"too many \'default\'\x00" as *const u8
                                    as *const std::os::raw::c_char,
                            );
                        }
                        has_default = 1 as std::os::raw::c_int;
                        if has_match == 0 {
                            learn = 1 as std::os::raw::c_int
                        }
                        next();
                    } else {
                        let mut ad_tmp: AttributeDef = AttributeDef {
                            a: SymAttr([0; 2]),
                            f: FuncAttr([0; 4]),
                            section: 0 as *mut Section,
                            cleanup_func: 0 as *mut Sym,
                            alias_target: 0,
                            asm_label: 0,
                            attr_mode: 0,
                        };
                        let mut itmp: std::os::raw::c_int = 0;
                        let mut cur_type: CType = CType {
                            t: 0,
                            ref_0: 0 as *const Sym as *mut Sym,
                        };
                        in_generic += 1;
                        parse_btype(&mut cur_type, &mut ad_tmp);
                        in_generic -= 1;
                        type_decl(
                            &mut cur_type,
                            &mut ad_tmp,
                            &mut itmp,
                            1 as std::os::raw::c_int,
                        );
                        if compare_types(
                            &mut controlling_type,
                            &mut cur_type,
                            0 as std::os::raw::c_int,
                        ) != 0
                        {
                            if has_match != 0 {
                                _tcc_error(
                                    b"type match twice\x00" as *const u8
                                        as *const std::os::raw::c_char,
                                );
                            }
                            has_match = 1 as std::os::raw::c_int;
                            learn = 1 as std::os::raw::c_int
                        }
                    }
                    skip(':' as i32);
                    if learn != 0 {
                        if !str.is_null() {
                            tok_str_free(str);
                        }
                        skip_or_save_block(&mut str);
                    } else {
                        skip_or_save_block(0 as *mut *mut TokenString);
                    }
                    if tok == ')' as i32 {
                        break;
                    }
                }
                if str.is_null() {
                    let mut buf: [std::os::raw::c_char; 60] = [0; 60];
                    type_to_str(
                        buf.as_mut_ptr(),
                        ::std::mem::size_of::<[std::os::raw::c_char; 60]>() as std::os::raw::c_ulong
                            as std::os::raw::c_int,
                        &mut controlling_type,
                        0 as *const std::os::raw::c_char,
                    );
                    _tcc_error(
                        b"type \'%s\' does not match any association\x00" as *const u8
                            as *const std::os::raw::c_char,
                        buf.as_mut_ptr(),
                    );
                }
                begin_macro(str, 1 as std::os::raw::c_int);
                next();
                expr_eq();
                if tok != -(1 as std::os::raw::c_int) {
                    expect(b",\x00" as *const u8 as *const std::os::raw::c_char);
                }
                end_macro();
                next();
                current_block = 15568404490809570198;
                break;
            },
            340 => {
                // special qnan , snan and infinity values
                n = 0x7fc00000 as std::os::raw::c_int;
                current_block = 4156337385767102043;
                break;
            },
            341 => {
                n = 0x7f800001 as std::os::raw::c_int;
                current_block = 4156337385767102043;
                break;
            },
            342 => {
                n = 0x7f800000 as std::os::raw::c_int;
                current_block = 4156337385767102043;
                break;
            },
            _ => {
                current_block = 9048094288174807049;
                break;
            },
        }
    }
    match current_block {
        9728093949049737828 => {
            next();
            /* allow to take the address of a label */
            if tok < TOK_DEFINE as std::os::raw::c_int {
                expect(b"label identifier\x00" as *const u8 as *const std::os::raw::c_char);
            }
            s = label_find(tok);
            if s.is_null() {
                s = label_push(&mut global_label_stack, tok, 1 as std::os::raw::c_int)
            } else if (*s).r as std::os::raw::c_int == 2 as std::os::raw::c_int {
                (*s).r = 1 as std::os::raw::c_int as std::os::raw::c_ushort
            }
            if (*s).type_0.t == 0 {
                (*s).type_0.t = 0 as std::os::raw::c_int;
                mk_pointer(&mut (*s).type_0);
                (*s).type_0.t |= 0x2000 as std::os::raw::c_int
            }
            vpushsym(&mut (*s).type_0, s);
            next();
        },
        17985719102395905236 => {
            if (*tcc_state).warn_write_strings != 0 {
                t |= 0x100 as std::os::raw::c_int
            }
            type_0.t = t;
            mk_pointer(&mut type_0);
            type_0.t |= 0x40 as std::os::raw::c_int;
            memset(
                &mut ad as *mut AttributeDef as *mut std::os::raw::c_void,
                0 as std::os::raw::c_int,
                ::std::mem::size_of::<AttributeDef>() as std::os::raw::c_ulong,
            );
            ad.section = (*tcc_state).rodata_section;
            decl_initializer_alloc(
                &mut type_0,
                &mut ad,
                0x30 as std::os::raw::c_int,
                2 as std::os::raw::c_int,
                0 as std::os::raw::c_int,
                0 as std::os::raw::c_int,
            );
        },
        4156337385767102043 => {
            vpushi(n);
            (*vtop).type_0.t = 8 as std::os::raw::c_int;
            next();
        },
        9048094288174807049 => {
            t = tok;
            next();
            if t < TOK_DEFINE as std::os::raw::c_int {
                expect(b"identifier\x00" as *const u8 as *const std::os::raw::c_char);
            }
            s = sym_find(t);
            if s.is_null()
                || (*s).type_0.t
                    & (0xf as std::os::raw::c_int
                        | (0 as std::os::raw::c_int
                            | (1 as std::os::raw::c_int) << 20 as std::os::raw::c_int))
                    == 0 as std::os::raw::c_int
                        | (1 as std::os::raw::c_int) << 20 as std::os::raw::c_int
            {
                let mut name: *const std::os::raw::c_char = get_tok_str(t, 0 as *mut CValue);
                if tok != '(' as i32 {
                    _tcc_error(
                        b"\'%s\' undeclared\x00" as *const u8 as *const std::os::raw::c_char,
                        name,
                    );
                }
                /* for simple function calls, we tolerate undeclared
                external reference to int() function */
                if (*tcc_state).warn_implicit_function_declaration != 0 {
                    _tcc_warning(
                        b"implicit declaration of function \'%s\'\x00" as *const u8
                            as *const std::os::raw::c_char,
                        name,
                    );
                }
                s = external_global_sym(t, &mut func_old_type)
            }
            r = (*s).r as std::os::raw::c_int;
            /* A symbol that has a register is a local register variable,
            which starts out as VT_LOCAL value.  */
            if (r & 0x3f as std::os::raw::c_int) < 0x30 as std::os::raw::c_int {
                r = r & !(0x3f as std::os::raw::c_int) | 0x32 as std::os::raw::c_int
            }
            vset(&mut (*s).type_0, r, (*s).c2rust_unnamed.c2rust_unnamed.c);
            /* Point to s as backpointer (even without r&VT_SYM).
            Will be used by at least the x86 inline asm parser for
            regvars.  */
            (*vtop).c2rust_unnamed_0.sym = s;
            if r & 0x200 as std::os::raw::c_int != 0 {
                (*vtop).c2rust_unnamed.c.i = 0 as std::os::raw::c_int as uint64_t
            } else if r == 0x30 as std::os::raw::c_int
                && (*s).type_0.t as std::os::raw::c_uint
                    & (((1 as std::os::raw::c_uint)
                        << 6 as std::os::raw::c_int + 6 as std::os::raw::c_int)
                        .wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_uint)
                        << 20 as std::os::raw::c_int
                        | 0x80 as std::os::raw::c_int as std::os::raw::c_uint)
                    == ((3 as std::os::raw::c_int) << 20 as std::os::raw::c_int)
                        as std::os::raw::c_uint
            {
                (*vtop).c2rust_unnamed.c.i = (*s).c2rust_unnamed.enum_val as uint64_t
            }
        },
        9632834660368191687 => {
            type_0.t = t;
            vsetc(&mut type_0, 0x30 as std::os::raw::c_int, &mut tokc);
            next();
        },
        10089449838879554637 =>
        /* fall thru */
        {
            let mut sec: *mut Section = 0 as *mut Section;
            let mut ptr: *mut std::os::raw::c_void = 0 as *mut std::os::raw::c_void;
            let mut len: std::os::raw::c_int = 0;
            /* special function name identifier */
            len = strlen(funcname).wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong)
                as std::os::raw::c_int;
            /* generate char[len] type */
            type_0.t = 1 as std::os::raw::c_int;
            if (*tcc_state).warn_write_strings != 0 {
                type_0.t |= 0x100 as std::os::raw::c_int
            }
            mk_pointer(&mut type_0);
            type_0.t |= 0x40 as std::os::raw::c_int;
            (*type_0.ref_0).c2rust_unnamed.c2rust_unnamed.c = len;
            sec = (*tcc_state).rodata_section;
            vpush_ref(
                &mut type_0,
                sec,
                (*sec).data_offset,
                len as std::os::raw::c_ulong,
            );
            if !(nocode_wanted > 0 as std::os::raw::c_int) {
                ptr = section_ptr_add(sec, len as Elf64_Addr);
                memcpy(
                    ptr,
                    funcname as *const std::os::raw::c_void,
                    len as std::os::raw::c_ulong,
                );
            }
            next();
        }
        _ => {},
    }
    loop
    /* post operations */
    {
        if tok == 0x82 as std::os::raw::c_int || tok == 0x80 as std::os::raw::c_int {
            inc(1 as std::os::raw::c_int, tok);
            next();
        } else if tok == '.' as i32
            || tok == 0xa0 as std::os::raw::c_int
            || tok == 0xcb as std::os::raw::c_int
        {
            let mut qualifiers: std::os::raw::c_int = 0;
            let mut cumofs: std::os::raw::c_int = 0 as std::os::raw::c_int;
            /* field */
            if tok == 0xa0 as std::os::raw::c_int {
                indir();
            }
            qualifiers =
                (*vtop).type_0.t & (0x100 as std::os::raw::c_int | 0x200 as std::os::raw::c_int);
            test_lvalue();
            gaddrof();
            /* expect pointer on structure */
            if (*vtop).type_0.t & 0xf as std::os::raw::c_int != 7 as std::os::raw::c_int {
                expect(b"struct or union\x00" as *const u8 as *const std::os::raw::c_char);
            }
            if tok == 0xcb as std::os::raw::c_int {
                expect(b"field name\x00" as *const u8 as *const std::os::raw::c_char);
            }
            next();
            if tok == 0xc2 as std::os::raw::c_int || tok == 0xc3 as std::os::raw::c_int {
                expect(b"field name\x00" as *const u8 as *const std::os::raw::c_char);
            }
            s = find_field(&mut (*vtop).type_0, tok, &mut cumofs);
            if s.is_null() {
                _tcc_error(
                    b"field not found: %s\x00" as *const u8 as *const std::os::raw::c_char,
                    get_tok_str(tok & !(0x20000000 as std::os::raw::c_int), &mut tokc),
                );
            }
            /* add field offset to pointer */
            (*vtop).type_0 = char_pointer_type; /* change type to 'char *' */
            vpushi(cumofs + (*s).c2rust_unnamed.c2rust_unnamed.c);
            gen_op('+' as i32);
            /* change type to field type, and set to lvalue */
            (*vtop).type_0 = (*s).type_0;
            (*vtop).type_0.t |= qualifiers;
            /* an array is never an lvalue */
            if (*vtop).type_0.t & 0x40 as std::os::raw::c_int == 0 {
                (*vtop).r = ((*vtop).r as std::os::raw::c_int | 0x100 as std::os::raw::c_int)
                    as std::os::raw::c_ushort;
                /* if bound checking, the referenced pointer must be checked */
                if (*tcc_state).do_bounds_check != 0 {
                    (*vtop).r = ((*vtop).r as std::os::raw::c_int | 0x4000 as std::os::raw::c_int)
                        as std::os::raw::c_ushort
                }
            }
            next();
        } else if tok == '[' as i32 {
            next();
            gexpr();
            gen_op('+' as i32);
            indir();
            skip(']' as i32);
        } else {
            if !(tok == '(' as i32) {
                break;
            }
            let mut ret: SValue = SValue {
                type_0: CType {
                    t: 0,
                    ref_0: 0 as *const Sym as *mut Sym,
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
            let mut sa: *mut Sym = 0 as *mut Sym;
            let mut nb_args: std::os::raw::c_int = 0;
            let mut ret_nregs: std::os::raw::c_int = 0;
            let mut ret_align: std::os::raw::c_int = 0;
            let mut regsize: std::os::raw::c_int = 0;
            let mut variadic: std::os::raw::c_int = 0;
            /* function call  */
            if (*vtop).type_0.t & 0xf as std::os::raw::c_int != 6 as std::os::raw::c_int {
                's_1756: {
                    /* pointer test (no array accepted) */
                    if (*vtop).type_0.t & (0xf as std::os::raw::c_int | 0x40 as std::os::raw::c_int)
                        == 5 as std::os::raw::c_int
                    {
                        (*vtop).type_0 = *pointed_type(&mut (*vtop).type_0);
                        if !((*vtop).type_0.t & 0xf as std::os::raw::c_int
                            != 6 as std::os::raw::c_int)
                        {
                            break 's_1756;
                        }
                    }
                    expect(b"function pointer\x00" as *const u8 as *const std::os::raw::c_char);
                }
            } else {
                (*vtop).r = ((*vtop).r as std::os::raw::c_int & !(0x100 as std::os::raw::c_int))
                    as std::os::raw::c_ushort
                /* no lvalue */
            }
            /* get return type */
            s = (*vtop).type_0.ref_0; /* first parameter */
            next();
            sa = (*s).c2rust_unnamed_0.next;
            regsize = 0 as std::os::raw::c_int;
            nb_args = regsize;
            ret.r2 = 0x30 as std::os::raw::c_int as std::os::raw::c_ushort;
            /* compute first implicit argument if a structure is returned */
            if (*s).type_0.t & 0xf as std::os::raw::c_int == 7 as std::os::raw::c_int {
                variadic = ((*s)
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .f
                    .func_type() as std::os::raw::c_int
                    == 3 as std::os::raw::c_int) as std::os::raw::c_int;
                ret_nregs = gfunc_sret(
                    &mut (*s).type_0,
                    variadic,
                    &mut ret.type_0,
                    &mut ret_align,
                    &mut regsize,
                );
                if ret_nregs <= 0 as std::os::raw::c_int {
                    /* get some space for the returned structure */
                    size = type_size(&mut (*s).type_0, &mut align);
                    loc = loc - size & -align;
                    ret.type_0 = (*s).type_0;
                    ret.r = (0x32 as std::os::raw::c_int | 0x100 as std::os::raw::c_int)
                        as std::os::raw::c_ushort;
                    /* pass it as 'int' to avoid structure arg passing
                    problems */
                    vseti(0x32 as std::os::raw::c_int, loc);
                    if (*tcc_state).do_bounds_check != 0 {
                        loc -= 1
                    }
                    ret.c2rust_unnamed.c = (*vtop).c2rust_unnamed.c;
                    if ret_nregs < 0 as std::os::raw::c_int {
                        vtop = vtop.offset(-1)
                    } else {
                        nb_args += 1
                    }
                }
            } else {
                ret_nregs = 1 as std::os::raw::c_int;
                ret.type_0 = (*s).type_0
            }
            if ret_nregs > 0 as std::os::raw::c_int {
                /* return in register */
                ret.c2rust_unnamed.c.i = 0 as std::os::raw::c_int as uint64_t;
                PUT_R_RET(&mut ret, ret.type_0.t);
            }
            if tok != ')' as i32 {
                loop {
                    expr_eq();
                    gfunc_param_typed(s, sa);
                    nb_args += 1;
                    if !sa.is_null() {
                        sa = (*sa).c2rust_unnamed_0.next
                    }
                    if tok == ')' as i32 {
                        break;
                    }
                    skip(',' as i32);
                }
            }
            if !sa.is_null() {
                _tcc_error(
                    b"too few arguments to function\x00" as *const u8
                        as *const std::os::raw::c_char,
                );
            }
            skip(')' as i32);
            gfunc_call(nb_args);
            if ret_nregs < 0 as std::os::raw::c_int {
                vsetc(
                    &mut ret.type_0,
                    ret.r as std::os::raw::c_int,
                    &mut ret.c2rust_unnamed.c,
                );
            } else {
                /* return value */
                r = ret.r as std::os::raw::c_int
                    + ret_nregs
                    + (ret_nregs == 0) as std::os::raw::c_int;
                loop {
                    let fresh9 = r;
                    r = r - 1;
                    if !(fresh9 > ret.r as std::os::raw::c_int) {
                        break;
                    }
                    vsetc(&mut ret.type_0, r, &mut ret.c2rust_unnamed.c);
                    (*vtop).r2 = ret.r2
                    /* Loop only happens when r2 is VT_CONST */
                }
                /* handle packed struct return */
                if (*s).type_0.t & 0xf as std::os::raw::c_int == 7 as std::os::raw::c_int
                    && ret_nregs != 0
                {
                    let mut addr: std::os::raw::c_int = 0;
                    let mut offset: std::os::raw::c_int = 0;
                    size = type_size(&mut (*s).type_0, &mut align);
                    /* We're writing whole regs often, make sure there's enough
                    space.  Assume register size is power of 2.  */
                    if regsize > align {
                        align = regsize
                    }
                    loc = loc - size & -align;
                    addr = loc;
                    offset = 0 as std::os::raw::c_int;
                    loop {
                        vset(
                            &mut ret.type_0,
                            0x32 as std::os::raw::c_int | 0x100 as std::os::raw::c_int,
                            addr + offset,
                        );
                        vswap();
                        vstore();
                        vtop = vtop.offset(-1);
                        ret_nregs -= 1;
                        if ret_nregs == 0 as std::os::raw::c_int {
                            break;
                        }
                        offset += regsize
                    }
                    vset(
                        &mut (*s).type_0,
                        0x32 as std::os::raw::c_int | 0x100 as std::os::raw::c_int,
                        addr,
                    );
                }
                /* Promote char/short return values. This is matters only
                for calling function that were not compiled by TCC and
                only on some architectures.  For those where it doesn't
                matter we expect things to be already promoted to int,
                but not larger.  */
                t = (*s).type_0.t & 0xf as std::os::raw::c_int;
                if t == 1 as std::os::raw::c_int
                    || t == 2 as std::os::raw::c_int
                    || t == 11 as std::os::raw::c_int
                {
                    (*vtop).r = ((*vtop).r as std::os::raw::c_uint
                        | ((0xc00 as std::os::raw::c_int
                            & !((0xc00 as std::os::raw::c_int) << 1 as std::os::raw::c_int))
                            as std::os::raw::c_uint)
                            .wrapping_mul(1 as std::os::raw::c_int as std::os::raw::c_uint))
                        as std::os::raw::c_ushort
                }
            }
            if (*s)
                .c2rust_unnamed
                .c2rust_unnamed
                .c2rust_unnamed
                .f
                .func_noreturn()
                != 0
            {
                if debug_modes != 0 {
                    tcc_tcov_block_end(tcov_data.line);
                }
                nocode_wanted |= 0x20000000 as std::os::raw::c_int
            }
        }
    }
}
/* original top-down parser */
/* defined precedence_parser */
unsafe extern "C" fn precedence(mut tok_0: std::os::raw::c_int) -> std::os::raw::c_int {
    match tok_0 {
        145 => return 1 as std::os::raw::c_int,
        144 => return 2 as std::os::raw::c_int,
        124 => return 3 as std::os::raw::c_int,
        94 => return 4 as std::os::raw::c_int,
        38 => return 5 as std::os::raw::c_int,
        148 | 149 => return 6 as std::os::raw::c_int,
        146 | 147 => {},
        60 | 62 => return 8 as std::os::raw::c_int,
        43 | 45 => return 9 as std::os::raw::c_int,
        42 | 47 | 37 => return 10 as std::os::raw::c_int,
        _ => {
            if !(tok_0 >= 0x96 as std::os::raw::c_int && tok_0 <= 0x9f as std::os::raw::c_int) {
                return 0 as std::os::raw::c_int;
            }
        },
    }
    return 7 as std::os::raw::c_int;
}
static mut prec: [std::os::raw::c_uchar; 256] = [0; 256];
/* *******************************************************/
unsafe extern "C" fn init_prec() {
    let mut i: std::os::raw::c_int = 0;
    i = 0 as std::os::raw::c_int;
    while i < 256 as std::os::raw::c_int {
        prec[i as usize] = precedence(i) as std::os::raw::c_uchar;
        i += 1
    }
}
unsafe extern "C" fn expr_infix(mut p: std::os::raw::c_int) {
    let mut t: std::os::raw::c_int = tok;
    let mut p2: std::os::raw::c_int = 0;
    loop {
        p2 = (if (t as std::os::raw::c_uint) < 256 as std::os::raw::c_int as std::os::raw::c_uint {
            prec[t as usize] as std::os::raw::c_int
        } else {
            0 as std::os::raw::c_int
        });
        if !(p2 >= p) {
            break;
        }
        if t == 0x91 as std::os::raw::c_int || t == 0x90 as std::os::raw::c_int {
            expr_landor(t);
        } else {
            next();
            unary();
            if (if (tok as std::os::raw::c_uint)
                < 256 as std::os::raw::c_int as std::os::raw::c_uint
            {
                prec[tok as usize] as std::os::raw::c_int
            } else {
                0 as std::os::raw::c_int
            }) > p2
            {
                expr_infix(p2 + 1 as std::os::raw::c_int);
            }
            gen_op(t);
        }
        t = tok
    }
}
/* Assuming vtop is a value used in a conditional context
(i.e. compared with zero) return 0 if it's false, 1 if
true and -1 if it can't be statically determined.  */
unsafe extern "C" fn condition_3way() -> std::os::raw::c_int {
    let mut c: std::os::raw::c_int = -(1 as std::os::raw::c_int);
    if (*vtop).r as std::os::raw::c_int
        & (0x3f as std::os::raw::c_int | 0x100 as std::os::raw::c_int)
        == 0x30 as std::os::raw::c_int
        && ((*vtop).r as std::os::raw::c_int & 0x200 as std::os::raw::c_int == 0
            || (*(*vtop).c2rust_unnamed_0.sym).a.weak() == 0)
    {
        vdup();
        gen_cast_s(11 as std::os::raw::c_int);
        c = (*vtop).c2rust_unnamed.c.i as std::os::raw::c_int;
        vpop();
    }
    return c;
}
unsafe extern "C" fn expr_landor(mut op: std::os::raw::c_int) {
    let mut t: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut cc: std::os::raw::c_int = 1 as std::os::raw::c_int;
    let mut f: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut i: std::os::raw::c_int = (op == 0x90 as std::os::raw::c_int) as std::os::raw::c_int;
    let mut c: std::os::raw::c_int = 0;
    loop {
        c = if f != 0 { i } else { condition_3way() };
        if c < 0 as std::os::raw::c_int {
            save_regs(1 as std::os::raw::c_int);
            cc = 0 as std::os::raw::c_int
        } else if c != i {
            nocode_wanted += 1;
            f = 1 as std::os::raw::c_int
        }
        if tok != op {
            break;
        }
        if c < 0 as std::os::raw::c_int {
            t = gvtst(i, t)
        } else {
            vpop();
        }
        next();
        unary();
        expr_infix(
            (if (op as std::os::raw::c_uint) < 256 as std::os::raw::c_int as std::os::raw::c_uint {
                prec[op as usize] as std::os::raw::c_int
            } else {
                0 as std::os::raw::c_int
            }) + 1 as std::os::raw::c_int,
        );
    }
    if cc != 0 || f != 0 {
        vpop();
        vpushi(i ^ f);
        gsym(t);
        nocode_wanted -= f
    } else {
        gvtst_set(i, t);
    };
}
unsafe extern "C" fn is_cond_bool(mut sv: *mut SValue) -> std::os::raw::c_int {
    if (*sv).r as std::os::raw::c_int
        & (0x3f as std::os::raw::c_int
            | 0x100 as std::os::raw::c_int
            | 0x200 as std::os::raw::c_int)
        == 0x30 as std::os::raw::c_int
        && (*sv).type_0.t & 0xf as std::os::raw::c_int == 3 as std::os::raw::c_int
    {
        return (((*sv).c2rust_unnamed.c.i as std::os::raw::c_uint)
            < 2 as std::os::raw::c_int as std::os::raw::c_uint)
            as std::os::raw::c_int;
    }
    if (*sv).r as std::os::raw::c_int == 0x33 as std::os::raw::c_int {
        return 1 as std::os::raw::c_int;
    }
    return 0 as std::os::raw::c_int;
}
unsafe extern "C" fn expr_cond() {
    let mut tt: std::os::raw::c_int = 0;
    let mut u: std::os::raw::c_int = 0;
    let mut r1: std::os::raw::c_int = 0;
    let mut r2: std::os::raw::c_int = 0;
    let mut rc: std::os::raw::c_int = 0;
    let mut t1: std::os::raw::c_int = 0;
    let mut t2: std::os::raw::c_int = 0;
    let mut islv: std::os::raw::c_int = 0;
    let mut c: std::os::raw::c_int = 0;
    let mut g: std::os::raw::c_int = 0;
    let mut sv: SValue = SValue {
        type_0: CType {
            t: 0,
            ref_0: 0 as *const Sym as *mut Sym,
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
    let mut type_0: CType = CType {
        t: 0,
        ref_0: 0 as *const Sym as *mut Sym,
    };
    let mut ncw_prev: std::os::raw::c_int = 0;
    unary();
    expr_infix(1 as std::os::raw::c_int);
    if tok == '?' as i32 {
        next();
        c = condition_3way();
        g = (tok == ':' as i32 && (*tcc_state).gnu_ext as std::os::raw::c_int != 0)
            as std::os::raw::c_int;
        tt = 0 as std::os::raw::c_int;
        if g == 0 {
            if c < 0 as std::os::raw::c_int {
                save_regs(1 as std::os::raw::c_int);
                tt = gvtst(1 as std::os::raw::c_int, 0 as std::os::raw::c_int)
            } else {
                vpop();
            }
        } else if c < 0 as std::os::raw::c_int {
            /* needed to avoid having different registers saved in
            each branch */
            save_regs(1 as std::os::raw::c_int); /* save value to handle it later */
            gv_dup(); /* no vpop so that FP stack is not flushed */
            tt = gvtst(0 as std::os::raw::c_int, 0 as std::os::raw::c_int)
        }
        ncw_prev = nocode_wanted;
        if c == 0 as std::os::raw::c_int {
            nocode_wanted += 1
        }
        if g == 0 {
            gexpr();
        }
        if (*vtop).type_0.t & 0xf as std::os::raw::c_int == 6 as std::os::raw::c_int {
            mk_pointer(&mut (*vtop).type_0);
        }
        sv = *vtop;
        vtop = vtop.offset(-1);
        if g != 0 {
            u = tt
        } else if c < 0 as std::os::raw::c_int {
            u = gjmp_acs(0 as std::os::raw::c_int);
            gsym(tt);
        } else {
            u = 0 as std::os::raw::c_int
        }
        nocode_wanted = ncw_prev;
        if c == 1 as std::os::raw::c_int {
            nocode_wanted += 1
        }
        skip(':' as i32);
        expr_cond();
        if c < 0 as std::os::raw::c_int && is_cond_bool(vtop) != 0 && is_cond_bool(&mut sv) != 0 {
            /* optimize "if (f ? a > b : c || d) ..." for example, where normally
            "a < b" and "c || d" would be forced to "(int)0/1" first, whereas
            this code jumps directly to the if's then/else branches. */
            t1 = gvtst(0 as std::os::raw::c_int, 0 as std::os::raw::c_int);
            t2 = gjmp_acs(0 as std::os::raw::c_int);
            gsym(u);
            vpushv(&mut sv);
            /* combine jump targets of 2nd op with VT_CMP of 1st op */
            gvtst_set(0 as std::os::raw::c_int, t1);
            gvtst_set(1 as std::os::raw::c_int, t2);
            nocode_wanted = ncw_prev;
            //  tcc_warning("two conditions expr_cond");
            return;
        }
        if (*vtop).type_0.t & 0xf as std::os::raw::c_int == 6 as std::os::raw::c_int {
            mk_pointer(&mut (*vtop).type_0);
        }
        /* cast operands to correct type according to ISOC rules */
        if combine_types(&mut type_0, &mut sv, vtop, '?' as i32) == 0 {
            type_incompatibility_error(
                &mut sv.type_0,
                &mut (*vtop).type_0,
                b"type mismatch in conditional expression (have \'%s\' and \'%s\')\x00" as *const u8
                    as *const std::os::raw::c_char,
            );
        }
        /* keep structs lvalue by transforming `(expr ? a : b)` to `*(expr ? &a : &b)` so
        that `(expr ? a : b).mem` does not error  with "lvalue expected" */
        islv = ((*vtop).r as std::os::raw::c_int & 0x100 as std::os::raw::c_int != 0
            && sv.r as std::os::raw::c_int & 0x100 as std::os::raw::c_int != 0
            && 7 as std::os::raw::c_int == type_0.t & 0xf as std::os::raw::c_int)
            as std::os::raw::c_int;
        /* now we convert second operand */
        if c != 1 as std::os::raw::c_int {
            gen_cast(&mut type_0);
            if islv != 0 {
                mk_pointer(&mut (*vtop).type_0);
                gaddrof();
            } else if 7 as std::os::raw::c_int == (*vtop).type_0.t & 0xf as std::os::raw::c_int {
                gaddrof();
            }
        }
        rc = RC_TYPE(type_0.t);
        /* for long longs, we use fixed registers to avoid having
        to handle a complicated move */
        if R2_RET(type_0.t) != 0x30 as std::os::raw::c_int {
            rc = RC_RET(type_0.t)
        }
        r2 = 0 as std::os::raw::c_int;
        tt = r2;
        if c < 0 as std::os::raw::c_int {
            r2 = gv(rc);
            tt = gjmp_acs(0 as std::os::raw::c_int)
        }
        gsym(u);
        nocode_wanted = ncw_prev;
        /* this is horrible, but we must also convert first
        operand */
        if c != 0 as std::os::raw::c_int {
            *vtop = sv;
            gen_cast(&mut type_0);
            if islv != 0 {
                mk_pointer(&mut (*vtop).type_0);
                gaddrof();
            } else if 7 as std::os::raw::c_int == (*vtop).type_0.t & 0xf as std::os::raw::c_int {
                gaddrof();
            }
        }
        if c < 0 as std::os::raw::c_int {
            r1 = gv(rc);
            move_reg(
                r2,
                r1,
                if islv != 0 {
                    5 as std::os::raw::c_int
                } else {
                    type_0.t
                },
            );
            (*vtop).r = r2 as std::os::raw::c_ushort;
            gsym(tt);
        }
        if islv != 0 {
            indir();
        }
    };
}
unsafe extern "C" fn expr_eq() {
    let mut t: std::os::raw::c_int = 0;
    expr_cond();
    t = tok;
    if t == '=' as i32 || t >= 0xb0 as std::os::raw::c_int && t <= 0xb9 as std::os::raw::c_int {
        test_lvalue();
        next();
        if t == '=' as i32 {
            expr_eq();
        } else {
            vdup();
            expr_eq();
            gen_op(
                (*::std::mem::transmute::<&[u8; 11], &[std::os::raw::c_char; 11]>(
                    b"+-*/%&|^<>\x00",
                ))[(t - 0xb0 as std::os::raw::c_int) as usize]
                    as std::os::raw::c_int,
            );
        }
        vstore();
    };
}
#[no_mangle]
pub unsafe extern "C" fn gexpr() {
    loop {
        expr_eq();
        if tok != ',' as i32 {
            break;
        }
        vpop();
        next();
    }
}
/* parse a constant expression and return value in vtop.  */
unsafe extern "C" fn expr_const1() {
    const_wanted += 1;
    nocode_wanted += 0xffff as std::os::raw::c_int + 1 as std::os::raw::c_int;
    expr_cond();
    nocode_wanted -= 0xffff as std::os::raw::c_int + 1 as std::os::raw::c_int;
    const_wanted -= 1;
}
/* parse an integer constant and return its value. */
#[inline]
unsafe extern "C" fn expr_const64() -> int64_t {
    let mut c: int64_t = 0;
    expr_const1();
    if (*vtop).r as std::os::raw::c_int
        & (0x3f as std::os::raw::c_int
            | 0x100 as std::os::raw::c_int
            | 0x200 as std::os::raw::c_int)
        != 0x30 as std::os::raw::c_int
    {
        expect(b"constant expression\x00" as *const u8 as *const std::os::raw::c_char);
    }
    c = (*vtop).c2rust_unnamed.c.i as int64_t;
    vpop();
    return c;
}
/* parse an integer constant and return its value.
Complain if it doesn't fit 32bit (signed or unsigned).  */
#[no_mangle]
pub unsafe extern "C" fn expr_const() -> std::os::raw::c_int {
    let mut c: std::os::raw::c_int = 0;
    let mut wc: int64_t = expr_const64();
    c = wc as std::os::raw::c_int;
    if c as std::os::raw::c_long != wc && c as std::os::raw::c_uint as std::os::raw::c_long != wc {
        _tcc_error(b"constant exceeds 32 bit\x00" as *const u8 as *const std::os::raw::c_char);
    }
    return c;
}
/* ------------------------------------------------------------------------- */
/* return from function */
unsafe extern "C" fn gfunc_return(mut func_type: *mut CType) {
    if (*func_type).t & 0xf as std::os::raw::c_int == 7 as std::os::raw::c_int {
        let mut type_0: CType = CType {
            t: 0,
            ref_0: 0 as *const Sym as *mut Sym,
        };
        let mut ret_type: CType = CType {
            t: 0,
            ref_0: 0 as *const Sym as *mut Sym,
        };
        let mut ret_align: std::os::raw::c_int = 0;
        let mut ret_nregs: std::os::raw::c_int = 0;
        let mut regsize: std::os::raw::c_int = 0;
        ret_nregs = gfunc_sret(
            func_type,
            func_var,
            &mut ret_type,
            &mut ret_align,
            &mut regsize,
        );
        if !(ret_nregs < 0 as std::os::raw::c_int) {
            if 0 as std::os::raw::c_int == ret_nregs {
                /* if returning structure, must copy it to implicit
                first pointer arg location */
                type_0 = *func_type;
                mk_pointer(&mut type_0);
                vset(
                    &mut type_0,
                    0x32 as std::os::raw::c_int | 0x100 as std::os::raw::c_int,
                    func_vc,
                );
                indir();
                vswap();
                /* copy structure value to pointer */
                vstore();
            } else {
                /* returning structure packed into registers */
                let mut size: std::os::raw::c_int = 0;
                let mut addr: std::os::raw::c_int = 0;
                let mut align: std::os::raw::c_int = 0;
                let mut rc: std::os::raw::c_int = 0;
                size = type_size(func_type, &mut align);
                if ((*vtop).r as std::os::raw::c_int
                    != 0x32 as std::os::raw::c_int | 0x100 as std::os::raw::c_int
                    || (*vtop).c2rust_unnamed.c.i
                        & (ret_align - 1 as std::os::raw::c_int) as std::os::raw::c_ulong
                        != 0)
                    && align & ret_align - 1 as std::os::raw::c_int != 0
                {
                    loc = loc - size & -ret_align;
                    addr = loc;
                    type_0 = *func_type;
                    vset(
                        &mut type_0,
                        0x32 as std::os::raw::c_int | 0x100 as std::os::raw::c_int,
                        addr,
                    );
                    vswap();
                    vstore();
                    vpop();
                    vset(
                        &mut ret_type,
                        0x32 as std::os::raw::c_int | 0x100 as std::os::raw::c_int,
                        addr,
                    );
                }
                (*vtop).type_0 = ret_type;
                rc = RC_RET(ret_type.t);
                if ret_nregs == 1 as std::os::raw::c_int {
                    gv(rc);
                } else {
                    loop {
                        vdup();
                        gv(rc);
                        vpop();
                        ret_nregs -= 1;
                        if ret_nregs == 0 as std::os::raw::c_int {
                            break;
                        }
                        /* We assume that when a structure is returned in multiple
                        registers, their classes are consecutive values of the
                        suite s(n) = 2^n */
                        rc <<= 1 as std::os::raw::c_int;
                        (*vtop).c2rust_unnamed.c.i =
                            ((*vtop).c2rust_unnamed.c.i as std::os::raw::c_ulong)
                                .wrapping_add(regsize as std::os::raw::c_ulong)
                                as uint64_t as uint64_t
                    }
                }
            }
        }
    } else {
        gv(RC_RET((*func_type).t));
    }
    vtop = vtop.offset(-1);
    /* NOT vpop() because on x86 it would flush the fp stack */
}
unsafe extern "C" fn check_func_return() {
    if func_vt.t & 0xf as std::os::raw::c_int == 0 as std::os::raw::c_int {
        return;
    }
    if strcmp(
        funcname,
        b"main\x00" as *const u8 as *const std::os::raw::c_char,
    ) == 0
        && func_vt.t & 0xf as std::os::raw::c_int == 3 as std::os::raw::c_int
    {
        /* main returns 0 by default */
        vpushi(0 as std::os::raw::c_int);
        gen_assign_cast(&mut func_vt);
        gfunc_return(&mut func_vt);
    } else {
        _tcc_warning(
            b"function might return no value: \'%s\'\x00" as *const u8
                as *const std::os::raw::c_char,
            funcname,
        );
    };
}
/* ------------------------------------------------------------------------- */
/* switch/case */
unsafe extern "C" fn case_cmpi(
    mut pa: *const std::os::raw::c_void,
    mut pb: *const std::os::raw::c_void,
) -> std::os::raw::c_int {
    let mut a: int64_t = (**(pa as *mut *mut case_t)).v1;
    let mut b: int64_t = (**(pb as *mut *mut case_t)).v1;
    return if a < b {
        -(1 as std::os::raw::c_int)
    } else {
        (a > b) as std::os::raw::c_int
    };
}
unsafe extern "C" fn case_cmpu(
    mut pa: *const std::os::raw::c_void,
    mut pb: *const std::os::raw::c_void,
) -> std::os::raw::c_int {
    let mut a: uint64_t = (**(pa as *mut *mut case_t)).v1 as uint64_t;
    let mut b: uint64_t = (**(pb as *mut *mut case_t)).v1 as uint64_t;
    return if a < b {
        -(1 as std::os::raw::c_int)
    } else {
        (a > b) as std::os::raw::c_int
    };
}
unsafe extern "C" fn gtst_addr(mut t: std::os::raw::c_int, mut a: std::os::raw::c_int) {
    gsym_addr(gvtst(0 as std::os::raw::c_int, t), a);
}
unsafe extern "C" fn gcase(
    mut base: *mut *mut case_t,
    mut len: std::os::raw::c_int,
    mut bsym: *mut std::os::raw::c_int,
) {
    let mut p: *mut case_t = 0 as *mut case_t;
    let mut e: std::os::raw::c_int = 0;
    let mut ll: std::os::raw::c_int = ((*vtop).type_0.t & 0xf as std::os::raw::c_int
        == 4 as std::os::raw::c_int) as std::os::raw::c_int;
    while len > 8 as std::os::raw::c_int {
        /* binary search */
        p = *base.offset((len / 2 as std::os::raw::c_int) as isize); /* v1 <= x <= v2 */
        vdup();
        if ll != 0 {
            vpushll((*p).v2 as std::os::raw::c_longlong);
        } else {
            vpushi((*p).v2 as std::os::raw::c_int);
        }
        gen_op(0x9e as std::os::raw::c_int);
        e = gvtst(1 as std::os::raw::c_int, 0 as std::os::raw::c_int);
        vdup();
        if ll != 0 {
            vpushll((*p).v1 as std::os::raw::c_longlong);
        } else {
            vpushi((*p).v1 as std::os::raw::c_int);
        }
        gen_op(0x9d as std::os::raw::c_int);
        gtst_addr(0 as std::os::raw::c_int, (*p).sym);
        /* x < v1 */
        gcase(base, len / 2 as std::os::raw::c_int, bsym);
        /* x > v2 */
        gsym(e);
        e = len / 2 as std::os::raw::c_int + 1 as std::os::raw::c_int;
        base = base.offset(e as isize);
        len -= e
    }
    loop
    /* linear scan */
    {
        let fresh10 = len;
        len = len - 1;
        if !(fresh10 != 0) {
            break;
        }
        let fresh11 = base;
        base = base.offset(1);
        p = *fresh11;
        vdup();
        if ll != 0 {
            vpushll((*p).v2 as std::os::raw::c_longlong);
        } else {
            vpushi((*p).v2 as std::os::raw::c_int);
        }
        if (*p).v1 == (*p).v2 {
            gen_op(0x94 as std::os::raw::c_int);
            gtst_addr(0 as std::os::raw::c_int, (*p).sym);
        } else {
            gen_op(0x9e as std::os::raw::c_int);
            e = gvtst(1 as std::os::raw::c_int, 0 as std::os::raw::c_int);
            vdup();
            if ll != 0 {
                vpushll((*p).v1 as std::os::raw::c_longlong);
            } else {
                vpushi((*p).v1 as std::os::raw::c_int);
            }
            gen_op(0x9d as std::os::raw::c_int);
            gtst_addr(0 as std::os::raw::c_int, (*p).sym);
            gsym(e);
        }
    }
    *bsym = gjmp_acs(*bsym);
}
/* ------------------------------------------------------------------------- */
/* __attribute__((cleanup(fn))) */
unsafe extern "C" fn try_call_scope_cleanup(mut stop: *mut Sym) {
    let mut cls: *mut Sym = (*cur_scope).cl.s;
    while cls != stop {
        let mut fs: *mut Sym = (*cls).c2rust_unnamed_0.next;
        let mut vs: *mut Sym = (*cls).prev_tok;
        vpushsym(&mut (*fs).type_0, fs);
        vset(
            &mut (*vs).type_0,
            (*vs).r as std::os::raw::c_int,
            (*vs).c2rust_unnamed.c2rust_unnamed.c,
        );
        (*vtop).c2rust_unnamed_0.sym = vs;
        mk_pointer(&mut (*vtop).type_0);
        gaddrof();
        gfunc_call(1 as std::os::raw::c_int);
        cls = (*cls).c2rust_unnamed.ncl
    }
}
unsafe extern "C" fn try_call_cleanup_goto(mut cleanupstate: *mut Sym) {
    let mut oc: *mut Sym = 0 as *mut Sym;
    let mut cc: *mut Sym = 0 as *mut Sym;
    let mut ocd: std::os::raw::c_int = 0;
    let mut ccd: std::os::raw::c_int = 0;
    if (*cur_scope).cl.s.is_null() {
        return;
    }
    /* search NCA of both cleanup chains given parents and initial depth */
    ocd = if !cleanupstate.is_null() {
        ((*cleanupstate).v) & !(0x20000000 as std::os::raw::c_int)
    } else {
        0 as std::os::raw::c_int
    };
    ccd = (*cur_scope).cl.n;
    oc = cleanupstate;
    while ocd > ccd {
        ocd -= 1;
        oc = (*oc).c2rust_unnamed.ncl
    }
    cc = (*cur_scope).cl.s;
    while ccd > ocd {
        ccd -= 1;
        cc = (*cc).c2rust_unnamed.ncl
    }
    while cc != oc {
        cc = (*cc).c2rust_unnamed.ncl;
        oc = (*oc).c2rust_unnamed.ncl;
        ccd -= 1
    }
    try_call_scope_cleanup(cc);
}
/* call 'func' for each __attribute__((cleanup(func))) */
unsafe extern "C" fn block_cleanup(mut o_0: *mut scope) {
    let mut jmp: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut g: *mut Sym = 0 as *mut Sym;
    let mut pg: *mut *mut Sym = 0 as *mut *mut Sym;
    pg = &mut pending_gotos;
    loop {
        g = *pg;
        if !(!g.is_null() && (*g).c2rust_unnamed.c2rust_unnamed.c > (*o_0).cl.n) {
            break;
        }
        let mut current_block_9: u64;
        if (*(*g).prev_tok).r as std::os::raw::c_int & 1 as std::os::raw::c_int != 0 {
            let mut pcl: *mut Sym = (*g).c2rust_unnamed_0.next;
            if jmp == 0 {
                jmp = gjmp_acs(0 as std::os::raw::c_int)
            }
            gsym((*pcl).c2rust_unnamed.c2rust_unnamed.c2rust_unnamed.jnext);
            try_call_scope_cleanup((*o_0).cl.s);
            (*pcl).c2rust_unnamed.c2rust_unnamed.c2rust_unnamed.jnext =
                gjmp_acs(0 as std::os::raw::c_int);
            if (*o_0).cl.n == 0 {
                current_block_9 = 10450825522478504889;
            } else {
                (*g).c2rust_unnamed.c2rust_unnamed.c = (*o_0).cl.n;
                pg = &mut (*g).prev;
                current_block_9 = 8236137900636309791;
            }
        } else {
            current_block_9 = 10450825522478504889;
        }
        match current_block_9 {
            10450825522478504889 => {
                *pg = (*g).prev;
                sym_free(g);
            },
            _ => {},
        }
    }
    gsym(jmp);
    try_call_scope_cleanup((*o_0).cl.s);
}
/* ------------------------------------------------------------------------- */
/* VLA */
unsafe extern "C" fn vla_restore(mut loc_0: std::os::raw::c_int) {
    if loc_0 != 0 {
        gen_vla_sp_restore(loc_0);
    };
}
unsafe extern "C" fn vla_leave(mut o_0: *mut scope) {
    let mut c: *mut scope = cur_scope;
    let mut v: *mut scope = 0 as *mut scope;
    while c != o_0 && !c.is_null() {
        if (*c).vla.num != 0 {
            v = c
        }
        c = (*c).prev
    }
    if !v.is_null() {
        vla_restore((*v).vla.locorig);
    };
}
/* ------------------------------------------------------------------------- */
/* local scopes */
#[no_mangle]
pub unsafe extern "C" fn new_scope(mut o_0: *mut scope) {
    /* copy and link previous scope */
    *o_0 = *cur_scope;
    (*o_0).prev = cur_scope;
    cur_scope = o_0;
    (*cur_scope).vla.num = 0 as std::os::raw::c_int;
    /* record local declaration stack position */
    (*o_0).lstk = local_stack;
    (*o_0).llstk = local_label_stack;
    local_scope += 1;
    if debug_modes != 0 {
        tcc_debug_stabn(tcc_state, N_LBRAC as std::os::raw::c_int, ind - func_ind);
    };
}
#[no_mangle]
pub unsafe extern "C" fn prev_scope(mut o_0: *mut scope, mut is_expr: std::os::raw::c_int) {
    vla_leave((*o_0).prev);
    if (*o_0).cl.s != (*(*o_0).prev).cl.s {
        block_cleanup((*o_0).prev);
    }
    /* pop locally defined labels */
    label_pop(&mut local_label_stack, (*o_0).llstk, is_expr);
    /* In the is_expr case (a statement expression is finished here),
    vtop might refer to symbols on the local_stack.  Either via the
    type or via vtop->sym.  We can't pop those nor any that in turn
    might be referred to.  To make it easier we don't roll back
    any symbols in that case; some upper level call to block() will
    do that.  We do have to remove such symbols from the lookup
    tables, though.  sym_pop will do that.  */
    /* pop locally defined symbols */
    pop_local_syms((*o_0).lstk, is_expr);
    cur_scope = (*o_0).prev;
    local_scope -= 1;
    if debug_modes != 0 {
        tcc_debug_stabn(tcc_state, N_RBRAC as std::os::raw::c_int, ind - func_ind);
    };
}
/* leave a scope via break/continue(/goto) */
#[no_mangle]
pub unsafe extern "C" fn leave_scope(mut o_0: *mut scope) {
    if o_0.is_null() {
        return;
    }
    try_call_scope_cleanup((*o_0).cl.s);
    vla_leave(o_0);
}
/* ------------------------------------------------------------------------- */
/* call block from 'for do while' loops */
unsafe extern "C" fn lblock(
    mut bsym: *mut std::os::raw::c_int,
    mut csym: *mut std::os::raw::c_int,
) {
    let mut lo: *mut scope = loop_scope;
    let mut co: *mut scope = cur_scope;
    let mut b: *mut std::os::raw::c_int = (*co).bsym;
    let mut c: *mut std::os::raw::c_int = (*co).csym;
    if !csym.is_null() {
        (*co).csym = csym;
        loop_scope = co
    }
    (*co).bsym = bsym;
    block(0 as std::os::raw::c_int);
    (*co).bsym = b;
    if !csym.is_null() {
        (*co).csym = c;
        loop_scope = lo
    };
}
unsafe extern "C" fn block(mut is_expr: std::os::raw::c_int) {
    let mut current_block: u64;
    let mut a: std::os::raw::c_int = 0;
    let mut b: std::os::raw::c_int = 0;
    let mut c: std::os::raw::c_int = 0;
    let mut d: std::os::raw::c_int = 0;
    let mut e: std::os::raw::c_int = 0;
    let mut t: std::os::raw::c_int = 0;
    let mut o_0: scope = scope {
        prev: 0 as *mut scope,
        vla: C2RustUnnamed_13 {
            loc: 0,
            locorig: 0,
            num: 0,
        },
        cl: C2RustUnnamed_14 {
            s: 0 as *mut Sym,
            n: 0,
        },
        bsym: 0 as *mut std::os::raw::c_int,
        csym: 0 as *mut std::os::raw::c_int,
        lstk: 0 as *mut Sym,
        llstk: 0 as *mut Sym,
    };
    let mut s: *mut Sym = 0 as *mut Sym;
    if is_expr != 0 {
        /* default return value is (void) */
        vpushi(0 as std::os::raw::c_int);
        (*vtop).type_0.t = 0 as std::os::raw::c_int
    }
    loop {
        t = tok;
        /* If the token carries a value, next() might destroy it. Only with
        invalid code such as f(){"123"4;} */
        if t >= 0xc0 as std::os::raw::c_int && t <= 0xcf as std::os::raw::c_int {
            current_block = 12072121998757195963;
            break;
        }
        next();
        if debug_modes != 0 {
            tcc_tcov_check_line(0 as std::os::raw::c_int);
            tcc_tcov_block_begin();
        }
        if t == TOK_IF as std::os::raw::c_int {
            skip('(' as i32);
            gexpr();
            skip(')' as i32);
            a = gvtst(1 as std::os::raw::c_int, 0 as std::os::raw::c_int);
            block(0 as std::os::raw::c_int);
            if tok == TOK_ELSE as std::os::raw::c_int {
                d = gjmp_acs(0 as std::os::raw::c_int);
                gsym(a);
                next();
                block(0 as std::os::raw::c_int);
                gsym(d);
            /* patch else jmp */
            } else {
                gsym(a);
            }
            current_block = 12374684203807546672;
            break;
        } else if t == TOK_WHILE as std::os::raw::c_int {
            d = gind();
            skip('(' as i32);
            gexpr();
            skip(')' as i32);
            a = gvtst(1 as std::os::raw::c_int, 0 as std::os::raw::c_int);
            b = 0 as std::os::raw::c_int;
            lblock(&mut a, &mut b);
            gjmp_addr_acs(d);
            gsym_addr(b, d);
            gsym(a);
            current_block = 12374684203807546672;
            break;
        } else if t == '{' as i32 {
            new_scope(&mut o_0);
            /* handle local labels declarations */
            while tok == TOK_LABEL as std::os::raw::c_int {
                loop {
                    next();
                    if tok < TOK_DEFINE as std::os::raw::c_int {
                        expect(b"label identifier\x00" as *const u8 as *const std::os::raw::c_char);
                    }
                    label_push(&mut local_label_stack, tok, 2 as std::os::raw::c_int);
                    next();
                    if !(tok == ',' as i32) {
                        break;
                    }
                }
                skip(';' as i32);
            }
            while tok != '}' as i32 {
                decl(0x32 as std::os::raw::c_int);
                if tok != '}' as i32 {
                    if is_expr != 0 {
                        vpop();
                    }
                    block(is_expr);
                }
            }
            prev_scope(&mut o_0, is_expr);
            if local_scope != 0 {
                next();
            } else if nocode_wanted == 0 {
                check_func_return();
            }
            current_block = 12374684203807546672;
            break;
        } else if t == TOK_RETURN as std::os::raw::c_int {
            b = (func_vt.t & 0xf as std::os::raw::c_int != 0 as std::os::raw::c_int)
                as std::os::raw::c_int;
            if tok != ';' as i32 {
                gexpr();
                if b != 0 {
                    gen_assign_cast(&mut func_vt);
                } else {
                    if (*vtop).type_0.t != 0 as std::os::raw::c_int {
                        _tcc_warning(
                            b"void function returns a value\x00" as *const u8
                                as *const std::os::raw::c_char,
                        );
                    }
                    vtop = vtop.offset(-1)
                }
            } else if b != 0 {
                _tcc_warning(
                    b"\'return\' with no value\x00" as *const u8 as *const std::os::raw::c_char,
                );
                b = 0 as std::os::raw::c_int
            }
            leave_scope(root_scope);
            if b != 0 {
                gfunc_return(&mut func_vt);
            }
            skip(';' as i32);
            /* jump unless last stmt in top-level block */
            if tok != '}' as i32 || local_scope != 1 as std::os::raw::c_int {
                rsym = gjmp_acs(rsym)
            }
            if debug_modes != 0 {
                tcc_tcov_block_end(tcov_data.line);
            }
            nocode_wanted |= 0x20000000 as std::os::raw::c_int;
            current_block = 12374684203807546672;
            break;
        } else if t == TOK_BREAK as std::os::raw::c_int {
            /* compute jump */
            if (*cur_scope).bsym.is_null() {
                _tcc_error(b"cannot break\x00" as *const u8 as *const std::os::raw::c_char);
            }
            if !cur_switch.is_null() && (*cur_scope).bsym == (*cur_switch).bsym {
                leave_scope((*cur_switch).scope);
            } else {
                leave_scope(loop_scope);
            }
            *(*cur_scope).bsym = gjmp_acs(*(*cur_scope).bsym);
            skip(';' as i32);
            current_block = 12374684203807546672;
            break;
        } else if t == TOK_CONTINUE as std::os::raw::c_int {
            /* compute jump */
            if (*cur_scope).csym.is_null() {
                _tcc_error(b"cannot continue\x00" as *const u8 as *const std::os::raw::c_char);
            }
            leave_scope(loop_scope);
            *(*cur_scope).csym = gjmp_acs(*(*cur_scope).csym);
            skip(';' as i32);
            current_block = 12374684203807546672;
            break;
        } else if t == TOK_FOR as std::os::raw::c_int {
            new_scope(&mut o_0);
            skip('(' as i32);
            if tok != ';' as i32 {
                /* c99 for-loop init decl? */
                if decl0(
                    0x32 as std::os::raw::c_int,
                    1 as std::os::raw::c_int,
                    0 as *mut Sym,
                ) == 0
                {
                    /* no, regular for-loop init expr */
                    gexpr(); /* save switch value */
                    vpop(); /* jump to first case */
                }
            } /* add implicit break */
            skip(';' as i32);
            b = 0 as std::os::raw::c_int;
            a = b;
            d = gind();
            c = d;
            if tok != ';' as i32 {
                gexpr();
                a = gvtst(1 as std::os::raw::c_int, 0 as std::os::raw::c_int)
            }
            skip(';' as i32);
            if tok != ')' as i32 {
                e = gjmp_acs(0 as std::os::raw::c_int);
                d = gind();
                gexpr();
                vpop();
                gjmp_addr_acs(c);
                gsym(e);
            }
            skip(')' as i32);
            lblock(&mut a, &mut b);
            gjmp_addr_acs(d);
            gsym_addr(b, d);
            gsym(a);
            prev_scope(&mut o_0, 0 as std::os::raw::c_int);
            current_block = 12374684203807546672;
            break;
        } else if t == TOK_DO as std::os::raw::c_int {
            b = 0 as std::os::raw::c_int;
            a = b;
            d = gind();
            lblock(&mut a, &mut b);
            gsym(b);
            skip(TOK_WHILE as std::os::raw::c_int);
            skip('(' as i32);
            gexpr();
            skip(')' as i32);
            skip(';' as i32);
            c = gvtst(0 as std::os::raw::c_int, 0 as std::os::raw::c_int);
            gsym_addr(c, d);
            gsym(a);
            current_block = 12374684203807546672;
            break;
        } else if t == TOK_SWITCH as std::os::raw::c_int {
            let mut sw: *mut switch_t = 0 as *mut switch_t;
            sw = tcc_mallocz(::std::mem::size_of::<switch_t>() as std::os::raw::c_ulong)
                as *mut switch_t;
            (*sw).bsym = &mut a;
            (*sw).scope = cur_scope;
            (*sw).prev = cur_switch;
            cur_switch = sw;
            skip('(' as i32);
            gexpr();
            skip(')' as i32);
            let fresh12 = vtop;
            vtop = vtop.offset(-1);
            (*sw).sv = *fresh12;
            a = 0 as std::os::raw::c_int;
            b = gjmp_acs(0 as std::os::raw::c_int);
            lblock(&mut a, 0 as *mut std::os::raw::c_int);
            a = gjmp_acs(a);
            /* case lookup */
            gsym(b);
            if (*sw).sv.type_0.t & 0x10 as std::os::raw::c_int != 0 {
                qsort(
                    (*sw).p as *mut std::os::raw::c_void,
                    (*sw).n as size_t,
                    ::std::mem::size_of::<*mut std::os::raw::c_void>() as std::os::raw::c_ulong,
                    Some(
                        case_cmpu
                            as unsafe extern "C" fn(
                                _: *const std::os::raw::c_void,
                                _: *const std::os::raw::c_void,
                            )
                                -> std::os::raw::c_int,
                    ),
                );
            } else {
                qsort(
                    (*sw).p as *mut std::os::raw::c_void,
                    (*sw).n as size_t,
                    ::std::mem::size_of::<*mut std::os::raw::c_void>() as std::os::raw::c_ulong,
                    Some(
                        case_cmpi
                            as unsafe extern "C" fn(
                                _: *const std::os::raw::c_void,
                                _: *const std::os::raw::c_void,
                            )
                                -> std::os::raw::c_int,
                    ),
                );
            }
            b = 1 as std::os::raw::c_int;
            while b < (*sw).n {
                if if (*sw).sv.type_0.t & 0x10 as std::os::raw::c_int != 0 {
                    ((**(*sw).p.offset((b - 1 as std::os::raw::c_int) as isize)).v2 as uint64_t
                        >= (**(*sw).p.offset(b as isize)).v1 as uint64_t)
                        as std::os::raw::c_int
                } else {
                    ((**(*sw).p.offset((b - 1 as std::os::raw::c_int) as isize)).v2
                        >= (**(*sw).p.offset(b as isize)).v1)
                        as std::os::raw::c_int
                } != 0
                {
                    _tcc_error(
                        b"duplicate case value\x00" as *const u8 as *const std::os::raw::c_char,
                    );
                }
                b += 1
            }
            vpushv(&mut (*sw).sv);
            gv(0x1 as std::os::raw::c_int);
            d = 0 as std::os::raw::c_int;
            gcase((*sw).p, (*sw).n, &mut d);
            vpop();
            if (*sw).def_sym != 0 {
                gsym_addr(d, (*sw).def_sym);
            } else {
                gsym(d);
            }
            /* break label */
            gsym(a);
            dynarray_reset(
                &mut (*sw).p as *mut *mut *mut case_t as *mut std::os::raw::c_void,
                &mut (*sw).n,
            );
            cur_switch = (*sw).prev;
            tcc_free(sw as *mut std::os::raw::c_void);
            current_block = 12374684203807546672;
            break;
        } else {
            if t == TOK_CASE as std::os::raw::c_int {
                let mut cr: *mut case_t =
                    tcc_malloc(::std::mem::size_of::<case_t>() as std::os::raw::c_ulong)
                        as *mut case_t;
                if cur_switch.is_null() {
                    expect(b"switch\x00" as *const u8 as *const std::os::raw::c_char);
                }
                (*cr).v2 = expr_const64();
                (*cr).v1 = (*cr).v2;
                if (*tcc_state).gnu_ext as std::os::raw::c_int != 0
                    && tok == 0xa1 as std::os::raw::c_int
                {
                    next();
                    (*cr).v2 = expr_const64();
                    if (*cur_switch).sv.type_0.t & 0x10 as std::os::raw::c_int == 0
                        && (*cr).v2 < (*cr).v1
                        || (*cur_switch).sv.type_0.t & 0x10 as std::os::raw::c_int != 0
                            && ((*cr).v2 as uint64_t) < (*cr).v1 as uint64_t
                    {
                        _tcc_warning(
                            b"empty case range\x00" as *const u8 as *const std::os::raw::c_char,
                        );
                    }
                }
                tcov_data.ind = 0 as std::os::raw::c_int;
                (*cr).sym = gind();
                dynarray_add(
                    &mut (*cur_switch).p as *mut *mut *mut case_t as *mut std::os::raw::c_void,
                    &mut (*cur_switch).n,
                    cr as *mut std::os::raw::c_void,
                );
                skip(':' as i32);
                is_expr = 0 as std::os::raw::c_int
            } else if t == TOK_DEFAULT as std::os::raw::c_int {
                if cur_switch.is_null() {
                    expect(b"switch\x00" as *const u8 as *const std::os::raw::c_char);
                }
                if (*cur_switch).def_sym != 0 {
                    _tcc_error(
                        b"too many \'default\'\x00" as *const u8 as *const std::os::raw::c_char,
                    );
                }
                tcov_data.ind = 0 as std::os::raw::c_int;
                (*cur_switch).def_sym = gind();
                skip(':' as i32);
                is_expr = 0 as std::os::raw::c_int
            } else if t == TOK_GOTO as std::os::raw::c_int {
                if (*cur_scope).vla.num != 0 {
                    vla_restore((*cur_scope).vla.locorig);
                }
                if tok == '*' as i32 && (*tcc_state).gnu_ext as std::os::raw::c_int != 0 {
                    /* computed goto */
                    next();
                    gexpr();
                    if (*vtop).type_0.t & 0xf as std::os::raw::c_int != 5 as std::os::raw::c_int {
                        expect(b"pointer\x00" as *const u8 as *const std::os::raw::c_char);
                    }
                    ggoto();
                } else if tok >= TOK_DEFINE as std::os::raw::c_int {
                    s = label_find(tok);
                    /* put forward definition if needed */
                    if s.is_null() {
                        s = label_push(&mut global_label_stack, tok, 1 as std::os::raw::c_int)
                    } else if (*s).r as std::os::raw::c_int == 2 as std::os::raw::c_int {
                        (*s).r = 1 as std::os::raw::c_int as std::os::raw::c_ushort
                    }
                    if (*s).r as std::os::raw::c_int & 1 as std::os::raw::c_int != 0 {
                        /* start new goto chain for cleanups, linked via label->next */
                        if !(*cur_scope).cl.s.is_null() && nocode_wanted == 0 {
                            sym_push2(
                                &mut pending_gotos,
                                0x20000000 as std::os::raw::c_int,
                                0 as std::os::raw::c_int,
                                (*cur_scope).cl.n,
                            );
                            (*pending_gotos).prev_tok = s;
                            s = sym_push2(
                                &mut (*s).c2rust_unnamed_0.next,
                                0x20000000 as std::os::raw::c_int,
                                0 as std::os::raw::c_int,
                                0 as std::os::raw::c_int,
                            );
                            (*pending_gotos).c2rust_unnamed_0.next = s
                        }
                        (*s).c2rust_unnamed.c2rust_unnamed.c2rust_unnamed.jnext =
                            gjmp_acs((*s).c2rust_unnamed.c2rust_unnamed.c2rust_unnamed.jnext)
                    } else {
                        try_call_cleanup_goto((*s).c2rust_unnamed_0.cleanupstate);
                        gjmp_addr_acs((*s).c2rust_unnamed.c2rust_unnamed.c2rust_unnamed.jnext);
                    }
                    next();
                } else {
                    expect(b"label identifier\x00" as *const u8 as *const std::os::raw::c_char);
                }
                skip(';' as i32);
                current_block = 12374684203807546672;
                break;
            } else if t == TOK_ASM1 as std::os::raw::c_int
                || t == TOK_ASM2 as std::os::raw::c_int
                || t == TOK_ASM3 as std::os::raw::c_int
            {
                asm_instr();
                current_block = 12374684203807546672;
                break;
            } else if tok == ':' as i32 && t >= TOK_DEFINE as std::os::raw::c_int {
                /* label case */
                next(); /* pending cleanup goto */
                s = label_find(t);
                if !s.is_null() {
                    if (*s).r as std::os::raw::c_int == 0 as std::os::raw::c_int {
                        _tcc_error(
                            b"duplicate label \'%s\'\x00" as *const u8
                                as *const std::os::raw::c_char,
                            get_tok_str((*s).v, 0 as *mut CValue),
                        );
                    }
                    (*s).r = 0 as std::os::raw::c_int as std::os::raw::c_ushort;
                    if !(*s).c2rust_unnamed_0.next.is_null() {
                        let mut pcl: *mut Sym = 0 as *mut Sym;
                        pcl = (*s).c2rust_unnamed_0.next;
                        while !pcl.is_null() {
                            gsym((*pcl).c2rust_unnamed.c2rust_unnamed.c2rust_unnamed.jnext);
                            pcl = (*pcl).prev
                        }
                        sym_pop(
                            &mut (*s).c2rust_unnamed_0.next,
                            0 as *mut Sym,
                            0 as std::os::raw::c_int,
                        );
                    } else {
                        gsym((*s).c2rust_unnamed.c2rust_unnamed.c2rust_unnamed.jnext);
                    }
                } else {
                    s = label_push(&mut global_label_stack, t, 0 as std::os::raw::c_int)
                }
                (*s).c2rust_unnamed.c2rust_unnamed.c2rust_unnamed.jnext = gind();
                (*s).c2rust_unnamed_0.cleanupstate = (*cur_scope).cl.s
            } else if t != ';' as i32
            /* expression case */
            {
                current_block = 1997090137068572327;
                break;
            } else {
                current_block = 12374684203807546672;
                break;
            }
            vla_restore((*cur_scope).vla.loc);
            /* we accept this, but it is a mistake */
            if !(tok == '}' as i32) {
                continue;
            }
            _tcc_warning(
                b"deprecated use of label at end of compound statement\x00" as *const u8
                    as *const std::os::raw::c_char,
            );
            current_block = 12374684203807546672;
            break;
        }
    }
    match current_block {
        1997090137068572327 => {
            unget_tok(t);
            current_block = 12072121998757195963;
        },
        _ => {},
    }
    match current_block {
        12072121998757195963 => {
            if is_expr != 0 {
                vpop();
                gexpr();
            } else {
                gexpr();
                vpop();
            }
            skip(';' as i32);
        },
        _ => {},
    }
    if debug_modes != 0 {
        tcc_tcov_check_line(0 as std::os::raw::c_int);
        tcc_tcov_block_end(0 as std::os::raw::c_int);
    };
}
/* This skips over a stream of tokens containing balanced {} and ()
pairs, stopping at outer ',' ';' and '}' (or matching '}' if we started
with a '{').  If STR then allocates and stores the skipped tokens
in *STR.  This doesn't check if () and {} are nested correctly,
i.e. "({)}" is accepted.  */
unsafe extern "C" fn skip_or_save_block(mut str: *mut *mut TokenString) {
    let mut braces: std::os::raw::c_int = (tok == '{' as i32) as std::os::raw::c_int;
    let mut level: std::os::raw::c_int = 0 as std::os::raw::c_int;
    if !str.is_null() {
        *str = tok_str_alloc()
    }
    while level > 0 as std::os::raw::c_int
        || tok != '}' as i32 && tok != ',' as i32 && tok != ';' as i32 && tok != ')' as i32
    {
        let mut t: std::os::raw::c_int = 0;
        if tok == -(1 as std::os::raw::c_int) {
            if !(!str.is_null() || level > 0 as std::os::raw::c_int) {
                break;
            }
            _tcc_error(b"unexpected end of file\x00" as *const u8 as *const std::os::raw::c_char);
        } else {
            if !str.is_null() {
                tok_str_add_tok(*str);
            }
            t = tok;
            next();
            if t == '{' as i32 || t == '(' as i32 {
                level += 1
            } else {
                if !(t == '}' as i32 || t == ')' as i32) {
                    continue;
                }
                level -= 1;
                if level == 0 as std::os::raw::c_int && braces != 0 && t == '}' as i32 {
                    break;
                }
            }
        }
    }
    if !str.is_null() {
        tok_str_add(*str, -(1 as std::os::raw::c_int));
        tok_str_add(*str, 0 as std::os::raw::c_int);
    };
}
unsafe extern "C" fn parse_init_elem(mut expr_type_0: std::os::raw::c_int) {
    let mut saved_global_expr: std::os::raw::c_int = 0;
    match expr_type_0 {
        1 => {
            /* compound literals must be allocated globally in this case */
            saved_global_expr = global_expr;
            global_expr = 1 as std::os::raw::c_int;
            expr_const1();
            global_expr = saved_global_expr;
            /* NOTE: symbols are accepted, as well as lvalue for anon symbols
            (compound literals).  */
            if (*vtop).r as std::os::raw::c_int
                & (0x3f as std::os::raw::c_int | 0x100 as std::os::raw::c_int)
                != 0x30 as std::os::raw::c_int
                && ((*vtop).r as std::os::raw::c_int
                    & (0x200 as std::os::raw::c_int | 0x100 as std::os::raw::c_int)
                    != 0x200 as std::os::raw::c_int | 0x100 as std::os::raw::c_int
                    || (*(*vtop).c2rust_unnamed_0.sym).v < 0x10000000 as std::os::raw::c_int)
            {
                _tcc_error(
                    b"initializer element is not constant\x00" as *const u8
                        as *const std::os::raw::c_char,
                );
            }
        },
        2 => {
            expr_eq();
        },
        _ => {},
    };
}
unsafe extern "C" fn init_assert(mut p: *mut init_params, mut offset: std::os::raw::c_int) {
    if if !(*p).sec.is_null() {
        (!(nocode_wanted > 0 as std::os::raw::c_int)
            && offset as std::os::raw::c_ulong > (*(*p).sec).data_offset)
            as std::os::raw::c_int
    } else {
        (nocode_wanted == 0 && offset > (*p).local_offset) as std::os::raw::c_int
    } != 0
    {
        _tcc_error(
            b"internal compiler error\n%s:%d: in %s(): initializer overflow\x00" as *const u8
                as *const std::os::raw::c_char,
            b"tccgen.c\x00" as *const u8 as *const std::os::raw::c_char,
            7786 as std::os::raw::c_int,
            (*::std::mem::transmute::<&[u8; 12], &[std::os::raw::c_char; 12]>(b"init_assert\x00"))
                .as_ptr(),
        );
    };
}
/* put zeros for variable based init */
unsafe extern "C" fn init_putz(
    mut p: *mut init_params,
    mut c: std::os::raw::c_ulong,
    mut size: std::os::raw::c_int,
) {
    init_assert(
        p,
        c.wrapping_add(size as std::os::raw::c_ulong) as std::os::raw::c_int,
    );
    if (*p).sec.is_null() {
        vpush_helper_func(TOK_memset as std::os::raw::c_int);
        vseti(0x32 as std::os::raw::c_int, c as std::os::raw::c_int);
        vpushi(0 as std::os::raw::c_int);
        vpushs(size as Elf64_Addr);
        gfunc_call(3 as std::os::raw::c_int);
    };
}
/* delete relocations for specified range c ... c + size. Unfortunatly
in very special cases, relocations may occur unordered */
unsafe extern "C" fn decl_design_delrels(
    mut sec: *mut Section,
    mut c: std::os::raw::c_int,
    mut size: std::os::raw::c_int,
) {
    let mut rel: *mut Elf64_Rela = 0 as *mut Elf64_Rela;
    let mut rel2: *mut Elf64_Rela = 0 as *mut Elf64_Rela;
    let mut rel_end: *mut Elf64_Rela = 0 as *mut Elf64_Rela;
    if sec.is_null() || (*sec).reloc.is_null() {
        return;
    }
    rel2 = (*(*sec).reloc).data as *mut Elf64_Rela;
    rel = rel2;
    rel_end = (*(*sec).reloc)
        .data
        .offset((*(*sec).reloc).data_offset as isize) as *mut Elf64_Rela;
    while rel < rel_end {
        if (*rel).r_offset >= c as std::os::raw::c_ulong
            && (*rel).r_offset < (c + size) as std::os::raw::c_ulong
        {
            (*(*sec).reloc).data_offset = (*(*sec).reloc)
                .data_offset
                .wrapping_sub(::std::mem::size_of::<Elf64_Rela>() as std::os::raw::c_ulong)
        } else {
            if rel2 != rel {
                memcpy(
                    rel2 as *mut std::os::raw::c_void,
                    rel as *const std::os::raw::c_void,
                    ::std::mem::size_of::<Elf64_Rela>() as std::os::raw::c_ulong,
                );
            }
            rel2 = rel2.offset(1)
        }
        rel = rel.offset(1)
    }
}
unsafe extern "C" fn decl_design_flex(
    mut p: *mut init_params,
    mut ref_0: *mut Sym,
    mut index: std::os::raw::c_int,
) {
    if ref_0 == (*p).flex_array_ref {
        if index >= (*ref_0).c2rust_unnamed.c2rust_unnamed.c {
            (*ref_0).c2rust_unnamed.c2rust_unnamed.c = index + 1 as std::os::raw::c_int
        }
    } else if (*ref_0).c2rust_unnamed.c2rust_unnamed.c < 0 as std::os::raw::c_int {
        _tcc_error(
            b"flexible array has zero size in this context\x00" as *const u8
                as *const std::os::raw::c_char,
        );
    };
}
/* t is the array or struct type. c is the array or struct
address. cur_field is the pointer to the current
field, for arrays the 'c' member contains the current start
index.  'flags' is as in decl_initializer.
'al' contains the already initialized length of the
current container (starting at c).  This returns the new length of that.  */
unsafe extern "C" fn decl_designator(
    mut p: *mut init_params,
    mut type_0: *mut CType,
    mut c: std::os::raw::c_ulong,
    mut cur_field: *mut *mut Sym,
    mut flags: std::os::raw::c_int,
    mut al: std::os::raw::c_int,
) -> std::os::raw::c_int {
    let mut cumofs: std::os::raw::c_int = 0;
    let mut current_block: u64;
    let mut s: *mut Sym = 0 as *mut Sym;
    let mut f: *mut Sym = 0 as *mut Sym;
    let mut index: std::os::raw::c_int = 0;
    let mut index_last: std::os::raw::c_int = 0;
    let mut align: std::os::raw::c_int = 0;
    let mut l: std::os::raw::c_int = 0;
    let mut nb_elems: std::os::raw::c_int = 0;
    let mut elem_size: std::os::raw::c_int = 0;
    let mut corig: std::os::raw::c_ulong = c;
    elem_size = 0 as std::os::raw::c_int;
    nb_elems = 1 as std::os::raw::c_int;
    if flags & 4 as std::os::raw::c_int != 0 {
        current_block = 15680099734316470477;
    } else {
        if (*tcc_state).gnu_ext as std::os::raw::c_int != 0
            && tok >= TOK_DEFINE as std::os::raw::c_int
        {
            l = tok;
            next();
            if tok == ':' as i32 {
                current_block = 13544863498864926042;
            } else {
                unget_tok(l);
                current_block = 3276175668257526147;
            }
        } else {
            current_block = 3276175668257526147;
        }
        loop {
            match current_block {
                3276175668257526147 =>
                /* NOTE: we only support ranges for last designator */
                {
                    if !(nb_elems == 1 as std::os::raw::c_int
                        && (tok == '[' as i32 || tok == '.' as i32))
                    {
                        break;
                    }
                    if tok == '[' as i32 {
                        if (*type_0).t & 0x40 as std::os::raw::c_int == 0 {
                            expect(b"array type\x00" as *const u8 as *const std::os::raw::c_char);
                        }
                        next();
                        index_last = expr_const();
                        index = index_last;
                        if tok == 0xa1 as std::os::raw::c_int
                            && (*tcc_state).gnu_ext as std::os::raw::c_int != 0
                        {
                            next();
                            index_last = expr_const()
                        }
                        skip(']' as i32);
                        s = (*type_0).ref_0;
                        decl_design_flex(p, s, index_last);
                        if index < 0 as std::os::raw::c_int
                            || index_last >= (*s).c2rust_unnamed.c2rust_unnamed.c
                            || index_last < index
                        {
                            _tcc_error(
                                b"index exceeds array bounds or range is empty\x00" as *const u8
                                    as *const std::os::raw::c_char,
                            );
                        }
                        if !cur_field.is_null() {
                            (**cur_field).c2rust_unnamed.c2rust_unnamed.c = index_last
                        }
                        type_0 = pointed_type(type_0);
                        elem_size = type_size(type_0, &mut align);
                        c = c.wrapping_add((index * elem_size) as std::os::raw::c_ulong);
                        nb_elems = index_last - index + 1 as std::os::raw::c_int
                    } else {
                        cumofs = 0;
                        next();
                        l = tok;
                        current_block = 13544863498864926042;
                        continue;
                    }
                }
                _ => {
                    next();
                    if (*type_0).t & 0xf as std::os::raw::c_int != 7 as std::os::raw::c_int {
                        expect(
                            b"struct/union type\x00" as *const u8 as *const std::os::raw::c_char,
                        );
                    }
                    cumofs = 0 as std::os::raw::c_int;
                    f = find_field(type_0, l, &mut cumofs);
                    if f.is_null() {
                        expect(b"field\x00" as *const u8 as *const std::os::raw::c_char);
                    }
                    if !cur_field.is_null() {
                        *cur_field = f
                    }
                    type_0 = &mut (*f).type_0;
                    c = c.wrapping_add(
                        (cumofs + (*f).c2rust_unnamed.c2rust_unnamed.c) as std::os::raw::c_ulong,
                    )
                },
            }
            cur_field = 0 as *mut *mut Sym;
            current_block = 3276175668257526147;
        }
        if cur_field.is_null() {
            if tok == '=' as i32 {
                next();
            } else if (*tcc_state).gnu_ext == 0 {
                expect(b"=\x00" as *const u8 as *const std::os::raw::c_char);
            }
            current_block = 13707613154239713890;
        } else {
            current_block = 15680099734316470477;
        }
    }
    match current_block {
        15680099734316470477 => {
            if (*type_0).t & 0x40 as std::os::raw::c_int != 0 {
                index = (**cur_field).c2rust_unnamed.c2rust_unnamed.c;
                s = (*type_0).ref_0;
                decl_design_flex(p, s, index);
                if index >= (*s).c2rust_unnamed.c2rust_unnamed.c {
                    _tcc_error(
                        b"too many initializers\x00" as *const u8 as *const std::os::raw::c_char,
                    );
                }
                type_0 = pointed_type(type_0);
                elem_size = type_size(type_0, &mut align);
                c = c.wrapping_add((index * elem_size) as std::os::raw::c_ulong)
            } else {
                f = *cur_field;
                while !f.is_null()
                    && (*f).v & 0x10000000 as std::os::raw::c_int != 0
                    && (*f).type_0.t & 0x80 as std::os::raw::c_int != 0
                {
                    f = (*f).c2rust_unnamed_0.next;
                    *cur_field = f
                }
                if f.is_null() {
                    _tcc_error(
                        b"too many initializers\x00" as *const u8 as *const std::os::raw::c_char,
                    );
                }
                type_0 = &mut (*f).type_0;
                c = c.wrapping_add((*f).c2rust_unnamed.c2rust_unnamed.c as std::os::raw::c_ulong)
            }
        },
        _ => {},
    }
    if elem_size == 0 {
        /* for structs */
        elem_size = type_size(type_0, &mut align)
    }
    /* Using designators the same element can be initialized more
    than once.  In that case we need to delete possibly already
    existing relocations. */
    if flags & 2 as std::os::raw::c_int == 0 && c.wrapping_sub(corig) < al as std::os::raw::c_ulong
    {
        decl_design_delrels((*p).sec, c as std::os::raw::c_int, elem_size * nb_elems);
        flags &= !(8 as std::os::raw::c_int)
        /* mark stack dirty too */
    }
    decl_initializer(p, type_0, c, flags & !(1 as std::os::raw::c_int));
    if flags & 2 as std::os::raw::c_int == 0 && nb_elems > 1 as std::os::raw::c_int {
        let mut aref: Sym = {
            let mut init = Sym {
                v: 0 as std::os::raw::c_int,
                r: 0,
                a: SymAttr([0; 2]),
                c2rust_unnamed: C2RustUnnamed_0 {
                    c2rust_unnamed: C2RustUnnamed_1 {
                        c: 0,
                        c2rust_unnamed: C2RustUnnamed_2 { sym_scope: 0 },
                    },
                },
                type_0: CType {
                    t: 0,
                    ref_0: 0 as *const Sym as *mut Sym,
                },
                c2rust_unnamed_0: C2RustUnnamed {
                    next: 0 as *mut Sym,
                },
                prev: 0 as *mut Sym,
                prev_tok: 0 as *mut Sym,
            };
            init
        };
        let mut t1: CType = CType {
            t: 0,
            ref_0: 0 as *const Sym as *mut Sym,
        };
        let mut i: std::os::raw::c_int = 0;
        if !(*p).sec.is_null() || (*type_0).t & 0x40 as std::os::raw::c_int != 0 {
            /* make init_putv/vstore believe it were a struct */
            aref.c2rust_unnamed.c2rust_unnamed.c = elem_size;
            t1.t = 7 as std::os::raw::c_int;
            t1.ref_0 = &mut aref;
            type_0 = &mut t1
        }
        if !(*p).sec.is_null() {
            vpush_ref(type_0, (*p).sec, c, elem_size as std::os::raw::c_ulong);
        } else {
            vset(
                type_0,
                0x32 as std::os::raw::c_int | 0x100 as std::os::raw::c_int,
                c as std::os::raw::c_int,
            );
        }
        i = 1 as std::os::raw::c_int;
        while i < nb_elems {
            vdup();
            init_putv(
                p,
                type_0,
                c.wrapping_add((elem_size * i) as std::os::raw::c_ulong),
            );
            i += 1
        }
        vpop();
    }
    c = c.wrapping_add((nb_elems * elem_size) as std::os::raw::c_ulong);
    if c.wrapping_sub(corig) > al as std::os::raw::c_ulong {
        al = c.wrapping_sub(corig) as std::os::raw::c_int
    }
    return al;
}
/* store a value or an expression directly in global data or in local array */
unsafe extern "C" fn init_putv(
    mut p: *mut init_params,
    mut type_0: *mut CType,
    mut c: std::os::raw::c_ulong,
) {
    let mut bt: std::os::raw::c_int = 0; /* need to do that to avoid false warning */
    let mut ptr: *mut std::os::raw::c_void = 0 as *mut std::os::raw::c_void;
    let mut dtype: CType = CType {
        t: 0,
        ref_0: 0 as *const Sym as *mut Sym,
    };
    let mut size: std::os::raw::c_int = 0;
    let mut align: std::os::raw::c_int = 0;
    let mut sec: *mut Section = (*p).sec;
    let mut val: uint64_t = 0;
    dtype = *type_0;
    dtype.t &= !(0x100 as std::os::raw::c_int);
    size = type_size(type_0, &mut align);
    if (*type_0).t & 0x80 as std::os::raw::c_int != 0 {
        size = (((*type_0).t >> 20 as std::os::raw::c_int & 0x3f as std::os::raw::c_int)
            + ((*type_0).t >> 20 as std::os::raw::c_int + 6 as std::os::raw::c_int
                & 0x3f as std::os::raw::c_int)
            + 7 as std::os::raw::c_int)
            / 8 as std::os::raw::c_int
    }
    init_assert(
        p,
        c.wrapping_add(size as std::os::raw::c_ulong) as std::os::raw::c_int,
    );
    if !sec.is_null() {
        /* XXX: not portable */
        /* XXX: generate error if incorrect relocation */
        gen_assign_cast(&mut dtype);
        bt = (*type_0).t & 0xf as std::os::raw::c_int;
        if (*vtop).r as std::os::raw::c_int & 0x200 as std::os::raw::c_int != 0
            && bt != 5 as std::os::raw::c_int
            && (bt
                != (if 8 as std::os::raw::c_int == 8 as std::os::raw::c_int {
                    4 as std::os::raw::c_int
                } else {
                    3 as std::os::raw::c_int
                })
                || (*type_0).t & 0x80 as std::os::raw::c_int != 0)
            && !((*vtop).r as std::os::raw::c_int & 0x30 as std::os::raw::c_int != 0
                && (*(*vtop).c2rust_unnamed_0.sym).v >= 0x10000000 as std::os::raw::c_int)
        {
            _tcc_error(
                b"initializer element is not computable at load time\x00" as *const u8
                    as *const std::os::raw::c_char,
            );
        }
        if nocode_wanted > 0 as std::os::raw::c_int {
            vtop = vtop.offset(-1);
            return;
        }
        ptr = (*sec).data.offset(c as isize) as *mut std::os::raw::c_void;
        val = (*vtop).c2rust_unnamed.c.i;
        /* XXX: make code faster ? */
        if (*vtop).r as std::os::raw::c_int
            & (0x200 as std::os::raw::c_int | 0x30 as std::os::raw::c_int)
            == 0x200 as std::os::raw::c_int | 0x30 as std::os::raw::c_int
            && (*(*vtop).c2rust_unnamed_0.sym).v >= 0x10000000 as std::os::raw::c_int
            && (*vtop).type_0.t & 0xf as std::os::raw::c_int != 5 as std::os::raw::c_int
        {
            /* These come from compound literals, memcpy stuff over.  */
            let mut ssec: *mut Section = 0 as *mut Section;
            let mut esym: *mut Elf64_Sym = 0 as *mut Elf64_Sym;
            let mut rel: *mut Elf64_Rela = 0 as *mut Elf64_Rela;
            esym = elfsym((*vtop).c2rust_unnamed_0.sym);
            ssec = *(*tcc_state).sections.offset((*esym).st_shndx as isize);
            memmove(
                ptr,
                (*ssec)
                    .data
                    .offset((*esym).st_value as isize)
                    .offset((*vtop).c2rust_unnamed.c.i as std::os::raw::c_int as isize)
                    as *const std::os::raw::c_void,
                size as std::os::raw::c_ulong,
            );
            if !(*ssec).reloc.is_null() {
                /* We need to copy over all memory contents, and that
                includes relocations.  Use the fact that relocs are
                created it order, so look from the end of relocs
                until we hit one before the copied region.  */
                let mut relofs: std::os::raw::c_ulong = (*(*ssec).reloc).data_offset;
                while relofs >= ::std::mem::size_of::<Elf64_Rela>() as std::os::raw::c_ulong {
                    relofs = relofs
                        .wrapping_sub(::std::mem::size_of::<Elf64_Rela>() as std::os::raw::c_ulong);
                    rel = (*(*ssec).reloc).data.offset(relofs as isize) as *mut Elf64_Rela;
                    if (*rel).r_offset
                        >= (*esym).st_value.wrapping_add(size as std::os::raw::c_ulong)
                    {
                        continue;
                    }
                    if (*rel).r_offset < (*esym).st_value {
                        break;
                    }
                    put_elf_reloca(
                        (*tcc_state).symtab_section,
                        sec,
                        c.wrapping_add((*rel).r_offset)
                            .wrapping_sub((*esym).st_value),
                        ((*rel).r_info
                            & 0xffffffff as std::os::raw::c_uint as std::os::raw::c_ulong)
                            as std::os::raw::c_int,
                        ((*rel).r_info >> 32 as std::os::raw::c_int) as std::os::raw::c_int,
                        (*rel).r_addend as Elf64_Addr,
                    );
                }
            }
        } else if (*type_0).t & 0x80 as std::os::raw::c_int != 0 {
            let mut bit_pos: std::os::raw::c_int = 0;
            let mut bit_size: std::os::raw::c_int = 0;
            let mut bits: std::os::raw::c_int = 0;
            let mut n: std::os::raw::c_int = 0;
            let mut p_0: *mut std::os::raw::c_uchar = 0 as *mut std::os::raw::c_uchar;
            let mut v: std::os::raw::c_uchar = 0;
            let mut m: std::os::raw::c_uchar = 0;
            bit_pos = (*vtop).type_0.t >> 20 as std::os::raw::c_int & 0x3f as std::os::raw::c_int;
            bit_size = (*vtop).type_0.t >> 20 as std::os::raw::c_int + 6 as std::os::raw::c_int
                & 0x3f as std::os::raw::c_int;
            p_0 = (ptr as *mut std::os::raw::c_uchar)
                .offset((bit_pos >> 3 as std::os::raw::c_int) as isize);
            bit_pos &= 7 as std::os::raw::c_int;
            bits = 0 as std::os::raw::c_int;
            while bit_size != 0 {
                n = 8 as std::os::raw::c_int - bit_pos;
                if n > bit_size {
                    n = bit_size
                }
                v = (val >> bits << bit_pos) as std::os::raw::c_uchar;
                m = ((((1 as std::os::raw::c_int) << n) - 1 as std::os::raw::c_int) << bit_pos)
                    as std::os::raw::c_uchar;
                *p_0 = (*p_0 as std::os::raw::c_int & !(m as std::os::raw::c_int)
                    | v as std::os::raw::c_int & m as std::os::raw::c_int)
                    as std::os::raw::c_uchar;
                bits += n;
                bit_size -= n;
                bit_pos = 0 as std::os::raw::c_int;
                p_0 = p_0.offset(1)
            }
        } else {
            match bt {
                11 => {
                    *(ptr as *mut std::os::raw::c_char) =
                        (val != 0 as std::os::raw::c_int as std::os::raw::c_ulong)
                            as std::os::raw::c_int as std::os::raw::c_char
                },
                1 => *(ptr as *mut std::os::raw::c_char) = val as std::os::raw::c_char,
                2 => {
                    write16le(ptr as *mut std::os::raw::c_uchar, val as uint16_t);
                },
                8 => {
                    write32le(ptr as *mut std::os::raw::c_uchar, val as uint32_t);
                },
                9 => {
                    write64le(ptr as *mut std::os::raw::c_uchar, val);
                },
                10 => {
                    /* Host and target platform may be different but both have x87.
                       On windows, tcc does not use VT_LDOUBLE, except when it is a
                       cross compiler.  In this case a mingw gcc as host compiler
                       comes here with 10-byte long doubles, while msvc or tcc won't.
                       tcc itself can still translate by asm.
                       In any case we avoid possibly random bytes 11 and 12.
                    */
                    if ::std::mem::size_of::<f128::f128>() as std::os::raw::c_ulong
                        >= 10 as std::os::raw::c_int as std::os::raw::c_ulong
                    {
                        memcpy(
                            ptr,
                            &mut (*vtop).c2rust_unnamed.c.ld as *mut f128::f128
                                as *const std::os::raw::c_void,
                            10 as std::os::raw::c_int as std::os::raw::c_ulong,
                        );
                    } else if !((*vtop).c2rust_unnamed.c.ld == f128::f128::new(0.0f64)) {
                        /* For other platforms it should work natively, but may not work
                        for cross compilers */
                        if ::std::mem::size_of::<f128::f128>() as std::os::raw::c_ulong
                            == 16 as std::os::raw::c_int as std::os::raw::c_ulong
                        {
                            memcpy(
                                ptr,
                                &mut (*vtop).c2rust_unnamed.c.ld as *mut f128::f128
                                    as *const std::os::raw::c_void,
                                16 as std::os::raw::c_int as std::os::raw::c_ulong,
                            );
                        } else if ::std::mem::size_of::<std::os::raw::c_double>()
                            as std::os::raw::c_ulong
                            == 16 as std::os::raw::c_int as std::os::raw::c_ulong
                        {
                            memcpy(
                                ptr,
                                &mut (*vtop).c2rust_unnamed.c.ld as *mut f128::f128
                                    as *const std::os::raw::c_void,
                                16 as std::os::raw::c_int as std::os::raw::c_ulong,
                            );
                        } else {
                            _tcc_error(
                                b"can\'t cross compile long double constants\x00" as *const u8
                                    as *const std::os::raw::c_char,
                            );
                        }
                    }
                },
                4 | 5 => {
                    /* intptr_t may need a reloc too, see tcctest.c:relocation_test() */
                    if (*vtop).r as std::os::raw::c_int & 0x200 as std::os::raw::c_int != 0 {
                        greloca(
                            sec,
                            (*vtop).c2rust_unnamed_0.sym,
                            c,
                            1 as std::os::raw::c_int,
                            val,
                        );
                    } else {
                        write64le(ptr as *mut std::os::raw::c_uchar, val);
                    }
                },
                3 => {
                    write32le(ptr as *mut std::os::raw::c_uchar, val as uint32_t);
                },
                _ => {},
            }
        }
        vtop = vtop.offset(-1)
    } else {
        vset(
            &mut dtype,
            0x32 as std::os::raw::c_int | 0x100 as std::os::raw::c_int,
            c as std::os::raw::c_int,
        );
        vswap();
        vstore();
        vpop();
    };
}
/* 't' contains the type and storage info. 'c' is the offset of the
object in section 'sec'. If 'sec' is NULL, it means stack based
allocation. 'flags & DIF_FIRST' is true if array '{' must be read (multi
dimension implicit array init handling). 'flags & DIF_SIZE_ONLY' is true if
size only evaluation is wanted (only for arrays). */
unsafe extern "C" fn decl_initializer(
    mut p: *mut init_params,
    mut type_0: *mut CType,
    mut c: std::os::raw::c_ulong,
    mut flags: std::os::raw::c_int,
) {
    let mut len: std::os::raw::c_int = 0;
    let mut n: std::os::raw::c_int = 0;
    let mut no_oblock: std::os::raw::c_int = 0;
    let mut i: std::os::raw::c_int = 0;
    let mut size1: std::os::raw::c_int = 0;
    let mut align1: std::os::raw::c_int = 0;
    let mut s: *mut Sym = 0 as *mut Sym;
    let mut f: *mut Sym = 0 as *mut Sym;
    let mut indexsym: Sym = Sym {
        v: 0,
        r: 0,
        a: SymAttr([0; 2]),
        c2rust_unnamed: C2RustUnnamed_0 {
            c2rust_unnamed: C2RustUnnamed_1 {
                c: 0,
                c2rust_unnamed: C2RustUnnamed_2 { sym_scope: 0 },
            },
        },
        type_0: CType {
            t: 0,
            ref_0: 0 as *const Sym as *mut Sym,
        },
        c2rust_unnamed_0: C2RustUnnamed {
            next: 0 as *mut Sym,
        },
        prev: 0 as *mut Sym,
        prev_tok: 0 as *mut Sym,
    };
    let mut t1: *mut CType = 0 as *mut CType;
    /* generate line number info */
    if debug_modes as std::os::raw::c_int != 0 && (*p).sec.is_null() {
        tcc_debug_line(tcc_state);
        tcc_tcov_check_line(1 as std::os::raw::c_int);
    }
    if flags & 4 as std::os::raw::c_int == 0
        && tok != '{' as i32
        && tok != 0xc9 as std::os::raw::c_int
        && tok != 0xc8 as std::os::raw::c_int
        && flags & 2 as std::os::raw::c_int == 0
    {
        parse_init_elem(if (*p).sec.is_null() {
            2 as std::os::raw::c_int
        } else {
            1 as std::os::raw::c_int
        });
        flags |= 4 as std::os::raw::c_int
    }
    let mut current_block_95: u64;
    if flags & 4 as std::os::raw::c_int != 0
        && (*type_0).t & 0x40 as std::os::raw::c_int == 0
        && is_compatible_unqualified_types(type_0, &mut (*vtop).type_0) != 0
    {
        current_block_95 = 11266845908400809545;
    } else {
        if (*type_0).t & 0x40 as std::os::raw::c_int != 0 {
            no_oblock = 1 as std::os::raw::c_int;
            if flags & 1 as std::os::raw::c_int != 0
                && tok != 0xc9 as std::os::raw::c_int
                && tok != 0xc8 as std::os::raw::c_int
                || tok == '{' as i32
            {
                skip('{' as i32);
                no_oblock = 0 as std::os::raw::c_int
            }
            s = (*type_0).ref_0;
            n = (*s).c2rust_unnamed.c2rust_unnamed.c;
            t1 = pointed_type(type_0);
            size1 = type_size(t1, &mut align1);
            /* only parse strings here if correct type (otherwise: handle
            them as ((w)char *) expressions */
            if tok == 0xc9 as std::os::raw::c_int
                && (*t1).t & 0xf as std::os::raw::c_int == 3 as std::os::raw::c_int
                || tok == 0xc8 as std::os::raw::c_int
                    && (*t1).t & 0xf as std::os::raw::c_int == 1 as std::os::raw::c_int
            {
                len = 0 as std::os::raw::c_int;
                cstr_reset(&mut initstr);
                if size1 as std::os::raw::c_ulong
                    != (if tok == 0xc8 as std::os::raw::c_int {
                        1 as std::os::raw::c_int as std::os::raw::c_ulong
                    } else {
                        ::std::mem::size_of::<nwchar_t>() as std::os::raw::c_ulong
                    })
                {
                    _tcc_error(
                        b"unhandled string literal merging\x00" as *const u8
                            as *const std::os::raw::c_char,
                    );
                }
                while tok == 0xc8 as std::os::raw::c_int || tok == 0xc9 as std::os::raw::c_int {
                    if initstr.size != 0 {
                        initstr.size -= size1
                    }
                    if tok == 0xc8 as std::os::raw::c_int {
                        len += tokc.str_0.size
                    } else {
                        len = (len as std::os::raw::c_ulong).wrapping_add(
                            (tokc.str_0.size as std::os::raw::c_ulong).wrapping_div(
                                ::std::mem::size_of::<nwchar_t>() as std::os::raw::c_ulong,
                            ),
                        ) as std::os::raw::c_int
                            as std::os::raw::c_int
                    }
                    len -= 1;
                    cstr_cat(
                        &mut initstr,
                        tokc.str_0.data as *const std::os::raw::c_char,
                        tokc.str_0.size,
                    );
                    next();
                }
                if tok != ')' as i32
                    && tok != '}' as i32
                    && tok != ',' as i32
                    && tok != ';' as i32
                    && tok != -(1 as std::os::raw::c_int)
                {
                    /* Not a lone literal but part of a bigger expression.  */
                    unget_tok(if size1 == 1 as std::os::raw::c_int {
                        0xc8 as std::os::raw::c_int
                    } else {
                        0xc9 as std::os::raw::c_int
                    });
                    tokc.str_0.size = initstr.size;
                    tokc.str_0.data = initstr.data;
                    current_block_95 = 473863394719183311;
                } else {
                    decl_design_flex(p, s, len);
                    if flags & 2 as std::os::raw::c_int == 0 {
                        let mut nb: std::os::raw::c_int = n;
                        if len < nb {
                            nb = len
                        }
                        if len > nb {
                            _tcc_warning(
                                b"initializer-string for array is too long\x00" as *const u8
                                    as *const std::os::raw::c_char,
                            );
                        }
                        /* in order to go faster for common case (char
                        string in global variable, we handle it
                        specifically */
                        if !(*p).sec.is_null() && size1 == 1 as std::os::raw::c_int {
                            init_assert(
                                p,
                                c.wrapping_add(nb as std::os::raw::c_ulong) as std::os::raw::c_int,
                            );
                            if !(nocode_wanted > 0 as std::os::raw::c_int) {
                                memcpy(
                                    (*(*p).sec).data.offset(c as isize)
                                        as *mut std::os::raw::c_void,
                                    initstr.data,
                                    nb as std::os::raw::c_ulong,
                                );
                            }
                        } else {
                            i = 0 as std::os::raw::c_int;
                            while i < n {
                                if i >= nb {
                                    /* only add trailing zero if enough storage (no
                                    warning in this case since it is standard) */
                                    if flags & 8 as std::os::raw::c_int != 0 {
                                        break;
                                    }
                                    if n - i >= 4 as std::os::raw::c_int {
                                        init_putz(
                                            p,
                                            c.wrapping_add((i * size1) as std::os::raw::c_ulong),
                                            (n - i) * size1,
                                        );
                                        break;
                                    } else {
                                        ch = 0 as std::os::raw::c_int
                                    }
                                } else if size1 == 1 as std::os::raw::c_int {
                                    ch = *(initstr.data as *mut std::os::raw::c_uchar)
                                        .offset(i as isize)
                                        as std::os::raw::c_int
                                } else {
                                    ch = *(initstr.data as *mut nwchar_t).offset(i as isize)
                                }
                                vpushi(ch);
                                init_putv(
                                    p,
                                    t1,
                                    c.wrapping_add((i * size1) as std::os::raw::c_ulong),
                                );
                                i += 1
                            }
                        }
                    }
                    current_block_95 = 18325745679564279244;
                }
            } else {
                current_block_95 = 473863394719183311;
            }
            match current_block_95 {
                18325745679564279244 => {},
                _ => {
                    indexsym.c2rust_unnamed.c2rust_unnamed.c = 0 as std::os::raw::c_int;
                    f = &mut indexsym;
                    current_block_95 = 1266639353313098561;
                },
            }
        } else if (*type_0).t & 0xf as std::os::raw::c_int == 7 as std::os::raw::c_int {
            no_oblock = 1 as std::os::raw::c_int;
            if flags & 1 as std::os::raw::c_int != 0 || tok == '{' as i32 {
                skip('{' as i32);
                no_oblock = 0 as std::os::raw::c_int
            }
            s = (*type_0).ref_0;
            f = (*s).c2rust_unnamed_0.next;
            n = (*s).c2rust_unnamed.c2rust_unnamed.c;
            size1 = 1 as std::os::raw::c_int;
            current_block_95 = 1266639353313098561;
        } else if tok == '{' as i32 {
            if flags & 4 as std::os::raw::c_int != 0 {
                skip(';' as i32);
            }
            next();
            decl_initializer(p, type_0, c, flags & !(4 as std::os::raw::c_int));
            skip('}' as i32);
            current_block_95 = 7019009297990327870;
        } else if flags & 2 as std::os::raw::c_int != 0 {
            /* If we supported only ISO C we wouldn't have to accept calling
            this on anything than an array if DIF_SIZE_ONLY (and even then
            only on the outermost level, so no recursion would be needed),
            because initializing a flex array member isn't supported.
            But GNU C supports it, so we need to recurse even into
            subfields of structs and arrays when DIF_SIZE_ONLY is set.  */
            /* just skip expression */
            skip_or_save_block(0 as *mut *mut TokenString);
            current_block_95 = 7019009297990327870;
        } else {
            if flags & 4 as std::os::raw::c_int == 0 {
                /* This should happen only when we haven't parsed
                the init element above for fear of committing a
                string constant to memory too early.  */
                if tok != 0xc8 as std::os::raw::c_int && tok != 0xc9 as std::os::raw::c_int {
                    expect(b"string constant\x00" as *const u8 as *const std::os::raw::c_char);
                }
                parse_init_elem(if (*p).sec.is_null() {
                    2 as std::os::raw::c_int
                } else {
                    1 as std::os::raw::c_int
                });
            }
            current_block_95 = 11266845908400809545;
        }
        match current_block_95 {
            11266845908400809545 => {},
            7019009297990327870 => {},
            _ => {
                match current_block_95 {
                    1266639353313098561 => {
                        /* zero memory once in advance */
                        if flags & (8 as std::os::raw::c_int | 2 as std::os::raw::c_int) == 0 {
                            init_putz(p, c, n * size1);
                            flags |= 8 as std::os::raw::c_int
                        }
                        len = 0 as std::os::raw::c_int;
                        while tok != '}' as i32 || flags & 4 as std::os::raw::c_int != 0 {
                            len = decl_designator(p, type_0, c, &mut f, flags, len);
                            flags &= !(4 as std::os::raw::c_int);
                            if (*type_0).t & 0x40 as std::os::raw::c_int != 0 {
                                indexsym.c2rust_unnamed.c2rust_unnamed.c += 1;
                                /* special test for multi dimensional arrays (may not
                                be strictly correct if designators are used at the
                                same time) */
                                if no_oblock != 0 && len >= n * size1 {
                                    break;
                                }
                            } else {
                                if (*s).type_0.t
                                    == (1 as std::os::raw::c_int) << 20 as std::os::raw::c_int
                                        | 7 as std::os::raw::c_int
                                {
                                    f = 0 as *mut Sym
                                } else {
                                    f = (*f).c2rust_unnamed_0.next
                                }
                                if no_oblock != 0 && f.is_null() {
                                    break;
                                }
                            }
                            if tok == '}' as i32 {
                                break;
                            }
                            skip(',' as i32);
                        }
                    },
                    _ => {},
                }
                if no_oblock == 0 {
                    skip('}' as i32);
                }
                current_block_95 = 7019009297990327870;
            },
        }
    }
    match current_block_95 {
        11266845908400809545 => {
            if (*p).sec.is_null()
                && flags & 8 as std::os::raw::c_int != 0
                && (*vtop).r as std::os::raw::c_int
                    & (0x3f as std::os::raw::c_int
                        | 0x100 as std::os::raw::c_int
                        | 0x200 as std::os::raw::c_int)
                    == 0x30 as std::os::raw::c_int
                && (*vtop).c2rust_unnamed.c.i == 0 as std::os::raw::c_int as std::os::raw::c_ulong
                && btype_size((*type_0).t & 0xf as std::os::raw::c_int) != 0
            {
                /* not for fp constants */
                vpop();
            } else {
                init_putv(p, type_0, c);
            }
        },
        _ => {},
    };
}
/* parse an initializer for type 't' if 'has_init' is non zero, and
allocate space in local or global data space ('r' is either
VT_LOCAL or VT_CONST). If 'v' is non zero, then an associated
variable 'v' of scope 'scope' is declared before initializers
are parsed. If 'v' is zero, then a reference to the new object
is put in the value stack. If 'has_init' is 2, a special parsing
is done to handle string constants. */
unsafe extern "C" fn decl_initializer_alloc(
    mut type_0: *mut CType,
    mut ad: *mut AttributeDef,
    mut r: std::os::raw::c_int,
    mut has_init: std::os::raw::c_int,
    mut v: std::os::raw::c_int,
    mut scope: std::os::raw::c_int,
) {
    let mut current_block: u64;
    let mut size: std::os::raw::c_int = 0;
    let mut align: std::os::raw::c_int = 0;
    let mut addr: std::os::raw::c_int = 0;
    let mut init_str: *mut TokenString = 0 as *mut TokenString;
    let mut sec: *mut Section = 0 as *mut Section;
    let mut flexible_array: *mut Sym = 0 as *mut Sym;
    let mut sym: *mut Sym = 0 as *mut Sym;
    let mut saved_nocode_wanted: std::os::raw::c_int = nocode_wanted;
    let mut bcheck: std::os::raw::c_int = ((*tcc_state).do_bounds_check as std::os::raw::c_int != 0
        && !(nocode_wanted > 0 as std::os::raw::c_int))
        as std::os::raw::c_int;
    let mut p: init_params = {
        let mut init = init_params {
            sec: 0 as *mut Section,
            local_offset: 0,
            flex_array_ref: 0 as *mut Sym,
        };
        init
    };
    /* Always allocate static or global variables */
    if v != 0 && r & 0x3f as std::os::raw::c_int == 0x30 as std::os::raw::c_int {
        nocode_wanted = (nocode_wanted as std::os::raw::c_uint | 0x80000000 as std::os::raw::c_uint)
            as std::os::raw::c_int
    }
    flexible_array = 0 as *mut Sym;
    size = type_size(type_0, &mut align);
    /* exactly one flexible array may be initialized, either the
    toplevel array or the last member of the toplevel struct */
    if size < 0 as std::os::raw::c_int {
        /* If the base type itself was an array type of unspecified size
        (like in 'typedef int arr[]; arr x = {1};') then we will
        overwrite the unknown size by the real one for this decl.
        We need to unshare the ref symbol holding that size. */
        (*type_0).ref_0 = sym_push(
            0x20000000 as std::os::raw::c_int,
            &mut (*(*type_0).ref_0).type_0,
            0 as std::os::raw::c_int,
            (*(*type_0).ref_0).c2rust_unnamed.c2rust_unnamed.c,
        );
        p.flex_array_ref = (*type_0).ref_0
    } else if has_init != 0 && (*type_0).t & 0xf as std::os::raw::c_int == 7 as std::os::raw::c_int
    {
        let mut field: *mut Sym = (*(*type_0).ref_0).c2rust_unnamed_0.next;
        if !field.is_null() {
            while !(*field).c2rust_unnamed_0.next.is_null() {
                field = (*field).c2rust_unnamed_0.next
            }
            if (*field).type_0.t & 0x40 as std::os::raw::c_int != 0
                && (*(*field).type_0.ref_0).c2rust_unnamed.c2rust_unnamed.c
                    < 0 as std::os::raw::c_int
            {
                flexible_array = field;
                p.flex_array_ref = (*field).type_0.ref_0;
                size = -(1 as std::os::raw::c_int)
            }
        }
    }
    if size < 0 as std::os::raw::c_int {
        /* If unknown size, do a dry-run 1st pass */
        if has_init == 0 {
            _tcc_error(b"unknown type size\x00" as *const u8 as *const std::os::raw::c_char);
        }
        if has_init == 2 as std::os::raw::c_int {
            /* only get strings */
            init_str = tok_str_alloc();
            while tok == 0xc8 as std::os::raw::c_int || tok == 0xc9 as std::os::raw::c_int {
                tok_str_add_tok(init_str);
                next();
            }
            tok_str_add(init_str, -(1 as std::os::raw::c_int));
            tok_str_add(init_str, 0 as std::os::raw::c_int);
        } else {
            skip_or_save_block(&mut init_str);
        }
        unget_tok(0 as std::os::raw::c_int);
        /* compute size */
        begin_macro(init_str, 1 as std::os::raw::c_int);
        next();
        decl_initializer(
            &mut p,
            type_0,
            0 as std::os::raw::c_int as std::os::raw::c_ulong,
            1 as std::os::raw::c_int | 2 as std::os::raw::c_int,
        );
        /* prepare second initializer parsing */
        macro_ptr = (*init_str).str_0;
        next();
        /* if still unknown size, error */
        size = type_size(type_0, &mut align);
        if size < 0 as std::os::raw::c_int {
            _tcc_error(b"unknown type size\x00" as *const u8 as *const std::os::raw::c_char);
        }
        /* If there's a flex member and it was used in the initializer
        adjust size.  */
        if !flexible_array.is_null()
            && (*(*flexible_array).type_0.ref_0)
                .c2rust_unnamed
                .c2rust_unnamed
                .c
                > 0 as std::os::raw::c_int
        {
            size += (*(*flexible_array).type_0.ref_0)
                .c2rust_unnamed
                .c2rust_unnamed
                .c
                * pointed_size(&mut (*flexible_array).type_0)
        }
    }
    /* take into account specified alignment if bigger */
    if (*ad).a.aligned() != 0 {
        let mut speca: std::os::raw::c_int = (1 as std::os::raw::c_int)
            << (*ad).a.aligned() as std::os::raw::c_int - 1 as std::os::raw::c_int;
        if speca > align {
            align = speca
        }
    } else if (*ad).a.packed() != 0 {
        align = 1 as std::os::raw::c_int
    }
    if v == 0 && nocode_wanted > 0 as std::os::raw::c_int {
        size = 0 as std::os::raw::c_int;
        align = 1 as std::os::raw::c_int
    }
    if r & 0x3f as std::os::raw::c_int == 0x32 as std::os::raw::c_int {
        sec = 0 as *mut Section;
        if bcheck != 0 && v != 0 {
            /* add padding between stack variables for bound checking */
            loc -= align
        }
        loc = loc - size & -align;
        addr = loc;
        p.local_offset = addr + size;
        if bcheck != 0 && v != 0 {
            /* add padding between stack variables for bound checking */
            loc -= align
        }
        if v != 0 {
            /* local variable */
            if (*ad).asm_label != 0 {
                let mut reg: std::os::raw::c_int = asm_parse_regvar((*ad).asm_label);
                if reg >= 0 as std::os::raw::c_int {
                    r = r & !(0x3f as std::os::raw::c_int) | reg
                }
            }
            sym = sym_push(v, type_0, r, addr);
            if !(*ad).cleanup_func.is_null() {
                (*cur_scope).cl.n += 1;
                let mut cls: *mut Sym = sym_push2(
                    &mut all_cleanups,
                    0x20000000 as std::os::raw::c_int | (*cur_scope).cl.n,
                    0 as std::os::raw::c_int,
                    0 as std::os::raw::c_int,
                );
                (*cls).prev_tok = sym;
                (*cls).c2rust_unnamed_0.next = (*ad).cleanup_func;
                (*cls).c2rust_unnamed.ncl = (*cur_scope).cl.s;
                (*cur_scope).cl.s = cls
            }
            (*sym).a = (*ad).a
        } else {
            /* push local reference */
            vset(type_0, r, addr);
        }
        current_block = 228501038991332163;
    } else {
        sym = 0 as *mut Sym;
        if v != 0 && scope == 0x30 as std::os::raw::c_int {
            /* see if the symbol was already defined */
            sym = sym_find(v);
            if !sym.is_null() {
                if !p.flex_array_ref.is_null()
                    && (*sym).type_0.t & (*type_0).t & 0x40 as std::os::raw::c_int != 0
                    && (*(*sym).type_0.ref_0).c2rust_unnamed.c2rust_unnamed.c
                        > (*(*type_0).ref_0).c2rust_unnamed.c2rust_unnamed.c
                {
                    /* flex array was already declared with explicit size
                    extern int arr[10];
                    int arr[] = { 1,2,3 }; */
                    (*(*type_0).ref_0).c2rust_unnamed.c2rust_unnamed.c =
                        (*(*sym).type_0.ref_0).c2rust_unnamed.c2rust_unnamed.c;
                    size = type_size(type_0, &mut align)
                }
                patch_storage(sym, ad, type_0);
                /* we accept several definitions of the same global variable. */
                if has_init == 0
                    && (*sym).c2rust_unnamed.c2rust_unnamed.c != 0
                    && (*elfsym(sym)).st_shndx as std::os::raw::c_int != 0 as std::os::raw::c_int
                {
                    current_block = 2303068089157008420;
                } else {
                    current_block = 11869735117417356968;
                }
            } else {
                current_block = 11869735117417356968;
            }
        } else {
            current_block = 11869735117417356968;
        }
        match current_block {
            2303068089157008420 => {},
            _ => {
                /* allocate symbol in corresponding section */
                sec = (*ad).section;
                if sec.is_null() {
                    let mut tp: *mut CType = type_0;
                    while (*tp).t & (0xf as std::os::raw::c_int | 0x40 as std::os::raw::c_int)
                        == 5 as std::os::raw::c_int | 0x40 as std::os::raw::c_int
                    {
                        tp = &mut (*(*tp).ref_0).type_0
                    }
                    if (*tp).t & 0x100 as std::os::raw::c_int != 0 {
                        sec = (*tcc_state).rodata_section
                    } else if has_init != 0 {
                        sec = (*tcc_state).data_section
                    /*if (tcc_state->g_debug & 4)
                    tcc_warning("rw data: %s", get_tok_str(v, 0));*/
                    } else if (*tcc_state).nocommon != 0 {
                        sec = (*tcc_state).bss_section
                    }
                }
                if !sec.is_null() {
                    addr = section_add(sec, size as Elf64_Addr, align) as std::os::raw::c_int;
                    /* add padding if bound check */
                    if bcheck != 0 {
                        section_add(
                            sec,
                            1 as std::os::raw::c_int as Elf64_Addr,
                            1 as std::os::raw::c_int,
                        ); /* SHN_COMMON is special, symbol value is align */
                    }
                } else {
                    addr = align;
                    sec = (*tcc_state).common_section
                }
                if v != 0 {
                    if sym.is_null() {
                        sym = sym_push(
                            v,
                            type_0,
                            r | 0x200 as std::os::raw::c_int,
                            0 as std::os::raw::c_int,
                        );
                        patch_storage(sym, ad, 0 as *mut CType);
                    }
                    /* update symbol definition */
                    put_extern_sym(sym, sec, addr as Elf64_Addr, size as std::os::raw::c_ulong);
                } else {
                    /* push global reference */
                    vpush_ref(
                        type_0,
                        sec,
                        addr as std::os::raw::c_ulong,
                        size as std::os::raw::c_ulong,
                    );
                    sym = (*vtop).c2rust_unnamed_0.sym;
                    (*vtop).r = ((*vtop).r as std::os::raw::c_int | r) as std::os::raw::c_ushort
                }
                /* handles bounds now because the symbol must be defined
                before for the relocation */
                if bcheck != 0 {
                    let mut bounds_ptr: *mut Elf64_Addr = 0 as *mut Elf64_Addr;
                    greloca(
                        (*tcc_state).bounds_section,
                        sym,
                        (*(*tcc_state).bounds_section).data_offset,
                        1 as std::os::raw::c_int,
                        0 as std::os::raw::c_int as Elf64_Addr,
                    );
                    /* then add global bound info */
                    bounds_ptr = section_ptr_add(
                        (*tcc_state).bounds_section,
                        (2 as std::os::raw::c_int as std::os::raw::c_ulong).wrapping_mul(
                            ::std::mem::size_of::<Elf64_Addr>() as std::os::raw::c_ulong,
                        ),
                    ) as *mut Elf64_Addr; /* relocated */
                    *bounds_ptr.offset(0 as std::os::raw::c_int as isize) =
                        0 as std::os::raw::c_int as Elf64_Addr;
                    *bounds_ptr.offset(1 as std::os::raw::c_int as isize) = size as Elf64_Addr
                }
                current_block = 228501038991332163;
            },
        }
    }
    match current_block {
        228501038991332163 => {
            if (*type_0).t & 0x400 as std::os::raw::c_int != 0 {
                let mut a: std::os::raw::c_int = 0;
                if !(nocode_wanted > 0 as std::os::raw::c_int) {
                    /* save before-VLA stack pointer if needed */
                    if (*cur_scope).vla.num == 0 as std::os::raw::c_int {
                        if !(*cur_scope).prev.is_null() && (*(*cur_scope).prev).vla.num != 0 {
                            (*cur_scope).vla.locorig = (*(*cur_scope).prev).vla.loc
                        } else {
                            loc -= 8 as std::os::raw::c_int;
                            gen_vla_sp_save(loc);
                            (*cur_scope).vla.locorig = loc
                        }
                    }
                    vla_runtime_type_size(type_0, &mut a);
                    gen_vla_alloc(type_0, a);
                    gen_vla_sp_save(addr);
                    (*cur_scope).vla.loc = addr;
                    (*cur_scope).vla.num += 1
                }
            } else if has_init != 0 {
                p.sec = sec;
                decl_initializer(
                    &mut p,
                    type_0,
                    addr as std::os::raw::c_ulong,
                    1 as std::os::raw::c_int,
                );
                /* patch flexible array member size back to -1, */
                /* for possible subsequent similar declarations */
                if !flexible_array.is_null() {
                    (*(*flexible_array).type_0.ref_0)
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .c = -(1 as std::os::raw::c_int)
                }
            }
        },
        _ => {},
    }
    /* restore parse state if needed */
    if !init_str.is_null() {
        end_macro();
        next();
    }
    nocode_wanted = saved_nocode_wanted;
}
/* parse a function defined by symbol 'sym' and generate its code in
'cur_text_section' */
unsafe extern "C" fn gen_function(mut sym: *mut Sym) {
    let mut f: scope = {
        let mut init = scope {
            prev: 0 as *mut scope,
            vla: C2RustUnnamed_13 {
                loc: 0,
                locorig: 0,
                num: 0,
            },
            cl: C2RustUnnamed_14 {
                s: 0 as *mut Sym,
                n: 0,
            },
            bsym: 0 as *mut std::os::raw::c_int,
            csym: 0 as *mut std::os::raw::c_int,
            lstk: 0 as *mut Sym,
            llstk: 0 as *mut Sym,
        };
        init
    };
    root_scope = &mut f;
    cur_scope = root_scope;
    nocode_wanted = 0 as std::os::raw::c_int;
    ind = (*(*tcc_state).cur_text_section).data_offset as std::os::raw::c_int;
    if (*sym).a.aligned() != 0 {
        let mut newoff: size_t = section_add(
            (*tcc_state).cur_text_section,
            0 as std::os::raw::c_int as Elf64_Addr,
            (1 as std::os::raw::c_int)
                << (*sym).a.aligned() as std::os::raw::c_int - 1 as std::os::raw::c_int,
        );
        gen_fill_nops(newoff.wrapping_sub(ind as std::os::raw::c_ulong) as std::os::raw::c_int);
    }
    /* NOTE: we patch the symbol size later */
    put_extern_sym(
        sym,
        (*tcc_state).cur_text_section,
        ind as Elf64_Addr,
        0 as std::os::raw::c_int as std::os::raw::c_ulong,
    );
    if (*(*sym).type_0.ref_0)
        .c2rust_unnamed
        .c2rust_unnamed
        .c2rust_unnamed
        .f
        .func_ctor()
        != 0
    {
        add_array(
            tcc_state,
            b".init_array\x00" as *const u8 as *const std::os::raw::c_char,
            (*sym).c2rust_unnamed.c2rust_unnamed.c,
        );
    }
    if (*(*sym).type_0.ref_0)
        .c2rust_unnamed
        .c2rust_unnamed
        .c2rust_unnamed
        .f
        .func_dtor()
        != 0
    {
        add_array(
            tcc_state,
            b".fini_array\x00" as *const u8 as *const std::os::raw::c_char,
            (*sym).c2rust_unnamed.c2rust_unnamed.c,
        );
    }
    funcname = get_tok_str((*sym).v, 0 as *mut CValue);
    func_ind = ind;
    func_vt = (*(*sym).type_0.ref_0).type_0;
    func_var = ((*(*sym).type_0.ref_0)
        .c2rust_unnamed
        .c2rust_unnamed
        .c2rust_unnamed
        .f
        .func_type() as std::os::raw::c_int
        == 3 as std::os::raw::c_int) as std::os::raw::c_int;
    /* put debug symbol */
    tcc_debug_funcstart(tcc_state, sym);
    /* push a dummy symbol to enable local sym storage */
    sym_push2(
        &mut local_stack,
        0x20000000 as std::os::raw::c_int,
        0 as std::os::raw::c_int,
        0 as std::os::raw::c_int,
    ); /* for function parameters */
    local_scope = 1 as std::os::raw::c_int;
    gfunc_prolog(sym);
    local_scope = 0 as std::os::raw::c_int;
    rsym = 0 as std::os::raw::c_int;
    clear_temp_local_var_list();
    block(0 as std::os::raw::c_int);
    gsym(rsym);
    nocode_wanted = 0 as std::os::raw::c_int;
    /* reset local stack */
    pop_local_syms(0 as *mut Sym, 0 as std::os::raw::c_int);
    gfunc_epilog();
    (*(*tcc_state).cur_text_section).data_offset = ind as std::os::raw::c_ulong;
    local_scope = 0 as std::os::raw::c_int;
    label_pop(
        &mut global_label_stack,
        0 as *mut Sym,
        0 as std::os::raw::c_int,
    );
    sym_pop(&mut all_cleanups, 0 as *mut Sym, 0 as std::os::raw::c_int);
    /* patch symbol size */
    (*elfsym(sym)).st_size = (ind - func_ind) as Elf64_Xword;
    /* end of function */
    tcc_debug_funcend(tcc_state, ind - func_ind);
    /* It's better to crash than to generate wrong code */
    (*tcc_state).cur_text_section = 0 as *mut Section; /* for safety */
    funcname = b"\x00" as *const u8 as *const std::os::raw::c_char; /* for safety */
    func_vt.t = 0 as std::os::raw::c_int; /* for safety */
    func_var = 0 as std::os::raw::c_int; /* for safety */
    ind = 0 as std::os::raw::c_int;
    nocode_wanted = 0x80000000 as std::os::raw::c_uint as std::os::raw::c_int;
    check_vstack();
    /* do this after funcend debug info */
    next();
}
unsafe extern "C" fn gen_inline_functions(mut s: *mut TCCState) {
    let mut sym: *mut Sym = 0 as *mut Sym;
    let mut inline_generated: std::os::raw::c_int = 0;
    let mut i: std::os::raw::c_int = 0;
    let mut fn_0: *mut InlineFunc = 0 as *mut InlineFunc;
    tcc_open_bf(
        s,
        b":inline:\x00" as *const u8 as *const std::os::raw::c_char,
        0 as std::os::raw::c_int,
    );
    loop
    /* iterate while inline function are referenced */
    {
        inline_generated = 0 as std::os::raw::c_int;
        i = 0 as std::os::raw::c_int;
        while i < (*s).nb_inline_fns {
            fn_0 = *(*s).inline_fns.offset(i as isize);
            sym = (*fn_0).sym;
            if !sym.is_null()
                && ((*sym).c2rust_unnamed.c2rust_unnamed.c != 0
                    || (*sym).type_0.t & 0x8000 as std::os::raw::c_int == 0)
            {
                /* the function was used or forced (and then not internal):
                generate its code and convert it to a normal function */
                (*fn_0).sym = 0 as *mut Sym;
                tcc_debug_putfile(s, (*fn_0).filename.as_mut_ptr());
                begin_macro((*fn_0).func_str, 1 as std::os::raw::c_int);
                next();
                (*tcc_state).cur_text_section = (*tcc_state).text_section;
                gen_function(sym);
                end_macro();
                inline_generated = 1 as std::os::raw::c_int
            }
            i += 1
        }
        if !(inline_generated != 0) {
            break;
        }
    }
    tcc_close();
}
unsafe extern "C" fn free_inline_functions(mut s: *mut TCCState) {
    let mut i: std::os::raw::c_int = 0;
    /* free tokens of unused inline functions */
    i = 0 as std::os::raw::c_int;
    while i < (*s).nb_inline_fns {
        let mut fn_0: *mut InlineFunc = *(*s).inline_fns.offset(i as isize);
        if !(*fn_0).sym.is_null() {
            tok_str_free((*fn_0).func_str);
        }
        i += 1
    }
    dynarray_reset(
        &mut (*s).inline_fns as *mut *mut *mut InlineFunc as *mut std::os::raw::c_void,
        &mut (*s).nb_inline_fns,
    );
}
/* 'l' is VT_LOCAL or VT_CONST to define default storage type, or VT_CMP
if parsing old style parameter decl list (and FUNC_SYM is set then) */
unsafe extern "C" fn decl0(
    mut l: std::os::raw::c_int,
    mut is_for_loop_init: std::os::raw::c_int,
    mut func_sym: *mut Sym,
) -> std::os::raw::c_int {
    let mut v: std::os::raw::c_int = 0;
    let mut has_init: std::os::raw::c_int = 0;
    let mut r: std::os::raw::c_int = 0;
    let mut oldint: std::os::raw::c_int = 0;
    let mut type_0: CType = CType {
        t: 0,
        ref_0: 0 as *const Sym as *mut Sym,
    };
    let mut btype: CType = CType {
        t: 0,
        ref_0: 0 as *const Sym as *mut Sym,
    };
    let mut sym: *mut Sym = 0 as *mut Sym;
    let mut ad: AttributeDef = AttributeDef {
        a: SymAttr([0; 2]),
        f: FuncAttr([0; 4]),
        section: 0 as *mut Section,
        cleanup_func: 0 as *mut Sym,
        alias_target: 0,
        asm_label: 0,
        attr_mode: 0,
    };
    let mut adbase: AttributeDef = AttributeDef {
        a: SymAttr([0; 2]),
        f: FuncAttr([0; 4]),
        section: 0 as *mut Section,
        cleanup_func: 0 as *mut Sym,
        alias_target: 0,
        asm_label: 0,
        attr_mode: 0,
    };
    loop {
        if tok == TOK_STATIC_ASSERT as std::os::raw::c_int {
            let mut error_str: CString = CString {
                size: 0,
                data: 0 as *const std::os::raw::c_void as *mut std::os::raw::c_void,
                size_allocated: 0,
            };
            let mut c: std::os::raw::c_int = 0;
            next();
            skip('(' as i32);
            c = expr_const();
            if tok == ')' as i32 {
                if c == 0 {
                    _tcc_error(
                        b"_Static_assert fail\x00" as *const u8 as *const std::os::raw::c_char,
                    );
                }
                next();
            } else {
                skip(',' as i32);
                parse_mult_str(
                    &mut error_str,
                    b"string constant\x00" as *const u8 as *const std::os::raw::c_char,
                );
                if c == 0 as std::os::raw::c_int {
                    _tcc_error(
                        b"%s\x00" as *const u8 as *const std::os::raw::c_char,
                        error_str.data as *mut std::os::raw::c_char,
                    );
                }
                cstr_free(&mut error_str);
                skip(')' as i32);
            }
            skip(';' as i32);
        } else {
            oldint = 0 as std::os::raw::c_int;
            if parse_btype(&mut btype, &mut adbase) == 0 {
                if is_for_loop_init != 0 {
                    return 0 as std::os::raw::c_int;
                }
                /* skip redundant ';' if not in old parameter decl scope */
                if tok == ';' as i32 && l != 0x33 as std::os::raw::c_int {
                    next();
                    continue;
                } else {
                    if l != 0x30 as std::os::raw::c_int {
                        break;
                    }
                    if tok == TOK_ASM1 as std::os::raw::c_int
                        || tok == TOK_ASM2 as std::os::raw::c_int
                        || tok == TOK_ASM3 as std::os::raw::c_int
                    {
                        /* global asm block */
                        asm_global_instr();
                        continue;
                    } else if tok >= TOK_DEFINE as std::os::raw::c_int {
                        /* special test for old K&R protos without explicit int
                        type. Only accepted when defining global data */
                        btype.t = 3 as std::os::raw::c_int;
                        oldint = 1 as std::os::raw::c_int
                    } else {
                        if tok != -(1 as std::os::raw::c_int) {
                            expect(b"declaration\x00" as *const u8 as *const std::os::raw::c_char);
                        }
                        break;
                    }
                }
            }
            if tok == ';' as i32 {
                if btype.t & 0xf as std::os::raw::c_int == 7 as std::os::raw::c_int {
                    v = (*btype.ref_0).v;
                    if v & 0x20000000 as std::os::raw::c_int == 0
                        && v & !(0x40000000 as std::os::raw::c_int)
                            >= 0x10000000 as std::os::raw::c_int
                    {
                        _tcc_warning(
                            b"unnamed struct/union that defines no instances\x00" as *const u8
                                as *const std::os::raw::c_char,
                        );
                    }
                    next();
                    continue;
                } else if btype.t as std::os::raw::c_uint
                    & (((1 as std::os::raw::c_uint)
                        << 6 as std::os::raw::c_int + 6 as std::os::raw::c_int)
                        .wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_uint)
                        << 20 as std::os::raw::c_int
                        | 0x80 as std::os::raw::c_int as std::os::raw::c_uint)
                    == ((2 as std::os::raw::c_int) << 20 as std::os::raw::c_int)
                        as std::os::raw::c_uint
                {
                    next();
                    continue;
                }
            }
            loop {
                /* iterate thru each declaration */
                type_0 = btype;
                ad = adbase;
                type_decl(&mut type_0, &mut ad, &mut v, 2 as std::os::raw::c_int);
                if type_0.t & 0xf as std::os::raw::c_int == 6 as std::os::raw::c_int {
                    if type_0.t & 0x2000 as std::os::raw::c_int != 0
                        && l == 0x32 as std::os::raw::c_int
                    {
                        _tcc_error(
                            b"function without file scope cannot be static\x00" as *const u8
                                as *const std::os::raw::c_char,
                        );
                    }
                    /* if old style function prototype, we accept a
                    declaration list */
                    sym = type_0.ref_0;
                    if (*sym)
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .f
                        .func_type() as std::os::raw::c_int
                        == 2 as std::os::raw::c_int
                        && l == 0x30 as std::os::raw::c_int
                    {
                        decl0(0x33 as std::os::raw::c_int, 0 as std::os::raw::c_int, sym);
                    }
                    /* always compile 'extern inline' */
                    if type_0.t & 0x1000 as std::os::raw::c_int != 0 {
                        type_0.t &= !(0x8000 as std::os::raw::c_int)
                    }
                } else if oldint != 0 {
                    _tcc_warning(
                        b"type defaults to int\x00" as *const u8 as *const std::os::raw::c_char,
                    );
                }
                if (*tcc_state).gnu_ext as std::os::raw::c_int != 0
                    && (tok == TOK_ASM1 as std::os::raw::c_int
                        || tok == TOK_ASM2 as std::os::raw::c_int
                        || tok == TOK_ASM3 as std::os::raw::c_int)
                {
                    ad.asm_label = asm_label_instr();
                    /* parse one last attribute list, after asm label */
                    parse_attribute(&mut ad);
                }
                if tok == '{' as i32 {
                    if l != 0x30 as std::os::raw::c_int {
                        _tcc_error(
                            b"cannot use local functions\x00" as *const u8
                                as *const std::os::raw::c_char,
                        );
                    }
                    if type_0.t & 0xf as std::os::raw::c_int != 6 as std::os::raw::c_int {
                        expect(
                            b"function definition\x00" as *const u8 as *const std::os::raw::c_char,
                        );
                    }
                    /* reject abstract declarators in function definition
                    make old style params without decl have int type */
                    sym = type_0.ref_0;
                    loop {
                        sym = (*sym).c2rust_unnamed_0.next;
                        if sym.is_null() {
                            break;
                        }
                        if (*sym).v & !(0x20000000 as std::os::raw::c_int) == 0 {
                            expect(b"identifier\x00" as *const u8 as *const std::os::raw::c_char);
                        }
                        if (*sym).type_0.t == 0 as std::os::raw::c_int {
                            (*sym).type_0 = int_type
                        }
                    }
                    /* apply post-declaraton attributes */
                    merge_funcattr(
                        &mut (*type_0.ref_0)
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .f,
                        &mut ad.f,
                    );
                    /* put function symbol */
                    type_0.t &= !(0x1000 as std::os::raw::c_int);
                    sym = external_sym(v, &mut type_0, 0 as std::os::raw::c_int, &mut ad);
                    /* static inline functions are just recorded as a kind
                    of macro. Their code will be emitted at the end of
                    the compilation unit only if they are used */
                    if (*sym).type_0.t & 0x8000 as std::os::raw::c_int != 0 {
                        let mut fn_0: *mut InlineFunc = 0 as *mut InlineFunc;
                        fn_0 = tcc_malloc(
                            (::std::mem::size_of::<InlineFunc>() as std::os::raw::c_ulong)
                                .wrapping_add(strlen((*file).filename.as_mut_ptr())),
                        ) as *mut InlineFunc;
                        strcpy((*fn_0).filename.as_mut_ptr(), (*file).filename.as_mut_ptr());
                        (*fn_0).sym = sym;
                        skip_or_save_block(&mut (*fn_0).func_str);
                        dynarray_add(
                            &mut (*tcc_state).inline_fns as *mut *mut *mut InlineFunc
                                as *mut std::os::raw::c_void,
                            &mut (*tcc_state).nb_inline_fns,
                            fn_0 as *mut std::os::raw::c_void,
                        );
                    } else {
                        /* compute text section */
                        (*tcc_state).cur_text_section = ad.section;
                        if (*tcc_state).cur_text_section.is_null() {
                            (*tcc_state).cur_text_section = (*tcc_state).text_section
                        }
                        gen_function(sym);
                    }
                    break;
                } else {
                    if l == 0x33 as std::os::raw::c_int {
                        let mut current_block_80: u64;
                        /* find parameter in function parameter list */
                        sym = (*func_sym).c2rust_unnamed_0.next;
                        loop {
                            if sym.is_null() {
                                current_block_80 = 10887629115603254199;
                                break;
                            }
                            if (*sym).v & !(0x20000000 as std::os::raw::c_int) == v {
                                current_block_80 = 4327879008955283700;
                                break;
                            }
                            sym = (*sym).c2rust_unnamed_0.next
                        }
                        match current_block_80 {
                            10887629115603254199 => {
                                _tcc_error(
                                    b"declaration for parameter \'%s\' but no such parameter\x00"
                                        as *const u8
                                        as *const std::os::raw::c_char,
                                    get_tok_str(v, 0 as *mut CValue),
                                );
                            },
                            _ => {
                                if type_0.t
                                    & (0x1000 as std::os::raw::c_int
                                        | 0x2000 as std::os::raw::c_int
                                        | 0x4000 as std::os::raw::c_int
                                        | 0x8000 as std::os::raw::c_int)
                                    != 0
                                {
                                    /* 'register' is okay */
                                    _tcc_error(
                                        b"storage class specified for \'%s\'\x00" as *const u8
                                            as *const std::os::raw::c_char,
                                        get_tok_str(v, 0 as *mut CValue),
                                    );
                                }
                                if (*sym).type_0.t != 0 as std::os::raw::c_int {
                                    _tcc_error(
                                        b"redefinition of parameter \'%s\'\x00" as *const u8
                                            as *const std::os::raw::c_char,
                                        get_tok_str(v, 0 as *mut CValue),
                                    );
                                }
                                convert_parameter_type(&mut type_0);
                                (*sym).type_0 = type_0
                            },
                        }
                    } else if type_0.t & 0x4000 as std::os::raw::c_int != 0 {
                        /* save typedefed type  */
                        /* XXX: test storage specifiers ? */
                        sym = sym_find(v);
                        if !sym.is_null()
                            && (*sym)
                                .c2rust_unnamed
                                .c2rust_unnamed
                                .c2rust_unnamed
                                .sym_scope
                                == local_scope
                        {
                            if is_compatible_types(&mut (*sym).type_0, &mut type_0) == 0
                                || (*sym).type_0.t & 0x4000 as std::os::raw::c_int == 0
                            {
                                _tcc_error(
                                    b"incompatible redefinition of \'%s\'\x00" as *const u8
                                        as *const std::os::raw::c_char,
                                    get_tok_str(v, 0 as *mut CValue),
                                );
                            }
                            (*sym).type_0 = type_0
                        } else {
                            sym = sym_push(
                                v,
                                &mut type_0,
                                0 as std::os::raw::c_int,
                                0 as std::os::raw::c_int,
                            )
                        }
                        (*sym).a = ad.a;
                        (*sym).c2rust_unnamed.c2rust_unnamed.c2rust_unnamed.f = ad.f;
                        if debug_modes != 0 {
                            tcc_debug_typedef(tcc_state, sym);
                        }
                    } else if type_0.t & 0xf as std::os::raw::c_int == 0 as std::os::raw::c_int
                        && type_0.t & 0x1000 as std::os::raw::c_int == 0
                    {
                        _tcc_error(
                            b"declaration of void object\x00" as *const u8
                                as *const std::os::raw::c_char,
                        );
                    } else {
                        r = 0 as std::os::raw::c_int;
                        if type_0.t & 0xf as std::os::raw::c_int == 6 as std::os::raw::c_int {
                            /* external function definition */
                            /* specific case for func_call attribute */
                            (*type_0.ref_0)
                                .c2rust_unnamed
                                .c2rust_unnamed
                                .c2rust_unnamed
                                .f = ad.f
                        } else if type_0.t & 0x40 as std::os::raw::c_int == 0 {
                            /* not lvalue if array */
                            r |= 0x100 as std::os::raw::c_int
                        }
                        has_init = (tok == '=' as i32) as std::os::raw::c_int;
                        if has_init != 0 && type_0.t & 0x400 as std::os::raw::c_int != 0 {
                            _tcc_error(
                                b"variable length array cannot be initialized\x00" as *const u8
                                    as *const std::os::raw::c_char,
                            );
                        }
                        if type_0.t & 0x1000 as std::os::raw::c_int != 0
                            && (has_init == 0 || l != 0x30 as std::os::raw::c_int)
                            || type_0.t & 0xf as std::os::raw::c_int == 6 as std::os::raw::c_int
                            || type_0.t & 0x40 as std::os::raw::c_int != 0
                                && has_init == 0
                                && l == 0x30 as std::os::raw::c_int
                                && (*type_0.ref_0).c2rust_unnamed.c2rust_unnamed.c
                                    < 0 as std::os::raw::c_int
                        {
                            /* external variable or function */
                            type_0.t |= 0x1000 as std::os::raw::c_int;
                            sym = external_sym(v, &mut type_0, r, &mut ad);
                            if ad.alias_target != 0 {
                                /* Aliases need to be emitted when their target
                                symbol is emitted, even if perhaps unreferenced.
                                We only support the case where the base is
                                already defined, otherwise we would need
                                deferring to emit the aliases until the end of
                                the compile unit.  */
                                let mut alias_target: *mut Sym = sym_find(ad.alias_target);
                                let mut esym: *mut Elf64_Sym = elfsym(alias_target);
                                if esym.is_null() {
                                    _tcc_error(
                                        b"unsupported forward __alias__ attribute\x00" as *const u8
                                            as *const std::os::raw::c_char,
                                    );
                                }
                                put_extern_sym2(
                                    sym,
                                    (*esym).st_shndx as std::os::raw::c_int,
                                    (*esym).st_value,
                                    (*esym).st_size,
                                    1 as std::os::raw::c_int,
                                );
                            }
                        } else {
                            if type_0.t & 0x2000 as std::os::raw::c_int != 0 {
                                r |= 0x30 as std::os::raw::c_int
                            } else {
                                r |= l
                            }
                            if has_init != 0 {
                                next();
                            } else if l == 0x30 as std::os::raw::c_int {
                                /* uninitialized global variables may be overridden */
                                type_0.t |= 0x1000 as std::os::raw::c_int
                            }
                            decl_initializer_alloc(&mut type_0, &mut ad, r, has_init, v, l);
                        }
                    }
                    if tok != ',' as i32 {
                        if is_for_loop_init != 0 {
                            return 1 as std::os::raw::c_int;
                        }
                        skip(';' as i32);
                        break;
                    } else {
                        next();
                    }
                }
            }
        }
    }
    return 0 as std::os::raw::c_int;
}
unsafe extern "C" fn decl(mut l: std::os::raw::c_int) {
    decl0(l, 0 as std::os::raw::c_int, 0 as *mut Sym);
}
/* ------------------------------------------------------------------------- */
/* ------------------------------------------------------------------------- */
