
extern "C" {
    pub type internal_state;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    fn zlibVersion() -> *const std::os::raw::c_char;
    #[no_mangle]
    fn deflate(strm: z_streamp, flush: std::os::raw::c_int) -> std::os::raw::c_int;
    #[no_mangle]
    fn deflateEnd(strm: z_streamp) -> std::os::raw::c_int;
    #[no_mangle]
    fn inflate(strm: z_streamp, flush: std::os::raw::c_int) -> std::os::raw::c_int;
    #[no_mangle]
    fn inflateEnd(strm: z_streamp) -> std::os::raw::c_int;
    #[no_mangle]
    fn deflateSetDictionary(strm: z_streamp, dictionary_0: *const Bytef,
                            dictLength: uInt) -> std::os::raw::c_int;
    #[no_mangle]
    fn deflateParams(strm: z_streamp, level: std::os::raw::c_int,
                     strategy: std::os::raw::c_int) -> std::os::raw::c_int;
    #[no_mangle]
    fn inflateSetDictionary(strm: z_streamp, dictionary_0: *const Bytef,
                            dictLength: uInt) -> std::os::raw::c_int;
    #[no_mangle]
    fn inflateSync(strm: z_streamp) -> std::os::raw::c_int;
    #[no_mangle]
    fn zlibCompileFlags() -> uLong;
    #[no_mangle]
    fn compress(dest: *mut Bytef, destLen: *mut uLongf, source: *const Bytef,
                sourceLen: uLong) -> std::os::raw::c_int;
    #[no_mangle]
    fn uncompress(dest: *mut Bytef, destLen: *mut uLongf,
                  source: *const Bytef, sourceLen: uLong) -> std::os::raw::c_int;
    #[no_mangle]
    fn deflateInit_(strm: z_streamp, level: std::os::raw::c_int,
                    version: *const std::os::raw::c_char, stream_size: std::os::raw::c_int)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn inflateInit_(strm: z_streamp, version: *const std::os::raw::c_char,
                    stream_size: std::os::raw::c_int) -> std::os::raw::c_int;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    fn strcmp(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char) -> std::os::raw::c_int;
    #[no_mangle]
    fn strcpy(_: *mut std::os::raw::c_char, _: *const std::os::raw::c_char)
     -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn strlen(_: *const std::os::raw::c_char) -> std::os::raw::c_ulong;
    #[no_mangle]
    fn calloc(_: std::os::raw::c_ulong, _: std::os::raw::c_ulong) -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn free(__ptr: *mut std::os::raw::c_void);
    #[no_mangle]
    fn exit(_: std::os::raw::c_int) -> !;
}
pub type size_t = std::os::raw::c_ulong;
pub type Byte = std::os::raw::c_uchar;
pub type uInt = std::os::raw::c_uint;
pub type uLong = std::os::raw::c_ulong;
pub type Bytef = Byte;
pub type uLongf = uLong;
pub type voidpf = *mut std::os::raw::c_void;
pub type __off_t = std::os::raw::c_long;
pub type __off64_t = std::os::raw::c_long;
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
static mut hello: [std::os::raw::c_char; 14] =
    unsafe {
        *::std::mem::transmute::<&[u8; 14],
                                 &mut [std::os::raw::c_char; 14]>(b"hello, hello!\x00")
    };
/* "hello world" would be more standard, but the repeated "hello"
 * stresses the compression code better, sorry...
 */
static mut dictionary: [std::os::raw::c_char; 6] =
    unsafe {
        *::std::mem::transmute::<&[u8; 6], &[std::os::raw::c_char; 6]>(b"hello\x00")
    };
static mut dictId: uLong = 0;
/* !Z_SOLO */
static mut zalloc: alloc_func = None;
static mut zfree: free_func = None;
/* ===========================================================================
 * Test compress() and uncompress()
 */
#[no_mangle]
pub unsafe extern "C" fn test_compress(mut compr: *mut Byte,
                                       mut comprLen: uLong,
                                       mut uncompr: *mut Byte,
                                       mut uncomprLen: uLong) {
    let mut err: std::os::raw::c_int = 0;
    let mut len: uLong =
        strlen(hello.as_mut_ptr()).wrapping_add(1 as std::os::raw::c_int as
                                                    std::os::raw::c_ulong);
    err =
        compress(compr, &mut comprLen, hello.as_mut_ptr() as *const Bytef,
                 len);
    if err != 0 as std::os::raw::c_int {
        fprintf(stderr,
                b"%s error: %d\n\x00" as *const u8 as *const std::os::raw::c_char,
                b"compress\x00" as *const u8 as *const std::os::raw::c_char, err);
        exit(1 as std::os::raw::c_int);
    }
    strcpy(uncompr as *mut std::os::raw::c_char,
           b"garbage\x00" as *const u8 as *const std::os::raw::c_char);
    err = uncompress(uncompr, &mut uncomprLen, compr, comprLen);
    if err != 0 as std::os::raw::c_int {
        fprintf(stderr,
                b"%s error: %d\n\x00" as *const u8 as *const std::os::raw::c_char,
                b"uncompress\x00" as *const u8 as *const std::os::raw::c_char, err);
        exit(1 as std::os::raw::c_int);
    }
    if strcmp(uncompr as *mut std::os::raw::c_char, hello.as_mut_ptr()) != 0 {
        fprintf(stderr,
                b"bad uncompress\n\x00" as *const u8 as *const std::os::raw::c_char);
        exit(1 as std::os::raw::c_int);
    } else {
        printf(b"uncompress(): %s\n\x00" as *const u8 as *const std::os::raw::c_char,
               uncompr as *mut std::os::raw::c_char);
    };
}
/* ===========================================================================
 * Test read/write of .gz files
 */
