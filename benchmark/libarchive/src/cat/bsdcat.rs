use ::libc;
extern "C" {
    /* Declare our basic types. */
    pub type archive;
    pub type archive_entry;
    /*
     * Detailed textual name/version of the library and its dependencies.
     * This has the form:
     *    "libarchive x.y.z zlib/a.b.c liblzma/d.e.f ... etc ..."
     * the list of libraries described here will vary depending on how
     * libarchive was compiled.
     */
    #[no_mangle]
    fn archive_version_details() -> *const libc::c_char;
    /*
     * Codes to identify various stream filters.
     */
    /*
     * Codes returned by archive_format.
     *
     * Top 16 bits identifies the format family (e.g., "tar"); lower
     * 16 bits indicate the variant.  This is updated by read_next_header.
     * Note that the lower 16 bits will often vary from entry to entry.
     * In some cases, this variation occurs as libarchive learns more about
     * the archive (for example, later entries might utilize extensions that
     * weren't necessary earlier in the archive; in this case, libarchive
     * will change the format code to indicate the extended format that
     * was used).  In other cases, it's because different tools have
     * modified the archive and so different parts of the archive
     * actually have slightly different formats.  (Both tar and cpio store
     * format codes in each entry, so it is quite possible for each
     * entry to be in a different format.)
     */
    /*
     * Codes returned by archive_read_format_capabilities().
     *
     * This list can be extended with values between 0 and 0xffff.
     * The original purpose of this list was to let different archive
     * format readers expose their general capabilities in terms of
     * encryption.
     */
    /* no special capabilities */
    /* reader can detect encrypted data */
    /* reader can detect encryptable metadata (pathname, mtime, etc.) */
    /*
     * Codes returned by archive_read_has_encrypted_entries().
     *
     * In case the archive does not support encryption detection at all
     * ARCHIVE_READ_FORMAT_ENCRYPTION_UNSUPPORTED is returned. If the reader
     * for some other reason (e.g. not enough bytes read) cannot say if
     * there are encrypted entries, ARCHIVE_READ_FORMAT_ENCRYPTION_DONT_KNOW
     * is returned.
     */
    /*-
     * Basic outline for reading an archive:
     *   1) Ask archive_read_new for an archive reader object.
     *   2) Update any global properties as appropriate.
     *      In particular, you'll certainly want to call appropriate
     *      archive_read_support_XXX functions.
     *   3) Call archive_read_open_XXX to open the archive
     *   4) Repeatedly call archive_read_next_header to get information about
     *      successive archive entries.  Call archive_read_data to extract
     *      data for entries of interest.
     *   5) Call archive_read_free to end processing.
     */
    #[no_mangle]
    fn archive_read_new() -> *mut archive;
    #[no_mangle]
    fn archive_read_support_filter_all(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_read_support_format_empty(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_read_support_format_raw(_: *mut archive) -> libc::c_int;
    /*
     * A variety of shortcuts that invoke archive_read_open() with
     * canned callbacks suitable for common situations.  The ones that
     * accept a block size handle tape blocking correctly.
     */
    /* Use this if you know the filename.  Note: NULL indicates stdin. */
    #[no_mangle]
    fn archive_read_open_filename(
        _: *mut archive,
        _filename: *const libc::c_char,
        _block_size: size_t,
    ) -> libc::c_int;
    /* Parses and returns next entry header. */
    #[no_mangle]
    fn archive_read_next_header(_: *mut archive, _: *mut *mut archive_entry) -> libc::c_int;
    #[no_mangle]
    fn archive_read_data_into_fd(_: *mut archive, fd: libc::c_int) -> libc::c_int;
    /* Close the file and release most resources. */
    #[no_mangle]
    fn archive_read_close(_: *mut archive) -> libc::c_int;
    /* Release all resources and destroy the object. */
    /* Note that archive_read_free will call archive_read_close for you. */
    #[no_mangle]
    fn archive_read_free(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    static mut stderr: *mut _IO_FILE;
    #[no_mangle]
    static mut stdout: *mut _IO_FILE;
    #[no_mangle]
    fn archive_error_string(_: *mut archive) -> *const libc::c_char;
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn bsdcat_getopt(_: *mut bsdcat) -> libc::c_int;
    /*-
     * Copyright (c) 2009 Joerg Sonnenberger
     * All rights reserved.
     *
     * Redistribution and use in source and binary forms, with or without
     * modification, are permitted provided that the following conditions
     * are met:
     * 1. Redistributions of source code must retain the above copyright
     *    notice, this list of conditions and the following disclaimer.
     * 2. Redistributions in binary form must reproduce the above copyright
     *    notice, this list of conditions and the following disclaimer in the
     *    documentation and/or other materials provided with the distribution.
     *
     * THIS SOFTWARE IS PROVIDED BY THE AUTHOR(S) ``AS IS'' AND ANY EXPRESS OR
     * IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES
     * OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED.
     * IN NO EVENT SHALL THE AUTHOR(S) BE LIABLE FOR ANY DIRECT, INDIRECT,
     * INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT
     * NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
     * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
     * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
     * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
     * THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
     */
    #[no_mangle]
    fn lafe_warnc(code: libc::c_int, fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn lafe_getprogname() -> *const libc::c_char;
    #[no_mangle]
    fn lafe_setprogname(name: *const libc::c_char, defaultname: *const libc::c_char);
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub __pad1: *mut libc::c_void,
    pub __pad2: *mut libc::c_void,
    pub __pad3: *mut libc::c_void,
    pub __pad4: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
pub type FILE = _IO_FILE;
/*-
 * Copyright (c) 2014, Mike Kazantsev
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE AUTHOR(S) ``AS IS'' AND ANY EXPRESS OR
 * IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES
 * OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED.
 * IN NO EVENT SHALL THE AUTHOR(S) BE LIABLE FOR ANY DIRECT, INDIRECT,
 * INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT
 * NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
 * THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */
/* Not having a config.h of some sort is a serious problem. */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bsdcat {
    pub getopt_state: libc::c_int,
    pub getopt_word: *mut libc::c_char,
    pub argc: libc::c_int,
    pub argv: *mut *mut libc::c_char,
    pub argument: *const libc::c_char,
}
pub type C2RustUnnamed = libc::c_uint;
pub const OPTION_VERSION: C2RustUnnamed = 0;
/*
 * Error codes: Use archive_errno() and archive_error_string()
 * to retrieve details.  Unless specified otherwise, all functions
 * that return 'int' use these codes.
 */
pub const ARCHIVE_EOF: libc::c_int = 1 as libc::c_int;
/* Found end of archive. */
pub const ARCHIVE_OK: libc::c_int = 0 as libc::c_int;
pub const NULL: libc::c_int = 0 as libc::c_int;
/* config.h.  Generated from config.h.in by configure.  */
/* config.h.in.  Generated from configure.ac by autoheader.  */
/* Darwin ACL support */
/* #undef ARCHIVE_ACL_DARWIN */
/* FreeBSD ACL support */
/* #undef ARCHIVE_ACL_FREEBSD */
/* FreeBSD NFSv4 ACL support */
/* #undef ARCHIVE_ACL_FREEBSD_NFS4 */
/* Linux POSIX.1e ACL support via libacl */
/* #undef ARCHIVE_ACL_LIBACL */
/* Linux NFSv4 ACL support via librichacl */
/* #undef ARCHIVE_ACL_LIBRICHACL */
/* Solaris ACL support */
/* #undef ARCHIVE_ACL_SUNOS */
/* Solaris NFSv4 ACL support */
/* #undef ARCHIVE_ACL_SUNOS_NFS4 */
/* MD5 via ARCHIVE_CRYPTO_MD5_LIBC supported. */
/* #undef ARCHIVE_CRYPTO_MD5_LIBC */
/* MD5 via ARCHIVE_CRYPTO_MD5_LIBMD supported. */
/* #undef ARCHIVE_CRYPTO_MD5_LIBMD */
/* MD5 via ARCHIVE_CRYPTO_MD5_LIBSYSTEM supported. */
/* #undef ARCHIVE_CRYPTO_MD5_LIBSYSTEM */
/* MD5 via ARCHIVE_CRYPTO_MD5_MBEDTLS supported. */
/* #undef ARCHIVE_CRYPTO_MD5_MBEDTLS */
/* MD5 via ARCHIVE_CRYPTO_MD5_NETTLE supported. */
/* #undef ARCHIVE_CRYPTO_MD5_NETTLE */
/* MD5 via ARCHIVE_CRYPTO_MD5_OPENSSL supported. */
/* MD5 via ARCHIVE_CRYPTO_MD5_WIN supported. */
/* #undef ARCHIVE_CRYPTO_MD5_WIN */
/* RMD160 via ARCHIVE_CRYPTO_RMD160_LIBC supported. */
/* #undef ARCHIVE_CRYPTO_RMD160_LIBC */
/* RMD160 via ARCHIVE_CRYPTO_RMD160_LIBMD supported. */
/* #undef ARCHIVE_CRYPTO_RMD160_LIBMD */
/* RMD160 via ARCHIVE_CRYPTO_RMD160_MBEDTLS supported. */
/* #undef ARCHIVE_CRYPTO_RMD160_MBEDTLS */
/* RMD160 via ARCHIVE_CRYPTO_RMD160_NETTLE supported. */
/* #undef ARCHIVE_CRYPTO_RMD160_NETTLE */
/* RMD160 via ARCHIVE_CRYPTO_RMD160_OPENSSL supported. */
/* SHA1 via ARCHIVE_CRYPTO_SHA1_LIBC supported. */
/* #undef ARCHIVE_CRYPTO_SHA1_LIBC */
/* SHA1 via ARCHIVE_CRYPTO_SHA1_LIBMD supported. */
/* #undef ARCHIVE_CRYPTO_SHA1_LIBMD */
/* SHA1 via ARCHIVE_CRYPTO_SHA1_LIBSYSTEM supported. */
/* #undef ARCHIVE_CRYPTO_SHA1_LIBSYSTEM */
/* SHA1 via ARCHIVE_CRYPTO_SHA1_MBEDTLS supported. */
/* #undef ARCHIVE_CRYPTO_SHA1_MBEDTLS */
/* SHA1 via ARCHIVE_CRYPTO_SHA1_NETTLE supported. */
/* #undef ARCHIVE_CRYPTO_SHA1_NETTLE */
/* SHA1 via ARCHIVE_CRYPTO_SHA1_OPENSSL supported. */
/* SHA1 via ARCHIVE_CRYPTO_SHA1_WIN supported. */
/* #undef ARCHIVE_CRYPTO_SHA1_WIN */
/* SHA256 via ARCHIVE_CRYPTO_SHA256_LIBC supported. */
/* #undef ARCHIVE_CRYPTO_SHA256_LIBC */
/* SHA256 via ARCHIVE_CRYPTO_SHA256_LIBC2 supported. */
/* #undef ARCHIVE_CRYPTO_SHA256_LIBC2 */
/* SHA256 via ARCHIVE_CRYPTO_SHA256_LIBC3 supported. */
/* #undef ARCHIVE_CRYPTO_SHA256_LIBC3 */
/* SHA256 via ARCHIVE_CRYPTO_SHA256_LIBMD supported. */
/* #undef ARCHIVE_CRYPTO_SHA256_LIBMD */
/* SHA256 via ARCHIVE_CRYPTO_SHA256_LIBSYSTEM supported. */
/* #undef ARCHIVE_CRYPTO_SHA256_LIBSYSTEM */
/* SHA256 via ARCHIVE_CRYPTO_SHA256_MBEDTLS supported. */
/* #undef ARCHIVE_CRYPTO_SHA256_MBEDTLS */
/* SHA256 via ARCHIVE_CRYPTO_SHA256_NETTLE supported. */
/* #undef ARCHIVE_CRYPTO_SHA256_NETTLE */
/* SHA256 via ARCHIVE_CRYPTO_SHA256_OPENSSL supported. */
/* SHA256 via ARCHIVE_CRYPTO_SHA256_WIN supported. */
/* #undef ARCHIVE_CRYPTO_SHA256_WIN */
/* SHA384 via ARCHIVE_CRYPTO_SHA384_LIBC supported. */
/* #undef ARCHIVE_CRYPTO_SHA384_LIBC */
/* SHA384 via ARCHIVE_CRYPTO_SHA384_LIBC2 supported. */
/* #undef ARCHIVE_CRYPTO_SHA384_LIBC2 */
/* SHA384 via ARCHIVE_CRYPTO_SHA384_LIBC3 supported. */
/* #undef ARCHIVE_CRYPTO_SHA384_LIBC3 */
/* SHA384 via ARCHIVE_CRYPTO_SHA384_LIBSYSTEM supported. */
/* #undef ARCHIVE_CRYPTO_SHA384_LIBSYSTEM */
/* SHA384 via ARCHIVE_CRYPTO_SHA384_MBEDTLS supported. */
/* #undef ARCHIVE_CRYPTO_SHA384_MBEDTLS */
/* SHA384 via ARCHIVE_CRYPTO_SHA384_NETTLE supported. */
/* #undef ARCHIVE_CRYPTO_SHA384_NETTLE */
/* SHA384 via ARCHIVE_CRYPTO_SHA384_OPENSSL supported. */
/* SHA384 via ARCHIVE_CRYPTO_SHA384_WIN supported. */
/* #undef ARCHIVE_CRYPTO_SHA384_WIN */
/* SHA512 via ARCHIVE_CRYPTO_SHA512_LIBC supported. */
/* #undef ARCHIVE_CRYPTO_SHA512_LIBC */
/* SHA512 via ARCHIVE_CRYPTO_SHA512_LIBC2 supported. */
/* #undef ARCHIVE_CRYPTO_SHA512_LIBC2 */
/* SHA512 via ARCHIVE_CRYPTO_SHA512_LIBC3 supported. */
/* #undef ARCHIVE_CRYPTO_SHA512_LIBC3 */
/* SHA512 via ARCHIVE_CRYPTO_SHA512_LIBMD supported. */
/* #undef ARCHIVE_CRYPTO_SHA512_LIBMD */
/* SHA512 via ARCHIVE_CRYPTO_SHA512_LIBSYSTEM supported. */
/* #undef ARCHIVE_CRYPTO_SHA512_LIBSYSTEM */
/* SHA512 via ARCHIVE_CRYPTO_SHA512_MBEDTLS supported. */
/* #undef ARCHIVE_CRYPTO_SHA512_MBEDTLS */
/* SHA512 via ARCHIVE_CRYPTO_SHA512_NETTLE supported. */
/* #undef ARCHIVE_CRYPTO_SHA512_NETTLE */
/* SHA512 via ARCHIVE_CRYPTO_SHA512_OPENSSL supported. */
/* SHA512 via ARCHIVE_CRYPTO_SHA512_WIN supported. */
/* #undef ARCHIVE_CRYPTO_SHA512_WIN */
/* AIX xattr support */
/* #undef ARCHIVE_XATTR_AIX */
/* Darwin xattr support */
/* #undef ARCHIVE_XATTR_DARWIN */
/* FreeBSD xattr support */
/* #undef ARCHIVE_XATTR_FREEBSD */
/* Linux xattr support */
/* Version number of bsdcat */
pub const BSDCAT_VERSION_STRING: [libc::c_char; 6] =
    unsafe { *::std::mem::transmute::<&[u8; 6], &[libc::c_char; 6]>(b"3.4.3\x00") };
pub const BYTES_PER_BLOCK: libc::c_int = 20 as libc::c_int * 512 as libc::c_int;
static mut a: *mut archive = 0 as *const archive as *mut archive;
static mut ae: *mut archive_entry = 0 as *const archive_entry as *mut archive_entry;
static mut bsdcat_current_path: *const libc::c_char = 0 as *const libc::c_char;
static mut exit_status: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn cat_usage(mut stream: *mut FILE, mut eval: libc::c_int) {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    p = lafe_getprogname();
    fprintf(
        stream,
        b"Usage: %s [-h] [--help] [--version] [--] [filenames...]\n\x00" as *const u8
            as *const libc::c_char,
        p,
    );
    exit(eval);
}
unsafe extern "C" fn version() {
    printf(
        b"bsdcat %s - %s \n\x00" as *const u8 as *const libc::c_char,
        BSDCAT_VERSION_STRING.as_ptr(),
        archive_version_details(),
    );
    exit(0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn bsdcat_next() {
    if !a.is_null() {
        if archive_read_close(a) != ARCHIVE_OK {
            bsdcat_print_error();
        }
        archive_read_free(a);
    }
    a = archive_read_new();
    archive_read_support_filter_all(a);
    archive_read_support_format_empty(a);
    archive_read_support_format_raw(a);
}
#[no_mangle]
pub unsafe extern "C" fn bsdcat_print_error() {
    lafe_warnc(
        0 as libc::c_int,
        b"%s: %s\x00" as *const u8 as *const libc::c_char,
        bsdcat_current_path,
        archive_error_string(a),
    );
    exit_status = 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn bsdcat_read_to_stdout(mut filename: *const libc::c_char) {
    let mut r: libc::c_int = 0;
    if archive_read_open_filename(a, filename, BYTES_PER_BLOCK as size_t) != ARCHIVE_OK {
        bsdcat_print_error();
    } else {
        r = archive_read_next_header(a, &mut ae);
        if r != ARCHIVE_OK && r != ARCHIVE_EOF {
            bsdcat_print_error();
        } else if !(r == ARCHIVE_EOF) {
            if archive_read_data_into_fd(a, 1 as libc::c_int) != ARCHIVE_OK {
                bsdcat_print_error();
            }
        }
    }
    if archive_read_close(a) != ARCHIVE_OK {
        bsdcat_print_error();
    }
    archive_read_free(a);
    a = NULL as *mut archive;
}
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    let mut bsdcat: *mut bsdcat = 0 as *mut bsdcat;
    let mut bsdcat_storage: bsdcat = bsdcat {
        getopt_state: 0,
        getopt_word: 0 as *mut libc::c_char,
        argc: 0,
        argv: 0 as *mut *mut libc::c_char,
        argument: 0 as *const libc::c_char,
    };
    let mut c: libc::c_int = 0;
    bsdcat = &mut bsdcat_storage;
    memset(
        bsdcat as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<bsdcat>() as libc::c_ulong,
    );
    lafe_setprogname(*argv, b"bsdcat\x00" as *const u8 as *const libc::c_char);
    (*bsdcat).argv = argv;
    (*bsdcat).argc = argc;
    loop {
        c = bsdcat_getopt(bsdcat);
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        match c {
            104 => {
                cat_usage(stdout, 0 as libc::c_int);
            }
            0 => {
                version();
            }
            _ => {
                cat_usage(stderr, 1 as libc::c_int);
            }
        }
    }
    bsdcat_next();
    if (*(*bsdcat).argv).is_null() {
        bsdcat_current_path = b"<stdin>\x00" as *const u8 as *const libc::c_char;
        bsdcat_read_to_stdout(NULL as *const libc::c_char);
    } else {
        while !(*(*bsdcat).argv).is_null() {
            let fresh0 = (*bsdcat).argv;
            (*bsdcat).argv = (*bsdcat).argv.offset(1);
            bsdcat_current_path = *fresh0;
            bsdcat_read_to_stdout(bsdcat_current_path);
            bsdcat_next();
        }
        archive_read_free(a);
        /* Help valgrind & friends */
    }
    exit(exit_status);
}
#[main]
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
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
            (args.len() - 1) as libc::c_int,
            args.as_mut_ptr() as *mut *mut libc::c_char,
        ) as i32)
    }
}
