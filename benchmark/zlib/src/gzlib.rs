use ::libc;
extern "C" {
    pub type internal_state;
    #[no_mangle]
    fn lseek64(__fd: libc::c_int, __offset: __off64_t, __whence: libc::c_int) -> __off64_t;
    #[no_mangle]
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type off_t = __off_t;
pub type off64_t = __off64_t;
/* zconf.h -- configuration of the zlib compression library
 * Copyright (C) 1995-2016 Jean-loup Gailly, Mark Adler
 * For conditions of distribution and use, see copyright notice in zlib.h
 */
/* @(#) $Id$ */
/* #undef Z_PREFIX */
/*
 * If you *really* need a unique prefix for all types and library functions,
 * compile with -DZ_PREFIX. The "standard" zlib should be compiled without it.
 * Even better than compiling with -DZ_PREFIX would be to use configure to set
 * this permanently in zconf.h using "./configure --zprefix".
 */
/* may be set to #if 1 by ./configure */
/*
 * Compile with -DMAXSEG_64K if the alloc function cannot allocate more
 * than 64k bytes at a time (needed on systems with 16-bit int).
 */
/* iSeries (formerly AS/400). */
pub type z_size_t = size_t;
/* Maximum value for memLevel in deflateInit2 */
/* Maximum value for windowBits in deflateInit2 and inflateInit2.
 * WARNING: reducing MAX_WBITS makes minigzip unable to extract .gz files
 * created by gzip. (Files created by minigzip can still be extracted by
 * gzip.)
 */
/* 32K LZ77 window */
/* The memory requirements for deflate are (in bytes):
            (1 << (windowBits+2)) +  (1 << (memLevel+9))
 that is: 128K for windowBits=15  +  128K for memLevel = 8  (default values)
 plus a few kilobytes for small objects. For example, if you want to reduce
 the default memory requirements from 256K to 128K, compile with
     make CFLAGS="-O -DMAX_WBITS=14 -DMAX_MEM_LEVEL=7"
 Of course this will generally degrade compression (there's no free lunch).

   The memory requirements for inflate are (in bytes) 1 << windowBits
 that is, 32K for windowBits=15 (default value) plus about 7 kilobytes
 for small objects.
*/
/* Type declarations */
/* function prototypes */
/* function prototypes for stdarg */
/* The following definitions for FAR are needed only for MSDOS mixed
 * model programming (small or medium model with some far allocations).
 * This was tested only with MSC; for other MSDOS compilers you may have
 * to define NO_MEMCPY in zutil.h.  If you don't need the mixed model,
 * just define FAR to be empty.
 */
pub type Byte = libc::c_uchar;
/* 8 bits */
pub type uInt = libc::c_uint;
/* 16 bits or more */
pub type uLong = libc::c_ulong;
/* 32 bits or more */
pub type Bytef = Byte;
pub type voidpf = *mut libc::c_void;
pub type alloc_func = Option<unsafe extern "C" fn(_: voidpf, _: uInt, _: uInt) -> voidpf>;
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
    pub msg: *mut libc::c_char,
    pub state: *mut internal_state,
    pub zalloc: alloc_func,
    pub zfree: free_func,
    pub opaque: voidpf,
    pub data_type: libc::c_int,
    pub adler: uLong,
    pub reserved: uLong,
}
pub type z_stream = z_stream_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gzFile_s {
    pub have: libc::c_uint,
    pub next: *mut libc::c_uchar,
    pub pos: off64_t,
}
pub type gzFile = *mut gzFile_s;
pub type gz_statep = *mut gz_state;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gz_state {
    pub x: gzFile_s,
    pub mode: libc::c_int,
    pub fd: libc::c_int,
    pub path: *mut libc::c_char,
    pub size: libc::c_uint,
    pub want: libc::c_uint,
    pub in_0: *mut libc::c_uchar,
    pub out: *mut libc::c_uchar,
    pub direct: libc::c_int,
    pub how: libc::c_int,
    pub start: off64_t,
    pub eof: libc::c_int,
    pub past: libc::c_int,
    pub level: libc::c_int,
    pub strategy: libc::c_int,
    pub skip: off64_t,
    pub seek: libc::c_int,
    pub err: libc::c_int,
    pub msg: *mut libc::c_char,
    pub strm: z_stream,
}
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const Z_OK: libc::c_int = 0 as libc::c_int;
pub const Z_MEM_ERROR: libc::c_int = -(4 as libc::c_int);
pub const Z_BUF_ERROR: libc::c_int = -(5 as libc::c_int);
pub const LOOK: libc::c_int = 0 as libc::c_int;
pub const GZ_READ: libc::c_int = 7247 as libc::c_int;
pub const SEEK_CUR: libc::c_int = 1 as libc::c_int;
pub const GZ_WRITE: libc::c_int = 31153 as libc::c_int;
pub const SEEK_END: libc::c_int = 2 as libc::c_int;
pub const GZ_APPEND: libc::c_int = 1 as libc::c_int;
pub const O_APPEND: libc::c_int = 0o2000 as libc::c_int;
pub const O_TRUNC: libc::c_int = 0o1000 as libc::c_int;
pub const O_EXCL: libc::c_int = 0o200 as libc::c_int;
pub const O_CREAT: libc::c_int = 0o100 as libc::c_int;
pub const O_WRONLY: libc::c_int = 0o1 as libc::c_int;
pub const O_RDONLY: libc::c_int = 0 as libc::c_int;
pub const __O_CLOEXEC: libc::c_int = 0o2000000 as libc::c_int;
pub const O_CLOEXEC: libc::c_int = __O_CLOEXEC;
pub const __O_LARGEFILE: libc::c_int = 0 as libc::c_int;
pub const O_LARGEFILE: libc::c_int = __O_LARGEFILE;
pub const GZ_NONE: libc::c_int = 0 as libc::c_int;
pub const Z_FIXED: libc::c_int = 4 as libc::c_int;
pub const Z_RLE: libc::c_int = 3 as libc::c_int;
pub const Z_HUFFMAN_ONLY: libc::c_int = 2 as libc::c_int;
pub const Z_FILTERED: libc::c_int = 1 as libc::c_int;
pub const Z_DEFAULT_STRATEGY: libc::c_int = 0 as libc::c_int;
pub const Z_DEFAULT_COMPRESSION: libc::c_int = -(1 as libc::c_int);
pub const GZBUFSIZE: libc::c_int = 8192 as libc::c_int;
pub const SEEK_SET: libc::c_int = 0 as libc::c_int;
pub const INT_MAX: libc::c_int = 2147483647 as libc::c_int;
pub const COPY: libc::c_int = 1 as libc::c_int;
/* gzlib.c -- zlib functions common to reading and writing gzip files
 * Copyright (C) 2004-2017 Mark Adler
 * For conditions of distribution and use, see copyright notice in zlib.h
 */