#[no_mangle]
pub unsafe extern "C" fn test_gzio(mut fname: *const std::os::raw::c_char,
                                   mut uncompr: *mut Byte,
                                   mut uncomprLen: uLong) {
    fprintf(stderr,
            b"NO_GZCOMPRESS -- gz* functions cannot compress\n\x00" as
                *const u8 as *const std::os::raw::c_char);
}
/* Adler32 value of the dictionary */
/* Z_SOLO */
/* ===========================================================================
 * Test deflate() with small buffers
 */
#[no_mangle]
pub unsafe extern "C" fn test_deflate(mut compr: *mut Byte,
                                      mut comprLen: uLong) {
    let mut c_stream: z_stream =
        z_stream{next_in: 0 as *mut Bytef,
                 avail_in: 0,
                 total_in: 0,
                 next_out: 0 as *mut Bytef,
                 avail_out: 0,
                 total_out: 0,
                 msg: 0 as *mut std::os::raw::c_char,
                 state: 0 as *mut internal_state,
                 zalloc: None,
                 zfree: None,
                 opaque: 0 as *mut std::os::raw::c_void,
                 data_type: 0,
                 adler: 0,
                 reserved: 0,}; /* compression stream */
    let mut err: std::os::raw::c_int = 0; /* force small buffers */
    let mut len: uLong =
        strlen(hello.as_mut_ptr()).wrapping_add(1 as std::os::raw::c_int as
                                                    std::os::raw::c_ulong);
    c_stream.zalloc = zalloc;
    c_stream.zfree = zfree;
    c_stream.opaque = 0 as voidpf;
    err =
        deflateInit_(&mut c_stream, -(1 as std::os::raw::c_int),
                     b"1.2.11-optipng\x00" as *const u8 as
                         *const std::os::raw::c_char,
                     ::std::mem::size_of::<z_stream>() as std::os::raw::c_ulong as
                         std::os::raw::c_int);
    if err != 0 as std::os::raw::c_int {
        fprintf(stderr,
                b"%s error: %d\n\x00" as *const u8 as *const std::os::raw::c_char,
                b"deflateInit\x00" as *const u8 as *const std::os::raw::c_char, err);
        exit(1 as std::os::raw::c_int);
    }
    c_stream.next_in = hello.as_mut_ptr() as *mut std::os::raw::c_uchar;
    c_stream.next_out = compr;
    while c_stream.total_in != len && c_stream.total_out < comprLen {
        c_stream.avail_out = 1 as std::os::raw::c_int as uInt;
        c_stream.avail_in = c_stream.avail_out;
        err = deflate(&mut c_stream, 0 as std::os::raw::c_int);
        if err != 0 as std::os::raw::c_int {
            fprintf(stderr,
                    b"%s error: %d\n\x00" as *const u8 as *const std::os::raw::c_char,
                    b"deflate\x00" as *const u8 as *const std::os::raw::c_char, err);
            exit(1 as std::os::raw::c_int);
        }
    }
    loop 
         /* Finish the stream, still forcing small buffers: */
         {
        c_stream.avail_out = 1 as std::os::raw::c_int as uInt;
        err = deflate(&mut c_stream, 4 as std::os::raw::c_int);
        if err == 1 as std::os::raw::c_int { break ; }
        if err != 0 as std::os::raw::c_int {
            fprintf(stderr,
                    b"%s error: %d\n\x00" as *const u8 as *const std::os::raw::c_char,
                    b"deflate\x00" as *const u8 as *const std::os::raw::c_char, err);
            exit(1 as std::os::raw::c_int);
        }
    }
    err = deflateEnd(&mut c_stream);
    if err != 0 as std::os::raw::c_int {
        fprintf(stderr,
                b"%s error: %d\n\x00" as *const u8 as *const std::os::raw::c_char,
                b"deflateEnd\x00" as *const u8 as *const std::os::raw::c_char, err);
        exit(1 as std::os::raw::c_int);
    };
}
/* ===========================================================================
 * Test inflate() with small buffers
 */
