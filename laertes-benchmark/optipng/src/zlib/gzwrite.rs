
extern "C" {
    pub type internal_state;
    #[no_mangle]
    fn close(__fd: std::os::raw::c_int) -> std::os::raw::c_int;
    #[no_mangle]
    fn write(__fd: std::os::raw::c_int, __buf: *const std::os::raw::c_void, __n: size_t)
     -> ssize_t;
    #[no_mangle]
    fn vsnprintf(_: *mut std::os::raw::c_char, _: std::os::raw::c_ulong,
                 _: *const std::os::raw::c_char, _: ::std::ffi::VaList)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn deflate(strm: z_streamp, flush: std::os::raw::c_int) -> std::os::raw::c_int;
    #[no_mangle]
    fn deflateEnd(strm: z_streamp) -> std::os::raw::c_int;
    #[no_mangle]
    fn deflateReset(strm: z_streamp) -> std::os::raw::c_int;
    #[no_mangle]
    fn deflateParams(strm: z_streamp, level: std::os::raw::c_int,
                     strategy: std::os::raw::c_int) -> std::os::raw::c_int;
    #[no_mangle]
    fn gz_error(_: gz_statep, _: std::os::raw::c_int, _: *const std::os::raw::c_char);
    #[no_mangle]
    fn __errno_location() -> *mut std::os::raw::c_int;
    #[no_mangle]
    fn strerror(_: std::os::raw::c_int) -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn free(__ptr: *mut std::os::raw::c_void);
    #[no_mangle]
    fn deflateInit2_(strm: z_streamp, level: std::os::raw::c_int, method: std::os::raw::c_int,
                     windowBits: std::os::raw::c_int, memLevel: std::os::raw::c_int,
                     strategy: std::os::raw::c_int, version: *const std::os::raw::c_char,
                     stream_size: std::os::raw::c_int) -> std::os::raw::c_int;
    #[no_mangle]
    fn malloc(_: std::os::raw::c_ulong) -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn memset(_: *mut std::os::raw::c_void, _: std::os::raw::c_int, _: std::os::raw::c_ulong)
     -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn memcpy(_: *mut std::os::raw::c_void, _: *const std::os::raw::c_void, _: std::os::raw::c_ulong)
     -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn strlen(_: *const std::os::raw::c_char) -> std::os::raw::c_ulong;
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
pub type size_t = std::os::raw::c_ulong;
pub type va_list = __builtin_va_list;
pub type __off64_t = std::os::raw::c_long;
pub type __ssize_t = std::os::raw::c_long;
pub type off64_t = __off64_t;
pub type ssize_t = __ssize_t;
pub type z_size_t = size_t;
pub type Byte = std::os::raw::c_uchar;
pub type uInt = std::os::raw::c_uint;
pub type uLong = std::os::raw::c_ulong;
pub type Bytef = Byte;
pub type voidpc = *const std::os::raw::c_void;
pub type voidpf = *mut std::os::raw::c_void;
pub type alloc_func
    =
    Option<unsafe extern "C" fn(_: voidpf, _: uInt, _: uInt) -> voidpf>;
pub type free_func = Option<unsafe extern "C" fn(_: voidpf, _: voidpf) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct z_stream_s {
    pub next_in: *mut Bytef,
    pub avail_in: uInt,
    pub total_in: uLong,
    pub next_out: *mut Bytef,
    pub avail_out: uInt,
    pub total_out: uLong,
    pub msg: *mut std::os::raw::c_char,
    pub state: *mut internal_state,
    pub zalloc: alloc_func,
    pub zfree: free_func,
    pub opaque: voidpf,
    pub data_type: std::os::raw::c_int,
    pub adler: uLong,
    pub reserved: uLong,
}
pub type z_stream = z_stream_s;
pub type z_streamp = *mut z_stream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gzFile_s {
    pub have: std::os::raw::c_uint,
    pub next: *mut std::os::raw::c_uchar,
    pub pos: off64_t,
}
pub type gzFile = *mut gzFile_s;
pub type gz_statep = *mut gz_state;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gz_state {
    pub x: gzFile_s,
    pub mode: std::os::raw::c_int,
    pub fd: std::os::raw::c_int,
    pub path: *mut std::os::raw::c_char,
    pub size: std::os::raw::c_uint,
    pub want: std::os::raw::c_uint,
    pub in_0: *mut std::os::raw::c_uchar,
    pub out: *mut std::os::raw::c_uchar,
    pub direct: std::os::raw::c_int,
    pub how: std::os::raw::c_int,
    pub start: off64_t,
    pub eof: std::os::raw::c_int,
    pub past: std::os::raw::c_int,
    pub level: std::os::raw::c_int,
    pub strategy: std::os::raw::c_int,
    pub skip: off64_t,
    pub seek: std::os::raw::c_int,
    pub err: std::os::raw::c_int,
    pub msg: *mut std::os::raw::c_char,
    pub strm: z_stream,
}
/* gzwrite.c -- zlib functions for writing gzip files
 * Copyright (C) 2004-2017 Mark Adler
 * For conditions of distribution and use, see copyright notice in zlib.h
 */
/* Local functions */
/* Initialize state for writing a gzip file.  Mark initialization by setting
   state->size to non-zero.  Return -1 on a memory allocation failure, or 0 on
   success. */
unsafe extern "C" fn gz_init(mut state: gz_statep) -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0;
    let mut strm: z_streamp = &mut (*state).strm;
    /* allocate input buffer (double size for gzprintf) */
    (*state).in_0 =
        malloc(((*state).want << 1 as std::os::raw::c_int) as std::os::raw::c_ulong) as
            *mut std::os::raw::c_uchar;
    if (*state).in_0.is_null() {
        gz_error(state, -(4 as std::os::raw::c_int),
                 b"out of memory\x00" as *const u8 as *const std::os::raw::c_char);
        return -(1 as std::os::raw::c_int)
    }
    /* only need output buffer and deflate state if compressing */
    if (*state).direct == 0 {
        /* allocate output buffer */
        (*state).out =
            malloc((*state).want as std::os::raw::c_ulong) as *mut std::os::raw::c_uchar;
        if (*state).out.is_null() {
            free((*state).in_0 as *mut std::os::raw::c_void);
            gz_error(state, -(4 as std::os::raw::c_int),
                     b"out of memory\x00" as *const u8 as
                         *const std::os::raw::c_char);
            return -(1 as std::os::raw::c_int)
        }
        /* allocate deflate memory, set up for gzip compression */
        (*strm).zalloc = None;
        (*strm).zfree = None;
        (*strm).opaque = 0 as voidpf;
        ret =
            deflateInit2_(strm, (*state).level, 8 as std::os::raw::c_int,
                          15 as std::os::raw::c_int + 16 as std::os::raw::c_int,
                          8 as std::os::raw::c_int, (*state).strategy,
                          b"1.2.11-optipng\x00" as *const u8 as
                              *const std::os::raw::c_char,
                          ::std::mem::size_of::<z_stream>() as std::os::raw::c_ulong
                              as std::os::raw::c_int);
        if ret != 0 as std::os::raw::c_int {
            free((*state).out as *mut std::os::raw::c_void);
            free((*state).in_0 as *mut std::os::raw::c_void);
            gz_error(state, -(4 as std::os::raw::c_int),
                     b"out of memory\x00" as *const u8 as
                         *const std::os::raw::c_char);
            return -(1 as std::os::raw::c_int)
        }
        (*strm).next_in = 0 as *mut Bytef
    }
    /* mark state as initialized */
    (*state).size = (*state).want;
    /* initialize write buffer if compressing */
    if (*state).direct == 0 {
        (*strm).avail_out = (*state).size;
        (*strm).next_out = (*state).out;
        (*state).x.next = (*strm).next_out
    }
    return 0 as std::os::raw::c_int;
}
/* Compress whatever is at avail_in and next_in and write to the output file.
   Return -1 if there is an error writing to the output file or if gz_init()
   fails to allocate memory, otherwise 0.  flush is assumed to be a valid
   deflate() flush value.  If flush is Z_FINISH, then the deflate() state is
   reset to start a new gzip stream.  If gz->direct is true, then simply write
   to the output file without compressing, and ignore flush. */
