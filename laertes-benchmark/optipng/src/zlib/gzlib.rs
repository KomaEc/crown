
extern "C" {
    pub type internal_state;
    #[no_mangle]
    fn lseek64(__fd: std::os::raw::c_int, __offset: __off64_t, __whence: std::os::raw::c_int)
     -> __off64_t;
    #[no_mangle]
    fn snprintf(_: *mut std::os::raw::c_char, _: std::os::raw::c_ulong,
                _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    fn free(__ptr: *mut std::os::raw::c_void);
    #[no_mangle]
    fn strlen(_: *const std::os::raw::c_char) -> std::os::raw::c_ulong;
    #[no_mangle]
    fn malloc(_: std::os::raw::c_ulong) -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn open(__file: *const std::os::raw::c_char, __oflag: std::os::raw::c_int, _: ...)
     -> std::os::raw::c_int;
}
pub type size_t = std::os::raw::c_ulong;
pub type __off_t = std::os::raw::c_long;
pub type __off64_t = std::os::raw::c_long;
pub type off_t = __off_t;
pub type off64_t = __off64_t;
pub type z_size_t = size_t;
pub type Byte = std::os::raw::c_uchar;
pub type uInt = std::os::raw::c_uint;
pub type uLong = std::os::raw::c_ulong;
pub type Bytef = Byte;
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
/* Local functions */
/* UNDER_CE */
/* Reset gzip file state */
unsafe extern "C" fn gz_reset(mut state: gz_statep) {
    (*state).x.have =
        0 as std::os::raw::c_int as std::os::raw::c_uint; /* no output data available */
    if (*state).mode == 7247 as std::os::raw::c_int {
        /* for reading ... */
        (*state).eof = 0 as std::os::raw::c_int; /* not at end of file */
        /* look for gzip header */
        (*state).past = 0 as std::os::raw::c_int; /* have not read past end yet */
        (*state).how = 0 as std::os::raw::c_int
    } /* no seek request pending */
    (*state).seek = 0 as std::os::raw::c_int; /* clear error */
    gz_error(state, 0 as std::os::raw::c_int,
             0 as *const std::os::raw::c_char); /* no uncompressed data yet */
    (*state).x.pos = 0 as std::os::raw::c_int as off64_t;
    (*state).strm.avail_in = 0 as std::os::raw::c_int as uInt;
    /* no input data yet */
}
/* Open a gzip file either by name or file descriptor. */
unsafe extern "C" fn gz_open(mut path: *const std::os::raw::c_void,
                             mut fd: std::os::raw::c_int,
                             mut mode: *const std::os::raw::c_char) -> gzFile {
    let mut state: gz_statep = 0 as *mut gz_state;
    let mut len: z_size_t = 0;
    let mut oflag: std::os::raw::c_int = 0;
    let mut cloexec: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut exclusive: std::os::raw::c_int = 0 as std::os::raw::c_int;
    /* check input */
    if path.is_null() { return 0 as gzFile }
    /* allocate gzFile structure to return */
    state =
        malloc(::std::mem::size_of::<gz_state>() as std::os::raw::c_ulong) as
            gz_statep; /* no buffers allocated yet */
    if state.is_null() { return 0 as gzFile } /* requested buffer size */
    (*state).size =
        0 as std::os::raw::c_int as std::os::raw::c_uint; /* no error message yet */
    (*state).want = 8192 as std::os::raw::c_int as std::os::raw::c_uint;
    (*state).msg = 0 as *mut std::os::raw::c_char;
    /* interpret mode */
    (*state).mode = 0 as std::os::raw::c_int;
    (*state).level = -(1 as std::os::raw::c_int);
    (*state).strategy = 0 as std::os::raw::c_int;
    (*state).direct = 0 as std::os::raw::c_int;
    while *mode != 0 {
        if *mode as std::os::raw::c_int >= '0' as i32 &&
               *mode as std::os::raw::c_int <= '9' as i32 {
            (*state).level = *mode as std::os::raw::c_int - '0' as i32
        } else {
            match *mode as std::os::raw::c_int {
                114 => { (*state).mode = 7247 as std::os::raw::c_int }
                43 => {
                    /* can't read and write at the same time */
                    free(state as *mut std::os::raw::c_void);
                    return 0 as gzFile
                }
                98 => { }
                101 => { cloexec = 1 as std::os::raw::c_int }
                120 => { exclusive = 1 as std::os::raw::c_int }
                102 => { (*state).strategy = 1 as std::os::raw::c_int }
                104 => { (*state).strategy = 2 as std::os::raw::c_int }
                82 => { (*state).strategy = 3 as std::os::raw::c_int }
                70 => { (*state).strategy = 4 as std::os::raw::c_int }
                84 => { (*state).direct = 1 as std::os::raw::c_int }
                _ => { }
            }
        }
        mode = mode.offset(1)
    }
    /* must provide an "r", "w", or "a" */
    if (*state).mode == 0 as std::os::raw::c_int {
        free(state as *mut std::os::raw::c_void);
        return 0 as gzFile
    }
    /* can't force transparent read */
    if (*state).mode == 7247 as std::os::raw::c_int {
        if (*state).direct != 0 {
            free(state as *mut std::os::raw::c_void);
            return 0 as gzFile
        }
        (*state).direct = 1 as std::os::raw::c_int
        /* for empty file */
    }
    /* save the path name for error messages */
    len = strlen(path as *const std::os::raw::c_char);
    (*state).path =
        malloc(len.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong)) as
            *mut std::os::raw::c_char;
    if (*state).path.is_null() {
        free(state as *mut std::os::raw::c_void);
        return 0 as gzFile
    }
    snprintf((*state).path,
             len.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong),
             b"%s\x00" as *const u8 as *const std::os::raw::c_char,
             path as *const std::os::raw::c_char);
    /* compute the flags for open() */
    oflag =
        0 as std::os::raw::c_int |
            (if cloexec != 0 {
                 0o2000000 as std::os::raw::c_int
             } else { 0 as std::os::raw::c_int }) |
            (if (*state).mode == 7247 as std::os::raw::c_int {
                 0 as std::os::raw::c_int
             } else {
                 (0o1 as std::os::raw::c_int | 0o100 as std::os::raw::c_int |
                      (if exclusive != 0 {
                           0o200 as std::os::raw::c_int
                       } else { 0 as std::os::raw::c_int })) |
                     (if (*state).mode == 31153 as std::os::raw::c_int {
                          0o1000 as std::os::raw::c_int
                      } else { 0o2000 as std::os::raw::c_int })
             });
    /* open the file with the appropriate flags (or just use fd) */
    (*state).fd =
        if fd > -(1 as std::os::raw::c_int) {
            fd
        } else {
            open(path as *const std::os::raw::c_char, oflag, 0o666 as std::os::raw::c_int)
        }; /* so gzoffset() is correct */
    if (*state).fd == -(1 as std::os::raw::c_int) {
        free((*state).path as *mut std::os::raw::c_void);
        free(state as *mut std::os::raw::c_void);
        return 0 as gzFile
    }
    if (*state).mode == 1 as std::os::raw::c_int {
        lseek64((*state).fd, 0 as std::os::raw::c_int as __off64_t, 2 as std::os::raw::c_int);
        (*state).mode = 31153 as std::os::raw::c_int
        /* simplify later checks */
    }
    /* save the current position for rewinding (only if reading) */
    if (*state).mode == 7247 as std::os::raw::c_int {
        (*state).start =
            lseek64((*state).fd, 0 as std::os::raw::c_int as __off64_t,
                    1 as std::os::raw::c_int);
        if (*state).start == -(1 as std::os::raw::c_int) as std::os::raw::c_long {
            (*state).start = 0 as std::os::raw::c_int as off64_t
        }
    }
    /* initialize stream */
    gz_reset(state);
    /* return stream */
    return state as gzFile;
}
/* -- see zlib.h -- */
#[no_mangle]
pub unsafe extern "C" fn gzopen(mut path: *const std::os::raw::c_char,
                                mut mode: *const std::os::raw::c_char) -> gzFile {
    return gz_open(path as *const std::os::raw::c_void, -(1 as std::os::raw::c_int), mode);
}
/* -- see zlib.h -- */
#[no_mangle]
pub unsafe extern "C" fn gzopen64(mut path: *const std::os::raw::c_char,
                                  mut mode: *const std::os::raw::c_char) -> gzFile {
    return gz_open(path as *const std::os::raw::c_void, -(1 as std::os::raw::c_int), mode);
}
/* -- see zlib.h -- */
#[no_mangle]
pub unsafe extern "C" fn gzdopen(mut fd: std::os::raw::c_int,
                                 mut mode: *const std::os::raw::c_char) -> gzFile {
    let mut path: *mut std::os::raw::c_char =
        0 as *mut std::os::raw::c_char; /* identifier for error messages */
    let mut gz: gzFile = 0 as *mut gzFile_s;
    if fd == -(1 as std::os::raw::c_int) ||
           {
               path =
                   malloc((7 as std::os::raw::c_int as
                               std::os::raw::c_ulong).wrapping_add((3 as std::os::raw::c_int
                                                                as
                                                                std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<std::os::raw::c_int>()
                                                                                                as
                                                                                                std::os::raw::c_ulong)))
                       as *mut std::os::raw::c_char;
               path.is_null()
           } {
        return 0 as gzFile
    }
    snprintf(path,
             (7 as std::os::raw::c_int as
                  std::os::raw::c_ulong).wrapping_add((3 as std::os::raw::c_int as
                                                   std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<std::os::raw::c_int>()
                                                                                   as
                                                                                   std::os::raw::c_ulong)),
             b"<fd:%d>\x00" as *const u8 as *const std::os::raw::c_char, fd);
    gz = gz_open(path as *const std::os::raw::c_void, fd, mode);
    free(path as *mut std::os::raw::c_void);
    return gz;
}
/* -- see zlib.h -- */
/* -- see zlib.h -- */
#[no_mangle]
pub unsafe extern "C" fn gzbuffer(mut file: gzFile, mut size: std::os::raw::c_uint)
 -> std::os::raw::c_int {
    let mut state: gz_statep = 0 as *mut gz_state;
    /* get internal structure and check integrity */
    if file.is_null() { return -(1 as std::os::raw::c_int) }
    state = file as gz_statep;
    if (*state).mode != 7247 as std::os::raw::c_int &&
           (*state).mode != 31153 as std::os::raw::c_int {
        return -(1 as std::os::raw::c_int)
    }
    /* make sure we haven't already allocated memory */
    if (*state).size != 0 as std::os::raw::c_int as std::os::raw::c_uint {
        return -(1 as std::os::raw::c_int)
    }
    /* check and set requested size */
    if (size << 1 as std::os::raw::c_int) < size {
        return -(1 as std::os::raw::c_int)
    } /* need to be able to double it */
    if size < 2 as std::os::raw::c_int as std::os::raw::c_uint {
        size = 2 as std::os::raw::c_int as std::os::raw::c_uint
    } /* need two bytes to check magic header */
    (*state).want = size;
    return 0 as std::os::raw::c_int;
}
/* -- see zlib.h -- */
#[no_mangle]
pub unsafe extern "C" fn gzrewind(mut file: gzFile) -> std::os::raw::c_int {
    let mut state: gz_statep = 0 as *mut gz_state;
    /* get internal structure */
    if file.is_null() { return -(1 as std::os::raw::c_int) }
    state = file as gz_statep;
    /* check that we're reading and that there's no error */
    if (*state).mode != 7247 as std::os::raw::c_int ||
           (*state).err != 0 as std::os::raw::c_int &&
               (*state).err != -(5 as std::os::raw::c_int) {
        return -(1 as std::os::raw::c_int)
    }
    /* back up and start over */
    if lseek64((*state).fd, (*state).start, 0 as std::os::raw::c_int) ==
           -(1 as std::os::raw::c_int) as std::os::raw::c_long {
        return -(1 as std::os::raw::c_int)
    }
    gz_reset(state);
    return 0 as std::os::raw::c_int;
}
/* -- see zlib.h -- */
#[no_mangle]
pub unsafe extern "C" fn gzseek64(mut file: gzFile, mut offset: off64_t,
                                  mut whence: std::os::raw::c_int) -> off64_t {
    let mut n: std::os::raw::c_uint = 0;
    let mut ret: off64_t = 0;
    let mut state: gz_statep = 0 as *mut gz_state;
    /* get internal structure and check integrity */
    if file.is_null() { return -(1 as std::os::raw::c_int) as off64_t }
    state = file as gz_statep;
    if (*state).mode != 7247 as std::os::raw::c_int &&
           (*state).mode != 31153 as std::os::raw::c_int {
        return -(1 as std::os::raw::c_int) as off64_t
    }
    /* check that there's no error */
    if (*state).err != 0 as std::os::raw::c_int && (*state).err != -(5 as std::os::raw::c_int)
       {
        return -(1 as std::os::raw::c_int) as off64_t
    }
    /* can only seek from start or relative to current position */
    if whence != 0 as std::os::raw::c_int && whence != 1 as std::os::raw::c_int {
        return -(1 as std::os::raw::c_int) as off64_t
    }
    /* normalize offset to a SEEK_CUR specification */
    if whence == 0 as std::os::raw::c_int {
        offset -= (*state).x.pos
    } else if (*state).seek != 0 { offset += (*state).skip }
    (*state).seek = 0 as std::os::raw::c_int;
    /* if within raw area while reading, just go there */
    if (*state).mode == 7247 as std::os::raw::c_int &&
           (*state).how == 1 as std::os::raw::c_int &&
           (*state).x.pos + offset >= 0 as std::os::raw::c_int as std::os::raw::c_long {
        ret =
            lseek64((*state).fd, offset - (*state).x.have as std::os::raw::c_long,
                    1 as std::os::raw::c_int);
        if ret == -(1 as std::os::raw::c_int) as std::os::raw::c_long {
            return -(1 as std::os::raw::c_int) as off64_t
        }
        (*state).x.have = 0 as std::os::raw::c_int as std::os::raw::c_uint;
        (*state).eof = 0 as std::os::raw::c_int;
        (*state).past = 0 as std::os::raw::c_int;
        (*state).seek = 0 as std::os::raw::c_int;
        gz_error(state, 0 as std::os::raw::c_int, 0 as *const std::os::raw::c_char);
        (*state).strm.avail_in = 0 as std::os::raw::c_int as uInt;
        (*state).x.pos += offset;
        return (*state).x.pos
    }
    /* calculate skip amount, rewinding if needed for back seek when reading */
    if offset < 0 as std::os::raw::c_int as std::os::raw::c_long {
        if (*state).mode != 7247 as std::os::raw::c_int {
            /* writing -- can't go backwards */
            return -(1 as std::os::raw::c_int) as off64_t
        }
        offset += (*state).x.pos;
        if offset < 0 as std::os::raw::c_int as std::os::raw::c_long {
            /* before start of file! */
            return -(1 as std::os::raw::c_int) as off64_t
        }
        if gzrewind(file) == -(1 as std::os::raw::c_int) {
            /* rewind, then skip to offset */
            return -(1 as std::os::raw::c_int) as off64_t
        }
    }
    /* if reading, skip what's in output buffer (one less gzgetc() check) */
    if (*state).mode == 7247 as std::os::raw::c_int {
        n =
            if ::std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong ==
                   ::std::mem::size_of::<off64_t>() as std::os::raw::c_ulong &&
                   (*state).x.have > 2147483647 as std::os::raw::c_int as std::os::raw::c_uint
                   || (*state).x.have as off64_t > offset {
                offset as std::os::raw::c_uint
            } else { (*state).x.have };
        (*state).x.have = (*state).x.have.wrapping_sub(n);
        (*state).x.next = (*state).x.next.offset(n as isize);
        (*state).x.pos += n as std::os::raw::c_long;
        offset -= n as std::os::raw::c_long
    }
    /* request skip (if not zero) */
    if offset != 0 {
        (*state).seek = 1 as std::os::raw::c_int;
        (*state).skip = offset
    }
    return (*state).x.pos + offset;
}
/* -- see zlib.h -- */
#[no_mangle]
pub unsafe extern "C" fn gzseek(mut file: gzFile, mut offset: off_t,
                                mut whence: std::os::raw::c_int) -> off_t {
    let mut ret: off64_t = 0;
    ret = gzseek64(file, offset, whence);
    return if ret == ret { ret } else { -(1 as std::os::raw::c_int) as std::os::raw::c_long };
}
/* -- see zlib.h -- */
#[no_mangle]
pub unsafe extern "C" fn gztell64(mut file: gzFile) -> off64_t {
    let mut state: gz_statep = 0 as *mut gz_state;
    /* get internal structure and check integrity */
    if file.is_null() { return -(1 as std::os::raw::c_int) as off64_t }
    state = file as gz_statep;
    if (*state).mode != 7247 as std::os::raw::c_int &&
           (*state).mode != 31153 as std::os::raw::c_int {
        return -(1 as std::os::raw::c_int) as off64_t
    }
    /* return position */
    return (*state).x.pos +
               (if (*state).seek != 0 {
                    (*state).skip
                } else { 0 as std::os::raw::c_int as std::os::raw::c_long });
}
/* -- see zlib.h -- */
#[no_mangle]
pub unsafe extern "C" fn gztell(mut file: gzFile) -> off_t {
    let mut ret: off64_t = 0;
    ret = gztell64(file);
    return if ret == ret { ret } else { -(1 as std::os::raw::c_int) as std::os::raw::c_long };
}
/* -- see zlib.h -- */
#[no_mangle]
pub unsafe extern "C" fn gzoffset64(mut file: gzFile) -> off64_t {
    let mut offset: off64_t = 0;
    let mut state: gz_statep = 0 as *mut gz_state;
    /* get internal structure and check integrity */
    if file.is_null() { return -(1 as std::os::raw::c_int) as off64_t }
    state = file as gz_statep;
    if (*state).mode != 7247 as std::os::raw::c_int &&
           (*state).mode != 31153 as std::os::raw::c_int {
        return -(1 as std::os::raw::c_int) as off64_t
    }
    /* compute and return effective offset in file */
    offset =
        lseek64((*state).fd, 0 as std::os::raw::c_int as __off64_t,
                1 as std::os::raw::c_int); /* don't count buffered input */
    if offset == -(1 as std::os::raw::c_int) as std::os::raw::c_long {
        return -(1 as std::os::raw::c_int) as off64_t
    }
    if (*state).mode == 7247 as std::os::raw::c_int {
        /* reading */
        offset -= (*state).strm.avail_in as std::os::raw::c_long
    }
    return offset;
}
/* -- see zlib.h -- */
#[no_mangle]
pub unsafe extern "C" fn gzoffset(mut file: gzFile) -> off_t {
    let mut ret: off64_t = 0;
    ret = gzoffset64(file);
    return if ret == ret { ret } else { -(1 as std::os::raw::c_int) as std::os::raw::c_long };
}
/* -- see zlib.h -- */
#[no_mangle]
pub unsafe extern "C" fn gzeof(mut file: gzFile) -> std::os::raw::c_int {
    let mut state: gz_statep = 0 as *mut gz_state;
    /* get internal structure and check integrity */
    if file.is_null() { return 0 as std::os::raw::c_int }
    state = file as gz_statep;
    if (*state).mode != 7247 as std::os::raw::c_int &&
           (*state).mode != 31153 as std::os::raw::c_int {
        return 0 as std::os::raw::c_int
    }
    /* return end-of-file state */
    return if (*state).mode == 7247 as std::os::raw::c_int {
               (*state).past
           } else { 0 as std::os::raw::c_int };
}
/* -- see zlib.h -- */
#[no_mangle]
pub unsafe extern "C" fn gzerror(mut file: gzFile,
                                 mut errnum: *mut std::os::raw::c_int)
 -> *const std::os::raw::c_char {
    let mut state: gz_statep = 0 as *mut gz_state;
    /* get internal structure and check integrity */
    if file.is_null() { return 0 as *const std::os::raw::c_char }
    state = file as gz_statep;
    if (*state).mode != 7247 as std::os::raw::c_int &&
           (*state).mode != 31153 as std::os::raw::c_int {
        return 0 as *const std::os::raw::c_char
    }
    /* return error information */
    if !errnum.is_null() { *errnum = (*state).err }
    return if (*state).err == -(4 as std::os::raw::c_int) {
               b"out of memory\x00" as *const u8 as *const std::os::raw::c_char
           } else if (*state).msg.is_null() {
               b"\x00" as *const u8 as *const std::os::raw::c_char
           } else { (*state).msg as *const std::os::raw::c_char };
}
/* -- see zlib.h -- */
#[no_mangle]
pub unsafe extern "C" fn gzclearerr(mut file: gzFile) {
    let mut state: gz_statep = 0 as *mut gz_state;
    /* get internal structure and check integrity */
    if file.is_null() { return }
    state = file as gz_statep;
    if (*state).mode != 7247 as std::os::raw::c_int &&
           (*state).mode != 31153 as std::os::raw::c_int {
        return
    }
    /* clear error and end-of-file */
    if (*state).mode == 7247 as std::os::raw::c_int {
        (*state).eof = 0 as std::os::raw::c_int;
        (*state).past = 0 as std::os::raw::c_int
    }
    gz_error(state, 0 as std::os::raw::c_int, 0 as *const std::os::raw::c_char);
}
/* Create an error message in allocated memory and set state->err and
   state->msg accordingly.  Free any previous error message already there.  Do
   not try to free or allocate space if the error is Z_MEM_ERROR (out of
   memory).  Simply save the error message as a static string.  If there is an
   allocation failure constructing the error message, then convert the error to
   out of memory. */