#[no_mangle]
pub unsafe extern "C" fn test_inflate(mut compr: *mut Byte,
                                      mut comprLen: uLong,
                                      mut uncompr: *mut Byte,
                                      mut uncomprLen: uLong) {
    let mut err: std::os::raw::c_int = 0; /* decompression stream */
    let mut d_stream: z_stream =
        z_stream{next_in: 0 as *mut Bytef,
                 avail_in: 0,
                 total_in: 0,
                 next_out: 0 as *mut Bytef,
                 avail_out: 0,
                 total_out: 0,
                 msg: 0 as *mut std::os::raw::c_char,
                 state: 0 as *mut internal_state,
                 zalloc: None,
                 zfree: None,
                 opaque: 0 as *mut std::os::raw::c_void,
                 data_type: 0,
                 adler: 0,
                 reserved: 0,}; /* force small buffers */
    strcpy(uncompr as *mut std::os::raw::c_char,
           b"garbage\x00" as *const u8 as *const std::os::raw::c_char);
    d_stream.zalloc = zalloc;
    d_stream.zfree = zfree;
    d_stream.opaque = 0 as voidpf;
    d_stream.next_in = compr;
    d_stream.avail_in = 0 as std::os::raw::c_int as uInt;
    d_stream.next_out = uncompr;
    err =
        inflateInit_(&mut d_stream,
                     b"1.2.11-optipng\x00" as *const u8 as
                         *const std::os::raw::c_char,
                     ::std::mem::size_of::<z_stream>() as std::os::raw::c_ulong as
                         std::os::raw::c_int);
    if err != 0 as std::os::raw::c_int {
        fprintf(stderr,
                b"%s error: %d\n\x00" as *const u8 as *const std::os::raw::c_char,
                b"inflateInit\x00" as *const u8 as *const std::os::raw::c_char, err);
        exit(1 as std::os::raw::c_int);
    }
    while d_stream.total_out < uncomprLen && d_stream.total_in < comprLen {
        d_stream.avail_out = 1 as std::os::raw::c_int as uInt;
        d_stream.avail_in = d_stream.avail_out;
        err = inflate(&mut d_stream, 0 as std::os::raw::c_int);
        if err == 1 as std::os::raw::c_int { break ; }
        if err != 0 as std::os::raw::c_int {
            fprintf(stderr,
                    b"%s error: %d\n\x00" as *const u8 as *const std::os::raw::c_char,
                    b"inflate\x00" as *const u8 as *const std::os::raw::c_char, err);
            exit(1 as std::os::raw::c_int);
        }
    }
    err = inflateEnd(&mut d_stream);
    if err != 0 as std::os::raw::c_int {
        fprintf(stderr,
                b"%s error: %d\n\x00" as *const u8 as *const std::os::raw::c_char,
                b"inflateEnd\x00" as *const u8 as *const std::os::raw::c_char, err);
        exit(1 as std::os::raw::c_int);
    }
    if strcmp(uncompr as *mut std::os::raw::c_char, hello.as_mut_ptr()) != 0 {
        fprintf(stderr,
                b"bad inflate\n\x00" as *const u8 as *const std::os::raw::c_char);
        exit(1 as std::os::raw::c_int);
    } else {
        printf(b"inflate(): %s\n\x00" as *const u8 as *const std::os::raw::c_char,
               uncompr as *mut std::os::raw::c_char);
    };
}
/* ===========================================================================
 * Test deflate() with large buffers and dynamic change of compression level
 */