unsafe extern "C" fn gz_comp(mut state: gz_statep, mut flush: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0;
    let mut writ: std::os::raw::c_int = 0;
    let mut have: std::os::raw::c_uint = 0;
    let mut put: std::os::raw::c_uint = 0;
    let mut max: std::os::raw::c_uint =
        (-(1 as std::os::raw::c_int) as std::os::raw::c_uint >>
             2 as std::os::raw::c_int).wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_uint);
    let mut strm: z_streamp = &mut (*state).strm;
    /* allocate memory if this is the first time through */
    if (*state).size == 0 as std::os::raw::c_int as std::os::raw::c_uint &&
           gz_init(state) == -(1 as std::os::raw::c_int) {
        return -(1 as std::os::raw::c_int)
    }
    /* write directly if requested */
    if (*state).direct != 0 {
        while (*strm).avail_in != 0 {
            put = if (*strm).avail_in > max { max } else { (*strm).avail_in };
            writ =
                write((*state).fd, (*strm).next_in as *const std::os::raw::c_void,
                      put as size_t) as std::os::raw::c_int;
            if writ < 0 as std::os::raw::c_int {
                gz_error(state, -(1 as std::os::raw::c_int),
                         strerror(*__errno_location()));
                return -(1 as std::os::raw::c_int)
            }
            (*strm).avail_in =
                ((*strm).avail_in as
                     std::os::raw::c_uint).wrapping_sub(writ as std::os::raw::c_uint) as uInt
                    as uInt;
            (*strm).next_in = (*strm).next_in.offset(writ as isize)
        }
        return 0 as std::os::raw::c_int
    }
    /* run deflate() on provided input until it produces no more output */
    ret = 0 as std::os::raw::c_int;
    loop  {
        /* write out current buffer contents if full, or if flushing, but if
           doing Z_FINISH then don't write until we get to Z_STREAM_END */
        if (*strm).avail_out == 0 as std::os::raw::c_int as std::os::raw::c_uint ||
               flush != 0 as std::os::raw::c_int &&
                   (flush != 4 as std::os::raw::c_int || ret == 1 as std::os::raw::c_int) {
            while (*strm).next_out > (*state).x.next {
                put =
                    if (*strm).next_out.offset_from((*state).x.next)
                           as std::os::raw::c_long >
                           max as std::os::raw::c_int as std::os::raw::c_long {
                        max
                    } else {
                        (*strm).next_out.offset_from((*state).x.next)
                            as std::os::raw::c_long as std::os::raw::c_uint
                    };
                writ =
                    write((*state).fd, (*state).x.next as *const std::os::raw::c_void,
                          put as size_t) as std::os::raw::c_int;
                if writ < 0 as std::os::raw::c_int {
                    gz_error(state, -(1 as std::os::raw::c_int),
                             strerror(*__errno_location()));
                    return -(1 as std::os::raw::c_int)
                }
                (*state).x.next = (*state).x.next.offset(writ as isize)
            }
            if (*strm).avail_out == 0 as std::os::raw::c_int as std::os::raw::c_uint {
                (*strm).avail_out = (*state).size;
                (*strm).next_out = (*state).out;
                (*state).x.next = (*state).out
            }
        }
        /* compress */
        have = (*strm).avail_out;
        ret = deflate(strm, flush);
        if ret == -(2 as std::os::raw::c_int) {
            gz_error(state, -(2 as std::os::raw::c_int),
                     b"internal error: deflate stream corrupt\x00" as
                         *const u8 as *const std::os::raw::c_char);
            return -(1 as std::os::raw::c_int)
        }
        have = have.wrapping_sub((*strm).avail_out);
        if !(have != 0) { break ; }
    }
    /* if that completed a deflate stream, allow another to start */
    if flush == 4 as std::os::raw::c_int { deflateReset(strm); }
    /* all done, no errors */
    return 0 as std::os::raw::c_int;
}
/* Compress len zeros to output.  Return -1 on a write error or memory
   allocation failure by gz_comp(), or 0 on success. */
