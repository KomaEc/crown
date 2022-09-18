use ::libc;
extern "C" {
    pub type archive_string_conv;
    pub type archive_entry;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn archive_write_get_bytes_per_block(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_set_error(_: *mut archive, _err: libc::c_int, fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn __archive_check_magic(
        _: *mut archive,
        magic: libc::c_uint,
        state: libc::c_uint,
        func: *const libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn __archive_write_filters_free(_: *mut archive);
    #[no_mangle]
    fn __archive_write_allocate_filter(_: *mut archive) -> *mut archive_write_filter;
    #[no_mangle]
    fn __archive_write_filter(
        _: *mut archive_write_filter,
        _: *const libc::c_void,
        _: size_t,
    ) -> libc::c_int;
}
pub type __int64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type int64_t = __int64_t;
pub type size_t = libc::c_ulong;
pub type ssize_t = __ssize_t;
pub type la_ssize_t = ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive {
    pub magic: libc::c_uint,
    pub state: libc::c_uint,
    pub vtable: *mut archive_vtable,
    pub archive_format: libc::c_int,
    pub archive_format_name: *const libc::c_char,
    pub compression_code: libc::c_int,
    pub compression_name: *const libc::c_char,
    pub file_count: libc::c_int,
    pub archive_error_number: libc::c_int,
    pub error: *const libc::c_char,
    pub error_string: archive_string,
    pub current_code: *mut libc::c_char,
    pub current_codepage: libc::c_uint,
    pub current_oemcp: libc::c_uint,
    pub sconv: *mut archive_string_conv,
    pub read_data_block: *const libc::c_char,
    pub read_data_offset: int64_t,
    pub read_data_output_offset: int64_t,
    pub read_data_remaining: size_t,
    pub read_data_is_posix_read: libc::c_char,
    pub read_data_requested: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_string {
    pub s: *mut libc::c_char,
    pub length: size_t,
    pub buffer_length: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_vtable {
    pub archive_close: Option<unsafe extern "C" fn(_: *mut archive) -> libc::c_int>,
    pub archive_free: Option<unsafe extern "C" fn(_: *mut archive) -> libc::c_int>,
    pub archive_write_header:
        Option<unsafe extern "C" fn(_: *mut archive, _: *mut archive_entry) -> libc::c_int>,
    pub archive_write_finish_entry: Option<unsafe extern "C" fn(_: *mut archive) -> libc::c_int>,
    pub archive_write_data:
        Option<unsafe extern "C" fn(_: *mut archive, _: *const libc::c_void, _: size_t) -> ssize_t>,
    pub archive_write_data_block: Option<
        unsafe extern "C" fn(
            _: *mut archive,
            _: *const libc::c_void,
            _: size_t,
            _: int64_t,
        ) -> ssize_t,
    >,
    pub archive_read_next_header:
        Option<unsafe extern "C" fn(_: *mut archive, _: *mut *mut archive_entry) -> libc::c_int>,
    pub archive_read_next_header2:
        Option<unsafe extern "C" fn(_: *mut archive, _: *mut archive_entry) -> libc::c_int>,
    pub archive_read_data_block: Option<
        unsafe extern "C" fn(
            _: *mut archive,
            _: *mut *const libc::c_void,
            _: *mut size_t,
            _: *mut int64_t,
        ) -> libc::c_int,
    >,
    pub archive_filter_count: Option<unsafe extern "C" fn(_: *mut archive) -> libc::c_int>,
    pub archive_filter_bytes:
        Option<unsafe extern "C" fn(_: *mut archive, _: libc::c_int) -> int64_t>,
    pub archive_filter_code:
        Option<unsafe extern "C" fn(_: *mut archive, _: libc::c_int) -> libc::c_int>,
    pub archive_filter_name:
        Option<unsafe extern "C" fn(_: *mut archive, _: libc::c_int) -> *const libc::c_char>,
}
pub type archive_write_callback = unsafe extern "C" fn(
    _: *mut archive,
    _: *mut libc::c_void,
    _: *const libc::c_void,
    _: size_t,
) -> la_ssize_t;
pub type archive_open_callback =
    unsafe extern "C" fn(_: *mut archive, _: *mut libc::c_void) -> libc::c_int;
pub type archive_close_callback =
    unsafe extern "C" fn(_: *mut archive, _: *mut libc::c_void) -> libc::c_int;
pub type archive_passphrase_callback =
    unsafe extern "C" fn(_: *mut archive, _: *mut libc::c_void) -> *const libc::c_char;
/*-
 * Copyright (c) 2003-2007 Tim Kientzle
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
 *
 * $FreeBSD: head/lib/libarchive/archive_write_private.h 201155 2009-12-29 05:20:12Z kientzle $
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_write_filter {
    pub bytes_written: int64_t,
    pub archive: *mut archive,
    pub next_filter: *mut archive_write_filter,
    pub options: Option<
        unsafe extern "C" fn(
            _: *mut archive_write_filter,
            _: *const libc::c_char,
            _: *const libc::c_char,
        ) -> libc::c_int,
    >,
    pub open: Option<unsafe extern "C" fn(_: *mut archive_write_filter) -> libc::c_int>,
    pub write: Option<
        unsafe extern "C" fn(
            _: *mut archive_write_filter,
            _: *const libc::c_void,
            _: size_t,
        ) -> libc::c_int,
    >,
    pub close: Option<unsafe extern "C" fn(_: *mut archive_write_filter) -> libc::c_int>,
    pub free: Option<unsafe extern "C" fn(_: *mut archive_write_filter) -> libc::c_int>,
    pub data: *mut libc::c_void,
    pub name: *const libc::c_char,
    pub code: libc::c_int,
    pub bytes_per_block: libc::c_int,
    pub bytes_in_last_block: libc::c_int,
    pub state: libc::c_int,
}
/* Table clear output code. */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct private_data {
    pub in_count: int64_t,
    pub out_count: int64_t,
    pub checkpoint: int64_t,
    pub code_len: libc::c_int,
    pub cur_maxcode: libc::c_int,
    pub max_maxcode: libc::c_int,
    pub hashtab: [libc::c_int; 69001],
    pub codetab: [libc::c_ushort; 69001],
    pub first_free: libc::c_int,
    pub compress_ratio: libc::c_int,
    pub cur_code: libc::c_int,
    pub cur_fcode: libc::c_int,
    pub bit_offset: libc::c_int,
    pub bit_buf: libc::c_uchar,
    pub compressed: *mut libc::c_uchar,
    pub compressed_buffer_size: size_t,
    pub compressed_offset: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_write {
    pub archive: archive,
    pub skip_file_set: libc::c_int,
    pub skip_file_dev: int64_t,
    pub skip_file_ino: int64_t,
    pub nulls: *const libc::c_uchar,
    pub null_length: size_t,
    pub client_opener: Option<archive_open_callback>,
    pub client_writer: Option<archive_write_callback>,
    pub client_closer: Option<archive_close_callback>,
    pub client_data: *mut libc::c_void,
    pub bytes_per_block: libc::c_int,
    pub bytes_in_last_block: libc::c_int,
    pub filter_first: *mut archive_write_filter,
    pub filter_last: *mut archive_write_filter,
    pub format_data: *mut libc::c_void,
    pub format_name: *const libc::c_char,
    pub format_init: Option<unsafe extern "C" fn(_: *mut archive_write) -> libc::c_int>,
    pub format_options: Option<
        unsafe extern "C" fn(
            _: *mut archive_write,
            _: *const libc::c_char,
            _: *const libc::c_char,
        ) -> libc::c_int,
    >,
    pub format_finish_entry: Option<unsafe extern "C" fn(_: *mut archive_write) -> libc::c_int>,
    pub format_write_header:
        Option<unsafe extern "C" fn(_: *mut archive_write, _: *mut archive_entry) -> libc::c_int>,
    pub format_write_data: Option<
        unsafe extern "C" fn(_: *mut archive_write, _: *const libc::c_void, _: size_t) -> ssize_t,
    >,
    pub format_close: Option<unsafe extern "C" fn(_: *mut archive_write) -> libc::c_int>,
    pub format_free: Option<unsafe extern "C" fn(_: *mut archive_write) -> libc::c_int>,
    pub passphrase: *mut libc::c_char,
    pub passphrase_callback: Option<archive_passphrase_callback>,
    pub passphrase_client_data: *mut libc::c_void,
}
pub const ENOMEM: libc::c_int = 12 as libc::c_int;
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const ARCHIVE_FATAL: libc::c_int = -(30 as libc::c_int);
pub const ARCHIVE_FILTER_COMPRESS: libc::c_int = 3 as libc::c_int;
pub const ARCHIVE_OK: libc::c_int = 0 as libc::c_int;
pub const ARCHIVE_WRITE_MAGIC: libc::c_uint = 0xb0c5c0de as libc::c_uint;
pub const HSIZE: libc::c_int = 69001 as libc::c_int;
/* 95% occupancy */
pub const HSHIFT: libc::c_int = 8 as libc::c_int;
/* 8 - trunc(log2(HSIZE / 65536)) */
pub const CHECK_GAP: libc::c_int = 10000 as libc::c_int;
/*
 * the next two codes should not be changed lightly, as they must not
 * lie within the contiguous general code space.
 */
pub const FIRST: libc::c_int = 257 as libc::c_int;
/* First free entry. */
pub const CLEAR: libc::c_int = 256 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn archive_write_set_compression_compress(
    mut a: *mut archive,
) -> libc::c_int {
    __archive_write_filters_free(a);
    return archive_write_add_filter_compress(a);
}
/*
 * Add a compress filter to this write handle.
 */
#[no_mangle]
pub unsafe extern "C" fn archive_write_add_filter_compress(mut _a: *mut archive) -> libc::c_int {
    let mut a: *mut archive_write = _a as *mut archive_write;
    let mut f: *mut archive_write_filter = __archive_write_allocate_filter(_a);
    let mut magic_test: libc::c_int = __archive_check_magic(
        &mut (*a).archive,
        0xb0c5c0de as libc::c_uint,
        1 as libc::c_uint,
        b"archive_write_add_filter_compress\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    (*f).open = Some(
        archive_compressor_compress_open
            as unsafe extern "C" fn(_: *mut archive_write_filter) -> libc::c_int,
    );
    (*f).code = ARCHIVE_FILTER_COMPRESS;
    (*f).name = b"compress\x00" as *const u8 as *const libc::c_char;
    return 0 as libc::c_int;
}
/*
 * Setup callback.
 */
unsafe extern "C" fn archive_compressor_compress_open(
    mut f: *mut archive_write_filter,
) -> libc::c_int {
    let mut state: *mut private_data = 0 as *mut private_data;
    let mut bs: size_t = 65536 as libc::c_int as size_t;
    let mut bpb: size_t = 0;
    (*f).code = ARCHIVE_FILTER_COMPRESS;
    (*f).name = b"compress\x00" as *const u8 as *const libc::c_char;
    state = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<private_data>() as libc::c_ulong,
    ) as *mut private_data;
    if state.is_null() {
        archive_set_error(
            (*f).archive,
            ENOMEM,
            b"Can\'t allocate data for compression\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    if (*(*f).archive).magic == ARCHIVE_WRITE_MAGIC {
        /* Buffer size should be a multiple number of the of bytes
         * per block for performance. */
        bpb = archive_write_get_bytes_per_block((*f).archive) as size_t; /* Should NEVER generate this code. */
        if bpb > bs {
            bs = bpb
        } else if bpb != 0 as libc::c_int as libc::c_ulong {
            bs = (bs as libc::c_ulong).wrapping_sub(bs.wrapping_rem(bpb)) as size_t as size_t
        }
    } /* Length of input. */
    (*state).compressed_buffer_size = bs; /* Includes 3-byte header mojo. */
    (*state).compressed = malloc((*state).compressed_buffer_size) as *mut libc::c_uchar;
    if (*state).compressed.is_null() {
        archive_set_error(
            (*f).archive,
            ENOMEM,
            b"Can\'t allocate data for compression buffer\x00" as *const u8 as *const libc::c_char,
        );
        free(state as *mut libc::c_void);
        return -(30 as libc::c_int);
    }
    (*f).write = Some(
        archive_compressor_compress_write
            as unsafe extern "C" fn(
                _: *mut archive_write_filter,
                _: *const libc::c_void,
                _: size_t,
            ) -> libc::c_int,
    );
    (*f).close = Some(
        archive_compressor_compress_close
            as unsafe extern "C" fn(_: *mut archive_write_filter) -> libc::c_int,
    );
    (*f).free = Some(
        archive_compressor_compress_free
            as unsafe extern "C" fn(_: *mut archive_write_filter) -> libc::c_int,
    );
    (*state).max_maxcode = 0x10000 as libc::c_int;
    (*state).in_count = 0 as libc::c_int as int64_t;
    (*state).bit_buf = 0 as libc::c_int as libc::c_uchar;
    (*state).bit_offset = 0 as libc::c_int;
    (*state).out_count = 3 as libc::c_int as int64_t;
    (*state).compress_ratio = 0 as libc::c_int;
    (*state).checkpoint = CHECK_GAP as int64_t;
    (*state).code_len = 9 as libc::c_int;
    (*state).cur_maxcode = ((1 as libc::c_int) << (*state).code_len) - 1 as libc::c_int;
    (*state).first_free = FIRST;
    memset(
        (*state).hashtab.as_mut_ptr() as *mut libc::c_void,
        0xff as libc::c_int,
        ::std::mem::size_of::<[libc::c_int; 69001]>() as libc::c_ulong,
    );
    /* Prime output buffer with a gzip header. */
    *(*state).compressed.offset(0 as libc::c_int as isize) = 0x1f as libc::c_int as libc::c_uchar; /* Compress */
    *(*state).compressed.offset(1 as libc::c_int as isize) = 0x9d as libc::c_int as libc::c_uchar; /* Block mode, 16bit max */
    *(*state).compressed.offset(2 as libc::c_int as isize) = 0x90 as libc::c_int as libc::c_uchar;
    (*state).compressed_offset = 3 as libc::c_int as size_t;
    (*f).data = state as *mut libc::c_void;
    return 0 as libc::c_int;
}
/*-
 * Output the given code.
 * Inputs:
 * 	code:	A n_bits-bit integer.  If == -1, then EOF.  This assumes
 *		that n_bits <= (long)wordsize - 1.
 * Outputs:
 * 	Outputs code to the file.
 * Assumptions:
 *	Chars are 8 bits long.
 * Algorithm:
 * 	Maintain a BITS character long buffer (so that 8 codes will
 * fit in it exactly).  Use the VAX insv instruction to insert each
 * code in turn.  When the buffer fills up empty it and start over.
 */
static mut rmask: [libc::c_uchar; 9] = [
    0 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0x7 as libc::c_int as libc::c_uchar,
    0xf as libc::c_int as libc::c_uchar,
    0x1f as libc::c_int as libc::c_uchar,
    0x3f as libc::c_int as libc::c_uchar,
    0x7f as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
];
unsafe extern "C" fn output_byte(
    mut f: *mut archive_write_filter,
    mut c: libc::c_uchar,
) -> libc::c_int {
    let mut state: *mut private_data = (*f).data as *mut private_data;
    let fresh0 = (*state).compressed_offset;
    (*state).compressed_offset = (*state).compressed_offset.wrapping_add(1);
    *(*state).compressed.offset(fresh0 as isize) = c;
    (*state).out_count += 1;
    if (*state).compressed_buffer_size == (*state).compressed_offset {
        let mut ret: libc::c_int = __archive_write_filter(
            (*f).next_filter,
            (*state).compressed as *const libc::c_void,
            (*state).compressed_buffer_size,
        );
        if ret != ARCHIVE_OK {
            return ARCHIVE_FATAL;
        }
        (*state).compressed_offset = 0 as libc::c_int as size_t
    }
    return ARCHIVE_OK;
}
unsafe extern "C" fn output_code(
    mut f: *mut archive_write_filter,
    mut ocode: libc::c_int,
) -> libc::c_int {
    let mut state: *mut private_data = (*f).data as *mut private_data;
    let mut bits: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut clear_flg: libc::c_int = 0;
    let mut bit_offset: libc::c_int = 0;
    clear_flg = (ocode == CLEAR) as libc::c_int;
    /*
     * Since ocode is always >= 8 bits, only need to mask the first
     * hunk on the left.
     */
    bit_offset = (*state).bit_offset % 8 as libc::c_int;
    (*state).bit_buf = ((*state).bit_buf as libc::c_int | ocode << bit_offset & 0xff as libc::c_int)
        as libc::c_uchar;
    output_byte(f, (*state).bit_buf);
    bits = (*state).code_len - (8 as libc::c_int - bit_offset);
    ocode >>= 8 as libc::c_int - bit_offset;
    /* Get any 8 bit parts in the middle (<=1 for up to 16 bits). */
    if bits >= 8 as libc::c_int {
        output_byte(f, (ocode & 0xff as libc::c_int) as libc::c_uchar);
        ocode >>= 8 as libc::c_int;
        bits -= 8 as libc::c_int
    }
    /* Last bits. */
    (*state).bit_offset += (*state).code_len;
    (*state).bit_buf = (ocode & rmask[bits as usize] as libc::c_int) as libc::c_uchar;
    if (*state).bit_offset == (*state).code_len * 8 as libc::c_int {
        (*state).bit_offset = 0 as libc::c_int
    }
    /*
     * If the next entry is going to be too big for the ocode size,
     * then increase it, if possible.
     */
    if clear_flg != 0 || (*state).first_free > (*state).cur_maxcode {
        /*
        	* Write the whole buffer, because the input side won't
        	* discover the size increase until after it has read it.
        	*/
        if (*state).bit_offset > 0 as libc::c_int {
            while (*state).bit_offset < (*state).code_len * 8 as libc::c_int {
                ret = output_byte(f, (*state).bit_buf);
                if ret != ARCHIVE_OK {
                    return ret;
                }
                (*state).bit_offset += 8 as libc::c_int;
                (*state).bit_buf = 0 as libc::c_int as libc::c_uchar
            }
        }
        (*state).bit_buf = 0 as libc::c_int as libc::c_uchar;
        (*state).bit_offset = 0 as libc::c_int;
        if clear_flg != 0 {
            (*state).code_len = 9 as libc::c_int;
            (*state).cur_maxcode = ((1 as libc::c_int) << (*state).code_len) - 1 as libc::c_int
        } else {
            (*state).code_len += 1;
            if (*state).code_len == 16 as libc::c_int {
                (*state).cur_maxcode = (*state).max_maxcode
            } else {
                (*state).cur_maxcode = ((1 as libc::c_int) << (*state).code_len) - 1 as libc::c_int
            }
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn output_flush(mut f: *mut archive_write_filter) -> libc::c_int {
    let mut state: *mut private_data = (*f).data as *mut private_data;
    let mut ret: libc::c_int = 0;
    /* At EOF, write the rest of the buffer. */
    if (*state).bit_offset % 8 as libc::c_int != 0 {
        (*state).code_len =
            ((*state).bit_offset % 8 as libc::c_int + 7 as libc::c_int) / 8 as libc::c_int;
        ret = output_byte(f, (*state).bit_buf);
        if ret != ARCHIVE_OK {
            return ret;
        }
    }
    return 0 as libc::c_int;
}
/*
 * Write data to the compressed stream.
 */
unsafe extern "C" fn archive_compressor_compress_write(
    mut f: *mut archive_write_filter,
    mut buff: *const libc::c_void,
    mut length: size_t,
) -> libc::c_int {
    let mut state: *mut private_data = (*f).data as *mut private_data; /* Xor hashing. */
    let mut i: libc::c_int = 0;
    let mut ratio: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut disp: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut bp: *const libc::c_uchar = 0 as *const libc::c_uchar;
    if length == 0 as libc::c_int as libc::c_ulong {
        return ARCHIVE_OK;
    }
    bp = buff as *const libc::c_uchar;
    if (*state).in_count == 0 as libc::c_int as libc::c_long {
        let fresh1 = bp;
        bp = bp.offset(1);
        (*state).cur_code = *fresh1 as libc::c_int;
        (*state).in_count += 1;
        length = length.wrapping_sub(1)
    }
    's_51: loop {
        let fresh2 = length;
        length = length.wrapping_sub(1);
        if !(fresh2 != 0) {
            break;
        }
        let fresh3 = bp;
        bp = bp.offset(1);
        c = *fresh3 as libc::c_int;
        (*state).in_count += 1;
        (*state).cur_fcode = (c << 16 as libc::c_int) + (*state).cur_code;
        i = c << HSHIFT ^ (*state).cur_code;
        if (*state).hashtab[i as usize] == (*state).cur_fcode {
            (*state).cur_code = (*state).codetab[i as usize] as libc::c_int
        } else {
            if !((*state).hashtab[i as usize] < 0 as libc::c_int) {
                /* Secondary hash (after G. Knott). */
                if i == 0 as libc::c_int {
                    disp = 1 as libc::c_int
                } else {
                    disp = HSIZE - i
                }
                loop {
                    i -= disp;
                    if i < 0 as libc::c_int {
                        i += HSIZE
                    }
                    if (*state).hashtab[i as usize] == (*state).cur_fcode {
                        (*state).cur_code = (*state).codetab[i as usize] as libc::c_int;
                        continue 's_51;
                    } else if !((*state).hashtab[i as usize] >= 0 as libc::c_int) {
                        break;
                    }
                }
            }
            /* Empty slot. */
            ret = output_code(f, (*state).cur_code); /* code -> hashtable */
            if ret != ARCHIVE_OK {
                return ret;
            }
            (*state).cur_code = c;
            if (*state).first_free < (*state).max_maxcode {
                let fresh4 = (*state).first_free;
                (*state).first_free = (*state).first_free + 1;
                (*state).codetab[i as usize] = fresh4 as libc::c_ushort;
                (*state).hashtab[i as usize] = (*state).cur_fcode
            } else {
                if (*state).in_count < (*state).checkpoint {
                    continue;
                }
                (*state).checkpoint = (*state).in_count + CHECK_GAP as libc::c_long;
                if (*state).in_count <= 0x7fffff as libc::c_int as libc::c_long
                    && (*state).out_count != 0 as libc::c_int as libc::c_long
                {
                    ratio = ((*state).in_count * 256 as libc::c_int as libc::c_long
                        / (*state).out_count) as libc::c_int
                } else {
                    ratio =
                        ((*state).out_count / 256 as libc::c_int as libc::c_long) as libc::c_int;
                    if ratio == 0 as libc::c_int {
                        ratio = 0x7fffffff as libc::c_int
                    } else {
                        ratio = ((*state).in_count / ratio as libc::c_long) as libc::c_int
                    }
                }
                if ratio > (*state).compress_ratio {
                    (*state).compress_ratio = ratio
                } else {
                    (*state).compress_ratio = 0 as libc::c_int;
                    memset(
                        (*state).hashtab.as_mut_ptr() as *mut libc::c_void,
                        0xff as libc::c_int,
                        ::std::mem::size_of::<[libc::c_int; 69001]>() as libc::c_ulong,
                    );
                    (*state).first_free = FIRST;
                    ret = output_code(f, CLEAR);
                    if ret != ARCHIVE_OK {
                        return ret;
                    }
                }
            }
        }
    }
    return 0 as libc::c_int;
}
/*
 * Finish the compression...
 */
unsafe extern "C" fn archive_compressor_compress_close(
    mut f: *mut archive_write_filter,
) -> libc::c_int {
    let mut state: *mut private_data = (*f).data as *mut private_data;
    let mut ret: libc::c_int = 0;
    ret = output_code(f, (*state).cur_code);
    if ret != ARCHIVE_OK {
        return ret;
    }
    ret = output_flush(f);
    if ret != ARCHIVE_OK {
        return ret;
    }
    /* Write the last block */
    ret = __archive_write_filter(
        (*f).next_filter,
        (*state).compressed as *const libc::c_void,
        (*state).compressed_offset,
    );
    return ret;
}
unsafe extern "C" fn archive_compressor_compress_free(
    mut f: *mut archive_write_filter,
) -> libc::c_int {
    let mut state: *mut private_data = (*f).data as *mut private_data;
    free((*state).compressed as *mut libc::c_void);
    free(state as *mut libc::c_void);
    return 0 as libc::c_int;
}