#[no_mangle]
pub unsafe extern "C" fn test_large_deflate(mut compr: *mut Byte,
                                            mut comprLen: uLong,
                                            mut uncompr: *mut Byte,
                                            mut uncomprLen: uLong) {
    let mut c_stream: z_stream =
        z_stream{next_in: 0 as *mut Bytef,
                 avail_in: 0,
                 total_in: 0,
                 next_out: 0 as *mut Bytef,
                 avail_out: 0,
                 total_out: 0,
                 msg: 0 as *mut std::os::raw::c_char,
                 state: 0 as *mut internal_state,
                 zalloc: None,
                 zfree: None,
                 opaque: 0 as *mut std::os::raw::c_void,
                 data_type: 0,
                 adler: 0,
                 reserved: 0,}; /* compression stream */
    let mut err: std::os::raw::c_int = 0;
    c_stream.zalloc = zalloc;
    c_stream.zfree = zfree;
    c_stream.opaque = 0 as voidpf;
    err =
        deflateInit_(&mut c_stream, 1 as std::os::raw::c_int,
                     b"1.2.11-optipng\x00" as *const u8 as
                         *const std::os::raw::c_char,
                     ::std::mem::size_of::<z_stream>() as std::os::raw::c_ulong as
                         std::os::raw::c_int);
    if err != 0 as std::os::raw::c_int {
        fprintf(stderr,
                b"%s error: %d\n\x00" as *const u8 as *const std::os::raw::c_char,
                b"deflateInit\x00" as *const u8 as *const std::os::raw::c_char, err);
        exit(1 as std::os::raw::c_int);
    }
    c_stream.next_out = compr;
    c_stream.avail_out = comprLen as uInt;
    /* At this point, uncompr is still mostly zeroes, so it should compress
     * very well:
     */
    c_stream.next_in = uncompr;
    c_stream.avail_in = uncomprLen as uInt;
    err = deflate(&mut c_stream, 0 as std::os::raw::c_int);
    if err != 0 as std::os::raw::c_int {
        fprintf(stderr,
                b"%s error: %d\n\x00" as *const u8 as *const std::os::raw::c_char,
                b"deflate\x00" as *const u8 as *const std::os::raw::c_char, err);
        exit(1 as std::os::raw::c_int);
    }
    if c_stream.avail_in != 0 as std::os::raw::c_int as std::os::raw::c_uint {
        fprintf(stderr,
                b"deflate not greedy\n\x00" as *const u8 as
                    *const std::os::raw::c_char);
        exit(1 as std::os::raw::c_int);
    }
    /* Feed in already compressed data and switch to no compression: */
    deflateParams(&mut c_stream, 0 as std::os::raw::c_int, 0 as std::os::raw::c_int);
    c_stream.next_in = compr;
    c_stream.avail_in =
        (comprLen as uInt).wrapping_div(2 as std::os::raw::c_int as std::os::raw::c_uint);
    err = deflate(&mut c_stream, 0 as std::os::raw::c_int);
    if err != 0 as std::os::raw::c_int {
        fprintf(stderr,
                b"%s error: %d\n\x00" as *const u8 as *const std::os::raw::c_char,
                b"deflate\x00" as *const u8 as *const std::os::raw::c_char, err);
        exit(1 as std::os::raw::c_int);
    }
    /* Switch back to compressing mode: */
    deflateParams(&mut c_stream, 9 as std::os::raw::c_int, 1 as std::os::raw::c_int);
    c_stream.next_in = uncompr;
    c_stream.avail_in = uncomprLen as uInt;
    err = deflate(&mut c_stream, 0 as std::os::raw::c_int);
    if err != 0 as std::os::raw::c_int {
        fprintf(stderr,
                b"%s error: %d\n\x00" as *const u8 as *const std::os::raw::c_char,
                b"deflate\x00" as *const u8 as *const std::os::raw::c_char, err);
        exit(1 as std::os::raw::c_int);
    }
    err = deflate(&mut c_stream, 4 as std::os::raw::c_int);
    if err != 1 as std::os::raw::c_int {
        fprintf(stderr,
                b"deflate should report Z_STREAM_END\n\x00" as *const u8 as
                    *const std::os::raw::c_char);
        exit(1 as std::os::raw::c_int);
    }
    err = deflateEnd(&mut c_stream);
    if err != 0 as std::os::raw::c_int {
        fprintf(stderr,
                b"%s error: %d\n\x00" as *const u8 as *const std::os::raw::c_char,
                b"deflateEnd\x00" as *const u8 as *const std::os::raw::c_char, err);
        exit(1 as std::os::raw::c_int);
    };
}
/* ===========================================================================
 * Test inflate() with large buffers
 */
#[no_mangle]
pub unsafe extern "C" fn test_large_inflate(mut compr: *mut Byte,
                                            mut comprLen: uLong,
                                            mut uncompr: *mut Byte,
                                            mut uncomprLen: uLong) {
    let mut err: std::os::raw::c_int = 0; /* decompression stream */
    let mut d_stream: z_stream =
        z_stream{next_in: 0 as *mut Bytef,
                 avail_in: 0,
                 total_in: 0,
                 next_out: 0 as *mut Bytef,
                 avail_out: 0,
                 total_out: 0,
                 msg: 0 as *mut std::os::raw::c_char,
                 state: 0 as *mut internal_state,
                 zalloc: None,
                 zfree: None,
                 opaque: 0 as *mut std::os::raw::c_void,
                 data_type: 0,
                 adler: 0,
                 reserved: 0,}; /* discard the output */
    strcpy(uncompr as *mut std::os::raw::c_char,
           b"garbage\x00" as *const u8 as *const std::os::raw::c_char);
    d_stream.zalloc = zalloc;
    d_stream.zfree = zfree;
    d_stream.opaque = 0 as voidpf;
    d_stream.next_in = compr;
    d_stream.avail_in = comprLen as uInt;
    err =
        inflateInit_(&mut d_stream,
                     b"1.2.11-optipng\x00" as *const u8 as
                         *const std::os::raw::c_char,
                     ::std::mem::size_of::<z_stream>() as std::os::raw::c_ulong as
                         std::os::raw::c_int);
    if err != 0 as std::os::raw::c_int {
        fprintf(stderr,
                b"%s error: %d\n\x00" as *const u8 as *const std::os::raw::c_char,
                b"inflateInit\x00" as *const u8 as *const std::os::raw::c_char, err);
        exit(1 as std::os::raw::c_int);
    }
    loop  {
        d_stream.next_out = uncompr;
        d_stream.avail_out = uncomprLen as uInt;
        err = inflate(&mut d_stream, 0 as std::os::raw::c_int);
        if err == 1 as std::os::raw::c_int { break ; }
        if err != 0 as std::os::raw::c_int {
            fprintf(stderr,
                    b"%s error: %d\n\x00" as *const u8 as *const std::os::raw::c_char,
                    b"large inflate\x00" as *const u8 as *const std::os::raw::c_char,
                    err);
            exit(1 as std::os::raw::c_int);
        }
    }
    err = inflateEnd(&mut d_stream);
    if err != 0 as std::os::raw::c_int {
        fprintf(stderr,
                b"%s error: %d\n\x00" as *const u8 as *const std::os::raw::c_char,
                b"inflateEnd\x00" as *const u8 as *const std::os::raw::c_char, err);
        exit(1 as std::os::raw::c_int);
    }
    if d_stream.total_out !=
           (2 as std::os::raw::c_int as
                std::os::raw::c_ulong).wrapping_mul(uncomprLen).wrapping_add(comprLen.wrapping_div(2
                                                                                               as
                                                                                               std::os::raw::c_int
                                                                                               as
                                                                                               std::os::raw::c_ulong))
       {
        fprintf(stderr,
                b"bad large inflate: %ld\n\x00" as *const u8 as
                    *const std::os::raw::c_char, d_stream.total_out);
        exit(1 as std::os::raw::c_int);
    } else {
        printf(b"large_inflate(): OK\n\x00" as *const u8 as
                   *const std::os::raw::c_char);
    };
}
/* ===========================================================================
 * Test deflate() with full flush
 */
