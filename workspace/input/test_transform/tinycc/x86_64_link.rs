use crate::bitfields::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type sym_version;
    #[no_mangle]
    fn memcpy(
        _: *mut std::os::raw::c_void,
        _: *const std::os::raw::c_void,
        _: std::os::raw::c_ulong,
    ) -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn memcmp(
        _: *const std::os::raw::c_void,
        _: *const std::os::raw::c_void,
        _: std::os::raw::c_ulong,
    ) -> std::os::raw::c_int;
    #[no_mangle]
    fn _tcc_error(fmt: *const std::os::raw::c_char, _: ...) -> !;
    #[no_mangle]
    fn section_ptr_add(sec: *mut Section, size: Elf64_Addr) -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn get_sym_attr(
        s1: *mut TCCState,
        index: std::os::raw::c_int,
        alloc: std::os::raw::c_int,
    ) -> *mut sym_attr;
    #[no_mangle]
    fn tcc_enter_state(s1: *mut TCCState);
}
pub type size_t = std::os::raw::c_ulong;
pub type __uint8_t = std::os::raw::c_uchar;
pub type __uint16_t = std::os::raw::c_ushort;
pub type __int32_t = std::os::raw::c_int;
pub type __uint32_t = std::os::raw::c_uint;
pub type __int64_t = std::os::raw::c_long;
pub type __uint64_t = std::os::raw::c_ulong;
pub type __off_t = std::os::raw::c_long;
pub type __off64_t = std::os::raw::c_long;
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
pub type gotplt_entry = std::os::raw::c_uint;
pub const ALWAYS_GOTPLT_ENTRY: gotplt_entry = 3;
pub const AUTO_GOTPLT_ENTRY: gotplt_entry = 2;
pub const BUILD_GOT_ONLY: gotplt_entry = 1;
pub const NO_GOTPLT_ENTRY: gotplt_entry = 0;
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
unsafe extern "C" fn write64le(mut p: *mut std::os::raw::c_uchar, mut x: uint64_t) {
    write32le(p, x as uint32_t);
    write32le(
        p.offset(4 as std::os::raw::c_int as isize),
        (x >> 32 as std::os::raw::c_int) as uint32_t,
    );
}
#[inline]
unsafe extern "C" fn add32le(mut p: *mut std::os::raw::c_uchar, mut x: int32_t) {
    write32le(p, read32le(p).wrapping_add(x as std::os::raw::c_uint));
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
unsafe extern "C" fn add64le(mut p: *mut std::os::raw::c_uchar, mut x: int64_t) {
    write64le(p, read64le(p).wrapping_add(x as std::os::raw::c_ulong));
}
#[inline]
unsafe extern "C" fn read64le(mut p: *mut std::os::raw::c_uchar) -> uint64_t {
    return read32le(p) as std::os::raw::c_ulong
        | (read32le(p.offset(4 as std::os::raw::c_int as isize)) as uint64_t)
            << 32 as std::os::raw::c_int;
}
/* !TARGET_DEFS_ONLY */
/* Returns 1 for a code relocation, 0 for a data relocation. For unknown
relocations, returns -1. */
#[no_mangle]
pub unsafe extern "C" fn code_reloc(mut reloc_type: std::os::raw::c_int) -> std::os::raw::c_int {
    match reloc_type {
        10 | 11 | 1 | 26 | 29 | 9 | 41 | 42 | 22 | 3 | 27 | 6 | 5 | 8 | 25 | 19 | 20 | 21 | 23 => {
            return 0 as std::os::raw::c_int;
        },
        2 | 24 | 4 | 31 | 7 => return 1 as std::os::raw::c_int,
        _ => {},
    }
    return -(1 as std::os::raw::c_int);
}
/* Returns an enumerator to describe whether and when the relocation needs a
GOT and/or PLT entry to be created. See tcc.h for a description of the
different values. */
#[no_mangle]
pub unsafe extern "C" fn gotplt_entry_type(
    mut reloc_type: std::os::raw::c_int,
) -> std::os::raw::c_int {
    match reloc_type {
        6 | 7 | 5 | 8 => return NO_GOTPLT_ENTRY as std::os::raw::c_int,
        10 | 11 | 1 | 2 | 24 => {
            /* The following relocs wouldn't normally need GOT or PLT
            slots, but we need them for simplicity in the link
            editor part.  See our caller for comments.  */
            return AUTO_GOTPLT_ENTRY as std::os::raw::c_int;
        },
        22 => return BUILD_GOT_ONLY as std::os::raw::c_int,
        3 | 27 | 26 | 29 | 25 | 9 | 41 | 19 | 20 | 21 | 23 | 42 | 4 | 31 => {
            return ALWAYS_GOTPLT_ENTRY as std::os::raw::c_int;
        },
        _ => {},
    }
    return -(1 as std::os::raw::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn create_plt_entry(
    mut s1: *mut TCCState,
    mut got_offset: std::os::raw::c_uint,
    mut attr: *mut sym_attr,
) -> std::os::raw::c_uint {
    let mut plt: *mut Section = (*s1).plt;
    let mut p: *mut uint8_t = 0 as *mut uint8_t;
    let mut modrm: std::os::raw::c_int = 0;
    let mut plt_offset: std::os::raw::c_uint = 0;
    let mut relofs: std::os::raw::c_uint = 0;
    modrm = 0x25 as std::os::raw::c_int;
    /* empty PLT: create PLT0 entry that pushes the library identifier
    (GOT + PTR_SIZE) and jumps to ld.so resolution routine
    (GOT + 2 * PTR_SIZE) */
    if (*plt).data_offset == 0 as std::os::raw::c_int as std::os::raw::c_ulong {
        p = section_ptr_add(plt, 16 as std::os::raw::c_int as Elf64_Addr) as *mut uint8_t; /* pushl got + PTR_SIZE */
        *p.offset(0 as std::os::raw::c_int as isize) = 0xff as std::os::raw::c_int as uint8_t; /* jmp *(got + PTR_SIZE * 2) */
        *p.offset(1 as std::os::raw::c_int as isize) =
            (modrm + 0x10 as std::os::raw::c_int) as uint8_t;
        write32le(
            p.offset(2 as std::os::raw::c_int as isize),
            8 as std::os::raw::c_int as uint32_t,
        );
        *p.offset(6 as std::os::raw::c_int as isize) = 0xff as std::os::raw::c_int as uint8_t;
        *p.offset(7 as std::os::raw::c_int as isize) = modrm as uint8_t;
        write32le(
            p.offset(8 as std::os::raw::c_int as isize),
            (8 as std::os::raw::c_int * 2 as std::os::raw::c_int) as uint32_t,
        );
    }
    plt_offset = (*plt).data_offset as std::os::raw::c_uint;
    /* The PLT slot refers to the relocation entry it needs via offset.
    The reloc entry is created below, so its offset is the current
    data_offset */
    relofs = if !(*(*s1).plt).reloc.is_null() {
        (*(*(*s1).plt).reloc).data_offset
    } else {
        0 as std::os::raw::c_int as std::os::raw::c_ulong
    } as std::os::raw::c_uint;
    /* Jump to GOT entry where ld.so initially put the address of ip + 4 */
    p = section_ptr_add(plt, 16 as std::os::raw::c_int as Elf64_Addr) as *mut uint8_t; /* jmp *(got + x) */
    *p.offset(0 as std::os::raw::c_int as isize) = 0xff as std::os::raw::c_int as uint8_t; /* push $xxx */
    *p.offset(1 as std::os::raw::c_int as isize) = modrm as uint8_t;
    write32le(p.offset(2 as std::os::raw::c_int as isize), got_offset);
    *p.offset(6 as std::os::raw::c_int as isize) = 0x68 as std::os::raw::c_int as uint8_t;
    /* On x86-64, the relocation is referred to by _index_ */
    write32le(
        p.offset(7 as std::os::raw::c_int as isize),
        (relofs as std::os::raw::c_ulong)
            .wrapping_div(::std::mem::size_of::<Elf64_Rela>() as std::os::raw::c_ulong)
            .wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong) as uint32_t,
    ); /* jmp plt_start */
    *p.offset(11 as std::os::raw::c_int as isize) = 0xe9 as std::os::raw::c_int as uint8_t;
    write32le(
        p.offset(12 as std::os::raw::c_int as isize),
        (*plt).data_offset.wrapping_neg() as uint32_t,
    );
    return plt_offset;
}
/* relocate the PLT: compute addresses and offsets in the PLT now that final
address for PLT and GOT are known (see fill_program_header) */
#[no_mangle]
pub unsafe extern "C" fn relocate_plt(mut s1: *mut TCCState) {
    let mut p: *mut uint8_t = 0 as *mut uint8_t;
    let mut p_end: *mut uint8_t = 0 as *mut uint8_t;
    if (*s1).plt.is_null() {
        return;
    }
    p = (*(*s1).plt).data;
    p_end = p.offset((*(*s1).plt).data_offset as isize);
    if p < p_end {
        let mut x: std::os::raw::c_int = (*(*s1).got)
            .sh_addr
            .wrapping_sub((*(*s1).plt).sh_addr)
            .wrapping_sub(6 as std::os::raw::c_int as std::os::raw::c_ulong)
            as std::os::raw::c_int;
        add32le(p.offset(2 as std::os::raw::c_int as isize), x);
        add32le(
            p.offset(8 as std::os::raw::c_int as isize),
            x - 6 as std::os::raw::c_int,
        );
        p = p.offset(16 as std::os::raw::c_int as isize);
        while p < p_end {
            add32le(
                p.offset(2 as std::os::raw::c_int as isize),
                (x as std::os::raw::c_long
                    + (*(*s1).plt).data.offset_from(p) as std::os::raw::c_long)
                    as int32_t,
            );
            p = p.offset(16 as std::os::raw::c_int as isize)
        }
    }
    if !(*(*s1).plt).reloc.is_null() {
        let mut rel: *mut Elf64_Rela = 0 as *mut Elf64_Rela;
        let mut x_0: std::os::raw::c_int = (*(*s1).plt)
            .sh_addr
            .wrapping_add(16 as std::os::raw::c_int as std::os::raw::c_ulong)
            .wrapping_add(6 as std::os::raw::c_int as std::os::raw::c_ulong)
            as std::os::raw::c_int;
        p = (*(*s1).got).data;
        rel = ((*(*(*s1).plt).reloc).data as *mut Elf64_Rela)
            .offset(0 as std::os::raw::c_int as isize);
        while rel
            < (*(*(*s1).plt).reloc)
                .data
                .offset((*(*(*s1).plt).reloc).data_offset as isize) as *mut Elf64_Rela
        {
            write64le(p.offset((*rel).r_offset as isize), x_0 as uint64_t);
            x_0 += 16 as std::os::raw::c_int;
            rel = rel.offset(1)
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn relocate(
    mut s1: *mut TCCState,
    mut rel: *mut Elf64_Rela,
    mut type_0: std::os::raw::c_int,
    mut ptr: *mut std::os::raw::c_uchar,
    mut addr: Elf64_Addr,
    mut val: Elf64_Addr,
) {
    let mut sym_index: std::os::raw::c_int = 0;
    let mut esym_index: std::os::raw::c_int = 0;
    sym_index = ((*rel).r_info >> 32 as std::os::raw::c_int) as std::os::raw::c_int;
    let mut current_block_64: u64;
    match type_0 {
        1 => {
            if (*s1).output_type == 3 as std::os::raw::c_int {
                esym_index = (*get_sym_attr(s1, sym_index, 0 as std::os::raw::c_int)).dyn_index;
                (*(*s1).qrel).r_offset = (*rel).r_offset;
                if esym_index != 0 {
                    (*(*s1).qrel).r_info = ((esym_index as Elf64_Xword)
                        << 32 as std::os::raw::c_int)
                        .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong);
                    (*(*s1).qrel).r_addend = (*rel).r_addend;
                    (*s1).qrel = (*s1).qrel.offset(1);
                    current_block_64 = 10261677128829721533;
                } else {
                    (*(*s1).qrel).r_info = ((0 as std::os::raw::c_int as Elf64_Xword)
                        << 32 as std::os::raw::c_int)
                        .wrapping_add(8 as std::os::raw::c_int as std::os::raw::c_ulong);
                    (*(*s1).qrel).r_addend = read64le(ptr).wrapping_add(val) as Elf64_Sxword;
                    (*s1).qrel = (*s1).qrel.offset(1);
                    current_block_64 = 13586036798005543211;
                }
            } else {
                current_block_64 = 13586036798005543211;
            }
            match current_block_64 {
                10261677128829721533 => {},
                _ => {
                    add64le(ptr, val as int64_t);
                    current_block_64 = 10261677128829721533;
                },
            }
        },
        10 | 11 => {
            if (*s1).output_type == 3 as std::os::raw::c_int {
                /* XXX: this logic may depend on TCC's codegen
                now TCC uses R_X86_64_32 even for a 64bit pointer */
                (*(*s1).qrel).r_offset = (*rel).r_offset;
                (*(*s1).qrel).r_info = ((0 as std::os::raw::c_int as Elf64_Xword)
                    << 32 as std::os::raw::c_int)
                    .wrapping_add(8 as std::os::raw::c_int as std::os::raw::c_ulong);
                /* Use sign extension! */
                (*(*s1).qrel).r_addend = (read32le(ptr) as std::os::raw::c_int
                    as std::os::raw::c_ulong)
                    .wrapping_add(val) as Elf64_Sxword;
                (*s1).qrel = (*s1).qrel.offset(1)
            }
            add32le(ptr, val as int32_t);
            current_block_64 = 10261677128829721533;
        },
        2 => {
            if (*s1).output_type == 3 as std::os::raw::c_int {
                /* DLL relocation */
                esym_index = (*get_sym_attr(s1, sym_index, 0 as std::os::raw::c_int)).dyn_index;
                if esym_index != 0 {
                    (*(*s1).qrel).r_offset = (*rel).r_offset;
                    (*(*s1).qrel).r_info = ((esym_index as Elf64_Xword)
                        << 32 as std::os::raw::c_int)
                        .wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong);
                    /* Use sign extension! */
                    (*(*s1).qrel).r_addend = read32le(ptr) as std::os::raw::c_int
                        as std::os::raw::c_long
                        + (*rel).r_addend;
                    (*s1).qrel = (*s1).qrel.offset(1);
                    current_block_64 = 10261677128829721533;
                } else {
                    current_block_64 = 18179937129981112328;
                }
            } else {
                current_block_64 = 18179937129981112328;
            }
        },
        4 => {
            current_block_64 = 18179937129981112328;
        },
        31 => {
            add64le(
                ptr,
                val.wrapping_sub((*(*s1).got).sh_addr)
                    .wrapping_add((*rel).r_addend as std::os::raw::c_ulong)
                    as int64_t,
            );
            current_block_64 = 10261677128829721533;
        },
        24 => {
            if (*s1).output_type == 3 as std::os::raw::c_int {
                /* DLL relocation */
                esym_index = (*get_sym_attr(s1, sym_index, 0 as std::os::raw::c_int)).dyn_index;
                if esym_index != 0 {
                    (*(*s1).qrel).r_offset = (*rel).r_offset;
                    (*(*s1).qrel).r_info = ((esym_index as Elf64_Xword)
                        << 32 as std::os::raw::c_int)
                        .wrapping_add(24 as std::os::raw::c_int as std::os::raw::c_ulong);
                    (*(*s1).qrel).r_addend = read64le(ptr)
                        .wrapping_add((*rel).r_addend as std::os::raw::c_ulong)
                        as Elf64_Sxword;
                    (*s1).qrel = (*s1).qrel.offset(1);
                    current_block_64 = 10261677128829721533;
                } else {
                    current_block_64 = 12381812505308290051;
                }
            } else {
                current_block_64 = 12381812505308290051;
            }
            match current_block_64 {
                10261677128829721533 => {},
                _ => {
                    add64le(ptr, val.wrapping_sub(addr) as int64_t);
                    current_block_64 = 10261677128829721533;
                },
            }
        },
        6 | 7 => {
            /* They don't need addend */
            write64le(
                ptr,
                val.wrapping_sub((*rel).r_addend as std::os::raw::c_ulong),
            );
            current_block_64 = 10261677128829721533;
        },
        9 | 41 | 42 => {
            add32le(
                ptr,
                (*(*s1).got)
                    .sh_addr
                    .wrapping_sub(addr)
                    .wrapping_add(
                        (*get_sym_attr(s1, sym_index, 0 as std::os::raw::c_int)).got_offset
                            as std::os::raw::c_ulong,
                    )
                    .wrapping_sub(4 as std::os::raw::c_int as std::os::raw::c_ulong)
                    as int32_t,
            );
            current_block_64 = 10261677128829721533;
        },
        26 => {
            add32le(
                ptr,
                (*(*s1).got)
                    .sh_addr
                    .wrapping_sub(addr)
                    .wrapping_add((*rel).r_addend as std::os::raw::c_ulong)
                    as int32_t,
            );
            current_block_64 = 10261677128829721533;
        },
        29 => {
            add64le(
                ptr,
                (*(*s1).got)
                    .sh_addr
                    .wrapping_sub(addr)
                    .wrapping_add((*rel).r_addend as std::os::raw::c_ulong)
                    as int64_t,
            );
            current_block_64 = 10261677128829721533;
        },
        22 => {
            add32le(ptr, val.wrapping_sub((*(*s1).got).sh_addr) as int32_t);
            current_block_64 = 10261677128829721533;
        },
        3 => {
            /* we load the got offset */
            add32le(
                ptr,
                (*get_sym_attr(s1, sym_index, 0 as std::os::raw::c_int)).got_offset as int32_t,
            );
            current_block_64 = 10261677128829721533;
        },
        27 => {
            /* we load the got offset */
            add64le(
                ptr,
                (*get_sym_attr(s1, sym_index, 0 as std::os::raw::c_int)).got_offset as int64_t,
            );
            current_block_64 = 10261677128829721533;
        },
        25 => {
            add64le(ptr, val.wrapping_sub((*(*s1).got).sh_addr) as int64_t);
            current_block_64 = 10261677128829721533;
        },
        19 => {
            static mut expect: [std::os::raw::c_uchar; 16] = [
                0x66 as std::os::raw::c_int as std::os::raw::c_uchar,
                0x48 as std::os::raw::c_int as std::os::raw::c_uchar,
                0x8d as std::os::raw::c_int as std::os::raw::c_uchar,
                0x3d as std::os::raw::c_int as std::os::raw::c_uchar,
                0 as std::os::raw::c_int as std::os::raw::c_uchar,
                0 as std::os::raw::c_int as std::os::raw::c_uchar,
                0 as std::os::raw::c_int as std::os::raw::c_uchar,
                0 as std::os::raw::c_int as std::os::raw::c_uchar,
                0x66 as std::os::raw::c_int as std::os::raw::c_uchar,
                0x66 as std::os::raw::c_int as std::os::raw::c_uchar,
                0x48 as std::os::raw::c_int as std::os::raw::c_uchar,
                0xe8 as std::os::raw::c_int as std::os::raw::c_uchar,
                0 as std::os::raw::c_int as std::os::raw::c_uchar,
                0 as std::os::raw::c_int as std::os::raw::c_uchar,
                0 as std::os::raw::c_int as std::os::raw::c_uchar,
                0 as std::os::raw::c_int as std::os::raw::c_uchar,
            ];
            static mut replace: [std::os::raw::c_uchar; 16] = [
                0x64 as std::os::raw::c_int as std::os::raw::c_uchar,
                0x48 as std::os::raw::c_int as std::os::raw::c_uchar,
                0x8b as std::os::raw::c_int as std::os::raw::c_uchar,
                0x4 as std::os::raw::c_int as std::os::raw::c_uchar,
                0x25 as std::os::raw::c_int as std::os::raw::c_uchar,
                0 as std::os::raw::c_int as std::os::raw::c_uchar,
                0 as std::os::raw::c_int as std::os::raw::c_uchar,
                0 as std::os::raw::c_int as std::os::raw::c_uchar,
                0 as std::os::raw::c_int as std::os::raw::c_uchar,
                0x48 as std::os::raw::c_int as std::os::raw::c_uchar,
                0x8d as std::os::raw::c_int as std::os::raw::c_uchar,
                0x80 as std::os::raw::c_int as std::os::raw::c_uchar,
                0 as std::os::raw::c_int as std::os::raw::c_uchar,
                0 as std::os::raw::c_int as std::os::raw::c_uchar,
                0 as std::os::raw::c_int as std::os::raw::c_uchar,
                0 as std::os::raw::c_int as std::os::raw::c_uchar,
            ];
            if memcmp(
                ptr.offset(-(4 as std::os::raw::c_int as isize)) as *const std::os::raw::c_void,
                expect.as_ptr() as *const std::os::raw::c_void,
                ::std::mem::size_of::<[std::os::raw::c_uchar; 16]>() as std::os::raw::c_ulong,
            ) == 0 as std::os::raw::c_int
            {
                let mut sym: *mut Elf64_Sym = 0 as *mut Elf64_Sym;
                let mut sec: *mut Section = 0 as *mut Section;
                let mut x: int32_t = 0;
                memcpy(
                    ptr.offset(-(4 as std::os::raw::c_int as isize)) as *mut std::os::raw::c_void,
                    replace.as_ptr() as *const std::os::raw::c_void,
                    ::std::mem::size_of::<[std::os::raw::c_uchar; 16]>() as std::os::raw::c_ulong,
                );
                (*rel.offset(1 as std::os::raw::c_int as isize)).r_info =
                    ((0 as std::os::raw::c_int as Elf64_Xword) << 32 as std::os::raw::c_int)
                        .wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong);
                sym = &mut *((*(*s1).symtab_section).data as *mut Elf64_Sym)
                    .offset(sym_index as isize) as *mut Elf64_Sym;
                sec = *(*s1).sections.offset((*sym).st_shndx as isize);
                x = (*sym)
                    .st_value
                    .wrapping_sub((*sec).sh_addr)
                    .wrapping_sub((*sec).data_offset) as int32_t;
                add32le(ptr.offset(8 as std::os::raw::c_int as isize), x);
            } else {
                tcc_enter_state(s1);
                Some(
                    _tcc_error as unsafe extern "C" fn(_: *const std::os::raw::c_char, _: ...) -> !,
                )
                .expect("non-null function pointer")(
                    b"unexpected R_X86_64_TLSGD pattern\x00" as *const u8
                        as *const std::os::raw::c_char,
                );
            }
            current_block_64 = 10261677128829721533;
        },
        20 => {
            static mut expect_0: [std::os::raw::c_uchar; 12] = [
                0x48 as std::os::raw::c_int as std::os::raw::c_uchar,
                0x8d as std::os::raw::c_int as std::os::raw::c_uchar,
                0x3d as std::os::raw::c_int as std::os::raw::c_uchar,
                0 as std::os::raw::c_int as std::os::raw::c_uchar,
                0 as std::os::raw::c_int as std::os::raw::c_uchar,
                0 as std::os::raw::c_int as std::os::raw::c_uchar,
                0 as std::os::raw::c_int as std::os::raw::c_uchar,
                0xe8 as std::os::raw::c_int as std::os::raw::c_uchar,
                0 as std::os::raw::c_int as std::os::raw::c_uchar,
                0 as std::os::raw::c_int as std::os::raw::c_uchar,
                0 as std::os::raw::c_int as std::os::raw::c_uchar,
                0 as std::os::raw::c_int as std::os::raw::c_uchar,
            ];
            static mut replace_0: [std::os::raw::c_uchar; 12] = [
                0x66 as std::os::raw::c_int as std::os::raw::c_uchar,
                0x66 as std::os::raw::c_int as std::os::raw::c_uchar,
                0x66 as std::os::raw::c_int as std::os::raw::c_uchar,
                0x64 as std::os::raw::c_int as std::os::raw::c_uchar,
                0x48 as std::os::raw::c_int as std::os::raw::c_uchar,
                0x8b as std::os::raw::c_int as std::os::raw::c_uchar,
                0x4 as std::os::raw::c_int as std::os::raw::c_uchar,
                0x25 as std::os::raw::c_int as std::os::raw::c_uchar,
                0 as std::os::raw::c_int as std::os::raw::c_uchar,
                0 as std::os::raw::c_int as std::os::raw::c_uchar,
                0 as std::os::raw::c_int as std::os::raw::c_uchar,
                0 as std::os::raw::c_int as std::os::raw::c_uchar,
            ];
            if memcmp(
                ptr.offset(-(3 as std::os::raw::c_int as isize)) as *const std::os::raw::c_void,
                expect_0.as_ptr() as *const std::os::raw::c_void,
                ::std::mem::size_of::<[std::os::raw::c_uchar; 12]>() as std::os::raw::c_ulong,
            ) == 0 as std::os::raw::c_int
            {
                memcpy(
                    ptr.offset(-(3 as std::os::raw::c_int as isize)) as *mut std::os::raw::c_void,
                    replace_0.as_ptr() as *const std::os::raw::c_void,
                    ::std::mem::size_of::<[std::os::raw::c_uchar; 12]>() as std::os::raw::c_ulong,
                );
                (*rel.offset(1 as std::os::raw::c_int as isize)).r_info =
                    ((0 as std::os::raw::c_int as Elf64_Xword) << 32 as std::os::raw::c_int)
                        .wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong)
            } else {
                tcc_enter_state(s1);
                Some(
                    _tcc_error as unsafe extern "C" fn(_: *const std::os::raw::c_char, _: ...) -> !,
                )
                .expect("non-null function pointer")(
                    b"unexpected R_X86_64_TLSLD pattern\x00" as *const u8
                        as *const std::os::raw::c_char,
                );
            }
            current_block_64 = 10261677128829721533;
        },
        21 | 23 => {
            let mut sym_0: *mut Elf64_Sym = 0 as *mut Elf64_Sym;
            let mut sec_0: *mut Section = 0 as *mut Section;
            let mut x_0: int32_t = 0;
            sym_0 = &mut *((*(*s1).symtab_section).data as *mut Elf64_Sym)
                .offset(sym_index as isize) as *mut Elf64_Sym;
            sec_0 = *(*s1).sections.offset((*sym_0).st_shndx as isize);
            x_0 = val
                .wrapping_sub((*sec_0).sh_addr)
                .wrapping_sub((*sec_0).data_offset) as int32_t;
            add32le(ptr, x_0);
            current_block_64 = 10261677128829721533;
        },
        0 | 8 | _ => {
            current_block_64 = 10261677128829721533;
        },
    }
    match current_block_64 {
        18179937129981112328 =>
        /* fallthrough: val already holds the PLT slot address */
        {
            let mut diff: std::os::raw::c_longlong = 0;
            diff = (val as std::os::raw::c_longlong as std::os::raw::c_ulonglong)
                .wrapping_sub(addr as std::os::raw::c_ulonglong)
                as std::os::raw::c_longlong;
            if diff < -(2147483648 as std::os::raw::c_longlong)
                || diff > 2147483647 as std::os::raw::c_longlong
            {
                tcc_enter_state(s1);
                Some(
                    _tcc_error as unsafe extern "C" fn(_: *const std::os::raw::c_char, _: ...) -> !,
                )
                .expect("non-null function pointer")(
                    b"internal error: relocation failed\x00" as *const u8
                        as *const std::os::raw::c_char,
                );
            }
            add32le(ptr, diff as int32_t);
        }
        _ => {},
    };
}
/* !TARGET_DEFS_ONLY */