#[no_mangle]
pub unsafe extern "C" fn gz_error(mut state: gz_statep, mut err: std::os::raw::c_int,
                                  mut msg: *const std::os::raw::c_char) {
    /* free previously allocated message and clear */
    if !(*state).msg.is_null() {
        if (*state).err != -(4 as std::os::raw::c_int) {
            free((*state).msg as *mut std::os::raw::c_void);
        }
        (*state).msg = 0 as *mut std::os::raw::c_char
    }
    /* if fatal, set state->x.have to 0 so that the gzgetc() macro fails */
    if err != 0 as std::os::raw::c_int && err != -(5 as std::os::raw::c_int) {
        (*state).x.have = 0 as std::os::raw::c_int as std::os::raw::c_uint
    }
    /* set error code, and if no message, then done */
    (*state).err = err;
    if msg.is_null() { return }
    /* for an out of memory error, return literal string when requested */
    if err == -(4 as std::os::raw::c_int) { return }
    /* construct error message with path */
    (*state).msg =
        malloc(strlen((*state).path).wrapping_add(strlen(msg)).wrapping_add(3
                                                                                as
                                                                                std::os::raw::c_int
                                                                                as
                                                                                std::os::raw::c_ulong))
            as *mut std::os::raw::c_char;
    if (*state).msg.is_null() { (*state).err = -(4 as std::os::raw::c_int); return }
    snprintf((*state).msg,
             strlen((*state).path).wrapping_add(strlen(msg)).wrapping_add(3 as
                                                                              std::os::raw::c_int
                                                                              as
                                                                              std::os::raw::c_ulong),
             b"%s%s%s\x00" as *const u8 as *const std::os::raw::c_char, (*state).path,
             b": \x00" as *const u8 as *const std::os::raw::c_char, msg);
}