#[no_mangle]
pub unsafe extern "C" fn test_flush(mut compr: *mut Byte,
                                    mut comprLen: *mut uLong) {
    let mut c_stream: z_stream =
        z_stream{next_in: 0 as *mut Bytef,
                 avail_in: 0,
                 total_in: 0,
                 next_out: 0 as *mut Bytef,
                 avail_out: 0,
                 total_out: 0,
                 msg: 0 as *mut std::os::raw::c_char,
                 state: 0 as *mut internal_state,
                 zalloc: None,
                 zfree: None,
                 opaque: 0 as *mut std::os::raw::c_void,
                 data_type: 0,
                 adler: 0,
                 reserved: 0,}; /* compression stream */
    let mut err: std::os::raw::c_int =
        0; /* force an error in first compressed block */
    let mut len: uInt =
        (strlen(hello.as_mut_ptr()) as
             uInt).wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_uint);
    c_stream.zalloc = zalloc;
    c_stream.zfree = zfree;
    c_stream.opaque = 0 as voidpf;
    err =
        deflateInit_(&mut c_stream, -(1 as std::os::raw::c_int),
                     b"1.2.11-optipng\x00" as *const u8 as
                         *const std::os::raw::c_char,
                     ::std::mem::size_of::<z_stream>() as std::os::raw::c_ulong as
                         std::os::raw::c_int);
    if err != 0 as std::os::raw::c_int {
        fprintf(stderr,
                b"%s error: %d\n\x00" as *const u8 as *const std::os::raw::c_char,
                b"deflateInit\x00" as *const u8 as *const std::os::raw::c_char, err);
        exit(1 as std::os::raw::c_int);
    }
    c_stream.next_in = hello.as_mut_ptr() as *mut std::os::raw::c_uchar;
    c_stream.next_out = compr;
    c_stream.avail_in = 3 as std::os::raw::c_int as uInt;
    c_stream.avail_out = *comprLen as uInt;
    err = deflate(&mut c_stream, 3 as std::os::raw::c_int);
    if err != 0 as std::os::raw::c_int {
        fprintf(stderr,
                b"%s error: %d\n\x00" as *const u8 as *const std::os::raw::c_char,
                b"deflate\x00" as *const u8 as *const std::os::raw::c_char, err);
        exit(1 as std::os::raw::c_int);
    }
    let ref mut fresh0 = *compr.offset(3 as std::os::raw::c_int as isize);
    *fresh0 = (*fresh0).wrapping_add(1);
    c_stream.avail_in = len.wrapping_sub(3 as std::os::raw::c_int as std::os::raw::c_uint);
    err = deflate(&mut c_stream, 4 as std::os::raw::c_int);
    if err != 1 as std::os::raw::c_int {
        if err != 0 as std::os::raw::c_int {
            fprintf(stderr,
                    b"%s error: %d\n\x00" as *const u8 as *const std::os::raw::c_char,
                    b"deflate\x00" as *const u8 as *const std::os::raw::c_char, err);
            exit(1 as std::os::raw::c_int);
        }
    }
    err = deflateEnd(&mut c_stream);
    if err != 0 as std::os::raw::c_int {
        fprintf(stderr,
                b"%s error: %d\n\x00" as *const u8 as *const std::os::raw::c_char,
                b"deflateEnd\x00" as *const u8 as *const std::os::raw::c_char, err);
        exit(1 as std::os::raw::c_int);
    }
    *comprLen = c_stream.total_out;
}
/* ===========================================================================
 * Test inflateSync()
 */
