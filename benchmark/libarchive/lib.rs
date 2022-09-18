#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(c_variadic)]
#![feature(const_raw_ptr_to_usize_cast)]
#![feature(const_transmute)]
#![feature(extern_types)]
#![feature(label_break_value)]
#![feature(main)]
#![feature(ptr_wrapping_offset_from)]
#![feature(register_tool)]
#![register_tool(c2rust)]

#[macro_use]
extern crate c2rust_bitfields;
extern crate libc;

pub mod src {
    pub mod cat {
        pub mod bsdcat;
        pub mod cmdline;
    } // mod cat
    pub mod cpio {
        pub mod cmdline;
        pub mod cpio;
    } // mod cpio
    pub mod libarchive {
        pub mod archive_acl;
        pub mod archive_blake2s_ref;
        pub mod archive_blake2sp_ref;
        pub mod archive_check_magic;
        pub mod archive_cmdline;
        pub mod archive_cryptor;
        pub mod archive_digest;
        pub mod archive_entry;
        pub mod archive_entry_copy_stat;
        pub mod archive_entry_link_resolver;
        pub mod archive_entry_sparse;
        pub mod archive_entry_stat;
        pub mod archive_entry_strmode;
        pub mod archive_entry_xattr;
        pub mod archive_getdate;
        pub mod archive_hmac;
        pub mod archive_match;
        pub mod archive_options;
        pub mod archive_pack_dev;
        pub mod archive_pathmatch;
        pub mod archive_ppmd7;
        pub mod archive_ppmd8;
        pub mod archive_random;
        pub mod archive_rb;
        pub mod archive_read;
        pub mod archive_read_add_passphrase;
        pub mod archive_read_append_filter;
        pub mod archive_read_data_into_fd;
        pub mod archive_read_disk_entry_from_file;
        pub mod archive_read_disk_posix;
        pub mod archive_read_disk_set_standard_lookup;
        pub mod archive_read_extract;
        pub mod archive_read_extract2;
        pub mod archive_read_open_fd;
        pub mod archive_read_open_file;
        pub mod archive_read_open_filename;
        pub mod archive_read_open_memory;
        pub mod archive_read_set_format;
        pub mod archive_read_set_options;
        pub mod archive_read_support_filter_all;
        pub mod archive_read_support_filter_bzip2;
        pub mod archive_read_support_filter_compress;
        pub mod archive_read_support_filter_grzip;
        pub mod archive_read_support_filter_gzip;
        pub mod archive_read_support_filter_lrzip;
        pub mod archive_read_support_filter_lz4;
        pub mod archive_read_support_filter_lzop;
        pub mod archive_read_support_filter_none;
        pub mod archive_read_support_filter_program;
        pub mod archive_read_support_filter_rpm;
        pub mod archive_read_support_filter_uu;
        pub mod archive_read_support_filter_xz;
        pub mod archive_read_support_filter_zstd;
        pub mod archive_read_support_format_7zip;
        pub mod archive_read_support_format_all;
        pub mod archive_read_support_format_ar;
        pub mod archive_read_support_format_by_code;
        pub mod archive_read_support_format_cab;
        pub mod archive_read_support_format_cpio;
        pub mod archive_read_support_format_empty;
        pub mod archive_read_support_format_iso9660;
        pub mod archive_read_support_format_lha;
        pub mod archive_read_support_format_mtree;
        pub mod archive_read_support_format_rar;
        pub mod archive_read_support_format_rar5;
        pub mod archive_read_support_format_raw;
        pub mod archive_read_support_format_tar;
        pub mod archive_read_support_format_warc;
        pub mod archive_read_support_format_xar;
        pub mod archive_read_support_format_zip;
        pub mod archive_string;
        pub mod archive_string_sprintf;
        pub mod archive_util;
        pub mod archive_version_details;
        pub mod archive_virtual;
        pub mod archive_write;
        pub mod archive_write_add_filter;
        pub mod archive_write_add_filter_b64encode;
        pub mod archive_write_add_filter_by_name;
        pub mod archive_write_add_filter_bzip2;
        pub mod archive_write_add_filter_compress;
        pub mod archive_write_add_filter_grzip;
        pub mod archive_write_add_filter_gzip;
        pub mod archive_write_add_filter_lrzip;
        pub mod archive_write_add_filter_lz4;
        pub mod archive_write_add_filter_lzop;
        pub mod archive_write_add_filter_none;
        pub mod archive_write_add_filter_program;
        pub mod archive_write_add_filter_uuencode;
        pub mod archive_write_add_filter_xz;
        pub mod archive_write_add_filter_zstd;
        pub mod archive_write_disk_posix;
        pub mod archive_write_disk_set_standard_lookup;
        pub mod archive_write_open_fd;
        pub mod archive_write_open_file;
        pub mod archive_write_open_filename;
        pub mod archive_write_open_memory;
        pub mod archive_write_set_format;
        pub mod archive_write_set_format_7zip;
        pub mod archive_write_set_format_ar;
        pub mod archive_write_set_format_by_name;
        pub mod archive_write_set_format_cpio;
        pub mod archive_write_set_format_cpio_newc;
        pub mod archive_write_set_format_filter_by_ext;
        pub mod archive_write_set_format_gnutar;
        pub mod archive_write_set_format_iso9660;
        pub mod archive_write_set_format_mtree;
        pub mod archive_write_set_format_pax;
        pub mod archive_write_set_format_raw;
        pub mod archive_write_set_format_shar;
        pub mod archive_write_set_format_ustar;
        pub mod archive_write_set_format_v7tar;
        pub mod archive_write_set_format_warc;
        pub mod archive_write_set_format_xar;
        pub mod archive_write_set_format_zip;
        pub mod archive_write_set_options;
        pub mod archive_write_set_passphrase;
        pub mod filter_fork_posix;
        pub mod xxhash;
    } // mod libarchive
    pub mod libarchive_fe {
        pub mod err;
        pub mod line_reader;
        pub mod passphrase;
    } // mod libarchive_fe
    pub mod tar {
        pub mod cmdline;
        pub mod creation_set;
        pub mod read;
        pub mod subst;
        pub mod util;
        pub mod write;
    } // mod tar
} // mod src