unsafe extern "C" fn gz_zero(mut state: gz_statep, mut len: off64_t)
 -> std::os::raw::c_int {
    let mut first: std::os::raw::c_int = 0;
    let mut n: std::os::raw::c_uint = 0;
    let mut strm: z_streamp = &mut (*state).strm;
    /* consume whatever's left in the input buffer */
    if (*strm).avail_in != 0 &&
           gz_comp(state, 0 as std::os::raw::c_int) == -(1 as std::os::raw::c_int) {
        return -(1 as std::os::raw::c_int)
    }
    /* compress len zeros (len guaranteed > 0) */
    first = 1 as std::os::raw::c_int;
    while len != 0 {
        n =
            if ::std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong ==
                   ::std::mem::size_of::<off64_t>() as std::os::raw::c_ulong &&
                   (*state).size > 2147483647 as std::os::raw::c_int as std::os::raw::c_uint
                   || (*state).size as off64_t > len {
                len as std::os::raw::c_uint
            } else { (*state).size };
        if first != 0 {
            memset((*state).in_0 as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
                   n as std::os::raw::c_ulong);
            first = 0 as std::os::raw::c_int
        }
        (*strm).avail_in = n;
        (*strm).next_in = (*state).in_0;
        (*state).x.pos += n as std::os::raw::c_long;
        if gz_comp(state, 0 as std::os::raw::c_int) == -(1 as std::os::raw::c_int) {
            return -(1 as std::os::raw::c_int)
        }
        len -= n as std::os::raw::c_long
    }
    return 0 as std::os::raw::c_int;
}
/* Write len bytes from buf to file.  Return the number of bytes written.  If
   the returned value is less than len, then there was an error. */