#[no_mangle]
pub unsafe extern "C" fn test_sync(mut compr: *mut Byte, mut comprLen: uLong,
                                   mut uncompr: *mut Byte,
                                   mut uncomprLen: uLong) {
    let mut err: std::os::raw::c_int = 0; /* decompression stream */
    let mut d_stream: z_stream =
        z_stream{next_in: 0 as *mut Bytef,
                 avail_in: 0,
                 total_in: 0,
                 next_out: 0 as *mut Bytef,
                 avail_out: 0,
                 total_out: 0,
                 msg: 0 as *mut std::os::raw::c_char,
                 state: 0 as *mut internal_state,
                 zalloc: None,
                 zfree: None,
                 opaque: 0 as *mut std::os::raw::c_void,
                 data_type: 0,
                 adler: 0,
                 reserved: 0,}; /* just read the zlib header */
    strcpy(uncompr as *mut std::os::raw::c_char,
           b"garbage\x00" as *const u8 as
               *const std::os::raw::c_char); /* read all compressed data */
    d_stream.zalloc = zalloc; /* but skip the damaged part */
    d_stream.zfree = zfree;
    d_stream.opaque = 0 as voidpf;
    d_stream.next_in = compr;
    d_stream.avail_in = 2 as std::os::raw::c_int as uInt;
    err =
        inflateInit_(&mut d_stream,
                     b"1.2.11-optipng\x00" as *const u8 as
                         *const std::os::raw::c_char,
                     ::std::mem::size_of::<z_stream>() as std::os::raw::c_ulong as
                         std::os::raw::c_int);
    if err != 0 as std::os::raw::c_int {
        fprintf(stderr,
                b"%s error: %d\n\x00" as *const u8 as *const std::os::raw::c_char,
                b"inflateInit\x00" as *const u8 as *const std::os::raw::c_char, err);
        exit(1 as std::os::raw::c_int);
    }
    d_stream.next_out = uncompr;
    d_stream.avail_out = uncomprLen as uInt;
    err = inflate(&mut d_stream, 0 as std::os::raw::c_int);
    if err != 0 as std::os::raw::c_int {
        fprintf(stderr,
                b"%s error: %d\n\x00" as *const u8 as *const std::os::raw::c_char,
                b"inflate\x00" as *const u8 as *const std::os::raw::c_char, err);
        exit(1 as std::os::raw::c_int);
    }
    d_stream.avail_in =
        (comprLen as uInt).wrapping_sub(2 as std::os::raw::c_int as std::os::raw::c_uint);
    err = inflateSync(&mut d_stream);
    if err != 0 as std::os::raw::c_int {
        fprintf(stderr,
                b"%s error: %d\n\x00" as *const u8 as *const std::os::raw::c_char,
                b"inflateSync\x00" as *const u8 as *const std::os::raw::c_char, err);
        exit(1 as std::os::raw::c_int);
    }
    err = inflate(&mut d_stream, 4 as std::os::raw::c_int);
    if err != -(3 as std::os::raw::c_int) {
        fprintf(stderr,
                b"inflate should report DATA_ERROR\n\x00" as *const u8 as
                    *const std::os::raw::c_char);
        /* Because of incorrect adler32 */
        exit(1 as std::os::raw::c_int);
    }
    err = inflateEnd(&mut d_stream);
    if err != 0 as std::os::raw::c_int {
        fprintf(stderr,
                b"%s error: %d\n\x00" as *const u8 as *const std::os::raw::c_char,
                b"inflateEnd\x00" as *const u8 as *const std::os::raw::c_char, err);
        exit(1 as std::os::raw::c_int);
    }
    printf(b"after inflateSync(): hel%s\n\x00" as *const u8 as
               *const std::os::raw::c_char, uncompr as *mut std::os::raw::c_char);
}
/* ===========================================================================
 * Test deflate() with preset dictionary
 */