pub const LSEEK: unsafe extern "C" fn(_: libc::c_int, _: __off64_t, _: libc::c_int) -> __off64_t =
    lseek64;
pub const __INT_MAX__: libc::c_int = 2147483647 as libc::c_int;
/* Local functions */
/* UNDER_CE */
/* Reset gzip file state */
unsafe extern "C" fn gz_reset(mut state: gz_statep) {
    (*state).x.have = 0 as libc::c_int as libc::c_uint; /* no output data available */
    if (*state).mode == GZ_READ {
        /* for reading ... */
        (*state).eof = 0 as libc::c_int; /* not at end of file */
        /* look for gzip header */
        (*state).past = 0 as libc::c_int; /* have not read past end yet */
        (*state).how = LOOK
    } /* no seek request pending */
    (*state).seek = 0 as libc::c_int; /* clear error */
    gz_error(state, Z_OK, NULL as *const libc::c_char); /* no uncompressed data yet */
    (*state).x.pos = 0 as libc::c_int as off64_t;
    (*state).strm.avail_in = 0 as libc::c_int as uInt;
    /* no input data yet */
}
/* Open a gzip file either by name or file descriptor. */
unsafe extern "C" fn gz_open(
    mut path: *const libc::c_void,
    mut fd: libc::c_int,
    mut mode: *const libc::c_char,
) -> gzFile {
    let mut state: gz_statep = 0 as *mut gz_state;
    let mut len: z_size_t = 0;
    let mut oflag: libc::c_int = 0;
    let mut cloexec: libc::c_int = 0 as libc::c_int;
    let mut exclusive: libc::c_int = 0 as libc::c_int;
    /* check input */
    if path == NULL as *const libc::c_void {
        return NULL as gzFile;
    }
    /* allocate gzFile structure to return */
    state = malloc(::std::mem::size_of::<gz_state>() as libc::c_ulong) as gz_statep; /* no buffers allocated yet */
    if state.is_null() {
        return NULL as gzFile;
    } /* requested buffer size */
    (*state).size = 0 as libc::c_int as libc::c_uint; /* no error message yet */
    (*state).want = GZBUFSIZE as libc::c_uint;
    (*state).msg = NULL as *mut libc::c_char;
    /* interpret mode */
    (*state).mode = GZ_NONE;
    (*state).level = Z_DEFAULT_COMPRESSION;
    (*state).strategy = Z_DEFAULT_STRATEGY;
    (*state).direct = 0 as libc::c_int;
    while *mode != 0 {
        if *mode as libc::c_int >= '0' as i32 && *mode as libc::c_int <= '9' as i32 {
            (*state).level = *mode as libc::c_int - '0' as i32
        } else {
            match *mode as libc::c_int {
                114 => (*state).mode = GZ_READ,
                119 => (*state).mode = GZ_WRITE,
                97 => (*state).mode = GZ_APPEND,
                43 => {
                    /* can't read and write at the same time */
                    free(state as *mut libc::c_void);
                    return NULL as gzFile;
                }
                98 => {}
                101 => cloexec = 1 as libc::c_int,
                120 => exclusive = 1 as libc::c_int,
                102 => (*state).strategy = Z_FILTERED,
                104 => (*state).strategy = Z_HUFFMAN_ONLY,
                82 => (*state).strategy = Z_RLE,
                70 => (*state).strategy = Z_FIXED,
                84 => (*state).direct = 1 as libc::c_int,
                _ => {}
            }
        }
        mode = mode.offset(1)
    }
    /* must provide an "r", "w", or "a" */
    if (*state).mode == GZ_NONE {
        free(state as *mut libc::c_void);
        return NULL as gzFile;
    }
    /* can't force transparent read */
    if (*state).mode == GZ_READ {
        if (*state).direct != 0 {
            free(state as *mut libc::c_void);
            return NULL as gzFile;
        }
        (*state).direct = 1 as libc::c_int
        /* for empty file */
    }
    /* save the path name for error messages */
    len = strlen(path as *const libc::c_char);
    (*state).path =
        malloc(len.wrapping_add(1 as libc::c_int as libc::c_ulong)) as *mut libc::c_char;
    if (*state).path.is_null() {
        free(state as *mut libc::c_void);
        return NULL as gzFile;
    }
    snprintf(
        (*state).path,
        len.wrapping_add(1 as libc::c_int as libc::c_ulong),
        b"%s\x00" as *const u8 as *const libc::c_char,
        path as *const libc::c_char,
    );
    /* compute the flags for open() */
    oflag = O_LARGEFILE
        | (if cloexec != 0 {
            O_CLOEXEC
        } else {
            0 as libc::c_int
        })
        | (if (*state).mode == GZ_READ {
            O_RDONLY
        } else {
            (O_WRONLY
                | O_CREAT
                | (if exclusive != 0 {
                    O_EXCL
                } else {
                    0 as libc::c_int
                }))
                | (if (*state).mode == GZ_WRITE {
                    O_TRUNC
                } else {
                    O_APPEND
                })
        });
    /* open the file with the appropriate flags (or just use fd) */
    (*state).fd = if fd > -(1 as libc::c_int) {
        fd
    } else {
        open(path as *const libc::c_char, oflag, 0o666 as libc::c_int)
    }; /* so gzoffset() is correct */
    if (*state).fd == -(1 as libc::c_int) {
        free((*state).path as *mut libc::c_void);
        free(state as *mut libc::c_void);
        return NULL as gzFile;
    }
    if (*state).mode == GZ_APPEND {
        lseek64((*state).fd, 0 as libc::c_int as __off64_t, SEEK_END);
        (*state).mode = GZ_WRITE
        /* simplify later checks */
    }
    /* save the current position for rewinding (only if reading) */
    if (*state).mode == GZ_READ {
        (*state).start = lseek64((*state).fd, 0 as libc::c_int as __off64_t, SEEK_CUR);
        if (*state).start == -(1 as libc::c_int) as libc::c_long {
            (*state).start = 0 as libc::c_int as off64_t
        }
    }
    /* initialize stream */
    gz_reset(state);
    /* return stream */
    return state as gzFile;
}
/* -- see zlib.h -- */
#[no_mangle]
pub unsafe extern "C" fn gzopen(
    mut path: *const libc::c_char,
    mut mode: *const libc::c_char,
) -> gzFile {
    return gz_open(path as *const libc::c_void, -(1 as libc::c_int), mode);
}
/* -- see zlib.h -- */
#[no_mangle]
pub unsafe extern "C" fn gzopen64(
    mut path: *const libc::c_char,
    mut mode: *const libc::c_char,
) -> gzFile {
    return gz_open(path as *const libc::c_void, -(1 as libc::c_int), mode);
}
/* -- see zlib.h -- */
#[no_mangle]
pub unsafe extern "C" fn gzdopen(mut fd: libc::c_int, mut mode: *const libc::c_char) -> gzFile {
    let mut path: *mut libc::c_char = 0 as *mut libc::c_char; /* identifier for error messages */
    let mut gz: gzFile = 0 as *mut gzFile_s;
    if fd == -(1 as libc::c_int) || {
        path = malloc(
            (7 as libc::c_int as libc::c_ulong).wrapping_add(
                (3 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
            ),
        ) as *mut libc::c_char;
        path.is_null()
    } {
        return NULL as gzFile;
    }
    snprintf(
        path,
        (7 as libc::c_int as libc::c_ulong).wrapping_add(
            (3 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ),
        b"<fd:%d>\x00" as *const u8 as *const libc::c_char,
        fd,
    );
    gz = gz_open(path as *const libc::c_void, fd, mode);
    free(path as *mut libc::c_void);
    return gz;
}
/* -- see zlib.h -- */
/* -- see zlib.h -- */
#[no_mangle]
pub unsafe extern "C" fn gzbuffer(mut file: gzFile, mut size: libc::c_uint) -> libc::c_int {
    let mut state: gz_statep = 0 as *mut gz_state;
    /* get internal structure and check integrity */
    if file.is_null() {
        return -(1 as libc::c_int);
    }
    state = file as gz_statep;
    if (*state).mode != GZ_READ && (*state).mode != GZ_WRITE {
        return -(1 as libc::c_int);
    }
    /* make sure we haven't already allocated memory */
    if (*state).size != 0 as libc::c_int as libc::c_uint {
        return -(1 as libc::c_int);
    }
    /* check and set requested size */
    if (size << 1 as libc::c_int) < size {
        return -(1 as libc::c_int);
    } /* need to be able to double it */
    if size < 2 as libc::c_int as libc::c_uint {
        size = 2 as libc::c_int as libc::c_uint
    } /* need two bytes to check magic header */
    (*state).want = size;
    return 0 as libc::c_int;
}
/* -- see zlib.h -- */
#[no_mangle]
pub unsafe extern "C" fn gzrewind(mut file: gzFile) -> libc::c_int {
    let mut state: gz_statep = 0 as *mut gz_state;
    /* get internal structure */
    if file.is_null() {
        return -(1 as libc::c_int);
    }
    state = file as gz_statep;
    /* check that we're reading and that there's no error */
    if (*state).mode != GZ_READ || (*state).err != Z_OK && (*state).err != Z_BUF_ERROR {
        return -(1 as libc::c_int);
    }
    /* back up and start over */
    if lseek64((*state).fd, (*state).start, SEEK_SET) == -(1 as libc::c_int) as libc::c_long {
        return -(1 as libc::c_int);
    }
    gz_reset(state);
    return 0 as libc::c_int;
}
/* gzgetc() macro and its supporting function and exposed data structure.  Note
 * that the real internal state is much larger than the exposed structure.
 * This abbreviated structure exposes just enough for the gzgetc() macro.  The
 * user should not mess with these exposed elements, since their names or
 * behavior could change in the future, perhaps even capriciously.  They can
 * only be used by the gzgetc() macro.  You have been warned.
 */
/* backward compatibility */
/* provide 64-bit offset functions if _LARGEFILE64_SOURCE defined, and/or
 * change the regular functions to 64 bits if _FILE_OFFSET_BITS is 64 (if
 * both are true, the application gets the *64 functions, and the regular
 * functions are changed to 64 bits) -- in case these are set on systems
 * without large file support, _LFS64_LARGEFILE must also be true
 */
/* -- see zlib.h -- */
#[no_mangle]
pub unsafe extern "C" fn gzseek64(
    mut file: gzFile,
    mut offset: off64_t,
    mut whence: libc::c_int,
) -> off64_t {
    let mut n: libc::c_uint = 0;
    let mut ret: off64_t = 0;
    let mut state: gz_statep = 0 as *mut gz_state;
    /* get internal structure and check integrity */
    if file.is_null() {
        return -(1 as libc::c_int) as off64_t;
    }
    state = file as gz_statep;
    if (*state).mode != GZ_READ && (*state).mode != GZ_WRITE {
        return -(1 as libc::c_int) as off64_t;
    }
    /* check that there's no error */
    if (*state).err != Z_OK && (*state).err != Z_BUF_ERROR {
        return -(1 as libc::c_int) as off64_t;
    }
    /* can only seek from start or relative to current position */
    if whence != SEEK_SET && whence != SEEK_CUR {
        return -(1 as libc::c_int) as off64_t;
    }
    /* normalize offset to a SEEK_CUR specification */
    if whence == SEEK_SET {
        offset -= (*state).x.pos
    } else if (*state).seek != 0 {
        offset += (*state).skip
    }
    (*state).seek = 0 as libc::c_int;
    /* if within raw area while reading, just go there */
    if (*state).mode == GZ_READ
        && (*state).how == COPY
        && (*state).x.pos + offset >= 0 as libc::c_int as libc::c_long
    {
        ret = lseek64(
            (*state).fd,
            offset - (*state).x.have as libc::c_long,
            SEEK_CUR,
        );
        if ret == -(1 as libc::c_int) as libc::c_long {
            return -(1 as libc::c_int) as off64_t;
        }
        (*state).x.have = 0 as libc::c_int as libc::c_uint;
        (*state).eof = 0 as libc::c_int;
        (*state).past = 0 as libc::c_int;
        (*state).seek = 0 as libc::c_int;
        gz_error(state, Z_OK, NULL as *const libc::c_char);
        (*state).strm.avail_in = 0 as libc::c_int as uInt;
        (*state).x.pos += offset;
        return (*state).x.pos;
    }
    /* calculate skip amount, rewinding if needed for back seek when reading */
    if offset < 0 as libc::c_int as libc::c_long {
        if (*state).mode != GZ_READ {
            /* writing -- can't go backwards */
            return -(1 as libc::c_int) as off64_t;
        }
        offset += (*state).x.pos;
        if offset < 0 as libc::c_int as libc::c_long {
            /* before start of file! */
            return -(1 as libc::c_int) as off64_t;
        }
        if gzrewind(file) == -(1 as libc::c_int) {
            /* rewind, then skip to offset */
            return -(1 as libc::c_int) as off64_t;
        }
    }
    /* if reading, skip what's in output buffer (one less gzgetc() check) */
    if (*state).mode == GZ_READ {
        n = if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
            == ::std::mem::size_of::<off64_t>() as libc::c_ulong
            && (*state).x.have > INT_MAX as libc::c_uint
            || (*state).x.have as off64_t > offset
        {
            offset as libc::c_uint
        } else {
            (*state).x.have
        };
        (*state).x.have = (*state).x.have.wrapping_sub(n);
        (*state).x.next = (*state).x.next.offset(n as isize);
        (*state).x.pos += n as libc::c_long;
        offset -= n as libc::c_long
    }
    /* request skip (if not zero) */
    if offset != 0 {
        (*state).seek = 1 as libc::c_int;
        (*state).skip = offset
    }
    return (*state).x.pos + offset;
}
/* -- see zlib.h -- */
#[no_mangle]
pub unsafe extern "C" fn gzseek(
    mut file: gzFile,
    mut offset: off_t,
    mut whence: libc::c_int,
) -> off_t {
    let mut ret: off64_t = 0;
    ret = gzseek64(file, offset, whence);
    return if ret == ret {
        ret
    } else {
        -(1 as libc::c_int) as libc::c_long
    };
}
/* -- see zlib.h -- */
#[no_mangle]
pub unsafe extern "C" fn gztell64(mut file: gzFile) -> off64_t {
    let mut state: gz_statep = 0 as *mut gz_state;
    /* get internal structure and check integrity */
    if file.is_null() {
        return -(1 as libc::c_int) as off64_t;
    }
    state = file as gz_statep;
    if (*state).mode != GZ_READ && (*state).mode != GZ_WRITE {
        return -(1 as libc::c_int) as off64_t;
    }
    /* return position */
    return (*state).x.pos
        + (if (*state).seek != 0 {
            (*state).skip
        } else {
            0 as libc::c_int as libc::c_long
        });
}
/* -- see zlib.h -- */
#[no_mangle]
pub unsafe extern "C" fn gztell(mut file: gzFile) -> off_t {
    let mut ret: off64_t = 0;
    ret = gztell64(file);
    return if ret == ret {
        ret
    } else {
        -(1 as libc::c_int) as libc::c_long
    };
}
/* -- see zlib.h -- */
#[no_mangle]
pub unsafe extern "C" fn gzoffset64(mut file: gzFile) -> off64_t {
    let mut offset: off64_t = 0;
    let mut state: gz_statep = 0 as *mut gz_state;
    /* get internal structure and check integrity */
    if file.is_null() {
        return -(1 as libc::c_int) as off64_t;
    }
    state = file as gz_statep;
    if (*state).mode != GZ_READ && (*state).mode != GZ_WRITE {
        return -(1 as libc::c_int) as off64_t;
    }
    /* compute and return effective offset in file */
    offset = lseek64((*state).fd, 0 as libc::c_int as __off64_t, SEEK_CUR); /* don't count buffered input */
    if offset == -(1 as libc::c_int) as libc::c_long {
        return -(1 as libc::c_int) as off64_t;
    }
    if (*state).mode == GZ_READ {
        /* reading */
        offset -= (*state).strm.avail_in as libc::c_long
    }
    return offset;
}
/* -- see zlib.h -- */
#[no_mangle]
pub unsafe extern "C" fn gzoffset(mut file: gzFile) -> off_t {
    let mut ret: off64_t = 0;
    ret = gzoffset64(file);
    return if ret == ret {
        ret
    } else {
        -(1 as libc::c_int) as libc::c_long
    };
}
/* -- see zlib.h -- */
#[no_mangle]
pub unsafe extern "C" fn gzeof(mut file: gzFile) -> libc::c_int {
    let mut state: gz_statep = 0 as *mut gz_state;
    /* get internal structure and check integrity */
    if file.is_null() {
        return 0 as libc::c_int;
    }
    state = file as gz_statep;
    if (*state).mode != GZ_READ && (*state).mode != GZ_WRITE {
        return 0 as libc::c_int;
    }
    /* return end-of-file state */
    return if (*state).mode == GZ_READ {
        (*state).past
    } else {
        0 as libc::c_int
    };
}
/* -- see zlib.h -- */
#[no_mangle]
pub unsafe extern "C" fn gzerror(
    mut file: gzFile,
    mut errnum: *mut libc::c_int,
) -> *const libc::c_char {
    let mut state: gz_statep = 0 as *mut gz_state;
    /* get internal structure and check integrity */
    if file.is_null() {
        return NULL as *const libc::c_char;
    }
    state = file as gz_statep;
    if (*state).mode != GZ_READ && (*state).mode != GZ_WRITE {
        return NULL as *const libc::c_char;
    }
    /* return error information */
    if !errnum.is_null() {
        *errnum = (*state).err
    }
    return if (*state).err == Z_MEM_ERROR {
        b"out of memory\x00" as *const u8 as *const libc::c_char
    } else if (*state).msg.is_null() {
        b"\x00" as *const u8 as *const libc::c_char
    } else {
        (*state).msg
    };
}
/* -- see zlib.h -- */
#[no_mangle]
pub unsafe extern "C" fn gzclearerr(mut file: gzFile) {
    let mut state: gz_statep = 0 as *mut gz_state;
    /* get internal structure and check integrity */
    if file.is_null() {
        return;
    }
    state = file as gz_statep;
    if (*state).mode != GZ_READ && (*state).mode != GZ_WRITE {
        return;
    }
    /* clear error and end-of-file */
    if (*state).mode == GZ_READ {
        (*state).eof = 0 as libc::c_int;
        (*state).past = 0 as libc::c_int
    }
    gz_error(state, Z_OK, NULL as *const libc::c_char);
}
/* gzguts.h -- zlib internal header definitions for gz* operations
 * Copyright (C) 2004, 2005, 2010, 2011, 2012, 2013, 2016 Mark Adler
 * For conditions of distribution and use, see copyright notice in zlib.h
 */
/* for compatibility with old definition */
/* unlike snprintf (which is required in C99), _snprintf does not guarantee
null termination of the result -- however this is only used in gzlib.c where
the result is assured to fit in the space provided */
/* since "static" is used to mean two completely different things in C, we
define "local" for the non-static meaning of "static", for readability
(compile with -Dlocal if your debugger can't find static symbols) */
/* gz* functions always use library allocation functions */
/* get errno and strerror definition */
/* provide prototypes for these when building zlib without LFS */
/* default memLevel */
/* default i/o buffer size -- double this for output when reading (this and
twice this must be able to fit in an unsigned type) */
/* gzip modes, also provide a little integrity check on the passed structure */
/* mode set to GZ_WRITE after the file is opened */
/* values for gz_state how */
/* look for a gzip header */
/* copy input directly */
/* decompress a gzip stream */
/* internal gzip file state data structure */
/* exposed contents for gzgetc() macro */
/* "x" for exposed */
/* x.have: number of bytes available at x.next */
/* x.next: next output data to deliver or write */
/* x.pos: current position in uncompressed data */
/* used for both reading and writing */
/* see gzip modes above */
/* file descriptor */
/* path or fd for error messages */
/* buffer size, zero if not allocated yet */
/* requested buffer size, default is GZBUFSIZE */
/* input buffer (double-sized when writing) */
/* output buffer (double-sized when reading) */
/* 0 if processing gzip, 1 if transparent */
/* just for reading */
/* 0: get header, 1: copy, 2: decompress */
/* where the gzip data started, for rewinding */
/* true if end of input file reached */
/* true if read requested past end */
/* just for writing */
/* compression level */
/* compression strategy */
/* seek request */
/* amount to skip (already rewound if backwards) */
/* true if seek request pending */
/* error information */
/* error code */
/* error message */
/* zlib inflate or deflate stream */
/* stream structure in-place (not a pointer) */
/* shared functions */
/* Create an error message in allocated memory and set state->err and
state->msg accordingly.  Free any previous error message already there.  Do
not try to free or allocate space if the error is Z_MEM_ERROR (out of
memory).  Simply save the error message as a static string.  If there is an
allocation failure constructing the error message, then convert the error to
out of memory. */
#[no_mangle]
pub unsafe extern "C" fn gz_error(
    mut state: gz_statep,
    mut err: libc::c_int,
    mut msg: *const libc::c_char,
) {
    /* free previously allocated message and clear */
    if !(*state).msg.is_null() {
        if (*state).err != Z_MEM_ERROR {
            free((*state).msg as *mut libc::c_void);
        }
        (*state).msg = NULL as *mut libc::c_char
    }
    /* if fatal, set state->x.have to 0 so that the gzgetc() macro fails */
    if err != Z_OK && err != Z_BUF_ERROR {
        (*state).x.have = 0 as libc::c_int as libc::c_uint
    }
    /* set error code, and if no message, then done */
    (*state).err = err;
    if msg.is_null() {
        return;
    }
    /* for an out of memory error, return literal string when requested */
    if err == Z_MEM_ERROR {
        return;
    }
    /* construct error message with path */
    (*state).msg = malloc(
        strlen((*state).path)
            .wrapping_add(strlen(msg))
            .wrapping_add(3 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    if (*state).msg.is_null() {
        (*state).err = Z_MEM_ERROR;
        return;
    }
    snprintf(
        (*state).msg,
        strlen((*state).path)
            .wrapping_add(strlen(msg))
            .wrapping_add(3 as libc::c_int as libc::c_ulong),
        b"%s%s%s\x00" as *const u8 as *const libc::c_char,
        (*state).path,
        b": \x00" as *const u8 as *const libc::c_char,
        msg,
    );
}
