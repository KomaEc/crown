use utils::similar;

use crate::type_qualifier::mutability_analysis;

#[test]
fn regression_mutability_libtree() {
    utils::rustc::run_compiler(utils::rustc::SourceCode::Libtree, |program| {
        let mutability_result = mutability_analysis(&program);
        let pretty = mutability_result.pretty(program.tcx);

        let diff = similar::TextDiff::from_lines(GROUND_TRUTH, &pretty);

        let mut any_difference = false;

        for change in diff.iter_all_changes() {
            match change.tag() {
                similar::ChangeTag::Delete => print!("- {}", change),
                similar::ChangeTag::Insert => print!("+ {}", change),
                similar::ChangeTag::Equal => continue,
            };
            any_difference = true
        }

        assert!(!any_difference)
    })
}

/// This is the ground truth obtained by the previous version of crown
const GROUND_TRUTH: &str = "src::libtree::apply_exclude_list: (&read_write, &read_write, &read) -> 
src::libtree::check_absolute_paths: (&read_write, &read_write, , &read_write, ) -> 
src::libtree::check_search_paths: (, , &read_write, &read_write, , &read_write, ) -> 
src::libtree::host_is_little_endian: () -> 
src::libtree::interpolate_variables: (&read_write, , &read) -> 
src::libtree::is_ascending_order: (&read, ) -> 
src::libtree::is_in_exclude_list: (&read) -> 
src::libtree::ld_conf_globbing: (&read_write, &read_write) -> 
src::libtree::libtree_state_free: (&read_write) -> 
src::libtree::libtree_state_init: (&read_write) -> 
src::libtree::main_0: (, &read_write &read_write) -> 
src::libtree::parse_ld_config_file: (&read_write, &read) -> 
src::libtree::parse_ld_library_path: (&read_write) -> 
src::libtree::parse_ld_so_conf: (&read_write) -> 
src::libtree::print_colon_delimited_paths: (&read_write, &read_write) -> 
src::libtree::print_error: (, , &read, &read_write, &read_write, ) -> 
src::libtree::print_line: (, &read_write, &read_write, &read_write, , , &read_write) -> 
src::libtree::print_tree: (, &read_write &read_write, &read_write) -> 
src::libtree::putchar: () -> 
src::libtree::recurse: (&read_write, , &read_write, , ) -> 
src::libtree::set_default_paths: (&read_write) -> 
src::libtree::small_vec_u64_append: (&read_write, ) -> 
src::libtree::small_vec_u64_free: (&read_write) -> 
src::libtree::small_vec_u64_init: (&read_write) -> 
src::libtree::stat: (&read_write, &read_write) -> 
src::libtree::string_table_copy_from_file: (&read_write, &read_write) -> 
src::libtree::string_table_maybe_grow: (&read_write, ) -> 
src::libtree::string_table_store: (&read_write, &read) -> 
src::libtree::tree_preamble: (&read, ) -> 
src::libtree::utoa: (&read_write, ) -> 
src::libtree::visited_files_append: (&read_write, &read) -> 
src::libtree::visited_files_contains: (&read, &read) -> 
src::libtree::_IO_FILE {
  _flags: ,
  _IO_read_ptr: &read,
  _IO_read_end: &read,
  _IO_read_base: &read,
  _IO_write_base: &read,
  _IO_write_ptr: &read,
  _IO_write_end: &read,
  _IO_buf_base: &read,
  _IO_buf_end: &read,
  _IO_save_base: &read,
  _IO_backup_base: &read,
  _IO_save_end: &read,
  _markers: &read,
  _chain: &read,
  _fileno: ,
  _flags2: ,
  _old_offset: ,
  _cur_column: ,
  _vtable_offset: ,
  _shortbuf: ,
  _lock: &read,
  _offset: ,
  __pad1: &read,
  __pad2: &read,
  __pad3: &read,
  __pad4: &read,
  __pad5: ,
  _mode: ,
  _unused2: ,
}
src::libtree::_IO_marker {
  _next: &read,
  _sbuf: &read,
  _pos: ,
}
src::libtree::compat_t {
  any: ,
  class: ,
  machine: ,
}
src::libtree::dyn_32_t {
  d_tag: ,
  d_val: ,
}
src::libtree::dyn_64_t {
  d_tag: ,
  d_val: ,
}
src::libtree::found_t {
  how: ,
  depth: ,
}
src::libtree::glob_t {
  gl_pathc: ,
  gl_pathv: &read &read,
  gl_offs: ,
  gl_flags: ,
  gl_closedir: ,
  gl_readdir: ,
  gl_opendir: ,
  gl_lstat: ,
  gl_stat: ,
}
src::libtree::header_32_t {
  e_type: ,
  e_machine: ,
  e_version: ,
  e_entry: ,
  e_phoff: ,
  e_shoff: ,
  e_flags: ,
  e_ehsize: ,
  e_phentsize: ,
  e_phnum: ,
  e_shentsize: ,
  e_shnum: ,
  e_shstrndx: ,
}
src::libtree::header_64_t {
  e_type: ,
  e_machine: ,
  e_version: ,
  e_entry: ,
  e_phoff: ,
  e_shoff: ,
  e_flags: ,
  e_ehsize: ,
  e_phentsize: ,
  e_phnum: ,
  e_shentsize: ,
  e_shnum: ,
  e_shstrndx: ,
}
src::libtree::libtree_state_t {
  verbosity: ,
  path: ,
  color: ,
  ld_conf_file: &read_write,
  max_depth: ,
  string_table: ,
  visited: ,
  PLATFORM: &read_write,
  LIB: &read_write,
  OSNAME: &read_write,
  OSREL: &read_write,
  rpath_offsets: ,
  ld_library_path_offset: ,
  default_paths_offset: ,
  ld_so_conf_offset: ,
  found_all_needed: ,
}
src::libtree::prog_32_t {
  p_type: ,
  p_offset: ,
  p_vaddr: ,
  p_paddr: ,
  p_filesz: ,
  p_memsz: ,
  p_flags: ,
  p_align: ,
}
src::libtree::prog_64_t {
  p_type: ,
  p_flags: ,
  p_offset: ,
  p_vaddr: ,
  p_paddr: ,
  p_filesz: ,
  p_memsz: ,
  p_align: ,
}
src::libtree::small_vec_u64_t {
  buf: ,
  p: &read_write,
  n: ,
  capacity: ,
}
src::libtree::stat {
  st_dev: ,
  st_ino: ,
  st_nlink: ,
  st_mode: ,
  st_uid: ,
  st_gid: ,
  __pad0: ,
  st_rdev: ,
  st_size: ,
  st_blksize: ,
  st_blocks: ,
  st_atime: ,
  st_atimensec: ,
  st_mtime: ,
  st_mtimensec: ,
  st_ctime: ,
  st_ctimensec: ,
  __glibc_reserved: ,
}
src::libtree::string_table_t {
  arr: &read_write,
  n: ,
  capacity: ,
}
src::libtree::utsname {
  sysname: ,
  nodename: ,
  release: ,
  version: ,
  machine: ,
  __domainname: ,
}
src::libtree::visited_file_array_t {
  arr: &read_write,
  n: ,
  capacity: ,
}
src::libtree::visited_file_t {
  st_dev: ,
  st_ino: ,
}";