#[no_mangle]
pub unsafe extern "C" fn test_dict_deflate(mut compr: *mut Byte,
                                           mut comprLen: uLong) {
    let mut c_stream: z_stream =
        z_stream{next_in: 0 as *mut Bytef,
                 avail_in: 0,
                 total_in: 0,
                 next_out: 0 as *mut Bytef,
                 avail_out: 0,
                 total_out: 0,
                 msg: 0 as *mut std::os::raw::c_char,
                 state: 0 as *mut internal_state,
                 zalloc: None,
                 zfree: None,
                 opaque: 0 as *mut std::os::raw::c_void,
                 data_type: 0,
                 adler: 0,
                 reserved: 0,}; /* compression stream */
    let mut err: std::os::raw::c_int = 0;
    c_stream.zalloc = zalloc;
    c_stream.zfree = zfree;
    c_stream.opaque = 0 as voidpf;
    err =
        deflateInit_(&mut c_stream, 9 as std::os::raw::c_int,
                     b"1.2.11-optipng\x00" as *const u8 as
                         *const std::os::raw::c_char,
                     ::std::mem::size_of::<z_stream>() as std::os::raw::c_ulong as
                         std::os::raw::c_int);
    if err != 0 as std::os::raw::c_int {
        fprintf(stderr,
                b"%s error: %d\n\x00" as *const u8 as *const std::os::raw::c_char,
                b"deflateInit\x00" as *const u8 as *const std::os::raw::c_char, err);
        exit(1 as std::os::raw::c_int);
    }
    err =
        deflateSetDictionary(&mut c_stream,
                             dictionary.as_ptr() as *const Bytef,
                             ::std::mem::size_of::<[std::os::raw::c_char; 6]>() as
                                 std::os::raw::c_ulong as std::os::raw::c_int as uInt);
    if err != 0 as std::os::raw::c_int {
        fprintf(stderr,
                b"%s error: %d\n\x00" as *const u8 as *const std::os::raw::c_char,
                b"deflateSetDictionary\x00" as *const u8 as
                    *const std::os::raw::c_char, err);
        exit(1 as std::os::raw::c_int);
    }
    dictId = c_stream.adler;
    c_stream.next_out = compr;
    c_stream.avail_out = comprLen as uInt;
    c_stream.next_in = hello.as_mut_ptr() as *mut std::os::raw::c_uchar;
    c_stream.avail_in =
        (strlen(hello.as_mut_ptr()) as
             uInt).wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_uint);
    err = deflate(&mut c_stream, 4 as std::os::raw::c_int);
    if err != 1 as std::os::raw::c_int {
        fprintf(stderr,
                b"deflate should report Z_STREAM_END\n\x00" as *const u8 as
                    *const std::os::raw::c_char);
        exit(1 as std::os::raw::c_int);
    }
    err = deflateEnd(&mut c_stream);
    if err != 0 as std::os::raw::c_int {
        fprintf(stderr,
                b"%s error: %d\n\x00" as *const u8 as *const std::os::raw::c_char,
                b"deflateEnd\x00" as *const u8 as *const std::os::raw::c_char, err);
        exit(1 as std::os::raw::c_int);
    };
}
/* ===========================================================================
 * Test inflate() with a preset dictionary
 */
#[no_mangle]
pub unsafe extern "C" fn test_dict_inflate(mut compr: *mut Byte,
                                           mut comprLen: uLong,
                                           mut uncompr: *mut Byte,
                                           mut uncomprLen: uLong) {
    let mut err: std::os::raw::c_int = 0; /* decompression stream */
    let mut d_stream: z_stream =
        z_stream{next_in: 0 as *mut Bytef,
                 avail_in: 0,
                 total_in: 0,
                 next_out: 0 as *mut Bytef,
                 avail_out: 0,
                 total_out: 0,
                 msg: 0 as *mut std::os::raw::c_char,
                 state: 0 as *mut internal_state,
                 zalloc: None,
                 zfree: None,
                 opaque: 0 as *mut std::os::raw::c_void,
                 data_type: 0,
                 adler: 0,
                 reserved: 0,};
    strcpy(uncompr as *mut std::os::raw::c_char,
           b"garbage\x00" as *const u8 as *const std::os::raw::c_char);
    d_stream.zalloc = zalloc;
    d_stream.zfree = zfree;
    d_stream.opaque = 0 as voidpf;
    d_stream.next_in = compr;
    d_stream.avail_in = comprLen as uInt;
    err =
        inflateInit_(&mut d_stream,
                     b"1.2.11-optipng\x00" as *const u8 as
                         *const std::os::raw::c_char,
                     ::std::mem::size_of::<z_stream>() as std::os::raw::c_ulong as
                         std::os::raw::c_int);
    if err != 0 as std::os::raw::c_int {
        fprintf(stderr,
                b"%s error: %d\n\x00" as *const u8 as *const std::os::raw::c_char,
                b"inflateInit\x00" as *const u8 as *const std::os::raw::c_char, err);
        exit(1 as std::os::raw::c_int);
    }
    d_stream.next_out = uncompr;
    d_stream.avail_out = uncomprLen as uInt;
    loop  {
        err = inflate(&mut d_stream, 0 as std::os::raw::c_int);
        if err == 1 as std::os::raw::c_int { break ; }
        if err == 2 as std::os::raw::c_int {
            if d_stream.adler != dictId {
                fprintf(stderr,
                        b"unexpected dictionary\x00" as *const u8 as
                            *const std::os::raw::c_char);
                exit(1 as std::os::raw::c_int);
            }
            err =
                inflateSetDictionary(&mut d_stream,
                                     dictionary.as_ptr() as *const Bytef,
                                     ::std::mem::size_of::<[std::os::raw::c_char; 6]>()
                                         as std::os::raw::c_ulong as std::os::raw::c_int as
                                         uInt)
        }
        if err != 0 as std::os::raw::c_int {
            fprintf(stderr,
                    b"%s error: %d\n\x00" as *const u8 as *const std::os::raw::c_char,
                    b"inflate with dict\x00" as *const u8 as
                        *const std::os::raw::c_char, err);
            exit(1 as std::os::raw::c_int);
        }
    }
    err = inflateEnd(&mut d_stream);
    if err != 0 as std::os::raw::c_int {
        fprintf(stderr,
                b"%s error: %d\n\x00" as *const u8 as *const std::os::raw::c_char,
                b"inflateEnd\x00" as *const u8 as *const std::os::raw::c_char, err);
        exit(1 as std::os::raw::c_int);
    }
    if strcmp(uncompr as *mut std::os::raw::c_char, hello.as_mut_ptr()) != 0 {
        fprintf(stderr,
                b"bad inflate with dict\n\x00" as *const u8 as
                    *const std::os::raw::c_char);
        exit(1 as std::os::raw::c_int);
    } else {
        printf(b"inflate with dictionary: %s\n\x00" as *const u8 as
                   *const std::os::raw::c_char, uncompr as *mut std::os::raw::c_char);
    };
}
/* ===========================================================================
 * Usage:  example [output.gz  [input.gz]]
 */