unsafe extern "C" fn gz_write(mut state: gz_statep, mut buf: voidpc,
                              mut len: z_size_t) -> z_size_t {
    let mut put: z_size_t = len;
    /* if len is zero, avoid unnecessary operations */
    if len == 0 as std::os::raw::c_int as std::os::raw::c_ulong {
        return 0 as std::os::raw::c_int as z_size_t
    }
    /* allocate memory if this is the first time through */
    if (*state).size == 0 as std::os::raw::c_int as std::os::raw::c_uint &&
           gz_init(state) == -(1 as std::os::raw::c_int) {
        return 0 as std::os::raw::c_int as z_size_t
    }
    /* check for seek request */
    if (*state).seek != 0 {
        (*state).seek = 0 as std::os::raw::c_int;
        if gz_zero(state, (*state).skip) == -(1 as std::os::raw::c_int) {
            return 0 as std::os::raw::c_int as z_size_t
        }
    }
    /* for small len, copy to input buffer, otherwise compress directly */
    if len < (*state).size as std::os::raw::c_ulong {
        loop 
             /* copy to input buffer, compress when full */
             {
            let mut have: std::os::raw::c_uint = 0;
            let mut copy: std::os::raw::c_uint = 0;
            if (*state).strm.avail_in == 0 as std::os::raw::c_int as std::os::raw::c_uint {
                (*state).strm.next_in = (*state).in_0
            }
            have =
                (*state).strm.next_in.offset((*state).strm.avail_in as
                                                 isize).offset_from((*state).in_0)
                    as std::os::raw::c_long as std::os::raw::c_uint;
            copy = (*state).size.wrapping_sub(have);
            if copy as std::os::raw::c_ulong > len { copy = len as std::os::raw::c_uint }
            memcpy((*state).in_0.offset(have as isize) as *mut std::os::raw::c_void,
                   buf, copy as std::os::raw::c_ulong);
            (*state).strm.avail_in =
                ((*state).strm.avail_in as std::os::raw::c_uint).wrapping_add(copy) as
                    uInt as uInt;
            (*state).x.pos += copy as std::os::raw::c_long;
            buf =
                (buf as *const std::os::raw::c_char).offset(copy as isize) as voidpc;
            len =
                (len as std::os::raw::c_ulong).wrapping_sub(copy as std::os::raw::c_ulong) as
                    z_size_t as z_size_t;
            if len != 0 &&
                   gz_comp(state, 0 as std::os::raw::c_int) == -(1 as std::os::raw::c_int) {
                return 0 as std::os::raw::c_int as z_size_t
            }
            if !(len != 0) { break ; }
        }
    } else {
        /* consume whatever's left in the input buffer */
        if (*state).strm.avail_in != 0 &&
               gz_comp(state, 0 as std::os::raw::c_int) == -(1 as std::os::raw::c_int) {
            return 0 as std::os::raw::c_int as z_size_t
        }
        /* directly compress user buffer to file */
        (*state).strm.next_in = buf as *mut Bytef;
        loop  {
            let mut n: std::os::raw::c_uint = -(1 as std::os::raw::c_int) as std::os::raw::c_uint;
            if n as std::os::raw::c_ulong > len { n = len as std::os::raw::c_uint }
            (*state).strm.avail_in = n;
            (*state).x.pos += n as std::os::raw::c_long;
            if gz_comp(state, 0 as std::os::raw::c_int) == -(1 as std::os::raw::c_int) {
                return 0 as std::os::raw::c_int as z_size_t
            }
            len =
                (len as std::os::raw::c_ulong).wrapping_sub(n as std::os::raw::c_ulong) as
                    z_size_t as z_size_t;
            if !(len != 0) { break ; }
        }
    }
    /* input was all buffered or compressed */
    return put;
}
/* -- see zlib.h -- */
#[no_mangle]
pub unsafe extern "C" fn gzwrite(mut file: gzFile, mut buf: voidpc,
                                 mut len: std::os::raw::c_uint) -> std::os::raw::c_int {
    let mut state: gz_statep = 0 as *mut gz_state;
    /* get internal structure */
    if file.is_null() { return 0 as std::os::raw::c_int }
    state = file as gz_statep;
    /* check that we're writing and that there's no error */
    if (*state).mode != 31153 as std::os::raw::c_int ||
           (*state).err != 0 as std::os::raw::c_int {
        return 0 as std::os::raw::c_int
    }
    /* since an int is returned, make sure len fits in one, otherwise return
       with an error (this avoids a flaw in the interface) */
    if (len as std::os::raw::c_int) < 0 as std::os::raw::c_int {
        gz_error(state, -(3 as std::os::raw::c_int),
                 b"requested length does not fit in int\x00" as *const u8 as
                     *const std::os::raw::c_char);
        return 0 as std::os::raw::c_int
    }
    /* write len bytes from buf (the return value will fit in an int) */
    return gz_write(state, buf, len as z_size_t) as std::os::raw::c_int;
}
/* -- see zlib.h -- */
#[no_mangle]
pub unsafe extern "C" fn gzfwrite(mut buf: voidpc, mut size: z_size_t,
                                  mut nitems: z_size_t, mut file: gzFile)
 -> z_size_t {
    let mut len: z_size_t = 0;
    let mut state: gz_statep = 0 as *mut gz_state;
    /* get internal structure */
    if file.is_null() { return 0 as std::os::raw::c_int as z_size_t }
    state = file as gz_statep;
    /* check that we're writing and that there's no error */
    if (*state).mode != 31153 as std::os::raw::c_int ||
           (*state).err != 0 as std::os::raw::c_int {
        return 0 as std::os::raw::c_int as z_size_t
    }
    /* compute bytes to read -- error on overflow */
    len = nitems.wrapping_mul(size);
    if size != 0 && len.wrapping_div(size) != nitems {
        gz_error(state, -(2 as std::os::raw::c_int),
                 b"request does not fit in a size_t\x00" as *const u8 as
                     *const std::os::raw::c_char);
        return 0 as std::os::raw::c_int as z_size_t
    }
    /* write len bytes to buf, return the number of full items written */
    return if len != 0 {
               gz_write(state, buf, len).wrapping_div(size)
           } else { 0 as std::os::raw::c_int as std::os::raw::c_ulong };
}
/* -- see zlib.h -- */
#[no_mangle]
pub unsafe extern "C" fn gzputc(mut file: gzFile, mut c: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut have: std::os::raw::c_uint = 0;
    let mut buf: [std::os::raw::c_uchar; 1] = [0; 1];
    let mut state: gz_statep = 0 as *mut gz_state;
    let mut strm: z_streamp = 0 as *mut z_stream;
    /* get internal structure */
    if file.is_null() { return -(1 as std::os::raw::c_int) }
    state = file as gz_statep;
    strm = &mut (*state).strm;
    /* check that we're writing and that there's no error */
    if (*state).mode != 31153 as std::os::raw::c_int ||
           (*state).err != 0 as std::os::raw::c_int {
        return -(1 as std::os::raw::c_int)
    }
    /* check for seek request */
    if (*state).seek != 0 {
        (*state).seek = 0 as std::os::raw::c_int;
        if gz_zero(state, (*state).skip) == -(1 as std::os::raw::c_int) {
            return -(1 as std::os::raw::c_int)
        }
    }
    /* try writing to input buffer for speed (state->size == 0 if buffer not
       initialized) */
    if (*state).size != 0 {
        if (*strm).avail_in == 0 as std::os::raw::c_int as std::os::raw::c_uint {
            (*strm).next_in = (*state).in_0
        }
        have =
            (*strm).next_in.offset((*strm).avail_in as
                                       isize).offset_from((*state).in_0)
                as std::os::raw::c_long as std::os::raw::c_uint;
        if have < (*state).size {
            *(*state).in_0.offset(have as isize) = c as std::os::raw::c_uchar;
            (*strm).avail_in = (*strm).avail_in.wrapping_add(1);
            (*state).x.pos += 1;
            return c & 0xff as std::os::raw::c_int
        }
    }
    /* no room in buffer or not initialized, use gz_write() */
    buf[0 as std::os::raw::c_int as usize] = c as std::os::raw::c_uchar;
    if gz_write(state, buf.as_mut_ptr() as voidpc,
                1 as std::os::raw::c_int as z_size_t) !=
           1 as std::os::raw::c_int as std::os::raw::c_ulong {
        return -(1 as std::os::raw::c_int)
    }
    return c & 0xff as std::os::raw::c_int;
}
/* -- see zlib.h -- */
#[no_mangle]
pub unsafe extern "C" fn gzputs(mut file: gzFile,
                                mut str: *const std::os::raw::c_char) -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0;
    let mut len: z_size_t = 0;
    let mut state: gz_statep = 0 as *mut gz_state;
    /* get internal structure */
    if file.is_null() { return -(1 as std::os::raw::c_int) }
    state = file as gz_statep;
    /* check that we're writing and that there's no error */
    if (*state).mode != 31153 as std::os::raw::c_int ||
           (*state).err != 0 as std::os::raw::c_int {
        return -(1 as std::os::raw::c_int)
    }
    /* write string */
    len = strlen(str);
    ret = gz_write(state, str as voidpc, len) as std::os::raw::c_int;
    return if ret == 0 as std::os::raw::c_int &&
                  len != 0 as std::os::raw::c_int as std::os::raw::c_ulong {
               -(1 as std::os::raw::c_int)
           } else { ret };
}
/* -- see zlib.h -- */
#[no_mangle]
pub unsafe extern "C" fn gzvprintf(mut file: gzFile,
                                   mut format: *const std::os::raw::c_char,
                                   mut va: ::std::ffi::VaList)
 -> std::os::raw::c_int {
    let mut len: std::os::raw::c_int = 0;
    let mut left: std::os::raw::c_uint = 0;
    let mut next: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut state: gz_statep = 0 as *mut gz_state;
    let mut strm: z_streamp = 0 as *mut z_stream;
    /* get internal structure */
    if file.is_null() { return -(2 as std::os::raw::c_int) }
    state = file as gz_statep;
    strm = &mut (*state).strm;
    /* check that we're writing and that there's no error */
    if (*state).mode != 31153 as std::os::raw::c_int ||
           (*state).err != 0 as std::os::raw::c_int {
        return -(2 as std::os::raw::c_int)
    }
    /* make sure we have some buffer space */
    if (*state).size == 0 as std::os::raw::c_int as std::os::raw::c_uint &&
           gz_init(state) == -(1 as std::os::raw::c_int) {
        return (*state).err
    }
    /* check for seek request */
    if (*state).seek != 0 {
        (*state).seek = 0 as std::os::raw::c_int;
        if gz_zero(state, (*state).skip) == -(1 as std::os::raw::c_int) {
            return (*state).err
        }
    }
    /* do the printf() into the input buffer, put length in len -- the input
       buffer is double-sized just for this function, so there is guaranteed to
       be state->size bytes available after the current contents */
    if (*strm).avail_in == 0 as std::os::raw::c_int as std::os::raw::c_uint {
        (*strm).next_in = (*state).in_0
    }
    next =
        (*state).in_0.offset((*strm).next_in.offset_from((*state).in_0)
                                 as std::os::raw::c_long as
                                 isize).offset((*strm).avail_in as isize) as
            *mut std::os::raw::c_char;
    *next.offset((*state).size.wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_uint)
                     as isize) = 0 as std::os::raw::c_int as std::os::raw::c_char;
    len =
        vsnprintf(next, (*state).size as std::os::raw::c_ulong, format,
                  va.as_va_list());
    /* check that printf() results fit in buffer */
    if len == 0 as std::os::raw::c_int || len as std::os::raw::c_uint >= (*state).size ||
           *next.offset((*state).size.wrapping_sub(1 as std::os::raw::c_int as
                                                       std::os::raw::c_uint) as isize)
               as std::os::raw::c_int != 0 as std::os::raw::c_int {
        return 0 as std::os::raw::c_int
    }
    /* update buffer and position, compress first half if past that */
    (*strm).avail_in =
        ((*strm).avail_in as std::os::raw::c_uint).wrapping_add(len as std::os::raw::c_uint)
            as uInt as uInt;
    (*state).x.pos += len as std::os::raw::c_long;
    if (*strm).avail_in >= (*state).size {
        left = (*strm).avail_in.wrapping_sub((*state).size);
        (*strm).avail_in = (*state).size;
        if gz_comp(state, 0 as std::os::raw::c_int) == -(1 as std::os::raw::c_int) {
            return (*state).err
        }
        memcpy((*state).in_0 as *mut std::os::raw::c_void,
               (*state).in_0.offset((*state).size as isize) as
                   *const std::os::raw::c_void, left as std::os::raw::c_ulong);
        (*strm).next_in = (*state).in_0;
        (*strm).avail_in = left
    }
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn gzprintf(mut file: gzFile,
                                  mut format: *const std::os::raw::c_char,
                                  mut args: ...) -> std::os::raw::c_int {
    let mut va: ::std::ffi::VaListImpl;
    let mut ret: std::os::raw::c_int = 0;
    va = args.clone();
    ret = gzvprintf(file, format, va.as_va_list());
    return ret;
}
/* !STDC && !Z_HAVE_STDARG_H */
/* -- see zlib.h -- */
#[no_mangle]
pub unsafe extern "C" fn gzflush(mut file: gzFile, mut flush: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut state: gz_statep = 0 as *mut gz_state;
    /* get internal structure */
    if file.is_null() { return -(2 as std::os::raw::c_int) }
    state = file as gz_statep;
    /* check that we're writing and that there's no error */
    if (*state).mode != 31153 as std::os::raw::c_int ||
           (*state).err != 0 as std::os::raw::c_int {
        return -(2 as std::os::raw::c_int)
    }
    /* check flush parameter */
    if flush < 0 as std::os::raw::c_int || flush > 4 as std::os::raw::c_int {
        return -(2 as std::os::raw::c_int)
    }
    /* check for seek request */
    if (*state).seek != 0 {
        (*state).seek = 0 as std::os::raw::c_int;
        if gz_zero(state, (*state).skip) == -(1 as std::os::raw::c_int) {
            return (*state).err
        }
    }
    /* compress remaining data with requested flush */
    gz_comp(state, flush);
    return (*state).err;
}
/* -- see zlib.h -- */
#[no_mangle]
pub unsafe extern "C" fn gzsetparams(mut file: gzFile, mut level: std::os::raw::c_int,
                                     mut strategy: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut state: gz_statep = 0 as *mut gz_state;
    let mut strm: z_streamp = 0 as *mut z_stream;
    /* get internal structure */
    if file.is_null() { return -(2 as std::os::raw::c_int) }
    state = file as gz_statep;
    strm = &mut (*state).strm;
    /* check that we're writing and that there's no error */
    if (*state).mode != 31153 as std::os::raw::c_int ||
           (*state).err != 0 as std::os::raw::c_int {
        return -(2 as std::os::raw::c_int)
    }
    /* if no change is requested, then do nothing */
    if level == (*state).level && strategy == (*state).strategy {
        return 0 as std::os::raw::c_int
    }
    /* check for seek request */
    if (*state).seek != 0 {
        (*state).seek = 0 as std::os::raw::c_int;
        if gz_zero(state, (*state).skip) == -(1 as std::os::raw::c_int) {
            return (*state).err
        }
    }
    /* change compression parameters for subsequent input */
    if (*state).size != 0 {
        /* flush previous input with previous parameters before changing */
        if (*strm).avail_in != 0 &&
               gz_comp(state, 5 as std::os::raw::c_int) == -(1 as std::os::raw::c_int) {
            return (*state).err
        }
        deflateParams(strm, level, strategy);
    }
    (*state).level = level;
    (*state).strategy = strategy;
    return 0 as std::os::raw::c_int;
}
/* -- see zlib.h -- */
#[no_mangle]
pub unsafe extern "C" fn gzclose_w(mut file: gzFile) -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut state: gz_statep = 0 as *mut gz_state;
    /* get internal structure */
    if file.is_null() { return -(2 as std::os::raw::c_int) }
    state = file as gz_statep;
    /* check that we're writing */
    if (*state).mode != 31153 as std::os::raw::c_int { return -(2 as std::os::raw::c_int) }
    /* check for seek request */
    if (*state).seek != 0 {
        (*state).seek = 0 as std::os::raw::c_int;
        if gz_zero(state, (*state).skip) == -(1 as std::os::raw::c_int) {
            ret = (*state).err
        }
    }
    /* flush, free memory, and close file */
    if gz_comp(state, 4 as std::os::raw::c_int) == -(1 as std::os::raw::c_int) {
        ret = (*state).err
    }
    if (*state).size != 0 {
        if (*state).direct == 0 {
            deflateEnd(&mut (*state).strm);
            free((*state).out as *mut std::os::raw::c_void);
        }
        free((*state).in_0 as *mut std::os::raw::c_void);
    }
    gz_error(state, 0 as std::os::raw::c_int, 0 as *const std::os::raw::c_char);
    free((*state).path as *mut std::os::raw::c_void);
    if close((*state).fd) == -(1 as std::os::raw::c_int) { ret = -(1 as std::os::raw::c_int) }
    free(state as *mut std::os::raw::c_void);
    return ret;
}