unsafe fn main_0(mut argc: std::os::raw::c_int, mut argv: *mut *mut std::os::raw::c_char)
 -> std::os::raw::c_int {
    let mut compr: *mut Byte = 0 as *mut Byte; /* don't overflow on MSDOS */
    let mut uncompr: *mut Byte = 0 as *mut Byte;
    let mut comprLen: uLong =
        (10000 as std::os::raw::c_int as
             std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<std::os::raw::c_int>()
                                             as std::os::raw::c_ulong);
    let mut uncomprLen: uLong = comprLen;
    static mut myVersion: *const std::os::raw::c_char =
        b"1.2.11-optipng\x00" as *const u8 as *const std::os::raw::c_char;
    if *zlibVersion().offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int !=
           *myVersion.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int {
        fprintf(stderr,
                b"incompatible zlib version\n\x00" as *const u8 as
                    *const std::os::raw::c_char);
        exit(1 as std::os::raw::c_int);
    } else {
        if strcmp(zlibVersion(),
                  b"1.2.11-optipng\x00" as *const u8 as *const std::os::raw::c_char)
               != 0 as std::os::raw::c_int {
            fprintf(stderr,
                    b"warning: different zlib version\n\x00" as *const u8 as
                        *const std::os::raw::c_char);
        }
    }
    printf(b"zlib version %s = 0x%04x, compile flags = 0x%lx\n\x00" as
               *const u8 as *const std::os::raw::c_char,
           b"1.2.11-optipng\x00" as *const u8 as *const std::os::raw::c_char,
           0x12bf as std::os::raw::c_int, zlibCompileFlags());
    compr =
        calloc(comprLen as uInt as std::os::raw::c_ulong,
               1 as std::os::raw::c_int as std::os::raw::c_ulong) as *mut Byte;
    uncompr =
        calloc(uncomprLen as uInt as std::os::raw::c_ulong,
               1 as std::os::raw::c_int as std::os::raw::c_ulong) as *mut Byte;
    /* compr and uncompr are cleared to avoid reading uninitialized
     * data and to ensure that uncompr compresses well.
     */
    if compr.is_null() || uncompr.is_null() {
        printf(b"out of memory\n\x00" as *const u8 as *const std::os::raw::c_char);
        exit(1 as std::os::raw::c_int);
    }
    test_compress(compr, comprLen, uncompr, uncomprLen);
    test_gzio(if argc > 1 as std::os::raw::c_int {
                  *argv.offset(1 as std::os::raw::c_int as isize) as
                      *const std::os::raw::c_char
              } else { b"foo.gz\x00" as *const u8 as *const std::os::raw::c_char },
              uncompr, uncomprLen);
    test_deflate(compr, comprLen);
    test_inflate(compr, comprLen, uncompr, uncomprLen);
    test_large_deflate(compr, comprLen, uncompr, uncomprLen);
    test_large_inflate(compr, comprLen, uncompr, uncomprLen);
    test_flush(compr, &mut comprLen);
    test_sync(compr, comprLen, uncompr, uncomprLen);
    comprLen = uncomprLen;
    test_dict_deflate(compr, comprLen);
    test_dict_inflate(compr, comprLen, uncompr, uncomprLen);
    free(compr as *mut std::os::raw::c_void);
    free(uncompr as *mut std::os::raw::c_void);
    return 0 as std::os::raw::c_int;
}
#[main]
pub fn main() {
    let mut args: Vec<*mut std::os::raw::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(::std::ffi::CString::new(arg).expect("Failed to convert argument into CString.").into_raw());
    };
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0((args.len() - 1) as std::os::raw::c_int,
                                    args.as_mut_ptr() as
                                        *mut *mut std::os::raw::c_char) as i32)
    }
}
